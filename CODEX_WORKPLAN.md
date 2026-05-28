# Codex Workplan

## Current Goal

Branch: `compression/huffman-maxheight`

Implement and verify the two follow-up compression improvements after merging the two-candidate matcher:

1. Encoder-side Zstd repeat-offset history for sequence offsets.
2. Conservative sequence FSE table selection, using the C zstd implementation as the guide.

Keep the branch correct against the local Rust decoder and C zstd decoder, then benchmark against upstream and C zstd. Treat excellent test coverage as a first-class part of the goal: a performance or compression win is not complete unless its correctness invariants are covered, emitted bitstreams are round-tripped where relevant, or the workplan explicitly justifies why benchmark-only coverage is appropriate.

Quality constraints:

- Keep the Rust implementation high quality, well structured, and idiomatic.
- Avoid `unsafe` code. Treat safe Rust as a goal constraint, not just a preference.
- Prefer clear state machines and small helpers over clever code that is harder to verify.
- Prefer explicit typed state over manual bit packing when the measured cost is acceptable. For matcher suffix candidates, the chosen direction is a small struct with two `Option<NonZeroU32>` values rather than packing two indexes into one `NonZeroU64`.
- Keep `usize` for Rust slice/window positions while searching, because slice indexing and lengths are naturally `usize`. Convert to `u32`/`NonZeroU32` only at bounded storage or bitstream boundaries, with checked conversions and cold invariant panics where the bound is guaranteed by the compressor window.
- Avoid `unwrap()`/`expect()` in production matcher code. Use explicit `match` branches with clear invariant messages on cold panic paths; do not replace these with `unsafe`.
- Maintain excellent test coverage for each compression feature; retained changes should have tests that make the behavior hard to regress.
- Cover private invariants with focused unit tests, especially compact matcher state such as suffix candidates and repeat-offset history.
- Cover emitted bitstreams with end-to-end tests through the Rust decoder and the C zstd decoder; helper-level tests alone are not enough.
- Every new compression heuristic should get either a regression test for the intended behavior or an explicit workplan note explaining why benchmark-only coverage is appropriate.

Test coverage bar:

- Correctness fixes require a failing or gap-focused regression test.
- Parser/matcher heuristics require private invariant tests plus at least one emitted-stream round trip when the emitted sequence stream can change.
- Entropy-mode or bitstream changes require Rust decoder and C zstd decoder coverage.
- Performance-only refactors may use benchmark-only coverage only when output bytes are proven unchanged and the workplan records that rationale.

## C zstd Guidance Used

