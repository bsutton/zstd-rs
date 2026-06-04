# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-single-segment-broad-local.csv`

Commentary: single-segment headers when content size is known

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
-------------------------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
build_libruzstd.rlib                   611,155         635,879    611,158    -0.0%          0.04s         0.00s  0.04s    +0.0%        
build_ruzstd-cli                       897,122         932,141    897,125    -0.0%          0.07s         0.00s  0.06s    +14.3%       
decodecorpus_z000003                   50,898          53,328     50,901     -0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000028                   95,230          105,226    95,233     -0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000030                   13,056          14,106     13,057     -0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000031                   112             127        112        +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000033                   530,433         571,529    530,436    -0.0%          0.03s         0.00s  0.04s    -33.3%       
decodecorpus_z000053                   304             299        305        -0.3%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000054                   9,567           9,999      9,568      -0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000059                   711             698        712        -0.1%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000062                   13              13         13         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000077                   21              21         21         +0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000079                   7,322           7,221      7,325      -0.0%          0.00s         0.00s  0.00s    +0.0%        
decodecorpus_z000080                   2,599           2,613      2,600      -0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_canberra-system-bootup.service    307             317        308        -0.3%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin                    19,668          20,145     19,669     -0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_git-daemon@.service               237             247        238        -0.4%          0.00s         0.00s  0.00s    +0.0%        
dict_glustereventsd.service            281             293        282        -0.4%          0.00s         0.00s  0.00s    +0.0%        
dict_gpm.service                       190             193        191        -0.5%          0.00s         0.00s  0.00s    +0.0%        
dict_quotaon.service                   412             420        413        -0.2%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-journal-gatewayd.service  622             631        623        -0.2%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-logind.service            1,122           1,120      1,123      -0.1%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-rfkill.service            462             470        463        -0.2%          0.00s         0.00s  0.00s    +0.0%        
dict_talk.service                      151             154        151        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_telnet@.service                   113             114        113        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_virtlockd.service                 442             458        443        -0.2%          0.00s         0.00s  0.00s    +0.0%        
dict_virtlogd.service                  527             543        528        -0.2%          0.00s         0.00s  0.00s    +0.0%        
dict_virtqemud.service                 664             673        665        -0.2%          0.00s         0.00s  0.00s    +0.0%        
dict_virtsecretd.service               308             320        309        -0.3%          0.00s         0.00s  0.00s    +0.0%        
generated_BUILD.bazel                  242             228        245        -1.2%          0.00s         0.00s  0.00s    +0.0%        
generated_Dockerfile                   211             217        214        -1.4%          0.00s         0.00s  0.00s    +0.0%        
generated_Gemfile                      158             160        161        -1.9%          0.00s         0.00s  0.00s    +0.0%        
generated_Gemfile.lock                 240             252        243        -1.2%          0.00s         0.00s  0.00s    +0.0%        
generated_WORKSPACE                    201             208        204        -1.5%          0.00s         0.00s  0.00s    +0.0%        
generated_buf.yaml                     187             189        190        -1.6%          0.00s         0.00s  0.00s    +0.0%        
generated_composer.lock                4,160           3,766      4,163      -0.1%          0.00s         0.00s  0.00s    +0.0%        
generated_cross_block_001m.bin         6,358           6,587      6,361      -0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_deno.json                    2,484           2,391      2,487      -0.1%          0.00s         0.00s  0.00s    +0.0%        
generated_go.mod                       164             168        167        -1.8%          0.00s         0.00s  0.00s    +0.0%        
generated_go.sum                       151             154        154        -2.0%          0.00s         0.00s  0.00s    +0.0%        
generated_json_logs_001m.jsonl         58,767          59,118     58,770     -0.0%          0.01s         0.00s  0.01s    +0.0%        
generated_nx.json                      2,484           2,391      2,487      -0.1%          0.00s         0.00s  0.00s    +0.0%        
generated_package-lock.json            4,383           4,414      4,386      -0.1%          0.00s         0.00s  0.00s    +0.0%        
generated_package.json                 3,785           3,826      3,788      -0.1%          0.00s         0.00s  0.00s    +0.0%        
generated_pipfile.lock                 2,804           3,167      2,807      -0.1%          0.00s         0.00s  0.00s    +0.0%        
generated_poetry.lock                  359             362        362        -0.8%          0.00s         0.00s  0.00s    +0.0%        
generated_pom.xml                      196             195        199        -1.5%          0.00s         0.00s  0.00s    +0.0%        
generated_pubspec.lock                 233             227        236        -1.3%          0.00s         0.00s  0.00s    +0.0%        
generated_pubspec.yaml                 187             189        190        -1.6%          0.00s         0.00s  0.00s    +0.0%        
generated_pyproject.toml               261             270        264        -1.1%          0.00s         0.00s  0.00s    +0.0%        
generated_repeated_text_001m.txt       208             220        211        -1.4%          0.00s         0.00s  0.00s    +0.0%        
generated_requirements.txt             189             193        192        -1.6%          0.00s         0.00s  0.00s    +0.0%        
generated_tsconfig.json                2,484           2,391      2,487      -0.1%          0.00s         0.00s  0.00s    +0.0%        
generated_turbo.json                   3,785           3,826      3,788      -0.1%          0.00s         0.00s  0.00s    +0.0%        
generated_wrangler.toml                261             270        264        -1.1%          0.00s         0.00s  0.00s    +0.0%        
generated_xorshift_001m.bin            1,048,610       1,048,614  1,048,613  -0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_yarn.lock                    383             393        386        -0.8%          0.00s         0.00s  0.00s    +0.0%        
repo_.gitignore                        166             164        166        +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.lock                        9,109           8,088      9,110      -0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.toml                        68              68         68         +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_benchmark_zstd.py                 2,814           2,845      2,815      -0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_ci.yml                            556             555        557        -0.2%          0.00s         0.00s  0.00s    +0.0%        
repo_cli_Cargo.toml                    489             499        490        -0.2%          0.00s         0.00s  0.00s    +0.0%        
repo_compressed.rs                     13,046          13,120     13,049     -0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_main.rs                           2,125           2,142      2,126      -0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_match_generator.rs                27,845          28,782     27,848     -0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_prepare_benchmark_suites.py       6,827           7,081      6,828      -0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_progress.rs                       3,125           3,124      3,126      -0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_ruzstd_Cargo.toml                 730             726        731        -0.1%          0.00s         0.00s  0.00s    +0.0%        
repo_ruzstd_fuzz_.gitignore            21              21         21         +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_ruzstd_fuzz_Cargo.toml            340             347        341        -0.3%          0.00s         0.00s  0.00s    +0.0%        
```
