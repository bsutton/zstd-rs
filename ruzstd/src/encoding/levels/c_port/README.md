# C Compressor Port Source Map

Authoritative C source checkout: `/tmp/zstd-reference`.

Do not re-discover or re-clone the C implementation for this port. Start from
`/tmp/zstd-reference`, and only use other local zstd checkouts for deliberate
comparisons.

This module is a staged Rust port of the upstream zstd C compressor. Keep new
code split by the same behavioral boundaries as the C implementation, while
using Rust ownership and types instead of transliterating pointer-heavy C.

Local C checkout:

- Use `/tmp/zstd-reference` as the authoritative local C source tree for this
  porting work.
- `/tmp/facebook-zstd` is also present, but prefer `/tmp/zstd-reference` unless
  a deliberate comparison between checkouts is needed.

Primary C references:

- `/tmp/zstd-reference/lib/compress/clevels.h`: compression level parameter
  table.
- `/tmp/zstd-reference/lib/compress/zstd_compress.c`: frame/block
  orchestration, parameter adjustment, block compressor selection, and one-shot
  API behavior.
- `/tmp/zstd-reference/lib/compress/zstd_fast.c`: level 1/2 fast match finder.
- `/tmp/zstd-reference/lib/compress/zstd_double_fast.c`: double-fast match
  finder.
- `/tmp/zstd-reference/lib/compress/zstd_lazy.c`: greedy, lazy, lazy2, and
  btlazy2 search.
- `/tmp/zstd-reference/lib/compress/zstd_opt.c`: btopt, btultra, and btultra2
  parser.
- `/tmp/zstd-reference/lib/compress/zstd_compress_literals.c`: literal block
  compression.
- `/tmp/zstd-reference/lib/compress/zstd_compress_sequences.c`: sequence entropy
  encoding.
- `/tmp/zstd-reference/lib/compress/zstd_compress_superblock.c`: superblock
  path.
- `/tmp/zstd-reference/lib/compress/zstd_compress.c`: dictionary loading via
  `ZSTD_compress_insertDictionary()` and `ZSTD_loadDictionaryContent()`.

Porting rule: add parity tests at the module boundary before wiring the module
into the active encoder path.
