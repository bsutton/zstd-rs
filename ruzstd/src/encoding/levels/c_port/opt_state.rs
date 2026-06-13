//! Optimal-parser state shared by the no-dictionary C optimal strategies.

use alloc::{vec, vec::Vec};

use super::{
    greedy::GreedyMatchState,
    opt_match::OptMatch,
    opt_price::{OptLevel, OptPriceState, ZSTD_MAX_PRICE},
    params::CompressionParameters,
    sequence_store::RepeatOffsets,
};

pub(super) const HASH_READ_SIZE: usize = 8;
pub(super) const ZSTD_OPT_NUM: usize = 1 << 12;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Optimal {
    pub(super) price: i32,
    pub(super) off: u32,
    pub(super) mlen: u32,
    pub(super) litlen: u32,
    pub(super) rep: [u32; 3],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ForwardResult {
    pub(super) last_pos: usize,
    pub(super) last_stretch: Option<Optimal>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum OptParserStrategy {
    BtOpt,
    BtUltra,
}

#[derive(Clone, Debug)]
pub(crate) struct OptBlockState {
    pub(super) match_state: GreedyMatchState,
    pub(super) price_state: OptPriceState,
    pub(super) matches: Vec<OptMatch>,
    pub(super) opt: Vec<Optimal>,
}

impl Default for Optimal {
    fn default() -> Self {
        Self {
            price: ZSTD_MAX_PRICE,
            off: 0,
            mlen: 0,
            litlen: 0,
            rep: RepeatOffsets::new().as_offsets(),
        }
    }
}

impl OptBlockState {
    pub(crate) fn new() -> Self {
        Self {
            match_state: GreedyMatchState::new(),
            price_state: OptPriceState::new(),
            matches: Vec::new(),
            opt: vec![Optimal::default(); ZSTD_OPT_NUM + 4],
        }
    }

    pub(crate) fn reset_for_frame(&mut self, params: CompressionParameters) {
        self.match_state.reset_for_frame(params);
    }
}

impl OptParserStrategy {
    pub(super) fn opt_level(self) -> OptLevel {
        match self {
            Self::BtOpt => OptLevel::BtOpt,
            Self::BtUltra => OptLevel::BtUltra,
        }
    }
}
