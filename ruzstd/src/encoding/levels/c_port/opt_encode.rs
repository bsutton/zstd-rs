//! Block encoder adapters for C optimal strategies.

use alloc::vec::Vec;

use super::{
    block_policy::BlockEncodingPolicy,
    greedy_block::{
        encode_prepared_block, encode_special_block, prepare_from_greedy_output,
        GreedyBlockEncodeContext, GreedyBlockSource, GreedyEncodedBlock, GreedyPreparedBlock,
    },
    opt_parser::compress_block_opt_no_dict_with_state,
    opt_state::{OptBlockState, OptParserStrategy},
    params::{CompressionParameters, Strategy},
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
) -> GreedyEncodedBlock {
    encode_block_btopt_no_dict_with_state_and_policy(
        source,
        last_block,
        params,
        config,
        repeat_offsets,
        opt_state,
        context,
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
) -> GreedyEncodedBlock {
    encode_block_btultra_no_dict_with_state_and_policy(
        source,
        last_block,
        params,
        config,
        repeat_offsets,
        opt_state,
        context,
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
    mut context: GreedyBlockEncodeContext<'_, '_>,
    strategy: OptParserStrategy,
    policy: BlockEncodingPolicy,
) -> GreedyEncodedBlock {
    let block = &source.src[source.block_range.clone()];
    let mut bytes = Vec::new();

    if let Some(encoded) =
        encode_special_block(block, last_block, repeat_offsets, policy, &mut bytes)
    {
        return encoded;
    }

    let previous_fse = context.fse_tables.clone();
    let previous_offsets = *context.offset_history;
    let output = compress_block_opt_no_dict_with_state(
        source.src,
        source.block_range,
        params,
        repeat_offsets,
        opt_state,
        strategy,
    );
    let prepared = prepare_from_greedy_output(block, repeat_offsets, &output);
    let prepared = GreedyPreparedBlock {
        prepared,
        repeat_offsets: output.repeat_offsets,
    };
    if params.strategy >= Strategy::BtOpt && params.window_log >= 17 {
        if let Some(encoded) = encode_split_block(
            block,
            last_block,
            policy,
            params.strategy,
            config,
            repeat_offsets,
            &prepared,
            previous_fse.clone(),
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
