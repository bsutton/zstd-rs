# zstd-rs Benchmark

Source CSV: `benchmarks/archive/tmp/knownsize-header-broad-local.csv`

Commentary: CLI whole-file compression sets frame content size and emits single-segment headers when source size is known.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
-------------------------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
build_libruzstd.rlib                   611,155         635,879    611,158    -0.0%          0.03s         0.00s  0.03s    +0.0%        
build_ruzstd-cli                       862,752         906,038    862,755    -0.0%          0.06s         0.00s  0.07s    -16.7%       
decodecorpus_z000003                   51,012          53,328     51,015     -0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000028                   98,381          105,226    98,384     -0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000030                   13,152          14,106     13,153     -0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000031                   112             127        112        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000033                   532,632         571,529    532,635    -0.0%          0.02s         0.00s  0.02s    +0.0%        
decodecorpus_z000053                   304             299        305        -0.3%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000054                   9,567           9,999      9,568      -0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000059                   711             698        712        -0.1%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000062                   13              13         13         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000077                   21              21         21         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000079                   7,321           7,221      7,324      -0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000080                   2,603           2,613      2,604      -0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_canberra-system-bootup.service    307             317        308        -0.3%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin                    20,160          20,145     20,161     -0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_git-daemon@.service               241             247        242        -0.4%          0.00s         0.00s  0.00s    +0.0%        
dict_glustereventsd.service            285             293        286        -0.4%          0.00s         0.00s  0.00s    +0.0%        
dict_gpm.service                       191             193        192        -0.5%          0.00s         0.00s  0.00s    +0.0%        
dict_quotaon.service                   412             420        413        -0.2%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-journal-gatewayd.service  622             631        623        -0.2%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-logind.service            1,122           1,120      1,123      -0.1%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-rfkill.service            462             470        463        -0.2%          0.00s         0.00s  0.00s    +0.0%        
dict_talk.service                      160             154        160        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_telnet@.service                   113             114        113        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_virtlockd.service                 442             458        443        -0.2%          0.00s         0.00s  0.00s    +0.0%        
dict_virtlogd.service                  527             543        528        -0.2%          0.00s         0.00s  0.00s    +0.0%        
dict_virtqemud.service                 664             673        665        -0.2%          0.00s         0.00s  0.00s    +0.0%        
dict_virtsecretd.service               308             320        309        -0.3%          0.00s         0.00s  0.00s    +0.0%        
generated_cross_block_001m.bin         6,358           6,587      6,361      -0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_json_logs_001m.jsonl         58,767          59,118     58,770     -0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_repeated_text_001m.txt       208             220        211        -1.4%          0.00s         0.00s  0.00s    +0.0%        
generated_xorshift_001m.bin            1,048,610       1,048,614  1,048,613  -0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_.gitignore                        172             164        172        +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.lock                        9,240           8,088      9,241      -0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.toml                        68              68         68         +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_benchmark_zstd.py                 2,814           2,845      2,815      -0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_ci.yml                            556             555        557        -0.2%          0.00s         0.00s  0.00s    +0.0%        
repo_cli_Cargo.toml                    489             499        490        -0.2%          0.00s         0.00s  0.00s    +0.0%        
repo_compressed.rs                     12,946          13,007     12,949     -0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_main.rs                           2,125           2,142      2,126      -0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_match_generator.rs                26,651          27,414     26,654     -0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_prepare_benchmark_suites.py       2,553           2,607      2,554      -0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_progress.rs                       3,125           3,124      3,126      -0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_ruzstd_Cargo.toml                 730             726        731        -0.1%          0.00s         0.00s  0.00s    +0.0%        
repo_ruzstd_fuzz_.gitignore            21              21         21         +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_ruzstd_fuzz_Cargo.toml            340             347        341        -0.3%          0.00s         0.00s  0.00s    +0.0%        
```
