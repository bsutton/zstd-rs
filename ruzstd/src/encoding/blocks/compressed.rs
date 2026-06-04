use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryFrom;

mod config;

pub(crate) use config::BlockCompressionConfig;
use config::HuffmanTableSearch;

#[cfg(test)]
mod tests;

use crate::{
    bit_io::BitWriter,
    encoding::frame_compressor::{CompressState, FseTables, OffsetHistory},
    encoding::util::likely_dependency_json_lockfile_text,
    encoding::{CompressionFileProfile, Matcher, Sequence},
    fse::fse_encoder::{build_table_from_data, FSETable, State},
    huff0::huff0_encoder,
};

const INITIAL_LITERALS_CAPACITY: usize = 1024;
const INITIAL_SEQUENCES_CAPACITY: usize = 256;
const COMPRESS_LITERALS_SIZE_MIN: usize = 63;
const REPEAT_LITERALS_SIZE_MIN: usize = 6;
const HUFFMAN_4_STREAMS_MIN: usize = 256;
const REPEAT_SINGLE_STREAM_LITERALS_MAX: usize = 1024;
const SMALL_HUFFMAN_TABLE_SEARCH_MAX_LITERALS: usize = 256;
const SMALL_HUFFMAN_TABLE_SEARCH_MAX_SEQUENCES: usize = 2;
const FILE_TYPE_SMALL_HUFFMAN_TABLE_SEARCH_MAX_LITERALS: usize = 4 * 1024;
const FAST_LITERAL_MIN_GAIN_LOG: u32 = 6;
const EXACT_SEQUENCE_TABLE_MIN_LOG: u8 = 7;
const LITERAL_LENGTH_SMALL_CODES: [(u8, u32, usize); 64] = small_literal_length_codes();
const MATCH_LENGTH_SMALL_CODES: [(u8, u32, usize); 128] = small_match_length_codes();
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
            FseTableMode::RepeateLast(_) => {}
        }
        match ml_mode {
            FseTableMode::Encoded(table) => fse_tables.ml_previous = Some(table),
            FseTableMode::Predefined(_) => fse_tables.ml_previous = None,
            FseTableMode::Rle(_) => fse_tables.ml_previous = None,
            FseTableMode::RepeateLast(_) => {}
        }
        match of_mode {
            FseTableMode::Encoded(table) => fse_tables.of_previous = Some(table),
            FseTableMode::Predefined(_) => fse_tables.of_previous = None,
            FseTableMode::Rle(_) => fse_tables.of_previous = None,
            FseTableMode::RepeateLast(_) => {}
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

#[derive(Clone)]
#[allow(clippy::large_enum_variant)]
enum FseTableMode<'a> {
    Predefined(&'a FSETable),
    Rle(u8),
    Encoded(FSETable),
    RepeateLast(&'a FSETable),
}

impl FseTableMode<'_> {
    pub fn table(&self) -> Option<&FSETable> {
        match self {
            Self::Predefined(t) => Some(t),
            Self::RepeateLast(t) => Some(t),
            Self::Encoded(t) => Some(t),
            Self::Rle(_) => None,
        }
    }
}

struct SequenceModeSearchConfig<'a> {
    ll_previous: Option<&'a FSETable>,
    ll_default: &'a FSETable,
    ml_previous: Option<&'a FSETable>,
    ml_default: &'a FSETable,
    of_previous: Option<&'a FSETable>,
    of_default: &'a FSETable,
    repeat_table_max_sequences: usize,
    llml_predefined_max_sequences: usize,
    of_predefined_max_sequences: usize,
    of_max_log: u8,
    exact_sequence_mode_search: bool,
}

#[derive(Clone, Copy)]
struct TableModeCandidateConfig {
    max_log: u8,
    repeat_table_max_sequences: usize,
    predefined_max_sequences: usize,
    exact_sequence_mode_search: bool,
}

