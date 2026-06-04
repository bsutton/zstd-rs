# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-config-oflog7-fast.csv`

Commentary: ConfigText uses offset_table_max_log = 7 at level 1.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                                 Upstream bytes  C bytes  New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
--------------------------------------  --------------  -------  ---------  -------------  ------------  -----  -------  -------------
build_ruzstd-cli                        855,679         894,099  855,679    +0.0%          0.06s         0.00s  0.06s    +0.0%        
decodecorpus_z000079                    7,321           7,221    7,321      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_NetworkManager-dispatcher.service  381             384      381        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin                     20,160          20,145   20,160     +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_fstrim.service                     299             295      299        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_kmod-static-nodes.service          486             483      486        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-logind.service             1,122           1,120    1,122      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-udev-settle.service        560             558      560        +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_main.rs                            2,105           2,101    2,105      +0.0%          0.00s         0.00s  0.00s    +0.0%        
```
