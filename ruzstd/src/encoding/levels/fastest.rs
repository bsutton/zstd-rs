use crate::{
    blocks::sequence_section::{MAX_LITERAL_LENGTH_CODE, MAX_MATCH_LENGTH_CODE, MAX_OFFSET_CODE},
    common::MAX_BLOCK_SIZE,
    encoding::{
        block_header::BlockHeader,
        blocks::{
            compress_block_with_config, compress_prepared_block, literal_length_code,
            match_length_code, offset_code, prepare_block, BlockCompressionConfig,
            PreparedBlockRef,
        },
        frame_compressor::{CompressState, FseTables, OffsetHistory},
        util::{
            likely_composer_lockfile_text, likely_incompressible, likely_lockfile_text, likely_text,
        },
        CompressionFileProfile, CompressionLevel, Matcher,
    },
    huff0::huff0_encoder::HuffmanTable,
};
use alloc::vec::Vec;
use core::slice;
#[cfg(feature = "std")]
use std::sync::OnceLock;

const BEST_SPLIT_MIN_SEQUENCES: usize = 300;
const BEST_SPLIT_MIN_ESTIMATED_GAIN_BITS: f64 = 512.0;
const BEST_SPLIT_MAX_PARTITIONS: usize = 8;

#[cfg(feature = "std")]
#[derive(Clone, Copy, Debug, Default)]
struct FastestTuningOverrides {
    composer_max_partitions: Option<usize>,
    lockfile_fastest_splits: Option<bool>,
    lockfile_compare_whole_text: Option<bool>,
}

#[cfg(feature = "std")]
static FASTEST_TUNING_OVERRIDES: OnceLock<FastestTuningOverrides> = OnceLock::new();

#[cfg(feature = "std")]
fn fastest_tuning_overrides() -> &'static FastestTuningOverrides {
    FASTEST_TUNING_OVERRIDES.get_or_init(FastestTuningOverrides::from_env)
}

#[cfg(feature = "std")]
impl FastestTuningOverrides {
    fn from_env() -> Self {
        Self {
            composer_max_partitions: std::env::var("RUZSTD_TUNE_COMPOSER_MAX_PARTITIONS")
                .ok()
                .and_then(|value| value.parse().ok()),
            lockfile_fastest_splits: parse_bool_env("RUZSTD_TUNE_LOCKFILE_FASTEST_SPLITS"),
            lockfile_compare_whole_text: parse_bool_env("RUZSTD_TUNE_LOCKFILE_COMPARE_WHOLE_TEXT"),
        }
    }
}

#[cfg(feature = "std")]
fn parse_bool_env(name: &str) -> Option<bool> {
    match std::env::var(name).ok()?.as_str() {
        "1" | "true" | "TRUE" | "yes" | "YES" | "on" | "ON" => Some(true),
        "0" | "false" | "FALSE" | "no" | "NO" | "off" | "OFF" => Some(false),
        _ => None,
    }
}

/// Compresses a single block at [`crate::encoding::CompressionLevel::Fastest`].
///
/// # Parameters
/// - `state`: [`CompressState`] so the compressor can refer to data before
///   the start of this block
/// - `last_block`: Whether or not this block is going to be the last block in the frame
///   (needed because this info is written into the block header)
/// - `uncompressed_data`: A block's worth of uncompressed data, taken from the
///   larger input
/// - `output`: As `uncompressed_data` is compressed, it's appended to `output`.
#[inline]
pub fn compress_fastest<M: Matcher>(
    state: &mut CompressState<M>,
    last_block: bool,
    uncompressed_data: Vec<u8>,
    output: &mut Vec<u8>,
) {
    compress_at_level_with_options(
        state,
        CompressionLevel::Fastest,
        last_block,
        uncompressed_data,
        output,
        true,
    );
}

