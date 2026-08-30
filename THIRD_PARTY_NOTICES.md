# Third-Party Notices

## Zstandard compressor algorithms

Portions of the Rust compressor were derived from and translated from the
Zstandard 1.5.7 compressor implementation. The principal reference sources are
the files under `lib/compress/` in Zstandard commit
`ac66b19e6bd6b83238bf008eecc1298105298532`.

Zstandard offers those sources under either the BSD-style license or GPLv2.
For this project and its binary and source distributions, the BSD-style option
is selected. Its notice, conditions, and disclaimer form the package's
`BSD-3-Clause` license in `LICENSE`.

The Rust implementation is not represented as a clean-room implementation.
Use of automated translation, analysis, or code-generation tools does not
alter the provenance or licensing of the translated portions.

The detailed source map is maintained in
`ruzstd/src/encoding/levels/c_port/README.md`. The areas containing derived
compressor work include that module, the compressor entropy/table-building
paths it calls, and the private generated kernels under `ruzstd/src/kernel/`.
This repository is a fork of Moritz Borcherding's
[`KillingSpark/zstd-rs`](https://github.com/KillingSpark/zstd-rs) (`ruzstd`),
developed with its contributors. Substantial pre-existing Rust remains,
particularly the decoder, frame parsing, ring buffer, bit I/O, FSE/Huffman
decoding, dictionaries, and `no_std` I/O abstractions. Its copyright and MIT
permission notice are retained in `LICENSES/ruzstd-MIT.txt`. That required
attribution is not a second outbound license choice for `zstd-complete`, which
is distributed under `BSD-3-Clause`.