fn choose_table<'a>(
    previous: Option<&'a FSETable>,
    default_table: &'a FSETable,
    sequences: &[crate::blocks::sequence_section::Sequence],
    code: impl Fn(&crate::blocks::sequence_section::Sequence) -> u8 + Copy,
    max_log: u8,
    repeat_table_max_sequences: usize,
    predefined_max_sequences: usize,
) -> FseTableMode<'a> {
    let first_code = code(&sequences[0]);
    let all_same_code = sequences
        .iter()
        .skip(1)
        .all(|sequence| code(sequence) == first_code);

    if all_same_code && sequences.len() > 2 {
        return FseTableMode::Rle(first_code);
    }

    if sequences.len() <= predefined_max_sequences
        && sequences
            .iter()
            .all(|sequence| default_table.can_encode_symbol(code(sequence)))
    {
        return FseTableMode::Predefined(default_table);
    }

    if all_same_code {
        return FseTableMode::Rle(first_code);
    }

    if let Some(previous) = previous {
        if sequences.len() < repeat_table_max_sequences
            && sequences
                .iter()
                .all(|sequence| previous.can_encode_symbol(code(sequence)))
        {
            return FseTableMode::RepeateLast(previous);
        }
    }

    FseTableMode::Encoded(build_table_from_data(
        sequences.iter().map(code),
        max_log,
        true,
    ))
}

fn candidate_table_modes<'a>(
    previous: Option<&'a FSETable>,
    default_table: &'a FSETable,
    sequences: &[crate::blocks::sequence_section::Sequence],
    code: impl Fn(&crate::blocks::sequence_section::Sequence) -> u8 + Copy,
    config: TableModeCandidateConfig,
) -> Vec<FseTableMode<'a>> {
    let heuristic = choose_table(
        previous,
        default_table,
        sequences,
        code,
        config.max_log,
        config.repeat_table_max_sequences,
        config.predefined_max_sequences,
    );

    let mut candidates = vec![heuristic];
    let first_code = code(&sequences[0]);
    let all_same_code = sequences
        .iter()
        .skip(1)
        .all(|sequence| code(sequence) == first_code);

    if sequences.len() <= config.predefined_max_sequences
        && sequences
            .iter()
            .all(|sequence| default_table.can_encode_symbol(code(sequence)))
    {
        candidates.push(FseTableMode::Predefined(default_table));
    }

    if let Some(previous) = previous {
        if sequences.len() < config.repeat_table_max_sequences
            && sequences
                .iter()
                .all(|sequence| previous.can_encode_symbol(code(sequence)))
        {
            candidates.push(FseTableMode::RepeateLast(previous));
        }
    }

    if all_same_code {
        if sequences.len() > 2 {
            candidates.push(FseTableMode::Rle(first_code));
        }
    } else {
        let exact_min_log = if config.exact_sequence_mode_search {
            EXACT_SEQUENCE_TABLE_MIN_LOG.min(config.max_log)
        } else {
            config.max_log
        };
        for candidate_max_log in exact_min_log..=config.max_log {
            candidates.push(FseTableMode::Encoded(build_table_from_data(
                sequences.iter().map(code),
                candidate_max_log,
                true,
            )));
        }
    }

    candidates
}

