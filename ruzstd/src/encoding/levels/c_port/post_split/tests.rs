use alloc::{vec, vec::Vec};

use super::*;
use crate::blocks::block::BlockType;

#[test]
fn prepared_chunk_splits_literals_and_source_span() {
    let prepared = PreparedBlock {
        literals: b"aabbtail".to_vec(),
        sequences: vec![
            PreparedSequence {
                ll: 2,
                ml: 3,
                raw_offset: 4,
                encoded_offset_value: None,
            },
            PreparedSequence {
                ll: 2,
                ml: 5,
                raw_offset: 7,
                encoded_offset_value: None,
            },
        ],
    };
    let block = b"aa123bb45678tail";

    let first = prepared_chunk(block, &prepared, 0, 1);
    assert_eq!(first.source, b"aa123");
    assert_eq!(first.prepared.literals, b"aa");
    assert_eq!(first.prepared.sequences.len(), 1);

    let second = prepared_chunk(block, &prepared, 1, 2);
    assert_eq!(second.source, b"bb45678tail");
    assert_eq!(second.prepared.literals, b"bbtail");
    assert_eq!(second.prepared.sequences.len(), 1);
}

#[test]
fn partition_offcode_resolution_rewrites_repcodes_after_raw_partition_like_c() {
    let mut first_partition = PreparedBlock {
        literals: b"a".to_vec(),
        sequences: vec![PreparedSequence {
            ll: 1,
            ml: 3,
            raw_offset: 4,
            encoded_offset_value: Some(OffBase::offset_to_c_value(4)),
        }],
    };
    let mut second_partition = PreparedBlock {
        literals: b"b".to_vec(),
        sequences: vec![PreparedSequence {
            ll: 1,
            ml: 3,
            raw_offset: 4,
            encoded_offset_value: Some(1),
        }],
    };
    let initial_repeats = RepeatOffsets::new();
    let mut compression_repeats = initial_repeats;
    let mut decompression_repeats = initial_repeats;

    resolve_partition_off_codes(
        &mut first_partition,
        &mut decompression_repeats,
        &mut compression_repeats,
    );
    decompression_repeats = initial_repeats;

    resolve_partition_off_codes(
        &mut second_partition,
        &mut decompression_repeats,
        &mut compression_repeats,
    );

    assert_eq!(
        second_partition.sequences[0].encoded_offset_value,
        Some(OffBase::offset_to_c_value(4))
    );
    assert_eq!(second_partition.sequences[0].raw_offset, 4);
    assert_eq!(decompression_repeats.as_offsets(), [4, 1, 4]);
    assert_eq!(compression_repeats.as_offsets(), [4, 1, 4]);
}

#[test]
fn derive_block_splits_refuses_tiny_sequence_counts() {
    let prepared = PreparedBlock {
        literals: vec![b'a'; 8],
        sequences: vec![
            PreparedSequence {
                ll: 1,
                ml: 3,
                raw_offset: 1,
                encoded_offset_value: None,
            };
            4
        ],
    };
    let splits = derive_block_splits(
        &[b'a'; 20],
        &prepared,
        BlockCompressionConfig::for_c_strategy(7),
        &FseTables::new(),
        OffsetHistory::new(),
        None,
    );

    assert!(splits.is_empty());
}

#[test]
fn derive_block_splits_finds_cheaper_halves() {
    let mut block = Vec::new();
    let mut literals = Vec::new();
    let mut sequences = Vec::new();
    for idx in 0..600 {
        let literal = if idx < 300 { b'a' } else { b'z' };
        block.extend_from_slice(&[literal; 4]);
        literals.push(literal);
        sequences.push(PreparedSequence {
            ll: 1,
            ml: 3,
            raw_offset: 1,
            encoded_offset_value: None,
        });
    }
    let prepared = PreparedBlock {
        literals,
        sequences,
    };

    let splits = derive_block_splits(
        &block,
        &prepared,
        BlockCompressionConfig::for_c_strategy(7),
        &FseTables::new(),
        OffsetHistory::new(),
        None,
    );

    assert!(splits.contains(&300));
    assert_eq!(splits.last(), Some(&600));
}

#[test]
fn partition_rle_obeys_first_block_policy_like_c() {
    let encoded = encode_rle_candidate_partition(BlockEncodingPolicy::frame_first_block());
    assert_ne!(parse_block_header(&encoded.bytes).1, BlockType::RLE);
}

#[test]
fn partition_can_emit_rle_after_first_block_like_c() {
    let encoded = encode_rle_candidate_partition(BlockEncodingPolicy::normal());
    let (last_block, block_type, block_size) = parse_block_header(&encoded.bytes);

    assert!(last_block);
    assert_eq!(block_type, BlockType::RLE);
    assert_eq!(block_size as usize, 32);
    assert_eq!(encoded.bytes, [0x03, 0x01, 0x00, 0x44]);
}

fn encode_rle_candidate_partition(policy: BlockEncodingPolicy) -> GreedyEncodedBlock {
    let block = [0x44; 32];
    let prepared = PreparedBlock {
        literals: block.to_vec(),
        sequences: Vec::new(),
    };
    let mut fse_tables = FseTables::new();
    let mut offset_history = OffsetHistory::new();

    encode_partition(
        &block,
        true,
        RepeatOffsets::new(),
        prepared.as_ref(),
        PartitionEncodeContext {
            policy,
            strategy: Strategy::BtOpt,
            config: BlockCompressionConfig::for_c_strategy(Strategy::BtOpt as u8),
            fse_tables: &mut fse_tables,
            offset_history: &mut offset_history,
            previous_huff_table: None,
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
