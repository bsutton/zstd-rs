use alloc::vec::Vec;

use crate::{bit_io::BitWriter, huff0::huff0_encoder::HuffmanTable};

use super::literals::{compressed_literals_header_len, write_compressed_literals, LiteralStats};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum HuffmanLiteralMode {
    Compressed,
    Repeat,
}

pub(crate) struct HuffmanLiteralSectionEmission {
    pub(crate) byte_size: usize,
    pub(crate) new_huffman_table: Option<HuffmanTable>,
}

pub(crate) fn build_huffman_literal_table(literals: &[u8]) -> Option<HuffmanTable> {
    if literals.is_empty() {
        return None;
    }

    let (stats, _) = LiteralStats::from_literals_with_stream_counts(literals, false);
    if stats.largest() == literals.len() || stats.likely_incompressible(literals.len()) {
        return None;
    }
    Some(HuffmanTable::build_from_counts(stats.counts()))
}

pub(crate) fn append_huffman_literal_section(
    literals: &[u8],
    previous_table: Option<&HuffmanTable>,
    mode: HuffmanLiteralMode,
    output: &mut Vec<u8>,
) -> Option<HuffmanLiteralSectionEmission> {
    if literals.is_empty() {
        return None;
    }

    let table = match mode {
        HuffmanLiteralMode::Compressed => build_huffman_literal_table(literals)?,
        HuffmanLiteralMode::Repeat => previous_table
            .filter(|table| {
                let (stats, _) = LiteralStats::from_literals_with_stream_counts(literals, false);
                table.can_encode_counts(stats.counts())
            })?
            .clone(),
    };

    append_huffman_literal_section_with_table(literals, &table, mode, output)
}

pub(crate) fn append_huffman_literal_section_with_table(
    literals: &[u8],
    table: &HuffmanTable,
    mode: HuffmanLiteralMode,
    output: &mut Vec<u8>,
) -> Option<HuffmanLiteralSectionEmission> {
    if literals.is_empty() {
        return None;
    }
    if matches!(mode, HuffmanLiteralMode::Repeat) {
        let (stats, _) = LiteralStats::from_literals_with_stream_counts(literals, false);
        if !table.can_encode_counts(stats.counts()) {
            return None;
        }
    }

    let start = output.len();
    let mut writer = BitWriter::from(output);
    let (size_format, size_bits) = sub_block_huffman_size_format(
        literals.len(),
        matches!(mode, HuffmanLiteralMode::Compressed),
    );

    write_compressed_literals(
        literals,
        table,
        matches!(mode, HuffmanLiteralMode::Compressed),
        size_format,
        size_bits,
        &mut writer,
    );
    writer.flush();

    let byte_size = writer.index() / 8 - start;
    let header_size = compressed_literals_header_len(size_format);
    let compressed_size = byte_size.checked_sub(header_size)?;
    if header_size < compressed_literal_header_size(compressed_size)
        || (matches!(mode, HuffmanLiteralMode::Repeat) && compressed_size >= literals.len())
    {
        writer.reset_to(start * 8);
        return None;
    }

    Some(HuffmanLiteralSectionEmission {
        byte_size,
        new_huffman_table: matches!(mode, HuffmanLiteralMode::Compressed).then_some(table.clone()),
    })
}

fn sub_block_huffman_size_format(literal_size: usize, write_entropy: bool) -> (u8, usize) {
    let entropy_guess = if write_entropy { 200 } else { 0 };
    if literal_size >= 16 * 1024usize - entropy_guess {
        (0b11, 18)
    } else if literal_size >= 1024usize - entropy_guess {
        (0b10, 14)
    } else {
        (0b00, 10)
    }
}

fn compressed_literal_header_size(compressed_size: usize) -> usize {
    3 + usize::from(compressed_size >= 1024) + usize::from(compressed_size >= 16 * 1024)
}
