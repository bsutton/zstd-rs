//! No-dictionary fast block compressor ported from `zstd_fast.c`.

use alloc::vec::Vec;
use core::{convert::TryInto, ops::Range};

use super::params::CompressionParameters;
use super::sequence_store::{OffBase, RepeatCode, RepeatOffsets, StoredSequence};

const HASH_READ_SIZE: usize = 8;
const SEARCH_STRENGTH: usize = 8;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FastBlockOutput {
    pub(crate) sequences: Vec<StoredSequence>,
    pub(crate) last_literals: u32,
    pub(crate) repeat_offsets: RepeatOffsets,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FastMatchState {
    hash_table: Vec<u32>,
    hash_log: u32,
}

impl FastMatchState {
    pub(crate) fn new() -> Self {
        Self {
            hash_table: Vec::new(),
            hash_log: 0,
        }
    }

    fn table_for(&mut self, hash_log: u32) -> &mut [u32] {
        if self.hash_log != hash_log {
            self.hash_log = hash_log;
            self.hash_table.clear();
        }

        let table_size = 1_usize << hash_log;
        if self.hash_table.len() != table_size {
            self.hash_table.resize(table_size, 0);
        }

        &mut self.hash_table
    }
}

pub(crate) fn compress_block_fast_no_dict(
    src: &[u8],
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
) -> FastBlockOutput {
    let mut state = FastMatchState::new();
    compress_block_fast_no_dict_with_state(src, 0..src.len(), params, repeat_offsets, &mut state)
}

pub(crate) fn compress_block_fast_no_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut FastMatchState,
) -> FastBlockOutput {
    debug_assert!(block_range.start <= block_range.end);
    debug_assert!(block_range.end <= src.len());

    let mut rep = repeat_offsets.as_offsets();
    let mut sequences = Vec::new();
    let block_start = block_range.start;
    let block_end = block_range.end;
    let block_len = block_end - block_start;

    if block_len <= HASH_READ_SIZE {
        return FastBlockOutput {
            sequences,
            last_literals: block_len as u32,
            repeat_offsets,
        };
    }

    let hlog = params.hash_log;
    let min_match = params.min_match;
    let step_size = params.target_length as usize + usize::from(params.target_length == 0) + 1;
    let prefix_start_index = lowest_prefix_index(block_end, params.window_log);
    let ilimit = block_end - HASH_READ_SIZE;

    let hash_table = state.table_for(hlog);
    let mut anchor = block_start;
    let mut ip0 = if block_start == 0 && prefix_start_index == 0 {
        1
    } else {
        block_start
    };

    let mut rep_offset1 = rep[0] as usize;
    let mut rep_offset2 = rep[1] as usize;
    let mut offset_saved1 = 0_usize;
    let mut offset_saved2 = 0_usize;

    let curr = ip0;
    let window_low = lowest_prefix_index(curr, params.window_log);
    let max_rep = curr - window_low;
    if rep_offset2 > max_rep {
        offset_saved2 = rep_offset2;
        rep_offset2 = 0;
    }
    if rep_offset1 > max_rep {
        offset_saved1 = rep_offset1;
        rep_offset1 = 0;
    }

    'restart: loop {
        let mut step = step_size;
        let mut next_step = ip0 + (1 << (SEARCH_STRENGTH - 1));
        let mut ip1 = ip0 + 1;
        let mut ip2 = ip0 + step;
        let mut ip3 = ip2 + 1;

        if ip3 >= ilimit {
            break;
        }

        let mut hash0 = hash_ptr(src, ip0, hlog, min_match);
        let mut hash1 = hash_ptr(src, ip1, hlog, min_match);
        let mut match_idx = hash_table[hash0] as usize;

        while ip3 < ilimit {
            let current0 = ip0;
            hash_table[hash0] = current0 as u32;

            if rep_offset1 > 0
                && ip2 >= rep_offset1
                && read32(src, ip2) == read32(src, ip2 - rep_offset1)
            {
                ip0 = ip2;
                let mut match0 = ip0 - rep_offset1;
                let backward = usize::from(ip0 > 0 && src[ip0 - 1] == src[match0 - 1]);
                ip0 -= backward;
                match0 -= backward;
                hash_table[hash1] = ip1 as u32;
                store_match(
                    src,
                    &mut sequences,
                    &mut anchor,
                    &mut ip0,
                    match0,
                    OffBase::Repeat(RepeatCode::First),
                    4 + backward,
                    block_end,
                );
                fill_after_match(src, hash_table, hlog, min_match, current0, ip0, ilimit);
                consume_immediate_repcodes(
                    src,
                    hash_table,
                    &mut sequences,
                    hlog,
                    min_match,
                    &mut anchor,
                    &mut ip0,
                    ilimit,
                    &mut rep_offset1,
                    &mut rep_offset2,
                    block_end,
                );
                continue 'restart;
            }

            if match4_found(src, ip0, match_idx, prefix_start_index, block_end) {
                hash_table[hash1] = ip1 as u32;
                let mut match0 = match_idx;
                rep_offset2 = rep_offset1;
                rep_offset1 = ip0 - match0;
                let mut match_length = 4;
                while ip0 > anchor && match0 > prefix_start_index && src[ip0 - 1] == src[match0 - 1]
                {
                    ip0 -= 1;
                    match0 -= 1;
                    match_length += 1;
                }
                store_match(
                    src,
                    &mut sequences,
                    &mut anchor,
                    &mut ip0,
                    match0,
                    OffBase::Offset(rep_offset1 as u32),
                    match_length,
                    block_end,
                );
                fill_after_match(src, hash_table, hlog, min_match, current0, ip0, ilimit);
                consume_immediate_repcodes(
                    src,
                    hash_table,
                    &mut sequences,
                    hlog,
                    min_match,
                    &mut anchor,
                    &mut ip0,
                    ilimit,
                    &mut rep_offset1,
                    &mut rep_offset2,
                    block_end,
                );
                continue 'restart;
            }

            match_idx = hash_table[hash1] as usize;
            hash0 = hash1;
            hash1 = hash_ptr(src, ip2, hlog, min_match);
            ip0 = ip1;
            ip1 = ip2;
            ip2 = ip3;

            let current0 = ip0;
            hash_table[hash0] = current0 as u32;

            if match4_found(src, ip0, match_idx, prefix_start_index, block_end) {
                if step <= 4 {
                    hash_table[hash1] = ip1 as u32;
                }
                let mut match0 = match_idx;
                rep_offset2 = rep_offset1;
                rep_offset1 = ip0 - match0;
                let mut match_length = 4;
                while ip0 > anchor && match0 > prefix_start_index && src[ip0 - 1] == src[match0 - 1]
                {
                    ip0 -= 1;
                    match0 -= 1;
                    match_length += 1;
                }
                store_match(
                    src,
                    &mut sequences,
                    &mut anchor,
                    &mut ip0,
                    match0,
                    OffBase::Offset(rep_offset1 as u32),
                    match_length,
                    block_end,
                );
                fill_after_match(src, hash_table, hlog, min_match, current0, ip0, ilimit);
                consume_immediate_repcodes(
                    src,
                    hash_table,
                    &mut sequences,
                    hlog,
                    min_match,
                    &mut anchor,
                    &mut ip0,
                    ilimit,
                    &mut rep_offset1,
                    &mut rep_offset2,
                    block_end,
                );
                continue 'restart;
            }

            match_idx = hash_table[hash1] as usize;
            hash0 = hash1;
            hash1 = hash_ptr(src, ip2, hlog, min_match);
            ip0 = ip1;
            ip1 = ip2;
            ip2 = ip0 + step;
            ip3 = ip1 + step;

            if ip2 >= next_step {
                step += 1;
                next_step += 1 << (SEARCH_STRENGTH - 1);
            }
        }

        break;
    }

    if offset_saved1 != 0 && rep_offset1 != 0 {
        offset_saved2 = offset_saved1;
    }
    rep[0] = (if rep_offset1 != 0 {
        rep_offset1
    } else {
        offset_saved1
    }) as u32;
    rep[1] = (if rep_offset2 != 0 {
        rep_offset2
    } else {
        offset_saved2
    }) as u32;

    FastBlockOutput {
        sequences,
        last_literals: (block_end - anchor) as u32,
        repeat_offsets: RepeatOffsets::from_offsets(rep[0], rep[1], rep[2]),
    }
}

