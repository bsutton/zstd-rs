# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-current-longhash-distant-newest-prune-repeat-l4.csv`

Commentary: After a current-entry long-hash hit, skipped older-entry newest candidates for entry distances greater than or equal to 2. Repeat-run validation on the main fixtures.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  4,675,782       4,789,813  4,675,636  +0.0%          0.84s         0.07s  0.82s    +2.4%        
json_logs_32m.jsonl    602,826         1,361,274  602,826    +0.0%          0.21s         0.07s  0.21s    +0.0%        
```
