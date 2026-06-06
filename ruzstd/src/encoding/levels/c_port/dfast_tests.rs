use alloc::vec::Vec;

use super::dfast::{
    compress_block_double_fast_no_dict, compress_block_double_fast_no_dict_with_state,
    DFastMatchState,
};
use super::params::CompressionParameters;
use super::sequence_store::{OffBase, RepeatCode, RepeatOffsets, StoredSequence};
use crate::common::MAX_BLOCK_SIZE;

fn level3_params(src_len: usize) -> CompressionParameters {
    CompressionParameters::for_level(3, src_len as u64, 0)
}

#[test]
fn double_fast_no_dict_keeps_tiny_blocks_as_last_literals() {
    let data = b"abcdefgh";

    let output =
        compress_block_double_fast_no_dict(data, level3_params(data.len()), RepeatOffsets::new());

    assert!(output.sequences.is_empty());
    assert_eq!(output.last_literals, data.len() as u32);
    assert_eq!(output.repeat_offsets, RepeatOffsets::new());
}

#[test]
fn double_fast_no_dict_emits_repcode_at_next_position() {
    let data = b"aaaaaaaaaaaaaaaa";

    let output =
        compress_block_double_fast_no_dict(data, level3_params(data.len()), RepeatOffsets::new());

    assert_eq!(
        output.sequences,
        [StoredSequence::new(
            2,
            OffBase::Repeat(RepeatCode::First),
            14
        )]
    );
    assert_eq!(output.last_literals, 0);
    assert_eq!(output.repeat_offsets, RepeatOffsets::new());
}

#[test]
fn double_fast_no_dict_uses_long_match_over_short_match() {
    let data = b"abcde12345abcde12345-tail";

    let output =
        compress_block_double_fast_no_dict(data, level3_params(data.len()), RepeatOffsets::new());

    assert_eq!(
        output.sequences,
        [StoredSequence::new(10, OffBase::Offset(10), 10)]
    );
    assert_eq!(output.last_literals, 5);
    assert_eq!(output.repeat_offsets.as_offsets(), [10, 1, 8]);
}

#[test]
fn double_fast_state_finds_previous_block_prefix_match() {
    let marker = b"double-fast-cross-block-marker:0123456789abcdef";
    let mut data = deterministic_bytes(MAX_BLOCK_SIZE as usize);
    for pos in [2048, 16384, MAX_BLOCK_SIZE as usize - 1024] {
        data[pos..pos + marker.len()].copy_from_slice(marker);
    }
    let second_block_start = data.len();
    data.extend_from_slice(marker);
    data.extend_from_slice(&deterministic_bytes(512));

    let params = level3_params(data.len());
    let mut state = DFastMatchState::new();
    let first = compress_block_double_fast_no_dict_with_state(
        &data,
        0..second_block_start,
        params,
        RepeatOffsets::new(),
        &mut state,
    );
    let second = compress_block_double_fast_no_dict_with_state(
        &data,
        second_block_start..data.len(),
        params,
        first.repeat_offsets,
        &mut state,
    );

    assert!(second.sequences.iter().any(|sequence| matches!(
        sequence.off_base,
        OffBase::Offset(offset) if sequence.lit_len == 0
            && offset as usize >= marker.len()
    )));
}

fn deterministic_bytes(len: usize) -> Vec<u8> {
    let mut state = 0x9E37_79B9_u32;
    let mut bytes = Vec::with_capacity(len);
    for _ in 0..len {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        bytes.push(state as u8);
    }
    bytes
}
