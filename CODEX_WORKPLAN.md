# Codex Workplan

## Current Goal

Branch: `compression/huffman-maxheight`

Implement and verify the two follow-up compression improvements after merging the two-candidate matcher:

1. Encoder-side Zstd repeat-offset history for sequence offsets.
2. Conservative sequence FSE table selection, using the C zstd implementation as the guide.

Keep the branch correct against the local Rust decoder and C zstd decoder, then benchmark against upstream and C zstd.

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

## Verification So Far

Latest successful commands after the raw-fallback history fix:

- `cargo fmt --all --check`
- `cargo test -q -p ruzstd encoding::blocks::compressed`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q -p ruzstd`
- `cargo build --release -p ruzstd-cli`
- `/tmp/zstd_bench_current_branch.py`

Still pending:

- Decide whether to keep both implemented items together when committing.

## Latest Benchmark Snapshot

Script: `/tmp/zstd_bench_current_branch.py`

This script benchmarks fixtures from `/tmp/zstd-bench/fixtures` one output at a time because `/tmp` is nearly full.

Last run after the raw-fallback history fix:

| Fixture | Upstream bytes | Current bytes | C zstd -1 bytes | Upstream CPU | Current CPU | C zstd -1 CPU |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | 5,976,095 | 5,450,882 | 5,385,951 | 0.13s | 0.15s | 0.05s |
| `json_logs_32m.jsonl` | 3,392,237 | 2,254,327 | 1,138,701 | 0.18s | 0.19s | 0.04s |
| `repeated_text_32m.txt` | 31,757 | 25,382 | 3,116 | 0.11s | 0.12s | 0.02s |
| `xorshift_32m.bin` | 33,555,213 | 33,555,213 | 33,555,214 | 0.59s | 0.56s | 0.05s |

Interpretation:

- Size improved materially on `decodecorpus_pack.bin`, `json_logs_32m.jsonl`, and `repeated_text_32m.txt`.
- CPU is roughly flat on the repetitive fixture after the raw-fallback history fix and faster on the incompressible fixture in the latest run.
- Perf sample on `repeated_text_32m.txt` showed time dominated by `MatchGeneratorDriver::start_matching`; the repeat-offset callback is inlined into that symbol, but the larger future CPU opportunity is still matcher logic.

## Next Steps

1. Decide whether to keep both implemented items together when committing.
2. If optimizing CPU next, profile matcher match-extension paths and compare against C zstd's architecture-specific match/copy routines.
3. Do not start SIMD work in the repeat-offset or FSE selection paths; the useful SIMD target is matcher byte comparison/match extension.
