//! Adapters from C fast sequences to the existing Rust block encoder.

use alloc::vec::Vec;
use core::ops::Range;

use super::block_emit::{
    append_prepared_block_or_raw, append_special_block, PreparedBlockEmission,
};
use super::block_policy::BlockEncodingPolicy;
use super::fast::{
    compress_block_fast_no_dict, compress_block_fast_no_dict_with_state_and_loaded_dict,
    FastBlockOutput, FastMatchState,
};
use super::fast_ext::compress_block_fast_ext_dict_with_state;
use super::params::CompressionParameters;
use super::sequence_store::RepeatOffsets;
use crate::{
    encoding::{
        blocks::{BlockCompressionConfig, PreparedBlock, PreparedSequence},
        frame_compressor::{FseTableSnapshot, FseTables, OffsetHistory},
    },
    huff0::huff0_encoder::HuffmanTable,
};

pub(crate) struct FastPreparedBlock {
    pub(crate) prepared: PreparedBlock,
    pub(crate) repeat_offsets: RepeatOffsets,
}

pub(crate) struct FastEncodedBlock {
    pub(crate) bytes: Vec<u8>,
    pub(crate) repeat_offsets: RepeatOffsets,
    pub(crate) new_huffman_table: Option<HuffmanTable>,
}

pub(crate) struct FastBlockEncoding {
    pub(crate) repeat_offsets: RepeatOffsets,
    pub(crate) new_huffman_table: Option<HuffmanTable>,
}

pub(crate) struct FastBlockEncodeContext<'a, 'table> {
    pub(crate) previous_huff_table: Option<&'table HuffmanTable>,
    pub(crate) fse_tables: &'a mut FseTables,
    pub(crate) offset_history: &'a mut OffsetHistory,
}

pub(crate) struct FastBlockSource<'a> {
    pub(crate) src: &'a [u8],
    pub(crate) block_range: Range<usize>,
    pub(crate) loaded_dict_end: usize,
}

pub(crate) struct FastExtDictBlockSource<'a> {
    pub(crate) src: &'a [u8],
    pub(crate) block_range: Range<usize>,
    pub(crate) dict_limit: usize,
    pub(crate) loaded_dict_end: usize,
}

pub(crate) fn prepare_block_fast_no_dict(
    src: &[u8],
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
) -> FastPreparedBlock {
    let output = compress_block_fast_no_dict(src, params, repeat_offsets);
    let prepared = prepare_from_fast_output(src, repeat_offsets, &output);

    FastPreparedBlock {
        prepared,
        repeat_offsets: output.repeat_offsets,
    }
}

pub(crate) fn prepare_block_fast_no_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut FastMatchState,
) -> FastPreparedBlock {
    prepare_block_fast_no_dict_with_state_and_loaded_dict(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        0,
    )
}

pub(crate) fn prepare_block_fast_no_dict_with_state_and_loaded_dict(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut FastMatchState,
    loaded_dict_end: usize,
) -> FastPreparedBlock {
    let block = &src[block_range.clone()];
    let output = compress_block_fast_no_dict_with_state_and_loaded_dict(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        loaded_dict_end,
    );
    let prepared = prepare_from_fast_output(block, repeat_offsets, &output);

    FastPreparedBlock {
        prepared,
        repeat_offsets: output.repeat_offsets,
    }
}

pub(crate) fn prepare_block_fast_ext_dict_with_state(
    source: FastExtDictBlockSource<'_>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut FastMatchState,
) -> FastPreparedBlock {
    let block = &source.src[source.block_range.clone()];
    let output = compress_block_fast_ext_dict_with_state(
        source.src,
        source.block_range,
        source.dict_limit,
        params,
        repeat_offsets,
        state,
        source.loaded_dict_end,
    );
    let prepared = prepare_from_fast_output(block, repeat_offsets, &output);

    FastPreparedBlock {
        prepared,
        repeat_offsets: output.repeat_offsets,
    }
}