- Repeat offsets: mirror the decoder/spec rules and the C compressor's repeat-code choice/update behavior.
- FSE table modes: follow the conservative fast-path idea from C zstd: use predefined tables for tiny sequence counts, repeat previous tables only when the symbols are valid, and avoid broad heuristics without cost modeling.
- Literal compression: follow C zstd's fast-level guardrails for small/repeated literals, single-stream Huffman below 256 bytes, and the `(srcSize >> 6) + 2` minimum literal gain before accepting a Huffman literal section.
- Huffman work remains C-guided but not C-cloned: compare table-depth choices, literal mode selection, repeat-table reuse, and payload-cost guards against C zstd, then keep the Rust shape idiomatic and covered.
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
- Replaced the suffix-candidate iterator chain in the matcher window-search loop with explicit newest/oldest candidate checks through a small helper. This preserves the existing two-candidate order, avoids iterator setup in the dominant matcher loop, and adds focused helper coverage for best-candidate updates and block-end early exits.
- Replaced sequence-emission repeat-offset encoding's temporary repeat-candidate array search with direct branch checks. Existing offset-history tests cover the repeat-code semantics, and the now-unused array helper was removed.
- Inlined the two safe `usize` to `u32` offset boundary helpers used during sequence emission. This preserves the public `Sequence` offset type and matcher `usize` internals while letting the optimizer fold the checked conversions into their callers.
- Hoisted the current block length into a local scalar in the matcher loop, mirroring the C fast parser's local `iend`/`ilimit` style and avoiding repeated `last_entry.data.len()` reads in the dominant path.
- Removed production `unwrap()` calls from Huffman length-limited tree construction and rank-limited weight distribution. These invariants now use explicit branches with a cold panic helper, while tests cover deterministic tied-count behavior plus representative bounded/prefix-free Huffman tables.
- Added encoder emission for RLE literal sections when the literal payload for a compressed block is one repeated byte. This matches an existing decoder-supported Zstd literal section mode and adds both literal-section decoder coverage and a full frame round-trip through the Rust and C zstd decoders.
- Lowered the fastest encoder's literal-compression threshold from more than 1024 literals to more than 63 literals, matching C zstd's `COMPRESS_LITERALS_SIZE_MIN` heuristic. Added a focused small-literal compressed-block test that verifies Huffman literal emission and round-trips the full frame through both the Rust and C zstd decoders.
- Lowered the literal-compression threshold to more than 6 literals when a previous Huffman table is available, matching C zstd's repeat-table `minLitSize` heuristic. Added a full-frame Rust/C decoder round-trip showing small repeated literal payloads can use RLE once the previous table makes the smaller threshold valid.
- Added an early raw-literals fallback when the exact Huffman encoded-size estimate plus literal-section header cannot beat raw literals. This mirrors C zstd's no-gain entropy decision and avoids doing the actual Huffman encode only to rewind. Added a full-frame Rust/C decoder round-trip for a high-alphabet payload that passes the histogram gate but stays raw by estimate.
- Fixed frame-header descriptor emission for 8-byte frame content sizes. The encoder now writes FCS flag 3 when `find_min_size()` returns 8, and focused coverage verifies that a large single-segment frame content size round-trips through the local frame-header parser.
- Added one-byte read lookahead in frame compression so exact block-sized inputs mark the full block as final instead of emitting an extra empty raw block. Focused tests cover both the no-empty-final-block case and preserving the lookahead byte as the first byte of the next block.
- Added a hot BitWriter path for writes that exactly fill the 64-bit staging buffer, avoiding the cold overflow helper in that common boundary case. Added focused bit-level coverage that exact 64-bit fills flush correctly and preserve following writes.
- Split suffix hashing into a precomputed five-byte key value plus per-store slot mapping. Window search now reads the current suffix bytes once and reuses that value across all window entries, matching C zstd's local-hash shape while preserving each store's own table size. Added focused coverage that precomputed key lookup matches normal suffix lookup.
- Changed sequence bitstream encoding's reverse walk from a reversed range iterator to an explicit countdown loop, matching C zstd's indexed reverse-loop shape. Existing compressed-block and end-to-end Rust/C decoder tests cover the emitted bitstream.
- Tightened focused coverage after the coverage audit: suffix-store zero index candidates now round-trip through the one-based `NonZeroU32` representation, empty committed matcher entries emit no sequences and can be followed by normal data, and mixed predefined sequence modes round-trip through the decoder.
- Hardened `compress_fastest()` for empty direct block inputs before the RLE probe indexes byte 0. The normal frame compressor already handles empty frames before block compression, so this is a focused correctness/idiomatic-safety guard rather than a fixture benchmark change. Added focused coverage that an empty fastest block emits a valid raw block without panicking.
- Inlined the common literal-length and match-length code helpers after refreshed JSON profiles showed both as visible sequence-side symbols. Existing exhaustive helper-level spec tests cover these tables and the first uncached boundary, and a follow-up profile confirmed the helpers are folded into `encode_sequences`.
- Added a specialized matcher path for whole-block RLE history. Instead of indexing every suffix in a constant block, the default matcher stores only the first and last useful suffix for the single repeated key, preserving future matches while avoiding duplicate suffix insertion work. Custom matchers keep the existing default behavior through a new provided trait method.
- Hardened matcher suffix-store sizing for short-frame reuse: fresh driver stores are sized from the configured slice size, and the private store constructor has a minimum backing size. Added focused tests for zero-capacity store construction and reusing a one-byte frame's store for a later larger frame, plus an end-to-end reused-compressor Fastest test that compresses a one-byte frame followed by a larger compressible frame and round-trips both through the Rust and C zstd decoders.
- Added C-style repeat-offset availability pruning before repeat probes enter relative-window lookup. Repeat offsets that point before the retained window are now skipped explicitly, matching C fast's invalid-repcode guard while keeping the safe relative lookup for valid boundary-crossing matches.
- Reduced the default suffix hash table to the C zstd level-1 fast-parser scale: a 128 KiB block now uses 8 Ki hash slots. This gives back some retained compression headroom but keeps every PR fixture smaller than C zstd while materially reducing CPU and RSS. Added focused coverage for the driver sizing invariant and for keeping `Option<Candidates>` compact with the two-`NonZeroU32` representation.
- Changed hash-candidate window search to scan newest entries first and to stop once a candidate reaches the block end. This mirrors C fast's most-recent hash-table behavior and avoids continuing after the maximum possible match length is found. Added focused helper coverage for non-offset-1 block-end early exit and full matcher coverage that the newest previous block-end candidate wins.
- Switched Huffman-compressed literal payloads below 256 bytes to single-stream encoding, matching C zstd's `singleStream = srcSize < 256` selection while keeping 4-stream encoding for larger payloads. Added an emitted-bitstream test that verifies the single-stream literal header and round-trips the frame through both the Rust and C zstd decoders.
- Added C-style minimum-gain rejection for Huffman literal sections. Fast-level literals now require a compressed payload gain of `(srcSize >> 6) + 2` before emitting Huffman, preserving all PR fixtures smaller than C zstd while avoiding narrow literal wins that cost CPU. Added focused coverage for the exact boundary plus Rust/C decoder round-trip coverage for the emitted raw fallback.
- Disabled CLI progress-bar updates for non-terminal stderr. This keeps interactive progress behavior but removes `indicatif` update/tick overhead from redirected benchmark runs, making the CLI comparison with quiet C zstd cleaner. Added focused CLI coverage that the hidden progress monitor still reads through all input bytes.
- Added C-style single-stream literal encoding when reusing a previous Huffman table for sub-1 KiB literal payloads, but only when the exact estimated size is no larger than a newly generated table. This preserves decodecorpus size, improves JSON by 7 bytes on the PR fixture table, and avoids blindly preferring a worse previous table. Added Huffman-table symbol coverage plus a two-block emitted-bitstream test that round-trips the treeless single-stream repeat-table path through both the Rust and C zstd decoders.
- Removed the redundant previous-Huffman-table compatibility scan against the newly generated Huffman table. The literal-count compatibility check already proves the previous table can encode every symbol in the payload, so the repeat-table decision now avoids a second equivalent symbol scan while preserving exact fixture bytes.
- Added an all-RLE sequence encoding fast path. When literal-length, match-length, and offset sequence modes are all already RLE, the encoder now writes only the per-sequence additional bits and final padding instead of passing through the generic optional FSE-state update path. This does not change table selection or emitted bytes. Added decoder round-trip coverage with varying additional bits under identical RLE symbols.
- Added a conservative compressed-block output reservation in the fastest path before writing directly into the frame output buffer. This preserves exact emitted bytes and reduces output-buffer growth churn after the temporary compressed-block buffer was removed.

## Verification So Far

Latest successful commands:

