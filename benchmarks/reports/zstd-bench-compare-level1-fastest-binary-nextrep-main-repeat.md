# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-fastest-binary-nextrep-main-repeat.csv`

Commentary: Level-1 binary-path repeat check: Fastest non-text ip+1 repeat lookahead on the original four-fixture main screen.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes     New bytes   % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ----------  ----------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,323,478       5,385,951   5,319,265   +0.1%          0.18s         0.04s  0.20s    -11.1%       
json_logs_32m.jsonl    690,084         1,138,701   690,084     +0.0%          0.16s         0.05s  0.16s    +0.0%        
repeated_text_32m.txt  2,874           3,116       2,874       +0.0%          0.06s         0.02s  0.06s    +0.0%        
xorshift_32m.bin       33,555,210      33,555,214  33,555,210  +0.0%          0.02s         0.06s  0.02s    +0.0%        
```
