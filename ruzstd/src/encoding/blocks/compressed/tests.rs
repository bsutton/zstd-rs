use super::config::FILE_TYPE_SINGLE_STREAM_HUFFMAN_MAX_LITERALS;
use super::*;
use crate::encoding::frame_compressor::{CompressState, FseTables, OffsetHistory};
use crate::encoding::{CompressionFileType, CompressionLevel};
use crate::fse::fse_encoder::{default_ll_table, default_ml_table, default_of_table};

fn offset_history(newest: u32, second: u32, third: u32) -> OffsetHistory {
    OffsetHistory {
        newest,
        second,
        third,
    }
}

struct LiteralPayloadMatcher {
    literals: Vec<u8>,
    emitted: bool,
}

impl Matcher for LiteralPayloadMatcher {
    fn get_next_space(&mut self) -> Vec<u8> {
        Vec::new()
    }

    fn get_last_space(&self) -> &[u8] {
        &[]
    }

    fn commit_space(&mut self, _space: Vec<u8>) {}

    fn skip_matching(&mut self) {}

    fn start_matching(&mut self, mut handle_sequence: impl for<'a> FnMut(Sequence<'a>)) {
        if !self.emitted {
            self.emitted = true;
            handle_sequence(Sequence::Triple {
                literals: &self.literals,
                offset: 1,
                match_len: 16,
            });
        }
    }

    fn reset(&mut self, _level: crate::encoding::CompressionLevel) {
        self.emitted = false;
    }

    fn window_size(&self) -> u64 {
        128 * 1024
    }
}

fn compressed_frame_with_literal_payload(literals: Vec<u8>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    compressed_frame_with_literal_payload_and_last_table(literals, None)
}

fn compressed_frame_with_literal_payload_and_last_table(
    literals: Vec<u8>,
    last_huff_table: Option<huff0_encoder::HuffmanTable>,
) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    compressed_frame_with_literal_payload_and_config(
        literals,
        last_huff_table,
        BlockCompressionConfig::for_level(CompressionLevel::Fastest),
    )
}

fn compressed_frame_with_literal_payload_and_config(
    literals: Vec<u8>,
    last_huff_table: Option<huff0_encoder::HuffmanTable>,
    config: BlockCompressionConfig,
) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    assert!(!literals.is_empty());

    let mut state = CompressState {
        matcher: LiteralPayloadMatcher {
            literals: literals.clone(),
            emitted: false,
        },
        last_huff_table,
        fse_tables: FseTables::new(),
        offset_history: OffsetHistory::new(),
        file_type_hint: CompressionFileType::Unknown,
        file_profile_hint: CompressionFileProfile::None,
    };
    let mut block_payload = Vec::new();

    compress_block_with_config(&mut state, &mut block_payload, config);

    let mut frame = Vec::new();
    crate::encoding::frame_header::FrameHeader {
        frame_content_size: None,
        single_segment: false,
        content_checksum: false,
        dictionary_id: None,
        window_size: Some(128 * 1024),
    }
    .serialize(&mut frame);
    crate::encoding::block_header::BlockHeader {
        last_block: true,
        block_type: crate::blocks::block::BlockType::Compressed,
        block_size: block_payload.len() as u32,
    }
    .serialize(&mut frame);
    frame.extend_from_slice(&block_payload);

    let last_literal = literals[literals.len() - 1];
    let mut expected = literals;
    expected.extend_from_slice(&[last_literal; 16]);

    (block_payload, frame, expected)
}

fn literal_length_code_from_spec(len: u32) -> (u8, u32, usize) {
    match len {
        0..=15 => (len as u8, 0, 0),
        16..=17 => (16, len - 16, 1),
        18..=19 => (17, len - 18, 1),
        20..=21 => (18, len - 20, 1),
        22..=23 => (19, len - 22, 1),
        24..=27 => (20, len - 24, 2),
        28..=31 => (21, len - 28, 2),
        32..=39 => (22, len - 32, 3),
        40..=47 => (23, len - 40, 3),
        48..=63 => (24, len - 48, 4),
        64..=127 => (25, len - 64, 6),
        _ => panic!("test helper only covers literal lengths through code 25"),
    }
}

fn match_length_code_from_spec(len: u32) -> (u8, u32, usize) {
    match len {
        0..=2 => panic!("match lengths below 3 are invalid"),
        3..=34 => (len as u8 - 3, 0, 0),
        35..=36 => (32, len - 35, 1),
        37..=38 => (33, len - 37, 1),
        39..=40 => (34, len - 39, 1),
        41..=42 => (35, len - 41, 1),
        43..=46 => (36, len - 43, 2),
        47..=50 => (37, len - 47, 2),
        51..=58 => (38, len - 51, 3),
        59..=66 => (39, len - 59, 3),
        67..=82 => (40, len - 67, 4),
        83..=98 => (41, len - 83, 4),
        99..=130 => (42, len - 99, 5),
        131..=258 => (43, len - 131, 7),
        _ => panic!("test helper only covers match lengths through code 43"),
    }
}

