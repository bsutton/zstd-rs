use alloc::vec::Vec;

mod config;
mod estimate;
mod literals;
mod sequence_bitstream;
mod sequence_codes;
mod sequence_cost;
mod sequence_tables;

pub(crate) use config::BlockCompressionConfig;
use config::HuffmanTableSearch;
pub(crate) use estimate::{estimate_prepared_block_size_with_sequences, EstimateScratch};
use literals::{
    compress_literals, raw_literals, should_compress_literals, suspect_uncompressible_literals,
    LiteralCompressionOptions, COMPRESS_LITERALS_SIZE_MIN,
};
#[cfg(test)]
use literals::{
    compressed_literals_header_len, compressed_literals_size_format,
    literal_estimate_has_enough_gain, literal_min_gain, rle_literals, LiteralStats,
    REPEAT_LITERALS_SIZE_MIN,
};
use sequence_bitstream::{
    apply_fse_table_update, byte_size_between, encode_seqnum, encode_sequences,
    encode_sequences_for_history, encode_sequences_for_history_into, encode_table_count_size,
    fse_table_update, offset_to_u32, should_emit_raw_for_legacy_decoder,
};
use sequence_codes::{encode_literal_length, encode_match_len, encode_offset};
pub(crate) use sequence_codes::{literal_length_code, match_length_code, offset_code};
use sequence_tables::{
    choose_sequence_table_modes, encode_fse_table_modes, FseTableMode, SequenceModeSearchConfig,
};
#[cfg(test)]
use sequence_tables::{choose_table, encode_table, exact_sequence_section_size};

#[cfg(test)]
mod tests;

#[cfg(test)]
use crate::fse::fse_encoder::build_table_from_data;
use crate::{
    bit_io::BitWriter,
    encoding::frame_compressor::{CompressState, FseTables, OffsetHistory},
    encoding::util::likely_dependency_json_lockfile_text,
    encoding::{CompressionFileProfile, Matcher, Sequence},
    huff0::huff0_encoder,
};

const INITIAL_LITERALS_CAPACITY: usize = 1024;
const INITIAL_SEQUENCES_CAPACITY: usize = 256;
const SMALL_HUFFMAN_TABLE_SEARCH_MAX_LITERALS: usize = 256;
const SMALL_HUFFMAN_TABLE_SEARCH_MAX_SEQUENCES: usize = 2;
const FILE_TYPE_SMALL_HUFFMAN_TABLE_SEARCH_MAX_LITERALS: usize = 4 * 1024;
const EXACT_SEQUENCE_TABLE_MIN_LOG: u8 = 7;
pub(crate) struct PreparedBlock {
    pub(crate) literals: Vec<u8>,
    pub(crate) sequences: Vec<PreparedSequence>,
}

pub(crate) struct CompressedBlockResult {
    pub(crate) new_huffman_table: Option<huff0_encoder::HuffmanTable>,
    pub(crate) should_emit_raw_block: bool,
}

#[derive(Clone, Copy)]
pub(crate) struct PreparedBlockRef<'a> {
    pub(crate) literals: &'a [u8],
    pub(crate) sequences: &'a [PreparedSequence],
}

#[derive(Clone, Copy)]
pub(crate) struct PreparedSequence {
    pub(crate) ll: u32,
    pub(crate) ml: u32,
    pub(crate) raw_offset: u32,
    /// C-port matchers already choose explicit offsets versus repeat codes.
    /// Generic matchers leave this unset and let `OffsetHistory` choose.
    pub(crate) encoded_offset_value: Option<u32>,
}

impl PreparedBlock {
    pub(crate) fn as_ref(&self) -> PreparedBlockRef<'_> {
        PreparedBlockRef {
            literals: &self.literals,
            sequences: &self.sequences,
        }
    }
}

pub(crate) fn compress_block_with_config<M: Matcher>(
    state: &mut CompressState<M>,
    output: &mut Vec<u8>,
    config: BlockCompressionConfig,
) -> Option<huff0_encoder::HuffmanTable> {
    let mut config = config;
    if matches!(state.file_profile_hint, CompressionFileProfile::None)
        && likely_dependency_json_lockfile_text(state.matcher.get_last_space())
    {
        config.apply_dependency_json_lockfile_tuning();
    }
    let prepared = prepare_block(state);
    let previous_huff_table = state.last_huff_table.take();
    let result = compress_prepared_block(
        output,
        config,
        prepared.as_ref(),
        &mut state.fse_tables,
        &mut state.offset_history,
        previous_huff_table.as_ref(),
    );
    state.last_huff_table = previous_huff_table;
    result
}

