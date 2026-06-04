# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-shortline7-step2-vsstep3-main.csv`

Commentary: Level-1 text-path experiment: keep the retained short-line 7-byte threshold and also use the denser no-match probe step on short-line text blocks only.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,323,478       5,385,951  5,323,478  +0.0%          0.22s         0.04s  0.20s    +9.1%        
json_logs_32m.jsonl    690,084         1,138,701  690,084    +0.0%          0.15s         0.05s  0.16s    -6.7%        
```
