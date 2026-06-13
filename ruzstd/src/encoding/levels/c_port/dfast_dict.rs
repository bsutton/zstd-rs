//! Dictionary prefix loading for the double-fast match state.

use super::{
    dfast::{hash_ptr, DFastMatchState, HASH_READ_SIZE},
    params::CompressionParameters,
};

pub(crate) fn load_prefix(
    state: &mut DFastMatchState,
    src: &[u8],
    prefix_len: usize,
    params: CompressionParameters,
) {
    debug_assert!(prefix_len <= src.len());
    if prefix_len <= HASH_READ_SIZE {
        return;
    }

    state.ensure_tables(params);
    let iend = prefix_len - HASH_READ_SIZE;
    let mut ip = 0_usize;

    while ip + 2 <= iend {
        let hash_small = hash_ptr(src, ip, params.chain_log, params.min_match);
        let hash_long = hash_ptr(src, ip, params.hash_log, 8);
        state.hash_small[hash_small] = ip as u32;
        state.hash_long[hash_long] = ip as u32;
        ip += 3;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoding::levels::c_port::params::Strategy;

    #[test]
    fn double_fast_prefix_loader_fills_every_third_position_like_c_fast_load() {
        let data = b"abcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxy";
        let params = params();
        let mut state = DFastMatchState::new();

        load_prefix(&mut state, data, data.len(), params);

        let hash0_small = hash_ptr(data, 0, params.chain_log, params.min_match);
        let hash3_small = hash_ptr(data, 3, params.chain_log, params.min_match);
        let hash1_small = hash_ptr(data, 1, params.chain_log, params.min_match);
        let hash0_long = hash_ptr(data, 0, params.hash_log, 8);
        let hash3_long = hash_ptr(data, 3, params.hash_log, 8);

        assert_eq!(state.hash_small[hash0_small], 0);
        assert_eq!(state.hash_small[hash3_small], 3);
        assert_eq!(state.hash_small[hash1_small], 0);
        assert_eq!(state.hash_long[hash0_long], 0);
        assert_eq!(state.hash_long[hash3_long], 3);
    }

    fn params() -> CompressionParameters {
        CompressionParameters {
            window_log: 17,
            chain_log: 12,
            hash_log: 13,
            search_log: 1,
            min_match: 5,
            target_length: 0,
            strategy: Strategy::DFast,
        }
    }
}