#[inline]
pub fn compress_at_level<M: Matcher>(
    state: &mut CompressState<M>,
    level: CompressionLevel,
    last_block: bool,
    uncompressed_data: Vec<u8>,
    output: &mut Vec<u8>,
) {
    compress_at_level_with_options(state, level, last_block, uncompressed_data, output, true);
}

#[inline]
pub(crate) fn compress_at_level_without_incompressible_probe<M: Matcher>(
    state: &mut CompressState<M>,
    level: CompressionLevel,
    last_block: bool,
    uncompressed_data: Vec<u8>,
    output: &mut Vec<u8>,
) {
    compress_at_level_with_options(state, level, last_block, uncompressed_data, output, false);
}

#[inline]
fn compress_at_level_with_options<M: Matcher>(
    state: &mut CompressState<M>,
    level: CompressionLevel,
    last_block: bool,
    uncompressed_data: Vec<u8>,
    output: &mut Vec<u8>,
    allow_incompressible_probe: bool,
) {
    let block_size = uncompressed_data.len() as u32;
    let is_composer_dictionary_block =
        matches!(
            state.file_type_hint,
            crate::encoding::CompressionFileType::DictionaryText
        ) && (matches!(
            state.file_profile_hint,
            CompressionFileProfile::ComposerLock
        ) || likely_composer_lockfile_text(&uncompressed_data));
    let is_lockfile_dictionary_block = matches!(
        state.file_type_hint,
        crate::encoding::CompressionFileType::DictionaryText
    ) && likely_lockfile_text(&uncompressed_data);
    if uncompressed_data.is_empty() {
        write_raw_block(last_block, block_size, &uncompressed_data, output);
        return;
    }

    // First check to see if run length encoding can be used for the entire block
    if uncompressed_data.iter().all(|x| uncompressed_data[0].eq(x)) {
        let rle_byte = uncompressed_data[0];
        state.matcher.commit_space(uncompressed_data);
        state.matcher.skip_matching_for_rle();
        write_rle_block(last_block, block_size, rle_byte, output);
    } else if allow_incompressible_probe && likely_incompressible(&uncompressed_data) {
        state.matcher.commit_space(uncompressed_data);
        state.matcher.skip_matching_for_incompressible();
        write_raw_block(
            last_block,
            block_size,
            state.matcher.get_last_space(),
            output,
        );
    } else {
        // Compress as a standard compressed block
        state.matcher.commit_space(uncompressed_data);
        #[cfg(feature = "std")]
        let use_lockfile_fastest_splits = matches!(
            fastest_tuning_overrides().lockfile_fastest_splits,
            Some(true)
        ) && matches!(level, CompressionLevel::Fastest)
            && is_lockfile_dictionary_block;
        #[cfg(not(feature = "std"))]
        let use_lockfile_fastest_splits = false;
        if matches!(level, CompressionLevel::Best)
            || (matches!(level, CompressionLevel::Fastest) && is_composer_dictionary_block)
            || use_lockfile_fastest_splits
        {
            let partition_config = if matches!(level, CompressionLevel::Best) {
                BlockCompressionConfig::for_level(CompressionLevel::Best)
            } else {
                BlockCompressionConfig::for_level_and_hints(
                    level,
                    state.file_type_hint,
                    state.file_profile_hint,
                )
            };
            let max_partitions =
                if matches!(level, CompressionLevel::Fastest) && is_composer_dictionary_block {
                    #[cfg(feature = "std")]
                    if let Some(value) = fastest_tuning_overrides().composer_max_partitions {
                        value.max(1)
                    } else {
                        BEST_SPLIT_MAX_PARTITIONS
                    }
                    #[cfg(not(feature = "std"))]
                    {
                        BEST_SPLIT_MAX_PARTITIONS
                    }
                } else {
                    BEST_SPLIT_MAX_PARTITIONS
                };
            #[cfg(feature = "std")]
            let force_compare_whole = use_lockfile_fastest_splits
                && matches!(
                    fastest_tuning_overrides().lockfile_compare_whole_text,
                    Some(true)
                );
            #[cfg(not(feature = "std"))]
            let force_compare_whole = false;
            compress_best_with_estimated_splits(
                state,
                last_block,
                output,
                max_partitions,
                partition_config,
                force_compare_whole,
            );
        } else {
            emit_matcher_block_or_raw(state, level, last_block, output);
        }
    }
}

