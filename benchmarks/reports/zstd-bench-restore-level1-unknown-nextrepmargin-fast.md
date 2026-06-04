# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-restore-level1-unknown-nextrepmargin-fast.csv`

Commentary: Restore check after rejecting the large Unknown next-position repeat-margin bonus.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture               Upstream bytes  C bytes  New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
--------------------  --------------  -------  ---------  -------------  ------------  -----  -------  -------------
build_ruzstd-cli      855,679         894,099  855,679    +0.0%          0.06s         0.00s  0.06s    +0.0%        
decodecorpus_z000079  7,321           7,221    7,321      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin   20,160          20,145   20,160     +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_main.rs          2,105           2,101    2,105      +0.0%          0.00s         0.00s  0.00s    +0.0%        
```
