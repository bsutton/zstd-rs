//! Adapters from C fast sequences to the existing Rust block encoder.

use alloc::vec::Vec;

use super::fast::{compress_block_fast_no_dict, FastBlockOutput};
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

pub(crate) struct FastPreparedBlock {
    pub(crate) prepared: PreparedBlock,
    pub(crate) repeat_offsets: RepeatOffsets,
}

pub(crate) struct FastEncodedBlock {
    pub(crate) bytes: Vec<u8>,
    pub(crate) repeat_offsets: RepeatOffsets,
    pub(crate) new_huffman_table: Option<HuffmanTable>,
}

pub(crate) struct FastBlockEncodeContext<'a, 'table> {
    pub(crate) previous_huff_table: Option<&'table HuffmanTable>,
    pub(crate) fse_tables: &'a mut FseTables,
    pub(crate) offset_history: &'a mut OffsetHistory,
}

pub(crate) fn prepare_block_fast_no_dict(
    src: &[u8],
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
) -> FastPreparedBlock {
    let output = compress_block_fast_no_dict(src, params, repeat_offsets);
    let prepared = prepare_from_fast_output(src, repeat_offsets, &output);

    FastPreparedBlock {
        prepared,
        repeat_offsets: output.repeat_offsets,
    }
}

pub(crate) fn encode_block_fast_no_dict(
    src: &[u8],
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    context: FastBlockEncodeContext<'_, '_>,
) -> FastEncodedBlock {
    let previous_fse = context.fse_tables.clone();
    let previous_offsets = *context.offset_history;
    let prepared = prepare_block_fast_no_dict(src, params, repeat_offsets);
    let mut bytes = Vec::new();

    if src.is_empty() {
        write_raw_block(last_block, 0, src, &mut bytes);
        return FastEncodedBlock {
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
        config,
        prepared.prepared.as_ref(),
        context.fse_tables,
        context.offset_history,
        context.previous_huff_table,
    );
    let compressed_size = bytes.len() - compressed_start;

    if compressed_size >= src.len() || compressed_size > MAX_BLOCK_SIZE as usize {
        bytes.truncate(block_start);
        *context.fse_tables = previous_fse;
        *context.offset_history = previous_offsets;
        write_raw_block(last_block, src.len() as u32, src, &mut bytes);
        FastEncodedBlock {
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
        FastEncodedBlock {
            bytes,
            repeat_offsets: prepared.repeat_offsets,
            new_huffman_table,
        }
    }
}

fn prepare_from_fast_output(
    src: &[u8],
    initial_repeat_offsets: RepeatOffsets,
    output: &FastBlockOutput,
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

fn write_raw_block(last_block: bool, block_size: u32, data: &[u8], output: &mut Vec<u8>) {
    let header = BlockHeader {
        last_block,
        block_type: crate::blocks::block::BlockType::Raw,
        block_size,
    };
    header.serialize(output);
    output.extend_from_slice(data);
}
