//! Block encoder adapters for C optimal strategies.

use alloc::vec::Vec;

use super::{
    block_policy::BlockEncodingPolicy,
    greedy_block::{
        encode_prepared_block, encode_special_block, prepare_from_greedy_output,
        GreedyBlockEncodeContext, GreedyBlockSource, GreedyEncodedBlock, GreedyPreparedBlock,
    },
    ldm::opt::LdmOptCursor,
    opt_parser::compress_block_opt_no_dict_with_state_and_ldm,
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
    mut context: GreedyBlockEncodeContext<'_, '_>,
    strategy: OptParserStrategy,
    post_block_splitter: bool,
    policy: BlockEncodingPolicy,
    ldm_cursor: Option<&mut LdmOptCursor<'_>>,
) -> GreedyEncodedBlock {
    let block = &source.src[source.block_range.clone()];
    let mut bytes = Vec::new();

    if let Some(encoded) =
        encode_special_block(block, last_block, repeat_offsets, policy, &mut bytes)
    {
        return encoded;
    }

    let previous_fse = context.fse_tables.snapshot_previous();
    let previous_offsets = *context.offset_history;
    let output = compress_block_opt_no_dict_with_state_and_ldm(
        source.src,
        source.block_range,
        params,
        repeat_offsets,
        opt_state,
        strategy,
        ldm_cursor,
    );
    let prepared = prepare_from_greedy_output(block, repeat_offsets, &output);
    let prepared = GreedyPreparedBlock {
        prepared,
        repeat_offsets: output.repeat_offsets,
    };
    if post_block_splitter {
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
        previous_fse,
        previous_offsets,
        context,
        bytes,
    )
}
