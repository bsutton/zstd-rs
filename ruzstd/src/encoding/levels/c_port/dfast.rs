//! No-dictionary double-fast block compressor ported from `zstd_double_fast.c`.

use alloc::vec::Vec;
use core::{convert::TryInto, ops::Range};

use super::params::CompressionParameters;
use super::sequence_store::{OffBase, RepeatCode, RepeatOffsets, StoredSequence};

const HASH_READ_SIZE: usize = 8;
const SEARCH_STRENGTH: usize = 8;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DFastBlockOutput {
    pub(crate) sequences: Vec<StoredSequence>,
    pub(crate) last_literals: u32,
    pub(crate) repeat_offsets: RepeatOffsets,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DFastMatchState {
    hash_long: Vec<u32>,
    hash_small: Vec<u32>,
    hash_log: u32,
    chain_log: u32,
}

impl DFastMatchState {
    pub(crate) fn new() -> Self {
        Self {
            hash_long: Vec::new(),
            hash_small: Vec::new(),
            hash_log: 0,
            chain_log: 0,
        }
    }

    fn ensure_tables(&mut self, params: CompressionParameters) {
        if self.hash_log != params.hash_log {
            self.hash_log = params.hash_log;
            self.hash_long.clear();
        }
        if self.chain_log != params.chain_log {
            self.chain_log = params.chain_log;
            self.hash_small.clear();
        }

        let long_size = 1_usize << params.hash_log;
        if self.hash_long.len() != long_size {
            self.hash_long.resize(long_size, 0);
        }

        let small_size = 1_usize << params.chain_log;
        if self.hash_small.len() != small_size {
            self.hash_small.resize(small_size, 0);
        }
    }
}

pub(crate) fn compress_block_double_fast_no_dict(
    src: &[u8],
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
) -> DFastBlockOutput {
    let mut state = DFastMatchState::new();
    compress_block_double_fast_no_dict_with_state(
        src,
        0..src.len(),
        params,
        repeat_offsets,
        &mut state,
    )
}

pub(crate) fn compress_block_double_fast_no_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut DFastMatchState,
) -> DFastBlockOutput {
    debug_assert!(block_range.start <= block_range.end);
    debug_assert!(block_range.end <= src.len());

    let mut rep = repeat_offsets.as_offsets();
    let mut sequences = Vec::new();
    let block_start = block_range.start;
    let block_end = block_range.end;
    let block_len = block_end - block_start;

    if block_len <= HASH_READ_SIZE {
        return DFastBlockOutput {
            sequences,
            last_literals: block_len as u32,
            repeat_offsets,
        };
    }

    state.ensure_tables(params);
    let hash_long = &mut state.hash_long;
    let hash_small = &mut state.hash_small;
    let h_bits_l = params.hash_log;
    let h_bits_s = params.chain_log;
    let min_match = params.min_match;
    let prefix_lowest_index = lowest_prefix_index(block_end, params.window_log);
    let ilimit = block_end - HASH_READ_SIZE;

    let mut anchor = block_start;
    let mut ip = if block_start == prefix_lowest_index {
        block_start + 1
    } else {
        block_start
    };

    let mut offset_1 = rep[0] as usize;
    let mut offset_2 = rep[1] as usize;
    let mut offset_saved1 = 0_usize;
    let mut offset_saved2 = 0_usize;

    let current = ip;
    let window_low = lowest_prefix_index(current, params.window_log);
    let max_rep = current - window_low;
    if offset_2 > max_rep {
        offset_saved2 = offset_2;
        offset_2 = 0;
    }
    if offset_1 > max_rep {
        offset_saved1 = offset_1;
        offset_1 = 0;
    }

    'outer: loop {
        let mut step = 1_usize;
        let mut next_step = ip + (1 << SEARCH_STRENGTH);
        let mut ip1 = ip + step;

        if ip1 > ilimit {
            break;
        }

        let mut hl0 = hash_ptr(src, ip, h_bits_l, 8);
        let mut idxl0 = hash_long[hl0] as usize;
        let mut matchl0 = idxl0;

        loop {
            let hs0 = hash_ptr(src, ip, h_bits_s, min_match);
            let idxs0 = hash_small[hs0] as usize;
            let curr = ip;
            let mut matchs0 = idxs0;

            hash_long[hl0] = curr as u32;
            hash_small[hs0] = curr as u32;

            if offset_1 > 0
                && ip + 1 >= offset_1
                && read32(src, ip + 1 - offset_1) == read32(src, ip + 1)
            {
                let match_length =
                    count_match(src, ip + 1 + 4, ip + 1 + 4 - offset_1, block_end) + 4;
                ip += 1;
                store_match(
                    &mut sequences,
                    &mut anchor,
                    &mut ip,
                    OffBase::Repeat(RepeatCode::First),
                    match_length,
                );
                consume_immediate_repcodes(
                    src,
                    hash_long,
                    hash_small,
                    h_bits_l,
                    h_bits_s,
                    min_match,
                    &mut sequences,
                    &mut anchor,
                    &mut ip,
                    ilimit,
                    &mut offset_1,
                    &mut offset_2,
                    block_end,
                );
                continue 'outer;
            }

            let hl1 = hash_ptr(src, ip1, h_bits_l, 8);

            if idxl0 > prefix_lowest_index && read64(src, matchl0) == read64(src, ip) {
                let mut match_length = count_match(src, ip + 8, matchl0 + 8, block_end) + 8;
                let mut offset = ip - matchl0;
                while ip > anchor
                    && matchl0 > prefix_lowest_index
                    && src[ip - 1] == src[matchl0 - 1]
                {
                    ip -= 1;
                    matchl0 -= 1;
                    offset = ip - matchl0;
                    match_length += 1;
                }
                store_offset_match(
                    &mut sequences,
                    &mut anchor,
                    &mut ip,
                    &mut offset_1,
                    &mut offset_2,
                    offset,
                    match_length,
                );
                complementary_insert(
                    src, hash_long, hash_small, h_bits_l, h_bits_s, min_match, curr, ip, ilimit,
                );
                consume_immediate_repcodes(
                    src,
                    hash_long,
                    hash_small,
                    h_bits_l,
                    h_bits_s,
                    min_match,
                    &mut sequences,
                    &mut anchor,
                    &mut ip,
                    ilimit,
                    &mut offset_1,
                    &mut offset_2,
                    block_end,
                );
                continue 'outer;
            }

            let idxl1 = hash_long[hl1] as usize;
            let matchl1 = idxl1;

            if idxs0 > prefix_lowest_index && read32(src, matchs0) == read32(src, ip) {
                let mut match_length = count_match(src, ip + 4, matchs0 + 4, block_end) + 4;
                let mut offset = ip - matchs0;

                if idxl1 > prefix_lowest_index && read64(src, matchl1) == read64(src, ip1) {
                    let l1_len = count_match(src, ip1 + 8, matchl1 + 8, block_end) + 8;
                    if l1_len > match_length {
                        ip = ip1;
                        match_length = l1_len;
                        offset = ip - matchl1;
                        matchs0 = matchl1;
                    }
                }

                while ip > anchor
                    && matchs0 > prefix_lowest_index
                    && src[ip - 1] == src[matchs0 - 1]
                {
                    ip -= 1;
                    matchs0 -= 1;
                    offset = ip - matchs0;
                    match_length += 1;
                }

                if step < 4 {
                    hash_long[hl1] = ip1 as u32;
                }
                store_offset_match(
                    &mut sequences,
                    &mut anchor,
                    &mut ip,
                    &mut offset_1,
                    &mut offset_2,
                    offset,
                    match_length,
                );
                complementary_insert(
                    src, hash_long, hash_small, h_bits_l, h_bits_s, min_match, curr, ip, ilimit,
                );
                consume_immediate_repcodes(
                    src,
                    hash_long,
                    hash_small,
                    h_bits_l,
                    h_bits_s,
                    min_match,
                    &mut sequences,
                    &mut anchor,
                    &mut ip,
                    ilimit,
                    &mut offset_1,
                    &mut offset_2,
                    block_end,
                );
                continue 'outer;
            }

            if ip1 >= next_step {
                step += 1;
                next_step += 1 << SEARCH_STRENGTH;
            }
            ip = ip1;
            ip1 += step;

            hl0 = hl1;
            idxl0 = idxl1;
            matchl0 = matchl1;

            if ip1 > ilimit {
                break;
            }
        }
    }

    if offset_saved1 != 0 && offset_1 != 0 {
        offset_saved2 = offset_saved1;
    }
    rep[0] = (if offset_1 != 0 {
        offset_1
    } else {
        offset_saved1
    }) as u32;
    rep[1] = (if offset_2 != 0 {
        offset_2
    } else {
        offset_saved2
    }) as u32;

    DFastBlockOutput {
        sequences,
        last_literals: (block_end - anchor) as u32,
        repeat_offsets: RepeatOffsets::from_offsets(rep[0], rep[1], rep[2]),
    }
}