fn offset_code_from_spec(len: u32) -> (u8, u32, usize) {
    let code = len.ilog2();
    let additional = len - (1 << code);
    (code as u8, additional, code as usize)
}

#[test]
fn offset_history_uses_repeat_offsets_when_literals_are_present() {
    let mut history = OffsetHistory::new();

    assert_eq!(history.encode_offset_value(4, 3), 2);
    assert_eq!(history, offset_history(4, 1, 8));

    assert_eq!(history.encode_offset_value(4, 1), 1);
    assert_eq!(history, offset_history(4, 1, 8));

    assert_eq!(history.encode_offset_value(8, 2), 3);
    assert_eq!(history, offset_history(8, 4, 1));
}

#[test]
fn offset_history_uses_shifted_repeat_offsets_for_zero_literals() {
    let mut history = offset_history(5, 9, 13);

    assert_eq!(history.encode_offset_value(9, 0), 1);
    assert_eq!(history, offset_history(9, 5, 13));

    let mut history = offset_history(5, 9, 13);
    assert_eq!(history.encode_offset_value(13, 0), 2);
    assert_eq!(history, offset_history(13, 5, 9));

    let mut history = offset_history(5, 9, 13);
    assert_eq!(history.encode_offset_value(4, 0), 3);
    assert_eq!(history, offset_history(4, 5, 9));
}

#[test]
fn offset_history_encodes_new_offsets_and_updates_history() {
    let mut history = OffsetHistory::new();

    assert_eq!(history.encode_offset_value(10, 1), 13);
    assert_eq!(history, offset_history(10, 1, 4));
}

#[test]
fn choose_table_uses_predefined_tables_for_tiny_sequence_counts() {
    let default = default_ll_table();
    let sequences = [crate::blocks::sequence_section::Sequence {
        ll: 0,
        ml: 3,
        of: 1,
    }];

    assert!(matches!(
        choose_table(
            None,
            &default,
            &sequences,
            |seq| encode_literal_length(seq.ll).0,
            9,
            64,
            16,
        ),
        FseTableMode::Predefined(_)
    ));
}

#[test]
fn choose_table_uses_predefined_tables_for_small_non_rle_blocks() {
    let default = default_ll_table();
    let sequences = [
        crate::blocks::sequence_section::Sequence {
            ll: 0,
            ml: 3,
            of: 1,
        },
        crate::blocks::sequence_section::Sequence {
            ll: 2,
            ml: 3,
            of: 1,
        },
        crate::blocks::sequence_section::Sequence {
            ll: 4,
            ml: 3,
            of: 1,
        },
    ];

    assert!(matches!(
        choose_table(
            None,
            &default,
            &sequences,
            |seq| encode_literal_length(seq.ll).0,
            9,
            64,
            16,
        ),
        FseTableMode::Predefined(_)
    ));
}

#[test]
fn choose_table_uses_rle_for_repeated_codes() {
    let default = default_ll_table();
    let sequences = [crate::blocks::sequence_section::Sequence {
        ll: 5,
        ml: 8,
        of: 1,
    }; 3];

    assert!(matches!(
        choose_table(
            None,
            &default,
            &sequences,
            |seq| encode_literal_length(seq.ll).0,
            9,
            64,
            16,
        ),
        FseTableMode::Rle(5)
    ));
}

#[test]
fn previous_huffman_table_lowers_literal_compression_threshold() {
    assert!(!should_compress_literals(COMPRESS_LITERALS_SIZE_MIN, false));
    assert!(should_compress_literals(
        COMPRESS_LITERALS_SIZE_MIN + 1,
        false
    ));

    assert!(!should_compress_literals(REPEAT_LITERALS_SIZE_MIN, true));
    assert!(should_compress_literals(REPEAT_LITERALS_SIZE_MIN + 1, true));
}

#[test]
fn rle_sequence_modes_round_trip_through_decoder() {
    let sequences = [
        crate::blocks::sequence_section::Sequence {
            ll: 5,
            ml: 8,
            of: 1,
        },
        crate::blocks::sequence_section::Sequence {
            ll: 5,
            ml: 8,
            of: 1,
        },
        crate::blocks::sequence_section::Sequence {
            ll: 5,
            ml: 8,
            of: 1,
        },
    ];
    let ll_mode = FseTableMode::Rle(encode_literal_length(sequences[0].ll).0);
    let ml_mode = FseTableMode::Rle(encode_match_len(sequences[0].ml).0);
    let of_mode = FseTableMode::Rle(encode_offset(sequences[0].of).0);
    let mut encoded = Vec::new();
    let mut writer = BitWriter::from(&mut encoded);

    encode_seqnum(sequences.len(), &mut writer);
    writer.write_bits(encode_fse_table_modes(&ll_mode, &ml_mode, &of_mode), 8);
    encode_table(&ll_mode, &mut writer);
    encode_table(&of_mode, &mut writer);
    encode_table(&ml_mode, &mut writer);
    encode_sequences(&sequences, &mut writer, &ll_mode, &ml_mode, &of_mode);
    writer.flush();

    let mut header = crate::blocks::sequence_section::SequencesHeader::new();
    let header_size = header.parse_from_header(&encoded).unwrap();
    let mut scratch = crate::decoding::scratch::FSEScratch::new();
    let mut decoded = Vec::new();

    crate::decoding::sequence_section_decoder::decode_sequences(
        &header,
        &encoded[header_size as usize..],
        &mut scratch,
        &mut decoded,
    )
    .unwrap();

    assert_eq!(decoded.len(), sequences.len());
    for (actual, expected) in decoded.iter().zip(sequences) {
        assert_eq!(actual.ll, expected.ll);
        assert_eq!(actual.ml, expected.ml);
        assert_eq!(actual.of, expected.of);
    }
}

