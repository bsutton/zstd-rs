//! C-style compressed block size estimates used by post-sequence block splitting.
//!
//! These helpers mirror `ZSTD_buildEntropyStatisticsAndEstimateSubBlockSize()`:
//! they build the candidate entropy tables, estimate literal and sequence stream
//! costs, and intentionally avoid writing the actual block payload.

use alloc::vec::Vec;

use crate::{
    bit_io::BitWriter,
    blocks::sequence_section::Sequence,
    encoding::frame_compressor::{FseTables, OffsetHistory},
    fse::fse_encoder::FSETable,
    huff0::huff0_encoder,
};

use super::{
    config::HuffmanTableSearch,
    literals::{
        compressed_literals_header_len, compressed_literals_size_format, should_compress_literals,
        suspect_uncompressible_literals, LiteralStats,
    },
    sequence_bitstream::{encode_seqnum, encode_sequences_for_history},
    sequence_codes::{encode_literal_length, encode_match_len, encode_offset},
    sequence_cost::{cross_entropy_cost, repeat_table_cost, CodeCounts},
    sequence_tables::{
        choose_sequence_table_modes, encode_table, FseTableMode, SequenceModeSearchConfig,
    },
    BlockCompressionConfig, PreparedBlockRef,
};

const ZSTD_BLOCK_HEADER_SIZE: usize = 3;

pub(crate) fn estimate_prepared_block_size(
    config: BlockCompressionConfig,
    prepared: PreparedBlockRef<'_>,
    fse_tables: &FseTables,
    offset_history: OffsetHistory,
    previous_huff_table: Option<&huff0_encoder::HuffmanTable>,
) -> usize {
    let mut next_offset_history = offset_history;
    let sequences = encode_sequences_for_history(prepared.sequences, &mut next_offset_history);
    let literal_size = estimate_literal_section_size(
        prepared.literals,
        previous_huff_table,
        config,
        sequences.len(),
    );
    let sequence_size =
        estimate_sequence_section_size(&sequences, prepared.literals.len(), config, fse_tables);
    ZSTD_BLOCK_HEADER_SIZE + literal_size + sequence_size
}

fn estimate_literal_section_size(
    literals: &[u8],
    previous_table: Option<&huff0_encoder::HuffmanTable>,
    config: BlockCompressionConfig,
    sequence_count: usize,
) -> usize {
    if config.literal_compression_disabled
        || !should_compress_literals(
            literals.len(),
            previous_table.is_some(),
            config.literal_compression_min_size,
        )
    {
        return literals.len();
    }

    let force_single_stream = config
        .file_type_single_stream_huffman_max_literals
        .is_some_and(|max_literals| literals.len() <= max_literals);
    let (size_format, _) = compressed_literals_size_format(literals.len(), force_single_stream);
    let four_streams = size_format != 0;
    let (literal_stats, stream_counts) =
        LiteralStats::from_literals_with_stream_counts(literals, four_streams);

    if literal_stats.largest() == literals.len() {
        return usize::from(!literals.is_empty());
    }
    if literal_stats.likely_incompressible(literals.len())
        || suspect_uncompressible_literals(literals.len(), sequence_count)
            && sampled_literals_likely_incompressible(literals)
    {
        return literals.len();
    }

    let search_smallest_table = match config.huffman_table_search {
        HuffmanTableSearch::Heuristic => {
            sequence_count == 0
                || (sequence_count <= super::SMALL_HUFFMAN_TABLE_SEARCH_MAX_SEQUENCES
                    && literals.len() <= super::SMALL_HUFFMAN_TABLE_SEARCH_MAX_LITERALS)
        }
        HuffmanTableSearch::FileTypeSmall => {
            literals.len() <= super::FILE_TYPE_SMALL_HUFFMAN_TABLE_SEARCH_MAX_LITERALS
                || sequence_count == 0
                || (sequence_count <= super::SMALL_HUFFMAN_TABLE_SEARCH_MAX_SEQUENCES
                    && literals.len() <= super::SMALL_HUFFMAN_TABLE_SEARCH_MAX_LITERALS)
        }
        HuffmanTableSearch::AllSections => true,
    };
    let new_table = if search_smallest_table {
        huff0_encoder::HuffmanTable::build_smallest_from_counts(
            literal_stats.counts(),
            literals,
            four_streams,
        )
    } else {
        huff0_encoder::HuffmanTable::build_from_counts(literal_stats.counts())
    };
    let new_content_size =
        estimate_huffman_content_size(&new_table, literal_stats.counts(), stream_counts.as_ref());
    let new_table_size = new_table.encoded_len_from_counts(literal_stats.counts(), true)
        - new_table.encoded_len_from_counts(literal_stats.counts(), false);

    if let Some(previous_table) = previous_table {
        if previous_table.can_encode_counts(literal_stats.counts()) {
            let old_content_size = estimate_huffman_content_size(
                previous_table,
                literal_stats.counts(),
                stream_counts.as_ref(),
            );
            if old_content_size < literals.len()
                && (old_content_size <= new_table_size + new_content_size
                    || new_table_size + 12 >= literals.len())
            {
                return old_content_size + compressed_literals_header_len(size_format);
            }
        }
    }

    if new_content_size + new_table_size >= literals.len() {
        return literals.len();
    }
    new_content_size + new_table_size + compressed_literals_header_len(size_format)
}

