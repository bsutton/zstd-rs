# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-smalltext-threshold128-main-l1.csv`

Commentary: Level-1 experiment: classify printable blocks as text starting at 128 bytes instead of 1024, targeting the small text and service-file losses from the broader suite.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,324,267       5,385,951  5,324,267  +0.0%          0.19s         0.04s  0.20s    -5.3%        
json_logs_32m.jsonl    690,084         1,138,701  690,084    +0.0%          0.12s         0.05s  0.13s    -8.3%        
```
