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
- Maintain excellent test coverage for each compression feature, including tests that exercise emitted bitstreams and decoder interoperability, not only helper-level behavior.

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

## Verification So Far

Latest successful commands after the raw-fallback history fix:

- `cargo fmt --all --check`
- `cargo test -q -p ruzstd encoding::blocks::compressed`
- `cargo test -q -p ruzstd encoding::match_generator`
- `cargo test -q -p ruzstd fastest_reuses_history_across_blocks`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q -p ruzstd`
- `cargo build --release -p ruzstd-cli`
- `/tmp/zstd_bench_current_branch.py`

Still pending:

- Decide whether to keep both implemented items together when committing.

## Latest Benchmark Snapshot

Script: `/tmp/zstd_bench_current_branch.py`

This script benchmarks fixtures from `/tmp/zstd-bench/fixtures` one output at a time because `/tmp` is nearly full.

Last run after the larger window, match-length fix, RLE sequence modes, incompressibility gate, raw-block no-index fast path, compact raw literals headers, overlapping match extension, and chunked slice comparison:

| Fixture | Upstream bytes | Current bytes | C zstd -1 bytes | Upstream CPU | Current CPU | C zstd -1 CPU |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | 5,976,095 | 5,165,276 | 5,385,951 | 0.13s | 0.28s | 0.04s |
| `json_logs_32m.jsonl` | 3,392,237 | 2,102,500 | 1,138,701 | 0.18s | 0.26s | 0.04s |
| `repeated_text_32m.txt` | 31,757 | 2,875 | 3,116 | 0.11s | 0.20s | 0.02s |
| `xorshift_32m.bin` | 33,555,213 | 33,555,213 | 33,555,214 | 0.59s | 0.03s | 0.05s |

Interpretation:

- Size improved materially on `decodecorpus_pack.bin`, `json_logs_32m.jsonl`, and `repeated_text_32m.txt`; repeated text is now smaller than C zstd on this fixture.
- The incompressible fixture is now near C zstd CPU after the no-index raw fast path.
- The larger window and overlapping extension improve compression but raise CPU and RSS on compressible fixtures; next work should focus on matcher search strategy, repeat-offset probing, and early-exit heuristics.
- Perf sample on `repeated_text_32m.txt` showed time dominated by `MatchGeneratorDriver::start_matching`; the repeat-offset callback is inlined into that symbol, but the larger future CPU opportunity is still matcher logic.

## Next Steps

1. Implement matcher-side repeat-offset probing, because the JSON gap still appears dominated by offset-code distribution rather than literal compression.
2. Keep adding focused helper-level tests plus emitted-bitstream/Rust-decoder/C-decoder interoperability tests for each compression change.
3. If optimizing CPU next, profile matcher search and extension paths and compare against C zstd's architecture-specific match/copy routines.
4. Do not start SIMD work in the repeat-offset or FSE selection paths; the useful SIMD target is matcher byte comparison/match extension.
