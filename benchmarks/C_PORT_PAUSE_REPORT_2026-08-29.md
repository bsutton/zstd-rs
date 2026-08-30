# C Compressor Port Pause Report — 2026-08-29

This is the short restart and benchmark summary. `AGENTS.md` remains the
canonical durable handoff, and
`ruzstd/src/encoding/levels/c_port/README.md` contains the detailed retained and
rejected experiment history.

## Workspace at pause

- Branch: `faithful-c-compressor-port`.
- Committed HEAD: `433d266` (`Reuse selected sequence modes in target multi`).
- The C-port work after that commit is intentionally uncommitted: 97 modified
  tracked paths and 37 untracked paths after writing this report, with a
  tracked diff of about 24,373 insertions and 5,748 deletions.
- Do not reset, clean, or discard untracked paths. In particular,
  `ruzstd-{fast,dfast,huff0,row,seqstore}-codegen/` and the split module
  directories are retained source.
- Authoritative C reference: zstd 1.5.7 under the local `zstd-sys-2.0.16`
  checkout named in the C-port README.
- Focus fixture: `benchmarks/archive/tmp/realworld-100/corpus_z000033`.
- Correctness baseline: the latest full gate passed 737 library tests (732
  passed, 5 ignored), every workspace/codegen/tool target, strict all-target
  Clippy, formatting, release builds, `git diff --check`, and all 1,095 broad
  normal/target-2048/prepared-CDict rows. The pause report itself only added
  documentation and ran a decode-verified four-fixture benchmark; it did not
  rerun the full gate.

## Current version versus C zstd

Compression size is effectively at parity and is usually marginally smaller
than C. On the latest 73-fixture normal-mode matrix, Rust-minus-C aggregate
bytes are:

| C level | C bytes | Rust bytes | Rust - C | Gap | Positive rows |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 1,739,297 | 1,737,626 | -1,671 | -0.0961% | 0 |
| 3 | 1,589,014 | 1,587,731 | -1,283 | -0.0807% | 0 |
| 5 | 1,558,284 | 1,557,532 | -752 | -0.0483% | 0 |
| 8 | 1,542,583 | 1,542,369 | -214 | -0.0139% | 0 |
| 16 | 1,379,173 | 1,378,858 | -315 | -0.0228% | 0 |
| 19 | 1,304,019 | 1,303,845 | -174 | -0.0133% | 1 (+2 bytes) |
| 22 | 1,304,045 | 1,303,981 | -64 | -0.0049% | 1 (+2 bytes) |

Source: `benchmarks/tmp/c-dfast-u32-counts-normal-retained.csv`. All rows decode
and the byte columns are unchanged by the latest retained representation
cleanup.

The stable cross-level CPU checkpoint for 20 focused runs is 567,188,864,
987,941,835, 2,079,254,880, 2,660,557,934, and 7,551,897,701 Rust
instructions at levels 1/3/5/8/16. Against the paired C measurements, levels 1
and 3 remain approximately 13.29% and 17.16% above C. Levels 5 and 8 are about
5.0% and 7.6% below C, and level 16 is also ahead. Treat instruction counts as
the stable CPU signal; cycles are noisier and code-layout changes can move the
guard levels enough that only same-binary A/B results should decide whether a
small experiment is kept.

For the separately retained level-16 80-run paired sample, Rust produced
36,844,400 bytes versus C's 36,864,480 (-0.0545%), used 31,036,963,180 versus
34,608,032,979 instructions (-10.32%), and used 17,678,427,275 versus
18,731,833,914 cycles (-5.62%). Rust still executed 4,805,010,253 versus
4,309,914,383 branches (+11.49%) and had 193,896,413 versus 173,172,484 branch
misses. The residual level-16 issue is control-flow shape, not total instruction
count or compressed size.

## Current version versus the original Rust fork base

There are two useful historical Rust comparators. The archived benchmark's
`upstream` column is the original/reference Rust implementation from which the
compression work was forked. Commit `456fbe1`, preserved in
`benchmarks/tmp/worktrees/pre-c-port-base`, is the later immediate branch point
for the faithful C-port and already contains substantial pre-port compression
work. Their public levels were qualitative (`Fastest`, `Default`, `Better`,
`Best`), not the new numeric C-level API, so comparisons across the two level
systems are only exact where stated.

