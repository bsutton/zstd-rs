//! Hash-chain search primitives shared by the C greedy/lazy/lazy2 ports.

use core::convert::TryInto;

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