#[allow(clippy::too_many_arguments)]
fn complementary_insert(
    src: &[u8],
    hash_long: &mut [u32],
    hash_small: &mut [u32],
    h_bits_l: u32,
    h_bits_s: u32,
    min_match: u32,
    curr: usize,
    ip: usize,
    ilimit: usize,
) {
    if ip > ilimit {
        return;
    }

    let index_to_insert = curr + 2;
    if index_to_insert <= ilimit {
        hash_long[hash_ptr(src, index_to_insert, h_bits_l, 8)] = index_to_insert as u32;
        hash_small[hash_ptr(src, index_to_insert, h_bits_s, min_match)] = index_to_insert as u32;
    }
    if let Some(index) = ip.checked_sub(2).filter(|index| *index <= ilimit) {
        hash_long[hash_ptr(src, index, h_bits_l, 8)] = index as u32;
    }
    if let Some(index) = ip.checked_sub(1).filter(|index| *index <= ilimit) {
        hash_small[hash_ptr(src, index, h_bits_s, min_match)] = index as u32;
    }
}

#[allow(clippy::too_many_arguments)]
fn consume_immediate_repcodes(
    src: &[u8],
    hash_long: &mut [u32],
    hash_small: &mut [u32],
    h_bits_l: u32,
    h_bits_s: u32,
    min_match: u32,
    sequences: &mut Vec<StoredSequence>,
    anchor: &mut usize,
    ip: &mut usize,
    ilimit: usize,
    offset_1: &mut usize,
    offset_2: &mut usize,
    block_end: usize,
) {
    while *ip <= ilimit
        && *offset_2 > 0
        && *ip >= *offset_2
        && read32(src, *ip) == read32(src, *ip - *offset_2)
    {
        let repeat_length = count_match(src, *ip + 4, *ip + 4 - *offset_2, block_end) + 4;
        core::mem::swap(offset_1, offset_2);
        hash_small[hash_ptr(src, *ip, h_bits_s, min_match)] = *ip as u32;
        hash_long[hash_ptr(src, *ip, h_bits_l, 8)] = *ip as u32;
        store_match(
            sequences,
            anchor,
            ip,
            OffBase::Repeat(RepeatCode::First),
            repeat_length,
        );
    }
}

