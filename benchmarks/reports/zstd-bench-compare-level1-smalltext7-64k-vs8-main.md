# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-smalltext7-64k-vs8-main.csv`

Commentary: Level-1 text-path experiment: use a 7-byte non-repeat minimum only on text blocks smaller than 64 KiB, leaving the retained 8-byte threshold on 128 KiB streaming text blocks.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,323,478       5,385,951  5,323,478  +0.0%          0.18s         0.05s  0.19s    -5.6%        
json_logs_32m.jsonl    690,084         1,138,701  690,084    +0.0%          0.14s         0.04s  0.13s    +7.1%        
```
