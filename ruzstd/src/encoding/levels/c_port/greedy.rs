//! No-dictionary greedy block compressor ported from `zstd_lazy.c`.

use alloc::vec::Vec;
use core::ops::Range;

use super::bt_match::bt_find_best_match;
use super::greedy_bounds::LazyDictionaryBounds;
pub(crate) use super::greedy_state::GreedyMatchState;
use super::hash_chain_match::{
    hc_find_best_match, highbit32, lowest_prefix_index_with_loaded_dict, MatchSearchConfig,
};
use super::params::CompressionParameters;
use super::row_match::{fill_hash_cache, row_find_best_match, row_match_finder_enabled};
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

pub(super) const SEARCH_HASH_CHAIN: u8 = 0;
pub(super) const SEARCH_BINARY_TREE: u8 = 1;
pub(super) const SEARCH_ROW_HASH: u8 = 2;

struct LazySearchContext<'a> {
    src: &'a [u8],
    block_end: usize,
    config: MatchSearchConfig,
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
    compress_block_greedy_no_dict_with_state_and_loaded_dict(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        0,
    )
}

pub(crate) fn compress_block_greedy_no_dict_with_state_and_loaded_dict(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
    loaded_dict_end: usize,
) -> GreedyBlockOutput {
    compress_block_hash_or_row_no_dict_with_state(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        0,
        loaded_dict_end,
    )
}

pub(crate) fn compress_block_lazy_no_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
) -> GreedyBlockOutput {
    compress_block_lazy_no_dict_with_state_and_loaded_dict(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        0,
    )
}

pub(crate) fn compress_block_lazy_no_dict_with_state_and_loaded_dict(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
    loaded_dict_end: usize,
) -> GreedyBlockOutput {
    compress_block_hash_or_row_no_dict_with_state(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        1,
        loaded_dict_end,
    )
}

pub(crate) fn compress_block_lazy2_no_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
) -> GreedyBlockOutput {
    compress_block_lazy2_no_dict_with_state_and_loaded_dict(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        0,
    )
}

pub(crate) fn compress_block_lazy2_no_dict_with_state_and_loaded_dict(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
    loaded_dict_end: usize,
) -> GreedyBlockOutput {
    compress_block_hash_or_row_no_dict_with_state(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        2,
        loaded_dict_end,
    )
}

pub(crate) fn compress_block_btlazy2_no_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
) -> GreedyBlockOutput {
    compress_block_btlazy2_no_dict_with_state_and_loaded_dict(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        0,
    )
}

