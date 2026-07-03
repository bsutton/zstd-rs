//! Frame-level adapter for the C optimal no-dictionary path.

use alloc::vec::Vec;

use super::{
    block_compressor::{select_block_compressor, DictionaryMode},
    block_policy::BlockEncodingPolicy,
    c_frame_header::write_frame_header_no_dict,
    cctx_params::{CctxParameters, ParamSwitch},
    dictionary::ParsedDictionary,
    dictionary_frame::DictionaryFrameContext,
    frame_state::FrameBlockState,
    greedy_block::{GreedyBlockEncodeContext, GreedyBlockSource},
    greedy_ext_block::GreedyExtDictBlockSource,
    ldm::{
        opt::{LdmOptCursor, LdmRawSeqStore},
        sequence::{
            fill_prefix_hash_table, generate_sequences_no_dict, generate_sequences_with_prefix,
        },
        LdmHashTable,
    },
    opt_block::prime_btultra2_stats_no_dict,
    opt_dict::load_prefix,
    opt_encode::{
        encode_block_btopt_no_dict_with_state_and_policy,
        encode_block_btultra_no_dict_with_state_and_policy,
        encode_block_opt_ext_dict_with_state_and_policy_and_ldm,
        encode_block_opt_no_dict_with_state_and_policy_and_ldm,
    },
    opt_state::OptBlockState,
    params::{CompressionParameters, Strategy},
    sequence_store::RepeatOffsets,
};
use crate::encoding::blocks::BlockCompressionConfig;

const ZSTD_PREDEF_THRESHOLD: usize = 8;

pub(crate) fn encode_frame_btopt_no_dict(src: &[u8], level: i32) -> Vec<u8> {
    encode_frame_opt_no_dict(src, level, OptFrameStrategy::BtOpt)
}

pub(crate) fn encode_frame_btultra_no_dict(src: &[u8], level: i32) -> Vec<u8> {
    encode_frame_opt_no_dict(src, level, OptFrameStrategy::BtUltra)
}

pub(crate) fn encode_frame_btultra2_no_dict(src: &[u8], level: i32) -> Vec<u8> {
    encode_frame_opt_no_dict(src, level, OptFrameStrategy::BtUltra2)
}

pub(crate) fn encode_frame_btopt_with_dictionary(
    src: &[u8],
    level: i32,
    dictionary: ParsedDictionary<'_>,
) -> Vec<u8> {
    encode_frame_opt_with_dictionary(src, level, dictionary, OptFrameStrategy::BtOpt)
}

pub(crate) fn encode_frame_btultra_with_dictionary(
    src: &[u8],
    level: i32,
    dictionary: ParsedDictionary<'_>,
) -> Vec<u8> {
    encode_frame_opt_with_dictionary(src, level, dictionary, OptFrameStrategy::BtUltra)
}

