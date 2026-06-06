//! Hash-chain search primitives shared by the C greedy/lazy/lazy2 ports.

use core::convert::TryInto;

use super::{greedy::GreedyMatchState, params::CompressionParameters, sequence_store::OffBase};

pub(super) fn hc_find_best_match(
    src: &[u8],
    ip: usize,
    block_end: usize,
    off_base: &mut u32,
    params: CompressionParameters,
    min_match: u32,
    state: &mut GreedyMatchState,
) -> usize {
    let chain_size = 1_usize << params.chain_log;
    let chain_mask = chain_size - 1;
    let curr = ip;
    let max_distance = 1_usize << params.window_log;
    let low_limit = curr.saturating_sub(max_distance);
    let min_chain = curr.saturating_sub(chain_size);
    let mut attempts = 1_usize << params.search_log;
    let mut ml = 3_usize;
    let mut match_index = insert_and_find_first_index(src, ip, params, min_match, state);

    while match_index >= low_limit && attempts > 0 {
        attempts -= 1;
        let current_ml = if read32(src, match_index + ml - 3) == read32(src, ip + ml - 3) {
            count_match(src, ip, match_index, block_end)
        } else {
            0
        };

        if current_ml > ml {
            ml = current_ml;
            *off_base = OffBase::from_offset((curr - match_index) as u32)
                .expect("hash-chain match has non-zero offset")
                .to_c_value();
            if ip + current_ml == block_end {
                break;
            }
        }

        if match_index <= min_chain {
            break;
        }
        match_index = state.chain_table[match_index & chain_mask] as usize;
    }

    ml
}

fn insert_and_find_first_index(
    src: &[u8],
    ip: usize,
    params: CompressionParameters,
    min_match: u32,
    state: &mut GreedyMatchState,
) -> usize {
    let chain_mask = (1_usize << params.chain_log) - 1;
    let mut idx = state.next_to_update;

    while idx < ip {
        let hash = hash_ptr(src, idx, params.hash_log, min_match);
        state.chain_table[idx & chain_mask] = state.hash_table[hash];
        state.hash_table[hash] = idx as u32;
        idx += 1;
        if state.lazy_skipping {
            break;
        }
    }

    state.next_to_update = ip;
    state.hash_table[hash_ptr(src, ip, params.hash_log, min_match)] as usize
}

pub(super) fn count_match(
    src: &[u8],
    mut pos: usize,
    mut match_pos: usize,
    match_limit: usize,
) -> usize {
    let start = pos;
    while pos + 8 <= match_limit && read64(src, pos) == read64(src, match_pos) {
        pos += 8;
        match_pos += 8;
    }
    while pos < match_limit && src[pos] == src[match_pos] {
        pos += 1;
        match_pos += 1;
    }
    pos - start
}

pub(super) fn hash_ptr(src: &[u8], pos: usize, h_bits: u32, min_match: u32) -> usize {
    match min_match {
        5 => hash5(read64(src, pos), h_bits),
        6 => hash6(read64(src, pos), h_bits),
        _ => hash4(read32(src, pos), h_bits),
    }
}

pub(super) fn hash3_ptr(src: &[u8], pos: usize, h_bits: u32) -> usize {
    const PRIME_3_BYTES: u32 = 506_832_829;
    (read32(src, pos) << 8)
        .wrapping_mul(PRIME_3_BYTES)
        .wrapping_shr(32 - h_bits) as usize
}

pub(super) fn equal_min_match(src: &[u8], left: usize, right: usize, min_match: u32) -> bool {
    let len = min_match as usize;
    src[left..left + len] == src[right..right + len]
}

pub(super) fn read32(src: &[u8], pos: usize) -> u32 {
    u32::from_le_bytes(src[pos..pos + 4].try_into().expect("read32 in bounds"))
}

pub(super) fn lowest_prefix_index(pos: usize, window_log: u32) -> usize {
    pos.saturating_sub(1_usize << window_log)
}

pub(super) fn highbit32(value: u32) -> u32 {
    debug_assert!(value > 0);
    u32::BITS - 1 - value.leading_zeros()
}

fn hash4(value: u32, h_bits: u32) -> usize {
    const PRIME_4_BYTES: u32 = 2_654_435_761;
    value.wrapping_mul(PRIME_4_BYTES).wrapping_shr(32 - h_bits) as usize
}

fn hash5(value: u64, h_bits: u32) -> usize {
    const PRIME_5_BYTES: u64 = 889_523_592_379;
    ((value << (64 - 40)).wrapping_mul(PRIME_5_BYTES) >> (64 - h_bits)) as usize
}

fn hash6(value: u64, h_bits: u32) -> usize {
    const PRIME_6_BYTES: u64 = 227_718_039_650_203;
    ((value << (64 - 48)).wrapping_mul(PRIME_6_BYTES) >> (64 - h_bits)) as usize
}

fn read64(src: &[u8], pos: usize) -> u64 {
    u64::from_le_bytes(src[pos..pos + 8].try_into().expect("read64 in bounds"))
}
