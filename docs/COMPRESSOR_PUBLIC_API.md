# Compressor public API target

This is the publication contract and records which parts are implemented.
Temporary numeric C-port entry points remain benchmark/compatibility hooks and
are not the recommended application API.

## Primary API

The standard-library API is centered on an `Encoder<W>` that implements
`std::io::Write`. It accepts arbitrarily sized streams, emits output
incrementally, and returns the underlying writer from
`finish(self) -> Result<W, EncodeError>`. Dropping an unfinished encoder must
not pretend that a complete frame was written.

A convenience function should provide:

```rust,ignore
pub fn encode<R: std::io::Read, W: std::io::Write>(
    source: R,
    target: W,
    options: EncoderOptions,
) -> Result<(), EncodeError>;
```

`encode_all(R, EncoderOptions) -> Result<Vec<u8>, EncodeError>` is useful for
small in-memory inputs, but is not the primary interface. The `no_std`
compressor continues to accept bounded input slices. A caller-provided-output
API remains possible follow-up work and is not required by the `std::io`
streaming contract.

The current bounded implementation emits independent frames at configured
input chunk boundaries. Concatenated frames are part of the Zstandard archive
format. This gives a simple, auditable memory bound and makes `flush()` close a
meaningful unit, at the cost of losing matches across frame boundaries.

## Configuration

`CompressionLevel` is a validated Rust type with named constants such as
`FASTEST`, `DEFAULT`, and `MAXIMUM`. A fallible numeric constructor and
`TryFrom<i32>` exist for interoperability, but numeric C-level functions do
not belong in the normal public API.

`EncoderOptions` owns level, checksum selection, an optional prepared
dictionary, frame chunk size, and an explicit memory budget. A pledged source
size is not needed for independently sized frames. Target compressed block
size remains an internal validation/tuning control unless an independent user
case demonstrates that it belongs in the stable API.

`EncoderDictionary` owns validated dictionary state and is reusable across
frames. Dictionary parse errors are distinct from I/O and memory-budget errors.

## Errors

All public I/O paths return `Result`; no read, write, configuration, dictionary,
allocation-policy, or finalization failure is unwrapped. `EncodeError` retains
the underlying `std::io::Error` where applicable and has explicit variants for
invalid options and `MemoryLimitExceeded`. `DictionaryError` reports failures
while preparing a dictionary before the encoder is constructed.

## Memory contract

The default streaming configuration should normally remain within roughly
50-100 MiB RSS. This cannot be promised for every level: zstd level 22's
tables alone can require several hundred MiB. Before allocation, the encoder
computes the required window/table/scratch budget and either stays within the
configured limit or returns `MemoryLimitExceeded`; selecting an extreme level
is an explicit opt-in to its larger requirement.

The implementation retains one bounded input frame, one bounded output frame,
matcher/entropy tables, and reusable scratch. Release tests include a 64 MiB
generated source with a 1 MiB frame chunk and verify that input allocation does
not grow with total stream length. Peak-RSS profiling is a release gate rather
than a promise that the conservative estimate exactly equals allocator RSS.

## Compatibility and maintenance

The API promises Zstandard format interoperability, not byte-identical output
or permanent alignment with C internals. The zstd C implementation is the
disclosed source and performance reference for this work. Future design and
optimization should use Rust-owned objects and invariants, may diverge when
measured improvements justify it, and do not require recurring C-source audits.