#[test]
fn all_rle_sequence_modes_preserve_additional_bits() {
    let sequences = [
        crate::blocks::sequence_section::Sequence {
            ll: 16,
            ml: 35,
            of: 4,
        },
        crate::blocks::sequence_section::Sequence {
            ll: 17,
            ml: 36,
            of: 5,
        },
        crate::blocks::sequence_section::Sequence {
            ll: 16,
            ml: 35,
            of: 6,
        },
    ];
    let ll_mode = FseTableMode::Rle(encode_literal_length(sequences[0].ll).0);
    let ml_mode = FseTableMode::Rle(encode_match_len(sequences[0].ml).0);
    let of_mode = FseTableMode::Rle(encode_offset(sequences[0].of).0);
    let mut encoded = Vec::new();
    let mut writer = BitWriter::from(&mut encoded);

    encode_seqnum(sequences.len(), &mut writer);
    writer.write_bits(encode_fse_table_modes(&ll_mode, &ml_mode, &of_mode), 8);
    encode_table(&ll_mode, &mut writer);
    encode_table(&of_mode, &mut writer);
    encode_table(&ml_mode, &mut writer);
    encode_sequences(&sequences, &mut writer, &ll_mode, &ml_mode, &of_mode);
    writer.flush();

    let mut header = crate::blocks::sequence_section::SequencesHeader::new();
    let header_size = header.parse_from_header(&encoded).unwrap();
    let mut scratch = crate::decoding::scratch::FSEScratch::new();
    let mut decoded = Vec::new();

    crate::decoding::sequence_section_decoder::decode_sequences(
        &header,
        &encoded[header_size as usize..],
        &mut scratch,
        &mut decoded,
    )
    .unwrap();

    assert_eq!(decoded.len(), sequences.len());
    for (actual, expected) in decoded.iter().zip(sequences) {
        assert_eq!(actual.ll, expected.ll);
        assert_eq!(actual.ml, expected.ml);
        assert_eq!(actual.of, expected.of);
    }
}

#[test]
fn mixed_predefined_sequence_modes_round_trip_through_decoder() {
    let sequences = [
        crate::blocks::sequence_section::Sequence {
            ll: 0,
            ml: 3,
            of: 1,
        },
        crate::blocks::sequence_section::Sequence {
            ll: 1,
            ml: 4,
            of: 2,
        },
        crate::blocks::sequence_section::Sequence {
            ll: 4,
            ml: 8,
            of: 4,
        },
        crate::blocks::sequence_section::Sequence {
            ll: 12,
            ml: 16,
            of: 8,
        },
    ];
    let ll_default = default_ll_table();
    let ml_default = default_ml_table();
    let of_default = default_of_table();
    let ll_mode = FseTableMode::Predefined(&ll_default);
    let ml_mode = FseTableMode::Predefined(&ml_default);
    let of_mode = FseTableMode::Predefined(&of_default);
    let mut encoded = Vec::new();
    let mut writer = BitWriter::from(&mut encoded);

    encode_seqnum(sequences.len(), &mut writer);
    writer.write_bits(encode_fse_table_modes(&ll_mode, &ml_mode, &of_mode), 8);
    encode_table(&ll_mode, &mut writer);
    encode_table(&of_mode, &mut writer);
    encode_table(&ml_mode, &mut writer);
    encode_sequences(&sequences, &mut writer, &ll_mode, &ml_mode, &of_mode);
    writer.flush();

    let mut header = crate::blocks::sequence_section::SequencesHeader::new();
    let header_size = header.parse_from_header(&encoded).unwrap();
    let mut scratch = crate::decoding::scratch::FSEScratch::new();
    let mut decoded = Vec::new();

    crate::decoding::sequence_section_decoder::decode_sequences(
        &header,
        &encoded[header_size as usize..],
        &mut scratch,
        &mut decoded,
    )
    .unwrap();

    assert_eq!(decoded.len(), sequences.len());
    for (actual, expected) in decoded.iter().zip(sequences) {
        assert_eq!(actual.ll, expected.ll);
        assert_eq!(actual.ml, expected.ml);
        assert_eq!(actual.of, expected.of);
    }
}

#[test]
fn match_length_code_52_uses_65539_baseline() {
    assert_eq!(encode_match_len(65538), (51, 32767, 15));
    assert_eq!(encode_match_len(65539), (52, 0, 16));
    assert_eq!(encode_match_len(98264), (52, 32725, 16));
    assert_eq!(encode_match_len(131074), (52, 65535, 16));
}

