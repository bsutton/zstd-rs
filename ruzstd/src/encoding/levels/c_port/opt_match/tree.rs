//! Binary-tree update and match enumeration for the optimal parser.

use alloc::vec::Vec;

use super::{collect_repcode_matches, should_stop_after_best_match, OptMatch, OptMatchBounds};
use crate::encoding::levels::c_port::{
    greedy::GreedyMatchState,
    hash_chain_match::{
        count_match_no_dict, hash3_ptr, hash_ptr_mls, lowest_prefix_index_with_loaded_dict,
    },
    params::CompressionParameters,
    sequence_store::{OffBase, RepeatOffsets},
};

const ZSTD_OPT_NUM: usize = 1 << 12;
const TREE_SLOT_NONE: usize = usize::MAX;

#[inline(always)]
pub(super) fn bt_get_all_matches_no_dict_mls<const MLS: u32>(
    matches: &mut Vec<OptMatch>,
    request: super::BtMatchRequest<'_>,
    state: &mut GreedyMatchState,
) {
    let super::BtMatchRequest {
        src,
        ip,
        block_end,
        rep,
        ll0,
        length_to_beat,
        params,
        bounds,
    } = request;

    update_tree_no_dict_mls::<MLS>(src, ip, block_end, params, state, bounds.loaded_dict_end);
    insert_bt_and_get_all_matches_no_dict::<MLS>(
        matches,
        src,
        ip,
        block_end,
        rep,
        ll0,
        length_to_beat,
        params,
        state,
        bounds,
    );
}

#[inline(always)]
pub(super) fn update_tree_no_dict_mls<const MLS: u32>(
    src: &[u8],
    target: usize,
    block_end: usize,
    params: CompressionParameters,
    state: &mut GreedyMatchState,
    loaded_dict_end: usize,
) {
    let mut idx = state.next_to_update;
    while idx < target {
        let forward =
            insert_bt1_no_dict::<MLS>(src, idx, block_end, target, state, params, loaded_dict_end);
        debug_assert!(forward > 0);
        idx += forward;
    }
    state.next_to_update = target;
}

