# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-trailing-rle-mainish.csv`

Commentary: Experiment: split long trailing single-byte runs into separate Fastest RLE blocks when the suffix run is at least 32 KiB.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                         Upstream bytes  C bytes  New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
------------------------------  --------------  -------  ---------  -------------  ------------  -----  -------  -------------
build_libruzstd.rlib            611,155         635,879  611,155    +0.0%          0.03s         0.00s  0.03s    +0.0%        
build_ruzstd-cli                860,072         894,099  860,072    +0.0%          0.05s         0.00s  0.07s    -40.0%       
decodecorpus_z000079            7,540           7,221    7,750      -2.8%          0.00s         0.00s  0.00s    +0.0%        
generated_json_logs_001m.jsonl  58,767          59,118   58,767     +0.0%          0.00s         0.00s  0.00s    +0.0%        
```
