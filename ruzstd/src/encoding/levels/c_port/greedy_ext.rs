//! External-dictionary entry points for the C greedy/lazy/lazy2 ports.

use core::ops::Range;

use super::{
    greedy::{
        compress_block_lazy_generic_with_state, GreedyBlockOutput, GreedyMatchState,
        SEARCH_BINARY_TREE, SEARCH_HASH_CHAIN, SEARCH_ROW_HASH,
    },
    greedy_bounds::LazyDictionaryBounds,
    params::CompressionParameters,
    row_match::row_match_finder_enabled,
    sequence_store::RepeatOffsets,
};

pub(crate) fn compress_block_greedy_ext_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    dict_limit: usize,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
    loaded_dict_end: usize,
) -> GreedyBlockOutput {
    compress_block_hash_or_row_ext_dict_with_state(
        src,
        block_range,
        dict_limit,
        params,
        repeat_offsets,
        state,
        0,
        loaded_dict_end,
    )
}

pub(crate) fn compress_block_lazy_ext_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    dict_limit: usize,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
    loaded_dict_end: usize,
) -> GreedyBlockOutput {
    compress_block_hash_or_row_ext_dict_with_state(
        src,
        block_range,
        dict_limit,
        params,
        repeat_offsets,
        state,
        1,
        loaded_dict_end,
    )
}

pub(crate) fn compress_block_lazy2_ext_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    dict_limit: usize,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
    loaded_dict_end: usize,
) -> GreedyBlockOutput {
    compress_block_hash_or_row_ext_dict_with_state(
        src,
        block_range,
        dict_limit,
        params,
        repeat_offsets,
        state,
        2,
        loaded_dict_end,
    )
}

pub(crate) fn compress_block_btlazy2_ext_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    dict_limit: usize,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
    loaded_dict_end: usize,
) -> GreedyBlockOutput {
    compress_block_lazy_generic_with_state::<SEARCH_BINARY_TREE>(
        src,
        block_range.clone(),
        params,
        repeat_offsets,
        state,
        2,
        LazyDictionaryBounds::ext_dict(block_range.end, dict_limit, params, loaded_dict_end),
    )
}

#[allow(clippy::too_many_arguments)]
fn compress_block_hash_or_row_ext_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    dict_limit: usize,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
    depth: u32,
    loaded_dict_end: usize,
) -> GreedyBlockOutput {
    let bounds =
        LazyDictionaryBounds::ext_dict(block_range.end, dict_limit, params, loaded_dict_end);
    if row_match_finder_enabled(params) {
        compress_block_lazy_generic_with_state::<SEARCH_ROW_HASH>(
            src,
            block_range,
            params,
            repeat_offsets,
            state,
            depth,
            bounds,
        )
    } else {
        compress_block_lazy_generic_with_state::<SEARCH_HASH_CHAIN>(
            src,
            block_range,
            params,
            repeat_offsets,
            state,
            depth,
            bounds,
        )
    }
}
