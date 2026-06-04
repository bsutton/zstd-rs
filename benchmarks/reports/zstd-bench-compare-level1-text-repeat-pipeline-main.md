# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-text-repeat-pipeline-main.csv`

Commentary: Level-1 experiment: enable the retained text repeat pipeline for all compressed levels, while keeping the existing text-classification cutoff.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,324,267       5,385,951  5,324,269  -0.0%          0.19s         0.05s  0.19s    +0.0%        
json_logs_32m.jsonl    690,084         1,138,701  690,074    +0.0%          0.13s         0.05s  0.14s    -7.7%        
```
