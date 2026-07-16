//! Target-compressed-block-size block adapter.

use alloc::vec::Vec;

use super::{
    block_emit::append_raw_block,
    greedy_block::{GreedyBlockEncodeContext, GreedyEncodedBlock, GreedyPreparedBlock},
    sequence_store::RepeatOffsets,
    superblock::{
        append_basic_sub_block, append_literal_only_sub_block, should_commit_sub_block,
        EntropyTableMode, SequenceEntropyModes,
    },
};

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
        let previous_offsets = *context.offset_history;
        let mut candidate = bytes.clone();
        if let Some(emission) = append_basic_sub_block(
            prepared.prepared.literals.as_slice(),
            prepared.prepared.sequences.as_slice(),
            last_block,
            EntropyTableMode::Basic,
            basic_sequence_modes(),
            true,
            true,
            context.fse_tables,
            context.offset_history,
            &mut candidate,
        ) {
            if should_commit_sub_block(emission.byte_size, block.len()) {
                context.fse_tables.reset();
                return GreedyEncodedBlock {
                    bytes: candidate,
                    repeat_offsets: prepared.repeat_offsets,
                    new_huffman_table: None,
                };
            }
        }
        *context.offset_history = previous_offsets;
    }

    encode_target_block_raw_fallback(block, last_block, repeat_offsets, bytes)
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
