//! Frame-level adapter for the C optimal no-dictionary path.

use alloc::vec::Vec;

use super::{
    greedy_block::{
        encode_block_btopt_no_dict_with_state, GreedyBlockEncodeContext, GreedyBlockSource,
    },
    opt_parser::OptBlockState,
    params::CompressionParameters,
    sequence_store::RepeatOffsets,
};
use crate::{
    common::MAX_BLOCK_SIZE,
    encoding::{
        blocks::BlockCompressionConfig,
        frame_compressor::{FseTables, OffsetHistory},
        frame_header::FrameHeader,
        CompressionLevel,
    },
};

pub(crate) fn encode_frame_btopt_no_dict(src: &[u8], level: i32) -> Vec<u8> {
    let mut output = Vec::new();
    FrameHeader {
        frame_content_size: Some(src.len() as u64),
        single_segment: true,
        content_checksum: false,
        dictionary_id: None,
        window_size: None,
    }
    .serialize(&mut output);

    let mut fse_tables = FseTables::new();
    let mut offset_history = OffsetHistory::new();
    let mut opt_state = OptBlockState::new();
    let mut last_huff_table = None;
    let mut repeat_offsets = RepeatOffsets::new();
    let params = CompressionParameters::for_level(level, src.len() as u64, 0);

    if src.is_empty() {
        let encoded_block = encode_block_btopt_no_dict_with_state(
            GreedyBlockSource {
                src,
                block_range: 0..0,
            },
            true,
            params,
            BlockCompressionConfig::for_level(CompressionLevel::Default),
            repeat_offsets,
            &mut opt_state,
            GreedyBlockEncodeContext {
                previous_huff_table: None,
                fse_tables: &mut fse_tables,
                offset_history: &mut offset_history,
            },
        );
        output.extend_from_slice(&encoded_block.bytes);
        return output;
    }

    let mut block_start = 0;
    while block_start < src.len() {
        let block_end = (block_start + MAX_BLOCK_SIZE as usize).min(src.len());
        let encoded_block = encode_block_btopt_no_dict_with_state(
            GreedyBlockSource {
                src,
                block_range: block_start..block_end,
            },
            block_end == src.len(),
            params,
            BlockCompressionConfig::for_level(CompressionLevel::Default),
            repeat_offsets,
            &mut opt_state,
            GreedyBlockEncodeContext {
                previous_huff_table: last_huff_table.as_ref(),
                fse_tables: &mut fse_tables,
                offset_history: &mut offset_history,
            },
        );
        repeat_offsets = encoded_block.repeat_offsets;
        last_huff_table = encoded_block.new_huffman_table;
        output.extend_from_slice(&encoded_block.bytes);
        block_start = block_end;
    }

    output
}
