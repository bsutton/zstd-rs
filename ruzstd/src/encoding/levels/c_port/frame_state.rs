//! Shared state for C-style frame block emission.

use super::{
    block_policy::BlockEncodingPolicy,
    dictionary::ParsedDictionary,
    params::{CompressionParameters, Strategy},
    pre_split::FrameProgress,
    sequence_store::RepeatOffsets,
};
use crate::{
    common::MAX_BLOCK_SIZE,
    encoding::{
        blocks::BlockCompressionConfig,
        frame_compressor::{FseTables, OffsetHistory},
    },
    huff0::huff0_encoder::HuffmanTable,
};

pub(crate) struct FrameBlockState {
    pub(crate) fse_tables: FseTables,
    pub(crate) offset_history: OffsetHistory,
    pub(crate) last_huff_table: Option<HuffmanTable>,
    pub(crate) repeat_offsets: RepeatOffsets,
    pub(crate) block_config: BlockCompressionConfig,
    progress: FrameProgress,
}

impl FrameBlockState {
    pub(crate) fn new(params: CompressionParameters, frame_header_len: usize) -> Self {
        let block_config = block_config(params);
        Self {
            fse_tables: FseTables::new(),
            offset_history: OffsetHistory::new(),
            last_huff_table: None,
            repeat_offsets: RepeatOffsets::new(),
            block_config,
            progress: FrameProgress::new(frame_header_len),
        }
    }

    pub(crate) fn with_dictionary(
        params: CompressionParameters,
        frame_header_len: usize,
        dictionary: &ParsedDictionary<'_>,
    ) -> Self {
        let offsets = dictionary.repeat_offsets.as_offsets();
        let block_config = block_config(params);
        Self {
            fse_tables: dictionary.initial_fse_tables(),
            offset_history: OffsetHistory::from_offsets(offsets[0], offsets[1], offsets[2]),
            last_huff_table: dictionary.initial_huffman_table(),
            repeat_offsets: dictionary.repeat_offsets,
            block_config,
            progress: FrameProgress::new(frame_header_len),
        }
    }

    pub(crate) fn next_block_size(&mut self, remaining: &[u8], strategy: Strategy) -> usize {
        self.progress.next_block_size(remaining, strategy)
    }

    pub(crate) fn next_frame_chunk_block_size(
        &mut self,
        remaining: &[u8],
        source_offset: usize,
        strategy: Strategy,
    ) -> usize {
        // C's streaming path feeds compression in block-sized frame chunks, so
        // pre-splitting can only subdivide the current 128 KiB chunk once.
        let chunk_remaining = MAX_BLOCK_SIZE as usize - (source_offset % MAX_BLOCK_SIZE as usize);
        let visible_remaining = remaining.len().min(chunk_remaining);
        self.next_block_size(&remaining[..visible_remaining], strategy)
    }

    pub(crate) fn block_policy(first_block: bool) -> BlockEncodingPolicy {
        if first_block {
            BlockEncodingPolicy::frame_first_block()
        } else {
            BlockEncodingPolicy::normal()
        }
    }

    pub(crate) fn record_encoded_block(
        &mut self,
        block_size: usize,
        encoded_size: usize,
        repeat_offsets: RepeatOffsets,
        new_huffman_table: Option<HuffmanTable>,
    ) {
        self.repeat_offsets = repeat_offsets;
        if let Some(table) = new_huffman_table {
            self.last_huff_table = Some(table);
        }
        self.progress.record_block(block_size, encoded_size);
    }
}

fn block_config(params: CompressionParameters) -> BlockCompressionConfig {
    let mut config = BlockCompressionConfig::for_c_strategy(params.strategy as u8);
    if params.strategy == Strategy::Fast && params.target_length > 0 {
        config.disable_literal_compression();
    }
    config
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params(strategy: Strategy, target_length: u32) -> CompressionParameters {
        CompressionParameters {
            window_log: 19,
            chain_log: 12,
            hash_log: 13,
            search_log: 1,
            min_match: 6,
            target_length,
            strategy,
        }
    }

    #[test]
    fn c_fast_target_length_disables_literal_compression_like_auto_mode() {
        let config = block_config(params(Strategy::Fast, 4));

        assert!(config.literal_compression_disabled());
    }

    #[test]
    fn c_fast_zero_target_length_keeps_literal_compression_enabled() {
        let config = block_config(params(Strategy::Fast, 0));

        assert!(!config.literal_compression_disabled());
    }

    #[test]
    fn c_non_fast_target_length_keeps_literal_compression_enabled() {
        let config = block_config(params(Strategy::DFast, 4));

        assert!(!config.literal_compression_disabled());
    }
}
