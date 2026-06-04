//! Utilities and interfaces for encoding an entire frame. Allows reusing resources

use alloc::vec::Vec;
use core::convert::TryInto;
#[cfg(feature = "hash")]
use twox_hash::XxHash64;

#[cfg(feature = "hash")]
use core::hash::Hasher;

use super::{
    block_header::BlockHeader,
    frame_header::FrameHeader,
    levels::*,
    match_generator::MatchGeneratorDriver,
    util::{likely_incompressible, likely_text},
    CompressionFileProfile, CompressionFileType, CompressionLevel, Matcher,
};
use crate::fse::fse_encoder::{default_ll_table, default_ml_table, default_of_table, FSETable};

use crate::io::{Read, Write};

const BEST_SPLIT_CHUNK_SIZE: usize = 8 * 1024;
const BEST_SPLIT_COMPRESSIBLE_RUN_MAX: usize = 128 * 1024;
const BEST_SPLIT_MIN_INCOMPRESSIBLE_RUN_CHUNKS: usize = 1;

/// An interface for compressing arbitrary data with the ZStandard compression algorithm.
///
/// `FrameCompressor` will generally be used by:
/// 1. Initializing a compressor by providing a buffer of data using `FrameCompressor::new()`
/// 2. Starting compression and writing that compression into a vec using `FrameCompressor::begin`
///
/// # Examples
/// ```
/// use ruzstd::encoding::{FrameCompressor, CompressionLevel};
/// let mock_data: &[_] = &[0x1, 0x2, 0x3, 0x4];
/// let mut output = std::vec::Vec::new();
/// // Initialize a compressor.
/// let mut compressor = FrameCompressor::new(CompressionLevel::Uncompressed);
/// compressor.set_source(mock_data);
/// compressor.set_drain(&mut output);
///
/// // `compress` writes the compressed output into the provided buffer.
/// compressor.compress();
/// ```
pub struct FrameCompressor<R: Read, W: Write, M: Matcher> {
    uncompressed_data: Option<R>,
    compressed_data: Option<W>,
    compression_level: CompressionLevel,
    file_type_hint: CompressionFileType,
    file_profile_hint: CompressionFileProfile,
    state: CompressState<M>,
    #[cfg(feature = "hash")]
    hasher: XxHash64,
}

#[derive(Clone)]
pub(crate) struct FseTables {
    pub(crate) ll_default: FSETable,
    pub(crate) ll_previous: Option<FSETable>,
    pub(crate) ml_default: FSETable,
    pub(crate) ml_previous: Option<FSETable>,
    pub(crate) of_default: FSETable,
    pub(crate) of_previous: Option<FSETable>,
}

impl FseTables {
    pub fn new() -> Self {
        Self {
            ll_default: default_ll_table(),
            ll_previous: None,
            ml_default: default_ml_table(),
            ml_previous: None,
            of_default: default_of_table(),
            of_previous: None,
        }
    }

    pub fn reset(&mut self) {
        self.ll_previous = None;
        self.ml_previous = None;
        self.of_previous = None;
    }
}

pub(crate) struct CompressState<M: Matcher> {
    pub(crate) matcher: M,
    pub(crate) last_huff_table: Option<crate::huff0::huff0_encoder::HuffmanTable>,
    pub(crate) fse_tables: FseTables,
    pub(crate) offset_history: OffsetHistory,
    pub(crate) file_type_hint: CompressionFileType,
    pub(crate) file_profile_hint: CompressionFileProfile,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct OffsetHistory {
    pub(crate) newest: u32,
    pub(crate) second: u32,
    pub(crate) third: u32,
}

impl OffsetHistory {
    pub(crate) const fn new() -> Self {
        Self {
            newest: 1,
            second: 4,
            third: 8,
        }
    }

    pub(crate) const fn from_offsets(newest: u32, second: u32, third: u32) -> Self {
        Self {
            newest,
            second,
            third,
        }
    }

    pub(crate) fn as_offsets(self) -> (u32, u32, u32) {
        (self.newest, self.second, self.third)
    }

    pub(crate) fn encode_offset_value(&mut self, offset: u32, lit_len: u32) -> u32 {
        let offset_value = if lit_len > 0 {
            if offset == self.newest {
                1
            } else if offset == self.second {
                2
            } else if offset == self.third {
                3
            } else {
                offset + 3
            }
        } else if offset == self.second {
            1
        } else if offset == self.third {
            2
        } else if self.newest.checked_sub(1) == Some(offset) {
            3
        } else {
            offset + 3
        };

        self.update_from_offset_value(offset_value, lit_len, offset);
        offset_value
    }

