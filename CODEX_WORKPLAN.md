# Codex Workplan

## Current Goal

Branch: `compression/huffman-maxheight`

Implement and verify the two follow-up compression improvements after merging the two-candidate matcher:

1. Encoder-side Zstd repeat-offset history for sequence offsets.
2. Conservative sequence FSE table selection, using the C zstd implementation as the guide.

Keep the branch correct against the local Rust decoder and C zstd decoder, then benchmark against upstream and C zstd.

Quality constraints:

- Keep the Rust implementation high quality, well structured, and idiomatic.
- Avoid `unsafe` code. Treat safe Rust as a goal constraint, not just a preference.
- Prefer clear state machines and small helpers over clever code that is harder to verify.
- Maintain excellent test coverage for each compression feature.
- Cover private invariants with focused unit tests, especially compact matcher state such as suffix candidates and repeat-offset history.
- Cover emitted bitstreams with end-to-end tests through the Rust decoder and the C zstd decoder; helper-level tests alone are not enough.
- Every new compression heuristic should get either a regression test for the intended behavior or an explicit workplan note explaining why benchmark-only coverage is appropriate.

## C zstd Guidance Used

- Repeat offsets: mirror the decoder/spec rules and the C compressor's repeat-code choice/update behavior.
- FSE table modes: follow the conservative fast-path idea from C zstd: use predefined tables for tiny sequence counts, repeat previous tables only when the symbols are valid, and avoid broad heuristics without cost modeling.
- SIMD/hardware-vector work should target the matcher later, especially match extension/comparison. The repeat-offset and table-selection paths are scalar/control-heavy and are not good SIMD candidates.

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

## Verification So Far

Latest successful commands:

- `cargo fmt --all --check`
- `cargo test -q -p ruzstd encoding::blocks::compressed`
- `cargo test -q -p ruzstd encoding::match_generator`
- `cargo test -q -p ruzstd encoding::levels::fastest_tests`
- `cargo test -q -p ruzstd fastest_reuses_history_across_blocks`
- `cargo test -q -p ruzstd huff0::huff0_encoder::encoded_len`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q -p ruzstd`
- `cargo build --release -p ruzstd-cli`
- `/tmp/zstd_bench_current_branch.py`

## Latest Benchmark Snapshot

Script: `/tmp/zstd_bench_current_branch.py`

This script benchmarks fixtures from `/tmp/zstd-bench/fixtures` one output at a time because `/tmp` is nearly full.

Last run after the larger window, match-length fix, RLE sequence modes, incompressibility gate, raw-block no-index fast path, compact raw literals headers, overlapping match extension, chunked slice comparison, matcher-side repeat-offset probing, hash-match backward extension, exact Huffman table reuse estimates, text-aware non-repeat match threshold, small-block predefined FSE tables, repeat-offset-biased match selection, the 10-byte repeat-offset search early exit, sparse suffix indexing for matches longer than 128 bytes, repeat-offset and hash-candidate minimum-match prechecks, hot helper inlining, the repeat-aware no-match probe step, and fixed repeat-candidate loops:

| Fixture | Upstream bytes | Current bytes | C zstd -1 bytes | Upstream CPU | Current CPU | C zstd -1 CPU |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | 5,976,095 | 5,160,673 | 5,385,951 | 0.13s | 0.26s | 0.05s |
| `json_logs_32m.jsonl` | 3,392,237 | 849,901 | 1,138,701 | 0.17s | 0.18s | 0.05s |
| `repeated_text_32m.txt` | 31,757 | 2,877 | 3,116 | 0.11s | 0.02s | 0.02s |
| `xorshift_32m.bin` | 33,555,213 | 33,555,213 | 33,555,214 | 0.60s | 0.03s | 0.05s |

Interpretation:

- Size improved materially on `decodecorpus_pack.bin`, `json_logs_32m.jsonl`, and `repeated_text_32m.txt`; repeated text and JSON remain smaller than C zstd on these fixtures.
- The repeat-offset search early exit, sparse long-match indexing, and no-match probe step trade about 50 KiB of decodecorpus compression and 2 bytes of repeated-text compression for a large CPU improvement, while JSON is now materially smaller than before the CPU parser shortcuts. The repeat/hash prechecks, hot helper inlining, and fixed repeat-candidate loops keep output sizes unchanged and improve CPU further. The measured current aggregate CPU is now about 0.48s versus about 0.90s before these CPU-focused parser shortcuts.
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

## Next Steps

1. Profile matcher search and extension paths again after the repeat-offset precheck; `match_len_at_offset` should still be a main target, but sequence encoding now shows up more clearly on JSON.
2. Investigate further safe early-exit or candidate-pruning heuristics in match selection; keep compression-ratio guardrails in tests and benchmarks.
3. Keep adding focused helper-level tests plus emitted-bitstream/Rust-decoder/C-decoder interoperability tests for each compression change.
4. Do not start SIMD work in the repeat-offset or FSE selection paths; the useful SIMD target is matcher byte comparison/match extension.