fn choose_sequence_table_modes<'a>(
    sequences: &[crate::blocks::sequence_section::Sequence],
    config: SequenceModeSearchConfig<'a>,
) -> (FseTableMode<'a>, FseTableMode<'a>, FseTableMode<'a>) {
    let ll_candidates = candidate_table_modes(
        config.ll_previous,
        config.ll_default,
        sequences,
        |seq| encode_literal_length(seq.ll).0,
        TableModeCandidateConfig {
            max_log: 9,
            repeat_table_max_sequences: config.repeat_table_max_sequences,
            predefined_max_sequences: config.llml_predefined_max_sequences,
            exact_sequence_mode_search: config.exact_sequence_mode_search,
        },
    );
    let ml_candidates = candidate_table_modes(
        config.ml_previous,
        config.ml_default,
        sequences,
        |seq| encode_match_len(seq.ml).0,
        TableModeCandidateConfig {
            max_log: 9,
            repeat_table_max_sequences: config.repeat_table_max_sequences,
            predefined_max_sequences: config.llml_predefined_max_sequences,
            exact_sequence_mode_search: config.exact_sequence_mode_search,
        },
    );
    let of_candidates = candidate_table_modes(
        config.of_previous,
        config.of_default,
        sequences,
        |seq| encode_offset(seq.of).0,
        TableModeCandidateConfig {
            max_log: config.of_max_log,
            repeat_table_max_sequences: config.repeat_table_max_sequences,
            predefined_max_sequences: config.of_predefined_max_sequences,
            exact_sequence_mode_search: config.exact_sequence_mode_search,
        },
    );

    if !config.exact_sequence_mode_search {
        return (
            ll_candidates.into_iter().next().unwrap(),
            ml_candidates.into_iter().next().unwrap(),
            of_candidates.into_iter().next().unwrap(),
        );
    }

    let mut ll_candidates = ll_candidates;
    let mut ml_candidates = ml_candidates;
    let mut of_candidates = of_candidates;
    let mut best_ll = 0usize;
    let mut best_ml = 0usize;
    let mut best_of = 0usize;
    let mut best_size = exact_sequence_section_size(
        sequences,
        &ll_candidates[0],
        &ml_candidates[0],
        &of_candidates[0],
    );

    for (ll_idx, ll_mode) in ll_candidates.iter().enumerate() {
        for (ml_idx, ml_mode) in ml_candidates.iter().enumerate() {
            for (of_idx, of_mode) in of_candidates.iter().enumerate() {
                let size = exact_sequence_section_size(sequences, ll_mode, ml_mode, of_mode);
                if size < best_size {
                    best_ll = ll_idx;
                    best_ml = ml_idx;
                    best_of = of_idx;
                    best_size = size;
                }
            }
        }
    }

    (
        ll_candidates.swap_remove(best_ll),
        ml_candidates.swap_remove(best_ml),
        of_candidates.swap_remove(best_of),
    )
}

fn exact_sequence_section_size(
    sequences: &[crate::blocks::sequence_section::Sequence],
    ll_mode: &FseTableMode<'_>,
    ml_mode: &FseTableMode<'_>,
    of_mode: &FseTableMode<'_>,
) -> usize {
    let mut encoded = Vec::new();
    let mut writer = BitWriter::from(&mut encoded);
    writer.write_bits(encode_fse_table_modes(ll_mode, ml_mode, of_mode), 8);
    encode_table(ll_mode, &mut writer);
    encode_table(of_mode, &mut writer);
    encode_table(ml_mode, &mut writer);
    encode_sequences(sequences, &mut writer, ll_mode, ml_mode, of_mode);
    writer.flush();
    encoded.len()
}

fn encode_table(mode: &FseTableMode<'_>, writer: &mut BitWriter<&mut Vec<u8>>) {
    match mode {
        FseTableMode::Predefined(_) => {}
        FseTableMode::Rle(symbol) => writer.write_bits(*symbol, 8),
        FseTableMode::RepeateLast(_) => {}
        FseTableMode::Encoded(table) => table.write_table(writer),
    }
}

fn encode_fse_table_modes(
    ll_mode: &FseTableMode<'_>,
    ml_mode: &FseTableMode<'_>,
    of_mode: &FseTableMode<'_>,
) -> u8 {
    fn mode_to_bits(mode: &FseTableMode<'_>) -> u8 {
        match mode {
            FseTableMode::Predefined(_) => 0,
            FseTableMode::Rle(_) => 1,
            FseTableMode::Encoded(_) => 2,
            FseTableMode::RepeateLast(_) => 3,
        }
    }
    mode_to_bits(ll_mode) << 6 | mode_to_bits(of_mode) << 4 | mode_to_bits(ml_mode) << 2
}

