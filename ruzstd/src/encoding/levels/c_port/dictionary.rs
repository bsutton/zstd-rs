//! Dictionary metadata handling ported from `ZSTD_compress_insertDictionary()`.

use alloc::boxed::Box;
use core::fmt;

use crate::decoding::dictionary::{Dictionary, MAGIC_NUM};
use crate::encoding::frame_compressor::FseTables;
use crate::fse::fse_encoder;
use crate::huff0::huff0_encoder;

use super::sequence_store::RepeatOffsets;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DictionaryContentType {
    Auto,
    RawContent,
    FullDict,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DictionaryKind {
    RawContent,
    Full,
}

#[derive(Clone)]
pub(crate) struct ParsedDictionary<'a> {
    pub(crate) kind: DictionaryKind,
    pub(crate) dict_id: u32,
    pub(crate) content: &'a [u8],
    pub(crate) repeat_offsets: RepeatOffsets,
    entropy: Option<Box<DictionaryEntropy>>,
}

impl ParsedDictionary<'_> {
    pub(crate) fn initial_fse_tables(&self) -> FseTables {
        self.entropy
            .as_ref()
            .map(|entropy| entropy.fse_tables.clone())
            .unwrap_or_else(FseTables::new)
    }

    pub(crate) fn initial_huffman_table(&self) -> Option<huff0_encoder::HuffmanTable> {
        self.entropy
            .as_ref()
            .map(|entropy| entropy.huffman_table.clone())
    }
}

impl fmt::Debug for ParsedDictionary<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ParsedDictionary")
            .field("kind", &self.kind)
            .field("dict_id", &self.dict_id)
            .field("content", &self.content)
            .field("repeat_offsets", &self.repeat_offsets)
            .field("has_entropy", &self.entropy.is_some())
            .finish()
    }
}

impl PartialEq for ParsedDictionary<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.dict_id == other.dict_id
            && self.content == other.content
            && self.repeat_offsets == other.repeat_offsets
    }
}

impl Eq for ParsedDictionary<'_> {}

#[derive(Clone)]
struct DictionaryEntropy {
    huffman_table: huff0_encoder::HuffmanTable,
    fse_tables: FseTables,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum DictionaryParseError {
    WrongDictionary,
    CorruptedDictionary,
}

pub(crate) fn parse_dictionary<'a>(
    dict: &'a [u8],
    content_type: DictionaryContentType,
    no_dict_id: bool,
) -> Result<Option<ParsedDictionary<'a>>, DictionaryParseError> {
    if dict.len() < 8 {
        return match content_type {
            DictionaryContentType::FullDict => Err(DictionaryParseError::WrongDictionary),
            DictionaryContentType::Auto | DictionaryContentType::RawContent => Ok(None),
        };
    }

    if matches!(content_type, DictionaryContentType::RawContent) {
        return Ok(Some(raw_dictionary(dict)));
    }

    if !has_zstd_dictionary_magic(dict) {
        return match content_type {
            DictionaryContentType::Auto => Ok(Some(raw_dictionary(dict))),
            DictionaryContentType::FullDict => Err(DictionaryParseError::WrongDictionary),
            DictionaryContentType::RawContent => unreachable!("raw content returned earlier"),
        };
    }

    parse_full_dictionary(dict, no_dict_id)
}

fn raw_dictionary(dict: &[u8]) -> ParsedDictionary<'_> {
    ParsedDictionary {
        kind: DictionaryKind::RawContent,
        dict_id: 0,
        content: dict,
        repeat_offsets: RepeatOffsets::new(),
        entropy: None,
    }
}