At the old level 1 / current C level 1 on the same four retained fixtures, the
aggregate original-upstream output was 42,955,302 bytes and the current C-port
output is 40,068,766 bytes: 2,886,536 bytes smaller, a 6.72% reduction. Per
fixture:

| Fixture | Original Rust L1 | Current Rust C-L1 | Change |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | 5,976,095 | 5,355,397 | -620,698 (-10.39%) |
| `json_logs_32m.jsonl` | 3,392,237 | 1,155,032 | -2,237,205 (-65.95%) |
| `repeated_text_32m.txt` | 31,757 | 3,127 | -28,630 (-90.15%) |
| `xorshift_32m.bin` | 33,555,213 | 33,555,210 | -3 |

The immediate pre-C-port `456fbe1` implementation produced 39,572,435 bytes on
the same old level-1 suite (5,324,267; 690,084; 2,874; and 33,555,210). The
current C-level-1 total is 496,331 bytes larger (+1.25%) because the custom
pre-port heuristic compressed the synthetic JSON fixture far beyond C, while
the faithful port intentionally follows C's level-1 decisions. Against C on
the fresh run, current Rust is only 3,811 aggregate bytes smaller (-0.0095%).

The original `Best` level was documented as roughly C level 11, but it was a
different heuristic compressor rather than a faithful level mapping. On that
approximate comparison, original-upstream `Best` totals 39,007,206 bytes and
current C-level 11 totals 39,214,480 bytes (+0.53%). The immediate pre-C-port
implementation totals 39,038,926 bytes, making current +0.45%. The difference
is concentrated in structured fixtures where the old heuristic sometimes beat
C substantially: original-upstream/current are 4,902,872/4,659,474 for
`decodecorpus_pack`, 546,250/996,662 for JSON, 2,874/3,134 for repeated text,
and 33,555,210/33,555,210 for xorshift. This is not a regression in the port's
stated goal: the current API deliberately follows C's numeric strategies and
is much more predictable across real-world data, dictionaries, target block
sizes, and all levels.

The fresh current-versus-C run on those four fixtures is in
`benchmarks/tmp/pause-2026-08-29-original-suite-current-vs-c.csv`. Aggregate
current size is 0.0095% smaller than C at level 1 and 0.0088% smaller at level
11. Its one-run `/usr/bin/time` CPU values are smoke data only and should not be
mixed with the hardware-counter results above. The original values come from
`benchmarks/archive/system-tmp/zstd-rs-benchmark-source-current-summary.md`.

## Current version versus latest upstream

Upstream `master` was fetched at `eb7e03c` (`ruzstd` 0.9.1). An isolated
external-consumer harness built both dependencies with one release codegen unit
and compared upstream `CompressionLevel::Fastest` with this port's level 1.
Higher upstream modes are still marked unimplemented, so level 1 is the only
exact current-upstream comparison.

Across the 73-file, 3,395,938-byte corpus, this port emits 1,737,641 bytes and
upstream emits 1,884,617 bytes. The port is 146,976 bytes (7.80%) smaller and is
larger on one row. On `corpus_z000033`, 20 runs produce 11,430,500 versus
12,382,020 bytes. Three-repeat `perf stat` averages are:

| Implementation | Instructions | Cycles | Branches | Branch misses |
| --- | ---: | ---: | ---: | ---: |
| This port, level 1 | 556,142,594 | 228,370,612 | 72,757,039 | 2,102,388 |
| Upstream 0.9.1, `Fastest` | 2,573,721,448 | 1,154,823,518 | 373,654,471 | 8,642,232 |
| zstd 1.5.7 C, level 1 | 507,683,580 | 184,803,245 | 49,990,496 | 1,612,169 |

Thus this focused build uses about 78.39% fewer instructions and 80.22% fewer
cycles than latest upstream while producing 7.68% fewer bytes. Against C it is
byte-identical for this fixture, but uses about 9.55% more instructions and
23.58% more cycles. Counter artifacts are
`benchmarks/tmp/latest-upstream-fastest-{current-l1,upstream,c-zstd-1.5.7}.stat`.
The harness is `benchmarks/tmp/downstream-compare`.

The 73-file set remains a strong differential regression matrix, not a
sufficient publication performance corpus: it is only about 3.4 MiB and has
many short files. Add licensed large-file material, official zstd-generated
interoperability/dictionary fixtures, synthetic data classes, multi-gigabyte
streaming, and allocator/RSS tests before making release-wide claims.