pub(crate) fn encode_block_fast_no_dict(
    src: &[u8],
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    context: FastBlockEncodeContext<'_, '_>,
) -> FastEncodedBlock {
    encode_block_fast_no_dict_with_policy(
        src,
        last_block,
        params,
        config,
        repeat_offsets,
        context,
        BlockEncodingPolicy::normal(),
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn encode_block_fast_no_dict_with_policy(
    src: &[u8],
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    context: FastBlockEncodeContext<'_, '_>,
    policy: BlockEncodingPolicy,
) -> FastEncodedBlock {
    let mut bytes = Vec::new();
    let encoding = append_block_fast_no_dict_with_policy(
        src,
        last_block,
        params,
        config,
        repeat_offsets,
        context,
        policy,
        &mut bytes,
    );

    FastEncodedBlock {
        bytes,
        repeat_offsets: encoding.repeat_offsets,
        new_huffman_table: encoding.new_huffman_table,
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn append_block_fast_no_dict_with_policy(
    src: &[u8],
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    context: FastBlockEncodeContext<'_, '_>,
    policy: BlockEncodingPolicy,
    output: &mut Vec<u8>,
) -> FastBlockEncoding {
    if append_special_block(src, last_block, output) {
        return FastBlockEncoding {
            repeat_offsets,
            new_huffman_table: None,
        };
    }

    let previous_fse = context.fse_tables.snapshot_previous();
    let previous_offsets = *context.offset_history;
    let prepared = prepare_block_fast_no_dict(src, params, repeat_offsets);
    encode_prepared_block(
        src,
        last_block,
        params,
        config,
        repeat_offsets,
        prepared,
        policy,
        previous_fse,
        previous_offsets,
        context,
        output,
    )
}

pub(crate) fn encode_block_fast_no_dict_with_state(
    source: FastBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    match_state: &mut FastMatchState,
    context: FastBlockEncodeContext<'_, '_>,
) -> FastEncodedBlock {
    encode_block_fast_no_dict_with_state_and_policy(
        source,
        last_block,
        params,
        config,
        repeat_offsets,
        match_state,
        context,
        BlockEncodingPolicy::normal(),
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn encode_block_fast_no_dict_with_state_and_policy(
    source: FastBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    match_state: &mut FastMatchState,
    context: FastBlockEncodeContext<'_, '_>,
    policy: BlockEncodingPolicy,
) -> FastEncodedBlock {
    let mut bytes = Vec::new();
    let encoding = append_block_fast_no_dict_with_state_and_policy(
        source,
        last_block,
        params,
        config,
        repeat_offsets,
        match_state,
        context,
        policy,
        &mut bytes,
    );

    FastEncodedBlock {
        bytes,
        repeat_offsets: encoding.repeat_offsets,
        new_huffman_table: encoding.new_huffman_table,
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn append_block_fast_no_dict_with_state_and_policy(
    source: FastBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    match_state: &mut FastMatchState,
    context: FastBlockEncodeContext<'_, '_>,
    policy: BlockEncodingPolicy,
    output: &mut Vec<u8>,
) -> FastBlockEncoding {
    let block = &source.src[source.block_range.clone()];

    if append_special_block(block, last_block, output) {
        return FastBlockEncoding {
            repeat_offsets,
            new_huffman_table: None,
        };
    }

    let previous_fse = context.fse_tables.snapshot_previous();
    let previous_offsets = *context.offset_history;
    let prepared = prepare_block_fast_no_dict_with_state_and_loaded_dict(
        source.src,
        source.block_range.clone(),
        params,
        repeat_offsets,
        match_state,
        source.loaded_dict_end,
    );
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
        output,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn encode_block_fast_ext_dict_with_state_and_policy(
    source: FastExtDictBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    match_state: &mut FastMatchState,
    context: FastBlockEncodeContext<'_, '_>,
    policy: BlockEncodingPolicy,
) -> FastEncodedBlock {
    let mut bytes = Vec::new();
    let encoding = append_block_fast_ext_dict_with_state_and_policy(
        source,
        last_block,
        params,
        config,
        repeat_offsets,
        match_state,
        context,
        policy,
        &mut bytes,
    );

    FastEncodedBlock {
        bytes,
        repeat_offsets: encoding.repeat_offsets,
        new_huffman_table: encoding.new_huffman_table,
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn append_block_fast_ext_dict_with_state_and_policy(
    source: FastExtDictBlockSource<'_>,
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    match_state: &mut FastMatchState,
    context: FastBlockEncodeContext<'_, '_>,
    policy: BlockEncodingPolicy,
    output: &mut Vec<u8>,
) -> FastBlockEncoding {
    let block = &source.src[source.block_range.clone()];

    if append_special_block(block, last_block, output) {
        return FastBlockEncoding {
            repeat_offsets,
            new_huffman_table: None,
        };
    }

    let previous_fse = context.fse_tables.snapshot_previous();
    let previous_offsets = *context.offset_history;
    let prepared =
        prepare_block_fast_ext_dict_with_state(source, params, repeat_offsets, match_state);
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
        output,
    )
}

#[allow(clippy::too_many_arguments)]
fn encode_prepared_block(
    block: &[u8],
    last_block: bool,
    params: CompressionParameters,
    config: BlockCompressionConfig,
    repeat_offsets: RepeatOffsets,
    prepared: FastPreparedBlock,
    policy: BlockEncodingPolicy,
    previous_fse: FseTableSnapshot,
    previous_offsets: OffsetHistory,
    context: FastBlockEncodeContext<'_, '_>,
    output: &mut Vec<u8>,
) -> FastBlockEncoding {
    let compressed_repeat_offsets = prepared.repeat_offsets;
    match append_prepared_block_or_raw(
        block,
        last_block,
        params.strategy,
        policy,
        config,
        prepared.prepared.as_ref(),
        previous_fse,
        previous_offsets,
        context.previous_huff_table,
        context.fse_tables,
        context.offset_history,
        output,
    ) {
        PreparedBlockEmission::Raw | PreparedBlockEmission::Rle => FastBlockEncoding {
            repeat_offsets,
            new_huffman_table: None,
        },
        PreparedBlockEmission::Compressed { new_huffman_table } => FastBlockEncoding {
            repeat_offsets: compressed_repeat_offsets,
            new_huffman_table,
        },
    }
}

fn prepare_from_fast_output(
    src: &[u8],
    initial_repeat_offsets: RepeatOffsets,
    output: &FastBlockOutput,
) -> PreparedBlock {
    let mut literals = Vec::with_capacity(src.len());
    let mut sequences = Vec::with_capacity(output.sequences.len());
    let mut repeat_offsets = initial_repeat_offsets;
    let mut anchor = 0_usize;

    for sequence in &output.sequences {
        let lit_len = sequence.lit_len as usize;
        let match_len = sequence.match_len as usize;
        let lit_end = anchor + lit_len;
        debug_assert!(lit_end <= src.len());
        literals.extend_from_slice(&src[anchor..lit_end]);

        let raw_offset = repeat_offsets.resolve(sequence.off_base, sequence.lit_len);
        sequences.push(PreparedSequence {
            ll: sequence.lit_len,
            ml: sequence.match_len,
            raw_offset,
            encoded_offset_value: Some(sequence.off_base.to_c_value()),
        });
        repeat_offsets.update(sequence.off_base, sequence.lit_len);
        anchor = lit_end + match_len;
        debug_assert!(anchor <= src.len());
    }

    let tail_end = anchor + output.last_literals as usize;
    debug_assert_eq!(tail_end, src.len());
    literals.extend_from_slice(&src[anchor..tail_end]);

    PreparedBlock {
        literals,
        sequences,
    }
}
