# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-current-longhash-prefix8-prune.csv`

Commentary: After a current-entry long-hash hit, required older-entry candidates to clear an 8-byte prefix before full match expansion.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes     New bytes   % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ----------  ----------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  4,675,782       4,789,813   4,675,559   +0.0%          1.07s         0.06s  0.91s    +15.0%       
json_logs_32m.jsonl    602,826         1,361,274   602,826     +0.0%          0.23s         0.08s  0.22s    +4.3%        
repeated_text_32m.txt  2,874           3,128       2,874       +0.0%          0.03s         0.04s  0.03s    +0.0%        
xorshift_32m.bin       33,555,210      33,555,214  33,555,210  +0.0%          0.06s         0.09s  0.06s    +0.0%        
```
