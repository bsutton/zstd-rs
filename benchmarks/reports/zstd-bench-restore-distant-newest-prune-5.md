# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-restore-distant-newest-prune-5.csv`

Commentary: Restore check after rejecting the newest@1-gated distant-oldest prune; confirms the live source tree matches the retained distant-newest baseline at level 4.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  4,675,636       4,789,813  4,675,636  +0.0%          0.89s         0.08s  0.86s    +3.4%        
json_logs_32m.jsonl    602,826         1,361,274  602,826    +0.0%          0.22s         0.08s  0.22s    +0.0%        
```
