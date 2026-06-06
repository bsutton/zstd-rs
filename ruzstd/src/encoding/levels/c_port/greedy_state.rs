use alloc::vec::Vec;

use super::params::CompressionParameters;
use super::row_match::{row_log, row_match_finder_enabled};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GreedyMatchState {
    pub(super) hash_table: Vec<u32>,
    pub(super) hash_table3: Vec<u32>,
    pub(super) chain_table: Vec<u32>,
    pub(super) hash_log: u32,
    pub(super) hash_log3: u32,
    pub(super) chain_log: u32,
    pub(super) row_log: u32,
    pub(super) next_to_update: usize,
    pub(super) next_to_update3: usize,
    pub(super) lazy_skipping: bool,
    pub(super) tag_table: Vec<u8>,
    pub(super) hash_salt: u64,
    pub(super) hash_salt_entropy: u32,
}

impl GreedyMatchState {
    pub(crate) fn new() -> Self {
        Self {
            hash_table: Vec::new(),
            hash_table3: Vec::new(),
            chain_table: Vec::new(),
            hash_log: 0,
            hash_log3: 0,
            chain_log: 0,
            row_log: 0,
            next_to_update: 0,
            next_to_update3: 0,
            lazy_skipping: false,
            tag_table: Vec::new(),
            hash_salt: 0,
            hash_salt_entropy: 0,
        }
    }

    pub(crate) fn reset_for_frame(&mut self, params: CompressionParameters) {
        self.next_to_update = 0;
        self.next_to_update3 = 0;
        self.lazy_skipping = false;
        self.hash_table.fill(0);
        self.hash_table3.fill(0);
        self.chain_table.fill(0);
        if row_match_finder_enabled(params) {
            self.advance_hash_salt();
        } else {
            self.hash_salt = 0;
        }
    }

    pub(super) fn ensure_tables(&mut self, params: CompressionParameters) {
        if self.hash_log != params.hash_log {
            self.hash_log = params.hash_log;
            self.hash_table.clear();
            self.next_to_update = 0;
        }
        let hash_log3 = if params.min_match == 3 {
            params.window_log.min(17)
        } else {
            0
        };
        if self.hash_log3 != hash_log3 {
            self.hash_log3 = hash_log3;
            self.hash_table3.clear();
            self.next_to_update3 = 0;
        }
        if self.chain_log != params.chain_log {
            self.chain_log = params.chain_log;
            self.chain_table.clear();
            self.next_to_update = 0;
        }

        let hash_size = 1_usize << params.hash_log;
        if self.hash_table.len() != hash_size {
            self.hash_table.resize(hash_size, 0);
        }

        let hash3_size = if self.hash_log3 > 0 {
            1_usize << self.hash_log3
        } else {
            0
        };
        if self.hash_table3.len() != hash3_size {
            self.hash_table3.resize(hash3_size, 0);
        }

        let chain_size = 1_usize << params.chain_log;
        if self.chain_table.len() != chain_size {
            self.chain_table.resize(chain_size, 0);
        }

        let row_log = row_log(params);
        if self.row_log != row_log {
            self.row_log = row_log;
            self.tag_table.clear();
            self.next_to_update = 0;
        }
        if row_match_finder_enabled(params) && self.tag_table.len() != hash_size {
            self.tag_table.resize(hash_size, 0);
        }
    }

    fn advance_hash_salt(&mut self) {
        self.hash_salt = bitmix(self.hash_salt, 8) ^ bitmix(u64::from(self.hash_salt_entropy), 4);
    }
}

fn bitmix(mut value: u64, len: u64) -> u64 {
    const PRIME: u64 = 0x9FB2_1C65_1E98_DF25;
    value ^= value.rotate_right(49) ^ value.rotate_right(24);
    value = value.wrapping_mul(PRIME);
    value ^= (value >> 35).wrapping_add(len);
    value = value.wrapping_mul(PRIME);
    value ^ (value >> 28)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoding::levels::c_port::params::Strategy;

    fn row_params() -> CompressionParameters {
        CompressionParameters {
            window_log: 18,
            chain_log: 16,
            hash_log: 16,
            search_log: 5,
            min_match: 4,
            target_length: 0,
            strategy: Strategy::Greedy,
        }
    }

    #[test]
    fn reset_for_frame_advances_row_hash_salt() {
        let mut state = GreedyMatchState::new();
        let params = row_params();

        state.reset_for_frame(params);
        let first_salt = state.hash_salt;
        state.hash_salt_entropy = 12345;
        state.reset_for_frame(params);

        assert_ne!(first_salt, 0);
        assert_ne!(state.hash_salt, first_salt);
    }

    #[test]
    fn reset_for_frame_clears_indexes_but_keeps_allocations() {
        let params = row_params();
        let mut state = GreedyMatchState::new();
        state.ensure_tables(params);
        state.hash_table[3] = 99;
        state.hash_table3.resize(8, 7);
        state.chain_table[4] = 88;
        state.next_to_update = 42;
        state.next_to_update3 = 24;
        let hash_capacity = state.hash_table.capacity();

        state.reset_for_frame(params);

        assert_eq!(state.next_to_update, 0);
        assert_eq!(state.next_to_update3, 0);
        assert!(state.hash_table.iter().all(|&index| index == 0));
        assert!(state.hash_table3.iter().all(|&index| index == 0));
        assert!(state.chain_table.iter().all(|&index| index == 0));
        assert_eq!(state.hash_table.capacity(), hash_capacity);
    }
}