#[test]
fn small_length_code_tables_match_spec_ranges() {
    for len in 0..=64 {
        assert_eq!(
            encode_literal_length(len),
            literal_length_code_from_spec(len)
        );
    }

    for len in 3..=131 {
        assert_eq!(encode_match_len(len), match_length_code_from_spec(len));
    }

    for len in 1..=129 {
        assert_eq!(encode_offset(len), offset_code_from_spec(len));
    }
}

#[test]
fn raw_literals_use_shortest_header_form() {
    let mut one_byte_header = Vec::new();
    let mut writer = BitWriter::from(&mut one_byte_header);
    raw_literals(&[7; 31], &mut writer);
    writer.flush();
    assert_eq!(one_byte_header[0], 31 << 3);
    assert_eq!(&one_byte_header[1..], &[7; 31]);

    let mut two_byte_header = Vec::new();
    let mut writer = BitWriter::from(&mut two_byte_header);
    raw_literals(&[9; 44], &mut writer);
    writer.flush();
    assert_eq!(&two_byte_header[..2], &[0xC4, 0x02]);
    assert_eq!(&two_byte_header[2..], &[9; 44]);

    let mut three_byte_header = Vec::new();
    let mut writer = BitWriter::from(&mut three_byte_header);
    raw_literals(&[11; 4096], &mut writer);
    writer.flush();
    assert_eq!(&three_byte_header[..3], &[0x0C, 0x00, 0x01]);
    assert_eq!(&three_byte_header[3..], &[11; 4096]);
}

#[test]
fn rle_literals_use_shortest_header_form() {
    let mut one_byte_header = Vec::new();
    let mut writer = BitWriter::from(&mut one_byte_header);
    rle_literals(&[7; 31], &mut writer);
    writer.flush();
    assert_eq!(&one_byte_header, &[0xF9, 7]);

    let mut two_byte_header = Vec::new();
    let mut writer = BitWriter::from(&mut two_byte_header);
    rle_literals(&[9; 44], &mut writer);
    writer.flush();
    assert_eq!(&two_byte_header, &[0xC5, 0x02, 9]);

    let mut three_byte_header = Vec::new();
    let mut writer = BitWriter::from(&mut three_byte_header);
    rle_literals(&[11; 4096], &mut writer);
    writer.flush();
    assert_eq!(&three_byte_header, &[0x0D, 0x00, 0x01, 11]);
}

#[test]
fn rle_literals_round_trip_through_decoder() {
    let mut encoded = Vec::new();
    let mut writer = BitWriter::from(&mut encoded);
    rle_literals(&[42; 44], &mut writer);
    writer.flush();

    let mut section = crate::blocks::literals_section::LiteralsSection::new();
    let header_size = section.parse_from_header(&encoded).unwrap();
    assert!(matches!(
        section.ls_type,
        crate::blocks::literals_section::LiteralsSectionType::RLE
    ));

    let mut scratch = crate::decoding::scratch::HuffmanScratch::new();
    let mut decoded = Vec::new();
    let bytes_read = crate::decoding::literals_section_decoder::decode_literals(
        &section,
        &mut scratch,
        &encoded[header_size as usize..],
        &mut decoded,
    )
    .unwrap();

    assert_eq!(bytes_read, 1);
    assert_eq!(decoded, [42; 44]);
}

#[test]
fn rle_literals_frame_round_trips_through_rust_and_c_decoders() {
    let (block_payload, frame, expected) =
        compressed_frame_with_literal_payload(alloc::vec![42; 2048]);

    assert_eq!(block_payload[0] & 0b11, 1, "literal section should be RLE");

    let mut rust_decoded = Vec::with_capacity(expected.len());
    let mut decoder = crate::decoding::FrameDecoder::new();
    decoder
        .decode_all_to_vec(&frame, &mut rust_decoded)
        .unwrap();
    assert_eq!(rust_decoded, expected);

    let mut c_decoded = Vec::new();
    zstd::stream::copy_decode(frame.as_slice(), &mut c_decoded).unwrap();
    assert_eq!(c_decoded, expected);
}

#[test]
fn small_rle_literals_use_previous_table_threshold_and_round_trip() {
    let previous_table = huff0_encoder::HuffmanTable::build_from_counts(&[8, 1, 1, 1, 1, 1, 1, 1]);
    let (block_payload, frame, expected) = compressed_frame_with_literal_payload_and_last_table(
        alloc::vec![42; 7],
        Some(previous_table),
    );

    assert_eq!(
        block_payload[0] & 0b11,
        1,
        "small repeated literals should use RLE when a previous table lowers the threshold"
    );

    let mut rust_decoded = Vec::with_capacity(expected.len());
    let mut decoder = crate::decoding::FrameDecoder::new();
    decoder
        .decode_all_to_vec(&frame, &mut rust_decoded)
        .unwrap();
    assert_eq!(rust_decoded, expected);

    let mut c_decoded = Vec::new();
    zstd::stream::copy_decode(frame.as_slice(), &mut c_decoded).unwrap();
    assert_eq!(c_decoded, expected);
}

