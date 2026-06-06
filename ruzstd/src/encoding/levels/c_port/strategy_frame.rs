//! Strategy-dispatching frame adapter for the C frame paths.

use alloc::vec::Vec;

use super::{
    dfast_frame::{encode_frame_double_fast_no_dict, encode_frame_double_fast_with_dictionary},
    dictionary::{parse_dictionary, DictionaryContentType, DictionaryParseError, ParsedDictionary},
    fast_frame::{encode_frame_fast_no_dict, encode_frame_fast_with_dictionary},
    greedy_frame::{
        encode_frame_btlazy2_no_dict, encode_frame_btlazy2_with_dictionary,
        encode_frame_greedy_no_dict, encode_frame_greedy_with_dictionary,
        encode_frame_lazy2_no_dict, encode_frame_lazy2_with_dictionary, encode_frame_lazy_no_dict,
        encode_frame_lazy_with_dictionary,
    },
    opt_frame::{
        encode_frame_btopt_no_dict, encode_frame_btopt_with_dictionary,
        encode_frame_btultra2_no_dict, encode_frame_btultra2_with_dictionary,
        encode_frame_btultra_no_dict, encode_frame_btultra_with_dictionary,
    },
    params::{CompressionParameters, Strategy},
};

pub(crate) fn strategy_for_level(level: i32, src_size: usize) -> Strategy {
    CompressionParameters::for_level(level, src_size as u64, 0).strategy
}

pub(crate) fn strategy_for_level_with_dictionary(
    level: i32,
    src_size: usize,
    dict_size: usize,
) -> Strategy {
    CompressionParameters::for_level(level, src_size as u64, dict_size).strategy
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

pub(crate) fn encode_frame_with_dictionary(
    src: &[u8],
    level: i32,
    dictionary: &[u8],
) -> Result<Vec<u8>, DictionaryParseError> {
    let Some(parsed) = parse_dictionary(dictionary, DictionaryContentType::Auto, false)? else {
        return Ok(encode_frame_no_dict(src, level));
    };

    Ok(encode_frame_with_parsed_dictionary(src, level, parsed))
}

fn encode_frame_with_parsed_dictionary(
    src: &[u8],
    level: i32,
    dictionary: ParsedDictionary<'_>,
) -> Vec<u8> {
    match strategy_for_level_with_dictionary(level, src.len(), dictionary.content.len()) {
        Strategy::Fast => encode_frame_fast_with_dictionary(src, level, dictionary),
        Strategy::DFast => encode_frame_double_fast_with_dictionary(src, level, dictionary),
        Strategy::Greedy => encode_frame_greedy_with_dictionary(src, level, dictionary),
        Strategy::Lazy => encode_frame_lazy_with_dictionary(src, level, dictionary),
        Strategy::Lazy2 => encode_frame_lazy2_with_dictionary(src, level, dictionary),
        Strategy::BtLazy2 => encode_frame_btlazy2_with_dictionary(src, level, dictionary),
        Strategy::BtOpt => encode_frame_btopt_with_dictionary(src, level, dictionary),
        Strategy::BtUltra => encode_frame_btultra_with_dictionary(src, level, dictionary),
        Strategy::BtUltra2 => encode_frame_btultra2_with_dictionary(src, level, dictionary),
    }
}