pub(crate) fn prepare_block<M: Matcher>(state: &mut CompressState<M>) -> PreparedBlock {
    let mut literals_vec = Vec::with_capacity(INITIAL_LITERALS_CAPACITY);
    let mut sequences = Vec::with_capacity(INITIAL_SEQUENCES_CAPACITY);
    let (newest, second, third) = state.offset_history.as_offsets();
    state.matcher.set_repeat_offsets(newest, second, third);
    state.matcher.start_matching(|seq| match seq {
        Sequence::Literals { literals } => literals_vec.extend_from_slice(literals),
        Sequence::Triple {
            literals,
            offset,
            match_len,
        } => {
            literals_vec.extend_from_slice(literals);
            sequences.push(PreparedSequence {
                ll: literals.len() as u32,
                ml: match_len as u32,
                raw_offset: offset_to_u32(offset),
                encoded_offset_value: None,
            });
        }
    });

    PreparedBlock {
        literals: literals_vec,
        sequences,
    }
}

pub(crate) fn compress_prepared_block(
    output: &mut Vec<u8>,
    config: BlockCompressionConfig,
    prepared: PreparedBlockRef<'_>,
    fse_tables: &mut FseTables,
    offset_history: &mut OffsetHistory,
    previous_huff_table: Option<&huff0_encoder::HuffmanTable>,
) -> Option<huff0_encoder::HuffmanTable> {
    compress_prepared_block_with_huffman_repeat(
        output,
        config,
        prepared,
        fse_tables,
        offset_history,
        previous_huff_table,
        previous_huff_table.is_some(),
    )
}

pub(crate) fn compress_prepared_block_with_huffman_repeat(
    output: &mut Vec<u8>,
    config: BlockCompressionConfig,
    prepared: PreparedBlockRef<'_>,
    fse_tables: &mut FseTables,
    offset_history: &mut OffsetHistory,
    previous_huff_table: Option<&huff0_encoder::HuffmanTable>,
    previous_huff_table_is_valid: bool,
) -> Option<huff0_encoder::HuffmanTable> {
    compress_prepared_block_with_stats(
        output,
        config,
        prepared,
        fse_tables,
        offset_history,
        previous_huff_table,
        previous_huff_table_is_valid,
    )
    .new_huffman_table
}

