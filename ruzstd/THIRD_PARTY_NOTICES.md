# Third-Party Notices

Portions of this package's Rust compressor were derived from and translated
from the Zstandard 1.5.7 compressor implementation, principally `lib/compress/`
at commit `ac66b19e6bd6b83238bf008eecc1298105298532`.

Zstandard offers those sources under either its BSD license or GPLv2. This
package selects the BSD option. The required copyright notice, conditions, and
disclaimer are in `LICENSE-BSD-3-Clause`.

The implementation is not represented as clean-room. Automated translation,
analysis, and code-generation do not alter its provenance. Derived areas
include `src/encoding/levels/c_port/`, compressor entropy/table-building code,
and the private generated kernels under `src/kernel/`. The detailed source map
is maintained in `src/encoding/levels/c_port/README.md`.

Existing independently authored ruzstd code remains covered by `LICENSE-MIT`.
The package as a whole is distributed under `MIT AND BSD-3-Clause`.
