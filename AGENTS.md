# Agent Workflow

## Read This First On Resume

This file is the durable restart handoff for Codex work in this repository and
is the place to put notes that must be seen when this project is resumed. Codex
reads `AGENTS.md` when entering this project, so keep restart-critical notes
near the top of this file. Start here, then read
`ruzstd/src/encoding/levels/c_port/README.md` before continuing the C compressor
port.

Restart anchor: if a future session starts cold, use this file as the canonical
handoff and continue from the "Next Resume Action" section before doing new
analysis or code changes.

Release completion as of August 31, 2026: `zstd-complete` 0.2.0 was published
from commit `b756ed244ad9ed6886d3c8bb6edf9c233367bc97` through crates.io Trusted
Publishing and tagged `zstd-complete-v0.2.0`. Hosted CI run `33336565495`
passed every job; publish run `33340155052` completed successfully. The final
crate artifact SHA-256 was
`1fbd3540d270b086caa6f53b83aa9d7e12f5ba870029da73b6950fd530610feb`.
The crates.io API and docs.rs both exposed 0.2.0, and a fresh registry consumer
passed Rust-to-zstd-C and zstd-C-to-Rust round trips. Issue #11 is merged; its
GitHub issue remains open only because the lifecycle policy requires an
explicit human statement that review is complete before applying `reviewed`
and closing it. Resume post-release Rewise/SIMD work only when requested.

Workspace-context checkpoint as of August 30, 2026: issue #11 is implemented
directly on `master` under the user's explicit override of the issue-worktree
default. `EncoderWorkspace`/`DecoderWorkspace` own reusable typed state;
`StaticEncoderWorkspace`/`StaticDecoderWorkspace` place the same state in an
arbitrary caller byte slice. Valid bounded operations write into caller output
without allocation, including raw/formatted decoder dictionaries,
uncompressed and negative-fast encoder modes, all positive strategies, the
level-16 post-block splitter, and automatic level-22 long-distance matching.
The multithreaded encoder prepares one reusable workspace per eligible worker;
the default single-thread entry points remain unchanged.

Counting-allocator, C-decode, repeated-operation, no-default, multithreaded,
dictionary-builder, strict Clippy, doctest, wasm, and large level-22 LDM gates
pass. The forced LDM test uses a 64 MiB bound, performs two zero-allocation
operations, and peaks near 740 MiB RSS. The final preserved-binary comparison
at levels 1/3/5/8/16 is recorded in
`benchmarks/WORKSPACE_CONTEXTS_2026-08-30.md`: ordinary instruction changes
are -0.012%/+0.123%/-0.622%/-0.454%/-0.581%, workspace operations are another
0.09-0.51% cheaper, and all measured output is byte-identical. Retain the
arena implementation. This validated transaction completes issue #11. The next
release must choose and set a version newer than the already-published 0.1.0;
do not attempt to republish 0.1.0.

Publishing correction as of August 30, 2026: `zstd-complete` 0.1.0 is live on
crates.io and the exact published commit is tagged `zstd-complete-v0.1.0`.
Earlier release notes below saying that the name is still available or that
the upload remains pending are stale historical checkpoints. The repository
now contains `.github/workflows/release.yml` for subsequent OIDC releases; it
uses the exact Trusted Publisher claims owner `bsutton`, repository `zstd-rs`,
workflow `release.yml`, and GitHub environment `release`. The crate owner must
register those claims in the crate's crates.io Settings page. Do not create or
retain a long-lived Cargo registry secret after Trusted Publishing is enabled.

Feature-parity checkpoint as of August 30, 2026: roadmap issue #5's remaining
application-level gaps are implemented directly on `master`. The standard
library API now has a bounded `MultiFrameDecoder` for concatenated and
skippable frames; negative fast levels; typed strategy, matcher, LDM, target
block, pledged-size, and frame-content-size controls; and opt-in formatted
dictionary training from independent samples. `docs/ZSTD_1_5_7_SUPPORT_MATRIX.md`
records the intentional boundary: RFC 8878/application capability parity is
the objective, while the C ABI, custom allocators, deprecated symbols, numeric
experimental parameters, and byte-identical output are not. Default encoder
options explicitly retain the pre-existing specialized single-thread frame
functions; a regression test requires exact output from that path. Luna/medium
validation passes formatting, strict default/multithreaded/no-default Clippy,
workspace tests, 776 multithreaded tests plus five ignored, 689 no-default
tests, and all doctests. Child issues are #7 (archive decoder), #8 (advanced
controls), #9 (formatted training), and #10 (capability audit).

Active issue checkpoint as of August 30, 2026: issue #6 is implemented on
branch `issue-6-multithreaded-encoder` in persistent worktree
`/home/bsutton/git/.codex.workspaces/zstd-rs-issue-6-multithreaded-encoder`.
The non-default `multithreading` feature adds a bounded ordered
`ParallelEncoder`; one worker delegates to the unchanged `Encoder`, while two
or more workers compress independent frames with thread-local state. The
all-input one-worker helpers call the direct encoder. Normalized-path control
and candidate release binaries have byte-identical `.text`; same-binary
one-worker instruction deltas across levels 1/3/5/8/16 are neutral. Silesia
four-worker speedups are 1.77x/1.85x/2.25x at levels 1/3/8. True streaming RSS
on the 51 MiB `mozilla` file was 11,188 KiB at one worker and 35,420 KiB at
four. Full feature tests, 117-configuration check/Clippy powersets, focused
no-default/forced-scalar, Rust 1.87, four cross-targets, Miri, ASan, rustdoc,
formatting, and package consumer checks pass. Worker panic containment now has
explicit failure injection. See `docs/MULTITHREADED_COMPRESSION.md`. Keep the
default path free of worker checks. Persistent pool reuse remains optional
follow-up work for extremely cheap inputs where thread startup dominates.

Release checkpoint as of August 30, 2026: optimization is stopped and the
publishable package is now `zstd-complete` 0.1.0 (Rust import
`zstd_complete`). The crates.io API returned 404 for the name and repeated live
`cargo publish -p zstd-complete --dry-run --allow-dirty` runs passed; the name
is not reserved until actual publication. The clean regenerated crate artifact
is `target/package/zstd-complete-0.1.0.crate`: 235 files, 2.6 MiB unpacked and
about 500 KiB compressed. It passes `gzip -t`, builds from packaged source, and
contains its BSD-3-Clause license, inherited ruzstd MIT attribution, provenance
notice, and release README. Recompute its
SHA-256 after the release commit is fixed because Cargo embeds that commit in
`.cargo_vcs_info.json`; an artifact hash cannot be self-consistently recorded
inside the same commit. Push the release commit and require the hosted
Windows/macOS/AArch64 CI matrix to pass. Then follow `docs/RELEASING.md`; the
actual crates.io upload still requires explicit human authorization.

Final local release gates pass: 758 default tests plus seven doctests, 769
`dict_builder` tests, 689 no-default-feature tests, the full forced-scalar
suite, strict all-target Clippy, formatting/diff checks, Rust 1.87 MSRV,
docs.rs-configured rustdoc, wasm/Linux-AArch64 no_std builds, Windows x86-64
and macOS x86-64/AArch64 builds, four real focused Miri tests, and 353 focused
AddressSanitizer tests. Bounded libFuzzer/ASan runs completed 2,644 decoder
cases and 424 public-encoder cases; encoder output was decoded by Rust and zstd
C. ELF/COFF/Mach-O section flags were inspected and wasm correctly omits the
native sections. The README discloses the absence of an independent human
line-by-line review and gives both the final focused C counters and the public
16 MiB streaming matrix, including the large all-zero CPU deficit. Current
release-only remaining work is hosted native CI, a clean commit/tag, explicit
human publication approval, and the actual upload. Licensed large-corpus and
long-running fuzz expansion are follow-up hardening; SIMD/Rewise remain
post-release optimization work.

Large-corpus release evidence added August 30, 2026: the standard 12-file,
211,938,580-byte Silesia corpus was acquired from mirror commit
`3f3fa2cdbbb3795c903b74e774acb309e1360337`; every file matches its published
MD5. The fair resident-input/owned-output public-API comparison at levels
1/3/8 has aggregate Rust-minus-C size gaps of -0.024%/+0.024%/+0.184% and
instruction gaps of +19.99%/+8.83%/+6.66%. Rust throughput was
295.1/212.3/59.5 MB/s versus C 403.2/240.0/69.4 MB/s. See
`benchmarks/RELEASE_BENCHMARKS_2026-08-30.md`; do not substitute the earlier
mixed file-I/O timing artifacts. All Rust outputs decoded with C and all 12 C
level-3 outputs decoded exactly with Rust. The official zstd 1.5.7 source suite
adds 245 C-decoded Rust archives across seven levels, and Rust exactly decoded
the four official golden frames. This closes the large recognized corpus and
broader bidirectional fixture release gates.

Publication direction superseding the former faithful-port end state as of
August 29, 2026: zstd 1.5.7 remains the provenance and algorithmic reference,
but this project is no longer trying to remain a line-for-line behavioral port
or to conduct recurring audits of later C releases. The product goal is an
idiomatic, format-compatible, high-performance pure-Rust compressor. Preserve
measured improvements even when they intentionally diverge from C decisions.
Public APIs must use Rust concepts (`CompressionLevel`, streaming `Read`/`Write`
or explicit incremental state, and `Result` errors), support inputs larger than
memory, and target roughly 50-100 MiB peak RSS under normal streaming use.
Numeric C-level and `targetCBlockSize` validation hooks must not remain public
unless they have an independently useful Rust-facing contract.

Release-preparation checkpoint as of August 29, 2026: the first bounded public
encoder is implemented in `encoding::streaming_encoder`. `Encoder<W>` emits
independent 8 MiB frames by default, implements `std::io::Write`, and has
fallible `finish`; `encode`/`encode_all` propagate I/O errors. `EncoderOptions`
contains a validated level, 96 MiB default memory budget, frame chunk size,
optional checksum, and reusable `EncoderDictionary`. Level 22 requires an
explicitly larger budget. Numeric C/target APIs are now gated behind the
non-default `c-port-validation` feature used by tools. Tests cover concatenated
Rust/C decode, checksums, dictionaries, empty/raw frames, write failures,
bounded 64 MiB generated input, and budget rejection. Local peak RSS at level
3/default settings was 19,852 KiB for a 128 MiB random file and 11,900 KiB for
a 256 MiB sparse file. Read `docs/RELEASE_READINESS.md` for exact benchmark
tables, topology decision, and remaining blockers. The new
`generate_release_corpus` tool deterministically creates zeros, random,
repeated-record, structured-JSON, and long-distance fixtures; its five 16 MiB
smoke fixtures all pass zstd C 1.5.7 archive validation at level 3.

Next Resume Action: finish release gates before resuming low-level CPU/SIMD
experiments. Prepare a clean release commit, push it, and require the hosted
Windows/macOS/AArch64/wasm/forced-scalar/Miri/ASan/package CI matrix to pass.
Then obtain explicit human authorization for `cargo publish` and follow
`docs/RELEASING.md`. Run Rewise and SIMD experiments after publication.

Crate-topology correction superseding historical interpretations below: the
recorded 4.602%/2.575% Huffman and 3.1406% Fast improvements are valid
end-to-end measurements of their complete candidate transactions, but they do
not measure separate crate versus module. Historical instructions such as
"preserve the dedicated crate" document the conclusion reached at the time;
they are not evidence. The five crates have now been moved into private
`ruzstd/src/kernel/` modules with their primitive interfaces and section
attributes retained. Complete correctness and topology-only performance gates
showed no material regression, so retain this consolidated layout. Against the
preserved five-crate release binary, one-crate instruction changes at levels
1/3/5/8/16 were -0.0060%/-0.0016%/-0.0054%/-0.0043%/-0.0131%; output bytes
were exact and branch counts were likewise neutral. The artifacts are
`benchmarks/tmp/perf-z000033-topology-{five-crates,single-crate}-l*.stat`.

Upstream/publishing policy: the separately published package selects
BSD-3-Clause while preserving inherited ruzstd MIT attribution and complete
provenance. Disclose that there will be no human source-code
review. The human contributor will review and handle PR discussions; the code
has instead undergone extensive differential correctness and benchmark gates.
This does not satisfy upstream's present human-review policy, so maintainers
must explicitly decide whether they want to inspect/adopt the code. Publication
as a separate package remains the fallback and is expected even if upstream
declines. Do not split the implementation into many upstream PRs merely for
review convenience. Run Rewise before publication if it becomes available,
and extract only cohesive identifiable objects/transactions, not collections
of small helpers. Benchmark every performance-sensitive refactor.

Pause checkpoint as of August 29, 2026: work is deliberately paused on branch
`faithful-c-compressor-port` at committed HEAD `433d266`, with the complete
C-port implementation still present as a large uncommitted worktree (97
modified tracked paths and 37 untracked paths after writing this pause report).
Do not clean,
reset, checkout, or recreate this worktree. The untracked codegen crates and
split Rust modules are part of the retained implementation, not disposable
build output. A concise benchmark and restart report is in
`benchmarks/C_PORT_PAUSE_REPORT_2026-08-29.md`; read it after this file and
`ruzstd/src/encoding/levels/c_port/README.md`.

After the release-preparation gates above, resume low-level CPU parity at
levels 1 and 3 on `corpus_z000033`, below the already-cheaper Fast and DFast
matchers. Refresh same-session Rust/C attribution before changing code, then
work in shared Huffman construction/emission or the sequence statistics/table-
selection producer-to-consumer boundary. Preserve the retained dynamic BMI2
entropy functions, explicit fixed-width Huffman unroll, native SeqStore paths,
matcher-produced repeat-offset handoff, C-width DFast counts, and frame-owned
Huffman/FSE scratch and output-table recycling. Do not retry the rejected
initialized row candidate buffer, wrapper-only sequence orchestration,
in-record sequence-code marker cache, blanket C high-bit LL/ML conversion,
or standalone generated Huffman-weight FSE serializer without new causal
evidence.

Required SIMD follow-up when benchmarking resumes: after refreshing the
level-1/3 attribution, perform a narrow SIMD/code-generation audit and measure
each viable candidate with the normal same-binary controls. First inspect the
release assembly and counters for the retained four-lane literal histogram to
see whether LLVM already schedules or vectorizes it effectively. Then, only
where attribution supports it, test AVX2 row-tag masks against retained SSE2,
a length-gated SIMD long-match comparison, and batched LL/ML sequence-code
classification. Keep the existing scalar/portable paths and runtime feature
dispatch. Do not treat BMI2 as SIMD, and do not spend the first tranche on
Fast/DFast matching unless the refreshed whole-compressor profile contradicts
the current evidence that both Rust matchers are already cheaper than C.
Require exact byte/decode gates plus instruction, cycle, branch, and miss
counters; reject candidates that merely reduce branches or symbol size while
regressing stable end-to-end instructions. Also record whether an ARM NEON
analogue is feasible, but do not implement or benchmark NEON on an x86 host.

Optimization checkpoint as of August 30, 2026: the first resumed attribution
and SIMD audit are complete. Fresh paired 20-run counters on
`corpus_z000033` were Rust/C level 1 `554,872,570`/`506,400,715`
instructions and level 3 `960,035,636`/`851,049,382`; Rust therefore began
this tranche about `9.57%` and `12.81%` above C. Output was exact at level 1,
while Rust was 20 bytes smaller across 20 level-3 runs. Instruction profiles
confirmed both Fast/DFast matchers remain cheaper than C in absolute terms;
the useful excess was still in entropy statistics and emission.

The literal-histogram SIMD audit found that LLVM already emits SSE2 packed
adds for the retained four-lane merge. The hot byte-to-bin updates are
data-dependent scatters, which AVX2 and NEON cannot express directly. Do not
replace this already-cheaper-than-C histogram with gather/extract machinery;
AVX-512 conflict-detection/scatter could be reconsidered only as a separately
dispatched, broadly available-enough experiment.

Two post-count sequence-statistics transactions are retained. DFast now only
increments its three compact `u32` count lanes during the record walk, then
derives most-frequent and maximum-symbol metadata from the bounded lanes once.
Its same-binary control was `RUZSTD_TUNE_C_DFAST_POST_COUNT_STATS=0` before
removal. Level-3 candidate/control instructions were
`939,787,271`/`966,011,665` (`-2.7147%`); levels 1/5/8/16 were instruction-
neutral. After removing the control, level 3 measured `940,276,179`
instructions, about `2.06%` below the fresh pre-change binary. Fast now also
increments its three `CodeCounts` arrays without per-record maximum/total
bookkeeping and derives those fields once afterward. Its removed same-binary
control was `RUZSTD_TUNE_C_FAST_POST_COUNT_STATS=0`; three paired 50-run
level-1 samples improved instructions by `1.7289%`, `1.9743%`, and `2.0391%`.
Cycle results improved in two samples and were `+0.6090%` in the noisy third;
branch counts were neutral. Each transaction passed 365 normal, 292 target-
2048, and 292 prepared-CDict candidate/control rows with exact first five CSV
columns and successful decoding. Artifacts begin with
`dfast-post-count-stats-` and `fast-post-count-stats-`; the refreshed profiles
begin with `profile-z000033-resume-`. Continue from the retained post-count
baseline. The next attribution targets are level-1 Huffman emission and
level-3 sequence FSE emission/table construction; do not return to histogram
SIMD without new hardware support or causal evidence.

The first clean post-control three-run samples are level 1 `549,164,078`
instructions and level 3 `938,805,849` instructions, with focused output still
`11,430,500` and `9,977,820` bytes for 20 runs. Relative to the fresh resumed
pre-change samples, instructions are about `1.03%` lower at level 1 and
`2.21%` lower at level 3; same-binary controls remain the authoritative
attribution because removing controls also changes layout.

Latest kept optimization from August 30, 2026: small literal-length and
match-length lookup entries now use an explicit 8-byte `SmallLengthCode`
(`u32` additional bits plus `u8` code and bit width) instead of Rust tuples
whose `usize` member made every entry 16 bytes. The public internal helpers
still return `(u8, u32, usize)` and every formula is unchanged; widening occurs
only after the compact table load. A compile-time layout assertion fixes the
entry size. The two hot tables shrink from 3,072 to 1,536 bytes.

Against a preserved immediately preceding release binary, focused 20-run
instruction changes at levels 1/3/5/8/16 were `-0.6041%`, `-0.3970%`,
`-0.2200%`, `-0.1527%`, and `-5.5166%`. The large optimal-level effect was
repeated in three independent 20-run pairs: `-5.5851%`, `-5.5445%`, and
`-5.5598%` instructions; branches improved about `7.0%`, misses about
`17.4%`, and all three cycle samples improved. This is a representation and
working-set win used throughout optimal pricing/parsing, not merely a sequence
emitter improvement. All 511 normal, 292 target-2048, and 292 prepared-CDict
candidate/control rows matched in their first five CSV columns and decoded.
Validation passed 758 library tests with 5 ignored, 7 integration tests,
strict all-target Clippy, wasm `no_std`, formatting, and `git diff --check`.

Latest rejected FSE representation experiment from August 30, 2026: the
12-byte `SymbolTransform` was temporarily narrowed to an 8-byte record by
storing normalized probability and `delta_find_state` as `i16`. The bounds
were proven from the maximum table log of 12 (`probability` is in
`[-1, 4096]`; `delta_find_state` is in `[-4096, 4093]`), checked conversions
preserved the existing `i32` calculation semantics, focused FSE oracles
passed, and focused output remained exact. Against the preserved immediately
preceding binary, instructions changed by `-0.2547%`, `-0.3552%`, `+0.3275%`,
`-0.0016%`, and `-0.3235%` at levels 1/3/5/8/16. The level-5 regression
reproduced in two further independent pairs at `+0.3183%` and `+0.2610%`, with
a corresponding branch increase. The candidate was therefore reverted. Do
not compact the shared transform globally again without a design that avoids
the Greedy level-5 regression. Counter artifacts begin with
`perf-z000033-compact-fse-transform-`.

A second rejected variant separated C's full-width 8-byte hot encoding pair
(`delta_nb_bits`, `delta_find_state`) from a recycled compact `i16`
probability vector. This removed narrowing/sign extension from the encoder and
made the producer/consumer split explicit, but the extra cold-metadata
ownership still regressed level 5. Preserved-binary instruction changes at
levels 1/3/5/8/16 were `-0.4585%`, `-0.3241%`, `+0.3571%`, `+0.0536%`, and
`-4.0657%`; level-5 branches rose about `0.81%`. This candidate was also
reverted. Artifacts begin with `perf-z000033-split-fse-transform-`. A future
attempt must avoid changing the shared `FSETable` ownership/ABI for
Greedy/Lazy/optimal, likely by specializing a complete Fast/DFast consumer
boundary rather than adding another vector to every table.

Latest rejected FSE load-shape experiment from August 30, 2026: after a paired
C/Rust profile showed Rust's level-3 sequence emitter using about 16% more
absolute instructions, `encode_symbol()` temporarily read the existing
contiguous full-width `delta_nb_bits` and `delta_find_state` fields with one
unaligned `u64` load and explicit endian extraction, matching C's packed-load
shape without changing the 12-byte record. Focused oracles and output passed,
but preserved-binary instruction changes at levels 1/3/5/8/16 were
`+0.2100%`, `-0.0110%`, `+0.1164%`, `+0.0718%`, and `-5.3270%`. The intended
level 3 was neutral and three guards regressed, so the candidate and added
unsafe read were removed. Retain LLVM's separate field loads. Artifacts begin
with `perf-z000033-packed-fse-transform-load-`; paired profiles are
`profile-z000033-after-compact-length-rust-l3.perf.data` and
`profile-z000033-current-c-api-l3.perf.data`.

After reverting all three FSE experiments, the accepted release tools rebuilt
byte-for-byte to the preserved pre-experiment binaries: `profile_c_port`
SHA-256 `ca371dd0ed962589ff746a43ff8336fbc5bc3f4968e855ef16c8f41d7a0588b4`
and `benchmark_c_port`
`4a169e1e0991e9eae1f60c7d507893b075b4153abd80ab774dfc12a6df594a0e`.
Fresh three-run 20-iteration samples are level 1 `546,912,692` instructions
and level 3 `934,905,679` instructions with exact retained output totals.
Artifacts end in `after-fse-experiment-reverts-l{1,3}.stat`. Resume from this
accepted compact-length/post-count binary, not any intervening candidate.

Latest rejected Fast/DFast producer-to-consumer experiment from August 30,
2026: after literal gathering, the existing statistics walk temporarily
converted each 12-byte matcher-owned `StoredSequence` in place to a distinct
12-byte entropy record carrying LL/ML code, extra bits, and bit width. The
reverse all-table, mixed-table, and RLE emitters consumed that typed view
directly. This was the complete replacement requested by the earlier handoff:
there was no allocation, larger record, sidecar, marker, or additional
traversal, and Greedy/Lazy/optimal retained their original record path. The
same-binary control was
`RUZSTD_TUNE_C_FAST_ENTROPY_SEQUENCE_HANDOFF=0`.

Exhaustive packing tests covered every legal block-sized LL and ML value. The
candidate passed 759 library tests with 5 ignored; the control passed the
direct two-block entropy and representative strategy-frame oracles. Focused
candidate/control output was exact. Same-binary 20-run instruction medians at
levels 1/3/5/8/16 were `550,153,251`/`545,685,548`,
`944,893,783`/`935,199,971`, `2,013,650,314`/`2,013,608,179`,
`2,765,531,557`/`2,764,229,328`, and
`7,892,551,168`/`7,885,684,486`: changes of `+0.8187%`, `+1.0365%`,
`+0.0021%`, `+0.0471%`, and `+0.0871%`. The intended low levels regressed
decisively. The compact LL/ML lookup retained immediately before this
experiment makes on-demand classification cheaper than packing during the
producer pass and unpacking during emission. The candidate, control, mutable
plumbing, added unsafe conversion, and tests were completely removed.
Artifacts begin with `perf-z000033-fast-entropy-sequence-handoff-`. Do not
retry code-ready sequence handoff before release without a representation that
removes more work than the retained compact lookup path.

Final optimization-stop checkpoint for crates.io publication: after removing
the rejected handoff, full validation returned to 758 passed library tests
with 5 ignored, formatting and `git diff --check` passed, and both release
tools rebuilt byte-for-byte to the accepted preserved binaries (`profile_c_port`
SHA-256 `ca371dd0ed962589ff746a43ff8336fbc5bc3f4968e855ef16c8f41d7a0588b4`,
`benchmark_c_port`
`4a169e1e0991e9eae1f60c7d507893b075b4153abd80ab774dfc12a6df594a0e`).
Fresh three-run 20-iteration samples are level 1 `546,924,938` instructions
and level 3 `934,938,591` instructions with retained output totals. Artifacts
end in `after-fast-entropy-handoff-revert-l{1,3}.stat`. Stop optimization here
and proceed with the crates.io publication checklist; do not start another
benchmark experiment before publishing unless explicitly requested.
Artifacts begin with `compact-length-codes-`; the preserved control SHA-256
is `a4aca867c8ee1f2d875e3a8ec956a3ea839a13973ccd6563ca3b0f37616620c6`.
Retain the compact tables.

Rejected immediately before the compact-table win: BMI2 Huffman emission
selected its table-log unroll once outside the four-stream loop and then ran a
const-specialized four-stream body. Output and focused tests were exact, but
level-1 candidate/control instructions were
`1,361,073,948`/`1,361,014,532` (`+0.0044%`), cycles regressed about `1.02%`,
and the selected symbol grew from roughly 5.9 KiB to 8.0 KiB. The candidate,
control switch, and duplicated body were removed. Artifacts begin with
`perf-z000033-huffman-select-unroll-once-`. Do not retry dispatch-only Huffman
specialization; it removes negligible branches while increasing generated
code. Continue with compact-table attribution before selecting the next inner
producer-to-consumer boundary.

Current restart checkpoint as of July 18, 2026: target-mode byte parity is
closed for the focused level-16 case and broadly validated across the
real-world fixture set at target sizes 2048, 4096, and 8192. Normal-mode size
parity is slightly in Rust's favor on the current high-level broad benchmark.
The attached-dictionary specialization and the latest C-style entropy work put
Rust below C for focused instructions and same-session cycles, while Rust still
executes more branches. The
active focused benchmark is
`benchmarks/archive/tmp/realworld-100/corpus_z000033` at level 16. The latest
paired Rust sample is `36,844,400` output bytes, `31,036,963,180` instructions,
`17,678,427,275` cycles, `4,805,010,253` branches, and `193,896,413` branch
misses for 80 runs. Same-session C API is `36,864,480` output bytes,
`34,608,032,979` instructions, `18,731,833,914` cycles, `4,309,914,383`
branches, and `173,172,484` branch misses. The focused size gap is about
`-0.0545%`, Rust uses about `10.32%` fewer instructions and `5.62%` fewer
cycles in these paired samples, and the branch-count gap is about `+11.49%`.
Stable focused 20-run Rust samples after the direct Huffman bucket subslice
were `7,759,785,101`, `7,759,451,002`, and `7,759,785,299` instructions, with
about `1.20134B` branches. Treat cycles as
noisier than instructions;
branch/control-flow shape remains the useful residual CPU signal. Current
counter artifacts:
`benchmarks/tmp/perf-z000033-l16-rust-after-bucket-subslice.stat` and
`benchmarks/tmp/perf-z000033-l16-c-api-after-bucket-subslice.stat`.
The latest broad one-run API artifact is
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-bucket-subslice.csv`;
aggregate gaps at levels 1, 3, 5, 8, 16, 19, and 22 are respectively `-1671`,
`-1283`, `-714`, `-203`, `-315`, `-174`, and `-64` bytes versus C across 73
fixtures. Level 16 has no positive rows.

Current low-level CPU checkpoint as of August 28, 2026: the audited raw row
table, C-style Greedy/Lazy strategy/dictionary-mode specialization, isolated
fused no-dictionary row search, and once-per-block generated-function
selection are retained. The complete no-dictionary repeat-match continuation
is also inlined into the generated parser bodies. The new
`ruzstd-row-codegen` crate generates C's nine `(minMatch,rowLog)` row-search
functions behind a primitive slice/scalar ABI. Each function owns the cached
row updates, uninitialized bounded candidate collection and prefetch, current
position insertion, and match-count continuation. The parser selects the exact
generated function once per block and calls that pointer directly, like C's
`searchMax`; the former per-search dispatcher no longer exists in the release
binary. Every main, lazy-depth, and immediate repeat probe now follows C's
direct compare/count continuation rather than calling the shared
external-dictionary-aware `Option` routine; that standalone symbol is also
absent. This is materially different
from the rejected initialized safe candidate buffer, which cleared 256 stack
bytes per search. The generated functions are about `0x56c`-`0x5de` bytes,
close to C's focused `0x665` function.

Lunx 20-run instruction medians on `corpus_z000033` are now level 1
`567,188,864`, level 3 `987,941,835`, level 5 `2,079,254,880`, level 8
`2,660,557,934`, and level 16 `7,551,897,701`. The retained direct-repeat port
first improved levels 5 and 8 by `7.213%` and `8.354%`; the subsequent shared
Greedy/Lazy SeqStore handoff improves levels 5/8/16 by another
`0.743%`/`0.433%`/`0.230%`, while levels 1/3 move only
`+0.034%`/`+0.038%`. Exact preallocated entropy-sequence prefix filling then
improves levels 1/3/5/8/16 by another `0.475%`/`0.505%`/`0.241%`/`0.175%`/
`0.080%`. The subsequent complete isolated no-dictionary row-parser
transaction improves levels 5 and 8 by another `14.271%` and `14.414%`, while
levels 1, 3, and 16 remain neutral. The complete isolated DFast block
transaction then improves targeted level 3 by `1.789%`; the other guard levels
are neutral-to-better. The subsequent complete isolated Fast transaction
improves targeted level 1 by another `3.141%`; levels 3/5/8 improve by
`0.187%`/`0.096%`/`0.039%`, while level 16's `+0.030%` is noise. The following
direct PreparedSequence entropy path improves all five levels by
another `1.203%`/`1.022%`/`0.613%`/`0.482%`/`0.219%`. The subsequent explicit
fixed-width Huffman batch unroll improves them by another
`4.985%`/`2.068%`/`0.965%`/`0.770%`/`0.203%`. Relative to the pre-row-table
Huffman cursor checkpoint, levels 5 and 8 are now about `43.6%` and `42.4%`
lower in
instructions. Against the latest paired C samples, the focused gaps are about
`-5.0%` at level 5 and `-7.6%` at level 8: Rust now uses fewer instructions
at both levels. Focused bytes remain
`11,430,500`, `9,977,820`,
`9,767,260`, `9,686,720`, and `9,211,100`. Current counter artifacts end in
`c-dynamic-bmi2-entropy-{1,2,3}.stat`; validated broad artifacts end
in `c-dynamic-bmi2-entropy.csv`. All 1,095 rows decode and remain byte-identical
to the Fast-codegen checkpoint. Resume from this retained direct-entropy
plus explicit-Huffman-unroll and Fast/DFast sequence/table-transaction baseline.
Using the latest paired C totals, the remaining level-1 and level-3 instruction
gaps are approximately `+13.29%` and `+17.16%`,
but both Rust matchers are
already cheaper than their C counterparts. Continue in table selection and
entropy emission rather than returning to Fast/DFast matching.
Do not retry an initialized row candidate buffer, blanket alignment, pointer-
only FSE state caching, or wholesale Huffman construction without new causal
evidence.

Latest kept matcher-to-entropy state transaction from August 28, 2026: all
nine native stored-sequence strategies now carry the matcher's final repeat
offsets directly into deferred entropy state instead of replaying every
`{litLength, matchLength, offBase}` record during table selection to recompute
the same history. This matches C's ownership boundary: match collection owns
`ZSTD_updateRep()`, while entropy construction consumes codes and commits the
already-produced state only if the compressed block is accepted. Const-
specialized Fast and compact DFast selectors compile the history update out of
their count walks; Greedy/Lazy/optimal skip their separate replay pass. Raw
and RLE rejection discards the provisional matcher history along with pending
FSE state, and the old replay transaction remains the exact same-binary
control under `RUZSTD_TUNE_C_MATCHER_OFFSET_HANDOFF=0`.

Eager and deferred oracles cover all nine C strategies and compare output,
final history, table decisions, and raw/RLE rejection against replay. All
1,095 candidate/control rows decode and match in their first five CSV columns:
511 normal, 292 target-2048, and 292 prepared-CDict. Validation passed 730
library tests with 5 ignored, every workspace/codegen/tool target, strict
all-target Clippy, formatting, release builds, and `git diff --check`.
Lunx's unattended three-sample 20-run candidate/control instruction medians
at levels 1/3/5/8/16 are `549,077,573`/`556,712,965`,
`951,451,925`/`965,767,611`, `2,005,049,592`/`2,019,122,704`,
`2,747,793,891`/`2,760,470,050`, and
`7,840,892,862`/`7,840,892,936`: improvements of `1.3715%`, `1.4823%`,
`0.6970%`, `0.4592%`, and effectively zero. Branches improve by
`0.954%`, `0.813%`, `1.128%`, `0.618%`, and effectively zero; bytes are
exact and Lunx recommends KEEP. Counter artifacts are
`benchmarks/tmp/perf-z000033-matcher-offset-handoff-ab-{candidate,control}-l{1,3,5,8,16}-{1,2,3}.stat`;
broad artifacts begin with `matcher-offset-handoff-`.

Latest kept DFast C-width sequence-count transaction from August 28, 2026:
the compact LL/ML/OF histogram now remains C's `unsigned`/Rust `u32` from the
single `ZSTD_seqToCodes()`-style collection walk through table-log selection,
last-symbol removal, and both the fast and slow `FSE_normalizeCount()` paths.
The former DFast-only machine-word histogram and normalization path was removed
after acceptance. This halves the compact count workspace and removes the
width conversion at the `ZSTD_buildCTable()` boundary; Fast and all higher
strategies are unchanged.

An exhaustive normalization oracle covers alphabet maxima 0, 1, 5, 15, 31,
35, 52, and 63, 97 generated distributions apiece, table logs 5/6/8/9, and
both `-1` and `1` low-probability policies. A complete transaction oracle also
compares RLE, predefined, and encoded LL/OF/ML mode bytes and table
descriptions plus final repeat history with the general machine-word
reference. Before benchmarking, 737 library tests (732 passed, 5 ignored), all
workspace/codegen/tool targets, strict workspace Clippy, formatting, release
builds, `git diff --check`, and all 511 normal, 292 target-2048, and 292
prepared-CDict candidate/control rows passed exactly and decoded.

Lunx's unattended same-binary candidate/control instruction medians at levels
1/3/5/8/16 were `549,079,461`/`549,079,053`,
`952,320,973`/`952,398,373`, `2,005,005,008`/`2,005,005,745`,
`2,747,795,258`/`2,747,794,121`, and
`7,840,893,729`/`7,841,519,196`: effectively neutral at every guard and a
`-0.0081%` instruction improvement at the level-3 target. Level-3 branches
also fell `0.075%`; cycles and misses were noisy. Lunx recommended KEEP. The
temporary `RUZSTD_TUNE_C_DFAST_U32_SEQUENCE_COUNTS` control and old DFast path
were then removed; post-removal full validation passed again and retained
broad outputs remain identical to the candidate. Counter artifacts are
`benchmarks/tmp/perf-z000033-c-dfast-u32-counts-ab-{candidate,control}-l{1,3,5,8,16}-{1,2,3}.stat`;
broad artifacts begin with `c-dfast-u32-counts-`. Treat this primarily as a
completed faithful representation boundary with a small measured CPU win, not
as a material closure of the remaining level-3 gap.

Latest rejected Fast/DFast sequence-code handoff from August 28, 2026: the
existing forward sequence-statistics walk temporarily cached C's LL and ML
codes in the unused high bits of the matcher-owned 12-byte `StoredSequence`.
The reverse BMI2 entropy walk then consumed those codes through C's LL/ML
bit-count tables. This was the deliberately isolated alternative to both the
rejected allocated code sidecar and the rejected larger shared record: it
added no allocation, no traversal, and no record-size change. The same-binary
control was `RUZSTD_TUNE_C_FAST_CACHED_SEQUENCE_CODES=0`.

Exhaustive oracles covered every legal block-sized LL/ML value and all-table,
all-RLE, mixed, two-block history, fresh-table, repeat-table, and fallback
behavior. Full workspace tests, strict workspace Clippy, formatting, release
builds, `git diff --check`, and all 511 normal, 292 target-2048, and 292
prepared-CDict rows passed byte-identically. Lunx's unattended candidate/
control instruction medians at levels 1/3/5/8/16 were
`564,591,339`/`556,837,879`, `978,107,169`/`964,867,598`,
`2,015,067,161`/`2,015,069,056`, `2,756,980,136`/`2,756,979,135`, and
`7,840,892,680`/`7,841,519,764`: regressions of `+1.3924%` and `+1.3722%`
at the intended low levels, with the three guards neutral. Level-1/3 branches
fell by about `1.61%`/`1.45%`, but cycles still rose about `0.49%`/`0.45%`;
the cached marker extraction, length masking, and lookup work cost more than
reclassification. Lunx recommended REVERT, and the complete candidate,
control, mutable plumbing, helper refactors, and candidate-only tests were
removed. Counter artifacts are
`benchmarks/tmp/perf-z000033-cached-sequence-codes-ab-{candidate,control}-l{1,3,5,8,16}-{1,2,3}.stat`;
broad artifacts begin with `cached-sequence-codes-`. Do not retry in-record
code caching unless the matcher writes final code-ready values directly and a
larger consumer can avoid both marker decoding and retained length handling.

Latest rejected complete sequence-orchestration transaction from August 28,
2026: Fast/DFast temporarily moved C's complete
`ZSTD_buildSequencesStatistics()`-through-`ZSTD_encodeSequences()` ownership
boundary behind one strategy-gated function. It selected LL/OF/ML modes,
serialized the mode byte and table descriptions, emitted the reverse sequence
bitstream, applied the legacy-decoder fallback, and converted provisional
tables into deferred frame-state updates before returning. The former split
block orchestration remained in the same binary under
`RUZSTD_TUNE_C_STORED_SEQUENCE_TRANSACTION=0`.

An exact transaction/reference oracle covered Fast and DFast, matcher-produced
and replayed repeat histories, predefined/RLE/encoded mixtures, complete
bytes, resulting FSE tables, offset state, and fallback decisions. The full
workspace/static/release gates passed, and all 511 normal, 292 target-2048,
and 292 prepared-CDict candidate/control rows decoded and matched in their
first five CSV columns. Lunx's candidate/control instruction medians at levels
1/3/5/8/16 were `549,087,386`/`549,083,111`,
`951,513,231`/`951,458,873`, `2,005,008,430`/`2,005,054,582`,
`2,747,840,859`/`2,747,797,413`, and
`7,841,518,709`/`7,841,519,032`: `+0.0008%`, `+0.0057%`, `-0.0023%`,
`+0.0016%`, and effectively zero. Cycle and miss movements were noisy, while
instructions and branches proved the candidate neutral. Lunx recommended
REVERT, so the transaction module, switch, and candidate-only oracle were
removed. Post-revert validation passed 730 library tests with 5 ignored, all
workspace/codegen/tool targets, strict workspace Clippy, formatting, release
builds, `git diff --check`, and focused byte checks. Counter artifacts end in
`stored-sequence-transaction-ab-{candidate,control}`; broad artifacts begin
with `stored-sequence-transaction-`. Do not retry an outer orchestration-only
wrapper: it changes ownership structure but retains the selector return ABI
and both record traversals, so it removes no generated work. A future sequence
tranche must eliminate or replace an inner producer-to-consumer boundary.

Latest rejected C high-bit LL/ML conversion from August 28, 2026: the retained
piecewise large-length code conversion was replaced by C's exact `highbit`
and base-subtraction formulas. Exhaustive legal-domain tests, the complete
workspace/static/release gates, and all 511 normal, 292 target-2048, and 292
prepared-CDict candidate/control rows passed exactly. The candidate also
shrunk the generated BMI2 sequence emitters from roughly `0xc46`/`0xc6a` to
`0x7b4`/`0x7b8` bytes.

Lunx's preserved-binary candidate/control instruction medians at levels
1/3/5/8/16 were `550,140,436`/`549,075,424`,
`953,314,756`/`951,441,134`, `2,001,577,576`/`2,004,634,190`,
`2,744,692,098`/`2,747,826,672`, and
`7,754,986,391`/`7,840,890,331`: `+0.194%`, `+0.197%`, `-0.152%`,
`-0.115%`, and `-1.097%`. Branches fell at every level, but instructions
regressed at levels 1 and 3, the only remaining slower-than-C targets. Lunx
recommended REVERT and the formulas plus expanded candidate-only tests were
removed. Post-revert validation passed 730 library tests with 5 ignored, all
workspace/codegen/tool targets, strict workspace Clippy, formatting, release
builds, `git diff --check`, and focused output checks. Counter artifacts are
`benchmarks/tmp/perf-z000033-c-highbit-length-codes-ab-{candidate,control}-l{1,3,5,8,16}-{1,2,3}.stat`;
broad artifacts begin with `c-highbit-length-codes-`. Retain the piecewise
conversion for Fast/DFast; fewer branches and smaller symbols are not wins
when they increase total low-level instructions.

Latest kept nested entropy-workspace lifetime from August 28, 2026: normal
Fast and DFast Huffman table construction now borrows the frame-owned
`FSETableBuildScratch` while serializing its weight description. The temporary
weight FSE table is recovered from `FSEEncoder` after emission and returned to
the same three-table pool before LL/OF/ML construction begins. This extends
the retained current/next Huffman ownership transaction through C's sequential
`HUF_compressWeights()`/`FSE_buildCTable_wksp()` lifetime instead of allocating
a separate cumulative workspace, symbol-spread workspace, state table, and
symbol-transform table inside every new Huffman description. No additional
frame scratch object or persistent table is introduced. Target-size,
Greedy/Lazy, optimal, prepared-CDict, and non-scratch callers retain their
existing behavior. The exact same-binary allocating control is
`RUZSTD_TUNE_C_REUSE_HUFFMAN_WEIGHT_FSE_SCRATCH=0`.

Exact oracles cover normal and FSE-unusable weight descriptions, stable reuse
of both temporary table allocations, the complete generated Huffman table,
and the literal transaction. All 1,095 candidate/control rows decode and match
in their first five CSV columns: 511 normal, 292 target-2048, and 292
prepared-CDict. Validation passed 728 library tests with 5 ignored, every
workspace/codegen/tool target, strict all-target Clippy, formatting, release
builds, and `git diff --check`. Lunx's unattended three-sample 20-run
candidate/control instruction medians at levels 1/3/5/8/16 are
`556,703,513`/`556,807,992`, `965,705,303`/`966,922,450`,
`2,024,934,721`/`2,024,953,200`, `2,765,732,915`/`2,765,707,852`, and
`7,841,518,959`/`7,841,498,235`: changes of `-0.0188%`, `-0.1259%`,
`-0.0009%`, `+0.0009%`, and `+0.0003%`. Level 3 cycles also improve
`0.4628%`; the larger level-8/16 cycle movements have no corresponding
instruction or branch regression and are noise. Bytes are exact and Lunx
recommends KEEP. Counter artifacts are
`benchmarks/tmp/perf-z000033-huffman-weight-fse-scratch-ab-{candidate,control}-l{1,3,5,8,16}-{1,2,3}.stat`;
broad artifacts begin with `huffman-weight-fse-scratch-`.

Latest kept C-context Huffman output-table lifecycle from August 28, 2026:
normal Fast and DFast frames now alternate current and next owning Huffman
tables instead of allocating fresh code and description vectors for every new
literal table. `HuffmanBuildScratch` retains one released `HuffmanTable`; the
generated builder clears and refills its code vector, while a new in-place
parent serializer refills the table-description vector through the existing
exact raw/FSE selection. This complements the already-retained reusable tree
workspace and mirrors C's `prevEntropy`/`nextEntropy` table lifetime without a
fixed 2 KiB table in Rust return values.

All three release paths are covered: accepting a new table recycles the
superseded current table, raw/RLE block rejection recycles a provisional new
table, and choosing repeat literals recycles the freshly built but unselected
table. The pool is limited to Fast/DFast's normal scratch path; target,
Greedy/Lazy, optimal, and non-scratch callers retain their existing behavior.
`RUZSTD_TUNE_C_RECYCLE_FAST_HUFFMAN_TABLES=0` selects the allocating builder
and drop behavior as the same-binary control.

Allocation oracles prove exact codes, maximum bits, description bytes, and
stable addresses for both owning vectors. Separate lifecycle tests cover
accepted replacement, rejected block state, and unselected repeat tables; an
in-place serializer oracle covers small, raw, and FSE descriptions. Lunx's
three-sample 20-run candidate/control instruction medians at levels
1/3/5/8/16 are `556,786,463`/`557,223,747`,
`966,780,303`/`969,754,487`, `2,024,824,672`/`2,023,896,012`,
`2,765,603,524`/`2,764,705,446`, and
`7,840,148,575`/`7,835,273,783`: `-0.0785%`, `-0.3067%`,
`+0.0459%`, `+0.0325%`, and `+0.0622%`. Branches and misses improve at both
intended low levels; every guard remains below `0.063%`. Longitudinally versus
the prior retained FSE-table-lifecycle checkpoint, levels 1/3 move about
`-0.0583%`/`-0.2186%`, while levels 5/8/16 move only
`+0.0881%`/`+0.0643%`/`+0.1109%`. Bytes are exact and Lunx recommends KEEP.
Validation passed 725 library tests with 5 ignored, every workspace/codegen/
tool target, strict workspace Clippy, formatting, release builds, `git diff
--check`, and exact candidate/control comparison across 511 normal, 292
target-2048, and 292 prepared-CDict rows. Counter artifacts are
`benchmarks/tmp/perf-z000033-huffman-table-recycling-ab-{candidate,control}-l{1,3,5,8,16}-{1,2,3}.stat`;
broad artifacts begin with `huffman-table-recycling-`.

Latest rejected literal/Huffman ownership transaction from August 28, 2026:
the existing `ruzstd-huff0-codegen` unit temporarily owned C's complete
combined literal histogram through the fast table-log choice, canonical table
construction, weight conversion, and serialized table description. Reusable
context scratch held both the 256-bin counts and C's four 256-bin histogram
lanes, while the parent borrowed only the populated count prefix for repeat
table costing. The retained parent histogram plus generated-from-counts table
builder was the same-binary control under
`RUZSTD_TUNE_C_GENERATED_LITERAL_TABLE_TRANSACTION=0`.

Exact oracles covered the simple and four-lane histogram paths around the
1,500-byte threshold, RLE and incompressible exits, sparse, dense, and skewed
alphabets, maximum-symbol metadata, C table-log selection, canonical codes,
description bytes, scratch reuse, and full literal output. The candidate
passed 721 library tests with 5 ignored, every workspace/codegen/tool target,
strict workspace Clippy, formatting, release tool builds, `git diff --check`,
and all 511 normal, 292 target-2048, and 292 prepared-CDict candidate/control
rows byte-identically.

Lunx's unattended three-sample 20-run candidate/control instruction medians at
levels 1/3/5/8/16 were `562,309,180`/`557,131,220`,
`970,986,143`/`969,035,796`, `2,023,140,921`/`2,023,140,261`,
`2,763,982,083`/`2,763,983,031`, and
`7,831,310,928`/`7,831,312,369`: regressions of `+0.9294%` and
`+0.2013%` at the two intended low levels with the three guards neutral.
Branches rose by about `2.785M` at level 1 and `2.312M` at level 3. Lunx
recommended REVERT, and the implementation, oracle, and control switch were
removed. Counter artifacts are
`benchmarks/tmp/perf-z000033-generated-literal-table-transaction-ab-{candidate,control}-l{1,3,5,8,16}-{1,2,3}.stat`;
broad artifacts begin with `generated-literal-table-`. Do not retry a
scratch-owned dynamic histogram crossing counts back to the parent. A future
literal transaction must own costing/selection/emission too, or preserve the
retained fixed histogram's generated loop shape.

Latest kept C-context FSE output-table lifecycle from August 28, 2026: normal
Fast and DFast frames now reuse the owning state-table and symbol-transform
vectors of superseded sequence FSE tables. This completes the other half of
C's alternating `prevEntropy`/`nextEntropy` workspace lifetime: the retained
`FSETableBuildScratch` already reused temporary cumulative/spread workspaces,
and now keeps up to three released output tables for subsequent LL/ML/OF
builds. The builder selects the smallest recycled table with sufficient state
and alphabet capacity, clears and fills both vectors in place, and otherwise
falls back to the original initialized allocation path. No table layout,
normalization, NCount bytes, or encoding transform changed.

Recycling occurs at the retained deferred block-state boundary. A compressed
block commits its provisional tables and returns uniquely owned superseded
tables to the frame pool; raw/RLE rejection returns provisional replacement
tables without mutating current entropy state. `Rc::try_unwrap()` makes
dictionary/shared tables fall back safely rather than assuming unique
ownership. Target-size, Greedy/Lazy, optimal, prepared, and estimator paths do
not use the pool. The same-binary control is
`RUZSTD_TUNE_C_RECYCLE_FAST_FSE_TABLES=0`.

Allocation and transaction oracles prove exact table fields, preservation of
both owning allocation addresses across a rebuild, recycling of all three
superseded tables after commit, and recycling of all three provisional tables
after rejection without changing offsets. Lunx's three-sample 20-run
candidate/control instruction medians at levels 1/3/5/8/16 are
`557,111,340`/`557,312,198`, `968,898,134`/`970,183,740`,
`2,023,043,129`/`2,022,839,697`, `2,763,825,223`/`2,763,760,684`, and
`7,831,467,509`/`7,831,467,791`: changes of `-0.0360%`, `-0.1325%`,
`+0.0101%`, `+0.0023%`, and effectively zero. All longitudinal movements from
the deferred-commit checkpoint stay below `0.11%`. Bytes are exact and Lunx
recommends KEEP. Validation passed 720 library tests (5 ignored), every
workspace/codegen/doc suite, strict workspace Clippy, formatting, release
builds, `git diff --check`, and exact candidate/control comparison across 511
normal, 292 target-2048, and 292 prepared-CDict rows. Counter artifacts are
`benchmarks/tmp/perf-z000033-fse-table-recycling-ab-{candidate,control}-l{1,3,5,8,16}-{1,2,3}.stat`;
broad artifacts begin with `fse-table-recycling-`. The retained attribution
profile is
`benchmarks/tmp/profile-c-port-z000033-l3-after-deferred-commit.perf.data`.

Latest kept C-style stored entropy transaction from August 28, 2026: every
normal native `StoredSequence` path now builds its next LL/ML/OF repeat tables
and offset history provisionally, then commits them only after the compressed
block wins the raw/RLE acceptance decision. This matches C's deferred
`ZSTD_buildSeqStoreStats()`/block-state confirmation boundary and removes the
need to clone three reference-counted FSE table handles and restore four pieces
of frame state after every rejected candidate. Raw and RLE blocks simply drop
the pending transaction. Target-size, split-block, and prepared-sequence paths
retain their existing rollback behavior. The same-binary eager control is
`RUZSTD_TUNE_C_DEFER_STORED_ENTROPY_COMMIT=0`.

The first correct implementation put pending state inside the shared
`CompressedBlockResult`; despite neutral same-binary results, its larger return
ABI shifted the longitudinal level-5/8 medians by `+0.206%`/`+0.139%`. The
retained implementation restores the compact shared result and keeps the
pending transaction in a separate deferred-only, non-inlined emission frame;
the eager control has its own frame with the former snapshot/restore shape. A
two-block oracle covers all nine C strategies and proves that deferred table
pointers and offsets remain unchanged before commit and exactly match the eager
and prepared paths after commit.

Lunx's unattended three-sample 20-run candidate/control instruction medians at
levels 1/3/5/8/16 are `557,000,243`/`557,002,105`,
`968,516,014`/`968,509,421`, `2,020,995,238`/`2,020,996,998`,
`2,762,128,560`/`2,762,151,208`, and
`7,825,918,544`/`7,826,378,091`: changes of `-0.0003%`, `+0.0007%`,
`-0.0001%`, `-0.0008%`, and `-0.0059%`. Relative to the immediately retained
FSE-scratch medians, the isolated level-5/8 movement is only
`+0.0501%`/`+0.0345%`, restoring the expected layout band. Bytes are exact and
Lunx recommends KEEP. Validation passed 718 library tests (5 ignored), every
workspace/codegen/doc suite, strict workspace Clippy, formatting, release
builds, `git diff --check`, and exact candidate/control comparison across 511
normal, 292 target-2048, and 292 prepared-CDict rows. Counter artifacts are
`benchmarks/tmp/perf-z000033-deferred-entropy-commit-isolated-ab-{candidate,control}-l{1,3,5,8,16}-{1,2,3}.stat`;
broad artifacts begin with `deferred-entropy-commit-isolated-`.

Latest kept C-context FSE construction workspace from August 28, 2026: normal
Fast and DFast frames now retain one `FSETableBuildScratch` across blocks. Its
caller-owned cumulative-range and normalized-symbol-spread vectors are reused
sequentially for LL, OF, and ML, matching `FSE_buildCTable_wksp()`'s temporary
workspace lifetime without changing the returned compact `FSETable`, its
repeat-history ownership, normalization probabilities, or NCount emission.
Target-size, Greedy/Lazy, and optimal paths remain unchanged; raw and RLE
fallbacks preserve the frame workspace. The same-binary control is
`RUZSTD_TUNE_C_REUSE_FAST_FSE_BUILD_SCRATCH=0`. This is materially different
from the rejected fixed-short normalization workspace and single-write
`MaybeUninit` builder: it only elides allocations for the retained initialized
builder's two temporary vectors and leaves shared generated-code shapes intact.

A two-pass fresh/reused oracle covers dense, sparse, zero-containing, and
different-log alphabets, compares every state-table and symbol transform field,
and verifies retained capacity. Lunx's three-sample 20-run candidate/control
instruction medians at levels 1/3/5/8/16 are
`556,984,110`/`557,193,901`, `968,426,353`/`970,252,196`,
`2,019,982,933`/`2,019,967,112`, `2,761,176,403`/`2,761,176,530`, and
`7,825,922,719`/`7,826,382,998`: changes of `-0.0377%`, `-0.1882%`,
`+0.0008%`, `-0.000005%`, and `-0.0059%`. Levels 1 and 3 improve, all guards
are instruction-neutral, bytes are exact, and Lunx recommends KEEP.
Validation passed 718 library tests (5 ignored), every workspace/codegen/doc
suite, strict workspace Clippy, formatting, release builds, `git diff
--check`, and exact candidate/control comparison across 511 normal, 292
target-2048, and 292 prepared-CDict rows. Counter artifacts are
`benchmarks/tmp/perf-z000033-fast-fse-build-scratch-ab-{candidate,control}-l{1,3,5,8,16}-{1,2,3}.stat`;
broad artifacts begin with `fast-fse-build-scratch-`.

Latest kept C-context ownership change from August 28, 2026: normal Fast and
DFast frames now retain one `HuffmanBuildScratch` across blocks and thread it
through stored-SeqStore block emission into the generated Huffman
tree/rank/code builder. This ports C's compression-context workspace lifetime
instead of allocating fresh tree workspace for every new literal table.
Target-size, Greedy/Lazy, and optimal paths remain unchanged, while raw/RLE
fallbacks preserve the workspace. The same-binary control is
`RUZSTD_TUNE_C_REUSE_FAST_HUFFMAN_SCRATCH=0`. A two-pass oracle checks exact
fresh/reused bytes and retained generated-node capacity.

Lunx candidate/control 20-run instruction medians at levels 1/3/5/8/16 are
`557,111,143`/`557,439,523`, `968,289,290`/`970,551,646`,
`2,019,403,338`/`2,019,404,496`, `2,760,884,813`/`2,760,885,471`, and
`7,822,204,166`/`7,821,746,940`: changes of `-0.0589%`, `-0.2331%`,
`-0.0001%`, `-0.00002%`, and `+0.0058%`. The intended low levels improve,
and every guard is neutral. Candidate/control output bytes match at all five
levels; Lunx recommends KEEP. Validation passed 717 library tests (5 ignored),
all workspace/codegen suites, strict workspace Clippy, formatting, release
builds, `git diff --check`, and all 511 normal, 292 target-2048, and 292
prepared-CDict rows byte-identically. Counter artifacts end in
`fast-huffman-scratch-ab-{candidate,control}`; broad artifacts begin with
`fast-huffman-scratch-`. This supersedes the old blanket warning against
emitted Huffman scratch reuse: the retained generated-builder shape plus
Fast/DFast-only frame ownership has a measured win, while the former shared
frame/FSE-table layouts remain rejected.

Latest rejected entropy ownership follow-up from August 28, 2026: the complete
specialized Huffman-weight FSE serializer was moved into
`ruzstd-huff0-codegen` only after the retained tree/rank/weight transaction was
already complete. The candidate owned normalization and its slow fallback,
compact CTable construction, safe NCount emission, two-state interleaved FSE
encoding, end-marker handling, and raw/FSE selection. The retained parent
serializer was the same-binary control under
`RUZSTD_TUNE_C_GENERATED_HUFFMAN_WEIGHT_FSE=0`. A sparse-alphabet NCount
zero-run defect found by the first broad decode gate was fixed and added to the
exact oracle before benchmarking. The corrected candidate passed 718 workspace
tests (5 ignored), strict Clippy, formatting, release builds, `git diff
--check`, and all 511 normal, 292 target-2048, and 292 prepared-CDict rows
byte-identically against control.

Lunx candidate/control 20-run instruction medians at levels 1/3/5/8/16 were
`563,334,040`/`557,428,953`, `1,009,884,093`/`970,475,629`,
`2,058,275,252`/`2,019,387,175`, `2,799,173,920`/`2,760,868,680`, and
`8,054,474,210`/`7,822,096,548`, regressions of `+1.0593%`, `+4.0607%`,
`+1.9257%`, `+1.3874%`, and `+2.9708%`. Branches increased at all five
levels. Lunx recommended REVERT; the serializer port, control switch, and
serializer-only tests were removed while the previously retained combined
builder/description callback remains. Artifacts end in
`generated-huffman-weight-fse-ab-{candidate,control}`; broad artifacts begin
with `generated-huffman-weight-fse-`. Do not retry this owning cross-crate
serializer shape without an in-place representation or another material
generated-code change.

Latest kept direct native SeqStore entropy port from August 28, 2026: normal
Fast and DFast blocks now carry the matcher's 12-byte
`{litLength, matchLength, offBase}` records through table selection and final
sequence emission. A literal-only generated SeqStore pass replaces expansion
into 16-byte `PreparedSequence` records. The direct transaction covers normal
no-dictionary and external-dictionary blocks; target-size mode and higher
strategies deliberately retain the prepared representation. DFast recycles
its native sequence allocation after emission, while Fast retains the measured
non-recycling ownership shape. A two-block entropy oracle proves exact prepared
versus stored bytes, FSE state, and offset history for both Fast and DFast.

The release profile now uses one codegen unit and the major generated functions
have named sections. Absolute placement can still move under LLD, so the
retained `RUZSTD_TUNE_C_NATIVE_SEQUENCE_STORE=0` diagnostic override selects
the prepared path from the same binary. Lunx's unattended three-sample,
20-run same-binary A/B medians are candidate/baseline level 1
`557,482,949`/`569,154,719` (`-2.0507%`), level 3
`970,829,668`/`990,327,690` (`-1.9688%`), level 5
`2,041,258,492`/`2,041,281,119` (`-0.0011%`), level 8
`2,781,407,521`/`2,781,407,398` (`+0.000004%`), and level 16
`7,819,628,022`/`7,819,628,043` (effectively zero). Focused byte totals are
exact at `11,430,500`, `9,977,820`, `9,767,260`, `9,686,720`, and
`9,211,100`. Lunx recommends KEEP. Counter artifacts are
`benchmarks/tmp/perf-z000033-stable-direct-stored-ab-{candidate,baseline}-l{1,3,5,8,16}-{1,2,3}.stat`.
Validation passed 718 library tests (713 passed, 5 ignored), all workspace and
codegen targets, 7 documentation tests, strict workspace Clippy, formatting,
release builds, `git diff --check`, and all 511 normal, 292 target-2048, and
292 prepared-CDict rows byte-identically against the same-binary prepared
path. Broad artifacts end in `stable-direct-stored.csv` and
`stable-prepared-ab.csv`. This supersedes the earlier layout-confounded
rejection below. Refresh paired C attribution at levels 1 and 3 from this
retained binary, then continue with another coherent shared Huffman/statistics
boundary; do not return to the already-cheaper matchers.

Rejected August 28 complete generated Huffman table-construction transaction:
the existing `ruzstd-huff0-codegen` unit temporarily owned the full safe
analogue of C's `HUF_buildCTable_wksp()` boundary: bucket sorting, tree merge,
maximum-height redistribution, and canonical `nbPerRank`/`valPerRank` table
construction. Dense, sparse, tied, skewed, constrained-table-log, and fallback
oracles proved exact equivalence with the retained local transaction. The full
workspace suite (719 library tests before removal), documentation tests,
strict Clippy, formatting, release builds, `git diff --check`, and all 511
normal, 292 target-2048, and 292 prepared-CDict candidate/control rows passed
exactly. Lunx's unattended same-binary candidate/control medians at levels
1/3/5/8/16 were `557,741,935`/`557,496,432`,
`972,706,460`/`970,922,904`, `2,044,255,151`/`2,042,385,596`,
`2,784,588,068`/`2,782,423,530`, and
`7,822,507,809`/`7,820,160,270`: regressions of `+0.0440%`, `+0.1837%`,
`+0.0915%`, `+0.0778%`, and `+0.0300%`. Bytes were exact. Lunx recommended
REVERT and the generated builder, wiring, and same-binary switch were removed.
Counter artifacts are
`benchmarks/tmp/perf-z000033-generated-huffman-build-ab-{candidate,control}-l{1,3,5,8,16}-{1,2,3}.stat`;
broad artifacts contain `generated-huffman-{candidate,control}`. Keep the
local compact-node builder. Do not move construction alone across the crate
boundary again; a future attempt needs to combine construction with its table-
description/emission consumer or otherwise remove the returned owning-vector
handoff.

Superseded first August 28 direct native stored-sequence entropy attempt:
normal Fast
and DFast blocks retained the matcher's 12-byte `{litLength, matchLength,
offBase}` records through table selection and final sequence emission, while a
literal-only SeqStore pass replaced construction of the 16-byte
`PreparedSequence` vector. Target mode and higher strategies kept the retained
prepared path. The implementation covered no-dictionary and external-
dictionary blocks, had an exact two-block prepared/stored entropy oracle for
both Fast and DFast, passed 718 library tests (713 passed, 5 ignored), every
workspace/codegen target, strict Clippy, formatting, release builds, and all
1,095 broad normal/target-2048/prepared-CDict rows byte-identically. Lunx
measured 20-run medians of `556,962,702`, `971,055,368`, `2,079,735,297`,
`2,661,157,140`, and `7,571,408,346` instructions at levels 1/3/5/8/16. Those
are strong `-1.8030%` and `-1.7093%` wins at the two remaining slow levels,
neutral `+0.0231%`/`+0.0225%` movement at levels 5/8, but a stable `+0.2584%`
level-16 regression beyond the `0.2%` guard. Lunx recommended REVERT and the
candidate source was fully removed. Artifacts end in
`c-direct-stored-sequence-entropy-{1,2,3}.stat`; rejected broad artifacts end
in `direct-stored-sequences.csv`. This proves replacing the prepared record is
worth about 1.7-1.8% at Fast/DFast. Revisit it only after isolating its added
selection/emission machine code or otherwise proving that the retained
level-16 layout is unchanged; do not repeat the same in-crate duplicated
boundary.

Post-revert rebuild warning from the same checkpoint: although every direct-
stored source marker was removed and bytes/tests returned to the retained
path, fresh local single samples from the rebuilt binary were `570,260,413`
instructions at level 1, `993,541,796` at level 3, and `7,562,092,283` at
level 16 rather than reproducing the historical `567.19M`/`987.94M`/`7.552B`
bands. The focused output totals remained exact. This is further evidence that
the current unresolved variable is whole-binary function placement. Before
judging another algorithm, first establish stable generated-section placement
or refresh a three-sample retained rebuild baseline; do not silently compare a
new binary against the unreproduced historical layout.

Rejected August 28 persistent Fast/DFast SeqStore code sidecar: a separate
reusable LL/ML/OF code vector was filled once per block and consumed by both
table selection and reverse sequence emission. The full implementation passed
all correctness gates and kept all 1,095 broad rows byte-identical, but Lunx's
three-sample medians were level 1 `602,736,817`, level 3 `1,033,824,923`,
level 5 `2,083,818,368`, level 8 `2,665,496,977`, and level 16
`7,551,920,134` instructions. Versus the retained DFast-compact baseline these
are `+2.0631%`, `+1.6863%`, `-0.1285%`, `-0.0926%`, and `-0.0715%`.
The sidecar was removed: its extra fill/traversal and memory traffic outweigh
avoided code conversion at the low levels that remain behind C. Do not retry
an additive code-array sidecar unless it replaces an existing representation
rather than supplementing it. Artifacts end in
`c-fast-persistent-sequence-codes-{1,2,3}.stat`.

Rejected August 28 isolated direct-output sequence transaction: a dedicated
no-std primitive-ABI crate ported the complete 64-bit all-table
`ZSTD_encodeSequences_body()` boundary, including reverse LL/ML/OF conversion,
compact FSE transitions, extra bits, final states, and the end marker. It used
a single pre-sized overlapping-word output cursor instead of the shared
`BitWriter`. Independent cursor/container tests and an end-to-end oracle proved
exact bytes; 717 library tests, all workspace/codegen/doc tests, strict
workspace Clippy, formatting, release builds, and broad normal/target/CDict
comparisons passed. The only broad rows excluded from before/after comparison
were the two Cargo manifest fixtures changed by temporarily adding the crate;
the other 497 normal, 284 target-2048, and 284 prepared-CDict rows were exact.
Lunx measured level-1/3/5/8/16 medians of `592,630,216`, `1,021,427,448`,
`2,085,005,722`, `2,666,630,126`, and `7,556,124,985` instructions. Versus
the retained DFast-compact baseline these are `+0.3517%`, `+0.4669%`,
`-0.0716%`, `-0.0502%`, and `-0.0158%`. Recommendation: REVERT; the complete
candidate was removed. The direct cursor's reservation and ABI boundary cost
more than the retained in-crate writer at the affected levels. Do not retry a
separate direct-output sequence crate without a different profile-backed data
flow that removes more than the writer itself. Artifacts end in
`c-fast-direct-sequence-output-{1,2,3}.stat`; rejected broad artifacts end in
`c-fast-direct-sequence-output.csv`.

Rejected August 28 DFast matcher-to-prepared-SeqStore fusion: the isolated
no-dictionary DFast matcher was generalized over stored and prepared sinks so
normal and target DFast blocks could copy literals, resolve numeric `offBase`
history, and write prepared records directly while finding matches. This
removed both the intermediate `StoredSequence` vector and the subsequent
replay/preparation pass, while leaving Fast, Greedy/Lazy, optimal, and
external-dictionary DFast unchanged. The complete candidate passed 716
library tests, workspace/codegen/integration tests, strict all-target Clippy,
formatting, release builds, and all 511 normal, 292 target-2048, and 292
prepared-CDict rows byte-for-byte. Lunx's unattended medians were level
1 `590,495,708`, level 3 `1,016,884,681`, level 5 `2,086,155,352`, level 8
`2,667,671,783`, and level 16 `7,560,202,653` instructions. Relative to the
retained DFast-compact baseline these are `-0.0098%`, `+0.0200%`, `-0.0165%`,
`-0.0111%`, and `+0.0381%`: entirely neutral, with no intended level-3 gain.
Recommendation: REVERT; the candidate was removed and the intermediate store
plus replay restored. This shows that eliminating the Rust handoff in this
shape does not reduce generated work; do not retry a generic sink boundary
without a materially different owning representation or profile signal.
Artifacts end in `dfast-fused-matcher-seqstore-{1,2,3}.stat`; rejected broad
artifacts end in `dfast-fused-matcher-seqstore.csv`.

Rejected August 28 DFast C-width normalization transaction: fresh post-BMI2
attribution showed the compact DFast table-selection function reserving a
`0x8f8`-byte frame, including a 2048-byte three-lane `usize` count workspace.
The complete experiment changed the lanes to C's `u32`, carried them through
separate fast, slow, and fallback normalizers without a widening conversion
pass, and fed the resulting `i32` probabilities directly into compact FSE
table construction. Generated-distribution tests proved identical normalized
probabilities, tables, start states, and every valid symbol/state transition.
The release frame fell to `0x4f8` and its memset from `0x800` to `0x400` bytes.
All 719 library tests, workspace/codegen/integration tests, strict Clippy,
release builds, and all 1,095 broad rows passed exactly.

Lunx nevertheless measured level-1/3/5/8/16 medians of `568,362,555`,
`988,325,721`, `2,073,887,535`, `2,656,619,576`, and `7,558,736,002`
instructions. Relative to the retained BMI2 baseline these are `+0.2069%`,
`+0.0389%`, `-0.2581%`, `-0.1480%`, and `+0.0906%`. Recommendation: REVERT;
the required level-3 gain did not appear and level 1 crossed the 0.2% guard.
The complete u32 path was removed and the retained compact `usize` lanes were
restored. Smaller stack/memset size is not an acceptance proxy for this path.
Artifacts end in `c-dfast-u32-normalization-{1,2,3}.stat`; rejected broad
artifacts end in `c-dfast-u32-normalization.csv`.

Rejected August 28 shared FSE single-write construction transaction: the
complete normalized-symbol spread and compact state-table construction used
`MaybeUninit` workspaces so every production-log slot was written once instead
of zero-filled and overwritten. The general encoder's logs below 5 retained
the initialized builder because their masked spread step is not a full
permutation; independent equivalence tests covered logs 5 through 12 and the
low-log/RLE fallback. All 719 library tests, workspace/codegen suites, strict
Clippy, formatting, release builds, focused bytes, and all 1,095 broad rows
passed exactly. Lunx measured level-1/3/5/8/16 medians of `569,369,696`,
`990,905,808`, `2,082,211,386`, `2,663,397,028`, and `7,568,714,122`
instructions. Those are regressions of `+0.3845%`, `+0.3000%`, `+0.1422%`,
`+0.1067%`, and `+0.2227%` versus the retained BMI2 baseline. Recommendation:
REVERT; every level lost, including both remaining low-level targets and the
level-16 guard. The initialized construction was restored. Do not retry raw
uninitialized FSE construction workspaces without a different ownership or
allocation-elision boundary; removing zero-fill alone worsens generated code.
Artifacts end in `c-fse-single-write-construction-{1,2,3}.stat`; rejected
broad artifacts end in `c-fse-single-write-construction.csv`.

Rejected August 28 Fast/DFast C selection-validity transaction: both the
general Fast and compact DFast `ZSTD_selectEncodingType()` paths temporarily
derived predefined-table support from the maximum code and trusted
`FSE_repeat_valid` as C's already-proven full-alphabet invariant, removing up
to six populated-alphabet validation scans per block. Debug builds retained
the old scans as equivalence assertions. All 717 library tests, every
workspace/codegen suite, strict Clippy, formatting, release builds, focused
bytes, and all 1,095 broad rows passed exactly. Lunx nevertheless measured
level-1/3/5/8/16 medians of `569,976,634`, `991,748,654`, `2,082,731,768`,
`2,665,049,573`, and `7,571,856,649` instructions, regressions of `+0.4915%`,
`+0.3853%`, `+0.1672%`, `+0.1688%`, and `+0.2643%` versus the retained BMI2
baseline. Recommendation: REVERT; the old scans were restored. The logical
work reduction perturbs inlined policy/generated-code shape enough to lose at
every level, so do not retry this local selection edit without isolating a
larger ownership/function boundary. Artifacts end in
`c-fast-selection-validity-{1,2,3}.stat`; rejected broad artifacts end in
`c-fast-selection-validity.csv`.

Rejected August 28 isolated Fast/DFast FSE-statistics transaction: a new
`no_std` code-generation crate temporarily owned the complete prepared
sequence-to-table-selection boundary: code derivation, Fast wide or DFast
compact counting, offset-history advancement, C heuristic selection, optimal
table log, and normalization. A primitive-array ABI returned the three modes
and normalized probabilities, leaving table construction in `ruzstd`.
Reference tests covered small, 80-, 512-, and 2,050-sequence inputs, all table
modes, and both strategies. All 718 library tests, workspace/codegen suites,
strict Clippy, formatting, release builds, and all unchanged rows in the 1,095
row broad matrix passed exactly; only Cargo-manifest fixture sizes changed.

With both strategies isolated, Lunx measured level-1/3/5/8/16 medians of
`569,663,528`, `982,920,767`, `2,084,932,503`, `2,665,082,931`, and
`7,571,882,723` instructions, or `+0.4363%`, `-0.5082%`, `+0.2731%`,
`+0.1700%`, and `+0.2646%` versus the retained BMI2 baseline. A DFast-only
refinement confirmed the level-3 gain with medians of `570,224,688`,
`982,758,015`, `2,084,672,060`, `2,664,924,776`, and `7,570,931,764`, or
`+0.5352%`, `-0.5247%`, `+0.2605%`, `+0.1641%`, and `+0.2520%`.
Recommendation: REVERT; the real DFast level-3 win does not compensate for
cross-level regressions beyond the 0.2% guard, apparently caused by the added
linked code-generation unit and whole-binary layout. The crate and wiring were
removed. Revisit this ownership boundary only if it can live inside an
existing generated unit or linker-layout effects are controlled. Artifacts
end in `c-isolated-fast-fse-statistics-{1,2,3}.stat` and
`c-isolated-dfast-fse-statistics-{1,2,3}.stat`; corresponding rejected broad
artifacts use the same stems with `.csv`.

Rejected August 28 existing-DFast-unit FSE-statistics transaction: the same
complete DFast code/count/history/policy/optimal-log/normalization boundary was
then placed inside the already-retained `ruzstd-dfast-codegen` crate instead
of adding a new linked unit. It used the prepared primitive record ABI and
returned primitive mode tags plus fixed normalized-probability arrays; concrete
FSE table ownership remained in `ruzstd`. Generated/reference equivalence
covered RLE, predefined, repeat, and encoded modes at 1, 2, 80, 512, and 2,050
sequences. Full validation passed 718 library tests (713 passed, 5 ignored),
all workspace/codegen and documentation tests, strict Clippy, formatting,
release builds, and all 511 normal, 292 target-2048, and 292 prepared-CDict
rows byte-for-byte against the retained BMI2 baseline.

Lunx measured level-1/3/5/8/16 medians of `570,224,582`, `980,766,081`,
`2,082,061,987`, `2,662,484,665`, and `7,566,164,786` instructions. Relative
to the retained BMI2 baseline these are `+0.5352%`, `-0.7263%`, `+0.1350%`,
`+0.0724%`, and `+0.1889%`. Recommendation: REVERT; placing the transaction
inside the existing DFast unit strengthens its real level-3 gain and keeps the
other DFast/Greedy/optimal guards inside 0.2%, but level 1 still suffers the
same roughly 0.54% whole-binary displacement as the prior new-unit variant.
The implementation and equivalence test were removed. This paired experiment
isolates code/section placement as the blocker: do not port this transaction
again until the Fast level-1 text/data placement can be held stable or the
candidate can replace an equal-size resident region. Counter artifacts end in
`c-dfast-existing-unit-fse-statistics-{1,2,3}.stat`; rejected broad artifacts
end in `dfast-existing-unit-fse-stats.csv`.

Latest kept dynamic BMI2 entropy port from August 28, 2026: Rust now mirrors
C's runtime `DYNAMIC_BMI2` boundary for the two dominant low-level entropy
emitters. One cached x86-64 CPUID leaf-7 decision dispatches Fast/DFast's
complete all-table sequence transaction and the isolated four-stream Huffman
transaction into separate `#[target_feature(enable = "bmi2")]` generated
functions; non-x86 and non-BMI2 machines retain the byte-identical portable
paths. The Huffman stream transaction is fully inlined into each generated
variant so the feature applies to the hot symbol loop rather than merely its
outer dispatcher. Release disassembly confirms `shrx`, `shlx`, `bzhi`,
`rorx`, and `mulx` in the specialized functions.

Lunx's unattended level-1/3/5/8/16 medians are `567,188,864`, `987,941,835`,
`2,079,254,880`, `2,660,557,934`, and `7,551,897,701` instructions. Relative
to the retained DFast-compact baseline, these improve by `3.9564%`, `2.8268%`,
`0.3472%`, `0.2778%`, and `0.0718%`. Focused bytes remain exact. Validation
passed 717 library tests (712 passed, 5 ignored), all workspace/codegen and
integration tests, strict workspace all-target Clippy, formatting, release
builds, `git diff --check`, and all 511 normal, 292 target-2048, and 292
prepared-CDict rows byte-for-byte. Counter artifacts end in
`c-dynamic-bmi2-entropy-{1,2,3}.stat`; broad artifacts end in
`c-dynamic-bmi2-entropy.csv`. Lunx recommends KEEP; this is the authoritative
low-level baseline.

Latest kept Fast/DFast sequence-emission port from August 28, 2026: the active
all-table prepared-sequence path now has a strategy-gated complete
`ZSTD_encodeSequences_body()` transaction. One isolated generated function
owns LL/ML/OF conversion, the three compact FSE state transitions, extra-bit
emission, final state flushes, and the end marker. Within each reverse sequence
iteration it joins the FSE batch and following extra-bit batch into one
`BitWriter` call whenever their combined width fits 64 bits; the uncommon wider
case splits exactly at the container boundary. This removes a larger unit of
writer state work than the rejected pointer-only FSE cache while preserving
the existing safe writer and 16-byte `PreparedSequence` representation.
Greedy/Lazy and higher strategies retain the prior emitter.

Lunx's unattended three-sample medians improve levels 1 and 3 by `0.2384%`
and `0.2890%` versus the explicit-Huffman-unroll baseline. The strategy-
excluded level 5, 8, and 16 guards move by `+0.1409%`, `+0.1027%`, and
`+0.0314%`; record these small code-layout costs rather than calling them
neutral. The candidate is retained because it improves the only two remaining
slower-than-C strategy levels while levels 5 and 8 remain about 5% and 7.6%
ahead of C. Focused bytes are exact at every level. Validation passed 715
library tests (710 passed, 5 ignored), 7 doc/integration tests, all five
codegen crates, workspace all-target Clippy with warnings denied, formatting,
release builds, `git diff --check`, and all 511 normal, 292 target-2048, and
292 prepared-CDict rows byte-for-byte. Counter artifacts end in
`c-fast-sequence-transaction-{1,2,3}.stat`; broad artifacts end in
`c-fast-sequence-transaction.csv`. Lunx recommends KEEP.

Latest kept Fast/DFast sequence-table preparation port from August 28, 2026:
the direct prepared path now owns C's three-table statistics transaction in
one strategy-isolated function. It converts and counts LL, ML, and OF codes in
one forward walk, then makes all three C Fast mode decisions and constructs
any required tables. This replaces three full prepared-sequence scans without
reintroducing C's separately allocated `llCode`/`mlCode`/`ofCode` arrays,
which had measured worse in Rust. Greedy/Lazy and higher strategies keep their
previous selection transaction.

Lunx's unattended medians are `591,477,364`, `1,023,159,220`,
`2,085,495,652`, `2,666,996,236`, and `7,566,926,455` instructions at levels
1/3/5/8/16. Levels 1 and 3 improve by another `0.6035%` and `0.7113%` over
the retained emission transaction; excluded levels move by only `+0.0039%`,
`+0.0007%`, and `+0.0080%`. Focused bytes remain exact. The same 715 library
tests, 7 doc tests, five codegen suites, strict workspace Clippy, formatting,
release builds, `git diff --check`, and 1,095 broad rows pass. Counter
artifacts end in `c-fast-sequence-table-transaction-{1,2,3}.stat`; broad
artifacts end in `c-fast-sequence-table-transaction.csv`. Lunx recommends
KEEP; this is the retained table-preparation precursor.

Latest kept Fast/DFast offset-history transaction from August 28, 2026: the
strategy-isolated one-pass LL/ML/OF statistics walk now also replays C's
canonical repeat-offset history. This removes the separate forward history
pass for Fast and DFast without trusting matcher-local repeat state or changing
the shared sequence representation. Greedy/Lazy and higher strategies retain
their prior history pass.

Lunx's unattended medians are `591,050,197`, `1,023,070,414`,
`2,086,187,823`, `2,667,689,851`, and `7,566,863,445` instructions at levels
1/3/5/8/16. Relative to the sequence-table baseline, the changes are
`-0.0722%`, `-0.0087%`, `+0.0332%`, `+0.0260%`, and `-0.0008%`: a small but
repeatable level-1 win with the other levels effectively neutral. Focused
bytes remain exact. All validation gates and all 1,095 normal, target-2048,
and prepared-CDict rows pass byte-for-byte. Counter artifacts end in
`c-fast-sequence-history-transaction-{1,2,3}.stat`; broad artifacts end in
`c-fast-sequence-history-transaction.csv`. Lunx recommends KEEP; this is the
retained history precursor.

Latest kept DFast compact sequence-statistics transaction from August 28,
2026: DFast now ports C's bounded sequence count workspace as three
non-overlapping 64-symbol lanes in one safe 256-entry array. Its single
prepared-record walk counts LL/ML/OF codes, advances canonical offset history,
and carries the known totals and maximum symbols directly into table
normalization. Table construction follows C's LL, OF, ML order. The lane index
remains a `u8`, making every workspace access statically in bounds without
unchecked indexing. Fast deliberately retains the prior three 256-entry
`CodeCounts`: applying the compact transaction to both strategies improved
level 3 by `0.6294%` but regressed level 1 by `0.6248%`, so Lunx recommended
REVERT for the shared form.

With DFast-only gating, Lunx's unattended medians are `590,553,364`,
`1,016,680,870`, `2,086,499,689`, `2,667,968,720`, and `7,557,320,806`
instructions at levels 1/3/5/8/16. Relative to the preceding history
transaction, the changes are `-0.0841%`, `-0.6245%`, `+0.0150%`, `+0.0105%`,
and `-0.1261%`. Focused bytes remain exact. Validation passed 716 library tests
(711 passed, 5 ignored), 7 doc tests, all five codegen suites, strict workspace
Clippy, formatting, release builds, `git diff --check`, and all 1,095 normal,
target-2048, and prepared-CDict rows byte-for-byte. Counter artifacts end in
`c-dfast-compact-sequence-statistics-{1,2,3}.stat`; broad artifacts end in
`c-dfast-compact-sequence-statistics.csv`. Lunx recommends KEEP; this is the
current baseline.

Rejected August 28 follow-up after that attribution: a borrowed C
`FSE_CState_t`-style encoder state cached the compact state-table and symbol-
transform bases once for each LL/ML/OF stream while preserving the retained
all-table batching and `BitWriter`. It passed all 708 library tests, 7
integration tests, workspace strict Clippy, formatting, release builds, and all
1,095 broad byte rows. Lunx nevertheless measured level-1/3/5/8/16 20-run
instruction medians of `659,140,831`, `1,093,037,801`, `2,495,232,615`,
`3,174,167,606`, and `7,623,589,137`, respectively `+0.0252%`, `+0.0247%`,
`+0.0103%`, `+0.0072%`, and `+0.0036%` versus the retained direct-repeat
baseline. The candidate was reverted because it produced no hardware-counter
gain. Artifacts end in `cached-fse-encode-state-{1,2,3}.stat`; the validated
broad artifacts end in `cached-fse-encode-state.csv`. Do not retry pointer
caching alone; the next sequence-emission port must remove a larger unit of
work or change the generated loop materially.

Latest kept shared SeqStore port from August 28, 2026: Greedy, Lazy, Lazy2,
BtLazy2, and optimal paths that consume `GreedyBlockOutput` now use the same
isolated primitive-ABI `ruzstd-seqstore-codegen` transaction already retained
for Fast/DFast. The complete duplicate Rust preparation loop was removed: the
leaf transaction owns bounded C-style literal wild copies, numeric repeat-
offset resolution/update, prepared-record writes, tail literals, and reusable
allocation handoff. `PreparedBlock` and primitive word vectors transfer
ownership through compile-time-proven layout casts without copying; a test
proves both allocation addresses and all record fields survive a round trip.
Full validation passed 709 library tests (704 passed, 5 ignored), 7 integration
tests, all three codegen crates, workspace all-target Clippy with warnings
denied, formatting, release builds, `git diff --check`, and all 1,095 broad
rows exactly. Lunx's corrected recommendation is KEEP; its first report
mistakenly described the negative level-16 delta as a regression, then
confirmed it is a `0.230%` improvement. The authoritative medians and artifacts
are recorded in the checkpoint above.

Rejected August 28 writer follow-up from the shared-SeqStore checkpoint:
`BitWriter::write_bits_64_cold()` replaced its per-byte remainder pushes with
one bounded whole-byte `extend_from_slice()`, mirroring C's single
`BIT_flushBitsFast()` transaction without changing the sequence loop or writer
ownership. It passed all strict validation; all 1,080 unchanged-input broad
rows were exact and the 15 `repo_bit_writer.rs` rows changed only because the
fixture source changed. Lunx nevertheless measured level-1/3/5/8/16 medians of
`660,113,503`, `1,094,506,387`, `2,477,879,262`, `3,161,667,048`, and
`7,609,029,350`: regressions of `0.139%`, `0.121%`, `0.058%`, `0.047%`, and
`0.043%` versus the shared-SeqStore baseline. It recommended REVERT and the
writer change was removed. Artifacts end in
`bitwriter-whole-byte-flush-{1,2,3}.stat`; broad artifacts end in
`bitwriter-whole-byte-flush.csv`. Keep the existing byte-push remainder path;
the compiler's small-count specialization is cheaper than a runtime-length
slice append here.

Latest kept entropy-sequence materialization change from August 28, 2026:
`encode_sequences_for_history_into()` now reserves the exact sequence count,
initializes that bounded spare-capacity prefix directly, and commits its length
once after every record has been written. This removes repeated safe `Vec`
length maintenance while preserving the existing 12-byte representation,
history transitions, table selection, and emission loops. A regression test
proves allocation identity and mixed generic/preencoded offset fields. Full
validation passed 710 library tests (705 passed, 5 ignored), 7 integration
tests, all codegen crates, workspace all-target Clippy with warnings denied,
formatting, release builds, `git diff --check`, and all 1,095 broad rows
exactly. Lunx recommended KEEP after three 20-run samples at every focused
level; medians are `656,070,543`, `1,087,655,516`, `2,470,462,300`,
`3,154,652,394`, and `7,599,718,995`, improvements of `0.475%`, `0.505%`,
`0.241%`, `0.175%`, and `0.080%` versus the shared-SeqStore checkpoint.
Artifacts end in `sequence-prefix-fill-{1,2,3}.stat`; broad artifacts end in
`sequence-prefix-fill.csv`.

Latest kept complete row-parser port from August 28, 2026: the normal no-
dictionary Greedy, Lazy, and Lazy2 row-hash parser now lives with the generated
row matcher in `ruzstd-row-codegen`. One primitive slice/scalar transaction
owns the block loop, cached row search, direct repeat probes, lazy decisions,
backward match extension, bounded uninitialized sequence-prefix writes, repeat
history, and matcher-state updates. The main crate reserves the proven maximum
sequence prefix once and commits only the initialized returned length. The 27
`(depth,minMatch,rowLog)` functions are selected once per block, removing the
hot cross-module row-search ABI while leaving hash-chain, binary-tree, ext-
dictionary, and attached-dictionary paths unchanged. A direct oracle test runs
the retained local parser and new transaction from identical state at all
three depths and compares both block output and every matcher-state field.

Full validation passed 711 library tests (706 passed, 5 ignored), 7 integration
tests, all codegen crates, workspace all-target Clippy with warnings denied,
formatting, packaging, release builds, `git diff --check`, and all 1,095 broad
rows exactly. Lunx's three 20-run samples were stable and its recommendation is
KEEP. Level-1/3/5/8/16 medians are `656,068,549`, `1,087,655,341`,
`2,117,901,080`, `2,699,936,389`, and `7,599,915,925`: changes of `-0.0003%`,
`-0.00002%`, `-14.271%`, `-14.414%`, and `+0.0026%` versus the sequence-prefix
baseline. Focused bytes remain exact. Counter artifacts end in
`row-parser-transaction-{1,2,3}.stat`; broad artifacts end in
`row-parser-transaction.csv`.

Latest kept complete DFast block port from August 28, 2026: the entire normal
no-dictionary double-fast transaction now lives in the dedicated no-std
`ruzstd-dfast-codegen` crate. Four `minMatch` specializations own both hash
probes/tables, repeat probing, long-versus-short match choice, backward
extension, complementary insertion, immediate repcodes, bounded uninitialized
sequence-prefix writes, and repeat-history output behind one primitive ABI.
The main crate reserves the proven maximum sequence count once and commits only
the initialized prefix. A direct oracle compares the former local transaction
with the isolated implementation for `minMatch` 4 through 7, including both
hash tables, emitted records, last literals, and repeat offsets.

Full validation passed 712 library tests (707 passed, 5 ignored), 7 integration
tests, all four codegen crates, workspace all-target Clippy with warnings
denied, formatting, packaging, release builds, and `git diff --check`. All
1,095 broad rows decode; the 1,065 unchanged-input rows are byte-identical to
the row-parser checkpoint. Lunx recommended KEEP after stable three-by-20
samples. Level-1/3/5/8/16 medians are `656,033,681`, `1,068,201,844`,
`2,117,751,776`, `2,698,973,209`, and `7,593,707,962`: improvements of
`0.0053%`, `1.7886%`, `0.0071%`, `0.0357%`, and `0.0817%` versus the retained
row-parser baseline. Focused bytes remain exact. Counter artifacts end in
`dfast-block-codegen-{1,2,3}.stat`; broad artifacts end in
`dfast-block-codegen.csv`.

Latest kept complete Fast block port from August 28, 2026: the entire normal
no-dictionary Fast transaction now lives in the dedicated no-std
`ruzstd-fast-codegen` crate. Four `minMatch` specializations own hash probes
and table updates, direct repeat probes, skip progression, backward extension,
C-style fill-after-match, immediate repcodes, bounded uninitialized sequence
writes, and repeat-history output behind one primitive ABI. The main crate
keeps Fast's deliberately fresh per-block sequence allocation, reserves the
proven maximum prefix, and commits only initialized records. A direct oracle
compares the former local implementation with all four generated functions,
including the hash table, output records, literals, and repeat state.

Full validation passed 713 library tests (708 passed, 5 ignored), 7 integration
tests, all five codegen crates, workspace all-target Clippy with warnings
denied, formatting, packaging at the then-current internal-crate checkpoint,
release builds, and `git diff --check`. All 1,095 broad rows decode; all 1,065
unchanged-input rows are byte-identical. Lunx recommended KEEP. Level-
1/3/5/8/16 medians are `635,430,077`, `1,066,202,536`, `2,115,728,625`,
`2,697,913,377`, and `7,595,945,021`: changes of `-3.1406%`, `-0.1872%`,
`-0.0955%`, `-0.0393%`, and `+0.0295%` versus the DFast checkpoint. Focused
bytes remain exact. Counter artifacts end in
`fast-block-codegen-{1,2,3}.stat`; broad artifacts end in
`fast-block-codegen.csv`.

Fresh 80-run profiles after that port put Rust's level-1 Fast transaction at
about `1.111B` sampled instructions versus C Fast at `1.135B`, and Rust's
level-3 DFast transaction at about `1.875B` versus C DFast at `2.038B`.
Remaining whole-compressor gaps are about `+26.9%` and `+26.4%`. The largest
Rust excesses are Huffman stream emission, sequence FSE emission, prepared-
sequence/code conversion, and table selection; literal histogram counting is
already cheaper than C. Profile artifacts are
`profile-c-port-z000033-l{1,3}-after-fast-block-codegen.perf.data` and matching
`profile-c-api` files. Do not return to Fast/DFast matcher work without new
evidence.

Rejected C-style `ZSTD_seqToCodes()` array port from August 28, 2026: normal
compression materialized LL/OF/ML codes once into three contiguous compact
arrays, reused them for C-fast/C-cost table selection, and used C's base/bit
tables to avoid reclassifying lengths in final FSE emission. Unlike the older
expanded and packed records, this preserved the 12-byte sequence value and the
now-isolated Fast/DFast/Greedy parser crates. A 280-sequence oracle proved
history, codes, and every emitted bit; 709 library tests plus 7 integration
tests, all codegen tests, strict workspace Clippy, release builds, formatting,
`git diff --check`, and all 1,095 broad rows passed with exact retained bytes.
Lunx nevertheless measured level-1/3/5/8/16 medians of `643,013,886`,
`1,083,545,950`, `2,134,196,342`, `2,715,217,975`, and `7,616,394,471`:
regressions of `1.1935%`, `1.6267%`, `0.8729%`, `0.6414%`, and `0.2692%`.
It recommended REVERT and the candidate was removed. Artifacts end in
`seq-to-codes-arrays-{1,2,3}.stat`; broad artifacts end in
`seq-to-codes-arrays.csv`. Parser isolation alone does not pay for an extra
code-array allocation and a second memory representation. A future sequence
port must own a larger transaction or reuse persistent C-style SeqStore
workspace rather than staging another per-block allocation.

Latest kept direct C PreparedSequence entropy port from August 28, 2026: the
C-port block emitter now consumes its existing 16-byte C-style prepared
records directly for history replay, LL/ML/OF counting and table selection,
and reverse FSE emission. It no longer allocates and fills the duplicate
12-byte generic `Sequence` vector for every block. The generic compressor and
its exact-mode search retain the existing materialized path. A stateful
two-block oracle compares the direct path against the former implementation,
including every output byte, final offset history, fresh entropy tables, and
second-block repeat-table behavior.

Full validation passed 714 library tests (709 passed, 5 ignored), 7 integration
tests, all five codegen crates, workspace all-target Clippy with warnings
denied, formatting, release builds, `git diff --check`, and all 1,095 broad
normal/target/prepared-CDict rows exactly. Lunx recommended KEEP after stable
three-by-20 samples. Level-1/3/5/8/16 medians are `627,787,929`,
`1,055,302,323`, `2,102,770,693`, `2,684,909,809`, and `7,579,348,047`,
improvements of `1.2027%`, `1.0223%`, `0.6125%`, `0.4820%`, and `0.2185%`
versus the Fast-codegen checkpoint. Focused bytes remain exact. Counter
artifacts end in `direct-prepared-entropy-{1,2,3}.stat`; broad artifacts end in
`direct-prepared-entropy.csv`. Preserve the dedicated direct path; unlike the
rejected code arrays, it removes an allocation and representation rather than
adding one.

Latest kept explicit Huffman batch-unroll port from August 28, 2026: the
isolated `ruzstd-huff0-codegen` stream now spells out every symbol addition for
C's complete `kUnroll` family 5 through 9. Full batches no longer enter the
runtime `start..end` reverse-symbol loop; only the single variable-width stream
remainder keeps that loop. The existing paired-container order, one-reservation
absolute output cursor, audited raw lookups, safe overlapping word stores, and
low-bit tuple table remain unchanged. The isolated four-stream symbol grows
from `0xf7f` to `0x128d`, deliberately trading cold code size for branch-free
full batches without perturbing the separately compiled matchers.

The table-log 1-11 byte oracle, 714 library tests (709 passed, 5 ignored), 7
integration tests, all codegen crates, strict workspace Clippy, formatting,
release builds, `git diff --check`, and all 1,095 broad rows passed exactly.
Lunx recommended KEEP after stable three-by-20 samples. Level-1/3/5/8/16
medians are `596,490,351`, `1,033,476,436`, `2,082,480,218`, `2,664,241,544`,
and `7,563,948,957`, improvements of `4.9854%`, `2.0682%`, `0.9649%`,
`0.7698%`, and `0.2032%` versus the direct-prepared-entropy checkpoint.
Focused bytes remain exact. Counter artifacts end in
`huff-explicit-unroll-{1,2,3}.stat`; broad artifacts end in
`huff-explicit-unroll.csv`. Preserve all five explicit full-batch functions;
this closes the runtime-loop gap that remained inside the earlier const-generic
outer dispatch.

The pre-unroll 80-run attribution on the retained direct-entropy binary was
Rust/C `2,509,788,942`/`2,001,407,290` instructions at level 1 and
`4,218,064,876`/`3,373,430,518` at level 3. Rust's matcher transactions were
already cheaper (`1.087B` vs `1.134B`, `1.879B` vs `2.035B`). Huffman stream
emission was `477M` vs `339M` at level 1 and `323M` vs `231M` at level 3;
sequence FSE emission was `342M` vs `249M` and `474M` vs `305M`. Artifacts end
in `after-direct-prepared-entropy.perf.{data,txt}`. The explicit unroll removes
about `31.3M` instructions per 20 level-1 runs, so refresh attribution from the
new binary before choosing between the residual Huffman and sequence FSE
boundaries.

Latest kept cross-crate Huffman emission port from August 28, 2026: the full
fixed-table four-stream compressor now lives in the dedicated no-std
`ruzstd-huff0-codegen` crate. It ports C's 64-bit table-log specializations
(`kUnroll` 5 through 9), builds two independent reverse batches before merging
them into the stream, owns the six-byte four-stream jump-table transaction, and
uses only safe bounded overlapping stores. Normal 256-symbol tables take this
path in one cross-crate call; compact dictionary/direct tables retain the
existing checked emitter. The release binary contains one isolated
`encode_four_streams` symbol of `0x1ea4` bytes, so the specializations no longer
perturb Greedy/Lazy code placement.

Lunx's unattended 20-run medians at levels 1/3/5/8/16 are `697,332,385`,
`1,273,481,236`, `3,520,660,311`, `4,416,116,638`, and `7,688,020,879`
instructions: improvements of `4.602%`, `2.575%`, `0.632%`, `0.513%`, and
`0.267%` versus the retained fixed-table baselines. Focused bytes remain exact
at `11,430,500`, `9,977,820`, `9,767,260`, `9,686,720`, and `9,211,100`.
Every unchanged row in the broad matrices is byte-identical: 497 normal, 284
target-2048, and 284 prepared-CDict rows. The excluded 14/8/8 rows are the two
Cargo-manifest fixtures whose source inputs necessarily changed when adding the
crate. A table-log 1 through 11 reference test proves the specialized output
against independent per-symbol four-stream emission. Validation passed the new
crate test, 636 library tests, 7 integration tests, strict all-target Clippy,
formatting, release builds, and `git diff --check`. Counter artifacts are
`benchmarks/tmp/level-{1,3,5,8,16}-cross-crate-paired-huffman.stat`; broad
artifacts end in `after-cross-crate-paired-huffman.csv`. This is the first
successful generated-code isolation boundary; preserve the dedicated crate and
use the same pattern for future large entropy specializations.

Rejected exact C `HUF_CElt`/`HUF_CStream_t` follow-up from August 28, 2026:
the isolated crate converted the tuple table once per four-stream transaction
to C's packed high-bit `size_t` representation, then ported C's two bit
containers, fast/slow final-symbol rules, merge cadence, flushes, and end mark.
The post-KEEP profile motivating it attributed `20.23%` of level-1 sampled
instructions to the isolated Huffman stream symbol, versus `12.12%` for
sequence emission; artifact:
`benchmarks/tmp/perf-instructions-z000033-l1-after-cross-crate-huffman.data`.
The table-log 1-11 reference test, all focused/full tests, strict Clippy,
packaging, formatting, release builds, and broad comparison passed. All 1,080
broad rows with unchanged inputs were byte-identical; the 15 excluded rows are
the edited `ruzstd/Cargo.toml` fixture.

Lunx nevertheless measured level-1/3/5/8/16 medians of `699,050,097`,
`1,278,700,651`, `3,525,494,131`, `4,421,012,144`, and `7,694,860,827`
instructions: regressions of `0.246%`, `0.410%`, `0.137%`, `0.111%`, and
`0.089%` versus the retained cross-crate low-bit batches. The exact C inner
state machine was removed. A post-revert level-1 sample returned to
`697,332,163` instructions with exact bytes. Counter artifacts are
`benchmarks/tmp/level-{1,3,5,8,16}-exact-c-huffman-stream.stat`; broad artifacts
end in `after-exact-c-huffman-stream.csv`; the post-revert artifact is
`benchmarks/tmp/perf-z000033-l1-after-exact-c-huffman-stream-revert.stat`.
KEEP the dedicated crate boundary but retain its Rust low-bit tuple batches:
C's packed high-bit element saves loads but costs more total instructions after
the per-transaction conversion and state-machine operations.

Latest kept one-reservation Huffman output port from August 28, 2026: the
isolated fixed-table four-stream compressor now reserves and initializes its
complete proven payload bound once, then emits all four specialized streams
through one absolute output cursor and truncates only after directly filling
the jump table. This finishes the safe Rust analogue of C's single
`HUF_compress4X_usingCTable_internal()` destination transaction without
reintroducing the previously rejected non-isolated outer boundary. The release
`encode_four_streams` symbol shrank from `0x1ea4` to `0x1d23` bytes.

Lunx's unattended level-1/3/5/8/16 medians are now `695,070,704`,
`1,271,810,296`, `3,517,211,158`, `4,412,570,416`, and `7,686,836,280`
instructions: improvements of `0.324%`, `0.131%`, `0.098%`, `0.080%`, and
`0.015%` over the retained cross-crate baseline. Focused bytes remain exact at
`11,430,500`, `9,977,820`, `9,767,260`, `9,686,720`, and `9,211,100`. All
511 normal, 292 target-2048, and 292 prepared-CDict rows are byte-identical to
the preceding checkpoint. Validation passed the table-log reference test, 636
library tests, 7 integration tests, strict Clippy, formatting, packaging,
release builds, and `git diff --check`. Counter artifacts are
`benchmarks/tmp/level-{1,3,5,8,16}-one-reservation-huffman.stat`; broad
artifacts end in `after-one-reservation-huffman.csv`. KEEP the one-reservation
cursor model; its win is small but stable at every strategy level.

Rejected isolated all-table sequence emitter from August 28, 2026: a dedicated
no-std `ruzstd-sequence-codegen` crate ported the complete hot all-table
`ZSTD_encodeSequences_body()` boundary. It owned LL/ML/OF code conversion,
three borrowed compact FSE table views and states, batched state/extra-bit
writes, state flushes, and the end marker. The existing 12-byte `Sequence` and
12-byte FSE symbol transform moved without layout expansion, and no per-block
staging allocation was added. A test compared every emitted byte with the
retained local encoder across 280 sequences spanning all LL/ML ranges and
offsets through `2^20`; decoder round trips, all focused/full validation, and
all 1,065 broad rows with unchanged source inputs passed.

Lunx measured level-1/3/5/8/16 medians of `691,984,284`, `1,278,011,853`,
`3,608,721,942`, `4,545,639,771`, and `7,681,890,857` instructions. The
changes versus the retained one-reservation baseline were `-0.444%`, `+0.488%`,
`+2.602%`, `+3.016%`, and `-0.064%`. The new crate, dependency, moved core
types, borrowed views, emitter, and reference test were all removed. A
post-revert level-1 sample returned to `695,070,878` instructions with exact
bytes. Counter artifacts are
`benchmarks/tmp/level-{1,3,5,8,16}-isolated-sequence-codegen.stat`; broad
artifacts end in `after-isolated-sequence-codegen.csv`; post-revert:
`benchmarks/tmp/perf-z000033-l1-after-isolated-sequence-codegen-revert.stat`.
Keep the all-table sequence emitter local. A separate crate is effective for
the self-contained Huffman transaction but is not automatically sufficient for
the FSE/Sequence graph shared with table selection and Greedy/Lazy codegen.

Rejected split literal-statistics boundary from August 28, 2026: normal
C-cost aggregate histogram counting and the legacy/small-table four-stream
counter were moved into separate functions. This reduced the aggregate release
symbol to `0x341` bytes and its stack frame from roughly 14 KiB to 6 KiB while
preserving the existing safe four-lane C histogram loop. All 511 normal, 292
target-2048, and 292 prepared-CDict rows were byte-identical, and 697 library
tests (692 passed, 5 ignored), 7 integration tests, the codegen crate test,
strict Clippy, packaging, formatting, release builds, and `git diff --check`
passed. Lunx measured level-1/3/5/8/16 medians of `694,844,238`,
`1,280,006,405`, `3,515,636,543`, `4,411,001,568`, and `7,677,781,242`
instructions: `-0.033%`, `+0.644%`, `-0.045%`, `-0.036%`, and `-0.118%`
versus the one-reservation baseline. The stable level-3 regression outweighed
the small gains, so the split was removed. A post-revert level-3 sample was
`1,271,810,567` instructions with exact bytes. Counter artifacts end in
`split-literal-stats.stat`; broad artifacts end in
`after-split-literal-stats.csv`; post-revert:
`benchmarks/tmp/perf-z000033-l3-after-split-literal-stats-revert.stat`. Keep
the mixed literal-statistics function unless a different histogram algorithm
produces a material cross-level win; stack/code-size reduction alone was not
enough.

Rejected C-width sequence histogram port from August 28, 2026: the complete
`CodeCounts` ownership graph used C's 32-bit counter width instead of Rust
`usize`, covering normal selection, C fast/cost policies, entropy and repeat
costs, estimator/superblock consumers, and an explicit conversion only at the
existing `usize` FSE-normalizer boundary. The safe 256-symbol domain was kept
so `u8` indexing remained statically in bounds. Each histogram shrank from
roughly 2,064 to 1,032 bytes. All 511 normal, 292 target-2048, and 292
prepared-CDict rows were byte-identical; 698 library tests (693 passed, 5
ignored), 7 integration tests, the codegen crate test, strict Clippy,
packaging, formatting, release builds, and `git diff --check` passed.

Lunx measured level-1/3/5/8/16 medians of `695,492,994`, `1,284,258,764`,
`3,616,935,750`, `4,554,702,137`, and `7,707,887,063` instructions:
regressions of `0.061%`, `0.979%`, `2.835%`, `3.221%`, and `0.274%`. The
representation and conversion path were removed. Post-revert level-5/8
samples returned to `3,517,220,646` and `4,412,570,054` instructions with
exact bytes. Counter artifacts end in `c-width-code-counts.stat`; broad
artifacts end in `after-c-width-code-counts.csv`; post-revert artifacts are
`benchmarks/tmp/perf-z000033-l{5,8}-after-c-width-code-counts-revert.stat`.
Keep pointer-width `CodeCounts`: halving its footprint does not compensate for
the conversion and narrower-load effects in the current safe Rust selection
and normalization graph.

Latest kept sequence-preparation CPU port from August 28, 2026: the retained
post-match `prepare_stored_sequences()` scan now uses a safe short-literal
analogue of C's `ZSTD_storeSeq()` wild copy. Its literal buffer reserves 64
padding bytes, 1-64-byte literals use fixed 16/32/48/64-byte initialized
stores when the source has enough input, and the buffer advances by only the
logical literal length via truncation. Long literals and near-end copies keep
the exact-slice path. This avoids the earlier rejected matcher-loop literal
storage and preserves the 12-byte `StoredSequence` and shared entropy layout.
Boundary tests cover every fixed-store transition and source-end fallback.

Lunx's unattended level-1/3/5/8/16 medians are now `690,436,740`,
`1,271,531,320`, `3,517,160,266`, `4,412,500,046`, and `7,687,029,038`
instructions: `-0.667%`, `-0.022%`, `-0.001%`, `-0.002%`, and `+0.003%`
versus the one-reservation baseline. The level-16 movement is neutral; the
level-1 win is material and the other active low levels do not regress.
Focused bytes remain exact at `11,430,500`, `9,977,820`, `9,767,260`,
`9,686,720`, and `9,211,100`. All 511 normal, 292 target-2048, and 292
prepared-CDict rows are byte-identical. Validation passed 699 library tests
(694 passed, 5 ignored), 7 integration tests, the codegen crate test, strict
Clippy, packaging, formatting, release builds, and `git diff --check`.
Artifacts end in `sequence-literal-wildcopy.stat` and
`after-sequence-literal-wildcopy.csv`. KEEP the padded post-match fixed-store
path; unlike storing literals inside the matchers, it improves Fast without
perturbing Greedy/Lazy.

Latest kept numeric-offset history port from August 28, 2026: C-port sequence
preparation now fuses numeric `offBase` resolution and repeat-offset mutation
in one `RepeatOffsets::resolve_and_update_c_value()` transition. The entropy
history pass likewise consumes the already stored nonzero C `offBase` through
`OffsetHistory::update_from_c_offset_value()` instead of resolving it to a raw
offset and walking the repeat history a second time. Generic sequences with a
zero pre-encoded value retain the existing raw-offset encoder. `StoredSequence`
remains 12 bytes and `PreparedSequence` remains 16 bytes because it carries
both the raw-offset debug/parity value and the encoded C value. Exhaustive
tests compare both fused transitions with the former separate operations;
debug builds also verify that production pre-encoded sequences carry the same
resolved raw offset. Two older synthetic entropy fixtures were corrected to
use their actual C repeat resolutions rather than placeholder raw offsets.

Lunx's unattended level-1/3/5/8/16 medians are `682,694,968`,
`1,257,718,491`, `3,498,033,539`, `4,394,623,260`, and `7,668,201,525`
instructions: improvements of `1.121%`, `1.086%`, `0.544%`, `0.405%`, and
`0.245%` over the retained sequence-literal wildcopy baseline. All three
samples at every level were tightly clustered. Focused bytes remain exact at
`11,430,500`, `9,977,820`, `9,767,260`, `9,686,720`, and `9,211,100`.
All 511 normal, 292 target-2048, and 292 prepared-CDict rows are byte-identical.
Validation passed 701 library tests (696 passed, 5 ignored), 7 integration
tests, the codegen crate test and package, strict all-target Clippy, formatting,
and `git diff --check`. Counter artifacts are
`benchmarks/tmp/level-{1,3,5,8,16}-fused-offbase-history.stat`; broad artifacts
end in `after-fused-offbase-history.csv`. KEEP the fused numeric transition as
the authoritative C `ZSTD_updateRep()` boundary.

Rejected August 28 compact `PreparedSequence` experiment: the record stored
either the raw offset or C `offBase` in one `u32` and tagged the kind in the
otherwise unused high match-length bit, reducing the layout from 16 to 12
bytes without narrowing the offset payload. Dedicated layout/kind tests, 702
library tests, 7 integration tests, the codegen tests, strict Clippy, and all
1,095 broad byte rows passed. Lunx nevertheless measured stable level-1/3/5/8/16
medians of `685,430,834`, `1,262,753,980`, `3,601,951,798`, `4,539,149,069`,
and `7,694,627,271` instructions: regressions of `0.401%`, `0.400%`, `2.971%`,
`3.289%`, and `0.345%` versus the fused-offBase baseline. The representation,
constructors, accessors, and fixture conversions were removed. Rejected broad
artifacts end in `after-compact-prepared-sequence.csv`. Keep the 16-byte
prepared record; eliminating its duplicate raw offset perturbs Greedy/Lazy
generated code much more than the smaller scan record saves. Post-revert
level-5/8 samples returned to `3,498,034,008` and `4,394,614,425`
instructions; artifacts end in `compact-prepared-sequence-revert.stat`.

Rejected August 28 direct final offset-history handoff: C-port preparation was
extended to return the fully replayed three-slot repeat history alongside the
prepared block. Normal Fast/DFast/Greedy/optimal entropy emission then mapped
the already stored numeric `offBase` values directly into `Sequence` and used
that final history, avoiding the retained entropy-history replay. The matcher
output intentionally leaves an unused third repeat slot stale in some
single-sequence cases, so the candidate correctly used preparation's canonical
state (`[10, 1, 4]` rather than `[10, 1, 8]`) and debug-verified it against the
old replay. This semantic correction did not change output: 702 library tests
(697 passed, 5 ignored), 7 integration tests, codegen validation, strict
Clippy, and all 1,095 broad rows passed exactly.

Lunx nevertheless measured level-1/3/5/8/16 medians of `670,402,403`,
`1,238,383,172`, `3,572,525,646`, `4,512,348,670`, and `7,663,654,724`
instructions: `-1.801%`, `-1.537%`, `+2.130%`, `+2.679%`, and `-0.059%`
versus the retained fused-offBase baseline. The direct mapping/handoff and
canonical wrapper-state change were removed. Post-revert level-5/8 samples
returned to `3,498,024,048` and `4,394,624,164` instructions with exact bytes.
Counter artifacts end in `direct-final-offset-history-{1,2,3}.stat`; broad
artifacts end in `after-final-offset-handoff.csv`; post-revert artifacts end in
`direct-final-offset-history-revert.stat`. Keep the retained fused numeric
transition and entropy replay: removing this second history walk helps
Fast/DFast, but again perturbs Greedy/Lazy generated code materially.

Rejected August 28 dual-history preparation follow-up: this separated the
previous candidate's two responsibilities completely. Matcher `RepeatOffsets`
remained byte-for-byte unchanged for the following block, while preparation
advanced a distinct entropy `OffsetHistory` from the encoder context in the
same sequence walk. The raw offset resolved through both histories was
debug-checked at every sequence, and only the resulting entropy history was
handed directly to final emission. This closed the multi-block third-slot
ambiguity without adding a second preparation scan. All 702 library tests (697
passed, 5 ignored), 7 integration tests, codegen validation, strict Clippy, and
all 1,095 broad byte rows passed exactly.

Lunx measured level-1/3/5/8/16 medians of `676,215,997`, `1,247,178,536`,
`3,583,244,950`, `4,522,419,655`, and `7,674,847,423` instructions: `-0.949%`,
`-0.838%`, `+2.436%`, `+2.908%`, and `+0.087%` versus the fused-offBase
baseline. The complete dual-state field, context threading, fused preparation
updates, direct entropy handoff, and tests were removed. Post-revert level-5/8
samples returned to `3,498,034,208` and `4,394,623,853` instructions with exact
bytes. Counter artifacts end in `dual-history-preparation-{1,2,3}.stat`; broad
artifacts end in `after-dual-history-preparation.csv`; revert artifacts end in
`dual-history-preparation-revert.stat`. This rules out changed matcher state as
the earlier regression's cause. Do not retry a direct final-history path in the
shared compressor compilation unit; it needs compilation-level isolation.

Rejected August 28 isolated C histogram boundary: the existing
`ruzstd-huff0-codegen` crate took ownership of the complete safe analogue of
`HIST_countFast_wksp()`, not just the large-input counter loop. It included
C's 1,500-byte simple/parallel threshold, four 256-entry `u32` lanes, cached
32-bit preload cadence, terminal-byte handling, lane reduction, max-symbol
discovery, and largest-count calculation. The optional four-stream literal
statistics path remained local because it computes different data. Boundary
tests covered both sides of the threshold and every terminal-read edge; all
1,095 broad rows and the full validation gate were exact.

Lunx measured level-1/3/5/8/16 medians of `700,617,232`, `1,270,638,489`,
`3,607,071,483`, `4,544,919,602`, and `7,708,260,599` instructions:
regressions of `2.625%`, `1.027%`, `3.117%`, `3.420%`, and `0.522%` versus the
fused-offBase baseline. The cross-crate histogram API, cached-load loop, caller
integration, and tests were removed. Post-revert level-1/5/8 samples returned
to `682,698,427`, `3,498,033,636`, and `4,394,623,245` instructions with exact
bytes. Counter artifacts end in `isolated-c-histogram-{1,2,3}.stat`; broad
artifacts end in `after-isolated-c-histogram.csv`; revert artifacts end in
`isolated-c-histogram-revert.stat`. Keep the local `chunks_exact(16)` striped
counter. Compilation isolation does not rescue C's cached unaligned-load
cadence when expressed through safe sliced reads and a cross-crate call.

Rejected August 28 fixed-width Huffman batch follow-up: the retained isolated
four-stream encoder kept its runtime-sized packer only for the initial
remainder, while every complete C `kUnroll` batch used a const-generic
five-through-nine-symbol array reference and reverse index loop. Direct tests
proved every fixed batch against the retained dynamic packer, the table-log
one-through-eleven stream oracle passed, focused bytes stayed exact, and all
511 normal, 292 target-2048, and 292 prepared-CDict rows were byte-identical.
Full tests, strict Clippy, packaging, formatting, release builds, and
`git diff --check` also passed before measurement.

Lunx measured level-1/3/5/8/16 medians of `683,809,753`, `1,258,468,460`,
`3,498,711,292`, `4,395,316,247`, and `7,668,525,239` instructions:
regressions of `0.163%`, `0.060%`, `0.019%`, `0.016%`, and `0.004%` versus the
fused-offBase baseline. All 15 samples were tightly clustered, so the fixed
packer and its test were removed. Counter artifacts end in
`fixed-width-huffman-batches-{1,2,3}.stat`; rejected broad artifacts end in
`after-fixed-width-huffman-batches.csv`. Keep the retained inlined dynamic
subslice packer: LLVM's existing specialization is marginally better than
forcing array conversion and a const reverse-index loop at every full batch.

Latest kept sequence-selection code-layout change from August 28, 2026: the
exact combinatorial sequence-mode search now lives in a separate non-inlined
`choose_exact_sequence_table_modes()` transaction. C levels always disable
that legacy/file-profile search, but its candidate construction and nested
exact-size evaluation previously occupied the same 18,009-byte release symbol
as their common selector. The common symbol is now 5,515 bytes and the isolated
exact-only body is 12,945 bytes. Exact-search tests, focused bytes, all 511
normal, 292 target-2048, and 292 prepared-CDict rows, 701 library tests, 7 doc
tests, strict Clippy, packaging, formatting, release builds, and
`git diff --check` all passed.

Lunx's unattended level-1/3/5/8/16 medians are now `681,588,277`,
`1,255,863,163`, `3,495,985,588`, `4,392,754,912`, and `7,666,283,736`
instructions: improvements of `0.162%`, `0.148%`, `0.059%`, `0.043%`, and
`0.025%` versus the fused-offBase baseline. Focused bytes remain exact at
`11,430,500`, `9,977,820`, `9,767,260`, `9,686,720`, and `9,211,100`.
Counter artifacts end in `exact-sequence-search-split-{1,2,3}.stat`; broad
artifacts end in `after-exact-sequence-search-split.csv`. KEEP this explicit
code-layout boundary: unlike small shared-loop rewrites, moving the entire
dormant exact-search ownership graph out of the C-strategy selector improves
every active strategy without changing its data layout.

Latest kept policy-isolated sequence selection from August 28, 2026: the
non-exact selector now dispatches the complete three-table transaction to
separate non-inlined C Fast, C cost, and legacy generated functions. The old
5,515-byte shared selector disappeared; release symbols are 1,507 bytes for C
Fast, 1,396 for C cost, 3,050 for legacy, and the already isolated exact search
remains 12,945 bytes. Each C policy owns its LL/ML/OF histogram-to-mode calls
without carrying the other policy or legacy decision graph. Records, tables,
and bytes are unchanged. Policy-specific tests, the exact-search oracle, 701
library tests, 7 doc tests, strict Clippy, packaging, formatting, release
builds, `git diff --check`, and all 1,095 broad rows passed.

Lunx's unattended level-1/3/5/8/16 medians are now `681,032,274`,
`1,254,871,589`, `3,494,953,335`, `4,391,789,596`, and `7,664,903,280`
instructions: further improvements of `0.082%`, `0.079%`, `0.030%`, `0.022%`,
and `0.018%` versus the exact-search-only split. Focused bytes remain
`11,430,500`, `9,977,820`, `9,767,260`, `9,686,720`, and `9,211,100`.
Counter artifacts end in
`policy-isolated-sequence-selection-{1,2,3}.stat`; broad artifacts end in
`after-policy-isolated-sequence-selection.csv`. KEEP the policy boundaries;
they extend the exact-search isolation win across every active strategy.

Rejected August 28 sequence-emitter variant isolation: the unchanged
`encode_sequences()` implementation was divided into a 376-byte dispatcher and
separate non-inlined all-table, mixed-table, and all-RLE transactions of 4,005,
4,298, and 1,253 release bytes. The retained batching, `BitWriter`, FSE tables,
and sequence records were untouched. Variant tests, exact-search sizing, all
1,095 broad rows, and the full strict validation gate passed exactly.

Lunx measured level-1/3/5/8/16 medians of `681,909,078`, `1,256,536,863`,
`3,593,755,427`, `4,531,673,040`, and `7,666,823,422` instructions: regressions
of `0.129%`, `0.133%`, `2.827%`, `3.185%`, and `0.025%` versus the retained
policy-isolated selector baseline. The dispatcher, mixed helper, and forced
non-inlining were removed. Counter artifacts end in
`sequence-emitter-variant-split-{1,2,3}.stat`; broad artifacts end in
`after-sequence-emitter-variant-split.csv`. Keep the unified/inlined local
emitter. Unlike isolating dormant selection graphs, adding calls between hot
emission variants materially damages Greedy/Lazy generated code.

Latest kept target-only literal code-layout change from August 28, 2026:
`compress_literals()` now moves the complete preferred-valid-repeat
transaction behind a non-inlined `try_preferred_repeat_literals()` boundary.
That transaction is enabled only by target-block selection and owns the prior
table checks, small-literal limit, RLE/treeless emission, expansion rollback,
and raw fallback. Normal focused compression never calls it. The release
helper is 447 bytes and the common literal compressor shrank from 5,280 to
4,865 bytes. Targeted repeat-table tests, 701 library tests (696 passed, 5
ignored), 7 doc tests, codegen tests, strict Clippy, packaging, formatting,
release builds, `git diff --check`, focused bytes, and all 511 normal, 292
target-2048, and 292 prepared-CDict rows passed exactly.

Lunx's unattended level-1/3/5/8/16 medians are `681,033,417`,
`1,254,869,656`, `3,494,951,231`, `4,391,787,357`, and `7,665,098,189`
instructions: `+0.000168%`, `-0.000154%`, `-0.000060%`, `-0.000051%`, and
`+0.002543%` versus the policy-isolated selector baseline. Focused bytes remain
`11,430,500`, `9,977,820`, `9,767,260`, `9,686,720`, and `9,211,100`.
Counter artifacts end in
`preferred-repeat-literal-split-{1,2,3}.stat`; broad artifacts end in
`after-preferred-repeat-literal-split.csv`. KEEP the boundary as a neutral
generated-code cleanup: it removes an entirely dormant target-only ownership
graph from normal compression without the Greedy/Lazy regressions caused by
splitting hot entropy emission.

Rejected August 28 literal-policy specialization: the complete literal
compression transaction was const-specialized by `search_smallest_table` and
`c_literal_cost_model`, producing four separate non-inlined release bodies of
1.8-2.8 KiB instead of the retained 4.8 KiB union. This gave low C levels a
body without the exhaustive Huffman-table search and gave each legacy policy
its own table construction, repeat comparison, gain test, and emission path.
Focused bytes, all 1,095 broad rows, 701 library tests, 7 doc tests, codegen
tests, strict Clippy, packaging, formatting, release builds, and
`git diff --check` passed exactly.

Lunx measured level-1/3/5/8/16 medians of `680,802,084`, `1,253,381,059`,
`3,590,407,174`, `4,528,393,540`, and `7,661,857,087` instructions:
`-0.034%`, `-0.119%`, `+2.731%`, `+3.110%`, and `-0.042%` versus the retained
preferred-repeat baseline. The const dispatch and four policy bodies were
removed. Counter artifacts end in
`literal-policy-specialization-{1,2,3}.stat`; broad artifacts end in
`after-literal-policy-specialization.csv`. This is not analogous to the kept
sequence-policy split: duplicating the literal writer/table transaction also
duplicates hot entropy emission and again perturbs Greedy/Lazy materially.
Keep the shared literal compressor and isolate only genuinely cold ownership
graphs such as the target-only preferred-repeat transaction.

Rejected August 28 compact C `SeqDef` port: matcher output ownership across
Fast, DFast, Greedy/Lazy, and optimal parsing was moved from Rust's 12-byte
`{u32 lit_len, u32 match_len, u32 offBase}` record to C's exact 8-byte
`{u32 offBase, u16 litLength, u16 mlBase}` representation. A block-level
long-length type and position restored C's single overflowing literal or match
length, and direct coverage exercised both >65,535 cases. The compact store was
threaded through retained allocation recycling and every preparation path.
All 702 library tests (697 passed, 5 ignored), 7 doc tests, strict Clippy,
formatting, release builds, `git diff --check`, focused bytes, and all 1,095
broad rows passed exactly.

Lunx measured level-1/3/5/8/16 medians of `726,375,370`, `1,294,874,612`,
`3,898,288,421`, `4,852,875,458`, and `7,687,098,506` instructions:
regressions of `6.658%`, `3.188%`, `11.541%`, `10.499%`, and `0.287%` versus
the retained preferred-repeat baseline. The compact record, store wrapper,
long-length metadata, accessors, and tests were removed. Post-revert level
1/5/8 samples returned to `681,032,041`, `3,494,937,948`, and `4,391,798,633`
instructions with exact bytes. Candidate artifacts end in
`compact-seqdef-{1,2,3}.stat`; revert artifacts end in
`compact-seqdef-revert.stat`; broad artifacts end in
`after-compact-seqdef.csv`. Keep the direct 12-byte full-length record. The
extra store wrapper and long-length reconstruction dominate any cache benefit,
and changing this shared record again perturbs Greedy/Lazy far more than C's
compact layout can recover in the current Rust pipeline.

Rejected August 28 bounded native-width sequence-code histograms: the complete
sequence-cost graph reduced each `CodeCounts` histogram from `[usize; 256]`
(2,064 bytes including the total) to `[usize; 64]` (528 bytes). This retained
native-width counters, unlike the earlier rejected C-width conversion, and
direct coverage proved the actual domains remain LL 0-35, ML 0-52, and OF
0-31. Focused bytes, all 1,095 broad rows, 702 library tests (697 passed, 5
ignored), 7 doc tests, Huffman codegen tests, strict Clippy, packaging,
formatting, release builds, and `git diff --check` passed exactly.

Lunx measured level-1/3/5/8/16 medians of `684,261,292`, `1,249,638,683`,
`3,587,374,595`, `4,526,079,269`, and `7,628,136,556` instructions: `+0.474%`,
`-0.417%`, `+2.644%`, `+3.058%`, and `-0.482%` versus the retained
preferred-repeat baseline. The bounded histograms and domain test were
removed. Post-revert level-1/5/8 samples returned to `681,029,938`,
`3,494,937,823`, and `4,391,785,472` instructions with exact bytes; the full
restored library suite passed 696 tests with 5 ignored. Candidate artifacts
end in `bounded-code-counts-{1,2,3}.stat`; revert artifacts end in
`bounded-code-counts-revert.stat`; broad artifacts end in
`after-bounded-code-counts.csv`. Keep the 256-entry arrays: indexing by the
full `u8` code domain is statically in bounds, while the smaller physical
arrays add bounds/generated-code effects and again perturb Greedy/Lazy far
more than their reduced footprint recovers.

Rejected August 28 direct preallocated `SeqStore` output arena: the complete
Fast/DFast post-match preparation transaction replaced repeated literal
`Vec::extend_from_slice()`/truncate operations and prepared-sequence pushes
with C-shaped reserved literal and sequence arrays. One narrowly owned
`MaybeUninit` writer performed the retained bounded 16/32/48/64-byte literal
wild copies directly into reserved storage, wrote every `PreparedSequence` by
index, and exposed each initialized prefix once at the end. Greedy/Lazy kept
its separate retained preparation loop. Boundary coverage exercised every
wild-copy size and near-source-end fallback. Focused bytes, all 1,095 broad
rows, 701 library tests (696 passed, 5 ignored), 7 integration tests, Huffman
codegen tests and packaging, strict Clippy, formatting, release builds, and
`git diff --check` passed exactly.

Lunx measured level-1/3/5/8/16 medians of `680,222,656`, `1,253,323,119`,
`3,591,958,291`, `4,529,963,261`, and `7,664,898,035` instructions: `-0.119%`,
`-0.123%`, `+2.776%`, `+3.146%`, and `-0.003%` versus the retained
preferred-repeat baseline. The arena module, raw initialized-prefix boundary,
and replacement tests were removed. Post-revert level-1/5/8 samples returned
to `681,031,844`, `3,494,951,077`, and `4,391,798,609` instructions with exact
bytes. Candidate artifacts end in `direct-seqstore-arena-{1,2,3}.stat`; revert
artifacts end in `direct-seqstore-arena-revert.stat`; broad artifacts end in
`after-direct-seqstore-arena.csv`. Keep the safe `Vec` extend/truncate and push
path: eliminating its repeated initialized boundaries is a real but very small
Fast/DFast gain, while the added arena code again perturbs Greedy/Lazy by about
3% despite those strategies not calling it.

Rejected August 28 cross-crate `SeqStore` ownership port: a new dedicated
no-std `ruzstd-seqstore-codegen` crate owned the complete C `StoredSequence`,
`PreparedSequence`, and repeat-offset representations, their numeric
`ZSTD_updateRep()` transition, and the direct preallocated Fast/DFast output
arena. The main crate retained a narrow facade, while Greedy/Lazy kept its
existing preparation algorithm. The isolated preparation body shrank from a
1,671-byte local symbol to a 1,072-byte external symbol. Layout, repeat-state,
wild-copy, and fallback coverage passed; focused bytes were exact. Full
validation passed 699 `ruzstd` library tests (694 passed, 5 ignored), 7
integration tests, 4 new crate tests, the Huffman codegen test, packaging,
strict Clippy, formatting, release builds, and `git diff --check`. All 1,065
broad rows whose source inputs did not change were byte-identical. The other
30 rows were the two Cargo manifests changed by adding the crate and remained
decode-verified against C.

Lunx measured level-1/3/5/8/16 medians of `680,620,453`, `1,253,936,185`,
`3,816,757,998`, `4,771,908,781`, and `7,657,776,362` instructions: `-0.061%`,
`-0.074%`, `+9.208%`, `+8.655%`, and `-0.096%` versus the retained baseline.
Moving the shared hot record and repeat primitives across the crate boundary
changed downstream monomorphization: the three Greedy/Lazy matcher bodies
became 2,640-3,191 bytes instead of the retained 3,631-4,329 bytes, but were
far slower. The crate, dependency, shared external types, facade, arena, and
tests were removed. Post-revert the exact original Greedy/Lazy symbol hashes,
addresses, and sizes returned, and level-1/5/8 samples returned to
`681,033,138`, `3,494,952,646`, and `4,391,784,979` instructions with exact
bytes. Candidate artifacts end in `cross-crate-seqstore-{1,2,3}.stat`; revert
artifacts end in `cross-crate-seqstore-revert.stat`; broad artifacts end in
`after-cross-crate-seqstore.csv`. A codegen crate isolates an owning function,
but not types and inline methods consumed by hot main-crate matchers. Keep the
sequence records and repeat primitives local; future isolation must use a
primitive ABI boundary that does not migrate their Rust type identity.

Latest kept August 28 primitive-ABI `SeqStore` isolation: the dedicated no-std
`ruzstd-seqstore-codegen` crate now owns only the complete Fast/DFast
post-match preparation transaction and communicates through fixed primitive
`[u32; 3]` stored-sequence and `[u32; 4]` prepared-sequence word arrays.
`StoredSequence`, `PreparedSequence`, `RepeatOffsets`, and every inline repeat
method remain local to `ruzstd`. Compile-time size, alignment, and field-offset
assertions prove the two narrow owning casts, and the leaf crate contains the
C-shaped preallocated literal/sequence arena, bounded 16/32/48/64-byte wild
copies, numeric repeat transition, and single initialized-prefix handoff. The
isolated release body is 1,087 bytes. All three Greedy/Lazy bodies retain their
exact 3,631/4,090/4,329-byte sizes, and their complete instruction-mnemonic
sequences hash identically to the pre-candidate binary.

Lunx measured level-1/3/5/8/16 medians of `681,236,851`, `1,255,079,947`,
`3,494,938,092`, `4,391,787,263`, and `7,664,899,486` instructions: `+0.030%`,
`+0.017%`, `-0.0004%`, `-0.000002%`, and `-0.00005%` versus the preceding
preferred-repeat baseline. These are effectively neutral across every
strategy, with exact focused bytes. Validation passed 701 library tests (696
passed, 5 ignored), 7 integration tests, 3 new primitive codegen tests, the
Huffman codegen test, both leaf-crate package verifications, strict Clippy,
formatting, release builds, and `git diff --check`. All 1,095 candidate broad
rows match the prior manifest-equivalent matrix exactly; 1,065 rows also match
the preceding retained matrix, while 30 rows use the two Cargo manifests whose
input changed by adding the crate. Counter artifacts end in
`primitive-abi-seqstore-{1,2,3}.stat`; broad artifacts end in
`after-primitive-abi-seqstore.csv`. KEEP this neutral structural boundary: it
is the first sequence-store isolation that preserves Greedy/Lazy generated
code and provides a safe separately compiled target for larger Fast/DFast C
ports without migrating shared Rust type identity.

Latest kept August 28 persistent primitive `SeqStore` workspace: normal
Fast/DFast block encoding now keeps the leaf crate's prepared literal and
sequence allocations in `FastMatchState`/`DFastMatchState` across blocks,
matching C's persistent `SeqStore_t` lifetime. Preparation takes the existing
primitive buffers, clears and reuses their allocations, and every compressed
or raw emission exit recycles them back into matcher state. The shared local
Rust record types remain outside the codegen crate, target mode still converts
to its existing owning representation, and Greedy/Lazy generated bodies retain
their exact 3,631/4,090/4,329-byte sizes and instruction-mnemonic hashes.
End-to-end Fast and DFast tests prove allocation identity across a compressed
block followed by raw fallback.

Lunx measured level-1/3/5/8/16 medians of `680,583,126`, `1,252,665,609`,
`3,494,756,270`, `4,391,599,355`, and `7,664,109,191` instructions: `-0.096%`,
`-0.192%`, `-0.005%`, `-0.004%`, and `-0.010%` versus the primitive-boundary
baseline. Focused bytes remained exact. Validation passed 703 library tests
(698 passed, 5 ignored), 7 integration tests, 4 sequence-codegen tests, the
Huffman codegen test, both leaf packages, strict Clippy, formatting, release
builds, and `git diff --check`. All 1,095 normal, target-2048, and prepared-
CDict rows match the prior retained matrix exactly. Counter artifacts end in
`persistent-primitive-seqstore-{1,2,3}.stat`; broad artifacts end in
`after-persistent-primitive-seqstore.csv`. KEEP this as the new baseline.

Latest kept August 28 DFast store-operation codegen change: the tiny
`dfast_helpers::store_match()` operation is now forced inline into the normal
and external-dictionary DFast matchers, matching C's inline
`ZSTD_storeSeqOnly()` update shape. This removes the standalone release helper
and its repeated sequence-push/IP/anchor call boundary without touching Fast
or any Greedy/Lazy code. Lunx measured level-1/3/5/8/16 medians of
`680,583,390`, `1,242,968,242`, `3,494,756,455`, `4,391,633,868`, and
`7,664,108,408` instructions: `+0.000039%`, `-0.774%`, `+0.000005%`,
`+0.000786%`, and `-0.000010%` versus the persistent-workspace baseline.
Focused bytes and all 1,095 broad rows remained exact. Validation passed 703
library tests (698 passed, 5 ignored), 7 integration tests, both codegen crate
tests, strict Clippy, formatting, release builds, and `git diff --check`.
Artifacts end in `dfast-inline-store-{1,2,3}.stat`; broad artifacts end in
`after-dfast-inline-store.csv`. KEEP this as the current level-3 baseline.

Rejected August 28 DFast `ZSTD_selectAddr()` candidate-load port: the normal
DFast inner search selected an always-readable source or dummy address for
both C candidate-validity boundaries, then compared the unconditional 64-bit
or 32-bit load before testing validity. Unsafe pointer selection stayed in
`unaligned.rs`, and focused output, 704 library tests (699 passed, 5 ignored),
the integration/codegen tests, strict Clippy, formatting, release builds, and
all 1,095 broad rows passed. Lunx nevertheless measured level-1/3/5/8/16
medians of `680,583,202`, `1,317,014,505`, `3,591,760,210`,
`4,529,796,266`, and `7,664,104,291` instructions. Levels 1/16 were neutral,
but the directly affected level 3 regressed `5.957%` and levels 5/8 regressed
`2.776%`/`3.146%`. The selected-read helpers and both call sites were removed.
Artifacts end in `dfast-selectaddr-{1,2,3}.stat`; broad artifacts end in
`after-dfast-selectaddr.csv`. Keep the explicit candidate-validity branches:
the C dummy-address shape is substantially worse in the Rust generated body.

Latest kept August 28 DFast raw table-access port: all normal and external-
dictionary DFast hash-table reads and writes now cross the audited
`dfast_table.rs` boundary. `DFastMatchState::ensure_tables()` establishes
exactly `1 << log` entries and every call site supplies a hash reduced to that
same log; debug builds assert the slot and release mirrors C's unchecked table
access. Dictionary construction remains checked because it is not hot. This
removes the repeated cold bounds-panic edges from the generated matcher and
shrinks its main release symbol from about `0x3504` to `0x3053` bytes. Lunx
measured level-1/3/5/8/16 medians of `680,582,604`, `1,122,787,855`,
`3,494,721,120`, `4,391,634,539`, and `7,664,108,827` instructions:
`-0.000115%`, `-9.669%`, `-0.001011%`, `+0.000015%`, and `+0.000005%` versus
the DFast-inline baseline. Focused bytes remained exact. Validation passed 704
library tests (699 passed, 5 ignored), 7 integration tests, both codegen
crates, strict Clippy, formatting, release builds, and all 1,095 normal,
target-2048, and prepared-CDict rows. Counter artifacts end in
`dfast-unchecked-table-{1,2,3}.stat`; broad artifacts end in
`after-dfast-unchecked-table.csv`. KEEP this as the current baseline.

Latest kept August 28 complete FSE CTable raw-access port: `FSETable` now
mirrors C's validated raw compression table across both halves of its
lifecycle. Emission uses unchecked symbol-transform and compact-state-table
loads after table selection has established symbol validity; construction uses
the normalized-distribution, masked-position, and cumulative-range invariants
for spread/state writes. Debug assertions remain at every unsafe boundary, and
an exhaustive test traverses every encodable symbol and every valid state for
the three predefined tables plus a mixed normalized table. The retained bit
writer, sequence-emitter layout, and owning vectors are unchanged.

Lunx measured level-1/3/5/8/16 medians of `667,947,037`, `1,099,186,757`,
`3,469,610,893`, `4,369,265,957`, and `7,629,779,483` instructions:
improvements of `1.857%`, `2.102%`, `0.719%`, `0.509%`, and `0.448%` versus
the DFast raw-table baseline. Every measured level improves and focused bytes
remain exact. Validation passed 705 library tests (700 passed, 5 ignored), 7
integration tests, both codegen crates, strict Clippy, formatting, release
builds, and all 1,095 normal, target-2048, and prepared-CDict rows. Counter
artifacts end in `fse-raw-table-{1,2,3}.stat`; broad artifacts end in
`after-fse-raw-table.csv`. KEEP this as the current baseline. Unsafe FSE table
access must remain inside `fse_encoder/table.rs` and retain its construction
and transform invariants. The post-change level-3 profile is
`benchmarks/tmp/profile-c-port-z000033-l3-after-fse-raw-table.perf.data`.
DFast is about `1.96B` sampled instructions for 80 runs versus C's `2.02B`, so
do not return to its matcher without new evidence. Sequence emission is about
`512M` versus C's `383M`, and Huffman stream emission about `334M` versus C's
`215M`; the next large boundary should come from those retained entropy paths,
without retrying the rejected separate sequence writer/crate shapes.

Latest kept August 28 isolated Huffman raw-cursor port: the retained
`ruzstd-huff0-codegen` four-stream emitter now crosses one complete audited C-
style cursor boundary. Batch traversal uses raw bounded input indexes and the
complete 256-entry code table, while every container flush uses an unaligned
overlapping word store. `encode_four_streams()` still reserves the sum of the
four tight stream bounds plus seven initialized padding bytes before emission,
and debug assertions state each input and output invariant. No representation,
unroll cadence, stream split, or caller layout changed. The release
`encode_stream` symbol shrank from `0x5a6` to `0x29d`, and the complete
four-stream symbol from `0x1d23` to `0xf7f`.

Lunx measured level-1/3/5/8/16 medians of `658,967,804`, `1,092,749,602`,
`3,463,615,192`, `4,363,196,208`, and `7,624,930,884` instructions:
improvements of `1.344%`, `0.586%`, `0.173%`, `0.139%`, and `0.064%` versus
the FSE raw-table baseline. Every guard level improves and focused bytes remain
exact. Validation passed 705 library tests (700 passed, 5 ignored), 7
integration tests, both codegen crates, strict Clippy, formatting, release
builds, and all 1,095 normal, target-2048, and prepared-CDict rows. Counter
artifacts end in `huffman-raw-cursor-{1,2,3}.stat`; broad artifacts end in
`after-huffman-raw-cursor.csv`. KEEP this as the current baseline. Preserve
the reservation-plus-padding proof with the raw cursor; do not expose these
unchecked accesses outside the isolated codegen crate. The post-change profile
is `benchmarks/tmp/profile-c-port-z000033-l3-after-huffman-raw-cursor.perf.data`.
Sequence emission remains about `524M` sampled instructions per 80 runs versus
C's `383M`; Huffman stream emission is now about `320M` versus C's `215M`.
Audit the retained unified sequence emitter next, but do not recreate the
rejected separate writer, variant split, or codegen-crate architectures.

Latest kept August 28 audited raw row-table port: active-prefix lookup,
attached-CDict lookup, row-head rotation, and row update/insertion now cross a
single `row_table.rs` boundary mirroring C's raw row pointers. Debug assertions
tie every unchecked access to `GreedyMatchState::ensure_tables()`: both tables
have exactly `1 << hash_log` entries, row starts use the corresponding high
hash bits, and positions are masked to the complete 16/32/64-entry row. Tests
exercise the first and last complete rows. `row_match.rs` remains
`#![forbid(unsafe_code)]`; unchecked access is confined to the audited helper.
The three Lazy-family release bodies shrink from approximately
`0xe2f`/`0xffa`/`0x10e9` bytes to `0xa50`/`0xbc2`/`0xc77`.

Lunx measured level-1/3/5/8/16 instruction medians of `658,971,056`,
`1,092,778,624`, `3,246,428,797`, `4,076,027,746`, and `7,625,098,399`:
`+0.000493%`, `+0.002656%`, `-6.271%`, `-6.582%`, and `+0.002197%` versus
the Huffman raw-cursor baseline. The guard levels are neutral and both directly
affected Greedy/Lazy levels improve materially, so Lunx recommended KEEP.
Focused bytes and all 1,095 normal, target-2048, and prepared-CDict rows are
exact. Validation passed 706 library tests (701 passed, 5 ignored), 7
integration tests, both codegen suites, strict Clippy, formatting, release
builds, and `git diff --check`. Counter artifacts end in
`row-raw-table-{1,2,3}.stat`; broad artifacts end in `after-row-raw-table.csv`.
KEEP this as the current cross-level baseline and keep all unsafe row access in
`row_table.rs` under the documented sizing/hash invariants.

Latest kept August 28 Greedy/Lazy generated-mode specialization: the complete
block parser now carries `DEPTH`, `EXT_DICT`, and `ATTACHED_DICT` as const
parameters alongside the search family. This mirrors C's separately generated
greedy/lazy/lazy2 and noDict/extDict/dictMatchState bodies. Normal compression
no longer executes runtime depth branches, external-dictionary repeat/window
checks, or attached-dictionary setup/dispatch. Row and binary-tree finders also
receive the attached-mode constant, and `LazyDictionaryBounds` compiles the
no-dictionary rep-match and backward-extension path separately from extDict.
All entry points select the specialization before entering the hot parser.

The normal row bodies shrink again from approximately
`0xa50`/`0xbc2`/`0xc77` bytes after the raw row-table port to about
`0x7b9`/`0x803`/`0x886`. Lunx measured level-1/3/5/8/16 medians of
`658,992,365`, `1,092,842,400`, `3,165,131,463`, `4,081,241,793`, and
`7,626,305,663`: `+0.0032%`, `+0.0058%`, `-2.5042%`, `+0.1279%`, and
`+0.0158%` versus the raw-row-table baseline. Level 5 improves materially;
the guard levels are neutral and level 8 remains inside the established `0.2%`
material-regression threshold. Lunx recommended KEEP. Focused bytes and all
1,095 normal, target-2048, and prepared-CDict rows are exact. Validation passed
706 library tests (701 passed, 5 ignored), 7 integration tests, both codegen
suites, strict Clippy, formatting, release builds, and `git diff --check`.
Counter artifacts end in
`greedy-mode-depth-specialization-{1,2,3}.stat`; broad artifacts end in
`after-greedy-mode-depth-specialization.csv`. KEEP this as the current
cross-level baseline and preserve specialization at the parser entry points.

Rejected August 28 compact coded-sequence port: an encoder-only 12-byte
`EntropySequence` packed the precomputed LL and ML codes into the unused high
bytes of their bounded `u32` values. This preserved the decoder's public
sequence type and avoided the earlier stored-code experiment's 16-byte record;
normal selection, post-split estimation, superblock estimation, table
construction, and final emission all consumed the stored codes and table-driven
extra-bit metadata. Exhaustive tests covered every legal LL/ML value and the
focused offset range. Before measurement, 706 library tests (701 passed, 5
ignored), 7 integration tests, both codegen crates, strict Clippy, formatting,
release builds, and all 1,095 broad rows passed with exact retained bytes.

Lunx nevertheless measured level-1/3/5/8/16 instruction medians of
`659,025,297`, `1,092,681,427`, `3,560,721,634`, `4,500,743,462`, and
`7,652,169,727`: `+0.009%`, `-0.006%`, `+2.804%`, `+3.152%`, and `+0.357%`
versus the retained Huffman-cursor baseline. Focused bytes were exact. Lunx
recommended REVERT, and the complete packed representation and its consumers
were removed. Artifacts end in `compact-coded-sequences-{1,2,3}.stat`. This
rules out record expansion as the sole cause of the stored-code regression:
even a 12-byte packed record materially perturbs Greedy/Lazy generated code.
Do not retry shared precomputed LL/ML storage without stronger compilation/link
isolation or a design that proves those hot functions remain unchanged.

Rejected August 28 C-width Huffman workspace port: `HUF_sort()` rank cursors
were reduced from Rust's 16-byte `{usize, usize}` entries to C's 4-byte
`{u16, u16}` representation, and the already-allocated compact node workspace
used one audited raw cursor through sorting, parent construction, depth
propagation, and base-length extraction. The retained nonzero-symbol ordering
was unchanged; this did not retry C's previously rejected zero-count workspace.
A checked reference matched every node field across 256 generated histograms.
The release tree-builder stack frame shrank from `0xc18` to `0x328` bytes and
the function to `0x601` bytes. All strict tests and all 1,095 broad rows passed
with exact bytes before measurement.

Lunx measured level-1/3/5/8/16 medians of `653,952,526`, `1,076,384,029`,
`3,543,925,286`, `4,485,133,003`, and `7,566,847,705`: `-0.761%`,
`-1.498%`, `+2.319%`, `+2.795%`, and `-0.762%` versus the retained baseline.
The local port is genuinely cheaper at Fast, DFast, and optimal levels, but it
again perturbs Greedy/Lazy enough to fail the cross-level gate. Lunx recommended
REVERT and the complete workspace candidate was removed. Artifacts end in
`compact-huffman-workspace-{1,2,3}.stat`. Do not retry compact rank cursors or
raw Huffman-tree nodes in the shared encoder binary until hot-code section
placement can be isolated from Greedy/Lazy.

Rejected August 28 Greedy/Lazy source-loop alignment experiment: binary A/B
inspection first established that the compact Huffman rank-only diagnostic did
not change the hot matcher/parser bodies themselves. It moved the row matcher
and all three lazy-parser monomorphizations by exactly `0x20`; the complete
`row_find_best_match` body was byte-for-byte identical, and the first lazy body
differed only in one relocation byte. Inserting x86-64 `.p2align 6` boundaries
before the row-search, attached-row, cache-fill, row-update, and outer lazy
loops made all inspected hot symbol addresses and sizes identical with and
without that entropy-layout perturbation. This proved link placement was the
cause of the earlier cross-strategy coupling, but did not prove that a zero-
modulo-64 loop address was favorable.

The isolated alignment candidate preserved focused bytes and passed 705
library tests (700 passed, 5 ignored), 7 integration tests, both codegen crate
suites/packages, strict Clippy, formatting, release builds, and all 1,095
normal, target-2048, and prepared-CDict rows. Lunx measured level-1/3/5/8/16
instruction medians of `658,973,918`, `1,092,750,013`, `3,503,365,988`,
`4,412,500,522`, and `7,624,931,297`: `+0.001%`, `+0.000038%`, `+1.148%`,
`+1.130%`, and `+0.000005%` versus the retained baseline. The alignment itself
therefore materially worsened Greedy/Lazy and Lunx recommended REVERT. The
helper and all five alignment sites were removed. Counter artifacts end in
`greedy-hot-loop-alignment-{1,2,3}.stat`; broad artifacts end in
`after-greedy-hot-loop-alignment.csv`. Do not retry blanket source-level
cache-line alignment. Future isolation should preserve the retained hot-body
layout through a compilation/link boundary rather than selecting a new common
loop offset.

Rejected August 28 isolated full C-style Huffman table-builder port: the
existing `ruzstd-huff0-codegen` crate temporarily owned the whole normal table
transaction rather than only stream emission. One opaque reusable workspace
covered C-width rank sorting, raw compact-node parent construction, height
redistribution, and canonical-code extraction. The main crate retained weight
serialization because it owns the FSE encoder. Public-boundary checks rejected
unrepresentable alphabets, counts outside `u32`, and a total equal to C's
`u32::MAX` cursor barrier. A checked in-crate builder matched the isolated
result across 256 generated histograms and table logs 4 through 11.

This was a genuine large compilation-isolation test. The external builder was
a standalone `0xc43`-byte symbol, while the retained row updater and all three
lazy-parser bodies kept both their exact sizes and their favorable retained
cache-line offsets (`0x30`, `0x20`, `0x20`, and `0x10`) despite large absolute
address movement. Focused bytes remained exact. Validation passed 706 library
tests (701 passed, 5 ignored), 7 integration tests, both codegen suites, the
Huffman package, strict Clippy, formatting, release builds, and all 1,095 broad
rows. Lunx nevertheless measured level-1/3/5/8/16 instruction medians of
`658,446,778`, `1,091,811,756`, `3,559,887,076`, `4,500,775,430`, and
`7,633,557,690`: `-0.079%`, `-0.086%`, `+2.780%`, `+3.153%`, and `+0.113%`
versus the retained baseline. Lunx recommended REVERT, and the entire builder
module, integration, scratch conversion, and generated reference test were
removed. Counter artifacts end in
`isolated-huffman-table-builder-{1,2,3}.stat`; broad artifacts end in
`after-isolated-huffman-table-builder.csv`. Preserving the obvious hot-symbol
size/alignment was not sufficient: a cross-crate construction call or wider
whole-binary cache/link interaction still penalized Greedy/Lazy. Do not move
the table builder wholesale across this crate boundary again without a more
direct causal profile.

Rejected precursor from the same checkpoint: combining the DFast inline store
with Fast source-sequence allocation reuse and Fast store inlining improved
level 3 by `0.777%`, but regressed the directly affected level-1 path by a
tight `0.338%`; levels 5/8/16 stayed neutral and all bytes were exact. The Fast
matcher body grew from 9,387 to 12,261 bytes. Lunx recommended REVERT, so the
Fast state, recycling, inline attributes, and lifecycle assertions were
removed before measuring the isolated retained DFast change. Rejected counter
artifacts end in `fast-source-store-inline-{1,2,3}.stat`; broad artifacts end
in `after-fast-source-store-inline.csv`. Do not combine Fast allocation reuse
with forced store inlining again without separating those effects first.

Rejected isolated Fast source-sequence allocation reuse follow-up: after
removing forced Fast inlining, `FastMatchState` alone retained and recycled its
`Vec<StoredSequence>` across normal and external-dictionary block preparation.
Lifecycle tests proved allocation identity across compressed-to-raw and two
external-dictionary blocks; 704 library tests, strict validation, focused
bytes, and all 1,095 broad rows passed. Lunx measured level-1/3/5/8/16 medians
of `687,923,574`, `1,242,969,376`, `3,494,719,667`, `4,391,633,788`, and
`7,664,238,379` instructions. The intended Fast path regressed tightly by
`1.079%`; other levels were neutral. The state field, take/recycle path, and
tests were removed. Counter artifacts end in
`fast-sequence-reuse-{1,2,3}.stat`; broad artifacts end in
`after-fast-sequence-reuse.csv`. Fast's fresh per-block sequence vector is a
better generated/allocation shape than carrying it through matcher state.

Rejected August 28 persistent packed Huffman-code representation: the complete
Huffman transaction replaced its 8-byte `(u32, u8)` code tuple with one
transparent 8-byte word carrying the low-bit code in bits 0-31 and its length
in bits 32-39. Construction, metrics, weight serialization, compact emission,
and the isolated four-stream codegen crate all consumed the packed cell
directly, so unlike the earlier rejected exact-C experiment there was no
per-block conversion or staging allocation. Exact field/layout and table-log
1-11 reference coverage passed, focused bytes stayed exact, all 1,095 broad
rows were unchanged, and full strict validation passed.

Lunx nevertheless measured level-1/3/5/8/16 medians of `699,259,846`,
`1,261,853,476`, `3,512,800,565`, `4,409,618,512`, and `7,700,086,840`
instructions: regressions of `2.744%`, `1.519%`, `0.516%`, `0.410%`, and
`0.469%` versus the retained DFast-inline baseline. The whole packed
representation was removed; the isolated four-stream symbol returned from
8,004 to its retained 7,459 bytes and focused output remained exact. Counter
artifacts end in `packed-huffman-code-{1,2,3}.stat`; broad artifacts end in
`after-packed-huffman-code.csv`. Keep the tuple: LLVM's separate field loads
beat explicit packed extraction even when conversion is eliminated.

RESOLVED EXPERIMENT CHECKPOINT, August 27, 2026: the strategy-gated packed
Huffman specialization was benchmarked and rejected. Its final separated-loop
shape measured level 1 `758,023,307`, level 3 `1,348,189,833`, level 5
`3,895,652,317`, and level 8 `4,859,256,442` instructions for 20 runs on
`corpus_z000033`. The Fast/DFast gains therefore remained, but Greedy/Lazy
still regressed by `5.52%` and `5.04%` against the retained tuple baselines.
Only the packed-emission config flag, threading, methods, stream loop, and
packed equivalence coverage were removed. The validated Fast
`match4_found()` bounds cleanup and safe overlapping eight-byte Huffman output
stores remain retained.

Post-revert 20-run samples were level 1 `777,539,471`, level 3
`1,360,409,338`, level 5 `3,691,755,908`, and level 8 `4,625,916,596`
instructions, all within `0.003%` of the retained tuple baselines. Output totals
were unchanged at `11,430,500`, `9,977,820`, `9,767,360`, and `9,686,760`
bytes respectively. The release profiler build, focused Huffman stream test,
and full suite passed (`689` library tests passed with `5` ignored, plus `7`
integration tests). Do not retry packed owning code tables without a materially
different generated-code design that cannot perturb Greedy/Lazy.

Latest kept C SeqStore ownership change from August 27, 2026: stateful DFast
and Greedy/Lazy match states now retain the matcher `StoredSequence` vector
after each block has been converted to the entropy-ready `PreparedBlock`.
Normal, ext-dictionary, and attached-dictionary adapters explicitly return the
cleared allocation to the match state, mirroring C's persistent `SeqStore_t`
ownership instead of allocating a new matcher sequence vector for every source
block. Boundary tests prove that the same allocation is reused by a following
contiguous block for both retained strategy families.

The first cross-strategy candidate also enabled reuse for Fast. Luna's
unattended three-sample level-1 gate measured a stable `784.490M` instruction
band, `+0.894%` versus the retained `777.539M` baseline, so only the Fast part
was removed. Post-split 20-run samples on `corpus_z000033` were level 1 median
`777,540,494` (`+0.0001%`), level 3 `1,350,181,549` (`-0.752%`), level 5
`3,584,991,877` (`-2.892%`), and level 8 `4,477,487,511` (`-3.209%`). Output
totals remained `11,430,500`, `9,977,820`, `9,767,360`, and `9,686,760`
bytes. The 73-fixture broad levels 1/3/5/8 matrix is byte-identical to the
pre-change checkpoint; artifact:
`benchmarks/tmp/normal-levels-1-3-5-8-api-after-seqstore-reuse.csv`.
Validation passed 691 library tests with 5 ignored, 7 integration tests,
strict all-target Clippy, formatting, release tool builds, and
`git diff --check`. Keep DFast and Greedy/Lazy reuse; do not restore Fast reuse
without a different ownership/codegen shape and new level-1 evidence.

Rejected follow-up from August 27, 2026: moving the matcher sequence allocation
out of the match states and into a separate `FrameBlockState` workspace did
avoid changing `FastMatchState` itself. It improved the focused level-1 median
to `775,915,797` instructions (`-0.209%`) and left level 3 effectively flat at
`1,350,358,896` (`+0.013%`), but the added frame-state ownership/layout
regressed level 5 to `3,682,010,420` (`+2.706%`) and level 8 to
`4,615,670,469` (`+3.086%`). All output totals were unchanged. The candidate
was reverted. Post-revert samples returned to level 1 `777,538,310`, level 3
`1,350,168,399`, level 5 `3,585,003,713`, and level 8 `4,477,487,616`
instructions, all within `0.001%` of the retained bands. Keep the
strategy-local DFast and Greedy/Lazy allocations; do
not add an otherwise-unused shared sequence workspace to `FrameBlockState`
without a generated-code design that isolates Greedy/Lazy.

Latest kept C numeric-offBase and preparation change from August 27, 2026:
matcher `StoredSequence` values now store C's nonzero numeric `offBase`
directly instead of carrying a Rust enum discriminant, reducing the safe Rust
record from 16 to 12 bytes. Entropy-ready `PreparedSequence` similarly uses
zero as the generic-matcher's unset sentinel and a nonzero C `offBase` for the
C-port path, reducing that record from 20 to 16 bytes. `RepeatOffsets` has
direct numeric resolve/update operations mirroring `ZSTD_updateRep()`, so the
hot preparation loop no longer reconstructs and rematches an enum. Fast and
DFast share one C-SeqStore preparation helper; Greedy/Lazy/optimal deliberately
keep their separate local generated loop and existing exact-capacity policy.

The first fully shared helper shape was rejected before retaining this split.
It improved levels 1/3 by `0.380%`/`0.840%`, but regressed levels 5/8 by
`2.341%`/`2.879%`. Restoring the Greedy-family loop removed that codegen
regression. Final 20-run medians on `corpus_z000033` are level 1
`771,787,559` (`-0.740%`), level 3 `1,333,940,664` (`-1.203%`), level 5
`3,577,062,412` (`-0.221%`), and level 8 `4,472,945,519` (`-0.101%`) versus
the retained SeqStore baselines. The level-16 guard median is
`7,730,418,599`, `-0.376%` versus the prior checkpoint midpoint. Output totals
are unchanged at all five levels. The full 511-row normal matrix is
byte-identical to the previous broad checkpoint; artifact:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-numeric-offbase.csv`.
Validation passed 691 library tests with 5 ignored, 7 integration tests,
strict all-target Clippy, formatting, release profiler/benchmark builds, and
`git diff --check`.

Rejected follow-up from August 27, 2026: moving Fast and DFast literal copies
into the matcher at the `ZSTD_storeSeq()` boundary preserved every focused
output total and all 511 broad fixture/level byte rows, but decisively
regressed the generated hot paths. Luna's 20-run medians were level 1
`818,887,957` (`+6.103%`), level 3 `1,366,367,235` (`+2.431%`), level 5
`3,673,997,531` (`+2.710%`), and level 8 `4,610,191,074` (`+3.068%`) versus
the retained numeric-offBase checkpoint. The matcher-owned literal vectors
and adapter handoff were removed. Keep the post-match preparation scan: C's
preallocated `SeqStore_t` plus wild-copy storage does not transfer to repeated
safe `Vec::extend_from_slice()` calls in these Rust matcher loops without a
materially different storage representation. The rejected broad artifact is
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-fast-dfast-literal-store.csv`.
The post-revert 20-run confirmation was level 1 `773,452,650`, level 3
`1,327,069,624`, level 5 `3,577,060,492`, and level 8 `4,472,955,558`
instructions. Levels 5/8 are within `0.0003%` of their retained medians, the
lower-level variation is far below the rejected candidate, and output totals
returned exactly to `11,430,500`, `9,977,820`, `9,767,360`, and `9,686,760`.

Rejected follow-up from August 27, 2026: the normal block compressor was
generalized over a compact three-field sequence-value interface so C-port
`PreparedSequence` records could feed table selection and FSE emission
directly, avoiding the duplicate encoded `Sequence` vector. Exact bytes and
repeat-history behavior were proven by a dedicated equivalence test, and all
511 broad rows were unchanged. Luna measured real gains at level 1
`762,212,089` (`-1.241%`), level 3 `1,317,142,498` (`-1.259%`), and level 16
`7,708,640,700` (`-0.282%`), but the generic shared generated function
regressed level 5 to `3,653,087,399` (`+2.125%`) and level 8 to
`4,591,122,462` (`+2.642%`). The generic interface and direct specialization
were removed. This repeats the earlier shared-helper result: Fast/DFast can
benefit from consuming numeric offBase directly, but that boundary must not
change the Greedy/Lazy monomorphization. The rejected broad artifact is
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-direct-c-sequences.csv`.
Post-revert 20-run instructions were level 1 `773,452,532`, level 3
`1,327,069,559`, level 5 `3,577,071,075`, and level 8 `4,472,955,283`, with
exact expected output totals. Levels 5/8 returned within normal variance of
the retained numeric-offBase bands.

Rejected follow-up from August 27, 2026: the promising direct numeric-offBase
entropy path was rebuilt as a completely separate Fast/DFast block compressor.
It had its own one-pass LL/ML/OF counting and history update, count-based table
selection, direct FSE stream loops, and block-emission entry point; the shared
generic compressor and Greedy/Lazy call sites were left source-identical. Two
equivalence tests covered predefined and multi-block compressed tables, and all
511 broad rows were unchanged. Luna still measured level 5
`3,674,261,984` (`+2.717%`) and level 8 `4,610,612,244` (`+3.078%`), despite
the expected level-1/3 gains to `765,905,125` (`-0.762%`) and
`1,322,981,644` (`-0.822%`); level 16 was neutral at `7,727,491,366`
(`-0.038%`). The entire isolated implementation was removed. This proves
source call-graph separation alone is insufficient: the added entropy code
perturbs Greedy/Lazy through the current codegen-unit/binary-layout boundary.
Do not retry a large parallel encoder without first isolating compilation or
demonstrating a smaller addition that leaves level-5/8 hardware instructions
stable. Rejected broad artifact:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-isolated-c-sequences.csv`.
Post-revert levels 1/3/5/8 measured `773,451,904`, `1,327,059,481`,
`3,577,070,245`, and `4,472,944,974` instructions with exact expected output
totals. Levels 5/8 returned within `0.0003%` of their retained medians.

Latest kept Greedy/Lazy C-parity change from August 27, 2026: when a row-hash
match exits lazy-skipping mode, the matcher now refills its eight-entry hash
cache before clearing the flag, matching `ZSTD_compressBlock_lazy_generic()`.
The 511-row broad matrix changes only six level-5/8 rows, all favorably:
level-5 aggregate Rust-minus-C bytes improve from `-714` to `-752`, level 8
from `-203` to `-214`, and three formerly positive rows become exact. Luna's
isolated three-sample 20-run medians are level 1 `773,455,753` (`+0.216%`
against the retained median), level 3 `1,327,071,957` (`-0.515%`), level 5
`3,570,566,874` (`-0.182%`), level 8 `4,466,926,964` (`-0.135%`), and level
16 `7,730,421,172` (flat). The affected levels therefore improve both size
and instructions. Artifact:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-row-cache-refill.csv`.
Validation passed 691 library tests with 5 ignored, 7 integration tests,
strict all-target Clippy, tool tests/Clippy, release builds, formatting, broad
decode comparison, and `git diff --check`.

Rejected companion from that checkpoint: const-specializing the lazy parser's
depth 0/1/2 wrappers reproduced essentially the entire earlier combined
regression even with the refill removed. Its medians were level 1
`774,002,339` (`+0.287%`), level 3 `1,337,865,735` (`+0.294%`), level 5
`3,627,367,430` (`+1.406%`), level 8 `4,573,468,867` (`+2.247%`), and level
16 `7,730,871,522` (flat), with unchanged bytes. It was removed. The earlier
combined artifact remains
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-lazy-specialization.csv`,
but its CPU loss is attributable to the Rust monomorphization, not the retained
cache refill. Keep runtime `depth` dispatch unless a materially different
generated-code boundary is demonstrated.

Rejected row-candidate pipeline follow-up from August 27, 2026: Rust ported
C's full two-phase `ZSTD_RowFindBestMatch()` shape for both active and attached
rows, collecting up to 64 matching indexes into initialized fixed stack
buffers, prefetching their source positions, inserting the current position,
and then scoring the buffered candidates. All focused bytes and all 511 broad
rows were identical to the retained row-refill checkpoint, and full validation
passed before measurement. Luna nevertheless measured severe 20-run
instruction regressions at the affected levels: level 5 `4,102,489,191`
(`+14.897%`) and level 8 `5,149,129,403` (`+15.272%`). Level 3 also regressed
`0.729%`; levels 1/16 were neutral. The buffers and candidate-source
prefetches were removed. Rejected artifact:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-buffered-row-candidates.csv`.
Post-revert levels 5/8 returned to `3,570,557,332` and `4,466,927,616`
instructions with the retained smaller outputs. Keep direct candidate scoring
under the current safe representation; C's uninitialized stack buffer and
prefetch pipeline does not transfer efficiently to initialized Rust arrays.

Rejected Greedy/Lazy direct-destination follow-up from August 27, 2026: all
normal no-dictionary, streaming ext-dictionary, loaded-dictionary, and
attached-dictionary frame paths temporarily moved the existing frame `Vec<u8>`
through block encoding so accepted blocks appended directly, matching C's
caller-owned destination and removing each block-local allocation plus final
copy. Target mode retained isolated candidate buffers. A boundary test proved
the frame prefix and allocation were preserved; the full 511-row normal matrix
was byte-identical, a dictionary levels-3/5/8 matrix decoded successfully, and
full validation passed. Luna still measured level 5 `3,659,043,219`
(`+2.478%`) and level 8 `4,596,381,301` (`+2.898%`) instructions; level 3
regressed `0.732%`, while levels 1/16 were neutral. The ownership variants and
test were removed. Rejected artifacts:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-greedy-direct-output.csv`
and
`benchmarks/tmp/dictionary-levels-3-5-8-api-after-greedy-direct-output.csv`.
Post-revert levels 5/8 returned to `3,570,567,157` and `4,466,936,269`
instructions. Keep Greedy's block-local encoded buffer and final frame copy;
moving the large frame allocation through the hot encoder harms generated code
more than the removed allocation/copy saves.

Rejected strategy-local prepared-workspace follow-up from August 28, 2026:
DFast and Greedy/Lazy match states temporarily retained the derived literal
and `PreparedSequence` vectors across blocks, completing the C `SeqStore_t`
ownership loop for normal, ext-dictionary, attached-dictionary, and target
exits. Allocation-reuse tests passed, all 511 normal rows were byte-identical,
prepared-CDict and target-2048 comparisons were unchanged, and full tests,
strict Clippy, formatting, and release builds passed. Luna measured stable
20-run medians of `776,167,107`, `1,341,320,753`, `3,669,881,436`,
`4,609,168,614`, and `7,731,471,263` instructions at levels 1, 3, 5, 8, and
16. Against the retained row-cache checkpoint, levels 3/5/8 regressed by
`1.074%`/`2.781%`/`3.184%`; level 16 was neutral. The prepared workspaces and
tests were removed. Rejected artifacts:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-prepared-workspace.csv`,
`benchmarks/tmp/prepared-cdict-levels-3-5-8-api-after-prepared-workspace.csv`,
and
`benchmarks/tmp/target-2048-levels-1-3-5-8-api-after-prepared-workspace.csv`.
Post-revert levels 5/8 returned to `3,570,557,746` and `4,466,928,765`
instructions. Keep fresh derived prepared vectors for DFast and Greedy/Lazy;
the retained matcher-owned `StoredSequence` reuse remains beneficial.

Rejected complete Fast virtual-index port from August 28, 2026: unlike the
earlier partial zero-sentinel experiment, every Fast hash-table producer and
consumer was converted coherently to C's `source index + 2` representation.
Zero became invalid, normal/prefix/CDict/ext-dictionary writers stored virtual
indexes, and consumers compared virtual window bounds before decoding for
source access. All 73 normal level-1, prepared-CDict level-1, and target-2048
level-1 rows were byte-stable; full tests and strict validation passed. Luna
measured level-1 samples `837,184,677`, `837,181,051`, and `837,184,932`
instructions, a median `+8.240%` regression versus retained. Levels 3/5/8
also regressed `0.612%`/`2.675%`/`3.060%`, while level 16 was neutral. The
complete representation was removed. Rejected artifacts:
`benchmarks/tmp/normal-level-1-api-after-fast-virtual-index.csv`,
`benchmarks/tmp/prepared-cdict-level-1-api-after-fast-virtual-index.csv`, and
`benchmarks/tmp/target-2048-level-1-api-after-fast-virtual-index.csv`. Keep
physical Fast indexes with `u32::MAX` invalid; the conversion arithmetic and
changed generated shape cost substantially more than its sentinel simplification.

Latest kept shared Huffman emission change from August 28, 2026: normal
256-symbol code vectors are converted once per stream to a safe
`&[(u32, u8); 256]` view and emitted by a separate hot loop. This mirrors C's
fixed `HUF_CElt CTable[256]`, proving every `u8` lookup statically in bounds
without padding or moving the compact owning `Vec`. Full dictionaries and
small direct tables retain a separately generated checked fallback; byte-
equivalence tests cover both loops. Luna's stable 20-run medians at levels
1/3/5/8/16 are `730,974,166`, `1,307,142,527`, `3,543,052,789`,
`4,438,895,995`, and `7,708,588,431` instructions, improvements of
`5.492%`, `1.502%`, `0.771%`, `0.628%`, and `0.282%` versus the retained
row-cache checkpoint. The paired level-1 80-run sample is Rust
`2,922,990,467` instructions, `1,138,756,110` cycles, `448,586,921`
branches, and `10,546,338` branch misses versus C API `2,002,999,839`,
`708,671,277`, `194,367,009`, and `6,238,719`; instructions remain about
`45.93%` above C, while cycles are noisier. All 511 normal rows, 292 prepared-
CDict rows at levels 1/3/5/8, and 292 target-2048 rows are byte-identical to
the preceding checkpoint. Validation passed 692 library tests with 5 ignored,
7 integration tests, strict all-target Clippy, formatting, release builds, and
`git diff --check`. Artifacts:
`benchmarks/tmp/perf-z000033-l1-rust-after-fixed-huffman-code-view.stat`,
`benchmarks/tmp/perf-z000033-l1-c-api-after-fixed-huffman-code-view.stat`,
`benchmarks/tmp/perf-instructions-z000033-l1-after-fixed-huffman-code-view.data`,
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-fixed-huffman-code-view.csv`,
`benchmarks/tmp/prepared-cdict-levels-1-3-5-8-api-after-fixed-huffman-code-view.csv`,
and
`benchmarks/tmp/target-2048-levels-1-3-5-8-api-after-fixed-huffman-code-view.csv`.

Rejected paired Huffman-container follow-up from August 28, 2026: the full
fixed-table path was expanded to C's table-log-specialized `K=5..9` joins and
two independent bit containers per paired batch. The global form preserved all
bytes and improved levels 1/3/16 by `4.584%`/`1.771%`/`0.299%`, but regressed
level 5 by a stable `0.824%` (`3,572,262,660` median), so Luna recommended
REVERT. A final strategy-isolated form retained source-distinct single-
container `encode/encode4x` methods for Greedy/Lazy/optimal and exposed the
paired methods only through a Fast/DFast configuration flag. Despite never
selecting the new methods, levels 5/8 then regressed much more severely by
`6.328%`/`4.702%`, to `3,767,250,613` and `4,647,624,240`; levels 1/3 kept
their gains and level 16 was neutral. Both paired forms, their configuration
threading, and tests were removed. Rejected broad artifacts:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-paired-huffman-containers.csv`
and
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-fast-paired-huffman-containers.csv`.
Keep the accepted fixed-array lookup with its single-container loop. Extra
monomorphized Huffman emission methods materially perturb Greedy/Lazy generated
code even when runtime strategy gating makes them unreachable there.

Rejected isolated paired-Huffman boundary from August 28, 2026: fresh retained
level-1 instruction profiles attributed about `1.005B` Rust instructions to the
Fast matcher versus about `1.128B` in C, but about `684M` Rust instructions to
the fixed Huffman stream path versus roughly `310M` for C's complete Huffman
compression. The candidate moved the fixed 256-code stream loop behind a
non-generic `#[inline(never)]` boundary (a standalone `0x455`-byte release
symbol), packed two reverse batches independently, and then merged and flushed
them in stream order. It preserved exact output across all 511 normal, 292
prepared-CDict, and 292 target-2048 rows; 636 library tests, 7 integration
tests, strict Clippy, formatting, release builds, and whitespace checks passed.
Lunx nevertheless measured level-1/3/5/8/16 medians of `754,467,511`,
`1,323,589,319`, `3,880,186,323`, `4,834,607,353`, and `7,717,300,844`
instructions: regressions of `3.214%`, `1.258%`, `9.515%`, `8.915%`, and
`0.113%`. The boundary and paired helpers were removed; post-revert level
1/5/8 samples returned to `730,974,257`, `3,543,043,799`, and `4,438,895,350`
instructions with exact bytes. Profile artifacts:
`benchmarks/tmp/perf-instructions-z000033-l1-retained-aug28.data` and
`benchmarks/tmp/perf-instructions-z000033-l1-c-api-retained-aug28.data`.
Rejected broad artifacts end in `after-isolated-paired-huffman.csv`; post-revert
counter artifacts end in `after-isolated-paired-huffman-revert.stat`. A normal
Rust hot-call boundary is not code-layout isolation: any further paired-stream
attempt needs a separately compiled artifact with explicit ABI/placement
control, or must improve the retained inlined loop without adding a boundary.

Latest kept cross-crate paired-Huffman follow-up from August 28, 2026: the
entire fixed-table four-stream transaction was moved into the dedicated no-std
`ruzstd-huff0-codegen` crate, including C's table-log-specific `kUnroll=5..9`
loops, two independently packed batches, safe overlapping stores, and direct
jump-table sizes. Compact tables retain the old checked path. Unlike the
ordinary `#[inline(never)]` function above, this produces one separately
compiled `0x1ea4`-byte release symbol and improved every measured strategy.
Lunx medians at levels 1/3/5/8/16 were `697,332,385`, `1,273,481,236`,
`3,520,660,311`, `4,416,116,638`, and `7,688,020,879`: `-4.602%`, `-2.575%`,
`-0.632%`, `-0.513%`, and `-0.267%`. Focused bytes are exact. All 1,065 broad
rows whose source inputs were unchanged are byte-identical; the remaining 30
rows cover the two Cargo manifests edited to add the crate. The specialized
table-log 1-11 reference test and full validation pass. Artifacts:
`benchmarks/tmp/level-{1,3,5,8,16}-cross-crate-paired-huffman.stat`, plus the
normal, target-2048, and prepared-CDict matrices ending in
`after-cross-crate-paired-huffman.csv`. KEEP the dedicated crate boundary.

Rejected exact C Huffman inner-state follow-up from August 28, 2026: inside
the retained codegen crate, the candidate converted the tuple table to C's
packed high-bit `HUF_CElt` representation and ported `HUF_CStream_t`'s two
containers, fast/slow last-symbol rules, merges, flushes, and close marker. All
bytes and validation gates passed, but Lunx level-1/3/5/8/16 medians were
`699,050,097`, `1,278,700,651`, `3,525,494,131`, `4,421,012,144`, and
`7,694,860,827`: regressions of `0.246%`, `0.410%`, `0.137%`, `0.111%`, and
`0.089%`. The inner port was removed; level 1 returned to `697,332,163`.
Artifacts end in `exact-c-huffman-stream.stat` and
`after-exact-c-huffman-stream.csv`; post-revert:
`benchmarks/tmp/perf-z000033-l1-after-exact-c-huffman-stream-revert.stat`.
Preserve the cross-crate boundary and its low-bit paired batches; do not assume
C's packed high-bit state is cheaper after safe Rust table conversion.

Latest kept one-reservation four-stream follow-up from August 28, 2026: the
retained isolated low-bit emitter now initializes its complete four-stream
worst-case destination once, advances one absolute cursor through all streams,
writes jump-table sizes directly, and truncates once at the end. This removes
four independent resize/truncate transactions while retaining safe overlapping
stores. All 1,095 broad rows are exact and full validation passes. Lunx
level-1/3/5/8/16 medians are `695,070,704`, `1,271,810,296`, `3,517,211,158`,
`4,412,570,416`, and `7,686,836,280`: improvements of `0.324%`, `0.131%`,
`0.098%`, `0.080%`, and `0.015%`. The release symbol shrank from `0x1ea4` to
`0x1d23`. Artifacts end in `one-reservation-huffman.stat` and
`after-one-reservation-huffman.csv`. KEEP the single-reservation cursor.

Rejected isolated all-table sequence-codegen follow-up from August 28, 2026:
the complete hot all-table `ZSTD_encodeSequences_body()` path moved into a
dedicated no-std crate with zero-copy 12-byte `Sequence` values and borrowed
compact FSE tables. It included length-code conversion, all three FSE states,
batched state/extra bits, flushes, and the end marker. Exact reference coverage
spanned 280 sequences and every LL/ML range; full validation and unchanged
broad rows passed. Lunx level-1/3/5/8/16 medians were `691,984,284`,
`1,278,011,853`, `3,608,721,942`, `4,545,639,771`, and `7,681,890,857`:
`-0.444%`, `+0.488%`, `+2.602%`, `+3.016%`, and `-0.064%`. The crate and all
integration changes were removed; post-revert level 1 returned to
`695,070,878`. Artifacts end in `isolated-sequence-codegen.stat` and
`after-isolated-sequence-codegen.csv`; post-revert artifact:
`benchmarks/tmp/perf-z000033-l1-after-isolated-sequence-codegen-revert.stat`.
Retain the local sequence emitter; do not generalize the successful isolated
Huffman pattern to shared FSE/Sequence core types without stronger layout
control.

Rejected four-stream Huffman output-boundary follow-up from August 28, 2026:
fresh paired instruction profiles showed Rust's Fast matcher already below C
in absolute work (about `1.01B` versus `1.06B` instructions for 80 runs), but
Huffman stream emission at roughly `704M` versus C's `320M`. The candidate
therefore ported `HUF_compress4X_usingCTable_internal()` as one aligned output
transaction: it reserved the six-byte jump table once, emitted all four
streams into that destination, and wrote their sizes directly, while retaining
the accepted single-container inner stream loop. A reference test proved exact
bytes against four independent stream writes for table logs 1 through 11 and
the compact-table fallback. All 511 normal, 292 prepared-CDict, and 292
target-2048 broad rows were byte-identical, and full validation passed. Lunx
measured level-1/3/5/8/16 medians of `724,110,786`, `1,299,084,797`,
`3,632,255,920`, `4,569,425,861`, and `7,689,874,882` instructions: changes of
`-0.939%`, `-0.616%`, `+2.518%`, `+2.941%`, and `-0.243%`. The boundary and
its test were removed. Post-revert levels 1/5/8 returned to `730,975,990`,
`3,543,053,399`, and `4,438,893,929` instructions with exact bytes. Rejected
artifacts:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-huffman-four-stream-boundary.csv`,
`benchmarks/tmp/prepared-cdict-levels-1-3-5-8-api-after-huffman-four-stream-boundary.csv`,
and
`benchmarks/tmp/target-2048-levels-1-3-5-8-api-after-huffman-four-stream-boundary.csv`.
Keep the four independent aligned stream calls and generic jump-table rewrite;
grouping only the outer C boundary still moves Greedy/Lazy code enough to erase
the real Huffman savings seen at Fast/DFast and optimal levels.

Rejected fused sequence-code statistics follow-up from August 28, 2026: normal
table selection, post-split estimation, and superblock table construction
shared one pass that collected LL/ML/OF counts, extra-bit totals, and terminal
codes. This did not change the `Sequence` record layout, and all 511 normal,
292 prepared-CDict, and 292 target-2048 rows were byte-identical to the fixed-
Huffman baseline. Full tests, strict Clippy, formatting, and release tool builds
also passed. Lunx measured level-1/3/5/8/16 medians of `730,940,662`,
`1,297,499,013`, `3,640,048,235`, `4,577,120,977`, and `7,696,608,802`
instructions: changes of `-0.005%`, `-0.738%`, `+2.738%`, `+3.114%`, and
`-0.155%`. The shared statistics type and reuse paths were removed. Post-revert
levels 1/5/8 returned to `730,973,862`, `3,543,045,884`, and `4,438,894,367`
instructions with exact expected bytes. Rejected artifacts:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-fused-sequence-stats.csv`,
`benchmarks/tmp/prepared-cdict-levels-1-3-5-8-api-after-fused-sequence-stats.csv`,
and
`benchmarks/tmp/target-2048-levels-1-3-5-8-api-after-fused-sequence-stats.csv`.
Retain the independent normal table-selection scans and estimator-local fused
materialize-then-count pass. Even without enlarging the sequence record, the
shared call and code-layout change again costs materially at Greedy/Lazy.

Rejected C-shaped Huffman remainder-join follow-up from August 28, 2026: a
fresh retained level-1 instruction profile attributed `25.17%` of Rust's work
to the fixed-table Huffman stream closure. The candidate replaced
`rchunks()` with C's join-to-unroll cadence: emit `srcSize % kUnroll` symbols
first, then consume exact `kUnroll` batches with the retained safe overlapping
word stores. It changed only `huff0_encoder/stream.rs`, added no emission
variants, and preserved the fixed tuple table and sequence records. All 511
normal, 292 prepared-CDict, and 292 target-2048 rows were byte-identical; 636
library tests, 7 integration tests, strict Clippy, formatting, and release
builds passed. Lunx nevertheless measured level-1/3/5/8/16 medians of
`771,231,436`, `1,336,256,618`, `3,570,200,508`, `4,466,460,984`, and
`7,728,090,602` instructions: regressions of `5.507%`, `2.227%`, `0.766%`,
`0.621%`, and `0.253%`. The loop was removed. Post-revert levels 1/5/8
returned to `730,974,191`, `3,543,045,946`, and `4,438,894,296` instructions
with exact expected bytes. Rejected artifacts:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-c-huffman-remainder-join.csv`,
`benchmarks/tmp/prepared-cdict-levels-1-3-5-8-api-after-c-huffman-remainder-join.csv`,
and
`benchmarks/tmp/target-2048-levels-1-3-5-8-api-after-c-huffman-remainder-join.csv`.
Keep the current `rchunks()` grouping: unlike C's fixed `kUnroll` template,
Rust's runtime batch width generates a substantially worse remainder/exact-loop
shape even though it removes the apparent per-batch minimum.

Rejected compact sequence-code view follow-up from August 28, 2026: normal
compression ported C's separate `llCode`/`mlCode`/`ofCode` arrays as one
three-byte Rust record, deliberately leaving the existing 12-byte entropy
sequence and all parser/match-store records unchanged. One forward conversion
pass fed both non-exact table selection and reverse sequence-FSE emission;
extra bits were recovered exactly like C from the low bits of `litLength`,
`mlBase`, and `offBase`, using safe 256-entry LL/ML bit-count tables. Exhaustive
length/offset equivalence coverage passed, as did 637 library tests, 7
integration tests, strict Clippy, formatting, release builds, and all 511
normal, 292 prepared-CDict, and 292 target-2048 byte rows. Lunx measured
level-1/3/5/8/16 medians of `738,957,269`, `1,323,508,161`, `3,656,251,400`,
`4,591,748,913`, and `7,718,439,297` instructions: regressions of `1.092%`,
`1.252%`, `3.195%`, `3.443%`, and `0.128%`. The code view, widened bit tables,
selection entry point, and emission overload were removed. Post-revert levels
1/5/8 returned to `730,975,540`, `3,543,052,962`, and `4,438,896,398`
instructions with exact expected bytes. Rejected artifacts:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-compact-sequence-code-view.csv`,
`benchmarks/tmp/prepared-cdict-levels-1-3-5-8-api-after-compact-sequence-code-view.csv`,
and
`benchmarks/tmp/target-2048-levels-1-3-5-8-api-after-compact-sequence-code-view.csv`.
Do not retry separately allocated code arrays: preserving the parser record
avoids the prior stored-record penalty, but the extra allocation/traversal and
larger emission/selection surface still lose at every strategy.

Rejected compact FSE symbol-transform follow-up from August 28, 2026: the
sequence encoder's 12-byte `SymbolTransform` was reduced in place to C's
8-byte footprint by storing the bounded probability and `deltaFindState` as
`i16`. The existing single allocation and all call boundaries were preserved.
All 511 normal, 292 prepared-CDict, and 292 target-2048 rows were byte-
identical; 637 library tests, 7 integration tests, strict Clippy, formatting,
and release builds passed. Lunx measured level-1/3/5/8/16 medians of
`730,324,966`, `1,297,120,681`, `3,639,699,436`, `4,576,949,891`, and
`7,707,862,341` instructions: changes of `-0.089%`, `-0.767%`, `+2.728%`,
`+3.110%`, and `-0.009%`. The narrower fields and footprint assertion were
removed. Post-revert levels 1/5/8 returned to `730,974,643`, `3,543,053,522`,
and `4,438,895,870` instructions with exact expected bytes. Rejected artifacts:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-compact-fse-symbol-transform.csv`,
`benchmarks/tmp/prepared-cdict-levels-1-3-5-8-api-after-compact-fse-symbol-transform.csv`,
and
`benchmarks/tmp/target-2048-levels-1-3-5-8-api-after-compact-fse-symbol-transform.csv`.
Do not retry field-width packing alone. C omits the stored probability rather
than narrowing it; a future attempt needs an architectural table-construction
change that writes NCount before discarding probabilities, or a genuinely
isolated table type. Even this in-place footprint change perturbed Greedy/Lazy
enough to fail the cross-level gate.

Rejected pre-serialized NCount/FSE-table separation from August 28, 2026: the
next candidate ported the larger C ownership boundary instead of narrowing
fields. It serialized the exact normalized-count bytes before table
construction, stored those bytes separately, and made the hot
`SymbolTransform` exactly C's two 32-bit words. Normalized probability
magnitudes used by cost selection were recovered from `deltaNbBits`; the
otherwise-lost `-1` versus `1` distinction remained exact in the serialized
description. Dedicated tests proved the eight-byte transform and exact header
bytes, all 511 normal, 292 prepared-CDict, and 292 target-2048 rows were byte-
identical, and 638 library tests, 7 integration tests, strict Clippy,
formatting, release builds, and `git diff --check` passed. Lunx measured
level-1/3/5/8/16 medians of `731,314,837`, `1,311,365,543`, `3,547,231,468`,
`4,440,781,615`, and `7,731,937,466` instructions: regressions of `0.047%`,
`0.323%`, `0.118%`, `0.042%`, and `0.303%`. With no compensating level, the
whole candidate was removed. Post-revert levels 1/5/8 returned to
`730,975,624`, `3,543,053,214`, and `4,438,903,481` instructions with exact
expected bytes. Rejected artifacts:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-preserialized-fse-ncount.csv`,
`benchmarks/tmp/prepared-cdict-levels-1-3-5-8-api-after-preserialized-fse-ncount.csv`,
and
`benchmarks/tmp/target-2048-levels-1-3-5-8-api-after-preserialized-fse-ncount.csv`.
The remaining faithful design would have to return a transient serialized
header beside the final table and consume it before the table enters repeat
history, without adding persistent `Vec` state or widening the shared mode.
Do not retry storing the pre-serialized header inside every `FSETable`.

Rejected transient `BuiltFSETable` lifecycle follow-up from August 28, 2026:
the full normalization, sequence selection/estimation, fresh-table emission,
superblock, Huffman-weight FSE, and repeat-history path was ported so normalized
counts lived only in a transient build result. The already allocated
normalization `Vec<i32>` was moved rather than copied; `into_table()` discarded
it before installing repeat history, leaving persistent `FSETable` with C's
exact two-word transform. A lifecycle test proved exact NCount bytes, the
eight-byte transform, and probability-cost recovery after discarding counts.
All 511 normal, 292 prepared-CDict, and 292 target-2048 rows were byte-
identical; 637 library tests, 7 integration tests, strict Clippy, formatting,
release builds, and `git diff --check` passed. Lunx measured level-1/3/5/8/16
medians of `730,501,920`, `1,307,895,897`, `3,640,700,873`, `4,575,984,182`,
and `7,709,373,399` instructions: changes of `-0.065%`, `+0.058%`, `+2.756%`,
`+3.088%`, and `+0.010%`. The entire build-result API and caller port were
removed. Post-revert levels 1/5/8 returned to `730,975,343`, `3,543,044,405`,
and `4,438,903,641` instructions with exact expected bytes. Rejected artifacts:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-transient-fse-build-result.csv`,
`benchmarks/tmp/prepared-cdict-levels-1-3-5-8-api-after-transient-fse-build-result.csv`,
and
`benchmarks/tmp/target-2048-levels-1-3-5-8-api-after-transient-fse-build-result.csv`.
This closes both obvious ownership shapes: neither persistent serialized bytes
nor a transient count-bearing `FseTableMode` passes the cross-level gate. Do
not retry compact FSE transforms through the shared mode enum without a real
compilation/codegen boundary that preserves Greedy/Lazy layout.

Rejected C no-low-probability FSE spread follow-up from August 28, 2026: the
remaining dedicated branch in `FSE_buildCTable_wksp()` was ported as one safe
workspace allocation. For distributions without `-1` counts it built C's
ordered spread buffer and populated table positions two symbols per iteration;
the existing general low-probability walk was unchanged. A reference test
proved exact state-table equivalence across balanced, zero-containing, and
uneven distributions. All 511 normal, 292 prepared-CDict, and 292 target-2048
rows were byte-identical; 637 library tests, 7 integration tests, strict
Clippy, formatting, release builds, and `git diff --check` passed. Lunx
measured level-1/3/5/8/16 medians of `731,424,865`, `1,311,279,392`,
`3,547,463,149`, `4,442,243,745`, and `7,716,142,370` instructions:
regressions of `0.062%`, `0.316%`, `0.124%`, `0.075%`, and `0.098%`. The
branch and reference test were removed. Post-revert levels 1/5/8 returned to
`730,975,706`, `3,543,043,658`, and `4,438,895,910` instructions with exact
expected bytes. Rejected artifacts:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-c-fse-no-lowprob-spread.csv`,
`benchmarks/tmp/prepared-cdict-levels-1-3-5-8-api-after-c-fse-no-lowprob-spread.csv`,
and
`benchmarks/tmp/target-2048-levels-1-3-5-8-api-after-c-fse-no-lowprob-spread.csv`.
C's path relies on caller-owned workspace and unchecked overlapping bulk fills;
the safe owning workspace plus slice fills does not recover that advantage.
Keep the single general spread walk unless a reusable workspace can be isolated
without repeating the already rejected caller-owned FSE workspace shapes.

Rejected sequence-stream writer follow-up from August 28, 2026: the complete
sequence bitstream was moved behind one aligned output boundary and emitted
through a dedicated safe `u64` container with pre-sized overlapping word
stores, mirroring C's `BIT_CStream_t` ownership instead of re-entering the
general `BitWriter` for each FSE and extra-bit batch. A 1,027-sequence
reference test proved exact bytes against the former writer, existing mixed
and RLE mode tests passed, and all 511 broad normal rows were byte-identical;
artifact:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-sequence-aligned-writer.csv`.
Luna nevertheless measured medians of `737,906,778`, `1,320,085,804`,
`3,879,937,733`, `4,832,461,996`, and `7,725,579,125` instructions at levels
1/3/5/8/16: regressions of `0.948%`, `0.990%`, `9.508%`, `8.866%`, and
`0.220%`. The dedicated writer and reference test were removed. Post-revert
samples returned to `730,973,806`, `3,543,043,884`, and `4,438,894,392` at
levels 1/5/8, all within `0.0003%` of the retained fixed-Huffman baselines.
Keep the current batched calls through `BitWriter`; pre-initializing the safe
worst-case sequence output and changing this hot function's layout costs far
more than the general-writer dispatch it removes.

Rejected C length-code arithmetic follow-up from August 28, 2026: large LL/ML
conversion ranges were replaced with C's `highbit` plus base-subtraction
formulas, removing the piecewise range matches without changing any caller or
adding code. Exhaustive coverage through the maximum valid literal and match
lengths, the full suite, strict Clippy, and all 511 broad rows passed; artifact:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-c-length-code-formulas.csv`.
Despite the smaller source and exact bytes, Luna measured medians of
`732,697,611`, `1,310,245,871`, `3,868,153,932`, `4,822,182,961`, and
`7,762,475,753` instructions at levels 1/3/5/8/16. These are regressions of
`0.236%`, `0.237%`, `9.176%`, `8.635%`, and `0.699%`, so the formulas and
expanded test helper were removed. The severe Greedy/Lazy movement from a
small shared-function rewrite confirms that binary placement/layout remains a
first-class acceptance constraint; retain the existing piecewise conversion
until compilation units or linker layout are deliberately isolated.

Rejected stored sequence-code boundary from August 28, 2026: the entropy-ready
sequence record was expanded from the three LL/ML/OF `u32` values to a 16-byte
record carrying the three `u8` codes produced once at the equivalent of C's
`ZSTD_seqToCodes()` boundary. Table selection, estimation, superblock handling,
repeat/RLE checks, and final emission all consumed those stored codes; safe
256-entry LL/ML base-and-bit tables recovered extra bits without hot code
conversion. The candidate passed 637 no-default-feature library tests, strict
all-target Clippy, formatting, release builds, and decode verification. All
511 normal, 292 prepared-CDict, and 292 target-2048 rows were byte-identical
to the fixed-Huffman checkpoint. Luna measured level-1/3/5/8/16 medians of
`717,688,938`, `1,286,820,812`, `3,618,345,406`, `4,556,628,387`, and
`7,702,517,562` instructions. Levels 1/3 improved by `1.817%`/`1.555%` and
level 16 was neutral, but levels 5/8 regressed by `2.125%`/`2.652%`; the
representation, lookup tables, and migrated consumers/tests were removed.
Post-revert samples were `730,976,109`, `3,543,053,555`, and `4,438,894,650`
at levels 1/5/8 with the expected bytes, confirming restoration. Rejected
artifacts:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-stored-sequence-codes.csv`,
`benchmarks/tmp/prepared-cdict-levels-1-3-5-8-api-after-stored-sequence-codes.csv`,
and
`benchmarks/tmp/target-2048-levels-1-3-5-8-api-after-stored-sequence-codes.csv`.
Do not retry a shared owning sequence-record expansion without first isolating
Greedy/Lazy code layout; the one-time conversion gain is real at Fast/DFast,
but the larger shared record and generated-code movement dominate there.

Rejected shared Fast match continuation from August 28, 2026: the three
no-dictionary Fast hit sites were converged onto one C-style `_offset`/`_match`
continuation, removing duplicated backward extension, match storage, hash
filling, and immediate-repcode handling. Focused tests and all validation
passed, and all 511 normal, 292 prepared-CDict, and 292 target-2048 rows were
byte-identical. The active Fast symbol shrank from 3,040 to 2,648 bytes
(`-12.9%`), but Luna measured level-1/3/5/8/16 medians of `742,745,041`,
`1,306,995,768`, `3,639,903,439`, `4,576,942,006`, and `7,699,065,699`
instructions. That is `+1.610%`, `-0.011%`, `+2.734%`, `+3.110%`, and
`-0.124%` versus the retained baselines, so the merged continuation was
removed. Post-revert levels 1/5/8 returned to `730,974,336`,
`3,543,055,559`, and `4,438,894,911` instructions with exact expected bytes.
Rejected artifacts:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-fast-shared-match.csv`,
`benchmarks/tmp/prepared-cdict-levels-1-3-5-8-api-after-fast-shared-match.csv`,
and
`benchmarks/tmp/target-2048-levels-1-3-5-8-api-after-fast-shared-match.csv`.
Preserve the duplicated Rust hit tails: despite the smaller symbol, the merged
live state and code layout cost more at Fast and Greedy/Lazy levels.

Rejected bounded-short FSE normalization workspace from August 28, 2026: the
normal, slow-fallback, sequence-table, Huffman-weight, dictionary-table, and
NCount paths were converted end-to-end from heap `Vec<i32>` normalized counts
to a safe 514-byte value carrying C's bounded 256-entry `short` alphabet plus
its active length. `FSETable` transforms also retained normalized probabilities
as `i16`, and decoded dictionary probabilities were checked and compacted at
the compression boundary. The candidate passed 637 no-default-feature library
tests, 7 doc tests, strict Clippy, formatting, release builds, and decode
verification. All 511 normal, 292 prepared-CDict, and 292 target-2048 rows
were byte-identical to the fixed-Huffman checkpoint. Luna measured level-
1/3/5/8/16 medians of `729,082,780`, `1,307,963,753`, `3,865,683,767`,
`4,821,136,450`, and `7,719,473,803` instructions: changes of `-0.259%`,
`+0.063%`, `+9.106%`, `+8.611%`, and `+0.141%`. The representation was
removed. Post-revert levels 1/5/8 returned to `730,975,348`,
`3,543,045,680`, and `4,438,893,887` instructions with exact expected bytes.
Rejected artifacts:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-fse-short-normalization.csv`,
`benchmarks/tmp/prepared-cdict-levels-1-3-5-8-api-after-fse-short-normalization.csv`,
and
`benchmarks/tmp/target-2048-levels-1-3-5-8-api-after-fse-short-normalization.csv`.
Do not retry a safe by-value full-alphabet normalization array: removing the
small allocations helps Fast, but the initialized fixed workspace and changed
shared generated-code shape cost roughly 9% at Greedy/Lazy. A faithful future
port needs caller-owned storage with stronger layout isolation, not a large
returned value.

Rejected caller-owned sequence-FSE normalization workspace from August 28,
2026: a follow-up isolated C's 256-entry `short` probabilities to sequence
tables only, initialized the workspace once behind shared `FseTables`
ownership, and left Huffman-weight FSE plus the owning `FSETable` layout
unchanged. Normalization, NCount costing, estimate-only transforms, and full
sequence table construction consumed the borrowed active prefix directly. The
candidate passed focused and full tests, strict Clippy, formatting, release
builds, and decode-verified broad comparison; all 511 normal, 292 prepared-
CDict, and 292 target-2048 rows were byte-identical to the fixed-Huffman
checkpoint. Luna measured level-1/3/5/8/16 medians of `731,115,297`,
`1,308,199,224`, `3,641,200,083`, `4,578,993,644`, and `7,710,757,638`
instructions: `+0.019%`, `+0.081%`, `+2.770%`, `+3.156%`, and `+0.028%`
versus retained. The workspace was removed. Post-revert levels 5/8 returned to
`3,543,044,044` and `4,438,903,406` instructions with exact bytes. Candidate
artifacts:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-fse-caller-workspace.csv`,
`benchmarks/tmp/prepared-cdict-levels-1-3-5-8-api-after-fse-caller-workspace.csv`,
and
`benchmarks/tmp/target-2048-levels-1-3-5-8-api-after-fse-caller-workspace.csv`.
Do not retry the fixed short normalization workspace merely by changing its
ownership: even sequence-only reuse preserved the Greedy/Lazy regression.
Retain the compact owning `Vec<i32>` normalization path until a profile points
to a materially different algorithm or a separately compiled codegen boundary.

Current cross-level CPU checkpoint from July 18, 2026: the remaining CPU
problem is now clearly at the low compression levels, not the optimal-parser
levels. A same-fixture, 20-run, user-space-counter sweep on
`corpus_z000033` measured Rust-minus-C instruction gaps of `+80.83%` at level
1, `+66.19%` at level 3, `+70.02%` at level 5, and `+61.60%` at level 8. Rust
was already ahead by `-10.32%`, `-8.96%`, and `-12.71%` at levels 16, 19, and
22 respectively. Compression sizes stayed at or slightly better than C across
the sweep. Do not use the favorable level-16 result as a whole-compressor CPU
parity claim; continue with Fast, DFast, and Greedy/Lazy generated-code work.

Latest kept Fast CPU change from July 18, 2026: `fast.rs::match4_found()` now
relies on the loop and hash-table writer invariants for its four-byte source
bounds instead of repeating release-mode current/match bounds checks on every
probe. It retains Rust's `u32::MAX` invalid table entry, which is the physical-
index equivalent of C's zero entry with `ZSTD_WINDOW_START_INDEX == 2`. No
unsafe indexing was added. Focused level-1 output remains byte-identical to C
at `45,722,000` bytes for 80 runs. Three 20-run Rust samples were
`828,760,766`, `828,754,328`, and `828,761,157` instructions, down from the
pre-change `905,913,382`; branches fell from `154,547,197` to about
`138.439M`. The paired 80-run result is Rust `3,314,147,380` instructions,
`1,204,296,151` cycles, and `553,571,777` branches versus C
`2,002,999,858`, `697,972,653`, and `194,367,030`. The remaining focused
level-1 gaps are about `+65.46%` instructions, `+72.54%` cycles, and
`+184.8%` branches. All 73 broad level-1 Rust byte rows are unchanged from the
prior 511-row checkpoint. Artifacts:
`benchmarks/tmp/perf-z000033-l1-rust-after-fast-match-bounds.stat`,
`benchmarks/tmp/perf-z000033-l1-c-api-after-fast-match-bounds.stat`, and
`benchmarks/tmp/normal-level-1-api-after-fast-match-bounds.csv`. Validation
passed all 694 library tests (689 passed, 5 ignored), 7 integration tests,
strict all-target Clippy, formatting, broad decode comparison, release tool
builds, and `git diff --check`.

Latest kept shared entropy-emission change from July 18, 2026:
`huff0_encoder/stream.rs` now mirrors C's overlapping word-store cadence
without unsafe code. The stream output is safely resized once to its proven
worst-case encoded length plus seven initialized padding bytes. Every batch
then copies one complete eight-byte container into an in-bounds slice, advances
by only the emitted whole bytes, and truncates the padding after the end marker.
This replaces millions of runtime-length 0-7-byte `extend_from_slice()` calls
with direct fixed-width stores. Focused level-1 20-run samples improved from
the retained Fast-bound `828.75M`-`828.76M` band to `777,544,508`,
`777,538,314`, and `777,540,424` instructions. Cycles fell from about
`302M`-`310M` to `274.6M`-`275.8M`, and branches from `138.44M` to
`124.73M`. Paired 80-run counters are Rust `3,109,288,699` instructions,
`1,098,707,379` cycles, and `498,746,893` branches versus C
`2,002,998,832`, `705,189,601`, and `194,366,695`; remaining level-1 gaps are
about `+55.23%`, `+55.80%`, and `+156.60%` respectively. All 511 broad byte
rows are unchanged. The post-change 20-run instruction gaps at levels 1, 3, 5,
8, 16, 19, and 22 are `+55.21%`, `+61.26%`, `+68.25%`, `+60.23%`,
`-10.71%`, `-8.98%`, and `-12.72%`. Artifacts:
`benchmarks/tmp/perf-z000033-l1-rust-after-huffman-overlap-store.stat`,
`benchmarks/tmp/perf-z000033-l1-c-api-after-huffman-overlap-store.stat`, and
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-huffman-overlap-store.csv`.
Validation passed all 694 library tests, 7 integration tests, strict all-target
Clippy, formatting, release profiler/benchmark builds, the full broad decode
comparison, and `git diff --check`.

Rejected Huffman emission shape immediately before the overlapping store:
dispatching each runtime byte count through fixed safe 1-8-byte
`extend_from_slice()` arms reduced focused branches to about `129.09M`, but
cycles regressed consistently to about `329M` and branch misses rose to about
`3.65M`. It was reverted. Keep the single bounded eight-byte overlapping store
unless a different safe output representation has stronger measurements.

Rejected immediately before the retained Fast bounds change: initializing the
Rust Fast table with zero and treating physical source index zero as a valid
match candidate reduced focused level-1 instructions further to about
`809.76M`, but it changed 24 previously stable broad rows and increased the
number of Rust/C differing rows from 2 to 25. C's table is zero-filled because
its first real virtual source index is 2; Rust uses physical indexes beginning
at zero, so accepting zero was not a faithful translation. The experiment was
reverted. Keep an explicit invalid representation unless the whole Fast state
is deliberately moved to C-style virtual indexes.

Latest kept estimator parity change from July 18, 2026: the post-split C-cost
literal estimate now applies C's fast `HUF_optimalTableLog` bound instead of
always allowing table log 11, and `EstimateScratch` reuses the compact Huffman
node workspace across estimates. Focused level-16 output moves from `460,572`
to `460,555` bytes per frame versus C's `460,806`. More importantly, the
source-aligned inspection improves from 17 differing groups with 322 absolute
bytes of difference to 8 groups with 255 absolute bytes. Across the 511-row
broad matrix only `corpus_z000033` and `corpus_z000050` at level 16 change,
by `-17` and `-45` Rust bytes respectively. Inspection artifact:
`benchmarks/tmp/inspect-z000033-l16-after-estimate-fast-log-scratch`.

Rejected follow-up from the same checkpoint: reusing Huffman construction
scratch for emitted blocks preserved bytes but regressed focused 20-run
instructions from the retained `8.276B`-`8.281B` band to roughly
`8.317B`-`8.326B`. Both an explicit frame/parser-context pointer and storage
beside `FseTables` were measured and reverted. Do not retry emitted Huffman
scratch reuse without a different generated-code shape and a fresh profile
signal; estimator-local reuse remains retained.

Rejected estimator follow-ups from the same checkpoint: precomputing six
separate LL/ML/OF code and extra-bit vectors preserved focused bytes but
regressed 20-run instructions to about `8.698B`-`8.704B`. Fusing direct
`PreparedSequence` code counting into the estimator reduced that experiment to
about `8.667B`-`8.676B`, but remained far behind the retained path. Both were
removed. Keep the reusable materialize-then-count path in
`EstimateScratch::sequences`; it restored `9,211,100` bytes and an initial
`8.212B`-`8.213B` instruction band before the compact FSE change.

Latest kept FSE CPU and safety change from July 18, 2026: the encoder now uses
C's compact `stateTable` plus one compression transform per symbol instead of
allocating a `Vec<State>` and direct lookup vector for every symbol. The C
state range is represented explicitly and masked to its low `tableLog` bits at
stream flushes. `FSE_MAX_TABLELOG` is C's default 12, making the safe `u16`
state table sufficient. This removes the old release-only unsafe uninitialized
lookup writes. The implementation is split by role into `fse_encoder.rs` (355
lines), `fse_encoder/normalize.rs` (305), and `fse_encoder/table.rs` (244).
Focused bytes are unchanged. Branch attribution for
`build_table_from_probabilities()` fell from `2.50%` to `0.84%`; the artifact
is `benchmarks/tmp/perf-branches-z000033-l16-rust-after-compact-fse-table.data`.
The paired counters and broad artifact are recorded in the restart checkpoint.
The 511 broad fixture/level rows are byte-identical to the prior estimator
checkpoint. Validation passed all 691 library tests (686 passed, 5 ignored),
7 integration tests, all-target clippy with warnings denied, formatting,
release tool builds, broad decode-verified comparison, and `git diff --check`.

Latest kept Huffman CPU and C-parity change from July 18, 2026: normal C-cost
table construction now keeps depths in the compact 8-byte Huffman nodes,
applies C's `HUF_setMaxHeight()` redistribution directly to those nodes, and
builds canonical codes with C's `nbPerRank`/`valPerRank` passes. It no longer
materializes generic `usize` length and symbol vectors before rebuilding the
same table, and it no longer runs Rust's legacy flat-distribution bypass in
the C-style single-table path. The alternate small/optimal table searches keep
their vector-based builders and legacy gate. No unsafe code was added. An
equivalence test compares the direct builder with the retained vector builder
across table logs 4 through 11. Focused output remains `36,844,400` Rust bytes
versus `36,864,480` C bytes for 80 runs. The current paired counters and broad
artifact are recorded in the restart checkpoint above. The broad 511-row
matrix is byte-identical to the compact-FSE checkpoint. The former 2,000-line
encoder source is now split into real Rust modules by responsibility:
`table.rs` (424 lines), `tree.rs` (420), `lengths.rs` (324), `metrics.rs`
(92), `weights.rs` (128), `tests.rs` (489), `stream.rs` (81), and the 124-line
root. This uses normal private modules rather than textual `include!` files.
Final validation passed 687 tests with 5 ignored plus 7 integration tests,
strict all-target Clippy, formatting, release tool builds, the decode-verified
broad comparison, and `git diff --check`.

Rejected follow-up after the direct Huffman checkpoint: constructing the
temporary owning code `Vec` at a fixed 256 entries and truncating it after
u8-indexed writes preserved bytes but regressed focused 20-run instructions to
`7.857B`-`7.858B` and slightly increased branches versus the retained
`7.851B`-`7.854B` pre-split band. It was reverted; keep the compact runtime-length `Vec`
unless a different safe representation has stronger profile evidence.

Latest kept sequence-emission CPU change from July 18, 2026: the all-table
sequence bitstream path now batches C's OF/ML/LL FSE outputs into one bounded
`u64` write and batches the three extra-bit fields into another. RLE sequences
also write their extra fields as one batch. The FSE batch is at most 26 bits
and the extra-bit batch at most 63 bits, so this stays in safe Rust while
matching C's bit-container accumulation order. Focused bytes and all 511 broad
rows are unchanged. Compared with the direct-Huffman module checkpoint, the
80-run sample removes about `24.4M` instructions and `31.4M` branches. In the
fresh branch profile, `encode_sequences` falls from about `2.03%` to `1.41%`
while total sampled branches fall from about `1.222B` to `1.214B` per 20 runs.
Artifacts:
`benchmarks/tmp/perf-z000033-l16-rust-after-sequence-bit-batching.stat`,
`benchmarks/tmp/perf-branches-z000033-l16-rust-after-sequence-bit-batching.data`,
and
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-sequence-bit-batching.csv`.
Final validation passed 687 tests with 5 ignored plus 7 integration tests,
strict all-target Clippy, formatting, release tool builds, broad decode
comparison, and `git diff --check`.

Latest kept FSE header/normalization changes from July 18, 2026:
`write_normalized_probabilities()` now ports the safe-buffer path of
`FSE_writeNCount_generic()` directly. It uses C's bounded `u32` accumulator,
24-zero and 3-zero run encoding, 16-bit flush cadence, and final partial-byte
flush while appending through `BitWriter::append_aligned_with()`. A test-only
reference serializer proves exact bytes across default and generated tables.
Normalization now also accepts the sequence total already known by its callers,
matching C's `FSE_normalizeCount(..., total, ...)` API and avoiding redundant
count summation in both the normal and slow paths. No unsafe code was added.
Focused output and all 511 broad byte rows are unchanged. Relative to the
sequence-batching checkpoint, the combined 80-run sample removes about `65.8M`
instructions and `19.0M` branches. Artifacts:
`benchmarks/tmp/perf-z000033-l16-rust-after-c-ncount-known-total.stat`,
`benchmarks/tmp/perf-z000033-l16-c-api-after-c-ncount.stat`, and
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-c-ncount-known-total.csv`.
Validation passed 688 library tests with 5 ignored plus 7 integration tests,
strict all-target Clippy, formatting, release tool builds, broad decode
comparison, and `git diff --check`.

Rejected follow-up after the Huffman-weight FSE batching checkpoint: recycling
the estimator's owning Huffman code vector through `HuffmanBuildScratch`
preserved focused bytes but did not produce a hardware-counter win. Three
matched user-space 20-run samples were `7,786,037,762`, `7,786,038,630`, and
`7,786,038,413` instructions versus the retained `7.7839B`-`7.7841B` band.
The 80-run sample was `31,143,997,691` instructions and `4,840,366,937`
branches versus the retained `31,135,722,824` and `4,843,072,260`.
Callgrind improved from `405,210,330` to `404,635,309` Ir, but the real
hardware instruction increase and noisy/worse cycle sample made the tradeoff
indefensible. It was reverted. Keep fresh estimator code-vector allocation
unless a materially different owning representation has stronger hardware
counter evidence. Benchmark hardware events must retain the `:u` suffix;
including kernel events added about 60M unrelated instructions per 20 runs and
initially obscured this A/B.

Latest kept Huffman table-description CPU change from July 18, 2026:
`weights_from_codes()` now mirrors C's `HUF_writeCTable_wksp()`
`bitsToWeight` conversion instead of branching on every code length. Rust uses
a 256-entry safe lookup rather than C's 12-entry unchecked workspace table, so
every possible `u8` index is statically in bounds and LLVM removes the bounds
check without unsafe code. Focused and all 511 broad byte rows are unchanged.
Relative to the FSE-batching checkpoint, the 80-run sample removes about
`39.7M` instructions and `14.0M` branches. The paired checkpoint is Rust
`31,096,039,711` instructions and `4,829,102,596` branches versus C
`34,608,031,325` and `4,309,914,212`; the residual gaps are `-10.15%`
instructions and `+12.05%` branches. Artifacts:
`benchmarks/tmp/perf-z000033-l16-rust-after-bits-to-weight.stat`,
`benchmarks/tmp/perf-z000033-l16-c-api-after-bits-to-weight.stat`, and
`benchmarks/tmp/perf-branches-z000033-l16-rust-after-bits-to-weight.data`,
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-bits-to-weight.csv`.
Validation passed 689 library tests with 5 ignored plus 7 integration tests,
strict all-target Clippy, formatting, release builds, broad decode comparison,
file-size checks, and `git diff --check`.

Rejected follow-up after the bucket-subslice checkpoint: populating zero-count
symbols into the Huffman sort workspace exactly like C, carrying the nonzero
leaf rank separately, and explicitly filling the overlapping parent range with
barrier nodes preserved focused bytes but regressed all matched counters.
Three 20-run samples were `7,776,922,573`, `7,777,256,933`, and
`7,777,255,936` instructions with about `1.20395B` branches, versus the retained
`7.75945B`-`7.75979B` instruction and `1.20131B`-`1.20134B` branch bands. The
compact Rust nonzero workspace was restored; a post-revert sample returned to
`7,759,449,669` instructions and `1,201,306,436` branches. C's zero-symbol
writes plus explicit parent-barrier initialization cost more than Rust's skip
branch, so do not retry full-alphabet workspace population without a materially
different representation.

Rejected parser/match follow-up after the bucket-subslice checkpoint: replacing
the no-dictionary repcode collector's hot `if ll0` split with a safe four-entry
logical offset array (`rep0`, `rep1`, `rep2`, `rep0 - 1`) and three unrolled
indexed probes preserved focused bytes and reduced 20-run branches from the
retained `1.20134B` band to `1,187,833,579`. However, instructions regressed
sharply from about `7.7595B` to `7,941,790,200`. The branch split was restored;
a post-revert sample returned to `7,759,786,992` instructions and
`1,201,342,031` branches. Keep the current predictable branch and unrolled
direct offset accesses. This experiment confirms that reducing Rust's residual
branch count is not itself an acceptance criterion when it increases total CPU
work and the retained Rust path already beats C on instructions and paired
cycles.

Latest kept Huffman canonical-table CPU change from July 18, 2026: the direct
C-style builder's `counts_by_length` and `values_by_length` arrays now cover
all 256 possible `u8` lengths. C trusts its already-validated tree depth when
indexing compact rank arrays; the wider safe Rust arrays make every node/code
length statically in bounds, eliminating the corresponding hot-loop bounds
checks without unsafe indexing. Focused and all 511 broad byte rows are
unchanged. Relative to the bits-to-weight checkpoint, the 80-run sample removes
about `42.8M` instructions and `21.6M` branches. The paired checkpoint is Rust
`31,053,244,659` instructions and `4,807,544,745` branches versus C
`34,608,033,425` and `4,309,914,483`; residual gaps are `-10.27%`
instructions and `+11.55%` branches. Artifacts:
`benchmarks/tmp/perf-z000033-l16-rust-after-rank-u8-tables.stat`,
`benchmarks/tmp/perf-z000033-l16-c-api-after-rank-u8-tables.stat`,
`benchmarks/tmp/perf-branches-z000033-l16-rust-after-rank-u8-tables.data`, and
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-rank-u8-tables.csv`.
Validation passed 689 library tests with 5 ignored plus 7 integration tests,
strict all-target Clippy, formatting, release builds, broad decode comparison,
file-size checks, and `git diff --check`.

Latest kept Huffman tree-sort follow-up from July 18, 2026: the C bucket-sort
walk now iterates the exact bounded rank-position subslice directly instead of
stacking `iter().take().skip()`. The adapter chain compiled into `Take::nth`
state in the hot tree builder, while the subslice compiles to the same simple
ordered bucket walk as C and satisfies strict Clippy without an index-only
range loop. Focused and all 511 broad byte rows are unchanged. Relative to the
rank-table checkpoint, the 80-run sample removes about `16.3M` instructions
and `2.5M` branches. Current paired counters are Rust `31,036,963,180`
instructions and `4,805,010,253` branches versus C `34,608,032,979` and
`4,309,914,383`; residual gaps are `-10.32%` instructions and `+11.49%`
branches. Artifacts:
`benchmarks/tmp/perf-z000033-l16-rust-after-bucket-subslice.stat`,
`benchmarks/tmp/perf-z000033-l16-c-api-after-bucket-subslice.stat`,
`benchmarks/tmp/perf-branches-z000033-l16-rust-after-bucket-subslice.data`, and
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-bucket-subslice.csv`.
Validation passed 689 library tests with 5 ignored plus 7 integration tests,
strict all-target Clippy, formatting, release builds, broad decode comparison,
file-size checks, and `git diff --check`.

Latest kept Huffman-weight FSE CPU change from July 18, 2026:
`FSEEncoder::encode_interleaved()` now follows C's
`FSE_compress_usingCTable_generic()` bit-container cadence more closely. It
batches the two-symbol join, four symbols per main-loop iteration, and the two
final states into bounded safe `u64` writes instead of calling `BitWriter` for
every symbol. The main FSE batch is at most 48 bits because the table log is
bounded at 12. Focused and all 511 broad byte rows are unchanged; the 80-run
sample removes about `14.2M` instructions and `16.6M` branches from the NCount
checkpoint. No unsafe code was added. The FSE source is now split into a
341-line stream/NCount root, 305-line normalization module, 244-line table
module, and 180-line tests module. Artifacts:
`benchmarks/tmp/perf-z000033-l16-rust-after-weight-fse-batching.stat`,
`benchmarks/tmp/perf-z000033-l16-c-api-after-weight-fse-batching.stat`,
`benchmarks/tmp/perf-branches-z000033-l16-rust-after-weight-fse-batching.data`,
`benchmarks/tmp/callgrind-z000033-l16-rust-after-weight-fse-batching.out`, and
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-weight-fse-batching.csv`.
Validation passed 688 library tests with 5 ignored plus 7 integration tests,
strict all-target Clippy, formatting, release tool builds, broad decode
comparison, and `git diff --check`.

Rejected follow-up after that checkpoint: changing Huffman weight-table FSE
encoding to reserve byte zero directly and assign the encoded length after the
payload, instead of using the existing `BitWriter` placeholder plus
`change_bits()`, preserved focused bytes but regressed three 20-run instruction
samples to `7,788,306,605`, `7,788,306,067`, and `7,788,307,416` versus the
retained `7.7874B`-`7.7877B` band. It was reverted; keep the existing
placeholder shape unless a different generated-code reason appears.

Latest kept CPU change from July 18, 2026: optimal-parser dispatch now carries
an `ATTACHED_DICT` const specialization alongside `EXT_DICT` and
`LOADED_DICT`. C generates separate no-dictionary and `dictMatchState`
collectors; Rust previously threaded an `Option` through the inlined normal
binary-tree path, leaving attached-dictionary repcode dispatch, terminal-match
bookkeeping, and the attached-tree call in the focused generated function.
The no-attached specialization compiles those paths out without unsafe code.
Focused output stayed at `9,211,440` bytes for 20 runs and `36,845,760` for
80. Before the change, three 20-run samples were `9,053,674,750`,
`9,053,468,685`, and `9,055,183,072` instructions with about `1.412B`
branches. Afterward they were `8,432,402,931`, `8,428,953,166`, and
`8,430,558,271` instructions with about `1.352B` branches. The same-session
80-run Rust/C comparison is recorded in the current restart checkpoint above.
Broad normal bytes remained level 16 `-253`, level 19 `-174`, and level 22
`-64` versus C across 73 fixtures. All 800 prepared-CDict fixture/level byte
rows were identical to the prior checkpoint. Validation passed formatting,
focused and full tests, all-target clippy with warnings denied, release builds,
broad normal and prepared-dictionary comparisons, and `git diff --check`.

Latest kept entropy-state parity change from July 18, 2026: sequence FSE
history now distinguishes C's `FSE_repeat_check` from `FSE_repeat_valid`.
Freshly emitted compressed tables are check-only, so Fast/DFast no longer use
them through C's dictionary-only repeat shortcut. Full dictionaries mark
LL/ML valid only when every C alphabet symbol has nonzero probability, and
mark OF valid through C's reachable bound
`highbit(dictionary_content_size + 128 KiB)`. The dictionary OF valid state is
downgraded after the first source block in `FrameBlockState`, including raw,
RLE, split, and target-mode source blocks.

This closes the focused normal `corpus_z000050` level-3 gap exactly: Rust and
C both emit `440,380` bytes with identical 12 blocks, literals, 5,506
sequences, and fresh `fse/fse/fse` modes in all 11 compressed blocks. Across
73 real-world fixtures, final normal levels 1-5 are respectively `-1690`,
`-1812`, `-1287`, `-699`, and `-718` aggregate bytes versus C; levels 1-3
have no positive rows. The full levels 1-22 artifact remains smaller in
aggregate at every level. Target-2048 level 1 is now exact across all 73 rows;
level 3 improves from three differing rows and `-1829` aggregate bytes to two
rows and `-82`, with one `+50` row. The 800-row one-block prepared-CDict
matrix is byte-identical to the attached-specialization checkpoint. A new
multi-block dictionary check confirms Rust and C both downgrade OF after an
initial raw source block. Artifacts:
`benchmarks/tmp/inspect-z000050-l3-after-fse-repeat-validity`,
`benchmarks/tmp/normal-levels-1-22-api-after-fse-repeat-validity.csv`,
`benchmarks/tmp/normal-levels-1-5-api-final-fse-repeat-state.csv`,
`benchmarks/tmp/target-2048-levels-1-3-api-final-fse-repeat-state.csv`, and
`benchmarks/tmp/prepared-cdict-realworld-levels-1-3-after-fse-repeat-state.csv`.
Validation passed 690 library tests plus 7 integration tests, all-target
clippy with warnings denied, formatting, release profiler/benchmark builds,
decode-verified broad comparisons, and `git diff --check`.

Latest kept literal-histogram CPU changes from July 18, 2026: normal C-cost
literal compression and post-split estimation no longer allocate or fill four
per-stream 256-bin histograms when only C's combined histogram estimate is
used. Per-stream counts remain enabled for the small-table search and the
legacy exact-stream model. Combined scans of at least 1,500 literals now also
use a safe port of C's four-lane, 16-byte-striped histogram counter, breaking
repeated-symbol dependency chains without unchecked indexing or other unsafe
code. Histogram state and counting live in the logical
`compressed/literals/stats.rs` submodule; `literals.rs` is now 413 lines and
the statistics module is 109 lines. Focused bytes stayed at `9,211,440` for 20
runs and `36,845,760` for 80.
The stream-count removal alone moved the 80-run Rust sample from
`33,431,688,254` to `33,054,538,090` instructions and from `5,349,880,293`
to `5,313,986,809` hardware branches. The final striped-counter samples were
stable near `33.03B` instructions and `5.295B` branches; the paired Rust/C
sample is recorded in the restart checkpoint above. Callgrind clearly measured
the stream-count removal (`436,226,271` to `427,452,551` instructions and
`82,921,791` to `78,168,439` branches), but moved oppositely for the striped
counter (`428,392,552` instructions and `79,216,008` branches) despite three
stable hardware-counter improvements; retain the C-shaped counter but keep
that discrepancy in mind. The broad 511-row byte projection is identical
before and after both changes. Artifacts:
`benchmarks/tmp/perf-z000033-l16-rust-after-c-histogram-stream-skip.data`,
`benchmarks/tmp/perf-z000033-l16-rust-after-c-parallel-histogram.data`,
`benchmarks/tmp/callgrind-z000033-l16-rust-after-c-histogram-stream-skip.out`,
`benchmarks/tmp/callgrind-z000033-l16-rust-after-c-parallel-histogram.out`, and
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-c-parallel-histogram.csv`.
Validation passed focused and full tests, clippy for all `ruzstd` targets with
warnings denied, formatting, release builds, broad byte comparison, and
`git diff --check`.

Latest kept Huffman stream CPU change from July 18, 2026: Huffman emission now
ports C's table-log-sized batching rather than calling the general `BitWriter`
state machine for every literal. The safe stream helper packs 5-56-bit batches
into a local `u64`, flushes complete little-endian bytes, and emits the same
one-bit end marker and zero padding. It uses no unchecked indexing or other
unsafe code. `BitWriter::append_aligned_with()` provides the narrow aligned
output boundary, and the stream implementation lives in the logical
`huff0_encoder/stream.rs` submodule. A regression test compares every emitted
byte with the old per-symbol writer for table logs 1 through 11 and uneven
1,027-byte input.

Focused output stayed at `9,211,440` bytes for 20 runs and `36,845,760` for
80. Relative to the histogram checkpoint, the paired 80-run Rust sample moves
from `5,295,708,237` to `5,270,036,897` hardware branches and from
`215,069,662` to `212,467,075` branch misses. Instructions are effectively
flat (`33,032,905,127` to `33,050,604,598`), while paired cycles are now about
`0.65%` above C. Direct branch-event profiling moves Huffman stream emission
from about `121M` sampled branches in `encode4x` to about `84M` in the batched
aligned path. The same profile establishes that focused parser/tree work
accounts for only about `86M` of the roughly `953M` Rust/C branch excess; most
of the remaining gap is entropy construction, encoding, estimation, and
allocation work. Broad aggregate byte gaps remain unchanged at all 511 sampled
fixture/level rows; the only input-size difference from the prior CSV is the
`repo_bit_writer.rs` fixture changing because this patch edits that source,
and Rust/C remain equal on that row. Artifacts:
`benchmarks/tmp/perf-z000033-l16-rust-after-batched-huffman-stream.data`,
`benchmarks/tmp/perf-branches-z000033-l16-rust-after-batched-huffman.data`, and
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-batched-huffman-stream.csv`.
Validation passed 691 library tests plus 7 integration tests, clippy for all
targets with warnings denied, formatting, release builds, focused and broad
byte comparisons, and `git diff --check`.

Latest kept Huffman table-construction CPU change from July 18, 2026:
`base_code_lengths()` now uses a safe fixed workspace with the same cursor
barriers as C's `HUF_buildCTable_wksp()`. Index zero is a real maximum-count
leaf barrier, parent slots are preallocated with the same
dummy count, and the hot pop operation is a direct leaf/parent count compare
instead of an `Option` dispatcher and signed leaf cursor. Real leaves occupy
`1..=N` and parents `N+1..=2N-1`; no unsafe indexing was introduced. Focused
bytes stayed exactly `9,211,440` for 20 runs and `36,845,760` for 80 runs.
Three focused 20-run samples were `8,326,722,936`, `8,325,783,927`, and
`8,327,392,518` instructions with `1.3233B` to `1.3236B` branches, versus the
preceding `8.332B` instruction and `1.3321B` branch band. The fresh direct
branch profile attributes `3.51%` of `5.223B` branch events to
`base_code_lengths`, down from `3.68%` of `5.262B` at the batched-stream
checkpoint. The broad 511-row normal matrix is byte-identical to the preceding
checkpoint. Validation passed 691 library tests plus 7 integration tests,
focused Huffman and compressed-block tests, all-target clippy with warnings
denied, formatting, release profiler/benchmark builds, broad comparison, and
`git diff --check`. Artifacts:
`benchmarks/tmp/perf-branches-z000033-l16-rust-after-fixed-huffman-workspace.data`
and
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-fixed-huffman-workspace.csv`.

Latest kept Huffman workspace-layout change after that checkpoint:
`HuffmanNode` now matches C's 8-byte `nodeElt` representation: a `u32` count,
`u16` parent, `u8` symbol, and `u8` depth. Checked conversions protect the
public `usize` count boundary, parent sums are checked, and the test suite
asserts the 8-byte layout. The previous four-`usize` node occupied 32 bytes,
making every 512-node sort/tree workspace four times larger than C's. No unsafe
code was added. Focused bytes stayed exactly `9,211,440` for 20 runs and
`36,845,760` for 80 runs. Three 20-run samples improved to
`8,267,309,488`, `8,266,117,906`, and `8,264,037,566` instructions from the
preceding `8.326B` band, with hardware branches essentially flat. The paired
80-run Rust/C result is recorded in the top restart checkpoint. All 511 broad
size columns are byte-identical to the fixed-workspace artifact. Validation
passed 691 library tests plus 7 integration tests, focused Huffman/compressed
tests, all-target clippy with warnings denied, formatting, release builds,
broad comparison, and `git diff --check`. Artifacts:
`benchmarks/tmp/perf-instructions-z000033-l16-rust-after-compact-huffman-nodes.data`
and
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-compact-huffman-nodes.csv`.

Rejected follow-up from the fixed-workspace checkpoint: replacing the owning
Huffman code `Vec` with an inline fixed-capacity 256-entry table preserved
focused bytes but regressed three 20-run samples to `8,596,221,966`,
`8,587,635,881`, and `8,589,002,751` instructions, with about `1.378B`
branches. Moving/copying the 2 KiB table outweighed the saved allocation. It
was reverted; keep the compact owning `Vec` output table and pursue reusable
external construction scratch instead of embedding C's output CTable directly.

Retained C-heuristic cleanup immediately before that checkpoint:
`BlockCompressionConfig::search_smallest_huffman_table()` now maps C-cost
`HuffmanTableSearch::Heuristic` to C's single fast table-log build, while
`AllSections` still performs the BtUltra+ optimal-depth search and legacy/file
type modes retain their prior search behavior. Focused bytes and CPU were
neutral; broad low-level aggregate gaps moved by only zero to nineteen bytes
and remained in Rust's favor. Keep this as semantic C parity, not as a claimed
performance win. Artifact:
`benchmarks/tmp/normal-levels-1-3-5-8-16-19-22-api-after-c-heuristic-single-table.csv`.

Latest prepared-CDict checkpoint from July 18, 2026: two independent parity
defects are fixed. First, attached dictionary setup now retains dictionary
content using the prepared dictionary's `CreateCDict` hash/chain parameters,
not the smaller active source tables. The old path truncated the focused
51,812-byte dictionary content to 32 KiB at level 16. Second, full-dictionary
entropy now records C's `HUF_repeat_valid` condition (all 256 symbols have
nonzero weights). Fast, DFast, and Greedy normal blocks use that valid table
before C's small-input histogram rejection, matching
`ZSTD_compressLiterals()`/`HUF_flags_preferRepeat`; a newly emitted Huffman
table clears the valid state back to the check path.

Focused `/usr/lib/systemd/system/apt-show-versions.timer` level 5 prepared
compression is now exactly 110 bytes in both Rust and C, with the same
17 sequences and a 34-byte treeless literal payload. Focused
`/usr/lib/systemd/system/autovt@.service` level 16 remains byte-for-byte exact
at 314 bytes; both SHA-256 digests are
`22562be06fbc3266e9fabb342d48fe15aae3ea212e201f16b66b49496053d651`.
The current 100-file prepared-CDict artifact is
`benchmarks/tmp/prepared-dictionary-systemd-100-levels-1-3-5-8-11-15-16-19-api-after-valid-repeat.csv`.
Aggregate Rust-versus-C gaps are level 1 `-219` bytes (`-0.990%`), level 3
`-67` (`-0.332%`), level 5 `+43` (`+0.222%`), level 8 `+0`, level 11 `-1`,
level 15 `-7`, level 16 `-1`, and level 19 `+0`. Before dictionary retention,
levels 5, 8, 11, 15, and 16 were respectively `+2704`, `+2460`, `+2703`,
`+2707`, and `+2710` bytes. Validation passed full `ruzstd` tests, clippy for
all targets with warnings denied, formatting, `git diff --check`, release tool
rebuilds, focused inspectors, and the broad decoding-verified matrix. The
retained debug C source is `/tmp/zstd-est-trace/zstd`; the cargo-registry source
listed in "Next Resume Action" remains authoritative.

Latest dictionary attach-mode checkpoint: `params.rs` now has a tested
`should_attach_dict_by_default()` helper mirroring C's default
`ZSTD_shouldAttachDict()` strategy cutoffs: fast `8 KiB`, dfast `16 KiB`,
greedy/lazy/lazy2/btlazy2/btopt `32 KiB`, and btultra/btultra2 `8 KiB`, with
unknown source size attaching by default. The focused
`repo_Cargo.lock` level-5 target-2048 dictionary case (`31,858` pledged source
bytes, greedy CDict strategy) is therefore confirmed to be a C attach-CDict
case. This helper is intentionally not wired into the frame encoder yet:
current Rust greedy/lazy dictionary frames still use a combined-buffer
loaded-prefix/ext-dict approximation. Do not switch those paths to
`CParamMode::AttachDict` until the attached `dictMatchState` match-state
topology is ported, otherwise parameters and match finder behavior will be a
non-C hybrid. Validation for this checkpoint passed `cargo fmt --check`,
`cargo test -p ruzstd default_attach_dict_mode --quiet`, and
`cargo test -p ruzstd attach_dict_mode_ignores_dictionary_size --quiet`.

Latest attached row-dictionary match-state slice after that checkpoint: the
greedy/lazy row search plumbing can now carry an optional attached dictionary
descriptor. `row_match.rs` searches active row candidates first, inserts the
current active position, then searches the attached dictionary row table with
the remaining C-style attempt budget and translates the dictionary index with
the same virtual-index model as C's
`ZSTD_RowFindBestMatch(..., ZSTD_dictMatchState)`. The important C detail is
that `ZSTD_WINDOW_START_INDEX` is `2`, so CDict row tables store dictionary
byte 0 at virtual index `2`; the Rust descriptor now carries both the CDict
virtual index base and the Rust active-prefix slice start before computing
offsets. A narrow
`compress_block_greedy_attached_row_dict_with_state()` block adapter runs the
existing lazy loop with a clean active state and a separate dictionary
`GreedyMatchState`. Tests cover the row helper and block adapter. This is still
not wired into dictionary frame routing; the next integration step must build
the CDict-side row state with the right `CreateCDict` parameters, select attach
only when `should_attach_dict_by_default()` says C would attach, and preserve
the active CCtx/frame-state behavior. Validation passed `cargo fmt --check`,
`cargo test -p ruzstd row_match --quiet`, `cargo test -p ruzstd greedy_ext
--quiet`, `cargo test -p ruzstd params --quiet`, `cargo clippy -p ruzstd
--all-targets -- -D warnings`, and `git diff --check`.

Rejected frame-routing follow-up after the attached row-DMS slice: routing
greedy dictionary frames directly through the new attached row block adapter,
using CDict `CreateCDict` parameters adjusted with `CParamMode::AttachDict`,
decoded correctly but badly regressed the focused dictionary+target grid.
Artifact:
`benchmarks/tmp/dictionary-target-levels-1-3-5-8-16-22-api-after-attached-row-frame.csv`.
`repo_Cargo.lock` level 5 target 2048 moved from the kept `7997` Rust bytes
versus `7993` C bytes (`+4`) to `8148` Rust bytes (`+155`), and level 5
aggregate moved from `-21` to `+130`. The frame route was removed. Post-revert
artifact:
`benchmarks/tmp/dictionary-target-levels-1-3-5-8-16-22-api-after-attached-row-frame-revert.csv`
restores the kept grid: level 1 `+0`, level 3 `+0`, level 5 `-21` with
`repo_Cargo.lock +4`, level 8 `+0`, level 16 `+1` from
`generated_poetry.lock`, and level 22 `+0`. Keep the row-DMS primitive and
tests, but do not retry this simple clean-active-state frame route without
matching more of C's CDict/frame-state setup and trace evidence explaining the
parse divergence. Validation after revert passed `cargo fmt --check`, focused
greedy frame/ext/strategy/row/params tests, release rebuild of
`benchmark_c_port`, the focused benchmark grid, `cargo clippy -p ruzstd
--all-targets -- -D warnings`, and `git diff --check`.

Latest kept follow-up after the C-indexed row-DMS fix: the greedy dictionary
frame path now uses the attached row-DMS block adapter only for the C-shaped
case: greedy strategy, row match enabled, C's default
`should_attach_dict_by_default()` says to attach, and active/CDict row widths
match. The CDict row state is loaded with `ZSTD_WINDOW_START_INDEX == 2`, and
the active state starts clean at the Rust active prefix. This is deliberately
not routed for lazy/lazy2/btlazy2 or row-width mismatches. Focused
dictionary+target artifact
`benchmarks/tmp/dictionary-target-levels-1-3-5-8-16-22-api-after-cindexed-attached-row-frame.csv`
improves the remaining `repo_Cargo.lock` level-5 target-2048 row from the kept
`+4` bytes (`7997` Rust versus `7993` C) to `-1` byte (`7992` Rust versus
`7993` C). Aggregate level 5 moves from `-21` to `-26`; levels 1, 3, 8, and
22 are exact, and the only remaining positive row is the existing
`generated_poetry.lock` level 16 `+1`. Validation passed `cargo fmt --check`,
focused greedy/strategy tests, full `cargo test -p ruzstd --quiet`, `cargo
clippy -p ruzstd --all-targets -- -D warnings`, the release `benchmark_c_port`
rebuild, the focused dictionary+target grid, and `git diff --check`.

Latest attached-dictionary port extension: the safe row-based
`dictMatchState` adapter now carries the actual lazy-search depth, and the
dictionary frame route uses it for C's row-hash `Greedy`, `Lazy`, and `Lazy2`
strategies when default attach mode is selected and the active/CDict row widths
match. `BtLazy2` remains on the separate binary-tree/ext-dict path. Direct tests
prove attached dictionary matches at lazy depths 1 and 2 and prove frame
selection includes small lazy/lazy2 inputs while excluding `BtLazy2` and
sources above C's 32 KiB attach cutoff. The target+dictionary artifact
`benchmarks/tmp/dictionary-target-levels-1-3-5-8-16-22-api-after-attached-lazy-row.csv`
has no positive rows: levels 1, 3, 8, and 22 are exact, level 5 totals `-26`
bytes, and level 16 totals `-1` across five fixtures. The normal-mode artifact
`benchmarks/tmp/dictionary-normal-levels-5-10-api-after-attached-lazy-row.csv`
shows the affected 31,858-byte `repo_Cargo.lock` row exact at level 6, `-1` at
level 7, and exact at levels 8-10. Validation passed focused attached-row and
greedy-ext tests, full `cargo test -p ruzstd --quiet` (`674` passed, `5`
ignored), clippy for all `ruzstd` targets with warnings denied, format,
dictionary decode comparison, and `git diff --check`.

Latest attached-dictionary binary-tree extension: C's `BtLazy2`
`dictMatchState` topology is now routed for default-attach frames. The new
`bt_match/attached.rs` builds the CDict's fully sorted tree with C's
`ZSTD_updateTree()`/`ZSTD_insertBt1()` shape, keeps dictionary entries in the
virtual index space beginning at `ZSTD_WINDOW_START_INDEX == 2`, and searches
that tree only with the comparison budget left by the active DUBT walk. The
dictionary candidate penalty, two-segment match continuation, end-of-input
stop, and offset translation mirror `ZSTD_DUBT_findBetterDictMatch()`. No
unsafe code was added. The attach-eligible 31,858-byte `repo_Cargo.lock`
normal level-11 row improved from Rust `7768` versus C `7767` to exact `7767`.
Across the five-fixture normal artifact
`benchmarks/tmp/dictionary-normal-levels-11-15-api-after-attached-bt.csv`,
level 11 therefore moved from aggregate `+5` to `+4`; the remaining positive
level-11 fixture is 51,962 bytes and correctly exceeds C's 32 KiB attach
cutoff. Levels 12-15 were unchanged. The target-2048 artifact is
`benchmarks/tmp/dictionary-target-levels-11-15-api-after-attached-bt.csv`;
the attach-eligible level-11 row is exact there as well. Direct attached-tree
matching and frame-selection tests pass. For reviewability, `bt_match.rs` is
now 283 lines, `bt_match/attached.rs` is 239, `greedy_frame.rs` is 447, and
its extracted `greedy_frame/attached_tests.rs` is 54 lines.
The post-split release-binary smoke is
`benchmarks/tmp/dictionary-normal-level-11-api-after-attached-bt-split.csv`;
it reproduces the exact `repo_Cargo.lock` row.

Latest kept target+dictionary parity follow-up: the remaining
`generated_poetry.lock` level-16 target-2048 `+1` row was traced to the final
optimal target block. C emits the final one-sequence block as one raw literal
plus `LL=1, ML=567, OF=1` (`OF=1` is rep1), while Rust emitted a zero-literal
full-block match with a large explicit offset. A rejected target-emitter
experiment that moved the first byte into literals but kept the explicit raw
offset regressed the final block to 332 bytes, proving the win is the repcode
choice, not the literal split alone. The kept fix is in
`opt_encode.rs`: before target-mode optimal block emission, a verified helper
rewrites only final single full-block zero-literal matches when the incoming
rep1 reproduces `block[1..]`, changing the prepared sequence to one leading
literal and `encoded_offset_value = Some(1)`. Focused inspect artifact
`benchmarks/tmp/inspect-dict-target-poetry-l16-opt-rep1-leading-lit` now shows
Rust and C both at `330` bytes with matching block summaries. Focused
dictionary+target artifact
`benchmarks/tmp/dictionary-target-levels-1-3-5-8-16-22-api-after-opt-rep1-leading-lit.csv`
has no positive rows: levels 1, 3, 8, and 22 are exact, level 5 is `-26`
bytes, and level 16 is `-1` byte across the five fixtures. Validation passed
`cargo fmt --check`, the new targeted `target_leading_repcode_literal` tests,
full `cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd --all-targets
-- -D warnings`, `git diff --check`, release `benchmark_c_port` rebuild, focused
inspect, and the focused dictionary+target grid.

Rejected normal-mode post-split probe after the target+dictionary parity
follow-up: changing `post_split::derive_block_splits_helper()` to recompute the
original partition estimate at every recursive call, matching the literal C
control flow instead of reusing the already-computed parent half estimate,
preserved focused `corpus_z000033` level-16 output exactly. The inspect artifact
`benchmarks/tmp/inspect-z000033-l16-postsplit-recompute-estimates` still shows
Rust `461224` bytes versus C `460806`, Rust `191` compressed blocks versus C
`192`, `50,177` sequences on both sides, source-aligned summary
`groups=128,total_delta=418,abs_delta=1006,rust_groups=131,c_groups=132`, and
the same first source-boundary mismatch at block 33. It was reverted because it
only removes the current estimator reuse without closing the size or boundary
gap. Do not retry this shape unless a new estimator-side-effect profile gives a
different reason.

Rejected level-22 post-split follow-up after the high-level size review:
rerunning that same recursive-estimate recomputation on focused
`corpus_z000033` level 22 also preserved the output and block layout exactly.
Artifact `benchmarks/tmp/inspect-z000033-l22-recompute-recursive-estimates`
still shows Rust `426585` bytes versus C `426312`, Rust `252` compressed blocks
versus C `256`, `114,233` sequences on both sides, source-aligned summary
`groups=34,total_delta=273,abs_delta=461,rust_groups=44,c_groups=48`, and the
dominant final source-aligned row `985341..1022035` at `+259` bytes where Rust
keeps one partition while C emits eight. The recompute change was reverted. Do
not retry recursive original-estimate recomputation for this final-block
level-22 gap unless a new trace shows estimator scratch side effects rather
than the reused numeric estimate are the cause.

Follow-up diagnostic after that rejection: a temporary forced split at the
exact final `corpus_z000033` level-22 helper subrange (`985341..1022035`,
`36,694` source bytes, `2,840` sequences) used C's observed recursive
boundaries `[710, 887, 1065, 1420, 1775, 1952, 2130]`. Artifact
`benchmarks/tmp/inspect-z000033-l22-force-c-final-helper-splits` shows Rust
improves from `426585` bytes to `426324` bytes versus C `426312`; the
source-aligned total gap drops from `+273` to `+12`, and the final
source-aligned `+259` row disappears. The forced Rust tail partitions match C
except for a `-2` byte row on `991395..993350`. A separate temporary estimate
panic for the same helper range measured Rust estimates `original=17652`,
`first=5199`, `second=12463`, `sum=17662`, so Rust rejects the split by only
`10` estimated bytes even though emitted partitions are much smaller. The
forced split was reverted. Next size-parity work should compare Rust's
post-split estimator with C for this exact range; do not add a blind split
tolerance without explaining the estimator mismatch, because older tolerance
artifacts shifted other rows.

Latest kept size-parity fix after that diagnostic:
`BlockCompressionConfig::for_c_block_split_estimate()` now preserves
`HuffmanTableSearch::AllSections` when the strategy config already selected it
for `BtUltra`/`BtUltra2`, while still mapping non-optimal searches such as
`FileTypeSmall` to C's normal `Heuristic` path. The previous estimate cleanup
was correct for level-16 `BtOpt`, but it accidentally disabled C's
`HUF_flags_optimalDepth` behavior for level-22 post-split estimates. An
instrumented C copy under `/tmp/zstd-est-trace` showed the decisive final
`corpus_z000033` level-22 range had matching original and second-half estimates
but a first-half literal-estimate difference: C `first=5183` with `lit=2119`
and `seq=3061`, Rust before the fix `first=5199` with `lit=2135` and
`seq=3061`. Preserving optimal-depth Huffman search in the post-split estimate
made Rust accept the C-shaped recursive split. Focused inspect artifact
`benchmarks/tmp/inspect-z000033-l22-after-btultra-postsplit-optdepth` shows
Rust `426270` bytes versus C `426312` on `corpus_z000033` level 22; the
source-aligned total gap is now `-42` and the largest positive rows are only
`+10` bytes. Broad artifact
`benchmarks/tmp/normal-levels-16-19-22-api-runs1-after-btultra-postsplit-optdepth-rebuilt.csv`
shows level 16 unchanged at `+402` bytes versus C across 73 fixtures, level 19
improved to `-174`, and level 22 improved to `-64`; worst positive rows at
levels 19 and 22 are now `+2` bytes on `corpus_z000044`. Focused 20-run
wall-clock smoke emitted Rust `8,525,400` bytes in `3.352s` versus C API
`8,526,240` bytes in `3.915s`. Validation passed `cargo fmt --check`, focused
config tests, `cargo test -p ruzstd compressed --quiet`, full `cargo test -p
ruzstd --quiet`, `cargo clippy -p ruzstd --all-targets -- -D warnings`, release
rebuilds for `benchmark_c_port`, `profile_c_port`, and `profile_c_api`, the
focused inspect, broad API comparison, and `git diff --check`.

Diagnostic follow-up after that rejected probe: `inspect_c_port_blocks` now
writes durable CSVs into its output directory: `rust.blocks.csv`,
`c.blocks.csv`, and `source-aligned.csv`. A focused rerun on
`corpus_z000033` level 16 with the C API is
`benchmarks/tmp/inspect-z000033-l16-current-with-source-csv`. The CSVs confirm
the printed summary: `source-aligned.csv` has `128` delta rows, total delta
`+418`, and absolute delta `1006`; `rust.blocks.csv` has `191` blocks and
`c.blocks.csv` has `192`. The first source-boundary mismatch remains
`168226..172032`: Rust splits it into two blocks (`168226..169509` and
`169509..172032`) while C emits one block. This is not the aggregate root cause:
only seven source-aligned groups have different block counts, and the largest
positive same-source-range deltas are still small (`+24`, `+23`, `+23`). Use
the CSVs for future split/entropy hypotheses instead of re-parsing console
output.

Latest kept normal-mode Huffman table-log fix after the level-16 source-aligned
CSV diagnostic: `inspect_c_port_blocks` now records literal Huffman table
description size (`literal_table_size`) in `rust.blocks.csv`, `c.blocks.csv`,
and the printed section suffix. That exposed the remaining level-16
`corpus_z000033` positive rows as fresh Huffman table descriptions that were
larger than C's, even when literal counts matched. For normal C-strategy
emitted blocks, `compress_literals()` now requests the C fast/non-optimal table
log (`HUF_optimalTableLog()` without `HUF_flags_optimalDepth`) when the
strategy config uses the C literal cost model and not the Rust smallest-table
search. The focused diagnostic block that was `+23` bytes now matches C's
`60`-byte table description instead of Rust's former `87`-byte description.
Focused `corpus_z000033` level-16 one-run output improved from Rust `461,224`
versus C `460,806` (`+418`) to Rust `460,572` versus C `460,806` (`-234`).
The broad artifact
`benchmarks/tmp/normal-levels-16-19-22-api-runs1-after-c-fast-huff-log.csv`
shows level 16 `-253` bytes (`-0.018%`), level 19 `-174` bytes (`-0.013%`),
and level 22 `-64` bytes (`-0.005%`) versus C across 73 fixtures. Focused
80-run perf counters are Rust `36,845,760` bytes,
`33,678,645,198` instructions, `19,878,801,734` cycles,
`5,449,699,950` branches, and `215,219,140` branch misses; same-session C API
is `36,864,480` bytes, `34,690,615,149` instructions, `18,514,582,045`
cycles, `4,327,110,245` branches, and `175,021,650` branch misses. Validation
passed `cargo fmt --check`, focused `c_fast_table_log` tests,
`cargo test -p ruzstd compressed --quiet`, full `cargo test -p ruzstd --quiet`,
`cargo clippy -p ruzstd --all-targets -- -D warnings`, release rebuilds for
`profile_c_port`, `profile_c_api`, `benchmark_c_port`, focused inspect,
broad API comparison, and `git diff --check`.

Latest dictionary benchmark tooling checkpoint: `benchmark_c_port` now accepts
`--dictionary PATH` for Rust-vs-C comparisons through both the C API and CLI
backends. Dictionary outputs are decoded with `zstd -D` before byte comparison.
A focused API run on
`benchmarks/archive/tmp/dict-secondnewest-focused-fixtures` with levels
`1,3,8,16,22` produced
`benchmarks/tmp/dictionary-focused-levels-1-3-8-16-22-api-current.csv`: level
1 aggregate gap `+0` bytes, level 3 `+22` bytes (`+0.235%`), level 8 `+0`,
level 16 `+1` byte (`+0.012%`), and level 22 `+0` across 5 fixtures. Treat the
CPU timings in that artifact as smoke only; files are too small for reliable
CPU conclusions. Validation passed `cargo fmt --check`,
`cargo clippy -p zstd-rs-tools --bin benchmark_c_port -- -D warnings`,
`git diff --check`, release rebuild of `benchmark_c_port`, an API dictionary
smoke, and a CLI dictionary smoke. Combined dictionary+target support was
ported later; see the target+dictionary checkpoint below.

Latest DFast target+dictionary parity checkpoint: the earlier assumption that
`ZSTD_CCtx_loadDictionary()` used the DFast `dictMatchState` block compressor
for the focused C API target+dictionary benchmark was wrong. A fresh `perf`
profile on `generated_yarn.lock` level 3 target 2048 with the focused
dictionary shows C spends block time in
`ZSTD_compressBlock_doubleFast_extDict_generic`, while dictionary setup builds
a CDict (`ZSTD_createCDict_advanced2`) and copies CDict tables into the active
CCtx. The kept Rust fix mirrors that path for DFast dictionary frames:
`ParsedDictionary` now carries the raw dictionary size, `dfast_frame` derives
copied-CDict compression parameters with `CParamMode::CreateCDict` while
preserving the active window log and target size, and `dfast_dict` has
`load_cdict_copy_prefix()` which fills the CDict-style tagged double-fast
tables and strips tags into the active match state like
`ZSTD_copyCDictTableIntoCCtx()`.

This closed the previous worst level-3 target+dictionary row:
`generated_yarn.lock` is now C/Rust `379` bytes. Focused levels 1 and 3 across
the 5 dictionary fixtures are
`benchmarks/tmp/dictionary-target-fast-dfast-api-after-dfast-cdict-params.csv`:
level 1 stayed `+16` bytes (`+0.167%`), while level 3 improved from `+41`
bytes (`+0.435%`) to `+10` bytes (`+0.106%`). The broader focused artifact
`benchmarks/tmp/dictionary-target-levels-1-3-5-8-16-22-api-after-dfast-cdict-params.csv`
shows only level 3 changed: level 1 `+16`, level 3 `+10`, level 5 `-11`,
level 8 `+9`, level 16 `+9`, and level 22 `+8` bytes versus C. Remaining
positive rows are dominated by `generated_go.sum` at all levels; continue
there next. Rejected/removed in this checkpoint: the untracked
`dfast_dict_match.rs` route and the artificial `>4` dictionary short-match
filter. Validation passed `cargo fmt --check`, `cargo test -p ruzstd dfast
--quiet`, `cargo test -p ruzstd strategy_frame_dictionary_target_c_block_size
--quiet`, full `cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd
--all-targets -- -D warnings`, release rebuild of `benchmark_c_port`, the
focused dictionary+target smokes, and `git diff --check`.

Latest dictionary+target Huffman-repeat checkpoint after the DFast CDict-copy
fix: `target_block.rs` now mirrors C target/superblock literal entropy
selection when a previous Huffman table exists. Instead of accepting the fresh
compressed-Huffman single-sub-block candidate before considering treeless
literals, Rust now checks the C-style
`ZSTD_buildBlockEntropyStats_literals()` criterion: the previous table must be
able to encode the literal counts, its estimated payload must be below raw
size, and it must beat/tie the new table after including the new table
description, or the new table description must be too expensive
(`hSize + 12 >= srcSize`). When that criterion selects repeat, target mode
tries treeless Huffman before the fresh table. A focused regression test covers
this ordering for a valid previous Huffman table.

This closed the remaining `generated_go.sum` target+dictionary gaps in the
focused 5-fixture set: level 3 moved from C/Rust `154/164` to `154/154`, and
level 16 moved from `138/146` to `138/138`. The broader focused artifact is
`benchmarks/tmp/dictionary-target-levels-1-3-5-8-16-22-api-after-target-repeat-estimate.csv`:
level 1 is now `+7` bytes, level 3 `+0`, level 5 `-21`, level 8 `+0`, level
16 `+1`, and level 22 `+0` versus C across the 5 dictionary fixtures. The
only remaining positive rows are `generated_yarn.lock` level 1 `+2`,
`repo_Cargo.lock` level 1 `+5`, `repo_Cargo.lock` level 5 `+4`, and
`generated_poetry.lock` level 16 `+1`. Rejected follow-up from this checkpoint:
a broad `strategy < Lazy && literals <= 1024` repeat-first rule closed
`generated_go.sum` level 3 but regressed `generated_yarn.lock` level 3 from
`379/379` to `379/385`; keep the C-estimate criterion instead. Validation
passed `cargo fmt --check`, focused `target_block` and
`strategy_frame_dictionary_target_c_block_size` tests, full
`cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd --all-targets -- -D
warnings`, `git diff --check`, release rebuild of `benchmark_c_port`, and the
focused dictionary+target benchmarks.

Latest Fast target+dictionary CDict-copy checkpoint after that: Fast dictionary
frames now mirror DFast's copied-CDict setup. `FastMatchState` has
`load_cdict_copy_prefix()`, which fills a C-style tagged CDict hash table with
`hashLog + 8`, full every-third-plus-empty-slot coverage, and then strips tags
into the active CCtx hash table like `ZSTD_copyCDictTableIntoCCtx()`.
`fast_frame` now derives copied-CDict table parameters with
`CParamMode::CreateCDict` while preserving the active window log and target
block size. This closed the remaining level-1 target+dictionary positives in
the focused 5-fixture set: `generated_yarn.lock` moved from `+2` to `+0`, and
`repo_Cargo.lock` moved from `+5` to `+0`. Artifact:
`benchmarks/tmp/dictionary-target-levels-1-3-5-8-16-22-api-after-fast-cdict-copy.csv`.
Focused aggregate gaps are now level 1 `+0`, level 3 `+0`, level 5 `-21`,
level 8 `+0`, level 16 `+1`, and level 22 `+0` bytes versus C. The only
remaining positive rows in that focused grid are `repo_Cargo.lock` level 5
`+4` and `generated_poetry.lock` level 16 `+1`. Rejected follow-up from this
checkpoint: changing greedy/hash-chain dictionary frames to use only the
copied-CDict parameter derivation did not close `repo_Cargo.lock` level 5 and
regressed useful negative level-5 gaps on `generated_poetry.lock` and
`generated_yarn.lock` back to exact parity; it was reverted. Validation passed
`cargo fmt --check`, focused Fast and target/dictionary tests, full
`cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd --all-targets -- -D
warnings`, release rebuilds of `benchmark_c_port` and `inspect_c_port_blocks`,
the focused dictionary+target benchmark, and `git diff --check`.

Follow-up investigation after the Fast CDict-copy checkpoint: a short C API
profile for `repo_Cargo.lock` level 5 with target size 2048 and the focused
dictionary was recorded as
`benchmarks/tmp/perf-c-api-repo-cargo-l5-target2048-dict.data`. The top C
symbols are `ZSTD_RowFindBestMatch_dictMatchState_4_4`,
`ZSTD_row_update`, and `ZSTD_compressBlock_greedy_dictMatchState_row`.
Therefore the remaining `repo_Cargo.lock` level-5 `+4` gap is not a
hash-chain/ext-dict issue: C is using greedy row match with a separate
dictionary match state, while Rust still models dictionary frames by loading
the dictionary into the active combined-buffer match state. A probe that made
Rust's hash-chain ext-dict branch use C's dictionary-candidate first-four-byte
test was byte-neutral across the focused grid
(`benchmarks/tmp/dictionary-target-levels-1-3-5-8-16-22-api-after-hc-extdict-dict-probe.csv`)
and was reverted. Next useful work for this row is a row-match
dictMatchState port or a narrower row-DMS parity experiment, not more
hash-chain/ext-dict tweaks. Validation for the probe pass included
`cargo fmt --check`, focused greedy/row/target tests, release rebuilds of
`profile_c_api`, `benchmark_c_port`, and `inspect_c_port_blocks`, the focused
benchmark grid, and `git diff --check`.

Rejected follow-up after that row-DMS investigation: adding a narrow separate
row dictionary match-state path for greedy dictionary frames, while leaving
the active row table unloaded with dictionary rows, did not close the focused
gap. It preserved all other focused rows but moved `repo_Cargo.lock` level 5
from `+4` bytes to `+5` bytes, with level-5 aggregate `-21 -> -20` bytes
versus C. Artifact:
`benchmarks/tmp/dictionary-target-levels-1-3-5-8-16-22-api-after-row-dms.csv`.
The experiment was reverted and the post-revert artifact
`benchmarks/tmp/dictionary-target-levels-1-3-5-8-16-22-api-after-row-dms-revert.csv`
matches the Fast CDict-copy checkpoint exactly: level 5 total `-21` with
`repo_Cargo.lock` still `+4`, and level 16 total `+1` from
`generated_poetry.lock`. Do not retry that exact "separate row state plus
active table starts after dictionary" shape without deeper evidence from C row
iteration/order parity.

Follow-up row evidence after the rejected row-DMS shape: a C-like active-row
buffering experiment changed `row_match.rs` to collect row candidates, insert
the current position, and then score the buffered candidates. Focused artifact
`benchmarks/tmp/dictionary-target-levels-1-3-5-8-16-22-api-after-row-active-buffer.csv`
was byte-neutral versus the post-revert checkpoint: level 5 total stayed `-21`
with `repo_Cargo.lock` still `+4`, and level 16 stayed `+1`. The experiment was
removed because it added hot-path work without a byte win. Temporary probes
showed Rust dispatches the focused row through greedy ext-dict with loaded
dictionary length `51812`; C's divergent sequence at source block position
`3466` corresponds to combined `ip=55278`, C offBase `25977`, and dictionary
match index `29304`. A dictionary-only row-table simulation contains that
candidate, but the actual Rust active row table at `ip=55278` does not. A later
additive row-DMS experiment kept the current active-table loading behavior and
searched a copied dictionary row table as a secondary candidate source. It also
failed to close the gap: artifact
`benchmarks/tmp/dictionary-target-levels-1-3-5-8-16-22-api-after-additive-row-dms.csv`
has level 5 total `-20` and `repo_Cargo.lock` level 5 `+5`, regressing from the
kept `+4`. That implementation was removed; post-revert artifact
`benchmarks/tmp/dictionary-target-levels-1-3-5-8-16-22-api-after-additive-row-dms-revert.csv`
restored level 5 total `-21` with `repo_Cargo.lock` level 5 `+4`, and level 16
total `+1` from `generated_poetry.lock`. Do not retry either row-DMS shape
without deeper C row candidate-order or lazy-parser evidence.

Additional row-DMS probe after reading C `zstd_lazy.c`: C's
`ZSTD_RowFindBestMatch(..., ZSTD_dictMatchState)` computes the dictionary-match
state row with `ZSTD_hashPtr()` on `dms->rowHashLog`, while active rows use the
salted active hash. A corrected additive probe built a separate unsalted
dictionary row table and searched it after the active row candidates. It still
regressed the focused grid exactly like the earlier additive probe: artifact
`benchmarks/tmp/dictionary-target-levels-1-3-5-8-16-22-api-after-unsalted-row-dms.csv`
has level 5 total `-20` and `repo_Cargo.lock` level 5 `+5`. The implementation
was removed; post-revert artifact
`benchmarks/tmp/dictionary-target-levels-1-3-5-8-16-22-api-after-unsalted-row-dms-revert.csv`
restored level 5 total `-21` with `repo_Cargo.lock` level 5 `+4`, and level 16
total `+1` from `generated_poetry.lock`. The remaining issue is therefore not
just the absence of C's dictionary candidate at `ip=55278`; adding that
candidate without the rest of C's row/lazy interaction worsens the selected
parse.

Rejected follow-up after the unsalted row-DMS probe: matching C's row ordering
more exactly by collecting active candidates, inserting the current position,
then collecting unsalted DMS candidates before scoring the combined candidate
buffer also regressed the focused grid. Artifact
`benchmarks/tmp/dictionary-target-levels-1-3-5-8-16-22-api-after-exact-order-row-dms.csv`
has level 5 total `-20` and `repo_Cargo.lock` level 5 `+5`, with level 16 still
`+1`. That implementation was removed; post-revert artifact
`benchmarks/tmp/dictionary-target-levels-1-3-5-8-16-22-api-after-exact-order-row-dms-revert.csv`
restored level 5 total `-21` with `repo_Cargo.lock` level 5 `+4`, and level 16
total `+1` from `generated_poetry.lock`. Do not retry active-buffered row-DMS
ordering without new C trace evidence that explains why C accepts the early
`ip=55278` dictionary match without causing the later parse regression.

Fresh current-source sequence trace after the row-DMS rejections: rebuilt
`inspect_c_port_blocks` and generated fresh focused outputs in
`benchmarks/tmp/inspect-dict-target-repo-cargo-l5-current/`, then used the
ignored `ruzstd` `inspect_archive_from_env` diagnostic with
`RUZSTD_INSPECT_SEQUENCE_DUMP_BLOCK=0` to decode block-0 sequences. Artifacts:
`benchmarks/tmp/dictionary-target-repo-cargo-l5-current-c-block0-sequences.txt`,
`benchmarks/tmp/dictionary-target-repo-cargo-l5-current-rust-block0-sequences.txt`,
the corresponding `.csv` files, and summary
`benchmarks/tmp/dictionary-target-repo-cargo-l5-current-block0-diff-summary.txt`.
Current block 0 has `266` C sequences versus `264` Rust sequences. The first
content split is still C seq 118 `start=3465 ll=1 match_start=3466 ml=4
of=25977 end=3470` plus C seq 119 `start=3470 ll=5 match_start=3475 ml=4
of=6808 end=3479`, while Rust seq 118 is `start=3465 ll=10 match_start=3475
ml=4 of=6808 end=3479`; both streams resynchronize by `end=3483`. The next
independent split after resync is at decoded position `5654`: C emits
`ll=0 ml=5 of=38947`, while Rust emits `ll=1 ml=4 of=6725`, and both end at
`5659`. C also has an additional final block-0 sequence at `start=6878 ll=4
match_start=6882 ml=6 of=44 end=6888`, while Rust block 0 ends at `6878`.
Mechanical comparison by decoded start/end positions found only these two
content differences in shared block-0 positions, plus C's final extra sequence.
This suggests the next investigation should look for a repeated pattern in
zero-literal dictionary matches selected by C but skipped/delayed by Rust,
rather than focusing only on the first `ip=55278` row-DMS candidate.

Latest benchmark-tool reviewability cleanup after that checkpoint:
`tools/src/bin/benchmark_c_port.rs` was split by responsibility. The root
binary now owns CLI parsing, fixture walking, and benchmark orchestration
(`469` lines), `tools/src/bin/benchmark_c_port/reference.rs` owns C/Rust
reference compression, dictionary decoding, target-size handling, and CPU
timing helpers (`332` lines), and
`tools/src/bin/benchmark_c_port/report.rs` owns CSV/Markdown rendering
(`164` lines). Validation passed `cargo fmt --check`,
`cargo test -p zstd-rs-tools --bin benchmark_c_port --quiet`,
`cargo clippy -p zstd-rs-tools --bin benchmark_c_port -- -D warnings`,
release rebuild of `benchmark_c_port`, API dictionary, CLI dictionary, and
target-mode smokes, and `git diff --check`.

Latest dictionary profiling tooling checkpoint: `profile_c_port` and
`profile_c_api` now accept dictionary profiling in the same positional command
shape as target profiling. The fourth argument remains `TARGET_C_BLOCK_SIZE`
when it parses as a number; otherwise it is treated as `DICTIONARY_PATH`.
Dictionary profiling was intentionally rejected when combined with
targetCBlockSize in this older checkpoint; combined dictionary+target support
was ported later. Normal and target profiling smokes still match Rust/C bytes.
Dictionary smoke on
`benchmarks/archive/tmp/dict-secondnewest-focused-fixtures/repo_Cargo.lock`
level 16 for 20 runs emitted identical Rust and C API totals (`148,960` bytes).
An 80-run `perf stat` dictionary sample on the same fixture emitted identical
totals (`595,840` bytes); Rust counters were `6,012,722,969` instructions,
`2,760,080,384` cycles, `986,207,423` branches, and `28,132,169` branch
misses versus C API `5,901,855,672` instructions, `2,478,522,349` cycles,
`750,664,636` branches, and `22,494,473` branch misses. Validation passed
`cargo fmt --check`, release rebuild of both profilers, clippy for both
profiler binaries with warnings denied, normal/target/dictionary smokes,
and `git diff --check`.

Latest target+dictionary porting checkpoint: C `zstd` supports combining
`-D` with `--target-compressed-block-size`; a CLI smoke on
`benchmarks/archive/tmp/dict-secondnewest-focused-fixtures/repo_Cargo.lock`
at level 16 decoded correctly and changed C bytes from `7448` normal
dictionary bytes to `7509` target+dictionary bytes. Rust has a validation hook,
`compress_slice_c_level_with_dictionary_and_target_c_block_size()`, and the
internal `encode_frame_with_dictionary_and_target_c_block_size()` path supports
optimal dictionary strategies (`BtOpt`, `BtUltra`, and `BtUltra2`) by threading
a target-sized `CctxParameters` into the existing dictionary optimal frame
path. Follow-up support now also wires dictionary hash-chain strategies
(`Greedy`, `Lazy`, `Lazy2`, and `BtLazy2`) through a cctx-aware dictionary frame
entry point and the existing target-aware hash-chain block path. Fast and
double-fast dictionary target modes are now wired too, via cctx-aware
dictionary frame entry points and target-aware Fast/DFast ext-dict block
adapters. Combined target+dictionary is now strategy-wired; remaining work is
byte and CPU parity rather than unsupported strategy dispatch.

The benchmark and profiler tools now allow target+dictionary instead of
rejecting it globally. Unsupported target sizes fail explicitly when the
target size is outside C's accepted range. Focused level-16
`repo_Cargo.lock` target 2048 dictionary profiling matched C bytes exactly:
20-run totals were Rust/C `150,180` bytes, and 80-run `perf stat` totals were
Rust/C `600,720` bytes. The 80-run Rust counters were `5,552,023,258`
instructions, `2,340,172,068` cycles, `909,915,589` branches, and
`24,567,705` branch misses versus C API `5,851,987,642` instructions,
`2,508,155,831` cycles, `743,936,771` branches, and `22,454,540` branch
misses. A two-fixture benchmark smoke is
`benchmarks/tmp/dictionary-target-api-smoke-current.csv`: aggregate C `296`
bytes, Rust `304` bytes, gap `+8` bytes (`+2.703%`), with the positive gap on
`generated_go.sum`.

Latest hash-chain target+dictionary smoke after that follow-up:
`benchmarks/tmp/dictionary-target-hashchain-api-smoke-current.csv` covers
level 5 with target 2048 and dictionary across the 5 focused dictionary
fixtures. Aggregate C API bytes were `9074`, Rust bytes were `9063`, gap
`-11` bytes (`-0.121%`), with `4` differing rows and `2` positive Rust rows.
Latest Fast/DFast target+dictionary smoke:
`benchmarks/tmp/dictionary-target-fast-dfast-api-smoke-current.csv` covers
levels 1 and 3 with target 2048 and dictionary across the same 5 focused
dictionary fixtures. Aggregate C API bytes were `19019`, Rust bytes were
`19076`, gap `+57` bytes (`+0.300%`), with `8` differing rows and `7`
positive Rust rows. A broader representative combined-mode artifact is
`benchmarks/tmp/dictionary-target-levels-1-3-5-8-16-22-api-current.csv`.
Across the same 5 fixtures at levels `1,3,5,8,16,22`, aggregate level gaps are
level 1 `+16` bytes (`+0.167%`), level 3 `+41` bytes (`+0.435%`), level 5
`-11` bytes (`-0.121%`), level 8 `+9` bytes (`+0.101%`), level 16 `+9` bytes
(`+0.106%`), and level 22 `+8` bytes (`+0.094%`). The rounded CPU fields are
smoke-only.

Historical note: the next paragraph records the initial inspection before the
DFast CDict-copy fix above. Its conclusion that C used the DFast
`dictMatchState` block compressor for the focused C API target+dictionary
benchmark was later disproved by perf; keep it only as historical context.
`inspect_c_port_blocks` now accepts `--dictionary` and can inspect/decode
dictionary and dictionary+target frames. First structural inspection artifact:
`benchmarks/tmp/dictionary-target-inspect-yarn-l3.txt` for
`generated_yarn.lock` level 3 target 2048 dictionary, the worst positive row.
Rust and C both emitted two compressed blocks with identical source ranges; the
18-byte gap is entirely in block 0. Rust block 0 is `370` bytes with
`59` sequences, regenerated literals `202`, and literal payload `173`; C block
0 is `352` bytes with `41` sequences, regenerated literals `262`, and literal
payload `217`. This points to match-choice/sequence-cost parity, not frame or
block-split dispatch. Fresh C-source check: `ZSTD_CCtx_loadDictionary()` uses
Fast/DFast `dictMatchState` block compressors for this mode, while the current
Rust dictionary frame path still loads the dictionary into the active match
state and compresses through the ext-dict/combined-buffer approximation. In
particular, C fast `dictMatchState` only uses a dictionary match when the
normal prefix match index is invalid. Treat porting Fast/DFast `dictMatchState`
behavior as the next parity target for these remaining combined-mode byte
gaps. Validation passed focused
strategy/public tests, full
`cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd --all-targets -- -D
warnings`, profiler/benchmark release rebuilds, profiler and benchmark smokes,
tool clippy for `profile_c_port`, `profile_c_api`, `benchmark_c_port`, and
`inspect_c_port_blocks`, `cargo test -p zstd-rs-tools --quiet`, `cargo fmt
--check`, and `git diff --check`.

Rejected DFast dictionary-match-state checkpoint: `dfast_frame` temporarily
kept loaded dictionary tables separate from active prefix tables for DFast
dictionary frames and routed loaded-dictionary blocks through an untracked
`dfast_dict_match.rs`, a safe Rust port attempt of C's
`ZSTD_compressBlock_doubleFast_dictMatchState_*` search order. This was the
wrong model for the focused C API target+dictionary benchmark and was removed
after perf proved C uses copied-CDict tables with the extDict block compressor.
The normal dictionary 5-fixture API smoke
`benchmarks/tmp/dictionary-fast-dfast-api-after-dfast-dictmatch.csv` reports
level 1 `+0` bytes and level 3 `+19` bytes (`+0.203%`) versus C. The
target+dictionary smoke
`benchmarks/tmp/dictionary-target-fast-dfast-api-after-dfast-dictmatch.csv`
reports level 1 `+16` bytes (`+0.167%`) and level 3 `+39` bytes (`+0.414%`),
only a 2-byte improvement from the previous `+41` level-3 target gap. The
worst `generated_yarn.lock` level-3 target row is unchanged at Rust `397`
bytes versus C `379`; the fresh inspection artifact
`benchmarks/tmp/dictionary-target-inspect-yarn-l3-after-dfast-dictmatch.txt`
still shows Rust block 0 with `59` sequences versus C `41`. Continue below the
frame/dispatch layer, likely in DFast candidate choice, dictionary table
representation, or sequence-cost parity. Validation passed
`cargo test -p ruzstd strategy_frame_dictionary_target_c_block_size --quiet`,
`cargo test -p ruzstd dfast --quiet`, full `cargo test -p ruzstd --quiet`,
`cargo clippy -p ruzstd --all-targets -- -D warnings`, `cargo fmt --check`,
and `git diff --check`.

Fresh focused profile after the benchmark-tool split: release `profile_c_port`
and `profile_c_api` were rebuilt and run on
`benchmarks/archive/tmp/realworld-100/corpus_z000033` at level 16 for 80 runs.
Rust emitted `36,897,920` bytes and C API emitted `36,864,480` bytes. `perf
stat` counters were Rust `33,579,117,341` instructions, `20,605,748,481`
cycles, `5,451,552,337` branches, and `213,027,639` branch misses; same-session
C API was `34,696,939,913` instructions, `19,146,844,452` cycles,
`4,328,350,102` branches, and `175,228,833` branch misses. Rust remains about
`-3.22%` instructions versus C but about `+25.9%` branches. Fresh branch
profile artifact:
`benchmarks/tmp/perf-z000033-l16-rust-branches-after-benchmark-split.data`.
Top branch self costs were `forward_pass()` at `40.13%` and
`compress_block_opt_with_state_and_ldm()` at `22.43%`.

Rejected follow-up from that profile: replacing
`literal_length_code_transition()`'s full match table with a small <=64 match
plus `is_power_of_two()`/`trailing_zeros()` arithmetic for large literal
lengths preserved focused bytes (`9,224,480` for 20 runs) but regressed the
focused 20-run sample to `8,434,968,214` instructions and `1,371,012,978`
branches versus the fresh kept baseline implied by the 80-run sample near
`8.395B` instructions and `1.363B` branches. It was reverted; keep the current
literal-length transition match unless a new profile gives stronger evidence.

Latest resume investigation after the refactor-validation checkpoint:
`perf record -e branches:u -c 100000` on focused `corpus_z000033` level 16
for 20 Rust runs produced
`benchmarks/tmp/perf-z000033-l16-rust-branches-current.data`. Branch samples
confirmed `forward_pass()` is still the dominant branch source at about
`40.7%` self branch samples, followed by
`compress_block_opt_with_state_and_ldm()` at about `21.9%`. Line-level
inspection again highlighted literal-length increment pricing and parser-node
access in `forward_pass()`, but the obvious pass-local short literal-length
increment cache is already documented as rejected and was not retried. Continue
with generated-code inspection or a fresh profile-backed candidate; do not
reintroduce the rejected increment-cache shape without new evidence.

Rejected follow-up after that branch-profile pass: changing
`refresh_node_reps()` to check `state.opt[cur].litlen` before copying the
current `Optimal`, and then loading only `mlen`/`off` on the match endpoint
path, preserved focused bytes (`9,224,480` for 20 runs) but did not reduce the
focused branch/instruction band. Three 20-run samples were `8,439,203,742`,
`8,438,362,108`, and `8,439,682,077` instructions, with branches still about
`1.372B` to `1.373B`. It was reverted; keep the current whole-node copy in
`refresh_node_reps()` unless a new profile gives a stronger reason.

Latest reviewability cleanup after that branch-profile pass:
`ruzstd/src/encoding/blocks/compressed/sequence_tables/selection.rs` was split
so C fast/cost table-selection policy helpers now live in
`ruzstd/src/encoding/blocks/compressed/sequence_tables/selection/policy.rs`.
The main selection orchestrator is now `488` lines and the policy helper file
is `126` lines. This is intended as a behavior-preserving split for PR review.
Validation passed `cargo fmt --check`, `cargo test -p ruzstd compressed
--quiet`, full `cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd
--all-targets -- -D warnings`, and `git diff --check`.

Latest target-mode porting checkpoint after the sequence-table split: fast and
double-fast no-dictionary frames now route `targetCBlockSize` through the same
target-block/superblock emission path as hash-chain and optimal strategies.
`strategy_frame` no longer rejects levels 1 and 3 for valid target sizes, and
the fast/double-fast block appenders have explicit `BlockEncodeMode` entry
points so normal compression still uses the existing wrappers. Broad one-run C
API target comparisons across 73 real-world fixtures were recorded for levels
1 and 3:
`benchmarks/tmp/target-levels-1-3-api-runs1-after-fast-dfast-target.csv`
for target 2048, `target4096-after-fast-dfast-target.csv` for target 4096, and
`target8192-after-fast-dfast-target.csv` for target 8192. Aggregate byte gaps
versus C API were target 2048: level 1 `-39` bytes (`-0.002%`) and level 3
`-1829` bytes (`-0.115%`); target 4096: level 1 `-48` bytes (`-0.003%`) and
level 3 `-1761` bytes (`-0.111%`); target 8192: level 1 `-50` bytes
(`-0.003%`) and level 3 `-1704` bytes (`-0.108%`). Level 1 had no positive
fixture gaps; level 3 had two tiny positive fixture gaps at each target size,
with worst positives `+57`, `+133`, and `+191` bytes on `corpus_z000050` as
target size increased. Validation passed focused target/fast/double-fast tests,
release rebuilds for `benchmark_c_port`, `profile_c_port`, and
`profile_c_api`, full `cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd
--all-targets -- -D warnings`, `cargo fmt --check`, and `git diff --check`.

Latest reviewability cleanup after the fast/double-fast target checkpoint:
stateful `BlockEncodeMode` adapters now live in
`ruzstd/src/encoding/levels/c_port/fast_block/mode.rs` and
`ruzstd/src/encoding/levels/c_port/dfast_block/mode.rs`, while the
target/superblock handoff lives in sibling `target.rs` files. This brings
`fast_block.rs` to `456` lines and `dfast_block.rs` to `457` lines. The stale
target-mode rejection test name was also updated now that levels 1 and 3 are
ported. Validation passed `cargo fmt --check`, focused
`strategy_frame_target_c_block_size`, `fast`, `dfast`, and `target_block`
tests, full `cargo test -p ruzstd --quiet`, and `cargo clippy -p ruzstd
--all-targets -- -D warnings`. Release benchmark tools were rebuilt, and the
level 1/3 target-2048 byte comparison in
`benchmarks/tmp/target-levels-1-3-api-runs1-after-fast-dfast-target-refactor.csv`
matched the previous checkpoint row-by-row for C and Rust compressed bytes.

Latest broad all-level C API checkpoint after the fast/double-fast target
reviewability split: `benchmarks/tmp/normal-levels-1-22-api-runs1-current.csv`
covers all levels 1 through 22 across the 73 real-world fixtures. Normal-mode
aggregate size gaps versus C range from level 3 `-2589` bytes (`-0.163%`) to
level 22 `+1347` bytes (`+0.103%`). Levels 1 through 5 are smaller than C in
aggregate; levels 6 through 22 are within `+0.103%` at worst. The largest
positive fixture row is focused `corpus_z000033` level 22 at `+1026` bytes
(`427338` Rust versus `426312` C). Focused 80-run perf counters on
`corpus_z000033` show high levels are not the remaining CPU problem: level 18
Rust emitted `34219200` bytes versus C `34144480`, with `62.147B`
instructions, `36.274B` cycles, `10.087B` branches, and `367.7M` branch misses
versus C `74.142B` instructions, `41.624B` cycles, `9.854B` branches, and
`327.8M` branch misses; level 22 Rust emitted `34187040` bytes versus C
`34104960`, with `124.270B` instructions, `55.271B` cycles, `21.092B`
branches, and `420.4M` branch misses versus C `147.908B` instructions,
`68.618B` cycles, `17.838B` branches, and `375.9M` branch misses. Treat the
remaining high-level issue as small compression-size drift plus higher branch
count, not instruction/cycle regression.

Latest all-level target-mode checkpoint: `benchmarks/tmp/target-levels-1-22-api-runs1-target2048-current.csv`
covers all levels 1 through 22 across the same 73 fixtures with
`targetCBlockSize=2048`. Levels 13 through 16 and 18 through 22 are
byte-identical to C on every fixture; level 17 is `-9` bytes in aggregate with
no positive rows. Lower levels remain tiny: level 3 is the largest aggregate
negative gap at `-1829` bytes (`-0.115%`), and the largest positive row is
`corpus_z000050` level 3 at `+57` bytes. This supersedes older notes that
target-mode fast paths were unported.

Latest kept high-level compression-size change after the all-level checkpoint:
`BlockCompressionConfig::for_c_strategy()` now uses the existing optimal-depth
Huffman search (`HuffmanTableSearch::AllSections`) for strategies
`BtUltra`/`BtUltra2` (`strategy >= 8`) while preserving the cheaper heuristic
for `BtOpt` and below. This matches C's `HUF_flags_optimalDepth` threshold
without reintroducing the rejected level-16 `BtOpt` optimal-depth dispatch.
The diagnostic tool `inspect_c_port_blocks` now also prints
`source_aligned_deltas`, grouping blocks by decompressed source range so
post-split boundary differences do not dominate the block-delta view.

Validation for that change: focused compressed tests passed, full
`cargo test -p ruzstd --quiet` passed, `cargo clippy -p ruzstd --all-targets
-- -D warnings` passed, release benchmark tools rebuilt, and
`cargo fmt --check`/`git diff --check` passed. The broad normal artifact is
`benchmarks/tmp/normal-levels-1-22-api-runs1-after-btultra-huff-optdepth.csv`.
Compared with the previous all-level artifact, aggregate gaps improved at
levels 13 through 22 and levels 1 through 12 were unchanged. Key high-level
gaps: level 18 improved from `+1278` to `+183` bytes, level 19 from `+1057` to
`-77`, level 20 from `+1043` to `-75`, level 21 from `+1205` to `+143`, and
level 22 from `+1347` to `+265`. Focused `corpus_z000033` level 22 improved
from `+1026` bytes to `+273` bytes (`426585` Rust versus `426312` C), and the
literal payload gap dropped from `+1162` bytes to `+401` bytes. Focused 80-run
perf after the change: level 18 Rust emitted `34160240` bytes versus C
`34144480`, with `64.669B` instructions, `38.093B` cycles, `10.577B`
branches, and `380.5M` branch misses versus C `74.135B` instructions,
`41.318B` cycles, `9.852B` branches, and `327.7M` branch misses; level 22 Rust
emitted `34126800` bytes versus C `34104960`, with `126.763B` instructions,
`55.769B` cycles, `21.577B` branches, and `431.6M` branch misses versus C
`147.903B` instructions, `68.341B` cycles, `17.837B` branches, and `376.2M`
branch misses. Target-mode high levels stayed stable in
`benchmarks/tmp/target-levels-13-22-api-runs1-target2048-after-btultra-huff-optdepth.csv`:
levels 13 through 16 and 18 through 22 remain byte-identical to C, and level 17
remains `-9` bytes with no positive rows.

Latest focused level-16 investigation after the high-level Huffman checkpoint:
`inspect_c_port_blocks` now prints a `source_aligned_summary` before the top
source-aligned deltas. This totals the same-source-range Rust-vs-C byte delta,
absolute delta, and grouped block counts, making it easier to distinguish true
entropy drift from shifted split boundaries. For focused `corpus_z000033`
level 16, the artifact
`benchmarks/tmp/inspect-z000033-l16-current/source-aligned.txt` shows Rust
`461224` bytes versus C `460806` (`+418`). The source-aligned summary is
`groups=128,total_delta=418,abs_delta=1006,rust_groups=131,c_groups=132`.
Largest same-source-range deltas are small and include wins, e.g.
`772352..786432` is `-114` bytes and `96323..111546` is `-66` bytes, while the
largest positive same-range deltas are only `+24`, `+23`, and `+23` bytes in
the top rows. Do not chase the large index-aligned `largest_deltas` for this
case without source alignment; they are dominated by the first source-boundary
mismatch at block 33 and subsequent post-split boundary shifts. Existing
runtime tuning probes for exact sequence modes / all-section Huffman /
repeat-table thresholds are not valid for `for_c_strategy()` because that path
does not apply file-type tuning overrides.

Latest reviewability cleanup after the level-16 source-aligned diagnosis:
`tools/src/bin/inspect_c_port_blocks.rs` was split so comparison and
source-aligned delta reporting now lives in
`tools/src/bin/inspect_c_port_blocks/comparison.rs`. The main binary is now
`434` lines and the comparison module is `339` lines. Validation passed
`cargo fmt --check`, release rebuild of `inspect_c_port_blocks`,
`cargo clippy -p zstd-rs-tools --bin inspect_c_port_blocks -- -D warnings`,
and a smoke run on focused `corpus_z000033` level 22 confirmed
`source_aligned_summary,groups=34,total_delta=273,abs_delta=461,rust_groups=44,c_groups=48`
is still emitted.

Latest target-mode surface cleanup after the inspect-tool split: the public
`compress_slice_c_level_with_target_c_block_size()` doc comment and the
`profile_c_port`, `benchmark_c_port`, and `inspect_c_port_blocks` error
messages no longer say target mode may be unported for the resolved level.
All no-dictionary levels 1 through 22 now route to an implementation; the
helper returns `None` only when `targetCBlockSize` is outside C's accepted
range.

Latest all-target-size validation after the target-mode surface cleanup:
all-level C API target-mode comparisons now cover target sizes 2048, 4096, and
8192 across all no-dictionary levels 1 through 22 and the 73 real-world
fixtures. New artifacts:
`benchmarks/tmp/target-levels-1-22-api-runs1-target4096-current.csv` and
`benchmarks/tmp/target-levels-1-22-api-runs1-target8192-current.csv`, alongside
the existing `target2048-current.csv`. At target sizes 4096 and 8192, levels
13 through 22 are byte-identical to C on every fixture. At target 2048, levels
13 through 16 and 18 through 22 are byte-identical, while level 17 is `-9`
bytes in aggregate with no positive rows. Lower levels remain tiny and usually
smaller than C in aggregate: the worst positive rows across all three target
sizes are `corpus_z000050` level 3 at `+191` bytes for target 8192, `+133`
bytes for target 4096, and `+57` bytes for target 2048.

Latest kept CPU changes: the no-dictionary optimal binary-tree walks in
`ruzstd/src/encoding/levels/c_port/opt_match/tree.rs` now carry the
`nb_compares` and match-low checks in the `while` condition,
`forward_pass()` in
`ruzstd/src/encoding/levels/c_port/opt_parser/forward.rs` now carries the
`ZSTD_OPT_NUM` guard in the main loop condition, and
`HuffmanTable::build_smallest_from_counts_with_stream_counts()` now skips
max-bit candidates that can only reproduce the already evaluated base Huffman
table while `base_code_lengths()` carries its largest bit length for the
candidate bounds, no-dictionary optimal repcode collection now reuses the
already-computed `window_low` bound from the binary-tree path, and the normal
no-LDM optimal parser path now calls a no-cursor match collector instead of
threading `Option<&mut LdmOptCursor>` through the hot no-LDM match calls, and
`select_path()` now updates the last path entry directly instead of checking
`path.last_mut()` in the reconstruction loop, and the post-split estimator now
uses the C-like block-split Huffman search mode, and
`BlockCompressionConfig::for_c_strategy()` now uses the C-like normal Huffman
search heuristic rather than the Rust file-type small-table search,
pre-split's rate-1 2-byte fingerprint recorder now uses a safe rolling
two-byte value instead of reloading both bytes for every adjacent sample,
`Fingerprint::merge()` now reads the other event table by reference instead of
copying the full fixed array before merging, and the
optimal parser now threads a compile-time no-dict/ext-dict split through match
collection so the no-dictionary binary-tree collector calls
`collect_repcode_matches_no_dict()` directly while ext-dict still uses the
generic repcode path, and the optimal parser now threads a compile-time
loaded-dictionary split through match collection so focused no-dict/no-loaded
frames avoid loaded-dictionary window-bound logic in the tree loops, and the
no-LDM forward-pass call now passes `None` directly instead of reborrowing the
LDM cursor option, and
`OptPriceState::dynamic_lit_length_increment_price_unchecked()` now handles
literal lengths `1..=15` as direct adjacent-code deltas before falling back to
the sparse transition helper, and `refresh_node_reps()` now copies the current
and previous parser nodes into locals before writing the updated repcode
history, and `raw_literal_cost()` now borrows only the parser price state and
literal-price cache instead of the whole optimal block state, and the optimal
encoder now reuses a persistent `EstimateScratch` for post-split block-size
estimation through `OptBlockState`, and the optimal no-dictionary/ext-dictionary
block encoders now pre-size their per-block output buffer to the block length
plus the zstd block header, and `prepare_from_greedy_output()` now reserves the
prepared literal stream to the exact emitted literal count instead of the whole
source block length, and the optimal encoder now recycles the prepared-block
literal and sequence vectors through `OptBlockState` after target/split/normal
emission, and the optimal frame loop now recycles the per-block encoded byte
buffer through `OptBlockState` after appending it to the final frame. Do not
treat these changes as pending.

Rejected follow-up after the block-byte-reuse checkpoint: threading a reusable
`CompressedBlockScratch` through optimal normal and post-split emission, so
`compress_prepared_block_with_stats()` could reuse its encoded sequence vector,
preserved focused bytes (`9,224,480` for 20 runs) but did not improve the
focused CPU band. Three focused 20-run samples were `8,459,276,213`,
`8,459,668,496`, and `8,460,628,884` instructions with branch counts around
`1.379B`, versus the kept `8.432B` to `8.469B` band and a restored-code sample
of `8,436,762,404` instructions and `1,370,643,323` branches. It was reverted;
do not retry compressed-block sequence scratch threading without a new profile
signal.

Rejected follow-up after the same checkpoint: adding
`OptMatchTable::last_value()` and using it at the longest-match call sites
preserved focused bytes (`9,224,480` for 20 runs) but did not improve the
focused CPU band enough to justify keeping the helper. Focused 20-run samples
were `8,435,314,851`, `8,435,069,033`, and `8,432,093,200` instructions with
branch counts around `1.370B`, versus the restored-code sample of
`8,436,762,404` instructions and `1,370,643,323` branches. It was reverted;
keep the current `matches.get(match_count - 1)` shape unless a new profile
points back there.

Fresh post-revert focused baseline from the same checkpoint:
`profile_c_port` emitted `36,897,920` bytes for 80 runs on
`corpus_z000033` level 16 with `33,586,353,604` instructions,
`19,495,744,652` cycles, `5,449,536,019` branches, and `213,324,059` branch
misses. Same-session `profile_c_api` emitted `36,864,480` bytes with
`34,698,155,418` instructions, `19,011,420,718` cycles, `4,328,407,208`
branches, and `175,420,015` branch misses. The focused size gap remains
`+0.091%`, Rust is about `-3.20%` instructions versus C, and the branch-count
gap remains about `+25.9%`. Continue treating branch/control-flow shape and
cycles as the residual focused CPU work.

Latest kept correctness cleanup after that baseline: literal-section size
fallbacks in `ruzstd/src/encoding/blocks/compressed/literals.rs` and the
C-port superblock literal writer now use explicit invariant `panic!` messages
instead of `unimplemented!("too many literals")`. Legal zstd blocks are bounded
to 128 KiB, below the raw/RLE and compressed-literals size-field limits, so
these arms are invariant violations rather than missing port work. Added tests
for max-block raw/RLE literal headers and the max-block compressed literal size
format. Validation passed the three focused new tests, the compressed-block
test module, `cargo fmt --check`, `git diff --check`, full `cargo test -p
ruzstd --quiet`, and `cargo clippy -p ruzstd --all-targets -- -D warnings`.

Latest kept unsafe-reduction cleanup after that baseline:
`ruzstd/src/encoding/levels/c_port/row_match.rs` now uses safe slice/index
access for row-table slice creation, candidate lookup, and row insertion/update
writes instead of `from_raw_parts()` and `get_unchecked()` for those table
accesses. Remaining unsafe in that file is limited to x86 prefetch/SSE2 row
mask loads and unaligned scalar row-mask reads. The affected-path broad smoke
`benchmarks/tmp/normal-level5-api-runs1-after-row-safe-indexing.csv` passed
decode validation across 73 fixtures at level 5; Rust total output was
`1,555,507` bytes versus C API `1,557,737` bytes (`-2,230`, `-0.143%`), with
largest positive fixture gap `+27` bytes on `corpus_z000057`. Validation passed
row-match tests, greedy tests, strategy-frame tests, `cargo fmt --check`,
`cargo clippy -p ruzstd --all-targets -- -D warnings`, full `cargo test -p
ruzstd --quiet`, `git diff --check`, and release rebuilds for
`benchmark_c_port` and `profile_c_port`.

Latest kept unsafe-centralization cleanup after the row safe-indexing
checkpoint: duplicated unaligned little-endian read helpers in fast,
double-fast, hash-chain, match-count, and row-match code now route through
`ruzstd/src/encoding/levels/c_port/unaligned.rs`. This keeps the intentional
unsafe `ptr::read_unaligned` operations in one documented module while
preserving the existing helper exports used by sibling modules. Remaining
unsafe occurrences under `c_port` at that checkpoint were the three shared
unaligned reads plus the row matcher's x86 prefetch/SSE2 row-mask blocks.
Focused
`corpus_z000033` level-16 bytes stayed unchanged (`9,224,480` for 20 runs and
`36,897,920` for 80 runs). Three focused 20-run samples were
`8,435,434,022`, `8,435,255,036`, and `8,429,928,377` instructions with
branches around `1.370B`. The focused 80-run Rust sample was
`33,579,613,394` instructions, `19,120,891,902` cycles, `5,449,333,509`
branches, and `213,485,253` branch misses. The level-5 affected-path broad
artifact is
`benchmarks/tmp/normal-level5-api-runs1-after-unaligned-read-centralization.csv`;
it is byte-identical to the previous row-safe-indexing artifact and totals
Rust `1,555,507` bytes versus C API `1,557,737` bytes (`-2,230`, `-0.143%`)
across 73 fixtures. Validation passed focused fast, double-fast, row-match,
opt-match, and match-count tests, full `cargo test -p ruzstd --quiet`, clippy
for all `ruzstd` targets with warnings denied, `cargo fmt --check`, `git diff
--check`, release rebuilds for `profile_c_port` and `benchmark_c_port`, the
focused perf samples, and the level-5 broad smoke.

Latest kept unsafe-boundary cleanup after the unaligned-read centralization:
the row matcher's x86 prefetch and SSE2 row-mask intrinsics now live in
`ruzstd/src/encoding/levels/c_port/x86.rs`, so `row_match.rs` has no remaining
unsafe blocks. The remaining unsafe occurrences under `c_port` are centralized
in `unaligned.rs` and `x86.rs`. The level-5 affected-path broad artifact
`benchmarks/tmp/normal-level5-api-runs1-after-x86-helper-isolation.csv` is
byte-identical to the prior unaligned-read centralization artifact and totals
Rust `1,555,507` bytes versus C API `1,557,737` bytes (`-2,230`, `-0.143%`)
across 73 fixtures. Focused `corpus_z000033` level-16 bytes stayed unchanged
(`9,224,480` for 20 runs); the focused 20-run sample was `8,435,296,554`
instructions, `5,040,718,936` cycles, `1,370,685,814` branches, and
`54,651,059` branch misses. Validation passed row-match tests, full
`cargo test -p ruzstd --quiet`, clippy for all `ruzstd` targets with warnings
denied, `cargo fmt --check`, `git diff --check`, release rebuilds for
`profile_c_port` and `benchmark_c_port`, the level-5 broad smoke, and the
focused perf smoke.

Latest kept safety-coverage cleanup after the x86 helper isolation:
`ruzstd/src/encoding/levels/c_port/unaligned.rs` now has direct unit tests for
little-endian interpretation and unaligned offsets for `read16`, `read32`, and
`read64`. This covers the centralized unsafe read boundary itself rather than
only its match-finder callers. Validation passed the focused unaligned-read
tests, match-count tests, row-match tests, fast tests, full `cargo test -p
ruzstd --quiet` (`647` passed, `5` ignored), clippy for all `ruzstd` targets
with warnings denied, `cargo fmt --check`, and `git diff --check`.

Latest kept x86 safety-coverage cleanup after that: `x86.rs` now has direct
unit tests for the raw SSE2 row tag mask helper across 16-, 32-, and 64-byte
row widths, plus a no-match case. This covers the intrinsic wrapper itself
instead of only the rotated row matcher path. Validation passed the focused
`row_tag_match_mask`, `row_match_mask`, and `row_match` tests, full `cargo
test -p ruzstd --quiet` (`649` passed, `5` ignored), clippy for all `ruzstd`
targets with warnings denied, `cargo fmt --check`, and `git diff --check`.

Latest kept target-block reviewability cleanup after the x86 safety coverage:
single-sub-block target-compressed-size emission helpers moved from
`target_block.rs` into `target_single.rs`. The target-block adapter now keeps
the candidate ordering and fallback policy in one file (`455` lines), while the
new helper module owns the Huffman/basic single-sub-block emission mechanics
(`202` lines). This is a pure module split; no benchmark rerun was needed.
Validation passed `cargo fmt --check`, focused `target_block` and
`strategy_frame` tests, full `cargo test -p ruzstd --quiet` (`649` passed,
`5` ignored), `cargo clippy -p ruzstd --all-targets -- -D warnings`, and
`git diff --check`.

Latest kept optimal-parser reviewability cleanup after the target-block split:
match-collection dispatch and LDM match-candidate integration moved from
`opt_parser.rs` into `opt_parser/matches.rs`. The main parser file now keeps
the public no-dict/ext-dict entry points, min-match/strategy dispatch, and the
optimal parse loop (`457` lines), while `matches.rs` owns the mls/ext-dict/
loaded-dict match collection matrix (`222` lines). This is a pure module split;
no benchmark rerun was needed. Validation passed `cargo fmt --check`, focused
`opt_parser`, `opt_match`, and `strategy_frame` tests, full `cargo test -p
ruzstd --quiet` (`649` passed, `5` ignored), `cargo clippy -p ruzstd
--all-targets -- -D warnings`, and `git diff --check`.

Latest kept optimal-match reviewability cleanup after the parser split:
`OptMatch`/`OptMatchTable` moved from `opt_match.rs` into
`opt_match/table.rs`, and the `BtMatchRequest` descriptor moved into
`opt_match/request.rs`. The main optimal-match module now keeps the match
collection entry points, repcode helpers, and bounds logic (`492` lines),
while `tree.rs` keeps the binary-tree walks. This is a pure type/module split;
no benchmark rerun was needed. Validation passed `cargo fmt --check`, focused
`opt_match`, `opt_parser`, and `strategy_frame` tests, full `cargo test -p
ruzstd --quiet` (`649` passed, `5` ignored), `cargo clippy -p ruzstd
--all-targets -- -D warnings`, and `git diff --check`.

Latest kept price/greedy reviewability cleanup after the optimal-match split:
weight/scaling helpers and `OptLevel` helper methods moved from `opt_price.rs`
into `opt_price/weights.rs`, leaving the main price-state API at `476` lines.
The greedy block special/raw-block shortcut moved from `greedy_block.rs` into
`greedy_block/special.rs`, leaving the main greedy block adapter at `482`
lines. These are pure helper/module splits; no benchmark rerun was needed.
Validation passed `cargo fmt --check`, focused `greedy`, `opt_price`,
`opt_parser`, `target_block`, and `strategy_frame` tests, full `cargo test -p
ruzstd --quiet` (`649` passed, `5` ignored), `cargo clippy -p ruzstd
--all-targets -- -D warnings`, and `git diff --check`. At this checkpoint all
production files under `ruzstd/src/encoding/levels/c_port` are within the
3-500 line reviewability target; remaining larger files there are test modules.

Latest kept unsafe-boundary guard after the production file-size cleanup:
former unsafe call-site modules now carry `#![forbid(unsafe_code)]`:
`row_match.rs`, `fast_helpers.rs`, `dfast_helpers.rs`,
`hash_chain_match.rs`, and `match_count.rs`. A fresh unsafe audit shows actual
unsafe blocks under `c_port` remain centralized in `unaligned.rs` and `x86.rs`;
row prefetch calls and unaligned reads now go through safe wrappers from those
modules. Validation passed `cargo fmt --check`, focused `row_match`,
`match_count`, `fast`, `dfast`, `greedy`, and `strategy_frame` tests, full
`cargo test -p ruzstd --quiet` (`649` passed, `5` ignored), `cargo clippy -p
ruzstd --all-targets -- -D warnings`, and `git diff --check`.

Latest kept post-refactor benchmark validation after the unsafe-boundary guard:
release `profile_c_port`, `profile_c_api`, and `benchmark_c_port` were rebuilt
after the module-split and safety-lint changes. Focused
`corpus_z000033` level-16 smoke stayed byte-stable: Rust emitted `461,224`
bytes for one run and `36,897,920` bytes for 80 runs; C API emitted `460,806`
bytes for one run and `36,864,480` bytes for 80 runs. Focused 80-run perf
counters were Rust `33,592,986,113` instructions, `22,961,284,772` cycles,
`5,467,042,819` branches, and `213,671,617` branch misses; same-session C API
was `34,703,969,060` instructions, `23,021,126,942` cycles, `4,329,760,869`
branches, and `175,666,604` branch misses. The broad one-run API artifact is
`benchmarks/tmp/refactor-validation-normal-levels-8-16-19-api-runs1.csv`; it
is byte-identical in aggregate to the previous block-byte-reuse artifact:
level 8 `+277` bytes (`+0.018%`), level 16 `+666` bytes (`+0.048%`), and
level 19 `+1057` bytes (`+0.081%`) versus C across 73 fixtures. Worst positive
gaps remain `corpus_z000033`: `+319`, `+418`, and `+724` bytes at levels 8,
16, and 19 respectively. Validation also passed `cargo fmt --check`, full
`cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd --all-targets -- -D
warnings`, and `git diff --check`.

Latest kept CPU change after the C-strategy Huffman-search checkpoint: the
no-dictionary optimal binary-tree path in
`ruzstd/src/encoding/levels/c_port/opt_match/tree.rs` now uses a compile-time
`EXT_DICT` specialization, so no-dict compression bypasses
`collect_repcode_matches()` and calls `collect_repcode_matches_no_dict()`
directly while ext-dict still uses the generic repcode dispatcher. Focused
`corpus_z000033` level-16 bytes stayed unchanged (`9,224,480` for 20 runs and
`36,897,920` for 80 runs). Final focused 20-run samples were `9.113B` and
`9.109B` instructions, versus a fresh pre-change `9.408B` sample. The focused
80-run sample was `35,672,501,984` instructions, `26,222,223,890` cycles,
`5,792,228,377` branches, and `243,530,382` branch
misses. Same-session C API was `36,864,480` bytes, `34,714,927,527`
instructions, `23,128,216,291` cycles, `4,331,862,481` branches, and
`177,059,972` branch misses. Final-code profile artifact:
`benchmarks/tmp/perf-z000033-l16-rust-after-nodict-repcode-specialized.data`.

Latest kept CPU change after the no-dict repcode specialization: the same
parser/match path now also carries a compile-time `LOADED_DICT` specialization.
Normal no-dictionary/no-loaded frames use the plain window-low calculation in
the binary-tree update and match-collection loops, while dictionary and ext-dict
paths still use the loaded-dictionary bound logic. Focused `corpus_z000033`
level-16 bytes stayed unchanged (`9,224,480` for 20 runs and `36,897,920` for
80 runs). Three focused 20-run samples were `9.074B`, `9.072B`, and `9.070B`
instructions. The focused 80-run sample was `35,509,415,821` instructions,
`21,825,122,189` cycles, `5,825,392,946` branches, and `240,285,767` branch
misses. Same-session C API was `36,864,480` bytes, `34,714,791,138`
instructions, `24,165,225,995` cycles, `4,331,728,208` branches, and
`177,645,328` branch misses. Final-code profile artifact:
`benchmarks/tmp/perf-z000033-l16-rust-after-loaded-bound-specialized.data`.

Rejected follow-up after the loaded-bound specialization: changing repcode
collection to return the best-possible stop decision directly, instead of
calling `should_stop_after_best_match()` after collection, preserved focused
bytes but regressed focused 20-run instruction samples to about `9.134B` to
`9.137B` (`9,137,329,399`, `9,134,916,040`, `9,134,387,407`) versus the kept
`9.070B` to `9.074B` band. It was reverted; keep the current helper shape
unless a new profile gives a stronger reason.

Latest kept tiny CPU cleanup after the loaded-bound specialization: the
no-LDM optimal parser call into `forward_pass()` now passes `None` directly,
and only the LDM specialization reborrows `ldm_cursor.as_deref_mut()`. Focused
bytes stayed unchanged (`9,224,480` for 20 runs and `36,897,920` for
80 runs). Three focused 20-run samples were `9.073B`, `9.070B`, and `9.069B`
instructions. The focused 80-run sample was `35,509,335,228` instructions,
`21,775,128,903` cycles, `5,825,740,531` branches, and `239,986,812` branch
misses. Treat this as neutral-to-tiny cleanup rather than a material gap
closure. Final-code profile artifact:
`benchmarks/tmp/perf-z000033-l16-rust-after-forward-ldm-cursor-boundary.data`.

Rejected follow-up after the forward LDM-cursor boundary checkpoint: adding
separate code-only LL/ML lookup tables for `literal_length_code()` and
`match_length_code()` in `sequence_codes.rs` preserved focused bytes
(`9,224,480` for 20 runs) but regressed focused 20-run instruction samples to
`9,145,930,272` and `9,148,757,349` versus the kept `9.069B` to `9.073B`
band. It was reverted; keep the current tuple-table helper shape unless a new
profile gives stronger evidence.

Rejected follow-up from the same checkpoint: adding an `OptMatchTable::push_parts()`
helper and routing the no-dictionary binary-tree match recording through direct
field writes preserved focused bytes but did not improve the current CPU band.
Three focused 20-run samples were `9,070,970,688`, `9,066,929,693`, and
`9,080,035,484`; the focused 80-run sample was `35,515,458,633`
instructions versus the kept `35,509,335,228`. It was reverted; keep the
current `matches.push(OptMatch { .. })` shape unless a new profile says
otherwise.

Rejected follow-up after that: changing normal `select_path()` reconstruction
to write the selected forward path in-place into `state.opt`, while keeping the
large-match scratch path unchanged, preserved focused bytes (`9,224,480` for
20 runs and `36,897,920` for 80 runs) but regressed two focused 20-run
instruction samples to `9,087,576,512` and `9,080,310,676` versus the kept
`9.069B` to `9.073B` band. It was reverted; keep the separate `path` plus
`path.reverse()` shape unless a new profile gives stronger evidence.

Latest kept CPU change after those rejections: non-row `GreedyMatchState`
chain tables now allocate one extra dummy entry, and the no-dictionary optimal
binary-tree cleanup writes in `opt_match/tree.rs` use that real dummy slot
instead of carrying a `TREE_SLOT_NONE` sentinel plus conditional final writes.
This mirrors C's `dummy32` cleanup sink without unsafe pointers. Focused bytes
stayed unchanged (`9,224,480` for 20 runs and `36,897,920` for 80 runs).
Three focused 20-run instruction samples were `8,955,119,650`,
`8,948,194,542`, and `8,948,996,934`. The focused 80-run sample was
`35,012,763,886` instructions, `22,943,765,039` cycles, `5,715,690,317`
branches, and `240,853,109` branch misses. Same-session C API was
`36,864,480` bytes, `34,718,897,835` instructions, `24,417,110,177` cycles,
`4,332,511,818` branches, and `178,307,441` branch misses. Final-code profile
artifact: `benchmarks/tmp/perf-z000033-l16-rust-after-chain-dummy-slot.data`.
The broad one-run API artifact is
`benchmarks/tmp/normal-levels-8-16-19-api-runs1-after-chain-dummy-slot.csv`;
byte totals are unchanged from the previous resume artifact: level 8 is `+277`
bytes (`+0.018%`), level 16 is `+666` bytes (`+0.048%`), and level 19 is
`+1057` bytes (`+0.081%`) versus C across 73 fixtures. Validation passed
`cargo fmt --check`, focused greedy-state/opt-match/opt-parser tests, full
`cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd --all-targets -- -D
warnings`, release rebuilds for `profile_c_port`, `profile_c_api`, and
`benchmark_c_port`, broad byte comparison, and `git diff --check`.

Rejected follow-up after the dummy-slot checkpoint: hoisting `ip +
match_length` into a local `ip_match` in the two no-dictionary binary-tree
loops preserved focused bytes (`9,224,480` for 20 runs) but immediately
regressed the first focused 20-run instruction sample to `9,073,101,669`
versus the kept `8.95B` band. It was reverted; keep the current repeated
`ip + match_length` expression shape unless a new profile gives stronger
evidence.

Fresh no-edit focused baseline from the July 17 resume: release profiler
binaries were current, `profile_c_port` emitted `9,224,480` bytes for 20 runs
and `36,897,920` bytes for 80 runs on `corpus_z000033` level 16, while
`profile_c_api` emitted `9,216,120` bytes for 20 runs and `36,864,480` bytes
for 80 runs. The 80-run Rust counters were `34,995,745,307` instructions,
`21,004,415,236` cycles, `5,712,223,476` branches, and `238,741,615` branch
misses. Same-session C API counters were `34,698,563,324` instructions,
`19,177,252,159` cycles, `4,328,684,308` branches, and `175,513,556` branch
misses. The focused instruction gap is about `+0.86%`; branch count and cycles
remain the more visible residual gap.

Rejected normal-mode Huffman-depth experiment after the pre-split merge-copy
checkpoint: enabling the existing C-style optimal-depth Huffman table builder
for btopt+ normal literal sections found a real compression lever but was too
expensive in its current Rust form. The all-sections variant improved focused
`corpus_z000033` level-16 size to `460,015` bytes, beating the C API's
`460,806` bytes, and broad one-run API totals moved to level 16 `-880` bytes
and level 19 `-17` bytes versus C. However, focused 20-run instructions
regressed to `9,580,409,250` versus the kept `8.88B` band and same-session C
at `8,714,318,613`. A sparse-alphabet gate still produced `460,940` bytes and
`9,249,696,103` instructions. Both variants were reverted. Do not re-enable
normal-mode optimal Huffman depth without first making
`build_c_optimal_depth_from_counts()` much cheaper or proving a narrower gate.

Follow-up kept after that rejection: `HuffmanTable::build_c_optimal_depth_from_counts()`
now builds the base Huffman tree once and reuses it while probing candidate
depths, instead of rebuilding the sorted tree for every table log. A regression
test compares the optimized path with the old rebuild-each-depth behavior. This
preserves bytes and helps target/superblock code that already uses optimal
depth. Re-testing the full normal-mode optimal-depth dispatch after this change
still produced `460,015` bytes on focused `corpus_z000033` level 16, but
focused 20-run instructions were still `9,417,604,355` versus same-session C
at `8,795,793,446` and the kept Rust baseline near the `8.88B` band. The
normal dispatch experiment was reverted again; keep only the builder reuse
optimization.

Latest kept CPU change after the fresh no-edit baseline:
`OptPriceState::update_stats()` now skips the compressed-literal frequency loop
entirely when `lit_length == 0`, while still updating literal-length, offset,
and match-length statistics. This preserves the C price model and avoids
building an empty literal slice for the many zero-literal optimal-parser
sequences.
Focused bytes stayed unchanged (`9,224,480` for 20 runs and `36,897,920` for
80 runs). Three focused 20-run instruction samples were `8,943,075,115`,
`8,942,462,061`, and `8,942,590,605`. The focused 80-run sample was
`34,982,048,359` instructions, `20,555,284,514` cycles, `5,708,600,270`
branches, and `238,547,710` branch misses. Same-session C API was
`36,864,480` bytes, `34,697,125,531` instructions, `18,794,718,268` cycles,
`4,328,280,285` branches, and `175,395,203` branch misses. Validation passed
`cargo fmt --check`, focused opt-price and opt-parser tests, full
`cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd --all-targets -- -D
warnings`, and a release `profile_c_port` rebuild.

Rejected follow-up after the zero-literal `update_stats()` checkpoint:
splitting `forward_pass()` into a match-collection loop bounded by
`ip + cur <= ilimit` plus a tail literal-collapse loop preserved focused bytes
(`9,224,480` for 20 runs) but regressed the first focused 20-run instruction
sample to `9,080,890,905` versus the kept `8.942B` to `8.943B` band. It was
reverted; keep the current in-loop `if inr > ilimit { cur += 1; continue; }`
shape unless a fresh profile gives a stronger reason.

Latest kept CPU change after the zero-literal `update_stats()` checkpoint: the
two no-dictionary binary-tree branch decisions in `opt_match/tree.rs` now load
`src[ip + match_length]` into a local `current_byte` before comparing it with
the candidate match byte. This is narrower than the rejected `ip_match` index
hoist and keeps the proven match-count and end-check expression shapes. Focused
bytes stayed unchanged (`9,224,480` for 20 runs and `36,897,920` for 80 runs).
Three focused 20-run instruction samples were `8,918,956,029`,
`8,917,256,880`, and `8,917,844,516`. The focused 80-run sample was
`34,907,574,328` instructions, `20,866,673,858` cycles, `5,698,471,717`
branches, and `239,276,665` branch misses. Same-session C API was
`36,864,480` bytes, `34,699,413,590` instructions, `18,737,483,139` cycles,
`4,328,755,062` branches, and `175,530,872` branch misses. Validation passed
`cargo fmt --check`, focused opt-match and opt-parser tests, full
`cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd --all-targets -- -D
warnings`, and a release `profile_c_port` rebuild.

Rejected follow-up after the branch-byte `current_byte` checkpoint: also
loading `src[current_match_index + match_length]` into a local `match_byte`
preserved focused bytes but did not improve the current focused instruction
band. Three focused 20-run samples were `8,917,151,713`, `8,917,523,902`, and
`8,922,681,533` versus the kept `8.917B` to `8.919B` band. It was reverted;
keep only the `current_byte` local unless a new profile gives stronger
evidence.

Rejected follow-up after the same checkpoint: adding an
`OptPriceState::update_stats()` repeat-code fast path that mapped offBase
values 1-3 directly to offset codes preserved focused bytes but regressed the
focused 20-run instruction band. Samples were `8,918,791,897`,
`8,922,373,720`, and `8,921,882,692` versus the kept `8.917B` to `8.919B`
band. It was reverted; keep the direct `highbit32(off_base)` update-stats
shape unless a fresh profile points back there.

Rejected follow-up after the same checkpoint: adding direct BtOpt/BtUltra
const-specialized dynamic price helpers and routing `forward_pass()` through
them preserved focused bytes (`9,224,480` for 20 runs) but regressed the
focused 20-run instruction band. Samples were `8,919,964,991`,
`8,921,688,733`, and `8,921,417,615` versus the kept `8.917B` to `8.919B`
band. It was reverted; keep the current `OptLevel`-threaded dynamic helper
shape unless a fresh profile gives stronger evidence.

Rejected follow-up from the July 17 continuation after the parser borrow
cleanup: replacing the final safe tail-byte comparisons in `match_count.rs`
with a checked-by-caller `read8()` helper preserved focused bytes
(`9,224,480` for 20 runs) but regressed three focused 20-run instruction
samples to `8,983,615,053`, `8,980,016,221`, and `8,980,931,144` versus the
kept `8.874B` to `8.878B` band. It was reverted; keep the safe single-byte
tail comparisons unless a new profile gives stronger evidence.

Rejected follow-up after that: replacing the decisive safe byte reads in the
two no-dictionary binary-tree loops with an unsafe `get_unchecked()` helper,
guarded by the existing match-end checks, preserved focused bytes
(`9,224,480` for 20 runs) but regressed focused 20-run instruction samples to
`8,908,106,906`, `8,906,476,774`, and `8,907,544,796`. Branch counts were
around `1.431B`, not enough to justify the extra unsafe code or the instruction
regression. It was reverted; keep the current safe byte reads unless a fresh
profile gives a stronger signal.

Latest kept CPU change after the parser borrow cleanup: `OptBlockState` now
owns a reusable `EstimateScratch`, and the no-dictionary/ext-dictionary optimal
post-split paths pass it into `encode_split_block()` instead of constructing a
fresh estimator scratch for every split decision tree. This mirrors C's
reusable temporary workspace while staying in safe Rust. Focused
`corpus_z000033` level-16 bytes stayed unchanged (`9,224,480` for 20 runs and
`36,897,920` for 80 runs). Three focused 20-run instruction samples were
`8,871,865,495`, `8,876,424,982`, and `8,870,265,527`. The focused 80-run Rust
sample was `34,082,041,893` instructions, `24,302,296,491` cycles,
`5,556,270,386` branches, and `227,186,708` branch misses. Same-session C API
was `36,864,480` bytes, `34,717,968,848` instructions, `25,176,232,975`
cycles, `4,332,657,101` branches, and `177,931,971` branch misses. This puts
the focused instruction sample about `-1.83%` below C, while branch count
remains about `+28%`.

Latest kept CPU change after the branch-byte `current_byte` checkpoint:
`Fingerprint::record_hash2()` in `pre_split.rs` now routes the
`SAMPLING_RATE == 1` case through a safe rolling two-byte value. This preserves
the exact hash stream for level-16 pre-split fingerprinting while avoiding a
second adjacent byte reload on every sample. Focused bytes stayed unchanged
(`9,224,480` for 20 runs and `36,897,920` for 80 runs). Three focused 20-run
instruction samples were `8,889,753,086`, `8,894,047,177`, and
`8,890,261,731`. The focused 80-run sample was `34,791,411,780`
instructions, `21,108,074,835` cycles, `5,639,777,323` branches, and
`238,545,892` branch misses. Same-session C API was `36,864,480` bytes,
`34,693,551,015` instructions, `19,048,337,920` cycles, `4,327,672,182`
branches, and `175,343,827` branch misses. The broad normal API artifact is
`benchmarks/tmp/normal-levels-8-16-19-api-after-presplit-rolling.csv`; byte
totals stayed at level 8 `+277` bytes (`+0.018%`), level 16 `+666` bytes
(`+0.048%`), and level 19 `+1057` bytes (`+0.081%`) versus C across
73 fixtures. Validation passed `cargo fmt --check`, focused pre-split and
strategy-frame tests, full `cargo test -p ruzstd --quiet`, `cargo clippy -p
ruzstd --all-targets -- -D warnings`, release rebuilds for `profile_c_port`,
`profile_c_api`, and `benchmark_c_port`, and the broad byte comparison.

Latest kept CPU follow-up after the pre-split rolling hash checkpoint:
`Fingerprint::merge()` now iterates over `other.events.iter().copied()` instead
of zipping with `other.events` by value. That avoids copying the whole
1024-entry fixed array out of the borrowed fingerprint before the merge loop,
matching C's read-from-the-other-buffer shape in safe Rust. Focused bytes stayed
unchanged (`9,224,480` for 20 runs and `36,897,920` for 80 runs). Three
focused 20-run instruction samples were `8,881,179,201`, `8,888,140,513`, and
`8,886,629,934`. The focused 80-run sample was `34,757,167,095`
instructions, `21,543,566,925` cycles, `5,638,915,934` branches, and
`238,294,624` branch misses. Same-session C API was `36,864,480` bytes,
`34,692,450,623` instructions, `18,757,952,155` cycles, `4,327,475,208`
branches, and `175,328,491` branch misses. The broad normal API artifact is
`benchmarks/tmp/normal-levels-8-16-19-api-after-presplit-merge-ref.csv`; byte
totals stayed at level 8 `+277` bytes (`+0.018%`), level 16 `+666` bytes
(`+0.048%`), and level 19 `+1057` bytes (`+0.081%`) versus C across
73 fixtures. Validation passed `cargo fmt --check`, focused pre-split tests,
full `cargo test -p ruzstd --quiet`, clippy for all `ruzstd` targets with
warnings denied, release rebuilds for `profile_c_port`, `profile_c_api`, and
`benchmark_c_port`, and the broad byte comparison.

Rejected follow-ups after the pre-split merge-copy checkpoint: changing
`record_hash2_rate1()` to process the final sample outside the loop removed the
inner `if pos < limit` branch and preserved focused bytes, but did not beat the
kept 80-run checkpoint. Three focused 20-run samples were `8,885,351,925`,
`8,887,788,972`, and `8,881,763,152`; the focused 80-run sample was
`34,761,274,300` instructions, `21,895,797,871` cycles, `5,638,591,834`
branches, and `239,446,970` branch misses versus the kept
`34,757,167,095` instruction checkpoint. It was reverted. Changing
`fingerprint_distance()` from its range iterator/map/sum shape to an explicit
`while` loop also preserved bytes but immediately regressed focused 20-run
instruction samples to `8,899,106,681` and `8,904,636,616`, with higher branch
misses. It was reverted; keep the current iterator distance shape unless a new
profile gives stronger evidence.

Rejected literal-statistics follow-up after the pre-split merge-copy
checkpoint: changing `LiteralStats::from_literals_with_stream_counts()` from
`literals.chunks(split_size).enumerate()` to an explicit four-stream
start/end loop preserved focused bytes but regressed the focused 20-run
instruction/branch band. Samples were `8,896,159,806` and `8,892,897,905`
instructions, with branches around `1.456B`, versus the kept `8.881B` to
`8.888B` instruction band and about `1.449B` to `1.451B` branches. It was
reverted; keep the chunk iterator shape unless a new profile gives stronger
evidence.

Rejected Huffman stream follow-up after the same checkpoint: changing
`HuffmanEncoder::encode_stream()` from `data.iter().rev()` to a counted reverse
index loop preserved focused bytes but did not improve the focused band.
Samples were `8,886,757,164`, `8,888,734,628`, and `8,884,567,000`
instructions, with branch misses around `64.1M` to `64.4M`. It was reverted;
keep the reversed slice iterator unless a new profile gives stronger evidence.

Rejected follow-up after the same checkpoint: changing
`HuffmanEncoder::write_table()` to emit the already-cached table description
with `BitWriter::append_bytes()` instead of byte-by-byte `write_bits()`
preserved focused bytes (`9,224,480` for 20 runs) but did not produce a
defensible CPU win. The append-bytes samples were `8,922,544,583`,
`8,923,295,990`, and `8,920,131,310`; post-revert samples were
`8,925,040,596`, `8,922,236,879`, and `8,917,646,679`. Keep the cached
table-description data, but emit it through `write_bits()` unless a fresh
profile points back to this path.

Rejected C-parity probe after the same checkpoint: clamping the no-dictionary
binary-tree `window_low`/`match_low` values to at least `1`, to mimic C's raw
hash-table zero sentinel, is not valid with Rust's `index + 1` stored-value
layout. It breaks the existing `opt_match` source-index-zero and repcode
boundary tests, so keep the current `window_low` values unless the table
storage representation is deliberately redesigned.

Fresh resume profile and broad benchmark from July 17, 2026: after reverting
the rejected LL/ML code-table and match-recording experiments, the focused
`corpus_z000033` level-16 80-run smoke still emits `36,897,920` bytes. Fresh
Rust counters were `35,509,474,210` instructions, `22,296,520,000` cycles,
`5,825,157,267` branches, and `240,860,614` branch misses. Fresh profile
artifact: `benchmarks/tmp/perf-z000033-l16-rust-resume-20260717.data`.
The delegated broad one-run API benchmark artifact is
`benchmarks/tmp/agent-validation-resume-20260717.csv`: level 8 is `+277`
bytes (`+0.018%`), level 16 is `+666` bytes (`+0.048%`), and level 19 is
`+1057` bytes (`+0.081%`) versus C across 73 real-world fixtures. Worst
positive gaps remain `corpus_z000033`: `+319`, `+418`, and `+724` bytes at
levels 8, 16, and 19 respectively. Treat the rounded broad CPU seconds as
noisy; use focused perf counters for CPU conclusions.

Next resume action: continue after the direct Huffman bucket-subslice
checkpoint. Focused normal-mode level 16 is slightly smaller than C and uses
about `10.32%` fewer instructions, but still executes about `4.81B` hardware
branches versus C's `4.31B` for 80 runs. The batching change reduced
`FSEEncoder::encode_interleaved()` by about 183K Callgrind instructions per
frame, so do not undo it or restore per-symbol BitWriter calls. Fresh artifacts
are `benchmarks/tmp/perf-z000033-l16-rust-after-weight-fse-batching.stat`,
`benchmarks/tmp/perf-z000033-l16-c-api-after-weight-fse-batching.stat`, and
`benchmarks/tmp/callgrind-z000033-l16-rust-after-weight-fse-batching.out`.
The current paired counter artifacts are
`benchmarks/tmp/perf-z000033-l16-rust-after-bucket-subslice.stat` and
`benchmarks/tmp/perf-z000033-l16-c-api-after-bucket-subslice.stat`. A fresh C
branch profile is
`benchmarks/tmp/perf-branches-z000033-l16-c-api-after-weight-fse-batching.data`.
The matching Rust branch profile is
`benchmarks/tmp/perf-branches-z000033-l16-rust-after-bucket-subslice.data`.
At 20 runs, Rust's two inlined parser/match symbols account for about `846M`
branches versus about `821M` across C's `ZSTD_btGetAllMatches_noDict_5`,
`ZSTD_insertBt1`, and `ZSTD_compressBlock_opt0`; the residual there is roughly
`26M`. Continue line-by-line C comparison in parser/match collection and the
remaining Huffman tree/table loops. The old weight-conversion and rank-array
bounds-check hotspots are gone and should not be revisited.
Fresh Callgrind artifacts are
`benchmarks/tmp/callgrind-z000033-l16-rust-after-c-ncount-known-total.out` and
`benchmarks/tmp/callgrind-z000033-l16-c-api-current.out`. They show that Huffman
invocation count is not the problem: Rust performs 569 table builds per frame
(376 estimate and 193 emission), while C performs 648 (456 estimate and 192
emission). Rust costs about 52K Callgrind instructions per build versus roughly
31-32K for C. The tree walk itself is much closer; table-description
finalization and owning allocation remain the larger per-build difference.
Investigate a genuinely new safe representation or ownership shape there, not
another direct-from-codes/weights retry. Do not retry emitted Huffman scratch
reuse: both
the explicit frame-context and `FseTables` ownership shapes regressed by about
40-50M instructions per 20 runs. Do not replace the owning Huffman output
`Vec` with an inline 2 KiB table either. Remaining costs include FSE table
construction/encoding, literal counting, Huffman length limiting/code
assignment, and post-split estimation. Compare subsystem totals before
returning to parser micro-edits.
Do not reintroduce per-stream literal counts where C's combined estimate is
sufficient. Prefer
safe representations that make bounds visible to LLVM; do not introduce
unchecked indexing until the invariant and measured benefit are both strong
enough to justify the unsafe boundary. Do not retry the
rejected LL/ML code-table, direct match-recording, safe in-place
path-reconstruction, parser cache inline, or Huffman table-description emission
shapes without a fresh profile signal. Before changing Huffman code again,
search this file and `ruzstd/src/encoding/levels/c_port/README.md` for
rejected notes around `table_description`, `weights_from_codes`,
`build_from_code_lengths`, `length_limited`, and base code lengths.

Dictionary-port status: attached `dictMatchState` routing covers row-hash
`Greedy`/`Lazy`/`Lazy2`, binary-tree `BtLazy2`, and optimal-parser
`BtOpt`/`BtUltra`/`BtUltra2`. Prepared-CDict retention and repeat-Huffman state
are also validated. Do not treat the older chronological note below that calls
optimal attach variants unported as current state.

Rejected hash-table follow-ups after the table-derived binary-tree mask
checkpoint: deriving primary `hash_log` from the allocated hash-table length
preserved focused bytes but raised a 20-run instruction sample from the kept
`8.416B` band to `8.554B`; it was reverted. Replacing the two primary hash
reads/writes with narrowly wrapped `get_unchecked()` access reduced branches
from about `1.341B` to `1.333B` but raised instructions to
`8.504B`-`8.505B` and showed no reliable cycle win. It was also reverted, so
no new unsafe hash access remains. Do not retry either shape without materially
different profile evidence.

Rejected follow-up after the forward-loop-bound checkpoint: replacing
`ForwardResult.last_stretch: Option<Optimal>` with a C-like `lastStretch` plus
valid flag preserved focused bytes but did not improve the current focused CPU
band. Three 20-run samples were `11.650B`, `11.649B`, and `11.654B`; the
80-run sample was `45,869,289,435` instructions versus the current kept
`45,865,640,102` checkpoint. It was reverted; do not retry without a new
profile signal.

Rejected follow-up after the same checkpoint: adding direct BtOpt dynamic-price
helpers in `OptPriceState` and routing the hot `forward.rs` price calls through
const-generic BtOpt/BtUltra dispatch preserved focused bytes but regressed the
current focused instruction band. Three 20-run samples were `11.728B`,
`11.729B`, and `11.727B` instructions versus the current `11.65B` band. It was
reverted; do not retry this specialization without a new profile signal.

Rejected follow-up from the July 17 resume: changing the initial
`seed_match_prices()` endpoint writes from the current struct update to direct
field mutation preserved focused bytes but did not improve the current focused
instruction band. Three 20-run samples were `11.689B`, `11.687B`, and
`11.683B`; the 80-run sample was `46,006,881,364` instructions versus the
current `46,005,243,303` checkpoint. It was reverted; do not retry without a
new profile signal.

Fresh July 17 profile comparison after that rejection:
`benchmarks/tmp/perf-z000033-l16-rust-current.data` and
`benchmarks/tmp/perf-z000033-l16-c-api-current.data` were recorded with the
same focused `corpus_z000033` level-16 80-run command. Current counters:
Rust `36,793,680` bytes, `46.0B` instructions, `25.5B` cycles, `7.84B`
branches, and `276.8M` branch misses; C API `36,864,480` bytes, `34.7B`
instructions, `19.3B` cycles, `4.33B` branches, and `175.7M` branch misses.
The C profile shows `ZSTD_compressBlock_opt0`, and Rust level dispatch tests
confirm level 16 maps to `BtOpt`, so the focused CPU gap is not a wrong
BtOpt/BtUltra strategy selection. The next useful investigation should stay on
binary-tree match collection / parser branch count, with `count_match_no_dict`
loop variants already documented as tried.

Rejected follow-up after the fresh Rust/C profile: changing
`opt_match/tree.rs` relinks from `stored_value(current_match_index)` to the
already-loaded `match_index as u32` preserved focused bytes but regressed the
focused CPU band. Three 20-run samples were `11.695B`, `11.690B`, and
`11.692B`; the 80-run sample was `46,025,113,943` instructions. It was
reverted; keep the current `stored_value(current_match_index)` form unless a
new profile says otherwise.

Rejected follow-up after the July 17 resume: keeping
`opt_match/tree.rs` tree cursor values as `u32` in the two no-dictionary
binary-tree loops, only widening to `usize` at table/slice indexing points,
preserved focused bytes (`9,198,420` for 20 runs) but clearly regressed the
focused instruction band. Two 20-run samples were `11.493B` and `11.493B`
instructions versus the kept `11.33B` to `11.34B` band. It was reverted; keep
the current immediate `usize` widening shape unless a new profile gives a
stronger reason.

Fresh work-count evidence from the July 17 continuation: one-run Callgrind on
the focused fixture at level 16 produced `434,843,293` Ir for C API and
`564,897,202` Ir for Rust. Both paths processed 86 optimal blocks. C
`ZSTD_compressBlock_opt0` called `ZSTD_btGetAllMatches_noDict_5` `672,736`
times and `ZSTD_btGetAllMatches_noDict_5` called `ZSTD_insertBt1` `186,992`
times. Temporary Rust counters, removed after measurement, showed the same
`672,736` match-collector wrapper calls and the same `186,992`
`insert_bt1_no_dict()` calls; `9,242` Rust collector calls returned at the
skipped-area guard before entering the lower tree helper, matching C's
`ip < nextToUpdate` behavior. The remaining focused CPU gap is therefore not
from extra top-level optimal blocks, extra match-collector calls, or extra
tree-update insert calls; continue looking at per-call generated-code shape,
parser table work, entropy/post-split work, or deeper loop iteration costs.

Latest kept CPU change from the July 17 continuation:
`BlockCompressionConfig::for_c_block_split_estimate()` now switches Huffman
table search to the standard heuristic for the post-split entropy-estimation
path only. This matches C level 16 more closely: C's block-split estimator uses
`ZSTD_buildBlockEntropyStats_literals()` with `HUF_flags_optimalDepth` disabled
for `btopt`, so it builds one normal Huffman table for each split estimate
rather than running the Rust file-type small-table search for every small
partition. Emitted partitions still use the original block config. Focused
`corpus_z000033` level-16 size moved closer to C: Rust changed from
`36,793,680` bytes for 80 runs to `36,812,080`, versus C API `36,864,480`.
Focused 80-run counters improved to `39,191,745,940` instructions,
`23,451,212,641` cycles, `6,336,575,376` branches, and `249,372,902` branch
misses. Same-session C API counters were `34,710,580,784` instructions,
`21,593,230,051` cycles, `4,331,183,411` branches, and `177,317,409` branch
misses. The corrected broad one-run API artifact is
`benchmarks/tmp/normal-levels-8-16-19-api-runs1-after-c-split-estimate-huffman-rebuilt.csv`:
level 8 remains `-560` bytes versus C, level 16 moves to `-737` bytes, and
level 19 moves to `-58` bytes; worst positive gap is `+7` bytes on
`corpus_z000050` level 19. Validation passed `cargo fmt --check`, full
`cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd --all-targets -- -D
warnings`, `git diff --check`, and release rebuilds of `profile_c_port`,
`profile_c_api`, and `benchmark_c_port`.

Latest kept CPU change after the post-split estimate checkpoint:
`BlockCompressionConfig::for_c_strategy()` now sets `HuffmanTableSearch` to
`Heuristic` instead of `FileTypeSmall`. This is more faithful to C strategy
compression: C only enables `HUF_flags_optimalDepth` for `btultra` and above,
so focused level 16 (`btopt`) should not run the Rust file-type small-Huffman
table search for emitted blocks. The focused `corpus_z000033` level-16 output
is now slightly larger than C instead of smaller: Rust `36,897,920` bytes for
80 runs versus C API `36,864,480`. Focused 80-run counters improved to
`36,834,241,992` instructions, `22,029,444,650` cycles, `5,894,194,022`
branches, and `239,865,041` branch misses. This leaves the focused instruction
gap at about 6.1% over the same-session C API `34,710,580,784` instruction
sample. The broad normal one-run API artifact is
`benchmarks/tmp/normal-levels-8-16-19-api-runs1-after-c-strategy-heuristic-huffman.csv`:
level 8 is `+277` bytes (`+0.018%`), level 16 is `+666` bytes (`+0.048%`),
and level 19 is `+1057` bytes (`+0.081%`) versus C, with the largest positive
gap on `corpus_z000033`. Target mode stayed byte-identical across the full
3x3 grid: target sizes 2048, 4096, and 8192 at levels 13, 16, and 19 all
reported `rows=73`, `differing=0`, `positive=0`, and `total_gap=+0`.
The fresh profile artifact is
`benchmarks/tmp/perf-z000033-l16-rust-after-c-strategy-heuristic-huffman.data`;
top self costs are again `forward_pass` at about 36.0% and
`compress_block_opt_with_state_and_ldm` at about 29.8%, followed by much
smaller costs in `base_code_lengths`, pre-split fingerprinting, literal stats,
FSE table building, `OptPriceState::update_stats`, and sequence encoding.
Validation passed `cargo fmt --check`, full `cargo test -p ruzstd --quiet`,
`cargo clippy -p ruzstd --all-targets -- -D warnings`, `git diff --check`,
focused compressed/price/match tests, and release rebuilds of `profile_c_port`
and `benchmark_c_port`.

Rejected follow-up after the same checkpoint: changing
`hash_chain_match::highbit32()` from the current
`u32::BITS - 1 - value.leading_zeros()` form to `value.ilog2()` preserved the
new focused bytes (`9,224,480` for 20 runs) but regressed focused instruction
samples to `10.255B` and `10.257B` for 20 runs, versus the current
`9.400B` band after the C-strategy Huffman-search change. It was reverted; do
not retry the `ilog2()` highbit shape without a new profile signal.

Latest kept match-finder CPU change: the two no-dictionary optimal binary-tree
walks in `opt_match/tree.rs` now put the `nb_compares` and match-low checks in
the `while` condition, matching C's `for (; nbCompares && matchIndex >=
matchLow; --nbCompares)` shape after accounting for Rust's `index + 1` tree
sentinel encoding. Focused `corpus_z000033` level-16 bytes stayed unchanged
(`9198420` for 20 runs, `36793680` for 80 runs). Three focused 20-run samples
were `11.664B`, `11.662B`, and `11.661B`; the focused 80-run sample was
`45,908,122,464` instructions, `25,730,746,419` cycles, `7,662,146,783`
branches, and `278,838,202` branch misses. The broad one-run normal artifact
`benchmarks/tmp/normal-levels-8-16-19-api-runs1-after-tree-loop-condition.csv`
has byte fields identical to the previous
`normal-levels-8-16-19-api-runs1-after-rank-limited-bucket-reduce.csv`
artifact. Target smoke `corpus_z000022` level 19 target 8192 still matches the
C API at `78067` bytes. Validation passed `cargo fmt --check`, focused
opt-match/parser tests, full `cargo test -p ruzstd --quiet`, `cargo clippy -p
ruzstd --all-targets -- -D warnings`, and release profiler rebuild.

Latest kept parser CPU follow-up: `forward_pass()` now carries the
`ZSTD_OPT_NUM` guard in the main loop condition instead of entering the loop
and immediately branching out. This preserves the C parser's bounded-table
invariant while removing a hot branch from the Rust loop shape. Focused
`corpus_z000033` level-16 bytes stayed unchanged (`9198420` for 20 runs,
`36793680` for 80 runs). Three focused 20-run samples were `11.655B`,
`11.652B`, and `11.652B`; the focused 80-run sample was `45,865,640,102`
instructions, `26,469,839,330` cycles, `7,652,782,851` branches, and
`277,699,116` branch misses. The broad one-run normal artifact
`benchmarks/tmp/normal-levels-8-16-19-api-runs1-after-forward-loop-bound.csv`
has byte fields identical to the previous
`normal-levels-8-16-19-api-runs1-after-tree-loop-condition.csv` artifact.
Target smoke `corpus_z000022` level 19 target 8192 still matches the C API at
`78067` bytes. Validation passed `cargo fmt --check`, focused opt-match/parser
tests, full `cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd
--all-targets -- -D warnings`, and release profiler rebuild.

Latest kept Huffman CPU follow-up: when the base Huffman lengths already fit
`MAX_HUFFMAN_BITS`, `build_smallest_from_counts_with_stream_counts()` now
builds the base candidate directly and only searches candidate `max_bits` below
the base tree's largest bit length. `base_code_lengths()` carries that largest
bit length out with the lengths and sorted symbols so the caller does not need
a second symbol scan. Larger candidate limits reproduce the same base table, so
this avoids duplicate length-vector/symbol-vector cloning and duplicate
Huffman table construction without changing the serialized table-description
path. Focused `corpus_z000033` level-16 bytes stayed unchanged (`9198420` for
20 runs, `36793680` for 80 runs). Three final focused 20-run samples were
`11.337B`, `11.339B`, and `11.341B`; the focused 80-run sample was
`44,626,089,276` instructions, `26,884,991,446` cycles, `7,373,300,245`
branches, and `278,152,360` branch misses. The fresh profile artifact is
`benchmarks/tmp/perf-z000033-l16-rust-after-huffman-base-largest-bits.data`.
The broad one-run normal artifact
`benchmarks/tmp/normal-levels-8-16-19-api-runs1-after-huffman-base-largest-bits.csv`
has byte fields identical to
`benchmarks/tmp/normal-levels-8-16-19-api-runs1-after-forward-loop-bound.csv`;
aggregate byte gaps remain level 8 `-560`, level 16 `-967`, and level 19
`-155` bytes against C. Target smoke `corpus_z000022` level 19 target 8192
still matches the C API at `78067` bytes. Validation passed `cargo fmt
--check`, focused Huffman/parser tests, full `cargo test -p ruzstd --quiet`,
a release `profile_c_port` rebuild, broad byte comparison, and the target
smoke.

Latest kept match-bound follow-up: `insert_bt_and_get_all_matches_no_dict()`
already computes the C window low bound before repcode collection, so
`collect_repcode_matches()` now accepts that `window_low` instead of
recomputing the same loaded-dictionary/window bound for the same `ip`. This is
a small C-shape cleanup in the hot no-dictionary optimal match path. Focused
`corpus_z000033` level-16 bytes stayed unchanged (`9198420` for 20 runs,
`36793680` for 80 runs). Three focused 20-run samples were `11.341B`,
`11.337B`, and `11.338B`; the focused 80-run sample was `44,619,350,625`
instructions, `26,000,036,427` cycles, `7,371,035,905` branches, and
`276,500,385` branch misses. The fresh profile artifact is
`benchmarks/tmp/perf-z000033-l16-rust-after-match-window-low-threading.data`.
The broad one-run normal artifact
`benchmarks/tmp/normal-levels-8-16-19-api-runs1-after-match-window-low-threading.csv`
has byte fields identical to
`benchmarks/tmp/normal-levels-8-16-19-api-runs1-after-huffman-base-largest-bits.csv`;
aggregate byte gaps remain level 8 `-560`, level 16 `-967`, and level 19
`-155` bytes against C. Target smoke `corpus_z000022` level 19 target 8192
still matches the C API at `78067` bytes. Validation passed `cargo fmt
--check`, focused opt-match/parser tests, full `cargo test -p ruzstd --quiet`,
`cargo clippy -p ruzstd --all-targets -- -D warnings`, a release
`profile_c_port` rebuild, broad byte comparison, `git diff --check`, and the
target smoke.

Rejected follow-up after the match-bound cleanup: threading the already
computed `sufficient_len` through `collect_matches_mls()` and
`BtMatchRequest` into `insert_bt_and_get_all_matches_no_dict()` preserved
focused bytes but regressed the current focused instruction band. Three
20-run samples were `11.347B`, `11.351B`, and `11.341B`; the 80-run sample
was `44,637,165,017` instructions versus the current kept `44,619,350,625`.
It was reverted; the larger request payload costs more than recomputing the
capped target length in the tree collector.

Rejected follow-up after the same checkpoint: changing the
`build_table_from_probabilities()` negative-probability pass from the
iterator/filter shape to an explicit loop preserved focused bytes but did not
beat the current focused instruction band. Three 20-run samples were
`11.340B`, `11.339B`, and `11.341B`. It was reverted; do not retry this FSE
table-builder loop shape without a new profile signal.

Rejected follow-up after the same checkpoint: changing
`LiteralPriceCache::lookup()` from `then_some(self.prices[idx])` to an explicit
branch preserved focused bytes and had neutral 20-run samples
(`11.342B`, `11.337B`, `11.339B`), but the 80-run sample regressed to
`44,631,142,272` instructions versus the kept `44,619,350,625`. It was
reverted; keep the current cache lookup shape unless a new profile points back
there.

Rejected follow-up after the select-path direct-last checkpoint: adding indexed
`LiteralPriceCache` accessors and computing the literal table index once in
`raw_literal_cost()` preserved focused bytes but did not beat the current
focused CPU band. Three 20-run samples were `11.334B`, `11.337B`, and
`11.336B`; the focused 80-run sample regressed to `44,611,877,335`
instructions versus the kept `44,599,131,257`. It was reverted; keep the
current literal cache `lookup(literal)` / `insert(literal, price)` shape unless
a new profile gives a stronger reason.

Rejected follow-up after the same checkpoint: replacing the FSE table-builder
single-state checks from `state.states.len() == 1` to the already-normalized
`prob == 1` preserved focused bytes but worsened sequential 20-run instruction
samples to `11.352B` and `11.346B` versus the kept `11.337B` to `11.341B`
band. It was reverted; keep the current `Vec::len()`-based checks unless a new
profile gives a stronger reason.

Rejected follow-up after the select-path direct-last checkpoint: rewriting
`pop_smallest_huffman_node()` from the current `Option`/`match` shape to direct
availability branches preserved focused bytes and had neutral 20-run samples
(`11.334B`, `11.339B`, `11.334B`, `11.332B`), but two focused 80-run samples
regressed to `44,607,199,252` and `44,610,423,941` instructions versus the
kept `44,599,131,257`. It was reverted; keep the current Huffman pop helper
unless a new profile points clearly at it.

Rejected follow-up after the same checkpoint: changing the hot
`update_match_prices()` endpoint write from the current struct update with
`..state.opt[pos]` to direct `price`/`off`/`mlen`/`litlen` field mutation
preserved focused bytes but worsened 20-run instruction samples to `11.341B`,
`11.336B`, and `11.338B` versus the kept `11.331B` to `11.339B` band. It was
reverted; keep the current struct update shape unless a new profile points
back to this exact write.

Rejected follow-up after the select-path direct-last checkpoint: changing the
`update_match_prices()` downward match-length scan from the current
`while match_len >= start_len` plus bottom equality break to a `loop` guarded
only by the bottom equality check preserved focused bytes but clearly regressed
the 20-run instruction band. Samples were `11.394B`, `11.387B`, and `11.390B`
versus the kept `11.331B` to `11.339B` band. It was reverted; keep the current
scan loop unless a new profile gives a stronger reason.

Rejected follow-up after the select-path direct-last checkpoint: projecting
`state.price_state`, `state.matches`, and `state.opt` into local borrows inside
`update_match_prices()` preserved focused bytes but did not improve the focused
CPU band. Three 20-run samples were `11.340B`, `11.339B`, and `11.333B`; the
focused 80-run sample regressed to `44,622,960,223` instructions versus the
kept `44,599,131,257`. It was reverted; keep the current direct `state.*`
access shape unless a new profile gives a stronger reason.

Rejected follow-up after the same checkpoint: changing
`pre_split::Fingerprint::merge()` from `zip(other.events)` to
`zip(other.events.iter().copied())` preserved focused bytes but did not improve
the focused CPU band. Three 20-run samples were `11.335B`, `11.336B`, and
`11.338B`; the 80-run sample regressed to `44,608,752,112` instructions versus
the kept `44,599,131,257`. It was reverted; keep the current merge shape unless
a new profile points back there.

Latest kept parser follow-up: the no-LDM optimal parser path now calls
`collect_matches_no_ldm_mls()` and only the LDM path calls
`collect_matches_with_ldm_mls()`, avoiding the hot-path
`Option<&mut LdmOptCursor>` reborrow/plumbing in normal compression while
preserving the existing LDM behavior. Focused `corpus_z000033` level-16 bytes
stayed unchanged (`9198420` for 20 runs, `36793680` for 80 runs). Three
focused 20-run instruction samples were `11.333B`, `11.335B`, and `11.338B`;
the focused 80-run sample was `44,604,623,552` instructions,
`25,155,228,543` cycles, `7,367,765,426` branches, and `274,272,622` branch
misses. Current profile artifact:
`benchmarks/tmp/perf-z000033-l16-rust-after-no-ldm-match-specialization.data`.
The broad one-run normal artifact
`benchmarks/tmp/normal-levels-8-16-19-api-runs1-after-no-ldm-match-specialization.csv`
has byte fields identical to
`benchmarks/tmp/normal-levels-8-16-19-api-runs1-after-match-window-low-threading.csv`;
aggregate byte gaps remain level 8 `-560`, level 16 `-967`, and level 19
`-155` bytes against C. Target smoke `corpus_z000022` level 19 target 8192
still matches the C API at `78067` bytes. Validation passed `cargo fmt
--check`, focused parser/LDM/FSE tests, full `cargo test -p ruzstd --quiet`,
`cargo clippy -p ruzstd --all-targets -- -D warnings`, release profiler and
benchmark rebuilds, broad byte comparison, `git diff --check`, and the target
smoke.

Latest kept path reconstruction follow-up: `select_path()` now updates the
last path entry by direct index after the initial stretch has been pushed,
instead of checking `path.last_mut()` on each reverse traversal step. This is a
small safe step toward C's in-place `_shortestPath` reconstruction while
keeping the existing validated output order and final `path.reverse()` shape.
Focused `corpus_z000033` level-16 bytes stayed unchanged (`9198420` for
20 runs, `36793680` for 80 runs). Three focused 20-run instruction samples
were `11.339B`, `11.331B`, and `11.335B`; the focused 80-run sample was
`44,599,131,257` instructions, `25,202,115,212` cycles, `7,367,593,524`
branches, and `279,163,848` branch misses. Current profile artifact:
`benchmarks/tmp/perf-z000033-l16-rust-after-select-path-direct-last.data`.
The broad one-run normal artifact
`benchmarks/tmp/normal-levels-8-16-19-api-runs1-after-select-path-direct-last.csv`
has byte fields identical to
`benchmarks/tmp/normal-levels-8-16-19-api-runs1-after-no-ldm-match-specialization.csv`;
aggregate byte gaps remain level 8 `-560`, level 16 `-967`, and level 19
`-155` bytes against C. Target smoke `corpus_z000022` level 19 target 8192
still matches the C API at `78067` bytes. Validation passed `cargo fmt
--check`, focused parser tests, full `cargo test -p ruzstd --quiet`,
`cargo clippy -p ruzstd --all-targets -- -D warnings`, release profiler and
benchmark rebuilds, broad byte comparison, `git diff --check`, and the target
smoke.

## Persistent Objective

The long-running goal on branch `faithful-c-compressor-port` is to continue the
Rust port until the compressor is a faithful port of the C zstd compressor and
the focused and broad benchmarks are comparable with the C implementation for
both compressed size and CPU. Do not treat the PR as ready solely because Rust
output is slightly smaller on some fixtures; the remaining CPU gap still needs
evidence-driven C parity work.

Future sessions must not mark this objective complete until the C-port
compression-size and CPU parity work is genuinely done. When restarting this
project, resume from this file and the c-port README instead of rediscovering
the project state from scratch.

Keep code ownership and benchmark interpretation with the main agent. Use
sidecar workers for command-heavy validation, benchmark runs, CI log collection,
and mechanical commit execution only after the main agent has decided exact
scope and messages.

## Next Resume Action

Start from the notes in this section before continuing the chronological
history below. The current branch is `faithful-c-compressor-port`; the
authoritative C compressor source is
`/home/bsutton/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.16+zstd.1.5.7/zstd`.

Current July 18 resume target: continue on low-level CPU parity, starting with
level 1 on `corpus_z000033`. The retained Fast bounds and safe Huffman
overlapping-store changes reduce the original instruction gap from `+80.83%`
to `+55.23%`, but the remaining branch gap is still `+156.60%`. Record a fresh
instruction and branch profile after the overlap-store checkpoint and compare
absolute Rust/C costs. The pre-overlap profile already showed the Rust Fast
matcher below C in absolute sampled instructions; shared literal/sequence
preparation and entropy emission were the larger remaining excess, so do not
assume the next candidate belongs in `fast.rs` without new profile evidence.
Do not retry accepting physical index zero as a valid hash candidate; C's zero
sentinel is invalid because its window starts at virtual index 2. Levels 3, 5,
and 8 also remain about 60-68% above C in instructions and should follow once
the reusable level-1 costs are reduced. The post-split estimator, explicit C
`FSE_repeat_check`/`FSE_repeat_valid` state, and favorable high-level CPU
checkpoints remain retained.

Current resume action after the August 28 gates: the dedicated cross-crate
four-stream Huffman compressor and its single-reservation cursor are retained,
establishing the working pattern for large generated-code ports. Level 1 is now
`695.07M` instructions per 20 runs, about `39%` above the paired C baseline,
while Rust's Fast matcher already uses fewer absolute instructions than C's.
Re-profile the retained binary and
continue below the matcher in literal statistics, sequence preparation, and
table selection. The attempted isolated all-table sequence emitter improved
Fast and optimal but regressed Greedy/Lazy by `2.60%` to `3.02%`; retain its
local generated code. Prefer another coherent C boundary large enough to justify its call,
and place substantial table-log or strategy specializations in a dedicated
no-std crate/codegen unit so they cannot perturb Greedy/Lazy layout. Do not
split ordinary small helpers across a call boundary.
The later isolated fused no-dictionary row-search port plus once-per-block
generated-function selection is now the authoritative Greedy/Lazy baseline.
The fused boundary first improved levels 5/8 by `1.563%`/`3.062%`; moving C's
function choice out of every search then improved them by another
`13.696%`/`12.461%`. Refresh level-8 Rust/C attribution from that binary; the
prior profile is no longer authoritative for the balance inside row search.
The subsequent direct no-dictionary repeat continuation improves levels 5/8
by a further `7.213%`/`8.354%` and is the newer authoritative baseline. Refresh
attribution again before choosing between the remaining parser/sequence-store
pipeline and shared entropy excess.
The subsequent aggregate-versus-four-stream literal-statistics split reduced
stack and symbol size but regressed level 3 by `0.644%`; do not retry function
separation without changing the underlying histogram work.
The following full C-width `CodeCounts` port also failed: despite halving each
sequence histogram, it regressed levels 5/8 by `2.835%`/`3.221%`. Preserve
`usize` counts until normalization and table construction can remain 32-bit
end-to-end without a conversion boundary.
The subsequent safe post-match `ZSTD_storeSeq()`-style literal wild copy and
fused numeric `offBase`/history transition remain retained. The following
exact sequence-search, policy isolation, target-only preferred-repeat literal
isolation, primitive-ABI `SeqStore` codegen boundary, and persistent prepared
workspace are the current baseline.
Preserve the padded fixed 16/32/48/64-byte stores, keep literal copying out of
the generated matcher loops, and keep pre-encoded C offsets numeric through
the entropy-history pass. Level-1/3/5/8/16 instruction medians are now
`658,967,804`, `1,092,749,602`,
`3,463,615,192`, `4,363,196,208`, and `7,624,930,884`. Use the primitive ABI,
not external Rust record types, for larger isolated Fast/DFast sequence-store
work. Re-profile this retained binary before choosing the next large boundary;
the remaining Rust-only history, literal-statistics, table-selection, and
entropy costs are still preferable targets to the already-cheaper Fast
matcher.

Latest August 28 resume refinement: the dynamic BMI2 entropy port is the
authoritative low-level baseline. It measures `567,188,864`, `987,941,835`,
`2,079,254,880`, `2,660,557,934`, and `7,551,897,701` instructions at levels
1/3/5/8/16. The remaining paired-C gaps are approximately `+13.29%` at level
1 and `+17.16%` at level 3; levels 5, 8, and 16 remain ahead of C. Preserve
the cached CPUID dispatch and separate BMI2/portable generated functions for
both Fast/DFast sequence emission and four-stream Huffman emission. Also
preserve the compact lane workspace only for DFast: its shared Fast/DFast form
regressed level 1 by `0.6248%`, while the retained gate improves level 3 by
`0.6245%`. Current counter artifacts end in
`c-dynamic-bmi2-entropy-{1,2,3}.stat`, and current broad artifacts end in
`c-dynamic-bmi2-entropy.csv`. Fresh paired 80-run attribution now lives in
`profile-c-port-z000033-l{1,3}-post-fse-revert.perf.data` and
`profile-c-api-z000033-l{1,3}-current-paired.perf.data`. At level 1 the Rust
Fast matcher is about `1.108B` sampled instructions versus C's `1.142B`; at
level 3 DFast is about `2.008B` versus `2.016B`. Do not return to either
matcher. The main absolute level-3 excess is now Huffman construction plus the
sequence statistics/table-selection transaction. Two complete isolated forms
of the latter proved a real `0.52%` to `0.73%` level-3 win but displaced Fast
enough to regress level 1 by about `0.54%`; both were reverted. Continue with
a shared low-level boundary that can repay layout movement at levels 1 and 3,
preferably Huffman construction/statistics inside the already-retained Huffman
generated unit, or first establish a portable way to keep Fast placement
stable. Do not retry the DFast-only transaction unchanged.

Latest August 28 resume refinement after matcher-offset handoff: the complete
DFast compact count-to-normalization pipeline now uses C-width `u32` values
end to end and is retained. Lunx measured a small `-0.0081%` level-3
instruction improvement with levels 1/5/8/16 neutral; all 1,095 rows remain
exact. This is a faithful representation cleanup, not material closure of the
remaining low-level gap. Fresh attribution still places the useful next large
tranche in shared Huffman construction/emission or a producer-to-consumer
sequence representation that actually removes conversion work. Do not repeat
the rejected wrapper-only orchestration, in-record marker cache, or global
high-bit LL/ML substitutions.

Latest August 28 native-SeqStore refinement: direct consumption of the 12-byte
Fast/DFast stored records is now retained. A same-binary prepared-path switch
removes the layout ambiguity that caused the first attempt's apparent
level-16 regression. Lunx measures `-2.0507%` at level 1 and `-1.9688%` at
level 3, with levels 5/8/16 neutral and all bytes exact. Preserve the native
path and `RUZSTD_TUNE_C_NATIVE_SEQUENCE_STORE=0` diagnostic A/B switch. Next
refresh paired C level-1/3 attribution and choose another large entropy or
table-statistics boundary; do not optimize Fast/DFast matching, which is
already cheaper than C.

Latest August 28 Greedy/Lazy native-SeqStore refinement: the same direct
12-byte stored-record lifecycle now covers normal Greedy, Lazy, Lazy2, and
BtLazy2 no-dictionary, external-dictionary, and attached-dictionary blocks.
Target-size mode retains prepared records because its split/estimate path owns
that representation. The separate same-binary control is
`RUZSTD_TUNE_C_GREEDY_NATIVE_SEQUENCE_STORE=0`; it does not disable the
retained Fast/DFast native path. Lunx's candidate/control medians at levels
1/3/5/8/16 are `557,501,764`/`557,501,890`,
`970,955,186`/`970,978,110`, `2,019,393,304`/`2,041,364,903`,
`2,760,507,264`/`2,781,490,603`, and
`7,819,066,651`/`7,819,629,814`. Levels 5 and 8 improve by `1.0763%` and
`0.7544%`; levels 1, 3, and 16 are neutral and every expected byte total is
exact. KEEP. Full workspace tests, strict Clippy, formatting, release builds,
`git diff --check`, and all 1,095 candidate/control broad rows pass. Counter
artifacts end in `greedy-native-stored-ab-{candidate,control}`; broad artifacts
end in `greedy-native-store-{candidate,control}.csv`. The remaining CPU parity
work is still levels 1 and 3 below their already-cheaper matchers, not Greedy/
Lazy or optimal parsing.

Latest August 28 optimal native-SeqStore refinement: normal BtOpt, BtUltra,
and dictionary-routed BtUltra2 now carry the same 12-byte stored records
through entropy emission for no-dictionary, external-dictionary, and attached-
dictionary blocks. Target-size and post-split modes retain prepared records.
The final full-block leading-repcode preference has a stored-record equivalent
proved against materialization. The independent control is
`RUZSTD_TUNE_C_OPT_NATIVE_SEQUENCE_STORE=0`. Lunx candidate/control medians at
levels 1/3/5/8/16/19/22 are `557,503,369`/`557,505,925`,
`970,977,956`/`970,955,188`, `2,019,415,422`/`2,019,394,120`,
`2,760,486,533`/`2,760,485,865`, `7,819,629,390`/`7,819,069,594`,
`20,695,715,385`/`20,695,715,971`, and
`33,326,521,011`/`33,328,020,673`. Every delta is below `0.008%`; this is a
CPU-neutral completeness KEEP, not a claimed speedup. Bytes are exact. Full
tests, strict gates, and candidate/control comparison passed across 511 normal,
292 target-2048, and an expanded 511-row prepared-CDict matrix through level
22. Counter artifacts end in `opt-native-stored-ab-{candidate,control}`;
broad artifacts end in `opt-native-store-{candidate,control}.csv`. Normal mode
now keeps native C SeqStore records through entropy for every C strategy. The
next performance tranche should again target the measured shared level-1/3
Huffman construction/statistics or sequence table-selection excess.

Latest August 28 combined generated Huffman construction/description
refinement: the existing `ruzstd-huff0-codegen` unit now owns C's complete
bucket-sort, tree-merge, height-limit, canonical-code, and bits-to-weight
transaction. It consumes the weights immediately through one call to the
retained exact FSE/raw description serializer and returns the finished table;
the active path does not allocate or return the intermediate weights vector
that made the earlier builder-only boundary unprofitable. The same-binary
control is `RUZSTD_TUNE_C_GENERATED_HUFFMAN_TABLE=0`. Lunx candidate/control
medians at levels 1/3/5/8/16 are `557,434,210`/`557,508,345`,
`970,468,134`/`971,003,794`, `2,019,378,725`/`2,019,456,436`,
`2,760,800,008`/`2,760,547,400`, and
`7,821,713,922`/`7,819,736,195`: `-0.0133%`, `-0.0552%`, `-0.0038%`,
`+0.0092%`, and `+0.0253%`. KEEP as a modest level-1/3 instruction gain and a
substantial ownership port; do not describe it as a major speedup. Bytes and
all 1,095 broad rows are exact. Full tests (716 passed, 5 ignored), codegen
suites, strict Clippy, formatting, release builds, and `git diff --check`
pass. Counter artifacts end in
`generated-huffman-described-ab-{candidate,control}`; broad artifacts end in
`generated-huffman-described-{candidate,control}.csv`. A future follow-up
should move the specialized Huffman-weight FSE serializer itself into the same
unit only as part of a complete description transaction, not as a small cross-
crate helper.

Latest August 28 resume refinement after the native-SeqStore checkpoint: the
complete generated `HUF_buildCTable_wksp()` analogue was implemented and then
rejected after the full correctness and 1,095-row byte gates. Same-binary
candidate/control instruction deltas were `+0.0440%`, `+0.1837%`, `+0.0915%`,
`+0.0778%`, and `+0.0300%` at levels 1/3/5/8/16. Preserve the existing local
compact-node construction. The next large Huffman boundary must include a
consumer such as table-description construction or emission instead of
returning a newly owned code vector across the crate boundary. Alternatively,
refresh paired C level-1/3 profiles and select a different shared statistics
transaction with a demonstrated absolute excess.

August 28 refinement: the fixed 256-entry Huffman lookup is retained, while
paired Huffman containers, a dedicated aligned sequence writer, C high-bit
LL/ML formulas, and a complete stored sequence-code boundary all caused
material level-5/8 regressions despite exact bytes. The stored-code boundary
proved the conversion work is worth about 1.5-1.8% at levels 1/3, but its
larger shared record regressed levels 5/8 by 2.1-2.7%. Before the next shared
sequence or Huffman rewrite, first establish compilation/link-layout and data-
layout isolation (for example a deliberately separate Fast/DFast codegen unit
with stable section placement) or use a change that demonstrably preserves
the retained Greedy/Lazy machine code and record layout. Do not infer that
smaller source or runtime strategy gating is sufficient isolation. An explicit
non-generic, non-inlined fixed-Huffman boundary also failed: it regressed levels
1/3 by `3.21%`/`1.26%` and levels 5/8 by `9.52%`/`8.91%`, even though fresh
profiles still identify Huffman emission as the main level-1 gap after the
already-cheaper Rust Fast matcher. The subsequent dedicated
`ruzstd-huff0-codegen` crate supplied that missing isolation and retained
cross-level improvements of `4.60%`, `2.58%`, `0.63%`, `0.51%`, and `0.27%`
at levels 1/3/5/8/16. Preserve it as the model for future large generated-code
ports. A direct
C-style Fast `_offset`/`_match` convergence also shrank the active function by
12.9% but regressed levels 1, 5, and 8 by 1.6-3.1%; preserve the duplicated
Rust hit tails unless a different shape improves hardware counters. A safe
by-value 256-entry C-short FSE normalization workspace likewise improved level
1 by 0.26% but regressed levels 5/8 by 8.6-9.1%; retain the compact heap vectors
instead. A subsequent caller-owned, sequence-only, initialized-once workspace
still regressed levels 5/8 by 2.77-3.16%, so changing ownership alone is not
sufficient; require a materially different algorithm or separately compiled
codegen boundary before revisiting fixed short probabilities. The
outer C four-stream Huffman destination boundary was also insufficient
isolation: it improved levels 1/3/16 by `0.94%`/`0.62%`/`0.24%` but regressed
levels 5/8 by `2.52%`/`2.94%`. Preserve the four independent stream calls.
Fusing LL/ML/OF code statistics across normal selection, estimation, and
superblock construction likewise improved level 3 by `0.74%` and level 16 by
`0.16%`, but regressed levels 5/8 by `2.74%`/`3.11%`; preserve the independent
normal selection scans and estimator-local statistics pass.
Porting C's Huffman remainder join without also making `kUnroll` a compile-time
template regressed every level, including `5.51%` at level 1. Preserve the
current `rchunks()` loop; do not retry the runtime-width remainder/exact-batch
shape. The earlier const-specialized paired-container experiment shows that
template expansion has its own cross-level layout cost, so another Huffman
loop rewrite needs a materially different isolation mechanism.
Likewise, a separately allocated three-byte LL/ML/OF code view preserved the
12-byte sequence record but regressed every level, including `3.20%`/`3.44%`
at levels 5/8. Keep code conversion fused into its consumers unless the codes
can live in already-owned persistent storage without another allocation or
parallel traversal; do not enlarge the shared sequence record.
Shrinking the existing FSE symbol transform from 12 to 8 bytes in place also
failed the gate: levels 1/3 improved by `0.09%`/`0.77%`, but levels 5/8
regressed by `2.73%`/`3.11%`. Retain the `i32` probability and delta fields.
Because C does not retain probability in its compression transform at all, a
future compact-table attempt should separate NCount serialization from the
final transform rather than merely narrow fields in the shared builder.
The subsequent full separation pre-serialized NCount and used C's exact
two-word transform, but persisting the description `Vec` in `FSETable`
regressed every measured level (`0.04%` to `0.32%`). If revisited, make the
description a transient build result consumed before repeat-history storage;
do not add it to the persistent table or shared mode layout.
That transient build-result design was then implemented end to end and still
regressed levels 5/8 by `2.76%`/`3.09%`, despite dropping counts before repeat
history and slightly improving level 1. The live build result/shared
`FseTableMode` shape itself is not isolated enough. Preserve the 12-byte
single-vector table until a separate compilation boundary can prevent this
cross-strategy generated-code movement.
C's dedicated no-low-probability two-symbol spread was also ported with one
safe workspace and exact state tables, but regressed every measured level
(`0.06%` to `0.32%`). Preserve the general spread walk. The C branch's
caller-owned scratch and overlapping bulk stores cannot be judged separately
from the rejected workspace/layout experiments already recorded here.
The
retained medians remain level 1 `730,974,166`,
level 3 `1,307,142,527`, level 5 `3,543,052,789`, level 8 `4,438,895,995`,
and level 16 `7,708,588,431` instructions.

Live checkpoint from July 17, 2026: broad target-mode byte parity is closed
across the 73-fixture real-world corpus for levels 13, 16, and 19 at
`targetCBlockSize` values 2048, 4096, and 8192. Fresh artifacts:
`benchmarks/tmp/target-cblock-2048-levels-13-16-19-after-c-huf-sort.csv`,
`benchmarks/tmp/target-cblock-4096-levels-13-16-19-after-c-huf-sort.csv`,
and
`benchmarks/tmp/target-cblock-8192-levels-13-16-19-after-c-huf-sort.csv`.
All nine target/level combinations report `rows=73`, `differing=0`,
`positive=0`, and total byte gap `+0` against the C API. The one-run CPU values
in these target-mode sweeps are noisy; do not treat them as the CPU parity
conclusion.

July 18 prepared-CDict resume action: the large prepared-dictionary regression
is closed by dictionary-parameter retention and valid-repeat Huffman state;
do not reopen the disproven `CreateCDict` parameter hypothesis. C's actual
focused parameters were `window=16 chain=17 hash=17 search=6 min=3 target=128
strategy=8`, exactly matching Rust. Prepared levels 8-19 are now at aggregate
parity and level 5 is within `+0.222%` across 100 systemd files. Continue with
normal-mode CPU branch/control-flow work on `corpus_z000033` level 16 unless a
broader prepared-dictionary corpus exposes a new material gap.

The latest target-mode fixes are in
`ruzstd/src/encoding/levels/c_port/target_block.rs`. Target mode now tries
repeat/treeless Huffman literals with the selected sequence modes, which closed
the prior positive `corpus_z000044` level-19 gap. The ordering matters:
fresh compressed-literal/all-compressed sequence candidates must run before the
pure repeat-literal/all-repeat fallback. Trying repeat/all-repeat earlier made
Rust smaller than C on `corpus_z000033` at level 19 for target sizes 4096 and
8192 by using repeat FSE tables where C writes fresh FSE tables.

The final `corpus_z000022` level-19 target 8192 byte gap was closed by making
the Rust Huffman tree builder use C's `HUF_sort()` bucket ordering. C preserves
symbol order for small exact-count buckets, but for larger log buckets it uses
an unstable quicksort by count only. Rust had been sorting all equal counts by
symbol, which produced the same weight histogram but swapped four symbol
weights in the superblock Huffman table. The regression test is
`huffman_sort_matches_c_log_bucket_tie_order`. Focused inspector artifact:
`benchmarks/tmp/inspect-target8192-l19-corpus-z000022-after-c-huf-sort.txt`;
it reports `content_delta=0`, `source_delta=0`, `type_diffs=0`, and
`first_diff=none`.

Latest normal-mode CPU checkpoint from July 17, 2026: after the C `HUF_sort()`
parity fix, `c_sorted_huffman_nodes()` was tightened to allocate and fill only
the nonzero node prefix while still computing C-compatible bucket positions.
This preserves the C ordering but avoids initializing/truncating zero-count
nodes. Focused `corpus_z000033` level-16 bytes stayed unchanged
(`9198420` for 20 runs, `36793680` for 80 runs), and the target-mode parity
matrix stayed exact. `perf stat` on the focused 80-run normal-mode smoke moved
from `49,548,126,529` instructions before the cleanup to `48,728,786,295`
after it. The profile artifact is
`benchmarks/tmp/perf-z000033-l16-rust-after-nonzero-huf-sort.data`.

Latest CPU follow-up: `opt_match/tree.rs` now keeps the binary-tree link walks
in the C table representation (`0` means empty, otherwise `index + 1`) and only
subtracts one after confirming a non-empty slot. This avoids constructing
`Option<usize>` in the two hot tree loops while preserving the existing
source-index-zero parity fix. Focused `corpus_z000033` level-16 bytes stayed at
`9198420` for 20 runs and `36793680` for 80 runs. Same-session 20-run
instruction samples moved from `12.376B` and `12.378B` before the change to
`12.325B`, `12.325B`, and `12.325B` after it. Target parity smoke
`corpus_z000022` level 19 target 8192 still matches the C API at `78067` bytes.
Validation passed `cargo fmt --check`, `cargo test -p ruzstd opt_match
--quiet`, `cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd
--all-targets -- -D warnings`, and `git diff --check`. A same-session A/B of
adding `#[inline(always)]` to `hash_chain_match::highbit32()` was neutral
(`12.377B`, `12.381B`, `12.376B` with the attribute versus `12.376B`,
`12.378B` without), so the attribute was not kept.

Fresh follow-up profile and broad benchmark after the tree-link change:
`benchmarks/tmp/perf-z000033-l16-rust-after-tree-sentinel.data` is the current
focused Rust profile. It still shows `forward_pass` and
`compress_block_opt_with_state_and_ldm` as the dominant CPU costs. The broad
normal API benchmark artifacts are
`benchmarks/tmp/normal-levels-8-16-19-api-runs3-after-tree-sentinel.csv` and
`.md`. Across 73 real-world fixtures, size totals versus C API are still
effectively at parity: level 8 Rust `-560` bytes (`-0.036%`), level 16 Rust
`-967` bytes (`-0.070%`), and level 19 Rust `-155` bytes (`-0.012%`). Worst
positive byte gaps remain tiny: `+6` at level 8, `+4` at level 16, and `+5` at
level 19. The three-run broad CPU seconds are rounded and noisy, so use focused
perf counters for CPU conclusions. Current focused 80-run `corpus_z000033`
level-16 counters: Rust `48,529,866,636` instructions,
`27,729,170,739` cycles, `8,365,254,048` branches, and `279,943,935` branch
misses; C API `34,677,320,506` instructions, `20,406,964,727` cycles,
`4,332,735,580` branches, and `175,791,305` branch misses. This is progress
from the previous Rust `48,728,786,295` instruction checkpoint, but the focused
CPU gap is still real and the next work should stay on parser/match-finder
instruction count rather than target-mode byte parity.

Latest kept CPU change after the tree-link checkpoint: sequence C-cost table
selection now reuses the already-computed max symbol when calculating entropy,
predefined, and repeat costs. This avoids repeatedly scanning/filtering all 256
code slots inside `choose_c_cost_table()` without reintroducing the rejected
`CodeCounts::max_symbol` field in the per-code counting path. Focused
`corpus_z000033` level-16 bytes stayed unchanged (`9198420` for 20 runs,
`36793680` for 80 runs). Three focused 20-run instruction samples were
`12.196B`, `12.201B`, and `12.200B`; two focused 80-run samples were
`48,030,727,202` and `48,086,528,143` instructions. Target parity smoke
`corpus_z000022` level 19 target 8192 still matches the C API at `78067` bytes.
The broad normal API artifacts are
`benchmarks/tmp/normal-levels-8-16-19-api-runs3-after-seq-cost-max-symbol.csv`
and `.md`; their C/Rust byte fields match the previous tree-sentinel artifact
exactly, with the same aggregate gaps (`-560`, `-967`, `-155` bytes for levels
8, 16, and 19). Validation passed `cargo fmt --check`, focused sequence and
compressed tests, `cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd
--all-targets -- -D warnings`, and `git diff --check`.

Follow-up FSE/Huffman cleanup after the sequence-cost max-symbol change:
`build_huffman_weight_table_from_data()` now uses `data.len()` for the total
weight count instead of summing the already-counted nonzero prefix. This
preserves behavior while removing a small redundant scan in the Huffman weight
FSE path. Focused `corpus_z000033` level-16 bytes stayed unchanged (`9198420`
for 20 runs, `36793680` for 80 runs). Three focused 20-run instruction samples
were `12.195B`, `12.193B`, and `12.191B`; the focused 80-run sample was
`48,036,223,779` instructions. Target parity smoke `corpus_z000022` level 19
target 8192 still matches the C API at `78067` bytes. Validation passed
`cargo fmt --check`, `cargo test -p ruzstd fse --quiet`, `cargo test -p ruzstd
huff --quiet`, `cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd
--all-targets -- -D warnings`, and `git diff --check`.

Follow-up Huffman stream-count reuse: `HuffmanTable` now has a
`build_smallest_from_counts_with_stream_counts()` entry point so the compressed
literal encoder and estimator can reuse the four-stream literal counts they
already computed, instead of recounting the same literal streams inside
`build_smallest_from_counts()`. The old data-based wrapper is now test-only.
Focused `corpus_z000033` level-16 bytes stayed unchanged (`9198420` for
20 runs, `36793680` for 80 runs). Three focused 20-run instruction samples
were `12.139B`, `12.141B`, and `12.141B`; the focused 80-run sample was
`47,817,650,572` instructions. Current profile artifact:
`benchmarks/tmp/perf-z000033-l16-rust-after-stream-count-reuse.data`. The
one-run broad API artifact
`benchmarks/tmp/normal-levels-8-16-19-api-runs1-after-stream-count-reuse.csv`
has byte fields identical to
`benchmarks/tmp/normal-levels-8-16-19-api-runs3-after-seq-cost-max-symbol.csv`;
aggregate byte gaps remain level 8 `-560`, level 16 `-967`, and level 19
`-155` bytes against C. Target parity smoke `corpus_z000022` level 19 target
8192 still matches the C API at `78067` bytes. Validation passed `cargo
fmt --check`, focused Huffman and compressed tests, `cargo test -p ruzstd
--quiet`, `cargo clippy -p ruzstd --all-targets -- -D warnings`, and
`git diff --check`.

Rejected follow-up parser cache A/B: forcing `#[inline(always)]` on
`LiteralPriceCache::begin_pass()`, `lookup()`, and `insert()` preserved focused
bytes but did not improve the current instruction band (`12.193B`, `12.196B`,
`12.192B` for 20-run samples versus the preceding `12.195B`, `12.193B`,
`12.191B` band). It was reverted; do not retry without a different profile
signal.

Follow-up rank-limited Huffman redistribution cleanup: the sorted
`rank_limited_nonzero_weights()` buffer is now reduced by locating the first
entry in the highest eligible weight bucket with `partition_point()`, instead
of rescanning all weights plus a parallel unit table for every reduction step.
The tie-break is the same first-entry-in-bucket choice as the old full scan,
and the sorted order remains intact after each decrement. Focused
`corpus_z000033` level-16 bytes stayed unchanged (`9198420` for 20 runs,
`36793680` for 80 runs). Three focused 20-run instruction samples were
`11.695B`, `11.692B`, and `11.689B`; the focused 80-run sample was
`46,036,337,077` instructions. Current profile artifact:
`benchmarks/tmp/perf-z000033-l16-rust-after-rank-limited-bucket-reduce.data`;
`rank_limited_nonzero_weights()` dropped to about `0.81%` self time in that
profile. The one-run broad API artifact
`benchmarks/tmp/normal-levels-8-16-19-api-runs1-after-rank-limited-bucket-reduce.csv`
has byte fields identical to
`benchmarks/tmp/normal-levels-8-16-19-api-runs1-after-stream-count-reuse.csv`;
aggregate byte gaps remain level 8 `-560`, level 16 `-967`, and level 19
`-155` bytes against C. Target parity smoke `corpus_z000022` level 19 target
8192 still matches the C API at `78067` bytes. Validation passed `cargo
fmt --check`, focused Huffman and compressed tests, `cargo test -p ruzstd
--quiet`, `cargo clippy -p ruzstd --all-targets -- -D warnings`, and
`git diff --check`.

Follow-up FSE direct-lookup cast refresh: the nested lookup fill in
`build_table_from_probabilities()` now uses a debug-asserted `state_idx as u16`
instead of a release `u16::try_from()` branch, matching the documented kept
shape for this hot bounded index. Focused `corpus_z000033` level-16 bytes
stayed unchanged (`9198420` for 20 runs, `36793680` for 80 runs). Three
focused 20-run instruction samples were `11.685B`, `11.687B`, and `11.687B`;
the focused 80-run sample was `46,005,243,303` instructions. Current profile
artifact:
`benchmarks/tmp/perf-z000033-l16-rust-after-fse-lookup-cast-refresh.data`.
Target parity smoke `corpus_z000022` level 19 target 8192 still matches the C
API at `78067` bytes. Validation passed `cargo fmt --check`, focused FSE
tests, `cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd --all-targets
-- -D warnings`, and `git diff --check`.

Current normal-mode broad benchmark artifact:
`benchmarks/tmp/normal-levels-8-16-19-api-runs3-after-nonzero-huf-sort.csv`.
Summary against the C API across 73 fixtures: level 8 Rust is `-560` bytes
(`-0.036%`) with aggregate rounded CPU `0.04s` vs C `0.02s`; level 16 Rust is
`-967` bytes (`-0.070%`) with CPU `0.25s` vs C `0.23s`; level 19 Rust is
`-155` bytes (`-0.012%`) with CPU `0.51s` vs C `0.50s`. The worst positive
byte gaps are small (`+6` at level 8, `+4` at level 16, `+5` at level 19), so
the next meaningful work is CPU parity, especially the focused
`corpus_z000033` level-16 normal path where the three-run broad row still shows
Rust `0.08s` vs C `0.06s` and `-885` bytes.

Immediate resume validation:

```sh
cargo fmt --check
cargo test -p ruzstd --quiet
cargo clippy -p ruzstd --all-targets -- -D warnings
git diff --check
cargo build --release -p zstd-rs-tools --bin benchmark_c_port --bin inspect_c_port_blocks --bin profile_c_port --bin profile_c_api
target/release/profile_c_port benchmarks/archive/tmp/realworld-100/corpus_z000022 19 1 8192
target/release/profile_c_api benchmarks/archive/tmp/realworld-100/corpus_z000022 19 1 8192
```

Expected focused result: `corpus_z000022` level 19 target 8192 now matches the
C API at `78067` bytes. Do not spend more time on the earlier
`corpus_z000044` positive gap, `corpus_z000033` larger-target negative gaps, or
`corpus_z000022` target 8192 gap unless a new broad run reopens them. Next,
return to CPU parity work and broader normal-mode C comparisons.

Latest parser-divergence checkpoint from July 17, 2026: the `corpus_z000021`
level-16 optimal-parser gap is closed. Rust now matches the C API at
`26329` bytes with `targetCBlockSize=2048` and at `24877` bytes in normal
non-target mode. Inspector artifact
`benchmarks/tmp/inspect-target2048-z000021-after-opt-index-bias.txt` reports
`content_delta=0`, `source_delta=0`, `type_diffs=0`, `first_diff=none`, and
`first_source_diff=none`.

The fix was in `opt_match/tree.rs`: the optimal binary-tree match finder now
stores table indexes as `source_index + 1`, keeping `0` as the empty sentinel
while allowing real source index `0` to be matched. This mirrors C's effective
window indexing, where `matchLow == 1` still permits the first source byte
because the C window base is biased. The regression test is
`opt_match_collector_can_match_source_index_zero`.

Latest targetCBlockSize checkpoint from July 17, 2026: the 73-row level-16
real-world target smoke at `targetCBlockSize=2048` is byte-identical with the C
API. Artifact:
`benchmarks/tmp/target-cblock-2048-after-c-optimal-huffman.csv`. Summary:
`rows=73`, `differing=0`, C bytes `1412464`, Rust bytes `1412464`, total gap
`+0`. The remaining one-run CPU numbers in that smoke are noisy
(`0.24s` C vs `0.29s` Rust), so do not treat them as a performance conclusion.

The fixes after the `z000021` index-bias checkpoint were:

- Target Huffman literal table construction now uses C's strategy gate:
  strategies `BtUltra` and `BtUltra2` use a C-style `HUF_optimalTableLog()`
  shallow-to-deep probe instead of Rust's broader smallest-table search.
- The C-style optimal-depth builder keeps the first shallower table when
  estimated compressed size plus table description ties, and it does not try
  Rust's rank-limited alternate table. This closed `corpus_z000002`
  (`580 -> 577`), `corpus_z000064` (`1273 -> 1272`), and the negative
  `repo_io_std.rs` delta (`131 -> 133`) against the C API.
- Target mode now tries compressed Huffman literal-only blocks for zero-sequence
  non-RLE literal blocks before raw fallback. This closed `repo_Cargo.toml`
  (`73 -> 67`).
- Target mode now tries all-basic/predefined selected sequence modes before
  repeat/RLE sequence candidates. This closed the one-sequence `corpus_z000016`
  gap (`19 -> 18`).

Fresh block-identical inspector artifacts for the closed gaps:
`benchmarks/tmp/inspect-target2048-z000002-after-optimal-depth-literals.txt`,
`benchmarks/tmp/inspect-target2048-repo-Cargo-toml-after-literal-only-huffman.txt`,
`benchmarks/tmp/inspect-target2048-corpus-z000016-after-basic-single-sequence.txt`,
and `benchmarks/tmp/inspect-target2048-corpus-z000064-after-huffman-tie-break.txt`.

Historical target smoke after the earlier opt-index fix:
`benchmarks/tmp/target-cblock-2048-after-opt-index-bias.csv` and `.md`.
For 73 level-16 rows, C totals `1412464` bytes and Rust totals `1412542`
bytes, a remaining `+78` bytes (`+0.006%`). `corpus_z000021` is exact;
`corpus_z000002` improved to `580` Rust bytes vs `577` C bytes (`+3`). The
largest remaining positive fixture gap in that historical smoke was
`repo_Cargo.toml` at `+6` bytes. Those gaps are now closed by the latest
targetCBlockSize checkpoint above. CPU in the one-run smoke was noisy
(`0.29s` C vs `0.26s` Rust), so do not treat it as a performance conclusion.

Validation for this checkpoint passed `cargo fmt --check`, `cargo test -p
ruzstd opt_match --quiet`, `cargo test -p ruzstd opt_parser --quiet`, `cargo
clippy -p ruzstd --all-targets -- -D warnings`, `git diff --check`, release
builds for `profile_c_port`, `profile_c_api`, `inspect_c_port_blocks`, and
`benchmark_c_port`, focused `z000033` target smokes (`477631`, `477121`,
`476853`), and the normal `z000033` 20-run smoke (`9198420`). The follow-up
full-suite failure in
`greedy_tests::btlazy2_loaded_dictionary_keeps_full_dictionary_valid_like_c`
was fixed by making `greedy_dict::load_binary_tree_prefix()` use btlazy2's DUBT
prefix loader from `bt_match.rs` instead of the optimal-parser sorted-tree
loader. After that fix, `cargo test -p ruzstd --quiet`, `cargo clippy -p
ruzstd --all-targets -- -D warnings`, `cargo fmt --check`, and `git diff
--check` all passed.

Historical `z000021` target block 0 evidence: decoded sequence dumps from
`benchmarks/tmp/c-port-block-inspect/corpus_z000021.l16.target2048.rust.zst`
and `.c.zst` show the first local divergence as C choosing `ll=1 ml=10 of=8`
then `ll=0 ml=50 of=30`, while Rust chooses `ll=1 ml=11 of=8` then
`ll=0 ml=49 of=30`. Both paths rejoin at the same source position. A larger
grouping divergence starts around sequence 91, where C combines what Rust emits
as a short `ml=4` step plus the following match. The root cause was not
path-backtracking or a price comparison; Rust's binary-tree tables could not
represent source index `0` as a valid match because `0` was also the empty
sentinel.

Latest target-superblock checkpoint: Rust is block-identical with the C API on
`benchmarks/archive/tmp/realworld-100/corpus_z000033` at level 16 for
`targetCBlockSize` 2048, 4096, and 8192. The last parity fix made the
target-superblock literal planner match C's `ZSTD_estimateSubBlockSize_literal()`
rough estimate by using `HUF_estimateCompressedSize + hufDesSize + 3` and
excluding the emitted four-stream jump table from the planner estimate.
Inspector artifacts:
`benchmarks/tmp/inspect-target2048-after-superblock-estimate-parity.txt`,
`benchmarks/tmp/inspect-target4096-after-superblock-estimate-parity.txt`, and
`benchmarks/tmp/inspect-target8192-after-superblock-estimate-parity.txt`.
All three report `content_delta=0`, `source_delta=0`, `type_diffs=0`,
`first_diff=none`, and `first_source_diff=none`.

`benchmark_c_port` now supports `--target-c-block-size N` for API-backed C
comparisons. It is intentionally rejected with the CLI backend. A first
10-fixture smoke at level 16, target 2048 produced
`benchmarks/tmp/target-cblock-2048-smoke.csv` and
`benchmarks/tmp/target-cblock-2048-smoke.md`: 10 rows, C `602540` bytes, Rust
`602628` bytes, total gap `+88` bytes across 6 differing rows. The largest
smoke gap was `corpus_z000021` at `+78` bytes; inspector artifacts
`benchmarks/tmp/inspect-target2048-z000021-smoke-gap.txt`,
`benchmarks/tmp/inspect-target2048-z000002-smoke-gap.txt`, and
`benchmarks/tmp/inspect-target2048-z000023-smoke-gap.txt` show the next target
gaps are caused by different sequence/literal stores or earlier parser/block
choices, not just the already-fixed target sub-block sizing loop.

The target compressed block size / superblock checkpoint is now broadly
validated for the current fixture set. As of July 17, 2026, target mode
dispatch is threaded through the CCtx, frame state, optimal frame path, and
hash-chain frame path. The active target encoder tries a literal-only RLE
superblock and otherwise falls through target sub-block paths before falling
back to raw blocks.

Latest target-superblock change: target mode can now be profiled explicitly via
`profile_c_port INPUT LEVEL RUNS TARGET_C_BLOCK_SIZE`, `profile_c_api INPUT
LEVEL RUNS TARGET_C_BLOCK_SIZE`, and `inspect_c_port_blocks
--target-c-block-size N --c-backend api`. The Rust helper is deliberately
scoped to the C-port target-size validation path and returns `None` for invalid
target sizes; no-dictionary fast, double-fast, hash-chain, and optimal
strategies are now wired through target mode.

Recent target-superblock fixes moved basic-literal multi-sub-block attempts
after Huffman literal multi-sub-block and Huffman single-subblock candidates,
then made the target Huffman table builder mirror C's
`ZSTD_buildBlockEntropyStats_literals()` no-gain gate:
`newCSize + hSize >= srcSize` selects basic/raw literals instead of a new
Huffman table. The target table builder now also uses C's non-optimal-depth
`HUF_optimalTableLog()` shape, which delegates to
`FSE_optimalTableLog_internal(max=11, srcSize, maxSymbolValue, minus=1)` for
level-16 `BtOpt` blocks. Focused target smokes on
`benchmarks/archive/tmp/realworld-100/corpus_z000033` at level 16 now show:
target 2048 Rust `477631` bytes vs C API `477631`, target 4096 Rust `477121`
vs C API `477121`, and target 8192 Rust `476853` vs C API `476853`.

Validation after the latest target-superblock estimate parity fix passed for
`cargo fmt --check`, `cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd
--all-targets -- -D warnings`, `git diff --check`, the release build of
`profile_c_port`, `profile_c_api`, and `inspect_c_port_blocks`, the focused
target smokes above, and the normal focused 20-run smoke (`9198420` bytes).

Latest target-mode broad validation on July 17, 2026:
`benchmark_c_port --fixtures benchmarks/archive/tmp/realworld-100 --levels
8,16,19 --runs 1 --c-backend api --target-c-block-size N --no-sync` was run
for target sizes 2048, 4096, and 8192. Artifacts are
`benchmarks/tmp/target-cblock-2048-levels-8-16-19-resume.{csv,md}`,
`benchmarks/tmp/target-cblock-4096-levels-8-16-19-resume.{csv,md}`, and
`benchmarks/tmp/target-cblock-8192-levels-8-16-19-resume.{csv,md}`. At
target 2048, level 8/16/19 aggregate byte gaps were `+0`, `+0`, and `+0`
across 73 fixtures. At target 4096 they were `-1`, `+0`, and `+0`. At target
8192 they were `+2`, `+0`, and `+0`. Only level 8 had differing rows in these
sweeps: four fixtures per target size, with worst positive fixture
`corpus_z000048 +4` bytes and best negative fixture `corpus_z000085 -9` bytes.

Next implementation step: return to normal-mode CPU parity work on focused
`corpus_z000033` level 16. Do not spend more time on target-mode
`corpus_z000021`, `corpus_z000002`, `repo_Cargo.toml`, `corpus_z000016`,
`corpus_z000064`, or `repo_io_std.rs` unless a broader fixture reopens one of
them.

Latest targetCBlockSize tooling result: `profile_c_port` and `profile_c_api`
now accept an optional fourth positional `TARGET_C_BLOCK_SIZE` argument. The
Rust helper is intentionally narrow and returns `None` for invalid target sizes
or strategies whose target mode is not ported yet. The first focused smoke
found and fixed a repeat-FSE-table panic in target mode by making sequence
emitters reject repeat/predefined/mixed table modes that cannot encode the
candidate symbols and restore offset/FSE state. After rebuilding the release
tools, `target/release/profile_c_port
benchmarks/archive/tmp/realworld-100/corpus_z000033 16 1 2048` completed at
`488728` bytes, while `target/release/profile_c_api
benchmarks/archive/tmp/realworld-100/corpus_z000033 16 1 2048` completed at
`477631` bytes. Later target-superblock fixes closed this focused target-mode
compression gap; keep this paragraph as historical context only.

Follow-up targetCBlockSize result: `inspect_c_port_blocks` now supports
`--target-c-block-size` with the C API backend. It showed Rust was trying
all-compressed sequence modes before C-selected mixed/repeat modes for
single-subblock target candidates. `target_block.rs` now tries compressed
Huffman literals with the selected C sequence modes before the all-compressed
fallback, and then tries basic-literal multi-sub-blocks only after the Huffman
multi/single candidates. The target Huffman table builder now also rejects
new tables whose estimated payload plus table description is not smaller than
raw literals, matching C's superblock literal entropy selector, and uses C's
fast Huffman table-log calculation for non-`BtUltra` strategies. The final
planner-estimate fix removed the emitted four-stream jump table from C's rough
target-superblock literal estimate. Together these moved every focused target
smoke to C parity: target 2048 improved from `488728` to `477631` bytes, target
4096 from `479088` to `477121`, and target 8192 from `477751` to `476853`. C
API outputs are `477631`, `477121`, and `476853` respectively. The latest target
inspector artifacts are named
`benchmarks/tmp/inspect-target{2048,4096,8192}-after-superblock-estimate-parity.txt`.

Focused validation should still use
`benchmarks/archive/tmp/realworld-100/corpus_z000033` at level 16 when a change
can affect normal compression. The expected Rust output is `9224480` bytes for
20 runs and `36897920` bytes for 80 runs; the C API 80-run comparison is
`36864480` bytes. Rebuild `target/release/profile_c_port` before comparing
performance if the binary may be stale.

Latest kept change: `update_literal_price()` now uses an explicit `if` when
capturing the previous match for the ULTRA-only match-plus-one-literal
replacement branch. This preserves the earlier parser copy gate while making
the old-match copy unambiguously lazy in Rust source, matching C's branch shape
more closely than the previous `then_some(state.opt[cur])` expression. Focused
bytes stayed stable at `9198420` for 20 runs and `36793680` for 80 runs. Three
focused 20-run instruction samples were `12.460B`, `12.462B`, and `12.462B`
(`12460483361`, `12461702264`, `12461660527`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-explicit-lazy-prev-match-copy.perf.data`.
Validation passed for `cargo fmt --check`, focused parser/price tests, full
`ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes),
`git diff --check`, and a release `profile_c_port` rebuild.

Previous kept change: `rank_limited_nonzero_weights()` now threads the
power-of-two weight sum log from `distribute_weights_with_sum_log()` into
`redistribute_weights_with_sum_log()`, avoiding the old scan over generated
weights before redistribution. The release path uses the sum-log helper
directly; compatibility wrappers are test-only. Focused bytes stayed stable at
`9198420` for 20 runs and `36793680` for 80 runs. Three focused 20-run
instruction samples were `12.470B`, `12.461B`, and `12.466B`
(`12470025456`, `12461412754`, `12466474150`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-rank-limited-sum-log-threading.perf.data`.
Validation passed for `cargo fmt --check`, focused Huffman/weights tests, full
`ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes), and a
release `profile_c_port` rebuild.

Rejected follow-up after the explicit lazy parser-copy checkpoint: adding an
early return in `redistribute_weights_with_sum_log()` when the lower-weight
raise pass produces `added_weights == 0` preserved focused bytes but did not
improve instruction samples (`12.465B`, `12.461B`, `12.466B`) versus the latest
`12.460B`, `12.462B`, `12.462B` checkpoint, so it was reverted. Do not retry
that branch-only shape without new evidence.

Rejected pre-split table-size experiment after the explicit lazy parser-copy
checkpoint: changing `Fingerprint` to const-generic table sizes so hashLog 8
and 9 zero only 256/512 entries, closer to C's `recordFingerprint_generic()`,
preserved focused bytes but regressed the first two focused 20-run instruction
samples to `12.476B` and `12.477B`. It was reverted. Do not retry that safe
const-generic shape without new evidence; the code-size/monomorphization cost
outweighed the smaller zeroing on the focused fixture.

Previous kept change: `update_literal_price()` now delays copying the old
`Optimal` node until the ULTRA-only match-plus-one-literal replacement branch
is actually eligible. This preserves C's price/path logic while avoiding a
full old-node copy for literal replacements that cannot use it. Focused bytes
stayed stable at `9198420` for 20 runs and `36793680` for 80 runs. Three
focused 20-run instruction samples were `12.470B`, `12.467B`, and `12.465B`
(`12469947147`, `12466918100`, `12465320113`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-literal-prev-match-copy-gate.perf.data`.

Performance context: the latest kept code change is the explicit lazy
previous-match copy gate described above. Its profile was dominated by:

- `forward_pass()` around 28%.
- `compress_block_opt_with_state_and_ldm()` around 20%.
- `rank_limited_nonzero_weights()` around 3.0%.
- `HuffmanTable::build_smallest_from_counts()` around 3.0%.
- `FSEEncoder::encode_interleaved()` around 3.0%.
- `HuffmanTable::build_from_code_lengths()` around 2.5%.
- `build_table_from_probabilities()` around 2.4%.
- `table_description_bytes_from_weights()` around 1.6%.

Do not let this older performance profile override the current superblock
checkpoint above. Return to the optimal-parser/match-finder path only after the
target compressed block size / superblock checkpoint is handled or a new
benchmark points there. Treat `benchmarks/tmp/*.perf.data` and benchmark
CSV/Markdown reports as local artifacts unless the user explicitly asks to
commit them.

Rejected sequence-bitstream compile-shape experiment after the short
literal-length increment fast path: forcing the all-table and all-RLE sequence
encoders to remain separate with `#[inline(never)]` preserved focused bytes but
worsened three focused 20-run instruction samples to about `12.485B`,
`12.489B`, and `12.485B` (`12484608895`, `12489007295`, `12485197671`), so it
was reverted.

Rejected literal-length delta experiment after the short literal-length
increment fast path: changing `dynamic_lit_length_code_delta()` to compute the
transition delta directly from bit-count and frequency-weight differences
preserved focused bytes but worsened three focused 20-run instruction samples
to about `12.488B`, `12.490B`, and `12.491B` (`12488111147`, `12489540061`,
`12491306078`), so it was reverted.

Rejected opt-match tree arithmetic experiment after the short literal-length
increment fast path: replacing hot `saturating_sub()` uses in
`opt_match/tree.rs` with explicit C-style `if` branches for `btLow` and
repetitive-pattern skip positions preserved focused bytes but worsened three
focused 20-run instruction samples to about `12.560B`, `12.557B`, and
`12.556B` (`12560230366`, `12556558855`, `12555889182`), so it was reverted.

Rejected static bit-weight lookup experiment after the short literal-length
increment fast path: adding a 4096-entry stateless lookup table for BtOpt
`bit_weight()` preserved focused bytes but worsened three focused 20-run
instruction samples to about `12.604B`, `12.605B`, and `12.603B`
(`12603629925`, `12605207402`, `12603013097`), so it was reverted.

## Current Restart Checkpoint

When resuming the zstd C compressor port on branch
`faithful-c-compressor-port`, read
`ruzstd/src/encoding/levels/c_port/README.md` first. Target compressed block
size / superblock byte parity is broadly validated, so the active checkpoint is
normal-mode CPU parity on focused `corpus_z000033` level 16 unless a new
target-mode benchmark reopens a gap.

The authoritative C compressor source for this port is:

```text
/home/bsutton/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.16+zstd.1.5.7/zstd
```

Immediate resume checklist:

- Read `AGENTS.md` and `ruzstd/src/encoding/levels/c_port/README.md`.
- Check `git status --short --branch` before touching files.
- Keep the C source path above as the authoritative port reference.
- Use `benchmarks/archive/tmp/realworld-100/corpus_z000033` at level 16 as the
  focused benchmark fixture. Expected focused Rust output is `9224480` bytes
  for 20 runs and `36897920` bytes for 80 runs; the C API 80-run comparison is
  `36864480` bytes.
- Inspect current profiles around `opt_parser/forward.rs` and
  `opt_match/tree.rs` before making the next CPU change; target-superblock
  parity is no longer the active checkpoint unless a fresh target benchmark
  reopens it.
- Rebuild `profile_c_port` before benchmark comparisons if the binary may be
  stale.
- Verify the focused level-16 fixture before keeping a performance change:
  `target/release/profile_c_port benchmarks/archive/tmp/realworld-100/corpus_z000033 16 20`.
- Treat `benchmarks/tmp/*.perf.data` and benchmark CSV/Markdown reports as
  local artifacts unless the user explicitly asks to commit them.

Current WIP snapshot for restart:

- Target mode dispatch is already threaded through the CCtx, frame state,
  optimal frame path, and hash-chain frame path.
- The active target encoder tries a literal-only RLE superblock and otherwise
  falls through target sub-block paths before falling back to raw blocks.
- Target compressed block size / superblock byte parity is now broadly
  validated across the 73-fixture set, so do not restart the old
  target-superblock audit unless a fresh target-mode benchmark reopens a gap.
- The next implementation step is normal-mode CPU parity. Focused instruction
  count is effectively at C parity, so prioritize line-level generated-code
  inspection for parser/tree branch count, cycle count, and wall time.

Latest kept parser price note from July 17, 2026:
`OptPriceState::dynamic_lit_length_increment_price()` now handles the dense
literal-length code transitions `1..=16` directly before falling back to the
larger sparse transition match. This is a no-state version of the short
literal-length increment optimization; unlike the rejected pass-local table, it
does not add per-pass stack state. It preserved focused bytes (`9198420` for
20 runs, `36793680` for 80 runs) and reduced three focused 20-run instruction
samples to about `12.481B`, `12.483B`, and `12.483B` (`12481190235`,
`12482917789`, `12483042243`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-short-ll-increment-fast-path.perf.data`.
Validation passed for `cargo fmt --check`, focused price/parser tests, full
`ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes),
`git diff --check`, and a release `profile_c_port` rebuild.

Latest kept parser price follow-up after the short literal-length increment
fast path: `dynamic_lit_length_increment_price_unchecked()` now handles
literal lengths `1..=15` as adjacent dense-code deltas directly, with a
separate `lit_length == 16` transition into the sparse helper. For those dense
codes, `LL_BITS` and the sum-base terms cancel, so the delta is just
`weight(previous_freq) - weight(current_freq)`. Focused bytes stayed unchanged
(`9,224,480` for 20 runs and `36,897,920` for 80 runs). Two focused 20-run
Rust samples were `8,880,236,217` and `8,873,129,257` instructions; the
focused 80-run sample was `34,724,664,562` instructions, `23,541,986,061`
cycles, `5,689,710,362` branches, and `241,570,850` branch misses.
Same-session C API was `36,864,480` bytes, `34,710,851,353` instructions,
`22,595,867,999` cycles, `4,330,895,918` branches, and `176,546,761` branch
misses. This leaves the focused instruction gap at about `+0.04%`; the
remaining measurable gap is cycles, wall time, and branch/control-flow shape.
Validation passed `cargo fmt --check`, focused opt-price and opt-parser tests,
full `cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd --all-targets -- -D
warnings`, release rebuilds for `profile_c_port` and `profile_c_api`, and
`git diff --check`.

Rejected follow-up after the direct dense LL delta checkpoint: adding an
equal-frequency fast path for literal lengths `1..=15`, returning zero before
the two `weight()` calls when adjacent LL frequencies matched, preserved
focused bytes but regressed focused 20-run instruction samples to
`8,905,119,942` and `8,906,188,453` versus the kept `8.873B` to `8.880B`
band. It also raised branch count to about `1.472B` for 20 runs. It was
reverted; do not retry without a fresh profile signal.

Latest kept tiny parser cleanup after the direct dense LL delta checkpoint:
`refresh_node_reps()` now copies the current and previous parser nodes into
locals before writing the updated repcode history, reducing repeated
`state.opt` indexing in the hot forward pass without changing the algorithm.
Focused bytes stayed unchanged (`9,224,480` for 20 runs and `36,897,920` for
80 runs). Three focused 20-run instruction samples were `8,881,873,333`,
`8,876,467,976`, and `8,872,578,047`. The focused 80-run sample was
`34,724,188,577` instructions, `23,785,786,398` cycles, `5,689,785,477`
branches, and `242,995,852` branch misses. Same-session C API was
`36,864,480` bytes, `34,711,691,664` instructions, `22,545,950,850` cycles,
`4,331,112,642` branches, and `177,105,696` branch misses. Treat this as a
neutral-to-tiny instruction cleanup; the remaining work is still branch and
cycle parity.

Latest kept parser borrow cleanup after that checkpoint: `raw_literal_cost()`
now takes `&OptPriceState` and `&mut LiteralPriceCache` directly instead of
borrowing the whole `OptBlockState`. This keeps the literal-price cache shape
unchanged while narrowing aliases in the hot literal-price path. Focused bytes
stayed unchanged (`9,224,480` for 20 runs and `36,897,920` for 80 runs). Three
focused 20-run instruction samples were `8,877,574,614`, `8,874,501,779`, and
`8,874,302,490`. The focused 80-run sample was `34,721,273,930` instructions,
`21,058,286,081` cycles, `5,688,820,453` branches, and `240,112,501` branch
misses. Same-session C API was `36,864,480` bytes, `34,708,905,974`
instructions, `23,368,568,796` cycles, `4,330,775,908` branches, and
`177,607,455` branch misses. The focused instruction gap remains about
`+0.04%`; branch count remains about `+31%`.

Rejected follow-up after the parser borrow cleanup: applying the same
field-borrow split to `update_match_prices()` preserved focused bytes and
reduced branch count slightly, but regressed the focused 20-run instruction
band to `8,884,808,156`, `8,889,492,645`, and `8,888,961,923` versus the kept
`8.874B` to `8.878B` band. It was reverted; keep `update_match_prices()`
taking `&mut OptBlockState` unless a stronger profile signal appears.

Rejected follow-up after the same checkpoint: snapshotting `state.opt[cur]`
once after `refresh_node_reps()` and reusing that local for the skip check, rep
history, zero-literal flag, and large-match stretch preserved focused bytes but
regressed focused 20-run instruction samples to `8,918,738,569`,
`8,913,761,120`, and `8,915,618,718`, with branch count also higher at about
`1.471B` per 20 runs. It was reverted; keep the current direct `state.opt`
access shape unless a new profile gives a much stronger reason.

Rejected July 17 parser price arithmetic experiment after the short
literal-length increment fast path: changing `price_delta()` from an `i64`
widening subtraction to bounded `i32` subtraction preserved focused bytes but
worsened focused 20-run instruction samples to about `12.488B` and `12.487B`
(`12487885688`, `12486717750`), so it was reverted.

Rejected July 17 FSE interleaved encode experiment after the short
literal-length increment fast path: caching `acc_log` once for the two final
state writes in `FSEEncoder::encode_interleaved()` preserved focused bytes but
worsened focused 20-run instruction samples to about `12.485B` and `12.486B`
(`12485299857`, `12486216310`), so it was reverted.

Latest kept Huffman note from July 17, 2026:
`HuffmanTable::build_smallest_from_counts()` now reuses the nonzero symbol
count already available from `base_code_lengths()` when choosing the minimum
candidate bit width. The fallback path still scans counts when the base tree is
unavailable, so table selection and output are unchanged. This preserved
focused bytes (`9198420` for 20 runs, `36793680` for 80 runs) and reduced three
focused 20-run instruction samples to about `12.498B`, `12.497B`, and
`12.495B` (`12498055159`, `12497391247`, `12494754101`). The profile artifact
is
`benchmarks/tmp/profile-c-port-z000033-l16-after-huffman-nonzero-count-reuse.perf.data`.
Validation passed for `cargo fmt --check`, focused Huffman/parser tests, full
`ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes),
`git diff --check`, and a release `profile_c_port` rebuild.

Rejected July 17 parser increment-cache experiment after the Huffman nonzero
count reuse: precomputing a pass-local `[i32; 17]` table for short
literal-length increment prices preserved focused bytes but the first focused
20-run instruction sample regressed to about `12.681B` (`12681380226`) versus
the `12.495B` to `12.498B` checkpoint, so it was reverted.

Latest kept parser note from July 17, 2026: `forward_pass()` now uses
`OptPriceState::dynamic_lit_length_price()` for parser-root and zero-literal
match prices. Optimal parser blocks return before the predefined-price case
(`block_len <= HASH_READ_SIZE`), so this preserves the focused C price model
while avoiding dead predefined-price branches in the hot BtOpt setup and match
price paths. This preserved focused bytes (`9198420` for 20 runs, `36793680`
for 80 runs) and reduced three focused 20-run instruction samples to about
`12.506B`, `12.505B`, and `12.506B` (`12506188790`, `12505167428`,
`12506493283`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-dynamic-literal-length-price-helper.perf.data`.
Validation passed for `cargo fmt --check`, focused price/parser tests, full
`ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes),
`git diff --check`, and a release `profile_c_port` rebuild.

Rejected July 17 parser stats experiment after the dynamic literal-length price
helper: changing `OptPriceState::update_stats()` to take an exact literal slice
instead of C-shaped `(lit_length, literals)` arguments preserved focused bytes
but worsened three focused 20-run instruction samples to about `12.517B`
(`12517484677`, `12517664869`, `12516688879`), so it was reverted.

Latest kept parser note from July 17, 2026: `forward_pass()` now calls
`OptPriceState::dynamic_raw_literal_cost()` when filling the parser's lazy
literal-price cache. Optimal parser blocks return before the predefined-price
case (`block_len <= HASH_READ_SIZE`), and this Rust port has no active setter
that disables compressed literal pricing on this path, so this preserves the
focused C price model while avoiding dead literal-price branches in the hot
BtOpt path. This preserved focused bytes (`9198420` for 20 runs, `36793680`
for 80 runs) and reduced three focused 20-run instruction samples to about
`12.529B`, `12.529B`, and `12.535B` (`12529881732`, `12528528169`,
`12534818387`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-dynamic-raw-literal-price-helper.perf.data`.
Validation passed for `cargo fmt --check`, focused price/parser tests, full
`ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes),
`git diff --check`, and a release `profile_c_port` rebuild.

Latest kept parser note from July 17, 2026: `forward_pass()` now calls
`OptPriceState::dynamic_match_offset_price()` and
`dynamic_match_length_price()` while seeding and updating match prices. Optimal
parser blocks return before the predefined-price case, so this preserves the
focused C price model while avoiding dead predefined branches in the hot BtOpt
match-price path. This preserved focused bytes (`9198420` for 20 runs,
`36793680` for 80 runs) and reduced three focused 20-run instruction samples
to about `12.552B`, `12.549B`, and `12.550B` (`12551897402`, `12549331712`,
`12549532267`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-dynamic-match-price-helpers.perf.data`.
Validation passed for `cargo fmt --check`, focused price/parser tests, full
`ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes),
`git diff --check`, and a release `profile_c_port` rebuild.

Previous kept parser note from July 17, 2026: `forward_pass()` now calls
`OptPriceState::dynamic_lit_length_increment_price()` through its local
`ll_increment_price()` helper. Optimal parser blocks return before the
predefined-price case (`block_len <= HASH_READ_SIZE`), so this preserves the
focused C price model while avoiding the dead predefined branch in the hot
literal update path. This preserved focused bytes (`9198420` for 20 runs,
`36793680` for 80 runs) and reduced three focused 20-run instruction samples
to about `12.579B`, `12.583B`, and `12.578B` (`12579256659`, `12583461014`,
`12577903159`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-dynamic-ll-increment-helper.perf.data`.
Validation passed for `cargo fmt --check`, focused price/parser tests, full
`ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes),
`git diff --check`, and a release `profile_c_port` rebuild. Rejected in the
same pass: replacing match-node struct-update writes with direct field
assignments preserved bytes but worsened focused 20-run instruction samples to
about `12.631B`, `12.633B`, and `12.628B`, so it was reverted.

Previous kept parser note from July 17, 2026: `seed_match_prices()` now returns
the computed `LL_PRICE(0)` value and `forward_pass()` reuses it when calling
`update_match_prices()`. The price state is unchanged between seeding and the
forward pass, so this preserves the C price model while avoiding a duplicate
literal-length price calculation for every parser segment. This preserved
focused bytes (`9198420` for 20 runs, `36793680` for 80 runs) and reduced
three focused 20-run instruction samples to about `12.627B`, `12.625B`, and
`12.628B` (`12626825870`, `12624992297`, `12627862842`). The profile artifact
is
`benchmarks/tmp/profile-c-port-z000033-l16-after-seeded-zero-ll-price-threading.perf.data`.
Validation passed for `cargo fmt --check`, focused parser tests, full `ruzstd`
tests, clippy, an 80-run focused smoke (`36793680` bytes), `git diff --check`,
and a release `profile_c_port` rebuild.

Previous kept parser note from July 17, 2026: `forward_pass()` now computes
`LL_PRICE(0)` once per pass and passes it into `update_match_prices()`. The
price state is unchanged during the pass, so this preserves the C price model
while avoiding repeated literal-length price work before match-price updates.
This preserved focused bytes (`9198420` for 20 runs, `36793680` for 80 runs)
and reduced three focused 20-run instruction samples to about `12.669B`,
`12.665B`, and `12.668B` (`12669108032`, `12664597852`, `12667522755`). The
profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-forward-zero-ll-price-hoist.perf.data`.
Validation passed for `cargo fmt --check`, focused parser tests, full `ruzstd`
tests, clippy, an 80-run focused smoke (`36793680` bytes), `git diff --check`,
and a release `profile_c_port` rebuild. Rejected in the same pass: caching
dynamic literal-length symbol prices and transition increments inside
`OptPriceState` preserved bytes but regressed the first focused 20-run
instruction sample to about `13.520B` (`13519765282`), likely due to extra
state/layout or refresh cost; it was reverted.

Previous kept parser note from July 17, 2026: `forward_pass()` now computes the
ULTRA `LL_INCPRICE(1)` value once per pass and passes it into
`update_literal_price()`. The price state is unchanged during the pass, so this
preserves the C price model while avoiding repeated literal-length increment
work in the hot literal-replacement branch. This preserved focused bytes
(`9198420` for 20 runs, `36793680` for 80 runs). Six focused 20-run
instruction samples were `12.689B`, `12.684B`, `12.689B`, `12.685B`,
`12.684B`, and `12.684B` (`12688839304`, `12684087332`, `12689369275`,
`12685012766`, `12684243667`, `12684362756`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-forward-one-literal-increment-hoist.perf.data`.
Validation passed for `cargo fmt --check`, focused parser tests, full `ruzstd`
tests, clippy, an 80-run focused smoke (`36793680` bytes), `git diff --check`,
and a release `profile_c_port` rebuild.

Previous kept parser note from July 17, 2026: `refresh_node_reps()` removes
the extra Rust-only `mlen == 0` guard and matches C's offset-history refresh
condition, `if (opt[cur].litlen == 0)`. Parser initialization should guarantee
that, for `cur >= 1`, a zero literal length means the node ends in a real
match. This preserved focused bytes (`9198420` for 20 runs, `36793680` for
80 runs) and reduced three focused 20-run instruction samples to about
`12.687B`, `12.688B`, and `12.690B` (`12687496008`, `12687623422`,
`12689835773`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-refresh-node-reps-c-guard.perf.data`.
Validation passed for `cargo fmt --check`, focused parser tests, full `ruzstd`
tests, clippy, an 80-run focused smoke (`36793680` bytes), `git diff --check`,
and a release `profile_c_port` rebuild.

Latest kept Huffman note from July 17, 2026:
`HuffmanTable::build_from_weights()` now computes the minimum nonzero Huffman
weight during its first weight scan and sets `table.max_num_bits` once from
that minimum instead of updating it for every symbol. This preserves canonical
codes because `current_num_bits` is `max_num_bits - weight + 1`, so the maximum
occurs at the smallest nonzero weight. Focused bytes were unchanged (`9198420`
for 20 runs, `36793680` for 80 runs), and three focused 20-run instruction
samples were `12.697B`, `12.701B`, and `12.698B` (`12697186055`,
`12701063410`, `12698498767`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-huffman-max-bits-once.perf.data`.
Validation passed for `cargo fmt --check`, focused Huffman tests, full
`ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes),
`git diff --check`, and a release `profile_c_port` rebuild.

Avoid reapplying already rejected experiments unless a fresh profile points
there: moving FSE spread-symbol scratch to the stack, lowering the FSE direct
lookup threshold, adding a tracked `CodeCounts::max_symbol` field, the one-entry
parser `LiteralCostCache`, the direct-mapped match-length cache, the 64-entry
literal-price cache, the parser path-order reversal change, the ext-dict
repcode helper forced-inline experiment, the deeper FSE transform-table
experiment, the pre-split workspace active-clear reuse experiment, the raw
offBase enum round-trip cleanup experiment, forcing `seed_parser_root()` or
`forward_pass()` inline, forcing inline on the loaded-dictionary low-bound
helpers, using input weights directly for Huffman table descriptions, shrinking
the Huffman-weight FSE count array to 16 entries, replacing sequence-cost
`iter_present()` chains with direct loops, removing the hot `BtMatchRequest`
wrapper, making the no-dict repcode helper const-generic, and replacing literal
stream counting's chunk iterator with a fixed four-slice loop. Also avoid the
direct Huffman table-description-from-codes experiment: it preserved bytes but
regressed the focused instruction samples to about `12.94B`. Rejected FSE
experiments from the same checkpoint: replacing the release direct-lookup
unsafe initialization with safe sentinel or zero-filled vectors preserved bytes
but regressed focused samples to about `12.79B` to `12.80B`; forcing
`FSETable::c_start_state_index()` inline preserved bytes but was slightly worse
at about `12.718B` to `12.720B`. Rejected parser experiments from the same
checkpoint: copying `state.opt[cur]` into a local `Optimal` after rep refresh
preserved bytes but regressed focused samples to about `12.80B`; replacing the
half-bit skip threshold helper call with a local constant preserved bytes but
only sampled around `12.716B` to `12.718B`, so it was reverted as noise/worse
than the checkpoint.

First resume note from July 17, 2026: the unvalidated one-entry
`LiteralCostCache` experiment in `opt_parser/forward.rs` was rejected and
reverted. It preserved focused bytes (`9198420` for 20 runs) but worsened the
20-run instruction samples to about `18.39B` (`18.396B`, `18.390B`,
`18.393B`) versus the prior `18.366B`, `18.359B`, `18.360B` baseline. Do not
reintroduce that cache without new evidence. After the revert, `cargo
fmt --check`, focused `opt_parser` and `opt_match` tests, and the release
`profile_c_port` build passed; the focused smoke still produced `9198420`
bytes and one instruction sample returned to about `18.356B`.

Latest kept performance note from July 17, 2026: Huffman length limiting now
reuses the already sorted leaf order from `length_limited_code_lengths()` when
entering `limit_code_lengths()`, removing a duplicate non-zero-symbol
collection and sort. This preserved focused bytes (`9198420` for 20 runs,
`36793680` for 80 runs) and reduced three focused 20-run instruction samples to
about `17.07B` (`17.069B`, `17.070B`, `17.064B`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-huffman-limit-sort-reuse.perf.data`.
Validation passed for `cargo fmt --check`, `cargo test -p ruzstd huff --quiet`,
`cargo test -p ruzstd opt_parser --quiet`, `cargo test -p ruzstd --quiet`,
`cargo clippy -p ruzstd --all-targets -- -D warnings`, and `git diff --check`.

Follow-up kept Huffman note from July 17, 2026: `build_smallest_from_counts()`
now computes the base Huffman code lengths once and reuses cloned base lengths
for each candidate max-bit limit, instead of rebuilding the same Huffman tree
for every candidate. This preserved focused bytes (`9198420` for 20 runs,
`36793680` for 80 runs) and reduced three focused 20-run instruction samples to
about `15.97B` (`15.972B`, `15.974B`, `15.971B`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-huffman-base-length-reuse.perf.data`.
Validation passed for `cargo fmt --check`, `cargo test -p ruzstd huff --quiet`,
`cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd --all-targets -- -D warnings`,
and `git diff --check`.

Latest kept Huffman note from July 17, 2026: `build_smallest_from_counts()` now
also derives the initial best table from the same cached base Huffman lengths,
removing the remaining duplicate `build_from_counts()` tree build before the
candidate max-bit search. This preserved focused bytes (`9198420` for 20 runs,
`36793680` for 80 runs) and reduced three focused 20-run instruction samples to
about `15.50B` (`15.498B`, `15.503B`, `15.500B`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-huffman-best-base-reuse.perf.data`.
Validation passed for `cargo fmt --check`, focused Huffman and parser tests,
`cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd --all-targets -- -D warnings`,
and `git diff --check`.

Follow-up kept Huffman note from July 17, 2026: rank-limited Huffman table
construction inside `build_smallest_from_counts()` now reuses the base Huffman
pass's existing symbols sorted by count, preserving tie order while avoiding a
second non-zero-symbol sort. This preserved focused bytes (`9198420` for
20 runs, `36793680` for 80 runs) and reduced three focused 20-run instruction
samples to about `15.34B` (`15.340B`, `15.340B`, `15.338B`). The profile
artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-rank-limited-order-reuse.perf.data`.
Validation passed for `cargo fmt --check`, focused Huffman/rank-limited/parser
tests, `cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd --all-targets -- -D warnings`,
and `git diff --check`.

Latest kept Huffman note from July 17, 2026: `build_from_weights()` now
precomputes per-weight symbol counts and starting codes, then assigns canonical
codes in one symbol pass instead of scanning all symbols once per weight rank.
This preserved focused bytes (`9198420` for 20 runs, `36793680` for 80 runs)
and reduced three focused 20-run instruction samples to about `14.83B`
(`14.823B`, `14.827B`, `14.827B`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-huffman-code-start-table.perf.data`.
Validation passed for `cargo fmt --check`, focused Huffman/parser tests,
`cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd --all-targets -- -D warnings`,
and `git diff --check`.

Follow-up kept Huffman note from July 17, 2026: rank-limited weight generation
now consumes the generated non-zero weights in forward order instead of
reversing the vector and popping it back to the original order; the distributed
weight vector also reserves its exact capacity. This preserved focused bytes
(`9198420` for 20 runs, `36793680` for 80 runs) and reduced three focused
20-run instruction samples to about `14.80B` (`14.796B`, `14.800B`,
`14.803B`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-rank-limited-forward-consume.perf.data`.
Validation passed for `cargo fmt --check`, focused Huffman/parser tests,
`cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd --all-targets -- -D warnings`,
and `git diff --check`.

Latest kept parser note from July 17, 2026: `forward_pass()` now keeps a
per-pass lazy literal-price cache for `raw_literal_cost()` lookups. This avoids
repeating the hot literal `highbit32()` work without reintroducing the rejected
eager 256-entry `OptPriceState` cache refresh cost. This preserved focused
bytes (`9198420` for 20 runs, `36793680` for 80 runs) and reduced three focused
20-run instruction samples to about `14.70B` (`14.770B`, `14.771B`,
`14.769B`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-forward-literal-price-cache.perf.data`.
Validation passed for `cargo fmt --check`, focused parser/price tests,
`cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd --all-targets -- -D warnings`,
and `git diff --check`.

Latest kept FSE note from July 17, 2026: `FSETable` now caches the
`c_start_state_index()` result for each symbol while building the table,
matching C's precomputed `FSE_initCState2()` transform path instead of
reconstructing the symbol rank during each stream initialization. This
preserved focused bytes (`9198420` for 20 runs, `36793680` for 80 runs) and
reduced three focused 20-run instruction samples to about `14.74B`
(`14.735B`, `14.735B`, `14.736B`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-fse-start-state-cache.perf.data`.
Validation passed for `cargo fmt --check`, `cargo test -p ruzstd fse --quiet`,
`cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd --all-targets -- -D warnings`,
and `git diff --check`.

Latest kept literal-stats note from July 17, 2026:
`LiteralStats::from_literals_with_stream_counts()` now uses chunk iteration for
four-stream literal counts instead of computing `pos / split_size` for every
literal while estimating split partitions. This preserved focused bytes
(`9198420` for 20 runs, `36793680` for 80 runs) and reduced three focused
20-run instruction samples to about `14.44B` (`14.436B`, `14.443B`,
`14.436B`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-literal-stats-chunked-stream-counts.perf.data`.
Validation passed for `cargo fmt --check`, focused literal/compressed/Huffman
tests, full `ruzstd` tests, clippy, an 80-run focused smoke
(`36793680` bytes), and `git diff --check`.

Latest kept optimal-parser allocation note from July 17, 2026: the optimal
parser now reserves sequence output at `block_len / min_match`, matching C's
`ZSTD_maxNbSeq(blockSize, minMatch, ...)` capacity formula for the no external
sequence-producer path. The previous Rust reserve was effectively
`block_len / (min_match * 4)`, which caused extra reallocation/memmove work in
the hot level-16 fixture. This preserved focused bytes (`9198420` for 20 runs,
`36793680` for 80 runs) and reduced three focused 20-run instruction samples
to about `13.93B` (`13.928B`, `13.928B`, `13.928B`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-c-sequence-capacity.perf.data`.
Validation passed for `cargo fmt --check`, focused `opt_parser` tests, full
`ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes), and
`git diff --check`.

Latest kept Huffman note from July 17, 2026: `redistribute_weights()` now
caches each weight's half-rank unit while repaying overflow instead of
recomputing `1 << (weight - 1)` in the repeated scan. This preserves the old
scan order and candidate selection while removing hot-loop shift work. Focused
bytes were unchanged (`9198420` for 20 runs, `36793680` for 80 runs), and
three focused 20-run instruction samples were about `13.904B` (`13.904B`,
`13.904B`, `13.904B`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-huffman-redistribute-unit-cache.perf.data`.
Validation passed for `cargo fmt --check`, focused Huffman tests, full
`ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes), and
`git diff --check`.

Latest kept FSE note from July 17, 2026: direct lookup construction now uses a
debug-asserted `u16` cast for bounded per-symbol state indexes instead of a
release-time `u16::try_from()` branch in the nested lookup fill loop. Focused
bytes were unchanged (`9198420` for 20 runs, `36793680` for 80 runs), and
three focused 20-run instruction samples were about `13.900B` (`13.900B`,
`13.900B`, `13.900B`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-fse-lookup-index-cast.perf.data`.
Validation passed for `cargo fmt --check`, focused FSE tests, full `ruzstd`
tests, clippy, an 80-run focused smoke (`36793680` bytes), and
`git diff --check`.

Latest kept parser cache note from July 17, 2026: the full 256-entry lazy
literal-price cache now lives in `OptBlockState` and uses per-pass generation
stamps, preserving the lazy per-symbol lookup behavior while avoiding the
per-forward-pass `[u32; 256]` sentinel memset. A follow-up narrowed those stamps
from `u32` to `u16`, reducing cache footprint while keeping rare wraparound
clearing. This is distinct from the rejected eager `OptPriceState` literal
table, one-entry cache, and 64-entry direct-mapped cache shapes. Focused bytes
were unchanged (`9198420` for 20 runs, `36793680` for 80 runs), and three
focused 20-run instruction samples were `13.868B`, `13.868B`, and `13.868B`
(`13868231751`, `13868206912`, `13868208454`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-literal-cache-u16-generation.perf.data`.
Validation passed for `cargo fmt --check`, focused parser and price tests, full
`ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes), and
`git diff --check`.

Latest kept Huffman note from July 17, 2026: the rank-limited non-zero weight
generator now stores temporary weights as `u8` and casts back to `usize` only
when distributing them into the public table-builder input. The generated
weight values are tiny (`<= MAX_HUFFMAN_BITS`), so this preserves output while
reducing temporary memory traffic in `rank_limited_nonzero_weights()`. Focused
bytes were unchanged (`9198420` for 20 runs, `36793680` for 80 runs), and three
focused 20-run instruction samples were `13.867B`, `13.867B`, and `13.867B`
(`13866999168`, `13867018924`, `13867018692`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-rank-limited-u8-weights.perf.data`.
Validation passed for `cargo fmt --check`, focused Huffman tests, full
`ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes), and
`git diff --check`.

Latest kept optimal-parser workspace note from July 17, 2026: the normal
optimal encode path now returns its temporary `StoredSequence` vector to
`OptBlockState` after `prepare_from_greedy_output()` has copied it into the
prepared block. The next optimal block takes that buffer back and reserves only
when its capacity is too small, matching C's reused seqStore shape more closely
without unsafe code. Focused bytes were unchanged (`9198420` for 20 runs,
`36793680` for 80 runs), and three focused 20-run instruction samples were
`13.820B`, `13.820B`, and `13.820B` (`13820460179`, `13820386739`,
`13820461433`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-opt-sequence-scratch.perf.data`.
Validation passed for `cargo fmt --check`, focused parser tests, full `ruzstd`
tests, clippy, an 80-run focused smoke (`36793680` bytes), and a release
`profile_c_port` rebuild.

Latest kept FSE table note from July 17, 2026: `FSETable` now stores
`SymbolStates` only up to the actual normalized-probability slice length
instead of always constructing a 256-symbol array. Missing symbols still report
probability zero, while valid encoded symbols use the same state lookup path.
This matches the C encoder's max-symbol bounded FSE table construction more
closely and avoids the hot fixed 256-entry initialization cost without adding
unsafe code. Focused bytes were unchanged (`9198420` for 20 runs,
`36793680` for 80 runs), and three focused 20-run instruction samples were
`13.045B`, `13.045B`, and `13.045B` (`13045049097`, `13045339529`,
`13045048963`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-fse-symbol-vector.perf.data`.
Validation passed for `cargo fmt --check`, focused FSE tests, full `ruzstd`
tests, clippy, an 80-run focused smoke (`36793680` bytes), `git diff --check`,
and a release `profile_c_port` rebuild.

Latest kept pre-split note from July 17, 2026:
`pre_split::Fingerprint::record()` now dispatches the four C-generated
fingerprint recorder modes `(43,8)`, `(11,9)`, `(5,10)`, and `(1,10)` to
const-specialized safe Rust helpers. This mirrors `zstd_preSplit.c`'s
compile-time record functions while preserving C's `limit / samplingRate`
`nbEvents` accounting. Focused bytes were unchanged (`9198420` for 20 runs,
`36793680` for 80 runs), and three focused 20-run instruction samples were
`12.915B`, `12.915B`, and `12.915B` (`12914867945`, `12914576966`,
`12914576818`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-presplit-specialized-fingerprint.perf.data`.
Validation passed for `cargo fmt --check`, focused pre-split tests, full
`ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes),
`git diff --check`, and a release `profile_c_port` rebuild.

Latest kept optimal-match plumbing note from July 17, 2026: `BtMatchRequest`
now carries the parser's raw `[u32; 3]` repcode array, and
`collect_repcode_matches()` consumes that array directly. This matches
`zstd_opt.c`'s direct `rep` array flow and removes the hot-path
`RepeatOffsets::from_offsets(...).as_offsets()` wrapper churn from every
optimal match collection. Focused bytes were unchanged (`9198420` for 20 runs,
`36793680` for 80 runs), and three focused 20-run instruction samples were
`12.906B`, `12.905B`, and `12.905B` (`12905636712`, `12905345365`,
`12905345539`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-raw-rep-match-request.perf.data`.
Validation passed for `cargo fmt --check`, focused optimal match/parser tests,
full `ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes),
`git diff --check`, and a release `profile_c_port` rebuild.

Latest kept Huffman note from July 17, 2026:
`build_smallest_from_counts()` now reuses a cached rank-limited weight vector
when the base Huffman candidate is unavailable, avoiding a duplicate build and
evaluation of the same rank-limited fallback table. Candidate selection and
canonical table output are unchanged. Focused bytes were unchanged (`9198420`
for 20 runs, `36793680` for 80 runs), and three focused 20-run instruction
samples were `12.901B`, `12.902B`, and `12.902B` (`12901378143`,
`12901668749`, `12901669146`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-huffman-rank-limited-fallback-reuse.perf.data`.
Validation passed for `cargo fmt --check`, focused Huffman tests, full
`ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes),
`git diff --check`, and a release `profile_c_port` rebuild.

Latest kept FSE note from July 17, 2026: `SymbolStates::get()` now keeps the
rare state-search fallback in a cold, non-inlined helper. The common small-table
path still uses the direct lookup vector, matching C's transform-table style
more closely in the encoder hot loop, while large-table fallback behavior is
unchanged. A fresh rebuild before this change measured the focused level-16
fixture at about `13.407B` instructions (`13405533398`, `13410399268`,
`13406722006`), so the older `12.902B` checkpoint number should be considered
stale unless reproduced. The FSE cold-helper change preserved focused bytes
(`9198420` for 20 runs, `36793680` for 80 runs), and three focused 20-run
instruction samples were `13.396B`, `13.396B`, and `13.394B` (`13396119373`,
`13395971850`, `13394379395`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-fse-cold-search-helper.perf.data`.
Validation passed for `cargo fmt --check`, `cargo test -p ruzstd fse --quiet`,
full `ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes),
`git diff --check`, and a release `profile_c_port` rebuild.

Rejected July 17 offBase cleanup experiment after the Huffman rank-limited
fallback reuse change: direct repcode offBase construction and a checked
non-`Option` `OffBase::from_c_value_valid()` conversion preserved focused bytes
but worsened focused 20-run instruction samples to about `13.44B`
(`13439378761`, `13440527691`, `13440943666`, then `13444795891`,
`13440953835` with an inline hint), so it was reverted.

Rejected July 17 micro-experiments after the FSE cold search helper: changing
`next_rank_limited_weight()` from `slice.get()` to direct indexing preserved
focused bytes but worsened focused 20-run instruction samples to about
`13.40B` (`13404474554`, `13402053703`, `13400211416`); replacing hot
`Optimal { ..state.opt[pos] }` match-stretch writes with individual field
assignments also preserved bytes but worsened samples to about `13.41B`
(`13414804913`, `13411296647`, `13410483366`); combining the FSE normalized
probability total and max-symbol scans preserved bytes but landed around
`13.40B` (`13401203353`, `13398717471`, `13400125353`). All three were
reverted. Treat the compiler's current struct-update and iterator shapes as
measured, not obviously wasteful, unless a new profile says otherwise.

Latest kept parser note from July 17, 2026: `seed_match_prices()` is now
`#[inline(always)]`, matching C's inlined initial match-price setup inside
`ZSTD_compressBlock_opt_generic()` instead of keeping it as a separate hot
symbol. Focused bytes were unchanged (`9198420` for 20 runs, `36793680` for
80 runs), and three focused 20-run instruction samples were `13.390B`,
`13.392B`, and `13.387B` (`13389556631`, `13392412402`, `13387000775`). The
profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-inline-seed-match-prices.perf.data`.
Validation passed for `cargo fmt --check`, focused parser tests, full `ruzstd`
tests, clippy, an 80-run focused smoke (`36793680` bytes), `git diff --check`,
and a release `profile_c_port` rebuild.

Latest kept parser note from July 17, 2026: the optimal parser now has a const
`WITH_LDM` specialization threaded through the MLS/strategy dispatch,
`forward_pass()`, and `collect_matches_mls()`. Normal no-LDM calls compile out
the LDM candidate processing branch, while calls with a cursor still use the
ported C-style LDM candidate path. Focused bytes were unchanged (`9198420` for
20 runs, `36793680` for 80 runs), and three focused 20-run instruction samples
were `13.358B`, `13.353B`, and `13.351B` (`13357588368`, `13353432010`,
`13350670590`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-no-ldm-specialization.perf.data`.
Validation passed for `cargo fmt --check`, focused parser tests, full `ruzstd`
tests, clippy, an 80-run focused smoke (`36793680` bytes), `git diff --check`,
and a release `profile_c_port` rebuild.

Latest kept sequence-cost note from July 17, 2026: C-style optimal sequence
table selection no longer performs a full `default_allowed()` scan before
computing basic-table cross entropy, and `repeat_table_cost()` no longer scans
once to prove support before scanning again to compute bit cost. Unsupported
symbols still return `None` from `cross_entropy_cost()` or `bit_cost()`, so the
selection result is unchanged while redundant symbol-count scans are removed.
Focused bytes were unchanged (`9198420` for 20 runs, `36793680` for 80 runs),
and three focused 20-run instruction samples were `13.300B`, `13.299B`, and
`13.294B` (`13299515088`, `13298522967`, `13294162063`). The profile artifact
is
`benchmarks/tmp/profile-c-port-z000033-l16-after-sequence-cost-scan-removal.perf.data`.
Validation passed for `cargo fmt --check`, focused sequence tests, full
`ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes),
`git diff --check`, and a release `profile_c_port` rebuild.

Rejected July 17 parser inline experiment after the no-LDM specialization:
forcing `seed_parser_root()` inline preserved focused bytes but worsened three
focused 20-run instruction samples to about `13.44B` (`13440615080`,
`13438463217`, `13436664765`) versus the `13.35B` baseline, so it was reverted.

Latest kept Huffman note from July 17, 2026:
`HuffmanTable::build_from_weights()` now uses an explicit loop with a zero skip
instead of `weights.iter().copied().filter(...)` when summing weight counts and
max weight. Focused bytes were unchanged (`9198420` for 20 runs, `36793680`
for 80 runs), and three focused 20-run instruction samples were `13.297B`,
`13.295B`, and `13.298B` (`13297100154`, `13295365817`, `13297570728`). The
profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-huffman-weight-loop-cleanup.perf.data`.
Validation passed for `cargo fmt --check`, focused Huffman tests, full
`ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes),
`git diff --check`, and a release `profile_c_port` rebuild.

Rejected July 17 Huffman/FSE experiments after the sequence-cost scan removal:
using the input `weights` directly for Huffman table descriptions broke
Huffman/compressed-block tests with decode corruption and changed the expected
C-style sparse short weight table size, so it was reverted; shrinking
`build_huffman_weight_table_from_data()` from a 256-entry count array to a
16-entry count array preserved focused bytes but worsened focused 20-run
instruction samples to about `13.42B` (`13342441267`, `13342413364`,
`13341600468`), so it was reverted; replacing sequence-cost `iter_present()`
chains with direct loops preserved focused bytes but regressed samples to about
`13.300B` (`13300498198`, `13300799788`, `13300154161`), so it was reverted.

Latest kept match-path note from July 17, 2026:
`should_stop_after_best_match()` now checks `matches.is_empty()` and reads the
last match with `get(len - 1)` directly instead of using
`matches.last().is_some_and(...)`. This preserves the C-style "stop if latest
best match is sufficient or reaches block end" condition while avoiding the
option/closure path in the hot match collection loop. Focused bytes were
unchanged (`9198420` for 20 runs, `36793680` for 80 runs), and three focused
20-run instruction samples were `13.270B`, `13.269B`, and `13.268B`
(`13270212203`, `13268583477`, `13268343682`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-direct-best-match-stop.perf.data`.
Validation passed for `cargo fmt --check`, focused match/parser tests, full
`ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes),
`git diff --check`, and a release `profile_c_port` rebuild.

Rejected July 17 parser/match inline experiments after the Huffman weight-loop
cleanup: forcing `forward_pass()` inline preserved focused bytes but worsened
three focused 20-run instruction samples to about `13.45B` (`13455426768`,
`13452489912`, `13454993841`), so it was reverted; forcing inline on
`OptMatchBounds::lowest_match_index()` and
`lowest_prefix_index_with_loaded_dict()` preserved focused bytes but was neutral
to slightly worse at about `13.30B` (`13299036958`, `13297773146`,
`13295556789`), so it was reverted.

Latest kept match-path note from July 17, 2026:
`collect_repcode_matches_no_dict()` now unrolls the fixed no-dictionary repcode
probe order instead of iterating over a computed `first_rep..last_rep` range,
and shares the per-rep probe in `try_repcode_match_no_dict()`. This mirrors the
C path's tiny fixed repcode loop more closely while leaving ext-dict repcode
handling unchanged. Focused bytes were unchanged (`9198420` for 20 runs,
`36793680` for 80 runs), and three focused 20-run instruction samples were
`12.950B`, `12.949B`, and `12.950B` (`12950491653`, `12948870263`,
`12949894502`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-unrolled-nodict-repcodes.perf.data`.
Validation passed for `cargo fmt --check`, focused match/parser tests, full
`ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes),
`git diff --check`, and a release `profile_c_port` rebuild.

Latest kept Huffman note from July 17, 2026: `distribute_weights()` now fills
runs of identical generated rank-limited weights with `Vec::resize()` instead
of repeatedly pushing the same value. Focused bytes were unchanged (`9198420`
for 20 runs, `36793680` for 80 runs), and three focused 20-run instruction
samples were `12.947B`, `12.944B`, and `12.943B` (`12947222283`,
`12943655778`, `12942817724`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-rank-limited-resize-fill.perf.data`.
Validation passed for `cargo fmt --check`, focused Huffman tests, full `ruzstd`
tests, clippy, an 80-run focused smoke (`36793680` bytes), `git diff --check`,
and a release `profile_c_port` rebuild.

Rejected July 17 experiments after the unrolled no-dict repcodes: removing the
hot `BtMatchRequest` wrapper preserved focused bytes but regressed focused
20-run instruction samples to about `12.98B` (`12976512552`, `12979895227`,
`12981386844`), so it was reverted; making `try_repcode_match_no_dict()`
const-generic over the repcode was noise-level to slightly worse after six
samples (`12947905035`, `12950739383`, `12950125935`, `12955112427`,
`12950030887`, `12951598971`), so it was reverted; replacing literal stream
counting's `chunks(split_size).enumerate()` loop with a fixed four-slice loop
preserved focused bytes but regressed samples to about `12.95B`
(`12954098953`, `12952131889`, `12953769793`), so it was reverted.

Latest kept literal-stats note from July 17, 2026:
`LiteralStats::from_literals_with_stream_counts()` now counts literals first
and computes `largest` plus `max_symbol` in a fixed 256-counter pass afterward,
instead of updating both maxima for every literal while counting. This mirrors
C's histogram-count-then-inspect shape and preserves the optional four-stream
counts. Focused bytes were unchanged (`9198420` for 20 runs, `36793680` for
80 runs), and three focused 20-run instruction samples were `12.751B`,
`12.750B`, and `12.751B` (`12751122656`, `12750146084`, `12751293243`). The
profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-literal-stats-post-count-max.perf.data`.
Validation passed for `cargo fmt --check`, focused literal/compressed tests,
full `ruzstd` tests, clippy, an 80-run focused smoke (`36793680` bytes),
`git diff --check`, and a release `profile_c_port` rebuild.

Rejected July 17 pre-split workspace experiment after the raw repcode match
request change: reusing two `Fingerprint` workspaces and clearing only the
active hash range preserved focused bytes but worsened three focused 20-run
instruction samples to about `12.918B` (`12917779526`, `12917779276`,
`12917781583`) versus the `12.905B` baseline, so it was reverted.

Rejected July 17 parser/match experiment after the Huffman redistribution
unit-cache change: forcing `OptMatchBounds::rep_match_length()` inline preserved
focused bytes but worsened the first focused 20-run instruction sample to about
`13.952B` versus the `13.904B` baseline, so it was reverted. The focused
fixture is no-dictionary and the extra code size outweighed removing the cold
ext-dict helper call.

Rejected July 17 parser cache experiments after the kept literal-price cache:
a direct-mapped per-pass `match_length_price()` cache preserved focused bytes
but worsened the 20-run instruction sample to about `14.864B`; shrinking the
literal-price cache to a 64-entry direct-mapped cache also preserved bytes but
worsened the sample to about `14.801B`. Both were reverted. Keep the full
256-entry lazy literal-price cache unless a new profile supports a different
shape.

Rejected July 17 parser match-length price experiment after the FSE start-state
cache: a tiny per-match ML-code price cache in `forward.rs` preserved focused
bytes (`9198420` for 20 runs) but worsened the first 20-run instruction sample
to about `14.868B` versus the `14.735B` to `14.736B` baseline, so it was
reverted. Do not retry this shape; the extra branch/cache state costs more than
the avoided repeated code-price work on the focused fixture.

Rejected July 17 parser path-order experiment: leaving `select_path()` output
in backward order and consuming it with `path.iter().rev()` removed the explicit
`path.reverse()` and preserved focused bytes, but worsened two 20-run
instruction samples to about `14.757B` and `14.755B` versus the current
`14.735B` to `14.737B` baseline. It was reverted; keep the explicit reverse
unless a new profile shows a different path-construction bottleneck.

Rejected July 17 FSE transform-table experiment: adding C-style
`deltaNbBits`/`deltaFindState` transforms and a C-ordered state table for
`FSEEncoder::encode_symbol()` preserved focused bytes (`9198420` for 20 runs)
after fixing Rust's explicit bit masking and the single-symbol wrapping
subtraction, but the first focused instruction sample worsened to about
`14.990B` versus the `14.733B` to `14.737B` baseline. It was reverted; the
extra table construction/state-table work costs more than the avoided range
lookup on the focused fixture.

Rejected July 17 Huffman table-description experiment: deriving serialized
table-description weights directly from the original weight vector preserved
focused bytes but worsened focused instruction samples to about `15.447B`
(`15.447B`, `15.447B`, `15.447B`) versus the `15.34B` baseline, so it was
reverted. Keep `weights_from_codes()` unless a new profile shows a different
reason to change that path.

As of July 16, 2026, target mode dispatch is threaded through the CCtx, frame
state, optimal frame path, and hash-chain frame path. The active target encoder
tries literal-only RLE superblocks, non-empty sequence sub-blocks with
Huffman compressed or treeless literal metadata, and basic literal sub-blocks
with all-RLE, all-repeat, or all-compressed sequence metadata before falling
back to raw blocks. The resolved `targetCBlockSize` value is consumed by the
multi-sub-block path, which can write a full-superblock Huffman literal table
once, use treeless literal sections later, write full-superblock FSE sequence
tables once on the Huffman-literal and basic-literal paths, use table-backed
mixed LL/ML/OF modes when they can be repeated by later sub-blocks, use repeat
sequence metadata for later sub-blocks, and fall back to a single-subblock path
that can select mixed LL/ML/OF sequence entropy modes.

The target multi-sub-block paths now include raw-tail fallback for cases where
only part of the target superblock compresses. After at least one compressed
sub-block has been committed, the final sub-block attempt snapshots FSE and
repeat-offset state, and if the final sub-block is not worth committing it
restores to the committed prefix and appends a raw block for the remaining
source bytes.

Target sequence entropy selection is now strategy-aware and reuses the shared
C-style sequence table selector from the regular compressed-block path. Fast
strategies use the C fast heuristic, optimal strategies use the C cost model,
and target-mode tests include a direct guard for that strategy split.
Target multi-sub-block encoding now also primes temporary repeat tables after
the first committed sequence entropy write, so later sub-blocks can use repeat
mode after C-style basic/RLE/mixed metadata without making those basic/RLE
tables repeat-valid for later normal blocks.
Target multi-sub-block planning now uses C-style rough estimates for Huffman
literal sections and sequence sections, including the hard-coded 3-byte
sequence-header estimate and FSE table-definition costs, instead of estimating
by fully emitting a candidate block.
The final target block wrapper now applies C's whole-superblock minimum-gain
acceptance gate and restores FSE/offset state before emitting a single raw
block when the target candidate is rejected.

Validation at the July 16 checkpoint passed for `cargo fmt --check`,
`cargo test -p ruzstd --quiet`, `cargo clippy -p ruzstd --all-targets -- -D warnings`,
`git diff --check`, and the release build of `profile_c_port`,
`benchmark_c_port`, and `profile_c_api`. The focused smoke test
`target/release/profile_c_port benchmarks/archive/tmp/realworld-100/corpus_z000033 16 80`
produced `36793680` total output bytes.

The broad API benchmark over available `realworld-100` fixtures had 73 usable
files because 27 fixture links were broken. Rust output was slightly smaller
than C output at levels 8, 16, and 19, while CPU still lagged at optimal levels:
level 8 `-559` bytes, level 16 `-880` bytes, and level 19 `-211` bytes versus
C. The broad benchmark artifacts are `benchmarks/tmp/agent-validation.csv` and
`benchmarks/tmp/agent-validation.md`.

Active WIP: investigate the remaining level-16/level-19 CPU gap before adding
more broad superblock functionality. The dominant focused case is
`benchmarks/archive/tmp/realworld-100/corpus_z000033` at level 16. A local
`perf stat` run over 20 iterations after the literal-length increment fast path
and C-style match-finder branch conditions showed Rust producing `9198420`
total bytes in about `2.52s`, while C API produced `9216120` total bytes in
about `1.42s`. After reusing precomputed sequence code counts in the
post-split estimator, Rust uses about `20.92B` instructions versus C's `8.71B`,
so the current gap is mostly instruction count while Rust compression is already
slightly smaller. The current 80-run focused samples are Rust `36793680` bytes
in about `9.4s` to `10.2s` versus C API `36864480` bytes in about `4.9s`. Recent Rust perf
artifacts are `benchmarks/tmp/profile-c-port-z000033-l16.perf.data`,
`benchmarks/tmp/profile-c-port-z000033-l16-after-ll-fastpath.perf.data`,
`benchmarks/tmp/profile-c-port-z000033-l16-after-match-branch-shape.perf.data`,
and `benchmarks/tmp/profile-c-port-z000033-l16-after-estimate-count-reuse.perf.data`.

Latest resume note from July 16, 2026: an estimate-only offBase conversion fast
path in `sequence_bitstream.rs` was added so estimator calls can reuse
preencoded C-port sequence offset values without replaying local offset-history
updates when all sequences already have encoded offsets. This preserved the
focused level-16 bytes and reduced the hot 20-run instruction sample to about
`19.11B` instructions. The artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-estimate-offbase-fastpath.perf.data`.
Full `ruzstd` tests and clippy now pass after this change.

A follow-up parser micro-change replaced the hot `price_i32()` saturating
`TryFrom<u32>` conversion with a debug-asserted cast, matching the C parser's
bounded `int` price arithmetic. It preserved the focused level-16 bytes and
reduced the 20-run sample to about `19.10B` instructions. This is a small
cleanup, not a material closure of the C CPU gap.

A follow-up literal-price micro-change caches the scalar
`lit_sum_base_price - BITCOST_MULTIPLIER` as `lit_price_max` when base prices
are refreshed. This preserved the focused level-16 bytes and stayed around
`19.10B` instructions over three 20-run samples (`19.096B`, `19.104B`,
`19.098B`). Treat it as neutral-to-tiny cleanup, not a material CPU win.

Latest handoff note from July 16, 2026: the post-split allocation pass was
checked and kept. `PreparedChunk` / `prepared_chunk()` are test-only. The hot
path now preallocates the split list, the final partition output buffer, and
each partition encoder's raw-size buffer. This preserved the focused
level-16 bytes (`9198420` for 20 runs) and produced three `perf stat` samples
around `19.09B` instructions (`19.092B`, `19.089B`, `19.090B`). Treat this as a
tiny allocation cleanup, not a material closure of the C CPU gap. The next
high-value work remains the optimal-parser and match-finder hot paths,
especially `opt_match::tree::bt_get_all_matches_no_dict_mls` and
`opt_parser::forward::forward_pass`.

Fresh profile note from July 16, 2026: after the post-split preallocation
cleanup, `perf record` artifact
`benchmarks/tmp/profile-c-port-z000033-l16-after-post-split-prealloc.perf.data`
showed the same dominant hot spots: `forward_pass` at about `21.1%`,
`compress_block_opt_with_state_and_ldm` at about `15.9%`,
`post_split::estimate_partition_size_with_sequences` at about `4.8%`, and
`__memmove_avx_unaligned_erms` at about `4.8%`. A suspected C/Rust
`nextToUpdate` parity gap was checked and is already handled by the public
`opt_match.rs` wrapper before dispatching into the lower-level tree helper.
Do not add a duplicate guard in `opt_match/tree.rs`.

Latest parser note from July 16, 2026: `OptPriceState::match_price()` was split
into exact offset and match-length components, and `forward_pass` now hoists the
offset-price component once per `OptMatch` while scanning candidate match
lengths. This preserves the C price formula and focused level-16 bytes
(`9198420` for 20 runs), while three `perf stat` samples dropped to about
`19.05B` to `19.06B` instructions (`19.057B`, `19.058B`, `19.050B`). This is
another small parser cleanup, not a material closure of the C CPU gap.

Latest handoff note from July 17, 2026: `FSEEncoder::encode_symbol()` now has
`#[inline(always)]`. This preserved focused level-16 bytes (`9198420` for
20 runs) and dropped focused instruction samples from about `19.05B` after the
match-price hoist to about `18.73B` (`18.729B`, `18.732B`, `18.728B`). The
profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-fse-encode-symbol-inline.perf.data`.
The interrupted validation was later completed: `cargo clippy -p ruzstd
--all-targets -- -D warnings` and `git diff --check` passed, and the short
report showed `forward_pass` and `compress_block_opt_with_state_and_ldm`
remaining dominant.

Rejected experiments from the same pass: reusing Huffman weight-table analysis
inside `huff0_encoder.rs` preserved bytes but worsened the focused sample to
about `19.075B` instructions, and hoisting `state.opt[cur].rep` in the parser
candidate loop preserved bytes but worsened the sample to about `19.09B`
instructions. Both were reverted. Do not reapply them without new evidence.

Latest match-count note from July 17, 2026: `count_match_no_dict()` now mirrors
C's `ZSTD_count()` loop shape by precomputing `match_limit - 7`, doing the first
word compare before the repeated loop, and then comparing `pos < loop_limit`
instead of recomputing `pos + 8 <= match_limit` each iteration. This preserved
focused bytes (`9198420` for 20 runs, `36793680` for 80 runs) and reduced three
20-run instruction samples to about `18.56B` (`18.567B`, `18.558B`,
`18.565B`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-count-match-loop-limit.perf.data`.
Validation passed for `cargo fmt --check`, `cargo test -p ruzstd match_count
--quiet`, `cargo test -p ruzstd opt_match --quiet`, `cargo test -p ruzstd
opt_parser --quiet`, `cargo clippy -p ruzstd --all-targets -- -D warnings`,
and `git diff --check`.

Rejected July 17 literal-price cache experiment: caching all 256 literal symbol
prices in `OptPriceState` preserved focused bytes but made refresh work dominate
and worsened the 20-run sample to about `22.70B` instructions. It was reverted;
do not reintroduce it without a different refresh strategy and fresh evidence.

Latest Huffman note from July 17, 2026: `HuffmanNode` now uses sentinel `usize`
values for missing parent/symbol instead of `Option<usize>`, matching the
compact C workspace shape more closely, and `length_limited_code_lengths()`
reserves the exact parent-node capacity before building the tree. This preserved
focused bytes (`9198420` for 20 runs, `36793680` for 80 runs). The sentinel-only
samples were about `18.49B` instructions (`18.495B`, `18.487B`, `18.486B`);
with the parent-node reserve they were about `18.48B` (`18.485B`, `18.479B`,
`18.477B`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-huffman-node-workspace.perf.data`.
Validation passed for `cargo fmt --check`, `cargo test -p ruzstd huff --quiet`,
`cargo clippy -p ruzstd --all-targets -- -D warnings`, and `git diff --check`.

Latest FSE note from July 17, 2026: `build_table_from_data()` and
`build_huffman_weight_table_from_data()` now update `max_symbol` while counting
input symbols, removing the follow-up 256-entry count-table scan before
normalization. This preserved focused bytes (`9198420` for 20 runs,
`36793680` for 80 runs) and reduced three 20-run instruction samples to about
`18.43B` (`18.434B`, `18.430B`, `18.429B`). The profile artifact is
`benchmarks/tmp/profile-c-port-z000033-l16-after-fse-max-symbol-inline.perf.data`.
Validation passed for `cargo fmt --check`, `cargo test -p ruzstd fse --quiet`,
`cargo clippy -p ruzstd --all-targets -- -D warnings`, and `git diff --check`.

Latest parser note from July 17, 2026: `update_match_prices()` no longer writes
the current rep history into every candidate match endpoint. C only writes
`mlen`, `off`, `litlen`, and `price` in this path; Rust still refreshes rep
history when a priced endpoint becomes the current node. This preserved focused
bytes (`9198420` for 20 runs, `36793680` for 80 runs) and reduced three 20-run
instruction samples to about `18.40B` (`18.405B`, `18.403B`, `18.398B`).
Validation passed for `cargo fmt --check`, `cargo test -p ruzstd --quiet`,
`cargo test -p ruzstd opt_parser --quiet`, `cargo test -p ruzstd opt_match
--quiet`, `cargo test -p ruzstd fse --quiet`, `cargo clippy -p ruzstd
--all-targets -- -D warnings`, and `git diff --check`.

Follow-up parser note from July 17, 2026: `seed_match_prices()` now also leaves
rep history untouched for initial match endpoints, matching C's initial match
seeding path. Rep history is still present for literal states and is refreshed
when a match endpoint becomes current. This preserved focused bytes (`9198420`
for 20 runs, `36793680` for 80 runs) and reduced three 20-run instruction
samples to about `18.39B` (`18.398B`, `18.390B`, `18.388B`). Validation passed
for `cargo fmt --check`, `cargo test -p ruzstd --quiet`, `cargo test -p ruzstd
opt_parser --quiet`, `cargo test -p ruzstd opt_match --quiet`, `cargo clippy
-p ruzstd --all-targets -- -D warnings`, and `git diff --check`.

Follow-up parser initialization note from July 17, 2026: the initial
`seed_match_prices()` placeholder loop now mutates only `price`, `mlen`, and
`litlen`, matching C's partial initialization before the literal update pass.
This avoids rewriting `off` and rep history for placeholders that will be fixed
by normal literal propagation. It preserved focused bytes (`9198420` for
20 runs, `36793680` for 80 runs) and reduced three 20-run instruction samples
to about `18.38B` (`18.385B`, `18.380B`, `18.376B`). Validation passed for
`cargo fmt --check`, `cargo test -p ruzstd --quiet`, `cargo test -p ruzstd
opt_parser --quiet`, `cargo test -p ruzstd opt_match --quiet`, `cargo clippy
-p ruzstd --all-targets -- -D warnings`, and `git diff --check`.

Follow-up parser frontier note from July 17, 2026: the `update_match_prices()`
frontier extension loop now mutates only `price` and nonzero `litlen` for empty
positions, matching C's `opt[last_pos].price = ZSTD_MAX_PRICE` and
`opt[last_pos].litlen = !0` writes. It preserved focused bytes (`9198420` for
20 runs, `36793680` for 80 runs) and reduced three 20-run instruction samples
to about `18.37B` (`18.373B`, `18.372B`, `18.370B`). Validation passed for
`cargo fmt --check`, `cargo test -p ruzstd --quiet`, `cargo test -p ruzstd
opt_parser --quiet`, `cargo test -p ruzstd opt_match --quiet`, `cargo clippy
-p ruzstd --all-targets -- -D warnings`, and `git diff --check`.

Follow-up parser literal note from July 17, 2026: `update_literal_price()` now
reads only the previous node's `price` and `litlen` before comparing the literal
candidate price, and copies the full previous `Optimal` only after the literal
path wins. This mirrors C's `opt[cur] = opt[cur-1]` placement after the price
comparison. It preserved focused bytes (`9198420` for 20 runs, `36793680` for
80 runs) and reduced three 20-run instruction samples to about `18.36B`
(`18.366B`, `18.359B`, `18.360B`). Validation passed for `cargo fmt --check`,
`cargo test -p ruzstd --quiet`, `cargo test -p ruzstd opt_parser --quiet`,
`cargo test -p ruzstd opt_match --quiet`, `cargo clippy -p ruzstd
--all-targets -- -D warnings`, and `git diff --check`.

Historical parser performance note: before the checkpoint moved back to target
superblock parity, the prior investigation was inspecting the optimal-parser and
match-finder hot paths. A
debug-line release profile was refreshed at
`benchmarks/tmp/profile-c-port-z000033-l16-current-debug-lines.perf.data`;
source-line sorting collapsed to codegen units, so use `perf annotate --stdio` on
`opt_parser::forward::forward_pass` and related symbols for local instruction
evidence. `perf report --stdio --no-children` after the estimate-only offBase
fast path showed
`opt_parser::forward::forward_pass`, `compress_block_opt_with_state_and_ldm`,
`post_split::estimate_partition_size_with_sequences`, Huffman table
construction, and `FSEEncoder::encode_symbol` near the top. The
literal-length increment-price
fast path in `opt_price.rs` was kept because it preserved bytes and modestly
improved the focused A/B timing. C-style non-short-circuit match-finder branch
conditions in `opt_match` were also kept because they preserved bytes and the
longer focused timing/counter samples improved modestly. The post-split
estimator now reuses already-computed LL/ML/OF code counts for non-exact
estimate table selection; this preserved bytes and cut the focused instruction
count, though long wall-clock samples were roughly neutral. A C-shaped
no-dictionary repcode window guard in `opt_match` was also kept because it
preserved focused bytes and shaved a tiny number of instructions from the hot
level-16 sample. A direct `OptMatchTable::get()` accessor was kept because it
preserved focused bytes and cut the level-16 focused instruction count to about
`20.88B` by avoiding the sliced `Index` path in hot parser loops. The no-dict
repcode guard now also precomputes C's `curr - windowLow` distance and uses
wrapping `rep[0] - 1`; this preserved focused bytes and cut the hot sample to
about `20.83B` instructions. The initial parser match-seed sentinel now mirrors
C by resetting only the sentinel price instead of overwriting the full
`Optimal`; this preserved focused bytes and shaved the hot sample to about
`20.82B` instructions. The later C-shaped `count_match_no_dict()` loop-limit
change from July 17 was also kept; see the note above for its current evidence.
Huffman table construction now caches the serialized
table-description bytes so encode-time emission does not rebuild the same
description; after fixing the small-table nibble order, full `ruzstd` tests pass
and the hot sample is about `20.69B` instructions. The literal C cost model and
block-size estimator now reuse that cached table-description length instead of
subtracting two encoded-length estimates, preserving focused bytes and reducing
the hot sample to about `20.66B` instructions. Post-split recursion now reuses
the parent-computed half-size estimates when a split is accepted, avoiding a
duplicate estimator call for each accepted child interval; this preserved
focused bytes and reduced the hot sample to about `19.18B` instructions. The fixed boxed optimal table, cached
literal-price table, earlier failed match-count loop variants, match-candidate
partial-field mutation, forced inline `refresh_node_reps()`, small LL/ML/OF
symbol-price cache in `OptPriceState`, cached dynamic literal-length transition
prices in `OptPriceState`, and tracked
`CodeCounts::max_symbol` experiments, and the one-entry parser
`LiteralCostCache` experiment preserved bytes but made the focused timing or
instruction count worse, so they were reverted. Do not keep speculative
performance changes without evidence.

If resuming after a context reset, first inspect these files and the worktree
status:

```sh
git status --short --branch
sed -n '1,140p' ruzstd/src/encoding/blocks/compressed/sequence_codes.rs
sed -n '230,285p' ruzstd/src/encoding/levels/c_port/opt_price.rs
sed -n '1,180p' ruzstd/src/encoding/levels/c_port/opt_price_tests.rs
target/release/profile_c_port benchmarks/archive/tmp/realworld-100/corpus_z000033 16 20
cargo test -p ruzstd opt_parser --quiet
cargo test -p ruzstd opt_match --quiet
```

## Operating Principle

Use delegation to remove waiting, log scraping, and mechanical follow-through
from the main agent. Keep code ownership, architectural judgment, benchmark
interpretation, and PR positioning with the main agent.

The default sidecar mode is no-edit. A worker should only change files when the
main agent gives an exact file list, exact command sequence, and exact expected
outcome. Prefer narrow prompts over sharing broad conversation history.

## Performance and Validation Delegation

Use a cheaper sidecar agent for repeatable validation and benchmark runs when the
main task is active code work, porting, or review. The main agent should keep
ownership of code changes, interpretation of results, and final commit decisions.

Preferred split:

- Main agent: inspect C/Rust behavior, make scoped code changes, review diffs,
  decide whether to keep or revert experiments, and write commits.
- Sidecar worker: run deterministic checks and benchmark commands, then report
  exact pass/fail status and numbers. Sidecar workers must not edit files or
  revert changes.

Use a low-cost worker model for this validation loop when available. Give the
worker a self-contained prompt rather than forking full history if model
overrides are needed.

Recommended worker scopes:

- `validation-worker`: runs `cargo fmt`, focused tests, clippy, and build
  commands; reports exact failures and artifact paths.
- `benchmark-worker`: runs fixed benchmark commands; reports CSV/Markdown paths
  and summary lines without interpreting whether the PR is worthwhile.
- `ci-worker`: collects GitHub Actions status and failing log excerpts; does not
  choose fixes.
- `hygiene-worker`: reports `git status`, changed-file lists, diff stats,
  generated artifacts, and large-file or Rust file-size checks.
- `commit-worker`: stages an exact file list and commits an exact message after
  the main agent has decided both.

## Cost Control

Delegation should reduce the expensive model's time spent waiting on commands
and reading large logs. It will not reduce CPU time, wall-clock time for the
benchmark itself, or the cost of running the benchmark machine.

Use a sidecar when all of these are true:

- the task is command-heavy or log-heavy,
- the expected output can be summarized in a few lines,
- the worker does not need to make design decisions,
- the main agent can verify the result from the reported command and artifact
  paths.

Do not delegate when the prompt would need large parts of the repository
history, when interpreting the result requires code ownership context, or when
the worker would need to choose between competing implementation approaches.

Do not use delegation merely to avoid thinking about a result. A sidecar should
produce evidence; the main agent should decide what that evidence means.

Keep sidecar prompts narrow. Include the current branch, exact command, expected
artifact paths, and report format. Avoid sending full prior conversation
history unless the worker genuinely needs it.

## Other Good Delegation Targets

Delegate tasks that are mechanical, bounded, and easy for the main agent to
verify. Good sidecar tasks include:

- validation runs,
- benchmark summarisation and CSV comparison,
- worktree hygiene checks,
- diff stats and changed-file summaries,
- Rust file-size reports,
- documentation consistency checks,
- CI log collection and failure summaries,
- no-edit code searches,
- post-change smoke tests.

Delegation is most likely to reduce cost when the task is long-running,
command-heavy, low-reasoning, and easy to summarize. It does not reduce machine
time, and it can add overhead if prompts or reports are too broad.

Keep design decisions, risky code changes, benchmark interpretation, PR
positioning, and final keep-or-revert decisions in the main agent.

## Sidecar Prompt Patterns

Validation prompt:

```text
You are a no-edit validation worker in /home/bsutton/git/zstd-rs on branch
<branch>. Do not modify files. Run the following commands exactly, capture
pass/fail and the important error lines, and report latest commit plus
git status --short:

<commands>
```

Benchmark prompt:

```text
You are a no-edit benchmark worker in /home/bsutton/git/zstd-rs on branch
<branch>. Do not modify files. Run the following benchmark command exactly.
Then summarize the resulting CSV with the supplied awk command and report the
artifact paths, exact summary lines, latest commit, and git status --short:

<benchmark command>
<summary command>
```

Commit prompt:

```text
You are a commit worker in /home/bsutton/git/zstd-rs on branch <branch>. Stage
only these files:

<files>

Inspect git diff --cached and commit with this exact message:

<message>

Report the new commit hash and git status --short. Do not amend, rebase, reset,
revert, or stage any other files.
```

## Delegating Commits

Commit execution can be delegated only after the main agent has decided the
exact commit scope and message. The sidecar prompt must name the exact files to
stage and the exact commit message.

For delegated commits, require the worker to:

- confirm the latest commit and worktree status,
- stage only the named files,
- inspect `git diff --cached`,
- commit with the exact requested message,
- report the new commit hash.

Sidecar workers must not decide what belongs in a commit, stage unrelated files,
amend commits, rebase, reset, or revert changes.

Standard validation command set:

```sh
cargo fmt --check
cargo test -p ruzstd --quiet
cargo clippy -p ruzstd --all-targets -- -D warnings
cargo build --release -p zstd-rs-tools --bin profile_c_port --bin benchmark_c_port --bin profile_c_api
```

Focused benchmark:

```sh
target/release/profile_c_port benchmarks/archive/tmp/realworld-100/corpus_z000033 16 80
```

Broad benchmark:

```sh
target/release/benchmark_c_port --fixtures benchmarks/archive/tmp/realworld-100 --levels 8,16,19 --runs 1 --c-backend api --csv-output benchmarks/tmp/agent-validation.csv --md-output benchmarks/tmp/agent-validation.md --no-sync
```

Broad benchmark summary:

```sh
awk -F, 'NR>1{rows[$2]++; c[$2]+=$4; r[$2]+=$5; ccpu[$2]+=$7; rcpu[$2]+=$8; gap=$5-$4; if(!(($2) in worst) || gap>worst[$2]){worst[$2]=gap; worstf[$2]=$1}} END{for (l in rows) printf "L%s rows=%d c_bytes=%d rust_bytes=%d gap=%+d gap_pct=%+.3f c_cpu=%.4f rust_cpu=%.4f ratio=%.2f worst=%s %+d\n", l, rows[l], c[l], r[l], r[l]-c[l], (r[l]-c[l])*100/c[l], ccpu[l], rcpu[l], rcpu[l]/ccpu[l], worstf[l], worst[l]}' benchmarks/tmp/agent-validation.csv
```

Sidecar reports should include:

- latest commit and worktree status,
- pass/fail for each validation command,
- exact focused benchmark output for each run,
- broad benchmark summary lines,
- any command failures.
