//! Shared C-port block emission helpers.
//!
//! Match finders produce prepared sequences differently, but the final
//! raw/RLE/compressed block policy is shared by the C strategy adapters.

use alloc::vec::Vec;

use super::{
    block_policy::{
        compressed_block_is_worthwhile, should_skip_sequence_build, BlockEncodingPolicy,
    },
    params::Strategy,
};
use crate::{
    common::MAX_BLOCK_SIZE,
    encoding::{
        block_header::BlockHeader,
        blocks::{compress_prepared_block_with_stats, BlockCompressionConfig, PreparedBlockRef},
        frame_compressor::{FseTableSnapshot, FseTables, OffsetHistory},
    },
    huff0::huff0_encoder::HuffmanTable,
};

pub(super) enum PreparedBlockEmission {
    Raw,
    Compressed {
        new_huffman_table: Option<HuffmanTable>,
    },
}

#[allow(clippy::too_many_arguments)]
pub(super) fn append_prepared_block_or_raw(
    block: &[u8],
    last_block: bool,
    strategy: Strategy,
    config: BlockCompressionConfig,
    prepared: PreparedBlockRef<'_>,
    previous_fse: FseTableSnapshot,
    previous_offsets: OffsetHistory,
    previous_huff_table: Option<&HuffmanTable>,
    fse_tables: &mut FseTables,
    offset_history: &mut OffsetHistory,
    output: &mut Vec<u8>,
) -> PreparedBlockEmission {
    let block_start = output.len();
    output.extend_from_slice(&[0; 3]);
    let compressed_start = output.len();
    let compression_result = compress_prepared_block_with_stats(
        output,
        config,
        prepared,
        fse_tables,
        offset_history,
        previous_huff_table,
    );
    let compressed_size = output.len() - compressed_start;

    if compression_result.should_emit_raw_block
        || !compressed_block_is_worthwhile(block.len(), compressed_size, strategy)
        || compressed_size > MAX_BLOCK_SIZE as usize
    {
        output.truncate(block_start);
        fse_tables.restore_previous(previous_fse);
        *offset_history = previous_offsets;
        write_raw_block(last_block, block.len() as u32, block, output);
        PreparedBlockEmission::Raw
    } else {
        let header = BlockHeader {
            last_block,
            block_type: crate::blocks::block::BlockType::Compressed,
            block_size: compressed_size as u32,
        };
        output[block_start..compressed_start].copy_from_slice(&header.serialize_to_bytes());
        PreparedBlockEmission::Compressed {
            new_huffman_table: compression_result.new_huffman_table,
        }
    }
}

pub(super) fn append_special_block(
    block: &[u8],
    last_block: bool,
    policy: BlockEncodingPolicy,
    output: &mut Vec<u8>,
) -> bool {
    if block.is_empty() {
        write_raw_block(last_block, 0, block, output);
        return true;
    }

    if should_skip_sequence_build(block.len()) {
        write_raw_block(last_block, block.len() as u32, block, output);
        return true;
    }

    if policy.allows_rle() {
        if let Some(rle_byte) = rle_byte(block) {
            write_rle_block(last_block, block.len() as u32, rle_byte, output);
            return true;
        }
    }

    false
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
