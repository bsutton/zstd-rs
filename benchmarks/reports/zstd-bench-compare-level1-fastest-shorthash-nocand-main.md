# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-fastest-shorthash-nocand-main.csv`

Commentary: Level-1 binary-path experiment: add a Fastest-only current-entry 4-byte hash for non-text blocks, probed only when the current position has no candidate.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes     New bytes   % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ----------  ----------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,319,265       5,385,951   5,282,033   +0.7%          0.20s         0.05s  0.23s    -15.0%       
json_logs_32m.jsonl    690,084         1,138,701   690,084     +0.0%          0.17s         0.04s  0.16s    +5.9%        
repeated_text_32m.txt  2,874           3,116       2,874       +0.0%          0.06s         0.02s  0.07s    -16.7%       
xorshift_32m.bin       33,555,210      33,555,214  33,555,210  +0.0%          0.02s         0.05s  0.02s    +0.0%        
```
