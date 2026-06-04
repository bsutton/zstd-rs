# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-restore-skipolder56-prefix8-prune.csv`

Commentary: Raised the current-entry long-hash skip-older threshold from 48 to 56.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  4,675,782       4,789,813  4,675,782  +0.0%          0.83s         0.07s  0.82s    +1.2%        
json_logs_32m.jsonl    602,826         1,361,274  602,826    +0.0%          0.21s         0.08s  0.21s    +0.0%        
```
