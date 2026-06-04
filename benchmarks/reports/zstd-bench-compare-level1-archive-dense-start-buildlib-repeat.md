# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-archive-dense-start-buildlib-repeat.csv`

Commentary: Repeat check for the archive-like dense-start experiment on the build_libruzstd.rlib archive fixture.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture               Upstream bytes  C bytes  New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
--------------------  --------------  -------  ---------  -------------  ------------  -----  -------  -------------
build_libruzstd.rlib  611,155         635,879  600,329    +1.8%          0.03s         0.00s  0.04s    -33.3%       
```
