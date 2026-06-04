# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-current-longhash-distant-oldest-prefix6-repeat-l4.csv`

Commentary: Repeat benchmark for the distant-oldest prefix-6 gate after a current long-hash hit; validates CPU stability against the retained distant-newest baseline.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  4,675,636       4,789,813  4,675,636  +0.0%          0.86s         0.07s  0.96s    -11.6%       
json_logs_32m.jsonl    602,826         1,361,274  602,826    +0.0%          0.25s         0.09s  0.26s    -4.0%        
```
