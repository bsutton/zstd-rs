//! Dictionary and prefix bounds for the C greedy/lazy match loop.

use super::{
    hash_chain_match::{count_match, lowest_prefix_index_with_loaded_dict, read32},
    params::CompressionParameters,
};

#[derive(Clone, Copy, Debug)]
pub(super) struct LazyDictionaryBounds {
    pub(super) dict_limit: usize,
    pub(super) prefix_start_index: usize,
    pub(super) loaded_dict_end: usize,
    pub(super) ext_dict: bool,
    dict_start_index: usize,
}

impl LazyDictionaryBounds {
    pub(super) fn no_dict(
        block_end: usize,
        params: CompressionParameters,
        loaded_dict_end: usize,
    ) -> Self {
        let prefix_start_index =
            lowest_prefix_index_with_loaded_dict(block_end, params.window_log, loaded_dict_end);
        Self {
            dict_limit: prefix_start_index,
            dict_start_index: prefix_start_index,
            prefix_start_index,
            loaded_dict_end,
            ext_dict: false,
        }
    }

    pub(super) fn ext_dict(
        block_end: usize,
        dict_limit: usize,
        params: CompressionParameters,
        loaded_dict_end: usize,
    ) -> Self {
        let dict_start_index =
            lowest_prefix_index_with_loaded_dict(block_end, params.window_log, loaded_dict_end);
        Self {
            dict_limit,
            dict_start_index,
            prefix_start_index: dict_limit.max(dict_start_index),
            loaded_dict_end,
            ext_dict: true,
        }
    }

    pub(super) fn low_match_index(self, match_index: usize) -> usize {
        if self.ext_dict && match_index < self.dict_limit {
            self.dict_start_index
        } else {
            self.prefix_start_index
        }
    }

    pub(super) fn rep_match_length(
        self,
        src: &[u8],
        current: usize,
        offset: usize,
        params: CompressionParameters,
        block_end: usize,
    ) -> Option<usize> {
        if offset == 0 || current < offset {
            return None;
        }
        let rep_index = current - offset;
        if self.ext_dict {
            let window_low = lowest_prefix_index_with_loaded_dict(
                current,
                params.window_log,
                self.loaded_dict_end,
            );
            if offset > current.saturating_sub(window_low)
                || !index_overlap_check(self.dict_limit, rep_index)
            {
                return None;
            }
        }
        if rep_index + 4 > src.len() || current + 4 > src.len() {
            return None;
        }
        (read32(src, rep_index) == read32(src, current))
            .then(|| count_match(src, current + 4, rep_index + 4, block_end) + 4)
    }
}

fn index_overlap_check(prefix_lowest_index: usize, rep_index: usize) -> bool {
    prefix_lowest_index.wrapping_sub(1).wrapping_sub(rep_index) >= 3
}