#[allow(clippy::too_many_arguments)]
fn store_match(
    src: &[u8],
    sequences: &mut Vec<StoredSequence>,
    anchor: &mut usize,
    ip: &mut usize,
    match_pos: usize,
    off_base: OffBase,
    match_length: usize,
    match_limit: usize,
) {
    let match_length = match_length
        + count_match(
            src,
            *ip + match_length,
            match_pos + match_length,
            match_limit,
        );
    sequences.push(StoredSequence::new(
        (*ip - *anchor) as u32,
        off_base,
        match_length as u32,
    ));
    *ip += match_length;
    *anchor = *ip;
}

fn fill_after_match(
    src: &[u8],
    hash_table: &mut [u32],
    hlog: u32,
    min_match: u32,
    current0: usize,
    ip: usize,
    ilimit: usize,
) {
    if ip > ilimit {
        return;
    }
    if current0 + 2 <= ilimit {
        hash_table[hash_ptr(src, current0 + 2, hlog, min_match)] = (current0 + 2) as u32;
    }
    if ip >= 2 && ip - 2 <= ilimit {
        hash_table[hash_ptr(src, ip - 2, hlog, min_match)] = (ip - 2) as u32;
    }
}

#[allow(clippy::too_many_arguments)]
fn consume_immediate_repcodes(
    src: &[u8],
    hash_table: &mut [u32],
    sequences: &mut Vec<StoredSequence>,
    hlog: u32,
    min_match: u32,
    anchor: &mut usize,
    ip: &mut usize,
    ilimit: usize,
    rep_offset1: &mut usize,
    rep_offset2: &mut usize,
    match_limit: usize,
) {
    if *rep_offset2 == 0 {
        return;
    }

    while *ip <= ilimit
        && *ip >= *rep_offset2
        && read32(src, *ip) == read32(src, *ip - *rep_offset2)
    {
        let repeat_length = count_match(src, *ip + 4, *ip + 4 - *rep_offset2, match_limit) + 4;
        core::mem::swap(rep_offset1, rep_offset2);
        hash_table[hash_ptr(src, *ip, hlog, min_match)] = *ip as u32;
        *ip += repeat_length;
        sequences.push(StoredSequence::new(
            0,
            OffBase::Repeat(RepeatCode::First),
            repeat_length as u32,
        ));
        *anchor = *ip;
    }
}

fn match4_found(
    src: &[u8],
    current: usize,
    match_idx: usize,
    prefix_start_index: usize,
    match_limit: usize,
) -> bool {
    match_idx >= prefix_start_index
        && current + 4 <= match_limit
        && match_idx + 4 <= src.len()
        && read32(src, current) == read32(src, match_idx)
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
