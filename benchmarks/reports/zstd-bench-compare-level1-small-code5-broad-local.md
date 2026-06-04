# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-small-code5-broad-local.csv`

Commentary: Lowered CodeText short-line non-repeat floor from 6 to 5 only for code-like files up to 8 KiB, to target repo_main.rs without reopening large source files or JSON.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                                 Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
--------------------------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
build_libruzstd.rlib                    611,155         635,879    611,155    +0.0%          0.03s         0.00s  0.03s    +0.0%        
build_ruzstd-cli                        855,908         894,099    855,908    +0.0%          0.06s         0.00s  0.06s    +0.0%        
decodecorpus_z000003                    51,006          53,328     51,006     +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000028                    98,592          105,226    98,592     +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000030                    13,152          14,106     13,152     +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000031                    112             127        112        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000033                    532,333         571,529    532,333    +0.0%          0.02s         0.00s  0.02s    +0.0%        
decodecorpus_z000053                    322             299        322        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000054                    9,567           9,999      9,567      +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000059                    711             698        711        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000062                    13              13         13         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000077                    21              21         21         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000079                    7,344           7,221      7,344      +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000080                    2,603           2,613      2,603      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_NetworkManager-dispatcher.service  391             384        391        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin                     20,175          20,145     20,175     +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_e2scrub_reap.service               381             378        381        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_fstrim.service                     308             295        308        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_ftpd.service                       168             164        168        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_kmod-static-nodes.service          497             483        497        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_netctl@.service                    206             210        206        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-coredump@.service          690             684        690        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-logind.service             1,134           1,120      1,134      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-udev-settle.service        568             558        568        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_telnet@.service                    113             114        113        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_cross_block_001m.bin          6,358           6,587      6,358      +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_json_logs_001m.jsonl          58,767          59,118     58,767     +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_repeated_text_001m.txt        208             220        208        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_xorshift_001m.bin             1,048,610       1,048,614  1,048,610  +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.toml                         68              68         68         +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_main.rs                            2,137           2,101      2,136      +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_match_generator.rs                 22,587          22,797     22,587     +0.0%          0.00s         0.00s  0.00s    +0.0%        
```