fn emit_matcher_block_or_raw<M: Matcher>(
    state: &mut CompressState<M>,
    level: CompressionLevel,
    last_block: bool,
    output: &mut Vec<u8>,
) {
    let block_size = state.matcher.get_last_space().len() as u32;
    let config = BlockCompressionConfig::for_level_and_hints(
        level,
        state.file_type_hint,
        state.file_profile_hint,
    );
    let previous_ll = state.fse_tables.ll_previous.clone();
    let previous_ml = state.fse_tables.ml_previous.clone();
    let previous_of = state.fse_tables.of_previous.clone();
    let previous_offsets = state.offset_history;
    let block_start = output.len();
    output.extend_from_slice(&[0; 3]);
    output.reserve(compressed_block_reserve(block_size));
    let compressed_start = output.len();
    let new_huffman_table = compress_block_with_config(state, output, config);
    let compressed_size = output.len() - compressed_start;
    if compressed_size >= block_size as usize || compressed_size > MAX_BLOCK_SIZE as usize {
        output.truncate(block_start);
        state.fse_tables.ll_previous = previous_ll;
        state.fse_tables.ml_previous = previous_ml;
        state.fse_tables.of_previous = previous_of;
        state.offset_history = previous_offsets;
        let (newest, second, third) = previous_offsets.as_offsets();
        state.matcher.set_repeat_offsets(newest, second, third);

        write_raw_block(
            last_block,
            block_size,
            state.matcher.get_last_space(),
            output,
        );
    } else {
        let header = BlockHeader {
            last_block,
            block_type: crate::blocks::block::BlockType::Compressed,
            block_size: compressed_size as u32,
        };
        output[block_start..compressed_start].copy_from_slice(&header.serialize_to_bytes());
        if let Some(table) = new_huffman_table {
            state.last_huff_table = Some(table);
        }
    }
}

fn compress_best_with_estimated_splits<M: Matcher>(
    state: &mut CompressState<M>,
    last_block: bool,
    output: &mut Vec<u8>,
    max_partitions: usize,
    partition_config: BlockCompressionConfig,
    force_compare_whole: bool,
) {
    let baseline_offsets = state.offset_history;
    let baseline_last_huff = state.last_huff_table.take();
    let prepared = prepare_block(state);
    let data = state.matcher.get_last_space();
    let whole_range = PreparedRange {
        prepared: prepared.as_ref(),
        data,
    };
    let mut partitions = Vec::with_capacity(max_partitions);
    derive_best_partitions(
        whole_range,
        baseline_offsets,
        max_partitions,
        &mut partitions,
    );
    let compare_against_whole =
        partitions.len() == max_partitions && (force_compare_whole || !likely_text(data));
    let baseline_fse_previous =
        compare_against_whole.then(|| FsePreviousState::snapshot(&state.fse_tables));
    let whole_candidate = if compare_against_whole {
        let mut encode_state = CandidateEncodeState {
            fse_tables: &mut state.fse_tables,
            offset_history: &mut state.offset_history,
        };
        attempt_partitioned_candidate(
            slice::from_ref(&whole_range),
            last_block,
            partition_config,
            baseline_last_huff.as_ref(),
            &mut encode_state,
        )
    } else {
        None
    };

    if let Some(baseline_fse_previous) = baseline_fse_previous.as_ref() {
        baseline_fse_previous.clone().restore(&mut state.fse_tables);
        state.offset_history = baseline_offsets;
    }

    let partitioned_candidate = {
        let mut encode_state = CandidateEncodeState {
            fse_tables: &mut state.fse_tables,
            offset_history: &mut state.offset_history,
        };
        attempt_partitioned_candidate(
            &partitions,
            last_block,
            partition_config,
            baseline_last_huff.as_ref(),
            &mut encode_state,
        )
    };

    let chosen = choose_best_candidate(whole_candidate, partitioned_candidate);
    let Some(chosen) = chosen else {
        if let Some(baseline_fse_previous) = baseline_fse_previous {
            baseline_fse_previous.restore(&mut state.fse_tables);
        }
        state.offset_history = baseline_offsets;
        state.last_huff_table = baseline_last_huff;
        write_raw_block(last_block, data.len() as u32, data, output);
        return;
    };
    if partitions.len() > 1 && chosen.bytes.len() >= data.len() + 3 {
        state.fse_tables.reset();
        state.offset_history = baseline_offsets;
        state.last_huff_table = baseline_last_huff;
        write_raw_block(last_block, data.len() as u32, data, output);
        return;
    }

    chosen.final_fse_previous.restore(&mut state.fse_tables);
    state.offset_history = chosen.final_offset_history;
    state.last_huff_table = chosen.final_last_huff.or(baseline_last_huff);
    output.extend_from_slice(&chosen.bytes);
}

