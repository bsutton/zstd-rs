# Multithreaded compression design and initial baseline

Status: merged for the initial `zstd-complete` release, 2026-08-30.

## Boundary

The first parallel mode schedules the bounded encoder's existing independent
frames across worker threads. Workers never share compressor state. Completed
frames carry monotonically increasing sequence numbers and are written in input
order, so scheduling does not affect archive contents.

The API is separate and opt-in behind the `multithreading` feature.
`ParallelEncoder` accepts a `NonZeroUsize` worker count. One worker contains and
delegates directly to the existing `Encoder`; two or more select the parallel
implementation. The existing `Encoder`, `encode`, and `encode_all` paths have
no queues, locks, atomics, worker-count checks, or thread lifecycle changes.

Prepared dictionary internals contain thread-local `Rc` state. They were not
changed to atomic shared ownership because that would alter the retained
single-thread representation. When the feature is enabled, `EncoderDictionary`
also retains its original bytes; each worker parses those bytes once into its
own local prepared state.

## Bounded scheduling

At most one frame per worker is counted as in flight. The caller stops
dispatching and consumes completed results when that bound is reached. Results
use an unbounded standard-library channel, but the in-flight invariant bounds
the number of messages to the worker count. Out-of-order frames are kept in an
ordered map until all earlier sequence numbers have arrived.

The aggregate estimate is the existing per-frame conservative estimate times
the worker count, plus the caller's current input buffer and retained main-thread
dictionary state. Construction fails before threads start if that exceeds the
configured memory limit.

A streaming run over the 51 MiB Silesia `mozilla` file, with a counting sink
that retains neither input nor output, measured 11,188 KiB peak process RSS at
one worker and 35,420 KiB at four workers. Both emitted 18,301,776 bytes. The
four-worker conservative encoder estimate was 74,514,432 bytes, so the measured
resident set remains both bounded and below the preflight limit.

`flush()` dispatches the current partial frame, waits for every earlier frame,
writes them in order, and then flushes the target. `finish()` does the same,
stops and joins all workers, and returns the target. Drop also stops and joins
workers, including after output failures. Worker compression is panic-contained
and reported as `EncodeError::WorkerFailed`.

## Performance guard

The pre-change control is commit `87629d1`. Control and candidate release
binaries were rebuilt with each worktree path remapped to the same `/zstd-rs`
source prefix. Their extracted ELF `.text` sections are byte-identical (SHA-256
`27ea52108b4192707c2cfda4442a8a302c84a93b85de0932e86334bddc748836`).
The default feature set therefore adds no executable instructions to the
single-threaded encoder.

The feature-enabled benchmark also compares `encode_all_parallel(..., 1)` with
direct `encode_all` in the same binary. Output was exact at levels 1, 3, 5, 8,
and 16. Median instruction changes were `+0.0116%`, `-0.0061%`, `+0.0074%`,
`+0.0050%`, and `-0.0134%`, respectively. The level-1 result used ten samples
of 200 repetitions; the remaining rows used 20 repetitions. These are neutral
counter movements. The all-input helper explicitly calls the direct encoder
when the worker count is one, while `ParallelEncoder<W>` retains its delegating
single-worker variant for incremental `Write` use.

The first five-shape smoke used the same 32 MiB inputs and 4 MiB frames, with
five fresh encoder runs per row. Output size was exact at every worker count.

| Input | 1 worker | 2 workers | 4 workers | 4-worker relative throughput |
|---|---:|---:|---:|---:|
| Deterministic random | 0.172 s | 0.151 s | 0.134 s | 1.28x |
| Long-distance | 0.119 s | 0.098 s | 0.099 s | 1.20x |
| Repeated records | 0.029 s | 0.054 s | 0.059 s | 0.49x |
| Structured JSONL | 0.210 s | 0.171 s | 0.120 s | 1.75x |
| Zeros | 0.099 s | 0.082 s | 0.067 s | 1.48x |