fn should_compress_literals(len: usize, has_previous_table: bool) -> bool {
    let min_size = if has_previous_table {
        REPEAT_LITERALS_SIZE_MIN
    } else {
        COMPRESS_LITERALS_SIZE_MIN
    };
    len > min_size
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

#[inline(always)]
pub(crate) fn literal_length_code(len: u32) -> u8 {
    encode_literal_length(len).0
}

#[inline(always)]
pub(crate) fn match_length_code(len: u32) -> u8 {
    encode_match_len(len).0
}

#[inline(always)]
pub(crate) fn offset_code(offset_value: u32) -> u8 {
    encode_offset(offset_value).0
}

#[inline(always)]
fn encode_literal_length(len: u32) -> (u8, u32, usize) {
    if len < LITERAL_LENGTH_SMALL_CODES.len() as u32 {
        return LITERAL_LENGTH_SMALL_CODES[len as usize];
    }

    match len {
        0..=63 => unreachable!(),
        64..=127 => (25, len - 64, 6),
        128..=255 => (26, len - 128, 7),
        256..=511 => (27, len - 256, 8),
        512..=1023 => (28, len - 512, 9),
        1024..=2047 => (29, len - 1024, 10),
        2048..=4095 => (30, len - 2048, 11),
        4096..=8191 => (31, len - 4096, 12),
        8192..=16383 => (32, len - 8192, 13),
        16384..=32767 => (33, len - 16384, 14),
        32768..=65535 => (34, len - 32768, 15),
        65536..=131071 => (35, len - 65536, 16),
        131072.. => unreachable!(),
    }
}

#[inline(always)]
fn encode_match_len(len: u32) -> (u8, u32, usize) {
    if (3..=130).contains(&len) {
        return MATCH_LENGTH_SMALL_CODES[(len - 3) as usize];
    }

    match len {
        0..=2 => unreachable!(),
        3..=130 => unreachable!(),
        131..=258 => (43, len - 131, 7),
        259..=514 => (44, len - 259, 8),
        515..=1026 => (45, len - 515, 9),
        1027..=2050 => (46, len - 1027, 10),
        2051..=4098 => (47, len - 2051, 11),
        4099..=8194 => (48, len - 4099, 12),
        8195..=16386 => (49, len - 8195, 13),
        16387..=32770 => (50, len - 16387, 14),
        32771..=65538 => (51, len - 32771, 15),
        65539..=131074 => (52, len - 65539, 16),
        131075.. => unreachable!(),
    }
}

const fn small_literal_length_codes() -> [(u8, u32, usize); 64] {
    let mut codes = [(0, 0, 0); 64];
    let mut len = 0usize;
    while len < codes.len() {
        codes[len] = match len {
            0..=15 => (len as u8, 0, 0),
            16..=17 => (16, len as u32 - 16, 1),
            18..=19 => (17, len as u32 - 18, 1),
            20..=21 => (18, len as u32 - 20, 1),
            22..=23 => (19, len as u32 - 22, 1),
            24..=27 => (20, len as u32 - 24, 2),
            28..=31 => (21, len as u32 - 28, 2),
            32..=39 => (22, len as u32 - 32, 3),
            40..=47 => (23, len as u32 - 40, 3),
            48..=63 => (24, len as u32 - 48, 4),
            _ => unreachable!(),
        };
        len += 1;
    }
    codes
}

const fn small_match_length_codes() -> [(u8, u32, usize); 128] {
    let mut codes = [(0, 0, 0); 128];
    let mut idx = 0usize;
    while idx < codes.len() {
        let len = idx + 3;
        codes[idx] = match len {
            3..=34 => (len as u8 - 3, 0, 0),
            35..=36 => (32, len as u32 - 35, 1),
            37..=38 => (33, len as u32 - 37, 1),
            39..=40 => (34, len as u32 - 39, 1),
            41..=42 => (35, len as u32 - 41, 1),
            43..=46 => (36, len as u32 - 43, 2),
            47..=50 => (37, len as u32 - 47, 2),
            51..=58 => (38, len as u32 - 51, 3),
            59..=66 => (39, len as u32 - 59, 3),
            67..=82 => (40, len as u32 - 67, 4),
            83..=98 => (41, len as u32 - 83, 4),
            99..=130 => (42, len as u32 - 99, 5),
            _ => unreachable!(),
        };
        idx += 1;
    }
    codes
}

fn encode_offset(len: u32) -> (u8, u32, usize) {
    let log = len.ilog2();
    let lower = len & ((1 << log) - 1);
    (log as u8, lower, log as usize)
}

fn raw_literals(literals: &[u8], writer: &mut BitWriter<&mut Vec<u8>>) {
    writer.write_bits(0u8, 2); // Raw_Literals_Block
    match literals.len() {
        0..=31 => {
            writer.write_bits(0u8, 1);
            writer.write_bits(literals.len() as u32, 5);
        }
        32..=4095 => {
            writer.write_bits(0b01u8, 2);
            writer.write_bits(literals.len() as u32, 12);
        }
        4096..=1_048_575 => {
            writer.write_bits(0b11u8, 2);
            writer.write_bits(literals.len() as u32, 20);
        }
        _ => unimplemented!("too many literals"),
    }
    writer.append_bytes(literals);
}

fn rle_literals(literals: &[u8], writer: &mut BitWriter<&mut Vec<u8>>) {
    debug_assert!(!literals.is_empty());
    writer.write_bits(1u8, 2); // RLE_Literals_Block
    match literals.len() {
        0..=31 => {
            writer.write_bits(0u8, 1);
            writer.write_bits(literals.len() as u32, 5);
        }
        32..=4095 => {
            writer.write_bits(0b01u8, 2);
            writer.write_bits(literals.len() as u32, 12);
        }
        4096..=1_048_575 => {
            writer.write_bits(0b11u8, 2);
            writer.write_bits(literals.len() as u32, 20);
        }
        _ => unimplemented!("too many literals"),
    }
    writer.write_bits(literals[0], 8);
}

fn compress_literals(
    literals: &[u8],
    last_table: Option<&huff0_encoder::HuffmanTable>,
    search_smallest_table: bool,
    force_single_stream_max_literals: Option<usize>,
    writer: &mut BitWriter<&mut Vec<u8>>,
) -> Option<huff0_encoder::HuffmanTable> {
    let reset_idx = writer.index();

    let literal_stats = LiteralStats::from_literals(literals);
    if literal_stats.largest == literals.len()
        || literal_stats.likely_incompressible(literals.len())
    {
        if !literals.is_empty() && literal_stats.largest == literals.len() {
            rle_literals(literals, writer);
        } else {
            raw_literals(literals, writer);
        }
        return None;
    }

    let force_single_stream =
        force_single_stream_max_literals.is_some_and(|max_literals| literals.len() <= max_literals);
    let (size_format, size_bits) =
        compressed_literals_size_format(literals.len(), force_single_stream);
    let four_streams = size_format != 0;
    let header_len = compressed_literals_header_len(size_format);
    let new_encoder_table = if search_smallest_table {
        huff0_encoder::HuffmanTable::build_smallest_from_counts(
            literal_stats.counts(),
            literals,
            four_streams,
        )
    } else {
        huff0_encoder::HuffmanTable::build_from_counts(literal_stats.counts())
    };
    let new_len = new_encoder_table.encoded_len(literals, true, four_streams);
    let new_choice = LiteralEncodingChoice {
        encoder_table: &new_encoder_table,
        new_table: true,
        estimated_len: new_len,
        size_format,
        size_bits,
        header_len,
    };
    let choice = last_table
        .and_then(|previous_table| {
            repeat_huffman_choice(
                previous_table,
                &literal_stats,
                literals,
                new_choice,
                force_single_stream,
            )
        })
        .unwrap_or(new_choice);

    if !literal_estimate_has_enough_gain(choice.estimated_len, choice.header_len, literals.len()) {
        raw_literals(literals, writer);
        return None;
    }

    write_compressed_literals(
        literals,
        choice.encoder_table,
        choice.new_table,
        choice.size_format,
        choice.size_bits,
        writer,
    );
    let total_len = (writer.index() - reset_idx) / 8;

    // If encoded len is bigger than the raw literals we are better off just writing the raw literals here
    if total_len >= literals.len() {
        writer.reset_to(reset_idx);
        raw_literals(literals, writer);
        None
    } else if choice.new_table {
        Some(new_encoder_table)
    } else {
        None
    }
}

#[derive(Clone, Copy)]
struct LiteralEncodingChoice<'table> {
    encoder_table: &'table huff0_encoder::HuffmanTable,
    new_table: bool,
    estimated_len: usize,
    size_format: u8,
    size_bits: usize,
    header_len: usize,
}

