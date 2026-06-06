use super::fast::compress_block_fast_no_dict;
use super::params::CompressionParameters;
use super::sequence_store::{OffBase, RepeatCode, RepeatOffsets, StoredSequence};

fn level1_params(src_len: usize) -> CompressionParameters {
    CompressionParameters::for_level(1, src_len as u64, 0)
}

#[test]
fn fast_no_dict_keeps_tiny_blocks_as_last_literals() {
    let data = b"abcdefgh";

    let output = compress_block_fast_no_dict(data, level1_params(data.len()), RepeatOffsets::new());

    assert!(output.sequences.is_empty());
    assert_eq!(output.last_literals, data.len() as u32);
    assert_eq!(output.repeat_offsets, RepeatOffsets::new());
}

#[test]
fn fast_no_dict_emits_offset_one_run_like_c_fast() {
    let data = b"aaaaaaaaaaaaaaaa";

    let output = compress_block_fast_no_dict(data, level1_params(data.len()), RepeatOffsets::new());

    assert_eq!(
        output.sequences,
        [StoredSequence::new(
            2,
            OffBase::Repeat(RepeatCode::First),
            14
        )]
    );
    assert_eq!(output.last_literals, 0);
    assert_eq!(output.repeat_offsets.as_offsets(), [1, 4, 8]);
}

#[test]
fn fast_no_dict_emits_repeated_pattern_match() {
    let data = b"abcdeabcdeabcde-tail";

    let output = compress_block_fast_no_dict(data, level1_params(data.len()), RepeatOffsets::new());

    assert_eq!(
        output.sequences,
        [StoredSequence::new(5, OffBase::Offset(5), 10)]
    );
    assert_eq!(output.last_literals, 5);
}

#[test]
fn fast_no_dict_extends_offset_match_before_immediate_repcode_probe() {
    let data = b"abcdabcdabcdabcdabcd";

    let output = compress_block_fast_no_dict(data, level1_params(data.len()), RepeatOffsets::new());

    assert_eq!(
        output.sequences,
        [StoredSequence::new(4, OffBase::Offset(4), 16)]
    );
    assert_eq!(output.last_literals, 0);
}