#[test]
fn small_compressible_literals_use_huffman_and_round_trip() {
    let mut literals = alloc::vec![b'a'; 512];
    for idx in (15..literals.len()).step_by(16) {
        literals[idx] = b'b';
    }

    let (block_payload, frame, expected) = compressed_frame_with_literal_payload(literals);

    assert_eq!(
        block_payload[0] & 0b11,
        2,
        "small skewed literal section should use Huffman compression"
    );

    let mut rust_decoded = Vec::with_capacity(expected.len());
    let mut decoder = crate::decoding::FrameDecoder::new();
    decoder
        .decode_all_to_vec(&frame, &mut rust_decoded)
        .unwrap();
    assert_eq!(rust_decoded, expected);

    let mut c_decoded = Vec::new();
    zstd::stream::copy_decode(frame.as_slice(), &mut c_decoded).unwrap();
    assert_eq!(c_decoded, expected);
}

#[test]
fn small_huffman_literals_use_single_stream_and_round_trip() {
    let mut literals = alloc::vec![b'a'; 128];
    for idx in (15..literals.len()).step_by(16) {
        literals[idx] = b'b';
    }

    let (block_payload, frame, expected) = compressed_frame_with_literal_payload(literals);

    assert_eq!(
        block_payload[0] & 0b11,
        2,
        "small skewed literal section should use Huffman compression"
    );
    assert_eq!(
        (block_payload[0] >> 2) & 0b11,
        0,
        "small Huffman literal payloads should use the single-stream header"
    );

    let mut rust_decoded = Vec::with_capacity(expected.len());
    let mut decoder = crate::decoding::FrameDecoder::new();
    decoder
        .decode_all_to_vec(&frame, &mut rust_decoded)
        .unwrap();
    assert_eq!(rust_decoded, expected);

    let mut c_decoded = Vec::new();
    zstd::stream::copy_decode(frame.as_slice(), &mut c_decoded).unwrap();
    assert_eq!(c_decoded, expected);
}

#[test]
fn small_literals_prefer_previous_huffman_table_and_single_stream() {
    let mut first_literals = alloc::vec![0; 512];
    for idx in (15..first_literals.len()).step_by(16) {
        first_literals[idx] = 1;
    }
    let second_literals = first_literals.clone();

    let mut state = CompressState {
        matcher: LiteralPayloadMatcher {
            literals: first_literals.clone(),
            emitted: false,
        },
        last_huff_table: None,
        fse_tables: FseTables::new(),
        offset_history: OffsetHistory::new(),
        file_type_hint: CompressionFileType::Unknown,
        file_profile_hint: CompressionFileProfile::None,
    };
    let mut first_payload = Vec::new();
    state.last_huff_table = compress_block_with_config(
        &mut state,
        &mut first_payload,
        BlockCompressionConfig::for_level(CompressionLevel::Fastest),
    );

    state.matcher = LiteralPayloadMatcher {
        literals: second_literals.clone(),
        emitted: false,
    };
    let mut second_payload = Vec::new();
    compress_block_with_config(
        &mut state,
        &mut second_payload,
        BlockCompressionConfig::for_level(CompressionLevel::Fastest),
    );

    assert_eq!(
        second_payload[0] & 0b11,
        3,
        "small literals encodable by previous table should use treeless Huffman"
    );
    assert_eq!(
        (second_payload[0] >> 2) & 0b11,
        0,
        "small repeated-table Huffman literals should use the single-stream header"
    );

    let mut frame = Vec::new();
    crate::encoding::frame_header::FrameHeader {
        frame_content_size: None,
        single_segment: false,
        content_checksum: false,
        dictionary_id: None,
        window_size: Some(128 * 1024),
    }
    .serialize(&mut frame);
    crate::encoding::block_header::BlockHeader {
        last_block: false,
        block_type: crate::blocks::block::BlockType::Compressed,
        block_size: first_payload.len() as u32,
    }
    .serialize(&mut frame);
    frame.extend_from_slice(&first_payload);
    crate::encoding::block_header::BlockHeader {
        last_block: true,
        block_type: crate::blocks::block::BlockType::Compressed,
        block_size: second_payload.len() as u32,
    }
    .serialize(&mut frame);
    frame.extend_from_slice(&second_payload);

    let mut expected = first_literals.clone();
    expected.extend_from_slice(&[1; 16]);
    expected.extend_from_slice(&second_literals);
    expected.extend_from_slice(&[1; 16]);

    let mut rust_decoded = Vec::with_capacity(expected.len());
    let mut decoder = crate::decoding::FrameDecoder::new();
    decoder
        .decode_all_to_vec(&frame, &mut rust_decoded)
        .unwrap();
    assert_eq!(rust_decoded, expected);

    let mut c_decoded = Vec::new();
    zstd::stream::copy_decode(frame.as_slice(), &mut c_decoded).unwrap();
    assert_eq!(c_decoded, expected);
}

