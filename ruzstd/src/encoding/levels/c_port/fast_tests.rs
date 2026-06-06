use alloc::vec::Vec;

use super::fast::{
    compress_block_fast_no_dict, compress_block_fast_no_dict_with_state, FastMatchState,
};
use super::fast_block::{
    encode_block_fast_no_dict, prepare_block_fast_no_dict, FastBlockEncodeContext,
};
use super::fast_frame::{encode_frame_fast_no_dict, encode_single_block_frame_fast_no_dict};
use super::params::CompressionParameters;
use super::sequence_store::{OffBase, RepeatCode, RepeatOffsets, StoredSequence};
use crate::blocks::block::BlockType;
use crate::common::MAX_BLOCK_SIZE;
use crate::decoding::FrameDecoder;
use crate::encoding::blocks::BlockCompressionConfig;
use crate::encoding::frame_compressor::{FseTables, OffsetHistory};
use crate::encoding::CompressionLevel;

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

#[test]
fn fast_no_dict_state_finds_previous_block_prefix_match() {
    let marker = b"cross-block-stateful-marker:0123456789abcdef";
    let mut data = deterministic_bytes(MAX_BLOCK_SIZE as usize);
    for pos in [1024, 8192, MAX_BLOCK_SIZE as usize - 512] {
        data[pos..pos + marker.len()].copy_from_slice(marker);
    }
    let second_block_start = data.len();
    data.extend_from_slice(marker);
    data.extend_from_slice(&deterministic_bytes(256));

    let params = level1_params(data.len());
    let mut state = FastMatchState::new();
    let first = compress_block_fast_no_dict_with_state(
        &data,
        0..second_block_start,
        params,
        RepeatOffsets::new(),
        &mut state,
    );
    let second = compress_block_fast_no_dict_with_state(
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

#[test]
fn fast_no_dict_prepared_block_keeps_tiny_block_as_literals() {
    let data = b"abcdefgh";

    let prepared =
        prepare_block_fast_no_dict(data, level1_params(data.len()), RepeatOffsets::new());

    assert_eq!(prepared.prepared.literals, data);
    assert!(prepared.prepared.sequences.is_empty());
    assert_eq!(prepared.repeat_offsets, RepeatOffsets::new());
}

#[test]
fn fast_no_dict_prepared_block_resolves_repcode_raw_offset() {
    let data = b"aaaaaaaaaaaaaaaa";

    let prepared =
        prepare_block_fast_no_dict(data, level1_params(data.len()), RepeatOffsets::new());

    assert_eq!(prepared.prepared.literals, b"aa");
    assert_eq!(prepared.prepared.sequences.len(), 1);
    let sequence = prepared.prepared.sequences[0];
    assert_eq!(sequence.ll, 2);
    assert_eq!(sequence.ml, 14);
    assert_eq!(sequence.raw_offset, 1);
    assert_eq!(prepared.repeat_offsets.as_offsets(), [1, 4, 8]);
}

#[test]
fn fast_no_dict_prepared_block_reconstructs_literals_and_raw_offsets() {
    let data = b"abcdeabcdeabcde-tail";

    let prepared =
        prepare_block_fast_no_dict(data, level1_params(data.len()), RepeatOffsets::new());

    assert_eq!(prepared.prepared.literals, b"abcde-tail");
    assert_eq!(prepared.prepared.sequences.len(), 1);
    let sequence = prepared.prepared.sequences[0];
    assert_eq!(sequence.ll, 5);
    assert_eq!(sequence.ml, 10);
    assert_eq!(sequence.raw_offset, 5);
    assert_eq!(prepared.repeat_offsets.as_offsets(), [5, 1, 8]);
}

#[test]
fn fast_no_dict_hidden_block_emits_compressed_block() {
    let data = b"abcdeabcdeabcde-tail";
    let mut fse_tables = FseTables::new();
    let mut offset_history = OffsetHistory::new();

    let encoded = encode_block_fast_no_dict(
        data,
        true,
        level1_params(data.len()),
        BlockCompressionConfig::for_level(CompressionLevel::Fastest),
        RepeatOffsets::new(),
        FastBlockEncodeContext {
            previous_huff_table: None,
            fse_tables: &mut fse_tables,
            offset_history: &mut offset_history,
        },
    );
    let (last_block, block_type, block_size) = parse_block_header(&encoded.bytes);

    assert!(last_block);
    assert_eq!(block_type, BlockType::Compressed);
    assert_eq!(block_size as usize, encoded.bytes.len() - 3);
    assert_eq!(encoded.repeat_offsets.as_offsets(), [5, 1, 8]);
}

#[test]
fn fast_no_dict_hidden_block_falls_back_to_raw_when_not_smaller() {
    let data = b"abcdefgh";
    let mut fse_tables = FseTables::new();
    let mut offset_history = OffsetHistory::new();

    let encoded = encode_block_fast_no_dict(
        data,
        false,
        level1_params(data.len()),
        BlockCompressionConfig::for_level(CompressionLevel::Fastest),
        RepeatOffsets::new(),
        FastBlockEncodeContext {
            previous_huff_table: None,
            fse_tables: &mut fse_tables,
            offset_history: &mut offset_history,
        },
    );
    let (last_block, block_type, block_size) = parse_block_header(&encoded.bytes);

    assert!(!last_block);
    assert_eq!(block_type, BlockType::Raw);
    assert_eq!(block_size as usize, data.len());
    assert_eq!(&encoded.bytes[3..], data);
    assert_eq!(encoded.repeat_offsets, RepeatOffsets::new());
}

#[test]
fn fast_no_dict_hidden_frame_round_trips_compressed_block() {
    let data = b"abcdeabcdeabcde-tail";
    let encoded = encode_single_block_frame_fast_no_dict(data, 1);

    assert_round_trips(&encoded, data);
}

#[test]
fn fast_no_dict_hidden_frame_round_trips_raw_fallback_block() {
    let data = b"abcdefgh";
    let encoded = encode_single_block_frame_fast_no_dict(data, 1);

    assert_round_trips(&encoded, data);
}

#[test]
fn fast_no_dict_hidden_frame_round_trips_multiple_blocks() {
    let mut data = Vec::new();
    while data.len() < (MAX_BLOCK_SIZE as usize * 2) + 777 {
        data.extend_from_slice(b"tenant=alpha route=/archive status=200 bytes=4812\n");
    }
    data.truncate((MAX_BLOCK_SIZE as usize * 2) + 777);

    let encoded = encode_frame_fast_no_dict(&data, 1);

    assert!(count_frame_blocks(&encoded) > 1);
    assert_round_trips(&encoded, &data);
}

fn parse_block_header(bytes: &[u8]) -> (bool, BlockType, u32) {
    assert!(bytes.len() >= 3);
    let raw = u32::from(bytes[0]) | (u32::from(bytes[1]) << 8) | (u32::from(bytes[2]) << 16);
    let block_type = match (raw >> 1) & 0b11 {
        0 => BlockType::Raw,
        1 => BlockType::RLE,
        2 => BlockType::Compressed,
        _ => BlockType::Reserved,
    };
    (raw & 1 != 0, block_type, raw >> 3)
}

fn assert_round_trips(encoded: &[u8], expected: &[u8]) {
    let mut decoded = Vec::with_capacity(expected.len());
    FrameDecoder::new()
        .decode_all_to_vec(encoded, &mut decoded)
        .unwrap();

    assert_eq!(decoded, expected);
}

fn count_frame_blocks(encoded: &[u8]) -> usize {
    let (_, frame_header_size) =
        crate::decoding::frame::read_frame_header(encoded).expect("frame header should parse");
    let mut block_decoder = crate::decoding::block_decoder::new();
    let mut offset = frame_header_size as usize;
    let mut blocks = 0;

    loop {
        let (header, block_header_size) = block_decoder
            .read_block_header(&encoded[offset..])
            .expect("block header should parse");
        offset += block_header_size as usize + header.content_size as usize;
        blocks += 1;

        if header.last_block {
            break blocks;
        }
    }
}

fn deterministic_bytes(len: usize) -> Vec<u8> {
    let mut state = 0x1234_5678_u32;
    let mut bytes = Vec::with_capacity(len);
    for _ in 0..len {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        bytes.push(state as u8);
    }
    bytes
}
