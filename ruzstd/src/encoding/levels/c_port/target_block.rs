//! Target-compressed-block-size block adapter.

use alloc::vec::Vec;

use super::{
    block_emit::append_raw_block,
    greedy_block::{GreedyBlockEncodeContext, GreedyEncodedBlock, GreedyPreparedBlock},
    sequence_store::RepeatOffsets,
    superblock::{
        append_literal_only_sub_block, append_sequence_sub_block,
        append_supported_sub_block_sequences, should_commit_sub_block, EntropyTableMode,
        SequenceEntropyModes,
    },
    target_multi::{
        try_basic_literal_multi_sub_blocks, try_huffman_literal_multi_sub_blocks, TargetMultiBlock,
    },
};
use crate::{
    blocks::block::BlockType,
    encoding::{
        block_header::BlockHeader,
        blocks::{append_huffman_literal_section, HuffmanLiteralMode},
        frame_compressor::{FseTables, OffsetHistory},
    },
    huff0::huff0_encoder::HuffmanTable,
};

const BLOCK_HEADER_SIZE: usize = 3;

pub(super) fn encode_target_block_with_superblock_fallback(
    block: &[u8],
    last_block: bool,
    target_c_block_size: usize,
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
        let previous_huff_table = context.previous_huff_table;
        let fse_tables = context.fse_tables;
        let offset_history = context.offset_history;
        let multi_target = TargetMultiBlock {
            block,
            last_block,
            target_c_block_size,
            initial_repeat_offsets: repeat_offsets,
            bytes: &bytes,
        };
        if let Some(encoded) =
            try_huffman_literal_multi_sub_blocks(multi_target, prepared, fse_tables, offset_history)
        {
            return encoded;
        }
        if let Some(encoded) =
            try_basic_literal_multi_sub_blocks(multi_target, prepared, fse_tables, offset_history)
        {
            return encoded;
        }
        if let Some(encoded) = try_huffman_sequence_sub_block(
            block,
            last_block,
            prepared,
            previous_huff_table,
            fse_tables,
            offset_history,
            &bytes,
            HuffmanLiteralMode::Repeat,
            repeat_sequence_modes(),
        ) {
            return encoded;
        }
        if let Some(encoded) = try_huffman_sequence_sub_block(
            block,
            last_block,
            prepared,
            previous_huff_table,
            fse_tables,
            offset_history,
            &bytes,
            HuffmanLiteralMode::Compressed,
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

#[allow(clippy::too_many_arguments)]
fn try_huffman_sequence_sub_block(
    block: &[u8],
    last_block: bool,
    prepared: &GreedyPreparedBlock,
    previous_huff_table: Option<&HuffmanTable>,
    fse_tables: &mut FseTables,
    offset_history: &mut OffsetHistory,
    bytes: &[u8],
    literal_mode: HuffmanLiteralMode,
    sequence_modes: SequenceEntropyModes,
) -> Option<GreedyEncodedBlock> {
    let previous_offsets = *offset_history;
    let previous_fse = fse_tables.snapshot_previous();
    let mut candidate = bytes.to_vec();
    let block_start = candidate.len();
    candidate.extend_from_slice(&[0; BLOCK_HEADER_SIZE]);
    let content_start = candidate.len();
    let Some(literal_emission) = append_huffman_literal_section(
        prepared.prepared.literals.as_slice(),
        previous_huff_table,
        literal_mode,
        &mut candidate,
    ) else {
        fse_tables.restore_previous(previous_fse);
        *offset_history = previous_offsets;
        return None;
    };
    let Some(sequence_emission) = append_supported_sub_block_sequences(
        prepared.prepared.sequences.as_slice(),
        sequence_modes,
        true,
        fse_tables,
        offset_history,
        &mut candidate,
    ) else {
        fse_tables.restore_previous(previous_fse);
        *offset_history = previous_offsets;
        return None;
    };

    let content_size = candidate.len() - content_start;
    debug_assert_eq!(
        literal_emission.byte_size + sequence_emission.byte_size,
        content_size
    );
    let header = BlockHeader {
        last_block,
        block_type: BlockType::Compressed,
        block_size: content_size as u32,
    };
    candidate[block_start..content_start].copy_from_slice(&header.serialize_to_bytes());

    if should_commit_sub_block(BLOCK_HEADER_SIZE + content_size, block.len()) {
        if sequence_modes_clear_previous(sequence_modes) {
            fse_tables.reset();
        }
        Some(GreedyEncodedBlock {
            bytes: candidate,
            repeat_offsets: prepared.repeat_offsets,
            new_huffman_table: literal_emission.new_huffman_table,
        })
    } else {
        fse_tables.restore_previous(previous_fse);
        *offset_history = previous_offsets;
        None
    }
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
