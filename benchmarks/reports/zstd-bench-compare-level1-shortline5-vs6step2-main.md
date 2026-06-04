# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-shortline5-vs6step2-main.csv`

Commentary: Level-1 text-path experiment: lower the retained short-line threshold from 6 to 5 while keeping the denser short-line probe step.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,323,478       5,385,951  5,323,478  +0.0%          0.19s         0.04s  0.19s    +0.0%        
json_logs_32m.jsonl    690,084         1,138,701  690,084    +0.0%          0.16s         0.04s  0.17s    -6.3%        
```