pub(crate) fn compress_block_btlazy2_no_dict_with_state_and_loaded_dict(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
    loaded_dict_end: usize,
) -> GreedyBlockOutput {
    compress_block_lazy_generic_no_dict_with_state::<SEARCH_BINARY_TREE>(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        2,
        loaded_dict_end,
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
    compress_block_lazy_generic_no_dict_with_state::<SEARCH_HASH_CHAIN>(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        depth,
        0,
    )
}

fn compress_block_hash_or_row_no_dict_with_state(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
    depth: u32,
    loaded_dict_end: usize,
) -> GreedyBlockOutput {
    if row_match_finder_enabled(params) {
        compress_block_lazy_generic_no_dict_with_state::<SEARCH_ROW_HASH>(
            src,
            block_range,
            params,
            repeat_offsets,
            state,
            depth,
            loaded_dict_end,
        )
    } else {
        compress_block_lazy_generic_no_dict_with_state::<SEARCH_HASH_CHAIN>(
            src,
            block_range,
            params,
            repeat_offsets,
            state,
            depth,
            loaded_dict_end,
        )
    }
}

fn compress_block_lazy_generic_no_dict_with_state<const SEARCH: u8>(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
    depth: u32,
    loaded_dict_end: usize,
) -> GreedyBlockOutput {
    let bounds = LazyDictionaryBounds::no_dict(block_range.end, params, loaded_dict_end);
    compress_block_lazy_generic_with_state::<SEARCH>(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        depth,
        bounds,
    )
}

pub(super) fn compress_block_lazy_generic_with_state<const SEARCH: u8>(
    src: &[u8],
    block_range: Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut GreedyMatchState,
    depth: u32,
    bounds: LazyDictionaryBounds,
) -> GreedyBlockOutput {
    debug_assert!(block_range.start <= block_range.end);
    debug_assert!(block_range.end <= src.len());
    debug_assert!(!bounds.ext_dict || bounds.dict_limit <= block_range.start);

    let mut rep = repeat_offsets.as_offsets();
    let mut sequences = Vec::new();
    let block_start = block_range.start;
    let block_end = block_range.end;
    let block_len = block_end - block_start;

    let search_read_size = search_read_size::<SEARCH>();
    if block_len <= search_read_size {
        return GreedyBlockOutput {
            sequences,
            last_literals: block_len as u32,
            repeat_offsets,
        };
    }

    state.ensure_tables(params);
    state.correct_after_long_match_gap(block_start);
    state.lazy_skipping = false;

    let min_match = params.min_match.clamp(4, 6);
    let search_config = MatchSearchConfig::new(params, min_match, bounds.loaded_dict_end);
    let ilimit = block_end - search_read_size;
    let search_context = LazySearchContext {
        src,
        block_end,
        config: search_config,
    };
    let mut ip = block_start + usize::from(block_start == bounds.prefix_start_index);
    if SEARCH == SEARCH_ROW_HASH {
        fill_hash_cache(src, state.next_to_update, ilimit, params, min_match, state);
    }
    let mut anchor = block_start;

    let mut offset_1 = rep[0] as usize;
    let mut offset_2 = rep[1] as usize;
    let mut offset_saved1 = 0_usize;
    let mut offset_saved2 = 0_usize;

    if !bounds.ext_dict {
        let window_low =
            lowest_prefix_index_with_loaded_dict(ip, params.window_log, bounds.loaded_dict_end);
        let max_rep = ip - window_low;
        if offset_2 > max_rep {
            offset_saved2 = offset_2;
            offset_2 = 0;
        }
        if offset_1 > max_rep {
            offset_saved1 = offset_1;
            offset_1 = 0;
        }
    }

    while ip < ilimit {
        let mut match_length = 0_usize;
        let mut off_base = OffBase::Repeat(RepeatCode::First).to_c_value();
        let mut start = ip + 1;

        if let Some(length) = bounds.rep_match_length(src, ip + 1, offset_1, params, block_end) {
            match_length = length;
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
                    params,
                    bounds,
                );
                continue;
            }
        }

        let mut offbase_found = 999_999_999_u32;
        let ml2 = search_max::<SEARCH>(&search_context, ip, &mut offbase_found, state);
        if ml2 > match_length {
            match_length = ml2;
            start = ip;
            off_base = offbase_found;
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
                if off_base != 0 {
                    let ml_rep = bounds
                        .rep_match_length(src, ip, offset_1, params, block_end)
                        .unwrap_or(0);
                    let gain2 = (ml_rep * 3) as i32;
                    let gain1 = (match_length * 3) as i32 - highbit32(off_base) as i32 + 1;
                    if ml_rep >= 4 && gain2 > gain1 {
                        match_length = ml_rep;
                        off_base = OffBase::Repeat(RepeatCode::First).to_c_value();
                        start = ip;
                    }
                }

                let mut ofb_candidate = 999_999_999_u32;
                let ml2 = search_max::<SEARCH>(&search_context, ip, &mut ofb_candidate, state);
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
                    if off_base != 0 {
                        let ml_rep = bounds
                            .rep_match_length(src, ip, offset_1, params, block_end)
                            .unwrap_or(0);
                        let gain2 = (ml_rep * 4) as i32;
                        let gain1 = (match_length * 4) as i32 - highbit32(off_base) as i32 + 1;
                        if ml_rep >= 4 && gain2 > gain1 {
                            match_length = ml_rep;
                            off_base = OffBase::Repeat(RepeatCode::First).to_c_value();
                            start = ip;
                        }
                    }

                    let mut ofb_candidate = 999_999_999_u32;
                    let ml2 = search_max::<SEARCH>(&search_context, ip, &mut ofb_candidate, state);
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
            let mut match_pos = start - offset;
            let low_match = bounds.low_match_index(match_pos);
            while start > anchor && match_pos > low_match && src[start - 1] == src[match_pos - 1] {
                start -= 1;
                match_pos -= 1;
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
            params,
            bounds,
        );
    }

    if bounds.ext_dict {
        rep[0] = offset_1 as u32;
        rep[1] = offset_2 as u32;
    } else {
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
    }

    GreedyBlockOutput {
        sequences,
        last_literals: (block_end - anchor) as u32,
        repeat_offsets: RepeatOffsets::from_offsets(rep[0], rep[1], rep[2]),
    }
}

fn search_max<const SEARCH: u8>(
    context: &LazySearchContext<'_>,
    ip: usize,
    off_base: &mut u32,
    state: &mut GreedyMatchState,
) -> usize {
    match SEARCH {
        SEARCH_HASH_CHAIN => hc_find_best_match(
            context.src,
            ip,
            context.block_end,
            off_base,
            state,
            context.config,
        ),
        SEARCH_ROW_HASH => row_find_best_match(
            context.src,
            ip,
            context.block_end,
            off_base,
            state,
            context.config,
        ),
        SEARCH_BINARY_TREE => bt_find_best_match(
            context.src,
            ip,
            context.block_end,
            off_base,
            state,
            context.config,
        ),
        _ => unreachable!("unknown lazy search kind"),
    }
}

fn search_read_size<const SEARCH: u8>() -> usize {
    match SEARCH {
        SEARCH_ROW_HASH => HASH_READ_SIZE + ROW_HASH_CACHE_SIZE,
        SEARCH_HASH_CHAIN | SEARCH_BINARY_TREE => HASH_READ_SIZE,
        _ => unreachable!("unknown lazy search kind"),
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
    params: CompressionParameters,
    bounds: LazyDictionaryBounds,
) {
    while *ip <= ilimit {
        let Some(repeat_length) = bounds.rep_match_length(src, *ip, *offset_2, params, block_end)
        else {
            break;
        };
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
