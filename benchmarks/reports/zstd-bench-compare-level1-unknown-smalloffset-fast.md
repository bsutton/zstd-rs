# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-unknown-smalloffset-fast.csv`

Commentary: Prefer smaller-offset non-repeat matches over slightly longer ones on large Fastest Unknown non-text blocks to reduce far-match bias on z000079-like data.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes     New bytes   % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ----------  ----------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,319,265       5,385,951   5,319,265   +0.0%          0.23s         0.04s  0.25s    -8.7%        
json_logs_32m.jsonl    690,084         1,138,701   690,084     +0.0%          0.17s         0.05s  0.17s    +0.0%        
repeated_text_32m.txt  2,874           3,116       2,874       +0.0%          0.06s         0.01s  0.06s    +0.0%        
xorshift_32m.bin       33,555,210      33,555,214  33,555,210  +0.0%          0.01s         0.05s  0.01s    +0.0%        
```
