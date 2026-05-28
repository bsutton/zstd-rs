# Codex Workplan

## Current Goal

Branch: `compression/huffman-maxheight`

Implement and verify the two follow-up compression improvements after merging the two-candidate matcher:

1. Encoder-side Zstd repeat-offset history for sequence offsets.
2. Conservative sequence FSE table selection, using the C zstd implementation as the guide.

Keep the branch correct against the local Rust decoder and C zstd decoder, then benchmark against upstream and C zstd. Treat excellent test coverage as part of the goal: a performance win is not complete unless its correctness invariants are covered or the workplan explicitly justifies why benchmark-only coverage is appropriate.

Quality constraints:

- Keep the Rust implementation high quality, well structured, and idiomatic.
- Avoid `unsafe` code. Treat safe Rust as a goal constraint, not just a preference.
- Prefer clear state machines and small helpers over clever code that is harder to verify.
- Maintain excellent test coverage for each compression feature; retained changes should have tests that make the behavior hard to regress.
- Cover private invariants with focused unit tests, especially compact matcher state such as suffix candidates and repeat-offset history.
- Cover emitted bitstreams with end-to-end tests through the Rust decoder and the C zstd decoder; helper-level tests alone are not enough.
- Every new compression heuristic should get either a regression test for the intended behavior or an explicit workplan note explaining why benchmark-only coverage is appropriate.

## C zstd Guidance Used

- Repeat offsets: mirror the decoder/spec rules and the C compressor's repeat-code choice/update behavior.
- FSE table modes: follow the conservative fast-path idea from C zstd: use predefined tables for tiny sequence counts, repeat previous tables only when the symbols are valid, and avoid broad heuristics without cost modeling.
- SIMD/hardware-vector work should target the matcher later, especially match extension/comparison. The repeat-offset and table-selection paths are scalar/control-heavy and are not good SIMD candidates.
- On the current `rustc 1.94.1` toolchain, `std::simd`/portable SIMD is still unstable, so direct SIMD in the encoder would require nightly, an additional dependency, or unsafe target intrinsics. Those conflict with the current safe-Rust/no-new-risk constraints unless a future change explicitly revisits that tradeoff.

## Completed

- Added encoder repeat-offset state to `CompressState`.
- Reset repeat-offset state and previous FSE table state at the start of each frame.
- Encoded offsets using repeat-code values when they match the current offset history.
- Added tests for:
  - repeat offsets with non-zero literal lengths,
  - shifted repeat offsets with zero literal lengths,
  - new offset history updates,
  - tiny-sequence predefined FSE table selection,
  - small-block repeat FSE table selection.
