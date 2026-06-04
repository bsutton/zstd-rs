# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-code-config-smallhuff-mainish.csv`

Commentary: Focused check for small code/config exact-Huffman against representative retained level-1 fixtures.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                         Upstream bytes  C bytes  New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
------------------------------  --------------  -------  ---------  -------------  ------------  -----  -------  -------------
build_libruzstd.rlib            611,155         635,879  611,155    +0.0%          0.03s         0.00s  0.04s    -33.3%       
dict_dictionary.bin             20,667          20,145   20,667     +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_json_logs_001m.jsonl  58,767          59,118   58,767     +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_match_generator.rs         22,591          22,797   22,587     +0.0%          0.00s         0.00s  0.00s    +0.0%        
```
