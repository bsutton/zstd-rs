# zstd-complete

[![crates.io](https://img.shields.io/crates/v/zstd-complete.svg)](https://crates.io/crates/zstd-complete)
[![API documentation](https://docs.rs/zstd-complete/badge.svg)](https://docs.rs/zstd-complete)
[![CI](https://github.com/bsutton/zstd-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/bsutton/zstd-rs/actions/workflows/ci.yml)

`zstd-complete` is a pure-Rust Zstandard compressor and decompressor. It
supports bounded streaming compression, all standard positive compression
levels, dictionaries, checksums, portable `no_std` operation, and safe public
interfaces.

“Complete” means that the crate provides both sides of the Zstandard format. It
does **not** mean that it reproduces the zstd C API or every advanced feature of
the C library. The exact differences are listed below.

The project is a fork of
[`ruzstd`](https://github.com/KillingSpark/zstd-rs). Its high-performance
compressor was derived from and translated from zstd C 1.5.7, then substantially
reworked around Rust ownership, errors, and streaming. Format interoperability
is a requirement; byte-for-byte identity with C output is not.

## Install

```toml
[dependencies]
zstd-complete = "0.1"
```

The package name contains a hyphen, so Rust code imports it as
`zstd_complete`.

The default features are `std` and `hash`. Disable default features for the
lower-level `no_std` compressor and decompressor. Enable `dict_builder` to
train raw-content dictionaries.

## Bounded streaming compression

`Encoder<W>` implements `std::io::Write` and reports configuration, memory
budget, and underlying I/O failures. It defaults to level 3, 8 MiB input
frames, and a conservative 96 MiB working-memory budget.

```rust,no_run
# #[cfg(feature = "std")]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use std::{fs::File, io::Write};
use zstd_complete::encoding::{CompressionLevel, Encoder, EncoderOptions};

let input = b"data can be supplied in any number of writes";
let output = File::create("archive.zst")?;
let options = EncoderOptions::new(CompressionLevel::DEFAULT)
    .with_checksum(true);
let mut encoder = Encoder::new(output, options)?;
encoder.write_all(input)?;
encoder.finish()?;
# Ok(())
# }
# #[cfg(not(feature = "std"))]
# fn main() {}
```

Each configured input chunk is emitted as an independent Zstandard frame.
Concatenated frames are valid Zstandard and keep memory bounded regardless of
total input size. Smaller chunks can reduce compression ratio because matches
do not cross frame boundaries. `flush()` closes the current frame; call
`finish()` to write the final frame and recover the wrapped writer.

For small inputs, `encode` streams from a reader to a writer and `encode_all`
returns a `Vec<u8>`. The older `compress`, `compress_to_vec`, and
`FrameCompressor` interfaces remain available, including in `no_std`, but the
bounded, fallible API is preferred for new `std` applications.

### Opt-in parallel compression

Enable the non-default `multithreading` feature to compress independent frames
on multiple worker threads. `ParallelEncoder` implements `Write`, preserves
frame order, bounds the number of in-flight frames, and applies the configured
memory limit to the aggregate worker estimate. A worker count of one delegates
directly to the existing `Encoder`; the default single-threaded implementation
does not contain channels, locks, atomics, or worker checks in its compression
path.

```rust,no_run
# #[cfg(all(feature = "std", feature = "multithreading"))]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use std::{io::Write, num::NonZeroUsize};
use zstd_complete::encoding::{CompressionLevel, EncoderOptions, ParallelEncoder};

let options = EncoderOptions::new(CompressionLevel::DEFAULT)
    .with_memory_limit(256 * 1024 * 1024);
let mut encoder =
    ParallelEncoder::new(Vec::new(), options, NonZeroUsize::new(4).unwrap())?;
encoder.write_all(b"input split into ordered independent frames")?;
let compressed = encoder.finish()?;
# let _ = compressed;
# Ok(())
# }
# #[cfg(not(all(feature = "std", feature = "multithreading")))]
# fn main() {}
```

Parallelism begins only when at least two complete input frames are available,
so inputs no larger than one configured frame do not gain throughput. Smaller
frame chunks expose more parallel work but may reduce compression ratio because
matches never cross frame boundaries.

## Decompression

`StreamingDecoder` implements the crate's `Read` interface for one frame:

```rust,no_run
# #[cfg(feature = "std")]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use std::{fs::File, io::Read};
use zstd_complete::decoding::StreamingDecoder;

let input = File::open("archive.zst")?;
let mut decoder = StreamingDecoder::new(input)?;
let mut decoded = Vec::new();
decoder.read_to_end(&mut decoded)?;
# Ok(())
# }
# #[cfg(not(feature = "std"))]
# fn main() {}
```

Use `FrameDecoder` when you need incremental buffer collection, dictionary
selection, explicit block limits, concatenated-frame handling, or control over
skippable frames. `StreamingDecoder` currently stops after one frame; this is
important when consuming the multi-frame output of a bounded `Encoder`.

## Coverage compared with zstd C 1.5.7

| Capability | `zstd-complete` 0.1 | Notes |
|---|---|---|
| Zstandard compression | Yes | Standard levels 1 through 22 plus uncompressed frames |
| Zstandard decompression | Yes | Raw, RLE, compressed blocks, checksums, and dictionaries |
| Raw-content and formatted dictionaries | Yes | Reusable prepared encoder dictionaries and decoder dictionaries |
| Bounded compression of arbitrarily large input | Yes | Emits independent frames at configurable chunk boundaries |
| `no_std` with `alloc` | Yes | Lower-level encode/decode APIs; the `std::io` encoder requires `std` |
| Portable scalar implementation | Yes | Used on non-x86 targets and available as a forced release gate |
| Runtime-selected x86-64 acceleration | Yes | BMI2 paths are selected at runtime; distributed binaries need not use `target-cpu=native` |
| Multithreaded compression | Opt-in | The `multithreading` feature parallelizes ordered independent frames |
| zstd C ABI/API compatibility | No | This is an idiomatic Rust API, not a drop-in C replacement |
| Full advanced parameter surface | No | Negative levels, worker controls, pledged-size tuning, and `targetCBlockSize` are not stable public options |
| Stable byte identity with zstd C | No | Output is format-compatible and can legitimately differ in block and match choices |

The compressor implements the nine zstd strategy families used by the positive
level table: Fast, DFast, Greedy, Lazy, Lazy2, BTLazy2, BTOpt, BTUltra, and
BTUltra2. The public API intentionally exposes validated `CompressionLevel`
values rather than C-style numeric tuning controls.

Interoperability tests cover Rust-produced output decoded by zstd C, C-produced
fixtures decoded by Rust, checksummed frames, raw blocks, concatenated frames,
and raw and formatted dictionaries. The retained differential matrix has 1,095
normal, target-size, and prepared-dictionary rows across 73 real-world
fixtures; all rows decode. This is strong regression coverage, not a claim that
every possible Zstandard stream has been exhaustively tested.

The release gate also compressed 35 files selected from the official zstd
1.5.7 source tree at levels 1, 3, 5, 8, 16, 19, and 22. All 245 Rust archives
decoded with zstd C and reproduced their inputs. Rust also reproduced zstd C's
decoded output for the four official golden decompression frames, including
empty, RLE-first, zero-sequence, and 128 KiB block cases.

## Safety

The normal public API is safe Rust: callers do not need an `unsafe` block.
Performance-critical internals do use `unsafe` for bounded unaligned loads,
initialized-buffer construction, entropy emission, and the decoder ring
buffer. Those modules are private, their call-site invariants are documented,
and portable scalar implementations remain available.

For the 0.1 release candidate, the local gate passed 758 tests (5 diagnostic
tests ignored), strict Clippy, rustdoc, `no_std` wasm checks, forced-scalar
tests, packaged-source compilation, 353 focused AddressSanitizer tests, and
focused Miri checks of the ring buffer, short writes, unaligned loads, and
prepared-sequence entropy path. Bounded ASan/libFuzzer campaigns completed
2,644 decoder cases and 424 public-encoder cases; encoder cases round-tripped
through both Rust and zstd C. CI also defines native Windows and Apple-Silicon
jobs. These checks are evidence supporting memory safety; they are not a formal
proof or a warranty.

The translated compressor has received extensive differential testing,
interoperability testing, and repeated hardware-counter benchmarking. It has
**not** received an independent human line-by-line source review. This is
disclosed so adopters can apply their own assurance requirements.

## Benchmarks against zstd C

The primary real-world benchmark is the standard 211,938,580-byte Silesia
corpus: 12 individually compressed text, executable, source, database, XML,
and medical-image files ranging from 5 MiB to 51 MiB. Each implementation
compressed every file three times in each of three `perf stat` samples, with
input resident in memory and an owned compressed buffer returned. Rust used
the public bounded API and its default 8 MiB frames; C 1.5.7 used
`ZSTD_compress2()` and one frame per file.

| Level | Rust bytes | C bytes | Size gap | Rust MB/s | C MB/s | Rust instructions/corpus | C instructions/corpus | Instruction gap |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 73,259,577 | 73,276,937 | -0.024% | 295.1 | 403.2 | 6.029 B | 5.024 B | +19.99% |
| 3 | 66,232,654 | 66,216,569 | +0.024% | 212.3 | 240.0 | 7.204 B | 6.619 B | +8.83% |
| 8 | 59,930,104 | 59,820,035 | +0.184% | 59.5 | 69.4 | 35.041 B | 32.854 B | +6.66% |

Negative size gaps favor Rust. The result shows near-identical aggregate
compression ratio and a remaining C speed advantage, especially at level 1.
Every Rust archive was accepted by zstd C, and Rust decoded and byte-verified
all 12 C level-3 archives. Corpus provenance, hashes, exact methodology,
cycles, the seven-level official-source matrix, and reproduction commands are
in [`RELEASE_BENCHMARKS_2026-08-30.md`](https://github.com/bsutton/zstd-rs/blob/main/benchmarks/RELEASE_BENCHMARKS_2026-08-30.md).

The smaller generated-shape benchmark below exposes cases that a mixed corpus
can hide, particularly incompressible and all-zero data.

The public bounded encoder was tested first. Each generated input is 16 MiB;
Rust used its default two 8 MiB frames while the C one-shot API used one frame.
Each program compressed the input three times, and each counter is the mean of
three `perf stat` runs. Every Rust archive passed `zstd -t` with zstd C 1.5.7.

| Level-3 input | Rust bytes | C bytes | Rust instructions/run | C instructions/run | Rust instruction gap |
|---|---:|---:|---:|---:|---:|
| Deterministic random | 16,777,620 | 16,777,610 | 54.58 M | 45.10 M | +21.02% |
| Long-distance repetition | 8,390,996 | 8,390,960 | 85.71 M | 77.12 M | +11.14% |
| Repeated records | 1,706 | 1,621 | 44.78 M | 39.78 M | +12.56% |
| Structured JSONL | 1,337,412 | 1,340,481 | 239.83 M | 221.22 M | +8.41% |
| All zeros | 550 | 531 | 133.07 M | 45.44 M | +192.86% |

This is the relevant end-user comparison, including the bounded encoder's
frame-management cost and compression-ratio tradeoff. The all-zero case makes
the largest remaining low-entropy overhead especially clear. Different frame
counts mean this is a product-level comparison, not isolated algorithm parity.

The following narrower benchmark compares the retained one-shot compressor
core with C under matching framing. It is useful for CPU attribution across
strategy levels, but should not be substituted for the public-API table above.

The table below is the final release snapshot from 2026-08-30 on an AMD Ryzen 7
3700X, Linux x86-64, rustc 1.94.1. Both implementations compressed the same
1,022,035-byte real-world fixture 20 times in one process; each counter is the
mean of three `perf stat` runs. Rust was built with `--release` and one
code-generation unit. The C comparison is zstd 1.5.7 through `ZSTD_compress2`.

| Level | Rust bytes/run | C bytes/run | Rust instructions/run | C instructions/run | Rust instruction gap |
|---:|---:|---:|---:|---:|---:|
| 1 | 571,525 | 571,525 | 27.34 M | 25.32 M | +8.00% |
| 3 | 498,891 | 498,911 | 46.75 M | 42.56 M | +9.85% |
| 5 | 488,363 | 489,007 | 100.68 M | 110.35 M | -8.76% |
| 8 | 484,336 | 484,336 | 138.19 M | 145.35 M | -4.93% |
| 16 | 460,555 | 460,806 | 393.91 M | 435.47 M | -9.54% |

Negative gaps favor Rust. Instruction counts are used because wall-clock cycles
are more sensitive to frequency and scheduling. This focused fixture shows
where the remaining low-level CPU gap lies, but it is not a general throughput
claim. The broader corpus, exact commands, output-size comparisons, upstream
`ruzstd` comparison, streaming ratio measurements, and known limitations are in
[`C_PORT_PAUSE_REPORT_2026-08-29.md`](https://github.com/bsutton/zstd-rs/blob/main/benchmarks/C_PORT_PAUSE_REPORT_2026-08-29.md)
and [`RELEASE_READINESS.md`](https://github.com/bsutton/zstd-rs/blob/main/docs/RELEASE_READINESS.md).

For historical context, on the same level-1 fixture the pre-port upstream
`ruzstd` 0.9.1 `Fastest` path produced 619,101 bytes/run and used about 128.69
million instructions/run. This release produces 571,525 bytes/run and uses
about 27.34 million instructions/run. That comparison covers level 1 only;
upstream did not contain this release's complete C level table.

## Building for performance

Normal Cargo release builds are supported. The benchmarked configuration uses
one code-generation unit in the final application:

```toml
[profile.release]
codegen-units = 1
```

Cargo profiles belong to the final application, so this repository's setting
does not automatically apply to downstream users. The optimization is optional:
the external-consumer benchmark also showed a large advantage over upstream
under Cargo's default release profile.

Do not set `target-cpu=native` for binaries distributed to other machines.
Optimized x86-64 kernels use runtime feature detection, while scalar paths cover
x86-64 without BMI2, AArch64, Windows, Apple platforms, and wasm `no_std`.

## Dictionary training

Enable `dict_builder` for the `dictionary` module's raw-content dictionary
trainer. It does not generate formatted dictionaries containing entropy tables.
On the historical `github-users` sample set, its output was within 0.2% of the
official trainer's compression result. Existing formatted dictionaries can
still be consumed by the encoder and decoder.

## Licensing and provenance

This package is distributed under `BSD-3-Clause`, selecting the BSD option
offered by the Zstandard reference implementation.

This project is a fork of
[`KillingSpark/zstd-rs`](https://github.com/KillingSpark/zstd-rs), created by
Moritz Borcherding and developed by its contributors. Substantial inherited
Rust remains in the decoder and supporting format, entropy, dictionary, and
I/O code. Its MIT copyright and permission notice is preserved as required
attribution in `LICENSES/ruzstd-MIT.txt`; it is not a second outbound license
choice for this package. See `THIRD_PARTY_NOTICES.md` for credit, provenance,
and the detailed compressor source map.

The implementation is not represented as clean-room work and is not intended
to remain a faithful C port. Future maintenance will improve the Rust design
and performance directly while preserving Zstandard format interoperability;
recurring audits of future zstd C releases are not part of that model.
