# zstd-rs Benchmark

Source CSV: `/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-skipolder40.csv`

Commentary: Raised the current-entry long-hash skip-older threshold from 32 to 40.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes     New bytes   % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ----------  ----------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  4,675,858       4,789,813   4,675,820   +0.0%          1.15s         0.08s  1.07s    +7.0%        
json_logs_32m.jsonl    602,826         1,361,274   602,826     +0.0%          0.24s         0.08s  0.24s    +0.0%        
repeated_text_32m.txt  2,874           3,128       2,874       +0.0%          0.02s         0.04s  0.03s    -50.0%       
xorshift_32m.bin       33,555,210      33,555,214  33,555,210  +0.0%          0.06s         0.09s  0.06s    +0.0%        
```