fn choose_best_candidate(
    whole: Option<CandidateResult>,
    partitioned: Option<CandidateResult>,
) -> Option<CandidateResult> {
    match (whole, partitioned) {
        (Some(whole), Some(partitioned)) => {
            if partitioned.bytes.len() < whole.bytes.len() {
                Some(partitioned)
            } else {
                Some(whole)
            }
        }
        (Some(whole), None) => Some(whole),
        (None, Some(partitioned)) => Some(partitioned),
        (None, None) => None,
    }
}

#[derive(Clone, Copy)]
struct PreparedRange<'a> {
    prepared: PreparedBlockRef<'a>,
    data: &'a [u8],
}

struct SplitEstimate {
    mid: usize,
}

fn best_mid_split_estimate(
    prepared: PreparedBlockRef<'_>,
    initial_offsets: OffsetHistory,
) -> Option<SplitEstimate> {
    if prepared.sequences.len() < BEST_SPLIT_MIN_SEQUENCES {
        return None;
    }

    let mid = best_split_mid_by_decompressed_bytes(prepared);
    if mid == 0 || mid == prepared.sequences.len() {
        return None;
    }

    let first_literals_len: usize = prepared.sequences[..mid]
        .iter()
        .map(|sequence| sequence.ll as usize)
        .sum();

    let mut whole_literal_counts = [0u32; 256];
    let mut first_literal_counts = [0u32; 256];
    let mut second_literal_counts = [0u32; 256];
    for (idx, &byte) in prepared.literals.iter().enumerate() {
        whole_literal_counts[byte as usize] += 1;
        if idx < first_literals_len {
            first_literal_counts[byte as usize] += 1;
        } else {
            second_literal_counts[byte as usize] += 1;
        }
    }

    let mut whole_ll_counts = [0u32; MAX_LITERAL_LENGTH_CODE as usize + 1];
    let mut first_ll_counts = [0u32; MAX_LITERAL_LENGTH_CODE as usize + 1];
    let mut second_ll_counts = [0u32; MAX_LITERAL_LENGTH_CODE as usize + 1];
    let mut whole_ml_counts = [0u32; MAX_MATCH_LENGTH_CODE as usize + 1];
    let mut first_ml_counts = [0u32; MAX_MATCH_LENGTH_CODE as usize + 1];
    let mut second_ml_counts = [0u32; MAX_MATCH_LENGTH_CODE as usize + 1];
    let mut whole_of_counts = [0u32; MAX_OFFSET_CODE as usize + 1];
    let mut first_of_counts = [0u32; MAX_OFFSET_CODE as usize + 1];
    let mut second_of_counts = [0u32; MAX_OFFSET_CODE as usize + 1];

    let mut offset_history = initial_offsets;
    for (idx, sequence) in prepared.sequences.iter().enumerate() {
        let ll_code = literal_length_code(sequence.ll);
        let ml_code = match_length_code(sequence.ml);
        let offset_value = offset_history.encode_offset_value(sequence.raw_offset, sequence.ll);
        let of_code = offset_code(offset_value);

        whole_ll_counts[ll_code as usize] += 1;
        whole_ml_counts[ml_code as usize] += 1;
        whole_of_counts[of_code as usize] += 1;

        let (ll_counts, ml_counts, of_counts) = if idx < mid {
            (
                &mut first_ll_counts,
                &mut first_ml_counts,
                &mut first_of_counts,
            )
        } else {
            (
                &mut second_ll_counts,
                &mut second_ml_counts,
                &mut second_of_counts,
            )
        };
        ll_counts[ll_code as usize] += 1;
        ml_counts[ml_code as usize] += 1;
        of_counts[of_code as usize] += 1;
    }

    let whole_bits = estimated_entropy_bits(&whole_literal_counts, prepared.literals.len())
        + estimated_entropy_bits(&whole_ll_counts, prepared.sequences.len())
        + estimated_entropy_bits(&whole_ml_counts, prepared.sequences.len())
        + estimated_entropy_bits(&whole_of_counts, prepared.sequences.len());
    let split_bits = estimated_entropy_bits(&first_literal_counts, first_literals_len)
        + estimated_entropy_bits(
            &second_literal_counts,
            prepared.literals.len().saturating_sub(first_literals_len),
        )
        + estimated_entropy_bits(&first_ll_counts, mid)
        + estimated_entropy_bits(&second_ll_counts, prepared.sequences.len() - mid)
        + estimated_entropy_bits(&first_ml_counts, mid)
        + estimated_entropy_bits(&second_ml_counts, prepared.sequences.len() - mid)
        + estimated_entropy_bits(&first_of_counts, mid)
        + estimated_entropy_bits(&second_of_counts, prepared.sequences.len() - mid);

    if split_bits + BEST_SPLIT_MIN_ESTIMATED_GAIN_BITS >= whole_bits {
        return None;
    }

    Some(SplitEstimate { mid })
}