- Avoided per-block temporary code vectors for FSE table selection.
- Added a scalar `OffsetHistory` struct instead of a `[u32; 3]` array in encoder state.
- Found and fixed a correctness issue in raw fallback: if `compress_fastest` builds a compressed block and then discards it as raw, it now restores FSE and repeat-offset encoder history because the decoder will not see the discarded compressed block.
- Deferred committing a new Huffman table until `compress_fastest` knows the compressed block will actually be emitted. This avoids cloning `HuffmanTable`, which is not `Clone`, and avoids committing entropy history for a discarded raw fallback block.
- Added encoder support for RLE sequence table modes and a decoder-facing RLE bitstream round-trip test.
- Increased the default fastest matcher window from one 128 KiB block to four 128 KiB blocks, matching the 512 KiB level-1 window observed from C zstd on the repeated-text fixture.
- Fixed match-length code 52 encoding, which had the wrong baseline and corrupted streams once cross-block matches exposed match lengths above 65,538 bytes.
- Added a cross-block repetitive-data test that verifies both the Rust decoder and C zstd decoder can decode the emitted stream and that compression stays compact.
- Added a sampled incompressibility gate so random-looking blocks skip expensive match search and are emitted raw while still being committed for future history.
- Added a matcher fast path for incompressible raw blocks that marks the block processed without indexing every suffix. This preserves safe behavior for custom matchers via a default trait method while allowing the default matcher to avoid wasted history work.
- Added shortest-form raw literals headers for 0-31 and 32-4095 byte raw literal sections, matching the Zstd format and reducing per-block overhead.
- Enabled overlapping same-block match extension using the current block's original bytes. This lets the matcher emit long matches with small offsets instead of artificial doubling sequences.
- Reworked overlapping match extension to compare contiguous slices in chunks instead of one byte at a time, staying in safe Rust and adding focused tests for same-block overlap, previous-window lookups, and chunk-boundary mismatches.
- Added matcher-side repeat-offset probing. The default matcher now stays synchronized with the encoder repeat-offset history, probes repeat offsets even when no suffix entry exists, and restores matcher repeat-offset state when a compressed attempt is discarded as raw.
- Added safe backward extension for hash-table match candidates, following C zstd's fast parser behavior of moving a match start back toward the current sequence anchor when the preceding bytes also match.
- Replaced the rough Huffman previous-table reuse heuristic with an exact encoded-size estimate. The estimator is covered against the real single-stream and four-stream Huffman encoders, and avoids full trial encoding of the literal payload.
- Added a text-like block classifier that uses a longer non-repeat match threshold on mostly printable blocks while preserving short non-repeat matches for binary-looking blocks. Thresholds 7, 8, 9, 10, and 12 were benchmarked; 10 was the best measured aggregate point on the current fixture set.
- Broadened predefined FSE table use from tiny blocks to small non-RLE sequence blocks. A broad "always predefined when possible" experiment heavily regressed JSON, so this stays intentionally narrow.
- Added repeat-offset-biased match selection. Repeat candidates may win when they are only slightly shorter than a normal match; margins 1, 2, 3, and 4 were benchmarked, and margin 2 was the best measured aggregate point.
- Added matcher-internal tests for the compact two-candidate suffix store so the `oldest`/`newest` invariant is covered directly.
- Added fastest-compressor mixed-frame regression tests that round-trip through both the Rust decoder and C zstd decoder. These cover text-like compressed blocks, binary-looking blocks, incompressible/raw blocks, and reuse of repetitive history after an incompressible block.
- Added a repeat-offset early-exit heuristic: if a repeat-offset candidate already reaches the block end, or has at least 10 bytes, the matcher skips hash-table search for that position. Thresholds 5, 10, 16, and 64 were benchmarked; 10 gave the best measured CPU/size balance while keeping every fixture smaller than C zstd.
- Added focused tests for the repeat-offset early-exit decision.
- Added sparse suffix indexing for long emitted matches, following C zstd's fast parser shape of maintaining only a few hash entries around a long match instead of indexing every byte skipped by the match. Dense limits 64, 128, and 256 were benchmarked; 128 kept the JSON size/CPU balance from the repeat early-exit work and made the repeated-text fixture effectively C-speed.
- Added focused tests that short match ranges are still indexed densely while long match ranges are indexed sparsely.
- Added a repeat-offset minimum-match precheck. Obvious repeat misses now compare the first minimum-match bytes before entering the full cross-window match-length loop; boundary-crossing cases fall back to the existing matcher path. This keeps output bytes unchanged while reducing repeat-probe CPU.
- Added focused tests for repeat-offset precheck accept/reject behavior.
- Inlined the small hot-path match extension helpers after profiling showed the repeat-offset precheck and relative-slice lookup visible as separate symbols. This keeps output bytes unchanged and reduces matcher call overhead in the release build.
- Added an exact minimum-match precheck for hash-table candidates, matching C zstd's fast-parser shape of checking candidate bytes before full match extension. Hash collisions now avoid the offset-based match-length path. This keeps output bytes unchanged and reduces decodecorpus CPU.
- Added focused tests for hash-candidate precheck accept/reject behavior.
- Added a C-fast-style no-match probe step. After a miss, the matcher indexes the next suffix and advances by two probes when doing so would not skip over an immediate repeat-offset match. This trades a small decodecorpus size loss for materially lower decodecorpus CPU and better JSON size.
- Added focused coverage that the no-match step does not skip the next repeat-offset match.
- Replaced hot repeat-offset candidate iterator chains in the matcher with fixed arrays and explicit loops. This keeps output bytes unchanged while reducing iterator overhead in repeat probing and no-match skip guards.
- Simplified the no-match skip guard so it checks repeat-offset bytes directly instead of building a temporary match context. This keeps output bytes unchanged; benchmark medians were neutral, but the hot path is simpler and avoids unnecessary context construction.
- Inlined the remaining hot candidate helpers after profiling showed `match_candidate` and `repeat_offset_can_match_at` as visible call targets. This keeps output bytes unchanged and reduces decodecorpus CPU slightly.
- Switched deterministic FSE/Huffman table-construction sorts to unstable sorting where explicit keys fully define the order. This keeps output bytes unchanged and avoids stable-sort overhead in entropy setup.
- Added a text-only wider no-match probe step. Text-like blocks now use step 3 while binary-looking blocks keep step 2; this keeps the global step-3 decodecorpus regression out while recovering its JSON size win.
- Added focused tests for text and binary no-match probe step selection.
- Added focused Huffman tests for equal-count rank-limited weight assignment and deterministic length-limited code lengths. These lock down tie behavior before any future table-construction optimization.
- Removed `unwrap`/`expect`-style invariant handling from production matcher code. The suffix index and committed-window invariants now use explicit safe `match` branches with cold panic paths, keeping the hot path clear without introducing `unsafe`.
- Removed a redundant literal-statistics pass by tracking the largest symbol count while building the histogram, and simplified Huffman encoded-length estimation to a direct loop. This keeps output bytes unchanged while reducing entropy-path bookkeeping.
- Cached the encoder Huffman table's actual maximum code length when building the table, avoiding a scan over `codes` each time table-description weights are generated. Added a focused invariant test so the cached value stays tied to the generated codes.
- Removed the temporary compressed-block buffer in the fastest path. Compressed attempts now write directly behind a 3-byte header placeholder in the caller's output buffer, then either patch the compressed header or truncate and emit raw fallback after restoring entropy/repeat state.
- Added conservative initial capacities for per-block literals and sequence buffers to avoid early growth in compressed blocks without reserving a full 128 KiB block.
- Kept repeat-offset probe candidate selection in `usize` until sequence emission. The repeat-candidate order only depends on whether the current literal length is zero, so this avoids a hot checked `u32` conversion while preserving checked conversion at the bitstream boundary. Added focused tests for zero-literal and non-zero-literal repeat-candidate ordering.
- Changed suffix-store reuse to clear only touched hash slots instead of resizing the whole slot table to `None` for each returned block. This follows C zstd's long-lived hash-table shape while keeping stale entries impossible in safe Rust. Touched slot indexes are stored as `u32`, and a 32K touched-slot threshold falls back to full sequential clearing for dense blocks to limit RSS growth. Focused tests cover stale-candidate removal, reinsertion after clear, full-clear mode, and the threshold switch.
- Added a direct repeat-history update path for the matcher. The matcher only needs to keep its repeat probes synchronized, so it now updates the three repeat offsets directly instead of calling `encode_offset_value()` and discarding the encoded value. The sequence encoder still uses `encode_offset_value()` at the bitstream boundary. Added equivalence tests against the encoded update path for literal and zero-literal matches.
- Changed negative relative window lookups to scan previous window entries newest-first, skipping the current block because non-negative relative lookups already handle current-block matches. Cross-block repeat and hash matches most often target the most recent previous block, matching the C fast parser's prefix-oriented shape. Added a focused test with two previous entries so most-recent previous-window matching stays covered.
- Cached the encoder FSE table accuracy log at table construction time and used that cached value when flushing sequence states. Added a focused invariant test for the predefined tables so `acc_log` stays tied to `table_size`.
- Changed sparse indexing after long matches to store the final sparse hash at `match_end - 2`, matching the C fast parser's post-match hash fill shape. Added focused coverage that long matches index exactly the start, start+2, and end-2 positions.
- Carried verified minimum-match prefixes into full match-length scans so accepted repeat and hash candidates do not compare the first five bytes twice. Added focused tests for the normal skipped-prefix case and the previous-window boundary fallback.
- Replaced repeated stable sorting in Huffman length-limited tree construction with a deterministic min-heap. Existing Huffman tie-behavior tests cover the ordering invariants, and a same-window direct benchmark against the previous commit confirmed exact bytes with decodecorpus CPU slightly better and JSON neutral.
- Cached resolved FSE table references once per sequence section before encoding FSE states, avoiding repeated table-mode enum matching in the per-sequence hot path. Existing compressed-block and end-to-end tests cover the emitted bitstream.
- Cached the common literal-length and match-length sequence code ranges in const lookup tables. Added an exhaustive helper-level test over the cached ranges plus the next uncached boundary, using independently stated spec ranges so table-generation mistakes are caught.
- Removed the redundant modulo from suffix hash lookup. The shifted multiplicative hash is already bounded by `len_log`, matching C zstd's mask-style hash-table indexing while staying safe for the existing non-power-of-two test capacities.
- Added a same-block forward match-length fast path. When the match source is already in the current block, the matcher now compares the current-block source and target slices directly with the existing safe chunked prefix comparison instead of repeatedly resolving relative window slices. This follows C zstd's contiguous `ZSTD_count()` shape while preserving overlap behavior.
- Preallocated a modest 1024 entries for suffix-store touched-slot tracking. This avoids first-use growth in the touched-slot clearing path while staying well below the 32K full-clear threshold and adding only a small bounded allocation per suffix store.
- Added focused matcher coverage for same-block minimum-match precheck hits and misses. This locks down repeat-offset precheck behavior even though the generic relative-window implementation remains the fastest measured runtime path.
- Replaced the suffix-candidate iterator chain in the matcher window-search loop with explicit newest/oldest candidate checks through a small helper. This preserves the existing two-candidate order, avoids iterator setup in the dominant matcher loop, and adds focused helper coverage for best-candidate updates and the offset-1 block-end early exit.
- Replaced sequence-emission repeat-offset encoding's temporary repeat-candidate array search with direct branch checks. Existing offset-history tests cover the repeat-code semantics, and the now-unused array helper was removed.
- Inlined the two safe `usize` to `u32` offset boundary helpers used during sequence emission. This preserves the public `Sequence` offset type and matcher `usize` internals while letting the optimizer fold the checked conversions into their callers.
- Hoisted the current block length into a local scalar in the matcher loop, mirroring the C fast parser's local `iend`/`ilimit` style and avoiding repeated `last_entry.data.len()` reads in the dominant path.

