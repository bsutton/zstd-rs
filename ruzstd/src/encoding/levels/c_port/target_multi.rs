//! Multi-sub-block target-compressed-size adapter.

use alloc::vec::Vec;

use super::{
    greedy_block::{GreedyEncodedBlock, GreedyPreparedBlock},
    superblock::{
        append_sequence_sub_block, count_literals, decompressed_size, should_commit_sub_block,
        size_block_sequences, sub_block_budget_plan, EntropyTableMode, EstimatedSubBlockSize,
        SequenceEntropyModes,
    },
};
use crate::encoding::frame_compressor::{FseTables, OffsetHistory};

pub(super) fn try_basic_literal_multi_sub_blocks(
    block: &[u8],
    last_block: bool,
    target_c_block_size: usize,
    prepared: &GreedyPreparedBlock,
    fse_tables: &mut FseTables,
    offset_history: &mut OffsetHistory,
    bytes: &[u8],
) -> Option<GreedyEncodedBlock> {
    let estimate = estimate_basic_literal_sequence_block(
        prepared,
        fse_tables,
        *offset_history,
        compressed_sequence_modes(),
    )?;
    let plan = sub_block_budget_plan(
        estimate,
        prepared.prepared.literals.len(),
        prepared.prepared.sequences.len(),
        target_c_block_size,
        block.len(),
    )?;
    if plan.nb_sub_blocks <= 1 {
        return None;
    }

    let previous_offsets = *offset_history;
    let previous_fse = fse_tables.snapshot_previous();
    let mut candidate = bytes.to_vec();
    let mut literal_pos = 0usize;
    let mut start_sequence = 0usize;
    let mut decompressed_pos = 0usize;
    let mut sequence_entropy_written = false;

    for block_idx in 0..plan.nb_sub_blocks.saturating_sub(1) {
        let remaining = &prepared.prepared.sequences[start_sequence..];
        if remaining.is_empty() {
            break;
        }

        let sequence_count = size_block_sequences(
            remaining,
            plan.avg_block_budget,
            plan.avg_lit_cost,
            plan.avg_seq_cost,
            block_idx == 0,
        );
        let end_sequence = start_sequence + sequence_count;
        if end_sequence == prepared.prepared.sequences.len() {
            break;
        }

        let sequences = &prepared.prepared.sequences[start_sequence..end_sequence];
        let literal_size = count_literals(sequences);
        let decompressed_size = decompressed_size(sequences, literal_size, false);
        let Some(literals) = prepared
            .prepared
            .literals
            .get(literal_pos..literal_pos + literal_size)
        else {
            break;
        };
        let write_sequence_entropy = !sequence_entropy_written;
        let sequence_modes = if write_sequence_entropy {
            compressed_sequence_modes()
        } else {
            repeat_sequence_modes()
        };
        let output_start = candidate.len();
        let offsets_before_sub_block = *offset_history;
        let fse_before_sub_block = fse_tables.snapshot_previous();
        let Some(emission) = append_sequence_sub_block(
            literals,
            sequences,
            false,
            EntropyTableMode::Basic,
            sequence_modes,
            false,
            write_sequence_entropy,
            fse_tables,
            offset_history,
            &mut candidate,
        ) else {
            candidate.truncate(output_start);
            fse_tables.restore_previous(fse_before_sub_block);
            *offset_history = offsets_before_sub_block;
            continue;
        };
        if should_commit_sub_block(emission.byte_size, decompressed_size) {
            literal_pos += literal_size;
            start_sequence = end_sequence;
            decompressed_pos += decompressed_size;
            if emission.sequence_entropy_written {
                sequence_entropy_written = true;
            }
        } else {
            candidate.truncate(output_start);
            fse_tables.restore_previous(fse_before_sub_block);
            *offset_history = offsets_before_sub_block;
        }
    }

    if start_sequence == 0 {
        fse_tables.restore_previous(previous_fse);
        *offset_history = previous_offsets;
        return None;
    }

    let remaining_sequences = &prepared.prepared.sequences[start_sequence..];
    let remaining_literals = prepared.prepared.literals.get(literal_pos..)?;
    let remaining_decompressed = block.len().checked_sub(decompressed_pos)?;
    let write_sequence_entropy = !sequence_entropy_written;
    let sequence_modes = if write_sequence_entropy {
        compressed_sequence_modes()
    } else {
        repeat_sequence_modes()
    };
    let output_start = candidate.len();
    let Some(emission) = append_sequence_sub_block(
        remaining_literals,
        remaining_sequences,
        last_block,
        EntropyTableMode::Basic,
        sequence_modes,
        false,
        write_sequence_entropy,
        fse_tables,
        offset_history,
        &mut candidate,
    ) else {
        candidate.truncate(output_start);
        fse_tables.restore_previous(previous_fse);
        *offset_history = previous_offsets;
        return None;
    };
    if !should_commit_sub_block(emission.byte_size, remaining_decompressed) {
        candidate.truncate(output_start);
        fse_tables.restore_previous(previous_fse);
        *offset_history = previous_offsets;
        return None;
    }

    Some(GreedyEncodedBlock {
        bytes: candidate,
        repeat_offsets: prepared.repeat_offsets,
        new_huffman_table: None,
    })
}

fn estimate_basic_literal_sequence_block(
    prepared: &GreedyPreparedBlock,
    fse_tables: &FseTables,
    offset_history: OffsetHistory,
    sequence_modes: SequenceEntropyModes,
) -> Option<EstimatedSubBlockSize> {
    let mut fse_tables = fse_tables.clone();
    let mut offset_history = offset_history;
    let mut output = Vec::new();
    let emission = append_sequence_sub_block(
        prepared.prepared.literals.as_slice(),
        prepared.prepared.sequences.as_slice(),
        true,
        EntropyTableMode::Basic,
        sequence_modes,
        false,
        true,
        &mut fse_tables,
        &mut offset_history,
        &mut output,
    )?;
    Some(EstimatedSubBlockSize {
        literal_size: prepared.prepared.literals.len(),
        block_size: emission.byte_size,
    })
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