#[inline(always)]
fn insert_bt1_no_dict<const MLS: u32>(
    src: &[u8],
    ip: usize,
    block_end: usize,
    target: usize,
    state: &mut GreedyMatchState,
    params: CompressionParameters,
    loaded_dict_end: usize,
) -> usize {
    let hash = hash_ptr_mls::<MLS>(src, ip, params.hash_log);
    let hash_table = &mut state.hash_table;
    let chain_table = &mut state.chain_table;
    let mut match_index = hash_table[hash] as usize;
    let mask = bt_mask(params);
    let bt_low = ip.saturating_sub(mask);
    let window_low =
        lowest_prefix_index_with_loaded_dict(target, params.window_log, loaded_dict_end).max(1);
    let mut common_smaller = 0_usize;
    let mut common_larger = 0_usize;
    let mut smaller_slot = tree_slot(ip, mask);
    let mut larger_slot = smaller_slot + 1;
    let mut match_end_idx = ip + 9;
    let mut best_length = 8_usize;
    let mut nb_compares = 1_usize << params.search_log;
    hash_table[hash] = ip as u32;

    while nb_compares > 0 && match_index >= window_low {
        nb_compares -= 1;
        let next_slot = tree_slot(match_index, mask);

        let mut match_length = common_smaller.min(common_larger);
        match_length += count_match_no_dict(
            src,
            ip + match_length,
            match_index + match_length,
            block_end,
        );

        if match_length > best_length {
            best_length = match_length;
            if match_length > match_end_idx - match_index {
                match_end_idx = match_index + match_length;
            }
        }

        if ip + match_length == block_end {
            break;
        }

        if src[match_index + match_length] < src[ip + match_length] {
            chain_table[smaller_slot] = match_index as u32;
            common_smaller = match_length;
            if match_index <= bt_low {
                smaller_slot = TREE_SLOT_NONE;
                break;
            }
            smaller_slot = next_slot + 1;
            match_index = chain_table[next_slot + 1] as usize;
        } else {
            chain_table[larger_slot] = match_index as u32;
            common_larger = match_length;
            if match_index <= bt_low {
                larger_slot = TREE_SLOT_NONE;
                break;
            }
            larger_slot = next_slot;
            match_index = chain_table[next_slot] as usize;
        }
    }

    write_tree_slot(chain_table, smaller_slot, 0);
    write_tree_slot(chain_table, larger_slot, 0);

    let positions = best_length.saturating_sub(384).min(192);
    positions.max(match_end_idx - (ip + 8))
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn insert_bt_and_get_all_matches_no_dict<const MLS: u32>(
    matches: &mut Vec<OptMatch>,
    src: &[u8],
    ip: usize,
    block_end: usize,
    rep: RepeatOffsets,
    ll0: bool,
    length_to_beat: u32,
    params: CompressionParameters,
    state: &mut GreedyMatchState,
    bounds: OptMatchBounds,
) {
    let sufficient_len = params.target_length.min((ZSTD_OPT_NUM - 1) as u32) as usize;
    let min_match = if MLS == 3 { 3 } else { 4 };
    let hash = hash_ptr_mls::<MLS>(src, ip, params.hash_log);
    let mut match_index = state.hash_table[hash] as usize;
    let mask = bt_mask(params);
    let bt_low = ip.saturating_sub(mask);
    let window_low = bounds.lowest_match_index(ip, params);
    let match_low = window_low.max(1);
    let mut common_smaller = 0_usize;
    let mut common_larger = 0_usize;
    let mut smaller_slot = tree_slot(ip, mask);
    let mut larger_slot = smaller_slot + 1;
    let mut match_end_idx = ip + 9;
    let mut best_length = length_to_beat.saturating_sub(1) as usize;
    let mut nb_compares = 1_usize << params.search_log;

    collect_repcode_matches(
        matches,
        src,
        ip,
        block_end,
        rep,
        ll0,
        min_match,
        params,
        bounds,
        sufficient_len,
        &mut best_length,
    );
    if should_stop_after_best_match(matches, ip, block_end, sufficient_len) {
        return;
    }

    if MLS == 3 && best_length < 3 {
        if let Some(match_index3) = insert_and_find_first_index_hash3(src, ip, state) {
            let within_price_heuristic = ip - match_index3 < (1 << 18);
            if match_index3 >= match_low && within_price_heuristic {
                let len = count_match_no_dict(src, ip, match_index3, block_end);
                if len >= 3 {
                    best_length = len;
                    matches.clear();
                    matches.push(OptMatch {
                        off_base: OffBase::offset_to_c_value((ip - match_index3) as u32),
                        len: len as u32,
                    });
                    if len > sufficient_len || ip + len == block_end {
                        state.next_to_update = ip + 1;
                        return;
                    }
                }
            }
        }
    }

    let hash_table = &mut state.hash_table;
    let chain_table = &mut state.chain_table;
    hash_table[hash] = ip as u32;

    while nb_compares > 0 && match_index >= match_low {
        nb_compares -= 1;
        let next_slot = tree_slot(match_index, mask);
        let mut match_length = common_smaller.min(common_larger);
        match_length += count_match_no_dict(
            src,
            ip + match_length,
            match_index + match_length,
            block_end,
        );

        if match_length > best_length {
            if match_length > match_end_idx - match_index {
                match_end_idx = match_index + match_length;
            }
            best_length = match_length;
            matches.push(OptMatch {
                off_base: OffBase::offset_to_c_value((ip - match_index) as u32),
                len: match_length as u32,
            });
            if match_length > ZSTD_OPT_NUM || ip + match_length == block_end {
                break;
            }
        }

        if src[match_index + match_length] < src[ip + match_length] {
            chain_table[smaller_slot] = match_index as u32;
            common_smaller = match_length;
            if match_index <= bt_low {
                smaller_slot = TREE_SLOT_NONE;
                break;
            }
            smaller_slot = next_slot + 1;
            match_index = chain_table[next_slot + 1] as usize;
        } else {
            chain_table[larger_slot] = match_index as u32;
            common_larger = match_length;
            if match_index <= bt_low {
                larger_slot = TREE_SLOT_NONE;
                break;
            }
            larger_slot = next_slot;
            match_index = chain_table[next_slot] as usize;
        }
    }

    write_tree_slot(chain_table, smaller_slot, 0);
    write_tree_slot(chain_table, larger_slot, 0);
    state.next_to_update = match_end_idx - 8;
}

fn insert_and_find_first_index_hash3(
    src: &[u8],
    ip: usize,
    state: &mut GreedyMatchState,
) -> Option<usize> {
    if state.hash_log3 == 0 {
        return None;
    }

    let mut idx = state.next_to_update3;
    while idx < ip {
        state.hash_table3[hash3_ptr(src, idx, state.hash_log3)] = idx as u32;
        idx += 1;
    }

    state.next_to_update3 = ip;
    Some(state.hash_table3[hash3_ptr(src, ip, state.hash_log3)] as usize)
}

#[inline(always)]
fn bt_mask(params: CompressionParameters) -> usize {
    (1_usize << (params.chain_log - 1)) - 1
}

#[inline(always)]
fn tree_slot(index: usize, mask: usize) -> usize {
    2 * (index & mask)
}

#[inline(always)]
fn write_tree_slot(chain_table: &mut [u32], slot: usize, value: u32) {
    if slot != TREE_SLOT_NONE {
        chain_table[slot] = value;
    }
}
