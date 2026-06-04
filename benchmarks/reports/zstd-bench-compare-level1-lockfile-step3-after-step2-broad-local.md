# zstd-rs Benchmark

Source CSV: `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-step3-after-step2.csv`

Commentary: Lockfile-specific probe step 3 vs retained probe step 2.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture          Upstream bytes  C bytes  New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------  --------------  -------  ---------  -------------  ------------  -----  -------  -------------
repo_Cargo.lock  9,170           8,088    9,223      -0.6%          0.00s         0.00s  0.00s    +0.0%        
```
