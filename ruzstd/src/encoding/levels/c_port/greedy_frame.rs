//! Frame-level adapter for the C greedy no-dictionary path.

use alloc::vec::Vec;

use super::{
    c_frame_header::write_frame_header_no_dict,
    cctx_params::CctxParameters,
    compress_bound::compress_bound,
    dictionary::ParsedDictionary,
    dictionary_frame::DictionaryFrameContext,
    frame_state::{streaming_dict_limit, BlockEncodeMode, FrameBlockState},
    greedy::GreedyMatchState,
    greedy_block::{
        encode_block_hash_chain_no_dict,
        encode_block_hash_chain_no_dict_with_state_and_policy_in_mode, GreedyBlockEncodeContext,
        GreedyBlockSource, LazyBlockStrategy,
    },
    greedy_dict::{load_binary_tree_prefix, load_prefix},
    greedy_ext_block::{
        encode_block_hash_chain_ext_dict_with_state_and_policy_in_mode, GreedyExtDictBlockSource,
    },
};
use crate::common::MAX_BLOCK_SIZE;

pub(crate) fn encode_single_block_frame_greedy_no_dict(src: &[u8], level: i32) -> Vec<u8> {
    debug_assert!(src.len() <= MAX_BLOCK_SIZE as usize);
    encode_frame_greedy_no_dict(src, level)
}

pub(crate) fn encode_frame_greedy_no_dict(src: &[u8], level: i32) -> Vec<u8> {
    encode_frame_hash_chain_no_dict(src, level, LazyBlockStrategy::Greedy)
}

pub(crate) fn encode_single_block_frame_lazy_no_dict(src: &[u8], level: i32) -> Vec<u8> {
    debug_assert!(src.len() <= MAX_BLOCK_SIZE as usize);
    encode_frame_lazy_no_dict(src, level)
}

pub(crate) fn encode_frame_lazy_no_dict(src: &[u8], level: i32) -> Vec<u8> {
    encode_frame_hash_chain_no_dict(src, level, LazyBlockStrategy::Lazy)
}

pub(crate) fn encode_single_block_frame_lazy2_no_dict(src: &[u8], level: i32) -> Vec<u8> {
    debug_assert!(src.len() <= MAX_BLOCK_SIZE as usize);
    encode_frame_lazy2_no_dict(src, level)
}

pub(crate) fn encode_frame_lazy2_no_dict(src: &[u8], level: i32) -> Vec<u8> {
    encode_frame_hash_chain_no_dict(src, level, LazyBlockStrategy::Lazy2)
}

pub(crate) fn encode_frame_greedy_with_dictionary(
    src: &[u8],
    level: i32,
    dictionary: ParsedDictionary<'_>,
) -> Vec<u8> {
    encode_frame_hash_chain_with_dictionary(src, level, dictionary, LazyBlockStrategy::Greedy)
}

pub(crate) fn encode_frame_lazy_with_dictionary(
    src: &[u8],
    level: i32,
    dictionary: ParsedDictionary<'_>,
) -> Vec<u8> {
    encode_frame_hash_chain_with_dictionary(src, level, dictionary, LazyBlockStrategy::Lazy)
}

pub(crate) fn encode_frame_lazy2_with_dictionary(
    src: &[u8],
    level: i32,
    dictionary: ParsedDictionary<'_>,
) -> Vec<u8> {
    encode_frame_hash_chain_with_dictionary(src, level, dictionary, LazyBlockStrategy::Lazy2)
}

pub(crate) fn encode_single_block_frame_btlazy2_no_dict(src: &[u8], level: i32) -> Vec<u8> {
    debug_assert!(src.len() <= MAX_BLOCK_SIZE as usize);
    encode_frame_btlazy2_no_dict(src, level)
}

pub(crate) fn encode_frame_btlazy2_no_dict(src: &[u8], level: i32) -> Vec<u8> {
    encode_frame_hash_chain_no_dict(src, level, LazyBlockStrategy::BtLazy2)
}

pub(crate) fn encode_frame_btlazy2_with_dictionary(
    src: &[u8],
    level: i32,
    dictionary: ParsedDictionary<'_>,
) -> Vec<u8> {
    encode_frame_hash_chain_with_dictionary(src, level, dictionary, LazyBlockStrategy::BtLazy2)
}

fn encode_frame_hash_chain_no_dict(src: &[u8], level: i32, depth: LazyBlockStrategy) -> Vec<u8> {
    let cctx = CctxParameters::for_level(level, src.len() as u64, 0);
    encode_frame_hash_chain_no_dict_with_cctx(src, cctx, depth)
}

