use alloc::vec::Vec;
use core::convert::TryFrom;

use crate::{
    bit_io::BitWriter,
    encoding::frame_compressor::CompressState,
    encoding::{Matcher, Sequence},
    fse::fse_encoder::{build_table_from_data, FSETable, State},
    huff0::huff0_encoder,
};

const INITIAL_LITERALS_CAPACITY: usize = 1024;
const INITIAL_SEQUENCES_CAPACITY: usize = 256;
const COMPRESS_LITERALS_SIZE_MIN: usize = 63;
const REPEAT_LITERALS_SIZE_MIN: usize = 6;
const HUFFMAN_4_STREAMS_MIN: usize = 256;
const FAST_LITERAL_MIN_GAIN_LOG: u32 = 6;
const LITERAL_LENGTH_SMALL_CODES: [(u8, u32, usize); 64] = small_literal_length_codes();
const MATCH_LENGTH_SMALL_CODES: [(u8, u32, usize); 128] = small_match_length_codes();

/// A block of [`crate::common::BlockType::Compressed`]
pub fn compress_block<M: Matcher>(
    state: &mut CompressState<M>,
    output: &mut Vec<u8>,
) -> Option<huff0_encoder::HuffmanTable> {
    let mut literals_vec = Vec::with_capacity(INITIAL_LITERALS_CAPACITY);
    let mut sequences = Vec::with_capacity(INITIAL_SEQUENCES_CAPACITY);
    let mut new_huffman_table = None;
    let offset_history = &mut state.offset_history;
    let (newest, second, third) = offset_history.as_offsets();
    state.matcher.set_repeat_offsets(newest, second, third);
    state.matcher.start_matching(|seq| match seq {
        Sequence::Literals { literals } => literals_vec.extend_from_slice(literals),
        Sequence::Triple {
            literals,
            offset,
            match_len,
        } => {
            literals_vec.extend_from_slice(literals);
            let offset = offset_to_u32(offset);
            sequences.push(crate::blocks::sequence_section::Sequence {
                ll: literals.len() as u32,
                ml: match_len as u32,
                of: offset_history.encode_offset_value(offset, literals.len() as u32),
            });
        }
    });

    // literals section

    let mut writer = BitWriter::from(output);
    if should_compress_literals(literals_vec.len(), state.last_huff_table.is_some()) {
        if let Some(table) =
            compress_literals(&literals_vec, state.last_huff_table.as_ref(), &mut writer)
        {
            new_huffman_table = Some(table);
        }
    } else {
        raw_literals(&literals_vec, &mut writer);
    }

    // sequences section

    if sequences.is_empty() {
        writer.write_bits(0u8, 8);
    } else {
        encode_seqnum(sequences.len(), &mut writer);

        // Choose the tables.
        let ll_mode = choose_table(
            state.fse_tables.ll_previous.as_ref(),
            &state.fse_tables.ll_default,
            &sequences,
            |seq| encode_literal_length(seq.ll).0,
            9,
        );
        let ml_mode = choose_table(
            state.fse_tables.ml_previous.as_ref(),
            &state.fse_tables.ml_default,
            &sequences,
            |seq| encode_match_len(seq.ml).0,
            9,
        );
        let of_mode = choose_table(
            state.fse_tables.of_previous.as_ref(),
            &state.fse_tables.of_default,
            &sequences,
            |seq| encode_offset(seq.of).0,
            8,
        );

        writer.write_bits(encode_fse_table_modes(&ll_mode, &ml_mode, &of_mode), 8);

        encode_table(&ll_mode, &mut writer);
        encode_table(&of_mode, &mut writer);
        encode_table(&ml_mode, &mut writer);

        encode_sequences(&sequences, &mut writer, &ll_mode, &ml_mode, &of_mode);

        match ll_mode {
            FseTableMode::Encoded(table) => state.fse_tables.ll_previous = Some(table),
            FseTableMode::Predefined(_) => state.fse_tables.ll_previous = None,
            FseTableMode::Rle(_) => state.fse_tables.ll_previous = None,
            FseTableMode::RepeateLast(_) => {}
        }
        match ml_mode {
            FseTableMode::Encoded(table) => state.fse_tables.ml_previous = Some(table),
            FseTableMode::Predefined(_) => state.fse_tables.ml_previous = None,
            FseTableMode::Rle(_) => state.fse_tables.ml_previous = None,
            FseTableMode::RepeateLast(_) => {}
        }
        match of_mode {
            FseTableMode::Encoded(table) => state.fse_tables.of_previous = Some(table),
            FseTableMode::Predefined(_) => state.fse_tables.of_previous = None,
            FseTableMode::Rle(_) => state.fse_tables.of_previous = None,
            FseTableMode::RepeateLast(_) => {}
        }
    }
    writer.flush();
    new_huffman_table
}

