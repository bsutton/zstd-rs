# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-current-longhash-prefix8-prune-repeat-l4.csv`

Commentary: After a current-entry long-hash hit, required older-entry candidates to clear an 8-byte prefix before full match expansion. Repeat-run validation on the main fixtures.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  4,675,782       4,789,813  4,675,559  +0.0%          0.97s         0.07s  0.98s    -1.0%        
json_logs_32m.jsonl    602,826         1,361,274  602,826    +0.0%          0.23s         0.08s  0.22s    +4.3%        
```
