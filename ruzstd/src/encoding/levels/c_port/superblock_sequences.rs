//! Sequence-section helpers for target-compressed-size superblocks.

use alloc::vec::Vec;

use super::superblock::{EntropyTableMode, SequenceEntropyModes, SubBlockSequenceEmission};
use crate::encoding::{
    blocks::{
        append_compressed_sequence_section, append_predefined_sequence_section,
        append_repeat_sequence_section, append_rle_sequence_section,
        append_sequence_section_with_table_modes, build_compressed_sequence_tables,
        literal_length_code, match_length_code, offset_code, PreparedSequence, SequenceTableMode,
        SequenceTableModes,
    },
    frame_compressor::{FseTables, OffsetHistory},
};
use crate::fse::fse_encoder::FSETable;

const REPEAT_TABLE_MAX_SEQUENCES: usize = 64;

pub(super) fn append_sub_block_sequences(
    sequences: &[PreparedSequence],
    _modes: SequenceEntropyModes,
    _write_entropy: bool,
    output: &mut Vec<u8>,
) -> Option<SubBlockSequenceEmission> {
    if !sequences.is_empty() {
        return None;
    }

    output.push(0);
    Some(SubBlockSequenceEmission {
        byte_size: 1,
        entropy_written: false,
    })
}

pub(super) fn append_supported_sub_block_sequences(
    sequences: &[PreparedSequence],
    modes: SequenceEntropyModes,
    write_entropy: bool,
    fse_tables: &mut FseTables,
    offset_history: &mut OffsetHistory,
    output: &mut Vec<u8>,
) -> Option<SubBlockSequenceEmission> {
    if sequences.is_empty() {
        return append_sub_block_sequences(sequences, modes, write_entropy, output);
    }
    if !write_entropy {
        return append_repeat_sequence_section(sequences, fse_tables, offset_history, output).map(
            |byte_size| SubBlockSequenceEmission {
                byte_size,
                entropy_written: true,
            },
        );
    }

    let byte_size = if sequence_modes_are(modes, EntropyTableMode::Basic) {
        append_predefined_sequence_section(sequences, fse_tables, offset_history, output)?
    } else if sequence_modes_are(modes, EntropyTableMode::Rle) {
        append_rle_sequence_section(sequences, offset_history, output)?
    } else if sequence_modes_are(modes, EntropyTableMode::Repeat) {
        append_repeat_sequence_section(sequences, fse_tables, offset_history, output)?
    } else if sequence_modes_are(modes, EntropyTableMode::Compressed) {
        append_compressed_sequence_section(sequences, fse_tables, offset_history, output)?
    } else {
        let compressed_tables = need_compressed_sequence_tables(modes)
            .then(|| build_compressed_sequence_tables(sequences, *offset_history))
            .flatten();
        if need_compressed_sequence_tables(modes) && compressed_tables.is_none() {
            return None;
        }
        append_sequence_section_with_table_modes(
            sequences,
            sequence_table_modes(modes),
            compressed_tables.as_ref(),
            fse_tables,
            offset_history,
            output,
        )?
    };
    Some(SubBlockSequenceEmission {
        byte_size,
        entropy_written: true,
    })
}

pub(super) fn need_sequence_entropy_tables(modes: SequenceEntropyModes) -> bool {
    [modes.ll, modes.ml, modes.of]
        .iter()
        .any(|mode| matches!(mode, EntropyTableMode::Compressed | EntropyTableMode::Rle))
}

pub(super) fn select_sequence_entropy_modes(
    sequences: &[PreparedSequence],
    fse_tables: &FseTables,
    offset_history: OffsetHistory,
) -> SequenceEntropyModes {
    let mut offset_history = offset_history;
    let mut ll_codes = Vec::with_capacity(sequences.len());
    let mut ml_codes = Vec::with_capacity(sequences.len());
    let mut of_codes = Vec::with_capacity(sequences.len());

    for sequence in sequences {
        let offset_value = if let Some(offset_value) = sequence.encoded_offset_value {
            offset_history.update_from_offset_value(offset_value, sequence.ll, sequence.raw_offset);
            offset_value
        } else {
            offset_history.encode_offset_value(sequence.raw_offset, sequence.ll)
        };
        ll_codes.push(literal_length_code(sequence.ll));
        ml_codes.push(match_length_code(sequence.ml));
        of_codes.push(offset_code(offset_value));
    }

    SequenceEntropyModes {
        ll: select_stream_entropy_mode(
            &ll_codes,
            &fse_tables.ll_default,
            fse_tables.ll_previous.as_deref(),
        ),
        ml: select_stream_entropy_mode(
            &ml_codes,
            &fse_tables.ml_default,
            fse_tables.ml_previous.as_deref(),
        ),
        of: select_stream_entropy_mode(
            &of_codes,
            &fse_tables.of_default,
            fse_tables.of_previous.as_deref(),
        ),
    }
}

fn select_stream_entropy_mode(
    codes: &[u8],
    default_table: &FSETable,
    previous_table: Option<&FSETable>,
) -> EntropyTableMode {
    let all_same = codes
        .first()
        .is_some_and(|first| codes.iter().all(|code| code == first));
    if all_same && codes.len() > 2 {
        return EntropyTableMode::Rle;
    }
    if let Some(previous_table) = previous_table {
        if codes.len() < REPEAT_TABLE_MAX_SEQUENCES
            && codes
                .iter()
                .all(|code| previous_table.can_encode_symbol(*code))
        {
            return EntropyTableMode::Repeat;
        }
    }
    if codes
        .iter()
        .all(|code| default_table.can_encode_symbol(*code))
    {
        return EntropyTableMode::Basic;
    }
    EntropyTableMode::Compressed
}

fn sequence_modes_are(modes: SequenceEntropyModes, mode: EntropyTableMode) -> bool {
    modes.ll == mode && modes.ml == mode && modes.of == mode
}

fn need_compressed_sequence_tables(modes: SequenceEntropyModes) -> bool {
    [modes.ll, modes.ml, modes.of]
        .iter()
        .any(|mode| matches!(mode, EntropyTableMode::Compressed))
}

fn sequence_table_modes(modes: SequenceEntropyModes) -> SequenceTableModes {
    SequenceTableModes {
        ll: sequence_table_mode(modes.ll),
        ml: sequence_table_mode(modes.ml),
        of: sequence_table_mode(modes.of),
    }
}

fn sequence_table_mode(mode: EntropyTableMode) -> SequenceTableMode {
    match mode {
        EntropyTableMode::Basic => SequenceTableMode::Predefined,
        EntropyTableMode::Rle => SequenceTableMode::Rle,
        EntropyTableMode::Compressed => SequenceTableMode::Compressed,
        EntropyTableMode::Repeat => SequenceTableMode::Repeat,
    }
}
