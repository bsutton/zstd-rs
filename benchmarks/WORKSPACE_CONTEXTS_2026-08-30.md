# Reusable workspace performance — August 30, 2026

Issue #11 adds typed reusable encoder and decoder workspaces, guaranteed
allocation-free prepared operations, and contexts backed by arbitrary caller
byte slices. This report checks that the supporting arena representation does
not penalize the established single-thread compressor.

## Method

- Corpus: `benchmarks/archive/tmp/realworld-100/corpus_z000033` (1,022,035 bytes)
- Levels: 1, 3, 5, 8, and 16
- Ten compressions per sample, three samples per mode; tables report medians
- Counters: Linux `perf stat` instructions, cycles, branches, and branch misses
- Baseline: preserved release binary from commit `d21b532`, SHA-256
  `ca371dd0ed962589ff746a43ff8336fbc5bc3f4968e855ef16c8f41d7a0588b4`
- Candidate ordinary API binary SHA-256:
  `73e0ce18079365bec81adac75a2f3b5c89eb49b61277137137fc31cce05e79fa`
- Candidate workspace API binary SHA-256:
  `ac2fbad34f546d08f581874cc7c6da405eee65e33381db22fdf4cbbc9da46316`

Raw counter files and generated summaries are in
`benchmarks/tmp/workspace-context-perf-final-bound-check/`.

## Results

| Level | Output bytes, 10 runs | Baseline instructions | Ordinary instructions | Ordinary vs baseline | Workspace vs ordinary |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 5,715,250 | 275,538,298 | 275,504,897 | -0.0121% | -0.3947% |
| 3 | 4,988,910 | 471,190,768 | 471,770,696 | +0.1231% | -0.5254% |
| 5 | 4,883,630 | 1,012,698,279 | 1,006,400,913 | -0.6218% | -0.0875% |
| 8 | 4,843,360 | 1,391,494,212 | 1,385,175,138 | -0.4541% | -0.2239% |
| 16 | 4,605,550 | 3,962,655,095 | 3,939,650,492 | -0.5805% | -0.3428% |

Output sizes are byte-identical across the baseline, current ordinary API, and
workspace API at every measured level. Workspace cycle changes relative to the
current ordinary API are -2.26%, -2.11%, -2.30%, +0.62%, and -3.65% at levels
1, 3, 5, 8, and 16 respectively.

The ordinary path has no 1% instruction regression; its worst measured change
is +0.12% at level 3. The workspace path is instruction-cheaper at all five
levels. Retain the arena implementation.

## Allocation and large-level evidence

Counting-allocator tests perform repeated prepared operations with zero
allocations for uncompressed mode, negative fast mode, and representative
positive levels 1, 3, 5, 8, 16, and 22. Rust output round-trips through zstd C.
Raw and formatted dictionary decoding are also allocation-free.

An ignored release-scale test forces automatic long-distance matching by
constructing a level-22 workspace for a 64 MiB bound. Two operations complete
without allocation and decode correctly. The test takes about eight seconds
and peaks near 740 MiB RSS, illustrating that deterministic preparation at the
strongest level can require a deliberately large caller allocation.
