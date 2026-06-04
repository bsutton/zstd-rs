//! Matching algorithm used find repeated parts in the original data
//!
//! The Zstd format relies on finden repeated sequences of data and compressing these sequences as instructions to the decoder.
//! A sequence basically tells the decoder "Go back X bytes and copy Y bytes to the end of your decode buffer".
//!
//! The task here is to efficiently find matches in the already encoded data for the current suffix of the not yet encoded data.

use alloc::vec::Vec;
#[cfg(test)]
use core::cell::{Ref, RefCell};
use core::convert::{TryFrom, TryInto};
use core::num::NonZeroU32;
#[cfg(feature = "std")]
use std::sync::OnceLock;

use super::frame_compressor::OffsetHistory;
use super::util::{likely_composer_lockfile_text, likely_lockfile_text};
use super::CompressionFileProfile;
use super::CompressionFileType;
use super::CompressionLevel;
use super::Matcher;
use super::Sequence;

const MIN_MATCH_LEN: usize = 5;
const SMALL_TEXT_MIN_NON_REPEAT_MATCH_LEN: usize = 5;
const CODE_LIKE_SHORT_TEXT_MIN_NON_REPEAT_MATCH_LEN: usize = 6;
const TEXT_MIN_NON_REPEAT_MATCH_LEN: usize = 8;
const SHORT_TEXT_LINE_LEN_LIMIT: usize = 96;
const SHORT_TEXT_LINE_FRACTION_PERCENT: usize = 95;
const CODE_LIKE_SEMI_PER_100_LINES: usize = 15;
const CODE_LIKE_BRACES_PER_100_LINES: usize = 15;
const SMALL_CODE_TEXT_MIN_NON_REPEAT_MAX_BLOCK_LEN: usize = 16 * 1024;
const CODE_TEXT_DENSE_PROBE_MAX_BLOCK_LEN: usize = 96 * 1024;
const CONFIG_TEXT_DENSE_PROBE_MAX_BLOCK_LEN: usize = 8 * 1024;
const STRUCTURED_JSON_CONFIG_DENSE_PROBE_MAX_BLOCK_LEN: usize = 128 * 1024;
const SHORT_LINE_TEXT_NO_MATCH_PROBE_STEP: usize = 2;
const TSCONFIG_JSON_TEXT_NO_MATCH_PROBE_STEP: usize = 6;
const COMPOSER_JSON_LOCKFILE_NO_MATCH_PROBE_STEP: usize = 5;
const REPEAT_MATCH_LEN_MARGIN: usize = 2;
const DICTIONARY_SMALLER_OFFSET_BITS_GAIN_MIN: usize = 2;
const DICTIONARY_SMALLER_OFFSET_MATCH_LOSS_MAX: usize = 1;
const LARGE_UNKNOWN_SMALLER_OFFSET_BITS_GAIN_MIN: usize = 2;
const LARGE_UNKNOWN_SMALLER_OFFSET_MATCH_LOSS_MAX: usize = 1;
const LARGE_UNKNOWN_NEWEST_DISPLACEMENT_MIN_GAIN: usize = 2;
const LARGE_UNKNOWN_OLDEST_DISPLACEMENT_MIN_GAIN: usize = 2;
const LOCKFILE_OLDEST_DISPLACEMENT_MIN_GAIN: usize = 2;
const LOCKFILE_SAME_END_SMALLER_OFFSET_MATCH_LOSS_MAX: usize = 1;
const LOCKFILE_REPEAT_KIND_MATCH_LOSS_MAX: usize = 1;
const LOCKFILE_NEXT_POSITION_MAX_CURRENT_MATCH_LEN: usize = 7;
const LOCKFILE_NEXT_POSITION_MAX_SKIP_LITERALS: usize = 2;
const LOCKFILE_NEXT_POSITION_MATCH_LOSS_MAX: usize = 0;
const LOCKFILE_NEXT_POSITION_LITERAL_WEIGHT: usize = 6;
const LOCKFILE_NEXT_POSITION_MATCH_REWARD: usize = 2;
const LOCKFILE_NEXT_POSITION_OFFSET_WEIGHT: usize = 3;
const LOCKFILE_NEXT_POSITION_MARGIN: usize = 1;
const COMPOSER_REPEAT_KIND_MATCH_LOSS_MAX: usize = 2;
const REPEAT_SEARCH_EARLY_EXIT_LEN: usize = 10;
const DENSE_MATCH_INDEX_LIMIT: usize = 128;
const NO_MATCH_PROBE_STEP: usize = 2;
const TEXT_NO_MATCH_PROBE_STEP: usize = 3;
const BEST_BINARY_NO_MATCH_SEARCH_STRENGTH: usize = 8;
const BEST_SECOND_NEWEST_RECENT_ENTRY_LIMIT: usize = 1;
const LOCKFILE_SECOND_NEWEST_RECENT_ENTRY_LIMIT: usize = 2;
const BEST_SECOND_NEWEST_MIN_BLOCK_LEN: usize = 16 * 1024;
const CODE_TEXT_SECOND_NEWEST_MAX_BLOCK_LEN: usize = 64 * 1024;
const FASTEST_SECOND_NEWEST_MAX_BLOCK_LEN: usize = 64 * 1024;
const FASTEST_UNKNOWN_SECOND_NEWEST_MAX_BLOCK_LEN: usize = 128 * 1024;
const FASTEST_DENSE_BINARY_PROBE_MAX_BLOCK_LEN: usize = 64 * 1024;
const BEST_CURRENT_LONG_HASH_MIN_BLOCK_LEN: usize = 32 * 1024;
const BEST_CURRENT_LONG_HASH_SKIP_OLDER_LEN: usize = 56;
const BEST_CURRENT_LONG_HASH_DISTANT_NEWEST_ENTRY_START: usize = 2;
const SPARSE_MATCH_END_INDEX_BACKOFF: usize = 2;
const INITIAL_TOUCHED_SLOT_CAPACITY: usize = 1024;
const TOUCHED_SLOT_CLEAR_LIMIT: usize = 32 * 1024;
const SUFFIX_STORE_CAPACITY_DIVISOR: usize = 16;
const BEST_SUFFIX_STORE_CAPACITY_MULTIPLIER: usize = 2;
const FASTEST_WINDOW_BLOCKS: usize = 4;
const BEST_WINDOW_BLOCKS: usize = 16;

#[cfg(feature = "std")]
#[derive(Clone, Copy, Debug, Default)]
struct MatcherTuningOverrides {
    lockfile_probe_step: Option<usize>,
    composer_probe_step: Option<usize>,
    structured_json_probe_step: Option<usize>,
    tsconfig_probe_step: Option<usize>,
    dictionary_same_start_bits_gain_min: Option<usize>,
    dictionary_same_start_match_loss_max: Option<usize>,
    lockfile_same_end_bits_gain_min: Option<usize>,
    lockfile_same_end_match_loss_max: Option<usize>,
    lockfile_repeat_kind_match_loss_max: Option<usize>,
    lockfile_second_newest_zero_literals: Option<bool>,
    lockfile_zero_literal_window_disable: Option<bool>,
    lockfile_zero_literal_window_max_match_len: Option<usize>,
    lockfile_zero_literal_window_min_offset_bits: Option<usize>,
    lockfile_next_position_max_skip_literals: Option<usize>,
    lockfile_next_position_max_current_match_len: Option<usize>,
    lockfile_next_position_match_loss_max: Option<usize>,
    lockfile_next_position_literal_weight: Option<usize>,
    lockfile_next_position_match_reward: Option<usize>,
    lockfile_next_position_offset_weight: Option<usize>,
    lockfile_next_position_margin: Option<usize>,
    composer_window_disable: Option<bool>,
    composer_repeat_kind_match_loss_max: Option<usize>,
    composer_repeat_kind_zero_literals_only: Option<bool>,
    composer_zero_literal_repeat_candidate_limit: Option<usize>,
}

#[cfg(feature = "std")]
static MATCHER_TUNING_OVERRIDES: OnceLock<MatcherTuningOverrides> = OnceLock::new();

#[cfg(feature = "std")]
fn matcher_tuning_overrides() -> &'static MatcherTuningOverrides {
    MATCHER_TUNING_OVERRIDES.get_or_init(MatcherTuningOverrides::from_env)
}

#[cfg(feature = "std")]
impl MatcherTuningOverrides {
    fn from_env() -> Self {
        Self {
            lockfile_probe_step: Self::parse_usize("RUZSTD_TUNE_LOCKFILE_PROBE_STEP"),
            composer_probe_step: Self::parse_usize("RUZSTD_TUNE_COMPOSER_PROBE_STEP"),
            structured_json_probe_step: Self::parse_usize("RUZSTD_TUNE_STRUCTURED_JSON_PROBE_STEP"),
            tsconfig_probe_step: Self::parse_usize("RUZSTD_TUNE_TSCONFIG_PROBE_STEP"),
            dictionary_same_start_bits_gain_min: Self::parse_usize(
                "RUZSTD_TUNE_DICTIONARY_SAME_START_BITS_GAIN_MIN",
            ),
            dictionary_same_start_match_loss_max: Self::parse_usize(
                "RUZSTD_TUNE_DICTIONARY_SAME_START_MATCH_LOSS_MAX",
            ),
            lockfile_same_end_bits_gain_min: Self::parse_usize(
                "RUZSTD_TUNE_LOCKFILE_SAME_END_BITS_GAIN_MIN",
            ),
            lockfile_same_end_match_loss_max: Self::parse_usize(
                "RUZSTD_TUNE_LOCKFILE_SAME_END_MATCH_LOSS_MAX",
            ),
            lockfile_repeat_kind_match_loss_max: Self::parse_usize(
                "RUZSTD_TUNE_LOCKFILE_REPEAT_KIND_MATCH_LOSS_MAX",
            ),
            lockfile_second_newest_zero_literals: Self::parse_bool(
                "RUZSTD_TUNE_LOCKFILE_SECOND_NEWEST_ZERO_LITERALS",
            ),
            lockfile_zero_literal_window_disable: Self::parse_bool(
                "RUZSTD_TUNE_LOCKFILE_ZERO_LITERAL_WINDOW_DISABLE",
            ),
            lockfile_zero_literal_window_max_match_len: Self::parse_usize(
                "RUZSTD_TUNE_LOCKFILE_ZERO_LITERAL_WINDOW_MAX_MATCH_LEN",
            ),
            lockfile_zero_literal_window_min_offset_bits: Self::parse_usize(
                "RUZSTD_TUNE_LOCKFILE_ZERO_LITERAL_WINDOW_MIN_OFFSET_BITS",
            ),
            lockfile_next_position_max_skip_literals: Self::parse_usize(
                "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_SKIP_LITERALS",
            ),
            lockfile_next_position_max_current_match_len: Self::parse_usize(
                "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_CURRENT_MATCH_LEN",
            ),
            lockfile_next_position_match_loss_max: Self::parse_usize(
                "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MATCH_LOSS_MAX",
            ),
            lockfile_next_position_literal_weight: Self::parse_usize(
                "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_LITERAL_WEIGHT",
            ),
            lockfile_next_position_match_reward: Self::parse_usize(
                "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MATCH_REWARD",
            ),
            lockfile_next_position_offset_weight: Self::parse_usize(
                "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_OFFSET_WEIGHT",
            ),
            lockfile_next_position_margin: Self::parse_usize(
                "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MARGIN",
            ),
            composer_window_disable: Self::parse_bool("RUZSTD_TUNE_COMPOSER_WINDOW_DISABLE"),
            composer_repeat_kind_match_loss_max: Self::parse_usize(
                "RUZSTD_TUNE_COMPOSER_REPEAT_KIND_MATCH_LOSS_MAX",
            ),
            composer_repeat_kind_zero_literals_only: Self::parse_bool(
                "RUZSTD_TUNE_COMPOSER_REPEAT_KIND_ZERO_LITERALS_ONLY",
            ),
            composer_zero_literal_repeat_candidate_limit: Self::parse_usize(
                "RUZSTD_TUNE_COMPOSER_ZERO_LITERAL_REPEAT_CANDIDATE_LIMIT",
            ),
        }
    }

    fn parse_usize(name: &str) -> Option<usize> {
        std::env::var(name).ok()?.parse().ok()
    }

    fn parse_bool(name: &str) -> Option<bool> {
        match std::env::var(name).ok()?.as_str() {
            "1" | "true" | "TRUE" | "yes" | "YES" | "on" | "ON" => Some(true),
            "0" | "false" | "FALSE" | "no" | "NO" | "off" | "OFF" => Some(false),
            _ => None,
        }
    }
}

/// This is the default implementation of the `Matcher` trait. It allocates and reuses the buffers when possible.
pub struct MatchGeneratorDriver {
    vec_pool: Vec<Vec<u8>>,
    suffix_pool: Vec<SuffixStore>,
    match_generator: MatchGenerator,
    slice_size: usize,
    suffix_store_capacity: usize,
    adaptive_binary_no_match_probe: bool,
    use_fast_small_dense_binary_probe: bool,
    prefer_binary_next_position_repeat_lookahead: bool,
    prefer_fast_binary_next_position_repeat_lookahead: bool,
    prefer_binary_next_position_lookahead: bool,
    prefer_oldest_first_window_probe: bool,
    use_complementary_end_insertion: bool,
    use_second_newest_probe: bool,
    use_fast_binary_small_second_newest: bool,
    use_text_repeat_pipeline: bool,
    file_type_hint: CompressionFileType,
    file_profile_hint: CompressionFileProfile,
}

impl MatchGeneratorDriver {
    /// slice_size says how big the slices should be that are allocated to work with
    /// max_slices_in_window says how many slices should at most be used while looking for matches
    pub(crate) fn new(slice_size: usize, max_slices_in_window: usize) -> Self {
        Self {
            vec_pool: Vec::new(),
            suffix_pool: Vec::new(),
            match_generator: MatchGenerator::new(max_slices_in_window * slice_size),
            slice_size,
            suffix_store_capacity: slice_size / SUFFIX_STORE_CAPACITY_DIVISOR,
            adaptive_binary_no_match_probe: false,
            use_fast_small_dense_binary_probe: false,
            prefer_binary_next_position_repeat_lookahead: false,
            prefer_fast_binary_next_position_repeat_lookahead: false,
            prefer_binary_next_position_lookahead: false,
            prefer_oldest_first_window_probe: false,
            use_complementary_end_insertion: false,
            use_second_newest_probe: false,
            use_fast_binary_small_second_newest: false,
            use_text_repeat_pipeline: false,
            file_type_hint: CompressionFileType::Unknown,
            file_profile_hint: CompressionFileProfile::None,
        }
    }

    #[cfg(test)]
    pub(crate) fn repeat_offsets(&self) -> (u32, u32, u32) {
        self.match_generator.offset_history.as_offsets()
    }

    #[cfg(test)]
    pub(crate) fn diagnostics(&self) -> Ref<'_, MatcherDiagnostics> {
        self.match_generator.diagnostics.borrow()
    }
}

impl Matcher for MatchGeneratorDriver {
    fn set_file_type_hint(&mut self, file_type: CompressionFileType) {
        self.file_type_hint = file_type;
        self.match_generator.file_type_hint = file_type;
    }

    fn set_internal_file_profile_hint(&mut self, file_profile_code: u8) {
        let file_profile = CompressionFileProfile::from_internal_hint_code(file_profile_code);
        self.file_profile_hint = file_profile;
        self.match_generator.file_profile_hint = file_profile;
    }

    fn reset(&mut self, level: CompressionLevel) {
        let vec_pool = &mut self.vec_pool;
        let suffix_pool = &mut self.suffix_pool;
        let fast_window_size = self.slice_size * FASTEST_WINDOW_BLOCKS;
        self.suffix_store_capacity = Self::suffix_store_capacity(self.slice_size, level);
        self.adaptive_binary_no_match_probe = Self::adaptive_binary_no_match_probe(level);
        self.use_fast_small_dense_binary_probe = Self::use_fast_small_dense_binary_probe(level);
        self.prefer_binary_next_position_repeat_lookahead =
            Self::prefer_binary_next_position_repeat_lookahead(level);
        self.prefer_fast_binary_next_position_repeat_lookahead =
            Self::prefer_fast_binary_next_position_repeat_lookahead(level);
        self.prefer_binary_next_position_lookahead =
            Self::prefer_binary_next_position_lookahead(level);
        self.prefer_oldest_first_window_probe = Self::prefer_oldest_first_window_probe(level);
        self.use_complementary_end_insertion = Self::use_complementary_end_insertion(level);
        self.use_second_newest_probe = Self::use_second_newest_probe(level);
        self.use_fast_binary_small_second_newest = Self::use_fast_binary_small_second_newest(level);
        self.use_text_repeat_pipeline = Self::use_text_repeat_pipeline(level);

        self.match_generator.reset(|mut data, mut suffixes| {
            data.resize(data.capacity(), 0);
            vec_pool.push(data);
            suffixes.clear();
            suffix_pool.push(suffixes);
        });
        self.match_generator.set_window_sizes(
            self.slice_size * Self::window_blocks(level),
            fast_window_size,
        );
        self.match_generator.adaptive_binary_no_match_probe = self.adaptive_binary_no_match_probe;
        self.match_generator.use_fast_small_dense_binary_probe =
            self.use_fast_small_dense_binary_probe;
        self.match_generator
            .prefer_binary_next_position_repeat_lookahead =
            self.prefer_binary_next_position_repeat_lookahead;
        self.match_generator
            .prefer_fast_binary_next_position_repeat_lookahead =
            self.prefer_fast_binary_next_position_repeat_lookahead;
        self.match_generator.prefer_binary_next_position_lookahead =
            self.prefer_binary_next_position_lookahead;
        self.match_generator.prefer_oldest_first_window_probe =
            self.prefer_oldest_first_window_probe;
        self.match_generator.use_complementary_end_insertion = self.use_complementary_end_insertion;
        self.match_generator.use_second_newest_probe = self.use_second_newest_probe;
        self.match_generator.use_fast_binary_small_second_newest =
            self.use_fast_binary_small_second_newest;
        self.match_generator.use_text_repeat_pipeline = self.use_text_repeat_pipeline;
        self.match_generator.file_type_hint = self.file_type_hint;
        self.match_generator.file_profile_hint = self.file_profile_hint;
    }

    fn window_size(&self) -> u64 {
        self.match_generator.max_window_size as u64
    }

    fn get_next_space(&mut self) -> Vec<u8> {
        match self.vec_pool.pop() {
            Some(space) => space,
            None => {
                let mut space = alloc::vec![0; self.slice_size];
                space.resize(space.capacity(), 0);
                space
            }
        }
    }

    fn get_last_space(&self) -> &[u8] {
        self.match_generator.last_entry().data.as_slice()
    }

    fn commit_space(&mut self, space: Vec<u8>) {
        let vec_pool = &mut self.vec_pool;
        let suffix_capacity = self.suffix_store_capacity;
        let suffixes = match self.suffix_pool.pop() {
            Some(suffixes)
                if suffixes.capacity() == SuffixStore::normalized_capacity(suffix_capacity) =>
            {
                suffixes
            }
            _ => SuffixStore::with_capacity(suffix_capacity),
        };
        let suffix_pool = &mut self.suffix_pool;
        self.match_generator
            .add_data(space, suffixes, |mut data, mut suffixes| {
                data.resize(data.capacity(), 0);
                vec_pool.push(data);
                suffixes.clear();
                suffix_pool.push(suffixes);
            });
    }

    fn start_matching(&mut self, mut handle_sequence: impl for<'a> FnMut(Sequence<'a>)) {
        while self.match_generator.next_sequence(&mut handle_sequence) {}
    }

    fn set_repeat_offsets(&mut self, newest: u32, second: u32, third: u32) {
        self.match_generator.offset_history = OffsetHistory::from_offsets(newest, second, third);
    }

    fn skip_matching(&mut self) {
        self.match_generator.skip_matching();
    }

    fn skip_matching_for_incompressible(&mut self) {
        self.match_generator.skip_matching_for_incompressible();
    }

    fn skip_matching_for_rle(&mut self) {
        self.match_generator.skip_matching_for_rle();
    }
}

impl MatchGeneratorDriver {
    fn window_blocks(level: CompressionLevel) -> usize {
        match level {
            CompressionLevel::Best => BEST_WINDOW_BLOCKS,
            CompressionLevel::Uncompressed
            | CompressionLevel::Fastest
            | CompressionLevel::Default
            | CompressionLevel::Better => FASTEST_WINDOW_BLOCKS,
        }
    }

    fn suffix_store_capacity(slice_size: usize, level: CompressionLevel) -> usize {
        match level {
            CompressionLevel::Best => slice_size * BEST_SUFFIX_STORE_CAPACITY_MULTIPLIER,
            CompressionLevel::Uncompressed
            | CompressionLevel::Fastest
            | CompressionLevel::Default
            | CompressionLevel::Better => slice_size / SUFFIX_STORE_CAPACITY_DIVISOR,
        }
    }

    fn adaptive_binary_no_match_probe(level: CompressionLevel) -> bool {
        matches!(level, CompressionLevel::Best)
    }

    fn use_fast_small_dense_binary_probe(level: CompressionLevel) -> bool {
        matches!(level, CompressionLevel::Fastest)
    }

    fn prefer_binary_next_position_repeat_lookahead(level: CompressionLevel) -> bool {
        matches!(level, CompressionLevel::Best)
    }

    fn prefer_fast_binary_next_position_repeat_lookahead(level: CompressionLevel) -> bool {
        matches!(level, CompressionLevel::Fastest)
    }

    fn prefer_binary_next_position_lookahead(level: CompressionLevel) -> bool {
        matches!(level, CompressionLevel::Best)
    }

    fn prefer_oldest_first_window_probe(level: CompressionLevel) -> bool {
        matches!(level, CompressionLevel::Best)
    }

    fn use_complementary_end_insertion(level: CompressionLevel) -> bool {
        matches!(level, CompressionLevel::Best)
    }

    fn use_second_newest_probe(level: CompressionLevel) -> bool {
        matches!(level, CompressionLevel::Best)
    }

    fn use_fast_binary_small_second_newest(level: CompressionLevel) -> bool {
        matches!(level, CompressionLevel::Fastest)
    }

    fn use_text_repeat_pipeline(level: CompressionLevel) -> bool {
        matches!(level, CompressionLevel::Best)
    }
}

/// This stores the index of a suffix of a string by hashing the first few bytes of that suffix
/// This means that collisions just overwrite and that you need to check validity after a get
struct SuffixStore {
    slots: Vec<Option<Candidates>>,
    touched_slots: Vec<u32>,
    clear_all_slots: bool,
    len_log: u32,
}

#[derive(Copy, Clone)]
struct Candidates {
    // We need 17 bits per index to store the maximum block size of 128kb.
    // We store indexes using one-based so Option can use a NonZeroU32 niche.
    oldest: NonZeroU32,
    newest: NonZeroU32,
}

struct CandidateIndexes {
    oldest: usize,
    newest: Option<usize>,
}

impl SuffixStore {
    fn with_capacity(capacity: usize) -> Self {
        let capacity = Self::normalized_capacity(capacity);
        Self {
            slots: alloc::vec![None; capacity],
            touched_slots: Vec::with_capacity(INITIAL_TOUCHED_SLOT_CAPACITY),
            clear_all_slots: false,
            len_log: capacity.ilog2(),
        }
    }

    fn normalized_capacity(capacity: usize) -> usize {
        capacity.max(2)
    }

    fn capacity(&self) -> usize {
        self.slots.len()
    }

    #[inline(always)]
    fn insert(&mut self, suffix: &[u8], idx: usize) {
        let key = self.slot_key(Self::key_value(suffix)).index;
        let idx = Self::stored_index(idx);
        if let Some(slot) = self.slots[key] {
            self.slots[key] = Some(Candidates {
                oldest: slot.oldest,
                newest: idx,
            });
        } else {
            self.record_touched_slot(key);
            self.slots[key] = Some(Candidates {
                oldest: idx,
                newest: idx,
            });
        }
    }

    fn clear(&mut self) {
        if self.clear_all_slots {
            self.slots.fill(None);
            self.touched_slots.clear();
            self.clear_all_slots = false;
            return;
        }

        for key in self.touched_slots.drain(..) {
            self.slots[key as usize] = None;
        }
    }

    #[inline(always)]
    fn record_touched_slot(&mut self, key: usize) {
        if self.clear_all_slots {
            return;
        }
        if self.touched_slots.len() == TOUCHED_SLOT_CLEAR_LIMIT {
            self.touched_slots.clear();
            self.clear_all_slots = true;
            return;
        }
        self.touched_slots.push(Self::stored_slot_key(key));
    }

    #[inline(always)]
    fn stored_slot_key(key: usize) -> u32 {
        match u32::try_from(key) {
            Ok(key) => key,
            Err(_) => Self::invalid_stored_index(),
        }
    }

    #[inline(always)]
    fn stored_index(idx: usize) -> NonZeroU32 {
        let Some(idx) = idx.checked_add(1) else {
            Self::invalid_stored_index()
        };
        let Ok(idx) = u32::try_from(idx) else {
            Self::invalid_stored_index()
        };
        let Some(idx) = NonZeroU32::new(idx) else {
            Self::invalid_stored_index()
        };
        idx
    }

    #[cold]
    #[inline(never)]
    fn invalid_stored_index() -> ! {
        panic!("suffix index must fit in non-zero u32")
    }

    #[cfg(test)]
    #[inline(always)]
    fn candidates(&self, suffix: &[u8]) -> Option<CandidateIndexes> {
        self.candidates_for_key_value(Self::key_value(suffix))
    }

    #[inline(always)]
    fn candidates_for_key_value(&self, value: u64) -> Option<CandidateIndexes> {
        self.candidates_for_slot_key(self.slot_key(value))
    }

    #[inline(always)]
    fn candidates_for_slot_key(&self, slot_key: SlotKey) -> Option<CandidateIndexes> {
        let key = slot_key.index;
        let slot = self.slots[key]?;
        let oldest = slot.oldest.get() as usize - 1;
        let newest = slot.newest.get() as usize - 1;
        let newest = if oldest == newest { None } else { Some(newest) };
        Some(CandidateIndexes { oldest, newest })
    }

    #[cfg(test)]
    #[inline(always)]
    fn key(&self, suffix: &[u8]) -> usize {
        self.slot_key(Self::key_value(suffix)).index
    }

    #[inline(always)]
    fn key_value(suffix: &[u8]) -> u64 {
        u64::from(suffix[0])
            | (u64::from(suffix[1]) << 8)
            | (u64::from(suffix[2]) << 16)
            | (u64::from(suffix[3]) << 24)
            | (u64::from(suffix[4]) << 32)
    }

    #[cfg(test)]
    #[inline(always)]
    fn key_from_value(&self, value: u64) -> usize {
        self.slot_key(value).index
    }

    #[inline(always)]
    fn slot_key(&self, value: u64) -> SlotKey {
        let hash = value.wrapping_mul(0x9E37_79B1_85EB_CA87);
        let index = (hash >> (64 - self.len_log)) as usize;
        SlotKey { index }
    }
}

#[derive(Copy, Clone)]
struct SlotKey {
    index: usize,
}

/// We keep a window of a few of these entries
/// All of these are valid targets for a match to be generated for
struct WindowEntry {
    data: Vec<u8>,
    /// Stores indexes into data
    suffixes: SuffixStore,
    /// Makes offset calculations efficient
    base_offset: usize,
}

struct MatchCandidateContext<'data> {
    suffix_idx: usize,
    anchor_idx: usize,
    min_non_repeat_match_len: usize,
    data_slice: &'data [u8],
    #[cfg(debug_assertions)]
    last_entry_len: usize,
    #[cfg(debug_assertions)]
    concat_window: &'data [u8],
}

#[derive(Clone, Copy)]
enum WindowCandidateKind {
    Oldest,
    Newest,
    SecondNewest,
}

