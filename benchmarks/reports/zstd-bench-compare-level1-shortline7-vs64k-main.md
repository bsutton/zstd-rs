# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-shortline7-vs64k-main.csv`

Commentary: Level-1 text-path experiment: use a 7-byte non-repeat minimum on short-line text blocks instead of the earlier 64 KiB size gate, to reach larger source-like text without touching long-line JSON blocks.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,323,478       5,385,951  5,323,478  +0.0%          0.20s         0.04s  0.20s    +0.0%        
json_logs_32m.jsonl    690,084         1,138,701  690,084    +0.0%          0.14s         0.04s  0.15s    -7.1%        
```
