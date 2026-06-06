//! Scalar row-based match finder ported from the no-dictionary row path in
//! `zstd_lazy.c`.

use core::convert::TryInto;

use super::{greedy::GreedyMatchState, params::CompressionParameters, sequence_store::OffBase};

const TAG_BITS: u32 = 8;
const TAG_MASK: u32 = (1 << TAG_BITS) - 1;
const SKIP_THRESHOLD: usize = 384;
const MAX_MATCH_START_POSITIONS_TO_UPDATE: usize = 96;
const MAX_MATCH_END_POSITIONS_TO_UPDATE: usize = 32;

pub(super) fn row_find_best_match(
    src: &[u8],
    ip: usize,
    block_end: usize,
    off_base: &mut u32,
    params: CompressionParameters,
    min_match: u32,
    state: &mut GreedyMatchState,
) -> usize {
    let row_log = row_log(params);
    let row_entries = 1usize << row_log;
    let row_mask = row_entries - 1;
    let row_hash_log = params.hash_log - row_log;
    let max_attempts = 1usize << params.search_log.min(row_log);
    let curr = ip;
    let max_distance = 1usize << params.window_log;
    let low_limit = curr.saturating_sub(max_distance);

    let hash = if state.lazy_skipping {
        state.next_to_update = curr;
        hash_ptr_salted(
            src,
            curr,
            row_hash_log + TAG_BITS,
            min_match,
            state.hash_salt,
        )
    } else {
        update_rows(src, curr, params, min_match, state);
        hash_ptr_salted(
            src,
            curr,
            row_hash_log + TAG_BITS,
            min_match,
            state.hash_salt,
        )
    };

    state.hash_salt_entropy = state.hash_salt_entropy.wrapping_add(hash);

    let row_start = ((hash >> TAG_BITS) as usize) << row_log;
    let tag = (hash & TAG_MASK) as u8;
    let head = usize::from(state.tag_table[row_start] & row_mask as u8);
    let mut matches = [0usize; 64];
    let mut match_count = 0usize;

    for step in 0..row_entries {
        if match_count == max_attempts {
            break;
        }
        let pos = (head + step) & row_mask;
        if pos == 0 || state.tag_table[row_start + pos] != tag {
            continue;
        }

        let match_index = state.hash_table[row_start + pos] as usize;
        if match_index < low_limit {
            break;
        }
        if match_index < curr {
            matches[match_count] = match_index;
            match_count += 1;
        }
    }

    let insert_pos = next_row_index(&mut state.tag_table[row_start], row_mask);
    state.tag_table[row_start + insert_pos] = tag;
    state.hash_table[row_start + insert_pos] = state.next_to_update as u32;
    state.next_to_update += 1;

    let mut best_len = 3usize;
    for &match_index in matches.iter().take(match_count) {
        let mut current_len = 0usize;
        if read32(src, match_index + best_len - 3) == read32(src, ip + best_len - 3) {
            current_len = super::hash_chain_match::count_match(src, ip, match_index, block_end);
        }

        if current_len > best_len {
            best_len = current_len;
            *off_base = OffBase::from_offset((curr - match_index) as u32)
                .expect("row match has non-zero offset")
                .to_c_value();
            if ip + current_len == block_end {
                break;
            }
        }
    }

    best_len
}

fn update_rows(
    src: &[u8],
    target: usize,
    params: CompressionParameters,
    min_match: u32,
    state: &mut GreedyMatchState,
) {
    update_rows_internal(src, target, params, min_match, state, true);
}

pub(super) fn load_dictionary_rows(
    src: &[u8],
    target: usize,
    params: CompressionParameters,
    min_match: u32,
    state: &mut GreedyMatchState,
) {
    update_rows_internal(src, target, params, min_match, state, false);
}

fn update_rows_internal(
    src: &[u8],
    target: usize,
    params: CompressionParameters,
    min_match: u32,
    state: &mut GreedyMatchState,
    use_cache_skip: bool,
) {
    let row_log = row_log(params);
    let row_mask = (1usize << row_log) - 1;
    let row_hash_log = params.hash_log - row_log;
    let mut idx = state.next_to_update;

    if use_cache_skip && target.saturating_sub(idx) > SKIP_THRESHOLD {
        let start_bound = idx + MAX_MATCH_START_POSITIONS_TO_UPDATE;
        update_rows_range(
            src,
            idx,
            start_bound,
            row_hash_log,
            row_log,
            row_mask,
            min_match,
            state,
        );
        idx = target - MAX_MATCH_END_POSITIONS_TO_UPDATE;
    }

    update_rows_range(
        src,
        idx,
        target,
        row_hash_log,
        row_log,
        row_mask,
        min_match,
        state,
    );
    state.next_to_update = target;
}

