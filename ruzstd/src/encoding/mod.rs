//! Structures and utilities used for compressing/encoding data into the Zstd format.

pub(crate) mod block_header;
pub(crate) mod blocks;
mod file_profile;
pub(crate) mod frame_header;
pub(crate) mod match_generator;
pub(crate) mod util;

mod frame_compressor;
mod levels;
pub(crate) use file_profile::CompressionFileProfile;
pub use file_profile::CompressionFileType;
#[cfg(feature = "std")]
pub(crate) use file_profile::{compression_file_profile_for_path_and_data, read_file_type_sample};
#[cfg(feature = "std")]
pub use file_profile::{compression_file_type_for_path, compression_file_type_for_path_and_data};
pub use frame_compressor::FrameCompressor;
pub use match_generator::MatchGeneratorDriver;

use crate::io::{Read, Write};
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::path::Path;

/// Convenience function to compress some source into a target without reusing any resources of the compressor
/// ```rust
/// use ruzstd::encoding::{compress, CompressionLevel};
/// let data: &[u8] = &[0,0,0,0,0,0,0,0,0,0,0,0];
/// let mut target = Vec::new();
/// compress(data, &mut target, CompressionLevel::Fastest);
/// ```
pub fn compress<R: Read, W: Write>(source: R, target: W, level: CompressionLevel) {
    let mut frame_enc = FrameCompressor::new(level);
    frame_enc.set_source(source);
    frame_enc.set_drain(target);
    frame_enc.compress();
}

/// Convenience function to compress some source into a target using a coarse file-type hint.
///
/// The public API stays narrow: callers provide only the requested compression level and
/// the file family. The encoder decides the internal starting point from there.
pub fn compress_with_file_type<R: Read, W: Write>(
    source: R,
    target: W,
    file_type: CompressionFileType,
    level: CompressionLevel,
) {
    let mut frame_enc =
        FrameCompressor::new_with_hints(level, file_type, CompressionFileProfile::None);
    frame_enc.set_source(source);
    frame_enc.set_drain(target);
    frame_enc.compress();
}

/// Convenience function to compress some source into a target using a path-based file-type hint.
#[cfg(feature = "std")]
pub fn compress_with_path<R: Read, W: Write>(
    mut source: R,
    target: W,
    path: &Path,
    level: CompressionLevel,
) {
    let sample = read_file_type_sample(&mut source);
    let file_type = compression_file_type_for_path_and_data(path, &sample);
    let file_profile = compression_file_profile_for_path_and_data(path, &sample);
    let mut frame_enc = FrameCompressor::new_with_hints(level, file_type, file_profile);
    frame_enc.set_source(sample.as_slice().chain(source));
    frame_enc.set_drain(target);
    frame_enc.compress();
}

/// Convenience function to compress some source into a Vec without reusing any resources of the compressor
/// ```rust
/// use ruzstd::encoding::{compress_to_vec, CompressionLevel};
/// let data: &[u8] = &[0,0,0,0,0,0,0,0,0,0,0,0];
/// let compressed = compress_to_vec(data, CompressionLevel::Fastest);
/// ```
pub fn compress_to_vec<R: Read>(source: R, level: CompressionLevel) -> Vec<u8> {
    let mut vec = Vec::new();
    compress(source, &mut vec, level);
    vec
}

/// Compress a full source into a target using the faithful C no-dictionary level table.
///
/// This entry point accepts the same numeric level range as upstream zstd. It currently
/// targets the no-dictionary path and buffers the complete source so that the C-port
/// frame encoder can choose block strategies from the full content size.
pub fn compress_c_level<R: Read, W: Write>(mut source: R, mut target: W, level: i32) {
    let mut input = Vec::new();
    source.read_to_end(&mut input).unwrap();
    let compressed = levels::c_port::encode_frame_no_dict(&input, level);
    target.write_all(&compressed).unwrap();
}

/// Compress a full source into a Vec using the faithful C no-dictionary level table.
pub fn compress_to_vec_c_level<R: Read>(source: R, level: i32) -> Vec<u8> {
    let mut vec = Vec::new();
    compress_c_level(source, &mut vec, level);
    vec
}

/// Convenience function to compress some source into a Vec using a coarse file-type hint.
pub fn compress_to_vec_with_file_type<R: Read>(
    source: R,
    file_type: CompressionFileType,
    level: CompressionLevel,
) -> Vec<u8> {
    let mut vec = Vec::new();
    compress_with_file_type(source, &mut vec, file_type, level);
    vec
}

/// Convenience function to compress some source into a Vec using a path-based file-type hint.
#[cfg(feature = "std")]
pub fn compress_to_vec_with_path<R: Read>(
    source: R,
    path: &Path,
    level: CompressionLevel,
) -> Vec<u8> {
    let mut vec = Vec::new();
    compress_with_path(source, &mut vec, path, level);
    vec
}

/// The compression mode used impacts the speed of compression,
/// and resulting compression ratios. Faster compression will result
/// in worse compression ratios, and vice versa.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CompressionLevel {
    /// This level does not compress the data at all, and simply wraps
    /// it in a Zstandard frame.
    Uncompressed,
    /// This level is roughly equivalent to Zstd compression level 1
    Fastest,
    /// This level is roughly equivalent to Zstd level 3,
    /// or the one used by the official compressor when no level
    /// is specified.
    ///
    /// UNIMPLEMENTED
    Default,
    /// This level is roughly equivalent to Zstd level 7.
    ///
    /// UNIMPLEMENTED
    Better,
    /// This level is roughly equivalent to Zstd level 11.
    ///
    /// UNIMPLEMENTED
    Best,
}

