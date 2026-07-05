use std::io;

use ruzstd::decoding::{BlockDecodingStrategy, FrameDecoder};

const ZSTD_MAGIC: u32 = 0xfd2f_b528;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlockType {
    Raw,
    Rle,
    Compressed,
    Reserved,
}

#[derive(Clone, Debug)]
pub struct BlockInfo {
    pub index: usize,
    pub offset: usize,
    pub block_type: BlockType,
    pub last: bool,
    pub content_size: usize,
    pub decompressed_size: Option<usize>,
    pub source_offset: Option<usize>,
    pub section_info: Option<CompressedSectionInfo>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiteralSectionType {
    Raw,
    Rle,
    Compressed,
    Treeless,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SequenceMode {
    Predefined,
    Rle,
    FseCompressed,
    Repeat,
}

#[derive(Clone, Debug)]
pub struct CompressedSectionInfo {
    pub literal_type: LiteralSectionType,
    pub literal_regenerated_size: usize,
    pub literal_payload_size: usize,
    pub literal_streams: Option<u8>,
    pub sequences: usize,
    pub ll_mode: Option<SequenceMode>,
    pub of_mode: Option<SequenceMode>,
    pub ml_mode: Option<SequenceMode>,
}

pub fn inspect_frame(encoded: &[u8]) -> io::Result<Vec<BlockInfo>> {
    let mut offset = frame_header_size(encoded)?;
    let mut blocks = Vec::new();
    loop {
        if offset + 3 > encoded.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "missing block header",
            ));
        }
        let header_offset = offset;
        let raw = u32::from(encoded[offset])
            | (u32::from(encoded[offset + 1]) << 8)
            | (u32::from(encoded[offset + 2]) << 16);
        offset += 3;

        let last = (raw & 1) != 0;
        let block_type = match (raw >> 1) & 0b11 {
            0 => BlockType::Raw,
            1 => BlockType::Rle,
            2 => BlockType::Compressed,
            _ => BlockType::Reserved,
        };
        let block_size = (raw >> 3) as usize;
        let content_size = match block_type {
            BlockType::Raw | BlockType::Compressed => block_size,
            BlockType::Rle => 1,
            BlockType::Reserved => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "reserved block type",
                ))
            }
        };
        if offset + content_size > encoded.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "truncated block payload",
            ));
        }
        blocks.push(BlockInfo {
            index: blocks.len(),
            offset: header_offset,
            block_type,
            last,
            content_size,
            decompressed_size: None,
            source_offset: None,
            section_info: if block_type == BlockType::Compressed {
                Some(inspect_compressed_sections(
                    &encoded[offset..offset + content_size],
                )?)
            } else {
                None
            },
        });
        offset += content_size;
        if last {
            break;
        }
    }
    Ok(blocks)
}

pub fn inspect_frame_with_decoded_sizes(encoded: &[u8]) -> io::Result<Vec<BlockInfo>> {
    let mut blocks = inspect_frame(encoded)?;
    let mut decoder = FrameDecoder::new();
    let mut source = encoded;
    decoder
        .reset(&mut source)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

    let mut source_offset = 0usize;
    for block in &mut blocks {
        let decoded_before = decoder.decoded_size();
        decoder
            .decode_blocks(&mut source, BlockDecodingStrategy::UptoBlocks(1))
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
        let decompressed_size = (decoder.decoded_size() - decoded_before) as usize;
        block.source_offset = Some(source_offset);
        block.decompressed_size = Some(decompressed_size);
        source_offset += decompressed_size;
    }

    Ok(blocks)
}

fn inspect_compressed_sections(payload: &[u8]) -> io::Result<CompressedSectionInfo> {
    let literal = parse_literal_section(payload)?;
    let sequence_offset = literal.header_size + literal.payload_size;
    if sequence_offset > payload.len() {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "literal section exceeds compressed block",
        ));
    }
    let sequence = parse_sequence_header(&payload[sequence_offset..])?;
    Ok(CompressedSectionInfo {
        literal_type: literal.section_type,
        literal_regenerated_size: literal.regenerated_size,
        literal_payload_size: literal.payload_size,
        literal_streams: literal.streams,
        sequences: sequence.sequences,
        ll_mode: sequence.modes.map(|modes| decode_sequence_mode(modes >> 6)),
        of_mode: sequence
            .modes
            .map(|modes| decode_sequence_mode((modes >> 4) & 0x3)),
        ml_mode: sequence
            .modes
            .map(|modes| decode_sequence_mode((modes >> 2) & 0x3)),
    })
}

struct LiteralSectionHeader {
    section_type: LiteralSectionType,
    regenerated_size: usize,
    payload_size: usize,
    header_size: usize,
    streams: Option<u8>,
}

