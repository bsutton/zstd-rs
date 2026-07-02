//! Frame-level adapter for the C optimal no-dictionary path.

use alloc::vec::Vec;

use super::{
    block_policy::BlockEncodingPolicy,
    c_frame_header::{write_frame_header, write_frame_header_no_dict},
    cctx_params::{CctxParameters, ParamSwitch},
    dictionary::ParsedDictionary,
    frame_state::FrameBlockState,
    greedy_block::{GreedyBlockEncodeContext, GreedyBlockSource},
    ldm::{
        opt::{LdmOptCursor, LdmRawSeqStore},
        sequence::generate_sequences_no_dict,
        LdmHashTable,
    },
    opt_block::prime_btultra2_stats_no_dict,
    opt_dict::load_prefix,
    opt_encode::{
        encode_block_btopt_no_dict_with_state_and_policy,
        encode_block_btultra_no_dict_with_state_and_policy,
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
    encode_frame_opt_with_dictionary(src, level, dictionary, OptFrameStrategy::BtUltra2)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OptFrameStrategy {
    BtOpt,
    BtUltra,
    BtUltra2,
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
    let mut combined = Vec::with_capacity(dictionary.content.len() + src.len());
    combined.extend_from_slice(dictionary.content);
    combined.extend_from_slice(src);

    let dict_len = dictionary.content.len();
    let cctx = CctxParameters::for_level(level, src.len() as u64, dict_len);
    cctx.assert_resolved();
    let params = cctx.compression;
    let post_block_splitter = cctx.post_block_splitter == ParamSwitch::Enable;
    let mut output = Vec::new();
    let dictionary_id = (dictionary.dict_id != 0).then_some(dictionary.dict_id);
    write_frame_header(&mut output, src.len(), params, dictionary_id);
    let mut frame_state = FrameBlockState::with_dictionary(params, output.len(), &dictionary);

    let mut opt_state = OptBlockState::new();
    opt_state.reset_for_frame(params);
    load_prefix(&mut opt_state, &combined, dict_len, params);

    if src.is_empty() {
        let encoded_block = encode_block_opt_no_dict_with_state(
            GreedyBlockSource {
                src,
                block_range: 0..0,
            },
            true,
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
            FrameBlockState::block_policy(true),
            None,
        );
        output.extend_from_slice(&encoded_block.bytes);
        return output;
    }

    let mut block_start = dict_len;
    let src_end = combined.len();
    while block_start < src_end {
        let block_size =
            frame_state.next_block_size(&combined[block_start..src_end], params.strategy);
        let block_end = block_start + block_size;
        if block_start == dict_len
            && strategy == OptFrameStrategy::BtUltra2
            && combined[block_start..block_end].len() > ZSTD_PREDEF_THRESHOLD
        {
            prime_btultra2_stats_no_dict(&combined, block_start..block_end, params, &mut opt_state);
        }

        let encoded_block = encode_block_opt_no_dict_with_state(
            GreedyBlockSource {
                src: &combined,
                block_range: block_start..block_end,
            },
            block_end == src_end,
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
            FrameBlockState::block_policy(block_start == dict_len),
            None,
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
