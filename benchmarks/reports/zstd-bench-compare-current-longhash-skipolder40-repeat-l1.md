# zstd-rs Benchmark

Source CSV: `/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-skipolder40-repeat-l1.csv`

Commentary: Raised the current-entry long-hash skip-older threshold from 32 to 40. Repeat-run validation on the main fixtures.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,324,267       5,385,951  5,324,267  +0.0%          0.20s         0.04s  0.19s    +5.0%        
json_logs_32m.jsonl    690,084         1,138,701  690,084    +0.0%          0.13s         0.05s  0.13s    +0.0%        
```
