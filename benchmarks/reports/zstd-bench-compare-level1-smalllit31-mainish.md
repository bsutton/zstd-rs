# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-smalllit31-mainish.csv`

Commentary: Experiment: lower the initial literal-compression floor from 63 bytes to 31 bytes and rely on the existing gain check to reject bad cases.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                         Upstream bytes  C bytes  New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
------------------------------  --------------  -------  ---------  -------------  ------------  -----  -------  -------------
build_libruzstd.rlib            611,155         635,879  611,155    +0.0%          0.03s         0.00s  0.03s    +0.0%        
build_ruzstd-cli                860,072         894,099  860,072    +0.0%          0.05s         0.01s  0.06s    -20.0%       
decodecorpus_z000079            7,540           7,221    8,338      -10.6%         0.00s         0.00s  0.00s    +0.0%        
generated_json_logs_001m.jsonl  58,767          59,118   58,767     +0.0%          0.00s         0.00s  0.00s    +0.0%        
```