#[inline(always)]
fn offset_to_u32(offset: usize) -> u32 {
    match u32::try_from(offset) {
        Ok(offset) => offset,
        Err(_) => unreachable!("match offsets are bounded by the compressor window"),
    }
}

#[derive(Clone)]
#[allow(clippy::large_enum_variant)]
enum FseTableMode<'a> {
    Predefined(&'a FSETable),
    Rle(u8),
    Encoded(FSETable),
    RepeateLast(&'a FSETable),
}

impl FseTableMode<'_> {
    pub fn table(&self) -> Option<&FSETable> {
        match self {
            Self::Predefined(t) => Some(t),
            Self::RepeateLast(t) => Some(t),
            Self::Encoded(t) => Some(t),
            Self::Rle(_) => None,
        }
    }
}

fn choose_table<'a>(
    previous: Option<&'a FSETable>,
    default_table: &'a FSETable,
    sequences: &[crate::blocks::sequence_section::Sequence],
    code: impl Fn(&crate::blocks::sequence_section::Sequence) -> u8 + Copy,
    max_log: u8,
) -> FseTableMode<'a> {
    let first_code = code(&sequences[0]);
    let all_same_code = sequences
        .iter()
        .skip(1)
        .all(|sequence| code(sequence) == first_code);

    if all_same_code && sequences.len() > 2 {
        return FseTableMode::Rle(first_code);
    }

    if sequences.len() <= 16
        && sequences
            .iter()
            .all(|sequence| default_table.can_encode_symbol(code(sequence)))
    {
        return FseTableMode::Predefined(default_table);
    }

    if all_same_code {
        return FseTableMode::Rle(first_code);
    }

    if let Some(previous) = previous {
        if sequences.len() < 64
            && sequences
                .iter()
                .all(|sequence| previous.can_encode_symbol(code(sequence)))
        {
            return FseTableMode::RepeateLast(previous);
        }
    }

    FseTableMode::Encoded(build_table_from_data(
        sequences.iter().map(code),
        max_log,
        true,
    ))
}

fn encode_table(mode: &FseTableMode<'_>, writer: &mut BitWriter<&mut Vec<u8>>) {
    match mode {
        FseTableMode::Predefined(_) => {}
        FseTableMode::Rle(symbol) => writer.write_bits(*symbol, 8),
        FseTableMode::RepeateLast(_) => {}
        FseTableMode::Encoded(table) => table.write_table(writer),
    }
}

fn encode_fse_table_modes(
    ll_mode: &FseTableMode<'_>,
    ml_mode: &FseTableMode<'_>,
    of_mode: &FseTableMode<'_>,
) -> u8 {
    fn mode_to_bits(mode: &FseTableMode<'_>) -> u8 {
        match mode {
            FseTableMode::Predefined(_) => 0,
            FseTableMode::Rle(_) => 1,
            FseTableMode::Encoded(_) => 2,
            FseTableMode::RepeateLast(_) => 3,
        }
    }
    mode_to_bits(ll_mode) << 6 | mode_to_bits(of_mode) << 4 | mode_to_bits(ml_mode) << 2
}

