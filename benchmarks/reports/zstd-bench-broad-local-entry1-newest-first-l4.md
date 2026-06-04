# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-broad-local-entry1-newest-first-l4.csv`

Commentary: Broader local suite for the retained entry-distance-1 newest-first override. Covers generated text/binary data, local decodecorpus samples, dictionary-style service files, repo source files, and build artifacts.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                                 Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
--------------------------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
build_libruzstd.rlib                    578,280         585,932    578,280    +0.0%          0.12s         0.01s  0.11s    +8.3%        
build_ruzstd-cli                        879,794         806,941    879,794    +0.0%          0.23s         0.02s  0.20s    +13.0%       
decodecorpus_z000003                    47,539          48,137     47,539     +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000028                    87,525          94,694     87,525     +0.0%          0.02s         0.00s  0.02s    +0.0%        
decodecorpus_z000030                    13,121          13,348     13,121     +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000031                    118             103        118        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000033                    496,060         505,563    496,060    +0.0%          0.13s         0.00s  0.12s    +7.7%        
decodecorpus_z000053                    323             277        323        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000054                    9,242           9,729      9,242      +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000059                    721             715        721        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000062                    13              13         13         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000077                    21              21         21         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000079                    7,517           6,966      7,517      +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000080                    2,660           2,597      2,660      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_NetworkManager-dispatcher.service  393             379        393        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin                     23,641          18,425     23,641     +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_e2scrub_reap.service               381             377        381        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_fstrim.service                     312             296        312        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_ftpd.service                       172             164        172        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_kmod-static-nodes.service          497             480        497        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_netctl@.service                    212             211        212        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-coredump@.service          725             679        725        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-logind.service             1,201           1,087      1,201      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-udev-settle.service        569             554        569        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_telnet@.service                    113             114        113        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_cross_block_001m.bin          6,358           6,540      6,358      +0.0%          0.01s         0.00s  0.01s    +0.0%        
generated_json_logs_001m.jsonl          51,913          65,785     51,913     +0.0%          0.02s         0.00s  0.01s    +50.0%       
generated_repeated_text_001m.txt        208             219        208        +0.0%          0.01s         0.00s  0.01s    +0.0%        
generated_xorshift_001m.bin             1,048,610       1,048,613  1,048,610  +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.toml                         68              68         68         +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_main.rs                            2,399           1,972      2,399      +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_match_generator.rs                 23,954          19,320     23,954     +0.0%          0.00s         0.00s  0.00s    +0.0%        
```
