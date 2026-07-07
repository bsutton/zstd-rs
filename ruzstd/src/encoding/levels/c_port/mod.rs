//! Faithful Rust port of the upstream C compressor.
//!
//! The existing encoder remains the active implementation while this module is
//! built out and checked against the C reference. Keep C-derived behavior here
//! until it has enough parity coverage to replace the current strategy code.

mod block_compressor;
mod block_emit;
mod block_policy;
mod bt_match;
mod c_frame_header;
mod cctx_params;
mod compress_bound;
mod dfast;
mod dfast_block;
mod dfast_dict;
mod dfast_ext;
mod dfast_frame;
mod dfast_helpers;
mod dictionary;
mod dictionary_frame;
mod fast;
mod fast_block;
mod fast_ext;
mod fast_frame;
mod fast_helpers;
mod frame_state;
mod greedy;
mod greedy_block;
mod greedy_bounds;
mod greedy_dict;
mod greedy_ext;
mod greedy_ext_block;
mod greedy_frame;
mod greedy_state;
mod hash_chain_match;
mod ldm;
mod match_count;
mod opt_block;
mod opt_dict;
mod opt_encode;
mod opt_frame;
mod opt_match;
mod opt_parser;
mod opt_path;
mod opt_price;
mod opt_state;
mod params;
mod post_split;
mod pre_split;
mod row_match;
mod sequence_store;
mod strategy_frame;
mod superblock;

pub(crate) use strategy_frame::encode_frame_no_dict;
pub(crate) use strategy_frame::encode_frame_with_dictionary;

pub(crate) use dictionary::DictionaryParseError;

#[cfg(test)]
mod cctx_params_tests;
#[cfg(test)]
mod dfast_ext_tests;
#[cfg(test)]
mod dfast_tests;
#[cfg(test)]
mod fast_ext_tests;
#[cfg(test)]
mod fast_tests;
#[cfg(test)]
mod greedy_ext_tests;
#[cfg(test)]
mod greedy_frame_tests;
#[cfg(test)]
mod greedy_tests;
#[cfg(test)]
mod ldm_tests;
#[cfg(test)]
mod ldm_window_tests;
#[cfg(test)]
mod opt_frame_tests;
#[cfg(test)]
mod opt_match_tests;
#[cfg(test)]
mod opt_parser_tests;
#[cfg(test)]
mod opt_price_tests;
#[cfg(test)]
mod params_tests;
#[cfg(test)]
mod sequence_store_tests;
#[cfg(test)]
mod strategy_frame_tests;
#[cfg(test)]
mod test_dictionary;

#[cfg(test)]
pub(crate) use params::{
    CompressionParameters, Strategy, MAX_COMPRESSION_LEVEL, MIN_COMPRESSION_LEVEL,
};
