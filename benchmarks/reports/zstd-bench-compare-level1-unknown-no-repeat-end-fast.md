# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-unknown-no-repeat-end-fast.csv`

Commentary: Disable all repeat early-exit, including block-end exits, for large Fastest Unknown non-text blocks so window candidates always compete on z000079-like data.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes     New bytes   % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ----------  ----------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,319,265       5,385,951   5,319,265   +0.0%          0.22s         0.04s  0.25s    -13.6%       
json_logs_32m.jsonl    690,084         1,138,701   690,084     +0.0%          0.17s         0.04s  0.16s    +5.9%        
repeated_text_32m.txt  2,874           3,116       2,874       +0.0%          0.06s         0.01s  0.05s    +16.7%       
xorshift_32m.bin       33,555,210      33,555,214  33,555,210  +0.0%          0.01s         0.04s  0.01s    +0.0%        
```
