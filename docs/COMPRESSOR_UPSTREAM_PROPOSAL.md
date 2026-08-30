# Proposed high-performance Rust compressor contribution

## Proposal

We have developed a pure-Rust Zstandard compressor using the zstd 1.5.7 C
compressor as an explicit algorithmic and performance reference. We would
prefer to contribute the result to `KillingSpark/zstd-rs`, but intend to publish
it as a separate package if upstream does not want to take on its licensing,
size, unsafe-code, or maintenance surface.

This is not offered as a clean-room implementation or as an ongoing faithful
port. The intended product is an idiomatic Rust compressor compatible with the
Zstandard file format. It may retain improvements that differ from C and there
is no plan for recurring audits of future C releases.

## Licensing and provenance

The translated Zstandard sources are dual-licensed BSD-style or GPLv2. We
select the BSD-style option, and the separately published `zstd-complete`
package uses `BSD-3-Clause` as its outbound license. Meta's notice, conditions,
and disclaimer are retained in `LICENSE`; provenance is disclosed in
`THIRD_PARTY_NOTICES.md` and the compressor source map. The copyright and MIT
permission notice for substantial inherited ruzstd decoder and supporting code
is also preserved as required attribution in `LICENSES/ruzstd-MIT.txt`.

For an upstream contribution, we ask the maintainer to confirm whether this
BSD-licensed package direction is acceptable for the MIT-licensed repository.
If the repository must remain MIT-only, the compressor will remain in the
separately published package rather than be described as clean-room or have
Meta's required notice removed.

## Review disclosure

The contribution was developed and validated with extensive automated-agent
assistance. The contributor will review and personally handle all proposal and
PR discussions, but will not perform a human source-code review of the complete
implementation. This does not meet the repository's currently stated AI
contribution policy. We disclose that directly so the maintainers can decide
whether they are willing to inspect, adopt, or decline the code.

The substitute evidence is not presented as equivalent to human review. It
includes differential byte and decode oracles, all-strategy state-transition
tests, broad normal/target/dictionary matrices, strict lint/build gates, and
repeated hardware-counter benchmarks. The optimization report is maintained in
`benchmarks/C_PORT_PAUSE_REPORT_2026-08-29.md`; publication gates and current
upstream/C comparisons are in `docs/RELEASE_READINESS.md`.

## Intended Rust API

- User-facing levels are validated `CompressionLevel` values. Numeric C and
  target-size hooks are behind a non-default validation feature.
- `Encoder<W>` implements `std::io::Write`; bounded chunks become independent
  frames, so total input size is not retained in memory.
- `EncoderOptions` covers level, frame chunk, explicit memory budget, checksum,
  and a reusable prepared dictionary. I/O/configuration failures are typed
  `Result` errors.
- The default is an 8 MiB frame chunk and 96 MiB conservative budget. Local
  level-3 profiling peaked below 20 MiB RSS on 128-256 MiB generated files.
- Target block-size controls remain public only if they have a clear,
  independently useful Rust-facing use case.
- Exact C output is not an API promise. Zstandard format compatibility and
  interoperability are required.

## Architecture and performance

Five temporary no-std code-generation packages contained cohesive hot
transactions, but no benchmark isolated that package topology from the
algorithm and code-generation changes introduced at the same time. They are
now private modules in the main crate. Earlier
4.60%/2.58% Huffman and 3.14% Fast improvements must not be cited as evidence
that separate crates are faster.

A controlled consolidation experiment preserved algorithms and primitive
interfaces while changing the compilation/package boundary. Against the
preserved five-crate binary, one-crate instruction changes at levels
1/3/5/8/16 were -0.0060%/-0.0016%/-0.0054%/-0.0043%/-0.0131%, with exact
output bytes. This is neutral and provides no reason to publish internal
packages, so the proposal uses one `ruzstd` package. We will also run Rewise if
available and prefer identifiable stateful objects or complete producer-to-
consumer transactions over collections of small helpers.

Rewise was requested for the structural pass, but it was not available in the
current Codex plugin catalogue. No Rewise result is claimed. The measured
crate-to-module consolidation and existing object/state boundaries are used
for the initial release; Rewise remains post-release maintainability work.

