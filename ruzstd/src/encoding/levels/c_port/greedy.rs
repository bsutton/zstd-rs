//! No-dictionary greedy block compressor ported from `zstd_lazy.c`.

use alloc::vec::Vec;
use core::ops::Range;

use super::bt_match::bt_find_best_match;
pub(crate) use super::greedy_state::GreedyMatchState;
use super::hash_chain_match::{
    count_match, hc_find_best_match, highbit32, lowest_prefix_index, read32,
};
use super::params::CompressionParameters;
use super::row_match::{row_find_best_match, row_match_finder_enabled};
use super::sequence_store::{OffBase, RepeatCode, RepeatOffsets, StoredSequence};

const HASH_READ_SIZE: usize = 8;
const ROW_HASH_CACHE_SIZE: usize = 8;
const SEARCH_STRENGTH: usize = 8;
const LAZY_SKIPPING_STEP: usize = 8;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GreedyBlockOutput {
    pub(crate) sequences: Vec<StoredSequence>,
    pub(crate) last_literals: u32,
    pub(crate) repeat_offsets: RepeatOffsets,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LazySearch {
    HashChain,
    BinaryTree,
    RowHash,
}

struct LazySearchContext<'a> {
    search: LazySearch,
    src: &'a [u8],
    block_end: usize,
    params: CompressionParameters,
    min_match: u32,
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
    let search = if row_match_finder_enabled(params) {
        LazySearch::RowHash
    } else {
        LazySearch::HashChain
    };
    compress_block_lazy_generic_no_dict_with_state(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        0,
        search,
    )
}

pub(crate) fn compress_block_lazy_no_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
) -> GreedyBlockOutput {
    let search = if row_match_finder_enabled(params) {
        LazySearch::RowHash
    } else {
        LazySearch::HashChain
    };
    compress_block_lazy_generic_no_dict_with_state(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        1,
        search,
    )
}

pub(crate) fn compress_block_lazy2_no_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
) -> GreedyBlockOutput {
    let search = if row_match_finder_enabled(params) {
        LazySearch::RowHash
    } else {
        LazySearch::HashChain
    };
    compress_block_lazy_generic_no_dict_with_state(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        2,
        search,
    )
}

pub(crate) fn compress_block_btlazy2_no_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
) -> GreedyBlockOutput {
    compress_block_lazy_generic_no_dict_with_state(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        2,
        LazySearch::BinaryTree,
    )
}

fn compress_block_hash_chain_no_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
    depth: u32,
) -> GreedyBlockOutput {
    compress_block_lazy_generic_no_dict_with_state(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        depth,
        LazySearch::HashChain,
    )
}

