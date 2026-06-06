//! Adapters from C greedy sequences to the existing Rust block encoder.

use alloc::vec::Vec;
use core::ops::Range;

use super::block_policy::{
    compressed_block_is_worthwhile, should_skip_sequence_build, BlockEncodingPolicy,
};
use super::greedy::{
    compress_block_btlazy2_no_dict_with_state, compress_block_greedy_no_dict_with_state,
    compress_block_lazy2_no_dict_with_state, compress_block_lazy_no_dict_with_state,
    GreedyBlockOutput, GreedyMatchState,
};
use super::params::CompressionParameters;
use super::sequence_store::RepeatOffsets;
use crate::{
    common::MAX_BLOCK_SIZE,
    encoding::{
        block_header::BlockHeader,
        blocks::{
            compress_prepared_block, BlockCompressionConfig, PreparedBlock, PreparedSequence,
        },
        frame_compressor::{FseTables, OffsetHistory},
    },
    huff0::huff0_encoder::HuffmanTable,
};

pub(crate) struct GreedyPreparedBlock {
    pub(crate) prepared: PreparedBlock,
    pub(crate) repeat_offsets: RepeatOffsets,
}

pub(crate) struct GreedyEncodedBlock {
    pub(crate) bytes: Vec<u8>,
    pub(crate) repeat_offsets: RepeatOffsets,
    pub(crate) new_huffman_table: Option<HuffmanTable>,
}

pub(crate) struct GreedyBlockEncodeContext<'a, 'table> {
    pub(crate) previous_huff_table: Option<&'table HuffmanTable>,
    pub(crate) fse_tables: &'a mut FseTables,
    pub(crate) offset_history: &'a mut OffsetHistory,
}

pub(crate) struct GreedyBlockSource<'a> {
    pub(crate) src: &'a [u8],
    pub(crate) block_range: Range<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LazyBlockStrategy {
    Greedy,
    Lazy,
    Lazy2,
    BtLazy2,
}

pub(crate) fn prepare_block_greedy_no_dict(
    src: &[u8],
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
) -> GreedyPreparedBlock {
    prepare_block_hash_chain_no_dict(src, params, repeat_offsets, LazyBlockStrategy::Greedy)
}

fn prepare_block_hash_chain_no_dict(
    src: &[u8],
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    depth: LazyBlockStrategy,
) -> GreedyPreparedBlock {
    let mut state = GreedyMatchState::new();
    let output = compress_block_for_depth_with_state(
        src,
        0..src.len(),
        params,
        repeat_offsets,
        &mut state,
        depth,
    );
    let prepared = prepare_from_greedy_output(src, repeat_offsets, &output);

    GreedyPreparedBlock {
        prepared,
        repeat_offsets: output.repeat_offsets,
    }
}

pub(crate) fn prepare_block_greedy_no_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
) -> GreedyPreparedBlock {
    prepare_block_hash_chain_no_dict_with_state(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        LazyBlockStrategy::Greedy,
    )
}

fn prepare_block_hash_chain_no_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
    depth: LazyBlockStrategy,
) -> GreedyPreparedBlock {
    let block = &src[block_range.clone()];
    let output =
        compress_block_for_depth_with_state(src, block_range, params, repeat_offsets, state, depth);
    let prepared = prepare_from_greedy_output(block, repeat_offsets, &output);

    GreedyPreparedBlock {
        prepared,
        repeat_offsets: output.repeat_offsets,
    }
}