fn should_compress_literals(len: usize, has_previous_table: bool) -> bool {
    let min_size = if has_previous_table {
        REPEAT_LITERALS_SIZE_MIN
    } else {
        COMPRESS_LITERALS_SIZE_MIN
    };
    len > min_size
}

fn encode_sequences(
    sequences: &[crate::blocks::sequence_section::Sequence],
    writer: &mut BitWriter<&mut Vec<u8>>,
    ll_mode: &FseTableMode<'_>,
    ml_mode: &FseTableMode<'_>,
    of_mode: &FseTableMode<'_>,
) {
    let sequence = sequences[sequences.len() - 1];
    let ll_table = ll_mode.table();
    let ml_table = ml_mode.table();
    let of_table = of_mode.table();
    let (ll_code, ll_add_bits, ll_num_bits) = encode_literal_length(sequence.ll);
    let (of_code, of_add_bits, of_num_bits) = encode_offset(sequence.of);
    let (ml_code, ml_add_bits, ml_num_bits) = encode_match_len(sequence.ml);
    let mut ll_state = init_fse_state(ll_mode, ll_code);
    let mut ml_state = init_fse_state(ml_mode, ml_code);
    let mut of_state = init_fse_state(of_mode, of_code);

    writer.write_bits(ll_add_bits, ll_num_bits);
    writer.write_bits(ml_add_bits, ml_num_bits);
    writer.write_bits(of_add_bits, of_num_bits);

    // Encode backwards so the decoder reads the first sequence first.
    let mut sequence_idx = sequences.len() - 1;
    while sequence_idx > 0 {
        sequence_idx -= 1;
        let sequence = sequences[sequence_idx];
        let (ll_code, ll_add_bits, ll_num_bits) = encode_literal_length(sequence.ll);
        let (of_code, of_add_bits, of_num_bits) = encode_offset(sequence.of);
        let (ml_code, ml_add_bits, ml_num_bits) = encode_match_len(sequence.ml);

        {
            update_fse_state(of_table, &mut of_state, of_code, writer);
        }
        {
            update_fse_state(ml_table, &mut ml_state, ml_code, writer);
        }
        {
            update_fse_state(ll_table, &mut ll_state, ll_code, writer);
        }

        writer.write_bits(ll_add_bits, ll_num_bits);
        writer.write_bits(ml_add_bits, ml_num_bits);
        writer.write_bits(of_add_bits, of_num_bits);
    }
    flush_fse_state(ml_table, ml_state, writer);
    flush_fse_state(of_table, of_state, writer);
    flush_fse_state(ll_table, ll_state, writer);

    let bits_to_fill = writer.misaligned();
    if bits_to_fill == 0 {
        writer.write_bits(1u32, 8);
    } else {
        writer.write_bits(1u32, bits_to_fill);
    }
}

fn init_fse_state<'a>(mode: &'a FseTableMode<'_>, symbol: u8) -> Option<&'a State> {
    match mode {
        FseTableMode::Rle(rle_symbol) => {
            debug_assert_eq!(*rle_symbol, symbol);
            None
        }
        _ => mode.table().map(|table| table.start_state(symbol)),
    }
}

fn update_fse_state<'a>(
    table: Option<&'a FSETable>,
    state: &mut Option<&'a State>,
    symbol: u8,
    writer: &mut BitWriter<&mut Vec<u8>>,
) {
    if let Some(table) = table {
        if let Some(current) = *state {
            let next = table.next_state(symbol, current.index);
            let diff = current.index - next.baseline;
            writer.write_bits(diff as u64, next.num_bits as usize);
            *state = Some(next);
        } else {
            unreachable!("non-RLE FSE mode must have a state");
        }
    }
}