- `cargo fmt --all --check`
- `cargo test -q -p ruzstd bit_writer`
- `cargo test -q -p ruzstd fse`
- `cargo test -q -p ruzstd encoding::blocks::compressed`
- `cargo test -q -p ruzstd all_rle_sequence_modes_preserve_additional_bits`
- `cargo test -q -p ruzstd encoding::blocks::compressed::tests::literal_min_gain_boundary_uses_raw_literals_and_round_trips`
- `cargo test -q -p ruzstd encoding::blocks::compressed::tests::small_huffman_literals_use_single_stream_and_round_trip`
- `cargo test -q -p ruzstd encoding::blocks::compressed::tests::small_literals_prefer_previous_huffman_table_and_single_stream`
- `cargo test -q -p ruzstd encoding::match_generator`
- `cargo test -q -p ruzstd encoding::frame_compressor::tests::fastest_reused_compressor_handles_tiny_then_compressible_frame`
- `cargo test -q -p ruzstd encoding::levels::fastest_tests`
- `cargo test -q -p ruzstd fastest_reuses_history_across_blocks`
- `cargo test -q -p ruzstd can_encode_counts_checks_symbols_without_building_table`
- `cargo test -q -p ruzstd huff0::huff0_encoder::encoded_len`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q -p ruzstd`
- `cargo test -q --workspace`
- `cargo build --release -p ruzstd-cli`
- `cargo test -q -p ruzstd-cli progress`
- `cargo clippy -q -p ruzstd-cli -- -D warnings`
- `python3 /tmp/zstd_bench_current_branch.py`
- `python3 tools/benchmark_zstd.py --csv-output /tmp/zstd-rs-benchmark-no-progress.csv --md-output /tmp/zstd-rs-benchmark-no-progress.md`
- `python3 tools/benchmark_zstd.py --csv-output /tmp/zstd-rs-benchmark-repeat-huff-single-when-smaller.csv --md-output /tmp/zstd-rs-benchmark-repeat-huff-single-when-smaller.md`
- `python3 tools/benchmark_zstd.py --csv-output /tmp/zstd-rs-benchmark-repeat-huff-count-check.csv --md-output /tmp/zstd-rs-benchmark-repeat-huff-count-check.md`
- `python3 tools/benchmark_zstd.py --csv-output /tmp/zstd-rs-benchmark-rle-sequence-fast-path.csv --md-output /tmp/zstd-rs-benchmark-rle-sequence-fast-path.md`
- `python3 tools/benchmark_zstd.py --csv-output /tmp/zstd-rs-benchmark-output-reserve.csv --md-output /tmp/zstd-rs-benchmark-output-reserve.md`
- `perf record -F 999 -g -o /tmp/ruzstd-decodecorpus-after-usize-rep.perf.data -- /tmp/ruzstd-cli-huffman-maxheight compress /tmp/zstd-bench/fixtures/decodecorpus_pack.bin /tmp/ruzstd-decodecorpus-profile.zst -l 1`
- `perf report --stdio -i /tmp/ruzstd-decodecorpus-after-usize-rep.perf.data --sort=symbol --no-children`
- `perf record -F 999 -g -o /tmp/ruzstd-json-touched-u32-clear.perf.data -- /tmp/ruzstd-cli-huffman-maxheight compress /tmp/zstd-bench/fixtures/json_logs_32m.jsonl /tmp/ruzstd-json-profile.zst -l 1`
- `perf report --stdio -i /tmp/ruzstd-json-touched-u32-clear.perf.data --sort=symbol --no-children`
- `perf record -F 999 -g -o /tmp/ruzstd-json-direct-repeat-update.perf.data -- /tmp/ruzstd-cli-huffman-maxheight compress /tmp/zstd-bench/fixtures/json_logs_32m.jsonl /tmp/ruzstd-json-profile.zst -l 1`
- `perf report --stdio -i /tmp/ruzstd-json-direct-repeat-update.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /tmp/ruzstd-decodecorpus-heap-huff.perf.data -- /tmp/ruzstd-cli-huffman-maxheight compress /tmp/zstd-bench/fixtures/decodecorpus_pack.bin /tmp/ruzstd-decodecorpus-profile.zst -l 1`
- `perf report --stdio -i /tmp/ruzstd-decodecorpus-heap-huff.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /tmp/ruzstd-decodecorpus-after-bitwriter.perf.data -- /tmp/ruzstd-cli-huffman-maxheight compress /tmp/zstd-bench/fixtures/decodecorpus_pack.bin /tmp/ruzstd-decodecorpus-profile.zst -l 1`
- `perf report --stdio -i /tmp/ruzstd-decodecorpus-after-bitwriter.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /tmp/ruzstd-json-after-bitwriter.perf.data -- /tmp/ruzstd-cli-huffman-maxheight compress /tmp/zstd-bench/fixtures/json_logs_32m.jsonl /tmp/ruzstd-json-profile.zst -l 1`
- `perf report --stdio -i /tmp/ruzstd-json-after-bitwriter.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /tmp/ruzstd-decodecorpus-after-keyvalue.perf.data -- /tmp/ruzstd-cli-huffman-maxheight compress /tmp/zstd-bench/fixtures/decodecorpus_pack.bin /tmp/ruzstd-decodecorpus-profile.zst -l 1`
- `perf report --stdio -i /tmp/ruzstd-decodecorpus-after-keyvalue.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /tmp/ruzstd-json-after-keyvalue.perf.data -- /tmp/ruzstd-cli-huffman-maxheight compress /tmp/zstd-bench/fixtures/json_logs_32m.jsonl /tmp/ruzstd-json-profile.zst -l 1`
- `perf report --stdio -i /tmp/ruzstd-json-after-keyvalue.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /tmp/ruzstd-decodecorpus-current.perf.data -- /tmp/ruzstd-cli-huffman-maxheight compress /tmp/zstd-bench/fixtures/decodecorpus_pack.bin /tmp/ruzstd-decodecorpus-profile.zst -l 1`
- `perf report --stdio -i /tmp/ruzstd-decodecorpus-current.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /tmp/ruzstd-json-current.perf.data -- /tmp/ruzstd-cli-huffman-maxheight compress /tmp/zstd-bench/fixtures/json_logs_32m.jsonl /tmp/ruzstd-json-profile.zst -l 1`
- `perf report --stdio -i /tmp/ruzstd-json-current.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /tmp/ruzstd-decodecorpus-f9735b7.perf.data -- /tmp/ruzstd-cli-huffman-maxheight compress /tmp/zstd-bench/fixtures/decodecorpus_pack.bin /tmp/ruzstd-decodecorpus-profile.zst -l 1`
- `perf report --stdio -i /tmp/ruzstd-decodecorpus-f9735b7.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /tmp/ruzstd-json-f9735b7.perf.data -- /tmp/ruzstd-cli-huffman-maxheight compress /tmp/zstd-bench/fixtures/json_logs_32m.jsonl /tmp/ruzstd-json-profile.zst -l 1`
- `perf report --stdio -i /tmp/ruzstd-json-f9735b7.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /tmp/ruzstd-decodecorpus-after-c-hash.perf.data -- /tmp/ruzstd-cli-huffman-maxheight compress /tmp/zstd-bench/fixtures/decodecorpus_pack.bin /tmp/ruzstd-decodecorpus-profile.zst -l 1`
- `perf report --stdio -i /tmp/ruzstd-decodecorpus-after-c-hash.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /tmp/ruzstd-json-after-c-hash.perf.data -- /tmp/ruzstd-cli-huffman-maxheight compress /tmp/zstd-bench/fixtures/json_logs_32m.jsonl /tmp/ruzstd-json-profile.zst -l 1`
- `perf report --stdio -i /tmp/ruzstd-json-after-c-hash.perf.data --sort=symbol --no-children`

## Latest Benchmark Snapshot

Script: `tools/benchmark_zstd.py`

This script benchmarks fixtures from `/tmp/zstd-bench/fixtures` one output at a time because `/tmp` is nearly full. The verifier decodes each compressed output with C zstd and compares the decoded bytes against the original fixture bytes; benchmark rows therefore prove both decode success and byte-for-byte identity. The latest saved byte-verified outputs are `/tmp/zstd-rs-benchmark-output-reserve.csv` and `/tmp/zstd-rs-benchmark-output-reserve.md`.

Last run after the larger window, match-length fix, RLE sequence modes, incompressibility gate, raw-block no-index fast path, compact raw literals headers, overlapping match extension, chunked slice comparison, matcher-side repeat-offset probing, hash-match backward extension, exact Huffman table reuse estimates, text-aware non-repeat match threshold, small-block predefined FSE tables, repeat-offset-biased match selection, the 10-byte repeat-offset search early exit, sparse suffix indexing for matches longer than 128 bytes, repeat-offset and hash-candidate minimum-match prechecks, verified-prefix match-length scans, hot helper inlining, the repeat-aware no-match probe step, fixed repeat-candidate loops, candidate-helper inlining, deterministic unstable entropy sorts, text-only wider no-match probing, `usize` repeat-candidate selection, touched-slot suffix-store clearing, direct matcher repeat-history updates, previous-entry-only newest-first cross-window lookup, cached encoder FSE `acc_log`, C-style end-2 sparse match indexing, heap-based Huffman tree construction, cached sequence FSE table references, cached common sequence length-code tables, suffix-hash modulo removal, same-block forward match-length fast path, modest touched-slot preallocation, explicit suffix-candidate checks, direct repeat-offset encoding branches, inlined offset boundary conversions, matcher block-length hoisting, C-style small literal-compression threshold, exact-block EOF lookahead, BitWriter exact-fill fast path, precomputed suffix key values, countdown sequence encoding, inlined literal/match length-code helpers, sparse RLE history indexing, hardened suffix-store sizing, repeat-offset availability pruning, C-sized suffix hash tables, newest-first block-end hash search, C-style single-stream Huffman literals below 256 bytes, C-style minimum-gain rejection for Huffman literal sections, C-style single-stream repeat-Huffman literals below 1 KiB when estimated no larger than a new table, the all-RLE sequence encoding fast path, and conservative compressed-block output reservation:

| Fixture | Upstream bytes | Current bytes | C zstd -1 bytes | Upstream CPU | Current CPU | C zstd -1 CPU |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | 5,976,095 | 5,371,424 | 5,385,951 | 0.13s | 0.17s | 0.04s |
| `json_logs_32m.jsonl` | 3,392,237 | 742,720 | 1,138,701 | 0.18s | 0.11s | 0.04s |
| `repeated_text_32m.txt` | 31,757 | 2,874 | 3,116 | 0.12s | 0.00s | 0.02s |
| `xorshift_32m.bin` | 33,555,213 | 33,555,210 | 33,555,214 | 0.59s | 0.02s | 0.06s |

Peak RSS from the same run:

| Fixture | Upstream RSS | Current RSS | C zstd -1 RSS |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | 6,532 KB | 5,352 KB | 22,048 KB |
| `json_logs_32m.jsonl` | 5,764 KB | 4,656 KB | 19,080 KB |
| `repeated_text_32m.txt` | 5,596 KB | 4,240 KB | 17,768 KB |
| `xorshift_32m.bin` | 6,236 KB | 4,504 KB | 25,612 KB |

Interpretation:

- Size improved materially on `decodecorpus_pack.bin`, `json_logs_32m.jsonl`, and `repeated_text_32m.txt`; the current branch remains smaller than C zstd on all three compressible fixtures and four bytes smaller on xorshift.
- Conservative compressed-block output reservation preserved exact fixture byte counts. The refreshed table measured decodecorpus at 0.16s, JSON at 0.11s, repeated text at 0.00s, and xorshift at 0.02s, with current RSS still well below C zstd on every fixture.
- The all-RLE sequence encoding fast path preserved exact fixture byte counts. The refreshed table measured JSON at 0.10s, decodecorpus at 0.17s, repeated text at 0.00s, and xorshift at 0.02s. Keep it as a covered sequence-side simplification for blocks whose sequence table modes are already RLE, not as a table-selection heuristic.
- C-style single-stream repeat-Huffman literals below 1 KiB, gated by exact estimated size versus a newly generated table, preserved decodecorpus/repeated/xorshift byte counts and improved JSON by 7 bytes. The same table run measured JSON CPU in the retained 0.11s band and decodecorpus at 0.17s; treat the decodecorpus CPU as noise risk rather than a proven regression because byte counts are unchanged and the change is literal-path only. A broader blind-prefer-previous-table variant was rejected after it regressed JSON by 7,340 bytes and decodecorpus by 522 bytes despite improving JSON CPU to 0.10s.
- Removing the redundant previous-Huffman compatibility scan preserved exact fixture byte counts. The refreshed table measured CPU in the retained band: decodecorpus 0.17s, JSON 0.11s, repeated 0.00s, xorshift 0.01s.
- Disabling CLI progress updates for non-terminal benchmark runs preserved exact fixture byte counts and kept CPU medians in the existing band. A follow-up JSON perf sample no longer showed `indicatif` progress symbols near the top; matcher search remained dominant at about 69% and sequence encoding remained secondary at about 9%.
- C-style minimum-gain rejection for Huffman literal sections regressed `decodecorpus_pack.bin` by 2,901 bytes versus the previous retained snapshot, but still keeps it 14,527 bytes smaller than C zstd. Two runs measured decodecorpus CPU at 0.16s instead of the previous 0.17s band, with the other fixture byte counts unchanged. Keep it because it matches C zstd's fast literal acceptance guardrail and has focused emitted-bitstream Rust/C coverage.
- Single-stream Huffman literals below 256 bytes improved `decodecorpus_pack.bin` by 23 bytes, preserved the other PR fixture byte counts, and kept CPU in the existing noise band. Keep it because it matches C zstd's literal-stream selection and has full Rust/C emitted-bitstream coverage.
- C-sized suffix hash tables trade retained compression headroom for a large CPU/RSS win. Divisors 2, 4, 8, 16, and 32 were tested. Divisor 16 keeps `decodecorpus_pack.bin` 17,428 bytes smaller than C zstd and keeps JSON 395,974 bytes smaller than C while reducing current RSS below upstream on the table run. Divisor 32 improved CPU further but regressed decodecorpus to 5,488,132 bytes, larger than C zstd, so it was rejected.
- Newest-first block-end hash search preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.17s both times, JSON at 0.10s then 0.11s, repeated text at 0.00s, and xorshift at 0.02s. Keep it as a C-shaped control-flow cleanup with focused block-end early-exit coverage.
- Repeat-offset availability pruning preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.21s then 0.20s, JSON at 0.12s both times, repeated text at 0.00s/0.01s, and xorshift at 0.02s both times. Keep it as covered C-style stale-repeat pruning rather than a fixture-specific speed win.
- Hardened suffix-store sizing preserved exact fixture byte counts. The table run measured decodecorpus at 0.21s, JSON at 0.12s, repeated text at 0.00s, and xorshift at 0.02s. Keep it as a covered matcher-pool correctness fix rather than a fixture-specific speed win.
- Sparse RLE history indexing preserved exact PR fixture byte counts. Two table runs measured decodecorpus at 0.21s then 0.20s, JSON at 0.11s then 0.12s, repeated text at 0.01s/0.00s, and xorshift at 0.02s both times. The PR fixtures do not heavily exercise whole-block RLE history, so keep this as covered C-shaped RLE-path CPU cleanup rather than a fixture-specific speed win.
- Inlining the literal-length and match-length code helpers preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.21s then 0.20s, JSON at 0.11s both times, repeated text at 0.01s/0.00s, and xorshift at 0.02s both times. The follow-up JSON profile no longer showed `encode_literal_length` or `encode_match_len` as separate top-level symbols, so keep this narrow sequence-side cleanup.
- The empty fastest-block guard preserved exact fixture byte counts. The refreshed table run measured decodecorpus at 0.21s and JSON at 0.12s, within the existing noise band for this branch. Keep it as a covered direct-block correctness guard; normal frame compression already handles empty frames before calling `compress_fastest()`.
- Countdown sequence encoding preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.20s both times and JSON at 0.11s both times; keep it as a small sequence-side cleanup that matches C zstd's indexed reverse-loop shape.
- Precomputed suffix key values preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.20s then 0.21s and JSON at 0.11s both times; keep it as a small safe matcher cleanup that avoids re-reading the current five-byte suffix for every window entry during hash lookup.
- Refreshed profiles after suffix-key reuse still show matcher search as the dominant cost: about 73% of decodecorpus samples and 66% of JSON samples. JSON also showed sequence encoding around 11%, so sequence-side scalar cleanup remains a secondary target when matcher experiments stop moving.
- Exact-block EOF lookahead removed the extra empty final raw block for exact block-multiple inputs. This improved `repeated_text_32m.txt` and `xorshift_32m.bin` by 3 bytes each, with decodecorpus and JSON byte-identical and CPU in the existing noise band across two runs.
- BitWriter exact-fill flushing preserved exact fixture byte counts. Two table runs kept decodecorpus at 0.21s, JSON at 0.11s, and repeated/xorshift in their existing bands; retain it because it removes a cold helper call from a common bitstream boundary and has focused bit-level coverage.
- Refreshed profiles after the BitWriter exact-fill change still show matcher search as the dominant CPU cost: about 70% of decodecorpus samples and 76% of JSON samples. The former `write_bits_64_cold` hotspot dropped to about 0.5% of decodecorpus samples, so further CPU work should stay focused on matcher search/counting unless future profiles shift.
- Lowering the literal-compression threshold to C zstd's 63-byte heuristic improved JSON size by 80,942 bytes and decodecorpus by 1,164 bytes versus the previous retained snapshot. Two runs kept JSON CPU at 0.11s, repeated/xorshift unchanged, and decodecorpus at 0.22s then 0.20s; keep it as a clear compression win with covered bitstream behavior.
- Lowering the repeat-Huffman literal threshold to C zstd's 6-byte heuristic preserved exact PR fixture bytes. Two table runs measured decodecorpus at 0.20s both times, JSON at 0.12s then 0.11s, and repeated/xorshift unchanged; keep it because it is covered C-guided compression-quality work for small literal payloads after Huffman history exists.
- Early raw-literals fallback from the exact Huffman estimate preserved exact PR fixture bytes. Three table runs measured decodecorpus at 0.20s, 0.21s, then 0.20s and JSON at 0.11s, 0.12s, then 0.11s; keep it because it avoids known no-gain Huffman encode work and has focused emitted-bitstream coverage for the estimate-raw path.
- The repeat-offset search early exit, sparse long-match indexing, no-match probe step, and text-only wider probing trade about 50 KiB of decodecorpus compression and 2 bytes of repeated-text compression for a large CPU improvement, while JSON is now materially smaller than before the CPU parser shortcuts. The repeat/hash prechecks, hot helper inlining, fixed repeat-candidate loops, candidate-helper inlining, suffix-hash modulo removal, same-block forward match-length fast path, modest touched-slot preallocation, explicit suffix-candidate checks, direct repeat-offset encoding branches, inlined offset boundary conversions, and matcher block-length hoisting keep output sizes unchanged and improve or simplify CPU hot paths further. The measured current aggregate CPU is now about 0.34s versus about 0.90s before these CPU-focused parser shortcuts.
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
- Tested direct branches for common offset codes 1, 2, and 3 as a narrower alternative to the rejected offset-code cache. Output bytes were unchanged, but decodecorpus measured 0.23s on the first run and only recovered to 0.20s on the repeat, so the branch-free `ilog2` offset-code path remains better.
- Removing Huffman tree-construction `unwrap()` calls preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.20s both times and JSON at 0.11s both times, so keep the explicit invariant handling as a safe, benchmark-neutral cleanup.
- Tested a conservative C-fast-style no-match acceleration step that increased the probe distance after each 128 bytes since the current anchor while still checking skipped repeat-offset positions and indexing skipped suffixes. Decodecorpus grew by 1,438 bytes with no CPU improvement and JSON drifted to 0.11s CPU, so the fixed step-2/step-3 matcher remains better.
- Tested specializing sequence FSE state updates for the common all-table case to remove per-symbol `Option` checks, mirroring C's resolved-state shape. Output bytes were unchanged, but decodecorpus regressed to 0.21s and JSON did not improve, so the existing compact `Option`-based state update remains better.
- Tested comparing previous/new Huffman literal table reuse lengths in one pass over the literal payload instead of separate `encoded_len()` scans. Output bytes were unchanged, but decodecorpus regressed to 0.22s and JSON to 0.12s, so the existing separate estimator scans remain better.
- Tested forcing the safe chunked prefix helper itself inline after profiles kept the matcher body dominant. Output bytes were unchanged, but the repeat table run drifted decodecorpus to 0.21s and did not improve JSON, so the optimizer's existing inlining choice remains better.
- Tested packing a short suffix fingerprint into the high bits of each stored two-candidate suffix index so hash-slot collisions can be filtered before candidate byte comparison, then narrowed it to binary-looking blocks only. Output bytes stayed unchanged and decodecorpus improved to 0.19s, but JSON regressed to 0.12s across runs, so the untagged two-candidate store remains the best aggregate choice.
- RLE literal-section emission did not change the current PR fixture byte counts, so those fixtures do not exercise all-same large literal payloads. Two table runs stayed in the current noise band with decodecorpus at 0.20s/0.19s and JSON at 0.12s both times; keep the change as covered format/compression-quality work rather than as a fixture-specific benchmark win.
- Tested raising the previous-FSE-table repeat threshold from the current small-block limit of 64 sequences to C fast's broader 1000-sequence heuristic, while keeping the all-symbols-encodable guard. Decodecorpus grew by 107 bytes and CPU regressed to 0.22s with no JSON size benefit, so the local narrower repeat-table policy remains better.
- Tested preallocating the temporary buffer used to estimate Huffman table-description length after the small-literal threshold made literal compression more visible in profiles. Fixture bytes were unchanged, but two table runs were CPU-neutral and JSON drifted from 0.11s to 0.12s on the repeat, so the original minimal `Vec::new()` remains better.
- Tested C zstd's cheap `HUF_optimalTableLog()` depth reduction for Huffman table construction after the small-literal threshold made more blocks eligible for Huffman compression. Focused helper tests passed, but fixture sizes regressed: decodecorpus grew by 311 bytes and JSON by 342 bytes, with decodecorpus CPU also drifting to 0.22s, so the retained encoder keeps the 11-bit Huffman cap.
- Tested replacing exact-EOF one-byte lookahead with a full pending-block lookahead to avoid tiny reads on full-block streams. Output bytes were unchanged, but decodecorpus repeatedly regressed to 0.22s and RSS rose slightly, so the simpler one-byte lookahead remains better.
- Tested replacing hot five-byte minimum-match slice comparisons with an explicit safe byte-by-byte helper, as a safe approximation of C zstd's fixed-width `MEM_read32` prechecks. Output bytes stayed unchanged and focused matcher tests passed, but decodecorpus regressed to 0.24s then 0.23s and JSON to 0.12s across two table runs, so the compiler's original slice-comparison shape remains better.
- Tested reducing the matcher text-classifier sample count from 256 to 128 to lower per-block classifier work while preserving the text/binary parser split. Output bytes stayed unchanged and matcher classifier tests passed, but the measured CPU band was indistinguishable from the 256-sample A/B run, so the original more conservative 256-sample classifier remains.
- Tested precomputing the non-zero-literal repeat-candidate array for the no-match skip guard, since skipped probe positions always have at least one literal before them. Output bytes stayed unchanged and focused matcher tests passed, but JSON stayed at 0.12s and decodecorpus drifted to 0.22s on the repeat run, so the original per-position helper remains better.
- Tested forcing the sequence FSE state helper functions inline after JSON profiles showed sequence encoding as a secondary CPU target. Output bytes stayed unchanged and focused tests passed, but JSON regressed to 0.12s across two table runs, so the optimizer's original helper inlining decisions remain better.
- Tested rewriting the whole-block RLE check to load the first byte once and scan the rest of the block. Output bytes stayed unchanged and focused fastest/compressed-block tests passed, but decodecorpus drifted to 0.21s on the repeat run with no clear fixture-wide CPU win, so the original compact iterator check remains.
- Tested using RLE FSE table modes for two repeated sequence codes instead of only three-or-more repeated codes. Output bytes stayed unchanged on the PR fixtures and CPU had no stable win, so the existing small-block predefined-table preference remains better.
- Tested reducing the fastest incompressibility gate sample count from 256 to 128 after fresh profiles showed the gate as a small JSON cost. Focused gate tests passed and xorshift stayed raw, but decodecorpus grew from 5,159,814 bytes to 5,223,327 bytes, so 128 samples are too weak for mixed binary/text inputs and the 256-sample gate remains.
- Tested a less aggressive 192-sample incompressibility gate as a compromise after the 128-sample version regressed decodecorpus size. Output bytes stayed unchanged, but JSON stayed at 0.12s across both table runs and decodecorpus only improved on the first run before returning to 0.21s, so the 256-sample gate remains the best measured point.
- Tested hoisting the current-block byte slice out of the backward-extension loop. Output bytes stayed unchanged and focused matcher tests passed, but decodecorpus regressed to 0.23s on the first table run and only recovered to the normal band on repeat, so the original loop shape remains.
- Tested deferring the current suffix key-value computation until after repeat-offset probing, so repeat matches that skip hash-table search would avoid one five-byte key load. Output bytes stayed unchanged, but two table runs measured decodecorpus at 0.21s with no JSON CPU improvement, so the existing eager key computation remains.
- Tested C-style precomputed sequence code byte arrays for literal length, match length, and offset code selection/encoding. Focused compressed-block, fastest-frame, and clippy checks passed and output bytes stayed unchanged, but the first table run regressed JSON CPU to 0.12s and the repeat showed no stable improvement while adding three side vectors, so the simpler recompute-on-use path remains.
- Tested carrying the current block slice in `MatchCandidateContext` so hot match helpers could avoid reacquiring the last window entry. Focused matcher and fastest-frame tests plus clippy passed and output bytes stayed unchanged, but two table runs held decodecorpus at 0.22s and JSON drifted to 0.12s on the repeat, so the current narrower context remains better.
- Tested replacing BitWriter cold-path per-byte pushes with a single slice copy for full spilled bytes after a 64-bit flush. Focused BitWriter, compressed-block, fastest-frame, and clippy checks passed and output bytes stayed unchanged, but the repeat table run regressed decodecorpus to 0.22s with no stable JSON win, so the original compact byte loop remains.
- Tested replacing the incompressibility gate's sampled-key linear duplicate search with a fixed 512-slot open-addressed set. Output bytes stayed unchanged and focused fastest tests plus clippy passed, but two table runs showed no stable CPU win: decodecorpus measured 0.20s then 0.21s, JSON stayed at 0.11s, and xorshift stayed at 0.02s. The simpler linear scan remains.
- Refreshed profiles after C-sized suffix hash tables. Matcher search still dominates at about 71% of decodecorpus samples and 74% of JSON samples; sequence encoding is a smaller secondary target around 6%.
- Retested suffix-candidate key tags after the smaller C-sized hash table increased collision pressure. Full tag filtering preserved bytes and improved decodecorpus to 0.16s on one run, but JSON regressed badly to 0.14s. Narrowing tags to binary-looking blocks still regressed JSON to 0.14s with no decodecorpus win over the retained C-sized hash baseline, so the untagged two-candidate store remains better.
- Tested a text-only 4 Ki effective suffix hash table after the global 4 Ki table was rejected. It improved decodecorpus by 5 bytes but regressed JSON from 742,727 bytes to 748,825 bytes with no CPU gain, so text-like blocks keep the retained 8 Ki C-fast hash scale.
- Tested narrowing the no-match skip guard so it only protects the primary repeat-offset candidate from being skipped. Decodecorpus grew by 544 bytes and JSON grew by 8,536 bytes with no JSON CPU improvement, so the guard continues checking all three repeat candidates.
- Tested removing the hot-path temporary repeat-candidate array from matcher probing by expanding the three repeat-offset checks into direct branches. Output bytes stayed unchanged, but decodecorpus CPU repeatedly measured 0.17s instead of the retained 0.16s band, so the compact fixed-array helper remains better.

## Next Steps

1. Finish committing and pushing retained progress on the Huffman branch, including the `.gitignore`/benchmark hygiene state where relevant.
2. Profile matcher search and extension paths again on `decodecorpus_pack.bin` and `json_logs_32m.jsonl`; current samples still show matcher search as the dominant cost and sequence encoding as the secondary JSON target.
3. Investigate further safe early-exit or candidate-pruning heuristics in match selection; keep compression-ratio guardrails in tests and benchmarks and compare every retained idea against C zstd's fast parser shape.
4. Continue C-guided Huffman work only where it improves compression, CPU, or correctness clarity. Candidate areas are table-log/depth selection, literal mode selection, repeat-table reuse, and cost estimation, but previously rejected C heuristics should not be retried without new evidence.
5. Keep adding focused helper-level tests plus emitted-bitstream/Rust-decoder/C-decoder interoperability tests for each compression change; excellent coverage is a hard acceptance criterion for retained work.
6. SIMD remains a matcher byte-comparison topic, but current stable safe-Rust options have not beaten the retained chunked comparison; avoid unsafe/nightly SIMD unless the project explicitly accepts that tradeoff.

## Design Rules From PR Review

- Suffix candidate storage should stay readable and typed: use two `Option<NonZeroU32>` fields for oldest/newest candidate indexes rather than reintroducing the packed `Option<NonZeroU64>` representation.
- Keep one-based `NonZeroU32` only inside compact suffix-store storage. Convert back to `usize` before indexing slices/windows, and keep those conversions checked.
- Access to the current/last committed window entry should go through small helpers or explicit `match` branches so empty-window invariants are obvious. Avoid raw `len() - 1` indexing in new matcher code unless the non-empty invariant is immediately established.
- A window entry may have empty data in edge cases, but suffix insertion must not assume a non-empty committed window or a minimum payload length. Empty and shorter-than-min-match entries should return without indexing suffixes.
- `unwrap()`/`expect()` is acceptable in tests, but production matcher/compression logic should use explicit invariant handling. `unsafe` is not a better alternative to `unwrap()` for these paths.
- Keep casts between `usize` and `u32` intentional. `usize` being 64-bit on the current target is not a problem for local slice positions; the memory savings come from compact stored candidate indexes, not from forcing every transient index into `u32`.

## Benchmark And Reporting Rules

- Keep `tools/benchmark_zstd.py` as the canonical local benchmark harness. It must decode each Rust and C compressed output with C zstd and byte-compare the decompressed bytes with the original fixture.
- Save benchmark reports under `/tmp` unless the user asks for a committed artifact. Current retained report paths are `/tmp/zstd-rs-benchmark-no-progress.csv` and `/tmp/zstd-rs-benchmark-no-progress.md`.
- Markdown benchmark reports should use a fixed-width table with these columns: `fixture`, `upstream bytes`, `C bytes`, `new bytes`, `% improvement`, `upstream cpu`, `C cpu`, `new cpu`, `% improvement`.
- Treat the PR's four fixtures as the stable review table, not as sufficient proof of compressor quality. Any major parser/Huffman change should also be checked against the broader fixture expansion plan below.
- Compare against both upstream Rust and C zstd `-1`. Compression wins should state whether they are against upstream, C, or both; CPU regressions should be called out even when size improves.
- For profile runs, prefer `perf record -m 64 -F 999 -g` followed by `perf report --stdio --sort=symbol --no-children`, and record in this file which symbols remain dominant.

## Rejected Ideas To Avoid Repeating

- Directly expanding the matcher repeat-probe fixed-array helper into three explicit branches preserved bytes but repeatedly regressed decodecorpus CPU from the retained 0.16s band to about 0.17s.
- Searching only the newest suffix candidate when a hash slot has both oldest and newest candidates regressed compression: decodecorpus grew to 5,437,724 bytes, larger than C zstd's 5,385,951 bytes, with no CPU win.
- Text-only 4 Ki effective suffix hash tables regressed JSON size with no CPU gain; keep the retained 8 Ki C-fast hash scale.
- Narrowing the no-match skip guard to only the primary repeat offset regressed both decodecorpus and JSON without a JSON CPU improvement.
- Forcing sequence/FSE helpers inline, forcing `encode_offset()` inline, specializing all-table FSE state updates, and caching offset-code tables all failed to produce stable CPU wins.
- Stable safe SIMD-adjacent rewrites tried so far, including wider chunk comparisons and `as_chunks::<N>()`, did not beat the retained `chunks_exact` comparison shape.

## Fixture Expansion Plan

The current four PR fixtures are useful for a stable review table, but they are not broad enough to prove best-quality compressor behavior. Keep them for the PR comparison and add a broader byte-verified local validation suite without committing large binary fixtures.

Planned fixture coverage:

- Silesia corpus or an equivalent mixed real-world corpus, matching the C zstd benchmark style.
- C zstd `datagen` matrix across input sizes and compressibility levels.
- Many small files and tiny payloads, including 0, 1, 6, 63, 128, 256, 4 KiB, and 128 KiB boundary cases.
- Dictionary compression cases, including small samples and dictionary reuse.
- Long-distance and cross-block repetition cases.
- Already-compressed/high-entropy formats to exercise raw fallback and incompressibility gates.
- Generated decodecorpus frame/data pairs where the original data is available for byte comparison.
- Streaming/chunked input cases to catch behavior that whole-buffer tests miss.

Benchmark acceptance rule:

- Every benchmarked compressed output must be decoded with C zstd and compared byte-for-byte against the original fixture.
- The committed PR fixture table may remain small, but optimization decisions should be checked against the broader suite before considering the C-comparison work complete.

## Coverage Audit

- Current branch has focused unit tests for matcher suffix candidates, repeat-offset candidate ordering, repeat-history updates, prechecks, sparse indexing, no-match step selection, FSE table selection, FSE `acc_log` caching, and Huffman length/weight invariants.
- Current branch has focused coverage that suffix hash keys stay inside the slot table without a final modulo, including a non-power-of-two capacity.
- Current branch has focused coverage that suffix lookup through a precomputed five-byte key value matches ordinary suffix lookup.
- Current branch has focused coverage that the default driver allocates suffix stores at the retained C-fast hash-table scale.
- Current branch has focused coverage that touched-slot preallocation stays modest and below the full-clear threshold.
- Current branch has focused coverage that same-block match-length scanning with a verified prefix still handles overlapping matches by comparing against the generic relative-window scanner.
- Current branch has focused coverage for the explicit suffix-candidate helper, including best-candidate replacement and non-offset-1 block-end early exit.
- Current branch has focused coverage that unavailable repeat offsets, including zero and offsets before the retained window, are rejected before repeat probing.
- Current branch has exhaustive helper-level coverage for the cached common literal-length and match-length sequence code tables, including the first uncached boundary for each table. This also covers the retained explicit inlining of those helper paths.
- Current branch has helper-level coverage for offset-code generation across the small repeat-code range plus the first uncached boundary from the rejected offset-code cache experiment.
- Current branch's sequence bitstream loop shape is covered by compressed-block tests, end-to-end Rust/C decoder round-trips, and a mixed predefined-mode sequence-section decoder round-trip that uses varied literal-length, match-length, and offset symbols.
- Current branch has focused coverage that the all-RLE sequence encoding fast path preserves additional bits even when equal RLE symbols cover different represented values.
- Current branch has focused coverage for the one-based `NonZeroU32` suffix candidate representation, including index zero.
- Current branch has focused coverage that an empty committed matcher entry emits no sequence and does not prevent processing a following entry.
- Current branch has emitted-bitstream tests that round-trip fastest compression through the Rust decoder and the C zstd decoder, including mixed text/binary/random frames, history reuse after incompressible blocks, and cross-block repetitive data.
- Current branch has focused fastest-level coverage that whole-block RLE emission uses an RLE block header and round-trips through both the Rust decoder and the C zstd decoder.
- Current branch has focused matcher coverage that whole-block RLE history indexes only the extreme suffixes for the repeated key and still matches a following repeated block when repeat offsets are out of range.
- Current branch has emitted-bitstream coverage that the previous-Huffman-table literal threshold allows a small repeated literal payload to use an RLE literal section and round-trip through both the Rust decoder and the C zstd decoder.
- Current branch has emitted-bitstream coverage that a high-alphabet literal payload with no estimated Huffman gain uses raw literals and round-trips through both the Rust decoder and the C zstd decoder.
- Current branch has emitted-bitstream coverage that Huffman-compressed literal payloads below 256 bytes use the single-stream header and round-trip through both the Rust decoder and the C zstd decoder.
- Current branch has Huffman-table helper coverage that previous-table symbol checks can be done from counts, plus emitted-bitstream coverage that a sub-1 KiB treeless repeat-Huffman literal section uses the single-stream header after an earlier block establishes the table and round-trips through both the Rust decoder and the C zstd decoder.
- Current branch has frame-header coverage for 8-byte frame content sizes so large known-size frames serialize a valid descriptor.
- Current branch has frame-level coverage that exact full-block inputs do not emit an extra empty final block and that one-byte EOF lookahead preserves the first byte of the following block.
- Current branch has focused fastest-block coverage that empty direct block inputs emit a valid raw block without indexing byte 0.
- Current branch still has the existing encode/decode corpus tests and fuzz targets for encode/decode/FSE/Huff0 interop. These are not a replacement for focused regression tests, but they are useful broad coverage.
- Acceptance rule for future retained changes: add a focused unit/regression test for the changed invariant or an end-to-end Rust+C decode test for emitted-bitstream behavior. If a change is purely a benchmark-only micro-optimization, document why in this file.