    #[inline(always)]
    pub(crate) fn update_after_match(&mut self, offset: u32, has_literals: bool) {
        if has_literals {
            if offset == self.newest {
                return;
            }
            if offset == self.second {
                self.second = self.newest;
                self.newest = offset;
                return;
            }
        } else if offset == self.second {
            self.second = self.newest;
            self.newest = offset;
            return;
        }

        self.third = self.second;
        self.second = self.newest;
        self.newest = offset;
    }

    fn update_from_offset_value(&mut self, offset_value: u32, lit_len: u32, actual_offset: u32) {
        if lit_len > 0 {
            match offset_value {
                1 => {}
                2 => {
                    self.second = self.newest;
                    self.newest = actual_offset;
                }
                _ => {
                    self.third = self.second;
                    self.second = self.newest;
                    self.newest = actual_offset;
                }
            }
        } else {
            match offset_value {
                1 => {
                    self.second = self.newest;
                    self.newest = actual_offset;
                }
                _ => {
                    self.third = self.second;
                    self.second = self.newest;
                    self.newest = actual_offset;
                }
            }
        }
    }
}

impl<R: Read, W: Write> FrameCompressor<R, W, MatchGeneratorDriver> {
    /// Create a new `FrameCompressor`
    pub fn new(compression_level: CompressionLevel) -> Self {
        Self {
            uncompressed_data: None,
            compressed_data: None,
            compression_level,
            file_type_hint: CompressionFileType::Unknown,
            file_profile_hint: CompressionFileProfile::None,
            state: CompressState {
                matcher: MatchGeneratorDriver::new(1024 * 128, 4),
                last_huff_table: None,
                fse_tables: FseTables::new(),
                offset_history: OffsetHistory::new(),
                file_type_hint: CompressionFileType::Unknown,
                file_profile_hint: CompressionFileProfile::None,
            },
            #[cfg(feature = "hash")]
            hasher: XxHash64::with_seed(0),
        }
    }

    /// Create a new `FrameCompressor` with a coarse file-type hint.
    pub fn new_with_file_type(
        compression_level: CompressionLevel,
        file_type_hint: CompressionFileType,
    ) -> Self {
        Self::new_with_hints(
            compression_level,
            file_type_hint,
            CompressionFileProfile::None,
        )
    }

    pub(crate) fn new_with_hints(
        compression_level: CompressionLevel,
        file_type_hint: CompressionFileType,
        file_profile_hint: CompressionFileProfile,
    ) -> Self {
        let mut compressor = Self::new(compression_level);
        compressor.file_type_hint = file_type_hint;
        compressor.file_profile_hint = file_profile_hint;
        compressor
    }
}

impl<R: Read, W: Write, M: Matcher> FrameCompressor<R, W, M> {
    /// Create a new `FrameCompressor` with a custom matching algorithm implementation
    pub fn new_with_matcher(matcher: M, compression_level: CompressionLevel) -> Self {
        Self {
            uncompressed_data: None,
            compressed_data: None,
            file_type_hint: CompressionFileType::Unknown,
            file_profile_hint: CompressionFileProfile::None,
            state: CompressState {
                matcher,
                last_huff_table: None,
                fse_tables: FseTables::new(),
                offset_history: OffsetHistory::new(),
                file_type_hint: CompressionFileType::Unknown,
                file_profile_hint: CompressionFileProfile::None,
            },
            compression_level,
            #[cfg(feature = "hash")]
            hasher: XxHash64::with_seed(0),
        }
    }

    /// Before calling [FrameCompressor::compress] you need to set the source.
    ///
    /// This is the data that is compressed and written into the drain.
    pub fn set_source(&mut self, uncompressed_data: R) -> Option<R> {
        self.uncompressed_data.replace(uncompressed_data)
    }

    /// Before calling [FrameCompressor::compress] you need to set the drain.
    ///
    /// As the compressor compresses data, the drain serves as a place for the output to be writte.
    pub fn set_drain(&mut self, compressed_data: W) -> Option<W> {
        self.compressed_data.replace(compressed_data)
    }

