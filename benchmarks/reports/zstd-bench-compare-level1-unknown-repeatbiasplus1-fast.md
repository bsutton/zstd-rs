# zstd-rs Benchmark

Source CSV: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark.csv`

Commentary: Large Unknown Fastest path: increase repeat-vs-normal match margin from 2 to 3 via a context-sensitive comparator, to see if cheaper repeat offsets can shrink the remaining z000079 gap with less collateral than the +2 variant.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes     New bytes   % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ----------  ----------  -------------  ------------  -----  -------  -------------
build_ruzstd-cli       855,908         894,099     856,018     -0.0%          0.06s         0.00s  0.06s    +0.0%        
decodecorpus_pack.bin  5,319,265       5,385,951   5,319,265   +0.0%          0.22s         0.04s  0.22s    +0.0%        
decodecorpus_z000079   7,340           7,221       7,340       +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin    20,175          20,145      20,175      +0.0%          0.00s         0.00s  0.00s    +0.0%        
json_logs_32m.jsonl    690,084         1,138,701   690,084     +0.0%          0.17s         0.04s  0.16s    +5.9%        
repeated_text_32m.txt  2,874           3,116       2,874       +0.0%          0.05s         0.02s  0.05s    +0.0%        
repo_main.rs           2,137           2,101       2,137       +0.0%          0.00s         0.00s  0.00s    +0.0%        
xorshift_32m.bin       33,555,210      33,555,214  33,555,210  +0.0%          0.01s         0.05s  0.01s    +0.0%        
```
