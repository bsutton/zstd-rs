//! Strategy-dispatching frame adapter for the C no-dictionary paths.

use alloc::vec::Vec;

use super::{
    dfast_frame::encode_frame_double_fast_no_dict,
    fast_frame::encode_frame_fast_no_dict,
    greedy_frame::{
        encode_frame_greedy_no_dict, encode_frame_lazy2_no_dict, encode_frame_lazy_no_dict,
    },
    params::{CompressionParameters, Strategy},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UnsupportedStrategy {
    pub(crate) strategy: Strategy,
}

pub(crate) fn strategy_for_level(level: i32, src_size: usize) -> Strategy {
    CompressionParameters::for_level(level, src_size as u64, 0).strategy
}

pub(crate) fn encode_frame_no_dict(src: &[u8], level: i32) -> Result<Vec<u8>, UnsupportedStrategy> {
    match strategy_for_level(level, src.len()) {
        Strategy::Fast => Ok(encode_frame_fast_no_dict(src, level)),
        Strategy::DFast => Ok(encode_frame_double_fast_no_dict(src, level)),
        Strategy::Greedy => Ok(encode_frame_greedy_no_dict(src, level)),
        Strategy::Lazy => Ok(encode_frame_lazy_no_dict(src, level)),
        Strategy::Lazy2 => Ok(encode_frame_lazy2_no_dict(src, level)),
        strategy => Err(UnsupportedStrategy { strategy }),
    }
}
