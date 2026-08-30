# Multithreaded compression design and initial baseline

Status: issue #6 implementation branch, 2026-08-30.

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

A four-worker, 4 MiB-frame run on the 32 MiB deterministic-random fixture
reported a 74,514,432-byte encoder estimate and 109,204 KiB process peak RSS.
The process figure also includes the benchmark's separately resident 32 MiB
source and returned 32 MiB incompressible output; those caller-owned buffers are
outside the encoder budget.

`flush()` dispatches the current partial frame, waits for every earlier frame,
writes them in order, and then flushes the target. `finish()` does the same,
stops and joins all workers, and returns the target. Drop also stops and joins
workers, including after output failures. Worker compression is panic-contained
and reported as `EncodeError::WorkerFailed`.

## Initial performance guard

The pre-change release baseline was built from commit `87629d1` with the
workspace's one-codegen-unit release profile. The direct-path control used the
1,022,035-byte `corpus_z000033` fixture, level 3, 8 MiB frames, 20 repetitions,
and ten `perf stat` samples on the same host.

| Path | Output bytes/run | Instructions | Cycles | Branches | Branch misses |
|---|---:|---:|---:|---:|---:|
| Pre-change `encode_all` | 498,891 | 940,284,789 | 511,681,643 | 135,641,931 | 6,150,105 |
| Candidate default `encode_all` | 498,891 | 940,341,528 | 510,808,177 | 135,657,838 | 6,170,889 |

The default-path instruction change is `+0.0060%`, output is exact, cycles are
`-0.17%`, and branches are `+0.0117%`. These are neutral same-session changes
and the default encoder's compiled path contains none of the parallel
orchestration.

A separate five-sample check compared the pre-change direct encoder with the
candidate `ParallelEncoder` configured for one worker on the deterministic 32
MiB structured-JSON fixture, level 3, 4 MiB frames, and 10 repetitions. Output
was exact; instructions were `4,606,168,105`/`4,606,943,012`, a `+0.0168%`
change. The one-worker wrapper therefore also remains instruction-neutral.

A preliminary elapsed-time run on the same 32 MiB fixture with 4 MiB frames and
10 repetitions measured 0.428/0.350/0.238 seconds at 1/2/4 workers. This is
about 1.22x and 1.80x relative throughput for 2 and 4 workers. It is an early
single-fixture result, not a general scaling claim. Silesia and zstd C
`nbWorkers` comparisons remain before the complete issue benchmark gate.

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

The first Silesia run compressed all 12 files individually at level 3, with
4 MiB frames, three compression repetitions per file, and three complete
`perf stat` samples. Every worker count produced the same 66,356,499 bytes per
corpus repetition.

| Workers | Wall time | Relative throughput | CPU task time | Instructions | Cycles |
|---:|---:|---:|---:|---:|---:|
| 1 | 2.984 s | 1.00x | 2.969 s | 21.506 B | 12.485 B |
| 2 | 1.954 s | 1.53x | 3.453 s | 22.012 B | 13.165 B |
| 4 | 1.615 s | 1.85x | 3.687 s | 22.205 B | 13.720 B |

Parallel scheduling adds about 2.35%/3.25% aggregate instructions at two/four
workers while reducing elapsed time. The table includes fresh thread creation
for each file and each repetition.

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
