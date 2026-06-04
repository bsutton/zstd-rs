# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-current-longhash-entry1-newest-first-repeat-l1.csv`

Commentary: Repeat benchmark for the entry-distance-1 newest-first override after a current long-hash hit; validates CPU stability against the retained distant-newest baseline.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
decodecorpus_pack.bin  5,324,267       5,385,951  5,324,267  +0.0%          0.19s         0.04s  0.18s    +5.3%        
json_logs_32m.jsonl    690,084         1,138,701  690,084    +0.0%          0.13s         0.04s  0.12s    +7.7%        
```
