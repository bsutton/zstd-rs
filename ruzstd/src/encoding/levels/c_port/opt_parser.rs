//! No-dictionary optimal parser ported from `zstd_opt.c`.

use alloc::vec::Vec;

use super::{
    greedy::GreedyBlockOutput,
    ldm::opt::LdmOptCursor,
    opt_match::{bt_get_all_matches_no_dict_mls, BtMatchRequest, OptMatchBounds},
    opt_path::{select_path, update_reps},
    opt_state::{OptBlockState, OptParserStrategy, Optimal, HASH_READ_SIZE, ZSTD_OPT_NUM},
    params::CompressionParameters,
    sequence_store::{OffBase, RepeatOffsets, StoredSequence},
};

mod forward;

use forward::{forward_pass, seed_match_prices, seed_parser_root};

pub(crate) fn compress_block_opt_no_dict_with_state(
    src: &[u8],
    block_range: core::ops::Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut OptBlockState,
    strategy: OptParserStrategy,
) -> GreedyBlockOutput {
    compress_block_opt_no_dict_with_state_and_ldm(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        strategy,
        None,
        0,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn compress_block_opt_no_dict_with_state_and_ldm(
    src: &[u8],
    block_range: core::ops::Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut OptBlockState,
    strategy: OptParserStrategy,
    ldm_cursor: Option<&mut LdmOptCursor<'_>>,
    loaded_dict_end: usize,
) -> GreedyBlockOutput {
    let bounds = OptMatchBounds::no_dict(block_range.end, params, loaded_dict_end);
    compress_block_opt_with_state_and_ldm(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        strategy,
        ldm_cursor,
        bounds,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn compress_block_opt_ext_dict_with_state_and_ldm(
    src: &[u8],
    block_range: core::ops::Range<usize>,
    dict_limit: usize,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut OptBlockState,
    strategy: OptParserStrategy,
    ldm_cursor: Option<&mut LdmOptCursor<'_>>,
    loaded_dict_end: usize,
) -> GreedyBlockOutput {
    let bounds = OptMatchBounds::ext_dict(block_range.end, dict_limit, params, loaded_dict_end);
    compress_block_opt_with_state_and_ldm(
        src,
        block_range,
        params,
        repeat_offsets,
        state,
        strategy,
        ldm_cursor,
        bounds,
    )
}

#[allow(clippy::too_many_arguments)]
fn compress_block_opt_with_state_and_ldm(
    src: &[u8],
    block_range: core::ops::Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut OptBlockState,
    strategy: OptParserStrategy,
    ldm_cursor: Option<&mut LdmOptCursor<'_>>,
    bounds: OptMatchBounds,
) -> GreedyBlockOutput {
    match params.min_match.clamp(3, 6) {
        3 => compress_block_opt_with_state_and_ldm_mls::<3>(
            src,
            block_range,
            params,
            repeat_offsets,
            state,
            strategy,
            ldm_cursor,
            bounds,
        ),
        4 => compress_block_opt_with_state_and_ldm_mls::<4>(
            src,
            block_range,
            params,
            repeat_offsets,
            state,
            strategy,
            ldm_cursor,
            bounds,
        ),
        5 => compress_block_opt_with_state_and_ldm_mls::<5>(
            src,
            block_range,
            params,
            repeat_offsets,
            state,
            strategy,
            ldm_cursor,
            bounds,
        ),
        6 => compress_block_opt_with_state_and_ldm_mls::<6>(
            src,
            block_range,
            params,
            repeat_offsets,
            state,
            strategy,
            ldm_cursor,
            bounds,
        ),
        _ => unreachable!("mls is clamped to 3..=6"),
    }
}

#[allow(clippy::too_many_arguments)]
fn compress_block_opt_with_state_and_ldm_mls<const MLS: u32>(
    src: &[u8],
    block_range: core::ops::Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut OptBlockState,
    strategy: OptParserStrategy,
    ldm_cursor: Option<&mut LdmOptCursor<'_>>,
    bounds: OptMatchBounds,
) -> GreedyBlockOutput {
    match strategy {
        OptParserStrategy::BtOpt => compress_block_opt_with_state_and_ldm_mls_level::<MLS, false>(
            src,
            block_range,
            params,
            repeat_offsets,
            state,
            ldm_cursor,
            bounds,
        ),
        OptParserStrategy::BtUltra => compress_block_opt_with_state_and_ldm_mls_level::<MLS, true>(
            src,
            block_range,
            params,
            repeat_offsets,
            state,
            ldm_cursor,
            bounds,
        ),
    }
}

#[allow(clippy::too_many_arguments)]
fn compress_block_opt_with_state_and_ldm_mls_level<const MLS: u32, const ULTRA: bool>(
    src: &[u8],
    block_range: core::ops::Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut OptBlockState,
    mut ldm_cursor: Option<&mut LdmOptCursor<'_>>,
    bounds: OptMatchBounds,
) -> GreedyBlockOutput {
    debug_assert!(block_range.start <= block_range.end);
    debug_assert!(block_range.end <= src.len());

    let block_start = block_range.start;
    let block_end = block_range.end;
    let block_len = block_end - block_start;
    if block_len <= HASH_READ_SIZE {
        return GreedyBlockOutput {
            sequences: Vec::new(),
            last_literals: block_len as u32,
            repeat_offsets,
        };
    }

    // C writes into a reused seqStore; reserve conservatively to avoid repeated
    // growth without allocating for the theoretical maximum sequence density.
    let sequence_capacity = (block_len / (min_match_len::<MLS>() * 4)).min(block_len);
    let mut sequences = Vec::with_capacity(sequence_capacity);
    let mut path = Vec::with_capacity(16);

    state.match_state.ensure_tables(params);
    state.match_state.correct_after_long_match_gap(block_start);
    state.match_state.reset_hash3_cursor_to_primary();
    state.match_state.lazy_skipping = false;
    let opt_level = if ULTRA {
        super::opt_price::OptLevel::BtUltra
    } else {
        super::opt_price::OptLevel::BtOpt
    };
    state
        .price_state
        .rescale_freqs(&src[block_range], opt_level);

    let ilimit = block_end - HASH_READ_SIZE;
    let sufficient_len = params.target_length.min((ZSTD_OPT_NUM - 1) as u32);
    let min_match = if MLS == 3 { 3 } else { 4 };
    let mut rep = repeat_offsets.as_offsets();
    let mut ip = block_start + usize::from(block_start == bounds.prefix_start_index());
    let mut anchor = block_start;

    while ip < ilimit {
        let litlen = ip - anchor;
        let match_count = collect_matches_mls::<MLS>(
            src,
            ip,
            block_end,
            rep,
            litlen == 0,
            min_match,
            params,
            state,
            block_start,
            ldm_cursor.as_deref_mut(),
            bounds,
        );

        if match_count == 0 {
            ip += 1;
            continue;
        }

        let longest = state.matches[match_count - 1];
        seed_parser_root::<ULTRA>(ip, anchor, rep, state);
        path.clear();
        if longest.len > sufficient_len {
            let litlen = (ip - anchor) as u32;
            rep = update_reps(rep, longest.off_base, litlen == 0);
            path.push(Optimal {
                price: 0,
                off: longest.off_base,
                mlen: longest.len,
                litlen,
                rep,
            });
        } else {
            let seeded_last_pos = seed_match_prices::<ULTRA>(min_match, match_count, state);
            let result = forward_pass::<MLS, ULTRA>(
                src,
                ip,
                block_end,
                ilimit,
                seeded_last_pos,
                min_match,
                sufficient_len,
                params,
                state,
                block_start,
                ldm_cursor.as_deref_mut(),
                bounds,
            );

            let empty_stretch = match result.last_stretch {
                Some(stretch) => stretch.mlen == 0,
                None => state.opt[result.last_pos].mlen == 0,
            };
            if empty_stretch {
                ip += result.last_pos;
                continue;
            }

            select_path(
                result.last_pos,
                result.last_stretch,
                &mut rep,
                state,
                &mut path,
            );
        }

        for &step in &path {
            if step.mlen == 0 {
                ip = anchor + step.litlen as usize;
                continue;
            }

            let lit_length = step.litlen;
            let match_length = step.mlen;
            let off_base = step.off;
            let literals = &src[anchor..];
            state
                .price_state
                .update_stats(lit_length, literals, off_base, match_length);
            sequences.push(StoredSequence::new(
                lit_length,
                OffBase::from_c_value(off_base).expect("optimal parser offBase"),
                match_length,
            ));
            anchor += lit_length as usize + match_length as usize;
            ip = anchor;
        }

        state.price_state.refresh_base_prices(opt_level);
    }

    GreedyBlockOutput {
        sequences,
        last_literals: (block_end - anchor) as u32,
        repeat_offsets: RepeatOffsets::from_offsets(rep[0], rep[1], rep[2]),
    }
}

const fn min_match_len<const MLS: u32>() -> usize {
    if MLS == 3 {
        3
    } else {
        4
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn collect_matches(
    src: &[u8],
    ip: usize,
    block_end: usize,
    rep: [u32; 3],
    ll0: bool,
    length_to_beat: u32,
    params: CompressionParameters,
    state: &mut OptBlockState,
    block_start: usize,
    ldm_cursor: Option<&mut LdmOptCursor<'_>>,
    bounds: OptMatchBounds,
) -> usize {
    match params.min_match.clamp(3, 6) {
        3 => collect_matches_mls::<3>(
            src,
            ip,
            block_end,
            rep,
            ll0,
            length_to_beat,
            params,
            state,
            block_start,
            ldm_cursor,
            bounds,
        ),
        4 => collect_matches_mls::<4>(
            src,
            ip,
            block_end,
            rep,
            ll0,
            length_to_beat,
            params,
            state,
            block_start,
            ldm_cursor,
            bounds,
        ),
        5 => collect_matches_mls::<5>(
            src,
            ip,
            block_end,
            rep,
            ll0,
            length_to_beat,
            params,
            state,
            block_start,
            ldm_cursor,
            bounds,
        ),
        6 => collect_matches_mls::<6>(
            src,
            ip,
            block_end,
            rep,
            ll0,
            length_to_beat,
            params,
            state,
            block_start,
            ldm_cursor,
            bounds,
        ),
        _ => unreachable!("mls is clamped to 3..=6"),
    }
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub(super) fn collect_matches_mls<const MLS: u32>(
    src: &[u8],
    ip: usize,
    block_end: usize,
    rep: [u32; 3],
    ll0: bool,
    length_to_beat: u32,
    params: CompressionParameters,
    state: &mut OptBlockState,
    block_start: usize,
    ldm_cursor: Option<&mut LdmOptCursor<'_>>,
    bounds: OptMatchBounds,
) -> usize {
    bt_get_all_matches_no_dict_mls::<MLS>(
        &mut state.matches,
        BtMatchRequest {
            src,
            ip,
            block_end,
            rep: RepeatOffsets::from_offsets(rep[0], rep[1], rep[2]),
            ll0,
            length_to_beat,
            params,
            bounds,
        },
        &mut state.match_state,
    );
    if let Some(cursor) = ldm_cursor {
        debug_assert!(ip >= block_start);
        cursor.process_match_candidate(
            &mut state.matches,
            (ip - block_start) as u32,
            (block_end - ip) as u32,
            length_to_beat,
        );
    }
    state.matches.len()
}
