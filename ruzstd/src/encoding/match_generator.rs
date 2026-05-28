//! Matching algorithm used find repeated parts in the original data
//!
//! The Zstd format relies on finden repeated sequences of data and compressing these sequences as instructions to the decoder.
//! A sequence basically tells the decoder "Go back X bytes and copy Y bytes to the end of your decode buffer".
//!
//! The task here is to efficiently find matches in the already encoded data for the current suffix of the not yet encoded data.

use alloc::vec::Vec;
use core::convert::TryFrom;
use core::num::NonZeroU32;

use super::frame_compressor::OffsetHistory;
use super::CompressionLevel;
use super::Matcher;
use super::Sequence;

const MIN_MATCH_LEN: usize = 5;
const TEXT_MIN_NON_REPEAT_MATCH_LEN: usize = 10;
const REPEAT_MATCH_LEN_MARGIN: usize = 2;
const REPEAT_SEARCH_EARLY_EXIT_LEN: usize = 10;
const DENSE_MATCH_INDEX_LIMIT: usize = 128;
const NO_MATCH_PROBE_STEP: usize = 2;
const TEXT_NO_MATCH_PROBE_STEP: usize = 3;

/// This is the default implementation of the `Matcher` trait. It allocates and reuses the buffers when possible.
pub struct MatchGeneratorDriver {
    vec_pool: Vec<Vec<u8>>,
    suffix_pool: Vec<SuffixStore>,
    match_generator: MatchGenerator,
    slice_size: usize,
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
        }
    }

    #[cfg(test)]
    pub(crate) fn repeat_offsets(&self) -> (u32, u32, u32) {
        self.match_generator.offset_history.as_offsets()
    }
}