The repeated-record case is so cheap that creating fresh worker threads for
each 32 MiB encoder costs more than compression. Parallel mode is therefore not
an unconditional win; callers should retain one worker for latency-sensitive
or extremely cheap inputs. Persistent pool reuse or a justified adaptive gate
is follow-up work, and must not add checks to the default encoder.

Silesia scaling compressed all 12 files individually with 4 MiB frames, three
compression repetitions per file, and three complete `perf stat` samples. The
ordering tests separately require byte-exact output across worker counts.

| Level | 1-worker wall | 2-worker wall / speedup | 4-worker wall / speedup | 1/2/4-worker instructions |
|---:|---:|---:|---:|---:|
| 1 | 2.056 s | 1.455 s / 1.41x | 1.161 s / 1.77x | 17.777 / 18.248 / 18.374 B |
| 3 | 2.984 s | 1.954 s / 1.53x | 1.615 s / 1.85x | 21.506 / 22.012 / 22.205 B |
| 8 | 10.350 s | 6.173 s / 1.68x | 4.609 s / 2.25x | 101.434 / 102.208 / 102.544 B |

The table includes fresh thread creation for each file and repetition. As
expected, scaling improves when each frame contains more compression work.
Very cheap inputs can still lose to thread startup: the 32 MiB repeated-record
fixture measured 0.029/0.054/0.059 seconds at one/two/four workers. Parallel
mode is opt-in rather than an unconditional default.

The matched zstd C 1.5.7 comparison used `ZSTD_c_nbWorkers` and 4 MiB jobs. C
emits one frame per file with overlapped jobs, whereas Rust emits independent
4 MiB frames, so this is a product-level comparison rather than identical
framing.

| Workers | Rust wall | C wall | Rust gap | Rust instructions | C instructions |
|---:|---:|---:|---:|---:|---:|
| 1 | 2.984 s | 2.806 s | +6.3% | 21.506 B | 20.535 B |
| 2 | 1.954 s | 1.844 s | +6.0% | 22.012 B | 20.658 B |
| 4 | 1.615 s | 1.517 s | +6.5% | 22.205 B | 20.796 B |

C produced 66,264,405 bytes per corpus repetition and Rust produced
66,356,499 bytes, a `+0.139%` Rust size gap under the different framing models.
On the 32 MiB structured fixture, Rust was 0.14% smaller than C but C remained
faster at every worker count. The comparison harness builds zstd-safe with its
explicit `zstdmt` feature and is separate from normal package builds.

Commands:

```text
target/release/generate_release_corpus /tmp/zstd-rs-parallel-corpus 32
cargo build --release -p zstd-rs-tools --features multithreading --bin profile_parallel_api
target/release/profile_parallel_api /tmp/zstd-rs-parallel-corpus/structured-json.jsonl 3 10 4 512 WORKERS
cargo build --release -p zstd-rs-tools --features c-multithreading --bin profile_c_parallel_api
target/release/profile_c_parallel_api /tmp/zstd-rs-parallel-corpus/structured-json.jsonl 3 10 4 WORKERS
```

For `profile_parallel_api`, worker count `0` selects the direct `encode_all`
control in the same feature-enabled binary; `1` selects the one-worker helper,
which calls that direct path.

## Verification matrix

The final local gate covered:

- 768 feature-enabled library tests, five ignored diagnostics, and eight
  doctests;
- focused no-default-feature and forced-scalar parallel suites;
- all 117 feature combinations under `cargo hack check` and strict Clippy;
- Rust 1.87 MSRV and Windows x86-64, macOS AArch64, Linux AArch64, and wasm32
  cross-checks with `multithreading` enabled;
- the worker-panic lifecycle and an end-to-end two-worker frame under Miri;
- all ten parallel tests under AddressSanitizer;
- warnings-denied rustdoc, formatting, and package verification; and
- a consumer-style `cargo check --features multithreading` from the unpacked
  publishable crate.

The packaged crate intentionally excludes maintainer corpus fixtures, so its
internal `cfg(test)` build cannot resolve their `include_bytes!` paths. Normal
package verification and the feature-enabled consumer build both pass.