    /// Compress the uncompressed data from the provided source as one Zstd frame and write it to the provided drain
    ///
    /// This will repeatedly call [Read::read] on the source to fill up blocks until the source returns 0 on the read call.
    /// Also [Write::write_all] will be called on the drain after each block has been encoded.
    ///
    /// To avoid endlessly encoding from a potentially endless source (like a network socket) you can use the
    /// [Read::take] function
    pub fn compress(&mut self) {
        // Clearing buffers to allow re-using of the compressor
        self.state.file_type_hint = self.file_type_hint;
        self.state.file_profile_hint = self.file_profile_hint;
        self.state.matcher.set_file_type_hint(self.file_type_hint);
        self.state
            .matcher
            .set_internal_file_profile_hint(self.file_profile_hint.internal_hint_code());
        self.state.matcher.reset(self.compression_level);
        self.state.last_huff_table = None;
        self.state.fse_tables.reset();
        self.state.offset_history = OffsetHistory::new();
        let (newest, second, third) = self.state.offset_history.as_offsets();
        self.state.matcher.set_repeat_offsets(newest, second, third);
        #[cfg(feature = "hash")]
        {
            self.hasher = XxHash64::with_seed(0);
        }
        let source = self.uncompressed_data.as_mut().unwrap();
        let drain = self.compressed_data.as_mut().unwrap();
        // As the frame is compressed, it's stored here
        let output: &mut Vec<u8> = &mut Vec::with_capacity(1024 * 130);
        // First write the frame header
        let header = FrameHeader {
            frame_content_size: None,
            single_segment: false,
            content_checksum: cfg!(feature = "hash"),
            dictionary_id: None,
            window_size: Some(self.state.matcher.window_size()),
        };
        header.serialize(output);
        // Now compress block by block
        let mut pending_byte = None;
        loop {
            // Read a single block's worth of uncompressed data from the input
            let mut uncompressed_data = self.state.matcher.get_next_space();
            let mut read_bytes = if let Some(byte) = pending_byte.take() {
                uncompressed_data[0] = byte;
                1
            } else {
                0
            };
            let mut last_block;
            'read_loop: loop {
                let new_bytes = source.read(&mut uncompressed_data[read_bytes..]).unwrap();
                if new_bytes == 0 {
                    last_block = true;
                    break 'read_loop;
                }
                read_bytes += new_bytes;
                if read_bytes == uncompressed_data.len() {
                    last_block = false;
                    break 'read_loop;
                }
            }
            if !last_block {
                let mut lookahead = [0u8; 1];
                match source.read(&mut lookahead).unwrap() {
                    0 => last_block = true,
                    1 => pending_byte = Some(lookahead[0]),
                    _ => unreachable!("single-byte read cannot return more than one byte"),
                }
            }
            uncompressed_data.resize(read_bytes, 0);
            // As we read, hash that data too
            #[cfg(feature = "hash")]
            self.hasher.write(&uncompressed_data);
            // Special handling is needed for compression of a totally empty file (why you'd want to do that, I don't know)
            if uncompressed_data.is_empty() {
                let header = BlockHeader {
                    last_block: true,
                    block_type: crate::blocks::block::BlockType::Raw,
                    block_size: 0,
                };
                // Write the header, then the block
                header.serialize(output);
                drain.write_all(output).unwrap();
                output.clear();
                break;
            }

            compress_with_level_policy(
                &mut self.state,
                self.compression_level,
                last_block,
                uncompressed_data,
                output,
            );
            drain.write_all(output).unwrap();
            output.clear();
            if last_block {
                break;
            }
        }

        // If the `hash` feature is enabled, then `content_checksum` is set to true in the header
        // and a 32 bit hash is written at the end of the data.
        #[cfg(feature = "hash")]
        {
            // Because we only have the data as a reader, we need to read all of it to calculate the checksum
            // Possible TODO: create a wrapper around self.uncompressed data that hashes the data as it's read?
            let content_checksum = self.hasher.finish();
            drain
                .write_all(&(content_checksum as u32).to_le_bytes())
                .unwrap();
        }
    }

    /// Get a mutable reference to the source
    pub fn source_mut(&mut self) -> Option<&mut R> {
        self.uncompressed_data.as_mut()
    }

    /// Get a mutable reference to the drain
    pub fn drain_mut(&mut self) -> Option<&mut W> {
        self.compressed_data.as_mut()
    }

    /// Get a reference to the source
    pub fn source(&self) -> Option<&R> {
        self.uncompressed_data.as_ref()
    }

    /// Get a reference to the drain
    pub fn drain(&self) -> Option<&W> {
        self.compressed_data.as_ref()
    }

    /// Retrieve the source
    pub fn take_source(&mut self) -> Option<R> {
        self.uncompressed_data.take()
    }

    /// Retrieve the drain
    pub fn take_drain(&mut self) -> Option<W> {
        self.compressed_data.take()
    }

    /// Before calling [FrameCompressor::compress] you can replace the matcher
    pub fn replace_matcher(&mut self, mut match_generator: M) -> M {
        core::mem::swap(&mut match_generator, &mut self.state.matcher);
        match_generator
    }

    /// Before calling [FrameCompressor::compress] you can replace the compression level
    pub fn set_compression_level(
        &mut self,
        compression_level: CompressionLevel,
    ) -> CompressionLevel {
        let old = self.compression_level;
        self.compression_level = compression_level;
        old
    }

