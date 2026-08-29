# Compressor release readiness

Status date: 2026-08-30. The release package is now named `zstd-complete`
version 0.1.0. This is a working release gate, not authorization to publish.

## Completed foundations

- The default public API uses validated `CompressionLevel` values and a
  bounded `Encoder<W: std::io::Write>` with `Result`-based I/O/configuration
  failures. It supports prepared dictionaries, optional checksums, configurable
  frame chunks, and a conservative memory limit/estimate.
- Numeric C levels and `targetCBlockSize` are confined to the non-default
  `c-port-validation` feature used by differential tools.
- A 64 MiB generated-source test uses a 1 MiB frame chunk without growing the
  retained input allocation. Local release profiling of a 128 MiB random file
  at level 3 with the default 8 MiB/96 MiB configuration peaked at 19,852 KiB
  RSS. A 256 MiB sparse file peaked at 11,900 KiB.
- Concatenated-frame output round-trips through both ruzstd and the zstd C
  decoder. Separate tests cover raw blocks, checksums, prepared dictionaries,
  empty input, I/O-error preservation, and budget rejection.
- Mixed MIT and selected BSD-3-Clause terms, provenance, and per-package
  notices are present. The lack of a human source review must be disclosed in
  any submission.
- The consolidated package contents include both licenses, the third-party
  notice, the private kernels, and the new streaming implementation. A fresh
  offline `cargo package --allow-dirty` verification of the single package
  succeeds, including compilation of the packaged source.
- CI now describes native Windows and Apple-Silicon tests, AArch64 and wasm
  `no_std` checks, forced-scalar execution, focused Miri coverage, and package
  content/license checks. Cross-builds pass locally for Windows x86-64, macOS
  x86-64/AArch64, Linux AArch64 `no_std`, and wasm `no_std`. Native Windows
  and Apple-Silicon execution has also passed on the pushed draft PR.
- The local library gate passes 758 tests with five diagnostic tests ignored;
  strict Clippy, wasm `no_std`, forced-scalar, rustdoc, formatting, and
  `git diff --check` pass. A focused AddressSanitizer run of the unaligned-read
  kernels also passes (leak detection is disabled because it is outside this
  memory-access gate). Four corrected focused Miri gates pass. Bounded
  libFuzzer/ASan runs completed 2,644 decoder cases and 424 public streaming
  encoder cases, with encoder output decoded by both Rust and zstd C.
- The crates.io name was unregistered when checked, and a live
  `cargo publish -p zstd-complete --dry-run --allow-dirty` passed. The package
  contains 235 files, is 2.6 MiB unpacked/500.5 KiB compressed, and compiles from
  the exact tarball. A docs.rs-configured nightly rustdoc build also passes.
- Object inspection confirms executable `.text.sorted.*` sections on ELF,
  code/read/execute `.text$*` sections on COFF, `__TEXT` sections marked with
  `SomeInstructions` on Mach-O, and deliberate omission of native custom code
  sections on wasm.
- The standard 211.94 MiB Silesia corpus now provides the large, heterogeneous
  release benchmark. Its 12 files match their published MD5 values and are
  downloaded from a pinned mirror commit rather than redistributed. The
  official zstd 1.5.7 source-fixture matrix adds 245 C-decoded Rust archives,
  and Rust exactly decodes the four official golden decompression frames.

## Benchmark position

The primary public-API release comparison is the 211,938,580-byte Silesia
corpus. Both inputs were resident before timing and both sides returned owned
compressed buffers. Rust used its public bounded API and 8 MiB frames; C used
`ZSTD_compress2()` and one frame per corpus file. Each of three `perf stat`
samples compressed all 12 files three times:

| Level | Rust bytes | C bytes | Size gap | Rust MB/s | C MB/s | Instruction gap |
|---:|---:|---:|---:|---:|---:|---:|
| 1 | 73,259,577 | 73,276,937 | -0.024% | 295.1 | 403.2 | +19.99% |
| 3 | 66,232,654 | 66,216,569 | +0.024% | 212.3 | 240.0 | +8.83% |
| 8 | 59,930,104 | 59,820,035 | +0.184% | 59.5 | 69.4 | +6.66% |

The complete methodology, cycles, provenance, source-fixture aggregates, and
reproduction commands are in `benchmarks/RELEASE_BENCHMARKS_2026-08-30.md`.
This closes the recognized large-real-world-corpus gate while honestly showing
the remaining C speed advantage.

The final release snapshot uses a 1,022,035-byte fixture repeated 20 times.
Each counter is the mean of three same-session `perf stat` runs on an AMD Ryzen
7 3700X. Rust and zstd C 1.5.7 use their positive level tables through the
same one-shot framing conditions:

| Level | Rust output | C output | Rust instructions | C instructions | Gap |
|---:|---:|---:|---:|---:|---:|
| 1 | 11,430,500 | 11,430,500 | 546,890,096 | 506,395,702 | +8.00% |
| 3 | 9,977,820 | 9,978,220 | 934,970,838 | 851,136,115 | +9.85% |
| 5 | 9,767,260 | 9,780,140 | 2,013,587,932 | 2,207,009,101 | -8.76% |
| 8 | 9,686,720 | 9,686,720 | 2,763,805,960 | 2,906,994,382 | -4.93% |
| 16 | 9,211,100 | 9,216,120 | 7,878,283,979 | 8,709,360,639 | -9.54% |