#[test]
fn literal_estimate_without_gain_uses_raw_literals_and_round_trips() {
    let mut literals = alloc::vec![0; 6];
    for value in 1..=64u8 {
        literals.push(value);
    }

    let (block_payload, frame, expected) = compressed_frame_with_literal_payload(literals);

    assert_eq!(
        block_payload[0] & 0b11,
        0,
        "literal estimate should choose raw when Huffman cannot beat raw"
    );

    let mut rust_decoded = Vec::with_capacity(expected.len());
    let mut decoder = crate::decoding::FrameDecoder::new();
    decoder
        .decode_all_to_vec(&frame, &mut rust_decoded)
        .unwrap();
    assert_eq!(rust_decoded, expected);

    let mut c_decoded = Vec::new();
    zstd::stream::copy_decode(frame.as_slice(), &mut c_decoded).unwrap();
    assert_eq!(c_decoded, expected);
}

#[test]
fn literal_min_gain_boundary_uses_exact_table_search_and_round_trips() {
    let len = 128usize;
    let period = 86u32;
    let mut state = (len as u32).wrapping_mul(1_664_525).wrapping_add(period);
    let mut literals = Vec::with_capacity(len);
    for _ in 0..len {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        literals.push((state % period) as u8);
    }

    let literal_stats = LiteralStats::from_literals(&literals);
    let table = huff0_encoder::HuffmanTable::build_from_counts(literal_stats.counts());
    let estimated_len = table.encoded_len(&literals, true, false);
    let exact_table = huff0_encoder::HuffmanTable::build_smallest_from_counts(
        literal_stats.counts(),
        &literals,
        false,
    );
    let exact_estimated_len = exact_table.encoded_len(&literals, true, false);
    let header_len = compressed_literals_header_len(0);

    assert_eq!(literal_min_gain(literals.len()), 4);
    assert!(
        estimated_len + header_len < literals.len(),
        "without the min-gain check this payload would be Huffman-compressed"
    );
    assert!(
        estimated_len
            >= literals
                .len()
                .saturating_sub(literal_min_gain(literals.len())),
        "C-style min-gain threshold should reject this narrow literal gain"
    );
    assert!(
        literal_estimate_has_enough_gain(exact_estimated_len, header_len, literals.len()),
        "exact table search should find enough gain for small all-literal payloads"
    );

    let (block_payload, frame, expected) = compressed_frame_with_literal_payload(literals);

    assert_eq!(
        block_payload[0] & 0b11,
        2,
        "small all-literal payloads should use the exact Huffman table when it has enough gain"
    );

    let mut rust_decoded = Vec::with_capacity(expected.len());
    let mut decoder = crate::decoding::FrameDecoder::new();
    decoder
        .decode_all_to_vec(&frame, &mut rust_decoded)
        .unwrap();
    assert_eq!(rust_decoded, expected);

    let mut c_decoded = Vec::new();
    zstd::stream::copy_decode(frame.as_slice(), &mut c_decoded).unwrap();
    assert_eq!(c_decoded, expected);
}

#[test]
fn best_level_searches_exact_huffman_tables_beyond_small_literal_sections() {
    let len = SMALL_HUFFMAN_TABLE_SEARCH_MAX_LITERALS + 256;
    let period = 86u32;
    let mut state = (len as u32).wrapping_mul(1_664_525).wrapping_add(period);
    let mut literals = Vec::with_capacity(len);
    for _ in 0..len {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        literals.push((state % period) as u8);
    }

    let literal_stats = LiteralStats::from_literals(&literals);
    let baseline_table = huff0_encoder::HuffmanTable::build_from_counts(literal_stats.counts());
    let exact_table = huff0_encoder::HuffmanTable::build_smallest_from_counts(
        literal_stats.counts(),
        &literals,
        true,
    );
    assert!(
        exact_table.encoded_len(&literals, true, true)
            <= baseline_table.encoded_len(&literals, true, true),
        "exact table search should not be worse on this higher-level fixture"
    );

    let (fast_block, _, _) = compressed_frame_with_literal_payload_and_config(
        literals.clone(),
        None,
        BlockCompressionConfig::for_level_and_file_type(
            CompressionLevel::Fastest,
            CompressionFileType::ArchiveLike,
        ),
    );
    let (best_block, best_frame, expected) = compressed_frame_with_literal_payload_and_config(
        literals,
        None,
        BlockCompressionConfig::for_level_and_file_type(
            CompressionLevel::Best,
            CompressionFileType::ArchiveLike,
        ),
    );

    assert!(
        best_block.len() <= fast_block.len(),
        "best-level exact Huffman search should not emit a larger block: {} > {}",
        best_block.len(),
        fast_block.len()
    );

    let mut rust_decoded = Vec::with_capacity(expected.len());
    let mut decoder = crate::decoding::FrameDecoder::new();
    decoder
        .decode_all_to_vec(&best_frame, &mut rust_decoded)
        .unwrap();
    assert_eq!(rust_decoded, expected);

    let mut c_decoded = Vec::new();
    zstd::stream::copy_decode(best_frame.as_slice(), &mut c_decoded).unwrap();
    assert_eq!(c_decoded, expected);
}

