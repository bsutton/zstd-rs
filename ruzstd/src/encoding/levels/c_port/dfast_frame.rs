//! Frame-level adapter for the C double-fast no-dictionary path.

use alloc::vec::Vec;

use super::{
    c_frame_header::write_frame_header_no_dict,
    cctx_params::CctxParameters,
    dfast::DFastMatchState,
    dfast_block::{
        encode_block_double_fast_ext_dict_with_state_and_policy, encode_block_double_fast_no_dict,
        encode_block_double_fast_no_dict_with_state_and_policy, DFastBlockEncodeContext,
        DFastBlockSource, DFastExtDictBlockSource,
    },
    dfast_dict::load_prefix,
    dictionary::ParsedDictionary,
    dictionary_frame::DictionaryFrameContext,
    frame_state::FrameBlockState,
};
use crate::common::MAX_BLOCK_SIZE;

pub(crate) fn encode_single_block_frame_double_fast_no_dict(src: &[u8], level: i32) -> Vec<u8> {
    debug_assert!(src.len() <= MAX_BLOCK_SIZE as usize);
    encode_frame_double_fast_no_dict(src, level)
}

pub(crate) fn encode_frame_double_fast_no_dict(src: &[u8], level: i32) -> Vec<u8> {
    let mut output = Vec::new();
    let cctx = CctxParameters::for_level(level, src.len() as u64, 0);
    cctx.assert_resolved();
    let params = cctx.compression;
    write_frame_header_no_dict(&mut output, src.len(), params);
    let mut frame_state = FrameBlockState::new(params, output.len());
    let mut match_state = DFastMatchState::new();

    if src.is_empty() {
        let encoded_block = encode_block_double_fast_no_dict(
            src,
            true,
            params,
            frame_state.block_config,
            frame_state.repeat_offsets,
            DFastBlockEncodeContext {
                previous_huff_table: None,
                fse_tables: &mut frame_state.fse_tables,
                offset_history: &mut frame_state.offset_history,
            },
        );
        output.extend_from_slice(&encoded_block.bytes);
        return output;
    }

    let mut block_start = 0;
    while block_start < src.len() {
        let block_size = frame_state.next_block_size(&src[block_start..], params.strategy);
        let block_end = block_start + block_size;
        let policy = FrameBlockState::block_policy(block_start == 0);
        let encoded_block = encode_block_double_fast_no_dict_with_state_and_policy(
            DFastBlockSource {
                src,
                block_range: block_start..block_end,
                loaded_dict_end: 0,
            },
            block_end == src.len(),
            params,
            frame_state.block_config,
            frame_state.repeat_offsets,
            &mut match_state,
            DFastBlockEncodeContext {
                previous_huff_table: frame_state.last_huff_table.as_ref(),
                fse_tables: &mut frame_state.fse_tables,
                offset_history: &mut frame_state.offset_history,
            },
            policy,
        );
        frame_state.record_encoded_block(
            block_size,
            encoded_block.bytes.len(),
            encoded_block.repeat_offsets,
            encoded_block.new_huffman_table,
        );
        output.extend_from_slice(&encoded_block.bytes);
        block_start = block_end;
    }

    output
}

pub(crate) fn encode_frame_double_fast_with_dictionary(
    src: &[u8],
    level: i32,
    dictionary: ParsedDictionary<'_>,
) -> Vec<u8> {
    let mut context = DictionaryFrameContext::new(src, level, dictionary);
    let params = context.cctx.compression;

    let mut match_state = DFastMatchState::new();
    load_prefix(
        &mut match_state,
        &context.combined,
        context.dict_len,
        params,
    );

    if src.is_empty() {
        let encoded_block = encode_block_double_fast_no_dict(
            src,
            true,
            params,
            context.frame_state.block_config,
            context.frame_state.repeat_offsets,
            DFastBlockEncodeContext {
                previous_huff_table: context.frame_state.last_huff_table.as_ref(),
                fse_tables: &mut context.frame_state.fse_tables,
                offset_history: &mut context.frame_state.offset_history,
            },
        );
        context.output.extend_from_slice(&encoded_block.bytes);
        return context.output;
    }

    let mut block_start = context.dict_len;
    let src_end = context.src_end();
    while block_start < src_end {
        let block_size = context
            .frame_state
            .next_block_size(&context.combined[block_start..src_end], params.strategy);
        let block_end = block_start + block_size;
        let policy = FrameBlockState::block_policy(block_start == context.dict_len);
        let encoded_block = encode_block_double_fast_ext_dict_with_state_and_policy(
            DFastExtDictBlockSource {
                src: &context.combined,
                block_range: block_start..block_end,
                dict_limit: context.dict_len,
                loaded_dict_end: context.dict_len,
            },
            block_end == src_end,
            params,
            context.frame_state.block_config,
            context.frame_state.repeat_offsets,
            &mut match_state,
            DFastBlockEncodeContext {
                previous_huff_table: context.frame_state.last_huff_table.as_ref(),
                fse_tables: &mut context.frame_state.fse_tables,
                offset_history: &mut context.frame_state.offset_history,
            },
            policy,
        );
        context.frame_state.record_encoded_block(
            block_size,
            encoded_block.bytes.len(),
            encoded_block.repeat_offsets,
            encoded_block.new_huffman_table,
        );
        context.output.extend_from_slice(&encoded_block.bytes);
        block_start = block_end;
    }

    context.output
}
