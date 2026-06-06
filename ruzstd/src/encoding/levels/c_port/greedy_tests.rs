use alloc::vec::Vec;

use super::greedy::{
    compress_block_greedy_no_dict, compress_block_greedy_no_dict_with_state, GreedyMatchState,
};
use super::greedy_block::{
    encode_block_greedy_no_dict, prepare_block_greedy_no_dict, GreedyBlockEncodeContext,
};
use super::greedy_frame::{encode_frame_greedy_no_dict, encode_single_block_frame_greedy_no_dict};
use super::params::CompressionParameters;
use super::sequence_store::{OffBase, RepeatCode, RepeatOffsets, StoredSequence};
use crate::blocks::block::BlockType;
use crate::common::MAX_BLOCK_SIZE;
use crate::decoding::FrameDecoder;
use crate::encoding::blocks::BlockCompressionConfig;
use crate::encoding::frame_compressor::{FseTables, OffsetHistory};
use crate::encoding::CompressionLevel;

fn greedy_params(src_len: usize) -> CompressionParameters {
    CompressionParameters::for_level(greedy_level(src_len), src_len as u64, 0)
}

fn greedy_level(src_len: usize) -> i32 {
    if src_len <= 16 * 1024 {
        4
    } else {
        5
    }
}

#[test]
fn greedy_no_dict_keeps_tiny_blocks_as_last_literals() {
    let data = b"abcdefgh";

    let output =
        compress_block_greedy_no_dict(data, greedy_params(data.len()), RepeatOffsets::new());

    assert!(output.sequences.is_empty());
    assert_eq!(output.last_literals, data.len() as u32);
    assert_eq!(output.repeat_offsets, RepeatOffsets::new());
}

#[test]
fn greedy_no_dict_emits_repcode_at_next_position() {
    let data = b"aaaaaaaaaaaaaaaa";

    let output =
        compress_block_greedy_no_dict(data, greedy_params(data.len()), RepeatOffsets::new());

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
fn greedy_no_dict_uses_hash_chain_match() {
    let data = b"abcde12345abcde12345-tail";

    let output =
        compress_block_greedy_no_dict(data, greedy_params(data.len()), RepeatOffsets::new());

    assert_eq!(
        output.sequences,
        [StoredSequence::new(10, OffBase::Offset(10), 10)]
    );
    assert_eq!(output.last_literals, 5);
    assert_eq!(output.repeat_offsets.as_offsets(), [10, 1, 8]);
}

#[test]
fn greedy_no_dict_state_finds_previous_block_prefix_match() {
    let marker = b"greedy-cross-block-marker:0123456789abcdef";
    let mut data = deterministic_bytes(MAX_BLOCK_SIZE as usize);
    for pos in [4096, 24576, MAX_BLOCK_SIZE as usize - 1536] {
        data[pos..pos + marker.len()].copy_from_slice(marker);
    }
    let second_block_start = data.len();
    data.extend_from_slice(marker);
    data.extend_from_slice(&deterministic_bytes(512));

    let params = greedy_params(data.len());
    let mut state = GreedyMatchState::new();
    let first = compress_block_greedy_no_dict_with_state(
        &data,
        0..second_block_start,
        params,
        RepeatOffsets::new(),
        &mut state,
    );
    let second = compress_block_greedy_no_dict_with_state(
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
fn greedy_no_dict_prepared_block_resolves_sequences() {
    let data = b"abcde12345abcde12345-tail";

    let prepared =
        prepare_block_greedy_no_dict(data, greedy_params(data.len()), RepeatOffsets::new());

    assert_eq!(prepared.prepared.literals, b"abcde12345-tail");
    assert_eq!(prepared.prepared.sequences.len(), 1);
    let sequence = prepared.prepared.sequences[0];
    assert_eq!(sequence.ll, 10);
    assert_eq!(sequence.ml, 10);
    assert_eq!(sequence.raw_offset, 10);
    assert_eq!(prepared.repeat_offsets.as_offsets(), [10, 1, 8]);
}

#[test]
fn greedy_hidden_block_emits_compressed_block() {
    let data = b"abcde12345abcde12345-tail";
    let mut fse_tables = FseTables::new();
    let mut offset_history = OffsetHistory::new();

    let encoded = encode_block_greedy_no_dict(
        data,
        true,
        greedy_params(data.len()),
        BlockCompressionConfig::for_level(CompressionLevel::Default),
        RepeatOffsets::new(),
        GreedyBlockEncodeContext {
            previous_huff_table: None,
            fse_tables: &mut fse_tables,
            offset_history: &mut offset_history,
        },
    );
    let (last_block, block_type, block_size) = parse_block_header(&encoded.bytes);

    assert!(last_block);
    assert_eq!(block_type, BlockType::Compressed);
    assert_eq!(block_size as usize, encoded.bytes.len() - 3);
    assert_eq!(encoded.repeat_offsets.as_offsets(), [10, 1, 8]);
}

#[test]
fn greedy_hidden_block_falls_back_to_raw_when_not_smaller() {
    let data = b"abcdefgh";
    let mut fse_tables = FseTables::new();
    let mut offset_history = OffsetHistory::new();

    let encoded = encode_block_greedy_no_dict(
        data,
        false,
        greedy_params(data.len()),
        BlockCompressionConfig::for_level(CompressionLevel::Default),
        RepeatOffsets::new(),
        GreedyBlockEncodeContext {
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
fn greedy_hidden_block_emits_rle_for_single_byte_run() {
    let data = [0x6D; 256];
    let mut fse_tables = FseTables::new();
    let mut offset_history = OffsetHistory::new();

    let encoded = encode_block_greedy_no_dict(
        &data,
        true,
        greedy_params(data.len()),
        BlockCompressionConfig::for_level(CompressionLevel::Default),
        RepeatOffsets::new(),
        GreedyBlockEncodeContext {
            previous_huff_table: None,
            fse_tables: &mut fse_tables,
            offset_history: &mut offset_history,
        },
    );
    let (last_block, block_type, block_size) = parse_block_header(&encoded.bytes);

    assert!(last_block);
    assert_eq!(block_type, BlockType::RLE);
    assert_eq!(block_size as usize, data.len());
    assert_eq!(encoded.bytes, [0x03, 0x08, 0x00, 0x6D]);
    assert_eq!(encoded.repeat_offsets, RepeatOffsets::new());
}

#[test]
fn greedy_hidden_frame_round_trips_compressed_block() {
    let data = b"abcde12345abcde12345-tail";
    let encoded = encode_single_block_frame_greedy_no_dict(data, greedy_level(data.len()));

    assert_round_trips(&encoded, data);
}

#[test]
fn greedy_hidden_frame_round_trips_rle_block() {
    let data = [0x42; 4096];
    let encoded = encode_single_block_frame_greedy_no_dict(&data, greedy_level(data.len()));

    assert_round_trips(&encoded, &data);
}

#[test]
fn greedy_hidden_frame_round_trips_multiple_blocks() {
    let mut data = Vec::new();
    while data.len() < (MAX_BLOCK_SIZE as usize * 2) + 1536 {
        data.extend_from_slice(b"tenant=gamma method=POST route=/v3/items status=201 bytes=244\n");
    }
    data.truncate((MAX_BLOCK_SIZE as usize * 2) + 1536);

    let encoded = encode_frame_greedy_no_dict(&data, greedy_level(data.len()));

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
    let mut state = 0xA511_E9B3_u32;
    let mut bytes = Vec::with_capacity(len);
    for _ in 0..len {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        bytes.push(state as u8);
    }
    bytes
}
