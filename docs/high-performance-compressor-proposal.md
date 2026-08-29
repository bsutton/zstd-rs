# High-performance compressor contribution proposal

## Purpose

This document proposes contributing a substantially expanded pure-Rust
Zstandard compressor to `ruzstd`. It records the evidence, licensing and review
constraints, intended Rust API, portability gates, and benchmark work before
any implementation PR is submitted.

The work uses the zstd 1.5.7 C compressor as an explicit source, algorithmic
reference, and performance oracle. It is not a clean-room implementation. The
preferred outcome is an upstream contribution; if upstream does not want the
licensing, review, maintenance, unsafe-code, or implementation surface, the
same work is intended to be published as a separate package.

This proposal does not change code or ask maintainers to accept the complete
implementation. It asks whether the project is open to evaluating it under the
conditions below.

## Review disclosure

The compressor was developed and validated with extensive automated-agent
assistance. The contributor will personally review and handle proposal and PR
discussion, but will not perform a human source-code review of the complete
implementation.

That does not satisfy the repository's current AI contribution policy, which
requires the submitting human to read and verify the contribution. Automated
tests and benchmarks are not presented as equivalent to human review. This is
disclosed before submitting the code so maintainers can decide whether to grant
an exception, perform their own assessment, request different conditions, or
decline the contribution.

## Licensing and provenance

The translated Zstandard compressor sources are offered by their authors under
either the BSD-style license or GPLv2. This work selects the BSD option.

The proposed repository/package treatment is:

- preserve Meta's copyright notice, BSD conditions, and disclaimer;
- identify zstd 1.5.7 and the exact reference commit/files in a third-party
  notice and source map;
- describe the work as derived rather than clean-room;
- retain the existing MIT terms for independently authored ruzstd code;
- declare packages containing both bodies of work as
  `MIT AND BSD-3-Clause` and package both license texts;
- retain provenance when code is moved or refactored.

This is intended to satisfy the selected BSD terms, but maintainers must decide
whether mixed licensing is acceptable for this repository. Independent advice
from an open-source/IP lawyer is optional rather than a BSD requirement; its
value would be assessing the classification of files that combine existing MIT
code and translated code.

If the repository must remain MIT-only, the implementation should be declined
or published separately, not relabelled as clean-room.

## Scope and maintenance model

The desired product is an idiomatic Rust compressor compatible with the
Zstandard format. Faithful C behavior and byte-identical output are useful
development oracles, not stable API promises.

The implementation may retain measured Rust-specific improvements and diverge
from C internals. There is no plan for recurring line-by-line audits of future
zstd C releases. Maintenance should be based on format interoperability, Rust
invariants, tests, profiles, and user-facing behavior.

## Current benchmark evidence

Latest upstream was measured at commit `eb7e03c` (`ruzstd` 0.9.1). Its only
exactly comparable implemented mode is `CompressionLevel::Fastest`, which was
compared with the proposed compressor's level 1 in an external consumer build
using one release code-generation unit.

On a 1,022,035-byte focus fixture, repeated 20 times:

| Implementation | Output bytes | Instructions | Cycles | Branches | Branch misses |
| --- | ---: | ---: | ---: | ---: | ---: |
| Proposed Rust, level 1 | 11,430,500 | 556,142,594 | 228,370,612 | 72,757,039 | 2,102,388 |
| Upstream ruzstd 0.9.1, `Fastest` | 12,382,020 | 2,573,721,448 | 1,154,823,518 | 373,654,471 | 8,642,232 |
| zstd C 1.5.7, level 1 | 11,430,500 | 507,683,580 | 184,803,245 | 49,990,496 | 1,612,169 |

These are three-repeat `perf stat` averages. Against latest upstream, the
proposed compressor produced 7.68% fewer bytes and used about 78.39% fewer
instructions and 80.22% fewer cycles. Against C, its output was identical on
this fixture, while Rust used about 9.55% more instructions and 23.58% more
cycles.

