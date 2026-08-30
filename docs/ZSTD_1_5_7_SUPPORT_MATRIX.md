# zstd 1.5.7 capability matrix

This matrix compares stable application capabilities, not ABI symbol count.
`zstd-complete` is a native Rust implementation of the RFC 8878 format; it is
not a Rust spelling of `zstd.h` and does not promise byte-identical output.

| Capability group | Status | Rust-facing contract |
|---|---|---|
| Frame and block decompression | Supported | Single-frame `StreamingDecoder`, low-level `FrameDecoder`, and bounded archive `MultiFrameDecoder` |
| Concatenated and skippable frames | Supported | Configurable skip/reject policy and resource limits |
| Checksums and dictionary-ID selection | Supported | Checked frames and reusable parsed dictionaries |
| Streaming compression | Supported | Bounded `Write` encoder emitting independent interoperable frames |
| Compression levels | Supported | Positive 1–22, `CompressionLevel::fast()` negative acceleration, and explicit uncompressed output |
| Multithreaded compression | Supported, opt-in | Ordered independent-frame workers behind `multithreading`; one worker is the unchanged sequential path |
| Raw and formatted dictionary use | Supported | Reusable encoder dictionaries and decoder dictionaries |
| Dictionary training | Supported, opt-in | Native sample-set formatted trainer plus legacy raw-content helpers behind `dict_builder` |
| Advanced compression parameters | Supported | Typed strategy, matcher, long-distance matching, target-block, pledged-size, and frame-content-size controls |
| `no_std` encode/decode | Supported | Includes typed reusable and caller-byte-buffer allocation-free prepared workspaces; `Read`/`Write`, training, and threading require `std` |
| C ABI and context lifecycle | Intentionally different | Rust ownership and `Result` errors replace opaque contexts, manual reset, and error-code inspection |
| Custom allocator callbacks and C ABI static contexts | Intentionally different | Rust has no allocator-callback ABI; typed `StaticEncoderWorkspace` and `StaticDecoderWorkspace` use arbitrary caller byte slices instead |
| Experimental parameter-number API | Out of scope | Only stable, independently useful typed controls are public |
| Sequence producer, block splitter, and low-level entropy APIs | Out of scope | Internal implementation details rather than application contracts |
| Stable byte identity with zstd C | Non-goal | RFC-compatible streams may improve ratio or choose different blocks and matches |
| Legacy/deprecated zstd C symbols | Out of scope | No ABI compatibility objective |

The local release matrix covers zstd C decoding of Rust archives and Rust
decoding of C archives, standard corpus fixtures, dictionaries, checksums,
scalar and accelerated paths, hosted operating systems, `no_std`, Miri, ASan,
and bounded fuzzing. See `RELEASE_READINESS.md` and `RELEASING.md` for the exact
release gates.