pub(crate) fn encode_frame_btultra2_with_dictionary(
    src: &[u8],
    level: i32,
    dictionary: ParsedDictionary<'_>,
) -> Vec<u8> {
    let selected = select_block_compressor(
        Strategy::BtUltra2,
        ParamSwitch::Disable,
        DictionaryMode::DictMatchState,
    )
    .expect("C supports btultra2 dictionary routing through btultra");
    encode_frame_opt_with_dictionary(
        src,
        level,
        dictionary,
        selected_opt_frame_strategy(selected.strategy),
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OptFrameStrategy {
    BtOpt,
    BtUltra,
    BtUltra2,
}

fn selected_opt_frame_strategy(strategy: Strategy) -> OptFrameStrategy {
    match strategy {
        Strategy::BtOpt => OptFrameStrategy::BtOpt,
        Strategy::BtUltra => OptFrameStrategy::BtUltra,
        Strategy::BtUltra2 => OptFrameStrategy::BtUltra2,
        _ => unreachable!("only optimal strategies route through opt_frame"),
    }
}

fn opt_parser_strategy(strategy: OptFrameStrategy) -> super::opt_state::OptParserStrategy {
    match strategy {
        OptFrameStrategy::BtOpt => super::opt_state::OptParserStrategy::BtOpt,
        OptFrameStrategy::BtUltra | OptFrameStrategy::BtUltra2 => {
            super::opt_state::OptParserStrategy::BtUltra
        }
    }
}

fn encode_frame_opt_no_dict(src: &[u8], level: i32, strategy: OptFrameStrategy) -> Vec<u8> {
    let mut output = Vec::new();
    let cctx = CctxParameters::for_level(level, src.len() as u64, 0);
    cctx.assert_resolved();
    let params = cctx.compression;
    let post_block_splitter = cctx.post_block_splitter == ParamSwitch::Enable;
    let ldm_sequences = if cctx.ldm.enable_ldm == ParamSwitch::Enable {
        let mut ldm_table = LdmHashTable::new(cctx.ldm);
        Some(generate_sequences_no_dict(src, cctx.ldm, &mut ldm_table))
    } else {
        None
    };
    let mut ldm_store = ldm_sequences
        .as_ref()
        .map(|result| LdmRawSeqStore::new(&result.sequences));
    write_frame_header_no_dict(&mut output, src.len(), params);
    let mut frame_state = FrameBlockState::new(params, output.len());
    let mut opt_state = OptBlockState::new();
    opt_state.reset_for_frame(params);

    if src.is_empty() {
        let encoded_block = encode_block_opt_no_dict_with_state(
            GreedyBlockSource {
                src,
                block_range: 0..0,
                loaded_dict_end: 0,
            },
            true,
            params,
            frame_state.block_config,
            frame_state.repeat_offsets,
            &mut opt_state,
            GreedyBlockEncodeContext {
                previous_huff_table: None,
                fse_tables: &mut frame_state.fse_tables,
                offset_history: &mut frame_state.offset_history,
            },
            strategy,
            post_block_splitter,
            FrameBlockState::block_policy(true),
            None,
        );
        output.extend_from_slice(&encoded_block.bytes);
        return output;
    }

    let mut block_start = 0;
    while block_start < src.len() {
        let block_size = frame_state.next_block_size(&src[block_start..], params.strategy);
        let block_end = block_start + block_size;
        if block_start == 0
            && strategy == OptFrameStrategy::BtUltra2
            && src[block_start..block_end].len() > ZSTD_PREDEF_THRESHOLD
        {
            prime_btultra2_stats_no_dict(src, block_start..block_end, params, &mut opt_state);
        }
        let mut ldm_cursor =
            ldm_store.map(|store| LdmOptCursor::from_store_for_block(store, block_size as u32));

        let encoded_block = encode_block_opt_no_dict_with_state(
            GreedyBlockSource {
                src,
                block_range: block_start..block_end,
                loaded_dict_end: 0,
            },
            block_end == src.len(),
            params,
            frame_state.block_config,
            frame_state.repeat_offsets,
            &mut opt_state,
            GreedyBlockEncodeContext {
                previous_huff_table: frame_state.last_huff_table.as_ref(),
                fse_tables: &mut frame_state.fse_tables,
                offset_history: &mut frame_state.offset_history,
            },
            strategy,
            post_block_splitter,
            FrameBlockState::block_policy(block_start == 0),
            ldm_cursor.as_mut(),
        );
        if let Some(store) = ldm_store.as_mut() {
            store.skip_bytes(block_size as u32);
        }
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

fn encode_frame_opt_with_dictionary(
    src: &[u8],
    level: i32,
    dictionary: ParsedDictionary<'_>,
    strategy: OptFrameStrategy,
) -> Vec<u8> {
    let mut context = DictionaryFrameContext::new(src, level, dictionary);
    let params = context.cctx.compression;
    let post_block_splitter = context.cctx.post_block_splitter == ParamSwitch::Enable;
    let ldm_sequences = if context.cctx.ldm.enable_ldm == ParamSwitch::Enable {
        let mut ldm_table = LdmHashTable::new(context.cctx.ldm);
        fill_prefix_hash_table(
            &context.combined,
            0..context.dict_len,
            context.cctx.ldm,
            &mut ldm_table,
        );
        Some(generate_sequences_with_prefix(
            &context.combined,
            context.dict_len..context.combined.len(),
            context.cctx.ldm,
            &mut ldm_table,
        ))
    } else {
        None
    };
    let mut ldm_store = ldm_sequences
        .as_ref()
        .map(|result| LdmRawSeqStore::new(&result.sequences));

    let mut opt_state = OptBlockState::new();
    opt_state.reset_for_frame(params);
    load_prefix(&mut opt_state, &context.combined, context.dict_len, params);

    if src.is_empty() {
        let encoded_block = encode_block_opt_no_dict_with_state(
            GreedyBlockSource {
                src,
                block_range: 0..0,
                loaded_dict_end: context.dict_len,
            },
            true,
            params,
            context.frame_state.block_config,
            context.frame_state.repeat_offsets,
            &mut opt_state,
            GreedyBlockEncodeContext {
                previous_huff_table: context.frame_state.last_huff_table.as_ref(),
                fse_tables: &mut context.frame_state.fse_tables,
                offset_history: &mut context.frame_state.offset_history,
            },
            strategy,
            post_block_splitter,
            FrameBlockState::block_policy(true),
            None,
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
        if block_start == context.dict_len
            && strategy == OptFrameStrategy::BtUltra2
            && context.combined[block_start..block_end].len() > ZSTD_PREDEF_THRESHOLD
        {
            prime_btultra2_stats_no_dict(
                &context.combined,
                block_start..block_end,
                params,
                &mut opt_state,
            );
        }

        let mut ldm_cursor =
            ldm_store.map(|store| LdmOptCursor::from_store_for_block(store, block_size as u32));
        let loaded_dict_end = context.loaded_dict_end_for_block(block_end, params);
        let block_context = GreedyBlockEncodeContext {
            previous_huff_table: context.frame_state.last_huff_table.as_ref(),
            fse_tables: &mut context.frame_state.fse_tables,
            offset_history: &mut context.frame_state.offset_history,
        };
        let policy = FrameBlockState::block_policy(block_start == context.dict_len);
        let encoded_block = if loaded_dict_end == 0 {
            encode_block_opt_no_dict_with_state(
                GreedyBlockSource {
                    src: &context.combined,
                    block_range: block_start..block_end,
                    loaded_dict_end,
                },
                block_end == src_end,
                params,
                context.frame_state.block_config,
                context.frame_state.repeat_offsets,
                &mut opt_state,
                block_context,
                strategy,
                post_block_splitter,
                policy,
                ldm_cursor.as_mut(),
            )
        } else {
            encode_block_opt_ext_dict_with_state_and_policy_and_ldm(
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
                &mut opt_state,
                block_context,
                opt_parser_strategy(strategy),
                post_block_splitter,
                policy,
                ldm_cursor.as_mut(),
            )
        };
        if let Some(store) = ldm_store.as_mut() {
            store.skip_bytes(block_size as u32);
        }
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

#[allow(clippy::too_many_arguments)]
fn encode_block_opt_no_dict_with_state(
    source: GreedyBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    opt_state: &mut OptBlockState,
    context: GreedyBlockEncodeContext<'_, '_>,
    strategy: OptFrameStrategy,
    post_block_splitter: bool,
    policy: BlockEncodingPolicy,
    ldm_cursor: Option<&mut LdmOptCursor<'_>>,
) -> super::greedy_block::GreedyEncodedBlock {
    match strategy {
        OptFrameStrategy::BtOpt => {
            if ldm_cursor.is_some() {
                encode_block_opt_no_dict_with_state_and_policy_and_ldm(
                    source,
                    last_block,
                    params,
                    config,
                    repeat_offsets,
                    opt_state,
                    context,
                    super::opt_state::OptParserStrategy::BtOpt,
                    post_block_splitter,
                    policy,
                    ldm_cursor,
                )
            } else {
                encode_block_btopt_no_dict_with_state_and_policy(
                    source,
                    last_block,
                    params,
                    config,
                    repeat_offsets,
                    opt_state,
                    context,
                    post_block_splitter,
                    policy,
                )
            }
        }
        OptFrameStrategy::BtUltra | OptFrameStrategy::BtUltra2 => {
            debug_assert!(matches!(
                params.strategy,
                Strategy::BtUltra | Strategy::BtUltra2
            ));
            if strategy == OptFrameStrategy::BtUltra2 || ldm_cursor.is_some() {
                encode_block_opt_no_dict_with_state_and_policy_and_ldm(
                    source,
                    last_block,
                    params,
                    config,
                    repeat_offsets,
                    opt_state,
                    context,
                    super::opt_state::OptParserStrategy::BtUltra,
                    post_block_splitter,
                    policy,
                    ldm_cursor,
                )
            } else {
                encode_block_btultra_no_dict_with_state_and_policy(
                    source,
                    last_block,
                    params,
                    config,
                    repeat_offsets,
                    opt_state,
                    context,
                    post_block_splitter,
                    policy,
                )
            }
        }
    }
}
