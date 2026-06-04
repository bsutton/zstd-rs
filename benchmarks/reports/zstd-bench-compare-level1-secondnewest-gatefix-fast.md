# zstd-rs Benchmark

Source CSV: `benchmarks/archive/tmp/fastguard-gatefix2-seq.csv`

Commentary: Gate second_newest probes by should_track_second_newest_for_current_entry() and cache lockfile classification per block.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture                         Upstream bytes  C bytes  New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
------------------------------  --------------  -------  ---------  -------------  ------------  -----  -------  -------------
build_ruzstd-cli                862,752         906,038  854,529    +1.0%          0.06s         0.00s  0.06s    +0.0%        
decodecorpus_z000079            7,321           7,221    7,322      -0.0%          0.00s         0.00s  0.00s    +0.0%        
dict_dictionary.bin             20,160          20,145   20,160     +0.0%          0.57s         0.00s  0.00s    +100.0%      
generated_json_logs_001m.jsonl  58,767          59,118   58,767     +0.0%          0.00s         0.00s  0.00s    +0.0%        
repo_Cargo.lock                 9,197           8,088    9,185      +0.1%          0.01s         0.00s  0.00s    +100.0%      
```
