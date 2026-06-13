use alloc::vec::Vec;

use crate::{bit_io::BitWriter, huff0::huff0_encoder};

pub(super) const COMPRESS_LITERALS_SIZE_MIN: usize = 63;
pub(super) const REPEAT_LITERALS_SIZE_MIN: usize = 6;
pub(super) const HUFFMAN_4_STREAMS_MIN: usize = 256;
pub(super) const REPEAT_SINGLE_STREAM_LITERALS_MAX: usize = 1024;
pub(super) const FAST_LITERAL_MIN_GAIN_LOG: u32 = 6;

pub(super) fn should_compress_literals(len: usize, has_previous_table: bool) -> bool {
    let min_size = if has_previous_table {
        REPEAT_LITERALS_SIZE_MIN
    } else {
        COMPRESS_LITERALS_SIZE_MIN
    };
    len > min_size
}

pub(super) fn raw_literals(literals: &[u8], writer: &mut BitWriter<&mut Vec<u8>>) {
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

pub(super) fn rle_literals(literals: &[u8], writer: &mut BitWriter<&mut Vec<u8>>) {
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

pub(super) fn compress_literals(
    literals: &[u8],
    last_table: Option<&huff0_encoder::HuffmanTable>,
    search_smallest_table: bool,
    force_single_stream_max_literals: Option<usize>,
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

    let force_single_stream =
        force_single_stream_max_literals.is_some_and(|max_literals| literals.len() <= max_literals);
    let (size_format, size_bits) =
        compressed_literals_size_format(literals.len(), force_single_stream);
    let four_streams = size_format != 0;
    let header_len = compressed_literals_header_len(size_format);
    let new_encoder_table = if search_smallest_table {
        huff0_encoder::HuffmanTable::build_smallest_from_counts(
            literal_stats.counts(),
            literals,
            four_streams,
        )
    } else {
        huff0_encoder::HuffmanTable::build_from_counts(literal_stats.counts())
    };
    let new_len = new_encoder_table.encoded_len(literals, true, four_streams);
    let new_choice = LiteralEncodingChoice {
        encoder_table: &new_encoder_table,
        new_table: true,
        estimated_len: new_len,
        size_format,
        size_bits,
        header_len,
    };
    let choice = last_table
        .and_then(|previous_table| {
            repeat_huffman_choice(
                previous_table,
                &literal_stats,
                literals,
                new_choice,
                force_single_stream,
            )
        })
        .unwrap_or(new_choice);

    if !literal_estimate_has_enough_gain(choice.estimated_len, choice.header_len, literals.len()) {
        raw_literals(literals, writer);
        return None;
    }

    write_compressed_literals(
        literals,
        choice.encoder_table,
        choice.new_table,
        choice.size_format,
        choice.size_bits,
        writer,
    );
    let total_len = (writer.index() - reset_idx) / 8;

    // If encoded len is bigger than the raw literals we are better off just writing the raw literals here
    if total_len >= literals.len() {
        writer.reset_to(reset_idx);
        raw_literals(literals, writer);
        None
    } else if choice.new_table {
        Some(new_encoder_table)
    } else {
        None
    }
}

#[derive(Clone, Copy)]
struct LiteralEncodingChoice<'table> {
    encoder_table: &'table huff0_encoder::HuffmanTable,
    new_table: bool,
    estimated_len: usize,
    size_format: u8,
    size_bits: usize,
    header_len: usize,
}

impl LiteralEncodingChoice<'_> {
    fn total_estimated_len(self) -> usize {
        self.estimated_len + self.header_len
    }
}

fn repeat_huffman_choice<'table>(
    previous_table: &'table huff0_encoder::HuffmanTable,
    literal_stats: &LiteralStats,
    literals: &[u8],
    new_choice: LiteralEncodingChoice<'_>,
    force_single_stream: bool,
) -> Option<LiteralEncodingChoice<'table>> {
    if !previous_table.can_encode_counts(literal_stats.counts()) {
        return None;
    }

    let (size_format, size_bits) =
        compressed_literals_repeat_size_format(literals.len(), force_single_stream);
    let header_len = compressed_literals_header_len(size_format);
    let four_streams = size_format != 0;
    let estimated_len = previous_table.encoded_len(literals, false, four_streams);
    if estimated_len < literals.len()
        && estimated_len + header_len <= new_choice.total_estimated_len()
    {
        Some(LiteralEncodingChoice {
            encoder_table: previous_table,
            new_table: false,
            estimated_len,
            size_format,
            size_bits,
            header_len,
        })
    } else {
        None
    }
}

pub(super) fn compressed_literals_size_format(
    len: usize,
    force_single_stream: bool,
) -> (u8, usize) {
    if force_single_stream && len < HUFFMAN_4_STREAMS_MIN * 4 {
        return (0b00u8, 10);
    }

    match len {
        0..HUFFMAN_4_STREAMS_MIN => (0b00u8, 10),
        HUFFMAN_4_STREAMS_MIN..1024 => (0b01, 10),
        1024..16384 => (0b10, 14),
        16384..262144 => (0b11, 18),
        _ => unimplemented!("too many literals"),
    }
}

fn compressed_literals_repeat_size_format(len: usize, force_single_stream: bool) -> (u8, usize) {
    if force_single_stream || len < REPEAT_SINGLE_STREAM_LITERALS_MAX {
        return (0b00, 10);
    }

    compressed_literals_size_format(len, false)
}

pub(super) fn compressed_literals_header_len(size_format: u8) -> usize {
    match size_format {
        0b00 | 0b01 => 3,
        0b10 => 4,
        0b11 => 5,
        _ => unreachable!(),
    }
}

pub(super) fn literal_min_gain(len: usize) -> usize {
    (len >> FAST_LITERAL_MIN_GAIN_LOG) + 2
}

pub(super) fn literal_estimate_has_enough_gain(
    estimated_len: usize,
    header_len: usize,
    literal_len: usize,
) -> bool {
    estimated_len < literal_len.saturating_sub(literal_min_gain(literal_len))
        && estimated_len + header_len < literal_len
}

fn write_compressed_literals(
    literals: &[u8],
    encoder_table: &huff0_encoder::HuffmanTable,
    new_table: bool,
    size_format: u8,
    size_bits: usize,
    writer: &mut BitWriter<&mut Vec<u8>>,
) {
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
}

pub(super) struct LiteralStats {
    counts: [usize; 256],
    max_symbol: usize,
    largest: usize,
}

impl LiteralStats {
    pub(super) fn from_literals(literals: &[u8]) -> Self {
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

    pub(super) fn counts(&self) -> &[usize] {
        &self.counts[..=self.max_symbol]
    }

    pub(super) fn likely_incompressible(&self, len: usize) -> bool {
        self.largest <= (len >> 7) + 4
    }
}