#[derive(Clone, Copy)]
#[cfg_attr(not(test), allow(dead_code))]
struct WindowCandidateMeta {
    entry_distance: usize,
    kind: WindowCandidateKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RepeatCandidateKind {
    First,
    Second,
    Third,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CandidateSource {
    RepeatCurrent(RepeatCandidateKind),
    RepeatNextPosition(RepeatCandidateKind),
    WindowCurrentLongHash,
    WindowCurrentNewest { entry_distance: usize },
    WindowCurrentSecondNewest { entry_distance: usize },
    WindowCurrentOldest { entry_distance: usize },
    WindowNextPositionNewest { entry_distance: usize },
    WindowNextPositionSecondNewest { entry_distance: usize },
    WindowNextPositionOldest { entry_distance: usize },
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RepeatNextPositionSelectionReason {
    NoCurrentCandidate,
    BeatsCurrentMinNonRepeat,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct MatchCandidate {
    start_idx: usize,
    offset: usize,
    match_len: usize,
    repeat_offset: bool,
    #[cfg(test)]
    source: CandidateSource,
}

impl MatchCandidate {
    #[cfg_attr(not(test), allow(dead_code))]
    fn is_better_than(self, other: Self) -> bool {
        if self.repeat_offset != other.repeat_offset {
            if self.repeat_offset {
                return self.match_len + REPEAT_MATCH_LEN_MARGIN >= other.match_len;
            }
            return self.match_len > other.match_len + REPEAT_MATCH_LEN_MARGIN;
        }

        self.match_len > other.match_len
            || (self.match_len == other.match_len
                && (self.start_idx < other.start_idx
                    || self.start_idx == other.start_idx && self.offset < other.offset))
    }

    fn worth_emitting(self, min_non_repeat_match_len: usize) -> bool {
        self.repeat_offset || self.match_len >= min_non_repeat_match_len
    }

    #[cfg(test)]
    fn can_skip_window_search(self, block_len: usize) -> bool {
        self.repeat_offset
            && (self.start_idx + self.match_len == block_len
                || self.match_len >= REPEAT_SEARCH_EARLY_EXIT_LEN)
    }

    #[cfg(test)]
    fn source_repeat_kind(self) -> RepeatCandidateKind {
        match self.source {
            CandidateSource::RepeatCurrent(kind) | CandidateSource::RepeatNextPosition(kind) => {
                kind
            }
            other => panic!("expected repeat candidate source, got {:?}", other),
        }
    }
}

#[cfg(test)]
#[derive(Clone, Debug, Default)]
pub(crate) struct MatcherDiagnostics {
    pub(crate) total_sequences: usize,
    pub(crate) repeat_current: [usize; 3],
    pub(crate) repeat_current_zero_literals: [usize; 3],
    pub(crate) repeat_current_with_literals: [usize; 3],
    pub(crate) repeat_best_before_window: [usize; 3],
    pub(crate) repeat_best_before_window_zero_literals: [usize; 3],
    pub(crate) repeat_best_before_window_with_literals: [usize; 3],
    pub(crate) repeat_best_before_window_overridden_by_window: [usize; 3],
    pub(crate) repeat_next_position: [usize; 3],
    pub(crate) repeat_next_position_zero_literals: [usize; 3],
    pub(crate) repeat_next_position_with_literals: [usize; 3],
    pub(crate) repeat_next_position_selected_without_current_candidate: [usize; 3],
    pub(crate) repeat_next_position_selected_over_current_min_non_repeat: [usize; 3],
    pub(crate) current_long_hash_found: usize,
    pub(crate) current_long_hash_overridden: usize,
    pub(crate) current_long_hash_improved_by_newest: [usize; BEST_WINDOW_BLOCKS],
    pub(crate) current_long_hash_improved_by_second_newest: [usize; BEST_WINDOW_BLOCKS],
    pub(crate) current_long_hash_improved_by_oldest: [usize; BEST_WINDOW_BLOCKS],
    pub(crate) current_long_hash_end_break_by_newest: [usize; BEST_WINDOW_BLOCKS],
    pub(crate) current_long_hash_end_break_by_second_newest: [usize; BEST_WINDOW_BLOCKS],
    pub(crate) current_long_hash_end_break_by_oldest: [usize; BEST_WINDOW_BLOCKS],
    pub(crate) current_long_hash_end_break_without_improvement_by_newest:
        [usize; BEST_WINDOW_BLOCKS],
    pub(crate) current_long_hash_end_break_without_improvement_by_second_newest:
        [usize; BEST_WINDOW_BLOCKS],
    pub(crate) current_long_hash_end_break_without_improvement_by_oldest:
        [usize; BEST_WINDOW_BLOCKS],
    pub(crate) current_long_hash_overridden_by_newest: [usize; BEST_WINDOW_BLOCKS],
    pub(crate) current_long_hash_overridden_by_second_newest: [usize; BEST_WINDOW_BLOCKS],
    pub(crate) current_long_hash_overridden_by_oldest: [usize; BEST_WINDOW_BLOCKS],
    pub(crate) window_current_long_hash: usize,
    pub(crate) window_current_long_hash_zero_literals: usize,
    pub(crate) window_current_long_hash_with_literals: usize,
    pub(crate) window_current_newest: [usize; BEST_WINDOW_BLOCKS],
    pub(crate) window_current_second_newest: [usize; BEST_WINDOW_BLOCKS],
    pub(crate) window_current_second_newest_zero_literals: [usize; BEST_WINDOW_BLOCKS],
    pub(crate) window_current_second_newest_with_literals: [usize; BEST_WINDOW_BLOCKS],
    pub(crate) window_current_oldest: [usize; BEST_WINDOW_BLOCKS],
    pub(crate) window_next_position_newest: [usize; BEST_WINDOW_BLOCKS],
    pub(crate) window_next_position_second_newest: [usize; BEST_WINDOW_BLOCKS],
    pub(crate) window_next_position_oldest: [usize; BEST_WINDOW_BLOCKS],
}

#[cfg(test)]
impl MatcherDiagnostics {
    fn repeat_kind_index(kind: RepeatCandidateKind) -> usize {
        match kind {
            RepeatCandidateKind::First => 0,
            RepeatCandidateKind::Second => 1,
            RepeatCandidateKind::Third => 2,
        }
    }

    fn increment_distance_bucket(buckets: &mut [usize; BEST_WINDOW_BLOCKS], entry_distance: usize) {
        if let Some(bucket) = buckets.get_mut(entry_distance) {
            *bucket += 1;
        }
    }

    fn record_current_long_hash_outcome(
        &mut self,
        found: MatchCandidate,
        selected: Option<MatchCandidate>,
    ) {
        debug_assert!(matches!(
            found.source,
            CandidateSource::WindowCurrentLongHash
        ));
        self.current_long_hash_found += 1;
        let Some(selected) = selected else {
            return;
        };
        if matches!(selected.source, CandidateSource::WindowCurrentLongHash) {
            return;
        }
        self.current_long_hash_overridden += 1;
        match selected.source {
            CandidateSource::WindowCurrentNewest { entry_distance } => {
                Self::increment_distance_bucket(
                    &mut self.current_long_hash_overridden_by_newest,
                    entry_distance,
                );
            }
            CandidateSource::WindowCurrentSecondNewest { entry_distance } => {
                Self::increment_distance_bucket(
                    &mut self.current_long_hash_overridden_by_second_newest,
                    entry_distance,
                );
            }
            CandidateSource::WindowCurrentOldest { entry_distance } => {
                Self::increment_distance_bucket(
                    &mut self.current_long_hash_overridden_by_oldest,
                    entry_distance,
                );
            }
            _ => {}
        }
    }

    fn record_repeat_next_position_selection(
        &mut self,
        kind: RepeatCandidateKind,
        reason: RepeatNextPositionSelectionReason,
    ) {
        let index = Self::repeat_kind_index(kind);
        match reason {
            RepeatNextPositionSelectionReason::NoCurrentCandidate => {
                self.repeat_next_position_selected_without_current_candidate[index] += 1;
            }
            RepeatNextPositionSelectionReason::BeatsCurrentMinNonRepeat => {
                self.repeat_next_position_selected_over_current_min_non_repeat[index] += 1;
            }
        }
    }

    fn record_repeat_best_before_window(
        &mut self,
        kind: RepeatCandidateKind,
        literals_empty: bool,
    ) {
        let index = Self::repeat_kind_index(kind);
        self.repeat_best_before_window[index] += 1;
        if literals_empty {
            self.repeat_best_before_window_zero_literals[index] += 1;
        } else {
            self.repeat_best_before_window_with_literals[index] += 1;
        }
    }

    fn record_repeat_best_before_window_overridden_by_window(&mut self, kind: RepeatCandidateKind) {
        let index = Self::repeat_kind_index(kind);
        self.repeat_best_before_window_overridden_by_window[index] += 1;
    }

    fn record_current_long_hash_improvement(&mut self, selected: MatchCandidate) {
        match selected.source {
            CandidateSource::WindowCurrentNewest { entry_distance } => {
                Self::increment_distance_bucket(
                    &mut self.current_long_hash_improved_by_newest,
                    entry_distance,
                );
            }
            CandidateSource::WindowCurrentSecondNewest { entry_distance } => {
                Self::increment_distance_bucket(
                    &mut self.current_long_hash_improved_by_second_newest,
                    entry_distance,
                );
            }
            CandidateSource::WindowCurrentOldest { entry_distance } => {
                Self::increment_distance_bucket(
                    &mut self.current_long_hash_improved_by_oldest,
                    entry_distance,
                );
            }
            _ => {}
        }
    }

    fn record_current_long_hash_end_break(&mut self, selected: MatchCandidate, improved: bool) {
        let target = match (selected.source, improved) {
            (CandidateSource::WindowCurrentNewest { .. }, true) => {
                &mut self.current_long_hash_end_break_by_newest
            }
            (CandidateSource::WindowCurrentSecondNewest { .. }, true) => {
                &mut self.current_long_hash_end_break_by_second_newest
            }
            (CandidateSource::WindowCurrentOldest { .. }, true) => {
                &mut self.current_long_hash_end_break_by_oldest
            }
            (CandidateSource::WindowCurrentNewest { .. }, false) => {
                &mut self.current_long_hash_end_break_without_improvement_by_newest
            }
            (CandidateSource::WindowCurrentSecondNewest { .. }, false) => {
                &mut self.current_long_hash_end_break_without_improvement_by_second_newest
            }
            (CandidateSource::WindowCurrentOldest { .. }, false) => {
                &mut self.current_long_hash_end_break_without_improvement_by_oldest
            }
            _ => return,
        };

        let entry_distance = match selected.source {
            CandidateSource::WindowCurrentNewest { entry_distance }
            | CandidateSource::WindowCurrentSecondNewest { entry_distance }
            | CandidateSource::WindowCurrentOldest { entry_distance } => entry_distance,
            _ => return,
        };
        Self::increment_distance_bucket(target, entry_distance);
    }

    fn record(&mut self, candidate: MatchCandidate, literals_empty: bool) {
        self.total_sequences += 1;
        match candidate.source {
            CandidateSource::RepeatCurrent(kind) => {
                let index = Self::repeat_kind_index(kind);
                self.repeat_current[index] += 1;
                if literals_empty {
                    self.repeat_current_zero_literals[index] += 1;
                } else {
                    self.repeat_current_with_literals[index] += 1;
                }
            }
            CandidateSource::RepeatNextPosition(kind) => {
                let index = Self::repeat_kind_index(kind);
                self.repeat_next_position[index] += 1;
                if literals_empty {
                    self.repeat_next_position_zero_literals[index] += 1;
                } else {
                    self.repeat_next_position_with_literals[index] += 1;
                }
            }
            CandidateSource::WindowCurrentLongHash => {
                self.window_current_long_hash += 1;
                if literals_empty {
                    self.window_current_long_hash_zero_literals += 1;
                } else {
                    self.window_current_long_hash_with_literals += 1;
                }
            }
            CandidateSource::WindowCurrentNewest { entry_distance } => {
                Self::increment_distance_bucket(&mut self.window_current_newest, entry_distance);
            }
            CandidateSource::WindowCurrentSecondNewest { entry_distance } => {
                Self::increment_distance_bucket(
                    &mut self.window_current_second_newest,
                    entry_distance,
                );
                if literals_empty {
                    Self::increment_distance_bucket(
                        &mut self.window_current_second_newest_zero_literals,
                        entry_distance,
                    );
                } else {
                    Self::increment_distance_bucket(
                        &mut self.window_current_second_newest_with_literals,
                        entry_distance,
                    );
                }
            }
            CandidateSource::WindowCurrentOldest { entry_distance } => {
                Self::increment_distance_bucket(&mut self.window_current_oldest, entry_distance);
            }
            CandidateSource::WindowNextPositionNewest { entry_distance } => {
                Self::increment_distance_bucket(
                    &mut self.window_next_position_newest,
                    entry_distance,
                );
            }
            CandidateSource::WindowNextPositionSecondNewest { entry_distance } => {
                Self::increment_distance_bucket(
                    &mut self.window_next_position_second_newest,
                    entry_distance,
                );
            }
            CandidateSource::WindowNextPositionOldest { entry_distance } => {
                Self::increment_distance_bucket(
                    &mut self.window_next_position_oldest,
                    entry_distance,
                );
            }
        }
    }
}

pub(crate) struct MatchGenerator {
    max_window_size: usize,
    fast_window_size: usize,
    /// Data window we are operating on to find matches
    /// The data we want to find matches for is in the last slice
    window: Vec<WindowEntry>,
    current_second_newest_sidecar: Vec<Option<NonZeroU32>>,
    current_third_newest_sidecar: Vec<Option<NonZeroU32>>,
    current_fourth_newest_sidecar: Vec<Option<NonZeroU32>>,
    current_fifth_newest_sidecar: Vec<Option<NonZeroU32>>,
    current_sixth_newest_sidecar: Vec<Option<NonZeroU32>>,
    current_seventh_newest_sidecar: Vec<Option<NonZeroU32>>,
    current_eighth_newest_sidecar: Vec<Option<NonZeroU32>>,
    current_ninth_newest_sidecar: Vec<Option<NonZeroU32>>,
    current_tenth_newest_sidecar: Vec<Option<NonZeroU32>>,
    current_eleventh_newest_sidecar: Vec<Option<NonZeroU32>>,
    current_twelfth_newest_sidecar: Vec<Option<NonZeroU32>>,
    current_thirteenth_newest_sidecar: Vec<Option<NonZeroU32>>,
    current_long_hash: Vec<Option<NonZeroU32>>,
    window_size: usize,
    #[cfg(debug_assertions)]
    concat_window: Vec<u8>,
    uniform_suffix_len_log: Option<u32>,
    /// Index in the last slice that we already processed
    suffix_idx: usize,
    /// Gets updated when a new sequence is returned to point right behind that sequence
    last_idx_in_sequence: usize,
    offset_history: OffsetHistory,
    is_text_block: bool,
    is_short_line_text: bool,
    min_non_repeat_match_len: usize,
    adaptive_binary_no_match_probe: bool,
    use_fast_small_dense_binary_probe: bool,
    prefer_binary_next_position_repeat_lookahead: bool,
    prefer_fast_binary_next_position_repeat_lookahead: bool,
    prefer_binary_next_position_lookahead: bool,
    prefer_oldest_first_window_probe: bool,
    use_complementary_end_insertion: bool,
    use_second_newest_probe: bool,
    use_fast_binary_small_second_newest: bool,
    use_text_repeat_pipeline: bool,
    current_block_is_dictionary_lockfile_text: bool,
    current_block_is_composer_dictionary_text: bool,
    current_block_is_structured_json_config_text: bool,
    current_block_is_tsconfig_json_config_text: bool,
    file_type_hint: CompressionFileType,
    file_profile_hint: CompressionFileProfile,
    #[cfg(test)]
    diagnostics: RefCell<MatcherDiagnostics>,
}

impl MatchGenerator {
    /// max_size defines how many bytes will be used at most in the window used for matching
    fn new(max_size: usize) -> Self {
        Self {
            max_window_size: max_size,
            fast_window_size: max_size,
            window: Vec::new(),
            current_second_newest_sidecar: Vec::new(),
            current_third_newest_sidecar: Vec::new(),
            current_fourth_newest_sidecar: Vec::new(),
            current_fifth_newest_sidecar: Vec::new(),
            current_sixth_newest_sidecar: Vec::new(),
            current_seventh_newest_sidecar: Vec::new(),
            current_eighth_newest_sidecar: Vec::new(),
            current_ninth_newest_sidecar: Vec::new(),
            current_tenth_newest_sidecar: Vec::new(),
            current_eleventh_newest_sidecar: Vec::new(),
            current_twelfth_newest_sidecar: Vec::new(),
            current_thirteenth_newest_sidecar: Vec::new(),
            current_long_hash: Vec::new(),
            window_size: 0,
            #[cfg(debug_assertions)]
            concat_window: Vec::new(),
            uniform_suffix_len_log: None,
            suffix_idx: 0,
            last_idx_in_sequence: 0,
            offset_history: OffsetHistory::new(),
            is_text_block: false,
            is_short_line_text: false,
            min_non_repeat_match_len: MIN_MATCH_LEN,
            adaptive_binary_no_match_probe: false,
            use_fast_small_dense_binary_probe: false,
            prefer_binary_next_position_repeat_lookahead: false,
            prefer_fast_binary_next_position_repeat_lookahead: false,
            prefer_binary_next_position_lookahead: false,
            prefer_oldest_first_window_probe: false,
            use_complementary_end_insertion: false,
            use_second_newest_probe: false,
            use_fast_binary_small_second_newest: false,
            use_text_repeat_pipeline: false,
            current_block_is_dictionary_lockfile_text: false,
            current_block_is_composer_dictionary_text: false,
            current_block_is_structured_json_config_text: false,
            current_block_is_tsconfig_json_config_text: false,
            file_type_hint: CompressionFileType::Unknown,
            file_profile_hint: CompressionFileProfile::None,
            #[cfg(test)]
            diagnostics: RefCell::new(MatcherDiagnostics::default()),
        }
    }

    fn reset(&mut self, mut reuse_space: impl FnMut(Vec<u8>, SuffixStore)) {
        self.window_size = 0;
        self.current_second_newest_sidecar.clear();
        self.current_third_newest_sidecar.clear();
        self.current_fourth_newest_sidecar.clear();
        self.current_fifth_newest_sidecar.clear();
        self.current_sixth_newest_sidecar.clear();
        self.current_seventh_newest_sidecar.clear();
        self.current_eighth_newest_sidecar.clear();
        self.current_ninth_newest_sidecar.clear();
        self.current_tenth_newest_sidecar.clear();
        self.current_eleventh_newest_sidecar.clear();
        self.current_twelfth_newest_sidecar.clear();
        self.current_thirteenth_newest_sidecar.clear();
        self.current_long_hash.clear();
        #[cfg(debug_assertions)]
        self.concat_window.clear();
        self.uniform_suffix_len_log = None;
        self.suffix_idx = 0;
        self.last_idx_in_sequence = 0;
        self.offset_history = OffsetHistory::new();
        self.is_text_block = false;
        self.is_short_line_text = false;
        self.min_non_repeat_match_len = MIN_MATCH_LEN;
        self.adaptive_binary_no_match_probe = false;
        self.use_fast_small_dense_binary_probe = false;
        self.prefer_binary_next_position_repeat_lookahead = false;
        self.prefer_fast_binary_next_position_repeat_lookahead = false;
        self.prefer_binary_next_position_lookahead = false;
        self.prefer_oldest_first_window_probe = false;
        self.use_complementary_end_insertion = false;
        self.use_second_newest_probe = false;
        self.use_fast_binary_small_second_newest = false;
        self.use_text_repeat_pipeline = false;
        self.current_block_is_dictionary_lockfile_text = false;
        self.current_block_is_composer_dictionary_text = false;
        self.current_block_is_structured_json_config_text = false;
        self.current_block_is_tsconfig_json_config_text = false;
        self.file_type_hint = CompressionFileType::Unknown;
        self.file_profile_hint = CompressionFileProfile::None;
        #[cfg(test)]
        {
            *self.diagnostics.borrow_mut() = MatcherDiagnostics::default();
        }
        self.window.drain(..).for_each(|entry| {
            reuse_space(entry.data, entry.suffixes);
        });
    }

    fn set_window_sizes(&mut self, max_size: usize, fast_size: usize) {
        debug_assert!(self.window.is_empty());
        self.max_window_size = max_size;
        self.fast_window_size = fast_size.min(max_size);
    }

    #[inline(always)]
    fn last_entry(&self) -> &WindowEntry {
        match self.window.last() {
            Some(entry) => entry,
            None => Self::missing_window_entry(),
        }
    }

    #[inline(always)]
    fn last_entry_mut(&mut self) -> &mut WindowEntry {
        match self.window.last_mut() {
            Some(entry) => entry,
            None => Self::missing_window_entry(),
        }
    }

    #[inline(always)]
    fn last_entry_index(&self) -> usize {
        match self.window.len().checked_sub(1) {
            Some(idx) => idx,
            None => Self::missing_window_entry(),
        }
    }

    #[cold]
    #[inline(never)]
    fn missing_window_entry() -> ! {
        panic!("match generator requires a committed window entry")
    }

    /// Processes bytes in the current window until either a match is found or no more matches can be found
    /// * If a match is found handle_sequence is called with the Triple variant
    /// * If no more matches can be found but there are bytes still left handle_sequence is called with the Literals variant
    /// * If no more matches can be found and no more bytes are left this returns false
    fn next_sequence(&mut self, mut handle_sequence: impl for<'a> FnMut(Sequence<'a>)) -> bool {
        if self.use_best_text_repeat_pipeline() {
            return self.next_sequence_best_text(&mut handle_sequence);
        }

        loop {
            let block_len = self.last_entry().data.len();

            // We already reached the end of the window, check if we need to return a Literals{}
            if self.suffix_idx >= block_len {
                if self.last_idx_in_sequence != self.suffix_idx {
                    let last_entry_idx = self.last_entry_index();
                    let literals = &self.window[last_entry_idx].data[self.last_idx_in_sequence..];
                    self.last_idx_in_sequence = self.suffix_idx;
                    handle_sequence(Sequence::Literals { literals });
                    return true;
                } else {
                    return false;
                }
            }

            // If the remaining data is smaller than the minimum match length we can stop and return a Literals{}
            let last_entry_idx = self.last_entry_index();
            let last_entry = &self.window[last_entry_idx];
            let data_slice = &last_entry.data[self.suffix_idx..];
            if data_slice.len() < MIN_MATCH_LEN {
                let last_idx_in_sequence = self.last_idx_in_sequence;
                self.last_idx_in_sequence = block_len;
                self.suffix_idx = block_len;
                handle_sequence(Sequence::Literals {
                    literals: &last_entry.data[last_idx_in_sequence..],
                });
                return true;
            }

            // This is the key we are looking to find a match for
            let key_value = SuffixStore::key_value(&data_slice[..MIN_MATCH_LEN]);

            // Look in each window entry
            let match_context = MatchCandidateContext {
                suffix_idx: self.suffix_idx,
                anchor_idx: self.last_idx_in_sequence,
                min_non_repeat_match_len: self.min_non_repeat_match_len,
                data_slice,
                #[cfg(debug_assertions)]
                last_entry_len: block_len,
                #[cfg(debug_assertions)]
                concat_window: &self.concat_window,
            };

            let previous_window_len = self.window_size - block_len;
            let literal_len = self.suffix_idx - self.last_idx_in_sequence;
            let mut candidate = None;
            for (repeat_idx, &(_repeat_kind, offset)) in self
                .repeat_offset_candidates(literal_len)
                .iter()
                .enumerate()
            {
                if !self.allow_repeat_candidate(literal_len, repeat_idx) {
                    continue;
                }
                if !Self::repeat_offset_is_available(offset, previous_window_len, self.suffix_idx) {
                    continue;
                }
                let Some(verified_prefix_len) =
                    self.verified_min_match_prefix_len(offset, &match_context)
                else {
                    continue;
                };
                let match_len = self.match_len_at_offset_with_prefix(
                    offset,
                    &match_context,
                    verified_prefix_len,
                );
                if match_len >= MIN_MATCH_LEN {
                    let found = MatchCandidate {
                        start_idx: self.suffix_idx,
                        offset,
                        match_len,
                        repeat_offset: true,
                        #[cfg(test)]
                        source: CandidateSource::RepeatCurrent(_repeat_kind),
                    };
                    if candidate
                        .map(|current| self.candidate_is_better_than(found, current))
                        .unwrap_or(true)
                    {
                        candidate = Some(found);
                    }
                }
            }

            if self.prefer_binary_next_position_repeat_lookahead
                || (self.prefer_fast_binary_next_position_repeat_lookahead && !self.is_text_block)
            {
                let should_probe = match candidate {
                    None => true,
                    Some(current) => {
                        !current.repeat_offset
                            && current.start_idx == self.suffix_idx
                            && current.match_len == MIN_MATCH_LEN
                    }
                };
                if should_probe {
                    if let Some(next_candidate) = self.best_repeat_candidate_at(
                        self.suffix_idx + 1,
                        self.last_idx_in_sequence,
                        previous_window_len,
                        block_len,
                    ) {
                        #[cfg(test)]
                        let next_candidate = MatchCandidate {
                            source: CandidateSource::RepeatNextPosition(
                                next_candidate.source_repeat_kind(),
                            ),
                            ..next_candidate
                        };
                        match candidate {
                            None => {
                                #[cfg(test)]
                                self.diagnostics
                                    .borrow_mut()
                                    .record_repeat_next_position_selection(
                                        next_candidate.source_repeat_kind(),
                                        RepeatNextPositionSelectionReason::NoCurrentCandidate,
                                    );
                                candidate = Some(next_candidate);
                            }
                            Some(current)
                                if self.candidate_is_better_than(next_candidate, current) =>
                            {
                                #[cfg(test)]
                                self.diagnostics
                                    .borrow_mut()
                                    .record_repeat_next_position_selection(
                                        next_candidate.source_repeat_kind(),
                                        RepeatNextPositionSelectionReason::BeatsCurrentMinNonRepeat,
                                    );
                                candidate = Some(next_candidate);
                            }
                            _ => {}
                        }
                    }
                }
            }

            #[cfg(test)]
            if let Some(found) = candidate.filter(|found| found.repeat_offset) {
                self.diagnostics
                    .borrow_mut()
                    .record_repeat_best_before_window(
                        found.source_repeat_kind(),
                        found.start_idx == self.last_idx_in_sequence,
                    );
            }
            let repeat_match_reaches_end_or_is_long = candidate
                .is_some_and(|found| self.repeat_match_can_skip_window_search(found, block_len));
            if !repeat_match_reaches_end_or_is_long {
                if let Some(window_candidate) =
                    self.best_window_candidate(key_value, &match_context, block_len)
                {
                    #[cfg(test)]
                    if let Some(found) = candidate.filter(|found| found.repeat_offset) {
                        if self.candidate_is_better_than(window_candidate, found) {
                            self.diagnostics
                                .borrow_mut()
                                .record_repeat_best_before_window_overridden_by_window(
                                    found.source_repeat_kind(),
                                );
                        }
                    }
                    if candidate
                        .map(|current| self.candidate_is_better_than(window_candidate, current))
                        .unwrap_or(true)
                    {
                        candidate = Some(window_candidate);
                    }
                }
                if self.prefer_binary_next_position_lookahead {
                    candidate = self.prefer_next_position_window_candidate(candidate, block_len);
                }
            }

            candidate = self.prefer_lockfile_zero_literal_next_position_candidate(
                candidate,
                previous_window_len,
                block_len,
            );

            if let Some(candidate) = candidate {
                self.emit_candidate(candidate, &mut handle_sequence);
                return true;
            }

            let suffix_idx = self.suffix_idx;
            let probe_step = self.no_match_probe_step(suffix_idx);
            let can_skip_next_probe = suffix_idx + probe_step + MIN_MATCH_LEN <= block_len
                && (1..probe_step).all(|skip| {
                    !self.repeat_offset_can_match_at(suffix_idx + skip, previous_window_len)
                });
            self.add_suffix_at(suffix_idx);
            let step = if can_skip_next_probe {
                for skip in 1..probe_step {
                    self.add_suffix_at(suffix_idx + skip);
                }
                probe_step
            } else {
                1
            };
            self.suffix_idx += step;
        }
    }

    #[inline(never)]
    fn next_sequence_best_text(
        &mut self,
        mut handle_sequence: impl for<'a> FnMut(Sequence<'a>),
    ) -> bool {
        loop {
            let block_len = self.last_entry().data.len();

            if self.suffix_idx >= block_len {
                if self.last_idx_in_sequence != self.suffix_idx {
                    let last_entry_idx = self.last_entry_index();
                    let literals = &self.window[last_entry_idx].data[self.last_idx_in_sequence..];
                    self.last_idx_in_sequence = self.suffix_idx;
                    handle_sequence(Sequence::Literals { literals });
                    return true;
                } else {
                    return false;
                }
            }

            let last_entry_idx = self.last_entry_index();
            let last_entry = &self.window[last_entry_idx];
            let data_slice = &last_entry.data[self.suffix_idx..];
            if data_slice.len() < MIN_MATCH_LEN {
                let last_idx_in_sequence = self.last_idx_in_sequence;
                self.last_idx_in_sequence = block_len;
                self.suffix_idx = block_len;
                handle_sequence(Sequence::Literals {
                    literals: &last_entry.data[last_idx_in_sequence..],
                });
                return true;
            }

            let key_value = SuffixStore::key_value(&data_slice[..MIN_MATCH_LEN]);
            let match_context = MatchCandidateContext {
                suffix_idx: self.suffix_idx,
                anchor_idx: self.last_idx_in_sequence,
                min_non_repeat_match_len: self.min_non_repeat_match_len,
                data_slice,
                #[cfg(debug_assertions)]
                last_entry_len: block_len,
                #[cfg(debug_assertions)]
                concat_window: &self.concat_window,
            };

            let previous_window_len = self.window_size - block_len;
            let mut candidate =
                self.best_text_repeat_candidate(&match_context, previous_window_len, block_len);

            #[cfg(test)]
            if let Some(found) = candidate.filter(|found| found.repeat_offset) {
                self.diagnostics
                    .borrow_mut()
                    .record_repeat_best_before_window(
                        found.source_repeat_kind(),
                        found.start_idx == self.last_idx_in_sequence,
                    );
            }
            let repeat_match_reaches_end_or_is_long = candidate
                .is_some_and(|found| self.repeat_match_can_skip_window_search(found, block_len));
            if !repeat_match_reaches_end_or_is_long {
                if let Some(window_candidate) =
                    self.best_window_candidate(key_value, &match_context, block_len)
                {
                    #[cfg(test)]
                    if let Some(found) = candidate.filter(|found| found.repeat_offset) {
                        if self.candidate_is_better_than(window_candidate, found) {
                            self.diagnostics
                                .borrow_mut()
                                .record_repeat_best_before_window_overridden_by_window(
                                    found.source_repeat_kind(),
                                );
                        }
                    }
                    if candidate
                        .map(|current| self.candidate_is_better_than(window_candidate, current))
                        .unwrap_or(true)
                    {
                        candidate = Some(window_candidate);
                    }
                }
                if self.prefer_binary_next_position_lookahead {
                    candidate = self.prefer_next_position_window_candidate(candidate, block_len);
                }
            }

            candidate = self.prefer_lockfile_zero_literal_next_position_candidate(
                candidate,
                previous_window_len,
                block_len,
            );

            if let Some(candidate) = candidate {
                self.emit_candidate(candidate, &mut handle_sequence);
                return true;
            }

            let suffix_idx = self.suffix_idx;
            let probe_step = self.no_match_probe_step(suffix_idx);
            let can_skip_next_probe = suffix_idx + probe_step + MIN_MATCH_LEN <= block_len
                && (1..probe_step).all(|skip| {
                    !self.repeat_offset_can_match_at(suffix_idx + skip, previous_window_len)
                });
            self.add_suffix_at(suffix_idx);
            let step = if can_skip_next_probe {
                for skip in 1..probe_step {
                    self.add_suffix_at(suffix_idx + skip);
                }
                probe_step
            } else {
                1
            };
            self.suffix_idx += step;
        }
    }

    #[inline(always)]
    fn best_window_candidate(
        &self,
        key_value: u64,
        context: &MatchCandidateContext<'_>,
        block_len: usize,
    ) -> Option<MatchCandidate> {
        let mut candidate = self.best_current_long_hash_candidate(context, block_len);
        #[cfg(test)]
        let long_hash_found = candidate;
        if candidate
            .is_some_and(|candidate| candidate.match_len >= BEST_CURRENT_LONG_HASH_SKIP_OLDER_LEN)
        {
            #[cfg(test)]
            if let Some(found) = long_hash_found {
                self.diagnostics
                    .borrow_mut()
                    .record_current_long_hash_outcome(found, candidate);
            }
            return candidate;
        }
        let current_long_hash_active = candidate.is_some();
        let skip_current_entry = candidate.is_some();
        if self.uniform_suffix_len_log == Some(self.last_entry().suffixes.len_log) {
            let slot_key = self.last_entry().suffixes.slot_key(key_value);
            for (entry_distance, match_entry) in self
                .window
                .iter()
                .rev()
                .enumerate()
                .skip(skip_current_entry as usize)
            {
                let skip_newest_for_entry = current_long_hash_active
                    && entry_distance >= BEST_CURRENT_LONG_HASH_DISTANT_NEWEST_ENTRY_START;
                let Some(candidates) = match_entry.suffixes.candidates_for_slot_key(slot_key)
                else {
                    continue;
                };
                if self.prefer_oldest_first_window_probe {
                    let prefer_newest_first_for_entry =
                        current_long_hash_active && entry_distance == 1;

                    if prefer_newest_first_for_entry {
                        if !skip_newest_for_entry {
                            if let Some(match_index) = candidates.newest {
                                if self.consider_window_candidate_with_tracking(
                                    match_entry,
                                    match_index,
                                    context,
                                    WindowCandidateMeta {
                                        entry_distance,
                                        kind: WindowCandidateKind::Newest,
                                    },
                                    &mut candidate,
                                    block_len,
                                    current_long_hash_active,
                                ) {
                                    break;
                                }
                            }
                        }
                    } else if self.consider_window_candidate_with_tracking(
                        match_entry,
                        candidates.oldest,
                        context,
                        WindowCandidateMeta {
                            entry_distance,
                            kind: WindowCandidateKind::Oldest,
                        },
                        &mut candidate,
                        block_len,
                        current_long_hash_active,
                    ) {
                        break;
                    }

                    if self.should_track_second_newest_for_current_entry() {
                        if let Some(match_index) = self.best_second_newest_candidate(
                            entry_distance,
                            slot_key.index,
                            context,
                            candidate,
                        ) {
                            if self.consider_window_candidate_with_tracking(
                                match_entry,
                                match_index,
                                context,
                                WindowCandidateMeta {
                                    entry_distance,
                                    kind: WindowCandidateKind::SecondNewest,
                                },
                                &mut candidate,
                                block_len,
                                current_long_hash_active,
                            ) {
                                break;
                            }
                        }
                    }

                    if prefer_newest_first_for_entry {
                        if self.consider_window_candidate_with_tracking(
                            match_entry,
                            candidates.oldest,
                            context,
                            WindowCandidateMeta {
                                entry_distance,
                                kind: WindowCandidateKind::Oldest,
                            },
                            &mut candidate,
                            block_len,
                            current_long_hash_active,
                        ) {
                            break;
                        }
                    } else if !skip_newest_for_entry {
                        if let Some(match_index) = candidates.newest {
                            if self.consider_window_candidate_with_tracking(
                                match_entry,
                                match_index,
                                context,
                                WindowCandidateMeta {
                                    entry_distance,
                                    kind: WindowCandidateKind::Newest,
                                },
                                &mut candidate,
                                block_len,
                                current_long_hash_active,
                            ) {
                                break;
                            }
                        }
                    }
                } else {
                    if self.prefer_lockfile_second_newest_before_newest()
                        && self.should_track_second_newest_for_current_entry()
                    {
                        if let Some(match_index) = self.best_second_newest_candidate(
                            entry_distance,
                            slot_key.index,
                            context,
                            candidate,
                        ) {
                            if self.consider_window_candidate_with_tracking(
                                match_entry,
                                match_index,
                                context,
                                WindowCandidateMeta {
                                    entry_distance,
                                    kind: WindowCandidateKind::SecondNewest,
                                },
                                &mut candidate,
                                block_len,
                                current_long_hash_active,
                            ) {
                                break;
                            }
                        }
                        if let Some(match_index) = self.best_third_newest_candidate(
                            entry_distance,
                            slot_key.index,
                            context,
                        ) {
                            if self.consider_window_candidate_with_tracking(
                                match_entry,
                                match_index,
                                context,
                                WindowCandidateMeta {
                                    entry_distance,
                                    kind: WindowCandidateKind::SecondNewest,
                                },
                                &mut candidate,
                                block_len,
                                current_long_hash_active,
                            ) {
                                break;
                            }
                        }
                        if let Some(match_index) = self.best_fourth_newest_candidate(
                            entry_distance,
                            slot_key.index,
                            context,
                        ) {
                            if self.consider_window_candidate_with_tracking(
                                match_entry,
                                match_index,
                                context,
                                WindowCandidateMeta {
                                    entry_distance,
                                    kind: WindowCandidateKind::SecondNewest,
                                },
                                &mut candidate,
                                block_len,
                                current_long_hash_active,
                            ) {
                                break;
                            }
                        }
                        if let Some(match_index) = self.best_fifth_newest_candidate(
                            entry_distance,
                            slot_key.index,
                            context,
                        ) {
                            if self.consider_window_candidate_with_tracking(
                                match_entry,
                                match_index,
                                context,
                                WindowCandidateMeta {
                                    entry_distance,
                                    kind: WindowCandidateKind::SecondNewest,
                                },
                                &mut candidate,
                                block_len,
                                current_long_hash_active,
                            ) {
                                break;
                            }
                        }
                        if let Some(match_index) = self.best_sixth_newest_candidate(
                            entry_distance,
                            slot_key.index,
                            context,
                        ) {
                            if self.consider_window_candidate_with_tracking(
                                match_entry,
                                match_index,
                                context,
                                WindowCandidateMeta {
                                    entry_distance,
                                    kind: WindowCandidateKind::SecondNewest,
                                },
                                &mut candidate,
                                block_len,
                                current_long_hash_active,
                            ) {
                                break;
                            }
                        }
                        if let Some(match_index) = self.best_seventh_newest_candidate(
                            entry_distance,
                            slot_key.index,
                            context,
                        ) {
                            if self.consider_window_candidate_with_tracking(
                                match_entry,
                                match_index,
                                context,
                                WindowCandidateMeta {
                                    entry_distance,
                                    kind: WindowCandidateKind::SecondNewest,
                                },
                                &mut candidate,
                                block_len,
                                current_long_hash_active,
                            ) {
                                break;
                            }
                        }
                        if let Some(match_index) = self.best_eighth_newest_candidate(
                            entry_distance,
                            slot_key.index,
                            context,
                        ) {
                            if self.consider_window_candidate_with_tracking(
                                match_entry,
                                match_index,
                                context,
                                WindowCandidateMeta {
                                    entry_distance,
                                    kind: WindowCandidateKind::SecondNewest,
                                },
                                &mut candidate,
                                block_len,
                                current_long_hash_active,
                            ) {
                                break;
                            }
                        }
                        if let Some(match_index) = self.best_ninth_newest_candidate(
                            entry_distance,
                            slot_key.index,
                            context,
                        ) {
                            if self.consider_window_candidate_with_tracking(
                                match_entry,
                                match_index,
                                context,
                                WindowCandidateMeta {
                                    entry_distance,
                                    kind: WindowCandidateKind::SecondNewest,
                                },
                                &mut candidate,
                                block_len,
                                current_long_hash_active,
                            ) {
                                break;
                            }
                        }
                        if let Some(match_index) = self.best_tenth_newest_candidate(
                            entry_distance,
                            slot_key.index,
                            context,
                        ) {
                            if self.consider_window_candidate_with_tracking(
                                match_entry,
                                match_index,
                                context,
                                WindowCandidateMeta {
                                    entry_distance,
                                    kind: WindowCandidateKind::SecondNewest,
                                },
                                &mut candidate,
                                block_len,
                                current_long_hash_active,
                            ) {
                                break;
                            }
                        }
                        if let Some(match_index) = self.best_eleventh_newest_candidate(
                            entry_distance,
                            slot_key.index,
                            context,
                        ) {
                            if self.consider_window_candidate_with_tracking(
                                match_entry,
                                match_index,
                                context,
                                WindowCandidateMeta {
                                    entry_distance,
                                    kind: WindowCandidateKind::SecondNewest,
                                },
                                &mut candidate,
                                block_len,
                                current_long_hash_active,
                            ) {
                                break;
                            }
                        }
                        if let Some(match_index) = self.best_twelfth_newest_candidate(
                            entry_distance,
                            slot_key.index,
                            context,
                        ) {
                            if self.consider_window_candidate_with_tracking(
                                match_entry,
                                match_index,
                                context,
                                WindowCandidateMeta {
                                    entry_distance,
                                    kind: WindowCandidateKind::SecondNewest,
                                },
                                &mut candidate,
                                block_len,
                                current_long_hash_active,
                            ) {
                                break;
                            }
                        }
                        if let Some(match_index) = self.best_thirteenth_newest_candidate(
                            entry_distance,
                            slot_key.index,
                            context,
                        ) {
                            if self.consider_window_candidate_with_tracking(
                                match_entry,
                                match_index,
                                context,
                                WindowCandidateMeta {
                                    entry_distance,
                                    kind: WindowCandidateKind::SecondNewest,
                                },
                                &mut candidate,
                                block_len,
                                current_long_hash_active,
                            ) {
                                break;
                            }
                        }
                    }

                    if !skip_newest_for_entry {
                        if let Some(match_index) = candidates.newest {
                            if self.consider_window_candidate_with_tracking(
                                match_entry,
                                match_index,
                                context,
                                WindowCandidateMeta {
                                    entry_distance,
                                    kind: WindowCandidateKind::Newest,
                                },
                                &mut candidate,
                                block_len,
                                current_long_hash_active,
                            ) {
                                break;
                            }
                        }
                    }

                    if !self.prefer_lockfile_second_newest_before_newest()
                        && self.should_track_second_newest_for_current_entry()
                    {
                        if let Some(match_index) = self.best_second_newest_candidate(
                            entry_distance,
                            slot_key.index,
                            context,
                            candidate,
                        ) {
                            if self.consider_window_candidate_with_tracking(
                                match_entry,
                                match_index,
                                context,
                                WindowCandidateMeta {
                                    entry_distance,
                                    kind: WindowCandidateKind::SecondNewest,
                                },
                                &mut candidate,
                                block_len,
                                current_long_hash_active,
                            ) {
                                break;
                            }
                        }
                    }

                    if self.consider_window_candidate_with_tracking(
                        match_entry,
                        candidates.oldest,
                        context,
                        WindowCandidateMeta {
                            entry_distance,
                            kind: WindowCandidateKind::Oldest,
                        },
                        &mut candidate,
                        block_len,
                        current_long_hash_active,
                    ) {
                        break;
                    }
                }
            }
            #[cfg(test)]
            if let Some(found) = long_hash_found {
                self.diagnostics
                    .borrow_mut()
                    .record_current_long_hash_outcome(found, candidate);
            }
            return candidate;
        }

        for (entry_distance, match_entry) in self
            .window
            .iter()
            .rev()
            .enumerate()
            .skip(skip_current_entry as usize)
        {
            let skip_newest_for_entry = current_long_hash_active
                && entry_distance >= BEST_CURRENT_LONG_HASH_DISTANT_NEWEST_ENTRY_START;
            let Some(candidates) = match_entry.suffixes.candidates_for_key_value(key_value) else {
                continue;
            };
            if self.prefer_oldest_first_window_probe {
                let prefer_newest_first_for_entry = current_long_hash_active && entry_distance == 1;

                if prefer_newest_first_for_entry {
                    if !skip_newest_for_entry {
                        if let Some(match_index) = candidates.newest {
                            if self.consider_window_candidate_with_tracking(
                                match_entry,
                                match_index,
                                context,
                                WindowCandidateMeta {
                                    entry_distance,
                                    kind: WindowCandidateKind::Newest,
                                },
                                &mut candidate,
                                block_len,
                                current_long_hash_active,
                            ) {
                                break;
                            }
                        }
                    }
                } else if self.consider_window_candidate_with_tracking(
                    match_entry,
                    candidates.oldest,
                    context,
                    WindowCandidateMeta {
                        entry_distance,
                        kind: WindowCandidateKind::Oldest,
                    },
                    &mut candidate,
                    block_len,
                    current_long_hash_active,
                ) {
                    break;
                }

                if self.should_track_second_newest_for_current_entry() {
                    if let Some(match_index) = self.best_second_newest_candidate(
                        entry_distance,
                        match_entry.suffixes.slot_key(key_value).index,
                        context,
                        candidate,
                    ) {
                        if self.consider_window_candidate_with_tracking(
                            match_entry,
                            match_index,
                            context,
                            WindowCandidateMeta {
                                entry_distance,
                                kind: WindowCandidateKind::SecondNewest,
                            },
                            &mut candidate,
                            block_len,
                            current_long_hash_active,
                        ) {
                            break;
                        }
                    }
                }

                if prefer_newest_first_for_entry {
                    if self.consider_window_candidate_with_tracking(
                        match_entry,
                        candidates.oldest,
                        context,
                        WindowCandidateMeta {
                            entry_distance,
                            kind: WindowCandidateKind::Oldest,
                        },
                        &mut candidate,
                        block_len,
                        current_long_hash_active,
                    ) {
                        break;
                    }
                } else if !skip_newest_for_entry {
                    if let Some(match_index) = candidates.newest {
                        if self.consider_window_candidate_with_tracking(
                            match_entry,
                            match_index,
                            context,
                            WindowCandidateMeta {
                                entry_distance,
                                kind: WindowCandidateKind::Newest,
                            },
                            &mut candidate,
                            block_len,
                            current_long_hash_active,
                        ) {
                            break;
                        }
                    }
                }
            } else {
                if self.prefer_lockfile_second_newest_before_newest()
                    && self.should_track_second_newest_for_current_entry()
                {
                    if let Some(match_index) = self.best_second_newest_candidate(
                        entry_distance,
                        match_entry.suffixes.slot_key(key_value).index,
                        context,
                        candidate,
                    ) {
                        if self.consider_window_candidate(
                            match_entry,
                            match_index,
                            context,
                            WindowCandidateMeta {
                                entry_distance,
                                kind: WindowCandidateKind::SecondNewest,
                            },
                            &mut candidate,
                            block_len,
                        ) {
                            break;
                        }
                    }
                    if let Some(match_index) = self.best_third_newest_candidate(
                        entry_distance,
                        match_entry.suffixes.slot_key(key_value).index,
                        context,
                    ) {
                        if self.consider_window_candidate(
                            match_entry,
                            match_index,
                            context,
                            WindowCandidateMeta {
                                entry_distance,
                                kind: WindowCandidateKind::SecondNewest,
                            },
                            &mut candidate,
                            block_len,
                        ) {
                            break;
                        }
                    }
                    if let Some(match_index) = self.best_fourth_newest_candidate(
                        entry_distance,
                        match_entry.suffixes.slot_key(key_value).index,
                        context,
                    ) {
                        if self.consider_window_candidate(
                            match_entry,
                            match_index,
                            context,
                            WindowCandidateMeta {
                                entry_distance,
                                kind: WindowCandidateKind::SecondNewest,
                            },
                            &mut candidate,
                            block_len,
                        ) {
                            break;
                        }
                    }
                    if let Some(match_index) = self.best_fifth_newest_candidate(
                        entry_distance,
                        match_entry.suffixes.slot_key(key_value).index,
                        context,
                    ) {
                        if self.consider_window_candidate(
                            match_entry,
                            match_index,
                            context,
                            WindowCandidateMeta {
                                entry_distance,
                                kind: WindowCandidateKind::SecondNewest,
                            },
                            &mut candidate,
                            block_len,
                        ) {
                            break;
                        }
                    }
                    if let Some(match_index) = self.best_sixth_newest_candidate(
                        entry_distance,
                        match_entry.suffixes.slot_key(key_value).index,
                        context,
                    ) {
                        if self.consider_window_candidate(
                            match_entry,
                            match_index,
                            context,
                            WindowCandidateMeta {
                                entry_distance,
                                kind: WindowCandidateKind::SecondNewest,
                            },
                            &mut candidate,
                            block_len,
                        ) {
                            break;
                        }
                    }
                    if let Some(match_index) = self.best_seventh_newest_candidate(
                        entry_distance,
                        match_entry.suffixes.slot_key(key_value).index,
                        context,
                    ) {
                        if self.consider_window_candidate(
                            match_entry,
                            match_index,
                            context,
                            WindowCandidateMeta {
                                entry_distance,
                                kind: WindowCandidateKind::SecondNewest,
                            },
                            &mut candidate,
                            block_len,
                        ) {
                            break;
                        }
                    }
                    if let Some(match_index) = self.best_eighth_newest_candidate(
                        entry_distance,
                        match_entry.suffixes.slot_key(key_value).index,
                        context,
                    ) {
                        if self.consider_window_candidate(
                            match_entry,
                            match_index,
                            context,
                            WindowCandidateMeta {
                                entry_distance,
                                kind: WindowCandidateKind::SecondNewest,
                            },
                            &mut candidate,
                            block_len,
                        ) {
                            break;
                        }
                    }
                    if let Some(match_index) = self.best_ninth_newest_candidate(
                        entry_distance,
                        match_entry.suffixes.slot_key(key_value).index,
                        context,
                    ) {
                        if self.consider_window_candidate(
                            match_entry,
                            match_index,
                            context,
                            WindowCandidateMeta {
                                entry_distance,
                                kind: WindowCandidateKind::SecondNewest,
                            },
                            &mut candidate,
                            block_len,
                        ) {
                            break;
                        }
                    }
                    if let Some(match_index) = self.best_tenth_newest_candidate(
                        entry_distance,
                        match_entry.suffixes.slot_key(key_value).index,
                        context,
                    ) {
                        if self.consider_window_candidate(
                            match_entry,
                            match_index,
                            context,
                            WindowCandidateMeta {
                                entry_distance,
                                kind: WindowCandidateKind::SecondNewest,
                            },
                            &mut candidate,
                            block_len,
                        ) {
                            break;
                        }
                    }
                    if let Some(match_index) = self.best_eleventh_newest_candidate(
                        entry_distance,
                        match_entry.suffixes.slot_key(key_value).index,
                        context,
                    ) {
                        if self.consider_window_candidate(
                            match_entry,
                            match_index,
                            context,
                            WindowCandidateMeta {
                                entry_distance,
                                kind: WindowCandidateKind::SecondNewest,
                            },
                            &mut candidate,
                            block_len,
                        ) {
                            break;
                        }
                    }
                    if let Some(match_index) = self.best_twelfth_newest_candidate(
                        entry_distance,
                        match_entry.suffixes.slot_key(key_value).index,
                        context,
                    ) {
                        if self.consider_window_candidate(
                            match_entry,
                            match_index,
                            context,
                            WindowCandidateMeta {
                                entry_distance,
                                kind: WindowCandidateKind::SecondNewest,
                            },
                            &mut candidate,
                            block_len,
                        ) {
                            break;
                        }
                    }
                    if let Some(match_index) = self.best_thirteenth_newest_candidate(
                        entry_distance,
                        match_entry.suffixes.slot_key(key_value).index,
                        context,
                    ) {
                        if self.consider_window_candidate(
                            match_entry,
                            match_index,
                            context,
                            WindowCandidateMeta {
                                entry_distance,
                                kind: WindowCandidateKind::SecondNewest,
                            },
                            &mut candidate,
                            block_len,
                        ) {
                            break;
                        }
                    }
                }

                if !skip_newest_for_entry {
                    if let Some(match_index) = candidates.newest {
                        if self.consider_window_candidate_with_tracking(
                            match_entry,
                            match_index,
                            context,
                            WindowCandidateMeta {
                                entry_distance,
                                kind: WindowCandidateKind::Newest,
                            },
                            &mut candidate,
                            block_len,
                            current_long_hash_active,
                        ) {
                            break;
                        }
                    }
                }

                if !self.prefer_lockfile_second_newest_before_newest()
                    && self.should_track_second_newest_for_current_entry()
                {
                    if let Some(match_index) = self.best_second_newest_candidate(
                        entry_distance,
                        match_entry.suffixes.slot_key(key_value).index,
                        context,
                        candidate,
                    ) {
                        if self.consider_window_candidate(
                            match_entry,
                            match_index,
                            context,
                            WindowCandidateMeta {
                                entry_distance,
                                kind: WindowCandidateKind::SecondNewest,
                            },
                            &mut candidate,
                            block_len,
                        ) {
                            break;
                        }
                    }
                }

                if self.consider_window_candidate_with_tracking(
                    match_entry,
                    candidates.oldest,
                    context,
                    WindowCandidateMeta {
                        entry_distance,
                        kind: WindowCandidateKind::Oldest,
                    },
                    &mut candidate,
                    block_len,
                    current_long_hash_active,
                ) {
                    break;
                }
            }
        }

        #[cfg(test)]
        if let Some(found) = long_hash_found {
            self.diagnostics
                .borrow_mut()
                .record_current_long_hash_outcome(found, candidate);
        }
        candidate
    }

    #[inline(always)]
    fn best_current_long_hash_candidate(
        &self,
        context: &MatchCandidateContext<'_>,
        block_len: usize,
    ) -> Option<MatchCandidate> {
        if !self.should_track_current_long_hash()
            || context.suffix_idx != self.suffix_idx
            || context.data_slice.len() < 8
        {
            return None;
        }

        let long_key = u64::from_le_bytes(context.data_slice[..8].try_into().ok()?);
        let slot_index = self.last_entry().suffixes.slot_key(long_key).index;
        let match_index = self
            .current_long_hash
            .get(slot_index)
            .and_then(|idx| *idx)
            .map(|idx| idx.get() as usize - 1)?;

        let current_entry = self.last_entry();
        if !Self::has_long_match_at_index(current_entry, match_index, context) {
            return None;
        }

        let offset = context.suffix_idx - match_index;
        let match_len = self.match_len_at_offset_with_prefix(offset, context, 8);
        if match_len < MIN_MATCH_LEN {
            return None;
        }
        let (start_idx, match_len) = self.extend_match_backwards(offset, match_len, context);

        #[cfg(debug_assertions)]
        {
            let unprocessed = context.last_entry_len - context.suffix_idx;
            let current_start = context.concat_window.len() - unprocessed;
            let current_match_start = current_start - (context.suffix_idx - start_idx);
            let match_start = current_match_start - offset;
            let match_end = match_start + match_len;
            let check_slice = &context.concat_window[match_start..match_end];
            let current_end = start_idx + match_len;
            debug_assert_eq!(check_slice, &self.last_entry().data[start_idx..current_end]);
        }

        let candidate = MatchCandidate {
            start_idx,
            offset,
            match_len,
            repeat_offset: false,
            #[cfg(test)]
            source: CandidateSource::WindowCurrentLongHash,
        };
        if candidate.start_idx + candidate.match_len == block_len {
            return Some(candidate);
        }
        Some(candidate)
    }

    #[inline(always)]
    fn best_second_newest_candidate(
        &self,
        entry_distance: usize,
        slot_index: usize,
        context: &MatchCandidateContext<'_>,
        candidate: Option<MatchCandidate>,
    ) -> Option<usize> {
        if !self.should_probe_second_newest(context, entry_distance, candidate) {
            return None;
        }

        self.current_second_newest_sidecar
            .get(slot_index)
            .and_then(|idx| *idx)
            .map(|idx| idx.get() as usize - 1)
    }

    #[inline(always)]
    fn best_third_newest_candidate(
        &self,
        entry_distance: usize,
        slot_index: usize,
        context: &MatchCandidateContext<'_>,
    ) -> Option<usize> {
        if !self.should_probe_third_newest(context, entry_distance) {
            return None;
        }

        self.current_third_newest_sidecar
            .get(slot_index)
            .and_then(|idx| *idx)
            .map(|idx| idx.get() as usize - 1)
    }

    #[inline(always)]
    fn best_fourth_newest_candidate(
        &self,
        entry_distance: usize,
        slot_index: usize,
        context: &MatchCandidateContext<'_>,
    ) -> Option<usize> {
        if !self.should_probe_fourth_newest(context, entry_distance) {
            return None;
        }

        self.current_fourth_newest_sidecar
            .get(slot_index)
            .and_then(|idx| *idx)
            .map(|idx| idx.get() as usize - 1)
    }

    #[inline(always)]
    fn best_fifth_newest_candidate(
        &self,
        entry_distance: usize,
        slot_index: usize,
        context: &MatchCandidateContext<'_>,
    ) -> Option<usize> {
        if !self.should_probe_fifth_newest(context, entry_distance) {
            return None;
        }

        self.current_fifth_newest_sidecar
            .get(slot_index)
            .and_then(|idx| *idx)
            .map(|idx| idx.get() as usize - 1)
    }

    #[inline(always)]
    fn best_sixth_newest_candidate(
        &self,
        entry_distance: usize,
        slot_index: usize,
        context: &MatchCandidateContext<'_>,
    ) -> Option<usize> {
        if !self.should_probe_sixth_newest(context, entry_distance) {
            return None;
        }

        self.current_sixth_newest_sidecar
            .get(slot_index)
            .and_then(|idx| *idx)
            .map(|idx| idx.get() as usize - 1)
    }

    #[inline(always)]
    fn best_seventh_newest_candidate(
        &self,
        entry_distance: usize,
        slot_index: usize,
        context: &MatchCandidateContext<'_>,
    ) -> Option<usize> {
        if !self.should_probe_seventh_newest(context, entry_distance) {
            return None;
        }

        self.current_seventh_newest_sidecar
            .get(slot_index)
            .and_then(|idx| *idx)
            .map(|idx| idx.get() as usize - 1)
    }

    #[inline(always)]
    fn best_eighth_newest_candidate(
        &self,
        entry_distance: usize,
        slot_index: usize,
        context: &MatchCandidateContext<'_>,
    ) -> Option<usize> {
        if !self.should_probe_eighth_newest(context, entry_distance) {
            return None;
        }

        self.current_eighth_newest_sidecar
            .get(slot_index)
            .and_then(|idx| *idx)
            .map(|idx| idx.get() as usize - 1)
    }

    #[inline(always)]
    fn best_ninth_newest_candidate(
        &self,
        entry_distance: usize,
        slot_index: usize,
        context: &MatchCandidateContext<'_>,
    ) -> Option<usize> {
        if !self.should_probe_ninth_newest(context, entry_distance) {
            return None;
        }

        self.current_ninth_newest_sidecar
            .get(slot_index)
            .and_then(|idx| *idx)
            .map(|idx| idx.get() as usize - 1)
    }

    #[inline(always)]
    fn best_tenth_newest_candidate(
        &self,
        entry_distance: usize,
        slot_index: usize,
        context: &MatchCandidateContext<'_>,
    ) -> Option<usize> {
        if !self.should_probe_tenth_newest(context, entry_distance) {
            return None;
        }

        self.current_tenth_newest_sidecar
            .get(slot_index)
            .and_then(|idx| *idx)
            .map(|idx| idx.get() as usize - 1)
    }

    #[inline(always)]
    fn best_eleventh_newest_candidate(
        &self,
        entry_distance: usize,
        slot_index: usize,
        context: &MatchCandidateContext<'_>,
    ) -> Option<usize> {
        if !self.should_probe_eleventh_newest(context, entry_distance) {
            return None;
        }

        self.current_eleventh_newest_sidecar
            .get(slot_index)
            .and_then(|idx| *idx)
            .map(|idx| idx.get() as usize - 1)
    }

    #[inline(always)]
    fn best_twelfth_newest_candidate(
        &self,
        entry_distance: usize,
        slot_index: usize,
        context: &MatchCandidateContext<'_>,
    ) -> Option<usize> {
        if !self.should_probe_twelfth_newest(context, entry_distance) {
            return None;
        }

        self.current_twelfth_newest_sidecar
            .get(slot_index)
            .and_then(|idx| *idx)
            .map(|idx| idx.get() as usize - 1)
    }

    #[inline(always)]
    fn best_thirteenth_newest_candidate(
        &self,
        entry_distance: usize,
        slot_index: usize,
        context: &MatchCandidateContext<'_>,
    ) -> Option<usize> {
        if !self.should_probe_thirteenth_newest(context, entry_distance) {
            return None;
        }

        self.current_thirteenth_newest_sidecar
            .get(slot_index)
            .and_then(|idx| *idx)
            .map(|idx| idx.get() as usize - 1)
    }

    #[inline(always)]
    fn should_probe_second_newest(
        &self,
        context: &MatchCandidateContext<'_>,
        entry_distance: usize,
        candidate: Option<MatchCandidate>,
    ) -> bool {
        let zero_literals = context.suffix_idx == context.anchor_idx;
        #[cfg(feature = "std")]
        if self.uses_dictionary_lockfile_second_newest_path()
            && zero_literals
            && matches!(
                matcher_tuning_overrides().lockfile_second_newest_zero_literals,
                Some(false)
            )
        {
            return false;
        }
        let recent_entry_limit = if self.uses_dictionary_lockfile_second_newest_path() {
            LOCKFILE_SECOND_NEWEST_RECENT_ENTRY_LIMIT
        } else {
            BEST_SECOND_NEWEST_RECENT_ENTRY_LIMIT
        };
        self.should_track_second_newest_for_current_entry()
            && context.suffix_idx == self.suffix_idx
            && entry_distance < recent_entry_limit
            && (!self.use_fast_binary_small_second_newest || candidate.is_none())
    }

    #[inline(always)]
    fn should_probe_third_newest(
        &self,
        context: &MatchCandidateContext<'_>,
        entry_distance: usize,
    ) -> bool {
        self.uses_dictionary_lockfile_second_newest_path()
            && context.suffix_idx == self.suffix_idx
            && entry_distance == 0
    }

    #[inline(always)]
    fn should_probe_fourth_newest(
        &self,
        context: &MatchCandidateContext<'_>,
        entry_distance: usize,
    ) -> bool {
        self.uses_dictionary_lockfile_second_newest_path()
            && context.suffix_idx == self.suffix_idx
            && entry_distance == 0
    }

    #[inline(always)]
    fn should_probe_fifth_newest(
        &self,
        context: &MatchCandidateContext<'_>,
        entry_distance: usize,
    ) -> bool {
        self.uses_dictionary_lockfile_second_newest_path()
            && context.suffix_idx == self.suffix_idx
            && entry_distance == 0
    }

    #[inline(always)]
    fn should_probe_sixth_newest(
        &self,
        context: &MatchCandidateContext<'_>,
        entry_distance: usize,
    ) -> bool {
        self.uses_dictionary_lockfile_second_newest_path()
            && context.suffix_idx == self.suffix_idx
            && entry_distance == 0
    }

    #[inline(always)]
    fn should_probe_seventh_newest(
        &self,
        context: &MatchCandidateContext<'_>,
        entry_distance: usize,
    ) -> bool {
        self.uses_dictionary_lockfile_second_newest_path()
            && context.suffix_idx == self.suffix_idx
            && entry_distance == 0
    }

    #[inline(always)]
    fn should_probe_eighth_newest(
        &self,
        context: &MatchCandidateContext<'_>,
        entry_distance: usize,
    ) -> bool {
        self.uses_dictionary_lockfile_second_newest_path()
            && context.suffix_idx == self.suffix_idx
            && entry_distance == 0
    }

    #[inline(always)]
    fn should_probe_ninth_newest(
        &self,
        context: &MatchCandidateContext<'_>,
        entry_distance: usize,
    ) -> bool {
        self.uses_dictionary_lockfile_second_newest_path()
            && context.suffix_idx == self.suffix_idx
            && entry_distance == 0
    }

    #[inline(always)]
    fn should_probe_tenth_newest(
        &self,
        context: &MatchCandidateContext<'_>,
        entry_distance: usize,
    ) -> bool {
        self.uses_dictionary_lockfile_second_newest_path()
            && context.suffix_idx == self.suffix_idx
            && entry_distance == 0
    }

    #[inline(always)]
    fn should_probe_eleventh_newest(
        &self,
        context: &MatchCandidateContext<'_>,
        entry_distance: usize,
    ) -> bool {
        self.uses_dictionary_lockfile_second_newest_path()
            && context.suffix_idx == self.suffix_idx
            && entry_distance == 0
    }

    #[inline(always)]
    fn should_probe_twelfth_newest(
        &self,
        context: &MatchCandidateContext<'_>,
        entry_distance: usize,
    ) -> bool {
        self.uses_dictionary_lockfile_second_newest_path()
            && context.suffix_idx == self.suffix_idx
            && entry_distance == 0
    }

    #[inline(always)]
    fn should_probe_thirteenth_newest(
        &self,
        context: &MatchCandidateContext<'_>,
        entry_distance: usize,
    ) -> bool {
        self.uses_dictionary_lockfile_second_newest_path()
            && context.suffix_idx == self.suffix_idx
            && entry_distance == 0
    }

    #[inline(always)]
    fn should_track_second_newest_for_current_entry(&self) -> bool {
        (self.use_second_newest_probe
            || self.uses_dictionary_current_entry_second_newest_path()
            || self.uses_code_text_current_entry_second_newest_path()
            || self.uses_dictionary_lockfile_second_newest_path()
            || (self.use_fast_binary_small_second_newest
                && !self.is_text_block
                && (self.last_entry().data.len() <= FASTEST_SECOND_NEWEST_MAX_BLOCK_LEN
                    || (matches!(self.file_type_hint, CompressionFileType::Unknown)
                        && self.last_entry().data.len()
                            <= FASTEST_UNKNOWN_SECOND_NEWEST_MAX_BLOCK_LEN))))
            && (self.min_non_repeat_match_len == MIN_MATCH_LEN
                || self.uses_code_text_current_entry_second_newest_path())
            && (self.last_entry().data.len() >= BEST_SECOND_NEWEST_MIN_BLOCK_LEN
                || self.use_fast_binary_small_second_newest)
    }

    #[inline(always)]
    fn should_track_current_long_hash(&self) -> bool {
        self.use_second_newest_probe
            && self.min_non_repeat_match_len == MIN_MATCH_LEN
            && self.last_entry().data.len() >= BEST_CURRENT_LONG_HASH_MIN_BLOCK_LEN
    }

    #[inline(always)]
    fn use_best_text_repeat_pipeline(&self) -> bool {
        self.is_text_block
            && (self.use_text_repeat_pipeline
                || matches!(self.file_type_hint, CompressionFileType::DictionaryText))
    }

    #[inline(always)]
    fn uses_dictionary_current_entry_second_newest_path(&self) -> bool {
        matches!(self.file_type_hint, CompressionFileType::DictionaryText)
            && self.is_text_block
            && !self.current_block_is_dictionary_lockfile_text
            && self.last_entry().data.len() >= BEST_SECOND_NEWEST_MIN_BLOCK_LEN
    }

    #[inline(always)]
    fn uses_code_text_current_entry_second_newest_path(&self) -> bool {
        matches!(self.file_type_hint, CompressionFileType::CodeText)
            && self.is_short_line_text
            && self.last_entry().data.len() >= BEST_SECOND_NEWEST_MIN_BLOCK_LEN
            && self.last_entry().data.len() <= CODE_TEXT_SECOND_NEWEST_MAX_BLOCK_LEN
    }

    #[inline(always)]
    fn uses_dictionary_lockfile_second_newest_path(&self) -> bool {
        self.current_block_is_dictionary_lockfile_text
    }

    #[inline(always)]
    fn prefer_lockfile_second_newest_before_newest(&self) -> bool {
        self.uses_dictionary_lockfile_second_newest_path()
    }

    #[inline(always)]
    fn use_complementary_end_insertion_for_current_block(&self) -> bool {
        self.use_complementary_end_insertion
    }

    #[inline(always)]
    fn uses_large_unknown_fastest_path(&self) -> bool {
        self.prefer_fast_binary_next_position_repeat_lookahead
            && matches!(self.file_type_hint, CompressionFileType::Unknown)
            && !self.is_text_block
            && self.min_non_repeat_match_len == MIN_MATCH_LEN
            && self.last_entry().data.len() >= FASTEST_UNKNOWN_SECOND_NEWEST_MAX_BLOCK_LEN
    }

    #[inline(always)]
    fn repeat_match_len_margin(&self) -> usize {
        let mut margin = REPEAT_MATCH_LEN_MARGIN;
        if self.uses_large_unknown_fastest_path() {
            margin += 3;
        }
        margin
    }

    #[inline(always)]
    fn candidate_is_better_than(&self, found: MatchCandidate, current: MatchCandidate) -> bool {
        if matches!(self.file_type_hint, CompressionFileType::DictionaryText) {
            if self.prefer_lockfile_smaller_offset_same_end(found, current) {
                return true;
            }
            if self.prefer_lockfile_smaller_offset_same_end(current, found) {
                return false;
            }
            if self.prefer_lockfile_repeat_kind_same_start(found, current) {
                return true;
            }
            if self.prefer_lockfile_repeat_kind_same_start(current, found) {
                return false;
            }
            if self.prefer_composer_repeat_kind_same_start(found, current) {
                return true;
            }
            if self.prefer_composer_repeat_kind_same_start(current, found) {
                return false;
            }
            if self.prefer_dictionary_smaller_offset_same_start(found, current) {
                return true;
            }
            if self.prefer_dictionary_smaller_offset_same_start(current, found) {
                return false;
            }
        }
        if self.uses_large_unknown_fastest_path() {
            if self.prefer_large_unknown_smaller_offset(found, current) {
                return true;
            }
            if self.prefer_large_unknown_smaller_offset(current, found) {
                return false;
            }
        }

        if found.repeat_offset != current.repeat_offset {
            let margin = self.repeat_match_len_margin();
            if found.repeat_offset {
                return found.match_len + margin >= current.match_len;
            }
            return found.match_len > current.match_len + margin;
        }

        found.match_len > current.match_len
            || (found.match_len == current.match_len
                && (found.start_idx < current.start_idx
                    || (found.start_idx == current.start_idx && found.offset < current.offset)))
    }

    #[inline(always)]
    fn window_candidate_is_better_than(
        &self,
        found: MatchCandidate,
        current: MatchCandidate,
        meta: WindowCandidateMeta,
    ) -> bool {
        if self.keep_lockfile_current_candidate_over_oldest(found, current, meta) {
            return false;
        }
        if self.keep_large_unknown_current_candidate_over_newest(found, current, meta) {
            return false;
        }
        if self.keep_large_unknown_current_candidate_over_oldest(found, current, meta) {
            return false;
        }
        self.candidate_is_better_than(found, current)
    }

    fn keep_lockfile_current_candidate_over_oldest(
        &self,
        found: MatchCandidate,
        current: MatchCandidate,
        meta: WindowCandidateMeta,
    ) -> bool {
        self.uses_dictionary_lockfile_second_newest_path()
            && matches!(meta.kind, WindowCandidateKind::Oldest)
            && !found.repeat_offset
            && !current.repeat_offset
            && current.offset < found.offset
            && found.match_len < current.match_len + LOCKFILE_OLDEST_DISPLACEMENT_MIN_GAIN
    }

    #[inline(always)]
    fn keep_large_unknown_current_candidate_over_newest(
        &self,
        found: MatchCandidate,
        current: MatchCandidate,
        meta: WindowCandidateMeta,
    ) -> bool {
        self.uses_large_unknown_fastest_path()
            && matches!(meta.kind, WindowCandidateKind::Newest)
            && !found.repeat_offset
            && !current.repeat_offset
            && current.offset < found.offset
            && found.match_len < current.match_len + LARGE_UNKNOWN_NEWEST_DISPLACEMENT_MIN_GAIN
    }

    #[inline(always)]
    fn keep_large_unknown_current_candidate_over_oldest(
        &self,
        found: MatchCandidate,
        current: MatchCandidate,
        meta: WindowCandidateMeta,
    ) -> bool {
        self.uses_large_unknown_fastest_path()
            && matches!(meta.kind, WindowCandidateKind::Oldest)
            && !found.repeat_offset
            && !current.repeat_offset
            && current.offset < found.offset
            && found.match_len < current.match_len + LARGE_UNKNOWN_OLDEST_DISPLACEMENT_MIN_GAIN
    }

    #[inline(always)]
    fn dictionary_same_start_bits_gain_min(&self) -> usize {
        #[cfg(feature = "std")]
        if let Some(value) = matcher_tuning_overrides().dictionary_same_start_bits_gain_min {
            return value;
        }
        DICTIONARY_SMALLER_OFFSET_BITS_GAIN_MIN
    }

    #[inline(always)]
    fn dictionary_same_start_match_loss_max(&self) -> usize {
        #[cfg(feature = "std")]
        if let Some(value) = matcher_tuning_overrides().dictionary_same_start_match_loss_max {
            return value;
        }
        DICTIONARY_SMALLER_OFFSET_MATCH_LOSS_MAX
    }

    #[inline(always)]
    fn lockfile_same_end_bits_gain_min(&self) -> usize {
        #[cfg(feature = "std")]
        if let Some(value) = matcher_tuning_overrides().lockfile_same_end_bits_gain_min {
            return value;
        }
        DICTIONARY_SMALLER_OFFSET_BITS_GAIN_MIN
    }

    #[inline(always)]
    fn lockfile_same_end_match_loss_max(&self) -> usize {
        #[cfg(feature = "std")]
        if let Some(value) = matcher_tuning_overrides().lockfile_same_end_match_loss_max {
            return value;
        }
        LOCKFILE_SAME_END_SMALLER_OFFSET_MATCH_LOSS_MAX
    }

    #[inline(always)]
    fn lockfile_repeat_kind_match_loss_max(&self) -> usize {
        #[cfg(feature = "std")]
        if let Some(value) = matcher_tuning_overrides().lockfile_repeat_kind_match_loss_max {
            return value;
        }
        LOCKFILE_REPEAT_KIND_MATCH_LOSS_MAX
    }

    #[inline(always)]
    fn composer_repeat_kind_match_loss_max(&self) -> usize {
        #[cfg(feature = "std")]
        if let Some(value) = matcher_tuning_overrides().composer_repeat_kind_match_loss_max {
            return value;
        }
        COMPOSER_REPEAT_KIND_MATCH_LOSS_MAX
    }

    #[inline(always)]
    fn composer_repeat_kind_zero_literals_only(&self) -> bool {
        #[cfg(feature = "std")]
        if let Some(value) = matcher_tuning_overrides().composer_repeat_kind_zero_literals_only {
            return value;
        }
        false
    }

    fn prefer_dictionary_smaller_offset_same_start(
        &self,
        preferred: MatchCandidate,
        other: MatchCandidate,
    ) -> bool {
        if !matches!(self.file_type_hint, CompressionFileType::DictionaryText)
            || preferred.repeat_offset
            || other.repeat_offset
            || preferred.start_idx != other.start_idx
            || preferred.offset >= other.offset
            || preferred.match_len + self.dictionary_same_start_match_loss_max() < other.match_len
        {
            return false;
        }

        let preferred_bits = self.non_repeat_offset_code_bits(preferred.offset);
        let other_bits = self.non_repeat_offset_code_bits(other.offset);
        other_bits >= preferred_bits + self.dictionary_same_start_bits_gain_min()
    }

    fn prefer_lockfile_repeat_kind_same_start(
        &self,
        preferred: MatchCandidate,
        other: MatchCandidate,
    ) -> bool {
        if !self.current_block_is_dictionary_lockfile_text
            || !preferred.repeat_offset
            || !other.repeat_offset
            || preferred.start_idx != other.start_idx
            || preferred.start_idx != self.suffix_idx
            || preferred.match_len + self.lockfile_repeat_kind_match_loss_max() < other.match_len
        {
            return false;
        }

        let literal_len = self.suffix_idx - self.last_idx_in_sequence;
        let Some(preferred_rank) = self.repeat_kind_preference_rank(preferred.offset, literal_len)
        else {
            return false;
        };
        let Some(other_rank) = self.repeat_kind_preference_rank(other.offset, literal_len) else {
            return false;
        };
        preferred_rank < other_rank
    }

    fn prefer_lockfile_smaller_offset_same_end(
        &self,
        preferred: MatchCandidate,
        other: MatchCandidate,
    ) -> bool {
        if !self.current_block_is_dictionary_lockfile_text
            || preferred.repeat_offset
            || other.repeat_offset
            || preferred.offset >= other.offset
            || preferred.match_len + self.lockfile_same_end_match_loss_max() < other.match_len
        {
            return false;
        }

        let preferred_end = preferred.start_idx + preferred.match_len;
        let other_end = other.start_idx + other.match_len;
        if preferred_end != other_end || preferred.start_idx <= other.start_idx {
            return false;
        }

        let preferred_bits = self.non_repeat_offset_code_bits(preferred.offset);
        let other_bits = self.non_repeat_offset_code_bits(other.offset);
        other_bits >= preferred_bits + self.lockfile_same_end_bits_gain_min()
    }

    fn prefer_composer_repeat_kind_same_start(
        &self,
        preferred: MatchCandidate,
        other: MatchCandidate,
    ) -> bool {
        let literal_len = self.suffix_idx - self.last_idx_in_sequence;
        if !self.current_block_is_composer_dictionary_text
            || !preferred.repeat_offset
            || !other.repeat_offset
            || preferred.start_idx != other.start_idx
            || preferred.start_idx != self.suffix_idx
            || (self.composer_repeat_kind_zero_literals_only() && literal_len != 0)
            || preferred.match_len + self.composer_repeat_kind_match_loss_max() < other.match_len
        {
            return false;
        }

        let Some(preferred_rank) = self.repeat_kind_preference_rank(preferred.offset, literal_len)
        else {
            return false;
        };
        let Some(other_rank) = self.repeat_kind_preference_rank(other.offset, literal_len) else {
            return false;
        };
        preferred_rank < other_rank
    }

    fn repeat_kind_preference_rank(&self, offset: usize, literal_len: usize) -> Option<usize> {
        self.repeat_offset_candidates(literal_len)
            .iter()
            .position(|(_, candidate_offset)| *candidate_offset == offset)
    }

    fn prefer_large_unknown_smaller_offset(
        &self,
        preferred: MatchCandidate,
        other: MatchCandidate,
    ) -> bool {
        if !self.uses_large_unknown_fastest_path()
            || preferred.repeat_offset
            || other.repeat_offset
            || preferred.offset >= other.offset
            || preferred.match_len + LARGE_UNKNOWN_SMALLER_OFFSET_MATCH_LOSS_MAX < other.match_len
        {
            return false;
        }

        let preferred_bits = self.non_repeat_offset_code_bits(preferred.offset);
        let other_bits = self.non_repeat_offset_code_bits(other.offset);
        other_bits >= preferred_bits + LARGE_UNKNOWN_SMALLER_OFFSET_BITS_GAIN_MIN
    }

    #[inline(always)]
    fn non_repeat_offset_code_bits(&self, offset: usize) -> usize {
        Self::bounded_u32(offset + 3).ilog2() as usize
    }

    #[inline(always)]
    fn allow_repeat_length_early_exit(&self) -> bool {
        !self.uses_large_unknown_fastest_path()
    }

    #[inline(always)]
    fn allow_repeat_block_end_early_exit(&self) -> bool {
        true
    }

    #[inline(always)]
    fn repeat_match_can_skip_window_search(
        &self,
        candidate: MatchCandidate,
        block_len: usize,
    ) -> bool {
        candidate.repeat_offset
            && ((self.allow_repeat_block_end_early_exit()
                && candidate.start_idx + candidate.match_len == block_len)
                || (self.allow_repeat_length_early_exit()
                    && candidate.match_len >= REPEAT_SEARCH_EARLY_EXIT_LEN))
    }

    #[inline(never)]
    fn best_text_repeat_candidate(
        &self,
        context: &MatchCandidateContext<'_>,
        previous_window_len: usize,
        block_len: usize,
    ) -> Option<MatchCandidate> {
        let literal_len = context.suffix_idx - context.anchor_idx;
        let allow_next_position = context.suffix_idx + 1 + MIN_MATCH_LEN <= block_len;
        let next_context = if allow_next_position {
            let next_slice = &context.data_slice[1..];
            Some(MatchCandidateContext {
                suffix_idx: context.suffix_idx + 1,
                anchor_idx: context.anchor_idx,
                min_non_repeat_match_len: context.min_non_repeat_match_len,
                data_slice: next_slice,
                #[cfg(debug_assertions)]
                last_entry_len: block_len,
                #[cfg(debug_assertions)]
                concat_window: context.concat_window,
            })
        } else {
            None
        };

        let mut current_candidate = None;
        let mut next_candidate = None;

        for (repeat_idx, &(_repeat_kind, offset)) in self
            .repeat_offset_candidates(literal_len)
            .iter()
            .enumerate()
        {
            if !self.allow_repeat_candidate(literal_len, repeat_idx) {
                continue;
            }
            if Self::repeat_offset_is_available(offset, previous_window_len, context.suffix_idx) {
                let Some(verified_prefix_len) = self.verified_min_match_prefix_len(offset, context)
                else {
                    continue;
                };
                let match_len =
                    self.match_len_at_offset_with_prefix(offset, context, verified_prefix_len);
                if match_len >= MIN_MATCH_LEN {
                    let found = MatchCandidate {
                        start_idx: context.suffix_idx,
                        offset,
                        match_len,
                        repeat_offset: true,
                        #[cfg(test)]
                        source: CandidateSource::RepeatCurrent(_repeat_kind),
                    };
                    if current_candidate
                        .map(|current| self.candidate_is_better_than(found, current))
                        .unwrap_or(true)
                    {
                        current_candidate = Some(found);
                    }
                }
            }

            let Some(next_context) = next_context.as_ref() else {
                continue;
            };
            if current_candidate.is_some() {
                continue;
            }
            if !Self::repeat_offset_is_available(
                offset,
                previous_window_len,
                next_context.suffix_idx,
            ) {
                continue;
            }
            let Some(verified_prefix_len) =
                self.verified_min_match_prefix_len(offset, next_context)
            else {
                continue;
            };
            let match_len =
                self.match_len_at_offset_with_prefix(offset, next_context, verified_prefix_len);
            if match_len < MIN_MATCH_LEN {
                continue;
            }
            let found = MatchCandidate {
                start_idx: next_context.suffix_idx,
                offset,
                match_len,
                repeat_offset: true,
                #[cfg(test)]
                source: CandidateSource::RepeatCurrent(_repeat_kind),
            };
            if next_candidate
                .map(|current| self.candidate_is_better_than(found, current))
                .unwrap_or(true)
            {
                next_candidate = Some(found);
            }
        }

        #[cfg(test)]
        let next_candidate = next_candidate.map(|candidate| MatchCandidate {
            source: CandidateSource::RepeatNextPosition(candidate.source_repeat_kind()),
            ..candidate
        });

        current_candidate.or(next_candidate)
    }

    #[inline(always)]
    fn best_repeat_candidate_at(
        &self,
        suffix_idx: usize,
        anchor_idx: usize,
        previous_window_len: usize,
        _block_len: usize,
    ) -> Option<MatchCandidate> {
        let last_entry = self.last_entry();
        let data_slice = last_entry.data.get(suffix_idx..)?;
        if data_slice.len() < MIN_MATCH_LEN {
            return None;
        }

        let context = MatchCandidateContext {
            suffix_idx,
            anchor_idx,
            min_non_repeat_match_len: self.min_non_repeat_match_len,
            data_slice,
            #[cfg(debug_assertions)]
            last_entry_len: _block_len,
            #[cfg(debug_assertions)]
            concat_window: &self.concat_window,
        };
        let literal_len = suffix_idx - anchor_idx;
        let mut candidate = None;
        for (repeat_idx, &(_repeat_kind, offset)) in self
            .repeat_offset_candidates(literal_len)
            .iter()
            .enumerate()
        {
            if !self.allow_repeat_candidate(literal_len, repeat_idx) {
                continue;
            }
            if !Self::repeat_offset_is_available(offset, previous_window_len, suffix_idx) {
                continue;
            }
            let Some(verified_prefix_len) = self.verified_min_match_prefix_len(offset, &context)
            else {
                continue;
            };
            let match_len =
                self.match_len_at_offset_with_prefix(offset, &context, verified_prefix_len);
            if match_len < MIN_MATCH_LEN {
                continue;
            }
            let found = MatchCandidate {
                start_idx: suffix_idx,
                offset,
                match_len,
                repeat_offset: true,
                #[cfg(test)]
                source: CandidateSource::RepeatCurrent(_repeat_kind),
            };
            if candidate
                .map(|current| self.candidate_is_better_than(found, current))
                .unwrap_or(true)
            {
                candidate = Some(found);
            }
        }

        candidate
    }

    #[inline(always)]
    fn prefer_next_position_window_candidate(
        &self,
        candidate: Option<MatchCandidate>,
        block_len: usize,
    ) -> Option<MatchCandidate> {
        if !self.prefer_binary_next_position_lookahead
            || self.min_non_repeat_match_len != MIN_MATCH_LEN
            || self.suffix_idx + 1 + MIN_MATCH_LEN > block_len
        {
            return candidate;
        }

        let should_probe = match candidate {
            None => true,
            Some(current) => {
                !current.repeat_offset
                    && current.start_idx == self.suffix_idx
                    && current.match_len == MIN_MATCH_LEN
            }
        };
        if !should_probe {
            return candidate;
        }

        let last_entry = self.last_entry();
        let next_slice = &last_entry.data[self.suffix_idx + 1..];
        let next_context = MatchCandidateContext {
            suffix_idx: self.suffix_idx + 1,
            anchor_idx: self.last_idx_in_sequence,
            min_non_repeat_match_len: self.min_non_repeat_match_len,
            data_slice: next_slice,
            #[cfg(debug_assertions)]
            last_entry_len: block_len,
            #[cfg(debug_assertions)]
            concat_window: &self.concat_window,
        };
        let key_value = SuffixStore::key_value(&next_slice[..MIN_MATCH_LEN]);
        let Some(next_candidate) = self.best_window_candidate(key_value, &next_context, block_len)
        else {
            return candidate;
        };
        #[cfg(test)]
        let next_candidate = MatchCandidate {
            source: match next_candidate.source {
                CandidateSource::WindowCurrentNewest { entry_distance } => {
                    CandidateSource::WindowNextPositionNewest { entry_distance }
                }
                CandidateSource::WindowCurrentSecondNewest { entry_distance } => {
                    CandidateSource::WindowNextPositionSecondNewest { entry_distance }
                }
                CandidateSource::WindowCurrentOldest { entry_distance } => {
                    CandidateSource::WindowNextPositionOldest { entry_distance }
                }
                other => other,
            },
            ..next_candidate
        };

        match candidate {
            None if !next_candidate.repeat_offset => Some(next_candidate),
            Some(current)
                if !next_candidate.repeat_offset
                    && self.candidate_is_better_than(next_candidate, current) =>
            {
                Some(next_candidate)
            }
            Some(current) => Some(current),
            None => None,
        }
    }

    #[inline(always)]
    fn lockfile_next_position_tuning(
        &self,
    ) -> Option<(usize, usize, usize, usize, usize, usize, usize)> {
        if !self.current_block_is_dictionary_lockfile_text {
            return None;
        }
        #[cfg(feature = "std")]
        {
            let overrides = matcher_tuning_overrides();
            let max_skip_literals = overrides
                .lockfile_next_position_max_skip_literals
                .unwrap_or(LOCKFILE_NEXT_POSITION_MAX_SKIP_LITERALS);
            let max_current_match_len = overrides
                .lockfile_next_position_max_current_match_len
                .unwrap_or(LOCKFILE_NEXT_POSITION_MAX_CURRENT_MATCH_LEN);
            let max_match_loss = overrides
                .lockfile_next_position_match_loss_max
                .unwrap_or(LOCKFILE_NEXT_POSITION_MATCH_LOSS_MAX);
            let literal_weight = overrides
                .lockfile_next_position_literal_weight
                .unwrap_or(LOCKFILE_NEXT_POSITION_LITERAL_WEIGHT);
            let match_reward = overrides
                .lockfile_next_position_match_reward
                .unwrap_or(LOCKFILE_NEXT_POSITION_MATCH_REWARD);
            let offset_weight = overrides
                .lockfile_next_position_offset_weight
                .unwrap_or(LOCKFILE_NEXT_POSITION_OFFSET_WEIGHT);
            let margin = overrides
                .lockfile_next_position_margin
                .unwrap_or(LOCKFILE_NEXT_POSITION_MARGIN);
            return Some((
                max_skip_literals.max(1),
                max_current_match_len.max(MIN_MATCH_LEN),
                max_match_loss,
                literal_weight.max(1),
                match_reward.max(1),
                offset_weight.max(1),
                margin,
            ));
        }
        #[allow(unreachable_code)]
        Some((
            LOCKFILE_NEXT_POSITION_MAX_SKIP_LITERALS,
            LOCKFILE_NEXT_POSITION_MAX_CURRENT_MATCH_LEN,
            LOCKFILE_NEXT_POSITION_MATCH_LOSS_MAX,
            LOCKFILE_NEXT_POSITION_LITERAL_WEIGHT,
            LOCKFILE_NEXT_POSITION_MATCH_REWARD,
            LOCKFILE_NEXT_POSITION_OFFSET_WEIGHT,
            LOCKFILE_NEXT_POSITION_MARGIN,
        ))
    }

    #[inline(always)]
    fn prefer_lockfile_zero_literal_next_position_candidate(
        &self,
        candidate: Option<MatchCandidate>,
        previous_window_len: usize,
        block_len: usize,
    ) -> Option<MatchCandidate> {
        let Some((
            max_skip_literals,
            max_current_match_len,
            max_match_loss,
            literal_weight,
            match_reward,
            offset_weight,
            margin,
        )) = self.lockfile_next_position_tuning()
        else {
            return candidate;
        };
        let Some(current) = candidate else {
            return candidate;
        };
        if !self.current_block_is_dictionary_lockfile_text
            || self.suffix_idx != self.last_idx_in_sequence
            || current.start_idx != self.suffix_idx
            || current.match_len > max_current_match_len
        {
            return Some(current);
        }
        let current_cost = self.lockfile_estimated_local_path_cost(
            current,
            self.last_idx_in_sequence,
            literal_weight,
            match_reward,
            offset_weight,
        );
        let mut best_candidate = current;
        let mut best_cost = current_cost;

        for skip in 1..=max_skip_literals {
            if self.suffix_idx + skip + MIN_MATCH_LEN > block_len {
                break;
            }
            let Some(next_candidate) = self.best_candidate_at_position(
                self.suffix_idx + skip,
                self.last_idx_in_sequence,
                previous_window_len,
                block_len,
            ) else {
                continue;
            };
            if next_candidate.start_idx != self.suffix_idx + skip {
                continue;
            }
            if next_candidate.match_len + max_match_loss < current.match_len {
                continue;
            }

            let next_cost = self.lockfile_estimated_local_path_cost(
                next_candidate,
                self.last_idx_in_sequence,
                literal_weight,
                match_reward,
                offset_weight,
            );
            if next_cost + margin < best_cost {
                best_candidate = next_candidate;
                best_cost = next_cost;
            }
        }

        Some(best_candidate)
    }

    #[inline(always)]
    fn best_candidate_at_position(
        &self,
        suffix_idx: usize,
        anchor_idx: usize,
        previous_window_len: usize,
        block_len: usize,
    ) -> Option<MatchCandidate> {
        let last_entry = self.last_entry();
        let data_slice = last_entry.data.get(suffix_idx..)?;
        if data_slice.len() < MIN_MATCH_LEN {
            return None;
        }

        let context = MatchCandidateContext {
            suffix_idx,
            anchor_idx,
            min_non_repeat_match_len: self.min_non_repeat_match_len,
            data_slice,
            #[cfg(debug_assertions)]
            last_entry_len: block_len,
            #[cfg(debug_assertions)]
            concat_window: &self.concat_window,
        };

        let mut candidate =
            self.best_repeat_candidate_at(suffix_idx, anchor_idx, previous_window_len, block_len);
        let key_value = SuffixStore::key_value(&data_slice[..MIN_MATCH_LEN]);
        if let Some(window_candidate) = self.best_window_candidate(key_value, &context, block_len) {
            if candidate
                .map(|current| self.candidate_is_better_than(window_candidate, current))
                .unwrap_or(true)
            {
                candidate = Some(window_candidate);
            }
        }
        candidate
    }

    #[inline(always)]
    fn lockfile_estimated_local_parse_cost_with_history(
        &self,
        candidate: MatchCandidate,
        anchor_idx: usize,
        offset_history: &mut OffsetHistory,
        literal_weight: usize,
        match_reward: usize,
        offset_weight: usize,
    ) -> usize {
        let literal_len = candidate.start_idx.saturating_sub(anchor_idx);
        let offset_value =
            offset_history.encode_offset_value(candidate.offset as u32, literal_len as u32);
        let ll_num_bits = Self::local_literal_length_extra_bits(literal_len as u32);
        let ml_num_bits = Self::local_match_length_extra_bits(candidate.match_len as u32);
        let of_code = Self::local_offset_code(offset_value);
        let of_num_bits = of_code as usize;

        let explicit_bits = ll_num_bits + ml_num_bits + of_num_bits;
        let symbol_penalty = of_code as usize * offset_weight;
        let literal_penalty = candidate.start_idx.saturating_sub(anchor_idx) * literal_weight;
        let match_credit = candidate.match_len * match_reward;

        literal_penalty
            .saturating_add(explicit_bits)
            .saturating_add(symbol_penalty)
            .saturating_sub(match_credit)
    }

    #[inline(always)]
    fn lockfile_estimated_local_path_cost(
        &self,
        candidate: MatchCandidate,
        anchor_idx: usize,
        literal_weight: usize,
        match_reward: usize,
        offset_weight: usize,
    ) -> usize {
        let mut offset_history = self.offset_history;
        self.lockfile_estimated_local_parse_cost_with_history(
            candidate,
            anchor_idx,
            &mut offset_history,
            literal_weight,
            match_reward,
            offset_weight,
        )
    }

    #[inline(always)]
    fn consider_window_candidate(
        &self,
        match_entry: &WindowEntry,
        match_index: usize,
        context: &MatchCandidateContext<'_>,
        meta: WindowCandidateMeta,
        candidate: &mut Option<MatchCandidate>,
        block_len: usize,
    ) -> bool {
        let Some(found) = self.match_candidate(match_entry, match_index, context, meta) else {
            return false;
        };
        if self.reject_composer_window_candidate(found) {
            return false;
        }
        if self.reject_lockfile_zero_literal_window_candidate(found, context) {
            return false;
        }
        if !found.worth_emitting(context.min_non_repeat_match_len) {
            return false;
        }

        let improved = candidate
            .map(|current| self.window_candidate_is_better_than(found, current, meta))
            .unwrap_or(true);
        if improved {
            *candidate = Some(found);
        }

        if found.start_idx + found.match_len == block_len {
            return true;
        }

        false
    }

    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    fn consider_window_candidate_with_tracking(
        &self,
        match_entry: &WindowEntry,
        match_index: usize,
        context: &MatchCandidateContext<'_>,
        meta: WindowCandidateMeta,
        candidate: &mut Option<MatchCandidate>,
        block_len: usize,
        _current_long_hash_active: bool,
    ) -> bool {
        let Some(found) = self.match_candidate(match_entry, match_index, context, meta) else {
            return false;
        };
        if self.reject_composer_window_candidate(found) {
            return false;
        }
        if self.reject_lockfile_zero_literal_window_candidate(found, context) {
            return false;
        }
        if !found.worth_emitting(context.min_non_repeat_match_len) {
            return false;
        }

        let improved = candidate
            .map(|current| self.window_candidate_is_better_than(found, current, meta))
            .unwrap_or(true);
        if improved {
            *candidate = Some(found);
        }

        let should_break = found.start_idx + found.match_len == block_len;

        #[cfg(test)]
        if _current_long_hash_active {
            if improved {
                if let Some(selected) = candidate {
                    self.diagnostics
                        .borrow_mut()
                        .record_current_long_hash_improvement(*selected);
                }
            }
            if should_break {
                self.diagnostics
                    .borrow_mut()
                    .record_current_long_hash_end_break(found, improved);
            }
        }

        should_break
    }

    #[inline(always)]
    fn match_candidate(
        &self,
        match_entry: &WindowEntry,
        match_index: usize,
        context: &MatchCandidateContext<'_>,
        _meta: WindowCandidateMeta,
    ) -> Option<MatchCandidate> {
        if !Self::has_min_match_at_index(match_entry, match_index, context) {
            return None;
        }

        let offset = match_entry.base_offset + context.suffix_idx - match_index;
        let match_len = self.match_len_at_offset_with_prefix(offset, context, MIN_MATCH_LEN);
        if match_len < MIN_MATCH_LEN {
            return None;
        }
        let (start_idx, match_len) = self.extend_match_backwards(offset, match_len, context);

        #[cfg(debug_assertions)]
        {
            let unprocessed = context.last_entry_len - context.suffix_idx;
            let current_start = context.concat_window.len() - unprocessed;
            let current_match_start = current_start - (context.suffix_idx - start_idx);
            let match_start = current_match_start - offset;
            let match_end = match_start + match_len;
            let check_slice = &context.concat_window[match_start..match_end];
            let current_end = start_idx + match_len;
            debug_assert_eq!(check_slice, &self.last_entry().data[start_idx..current_end]);
        }

        Some(MatchCandidate {
            start_idx,
            offset,
            match_len,
            repeat_offset: false,
            #[cfg(test)]
            source: match _meta.kind {
                WindowCandidateKind::Newest => CandidateSource::WindowCurrentNewest {
                    entry_distance: _meta.entry_distance,
                },
                WindowCandidateKind::SecondNewest => CandidateSource::WindowCurrentSecondNewest {
                    entry_distance: _meta.entry_distance,
                },
                WindowCandidateKind::Oldest => CandidateSource::WindowCurrentOldest {
                    entry_distance: _meta.entry_distance,
                },
            },
        })
    }

    #[inline(always)]
    fn reject_lockfile_zero_literal_window_candidate(
        &self,
        found: MatchCandidate,
        context: &MatchCandidateContext<'_>,
    ) -> bool {
        #[cfg(feature = "std")]
        {
            let zero_literals = context.suffix_idx == context.anchor_idx;
            if matches!(
                matcher_tuning_overrides().lockfile_zero_literal_window_disable,
                Some(true)
            ) {
                return self.current_block_is_dictionary_lockfile_text
                    && !found.repeat_offset
                    && zero_literals;
            }
            let Some(max_match_len) =
                matcher_tuning_overrides().lockfile_zero_literal_window_max_match_len
            else {
                return false;
            };
            let Some(min_offset_bits) =
                matcher_tuning_overrides().lockfile_zero_literal_window_min_offset_bits
            else {
                return false;
            };
            self.current_block_is_dictionary_lockfile_text
                && !found.repeat_offset
                && zero_literals
                && found.match_len <= max_match_len
                && self.non_repeat_offset_code_bits(found.offset) >= min_offset_bits
        }
        #[cfg(not(feature = "std"))]
        {
            let _ = (found, context);
            false
        }
    }

    #[inline(always)]
    fn reject_composer_window_candidate(&self, found: MatchCandidate) -> bool {
        #[cfg(feature = "std")]
        {
            matches!(
                matcher_tuning_overrides().composer_window_disable,
                Some(true)
            ) && self.current_block_is_composer_dictionary_text
                && !found.repeat_offset
        }
        #[cfg(not(feature = "std"))]
        {
            let _ = found;
            false
        }
    }

    #[inline(always)]
    fn allow_repeat_candidate(&self, literal_len: usize, repeat_idx: usize) -> bool {
        if self.current_block_is_composer_dictionary_text && literal_len == 0 {
            #[cfg(feature = "std")]
            if let Some(limit) =
                matcher_tuning_overrides().composer_zero_literal_repeat_candidate_limit
            {
                return repeat_idx < limit;
            }
        }
        true
    }

    #[inline(always)]
    fn repeat_offset_can_match_at(&self, suffix_idx: usize, previous_window_len: usize) -> bool {
        let literal_len = suffix_idx - self.last_idx_in_sequence;
        for (_, offset) in self.repeat_offset_candidates(literal_len) {
            if Self::repeat_offset_is_available(offset, previous_window_len, suffix_idx)
                && self.has_min_match_at_index_offset(suffix_idx, offset)
            {
                return true;
            }
        }
        false
    }

    #[inline(always)]
    fn repeat_offset_is_available(
        offset: usize,
        previous_window_len: usize,
        suffix_idx: usize,
    ) -> bool {
        offset != 0 && offset <= previous_window_len + suffix_idx
    }

    #[inline(always)]
    fn no_match_probe_step(&self, suffix_idx: usize) -> usize {
        if self.is_text_block {
            if matches!(self.file_type_hint, CompressionFileType::DictionaryText) {
                if self.uses_dictionary_lockfile_second_newest_path() {
                    #[cfg(feature = "std")]
                    if let Some(value) = matcher_tuning_overrides().lockfile_probe_step {
                        return value;
                    }
                    return SHORT_LINE_TEXT_NO_MATCH_PROBE_STEP;
                }
                if self.current_block_is_composer_dictionary_text {
                    #[cfg(feature = "std")]
                    if let Some(value) = matcher_tuning_overrides().composer_probe_step {
                        return value;
                    }
                    return COMPOSER_JSON_LOCKFILE_NO_MATCH_PROBE_STEP;
                }
                return 1;
            }
            if self.is_short_line_text {
                match self.file_type_hint {
                    CompressionFileType::CodeText
                        if self.last_entry().data.len() <= CODE_TEXT_DENSE_PROBE_MAX_BLOCK_LEN =>
                    {
                        return 1;
                    }
                    CompressionFileType::ConfigText
                        if self.current_block_is_tsconfig_json_config_text =>
                    {
                        #[cfg(feature = "std")]
                        if let Some(value) = matcher_tuning_overrides().tsconfig_probe_step {
                            return value;
                        }
                        return TSCONFIG_JSON_TEXT_NO_MATCH_PROBE_STEP;
                    }
                    CompressionFileType::ConfigText
                        if self.current_block_is_structured_json_config_text
                            && self.last_entry().data.len()
                                <= STRUCTURED_JSON_CONFIG_DENSE_PROBE_MAX_BLOCK_LEN =>
                    {
                        #[cfg(feature = "std")]
                        if let Some(value) = matcher_tuning_overrides().structured_json_probe_step {
                            return value;
                        }
                        return 1;
                    }
                    CompressionFileType::ConfigText
                        if self.last_entry().data.len()
                            <= CONFIG_TEXT_DENSE_PROBE_MAX_BLOCK_LEN =>
                    {
                        return 1;
                    }
                    _ => {}
                }
            }
            if self.is_short_line_text {
                return SHORT_LINE_TEXT_NO_MATCH_PROBE_STEP;
            }
            return TEXT_NO_MATCH_PROBE_STEP;
        }

        if self.use_fast_small_dense_binary_probe
            && self.last_entry().data.len() <= FASTEST_DENSE_BINARY_PROBE_MAX_BLOCK_LEN
        {
            return 1;
        }

        let base_step = NO_MATCH_PROBE_STEP;
        if !self.adaptive_binary_no_match_probe {
            return base_step;
        }

        let literal_run_len = suffix_idx - self.last_idx_in_sequence;
        base_step + (literal_run_len >> BEST_BINARY_NO_MATCH_SEARCH_STRENGTH)
    }

    #[inline(always)]
    fn has_min_match_at_index_offset(&self, suffix_idx: usize, offset: usize) -> bool {
        let source_relative = suffix_idx as isize - offset as isize;
        let Some(source) = self.slice_at_relative(source_relative) else {
            return false;
        };

        if source.len() < MIN_MATCH_LEN {
            return true;
        }

        source[..MIN_MATCH_LEN] == self.last_entry().data[suffix_idx..suffix_idx + MIN_MATCH_LEN]
    }

    #[inline(always)]
    fn repeat_offset_candidates(&self, literal_len: usize) -> [(RepeatCandidateKind, usize); 3] {
        if literal_len > 0 {
            [
                (
                    RepeatCandidateKind::First,
                    self.offset_history.newest as usize,
                ),
                (
                    RepeatCandidateKind::Second,
                    self.offset_history.second as usize,
                ),
                (
                    RepeatCandidateKind::Third,
                    self.offset_history.third as usize,
                ),
            ]
        } else {
            [
                (
                    RepeatCandidateKind::Second,
                    self.offset_history.second as usize,
                ),
                (
                    RepeatCandidateKind::Third,
                    self.offset_history.third as usize,
                ),
                (
                    RepeatCandidateKind::First,
                    self.offset_history.newest.saturating_sub(1) as usize,
                ),
            ]
        }
    }

    #[inline(always)]
    fn has_min_match_at_index(
        match_entry: &WindowEntry,
        match_index: usize,
        context: &MatchCandidateContext<'_>,
    ) -> bool {
        match_entry
            .data
            .get(match_index..match_index + MIN_MATCH_LEN)
            .is_some_and(|source| source == &context.data_slice[..MIN_MATCH_LEN])
    }

    #[inline(always)]
    fn has_long_match_at_index(
        match_entry: &WindowEntry,
        match_index: usize,
        context: &MatchCandidateContext<'_>,
    ) -> bool {
        match_entry
            .data
            .get(match_index..match_index + 8)
            .is_some_and(|source| source == &context.data_slice[..8])
    }

    #[inline(always)]
    fn bounded_u32(value: usize) -> u32 {
        match u32::try_from(value) {
            Ok(value) => value,
            Err(_) => unreachable!("match generator indexes are bounded by the compressor window"),
        }
    }

    fn min_non_repeat_match_len_for_text(&self, data: &[u8], is_text_block: bool) -> usize {
        if is_text_block {
            if !self.use_text_repeat_pipeline && Self::likely_short_line_text(data) {
                if Self::likely_code_like_short_text(data) {
                    if matches!(self.file_type_hint, CompressionFileType::CodeText)
                        && data.len() <= SMALL_CODE_TEXT_MIN_NON_REPEAT_MAX_BLOCK_LEN
                    {
                        SMALL_TEXT_MIN_NON_REPEAT_MATCH_LEN
                    } else {
                        CODE_LIKE_SHORT_TEXT_MIN_NON_REPEAT_MATCH_LEN
                    }
                } else {
                    SMALL_TEXT_MIN_NON_REPEAT_MATCH_LEN
                }
            } else {
                TEXT_MIN_NON_REPEAT_MATCH_LEN
            }
        } else {
            MIN_MATCH_LEN
        }
    }

    fn likely_text(data: &[u8]) -> bool {
        const SAMPLE_COUNT: usize = 256;
        const MIN_TEXT_BYTES: usize = 1024;

        if data.len() < MIN_TEXT_BYTES {
            return false;
        }

        let step = (data.len() / SAMPLE_COUNT).max(1);
        let mut printable = 0usize;
        let mut total = 0usize;
        for idx in (0..data.len()).step_by(step).take(SAMPLE_COUNT) {
            total += 1;
            let byte = data[idx];
            if byte == b'\n'
                || byte == b'\r'
                || byte == b'\t'
                || byte.is_ascii_graphic()
                || byte == b' '
            {
                printable += 1;
            }
        }

        printable * 100 >= total * 90
    }

    fn likely_short_line_text(data: &[u8]) -> bool {
        let mut short_lines = 0usize;
        let mut nonempty_lines = 0usize;
        let mut current_len = 0usize;

        for &byte in data {
            if byte == b'\n' {
                if current_len != 0 {
                    nonempty_lines += 1;
                    if current_len <= SHORT_TEXT_LINE_LEN_LIMIT {
                        short_lines += 1;
                    }
                }
                current_len = 0;
            } else if byte != b'\r' {
                current_len += 1;
            }
        }

        if current_len != 0 {
            nonempty_lines += 1;
            if current_len <= SHORT_TEXT_LINE_LEN_LIMIT {
                short_lines += 1;
            }
        }

        nonempty_lines != 0
            && short_lines * 100 >= nonempty_lines * SHORT_TEXT_LINE_FRACTION_PERCENT
    }

    fn likely_lockfile_text(data: &[u8]) -> bool {
        likely_lockfile_text(data)
    }

    fn likely_composer_dictionary_text(data: &[u8]) -> bool {
        likely_composer_lockfile_text(data)
    }

    fn likely_structured_json_config_text(data: &[u8]) -> bool {
        let Some(first_non_ws) = data
            .iter()
            .copied()
            .find(|byte| !matches!(byte, b' ' | b'\t' | b'\r' | b'\n'))
        else {
            return false;
        };
        if first_non_ws != b'{' {
            return false;
        }

        let mut keyed_lines = 0usize;
        let mut content_lines = 0usize;
        for line in data.split(|&byte| byte == b'\n').take(256) {
            let line = line
                .iter()
                .skip_while(|&&byte| matches!(byte, b' ' | b'\t'))
                .copied()
                .collect::<Vec<u8>>();
            let line = line.as_slice();
            if line.is_empty() {
                continue;
            }
            if matches!(line, b"{" | b"}" | b"}," | b"[" | b"]" | b"],") {
                continue;
            }
            content_lines += 1;
            if line.starts_with(b"\"") && line.contains(&b':') {
                keyed_lines += 1;
            }
        }

        content_lines >= 4 && keyed_lines * 100 >= content_lines * 60
    }

    fn likely_tsconfig_json_config_text(data: &[u8]) -> bool {
        let Some(first_non_ws) = data
            .iter()
            .copied()
            .find(|byte| !matches!(byte, b' ' | b'\t' | b'\r' | b'\n'))
        else {
            return false;
        };
        if first_non_ws != b'{' {
            return false;
        }

        let sample = &data[..data.len().min(16 * 1024)];
        let mut compiler_options = false;
        let mut paths = false;
        let mut include_or_exclude = 0usize;
        let mut feature_aliases = 0usize;

        for line in sample.split(|&byte| byte == b'\n').take(512) {
            let line = line
                .iter()
                .skip_while(|&&byte| matches!(byte, b' ' | b'\t'))
                .copied()
                .collect::<Vec<u8>>();
            let line = line.as_slice();
            if line.is_empty() {
                continue;
            }

            if line.starts_with(br#""compilerOptions":"#)
                || line.starts_with(br#""compilerOptions": {"#)
            {
                compiler_options = true;
            } else if line.starts_with(br#""paths":"#) || line.starts_with(br#""paths": {"#) {
                paths = true;
            } else if line.starts_with(br#""include":"#)
                || line.starts_with(br#""include": ["#)
                || line.starts_with(br#""exclude":"#)
                || line.starts_with(br#""exclude": ["#)
                || line.starts_with(br#""references":"#)
                || line.starts_with(br#""references": ["#)
            {
                include_or_exclude += 1;
            } else if line.starts_with(br#""@feature/"#) {
                feature_aliases += 1;
            }
        }

        compiler_options && paths && (include_or_exclude >= 1 || feature_aliases >= 8)
    }

    fn likely_code_like_short_text(data: &[u8]) -> bool {
        let mut nonempty_lines = 0usize;
        let mut semicolons = 0usize;
        let mut braces = 0usize;
        let mut current_len = 0usize;

        for &byte in data {
            match byte {
                b';' => {
                    semicolons += 1;
                    current_len += 1;
                }
                b'{' | b'}' => {
                    braces += 1;
                    current_len += 1;
                }
                b'\n' => {
                    if current_len != 0 {
                        nonempty_lines += 1;
                    }
                    current_len = 0;
                }
                b'\r' => {}
                _ => current_len += 1,
            }
        }

        if current_len != 0 {
            nonempty_lines += 1;
        }

        nonempty_lines != 0
            && (semicolons * 100 >= nonempty_lines * CODE_LIKE_SEMI_PER_100_LINES
                || braces * 100 >= nonempty_lines * CODE_LIKE_BRACES_PER_100_LINES)
    }

    fn active_window_size_for_text_kind(&self, is_text_block: bool) -> usize {
        if is_text_block {
            self.max_window_size
        } else {
            self.fast_window_size
        }
    }

    fn extend_match_backwards(
        &self,
        offset: usize,
        match_len: usize,
        context: &MatchCandidateContext<'_>,
    ) -> (usize, usize) {
        let mut start_idx = context.suffix_idx;
        let mut match_len = match_len;
        while start_idx > context.anchor_idx {
            let target_idx = start_idx - 1;
            let source_relative = target_idx as isize - offset as isize;
            let Some(source) = self
                .slice_at_relative(source_relative)
                .and_then(|source| source.first())
            else {
                break;
            };
            if *source != self.last_entry().data[target_idx] {
                break;
            }

            start_idx = target_idx;
            match_len += 1;
        }

        (start_idx, match_len)
    }

    #[cfg(test)]
    fn match_len_at_offset(&self, offset: usize, context: &MatchCandidateContext<'_>) -> usize {
        if offset == 0 {
            return 0;
        }

        let mut len = 0usize;
        while len < context.data_slice.len() {
            let source_relative = context.suffix_idx as isize + len as isize - offset as isize;
            let Some(source) = self.slice_at_relative(source_relative) else {
                break;
            };

            let target = &context.data_slice[len..];
            let matched = Self::common_prefix_len(source, target);
            len += matched;
            if matched < source.len().min(target.len()) {
                break;
            }
        }
        len
    }

    #[cfg(test)]
    fn has_min_match_at_offset(&self, offset: usize, context: &MatchCandidateContext<'_>) -> bool {
        self.verified_min_match_prefix_len(offset, context)
            .is_some()
    }

    #[inline(always)]
    fn verified_min_match_prefix_len(
        &self,
        offset: usize,
        context: &MatchCandidateContext<'_>,
    ) -> Option<usize> {
        if offset == 0 {
            return None;
        }

        let source_relative = context.suffix_idx as isize - offset as isize;
        let source = self.slice_at_relative(source_relative)?;

        if source.len() < MIN_MATCH_LEN {
            return Some(0);
        }

        if source[..MIN_MATCH_LEN] == context.data_slice[..MIN_MATCH_LEN] {
            Some(MIN_MATCH_LEN)
        } else {
            None
        }
    }

    #[inline(always)]
    fn match_len_at_offset_with_prefix(
        &self,
        offset: usize,
        context: &MatchCandidateContext<'_>,
        verified_prefix_len: usize,
    ) -> usize {
        if offset == 0 {
            return 0;
        }

        if offset <= context.suffix_idx {
            let source_start = context.suffix_idx - offset + verified_prefix_len;
            let source = &self.last_entry().data[source_start..];
            let target = &context.data_slice[verified_prefix_len..];
            return verified_prefix_len + Self::common_prefix_len(source, target);
        }

        let mut len = verified_prefix_len;
        while len < context.data_slice.len() {
            let source_relative = context.suffix_idx as isize + len as isize - offset as isize;
            let Some(source) = self.slice_at_relative(source_relative) else {
                break;
            };
            let target = &context.data_slice[len..];
            let matched = Self::common_prefix_len(source, target);
            len += matched;
            if matched < source.len().min(target.len()) {
                break;
            }
        }
        len
    }

    #[inline(always)]
    fn slice_at_relative(&self, relative_to_current: isize) -> Option<&[u8]> {
        if relative_to_current >= 0 {
            return self.last_entry().data.get(relative_to_current as usize..);
        }

        let previous_entries = self.last_entry_index();
        for entry in self.window[..previous_entries].iter().rev() {
            let start = -(entry.base_offset as isize);
            let end = start + entry.data.len() as isize;
            if (start..end).contains(&relative_to_current) {
                return Some(&entry.data[(relative_to_current - start) as usize..]);
            }
        }

        None
    }

    /// Find the common prefix length between two byte slices.
    #[inline(always)]
    fn common_prefix_len(a: &[u8], b: &[u8]) -> usize {
        Self::mismatch_chunks::<8>(a, b)
    }

    /// Find the common prefix length between two byte slices with a configurable chunk length.
    /// The chunked shape is easy for the optimizer to vectorize while staying in safe Rust.
    fn mismatch_chunks<const N: usize>(xs: &[u8], ys: &[u8]) -> usize {
        let off = core::iter::zip(xs.chunks_exact(N), ys.chunks_exact(N))
            .take_while(|(x, y)| x == y)
            .count()
            * N;
        off + core::iter::zip(&xs[off..], &ys[off..])
            .take_while(|(x, y)| x == y)
            .count()
    }

    #[inline(always)]
    fn local_offset_code(offset_value: u32) -> u8 {
        offset_value.ilog2() as u8
    }

    #[inline(always)]
    fn local_literal_length_extra_bits(len: u32) -> usize {
        match len {
            0..=15 => 0,
            16..=19 => 1,
            20..=23 => 2,
            24..=31 => 3,
            32..=47 => 4,
            48..=63 => 5,
            64..=127 => 6,
            128..=255 => 7,
            256..=511 => 8,
            512..=1023 => 9,
            1024..=2047 => 10,
            2048..=4095 => 11,
            4096..=8191 => 12,
            8192..=16383 => 13,
            16384..=32767 => 14,
            32768..=65535 => 15,
            _ => 16,
        }
    }

    #[inline(always)]
    fn local_match_length_extra_bits(len: u32) -> usize {
        match len {
            3..=34 => 0,
            35..=50 => 1,
            51..=66 => 2,
            67..=82 => 4,
            83..=98 => 4,
            99..=130 => 5,
            131..=258 => 7,
            259..=514 => 8,
            515..=1026 => 9,
            1027..=2050 => 10,
            2051..=4098 => 11,
            4099..=8194 => 12,
            8195..=16386 => 13,
            16387..=32770 => 14,
            32771..=65538 => 15,
            _ => 16,
        }
    }

    /// Process bytes and add the suffixes to the suffix store up to a specific index
    #[inline(always)]
    fn add_suffixes_till(&mut self, idx: usize) {
        if self.last_entry().data.len() < MIN_MATCH_LEN {
            return;
        }
        if !self.should_track_second_newest_for_current_entry() {
            let suffix_idx = self.suffix_idx;
            let last_entry = self.last_entry_mut();
            let slice = &last_entry.data[suffix_idx..idx];
            for (key_index, key) in slice.windows(MIN_MATCH_LEN).enumerate() {
                last_entry.suffixes.insert(key, suffix_idx + key_index);
            }
            return;
        }

        for insert_idx in self.suffix_idx..idx {
            self.add_suffix_at(insert_idx);
        }
    }

    #[cfg(test)]
    #[inline(always)]
    fn add_suffixes_for_match(&mut self, idx: usize) {
        self.add_suffixes_for_match_with_dense_limit(idx, DENSE_MATCH_INDEX_LIMIT);
    }

    #[inline(always)]
    fn add_suffixes_for_match_with_dense_limit(&mut self, idx: usize, dense_limit: usize) {
        if idx - self.suffix_idx <= dense_limit {
            self.add_suffixes_till(idx);
            return;
        }

        let suffix_idx = self.suffix_idx;
        self.add_suffix_at(suffix_idx);
        self.add_suffix_at(suffix_idx + 2);
        if self.use_complementary_end_insertion_for_current_block() {
            self.add_suffix_at(idx.saturating_sub(1));
        }
        self.add_suffix_at(idx.saturating_sub(SPARSE_MATCH_END_INDEX_BACKOFF));
    }

    #[inline(always)]
    fn add_suffixes_for_sparse_best_match(&mut self, idx: usize) {
        let suffix_idx = self.suffix_idx;
        self.add_suffix_at(suffix_idx);
        self.add_suffix_at(suffix_idx + 2);
        if self.use_complementary_end_insertion_for_current_block() {
            self.add_suffix_at(idx.saturating_sub(1));
        }
        self.add_suffix_at(idx.saturating_sub(SPARSE_MATCH_END_INDEX_BACKOFF));
    }

    #[inline(always)]
    fn emit_candidate(
        &mut self,
        candidate: MatchCandidate,
        handle_sequence: &mut impl for<'a> FnMut(Sequence<'a>),
    ) {
        let MatchCandidate {
            start_idx,
            offset,
            match_len,
            repeat_offset,
            ..
        } = candidate;
        let literals_empty = start_idx == self.last_idx_in_sequence;
        let sparse_match = start_idx + match_len - self.suffix_idx > DENSE_MATCH_INDEX_LIMIT;
        if sparse_match && start_idx > self.suffix_idx {
            self.add_suffix_at(start_idx);
        }
        if self.use_complementary_end_insertion_for_current_block()
            && literals_empty
            && repeat_offset
        {
            self.add_suffixes_for_sparse_best_match(start_idx + match_len);
        } else {
            self.add_suffixes_for_match_with_dense_limit(
                start_idx + match_len,
                DENSE_MATCH_INDEX_LIMIT,
            );
        }

        let last_entry_idx = self.last_entry_index();
        let last_entry = &self.window[last_entry_idx];
        let literals = &last_entry.data[self.last_idx_in_sequence..start_idx];
        #[cfg(test)]
        self.diagnostics
            .borrow_mut()
            .record(candidate, literals_empty);
        let offset_value = Self::bounded_u32(offset);
        self.offset_history
            .update_after_match(offset_value, !literals_empty);

        self.suffix_idx = start_idx + match_len;
        self.last_idx_in_sequence = self.suffix_idx;
        handle_sequence(Sequence::Triple {
            literals,
            offset,
            match_len,
        });
    }

    #[inline(always)]
    fn add_suffix_at(&mut self, idx: usize) {
        let key_value = {
            let last_entry = self.last_entry();
            let Some(key) = last_entry.data.get(idx..idx + MIN_MATCH_LEN) else {
                return;
            };
            SuffixStore::key_value(key)
        };
        let track_second_newest = self.should_track_second_newest_for_current_entry();
        let track_third_newest = self.uses_dictionary_lockfile_second_newest_path();
        let sidecar_update = if track_second_newest {
            let last_entry = self.last_entry();
            let slot_index = last_entry.suffixes.slot_key(key_value).index;
            let previous_newest = last_entry.suffixes.slots[slot_index]
                .and_then(|slot| (slot.oldest != slot.newest).then_some(slot.newest));
            let previous_second_newest = if track_third_newest {
                self.current_second_newest_sidecar[slot_index]
            } else {
                None
            };
            let previous_third_newest = if track_third_newest {
                self.current_third_newest_sidecar[slot_index]
            } else {
                None
            };
            let previous_fourth_newest = if track_third_newest {
                self.current_fourth_newest_sidecar[slot_index]
            } else {
                None
            };
            let previous_fifth_newest = if track_third_newest {
                self.current_fifth_newest_sidecar[slot_index]
            } else {
                None
            };
            let previous_sixth_newest = if track_third_newest {
                self.current_sixth_newest_sidecar[slot_index]
            } else {
                None
            };
            let previous_seventh_newest = if track_third_newest {
                self.current_seventh_newest_sidecar[slot_index]
            } else {
                None
            };
            let previous_eighth_newest = if track_third_newest {
                self.current_eighth_newest_sidecar[slot_index]
            } else {
                None
            };
            let previous_ninth_newest = if track_third_newest {
                self.current_ninth_newest_sidecar[slot_index]
            } else {
                None
            };
            let previous_tenth_newest = if track_third_newest {
                self.current_tenth_newest_sidecar[slot_index]
            } else {
                None
            };
            let previous_eleventh_newest = if track_third_newest {
                self.current_eleventh_newest_sidecar[slot_index]
            } else {
                None
            };
            let previous_twelfth_newest = if track_third_newest {
                self.current_twelfth_newest_sidecar[slot_index]
            } else {
                None
            };
            Some((
                slot_index,
                previous_newest,
                previous_second_newest,
                previous_third_newest,
                previous_fourth_newest,
                previous_fifth_newest,
                previous_sixth_newest,
                previous_seventh_newest,
                previous_eighth_newest,
                previous_ninth_newest,
                previous_tenth_newest,
                previous_eleventh_newest,
                previous_twelfth_newest,
            ))
        } else {
            None
        };
        let long_hash_update = if self.should_track_current_long_hash() {
            let last_entry = self.last_entry();
            last_entry.data.get(idx..idx + 8).and_then(|key| {
                let key = u64::from_le_bytes(key.try_into().ok()?);
                Some((
                    last_entry.suffixes.slot_key(key).index,
                    SuffixStore::stored_index(idx),
                ))
            })
        } else {
            None
        };
        if let Some((
            slot_index,
            previous_newest,
            previous_second_newest,
            previous_third_newest,
            previous_fourth_newest,
            previous_fifth_newest,
            previous_sixth_newest,
            previous_seventh_newest,
            previous_eighth_newest,
            previous_ninth_newest,
            previous_tenth_newest,
            previous_eleventh_newest,
            previous_twelfth_newest,
        )) = sidecar_update
        {
            self.current_second_newest_sidecar[slot_index] = previous_newest;
            if track_third_newest {
                self.current_third_newest_sidecar[slot_index] = previous_second_newest;
                self.current_fourth_newest_sidecar[slot_index] = previous_third_newest;
                self.current_fifth_newest_sidecar[slot_index] = previous_fourth_newest;
                self.current_sixth_newest_sidecar[slot_index] = previous_fifth_newest;
                self.current_seventh_newest_sidecar[slot_index] = previous_sixth_newest;
                self.current_eighth_newest_sidecar[slot_index] = previous_seventh_newest;
                self.current_ninth_newest_sidecar[slot_index] = previous_eighth_newest;
                self.current_tenth_newest_sidecar[slot_index] = previous_ninth_newest;
                self.current_eleventh_newest_sidecar[slot_index] = previous_tenth_newest;
                self.current_twelfth_newest_sidecar[slot_index] = previous_eleventh_newest;
                self.current_thirteenth_newest_sidecar[slot_index] = previous_twelfth_newest;
            }
        }
        if let Some((slot_index, stored_index)) = long_hash_update {
            self.current_long_hash[slot_index] = Some(stored_index);
        }
        let last_entry = self.last_entry_mut();
        let Some(key) = last_entry.data.get(idx..idx + MIN_MATCH_LEN) else {
            return;
        };
        last_entry.suffixes.insert(key, idx);
    }

    /// Skip matching for the whole current window entry
    fn skip_matching(&mut self) {
        let len = self.last_entry().data.len();
        self.add_suffixes_till(len);
        self.suffix_idx = len;
        self.last_idx_in_sequence = len;
    }

    fn skip_matching_for_incompressible(&mut self) {
        let len = self.last_entry().data.len();
        self.suffix_idx = len;
        self.last_idx_in_sequence = len;
    }

    fn skip_matching_for_rle(&mut self) {
        let len = self.last_entry().data.len();
        if len >= MIN_MATCH_LEN {
            let first_suffix = self.suffix_idx;
            self.add_suffix_at(first_suffix);
            let last_suffix = len - MIN_MATCH_LEN;
            if last_suffix != first_suffix {
                self.add_suffix_at(last_suffix);
            }
        }
        self.suffix_idx = len;
        self.last_idx_in_sequence = len;
    }

    /// Add a new window entry. Will panic if the last window entry hasn't been processed properly.
    /// If any resources are released by pushing the new entry they are returned via the callback
    fn add_data(
        &mut self,
        data: Vec<u8>,
        suffixes: SuffixStore,
        reuse_space: impl FnMut(Vec<u8>, SuffixStore),
    ) {
        assert!(self.window.is_empty() || self.suffix_idx == self.last_entry().data.len());
        assert!(data.len() <= u32::MAX as usize);
        let len = data.len();
        let is_text_block = Self::likely_text(&data);
        let is_short_line_text = is_text_block && Self::likely_short_line_text(&data);
        let min_non_repeat_match_len = self.min_non_repeat_match_len_for_text(&data, is_text_block);
        let active_window_size = self.active_window_size_for_text_kind(is_text_block);
        self.reserve(data.len(), active_window_size, reuse_space);
        #[cfg(debug_assertions)]
        self.concat_window.extend_from_slice(&data);

        if let Some(last_len) = self.window.last().map(|last| last.data.len()) {
            for entry in self.window.iter_mut() {
                entry.base_offset += last_len;
            }
        }

        self.window.push(WindowEntry {
            data,
            suffixes,
            base_offset: 0,
        });
        let dictionary_lockfile_text =
            matches!(self.file_type_hint, CompressionFileType::DictionaryText)
                && is_text_block
                && min_non_repeat_match_len == MIN_MATCH_LEN
                && (matches!(self.file_profile_hint, CompressionFileProfile::CargoLock)
                    || Self::likely_lockfile_text(&self.last_entry().data));
        let composer_dictionary_text =
            matches!(self.file_type_hint, CompressionFileType::DictionaryText)
                && is_text_block
                && is_short_line_text
                && (matches!(self.file_profile_hint, CompressionFileProfile::ComposerLock)
                    || Self::likely_composer_dictionary_text(&self.last_entry().data));
        self.current_block_is_dictionary_lockfile_text = dictionary_lockfile_text;
        self.current_block_is_composer_dictionary_text =
            composer_dictionary_text && !dictionary_lockfile_text;
        self.current_block_is_structured_json_config_text =
            matches!(self.file_type_hint, CompressionFileType::ConfigText)
                && is_text_block
                && is_short_line_text
                && Self::likely_structured_json_config_text(&self.last_entry().data);
        self.current_block_is_tsconfig_json_config_text =
            matches!(self.file_type_hint, CompressionFileType::ConfigText)
                && is_text_block
                && is_short_line_text
                && Self::likely_tsconfig_json_config_text(&self.last_entry().data);
        let dictionary_current_entry_second_newest =
            matches!(self.file_type_hint, CompressionFileType::DictionaryText)
                && is_text_block
                && min_non_repeat_match_len == MIN_MATCH_LEN
                && len >= BEST_SECOND_NEWEST_MIN_BLOCK_LEN;
        let code_text_current_entry_second_newest =
            matches!(self.file_type_hint, CompressionFileType::CodeText)
                && is_short_line_text
                && (BEST_SECOND_NEWEST_MIN_BLOCK_LEN..=CODE_TEXT_SECOND_NEWEST_MAX_BLOCK_LEN)
                    .contains(&len);
        let track_second_newest = (self.use_second_newest_probe
            && min_non_repeat_match_len == MIN_MATCH_LEN
            && len >= BEST_SECOND_NEWEST_MIN_BLOCK_LEN)
            || dictionary_current_entry_second_newest
            || code_text_current_entry_second_newest
            || dictionary_lockfile_text
            || (self.use_fast_binary_small_second_newest
                && !is_text_block
                && min_non_repeat_match_len == MIN_MATCH_LEN
                && (len <= FASTEST_SECOND_NEWEST_MAX_BLOCK_LEN
                    || (matches!(self.file_type_hint, CompressionFileType::Unknown)
                        && len <= FASTEST_UNKNOWN_SECOND_NEWEST_MAX_BLOCK_LEN)));
        if track_second_newest {
            let sidecar_len = self.last_entry().suffixes.slots.len();
            self.current_second_newest_sidecar = alloc::vec![None; sidecar_len];
            if dictionary_lockfile_text {
                self.current_third_newest_sidecar = alloc::vec![None; sidecar_len];
                self.current_fourth_newest_sidecar = alloc::vec![None; sidecar_len];
                self.current_fifth_newest_sidecar = alloc::vec![None; sidecar_len];
                self.current_sixth_newest_sidecar = alloc::vec![None; sidecar_len];
                self.current_seventh_newest_sidecar = alloc::vec![None; sidecar_len];
                self.current_eighth_newest_sidecar = alloc::vec![None; sidecar_len];
                self.current_ninth_newest_sidecar = alloc::vec![None; sidecar_len];
                self.current_tenth_newest_sidecar = alloc::vec![None; sidecar_len];
                self.current_eleventh_newest_sidecar = alloc::vec![None; sidecar_len];
                self.current_twelfth_newest_sidecar = alloc::vec![None; sidecar_len];
                self.current_thirteenth_newest_sidecar = alloc::vec![None; sidecar_len];
            } else {
                self.current_third_newest_sidecar.clear();
                self.current_fourth_newest_sidecar.clear();
                self.current_fifth_newest_sidecar.clear();
                self.current_sixth_newest_sidecar.clear();
                self.current_seventh_newest_sidecar.clear();
                self.current_eighth_newest_sidecar.clear();
                self.current_ninth_newest_sidecar.clear();
                self.current_tenth_newest_sidecar.clear();
                self.current_eleventh_newest_sidecar.clear();
                self.current_twelfth_newest_sidecar.clear();
                self.current_thirteenth_newest_sidecar.clear();
            }
            if self.should_track_current_long_hash() {
                self.current_long_hash = alloc::vec![None; sidecar_len];
            } else {
                self.current_long_hash.clear();
            }
        } else {
            self.current_second_newest_sidecar.clear();
            self.current_third_newest_sidecar.clear();
            self.current_fourth_newest_sidecar.clear();
            self.current_fifth_newest_sidecar.clear();
            self.current_sixth_newest_sidecar.clear();
            self.current_seventh_newest_sidecar.clear();
            self.current_eighth_newest_sidecar.clear();
            self.current_ninth_newest_sidecar.clear();
            self.current_tenth_newest_sidecar.clear();
            self.current_eleventh_newest_sidecar.clear();
            self.current_twelfth_newest_sidecar.clear();
            self.current_thirteenth_newest_sidecar.clear();
            self.current_long_hash.clear();
        }
        let last_suffix_len_log = self.last_entry().suffixes.len_log;
        self.uniform_suffix_len_log = match (self.uniform_suffix_len_log, self.window.len()) {
            (_, 1) => Some(last_suffix_len_log),
            (Some(len_log), _) if len_log == last_suffix_len_log => Some(len_log),
            _ => None,
        };
        self.window_size += len;
        self.suffix_idx = 0;
        self.last_idx_in_sequence = 0;
        self.is_text_block = is_text_block;
        self.is_short_line_text = is_short_line_text;
        self.min_non_repeat_match_len = min_non_repeat_match_len;
    }

    /// Reserve space for a new window entry
    /// If any resources are released by pushing the new entry they are returned via the callback
    fn reserve(
        &mut self,
        amount: usize,
        max_window_size: usize,
        mut reuse_space: impl FnMut(Vec<u8>, SuffixStore),
    ) {
        assert!(self.max_window_size >= amount);
        assert!(max_window_size >= amount);
        while self.window_size + amount > max_window_size {
            let removed = self.window.remove(0);
            self.window_size -= removed.data.len();
            #[cfg(debug_assertions)]
            self.concat_window.drain(0..removed.data.len());

            let WindowEntry {
                suffixes,
                data: leaked_vec,
                base_offset: _,
            } = removed;
            reuse_space(leaked_vec, suffixes);
        }
    }
}

#[test]
fn suffix_store_slot_stays_compact() {
    assert_eq!(core::mem::size_of::<Candidates>(), 8);
    assert_eq!(core::mem::size_of::<Option<Candidates>>(), 8);
}

#[test]
fn suffix_store_reports_single_candidate_once() {
    let mut suffixes = SuffixStore::with_capacity(64);

    suffixes.insert(b"abcde", 7);

    let candidates = suffixes
        .candidates(b"abcde")
        .expect("candidate should exist");
    assert_eq!(candidates.oldest, 7);
    assert_eq!(candidates.newest, None);
}

#[test]
fn suffix_store_round_trips_zero_index_candidate() {
    let mut suffixes = SuffixStore::with_capacity(64);

    suffixes.insert(b"abcde", 0);

    let candidates = suffixes
        .candidates(b"abcde")
        .expect("zero index candidate should exist");
    assert_eq!(candidates.oldest, 0);
    assert_eq!(candidates.newest, None);
}

#[test]
fn suffix_store_preserves_oldest_and_latest_candidates() {
    let mut suffixes = SuffixStore::with_capacity(64);

    suffixes.insert(b"abcde", 3);
    suffixes.insert(b"abcde", 8);
    suffixes.insert(b"abcde", 15);

    let candidates = suffixes
        .candidates(b"abcde")
        .expect("candidate should exist");
    assert_eq!(candidates.oldest, 3);
    assert_eq!(candidates.newest, Some(15));
}

#[test]
fn suffix_store_clear_removes_touched_candidates() {
    let mut suffixes = SuffixStore::with_capacity(64);

    suffixes.insert(b"abcde", 3);
    suffixes.insert(b"fghij", 8);
    suffixes.clear();

    assert!(suffixes.candidates(b"abcde").is_none());
    assert!(suffixes.candidates(b"fghij").is_none());
}

#[test]
fn suffix_store_reuses_slots_after_clear() {
    let mut suffixes = SuffixStore::with_capacity(64);

    suffixes.insert(b"abcde", 3);
    suffixes.clear();
    suffixes.insert(b"abcde", 9);

    let candidates = suffixes
        .candidates(b"abcde")
        .expect("candidate should exist after reinsertion");
    assert_eq!(candidates.oldest, 9);
    assert_eq!(candidates.newest, None);
}

#[test]
fn suffix_store_full_clear_mode_removes_untracked_candidates() {
    let mut suffixes = SuffixStore::with_capacity(64);
    let stored = NonZeroU32::new(1).expect("one is non-zero");
    suffixes.slots[1] = Some(Candidates {
        oldest: stored,
        newest: stored,
    });
    suffixes.clear_all_slots = true;

    suffixes.clear();

    assert!(suffixes.slots.iter().all(Option::is_none));
    assert!(!suffixes.clear_all_slots);
    assert!(suffixes.touched_slots.is_empty());
}

#[test]
fn suffix_store_switches_to_full_clear_after_many_touched_slots() {
    let mut suffixes = SuffixStore::with_capacity(TOUCHED_SLOT_CLEAR_LIMIT + 1);

    for key in 0..TOUCHED_SLOT_CLEAR_LIMIT {
        suffixes.record_touched_slot(key);
    }
    assert!(!suffixes.clear_all_slots);

    suffixes.record_touched_slot(TOUCHED_SLOT_CLEAR_LIMIT);

    assert!(suffixes.clear_all_slots);
    assert!(suffixes.touched_slots.is_empty());
}

#[test]
fn suffix_store_preallocates_touched_slots_modestly() {
    let suffixes = SuffixStore::with_capacity(64);

    assert_eq!(
        suffixes.touched_slots.capacity(),
        INITIAL_TOUCHED_SLOT_CAPACITY
    );
    assert!(suffixes.touched_slots.capacity() < TOUCHED_SLOT_CLEAR_LIMIT);
}

#[test]
fn suffix_store_handles_zero_capacity_request() {
    let mut suffixes = SuffixStore::with_capacity(0);

    suffixes.insert(b"abcde", 0);

    let candidates = suffixes
        .candidates(b"abcde")
        .expect("candidate should exist with minimum backing storage");
    assert_eq!(candidates.oldest, 0);
    assert_eq!(candidates.newest, None);
}

#[test]
fn suffix_store_key_is_bounded_without_modulo() {
    let suffixes = SuffixStore::with_capacity(100);

    for suffix in [
        b"abcde".as_slice(),
        b"vwxyz".as_slice(),
        b"12345".as_slice(),
        b"\xff\xff\xff\xff\xff".as_slice(),
    ] {
        assert!(suffixes.key(suffix) < suffixes.slots.len());
    }
}

#[test]
fn suffix_store_reuses_precomputed_key_values_for_lookup() {
    let mut suffixes = SuffixStore::with_capacity(64);

    suffixes.insert(b"abcde", 11);
    let key_value = SuffixStore::key_value(b"abcde");
    let candidates = suffixes
        .candidates_for_key_value(key_value)
        .expect("candidate should exist for precomputed key");

    assert_eq!(suffixes.key(b"abcde"), suffixes.key_from_value(key_value));
    assert_eq!(candidates.oldest, 11);
    assert_eq!(candidates.newest, None);
    assert!(suffixes
        .candidates_for_key_value(SuffixStore::key_value(b"vwxyz"))
        .is_none());
}

#[test]
fn suffix_store_reuses_precomputed_slot_keys_for_lookup() {
    let mut suffixes = SuffixStore::with_capacity(64);

    suffixes.insert(b"abcde", 11);
    let slot_key = suffixes.slot_key(SuffixStore::key_value(b"abcde"));
    let candidates = suffixes
        .candidates_for_slot_key(slot_key)
        .expect("candidate should exist for precomputed slot key");

    assert_eq!(candidates.oldest, 11);
    assert_eq!(candidates.newest, None);
}

#[test]
fn suffix_store_slot_keys_are_not_reused_across_different_capacities() {
    let mut small = SuffixStore::with_capacity(64);
    let mut large = SuffixStore::with_capacity(128);

    small.insert(b"abcde", 11);
    large.insert(b"abcde", 29);

    assert_ne!(small.len_log, large.len_log);

    let large_candidates = large
        .candidates_for_key_value(SuffixStore::key_value(b"abcde"))
        .expect("capacity-aware lookup should still find candidate");

    assert_eq!(large_candidates.oldest, 29);
    assert_eq!(large_candidates.newest, None);
}

#[test]
fn match_len_extends_overlapping_same_block() {
    let mut matcher = MatchGenerator::new(100);
    matcher.add_data(
        alloc::vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        SuffixStore::with_capacity(100),
        |_, _| {},
    );

    let last_entry = matcher.last_entry();
    let context = MatchCandidateContext {
        suffix_idx: 1,
        anchor_idx: 0,
        min_non_repeat_match_len: MIN_MATCH_LEN,
        data_slice: &last_entry.data[1..],
        #[cfg(debug_assertions)]
        last_entry_len: last_entry.data.len(),
        #[cfg(debug_assertions)]
        concat_window: &matcher.concat_window,
    };

    assert_eq!(matcher.match_len_at_offset(1, &context), 9);
}

#[test]
fn match_len_stops_at_chunk_boundary_mismatch() {
    let mut matcher = MatchGenerator::new(100);
    matcher.add_data(
        b"abcdefghabcdefghZ".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );

    let last_entry = matcher.last_entry();
    let context = MatchCandidateContext {
        suffix_idx: 8,
        anchor_idx: 0,
        min_non_repeat_match_len: MIN_MATCH_LEN,
        data_slice: &last_entry.data[8..],
        #[cfg(debug_assertions)]
        last_entry_len: last_entry.data.len(),
        #[cfg(debug_assertions)]
        concat_window: &matcher.concat_window,
    };

    assert_eq!(matcher.match_len_at_offset(8, &context), 8);
}

#[test]
fn match_len_reads_from_previous_window_entry() {
    let mut matcher = MatchGenerator::new(100);
    matcher.add_data(
        b"prefix_MATCHTAIL".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );
    matcher.skip_matching();
    matcher.add_data(
        b"MATCHTAILx".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );

    let last_entry = matcher.last_entry();
    let context = MatchCandidateContext {
        suffix_idx: 0,
        anchor_idx: 0,
        min_non_repeat_match_len: MIN_MATCH_LEN,
        data_slice: &last_entry.data,
        #[cfg(debug_assertions)]
        last_entry_len: last_entry.data.len(),
        #[cfg(debug_assertions)]
        concat_window: &matcher.concat_window,
    };

    assert_eq!(matcher.match_len_at_offset(b"MATCHTAIL".len(), &context), 9);
}

#[test]
fn verified_min_match_prefix_skips_rechecked_bytes() {
    let mut matcher = MatchGenerator::new(100);
    matcher.add_data(
        b"abcdefghabcdefghZ".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );

    let last_entry = matcher.last_entry();
    let context = MatchCandidateContext {
        suffix_idx: 8,
        anchor_idx: 0,
        min_non_repeat_match_len: MIN_MATCH_LEN,
        data_slice: &last_entry.data[8..],
        #[cfg(debug_assertions)]
        last_entry_len: last_entry.data.len(),
        #[cfg(debug_assertions)]
        concat_window: &matcher.concat_window,
    };

    assert_eq!(
        matcher.verified_min_match_prefix_len(8, &context),
        Some(MIN_MATCH_LEN)
    );
    assert_eq!(
        matcher.match_len_at_offset_with_prefix(8, &context, MIN_MATCH_LEN),
        matcher.match_len_at_offset(8, &context)
    );
}

#[test]
fn same_block_min_match_precheck_handles_hits_and_misses() {
    let mut matcher = MatchGenerator::new(100);
    matcher.add_data(
        b"abcdeXabcdeYabcdnZ".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );

    let last_entry = matcher.last_entry();
    let matching_context = MatchCandidateContext {
        suffix_idx: 6,
        anchor_idx: 0,
        min_non_repeat_match_len: MIN_MATCH_LEN,
        data_slice: &last_entry.data[6..],
        #[cfg(debug_assertions)]
        last_entry_len: last_entry.data.len(),
        #[cfg(debug_assertions)]
        concat_window: &matcher.concat_window,
    };
    let mismatching_context = MatchCandidateContext {
        suffix_idx: 12,
        anchor_idx: 0,
        min_non_repeat_match_len: MIN_MATCH_LEN,
        data_slice: &last_entry.data[12..],
        #[cfg(debug_assertions)]
        last_entry_len: last_entry.data.len(),
        #[cfg(debug_assertions)]
        concat_window: &matcher.concat_window,
    };

    assert_eq!(
        matcher.verified_min_match_prefix_len(6, &matching_context),
        Some(MIN_MATCH_LEN)
    );
    assert!(matcher.has_min_match_at_index_offset(6, 6));
    assert_eq!(
        matcher.verified_min_match_prefix_len(6, &mismatching_context),
        None
    );
    assert!(!matcher.has_min_match_at_index_offset(12, 6));
}

#[test]
fn same_block_prefix_fast_path_handles_overlap() {
    let mut matcher = MatchGenerator::new(100);
    matcher.add_data(
        alloc::vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        SuffixStore::with_capacity(100),
        |_, _| {},
    );

    let last_entry = matcher.last_entry();
    let context = MatchCandidateContext {
        suffix_idx: 1,
        anchor_idx: 0,
        min_non_repeat_match_len: MIN_MATCH_LEN,
        data_slice: &last_entry.data[1..],
        #[cfg(debug_assertions)]
        last_entry_len: last_entry.data.len(),
        #[cfg(debug_assertions)]
        concat_window: &matcher.concat_window,
    };

    assert_eq!(
        matcher.match_len_at_offset_with_prefix(1, &context, MIN_MATCH_LEN),
        matcher.match_len_at_offset(1, &context)
    );
}

#[test]
fn short_previous_entry_prefix_falls_back_to_full_match_scan() {
    let mut matcher = MatchGenerator::new(100);
    matcher.add_data(b"ATCH".to_vec(), SuffixStore::with_capacity(100), |_, _| {});
    matcher.skip_matching();
    matcher.add_data(
        b"MATCHTAIL".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );

    let last_entry = matcher.last_entry();
    let context = MatchCandidateContext {
        suffix_idx: 0,
        anchor_idx: 0,
        min_non_repeat_match_len: MIN_MATCH_LEN,
        data_slice: &last_entry.data,
        #[cfg(debug_assertions)]
        last_entry_len: last_entry.data.len(),
        #[cfg(debug_assertions)]
        concat_window: &matcher.concat_window,
    };

    assert_eq!(matcher.verified_min_match_prefix_len(1, &context), Some(0));
    assert_eq!(
        matcher.match_len_at_offset_with_prefix(1, &context, 0),
        matcher.match_len_at_offset(1, &context)
    );
}

#[test]
fn match_len_reads_from_most_recent_previous_window_entry() {
    let mut matcher = MatchGenerator::new(100);
    matcher.add_data(
        b"older_MATCH".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );
    matcher.skip_matching();
    matcher.add_data(
        b"recent_MATCH".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );
    matcher.skip_matching();
    matcher.add_data(
        b"MATCH!".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );

    let last_entry = matcher.last_entry();
    let context = MatchCandidateContext {
        suffix_idx: 0,
        anchor_idx: 0,
        min_non_repeat_match_len: MIN_MATCH_LEN,
        data_slice: &last_entry.data,
        #[cfg(debug_assertions)]
        last_entry_len: last_entry.data.len(),
        #[cfg(debug_assertions)]
        concat_window: &matcher.concat_window,
    };

    assert_eq!(matcher.match_len_at_offset(b"MATCH".len(), &context), 5);
}

#[test]
fn repeat_offset_precheck_rejects_obvious_miss() {
    let mut matcher = MatchGenerator::new(100);
    matcher.add_data(
        b"abcde-----vwxyz".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );

    let last_entry = matcher.last_entry();
    let context = MatchCandidateContext {
        suffix_idx: 10,
        anchor_idx: 0,
        min_non_repeat_match_len: MIN_MATCH_LEN,
        data_slice: &last_entry.data[10..],
        #[cfg(debug_assertions)]
        last_entry_len: last_entry.data.len(),
        #[cfg(debug_assertions)]
        concat_window: &matcher.concat_window,
    };

    assert!(!matcher.has_min_match_at_offset(10, &context));
}

#[test]
fn repeat_offset_precheck_accepts_candidate_match() {
    let mut matcher = MatchGenerator::new(100);
    matcher.add_data(
        b"abcde-----abcde".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );

    let last_entry = matcher.last_entry();
    let context = MatchCandidateContext {
        suffix_idx: 10,
        anchor_idx: 0,
        min_non_repeat_match_len: MIN_MATCH_LEN,
        data_slice: &last_entry.data[10..],
        #[cfg(debug_assertions)]
        last_entry_len: last_entry.data.len(),
        #[cfg(debug_assertions)]
        concat_window: &matcher.concat_window,
    };

    assert!(matcher.has_min_match_at_offset(10, &context));
}

#[test]
fn hash_candidate_precheck_rejects_collision() {
    let mut matcher = MatchGenerator::new(100);
    matcher.add_data(
        b"abcde-----vwxyz".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );

    let last_entry = matcher.last_entry();
    let context = MatchCandidateContext {
        suffix_idx: 10,
        anchor_idx: 0,
        min_non_repeat_match_len: MIN_MATCH_LEN,
        data_slice: &last_entry.data[10..],
        #[cfg(debug_assertions)]
        last_entry_len: last_entry.data.len(),
        #[cfg(debug_assertions)]
        concat_window: &matcher.concat_window,
    };

    assert!(!MatchGenerator::has_min_match_at_index(
        last_entry, 0, &context
    ));
}

#[test]
fn hash_candidate_precheck_accepts_candidate_match() {
    let mut matcher = MatchGenerator::new(100);
    matcher.add_data(
        b"abcde-----abcde".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );

    let last_entry = matcher.last_entry();
    let context = MatchCandidateContext {
        suffix_idx: 10,
        anchor_idx: 0,
        min_non_repeat_match_len: MIN_MATCH_LEN,
        data_slice: &last_entry.data[10..],
        #[cfg(debug_assertions)]
        last_entry_len: last_entry.data.len(),
        #[cfg(debug_assertions)]
        concat_window: &matcher.concat_window,
    };

    assert!(MatchGenerator::has_min_match_at_index(
        last_entry, 0, &context
    ));
}

#[test]
fn window_candidate_helper_updates_best_candidate() {
    let mut matcher = MatchGenerator::new(100);
    matcher.add_data(
        b"abcdefghabcdefghZ".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );

    let last_entry = matcher.last_entry();
    let context = MatchCandidateContext {
        suffix_idx: 8,
        anchor_idx: 0,
        min_non_repeat_match_len: MIN_MATCH_LEN,
        data_slice: &last_entry.data[8..],
        #[cfg(debug_assertions)]
        last_entry_len: last_entry.data.len(),
        #[cfg(debug_assertions)]
        concat_window: &matcher.concat_window,
    };
    let mut candidate = Some(MatchCandidate {
        start_idx: 8,
        offset: 16,
        match_len: MIN_MATCH_LEN,
        repeat_offset: false,
        #[cfg(test)]
        source: CandidateSource::WindowCurrentNewest { entry_distance: 0 },
    });

    assert!(!matcher.consider_window_candidate(
        last_entry,
        0,
        &context,
        WindowCandidateMeta {
            entry_distance: 0,
            kind: WindowCandidateKind::Newest,
        },
        &mut candidate,
        last_entry.data.len(),
    ));
    assert_eq!(candidate.map(|candidate| candidate.match_len), Some(8));
}

#[test]
fn window_candidate_helper_stops_on_block_end_match() {
    let mut matcher = MatchGenerator::new(100);
    matcher.add_data(
        alloc::vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        SuffixStore::with_capacity(100),
        |_, _| {},
    );

    let last_entry = matcher.last_entry();
    let context = MatchCandidateContext {
        suffix_idx: 2,
        anchor_idx: 0,
        min_non_repeat_match_len: MIN_MATCH_LEN,
        data_slice: &last_entry.data[2..],
        #[cfg(debug_assertions)]
        last_entry_len: last_entry.data.len(),
        #[cfg(debug_assertions)]
        concat_window: &matcher.concat_window,
    };
    let mut candidate = None;

    assert!(matcher.consider_window_candidate(
        last_entry,
        0,
        &context,
        WindowCandidateMeta {
            entry_distance: 0,
            kind: WindowCandidateKind::Newest,
        },
        &mut candidate,
        last_entry.data.len(),
    ));
    assert_eq!(candidate.map(|candidate| candidate.offset), Some(2));
}

#[test]
fn window_search_uses_newest_block_end_candidate() {
    let mut matcher = MatchGenerator::new(100);
    matcher.add_data(
        b"abcde?".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );
    matcher.skip_matching();
    matcher.add_data(
        b"abcde!".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );
    matcher.skip_matching();
    matcher.add_data(
        b"abcde!".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );

    matcher.next_sequence(|seq| {
        assert_eq!(
            seq,
            Sequence::Triple {
                literals: &[],
                offset: 6,
                match_len: 6,
            },
        );
    });
    assert!(!matcher.next_sequence(|_| {}));
}

#[test]
fn repeat_offset_probe_finds_match_without_suffix_index() {
    let mut matcher = MatchGenerator::new(100);
    matcher.add_data(
        b"MATCHTAIL".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );
    matcher.skip_matching_for_incompressible();
    matcher.offset_history = OffsetHistory::from_offsets(10, 4, 8);
    matcher.add_data(
        b"xMATCHTAIL".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );

    matcher.next_sequence(|seq| {
        assert_eq!(
            seq,
            Sequence::Triple {
                literals: b"x",
                offset: 10,
                match_len: 9,
            },
        );
    });
    assert!(!matcher.next_sequence(|_| {}));
    assert_eq!(matcher.offset_history.as_offsets(), (10, 4, 8));
}

#[test]
fn repeat_offset_candidates_keep_history_order_after_literals() {
    let mut matcher = MatchGenerator::new(100);
    matcher.offset_history = OffsetHistory::from_offsets(7, 11, 13);

    assert_eq!(
        matcher.repeat_offset_candidates(3),
        [
            (RepeatCandidateKind::First, 7),
            (RepeatCandidateKind::Second, 11),
            (RepeatCandidateKind::Third, 13),
        ]
    );
}

#[test]
fn repeat_offset_candidates_shift_for_zero_literals() {
    let mut matcher = MatchGenerator::new(100);
    matcher.offset_history = OffsetHistory::from_offsets(7, 11, 13);

    assert_eq!(
        matcher.repeat_offset_candidates(0),
        [
            (RepeatCandidateKind::Second, 11),
            (RepeatCandidateKind::Third, 13),
            (RepeatCandidateKind::First, 6),
        ]
    );
}

#[test]
fn match_candidate_extends_backwards_to_anchor() {
    let mut matcher = MatchGenerator::new(100);
    matcher.add_data(
        b"XabcdeXabcde".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );

    matcher.next_sequence(|seq| {
        assert_eq!(
            seq,
            Sequence::Triple {
                literals: b"Xabcde",
                offset: 6,
                match_len: 6,
            },
        );
    });
    assert!(!matcher.next_sequence(|_| {}));
}

#[test]
fn small_text_blocks_use_shorter_non_repeat_match_minimum() {
    let mut matcher = MatchGenerator::new(2048);
    matcher.add_data(
        b"tenant=alpha path=/v1/archive status=200\n"
            .repeat(32)
            .to_vec(),
        SuffixStore::with_capacity(2048),
        |_, _| {},
    );

    assert_eq!(
        matcher.min_non_repeat_match_len,
        SMALL_TEXT_MIN_NON_REPEAT_MATCH_LEN
    );
}

#[test]
fn large_short_line_text_blocks_use_shorter_non_repeat_match_minimum() {
    let mut matcher = MatchGenerator::new(512 * 1024);
    matcher.add_data(
        b"let alpha = archive_status(path, tenant);\n"
            .repeat((128 * 1024 / 40) + 512)
            .to_vec(),
        SuffixStore::with_capacity(512 * 1024),
        |_, _| {},
    );

    assert_eq!(
        matcher.min_non_repeat_match_len,
        CODE_LIKE_SHORT_TEXT_MIN_NON_REPEAT_MATCH_LEN
    );
}

#[test]
fn large_long_line_text_blocks_keep_retained_non_repeat_match_minimum() {
    let mut matcher = MatchGenerator::new(512 * 1024);
    matcher.add_data(
        b"{\"tenant\":\"alpha\",\"path\":\"/v1/archive\",\"status\":200,\"request_id\":\"0123456789abcdef0123456789abcdef\",\"message\":\"structured-log-line\"}\n"
            .repeat((128 * 1024 / 120) + 512)
            .to_vec(),
        SuffixStore::with_capacity(512 * 1024),
        |_, _| {},
    );

    assert_eq!(
        matcher.min_non_repeat_match_len,
        TEXT_MIN_NON_REPEAT_MATCH_LEN
    );
}

#[test]
fn large_short_line_config_text_keeps_smaller_non_repeat_match_minimum() {
    let mut matcher = MatchGenerator::new(512 * 1024);
    matcher.add_data(
        b"Description=Login Service\nProtectSystem=strict\nReadWritePaths=/etc /run\n"
            .repeat((128 * 1024 / 72) + 512)
            .to_vec(),
        SuffixStore::with_capacity(512 * 1024),
        |_, _| {},
    );

    assert_eq!(
        matcher.min_non_repeat_match_len,
        SMALL_TEXT_MIN_NON_REPEAT_MATCH_LEN
    );
}

#[test]
fn small_short_line_config_text_keeps_smaller_non_repeat_match_minimum() {
    let mut matcher = MatchGenerator::new(8 * 1024);
    matcher.add_data(
        b"Description=Login Service\nProtectSystem=strict\nReadWritePaths=/etc /run\n"
            .repeat(8)
            .to_vec(),
        SuffixStore::with_capacity(8 * 1024),
        |_, _| {},
    );

    assert_eq!(
        matcher.min_non_repeat_match_len,
        SMALL_TEXT_MIN_NON_REPEAT_MATCH_LEN
    );
}

#[test]
fn binary_blocks_keep_short_non_repeat_matches() {
    let mut matcher = MatchGenerator::new(2048);
    matcher.add_data(xorshift(2048), SuffixStore::with_capacity(2048), |_, _| {});

    assert_eq!(matcher.min_non_repeat_match_len, MIN_MATCH_LEN);
}

#[test]
fn short_line_text_blocks_use_denser_no_match_probe_step() {
    let mut matcher = MatchGenerator::new(512 * 1024);
    matcher.add_data(
        b"Description=Login Service\nProtectSystem=strict\nReadWritePaths=/etc /run\n"
            .repeat((128 * 1024 / 72) + 512)
            .to_vec(),
        SuffixStore::with_capacity(512 * 1024),
        |_, _| {},
    );

    assert_eq!(
        matcher.no_match_probe_step(0),
        SHORT_LINE_TEXT_NO_MATCH_PROBE_STEP
    );
}

#[test]
fn long_line_text_blocks_keep_wider_no_match_probe_step() {
    let mut matcher = MatchGenerator::new(512 * 1024);
    matcher.add_data(
        b"{\"tenant\":\"alpha\",\"path\":\"/v1/archive\",\"status\":200,\"request_id\":\"0123456789abcdef0123456789abcdef\",\"message\":\"structured-log-line\"}\n"
            .repeat((128 * 1024 / 120) + 512)
            .to_vec(),
        SuffixStore::with_capacity(512 * 1024),
        |_, _| {},
    );

    assert_eq!(matcher.no_match_probe_step(0), TEXT_NO_MATCH_PROBE_STEP);
}

#[test]
fn binary_blocks_keep_default_no_match_probe_step() {
    let mut matcher = MatchGenerator::new(2048);
    matcher.add_data(xorshift(2048), SuffixStore::with_capacity(2048), |_, _| {});

    assert_eq!(matcher.no_match_probe_step(0), NO_MATCH_PROBE_STEP);
}

#[test]
fn empty_committed_entry_has_no_sequences() {
    let mut matcher = MatchGenerator::new(100);
    matcher.add_data(Vec::new(), SuffixStore::with_capacity(100), |_, _| {});

    assert!(!matcher.next_sequence(|_| panic!("empty entry should not emit sequences")));

    matcher.add_data(
        b"abcdeabcde".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );
    assert!(matcher.next_sequence(|_| {}));
}

#[test]
fn driver_reuses_short_frame_suffix_store_for_larger_frame() {
    let mut matcher = MatchGeneratorDriver::new(32, 2);
    matcher.commit_space(b"a".to_vec());
    matcher.skip_matching_for_rle();
    matcher.reset(CompressionLevel::Fastest);

    matcher.commit_space(b"abcdeabcdeabcde".to_vec());

    let mut emitted_sequence = false;
    matcher.start_matching(|_| emitted_sequence = true);
    assert!(emitted_sequence);
}

#[test]
fn driver_uses_c_fast_sized_suffix_store() {
    let mut matcher = MatchGeneratorDriver::new(128, 2);

    matcher.commit_space(b"abcdeabcde".to_vec());

    assert_eq!(
        matcher.match_generator.last_entry().suffixes.slots.len(),
        128 / SUFFIX_STORE_CAPACITY_DIVISOR
    );
}

#[test]
fn driver_uses_larger_suffix_store_for_best_level() {
    let mut matcher = MatchGeneratorDriver::new(128, 2);

    for (level, expected_capacity) in [
        (
            CompressionLevel::Fastest,
            128 / SUFFIX_STORE_CAPACITY_DIVISOR,
        ),
        (
            CompressionLevel::Default,
            128 / SUFFIX_STORE_CAPACITY_DIVISOR,
        ),
        (
            CompressionLevel::Better,
            128 / SUFFIX_STORE_CAPACITY_DIVISOR,
        ),
        (
            CompressionLevel::Best,
            128 * BEST_SUFFIX_STORE_CAPACITY_MULTIPLIER,
        ),
    ] {
        matcher.reset(level);
        matcher.commit_space(b"abcdeabcde".to_vec());

        assert_eq!(
            matcher.match_generator.last_entry().suffixes.slots.len(),
            expected_capacity,
            "{level:?} should use its configured suffix table size"
        );
        matcher.skip_matching();
    }
}

#[test]
fn driver_uses_larger_window_for_best_level() {
    let mut matcher = MatchGeneratorDriver::new(128, 2);

    matcher.reset(CompressionLevel::Fastest);
    assert_eq!(matcher.window_size(), (128 * FASTEST_WINDOW_BLOCKS) as u64);

    matcher.reset(CompressionLevel::Default);
    assert_eq!(matcher.window_size(), (128 * FASTEST_WINDOW_BLOCKS) as u64);

    matcher.reset(CompressionLevel::Better);
    assert_eq!(matcher.window_size(), (128 * FASTEST_WINDOW_BLOCKS) as u64);

    matcher.reset(CompressionLevel::Best);
    assert_eq!(matcher.window_size(), (128 * BEST_WINDOW_BLOCKS) as u64);
}

#[test]
fn driver_enables_adaptive_binary_no_match_probe_for_best_level() {
    let mut matcher = MatchGeneratorDriver::new(128, 2);

    matcher.reset(CompressionLevel::Fastest);
    assert!(!matcher.match_generator.adaptive_binary_no_match_probe);

    matcher.reset(CompressionLevel::Best);
    assert!(matcher.match_generator.adaptive_binary_no_match_probe);
}

#[test]
fn driver_enables_fast_small_dense_binary_probe_for_fastest_level() {
    let mut matcher = MatchGeneratorDriver::new(128, 2);

    matcher.reset(CompressionLevel::Fastest);
    assert!(matcher.match_generator.use_fast_small_dense_binary_probe);
    assert!(!matcher.match_generator.adaptive_binary_no_match_probe);

    matcher.reset(CompressionLevel::Best);
    assert!(!matcher.match_generator.use_fast_small_dense_binary_probe);
}

#[test]
fn driver_enables_next_position_repeat_lookahead_for_best_level() {
    let mut matcher = MatchGeneratorDriver::new(128, 2);

    matcher.reset(CompressionLevel::Fastest);
    assert!(
        !matcher
            .match_generator
            .prefer_binary_next_position_repeat_lookahead
    );

    matcher.reset(CompressionLevel::Best);
    assert!(
        matcher
            .match_generator
            .prefer_binary_next_position_repeat_lookahead
    );
}

#[test]
fn driver_enables_fast_binary_next_position_repeat_lookahead_for_fastest_level() {
    let mut matcher = MatchGeneratorDriver::new(128, 2);

    matcher.reset(CompressionLevel::Fastest);
    assert!(
        matcher
            .match_generator
            .prefer_fast_binary_next_position_repeat_lookahead
    );

    matcher.reset(CompressionLevel::Best);
    assert!(
        !matcher
            .match_generator
            .prefer_fast_binary_next_position_repeat_lookahead
    );
}

#[test]
fn driver_enables_next_position_lookahead_for_best_level() {
    let mut matcher = MatchGeneratorDriver::new(128, 2);

    matcher.reset(CompressionLevel::Fastest);
    assert!(
        !matcher
            .match_generator
            .prefer_binary_next_position_lookahead
    );

    matcher.reset(CompressionLevel::Best);
    assert!(
        matcher
            .match_generator
            .prefer_binary_next_position_lookahead
    );
}

#[test]
fn driver_enables_oldest_first_window_probe_for_best_level() {
    let mut matcher = MatchGeneratorDriver::new(128, 2);

    matcher.reset(CompressionLevel::Fastest);
    assert!(!matcher.match_generator.prefer_oldest_first_window_probe);

    matcher.reset(CompressionLevel::Best);
    assert!(matcher.match_generator.prefer_oldest_first_window_probe);
}

#[test]
fn driver_enables_complementary_end_insertion_for_best_level() {
    let mut matcher = MatchGeneratorDriver::new(128, 2);

    matcher.reset(CompressionLevel::Fastest);
    assert!(!matcher.match_generator.use_complementary_end_insertion);

    matcher.reset(CompressionLevel::Best);
    assert!(matcher.match_generator.use_complementary_end_insertion);
}

#[test]
fn driver_enables_second_newest_probe_for_best_level() {
    let mut matcher = MatchGeneratorDriver::new(128, 2);

    matcher.reset(CompressionLevel::Fastest);
    assert!(!matcher.match_generator.use_second_newest_probe);

    matcher.reset(CompressionLevel::Best);
    assert!(matcher.match_generator.use_second_newest_probe);
}

#[test]
fn driver_enables_fast_binary_small_second_newest_for_fastest_level() {
    let mut matcher = MatchGeneratorDriver::new(128, 2);

    matcher.reset(CompressionLevel::Fastest);
    assert!(matcher.match_generator.use_fast_binary_small_second_newest);
    assert!(!matcher.match_generator.use_second_newest_probe);

    matcher.reset(CompressionLevel::Best);
    assert!(!matcher.match_generator.use_fast_binary_small_second_newest);
}

#[test]
fn driver_enables_text_repeat_pipeline_for_best_level() {
    let mut matcher = MatchGeneratorDriver::new(128, 2);

    matcher.reset(CompressionLevel::Fastest);
    assert!(!matcher.match_generator.use_text_repeat_pipeline);

    matcher.reset(CompressionLevel::Best);
    assert!(matcher.match_generator.use_text_repeat_pipeline);
}

#[test]
fn matcher_tracks_uniform_suffix_store_capacity() {
    let mut matcher = MatchGenerator::new(1024);

    matcher.add_data(b"abcde".to_vec(), SuffixStore::with_capacity(64), |_, _| {});
    assert_eq!(matcher.uniform_suffix_len_log, Some(6));

    matcher.skip_matching();
    matcher.add_data(b"vwxyz".to_vec(), SuffixStore::with_capacity(64), |_, _| {});
    assert_eq!(matcher.uniform_suffix_len_log, Some(6));
}

#[test]
fn matcher_disables_uniform_suffix_store_fast_path_for_mixed_capacities() {
    let mut matcher = MatchGenerator::new(1024);

    matcher.add_data(b"abcde".to_vec(), SuffixStore::with_capacity(64), |_, _| {});
    matcher.skip_matching();
    matcher.add_data(
        b"vwxyz".to_vec(),
        SuffixStore::with_capacity(128),
        |_, _| {},
    );

    assert_eq!(matcher.uniform_suffix_len_log, None);
}

#[test]
fn best_level_keeps_large_active_window_for_text_blocks_only() {
    let mut matcher = MatchGenerator::new(4 * 1024);
    matcher.set_window_sizes(4 * 1024, 2 * 1024);

    for _ in 0..3 {
        matcher.add_data(
            alloc::vec![b'a'; 1024],
            SuffixStore::with_capacity(128),
            |_, _| {},
        );
        matcher.skip_matching();
    }
    assert_eq!(matcher.window_size, 3 * 1024);

    matcher.reset(|_, _| {});
    matcher.set_window_sizes(4 * 1024, 2 * 1024);
    for _ in 0..3 {
        matcher.add_data(xorshift(1024), SuffixStore::with_capacity(128), |_, _| {});
        matcher.skip_matching_for_incompressible();
    }
    assert_eq!(matcher.window_size, 2 * 1024);
}

#[test]
fn repeat_offset_candidate_can_win_with_small_length_deficit() {
    let repeat = MatchCandidate {
        start_idx: 10,
        offset: 16,
        match_len: 8,
        repeat_offset: true,
        #[cfg(test)]
        source: CandidateSource::RepeatCurrent(RepeatCandidateKind::First),
    };
    let normal = MatchCandidate {
        start_idx: 10,
        offset: 1024,
        match_len: 8 + REPEAT_MATCH_LEN_MARGIN,
        repeat_offset: false,
        #[cfg(test)]
        source: CandidateSource::WindowCurrentNewest { entry_distance: 0 },
    };

    assert!(repeat.is_better_than(normal));
}

#[test]
fn dictionary_text_can_prefer_smaller_offset_at_same_start() {
    let mut matcher = MatchGenerator::new(1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;

    let smaller_offset = MatchCandidate {
        start_idx: 10,
        offset: 29,
        match_len: 11,
        repeat_offset: false,
        #[cfg(test)]
        source: CandidateSource::WindowCurrentNewest { entry_distance: 0 },
    };
    let longer_farther = MatchCandidate {
        start_idx: 10,
        offset: 125,
        match_len: 12,
        repeat_offset: false,
        #[cfg(test)]
        source: CandidateSource::WindowCurrentOldest { entry_distance: 0 },
    };

    assert!(matcher.candidate_is_better_than(smaller_offset, longer_farther));
    assert!(!matcher.candidate_is_better_than(longer_farther, smaller_offset));
}

#[test]
fn composer_dictionary_text_can_prefer_repeat_kind_at_same_start() {
    let mut matcher = MatchGenerator::new(1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    matcher.current_block_is_composer_dictionary_text = true;
    matcher.offset_history = OffsetHistory::from_offsets(20, 40, 60);
    matcher.suffix_idx = 100;
    matcher.last_idx_in_sequence = 100;

    let preferred = MatchCandidate {
        start_idx: 100,
        offset: 40,
        match_len: 10,
        repeat_offset: true,
        #[cfg(test)]
        source: CandidateSource::RepeatCurrent(RepeatCandidateKind::Second),
    };
    let other = MatchCandidate {
        start_idx: 100,
        offset: 19,
        match_len: 11,
        repeat_offset: true,
        #[cfg(test)]
        source: CandidateSource::RepeatCurrent(RepeatCandidateKind::First),
    };

    assert!(matcher.candidate_is_better_than(preferred, other));
    assert!(!matcher.candidate_is_better_than(other, preferred));
}

#[test]
fn lockfile_dictionary_text_can_prefer_repeat_kind_at_same_start() {
    let mut matcher = MatchGenerator::new(1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    matcher.current_block_is_dictionary_lockfile_text = true;
    matcher.offset_history = OffsetHistory::from_offsets(20, 40, 60);
    matcher.suffix_idx = 100;
    matcher.last_idx_in_sequence = 100;

    let preferred = MatchCandidate {
        start_idx: 100,
        offset: 40,
        match_len: 10,
        repeat_offset: true,
        #[cfg(test)]
        source: CandidateSource::RepeatCurrent(RepeatCandidateKind::Second),
    };
    let other = MatchCandidate {
        start_idx: 100,
        offset: 19,
        match_len: 11,
        repeat_offset: true,
        #[cfg(test)]
        source: CandidateSource::RepeatCurrent(RepeatCandidateKind::First),
    };

    assert!(matcher.candidate_is_better_than(preferred, other));
    assert!(!matcher.candidate_is_better_than(other, preferred));
}

#[test]
fn lockfile_dictionary_text_can_prefer_smaller_offset_at_same_end() {
    let mut matcher = MatchGenerator::new(1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    matcher.current_block_is_dictionary_lockfile_text = true;

    let smaller_offset = MatchCandidate {
        start_idx: 11,
        offset: 29,
        match_len: 11,
        repeat_offset: false,
        #[cfg(test)]
        source: CandidateSource::WindowCurrentSecondNewest { entry_distance: 0 },
    };
    let longer_farther = MatchCandidate {
        start_idx: 10,
        offset: 125,
        match_len: 12,
        repeat_offset: false,
        #[cfg(test)]
        source: CandidateSource::WindowCurrentOldest { entry_distance: 0 },
    };

    assert!(matcher.candidate_is_better_than(smaller_offset, longer_farther));
    assert!(!matcher.candidate_is_better_than(longer_farther, smaller_offset));
}

#[test]
fn longer_normal_candidate_wins_beyond_repeat_offset_margin() {
    let repeat = MatchCandidate {
        start_idx: 10,
        offset: 16,
        match_len: 8,
        repeat_offset: true,
        #[cfg(test)]
        source: CandidateSource::RepeatCurrent(RepeatCandidateKind::First),
    };
    let normal = MatchCandidate {
        start_idx: 10,
        offset: 1024,
        match_len: 8 + REPEAT_MATCH_LEN_MARGIN + 1,
        repeat_offset: false,
        #[cfg(test)]
        source: CandidateSource::WindowCurrentNewest { entry_distance: 0 },
    };

    assert!(normal.is_better_than(repeat));
}

#[test]
fn long_repeat_offset_candidate_skips_window_search() {
    let repeat = MatchCandidate {
        start_idx: 10,
        offset: 16,
        match_len: REPEAT_SEARCH_EARLY_EXIT_LEN,
        repeat_offset: true,
        #[cfg(test)]
        source: CandidateSource::RepeatCurrent(RepeatCandidateKind::First),
    };
    let normal = MatchCandidate {
        start_idx: 10,
        offset: 16,
        match_len: REPEAT_SEARCH_EARLY_EXIT_LEN,
        repeat_offset: false,
        #[cfg(test)]
        source: CandidateSource::WindowCurrentNewest { entry_distance: 0 },
    };

    assert!(repeat.can_skip_window_search(128));
    assert!(!normal.can_skip_window_search(128));
}

#[test]
fn repeat_offset_candidate_skips_window_search_at_block_end() {
    let repeat = MatchCandidate {
        start_idx: 10,
        offset: 16,
        match_len: MIN_MATCH_LEN,
        repeat_offset: true,
        #[cfg(test)]
        source: CandidateSource::RepeatCurrent(RepeatCandidateKind::First),
    };

    assert!(repeat.can_skip_window_search(10 + MIN_MATCH_LEN));
}

#[test]
fn no_match_step_does_not_skip_next_repeat_offset_match() {
    let mut matcher = MatchGenerator::new(100);
    matcher.add_data(
        b"MATCHTAIL".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );
    matcher.skip_matching_for_incompressible();
    matcher.offset_history = OffsetHistory::from_offsets(10, 4, 8);
    matcher.add_data(
        b"xMATCHTAIL".to_vec(),
        SuffixStore::with_capacity(100),
        |_, _| {},
    );

    assert!(matcher.repeat_offset_can_match_at(1, "MATCHTAIL".len()));
}

#[test]
#[ignore]
fn inspect_best_matcher_from_env() {
    use std::fs;
    use std::println;

    let fixture = std::env::var("RUZSTD_MATCHER_FIXTURE")
        .expect("set RUZSTD_MATCHER_FIXTURE to a fixture path");
    let input = fs::read(&fixture).expect("fixture must be readable");

    let mut matcher = MatchGeneratorDriver::new(128 * 1024, 4);
    matcher.reset(CompressionLevel::Best);
    if let Ok(file_type) = std::env::var("RUZSTD_MATCHER_FILE_TYPE") {
        let file_type = match file_type.as_str() {
            "unknown" => CompressionFileType::Unknown,
            "archive" => CompressionFileType::ArchiveLike,
            "binary" => CompressionFileType::BinaryLike,
            "code" => CompressionFileType::CodeText,
            "config" => CompressionFileType::ConfigText,
            "json" => CompressionFileType::JsonText,
            "dictionary" => CompressionFileType::DictionaryText,
            other => panic!("unsupported RUZSTD_MATCHER_FILE_TYPE={}", other),
        };
        matcher.set_file_type_hint(file_type);
    }

    let mut offset = 0usize;
    while offset < input.len() {
        let end = (offset + 128 * 1024).min(input.len());
        let chunk = &input[offset..end];
        let mut space = matcher.get_next_space();
        space[..chunk.len()].copy_from_slice(chunk);
        space.truncate(chunk.len());
        matcher.commit_space(space);
        matcher.start_matching(|_| {});
        offset = end;
    }

    println!("fixture: {fixture}");
    println!("{:#?}", matcher.diagnostics());
}

#[test]
#[ignore]
fn inspect_fastest_matcher_from_env() {
    use std::fs;
    use std::println;

    let fixture = std::env::var("RUZSTD_MATCHER_FIXTURE")
        .expect("set RUZSTD_MATCHER_FIXTURE to a fixture path");
    let input = fs::read(&fixture).expect("fixture must be readable");

    let mut matcher = MatchGeneratorDriver::new(128 * 1024, 4);
    matcher.reset(CompressionLevel::Fastest);
    if let Ok(file_type) = std::env::var("RUZSTD_MATCHER_FILE_TYPE") {
        let file_type = match file_type.as_str() {
            "unknown" => CompressionFileType::Unknown,
            "archive" => CompressionFileType::ArchiveLike,
            "binary" => CompressionFileType::BinaryLike,
            "code" => CompressionFileType::CodeText,
            "config" => CompressionFileType::ConfigText,
            "json" => CompressionFileType::JsonText,
            "dictionary" => CompressionFileType::DictionaryText,
            other => panic!("unsupported RUZSTD_MATCHER_FILE_TYPE={}", other),
        };
        matcher.set_file_type_hint(file_type);
    }

    let mut offset = 0usize;
    while offset < input.len() {
        let end = (offset + 128 * 1024).min(input.len());
        let chunk = &input[offset..end];
        let mut space = matcher.get_next_space();
        space[..chunk.len()].copy_from_slice(chunk);
        space.truncate(chunk.len());
        matcher.commit_space(space);
        matcher.start_matching(|_| {});
        offset = end;
    }

    println!("fixture: {fixture}");
    println!("{:#?}", matcher.diagnostics());
}

#[test]
fn adaptive_binary_no_match_probe_grows_with_literal_run() {
    let mut matcher = MatchGenerator::new(1024);
    matcher.adaptive_binary_no_match_probe = true;

    assert_eq!(matcher.no_match_probe_step(0), NO_MATCH_PROBE_STEP);
    assert_eq!(matcher.no_match_probe_step(255), NO_MATCH_PROBE_STEP);
    assert_eq!(matcher.no_match_probe_step(256), NO_MATCH_PROBE_STEP + 1);
    assert_eq!(matcher.no_match_probe_step(512), NO_MATCH_PROBE_STEP + 2);
}

#[test]
fn adaptive_binary_no_match_probe_does_not_change_text_probe_step() {
    let mut matcher = MatchGenerator::new(1024);
    matcher.adaptive_binary_no_match_probe = true;
    matcher.is_text_block = true;
    matcher.min_non_repeat_match_len = TEXT_MIN_NON_REPEAT_MATCH_LEN;

    assert_eq!(matcher.no_match_probe_step(1024), TEXT_NO_MATCH_PROBE_STEP);
}

#[test]
fn fast_dense_binary_probe_uses_dense_step_for_small_non_text() {
    let mut matcher = MatchGenerator::new(FASTEST_DENSE_BINARY_PROBE_MAX_BLOCK_LEN + 1);
    matcher.use_fast_small_dense_binary_probe = true;
    matcher.add_data(
        alloc::vec![0u8; FASTEST_DENSE_BINARY_PROBE_MAX_BLOCK_LEN],
        SuffixStore::with_capacity(64),
        |_, _| {},
    );

    assert_eq!(matcher.no_match_probe_step(0), 1);
}

#[test]
fn fast_dense_binary_probe_keeps_default_step_for_large_non_text() {
    let mut matcher = MatchGenerator::new(FASTEST_DENSE_BINARY_PROBE_MAX_BLOCK_LEN + 1);
    matcher.use_fast_small_dense_binary_probe = true;
    matcher.add_data(
        alloc::vec![0u8; FASTEST_DENSE_BINARY_PROBE_MAX_BLOCK_LEN + 1],
        SuffixStore::with_capacity(64),
        |_, _| {},
    );

    assert_eq!(matcher.no_match_probe_step(0), NO_MATCH_PROBE_STEP);
}

#[test]
fn medium_code_text_blocks_keep_dense_probe_step() {
    let mut matcher = MatchGenerator::new(CODE_TEXT_DENSE_PROBE_MAX_BLOCK_LEN + 1);
    matcher.file_type_hint = CompressionFileType::CodeText;
    matcher.add_data(
        b"fn progress_step(value: usize) { println!(\"{}\", value); }\n"
            .repeat((9 * 1024 / 58) + 8)
            .to_vec(),
        SuffixStore::with_capacity(CODE_TEXT_DENSE_PROBE_MAX_BLOCK_LEN + 1),
        |_, _| {},
    );

    assert!(matcher.is_short_line_text);
    assert!(matcher.last_entry().data.len() > CONFIG_TEXT_DENSE_PROBE_MAX_BLOCK_LEN);
    assert!(matcher.last_entry().data.len() <= CODE_TEXT_DENSE_PROBE_MAX_BLOCK_LEN);
    assert_eq!(matcher.no_match_probe_step(0), 1);
}

#[test]
fn medium_config_text_blocks_keep_short_line_probe_step() {
    let mut matcher = MatchGenerator::new(CODE_TEXT_DENSE_PROBE_MAX_BLOCK_LEN + 1);
    matcher.file_type_hint = CompressionFileType::ConfigText;
    matcher.add_data(
        b"Description=Progress Service\nProtectSystem=strict\nReadWritePaths=/run/progress\n"
            .repeat((9 * 1024 / 78) + 8)
            .to_vec(),
        SuffixStore::with_capacity(CODE_TEXT_DENSE_PROBE_MAX_BLOCK_LEN + 1),
        |_, _| {},
    );

    assert!(matcher.is_short_line_text);
    assert!(matcher.last_entry().data.len() > CONFIG_TEXT_DENSE_PROBE_MAX_BLOCK_LEN);
    assert!(matcher.last_entry().data.len() <= CODE_TEXT_DENSE_PROBE_MAX_BLOCK_LEN);
    assert_eq!(
        matcher.no_match_probe_step(0),
        SHORT_LINE_TEXT_NO_MATCH_PROBE_STEP
    );
}

#[test]
fn structured_json_config_text_is_detected() {
    let data = br#"{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext"
  },
  "include": [
    "src/**/*.ts"
  ],
  "exclude": [
    "dist"
  ]
}"#;

    assert!(MatchGenerator::likely_structured_json_config_text(data));
}

#[test]
fn medium_structured_json_config_text_blocks_keep_dense_probe_step() {
    let mut matcher = MatchGenerator::new(STRUCTURED_JSON_CONFIG_DENSE_PROBE_MAX_BLOCK_LEN + 1);
    matcher.file_type_hint = CompressionFileType::ConfigText;
    matcher.add_data(
        b"{\n  \"dependencies\": {\n    \"dep-0001\": \"^1.0.0\",\n    \"dep-0002\": \"^2.0.0\"\n  },\n  \"devDependencies\": {\n    \"dev-dep-0001\": \"~1.0.0\",\n    \"dev-dep-0002\": \"~2.0.0\"\n  },\n  \"scripts\": {\n    \"build\": \"vite build\",\n    \"test\": \"vitest run\"\n  }\n}\n"
            .repeat((96 * 1024 / 304) + 64)
            .to_vec(),
        SuffixStore::with_capacity(STRUCTURED_JSON_CONFIG_DENSE_PROBE_MAX_BLOCK_LEN + 1),
        |_, _| {},
    );

    assert!(matcher.is_short_line_text);
    assert!(matcher.current_block_is_structured_json_config_text);
    assert!(!matcher.current_block_is_tsconfig_json_config_text);
    assert!(matcher.last_entry().data.len() > CONFIG_TEXT_DENSE_PROBE_MAX_BLOCK_LEN);
    assert!(matcher.last_entry().data.len() <= STRUCTURED_JSON_CONFIG_DENSE_PROBE_MAX_BLOCK_LEN);
    assert_eq!(matcher.no_match_probe_step(0), 1);
}

#[test]
fn tsconfig_json_config_text_is_detected() {
    let data = br#"{
  "compilerOptions": {
    "module": "NodeNext",
    "paths": {
      "@feature/0000/*": [
        "./src/feature_0000/*"
      ]
    }
  },
  "include": [
    "src/**/*.ts"
  ],
  "exclude": [
    "dist"
  ]
}"#;

    assert!(MatchGenerator::likely_tsconfig_json_config_text(data));
}

#[test]
fn tsconfig_json_config_text_keeps_wider_probe_step() {
    let mut matcher = MatchGenerator::new(STRUCTURED_JSON_CONFIG_DENSE_PROBE_MAX_BLOCK_LEN + 1);
    matcher.file_type_hint = CompressionFileType::ConfigText;
    matcher.add_data(
        b"{\n  \"compilerOptions\": {\n    \"module\": \"NodeNext\",\n    \"paths\": {\n      \"@feature/0000/*\": [\"./src/feature_0000/*\"],\n      \"@feature/0001/*\": [\"./src/feature_0001/*\"]\n    }\n  },\n  \"include\": [\"src/**/*.ts\"],\n  \"exclude\": [\"dist\"]\n}\n"
            .repeat((96 * 1024 / 268) + 64)
            .to_vec(),
        SuffixStore::with_capacity(STRUCTURED_JSON_CONFIG_DENSE_PROBE_MAX_BLOCK_LEN + 1),
        |_, _| {},
    );

    assert!(matcher.is_short_line_text);
    assert!(matcher.current_block_is_structured_json_config_text);
    assert!(matcher.current_block_is_tsconfig_json_config_text);
    assert_eq!(
        matcher.no_match_probe_step(0),
        TSCONFIG_JSON_TEXT_NO_MATCH_PROBE_STEP
    );
}

#[test]
fn composer_dictionary_text_is_detected() {
    let chunk = br#"{
  "packages": [
    {
      "name": "vendor/package-0001",
      "require": {
        "php": ">=8.2"
      },
      "source": {
        "reference": "0000000000000000000000000000000000000001",
        "type": "git",
        "url": "https://example.com/vendor/package-0001.git"
      },
      "version": "1.0.0"
    },
    {
      "name": "vendor/package-0002",
      "require": {
        "php": ">=8.2"
      },
      "source": {
        "reference": "0000000000000000000000000000000000000002",
        "type": "git",
        "url": "https://example.com/vendor/package-0002.git"
      },
      "version": "1.0.1"
    }
  ]
}
"#;
    let data = chunk.repeat((16 * 1024 / chunk.len()) + 4);

    assert!(MatchGenerator::likely_composer_dictionary_text(&data));
}

#[test]
fn composer_dictionary_text_keeps_wider_probe_step() {
    let mut matcher = MatchGenerator::new(STRUCTURED_JSON_CONFIG_DENSE_PROBE_MAX_BLOCK_LEN + 1);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    matcher.add_data(
        b"{\n  \"packages\": [\n    {\n      \"name\": \"vendor/package-0001\",\n      \"require\": {\n        \"php\": \">=8.2\",\n        \"vendor/dependency-0002\": \"^2.0\"\n      },\n      \"source\": {\n        \"reference\": \"0000000000000000000000000000000000000001\",\n        \"type\": \"git\",\n        \"url\": \"https://example.com/vendor/package-0001.git\"\n      },\n      \"version\": \"1.0.0\"\n    }\n  ]\n}\n"
            .repeat((96 * 1024 / 420) + 96)
            .to_vec(),
        SuffixStore::with_capacity(STRUCTURED_JSON_CONFIG_DENSE_PROBE_MAX_BLOCK_LEN + 1),
        |_, _| {},
    );

    assert!(matcher.is_short_line_text);
    assert!(matcher.current_block_is_composer_dictionary_text);
    assert!(!matcher.current_block_is_dictionary_lockfile_text);
    assert_eq!(
        matcher.no_match_probe_step(0),
        COMPOSER_JSON_LOCKFILE_NO_MATCH_PROBE_STEP
    );
}

#[test]
fn large_code_text_blocks_keep_dense_probe_step() {
    let mut matcher = MatchGenerator::new(CODE_TEXT_DENSE_PROBE_MAX_BLOCK_LEN + 1);
    matcher.file_type_hint = CompressionFileType::CodeText;
    matcher.add_data(
        b"pub(crate) fn emit_symbol(code: u8) { table.push(code); }\n"
            .repeat((48 * 1024 / 57) + 16)
            .to_vec(),
        SuffixStore::with_capacity(CODE_TEXT_DENSE_PROBE_MAX_BLOCK_LEN + 1),
        |_, _| {},
    );

    assert!(matcher.is_short_line_text);
    assert!(matcher.last_entry().data.len() > 32 * 1024);
    assert!(matcher.last_entry().data.len() <= CODE_TEXT_DENSE_PROBE_MAX_BLOCK_LEN);
    assert_eq!(matcher.no_match_probe_step(0), 1);
}

#[test]
fn code_text_blocks_just_above_64k_keep_dense_probe_step() {
    let mut matcher = MatchGenerator::new(CODE_TEXT_DENSE_PROBE_MAX_BLOCK_LEN + 1);
    matcher.file_type_hint = CompressionFileType::CodeText;
    matcher.add_data(
        b"pub(crate) fn write_block_header(bits: &mut BitWriter, compressed: bool) {\n"
            .repeat((66 * 1024 / 73) + 24)
            .to_vec(),
        SuffixStore::with_capacity(CODE_TEXT_DENSE_PROBE_MAX_BLOCK_LEN + 1),
        |_, _| {},
    );

    assert!(matcher.is_short_line_text);
    assert!(matcher.last_entry().data.len() > 64 * 1024);
    assert!(matcher.last_entry().data.len() <= CODE_TEXT_DENSE_PROBE_MAX_BLOCK_LEN);
    assert_eq!(matcher.no_match_probe_step(0), 1);
}

#[test]
fn small_code_text_blocks_use_lower_non_repeat_floor() {
    let mut matcher = MatchGenerator::new(SMALL_CODE_TEXT_MIN_NON_REPEAT_MAX_BLOCK_LEN + 1);
    matcher.file_type_hint = CompressionFileType::CodeText;
    matcher.add_data(
        b"pub(crate) fn update_progress(done: usize, total: usize) { println!(\"{done}/{total}\"); }\n"
            .repeat((12 * 1024 / 91) + 12)
            .to_vec(),
        SuffixStore::with_capacity(SMALL_CODE_TEXT_MIN_NON_REPEAT_MAX_BLOCK_LEN + 1),
        |_, _| {},
    );

    assert!(matcher.is_short_line_text);
    assert!(matcher.last_entry().data.len() <= SMALL_CODE_TEXT_MIN_NON_REPEAT_MAX_BLOCK_LEN);
    assert_eq!(
        matcher.min_non_repeat_match_len,
        SMALL_TEXT_MIN_NON_REPEAT_MATCH_LEN
    );
}

#[test]
fn large_code_text_blocks_keep_code_non_repeat_floor() {
    let mut matcher = MatchGenerator::new(CODE_TEXT_DENSE_PROBE_MAX_BLOCK_LEN + 1);
    matcher.file_type_hint = CompressionFileType::CodeText;
    matcher.add_data(
        b"pub(crate) fn emit_symbol(code: u8) { table.push(code); }\n"
            .repeat((48 * 1024 / 57) + 16)
            .to_vec(),
        SuffixStore::with_capacity(CODE_TEXT_DENSE_PROBE_MAX_BLOCK_LEN + 1),
        |_, _| {},
    );

    assert!(matcher.is_short_line_text);
    assert!(matcher.last_entry().data.len() > SMALL_CODE_TEXT_MIN_NON_REPEAT_MAX_BLOCK_LEN);
    assert_eq!(
        matcher.min_non_repeat_match_len,
        CODE_LIKE_SHORT_TEXT_MIN_NON_REPEAT_MATCH_LEN
    );
}

#[test]
fn best_level_can_prefer_longer_next_position_window_match() {
    let mut matcher = MatchGenerator::new(1024);
    matcher.prefer_binary_next_position_lookahead = true;
    matcher.add_data(
        b"ABCDEBCDE1234567890".to_vec(),
        SuffixStore::with_capacity(1024),
        |_, _| {},
    );
    matcher.skip_matching();
    matcher.add_data(
        b"ABCDE1234567890".to_vec(),
        SuffixStore::with_capacity(1024),
        |_, _| {},
    );

    matcher.next_sequence(|seq| {
        assert_eq!(
            seq,
            Sequence::Triple {
                literals: b"A",
                offset: 15,
                match_len: 14,
            }
        );
    });
    assert!(!matcher.next_sequence(|_| {}));
}

#[test]
fn best_level_can_find_next_position_window_match_without_current_candidate() {
    let mut matcher = MatchGenerator::new(1024);
    matcher.prefer_binary_next_position_lookahead = true;
    matcher.add_data(
        b"BCDE1234567890".to_vec(),
        SuffixStore::with_capacity(1024),
        |_, _| {},
    );
    matcher.skip_matching();
    matcher.add_data(
        b"ABCDE1234567890".to_vec(),
        SuffixStore::with_capacity(1024),
        |_, _| {},
    );

    matcher.next_sequence(|seq| match seq {
        Sequence::Triple {
            literals,
            offset,
            match_len,
        } => {
            assert_eq!(literals, b"A");
            assert_eq!(match_len, 14);
            assert!(offset > 1);
        }
        other => panic!("expected triple, got {:?}", other),
    });
    assert!(!matcher.next_sequence(|_| {}));
}

#[test]
fn best_level_can_prefer_next_position_repeat_match() {
    let mut matcher = MatchGenerator::new(1024);
    matcher.prefer_binary_next_position_repeat_lookahead = true;
    matcher.add_data(
        b"MATCHTAIL".to_vec(),
        SuffixStore::with_capacity(1024),
        |_, _| {},
    );
    matcher.skip_matching_for_incompressible();
    matcher.offset_history = OffsetHistory::from_offsets(10, 4, 8);
    matcher.add_data(
        b"xMATCHTAIL".to_vec(),
        SuffixStore::with_capacity(1024),
        |_, _| {},
    );

    matcher.next_sequence(|seq| {
        assert_eq!(
            seq,
            Sequence::Triple {
                literals: b"x",
                offset: 10,
                match_len: 9,
            }
        );
    });
    assert!(!matcher.next_sequence(|_| {}));
}

#[test]
fn best_text_repeat_helper_can_prefer_next_position_repeat_match() {
    let mut matcher = MatchGenerator::new(1024);
    matcher.prefer_binary_next_position_repeat_lookahead = true;
    matcher.use_second_newest_probe = true;
    matcher.use_text_repeat_pipeline = true;
    matcher.is_text_block = true;
    matcher.min_non_repeat_match_len = TEXT_MIN_NON_REPEAT_MATCH_LEN;
    matcher.add_data(
        b"MATCHTAIL".to_vec(),
        SuffixStore::with_capacity(1024),
        |_, _| {},
    );
    matcher.skip_matching_for_incompressible();
    matcher.offset_history = OffsetHistory::from_offsets(10, 4, 8);
    matcher.add_data(
        b"xMATCHTAIL".to_vec(),
        SuffixStore::with_capacity(1024),
        |_, _| {},
    );

    matcher.next_sequence(|seq| {
        assert_eq!(
            seq,
            Sequence::Triple {
                literals: b"x",
                offset: 10,
                match_len: 9,
            }
        );
    });
    assert!(!matcher.next_sequence(|_| {}));
}

#[test]
fn best_text_repeat_helper_keeps_current_repeat_match() {
    let mut matcher = MatchGenerator::new(1024);
    matcher.use_text_repeat_pipeline = true;
    matcher.is_text_block = true;
    matcher.min_non_repeat_match_len = TEXT_MIN_NON_REPEAT_MATCH_LEN;
    matcher.add_data(
        b"MATCHTAIL".to_vec(),
        SuffixStore::with_capacity(1024),
        |_, _| {},
    );
    matcher.skip_matching_for_incompressible();
    matcher.offset_history = OffsetHistory::from_offsets(12, 9, 4);
    matcher.add_data(
        b"MATCHTAIL".to_vec(),
        SuffixStore::with_capacity(1024),
        |_, _| {},
    );

    matcher.next_sequence(|seq| {
        assert_eq!(
            seq,
            Sequence::Triple {
                literals: b"",
                offset: 9,
                match_len: 9,
            }
        );
    });
    assert!(!matcher.next_sequence(|_| {}));
}

#[test]
fn unavailable_repeat_offsets_are_rejected_before_lookup() {
    assert!(!MatchGenerator::repeat_offset_is_available(0, 8, 4));
    assert!(!MatchGenerator::repeat_offset_is_available(13, 8, 4));
    assert!(MatchGenerator::repeat_offset_is_available(12, 8, 4));
}

#[test]
fn rle_history_indexes_only_extreme_suffixes() {
    let mut matcher = MatchGenerator::new(1024);
    matcher.add_data(
        alloc::vec![0; 512],
        SuffixStore::with_capacity(1024),
        |_, _| {},
    );

    matcher.skip_matching_for_rle();

    let suffixes = &matcher.last_entry().suffixes;
    let indexed = suffixes.slots.iter().filter(|slot| slot.is_some()).count();
    assert_eq!(indexed, 1, "RLE block should only need one suffix key");

    let candidates = suffixes
        .candidates(&[0; MIN_MATCH_LEN])
        .expect("RLE suffix key should exist");
    assert_eq!(candidates.oldest, 0);
    assert_eq!(candidates.newest, Some(512 - MIN_MATCH_LEN));
}

#[test]
fn sparse_rle_history_still_matches_following_repeated_block() {
    let mut matcher = MatchGenerator::new(1024);
    matcher.add_data(
        alloc::vec![0; 512],
        SuffixStore::with_capacity(1024),
        |_, _| {},
    );
    matcher.skip_matching_for_rle();
    matcher.offset_history = OffsetHistory::from_offsets(2048, 2049, 2050);
    matcher.add_data(
        alloc::vec![0; 32],
        SuffixStore::with_capacity(1024),
        |_, _| {},
    );

    matcher.next_sequence(|seq| {
        assert_eq!(
            seq,
            Sequence::Triple {
                literals: &[],
                offset: MIN_MATCH_LEN,
                match_len: 32,
            },
        );
    });
    assert!(!matcher.next_sequence(|_| {}));
}

#[test]
fn short_match_ranges_are_indexed_densely() {
    let mut matcher = MatchGenerator::new(1024);
    matcher.add_data(xorshift(512), SuffixStore::with_capacity(1024), |_, _| {});
    matcher.suffix_idx = 10;

    matcher.add_suffixes_for_match(10 + DENSE_MATCH_INDEX_LIMIT);

    let indexed = matcher
        .last_entry()
        .suffixes
        .slots
        .iter()
        .filter(|slot| slot.is_some())
        .count();
    assert!(
        indexed > 16,
        "short match should index densely: {}",
        indexed
    );
}

#[test]
fn long_match_ranges_are_indexed_sparsely() {
    let mut matcher = MatchGenerator::new(1024);
    matcher.add_data(xorshift(512), SuffixStore::with_capacity(1024), |_, _| {});
    matcher.suffix_idx = 10;

    let match_end = 10 + DENSE_MATCH_INDEX_LIMIT + 1;
    matcher.add_suffixes_for_match(match_end);

    let last_entry = matcher.last_entry();
    let suffixes = &last_entry.suffixes;
    let indexed = suffixes.slots.iter().filter(|slot| slot.is_some()).count();
    assert!(
        indexed <= 3,
        "long match should index sparsely: {}",
        indexed
    );

    for idx in [10, 12, match_end - SPARSE_MATCH_END_INDEX_BACKOFF] {
        let key = &last_entry.data[idx..idx + MIN_MATCH_LEN];
        let candidates = suffixes.candidates(key).expect("sparse index must exist");
        assert_eq!(candidates.oldest, idx);
        assert_eq!(candidates.newest, None);
    }
}

#[test]
fn best_level_long_match_ranges_use_complementary_end_insertion() {
    let mut matcher = MatchGenerator::new(1024);
    matcher.use_complementary_end_insertion = true;
    matcher.add_data(xorshift(512), SuffixStore::with_capacity(1024), |_, _| {});
    matcher.suffix_idx = 10;

    let match_end = 10 + DENSE_MATCH_INDEX_LIMIT + 1;
    matcher.add_suffixes_for_match(match_end);

    let last_entry = matcher.last_entry();
    let suffixes = &last_entry.suffixes;

    for idx in [
        10,
        12,
        match_end - 1,
        match_end - SPARSE_MATCH_END_INDEX_BACKOFF,
    ] {
        let key = &last_entry.data[idx..idx + MIN_MATCH_LEN];
        let candidates = suffixes
            .candidates(key)
            .expect("best-level sparse index must exist");
        assert_eq!(candidates.oldest, idx);
        assert_eq!(candidates.newest, None);
    }
}

#[test]
fn best_level_zero_literal_repeat_matches_use_sparse_indexing() {
    let mut matcher = MatchGenerator::new(1024);
    matcher.use_complementary_end_insertion = true;
    matcher.add_data(xorshift(512), SuffixStore::with_capacity(1024), |_, _| {});
    matcher.suffix_idx = 10;
    matcher.last_idx_in_sequence = 10;

    let match_end = 10 + 32;
    matcher.emit_candidate(
        MatchCandidate {
            start_idx: 10,
            offset: 32,
            match_len: 32,
            repeat_offset: true,
            #[cfg(test)]
            source: CandidateSource::RepeatCurrent(RepeatCandidateKind::First),
        },
        &mut |_| {},
    );

    let last_entry = matcher.last_entry();
    let suffixes = &last_entry.suffixes;
    for idx in [
        10,
        12,
        match_end - 1,
        match_end - SPARSE_MATCH_END_INDEX_BACKOFF,
    ] {
        let key = &last_entry.data[idx..idx + MIN_MATCH_LEN];
        let candidates = suffixes
            .candidates(key)
            .expect("best-level zero-literal repeat sparse index must exist");
        assert_eq!(candidates.oldest, idx);
        assert_eq!(candidates.newest, None);
    }
}

#[test]
fn best_sidecar_tracks_second_newest_for_current_entry() {
    let mut matcher = MatchGenerator::new(64 * 1024);
    matcher.use_second_newest_probe = true;
    let data = [0u8, 1, 2, 3, 4].repeat(BEST_SECOND_NEWEST_MIN_BLOCK_LEN / 5 + 1);
    matcher.add_data(data, SuffixStore::with_capacity(64), |_, _| {});

    let key_value = SuffixStore::key_value(&matcher.last_entry().data[0..MIN_MATCH_LEN]);
    matcher.add_suffix_at(0);
    matcher.add_suffix_at(5);
    matcher.add_suffix_at(10);

    let slot_index = matcher.last_entry().suffixes.slot_key(key_value).index;
    let stored = matcher.current_second_newest_sidecar[slot_index];
    assert!(stored.is_some(), "expected second-newest sidecar entry");
}

#[test]
fn lockfile_sidecar_tracks_third_newest_for_current_entry() {
    let mut matcher = MatchGenerator::new(128 * 1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    matcher.file_profile_hint = CompressionFileProfile::CargoLock;
    let package = b"[[package]]\nname = \"crate\"\nversion = \"1.0.0\"\nchecksum = \"0123456789abcdef\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\ndependencies = [\n \"dep\",\n]\n\n";
    let data = package.repeat(BEST_SECOND_NEWEST_MIN_BLOCK_LEN / package.len() + 1);
    matcher.add_data(data, SuffixStore::with_capacity(64), |_, _| {});

    let key_value = SuffixStore::key_value(&matcher.last_entry().data[0..MIN_MATCH_LEN]);
    matcher.add_suffix_at(0);
    matcher.add_suffix_at(package.len());
    matcher.add_suffix_at(package.len() * 2);
    matcher.add_suffix_at(package.len() * 3);

    let slot_index = matcher.last_entry().suffixes.slot_key(key_value).index;
    let stored = matcher.current_third_newest_sidecar[slot_index];
    assert!(stored.is_some(), "expected third-newest sidecar entry");
}

#[test]
fn lockfile_sidecar_tracks_fourth_newest_for_current_entry() {
    let mut matcher = MatchGenerator::new(128 * 1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    matcher.file_profile_hint = CompressionFileProfile::CargoLock;
    let package = b"[[package]]\nname = \"crate\"\nversion = \"1.0.0\"\nchecksum = \"0123456789abcdef\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\ndependencies = [\n \"dep\",\n]\n\n";
    let data = package.repeat(BEST_SECOND_NEWEST_MIN_BLOCK_LEN / package.len() + 1);
    matcher.add_data(data, SuffixStore::with_capacity(64), |_, _| {});

    let key_value = SuffixStore::key_value(&matcher.last_entry().data[0..MIN_MATCH_LEN]);
    matcher.add_suffix_at(0);
    matcher.add_suffix_at(package.len());
    matcher.add_suffix_at(package.len() * 2);
    matcher.add_suffix_at(package.len() * 3);
    matcher.add_suffix_at(package.len() * 4);

    let slot_index = matcher.last_entry().suffixes.slot_key(key_value).index;
    let stored = matcher.current_fourth_newest_sidecar[slot_index];
    assert!(stored.is_some(), "expected fourth-newest sidecar entry");
}

#[test]
fn lockfile_sidecar_tracks_fifth_newest_for_current_entry() {
    let mut matcher = MatchGenerator::new(128 * 1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    matcher.file_profile_hint = CompressionFileProfile::CargoLock;
    let package = b"[[package]]\nname = \"crate\"\nversion = \"1.0.0\"\nchecksum = \"0123456789abcdef\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\ndependencies = [\n \"dep\",\n]\n\n";
    let data = package.repeat(BEST_SECOND_NEWEST_MIN_BLOCK_LEN / package.len() + 1);
    matcher.add_data(data, SuffixStore::with_capacity(64), |_, _| {});

    let key_value = SuffixStore::key_value(&matcher.last_entry().data[0..MIN_MATCH_LEN]);
    matcher.add_suffix_at(0);
    matcher.add_suffix_at(package.len());
    matcher.add_suffix_at(package.len() * 2);
    matcher.add_suffix_at(package.len() * 3);
    matcher.add_suffix_at(package.len() * 4);
    matcher.add_suffix_at(package.len() * 5);

    let slot_index = matcher.last_entry().suffixes.slot_key(key_value).index;
    let stored = matcher.current_fifth_newest_sidecar[slot_index];
    assert!(stored.is_some(), "expected fifth-newest sidecar entry");
}

#[test]
fn lockfile_sidecar_tracks_sixth_newest_for_current_entry() {
    let mut matcher = MatchGenerator::new(128 * 1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    matcher.file_profile_hint = CompressionFileProfile::CargoLock;
    let package = b"[[package]]\nname = \"crate\"\nversion = \"1.0.0\"\nchecksum = \"0123456789abcdef\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\ndependencies = [\n \"dep\",\n]\n\n";
    let data = package.repeat(BEST_SECOND_NEWEST_MIN_BLOCK_LEN / package.len() + 1);
    matcher.add_data(data, SuffixStore::with_capacity(64), |_, _| {});

    let key_value = SuffixStore::key_value(&matcher.last_entry().data[0..MIN_MATCH_LEN]);
    matcher.add_suffix_at(0);
    matcher.add_suffix_at(package.len());
    matcher.add_suffix_at(package.len() * 2);
    matcher.add_suffix_at(package.len() * 3);
    matcher.add_suffix_at(package.len() * 4);
    matcher.add_suffix_at(package.len() * 5);
    matcher.add_suffix_at(package.len() * 6);

    let slot_index = matcher.last_entry().suffixes.slot_key(key_value).index;
    let stored = matcher.current_sixth_newest_sidecar[slot_index];
    assert!(stored.is_some(), "expected sixth-newest sidecar entry");
}

#[test]
fn lockfile_sidecar_tracks_seventh_newest_for_current_entry() {
    let mut matcher = MatchGenerator::new(128 * 1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    matcher.file_profile_hint = CompressionFileProfile::CargoLock;
    let package = b"[[package]]\nname = \"crate\"\nversion = \"1.0.0\"\nchecksum = \"0123456789abcdef\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\ndependencies = [\n \"dep\",\n]\n\n";
    let data = package.repeat(BEST_SECOND_NEWEST_MIN_BLOCK_LEN / package.len() + 1);
    matcher.add_data(data, SuffixStore::with_capacity(64), |_, _| {});

    let key_value = SuffixStore::key_value(&matcher.last_entry().data[0..MIN_MATCH_LEN]);
    matcher.add_suffix_at(0);
    matcher.add_suffix_at(package.len());
    matcher.add_suffix_at(package.len() * 2);
    matcher.add_suffix_at(package.len() * 3);
    matcher.add_suffix_at(package.len() * 4);
    matcher.add_suffix_at(package.len() * 5);
    matcher.add_suffix_at(package.len() * 6);
    matcher.add_suffix_at(package.len() * 7);

    let slot_index = matcher.last_entry().suffixes.slot_key(key_value).index;
    let stored = matcher.current_seventh_newest_sidecar[slot_index];
    assert!(stored.is_some(), "expected seventh-newest sidecar entry");
}

#[test]
fn lockfile_sidecar_tracks_eighth_newest_for_current_entry() {
    let mut matcher = MatchGenerator::new(128 * 1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    matcher.file_profile_hint = CompressionFileProfile::CargoLock;
    let package = b"[[package]]\nname = \"crate\"\nversion = \"1.0.0\"\nchecksum = \"0123456789abcdef\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\ndependencies = [\n \"dep\",\n]\n\n";
    let data = package.repeat(BEST_SECOND_NEWEST_MIN_BLOCK_LEN / package.len() + 1);
    matcher.add_data(data, SuffixStore::with_capacity(64), |_, _| {});

    let key_value = SuffixStore::key_value(&matcher.last_entry().data[0..MIN_MATCH_LEN]);
    matcher.add_suffix_at(0);
    matcher.add_suffix_at(package.len());
    matcher.add_suffix_at(package.len() * 2);
    matcher.add_suffix_at(package.len() * 3);
    matcher.add_suffix_at(package.len() * 4);
    matcher.add_suffix_at(package.len() * 5);
    matcher.add_suffix_at(package.len() * 6);
    matcher.add_suffix_at(package.len() * 7);
    matcher.add_suffix_at(package.len() * 8);

    let slot_index = matcher.last_entry().suffixes.slot_key(key_value).index;
    let stored = matcher.current_eighth_newest_sidecar[slot_index];
    assert!(stored.is_some(), "expected eighth-newest sidecar entry");
}

#[test]
fn lockfile_sidecar_tracks_ninth_newest_for_current_entry() {
    let mut matcher = MatchGenerator::new(128 * 1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    matcher.file_profile_hint = CompressionFileProfile::CargoLock;
    let package = b"[[package]]\nname = \"crate\"\nversion = \"1.0.0\"\nchecksum = \"0123456789abcdef\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\ndependencies = [\n \"dep\",\n]\n\n";
    let data = package.repeat(BEST_SECOND_NEWEST_MIN_BLOCK_LEN / package.len() + 1);
    matcher.add_data(data, SuffixStore::with_capacity(64), |_, _| {});

    let key_value = SuffixStore::key_value(&matcher.last_entry().data[0..MIN_MATCH_LEN]);
    matcher.add_suffix_at(0);
    matcher.add_suffix_at(package.len());
    matcher.add_suffix_at(package.len() * 2);
    matcher.add_suffix_at(package.len() * 3);
    matcher.add_suffix_at(package.len() * 4);
    matcher.add_suffix_at(package.len() * 5);
    matcher.add_suffix_at(package.len() * 6);
    matcher.add_suffix_at(package.len() * 7);
    matcher.add_suffix_at(package.len() * 8);
    matcher.add_suffix_at(package.len() * 9);

    let slot_index = matcher.last_entry().suffixes.slot_key(key_value).index;
    let stored = matcher.current_ninth_newest_sidecar[slot_index];
    assert!(stored.is_some(), "expected ninth-newest sidecar entry");
}

#[test]
fn lockfile_sidecar_tracks_tenth_newest_for_current_entry() {
    let mut matcher = MatchGenerator::new(128 * 1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    matcher.file_profile_hint = CompressionFileProfile::CargoLock;
    let package = b"[[package]]\nname = \"crate\"\nversion = \"1.0.0\"\nchecksum = \"0123456789abcdef\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\ndependencies = [\n \"dep\",\n]\n\n";
    let data = package.repeat(BEST_SECOND_NEWEST_MIN_BLOCK_LEN / package.len() + 1);
    matcher.add_data(data, SuffixStore::with_capacity(64), |_, _| {});

    let key_value = SuffixStore::key_value(&matcher.last_entry().data[0..MIN_MATCH_LEN]);
    matcher.add_suffix_at(0);
    matcher.add_suffix_at(package.len());
    matcher.add_suffix_at(package.len() * 2);
    matcher.add_suffix_at(package.len() * 3);
    matcher.add_suffix_at(package.len() * 4);
    matcher.add_suffix_at(package.len() * 5);
    matcher.add_suffix_at(package.len() * 6);
    matcher.add_suffix_at(package.len() * 7);
    matcher.add_suffix_at(package.len() * 8);
    matcher.add_suffix_at(package.len() * 9);
    matcher.add_suffix_at(package.len() * 10);

    let slot_index = matcher.last_entry().suffixes.slot_key(key_value).index;
    let stored = matcher.current_tenth_newest_sidecar[slot_index];
    assert!(stored.is_some(), "expected tenth-newest sidecar entry");
}

#[test]
fn lockfile_sidecar_tracks_eleventh_newest_for_current_entry() {
    let mut matcher = MatchGenerator::new(128 * 1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    matcher.file_profile_hint = CompressionFileProfile::CargoLock;
    let package = b"[[package]]\nname = \"crate\"\nversion = \"1.0.0\"\nchecksum = \"0123456789abcdef\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\ndependencies = [\n \"dep\",\n]\n\n";
    let data = package.repeat(BEST_SECOND_NEWEST_MIN_BLOCK_LEN / package.len() + 1);
    matcher.add_data(data, SuffixStore::with_capacity(64), |_, _| {});

    let key_value = SuffixStore::key_value(&matcher.last_entry().data[0..MIN_MATCH_LEN]);
    matcher.add_suffix_at(0);
    matcher.add_suffix_at(package.len());
    matcher.add_suffix_at(package.len() * 2);
    matcher.add_suffix_at(package.len() * 3);
    matcher.add_suffix_at(package.len() * 4);
    matcher.add_suffix_at(package.len() * 5);
    matcher.add_suffix_at(package.len() * 6);
    matcher.add_suffix_at(package.len() * 7);
    matcher.add_suffix_at(package.len() * 8);
    matcher.add_suffix_at(package.len() * 9);
    matcher.add_suffix_at(package.len() * 10);
    matcher.add_suffix_at(package.len() * 11);

    let slot_index = matcher.last_entry().suffixes.slot_key(key_value).index;
    let stored = matcher.current_eleventh_newest_sidecar[slot_index];
    assert!(stored.is_some(), "expected eleventh-newest sidecar entry");
}

#[test]
fn lockfile_sidecar_tracks_twelfth_newest_for_current_entry() {
    let mut matcher = MatchGenerator::new(128 * 1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    matcher.file_profile_hint = CompressionFileProfile::CargoLock;
    let package = b"[[package]]\nname = \"crate\"\nversion = \"1.0.0\"\nchecksum = \"0123456789abcdef\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\ndependencies = [\n \"dep\",\n]\n\n";
    let data = package.repeat(BEST_SECOND_NEWEST_MIN_BLOCK_LEN / package.len() + 1);
    matcher.add_data(data, SuffixStore::with_capacity(64), |_, _| {});

    let key_value = SuffixStore::key_value(&matcher.last_entry().data[0..MIN_MATCH_LEN]);
    matcher.add_suffix_at(0);
    matcher.add_suffix_at(package.len());
    matcher.add_suffix_at(package.len() * 2);
    matcher.add_suffix_at(package.len() * 3);
    matcher.add_suffix_at(package.len() * 4);
    matcher.add_suffix_at(package.len() * 5);
    matcher.add_suffix_at(package.len() * 6);
    matcher.add_suffix_at(package.len() * 7);
    matcher.add_suffix_at(package.len() * 8);
    matcher.add_suffix_at(package.len() * 9);
    matcher.add_suffix_at(package.len() * 10);
    matcher.add_suffix_at(package.len() * 11);
    matcher.add_suffix_at(package.len() * 12);

    let slot_index = matcher.last_entry().suffixes.slot_key(key_value).index;
    let stored = matcher.current_twelfth_newest_sidecar[slot_index];
    assert!(stored.is_some(), "expected twelfth-newest sidecar entry");
}

#[test]
fn lockfile_sidecar_tracks_thirteenth_newest_for_current_entry() {
    let mut matcher = MatchGenerator::new(128 * 1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    matcher.file_profile_hint = CompressionFileProfile::CargoLock;
    let package = b"[[package]]\nname = \"crate\"\nversion = \"1.0.0\"\nchecksum = \"0123456789abcdef\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\ndependencies = [\n \"dep\",\n]\n\n";
    let data = package.repeat(BEST_SECOND_NEWEST_MIN_BLOCK_LEN / package.len() + 1);
    matcher.add_data(data, SuffixStore::with_capacity(64), |_, _| {});

    let key_value = SuffixStore::key_value(&matcher.last_entry().data[0..MIN_MATCH_LEN]);
    matcher.add_suffix_at(0);
    matcher.add_suffix_at(package.len());
    matcher.add_suffix_at(package.len() * 2);
    matcher.add_suffix_at(package.len() * 3);
    matcher.add_suffix_at(package.len() * 4);
    matcher.add_suffix_at(package.len() * 5);
    matcher.add_suffix_at(package.len() * 6);
    matcher.add_suffix_at(package.len() * 7);
    matcher.add_suffix_at(package.len() * 8);
    matcher.add_suffix_at(package.len() * 9);
    matcher.add_suffix_at(package.len() * 10);
    matcher.add_suffix_at(package.len() * 11);
    matcher.add_suffix_at(package.len() * 12);
    matcher.add_suffix_at(package.len() * 13);

    let slot_index = matcher.last_entry().suffixes.slot_key(key_value).index;
    let stored = matcher.current_thirteenth_newest_sidecar[slot_index];
    assert!(stored.is_some(), "expected thirteenth-newest sidecar entry");
}

#[test]
fn best_sidecar_is_disabled_for_small_blocks() {
    let mut matcher = MatchGenerator::new(1024);
    matcher.use_second_newest_probe = true;
    matcher.add_data(
        b"abcdeabcdeabcdeabcde".to_vec(),
        SuffixStore::with_capacity(64),
        |_, _| {},
    );

    assert!(matcher.current_second_newest_sidecar.is_empty());
    assert!(!matcher.should_track_second_newest_for_current_entry());
}

#[test]
fn dictionary_lockfile_text_tracks_second_newest_for_current_entry() {
    let mut matcher = MatchGenerator::new(128 * 1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    let package = b"[[package]]\nname = \"crate\"\nversion = \"1.0.0\"\nchecksum = \"0123456789abcdef\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\ndependencies = [\n \"dep\",\n]\n\n";
    let data = package.repeat(BEST_SECOND_NEWEST_MIN_BLOCK_LEN / package.len() + 1);
    matcher.add_data(data, SuffixStore::with_capacity(64), |_, _| {});

    assert!(!matcher.current_second_newest_sidecar.is_empty());
    assert!(matcher.should_track_second_newest_for_current_entry());
    assert!(matcher.prefer_lockfile_second_newest_before_newest());
    assert_eq!(
        matcher.no_match_probe_step(0),
        SHORT_LINE_TEXT_NO_MATCH_PROBE_STEP
    );
}

#[test]
fn cargo_lock_profile_marks_dictionary_lockfile_block_without_content_heuristic() {
    let mut matcher = MatchGenerator::new(32 * 1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    matcher.file_profile_hint = CompressionFileProfile::CargoLock;
    let data = b"name = \"crate\"\nversion = \"1.0.0\"\nsource = \"registry\"\n"
        .repeat(BEST_SECOND_NEWEST_MIN_BLOCK_LEN / 44 + 8);
    matcher.add_data(data, SuffixStore::with_capacity(64), |_, _| {});

    assert!(matcher.current_block_is_dictionary_lockfile_text);
}

#[test]
fn composer_lock_profile_marks_composer_block_without_content_heuristic() {
    let mut matcher = MatchGenerator::new(32 * 1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    matcher.file_profile_hint = CompressionFileProfile::ComposerLock;
    let data = b"name = \"vendor/package\"\nversion = \"1.0.0\"\nsource = \"git\"\n"
        .repeat(BEST_SECOND_NEWEST_MIN_BLOCK_LEN / 46 + 8);
    matcher.add_data(data, SuffixStore::with_capacity(64), |_, _| {});

    assert!(matcher.current_block_is_composer_dictionary_text);
    assert!(!matcher.current_block_is_dictionary_lockfile_text);
}

#[test]
fn dictionary_text_tracks_second_newest_for_current_entry() {
    let mut matcher = MatchGenerator::new(128 * 1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    let data = b"dictionary-entry\n".repeat(BEST_SECOND_NEWEST_MIN_BLOCK_LEN / 17 + 1);
    matcher.add_data(data, SuffixStore::with_capacity(64), |_, _| {});

    assert!(!matcher.current_second_newest_sidecar.is_empty());
    assert!(matcher.should_track_second_newest_for_current_entry());
    assert!(!matcher.prefer_lockfile_second_newest_before_newest());
}

#[test]
fn code_text_tracks_second_newest_for_current_entry() {
    let mut matcher = MatchGenerator::new(CODE_TEXT_SECOND_NEWEST_MAX_BLOCK_LEN + 1);
    matcher.file_type_hint = CompressionFileType::CodeText;
    let data = b"def build_fixture_manifest(root: Path, entries: list[str]) -> None:\n"
        .repeat(BEST_SECOND_NEWEST_MIN_BLOCK_LEN / 67 + 64);
    matcher.add_data(data, SuffixStore::with_capacity(64), |_, _| {});

    assert!(matcher.is_short_line_text);
    assert!(!matcher.current_second_newest_sidecar.is_empty());
    assert!(matcher.should_track_second_newest_for_current_entry());
    assert!(!matcher.prefer_lockfile_second_newest_before_newest());
}

#[test]
fn large_code_text_disables_second_newest_sidecar() {
    let mut matcher = MatchGenerator::new(CODE_TEXT_DENSE_PROBE_MAX_BLOCK_LEN + 1);
    matcher.file_type_hint = CompressionFileType::CodeText;
    let data = b"def build_fixture_manifest(root: Path, entries: list[str]) -> None:\n"
        .repeat(CODE_TEXT_SECOND_NEWEST_MAX_BLOCK_LEN / 67 + 256);
    matcher.add_data(data, SuffixStore::with_capacity(64), |_, _| {});

    assert!(matcher.is_short_line_text);
    assert!(matcher.last_entry().data.len() > CODE_TEXT_SECOND_NEWEST_MAX_BLOCK_LEN);
    assert!(matcher.current_second_newest_sidecar.is_empty());
    assert!(!matcher.should_track_second_newest_for_current_entry());
}

#[test]
fn dictionary_binary_does_not_track_second_newest_without_best_probe() {
    let mut matcher = MatchGenerator::new(128 * 1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    matcher.add_data(
        alloc::vec![0u8; BEST_SECOND_NEWEST_MIN_BLOCK_LEN],
        SuffixStore::with_capacity(64),
        |_, _| {},
    );

    assert!(matcher.current_second_newest_sidecar.is_empty());
    assert!(!matcher.should_track_second_newest_for_current_entry());
    assert!(!matcher.prefer_lockfile_second_newest_before_newest());
}

#[test]
fn lockfile_text_can_keep_current_candidate_over_farther_oldest() {
    let mut matcher = MatchGenerator::new(128 * 1024);
    matcher.file_type_hint = CompressionFileType::DictionaryText;
    matcher.current_block_is_dictionary_lockfile_text = true;

    let current = MatchCandidate {
        start_idx: 100,
        offset: 24,
        match_len: 10,
        repeat_offset: false,
        #[cfg(test)]
        source: CandidateSource::WindowCurrentSecondNewest { entry_distance: 0 },
    };
    let found = MatchCandidate {
        start_idx: 100,
        offset: 64,
        match_len: 11,
        repeat_offset: false,
        #[cfg(test)]
        source: CandidateSource::WindowCurrentOldest { entry_distance: 0 },
    };

    assert!(matcher.keep_lockfile_current_candidate_over_oldest(
        found,
        current,
        WindowCandidateMeta {
            entry_distance: 0,
            kind: WindowCandidateKind::Oldest,
        },
    ));
}

#[test]
fn best_current_long_hash_tracks_latest_current_entry_index() {
    let mut matcher = MatchGenerator::new(128 * 1024);
    matcher.use_second_newest_probe = true;
    let data = [0u8, 1, 2, 3, 4, 5, 6, 7].repeat(BEST_CURRENT_LONG_HASH_MIN_BLOCK_LEN / 8 + 1);
    matcher.add_data(data, SuffixStore::with_capacity(64), |_, _| {});

    matcher.add_suffix_at(0);
    matcher.add_suffix_at(8);
    matcher.add_suffix_at(16);

    let long_key = u64::from_le_bytes(
        matcher.last_entry().data[0..8]
            .try_into()
            .expect("need 8 bytes"),
    );
    let slot_index = matcher.last_entry().suffixes.slot_key(long_key).index;
    let stored = matcher.current_long_hash[slot_index];
    assert_eq!(stored.map(|idx| idx.get() as usize - 1), Some(16));
    assert!(matcher.should_track_current_long_hash());
}

#[test]
fn best_current_long_hash_is_disabled_below_threshold() {
    let mut matcher = MatchGenerator::new(BEST_CURRENT_LONG_HASH_MIN_BLOCK_LEN);
    matcher.use_second_newest_probe = true;
    matcher.add_data(
        alloc::vec![0u8; BEST_CURRENT_LONG_HASH_MIN_BLOCK_LEN - 1],
        SuffixStore::with_capacity(64),
        |_, _| {},
    );

    assert!(matcher.current_long_hash.is_empty());
    assert!(!matcher.should_track_current_long_hash());
}

#[test]
fn sparse_next_position_match_preserves_start_index() {
    let mut matcher = MatchGenerator::new(1024);
    matcher.add_data(xorshift(512), SuffixStore::with_capacity(1024), |_, _| {});
    matcher.suffix_idx = 10;

    matcher.emit_candidate(
        MatchCandidate {
            start_idx: 11,
            offset: 32,
            match_len: DENSE_MATCH_INDEX_LIMIT + 1,
            repeat_offset: true,
            #[cfg(test)]
            source: CandidateSource::RepeatNextPosition(RepeatCandidateKind::First),
        },
        &mut |_| {},
    );

    let last_entry = matcher.last_entry();
    let key = &last_entry.data[11..11 + MIN_MATCH_LEN];
    let candidates = last_entry
        .suffixes
        .candidates(key)
        .expect("next-position sparse match should keep start index");
    assert_eq!(candidates.oldest, 11);
    assert_eq!(candidates.newest, None);
}

#[test]
fn matches() {
    let mut matcher = MatchGenerator::new(1000);
    let mut original_data = Vec::new();
    let mut reconstructed = Vec::new();

    let reconstruct = |seq: Sequence<'_>, reconstructed: &mut Vec<u8>| match seq {
        Sequence::Literals { literals } => reconstructed.extend_from_slice(literals),
        Sequence::Triple {
            literals,
            offset,
            match_len,
        } => {
            reconstructed.extend_from_slice(literals);
            let start = reconstructed.len() - offset;
            for idx in 0..match_len {
                let byte = reconstructed[start + idx];
                reconstructed.push(byte);
            }
        }
    };
    let assert_seq_equal =
        |seq: Sequence<'_>, expected: Sequence<'_>, reconstructed: &mut Vec<u8>| {
            assert_eq!(seq, expected);
            reconstruct(seq, reconstructed);
        };

    matcher.add_data(
        alloc::vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        SuffixStore::with_capacity(100),
        |_, _| {},
    );
    original_data.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

    matcher.next_sequence(|seq| {
        assert_eq!(
            seq,
            Sequence::Triple {
                literals: &[0],
                offset: 1,
                match_len: 9,
            },
        );
        reconstruct(seq, &mut reconstructed);
    });

    assert!(!matcher.next_sequence(|_| {}));

    matcher.add_data(
        alloc::vec![1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6, 0, 0, 0, 0, 0,],
        SuffixStore::with_capacity(100),
        |_, _| {},
    );
    original_data.extend_from_slice(&[
        1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6, 0, 0, 0, 0, 0,
    ]);

    matcher.next_sequence(|seq| {
        assert_seq_equal(
            seq,
            Sequence::Triple {
                literals: &[1, 2, 3, 4, 5, 6],
                offset: 6,
                match_len: 12,
            },
            &mut reconstructed,
        );
    });
    matcher.next_sequence(|seq| {
        assert_seq_equal(
            seq,
            Sequence::Triple {
                literals: &[],
                offset: 23,
                match_len: 5,
            },
            &mut reconstructed,
        );
    });
    assert!(!matcher.next_sequence(|_| {}));

    matcher.add_data(
        alloc::vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 0, 0, 0, 0],
        SuffixStore::with_capacity(100),
        |_, _| {},
    );
    original_data.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 0, 0, 0, 0]);

    matcher.next_sequence(|seq| {
        assert_seq_equal(
            seq,
            Sequence::Triple {
                literals: &[],
                offset: 11,
                match_len: 6,
            },
            &mut reconstructed,
        );
    });
    matcher.next_sequence(|seq| {
        assert_seq_equal(
            seq,
            Sequence::Triple {
                literals: &[7, 8, 9, 10, 11],
                offset: 16,
                match_len: 5,
            },
            &mut reconstructed,
        );
    });
    assert!(!matcher.next_sequence(|_| {}));

    matcher.add_data(
        alloc::vec![0, 0, 0, 0, 0],
        SuffixStore::with_capacity(100),
        |_, _| {},
    );
    original_data.extend_from_slice(&[0, 0, 0, 0, 0]);

    matcher.next_sequence(|seq| {
        assert_seq_equal(
            seq,
            Sequence::Triple {
                literals: &[],
                offset: 5,
                match_len: 5,
            },
            &mut reconstructed,
        );
    });
    assert!(!matcher.next_sequence(|_| {}));

    matcher.add_data(
        alloc::vec![7, 8, 9, 10, 11],
        SuffixStore::with_capacity(100),
        |_, _| {},
    );
    original_data.extend_from_slice(&[7, 8, 9, 10, 11]);

    matcher.next_sequence(|seq| {
        assert_seq_equal(
            seq,
            Sequence::Triple {
                literals: &[],
                offset: 15,
                match_len: 5,
            },
            &mut reconstructed,
        );
    });
    assert!(!matcher.next_sequence(|_| {}));

    matcher.add_data(
        alloc::vec![1, 3, 5, 7, 9],
        SuffixStore::with_capacity(100),
        |_, _| {},
    );
    matcher.skip_matching();
    original_data.extend_from_slice(&[1, 3, 5, 7, 9]);
    reconstructed.extend_from_slice(&[1, 3, 5, 7, 9]);
    assert!(!matcher.next_sequence(|_| {}));

    matcher.add_data(
        alloc::vec![1, 3, 5, 7, 9],
        SuffixStore::with_capacity(100),
        |_, _| {},
    );
    original_data.extend_from_slice(&[1, 3, 5, 7, 9]);

    matcher.next_sequence(|seq| {
        assert_seq_equal(
            seq,
            Sequence::Triple {
                literals: &[],
                offset: 5,
                match_len: 5,
            },
            &mut reconstructed,
        );
    });
    assert!(!matcher.next_sequence(|_| {}));

    matcher.add_data(
        alloc::vec![31, 32, 33, 34, 35],
        SuffixStore::with_capacity(100),
        |_, _| {},
    );
    matcher.skip_matching_for_incompressible();
    original_data.extend_from_slice(&[31, 32, 33, 34, 35]);
    reconstructed.extend_from_slice(&[31, 32, 33, 34, 35]);
    assert!(!matcher.next_sequence(|_| {}));

    matcher.add_data(
        alloc::vec![31, 32, 33, 34, 35],
        SuffixStore::with_capacity(100),
        |_, _| {},
    );
    original_data.extend_from_slice(&[31, 32, 33, 34, 35]);

    matcher.next_sequence(|seq| {
        assert_seq_equal(
            seq,
            Sequence::Literals {
                literals: &[31, 32, 33, 34, 35],
            },
            &mut reconstructed,
        );
    });
    assert!(!matcher.next_sequence(|_| {}));

    matcher.add_data(
        alloc::vec![0, 0, 11, 13, 15, 17, 20, 11, 13, 15, 17, 20, 21, 23],
        SuffixStore::with_capacity(100),
        |_, _| {},
    );
    original_data.extend_from_slice(&[0, 0, 11, 13, 15, 17, 20, 11, 13, 15, 17, 20, 21, 23]);

    matcher.next_sequence(|seq| {
        assert_seq_equal(
            seq,
            Sequence::Triple {
                literals: &[0, 0, 11, 13, 15, 17, 20],
                offset: 5,
                match_len: 5,
            },
            &mut reconstructed,
        );
    });
    matcher.next_sequence(|seq| {
        assert_seq_equal(
            seq,
            Sequence::Literals {
                literals: &[21, 23],
            },
            &mut reconstructed,
        );
    });
    assert!(!matcher.next_sequence(|_| {}));

    assert_eq!(reconstructed, original_data);
}

#[cfg(test)]
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