## Verification So Far

Latest successful commands:

- `cargo fmt --all --check`
- `cargo test -q -p ruzstd fse`
- `cargo test -q -p ruzstd encoding::blocks::compressed`
- `cargo test -q -p ruzstd encoding::match_generator`
- `cargo test -q -p ruzstd encoding::levels::fastest_tests`
- `cargo test -q -p ruzstd fastest_reuses_history_across_blocks`
- `cargo test -q -p ruzstd huff0::huff0_encoder::encoded_len`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q -p ruzstd`
- `cargo test -q --workspace`
- `cargo build --release -p ruzstd-cli`
- `/tmp/zstd_bench_current_branch.py`
- `perf record -F 999 -g -o /tmp/ruzstd-decodecorpus-after-usize-rep.perf.data -- /tmp/ruzstd-cli-huffman-maxheight compress /tmp/zstd-bench/fixtures/decodecorpus_pack.bin /tmp/ruzstd-decodecorpus-profile.zst -l 1`
- `perf report --stdio -i /tmp/ruzstd-decodecorpus-after-usize-rep.perf.data --sort=symbol --no-children`
- `perf record -F 999 -g -o /tmp/ruzstd-json-touched-u32-clear.perf.data -- /tmp/ruzstd-cli-huffman-maxheight compress /tmp/zstd-bench/fixtures/json_logs_32m.jsonl /tmp/ruzstd-json-profile.zst -l 1`
- `perf report --stdio -i /tmp/ruzstd-json-touched-u32-clear.perf.data --sort=symbol --no-children`
- `perf record -F 999 -g -o /tmp/ruzstd-json-direct-repeat-update.perf.data -- /tmp/ruzstd-cli-huffman-maxheight compress /tmp/zstd-bench/fixtures/json_logs_32m.jsonl /tmp/ruzstd-json-profile.zst -l 1`
- `perf report --stdio -i /tmp/ruzstd-json-direct-repeat-update.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /tmp/ruzstd-decodecorpus-heap-huff.perf.data -- /tmp/ruzstd-cli-huffman-maxheight compress /tmp/zstd-bench/fixtures/decodecorpus_pack.bin /tmp/ruzstd-decodecorpus-profile.zst -l 1`
- `perf report --stdio -i /tmp/ruzstd-decodecorpus-heap-huff.perf.data --sort=symbol --no-children`

## Latest Benchmark Snapshot

Script: `/tmp/zstd_bench_current_branch.py`

This script benchmarks fixtures from `/tmp/zstd-bench/fixtures` one output at a time because `/tmp` is nearly full.

Last run after the larger window, match-length fix, RLE sequence modes, incompressibility gate, raw-block no-index fast path, compact raw literals headers, overlapping match extension, chunked slice comparison, matcher-side repeat-offset probing, hash-match backward extension, exact Huffman table reuse estimates, text-aware non-repeat match threshold, small-block predefined FSE tables, repeat-offset-biased match selection, the 10-byte repeat-offset search early exit, sparse suffix indexing for matches longer than 128 bytes, repeat-offset and hash-candidate minimum-match prechecks, verified-prefix match-length scans, hot helper inlining, the repeat-aware no-match probe step, fixed repeat-candidate loops, candidate-helper inlining, deterministic unstable entropy sorts, text-only wider no-match probing, `usize` repeat-candidate selection, touched-slot suffix-store clearing, direct matcher repeat-history updates, previous-entry-only newest-first cross-window lookup, cached encoder FSE `acc_log`, C-style end-2 sparse match indexing, heap-based Huffman tree construction, cached sequence FSE table references, cached common sequence length-code tables, suffix-hash modulo removal, same-block forward match-length fast path, and modest touched-slot preallocation:

| Fixture | Upstream bytes | Current bytes | C zstd -1 bytes | Upstream CPU | Current CPU | C zstd -1 CPU |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | 5,976,095 | 5,160,978 | 5,385,951 | 0.13s | 0.20s | 0.04s |
| `json_logs_32m.jsonl` | 3,392,237 | 826,471 | 1,138,701 | 0.18s | 0.11s | 0.05s |
| `repeated_text_32m.txt` | 31,757 | 2,877 | 3,116 | 0.12s | 0.00s | 0.02s |
| `xorshift_32m.bin` | 33,555,213 | 33,555,213 | 33,555,214 | 0.59s | 0.02s | 0.06s |

Peak RSS from the same run:

| Fixture | Upstream RSS | Current RSS | C zstd -1 RSS |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | 6,392 KB | 10,676 KB | 22,312 KB |
| `json_logs_32m.jsonl` | 5,764 KB | 9,568 KB | 18,832 KB |
| `repeated_text_32m.txt` | 5,456 KB | 9,096 KB | 17,836 KB |
| `xorshift_32m.bin` | 6,188 KB | 9,328 KB | 25,504 KB |

Interpretation:

- Size improved materially on `decodecorpus_pack.bin`, `json_logs_32m.jsonl`, and `repeated_text_32m.txt`; repeated text and JSON remain smaller than C zstd on these fixtures.
- The repeat-offset search early exit, sparse long-match indexing, no-match probe step, and text-only wider probing trade about 50 KiB of decodecorpus compression and 2 bytes of repeated-text compression for a large CPU improvement, while JSON is now materially smaller than before the CPU parser shortcuts. The repeat/hash prechecks, hot helper inlining, fixed repeat-candidate loops, candidate-helper inlining, suffix-hash modulo removal, same-block forward match-length fast path, and modest touched-slot preallocation keep output sizes unchanged and improve CPU further. The measured current aggregate CPU is now about 0.33s versus about 0.90s before these CPU-focused parser shortcuts.
- Text-only wider no-match probing improved JSON size from 849,901 bytes to 826,471 bytes with only a 370-byte decodecorpus size cost, avoiding the much larger global step-3 decodecorpus regression.
- Candidate-helper inlining improved decodecorpus CPU from about 0.26s to 0.25s with no size change on the fixture set.
- Deterministic unstable entropy sorts preserved output sizes and benchmarked neutral-to-slightly-better; keep them because they remove unnecessary stable-sort work in a profiled setup path.
- Fixed repeat-candidate loops improved decodecorpus CPU from about 0.27s to 0.26s and JSON CPU from about 0.19s to 0.18s with no size change on the fixture set.
- The repeat-aware no-match probe step improved decodecorpus CPU from about 0.31s to 0.27s, improved JSON size from 950,143 bytes to 849,901 bytes, and kept all fixture sizes smaller than C zstd.
- The hash-candidate precheck improved decodecorpus CPU from about 0.33s to 0.31s with no size change on the fixture set.
- Hot helper inlining improved decodecorpus CPU from about 0.36s to 0.33s and JSON CPU from about 0.19s to 0.18s with no size change on the fixture set.
- The repeat-offset precheck improved decodecorpus CPU from about 0.42s to 0.36s and JSON CPU from about 0.21s to 0.19s with no size change on the fixture set.
- Sparse long-match indexing is the largest repeated-text CPU improvement so far, dropping that fixture from about 0.20s to about 0.02s while keeping it smaller than C zstd.
- Backward extension is a net size win across the fixture set, but it slightly worsened JSON size versus repeat-offset probing alone; keep that tradeoff visible when evaluating future match selection changes.
- The incompressible fixture is now near C zstd CPU after the no-index raw fast path.
- The larger window, overlapping extension, and repeat-offset probing improve compression but raise CPU and RSS on compressible fixtures; next work should continue reducing matcher search cost without giving back the remaining compression advantage over C zstd.
- Perf sample on `repeated_text_32m.txt` showed time dominated by `MatchGeneratorDriver::start_matching`; the repeat-offset callback is inlined into that symbol, but the larger future CPU opportunity is still matcher logic.
- Profiling after the repeat-offset bias still shows `MatchGeneratorDriver::start_matching` and `match_len_at_offset` as the dominant CPU cost. A safe hand-written `u64` prefix mismatch loop was tested, but it was slower than the existing chunk-iterator comparison and was not kept.
- Tested C zstd level-1's `minMatch = 7` setting. It improved JSON slightly but regressed decodecorpus much more, so the current global `MIN_MATCH_LEN = 5` remains the better fixture-wide choice.
- Tested rejecting short non-repeat matches below length 7 with offset cutoffs 64, 1024, 4096, and 16384. This improved JSON size by up to about 15 KiB and CPU modestly, but regressed decodecorpus by more than the JSON gain at every cutoff, so it was not kept.
- Tested repeat-offset search early-exit thresholds 5, 10, 16, and 64. Threshold 64 had no useful CPU benefit, thresholds 5/10/16 all reduced JSON CPU sharply, and threshold 10 gave the best measured aggregate CPU/size balance.
- Tested sparse long-match indexing dense limits 64, 128, and 256. Limit 64 hurt JSON size more than needed, 256 lost JSON CPU compared with 128, and 128 was the best measured aggregate balance.
- Tested widening the repeat-aware no-match probe step from 2 to 3. It improved JSON size but worsened decodecorpus size by about 52 KiB and did not improve CPU, so step 2 remains the better aggregate point.
- Tested changing the remaining Huffman stable sorts to unstable sorts with explicit tie-breakers. Output sizes were unchanged, but decodecorpus CPU repeatedly regressed from the 0.26-0.27s band to about 0.30s, so the runtime change was not kept. The added tie-behavior tests were kept.
- Removing production matcher `unwrap`/`expect` calls preserved output sizes and benchmarked neutral: decodecorpus stayed at 0.26s, JSON stayed at 0.18s, and repeated text stayed at about 0.01-0.02s.
- Tested replacing suffix `windows(MIN_MATCH_LEN)` insertion with direct index-based key extraction. Output sizes were unchanged, but JSON CPU repeatedly regressed from the 0.18-0.19s band to about 0.20-0.21s, so the change was not kept.
- Tested replacing the hot `bounded_u32` `TryFrom` path with an explicit checked branch plus cold panic path. Output sizes were unchanged, but decodecorpus and JSON CPU drifted worse across two runs, so the change was not kept.
- Tested C-fast-style immediate zero-literal repeat early exits. Threshold 5 slightly improved decodecorpus size but regressed JSON size, threshold 6 regressed JSON and did not beat the baseline on decodecorpus, and threshold 7 was worse than both endpoints. The family was not kept because it trades away text compression without a reliable CPU win.
- Entropy-path bookkeeping cleanup preserved exact fixture byte counts. Benchmarks were neutral within noise: decodecorpus measured 0.26s then 0.28s, JSON stayed at 0.19s, and repeated text stayed at 0.01s.
- Tested wider safe match-extension chunks as a SIMD-adjacent experiment. Chunk widths 16 and 32 preserved exact output bytes, but neither improved decodecorpus CPU over the existing 8-byte chunk shape, so the existing safe chunk width remains in place.
- Tested combining the repeated FSE table-selection scans into one pass. Output bytes were unchanged, but JSON CPU repeatedly regressed to about 0.20s, so the original separate scans were kept.
- Caching the Huffman max code length preserved exact fixture byte counts and benchmarked neutral-to-slightly-positive within noise: decodecorpus measured 0.26s then 0.27s in clean runs, JSON measured 0.19s then 0.18s.
- Direct compressed-block output preserved exact fixture byte counts and removed one allocation/copy from accepted compressed blocks. Clean benchmark passes measured decodecorpus at 0.26s and JSON at 0.18s.
- Conservative literals/sequences preallocation preserved exact fixture byte counts and improved decodecorpus CPU to 0.25s across two clean runs while keeping JSON at 0.18s. RSS stayed within the existing measurement band.
- Tested moving per-block literal and sequence scratch buffers into `CompressState` for reuse across compressed blocks. Output bytes stayed unchanged and RSS was slightly lower, but decodecorpus CPU repeatedly measured about 0.27s instead of the 0.25s seen with simple per-block preallocation, so the added state complexity was not kept.
- Keeping repeat-offset probe candidate selection in `usize` preserved exact fixture byte counts, reduced decodecorpus CPU to 0.25s on the table run, and moved `bounded_u32` from a visible multi-percent matcher cost to a small residual emission-side cost in the follow-up profile.
- Touched-slot suffix-store clearing preserved exact fixture byte counts. It reduced JSON CPU from the 0.18-0.19s band to 0.17s, reduced xorshift CPU to 0.02s, and moved JSON `commit_space` from about 7% to about 1.1% in perf. A pure touched-slot vector raised decodecorpus RSS to about 11.4 MB, so the retained implementation switches to full sequential clearing after 32K touched slots; that kept the JSON CPU win while lowering decodecorpus RSS to about 10.7 MB. A 64K threshold was tested and rejected after it regressed decodecorpus and JSON CPU on the table run.
- Direct matcher repeat-history updates preserved exact fixture byte counts, kept JSON at 0.17s, and moved decodecorpus back to 0.25s on the table run. The follow-up JSON profile removed the previously visible matcher-side `OffsetHistory::encode_offset_value` cost, leaving only small residual repeat-history samples around sequence emission.
- Newest-first cross-window lookup preserved exact fixture byte counts. The initial newest-first change was neutral-to-positive; additionally skipping the impossible current-block check for negative relative lookups improved decodecorpus to 0.24s across two runs, while JSON drifted to 0.18s. The aggregate CPU stayed slightly better, so the C-shaped lookup order was kept with focused coverage for most-recent previous-window matches.
- Tested replacing the no-match skip guard's small range iterator with explicit branches for probe steps 2 and 3. Output bytes were unchanged, but decodecorpus drifted to 0.26s and JSON to 0.18s on the table run, so the original iterator-shaped guard was kept.
- Tested replacing FSE `SymbolStates::get()`'s iterator search with an explicit indexed loop and cold panic path. Output bytes were unchanged, but decodecorpus drifted to 0.26s and JSON to 0.18s on the table run, so the original iterator/`unwrap` form was kept for this hot lookup.
- Tested splitting the common literal-length and match-length code ranges into explicit early-return branches before the larger range matches. Output bytes were unchanged, but decodecorpus drifted to 0.26s and JSON to 0.18s on the table run, so the original single-match form was kept.
- Tested scanning hash-candidate window entries newest-first. Output bytes were unchanged and one run improved JSON to 0.16s, but the repeat run returned JSON to 0.18s while decodecorpus stayed at 0.25s, so the change was treated as noise and not kept.
- Caching the encoder FSE `acc_log` preserved exact fixture byte counts. The table run measured decodecorpus unchanged at 0.24s and JSON at 0.16s; treat the JSON improvement cautiously as run-to-run noise, but keep the change because it removes repeated `ilog2` work and has focused invariant coverage.
- Moving the sparse long-match tail hash from `match_end - MIN_MATCH_LEN` to `match_end - 2`, matching the C fast parser, improved decodecorpus by 65 bytes with no size change on the other fixtures. Two benchmark runs kept decodecorpus at 0.24s and JSON in the 0.16-0.17s noise band, so the C-shaped indexing position was kept.
- Carrying a verified minimum-match prefix into full match-length scans preserved exact fixture byte counts. The benchmark stayed neutral in the current noise band, but the change avoids rechecking the first five bytes for accepted candidates while preserving the full scan for previous-window boundary cases.
- Tested probing only the first two repeat-offset candidates in matcher search, closer to C fast's active repeat-offset checks. It regressed decodecorpus from 5,160,978 bytes to 5,166,985 bytes and JSON from 826,471 bytes to 854,443 bytes with no measurable CPU win, so the three-candidate matcher probe was kept.
- Replacing repeated stable sorts in Huffman length-limited tree construction with a deterministic min-heap preserved exact fixture byte counts. The full table runs were noisy, but a direct five-run comparison against the previous commit showed decodecorpus median CPU improving from about 0.24-0.25s to 0.23-0.24s and JSON staying neutral. The follow-up perf sample removed the previously visible `core::slice::sort::stable::drift::sort` symbol, leaving Huffman table construction around 1.2% of decodecorpus samples.
- Tested a same-block fast path for backward match extension using contiguous prefix slices instead of the existing byte walk through `slice_at_relative()`. Output bytes were unchanged, but the table run stayed neutral-to-worse and the follow-up perf sample still showed `extend_match_backwards` around 2.5-3%, so the simpler byte walk was kept.
- Caching sequence FSE table references preserved exact fixture byte counts and reduced repeated enum matching in `encode_sequences`. Two table runs measured decodecorpus at 0.23s, with JSON neutral at 0.17s; keep this as a small sequence-encoder CPU improvement.
- Tested replacing the safe chunk-iterator common-prefix comparison with a direct indexed chunk loop. Output bytes were unchanged, but decodecorpus regressed from the 0.23s band to 0.24-0.26s across two runs, so the iterator-shaped chunk comparison remains the better safe-Rust implementation.
- Tested guarding sequence additional-bit writes so zero-width writes skip `BitWriter::write_bits()`. Output bytes were unchanged, but table runs were mixed: decodecorpus measured 0.23s then 0.25s while JSON measured 0.18s then 0.16s. The unstable tradeoff was not enough to justify the extra branch, so the original direct writes were kept.
- Caching common literal-length and match-length sequence code ranges preserved exact fixture byte counts. Three table runs kept decodecorpus at 0.23s, with JSON in the 0.16-0.18s noise band; retain it as a small safe sequence-code cleanup distinct from the rejected explicit branch-split experiment.
- Removing the redundant suffix-hash modulo preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.22s then 0.20s and JSON at 0.12s both times; retain it because it removes hot-path integer division-style work and matches C zstd's bounded hash-table indexing shape.
- Same-block forward match-length fast path preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.21s then 0.20s and JSON at 0.11s then 0.12s; retain it because it removes repeated relative-window resolution for the common current-block case while keeping the existing safe chunked comparison.
- Modest touched-slot preallocation preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.21s then 0.20s, JSON at 0.11s both times, and RSS stayed in the existing band; retain it because it removes first-use growth in the touched-slot clear path with a small bounded allocation.
- Tested caching each sequence's derived literal-length, match-length, and offset FSE symbols/add-bits in a local `EncodedSequence` representation. Output bytes were unchanged and focused tests passed, but decodecorpus CPU regressed to 0.22s across two table runs and RSS rose to about 12.1 MB, so the runtime change was not kept.
- Tested a previous-window first-segment fast path for match-length scans, mirroring C zstd's two-segment count shape. Output bytes were unchanged and focused tests passed, but decodecorpus regressed to 0.21s then 0.22s while JSON stayed neutral, so the simpler generic relative-window loop remains better.
- Tested increasing the per-block sequence vector initial capacity from 256 to 512 after profiles showed a small `Vec::grow_one` sample on JSON. Output bytes were unchanged, but decodecorpus measured 0.23s then 0.20s and JSON worsened to 0.12s then 0.13s, so the original conservative capacity remains better.
- Tested replacing suffix-store `TryFrom`/checked-add index packing with an explicit upper-bound branch and cold panic path. Output bytes were unchanged and focused tests passed, but decodecorpus stayed at 0.22s across two runs with no clear aggregate CPU win, so the original checked conversion remains.
- Tested reusing the already-computed current suffix hash when inserting the current suffix after a no-match probe miss. Output bytes were unchanged and focused tests passed, but JSON regressed to 0.13s across two runs while decodecorpus stayed in its normal 0.20s-0.21s band, so the simpler insertion path remains better.
- Tested replacing FSE probability-normalization `unwrap()` calls with explicit helper loops and cold invariant panics. After matching iterator tie behavior, output bytes were unchanged, but decodecorpus measured 0.22s then 0.21s and JSON stayed at 0.12s, so the original iterator form remains better for this setup path.
- Tested C-fast-style sparse miss indexing that only stores the current suffix when the no-match probe skips ahead. CPU improved slightly on decodecorpus, but size regressed badly: decodecorpus grew to 5,387,388 bytes, JSON to 862,050 bytes, and repeated text to 2,965 bytes. The retained parser continues indexing skipped miss positions to preserve compression.
- Tested replacing Huffman `build_from_weights()`'s temporary sorted `Vec` with a fixed 256-entry stack array sorted over the filled prefix. Output bytes were unchanged and Huffman tests passed, but decodecorpus measured 0.23s then 0.21s with no clear win, so the existing temporary `Vec` remains better.
- Tested adding same-block direct paths to the repeat-offset minimum-match prechecks, mirroring the retained same-block full match-length fast path. Output bytes were unchanged and focused tests passed, but decodecorpus regressed to 0.22s then 0.21s and JSON regressed to 0.12s across both runs, so the generic relative-window precheck remains better.
- Explicit suffix-candidate checks preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.19s then 0.20s and JSON at 0.11s both times, so retain the helper as a small safe hot-loop cleanup.
- Direct repeat-offset encoding branches preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.20s both times and JSON at 0.11s both times; a follow-up JSON profile no longer showed `OffsetHistory::encode_offset_value` as a separate symbol, so keep the smaller direct branch shape.
- Tested a small cached offset-code table, mirroring the retained literal/match length code caches. Output bytes were unchanged and focused spec-range tests passed, but decodecorpus regressed to 0.21s then 0.22s, so the direct `ilog2` offset-code calculation remains better.
- Inlining boundary offset conversions preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.20s then 0.21s and JSON at 0.11s both times; a follow-up JSON profile no longer showed `offset_to_u32` or matcher `bounded_u32` as separate symbols, so keep this as a small neutral cleanup.
- Tested increasing the per-block literals vector initial capacity from 1024 to 2048 after profiles showed occasional allocation growth in compressed block assembly. Output bytes were unchanged, but decodecorpus regressed to 0.22s then 0.21s and JSON regressed to 0.12s both runs, so the existing conservative literal capacity remains better.
- Hoisting the matcher block length preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.21s then 0.20s and JSON at 0.12s then 0.11s; keep it as a small neutral hot-loop cleanup that reduces repeated length reads without changing search behavior.
- Tested forcing `encode_offset()` inline after JSON profiles showed it as a small sequence-side symbol. Output bytes were unchanged, but decodecorpus measured 0.23s on the first run and only returned to 0.20s on the repeat, so the optimizer's original inlining decision remains better.
- Tested replacing matcher prefix comparison's `chunks_exact(N)` shape with stable fixed-array `as_chunks::<N>()` as a safe SIMD-adjacent experiment. Output bytes were unchanged, but decodecorpus measured 0.22s then 0.20s and JSON measured 0.11s then 0.12s, so the existing `chunks_exact` shape remains better.
- Tested a C-fast-style end-of-block search cleanup that stops probing the last few bytes of large blocks. It improved decodecorpus size by 833 bytes but regressed JSON size by 4,048 bytes and did not improve CPU, so the retained matcher still searches down to the minimum match-length tail.

## Next Steps

1. Profile matcher search and extension paths again after the repeat-offset precheck; `match_len_at_offset` should still be a main target, but sequence encoding now shows up more clearly on JSON.
2. Investigate further safe early-exit or candidate-pruning heuristics in match selection; keep compression-ratio guardrails in tests and benchmarks.
3. Keep adding focused helper-level tests plus emitted-bitstream/Rust-decoder/C-decoder interoperability tests for each compression change; excellent coverage is a hard acceptance criterion for retained work.
4. SIMD remains a matcher byte-comparison topic, but current stable safe-Rust options have not beaten the retained chunked comparison; avoid unsafe/nightly SIMD unless the project explicitly accepts that tradeoff.

## Coverage Audit

- Current branch has focused unit tests for matcher suffix candidates, repeat-offset candidate ordering, repeat-history updates, prechecks, sparse indexing, no-match step selection, FSE table selection, FSE `acc_log` caching, and Huffman length/weight invariants.
- Current branch has focused coverage that suffix hash keys stay inside the slot table without a final modulo, including a non-power-of-two capacity.
- Current branch has focused coverage that touched-slot preallocation stays modest and below the full-clear threshold.
- Current branch has focused coverage that same-block match-length scanning with a verified prefix still handles overlapping matches by comparing against the generic relative-window scanner.
- Current branch has focused coverage for the explicit suffix-candidate helper, including best-candidate replacement and the offset-1 block-end early exit.
- Current branch has exhaustive helper-level coverage for the cached common literal-length and match-length sequence code tables, including the first uncached boundary for each table.
- Current branch has helper-level coverage for offset-code generation across the small repeat-code range plus the first uncached boundary from the rejected offset-code cache experiment.
- Current branch has emitted-bitstream tests that round-trip fastest compression through the Rust decoder and the C zstd decoder, including mixed text/binary/random frames, history reuse after incompressible blocks, and cross-block repetitive data.
- Current branch still has the existing encode/decode corpus tests and fuzz targets for encode/decode/FSE/Huff0 interop. These are not a replacement for focused regression tests, but they are useful broad coverage.
- Acceptance rule for future retained changes: add a focused unit/regression test for the changed invariant or an end-to-end Rust+C decode test for emitted-bitstream behavior. If a change is purely a benchmark-only micro-optimization, document why in this file.
