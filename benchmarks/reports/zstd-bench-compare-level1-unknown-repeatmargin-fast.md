# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-unknown-repeatmargin-fast.csv`

Commentary: Reduce repeat-match advantage from 2 bytes to 1 for large Fastest Unknown non-text blocks so window matches can win more often on z000079-like data.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes     New bytes   % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ----------  ----------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,319,265       5,385,951   5,319,265   +0.0%          0.23s         0.04s  0.26s    -13.0%       
json_logs_32m.jsonl    690,084         1,138,701   690,084     +0.0%          0.17s         0.04s  0.18s    -5.9%        
repeated_text_32m.txt  2,874           3,116       2,874       +0.0%          0.07s         0.02s  0.05s    +28.6%       
xorshift_32m.bin       33,555,210      33,555,214  33,555,210  +0.0%          0.01s         0.05s  0.01s    +0.0%        
```
