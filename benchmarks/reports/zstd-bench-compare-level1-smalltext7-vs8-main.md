# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-smalltext7-vs8-main.csv`

Commentary: Level-1 text-path experiment: use a 7-byte non-repeat minimum only on smaller text blocks, while keeping the retained text window and probe-step behavior and leaving large text blocks on the 8-byte threshold.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,323,478       5,385,951  5,324,210  -0.0%          0.21s         0.05s  0.19s    +9.5%        
json_logs_32m.jsonl    690,084         1,138,701  809,823    -17.4%         0.13s         0.05s  0.14s    -7.7%        
```
