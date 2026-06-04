# zstd-rs Benchmark

Source CSV: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark.csv`

Commentary: Large Unknown Fastest path: increase repeat-vs-normal match margin from 4 to 5 to see whether the retained z000079 gain can be pushed further.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes     New bytes   % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ----------  ----------  -------------  ------------  -----  -------  -------------
build_ruzstd-cli       855,996         894,099     856,110     -0.0%          0.06s         0.00s  0.06s    +0.0%        
decodecorpus_pack.bin  5,319,265       5,385,951   5,319,265   +0.0%          0.23s         0.04s  0.23s    +0.0%        
decodecorpus_z000079   7,329           7,221       7,326       +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin    20,175          20,145      20,175      +0.0%          0.00s         0.00s  0.00s    +0.0%        
json_logs_32m.jsonl    690,084         1,138,701   690,084     +0.0%          0.16s         0.05s  0.17s    -6.3%        
repeated_text_32m.txt  2,874           3,116       2,874       +0.0%          0.06s         0.02s  0.06s    +0.0%        
repo_main.rs           2,137           2,101       2,137       +0.0%          0.00s         0.00s  0.00s    +0.0%        
xorshift_32m.bin       33,555,210      33,555,214  33,555,210  +0.0%          0.01s         0.05s  0.01s    +0.0%        
```