fn store_offset_match(
    sequences: &mut Vec<StoredSequence>,
    anchor: &mut usize,
    ip: &mut usize,
    offset_1: &mut usize,
    offset_2: &mut usize,
    offset: usize,
    match_length: usize,
) {
    *offset_2 = *offset_1;
    *offset_1 = offset;
    store_match(
        sequences,
        anchor,
        ip,
        OffBase::Offset(offset as u32),
        match_length,
    );
}

fn store_match(
    sequences: &mut Vec<StoredSequence>,
    anchor: &mut usize,
    ip: &mut usize,
    off_base: OffBase,
    match_length: usize,
) {
    sequences.push(StoredSequence::new(
        (*ip - *anchor) as u32,
        off_base,
        match_length as u32,
    ));
    *ip += match_length;
    *anchor = *ip;
}

fn count_match(src: &[u8], mut pos: usize, mut match_pos: usize, match_limit: usize) -> usize {
    let start = pos;
    while pos < match_limit && match_pos < src.len() && src[pos] == src[match_pos] {
        pos += 1;
        match_pos += 1;
    }
    pos - start
}

fn lowest_prefix_index(end_index: usize, window_log: u32) -> usize {
    let window_size = 1_usize << window_log;
    end_index.saturating_sub(window_size)
}

fn hash_ptr(src: &[u8], pos: usize, h_bits: u32, min_match: u32) -> usize {
    debug_assert!(h_bits <= 32);
    debug_assert!(pos + HASH_READ_SIZE <= src.len());

    match min_match {
        5 => hash5(read64(src, pos), h_bits),
        6 => hash6(read64(src, pos), h_bits),
        7 => hash7(read64(src, pos), h_bits),
        8 => hash8(read64(src, pos), h_bits),
        _ => hash4(read32(src, pos), h_bits),
    }
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

fn hash7(value: u64, h_bits: u32) -> usize {
    const PRIME_7_BYTES: u64 = 58_295_818_150_454_627;
    ((value << (64 - 56)).wrapping_mul(PRIME_7_BYTES) >> (64 - h_bits)) as usize
}

fn hash8(value: u64, h_bits: u32) -> usize {
    const PRIME_8_BYTES: u64 = 0xCF1B_BCDC_B7A5_6463;
    value.wrapping_mul(PRIME_8_BYTES).wrapping_shr(64 - h_bits) as usize
}

fn read32(src: &[u8], pos: usize) -> u32 {
    u32::from_le_bytes(src[pos..pos + 4].try_into().expect("read32 in bounds"))
}

fn read64(src: &[u8], pos: usize) -> u64 {
    u64::from_le_bytes(src[pos..pos + 8].try_into().expect("read64 in bounds"))
}
