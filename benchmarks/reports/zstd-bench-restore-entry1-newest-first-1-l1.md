# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-restore-entry1-newest-first-1-l1.csv`

Commentary: Restore check after rejecting the distance-4 oldest prune on top of the retained entry-distance-1 newest-first baseline; confirms the live source tree matches that retained baseline at level 1.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,324,267       5,385,951  5,324,267  +0.0%          0.19s         0.04s  0.19s    +0.0%        
json_logs_32m.jsonl    690,084         1,138,701  690,084    +0.0%          0.13s         0.05s  0.13s    +0.0%        
```
