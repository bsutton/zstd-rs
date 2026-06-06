# C Compressor Port Source Map

This module is a staged Rust port of the upstream zstd C compressor. Keep new
code split by the same behavioral boundaries as the C implementation, while
using Rust ownership and types instead of transliterating pointer-heavy C.

Primary C references:

- `lib/compress/clevels.h`: compression level parameter table.
- `lib/compress/zstd_compress.c`: frame/block orchestration, parameter
  adjustment, block compressor selection, and one-shot API behavior.
- `lib/compress/zstd_fast.c`: level 1/2 fast match finder.
- `lib/compress/zstd_double_fast.c`: double-fast match finder.
- `lib/compress/zstd_lazy.c`: greedy, lazy, lazy2, and btlazy2 search.
- `lib/compress/zstd_opt.c`: btopt, btultra, and btultra2 parser.
- `lib/compress/zstd_compress_literals.c`: literal block compression.
- `lib/compress/zstd_compress_sequences.c`: sequence entropy encoding.
- `lib/compress/zstd_compress_superblock.c`: superblock path.

Porting rule: add parity tests at the module boundary before wiring the module
into the active encoder path.
