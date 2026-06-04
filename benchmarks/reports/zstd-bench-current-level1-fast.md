# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-current-level1-fast.csv`

Commentary: Current retained level-1 fast baseline after generic DictionaryText second_newest.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                         Upstream bytes  C bytes  New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
------------------------------  --------------  -------  ---------  -------------  ------------  -----  -------  -------------
build_ruzstd-cli                854,529         906,038  854,529    +0.0%          0.07s         0.00s  0.07s    +0.0%        
decodecorpus_z000079            7,322           7,221    7,322      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin             19,668          20,145   19,668     +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_json_logs_001m.jsonl  58,767          59,118   58,767     +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.lock                 9,114           8,088    9,114      +0.0%          0.00s         0.00s  0.00s    +0.0%        
```
