# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-current-longhash-newest1-gates-oldest.csv`

Commentary: After a current-entry long-hash hit, skip older-entry oldest candidates at entry distances greater than or equal to 3 only after newest at entry distance 1 has already improved the candidate.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes     New bytes   % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ----------  ----------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  4,675,636       4,789,813   4,675,688   -0.0%          0.91s         0.07s  0.90s    +1.1%        
json_logs_32m.jsonl    602,826         1,361,274   602,826     +0.0%          0.23s         0.07s  0.23s    +0.0%        
repeated_text_32m.txt  2,874           3,128       2,874       +0.0%          0.02s         0.04s  0.02s    +0.0%        
xorshift_32m.bin       33,555,210      33,555,214  33,555,210  +0.0%          0.05s         0.10s  0.05s    +0.0%        
```