fn compress_block_for_depth_with_state(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
    depth: LazyBlockStrategy,
) -> GreedyBlockOutput {
    match depth {
        LazyBlockStrategy::Greedy => compress_block_greedy_no_dict_with_state(
            src,
            block_range,
            params,
            repeat_offsets,
            state,
        ),
        LazyBlockStrategy::Lazy => {
            compress_block_lazy_no_dict_with_state(src, block_range, params, repeat_offsets, state)
        }
        LazyBlockStrategy::Lazy2 => {
            compress_block_lazy2_no_dict_with_state(src, block_range, params, repeat_offsets, state)
        }
        LazyBlockStrategy::BtLazy2 => compress_block_btlazy2_no_dict_with_state(
            src,
            block_range,
            params,
            repeat_offsets,
            state,
        ),
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn encode_block_hash_chain_no_dict(
    src: &[u8],
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    context: GreedyBlockEncodeContext<'_, '_>,
    depth: LazyBlockStrategy,
) -> GreedyEncodedBlock {
    encode_block_hash_chain_no_dict_with_policy(
        src,
        last_block,
        params,
        config,
        repeat_offsets,
        context,
        depth,
        BlockEncodingPolicy::normal(),
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn encode_block_hash_chain_no_dict_with_policy(
    src: &[u8],
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    context: GreedyBlockEncodeContext<'_, '_>,
    depth: LazyBlockStrategy,
    policy: BlockEncodingPolicy,
) -> GreedyEncodedBlock {
    let mut bytes = Vec::new();

    if let Some(encoded) = encode_special_block(src, last_block, repeat_offsets, policy, &mut bytes)
    {
        return encoded;
    }

    let previous_fse = context.fse_tables.clone();
    let previous_offsets = *context.offset_history;
    let prepared = prepare_block_hash_chain_no_dict(src, params, repeat_offsets, depth);
    encode_prepared_block(
        src,
        last_block,
        params,
        config,
        repeat_offsets,
        prepared,
        previous_fse,
        previous_offsets,
        context,
        bytes,
    )
}

pub(crate) fn encode_block_greedy_no_dict_with_state(
    source: GreedyBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    match_state: &mut GreedyMatchState,
    context: GreedyBlockEncodeContext<'_, '_>,
) -> GreedyEncodedBlock {
    encode_block_hash_chain_no_dict_with_state_and_policy(
        source,
        last_block,
        params,
        config,
        repeat_offsets,
        match_state,
        context,
        LazyBlockStrategy::Greedy,
        BlockEncodingPolicy::normal(),
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn encode_block_hash_chain_no_dict_with_state(
    source: GreedyBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    match_state: &mut GreedyMatchState,
    context: GreedyBlockEncodeContext<'_, '_>,
    depth: LazyBlockStrategy,
) -> GreedyEncodedBlock {
    encode_block_hash_chain_no_dict_with_state_and_policy(
        source,
        last_block,
        params,
        config,
        repeat_offsets,
        match_state,
        context,
        depth,
        BlockEncodingPolicy::normal(),
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn encode_block_hash_chain_no_dict_with_state_and_policy(
    source: GreedyBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    match_state: &mut GreedyMatchState,
    context: GreedyBlockEncodeContext<'_, '_>,
    depth: LazyBlockStrategy,
    policy: BlockEncodingPolicy,
) -> GreedyEncodedBlock {
    let block = &source.src[source.block_range.clone()];
    let mut bytes = Vec::new();

    if let Some(encoded) =
        encode_special_block(block, last_block, repeat_offsets, policy, &mut bytes)
    {
        return encoded;
    }

    let previous_fse = context.fse_tables.clone();
    let previous_offsets = *context.offset_history;
    let prepared = prepare_block_hash_chain_no_dict_with_state(
        source.src,
        source.block_range,
        params,
        repeat_offsets,
        match_state,
        depth,
    );
    encode_prepared_block(
        block,
        last_block,
        params,
        config,
        repeat_offsets,
        prepared,
        previous_fse,
        previous_offsets,
        context,
        bytes,
    )
}

#[allow(clippy::too_many_arguments)]
pub(super) fn encode_prepared_block(
    block: &[u8],
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    prepared: GreedyPreparedBlock,
    previous_fse: FseTables,
    previous_offsets: OffsetHistory,
    context: GreedyBlockEncodeContext<'_, '_>,
    mut bytes: Vec<u8>,
) -> GreedyEncodedBlock {
    let block_start = bytes.len();
    bytes.extend_from_slice(&[0; 3]);
    let compressed_start = bytes.len();
    let new_huffman_table = compress_prepared_block(
        &mut bytes,
        config,
        prepared.prepared.as_ref(),
        context.fse_tables,
        context.offset_history,
        context.previous_huff_table,
    );
    let compressed_size = bytes.len() - compressed_start;

    if !compressed_block_is_worthwhile(block.len(), compressed_size, params.strategy)
        || compressed_size > MAX_BLOCK_SIZE as usize
    {
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
        GreedyEncodedBlock {
            bytes,
            repeat_offsets: prepared.repeat_offsets,
            new_huffman_table,
        }
    }
}

pub(super) fn prepare_from_greedy_output(
    src: &[u8],
    initial_repeat_offsets: RepeatOffsets,
    output: &GreedyBlockOutput,
) -> PreparedBlock {
    let mut literals = Vec::new();
    let mut sequences = Vec::with_capacity(output.sequences.len());
    let mut repeat_offsets = initial_repeat_offsets;
    let mut anchor = 0_usize;

    for sequence in &output.sequences {
        let lit_len = sequence.lit_len as usize;
        let match_len = sequence.match_len as usize;
        let lit_end = anchor + lit_len;
        debug_assert!(lit_end <= src.len());
        literals.extend_from_slice(&src[anchor..lit_end]);

        let raw_offset = repeat_offsets.resolve(sequence.off_base, sequence.lit_len);
        sequences.push(PreparedSequence {
            ll: sequence.lit_len,
            ml: sequence.match_len,
            raw_offset,
        });
        repeat_offsets.update(sequence.off_base, sequence.lit_len);
        anchor = lit_end + match_len;
        debug_assert!(anchor <= src.len());
    }

    let tail_end = anchor + output.last_literals as usize;
    debug_assert_eq!(tail_end, src.len());
    literals.extend_from_slice(&src[anchor..tail_end]);

    PreparedBlock {
        literals,
        sequences,
    }
}

pub(super) fn encode_special_block(
    block: &[u8],
    last_block: bool,
    repeat_offsets: RepeatOffsets,
    policy: BlockEncodingPolicy,
    bytes: &mut Vec<u8>,
) -> Option<GreedyEncodedBlock> {
    if block.is_empty() {
        write_raw_block(last_block, 0, block, bytes);
        return Some(GreedyEncodedBlock {
            bytes: core::mem::take(bytes),
            repeat_offsets,
            new_huffman_table: None,
        });
    }

    if should_skip_sequence_build(block.len()) {
        write_raw_block(last_block, block.len() as u32, block, bytes);
        return Some(GreedyEncodedBlock {
            bytes: core::mem::take(bytes),
            repeat_offsets,
            new_huffman_table: None,
        });
    }

    if policy.allows_rle() {
        if let Some(rle_byte) = rle_byte(block) {
            write_rle_block(last_block, block.len() as u32, rle_byte, bytes);
            return Some(GreedyEncodedBlock {
                bytes: core::mem::take(bytes),
                repeat_offsets,
                new_huffman_table: None,
            });
        }
    }

    None
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
