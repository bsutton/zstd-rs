use alloc::vec::Vec;

use super::greedy_frame::{
    encode_frame_btlazy2_no_dict, encode_frame_greedy_no_dict, encode_frame_lazy2_no_dict,
    encode_frame_lazy_no_dict, encode_single_block_frame_btlazy2_no_dict,
    encode_single_block_frame_greedy_no_dict, encode_single_block_frame_lazy2_no_dict,
    encode_single_block_frame_lazy_no_dict,
};
use super::params::{CompressionParameters, Strategy};
use crate::common::MAX_BLOCK_SIZE;
use crate::decoding::FrameDecoder;

fn greedy_level(src_len: usize) -> i32 {
    if src_len <= 16 * 1024 {
        4
    } else {
        5
    }
}

fn lazy_params(src_len: usize) -> CompressionParameters {
    CompressionParameters::for_level(lazy_level(src_len), src_len as u64, 0)
}

fn lazy2_params(src_len: usize) -> CompressionParameters {
    CompressionParameters::for_level(lazy2_level(src_len), src_len as u64, 0)
}

fn btlazy2_params(src_len: usize) -> CompressionParameters {
    CompressionParameters::for_level(btlazy2_level(src_len), src_len as u64, 0)
}

fn lazy_level(src_len: usize) -> i32 {
    if src_len <= 16 * 1024 {
        5
    } else {
        6
    }
}

fn lazy2_level(src_len: usize) -> i32 {
    if src_len <= 16 * 1024 {
        6
    } else {
        8
    }
}

fn btlazy2_level(src_len: usize) -> i32 {
    if src_len <= 16 * 1024 {
        9
    } else {
        13
    }
}

#[test]
fn greedy_hidden_frame_round_trips_compressed_block() {
    let data = b"abcde12345abcde12345-tail";
    let encoded = encode_single_block_frame_greedy_no_dict(data, greedy_level(data.len()));

    assert_round_trips(&encoded, data);
}

#[test]
fn greedy_hidden_frame_round_trips_rle_block() {
    let data = [0x42; 4096];
    let encoded = encode_single_block_frame_greedy_no_dict(&data, greedy_level(data.len()));

    assert_round_trips(&encoded, &data);
}

#[test]
fn lazy_hidden_frame_round_trips_compressed_block() {
    let data = b"abcde12345abcde12345-tail";
    let encoded = encode_single_block_frame_lazy_no_dict(data, lazy_level(data.len()));

    assert_round_trips(&encoded, data);
}

#[test]
fn lazy2_hidden_frame_round_trips_compressed_block() {
    let data = b"abcde12345abcde12345-tail";
    let encoded = encode_single_block_frame_lazy2_no_dict(data, lazy2_level(data.len()));

    assert_round_trips(&encoded, data);
}

#[test]
fn btlazy2_hidden_frame_round_trips_compressed_block() {
    let data = b"abcde12345abcde12345-tail";
    let encoded = encode_single_block_frame_btlazy2_no_dict(data, btlazy2_level(data.len()));

    assert_round_trips(&encoded, data);
}

#[test]
fn greedy_hidden_frame_round_trips_multiple_blocks() {
    let mut data = Vec::new();
    while data.len() < (MAX_BLOCK_SIZE as usize * 2) + 1536 {
        data.extend_from_slice(b"tenant=gamma method=POST route=/v3/items status=201 bytes=244\n");
    }
    data.truncate((MAX_BLOCK_SIZE as usize * 2) + 1536);

    let encoded = encode_frame_greedy_no_dict(&data, greedy_level(data.len()));

    assert!(count_frame_blocks(&encoded) > 1);
    assert_round_trips(&encoded, &data);
}

#[test]
fn lazy_hidden_frame_round_trips_multiple_blocks() {
    let mut data = Vec::new();
    while data.len() < (MAX_BLOCK_SIZE as usize * 2) + 1536 {
        data.extend_from_slice(b"tenant=delta method=PATCH route=/v4/items status=204 bytes=99\n");
    }
    data.truncate((MAX_BLOCK_SIZE as usize * 2) + 1536);

    assert_eq!(lazy_params(data.len()).strategy, Strategy::Lazy);
    let encoded = encode_frame_lazy_no_dict(&data, lazy_level(data.len()));

    assert!(count_frame_blocks(&encoded) > 1);
    assert_round_trips(&encoded, &data);
}

#[test]
fn lazy2_hidden_frame_round_trips_multiple_blocks() {
    let mut data = Vec::new();
    while data.len() < (MAX_BLOCK_SIZE as usize * 2) + 1536 {
        data.extend_from_slice(b"tenant=epsilon method=PUT route=/v5/items status=200 bytes=122\n");
    }
    data.truncate((MAX_BLOCK_SIZE as usize * 2) + 1536);

    assert_eq!(lazy2_params(data.len()).strategy, Strategy::Lazy2);
    let encoded = encode_frame_lazy2_no_dict(&data, lazy2_level(data.len()));

    assert!(count_frame_blocks(&encoded) > 1);
    assert_round_trips(&encoded, &data);
}

#[test]
fn btlazy2_hidden_frame_round_trips_multiple_blocks() {
    let mut data = Vec::new();
    while data.len() < (MAX_BLOCK_SIZE as usize * 2) + 1536 {
        data.extend_from_slice(b"tenant=zeta method=PUT route=/v6/items status=200 bytes=355\n");
    }
    data.truncate((MAX_BLOCK_SIZE as usize * 2) + 1536);

    assert_eq!(btlazy2_params(data.len()).strategy, Strategy::BtLazy2);
    let encoded = encode_frame_btlazy2_no_dict(&data, btlazy2_level(data.len()));

    assert!(count_frame_blocks(&encoded) > 1);
    assert_round_trips(&encoded, &data);
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
