use super::{
    opt_parser::{compress_block_btopt_no_dict, OptBlockState},
    params::{CompressionParameters, Strategy},
    sequence_store::RepeatOffsets,
};

#[test]
fn btopt_parser_emits_sequences_for_repeated_data() {
    let data = b"alpha beta gamma alpha beta gamma alpha beta gamma";
    let params = btopt_params(data.len());

    let output = compress_block_btopt_no_dict(data, params, RepeatOffsets::new());

    assert!(!output.sequences.is_empty());
    assert!(output.last_literals < data.len() as u32);
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

    let first = super::opt_parser::compress_block_btopt_no_dict_with_state(
        data,
        0..split,
        params,
        RepeatOffsets::new(),
        &mut state,
    );
    let second = super::opt_parser::compress_block_btopt_no_dict_with_state(
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

fn btopt_params(src_size: usize) -> CompressionParameters {
    let params = CompressionParameters::for_level(11, src_size as u64, 0);
    assert_eq!(params.strategy, Strategy::BtOpt);
    params
}