pub(crate) fn compress_prepared_block_with_stats(
    output: &mut Vec<u8>,
    config: BlockCompressionConfig,
    prepared: PreparedBlockRef<'_>,
    fse_tables: &mut FseTables,
    offset_history: &mut OffsetHistory,
    previous_huff_table: Option<&huff0_encoder::HuffmanTable>,
    previous_huff_table_is_valid: bool,
) -> CompressedBlockResult {
    let mut result = CompressedBlockResult {
        new_huffman_table: None,
        should_emit_raw_block: false,
    };
    let mut next_offset_history = *offset_history;
    let sequences = encode_sequences_for_history(prepared.sequences, &mut next_offset_history);

    // literals section

    let mut writer = BitWriter::from(output);
    if !config.literal_compression_disabled
        && should_compress_literals(
            prepared.literals.len(),
            previous_huff_table_is_valid,
            config.literal_compression_min_size,
        )
    {
        let search_smallest_huffman_table = match config.huffman_table_search {
            HuffmanTableSearch::Heuristic => {
                sequences.is_empty()
                    || (sequences.len() <= SMALL_HUFFMAN_TABLE_SEARCH_MAX_SEQUENCES
                        && prepared.literals.len() <= SMALL_HUFFMAN_TABLE_SEARCH_MAX_LITERALS)
            }
            HuffmanTableSearch::FileTypeSmall => {
                prepared.literals.len() <= FILE_TYPE_SMALL_HUFFMAN_TABLE_SEARCH_MAX_LITERALS
                    || sequences.is_empty()
                    || (sequences.len() <= SMALL_HUFFMAN_TABLE_SEARCH_MAX_SEQUENCES
                        && prepared.literals.len() <= SMALL_HUFFMAN_TABLE_SEARCH_MAX_LITERALS)
            }
            HuffmanTableSearch::AllSections => true,
        };
        if let Some(table) = compress_literals(
            prepared.literals,
            previous_huff_table,
            previous_huff_table_is_valid,
            LiteralCompressionOptions {
                search_smallest_table: search_smallest_huffman_table,
                force_single_stream_max_literals: config
                    .file_type_single_stream_huffman_max_literals,
                suspect_uncompressible: suspect_uncompressible_literals(
                    prepared.literals.len(),
                    sequences.len(),
                ),
                c_literal_cost_model: config.c_literal_cost_model,
            },
            &mut writer,
        ) {
            result.new_huffman_table = Some(table);
        }
    } else {
        raw_literals(prepared.literals, &mut writer);
    }

    // sequences section

    if sequences.is_empty() {
        writer.write_bits(0u8, 8);
    } else {
        encode_seqnum(sequences.len(), &mut writer);

        // Choose the tables.
        let file_type_small_sequence_predefined_llml_max_sequences =
            if prepared.literals.len() >= COMPRESS_LITERALS_SIZE_MIN {
                config
                    .file_type_small_sequence_predefined_llml_max_sequences
                    .unwrap_or(16)
            } else {
                16
            };
        let (ll_mode, ml_mode, of_mode) = choose_sequence_table_modes(
            &sequences,
            SequenceModeSearchConfig {
                ll_previous: fse_tables.ll_previous.as_deref(),
                ll_default: &fse_tables.ll_default,
                ml_previous: fse_tables.ml_previous.as_deref(),
                ml_default: &fse_tables.ml_default,
                of_previous: fse_tables.of_previous.as_deref(),
                of_default: &fse_tables.of_default,
                repeat_table_max_sequences: config.repeat_table_max_sequences,
                llml_predefined_max_sequences:
                    file_type_small_sequence_predefined_llml_max_sequences,
                of_predefined_max_sequences: config.offset_predefined_max_sequences,
                of_max_log: config.offset_table_max_log,
                exact_sequence_mode_search: config.exact_sequence_mode_search,
                c_fast_heuristics: config.c_fast_sequence_table_heuristics,
                c_cost_model: config.c_cost_sequence_table_selection,
            },
        );

        writer.write_bits(encode_fse_table_modes(&ll_mode, &ml_mode, &of_mode), 8);

        let mut last_count_size = encode_table_count_size(&ll_mode, &mut writer);
        let off_count_size = encode_table_count_size(&of_mode, &mut writer);
        if off_count_size != 0 {
            last_count_size = off_count_size;
        }
        let ml_count_size = encode_table_count_size(&ml_mode, &mut writer);
        if ml_count_size != 0 {
            last_count_size = ml_count_size;
        }

        let bitstream_start = writer.index();
        encode_sequences(&sequences, &mut writer, &ll_mode, &ml_mode, &of_mode);
        let bitstream_size = byte_size_between(bitstream_start, writer.index());

        if should_emit_raw_for_legacy_decoder(last_count_size, bitstream_size) {
            result.should_emit_raw_block = true;
        }

        let ll_update = fse_table_update(ll_mode);
        let ml_update = fse_table_update(ml_mode);
        let of_update = fse_table_update(of_mode);
        apply_fse_table_update(&mut fse_tables.ll_previous, ll_update);
        apply_fse_table_update(&mut fse_tables.ml_previous, ml_update);
        apply_fse_table_update(&mut fse_tables.of_previous, of_update);
    }
    writer.flush();
    *offset_history = next_offset_history;
    result
}

pub(crate) fn append_predefined_sequence_section(
    sequences: &[PreparedSequence],
    fse_tables: &FseTables,
    offset_history: &mut OffsetHistory,
    output: &mut Vec<u8>,
) -> Option<usize> {
    if sequences.is_empty() {
        output.push(0);
        return Some(1);
    }

    let previous_offsets = *offset_history;
    let mut encoded_sequences = Vec::with_capacity(sequences.len());
    encode_sequences_for_history_into(sequences, offset_history, &mut encoded_sequences);

    let start = output.len();
    let mut writer = BitWriter::from(output);
    encode_seqnum(encoded_sequences.len(), &mut writer);
    let sequence_head_index = writer.index() / 8;
    let ll_mode = FseTableMode::Predefined(&fse_tables.ll_default);
    let ml_mode = FseTableMode::Predefined(&fse_tables.ml_default);
    let of_mode = FseTableMode::Predefined(&fse_tables.of_default);
    writer.write_bits(encode_fse_table_modes(&ll_mode, &ml_mode, &of_mode), 8);
    encode_sequences(
        &encoded_sequences,
        &mut writer,
        &ll_mode,
        &ml_mode,
        &of_mode,
    );
    writer.flush();

    let byte_size = writer.index() / 8 - start;
    if writer.index() / 8 - sequence_head_index < 4 {
        writer.reset_to(start * 8);
        *offset_history = previous_offsets;
        return None;
    }
    Some(byte_size)
}
