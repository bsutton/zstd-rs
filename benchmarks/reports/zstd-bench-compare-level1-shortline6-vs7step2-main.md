# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-shortline6-vs7step2-main.csv`

Commentary: Level-1 text-path experiment: lower the retained short-line threshold from 7 to 6 while keeping the denser short-line probe step.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,323,478       5,385,951  5,323,478  +0.0%          0.23s         0.07s  0.22s    +4.3%        
json_logs_32m.jsonl    690,084         1,138,701  690,084    +0.0%          0.18s         0.05s  0.18s    +0.0%        
```