pub(crate) fn encode_frame_hash_chain_no_dict_with_cctx(
    src: &[u8],
    cctx: CctxParameters,
    depth: LazyBlockStrategy,
) -> Vec<u8> {
    let mut output = Vec::with_capacity(compress_bound(src.len()));
    cctx.assert_resolved();
    let block_encode_mode = BlockEncodeMode::from_cctx(cctx);
    let params = cctx.compression;
    write_frame_header_no_dict(&mut output, src.len(), params);
    let mut frame_state = FrameBlockState::new(params, cctx.max_block_size);
    let mut match_state = GreedyMatchState::new();
    match_state.reset_for_frame(params);
    let mut dict_limit = 0_usize;

    if src.is_empty() {
        let encoded_block = encode_block_hash_chain_no_dict(
            src,
            true,
            params,
            frame_state.block_config,
            frame_state.repeat_offsets,
            GreedyBlockEncodeContext {
                previous_huff_table: None,
                fse_tables: &mut frame_state.fse_tables,
                offset_history: &mut frame_state.offset_history,
            },
            depth,
        );
        output.extend_from_slice(&encoded_block.bytes);
        return output;
    }

    let mut block_start = 0;
    while block_start < src.len() {
        let block_size = frame_state.next_frame_chunk_block_size(
            &src[block_start..],
            block_start,
            params.strategy,
        );
        let block_end = block_start + block_size;
        dict_limit = streaming_dict_limit(dict_limit, block_start, params.window_log);
        let policy = FrameBlockState::block_policy(block_start == 0);
        let block_context = GreedyBlockEncodeContext {
            previous_huff_table: frame_state.last_huff_table.as_ref(),
            fse_tables: &mut frame_state.fse_tables,
            offset_history: &mut frame_state.offset_history,
        };
        let encoded_block = if dict_limit == 0 {
            encode_block_hash_chain_no_dict_with_state_and_policy_in_mode(
                GreedyBlockSource {
                    src,
                    block_range: block_start..block_end,
                    loaded_dict_end: 0,
                },
                block_end == src.len(),
                params,
                frame_state.block_config,
                frame_state.repeat_offsets,
                &mut match_state,
                block_context,
                depth,
                policy,
                block_encode_mode,
            )
        } else {
            encode_block_hash_chain_ext_dict_with_state_and_policy_in_mode(
                GreedyExtDictBlockSource {
                    src,
                    block_range: block_start..block_end,
                    dict_limit,
                    loaded_dict_end: 0,
                },
                block_end == src.len(),
                params,
                frame_state.block_config,
                frame_state.repeat_offsets,
                &mut match_state,
                block_context,
                depth,
                policy,
                block_encode_mode,
            )
        };
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

fn encode_frame_hash_chain_with_dictionary(
    src: &[u8],
    level: i32,
    dictionary: ParsedDictionary<'_>,
    depth: LazyBlockStrategy,
) -> Vec<u8> {
    let mut context = DictionaryFrameContext::new(src, level, dictionary);
    let params = context.cctx.compression;
    let block_encode_mode = BlockEncodeMode::from_cctx(context.cctx);

    let mut match_state = GreedyMatchState::new();
    match_state.reset_for_frame(params);
    if depth == LazyBlockStrategy::BtLazy2 {
        load_binary_tree_prefix(
            &mut match_state,
            &context.combined,
            context.dict_len,
            params,
        );
    } else {
        load_prefix(
            &mut match_state,
            &context.combined,
            context.dict_len,
            params,
        );
    }

    if src.is_empty() {
        let encoded_block = encode_block_hash_chain_no_dict(
            src,
            true,
            params,
            context.frame_state.block_config,
            context.frame_state.repeat_offsets,
            GreedyBlockEncodeContext {
                previous_huff_table: context.frame_state.last_huff_table.as_ref(),
                fse_tables: &mut context.frame_state.fse_tables,
                offset_history: &mut context.frame_state.offset_history,
            },
            depth,
        );
        context.output.extend_from_slice(&encoded_block.bytes);
        return context.output;
    }

    let mut block_start = context.dict_len;
    let src_end = context.src_end();
    while block_start < src_end {
        let block_size = context.frame_state.next_frame_chunk_block_size(
            &context.combined[block_start..src_end],
            block_start - context.dict_len,
            params.strategy,
        );
        let block_end = block_start + block_size;
        let policy = FrameBlockState::block_policy(block_start == context.dict_len);
        let loaded_dict_end = context.loaded_dict_end_for_block(block_end, params);
        let block_context = GreedyBlockEncodeContext {
            previous_huff_table: context.frame_state.last_huff_table.as_ref(),
            fse_tables: &mut context.frame_state.fse_tables,
            offset_history: &mut context.frame_state.offset_history,
        };
        let encoded_block = if loaded_dict_end == 0 {
            encode_block_hash_chain_no_dict_with_state_and_policy_in_mode(
                GreedyBlockSource {
                    src: &context.combined,
                    block_range: block_start..block_end,
                    loaded_dict_end,
                },
                block_end == src_end,
                params,
                context.frame_state.block_config,
                context.frame_state.repeat_offsets,
                &mut match_state,
                block_context,
                depth,
                policy,
                block_encode_mode,
            )
        } else {
            encode_block_hash_chain_ext_dict_with_state_and_policy_in_mode(
                GreedyExtDictBlockSource {
                    src: &context.combined,
                    block_range: block_start..block_end,
                    dict_limit: context.dict_len,
                    loaded_dict_end,
                },
                block_end == src_end,
                params,
                context.frame_state.block_config,
                context.frame_state.repeat_offsets,
                &mut match_state,
                block_context,
                depth,
                policy,
                block_encode_mode,
            )
        };
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