#[test]
fn fastest_code_and_config_text_enable_small_literal_exact_search() {
    assert!(matches!(
        BlockCompressionConfig::for_level_and_file_type(
            CompressionLevel::Fastest,
            CompressionFileType::CodeText,
        )
        .huffman_table_search,
        HuffmanTableSearch::FileTypeSmall
    ));
    assert!(matches!(
        BlockCompressionConfig::for_level_and_file_type(
            CompressionLevel::Fastest,
            CompressionFileType::ConfigText,
        )
        .huffman_table_search,
        HuffmanTableSearch::FileTypeSmall
    ));
    assert!(matches!(
        BlockCompressionConfig::for_level_and_file_type(
            CompressionLevel::Fastest,
            CompressionFileType::Unknown,
        )
        .huffman_table_search,
        HuffmanTableSearch::FileTypeSmall
    ));
    assert!(matches!(
        BlockCompressionConfig::for_level_and_file_type(
            CompressionLevel::Fastest,
            CompressionFileType::JsonText,
        )
        .huffman_table_search,
        HuffmanTableSearch::Heuristic
    ));
    assert!(matches!(
        BlockCompressionConfig::for_level_and_file_type(
            CompressionLevel::Fastest,
            CompressionFileType::DictionaryText,
        )
        .huffman_table_search,
        HuffmanTableSearch::AllSections
    ));
}

#[test]
fn fastest_config_text_enables_small_single_stream_huffman_override() {
    assert_eq!(
        BlockCompressionConfig::for_level_and_file_type(
            CompressionLevel::Fastest,
            CompressionFileType::ConfigText,
        )
        .file_type_single_stream_huffman_max_literals,
        Some(FILE_TYPE_SINGLE_STREAM_HUFFMAN_MAX_LITERALS)
    );
    assert_eq!(
        BlockCompressionConfig::for_level_and_file_type(
            CompressionLevel::Fastest,
            CompressionFileType::CodeText,
        )
        .file_type_single_stream_huffman_max_literals,
        None
    );
}

#[test]
fn fastest_dictionary_text_keeps_default_predefined_llml_window() {
    let config = BlockCompressionConfig::for_level_and_file_type(
        CompressionLevel::Fastest,
        CompressionFileType::DictionaryText,
    );
    assert_eq!(
        config.file_type_small_sequence_predefined_llml_max_sequences,
        None
    );

    let code_config = BlockCompressionConfig::for_level_and_file_type(
        CompressionLevel::Fastest,
        CompressionFileType::CodeText,
    );
    assert_eq!(
        code_config.file_type_small_sequence_predefined_llml_max_sequences,
        None
    );
}

#[test]
fn fastest_dictionary_text_enables_exact_sequence_mode_search() {
    let dictionary_config = BlockCompressionConfig::for_level_and_hints(
        CompressionLevel::Fastest,
        CompressionFileType::DictionaryText,
        CompressionFileProfile::None,
    );
    assert!(dictionary_config.exact_sequence_mode_search);

    let dependency_json_config = BlockCompressionConfig::for_level_and_hints(
        CompressionLevel::Fastest,
        CompressionFileType::JsonText,
        CompressionFileProfile::DependencyJsonLockfile,
    );
    assert!(dependency_json_config.exact_sequence_mode_search);

    let small_text_lockfile_config = BlockCompressionConfig::for_level_and_hints(
        CompressionLevel::Fastest,
        CompressionFileType::ConfigText,
        CompressionFileProfile::SmallTextLockfile,
    );
    assert!(!small_text_lockfile_config.exact_sequence_mode_search);
    assert_eq!(small_text_lockfile_config.offset_table_max_log, 7);
    assert_eq!(
        small_text_lockfile_config.offset_predefined_max_sequences,
        64
    );
    assert_eq!(small_text_lockfile_config.repeat_table_max_sequences, 256);

    let code_config = BlockCompressionConfig::for_level_and_hints(
        CompressionLevel::Fastest,
        CompressionFileType::CodeText,
        CompressionFileProfile::None,
    );
    assert!(!code_config.exact_sequence_mode_search);
}

#[test]
fn forced_single_stream_huffman_uses_single_stream_size_format() {
    assert_eq!(compressed_literals_size_format(821, false), (0b01, 10));
    assert_eq!(compressed_literals_size_format(821, true), (0b00, 10));
}

#[test]
fn choose_table_repeats_previous_table_for_small_blocks_when_valid() {
    let default = default_of_table();
    let previous = build_table_from_data([29u8, 30, 30].iter().copied(), 8, true);
    let sequences = [
        crate::blocks::sequence_section::Sequence {
            ll: 0,
            ml: 3,
            of: 1 << 29,
        },
        crate::blocks::sequence_section::Sequence {
            ll: 0,
            ml: 3,
            of: 1 << 30,
        },
        crate::blocks::sequence_section::Sequence {
            ll: 0,
            ml: 3,
            of: 1 << 30,
        },
    ];

    assert!(matches!(
        choose_table(
            Some(&previous),
            &default,
            &sequences,
            |seq| encode_offset(seq.of).0,
            8,
            64,
            16,
        ),
        FseTableMode::RepeatLast(_)
    ));
}

