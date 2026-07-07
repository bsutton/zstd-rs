//! Block encoder adapters for C optimal strategies.

use alloc::vec::Vec;

use super::{
    block_policy::BlockEncodingPolicy,
    frame_state::BlockEncodeMode,
    greedy_block::{
        encode_prepared_block, encode_special_block, encode_target_block_raw_fallback,
        prepare_from_greedy_output, GreedyBlockEncodeContext, GreedyBlockSource,
        GreedyEncodedBlock, GreedyPreparedBlock,
    },
    greedy_ext_block::GreedyExtDictBlockSource,
    ldm::opt::LdmOptCursor,
    opt_parser::{
        compress_block_opt_ext_dict_with_state_and_ldm,
        compress_block_opt_no_dict_with_state_and_ldm,
    },
    opt_state::{OptBlockState, OptParserStrategy},
    params::CompressionParameters,
    post_split::encode_split_block,
    sequence_store::RepeatOffsets,
};
use crate::encoding::blocks::BlockCompressionConfig;

#[allow(clippy::too_many_arguments)]
pub(crate) fn encode_block_btopt_no_dict_with_state(
    source: GreedyBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    opt_state: &mut OptBlockState,
    context: GreedyBlockEncodeContext<'_, '_>,
    post_block_splitter: bool,
) -> GreedyEncodedBlock {
    encode_block_btopt_no_dict_with_state_and_policy(
        source,
        last_block,
        params,
        config,
        repeat_offsets,
        opt_state,
        context,
        post_block_splitter,
        BlockEncodingPolicy::normal(),
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn encode_block_btopt_no_dict_with_state_and_policy(
    source: GreedyBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    opt_state: &mut OptBlockState,
    context: GreedyBlockEncodeContext<'_, '_>,
    post_block_splitter: bool,
    policy: BlockEncodingPolicy,
) -> GreedyEncodedBlock {
    encode_block_opt_no_dict_with_state(
        source,
        last_block,
        params,
        config,
        repeat_offsets,
        opt_state,
        context,
        OptParserStrategy::BtOpt,
        post_block_splitter,
        policy,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn encode_block_btultra_no_dict_with_state(
    source: GreedyBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    opt_state: &mut OptBlockState,
    context: GreedyBlockEncodeContext<'_, '_>,
    post_block_splitter: bool,
) -> GreedyEncodedBlock {
    encode_block_btultra_no_dict_with_state_and_policy(
        source,
        last_block,
        params,
        config,
        repeat_offsets,
        opt_state,
        context,
        post_block_splitter,
        BlockEncodingPolicy::normal(),
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn encode_block_btultra_no_dict_with_state_and_policy(
    source: GreedyBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    opt_state: &mut OptBlockState,
    context: GreedyBlockEncodeContext<'_, '_>,
    post_block_splitter: bool,
    policy: BlockEncodingPolicy,
) -> GreedyEncodedBlock {
    encode_block_opt_no_dict_with_state(
        source,
        last_block,
        params,
        config,
        repeat_offsets,
        opt_state,
        context,
        OptParserStrategy::BtUltra,
        post_block_splitter,
        policy,
    )
}

#[allow(clippy::too_many_arguments)]
fn encode_block_opt_no_dict_with_state(
    source: GreedyBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    opt_state: &mut OptBlockState,
    context: GreedyBlockEncodeContext<'_, '_>,
    strategy: OptParserStrategy,
    post_block_splitter: bool,
    policy: BlockEncodingPolicy,
) -> GreedyEncodedBlock {
    encode_block_opt_no_dict_with_state_and_policy(
        source,
        last_block,
        params,
        config,
        repeat_offsets,
        opt_state,
        context,
        strategy,
        post_block_splitter,
        policy,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn encode_block_opt_no_dict_with_state_and_policy(
    source: GreedyBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    opt_state: &mut OptBlockState,
    context: GreedyBlockEncodeContext<'_, '_>,
    strategy: OptParserStrategy,
    post_block_splitter: bool,
    policy: BlockEncodingPolicy,
) -> GreedyEncodedBlock {
    encode_block_opt_no_dict_with_state_and_policy_and_ldm(
        source,
        last_block,
        params,
        config,
        repeat_offsets,
        opt_state,
        context,
        strategy,
        post_block_splitter,
        policy,
        None,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn encode_block_opt_no_dict_with_state_and_policy_and_ldm(
    source: GreedyBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    opt_state: &mut OptBlockState,
    context: GreedyBlockEncodeContext<'_, '_>,
    strategy: OptParserStrategy,
    post_block_splitter: bool,
    policy: BlockEncodingPolicy,
    ldm_cursor: Option<&mut LdmOptCursor<'_>>,
) -> GreedyEncodedBlock {
    encode_block_opt_no_dict_with_state_and_policy_and_ldm_in_mode(
        source,
        last_block,
        params,
        config,
        repeat_offsets,
        opt_state,
        context,
        strategy,
        block_encode_mode_from_splitter(post_block_splitter),
        policy,
        ldm_cursor,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn encode_block_opt_no_dict_with_state_and_policy_and_ldm_in_mode(
    source: GreedyBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    opt_state: &mut OptBlockState,
    mut context: GreedyBlockEncodeContext<'_, '_>,
    strategy: OptParserStrategy,
    block_encode_mode: BlockEncodeMode,
    policy: BlockEncodingPolicy,
    ldm_cursor: Option<&mut LdmOptCursor<'_>>,
) -> GreedyEncodedBlock {
    let block = &source.src[source.block_range.clone()];
    let mut bytes = Vec::new();

    if let Some(encoded) = encode_special_block(block, last_block, repeat_offsets, &mut bytes) {
        return encoded;
    }

    let previous_fse = context.fse_tables.snapshot_previous();
    let previous_offsets = *context.offset_history;
    let output = compress_block_opt_no_dict_with_state_and_ldm(
        source.src,
        source.block_range.clone(),
        params,
        repeat_offsets,
        opt_state,
        strategy,
        ldm_cursor,
        source.loaded_dict_end,
    );
    let prepared = prepare_from_greedy_output(block, repeat_offsets, &output);
    let prepared = GreedyPreparedBlock {
        prepared,
        repeat_offsets: output.repeat_offsets,
    };
    if let Some(_target_size) = block_encode_mode.target_c_block_size() {
        return encode_target_block_raw_fallback(block, last_block, repeat_offsets, bytes);
    }
    if block_encode_mode.split_block_enabled() {
        if let Some(encoded) = encode_split_block(
            block,
            last_block,
            policy,
            params.strategy,
            config,
            repeat_offsets,
            &prepared,
            previous_offsets,
            &mut context,
        ) {
            return encoded;
        }
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

#[allow(clippy::too_many_arguments)]
pub(crate) fn encode_block_opt_ext_dict_with_state_and_policy_and_ldm(
    source: GreedyExtDictBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    opt_state: &mut OptBlockState,
    context: GreedyBlockEncodeContext<'_, '_>,
    strategy: OptParserStrategy,
    post_block_splitter: bool,
    policy: BlockEncodingPolicy,
    ldm_cursor: Option<&mut LdmOptCursor<'_>>,
) -> GreedyEncodedBlock {
    encode_block_opt_ext_dict_with_state_and_policy_and_ldm_in_mode(
        source,
        last_block,
        params,
        config,
        repeat_offsets,
        opt_state,
        context,
        strategy,
        block_encode_mode_from_splitter(post_block_splitter),
        policy,
        ldm_cursor,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn encode_block_opt_ext_dict_with_state_and_policy_and_ldm_in_mode(
    source: GreedyExtDictBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    opt_state: &mut OptBlockState,
    mut context: GreedyBlockEncodeContext<'_, '_>,
    strategy: OptParserStrategy,
    block_encode_mode: BlockEncodeMode,
    policy: BlockEncodingPolicy,
    ldm_cursor: Option<&mut LdmOptCursor<'_>>,
) -> GreedyEncodedBlock {
    let block = &source.src[source.block_range.clone()];
    let mut bytes = Vec::new();

    if let Some(encoded) = encode_special_block(block, last_block, repeat_offsets, &mut bytes) {
        return encoded;
    }

    let previous_fse = context.fse_tables.snapshot_previous();
    let previous_offsets = *context.offset_history;
    let output = compress_block_opt_ext_dict_with_state_and_ldm(
        source.src,
        source.block_range.clone(),
        source.dict_limit,
        params,
        repeat_offsets,
        opt_state,
        strategy,
        ldm_cursor,
        source.loaded_dict_end,
    );
    let prepared = prepare_from_greedy_output(block, repeat_offsets, &output);
    let prepared = GreedyPreparedBlock {
        prepared,
        repeat_offsets: output.repeat_offsets,
    };
    if let Some(_target_size) = block_encode_mode.target_c_block_size() {
        return encode_target_block_raw_fallback(block, last_block, repeat_offsets, bytes);
    }
    if block_encode_mode.split_block_enabled() {
        if let Some(encoded) = encode_split_block(
            block,
            last_block,
            policy,
            params.strategy,
            config,
            repeat_offsets,
            &prepared,
            previous_offsets,
            &mut context,
        ) {
            return encoded;
        }
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

fn block_encode_mode_from_splitter(post_block_splitter: bool) -> BlockEncodeMode {
    if post_block_splitter {
        BlockEncodeMode::SplitBlock
    } else {
        BlockEncodeMode::Normal
    }
}
