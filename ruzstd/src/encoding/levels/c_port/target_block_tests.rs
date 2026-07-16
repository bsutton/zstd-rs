use alloc::vec::Vec;

use super::{
    greedy_block::{GreedyBlockEncodeContext, GreedyPreparedBlock},
    sequence_store::RepeatOffsets,
    target_block::encode_target_block_with_superblock_fallback,
};
use crate::{
    blocks::block::BlockType,
    encoding::{
        blocks::{PreparedBlock, PreparedSequence},
        frame_compressor::{FseTables, OffsetHistory},
    },
};

#[test]
fn target_block_uses_huffman_literals_for_sequence_block() {
    let (data, prepared) = huffman_friendly_sequence_block();
    let mut fse_tables = FseTables::new();
    let mut offset_history = OffsetHistory::new();

    let encoded = encode_target_block_with_superblock_fallback(
        &data,
        true,
        2048,
        RepeatOffsets::new(),
        &prepared,
        GreedyBlockEncodeContext {
            previous_huff_table: None,
            fse_tables: &mut fse_tables,
            offset_history: &mut offset_history,
        },
        Vec::new(),
    );
    let (last_block, block_type, block_size) = parse_block_header(&encoded.bytes);

    assert!(last_block);
    assert_eq!(block_type, BlockType::Compressed);
    assert_eq!(block_size as usize, encoded.bytes.len() - 3);
    assert_eq!(decode_compressed_block(&encoded.bytes), data);
    assert!(encoded.new_huffman_table.is_some());
    assert!(fse_tables.ll_previous.is_some());
    assert!(fse_tables.ml_previous.is_some());
    assert!(fse_tables.of_previous.is_some());
}

#[test]
fn target_block_splits_huffman_literal_sequence_block_by_target_size() {
    let (data, prepared) = split_friendly_sequence_block();
    let mut fse_tables = FseTables::new();
    let mut offset_history = OffsetHistory::new();

    let encoded = encode_target_block_with_superblock_fallback(
        &data,
        true,
        1340,
        RepeatOffsets::new(),
        &prepared,
        GreedyBlockEncodeContext {
            previous_huff_table: None,
            fse_tables: &mut fse_tables,
            offset_history: &mut offset_history,
        },
        Vec::new(),
    );
    let headers = parse_block_headers(&encoded.bytes);

    assert!(headers.len() > 1);
    assert!(headers
        .iter()
        .all(|header| header.0 == BlockType::Compressed));
    assert!(headers.last().is_some_and(|header| header.2));
    assert_eq!(decode_blocks(&encoded.bytes), data);
    assert!(encoded.new_huffman_table.is_some());
    assert!(fse_tables.ll_previous.is_some());
    assert!(fse_tables.ml_previous.is_some());
    assert!(fse_tables.of_previous.is_some());
}

#[test]
fn target_block_splits_basic_literal_sequence_block_by_target_size() {
    let (data, prepared) = high_entropy_literal_sequence_block();
    let mut fse_tables = FseTables::new();
    let mut offset_history = OffsetHistory::new();

    let encoded = encode_target_block_with_superblock_fallback(
        &data,
        true,
        1340,
        RepeatOffsets::new(),
        &prepared,
        GreedyBlockEncodeContext {
            previous_huff_table: None,
            fse_tables: &mut fse_tables,
            offset_history: &mut offset_history,
        },
        Vec::new(),
    );
    let headers = parse_block_headers(&encoded.bytes);

    assert!(headers.len() > 1);
    assert!(headers
        .iter()
        .all(|header| header.0 == BlockType::Compressed));
    assert!(headers.last().is_some_and(|header| header.2));
    assert_eq!(decode_blocks(&encoded.bytes), data);
    assert!(encoded.new_huffman_table.is_none());
    assert!(fse_tables.ll_previous.is_some());
    assert!(fse_tables.ml_previous.is_some());
    assert!(fse_tables.of_previous.is_some());
}

#[test]
fn target_block_emits_raw_tail_when_final_subblock_is_not_worthwhile() {
    let (data, prepared) = raw_tail_sequence_block();
    let mut fse_tables = FseTables::new();
    let mut offset_history = OffsetHistory::new();

    let encoded = encode_target_block_with_superblock_fallback(
        &data,
        true,
        1340,
        RepeatOffsets::new(),
        &prepared,
        GreedyBlockEncodeContext {
            previous_huff_table: None,
            fse_tables: &mut fse_tables,
            offset_history: &mut offset_history,
        },
        Vec::new(),
    );
    let headers = parse_block_headers(&encoded.bytes);

    assert!(headers.len() > 1);
    assert!(headers[..headers.len() - 1]
        .iter()
        .all(|header| header.0 == BlockType::Compressed));
    assert_eq!(headers.last().map(|header| header.0), Some(BlockType::Raw));
    assert!(headers.last().is_some_and(|header| header.2));
    assert_eq!(decode_blocks(&encoded.bytes), data);
}

fn huffman_friendly_sequence_block() -> (Vec<u8>, GreedyPreparedBlock) {
    let mut data = Vec::new();
    let mut literals = Vec::new();
    let mut sequences = Vec::new();

    for idx in 0..160 {
        let chunk = [
            b'a' + (idx % 4) as u8,
            b'a' + ((idx + 1) % 4) as u8,
            b'0' + (idx % 2) as u8,
            b'\n',
        ];
        literals.extend_from_slice(&chunk);
        data.extend_from_slice(&chunk);
        data.extend_from_slice(&chunk);
        sequences.push(PreparedSequence {
            ll: chunk.len() as u32,
            ml: chunk.len() as u32,
            raw_offset: chunk.len() as u32,
            encoded_offset_value: None,
        });
    }

    (
        data,
        GreedyPreparedBlock {
            prepared: PreparedBlock {
                literals,
                sequences,
            },
            repeat_offsets: RepeatOffsets::new(),
        },
    )
}

