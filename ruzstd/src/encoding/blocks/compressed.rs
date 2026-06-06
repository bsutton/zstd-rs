use alloc::vec::Vec;
use core::convert::TryFrom;

mod config;
mod literals;
mod sequence_codes;
mod sequence_tables;

pub(crate) use config::BlockCompressionConfig;
use config::HuffmanTableSearch;
use literals::{
    compress_literals, raw_literals, should_compress_literals, COMPRESS_LITERALS_SIZE_MIN,
};
#[cfg(test)]
use literals::{
    compressed_literals_header_len, compressed_literals_size_format,
    literal_estimate_has_enough_gain, literal_min_gain, rle_literals, LiteralStats,
    REPEAT_LITERALS_SIZE_MIN,
};
use sequence_codes::{encode_literal_length, encode_match_len, encode_offset};
pub(crate) use sequence_codes::{literal_length_code, match_length_code, offset_code};
use sequence_tables::{
    choose_sequence_table_modes, encode_fse_table_modes, encode_table, FseTableMode,
    SequenceModeSearchConfig,
};
#[cfg(test)]
use sequence_tables::{choose_table, exact_sequence_section_size};

#[cfg(test)]
mod tests;

#[cfg(test)]
use crate::fse::fse_encoder::build_table_from_data;
use crate::{
    bit_io::BitWriter,
    encoding::frame_compressor::{CompressState, FseTables, OffsetHistory},
    encoding::util::likely_dependency_json_lockfile_text,
    encoding::{CompressionFileProfile, Matcher, Sequence},
    fse::fse_encoder::{FSETable, State},
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
    let mut new_huffman_table = None;
    let mut next_offset_history = *offset_history;
    let sequences = encode_sequences_for_history(prepared.sequences, &mut next_offset_history);

    // literals section

    let mut writer = BitWriter::from(output);
    if should_compress_literals(prepared.literals.len(), previous_huff_table.is_some()) {
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
            search_smallest_huffman_table,
            config.file_type_single_stream_huffman_max_literals,
            &mut writer,
        ) {
            new_huffman_table = Some(table);
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
        let ll_previous = fse_tables.ll_previous.clone();
        let ml_previous = fse_tables.ml_previous.clone();
        let of_previous = fse_tables.of_previous.clone();
        let (ll_mode, ml_mode, of_mode) = choose_sequence_table_modes(
            &sequences,
            SequenceModeSearchConfig {
                ll_previous: ll_previous.as_ref(),
                ll_default: &fse_tables.ll_default,
                ml_previous: ml_previous.as_ref(),
                ml_default: &fse_tables.ml_default,
                of_previous: of_previous.as_ref(),
                of_default: &fse_tables.of_default,
                repeat_table_max_sequences: config.repeat_table_max_sequences,
                llml_predefined_max_sequences:
                    file_type_small_sequence_predefined_llml_max_sequences,
                of_predefined_max_sequences: config.offset_predefined_max_sequences,
                of_max_log: config.offset_table_max_log,
                exact_sequence_mode_search: config.exact_sequence_mode_search,
                c_fast_heuristics: config.c_fast_sequence_table_heuristics,
            },
        );

        writer.write_bits(encode_fse_table_modes(&ll_mode, &ml_mode, &of_mode), 8);

        encode_table(&ll_mode, &mut writer);
        encode_table(&of_mode, &mut writer);
        encode_table(&ml_mode, &mut writer);

        encode_sequences(&sequences, &mut writer, &ll_mode, &ml_mode, &of_mode);

        match ll_mode {
            FseTableMode::Encoded(table) => fse_tables.ll_previous = Some(table),
            FseTableMode::Predefined(_) => fse_tables.ll_previous = None,
            FseTableMode::Rle(_) => fse_tables.ll_previous = None,
            FseTableMode::RepeatLast(_) => {}
        }
        match ml_mode {
            FseTableMode::Encoded(table) => fse_tables.ml_previous = Some(table),
            FseTableMode::Predefined(_) => fse_tables.ml_previous = None,
            FseTableMode::Rle(_) => fse_tables.ml_previous = None,
            FseTableMode::RepeatLast(_) => {}
        }
        match of_mode {
            FseTableMode::Encoded(table) => fse_tables.of_previous = Some(table),
            FseTableMode::Predefined(_) => fse_tables.of_previous = None,
            FseTableMode::Rle(_) => fse_tables.of_previous = None,
            FseTableMode::RepeatLast(_) => {}
        }
    }
    writer.flush();
    *offset_history = next_offset_history;
    new_huffman_table
}

fn encode_sequences_for_history(
    sequences: &[PreparedSequence],
    offset_history: &mut OffsetHistory,
) -> Vec<crate::blocks::sequence_section::Sequence> {
    let mut encoded = Vec::with_capacity(sequences.len());
    for sequence in sequences {
        encoded.push(crate::blocks::sequence_section::Sequence {
            ll: sequence.ll,
            ml: sequence.ml,
            of: offset_history.encode_offset_value(sequence.raw_offset, sequence.ll),
        });
    }
    encoded
}

#[inline(always)]
fn offset_to_u32(offset: usize) -> u32 {
    match u32::try_from(offset) {
        Ok(offset) => offset,
        Err(_) => unreachable!("match offsets are bounded by the compressor window"),
    }
}

