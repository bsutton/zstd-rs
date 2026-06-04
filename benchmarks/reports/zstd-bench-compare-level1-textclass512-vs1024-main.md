# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-textclass512-vs1024-main.csv`

Commentary: Level-1 text-path experiment: classify printable blocks as text starting at 512 bytes instead of 1024, targeting mid-sized service and source fixtures without reclassifying the smallest text blocks.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,323,478       5,385,951  5,323,478  +0.0%          0.19s         0.04s  0.19s    +0.0%        
json_logs_32m.jsonl    690,084         1,138,701  690,084    +0.0%          0.12s         0.05s  0.12s    +0.0%        
```
