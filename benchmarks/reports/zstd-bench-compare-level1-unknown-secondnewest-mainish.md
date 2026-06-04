# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-unknown-secondnewest-mainish.csv`

Commentary: Experiment: extend the retained Fastest small-block second-newest path to Unknown non-text blocks up to 128 KiB.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                         Upstream bytes  C bytes  New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
------------------------------  --------------  -------  ---------  -------------  ------------  -----  -------  -------------
build_libruzstd.rlib            611,155         635,879  611,155    +0.0%          0.03s         0.00s  0.03s    +0.0%        
dict_dictionary.bin             23,871          20,145   23,871     +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_json_logs_001m.jsonl  58,767          59,118   58,767     +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_match_generator.rs         22,587          22,797   22,587     +0.0%          0.00s         0.00s  0.00s    +0.0%        
```
