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

- `targetCBlockSize` / superblock compression is selected by C only when
  `ZSTD_c_targetCBlockSize` is explicitly non-zero. Normal level-based
  compression, including the local level-16 benchmarks, uses the regular
  block path. Treat this as a full-port gap, not as an explanation for the
  current C comparison gap.
- `zstd_opt.c` seeds optimal-parser literal/LL/ML/offset frequencies from
  full-dictionary entropy tables when dictionary repeat tables are valid. The
  Rust dictionary path already seeds block entropy tables, but the optimal
  parser still needs a careful parity port of those price-model seeds.