## Publication checkpoint

- The release package now selects Zstandard's `BSD-3-Clause` option as its
  outbound license and carries both its provenance and the required inherited
  ruzstd MIT attribution.
- Generated kernel calls with caller-dependent memory invariants are explicitly
  unsafe and documented; `unsafe_op_in_unsafe_fn` is denied in the derived
  compressor and five kernel units.
- ELF, COFF, and Mach-O now use target-specific function-section syntax; wasm
  omits layout sections. Release builds pass for x86-64 Apple objects, AArch64
  Android, wasm `no_std`, and a final Windows/MinGW PE link. Native execution on
  Apple/Windows/AArch64 and execution on a real x86-64 CPU without BMI2 remain
  open. The diagnostic `force-scalar` feature passed the full library gate (732
  tests, 5 ignored) through portable compressor paths on this BMI2 host.
- Rewise was requested but was not available in the current plugin catalogue.
  Do not imply that its structural assessment has run.
- The five kernel crates were not a proven optimization: their introduction
  also changed algorithms and generated code. They have now been consolidated
  into private main-crate modules while preserving primitive interfaces and
  section attributes. A preserved-binary topology comparison found instruction
  changes of -0.0060%/-0.0016%/-0.0054%/-0.0043%/-0.0131% at levels
  1/3/5/8/16, with exact output bytes. Retain the single package; the separate
  crates had no measurable performance benefit.

## What is retained

- Byte parity for normal mode, target compressed block sizes, external and
  attached dictionaries, and prepared CDicts across the validated matrices.
- Private generated Fast, DFast, row-search, sequence-store, and Huffman modules
  with cached CPU-feature dispatch and isolated BMI2/portable functions.
- Native 12-byte stored sequences through entropy for all nine C strategies.
- Matcher-produced repeat-offset handoff into deferred entropy state.
- C-width `u32` DFast sequence counts through normalization/table building.
- Frame-owned Huffman/FSE construction scratch plus current/next output-table
  recycling and deferred entropy-state commit.
- Direct prepared-sequence entropy and explicit fixed-width Huffman batch
  emission.

## Resume here

1. Read `AGENTS.md`, then the C-port README, then inspect `git status --short
   --branch`. Do not alter the worktree while orienting.
2. Rebuild the release profilers if needed and collect fresh same-session Rust
   and C instruction/branch profiles for `corpus_z000033` at levels 1 and 3.
3. Do not work on Fast or DFast matching: both matchers already use fewer
   absolute instructions than their C counterparts.
4. Use attribution to select a coherent shared boundary in Huffman
   construction/emission or sequence statistics/table selection. A useful
   candidate must remove producer-to-consumer work and preserve generated code
   layout across Fast/DFast and Greedy/Lazy guards.
5. Run the required SIMD tranche: audit optimized assembly for the four-lane
   literal histogram first, then use the refreshed attribution to evaluate
   AVX2 versus SSE2 row masks, a length-gated SIMD long-match comparison, and
   batched LL/ML classification. Retain runtime dispatch and portable scalar
   fallbacks. Record ARM NEON feasibility separately; do not infer its results
   from x86 measurements.
6. Validate candidates with an exact same-binary control, focused counters,
   the 1,095-row decode/byte matrix, full workspace tests, strict Clippy,
   formatting, release builds, and `git diff --check` before keeping them.

The SIMD work is an evidence-gathering tranche, not a presumption that wider
instructions will win. BMI2 entropy specialization is already retained but is
not SIMD. Fast and DFast matching are presently cheaper than C, histogram
updates require indexed counters, and Huffman/FSE construction is branchy and
dependency-heavy. Prioritize only candidates supported by refreshed absolute
attribution, and reject any change that improves a microbenchmark while
regressing stable whole-compressor instructions or another strategy guard.

Do not retry the initialized row candidate buffer, blanket alignment,
pointer-only FSE state caching, wrapper-only stored-sequence orchestration,
in-record cached sequence-code markers, global C high-bit LL/ML formulas, or
the standalone generated Huffman-weight FSE serializer without new profile and
causal evidence. Their exact correctness gates passed, but their measured CPU
results were neutral or regressions.