fn estimate_huffman_content_size(
    table: &huff0_encoder::HuffmanTable,
    counts: &[usize],
    stream_counts: Option<&[[usize; 256]; 4]>,
) -> usize {
    if let Some(stream_counts) = stream_counts {
        table.encoded_len_from_stream_counts(stream_counts, false)
    } else {
        table.encoded_len_from_counts(counts, false)
    }
}

fn sampled_literals_likely_incompressible(literals: &[u8]) -> bool {
    const SAMPLE_SIZE: usize = 4096;
    const SAMPLE_RATIO: usize = 10;

    if literals.len() < SAMPLE_SIZE * SAMPLE_RATIO {
        return false;
    }

    let begin = &literals[..SAMPLE_SIZE];
    let end = &literals[literals.len() - SAMPLE_SIZE..];
    let largest_total = largest_symbol_count(begin) + largest_symbol_count(end);
    largest_total <= ((2 * SAMPLE_SIZE) >> 7) + 4
}

fn largest_symbol_count(literals: &[u8]) -> usize {
    let mut counts = [0usize; 256];
    let mut largest = 0usize;
    for &symbol in literals {
        counts[usize::from(symbol)] += 1;
        largest = largest.max(counts[usize::from(symbol)]);
    }
    largest
}

fn estimate_sequence_section_size(
    sequences: &[Sequence],
    literal_len: usize,
    config: BlockCompressionConfig,
    fse_tables: &FseTables,
) -> usize {
    if sequences.is_empty() {
        return 2;
    }

    let llml_predefined_max_sequences = if literal_len >= super::COMPRESS_LITERALS_SIZE_MIN {
        config
            .file_type_small_sequence_predefined_llml_max_sequences
            .unwrap_or(16)
    } else {
        16
    };
    let (ll_mode, ml_mode, of_mode) = choose_sequence_table_modes(
        sequences,
        SequenceModeSearchConfig {
            ll_previous: fse_tables.ll_previous.as_deref(),
            ll_default: &fse_tables.ll_default,
            ml_previous: fse_tables.ml_previous.as_deref(),
            ml_default: &fse_tables.ml_default,
            of_previous: fse_tables.of_previous.as_deref(),
            of_default: &fse_tables.of_default,
            repeat_table_max_sequences: config.repeat_table_max_sequences,
            llml_predefined_max_sequences,
            of_predefined_max_sequences: config.offset_predefined_max_sequences,
            of_max_log: config.offset_table_max_log,
            exact_sequence_mode_search: config.exact_sequence_mode_search,
            c_fast_heuristics: config.c_fast_sequence_table_heuristics,
            c_cost_model: config.c_cost_sequence_table_selection,
        },
    );

    sequence_header_size(sequences.len())
        + 1
        + table_definition_size(&ll_mode)
        + table_definition_size(&of_mode)
        + table_definition_size(&ml_mode)
        + estimate_symbol_stream_size(
            sequences,
            &of_mode,
            &fse_tables.of_default,
            |seq| encode_offset(seq.of).0,
            |seq| encode_offset(seq.of).2,
        )
        + estimate_symbol_stream_size(
            sequences,
            &ll_mode,
            &fse_tables.ll_default,
            |seq| encode_literal_length(seq.ll).0,
            |seq| encode_literal_length(seq.ll).2,
        )
        + estimate_symbol_stream_size(
            sequences,
            &ml_mode,
            &fse_tables.ml_default,
            |seq| encode_match_len(seq.ml).0,
            |seq| encode_match_len(seq.ml).2,
        )
}

fn sequence_header_size(seqnum: usize) -> usize {
    let mut bytes = Vec::new();
    let mut writer = BitWriter::from(&mut bytes);
    encode_seqnum(seqnum, &mut writer);
    writer.flush();
    bytes.len()
}

fn table_definition_size(mode: &FseTableMode<'_>) -> usize {
    let mut bytes = Vec::new();
    let mut writer = BitWriter::from(&mut bytes);
    encode_table(mode, &mut writer);
    writer.flush();
    bytes.len()
}

fn estimate_symbol_stream_size(
    sequences: &[Sequence],
    mode: &FseTableMode<'_>,
    default_table: &FSETable,
    code: impl Fn(&Sequence) -> u8,
    additional_bits: impl Fn(&Sequence) -> usize,
) -> usize {
    let counts = CodeCounts::from_codes(sequences.iter().map(&code));
    let symbol_bits = match mode {
        FseTableMode::Predefined(_) => cross_entropy_cost(default_table, &counts),
        FseTableMode::Rle(_) => Some(0),
        FseTableMode::Encoded(table) => repeat_table_cost(table, &counts),
        FseTableMode::RepeatLast(table) => repeat_table_cost(table, &counts),
    }
    .unwrap_or(sequences.len() * 10);
    let extra_bits = sequences.iter().map(additional_bits).sum::<usize>();
    (symbol_bits + extra_bits) >> 3
}
