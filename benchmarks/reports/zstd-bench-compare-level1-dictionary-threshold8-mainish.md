# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-dictionary-threshold8-mainish.csv`

Commentary: Experiment: raise the non-repeat floor only for DictionaryText to the retained text floor of 8.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                         Upstream bytes  C bytes  New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
------------------------------  --------------  -------  ---------  -------------  ------------  -----  -------  -------------
build_ruzstd-cli                860,072         894,099  860,072    +0.0%          0.05s         0.00s  0.05s    +0.0%        
dict_dictionary.bin             20,667          20,145   19,988     +3.3%          0.00s         0.00s  0.00s    +0.0%        
generated_json_logs_001m.jsonl  58,767          59,118   58,767     +0.0%          0.00s         0.00s  0.01s    +0.0%        
repo_match_generator.rs         22,587          22,797   22,587     +0.0%          0.00s         0.00s  0.00s    +0.0%        
```
