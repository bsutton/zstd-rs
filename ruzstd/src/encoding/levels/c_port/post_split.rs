//! Post-sequence block splitter, following `ZSTD_deriveBlockSplits()`.

use alloc::vec::Vec;

use super::{
    block_emit::{append_prepared_block_or_raw, append_special_block, PreparedBlockEmission},
    block_policy::BlockEncodingPolicy,
    greedy_block::{GreedyBlockEncodeContext, GreedyEncodedBlock, GreedyPreparedBlock},
    params::Strategy,
    sequence_store::{OffBase, RepeatOffsets},
};
#[cfg(test)]
use crate::encoding::blocks::PreparedSequence;
use crate::{
    encoding::{
        blocks::{
            estimate_prepared_block_size, BlockCompressionConfig, PreparedBlock, PreparedBlockRef,
        },
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
    policy: BlockEncodingPolicy,
    strategy: Strategy,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    prepared: &GreedyPreparedBlock,
    previous_offsets: OffsetHistory,
    context: &mut GreedyBlockEncodeContext<'_, '_>,
) -> Option<GreedyEncodedBlock> {
    let partitions = derive_block_splits(
        block,
        &prepared.prepared,
        config,
        context.fse_tables,
        previous_offsets,
        context.previous_huff_table,
    );
    if partitions.len() <= 1 {
        return None;
    }

    let mut bytes = Vec::new();
    let mut last_huff_table = None;
    let mut decompression_repeat_offsets = repeat_offsets;
    let mut compression_repeat_offsets = repeat_offsets;

    let mut start_seq = 0usize;
    for (idx, &end_seq) in partitions.iter().enumerate() {
        let last_partition = idx + 1 == partitions.len();
        let mut chunk = prepared_chunk(block, &prepared.prepared, start_seq, end_seq);
        let decompression_repeat_offsets_before = decompression_repeat_offsets;
        resolve_partition_off_codes(
            &mut chunk.prepared,
            &mut decompression_repeat_offsets,
            &mut compression_repeat_offsets,
        );
        let encoded = encode_partition(
            &chunk.source,
            last_block && last_partition,
            decompression_repeat_offsets_before,
            chunk.prepared.as_ref(),
            PartitionEncodeContext {
                policy,
                strategy,
                config,
                fse_tables: context.fse_tables,
                offset_history: context.offset_history,
                previous_huff_table: last_huff_table.as_ref().or(context.previous_huff_table),
            },
        );
        bytes.extend_from_slice(&encoded.bytes);
        decompression_repeat_offsets = encoded.repeat_offsets;
        if encoded.new_huffman_table.is_some() {
            last_huff_table = encoded.new_huffman_table;
        }
        start_seq = end_seq;
    }

    Some(GreedyEncodedBlock {
        bytes,
        repeat_offsets: decompression_repeat_offsets,
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
    let chunk = prepared_chunk_ref(block, prepared, start_seq, end_seq);
    if chunk.source.is_empty() {
        return 3;
    }

    estimate_prepared_block_size(
        context.config,
        chunk.prepared,
        context.fse_tables,
        context.offset_history,
        context.previous_huff_table,
    )
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
    let chunk = prepared_chunk_ref(block, prepared, start_seq, end_seq);
    PreparedChunk {
        source: chunk.source.to_vec(),
        prepared: PreparedBlock {
            literals: chunk.prepared.literals.to_vec(),
            sequences: chunk.prepared.sequences.to_vec(),
        },
    }
}

struct PreparedChunkRef<'a> {
    source: &'a [u8],
    prepared: PreparedBlockRef<'a>,
}

fn prepared_chunk_ref<'a>(
    block: &'a [u8],
    prepared: &'a PreparedBlock,
    start_seq: usize,
    end_seq: usize,
) -> PreparedChunkRef<'a> {
    debug_assert!(start_seq <= end_seq);
    debug_assert!(end_seq <= prepared.sequences.len());

    let start = sequence_prefix(prepared, start_seq);
    let mut end = sequence_prefix(prepared, end_seq);
    if end_seq == prepared.sequences.len() {
        end.literal_pos = prepared.literals.len();
        end.source_pos = block.len();
    }

    PreparedChunkRef {
        source: &block[start.source_pos..end.source_pos],
        prepared: PreparedBlockRef {
            literals: &prepared.literals[start.literal_pos..end.literal_pos],
            sequences: &prepared.sequences[start_seq..end_seq],
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

fn resolve_partition_off_codes(
    prepared: &mut PreparedBlock,
    decompression_repeat_offsets: &mut RepeatOffsets,
    compression_repeat_offsets: &mut RepeatOffsets,
) {
    for sequence in &mut prepared.sequences {
        let original_off_base = sequence
            .encoded_offset_value
            .and_then(OffBase::from_c_value)
            .expect("C-port split partitions require C offBase values");
        let mut decompression_off_base = original_off_base;

        if matches!(original_off_base, OffBase::Repeat(_)) {
            let decompression_raw_offset =
                decompression_repeat_offsets.resolve(original_off_base, sequence.ll);
            let compression_raw_offset =
                compression_repeat_offsets.resolve(original_off_base, sequence.ll);
            if decompression_raw_offset != compression_raw_offset {
                decompression_off_base = OffBase::Offset(compression_raw_offset);
                sequence.raw_offset = compression_raw_offset;
                sequence.encoded_offset_value = Some(decompression_off_base.to_c_value());
            }
        }

        decompression_repeat_offsets.update(decompression_off_base, sequence.ll);
        compression_repeat_offsets.update(original_off_base, sequence.ll);
    }
}

fn encode_partition(
    block: &[u8],
    last_block: bool,
    repeat_offsets: RepeatOffsets,
    prepared: crate::encoding::blocks::PreparedBlockRef<'_>,
    context: PartitionEncodeContext<'_, '_>,
) -> GreedyEncodedBlock {
    let previous_fse = context.fse_tables.snapshot_previous();
    let previous_offsets = *context.offset_history;
    let mut bytes = Vec::new();

    if append_special_block(block, last_block, &mut bytes) {
        return GreedyEncodedBlock {
            bytes,
            repeat_offsets,
            new_huffman_table: None,
        };
    }

    match append_prepared_block_or_raw(
        block,
        last_block,
        context.strategy,
        context.policy,
        context.config,
        prepared,
        previous_fse,
        previous_offsets,
        context.previous_huff_table,
        context.fse_tables,
        context.offset_history,
        &mut bytes,
    ) {
        PreparedBlockEmission::Raw | PreparedBlockEmission::Rle => GreedyEncodedBlock {
            bytes,
            repeat_offsets,
            new_huffman_table: None,
        },
        PreparedBlockEmission::Compressed { new_huffman_table } => {
            let (newest, second, third) = context.offset_history.as_offsets();
            GreedyEncodedBlock {
                bytes,
                repeat_offsets: RepeatOffsets::from_offsets(newest, second, third),
                new_huffman_table,
            }
        }
    }
}

struct PartitionEncodeContext<'a, 'table> {
    policy: BlockEncodingPolicy,
    strategy: Strategy,
    config: BlockCompressionConfig,
    fse_tables: &'a mut FseTables,
    offset_history: &'a mut OffsetHistory,
    previous_huff_table: Option<&'table HuffmanTable>,
}

#[cfg(test)]
mod tests;