impl LiteralEncodingChoice<'_> {
    fn total_estimated_len(self) -> usize {
        self.estimated_len + self.header_len
    }
}

fn repeat_huffman_choice<'table>(
    previous_table: &'table huff0_encoder::HuffmanTable,
    literal_stats: &LiteralStats,
    literals: &[u8],
    new_choice: LiteralEncodingChoice<'_>,
    force_single_stream: bool,
) -> Option<LiteralEncodingChoice<'table>> {
    if !previous_table.can_encode_counts(literal_stats.counts()) {
        return None;
    }

    let (size_format, size_bits) =
        compressed_literals_repeat_size_format(literals.len(), force_single_stream);
    let header_len = compressed_literals_header_len(size_format);
    let four_streams = size_format != 0;
    let estimated_len = previous_table.encoded_len(literals, false, four_streams);
    if estimated_len < literals.len()
        && estimated_len + header_len <= new_choice.total_estimated_len()
    {
        Some(LiteralEncodingChoice {
            encoder_table: previous_table,
            new_table: false,
            estimated_len,
            size_format,
            size_bits,
            header_len,
        })
    } else {
        None
    }
}

fn compressed_literals_size_format(len: usize, force_single_stream: bool) -> (u8, usize) {
    if force_single_stream && len < HUFFMAN_4_STREAMS_MIN * 4 {
        return (0b00u8, 10);
    }

    match len {
        0..HUFFMAN_4_STREAMS_MIN => (0b00u8, 10),
        HUFFMAN_4_STREAMS_MIN..1024 => (0b01, 10),
        1024..16384 => (0b10, 14),
        16384..262144 => (0b11, 18),
        _ => unimplemented!("too many literals"),
    }
}