fn parse_full_dictionary(
    dict: &[u8],
    no_dict_id: bool,
) -> Result<Option<ParsedDictionary<'_>>, DictionaryParseError> {
    let decoded =
        Dictionary::decode_dict(dict).map_err(|_| DictionaryParseError::CorruptedDictionary)?;
    let content_len = decoded.dict_content.len();
    let content_start = dict
        .len()
        .checked_sub(content_len)
        .ok_or(DictionaryParseError::CorruptedDictionary)?;
    let repeat_offsets = RepeatOffsets::from_offsets(
        decoded.offset_hist[0],
        decoded.offset_hist[1],
        decoded.offset_hist[2],
    );

    validate_repeat_offsets(repeat_offsets, content_len)?;

    Ok(Some(ParsedDictionary {
        kind: DictionaryKind::Full,
        dict_id: if no_dict_id { 0 } else { decoded.id },
        content: &dict[content_start..],
        repeat_offsets,
        entropy: Some(Box::new(dictionary_entropy(&decoded))),
    }))
}

fn dictionary_entropy(decoded: &Dictionary) -> DictionaryEntropy {
    let mut fse_tables = FseTables::new();
    fse_tables.ll_previous = Some(fse_encoder::build_table_from_probabilities(
        decoded.fse.literal_lengths.symbol_probabilities(),
        decoded.fse.literal_lengths.accuracy_log,
    ));
    fse_tables.ml_previous = Some(fse_encoder::build_table_from_probabilities(
        decoded.fse.match_lengths.symbol_probabilities(),
        decoded.fse.match_lengths.accuracy_log,
    ));
    fse_tables.of_previous = Some(fse_encoder::build_table_from_probabilities(
        decoded.fse.offsets.symbol_probabilities(),
        decoded.fse.offsets.accuracy_log,
    ));

    DictionaryEntropy {
        huffman_table: huff0_encoder::HuffmanTable::build_from_weights(
            &decoded.huf.table.encoder_weights(),
        ),
        fse_tables,
    }
}

fn validate_repeat_offsets(
    repeat_offsets: RepeatOffsets,
    content_len: usize,
) -> Result<(), DictionaryParseError> {
    if repeat_offsets
        .as_offsets()
        .iter()
        .any(|&offset| offset == 0 || offset as usize > content_len)
    {
        return Err(DictionaryParseError::CorruptedDictionary);
    }
    Ok(())
}

