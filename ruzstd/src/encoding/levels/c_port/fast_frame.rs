//! Frame-level adapter for the C fast no-dictionary path.

use alloc::vec::Vec;

use super::{
    block_policy::BlockEncodingPolicy,
    c_frame_header::write_frame_header_no_dict,
    fast::FastMatchState,
    fast_block::{
        encode_block_fast_no_dict, encode_block_fast_no_dict_with_state_and_policy,
        FastBlockEncodeContext, FastBlockSource,
    },
    params::CompressionParameters,
    pre_split::FrameProgress,
    sequence_store::RepeatOffsets,
};
use crate::{
    common::MAX_BLOCK_SIZE,
    encoding::{
        blocks::BlockCompressionConfig,
        frame_compressor::{FseTables, OffsetHistory},
    },
};

pub(crate) fn encode_single_block_frame_fast_no_dict(src: &[u8], level: i32) -> Vec<u8> {
    debug_assert!(src.len() <= MAX_BLOCK_SIZE as usize);
    encode_frame_fast_no_dict(src, level)
}

pub(crate) fn encode_frame_fast_no_dict(src: &[u8], level: i32) -> Vec<u8> {
    let mut output = Vec::new();
    let params = CompressionParameters::for_level(level, src.len() as u64, 0);
    write_frame_header_no_dict(&mut output, src.len(), params);
    let mut fse_tables = FseTables::new();
    let mut offset_history = OffsetHistory::new();
    let mut match_state = FastMatchState::new();
    let mut last_huff_table = None;
    let mut repeat_offsets = RepeatOffsets::new();
    let block_config = BlockCompressionConfig::for_c_strategy(params.strategy as u8);

    if src.is_empty() {
        let encoded_block = encode_block_fast_no_dict(
            src,
            true,
            params,
            block_config,
            repeat_offsets,
            FastBlockEncodeContext {
                previous_huff_table: None,
                fse_tables: &mut fse_tables,
                offset_history: &mut offset_history,
            },
        );
        output.extend_from_slice(&encoded_block.bytes);
        return output;
    }

    let mut block_start = 0;
    let mut progress = FrameProgress::new(output.len());
    while block_start < src.len() {
        let block_size = progress.next_block_size(&src[block_start..], params.strategy);
        let block_end = block_start + block_size;
        let policy = if block_start == 0 {
            BlockEncodingPolicy::frame_first_block()
        } else {
            BlockEncodingPolicy::normal()
        };
        let encoded_block = encode_block_fast_no_dict_with_state_and_policy(
            FastBlockSource {
                src,
                block_range: block_start..block_end,
            },
            block_end == src.len(),
            params,
            block_config,
            repeat_offsets,
            &mut match_state,
            FastBlockEncodeContext {
                previous_huff_table: last_huff_table.as_ref(),
                fse_tables: &mut fse_tables,
                offset_history: &mut offset_history,
            },
            policy,
        );
        repeat_offsets = encoded_block.repeat_offsets;
        last_huff_table = encoded_block.new_huffman_table;
        progress.record_block(block_size, encoded_block.bytes.len());
        output.extend_from_slice(&encoded_block.bytes);
        block_start = block_end;
    }

    output
}
