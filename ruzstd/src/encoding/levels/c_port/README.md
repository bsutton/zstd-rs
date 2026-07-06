# C Compressor Port Source Map

Authoritative C source: the local `zstd-sys` checkout matching `Cargo.lock`.

Do not re-discover or re-clone the C implementation for this port unless the
local `zstd-sys` checkout is missing. Use other local zstd checkouts only for
deliberate comparisons.

This module is a staged Rust port of the upstream zstd C compressor. Keep new
code split by the same behavioral boundaries as the C implementation, while
using Rust ownership and types instead of transliterating pointer-heavy C.

Local C source:

- Use the local `zstd-sys` source matching `Cargo.lock` as the authoritative C
  source tree for this porting work:
  `/home/bsutton/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zstd-sys-2.0.16+zstd.1.5.7/zstd`.
- `/tmp/zstd-reference` may exist in some sessions as a scratch checkout, but
  `/tmp` can be cleaned. Recreate it from the same upstream version before doing
  broad line-by-line parity audits that need an editable copy.

Primary C references:

- `lib/compress/clevels.h`: compression level parameter table.
- `lib/compress/zstd_compress.c`: frame/block orchestration, parameter
  adjustment, block compressor selection, one-shot API behavior, and dictionary
  loading via `ZSTD_compress_insertDictionary()` and
  `ZSTD_loadDictionaryContent()`.
- `lib/compress/zstd_fast.c`: level 1/2 fast match finder.
- `lib/compress/zstd_double_fast.c`: double-fast match finder.
- `lib/compress/zstd_lazy.c`: greedy, lazy, lazy2, and btlazy2 search.
- `lib/compress/zstd_opt.c`: btopt, btultra, and btultra2 parser.
- `lib/compress/zstd_compress_literals.c`: literal block compression.
- `lib/compress/zstd_compress_sequences.c`: sequence entropy encoding.
- `lib/compress/zstd_compress_superblock.c`: superblock path.

Porting rule: add parity tests at the module boundary before wiring the module
into the active encoder path.

Current parity notes:

- Be explicit about which C entry point a benchmark uses. The `zstd` CLI can
  produce different block boundaries from `ZSTD_compress2()` one-shot
  compression, even with `--single-thread`. For example, on
  `decodecorpus_files/z000044` at level 19, a local debug harness calling
  `ZSTD_compress2()` produced 231,857 bytes, matching Rust's 231,859 bytes,
  while the CLI comparison produced 231,665 bytes by taking a different
  streaming/chunking path. Treat CLI-only block-shape deltas as streaming-mode
  evidence until they reproduce through the one-shot C API.
- `targetCBlockSize` / superblock compression is selected by C only when
  `ZSTD_c_targetCBlockSize` is explicitly non-zero. Normal level-based
  compression, including the local level-16 benchmarks, uses the regular
  block path. Treat this as a full-port gap, not as an explanation for the
  current C comparison gap.
- `zstd_opt.c` seeds optimal-parser literal/LL/ML/offset frequencies from
  full-dictionary entropy tables when dictionary repeat tables are valid. The
  Rust full-dictionary path now derives and consumes the same shape of
  price-model seeds alongside the block entropy tables.

Current profiling notes:

- On July 6, 2026, the focused level-16 `corpus_z000033` comparison still shows
  the remaining CPU gap in the optimal-parser family rather than in an unrelated
  subsystem. `profile_c_port` compressed 80 runs in about 11.2s, while the
  matching `profile_c_api` helper compressed 80 runs through `ZSTD_compress2()`
  in about 4.5s.
- The Rust profile for that fixture is led by
  `opt_match::tree::bt_get_all_matches_no_dict_mls`, with secondary cost in
  post-split block-size estimation, FSE table building, Huffman table building,
  and optimal-parser price updates.
- The C API profile for the same fixture is led by
  `ZSTD_btGetAllMatches_noDict_5`, `ZSTD_compressBlock_opt0`, and
  `ZSTD_insertBt1`, so the next high-value work should stay focused on the
  `zstd_opt.c` match finder/parser implementation before broad entropy
  refactors.
- Rejected micro-optimizations from the same session: unchecked byte reads for
  the binary-tree branch byte compare, iterating over a match slice instead of
  indexing `state.matches`, moving the FSE spread-symbol scratch buffer to the
  stack, and lowering the FSE direct-lookup threshold. Each preserved compressed
  output bytes but did not improve the focused 80-run timing sample.
