# zstd-rs Benchmark

Source CSV: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark.csv`

Commentary: current retained baseline

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
-------------------------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
build_libruzstd.rlib                   611,155         635,879    611,155    +0.0%          0.03s         0.00s  0.05s    -66.7%       
build_ruzstd-cli                       894,349         932,141    897,122    -0.3%          0.06s         0.00s  0.08s    -33.3%       
decodecorpus_z000003                   51,012          53,328     50,898     +0.2%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000028                   98,381          105,226    95,230     +3.2%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000030                   13,152          14,106     13,056     +0.7%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000031                   112             127        112        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000033                   532,632         571,529    530,433    +0.4%          0.02s         0.00s  0.04s    -100.0%      
decodecorpus_z000053                   304             299        304        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000054                   9,567           9,999      9,567      +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000059                   711             698        711        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000062                   13              13         13         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000077                   21              21         21         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000079                   7,321           7,221      7,322      -0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000080                   2,603           2,613      2,599      +0.2%          0.00s         0.00s  0.00s    +0.0%        
dict_canberra-system-bootup.service    316             317        307        +2.8%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin                    20,160          20,145     19,668     +2.4%          0.00s         0.00s  0.00s    +0.0%        
dict_git-daemon@.service               248             247        237        +4.4%          0.00s         0.00s  0.00s    +0.0%        
dict_glustereventsd.service            292             293        281        +3.8%          0.00s         0.00s  0.00s    +0.0%        
dict_gpm.service                       191             193        190        +0.5%          0.00s         0.00s  0.00s    +0.0%        
dict_quotaon.service                   420             420        412        +1.9%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-journal-gatewayd.service  630             631        622        +1.3%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-logind.service            1,122           1,120      1,122      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-rfkill.service            469             470        462        +1.5%          0.00s         0.00s  0.00s    +0.0%        
dict_talk.service                      160             154        151        +5.6%          0.00s         0.00s  0.00s    +0.0%        
dict_telnet@.service                   113             114        113        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_virtlockd.service                 450             458        442        +1.8%          0.00s         0.00s  0.00s    +0.0%        
dict_virtlogd.service                  535             543        527        +1.5%          0.00s         0.00s  0.00s    +0.0%        
dict_virtqemud.service                 672             673        664        +1.2%          0.00s         0.00s  0.00s    +0.0%        
dict_virtsecretd.service               316             320        308        +2.5%          0.00s         0.00s  0.00s    +0.0%        
generated_BUILD.bazel                  230             228        242        -5.2%          0.00s         0.00s  0.00s    +0.0%        
generated_Dockerfile                   211             217        211        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_Gemfile                      166             160        158        +4.8%          0.00s         0.00s  0.00s    +0.0%        
generated_Gemfile.lock                 248             252        239        +3.6%          0.00s         0.00s  0.00s    +0.0%        
generated_WORKSPACE                    210             208        201        +4.3%          0.00s         0.00s  0.00s    +0.0%        
generated_buf.yaml                     188             189        187        +0.5%          0.00s         0.00s  0.00s    +0.0%        
generated_composer.lock                4,476           3,766      4,112      +8.1%          0.00s         0.00s  0.00s    +0.0%        
generated_cross_block_001m.bin         6,358           6,587      6,358      +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_deno.json                    2,500           2,391      2,484      +0.6%          0.00s         0.00s  0.00s    +0.0%        
generated_go.mod                       168             168        164        +2.4%          0.00s         0.00s  0.00s    +0.0%        
generated_go.sum                       151             154        151        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_json_logs_001m.jsonl         58,767          59,118     58,767     +0.0%          0.00s         0.00s  0.01s    +0.0%        
generated_nx.json                      2,500           2,391      2,484      +0.6%          0.00s         0.00s  0.00s    +0.0%        
generated_package-lock.json            4,392           4,414      4,381      +0.3%          0.00s         0.00s  0.00s    +0.0%        
generated_package.json                 3,960           3,826      3,785      +4.4%          0.00s         0.00s  0.00s    +0.0%        
generated_pipfile.lock                 2,882           3,167      2,804      +2.7%          0.00s         0.00s  0.00s    +0.0%        
generated_poetry.lock                  371             362        358        +3.5%          0.00s         0.00s  0.00s    +0.0%        
generated_pom.xml                      198             195        196        +1.0%          0.00s         0.00s  0.00s    +0.0%        
generated_pubspec.lock                 225             227        229        -1.8%          0.00s         0.00s  0.00s    +0.0%        
generated_pubspec.yaml                 188             189        187        +0.5%          0.00s         0.00s  0.00s    +0.0%        
generated_pyproject.toml               268             270        261        +2.6%          0.00s         0.00s  0.00s    +0.0%        
generated_repeated_text_001m.txt       208             220        208        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_requirements.txt             189             193        189        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_tsconfig.json                2,500           2,391      2,484      +0.6%          0.00s         0.00s  0.00s    +0.0%        
generated_turbo.json                   3,960           3,826      3,785      +4.4%          0.00s         0.00s  0.00s    +0.0%        
generated_wrangler.toml                268             270        261        +2.6%          0.00s         0.00s  0.00s    +0.0%        
generated_xorshift_001m.bin            1,048,610       1,048,614  1,048,610  +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_yarn.lock                    398             393        383        +3.8%          0.00s         0.00s  0.00s    +0.0%        
repo_.gitignore                        172             164        166        +3.5%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.lock                        9,255           8,088      9,010      +2.6%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.toml                        68              68         68         +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_benchmark_zstd.py                 2,865           2,845      2,814      +1.8%          0.00s         0.00s  0.00s    +0.0%        
repo_ci.yml                            563             555        556        +1.2%          0.00s         0.00s  0.00s    +0.0%        
repo_cli_Cargo.toml                    497             499        489        +1.6%          0.00s         0.00s  0.00s    +0.0%        
repo_compressed.rs                     13,208          13,120     13,046     +1.2%          0.00s         0.00s  0.00s    +0.0%        
repo_main.rs                           2,128           2,142      2,125      +0.1%          0.00s         0.00s  0.00s    +0.0%        
repo_match_generator.rs                28,100          28,782     27,845     +0.9%          0.00s         0.00s  0.00s    +0.0%        
repo_prepare_benchmark_suites.py       7,267           7,081      6,827      +6.1%          0.00s         0.00s  0.00s    +0.0%        
repo_progress.rs                       3,168           3,124      3,125      +1.4%          0.00s         0.00s  0.00s    +0.0%        
repo_ruzstd_Cargo.toml                 737             726        730        +0.9%          0.00s         0.00s  0.00s    +0.0%        
repo_ruzstd_fuzz_.gitignore            21              21         21         +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_ruzstd_fuzz_Cargo.toml            348             347        340        +2.3%          0.00s         0.00s  0.00s    +0.0%        
```