Across the retained 73-file, 3.40 MiB regression corpus:

| Implementation | Aggregate output |
| --- | ---: |
| Proposed Rust level 1 | 1,737,641 bytes |
| Upstream ruzstd 0.9.1 `Fastest` | 1,884,617 bytes |

The proposed compressor was 7.80% smaller and was larger on one file.

Against C across the same 73 fixtures, aggregate Rust-minus-C size gaps at
levels 1, 3, 5, 8, 16, 19, and 22 are respectively `-0.0961%`, `-0.0807%`,
`-0.0483%`, `-0.0139%`, `-0.0228%`, `-0.0133%`, and `-0.0049%`. The retained
normal, target-size, and prepared-dictionary matrices contain 1,095
decode-verified rows.

Against the original Rust implementation on a separate four-fixture level-1
suite, output fell from 42,955,302 to 40,068,766 bytes, a 6.72% reduction. The
immediate pre-C-port branch is 1.25% smaller on that synthetic suite because of
a JSON-specific heuristic, illustrating why C parity is not itself the final
product goal.

Instruction counts are the most stable CPU signal in these experiments.
Cycles and small layout movements require repeated, same-session measurement.

## Benchmark corpus expansion

The 73-file set is a strong differential regression suite, but is too small and
short-file-heavy for release-wide throughput, streaming-memory, or cache
claims. Before publication, add:

1. Deterministic zstd `datagen` inputs covering incompressible, repetitive,
   skewed-alphabet, structured, long-match, and 128 KiB boundary cases.
2. A licensed large-file suite covering source, text, JSON/logs, executables,
   databases, images, and already-compressed media. Silesia is a useful common
   reference, subject to recording its acquisition terms and hash.
3. Generated 1, 8, and 32 GiB streams that are never materialized completely,
   with allocator-accounted peak heap and OS RSS measurements.
4. Raw dictionaries and formatted dictionaries trained with current stable
   zstd from at least 100 representative samples, including prepared reuse,
   wrong-dictionary, and corrupted-dictionary cases.
5. Official zstd fuzz corpora for robustness, kept separate from throughput
   claims.
6. Valid golden frames and invalid cases from zstd's CLI tests.
7. Frames generated by current stable and selected older zstd releases, with
   checksum/content-size/dictionary-ID variations, concatenated and skippable
   frames, empty input, and raw/RLE edge cases.

Every Rust output must decode with current stable zstd C, plus selected older
decoders in a compatibility lane. Every valid C-produced frame must decode in
Rust. Large third-party corpora should be acquired through reproducible scripts
with their own licences and hashes rather than copied without provenance.

## Public API target

Streaming is the primary API because archives must not be buffered completely:

```rust,ignore
pub fn encode<R: std::io::Read, W: std::io::Write>(
    source: R,
    target: W,
    options: &EncoderOptions,
) -> Result<(), EncodeError>;
```

An `Encoder<W>` should implement `std::io::Write` and return
`Result<W, EncodeError>` from `finish`. `encode_all` may remain as a fallible
small-input convenience. The `no_std` core should accept bounded slices and a
caller-owned output sink without requiring an entire archive.

Additional API requirements:

- a validated Rust `CompressionLevel` with named constants; numeric conversion
  may be available through `TryFrom<i32>`, but C-named numeric entry points are
  not the normal public API;
- typed I/O, configuration, dictionary, memory-budget, and finalization errors;
- reusable `PreparedDictionary` state;
- `EncoderOptions` for level, checksum, dictionary, pledged input size, and
  memory limit;
- target compressed block size remains internal unless a significant
  independent user use case is established.

The normal/default configuration should target approximately 50-100 MiB peak
RSS. This cannot apply universally: level 22's window and match tables can
require several hundred MiB. The encoder should calculate required storage and
return `MemoryLimitExceeded`, while high-memory levels require explicit opt-in.

## Compilation architecture

