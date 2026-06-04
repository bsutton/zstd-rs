# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-shortline-probestep-split-main.csv`

Commentary: Level-1 text-path experiment: keep the retained code/config threshold split, but use the denser short-line probe step only for non-code short-line text.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,323,478       5,385,951  5,323,478  +0.0%          0.19s         0.04s  0.19s    +0.0%        
json_logs_32m.jsonl    690,084         1,138,701  690,084    +0.0%          0.17s         0.04s  0.17s    +0.0%        
```
