# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-textmin8-main.csv`

Commentary: Level-1 experiment: lower the text non-repeat minimum match length from 10 to 8 to see if small text and source-style fixtures move toward C without disturbing the binary path.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,324,267       5,385,951  5,323,478  +0.0%          0.18s         0.04s  0.19s    -5.6%        
json_logs_32m.jsonl    690,084         1,138,701  690,084    +0.0%          0.13s         0.04s  0.13s    +0.0%        
```
