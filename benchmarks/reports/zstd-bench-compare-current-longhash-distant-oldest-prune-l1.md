# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-current-longhash-distant-oldest-prune-l1.csv`

Commentary: After a current-entry long-hash hit, skipped older-entry oldest candidates for entry distances greater than or equal to 3.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes     New bytes   % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ----------  ----------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,324,267       5,385,951   5,324,267   +0.0%          0.19s         0.04s  0.21s    -10.5%       
json_logs_32m.jsonl    690,084         1,138,701   690,084     +0.0%          0.14s         0.04s  0.13s    +7.1%        
repeated_text_32m.txt  2,874           3,116       2,874       +0.0%          0.00s         0.03s  0.00s    +0.0%        
xorshift_32m.bin       33,555,210      33,555,214  33,555,210  +0.0%          0.02s         0.04s  0.02s    +0.0%        
```
