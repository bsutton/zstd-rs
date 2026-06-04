# Benchmark Archive

This directory is the local archive for benchmark, profiling, and scratch
compression artifacts that were previously left in `benchmarks/tmp` or the
system `/tmp` directory.

The archive is intentionally treated as a local artifact store. The large
payload files are ignored by git; only this README and `MANIFEST.tsv` are meant
to be tracked.

## Layout

- `tmp/` contains the old repo-local `benchmarks/tmp` contents.
- `system-tmp/` contains benchmark reports and output directories moved from
  the system `/tmp`.
- `system-tmp/perf-data/` contains archived `*.perf.data` captures.
- `system-tmp/artifacts/` contains one-off compressed outputs, retained
  binaries, matcher logs, helper scripts, and focused fixture directories moved
  from `/tmp`.

## Policy

- Put new committed benchmark reports under `benchmarks/reports/`.
- Put reproducible fixture manifests under `benchmarks/manifests/`.
- Use `benchmarks/tmp/` only for disposable local output.
- If a temporary artifact is worth keeping but too large or too noisy for git,
  move it under this archive and refresh `MANIFEST.tsv`.
- Do not commit large `.zst`, binary, or `perf.data` payloads directly to git.

## Manifest

`MANIFEST.tsv` lists archived files as:

```text
relative_path<TAB>size_bytes
```

Refresh it with:

```sh
find benchmarks/archive -maxdepth 3 -type f -printf '%P\t%s\n' | sort > benchmarks/archive/MANIFEST.tsv
```
