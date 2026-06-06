//! No-dictionary greedy block compressor ported from `zstd_lazy.c`.

use alloc::vec::Vec;
use core::{convert::TryInto, ops::Range};

use super::params::CompressionParameters;
use super::sequence_store::{OffBase, RepeatCode, RepeatOffsets, StoredSequence};

const HASH_READ_SIZE: usize = 8;
const SEARCH_STRENGTH: usize = 8;
const LAZY_SKIPPING_STEP: usize = 8;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GreedyBlockOutput {
    pub(crate) sequences: Vec<StoredSequence>,
    pub(crate) last_literals: u32,
    pub(crate) repeat_offsets: RepeatOffsets,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GreedyMatchState {
    hash_table: Vec<u32>,
    chain_table: Vec<u32>,
    hash_log: u32,
    chain_log: u32,
    next_to_update: usize,
    lazy_skipping: bool,
}

impl GreedyMatchState {
    pub(crate) fn new() -> Self {
        Self {
            hash_table: Vec::new(),
            chain_table: Vec::new(),
            hash_log: 0,
            chain_log: 0,
            next_to_update: 0,
            lazy_skipping: false,
        }
    }

    fn ensure_tables(&mut self, params: CompressionParameters) {
        if self.hash_log != params.hash_log {
            self.hash_log = params.hash_log;
            self.hash_table.clear();
            self.next_to_update = 0;
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

        let chain_size = 1_usize << params.chain_log;
        if self.chain_table.len() != chain_size {
            self.chain_table.resize(chain_size, 0);
        }
    }
}

pub(crate) fn compress_block_greedy_no_dict(
    src: &[u8],
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
) -> GreedyBlockOutput {
    let mut state = GreedyMatchState::new();
    compress_block_greedy_no_dict_with_state(src, 0..src.len(), params, repeat_offsets, &mut state)
}

pub(crate) fn compress_block_greedy_no_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
) -> GreedyBlockOutput {
    debug_assert!(block_range.start <= block_range.end);
    debug_assert!(block_range.end <= src.len());

    let mut rep = repeat_offsets.as_offsets();
    let mut sequences = Vec::new();
    let block_start = block_range.start;
    let block_end = block_range.end;
    let block_len = block_end - block_start;

    if block_len <= HASH_READ_SIZE {
        return GreedyBlockOutput {
            sequences,
            last_literals: block_len as u32,
            repeat_offsets,
        };
    }

    state.ensure_tables(params);
    state.lazy_skipping = false;

    let min_match = params.min_match.clamp(4, 6);
    let prefix_lowest = lowest_prefix_index(block_end, params.window_log);
    let ilimit = block_end - HASH_READ_SIZE;
    let mut ip = block_start + usize::from(block_start == prefix_lowest);
    let mut anchor = block_start;

    let mut offset_1 = rep[0] as usize;
    let mut offset_2 = rep[1] as usize;
    let mut offset_saved1 = 0_usize;
    let mut offset_saved2 = 0_usize;

    let window_low = lowest_prefix_index(ip, params.window_log);
    let max_rep = ip - window_low;
    if offset_2 > max_rep {
        offset_saved2 = offset_2;
        offset_2 = 0;
    }
    if offset_1 > max_rep {
        offset_saved1 = offset_1;
        offset_1 = 0;
    }

    while ip < ilimit {
        let mut match_length = 0_usize;
        let mut off_base = Some(OffBase::Repeat(RepeatCode::First));
        let mut start = ip + 1;

        if offset_1 > 0
            && ip + 1 >= offset_1
            && read32(src, ip + 1 - offset_1) == read32(src, ip + 1)
        {
            match_length = count_match(src, ip + 1 + 4, ip + 1 + 4 - offset_1, block_end) + 4;
        } else {
            let mut offbase_found = 0_u32;
            let ml2 = hc_find_best_match(
                src,
                ip,
                block_end,
                &mut offbase_found,
                params,
                min_match,
                state,
            );
            if ml2 > match_length {
                match_length = ml2;
                start = ip;
                off_base = OffBase::from_c_value(offbase_found);
            }
        }

        if match_length < 4 {
            let step = ((ip - anchor) >> SEARCH_STRENGTH) + 1;
            ip += step;
            state.lazy_skipping = step > LAZY_SKIPPING_STEP;
            continue;
        }

        let off_base = off_base.expect("stored match has an offBase");
        if let OffBase::Offset(offset) = off_base {
            let offset = offset as usize;
            while start > anchor
                && start - offset > prefix_lowest
                && src[start - 1] == src[start - offset - 1]
            {
                start -= 1;
                match_length += 1;
            }
            offset_2 = offset_1;
            offset_1 = offset;
        }

        store_sequence(
            &mut sequences,
            &mut anchor,
            &mut ip,
            start,
            off_base,
            match_length,
        );

        if state.lazy_skipping {
            state.lazy_skipping = false;
        }

        while ip <= ilimit && offset_2 > 0 && read32(src, ip) == read32(src, ip - offset_2) {
            let repeat_length = count_match(src, ip + 4, ip + 4 - offset_2, block_end) + 4;
            core::mem::swap(&mut offset_2, &mut offset_1);
            let repeat_start = anchor;
            store_sequence(
                &mut sequences,
                &mut anchor,
                &mut ip,
                repeat_start,
                OffBase::Repeat(RepeatCode::First),
                repeat_length,
            );
        }
    }

    if offset_saved1 != 0 && offset_1 != 0 {
        offset_saved2 = offset_saved1;
    }

    rep[0] = if offset_1 != 0 {
        offset_1
    } else {
        offset_saved1
    } as u32;
    rep[1] = if offset_2 != 0 {
        offset_2
    } else {
        offset_saved2
    } as u32;

    GreedyBlockOutput {
        sequences,
        last_literals: (block_end - anchor) as u32,
        repeat_offsets: RepeatOffsets::from_offsets(rep[0], rep[1], rep[2]),
    }
}

fn hc_find_best_match(
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

fn store_sequence(
    sequences: &mut Vec<StoredSequence>,
    anchor: &mut usize,
    ip: &mut usize,
    start: usize,
    off_base: OffBase,
    match_length: usize,
) {
    sequences.push(StoredSequence::new(
        (start - *anchor) as u32,
        off_base,
        match_length as u32,
    ));
    *ip = start + match_length;
    *anchor = *ip;
}

fn count_match(src: &[u8], mut pos: usize, mut match_pos: usize, match_limit: usize) -> usize {
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

fn hash_ptr(src: &[u8], pos: usize, h_bits: u32, min_match: u32) -> usize {
    match min_match {
        5 => hash5(read64(src, pos), h_bits),
        6 => hash6(read64(src, pos), h_bits),
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

fn read32(src: &[u8], pos: usize) -> u32 {
    u32::from_le_bytes(src[pos..pos + 4].try_into().expect("read32 in bounds"))
}

fn read64(src: &[u8], pos: usize) -> u64 {
    u64::from_le_bytes(src[pos..pos + 8].try_into().expect("read64 in bounds"))
}

fn lowest_prefix_index(pos: usize, window_log: u32) -> usize {
    pos.saturating_sub(1_usize << window_log)
}
