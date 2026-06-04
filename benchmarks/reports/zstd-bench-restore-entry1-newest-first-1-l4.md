# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-restore-entry1-newest-first-1-l4.csv`

Commentary: Restore check after rejecting the distance-4 oldest prune on top of the retained entry-distance-1 newest-first baseline; confirms the live source tree matches that retained baseline at level 4.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  4,675,636       4,789,813  4,675,636  +0.0%          0.90s         0.06s  0.84s    +6.7%        
json_logs_32m.jsonl    602,826         1,361,274  602,826    +0.0%          0.21s         0.07s  0.21s    +0.0%        
```
