# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-restore-distant-newest-prune-3.csv`

Commentary: Restore check after reverting the zero-literal selective distant-oldest prune experiment.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  4,675,636       4,789,813  4,675,636  +0.0%          0.93s         0.06s  0.85s    +8.6%        
json_logs_32m.jsonl    602,826         1,361,274  602,826    +0.0%          0.22s         0.08s  0.22s    +0.0%        
```
