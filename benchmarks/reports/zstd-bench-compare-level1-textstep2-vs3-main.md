# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-textstep2-vs3-main.csv`

Commentary: Level-1 text-path experiment: reduce the text no-match probe step from 3 to 2 to search more densely on classified text blocks.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,323,478       5,385,951  5,323,528  -0.0%          0.18s         0.05s  0.20s    -11.1%       
json_logs_32m.jsonl    690,084         1,138,701  713,323    -3.4%          0.13s         0.04s  0.13s    +0.0%        
```
