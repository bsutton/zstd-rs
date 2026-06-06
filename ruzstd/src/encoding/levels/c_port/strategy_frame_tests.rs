use alloc::{vec, vec::Vec};

use super::{
    params::{CompressionParameters, Strategy},
    strategy_frame::{encode_frame_no_dict, strategy_for_level, UnsupportedStrategy},
};
use crate::{common::MAX_BLOCK_SIZE, decoding::FrameDecoder};

#[test]
fn strategy_frame_routes_level_one_to_fast() {
    let data = b"level-one-fast-level-one-fast-level-one-fast";

    assert_eq!(strategy_for_level(1, data.len()), Strategy::Fast);
    assert_round_trips(&encode_frame_no_dict(data, 1).unwrap(), data);
}

#[test]
fn strategy_frame_routes_negative_levels_to_fast() {
    let data = b"negative-level-fast-negative-level-fast";

    assert_eq!(strategy_for_level(-5, data.len()), Strategy::Fast);
    assert_round_trips(&encode_frame_no_dict(data, -5).unwrap(), data);
}

#[test]
fn strategy_frame_routes_default_level_to_double_fast() {
    let data = b"default-level-double-fast-default-level-double-fast";

    assert_eq!(strategy_for_level(0, data.len()), Strategy::DFast);
    assert_round_trips(&encode_frame_no_dict(data, 0).unwrap(), data);
}

#[test]
fn strategy_frame_routes_level_three_to_double_fast() {
    let data = b"level-three-double-fast-level-three-double-fast";

    assert_eq!(strategy_for_level(3, data.len()), Strategy::DFast);
    assert_round_trips(&encode_frame_no_dict(data, 3).unwrap(), data);
}

#[test]
fn strategy_frame_routes_greedy_levels_to_greedy() {
    let mut data = Vec::new();
    while data.len() < (MAX_BLOCK_SIZE as usize * 2) + 1024 {
        data.extend_from_slice(b"strategy=greedy route=/sync status=202 bytes=1874\n");
    }
    data.truncate((MAX_BLOCK_SIZE as usize * 2) + 1024);

    assert_eq!(strategy_for_level(5, data.len()), Strategy::Greedy);
    assert_round_trips(&encode_frame_no_dict(&data, 5).unwrap(), &data);
}

#[test]
fn strategy_frame_routes_lazy_levels_to_lazy() {
    let mut data = Vec::new();
    while data.len() < (MAX_BLOCK_SIZE as usize * 2) + 1024 {
        data.extend_from_slice(b"strategy=lazy route=/sync status=202 bytes=1874\n");
    }
    data.truncate((MAX_BLOCK_SIZE as usize * 2) + 1024);

    assert_eq!(strategy_for_level(6, data.len()), Strategy::Lazy);
    assert_round_trips(&encode_frame_no_dict(&data, 6).unwrap(), &data);
}

#[test]
fn strategy_frame_routes_lazy2_levels_to_lazy2() {
    let mut data = Vec::new();
    while data.len() < (MAX_BLOCK_SIZE as usize * 2) + 1024 {
        data.extend_from_slice(b"strategy=lazy2 route=/sync status=202 bytes=1874\n");
    }
    data.truncate((MAX_BLOCK_SIZE as usize * 2) + 1024);

    assert_eq!(strategy_for_level(8, data.len()), Strategy::Lazy2);
    assert_round_trips(&encode_frame_no_dict(&data, 8).unwrap(), &data);
}

#[test]
fn strategy_frame_routes_btlazy2_levels_to_btlazy2() {
    let mut data = Vec::new();
    while data.len() < (MAX_BLOCK_SIZE as usize * 2) + 1024 {
        data.extend_from_slice(b"strategy=btlazy2 route=/sync status=202 bytes=1874\n");
    }
    data.truncate((MAX_BLOCK_SIZE as usize * 2) + 1024);

    assert_eq!(strategy_for_level(13, data.len()), Strategy::BtLazy2);
    assert_round_trips(&encode_frame_no_dict(&data, 13).unwrap(), &data);
}

#[test]
fn strategy_frame_round_trips_multiple_blocks() {
    let mut data = Vec::new();
    while data.len() < (MAX_BLOCK_SIZE as usize * 2) + 2048 {
        data.extend_from_slice(b"strategy=double-fast route=/events status=200 bytes=915\n");
    }
    data.truncate((MAX_BLOCK_SIZE as usize * 2) + 2048);

    let encoded = encode_frame_no_dict(&data, 3).unwrap();

    assert!(count_frame_blocks(&encoded) > 1);
    assert_round_trips(&encoded, &data);
}

#[test]
fn strategy_frame_reports_unsupported_strategies() {
    let data = vec![0x5Au8; (MAX_BLOCK_SIZE as usize * 2) + 256];
    let strategy = CompressionParameters::for_level(16, data.len() as u64, 0).strategy;

    assert!(!matches!(
        strategy,
        Strategy::Fast
            | Strategy::DFast
            | Strategy::Greedy
            | Strategy::Lazy
            | Strategy::Lazy2
            | Strategy::BtLazy2
    ));
    assert_eq!(
        encode_frame_no_dict(&data, 16),
        Err(UnsupportedStrategy { strategy })
    );
}

fn assert_round_trips(encoded: &[u8], expected: &[u8]) {
    let mut decoded = Vec::with_capacity(expected.len());
    FrameDecoder::new()
        .decode_all_to_vec(encoded, &mut decoded)
        .unwrap();

    assert_eq!(decoded, expected);
}

fn count_frame_blocks(encoded: &[u8]) -> usize {
    let (_, frame_header_size) =
        crate::decoding::frame::read_frame_header(encoded).expect("frame header should parse");
    let mut block_decoder = crate::decoding::block_decoder::new();
    let mut offset = frame_header_size as usize;
    let mut blocks = 0;

    loop {
        let (header, block_header_size) = block_decoder
            .read_block_header(&encoded[offset..])
            .expect("block header should parse");
        offset += block_header_size as usize + header.content_size as usize;
        blocks += 1;

        if header.last_block {
            break blocks;
        }
    }
}
