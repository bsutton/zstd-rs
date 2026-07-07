//! Target-compressed-block-size helpers ported from
//! `zstd_compress_superblock.c`.

use alloc::vec::Vec;

use crate::encoding::blocks::PreparedSequence;

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
mod tests;
