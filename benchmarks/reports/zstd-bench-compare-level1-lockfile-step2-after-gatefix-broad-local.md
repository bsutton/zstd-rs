# zstd-rs Benchmark

Source CSV: `benchmarks/archive/tmp/broad-local-lockfile-step2-after-gatefix.csv`

Commentary: Lockfile-specific probe step 2 on top of the retained second_newest gate-fix baseline.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
-------------------------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
build_libruzstd.rlib                   611,155         635,879    611,155    +0.0%          0.03s         0.00s  0.03s    +0.0%        
build_ruzstd-cli                       854,529         906,038    854,529    +0.0%          0.07s         0.00s  0.07s    +0.0%        
decodecorpus_z000003                   50,898          53,328     50,898     +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000028                   95,230          105,226    95,230     +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000030                   13,056          14,106     13,056     +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000031                   112             127        112        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000033                   530,433         571,529    530,433    +0.0%          0.03s         0.00s  0.03s    +0.0%        
decodecorpus_z000053                   304             299        304        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000054                   9,567           9,999      9,567      +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000059                   711             698        711        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000062                   13              13         13         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000077                   21              21         21         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000079                   7,322           7,221      7,322      +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000080                   2,599           2,613      2,599      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_canberra-system-bootup.service    307             317        307        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin                    20,160          20,145     20,160     +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_git-daemon@.service               241             247        241        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_glustereventsd.service            285             293        285        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_gpm.service                       191             193        191        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_quotaon.service                   412             420        412        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-journal-gatewayd.service  622             631        622        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-logind.service            1,122           1,120      1,122      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-rfkill.service            462             470        462        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_talk.service                      160             154        160        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_telnet@.service                   113             114        113        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_virtlockd.service                 442             458        442        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_virtlogd.service                  527             543        527        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_virtqemud.service                 664             673        664        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_virtsecretd.service               308             320        308        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_cross_block_001m.bin         6,358           6,587      6,358      +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_json_logs_001m.jsonl         58,767          59,118     58,767     +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_repeated_text_001m.txt       208             220        208        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_xorshift_001m.bin            1,048,610       1,048,614  1,048,610  +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_.gitignore                        172             164        172        +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.lock                        9,185           8,088      9,170      +0.2%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.toml                        68              68         68         +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_benchmark_zstd.py                 2,814           2,845      2,814      +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_ci.yml                            556             555        556        +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_cli_Cargo.toml                    489             499        489        +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_compressed.rs                     12,946          13,007     12,946     +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_main.rs                           2,125           2,142      2,125      +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_match_generator.rs                26,651          27,414     26,651     +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_prepare_benchmark_suites.py       2,553           2,607      2,553      +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_progress.rs                       3,125           3,124      3,125      +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_ruzstd_Cargo.toml                 730             726        730        +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_ruzstd_fuzz_.gitignore            21              21         21         +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_ruzstd_fuzz_Cargo.toml            340             347        340        +0.0%          0.00s         0.00s  0.00s    +0.0%        
```
