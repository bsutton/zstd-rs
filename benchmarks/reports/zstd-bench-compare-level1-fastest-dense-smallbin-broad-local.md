# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-fastest-dense-smallbin-broad-local.csv`

Commentary: Level-1 binary-path experiment: force byte-by-byte no-match probing for Fastest non-text blocks up to 64 KiB.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                                 Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
--------------------------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
build_libruzstd.rlib                    611,155         635,879    611,155    +0.0%          0.03s         0.00s  0.03s    +0.0%        
build_ruzstd-cli                        860,072         894,099    860,072    +0.0%          0.05s         0.00s  0.05s    +0.0%        
decodecorpus_z000003                    52,134          53,328     52,134     +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000028                    100,250         105,226    100,250    +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000030                    13,463          14,106     13,152     +2.3%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000031                    116             127        112        +3.4%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000033                    544,266         571,529    544,266    +0.0%          0.02s         0.00s  0.03s    -50.0%       
decodecorpus_z000053                    324             299        322        +0.6%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000054                    9,628           9,999      9,567      +0.6%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000059                    702             698        711        -1.3%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000062                    13              13         13         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000077                    21              21         21         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000079                    7,540           7,221      7,540      +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000080                    2,635           2,613      2,603      +1.2%          0.00s         0.00s  0.00s    +0.0%        
dict_NetworkManager-dispatcher.service  396             384        395        +0.3%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin                     20,667          20,145     20,667     +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_e2scrub_reap.service               383             378        381        +0.5%          0.00s         0.00s  0.00s    +0.0%        
dict_fstrim.service                     304             295        312        -2.6%          0.00s         0.00s  0.00s    +0.0%        
dict_ftpd.service                       172             164        172        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_kmod-static-nodes.service          499             483        497        +0.4%          0.00s         0.00s  0.00s    +0.0%        
dict_netctl@.service                    214             210        212        +0.9%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-coredump@.service          692             684        692        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-logind.service             1,134           1,120      1,134      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-udev-settle.service        572             558        569        +0.5%          0.00s         0.00s  0.00s    +0.0%        
dict_telnet@.service                    111             114        113        -1.8%          0.00s         0.00s  0.00s    +0.0%        
generated_cross_block_001m.bin          6,358           6,587      6,358      +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_json_logs_001m.jsonl          58,767          59,118     58,767     +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_repeated_text_001m.txt        208             220        208        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_xorshift_001m.bin             1,048,610       1,048,614  1,048,610  +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.toml                         68              68         68         +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_main.rs                            2,141           2,101      2,141      +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_match_generator.rs                 22,591          22,797     22,591     +0.0%          0.00s         0.00s  0.00s    +0.0%        
```
