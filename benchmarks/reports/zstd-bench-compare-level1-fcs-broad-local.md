# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-fcs-broad-local.csv`

Commentary: Experiment: use the known file size on the CLI path to emit a single-segment frame header when compressing files.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                                 Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
--------------------------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
build_libruzstd.rlib                    611,155         635,879    611,158    -0.0%          0.03s         0.00s  0.03s    +0.0%        
build_ruzstd-cli                        860,072         894,099    860,075    -0.0%          0.06s         0.00s  0.05s    +16.7%       
decodecorpus_z000003                    52,134          53,328     52,137     -0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000028                    100,250         105,226    100,253    -0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000030                    13,152          14,106     13,153     -0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000031                    112             127        112        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000033                    544,266         571,529    544,269    -0.0%          0.02s         0.00s  0.02s    +0.0%        
decodecorpus_z000053                    322             299        323        -0.3%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000054                    9,567           9,999      9,568      -0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000059                    711             698        712        -0.1%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000062                    13              13         13         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000077                    21              21         21         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000079                    7,540           7,221      7,543      -0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000080                    2,603           2,613      2,604      -0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_NetworkManager-dispatcher.service  391             384        392        -0.3%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin                     20,667          20,145     20,668     -0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_e2scrub_reap.service               381             378        382        -0.3%          0.00s         0.00s  0.00s    +0.0%        
dict_fstrim.service                     308             295        309        -0.3%          0.00s         0.00s  0.00s    +0.0%        
dict_ftpd.service                       168             164        168        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_kmod-static-nodes.service          497             483        498        -0.2%          0.00s         0.00s  0.00s    +0.0%        
dict_netctl@.service                    206             210        207        -0.5%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-coredump@.service          690             684        691        -0.1%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-logind.service             1,134           1,120      1,135      -0.1%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-udev-settle.service        568             558        569        -0.2%          0.00s         0.00s  0.00s    +0.0%        
dict_telnet@.service                    113             114        113        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_cross_block_001m.bin          6,358           6,587      6,361      -0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_json_logs_001m.jsonl          58,767          59,118     58,770     -0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_repeated_text_001m.txt        208             220        211        -1.4%          0.00s         0.00s  0.00s    +0.0%        
generated_xorshift_001m.bin             1,048,610       1,048,614  1,048,613  -0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.toml                         68              68         68         +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_main.rs                            2,137           2,101      2,138      -0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_match_generator.rs                 22,587          22,797     22,590     -0.0%          0.00s         0.00s  0.00s    +0.0%        
```
