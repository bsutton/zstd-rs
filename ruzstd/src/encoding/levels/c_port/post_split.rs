//! Post-sequence block splitter, following `ZSTD_deriveBlockSplits()`.

use alloc::vec::Vec;

use super::{
    greedy_block::{GreedyBlockEncodeContext, GreedyEncodedBlock, GreedyPreparedBlock},
    sequence_store::RepeatOffsets,
};
#[cfg(test)]
use crate::encoding::blocks::PreparedSequence;
use crate::{
    common::MAX_BLOCK_SIZE,
    encoding::{
        block_header::BlockHeader,
        blocks::{compress_prepared_block, BlockCompressionConfig, PreparedBlock},
        frame_compressor::{FseTables, OffsetHistory},
    },
    huff0::huff0_encoder::HuffmanTable,
};

const MIN_SEQUENCES_BLOCK_SPLITTING: usize = 300;
const MAX_NB_BLOCK_SPLITS: usize = 196;

#[allow(clippy::too_many_arguments)]
pub(super) fn encode_split_block(
    block: &[u8],
    last_block: bool,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    prepared: &GreedyPreparedBlock,
    previous_fse: FseTables,
    previous_offsets: OffsetHistory,
    context: &mut GreedyBlockEncodeContext<'_, '_>,
) -> Option<GreedyEncodedBlock> {
    let partitions = derive_block_splits(
        block,
        &prepared.prepared,
        config,
        &previous_fse,
        previous_offsets,
        context.previous_huff_table,
    );
    if partitions.len() <= 1 {
        return None;
    }

    let mut bytes = Vec::new();
    let mut last_huff_table = None;
    let mut next_repeat_offsets = repeat_offsets;

    let mut start_seq = 0usize;
    for (idx, &end_seq) in partitions.iter().enumerate() {
        let last_partition = idx + 1 == partitions.len();
        let chunk = prepared_chunk(block, &prepared.prepared, start_seq, end_seq);
        let encoded = encode_partition(
            &chunk.source,
            last_block && last_partition,
            next_repeat_offsets,
            chunk.prepared.as_ref(),
            PartitionEncodeContext {
                config,
                fse_tables: context.fse_tables,
                offset_history: context.offset_history,
                previous_huff_table: last_huff_table.as_ref().or(context.previous_huff_table),
            },
        );
        bytes.extend_from_slice(&encoded.bytes);
        next_repeat_offsets = encoded.repeat_offsets;
        if encoded.new_huffman_table.is_some() {
            last_huff_table = encoded.new_huffman_table;
        }
        start_seq = end_seq;
    }

    Some(GreedyEncodedBlock {
        bytes,
        repeat_offsets: next_repeat_offsets,
        new_huffman_table: last_huff_table,
    })
}

fn derive_block_splits(
    block: &[u8],
    prepared: &PreparedBlock,
    config: BlockCompressionConfig,
    fse_tables: &FseTables,
    offset_history: OffsetHistory,
    previous_huff_table: Option<&HuffmanTable>,
) -> Vec<usize> {
    let nb_seq = prepared.sequences.len();
    if nb_seq <= 4 {
        return Vec::new();
    }

    let mut splits = Vec::new();
    derive_block_splits_helper(
        &mut splits,
        0,
        nb_seq,
        block,
        prepared,
        EstimateContext {
            config,
            fse_tables,
            offset_history,
            previous_huff_table,
        },
    );
    splits.push(nb_seq);
    splits
}

#[allow(clippy::too_many_arguments)]
fn derive_block_splits_helper(
    splits: &mut Vec<usize>,
    start_idx: usize,
    end_idx: usize,
    block: &[u8],
    prepared: &PreparedBlock,
    context: EstimateContext<'_>,
) {
    if end_idx - start_idx < MIN_SEQUENCES_BLOCK_SPLITTING || splits.len() >= MAX_NB_BLOCK_SPLITS {
        return;
    }

    let mid_idx = (start_idx + end_idx) / 2;
    let original_size = estimate_partition_size(block, prepared, start_idx, end_idx, context);
    let first_half_size = estimate_partition_size(block, prepared, start_idx, mid_idx, context);
    let second_half_size = estimate_partition_size(block, prepared, mid_idx, end_idx, context);

    if first_half_size + second_half_size < original_size {
        derive_block_splits_helper(splits, start_idx, mid_idx, block, prepared, context);
        splits.push(mid_idx);
        derive_block_splits_helper(splits, mid_idx, end_idx, block, prepared, context);
    }
}

#[derive(Clone, Copy)]
struct EstimateContext<'a> {
    config: BlockCompressionConfig,
    fse_tables: &'a FseTables,
    offset_history: OffsetHistory,
    previous_huff_table: Option<&'a HuffmanTable>,
}

fn estimate_partition_size(
    block: &[u8],
    prepared: &PreparedBlock,
    start_seq: usize,
    end_seq: usize,
    context: EstimateContext<'_>,
) -> usize {
    let chunk = prepared_chunk(block, prepared, start_seq, end_seq);
    if chunk.source.is_empty() {
        return 3;
    }
    if rle_byte(&chunk.source).is_some() {
        return 4;
    }

    let mut bytes = Vec::new();
    let mut fse_tables = context.fse_tables.clone();
    let mut offset_history = context.offset_history;
    compress_prepared_block(
        &mut bytes,
        context.config,
        chunk.prepared.as_ref(),
        &mut fse_tables,
        &mut offset_history,
        context.previous_huff_table,
    );
    3 + bytes.len().min(chunk.source.len())
}

