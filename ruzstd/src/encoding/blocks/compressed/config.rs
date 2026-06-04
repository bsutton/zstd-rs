#[cfg(feature = "std")]
use core::convert::TryFrom;
#[cfg(feature = "std")]
use std::sync::OnceLock;

use crate::encoding::{CompressionFileProfile, CompressionFileType, CompressionLevel};

pub(super) const FILE_TYPE_SMALL_SEQUENCE_PREDEFINED_LLML_MAX_SEQUENCES: usize = 64;
pub(super) const FILE_TYPE_SINGLE_STREAM_HUFFMAN_MAX_LITERALS: usize = 1024;

#[derive(Clone, Copy)]
pub(crate) struct BlockCompressionConfig {
    pub(super) huffman_table_search: HuffmanTableSearch,
    pub(super) repeat_table_max_sequences: usize,
    pub(super) offset_table_max_log: u8,
    pub(super) offset_predefined_max_sequences: usize,
    pub(super) exact_sequence_mode_search: bool,
    pub(super) file_type_small_sequence_predefined_llml_max_sequences: Option<usize>,
    pub(super) file_type_single_stream_huffman_max_literals: Option<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum HuffmanTableSearch {
    Heuristic,
    FileTypeSmall,
    AllSections,
}

#[cfg(feature = "std")]
#[derive(Clone, Copy, Debug, Default)]
struct BlockCompressionTuningOverrides {
    huffman_table_search: Option<HuffmanTableSearch>,
    repeat_table_max_sequences: Option<usize>,
    offset_table_max_log: Option<u8>,
    offset_predefined_max_sequences: Option<usize>,
    exact_sequence_mode_search: Option<bool>,
    file_type_small_sequence_predefined_llml_max_sequences: Option<Option<usize>>,
    file_type_single_stream_huffman_max_literals: Option<Option<usize>>,
}

#[cfg(feature = "std")]
static BLOCK_COMPRESSION_TUNING_OVERRIDES: OnceLock<BlockCompressionTuningOverrides> =
    OnceLock::new();

#[cfg(feature = "std")]
fn block_compression_tuning_overrides() -> &'static BlockCompressionTuningOverrides {
    BLOCK_COMPRESSION_TUNING_OVERRIDES.get_or_init(BlockCompressionTuningOverrides::from_env)
}

#[cfg(feature = "std")]
impl BlockCompressionTuningOverrides {
    fn from_env() -> Self {
        Self {
            huffman_table_search: std::env::var("RUZSTD_TUNE_HUFFMAN_TABLE_SEARCH")
                .ok()
                .and_then(|value| match value.as_str() {
                    "heuristic" => Some(HuffmanTableSearch::Heuristic),
                    "filetype" => Some(HuffmanTableSearch::FileTypeSmall),
                    "allsections" => Some(HuffmanTableSearch::AllSections),
                    _ => None,
                }),
            repeat_table_max_sequences: Self::parse_usize("RUZSTD_TUNE_REPEAT_TABLE_MAX_SEQUENCES"),
            offset_table_max_log: Self::parse_usize("RUZSTD_TUNE_OFFSET_TABLE_MAX_LOG")
                .and_then(|value| u8::try_from(value).ok()),
            offset_predefined_max_sequences: Self::parse_usize(
                "RUZSTD_TUNE_OFFSET_PREDEFINED_MAX_SEQUENCES",
            ),
            exact_sequence_mode_search: std::env::var("RUZSTD_TUNE_EXACT_SEQUENCE_MODE_SEARCH")
                .ok()
                .and_then(|value| Self::parse_bool_value(&value))
                .or_else(|| {
                    std::env::var("RUZSTD_TUNE_EXACT_OFFSET_MODE_SEARCH")
                        .ok()
                        .and_then(|value| Self::parse_bool_value(&value))
                }),
            file_type_small_sequence_predefined_llml_max_sequences: Self::parse_option_usize(
                "RUZSTD_TUNE_FILE_TYPE_SMALL_SEQUENCE_PREDEFINED_LLML_MAX_SEQUENCES",
            ),
            file_type_single_stream_huffman_max_literals: Self::parse_option_usize(
                "RUZSTD_TUNE_FILE_TYPE_SINGLE_STREAM_HUFFMAN_MAX_LITERALS",
            ),
        }
    }

    fn parse_usize(name: &str) -> Option<usize> {
        std::env::var(name).ok()?.parse().ok()
    }

    fn parse_option_usize(name: &str) -> Option<Option<usize>> {
        let value = std::env::var(name).ok()?;
        if value == "none" {
            Some(None)
        } else {
            value.parse().ok().map(Some)
        }
    }

    fn parse_bool_value(value: &str) -> Option<bool> {
        match value {
            "1" | "true" | "TRUE" | "yes" | "YES" | "on" | "ON" => Some(true),
            "0" | "false" | "FALSE" | "no" | "NO" | "off" | "OFF" => Some(false),
            _ => None,
        }
    }
}

