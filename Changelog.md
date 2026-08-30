# Changelog

This document records the changes made between versions.

# 0.1.0 (2026-08-30)

First release as `zstd-complete`, forked from `ruzstd`.

* Add the complete standard compression-level table (levels 1 through 22) and
  a bounded `std::io::Write` encoder with validated levels, typed errors,
  configurable frame chunks, and an explicit memory budget.
* Add portable scalar execution and optimized runtime-selected x86-64 paths,
  plus release checks for Windows, Apple Silicon, AArch64, wasm `no_std`, and
  forced-scalar builds.
* Disclose the Zstandard 1.5.7 compressor provenance and distribute the package
  under the selected BSD-3-Clause option while retaining the inherited ruzstd
  copyright and MIT permission notice.
* Add generated Fast, DFast, row, Huffman, and sequence-store kernels as private
  modules in the main crate. The temporary five-crate development layout was
  removed before release.
* Add the idiomatic crate import name `zstd_complete`; workspace tools retain
  `ruzstd` only as a local dependency alias.
* Document exact zstd C feature coverage, safe public API boundaries, internal
  unsafe-code assurance, benchmark limitations, and the absence of an
  independent human line-by-line review.
* Improve Fastest match selection by retaining both oldest and newest hash
  candidates for each suffix bucket and using a cheaper five-byte hash.

* Avoid emitting compressed blocks when the compressed payload is not smaller
  than the raw block.
* Fix Dictionary decoding. It should not panic on invalid inputs.
* Improve Huffman literal table selection by using length-limited frequency
  code lengths when they can be repaired to the zstd 11-bit maximum.

# ruzstd history

The entries below predate the `zstd-complete` fork.

# After 0.8.2
* Introduce the `rust-version` field
* Fix checksum generation when repeatedly using the encoder
* Expose decoding::Dictionary as public
* Add Debug derive to CompressionLevel enum
* Make RLE and Raw block decoding more efficient and not use intermediary buffer on the stack

# After 0.8.1
* The CLI has been refactored to use `clap`
* The MatchDriverGenerator has been made public so users can name it as `M` in `FrameCompressor<R,W,M>`

# After 0.8.0
* The compressor now includes a `content_checksum` when the `hash` feature is enabled
* Dictionary generation has been added

# After 0.7.3
* Add initial compression support
* **Breaking** Refactor modules to reflect that this is now also a compression library

# After 0.7.2
* Soundness fix in decoding::RingBuffer. The lengths of the diferent regions where sometimes calculated wrongly, resulting in reads of heap memory not belonging to that ringbuffer
    * Fixed by https://github.com/paolobarbolini
    * Affected versions: 0.7.0 up to and including 0.7.2

* Added convenience functions to FrameDecoder to decode multiple frames from a buffer (https://github.com/philipc)

# After 0.7.1

* Remove byteorder dependency (https://github.com/workingjubilee)
* Preparations to become a std dependency (https://github.com/workingjubilee)

# After 0.7.0
* Fix for drain_to functions into limited targets (https://github.com/michaelkirk)

# After 0.6.0
* Small fix in the zstd binary, progress tracking was slighty off for skippable frames resulting in an error only when the last frame in a file was skippable
* Small performance improvement by reorganizing code with `#[cold]` annotations
* Documentation for `StreamDecoder` mentioning the limitations around multiple frames (https://github.com/Sorseg)
* Documentation around skippable frames (https://github.com/Sorseg)
* **Breaking** `StreamDecoder` API changes to get access to the inner parts (https://github.com/ifd3f)
* Big internal documentation contribution (https://github.com/zleyyij)
* Dropped derive_more as a dependency (https://github.com/xd009642)
* Small improvement by removing the error cases from the reverse bitreader (and making sure invalid requests can't even happen)

# After 0.5.0
* Make the hashing checksum optional (thanks to [@tamird](https://github.com/tamird))
    * breaking change as the public API changes based on features
* The FrameDecoder is now Send + Sync (RingBuffer impls these traits now)