fn has_zstd_dictionary_magic(dict: &[u8]) -> bool {
    dict.len() >= MAGIC_NUM.len() && dict[..MAGIC_NUM.len()] == MAGIC_NUM
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use super::*;

    #[test]
    fn short_auto_dictionary_is_ignored_like_c() {
        let parsed = parse_dictionary(b"abcdefg", DictionaryContentType::Auto, false).unwrap();

        assert_eq!(parsed, None);
    }

    #[test]
    fn short_full_dictionary_is_rejected_like_c() {
        let error = parse_dictionary(b"abcdefg", DictionaryContentType::FullDict, false)
            .expect_err("full dictionary requires a full zstd dictionary header");

        assert_eq!(error, DictionaryParseError::WrongDictionary);
    }

    #[test]
    fn auto_dictionary_without_magic_is_raw_content_like_c() {
        let raw = b"raw-dict-content";

        let parsed = parse_dictionary(raw, DictionaryContentType::Auto, false)
            .unwrap()
            .expect("raw dictionary");

        assert_eq!(parsed.kind, DictionaryKind::RawContent);
        assert_eq!(parsed.dict_id, 0);
        assert_eq!(parsed.content, raw);
        assert_eq!(parsed.repeat_offsets, RepeatOffsets::new());
    }

    #[test]
    fn raw_content_mode_ignores_zstd_magic_like_c() {
        let mut raw = MAGIC_NUM.to_vec();
        raw.extend_from_slice(b"raw-content");

        let parsed = parse_dictionary(&raw, DictionaryContentType::RawContent, false)
            .unwrap()
            .expect("raw dictionary");

        assert_eq!(parsed.kind, DictionaryKind::RawContent);
        assert_eq!(parsed.content, raw.as_slice());
    }

    #[test]
    fn full_dictionary_reports_id_content_and_repcodes() {
        let raw = full_dictionary_with_offsets([3, 10, 25]);

        let parsed = parse_dictionary(&raw, DictionaryContentType::Auto, false)
            .unwrap()
            .expect("full dictionary");

        assert_eq!(parsed.kind, DictionaryKind::Full);
        assert_eq!(parsed.dict_id, 0x4723_2101);
        assert_eq!(parsed.content, valid_full_dictionary_content());
        assert_eq!(
            parsed.repeat_offsets,
            RepeatOffsets::from_offsets(3, 10, 25)
        );
    }

    #[test]
    fn full_dictionary_seeds_encoder_entropy_tables_like_c() {
        let raw = full_dictionary_with_offsets([3, 10, 25]);

        let parsed = parse_dictionary(&raw, DictionaryContentType::Auto, false)
            .unwrap()
            .expect("full dictionary");
        let fse_tables = parsed.initial_fse_tables();

        assert!(parsed.initial_huffman_table().is_some());
        assert!(fse_tables.ll_previous.is_some());
        assert!(fse_tables.ml_previous.is_some());
        assert!(fse_tables.of_previous.is_some());
    }

    #[test]
    fn raw_dictionary_does_not_seed_encoder_entropy_tables_like_c() {
        let parsed = parse_dictionary(b"raw-dict-content", DictionaryContentType::Auto, false)
            .unwrap()
            .expect("raw dictionary");
        let fse_tables = parsed.initial_fse_tables();

        assert!(parsed.initial_huffman_table().is_none());
        assert!(fse_tables.ll_previous.is_none());
        assert!(fse_tables.ml_previous.is_none());
        assert!(fse_tables.of_previous.is_none());
    }

    #[test]
    fn full_dictionary_can_hide_dict_id_like_c() {
        let raw = full_dictionary_with_offsets([3, 10, 25]);

        let parsed = parse_dictionary(&raw, DictionaryContentType::Auto, true)
            .unwrap()
            .expect("full dictionary");

        assert_eq!(parsed.dict_id, 0);
    }

    #[test]
    fn full_dictionary_rejects_offsets_past_content_like_c() {
        let raw = full_dictionary_with_offsets([3, 10, 0x00AB_CDEF]);

        let error = parse_dictionary(&raw, DictionaryContentType::Auto, false)
            .expect_err("fixture offsets intentionally exceed dictionary content length");

        assert_eq!(error, DictionaryParseError::CorruptedDictionary);
    }

    fn full_dictionary_with_offsets(offsets: [u32; 3]) -> Vec<u8> {
        let mut raw = Vec::new();
        raw.extend_from_slice(&MAGIC_NUM);
        raw.extend_from_slice(&0x4723_2101_u32.to_le_bytes());
        raw.extend_from_slice(valid_full_dictionary_tables());
        for offset in offsets {
            raw.extend_from_slice(&offset.to_le_bytes());
        }
        raw.extend_from_slice(valid_full_dictionary_content());
        raw
    }

    fn valid_full_dictionary_tables() -> &'static [u8] {
        &[
            54, 16, 192, 155, 4, 0, 207, 59, 239, 121, 158, 116, 220, 93, 114, 229, 110, 41, 249,
            95, 165, 255, 83, 202, 254, 68, 74, 159, 63, 161, 100, 151, 137, 21, 184, 183, 189,
            100, 235, 209, 251, 174, 91, 75, 91, 185, 19, 39, 75, 146, 98, 177, 249, 14, 4, 35, 0,
            0, 0, 40, 40, 20, 10, 12, 204, 37, 196, 1, 173, 122, 0, 4, 0, 128, 1, 2, 2, 25, 32, 27,
            27, 22, 24, 26, 18, 12, 12, 15, 16, 11, 69, 37, 225, 48, 20, 12, 6, 2, 161, 80, 40, 20,
            44, 137, 145, 204, 46, 0, 0, 0, 0, 0, 116, 253, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]
    }

    fn valid_full_dictionary_content() -> &'static [u8] {
        &[
            1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 1, 1, 123, 3, 234, 23, 234, 34, 23, 234, 34, 34, 234,
            234,
        ]
    }
}