impl BlockCompressionConfig {
    pub(crate) fn for_level(level: CompressionLevel) -> Self {
        Self::for_level_and_file_type(level, CompressionFileType::Unknown)
    }

    pub(crate) fn for_level_and_file_type(
        level: CompressionLevel,
        file_type: CompressionFileType,
    ) -> Self {
        Self::for_level_and_hints(level, file_type, CompressionFileProfile::None)
    }

    pub(crate) fn for_level_and_hints(
        level: CompressionLevel,
        file_type: CompressionFileType,
        file_profile: CompressionFileProfile,
    ) -> Self {
        let huffman_table_search = match level {
            CompressionLevel::Best => HuffmanTableSearch::Heuristic,
            CompressionLevel::Uncompressed
            | CompressionLevel::Fastest
            | CompressionLevel::Default
            | CompressionLevel::Better => {
                if matches!(file_type, CompressionFileType::DictionaryText) {
                    HuffmanTableSearch::AllSections
                } else if matches!(
                    file_type,
                    CompressionFileType::CodeText
                        | CompressionFileType::ConfigText
                        | CompressionFileType::Unknown
                ) {
                    HuffmanTableSearch::FileTypeSmall
                } else {
                    HuffmanTableSearch::Heuristic
                }
            }
        };
        let repeat_table_max_sequences = match level {
            CompressionLevel::Best => 256,
            CompressionLevel::Uncompressed
            | CompressionLevel::Fastest
            | CompressionLevel::Default
            | CompressionLevel::Better => 64,
        };
        let mut config = Self {
            huffman_table_search,
            repeat_table_max_sequences,
            offset_table_max_log: if matches!(file_type, CompressionFileType::DictionaryText)
                || (matches!(file_type, CompressionFileType::Unknown)
                    && matches!(level, CompressionLevel::Fastest))
            {
                7
            } else {
                8
            },
            offset_predefined_max_sequences: 16,
            exact_sequence_mode_search: matches!(level, CompressionLevel::Fastest)
                && matches!(file_type, CompressionFileType::DictionaryText),
            file_type_small_sequence_predefined_llml_max_sequences: if matches!(
                level,
                CompressionLevel::Fastest
            ) && matches!(
                file_type,
                CompressionFileType::Unknown | CompressionFileType::ConfigText
            ) {
                Some(FILE_TYPE_SMALL_SEQUENCE_PREDEFINED_LLML_MAX_SEQUENCES)
            } else {
                None
            },
            file_type_single_stream_huffman_max_literals: if matches!(
                level,
                CompressionLevel::Fastest
            ) && matches!(
                file_type,
                CompressionFileType::ConfigText
            ) {
                Some(FILE_TYPE_SINGLE_STREAM_HUFFMAN_MAX_LITERALS)
            } else {
                None
            },
        };
        #[cfg(feature = "std")]
        config.apply_tuning_overrides();
        if matches!(file_profile, CompressionFileProfile::SmallTextLockfile) {
            config.apply_small_text_lockfile_tuning();
        } else if matches!(file_profile, CompressionFileProfile::DependencyJsonLockfile) {
            config.apply_dependency_json_lockfile_tuning();
        }
        config
    }

    #[cfg(feature = "std")]
    fn apply_tuning_overrides(&mut self) {
        let overrides = block_compression_tuning_overrides();
        if let Some(value) = overrides.huffman_table_search {
            self.huffman_table_search = value;
        }
        if let Some(value) = overrides.repeat_table_max_sequences {
            self.repeat_table_max_sequences = value;
        }
        if let Some(value) = overrides.offset_table_max_log {
            self.offset_table_max_log = value;
        }
        if let Some(value) = overrides.offset_predefined_max_sequences {
            self.offset_predefined_max_sequences = value;
        }
        if let Some(value) = overrides.exact_sequence_mode_search {
            self.exact_sequence_mode_search = value;
        }
        if let Some(value) = overrides.file_type_small_sequence_predefined_llml_max_sequences {
            self.file_type_small_sequence_predefined_llml_max_sequences = value;
        }
        if let Some(value) = overrides.file_type_single_stream_huffman_max_literals {
            self.file_type_single_stream_huffman_max_literals = value;
        }
    }

    pub(super) fn apply_dependency_json_lockfile_tuning(&mut self) {
        self.huffman_table_search = HuffmanTableSearch::AllSections;
        self.repeat_table_max_sequences = 256;
        self.offset_table_max_log = 8;
        self.exact_sequence_mode_search = true;
    }

    fn apply_small_text_lockfile_tuning(&mut self) {
        self.huffman_table_search = HuffmanTableSearch::AllSections;
        self.repeat_table_max_sequences = 256;
        self.offset_table_max_log = 7;
        self.offset_predefined_max_sequences = 64;
    }
}
