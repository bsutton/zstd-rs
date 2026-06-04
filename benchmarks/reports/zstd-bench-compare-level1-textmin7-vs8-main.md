# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-textmin7-vs8-main.csv`

Commentary: Level-1 follow-up experiment: lower the text non-repeat minimum match length from 8 to 7 after the retained threshold-8 win on broader text fixtures.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,323,478       5,385,951  5,324,210  -0.0%          0.19s         0.04s  0.22s    -15.8%       
json_logs_32m.jsonl    690,084         1,138,701  809,823    -17.4%         0.13s         0.05s  0.13s    +0.0%        
```
