# zstd-rs Benchmark

Source CSV: `benchmarks/reports/zstd-bench-compare-level1-fastest-binary-nextrep-repeat.csv`

Commentary: Level-1 binary-path repeat check: Fastest non-text ip+1 repeat lookahead, focused on the four broad-local binary fixtures with the largest compression or CPU movement in the full run.

Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.

```text
Fixture               Upstream bytes  C bytes  New bytes  % Improvement  Upstream CPU  C CPU  New CPU  % Improvement
--------------------  --------------  -------  ---------  -------------  ------------  -----  -------  -------------
build_libruzstd.rlib  619,650         635,879  611,155    +1.4%          0.03s         0.00s  0.03s    +0.0%        
build_ruzstd-cli      867,739         894,099  860,072    +0.9%          0.04s         0.00s  0.05s    -25.0%       
decodecorpus_z000033  544,118         571,529  544,266    -0.0%          0.02s         0.00s  0.02s    +0.0%        
decodecorpus_z000079  8,372           7,221    7,540      +9.9%          0.00s         0.00s  0.00s    +0.0%        
```