fn flush_fse_state(
    table: Option<&FSETable>,
    state: Option<&State>,
    writer: &mut BitWriter<&mut Vec<u8>>,
) {
    if let Some(table) = table {
        if let Some(state) = state {
            writer.write_bits(state.index as u64, table.acc_log() as usize);
        } else {
            unreachable!("non-RLE FSE mode must have a state");
        }
    }
}

fn encode_seqnum(seqnum: usize, writer: &mut BitWriter<impl AsMut<Vec<u8>>>) {
    const UPPER_LIMIT: usize = 0xFFFF + 0x7F00;
    match seqnum {
        1..=127 => writer.write_bits(seqnum as u32, 8),
        128..=0x7FFF => {
            let upper = ((seqnum >> 8) | 0x80) as u8;
            let lower = seqnum as u8;
            writer.write_bits(upper, 8);
            writer.write_bits(lower, 8);
        }
        0x8000..=UPPER_LIMIT => {
            let encode = seqnum - 0x7F00;
            let upper = (encode >> 8) as u8;
            let lower = encode as u8;
            writer.write_bits(255u8, 8);
            writer.write_bits(upper, 8);
            writer.write_bits(lower, 8);
        }
        _ => unreachable!(),
    }
}

#[inline(always)]
fn encode_literal_length(len: u32) -> (u8, u32, usize) {
    if len < LITERAL_LENGTH_SMALL_CODES.len() as u32 {
        return LITERAL_LENGTH_SMALL_CODES[len as usize];
    }

    match len {
        0..=63 => unreachable!(),
        64..=127 => (25, len - 64, 6),
        128..=255 => (26, len - 128, 7),
        256..=511 => (27, len - 256, 8),
        512..=1023 => (28, len - 512, 9),
        1024..=2047 => (29, len - 1024, 10),
        2048..=4095 => (30, len - 2048, 11),
        4096..=8191 => (31, len - 4096, 12),
        8192..=16383 => (32, len - 8192, 13),
        16384..=32767 => (33, len - 16384, 14),
        32768..=65535 => (34, len - 32768, 15),
        65536..=131071 => (35, len - 65536, 16),
        131072.. => unreachable!(),
    }
}

#[inline(always)]
fn encode_match_len(len: u32) -> (u8, u32, usize) {
    if (3..=130).contains(&len) {
        return MATCH_LENGTH_SMALL_CODES[(len - 3) as usize];
    }

    match len {
        0..=2 => unreachable!(),
        3..=130 => unreachable!(),
        131..=258 => (43, len - 131, 7),
        259..=514 => (44, len - 259, 8),
        515..=1026 => (45, len - 515, 9),
        1027..=2050 => (46, len - 1027, 10),
        2051..=4098 => (47, len - 2051, 11),
        4099..=8194 => (48, len - 4099, 12),
        8195..=16386 => (49, len - 8195, 13),
        16387..=32770 => (50, len - 16387, 14),
        32771..=65538 => (51, len - 32771, 15),
        65539..=131074 => (52, len - 65539, 16),
        131075.. => unreachable!(),
    }
}

const fn small_literal_length_codes() -> [(u8, u32, usize); 64] {
    let mut codes = [(0, 0, 0); 64];
    let mut len = 0usize;
    while len < codes.len() {
        codes[len] = match len {
            0..=15 => (len as u8, 0, 0),
            16..=17 => (16, len as u32 - 16, 1),
            18..=19 => (17, len as u32 - 18, 1),
            20..=21 => (18, len as u32 - 20, 1),
            22..=23 => (19, len as u32 - 22, 1),
            24..=27 => (20, len as u32 - 24, 2),
            28..=31 => (21, len as u32 - 28, 2),
            32..=39 => (22, len as u32 - 32, 3),
            40..=47 => (23, len as u32 - 40, 3),
            48..=63 => (24, len as u32 - 48, 4),
            _ => unreachable!(),
        };
        len += 1;
    }
    codes
}

