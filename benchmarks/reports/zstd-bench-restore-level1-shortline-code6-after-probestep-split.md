# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-restore-level1-shortline-code6-after-probestep-split.csv`

Commentary: Restore check after rejecting the split short-line probe-step experiment. Current tree should match the retained code/config threshold-split baseline on the main fixtures.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,323,478       5,385,951  5,323,478  +0.0%          0.20s         0.04s  0.19s    +5.0%        
json_logs_32m.jsonl    690,084         1,138,701  690,084    +0.0%          0.16s         0.05s  0.16s    +0.0%        
```
