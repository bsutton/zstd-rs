# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-current-longhash-entry1-newest-first-repeat-l4.csv`

Commentary: Repeat benchmark for the entry-distance-1 newest-first override after a current long-hash hit; validates CPU stability against the retained distant-newest baseline.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  4,675,636       4,789,813  4,675,636  +0.0%          0.91s         0.06s  0.84s    +7.7%        
json_logs_32m.jsonl    602,826         1,361,274  602,826    +0.0%          0.22s         0.07s  0.22s    +0.0%        
```
