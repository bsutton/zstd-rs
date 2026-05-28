use alloc::vec::Vec;

use crate::decoding::FrameDecoder;
use crate::encoding::frame_compressor::{CompressState, FseTables, OffsetHistory};
use crate::encoding::{compress_to_vec, CompressionLevel, MatchGeneratorDriver};

#[test]
fn fastest_does_not_expand_incompressible_blocks_past_raw_size() {
    assert_fastest_does_not_exceed_raw(8 * 1024);
}

#[test]
fn fastest_does_not_expand_incompressible_max_size_blocks() {
    assert_fastest_does_not_exceed_raw(128 * 1024);
}

#[test]
fn fastest_reuses_history_across_blocks() {
    let phrase = b"the quick brown fox jumps over the lazy dog\n";
    let mut data = Vec::with_capacity(512 * 1024);
    while data.len() < 512 * 1024 {
        data.extend_from_slice(phrase);
    }
    data.truncate(512 * 1024);

    let fastest = compress_to_vec(data.as_slice(), CompressionLevel::Fastest);
    let mut decoded = Vec::with_capacity(data.len());
    FrameDecoder::new()
        .decode_all_to_vec(fastest.as_slice(), &mut decoded)
        .unwrap();

    assert_eq!(decoded, data);

    let mut decoded_by_c = Vec::new();
    zstd::stream::copy_decode(fastest.as_slice(), &mut decoded_by_c).unwrap();
    assert_eq!(decoded_by_c, data);

    assert!(
        fastest.len() < 1024,
        "fastest should reuse previous blocks for repetitive data: got {} bytes",
        fastest.len()
    );
}

#[test]
fn incompressible_gate_distinguishes_random_from_repetitive_data() {
    let random = xorshift(128 * 1024);
    assert!(super::fastest::likely_incompressible(&random));

    let mut repetitive = Vec::with_capacity(128 * 1024);
    while repetitive.len() < 128 * 1024 {
        repetitive.extend_from_slice(b"tenant=alpha path=/v1/archive status=200\n");
    }
    repetitive.truncate(128 * 1024);
    assert!(!super::fastest::likely_incompressible(&repetitive));
}

#[test]
fn raw_fallback_restores_matcher_repeat_offsets() {
    let previous_offsets = OffsetHistory::from_offsets(7, 11, 13);
    let mut state = CompressState {
        matcher: MatchGeneratorDriver::new(128 * 1024, 4),
        last_huff_table: None,
        fse_tables: FseTables::new(),
        offset_history: previous_offsets,
    };

    let mut output = Vec::new();
    super::fastest::compress_fastest(&mut state, true, b"abcdeabcde".to_vec(), &mut output);

    assert_eq!(state.offset_history, previous_offsets);
    assert_eq!(
        state.matcher.repeat_offsets(),
        previous_offsets.as_offsets()
    );
}

fn assert_fastest_does_not_exceed_raw(len: usize) {
    let data = xorshift(len);
    let raw = compress_to_vec(data.as_slice(), CompressionLevel::Uncompressed);
    let fastest = compress_to_vec(data.as_slice(), CompressionLevel::Fastest);

    assert!(
        fastest.len() <= raw.len(),
        "fastest output should not exceed raw frame size for {len} bytes: {} > {}",
        fastest.len(),
        raw.len()
    );
}

fn xorshift(len: usize) -> Vec<u8> {
    let mut state = 0x1234_5678_9ABC_DEF0u64;
    let mut data = Vec::with_capacity(len);
    while data.len() < len {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        data.extend_from_slice(&state.to_le_bytes());
    }
    data.truncate(len);
    data
}
