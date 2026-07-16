//! Target-compressed-block-size block adapter.

use alloc::vec::Vec;

use super::{
    block_emit::append_raw_block,
    greedy_block::{GreedyBlockEncodeContext, GreedyEncodedBlock, GreedyPreparedBlock},
    sequence_store::RepeatOffsets,
    superblock::{
        append_literal_only_sub_block, append_sequence_sub_block, should_commit_sub_block,
        EntropyTableMode, SequenceEntropyModes,
    },
};
use crate::encoding::frame_compressor::{FseTables, OffsetHistory};

pub(super) fn encode_target_block_with_superblock_fallback(
    block: &[u8],
    last_block: bool,
    repeat_offsets: RepeatOffsets,
    prepared: &GreedyPreparedBlock,
    context: GreedyBlockEncodeContext<'_, '_>,
    bytes: Vec<u8>,
) -> GreedyEncodedBlock {
    if prepared.prepared.sequences.is_empty()
        && literal_rle_byte(prepared.prepared.literals.as_slice()).is_some()
    {
        let mut candidate = bytes.clone();
        if let Some(emission) = append_literal_only_sub_block(
            prepared.prepared.literals.as_slice(),
            last_block,
            EntropyTableMode::Rle,
            basic_sequence_modes(),
            false,
            false,
            &mut candidate,
        ) {
            if should_commit_sub_block(emission.byte_size, block.len()) {
                return GreedyEncodedBlock {
                    bytes: candidate,
                    repeat_offsets,
                    new_huffman_table: None,
                };
            }
        }
    }

    if !prepared.prepared.sequences.is_empty() {
        let fse_tables = context.fse_tables;
        let offset_history = context.offset_history;
        if let Some(encoded) = try_sequence_sub_block(
            block,
            last_block,
            prepared,
            fse_tables,
            offset_history,
            &bytes,
            repeat_sequence_modes(),
        ) {
            return encoded;
        }
        if let Some(encoded) = try_sequence_sub_block(
            block,
            last_block,
            prepared,
            fse_tables,
            offset_history,
            &bytes,
            rle_sequence_modes(),
        ) {
            return encoded;
        }
        if let Some(encoded) = try_sequence_sub_block(
            block,
            last_block,
            prepared,
            fse_tables,
            offset_history,
            &bytes,
            compressed_sequence_modes(),
        ) {
            return encoded;
        }
        if let Some(encoded) = try_sequence_sub_block(
            block,
            last_block,
            prepared,
            fse_tables,
            offset_history,
            &bytes,
            basic_sequence_modes(),
        ) {
            return encoded;
        }
    }

    encode_target_block_raw_fallback(block, last_block, repeat_offsets, bytes)
}

fn try_sequence_sub_block(
    block: &[u8],
    last_block: bool,
    prepared: &GreedyPreparedBlock,
    fse_tables: &mut FseTables,
    offset_history: &mut OffsetHistory,
    bytes: &[u8],
    sequence_modes: SequenceEntropyModes,
) -> Option<GreedyEncodedBlock> {
    let previous_offsets = *offset_history;
    let previous_fse = fse_tables.snapshot_previous();
    let mut candidate = bytes.to_vec();
    let Some(emission) = append_sequence_sub_block(
        prepared.prepared.literals.as_slice(),
        prepared.prepared.sequences.as_slice(),
        last_block,
        EntropyTableMode::Basic,
        sequence_modes,
        true,
        true,
        fse_tables,
        offset_history,
        &mut candidate,
    ) else {
        fse_tables.restore_previous(previous_fse);
        *offset_history = previous_offsets;
        return None;
    };
    if should_commit_sub_block(emission.byte_size, block.len()) {
        if sequence_modes_clear_previous(sequence_modes) {
            fse_tables.reset();
        }
        Some(GreedyEncodedBlock {
            bytes: candidate,
            repeat_offsets: prepared.repeat_offsets,
            new_huffman_table: None,
        })
    } else {
        fse_tables.restore_previous(previous_fse);
        *offset_history = previous_offsets;
        None
    }
}

fn encode_target_block_raw_fallback(
    block: &[u8],
    last_block: bool,
    repeat_offsets: RepeatOffsets,
    mut bytes: Vec<u8>,
) -> GreedyEncodedBlock {
    append_raw_block(block, last_block, &mut bytes);
    GreedyEncodedBlock {
        bytes,
        repeat_offsets,
        new_huffman_table: None,
    }
}

fn literal_rle_byte(literals: &[u8]) -> Option<u8> {
    let first = *literals.first()?;
    literals.iter().all(|byte| *byte == first).then_some(first)
}

fn basic_sequence_modes() -> SequenceEntropyModes {
    SequenceEntropyModes {
        ll: EntropyTableMode::Basic,
        ml: EntropyTableMode::Basic,
        of: EntropyTableMode::Basic,
    }
}

fn rle_sequence_modes() -> SequenceEntropyModes {
    SequenceEntropyModes {
        ll: EntropyTableMode::Rle,
        ml: EntropyTableMode::Rle,
        of: EntropyTableMode::Rle,
    }
}

fn repeat_sequence_modes() -> SequenceEntropyModes {
    SequenceEntropyModes {
        ll: EntropyTableMode::Repeat,
        ml: EntropyTableMode::Repeat,
        of: EntropyTableMode::Repeat,
    }
}

fn compressed_sequence_modes() -> SequenceEntropyModes {
    SequenceEntropyModes {
        ll: EntropyTableMode::Compressed,
        ml: EntropyTableMode::Compressed,
        of: EntropyTableMode::Compressed,
    }
}

fn sequence_modes_clear_previous(modes: SequenceEntropyModes) -> bool {
    matches!(modes.ll, EntropyTableMode::Basic | EntropyTableMode::Rle)
        && matches!(modes.ml, EntropyTableMode::Basic | EntropyTableMode::Rle)
        && matches!(modes.of, EntropyTableMode::Basic | EntropyTableMode::Rle)
}
