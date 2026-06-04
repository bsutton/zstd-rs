# zstd-rs Benchmark

Source CSV: `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-oflog8.csv`

Commentary: Lockfile-only offset_table_max_log 8 on top of the retained second_newest gate fix.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture          Upstream bytes  C bytes  New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------  --------------  -------  ---------  -------------  ------------  -----  -------  -------------
repo_Cargo.lock  9,185           8,088    9,185      +0.0%          0.00s         0.00s  0.00s    +0.0%        
```
