# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-restore-level1-textclass1024-after-textclass512.csv`

Commentary: Restore check after rejecting the 512-byte level-1 text classifier experiment. Current tree should match the retained text-threshold-8 baseline on the main fixtures.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,323,478       5,385,951  5,323,478  +0.0%          0.18s         0.05s  0.19s    -5.6%        
json_logs_32m.jsonl    690,084         1,138,701  690,084    +0.0%          0.12s         0.05s  0.12s    +0.0%        
```
