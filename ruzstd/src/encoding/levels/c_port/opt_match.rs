//! Optimal-parser binary-tree match collection ported from `zstd_opt.c`.

use alloc::vec::Vec;

use super::{
    greedy::GreedyMatchState,
    hash_chain_match::{
        count_match, count_match_no_dict, equal_min_match, lowest_prefix_index_with_loaded_dict,
    },
    params::CompressionParameters,
    sequence_store::{OffBase, RepeatCode, RepeatOffsets},
};

mod tree;

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
    pub(super) bounds: OptMatchBounds,
}

#[derive(Clone, Copy)]
pub(super) struct OptMatchBounds {
    dict_limit: usize,
    prefix_start_index: usize,
    loaded_dict_end: usize,
    ext_dict: bool,
}

impl OptMatchBounds {
    pub(super) fn no_dict(
        block_end: usize,
        params: CompressionParameters,
        loaded_dict_end: usize,
    ) -> Self {
        let prefix_start_index =
            lowest_prefix_index_with_loaded_dict(block_end, params.window_log, loaded_dict_end);
        Self {
            dict_limit: prefix_start_index,
            prefix_start_index,
            loaded_dict_end,
            ext_dict: false,
        }
    }

    pub(super) fn ext_dict(
        block_end: usize,
        dict_limit: usize,
        params: CompressionParameters,
        loaded_dict_end: usize,
    ) -> Self {
        let dict_start_index =
            lowest_prefix_index_with_loaded_dict(block_end, params.window_log, loaded_dict_end);
        Self {
            dict_limit,
            prefix_start_index: dict_limit.max(dict_start_index),
            loaded_dict_end,
            ext_dict: true,
        }
    }

    pub(super) fn prefix_start_index(self) -> usize {
        self.prefix_start_index
    }

    fn lowest_match_index(self, pos: usize, params: CompressionParameters) -> usize {
        lowest_prefix_index_with_loaded_dict(pos, params.window_log, self.loaded_dict_end)
    }

    fn rep_match_length(
        self,
        src: &[u8],
        ip: usize,
        rep_offset: usize,
        min_match: u32,
        block_end: usize,
        window_low: usize,
    ) -> Option<usize> {
        if rep_offset == 0 || rep_offset > ip {
            return None;
        }
        let rep_index = ip - rep_offset;
        if self.ext_dict {
            if rep_index >= self.dict_limit {
                if rep_index < window_low {
                    return None;
                }
            } else if rep_offset > ip.saturating_sub(window_low)
                || !index_overlap_check(self.dict_limit, rep_index)
            {
                return None;
            }
        } else if rep_index < window_low {
            return None;
        }

        if rep_index + min_match as usize > src.len()
            || ip + min_match as usize > src.len()
            || !equal_min_match(src, ip, rep_index, min_match)
        {
            return None;
        }

        Some(
            count_match(
                src,
                ip + min_match as usize,
                rep_index + min_match as usize,
                block_end,
            ) + min_match as usize,
        )
    }
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

    let params = request.params;
    debug_assert_tables_ready(state, params);
    let mls = params.min_match.clamp(3, 6);
    match mls {
        3 => bt_get_all_matches_no_dict_mls::<3>(matches, request, state),
        4 => bt_get_all_matches_no_dict_mls::<4>(matches, request, state),
        5 => bt_get_all_matches_no_dict_mls::<5>(matches, request, state),
        6 => bt_get_all_matches_no_dict_mls::<6>(matches, request, state),
        _ => unreachable!("mls is clamped to 3..=6"),
    }
}

fn bt_get_all_matches_no_dict_mls<const MLS: u32>(
    matches: &mut Vec<OptMatch>,
    request: BtMatchRequest<'_>,
    state: &mut GreedyMatchState,
) {
    tree::bt_get_all_matches_no_dict_mls::<MLS>(matches, request, state);
}

pub(super) fn update_tree_no_dict(
    src: &[u8],
    target: usize,
    block_end: usize,
    mls: u32,
    params: CompressionParameters,
    state: &mut GreedyMatchState,
    loaded_dict_end: usize,
) {
    match mls {
        3 => tree::update_tree_no_dict_mls::<3>(
            src,
            target,
            block_end,
            params,
            state,
            loaded_dict_end,
        ),
        4 => tree::update_tree_no_dict_mls::<4>(
            src,
            target,
            block_end,
            params,
            state,
            loaded_dict_end,
        ),
        5 => tree::update_tree_no_dict_mls::<5>(
            src,
            target,
            block_end,
            params,
            state,
            loaded_dict_end,
        ),
        6 => tree::update_tree_no_dict_mls::<6>(
            src,
            target,
            block_end,
            params,
            state,
            loaded_dict_end,
        ),
        _ => unreachable!("mls is clamped to 3..=6"),
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn collect_repcode_matches(
    matches: &mut Vec<OptMatch>,
    src: &[u8],
    ip: usize,
    block_end: usize,
    rep: RepeatOffsets,
    ll0: bool,
    min_match: u32,
    params: CompressionParameters,
    bounds: OptMatchBounds,
    sufficient_len: usize,
    best_length: &mut usize,
) {
    let offsets = rep.as_offsets();
    let window_low = bounds.lowest_match_index(ip, params);
    if !bounds.ext_dict {
        collect_repcode_matches_no_dict(
            matches,
            src,
            ip,
            block_end,
            offsets,
            ll0,
            min_match,
            window_low,
            sufficient_len,
            best_length,
        );
        return;
    }

    let first_rep = usize::from(ll0);
    let last_rep = 3 + usize::from(ll0);
    for rep_code in first_rep..last_rep {
        let rep_offset = if rep_code == 3 {
            offsets[0].saturating_sub(1)
        } else {
            offsets[rep_code]
        } as usize;
        let Some(rep_len) =
            bounds.rep_match_length(src, ip, rep_offset, min_match, block_end, window_low)
        else {
            continue;
        };
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

#[allow(clippy::too_many_arguments)]
fn collect_repcode_matches_no_dict(
    matches: &mut Vec<OptMatch>,
    src: &[u8],
    ip: usize,
    block_end: usize,
    offsets: [u32; 3],
    ll0: bool,
    min_match: u32,
    window_low: usize,
    sufficient_len: usize,
    best_length: &mut usize,
) {
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

        let rep_len = count_match_no_dict(
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

pub(super) fn should_stop_after_best_match(
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

fn index_overlap_check(prefix_lowest_index: usize, rep_index: usize) -> bool {
    prefix_lowest_index.wrapping_sub(1).wrapping_sub(rep_index) >= 3
}

#[cfg(debug_assertions)]
fn debug_assert_tables_ready(state: &GreedyMatchState, params: CompressionParameters) {
    debug_assert_eq!(state.hash_log, params.hash_log);
    debug_assert_eq!(state.chain_log, params.chain_log);
    debug_assert_eq!(state.hash_table.len(), 1_usize << params.hash_log);
    debug_assert_eq!(state.chain_table.len(), 1_usize << params.chain_log);

    let hash_log3 = if params.min_match == 3 {
        params.window_log.min(17)
    } else {
        0
    };
    debug_assert_eq!(state.hash_log3, hash_log3);
    let hash3_size = if hash_log3 > 0 {
        1_usize << hash_log3
    } else {
        0
    };
    debug_assert_eq!(state.hash_table3.len(), hash3_size);
}

#[cfg(not(debug_assertions))]
#[inline(always)]
fn debug_assert_tables_ready(_state: &GreedyMatchState, _params: CompressionParameters) {}
