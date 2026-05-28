use crate::{
    common::MAX_BLOCK_SIZE,
    encoding::{
        block_header::BlockHeader, blocks::compress_block, frame_compressor::CompressState, Matcher,
    },
};
use alloc::vec::Vec;

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
    let block_size = uncompressed_data.len() as u32;
    // First check to see if run length encoding can be used for the entire block
    if uncompressed_data.iter().all(|x| uncompressed_data[0].eq(x)) {
        let rle_byte = uncompressed_data[0];
        state.matcher.commit_space(uncompressed_data);
        state.matcher.skip_matching();
        let header = BlockHeader {
            last_block,
            block_type: crate::blocks::block::BlockType::RLE,
            block_size,
        };
        // Write the header, then the block
        header.serialize(output);
        output.push(rle_byte);
    } else if likely_incompressible(&uncompressed_data) {
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
        let mut compressed = Vec::new();
        state.matcher.commit_space(uncompressed_data);
        let previous_ll = state.fse_tables.ll_previous.clone();
        let previous_ml = state.fse_tables.ml_previous.clone();
        let previous_of = state.fse_tables.of_previous.clone();
        let previous_offsets = state.offset_history;
        let new_huffman_table = compress_block(state, &mut compressed);
        let compressed_size = compressed.len();
        // If compression does not shrink the block, store it raw instead.
        // Also preserve the format guard that compressed blocks must not
        // exceed the maximum block size.
        if compressed_size >= block_size as usize || compressed_size > MAX_BLOCK_SIZE as usize {
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
            if let Some(table) = new_huffman_table {
                state.last_huff_table = Some(table);
            }
            // Write the header, then the block
            header.serialize(output);
            output.extend(compressed);
        }
    }
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

pub(super) fn likely_incompressible(data: &[u8]) -> bool {
    const MIN_MATCH_LEN: usize = 5;
    const SAMPLE_COUNT: usize = 256;

    if data.len() < 8 * 1024 {
        return false;
    }

    let max_start = data.len() - MIN_MATCH_LEN;
    let samples = SAMPLE_COUNT.min(max_start + 1);
    let step = (max_start / samples).max(1);
    let mut keys = [0u64; SAMPLE_COUNT];
    for (used, sample) in (0..samples).enumerate() {
        let pos = (sample * step).min(max_start);
        let key = u64::from(data[pos])
            | (u64::from(data[pos + 1]) << 8)
            | (u64::from(data[pos + 2]) << 16)
            | (u64::from(data[pos + 3]) << 24)
            | (u64::from(data[pos + 4]) << 32);
        if keys[..used].contains(&key) {
            return false;
        }
        keys[used] = key;
    }

    true
}
