# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-unknown-offsetaware-fast.csv`

Commentary: For large Fastest Unknown non-text blocks, prefer a smaller-offset non-repeat candidate at the same start position when it saves at least 4 offset-code bits for at most a 2-byte match-length loss. This directly targets the retained z000079 offset-bit gap.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes     New bytes   % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ----------  ----------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,319,265       5,385,951   5,319,265   +0.0%          0.22s         0.04s  0.23s    -4.5%        
json_logs_32m.jsonl    690,084         1,138,701   690,084     +0.0%          0.17s         0.05s  0.17s    +0.0%        
repeated_text_32m.txt  2,874           3,116       2,874       +0.0%          0.06s         0.02s  0.07s    -16.7%       
xorshift_32m.bin       33,555,210      33,555,214  33,555,210  +0.0%          0.01s         0.06s  0.01s    +0.0%        
```