The workspace deliberately uses one release codegen unit. Compile time is a
secondary concern for this performance-oriented package, but consumer builds
must still be measured because dependency profile settings and linker behavior
can differ from workspace benchmarks.

## Portability and safety gates

The submission target includes Linux/ELF, macOS/Mach-O, Windows/COFF, AArch64,
wasm no-std, and x86-64 without BMI2. A scalar portable path remains mandatory.
Thirty-two-bit support is deferred. Unsafe operations must have local safety
contracts, invariant tests, and applicable Miri, sanitizer, fuzz, and
differential coverage.

The first cross-build pass found and fixed a wasm32 `usize` overflow and made
all custom code sections target-specific: ELF `.text.sorted.*`, COFF lexical
`.text$*` subsections, short Mach-O `__TEXT,__*` sections, and no forced custom
section on wasm. Release object generation now passes for x86-64/AArch64
Apple, Windows x86-64, Linux AArch64 `no_std`, and wasm `no_std`; Windows also
reaches a final PE link. Native Windows and Apple-Silicon execution is present
in the hosted release matrix. A diagnostic `force-scalar` build has passed the
full 758-test library gate (five ignored) through the portable compressor paths
on the current x86-64 host.

The generated primitive kernel entry points are now explicitly unsafe and
document their caller contracts instead of exposing safe APIs backed only by
debug assertions. `unsafe_op_in_unsafe_fn` is denied in the derived compressor
and kernel units. The broader pre-existing decoder unsafe surface remains a
separate audit item.

## Current benchmark evidence

The primary publication comparison is the standard 12-file, 211,938,580-byte
Silesia corpus. Inputs were resident before timing and both implementations
returned owned compressed buffers. Rust used the public bounded API with 8 MiB
frames; zstd C 1.5.7 used `ZSTD_compress2()` with one frame per file. At levels
1/3/8, aggregate Rust-minus-C size gaps are -0.024%/+0.024%/+0.184%, while
instruction gaps are +19.99%/+8.83%/+6.66%. Rust throughput is
295.1/212.3/59.5 MB/s versus C's 403.2/240.0/69.4 MB/s. Full methodology and
provenance are in `benchmarks/RELEASE_BENCHMARKS_2026-08-30.md`.

On the 73-file, 3.40 MiB regression corpus, level 1 from this branch totals
1,737,641 bytes versus 1,884,617 bytes for current upstream `ruzstd` 0.9.1
`Fastest`: 7.80% smaller, with this branch larger on one file. On the 1,022,035
byte focus fixture over 20 repetitions, the respective totals are 11,430,500
and 12,382,020 bytes. Three-run `perf stat` averages are 556,142,594 versus
2,573,721,448 instructions and 228,370,612 versus 1,154,823,518 cycles.

An external consumer forced to Cargo's normal 16 release codegen units retained
the same output sizes per run (570,525 versus 619,101) and averaged 554,599,741
versus 2,583,677,784 instructions. This checks that the large upstream
advantage is not an artifact of the workspace's one-codegen-unit profile.

The final focused one-shot comparison against zstd C at levels 1/3/5/8/16 has
Rust instruction gaps of +8.00%/+9.85%/-8.76%/-4.93%/-9.54%; this isolates the
core but is not substituted for Silesia. Broader matrices cover 73 fixtures at
levels 1, 3, 5, 8, 16, 19, and 22, plus target-size and prepared-dictionary
modes; all 1,095 retained rows decode. Another 245 Rust outputs made from 35
official zstd source fixtures decode with C, while Rust exactly decodes the
four official zstd golden frames. Generated 16 MiB incompressible, repetitive,
structured, long-distance, and zero streams expose shape-specific behavior.
Third-party corpus bytes are referenced rather than redistributed.

## Questions for upstream

1. Is a BSD-3-Clause contribution acceptable with the proposed notices and
   provenance in the currently MIT-licensed repository?
2. Given the explicit absence of full human source review, are maintainers
   willing to evaluate or adopt the code under an exception to the stated AI
   contribution policy?
3. Is the consolidated private-module layout acceptable, or would upstream
   require different internal object boundaries after its own review?
4. Which public streaming/configuration shape best fits ruzstd's existing API?
5. What additional benchmark evidence and target platforms would upstream require before
   considering the implementation production-ready?