    /// Get the current compression level
    pub fn compression_level(&self) -> CompressionLevel {
        self.compression_level
    }

    /// Replace the coarse file-type hint used to choose internal starting points.
    pub fn set_file_type_hint(
        &mut self,
        file_type_hint: CompressionFileType,
    ) -> CompressionFileType {
        let old = self.file_type_hint;
        self.file_type_hint = file_type_hint;
        old
    }
}

fn compress_with_level_policy<M: Matcher>(
    state: &mut CompressState<M>,
    level: CompressionLevel,
    last_block: bool,
    uncompressed_data: Vec<u8>,
    output: &mut Vec<u8>,
) {
    match level {
        CompressionLevel::Uncompressed => {
            let header = BlockHeader {
                last_block,
                block_type: crate::blocks::block::BlockType::Raw,
                block_size: uncompressed_data.len().try_into().unwrap(),
            };
            header.serialize(output);
            output.extend_from_slice(&uncompressed_data);
        }
        CompressionLevel::Fastest => compress_fastest(state, last_block, uncompressed_data, output),
        CompressionLevel::Default | CompressionLevel::Better => {
            compress_at_level(state, level, last_block, uncompressed_data, output)
        }
        CompressionLevel::Best => {
            compress_best_adaptive(state, level, last_block, uncompressed_data, output)
        }
    }
}

fn compress_best_adaptive<M: Matcher>(
    state: &mut CompressState<M>,
    level: CompressionLevel,
    last_block: bool,
    uncompressed_data: Vec<u8>,
    output: &mut Vec<u8>,
) {
    let segment_lengths = best_block_segment_lengths(uncompressed_data.as_slice());
    if segment_lengths.len() == 1 {
        compress_at_level(state, level, last_block, uncompressed_data, output);
        return;
    }

    let mut remaining = uncompressed_data;
    let segment_count = segment_lengths.len();
    for (segment_idx, segment_len) in segment_lengths.into_iter().enumerate() {
        let segment_last = last_block && segment_idx + 1 == segment_count;
        let segment = if segment_idx + 1 == segment_count {
            core::mem::take(&mut remaining)
        } else {
            let tail = remaining.split_off(segment_len);
            core::mem::replace(&mut remaining, tail)
        };
        compress_at_level_without_incompressible_probe(state, level, segment_last, segment, output);
    }
}

fn best_block_segment_lengths(data: &[u8]) -> Vec<usize> {
    if data.len() < BEST_SPLIT_CHUNK_SIZE * 2 {
        return alloc::vec![data.len()];
    }

    let chunk_count = data.len().div_ceil(BEST_SPLIT_CHUNK_SIZE);
    let mut incompressible_chunks = Vec::with_capacity(chunk_count);

    for chunk_idx in 0..chunk_count {
        let start = chunk_idx * BEST_SPLIT_CHUNK_SIZE;
        let end = (start + BEST_SPLIT_CHUNK_SIZE).min(data.len());
        let chunk = &data[start..end];
        incompressible_chunks.push(likely_incompressible(chunk) && !likely_text(chunk));
    }

    let mut split_chunks = alloc::vec![false; chunk_count];
    let mut any_split_chunk = false;
    let mut chunk_idx = 0usize;
    while chunk_idx < chunk_count {
        if !incompressible_chunks[chunk_idx] {
            chunk_idx += 1;
            continue;
        }

        let run_start = chunk_idx;
        while chunk_idx < chunk_count && incompressible_chunks[chunk_idx] {
            chunk_idx += 1;
        }
        let run_len = chunk_idx - run_start;
        if run_len >= BEST_SPLIT_MIN_INCOMPRESSIBLE_RUN_CHUNKS {
            for split_chunk in &mut split_chunks[run_start..chunk_idx] {
                *split_chunk = true;
            }
            any_split_chunk = true;
        }
    }

    if !any_split_chunk {
        return alloc::vec![data.len()];
    }

    if split_chunks.iter().all(|split_chunk| *split_chunk) && likely_incompressible(data) {
        return alloc::vec![data.len()];
    }

    let mut segments = Vec::with_capacity(chunk_count);
    let mut run_start_chunk = 0usize;

    for (chunk_idx, split_chunk) in split_chunks.iter().copied().enumerate() {
        if !split_chunk {
            continue;
        }

        push_compressible_run_segments(&mut segments, data, run_start_chunk, chunk_idx);
        let start = chunk_idx * BEST_SPLIT_CHUNK_SIZE;
        let end = (start + BEST_SPLIT_CHUNK_SIZE).min(data.len());
        segments.push(end - start);
        run_start_chunk = chunk_idx + 1;
    }

    push_compressible_run_segments(&mut segments, data, run_start_chunk, chunk_count);

    if segments.is_empty() {
        alloc::vec![data.len()]
    } else {
        segments
    }
}

