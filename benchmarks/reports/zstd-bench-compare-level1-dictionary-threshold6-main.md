# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-dictionary-threshold6-main.csv`

Commentary: Level-1 file-type starting point: DictionaryText uses exact Huffman literal search plus a 6-byte short-line non-repeat minimum; other file families stay on the retained baseline.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes     New bytes   % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ----------  ----------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,319,265       5,385,951   5,319,265   +0.0%          0.21s         0.04s  0.22s    -4.8%        
json_logs_32m.jsonl    690,084         1,138,701   690,084     +0.0%          0.18s         0.05s  0.16s    +11.1%       
repeated_text_32m.txt  2,874           3,116       2,874       +0.0%          0.06s         0.03s  0.07s    -16.7%       
xorshift_32m.bin       33,555,210      33,555,214  33,555,210  +0.0%          0.01s         0.05s  0.01s    +0.0%        
```
