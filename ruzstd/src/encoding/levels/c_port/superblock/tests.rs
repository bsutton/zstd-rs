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
fn target_superblock_acceptance_matches_c_minimum_gain_gate() {
    let src_size = 128 * 1024;
    let max_c_size = src_size - min_compression_gain(src_size, Strategy::Fast);

    assert!(should_accept_target_superblock(
        max_c_size + BLOCK_HEADER_SIZE - 1,
        src_size,
        Strategy::Fast
    ));
    assert!(!should_accept_target_superblock(
        max_c_size + BLOCK_HEADER_SIZE,
        src_size,
        Strategy::Fast
    ));
}

#[test]
fn target_superblock_acceptance_rejects_zero_like_c() {
    assert!(!should_accept_target_superblock(
        0,
        128 * 1024,
        Strategy::BtUltra2
    ));
}

#[test]
fn sub_block_literal_header_size_matches_c_thresholds_without_entropy() {
    assert_eq!(sub_block_literal_header_size(1023, false), 3);
    assert_eq!(sub_block_literal_header_size(1024, false), 4);
    assert_eq!(sub_block_literal_header_size((16 * 1024) - 1, false), 4);
    assert_eq!(sub_block_literal_header_size(16 * 1024, false), 5);
}

#[test]
fn sub_block_literal_header_size_reserves_entropy_header_guess_like_c() {
    assert_eq!(
        sub_block_literal_header_size(1024 - LITERAL_HEADER_ENTROPY_GUESS - 1, true),
        3
    );
    assert_eq!(
        sub_block_literal_header_size(1024 - LITERAL_HEADER_ENTROPY_GUESS, true),
        4
    );
    assert_eq!(
        sub_block_literal_header_size(16 * 1024 - LITERAL_HEADER_ENTROPY_GUESS - 1, true),
        4
    );
    assert_eq!(
        sub_block_literal_header_size(16 * 1024 - LITERAL_HEADER_ENTROPY_GUESS, true),
        5
    );
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