fn best_split_mid_by_decompressed_bytes(prepared: PreparedBlockRef<'_>) -> usize {
    let target = prepared_decompressed_len(prepared) / 2;
    split_mid_by_target_decompressed_bytes(prepared, target)
}

fn prepared_decompressed_len(prepared: PreparedBlockRef<'_>) -> usize {
    prepared.literals.len()
        + prepared
            .sequences
            .iter()
            .map(|sequence| sequence.ml as usize)
            .sum::<usize>()
}

fn split_mid_by_target_decompressed_bytes(prepared: PreparedBlockRef<'_>, target: usize) -> usize {
    let mut total = 0usize;
    let mut best_mid = prepared.sequences.len() / 2;
    let mut best_distance = usize::MAX;

    for (idx, sequence) in prepared.sequences.iter().enumerate() {
        total += sequence.ll as usize + sequence.ml as usize;
        let mid = idx + 1;
        if mid == prepared.sequences.len() {
            break;
        }

        let distance = total.abs_diff(target);
        if distance < best_distance {
            best_distance = distance;
            best_mid = mid;
        }
    }

    best_mid
}

fn derive_best_partitions<'a>(
    range: PreparedRange<'a>,
    initial_offsets: OffsetHistory,
    remaining_partitions: usize,
    partitions: &mut Vec<PreparedRange<'a>>,
) -> usize {
    if remaining_partitions <= 1 {
        partitions.push(range);
        return 1;
    }

    let Some(estimate) = best_mid_split_estimate(range.prepared, initial_offsets) else {
        partitions.push(range);
        return 1;
    };

    let (first, second) = split_prepared_range(range, estimate.mid);
    let second_offsets = advance_offset_history(initial_offsets, first.prepared.sequences);
    let used_left =
        derive_best_partitions(first, initial_offsets, remaining_partitions - 1, partitions);
    let remaining_for_right = remaining_partitions.saturating_sub(used_left);
    if remaining_for_right == 0 {
        return used_left;
    }
    let used_right =
        derive_best_partitions(second, second_offsets, remaining_for_right, partitions);
    used_left + used_right
}

