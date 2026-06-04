# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-code-secondnewest-broad-local.csv`

Commentary: CodeText short-line current-entry second_newest for 16-64KiB blocks

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                                Upstream bytes  C bytes    New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
-------------------------------------  --------------  ---------  ---------  -------------  ------------  -----  -------  -------------
build_libruzstd.rlib                   611,155         635,879    611,155    +0.0%          0.03s         0.00s  0.04s    -33.3%       
build_ruzstd-cli                       897,122         932,141    897,122    +0.0%          0.05s         0.00s  0.06s    -20.0%       
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
dict_dictionary.bin                    19,668          20,145     19,668     +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_git-daemon@.service               237             247        237        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_glustereventsd.service            281             293        281        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_gpm.service                       190             193        190        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_quotaon.service                   412             420        412        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-journal-gatewayd.service  622             631        622        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-logind.service            1,122           1,120      1,122      +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_systemd-rfkill.service            462             470        462        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_talk.service                      151             154        151        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_telnet@.service                   113             114        113        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_virtlockd.service                 442             458        442        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_virtlogd.service                  527             543        527        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_virtqemud.service                 664             673        664        +0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_virtsecretd.service               308             320        308        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_BUILD.bazel                  242             228        242        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_Dockerfile                   211             217        211        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_Gemfile                      158             160        158        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_Gemfile.lock                 240             252        240        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_WORKSPACE                    201             208        201        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_buf.yaml                     187             189        187        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_composer.lock                4,336           3,766      4,336      +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_cross_block_001m.bin         6,358           6,587      6,358      +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_deno.json                    2,492           2,391      2,492      +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_go.mod                       164             168        164        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_go.sum                       151             154        151        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_json_logs_001m.jsonl         58,767          59,118     58,767     +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_nx.json                      2,492           2,391      2,492      +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_package-lock.json            4,392           4,414      4,392      +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_package.json                 3,956           3,826      3,956      +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_pipfile.lock                 2,811           3,167      2,811      +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_poetry.lock                  359             362        359        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_pom.xml                      196             195        196        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_pubspec.lock                 233             227        233        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_pubspec.yaml                 187             189        187        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_pyproject.toml               261             270        261        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_repeated_text_001m.txt       208             220        208        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_requirements.txt             189             193        189        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_tsconfig.json                2,492           2,391      2,492      +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_turbo.json                   3,956           3,826      3,956      +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_wrangler.toml                261             270        261        +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_xorshift_001m.bin            1,048,610       1,048,614  1,048,610  +0.0%          0.00s         0.00s  0.00s    +0.0%        
generated_yarn.lock                    383             393        383        +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_.gitignore                        166             164        166        +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.lock                        9,114           8,088      9,114      +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.toml                        68              68         68         +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_benchmark_zstd.py                 2,814           2,845      2,814      +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_ci.yml                            556             555        556        +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_cli_Cargo.toml                    489             499        489        +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_compressed.rs                     13,046          13,120     13,046     +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_main.rs                           2,125           2,142      2,125      +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_match_generator.rs                28,078          28,782     27,845     +0.8%          0.00s         0.00s  0.00s    +0.0%        
repo_prepare_benchmark_suites.py       7,221           7,081      6,827      +5.5%          0.00s         0.00s  0.00s    +0.0%        
repo_progress.rs                       3,125           3,124      3,125      +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_ruzstd_Cargo.toml                 730             726        730        +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_ruzstd_fuzz_.gitignore            21              21         21         +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_ruzstd_fuzz_Cargo.toml            340             347        340        +0.0%          0.00s         0.00s  0.00s    +0.0%        
```
