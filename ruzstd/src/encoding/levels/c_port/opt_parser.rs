//! No-dictionary optimal parser ported from `zstd_opt.c`.

use alloc::{vec, vec::Vec};
use core::convert::TryFrom;

use super::{
    greedy::GreedyBlockOutput,
    hash_chain_match::lowest_prefix_index,
    opt_match::{bt_get_all_matches_no_dict, BtMatchRequest, OptMatch},
    opt_price::{OptLevel, OptPriceState, BITCOST_MULTIPLIER, ZSTD_MAX_PRICE},
    opt_state::{
        ForwardResult, OptBlockState, OptParserStrategy, Optimal, HASH_READ_SIZE, ZSTD_OPT_NUM,
    },
    params::CompressionParameters,
    sequence_store::{OffBase, RepeatOffsets, StoredSequence},
};

pub(crate) fn compress_block_opt_no_dict_with_state(
    src: &[u8],
    block_range: core::ops::Range<usize>,
    params: CompressionParameters,
    repeat_offsets: RepeatOffsets,
    state: &mut OptBlockState,
    strategy: OptParserStrategy,
) -> GreedyBlockOutput {
    debug_assert!(block_range.start <= block_range.end);
    debug_assert!(block_range.end <= src.len());

    let block_start = block_range.start;
    let block_end = block_range.end;
    let block_len = block_end - block_start;
    let mut sequences = Vec::new();

    if block_len <= HASH_READ_SIZE {
        return GreedyBlockOutput {
            sequences,
            last_literals: block_len as u32,
            repeat_offsets,
        };
    }

    state.match_state.ensure_tables(params);
    state.match_state.correct_after_long_match_gap(block_start);
    state.match_state.reset_hash3_cursor_to_primary();
    state.match_state.lazy_skipping = false;
    let opt_level = strategy.opt_level();
    state
        .price_state
        .rescale_freqs(&src[block_range], opt_level);

    let prefix_lowest = lowest_prefix_index(block_end, params.window_log);
    let ilimit = block_end - HASH_READ_SIZE;
    let sufficient_len = params.target_length.min((ZSTD_OPT_NUM - 1) as u32);
    let min_match = if params.min_match == 3 { 3 } else { 4 };
    let mut rep = repeat_offsets.as_offsets();
    let mut ip = block_start + usize::from(block_start == prefix_lowest);
    let mut anchor = block_start;

    while ip < ilimit {
        let litlen = ip - anchor;
        let match_count = collect_matches(
            src,
            ip,
            block_end,
            rep,
            litlen == 0,
            min_match,
            params,
            state,
        );

        if match_count == 0 {
            ip += 1;
            continue;
        }

        let longest = state.matches[match_count - 1];
        seed_parser_root(ip, anchor, rep, opt_level, state);
        let path = if longest.len > sufficient_len {
            let litlen = (ip - anchor) as u32;
            rep = update_reps(rep, longest.off_base, litlen == 0);
            vec![Optimal {
                price: 0,
                off: longest.off_base,
                mlen: longest.len,
                litlen,
                rep,
            }]
        } else {
            seed_match_prices(min_match, match_count, opt_level, state);
            let result = forward_pass(
                src,
                ip,
                block_end,
                ilimit,
                min_match,
                sufficient_len,
                params,
                opt_level,
                state,
            );

            let empty_stretch = match result.last_stretch {
                Some(stretch) => stretch.mlen == 0,
                None => state.opt[result.last_pos].mlen == 0,
            };
            if empty_stretch {
                ip += result.last_pos;
                continue;
            }

            select_path(result.last_pos, result.last_stretch, &mut rep, state)
        };

        for step in path {
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

#[allow(clippy::too_many_arguments)]
fn collect_matches(
    src: &[u8],
    ip: usize,
    block_end: usize,
    rep: [u32; 3],
    ll0: bool,
    length_to_beat: u32,
    params: CompressionParameters,
    state: &mut OptBlockState,
) -> usize {
    bt_get_all_matches_no_dict(
        &mut state.matches,
        BtMatchRequest {
            src,
            ip,
            block_end,
            rep: RepeatOffsets::from_offsets(rep[0], rep[1], rep[2]),
            ll0,
            length_to_beat,
            params,
        },
        &mut state.match_state,
    );
    state.matches.len()
}

fn seed_parser_root(
    ip: usize,
    anchor: usize,
    rep: [u32; 3],
    opt_level: OptLevel,
    state: &mut OptBlockState,
) {
    let litlen = (ip - anchor) as u32;
    state.opt[0] = Optimal {
        price: price_i32(state.price_state.lit_length_price(litlen, opt_level)),
        off: 0,
        mlen: 0,
        litlen,
        rep,
    };
}

fn seed_match_prices(
    min_match: u32,
    match_count: usize,
    opt_level: OptLevel,
    state: &mut OptBlockState,
) {
    let litlen = state.opt[0].litlen;
    let rep = state.opt[0].rep;
    for pos in 1..min_match as usize {
        state.opt[pos] = Optimal {
            price: ZSTD_MAX_PRICE,
            mlen: 0,
            litlen: litlen + pos as u32,
            rep,
            ..Optimal::default()
        };
    }

    let mut last_len = min_match;
    for match_index in 0..match_count {
        let OptMatch { off_base, len } = state.matches[match_index];
        for pos in last_len..=len {
            state.opt[pos as usize] = Optimal {
                price: state.opt[0].price
                    + price_i32(state.price_state.match_price(off_base, pos, opt_level))
                    + price_i32(state.price_state.lit_length_price(0, opt_level)),
                off: off_base,
                mlen: pos,
                litlen: 0,
                rep,
            };
        }
        last_len = len + 1;
    }

    let last_pos = last_len.saturating_sub(1) as usize;
    state.opt[last_pos + 1] = Optimal::default();
}

#[allow(clippy::too_many_arguments)]
fn forward_pass(
    src: &[u8],
    ip: usize,
    block_end: usize,
    ilimit: usize,
    min_match: u32,
    sufficient_len: u32,
    params: CompressionParameters,
    opt_level: OptLevel,
    state: &mut OptBlockState,
) -> ForwardResult {
    let mut last_pos = seeded_last_pos(state);
    let mut last_stretch = None;
    let mut cur = 1_usize;

    while cur <= last_pos {
        update_literal_price(src, ip, block_end, cur, &mut last_pos, opt_level, state);
        refresh_node_reps(cur, state);

        let inr = ip + cur;
        if inr > ilimit {
            break;
        }
        if cur == last_pos {
            break;
        }
        if opt_level == OptLevel::BtOpt
            && state.opt[cur + 1].price <= state.opt[cur].price + price_i32(BITCOST_MULTIPLIER / 2)
        {
            cur += 1;
            continue;
        }

        let rep = state.opt[cur].rep;
        let ll0 = state.opt[cur].litlen == 0;
        let match_count = collect_matches(src, inr, block_end, rep, ll0, min_match, params, state);
        if match_count == 0 {
            cur += 1;
            continue;
        }

        let longest = state.matches[match_count - 1];
        if longest.len > sufficient_len
            || cur + longest.len as usize >= ZSTD_OPT_NUM
            || inr + longest.len as usize >= block_end
        {
            last_pos = cur + longest.len as usize;
            last_stretch = Some(Optimal {
                price: state.opt[cur].price,
                off: longest.off_base,
                mlen: longest.len,
                litlen: 0,
                rep,
            });
            break;
        }

        update_match_prices(cur, min_match, match_count, &mut last_pos, opt_level, state);
        cur += 1;
    }

    ForwardResult {
        last_pos,
        last_stretch,
    }
}

fn update_literal_price(
    src: &[u8],
    ip: usize,
    block_end: usize,
    cur: usize,
    last_pos: &mut usize,
    opt_level: OptLevel,
    state: &mut OptBlockState,
) {
    let previous = state.opt[cur - 1];
    let litlen = previous.litlen + 1;
    let price = previous.price
        + price_i32(
            state
                .price_state
                .raw_literals_cost(&src[ip + cur - 1..ip + cur], opt_level),
        )
        + ll_increment_price(litlen, opt_level, &state.price_state);

    if price <= state.opt[cur].price {
        let prev_match = state.opt[cur];
        state.opt[cur] = Optimal {
            price,
            litlen,
            ..previous
        };

        if opt_level == OptLevel::BtUltra
            && prev_match.litlen == 0
            && ll_increment_price(1, opt_level, &state.price_state) < 0
            && ip + cur < block_end
        {
            let with_one_literal = prev_match.price
                + price_i32(
                    state
                        .price_state
                        .raw_literals_cost(&src[ip + cur..ip + cur + 1], opt_level),
                )
                + ll_increment_price(1, opt_level, &state.price_state);
            let with_more_literals = price
                + price_i32(
                    state
                        .price_state
                        .raw_literals_cost(&src[ip + cur..ip + cur + 1], opt_level),
                )
                + ll_increment_price(litlen + 1, opt_level, &state.price_state);
            if with_one_literal < with_more_literals && with_one_literal < state.opt[cur + 1].price
            {
                let prev = cur - prev_match.mlen as usize;
                state.opt[cur + 1] = Optimal {
                    price: with_one_literal,
                    litlen: 1,
                    rep: update_reps(
                        state.opt[prev].rep,
                        prev_match.off,
                        state.opt[prev].litlen == 0,
                    ),
                    ..prev_match
                };
                *last_pos = (*last_pos).max(cur + 1);
            }
        }
    }
}

fn refresh_node_reps(cur: usize, state: &mut OptBlockState) {
    if state.opt[cur].litlen != 0 || state.opt[cur].mlen == 0 {
        return;
    }

    let previous_index = cur - state.opt[cur].mlen as usize;
    state.opt[cur].rep = update_reps(
        state.opt[previous_index].rep,
        state.opt[cur].off,
        state.opt[previous_index].litlen == 0,
    );
}

fn update_match_prices(
    cur: usize,
    min_match: u32,
    match_count: usize,
    last_pos: &mut usize,
    opt_level: OptLevel,
    state: &mut OptBlockState,
) {
    let base_price =
        state.opt[cur].price + price_i32(state.price_state.lit_length_price(0, opt_level));
    let mut previous_len = min_match;

    for match_index in 0..match_count {
        let OptMatch { off_base, len } = state.matches[match_index];
        let start_len = previous_len;
        let mut match_len = len;
        while match_len >= start_len {
            let pos = cur + match_len as usize;
            let price = base_price
                + price_i32(
                    state
                        .price_state
                        .match_price(off_base, match_len, opt_level),
                );

            if pos > *last_pos || price < state.opt[pos].price {
                while *last_pos < pos {
                    *last_pos += 1;
                    state.opt[*last_pos] = Optimal {
                        price: ZSTD_MAX_PRICE,
                        litlen: u32::MAX,
                        ..Optimal::default()
                    };
                }
                state.opt[pos] = Optimal {
                    price,
                    off: off_base,
                    mlen: match_len,
                    litlen: 0,
                    rep: state.opt[cur].rep,
                };
            } else {
                if opt_level == OptLevel::BtOpt {
                    break;
                }
            }

            if match_len == start_len {
                break;
            }
            match_len -= 1;
        }
        previous_len = len + 1;
    }
}

fn seeded_last_pos(state: &OptBlockState) -> usize {
    let mut last_pos = 1_usize;
    while state.opt[last_pos].price != ZSTD_MAX_PRICE {
        last_pos += 1;
    }
    last_pos - 1
}

fn select_path(
    last_pos: usize,
    last_stretch: Option<Optimal>,
    rep: &mut [u32; 3],
    state: &mut OptBlockState,
) -> Vec<Optimal> {
    let mut path = Vec::new();
    let stretch = last_stretch.unwrap_or(state.opt[last_pos]);
    let mut cur = last_pos - stretch.mlen as usize;

    if stretch.litlen == 0 {
        *rep = update_reps(state.opt[cur].rep, stretch.off, state.opt[cur].litlen == 0);
    } else {
        *rep = stretch.rep;
        cur -= stretch.litlen as usize;
    }

    path.push(stretch);
    let mut stretch_pos = cur;
    loop {
        let next = state.opt[stretch_pos];
        if let Some(last) = path.last_mut() {
            last.litlen = next.litlen;
        }
        if next.mlen == 0 {
            break;
        }
        path.push(next);
        stretch_pos -= next.litlen as usize + next.mlen as usize;
    }

    path.reverse();
    path
}

fn update_reps(rep: [u32; 3], off_base: u32, previous_litlen_zero: bool) -> [u32; 3] {
    let mut repeat_offsets = RepeatOffsets::from_offsets(rep[0], rep[1], rep[2]);
    repeat_offsets.update(
        OffBase::from_c_value(off_base).expect("optimal parser rep offBase"),
        u32::from(!previous_litlen_zero),
    );
    repeat_offsets.as_offsets()
}

fn ll_increment_price(litlen: u32, opt_level: OptLevel, price_state: &OptPriceState) -> i32 {
    price_i32(price_state.lit_length_price(litlen, opt_level))
        - price_i32(price_state.lit_length_price(litlen - 1, opt_level))
}

fn price_i32(price: u32) -> i32 {
    i32::try_from(price).unwrap_or(ZSTD_MAX_PRICE)
}