Negative gaps favor Rust. Counter artifacts are
`benchmarks/tmp/release-zstd-complete-{rust,c}-l{1,3,5,8,16}.stat`.
The benchmark is intentionally reported as a focused CPU attribution case, not
as a universal throughput claim.

The historical upstream comparison is level 1 only. Upstream ruzstd 0.9.1
`Fastest`, built with one code-generation unit, produced 12,382,020 bytes and
used 2,573,721,448 instructions for the same 20-run case. `zstd-complete`
produces 11,430,500 bytes and now uses 546,890,096 instructions.

An external consumer forced to Cargo's normal 16 release codegen units produced
570,525 bytes/run for this implementation and 619,101 for upstream. Three-run
averages were 554,599,741 versus 2,583,677,784 instructions and 281,387,292
versus 1,181,786,682 cycles. Thus the advantage over upstream is not dependent
on the workspace's one-codegen-unit profile, although cgu=1 materially improves
cycles for this implementation.

Across the retained 73-file, 3.40 MiB differential corpus, level 1 totals 1,737,641 bytes
versus 1,884,617 for upstream. The complete retained matrix contains 1,095
normal, target-size, and prepared-dictionary rows and all decode. This corpus is
excellent regression evidence but remains too small and short-file-heavy for a
release-wide claim by itself. It is now supplemented by Silesia and the
generated incompressible, repetitive, structured, and long-distance streams.

The bounded frame design has a measurable ratio tradeoff. On the 3,990,328-byte
`check_dict_metrics` executable at level 3, a single frame produced 1,043,563
bytes/run; 1 MiB frames produced 1,068,617 bytes/run, 2.40% larger. The default
8 MiB chunk keeps this particular input in one frame. Streaming CPU and ratio
must be included in release benchmarks, not inferred from whole-slice results.

The repository-owned corpus generator has also been smoke-tested at 16 MiB per
fixture. At level 3/default chunks, C zstd 1.5.7 validates all five archives;
output sizes were 16,777,620 bytes (deterministic random), 8,390,996
(long-distance), 1,706 (repeated records), 1,337,412 (structured JSONL), and
550 (zeros).

The final public-API run repeated each compression three times inside each of
three `perf stat` samples. The C comparison uses one 16 MiB frame while the
bounded Rust default emits two 8 MiB frames:

| Input | Rust instructions/run | C instructions/run | Gap |
|---|---:|---:|---:|
| Deterministic random | 54.58 M | 45.10 M | +21.02% |
| Long-distance repetition | 85.71 M | 77.12 M | +11.14% |
| Repeated records | 44.78 M | 39.78 M | +12.56% |
| Structured JSONL | 239.83 M | 221.22 M | +8.41% |
| All zeros | 133.07 M | 45.44 M | +192.86% |

Raw counters are `benchmarks/tmp/release-streaming-{rust,c}-*.stat`. This
closes the generated-corpus correctness and repeated level-3 CPU gate. It also
identifies a substantial low-entropy public-stream overhead for follow-up; the
release documentation discloses it rather than presenting only one-shot core
results.

## Package topology

The five internal crates have now been consolidated into private modules under
`ruzstd/src/kernel/`. There was no valid benchmark isolating five crates versus
one crate.
The previously quoted 4.60%/2.58% Huffman and 3.14% Fast improvements compared
complete implementation changes that also moved code across crate boundaries.
They cannot be attributed to crate topology.

The release candidate is therefore one `zstd-complete` package. The moved modules
preserve the algorithms, primitive interfaces, unsafe contracts, and target-
specific section attributes. A direct comparison used a preserved release
binary from immediately before consolidation and a release binary built after
consolidation. Each encoded `corpus_z000033` 20 times, with `perf stat -r 3`:

| Level | Five-crate instructions | One-crate instructions | Change |
|---:|---:|---:|---:|
| 1 | 554,779,084 | 554,745,883 | -0.0060% |
| 3 | 960,137,657 | 960,122,449 | -0.0016% |
| 5 | 2,018,207,432 | 2,018,097,687 | -0.0054% |
| 8 | 2,768,227,822 | 2,768,107,991 | -0.0043% |
| 16 | 7,898,899,152 | 7,897,864,643 | -0.0131% |

Output bytes are exact at every level. Instruction and branch counts are
effectively unchanged; cycles and branch misses move inconsistently between
levels. The controlled experiment therefore finds no material performance
benefit from the five-crate topology. The simpler one-package layout is
retained.

## Remaining release blockers

1. Require the complete hosted matrix to pass on the final release commit.
2. Review the clean release commit, set the changelog date, and create the
   release tag only from that reviewed commit.
3. Obtain the required human decision on publication and disclose explicitly
   that no human source assessment was performed despite extensive automated,
   differential, interoperability, and benchmark validation.

The large-corpus, representative peak-RSS, generated-stream, bidirectional
zstd interoperability, and x86 forced-scalar gates are complete. Additional
multi-gigabyte streams and longer fuzz campaigns remain post-release
hardening, not blockers for 0.1.0.

The planned narrow SIMD/code-generation experiments and Rewise object-boundary
analysis remain post-release optimization/maintainability work. They are not
format-correctness or packaging blockers and must not delay this publication.
Long-running fuzzing remains ongoing hardening; the bounded pre-release
campaign, retained crash corpus, focused Miri, and AddressSanitizer gates are
complete.
