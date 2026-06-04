# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-current-longhash-skipolder48-repeat-l4.csv`

Commentary: Raised the current-entry long-hash skip-older threshold from 40 to 48. Repeat-run validation on the main fixtures.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  4,675,820       4,789,813  4,675,797  +0.0%          0.90s         0.07s  0.84s    +6.7%        
json_logs_32m.jsonl    602,826         1,361,274  602,826    +0.0%          0.22s         0.08s  0.22s    +0.0%        
```