fn split_friendly_sequence_block() -> (Vec<u8>, GreedyPreparedBlock) {
    let mut data = Vec::new();
    let mut literals = Vec::new();
    let mut sequences = Vec::new();

    for idx in 0..240 {
        let mut chunk = [0u8; 16];
        for (byte_idx, byte) in chunk.iter_mut().enumerate() {
            *byte = b'A' + ((idx + byte_idx) % 26) as u8;
        }
        literals.extend_from_slice(&chunk);
        data.extend_from_slice(&chunk);
        data.extend_from_slice(&chunk);
        sequences.push(PreparedSequence {
            ll: chunk.len() as u32,
            ml: chunk.len() as u32,
            raw_offset: chunk.len() as u32,
            encoded_offset_value: None,
        });
    }

    (
        data,
        GreedyPreparedBlock {
            prepared: PreparedBlock {
                literals,
                sequences,
            },
            repeat_offsets: RepeatOffsets::new(),
        },
    )
}

fn high_entropy_literal_sequence_block() -> (Vec<u8>, GreedyPreparedBlock) {
    let mut data = Vec::new();
    let mut literals = Vec::new();
    let mut sequences = Vec::new();
    let mut state = 0xA511_E9B3_u32;

    for _ in 0..240 {
        let mut chunk = [0u8; 16];
        for byte in &mut chunk {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            *byte = state as u8;
        }
        literals.extend_from_slice(&chunk);
        data.extend_from_slice(&chunk);
        data.extend_from_slice(&chunk);
        sequences.push(PreparedSequence {
            ll: chunk.len() as u32,
            ml: chunk.len() as u32,
            raw_offset: chunk.len() as u32,
            encoded_offset_value: None,
        });
    }

    (
        data,
        GreedyPreparedBlock {
            prepared: PreparedBlock {
                literals,
                sequences,
            },
            repeat_offsets: RepeatOffsets::new(),
        },
    )
}

fn raw_tail_sequence_block() -> (Vec<u8>, GreedyPreparedBlock) {
    let mut data = Vec::new();
    let mut literals = Vec::new();
    let mut sequences = Vec::new();

    for idx in 0..80 {
        let mut chunk = [0u8; 16];
        for (byte_idx, byte) in chunk.iter_mut().enumerate() {
            *byte = b'a' + ((idx + byte_idx) % 6) as u8;
        }
        literals.extend_from_slice(&chunk);
        data.extend_from_slice(&chunk);
        data.extend_from_slice(&chunk);
        sequences.push(PreparedSequence {
            ll: chunk.len() as u32,
            ml: chunk.len() as u32,
            raw_offset: chunk.len() as u32,
            encoded_offset_value: None,
        });
    }

    let mut state = 0xC0FF_EE31_u32;
    for _ in 0..50_000 {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        let byte = state as u8;
        literals.push(byte);
        data.push(byte);
    }

    (
        data,
        GreedyPreparedBlock {
            prepared: PreparedBlock {
                literals,
                sequences,
            },
            repeat_offsets: RepeatOffsets::new(),
        },
    )
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

fn parse_block_headers(bytes: &[u8]) -> Vec<(BlockType, usize, bool)> {
    let mut headers = Vec::new();
    let mut pos = 0usize;
    while pos < bytes.len() {
        let (last_block, block_type, block_size) = parse_block_header(&bytes[pos..]);
        headers.push((block_type, block_size as usize, last_block));
        pos += 3 + block_size as usize;
    }
    assert_eq!(pos, bytes.len());
    headers
}

fn decode_blocks(encoded: &[u8]) -> Vec<u8> {
    let mut block_decoder = crate::decoding::block_decoder::new();
    let mut scratch = crate::decoding::scratch::DecoderScratch::new(128 * 1024);
    let mut decoded = Vec::new();
    let mut pos = 0usize;

    while pos < encoded.len() {
        let (header, header_size) = block_decoder
            .read_block_header(&encoded[pos..])
            .expect("block header should parse");
        let content_start = pos + header_size as usize;
        let content_end = content_start + header.content_size as usize;
        match header.block_type {
            BlockType::Compressed => {
                block_decoder
                    .decode_block_content(
                        &header,
                        &mut scratch,
                        &encoded[content_start..content_end],
                    )
                    .expect("compressed block should decode");
                decoded.extend(scratch.buffer.drain());
            }
            BlockType::Raw => decoded.extend_from_slice(&encoded[content_start..content_end]),
            other => panic!("unexpected block type {:?}", other),
        }
        pos = content_end;
    }

    decoded
}

fn decode_compressed_block(encoded: &[u8]) -> Vec<u8> {
    let mut block_decoder = crate::decoding::block_decoder::new();
    let (header, header_size) = block_decoder
        .read_block_header(encoded)
        .expect("block header should parse");
    assert_eq!(header.block_type, BlockType::Compressed);
    assert_eq!(
        header.content_size as usize,
        encoded.len() - header_size as usize
    );

    let mut scratch = crate::decoding::scratch::DecoderScratch::new(128 * 1024);
    block_decoder
        .decode_block_content(&header, &mut scratch, &encoded[header_size as usize..])
        .expect("compressed block should decode");
    scratch.buffer.drain()
}
