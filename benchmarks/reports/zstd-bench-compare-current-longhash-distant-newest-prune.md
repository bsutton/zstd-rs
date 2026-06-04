# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-current-longhash-distant-newest-prune.csv`

Commentary: After a current-entry long-hash hit, skipped older-entry newest candidates for entry distances greater than or equal to 2.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes     New bytes   % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ----------  ----------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  4,675,782       4,789,813   4,675,636   +0.0%          0.87s         0.07s  0.84s    +3.4%        
json_logs_32m.jsonl    602,826         1,361,274   602,826     +0.0%          0.22s         0.08s  0.21s    +4.5%        
repeated_text_32m.txt  2,874           3,128       2,874       +0.0%          0.02s         0.04s  0.03s    -50.0%       
xorshift_32m.bin       33,555,210      33,555,214  33,555,210  +0.0%          0.05s         0.08s  0.05s    +0.0%        
```
