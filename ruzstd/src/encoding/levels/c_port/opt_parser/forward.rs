use core::convert::TryFrom;

use crate::encoding::levels::c_port::{
    ldm::opt::LdmOptCursor,
    opt_match::{OptMatch, OptMatchBounds},
    opt_path::update_reps,
    opt_price::{OptLevel, OptPriceState, BITCOST_MULTIPLIER, ZSTD_MAX_PRICE},
    opt_state::{ForwardResult, OptBlockState, Optimal, ZSTD_OPT_NUM},
    params::CompressionParameters,
};

pub(super) fn seed_parser_root(
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

pub(super) fn seed_match_prices(
    min_match: u32,
    match_count: usize,
    opt_level: OptLevel,
    state: &mut OptBlockState,
) -> usize {
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
    last_pos
}

#[allow(clippy::too_many_arguments)]
pub(super) fn forward_pass<const MLS: u32>(
    src: &[u8],
    ip: usize,
    block_end: usize,
    ilimit: usize,
    mut last_pos: usize,
    min_match: u32,
    sufficient_len: u32,
    params: CompressionParameters,
    opt_level: OptLevel,
    state: &mut OptBlockState,
    block_start: usize,
    mut ldm_cursor: Option<&mut LdmOptCursor<'_>>,
    bounds: OptMatchBounds,
) -> ForwardResult {
    let mut last_stretch = None;
    let mut cur = 1_usize;

    while cur <= last_pos {
        if cur > ZSTD_OPT_NUM {
            break;
        }
        update_literal_price(src, ip, block_end, cur, &mut last_pos, opt_level, state);
        refresh_node_reps(cur, state);

        let inr = ip + cur;
        // C still lets later priced positions collapse into literals here.
        if inr > ilimit {
            cur += 1;
            continue;
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
        let match_count = super::collect_matches_mls::<MLS>(
            src,
            inr,
            block_end,
            rep,
            ll0,
            min_match,
            params,
            state,
            block_start,
            ldm_cursor.as_deref_mut(),
            bounds,
        );
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
        // C refreshes the sentinel after each match-price update so stale
        // prices beyond the current frontier cannot influence later literals.
        state.opt[last_pos + 1].price = ZSTD_MAX_PRICE;
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
    let litlen_increment = ll_increment_price(litlen, opt_level, &state.price_state);
    let price = previous.price
        + price_i32(
            state
                .price_state
                .raw_literal_cost(src[ip + cur - 1], opt_level),
        )
        + litlen_increment;

    if price <= state.opt[cur].price {
        let prev_match = state.opt[cur];
        state.opt[cur] = Optimal {
            price,
            litlen,
            ..previous
        };

        let one_literal_increment = if opt_level == OptLevel::BtUltra {
            ll_increment_price(1, opt_level, &state.price_state)
        } else {
            0
        };
        if opt_level == OptLevel::BtUltra
            && prev_match.litlen == 0
            && one_literal_increment < 0
            && ip + cur < block_end
        {
            let next_literal_cost =
                price_i32(state.price_state.raw_literal_cost(src[ip + cur], opt_level));
            let with_one_literal = prev_match.price + next_literal_cost + one_literal_increment;
            let with_more_literals = price
                + next_literal_cost
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
            } else if opt_level == OptLevel::BtOpt {
                break;
            }

            if match_len == start_len {
                break;
            }
            match_len -= 1;
        }
        previous_len = len + 1;
    }
}

#[inline(always)]
fn ll_increment_price(litlen: u32, opt_level: OptLevel, price_state: &OptPriceState) -> i32 {
    price_state.lit_length_increment_price(litlen, opt_level)
}

fn price_i32(price: u32) -> i32 {
    i32::try_from(price).unwrap_or(ZSTD_MAX_PRICE)
}
