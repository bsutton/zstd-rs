use alloc::vec::Vec;

use super::{
    greedy::{GreedyBlockOutput, GreedyMatchState},
    greedy_dict::{load_binary_tree_prefix, load_prefix},
    greedy_ext::{
        compress_block_btlazy2_ext_dict_with_state, compress_block_greedy_ext_dict_with_state,
        compress_block_lazy2_ext_dict_with_state, compress_block_lazy_ext_dict_with_state,
    },
    params::{CompressionParameters, Strategy},
    sequence_store::{OffBase, RepeatCode, RepeatOffsets},
};

fn chain_params(strategy: Strategy) -> CompressionParameters {
    CompressionParameters {
        window_log: 14,
        chain_log: 13,
        hash_log: 15,
        search_log: 4,
        min_match: 4,
        target_length: 0,
        strategy,
    }
}

fn btlazy2_params() -> CompressionParameters {
    CompressionParameters {
        window_log: 17,
        chain_log: 18,
        hash_log: 17,
        search_log: 5,
        min_match: 4,
        target_length: 8,
        strategy: Strategy::BtLazy2,
    }
}

#[test]
fn greedy_ext_dict_rejects_overlap_repcode_like_c() {
    let dict = b"abcdefgh";
    let source = b"XfghXabcdefgh";
    let mut combined = Vec::new();
    combined.extend_from_slice(dict);
    combined.extend_from_slice(source);

    let params = chain_params(Strategy::Greedy);
    let mut state = GreedyMatchState::new();
    load_prefix(&mut state, &combined, dict.len(), params);
    let output = compress_block_greedy_ext_dict_with_state(
        &combined,
        dict.len()..combined.len(),
        dict.len(),
        params,
        RepeatOffsets::from_offsets(4, 3, 7),
        &mut state,
        dict.len(),
    );

    assert!(!output
        .sequences
        .first()
        .is_some_and(|seq| matches!(seq.off_base, OffBase::Repeat(RepeatCode::First))));
}

#[test]
fn greedy_ext_dict_finds_dictionary_offset_match() {
    let output = run_dictionary_match(Strategy::Greedy);

    assert_has_offset_match(&output);
}

#[test]
fn lazy_ext_dict_finds_dictionary_offset_match() {
    let output = run_dictionary_match(Strategy::Lazy);

    assert_has_offset_match(&output);
}

#[test]
fn lazy2_ext_dict_finds_dictionary_offset_match() {
    let output = run_dictionary_match(Strategy::Lazy2);

    assert_has_offset_match(&output);
}

#[test]
fn btlazy2_ext_dict_finds_dictionary_offset_match() {
    let dict = b"prefix:shared-payload";
    let source = b"shared-payload shared-payload";
    let mut combined = Vec::new();
    combined.extend_from_slice(dict);
    combined.extend_from_slice(source);

    let params = btlazy2_params();
    let mut state = GreedyMatchState::new();
    load_binary_tree_prefix(&mut state, &combined, dict.len(), params);
    let output = compress_block_btlazy2_ext_dict_with_state(
        &combined,
        dict.len()..combined.len(),
        dict.len(),
        params,
        RepeatOffsets::new(),
        &mut state,
        dict.len(),
    );

    assert_has_offset_match(&output);
}

fn run_dictionary_match(strategy: Strategy) -> GreedyBlockOutput {
    let dict = b"prefix:shared-payload";
    let source = b"shared-payload shared-payload";
    let mut combined = Vec::new();
    combined.extend_from_slice(dict);
    combined.extend_from_slice(source);

    let params = chain_params(strategy);
    let mut state = GreedyMatchState::new();
    load_prefix(&mut state, &combined, dict.len(), params);
    match strategy {
        Strategy::Greedy => compress_block_greedy_ext_dict_with_state(
            &combined,
            dict.len()..combined.len(),
            dict.len(),
            params,
            RepeatOffsets::new(),
            &mut state,
            dict.len(),
        ),
        Strategy::Lazy => compress_block_lazy_ext_dict_with_state(
            &combined,
            dict.len()..combined.len(),
            dict.len(),
            params,
            RepeatOffsets::new(),
            &mut state,
            dict.len(),
        ),
        Strategy::Lazy2 => compress_block_lazy2_ext_dict_with_state(
            &combined,
            dict.len()..combined.len(),
            dict.len(),
            params,
            RepeatOffsets::new(),
            &mut state,
            dict.len(),
        ),
        _ => unreachable!("test only covers hash-chain lazy strategies"),
    }
}

fn assert_has_offset_match(output: &GreedyBlockOutput) {
    assert!(output
        .sequences
        .iter()
        .any(|seq| matches!(seq.off_base, OffBase::Offset(_))));
}
