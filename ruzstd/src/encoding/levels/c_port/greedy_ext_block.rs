//! External-dictionary adapters for the C greedy/lazy block compressors.

use alloc::vec::Vec;
use core::ops::Range;

use super::{
    block_policy::BlockEncodingPolicy,
    frame_state::BlockEncodeMode,
    greedy::{GreedyBlockOutput, GreedyMatchState},
    greedy_block::{
        encode_prepared_block, encode_special_block, prepare_from_greedy_output,
        GreedyBlockEncodeContext, GreedyEncodedBlock, GreedyPreparedBlock, LazyBlockStrategy,
    },
    greedy_ext::{
        compress_block_btlazy2_ext_dict_with_state, compress_block_greedy_ext_dict_with_state,
        compress_block_lazy2_ext_dict_with_state, compress_block_lazy_ext_dict_with_state,
    },
    params::CompressionParameters,
    sequence_store::RepeatOffsets,
};
use crate::encoding::blocks::BlockCompressionConfig;

pub(crate) struct GreedyExtDictBlockSource<'a> {
    pub(crate) src: &'a [u8],
    pub(crate) block_range: Range<usize>,
    pub(crate) dict_limit: usize,
    pub(crate) loaded_dict_end: usize,
}

fn prepare_block_hash_chain_ext_dict_with_state(
    source: GreedyExtDictBlockSource<'_>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
    depth: LazyBlockStrategy,
) -> GreedyPreparedBlock {
    let block = &source.src[source.block_range.clone()];
    let output = compress_block_for_depth_ext_dict_with_state(
        source.src,
        source.block_range,
        source.dict_limit,
        params,
        repeat_offsets,
        state,
        depth,
        source.loaded_dict_end,
    );
    let prepared = prepare_from_greedy_output(block, repeat_offsets, &output);

    GreedyPreparedBlock {
        prepared,
        repeat_offsets: output.repeat_offsets,
    }
}

#[allow(clippy::too_many_arguments)]
fn compress_block_for_depth_ext_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    dict_limit: usize,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
    depth: LazyBlockStrategy,
    loaded_dict_end: usize,
) -> GreedyBlockOutput {
    match depth {
        LazyBlockStrategy::Greedy => compress_block_greedy_ext_dict_with_state(
            src,
            block_range,
            dict_limit,
            params,
            repeat_offsets,
            state,
            loaded_dict_end,
        ),
        LazyBlockStrategy::Lazy => compress_block_lazy_ext_dict_with_state(
            src,
            block_range,
            dict_limit,
            params,
            repeat_offsets,
            state,
            loaded_dict_end,
        ),
        LazyBlockStrategy::Lazy2 => compress_block_lazy2_ext_dict_with_state(
            src,
            block_range,
            dict_limit,
            params,
            repeat_offsets,
            state,
            loaded_dict_end,
        ),
        LazyBlockStrategy::BtLazy2 => compress_block_btlazy2_ext_dict_with_state(
            src,
            block_range,
            dict_limit,
            params,
            repeat_offsets,
            state,
            loaded_dict_end,
        ),
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn encode_block_hash_chain_ext_dict_with_state_and_policy(
    source: GreedyExtDictBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    match_state: &mut GreedyMatchState,
    context: GreedyBlockEncodeContext<'_, '_>,
    depth: LazyBlockStrategy,
    policy: BlockEncodingPolicy,
) -> GreedyEncodedBlock {
    encode_block_hash_chain_ext_dict_with_state_and_policy_in_mode(
        source,
        last_block,
        params,
        config,
        repeat_offsets,
        match_state,
        context,
        depth,
        policy,
        BlockEncodeMode::Normal,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn encode_block_hash_chain_ext_dict_with_state_and_policy_in_mode(
    source: GreedyExtDictBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    match_state: &mut GreedyMatchState,
    context: GreedyBlockEncodeContext<'_, '_>,
    depth: LazyBlockStrategy,
    policy: BlockEncodingPolicy,
    block_encode_mode: BlockEncodeMode,
) -> GreedyEncodedBlock {
    let block = &source.src[source.block_range.clone()];
    let mut bytes = Vec::new();

    if let Some(encoded) = encode_special_block(block, last_block, repeat_offsets, &mut bytes) {
        return encoded;
    }

    let previous_fse = context.fse_tables.snapshot_previous();
    let previous_offsets = *context.offset_history;
    let prepared = prepare_block_hash_chain_ext_dict_with_state(
        source,
        params,
        repeat_offsets,
        match_state,
        depth,
    );
    if let Some(_target_size) = block_encode_mode.target_c_block_size() {
        return super::greedy_block::encode_target_block_raw_fallback(
            block,
            last_block,
            repeat_offsets,
            bytes,
        );
    }
    encode_prepared_block(
        block,
        last_block,
        params,
        config,
        repeat_offsets,
        prepared,
        policy,
        previous_fse,
        previous_offsets,
        context,
        bytes,
    )
}
