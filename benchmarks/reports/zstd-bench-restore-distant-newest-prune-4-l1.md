# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-restore-distant-newest-prune-4-l1.csv`

Commentary: Restore check after rejecting the distant-oldest prefix-6 gate; confirms the live source tree matches the retained distant-newest baseline at level 1.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,324,267       5,385,951  5,324,267  +0.0%          0.19s         0.04s  0.19s    +0.0%        
json_logs_32m.jsonl    690,084         1,138,701  690,084    +0.0%          0.13s         0.05s  0.13s    +0.0%        
```
