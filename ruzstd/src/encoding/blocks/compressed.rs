use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryFrom;
#[cfg(feature = "std")]
use std::sync::OnceLock;

use crate::{
    bit_io::BitWriter,
    encoding::frame_compressor::{CompressState, FseTables, OffsetHistory},
    encoding::util::likely_dependency_json_lockfile_text,
    encoding::{CompressionFileProfile, CompressionFileType, CompressionLevel, Matcher, Sequence},
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
const FILE_TYPE_SMALL_SEQUENCE_PREDEFINED_LLML_MAX_SEQUENCES: usize = 64;
const FILE_TYPE_SINGLE_STREAM_HUFFMAN_MAX_LITERALS: usize = 1024;
const EXACT_SEQUENCE_TABLE_MIN_LOG: u8 = 7;
const LITERAL_LENGTH_SMALL_CODES: [(u8, u32, usize); 64] = small_literal_length_codes();
const MATCH_LENGTH_SMALL_CODES: [(u8, u32, usize); 128] = small_match_length_codes();

#[derive(Clone, Copy)]
pub(crate) struct BlockCompressionConfig {
    huffman_table_search: HuffmanTableSearch,
    repeat_table_max_sequences: usize,
    offset_table_max_log: u8,
    offset_predefined_max_sequences: usize,
    exact_sequence_mode_search: bool,
    file_type_small_sequence_predefined_llml_max_sequences: Option<usize>,
    file_type_single_stream_huffman_max_literals: Option<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HuffmanTableSearch {
    Heuristic,
    FileTypeSmall,
    AllSections,
}

#[cfg(feature = "std")]
#[derive(Clone, Copy, Debug, Default)]
struct BlockCompressionTuningOverrides {
    huffman_table_search: Option<HuffmanTableSearch>,
    repeat_table_max_sequences: Option<usize>,
    offset_table_max_log: Option<u8>,
    offset_predefined_max_sequences: Option<usize>,
    exact_sequence_mode_search: Option<bool>,
    file_type_small_sequence_predefined_llml_max_sequences: Option<Option<usize>>,
    file_type_single_stream_huffman_max_literals: Option<Option<usize>>,
}

#[cfg(feature = "std")]
static BLOCK_COMPRESSION_TUNING_OVERRIDES: OnceLock<BlockCompressionTuningOverrides> =
    OnceLock::new();

#[cfg(feature = "std")]
fn block_compression_tuning_overrides() -> &'static BlockCompressionTuningOverrides {
    BLOCK_COMPRESSION_TUNING_OVERRIDES.get_or_init(BlockCompressionTuningOverrides::from_env)
}

#[cfg(feature = "std")]
impl BlockCompressionTuningOverrides {
    fn from_env() -> Self {
        Self {
            huffman_table_search: std::env::var("RUZSTD_TUNE_HUFFMAN_TABLE_SEARCH")
                .ok()
                .and_then(|value| match value.as_str() {
                    "heuristic" => Some(HuffmanTableSearch::Heuristic),
                    "filetype" => Some(HuffmanTableSearch::FileTypeSmall),
                    "allsections" => Some(HuffmanTableSearch::AllSections),
                    _ => None,
                }),
            repeat_table_max_sequences: Self::parse_usize("RUZSTD_TUNE_REPEAT_TABLE_MAX_SEQUENCES"),
            offset_table_max_log: Self::parse_usize("RUZSTD_TUNE_OFFSET_TABLE_MAX_LOG")
                .and_then(|value| u8::try_from(value).ok()),
            offset_predefined_max_sequences: Self::parse_usize(
                "RUZSTD_TUNE_OFFSET_PREDEFINED_MAX_SEQUENCES",
            ),
            exact_sequence_mode_search: std::env::var("RUZSTD_TUNE_EXACT_SEQUENCE_MODE_SEARCH")
                .ok()
                .and_then(|value| Self::parse_bool_value(&value))
                .or_else(|| {
                    std::env::var("RUZSTD_TUNE_EXACT_OFFSET_MODE_SEARCH")
                        .ok()
                        .and_then(|value| Self::parse_bool_value(&value))
                }),
            file_type_small_sequence_predefined_llml_max_sequences: Self::parse_option_usize(
                "RUZSTD_TUNE_FILE_TYPE_SMALL_SEQUENCE_PREDEFINED_LLML_MAX_SEQUENCES",
            ),
            file_type_single_stream_huffman_max_literals: Self::parse_option_usize(
                "RUZSTD_TUNE_FILE_TYPE_SINGLE_STREAM_HUFFMAN_MAX_LITERALS",
            ),
        }
    }

    fn parse_usize(name: &str) -> Option<usize> {
        std::env::var(name).ok()?.parse().ok()
    }

    fn parse_option_usize(name: &str) -> Option<Option<usize>> {
        let value = std::env::var(name).ok()?;
        if value == "none" {
            Some(None)
        } else {
            value.parse().ok().map(Some)
        }
    }

    fn parse_bool_value(value: &str) -> Option<bool> {
        match value {
            "1" | "true" | "TRUE" | "yes" | "YES" | "on" | "ON" => Some(true),
            "0" | "false" | "FALSE" | "no" | "NO" | "off" | "OFF" => Some(false),
            _ => None,
        }
    }
}

impl BlockCompressionConfig {
    pub(crate) fn for_level(level: CompressionLevel) -> Self {
        Self::for_level_and_file_type(level, CompressionFileType::Unknown)
    }

    pub(crate) fn for_level_and_file_type(
        level: CompressionLevel,
        file_type: CompressionFileType,
    ) -> Self {
        Self::for_level_and_hints(level, file_type, CompressionFileProfile::None)
    }

    pub(crate) fn for_level_and_hints(
        level: CompressionLevel,
        file_type: CompressionFileType,
        file_profile: CompressionFileProfile,
    ) -> Self {
        let huffman_table_search = match level {
            CompressionLevel::Best => HuffmanTableSearch::Heuristic,
            CompressionLevel::Uncompressed
            | CompressionLevel::Fastest
            | CompressionLevel::Default
            | CompressionLevel::Better => {
                if matches!(file_type, CompressionFileType::DictionaryText) {
                    HuffmanTableSearch::AllSections
                } else if matches!(
                    file_type,
                    CompressionFileType::CodeText
                        | CompressionFileType::ConfigText
                        | CompressionFileType::Unknown
                ) {
                    HuffmanTableSearch::FileTypeSmall
                } else {
                    HuffmanTableSearch::Heuristic
                }
            }
        };
        let repeat_table_max_sequences = match level {
            CompressionLevel::Best => 256,
            CompressionLevel::Uncompressed
            | CompressionLevel::Fastest
            | CompressionLevel::Default
            | CompressionLevel::Better => 64,
        };
        let mut config = Self {
            huffman_table_search,
            repeat_table_max_sequences,
            offset_table_max_log: if matches!(file_type, CompressionFileType::DictionaryText)
                || (matches!(file_type, CompressionFileType::Unknown)
                    && matches!(level, CompressionLevel::Fastest))
            {
                7
            } else {
                8
            },
            offset_predefined_max_sequences: 16,
            exact_sequence_mode_search: matches!(level, CompressionLevel::Fastest)
                && matches!(file_type, CompressionFileType::DictionaryText),
            file_type_small_sequence_predefined_llml_max_sequences: if matches!(
                level,
                CompressionLevel::Fastest
            ) && matches!(
                file_type,
                CompressionFileType::Unknown | CompressionFileType::ConfigText
            ) {
                Some(FILE_TYPE_SMALL_SEQUENCE_PREDEFINED_LLML_MAX_SEQUENCES)
            } else {
                None
            },
            file_type_single_stream_huffman_max_literals: if matches!(
                level,
                CompressionLevel::Fastest
            ) && matches!(
                file_type,
                CompressionFileType::ConfigText
            ) {
                Some(FILE_TYPE_SINGLE_STREAM_HUFFMAN_MAX_LITERALS)
            } else {
                None
            },
        };
        #[cfg(feature = "std")]
        config.apply_tuning_overrides();
        if matches!(file_profile, CompressionFileProfile::SmallTextLockfile) {
            config.apply_small_text_lockfile_tuning();
        } else if matches!(file_profile, CompressionFileProfile::DependencyJsonLockfile) {
            config.apply_dependency_json_lockfile_tuning();
        }
        config
    }

    #[cfg(feature = "std")]
    fn apply_tuning_overrides(&mut self) {
        let overrides = block_compression_tuning_overrides();
        if let Some(value) = overrides.huffman_table_search {
            self.huffman_table_search = value;
        }
        if let Some(value) = overrides.repeat_table_max_sequences {
            self.repeat_table_max_sequences = value;
        }
        if let Some(value) = overrides.offset_table_max_log {
            self.offset_table_max_log = value;
        }
        if let Some(value) = overrides.offset_predefined_max_sequences {
            self.offset_predefined_max_sequences = value;
        }
        if let Some(value) = overrides.exact_sequence_mode_search {
            self.exact_sequence_mode_search = value;
        }
        if let Some(value) = overrides.file_type_small_sequence_predefined_llml_max_sequences {
            self.file_type_small_sequence_predefined_llml_max_sequences = value;
        }
        if let Some(value) = overrides.file_type_single_stream_huffman_max_literals {
            self.file_type_single_stream_huffman_max_literals = value;
        }
    }

    fn apply_dependency_json_lockfile_tuning(&mut self) {
        self.huffman_table_search = HuffmanTableSearch::AllSections;
        self.repeat_table_max_sequences = 256;
        self.offset_table_max_log = 8;
        self.exact_sequence_mode_search = true;
    }

    fn apply_small_text_lockfile_tuning(&mut self) {
        self.huffman_table_search = HuffmanTableSearch::AllSections;
        self.repeat_table_max_sequences = 256;
        self.offset_table_max_log = 7;
        self.offset_predefined_max_sequences = 64;
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoding::frame_compressor::{CompressState, FseTables, OffsetHistory};
    use crate::fse::fse_encoder::{default_ll_table, default_ml_table, default_of_table};

    fn offset_history(newest: u32, second: u32, third: u32) -> OffsetHistory {
        OffsetHistory {
            newest,
            second,
            third,
        }
    }

    struct LiteralPayloadMatcher {
        literals: Vec<u8>,
        emitted: bool,
    }

    impl Matcher for LiteralPayloadMatcher {
        fn get_next_space(&mut self) -> Vec<u8> {
            Vec::new()
        }

        fn get_last_space(&self) -> &[u8] {
            &[]
        }

        fn commit_space(&mut self, _space: Vec<u8>) {}

        fn skip_matching(&mut self) {}

        fn start_matching(&mut self, mut handle_sequence: impl for<'a> FnMut(Sequence<'a>)) {
            if !self.emitted {
                self.emitted = true;
                handle_sequence(Sequence::Triple {
                    literals: &self.literals,
                    offset: 1,
                    match_len: 16,
                });
            }
        }

        fn reset(&mut self, _level: crate::encoding::CompressionLevel) {
            self.emitted = false;
        }

        fn window_size(&self) -> u64 {
            128 * 1024
        }
    }

    fn compressed_frame_with_literal_payload(literals: Vec<u8>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        compressed_frame_with_literal_payload_and_last_table(literals, None)
    }

    fn compressed_frame_with_literal_payload_and_last_table(
        literals: Vec<u8>,
        last_huff_table: Option<huff0_encoder::HuffmanTable>,
    ) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        compressed_frame_with_literal_payload_and_config(
            literals,
            last_huff_table,
            BlockCompressionConfig::for_level(CompressionLevel::Fastest),
        )
    }

    fn compressed_frame_with_literal_payload_and_config(
        literals: Vec<u8>,
        last_huff_table: Option<huff0_encoder::HuffmanTable>,
        config: BlockCompressionConfig,
    ) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        assert!(!literals.is_empty());

        let mut state = CompressState {
            matcher: LiteralPayloadMatcher {
                literals: literals.clone(),
                emitted: false,
            },
            last_huff_table,
            fse_tables: FseTables::new(),
            offset_history: OffsetHistory::new(),
            file_type_hint: CompressionFileType::Unknown,
            file_profile_hint: CompressionFileProfile::None,
        };
        let mut block_payload = Vec::new();

        compress_block_with_config(&mut state, &mut block_payload, config);

        let mut frame = Vec::new();
        crate::encoding::frame_header::FrameHeader {
            frame_content_size: None,
            single_segment: false,
            content_checksum: false,
            dictionary_id: None,
            window_size: Some(128 * 1024),
        }
        .serialize(&mut frame);
        crate::encoding::block_header::BlockHeader {
            last_block: true,
            block_type: crate::blocks::block::BlockType::Compressed,
            block_size: block_payload.len() as u32,
        }
        .serialize(&mut frame);
        frame.extend_from_slice(&block_payload);

        let last_literal = literals[literals.len() - 1];
        let mut expected = literals;
        expected.extend_from_slice(&[last_literal; 16]);

        (block_payload, frame, expected)
    }

    fn literal_length_code_from_spec(len: u32) -> (u8, u32, usize) {
        match len {
            0..=15 => (len as u8, 0, 0),
            16..=17 => (16, len - 16, 1),
            18..=19 => (17, len - 18, 1),
            20..=21 => (18, len - 20, 1),
            22..=23 => (19, len - 22, 1),
            24..=27 => (20, len - 24, 2),
            28..=31 => (21, len - 28, 2),
            32..=39 => (22, len - 32, 3),
            40..=47 => (23, len - 40, 3),
            48..=63 => (24, len - 48, 4),
            64..=127 => (25, len - 64, 6),
            _ => panic!("test helper only covers literal lengths through code 25"),
        }
    }

    fn match_length_code_from_spec(len: u32) -> (u8, u32, usize) {
        match len {
            0..=2 => panic!("match lengths below 3 are invalid"),
            3..=34 => (len as u8 - 3, 0, 0),
            35..=36 => (32, len - 35, 1),
            37..=38 => (33, len - 37, 1),
            39..=40 => (34, len - 39, 1),
            41..=42 => (35, len - 41, 1),
            43..=46 => (36, len - 43, 2),
            47..=50 => (37, len - 47, 2),
            51..=58 => (38, len - 51, 3),
            59..=66 => (39, len - 59, 3),
            67..=82 => (40, len - 67, 4),
            83..=98 => (41, len - 83, 4),
            99..=130 => (42, len - 99, 5),
            131..=258 => (43, len - 131, 7),
            _ => panic!("test helper only covers match lengths through code 43"),
        }
    }

    fn offset_code_from_spec(len: u32) -> (u8, u32, usize) {
        let code = len.ilog2();
        let additional = len - (1 << code);
        (code as u8, additional, code as usize)
    }

    #[test]
    fn offset_history_uses_repeat_offsets_when_literals_are_present() {
        let mut history = OffsetHistory::new();

        assert_eq!(history.encode_offset_value(4, 3), 2);
        assert_eq!(history, offset_history(4, 1, 8));

        assert_eq!(history.encode_offset_value(4, 1), 1);
        assert_eq!(history, offset_history(4, 1, 8));

        assert_eq!(history.encode_offset_value(8, 2), 3);
        assert_eq!(history, offset_history(8, 4, 1));
    }

    #[test]
    fn offset_history_uses_shifted_repeat_offsets_for_zero_literals() {
        let mut history = offset_history(5, 9, 13);

        assert_eq!(history.encode_offset_value(9, 0), 1);
        assert_eq!(history, offset_history(9, 5, 13));

        let mut history = offset_history(5, 9, 13);
        assert_eq!(history.encode_offset_value(13, 0), 2);
        assert_eq!(history, offset_history(13, 5, 9));

        let mut history = offset_history(5, 9, 13);
        assert_eq!(history.encode_offset_value(4, 0), 3);
        assert_eq!(history, offset_history(4, 5, 9));
    }

    #[test]
    fn offset_history_encodes_new_offsets_and_updates_history() {
        let mut history = OffsetHistory::new();

        assert_eq!(history.encode_offset_value(10, 1), 13);
        assert_eq!(history, offset_history(10, 1, 4));
    }

    #[test]
    fn choose_table_uses_predefined_tables_for_tiny_sequence_counts() {
        let default = default_ll_table();
        let sequences = [crate::blocks::sequence_section::Sequence {
            ll: 0,
            ml: 3,
            of: 1,
        }];

        assert!(matches!(
            choose_table(
                None,
                &default,
                &sequences,
                |seq| encode_literal_length(seq.ll).0,
                9,
                64,
                16,
            ),
            FseTableMode::Predefined(_)
        ));
    }

    #[test]
    fn choose_table_uses_predefined_tables_for_small_non_rle_blocks() {
        let default = default_ll_table();
        let sequences = [
            crate::blocks::sequence_section::Sequence {
                ll: 0,
                ml: 3,
                of: 1,
            },
            crate::blocks::sequence_section::Sequence {
                ll: 2,
                ml: 3,
                of: 1,
            },
            crate::blocks::sequence_section::Sequence {
                ll: 4,
                ml: 3,
                of: 1,
            },
        ];

        assert!(matches!(
            choose_table(
                None,
                &default,
                &sequences,
                |seq| encode_literal_length(seq.ll).0,
                9,
                64,
                16,
            ),
            FseTableMode::Predefined(_)
        ));
    }

    #[test]
    fn choose_table_uses_rle_for_repeated_codes() {
        let default = default_ll_table();
        let sequences = [crate::blocks::sequence_section::Sequence {
            ll: 5,
            ml: 8,
            of: 1,
        }; 3];

        assert!(matches!(
            choose_table(
                None,
                &default,
                &sequences,
                |seq| encode_literal_length(seq.ll).0,
                9,
                64,
                16,
            ),
            FseTableMode::Rle(5)
        ));
    }

    #[test]
    fn previous_huffman_table_lowers_literal_compression_threshold() {
        assert!(!should_compress_literals(COMPRESS_LITERALS_SIZE_MIN, false));
        assert!(should_compress_literals(
            COMPRESS_LITERALS_SIZE_MIN + 1,
            false
        ));

        assert!(!should_compress_literals(REPEAT_LITERALS_SIZE_MIN, true));
        assert!(should_compress_literals(REPEAT_LITERALS_SIZE_MIN + 1, true));
    }

    #[test]
    fn rle_sequence_modes_round_trip_through_decoder() {
        let sequences = [
            crate::blocks::sequence_section::Sequence {
                ll: 5,
                ml: 8,
                of: 1,
            },
            crate::blocks::sequence_section::Sequence {
                ll: 5,
                ml: 8,
                of: 1,
            },
            crate::blocks::sequence_section::Sequence {
                ll: 5,
                ml: 8,
                of: 1,
            },
        ];
        let ll_mode = FseTableMode::Rle(encode_literal_length(sequences[0].ll).0);
        let ml_mode = FseTableMode::Rle(encode_match_len(sequences[0].ml).0);
        let of_mode = FseTableMode::Rle(encode_offset(sequences[0].of).0);
        let mut encoded = Vec::new();
        let mut writer = BitWriter::from(&mut encoded);

        encode_seqnum(sequences.len(), &mut writer);
        writer.write_bits(encode_fse_table_modes(&ll_mode, &ml_mode, &of_mode), 8);
        encode_table(&ll_mode, &mut writer);
        encode_table(&of_mode, &mut writer);
        encode_table(&ml_mode, &mut writer);
        encode_sequences(&sequences, &mut writer, &ll_mode, &ml_mode, &of_mode);
        writer.flush();

        let mut header = crate::blocks::sequence_section::SequencesHeader::new();
        let header_size = header.parse_from_header(&encoded).unwrap();
        let mut scratch = crate::decoding::scratch::FSEScratch::new();
        let mut decoded = Vec::new();

        crate::decoding::sequence_section_decoder::decode_sequences(
            &header,
            &encoded[header_size as usize..],
            &mut scratch,
            &mut decoded,
        )
        .unwrap();

        assert_eq!(decoded.len(), sequences.len());
        for (actual, expected) in decoded.iter().zip(sequences) {
            assert_eq!(actual.ll, expected.ll);
            assert_eq!(actual.ml, expected.ml);
            assert_eq!(actual.of, expected.of);
        }
    }

    #[test]
    fn all_rle_sequence_modes_preserve_additional_bits() {
        let sequences = [
            crate::blocks::sequence_section::Sequence {
                ll: 16,
                ml: 35,
                of: 4,
            },
            crate::blocks::sequence_section::Sequence {
                ll: 17,
                ml: 36,
                of: 5,
            },
            crate::blocks::sequence_section::Sequence {
                ll: 16,
                ml: 35,
                of: 6,
            },
        ];
        let ll_mode = FseTableMode::Rle(encode_literal_length(sequences[0].ll).0);
        let ml_mode = FseTableMode::Rle(encode_match_len(sequences[0].ml).0);
        let of_mode = FseTableMode::Rle(encode_offset(sequences[0].of).0);
        let mut encoded = Vec::new();
        let mut writer = BitWriter::from(&mut encoded);

        encode_seqnum(sequences.len(), &mut writer);
        writer.write_bits(encode_fse_table_modes(&ll_mode, &ml_mode, &of_mode), 8);
        encode_table(&ll_mode, &mut writer);
        encode_table(&of_mode, &mut writer);
        encode_table(&ml_mode, &mut writer);
        encode_sequences(&sequences, &mut writer, &ll_mode, &ml_mode, &of_mode);
        writer.flush();

        let mut header = crate::blocks::sequence_section::SequencesHeader::new();
        let header_size = header.parse_from_header(&encoded).unwrap();
        let mut scratch = crate::decoding::scratch::FSEScratch::new();
        let mut decoded = Vec::new();

        crate::decoding::sequence_section_decoder::decode_sequences(
            &header,
            &encoded[header_size as usize..],
            &mut scratch,
            &mut decoded,
        )
        .unwrap();

        assert_eq!(decoded.len(), sequences.len());
        for (actual, expected) in decoded.iter().zip(sequences) {
            assert_eq!(actual.ll, expected.ll);
            assert_eq!(actual.ml, expected.ml);
            assert_eq!(actual.of, expected.of);
        }
    }

    #[test]
    fn mixed_predefined_sequence_modes_round_trip_through_decoder() {
        let sequences = [
            crate::blocks::sequence_section::Sequence {
                ll: 0,
                ml: 3,
                of: 1,
            },
            crate::blocks::sequence_section::Sequence {
                ll: 1,
                ml: 4,
                of: 2,
            },
            crate::blocks::sequence_section::Sequence {
                ll: 4,
                ml: 8,
                of: 4,
            },
            crate::blocks::sequence_section::Sequence {
                ll: 12,
                ml: 16,
                of: 8,
            },
        ];
        let ll_default = default_ll_table();
        let ml_default = default_ml_table();
        let of_default = default_of_table();
        let ll_mode = FseTableMode::Predefined(&ll_default);
        let ml_mode = FseTableMode::Predefined(&ml_default);
        let of_mode = FseTableMode::Predefined(&of_default);
        let mut encoded = Vec::new();
        let mut writer = BitWriter::from(&mut encoded);

        encode_seqnum(sequences.len(), &mut writer);
        writer.write_bits(encode_fse_table_modes(&ll_mode, &ml_mode, &of_mode), 8);
        encode_table(&ll_mode, &mut writer);
        encode_table(&of_mode, &mut writer);
        encode_table(&ml_mode, &mut writer);
        encode_sequences(&sequences, &mut writer, &ll_mode, &ml_mode, &of_mode);
        writer.flush();

        let mut header = crate::blocks::sequence_section::SequencesHeader::new();
        let header_size = header.parse_from_header(&encoded).unwrap();
        let mut scratch = crate::decoding::scratch::FSEScratch::new();
        let mut decoded = Vec::new();

        crate::decoding::sequence_section_decoder::decode_sequences(
            &header,
            &encoded[header_size as usize..],
            &mut scratch,
            &mut decoded,
        )
        .unwrap();

        assert_eq!(decoded.len(), sequences.len());
        for (actual, expected) in decoded.iter().zip(sequences) {
            assert_eq!(actual.ll, expected.ll);
            assert_eq!(actual.ml, expected.ml);
            assert_eq!(actual.of, expected.of);
        }
    }

    #[test]
    fn match_length_code_52_uses_65539_baseline() {
        assert_eq!(encode_match_len(65538), (51, 32767, 15));
        assert_eq!(encode_match_len(65539), (52, 0, 16));
        assert_eq!(encode_match_len(98264), (52, 32725, 16));
        assert_eq!(encode_match_len(131074), (52, 65535, 16));
    }

    #[test]
    fn small_length_code_tables_match_spec_ranges() {
        for len in 0..=64 {
            assert_eq!(
                encode_literal_length(len),
                literal_length_code_from_spec(len)
            );
        }

        for len in 3..=131 {
            assert_eq!(encode_match_len(len), match_length_code_from_spec(len));
        }

        for len in 1..=129 {
            assert_eq!(encode_offset(len), offset_code_from_spec(len));
        }
    }

    #[test]
    fn raw_literals_use_shortest_header_form() {
        let mut one_byte_header = Vec::new();
        let mut writer = BitWriter::from(&mut one_byte_header);
        raw_literals(&[7; 31], &mut writer);
        writer.flush();
        assert_eq!(one_byte_header[0], 31 << 3);
        assert_eq!(&one_byte_header[1..], &[7; 31]);

        let mut two_byte_header = Vec::new();
        let mut writer = BitWriter::from(&mut two_byte_header);
        raw_literals(&[9; 44], &mut writer);
        writer.flush();
        assert_eq!(&two_byte_header[..2], &[0xC4, 0x02]);
        assert_eq!(&two_byte_header[2..], &[9; 44]);

        let mut three_byte_header = Vec::new();
        let mut writer = BitWriter::from(&mut three_byte_header);
        raw_literals(&[11; 4096], &mut writer);
        writer.flush();
        assert_eq!(&three_byte_header[..3], &[0x0C, 0x00, 0x01]);
        assert_eq!(&three_byte_header[3..], &[11; 4096]);
    }

    #[test]
    fn rle_literals_use_shortest_header_form() {
        let mut one_byte_header = Vec::new();
        let mut writer = BitWriter::from(&mut one_byte_header);
        rle_literals(&[7; 31], &mut writer);
        writer.flush();
        assert_eq!(&one_byte_header, &[0xF9, 7]);

        let mut two_byte_header = Vec::new();
        let mut writer = BitWriter::from(&mut two_byte_header);
        rle_literals(&[9; 44], &mut writer);
        writer.flush();
        assert_eq!(&two_byte_header, &[0xC5, 0x02, 9]);

        let mut three_byte_header = Vec::new();
        let mut writer = BitWriter::from(&mut three_byte_header);
        rle_literals(&[11; 4096], &mut writer);
        writer.flush();
        assert_eq!(&three_byte_header, &[0x0D, 0x00, 0x01, 11]);
    }

    #[test]
    fn rle_literals_round_trip_through_decoder() {
        let mut encoded = Vec::new();
        let mut writer = BitWriter::from(&mut encoded);
        rle_literals(&[42; 44], &mut writer);
        writer.flush();

        let mut section = crate::blocks::literals_section::LiteralsSection::new();
        let header_size = section.parse_from_header(&encoded).unwrap();
        assert!(matches!(
            section.ls_type,
            crate::blocks::literals_section::LiteralsSectionType::RLE
        ));

        let mut scratch = crate::decoding::scratch::HuffmanScratch::new();
        let mut decoded = Vec::new();
        let bytes_read = crate::decoding::literals_section_decoder::decode_literals(
            &section,
            &mut scratch,
            &encoded[header_size as usize..],
            &mut decoded,
        )
        .unwrap();

        assert_eq!(bytes_read, 1);
        assert_eq!(decoded, [42; 44]);
    }

    #[test]
    fn rle_literals_frame_round_trips_through_rust_and_c_decoders() {
        let (block_payload, frame, expected) =
            compressed_frame_with_literal_payload(alloc::vec![42; 2048]);

        assert_eq!(block_payload[0] & 0b11, 1, "literal section should be RLE");

        let mut rust_decoded = Vec::with_capacity(expected.len());
        let mut decoder = crate::decoding::FrameDecoder::new();
        decoder
            .decode_all_to_vec(&frame, &mut rust_decoded)
            .unwrap();
        assert_eq!(rust_decoded, expected);

        let mut c_decoded = Vec::new();
        zstd::stream::copy_decode(frame.as_slice(), &mut c_decoded).unwrap();
        assert_eq!(c_decoded, expected);
    }

    #[test]
    fn small_rle_literals_use_previous_table_threshold_and_round_trip() {
        let previous_table =
            huff0_encoder::HuffmanTable::build_from_counts(&[8, 1, 1, 1, 1, 1, 1, 1]);
        let (block_payload, frame, expected) = compressed_frame_with_literal_payload_and_last_table(
            alloc::vec![42; 7],
            Some(previous_table),
        );

        assert_eq!(
            block_payload[0] & 0b11,
            1,
            "small repeated literals should use RLE when a previous table lowers the threshold"
        );

        let mut rust_decoded = Vec::with_capacity(expected.len());
        let mut decoder = crate::decoding::FrameDecoder::new();
        decoder
            .decode_all_to_vec(&frame, &mut rust_decoded)
            .unwrap();
        assert_eq!(rust_decoded, expected);

        let mut c_decoded = Vec::new();
        zstd::stream::copy_decode(frame.as_slice(), &mut c_decoded).unwrap();
        assert_eq!(c_decoded, expected);
    }

    #[test]
    fn small_compressible_literals_use_huffman_and_round_trip() {
        let mut literals = alloc::vec![b'a'; 512];
        for idx in (15..literals.len()).step_by(16) {
            literals[idx] = b'b';
        }

        let (block_payload, frame, expected) = compressed_frame_with_literal_payload(literals);

        assert_eq!(
            block_payload[0] & 0b11,
            2,
            "small skewed literal section should use Huffman compression"
        );

        let mut rust_decoded = Vec::with_capacity(expected.len());
        let mut decoder = crate::decoding::FrameDecoder::new();
        decoder
            .decode_all_to_vec(&frame, &mut rust_decoded)
            .unwrap();
        assert_eq!(rust_decoded, expected);

        let mut c_decoded = Vec::new();
        zstd::stream::copy_decode(frame.as_slice(), &mut c_decoded).unwrap();
        assert_eq!(c_decoded, expected);
    }

    #[test]
    fn small_huffman_literals_use_single_stream_and_round_trip() {
        let mut literals = alloc::vec![b'a'; 128];
        for idx in (15..literals.len()).step_by(16) {
            literals[idx] = b'b';
        }

        let (block_payload, frame, expected) = compressed_frame_with_literal_payload(literals);

        assert_eq!(
            block_payload[0] & 0b11,
            2,
            "small skewed literal section should use Huffman compression"
        );
        assert_eq!(
            (block_payload[0] >> 2) & 0b11,
            0,
            "small Huffman literal payloads should use the single-stream header"
        );

        let mut rust_decoded = Vec::with_capacity(expected.len());
        let mut decoder = crate::decoding::FrameDecoder::new();
        decoder
            .decode_all_to_vec(&frame, &mut rust_decoded)
            .unwrap();
        assert_eq!(rust_decoded, expected);

        let mut c_decoded = Vec::new();
        zstd::stream::copy_decode(frame.as_slice(), &mut c_decoded).unwrap();
        assert_eq!(c_decoded, expected);
    }

    #[test]
    fn small_literals_prefer_previous_huffman_table_and_single_stream() {
        let mut first_literals = alloc::vec![0; 512];
        for idx in (15..first_literals.len()).step_by(16) {
            first_literals[idx] = 1;
        }
        let second_literals = first_literals.clone();

        let mut state = CompressState {
            matcher: LiteralPayloadMatcher {
                literals: first_literals.clone(),
                emitted: false,
            },
            last_huff_table: None,
            fse_tables: FseTables::new(),
            offset_history: OffsetHistory::new(),
            file_type_hint: CompressionFileType::Unknown,
            file_profile_hint: CompressionFileProfile::None,
        };
        let mut first_payload = Vec::new();
        state.last_huff_table = compress_block_with_config(
            &mut state,
            &mut first_payload,
            BlockCompressionConfig::for_level(CompressionLevel::Fastest),
        );

        state.matcher = LiteralPayloadMatcher {
            literals: second_literals.clone(),
            emitted: false,
        };
        let mut second_payload = Vec::new();
        compress_block_with_config(
            &mut state,
            &mut second_payload,
            BlockCompressionConfig::for_level(CompressionLevel::Fastest),
        );

        assert_eq!(
            second_payload[0] & 0b11,
            3,
            "small literals encodable by previous table should use treeless Huffman"
        );
        assert_eq!(
            (second_payload[0] >> 2) & 0b11,
            0,
            "small repeated-table Huffman literals should use the single-stream header"
        );

        let mut frame = Vec::new();
        crate::encoding::frame_header::FrameHeader {
            frame_content_size: None,
            single_segment: false,
            content_checksum: false,
            dictionary_id: None,
            window_size: Some(128 * 1024),
        }
        .serialize(&mut frame);
        crate::encoding::block_header::BlockHeader {
            last_block: false,
            block_type: crate::blocks::block::BlockType::Compressed,
            block_size: first_payload.len() as u32,
        }
        .serialize(&mut frame);
        frame.extend_from_slice(&first_payload);
        crate::encoding::block_header::BlockHeader {
            last_block: true,
            block_type: crate::blocks::block::BlockType::Compressed,
            block_size: second_payload.len() as u32,
        }
        .serialize(&mut frame);
        frame.extend_from_slice(&second_payload);

        let mut expected = first_literals.clone();
        expected.extend_from_slice(&[1; 16]);
        expected.extend_from_slice(&second_literals);
        expected.extend_from_slice(&[1; 16]);

        let mut rust_decoded = Vec::with_capacity(expected.len());
        let mut decoder = crate::decoding::FrameDecoder::new();
        decoder
            .decode_all_to_vec(&frame, &mut rust_decoded)
            .unwrap();
        assert_eq!(rust_decoded, expected);

        let mut c_decoded = Vec::new();
        zstd::stream::copy_decode(frame.as_slice(), &mut c_decoded).unwrap();
        assert_eq!(c_decoded, expected);
    }

    #[test]
    fn literal_estimate_without_gain_uses_raw_literals_and_round_trips() {
        let mut literals = alloc::vec![0; 6];
        for value in 1..=64u8 {
            literals.push(value);
        }

        let (block_payload, frame, expected) = compressed_frame_with_literal_payload(literals);

        assert_eq!(
            block_payload[0] & 0b11,
            0,
            "literal estimate should choose raw when Huffman cannot beat raw"
        );

        let mut rust_decoded = Vec::with_capacity(expected.len());
        let mut decoder = crate::decoding::FrameDecoder::new();
        decoder
            .decode_all_to_vec(&frame, &mut rust_decoded)
            .unwrap();
        assert_eq!(rust_decoded, expected);

        let mut c_decoded = Vec::new();
        zstd::stream::copy_decode(frame.as_slice(), &mut c_decoded).unwrap();
        assert_eq!(c_decoded, expected);
    }

    #[test]
    fn literal_min_gain_boundary_uses_exact_table_search_and_round_trips() {
        let len = 128usize;
        let period = 86u32;
        let mut state = (len as u32).wrapping_mul(1_664_525).wrapping_add(period);
        let mut literals = Vec::with_capacity(len);
        for _ in 0..len {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            literals.push((state % period) as u8);
        }

        let literal_stats = LiteralStats::from_literals(&literals);
        let table = huff0_encoder::HuffmanTable::build_from_counts(literal_stats.counts());
        let estimated_len = table.encoded_len(&literals, true, false);
        let exact_table = huff0_encoder::HuffmanTable::build_smallest_from_counts(
            literal_stats.counts(),
            &literals,
            false,
        );
        let exact_estimated_len = exact_table.encoded_len(&literals, true, false);
        let header_len = compressed_literals_header_len(0);

        assert_eq!(literal_min_gain(literals.len()), 4);
        assert!(
            estimated_len + header_len < literals.len(),
            "without the min-gain check this payload would be Huffman-compressed"
        );
        assert!(
            estimated_len
                >= literals
                    .len()
                    .saturating_sub(literal_min_gain(literals.len())),
            "C-style min-gain threshold should reject this narrow literal gain"
        );
        assert!(
            literal_estimate_has_enough_gain(exact_estimated_len, header_len, literals.len()),
            "exact table search should find enough gain for small all-literal payloads"
        );

        let (block_payload, frame, expected) = compressed_frame_with_literal_payload(literals);

        assert_eq!(
            block_payload[0] & 0b11,
            2,
            "small all-literal payloads should use the exact Huffman table when it has enough gain"
        );

        let mut rust_decoded = Vec::with_capacity(expected.len());
        let mut decoder = crate::decoding::FrameDecoder::new();
        decoder
            .decode_all_to_vec(&frame, &mut rust_decoded)
            .unwrap();
        assert_eq!(rust_decoded, expected);

        let mut c_decoded = Vec::new();
        zstd::stream::copy_decode(frame.as_slice(), &mut c_decoded).unwrap();
        assert_eq!(c_decoded, expected);
    }

    #[test]
    fn best_level_searches_exact_huffman_tables_beyond_small_literal_sections() {
        let len = SMALL_HUFFMAN_TABLE_SEARCH_MAX_LITERALS + 256;
        let period = 86u32;
        let mut state = (len as u32).wrapping_mul(1_664_525).wrapping_add(period);
        let mut literals = Vec::with_capacity(len);
        for _ in 0..len {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            literals.push((state % period) as u8);
        }

        let literal_stats = LiteralStats::from_literals(&literals);
        let baseline_table = huff0_encoder::HuffmanTable::build_from_counts(literal_stats.counts());
        let exact_table = huff0_encoder::HuffmanTable::build_smallest_from_counts(
            literal_stats.counts(),
            &literals,
            true,
        );
        assert!(
            exact_table.encoded_len(&literals, true, true)
                <= baseline_table.encoded_len(&literals, true, true),
            "exact table search should not be worse on this higher-level fixture"
        );

        let (fast_block, _, _) = compressed_frame_with_literal_payload_and_config(
            literals.clone(),
            None,
            BlockCompressionConfig::for_level_and_file_type(
                CompressionLevel::Fastest,
                CompressionFileType::ArchiveLike,
            ),
        );
        let (best_block, best_frame, expected) = compressed_frame_with_literal_payload_and_config(
            literals,
            None,
            BlockCompressionConfig::for_level_and_file_type(
                CompressionLevel::Best,
                CompressionFileType::ArchiveLike,
            ),
        );

        assert!(
            best_block.len() <= fast_block.len(),
            "best-level exact Huffman search should not emit a larger block: {} > {}",
            best_block.len(),
            fast_block.len()
        );

        let mut rust_decoded = Vec::with_capacity(expected.len());
        let mut decoder = crate::decoding::FrameDecoder::new();
        decoder
            .decode_all_to_vec(&best_frame, &mut rust_decoded)
            .unwrap();
        assert_eq!(rust_decoded, expected);

        let mut c_decoded = Vec::new();
        zstd::stream::copy_decode(best_frame.as_slice(), &mut c_decoded).unwrap();
        assert_eq!(c_decoded, expected);
    }

    #[test]
    fn fastest_code_and_config_text_enable_small_literal_exact_search() {
        assert!(matches!(
            BlockCompressionConfig::for_level_and_file_type(
                CompressionLevel::Fastest,
                CompressionFileType::CodeText,
            )
            .huffman_table_search,
            HuffmanTableSearch::FileTypeSmall
        ));
        assert!(matches!(
            BlockCompressionConfig::for_level_and_file_type(
                CompressionLevel::Fastest,
                CompressionFileType::ConfigText,
            )
            .huffman_table_search,
            HuffmanTableSearch::FileTypeSmall
        ));
        assert!(matches!(
            BlockCompressionConfig::for_level_and_file_type(
                CompressionLevel::Fastest,
                CompressionFileType::Unknown,
            )
            .huffman_table_search,
            HuffmanTableSearch::FileTypeSmall
        ));
        assert!(matches!(
            BlockCompressionConfig::for_level_and_file_type(
                CompressionLevel::Fastest,
                CompressionFileType::JsonText,
            )
            .huffman_table_search,
            HuffmanTableSearch::Heuristic
        ));
        assert!(matches!(
            BlockCompressionConfig::for_level_and_file_type(
                CompressionLevel::Fastest,
                CompressionFileType::DictionaryText,
            )
            .huffman_table_search,
            HuffmanTableSearch::AllSections
        ));
    }

    #[test]
    fn fastest_config_text_enables_small_single_stream_huffman_override() {
        assert_eq!(
            BlockCompressionConfig::for_level_and_file_type(
                CompressionLevel::Fastest,
                CompressionFileType::ConfigText,
            )
            .file_type_single_stream_huffman_max_literals,
            Some(FILE_TYPE_SINGLE_STREAM_HUFFMAN_MAX_LITERALS)
        );
        assert_eq!(
            BlockCompressionConfig::for_level_and_file_type(
                CompressionLevel::Fastest,
                CompressionFileType::CodeText,
            )
            .file_type_single_stream_huffman_max_literals,
            None
        );
    }

    #[test]
    fn fastest_dictionary_text_keeps_default_predefined_llml_window() {
        let config = BlockCompressionConfig::for_level_and_file_type(
            CompressionLevel::Fastest,
            CompressionFileType::DictionaryText,
        );
        assert_eq!(
            config.file_type_small_sequence_predefined_llml_max_sequences,
            None
        );

        let code_config = BlockCompressionConfig::for_level_and_file_type(
            CompressionLevel::Fastest,
            CompressionFileType::CodeText,
        );
        assert_eq!(
            code_config.file_type_small_sequence_predefined_llml_max_sequences,
            None
        );
    }

    #[test]
    fn fastest_dictionary_text_enables_exact_sequence_mode_search() {
        let dictionary_config = BlockCompressionConfig::for_level_and_hints(
            CompressionLevel::Fastest,
            CompressionFileType::DictionaryText,
            CompressionFileProfile::None,
        );
        assert!(dictionary_config.exact_sequence_mode_search);

        let dependency_json_config = BlockCompressionConfig::for_level_and_hints(
            CompressionLevel::Fastest,
            CompressionFileType::JsonText,
            CompressionFileProfile::DependencyJsonLockfile,
        );
        assert!(dependency_json_config.exact_sequence_mode_search);

        let small_text_lockfile_config = BlockCompressionConfig::for_level_and_hints(
            CompressionLevel::Fastest,
            CompressionFileType::ConfigText,
            CompressionFileProfile::SmallTextLockfile,
        );
        assert!(!small_text_lockfile_config.exact_sequence_mode_search);
        assert_eq!(small_text_lockfile_config.offset_table_max_log, 7);
        assert_eq!(
            small_text_lockfile_config.offset_predefined_max_sequences,
            64
        );
        assert_eq!(small_text_lockfile_config.repeat_table_max_sequences, 256);

        let code_config = BlockCompressionConfig::for_level_and_hints(
            CompressionLevel::Fastest,
            CompressionFileType::CodeText,
            CompressionFileProfile::None,
        );
        assert!(!code_config.exact_sequence_mode_search);
    }

    #[test]
    fn forced_single_stream_huffman_uses_single_stream_size_format() {
        assert_eq!(compressed_literals_size_format(821, false), (0b01, 10));
        assert_eq!(compressed_literals_size_format(821, true), (0b00, 10));
    }

    #[test]
    fn choose_table_repeats_previous_table_for_small_blocks_when_valid() {
        let default = default_of_table();
        let previous = build_table_from_data([29u8, 30, 30].iter().copied(), 8, true);
        let sequences = [
            crate::blocks::sequence_section::Sequence {
                ll: 0,
                ml: 3,
                of: 1 << 29,
            },
            crate::blocks::sequence_section::Sequence {
                ll: 0,
                ml: 3,
                of: 1 << 30,
            },
            crate::blocks::sequence_section::Sequence {
                ll: 0,
                ml: 3,
                of: 1 << 30,
            },
        ];

        assert!(matches!(
            choose_table(
                Some(&previous),
                &default,
                &sequences,
                |seq| encode_offset(seq.of).0,
                8,
                64,
                16,
            ),
            FseTableMode::RepeateLast(_)
        ));
    }

    #[test]
    fn exact_sequence_mode_search_never_worsens_threshold_choice() {
        let ll_default = default_ll_table();
        let ml_default = default_ml_table();
        let of_default = default_of_table();

        for a in 0..=8u32 {
            for b in 0..=8u32 {
                for c in 0..=8u32 {
                    for d in 0..=8u32 {
                        let sequences = [
                            crate::blocks::sequence_section::Sequence {
                                ll: 0,
                                ml: 3,
                                of: 1u32 << a,
                            },
                            crate::blocks::sequence_section::Sequence {
                                ll: 0,
                                ml: 3,
                                of: 1u32 << b,
                            },
                            crate::blocks::sequence_section::Sequence {
                                ll: 0,
                                ml: 3,
                                of: 1u32 << c,
                            },
                            crate::blocks::sequence_section::Sequence {
                                ll: 0,
                                ml: 3,
                                of: 1u32 << d,
                            },
                        ];
                        let heuristic_ll = choose_table(
                            None,
                            &ll_default,
                            &sequences,
                            |seq| encode_literal_length(seq.ll).0,
                            9,
                            64,
                            16,
                        );
                        let heuristic_ml = choose_table(
                            None,
                            &ml_default,
                            &sequences,
                            |seq| encode_match_len(seq.ml).0,
                            9,
                            64,
                            16,
                        );
                        let heuristic_of = choose_table(
                            None,
                            &of_default,
                            &sequences,
                            |seq| encode_offset(seq.of).0,
                            8,
                            64,
                            16,
                        );
                        let (ll_mode, ml_mode, of_mode) = choose_sequence_table_modes(
                            &sequences,
                            SequenceModeSearchConfig {
                                ll_previous: None,
                                ll_default: &ll_default,
                                ml_previous: None,
                                ml_default: &ml_default,
                                of_previous: None,
                                of_default: &of_default,
                                repeat_table_max_sequences: 64,
                                llml_predefined_max_sequences: 16,
                                of_predefined_max_sequences: 16,
                                of_max_log: 8,
                                exact_sequence_mode_search: true,
                            },
                        );
                        let heuristic_size = exact_sequence_section_size(
                            &sequences,
                            &heuristic_ll,
                            &heuristic_ml,
                            &heuristic_of,
                        );
                        let exact_size =
                            exact_sequence_section_size(&sequences, &ll_mode, &ml_mode, &of_mode);
                        assert!(exact_size <= heuristic_size);
                    }
                }
            }
        }
    }
}
