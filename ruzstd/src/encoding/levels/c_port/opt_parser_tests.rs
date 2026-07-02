use super::{
    ldm::{opt::LdmOptCursor, sequence::LdmRawSequence},
    opt_block::{compress_block_btopt_no_dict, compress_block_btultra_no_dict},
    opt_match::OptMatch,
    opt_parser::collect_matches,
    opt_state::OptBlockState,
    params::{CompressionParameters, Strategy},
    sequence_store::RepeatOffsets,
};
use alloc::vec::Vec;

#[test]
fn btopt_parser_emits_sequences_for_repeated_data() {
    let data = b"alpha beta gamma alpha beta gamma alpha beta gamma";
    let params = btopt_params(data.len());

    let output = compress_block_btopt_no_dict(data, params, RepeatOffsets::new());

    assert!(!output.sequences.is_empty());
    assert!(output.last_literals < data.len() as u32);
}

#[test]
fn btultra_parser_emits_sequences_for_repeated_data() {
    let data = b"alpha beta gamma alpha beta gamma alpha beta gamma";
    let params = btultra_params(data.len());

    let output = compress_block_btultra_no_dict(data, params, RepeatOffsets::new());
    let covered = output
        .sequences
        .iter()
        .map(|sequence| sequence.lit_len + sequence.match_len)
        .sum::<u32>()
        + output.last_literals;

    assert_eq!(covered, data.len() as u32);
}

#[test]
fn btopt_parser_round_trips_sequence_coverage() {
    let data = b"GET /index.html 200 GET /index.html 200 GET /index.html 200";
    let params = btopt_params(data.len());
    let output = compress_block_btopt_no_dict(data, params, RepeatOffsets::new());

    let covered = output
        .sequences
        .iter()
        .map(|sequence| sequence.lit_len + sequence.match_len)
        .sum::<u32>()
        + output.last_literals;

    assert_eq!(covered, data.len() as u32);
}

#[test]
fn btopt_parser_state_spans_blocks() {
    let data = b"first-block-shared-payload first-block-shared-payload second-block-shared-payload first-block-shared-payload";
    let params = btopt_params(data.len());
    let split = data.len() / 2;
    let mut state = OptBlockState::new();

    let first = super::opt_block::compress_block_btopt_no_dict_with_state(
        data,
        0..split,
        params,
        RepeatOffsets::new(),
        &mut state,
    );
    let second = super::opt_block::compress_block_btopt_no_dict_with_state(
        data,
        split..data.len(),
        params,
        first.repeat_offsets,
        &mut state,
    );

    assert_eq!(
        first
            .sequences
            .iter()
            .map(|sequence| sequence.lit_len + sequence.match_len)
            .sum::<u32>()
            + first.last_literals,
        split as u32
    );
    assert_eq!(
        second
            .sequences
            .iter()
            .map(|sequence| sequence.lit_len + sequence.match_len)
            .sum::<u32>()
            + second.last_literals,
        (data.len() - split) as u32
    );
}

#[test]
fn btultra_parser_emits_matches_for_structured_data() {
    let mut data = Vec::new();
    for i in 0..2048u32 {
        use core::fmt::Write;
        let mut line = alloc::string::String::new();
        writeln!(
            &mut line,
            "route=/v1/pkg/{:04} status={:03} bytes={:04} tag={}{}{}{}",
            i % 97,
            200 + (i % 17),
            1000 + (i % 311),
            char::from_u32(65 + (i % 7)).unwrap(),
            char::from_u32(65 + ((i + 1) % 11)).unwrap(),
            char::from_u32(65 + ((i + 2) % 13)).unwrap(),
            char::from_u32(65 + ((i + 3) % 17)).unwrap(),
        )
        .unwrap();
        data.extend_from_slice(line.as_bytes());
    }

    let params = CompressionParameters::for_level(16, data.len() as u64, 0);
    let mut state = OptBlockState::new();
    state.reset_for_frame(params);
    let output = super::opt_block::compress_block_btultra_no_dict_with_state(
        &data,
        0..data.len(),
        params,
        RepeatOffsets::new(),
        &mut state,
    );
    let match_bytes = output
        .sequences
        .iter()
        .map(|sequence| sequence.match_len)
        .sum::<u32>();
    let covered = output
        .sequences
        .iter()
        .map(|sequence| sequence.lit_len + sequence.match_len)
        .sum::<u32>()
        + output.last_literals;

    assert_eq!(covered, data.len() as u32);
    assert!(!output.sequences.is_empty());
    assert!(match_bytes > data.len() as u32 / 2);
}

#[test]
fn opt_match_collection_appends_ldm_candidate_like_c() {
    let data = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let params = btopt_params(data.len());
    let mut state = OptBlockState::new();
    state.reset_for_frame(params);
    let sequences = [LdmRawSequence {
        offset: 4,
        lit_length: 4,
        match_length: 10,
    }];
    let mut cursor = LdmOptCursor::new(&sequences, data.len() as u32);

    let count = collect_matches(
        data,
        4,
        data.len(),
        RepeatOffsets::new().as_offsets(),
        false,
        4,
        params,
        &mut state,
        0,
        Some(&mut cursor),
    );

    assert_eq!(count, 1);
    assert_eq!(
        state.matches,
        [OptMatch {
            off_base: 7,
            len: 10,
        }]
    );
}

fn btopt_params(src_size: usize) -> CompressionParameters {
    let params = CompressionParameters::for_level(11, src_size as u64, 0);
    assert_eq!(params.strategy, Strategy::BtOpt);
    params
}

fn btultra_params(src_size: usize) -> CompressionParameters {
    let params = CompressionParameters::for_level(13, src_size as u64, 0);
    assert_eq!(params.strategy, Strategy::BtUltra);
    params
}