fn advance_offset_history(
    mut offset_history: OffsetHistory,
    sequences: &[crate::encoding::blocks::PreparedSequence],
) -> OffsetHistory {
    for sequence in sequences {
        offset_history.update_after_match(sequence.raw_offset, sequence.ll != 0);
    }
    offset_history
}

fn estimated_entropy_bits(counts: &[u32], total: usize) -> f64 {
    if total <= 1 {
        return 0.0;
    }

    let total_f = total as f64;
    let mut bits = 0.0;
    for &count in counts {
        if count == 0 {
            continue;
        }
        let count_f = count as f64;
        bits += count_f * (total_f.log2() - count_f.log2());
    }
    bits
}

struct CandidateResult {
    bytes: Vec<u8>,
    final_fse_previous: FsePreviousState,
    final_offset_history: OffsetHistory,
    final_last_huff: Option<HuffmanTable>,
}

struct CandidateEncodeState<'a> {
    fse_tables: &'a mut FseTables,
    offset_history: &'a mut OffsetHistory,
}

#[derive(Clone)]
struct FsePreviousState {
    ll_previous: Option<crate::fse::fse_encoder::FSETable>,
    ml_previous: Option<crate::fse::fse_encoder::FSETable>,
    of_previous: Option<crate::fse::fse_encoder::FSETable>,
}

impl FsePreviousState {
    fn snapshot(fse_tables: &FseTables) -> Self {
        Self {
            ll_previous: fse_tables.ll_previous.clone(),
            ml_previous: fse_tables.ml_previous.clone(),
            of_previous: fse_tables.of_previous.clone(),
        }
    }

    fn restore(self, fse_tables: &mut FseTables) {
        fse_tables.ll_previous = self.ll_previous;
        fse_tables.ml_previous = self.ml_previous;
        fse_tables.of_previous = self.of_previous;
    }
}

enum CandidateHuffmanState<'a> {
    Unchanged(Option<&'a HuffmanTable>),
    Updated(HuffmanTable),
}

impl CandidateHuffmanState<'_> {
    fn as_ref(&self) -> Option<&HuffmanTable> {
        match self {
            Self::Unchanged(table) => *table,
            Self::Updated(table) => Some(table),
        }
    }

    fn update(&mut self, table: HuffmanTable) {
        *self = Self::Updated(table);
    }

    fn into_owned(self) -> Option<HuffmanTable> {
        match self {
            Self::Unchanged(_) => None,
            Self::Updated(table) => Some(table),
        }
    }
}