fn compress_block_lazy_generic_no_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
    depth: u32,
    search: LazySearch,
) -> GreedyBlockOutput {
    debug_assert!(block_range.start <= block_range.end);
    debug_assert!(block_range.end <= src.len());

    let mut rep = repeat_offsets.as_offsets();
    let mut sequences = Vec::new();
    let block_start = block_range.start;
    let block_end = block_range.end;
    let block_len = block_end - block_start;

    let search_read_size = search_read_size(search);
    if block_len <= search_read_size {
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
    let ilimit = block_end - search_read_size;
    let search_context = LazySearchContext {
        search,
        src,
        block_end,
        params,
        min_match,
    };
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
        let mut off_base = OffBase::Repeat(RepeatCode::First).to_c_value();
        let mut start = ip + 1;

        if offset_1 > 0
            && ip + 1 >= offset_1
            && read32(src, ip + 1 - offset_1) == read32(src, ip + 1)
        {
            match_length = count_match(src, ip + 1 + 4, ip + 1 + 4 - offset_1, block_end) + 4;
            if depth == 0 {
                store_sequence(
                    &mut sequences,
                    &mut anchor,
                    &mut ip,
                    start,
                    OffBase::from_c_value(off_base).expect("repcode offBase"),
                    match_length,
                );
                continue_immediate_repcodes(
                    src,
                    &mut sequences,
                    &mut anchor,
                    &mut ip,
                    ilimit,
                    block_end,
                    &mut offset_1,
                    &mut offset_2,
                );
                continue;
            }
        } else {
            let mut offbase_found = 999_999_999_u32;
            let ml2 = search_max(&search_context, ip, &mut offbase_found, state);
            if ml2 > match_length {
                match_length = ml2;
                start = ip;
                off_base = offbase_found;
            }
        }

        if match_length < 4 {
            let step = ((ip - anchor) >> SEARCH_STRENGTH) + 1;
            ip += step;
            state.lazy_skipping = step > LAZY_SKIPPING_STEP;
            continue;
        }

        if depth >= 1 {
            loop {
                ip += 1;
                if off_base != 0
                    && offset_1 > 0
                    && ip >= offset_1
                    && read32(src, ip) == read32(src, ip - offset_1)
                {
                    let ml_rep = count_match(src, ip + 4, ip + 4 - offset_1, block_end) + 4;
                    let gain2 = (ml_rep * 3) as i32;
                    let gain1 = (match_length * 3) as i32 - highbit32(off_base) as i32 + 1;
                    if ml_rep >= 4 && gain2 > gain1 {
                        match_length = ml_rep;
                        off_base = OffBase::Repeat(RepeatCode::First).to_c_value();
                        start = ip;
                    }
                }

                let mut ofb_candidate = 999_999_999_u32;
                let ml2 = search_max(&search_context, ip, &mut ofb_candidate, state);
                let gain2 = (ml2 * 4) as i32 - highbit32(ofb_candidate) as i32;
                let gain1 = (match_length * 4) as i32 - highbit32(off_base) as i32 + 4;
                if ml2 >= 4 && gain2 > gain1 {
                    match_length = ml2;
                    off_base = ofb_candidate;
                    start = ip;
                    if ip < ilimit {
                        continue;
                    }
                }

                if depth == 2 && ip < ilimit {
                    ip += 1;
                    if off_base != 0
                        && offset_1 > 0
                        && ip >= offset_1
                        && read32(src, ip) == read32(src, ip - offset_1)
                    {
                        let ml_rep = count_match(src, ip + 4, ip + 4 - offset_1, block_end) + 4;
                        let gain2 = (ml_rep * 4) as i32;
                        let gain1 = (match_length * 4) as i32 - highbit32(off_base) as i32 + 1;
                        if ml_rep >= 4 && gain2 > gain1 {
                            match_length = ml_rep;
                            off_base = OffBase::Repeat(RepeatCode::First).to_c_value();
                            start = ip;
                        }
                    }

                    let mut ofb_candidate = 999_999_999_u32;
                    let ml2 = search_max(&search_context, ip, &mut ofb_candidate, state);
                    let gain2 = (ml2 * 4) as i32 - highbit32(ofb_candidate) as i32;
                    let gain1 = (match_length * 4) as i32 - highbit32(off_base) as i32 + 7;
                    if ml2 >= 4 && gain2 > gain1 {
                        match_length = ml2;
                        off_base = ofb_candidate;
                        start = ip;
                        if ip < ilimit {
                            continue;
                        }
                    }
                }

                break;
            }
        }

        let off_base = OffBase::from_c_value(off_base).expect("stored match has an offBase");
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

        continue_immediate_repcodes(
            src,
            &mut sequences,
            &mut anchor,
            &mut ip,
            ilimit,
            block_end,
            &mut offset_1,
            &mut offset_2,
        );
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

fn search_max(
    context: &LazySearchContext<'_>,
    ip: usize,
    off_base: &mut u32,
    state: &mut GreedyMatchState,
) -> usize {
    match context.search {
        LazySearch::HashChain => hc_find_best_match(
            context.src,
            ip,
            context.block_end,
            off_base,
            context.params,
            context.min_match,
            state,
        ),
        LazySearch::RowHash => row_find_best_match(
            context.src,
            ip,
            context.block_end,
            off_base,
            context.params,
            context.min_match,
            state,
        ),
        LazySearch::BinaryTree => bt_find_best_match(
            context.src,
            ip,
            context.block_end,
            off_base,
            context.params,
            context.min_match,
            state,
        ),
    }
}

fn search_read_size(search: LazySearch) -> usize {
    match search {
        LazySearch::RowHash => HASH_READ_SIZE + ROW_HASH_CACHE_SIZE,
        LazySearch::HashChain | LazySearch::BinaryTree => HASH_READ_SIZE,
    }
}

#[allow(clippy::too_many_arguments)]
fn continue_immediate_repcodes(
    src: &[u8],
    sequences: &mut Vec<StoredSequence>,
    anchor: &mut usize,
    ip: &mut usize,
    ilimit: usize,
    block_end: usize,
    offset_1: &mut usize,
    offset_2: &mut usize,
) {
    while *ip <= ilimit && *offset_2 > 0 && read32(src, *ip) == read32(src, *ip - *offset_2) {
        let repeat_length = count_match(src, *ip + 4, *ip + 4 - *offset_2, block_end) + 4;
        core::mem::swap(offset_2, offset_1);
        let repeat_start = *anchor;
        store_sequence(
            sequences,
            anchor,
            ip,
            repeat_start,
            OffBase::Repeat(RepeatCode::First),
            repeat_length,
        );
    }
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