const fn small_match_length_codes() -> [(u8, u32, usize); 128] {
    let mut codes = [(0, 0, 0); 128];
    let mut idx = 0usize;
    while idx < codes.len() {
        let len = idx + 3;
        codes[idx] = match len {
            3..=34 => (len as u8 - 3, 0, 0),
            35..=36 => (32, len as u32 - 35, 1),
            37..=38 => (33, len as u32 - 37, 1),
            39..=40 => (34, len as u32 - 39, 1),
            41..=42 => (35, len as u32 - 41, 1),
            43..=46 => (36, len as u32 - 43, 2),
            47..=50 => (37, len as u32 - 47, 2),
            51..=58 => (38, len as u32 - 51, 3),
            59..=66 => (39, len as u32 - 59, 3),
            67..=82 => (40, len as u32 - 67, 4),
            83..=98 => (41, len as u32 - 83, 4),
            99..=130 => (42, len as u32 - 99, 5),
            _ => unreachable!(),
        };
        idx += 1;
    }
    codes
}

fn encode_offset(len: u32) -> (u8, u32, usize) {
    let log = len.ilog2();
    let lower = len & ((1 << log) - 1);
    (log as u8, lower, log as usize)
}

fn raw_literals(literals: &[u8], writer: &mut BitWriter<&mut Vec<u8>>) {
    writer.write_bits(0u8, 2); // Raw_Literals_Block
    match literals.len() {
        0..=31 => {
            writer.write_bits(0u8, 1);
            writer.write_bits(literals.len() as u32, 5);
        }
        32..=4095 => {
            writer.write_bits(0b01u8, 2);
            writer.write_bits(literals.len() as u32, 12);
        }
        4096..=1_048_575 => {
            writer.write_bits(0b11u8, 2);
            writer.write_bits(literals.len() as u32, 20);
        }
        _ => unimplemented!("too many literals"),
    }
    writer.append_bytes(literals);
}

fn rle_literals(literals: &[u8], writer: &mut BitWriter<&mut Vec<u8>>) {
    debug_assert!(!literals.is_empty());
    writer.write_bits(1u8, 2); // RLE_Literals_Block
    match literals.len() {
        0..=31 => {
            writer.write_bits(0u8, 1);
            writer.write_bits(literals.len() as u32, 5);
        }
        32..=4095 => {
            writer.write_bits(0b01u8, 2);
            writer.write_bits(literals.len() as u32, 12);
        }
        4096..=1_048_575 => {
            writer.write_bits(0b11u8, 2);
            writer.write_bits(literals.len() as u32, 20);
        }
        _ => unimplemented!("too many literals"),
    }
    writer.write_bits(literals[0], 8);
}

