//! Optimal-parser binary-tree match collection ported from `zstd_opt.c`.

use alloc::vec::Vec;

use super::{
    greedy::GreedyMatchState,
    hash_chain_match::{count_match, equal_min_match, hash3_ptr, hash_ptr, lowest_prefix_index},
    params::CompressionParameters,
    sequence_store::{OffBase, RepeatCode, RepeatOffsets},
};

const ZSTD_OPT_NUM: usize = 1 << 12;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct OptMatch {
    pub(super) off_base: u32,
    pub(super) len: u32,
}

#[derive(Clone, Copy)]
pub(super) struct BtMatchRequest<'a> {
    pub(super) src: &'a [u8],
    pub(super) ip: usize,
    pub(super) block_end: usize,
    pub(super) rep: RepeatOffsets,
    pub(super) ll0: bool,
    pub(super) length_to_beat: u32,
    pub(super) params: CompressionParameters,
}

pub(super) fn bt_get_all_matches_no_dict(
    matches: &mut Vec<OptMatch>,
    request: BtMatchRequest<'_>,
    state: &mut GreedyMatchState,
) {
    matches.clear();
    if request.ip < state.next_to_update {
        return;
    }

    let BtMatchRequest {
        src,
        ip,
        block_end,
        rep,
        ll0,
        length_to_beat,
        params,
    } = request;

    state.ensure_tables(params);
    let mls = params.min_match.clamp(3, 6);
    update_tree_no_dict(src, ip, block_end, mls, params, state);
    insert_bt_and_get_all_matches_no_dict(
        matches,
        src,
        ip,
        block_end,
        rep,
        ll0,
        length_to_beat,
        mls,
        params,
        state,
    );
}

pub(super) fn update_tree_no_dict(
    src: &[u8],
    target: usize,
    block_end: usize,
    mls: u32,
    params: CompressionParameters,
    state: &mut GreedyMatchState,
) {
    let mut idx = state.next_to_update;
    while idx < target {
        let forward = insert_bt1_no_dict(src, idx, block_end, target, mls, params, state);
        debug_assert!(forward > 0);
        idx += forward;
    }
    state.next_to_update = target;
}

fn insert_bt1_no_dict(
    src: &[u8],
    ip: usize,
    block_end: usize,
    target: usize,
    mls: u32,
    params: CompressionParameters,
    state: &mut GreedyMatchState,
) -> usize {
    let hash = hash_ptr(src, ip, params.hash_log, mls);
    let mut match_index = state.hash_table[hash] as usize;
    let mask = bt_mask(params);
    let bt_low = ip.saturating_sub(mask);
    let window_low = lowest_prefix_index(target, params.window_log).max(1);
    let mut common_smaller = 0_usize;
    let mut common_larger = 0_usize;
    let mut smaller_slot = Some(tree_slot(ip, mask));
    let mut larger_slot = Some(tree_slot(ip, mask) + 1);
    let mut match_end_idx = ip + 9;
    let mut best_length = 8_usize;
    let mut nb_compares = 1_usize << params.search_log;

    state.hash_table[hash] = ip as u32;

    while nb_compares > 0 && match_index >= window_low {
        nb_compares -= 1;
        let next_slot = tree_slot(match_index, mask);
        let mut match_length = common_smaller.min(common_larger);
        match_length += count_match(
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
            write_tree_slot(state, smaller_slot, match_index as u32);
            common_smaller = match_length;
            if match_index <= bt_low {
                smaller_slot = None;
                break;
            }
            smaller_slot = Some(next_slot + 1);
            match_index = state.chain_table[next_slot + 1] as usize;
        } else {
            write_tree_slot(state, larger_slot, match_index as u32);
            common_larger = match_length;
            if match_index <= bt_low {
                larger_slot = None;
                break;
            }
            larger_slot = Some(next_slot);
            match_index = state.chain_table[next_slot] as usize;
        }
    }

    write_tree_slot(state, smaller_slot, 0);
    write_tree_slot(state, larger_slot, 0);

    let positions = best_length.saturating_sub(384).min(192);
    positions.max(match_end_idx - (ip + 8))
}

