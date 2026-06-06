//! Strategy-dispatching frame adapter for the C no-dictionary paths.

use alloc::vec::Vec;

use super::{
    dfast_frame::encode_frame_double_fast_no_dict,
    fast_frame::encode_frame_fast_no_dict,
    greedy_frame::{
        encode_frame_btlazy2_no_dict, encode_frame_greedy_no_dict, encode_frame_lazy2_no_dict,
        encode_frame_lazy_no_dict,
    },
    opt_frame::{
        encode_frame_btopt_no_dict, encode_frame_btultra2_no_dict, encode_frame_btultra_no_dict,
    },
    params::{CompressionParameters, Strategy},
};

pub(crate) fn strategy_for_level(level: i32, src_size: usize) -> Strategy {
    CompressionParameters::for_level(level, src_size as u64, 0).strategy
}

pub(crate) fn encode_frame_no_dict(src: &[u8], level: i32) -> Vec<u8> {
    match strategy_for_level(level, src.len()) {
        Strategy::Fast => encode_frame_fast_no_dict(src, level),
        Strategy::DFast => encode_frame_double_fast_no_dict(src, level),
        Strategy::Greedy => encode_frame_greedy_no_dict(src, level),
        Strategy::Lazy => encode_frame_lazy_no_dict(src, level),
        Strategy::Lazy2 => encode_frame_lazy2_no_dict(src, level),
        Strategy::BtLazy2 => encode_frame_btlazy2_no_dict(src, level),
        Strategy::BtOpt => encode_frame_btopt_no_dict(src, level),
        Strategy::BtUltra => encode_frame_btultra_no_dict(src, level),
        Strategy::BtUltra2 => encode_frame_btultra2_no_dict(src, level),
    }
}
