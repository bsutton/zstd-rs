//! Target-compressed-block-size helpers ported from
//! `zstd_compress_superblock.c`.

use crate::encoding::blocks::PreparedSequence;
use alloc::vec::Vec;

const BYTESCALE: usize = 256;
const ENTROPY_HEADER_BUDGET: usize = 120 * BYTESCALE;

pub(super) const TARGET_CBLOCK_SIZE_MIN: usize = 1340;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct EstimatedSubBlockSize {
    pub(super) literal_size: usize,
    pub(super) block_size: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct SubBlockBudgetPlan {
    pub(super) avg_lit_cost: usize,
    pub(super) avg_seq_cost: usize,
    pub(super) nb_sub_blocks: usize,
    pub(super) avg_block_budget: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum EntropyTableMode {
    Basic,
    Rle,
    Compressed,
    Repeat,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct SequenceEntropyModes {
    pub(super) ll: EntropyTableMode,
    pub(super) ml: EntropyTableMode,
    pub(super) of: EntropyTableMode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct PlannedSubBlock {
    pub(super) start_sequence: usize,
    pub(super) end_sequence: usize,
    pub(super) literal_size: usize,
    pub(super) decompressed_size: usize,
    pub(super) last: bool,
}

pub(super) fn sub_block_budget_plan(
    estimate: EstimatedSubBlockSize,
    nb_literals: usize,
    nb_sequences: usize,
    target_c_block_size: usize,
    src_size: usize,
) -> Option<SubBlockBudgetPlan> {
    debug_assert!(nb_sequences > 0);
    debug_assert!(estimate.literal_size <= estimate.block_size);
    if estimate.block_size > src_size {
        return None;
    }

    let target = target_c_block_size.max(TARGET_CBLOCK_SIZE_MIN);
    let nb_sub_blocks = target_sub_block_count(estimate.block_size, target);
    let avg_lit_cost = if nb_literals > 0 {
        (estimate.literal_size * BYTESCALE) / nb_literals
    } else {
        BYTESCALE
    };
    let avg_seq_cost = ((estimate.block_size - estimate.literal_size) * BYTESCALE) / nb_sequences;
    let avg_block_budget = (estimate.block_size * BYTESCALE) / nb_sub_blocks;

    Some(SubBlockBudgetPlan {
        avg_lit_cost,
        avg_seq_cost,
        nb_sub_blocks,
        avg_block_budget,
    })
}

pub(super) fn should_commit_sub_block(compressed_size: usize, decompressed_size: usize) -> bool {
    compressed_size > 0 && compressed_size < decompressed_size
}

pub(super) fn need_sequence_entropy_tables(modes: SequenceEntropyModes) -> bool {
    [modes.ll, modes.ml, modes.of]
        .iter()
        .any(|mode| matches!(mode, EntropyTableMode::Compressed | EntropyTableMode::Rle))
}

pub(super) fn plan_sub_blocks(
    sequences: &[PreparedSequence],
    total_literal_size: usize,
    plan: SubBlockBudgetPlan,
    mut compressed_size_for: impl FnMut(PlannedSubBlock) -> usize,
) -> Vec<PlannedSubBlock> {
    debug_assert!(!sequences.is_empty());

    let mut sub_blocks = Vec::new();
    let mut start_sequence = 0;
    let mut literal_consumed = 0;

    for block_idx in 0..plan.nb_sub_blocks.saturating_sub(1) {
        let remaining = &sequences[start_sequence..];
        if remaining.is_empty() {
            break;
        }

        let sequence_count = size_block_sequences(
            remaining,
            plan.avg_block_budget,
            plan.avg_lit_cost,
            plan.avg_seq_cost,
            block_idx == 0,
        );
        let end_sequence = start_sequence + sequence_count;
        if end_sequence == sequences.len() {
            break;
        }

        let candidate = planned_sub_block(
            sequences,
            start_sequence,
            end_sequence,
            count_literals(&sequences[start_sequence..end_sequence]),
            false,
        );
        let compressed_size = compressed_size_for(candidate);
        if should_commit_sub_block(compressed_size, candidate.decompressed_size) {
            literal_consumed += candidate.literal_size;
            sub_blocks.push(candidate);
            start_sequence = end_sequence;
        }
    }

    debug_assert!(literal_consumed <= total_literal_size);
    sub_blocks.push(planned_sub_block(
        sequences,
        start_sequence,
        sequences.len(),
        total_literal_size - literal_consumed,
        true,
    ));
    sub_blocks
}

fn planned_sub_block(
    sequences: &[PreparedSequence],
    start_sequence: usize,
    end_sequence: usize,
    literal_size: usize,
    last: bool,
) -> PlannedSubBlock {
    PlannedSubBlock {
        start_sequence,
        end_sequence,
        literal_size,
        decompressed_size: decompressed_size(
            &sequences[start_sequence..end_sequence],
            literal_size,
            last,
        ),
        last,
    }
}

pub(super) fn count_literals(sequences: &[PreparedSequence]) -> usize {
    sequences.iter().map(|sequence| sequence.ll as usize).sum()
}

pub(super) fn decompressed_size(
    sequences: &[PreparedSequence],
    literal_size: usize,
    last_sub_block: bool,
) -> usize {
    let match_length_sum = sequences
        .iter()
        .map(|sequence| sequence.ml as usize)
        .sum::<usize>();
    let literal_length_sum = count_literals(sequences);
    if last_sub_block {
        debug_assert!(literal_length_sum <= literal_size);
    } else {
        debug_assert_eq!(literal_length_sum, literal_size);
    }
    match_length_sum + literal_size
}

pub(super) fn target_sub_block_count(
    estimated_block_size: usize,
    target_c_block_size: usize,
) -> usize {
    let target = target_c_block_size.max(TARGET_CBLOCK_SIZE_MIN);
    ((estimated_block_size + (target / 2)) / target).max(1)
}

pub(super) fn size_block_sequences(
    sequences: &[PreparedSequence],
    target_budget: usize,
    avg_lit_cost: usize,
    avg_seq_cost: usize,
    first_sub_block: bool,
) -> usize {
    debug_assert!(!sequences.is_empty());

    let mut budget = if first_sub_block {
        ENTROPY_HEADER_BUDGET
    } else {
        0
    };

    budget += sequences[0].ll as usize * avg_lit_cost + avg_seq_cost;
    if budget > target_budget {
        return 1;
    }

    let mut in_size = sequences[0].ll as usize + sequences[0].ml as usize;
    for (idx, sequence) in sequences.iter().enumerate().skip(1) {
        let current_cost = sequence.ll as usize * avg_lit_cost + avg_seq_cost;
        budget += current_cost;
        in_size += sequence.ll as usize + sequence.ml as usize;
        if budget > target_budget && budget < in_size * BYTESCALE {
            return idx;
        }
    }

    sequences.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sequence(ll: u32, ml: u32) -> PreparedSequence {
        PreparedSequence {
            ll,
            ml,
            raw_offset: 1,
            encoded_offset_value: None,
        }
    }

    #[test]
    fn target_sub_block_count_clamps_target_and_rounds_like_c() {
        assert_eq!(target_sub_block_count(0, 0), 1);
        assert_eq!(target_sub_block_count(1_000, 0), 1);
        assert_eq!(target_sub_block_count(2_010, 0), 2);
        assert_eq!(target_sub_block_count(6_700, 1_340), 5);
        assert_eq!(target_sub_block_count(6_701, 1_340), 5);
        assert_eq!(target_sub_block_count(7_371, 1_340), 6);
    }

    #[test]
    fn sub_block_budget_plan_matches_c_quick_estimation_formula() {
        let plan = sub_block_budget_plan(
            EstimatedSubBlockSize {
                literal_size: 300,
                block_size: 2_010,
            },
            100,
            30,
            1_340,
            8_000,
        )
        .expect("estimated superblock is compressible");

        assert_eq!(
            plan,
            SubBlockBudgetPlan {
                avg_lit_cost: 768,
                avg_seq_cost: 14_592,
                nb_sub_blocks: 2,
                avg_block_budget: 257_280,
            }
        );
    }

    #[test]
    fn sub_block_budget_plan_uses_one_byte_per_literal_when_no_literals() {
        let plan = sub_block_budget_plan(
            EstimatedSubBlockSize {
                literal_size: 0,
                block_size: 1_340,
            },
            0,
            10,
            1_340,
            4_096,
        )
        .expect("estimated superblock is compressible");

        assert_eq!(plan.avg_lit_cost, BYTESCALE);
        assert_eq!(plan.avg_seq_cost, 34_304);
    }

    #[test]
    fn sub_block_budget_plan_clamps_target_size_like_c() {
        let clamped = sub_block_budget_plan(
            EstimatedSubBlockSize {
                literal_size: 100,
                block_size: 2_010,
            },
            50,
            10,
            1,
            4_096,
        )
        .expect("estimated superblock is compressible");
        let explicit = sub_block_budget_plan(
            EstimatedSubBlockSize {
                literal_size: 100,
                block_size: 2_010,
            },
            50,
            10,
            TARGET_CBLOCK_SIZE_MIN,
            4_096,
        )
        .expect("estimated superblock is compressible");

        assert_eq!(clamped, explicit);
    }

    #[test]
    fn sub_block_budget_plan_bails_out_when_estimate_exceeds_source_size() {
        assert_eq!(
            sub_block_budget_plan(
                EstimatedSubBlockSize {
                    literal_size: 500,
                    block_size: 4_097,
                },
                100,
                10,
                1_340,
                4_096,
            ),
            None
        );
    }

    #[test]
    fn should_commit_sub_block_matches_c_compressibility_gate() {
        assert!(should_commit_sub_block(9, 10));
        assert!(!should_commit_sub_block(0, 10));
        assert!(!should_commit_sub_block(10, 10));
        assert!(!should_commit_sub_block(11, 10));
    }

    #[test]
    fn need_sequence_entropy_tables_matches_c_metadata_gate() {
        let no_tables = SequenceEntropyModes {
            ll: EntropyTableMode::Basic,
            ml: EntropyTableMode::Repeat,
            of: EntropyTableMode::Basic,
        };
        let rle_tables = SequenceEntropyModes {
            ll: EntropyTableMode::Basic,
            ml: EntropyTableMode::Rle,
            of: EntropyTableMode::Basic,
        };
        let compressed_tables = SequenceEntropyModes {
            ll: EntropyTableMode::Repeat,
            ml: EntropyTableMode::Basic,
            of: EntropyTableMode::Compressed,
        };

        assert!(!need_sequence_entropy_tables(no_tables));
        assert!(need_sequence_entropy_tables(rle_tables));
        assert!(need_sequence_entropy_tables(compressed_tables));
    }

    #[test]
    fn plan_sub_blocks_commits_compressible_tentative_block() {
        let sequences = [sequence(1, 200), sequence(2, 200), sequence(3, 200)];
        let plan = SubBlockBudgetPlan {
            avg_lit_cost: 0,
            avg_seq_cost: 100,
            nb_sub_blocks: 2,
            avg_block_budget: ENTROPY_HEADER_BUDGET + 150,
        };

        let mut outcomes = [3].iter().copied();
        let blocks = plan_sub_blocks(&sequences, 9, plan, |_| {
            outcomes.next().expect("one tentative block")
        });

        assert_eq!(
            blocks,
            alloc::vec![
                PlannedSubBlock {
                    start_sequence: 0,
                    end_sequence: 1,
                    literal_size: 1,
                    decompressed_size: 201,
                    last: false,
                },
                PlannedSubBlock {
                    start_sequence: 1,
                    end_sequence: 3,
                    literal_size: 8,
                    decompressed_size: 408,
                    last: true,
                },
            ]
        );
    }

    #[test]
    fn plan_sub_blocks_coalesces_failed_tentative_block_like_c() {
        let sequences = [sequence(0, 10), sequence(0, 10), sequence(0, 10)];
        let plan = SubBlockBudgetPlan {
            avg_lit_cost: 0,
            avg_seq_cost: 100,
            nb_sub_blocks: 3,
            avg_block_budget: 250,
        };

        let mut outcomes = [0, 10].iter().copied();
        let blocks = plan_sub_blocks(&sequences, 0, plan, |_| {
            outcomes.next().expect("two tentative blocks")
        });

        assert_eq!(
            blocks,
            alloc::vec![
                PlannedSubBlock {
                    start_sequence: 0,
                    end_sequence: 2,
                    literal_size: 0,
                    decompressed_size: 20,
                    last: false,
                },
                PlannedSubBlock {
                    start_sequence: 2,
                    end_sequence: 3,
                    literal_size: 0,
                    decompressed_size: 10,
                    last: true,
                },
            ]
        );
    }

    #[test]
    fn plan_sub_blocks_breaks_to_last_block_when_candidate_reaches_end() {
        let sequences = [sequence(1, 5), sequence(2, 7)];
        let plan = SubBlockBudgetPlan {
            avg_lit_cost: 0,
            avg_seq_cost: 1,
            nb_sub_blocks: 3,
            avg_block_budget: ENTROPY_HEADER_BUDGET + 100,
        };

        let blocks = plan_sub_blocks(&sequences, 5, plan, |_| {
            panic!("candidate reaches end and should not be compressed")
        });

        assert_eq!(
            blocks,
            alloc::vec![PlannedSubBlock {
                start_sequence: 0,
                end_sequence: 2,
                literal_size: 5,
                decompressed_size: 17,
                last: true,
            }]
        );
    }

    #[test]
    fn count_literals_sums_sequence_literal_lengths_like_c() {
        let sequences = [sequence(2, 5), sequence(0, 3), sequence(7, 11)];

        assert_eq!(count_literals(&sequences), 9);
        assert_eq!(count_literals(&[]), 0);
    }

    #[test]
    fn decompressed_size_adds_literal_size_and_match_lengths_like_c() {
        let sequences = [sequence(2, 5), sequence(0, 3), sequence(7, 11)];

        assert_eq!(decompressed_size(&sequences, 9, false), 28);
    }

    #[test]
    fn decompressed_size_allows_last_sub_block_to_include_last_literals() {
        let sequences = [sequence(2, 5), sequence(0, 3), sequence(7, 11)];

        assert_eq!(decompressed_size(&sequences, 13, true), 32);
    }

    #[test]
    fn size_block_sequences_returns_one_when_first_sequence_exceeds_budget() {
        let sequences = [sequence(1, 3), sequence(1, 3)];

        assert_eq!(size_block_sequences(&sequences, 100, 256, 256, false), 1);
    }

    #[test]
    fn size_block_sequences_charges_first_sub_block_entropy_budget() {
        let sequences = [sequence(0, 3), sequence(0, 3)];

        assert_eq!(
            size_block_sequences(&sequences, ENTROPY_HEADER_BUDGET - 1, 0, 0, true),
            1
        );
    }

    #[test]
    fn size_block_sequences_returns_previous_count_when_next_sequence_trips_budget() {
        let sequences = [sequence(1, 3), sequence(1, 3), sequence(1, 3)];

        assert_eq!(size_block_sequences(&sequences, 800, 256, 256, false), 1);
    }

    #[test]
    fn size_block_sequences_keeps_expanding_until_sub_block_is_compressible() {
        let sequences = [sequence(1, 3), sequence(1, 3), sequence(1, 3)];

        assert_eq!(
            size_block_sequences(&sequences, 2_500, 1_024, 1_024, false),
            sequences.len()
        );
    }

    #[test]
    fn size_block_sequences_returns_all_when_budget_is_not_reached() {
        let sequences = [sequence(2, 5), sequence(3, 8), sequence(1, 4)];

        assert_eq!(
            size_block_sequences(&sequences, 10_000, 128, 192, false),
            sequences.len()
        );
    }
}