#[allow(clippy::too_many_arguments)]
fn update_rows_range(
    src: &[u8],
    mut idx: usize,
    target: usize,
    row_hash_log: u32,
    row_log: u32,
    row_mask: usize,
    min_match: u32,
    state: &mut GreedyMatchState,
) {
    while idx < target {
        let hash = hash_ptr_salted(
            src,
            idx,
            row_hash_log + TAG_BITS,
            min_match,
            state.hash_salt,
        );
        let row_start = ((hash >> TAG_BITS) as usize) << row_log;
        let pos = next_row_index(&mut state.tag_table[row_start], row_mask);
        state.tag_table[row_start + pos] = (hash & TAG_MASK) as u8;
        state.hash_table[row_start + pos] = idx as u32;
        idx += 1;
    }
}

fn next_row_index(head: &mut u8, row_mask: usize) -> usize {
    let mut next = usize::from(head.wrapping_sub(1)) & row_mask;
    if next == 0 {
        next = row_mask;
    }
    *head = next as u8;
    next
}

pub(super) fn row_log(params: CompressionParameters) -> u32 {
    params.search_log.clamp(4, 6)
}

pub(super) fn row_match_finder_enabled(params: CompressionParameters) -> bool {
    matches!(
        params.strategy,
        super::params::Strategy::Greedy
            | super::params::Strategy::Lazy
            | super::params::Strategy::Lazy2
    ) && params.window_log > 14
}

fn hash_ptr_salted(src: &[u8], pos: usize, h_bits: u32, min_match: u32, salt: u64) -> u32 {
    match min_match {
        5 => hash5(read64(src, pos), h_bits, salt),
        6 => hash6(read64(src, pos), h_bits, salt),
        _ => hash4(read32(src, pos), h_bits, salt as u32),
    }
}

fn hash4(value: u32, h_bits: u32, salt: u32) -> u32 {
    const PRIME_4_BYTES: u32 = 2_654_435_761;
    (value.wrapping_mul(PRIME_4_BYTES) ^ salt).wrapping_shr(32 - h_bits)
}

fn hash5(value: u64, h_bits: u32, salt: u64) -> u32 {
    const PRIME_5_BYTES: u64 = 889_523_592_379;
    (((value << (64 - 40)).wrapping_mul(PRIME_5_BYTES) ^ salt) >> (64 - h_bits)) as u32
}

fn hash6(value: u64, h_bits: u32, salt: u64) -> u32 {
    const PRIME_6_BYTES: u64 = 227_718_039_650_203;
    (((value << (64 - 48)).wrapping_mul(PRIME_6_BYTES) ^ salt) >> (64 - h_bits)) as u32
}

fn read32(src: &[u8], pos: usize) -> u32 {
    u32::from_le_bytes(src[pos..pos + 4].try_into().expect("read32 in bounds"))
}

fn read64(src: &[u8], pos: usize) -> u64 {
    u64::from_le_bytes(src[pos..pos + 8].try_into().expect("read64 in bounds"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoding::levels::c_port::params::Strategy;
    use alloc::vec;

    fn params() -> CompressionParameters {
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
    fn row_next_index_cycles_backwards_and_skips_zero() {
        let mut head = 0u8;

        assert_eq!(next_row_index(&mut head, 15), 15);
        assert_eq!(head, 15);
        assert_eq!(next_row_index(&mut head, 15), 14);
    }

    #[test]
    fn row_finder_reports_previous_match() {
        let data = b"abcdefghabcdefgh-tail";
        let mut state = GreedyMatchState::new();
        let params = params();
        state.ensure_tables(params);
        let mut off_base = 0;

        let match_len =
            row_find_best_match(data, 8, data.len(), &mut off_base, params, 4, &mut state);

        assert!(match_len >= 8);
        assert_eq!(off_base, 11);
    }

    #[test]
    fn row_finder_uses_c_userspace_window_gate() {
        let mut params = params();
        assert!(row_match_finder_enabled(params));
        params.window_log = 15;
        assert!(row_match_finder_enabled(params));
        params.window_log = 14;
        assert!(!row_match_finder_enabled(params));
    }

    #[test]
    fn row_update_skips_middle_of_large_gaps_like_c() {
        let mut data = vec![0u8; 540];
        for (idx, byte) in data.iter_mut().enumerate() {
            *byte = (idx.wrapping_mul(37) & 0xFF) as u8;
        }
        let pattern = b"abcdefghijklmnopqrstuvwxyz";
        data[200..200 + pattern.len()].copy_from_slice(pattern);
        data[500..500 + pattern.len()].copy_from_slice(pattern);

        let mut state = GreedyMatchState::new();
        let params = params();
        state.ensure_tables(params);
        let mut off_base = 0;

        let match_len =
            row_find_best_match(&data, 500, data.len(), &mut off_base, params, 4, &mut state);

        assert_eq!(match_len, 3);
        assert_eq!(off_base, 0);
    }
}