fn compressed_literals_repeat_size_format(len: usize, force_single_stream: bool) -> (u8, usize) {
    if force_single_stream || len < REPEAT_SINGLE_STREAM_LITERALS_MAX {
        return (0b00, 10);
    }

    compressed_literals_size_format(len, false)
}

fn compressed_literals_header_len(size_format: u8) -> usize {
    match size_format {
        0b00 | 0b01 => 3,
        0b10 => 4,
        0b11 => 5,
        _ => unreachable!(),
    }
}

fn literal_min_gain(len: usize) -> usize {
    (len >> FAST_LITERAL_MIN_GAIN_LOG) + 2
}

fn literal_estimate_has_enough_gain(
    estimated_len: usize,
    header_len: usize,
    literal_len: usize,
) -> bool {
    estimated_len < literal_len.saturating_sub(literal_min_gain(literal_len))
        && estimated_len + header_len < literal_len
}

fn write_compressed_literals(
    literals: &[u8],
    encoder_table: &huff0_encoder::HuffmanTable,
    new_table: bool,
    size_format: u8,
    size_bits: usize,
    writer: &mut BitWriter<&mut Vec<u8>>,
) {
    if new_table {
        writer.write_bits(2u8, 2); // compressed literals type
    } else {
        writer.write_bits(3u8, 2); // treeless compressed literals type
    }

    writer.write_bits(size_format, 2);
    writer.write_bits(literals.len() as u32, size_bits);
    let size_index = writer.index();
    writer.write_bits(0u32, size_bits);
    let index_before = writer.index();
    let mut encoder = huff0_encoder::HuffmanEncoder::new(encoder_table, writer);
    if size_format == 0 {
        encoder.encode(literals, new_table)
    } else {
        encoder.encode4x(literals, new_table)
    };
    let encoded_len = (writer.index() - index_before) / 8;
    writer.change_bits(size_index, encoded_len as u64, size_bits);
}

struct LiteralStats {
    counts: [usize; 256],
    max_symbol: usize,
    largest: usize,
}

impl LiteralStats {
    fn from_literals(literals: &[u8]) -> Self {
        let mut counts = [0; 256];
        let mut max_symbol = 0usize;
        let mut largest = 0usize;
        for literal in literals {
            let symbol = *literal as usize;
            counts[symbol] += 1;
            largest = largest.max(counts[symbol]);
            max_symbol = max_symbol.max(symbol);
        }
        Self {
            counts,
            max_symbol,
            largest,
        }
    }

    fn counts(&self) -> &[usize] {
        &self.counts[..=self.max_symbol]
    }

    fn likely_incompressible(&self, len: usize) -> bool {
        self.largest <= (len >> 7) + 4
    }
}
