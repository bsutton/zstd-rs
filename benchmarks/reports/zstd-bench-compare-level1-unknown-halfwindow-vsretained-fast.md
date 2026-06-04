# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-unknown-halfwindow-vsretained-fast.csv`

Commentary: Large Unknown Fastest path: halve the effective search window for non-text blocks at 128 KiB and above.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes     New bytes   % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ----------  ----------  -------------  ------------  -----  -------  -------------
build_ruzstd-cli       855,679         894,099     865,333     -1.1%          0.06s         0.00s  0.05s    +16.7%       
decodecorpus_pack.bin  5,319,265       5,385,951   5,319,265   +0.0%          0.22s         0.04s  0.22s    +0.0%        
decodecorpus_z000079   7,321           7,221       7,321       +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin    20,160          20,145      20,160      +0.0%          0.00s         0.00s  0.00s    +0.0%        
json_logs_32m.jsonl    690,084         1,138,701   690,084     +0.0%          0.16s         0.05s  0.16s    +0.0%        
repeated_text_32m.txt  2,874           3,116       2,874       +0.0%          0.05s         0.02s  0.06s    -20.0%       
repo_main.rs           2,105           2,101       2,105       +0.0%          0.00s         0.00s  0.00s    +0.0%        
xorshift_32m.bin       33,555,210      33,555,214  33,555,210  +0.0%          0.01s         0.05s  0.01s    +0.0%        
```
