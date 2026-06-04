# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-fastest-binary-oldestfirst-repeat.csv`

Commentary: Level-1 binary-path repeat check: oldest-first window probing for Fastest only on non-text blocks, focused on the two broad-local binary fixtures that showed the only measurable CPU movement in the full run.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture               Upstream bytes  C bytes  New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
--------------------  --------------  -------  ---------  -------------  ------------  -----  -------  -------------
build_ruzstd-cli      867,739         894,099  867,739    +0.0%          0.04s         0.00s  0.04s    +0.0%        
decodecorpus_z000033  544,118         571,529  544,118    +0.0%          0.02s         0.00s  0.02s    +0.0%        
```