fn compress_literals(
    literals: &[u8],
    last_table: Option<&huff0_encoder::HuffmanTable>,
    writer: &mut BitWriter<&mut Vec<u8>>,
) -> Option<huff0_encoder::HuffmanTable> {
    let reset_idx = writer.index();

    let literal_stats = LiteralStats::from_literals(literals);
    if literal_stats.largest == literals.len()
        || literal_stats.likely_incompressible(literals.len())
    {
        if !literals.is_empty() && literal_stats.largest == literals.len() {
            rle_literals(literals, writer);
        } else {
            raw_literals(literals, writer);
        }
        return None;
    }

    let (size_format, size_bits) = match literals.len() {
        0..HUFFMAN_4_STREAMS_MIN => (0b00u8, 10),
        HUFFMAN_4_STREAMS_MIN..1024 => (0b01, 10),
        1024..16384 => (0b10, 14),
        16384..262144 => (0b11, 18),
        _ => unimplemented!("too many literals"),
    };

    let header_len = compressed_literals_header_len(size_format);
    let new_encoder_table = huff0_encoder::HuffmanTable::build_from_counts(literal_stats.counts());
    let new_len = new_encoder_table.encoded_len(literals, true, size_format != 0);
    let (encoder_table, new_table, estimated_len) = if let Some(previous_table) = last_table {
        if previous_table.can_encode(&new_encoder_table).is_some() {
            let four_streams = size_format != 0;
            let previous_len = previous_table.encoded_len(literals, false, four_streams);
            if previous_len < literals.len() && previous_len <= new_len {
                (previous_table, false, previous_len)
            } else {
                (&new_encoder_table, true, new_len)
            }
        } else {
            (&new_encoder_table, true, new_len)
        }
    } else {
        (&new_encoder_table, true, new_len)
    };

    if estimated_len
        >= literals
            .len()
            .saturating_sub(literal_min_gain(literals.len()))
        || estimated_len + header_len >= literals.len()
    {
        raw_literals(literals, writer);
        return None;
    }

    if new_table {
        writer.write_bits(2u8, 2); // compressed literals type
    } else {
        writer.write_bits(3u8, 2); // treeless compressed literals type
    }

    writer.write_bits(size_format, 2);
    writer.write_bits(literals.len() as u32, size_bits);
    let size_index = writer.index();
    writer.write_bits(0u32, size_bits);
    let index_before = writer.index();
    let mut encoder = huff0_encoder::HuffmanEncoder::new(encoder_table, writer);
    if size_format == 0 {
        encoder.encode(literals, new_table)
    } else {
        encoder.encode4x(literals, new_table)
    };
    let encoded_len = (writer.index() - index_before) / 8;
    writer.change_bits(size_index, encoded_len as u64, size_bits);
    let total_len = (writer.index() - reset_idx) / 8;

    // If encoded len is bigger than the raw literals we are better off just writing the raw literals here
    if total_len >= literals.len() {
        writer.reset_to(reset_idx);
        raw_literals(literals, writer);
        None
    } else if new_table {
        Some(new_encoder_table)
    } else {
        None
    }
}

fn compressed_literals_header_len(size_format: u8) -> usize {
    match size_format {
        0b00 | 0b01 => 3,
        0b10 => 4,
        0b11 => 5,
        _ => unreachable!(),
    }
}

fn literal_min_gain(len: usize) -> usize {
    (len >> FAST_LITERAL_MIN_GAIN_LOG) + 2
}

struct LiteralStats {
    counts: [usize; 256],
    max_symbol: usize,
    largest: usize,
}

impl LiteralStats {
    fn from_literals(literals: &[u8]) -> Self {
        let mut counts = [0; 256];
        let mut max_symbol = 0usize;
        let mut largest = 0usize;
        for literal in literals {
            let symbol = *literal as usize;
            counts[symbol] += 1;
            largest = largest.max(counts[symbol]);
            max_symbol = max_symbol.max(symbol);
        }
        Self {
            counts,
            max_symbol,
            largest,
        }
    }

    fn counts(&self) -> &[usize] {
        &self.counts[..=self.max_symbol]
    }

    fn likely_incompressible(&self, len: usize) -> bool {
        self.largest <= (len >> 7) + 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoding::frame_compressor::{CompressState, FseTables, OffsetHistory};
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

        fn get_last_space(&mut self) -> &[u8] {
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
        assert!(!literals.is_empty());

        let mut state = CompressState {
            matcher: LiteralPayloadMatcher {
                literals: literals.clone(),
                emitted: false,
            },
            last_huff_table,
            fse_tables: FseTables::new(),
            offset_history: OffsetHistory::new(),
        };
        let mut block_payload = Vec::new();

        compress_block(&mut state, &mut block_payload);

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
                9
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
                9
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
                9
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
        let previous_table =
            huff0_encoder::HuffmanTable::build_from_counts(&[8, 1, 1, 1, 1, 1, 1, 1]);
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
    fn literal_min_gain_boundary_uses_raw_literals_and_round_trips() {
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

        let (block_payload, frame, expected) = compressed_frame_with_literal_payload(literals);

        assert_eq!(
            block_payload[0] & 0b11,
            0,
            "narrow literal gains should fall back to raw literals"
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
                8
            ),
            FseTableMode::RepeateLast(_)
        ));
    }
}