fn encode_sequences(
    sequences: &[crate::blocks::sequence_section::Sequence],
    writer: &mut BitWriter<&mut Vec<u8>>,
    ll_mode: &FseTableMode<'_>,
    ml_mode: &FseTableMode<'_>,
    of_mode: &FseTableMode<'_>,
) {
    if let (
        FseTableMode::Rle(ll_symbol),
        FseTableMode::Rle(ml_symbol),
        FseTableMode::Rle(of_symbol),
    ) = (ll_mode, ml_mode, of_mode)
    {
        encode_rle_sequences(sequences, writer, *ll_symbol, *ml_symbol, *of_symbol);
        return;
    }

    let sequence = sequences[sequences.len() - 1];
    let ll_table = ll_mode.table();
    let ml_table = ml_mode.table();
    let of_table = of_mode.table();
    let (ll_code, ll_add_bits, ll_num_bits) = encode_literal_length(sequence.ll);
    let (of_code, of_add_bits, of_num_bits) = encode_offset(sequence.of);
    let (ml_code, ml_add_bits, ml_num_bits) = encode_match_len(sequence.ml);
    let mut ll_state = init_fse_state(ll_mode, ll_code);
    let mut ml_state = init_fse_state(ml_mode, ml_code);
    let mut of_state = init_fse_state(of_mode, of_code);

    writer.write_bits(ll_add_bits, ll_num_bits);
    writer.write_bits(ml_add_bits, ml_num_bits);
    writer.write_bits(of_add_bits, of_num_bits);

    // Encode backwards so the decoder reads the first sequence first.
    let mut sequence_idx = sequences.len() - 1;
    while sequence_idx > 0 {
        sequence_idx -= 1;
        let sequence = sequences[sequence_idx];
        let (ll_code, ll_add_bits, ll_num_bits) = encode_literal_length(sequence.ll);
        let (of_code, of_add_bits, of_num_bits) = encode_offset(sequence.of);
        let (ml_code, ml_add_bits, ml_num_bits) = encode_match_len(sequence.ml);

        {
            update_fse_state(of_table, &mut of_state, of_code, writer);
        }
        {
            update_fse_state(ml_table, &mut ml_state, ml_code, writer);
        }
        {
            update_fse_state(ll_table, &mut ll_state, ll_code, writer);
        }

        writer.write_bits(ll_add_bits, ll_num_bits);
        writer.write_bits(ml_add_bits, ml_num_bits);
        writer.write_bits(of_add_bits, of_num_bits);
    }
    flush_fse_state(ml_table, ml_state, writer);
    flush_fse_state(of_table, of_state, writer);
    flush_fse_state(ll_table, ll_state, writer);

    let bits_to_fill = writer.misaligned();
    if bits_to_fill == 0 {
        writer.write_bits(1u32, 8);
    } else {
        writer.write_bits(1u32, bits_to_fill);
    }
}

fn encode_rle_sequences(
    sequences: &[crate::blocks::sequence_section::Sequence],
    writer: &mut BitWriter<&mut Vec<u8>>,
    ll_symbol: u8,
    ml_symbol: u8,
    of_symbol: u8,
) {
    for sequence in sequences.iter().rev() {
        let (ll_code, ll_add_bits, ll_num_bits) = encode_literal_length(sequence.ll);
        let (of_code, of_add_bits, of_num_bits) = encode_offset(sequence.of);
        let (ml_code, ml_add_bits, ml_num_bits) = encode_match_len(sequence.ml);
        debug_assert_eq!(ll_code, ll_symbol);
        debug_assert_eq!(ml_code, ml_symbol);
        debug_assert_eq!(of_code, of_symbol);

        writer.write_bits(ll_add_bits, ll_num_bits);
        writer.write_bits(ml_add_bits, ml_num_bits);
        writer.write_bits(of_add_bits, of_num_bits);
    }

    let bits_to_fill = writer.misaligned();
    if bits_to_fill == 0 {
        writer.write_bits(1u32, 8);
    } else {
        writer.write_bits(1u32, bits_to_fill);
    }
}

fn init_fse_state<'a>(mode: &'a FseTableMode<'_>, symbol: u8) -> Option<&'a State> {
    match mode {
        FseTableMode::Rle(rle_symbol) => {
            debug_assert_eq!(*rle_symbol, symbol);
            None
        }
        _ => mode.table().map(|table| table.start_state(symbol)),
    }
}

fn update_fse_state<'a>(
    table: Option<&'a FSETable>,
    state: &mut Option<&'a State>,
    symbol: u8,
    writer: &mut BitWriter<&mut Vec<u8>>,
) {
    if let Some(table) = table {
        if let Some(current) = *state {
            let next = table.next_state(symbol, current.index);
            let diff = current.index - next.baseline;
            writer.write_bits(diff as u64, next.num_bits as usize);
            *state = Some(next);
        } else {
            unreachable!("non-RLE FSE mode must have a state");
        }
    }
}

fn flush_fse_state(
    table: Option<&FSETable>,
    state: Option<&State>,
    writer: &mut BitWriter<&mut Vec<u8>>,
) {
    if let Some(table) = table {
        if let Some(state) = state {
            writer.write_bits(state.index as u64, table.acc_log() as usize);
        } else {
            unreachable!("non-RLE FSE mode must have a state");
        }
    }
}

fn encode_seqnum(seqnum: usize, writer: &mut BitWriter<impl AsMut<Vec<u8>>>) {
    const UPPER_LIMIT: usize = 0xFFFF + 0x7F00;
    match seqnum {
        1..=127 => writer.write_bits(seqnum as u32, 8),
        128..=0x7FFF => {
            let upper = ((seqnum >> 8) | 0x80) as u8;
            let lower = seqnum as u8;
            writer.write_bits(upper, 8);
            writer.write_bits(lower, 8);
        }
        0x8000..=UPPER_LIMIT => {
            let encode = seqnum - 0x7F00;
            let upper = (encode >> 8) as u8;
            let lower = encode as u8;
            writer.write_bits(255u8, 8);
            writer.write_bits(upper, 8);
            writer.write_bits(lower, 8);
        }
        _ => unreachable!(),
    }
}
