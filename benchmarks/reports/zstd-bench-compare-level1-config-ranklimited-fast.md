# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-config-ranklimited-fast.csv`

Commentary: Rank-limited candidate in exact Huffman search

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                         Upstream bytes  C bytes  New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
------------------------------  --------------  -------  ---------  -------------  ------------  -----  -------  -------------
build_ruzstd-cli                854,529         906,038  854,529    +0.0%          0.07s         0.00s  0.07s    +0.0%        
decodecorpus_z000079            7,322           7,221    7,322      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin             20,160          20,145   20,160     +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_json_logs_001m.jsonl  58,767          59,118   58,767     +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.lock                 9,114           8,088    9,114      +0.0%          0.00s         0.00s  0.00s    +0.0%        
```
