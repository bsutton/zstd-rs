# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-shortline7-step2-vsstep3-broad-local.csv`

Commentary: Level-1 text-path experiment: keep the retained short-line 7-byte threshold and also use the denser no-match probe step on short-line text blocks only.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                                 Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
--------------------------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
build_libruzstd.rlib                    619,650         635,879    619,650    +0.0%          0.04s         0.00s  0.03s    +25.0%       
build_ruzstd-cli                        867,739         894,099    867,739    +0.0%          0.05s         0.00s  0.05s    +0.0%        
decodecorpus_z000003                    52,047          53,328     52,047     +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000028                    100,347         105,226    100,347    +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000030                    13,545          14,106     13,545     +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000031                    118             127        118        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000033                    544,118         571,529    544,118    +0.0%          0.01s         0.00s  0.02s    -100.0%      
decodecorpus_z000053                    324             299        324        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000054                    9,720           9,999      9,720      +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000059                    719             698        719        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000062                    13              13         13         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000077                    21              21         21         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000079                    8,372           7,221      8,372      +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000080                    2,669           2,613      2,669      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_NetworkManager-dispatcher.service  400             384        400        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin                     22,634          20,145     22,101     +2.4%          0.00s         0.00s  0.00s    +0.0%        
dict_e2scrub_reap.service               383             378        383        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_fstrim.service                     304             295        304        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_ftpd.service                       172             164        172        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_kmod-static-nodes.service          499             483        499        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_netctl@.service                    214             210        214        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-coredump@.service          708             684        708        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-logind.service             1,152           1,120      1,150      +0.2%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-udev-settle.service        572             558        572        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_telnet@.service                    111             114        111        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_cross_block_001m.bin          6,359           6,587      6,359      +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_json_logs_001m.jsonl          58,767          59,118     58,767     +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_repeated_text_001m.txt        208             220        208        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_xorshift_001m.bin             1,048,610       1,048,614  1,048,610  +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.toml                         68              68         68         +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_main.rs                            2,211           2,101      2,174      +1.7%          0.00s         0.00s  0.00s    +0.0%        
repo_match_generator.rs                 22,876          22,797     22,469     +1.8%          0.00s         0.00s  0.00s    +0.0%        
```
