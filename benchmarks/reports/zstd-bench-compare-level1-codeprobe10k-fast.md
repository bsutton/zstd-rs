# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-codeprobe10k-fast.csv`

Commentary: CodeText short-line dense probing widened from 8 KiB to 10 KiB; ConfigText stays at 8 KiB.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                 Upstream bytes  C bytes  New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
----------------------  --------------  -------  ---------  -------------  ------------  -----  -------  -------------
build_ruzstd-cli        866,649         909,857  866,649    +0.0%          0.06s         0.00s  0.06s    +0.0%        
decodecorpus_z000079    7,321           7,221    7,321      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin     20,160          20,145   20,160     +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_benchmark_zstd.py  2,865           2,845    2,846      +0.7%          0.00s         0.00s  0.00s    +0.0%        
repo_compressed.rs      12,839          12,752   12,839     +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_main.rs            2,128           2,142    2,128      +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_progress.rs        3,168           3,124    3,147      +0.7%          0.00s         0.00s  0.00s    +0.0%        
```