struct PreparedChunk {
    source: Vec<u8>,
    prepared: PreparedBlock,
}

fn prepared_chunk(
    block: &[u8],
    prepared: &PreparedBlock,
    start_seq: usize,
    end_seq: usize,
) -> PreparedChunk {
    debug_assert!(start_seq <= end_seq);
    debug_assert!(end_seq <= prepared.sequences.len());

    let start = sequence_prefix(prepared, start_seq);
    let mut end = sequence_prefix(prepared, end_seq);
    if end_seq == prepared.sequences.len() {
        end.literal_pos = prepared.literals.len();
        end.source_pos = block.len();
    }

    PreparedChunk {
        source: block[start.source_pos..end.source_pos].to_vec(),
        prepared: PreparedBlock {
            literals: prepared.literals[start.literal_pos..end.literal_pos].to_vec(),
            sequences: prepared.sequences[start_seq..end_seq].to_vec(),
        },
    }
}

#[derive(Clone, Copy)]
struct SequencePrefix {
    literal_pos: usize,
    source_pos: usize,
}

fn sequence_prefix(prepared: &PreparedBlock, seq_count: usize) -> SequencePrefix {
    let mut literal_pos = 0usize;
    let mut source_pos = 0usize;
    for sequence in prepared.sequences.iter().take(seq_count) {
        let lit_len = sequence.ll as usize;
        let match_len = sequence.ml as usize;
        literal_pos += lit_len;
        source_pos += lit_len + match_len;
    }
    SequencePrefix {
        literal_pos,
        source_pos,
    }
}

fn encode_partition(
    block: &[u8],
    last_block: bool,
    repeat_offsets: RepeatOffsets,
    prepared: crate::encoding::blocks::PreparedBlockRef<'_>,
    context: PartitionEncodeContext<'_, '_>,
) -> GreedyEncodedBlock {
    let previous_fse = context.fse_tables.clone();
    let previous_offsets = *context.offset_history;
    let mut bytes = Vec::new();

    if block.is_empty() {
        write_raw_block(last_block, 0, block, &mut bytes);
        return GreedyEncodedBlock {
            bytes,
            repeat_offsets,
            new_huffman_table: None,
        };
    }
    if let Some(rle_byte) = rle_byte(block) {
        write_rle_block(last_block, block.len() as u32, rle_byte, &mut bytes);
        return GreedyEncodedBlock {
            bytes,
            repeat_offsets,
            new_huffman_table: None,
        };
    }

    let block_start = bytes.len();
    bytes.extend_from_slice(&[0; 3]);
    let compressed_start = bytes.len();
    let new_huffman_table = compress_prepared_block(
        &mut bytes,
        context.config,
        prepared,
        context.fse_tables,
        context.offset_history,
        context.previous_huff_table,
    );
    let compressed_size = bytes.len() - compressed_start;

    if compressed_size >= block.len() || compressed_size > MAX_BLOCK_SIZE as usize {
        bytes.truncate(block_start);
        *context.fse_tables = previous_fse;
        *context.offset_history = previous_offsets;
        write_raw_block(last_block, block.len() as u32, block, &mut bytes);
        GreedyEncodedBlock {
            bytes,
            repeat_offsets,
            new_huffman_table: None,
        }
    } else {
        let header = BlockHeader {
            last_block,
            block_type: crate::blocks::block::BlockType::Compressed,
            block_size: compressed_size as u32,
        };
        bytes[block_start..compressed_start].copy_from_slice(&header.serialize_to_bytes());
        let (newest, second, third) = context.offset_history.as_offsets();
        GreedyEncodedBlock {
            bytes,
            repeat_offsets: RepeatOffsets::from_offsets(newest, second, third),
            new_huffman_table,
        }
    }
}

struct PartitionEncodeContext<'a, 'table> {
    config: BlockCompressionConfig,
    fse_tables: &'a mut FseTables,
    offset_history: &'a mut OffsetHistory,
    previous_huff_table: Option<&'table HuffmanTable>,
}

fn rle_byte(data: &[u8]) -> Option<u8> {
    let first = *data.first()?;
    data.iter().all(|byte| *byte == first).then_some(first)
}

fn write_rle_block(last_block: bool, block_size: u32, rle_byte: u8, output: &mut Vec<u8>) {
    let header = BlockHeader {
        last_block,
        block_type: crate::blocks::block::BlockType::RLE,
        block_size,
    };
    header.serialize(output);
    output.push(rle_byte);
}

fn write_raw_block(last_block: bool, block_size: u32, data: &[u8], output: &mut Vec<u8>) {
    let header = BlockHeader {
        last_block,
        block_type: crate::blocks::block::BlockType::Raw,
        block_size,
    };
    header.serialize(output);
    output.extend_from_slice(data);
}

#[cfg(test)]
mod tests {
    use alloc::{vec, vec::Vec};

    use super::*;

    #[test]
    fn prepared_chunk_splits_literals_and_source_span() {
        let prepared = PreparedBlock {
            literals: b"aabbtail".to_vec(),
            sequences: vec![
                PreparedSequence {
                    ll: 2,
                    ml: 3,
                    raw_offset: 4,
                },
                PreparedSequence {
                    ll: 2,
                    ml: 5,
                    raw_offset: 7,
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
    fn derive_block_splits_refuses_tiny_sequence_counts() {
        let prepared = PreparedBlock {
            literals: vec![b'a'; 8],
            sequences: vec![
                PreparedSequence {
                    ll: 1,
                    ml: 3,
                    raw_offset: 1,
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
}
