//! Frame-level adapter for the C optimal no-dictionary path.

use alloc::vec::Vec;

use super::{
    greedy_block::{
        encode_block_btopt_no_dict_with_state, encode_block_btultra_no_dict_with_state,
        GreedyBlockEncodeContext, GreedyBlockSource,
    },
    opt_block::prime_btultra2_stats_no_dict,
    opt_state::OptBlockState,
    params::{CompressionParameters, Strategy},
    sequence_store::RepeatOffsets,
};
use crate::{
    common::MAX_BLOCK_SIZE,
    encoding::{
        blocks::BlockCompressionConfig,
        frame_compressor::{FseTables, OffsetHistory},
        frame_header::FrameHeader,
    },
};

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OptFrameStrategy {
    BtOpt,
    BtUltra,
    BtUltra2,
}

fn encode_frame_opt_no_dict(src: &[u8], level: i32, strategy: OptFrameStrategy) -> Vec<u8> {
    let mut output = Vec::new();
    FrameHeader {
        frame_content_size: Some(src.len() as u64),
        single_segment: true,
        content_checksum: false,
        dictionary_id: None,
        window_size: None,
    }
    .serialize(&mut output);

    let mut fse_tables = FseTables::new();
    let mut offset_history = OffsetHistory::new();
    let mut opt_state = OptBlockState::new();
    let mut last_huff_table = None;
    let mut repeat_offsets = RepeatOffsets::new();
    let params = CompressionParameters::for_level(level, src.len() as u64, 0);
    let block_config = BlockCompressionConfig::for_c_strategy(params.strategy as u8);

    if src.is_empty() {
        let encoded_block = encode_block_opt_no_dict_with_state(
            GreedyBlockSource {
                src,
                block_range: 0..0,
            },
            true,
            params,
            block_config,
            repeat_offsets,
            &mut opt_state,
            GreedyBlockEncodeContext {
                previous_huff_table: None,
                fse_tables: &mut fse_tables,
                offset_history: &mut offset_history,
            },
            strategy,
        );
        output.extend_from_slice(&encoded_block.bytes);
        return output;
    }

    let mut block_start = 0;
    while block_start < src.len() {
        let block_end = (block_start + MAX_BLOCK_SIZE as usize).min(src.len());
        if block_start == 0
            && strategy == OptFrameStrategy::BtUltra2
            && src[block_start..block_end].len() > ZSTD_PREDEF_THRESHOLD
        {
            prime_btultra2_stats_no_dict(src, block_start..block_end, params, &mut opt_state);
        }

        let encoded_block = encode_block_opt_no_dict_with_state(
            GreedyBlockSource {
                src,
                block_range: block_start..block_end,
            },
            block_end == src.len(),
            params,
            block_config,
            repeat_offsets,
            &mut opt_state,
            GreedyBlockEncodeContext {
                previous_huff_table: last_huff_table.as_ref(),
                fse_tables: &mut fse_tables,
                offset_history: &mut offset_history,
            },
            strategy,
        );
        repeat_offsets = encoded_block.repeat_offsets;
        last_huff_table = encoded_block.new_huffman_table;
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
) -> super::greedy_block::GreedyEncodedBlock {
    match strategy {
        OptFrameStrategy::BtOpt => encode_block_btopt_no_dict_with_state(
            source,
            last_block,
            params,
            config,
            repeat_offsets,
            opt_state,
            context,
        ),
        OptFrameStrategy::BtUltra | OptFrameStrategy::BtUltra2 => {
            debug_assert!(matches!(
                params.strategy,
                Strategy::BtUltra | Strategy::BtUltra2
            ));
            encode_block_btultra_no_dict_with_state(
                source,
                last_block,
                params,
                config,
                repeat_offsets,
                opt_state,
                context,
            )
        }
    }
}
