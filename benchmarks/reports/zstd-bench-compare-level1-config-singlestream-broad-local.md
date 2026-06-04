# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-config-singlestream-broad-local.csv`

Commentary: Small ConfigText literal sections may force single-stream Huffman encoding up to 1024 literals at level 1.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
-------------------------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
build_libruzstd.rlib                   611,155         635,879    611,155    +0.0%          0.03s         0.00s  0.03s    +0.0%        
build_ruzstd-cli                       866,649         909,857    866,649    +0.0%          0.06s         0.00s  0.06s    +0.0%        
decodecorpus_z000003                   51,012          53,328     51,012     +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000028                   98,381          105,226    98,381     +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000030                   13,152          14,106     13,152     +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000031                   112             127        112        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000033                   532,632         571,529    532,632    +0.0%          0.02s         0.00s  0.02s    +0.0%        
decodecorpus_z000053                   304             299        304        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000054                   9,567           9,999      9,567      +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000059                   711             698        711        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000062                   13              13         13         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000077                   21              21         21         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000079                   7,321           7,221      7,321      +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000080                   2,603           2,613      2,603      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_canberra-system-bootup.service    316             317        307        +2.8%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin                    20,160          20,145     20,160     +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_git-daemon@.service               248             247        241        +2.8%          0.00s         0.00s  0.00s    +0.0%        
dict_glustereventsd.service            292             293        285        +2.4%          0.00s         0.00s  0.00s    +0.0%        
dict_gpm.service                       191             193        191        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_quotaon.service                   420             420        412        +1.9%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-journal-gatewayd.service  630             631        622        +1.3%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-logind.service            1,122           1,120      1,122      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-rfkill.service            469             470        462        +1.5%          0.00s         0.00s  0.00s    +0.0%        
dict_talk.service                      160             154        160        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_telnet@.service                   113             114        113        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_virtlockd.service                 450             458        442        +1.8%          0.00s         0.00s  0.00s    +0.0%        
dict_virtlogd.service                  535             543        527        +1.5%          0.00s         0.00s  0.00s    +0.0%        
dict_virtqemud.service                 672             673        664        +1.2%          0.00s         0.00s  0.00s    +0.0%        
dict_virtsecretd.service               316             320        308        +2.5%          0.00s         0.00s  0.00s    +0.0%        
generated_cross_block_001m.bin         6,358           6,587      6,358      +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_json_logs_001m.jsonl         58,767          59,118     58,767     +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_repeated_text_001m.txt       208             220        208        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_xorshift_001m.bin            1,048,610       1,048,614  1,048,610  +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_.gitignore                        172             164        172        +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.toml                        737             726        730        +0.9%          0.00s         0.00s  0.00s    +0.0%        
repo_benchmark_zstd.py                 2,814           2,845      2,814      +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_compressed.rs                     12,695          12,752     12,695     +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_main.rs                           2,125           2,142      2,125      +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_match_generator.rs                26,192          26,851     26,192     +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_prepare_benchmark_suites.py       2,291           2,347      2,291      +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_progress.rs                       3,125           3,124      3,125      +0.0%          0.00s         0.00s  0.00s    +0.0%        
```