/// Trait used by the encoder that users can use to extend the matching facilities with their own algorithm
/// making their own tradeoffs between runtime, memory usage and compression ratio
///
/// This trait operates on buffers that represent the chunks of data the matching algorithm wants to work on.
/// Each one of these buffers is referred to as a *space*. One or more of these buffers represent the window
/// the decoder will need to decode the data again.
///
/// This library asks the Matcher for a new buffer using `get_next_space` to allow reusing of allocated buffers when they are no longer part of the
/// window of data that is being used for matching.
///
/// The library fills the buffer with data that is to be compressed and commits them back to the matcher using `commit_space`.
///
/// Then it will either call `start_matching` or, if the space is deemed not worth compressing, `skip_matching` is called.
///
/// This is repeated until no more data is left to be compressed.
pub trait Matcher {
    /// Get a space where we can put data to be matched on. Will be encoded as one block. The maximum allowed size is 128 kB.
    fn get_next_space(&mut self) -> alloc::vec::Vec<u8>;
    /// Get a reference to the last commited space
    fn get_last_space(&self) -> &[u8];
    /// Commit a space to the matcher so it can be matched against
    fn commit_space(&mut self, space: alloc::vec::Vec<u8>);
    /// Just process the data in the last commited space for future matching
    fn skip_matching(&mut self);
    /// Process the data in the last commited space for future matching AND generate matches for the data
    fn start_matching(&mut self, handle_sequence: impl for<'a> FnMut(Sequence<'a>));
    /// Reset this matcher so it can be used for the next new frame
    fn reset(&mut self, level: CompressionLevel);
    /// Provide a coarse file-type hint so the matcher can choose an internal starting point.
    ///
    /// Implementations that do not care about path/extension hints can ignore this hook.
    fn set_file_type_hint(&mut self, _file_type: CompressionFileType) {}
    /// Provide a narrower internal file profile when one is known.
    ///
    /// This is encoded as a small integer so the public matcher trait does not expose the
    /// encoder's private profile enum.
    fn set_internal_file_profile_hint(&mut self, _file_profile_code: u8) {}
    /// Synchronize the matcher with the encoder's current repeat-offset history.
    ///
    /// Matchers that do not use repeat-offset history can ignore this hook.
    fn set_repeat_offsets(&mut self, _newest: u32, _second: u32, _third: u32) {}
    /// Mark the last committed space as processed without indexing it for future matches.
    ///
    /// This is intended for data that has already been classified as very unlikely to
    /// be useful match history, such as incompressible raw blocks.
    fn skip_matching_for_incompressible(&mut self) {
        self.skip_matching();
    }
    /// Mark the last committed space as processed after it was emitted as an RLE block.
    ///
    /// The default behavior preserves the existing matcher contract by indexing the block
    /// normally. Matchers can specialize this because every minimum-match suffix in an RLE
    /// block has the same key.
    fn skip_matching_for_rle(&mut self) {
        self.skip_matching();
    }
    /// The size of the window the decoder will need to execute all sequences produced by this matcher
    ///
    /// May change after a call to reset with a different compression level
    fn window_size(&self) -> u64;
}

#[derive(PartialEq, Eq, Debug)]
/// Sequences that a [`Matcher`] can produce
pub enum Sequence<'data> {
    /// Is encoded as a sequence for the decoder sequence execution.
    ///
    /// First the literals will be copied to the decoded data,
    /// then `match_len` bytes are copied from `offset` bytes back in the decoded data
    Triple {
        literals: &'data [u8],
        offset: usize,
        match_len: usize,
    },
    /// This is returned as the last sequence in a block
    ///
    /// These literals will just be copied at the end of the sequence execution by the decoder
    Literals { literals: &'data [u8] },
}

#[cfg(test)]
mod tests {
    use super::{compress_c_level, compress_to_vec_c_level, CompressionLevel};
    use crate::decoding::FrameDecoder;
    use alloc::vec::Vec;

    #[test]
    fn compression_level_equality_is_available_for_api_comparisons() {
        assert_eq!(CompressionLevel::Fastest, CompressionLevel::Fastest);
    }

    #[test]
    fn c_level_compression_round_trips_representative_strategies() {
        let mut data = Vec::new();
        while data.len() < (crate::common::MAX_BLOCK_SIZE as usize * 2) + 2048 {
            data.extend_from_slice(b"public-c-level route=/archive status=200 bytes=1874\n");
        }

        for level in [1, 3, 5, 8, 13, 16, 18, 19, 22] {
            let encoded = compress_to_vec_c_level(data.as_slice(), level);
            assert_round_trips(&encoded, &data);
        }
    }

    #[test]
    fn c_level_compression_writes_to_target() {
        let data = b"public-c-level-writer public-c-level-writer";
        let mut encoded = Vec::new();

        compress_c_level(data.as_slice(), &mut encoded, 16);

        assert_round_trips(&encoded, data);
    }

    fn assert_round_trips(encoded: &[u8], expected: &[u8]) {
        let mut decoded = Vec::with_capacity(expected.len());
        FrameDecoder::new()
            .decode_all_to_vec(encoded, &mut decoded)
            .unwrap();

        assert_eq!(decoded, expected);
    }
}
