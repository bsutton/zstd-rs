# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-current-longhash-skipolder48.csv`

Commentary: Raised the current-entry long-hash skip-older threshold from 40 to 48.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes     New bytes   % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ----------  ----------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  4,675,820       4,789,813   4,675,797   +0.0%          0.92s         0.07s  0.87s    +5.4%        
json_logs_32m.jsonl    602,826         1,361,274   602,826     +0.0%          0.22s         0.08s  0.23s    -4.5%        
repeated_text_32m.txt  2,874           3,128       2,874       +0.0%          0.02s         0.04s  0.02s    +0.0%        
xorshift_32m.bin       33,555,210      33,555,214  33,555,210  +0.0%          0.06s         0.10s  0.06s    +0.0%        
```
