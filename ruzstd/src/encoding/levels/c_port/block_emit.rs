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
    Rle,
    Compressed {
        new_huffman_table: Option<HuffmanTable>,
    },
}

#[allow(clippy::too_many_arguments)]
pub(super) fn append_prepared_block_or_raw(
    block: &[u8],
    last_block: bool,
    strategy: Strategy,
    policy: BlockEncodingPolicy,
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
        false,
    );
    let compressed_size = output.len() - compressed_start;

    if policy.allows_rle() && compressed_size < 25 && rle_byte(block).is_some() {
        output.truncate(block_start);
        fse_tables.restore_previous(previous_fse);
        *offset_history = previous_offsets;
        write_rle_block(last_block, block.len() as u32, block[0], output);
        PreparedBlockEmission::Rle
    } else if compression_result.should_emit_raw_block
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

pub(super) fn append_special_block(block: &[u8], last_block: bool, output: &mut Vec<u8>) -> bool {
    if block.is_empty() {
        write_raw_block(last_block, 0, block, output);
        return true;
    }

    if should_skip_sequence_build(block.len()) {
        write_raw_block(last_block, block.len() as u32, block, output);
        return true;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        blocks::block::BlockType,
        decoding::block_decoder,
        encoding::{
            blocks::PreparedBlock,
            frame_compressor::{FseTables, OffsetHistory},
        },
    };

    #[test]
    fn rle_is_emitted_after_prepared_compression_without_confirming_state() {
        let block = alloc::vec![b'a'; 4096];
        let prepared = PreparedBlock {
            literals: block.clone(),
            sequences: Vec::new(),
        };
        let mut fse_tables = FseTables::new();
        let previous_fse = fse_tables.snapshot_previous();
        let mut offset_history = OffsetHistory::from_offsets(11, 22, 33);
        let previous_offsets = offset_history;
        let mut output = Vec::new();

        let emission = append_prepared_block_or_raw(
            &block,
            true,
            Strategy::Fast,
            BlockEncodingPolicy::normal(),
            BlockCompressionConfig::for_c_strategy(Strategy::Fast as u8),
            prepared.as_ref(),
            previous_fse,
            previous_offsets,
            None,
            &mut fse_tables,
            &mut offset_history,
            &mut output,
        );

        assert!(matches!(emission, PreparedBlockEmission::Rle));
        assert_eq!(offset_history, previous_offsets);
        assert!(fse_tables.ll_previous.is_none());
        assert!(fse_tables.ml_previous.is_none());
        assert!(fse_tables.of_previous.is_none());

        let (header, header_size) = block_decoder::new()
            .read_block_header(output.as_slice())
            .expect("RLE block header should parse");
        assert_eq!(header_size, 3);
        assert!(header.last_block);
        assert_eq!(header.block_type, BlockType::RLE);
        assert_eq!(header.decompressed_size, block.len() as u32);
        assert_eq!(header.content_size, 1);
        assert_eq!(output[3], b'a');
    }
}