fn parse_literal_section(payload: &[u8]) -> io::Result<LiteralSectionHeader> {
    let Some(&first) = payload.first() else {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "missing literal section",
        ));
    };
    let section_type = match first & 0x3 {
        0 => LiteralSectionType::Raw,
        1 => LiteralSectionType::Rle,
        2 => LiteralSectionType::Compressed,
        _ => LiteralSectionType::Treeless,
    };
    let size_format = (first >> 2) & 0x3;

    let header_size = match section_type {
        LiteralSectionType::Raw | LiteralSectionType::Rle => match size_format {
            0 | 2 => 1,
            1 => 2,
            3 => 3,
            _ => unreachable!(),
        },
        LiteralSectionType::Compressed | LiteralSectionType::Treeless => match size_format {
            0 | 1 => 3,
            2 => 4,
            3 => 5,
            _ => unreachable!(),
        },
    };
    if payload.len() < header_size {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "truncated literal section header",
        ));
    }

    let (regenerated_size, compressed_size, streams) = match section_type {
        LiteralSectionType::Raw | LiteralSectionType::Rle => {
            let regenerated_size = match size_format {
                0 | 2 => usize::from(first >> 3),
                1 => usize::from(first >> 4) + (usize::from(payload[1]) << 4),
                3 => {
                    usize::from(first >> 4)
                        + (usize::from(payload[1]) << 4)
                        + (usize::from(payload[2]) << 12)
                }
                _ => unreachable!(),
            };
            (regenerated_size, None, None)
        }
        LiteralSectionType::Compressed | LiteralSectionType::Treeless => {
            let streams = if size_format == 0 { 1 } else { 4 };
            let (regenerated_size, compressed_size) = match size_format {
                0 | 1 => (
                    usize::from(first >> 4) + ((usize::from(payload[1]) & 0x3f) << 4),
                    usize::from(payload[1] >> 6) + (usize::from(payload[2]) << 2),
                ),
                2 => (
                    usize::from(first >> 4)
                        + (usize::from(payload[1]) << 4)
                        + ((usize::from(payload[2]) & 0x3) << 12),
                    (usize::from(payload[2]) >> 2) + (usize::from(payload[3]) << 6),
                ),
                3 => (
                    usize::from(first >> 4)
                        + (usize::from(payload[1]) << 4)
                        + ((usize::from(payload[2]) & 0x3f) << 12),
                    (usize::from(payload[2]) >> 6)
                        + (usize::from(payload[3]) << 2)
                        + (usize::from(payload[4]) << 10),
                ),
                _ => unreachable!(),
            };
            (regenerated_size, Some(compressed_size), Some(streams))
        }
    };

    let payload_size = compressed_size.unwrap_or_else(|| match section_type {
        LiteralSectionType::Rle => 1,
        LiteralSectionType::Raw => regenerated_size,
        LiteralSectionType::Compressed | LiteralSectionType::Treeless => unreachable!(),
    });
    Ok(LiteralSectionHeader {
        section_type,
        regenerated_size,
        payload_size,
        header_size,
        streams,
    })
}

struct SequenceHeader {
    sequences: usize,
    modes: Option<u8>,
}

fn parse_sequence_header(payload: &[u8]) -> io::Result<SequenceHeader> {
    let Some(&first) = payload.first() else {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "missing sequence section",
        ));
    };
    match first {
        0 => Ok(SequenceHeader {
            sequences: 0,
            modes: None,
        }),
        1..=127 => {
            let Some(&modes) = payload.get(1) else {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "missing one-byte sequence modes",
                ));
            };
            Ok(SequenceHeader {
                sequences: usize::from(first),
                modes: Some(modes),
            })
        }
        128..=254 => {
            let Some(&low) = payload.get(1) else {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "truncated two-byte sequence count",
                ));
            };
            let sequences = ((usize::from(first) - 128) << 8) + usize::from(low);
            let modes = if sequences == 0 {
                None
            } else {
                Some(*payload.get(2).ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "missing two-byte sequence modes",
                    )
                })?)
            };
            Ok(SequenceHeader { sequences, modes })
        }
        255 => {
            if payload.len() < 4 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "truncated three-byte sequence count",
                ));
            }
            Ok(SequenceHeader {
                sequences: usize::from(payload[1]) + (usize::from(payload[2]) << 8) + 0x7f00,
                modes: Some(payload[3]),
            })
        }
    }
}

fn decode_sequence_mode(mode: u8) -> SequenceMode {
    match mode {
        0 => SequenceMode::Predefined,
        1 => SequenceMode::Rle,
        2 => SequenceMode::FseCompressed,
        3 => SequenceMode::Repeat,
        _ => unreachable!(),
    }
}

fn frame_header_size(encoded: &[u8]) -> io::Result<usize> {
    if encoded.len() < 5 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "missing frame header",
        ));
    }
    let magic = u32::from_le_bytes([encoded[0], encoded[1], encoded[2], encoded[3]]);
    if magic != ZSTD_MAGIC {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unexpected zstd magic: {magic:#x}"),
        ));
    }

    let descriptor = encoded[4];
    let single_segment = (descriptor & 0b0010_0000) != 0;
    let dict_id_len = match descriptor & 0b0000_0011 {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => 4,
    };
    let fcs_len = match descriptor >> 6 {
        0 if single_segment => 1,
        0 => 0,
        1 => 2,
        2 => 4,
        _ => 8,
    };
    let header_size = 5 + usize::from(!single_segment) + dict_id_len + fcs_len;
    if header_size > encoded.len() {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "truncated frame header",
        ));
    }
    Ok(header_size)
}
