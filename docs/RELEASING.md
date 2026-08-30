# Releasing `zstd-complete`

This checklist is intentionally separate from optimization work. Publishing a
crate version is irreversible, and an unregistered name is not reserved by a
dry-run.

## 1. Prepare the release commit

1. Confirm `ruzstd/Cargo.toml` has package `zstd-complete`, the intended
   version, `BSD-3-Clause`, and the `bsutton/zstd-rs` repository URLs.
2. Replace `unreleased` in `Changelog.md` with the release date.
3. Review `Readme.md`, `LICENSE`, `THIRD_PARTY_NOTICES.md`, and the inherited
   ruzstd notice in `LICENSES/ruzstd-MIT.txt`. Keep the no-human-review
   disclosure and benchmark limitations.
4. Commit the complete implementation and release metadata. Do not publish
   from the current dirty development worktree.

## 2. Require green hosted CI

Push the release commit and require every job in `.github/workflows/ci.yml`:

- stable tests, feature-power-set checks, Clippy, and MSRV;
- nightly formatting, Clippy, and the four focused Miri tests;
- native Windows x86-64 and macOS Apple-Silicon tests;
- Linux AArch64 and wasm `no_std` checks;
- forced-scalar tests, package/license audit, and AddressSanitizer.

Do not substitute cross-build success for the two native test jobs.

## 3. Recreate and inspect the exact crate

From a clean release commit:

```sh
cargo fmt --all -- --check
cargo clippy -p zstd-complete --all-targets -- -D warnings
cargo test -p zstd-complete --lib
cargo test -p zstd-complete --doc
cargo package -p zstd-complete --locked
cargo publish -p zstd-complete --locked --dry-run
```

Inspect `target/package/zstd-complete-0.1.0.crate` and require
`LICENSE`, `LICENSES/ruzstd-MIT.txt`, `THIRD_PARTY_NOTICES.md`, and `Readme.md`.
Run `gzip -t` before recording the final SHA-256. If an old local
artifact has trailing bytes, move that generated artifact aside and rerun
`cargo package`; never upload the stale file.

## 4. Publish only with explicit approval

After confirming the crates.io account/token and while the name remains
available:

```sh
cargo publish -p zstd-complete --locked
```

This is the only step that uploads. It must not be run by inference from a
request to prepare the release.

## 5. Verify and tag

Wait for the crate and docs to become available, then compile a fresh external
consumer using `zstd-complete = "0.1.0"` and verify a Rust/C round trip. Tag the
exact published commit as `zstd-complete-v0.1.0` and push the tag. If the
published package has a defect, publish a new patch version; crates.io versions
cannot be replaced.
