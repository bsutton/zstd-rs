# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-entry1-newest-first-plus-distant-oldest4-l1.csv`

Commentary: On top of the retained entry-distance-1 newest-first override, skip older-entry oldest candidates only at entry distances greater than or equal to 4 after a current long-hash hit. Re-tested after adding intermediate improvement diagnostics.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes     New bytes   % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ----------  ----------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,324,267       5,385,951   5,324,267   +0.0%          0.19s         0.05s  0.20s    -5.3%        
json_logs_32m.jsonl    690,084         1,138,701   690,084     +0.0%          0.13s         0.05s  0.13s    +0.0%        
repeated_text_32m.txt  2,874           3,116       2,874       +0.0%          0.00s         0.02s  0.00s    +0.0%        
xorshift_32m.bin       33,555,210      33,555,214  33,555,210  +0.0%          0.01s         0.06s  0.02s    -100.0%      
```
