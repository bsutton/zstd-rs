# zstd-rs Benchmark

Source CSV: `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-step2-restore-after-step3-rebuilt.csv`

Commentary: Restore check after rejecting lockfile probe step 3, using rebuilt source.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture          Upstream bytes  C bytes  New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
---------------  --------------  -------  ---------  -------------  ------------  -----  -------  -------------
repo_Cargo.lock  9,170           8,088    9,170      +0.0%          0.00s         0.00s  0.00s    +0.0%        
```