#[test]
fn c_fast_sequence_heuristic_repeats_previous_table_before_default() {
    let ll_default = default_ll_table();
    let ml_default = default_ml_table();
    let of_default = default_of_table();
    let ll_previous = build_table_from_data([0u8, 2, 4].iter().copied(), 9, true);
    let sequences = [
        crate::blocks::sequence_section::Sequence {
            ll: 0,
            ml: 3,
            of: 1,
        },
        crate::blocks::sequence_section::Sequence {
            ll: 2,
            ml: 3,
            of: 1,
        },
        crate::blocks::sequence_section::Sequence {
            ll: 4,
            ml: 3,
            of: 1,
        },
    ];

    let (ll_mode, _, _) = choose_sequence_table_modes(
        &sequences,
        SequenceModeSearchConfig {
            ll_previous: Some(&ll_previous),
            ll_default: &ll_default,
            ml_previous: None,
            ml_default: &ml_default,
            of_previous: None,
            of_default: &of_default,
            repeat_table_max_sequences: 1000,
            llml_predefined_max_sequences: 56,
            of_predefined_max_sequences: 28,
            of_max_log: 8,
            exact_sequence_mode_search: false,
            c_fast_heuristics: true,
        },
    );

    assert!(matches!(ll_mode, FseTableMode::RepeatLast(_)));
}

#[test]
fn c_fast_sequence_heuristic_requires_default_table_before_repeat() {
    let ll_default = default_ll_table();
    let ml_default = default_ml_table();
    let of_default = default_of_table();
    let of_previous = build_table_from_data([29u8, 30, 30].iter().copied(), 8, true);
    let sequences = [
        crate::blocks::sequence_section::Sequence {
            ll: 0,
            ml: 3,
            of: 1 << 29,
        },
        crate::blocks::sequence_section::Sequence {
            ll: 0,
            ml: 3,
            of: 1 << 30,
        },
        crate::blocks::sequence_section::Sequence {
            ll: 0,
            ml: 3,
            of: 1 << 30,
        },
    ];

    let (_, _, of_mode) = choose_sequence_table_modes(
        &sequences,
        SequenceModeSearchConfig {
            ll_previous: None,
            ll_default: &ll_default,
            ml_previous: None,
            ml_default: &ml_default,
            of_previous: Some(&of_previous),
            of_default: &of_default,
            repeat_table_max_sequences: 1000,
            llml_predefined_max_sequences: 56,
            of_predefined_max_sequences: 28,
            of_max_log: 8,
            exact_sequence_mode_search: false,
            c_fast_heuristics: true,
        },
    );

    assert!(matches!(of_mode, FseTableMode::Encoded(_)));
}

#[test]
fn exact_sequence_mode_search_never_worsens_threshold_choice() {
    let ll_default = default_ll_table();
    let ml_default = default_ml_table();
    let of_default = default_of_table();

    for a in 0..=8u32 {
        for b in 0..=8u32 {
            for c in 0..=8u32 {
                for d in 0..=8u32 {
                    let sequences = [
                        crate::blocks::sequence_section::Sequence {
                            ll: 0,
                            ml: 3,
                            of: 1u32 << a,
                        },
                        crate::blocks::sequence_section::Sequence {
                            ll: 0,
                            ml: 3,
                            of: 1u32 << b,
                        },
                        crate::blocks::sequence_section::Sequence {
                            ll: 0,
                            ml: 3,
                            of: 1u32 << c,
                        },
                        crate::blocks::sequence_section::Sequence {
                            ll: 0,
                            ml: 3,
                            of: 1u32 << d,
                        },
                    ];
                    let heuristic_ll = choose_table(
                        None,
                        &ll_default,
                        &sequences,
                        |seq| encode_literal_length(seq.ll).0,
                        9,
                        64,
                        16,
                    );
                    let heuristic_ml = choose_table(
                        None,
                        &ml_default,
                        &sequences,
                        |seq| encode_match_len(seq.ml).0,
                        9,
                        64,
                        16,
                    );
                    let heuristic_of = choose_table(
                        None,
                        &of_default,
                        &sequences,
                        |seq| encode_offset(seq.of).0,
                        8,
                        64,
                        16,
                    );
                    let (ll_mode, ml_mode, of_mode) = choose_sequence_table_modes(
                        &sequences,
                        SequenceModeSearchConfig {
                            ll_previous: None,
                            ll_default: &ll_default,
                            ml_previous: None,
                            ml_default: &ml_default,
                            of_previous: None,
                            of_default: &of_default,
                            repeat_table_max_sequences: 64,
                            llml_predefined_max_sequences: 16,
                            of_predefined_max_sequences: 16,
                            of_max_log: 8,
                            exact_sequence_mode_search: true,
                            c_fast_heuristics: false,
                        },
                    );
                    let heuristic_size = exact_sequence_section_size(
                        &sequences,
                        &heuristic_ll,
                        &heuristic_ml,
                        &heuristic_of,
                    );
                    let exact_size =
                        exact_sequence_section_size(&sequences, &ll_mode, &ml_mode, &of_mode);
                    assert!(exact_size <= heuristic_size);
                }
            }
        }
    }
}
