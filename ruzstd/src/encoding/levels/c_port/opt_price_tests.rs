use super::{
    hash_chain_match::highbit32,
    opt_price::{OptLevel, OptPriceState, BITCOST_MULTIPLIER},
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