fn push_compressible_run_segments(
    segments: &mut Vec<usize>,
    data: &[u8],
    start_chunk: usize,
    end_chunk: usize,
) {
    if start_chunk == end_chunk {
        return;
    }

    let mut start = start_chunk * BEST_SPLIT_CHUNK_SIZE;
    let end = (end_chunk * BEST_SPLIT_CHUNK_SIZE).min(data.len());
    while start < end {
        let next_end = (start + BEST_SPLIT_COMPRESSIBLE_RUN_MAX).min(end);
        segments.push(next_end - start);
        start = next_end;
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;

    use super::{best_block_segment_lengths, FrameCompressor, OffsetHistory};
    use crate::common::MAGIC_NUM;
    use crate::decoding::FrameDecoder;

    #[test]
    fn frame_starts_with_magic_num() {
        let mock_data = [1_u8, 2, 3].as_slice();
        let mut output: Vec<u8> = Vec::new();
        let mut compressor = FrameCompressor::new(super::CompressionLevel::Uncompressed);
        compressor.set_source(mock_data);
        compressor.set_drain(&mut output);

        compressor.compress();
        assert!(output.starts_with(&MAGIC_NUM.to_le_bytes()));
    }

    #[test]
    fn direct_repeat_history_update_matches_encoded_update_with_literals() {
        for offset in [1, 4, 8, 9] {
            let mut encoded = OffsetHistory::new();
            let mut direct = OffsetHistory::new();

            encoded.encode_offset_value(offset, 3);
            direct.update_after_match(offset, true);

            assert_eq!(direct, encoded);
        }
    }

    #[test]
    fn direct_repeat_history_update_matches_encoded_update_without_literals() {
        for offset in [4, 8, 0, 9] {
            let mut encoded = OffsetHistory::new();
            let mut direct = OffsetHistory::new();

            encoded.encode_offset_value(offset, 0);
            direct.update_after_match(offset, false);

            assert_eq!(direct, encoded);
        }
    }

    #[test]
    fn very_simple_raw_compress() {
        let mock_data = [1_u8, 2, 3].as_slice();
        let mut output: Vec<u8> = Vec::new();
        let mut compressor = FrameCompressor::new(super::CompressionLevel::Uncompressed);
        compressor.set_source(mock_data);
        compressor.set_drain(&mut output);

        compressor.compress();
    }

    #[test]
    fn very_simple_compress() {
        let mut mock_data = vec![0; 1 << 17];
        mock_data.extend(vec![1; (1 << 17) - 1]);
        mock_data.extend(vec![2; (1 << 18) - 1]);
        mock_data.extend(vec![2; 1 << 17]);
        mock_data.extend(vec![3; (1 << 17) - 1]);
        let mut output: Vec<u8> = Vec::new();
        let mut compressor = FrameCompressor::new(super::CompressionLevel::Uncompressed);
        compressor.set_source(mock_data.as_slice());
        compressor.set_drain(&mut output);

        compressor.compress();

        let mut decoder = FrameDecoder::new();
        let mut decoded = Vec::with_capacity(mock_data.len());
        decoder.decode_all_to_vec(&output, &mut decoded).unwrap();
        assert_eq!(mock_data, decoded);

        let mut decoded = Vec::new();
        zstd::stream::copy_decode(output.as_slice(), &mut decoded).unwrap();
        assert_eq!(mock_data, decoded);
    }

    #[test]
    fn rle_compress() {
        let mock_data = vec![0; 1 << 19];
        let mut output: Vec<u8> = Vec::new();
        let mut compressor = FrameCompressor::new(super::CompressionLevel::Uncompressed);
        compressor.set_source(mock_data.as_slice());
        compressor.set_drain(&mut output);

        compressor.compress();

        let mut decoder = FrameDecoder::new();
        let mut decoded = Vec::with_capacity(mock_data.len());
        decoder.decode_all_to_vec(&output, &mut decoded).unwrap();
        assert_eq!(mock_data, decoded);
    }

    #[test]
    fn fastest_reused_compressor_handles_tiny_then_compressible_frame() {
        let mut compressor = FrameCompressor::new(super::CompressionLevel::Fastest);

        let tiny = b"a";
        let mut tiny_output = Vec::new();
        compressor.set_source(tiny.as_slice());
        compressor.set_drain(&mut tiny_output);
        compressor.compress();
        let tiny_frame = compressor.take_drain().expect("tiny frame drain is set");
        assert_decodes_with_rust_and_c(tiny_frame, tiny);

        let mut compressible = Vec::with_capacity(64 * 1024);
        while compressible.len() < 64 * 1024 {
            compressible.extend_from_slice(b"abcde-record-payload\n");
        }
        compressible.truncate(64 * 1024);

        let mut output = Vec::new();
        compressor.set_source(compressible.as_slice());
        compressor.set_drain(&mut output);
        compressor.compress();
        let output_frame = compressor.take_drain().expect("output drain is set");
        assert_decodes_with_rust_and_c(output_frame, &compressible);
        assert!(
            output_frame.len() < compressible.len() / 4,
            "reused fastest compressor should still compress repetitive data: {} bytes",
            output_frame.len()
        );
    }

    #[test]
    fn exact_full_block_is_marked_last_without_empty_block() {
        let mock_data = vec![7; 128 * 1024];
        let mut output: Vec<u8> = Vec::new();
        let mut compressor = FrameCompressor::new(super::CompressionLevel::Uncompressed);
        compressor.set_source(mock_data.as_slice());
        compressor.set_drain(&mut output);

        compressor.compress();

        let (_, frame_header_size) = crate::decoding::frame::read_frame_header(output.as_slice())
            .expect("frame header should parse");
        let mut block_decoder = crate::decoding::block_decoder::new();
        let (block_header, _) = block_decoder
            .read_block_header(&output[frame_header_size as usize..])
            .expect("block header should parse");

        assert!(block_header.last_block);
        assert_eq!(
            block_header.block_type,
            crate::blocks::block::BlockType::Raw
        );
        assert_eq!(block_header.content_size, mock_data.len() as u32);

        let mut decoder = FrameDecoder::new();
        let mut decoded = Vec::with_capacity(mock_data.len());
        decoder.decode_all_to_vec(&output, &mut decoded).unwrap();
        assert_eq!(mock_data, decoded);
    }

    #[test]
    fn full_block_lookahead_preserves_next_block_first_byte() {
        let mut mock_data = vec![3; 128 * 1024];
        mock_data.extend_from_slice(&[4, 5, 6]);
        let mut output: Vec<u8> = Vec::new();
        let mut compressor = FrameCompressor::new(super::CompressionLevel::Uncompressed);
        compressor.set_source(mock_data.as_slice());
        compressor.set_drain(&mut output);

        compressor.compress();

        let (_, frame_header_size) = crate::decoding::frame::read_frame_header(output.as_slice())
            .expect("frame header should parse");
        let first_block_start = frame_header_size as usize;
        let mut block_decoder = crate::decoding::block_decoder::new();
        let (first_header, first_header_size) = block_decoder
            .read_block_header(&output[first_block_start..])
            .expect("first block header should parse");
        let second_block_start =
            first_block_start + first_header_size as usize + first_header.content_size as usize;
        let mut block_decoder = crate::decoding::block_decoder::new();
        let (second_header, second_header_size) = block_decoder
            .read_block_header(&output[second_block_start..])
            .expect("second block header should parse");
        let second_body_start = second_block_start + second_header_size as usize;

        assert!(!first_header.last_block);
        assert!(second_header.last_block);
        assert_eq!(second_header.content_size, 3);
        assert_eq!(
            &output[second_body_start..second_body_start + 3],
            &[4, 5, 6]
        );

        let mut decoder = FrameDecoder::new();
        let mut decoded = Vec::with_capacity(mock_data.len());
        decoder.decode_all_to_vec(&output, &mut decoded).unwrap();
        assert_eq!(mock_data, decoded);
    }

    #[test]
    fn best_keeps_fully_compressible_block_whole() {
        let mut data = Vec::with_capacity(128 * 1024);
        while data.len() < 128 * 1024 {
            data.extend_from_slice(b"tenant=alpha path=/v1/archive status=200 bytes=4812\n");
        }
        data.truncate(128 * 1024);

        let compressed =
            crate::encoding::compress_to_vec(data.as_slice(), super::CompressionLevel::Best);
        assert_decodes_with_rust_and_c(&compressed, &data);

        let block_headers = collect_block_headers(&compressed);
        assert_eq!(block_headers.len(), 1);
        assert!(block_headers[0].last_block);
    }

    #[test]
    fn best_splits_mixed_block_and_round_trips() {
        let mut data = Vec::with_capacity(64 * 1024);
        while data.len() < 24 * 1024 {
            data.extend_from_slice(b"repeated-best-split-fixture-line\n");
        }
        data.truncate(24 * 1024);
        data.extend_from_slice(&xorshift(24 * 1024));
        while data.len() < 64 * 1024 {
            data.extend_from_slice(b"repeated-best-split-fixture-line\n");
        }
        data.truncate(64 * 1024);

        let compressed =
            crate::encoding::compress_to_vec(data.as_slice(), super::CompressionLevel::Best);
        assert_decodes_with_rust_and_c(&compressed, &data);

        let block_headers = collect_block_headers(&compressed);
        assert!(
            block_headers.len() > 1,
            "best level should split mixed blocks: got {} block(s)",
            block_headers.len()
        );
        assert!(block_headers
            .iter()
            .take(block_headers.len() - 1)
            .all(|header| !header.last_block));
        assert!(block_headers.last().expect("at least one block").last_block);
    }

    #[test]
    fn best_presplit_marks_single_chunk_incompressible_runs() {
        let mut data = Vec::with_capacity(40 * 1024);
        while data.len() < 16 * 1024 {
            data.extend_from_slice(b"repeated-best-split-fixture-line\n");
        }
        data.truncate(16 * 1024);
        data.extend_from_slice(&xorshift(8 * 1024));
        while data.len() < 40 * 1024 {
            data.extend_from_slice(b"repeated-best-split-fixture-line\n");
        }
        data.truncate(40 * 1024);

        let segments = best_block_segment_lengths(&data);
        assert!(
            segments.len() > 1,
            "a single incompressible 8KiB chunk should be split"
        );
    }

    #[test]
    fn best_keeps_fully_incompressible_block_unsplit() {
        let data = xorshift(128 * 1024);

        let compressed =
            crate::encoding::compress_to_vec(data.as_slice(), super::CompressionLevel::Best);
        assert_decodes_with_rust_and_c(&compressed, &data);

        let block_headers = collect_block_headers(&compressed);
        assert_eq!(block_headers.len(), 1);
        assert!(block_headers[0].last_block);
    }

    #[test]
    fn aaa_compress() {
        let mock_data = vec![0, 1, 3, 4, 5];
        let mut output: Vec<u8> = Vec::new();
        let mut compressor = FrameCompressor::new(super::CompressionLevel::Uncompressed);
        compressor.set_source(mock_data.as_slice());
        compressor.set_drain(&mut output);

        compressor.compress();

        let mut decoder = FrameDecoder::new();
        let mut decoded = Vec::with_capacity(mock_data.len());
        decoder.decode_all_to_vec(&output, &mut decoded).unwrap();
        assert_eq!(mock_data, decoded);

        let mut decoded = Vec::new();
        zstd::stream::copy_decode(output.as_slice(), &mut decoded).unwrap();
        assert_eq!(mock_data, decoded);
    }
    #[cfg(feature = "hash")]
    #[test]
    fn checksum_two_frames_reused_compressor() {
        // Compress the same data twice using the same compressor and verify that:
        // 1. The checksum written in each frame matches what the decoder calculates.
        // 2. The hasher is correctly reset between frames (no cross-contamination).
        //    If the hasher were NOT reset, the second frame's calculated checksum
        //    would differ from the one stored in the frame data, causing assert_eq to fail.
        let data: Vec<u8> = (0u8..=255).cycle().take(1024).collect();

        let mut compressor = FrameCompressor::new(super::CompressionLevel::Uncompressed);

        // --- Frame 1 ---
        let mut compressed1 = Vec::new();
        compressor.set_source(data.as_slice());
        compressor.set_drain(&mut compressed1);
        compressor.compress();

        // --- Frame 2 (reuse the same compressor) ---
        let mut compressed2 = Vec::new();
        compressor.set_source(data.as_slice());
        compressor.set_drain(&mut compressed2);
        compressor.compress();

        fn decode_and_collect(compressed: &[u8]) -> (Vec<u8>, Option<u32>, Option<u32>) {
            let mut decoder = FrameDecoder::new();
            let mut source = compressed;
            decoder.reset(&mut source).unwrap();
            while !decoder.is_finished() {
                decoder
                    .decode_blocks(&mut source, crate::decoding::BlockDecodingStrategy::All)
                    .unwrap();
            }
            let mut decoded = Vec::new();
            decoder.collect_to_writer(&mut decoded).unwrap();
            (
                decoded,
                decoder.get_checksum_from_data(),
                decoder.get_calculated_checksum(),
            )
        }

        let (decoded1, chksum_from_data1, chksum_calculated1) = decode_and_collect(&compressed1);
        assert_eq!(decoded1, data, "frame 1: decoded data mismatch");
        assert_eq!(
            chksum_from_data1, chksum_calculated1,
            "frame 1: checksum mismatch"
        );

        let (decoded2, chksum_from_data2, chksum_calculated2) = decode_and_collect(&compressed2);
        assert_eq!(decoded2, data, "frame 2: decoded data mismatch");
        assert_eq!(
            chksum_from_data2, chksum_calculated2,
            "frame 2: checksum mismatch"
        );

        // Same data compressed twice must produce the same checksum.
        // If state leaked across frames, the second calculated checksum would differ.
        assert_eq!(
            chksum_from_data1, chksum_from_data2,
            "frame 1 and frame 2 should have the same checksum (same data, hash must reset per frame)"
        );
    }

    fn assert_decodes_with_rust_and_c(compressed: &[u8], expected: &[u8]) {
        let mut rust_decoded = Vec::with_capacity(expected.len());
        FrameDecoder::new()
            .decode_all_to_vec(compressed, &mut rust_decoded)
            .unwrap();
        assert_eq!(rust_decoded, expected);

        let mut c_decoded = Vec::new();
        zstd::stream::copy_decode(compressed, &mut c_decoded).unwrap();
        assert_eq!(c_decoded, expected);
    }

    fn collect_block_headers(frame: &[u8]) -> Vec<crate::blocks::block::BlockHeader> {
        let (_, frame_header_size) =
            crate::decoding::frame::read_frame_header(frame).expect("frame header should parse");
        let mut headers = Vec::new();
        let mut offset = frame_header_size as usize;

        loop {
            let mut block_decoder = crate::decoding::block_decoder::new();
            let (header, header_size) = block_decoder
                .read_block_header(&frame[offset..])
                .expect("block header should parse");
            offset += header_size as usize + header.content_size as usize;
            headers.push(header);
            if headers.last().expect("pushed header").last_block {
                break;
            }
        }

        headers
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

    #[cfg(feature = "std")]
    #[test]
    fn fuzz_targets() {
        use std::io::Read;
        fn decode_ruzstd(data: &mut dyn std::io::Read) -> Vec<u8> {
            let mut decoder = crate::decoding::StreamingDecoder::new(data).unwrap();
            let mut result: Vec<u8> = Vec::new();
            decoder.read_to_end(&mut result).expect("Decoding failed");
            result
        }

        fn decode_ruzstd_writer(mut data: impl Read) -> Vec<u8> {
            let mut decoder = crate::decoding::FrameDecoder::new();
            decoder.reset(&mut data).unwrap();
            let mut result = vec![];
            while !decoder.is_finished() || decoder.can_collect() > 0 {
                decoder
                    .decode_blocks(
                        &mut data,
                        crate::decoding::BlockDecodingStrategy::UptoBytes(1024 * 1024),
                    )
                    .unwrap();
                decoder.collect_to_writer(&mut result).unwrap();
            }
            result
        }

        fn encode_zstd(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
            zstd::stream::encode_all(std::io::Cursor::new(data), 3)
        }

        fn encode_ruzstd_uncompressed(data: &mut dyn std::io::Read) -> Vec<u8> {
            let mut input = Vec::new();
            data.read_to_end(&mut input).unwrap();

            crate::encoding::compress_to_vec(
                input.as_slice(),
                crate::encoding::CompressionLevel::Uncompressed,
            )
        }

        fn encode_ruzstd_compressed(data: &mut dyn std::io::Read) -> Vec<u8> {
            let mut input = Vec::new();
            data.read_to_end(&mut input).unwrap();

            crate::encoding::compress_to_vec(
                input.as_slice(),
                crate::encoding::CompressionLevel::Fastest,
            )
        }

        fn decode_zstd(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
            let mut output = Vec::new();
            zstd::stream::copy_decode(data, &mut output)?;
            Ok(output)
        }
        if std::fs::exists("fuzz/artifacts/interop").unwrap_or(false) {
            for file in std::fs::read_dir("fuzz/artifacts/interop").unwrap() {
                if file.as_ref().unwrap().file_type().unwrap().is_file() {
                    let data = std::fs::read(file.unwrap().path()).unwrap();
                    let data = data.as_slice();
                    // Decoding
                    let compressed = encode_zstd(data).unwrap();
                    let decoded = decode_ruzstd(&mut compressed.as_slice());
                    let decoded2 = decode_ruzstd_writer(&mut compressed.as_slice());
                    assert!(
                        decoded == data,
                        "Decoded data did not match the original input during decompression"
                    );
                    assert_eq!(
                        decoded2, data,
                        "Decoded data did not match the original input during decompression"
                    );

                    // Encoding
                    // Uncompressed encoding
                    let mut input = data;
                    let compressed = encode_ruzstd_uncompressed(&mut input);
                    let decoded = decode_zstd(&compressed).unwrap();
                    assert_eq!(
                        decoded, data,
                        "Decoded data did not match the original input during compression"
                    );
                    // Compressed encoding
                    let mut input = data;
                    let compressed = encode_ruzstd_compressed(&mut input);
                    let decoded = decode_zstd(&compressed).unwrap();
                    assert_eq!(
                        decoded, data,
                        "Decoded data did not match the original input during compression"
                    );
                }
            }
        }
    }
}
