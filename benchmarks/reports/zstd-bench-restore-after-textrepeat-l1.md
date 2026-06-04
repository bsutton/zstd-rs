# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-restore-after-textrepeat-l1.csv`

Commentary: Restore check after reverting the rejected level-1 text-repeat-pipeline experiment.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                                 Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
--------------------------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
build_libruzstd.rlib                    619,650         635,879    619,650    +0.0%          0.03s         0.00s  0.03s    +0.0%        
build_ruzstd-cli                        870,526         894,099    870,526    +0.0%          0.06s         0.00s  0.05s    +16.7%       
decodecorpus_z000003                    52,047          53,328     52,047     +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000028                    100,347         105,226    100,347    +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000030                    13,545          14,106     13,545     +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000031                    118             127        118        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000033                    544,118         571,529    544,118    +0.0%          0.02s         0.00s  0.02s    +0.0%        
decodecorpus_z000053                    324             299        324        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000054                    9,720           9,999      9,720      +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000059                    719             698        719        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000062                    13              13         13         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000077                    21              21         21         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000079                    8,358           7,221      8,358      +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000080                    2,669           2,613      2,669      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_NetworkManager-dispatcher.service  400             384        400        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin                     25,598          20,145     25,598     +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_e2scrub_reap.service               383             378        383        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_fstrim.service                     304             295        304        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_ftpd.service                       172             164        172        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_kmod-static-nodes.service          499             483        499        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_netctl@.service                    214             210        214        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-coredump@.service          725             684        725        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-logind.service             1,206           1,120      1,206      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-udev-settle.service        572             558        572        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_telnet@.service                    111             114        111        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_cross_block_001m.bin          6,359           6,587      6,359      +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_json_logs_001m.jsonl          58,767          59,118     58,767     +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_repeated_text_001m.txt        208             220        208        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_xorshift_001m.bin             1,048,610       1,048,614  1,048,610  +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.toml                         68              68         68         +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_main.rs                            2,402           2,101      2,402      +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_match_generator.rs                 24,884          22,797     24,884     +0.0%          0.00s         0.00s  0.00s    +0.0%        
```