impl Matcher for MatchGeneratorDriver {
    fn reset(&mut self, _level: CompressionLevel) {
        let vec_pool = &mut self.vec_pool;
        let suffix_pool = &mut self.suffix_pool;

        self.match_generator.reset(|mut data, mut suffixes| {
            data.resize(data.capacity(), 0);
            vec_pool.push(data);
            suffixes.slots.clear();
            suffixes.slots.resize(suffixes.slots.capacity(), None);
            suffix_pool.push(suffixes);
        });
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

    fn get_last_space(&mut self) -> &[u8] {
        self.match_generator.last_entry().data.as_slice()
    }

    fn commit_space(&mut self, space: Vec<u8>) {
        let vec_pool = &mut self.vec_pool;
        let suffixes = match self.suffix_pool.pop() {
            Some(suffixes) => suffixes,
            None => SuffixStore::with_capacity(space.len()),
        };
        let suffix_pool = &mut self.suffix_pool;
        self.match_generator
            .add_data(space, suffixes, |mut data, mut suffixes| {
                data.resize(data.capacity(), 0);
                vec_pool.push(data);
                suffixes.slots.clear();
                suffixes.slots.resize(suffixes.slots.capacity(), None);
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
}

/// This stores the index of a suffix of a string by hashing the first few bytes of that suffix
/// This means that collisions just overwrite and that you need to check validity after a get
struct SuffixStore {
    slots: Vec<Option<Candidates>>,
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
        Self {
            slots: alloc::vec![None; capacity],
            len_log: capacity.ilog2(),
        }
    }

    #[inline(always)]
    fn insert(&mut self, suffix: &[u8], idx: usize) {
        let key = self.key(suffix);
        let idx = Self::stored_index(idx);
        if let Some(slot) = self.slots[key] {
            self.slots[key] = Some(Candidates {
                oldest: slot.oldest,
                newest: idx,
            });
        } else {
            self.slots[key] = Some(Candidates {
                oldest: idx,
                newest: idx,
            });
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

    #[inline(always)]
    fn candidates(&self, suffix: &[u8]) -> Option<CandidateIndexes> {
        let key = self.key(suffix);
        let slot = self.slots[key]?;
        let oldest = slot.oldest.get() as usize - 1;
        let newest = slot.newest.get() as usize - 1;
        let newest = if oldest == newest { None } else { Some(newest) };
        Some(CandidateIndexes { oldest, newest })
    }

    #[inline(always)]
    fn key(&self, suffix: &[u8]) -> usize {
        let value = u64::from(suffix[0])
            | (u64::from(suffix[1]) << 8)
            | (u64::from(suffix[2]) << 16)
            | (u64::from(suffix[3]) << 24)
            | (u64::from(suffix[4]) << 32);
        let index = value.wrapping_mul(0x9E37_79B1_85EB_CA87);
        let index = index >> (64 - self.len_log);
        index as usize % self.slots.len()
    }
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
struct MatchCandidate {
    start_idx: usize,
    offset: usize,
    match_len: usize,
    repeat_offset: bool,
}

impl MatchCandidate {
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

    fn can_skip_window_search(self, block_len: usize) -> bool {
        self.repeat_offset
            && (self.start_idx + self.match_len == block_len
                || self.match_len >= REPEAT_SEARCH_EARLY_EXIT_LEN)
    }
}

pub(crate) struct MatchGenerator {
    max_window_size: usize,
    /// Data window we are operating on to find matches
    /// The data we want to find matches for is in the last slice
    window: Vec<WindowEntry>,
    window_size: usize,
    #[cfg(debug_assertions)]
    concat_window: Vec<u8>,
    /// Index in the last slice that we already processed
    suffix_idx: usize,
    /// Gets updated when a new sequence is returned to point right behind that sequence
    last_idx_in_sequence: usize,
    offset_history: OffsetHistory,
    min_non_repeat_match_len: usize,
}

impl MatchGenerator {
    /// max_size defines how many bytes will be used at most in the window used for matching
    fn new(max_size: usize) -> Self {
        Self {
            max_window_size: max_size,
            window: Vec::new(),
            window_size: 0,
            #[cfg(debug_assertions)]
            concat_window: Vec::new(),
            suffix_idx: 0,
            last_idx_in_sequence: 0,
            offset_history: OffsetHistory::new(),
            min_non_repeat_match_len: MIN_MATCH_LEN,
        }
    }

    fn reset(&mut self, mut reuse_space: impl FnMut(Vec<u8>, SuffixStore)) {
        self.window_size = 0;
        #[cfg(debug_assertions)]
        self.concat_window.clear();
        self.suffix_idx = 0;
        self.last_idx_in_sequence = 0;
        self.offset_history = OffsetHistory::new();
        self.min_non_repeat_match_len = MIN_MATCH_LEN;
        self.window.drain(..).for_each(|entry| {
            reuse_space(entry.data, entry.suffixes);
        });
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
        loop {
            let last_entry_idx = self.last_entry_index();
            let last_entry = &self.window[last_entry_idx];
            let data_slice = &last_entry.data;

            // We already reached the end of the window, check if we need to return a Literals{}
            if self.suffix_idx >= data_slice.len() {
                if self.last_idx_in_sequence != self.suffix_idx {
                    let literals = &data_slice[self.last_idx_in_sequence..];
                    self.last_idx_in_sequence = self.suffix_idx;
                    handle_sequence(Sequence::Literals { literals });
                    return true;
                } else {
                    return false;
                }
            }

            // If the remaining data is smaller than the minimum match length we can stop and return a Literals{}
            let data_slice = &data_slice[self.suffix_idx..];
            if data_slice.len() < MIN_MATCH_LEN {
                let last_idx_in_sequence = self.last_idx_in_sequence;
                self.last_idx_in_sequence = last_entry.data.len();
                self.suffix_idx = last_entry.data.len();
                handle_sequence(Sequence::Literals {
                    literals: &last_entry.data[last_idx_in_sequence..],
                });
                return true;
            }

            // This is the key we are looking to find a match for
            let key = &data_slice[..MIN_MATCH_LEN];

            // Look in each window entry
            let mut candidate = None;
            let match_context = MatchCandidateContext {
                suffix_idx: self.suffix_idx,
                anchor_idx: self.last_idx_in_sequence,
                min_non_repeat_match_len: self.min_non_repeat_match_len,
                data_slice,
                #[cfg(debug_assertions)]
                last_entry_len: last_entry.data.len(),
                #[cfg(debug_assertions)]
                concat_window: &self.concat_window,
            };

            let literal_len = self.suffix_idx - self.last_idx_in_sequence;
            for offset in self.repeat_offset_candidates(literal_len) {
                if offset == 0 {
                    continue;
                }
                if !self.has_min_match_at_offset(offset, &match_context) {
                    continue;
                }
                let match_len = self.match_len_at_offset(offset, &match_context);
                if match_len >= MIN_MATCH_LEN {
                    let found = MatchCandidate {
                        start_idx: self.suffix_idx,
                        offset,
                        match_len,
                        repeat_offset: true,
                    };
                    if candidate
                        .map(|current| found.is_better_than(current))
                        .unwrap_or(true)
                    {
                        candidate = Some(found);
                    }
                }
            }

            let repeat_match_reaches_end_or_is_long =
                candidate.is_some_and(|found| found.can_skip_window_search(last_entry.data.len()));

            if !repeat_match_reaches_end_or_is_long {
                'window_search: for match_entry in self.window.iter() {
                    if let Some(candidates) = match_entry.suffixes.candidates(key) {
                        for match_index in candidates.newest.into_iter().chain([candidates.oldest])
                        {
                            let Some(found) =
                                self.match_candidate(match_entry, match_index, &match_context)
                            else {
                                continue;
                            };
                            if !found.worth_emitting(match_context.min_non_repeat_match_len) {
                                continue;
                            }

                            if candidate
                                .map(|current| found.is_better_than(current))
                                .unwrap_or(true)
                            {
                                candidate = Some(found);
                            }

                            if found.start_idx + found.match_len == last_entry.data.len()
                                && found.offset == 1
                            {
                                break 'window_search;
                            }
                        }
                    }
                }
            }

            if let Some(candidate) = candidate {
                let MatchCandidate {
                    start_idx,
                    offset,
                    match_len,
                    ..
                } = candidate;
                // For each index in the match we found we do not need to look for another match
                // But we still want them registered in the suffix store
                self.add_suffixes_for_match(start_idx + match_len);

                // All literals that were not included between this match and the last are now included here
                let last_entry_idx = self.last_entry_index();
                let last_entry = &self.window[last_entry_idx];
                let literals = &last_entry.data[self.last_idx_in_sequence..start_idx];
                let lit_len = Self::bounded_u32(literals.len());
                let offset_value = Self::bounded_u32(offset);
                self.offset_history
                    .encode_offset_value(offset_value, lit_len);

                // Update the indexes, all indexes upto and including the current index have been included in a sequence now
                self.suffix_idx = start_idx + match_len;
                self.last_idx_in_sequence = self.suffix_idx;
                handle_sequence(Sequence::Triple {
                    literals,
                    offset,
                    match_len,
                });

                return true;
            }

            let suffix_idx = self.suffix_idx;
            let last_entry_len = last_entry.data.len();
            let probe_step = self.no_match_probe_step();
            let can_skip_next_probe = suffix_idx + probe_step + MIN_MATCH_LEN <= last_entry_len
                && (1..probe_step).all(|skip| !self.repeat_offset_can_match_at(suffix_idx + skip));
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
    fn match_candidate(
        &self,
        match_entry: &WindowEntry,
        match_index: usize,
        context: &MatchCandidateContext<'_>,
    ) -> Option<MatchCandidate> {
        if !Self::has_min_match_at_index(match_entry, match_index, context) {
            return None;
        }

        let offset = match_entry.base_offset + context.suffix_idx - match_index;
        let match_len = self.match_len_at_offset(offset, context);
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
        })
    }

    #[inline(always)]
    fn repeat_offset_can_match_at(&self, suffix_idx: usize) -> bool {
        let literal_len = suffix_idx - self.last_idx_in_sequence;
        for offset in self.repeat_offset_candidates(literal_len) {
            if offset != 0 && self.has_min_match_at_index_offset(suffix_idx, offset) {
                return true;
            }
        }
        false
    }

    #[inline(always)]
    fn no_match_probe_step(&self) -> usize {
        if self.min_non_repeat_match_len == TEXT_MIN_NON_REPEAT_MATCH_LEN {
            TEXT_NO_MATCH_PROBE_STEP
        } else {
            NO_MATCH_PROBE_STEP
        }
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
    fn repeat_offset_candidates(&self, literal_len: usize) -> [usize; 3] {
        if literal_len > 0 {
            [
                self.offset_history.newest as usize,
                self.offset_history.second as usize,
                self.offset_history.third as usize,
            ]
        } else {
            [
                self.offset_history.second as usize,
                self.offset_history.third as usize,
                self.offset_history.newest.saturating_sub(1) as usize,
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

    fn bounded_u32(value: usize) -> u32 {
        match u32::try_from(value) {
            Ok(value) => value,
            Err(_) => unreachable!("match generator indexes are bounded by the compressor window"),
        }
    }

    fn min_non_repeat_match_len(data: &[u8]) -> usize {
        if Self::likely_text(data) {
            TEXT_MIN_NON_REPEAT_MATCH_LEN
        } else {
            MIN_MATCH_LEN
        }
    }

    fn likely_text(data: &[u8]) -> bool {
        const SAMPLE_COUNT: usize = 256;

        if data.len() < 1024 {
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

    #[inline(always)]
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

    #[inline(always)]
    fn has_min_match_at_offset(&self, offset: usize, context: &MatchCandidateContext<'_>) -> bool {
        if offset == 0 {
            return false;
        }

        let source_relative = context.suffix_idx as isize - offset as isize;
        let Some(source) = self.slice_at_relative(source_relative) else {
            return false;
        };

        if source.len() < MIN_MATCH_LEN {
            return true;
        }

        source[..MIN_MATCH_LEN] == context.data_slice[..MIN_MATCH_LEN]
    }

    #[inline(always)]
    fn slice_at_relative(&self, relative_to_current: isize) -> Option<&[u8]> {
        if relative_to_current >= 0 {
            return self.last_entry().data.get(relative_to_current as usize..);
        }

        for entry in &self.window {
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

    /// Process bytes and add the suffixes to the suffix store up to a specific index
    #[inline(always)]
    fn add_suffixes_till(&mut self, idx: usize) {
        let suffix_idx = self.suffix_idx;
        let last_entry = self.last_entry_mut();
        if last_entry.data.len() < MIN_MATCH_LEN {
            return;
        }
        let slice = &last_entry.data[suffix_idx..idx];
        for (key_index, key) in slice.windows(MIN_MATCH_LEN).enumerate() {
            last_entry.suffixes.insert(key, suffix_idx + key_index);
        }
    }

    #[inline(always)]
    fn add_suffixes_for_match(&mut self, idx: usize) {
        if idx - self.suffix_idx <= DENSE_MATCH_INDEX_LIMIT {
            self.add_suffixes_till(idx);
            return;
        }

        let suffix_idx = self.suffix_idx;
        self.add_suffix_at(suffix_idx);
        self.add_suffix_at(suffix_idx + 2);
        self.add_suffix_at(idx.saturating_sub(MIN_MATCH_LEN));
    }

    #[inline(always)]
    fn add_suffix_at(&mut self, idx: usize) {
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
        self.reserve(data.len(), reuse_space);
        #[cfg(debug_assertions)]
        self.concat_window.extend_from_slice(&data);

        if let Some(last_len) = self.window.last().map(|last| last.data.len()) {
            for entry in self.window.iter_mut() {
                entry.base_offset += last_len;
            }
        }

        let len = data.len();
        let min_non_repeat_match_len = Self::min_non_repeat_match_len(&data);
        self.window.push(WindowEntry {
            data,
            suffixes,
            base_offset: 0,
        });
        self.window_size += len;
        self.suffix_idx = 0;
        self.last_idx_in_sequence = 0;
        self.min_non_repeat_match_len = min_non_repeat_match_len;
    }

    /// Reserve space for a new window entry
    /// If any resources are released by pushing the new entry they are returned via the callback
    fn reserve(&mut self, amount: usize, mut reuse_space: impl FnMut(Vec<u8>, SuffixStore)) {
        assert!(self.max_window_size >= amount);
        while self.window_size + amount > self.max_window_size {
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

    assert_eq!(matcher.repeat_offset_candidates(3), [7, 11, 13]);
}

#[test]
fn repeat_offset_candidates_shift_for_zero_literals() {
    let mut matcher = MatchGenerator::new(100);
    matcher.offset_history = OffsetHistory::from_offsets(7, 11, 13);

    assert_eq!(matcher.repeat_offset_candidates(0), [11, 13, 6]);
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
fn text_blocks_use_longer_non_repeat_match_minimum() {
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
        TEXT_MIN_NON_REPEAT_MATCH_LEN
    );
}

#[test]
fn binary_blocks_keep_short_non_repeat_matches() {
    let mut matcher = MatchGenerator::new(2048);
    matcher.add_data(xorshift(2048), SuffixStore::with_capacity(2048), |_, _| {});

    assert_eq!(matcher.min_non_repeat_match_len, MIN_MATCH_LEN);
}

#[test]
fn text_blocks_use_wider_no_match_probe_step() {
    let mut matcher = MatchGenerator::new(2048);
    matcher.add_data(
        b"tenant=alpha path=/v1/archive status=200\n"
            .repeat(32)
            .to_vec(),
        SuffixStore::with_capacity(2048),
        |_, _| {},
    );

    assert_eq!(matcher.no_match_probe_step(), TEXT_NO_MATCH_PROBE_STEP);
}

#[test]
fn binary_blocks_keep_default_no_match_probe_step() {
    let mut matcher = MatchGenerator::new(2048);
    matcher.add_data(xorshift(2048), SuffixStore::with_capacity(2048), |_, _| {});

    assert_eq!(matcher.no_match_probe_step(), NO_MATCH_PROBE_STEP);
}

#[test]
fn repeat_offset_candidate_can_win_with_small_length_deficit() {
    let repeat = MatchCandidate {
        start_idx: 10,
        offset: 16,
        match_len: 8,
        repeat_offset: true,
    };
    let normal = MatchCandidate {
        start_idx: 10,
        offset: 1024,
        match_len: 8 + REPEAT_MATCH_LEN_MARGIN,
        repeat_offset: false,
    };

    assert!(repeat.is_better_than(normal));
}

#[test]
fn longer_normal_candidate_wins_beyond_repeat_offset_margin() {
    let repeat = MatchCandidate {
        start_idx: 10,
        offset: 16,
        match_len: 8,
        repeat_offset: true,
    };
    let normal = MatchCandidate {
        start_idx: 10,
        offset: 1024,
        match_len: 8 + REPEAT_MATCH_LEN_MARGIN + 1,
        repeat_offset: false,
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
    };
    let normal = MatchCandidate {
        start_idx: 10,
        offset: 16,
        match_len: REPEAT_SEARCH_EARLY_EXIT_LEN,
        repeat_offset: false,
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

    assert!(matcher.repeat_offset_can_match_at(1));
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

    matcher.add_suffixes_for_match(10 + DENSE_MATCH_INDEX_LIMIT + 1);

    let indexed = matcher
        .last_entry()
        .suffixes
        .slots
        .iter()
        .filter(|slot| slot.is_some())
        .count();
    assert!(
        indexed <= 3,
        "long match should index sparsely: {}",
        indexed
    );
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