#[allow(clippy::too_many_arguments)]
fn insert_bt_and_get_all_matches_no_dict(
    matches: &mut Vec<OptMatch>,
    src: &[u8],
    ip: usize,
    block_end: usize,
    rep: RepeatOffsets,
    ll0: bool,
    length_to_beat: u32,
    mls: u32,
    params: CompressionParameters,
    state: &mut GreedyMatchState,
) {
    let sufficient_len = params.target_length.min((ZSTD_OPT_NUM - 1) as u32) as usize;
    let min_match = if mls == 3 { 3 } else { 4 };
    let hash = hash_ptr(src, ip, params.hash_log, mls);
    let mut match_index = state.hash_table[hash] as usize;
    let mask = bt_mask(params);
    let bt_low = ip.saturating_sub(mask);
    let window_low = lowest_prefix_index(ip, params.window_log);
    let match_low = window_low.max(1);
    let mut common_smaller = 0_usize;
    let mut common_larger = 0_usize;
    let mut smaller_slot = Some(tree_slot(ip, mask));
    let mut larger_slot = Some(tree_slot(ip, mask) + 1);
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
        window_low,
        sufficient_len,
        &mut best_length,
    );
    if should_stop_after_best_match(matches, ip, block_end, sufficient_len) {
        return;
    }

    if mls == 3 && best_length < 3 {
        if let Some(match_index3) = insert_and_find_first_index_hash3(src, ip, state) {
            let within_price_heuristic = ip - match_index3 < (1 << 18);
            if match_index3 >= match_low && within_price_heuristic {
                let len = count_match(src, ip, match_index3, block_end);
                if len >= 3 {
                    best_length = len;
                    matches.clear();
                    matches.push(OptMatch {
                        off_base: OffBase::from_offset((ip - match_index3) as u32)
                            .expect("hash3 match has non-zero offset")
                            .to_c_value(),
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

    state.hash_table[hash] = ip as u32;

    while nb_compares > 0 && match_index >= match_low {
        nb_compares -= 1;
        let next_slot = tree_slot(match_index, mask);
        let mut match_length = common_smaller.min(common_larger);
        match_length += count_match(
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
                off_base: OffBase::from_offset((ip - match_index) as u32)
                    .expect("binary-tree match has non-zero offset")
                    .to_c_value(),
                len: match_length as u32,
            });
            if match_length > ZSTD_OPT_NUM || ip + match_length == block_end {
                break;
            }
        }

        if src[match_index + match_length] < src[ip + match_length] {
            write_tree_slot(state, smaller_slot, match_index as u32);
            common_smaller = match_length;
            if match_index <= bt_low {
                smaller_slot = None;
                break;
            }
            smaller_slot = Some(next_slot + 1);
            match_index = state.chain_table[next_slot + 1] as usize;
        } else {
            write_tree_slot(state, larger_slot, match_index as u32);
            common_larger = match_length;
            if match_index <= bt_low {
                larger_slot = None;
                break;
            }
            larger_slot = Some(next_slot);
            match_index = state.chain_table[next_slot] as usize;
        }
    }

    write_tree_slot(state, smaller_slot, 0);
    write_tree_slot(state, larger_slot, 0);
    state.next_to_update = match_end_idx - 8;
}

#[allow(clippy::too_many_arguments)]
fn collect_repcode_matches(
    matches: &mut Vec<OptMatch>,
    src: &[u8],
    ip: usize,
    block_end: usize,
    rep: RepeatOffsets,
    ll0: bool,
    min_match: u32,
    window_low: usize,
    sufficient_len: usize,
    best_length: &mut usize,
) {
    let offsets = rep.as_offsets();
    let first_rep = usize::from(ll0);
    let last_rep = 3 + usize::from(ll0);
    for rep_code in first_rep..last_rep {
        let rep_offset = if rep_code == 3 {
            offsets[0].saturating_sub(1)
        } else {
            offsets[rep_code]
        } as usize;
        if rep_offset == 0 || rep_offset > ip {
            continue;
        }
        let rep_index = ip - rep_offset;
        if rep_index < window_low || !equal_min_match(src, ip, rep_index, min_match) {
            continue;
        }

        let rep_len = count_match(
            src,
            ip + min_match as usize,
            rep_index + min_match as usize,
            block_end,
        ) + min_match as usize;
        if rep_len > *best_length {
            *best_length = rep_len;
            matches.push(OptMatch {
                off_base: repcode_to_off_base(rep_code - first_rep + 1),
                len: rep_len as u32,
            });
            if rep_len > sufficient_len || ip + rep_len == block_end {
                break;
            }
        }
    }
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

fn should_stop_after_best_match(
    matches: &[OptMatch],
    ip: usize,
    block_end: usize,
    sufficient_len: usize,
) -> bool {
    matches.last().is_some_and(|best| {
        best.len as usize > sufficient_len || ip + best.len as usize == block_end
    })
}

fn repcode_to_off_base(code: usize) -> u32 {
    match code {
        1 => OffBase::Repeat(RepeatCode::First),
        2 => OffBase::Repeat(RepeatCode::Second),
        3 => OffBase::Repeat(RepeatCode::Third),
        _ => unreachable!("C repcode value is between 1 and 3"),
    }
    .to_c_value()
}

fn bt_mask(params: CompressionParameters) -> usize {
    (1_usize << (params.chain_log - 1)) - 1
}

fn tree_slot(index: usize, mask: usize) -> usize {
    2 * (index & mask)
}

fn write_tree_slot(state: &mut GreedyMatchState, slot: Option<usize>, value: u32) {
    if let Some(slot) = slot {
        state.chain_table[slot] = value;
    }
}
