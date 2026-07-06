use super::{
    hash_chain_match::highbit32,
    opt_price::{DictionaryPriceSeeds, OptLevel, OptPriceState, BITCOST_MULTIPLIER},
    sequence_store::OffBase,
};
use crate::encoding::blocks::{literal_length_code, match_length_code};

#[test]
fn opt_price_uses_predefined_prices_for_tiny_first_block() {
    let mut state = OptPriceState::new();
    state.rescale_freqs(b"abcdefgh", OptLevel::BtOpt);

    assert_eq!(
        state.raw_literals_cost(b"abc", OptLevel::BtOpt),
        3 * 6 * BITCOST_MULTIPLIER
    );
    assert_eq!(
        state.lit_length_price(3, OptLevel::BtOpt),
        2 * BITCOST_MULTIPLIER
    );
    assert_eq!(
        state.match_price(OffBase::Offset(10).to_c_value(), 10, OptLevel::BtOpt),
        22 * BITCOST_MULTIPLIER
    );
}

#[test]
fn opt_price_uses_fractional_weights_for_ultra_levels() {
    let mut state = OptPriceState::new();
    state.rescale_freqs(b"abcdefgh", OptLevel::BtUltra);

    assert!(
        state.lit_length_price(3, OptLevel::BtUltra) > state.lit_length_price(3, OptLevel::BtOpt)
    );
}

#[test]
fn single_literal_cost_matches_slice_cost() {
    let mut state = OptPriceState::new();
    state.rescale_freqs(b"abcabcabcabc", OptLevel::BtOpt);

    for literal in [b'a', b'b', b'z'] {
        assert_eq!(
            state.raw_literal_cost(literal, OptLevel::BtOpt),
            state.raw_literals_cost(&[literal], OptLevel::BtOpt)
        );
    }
}

#[test]
fn literal_length_increment_matches_price_difference() {
    for opt_level in [OptLevel::BtOpt, OptLevel::BtUltra] {
        let mut state = OptPriceState::new();
        state.rescale_freqs(b"abcabcabcabcabcabcabcabc", opt_level);
        for lit_length in 1..=1024 {
            let expected = state.lit_length_price(lit_length, opt_level) as i64
                - state.lit_length_price(lit_length - 1, opt_level) as i64;
            assert_eq!(
                state.lit_length_increment_price(lit_length, opt_level),
                expected as i32,
                "lit_length={lit_length} opt_level={opt_level:?}"
            );
        }
    }
}

#[test]
fn opt_price_updates_sequence_statistics() {
    let mut state = OptPriceState::new();
    state.rescale_freqs(b"abcabcabcabc", OptLevel::BtOpt);

    let off_base = OffBase::Offset(10).to_c_value();
    let ll_code = literal_length_code(3) as usize;
    let ml_code = match_length_code(10) as usize;
    let off_code = highbit32(off_base) as usize;
    let before = state.frequency_snapshot(ll_code, ml_code, off_code);

    state.update_stats(3, b"abc", off_base, 10);
    let after = state.frequency_snapshot(ll_code, ml_code, off_code);

    assert_eq!(after, (before.0 + 1, before.1 + 1, before.2 + 1));
}

#[test]
fn opt_price_uses_dictionary_seeds_on_first_block() {
    let mut seeds = uniform_dictionary_seeds();
    let ll_code = 7;
    let ml_code = 11;
    let off_code = 13;
    seeds.set_lit_length_freq(ll_code, 64);
    seeds.set_match_length_freq(ml_code, 32);
    seeds.set_off_code_freq(off_code, 16);

    let mut state = OptPriceState::new();
    state.set_dictionary_seeds(seeds);
    state.rescale_freqs(b"abcdefgh", OptLevel::BtOpt);

    assert_eq!(
        state.frequency_snapshot(ll_code, ml_code, off_code),
        (64, 32, 16)
    );
}

#[test]
fn opt_price_reset_for_frame_restarts_first_block_statistics_like_c() {
    let mut state = OptPriceState::new();
    state.rescale_freqs(b"abcabcabcabc", OptLevel::BtOpt);
    state.update_stats(3, b"abc", OffBase::Offset(10).to_c_value(), 10);

    state.reset_for_frame();
    state.rescale_freqs(b"abcdefgh", OptLevel::BtOpt);

    assert_eq!(
        state.raw_literals_cost(b"abc", OptLevel::BtOpt),
        3 * 6 * BITCOST_MULTIPLIER
    );
}

#[test]
fn opt_price_reset_for_frame_preserves_staged_dictionary_seeds() {
    let mut seeds = uniform_dictionary_seeds();
    let ll_code = 9;
    let ml_code = 12;
    let off_code = 15;
    seeds.set_lit_length_freq(ll_code, 77);
    seeds.set_match_length_freq(ml_code, 55);
    seeds.set_off_code_freq(off_code, 33);

    let mut state = OptPriceState::new();
    state.set_dictionary_seeds(seeds);
    state.reset_for_frame();
    state.rescale_freqs(b"abcdefgh", OptLevel::BtOpt);

    assert_eq!(
        state.frequency_snapshot(ll_code, ml_code, off_code),
        (77, 55, 33)
    );
}

fn uniform_dictionary_seeds() -> DictionaryPriceSeeds {
    let mut seeds = DictionaryPriceSeeds::new();
    for symbol in 0..DictionaryPriceSeeds::LITERAL_COUNT {
        seeds.set_literal_freq(symbol, 1);
    }
    for symbol in 0..DictionaryPriceSeeds::LIT_LENGTH_COUNT {
        seeds.set_lit_length_freq(symbol, 1);
    }
    for symbol in 0..DictionaryPriceSeeds::MATCH_LENGTH_COUNT {
        seeds.set_match_length_freq(symbol, 1);
    }
    for symbol in 0..DictionaryPriceSeeds::OFF_CODE_COUNT {
        seeds.set_off_code_freq(symbol, 1);
    }
    seeds
}
