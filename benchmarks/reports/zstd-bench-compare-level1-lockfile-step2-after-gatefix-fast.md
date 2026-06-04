# zstd-rs Benchmark

Source CSV: `benchmarks/archive/tmp/fastguard-lockfile-step2-after-gatefix.csv`

Commentary: Repeat screen for lockfile-specific probe step 2 on top of the retained second_newest gate-fix baseline.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                         Upstream bytes  C bytes  New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
------------------------------  --------------  -------  ---------  -------------  ------------  -----  -------  -------------
build_ruzstd-cli                854,529         906,038  854,529    +0.0%          0.07s         0.00s  0.07s    +0.0%        
decodecorpus_z000079            7,322           7,221    7,322      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin             20,160          20,145   20,160     +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_json_logs_001m.jsonl  58,767          59,118   58,767     +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.lock                 9,185           8,088    9,170      +0.2%          0.00s         0.00s  0.00s    +0.0%        
```