fn attempt_partitioned_candidate(
    partitions: &[PreparedRange<'_>],
    last_block: bool,
    config: BlockCompressionConfig,
    previous_huff_table: Option<&HuffmanTable>,
    encode_state: &mut CandidateEncodeState<'_>,
) -> Option<CandidateResult> {
    attempt_partitioned_candidate_with_config(
        partitions,
        last_block,
        config,
        previous_huff_table,
        encode_state,
    )
}

fn attempt_partitioned_candidate_with_config(
    partitions: &[PreparedRange<'_>],
    last_block: bool,
    config: BlockCompressionConfig,
    previous_huff_table: Option<&HuffmanTable>,
    encode_state: &mut CandidateEncodeState<'_>,
) -> Option<CandidateResult> {
    let mut bytes = Vec::new();
    let mut current_last_huff = CandidateHuffmanState::Unchanged(previous_huff_table);
    for (idx, partition) in partitions.iter().enumerate() {
        emit_prepared_candidate(
            partition.prepared,
            partition.data,
            last_block && idx + 1 == partitions.len(),
            config,
            &mut bytes,
            &mut current_last_huff,
            encode_state,
        )?;
    }
    Some(CandidateResult {
        bytes,
        final_fse_previous: FsePreviousState::snapshot(encode_state.fse_tables),
        final_offset_history: *encode_state.offset_history,
        final_last_huff: current_last_huff.into_owned(),
    })
}

fn emit_prepared_candidate(
    prepared: PreparedBlockRef<'_>,
    data: &[u8],
    last_block: bool,
    config: BlockCompressionConfig,
    output: &mut Vec<u8>,
    current_last_huff: &mut CandidateHuffmanState<'_>,
    encode_state: &mut CandidateEncodeState<'_>,
) -> Option<()> {
    emit_prepared_candidate_with_config(
        prepared,
        data,
        last_block,
        config,
        output,
        current_last_huff,
        encode_state,
    )
}

fn emit_prepared_candidate_with_config(
    prepared: PreparedBlockRef<'_>,
    data: &[u8],
    last_block: bool,
    config: BlockCompressionConfig,
    output: &mut Vec<u8>,
    current_last_huff: &mut CandidateHuffmanState<'_>,
    encode_state: &mut CandidateEncodeState<'_>,
) -> Option<()> {
    let block_size = data.len() as u32;
    if data.is_empty() {
        write_raw_block(last_block, block_size, data, output);
        return Some(());
    }

    if data.iter().all(|byte| *byte == data[0]) {
        let header = BlockHeader {
            last_block,
            block_type: crate::blocks::block::BlockType::RLE,
            block_size,
        };
        header.serialize(output);
        output.push(data[0]);
        return Some(());
    }

    let previous_fse = FsePreviousState::snapshot(encode_state.fse_tables);
    let previous_offsets = *encode_state.offset_history;
    let block_start = output.len();
    output.extend_from_slice(&[0; 3]);
    output.reserve(compressed_block_reserve(block_size));
    let compressed_start = output.len();
    let new_huffman_table = compress_prepared_block(
        output,
        config,
        prepared,
        encode_state.fse_tables,
        encode_state.offset_history,
        current_last_huff.as_ref(),
    );
    let compressed_size = output.len() - compressed_start;

    if compressed_size >= block_size as usize || compressed_size > MAX_BLOCK_SIZE as usize {
        output.truncate(block_start);
        previous_fse.restore(encode_state.fse_tables);
        *encode_state.offset_history = previous_offsets;
        write_raw_block(last_block, block_size, data, output);
    } else {
        let header = BlockHeader {
            last_block,
            block_type: crate::blocks::block::BlockType::Compressed,
            block_size: compressed_size as u32,
        };
        output[block_start..compressed_start].copy_from_slice(&header.serialize_to_bytes());
        if let Some(table) = new_huffman_table {
            current_last_huff.update(table);
        }
    }

    Some(())
}

fn split_prepared_range(
    range: PreparedRange<'_>,
    mid: usize,
) -> (PreparedRange<'_>, PreparedRange<'_>) {
    let first_literals_len: usize = range.prepared.sequences[..mid]
        .iter()
        .map(|sequence| sequence.ll as usize)
        .sum();
    let first_match_len: usize = range.prepared.sequences[..mid]
        .iter()
        .map(|sequence| sequence.ml as usize)
        .sum();
    let first_data_len = first_literals_len + first_match_len;

    let first = PreparedRange {
        prepared: PreparedBlockRef {
            literals: &range.prepared.literals[..first_literals_len],
            sequences: &range.prepared.sequences[..mid],
        },
        data: &range.data[..first_data_len],
    };
    let second = PreparedRange {
        prepared: PreparedBlockRef {
            literals: &range.prepared.literals[first_literals_len..],
            sequences: &range.prepared.sequences[mid..],
        },
        data: &range.data[first_data_len..],
    };
    (first, second)
}

fn compressed_block_reserve(block_size: u32) -> usize {
    (block_size as usize / 2).max(1024)
}

fn write_rle_block(last_block: bool, block_size: u32, rle_byte: u8, output: &mut Vec<u8>) {
    let header = BlockHeader {
        last_block,
        block_type: crate::blocks::block::BlockType::RLE,
        block_size,
    };
    header.serialize(output);
    output.push(rle_byte);
}

fn write_raw_block(last_block: bool, block_size: u32, data: &[u8], output: &mut Vec<u8>) {
    let header = BlockHeader {
        last_block,
        block_type: crate::blocks::block::BlockType::Raw,
        block_size,
    };
    header.serialize(output);
    output.extend_from_slice(data);
}

#[cfg(test)]
mod tests {
    use super::{best_split_mid_by_decompressed_bytes, derive_best_partitions, PreparedRange};
    use crate::encoding::{
        blocks::{PreparedBlock, PreparedSequence},
        frame_compressor::OffsetHistory,
    };
    use alloc::{vec, vec::Vec};

    #[test]
    fn best_split_mid_biases_toward_balanced_decompressed_bytes() {
        let prepared = PreparedBlock {
            literals: vec![0; 103],
            sequences: vec![
                PreparedSequence {
                    ll: 100,
                    ml: 100,
                    raw_offset: 4,
                },
                PreparedSequence {
                    ll: 1,
                    ml: 1,
                    raw_offset: 4,
                },
                PreparedSequence {
                    ll: 1,
                    ml: 1,
                    raw_offset: 4,
                },
                PreparedSequence {
                    ll: 1,
                    ml: 1,
                    raw_offset: 4,
                },
            ],
        };

        assert_eq!(best_split_mid_by_decompressed_bytes(prepared.as_ref()), 1);
    }

    #[test]
    fn best_split_mid_keeps_even_sequences_centered() {
        let prepared = PreparedBlock {
            literals: vec![0; 8],
            sequences: vec![
                PreparedSequence {
                    ll: 2,
                    ml: 2,
                    raw_offset: 4,
                },
                PreparedSequence {
                    ll: 2,
                    ml: 2,
                    raw_offset: 4,
                },
                PreparedSequence {
                    ll: 2,
                    ml: 2,
                    raw_offset: 4,
                },
                PreparedSequence {
                    ll: 2,
                    ml: 2,
                    raw_offset: 4,
                },
            ],
        };

        assert_eq!(best_split_mid_by_decompressed_bytes(prepared.as_ref()), 2);
    }

    #[test]
    fn derive_best_partitions_respects_partition_budget() {
        let quarter_len = 300;
        let sequence_count = quarter_len * 4;
        let mut literals = Vec::with_capacity(sequence_count);
        literals.extend(core::iter::repeat_n(0u8, quarter_len));
        literals.extend(core::iter::repeat_n(1u8, quarter_len));
        literals.extend(core::iter::repeat_n(2u8, quarter_len));
        literals.extend(core::iter::repeat_n(3u8, quarter_len));
        let sequences = vec![
            PreparedSequence {
                ll: 1,
                ml: 3,
                raw_offset: 4,
            };
            sequence_count
        ];
        let prepared = PreparedBlock {
            literals,
            sequences,
        };
        let data = vec![0u8; sequence_count * 4];
        let whole_range = PreparedRange {
            prepared: prepared.as_ref(),
            data: &data,
        };
        let mut partitions = Vec::new();

        let used = derive_best_partitions(whole_range, OffsetHistory::new(), 2, &mut partitions);

        assert_eq!(used, 2);
        assert_eq!(partitions.len(), 2);
    }
}
