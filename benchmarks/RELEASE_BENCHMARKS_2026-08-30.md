# `zstd-complete` 0.1.0 release benchmarks

These are the publication benchmarks for the initial `zstd-complete` release.
They compare the safe, bounded public Rust API with zstd C 1.5.7. They are not
claims about every workload or machine.

## Environment

- AMD Ryzen 7 3700X, Linux x86-64
- rustc 1.94.1, Cargo `--release`, one code-generation unit
- zstd C 1.5.7 through `ZSTD_compress2()`
- `perf stat -r 3` with three compressions of every input per sample
- input resident in memory before the timed compression loop
- each implementation returns an owned compressed buffer

Raw counter files are named
`benchmarks/tmp/release-silesia-{bounded-api,c-api}-l{1,3,8}.stat`. The
`benchmarks/tmp` directory is intentionally not a release payload; the tables
below preserve the reviewed results.

## Silesia corpus

Silesia is a standard heterogeneous compression corpus containing 12 files
from 5 MiB to 51 MiB: text, executables, source, databases, XML, and medical
images. Its total uncompressed size is 211,938,580 bytes. Files were compressed
individually, following the normal Silesia methodology.

The corpus was obtained from the `MiloszKrajewski/SilesiaCorpus` mirror at
commit `3f3fa2cdbbb3795c903b74e774acb309e1360337`. All 12 extracted files match
the MD5 values published for Silesia. Corpus data is downloaded for the
benchmark and is not redistributed in this repository or crate.

Rust uses the public `encode_all()` API with its default 8 MiB bounded frame
size. C uses one frame per corpus file. This is an intentional product-level
comparison: bounded framing is part of the public Rust API and can cost ratio
and throughput on files larger than 8 MiB.

| Level | Rust bytes | C bytes | Size gap | Rust MB/s | C MB/s | Rust instructions/corpus | C instructions/corpus | Instruction gap |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 73,259,577 | 73,276,937 | -0.024% | 295.1 | 403.2 | 6.029 B | 5.024 B | +19.99% |
| 3 | 66,232,654 | 66,216,569 | +0.024% | 212.3 | 240.0 | 7.204 B | 6.619 B | +8.83% |
| 8 | 59,930,104 | 59,820,035 | +0.184% | 59.5 | 69.4 | 35.041 B | 32.854 B | +6.66% |

Negative size gaps favor Rust. Rust/C cycles per complete corpus were
3.022/2.199 billion at level 1, 4.226/3.793 billion at level 3, and
14.830/13.213 billion at level 8. Every Rust archive was accepted by zstd C;
all 12 C level-3 archives were decoded by Rust and byte-compared with the
original files.

The Rust harness is `tools/src/bin/profile_bounded_api.rs`; the C harness is
`tools/src/bin/profile_c_api.rs`. Their matched invocation shape is:

```sh
cargo build --release -p zstd-rs-tools \
  --bin profile_bounded_api --bin profile_c_api

# Run once for each level in 1, 3, and 8. Each globbed input is one Silesia file.
perf stat -r 3 -e instructions,cycles -- \
  bash -c 'for input in CORPUS/*; do \
    target/release/profile_bounded_api "$input" LEVEL 3 8 96; done'
perf stat -r 3 -e instructions,cycles -- \
  bash -c 'for input in CORPUS/*; do \
    target/release/profile_c_api "$input" LEVEL 3; done'
```

## Official zstd project source fixtures

The official zstd repository was checked out at the 1.5.7 annotated tag object
`ac66b19e6bd6b83238bf008eecc1298105298532` (commit
`f8745da6ff1ad1e7bab384bd1f9d742439278e99`). A deterministic selection of 35
files from its documentation, common, compressor, decompressor, program, and
test sources totals 1,581,447 bytes.

Rust and C each compressed every file at levels 1, 3, 5, 8, 16, 19, and 22.
All 245 Rust outputs decoded with zstd C and byte-compared with their input.
These small files are interoperability and output-size evidence, not useful
throughput evidence.

| Level | Rust aggregate bytes | C aggregate bytes | Rust minus C |
|---:|---:|---:|---:|
| 1 | 393,386 | 393,382 | +4 |
| 3 | 369,461 | 369,462 | -1 |
| 5 | 344,266 | 344,266 | 0 |
| 8 | 329,407 | 329,407 | 0 |
| 16 | 309,582 | 309,584 | -2 |
| 19 | 306,993 | 307,003 | -10 |
| 22 | 306,957 | 306,970 | -13 |

The selection manifest SHA-256 is
`f77444a2b51899d2b4c115006458db9dd71426f2444c2bfbce7bb41b75a5ab1f`.
The four official zstd golden decompression frames (`block-128k`,
`empty-block`, `rle-first-block`, and `zeroSeq_2B`) were also decoded by Rust
and byte-compared with zstd C's decoded output.

## Other retained evidence

The README contains the generated 16 MiB public-streaming matrix and the
focused one-shot core comparison across levels 1, 3, 5, 8, and 16. The focused
case is useful for implementation attribution; Silesia is the primary
real-world release benchmark.
