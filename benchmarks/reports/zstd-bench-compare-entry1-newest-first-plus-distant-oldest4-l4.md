# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-entry1-newest-first-plus-distant-oldest4-l4.csv`

Commentary: On top of the retained entry-distance-1 newest-first override, skip older-entry oldest candidates only at entry distances greater than or equal to 4 after a current long-hash hit. Re-tested after adding intermediate improvement diagnostics.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes     New bytes   % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ----------  ----------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  4,675,636       4,789,813   4,675,913   -0.0%          0.99s         0.07s  0.85s    +14.1%       
json_logs_32m.jsonl    602,826         1,361,274   602,826     +0.0%          0.22s         0.07s  0.23s    -4.5%        
repeated_text_32m.txt  2,874           3,128       2,874       +0.0%          0.02s         0.04s  0.02s    +0.0%        
xorshift_32m.bin       33,555,210      33,555,214  33,555,210  +0.0%          0.05s         0.09s  0.05s    +0.0%        
```