The current implementation uses five small `no_std` crates, each owning a
cohesive hot transaction: Fast, DFast, row parser/search, sequence-store
preparation, and Huffman construction/emission. They are not arbitrary helper
crates.

This isolation has measured value. An earlier ordinary in-crate Huffman
boundary regressed levels 1/3/5/8 by 3.214%/1.258%/9.515%/8.915%, whereas the
separate generated unit improved them by 4.602%/2.575%/0.632%/0.513%.
Complete row, DFast, and Fast units produced additional targeted improvements.

Five public registry packages would nevertheless complicate independent
publication. The preferred consolidation experiment is one separately
compiled internal kernel crate containing five modules. It must retain the
primitive ABI and compiler barrier, compare candidate/control within one
binary, and then repeat the benchmark after duplicate code is removed.

Folding directly into the main crate is not proposed without positive
performance evidence. Structural work should extract identifiable stateful
objects or complete producer-to-consumer transactions, not small helpers.
Rewise is intended as an additional structural pass when available; it was not
available in the current tool catalogue, so no Rewise assessment is claimed.

Compile time is secondary for this performance-sensitive package. Current
performance measurements use:

```toml
[profile.release]
codegen-units = 1
```

Cargo profiles are controlled by the final application. Therefore release
gates must also benchmark a small external consumer with Cargo's default
release settings and with the documented performance profile. This is what
"downstream profiling" means here.

## Portability and unsafe-code gates

The implementation target is Linux/ELF, Windows/COFF, macOS/Mach-O, AArch64,
wasm `no_std`, and x86-64 without BMI2. Thirty-two-bit targets are deferred.

The first portability pass found and corrected an overflowing wasm32 `usize`
constant and target-specific function-section requirements. The intended
section policy is ELF `.text.sorted.*`, COFF `.text$*`, short Mach-O
`__TEXT,__*` sections, and no forced custom layout on wasm.

Current cross-build evidence includes:

- Linux/ELF workspace builds and execution;
- a final Windows/MinGW PE link;
- x86-64 Apple release object generation;
- AArch64 Android `no_std` release generation;
- wasm32 `no_std` release generation;
- all 732 active library tests passing through an explicit forced-scalar build
  on the current x86-64 host, with five tests ignored.

Native Windows, macOS x86-64/AArch64, and AArch64 Linux execution remain
required. A genuine non-BMI2 machine remains a deployment check even though
the scalar paths can now be forced end to end.

Generated primitive kernel APIs whose correctness depends on table sizes,
initialized prefixes, source bounds, or CPU support must be explicitly unsafe
with documented caller contracts. Unsafe operations should be isolated in
small leaves, with `unsafe_op_in_unsafe_fn` denied for the derived compressor
and kernel units. Required gates include strict Clippy, Miri where applicable,
sanitizers, fuzzing, differential tests, and exact decode verification.

## Proposed path

1. Use this documentation PR to discuss licensing, the AI-policy exception,
   public API direction, maintenance expectations, and required evidence.
2. If upstream is interested, reconcile the implementation with current
   `master` and complete the streaming/memory API and remaining platform gates.
   Rebasing is integration work, not a performance optimization.
3. Submit the implementation as one coherent PR rather than splitting the
   compiler-coupled implementation merely to make review appear smaller.
4. Include reproducible benchmark scripts, raw counter artifacts, corpus
   provenance, mixed-license files, and unsafe invariant documentation.
5. Keep refactors and crate consolidation only when whole-compressor
   measurements support them.
6. If upstream declines, publish a separately named package under the same
   mixed-license and validation requirements.

## Questions for maintainers

1. Is the proposed `MIT AND BSD-3-Clause` package arrangement acceptable?
2. Would the project consider an explicit exception to its human-review policy
   given the disclosure above, or should this work be published separately?
3. Does the streaming, typed-error, memory-budget API direction fit ruzstd?
4. Is one separately compiled internal kernel crate preferable to five internal
   packages if its performance is validated?
5. Which additional platforms, corpora, and safety gates would maintainers
   require before considering the implementation?

