# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-lockfile-nextwindow-broad-local.csv`

Commentary: Lockfile-specific next-position window lookahead on top of retained step-2 baseline.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture          Upstream bytes  C bytes  New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------  --------------  -------  ---------  -------------  ------------  -----  -------  -------------
repo_Cargo.lock  9,170           8,088    9,170      +0.0%          0.00s         0.00s  0.00s    +0.0%        
```
