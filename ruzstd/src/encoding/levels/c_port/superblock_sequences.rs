//! Sequence-section helpers for target-compressed-size superblocks.

use alloc::vec::Vec;

use super::superblock::{EntropyTableMode, SequenceEntropyModes, SubBlockSequenceEmission};
use crate::encoding::{
    blocks::{
        append_compressed_sequence_section, append_predefined_sequence_section,
        append_repeat_sequence_section, append_rle_sequence_section,
        append_sequence_section_with_table_modes, build_compressed_sequence_tables,
        PreparedSequence, SequenceTableMode, SequenceTableModes,
    },
    frame_compressor::{FseTables, OffsetHistory},
};

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
