# Benchmark History

Newest results go at the top. Each entry records the change that produced the result, the benchmark tables, and the checked-in source report files under `benchmarks/reports/`.

All retained tables below were validated by decoding the produced output with C `zstd` and byte-comparing against the original fixture.

## 2026-06-01 - Retained Cargo.lock current-entry thirteenth-newest sidecar

Change notes:
- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added a thirteenth current-entry recent sidecar for lockfile-profiled text
  - probe the new thirteenth-newest candidate after the retained twelfth-newest path
- Added focused matcher coverage:
  - `lockfile_sidecar_tracks_thirteenth_newest_for_current_entry`

Retained:
- focused result:
  - `repo_Cargo.lock`: `9,012 -> 9,010`
  - unchanged controls:
    - `generated_go.sum`: `151`
    - `generated_poetry.lock`: `358`
    - `generated_yarn.lock`: `383`
    - `generated_composer.lock`: `4,112`
    - `generated_package-lock.json`: `4,381`
    - `generated_pipfile.lock`: `2,804`
- broad-local result:
  - only `repo_Cargo.lock` moved

Current broad-local vs C `zstd -1`:
- `71` fixtures
- `51 / 16 / 4` better / worse / equal
- `1,693` bytes above C on losing fixtures

Current biggest losses:
- `repo_Cargo.lock`: `9,010` vs `8,088` = `+922`
- `generated_composer.lock`: `4,112` vs `3,766` = `+346`
- `decodecorpus_z000079`: `7,322` vs `7,221` = `+101`
- `generated_tsconfig.json`: `2,484` vs `2,391` = `+93`
- `generated_deno.json`: `2,484` vs `2,391` = `+93`
- `generated_nx.json`: `2,484` vs `2,391` = `+93`

Useful signal:
- the current-entry recency family remains broad-safe and continues to move the dominant
  `Cargo.lock` target
- `Cargo.lock` is now below a `+923` byte gap to C on the retained baseline

Artifacts:
- [alternative lazy-parse sweep](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-cargo-lock-next-position-loss-posttwelfth.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargo-thirteenthnewest-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-cargo-thirteenthnewest-retained)

Verification passed:
- `cargo fmt --all --check`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q -p ruzstd lockfile_sidecar_tracks_thirteenth_newest_for_current_entry -- --nocapture`
- `cargo test -q --workspace`
- `cargo build --release -q -p ruzstd-cli`
- `python3 -m py_compile tools/tune_matcher_family.py tools/benchmark_zstd.py tools/prepare_benchmark_suites.py`

Useful conclusion:
- the alternative `Cargo.lock` lazy-parse surface stayed flat on the current baseline
- the recency family is still the only live `Cargo.lock` family in local search
- the next turn should decide whether to try one final recency slot or pivot to a broader
  non-local representation branch

## 2026-06-01 - Retained Cargo.lock current-entry twelfth-newest sidecar

Change notes:
- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added a twelfth current-entry recent sidecar for lockfile-profiled text
  - probe the new twelfth-newest candidate after the retained eleventh-newest path
- Added focused matcher coverage:
  - `lockfile_sidecar_tracks_twelfth_newest_for_current_entry`

Retained:
- focused result:
  - `repo_Cargo.lock`: `9,014 -> 9,012`
  - unchanged controls:
    - `generated_go.sum`: `151`
    - `generated_poetry.lock`: `358`
    - `generated_yarn.lock`: `383`
    - `generated_composer.lock`: `4,112`
    - `generated_package-lock.json`: `4,381`
    - `generated_pipfile.lock`: `2,804`
- broad-local result:
  - only `repo_Cargo.lock` moved

Current broad-local vs C `zstd -1`:
- `71` fixtures
- `51 / 16 / 4` better / worse / equal
- `1,695` bytes above C on losing fixtures

Current biggest losses:
- `repo_Cargo.lock`: `9,012` vs `8,088` = `+924`
- `generated_composer.lock`: `4,112` vs `3,766` = `+346`
- `decodecorpus_z000079`: `7,322` vs `7,221` = `+101`
- `generated_tsconfig.json`: `2,484` vs `2,391` = `+93`
- `generated_deno.json`: `2,484` vs `2,391` = `+93`
- `generated_nx.json`: `2,484` vs `2,391` = `+93`

Useful signal:
- the current-entry recency family remains broad-safe and continues to move the dominant
  `Cargo.lock` target
- `Cargo.lock` is now below a `+925` byte gap to C on the retained baseline

Artifacts:
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargo-twelfthnewest-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-cargo-twelfthnewest-retained)

Verification passed:
- `cargo fmt --all --check`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q -p ruzstd lockfile_sidecar_tracks_twelfth_newest_for_current_entry -- --nocapture`
- `cargo test -q --workspace`
- `cargo build --release -q -p ruzstd-cli`
- `python3 -m py_compile tools/tune_matcher_family.py tools/benchmark_zstd.py tools/prepare_benchmark_suites.py`

Useful conclusion:
- the current-entry recent-candidate family is still live on `Cargo.lock`
- the main `Cargo.lock` gap is now `+924`
- the next turn should explicitly decide whether to keep extending this family or pivot back to a
  different lockfile representation branch

## 2026-06-01 - Retained Cargo.lock current-entry eleventh-newest sidecar

Change notes:
- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added an eleventh current-entry recent sidecar for lockfile-profiled text
  - probe the new eleventh-newest candidate after the retained tenth-newest path
- Added focused matcher coverage:
  - `lockfile_sidecar_tracks_eleventh_newest_for_current_entry`

Retained:
- focused result:
  - `repo_Cargo.lock`: `9,021 -> 9,014`
  - unchanged controls:
    - `generated_go.sum`: `151`
    - `generated_poetry.lock`: `358`
    - `generated_yarn.lock`: `383`
    - `generated_composer.lock`: `4,112`
    - `generated_package-lock.json`: `4,381`
    - `generated_pipfile.lock`: `2,804`
- broad-local result:
  - only `repo_Cargo.lock` moved

Current broad-local vs C `zstd -1`:
- `71` fixtures
- `51 / 16 / 4` better / worse / equal
- `1,697` bytes above C on losing fixtures

Current biggest losses:
- `repo_Cargo.lock`: `9,014` vs `8,088` = `+926`
- `generated_composer.lock`: `4,112` vs `3,766` = `+346`
- `decodecorpus_z000079`: `7,322` vs `7,221` = `+101`
- `generated_tsconfig.json`: `2,484` vs `2,391` = `+93`
- `generated_deno.json`: `2,484` vs `2,391` = `+93`
- `generated_nx.json`: `2,484` vs `2,391` = `+93`

Useful signal:
- the current-entry recency family remains broad-safe and continues to move the dominant
  `Cargo.lock` target
- `Cargo.lock` is now below a `+930` byte gap to C on the retained baseline

Artifacts:
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargo-eleventhnewest-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-cargo-eleventhnewest-retained)

Verification passed:
- `cargo fmt --all --check`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q -p ruzstd lockfile_sidecar_tracks_eleventh_newest_for_current_entry -- --nocapture`
- `cargo test -q --workspace`
- `cargo build --release -q -p ruzstd-cli`
- `python3 -m py_compile tools/tune_matcher_family.py tools/benchmark_zstd.py tools/prepare_benchmark_suites.py`

Useful conclusion:
- the current-entry recent-candidate family is still live on `Cargo.lock`
- the main `Cargo.lock` gap is now `+926`
- the next turn should explicitly decide whether to keep extending this family or pivot back to a
  different lockfile representation branch

## 2026-06-01 - Retained Cargo.lock current-entry tenth-newest sidecar

Change notes:
- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added a tenth current-entry recent sidecar for lockfile-profiled text
  - probe the new tenth-newest candidate after the retained ninth-newest path
- Added focused matcher coverage:
  - `lockfile_sidecar_tracks_tenth_newest_for_current_entry`

Retained:
- focused result:
  - `repo_Cargo.lock`: `9,025 -> 9,021`
  - unchanged controls:
    - `generated_go.sum`: `151`
    - `generated_poetry.lock`: `358`
    - `generated_yarn.lock`: `383`
    - `generated_composer.lock`: `4,112`
    - `generated_package-lock.json`: `4,381`
    - `generated_pipfile.lock`: `2,804`
- broad-local result:
  - only `repo_Cargo.lock` moved

Current broad-local vs C `zstd -1`:
- `71` fixtures
- `51 / 16 / 4` better / worse / equal
- `1,704` bytes above C on losing fixtures

Current biggest losses:
- `repo_Cargo.lock`: `9,021` vs `8,088` = `+933`
- `generated_composer.lock`: `4,112` vs `3,766` = `+346`
- `decodecorpus_z000079`: `7,322` vs `7,221` = `+101`
- `generated_tsconfig.json`: `2,484` vs `2,391` = `+93`
- `generated_deno.json`: `2,484` vs `2,391` = `+93`
- `generated_nx.json`: `2,484` vs `2,391` = `+93`

Useful signal:
- the current-entry recency family remains broad-safe and continues to move the dominant
  `Cargo.lock` target
- `Cargo.lock` is now below a `+935` byte gap to C on the retained baseline

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/cargo-tenthnewest-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargo-tenthnewest-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-cargo-tenthnewest-retained)

Verification passed:
- `cargo fmt --all --check`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q -p ruzstd lockfile_sidecar_tracks_tenth_newest_for_current_entry -- --nocapture`
- `cargo test -q --workspace`
- `cargo build --release -q -p ruzstd-cli`
- `python3 -m py_compile tools/tune_matcher_family.py tools/benchmark_zstd.py tools/prepare_benchmark_suites.py`

Useful conclusion:
- the current-entry recent-candidate family is still live on `Cargo.lock`
- the main `Cargo.lock` gap is now `+933`
- the next turn should explicitly decide whether to keep extending this family or pivot back to a
  different lockfile representation branch

## 2026-06-01 - Retained Cargo.lock current-entry ninth-newest sidecar

Change notes:
- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added a ninth current-entry recent sidecar for lockfile-profiled text
  - probe the new ninth-newest candidate after the retained eighth-newest path
- Added focused matcher coverage:
  - `lockfile_sidecar_tracks_ninth_newest_for_current_entry`

Retained:
- focused result:
  - `repo_Cargo.lock`: `9,032 -> 9,025`
  - unchanged controls:
    - `generated_go.sum`: `151`
    - `generated_poetry.lock`: `358`
    - `generated_yarn.lock`: `383`
    - `generated_composer.lock`: `4,112`
    - `generated_package-lock.json`: `4,381`
    - `generated_pipfile.lock`: `2,804`
- broad-local result:
  - only `repo_Cargo.lock` moved

Current broad-local vs C `zstd -1`:
- `71` fixtures
- `51 / 16 / 4` better / worse / equal
- `1,708` bytes above C on losing fixtures

Current biggest losses:
- `repo_Cargo.lock`: `9,025` vs `8,088` = `+937`
- `generated_composer.lock`: `4,112` vs `3,766` = `+346`
- `decodecorpus_z000079`: `7,322` vs `7,221` = `+101`
- `generated_tsconfig.json`: `2,484` vs `2,391` = `+93`
- `generated_deno.json`: `2,484` vs `2,391` = `+93`
- `generated_nx.json`: `2,484` vs `2,391` = `+93`

Useful signal:
- the current-entry recency family remains broad-safe and continues to move the dominant
  `Cargo.lock` target
- `Cargo.lock` is now below a `+940` byte gap to C on the retained baseline

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/cargo-ninthnewest-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargo-ninthnewest-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-cargo-ninthnewest-retained)

Verification passed:
- `cargo fmt --all --check`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q -p ruzstd lockfile_sidecar_tracks_ninth_newest_for_current_entry -- --nocapture`
- `cargo test -q --workspace`
- `cargo build --release -q -p ruzstd-cli`
- `python3 -m py_compile tools/tune_matcher_family.py tools/benchmark_zstd.py tools/prepare_benchmark_suites.py`

Useful conclusion:
- the current-entry recent-candidate family is still live on `Cargo.lock`
- the main `Cargo.lock` gap is now `+937`
- the next turn should explicitly decide whether to keep extending this family or pivot back to a
  different lockfile representation branch

## 2026-06-01 - Retained Cargo.lock current-entry eighth-newest sidecar

Change notes:
- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added an eighth current-entry recent sidecar for lockfile-profiled text
  - probe the new eighth-newest candidate after the retained seventh-newest path
- Added focused matcher coverage:
  - `lockfile_sidecar_tracks_eighth_newest_for_current_entry`

Retained:
- focused result:
  - `repo_Cargo.lock`: `9,042 -> 9,032`
  - unchanged controls:
    - `generated_go.sum`: `151`
    - `generated_poetry.lock`: `358`
    - `generated_yarn.lock`: `383`
    - `generated_composer.lock`: `4,112`
    - `generated_package-lock.json`: `4,381`
    - `generated_pipfile.lock`: `2,804`
- broad-local result:
  - only `repo_Cargo.lock` moved

Current broad-local vs C `zstd -1`:
- `71` fixtures
- `51 / 16 / 4` better / worse / equal
- `1,715` bytes above C on losing fixtures

Current biggest losses:
- `repo_Cargo.lock`: `9,032` vs `8,088` = `+944`
- `generated_composer.lock`: `4,112` vs `3,766` = `+346`
- `decodecorpus_z000079`: `7,322` vs `7,221` = `+101`
- `generated_tsconfig.json`: `2,484` vs `2,391` = `+93`
- `generated_deno.json`: `2,484` vs `2,391` = `+93`
- `generated_nx.json`: `2,484` vs `2,391` = `+93`

Useful signal:
- the current-entry recency family is still broad-safe and continues to move the dominant
  `Cargo.lock` target
- `Cargo.lock` is now below a `+945` byte gap to C on the retained baseline

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/cargo-eighthnewest-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargo-eighthnewest-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-cargo-eighthnewest-retained)

Verification passed:
- `cargo fmt --all --check`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q -p ruzstd lockfile_sidecar_tracks_eighth_newest_for_current_entry -- --nocapture`
- `cargo test -q --workspace`
- `cargo build --release -q -p ruzstd-cli`
- `python3 -m py_compile tools/tune_matcher_family.py tools/benchmark_zstd.py tools/prepare_benchmark_suites.py`

Useful conclusion:
- the current-entry recent-candidate family is still live on `Cargo.lock`
- the main `Cargo.lock` gap is now `+944`
- the next turn should explicitly decide whether to keep extending this family or pivot back to a
  different lockfile representation branch

## 2026-06-01 - Retained Cargo.lock current-entry seventh-newest sidecar

Change notes:
- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added a seventh current-entry recent sidecar for lockfile-profiled text
  - probe the new seventh-newest candidate after the retained sixth-newest path
- Added focused matcher coverage:
  - `lockfile_sidecar_tracks_seventh_newest_for_current_entry`

Retained:
- focused result:
  - `repo_Cargo.lock`: `9,051 -> 9,042`
  - unchanged controls:
    - `generated_go.sum`: `151`
    - `generated_poetry.lock`: `358`
    - `generated_yarn.lock`: `383`
    - `generated_composer.lock`: `4,112`
    - `generated_package-lock.json`: `4,381`
    - `generated_pipfile.lock`: `2,804`
- broad-local result:
  - only `repo_Cargo.lock` moved

Current broad-local vs C `zstd -1`:
- `71` fixtures
- `51 / 16 / 4` better / worse / equal
- `1,725` bytes above C on losing fixtures

Current biggest losses:
- `repo_Cargo.lock`: `9,042` vs `8,088` = `+954`
- `generated_composer.lock`: `4,112` vs `3,766` = `+346`
- `decodecorpus_z000079`: `7,322` vs `7,221` = `+101`
- `generated_tsconfig.json`: `2,484` vs `2,391` = `+93`
- `generated_deno.json`: `2,484` vs `2,391` = `+93`
- `generated_nx.json`: `2,484` vs `2,391` = `+93`

Useful signal:
- the current-entry recency family remains broad-safe and continues to move the dominant
  `Cargo.lock` target
- `Cargo.lock` is now below a `+955` byte gap to C on the retained baseline

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/cargo-seventhnewest-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargo-seventhnewest-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-cargo-seventhnewest-retained)

Verification passed:
- `cargo fmt --all --check`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q -p ruzstd lockfile_sidecar_tracks_seventh_newest_for_current_entry -- --nocapture`
- `cargo test -q --workspace`
- `cargo build --release -q -p ruzstd-cli`
- `python3 -m py_compile tools/tune_matcher_family.py tools/benchmark_zstd.py tools/prepare_benchmark_suites.py`

Useful conclusion:
- the current-entry recent-candidate family is still live on `Cargo.lock`
- the main `Cargo.lock` gap is now `+954`
- the next turn should decide explicitly whether to keep extending this family or pivot to a
  different lockfile representation branch

## 2026-06-01 - Retained Cargo.lock current-entry sixth-newest sidecar

Change notes:
- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added a sixth current-entry recent sidecar for lockfile-profiled text
  - probe the new sixth-newest candidate after the retained fifth-newest path
- Added focused matcher coverage:
  - `lockfile_sidecar_tracks_sixth_newest_for_current_entry`

Retained:
- focused result:
  - `repo_Cargo.lock`: `9,065 -> 9,051`
  - unchanged controls:
    - `generated_go.sum`: `151`
    - `generated_poetry.lock`: `358`
    - `generated_yarn.lock`: `383`
    - `generated_composer.lock`: `4,112`
    - `generated_package-lock.json`: `4,381`
    - `generated_pipfile.lock`: `2,804`
- broad-local result:
  - only `repo_Cargo.lock` moved

Current broad-local vs C `zstd -1`:
- `71` fixtures
- `51 / 16 / 4` better / worse / equal
- `1,734` bytes above C on losing fixtures

Current biggest losses:
- `repo_Cargo.lock`: `9,051` vs `8,088` = `+963`
- `generated_composer.lock`: `4,112` vs `3,766` = `+346`
- `decodecorpus_z000079`: `7,322` vs `7,221` = `+101`
- `generated_tsconfig.json`: `2,484` vs `2,391` = `+93`
- `generated_deno.json`: `2,484` vs `2,391` = `+93`
- `generated_nx.json`: `2,484` vs `2,391` = `+93`

Useful signal:
- the same current-entry recency family remains broad-safe and keeps moving the dominant
  `Cargo.lock` target
- `Cargo.lock` is now below a `+965` byte gap to C on the retained baseline

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/cargo-sixthnewest-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargo-sixthnewest-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-cargo-sixthnewest-retained)

Verification passed:
- `cargo fmt --all --check`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q -p ruzstd lockfile_sidecar_tracks_sixth_newest_for_current_entry -- --nocapture`
- `cargo test -q --workspace`
- `cargo build --release -q -p ruzstd-cli`
- `python3 -m py_compile tools/tune_matcher_family.py tools/benchmark_zstd.py tools/prepare_benchmark_suites.py`

Useful conclusion:
- the current-entry recent-candidate family is still live
- the main `Cargo.lock` gap is now `+963`
- the next decision should be whether one more recency slot is still worth the extra state, or
  whether this is the right point to pivot back to a different lockfile representation family

## 2026-06-01 - Retained Cargo.lock current-entry fifth-newest sidecar

Change notes:
- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added a fifth current-entry recent sidecar for lockfile-profiled text
  - probe the new fifth-newest candidate after the retained fourth-newest path
- Added focused matcher coverage:
  - `lockfile_sidecar_tracks_fifth_newest_for_current_entry`

Retained:
- focused result:
  - `repo_Cargo.lock`: `9,073 -> 9,065`
  - unchanged controls:
    - `generated_go.sum`: `151`
    - `generated_poetry.lock`: `358`
    - `generated_yarn.lock`: `383`
    - `generated_composer.lock`: `4,112`
    - `generated_package-lock.json`: `4,381`
    - `generated_pipfile.lock`: `2,804`
- broad-local result:
  - only `repo_Cargo.lock` moved

Current broad-local vs C `zstd -1`:
- `71` fixtures
- `51 / 16 / 4` better / worse / equal
- `1,748` bytes above C on losing fixtures

Current biggest losses:
- `repo_Cargo.lock`: `9,065` vs `8,088` = `+977`
- `generated_composer.lock`: `4,112` vs `3,766` = `+346`
- `decodecorpus_z000079`: `7,322` vs `7,221` = `+101`
- `generated_tsconfig.json`: `2,484` vs `2,391` = `+93`
- `generated_deno.json`: `2,484` vs `2,391` = `+93`
- `generated_nx.json`: `2,484` vs `2,391` = `+93`

Useful signal:
- the same lockfile current-entry recency family kept producing isolated gains on the dominant
  target
- `Cargo.lock` is now below a `+980` byte gap to C on the retained baseline

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/cargo-fifthnewest-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargo-fifthnewest-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-cargo-fifthnewest-retained)

Verification passed:
- `cargo fmt --all --check`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q -p ruzstd lockfile_sidecar_tracks_fifth_newest_for_current_entry -- --nocapture`
- `cargo test -q --workspace`
- `cargo build --release -q -p ruzstd-cli`
- `python3 -m py_compile tools/tune_matcher_family.py tools/benchmark_zstd.py tools/prepare_benchmark_suites.py`

Useful conclusion:
- the current-entry recent-candidate family still has lockfile headroom, but the gains are
  shrinking per added slot
- the main `Cargo.lock` gap is now `+977`
- the next branch should probably test whether another recency slot is still worth the extra state
  before continuing to hand-unroll more of this family

## 2026-06-01 - Retained Cargo.lock current-entry fourth-newest sidecar

Change notes:
- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added a fourth current-entry recent sidecar for lockfile-profiled text
  - probe the new fourth-newest candidate after the retained third-newest path
- Added focused matcher coverage:
  - `lockfile_sidecar_tracks_fourth_newest_for_current_entry`

Retained:
- focused result:
  - `repo_Cargo.lock`: `9,087 -> 9,073`
  - unchanged controls:
    - `generated_go.sum`: `151`
    - `generated_poetry.lock`: `358`
    - `generated_yarn.lock`: `383`
    - `generated_composer.lock`: `4,112`
    - `generated_package-lock.json`: `4,381`
    - `generated_pipfile.lock`: `2,804`
- broad-local result:
  - only `repo_Cargo.lock` moved

Current broad-local vs C `zstd -1`:
- `71` fixtures
- `51 / 16 / 4` better / worse / equal
- `1,756` bytes above C on losing fixtures

Current biggest losses:
- `repo_Cargo.lock`: `9,073` vs `8,088` = `+985`
- `generated_composer.lock`: `4,112` vs `3,766` = `+346`
- `decodecorpus_z000079`: `7,322` vs `7,221` = `+101`
- `generated_tsconfig.json`: `2,484` vs `2,391` = `+93`
- `generated_deno.json`: `2,484` vs `2,391` = `+93`
- `generated_nx.json`: `2,484` vs `2,391` = `+93`

Useful signal:
- the broadened current-entry recency family kept moving the same dominant target while leaving
  the rest of the suite flat
- the live retained branch is now under a `+1000` byte gap to C on `repo_Cargo.lock`

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/cargo-fourthnewest-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargo-fourthnewest-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-cargo-fourthnewest-retained)

Verification passed:
- `cargo fmt --all --check`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q -p ruzstd lockfile_sidecar_tracks_fourth_newest_for_current_entry -- --nocapture`
- `cargo test -q --workspace`
- `cargo build --release -q -p ruzstd-cli`
- `python3 -m py_compile tools/tune_matcher_family.py tools/benchmark_zstd.py tools/prepare_benchmark_suites.py`

Useful conclusion:
- the lockfile current-entry recent-candidate representation still has measurable headroom
- the dominant `Cargo.lock` gap is now `+985`
- the next credible branch can still stay in this broader current-entry representation family, but
  it should be judged against diminishing returns from each extra recency slot

## 2026-06-01 - Retained Cargo.lock current-entry third-newest sidecar

Change notes:
- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added a third current-entry recent sidecar for lockfile-profiled text
  - probe the new third-newest candidate after `second_newest` and before `newest`
  - keep the diagnostics surface unchanged by reusing the existing second-newest bookkeeping in
    test-only candidate-source reporting
- Added focused matcher coverage:
  - `lockfile_sidecar_tracks_third_newest_for_current_entry`

Retained:
- focused result:
  - `repo_Cargo.lock`: `9,104 -> 9,087`
  - unchanged controls:
    - `generated_go.sum`: `151`
    - `generated_poetry.lock`: `358`
    - `generated_yarn.lock`: `383`
    - `generated_composer.lock`: `4,112`
    - `generated_package-lock.json`: `4,381`
    - `generated_pipfile.lock`: `2,804`
- broad-local result:
  - only `repo_Cargo.lock` moved

Current broad-local vs C `zstd -1`:
- `71` fixtures
- `51 / 16 / 4` better / worse / equal
- `1,770` bytes above C on losing fixtures

Current biggest losses:
- `repo_Cargo.lock`: `9,087` vs `8,088` = `+999`
- `generated_composer.lock`: `4,112` vs `3,766` = `+346`
- `decodecorpus_z000079`: `7,322` vs `7,221` = `+101`
- `generated_tsconfig.json`: `2,484` vs `2,391` = `+93`
- `generated_deno.json`: `2,484` vs `2,391` = `+93`
- `generated_nx.json`: `2,484` vs `2,391` = `+93`

Useful signal:
- live `Cargo.lock` inspect before the change:
  - `compressed_bytes=9104`
  - `literal_section_bytes=6902`
  - `sequence_payload_bytes=2182`
  - `decoded_literals=9953`
  - `sequences=810`
  - `match_bytes=21905`
  - `of_extra_bits=6773`
- the remaining lockfile gap is still parse/offset-shape driven, but the new current-entry
  representation cut the main target again without broad collateral

Artifacts:
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargo-thirdnewest-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-cargo-thirdnewest-retained)

Verification passed:
- `cargo fmt --all --check`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q -p ruzstd match_generator::lockfile_sidecar_tracks_third_newest_for_current_entry -- --nocapture`
- `cargo test -q --workspace`
- `cargo build --release -q -p ruzstd-cli`
- `python3 -m py_compile tools/tune_matcher_family.py tools/benchmark_zstd.py tools/prepare_benchmark_suites.py`

Useful conclusion:
- the lockfile current-entry recent-candidate family still had real headroom
- `Cargo.lock` is now below a `+1000` byte gap to C on the retained baseline
- the next credible branch should continue from this broader current-entry representation point,
  not return to already-bounded local threshold families

## 2026-06-01 - Rejected Cargo.lock local-parse current-window search

Change notes:
- Tested one broader lockfile parser-side family in `ruzstd/src/encoding/match_generator.rs`:
  - first a small local parse with simulated repeat history
  - then a widened local current-window search scoring several nearby window alternatives
- Added a temporary focused tuner preset:
  - `cargo-lock-local-parse`

Rejected:
- lockfile local-parse current-window search

Result:
- focused sweep stayed flat:
  - baseline focused family: `9,996`
  - best searched candidate: `9,996`
- report:
  - [focused sweep](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-cargo-lock-local-parse.md)

Useful conclusion:
- the next productive `Cargo.lock` move was not hidden in a slightly wider current-window local
  parse around the same active candidates
- do not retry this family in the same form
- the retained breakthrough came from broadening the current-entry recent-candidate
  representation instead

## 2026-06-01 - Rejected Cargo.lock byte-class lazy-parse literals and known-size single-segment frames

Change notes:
- Tested one new tune-only `Cargo.lock` lazy-parse branch in
  `ruzstd/src/encoding/match_generator.rs`:
  - replace the flat skipped-literal penalty inside the retained lockfile next-position compare
    with a byte-class literal model for lockfile syntax and common text bytes
- Added and ran focused tuner sweeps:
  - `cargo-lock-next-position-byteclass`
  - refreshed `cargo-lock-encoder` on the restored current baseline
- Also tested a broader frame-level branch:
  - plumb exact known content size through the path-based CLI compression flow
  - emit single-segment frames when size is known

Rejected:
- `Cargo.lock` byte-class lazy-parse literal model
- refreshed `Cargo.lock` encoder surface
- known-size single-segment frames on the file-path compression path

Result:
- `cargo-lock-next-position-byteclass`
  - focused family baseline: `9,996`
  - best searched candidate: `9,996`
  - report:
    - [focused sweep](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-cargo-lock-next-position-byteclass.md)
- refreshed `cargo-lock-encoder`
  - focused family baseline: `10,006`
  - best searched candidate: `10,006`
  - report:
    - [focused sweep](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-cargo-lock-encoder-postrevert.md)
- known-size single-segment frames
  - direct spot checks regressed and the branch was reverted immediately:
    - `repo_Cargo.lock`: `9,104 -> 9,105`
    - `generated_composer.lock`: `4,112 -> 4,115`
    - `generated_poetry.lock`: `358 -> 361`
    - `generated_pubspec.lock`: `229 -> 232`
    - `generated_package.json`: `3,785 -> 3,788`
    - `decodecorpus_z000079`: `7,322 -> 7,325`

Restore check:
- after reverting the failed frame branch and the tune-only byte-class branch, key fixtures matched
  the retained `composer-rep2` binary exactly:
  - `repo_Cargo.lock = 9,104`
  - `generated_composer.lock = 4,112`
  - `generated_poetry.lock = 358`
  - `generated_pubspec.lock = 229`
  - `generated_package-lock.json = 4,381`
  - `generated_pipfile.lock = 2,804`

Useful conclusion:
- the current `Cargo.lock` encoder surface is still flat on the live retained baseline
- the lazy-parse family does not move on a more optimistic byte-class literal penalty either
- C's single-segment known-size headers are not the reason we trail on these file inputs; for the
  live CLI path they are actually larger than the current unknown-size frame form
- the next credible branch still needs a broader parse/sequence representation change

## 2026-06-01 - Bounded wider Cargo.lock lazy-parse surface and composer repeat-kind >2

Change notes:
- Added focused tuner presets only in `tools/tune_matcher_family.py`:
  - `cargo-lock-next-position-wide`
  - `cargo-lock-combined-lazy`
  - `composer-repeatkind-wide`

Rejected:
- wider `Cargo.lock` lazy-parse-only surface
- combined `Cargo.lock` lazy-parse plus nearby retained tie-break surfaces
- composer repeat-kind match-loss `3` and `4`

Result:
- `cargo-lock-next-position-wide`
  - focused family baseline: `9,996`
  - best searched candidate: `9,996`
  - report:
    - [focused sweep](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-cargo-lock-next-position-wide.md)
- `cargo-lock-combined-lazy`
  - focused family baseline: `9,996`
  - best searched candidate: `9,996`
  - report:
    - [focused sweep](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-cargo-lock-combined-lazy.md)
- `composer-repeatkind-wide`
  - focused family baseline: `11,448`
  - best searched candidate: `11,448`
  - direct per-fixture check confirmed `loss=3` is an exact alias of the retained `loss=2`
    point:
    - `generated_composer.lock`: `4,112 -> 4,112`
    - `generated_pipfile.lock`: `2,804 -> 2,804`
    - `generated_package-lock.json`: `4,381 -> 4,381`
    - `generated_go.sum`: `151 -> 151`
  - report:
    - [focused sweep](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-composer-repeatkind-wide.md)

Current broad-local vs C `zstd -1` is unchanged:
- `71` fixtures
- `51 / 16 / 4` better / worse / equal
- `1,787` bytes above C on losing fixtures

Useful conclusion:
- the productive `Cargo.lock` lazy-parse family is bounded again on the wider searched local
  surface
- the retained composer same-start repeat-kind family is also bounded upward: `2`, `3`, and `4`
  are equivalent on the live focused family
- next credible work should move away from nearby local matcher-threshold combinations and back to
  broader representation changes

## 2026-06-01 - Retained composer repeat-kind match-loss 2

Change notes:
- Widened the retained composer same-start repeat-kind preference in
  `ruzstd/src/encoding/match_generator.rs`.
- For composer-profiled files, when two current-position repeat candidates start at the same byte,
  the matcher now prefers the repeat kind that better matches the encoder repeat-code order even
  when it loses up to `2` match bytes instead of `1`.

Retained:
- focused result:
  - `generated_composer.lock`: `4,119 -> 4,112`
  - unchanged controls:
    - `generated_pipfile.lock`: `2,804`
    - `generated_package-lock.json`: `4,381`
    - `generated_go.sum`: `151`
- broad-local result:
  - only `generated_composer.lock` moved

Current broad-local vs C `zstd -1`:
- `71` fixtures
- `51 / 16 / 4` better / worse / equal
- `1,787` bytes above C on losing fixtures

Current biggest losses:
- `repo_Cargo.lock`: `9,104` vs `8,088` = `+1,016`
- `generated_composer.lock`: `4,112` vs `3,766` = `+346`
- `decodecorpus_z000079`: `7,322` vs `7,221` = `+101`

Useful signal:
- same block count and same sequence count on `generated_composer.lock`
- `sequence_payload_bytes`: `3386 -> 3375`
- `literal_section_bytes`: `687 -> 691`

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-rep2-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-composer-rep2-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-composer-rep2-retained)

Verification passed:
- `cargo fmt --all --check`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q --workspace`
- `cargo build --release -q -p ruzstd-cli`
- `python3 -m py_compile tools/tune_matcher_family.py tools/benchmark_zstd.py tools/prepare_benchmark_suites.py`

Useful conclusion:
- the whole-file `ComposerLock` profile exposed one more real improvement from the same-start
  repeat-kind family
- the remaining composer gap is still sequence-section heavy, but it is smaller again
- the next highest-value target remains `repo_Cargo.lock`

## 2026-06-01 - Kept Cargo.lock matcher-profile plumbing, rejected broader exact LL/ML candidate window

Change notes:
- Extended the existing internal `Cargo.lock` profile so it now reaches the matcher as well as the
  encoder:
  - added an internal file-profile hint hook to the matcher API
  - threaded the `Cargo.lock` profile through `FrameCompressor` into `MatchGenerator`
  - `Cargo.lock` blocks can now take the exact profile path on the parse side without relying only
    on content heuristics
- Tested one broader exact sequence-mode branch in
  `ruzstd/src/encoding/blocks/compressed.rs`:
  - for `Cargo.lock`, widen the exact LL/ML candidate set to include predefined LL/ML tables up to
    `1024` sequences

Retained:
- Cargo.lock matcher-profile plumbing

Rejected:
- broader exact LL/ML candidate window for Cargo.lock

Result:
- the broader exact LL/ML branch was an exact no-op on the focused lockfile family and was
  reverted:
  - [focused](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/cargolock-llml.md)
- the retained matcher-profile plumbing is output-neutral against the retained
  `lockfamily-encoded-maxlog` baseline:
  - [restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/cargolock-profile-matcher.md)

Verification passed:
- `cargo fmt --all --check`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q --workspace`
- `cargo build --release -q -p ruzstd-cli`

Useful conclusion:
- `Cargo.lock` now has exact profile plumbing on both the encoder and matcher sides
- widening the exact LL/ML predefined candidate window did not move the live lockfile family
- the next credible `Cargo.lock` branch still needs a more substantive literal/sequence
  representation change

## 2026-06-01 - Kept Cargo.lock profile scaffold, rejected zero-literal rewrite family

Change notes:
- Added a dedicated internal `CompressionFileProfile::CargoLock` in
  `ruzstd/src/encoding/mod.rs`.
- `repo_Cargo.lock`-style named files now carry a specific profile hook for future
  extension-based compression work.
- Tested one broader post-parse representation family in
  `ruzstd/src/encoding/blocks/compressed.rs`:
  - tune-only rewrite of short zero-literal `Cargo.lock` matches into literals
  - the rewrite rebuilt the prepared block so repeat history stayed consistent end to end
- Swept:
  - `RUZSTD_TUNE_LOCKFILE_DROP_ZERO_LITERAL_MATCH_MAX_LEN = 5/6/7/8`
  - `RUZSTD_TUNE_LOCKFILE_DROP_ZERO_LITERAL_MATCH_MIN_OF_CODE = 7/8/9/10`
- Spot-checked narrower cases outside the sweep:
  - `(max_len=4, min_of_code=10)`
  - `(max_len=4, min_of_code=9)`
  - `(max_len=3, min_of_code=9)`

Retained:
- `Cargo.lock` profile scaffold only

Rejected:
- lockfile zero-literal post-parse rewrite family

Result:
- focused sweep regressed for `max_len >= 5`
  - baseline `10,002`
  - best swept candidate `10,052`
- narrower `max_len 3/4` spot-checks were exact no-ops:
  - `repo_Cargo.lock`: stayed `9,109`
  - `generated_go.sum`: stayed `151`
  - `generated_poetry.lock`: stayed `359`
  - `generated_yarn.lock`: stayed `383`
- after removing the rewrite branch and keeping only the `Cargo.lock` profile scaffold,
  restore check confirmed exact equality to the retained `lockfamily-encoded-maxlog` baseline:
  - [restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/cargolock-profile-restore.md)

Useful conclusion:
- `Cargo.lock` now has dedicated profile plumbing for future extension-specific algorithms
- this first post-parse zero-literal rewrite family is bounded:
  - `max_len >= 5` regresses
  - `max_len 3/4` is flat
- the next credible `Cargo.lock` branch still needs a different literal/sequence representation

## 2026-06-01 - Rejected lockfile lazy-parse family

Change notes:
- Tested one broader parser-side `Cargo.lock` family in
  `ruzstd/src/encoding/match_generator.rs`.
- Added a tune-only one-step lazy parse for lockfile-like `DictionaryText`:
  - score current and next-position candidates with a cheap local score
  - defer the current candidate when the next candidate wins by a configured margin
- Swept:
  - `RUZSTD_TUNE_LOCKFILE_LAZY_SCORE_DIVISOR = 1/2/3/4/5/6`
  - `RUZSTD_TUNE_LOCKFILE_LAZY_MIN_GAIN = 0/1/2/3`

Rejected:
- lockfile lazy-parse family

Result:
- focused tuner sweep stayed flat:
  - baseline `10,002`
  - best searched candidate `10,002`
- the runtime branch was reverted
- restore check confirmed the current tree is back on the retained
  `lockfamily-encoded-maxlog` baseline exactly:
  - [restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-lazy-restore.md)

Useful conclusion:
- `Cargo.lock` is not waiting on this one-step lazy-parse family either
- that closes another broader parser-side representation attempt around the current matcher

## 2026-06-01 - Rejected lockfile non-repeat offset-score family

Change notes:
- Tested one broader parser-side `Cargo.lock` family in
  `ruzstd/src/encoding/match_generator.rs`.
- Added a tune-only non-repeat offset scorer for lockfile-like `DictionaryText` that compares
  candidates by:
  - `match_len * divisor - offset_code_bits`
- Swept:
  - `RUZSTD_TUNE_LOCKFILE_NONREPEAT_OFFSET_PENALTY_DIVISOR = 1/2/3/4/5/6`

Rejected:
- lockfile non-repeat offset-score family

Result:
- focused tuner sweep stayed flat:
  - baseline `10,002`
  - best searched candidate `10,002`
- the runtime branch was reverted
- restore check confirmed the current tree is back on the retained
  `lockfamily-encoded-maxlog` baseline exactly:
  - [restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-general-offset-restore.md)

Useful conclusion:
- `Cargo.lock` is not waiting on a broader non-repeat offset-bit scoring rule either
- that closes another parser-side cost-model family around the current candidate comparer

## 2026-06-01 - Rejected generic smaller-offset lockfile matcher family

Change notes:
- Tested one broader parser-side lockfile family in `ruzstd/src/encoding/match_generator.rs`.
- Added a tuneable generic smaller-offset preference for non-repeat `Cargo.lock` candidates,
  instead of limiting offset-aware tie-breaking to the already-retained same-start and same-end
  special cases.
- Swept:
  - `RUZSTD_TUNE_LOCKFILE_SMALLER_OFFSET_MATCH_LOSS_MAX = 0/1/2/3`
  - `RUZSTD_TUNE_LOCKFILE_SMALLER_OFFSET_BITS_GAIN_MIN = 1/2/3/4`

Rejected:
- generic smaller-offset lockfile matcher family

Result:
- focused tuner sweep stayed flat:
  - baseline `10,002`
  - best searched candidate `10,002`
- the default branch itself was also an exact byte-for-byte no-op on broad-local against the
  retained `lockfamily-encoded-maxlog` baseline
- restore check confirmed the current tree is back on that retained baseline exactly:
  - [restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-general-offset-restore.md)

Useful conclusion:
- the remaining `Cargo.lock` gap is not waiting on a generic smaller-offset matcher preference
  either
- that closes another broader parser-side offset-scoring family

## 2026-06-01 - Rejected lockfile-family exact encoded-table normalization variant search

Change notes:
- Tested one broader follow-up inside the retained exact sequence-mode search in
  `ruzstd/src/encoding/blocks/compressed.rs`.
- For exact-sequence lockfile-family searches, encoded LL/ML/OF candidates were expanded to try
  both valid `avoid_0_numbit` normalization settings while still scanning encoded-table max-log
  choices.

Rejected:
- exact encoded-table normalization variant search for lockfile families

Result:
- exact byte-for-byte no-op on broad-local against the retained
  `lockfamily-encoded-maxlog` baseline
- restore check confirmed the current tree is back on that retained baseline exactly:
  - [restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfamily-avoidzero-restore.md)

Useful conclusion:
- the retained lockfile-family exact sequence search is not missing another nearby FSE
  normalization variant in this form
- the next credible `Cargo.lock` branch still needs a different literal/sequence representation

## 2026-06-01 - Retained broader exact encoded-table log search for lockfile families

Change notes:
- Expanded the retained exact sequence-mode search in
  `ruzstd/src/encoding/blocks/compressed.rs`.
- For fastest-level `DictionaryText` and dependency-JSON-lockfile profiles, exact sequence-mode
  search now compares:
  - the existing table modes
  - and, for encoded LL/ML/OF tables, additional valid encoded-table max-log choices in the
    `7..=max_log` range
- This keeps the broader search inside the families already paying for exact sequence re-encoding.

Retained:
- broad-local A/B against the retained `lockfamily-exact-seq` binary was clean
- moved fixture:
  - `generated_package-lock.json`: `4,383 -> 4,381`
- unchanged key controls:
  - `repo_Cargo.lock`: stayed `9,109`
  - `generated_composer.lock`: stayed `4,159`
  - `generated_pipfile.lock`: stayed `2,804`
  - `generated_pubspec.lock`: stayed `232`
  - `generated_Gemfile.lock`: stayed `239`

- Current broad-local vs C `zstd -1` remains:
  - `71` fixtures
  - `51 / 16 / 4` better / worse / equal
  - `1,842` bytes above C on losing fixtures

Artifacts:
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfamily-encoded-maxlog-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-lockfamily-encoded-maxlog-retained)

Useful conclusion:
- broader exact encoded-table log search is valid and broad-safe on the lockfile families
- it improves dependency-JSON lockfiles further, but still does not move the dominant
  `Cargo.lock` or composer gaps

## 2026-06-01 - Rejected lockfile next-position lookahead branch

Change notes:
- Tested two matcher-side lockfile branches in `ruzstd/src/encoding/match_generator.rs`:
  - let lockfile-like `DictionaryText` use the existing next-position window-lookahead gate
  - let lockfile-like `DictionaryText` compare next-position repeat candidates even when a
    current repeat candidate already exists
- Kept the retained exact-sequence encoder baseline unchanged

Rejected:
- lockfile next-position window/repeat lookahead for `Cargo.lock`-style `DictionaryText`

Result:
- exact byte-for-byte no-op on broad-local against the retained `lockfamily-exact-seq` baseline
- restore check confirmed the current tree is back on that retained baseline exactly:
  - [restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-nextpos-restore.md)

Useful conclusion:
- the remaining `Cargo.lock` gap is not waiting on next-position window or repeat lookahead in
  this form
- the next credible `Cargo.lock` branch still needs a broader literal/sequence representation

## 2026-06-01 - Rejected lockfile current-entry long-hash branch

Change notes:
- Tested a matcher branch in `ruzstd/src/encoding/match_generator.rs`:
  - enable the existing current-entry long-hash path for lockfile-like `DictionaryText`
  - keep the rest of the retained exact-sequence baseline unchanged

Rejected:
- current-entry long-hash for `Cargo.lock`-style `DictionaryText`

Result:
- exact byte-for-byte no-op on broad-local against the retained `lockfamily-exact-seq` baseline
- restore check confirmed the current tree is back on that retained baseline exactly:
  - [restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-longhash-restore.md)

Useful conclusion:
- the dormant current-entry long-hash path is not the missing `Cargo.lock` representation in this form
- the remaining dominant `Cargo.lock` gap still points away from another nearby matcher toggle

## 2026-06-01 - Retained exact LL/ML/OF sequence-mode search for lockfile families

Change notes:
- Expanded the retained encoder-side lockfile-family exact search in `ruzstd/src/encoding/blocks/compressed.rs`.
- For fastest-level `DictionaryText` and dependency-JSON-lockfile profiles, the encoder now:
  - enumerates the valid LL, ML, and OF table modes from the existing heuristic chooser family
  - exactly re-encodes the sequence section across the valid combinations
  - keeps the smallest valid LL/ML/OF combination
- Added focused unit coverage for:
  - enabling the exact sequence-mode search on the intended file families
  - the invariant that the exact sequence-mode chooser never emits a larger sequence section than the threshold path

Retained:
- broad-local A/B against the prior retained `lockfamily-exact-of` binary was clean
- moved fixture:
  - `generated_pubspec.lock`: `233 -> 232`
- unchanged key controls:
  - `repo_Cargo.lock`: stayed `9,109`
  - `generated_composer.lock`: stayed `4,159`
  - `generated_package-lock.json`: stayed `4,383`
  - `generated_pipfile.lock`: stayed `2,804`
- refreshed broad-local vs C `zstd -1`:
  - `71` fixtures
  - `51 / 16 / 4` better / worse / equal
  - `1,842` bytes above C on losing fixtures

Source reports:
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfamily-exact-seq-broad-local.md)
- [current broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-lockfamily-exact-seq-retained)

Useful conclusion:
- exact full sequence-mode comparison is broad-safe and slightly stronger than OF-only search
- it still does not move the dominant `Cargo.lock` or composer gaps
- the next credible branch is still a broader literal/sequence representation change, not another nearby FSE-table threshold

## 2026-06-01 - Retained exact OF-mode sequence-section search for lockfile families

Change notes:
- Kept one new encoder-side representation change in `ruzstd/src/encoding/blocks/compressed.rs`.
- For fastest-level `DictionaryText` and dependency-JSON-lockfile profiles, the encoder now:
  - builds the usual LL/ML/OF table modes
  - then exactly re-encodes the sequence section across the valid OF candidates
  - and keeps the smallest OF mode instead of relying only on the threshold chooser
- Added focused unit coverage for:
  - enabling the exact search on the intended file families
  - the invariant that the exact chooser never emits a larger sequence section than the threshold path

Retained:
- broad-local A/B against the pre-change binary was clean
- the only moved fixture was:
  - `generated_Gemfile.lock`: `240 -> 239`
- unchanged key controls:
  - `repo_Cargo.lock`: stayed `9,109`
  - `generated_composer.lock`: stayed `4,159`
  - `generated_package-lock.json`: stayed `4,383`
  - `generated_pipfile.lock`: stayed `2,804`
- broad-local vs C `zstd -1` stays:
  - `71` fixtures
  - `51 / 16 / 4` better / worse / equal
  - `1,843` bytes above C on losing fixtures

Source reports:
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfamily-exact-of-broad-local.md)
- [current broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-lockfamily-exact-of-retained)

Useful conclusion:
- exact OF-mode comparison is broad-safe, but it is not the missing `Cargo.lock` / composer breakthrough
- the remaining large gaps still point away from another small OF threshold family and toward a broader sequence/literal representation change

## 2026-06-01 - Rejected composer-style min non-repeat floor 5

Change notes:
- Stayed on the retained `composer-filetypeconfig` baseline:
  - `generated_composer.lock = 4,336`
  - `generated_pipfile.lock = 2,811`
  - `generated_package-lock.json = 4,392`
  - `generated_go.sum = 151`
  - `repo_Cargo.lock = 9,114`
- Focused the composer/lockfile family only.

Rejected:
- use minimum non-repeat match length `5` for composer-style `DictionaryText` blocks

Result:
- exact byte-for-byte no-op on the focused composer/lockfile family
- restore check confirmed the retained baseline exactly:
  - [restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-floor5-restore.md)

Useful conclusion:
- the remaining composer gap is not waiting on a lower non-repeat floor either
- the floor-5 family is now closed with direct focused evidence

## 2026-06-01 - Rejected composer-style probe step 2

Change notes:
- Stayed on the retained `composer-filetypeconfig` baseline:
  - `generated_composer.lock = 4,336`
  - `generated_pipfile.lock = 2,811`
  - `generated_package-lock.json = 4,392`
  - `generated_go.sum = 151`
  - `repo_Cargo.lock = 9,114`
- Focused the composer/lockfile family only.

Rejected:
- use no-match probe step `2` for composer-style `DictionaryText` blocks

Result:
- exact byte-for-byte no-op on the focused composer/lockfile family
- restore check confirmed the retained baseline exactly:
  - [restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-step2-restore.md)

Useful conclusion:
- the remaining composer gap is not waiting on a less-dense current-window probe step either
- the retained `Cargo.lock` step-2 win does not transfer to composer in this form

## 2026-06-01 - Rejected composer-style second_newest-before-newest probing

Change notes:
- Stayed on the retained `composer-filetypeconfig` baseline:
  - `generated_composer.lock = 4,336`
  - `generated_pipfile.lock = 2,811`
  - `generated_package-lock.json = 4,392`
  - `generated_go.sum = 151`
  - `repo_Cargo.lock = 9,114`
- Focused the composer/lockfile family only.

Rejected:
- probe current-entry `second_newest` before `newest` for composer-style `DictionaryText` blocks

Result:
- exact byte-for-byte no-op on the focused composer/lockfile family
- restore check confirmed the retained baseline exactly:
  - [restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-secondnewestfirst-restore.md)

Useful conclusion:
- the remaining composer gap is not waiting on lockfile-style `second_newest` probe ordering
- current-entry `second_newest` does not look like the missing representation for this family in the current matcher shape

## 2026-06-01 - Rejected two more composer-family structural branches

Change notes:
- Stayed on the retained `composer-filetypeconfig` baseline:
  - `generated_composer.lock = 4,336`
  - `generated_pipfile.lock = 2,811`
  - `generated_package-lock.json = 4,392`
  - `generated_go.sum = 151`
  - `repo_Cargo.lock = 9,114`
- Focused the composer/lockfile family only.

Rejected:
- disable the special text-repeat pipeline for composer-style `DictionaryText`
- search actual encoded composer partition candidates across partition budgets `1..=8` and keep the smallest

Result:
- both were exact byte-for-byte no-ops on the focused family
- restore check confirmed the retained baseline exactly:
  - [restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-restore-after-branches.md)

Useful conclusion:
- the remaining composer gap is not moving on this text-repeat pipeline distinction
- it is also not waiting on a broader actual-budget search over the current partition tree family

## 2026-06-01 - Retained strict partition-budget enforcement and rejected composer max-2 cap

Change notes:
- Fixed the estimated-split partition recursion in `ruzstd/src/encoding/levels/fastest.rs` so the partition budget is enforced strictly instead of letting the left subtree consume the budget and still appending the right half.
- Added focused unit coverage proving `derive_best_partitions()` cannot exceed the requested partition budget.
- Screened the composer/lockfile family only:
  - `generated_composer.lock`
  - `generated_pipfile.lock`
  - `generated_package-lock.json`
  - `generated_go.sum`
  - `repo_Cargo.lock`

Retained:
- the partition-budget fix itself
- focused restore versus the retained `composer-filetypeconfig` binary was exact:
  - `generated_composer.lock`: stayed `4,336`
  - `generated_pipfile.lock`: stayed `2,811`
  - `generated_package-lock.json`: stayed `4,392`
  - `generated_go.sum`: stayed `151`
  - `repo_Cargo.lock`: stayed `9,114`
- report:
  - [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-budgetfix-focused.md)

Rejected:
- cap the composer-style `DictionaryText` partition path at `2` partitions after fixing the budget bug

Result:
- focused regression:
  - `generated_composer.lock`: `4,336 -> 4,389`
- unchanged nearby controls:
  - `generated_pipfile.lock`: stayed `2,811`
  - `generated_package-lock.json`: stayed `4,392`
  - `generated_go.sum`: stayed `151`
  - `repo_Cargo.lock`: stayed `9,114`
- reports:
  - [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-cap2-focused.md)

Useful conclusion:
- the partition-budget bug was real and is now covered by test
- the live retained composer path was already behaving byte-identically on this focused family once the default `8`-partition budget is restored
- a hard `2`-partition cap is not the right follow-up branch for composer-style lockfiles

## 2026-06-01 - Rejected two composer partition sequence-entropy branches

Change notes:
- Stayed on the retained `composer-filetypeconfig` baseline:
  - `generated_composer.lock = 4,336`
  - `repo_Cargo.lock = 9,114`
- Focused the composer/lockfile family only:
  - `generated_composer.lock`
  - `generated_pipfile.lock`
  - `generated_package-lock.json`
  - `generated_go.sum`
  - `repo_Cargo.lock`

Rejected:
- allow composer partition candidates to reuse previous FSE tables up to `1024` sequences
- allow composer partition candidates to use predefined OF tables up to `1024` sequences

Result:
- repeat previous FSE tables: hard regression
  - `generated_composer.lock`: `4,336 -> 4,524`
- predefined OF tables: hard regression
  - `generated_composer.lock`: `4,336 -> 5,025`
- restore checks confirmed the retained baseline exactly:
  - [restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-entropy-restore2.md)

Useful conclusion:
- the remaining composer gap is not waiting on broader sequence-table reuse or a wider predefined-OF window on the partition blocks
- the current composer partition path wants its existing encoded FSE-table behavior

## 2026-06-01 - Rejected three more Cargo.lock-focused branches

Change notes:
- Stayed on the retained `composer-filetypeconfig` baseline:
  - `repo_Cargo.lock = 9,114`
  - `generated_composer.lock = 4,336`
- Focused the screen on the known lockfile family:
  - `repo_Cargo.lock`
  - `generated_composer.lock`
  - `generated_pipfile.lock`
  - `generated_package-lock.json`
  - `generated_go.sum`

Rejected:
- lockfile-like `DictionaryText` partitioned-block candidate path with fastest-level file-type block config
- lockfile current-over-`oldest` displacement when a two-byte `oldest` gain still costs at least two more offset-code bits
- lockfile current-over-`newest` displacement when a two-byte `newest` gain still costs at least two more offset-code bits

Result:
- partition path: exact no-op
  - `repo_Cargo.lock`: stayed `9,114`
- `oldest` bits branch: regression
  - `repo_Cargo.lock`: `9,114 -> 9,116`
- `newest` bits branch: exact no-op
  - `repo_Cargo.lock`: stayed `9,114`
- restore check confirmed the retained baseline exactly:
  - [restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-bits-restore.md)

Useful conclusion:
- the current `Cargo.lock` gap is not moving on another partition-path retest or another offset-bit-aware current-window displacement rule
- the remaining lockfile gap still points away from these local window-comparison branches

## 2026-06-01 - Retained file-type block config for composer partition candidates

Change notes:
- Kept one more composer-specific known-file-type compression change:
  - in `ruzstd/src/encoding/levels/fastest.rs`, the composer-style `DictionaryText` partitioned-block path now uses the live fastest-level file-type block config instead of the generic `Best` block config
- Refreshed the retained current broad-local baseline:
  - `57` fixtures
  - `43 / 10 / 4` better / worse / equal vs C
  - `1,725` bytes above C on the losing fixtures

Result:
- focused composer-family win:
  - `generated_composer.lock`: `4,340 -> 4,336`
- unchanged nearby controls:
  - `generated_pipfile.lock`: stayed `2,811`
  - `generated_package-lock.json`: stayed `4,392`
  - `generated_go.sum`: stayed `151`
  - `repo_Cargo.lock`: stayed `9,114`
- broad-local total bytes above C on losers:
  - `1,729 -> 1,725`

Current main losses:
- `repo_Cargo.lock`: `9,114` vs `8,088` (`+1,026`)
- `generated_composer.lock`: `4,336` vs `3,766` (`+570`)
- `decodecorpus_z000079`: `7,322` vs `7,221` (`+101`)

Source reports:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-filetypeconfig-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-composer-filetypeconfig-broad-local.md)
- [current broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-composer-filetypeconfig-retained)

Useful conclusion:
- the composer partition family still had a small retained gain left once it used the live `DictionaryText` fastest-level block config
- the known-file-type gap is still dominated by `repo_Cargo.lock`, with `generated_composer.lock` second

## 2026-06-01 - Retained composer-style `DictionaryText` partitioned-block path

Change notes:
- Kept one new known-file-type compression change:
  - in `ruzstd/src/encoding/levels/fastest.rs`, large composer-style `DictionaryText` blocks at level 1 now use the existing `compress_best_with_estimated_splits()` path
  - the gate uses `likely_composer_lockfile_text()` in `ruzstd/src/encoding/util.rs`
- Refreshed the retained current broad-local baseline:
  - `57` fixtures
  - `43 / 10 / 4` better / worse / equal vs C
  - `1,729` bytes above C on the losing fixtures

Result:
- focused known-file-type win:
  - `generated_composer.lock`: `4,461 -> 4,340`
- unchanged nearby controls:
  - `generated_pipfile.lock`: stayed `2,811`
  - `generated_package-lock.json`: stayed `4,392`
  - `generated_go.sum`: stayed `151`
  - `repo_Cargo.lock`: stayed `9,114`
- broad-local total bytes above C on losers:
  - `1,850 -> 1,729`

Current main losses:
- `repo_Cargo.lock`: `9,114` vs `8,088` (`+1,026`)
- `generated_composer.lock`: `4,340` vs `3,766` (`+574`)
- `decodecorpus_z000079`: `7,322` vs `7,221` (`+101`)

Source reports:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-partition-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-composer-partition-broad-local.md)
- [current broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-composer-partition-retained)

Useful conclusion:
- this is the first retained `generated_composer.lock` win after the public remaps and small matcher branches all failed
- the live composer family is now materially better, and the remaining known-file-type gap is concentrated in `repo_Cargo.lock` first and `generated_composer.lock` second

## 2026-06-01 - Expanded known-file-type corpus and rejected `composer.lock` / `Pipfile.lock` -> `JsonText`

Change notes:
- Expanded `broad-local` in `tools/prepare_benchmark_suites.py` with additional mapped known-file-type fixtures:
  - `generated_package-lock.json`
  - `generated_composer.lock`
  - `generated_pipfile.lock`
  - `generated_Gemfile`
  - `generated_Gemfile.lock`
  - `generated_go.mod`
- Refreshed current broad-local baseline on the expanded suite:
  - `57` fixtures
  - `43 / 10 / 4` better / worse / equal vs C
  - `1,850` bytes above C on the losing fixtures
- Newly exposed large known-file-type loss:
  - `generated_composer.lock`: `4,461` vs C `3,766` (`+695`)

Tested and rejected:
- map `composer.lock` and `Pipfile.lock` to `JsonText`
- focused result:
  - `generated_composer.lock`: `4,461 -> 4,482`
  - `generated_pipfile.lock`: `2,811 -> 2,885`
  - `repo_Cargo.lock`: unchanged at `9,114`
- reports:
  - [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/jsonlock-focused.md)

Useful conclusion:
- the corpus expansion is retained
- `composer.lock` and `Pipfile.lock` are now covered by benchmarks
- the plain `JsonText` starting point is wrong for both of those lockfiles in this form

## 2026-06-01 - Rejected lockfile package-boundary raw-data multi-block split

Change notes:
- Stayed on the retained live baseline for the dictionary/lockfile family:
  - `repo_Cargo.lock = 9,114`
  - `dict_dictionary.bin = 19,668`
  - `generated_go.sum = 151`
  - `generated_poetry.lock = 359`
  - `generated_yarn.lock = 383`
- Tested one structural literal-side branch in `ruzstd/src/encoding/frame_compressor.rs`:
  - for lockfile-like `DictionaryText` on the fastest path, split the raw input into package-aligned segments before matching and emit multiple compressed blocks
- Verified the helper would actually split `repo_Cargo.lock` into four package-aligned segments:
  - `8193 / 8217 / 8197 / 7251`

Rejected:
- exact byte-for-byte no-op on the focused dictionary/lockfile family
- report:
  - [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-rawsplit.md)

Useful conclusion:
- even a real package-aligned multi-block split does not move the lockfile path in this form
- this closes another structural literal-context family without changing the retained baseline

## 2026-06-01 - Rejected `DictionaryText` predefined OF up to 1024 sequences

Change notes:
- Stayed on the retained live baseline for the dictionary/lockfile family:
  - `repo_Cargo.lock = 9,114`
  - `dict_dictionary.bin = 19,668`
  - `generated_go.sum = 151`
  - `generated_poetry.lock = 359`
  - `generated_yarn.lock = 383`
- Tested one sequence-entropy branch in `ruzstd/src/encoding/blocks/compressed.rs`:
  - on the `DictionaryText` path, let OF use predefined tables up to `1024` sequences instead of the generic `16`

Rejected:
- exact byte-for-byte no-op on the focused dictionary/lockfile family
- report:
  - [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/dictof-focused.md)

Useful conclusion:
- the obvious `DictionaryText` predefined-OF window does not move the lockfile path in this form
- this closes another sequence-entropy family without changing the retained baseline

## 2026-06-01 - Rejected lockfile larger dense-match insertion limit

Change notes:
- Stayed on the retained live baseline for the lockfile family:
  - `repo_Cargo.lock = 9,114`
  - `dict_dictionary.bin = 19,668`
  - `generated_go.sum = 151`
  - `generated_poetry.lock = 359`
  - `generated_yarn.lock = 383`
- Tested one parse-representation branch in `ruzstd/src/encoding/match_generator.rs`:
  - for lockfile-like `DictionaryText`, increase the dense post-match suffix insertion limit from `128` to `256`

Rejected:
- exact byte-for-byte no-op on the focused lockfile family
- report:
  - [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-denselimit-focused2.md)

Useful conclusion:
- the lockfile dense post-match insertion family is now bounded on both sides:
  - `64` is worse
  - `256` is a no-op
- this closes another `Cargo.lock` parse-representation family without changing the retained baseline

## 2026-06-01 - Rejected lockfile smaller dense-match insertion limit

Change notes:
- Stayed on the retained live baseline for the rest of the lockfile family:
  - `dict_dictionary.bin = 19,668`
  - `generated_go.sum = 151`
  - `generated_poetry.lock = 359`
  - `generated_yarn.lock = 383`
- Tested one parse-representation branch in `ruzstd/src/encoding/match_generator.rs`:
  - for lockfile-like `DictionaryText`, reduce the dense post-match suffix insertion limit from `128` to `64`

Rejected:
- `repo_Cargo.lock`: `9,114 -> 9,116`
- report:
  - [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-denselimit-focused.md)

Useful conclusion:
- the lockfile path does not want a smaller dense post-match insertion limit
- this closes another lockfile parse-representation family without changing the retained baseline

## 2026-06-01 - Rejected small short-line `ConfigText` current-over-`oldest` displacement

Change notes:
- Stayed on the retained live baseline:
  - `repo_ruzstd_Cargo.toml = 730`
  - `repo_Cargo.toml = 68`
  - `repo_ci.yml = 556`
  - `repo_.gitignore = 166`
- Fresh `repo_ruzstd_Cargo.toml` evidence still showed a parser-side gap versus C:
  - Rust: `literal_section_bytes=570`, `sequence_payload_bytes=142`, `sequences=51`
  - C: `520`, `187`, `71`
- Tested one narrow parser branch in `ruzstd/src/encoding/match_generator.rs`:
  - for small short-line `ConfigText`, keep the current candidate over a farther `oldest` non-repeat candidate unless `oldest` gains at least `2` match bytes

Rejected:
- exact byte-for-byte no-op on the focused `ConfigText` family
- report:
  - [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/config-oldest-focused.md)

Useful conclusion:
- the remaining small `ConfigText` / TOML tail is not waiting on this current-vs-`oldest` displacement rule
- this closes another small `ConfigText` parser family without changing the retained baseline

## 2026-06-01 - Rejected `DictionaryText` adaptive single-stream vs four-stream Huffman literals

Change notes:
- Stayed on the retained live baseline:
  - `repo_Cargo.lock = 9,114`
  - `dict_dictionary.bin = 19,668`
  - `generated_go.sum = 151`
  - `generated_poetry.lock = 359`
  - `generated_yarn.lock = 383`
- Fresh `Cargo.lock` archive inspection still showed the remaining lockfile gap is literal-stream-side:
  - Rust: `literals_payload=6886`, `literals_stream=6855`
  - C: `5975`, `5930`
- Tested one literal-side branch in `ruzstd/src/encoding/blocks/compressed.rs`:
  - on the `DictionaryText` path, compare single-stream vs four-stream Huffman literal layouts and keep the smaller estimated encoding

Rejected:
- exact byte-for-byte no-op on the focused dictionary/lockfile family
- report:
  - [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-streammode-focused.md)

Useful conclusion:
- the remaining `Cargo.lock` gap is not waiting on single-stream vs four-stream Huffman layout selection in this form
- this closes another literal-layout family without changing the retained baseline

## 2026-06-01 - Rejected small short-line `ConfigText` next-position window lookahead

Change notes:
- Stayed on the retained live baseline:
  - `repo_ruzstd_Cargo.toml = 730`
  - `repo_ci.yml = 556`
  - `repo_.gitignore = 166`
- Fresh matcher diagnostics on `repo_ruzstd_Cargo.toml` showed:
  - `window_current_newest = 22`
  - `window_current_oldest = 28`
  - `window_next_position_* = 0`
- Tested one known-file-type parser branch in `ruzstd/src/encoding/match_generator.rs`:
  - enable next-position window lookahead for small short-line `ConfigText`

Rejected:
- exact byte-for-byte no-op across `broad-local`
- no fixture bytes moved at all
- report:
  - [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/config-nextwindow-broad-local.md)

Useful conclusion:
- the remaining small `ConfigText` / TOML tail is not waiting on next-position window lookahead in this form
- this closes another known-file-type parser family without changing the retained baseline

## 2026-06-01 - Rejected tiny single-stream `ConfigText` actual-byte Huffman table search

Change notes:
- Stayed on the retained live baseline:
  - `repo_.gitignore = 166`
  - `repo_ruzstd_Cargo.toml = 730`
- Fresh `.gitignore` archive inspection still showed a pure literal-side tail:
  - Rust: `literals_payload=131`, `literals_table_desc=22`, `literals_stream=109`
  - C: `129`, `24`, `105`
- Tested one literal-side branch in `ruzstd/src/huff0/huff0_encoder.rs` and [compressed.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/blocks/compressed.rs):
  - for tiny single-stream `ConfigText` literals, choose exact Huffman tables by actual encoded bytes instead of the estimate

Rejected:
- exact byte-for-byte no-op across `broad-local`
- no fixture bytes moved at all
- report:
  - [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/config-actualhuff-broad-local.md)

Useful conclusion:
- the remaining tiny `ConfigText` literal tail is not waiting on actual-byte re-ranking of the current exact Huffman candidate set
- this closes another literal-selection family without changing the retained baseline

## 2026-06-01 - Rejected small short-line `ConfigText` current-entry `second_newest`

Change notes:
- Stayed on the retained live baseline:
  - `repo_ruzstd_Cargo.toml = 730`
  - `repo_ci.yml = 556`
  - `repo_.gitignore = 166`
- Tested one known-file-type parser branch in `ruzstd/src/encoding/match_generator.rs`:
  - enable current-entry `second_newest` for small short-line `ConfigText` blocks

Rejected:
- exact byte-for-byte no-op across `broad-local`
- no fixture bytes moved at all
- report:
  - [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/config-secondnewest-broad-local.md)

Useful conclusion:
- the remaining small `ConfigText` / TOML tail is not waiting on current-entry `second_newest` in this form
- this closes another known-file-type parser family without changing the retained baseline

## 2026-06-01 - Rejected lockfile-only no-backward-extension parse branch

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,114`
- Tested one lockfile-specific parse-shape branch in `ruzstd/src/encoding/match_generator.rs`:
  - disable backward match extension for lockfile-like `DictionaryText`

Rejected:
- exact byte-for-byte no-op across the focused lockfile family:
  - `repo_Cargo.lock`: stayed `9,114`
  - `generated_go.sum`: stayed `151`
  - `generated_poetry.lock`: stayed `359`
  - `generated_yarn.lock`: stayed `383`
- restore check:
  - [focused restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-nobackextend-restore.md)

Useful conclusion:
- the remaining `Cargo.lock` gap is not waiting on backward match extension in this form
- this closes another lockfile parse-shape branch on the retained path

## 2026-06-01 - Rejected lockfile-only zero-literal nonrepeat extra floor

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,114`
- Tested one narrow lockfile-specific matcher rule in `ruzstd/src/encoding/match_generator.rs`:
  - zero-literal, non-repeat window candidates on the lockfile path must be `6` bytes long instead of `5`

Rejected:
- focused lockfile family result:
  - `repo_Cargo.lock`: `9,114 -> 9,143`
  - `generated_go.sum`: stayed `151`
  - `generated_poetry.lock`: stayed `359`
  - `generated_yarn.lock`: stayed `383`
- restore check:
  - [focused restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-zero-literal-floor-restore.md)

Useful conclusion:
- the live `Cargo.lock` histogram really does overuse zero-literal sequences versus C
- but a blunt extra floor on zero-literal non-repeat window matches over-cuts the retained lockfile parser and makes compression worse

## 2026-06-01 - Rejected fixed newline-aligned multi-block split for lockfile-like `DictionaryText`

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,114`
- Tested one structural lockfile branch in `ruzstd/src/encoding/levels/fastest.rs`:
  - for large lockfile-like `DictionaryText` blocks, split at fixed newline-aligned segment boundaries around `8 KiB`

Rejected:
- exact byte-for-byte no-op across the focused lockfile family:
  - `repo_Cargo.lock`: stayed `9,114`
  - `generated_go.sum`: stayed `151`
  - `generated_poetry.lock`: stayed `359`
  - `generated_yarn.lock`: stayed `383`
- restore check:
  - [focused restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-fixedsplit-restore-seq.md)

Useful conclusion:
- `Cargo.lock` is not waiting on a simple fixed multi-block split either
- this closes another structural split family for the retained lockfile path

## 2026-06-01 - Rejected rank-limited candidate in DictionaryText exact Huffman search

Change notes:
- Stayed on the retained live baseline for the `DictionaryText` family.
- Tested one deeper literal-model branch in `ruzstd/src/huff0/huff0_encoder.rs`:
  - when `build_smallest_from_counts()` searches exact Huffman tables for non-flat distributions, also consider the `rank_limited_weights()` candidate

Rejected:
- exact byte-for-byte no-op across the focused live `DictionaryText` family:
  - `repo_Cargo.lock`: stayed `9,114`
  - `dict_dictionary.bin`: stayed `20,160`
  - `generated_go.sum`: stayed `151`
- restore check:
  - [focused restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/dictionary-huff-restore.md)

Useful conclusion:
- the remaining `DictionaryText` gap is not waiting on the rank-limited weight family being added to the current exact Huffman search
- this closes another literal-model branch for both `Cargo.lock` and `dict_dictionary.bin`

## 2026-06-01 - Rejected lockfile zero-literal `second_newest` ordering change

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,114`
- Tested one narrow lockfile-specific parse rule:
  - keep the retained `second_newest-before-newest` ordering only when literals are pending
  - on zero-literal positions, fall back to the normal `newest`-first order

Rejected:
- focused `Cargo.lock` regressed immediately:
  - `repo_Cargo.lock`: `9,114 -> 9,164`
- restore check:
  - [focused restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-zero-lit-restore.md)

Useful conclusion:
- zero-literal `second_newest` wins are not the main reason `Cargo.lock` lags C
- this closes another zero-literal parse-shape branch on the retained lockfile path

## 2026-06-01 - Rejected lockfile zero-literal window displacement rule and kept literal-payload diagnostics

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,114`
- Added a retained archive-inspector diagnostic in `ruzstd/src/tests/mod.rs`:
  - compressed literal blocks now print:
    - `literals_table_desc`
    - `literals_stream`
- Tested one narrow lockfile-specific matcher rule:
  - keep the current non-repeat candidate over a zero-literal non-repeat window candidate unless the zero-literal candidate gains at least `2` match bytes

Rejected:
- the lockfile zero-literal window displacement rule was an exact no-op on focused `Cargo.lock`
  - `repo_Cargo.lock`: stayed `9,114`
- restore check:
  - [focused restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-zero-lit-restore.md)

Useful new evidence from the retained inspector:
- current `Cargo.lock`:
  - `literals_payload=6886`
  - `literals_table_desc=25`
  - `literals_stream=6855`
- C `Cargo.lock`:
  - `literals_payload=5975`
  - `literals_table_desc=39`
  - `literals_stream=5930`

Useful conclusion:
- the `Cargo.lock` literal gap is not a Huffman table-description problem
- C actually spends more bytes on the table description
- the remaining loss is in the coded literal stream itself, so the next credible branch is still parse/literal-shape oriented, not another table-header tweak

## 2026-06-01 - Rejected Huffman weight-table FSE max-log `7`

Change notes:
- Stayed on the retained live baseline with adaptive Huffman weight-table FSE max-log `5/6`.
- Tested one narrow literal-side branch in `ruzstd/src/huff0/huff0_encoder.rs`:
  - extend the retained weight-table search to also consider FSE max-log `7`

Rejected:
- this did not just fail to help, it produced invalid output on the focused lockfile family
- `tools/benchmark_zstd.py` failed decode verification on:
  - `generated_go.sum.current.zst`
- restore check after reverting the branch returned the focused lockfile family to the retained baseline:
  - `repo_Cargo.lock = 9,114`
  - `generated_go.sum = 151`
  - `generated_poetry.lock = 362`
  - `generated_yarn.lock = 390`

Reports:
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-huffweight7-restore.md)

Useful conclusion:
- the retained Huffman weight-table FSE search should stay bounded at max-log `5/6`
- do not retry `>6` in this branch without evidence that the emitted representation is valid

## 2026-06-01 - Rejected lockfile package-boundary partition candidates

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,114`
- Tested two structural partition branches inside the retained `DictionaryText` lockfile path:
  1. split once at the `[[package]]` boundary nearest the midpoint
  2. split at the `[[package]]` boundaries nearest the quartiles

Rejected:
- both were exact byte-for-byte no-ops on the focused lockfile family:
  - `repo_Cargo.lock`: stayed `9,114`
  - `generated_go.sum`: stayed `151`
  - `generated_poetry.lock`: stayed `362`
  - `generated_yarn.lock`: stayed `390`
- reports:
  - [focused midpoint split](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-split-focused.md)
  - [restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-split-restore.md)

Restore:
- rebuilt source returned to:
  - `repo_Cargo.lock = 9,114`

Useful conclusion:
- the retained `Cargo.lock` gap is not waiting on package-boundary block partitioning in this form
- this closes another structural parse family, not just another matcher threshold

## 2026-06-01 - Retained adaptive Huffman weight-table FSE max-log choice

Change notes:
- Kept the existing `huff0` branch in [ruzstd/src/huff0/huff0_encoder.rs](/home/bsutton/git/zstd-rs/ruzstd/src/huff0/huff0_encoder.rs):
  - for Huffman weight tables longer than `16` symbols, compare FSE weight-table encodings at max-log `6` and `5`
  - emit the shorter byte sequence
- Added focused unit coverage for the helper logic.

Retained:
- focused tiny known-file-type fixtures stayed byte-identical:
  - `repo_.gitignore`: `172`
  - `dict_talk.service`: `160`
  - `repo_ruzstd_Cargo.toml`: `730`
  - `repo_ci.yml`: `556`
- corrected `broad-local` changed only two already-winning fixtures:
  - `build_ruzstd-cli`: `866,125 -> 866,118`
  - `repo_match_generator.rs`: `27,879 -> 27,877`
- corrected broad-local bytes-above-C on losers stayed unchanged:
  - `1,182 -> 1,182`

Reports:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/huffweight-focused.md)
- [corrected broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-huffweight-maxlog5-broad-local.md)

Useful conclusion:
- this does not reduce the remaining known-file-type gap
- but it is broad-local clean, slightly improves two already-winning fixtures, and is now explicitly covered by tests

## 2026-06-01 - Rejected `DictionaryText` small-sequence LL/ML max-log lockfile branches

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,114`
- Tested two narrow sequence-entropy variants for lockfile-scale `DictionaryText` blocks:
  1. LL/ML FSE max-log `8`
  2. LL/ML FSE max-log `7`

Rejected:
- both were exact byte-for-byte no-ops on focused `Cargo.lock`
- reports:
  - [LL/ML max-log 8](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-dictllml8-focused.md)
  - [LL/ML max-log 7](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-dictllml7-focused.md)

Restore:
- [sequential restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-dictllmlmaxlog-seq.md)
- rebuilt source returned to:
  - `repo_Cargo.lock = 9,114`

Useful conclusion:
- the lockfile gap is not waiting on a smaller LL/ML FSE max-log in this family
- this small-sequence LL/ML max-log line is closed in the tested points

## 2026-06-01 - Fully bounded the retained lockfile current-vs-`oldest` family: `+1` and `+3` are both worse

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,114`
- After already rejecting `+3`, tested the other side of the same family:
  - require `oldest` to gain at least `1` match byte instead of `2`

Rejected:
- lockfile current-vs-`oldest` displacement `+1`
- report: [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-oldestgain1-after-oldestgain2-focused.md)
- result:
  - `repo_Cargo.lock`: `9,114 -> 9,116`

Restore:
- [sequential restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-oldestgain1-seq.md)
- rebuilt source returned to:
  - `repo_Cargo.lock = 9,114`

Useful conclusion:
- the lockfile current-vs-`oldest` family is now fully bounded on the active parser shape:
  - `+1` is worse
  - `+2` is the retained best point
  - `+3` is worse

## 2026-06-01 - Rejected wider same-start smaller-offset rule on the retained lockfile parser

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,114`
- Retested one older lockfile family against the current parser shape:
  - widen the same-start smaller-offset rule from `1` byte of allowed match loss to `2`
  - scope: lockfile-like `DictionaryText` only

Rejected:
- wider lockfile same-start smaller-offset rule
- report: [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-offset2-after-oldestgain2-focused.md)
- result:
  - `repo_Cargo.lock`: `9,114 -> 9,117`

Restore:
- [sequential restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-offset2-retest-seq.md)
- rebuilt source returned to:
  - `repo_Cargo.lock = 9,114`

Useful conclusion:
- this older offset-aware family still does not become valid on the new lockfile parser shape
- the retained same-start smaller-offset rule is still the useful edge

## 2026-06-01 - Rejected two more narrow lockfile branches on the retained `oldest +2` baseline

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,114`
- Tested two narrower lockfile-specific follow-ups on the active parser shape:
  1. zero-literal repeat-margin bonus
  2. current-vs-`second_newest` displacement

Rejected:
- both were exact no-ops in matcher diagnostics on live `Cargo.lock`
- no focused size run was worth promoting after the diagnostic no-op

Restore:
- [sequential restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-secondnewest-noop-seq.md)
- rebuilt source returned to:
  - `repo_Cargo.lock = 9,114`

Useful conclusion:
- the active lockfile parser does not want another narrow repeat-side bonus
- it also does not want a current-vs-`second_newest` displacement rule in this form

## 2026-06-01 - Bounded the retained lockfile current-vs-`oldest` family: `+2` is good, `+3` is worse

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,114`
- Tested the next obvious bound on the same lockfile-specific current-vs-`oldest` rule:
  - require `oldest` to gain at least `3` match bytes instead of `2`

Rejected:
- lockfile current-vs-`oldest` displacement `+3`
- report: [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-oldestgain3-after-oldestgain2-focused.md)
- result:
  - `repo_Cargo.lock`: `9,114 -> 9,117`

Restore:
- [sequential restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-oldestgain3-seq.md)
- rebuilt source returned to:
  - `repo_Cargo.lock = 9,114`

Useful conclusion:
- the lockfile `oldest`-displacement family is now bounded on the active parser shape:
  - `+2` retained best point
  - `+3` is worse

## 2026-06-01 - Retained lockfile-specific current-vs-`oldest` displacement on top of `second_newest`-first probing

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,116`
- Tested one narrower lockfile-specific current-window scoring rule:
  - keep the current non-repeat candidate over a farther `oldest` candidate unless `oldest` gains at least `2` match bytes

Retained:
- lockfile-specific current-vs-`oldest` displacement
- report: [corrected broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-oldestgain2-after-secondnewestfirst-broad-local.md)
- retained binary: [ruzstd-cli-level1-lockfile-oldestgain2-after-secondnewestfirst-retained](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-lockfile-oldestgain2-after-secondnewestfirst-retained)

Result:
- `repo_Cargo.lock`: `9,116 -> 9,114`
- every other corrected-suite fixture stayed byte-identical
- corrected broad-local bytes-above-C on losers:
  - `1,184 -> 1,182`

Useful matcher result:
- sequences stayed `821`
- `window_current_oldest[0]`: `211 -> 196`
- `window_current_second_newest[0]`: `100 -> 105`
- `window_current_newest[0]`: `413 -> 421`

Useful inspect result:
- `sequence_payload_bytes`: `2,213 -> 2,208`
- `of_extra_bits`: `6,944 -> 6,898`
- `literal_section_bytes`: `6,883 -> 6,886`

Conclusion:
- the active lockfile parser still had a small useful `oldest`-side scoring gain left
- this family is still worth exploring only in very narrow forms on top of the current lockfile parser shape

## 2026-06-01 - Retained lockfile-specific current-window probe order: `second_newest` before `newest`

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,170`
- Tested one narrower lockfile-specific current-window parser branch:
  - for lockfile-like `DictionaryText`, probe current-entry `second_newest` before `newest`
  - keep `oldest` last

Retained:
- lockfile-specific `second_newest`-before-`newest` current-window probing
- report: [corrected broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-secondnewestfirst-broad-local.md)
- retained binary: [ruzstd-cli-level1-lockfile-secondnewestfirst-retained](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-lockfile-secondnewestfirst-retained)

Result:
- `repo_Cargo.lock`: `9,170 -> 9,116`
- every other corrected-suite fixture stayed byte-identical
- corrected broad-local bytes-above-C on losers:
  - `1,238 -> 1,184`

Useful matcher result:
- sequences: `848 -> 821`
- `window_current_second_newest[0]`: `44 -> 100`
- `window_current_newest[0]`: `475 -> 413`

Useful inspect result:
- `sequence_payload_bytes`: `2,258 -> 2,213`
- `of_extra_bits`: `7,164 -> 6,944`
- `literal_section_bytes`: `6,892 -> 6,883`

Conclusion:
- the lockfile parser still had a real current-window ordering win available
- `second_newest` is now strong enough on this family that it should be probed before `newest`

## 2026-06-01 - Rejected lockfile-specific next-position window lookahead

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,170`
- Tested one more adjacent-position parser branch:
  - let lockfile-like `DictionaryText` reach the existing next-position window lookahead gate

Rejected:
- lockfile-specific next-position window lookahead
- report: [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-nextwindow-broad-local.md)
- result:
  - exact byte-for-byte no-op
  - `repo_Cargo.lock`: stayed `9,170`

Restore:
- [rebuilt restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-nextwindow.md)
- rebuilt source returned to:
  - `repo_Cargo.lock = 9,170`

Useful conclusion:
- the lockfile path does not benefit from next-position window lookahead either
- this closes another adjacent-position family on the retained lockfile parser shape

## 2026-06-01 - Rejected lockfile oldest-first current-window probing

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,170`
- Tested one best-level parser behavior selectively on the lockfile path:
  - current-window oldest-first probing

Rejected:
- lockfile oldest-first current-window probing
- report: [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-oldestfirst-focused.md)
- result:
  - `repo_Cargo.lock`: `9,170 -> 9,180`

Restore:
- [rebuilt restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-oldestfirst.md)
- rebuilt source returned to:
  - `repo_Cargo.lock = 9,170`

Useful conclusion:
- the lockfile path does not want oldest-first current-window probing
- this closes another best-level parser behavior on that family

## 2026-06-01 - Rejected lockfile-only `third_newest` current-entry sidecar

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,170`
- Tested a different current-entry representation:
  - lockfile-only `third_newest` sidecar
  - probed after the retained `second_newest` lockfile path

Rejected:
- lockfile-only `third_newest` current-entry sidecar
- report: [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-thirdnewest-focused.md)
- result:
  - exact byte-for-byte no-op
  - `repo_Cargo.lock`: stayed `9,170`

Useful matcher result:
- diagnostics stayed byte-for-byte identical too
- so the extra current-entry representation never changed the chosen parse on live `Cargo.lock`

Restore:
- [rebuilt restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-thirdnewest.md)
- rebuilt source returned to:
  - `repo_Cargo.lock = 9,170`

Useful conclusion:
- the lockfile gap is not waiting on a `third_newest` current-entry representation
- current-entry `newest` / `second_newest` / `oldest` is effectively bounded for this family in the current matcher design

## 2026-06-01 - Rejected lockfile same-end repeat promotion

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,170`
- Tested one narrower offset-side follow-up:
  - for lockfile-like `DictionaryText`, prefer a repeat-offset candidate over a non-repeat when both end at the same byte and the repeat loses at most one match byte

Rejected:
- lockfile same-end repeat promotion
- report: [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-repeat-sameend-focused.md)
- result:
  - exact byte-for-byte no-op
  - `repo_Cargo.lock`: stayed `9,170`

Useful matcher result:
- diagnostics stayed byte-for-byte identical too
- so this rule never fired on the live lockfile data

Restore:
- [rebuilt restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-repeat-sameend.md)
- rebuilt source returned to:
  - `repo_Cargo.lock = 9,170`

Useful conclusion:
- the remaining lockfile `of_code=0` gap is not waiting on this same-end repeat promotion family
- do not retry this branch in the current matcher representation

## 2026-06-01 - Rejected `DictionaryText` 1-stream vs 4-stream literal choice up to 16 KiB

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,170`
- Tested one more literal-side follow-up:
  - for `DictionaryText`, compare 1-stream vs 4-stream Huffman up to `16 KiB` of literals and keep the smaller estimate

Rejected:
- `DictionaryText` 1-stream vs 4-stream literal choice up to `16 KiB`
- report: [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-streamchoice16k-focused.md)
- result:
  - exact byte-for-byte no-op
  - `repo_Cargo.lock`: stayed `9,170`

Useful inspect result:
- the whole archive stayed byte-identical:
  - `literal_section_bytes`: stayed `6,892`
  - `sequence_payload_bytes`: stayed `2,258`
  - `sequences`: stayed `848`
- so the current encoder already kept the same literal-stream choice on this file

Restore:
- [rebuilt restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-streamchoice16k.md)
- rebuilt source returned to:
  - `repo_Cargo.lock = 9,170`

Useful conclusion:
- the lockfile gap is not hiding behind a 1-stream vs 4-stream literal-mode choice in the current encoder
- next credible lockfile literal-side work needs a different representation or table choice than this stream-mode family

## 2026-06-01 - Rejected broader predefined LL/ML tables for `DictionaryText`

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,170`
- Tested one sequence-entropy follow-up aimed at the dominant known-file-type loser:
  - allow `DictionaryText` LL/ML predefined tables up to `1024` sequences at level 1

Rejected:
- broader predefined LL/ML window for `DictionaryText`
- report: [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-predef1024-focused.md)
- result:
  - `repo_Cargo.lock`: `9,170 -> 9,408`

Useful inspect result:
- code histograms stayed identical
- only table/payload cost changed:
  - `sequence_payload_bytes`: `2,258 -> 2,496`
- so this branch only inflated table-description cost; it did not improve the parse at all

Restore:
- [rebuilt restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-predef1024.md)
- rebuilt source returned to:
  - `repo_Cargo.lock = 9,170`

Useful conclusion:
- the lockfile gap is not solved by forcing broader predefined LL/ML use on `DictionaryText`
- next credible lockfile entropy work needs a different table/representation choice, not a bigger predefined-table window

## 2026-06-01 - Closed two more structural `Cargo.lock` branches on the retained `step 2` baseline

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,170`
- Tested two structural lockfile-specific branches that were still ambiguous on the active parser shape.

Rejected:
1. lockfile-specific current-entry long-hash with the gate and allocation actually enabled
- report: [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-longhash-gatefix-focused.md)
- result:
  - exact byte-for-byte no-op
  - `repo_Cargo.lock`: stayed `9,170`
- useful diagnostic:
  - even with the path admitted, `current_long_hash_found` stayed `0`

2. lockfile-like `DictionaryText` uses the general text parser path instead of the special repeat-only text pipeline
- report: [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-nobesttext-focused.md)
- result:
  - exact byte-for-byte no-op
  - `repo_Cargo.lock`: stayed `9,170`
- useful diagnostic:
  - matcher counts stayed byte-for-byte identical too

Useful conclusion:
- the earlier lockfile long-hash branch is now fully closed, not just “suspected no-op”
- the remaining `Cargo.lock` gap is also not hiding behind the special DictionaryText text-repeat pipeline
- the next credible lockfile branch is narrower again: sequence-entropy or a different parse representation, not these two parser toggles

## 2026-06-01 - Rejected three more lockfile-specific follow-ups on the retained `step 2` baseline

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,170`
- Retested two repeat-side ideas against the active post-gate-fix parser shape, then tried one more lockfile-specific floor cut.

Rejected:
1. lockfile-specific repeat margin `+1`
- report: [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-repeatmargin-focused.md)
- result:
  - exact byte-for-byte no-op
  - `repo_Cargo.lock`: stayed `9,170`

2. lockfile-specific same-start repeat preference with material non-repeat offset-code savings
- report: [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-repeataware-focused.md)
- result:
  - exact byte-for-byte no-op
  - `repo_Cargo.lock`: stayed `9,170`

3. lockfile-specific short-line non-repeat floor `5 -> 6`
- report: [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-floor6-focused.md)
- result:
  - hard regression
  - `repo_Cargo.lock`: `9,170 -> 9,246`
- useful diagnostic:
  - it also killed the active `second_newest` lockfile wins entirely:
    - `window_current_second_newest[0]`: `44 -> 0`

Restore:
- [rebuilt restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-floor6.md)
- rebuilt source returned to:
  - `repo_Cargo.lock = 9,170`

Useful conclusion:
- on the active post-gate-fix lockfile parser shape:
  - blanket repeat-margin changes are still dead
  - same-start repeat-aware scoring is still dead
  - stronger lockfile floors are actively harmful because they suppress the retained `second_newest` path

## 2026-06-01 - Bounded the retained lockfile probe-step family: `2` is good, `3` is worse

Change notes:
- After retaining the lockfile-specific probe step `2` point, tested the next obvious bound:
  - lockfile-specific probe step `3`
  - compared directly against the retained `step 2` binary

Rejected:
- lockfile-specific probe step `3`
- report: [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-step3-after-step2-broad-local.md)
- result:
  - `repo_Cargo.lock`: `9,170 -> 9,223`

Restore:
- [rebuilt restore check](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-level1-lockfile-step2-after-step3.md)
- rebuilt source returned to:
  - `repo_Cargo.lock = 9,170`

Useful conclusion:
- the lockfile probe-density family is now bounded on the active parser shape:
  - dense step `1`: worse than retained
  - step `2`: retained best point
  - step `3`: worse

## 2026-06-01 - Retained lockfile-specific probe step `2` on top of the `second_newest` gate-fix baseline

Change notes:
- Revisited the earlier rejected lockfile probe-step family, but only after the retained `second_newest` gate fix made the lockfile sidecar actually active.
- Narrow runtime change in [match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs):
  - keep the retained lockfile-specific `DictionaryText` `second_newest` path
  - but use no-match probe step `2` instead of the generic dictionary dense step `1` only for content-detected `Cargo.lock`-like text

Fresh diagnostic result on the retained new point:
- `repo_Cargo.lock` matcher diagnostics:
  - `total_sequences`: `883 -> 848`
  - `repeat_current`: `48 -> 99`
  - `window_current_second_newest[0]`: `55 -> 44`
- fresh archive inspect:
  - `compressed_bytes`: `9,185 -> 9,170`
  - `literal_section_bytes`: `6,747 -> 6,892`
  - `sequence_payload_bytes`: `2,418 -> 2,258`
  - `sequences`: `883 -> 848`
  - `of_extra_bits`: `8,218 -> 7,164`
  - `of_codes 0`: now `70`

Retained reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-step2-after-gatefix-broad-local.md)
- [focused fast/repeat screen](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-step2-after-gatefix-fast.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-lockfile-step2-after-gatefix-retained)

Result versus the retained `secondnewest-gatefix` baseline:
- `repo_Cargo.lock`: `9,185 -> 9,170`
- every other corrected-suite fixture stayed byte-identical
- corrected broad-local bytes-above-C on losers improved:
  - `1,253 -> 1,238`

Useful conclusion:
- the older “lockfile step 2” family was exhausted on the pre-bug baseline, but it becomes a real win once the lockfile `second_newest` path is actually active
- the improvement comes from materially reducing sequence payload and offset-bit cost, even though literal payload rises a bit

## 2026-06-01 - Rejected two more lockfile-specific follow-ups after the retained `second_newest` gate fix

Change notes:
- Stayed on the retained `second_newest` gate-fix baseline for `Cargo.lock`-like `DictionaryText`:
  - `repo_Cargo.lock = 9,185`
- Tried one matcher-side follow-up and one entropy-side follow-up, both only for the lockfile family.

Rejected:
1. lockfile-only fully dense post-match suffix insertion
- report: [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-denseinsert-broad-local.md)
- result:
  - exact byte-for-byte no-op
  - `repo_Cargo.lock`: stayed `9,185`

2. lockfile-only `offset_table_max_log = 8`
- report: [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-oflog8-broad-local.md)
- result:
  - exact byte-for-byte no-op
  - `repo_Cargo.lock`: stayed `9,185`

Useful conclusion:
- the remaining `Cargo.lock` gap is not waiting on denser current-block suffix insertion
- it is also not fixed by simply undoing the retained `DictionaryText oflog7` choice for the lockfile family
- next credible lockfile work still needs a different parse/entropy representation, not another local indexing or OF-log toggle

## 2026-06-01 - Retained `second_newest` probe-gate fix, plus cached lockfile classification

Change notes:
- Found a real bug in [match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs):
  - the retained lockfile-specific `second_newest` sidecar and the older Fastest small-block `second_newest` path were being tracked
  - but the actual probe sites were still guarded by `use_second_newest_probe` alone
  - at level 1 Fastest, that meant the tracked sidecar was not being consulted where intended
- Fixed the probe sites to use `should_track_second_newest_for_current_entry()`.
- Then cached the lockfile classification per block so the new gate did not rescan whole `DictionaryText` blocks at hot matcher sites.

Fresh diagnostic result on the retained live `Cargo.lock` path:
- [matcher inspect](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-secondnewest-gatefix-fast.md)
- `window_current_second_newest[0]` is now active on `repo_Cargo.lock`:
  - `55` wins total
  - `26` zero-literal
  - `29` with literals

Retained reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-secondnewest-gatefix-broad-local.md)
- [focused fast/repeat screen](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-secondnewest-gatefix-fast.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-secondnewest-gatefix-retained)

Result versus the retained `lockfile-secondnewest` baseline:
- `repo_Cargo.lock`: `9,197 -> 9,185`
- `build_ruzstd-cli`: `862,752 -> 854,529`
- `decodecorpus_z000028`: `98,381 -> 95,230`
- `decodecorpus_z000033`: `532,632 -> 530,433`
- `decodecorpus_z000079`: `7,321 -> 7,322`
- `dict_dictionary.bin`: unchanged at `20,160`

Corrected `broad-local` summary vs C `zstd -1` is now:
- better / worse / equal: `32 / 11 / 4`
- bytes-above-C on losing fixtures: `1,264 -> 1,253`

Useful conclusion:
- this was not another heuristic branch; it was a real admission bug in the retained `second_newest` family
- the lockfile-specific sidecar is now actually consulted
- the older Fastest small-block `second_newest` family is now also active again through the intended gate

## 2026-06-01 - Rejected two more structural `Cargo.lock` matcher branches after refreshing the retained archive shape

Change notes:
- Refreshed the live retained `Cargo.lock` archive shape from the current `lockfile-secondnewest` baseline:
  - Rust retained:
    - `compressed_bytes=9197`
    - `literal_section_bytes=6766`
    - `sequence_payload_bytes=2411`
    - `decoded_literals=9756`
    - `sequences=879`
    - `match_bytes=22102`
  - C `zstd -1` remains:
    - `compressed_bytes=8088`
    - `literal_section_bytes=5975`
    - `sequence_payload_bytes=2092`
    - `decoded_literals=10360`
    - `sequences=784`
    - `match_bytes=21498`
- That confirmed the retained `second_newest` win reduced literals materially, but the remaining gap is still broad, not a tiny tail.

Rejected:
1. lockfile-like `DictionaryText` current-entry long-hash
- report: [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-longhash-broad-local.md)
- result:
  - exact byte-for-byte no-op on corrected `broad-local`
  - `repo_Cargo.lock`: stayed `9,197`

2. lockfile-like `DictionaryText` current-vs-window displacement
- report: [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-displacement-broad-local.md)
- result:
  - exact byte-for-byte no-op on corrected `broad-local`
  - `repo_Cargo.lock`: stayed `9,197`

Restore:
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-level1-lockfile-secondnewest-broad-local.md)

Useful conclusion:
- the first structural `Cargo.lock` win was the current-entry `second_newest` sidecar
- the nearby current-entry long-hash and current-vs-window displacement branches do not move the corrected suite in this form
- the next credible `Cargo.lock` branch is likely sequence-entropy or a different parse representation, not another small current-window rule

## 2026-06-01 - Rejected lockfile-specific repeat-margin increase

Change notes:
- After comparing retained `Cargo.lock` sequence histograms against C, the clearest mismatch was repeat-offset usage:
  - C showed a large `of_code=0` population
  - the retained Rust path still had none
- Tried the narrowest repeat-side follow-up in [match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs):
  - only on `Cargo.lock`-like `DictionaryText`
  - widen the repeat-vs-normal match margin by `1`

Source report:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-repeatbias-broad-local.md)

Why it was rejected:
- exact byte-for-byte no-op on corrected `broad-local`
- `repo_Cargo.lock`: stayed `9,197`

Useful conclusion:
- the remaining lockfile gap is not another local repeat-margin issue
- do not retry this repeat-bias family in the same form

## 2026-06-01 - Rejected lockfile `ip+1` repeat lookahead after adding file-type-aware matcher inspection

Change notes:
- Kept a useful test-only diagnostics improvement in [match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs):
  - the ignored matcher inspectors now accept `RUZSTD_MATCHER_FILE_TYPE`
  - that let me inspect the live retained `Cargo.lock` path as `DictionaryText`
- Diagnostics on the retained `Cargo.lock` path showed:
  - very few repeat wins overall:
    - first/second/third current repeats: `18 / 24 / 8`
  - zero `ip+1` repeat promotions
  - no current-entry long-hash activity
  - emitted window wins are still dominated by current-entry `newest` / `oldest`
- That made lockfile-specific `ip+1` repeat lookahead the next narrow repeat-side branch.

Source report:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-nextrep-broad-local.md)

Why it was rejected:
- exact byte-for-byte no-op on corrected `broad-local`
- `repo_Cargo.lock`: stayed `9,197`

Useful conclusion:
- even though retained `Cargo.lock` differs strongly from C on repeat-offset usage, the missing win is not unlocked by simply enabling the existing `ip+1` repeat-lookahead path
- do not retry this lockfile next-position repeat branch in the same form

## 2026-06-01 - Rejected lockfile-specific repeat-vs-non-repeat same-start preference

Change notes:
- Used the new file-type-aware matcher inspection on retained `repo_Cargo.lock` and confirmed:
  - repeat wins are sparse
  - window wins are still dominated by current-entry `newest` / `oldest`
- Tried the narrowest direct repeat-offset scoring follow-up in [match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs):
  - only on `Cargo.lock`-like `DictionaryText`
  - when a repeat and non-repeat candidate start at the same place, prefer the repeat if it loses at most 1 match byte and saves at least 2 offset-code bits

Source report:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-repeat-aware-broad-local.md)

Why it was rejected:
- exact byte-for-byte no-op on corrected `broad-local`
- `repo_Cargo.lock`: stayed `9,197`

Useful conclusion:
- the retained lockfile gap is not another same-start repeat-vs-non-repeat scoring problem
- do not retry this repeat-aware tie-break in the same form

## 2026-06-01 - Retained `Cargo.lock`-like `DictionaryText` current-entry `second_newest`

Change notes:
- Retained a new internal lockfile-specific matcher path in [ruzstd/src/encoding/match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs):
  - keep the public API unchanged
  - inside `DictionaryText`, detect `Cargo.lock`-like short-line text by content
  - only for that path, enable the existing current-entry `second_newest` sidecar at level 1
- Added focused matcher tests covering:
  - lockfile-like `DictionaryText` tracking the sidecar
  - binary/non-lockfile `DictionaryText` still not tracking it

Source reports:
- [retained A/B on corrected broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-secondnewest-broad-local.md)
- refreshed baselines:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
  - [fast](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-fast.md)

Retained result:
- `repo_Cargo.lock`: `9,240 -> 9,197`
- every other corrected-suite fixture stayed byte-identical
- corrected-suite broad-local summary vs C after retention:
  - better / worse / equal: `32 / 11 / 4`
  - bytes-above-C on losing fixtures: `1,307 -> 1,264`

Useful conclusion:
- the remaining `Cargo.lock` gap was not another threshold problem
- a different current-entry representation does help that family
- this is the first retained runtime change that materially narrows the corrected-suite known-file-type gap after the suite collision fix

## 2026-06-01 - Retained suffix-based named-file matching and `Cargo.lock -> DictionaryText`

Change notes:
- Retained a file-type classification improvement in [ruzstd/src/encoding/mod.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/mod.rs):
  - well-known filename families now match by suffix when preceded by a clear separator (`_`, `-`, or `.`)
  - this lets synthetic benchmark names like `repo_.gitignore` and `repo_Cargo.lock` hit the same named-file policy they would in real usage
- On top of that retained mapper fix, kept:
  - `Cargo.lock -> DictionaryText`

Source reports:
- [retained Cargo.lock DictionaryText A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargolockdict-live-broad-local.md)
- [rejected Cargo.lock CodeText A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargolockcode-live-broad-local.md)
- refreshed baselines:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
  - [fast](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-fast.md)

Retained result:
- `repo_Cargo.lock`: `9,255 -> 9,240`
- `Cargo.lock -> CodeText` matched the same gain at best on bytes in one-shot A/B, but `DictionaryText` is the better retained semantic fit for the current lockfile-specific path
- corrected-suite broad-local summary vs C after retention:
  - better / worse / equal: `31 / 12 / 4`
  - bytes-above-C on losing fixtures: `1,426 -> 1,411`

Useful conclusion:
- the filename-based policy layer is now being exercised more faithfully on synthetic benchmark fixtures
- `Cargo.lock` is still the dominant known-file-type loss, but the first real file-type policy win on the corrected suite is now retained

## 2026-06-01 - Corrected broad-local fixture naming collisions and expanded known-file-type coverage

Change notes:
- Fixed the `broad-local` suite generator in [tools/prepare_benchmark_suites.py](/home/bsutton/git/zstd-rs/tools/prepare_benchmark_suites.py):
  - repo-source fixture names are now unique
  - duplicate names like `repo_Cargo.toml` no longer overwrite each other on disk
- Added more explicit known-file-type fixtures:
  - `Cargo.lock`
  - `.github/workflows/ci.yml`
  - `ruzstd/fuzz/.gitignore`
  - `ruzstd/fuzz/Cargo.toml`

Generated artifacts:
- [broad-local manifest](/home/bsutton/git/zstd-rs/benchmarks/manifests/broad-local.json)
- current baselines:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
  - [fast](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-fast.md)

Current corrected-suite level-1 summary vs C `zstd -1`:
- better / worse / equal: `31 / 12 / 4`
- bytes-above-C on the losing fixtures: `1,426`

Largest losses on the corrected suite:
- `repo_Cargo.lock`: `9,255` vs `8,088` (`+1,167`)
- `repo_compressed.rs`: `13,111` vs `13,007` (`+104`)
- `decodecorpus_z000079`: `7,321` vs `7,221` (`+100`)
- `dict_dictionary.bin`: `20,160` vs `20,145` (`+15`)
- `decodecorpus_z000059`: `711` vs `698` (`+13`)

Useful conclusion:
- the old broad-local summary was materially understating the known-file-type gap because duplicated repo fixture names were collapsing multiple examples into one
- the corrected suite is the authoritative promotion corpus from here

## 2026-06-01 - Rejected `Cargo.lock -> CodeText` and `Cargo.lock -> DictionaryText` policy remaps

Change notes:
- After the corrected suite exposed `repo_Cargo.lock` as the largest known-file-type loser, I generated fresh archive inspections:
  - [current](/home/bsutton/git/zstd-rs/benchmarks/reports/archive-inspect/repo_Cargo.lock.current.l1.inspect.txt)
  - [C](/home/bsutton/git/zstd-rs/benchmarks/reports/archive-inspect/repo_Cargo.lock.c.l1.inspect.txt)
- Evidence:
  - Rust is worse on both literals and sequence payload:
    - `literal_section_bytes`: `6981` vs C `5975`
    - `sequence_payload_bytes`: `2254` vs C `2092`
  - Rust is also over-sequenced:
    - `836` vs C `784`
- Tried two pure filename-policy remaps in `encoding/mod.rs`:
  1. `Cargo.lock -> CodeText`
  2. `Cargo.lock -> DictionaryText`

Source reports:
- [Cargo.lock -> CodeText](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargolockcode-broad-local.md)
- [Cargo.lock -> DictionaryText](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargolockdict-broad-local.md)

Why they were rejected:
- both were exact no-ops on the corrected broad-local baseline:
  - `repo_Cargo.lock`: stayed `9,255`
  - no other fixture moved

Useful conclusion:
- the `Cargo.lock` gap is real, but it is not solved by reusing the retained `CodeText` or `DictionaryText` starting points as-is
- do not retry these plain remaps in the same form

## 2026-05-31 - Rejected tiny ConfigText exhaustive flat-distribution Huffman search

Change notes:
- Tried a tiny literal-side follow-up in `compressed.rs` and `huff0_encoder.rs`:
  - for tiny `ConfigText` literal sections, keep the smallest-table Huffman search active even when the literal distribution is considered flat
- Motivation:
  - fresh archive inspection still showed `repo_.gitignore` matching C on parse, sequence payload, stream count, and sequence table modes
  - that made the remaining `+8` bytes look like pure literal-payload slack
  - the current exact-search path still short-circuits on flat distributions, so this was the narrowest remaining literal-table search seam

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-config-flatsearch-broad-local.md)

Why it was rejected:
- It was an exact byte-for-byte no-op on the retained baseline:
  - `repo_.gitignore`: stayed `172`
  - `dict_talk.service`: stayed `160`
  - `repo_Cargo.toml`: stayed `730`
- No broad-local fixture moved at all.

Useful conclusion:
- the remaining tiny `ConfigText` literal gap is not hidden behind the current flat-distribution early return in the small-table search
- do not retry this flat-distribution exhaustive-search family in the same form

## 2026-05-31 - Rejected `.toml -> CodeText` file-type mapping at level 1

Change notes:
- Tried a pure starting-point policy change in `encoding/mod.rs`:
  - map `.toml` to `CodeText` instead of generic `ConfigText`
- Motivation:
  - known file types matter more than `Unknown`
  - `repo_Cargo.toml` was still one of the explicit remaining known-family losers
  - this was the cleanest extension-based experiment matching the public file-type design

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-tomlcode-broad-local.md)

Why it was rejected:
- It only moved the target, and it moved the wrong way:
  - `repo_Cargo.toml`: `730 -> 732`
- Everything else stayed byte-identical.

Useful conclusion:
- the retained `CodeText` matcher starting point is not a better fit for `.toml` in this form
- do not retry the plain `.toml -> CodeText` remap without stronger evidence or a more specific TOML family

## 2026-05-31 - Rejected DictionaryText current-over-oldest offset-bit displacement rule

Change notes:
- Tried a new `DictionaryText` window-candidate scoring rule in `match_generator.rs`:
  - only for `DictionaryText`
  - only when comparing a farther `oldest` non-repeat window candidate against the current non-repeat candidate
  - keep the closer current candidate when:
    - the farther `oldest` saves less than `2` match bytes
    - and it costs at least `4` more offset-code bits
- Motivation:
  - fresh current-vs-C archive inspection still shows the same dictionary shape:
    - Rust is over-sequenced: `4285` vs C `3461`
    - Rust pays much more offset extra bits: `47698` vs C `36827`
  - earlier blunt `oldest` penalties were too coarse, so this narrower offset-gated version was the next clean check

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dict-oldestbits-broad-local.md)

Why it was rejected:
- It only touched the target, and it moved the wrong way:
  - `dict_dictionary.bin`: `20,160 -> 20,161`
- Every other broad-local fixture stayed byte-identical.

Useful conclusion:
- the retained dictionary same-start smaller-offset rule is still the useful edge
- do not retry this current-vs-`oldest` displacement family in the same offset-bit-gated form

## 2026-05-31 - Rejected `ConfigText` single-stream-vs-4-stream literal candidate selection

Change notes:
- Tried narrowing the retained small-`ConfigText` single-stream Huffman rule:
  - keep the `ConfigText` single-stream path as a preferred candidate
  - but also estimate the normal 4-stream path
  - choose whichever estimated literal section is smaller
- Motivation:
  - `repo_Cargo.toml` still trailed C by `4` bytes
  - archive inspection showed C using a 4-stream literal section there while the retained Rust path used single-stream

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-config-streamchoice-broad-local.md)

Why it was rejected:
- It was an exact byte-for-byte no-op on the retained `config-singlestream` baseline:
  - `repo_Cargo.toml`: stayed `730`
  - `repo_.gitignore`: stayed `172`
  - `dict_talk.service`: stayed `160`
  - `dict_dictionary.bin`: stayed `20,160`
  - `decodecorpus_z000079`: stayed `7,321`
- No broad-local fixture moved at all.

Useful conclusion:
- the remaining `ConfigText` tail is not fixed by letting the single-stream override fall back to 4-stream on our current literal-size estimates
- do not retry this `ConfigText` literal stream-choice family without new evidence

## 2026-05-31 - Rejected adaptive Huffman weight-table description selection

Change notes:
- Tried a literal-table-description follow-up in `ruzstd/src/huff0/huff0_encoder.rs`:
  - for Huffman weight tables, choose the smaller of:
    - the existing direct nibble encoding
    - the existing FSE-compressed weight encoding
- Motivation:
  - after retaining the small-`ConfigText` single-stream Huffman rule, `repo_.gitignore` still trailed C by `8` bytes with the same parse and the same sequence payload
  - that made weight-table description overhead the next plausible literal-side target

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-hufftable-adaptive-broad-local.md)

Why it was rejected:
- It was an exact byte-for-byte no-op on the retained `config-singlestream` baseline:
  - `repo_.gitignore`: stayed `172`
  - `dict_talk.service`: stayed `160`
  - `repo_Cargo.toml`: stayed `730`
  - `decodecorpus_z000079`: stayed `7,321`
  - `build_ruzstd-cli`: stayed `866,649`
- No broad-local fixture moved at all.

Useful conclusion:
- the remaining `.gitignore` literal gap is not explained by choosing direct vs FSE Huffman weight-table encoding
- do not retry this Huff0 table-description family without new evidence

## 2026-05-31 - Retained wider dense probing for short-line `CodeText` up to 10 KiB

Change notes:
- Widened the retained dense short-line probe cutoff for `CompressionFileType::CodeText` only:
  - `CodeText`: `8 KiB -> 10 KiB`
  - `ConfigText`: stays at `8 KiB`
- Motivation:
  - the expanded broad-local suite exposed two new `CodeText` losers just above the retained `8 KiB` cutoff:
    - `repo_progress.rs` at `8,784` bytes
    - `repo_benchmark_zstd.py` at `8,997` bytes
  - both are short-line code and were missing the retained dense short-line path by a small margin

Source reports:
- [code-heavy fast screen](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-codeprobe10k-fast.md)
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-codeprobe10k-broad-local.md)
- [current broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [current fast](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-fast.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-codeprobe10k-retained)

Retained result on the expanded broad-local suite:
- targeted wins:
  - `repo_progress.rs`: `3,168 -> 3,147`
  - `repo_benchmark_zstd.py`: `2,865 -> 2,846`
- unchanged sentinels:
  - `build_ruzstd-cli`: `866,649`
  - `decodecorpus_z000079`: `7,321`
  - `dict_dictionary.bin`: `20,160`
  - `repo_compressed.rs`: `12,839`
  - `repo_main.rs`: `2,128`
- summary vs C `zstd -1` on the expanded suite:
  - better / worse / equal: `26 / 12 / 3`
  - bytes-above-C on losing fixtures: `312 -> 272`

Useful conclusion:
- the retained dense short-line `CodeText` idea still had room just above `8 KiB`
- that room was specific to `CodeText`; `ConfigText` remains bounded separately

## 2026-05-31 - Expanded the known-file-type broad-local corpus and refreshed the current live baseline

Change notes:
- Broadened `tools/prepare_benchmark_suites.py` so `broad-local` now includes more explicit known-file-type fixtures from the repo:
  - `.gitignore`
  - additional `Cargo.toml` files
  - `cli/src/progress.rs`
  - `ruzstd/src/encoding/blocks/compressed.rs`
  - benchmark tooling scripts
  - a wider spread of `.service` files
- Motivation:
  - known file types now matter more than the residual `Unknown` bucket
  - the benchmark corpus needed to reflect the larger extension/name mapping surface instead of the earlier smaller subset

Source reports:
- [current broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)

Current live result on the expanded broad-local suite:
- `41` comparable fixture rows in the report
- better / worse / equal vs C `zstd -1`: `26 / 12 / 3`
- total bytes-above-C on the losing fixtures: `312`

Largest remaining losses on the expanded suite:
- `decodecorpus_z000079`: `+100`
- `repo_compressed.rs`: `+87`
- `repo_progress.rs`: `+44`
- `repo_benchmark_zstd.py`: `+20`
- `dict_dictionary.bin`: `+15`

Useful conclusion:
- the broader known-file-type corpus increases the absolute remaining byte gap, but it gives the current file-type policies a much more representative benchmark target
- `Unknown` is still the biggest single remaining loser, but explicit `CodeText` and `ConfigText` residuals are now more visible and should be treated as first-class targets

## 2026-05-31 - Rejected `ConfigText` offset-table max-log `8 -> 7`

Change notes:
- Tried a new explicit file-type entropy setting:
  - `CompressionFileType::ConfigText` uses `offset_table_max_log = 7` at level 1
- Motivation:
  - `ConfigText` remained slightly behind C
  - both retained `DictionaryText` and retained `Unknown` had already benefited from `offset_table_max_log = 7`

Source reports:
- [config-heavy fast screen](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-config-oflog7-fast.md)

Why it was rejected:
- It was a clean no-op on the current live baseline:
  - `dict_kmod-static-nodes.service`: stayed `486`
  - `dict_fstrim.service`: stayed `299`
  - `dict_systemd-udev-settle.service`: stayed `560`
  - `dict_NetworkManager-dispatcher.service`: stayed `381`
- Guardrails also stayed exact:
  - `build_ruzstd-cli`: `855,679`
  - `decodecorpus_z000079`: `7,321`
  - `dict_dictionary.bin`: `20,160`
  - `repo_main.rs`: `2,105`

Useful conclusion:
- `ConfigText` is not waiting on the same offset-table-log move that helped `DictionaryText` and `Unknown`
- the next explicit file-type move for `ConfigText` needs a different signal than offset FSE log width

## 2026-05-31 - Rejected an extra repeat margin only for large-`Unknown` `RepeatNextPosition` wins

Change notes:
- Tested a narrower follow-up inside the dominant large-`Unknown` repeat family:
  - keep the retained large-`Unknown` repeat margin normally
  - add one more repeat-vs-normal margin point only when the repeat candidate is the `ip+1` `RepeatNextPosition` case
- Motivation:
  - live matcher diagnostics still show `repeat_next_position_selected_without_current_candidate` dominating `decodecorpus_z000079`
  - this was the narrowest way to distinguish that family without changing the public API or broadening the generic repeat bias again

Source reports:
- [fast A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-nextrepmargin-fast.md)
- [restore fast](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-level1-unknown-nextrepmargin-fast.md)

Why it was rejected:
- It made the target worse immediately:
  - `decodecorpus_z000079`: `7,321 -> 7,331`
- It also gave back bytes on an already-winning Unknown-family fixture:
  - `build_ruzstd-cli`: `855,679 -> 855,745`
- The other fast sentinels stayed flat:
  - `dict_dictionary.bin`: `20,160`
  - `repo_main.rs`: `2,105`

Useful conclusion:
- the remaining `z000079` gap is not fixed by giving `RepeatNextPosition` a stronger local repeat-vs-normal bias
- the large-`Unknown` next-position repeat family still needs a different parse/representation change, not another margin tweak

## 2026-05-31 - Rejected a 96 KiB block-size cap for level-1 `Unknown` files

Change notes:
- After exhausting matcher-local and post-match-indexing variants, tested a block-structure change instead:
  - for `CompressionFileType::Unknown` at level 1, cap block reads at `96 KiB` instead of the default `128 KiB`
- Motivation:
  - `decodecorpus_z000079` still differs materially from C in both parse shape and block shape
  - this was the first direct attempt to change the large-`Unknown` block structure rather than just the matcher internals

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-block96k-vsretained-broad-local.md)
- [fast](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-block96k-vsretained-fast.md)

Why it was rejected:
- It made the target much worse:
  - `decodecorpus_z000079`: `7,321 -> 7,772`
- It also badly regressed already-winning fixtures:
  - `build_ruzstd-cli`: `855,679 -> 884,939`
  - `generated_repeated_text_001m.txt`: `208 -> 241`
  - `repeated_text_32m.txt`: `2,874 -> 3,819`
- Fast guardrails also drifted the wrong way:
  - `decodecorpus_pack.bin` CPU: `0.22s -> 0.25s`

Useful conclusion:
- the remaining `z000079` gap is not helped by a simple smaller fixed block size on `Unknown`
- the next credible block-structure move, if any, has to be much more selective than a blunt 96 KiB cap

## 2026-05-31 - Rejected denser post-match suffix insertion on the large-`Unknown` `RepeatNextPosition` path

Change notes:
- After concluding that local scoring tweaks were exhausted, tested two structural state-updates on the dominant large-`Unknown` family:
  - give `RepeatNextPosition` wins a denser post-match suffix insertion limit of `256` instead of the default sparse threshold `128`
  - then fully dense post-match suffix insertion after those same wins
- Motivation:
  - live matcher diagnostics still show `repeat_next_position_selected_without_current_candidate` dominating `decodecorpus_z000079`
  - if the remaining gap were caused by too little future candidate availability after those wins, denser post-match insertion should have changed the parse

Source reports:
- [dense 256 broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-nextrep-dense256-vsretained-broad-local.md)
- [dense 256 fast](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-nextrep-dense256-vsretained-fast.md)
- [full dense broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-nextrep-fulldense-vsretained-broad-local.md)
- [full dense fast](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-nextrep-fulldense-vsretained-fast.md)

Why they were rejected:
- dense `256`:
  - exact byte-for-byte no-op on the target:
    - `decodecorpus_z000079`: stayed `7,321`
  - only CPU drift on the fast screen:
    - `json_logs_32m.jsonl`: `0.16s -> 0.18s`
- full dense:
  - still exact byte-for-byte no-op on the target:
    - `decodecorpus_z000079`: stayed `7,321`
  - slight noise elsewhere:
    - `build_ruzstd-cli`: `855,679 -> 855,655`
    - `json_logs_32m.jsonl` CPU: `0.16s -> 0.17s`

Useful conclusion:
- the dominant large-`Unknown` `RepeatNextPosition` path is not bottlenecked by sparse post-match suffix insertion
- the next credible move needs a different parse/representation change before or during candidate selection, not another post-match indexing density tweak

## 2026-05-31 - Rejected disabling backward match extension on the large-`Unknown` Fastest path

Change notes:
- Tested a direct parse-shape change for the main remaining loser:
  - on the large `CompressionFileType::Unknown` Fastest path, disable backward match extension so the current match no longer pulls preceding literals into the sequence
- Motivation:
  - fresh `decodecorpus_z000079` current-vs-C archive evidence showed the Rust path is still materially under-sequenced with too few literals
  - this was the narrowest structural way to reduce match coalescing without changing the candidate search itself

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-nobackextend-vsretained-broad-local.md)
- [fast](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-nobackextend-vsretained-fast.md)

Why it was rejected:
- It moved the target the wrong way:
  - `decodecorpus_z000079`: `7,321 -> 7,360`
- It also gave back already-retained Unknown-family wins:
  - `build_ruzstd-cli`: `855,679 -> 866,828`
  - `decodecorpus_z000033`: `532,632 -> 537,783`

Useful conclusion:
- the remaining `z000079` gap is not caused by backward extension alone
- large-`Unknown` under-sequencing is still a real signal, but a blunt no-backward-extension cut damages the rest of the parse too much

## 2026-05-31 - Refreshed live `decodecorpus_z000079` archive evidence and rejected three more large-`Unknown` structural branches

Change notes:
- Generated fresh live current-vs-C archive inspections for `decodecorpus_z000079` with block-level sequence histograms.
- Key live evidence:
  - current Rust:
    - `compressed_bytes=7321`
    - `literal_section_bytes=820`
    - `sequence_payload_bytes=6460`
    - `decoded_literals=1463`
    - `sequences=2806`
    - `match_bytes=391753`
  - C `zstd -1`:
    - `compressed_bytes=7221`
    - `literal_section_bytes=1449`
    - `sequence_payload_bytes=5722`
    - `decoded_literals=2799`
    - `sequences=4354`
    - `match_bytes=357649`
- That confirms the remaining `z000079` gap is still sequence/offset-side and materially under-sequenced on the Rust path.

Source reports:
- [half-window broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-halfwindow-vsretained-broad-local.md)
- [half-window fast](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-halfwindow-vsretained-fast.md)
- [ip+1 newest-only broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-nextrep-firstonly-vsretained-broad-local.md)
- [ip+1 newest-only fast](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-nextrep-firstonly-vsretained-fast.md)
- [large Unknown smaller-offset scoring broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-offsetscore-vsretained-broad-local.md)
- [large Unknown smaller-offset scoring fast](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-offsetscore-vsretained-fast.md)

Why they were rejected:
- large `Unknown` smaller-offset scoring without the same-start restriction:
  - exact byte-for-byte no-op
  - `decodecorpus_z000079` stayed `7,321`
- large `Unknown` half-window:
  - `decodecorpus_z000079` stayed `7,321`
  - `build_ruzstd-cli`: `855,679 -> 865,333`
- large `Unknown` `ip+1` newest-repeat-only:
  - hard regression on the target:
    - `decodecorpus_z000079`: `7,321 -> 7,606`
  - also regressed:
    - `build_ruzstd-cli`: `855,679 -> 856,949`

Useful conclusion:
- the remaining `z000079` gap is not waiting on:
  - smaller-offset rescoring inside the existing non-repeat candidate set
  - a narrower effective search window
  - dropping second/third repeat-history slots from the `ip+1` repeat probe
- the second and third repeat-history slots are still doing real work on `z000079`
- the next credible move needs a different large-`Unknown` sequence/offset representation or parse structure, not another local threshold or simple family cut

## 2026-05-31 - Rejected two more `Unknown` current-window scoring variants after refreshing live matcher diagnostics

Change notes:
- Refreshed ignored matcher diagnostics directly on the live retained baseline for:
  - `decodecorpus_z000079`
  - `dict_dictionary.bin`
  - `decodecorpus_z000059`
  - `decodecorpus_z000053`
- The refreshed signal stayed consistent:
  - `z000079` is still dominated by `repeat_next_position_selected_without_current_candidate`
  - current-window wins are still almost entirely `newest` / `oldest`
  - no long-hash and no `second_newest`
- Then tested two new scoring variants:
  - large `Unknown` same-start smaller-offset preference
  - extend the retained `Unknown` newest/oldest displacement rules only to tiny `Unknown` non-text blocks up to `4 KiB`

Source reports:
- [large Unknown same-start smaller-offset](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-samestart-offset-vsretained-broad-local.md)
- [large Unknown same-start smaller-offset fast](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-samestart-offset-vsretained-fast.md)
- [tiny Unknown displacement](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-smalldisplacement-vsretained-broad-local.md)
- [tiny Unknown displacement fast](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-smalldisplacement-vsretained-fast.md)

Why they were rejected:
- large `Unknown` same-start smaller-offset preference:
  - exact byte-for-byte no-op on both broad-local and fast guardrails
  - `decodecorpus_z000079` stayed `7,321`
- tiny `Unknown` displacement:
  - bought `2` bytes on `decodecorpus_z000059` (`711 -> 709`)
  - but regressed:
    - `decodecorpus_z000031`: `112 -> 113`
    - `decodecorpus_z000053`: `304 -> 305`
  - and still drifted `json_logs_32m.jsonl` CPU the wrong way on the fast screen

Useful conclusion:
- the remaining `z000079` gap is not a same-start smaller-offset fight
- the prior all-size `Unknown` displacement win on `z000059` does not become safe just by narrowing it to tiny blocks
- the next credible move is still a more structural large-`Unknown` sequence/offset decision, not another local current-window threshold split

## 2026-05-31 - Rejected two more `Unknown`-family follow-ups after the retained `ConfigText` compressed-literals LL/ML gate

Change notes:
- Tested two more `Unknown`-side follow-ups after restoring the retained `ConfigText` compressed-literals LL/ML point:
  - widen the retained `Unknown` compressed-literals predefined LL/ML gate from `64` to `256` sequences
  - keep the retained large-`Unknown` `newest +2` rule normally, but require `+3` when the farther `newest` candidate also costs at least 4 more offset-code bits

Source reports:
- [Unknown predef256 compressed-literals](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-predef256-complit-vsretained-broad-local.md)
- [large Unknown newest bits rule](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-newestbits-vsretained-broad-local.md)

Why they were rejected:
- `Unknown` predef256 compressed-literals:
  - hard regression on `decodecorpus_z000059`:
    - `711 -> 826`
  - `decodecorpus_z000079` stayed `7,321`
- conditional stronger large-`Unknown` `newest` rule:
  - `decodecorpus_z000079` stayed `7,321`
  - gave back an already-retained Unknown-family win:
    - `build_ruzstd-cli`: `855,679 -> 855,725`

Useful conclusion:
- the retained `Unknown` compressed-literals predefined LL/ML gate is bounded at `64`
- the remaining `z000079` gap is still not moving on another `newest`-side local threshold

## 2026-05-31 - Rejected three narrow follow-ups after the retained `ConfigText` compressed-literals LL/ML gate

Change notes:
- Tested three tightly scoped follow-ups after retaining the level-1 `ConfigText` compressed-literals LL/ML predefined-table gate:
  - large `Unknown` equal-length smaller-offset tie-break
  - `ConfigText` compressed-literals OF predefined-table gate up to `64` sequences
  - narrower `ConfigText` compressed-literals OF predefined-table gate up to `24` sequences

Source reports:
- [large Unknown equal-length tie-break](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-equaloffsettie-vsretained-broad-local.md)
- [ConfigText OF <=64](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-config-predef64-complit-of-vsllmlonly-broad-local.md)
- [ConfigText OF <=24](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-config-predef24-complit-of-vsretained-broad-local.md)

Why they were rejected:
- Large `Unknown` equal-length smaller-offset tie-break:
  - exact byte-for-byte no-op on broad-local
  - `decodecorpus_z000079` stayed `7,321`
- `ConfigText` OF predefined-table gate up to `64` sequences:
  - did not improve `dict_kmod-static-nodes.service`
  - regressed other service files:
    - `dict_systemd-coredump@.service`: `682 -> 688`
    - `dict_systemd-udev-settle.service`: `560 -> 562`
- `ConfigText` OF predefined-table gate up to `24` sequences:
  - still did not improve `dict_kmod-static-nodes.service`
  - only moved `dict_fstrim.service`: `299 -> 298`

Useful conclusion:
- the remaining `kmod`-style config residual is not fixed by broadening OF predefined-table eligibility on top of the retained `ConfigText` LL/ML gate
- the remaining large-`Unknown` gap is also not waiting on an equal-length smaller-offset tie-break

## 2026-05-31 - Retained predefined LL/ML table eligibility for small compressed-literals `ConfigText` blocks

Change notes:
- Extended the retained level-1 small compressed-literals predefined LL/ML table gate from `CompressionFileType::Unknown` to also cover `CompressionFileType::ConfigText`:
  - compressed-literals blocks only
  - at most `64` sequences
- This was motivated by the remaining service/config residuals, especially `dict_kmod-static-nodes.service`, whose archive shape still looked like table overhead rather than matcher drift.

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-config-predef64-complit-vsretained-broad-local.md)
- [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-config-predef64-complit-vsretained-fast.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-config-predef64-complit-retained)

Why it was retained:
- It improved multiple remaining `ConfigText` fixtures without moving the main level-1 guardrails:
  - `dict_kmod-static-nodes.service`: `497 -> 486`
  - `dict_NetworkManager-dispatcher.service`: `391 -> 381`
  - `dict_fstrim.service`: `308 -> 299`
  - `dict_systemd-coredump@.service`: `686 -> 682`
  - `dict_systemd-udev-settle.service`: `568 -> 560`
- Main fast guardrails stayed exact:
  - `decodecorpus_pack.bin`: `5,319,265`
  - `json_logs_32m.jsonl`: `690,084`
  - `decodecorpus_z000079`: `7,321`
  - `dict_dictionary.bin`: `20,160`
- Broad-local summary vs C improved materially:
  - better / worse / equal: `16 / 13 / 3 -> 18 / 11 / 3`
  - bytes-above-C on losing fixtures: `192 -> 155`

## 2026-05-31 - Rejected adding predefined OF-table eligibility on top of the retained Unknown small-sequence LL/ML gate

Change notes:
- Followed the retained Unknown small compressed-literals LL/ML predefined-table gate with the obvious entropy-side sibling:
  - keep the retained LL/ML gate
  - also allow predefined OF tables on the same `CompressionFileType::Unknown`, level-1, compressed-literals, small-sequence blocks

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-predef64-complit-of-vsllmlonly-broad-local.md)

Why it was rejected:
- It regressed the small Unknown fixtures immediately:
  - `decodecorpus_z000053`: `304 -> 305`
  - `decodecorpus_z000059`: `711 -> 747`
- It also added noise on already-winning Unknown fixtures:
  - `decodecorpus_z000033` CPU drifted the wrong way in that run

Useful conclusion:
- the retained `z000053` win is specifically about LL/ML table-mode choice
- the same broadening to OF tables is not safe for this block family

## 2026-05-31 - Retained an `Unknown` predefined LL/ML gate for small compressed-literals blocks

Change notes:
- After inspecting the smaller remaining Unknown losers, tested a sequence-entropy-side change rather than another matcher gate:
  - `CompressionFileType::Unknown`
  - level 1 only
  - if the block is in the compressed-literals path and has at most `64` sequences, allow predefined LL/ML tables instead of forcing encoded LL/ML tables
- This was motivated by `decodecorpus_z000053`, where C was already using predefined LL/ML while the retained Rust path still encoded both tables.

Source reports:
- retained point versus the retained `newest +2` baseline:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-predef64-complit-vsretained-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-predef64-complit-vsretained-fast.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-level1-unknown-predef64-complit.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-unknown-predef64-complit-retained)

Why it was retained:
- It materially improved one of the remaining Unknown losers without reopening the fast guardrails:
  - `decodecorpus_z000053`: `322 -> 304`
- It avoided the earlier tiny-file regression by only applying when the block is in the compressed-literals path:
  - `decodecorpus_z000031`: stayed `112`
- Broad-local summary vs C improved:
  - better / worse / equal stayed `16 / 13 / 3`
  - bytes-above-C on losing fixtures: `210 -> 192`
- Fast guardrails stayed exact on bytes and in the same CPU band.

Useful archive result:
- On `decodecorpus_z000053`, this flips Rust to the same table-mode shape C already uses:
  - before:
    - `ll_mode=fse`
    - `ml_mode=fse`
    - `sequence_payload_bytes=106`
  - after:
    - `ll_mode=predefined`
    - `ml_mode=predefined`
    - `sequence_payload_bytes=88`

## 2026-05-31 - Rejected a tiny-sequence-only `Unknown oflog6` gate

Change notes:
- After inspecting the two smaller Unknown losers (`decodecorpus_z000053`, `decodecorpus_z000059`), tried a narrower follow-up to the rejected small-sequence entropy family:
  - keep retained `Unknown oflog7` normally
  - use `oflog6` only when the `Unknown` level-1 block has at most `256` sequences
- This was meant to touch the tiny single-block Unknown cases without reopening the large `z000079` regression from the earlier `1,536`-sequence gate.

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-tinyseq-oflog6-vsretained-broad-local.md)

Why it was rejected:
- It regressed the exact fixtures it was meant to help:
  - `decodecorpus_z000053`: `322 -> 324`
  - `decodecorpus_z000059`: `711 -> 714`
- It also still nudged the large target the wrong way:
  - `decodecorpus_z000079`: `7,321 -> 7,325`

Useful conclusion:
- sequence-count alone is not a safe gate for a stronger `Unknown` offset entropy table, even at a much lower threshold
- this family is closed in its current representation

## 2026-05-31 - Rejected extending the retained `Unknown` displacement rules to all Unknown block sizes

Change notes:
- Followed the retained large-`Unknown` `oldest +2` and `newest +2` scoring rules with a narrower family extension:
  - keep both retained displacement rules
  - let them apply to all Fastest `CompressionFileType::Unknown` non-text blocks instead of only the large `128 KiB` path
- This was aimed directly at the smaller remaining Unknown losers:
  - `decodecorpus_z000053`
  - `decodecorpus_z000059`

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-allsize-displacement-vsretained-broad-local.md)
- [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-allsize-displacement-vsretained-fast.md)

Why it was rejected:
- It did improve a few smaller Unknown fixtures:
  - `decodecorpus_z000059`: `711 -> 709`
  - `decodecorpus_z000054`: `9,567 -> 9,565`
  - `decodecorpus_z000080`: `2,603 -> 2,602`
  - `decodecorpus_z000003`: `51,012 -> 51,001`
- But the total retained gain was too small:
  - broad-local bytes-above-C on losing fixtures: `210 -> 208`
- And the main fast guardrails drifted the wrong way:
  - `decodecorpus_pack.bin` CPU: `0.22s -> 0.23s`
  - `json_logs_32m.jsonl` CPU: `0.16s -> 0.17s`

Useful conclusion:
- the current-window displacement rules are worth keeping only on the large-Unknown path
- extending them below that block-size gate does move a few small Unknown fixtures, but not enough to pay for the CPU drift

## 2026-05-31 - Retained a large-`Unknown` newest-displacement rule and rejected the stronger point

Change notes:
- Followed the retained `Unknown oflog7` point with the missing sibling to the earlier retained `oldest +2` scoring rule:
  - only on large Fastest `CompressionFileType::Unknown` non-text blocks
  - when a farther `newest` current-window candidate tries to displace an already-found closer non-repeat candidate, require at least a 2-byte match gain

Source reports:
- retained `+2` point versus the retained `Unknown oflog7` baseline:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-newestgain2-vsretained-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-newestgain2-vsretained-fast.md)
- bounded `+3` follow-up:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-newestgain3-vs2-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-unknown-newestgain2-retained)

Why `+2` was retained:
- It is a clean broad-family win with no observed regressions:
  - `build_ruzstd-cli`: `855,822 -> 855,679`
  - `decodecorpus_z000033`: `532,650 -> 532,632`
- The target stayed flat:
  - `decodecorpus_z000079`: `7,321`
- Fast guardrails stayed in the same CPU band.
- Broad-local gap versus C stayed flat overall:
  - better / worse / equal stayed `16 / 13 / 3`
  - bytes-above-C on losing fixtures stayed `210`

Why `+3` was rejected:
- It gave back the only broad-family win:
  - `build_ruzstd-cli`: `855,679 -> 855,915`
- It still did nothing for the target:
  - `decodecorpus_z000079`: stayed `7,321`

Useful conclusion:
- the large-`Unknown` current-window scoring family is now bounded on both sides:
  - `oldest +2` helps, stronger versions fail
  - `newest +2` is worth keeping as a small safe improvement
  - `newest +3` is already too strong

## 2026-05-31 - Rejected a small-sequence-only `Unknown oflog6` gate

Change notes:
- Followed the retained `CompressionFileType::Unknown offset_table_max_log = 7` point with a block-local entropy variant:
  - keep retained `Unknown oflog7` normally
  - use `oflog6` only when the `Unknown` level-1 block has at most `1,536` sequences
- This was aimed directly at `decodecorpus_z000079`, whose retained archive shape is much smaller-sequence-count than the already-winning build-artifact blocks.

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-oflog6-smallseq-vs7-broad-local.md)
- [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-oflog6-smallseq-vs7-fast.md)

Why it was rejected:
- It regressed the target hard:
  - `decodecorpus_z000079`: `7,321 -> 7,413`
- It also regressed several neighboring decodecorpus samples:
  - `decodecorpus_z000053`: `322 -> 324`
  - `decodecorpus_z000054`: `9,567 -> 9,589`
  - `decodecorpus_z000059`: `711 -> 714`
  - `decodecorpus_z000030`: `13,152 -> 13,156`
- `build_ruzstd-cli` stayed flat, so the gate did not even buy a compensating broad Unknown-family shift.

Useful conclusion:
- the retained `Unknown oflog7` point is not too broad because of the large build-artifact blocks
- small-sequence-count alone is not a safe signal for a stronger `Unknown` offset entropy table

## 2026-05-31 - Retained `Unknown` offset FSE max-log `7` on top of the retained large-`Unknown` oldest-displacement rule

Change notes:
- Revisited the earlier `Unknown oflog7` idea on the newer retained baseline that already includes:
  - the large-`Unknown` current-window oldest-displacement `+2` rule
  - the retained file-type/path-hint mapping
- This time the `Unknown` offset-table entropy tweak moved enough to retain.

Source reports:
- retained `7` point versus the retained `+2` oldest-displacement baseline:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-oflog7-current-vsretained-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-oflog7-current-vsretained-fast.md)
- bounded `6` follow-up:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-oflog6-vs7-current-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-oflog6-vs7-current-fast.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-unknown-oflog7-retained)

Why `7` was retained:
- It improves the largest remaining loser again:
  - `decodecorpus_z000079`: `7,324 -> 7,321`
- The givebacks were only on fixtures still comfortably better than C:
  - `build_ruzstd-cli`: `855,782 -> 855,822`
  - `decodecorpus_z000033`: `532,546 -> 532,650`
  - `decodecorpus_z000003`: `51,006 -> 51,012`
- Fast guardrails stayed in the same CPU band.
- Broad-local total bytes-above-C on losing fixtures improved:
  - `213 -> 210`

Why `6` was rejected:
- It regressed the target hard:
  - `decodecorpus_z000079`: `7,321 -> 7,413`
- It also gave back already-winning fixtures broadly:
  - `build_ruzstd-cli`: `855,822 -> 860,261`
  - `decodecorpus_z000033`: `532,650 -> 536,771`
- This bounds the family:
  - `8 -> 7` helps on the current retained baseline
  - `7 -> 6` is clearly too strong

## 2026-05-31 - Rejected a distance-based stronger large-`Unknown` oldest-displacement rule

Change notes:
- Followed the retained large-`Unknown` oldest-displacement `+2` rule with another structural variant:
  - keep the retained `+2` rule inside the current entry
  - require `+3` only when `oldest` comes from the previous entry (`entry_distance >= 1`)

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-oldestdist1-vs2-broad-local.md)
- [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-oldestdist1-vs2-fast.md)

Why it was rejected:
- It gave back part of the retained unknown-family win:
  - `build_ruzstd-cli`: `855,782 -> 855,926`
- It did not improve the target at all:
  - `decodecorpus_z000079`: stayed `7,324`
- CPU stayed flat, so there was no compensating gain.

## 2026-05-31 - Rejected a selective stronger large-`Unknown` oldest-displacement rule

Change notes:
- Followed the retained large-`Unknown` oldest-displacement `+2` rule with a more selective stronger point:
  - still only on large Fastest `CompressionFileType::Unknown` non-text blocks
  - keep the retained `+2` rule normally
  - require `+3` only when the farther `oldest` candidate also costs at least 4 more offset-code bits

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-oldestgain2bits-vs2-broad-local.md)
- [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-oldestgain2bits-vs2-fast.md)

Why it was rejected:
- It gave back the target:
  - `decodecorpus_z000079`: `7,324 -> 7,328`
- It also gave back part of the retained unknown-family win:
  - `build_ruzstd-cli`: `855,782 -> 855,829`
- CPU stayed flat, but the compression direction was wrong.

Useful conclusion:
- the retained `+2` oldest-displacement rule is already near the edge
- strengthening it only on large offset-code gaps still over-penalizes useful `oldest` wins

## 2026-05-31 - Retained a large-`Unknown` oldest-displacement rule and bounded the family

Change notes:
- After the retained `dict oflog7` point, the largest remaining live level-1 loss was still `decodecorpus_z000079`, and the archive shape still showed too many farther offset-heavy current-window wins.
- Retained change:
  - only on large Fastest `CompressionFileType::Unknown` non-text blocks
  - when an `oldest` current-window candidate tries to displace an already-found closer non-repeat candidate, require at least a 2-byte match gain

Source reports:
- retained `+2` point versus the prior retained baseline:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-oldestgain2-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-oldestgain2-fast.md)
- bounded `+3` follow-up:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-oldestgain3-vs2-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-oldestgain3-vs2-fast.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-unknown-oldestgain2-retained)

Why `+2` was retained:
- It improves the target and a few already-winning unknown-family fixtures with flat fast-guardrail CPU:
  - `decodecorpus_z000079`: `7,326 -> 7,324`
  - `decodecorpus_z000028`: `98,567 -> 98,388`
  - `decodecorpus_z000033`: `532,592 -> 532,546`
  - `build_ruzstd-cli`: `856,110 -> 855,782`
- Broad-local summary vs C improved slightly:
  - better / worse / equal stayed `16 / 13 / 3`
  - bytes-above-C on losing fixtures: `215 -> 213`
- Fast guardrails stayed exact on the important fixtures:
  - `decodecorpus_pack.bin`: `5,319,265`
  - `json_logs_32m.jsonl`: `690,084`
  - `dict_dictionary.bin`: `20,160`

Why `+3` was rejected:
- It gave back the target and some of the already-winning fixtures:
  - `decodecorpus_z000079`: `7,324 -> 7,335`
  - `build_ruzstd-cli`: `855,782 -> 856,067`
  - `decodecorpus_z000033`: `532,546 -> 532,577`
- It also nudged fast-guardrail CPU the wrong way.
- This bounds the family:
  - `+2` helps
  - `+3` is worse

## 2026-05-31 - Rejected a wider large-`Unknown` `ip+1` window promotion

Change notes:
- Tested a narrower C-shaped follow-up on the retained `dict oflog7` baseline:
  - only on large Fastest `CompressionFileType::Unknown` non-text blocks
  - widen the `ip+1` normal-window promotion from exact-minimum current non-repeat hits to short current non-repeat hits up to `8` bytes
- This was aimed at matching the useful `double_fast` control-flow clue more closely than the earlier exact-minimum helper enable.

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-nextwindowwide-broad-local.md)
- [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-nextwindowwide-fast.md)

Why it was rejected:
- exact byte-for-byte no-op on both suites:
  - `decodecorpus_z000079`: stayed `7,326`
  - `build_ruzstd-cli`: stayed `856,110`
  - every broad-local fixture stayed exact
- so the remaining `z000079` gap is not waiting on a wider `ip+1` normal-window probe in the current matcher representation

## 2026-05-31 - Retained `DictionaryText` offset FSE max-log `7` and bounded the family

Change notes:
- The in-flight dictionary entropy experiment turned out to have drifted past the previously benchmarked point.
- I saved the live `6` candidate binary, rebuilt the `7` point, and ran a direct sequential A/B:
  - `6` as current
  - `7` as upstream
- This bounded the family cleanly without relying on stale retained binaries.

Source reports:
- retained stepping-stone point versus the prior `8` baseline:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dict-oflog7-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dict-oflog7-fast.md)
- direct `6` vs `7` bounding run:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dict-oflog6-vs7-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dict-oflog6-vs7-fast.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-dict-oflog7-retained)

Why `7` was retained:
- The earlier `7` point improved only the dictionary target and left everything else exact:
  - `dict_dictionary.bin`: `20,162 -> 20,160`
  - every other broad-local fixture stayed byte-identical
  - fast guardrails stayed byte-identical
- That moves the current broad-local gap vs C from `217` bytes above C on losing fixtures to `215`.

Why `6` was rejected:
- Direct `6` vs `7` A/B regressed only the target, but regressed it hard:
  - `dict_dictionary.bin`: `20,160 -> 20,432`
- Everything else stayed exact, so the family is now bounded:
  - `8 -> 7` helps slightly
  - `7 -> 6` is clearly worse

## 2026-05-31 - Rejected two more narrow no-op follow-ups after the retained dictionary offset-aware point

Change notes:
- Re-ran two narrow file-type follow-ups sequentially against the retained dictionary-offset-aware baseline:
  - tiny `ConfigText` short-line threshold `5 -> 4` below `1 KiB`
  - `DictionaryText` smaller-offset preference widened from exact same-start to “starts no later and covers to within 1 byte”

Source reports:
- tiny `ConfigText` threshold `4`:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-tiny-config4-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-tiny-config4-fast.md)
- dictionary similar-coverage offset preference:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dict-offsetcoverage-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dict-offsetcoverage-fast.md)

Why they were rejected:
- tiny `ConfigText` threshold `4`:
  - exact byte-for-byte no-op on the retained baseline
  - no service-file improvement at all
- dictionary similar-coverage offset preference:
  - exact byte-for-byte no-op versus the retained dictionary same-start rule
  - `dict_dictionary.bin` stayed `20,162`

Useful conclusion:
- the remaining config/service residuals are not waiting on one more tiny short-line threshold cut
- the retained dictionary same-start rule already captured the useful offset-side gain; widening it to near-same coverage buys nothing

## 2026-05-31 - Rejected two more `Unknown`-family follow-ups after the retained dictionary offset-aware point

Change notes:
- After retaining the dictionary-only same-start smaller-offset rule, tested two new `Unknown`-family ideas against the live retained baseline:
  - small `Unknown` same-start smaller-offset preference for non-repeat candidates
  - `Unknown` offset FSE max-log `8 -> 7`

Source reports:
- small `Unknown` offset-aware matcher rule:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-small-unknown-offsetaware-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-small-unknown-offsetaware-fast.md)
- `Unknown` offset FSE max-log `7`:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-oflog7-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-oflog7-fast.md)

Why they were rejected:
- small `Unknown` offset-aware matcher rule:
  - hard regression across the `decodecorpus_z...` family:
    - `decodecorpus_z000079`: `7,326 -> 7,946`
    - `build_ruzstd-cli`: `856,110 -> 878,123`
    - `decodecorpus_z000059`: `711 -> 747`
  - this closes the “reuse the dictionary same-start rule on small Unknown blocks” family
- `Unknown` offset FSE max-log `7`:
  - it did produce tiny wins:
    - `decodecorpus_z000079`: `7,326 -> 7,325`
    - `decodecorpus_z000059`: `711 -> 709`
    - `build_ruzstd-cli`: `856,110 -> 855,482`
  - but the gain was too small to justify another file-type entropy knob, and the fast repeat run drifted the wrong way on `build_ruzstd-cli`:
    - `0.06s -> 0.07s`

Useful conclusion:
- the retained dictionary offset-aware rule does not transfer to small Unknown binary blocks
- the remaining `Unknown` gap is not likely to be closed by another tiny offset-FSE-table-log tweak

## 2026-05-31 - Retained a dictionary-only same-start smaller-offset preference and bounded the family

Change notes:
- Starting from the retained `codeconfigprobe1` live baseline, the remaining `dict_dictionary.bin` gap still looked offset-side:
  - archive inspection showed too many sequences and too many offset extra bits versus C
  - matcher diagnostics showed the dictionary path is mainly a current-entry `newest` vs `oldest` fight, with no repeat, `second_newest`, or long-hash signal
- Retained change:
  - only for `CompressionFileType::DictionaryText`
  - only for non-repeat candidates
  - only when two candidates begin at the same `start_idx`
  - prefer the smaller offset when it saves at least 2 offset-code bits for at most a 1-byte match loss
- I validated it with a direct live-tree A/B:
  - saved the candidate binary with the new rule
  - disabled only that rule in source
  - rebuilt a baseline binary from the live tree
  - benchmarked candidate vs baseline directly

Source reports:
- [direct live broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dict-offsetaware-live-broad-local.md)
- [direct live fast A/B repeat run](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dict-offsetaware-live-fast.md)
- [candidate-vs-older-retained broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dict-offsetaware-broad-local.md)

Why it was retained:
- Direct live-tree A/B moved exactly one fixture:
  - `dict_dictionary.bin`: `20,175 -> 20,162`
  - C `zstd -1`: `20,145`
- Everything else in the broad-local suite stayed byte-identical.
- Fast guardrails stayed byte-identical, and the repeat run stayed in the same CPU band:
  - `decodecorpus_pack.bin`: `5,319,265`
  - `json_logs_32m.jsonl`: `690,084`
  - `decodecorpus_pack.bin` CPU: `0.22s -> 0.22s`
- Broad-local summary vs C improved:
  - better / worse / equal stayed `16 / 13 / 3`
  - bytes-above-C on losing fixtures: `230 -> 217`

Bounded follow-up:
- Allowing up to a 2-byte match loss produced the same dictionary result (`20,162`) and no additional wins elsewhere.
- That wider point is not worth keeping as a separate retained setting.

## 2026-05-31 - Rejected two more narrow sequence-side follow-ups after the retained `codeconfigprobe1` baseline

Change notes:
- After retaining dense short-line probing for tiny `CodeText` and `ConfigText`, tested two sequence-side follow-ups on the remaining binary/dictionary holdouts:
  - large Fastest `Unknown` `ip+1` repeat-lookahead widened from exact 5-byte current non-repeat hits to short current non-repeat hits up to 8 bytes
  - `DictionaryText` `oldest` candidates required more than a 1-byte gain before displacing an existing non-repeat candidate

Source reports:
- widened large-`Unknown` repeat lookahead:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-nextrepweak-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-nextrepweak-fast.md)
- dictionary `oldest` bias:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dict-oldestbias-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dict-oldestbias-fast.md)

Why they were rejected:
- widened large-`Unknown` repeat lookahead:
  - exact byte-for-byte no-op on the retained baseline:
    - `decodecorpus_z000079`: stayed `7,326`
    - `build_ruzstd-cli`: stayed `856,110`
  - CPU only drifted in the wrong direction on the fast screen
- dictionary `oldest` bias:
  - target moved the wrong way:
    - `dict_dictionary.bin`: `20,175 -> 20,177`
  - broad-local total worsened:
    - bytes-above-C on losing fixtures: `230 -> 232`

Useful conclusion:
- the remaining `decodecorpus_z000079` gap is not waiting on a looser adjacent-repeat probe gate
- the remaining `dict_dictionary.bin` gap is not just “oldest wins too cheaply”

## 2026-05-31 - Retained dense short-line probing for tiny level-1 `CodeText` and `ConfigText`

Change notes:
- Started from the retained large-`Unknown` repeat-bias baseline and targeted the remaining small text-side losses separately from the binary holdouts.
- Retained change:
  - for `CompressionFileType::CodeText` and `CompressionFileType::ConfigText`
  - only on short-line text blocks up to `8 KiB`
  - force dense no-match probing with step `1`

Source reports:
- [code-only stepping stone](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-codeprobe1-broad-local.md)
- [final broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-codeconfigprobe1-broad-local.md)
- [final fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-codeconfigprobe1-fast.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-codeconfigprobe1-retained)

Why it was retained:
- It nearly closed the last code-file gap:
  - `repo_main.rs`: `2,137 -> 2,105`
  - C `zstd -1`: `2,101`
- It also reduced the remaining small config/service losses:
  - `dict_systemd-logind.service`: `1,134 -> 1,122`
  - `dict_systemd-coredump@.service`: `690 -> 686`
- Broad-local summary vs C improved:
  - better / worse / equal stayed `16 / 13 / 3`
  - bytes-above-C on losing fixtures: `278 -> 230`
- The binary holdouts stayed exact:
  - `decodecorpus_z000079`: `7,326`
  - `dict_dictionary.bin`: `20,175`
  - `build_ruzstd-cli`: `856,110`

## 2026-05-31 - Retained a stronger still repeat-vs-normal bias for large level-1 `Unknown` blocks

Change notes:
- Followed the retained large-`Unknown` repeat-bias win with the next stronger point in the same family.
- Narrow retained change:
  - only on large Fastest `CompressionFileType::Unknown` non-text blocks
  - widen the repeat-vs-normal match-length margin from `4` to `5`

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-repeatbiasplus3-broad-local.md)
- [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-repeatbiasplus3-fast.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-unknown-repeatbiasplus3-retained)
- [archive inspect](/home/bsutton/git/zstd-rs/benchmarks/reports/archive-inspect/decodecorpus_z000079.unknown_repeatbiasplus3.l1.inspect.txt)

Why it was retained:
- It improves the main remaining loser again:
  - `decodecorpus_z000079`: `7,329 -> 7,326`
  - C `zstd -1`: `7,221`
- Broad-local summary vs C improved again:
  - better / worse / equal stayed `16 / 13 / 3`
  - bytes-above-C on losing fixtures: `281 -> 278`
- Fast guardrails stayed acceptable:
  - `decodecorpus_pack.bin`: unchanged at `5,319,265`
  - `json_logs_32m.jsonl`: unchanged at `690,084`
  - `dict_dictionary.bin`: unchanged at `20,175`
  - `repo_main.rs`: unchanged at `2,137`
- Known givebacks on already-winning binaries:
  - `build_ruzstd-cli`: `855,996 -> 856,110`
  - `decodecorpus_z000033`: `532,528 -> 532,592`
- Archive inspection shows the gain is still sequence/offset-side:
  - `sequence_payload_bytes`: `6469 -> 6464`
  - `decoded_literals`: `1460 -> 1464`
  - block 2 `of_extra_bits`: `4326 -> 4290`
  - block 1 `of_extra_bits` stayed flat at `12110`

## 2026-05-31 - Rejected the too-strong `+4` repeat-bias variant on large level-1 `Unknown` blocks

Change notes:
- After the retained `4 -> 5` point, tested one stronger variant in the same family:
  - widen the repeat-vs-normal margin from `5` to `6`

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-repeatbiasplus4-broad-local.md)
- [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-repeatbiasplus4-fast.md)

Why it was rejected:
- It moved the target back the wrong way:
  - `decodecorpus_z000079`: `7,329 -> 7,333`
- Broad-local summary vs C worsened versus the retained point:
  - bytes-above-C on losing fixtures: `281 -> 285`
- It also gave back more on already-winning binaries:
  - `build_ruzstd-cli`: `855,996 -> 856,134`
  - `decodecorpus_z000033`: `532,528 -> 532,616`
- So the family is now bounded:
  - `4` was good
  - `5` is better
  - `6` is worse

## 2026-05-31 - Retained a stronger repeat-vs-normal bias for large level-1 `Unknown` blocks

Change notes:
- Starting from the retained `unknown-smallhuff` baseline and the new `z000079` offset-bit diagnostics, tested whether the remaining gap would shrink if large Fastest `Unknown` non-text blocks let repeat matches beat normal matches with a wider length margin.
- Narrow retained change:
  - only on the large Fastest `Unknown` non-text path
  - increase the repeat-vs-normal match-length margin from `2` to `4`
  - no new file family, no new sidecar, no new entropy policy

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-repeatbias-broad-local.md)
- [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-repeatbias-fast.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-unknown-repeatbias-retained)
- [archive inspect](/home/bsutton/git/zstd-rs/benchmarks/reports/archive-inspect/decodecorpus_z000079.unknown_repeatbias.l1.inspect.txt)

Why it was retained:
- It improved the main remaining loser:
  - `decodecorpus_z000079`: `7,340 -> 7,329`
  - C `zstd -1`: `7,221`
- Broad-local summary vs C improved:
  - better / worse / equal stayed `16 / 13 / 3`
  - bytes-above-C on losing fixtures: `292 -> 281`
- Fast guardrails stayed acceptable:
  - `decodecorpus_pack.bin`: unchanged at `5,319,265`
  - `json_logs_32m.jsonl`: unchanged at `690,084`
  - `dict_dictionary.bin`: unchanged at `20,175`
  - `repo_main.rs`: unchanged at `2,137`
- Known givebacks on already-winning binary fixtures:
  - `build_ruzstd-cli`: `855,908 -> 855,996`
  - `decodecorpus_z000033`: `532,333 -> 532,528`
- Archive inspection shows this is a real sequence-side gain, not noise:
  - `sequence_payload_bytes`: `6480 -> 6469`
  - `decoded_literals`: `1455 -> 1460`
  - block 2 `of_extra_bits`: `4363 -> 4326`
  - block 1 `of_extra_bits` stayed flat at `12110`

## 2026-05-31 - Rejected the narrower `+1` repeat-bias variant on large level-1 `Unknown` blocks

Change notes:
- Followed the retained large-`Unknown` repeat-bias win with the narrower version:
  - increase the repeat-vs-normal match-length margin from `2` to `3` instead of `4`

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-repeatbiasplus1-broad-local.md)
- [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-repeatbiasplus1-fast.md)

Why it was rejected:
- It did nothing for the target:
  - `decodecorpus_z000079`: stayed `7,340`
- Broad-local summary stayed at the weaker retained point:
  - better / worse / equal stayed `16 / 13 / 3`
  - bytes-above-C on losing fixtures stayed `292`
- It still gave back bytes on already-winning binaries:
  - `build_ruzstd-cli`: `855,908 -> 856,018`
  - `decodecorpus_z000033`: `532,333 -> 532,439`
- That makes the family conclusion clear:
  - the retained signal starts at the stronger `+2` bump
  - the narrower `+1` version is just collateral without target benefit

## 2026-05-31 - Retained small-literal exact Huffman search for level-1 `Unknown` blocks

Change notes:
- After the retained large-`Unknown` no-repeat early-exit win, tested whether the remaining `decodecorpus_z000079` gap still had any entropy-side slack.
- Narrow retained change:
  - at level 1, `CompressionFileType::Unknown` now uses the same small-literal exact Huffman table search policy that `CodeText` and `ConfigText` already use
- This does not touch the matcher path. It only broadens the literal-table search on `Unknown` blocks with small literal sections.

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-smallhuff-broad-local.md)
- [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-smallhuff-fast.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-unknown-smallhuff-retained)
- [archive inspect](/home/bsutton/git/zstd-rs/benchmarks/reports/archive-inspect/decodecorpus_z000079.unknown_smallhuff.l1.inspect.txt)

Why it was retained:
- It improved the main remaining loser, even if only slightly:
  - `decodecorpus_z000079`: `7,344 -> 7,340`
  - C `zstd -1`: `7,221`
- Broad-local summary vs C improved:
  - better / worse / equal stayed `16 / 13 / 3`
  - bytes-above-C on losing fixtures: `296 -> 292`
- Fast guardrails stayed exact:
  - `decodecorpus_pack.bin`: `5,319,265`
  - `json_logs_32m.jsonl`: `690,084`
  - `xorshift_32m.bin`: `33,555,210`
- The archive diff confirms this is a literal-side win, not a parse change:
  - retained previous `z000079`: `literal_section_bytes=823`, `sequence_payload_bytes=6480`
  - new retained point: `literal_section_bytes=819`, `sequence_payload_bytes=6480`

## 2026-05-31 - Rejected two more narrow file-type follow-ups

Change notes:
- Tested two very narrow follow-ups after the retained no-repeat early-exit win:
  - `DictionaryText` matcher floor `5 -> 7`
  - `CodeText` matcher floor `6 -> 5` only for code-like files up to `8 KiB`
  - extend the retained small-block Fastest current-entry `second_newest` path to all `Unknown` non-text blocks while keeping the no-candidate gate
  - stop before older-entry search when the current entry already has a 16-byte non-repeat candidate on large `Unknown` blocks
  - offset-aware non-repeat comparison on large `Unknown` blocks when a smaller offset saves at least 4 offset-code bits for at most a 2-byte match loss

Source reports:
- dictionary threshold `7`:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dictionary-threshold7-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dictionary-threshold7-fast.md)
- tiny-code threshold `5`:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-small-code5-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-small-code5-fast.md)
- large `Unknown` `second_newest`:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-large-secondnewest-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-large-secondnewest-fast.md)
- current-entry cutoff:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-current-entry-cutoff-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-current-entry-cutoff-fast.md)
- offset-aware comparison:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-offsetaware-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-offsetaware-fast.md)

Why they were rejected:
- `DictionaryText` threshold `7`:
  - hard regression on the target:
    - `dict_dictionary.bin`: `20,175 -> 21,619`
  - fast guardrails stayed exact, but the dictionary regression is far too large to keep
- tiny-code threshold `5`:
  - only moved `repo_main.rs` by one byte:
    - `2,137 -> 2,136`
  - everything else stayed exact
  - that is not enough to justify another code-path split
- large `Unknown` `second_newest`:
  - exact byte-for-byte no-op on the retained `unknown-smallhuff` baseline
  - fast CPU drifted the wrong way:
    - `decodecorpus_pack.bin`: `0.22s -> 0.23s`
    - `json_logs_32m.jsonl`: `0.16s -> 0.17s`
  - so the old broad large-block second-newest idea is still exhausted even after the file-type split
- current-entry cutoff:
  - directly regressed the target:
    - `decodecorpus_z000079`: `7,340 -> 7,524`
  - and also nudged already-winning unknown fixtures the wrong way:
    - `build_ruzstd-cli`: `855,908 -> 856,008`
- offset-aware comparison:
  - exact byte-for-byte no-op on the retained baseline
  - fast CPU still drifted the wrong way:
    - `decodecorpus_pack.bin`: `0.22s -> 0.23s`
    - `repeated_text_32m.txt`: `0.06s -> 0.07s`

## 2026-05-31 - Added sequence-code archive diagnostics for `decodecorpus_z000079`

Change notes:
- Extended the ignored archive inspector in [ruzstd/src/tests/mod.rs](/home/bsutton/git/zstd-rs/ruzstd/src/tests/mod.rs) with:
  - per-block `ll_extra_bits`, `ml_extra_bits`, `of_extra_bits`
  - top code histograms for literal-length, match-length, and offset codes
- This is test-only diagnostic support. No runtime behavior changed.

What it showed on `decodecorpus_z000079`:
- C `zstd -1`:
  - `sequence_payload_bytes=5722`
  - `decoded_literals=2799`
  - `sequences=4354`
  - block 1 `of_extra_bits=2205`
  - block 2 `of_extra_bits=2298`
  - offset codes are dominated by `0`, with small counts in `6..13`
- current retained Rust:
  - `sequence_payload_bytes=6480`
  - `decoded_literals=1455`
  - `sequences=2804`
  - block 1 `of_extra_bits=12110`
  - block 2 `of_extra_bits=4363`
  - offset codes are dominated by `14`, `15`, `16` in block 1, and still heavily skewed to larger codes in block 2
- Useful conclusion:
  - the remaining `z000079` sequence-payload gap is overwhelmingly offset-bit cost, not literal cost
  - but the first two local attempts to exploit that clue both failed:
    - current-entry cutoff regressed the target
    - offset-aware candidate comparison was a no-op

## 2026-05-31 - Rejected large-Unknown split-family follow-ups for the remaining `decodecorpus_z000079` gap

Change notes:
- After the retained no-repeat early-exit win on large Fastest `Unknown` non-text blocks, tested a new family aimed at the same remaining `z000079` gap:
  - compare the whole block against a midpoint split candidate on that path
  - first with the existing estimate gate
  - then with an exact whole-vs-midpoint comparison that bypassed the estimate gate
- This was intentionally narrow:
  - level 1 only
  - `CompressionFileType::Unknown`
  - non-text
  - large Fastest blocks only

Source reports:
- estimate-gated split:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-split-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-split-fast.md)
- exact whole-vs-split compare:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-exactsplit-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-exactsplit-fast.md)

Why they were rejected:
- Both versions produced the same practical outcome:
  - `decodecorpus_z000079`: unchanged at `7,344`
  - broad-local bytes-above-C on losing fixtures stayed `296`
- They did improve several already-winning unknown-family fixtures:
  - `build_ruzstd-cli`: `855,908 -> 850,952`
  - `decodecorpus_z000028`: `98,592 -> 97,095`
  - `decodecorpus_z000033`: `532,333 -> 525,509`
- But the main target did not move, and the fast screen CPU drifted the wrong way:
  - `decodecorpus_pack.bin`: `0.22s -> 0.23s`
- Useful conclusion:
  - the remaining `z000079` gap is not waiting on a simple whole-vs-midpoint split candidate
  - bypassing the estimate gate did not change that, so the problem is not the split gate itself

## 2026-05-31 - Retained no-repeat early-exit disable for large Fastest Unknown non-text blocks

Change notes:
- Starting from the corrected live baseline (`DictionaryText` dense probe step plus retained `Unknown` `second_newest`), tested one parser-side change aimed directly at the remaining `decodecorpus_z000079` gap:
  - for large `CompressionFileType::Unknown` non-text blocks at level 1, do not let a long repeat match skip window search early
- This does not change candidate scoring or framing. It only keeps window competition alive after long repeat hits on that large unknown-family path.

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-no-repeat-earlyexit-broad-local.md)
- [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-no-repeat-earlyexit-fast.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-unknown-no-repeat-earlyexit-retained)

Why it was retained:
- It materially improved the main remaining loser:
  - `decodecorpus_z000079`: `7,518 -> 7,344`
  - C `zstd -1`: `7,221`
- It also improved several neighboring unknown-family winners without reopening the main level-1 guardrails:
  - `build_ruzstd-cli`: `856,479 -> 855,908`
  - `decodecorpus_z000028`: `98,656 -> 98,592`
  - `decodecorpus_z000033`: `532,424 -> 532,333`
- Broad-local summary vs C improved:
  - better / worse / equal stayed `16 / 13 / 3`
  - bytes-above-C on losing fixtures: `470 -> 296`
- Fast guardrails stayed byte-identical:
  - `decodecorpus_pack.bin`: `5,319,265`
  - `json_logs_32m.jsonl`: `690,084`
  - `xorshift_32m.bin`: `33,555,210`
- Archive inspection on the retained `z000079` output shows the win is not from matching C’s block shape:
  - retained corrected baseline: `7,518`, `decoded_literals=1,563`, `sequences=3,117`
  - new retained point: `7,344`, `decoded_literals=1,455`, `sequences=2,804`
  - so the improvement is coming from a better compressed-block parse under our own shape, not from becoming more C-like.

## 2026-05-31 - Rejected two more follow-ups after the retained no-repeat early-exit win

Change notes:
- After retaining the length-only no-repeat early-exit disable for large Fastest `Unknown` blocks, tested two direct follow-ups in the same family:
  - prefer smaller-offset non-repeat matches over slightly longer ones
  - remove the last remaining repeat early-exit too, including block-end exits

Source reports:
- small-offset non-repeat bias:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-smalloffset-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-smalloffset-fast.md)
- full repeat early-exit removal:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-no-repeat-end-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-no-repeat-end-fast.md)

Why they were rejected:
- Small-offset non-repeat bias:
  - it slightly improved a few already-winning unknown-family fixtures
  - but it did nothing for the main target:
    - `decodecorpus_z000079`: stayed `7,344`
  - and fast CPU got worse:
    - `decodecorpus_pack.bin`: `0.23s -> 0.25s`
- Full repeat early-exit removal:
  - exact byte-for-byte no-op versus the retained point
  - and fast CPU drifted the wrong way:
    - `decodecorpus_pack.bin`: `0.22s -> 0.25s`

## 2026-05-31 - Rejected two more `decodecorpus_z000079`-focused Unknown-family parser variants

Change notes:
- After diagnosing `z000079` as a repeat-heavy current-window case, tested two narrower follow-ups before the retained early-exit change:
  - complementary end-of-match insertion for large Fastest `Unknown` non-text blocks
  - reduce repeat-match advantage from `2` bytes to `1` for that same path

Source reports:
- complementary end insertion:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-complementary-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-complementary-fast.md)
- repeat-margin cut:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-repeatmargin-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-repeatmargin-fast.md)

Why they were rejected:
- Complementary end insertion:
  - slightly helped `build_ruzstd-cli`
  - but made the target worse:
    - `decodecorpus_z000079`: `7,518 -> 7,541`
  - and fast CPU drifted the wrong way:
    - `decodecorpus_pack.bin`: `0.23s -> 0.24s`
- Repeat-margin cut:
  - slightly helped some already-winning unknown-family fixtures
  - but it did nothing for `decodecorpus_z000079`:
    - `7,518 -> 7,518`
  - and CPU regressed on the fast screen:
    - `decodecorpus_pack.bin`: `0.23s -> 0.26s`
    - `json_logs_32m.jsonl`: `0.17s -> 0.18s`

## 2026-05-31 - Rejected two more `decodecorpus_z000079`-focused level-1 experiments on the corrected baseline

Change notes:
- After correcting the live `DictionaryText` baseline, tested two more targeted follow-ups for the remaining `decodecorpus_z000079` gap:
  - extend the retained dense non-text probe step to `CompressionFileType::Unknown` blocks up to `128 KiB`
  - let the retained `Unknown` non-text `second_newest` path also compete against a weak current min-length non-repeat match

Source reports:
- dense `Unknown` up to `128 KiB`:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-dense128-vs-dictstep1-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-dense128-vs-dictstep1-fast.md)
- weak-current `second_newest`:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-secondnewest-weakcurrent-vs-dictstep1-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-secondnewest-weakcurrent-vs-dictstep1-fast.md)

Why they were rejected:
- Dense `Unknown` up to `128 KiB`:
  - it helped some neighboring `decodecorpus_z...` fixtures
  - but it made the main remaining loser worse:
    - `decodecorpus_z000079`: `7,518 -> 7,531`
  - broad-local bytes-above-C on losing fixtures worsened:
    - `470 -> 483`
- Weak-current `second_newest`:
  - exact byte-for-byte no-op on the corrected retained baseline
  - and it nudged the fast CPU guardrails the wrong way:
    - `decodecorpus_pack.bin`: `0.22s -> 0.23s`

## 2026-05-31 - Retained DictionaryText rollback plus dense probe step on the corrected live level-1 baseline

Change notes:
- Reconciled the live source tree against the checked-in retained binaries and found that `DictionaryText` had drifted badly:
  - direct current CLI output on `dict_dictionary.bin` was `23,871`
  - the checked-in retained binaries were at `20,667`
- The live archive inspection showed the regression was matcher-side under-matching:
  - current live `23,871` archive:
    - `decoded_literals=22,688`
    - `sequences=1,974`
    - `sequence_payload_bytes=5,903`
  - older retained `20,667` archive:
    - `decoded_literals=12,603`
    - `sequences=3,988`
    - `sequence_payload_bytes=10,493`
- The direct cause was the `DictionaryText` threshold-8 override in the matcher.
- Retained two corrections on top of the live `Unknown` non-text `second_newest` baseline:
  - remove the `DictionaryText` threshold-8 override
  - give `DictionaryText` a fully dense text no-match probe step of `1`

Source reports:
- rollback threshold-8:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dictionary-no8-vs-live-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dictionary-no8-vs-live-fast.md)
- dense dictionary probe step:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dictionary-step1-vs-no8-broad-local.md)
  - [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dictionary-step1-vs-no8-fast.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-dictionary-step1-retained)

Why it was retained:
- Removing the threshold-8 override immediately restored the checked-in dictionary behavior:
  - `dict_dictionary.bin`: `23,871 -> 20,667`
  - broad-local bytes-above-C on losing fixtures: `4,166 -> 962`
- The dictionary-only dense probe step then improved the main remaining loser again:
  - `dict_dictionary.bin`: `20,667 -> 20,175`
  - C `zstd -1`: `20,145`
- Broad-local summary on the corrected retained baseline:
  - better / worse / equal vs C: `16 / 13 / 3`
  - bytes-above-C on losing fixtures: `470`
- Fast guardrails stayed exact across both retained corrections:
  - `decodecorpus_pack.bin`: `5,319,265`
  - `json_logs_32m.jsonl`: `690,084`
  - `xorshift_32m.bin`: `33,555,210`

## 2026-05-31 - Rejected Unknown non-text dense probe extension to 128 KiB

Change notes:
- Starting from the corrected retained baseline above, tested one more `decodecorpus_z000079`-focused matcher expansion:
  - extend the retained dense non-text probe step from `<= 64 KiB` to `CompressionFileType::Unknown` non-text blocks up to `128 KiB`

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-dense128-vs-dictstep1-broad-local.md)
- [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-dense128-vs-dictstep1-fast.md)

Why it was rejected:
- It helped some neighboring `decodecorpus_z...` fixtures and `build_ruzstd-cli`, but it moved the main remaining loser the wrong way:
  - `decodecorpus_z000079`: `7,518 -> 7,531`
- Broad-local summary also got worse:
  - bytes-above-C on losing fixtures: `470 -> 483`
- Fast guardrail bytes stayed exact, so this is a clean targeted rejection rather than a broad regression.

## 2026-05-31 - Retained Fastest Unknown non-text second-newest extension, and recorded live baseline drift

Change notes:
- While continuing the file-type starting-point work, I found that the live source tree no longer matched some older retained `DictionaryText` reports.
- A direct CLI check on `benchmarks/fixtures/broad-local/dict_dictionary.bin` currently produces `23,871` bytes, not the older reported `19,988`.
- I treated the live source tree as the authoritative baseline for this cycle and tested one narrow matcher change against that baseline:
  - keep the retained Fastest small-block `second_newest` path
  - extend it only to `CompressionFileType::Unknown` non-text blocks up to `128 KiB`
  - fix the probe gating so the extended path actually runs

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-secondnewest-fixed-broad-local.md)
- [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-secondnewest-fixed-fast.md)
- [focused mainish check](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-secondnewest-fixed-mainish.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-unknown-secondnewest-retained)

Why it was retained:
- It improves the remaining `decodecorpus_z...` unknown-family residuals without moving the main fast guardrails.
- Broad-local summary versus the live baseline improved:
  - better / worse / equal vs C stayed `16 / 13 / 3`
  - bytes-above-C on losing fixtures improved:
    - `4,188 -> 4,166`
- Main wins:
  - `decodecorpus_z000079`: `7,540 -> 7,518`
  - `decodecorpus_z000033`: `544,266 -> 532,424`
  - `decodecorpus_z000028`: `100,250 -> 98,656`
  - `decodecorpus_z000003`: `52,134 -> 51,006`
  - `build_ruzstd-cli`: `860,072 -> 856,479`
- Fast guardrails stayed exact:
  - `decodecorpus_pack.bin`: unchanged at `5,319,265`
  - `json_logs_32m.jsonl`: unchanged at `690,084`
  - `xorshift_32m.bin`: unchanged at `33,555,210`
- CPU shape:
  - fast guardrails stayed in-band
  - `build_ruzstd-cli` drifted `0.05s -> 0.06s`, which is acceptable for the current compression-first phase

## 2026-05-31 - Rejected two Unknown non-text matcher gates while isolating `decodecorpus_z000079`

Change notes:
- After identifying `decodecorpus_z000079` as the main remaining broad-local loser, tested two narrower `CompressionFileType::Unknown` non-text gates before the retained `second_newest` extension:
  - raise the non-repeat floor from `5` to `6`
  - lower the dense long-match indexing limit from `128` to `64`

Source reports:
- threshold-6:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-binary-threshold6-broad-local.md)
  - [mainish](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-binary-threshold6-mainish.md)
- dense-64:
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-dense64-broad-local.md)
  - [mainish](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-unknown-dense64-mainish.md)

Why they were rejected:
- Unknown non-text threshold `6`:
  - broad regression across the `decodecorpus_z...` family
  - `decodecorpus_z000079`: `7,540 -> 7,556`
  - `decodecorpus_z000033`: `544,266 -> 559,261`
  - `build_ruzstd-cli`: `860,072 -> 866,219`
- Unknown non-text dense limit `64`:
  - still hurt the target fixture
  - `decodecorpus_z000079`: `7,540 -> 7,548`
  - only marginal or noisy movement elsewhere
- Neither gate improved the live broad-local summary enough to justify keeping it.

## 2026-05-31 - Rejected follow-up entropy and framing experiments after the retained small CodeText/ConfigText Huffman change

Change notes:
- Tried four follow-ups after retaining small exact-Huffman search for `CodeText` / `ConfigText`:
  - widen `CodeText` / `ConfigText` exact-Huffman search from small literal sections to all literal sections
  - raise Fastest repeat-table reuse threshold from `64` to `2048` sequences
  - split long trailing single-byte runs into separate Fastest RLE blocks
  - use the known CLI file size to emit a single-segment frame header

Source reports:
- [all-literal Code/Config broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-code-config-allhuff-broad-local.md)
- [repeat-table 2048 broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fse-repeat2048-broad-local.md)
- [trailing-RLE broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-trailing-rle-broad-local.md)
- [frame-content-size broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fcs-broad-local.md)

Why they were rejected:
- `CodeText` / `ConfigText` all-literal exact-Huffman:
  - exact no-op versus the retained small-literal policy
- repeat-table reuse `64 -> 2048`:
  - exact no-op on bytes, with no useful CPU upside
- trailing-RLE block split:
  - hard regression on the target fixture:
    - `decodecorpus_z000079`: `7,540 -> 8,338`
  - broad-local bytes-above-C on losing fixtures worsened:
    - `984 -> 1,782`
- CLI frame-content-size single-segment hint:
  - broadly made files 1 to 3 bytes larger
  - examples:
    - `dict_dictionary.bin`: `20,667 -> 20,668`
    - `repo_main.rs`: `2,137 -> 2,138`
    - `build_ruzstd-cli`: `860,072 -> 860,075`

## 2026-05-31 - Retained small CodeText/ConfigText exact-Huffman literal search as a level-1 file-type starting point

Change notes:
- Kept the matcher path unchanged.
- Broadened exact Huffman table search only for small literal sections on:
  - `CompressionFileType::CodeText`
  - `CompressionFileType::ConfigText`
- Left these unchanged:
  - `JsonText`
  - `DictionaryText`
  - all non-text matcher behavior

Source reports:
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-code-config-smallhuff-broad-local.md)
- [focused check](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-code-config-smallhuff-mainish.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-filetype-smallhuff-retained)

Why it was retained:
- It improved eight fixtures on the broad-local suite and regressed none.
- Broad-local summary vs C improved:
  - better / worse / equal: `15 / 14 / 3 -> 16 / 13 / 3`
  - bytes above C on losing fixtures: `1,005 -> 984`
- Main wins:
  - `dict_NetworkManager-dispatcher.service`: `395 -> 391`
  - `dict_fstrim.service`: `312 -> 308`
  - `dict_ftpd.service`: `172 -> 168`
  - `dict_netctl@.service`: `212 -> 206`
  - `dict_systemd-coredump@.service`: `692 -> 690`
  - `dict_systemd-udev-settle.service`: `569 -> 568`
  - `repo_main.rs`: `2,141 -> 2,137`
  - `repo_match_generator.rs`: `22,591 -> 22,587`
- It did not disturb the retained binary and JSON guardrails:
  - `build_libruzstd.rlib`: unchanged at `611,155`
  - `build_ruzstd-cli`: unchanged at `860,072`
  - `dict_dictionary.bin`: unchanged at `20,667`
  - `generated_json_logs_001m.jsonl`: unchanged at `58,767`

## 2026-05-31 - Rejected DictionaryText wider text probe step as a level-1 starting point

Change notes:
- Tried the first dictionary-specific file-type starting point using the new path/file-type API:
  - keep all retained level-1 behavior unchanged for other file families
  - but for `CompressionFileType::DictionaryText`, use the wider text no-match probe step instead of the retained short-line dense step

Source reports:
- [main](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dictionary-textstep3-main.md)
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dictionary-textstep3-broad-local.md)

Why it was rejected:
- It made the target dictionary fixture materially worse:
  - `dict_dictionary.bin`: `20,667 -> 21,302`
- Broad-local summary stayed `15 / 14 / 3` better / worse / equal vs C, but total bytes above C on the losing fixtures got worse:
  - `1,005 -> 1,640`
- Main guardrail bytes stayed exact, so this is a clean parse-shape rejection rather than a broad regression:
  - `decodecorpus_pack.bin`: unchanged at `5,319,265`
  - `json_logs_32m.jsonl`: unchanged at `690,084`
- The archive-inspection clue that `dict_dictionary.bin` is overmatching was real, but widening the text probe step is not the right correction.

## 2026-05-31 - Rejected archive-like dense binary probing as a level-1 starting point

Change notes:
- Tried the first archive-specific file-type starting point using the new path/file-type API:
  - keep all retained level-1 behavior unchanged for other file families
  - but for `CompressionFileType::ArchiveLike`, use dense non-text probing across block sizes instead of the retained `<= 64 KiB` gate

Source reports:
- [main](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-archive-dense-start-main.md)
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-archive-dense-start-broad-local.md)
- [main repeat](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-archive-dense-start-main-repeat.md)
- [build-lib repeat](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-archive-dense-start-buildlib-repeat.md)

Why it was rejected:
- It produced one real compression win:
  - `build_libruzstd.rlib`: `611,155 -> 600,329`
- But that win did not justify the repeat CPU cost:
  - `build_libruzstd.rlib`: `0.03s -> 0.04s`
- It did not improve the broad-local gap against C:
  - better / worse / equal stayed `15 / 14 / 3`
  - bytes above C on losing fixtures stayed `1,005`
- Main fixture bytes stayed exact, but repeat CPU did not improve there either:
  - `decodecorpus_pack.bin`: unchanged at `5,319,265`, repeat CPU flat in the `0.22s` band
- So archive-like dense probing is a useful compression-only direction for some build artifacts, but not a retained level-1 starting point.

## 2026-05-31 - Rejected C-shaped `ip + step` repeat probe for Fastest non-text blocks

Change notes:
- Tried the closest narrow analogue of the C `ZSTD_fast` adjacent-position repeat check from `zstd_fast.c`:
  - for Fastest non-text blocks
  - only when the current position had no candidate
  - probe repeat at `ip + step` instead of just the retained `ip + 1` lookahead

Source reports:
- [main](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-step-repeat-main.md)

Why it was rejected:
- It failed the main guardrail immediately:
  - `decodecorpus_pack.bin`: `5,319,265 -> 5,330,232`
  - CPU: `0.22s -> 0.26s`
- `json_logs_32m.jsonl` stayed flat, but that does not matter once the main binary fixture moves the wrong way on both size and CPU.
- So this specific `ZSTD_fast`-shaped `ip + step` repeat probe does not transfer cleanly into the current Rust matcher representation.

## 2026-05-31 - Rejected compressibility-gated dense probing for Fastest non-text blocks

Change notes:
- Tried a broader follow-up to the retained small-block dense no-match probe:
  - keep text excluded
  - keep xorshift-like incompressible binary blocks excluded
  - but use byte-by-byte no-match probing for all compressible Fastest non-text blocks, not just blocks up to `64 KiB`

Source reports:
- [main](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-dense-compressible-main.md)
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-dense-compressible-broad-local.md)

Why it was rejected:
- It was a real compression direction, but the CPU hit was too large on the main binary guardrail:
  - `decodecorpus_pack.bin`: `5,319,265 -> 5,219,513`
  - CPU: `0.21s -> 0.28s`
- Broad-local also moved in a mixed way:
  - large wins:
    - `build_libruzstd.rlib`: `611,155 -> 600,329`
    - `build_ruzstd-cli`: `860,072 -> 846,556`
    - `decodecorpus_z000028`: `100,250 -> 97,386`
    - `decodecorpus_z000033`: `544,266 -> 533,010`
  - but one of the stubborn remaining losers moved the wrong way:
    - `decodecorpus_z000079`: `7,540 -> 7,565`
  - total bytes above C on worse fixtures also got slightly worse:
    - `1,005 -> 1,030`
- So this is useful evidence for a higher-compression binary direction, but not a retained level-1 point.

## 2026-05-31 - Rejected weak-current widening of the Fastest small-block `second_newest` probe

Change notes:
- Tried a narrow follow-up to the retained small-block `second_newest` baseline:
  - keep the same Fastest non-text block-size gate (`<= 64 KiB`)
  - keep the same current-entry sidecar
  - but allow `second_newest` to compete not only when there is no candidate, but also when the current candidate is a weak minimum-length non-repeat match

Source reports:
- [main](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-small-secondnewest-weakcurrent-main.md)
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-small-secondnewest-weakcurrent-broad-local.md)

Why it was rejected:
- It was a complete no-op on both suites.
- Main level-1 guardrails stayed exact:
  - `decodecorpus_pack.bin`: `5,319,265`
  - `json_logs_32m.jsonl`: `690,084`
- Broad-local summary stayed exact:
  - better / worse / equal vs C stayed `15 / 14 / 3`
  - total bytes above C on worse fixtures stayed `1,005`
- So the remaining binary gap is not waiting on “weak current non-repeat” widening of this `second_newest` path.

## 2026-05-31 - Retained dense no-match probing for small Fastest non-text blocks

Change notes:
- Added a narrow binary-path search-density change on top of the retained Fastest non-text `ip+1` repeat-lookahead and small-block `second_newest` baseline:
  - for Fastest non-text blocks up to `64 KiB`
  - force byte-by-byte no-match probing
  - leave text, large binary blocks, and current-candidate paths unchanged

Source reports:
- [main](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-dense-smallbin-main.md)
- [main repeat](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-dense-smallbin-main-repeat.md)
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-dense-smallbin-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-fastest-dense-smallbin-retained)

Why it was kept:
- Main level-1 guardrails stayed exact:
  - `decodecorpus_pack.bin`: `5,319,265`
  - `json_logs_32m.jsonl`: `690,084`
- Repeat run kept the main CPU shape in the same band:
  - `decodecorpus_pack.bin`: `0.22s -> 0.21s`
  - `json_logs_32m.jsonl`: `0.16s -> 0.16s`
- Broad-local compression gap vs C improved again:
  - better / worse / equal moved `14 / 15 / 3 -> 15 / 14 / 3`
  - total bytes above C on worse fixtures improved:
    - `1,022 -> 1,005`
- Notable wins:
  - `decodecorpus_z000030`: `13,463 -> 13,152`
  - `decodecorpus_z000031`: `116 -> 112`
  - `decodecorpus_z000054`: `9,628 -> 9,567`
  - `decodecorpus_z000080`: `2,635 -> 2,603`
    - now smaller than C `zstd -1` (`2,613`)
- Known tradeoffs:
  - `decodecorpus_z000059`: `702 -> 711`
  - `dict_fstrim.service`: `304 -> 312`
  - `dict_dictionary.bin`: unchanged at `20,667`
  - `decodecorpus_z000079`: unchanged at `7,540`

## 2026-05-31 - Rejected Fastest small-block oldest-first current-window probing for level-1 binary blocks

Change notes:
- Tried a probe-order-only follow-up to the retained small-block current-entry `second_newest` baseline:
  - for Fastest non-text blocks up to `64 KiB`
  - only when the current position has no candidate
  - prefer `oldest` before `newest` in current-window probing

Source reports:
- [main](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-small-oldestfirst-main.md)
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-small-oldestfirst-broad-local.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-level1-fastest-small-secondnewest-after-oldestfirst.md)

Why it was rejected:
- It was byte-identical on both suites.
- Main CPU got worse:
  - `decodecorpus_pack.bin`: `0.22s -> 0.27s`
  - `json_logs_32m.jsonl`: `0.17s -> 0.22s`
- Broad-local showed no byte movement at all versus the retained small-block `second_newest` baseline.
- So this closes off the small-block no-candidate oldest-first probe-order variant.

## 2026-05-30 - Retained Fastest small-block current-entry second-newest probe for level-1 binary blocks

Change notes:
- Added a narrow current-entry binary-path change on top of the retained Fastest non-text `ip+1` repeat-lookahead baseline:
  - enable a current-entry `second_newest` probe only for Fastest non-text blocks up to `64 KiB`
  - only probe it when the current position has no candidate

Source reports:
- [main](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-small-secondnewest-main.md)
- [main repeat](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-small-secondnewest-main-repeat.md)
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-small-secondnewest-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-fastest-small-secondnewest-retained)

Why it was kept:
- Main level-1 guardrails stayed exact:
  - `decodecorpus_pack.bin`: `5,319,265`
  - `json_logs_32m.jsonl`: `690,084`
- Repeat run kept the main CPU shape in the same band:
  - `decodecorpus_pack.bin`: `0.20s -> 0.21s`
  - `json_logs_32m.jsonl`: `0.16s -> 0.16s`
- Broad-local compression gap vs C improved:
  - better / worse / equal stayed `14 / 15 / 3`
  - total bytes above C on worse fixtures improved:
    - `1,073 -> 1,022`
- Notable wins:
  - `decodecorpus_z000030`: `13,545 -> 13,463`
  - `decodecorpus_z000054`: `9,756 -> 9,628`
  - `decodecorpus_z000059`: `717 -> 702`
  - `decodecorpus_z000080`: `2,669 -> 2,635`
  - `dict_NetworkManager-dispatcher.service`: `398 -> 396`

## 2026-05-30 - Rejected Fastest no-candidate 4-byte current-entry hash for level-1 binary blocks

Change notes:
- Tried a different current-entry representation than the rejected long-hash and second-candidate paths:
  - add a Fastest-only current-entry 4-byte hash for non-text blocks
  - only probe it when the current position has no candidate

Source reports:
- [main](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-shorthash-nocand-main.md)
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-shorthash-nocand-broad-local.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-level1-fastest-binary-nextrep-after-shorthash-nocand.md)

Why it was rejected:
- It improved the main binary guardrail substantially:
  - `decodecorpus_pack.bin`: `5,319,265 -> 5,282,033`
- But CPU still moved the wrong way:
  - `decodecorpus_pack.bin`: `0.20s -> 0.23s`
- Broad-local did not improve overall:
  - total bytes above C on worse fixtures: `1,073 -> 1,098`
  - big wins:
    - `build_ruzstd-cli`: `860,072 -> 855,340`
    - `build_libruzstd.rlib`: `611,155 -> 608,526`
    - `decodecorpus_z000028`: `100,250 -> 98,140`
    - `decodecorpus_z000033`: `544,266 -> 541,477`
  - but still-regressing fixtures moved the wrong way:
    - `decodecorpus_z000079`: `7,540 -> 7,565`
    - `dict_dictionary.bin`: unchanged at `20,667`
- So this is another real compression direction, but not a retained level-1 point.

## 2026-05-30 - Rejected Fastest no-candidate current-entry long-hash gate for level-1 binary blocks

Change notes:
- Tried a narrower follow-up to the rejected Fastest current-entry long-hash experiment:
  - enable the current-entry long-hash candidate for large Fastest non-text blocks
  - but only probe it when the current position has no candidate yet

Source reports:
- [main](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-longhash-nocand-main.md)
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-longhash-nocand-broad-local.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-level1-fastest-binary-nextrep-after-longhash-nocand.md)

Why it was rejected:
- It improved the main binary guardrail further:
  - `decodecorpus_pack.bin`: `5,319,265 -> 5,301,816`
- But CPU still moved the wrong way:
  - `decodecorpus_pack.bin`: `0.20s -> 0.24s`
- Broad-local regressed overall instead of improving:
  - total bytes above C on worse fixtures: `1,073 -> 1,147`
  - losses:
    - `build_libruzstd.rlib`: `611,155 -> 611,997`
    - `build_ruzstd-cli`: `860,072 -> 860,496`
    - `decodecorpus_z000079`: `7,540 -> 7,614`
  - wins:
    - `decodecorpus_z000028`: `100,250 -> 99,877`
    - `decodecorpus_z000033`: `544,266 -> 543,668`
- So the narrower no-candidate gate is still not retainable in this long-hash shape.

## 2026-05-30 - Rejected Fastest current-entry long-hash candidate for level-1 binary blocks

Change notes:
- Tried a narrower representation change than the rejected Fastest second-candidate sidecar:
  - enable the existing current-entry long-hash candidate machinery for large Fastest non-text blocks
  - keep the retained Fastest non-text `ip+1` repeat-lookahead path intact

Source reports:
- [main](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-longhash-main.md)
- [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-longhash-broad-local.md)

Why it was rejected:
- It improved the main binary guardrail:
  - `decodecorpus_pack.bin`: `5,319,265 -> 5,304,504`
- But CPU still moved the wrong way:
  - `decodecorpus_pack.bin`: `0.20s -> 0.24s`
- Broad-local was mixed rather than cleanly better:
  - wins:
    - `decodecorpus_z000028`: `100,250 -> 99,892`
    - `decodecorpus_z000033`: `544,266 -> 543,848`
  - losses:
    - `build_libruzstd.rlib`: `611,155 -> 612,668`
    - `build_ruzstd-cli`: `860,072 -> 860,798`
    - `decodecorpus_z000079`: `7,540 -> 7,579`
- So this is another real compression direction, but not a retained level-1 point in its current shape.

## 2026-05-30 - Rejected Fastest current-entry second-newest candidate for level-1 binary blocks

Change notes:
- Tried a broader binary-path compression move on top of the retained Fastest non-text `ip+1` repeat-lookahead baseline:
  - track a current-entry second-newest candidate for large Fastest non-text blocks
- Then tried the narrower follow-up:
  - only probe that extra current-entry candidate when there is no candidate yet

Source reports:
- full version:
  - [main](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-secondnewest-main.md)
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-secondnewest-broad-local.md)
- narrowed no-candidate variant:
  - [main](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-secondnewest-nocand-main.md)

Why it was rejected:
- The compression signal was strong:
  - `decodecorpus_pack.bin`: `5,319,265 -> 5,259,216`
  - `build_libruzstd.rlib`: `611,155 -> 609,561`
  - `build_ruzstd-cli`: `860,072 -> 856,479`
  - `decodecorpus_z000033`: `544,266 -> 532,424`
- But CPU cost was too large:
  - `decodecorpus_pack.bin`: `0.21s -> 0.25s`
  - `build_libruzstd.rlib`: `0.03s -> 0.04s`
  - `build_ruzstd-cli`: `0.05s -> 0.06s`
- Narrowing it to the “no candidate yet” case did not change the main result at all:
  - `decodecorpus_pack.bin`: still `5,259,216 @ 0.25s`
- So this is a useful compression-direction result, but not a retained level-1 point.

## 2026-05-30 - Rejected three Fastest non-text CPU-cut variants on top of the retained binary `ip+1` repeat win

Change notes:
- After keeping the level-1 Fastest non-text `ip+1` repeat lookahead win, tested three follow-up CPU cuts:
  - skip that next-position repeat probe at zero-literal positions
  - enable next-position window lookahead for Fastest non-text blocks
  - route Fastest non-text blocks through a dedicated parser loop that preserves the retained `ip+1` repeat behavior while removing shared-loop feature branches

Source reports:
- zero-literal gate:
  - [main](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-binary-nextrep-zerolit-main.md)
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-binary-nextrep-zerolit-broad-local.md)
- next-position window lookahead:
  - [main](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-binary-nextwindow-main.md)
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-binary-nextwindow-broad-local.md)
- dedicated Fastest non-text loop split:
  - [main](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastbinary-split-main.md)
  - [broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastbinary-split-broad-local.md)

Why they were rejected:
- Zero-literal gate:
  - looked plausible because diagnostics showed no final zero-literal `RepeatNextPosition` wins
  - but it still gave back real compression:
    - `decodecorpus_pack.bin`: `5,319,265 -> 5,321,178`
    - `build_libruzstd.rlib`: `611,155 -> 618,065`
    - `build_ruzstd-cli`: `860,072 -> 866,916`
    - `decodecorpus_z000079`: `7,540 -> 8,376`
- Fastest non-text `ip+1` window lookahead:
  - complete no-op on both suites
- Dedicated Fastest non-text parser loop split:
  - byte-identical, but no CPU win
  - broad-local even nudged `build_ruzstd-cli` from `0.04s` to `0.05s`

Useful diagnostic conclusion kept from this cycle:
- On both:
  - `decodecorpus_pack.bin`
  - `build_ruzstd-cli`
- every retained `RepeatNextPosition` win came from:
  - `NoCurrentCandidate`
- none came from:
  - `BeatsCurrentMinNonRepeat`
- but final zero-literal counts were not a safe gating signal, because zero-literal probes still changed the eventual parse enough to hurt compression.

## 2026-05-30 - Retained level-1 binary `ip+1` repeat lookahead for non-text blocks

Change notes:
- Added a level-1-only binary-path change in the matcher:
  - keep the retained short-line text code/config split unchanged
  - for `CompressionLevel::Fastest`, enable `ip+1` repeat lookahead only on non-text blocks
- This is the narrowest `double_fast`-shaped adjacent-position experiment on the level-1 path so far.

Source reports:
- [main level 1 screen](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-binary-nextrep-main.md)
- [main repeat check](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-binary-nextrep-main-repeat.md)
- [broad-local level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-binary-nextrep-broad-local.md)
- [focused binary repeat check](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-binary-nextrep-repeat.md)
- [retained level-1 binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-fastest-binary-nextrep-retained)

Why it was kept:
- Main level-1 guardrails:
  - `decodecorpus_pack.bin`: `5,323,478 -> 5,319,265`
  - `json_logs_32m.jsonl`: unchanged at `690,084`
  - `repeated_text_32m.txt`: unchanged at `2,874`
  - `xorshift_32m.bin`: unchanged at `33,555,210`
- Main repeat check confirmed the decodecorpus improvement exactly:
  - `decodecorpus_pack.bin`: stayed `5,319,265`
  - `json_logs_32m.jsonl`: stayed `690,084`
- Broad-local compression gap vs C improved materially:
  - better / worse / equal stayed `14 / 15 / 3`
  - total bytes above C on the fixtures where we still lose improved:
    - `1,909 -> 1,073`
- Largest broad-local gains:
  - `build_libruzstd.rlib`: `619,650 -> 611,155`
  - `build_ruzstd-cli`: `867,739 -> 860,072`
  - `decodecorpus_z000079`: `8,372 -> 7,540`

Tradeoffs:
- Main repeat CPU moved the wrong way on the primary binary guardrail:
  - `decodecorpus_pack.bin`: `0.18s -> 0.20s`
- `build_ruzstd-cli` broad-local repeat CPU also drifted slightly:
  - `0.04s -> 0.05s`
- So this is a retained compression win, not a CPU win.

## 2026-05-30 - Rejected level-1 binary oldest-first window probing on non-text blocks

Change notes:
- Tried a level-1-only binary-path experiment in the matcher:
  - keep the retained short-line text code/config split unchanged
  - on `CompressionLevel::Fastest`, probe window candidates oldest-first only for non-text blocks
- The goal was to see whether the remaining level-1 binary losses, especially `decodecorpus_z000079`, wanted a `Best`-style older-first probe order without disturbing the retained text wins.

Source reports:
- [broad-local level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-binary-oldestfirst-broad-local.md)
- [focused repeat check](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-fastest-binary-oldestfirst-repeat.md)

Why it was rejected:
- Broad-local bytes stayed completely unchanged across all 32 fixtures.
- The only visible CPU movement in the first pass was:
  - `build_ruzstd-cli`: `0.05s -> 0.04s`
  - `decodecorpus_z000033`: `0.02s -> 0.01s`
- But the focused repeat check on those two binary fixtures collapsed back to flat:
  - `build_ruzstd-cli`: `0.04s -> 0.04s`
  - `decodecorpus_z000033`: `0.02s -> 0.02s`
- So this was timing noise, not a retained CPU win.

## 2026-05-30 - Rejected split short-line probe step by code-like vs config-like text

Change notes:
- Tried to keep the retained code/config threshold split, but make the denser short-line probe step apply only to non-code short-line text.
- The goal was to see whether the remaining dictionary/config wins came mostly from the denser probe step, while letting code-like short-line text fall back to the wider text step.

Source reports:
- [main level 1 screen](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-shortline-probestep-split-main.md)
- [broad-local level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-shortline-probestep-split-broad-local.md)

Why it was rejected:
- Main level-1 guardrails stayed exact:
  - `decodecorpus_pack.bin`: `5,323,478`
  - `json_logs_32m.jsonl`: `690,084`
- But the broad-local total moved the wrong way:
  - bytes-above-C on worse fixtures: `1,909 -> 2,035`
- It preserved the dictionary/config wins, but gave back the recovered code-file wins:
  - `repo_match_generator.rs`: `22,591 -> 22,883`
  - `repo_main.rs`: `2,141 -> 2,181`
- So the branch was restored to the retained code/config threshold split baseline.

## 2026-05-30 - Retained level-1 code-like short-line text split

Change notes:
- Refined the retained short-line text path by splitting it into:
  - code-like short-line text: threshold `6`
  - non-code short-line text: threshold `5`
  - long-line text: threshold `8`
- This was aimed directly at the tradeoff from the retained `6 -> 5` step, where `dict_dictionary.bin` improved but `repo_match_generator.rs` crossed back to slightly worse than C.

Source reports:
- [main level 1 screen](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-shortline-code6-vs5-main.md)
- [broad-local level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-shortline-code6-vs5-broad-local.md)
- [retained level-1 binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-shortline-code6-retained)

Why it was kept:
- Main level-1 guardrails stayed byte-identical:
  - `decodecorpus_pack.bin`: `5,323,478`
  - `json_logs_32m.jsonl`: `690,084`
- Broad-local compression gap vs C improved again:
  - better / worse / equal moved from `13 / 16 / 3` to `14 / 15 / 3`
  - total bytes above C on the fixtures where we still lose improved:
    - `2,196 -> 1,909`
- It recovered the source-text regression while keeping the bigger dictionary/config wins:
  - `repo_match_generator.rs`: `23,085 -> 22,591` (C: `22,797`)
  - `dict_dictionary.bin`: stayed `20,667`
  - `dict_systemd-logind.service`: stayed `1,134`
  - `dict_systemd-coredump@.service`: stayed `692`
- Tradeoff:
  - `repo_main.rs`: `2,140 -> 2,141`
  - a negligible giveback of 1 byte

## 2026-05-30 - Retained level-1 short-line text threshold `6 -> 5`

Change notes:
- Built directly on the retained short-line threshold `6` plus short-line probe-step baseline.
- Lowered the short-line text non-repeat threshold one final step, from `6` to `5`, while keeping:
  - the short-line text gate
  - the denser short-line probe step
  - the long-line JSON path unchanged

Source reports:
- [main level 1 screen](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-shortline5-vs6step2-main.md)
- [broad-local level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-shortline5-vs6step2-broad-local.md)
- [retained level-1 binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-shortline5-step2-retained)

Why it was kept:
- Main level-1 guardrails stayed byte-identical:
  - `decodecorpus_pack.bin`: `5,323,478`
  - `json_logs_32m.jsonl`: `690,084`
- Broad-local compression gap vs C improved again:
  - better / worse / equal moved from `14 / 15 / 3` to `13 / 16 / 3`
  - but total bytes above C on the fixtures where we still lose improved:
    - `2,411 -> 2,196`
- Largest gains:
  - `dict_dictionary.bin`: `21,157 -> 20,667`
  - `dict_systemd-logind.service`: `1,144 -> 1,134`
  - `dict_systemd-coredump@.service`: `694 -> 692`
  - `repo_main.rs`: `2,141 -> 2,140`
- Tradeoff:
  - `repo_match_generator.rs`: `22,591 -> 23,085`
  - C `zstd -1`: `22,797`
  - so this crosses back to slightly worse than C on that one source-text fixture, even though the total bytes-above-C picture is still better overall

## 2026-05-30 - Retained level-1 short-line text threshold `7 -> 6`

Change notes:
- Built directly on the retained short-line text threshold plus short-line probe-step baseline.
- Lowered the short-line text non-repeat threshold one more step, from `7` to `6`, while keeping:
  - the short-line text gate
  - the denser short-line probe step
  - the long-line JSON path unchanged

Source reports:
- [main level 1 screen](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-shortline6-vs7step2-main.md)
- [broad-local level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-shortline6-vs7step2-broad-local.md)
- [retained level-1 binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-shortline6-step2-retained)

Why it was kept:
- Main level-1 guardrails stayed byte-identical:
  - `decodecorpus_pack.bin`: `5,323,478`
  - `json_logs_32m.jsonl`: `690,084`
- Broad-local compression gap vs C improved again:
  - better / worse / equal stayed `14 / 15 / 3`
  - total bytes above C on the fixtures where we still lose improved:
    - `3,408 -> 2,411`
- Largest gains:
  - `dict_dictionary.bin`: `22,101 -> 21,157`
  - `repo_main.rs`: `2,174 -> 2,141`
  - `dict_systemd-coredump@.service`: `708 -> 694`
  - `dict_systemd-logind.service`: `1,150 -> 1,144`
- Tradeoff:
  - `repo_match_generator.rs`: `22,469 -> 22,591`
  - but C `zstd -1` is still `22,797`, so this remained better than C and was acceptable against the broader gain

## 2026-05-30 - Retained level-1 short-line text probe step `3 -> 2`

Change notes:
- Built directly on the retained short-line text threshold gate.
- Kept the retained `7`-byte non-repeat minimum for short-line text, and additionally used the denser no-match probe step on those short-line text blocks only.
- This is the selective version of the old rejected global text probe-step change.

Source reports:
- [main level 1 screen](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-shortline7-step2-vsstep3-main.md)
- [broad-local level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-shortline7-step2-vsstep3-broad-local.md)
- [retained level-1 binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-shortline7-step2-retained)

Why it was kept:
- Main level-1 guardrails stayed byte-identical:
  - `decodecorpus_pack.bin`: `5,323,478`
  - `json_logs_32m.jsonl`: `690,084`
- Broad-local compression gap vs C improved again:
  - better / worse / equal moved from `14 / 15 / 3`
  - total bytes above C on the fixtures where we still lose improved:
    - `4,059 -> 3,408`
- The two biggest remaining short-line text losers both improved:
  - `dict_dictionary.bin`: `22,634 -> 22,101`
  - `repo_main.rs`: `2,211 -> 2,174`
- The previous retained source-text win was preserved and strengthened:
  - `repo_match_generator.rs`: `22,876 -> 22,469`
  - C `zstd -1`: `22,797`

## 2026-05-30 - Retained level-1 short-line text threshold `8 -> 7`

Change notes:
- Replaced the earlier `64 KiB` size gate with a text-shape gate.
- The matcher now uses a `7`-byte non-repeat minimum on short-line text blocks, instead of only on text blocks under a size cutoff.
- This reaches larger source-like text such as `repo_match_generator.rs` without reopening the long-line JSON failure.

Source reports:
- [main level 1 screen](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-shortline7-vs64k-main.md)
- [broad-local level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-shortline7-vs64k-broad-local.md)
- [retained level-1 binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-shortline7-retained)

Why it was kept:
- Main level-1 guardrails stayed byte-identical:
  - `decodecorpus_pack.bin`: `5,323,478`
  - `json_logs_32m.jsonl`: `690,084`
- Broad-local compression gap vs C improved again:
  - better / worse / equal counts stayed `13 / 16 / 3`
  - total bytes above C on the fixtures where we still lose improved:
    - `4,908 -> 4,059`
- The main win was exactly where the previous retained point was still weak:
  - `repo_match_generator.rs`: `23,725 -> 22,876`
  - C `zstd -1`: `22,797`
- Earlier retained wins were preserved:
  - `dict_dictionary.bin`: stayed `22,634`
  - `repo_main.rs`: stayed `2,211`
  - `dict_systemd-logind.service`: stayed `1,152`
  - `dict_systemd-coredump@.service`: stayed `708`

## 2026-05-30 - Retained level-1 small-text threshold `8 -> 7` for text blocks under `64 KiB`

Change notes:
- Separated “this block is text” from “which text threshold to use” inside the matcher.
- This fixed an important hidden coupling in the earlier `8 -> 7` experiments: changing the threshold had also implicitly changed text window sizing and text miss-step behavior because those paths were keyed off the retained `8` constant.
- With that untangled, kept a narrower level-1 text-path change:
  - use a `7`-byte non-repeat minimum only on text blocks smaller than `64 KiB`
  - keep the retained `8`-byte threshold on `128 KiB` streaming text blocks
  - keep the text probe step and text window behavior tied to text classification itself, not to the exact threshold value

Source reports:
- [main level 1 screen](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-smalltext7-64k-vs8-main.md)
- [broad-local level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-smalltext7-64k-vs8-broad-local.md)
- [retained level-1 binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-smalltext7-64k-retained)

Why it was kept:
- Main level-1 guardrails stayed byte-identical:
  - `decodecorpus_pack.bin`: unchanged at `5,323,478`
  - `json_logs_32m.jsonl`: unchanged at `690,084`
- Broad-local compression gap vs C improved materially:
  - better / worse / equal counts stayed `13 / 16 / 3`
  - but total bytes above C on the fixtures where we still lose improved:
    - `6,578 -> 4,908`
- Largest retained broad-local gains:
  - `dict_dictionary.bin`: `24,237 -> 22,634`
  - `repo_main.rs`: `2,249 -> 2,211`
  - `dict_systemd-logind.service`: `1,175 -> 1,152`
  - `dict_systemd-coredump@.service`: `722 -> 708`
- Tradeoffs:
  - `repo_match_generator.rs` moved slightly the wrong way: `23,717 -> 23,725`
  - main-screen CPU stayed in the same noise band, so this is a compression keep, not a CPU keep

## 2026-05-30 - Rejected level-1 small-text threshold `8 -> 7` for text blocks under `256 KiB`

Change notes:
- First attempt at salvaging the earlier global `8 -> 7` win by restricting it to “smaller text blocks”.
- The `256 KiB` cutoff was still above the compressor’s `128 KiB` block size, so it still hit every block of the large JSON fixture and reproduced the old failure mode.

Source reports:
- [main level 1 screen](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-smalltext7-vs8-main.md)
- [broad-local level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-smalltext7-vs8-broad-local.md)

Why it was rejected:
- Main level-1 JSON guardrail failed again:
  - `json_logs_32m.jsonl`: `690,084 -> 809,823`
  - `decodecorpus_pack.bin`: `5,323,478 -> 5,324,210`
- Broad-local direction was still attractive:
  - `dict_dictionary.bin`: `24,237 -> 22,634`
  - `repo_match_generator.rs`: `23,717 -> 22,876`
  - `dict_systemd-logind.service`: `1,175 -> 1,152`
- But the cutoff was simply too high to protect the large streaming JSON path, so that version was rejected and narrowed further.

## 2026-05-30 - Rejected level-1 text classifier minimum `1024 -> 512`

Change notes:
- Tried broadening the level-1 text classification gate so printable blocks start using the text-path matcher heuristics at `512` bytes instead of `1024`.
- This was aimed at mid-sized service/source fixtures without reopening the very small text cases.
- It was rejected because it was effectively neutral on the main guardrails and slightly worse on the broader compression gap.

Source reports:
- [main level 1 screen](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-textclass512-vs1024-main.md)
- [broad-local level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-textclass512-vs1024-broad-local.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-level1-textclass1024-after-textclass512.md)

Why it was rejected:
- Main level-1 guardrails were unchanged, so there was no compensating win:
  - `decodecorpus_pack.bin`: unchanged at `5,323,478`
  - `json_logs_32m.jsonl`: unchanged at `690,084`
- Broad-local movement was too small and net-negative:
  - improvements:
    - `dict_NetworkManager-dispatcher.service`: `400 -> 398`
    - `dict_kmod-static-nodes.service`: `499 -> 496`
  - regressions:
    - `dict_e2scrub_reap.service`: `383 -> 386`
    - `dict_systemd-udev-settle.service`: `572 -> 576`
- The total bytes above C on the fixtures where we still lose moved slightly the wrong way:
  - `6,578 -> 6,580`
- The branch was restored to the retained text-classifier-1024 baseline on top of the retained text-threshold-8 change.

## 2026-05-30 - Rejected level-1 text no-match probe step `3 -> 2`

Change notes:
- Tried a denser text-path search by lowering the text no-match probe step from `3` to `2`.
- This improved a handful of broad-local text fixtures, but it was rejected because it regressed the main level-1 JSON fixture badly and also made decodecorpus CPU worse.

Source reports:
- [main level 1 screen](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-textstep2-vs3-main.md)
- [broad-local level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-textstep2-vs3-broad-local.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-level1-textstep3-after-textstep2.md)

Why it was rejected:
- Main level-1 guardrail regression:
  - `json_logs_32m.jsonl`: `690,084 -> 713,323`
  - `decodecorpus_pack.bin`: `5,323,478 -> 5,323,528`, CPU `0.18s -> 0.20s`
- Broad-local improvements were real but not enough to justify the main regressions:
  - `dict_dictionary.bin`: `24,237 -> 23,871`
  - `repo_match_generator.rs`: `23,717 -> 22,693`
  - `generated_json_logs_001m.jsonl`: `58,767 -> 57,109`
- The branch was restored to the retained text-step-3 baseline.

## 2026-05-30 - Rejected level-1 text non-repeat threshold `8 -> 7`

Change notes:
- Followed the retained `10 -> 8` level-1 text-threshold win with a narrower step from `8` down to `7`.
- This continued to improve several broader-suite text losses, but it was rejected because it broke the main level-1 JSON guardrail and worsened decodecorpus CPU.

Source reports:
- [main level 1 screen](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-textmin7-vs8-main.md)
- [broad-local level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-textmin7-vs8-broad-local.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-level1-textmin8-after-textmin7.md)

Why it was rejected:
- Main level-1 guardrail regression:
  - `json_logs_32m.jsonl`: `690,084 -> 809,823`
  - `decodecorpus_pack.bin`: `5,323,478 -> 5,324,210`, CPU `0.19s -> 0.22s`
- Broad-local compression did improve further on some text fixtures:
  - `dict_dictionary.bin`: `24,237 -> 22,634`
  - `repo_match_generator.rs`: `23,717 -> 22,876`
  - `repo_main.rs`: `2,249 -> 2,211`
- But the main-fixture JSON failure is too large to keep. The branch was restored to the retained threshold-8 baseline.

## 2026-05-30 - Retained level-1 text non-repeat threshold `10 -> 8`

Change notes:
- Lowered the text non-repeat minimum match length in the matcher from `10` bytes to `8` bytes.
- This was targeted at the broader-suite level-1 losses on source-style and dictionary/service-style text after direct in-process profiling showed the hot path was still `MatchGenerator::next_sequence` on those fixtures.
- This is a retained level-1 compression change. It does not materially affect the higher-level binary `Best` path work.

Source reports:
- [main level 1 screen](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-textmin8-main.md)
- [broad-local level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-textmin8-broad-local.md)
- [direct source-text profile](/home/bsutton/git/zstd-rs/benchmarks/reports/perf-level1-repo_match_generator-direct.txt)
- [direct dictionary profile](/home/bsutton/git/zstd-rs/benchmarks/reports/perf-level1-dict_dictionary-direct.txt)

### Main fixture screen

| Fixture | Previous retained baseline | Current retained baseline | C `zstd -1` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `5,324,267` / `0.18s` CPU | `5,323,478` / `0.19s` CPU | `5,385,951` / `0.04s` CPU |
| `json_logs_32m.jsonl` | `690,084` / `0.13s` CPU | `690,084` / `0.13s` CPU | `1,138,701` / `0.04s` CPU |

### Broad-local summary vs C `zstd -1`

- Better/worse/equal counts stayed `13 / 16 / 3`, so this is not full compression equivalence yet.
- But the magnitude of the remaining broader-suite losses improved materially:
  - total bytes above C across the fixtures where we are still worse: `9,279 -> 6,578`
- The only broad-local regression was:
  - `decodecorpus_z000079`: `8,358 -> 8,372` (still worse than C `7,221`)

Largest retained gains from this change:
- `dict_dictionary.bin`: `25,598 -> 24,237`
- `repo_match_generator.rs`: `24,884 -> 23,717`
- `repo_main.rs`: `2,402 -> 2,249`
- `dict_systemd-logind.service`: `1,206 -> 1,175`
- `build_ruzstd-cli`: `870,526 -> 867,739`

Profile note:
- Direct in-process level-1 profiles on the broad-local loss cases still point at the matcher, not entropy coding, as the dominant user-space cost:
  - `repo_match_generator.rs`: `MatchGenerator::next_sequence` `62.61%`
  - `dict_dictionary.bin`: `MatchGenerator::next_sequence` `66.32%`
- Those profiles are checked in above and should guide the next level-1 text-side experiment.

## 2026-05-30 - Broad-local level-1 rerun on the current tree before a CPU-only pivot

Change notes:
- Re-ran the 32-fixture `broad-local` suite at level 1 on the live source tree to test whether the branch is now broadly comparable to C `zstd -1` on compression.
- This was a validation run, not a runtime change. The purpose was to decide whether level-1 work should pivot from compression to CPU.
- Result: no clean pivot yet. On this broader local suite, level 1 is mixed against C:
  - better on 13 fixtures,
  - worse on 16 fixtures,
  - equal on 3 fixtures.
- The branch remains stronger on the larger binary-heavy fixtures and generated JSON, but it is still weaker on several small dictionary/service/source fixtures and the dictionary blob itself.
- The suite is also too small and too fast for useful CPU-vs-C claims: almost every C timing rounds to `0.00s`, so this run answers the compression question more than the CPU question.

Suite sources:
- [broad-local manifest](/home/bsutton/git/zstd-rs/benchmarks/manifests/broad-local.json)
- [suite generator](/home/bsutton/git/zstd-rs/tools/prepare_benchmark_suites.py)

Source reports:
- [broad-local current-tree level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-broad-local-current-l1.md)

### Broad local level-1 summary vs C `zstd -1`

| Category | Count | Notes |
| --- | ---: | --- |
| Better than C | 13 | Mostly larger binary/build artifacts, several `decodecorpus` samples, generated JSON/text/binary fixtures |
| Worse than C | 16 | Mostly small `dict_tests` service files, the dictionary blob, and repo source/text fixtures |
| Equal to C | 3 | A few tiny fixtures where both encoders land on the same size |

### Largest wins vs C `zstd -1`

| Fixture | Current | C `zstd -1` | Delta |
| --- | ---: | ---: | ---: |
| `decodecorpus_z000033` | `544,118` | `571,529` | `-27,411` |
| `build_ruzstd-cli` | `870,526` | `894,099` | `-23,573` |
| `build_libruzstd.rlib` | `619,650` | `635,879` | `-16,229` |
| `decodecorpus_z000028` | `100,347` | `105,226` | `-4,879` |
| `decodecorpus_z000003` | `52,047` | `53,328` | `-1,281` |

### Largest losses vs C `zstd -1`

| Fixture | Current | C `zstd -1` | Delta |
| --- | ---: | ---: | ---: |
| `dict_dictionary.bin` | `25,598` | `20,145` | `+5,453` |
| `repo_match_generator.rs` | `24,884` | `22,797` | `+2,087` |
| `decodecorpus_z000079` | `8,358` | `7,221` | `+1,137` |
| `repo_main.rs` | `2,402` | `2,101` | `+301` |
| `dict_systemd-logind.service` | `1,206` | `1,120` | `+86` |

## 2026-05-30 - Rejected distance-4 distant-oldest prune on top of retained entry-distance-1 newest-first baseline

Change notes:
- Re-tested the distance-4 `oldest` prune on top of the retained entry-distance-1 newest-first baseline.
- This re-test was justified by stronger test-only diagnostics showing no intermediate long-hash-active candidate improvements from `oldest` beyond entry distance `3`, not just no final overrides.
- It was still rejected: bytes regressed again on decodecorpus and level-1 decodecorpus CPU drifted upward.

Source reports:
- [distance-4 re-test level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-entry1-newest-first-plus-distant-oldest4-l4.md)
- [distance-4 re-test level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-entry1-newest-first-plus-distant-oldest4-l1.md)
- [restore level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-entry1-newest-first-1-l4.md)
- [restore level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-entry1-newest-first-1-l1.md)

### Level 4

| Fixture | Retained entry1-newest-first baseline | Distance-4 re-test | C `zstd -4` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `4,675,636` / `0.99s` CPU | `4,675,913` / `0.85s` CPU | `4,789,813` / `0.07s` CPU |
| `json_logs_32m.jsonl` | `602,826` / `0.22s` CPU | `602,826` / `0.23s` CPU | `1,361,274` / `0.07s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.02s` CPU | `2,874` / `0.02s` CPU | `3,128` / `0.04s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.05s` CPU | `33,555,210` / `0.05s` CPU | `33,555,214` / `0.09s` CPU |

### Level 1

| Fixture | Retained entry1-newest-first baseline | Distance-4 re-test | C `zstd -1` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `5,324,267` / `0.19s` CPU | `5,324,267` / `0.20s` CPU | `5,385,951` / `0.05s` CPU |
| `json_logs_32m.jsonl` | `690,084` / `0.13s` CPU | `690,084` / `0.13s` CPU | `1,138,701` / `0.05s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.00s` CPU | `2,874` / `0.00s` CPU | `3,116` / `0.02s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.01s` CPU | `33,555,210` / `0.02s` CPU | `33,555,214` / `0.06s` CPU |

Restore note:
- After reverting the distance-4 re-test and rebuilding the CLI, the live source tree again matched the retained entry-distance-1 newest-first baseline at both levels.

## 2026-05-30 - Broader local suite validation for retained entry-distance-1 newest-first baseline

Change notes:
- Added `tools/prepare_benchmark_suites.py` and generated `benchmarks/fixtures/broad-local`, a 32-fixture broader local suite covering:
  - generated text/binary data
  - selected raw `decodecorpus_files` samples
  - selected `dict_tests/files/*.service` samples plus the dictionary blob
  - representative repo source files
  - release build artifacts
- This suite validates the retained entry-distance-1 newest-first change against the previous retained distant-newest baseline on a wider corpus than the tiny inner-loop screen.

Suite sources:
- [broad-local manifest](/home/bsutton/git/zstd-rs/benchmarks/manifests/broad-local.json)
- [suite generator](/home/bsutton/git/zstd-rs/tools/prepare_benchmark_suites.py)

Source reports:
- [broad-local level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-broad-local-entry1-newest-first-l4.md)
- [broad-local level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-broad-local-entry1-newest-first-l1.md)

### Broad local summary

- 32 fixtures total
- 0 byte deltas at level 4
- 0 byte deltas at level 1
- level-4 CPU improved on 4 fixtures and regressed on none
- level-1 CPU was flat across the suite at this resolution

Notable level-4 CPU wins:
- `build_ruzstd-cli`: `0.23s -> 0.20s`
- `build_libruzstd.rlib`: `0.12s -> 0.11s`
- `decodecorpus_z000033`: `0.13s -> 0.12s`
- `generated_json_logs_001m.jsonl`: `0.02s -> 0.01s`

## 2026-05-30 - Retained entry-distance-1 newest-first override after current long-hash hit

Change notes:
- On the binary `Best` path, when a current-entry long-hash candidate is already active and the older-entry probe is at entry distance `1`, `newest` is now probed before `oldest`.
- This is narrower than the earlier rejected distance and prefix gates: it only changes the candidate order in the one subspace where current diagnostics show `newest@1` overriding the long-hash slightly more often than `oldest@1`.
- Bytes stayed exact against the retained distant-newest baseline, while the focused repeat run improved CPU on the main fixtures.

Source reports:
- [entry-distance-1 newest-first level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-entry1-newest-first.csv)
- [entry-distance-1 newest-first level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-entry1-newest-first-l1.csv)
- [entry-distance-1 newest-first repeat level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-entry1-newest-first-repeat-l4.csv)
- [entry-distance-1 newest-first repeat level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-entry1-newest-first-repeat-l1.csv)

### Level 4

| Fixture | Previous retained baseline | Current retained baseline | C `zstd -4` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `4,675,636` / `0.91s` CPU | `4,675,636` / `0.88s` CPU | `4,789,813` / `0.07s` CPU |
| `json_logs_32m.jsonl` | `602,826` / `0.21s` CPU | `602,826` / `0.21s` CPU | `1,361,274` / `0.07s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.02s` CPU | `2,874` / `0.03s` CPU | `3,128` / `0.04s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.06s` CPU | `33,555,210` / `0.05s` CPU | `33,555,214` / `0.11s` CPU |

Repeat-run note:
- `decodecorpus_pack.bin` stayed byte-identical at `4,675,636`, with CPU improving from `0.91s` to `0.84s`.
- `json_logs_32m.jsonl` stayed byte-identical at `602,826`, with repeat CPU flat at `0.22s`.

### Level 1

| Fixture | Previous retained baseline | Current retained baseline | C `zstd -1` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `5,324,267` / `0.20s` CPU | `5,324,267` / `0.18s` CPU | `5,385,951` / `0.04s` CPU |
| `json_logs_32m.jsonl` | `690,084` / `0.13s` CPU | `690,084` / `0.13s` CPU | `1,138,701` / `0.05s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.00s` CPU | `2,874` / `0.00s` CPU | `3,116` / `0.01s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.02s` CPU | `33,555,210` / `0.01s` CPU | `33,555,214` / `0.05s` CPU |

Repeat-run note:
- `decodecorpus_pack.bin` stayed byte-identical at `5,324,267`, with CPU improving from `0.19s` to `0.18s`.
- `json_logs_32m.jsonl` stayed byte-identical at `690,084`, with repeat CPU improving from `0.13s` to `0.12s`.

## 2026-05-30 - Rejected distance-4 distant-oldest prune after current long-hash hit

Change notes:
- On the binary `Best` path, after a current-entry long-hash hit, older-entry `oldest` candidates were skipped only at entry distances `>= 4`.
- This was the narrowest distance cut tested in this family, chosen because the current diagnostics showed no long-hash overrides from `oldest` beyond distance `3`.
- It was rejected on the first full-table pass because decodecorpus bytes regressed and level-1 decodecorpus CPU drifted upward.

Source reports:
- [distance-4 distant-oldest level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-distant-oldest-prune4.csv)
- [distance-4 distant-oldest level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-distant-oldest-prune4-l1.csv)
- [restore level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-distant-newest-prune-6.md)
- [restore level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-distant-newest-prune-6-l1.md)

### Level 4

| Fixture | Retained distant-newest baseline | Distance-4 experiment | C `zstd -4` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `4,675,636` / `0.89s` CPU | `4,675,913` / `0.89s` CPU | `4,789,813` / `0.06s` CPU |
| `json_logs_32m.jsonl` | `602,826` / `0.21s` CPU | `602,826` / `0.22s` CPU | `1,361,274` / `0.07s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.03s` CPU | `2,874` / `0.03s` CPU | `3,128` / `0.04s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.05s` CPU | `33,555,210` / `0.05s` CPU | `33,555,214` / `0.08s` CPU |

### Level 1

| Fixture | Retained distant-newest baseline | Distance-4 experiment | C `zstd -1` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `5,324,267` / `0.19s` CPU | `5,324,267` / `0.20s` CPU | `5,385,951` / `0.05s` CPU |
| `json_logs_32m.jsonl` | `690,084` / `0.13s` CPU | `690,084` / `0.13s` CPU | `1,138,701` / `0.04s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.00s` CPU | `2,874` / `0.00s` CPU | `3,116` / `0.01s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.02s` CPU | `33,555,210` / `0.02s` CPU | `33,555,214` / `0.05s` CPU |

Restore note:
- After reverting the distance-4 cut and rebuilding the CLI, the live source tree again matched the retained distant-newest baseline at both levels.

## 2026-05-30 - Rejected newest@1-gated distant-oldest prune after current long-hash hit

Change notes:
- On the binary `Best` path, after a current-entry long-hash hit, older-entry `oldest` candidates at entry distances `>= 3` were skipped only after `newest` at entry distance `1` had already improved the candidate.
- This was a more selective probe-shape attempt than the earlier blanket distant-`oldest` cuts because it only weakened distant `oldest` competition after the strongest nearby `newest` override had already happened.
- It was rejected on the first full-table pass because decodecorpus bytes regressed and level-1 decodecorpus CPU drifted upward.

Source reports:
- [newest@1-gated distant-oldest level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-newest1-gates-oldest.csv)
- [newest@1-gated distant-oldest level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-newest1-gates-oldest-l1.csv)
- [restore level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-distant-newest-prune-5.csv)
- [restore level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-distant-newest-prune-5-l1.csv)

### Level 4

| Fixture | Retained distant-newest baseline | Newest@1-gated experiment | C `zstd -4` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `4,675,636` / `0.91s` CPU | `4,675,688` / `0.90s` CPU | `4,789,813` / `0.07s` CPU |
| `json_logs_32m.jsonl` | `602,826` / `0.23s` CPU | `602,826` / `0.23s` CPU | `1,361,274` / `0.07s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.02s` CPU | `2,874` / `0.02s` CPU | `3,128` / `0.04s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.05s` CPU | `33,555,210` / `0.05s` CPU | `33,555,214` / `0.10s` CPU |

### Level 1

| Fixture | Retained distant-newest baseline | Newest@1-gated experiment | C `zstd -1` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `5,324,267` / `0.20s` CPU | `5,324,267` / `0.21s` CPU | `5,385,951` / `0.04s` CPU |
| `json_logs_32m.jsonl` | `690,084` / `0.14s` CPU | `690,084` / `0.13s` CPU | `1,138,701` / `0.05s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.00s` CPU | `2,874` / `0.00s` CPU | `3,116` / `0.01s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.01s` CPU | `33,555,210` / `0.02s` CPU | `33,555,214` / `0.07s` CPU |

Restore note:
- After reverting the experiment and rebuilding the CLI, the live source tree again matched the retained distant-newest baseline at both levels.

## 2026-05-30 - Rejected distant-oldest prefix-6 gate after current long-hash hit

Change notes:
- On the binary `Best` path, after a current-entry long-hash candidate exists, older-entry `oldest` candidates at entry distances `>= 3` were required to clear a 6-byte prefix before paying full match expansion.
- This was tested as a more selective follow-up to the rejected blanket distance-3 `oldest` cuts.
- It was rejected after the repeat run failed the decodecorpus CPU guardrail, and the source tree was restored to the retained distant-newest baseline.

Source reports:
- [distant-oldest prefix-6 level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-distant-oldest-prefix6.csv)
- [distant-oldest prefix-6 level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-distant-oldest-prefix6-l1.csv)
- [distant-oldest prefix-6 repeat level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-distant-oldest-prefix6-repeat-l4.csv)
- [restore level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-distant-newest-prune-4.csv)
- [restore level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-distant-newest-prune-4-l1.csv)

### Level 4

| Fixture | Retained distant-newest baseline | Prefix-6 experiment | C `zstd -4` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `4,675,636` / `0.89s` CPU | `4,675,636` / `0.84s` CPU | `4,789,813` / `0.06s` CPU |
| `json_logs_32m.jsonl` | `602,826` / `0.21s` CPU | `602,826` / `0.21s` CPU | `1,361,274` / `0.08s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.02s` CPU | `2,874` / `0.03s` CPU | `3,128` / `0.03s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.05s` CPU | `33,555,210` / `0.05s` CPU | `33,555,214` / `0.09s` CPU |

Repeat-run note:
- `decodecorpus_pack.bin` stayed byte-identical at `4,675,636`, but CPU regressed from `0.86s` to `0.96s`.
- `json_logs_32m.jsonl` stayed byte-identical at `602,826`, but CPU drifted from `0.25s` to `0.26s`.
- That repeat result is why this experiment was rejected despite the better first-pass decodecorpus timing.

### Level 1

| Fixture | Retained distant-newest baseline | Prefix-6 experiment | C `zstd -1` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `5,324,267` / `0.19s` CPU | `5,324,267` / `0.19s` CPU | `5,385,951` / `0.04s` CPU |
| `json_logs_32m.jsonl` | `690,084` / `0.13s` CPU | `690,084` / `0.12s` CPU | `1,138,701` / `0.04s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.00s` CPU | `2,874` / `0.00s` CPU | `3,116` / `0.01s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.01s` CPU | `33,555,210` / `0.02s` CPU | `33,555,214` / `0.05s` CPU |

Restore note:
- After reverting the prefix-6 gate and rebuilding the CLI, the live source tree matched the retained distant-newest baseline again at both levels.

## 2026-05-30 - Retained distant-newest prune baseline

Change notes:
- On the binary `Best` path, once a current-entry long-hash candidate exists, older-entry `newest` candidates are only probed for entry distance `1`.
- For entry distances `>= 2`, older-entry `newest` candidates are skipped; `oldest` and the existing retained long-hash path still participate.
- This replaces the earlier `skipolder56` point as the current retained baseline.

Source reports:
- [distant-newest prune level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-distant-newest-prune.csv)
- [distant-newest prune level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-distant-newest-prune-l1.csv)
- [distant-newest prune repeat level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-distant-newest-prune-repeat-l4.csv)
- [distant-newest prune repeat level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-distant-newest-prune-repeat-l1.csv)

### Level 4

| Fixture | Previous retained baseline | Current retained baseline | C `zstd -4` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `4,675,782` / `0.84s` CPU | `4,675,636` / `0.84s` CPU | `4,789,813` / `0.07s` CPU |
| `json_logs_32m.jsonl` | `602,826` / `0.22s` CPU | `602,826` / `0.21s` CPU | `1,361,274` / `0.08s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.02s` CPU | `2,874` / `0.03s` CPU | `3,128` / `0.04s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.05s` CPU | `33,555,210` / `0.05s` CPU | `33,555,214` / `0.08s` CPU |

Repeat-run note:
- `decodecorpus_pack.bin` level-4 repeat run improved from `0.84s` to `0.82s` while moving bytes from `4,675,782` to `4,675,636`.
- `json_logs_32m.jsonl` stayed byte-identical at `602,826`, with repeat CPU flat at `0.21s`.

### Level 1

| Fixture | Previous retained baseline | Current retained baseline | C `zstd -1` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `5,324,267` / `0.19s` CPU | `5,324,267` / `0.19s` CPU | `5,385,951` / `0.04s` CPU |
| `json_logs_32m.jsonl` | `690,084` / `0.13s` CPU | `690,084` / `0.13s` CPU | `1,138,701` / `0.04s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.00s` CPU | `2,874` / `0.00s` CPU | `3,116` / `0.01s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.02s` CPU | `33,555,210` / `0.02s` CPU | `33,555,214` / `0.05s` CPU |

## 2026-05-30 - Retained `skipolder56` baseline

Change notes:
- `BEST_CURRENT_LONG_HASH_SKIP_OLDER_LEN` changed from `48` to `56`.
- On the binary `Best` path, once the current-entry 8-byte long-hash candidate reaches `56` bytes, older-entry competition is skipped entirely.
- This replaces the earlier `skipolder48` point as the current retained baseline.

Source reports:
- [skipolder56 level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-skipolder56.csv)
- [skipolder56 level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-skipolder56-l1.csv)
- [skipolder56 repeat level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-skipolder56-repeat-l4.csv)
- [skipolder56 repeat level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-skipolder56-repeat-l1.csv)

### Level 4

| Fixture | Previous retained baseline | Current retained baseline | C `zstd -4` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `4,675,797` / `0.85s` CPU | `4,675,782` / `0.85s` CPU | `4,789,813` / `0.07s` CPU |
| `json_logs_32m.jsonl` | `602,826` / `0.22s` CPU | `602,826` / `0.22s` CPU | `1,361,274` / `0.08s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.02s` CPU | `2,874` / `0.02s` CPU | `3,128` / `0.04s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.05s` CPU | `33,555,210` / `0.05s` CPU | `33,555,214` / `0.08s` CPU |

Repeat-run note:
- `decodecorpus_pack.bin` level-4 repeat run improved from `0.91s` to `0.83s` while keeping the `4,675,782` output.
- `json_logs_32m.jsonl` stayed byte-identical at `602,826`, with repeat CPU flat at `0.21s`.

### Level 1

| Fixture | Previous retained baseline | Current retained baseline | C `zstd -1` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `5,324,267` / `0.19s` CPU | `5,324,267` / `0.19s` CPU | `5,385,951` / `0.04s` CPU |
| `json_logs_32m.jsonl` | `690,084` / `0.13s` CPU | `690,084` / `0.13s` CPU | `1,138,701` / `0.05s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.00s` CPU | `2,874` / `0.00s` CPU | `3,116` / `0.02s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.02s` CPU | `33,555,210` / `0.02s` CPU | `33,555,214` / `0.06s` CPU |

## 2026-05-30 - Retained `skipolder48` baseline

Change notes:
- `BEST_CURRENT_LONG_HASH_SKIP_OLDER_LEN` changed from `40` to `48`.
- On the binary `Best` path, once the current-entry 8-byte long-hash candidate reaches `48` bytes, older-entry competition is skipped entirely.
- This replaces the earlier `skipolder40` point as the current retained baseline.

Source reports:
- [skipolder48 level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-skipolder48.csv)
- [skipolder48 level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-skipolder48-l1.csv)
- [skipolder48 repeat level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-skipolder48-repeat-l4.csv)
- [skipolder48 repeat level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-skipolder48-repeat-l1.csv)

### Level 4

| Fixture | Previous retained baseline | Current retained baseline | C `zstd -4` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `4,675,820` / `0.87s` CPU | `4,675,797` / `0.87s` CPU | `4,789,813` / `0.07s` CPU |
| `json_logs_32m.jsonl` | `602,826` / `0.23s` CPU | `602,826` / `0.23s` CPU | `1,361,274` / `0.08s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.02s` CPU | `2,874` / `0.02s` CPU | `3,128` / `0.04s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.06s` CPU | `33,555,210` / `0.06s` CPU | `33,555,214` / `0.10s` CPU |

Repeat-run note:
- `decodecorpus_pack.bin` level-4 repeat run improved from `0.90s` to `0.84s` while keeping the `4,675,797` output.
- `json_logs_32m.jsonl` stayed byte-identical at `602,826`, with repeat CPU flat at `0.22s`.

### Level 1

| Fixture | Previous retained baseline | Current retained baseline | C `zstd -1` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `5,324,267` / `0.19s` CPU | `5,324,267` / `0.19s` CPU | `5,385,951` / `0.04s` CPU |
| `json_logs_32m.jsonl` | `690,084` / `0.13s` CPU | `690,084` / `0.13s` CPU | `1,138,701` / `0.05s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.00s` CPU | `2,874` / `0.00s` CPU | `3,116` / `0.01s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.02s` CPU | `33,555,210` / `0.02s` CPU | `33,555,214` / `0.05s` CPU |

## 2026-05-30 - Retained `skipolder40` baseline

Change notes:
- `BEST_CURRENT_LONG_HASH_SKIP_OLDER_LEN` changed from `32` to `40`.
- On the binary `Best` path, once the current-entry 8-byte long-hash candidate reaches `40` bytes, older-entry competition is skipped entirely.
- This replaces the earlier `skipolder32` point as the current retained baseline.

Source reports:
- [skipolder40 level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-skipolder40.csv)
- [skipolder40 level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-skipolder40-l1.csv)
- [skipolder40 repeat level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-skipolder40-repeat-l4.csv)
- [skipolder40 repeat level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-skipolder40-repeat-l1.csv)

### Level 4

| Fixture | Previous retained baseline | Current retained baseline | C `zstd -4` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `4,675,858` / `0.83s` CPU | `4,675,820` / `1.07s` CPU | `4,789,813` / `0.08s` CPU |
| `json_logs_32m.jsonl` | `602,826` / `0.24s` CPU | `602,826` / `0.24s` CPU | `1,361,274` / `0.08s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.02s` CPU | `2,874` / `0.03s` CPU | `3,128` / `0.04s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.06s` CPU | `33,555,210` / `0.06s` CPU | `33,555,214` / `0.09s` CPU |

Repeat-run note:
- `decodecorpus_pack.bin` level-4 repeat run improved from `0.89s` to `0.84s` while keeping the `4,675,820` output.
- `json_logs_32m.jsonl` stayed byte-identical at `602,826`; repeat CPU drifted slightly from `0.21s` to `0.22s`.

### Level 1

| Fixture | Previous retained baseline | Current retained baseline | C `zstd -1` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `5,324,267` / `0.18s` CPU | `5,324,267` / `0.19s` CPU | `5,385,951` / `0.05s` CPU |
| `json_logs_32m.jsonl` | `690,084` / `0.12s` CPU | `690,084` / `0.13s` CPU | `1,138,701` / `0.06s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.00s` CPU | `2,874` / `0.00s` CPU | `3,116` / `0.02s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.02s` CPU | `33,555,210` / `0.02s` CPU | `33,555,214` / `0.05s` CPU |

## 2026-05-30 - Retained `skipolder32` baseline

Change notes:
- `BEST_CURRENT_LONG_HASH_SKIP_OLDER_LEN` changed from `24` to `32`.
- On the binary `Best` path, once the current-entry 8-byte long-hash candidate reaches `32` bytes, older-entry competition is skipped entirely.
- This was the previous retained baseline before the later `40`-byte threshold retune.

Source reports:
- [skipolder32 level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-skipolder32.csv)
- [skipolder32 level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-skipolder32-l1.csv)
- [skipolder32 repeat](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-skipolder32-repeat.csv)

### Level 4

| Fixture | Previous retained baseline | Current retained baseline | C `zstd -4` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `4,675,942` / `0.81s` CPU | `4,675,858` / `0.83s` CPU | `4,789,813` / `0.07s` CPU |
| `json_logs_32m.jsonl` | `602,826` / `0.21s` CPU | `602,826` / `0.21s` CPU | `1,361,274` / `0.09s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.02s` CPU | `2,874` / `0.02s` CPU | `3,128` / `0.04s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.07s` CPU | `33,555,210` / `0.06s` CPU | `33,555,214` / `0.08s` CPU |

Repeat-run note:
- `decodecorpus_pack.bin` level-4 CPU stayed in the `0.84s/0.85s` median band.
- `json_logs_32m.jsonl` level-4 CPU stayed in the `0.21s/0.22s` median band.

### Level 1

| Fixture | Previous retained baseline | Current retained baseline | C `zstd -1` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `5,324,267` / `0.18s` CPU | `5,324,267` / `0.18s` CPU | `5,385,951` / `0.05s` CPU |
| `json_logs_32m.jsonl` | `690,084` / `0.13s` CPU | `690,084` / `0.12s` CPU | `1,138,701` / `0.05s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.00s` CPU | `2,874` / `0.00s` CPU | `3,116` / `0.02s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.01s` CPU | `33,555,210` / `0.02s` CPU | `33,555,214` / `0.06s` CPU |

## 2026-05-30 - Retained `skipolder24` baseline

Change notes:
- `BEST_CURRENT_LONG_HASH_SKIP_OLDER_LEN` changed from `16` to `24`.
- This was the previous retained binary `Best` baseline before the later `32`-byte threshold retune.

Source reports:
- [skipolder24 vs16 level 4](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-skipolder24-vs16.md)
- [skipolder24 vs16 level 1](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-current-longhash-skipolder24-vs16-l1.md)

### Level 4

| Fixture | Previous retained baseline | Current retained baseline | C `zstd -4` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `4,676,147` / `0.87s` CPU | `4,675,942` / `0.83s` CPU | `4,789,813` / `0.07s` CPU |
| `json_logs_32m.jsonl` | `602,826` / `0.21s` CPU | `602,826` / `0.22s` CPU | `1,361,274` / `0.08s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.02s` CPU | `2,874` / `0.03s` CPU | `3,128` / `0.04s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.06s` CPU | `33,555,210` / `0.06s` CPU | `33,555,214` / `0.09s` CPU |

### Level 1

| Fixture | Previous retained baseline | Current retained baseline | C `zstd -1` |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | `5,324,267` / `0.19s` CPU | `5,324,267` / `0.18s` CPU | `5,385,951` / `0.04s` CPU |
| `json_logs_32m.jsonl` | `690,084` / `0.13s` CPU | `690,084` / `0.13s` CPU | `1,138,701` / `0.05s` CPU |
| `repeated_text_32m.txt` | `2,874` / `0.00s` CPU | `2,874` / `0.00s` CPU | `3,116` / `0.01s` CPU |
| `xorshift_32m.bin` | `33,555,210` / `0.02s` CPU | `33,555,210` / `0.02s` CPU | `33,555,214` / `0.06s` CPU |

## 2026-05-31 - Rejected `CodeText` exact Huffman search for all literal sections

Change notes:
- Tried promoting `CompressionFileType::CodeText` from the retained small-literal exact-Huffman search to exact search for all literal sections at level 1.
- Motivation:
  - fresh archive inspection on `repo_compressed.rs` showed a literal-section gap versus C and made a literal-side follow-up look plausible.

Source report:
- [code-allsections broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-code-allsections-broad-local.md)

Result:
- exact byte-for-byte no-op on the expanded broad-local suite
- `repo_compressed.rs`: stayed `12,839`
- `repo_progress.rs`: stayed `3,147`
- `repo_benchmark_zstd.py`: stayed `2,846`

Conclusion:
- `repo_compressed.rs` was not blocked on literal Huffman table search
- this `CodeText` literal-side expansion is closed in this form

## 2026-05-31 - Retained `CodeText` dense short-line probing up to 64 KiB

Change notes:
- Widened the retained short-line `CodeText` dense-probe cutoff:
  - `CodeText`: `10 KiB -> 64 KiB`
  - `ConfigText`: unchanged at `8 KiB`
- Added matcher coverage:
  - `large_code_text_blocks_keep_dense_probe_step`

Source reports:
- [codeprobe64k broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-codeprobe64k-broad-local.md)
- [current broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [current fast](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-fast.md)

Current broad-local summary vs C `zstd -1`:
- better / worse / equal: `27 / 11 / 3`
- bytes-above-C on losing fixtures: `185`

Main wins versus the previous retained `codeprobe10k` baseline:
- `repo_compressed.rs`: `12,839 -> 12,695`
- `repo_match_generator.rs`: `26,253 -> 26,192`

Unchanged sentinels:
- `decodecorpus_z000079`: `7,321`
- `dict_dictionary.bin`: `20,160`
- `repo_progress.rs`: `3,147`
- `repo_benchmark_zstd.py`: `2,846`

Largest remaining losses on the expanded suite:
- `decodecorpus_z000079`: `+100`
- `repo_progress.rs`: `+23`
- `dict_dictionary.bin`: `+15`
- `decodecorpus_z000059`: `+13`
- `repo_Cargo.toml`: `+11`

## 2026-05-31 - Retained lower non-repeat floor for small short-line `CodeText`

Change notes:
- For short-line `CodeText` blocks up to `16 KiB`, lowered the non-repeat match floor from `6` to `5`.
- Larger `CodeText` blocks keep the retained `6`-byte floor.
- Added matcher tests for both sides of the split.

Source reports:
- [code-smallfloor5 broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-code-smallfloor5-broad-local.md)
- [current broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [current fast](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-fast.md)

Current broad-local summary vs C `zstd -1`:
- better / worse / equal: `28 / 10 / 3`
- bytes-above-C on losing fixtures: `162`

Archive clue that motivated it:
- `repo_progress.rs`
  - Rust:
    - `literal_section_bytes=1946`
    - `sequence_payload_bytes=1181`
    - `decoded_literals=2874`
    - `sequences=488`
  - C:
    - `literal_section_bytes=1686`
    - `sequence_payload_bytes=1417`
    - `decoded_literals=2440`
    - `sequences=618`

Main wins versus the retained `codeprobe64k` baseline:
- `repo_progress.rs`: `3,147 -> 3,125`
- `repo_benchmark_zstd.py`: `2,846 -> 2,814`
- `repo_main.rs`: `2,128 -> 2,125`

Unchanged sentinels:
- `repo_compressed.rs`: `12,695`
- `repo_match_generator.rs`: `26,192`
- `decodecorpus_z000079`: `7,321`
- `dict_dictionary.bin`: `20,160`

Largest remaining losses on the expanded suite:
- `decodecorpus_z000079`: `+100`
- `dict_dictionary.bin`: `+15`
- `decodecorpus_z000059`: `+13`
- `repo_Cargo.toml`: `+11`
- `repo_.gitignore`: `+8`

## 2026-05-31 - Rejected two small-`ConfigText` matcher-side follow-ups

Change notes:
- Tried a same-start smaller-offset preference for small `ConfigText` blocks:
  - up to `16 KiB`
  - same-start only
  - save at least 2 offset-code bits
  - lose at most 1 match byte
- Also tried enabling the text repeat pipeline for small `ConfigText` blocks up to `16 KiB`.

Source reports:
- [config-offsetaware broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-config-offsetaware-broad-local.md)
- [config-textrepeat broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-config-textrepeat-broad-local.md)

Results:
- both were byte no-ops on the expanded broad-local suite for the target fixtures:
  - `repo_Cargo.toml`: stayed `737`
  - `repo_.gitignore`: stayed `172`
- the text repeat variant also drifted fast-screen CPU on already-winning binaries:
  - `build_ruzstd-cli`: `0.06s -> 0.07s`

Conclusion:
- the remaining small `ConfigText` losses were not waiting on another matcher-side rule

## 2026-05-31 - Retained single-stream Huffman for small `ConfigText` literal sections

Change notes:
- For `CompressionFileType::ConfigText` at level 1, compressed literal sections up to `1024` literals may force the single-stream Huffman path.
- Added focused block-encoder tests for:
  - the `ConfigText` config flag
  - the forced single-stream size-format decision

Source reports:
- [config-singlestream broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-config-singlestream-broad-local.md)
- [current broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [current fast](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-fast.md)

Current broad-local summary vs C `zstd -1`:
- better / worse / equal: `30 / 9 / 2`
- bytes-above-C on losing fixtures: `154`

Main wins versus the retained `code-smallfloor5` baseline:
- `repo_Cargo.toml`: `737 -> 730`
- `dict_canberra-system-bootup.service`: `316 -> 307`
- `dict_git-daemon@.service`: `248 -> 241`
- `dict_glustereventsd.service`: `292 -> 285`
- `dict_quotaon.service`: `420 -> 412`
- `dict_systemd-journal-gatewayd.service`: `630 -> 622`
- `dict_systemd-rfkill.service`: `469 -> 462`
- `dict_virtlockd.service`: `450 -> 442`
- `dict_virtlogd.service`: `535 -> 527`
- `dict_virtqemud.service`: `672 -> 664`
- `dict_virtsecretd.service`: `316 -> 308`

Unchanged sentinels:
- `decodecorpus_z000079`: `7,321`
- `dict_dictionary.bin`: `20,160`
- `repo_.gitignore`: `172`
- `repo_progress.rs`: `3,125`

Largest remaining losses on the expanded suite:
- `decodecorpus_z000079`: `+100`
- `dict_dictionary.bin`: `+15`
- `decodecorpus_z000059`: `+13`
- `repo_.gitignore`: `+8`
- `dict_talk.service`: `+6`
- `decodecorpus_z000053`: `+5`
- `repo_Cargo.toml`: `+4`

## 2026-05-31 - Rejected forced-text matching for tiny `ConfigText` blocks

Change notes:
- Tried treating small `ConfigText` blocks up to `1024` bytes as text for matching.
- Motivation:
  - `repo_.gitignore` and `dict_talk.service` sit below the generic `likely_text()` cutoff, so the retained text-side matcher path was not active for them.

Source report:
- [config-forcedtext broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-config-forcedtext-broad-local.md)

Result:
- no gain on the target fixtures:
  - `repo_.gitignore`: stayed `172`
  - `dict_talk.service`: stayed `160`
  - `repo_Cargo.toml`: stayed `730`
- one already-winning config fixture regressed by 1 byte:
  - `dict_glustereventsd.service`: `285 -> 286`

Conclusion:
- the remaining tiny `ConfigText` losses were not waiting on the text matcher path

## 2026-05-31 - Rejected broader `DictionaryText` smaller-offset preference

Change notes:
- Broadened the retained dictionary smaller-offset rule so a smaller-offset non-repeat candidate could win even without the same-start condition.
- Gate:
  - save at least 4 offset-code bits
  - lose at most 1 match byte

Source report:
- [dict-broader-offset broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dict-broader-offset-broad-local.md)

Result:
- `dict_dictionary.bin`: `20,160 -> 20,161`

Conclusion:
- the retained dictionary same-start smaller-offset rule is already at the useful edge

## 2026-06-01 - Rejected dedicated `LockfileText` family for `Cargo.lock`

Change notes:
- Tried introducing a new public `CompressionFileType::LockfileText` family.
- Intended starting point:
  - dense text probing like `DictionaryText`
  - exact Huffman table search for all sections
  - `offset_table_max_log = 7`
  - stronger short-line non-repeat floor
- Motivation:
  - after fixing duplicate fixture-name collisions in `broad-local`, `repo_Cargo.lock` became the largest known-file-type loss.

Source reports:
- [LockfileText A/B broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfiletext-broad-local.md)
- [restore broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-level1-lockfiletext-broad-local.md)
- [current broad-local baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)

Result:
- it made the target worse, not better:
  - `repo_Cargo.lock`: `9,240 -> 9,288`
- corrected broad-local total bytes-above-C on losing fixtures regressed:
  - `1,411 -> 1,459`

Restore check:
- rebuilt current CLI matches the retained baseline binary exactly on corrected `broad-local`
- restored current summary vs C `zstd -1`:
  - `31 / 12 / 4` better / worse / equal
  - `1,411` total bytes above C on losing fixtures

Conclusion:
- do not keep a separate public `LockfileText` family in this form
- current retained policy stays:
  - suffix-based named-file matching
  - `Cargo.lock -> DictionaryText`

## 2026-06-01 - Rejected two narrower lockfile-like `DictionaryText` matcher cuts

Change notes:
- After rejecting the public `LockfileText` family, tried two content-detected matcher cuts inside the retained `DictionaryText` path for Cargo-lock-like text:
  1. raise the short-line non-repeat floor from `5` to `7`
  2. keep the retained floor but stop forcing fully dense probe step `1`

Source reports:
- [lockfile floor 7 broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargolockfloor7-broad-local.md)
- [lockfile probe step 2 broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargolockstep2-broad-local.md)

Result:
- both changes only moved the target, and both moved it the wrong way:
  - floor `7`: `repo_Cargo.lock 9,240 -> 9,288`
  - probe step `2`: `repo_Cargo.lock 9,240 -> 9,255`

Conclusion:
- the obvious “make lockfile text less eager” matcher cuts are closed
- `Cargo.lock` does not want either a higher floor or a less-dense probe step in the current representation

## 2026-06-01 - Rejected lockfile-like `DictionaryText` smaller-offset rule with 2-byte match loss

Change notes:
- Tried one more lockfile-only matcher rule inside the retained `Cargo.lock -> DictionaryText` path:
  - same-start smaller-offset non-repeat candidates can win with up to `2` bytes of match loss
- Motivation:
  - `repo_Cargo.lock` still pays a large offset-side gap versus C
  - the retained generic dictionary smaller-offset rule only allows `1` byte of match loss

Source report:
- [lockfile offset-aware broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargolockoffset2-broad-local.md)

Result:
- target still regressed:
  - `repo_Cargo.lock`: `9,240 -> 9,243`
- corrected broad-local bytes-above-C on losers also moved the wrong way:
  - `1,307 -> 1,310`

Conclusion:
- the lockfile-specific smaller-offset family is closed in the current matcher representation

## 2026-06-01 - Rejected level-1 `DictionaryText` text-repeat pipeline

Change notes:
- Tried enabling the existing best-text repeat pipeline for `DictionaryText` text blocks at level 1.
- Motivation:
  - the obvious lockfile-specific local matcher cuts were already closed
  - this was the only remaining text-side parser shape in the current matcher that could still change `Cargo.lock`

Source report:
- [DictionaryText text-repeat broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dicttextrepeat-broad-local.md)

Result:
- exact byte-for-byte no-op on corrected `broad-local`
- key sentinels stayed exact:
  - `repo_Cargo.lock`: `9,240`
  - `dict_dictionary.bin`: `20,160`

Conclusion:
- the current text-repeat pipeline is not a missing piece for the dictionary/lockfile family at level 1

## 2026-06-01 - Rejected known-size single-segment frame headers for CLI file compression

Change notes:
- Tried threading the known input file size through CLI compression so the encoder could emit:
  - `frame_content_size`
  - `single_segment = true`
- Motivation:
  - current archive inspections still showed a header-shape difference versus C on file inputs

Source reports:
- [known-size header broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-knownsize-header-broad-local.md)
- [restore codeprobe96k broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-level1-codeprobe96k-broad-local.md)

Result:
- broadly made outputs larger by 1 to 3 bytes
- examples:
  - `repo_Cargo.lock`: `9,240 -> 9,241`
  - `repo_compressed.rs`: `12,946 -> 12,949`
  - `decodecorpus_z000079`: `7,321 -> 7,324`
- corrected broad-local bytes-above-C on losers moved the wrong way:
  - `1,307 -> 1,318`

Conclusion:
- keep the existing streaming-style frame header on the retained baseline
- the C-like known-size/single-segment header shape is not compression-positive here

## 2026-06-01 - Rejected small-sequence `DictionaryText` OF max-log `6`

Change notes:
- Tried lowering `offset_table_max_log` from `7` to `6` only for small-sequence `DictionaryText` blocks.
- Gate:
  - `CompressionLevel::Fastest`
  - `CompressionFileType::DictionaryText`
  - `sequence_count <= 1024`
- Motivation:
  - `repo_Cargo.lock` has `836` sequences and a large offset-side gap versus C
  - `dict_dictionary.bin` has over `4,000` sequences, so this gate avoided the large dictionary case entirely

Source reports:
- [small-sequence DictionaryText OF max-log 6 broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dictsmalloflog6-broad-local.md)
- [restore codeprobe96k broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-level1-codeprobe96k-broad-local.md)

Result:
- target regressed hard:
  - `repo_Cargo.lock`: `9,240 -> 9,292`
- corrected broad-local bytes-above-C on losers regressed:
  - `1,307 -> 1,359`

Conclusion:
- small-sequence OF-table tightening is the wrong direction for the `Cargo.lock` family

## 2026-06-01 - Retained `CodeText` dense probing up to 96 KiB

Change notes:
- widened the `CodeText` short-line dense probe cutoff in `match_generator.rs`:
  - `64 KiB -> 96 KiB`
- motivation:
  - fresh archive inspection for `repo_compressed.rs` showed it was slightly under-sequenced versus C
  - the fixture is just above the old `64 KiB` cutoff

Source reports:
- [A/B broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-codeprobe96k-broad-local.md)
- [current broad-local baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)

Result:
- `repo_compressed.rs`: `13,111 -> 12,946`
- C `zstd -1`: `13,007`
- corrected broad-local summary vs C improved:
  - `31 / 12 / 4 -> 32 / 11 / 4`
  - bytes-above-C on losers: `1,411 -> 1,307`

Current biggest losses after retaining this point:
- `repo_Cargo.lock`: `+1,152`
- `decodecorpus_z000079`: `+100`
- `dict_dictionary.bin`: `+15`

Retained binary:
- [ruzstd-cli-level1-codeprobe96k-retained](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-codeprobe96k-retained)

## 2026-06-01 - Rejected lockfile-specific repeat block-end early-exit change

Change notes:
- Tried disabling repeat block-end early-exit only for lockfile-like `DictionaryText`.
- This was tested on top of the retained lockfile `oldest +2` baseline.

Source reports:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-norepeatearlyexit-focused.md)
- [sequential restore](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-norepeatblockend-seq.md)

Result:
- focused `Cargo.lock` size was an exact no-op:
  - `repo_Cargo.lock`: `9,114 -> 9,114`
- matcher diagnostics moved slightly, but the parse bytes did not.

Conclusion:
- the remaining `Cargo.lock` gap is not waiting on repeat block-end early-exit
- do not retry this branch in the current lockfile parser shape

## 2026-06-01 - Expanded file-type name coverage and added lockfile/config fixtures to `broad-local`

Change notes:
- Expanded path-based file-type mapping in `ruzstd/src/encoding/mod.rs`.
- Added new named-file coverage:
  - `DictionaryText`:
    - `yarn.lock`
    - `poetry.lock`
    - `pipfile.lock`
    - `gemfile.lock`
    - `composer.lock`
    - `podfile.lock`
    - `mix.lock`
    - `go.sum`
    - `bun.lock`
  - `ConfigText`:
    - `.dockerignore`
    - `.npmrc`
    - `.prettierignore`
    - `.eslintignore`
    - `requirements.txt`
    - `go.mod`
    - `Gemfile`
    - `Pipfile`
- Added mapper tests for:
  - `repo_yarn.lock`
  - `go.sum`
  - `requirements.txt`
  - `repo_.npmrc`

- Expanded `broad-local` corpus in `tools/prepare_benchmark_suites.py` with generated fixtures that hit those mappings:
  - `generated_yarn.lock`
  - `generated_poetry.lock`
  - `generated_go.sum`
  - `generated_requirements.txt`

Source reports:
- [current broad-local baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [broad-local manifest](/home/bsutton/git/zstd-rs/benchmarks/manifests/broad-local.json)

Result:
- `broad-local` now has `51` fixtures.
- Current corrected suite summary vs C `zstd -1` is now:
  - `34 / 13 / 4` better / worse / equal
  - `1,216` total bytes above C on losing fixtures
- New exposed known-file-type losses:
  - `generated_poetry.lock`: `386` vs `362` (`+24`)
  - `generated_yarn.lock`: `403` vs `393` (`+10`)

Conclusion:
- known-file-type coverage is materially better
- future lockfile tuning now has broader targets than `Cargo.lock` alone

## 2026-06-01 - Rejected Poetry-style lockfile detector widening; retained evidence-based lockfile-name trimming

Change notes:
- Tried broadening the internal `likely_lockfile_text()` detector so Poetry-style `[[package]]` / `files = [` blocks would enter the retained Cargo-lock-specific parser path.
- That branch was tested against the retained `lockfile-oldestgain2-after-secondnewestfirst` baseline.

Source reports:
- [Poetry-detector broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-poetry-lockfile-detector-broad-local.md)
- [lockname trim broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockname-trim-broad-local.md)
- [current broad-local baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)

Result:
- broadening the detector was not the right move:
  - `generated_poetry.lock`: stayed on the harmful `386`-byte path
  - `generated_yarn.lock`: stayed on the harmful `403`-byte path
- the real issue was public over-classification:
  - `yarn.lock` and `poetry.lock` as `DictionaryText` were worse than the older generic path
- retained fix:
  - remove `yarn.lock` and `poetry.lock` from `DictionaryText` named-file mapping
- retained result:
  - `generated_poetry.lock`: `386 -> 371`
  - `generated_yarn.lock`: `403 -> 398`
  - no other fixture moved
- corrected broad-local summary vs C improved:
  - `34 / 13 / 4` better / worse / equal
  - bytes-above-C on losers: `1,216 -> 1,196`

Conclusion:
- keep lockfile-name expansion evidence-based
- `Cargo.lock` and `go.sum` stay mapped specially
- `yarn.lock` and `poetry.lock` should stay on the generic path for now

## 2026-06-01 - Retained `poetry.lock` and `yarn.lock` as `ConfigText`

Change notes:
- Tested a narrower file-type policy after trimming the harmful `DictionaryText` mapping:
  - map `poetry.lock` to `ConfigText`
  - map `yarn.lock` to `ConfigText`
- This keeps the change in the public file-type surface, not in another special internal lockfile parser.

Source reports:
- [A/B broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-poetryyarn-config-broad-local.md)
- [current broad-local baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)

Result:
- `generated_poetry.lock`: `371 -> 362`
- `generated_yarn.lock`: `398 -> 390`
- no other fixture moved
- corrected broad-local summary vs C improved:
  - `34 / 13 / 4 -> 35 / 11 / 5`
  - bytes-above-C on losers: `1,196 -> 1,182`

Conclusion:
- `poetry.lock` and `yarn.lock` are better served by the existing `ConfigText` path than by either `DictionaryText` or the generic path

## 2026-06-01 - Rejected `Cargo.lock` as `ConfigText` and `CodeText`

Change notes:
- Re-tested the public starting family for `Cargo.lock` directly against the current retained baseline.
- Tried:
  1. `Cargo.lock -> ConfigText`
  2. `Cargo.lock -> CodeText`

Source reports:
- [ConfigText A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargolock-config-broad-local.csv)
- [CodeText A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargolock-code-broad-local-currentshape.md)
- [restore after CodeText reject](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-cargolock-code-seq.md)

Result:
- both are materially worse than the retained `DictionaryText` path:
  - `ConfigText`:
    - `repo_Cargo.lock`: `9,114 -> 9,255`
  - `CodeText`:
    - `repo_Cargo.lock`: `9,114 -> 9,240`
- matcher diagnostics also showed both generic text families lost the retained lockfile-specific current-entry behavior:
  - no `second_newest` activity
  - heavier `newest` / `oldest` dominance

Conclusion:
- `Cargo.lock` should stay on the retained `DictionaryText` starting point
- the public-family branch is now closed:
  - `DictionaryText` is still clearly best for `Cargo.lock`

## 2026-06-01 - Rejected two more internal lockfile parser branches on the retained `Cargo.lock` path

Change notes:
- Stayed on the retained `Cargo.lock` `DictionaryText` parser path and tested two more internal matcher branches:
  1. require one extra match byte for repeat candidates on the lockfile-specific path
  2. retest dense lockfile probing (`step 1`) on the current retained parser shape

Source reports:
- [repeat-floor A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-repeatfloor6-broad-local.md)
- [step-1 retest A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-step1-retest-broad-local.md)
- [restore after both rejects](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-step1-retest-seq.md)

Result:
- repeat-floor branch regressed:
  - `repo_Cargo.lock`: `9,114 -> 9,117`
- step-1 retest also regressed:
  - `repo_Cargo.lock`: `9,114 -> 9,118`

Useful diagnostics:
- repeat-floor:
  - reduced repeat wins slightly, but did not improve the offset side
- step-1 retest:
  - pushed the parser back toward more sequences and heavier `newest` / `oldest` dominance

Conclusion:
- neither suppressing the shortest repeat matches nor restoring dense lockfile probing helps on the current retained parser shape

## 2026-06-01 - Rejected stronger repeat-vs-normal margin on the retained lockfile path

Change notes:
- Tried increasing the repeat-vs-normal match margin only on the retained lockfile-specific `DictionaryText` path.

Source reports:
- [A/B broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-repeatmarginplus2-broad-local.md)

Result:
- exact byte-for-byte no-op on the retained baseline
- `repo_Cargo.lock`: stayed `9,114`

Conclusion:
- the current lockfile path is not waiting on a larger repeat-vs-normal scoring margin

## 2026-06-01 - Rejected three more focused `Cargo.lock` branches

Change notes:
- Stayed on the retained `Cargo.lock` parser shape and tested three more focused branches against the retained baseline binary before promoting anything to `broad-local`:
  1. lockfile-only fastest whole-vs-estimated-split candidate using the existing best-level partition machinery
  2. lockfile-only zero-literal non-repeat floor `+1`
  3. `DictionaryText` exact Huffman search also evaluating flat-distribution max-bit variants

Source reports:
- [lockfile partition focused](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-partition-focused.md)
- [lockfile zero-literal floor focused](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-zero-literal-floor-focused.md)
- [lockfile flat-Huff search focused](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-flat-huff-search-focused.md)

Result:
- partition candidate: exact no-op
  - `repo_Cargo.lock`: stayed `9,114`
- zero-literal non-repeat floor `+1`: regression
  - `repo_Cargo.lock`: `9,114 -> 9,143`
- flat-distribution exact Huffman search: exact no-op
  - `repo_Cargo.lock`: stayed `9,114`

Conclusion:
- the current `Cargo.lock` path is not waiting on:
  - a best-level-style split candidate
  - a stricter zero-literal non-repeat floor
  - flat-distribution exact Huffman table search

## 2026-06-01 - Rejected TOML single-stream exception and DictionaryText `newest` displacement

Change notes:
- Tried a narrow `ConfigText` entropy policy exception:
  - keep the retained small-config single-stream Huffman rule generally
  - but skip it when the literal payload looks like a TOML manifest, so `Cargo.toml` can use the normal 4-stream path
- Also tried a narrow `DictionaryText` matcher rule:
  - keep the current non-repeat candidate over a farther `newest` window hit when the farther hit costs at least 4 more offset-code bits and gains less than 2 match bytes

Source reports:
- [TOML focused](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/toml-config-multistream-focused.md)
- [Dictionary newest focused](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/dict-newestgain-focused.md)

Result:
- TOML single-stream exception regressed:
  - `repo_ruzstd_Cargo.toml`: `730 -> 737`
- Dictionary `newest` displacement was an exact no-op:
  - `dict_dictionary.bin`: stayed `20,160`

Conclusion:
- the retained `ConfigText` single-stream policy should stay intact for now
- the remaining dictionary gap is not waiting on this `newest`-side current-window rule

## 2026-06-01 - Rejected generic DictionaryText probe step `1 -> 2`

Change notes:
- Re-tested the generic `DictionaryText` probe-density family with the lockfile path left unchanged.
- Changed only non-lockfile `DictionaryText` short-line probing from dense step `1` to step `2`.

Source reports:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/dict-step2-focused.md)

Result:
- hard regression on the target dictionary fixture:
  - `dict_dictionary.bin`: `20,160 -> 20,667`

Conclusion:
- the old rejected wider-step signal was real in the current parser shape too
- generic `DictionaryText` still wants fully dense probe step `1`

## 2026-06-01 - Rejected wider lockfile `second_newest` recent-entry reach

Change notes:
- Stayed on the retained `Cargo.lock` parser shape and widened the lockfile-only `second_newest` recent-entry limit from `2` to `3`.
- Added a focused matcher test first, then benchmarked `repo_Cargo.lock` directly against the retained baseline binary instead of spending a full `broad-local` pass up front.

Source reports:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-secondnewest-limit3-focused.md)

Result:
- exact byte-for-byte no-op on focused `Cargo.lock`
- `repo_Cargo.lock`: stayed `9,114`

Conclusion:
- the retained lockfile `second_newest` path does not benefit from reaching one more older current-window entry
- the next credible `Cargo.lock` branch is no longer a wider recent-entry reach in this family

## 2026-06-01 - Rejected lockfile zero-literal high-offset filter

Change notes:
- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,114`
- Tested one narrow lockfile-specific matcher rule in `ruzstd/src/encoding/match_generator.rs`:
  - reject zero-literal non-repeat window candidates when they are only `5` bytes long and cost at least `11` offset-code bits
- Benchmarked the focused lockfile family against the retained binary before promoting anything to `broad-local`.

Source reports:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-zero-literal-high-offset-focused.md)

Result:
- focused lockfile family:
  - `repo_Cargo.lock`: `9,114 -> 9,132`
  - `generated_go.sum`: stayed `151`
  - `generated_poetry.lock`: stayed `362`
  - `generated_yarn.lock`: stayed `390`

Conclusion:
- the retained `Cargo.lock` parser does not want this high-offset zero-literal filter
- the remaining lockfile gap is not improved by suppressing these short high-offset non-repeat window matches in this form

## 2026-06-01 - Retained rank-limited exact Huffman candidate

Change notes:
- Reintroduced a shared literal-model branch in `ruzstd/src/huff0/huff0_encoder.rs`:
  - when `build_smallest_from_counts()` searches exact Huffman tables, also evaluate the `rank_limited_weights()` candidate and keep it if the fully encoded literal section is shorter
- This was screened first on the tiny config-like and lockfile sentinels, then promoted to `broad-local`.

Source reports:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/config-ranklimited-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-config-ranklimited-broad-local.md)
- [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-config-ranklimited-fast.md)

Focused result:
- `repo_.gitignore`: `172 -> 166`
- `dict_talk.service`: `160 -> 151`
- `generated_poetry.lock`: `362 -> 359`
- `repo_Cargo.lock`: stayed `9,114`
- `repo_ruzstd_Cargo.toml`: stayed `730`

Useful live literal evidence:
- `repo_.gitignore`
  - before: `literals_payload=137`, `literals_table_desc=32`, `literals_stream=105`
  - after: `literals_payload=131`, `literals_table_desc=22`, `literals_stream=109`
  - C `zstd -1`: `literals_payload=129`, `literals_table_desc=24`, `literals_stream=105`
- `dict_talk.service`
  - before: `literals_payload=130`, `literals_table_desc=36`, `literals_stream=94`
  - after: `literals_payload=121`, `literals_table_desc=25`, `literals_stream=96`

Broad-local result vs the retained `huffweight-maxlog5` baseline:
- wins only, no regressions
- notable movers:
  - `dict_talk.service`: `160 -> 151`
  - `repo_.gitignore`: `172 -> 166`
  - `generated_yarn.lock`: `390 -> 383`
  - `generated_poetry.lock`: `362 -> 359`
  - `dict_git-daemon@.service`: `241 -> 237`
  - `dict_glustereventsd.service`: `285 -> 281`
  - `dict_gpm.service`: `191 -> 190`

Current corrected `broad-local` summary vs C `zstd -1`:
- `37 / 10 / 4` better / worse / equal
- `1,170` total bytes above C on the losing fixtures

Current top losses:
- `repo_Cargo.lock`: `+1,026`
- `decodecorpus_z000079`: `+101`
- `dict_dictionary.bin`: `+15`

Conclusion:
- this is a real retained literal-model win for known file types
- it improves the exact tiny literal-header tail that was still open
- it leaves the `Cargo.lock` and fast-guardrail paths unchanged

## 2026-06-01 - Rejected lockfile stream-first Huffman search

Change notes:
- Stayed on the retained `config-ranklimited` baseline:
  - `repo_Cargo.lock = 9,114`
- Tested one narrow lockfile-only literal branch:
  - for `Cargo.lock`-like `DictionaryText`, keep the current exact-table search space but choose the new Huffman table by coded stream size instead of total literal payload size
- This stayed entirely internal to `compress_literals()` and still competed normally against the repeat-table path on total estimated size.

Source reports:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-streamfirst-focused.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-streamfirst-restore.md)

Result:
- exact byte-for-byte no-op on the focused lockfile family
  - `repo_Cargo.lock`: stayed `9,114`
  - `generated_go.sum`: stayed `151`
  - `generated_poetry.lock`: stayed `359`
  - `generated_yarn.lock`: stayed `383`

Conclusion:
- the retained lockfile gap is not waiting on a stream-first re-ranking of the current exact Huffman candidate set
- this closes another lockfile literal-model family without disturbing the retained baseline

## 2026-06-01 - Retained generic DictionaryText current-entry second_newest

Change notes:
- In `ruzstd/src/encoding/match_generator.rs`, non-lockfile text-like `DictionaryText` blocks now track and probe the current-entry `second_newest` sidecar at level 1.
- This uses the existing current-entry `second_newest` machinery already retained for `Cargo.lock`, but it leaves the special lockfile ordering alone.

Source reports:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/dict-secondnewest-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dict-secondnewest-broad-local.md)
- [fast guardrails](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dict-secondnewest-fast.md)

Focused result:
- `dict_dictionary.bin`: `20,160 -> 19,668`
- `repo_Cargo.lock`: stayed `9,114`
- `generated_go.sum`: stayed `151`
- `generated_poetry.lock`: stayed `359`
- `generated_yarn.lock`: stayed `383`

Useful live matcher evidence on `dict_dictionary.bin`:
- before:
  - `window_current_second_newest[0] = 0`
  - `window_current_newest[0] = 2527`
  - `window_current_oldest[0] = 1710`
- after:
  - `window_current_second_newest[0] = 604`
  - `window_current_second_newest_zero_literals[0] = 378`
  - `window_current_newest[0] = 2417`
  - `window_current_oldest[0] = 1330`

Broad-local result vs the retained `config-ranklimited` baseline:
- only one fixture moved
  - `dict_dictionary.bin`: `20,160 -> 19,668`
- fast guardrails stayed flat outside that same fixture

Current corrected `broad-local` summary vs C `zstd -1`:
- `38 / 9 / 4` better / worse / equal
- `1,155` total bytes above C on the losing fixtures

Current top losses:
- `repo_Cargo.lock`: `+1,026`
- `decodecorpus_z000079`: `+101`
- `decodecorpus_z000059`: `+13`

Conclusion:
- this is a clean retained DictionaryText parser win
- it turns `dict_dictionary.bin` from a small loser into a meaningful win versus C
- the remaining known-file-type gap is now even more concentrated in `Cargo.lock`

## 2026-06-01 - Rejected lockfile structural midpoint split

Change notes:
- Stayed on the retained `dict-secondnewest` baseline:
  - `repo_Cargo.lock = 9,114`
- Tested one structural lockfile-only branch in `ruzstd/src/encoding/levels/fastest.rs`:
  - if a large `Cargo.lock`-like `DictionaryText` block is seen at level 1, split it once at the newline nearest the midpoint and compress the two halves as separate blocks
- This was meant to reset literal/sequence entropy per half without changing the public API.

Source reports:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-structuralsplit-focused.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-structuralsplit-restore-seq.md)

Result:
- exact byte-for-byte no-op on the focused lockfile family
  - `repo_Cargo.lock`: stayed `9,114`
  - `generated_go.sum`: stayed `151`
  - `generated_poetry.lock`: stayed `359`
  - `generated_yarn.lock`: stayed `383`

Conclusion:
- the retained `Cargo.lock` gap is not waiting on a forced two-block midpoint split in this form
- this closes another structural lockfile representation family without changing the retained baseline

## 2026-06-01 - Rejected lockfile zero-literal short second_newest filter

Change notes:
- Stayed on the retained `dict-secondnewest` baseline:
  - `repo_Cargo.lock = 9,114`
- Tested one narrow lockfile-only matcher branch in `ruzstd/src/encoding/match_generator.rs`:
  - reject lockfile `second_newest` window candidates when they are zero-literal, non-repeat, and only `5` bytes long
- This was aimed at trimming the weakest zero-literal `second_newest` wins because `Cargo.lock` is still over-sequenced relative to C.

Source reports:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-secondnewest-zerolit-floor-focused.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-secondnewest-zerolit-floor-restore-seq.md)

Result:
- exact byte-for-byte no-op on the focused lockfile family
  - `repo_Cargo.lock`: stayed `9,114`
  - `generated_go.sum`: stayed `151`
  - `generated_poetry.lock`: stayed `359`
  - `generated_yarn.lock`: stayed `383`

Conclusion:
- the retained lockfile gap is not waiting on a higher minimum length for zero-literal `second_newest` wins in this form
- this closes another narrow lockfile zero-literal matcher family without changing the retained baseline

## 2026-06-01 - Rejected Cargo.toml -> CodeText

Change notes:
- Stayed on the retained `dict-secondnewest` baseline.
- Tested one narrow path-mapping branch in `ruzstd/src/encoding/mod.rs`:
  - map `Cargo.toml` filenames to `CodeText` instead of `ConfigText`
- This was screened only on the `Cargo.toml` fixture family before considering any wider suite update.

Source reports:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/cargotoml-code-focused.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/cargotoml-code-restore-seq.md)

Result:
- exact byte-for-byte no-op on the focused `Cargo.toml` family
  - `repo_Cargo.toml`: stayed `68`
  - `repo_cli_Cargo.toml`: stayed `489`
  - `repo_ruzstd_Cargo.toml`: stayed `730`
  - `repo_ruzstd_fuzz_Cargo.toml`: stayed `340`

Conclusion:
- on the current retained baseline, `Cargo.toml` is not improved by switching from `ConfigText` to `CodeText`
- this closes another known-file-type mapping branch without changing the retained baseline

## 2026-06-01 - Rejected composer.lock and Pipfile.lock -> ConfigText

Change notes:
- Stayed on the retained `dict-secondnewest` baseline:
  - `repo_Cargo.lock = 9,114`
- Tested one narrow file-type mapping branch in `ruzstd/src/encoding/mod.rs`:
  - map `composer.lock` and `Pipfile.lock` to `ConfigText` instead of `DictionaryText`
- This was screened on the affected lockfile family before any broader suite refresh.

Source reports:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/configlock-focused.md)

Result:
- regression on both remapped targets:
  - `generated_composer.lock`: `4,461 -> 4,469`
  - `generated_pipfile.lock`: `2,811 -> 2,879`
- unchanged nearby controls:
  - `generated_package-lock.json`: stayed `4,392`
  - `generated_go.sum`: stayed `151`
  - `repo_Cargo.lock`: stayed `9,114`

Conclusion:
- `composer.lock` and `Pipfile.lock` are not improved by the current `ConfigText` path
- this closes the plain `ConfigText` remap for those two lockfiles without changing the retained baseline

## 2026-06-01 - Rejected three focused composer.lock internal parser branches

Change notes:
- Stayed on the retained `dict-secondnewest` baseline:
  - `generated_composer.lock = 4,461`
- Focused the screen on:
  - `generated_composer.lock`
  - `generated_pipfile.lock`
  - `generated_package-lock.json`
  - `generated_go.sum`
  - `repo_Cargo.lock`

Rejected branches:
- treat large composer-style JSON lockfiles as the retained lockfile parser path inside `DictionaryText`
- raise the non-repeat floor to `6` for large composer-style `DictionaryText`
- prefer smaller non-repeat offsets for composer-style `DictionaryText` when the farther match only gains a byte

Source reports:
- [lockfile-path focused](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-lockpath-focused.md)
- [floor 6 focused](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-floor6-focused.md)
- [smaller-offset focused](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-offset-focused.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-restore.md)

Result:
- all three were exact byte-for-byte no-ops on the focused family:
  - `generated_composer.lock`: stayed `4,461`
  - `generated_pipfile.lock`: stayed `2,811`
  - `generated_package-lock.json`: stayed `4,392`
  - `generated_go.sum`: stayed `151`
  - `repo_Cargo.lock`: stayed `9,114`

Conclusion:
- the remaining `generated_composer.lock` gap is not moving on:
  - the retained Cargo-style lockfile parser path
  - a stronger short non-repeat floor
  - a broader smaller-offset preference
- this keeps pointing at a different sequence/repeat representation rather than another small parser threshold

## 2026-06-01 - Rejected composer current-entry long-hash path

Change notes:
- Stayed on the retained `dict-secondnewest` baseline:
  - `generated_composer.lock = 4,461`
- Tested one focused internal matcher branch in `ruzstd/src/encoding/match_generator.rs`:
  - enable the current-entry long-hash path for large composer-style `DictionaryText`

Source reports:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-longhash-focused.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-longhash-restore.md)

Result:
- exact byte-for-byte no-op on the focused family:
  - `generated_composer.lock`: stayed `4,461`
  - `generated_pipfile.lock`: stayed `2,811`
  - `generated_package-lock.json`: stayed `4,392`
  - `generated_go.sum`: stayed `151`
  - `repo_Cargo.lock`: stayed `9,114`

Conclusion:
- the remaining composer gap is not waiting on the current-entry long-hash family either
- this further tightens the read that the live matcher space is largely exhausted for composer on the current representation

## 2026-06-01 - Rejected composer zero-literal rep1-1 ordering

Change notes:
- Stayed on the retained `dict-secondnewest` baseline:
  - `generated_composer.lock = 4,461`
- Tested one focused internal repeat branch in `ruzstd/src/encoding/match_generator.rs`:
  - for composer-style `DictionaryText`, try the zero-literal `rep1-1` candidate first instead of last

Source reports:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-repfirst-focused.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-repfirst-restore.md)

Result:
- exact byte-for-byte no-op on the focused family:
  - `generated_composer.lock`: stayed `4,461`
  - `generated_pipfile.lock`: stayed `2,811`
  - `generated_package-lock.json`: stayed `4,392`
  - `generated_go.sum`: stayed `151`
  - `repo_Cargo.lock`: stayed `9,114`

Conclusion:
- the remaining composer gap is not waiting on zero-literal repcode ordering either
- this is another sign that the live matcher search space is largely exhausted for composer on the current representation

## 2026-06-01 - Rejected composer ip+1 repeat comparison branch

Change notes:
- Stayed on the retained `dict-secondnewest` baseline:
  - `generated_composer.lock = 4,461`
- Tested one focused internal repeat branch in `ruzstd/src/encoding/match_generator.rs`:
  - for composer-style `DictionaryText`, allow `ip+1` repeat candidates to be compared even when a current-position repeat candidate already exists

Source reports:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-nextrep-focused.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-nextrep-restore.md)

Result:
- exact byte-for-byte no-op on the focused family:
  - `generated_composer.lock`: stayed `4,461`
  - `generated_pipfile.lock`: stayed `2,811`
  - `generated_package-lock.json`: stayed `4,392`
  - `generated_go.sum`: stayed `151`
  - `repo_Cargo.lock`: stayed `9,114`

Conclusion:
- the remaining composer gap is not waiting on the `ip+1` repeat family either
- the repeat-search space is now very tight on the current representation

## 2026-06-01 - Retained indexed file-type lookup expansion and sampled fallback

Change notes:
- Expanded `ruzstd/src/encoding/mod.rs` from small linear extension/name tables to indexed exact-match classification helpers:
  - broader known extension coverage
  - broader known file-name coverage
  - compound extension handling such as `tar.gz`
  - prefixed synthetic-name suffix matching still preserved
- Added `compression_file_type_for_path_and_data()` and wired `compress_with_path()` to sample up to `32 KiB` before falling back to `Unknown`
- Added sample-based fallback classification for:
  - archive signatures
  - binary signatures
  - JSON-like text
  - config-like text
  - code-like text
  - Cargo/composer-style lockfile text
- Expanded `broad-local` fixture generation in `tools/prepare_benchmark_suites.py` with:
  - `generated_package.json`
  - `generated_tsconfig.json`
  - `generated_pyproject.toml`
  - `generated_pom.xml`
  - `generated_Dockerfile`

Source reports:
- [current broad-local baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [broad-local manifest](/home/bsutton/git/zstd-rs/benchmarks/manifests/broad-local.json)

Current result:
- `62` fixtures
- vs C `zstd -1`: `43 / 15 / 4` better / worse / equal
- total bytes above C on losing fixtures: `2,421`

Largest remaining losses:
- `repo_Cargo.lock`: `9,114` vs `8,088` (`+1,026`)
- `generated_composer.lock`: `4,336` vs `3,766` (`+570`)
- `decodecorpus_z000079`: `7,530` vs `7,221` (`+309`)
- `generated_package.json`: `3,956` vs `3,826` (`+130`)
- `repo_prepare_benchmark_suites.py`: `6,463` vs `6,338` (`+125`)

Verification:
- `python3 -m py_compile tools/prepare_benchmark_suites.py`
- `cargo fmt --all`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q -p ruzstd encoding::tests -- --nocapture`
- `cargo test -q --workspace`
- `cargo build --release -q -p ruzstd-cli`

## 2026-06-01 - Tightened sampled fallback and expanded special-name coverage

Change notes:
- Refined sampled fallback in `ruzstd/src/encoding/mod.rs`:
  - removed the generic “plain text => ConfigText” fallback
  - sampled fallback now only claims a family on strong evidence
  - ambiguous text now stays `Unknown` after sampling
- Expanded special-name coverage for additional known file families:
  - Bazel/Starlark:
    - `BUILD.bazel`
    - `MODULE.bazel`
    - `WORKSPACE`
    - `.bzl` / `.bazel`
  - Dart / Flutter:
    - `pubspec.yaml`
    - `pubspec.lock`
    - `melos.yaml`
  - additional special names:
    - `Podfile`
    - `Brewfile`
- Expanded `broad-local` fixtures with:
  - `generated_pubspec.yaml`
  - `generated_pubspec.lock`
  - `generated_BUILD.bazel`
  - `generated_WORKSPACE`

Source reports:
- [current broad-local baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [broad-local manifest](/home/bsutton/git/zstd-rs/benchmarks/manifests/broad-local.json)

Current result:
- `66` fixtures
- vs C `zstd -1`: `46 / 16 / 4` better / worse / equal
- total bytes above C on losing fixtures: `2,122`

Important movement from the fallback tightening:
- `decodecorpus_z000079`: `7,530 -> 7,322`
- total bytes-above-C on losers improved from `2,421 -> 2,082` before adding the four new fixtures

Current largest losses after the new fixture expansion:
- `repo_Cargo.lock`: `9,114` vs `8,088` (`+1,026`)
- `generated_composer.lock`: `4,336` vs `3,766` (`+570`)
- `repo_prepare_benchmark_suites.py`: `7,184` vs `7,039` (`+145`)
- `generated_package.json`: `3,956` vs `3,826` (`+130`)
- `decodecorpus_z000079`: `7,322` vs `7,221` (`+101`)

Verification:
- `cargo fmt --all`
- `python3 -m py_compile tools/prepare_benchmark_suites.py`
- `cargo test -q -p ruzstd encoding::tests -- --nocapture`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo build --release -q -p ruzstd-cli`
- `cargo test -q --workspace`

## 2026-06-01 - Rejected JSON-config matcher branches

Change notes:
- Tested two narrow branches for large JSON-config known files:
  1. `package.json` / `tsconfig.json` / `jsconfig.json` / `composer.json` remap to `JsonText`
  2. `ConfigText` structured-JSON short-line non-repeat floor `7`

Source reports:
- [json-config remap focused broad-local](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/packagejson-jsontest.md)
- [structured-json floor 7 focused broad-local](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/jsonconfig-floor7.md)

Result:
- remap to `JsonText`: exact no-op on the exposed targets
  - `generated_package.json`: stayed `3,956`
  - `generated_tsconfig.json`: stayed `2,492`
- structured-JSON floor `7`: hard reject
  - `generated_package.json`: `3,956 -> 3,960`
  - `generated_tsconfig.json`: `2,492 -> 3,292`

Restore:
- rebuilt source tree confirmed back at:
  - `generated_package.json = 3,956`
  - `generated_tsconfig.json = 2,492`

Conclusion:
- the JSON-config tail is not fixed by a plain family remap to `JsonText`
- it is also not fixed by a blunt stronger short-line floor
- the remaining gap there needs a narrower parse/sequence representation change rather than a broader family or threshold move

## 2026-06-01 - Rejected structured-JSON cheaper repeat-code bias

Change notes:
- Tested one narrow matcher branch in `ruzstd/src/encoding/match_generator.rs`:
  - for structured JSON `ConfigText` at zero literals, prefer the cheaper repeat code over a more expensive repeat code when the match-length loss is at most `2`

Focused result:
- exact byte-for-byte no-op on the exposed fixtures:
  - `generated_package.json`: stayed `3,956`
  - `generated_tsconfig.json`: stayed `2,492`

Useful matcher evidence:
- the branch did change the chosen repeat family for `generated_tsconfig.json`:
  - before:
    - `repeat_current[2] = 4521`
    - `repeat_current[1] = 2`
  - under test:
    - `repeat_current[1] = 3015`
    - `repeat_current[0] = 1526`
    - `repeat_current[2] = 3`
- but that did not change:
  - compressed bytes
  - literal section bytes
  - sequence payload bytes
  - sequence count

Restore:
- rebuilt source tree confirmed back at:
  - `generated_package.json = 3,956`
  - `generated_tsconfig.json = 2,492`

Conclusion:
- shifting repeat-code rank alone is not enough for the JSON-config tail
- the remaining issue is a deeper zero-literal sequence-shape problem, not just which repeat code wins within the current shape

## 2026-06-01 - Rejected structured-JSON repeat-shape branches

Change notes:
- Tested two narrow structured-JSON `ConfigText` matcher branches:
  1. disable repeat-length early exit
  2. reject zero-literal repeat matches of length `5`

Focused result:
- repeat-length early exit disable: exact no-op
  - `generated_package.json`: stayed `3,956`
  - `generated_tsconfig.json`: stayed `2,492`
- zero-literal repeat min len `6`: exact byte-for-byte no-op on the focused targets
  - `generated_package.json`: stayed `3,956`
  - `generated_tsconfig.json`: stayed `2,492`

Useful matcher evidence:
- the zero-literal repeat min-len branch did change the chosen parse for `generated_tsconfig.json`:
  - total sequences: `4567 -> 4522`
  - `repeat_current[0]`: `21 -> 110`
  - `repeat_current[2]`: `4521 -> 4342`
  - `window_current_oldest[0]`: `13 -> 57`
- but that still did not move compressed bytes or aggregate section sizes

Restore:
- rebuilt source tree confirmed back at:
  - `generated_package.json = 3,956`
  - `generated_tsconfig.json = 2,492`

Conclusion:
- the JSON-config tail is not fixed by simply letting repeats compete longer
- it is also not fixed by suppressing minimum-length zero-literal repeats
- the next parser branch has to change sequence shape more materially than either of these local repeat gates

## 2026-06-01 - Retained known-name classifier expansion and broader broad-local coverage

Change notes:
- Expanded exact known-name and extension coverage in `ruzstd/src/encoding/mod.rs`.
- New named-file coverage includes:
  - `api-extractor.json`
  - `azure-pipelines.yml`
  - `buf.yaml`, `buf.gen.yaml`, `buf.work.yaml`
  - `deno.json`, `deno.jsonc`, `devcontainer.json`
  - `lerna.json`
  - `netlify.toml`, `nx.json`
  - `release-please-config.json`, `renovate.json`
  - `taskfile.yml`
  - `turbo.json`, `typedoc.json`
  - `wrangler.toml`
- New extension coverage includes:
  - code-like: `.bicep`, `.nix`
  - config-like: `.pbxproj`, `.props`, `.resx`, `.tf`, `.tfvars`, `.xcconfig`, `.xconfig`
  - JSON-like: `.hjson`, `.xcstrings`
- Expanded `broad-local` corpus in `tools/prepare_benchmark_suites.py` with:
  - `generated_turbo.json`
  - `generated_deno.json`
  - `generated_nx.json`
  - `generated_wrangler.toml`
  - `generated_buf.yaml`

Refreshed retained baseline:
- `71` fixtures
- vs C `zstd -1`: `48 / 19 / 4` better / worse / equal
- `2,449` total bytes above C on losing fixtures

Largest losses on the expanded suite:
- `repo_Cargo.lock`: `9,114` vs `8,088` (`+1,026`)
- `generated_composer.lock`: `4,336` vs `3,766` (`+570`)
- `repo_prepare_benchmark_suites.py`: `7,221` vs `7,081` (`+140`)
- `generated_package.json`: `3,956` vs `3,826` (`+130`)
- `generated_turbo.json`: `3,956` vs `3,826` (`+130`)
- `generated_tsconfig.json`: `2,492` vs `2,391` (`+101`)
- `generated_deno.json`: `2,492` vs `2,391` (`+101`)
- `generated_nx.json`: `2,492` vs `2,391` (`+101`)
- `decodecorpus_z000079`: `7,322` vs `7,221` (`+101`)

Conclusion:
- the classifier now covers materially more real config/build ecosystem names without relying on sample fallback
- the larger suite is more honest about known-file-type tails
- the next compression work should stay focused on the exposed known-family gaps, especially:
  - `Cargo.lock`
  - composer-style lockfiles
  - JSON-config files (`package.json` / `turbo.json`, `tsconfig.json` / `deno.json` / `nx.json`)

## 2026-06-01 - Retained CodeText short-line current-entry second_newest path

Change notes:
- Added a narrow `CodeText` current-entry `second_newest` matcher path in
  `ruzstd/src/encoding/match_generator.rs`.
- Scope:
  - `CodeText`
  - short-line text blocks
  - block length `16 KiB ..= 64 KiB`
- This reuses the existing current-entry `second_newest` sidecar machinery.
- Added focused matcher tests for:
  - code-text sidecar enabled within the new range
  - code-text sidecar disabled above the cutoff

Focused A/B vs the retained baseline:
- `repo_prepare_benchmark_suites.py`: `7,221 -> 6,827`
- `repo_match_generator.rs`: `28,078 -> 27,845`
- unchanged focused controls:
  - `repo_benchmark_zstd.py`: `2,814`
  - `repo_compressed.rs`: `13,046`
  - `repo_main.rs`: `2,125`

Useful matcher evidence on `repo_prepare_benchmark_suites.py`:
- before:
  - `window_current_second_newest[0] = 0`
  - `window_current_newest[0] = 866`
  - `window_current_oldest[0] = 430`
- after:
  - `window_current_second_newest[0] = 72`
  - `window_current_second_newest_zero_literals[0] = 32`
  - `window_current_newest[0] = 868`
  - `window_current_oldest[0] = 372`

Refreshed retained baseline:
- `71` fixtures
- vs C `zstd -1`: `49 / 18 / 4` better / worse / equal
- `2,309` total bytes above C on losing fixtures

Largest remaining losses:
- `repo_Cargo.lock`: `9,114` vs `8,088` (`+1,026`)
- `generated_composer.lock`: `4,336` vs `3,766` (`+570`)
- `generated_package.json`: `3,956` vs `3,826` (`+130`)
- `generated_turbo.json`: `3,956` vs `3,826` (`+130`)
- `generated_tsconfig.json`: `2,492` vs `2,391` (`+101`)
- `generated_deno.json`: `2,492` vs `2,391` (`+101`)
- `generated_nx.json`: `2,492` vs `2,391` (`+101`)
- `decodecorpus_z000079`: `7,322` vs `7,221` (`+101`)

Conclusion:
- this was a real code-family matcher gap
- the new path materially improves medium-size short-line code blocks without broad collateral movement
- the next highest-value work is still `Cargo.lock`, composer-style lockfiles, and JSON-config parse shape

## 2026-06-01 - Rejected structured-JSON stronger zero-literal repeat floor

Change notes:
- Tested a narrower structured-JSON `ConfigText` matcher branch in
  `ruzstd/src/encoding/match_generator.rs`.
- Scope:
  - structured JSON `ConfigText`
  - zero-literal repeat candidates only
  - require longer matches for the heavier repeat slots:
    - second repeat candidate: min len `8`
    - third repeat candidate: min len `10`

Focused result:
- `generated_package.json`: unchanged at `3,956`
- `generated_turbo.json`: unchanged at `3,956`
- hard regressions:
  - `generated_tsconfig.json`: `2,492 -> 3,446`
  - `generated_deno.json`: `2,492 -> 3,446`
  - `generated_nx.json`: `2,492 -> 3,446`

Restore:
- rebuilt source tree confirmed back at:
  - `generated_package.json = 3,956`
  - `generated_turbo.json = 3,956`
  - `generated_tsconfig.json = 2,492`
  - `generated_deno.json = 2,492`
  - `generated_nx.json = 2,492`

Conclusion:
- the JSON-config family does not want a blunt stronger zero-literal repeat floor on the heavier repeat slots
- this was more disruptive than the earlier no-op repeat branches and moved in the wrong direction
- the next credible JSON-config branch should avoid direct repeat-length gating and instead target a different sequence-shape mechanism

## 2026-06-01 - Rejected composer current-window offset-choice branch

Change notes:
- Tested one composer-style `DictionaryText` matcher branch in
  `ruzstd/src/encoding/match_generator.rs`.
- Idea:
  - for composer-like lockfiles only,
  - keep the current smaller-offset non-repeat candidate over a farther current-window
    `newest` or `oldest` candidate unless the farther candidate gains at least `2` match bytes
    and also overcomes the offset-code-bit gap

Focused result:
- exact byte-for-byte no-op on the focused family:
  - `generated_composer.lock`: stayed `4,336`
  - `generated_pipfile.lock`: stayed `2,811`
  - `generated_package-lock.json`: stayed `4,392`
  - `generated_go.sum`: stayed `151`
  - `repo_Cargo.lock`: stayed `9,114`

Restore:
- rebuilt source tree confirmed back at:
  - `generated_composer.lock = 4,336`
  - `generated_pipfile.lock = 2,811`
  - `generated_package-lock.json = 4,392`
  - `generated_go.sum = 151`
  - `repo_Cargo.lock = 9,114`

Conclusion:
- the remaining composer gap is not moving on this current-window offset-choice family either
- current-window `newest`/`oldest` scoring tweaks are effectively closed for the composer path in this form

## 2026-06-01 - Rejected package-style JSON next-position repeat branch

Change notes:
- Tested a package-style JSON `ConfigText` matcher branch in
  `ruzstd/src/encoding/match_generator.rs`.
- Idea:
  - identify package-style JSON configs (`package.json` / `turbo.json`-like content)
  - allow the existing `ip+1` repeat-lookahead path to run there so a short current-position
    match can yield to a longer next-position repeat

Focused result:
- exact byte-for-byte no-op on the focused family:
  - `generated_package.json`: stayed `3,956`
  - `generated_turbo.json`: stayed `3,956`
  - `generated_tsconfig.json`: stayed `2,492`
  - `generated_deno.json`: stayed `2,492`
  - `generated_nx.json`: stayed `2,492`

Restore:
- rebuilt source tree confirmed back at:
  - `generated_package.json = 3,956`
  - `generated_turbo.json = 3,956`
  - `generated_tsconfig.json = 2,492`
  - `generated_deno.json = 2,492`
  - `generated_nx.json = 2,492`

Conclusion:
- the package-style JSON family is not moving on this next-position repeat-lookahead path either
- the earlier broader structured-JSON attempt was not a safe win to keep
- the next credible JSON-config branch still needs a different sequence-shape mechanism

## 2026-06-01 - Rejected package-style JSON current-entry second_newest branch

Change notes:
- Tested a package-style JSON `ConfigText` matcher branch in
  `ruzstd/src/encoding/match_generator.rs`.
- Idea:
  - identify package-style JSON configs (`package.json` / `turbo.json`-like content)
  - track and probe the current-entry `second_newest` sidecar for that family, similar to the
    retained `CodeText` current-entry `second_newest` win

Focused result:
- exact byte-for-byte no-op on the focused family:
  - `generated_package.json`: stayed `3,956`
  - `generated_turbo.json`: stayed `3,956`
  - `generated_tsconfig.json`: stayed `2,492`
  - `generated_deno.json`: stayed `2,492`
  - `generated_nx.json`: stayed `2,492`

Restore:
- rebuilt source tree confirmed back at:
  - `generated_package.json = 3,956`
  - `generated_turbo.json = 3,956`
  - `generated_tsconfig.json = 2,492`
  - `generated_deno.json = 2,492`
  - `generated_nx.json = 2,492`

Conclusion:
- package-style JSON is not moving on current-entry `second_newest` either
- the next credible JSON-config branch still needs a different sequence-shape mechanism, not
  another current-entry sidecar or repeat-lookahead variant

## 2026-06-01 - Retained structured-JSON ConfigText dense probe step

Change notes:
- Added a structured-JSON `ConfigText` parser branch in
  `ruzstd/src/encoding/match_generator.rs`.
- Scope:
  - `CompressionFileType::ConfigText`
  - short-line text
  - content-detected structured JSON objects
  - block length `<= 128 KiB`
- Change:
  - use dense no-match probe step `1` for that family instead of the generic short-line text
    step `2`
- Added focused matcher tests for:
  - structured JSON config detection
  - dense probe step selection for medium structured JSON config blocks

Focused result vs the retained baseline:
- `generated_package.json`: `3,956 -> 3,785`
- `generated_turbo.json`: `3,956 -> 3,785`
- unchanged focused controls:
  - `generated_tsconfig.json`: `2,492`
  - `generated_deno.json`: `2,492`
  - `generated_nx.json`: `2,492`

Broad-local result:
- only two fixtures moved:
  - `generated_package.json`: `3,956 -> 3,785`
  - `generated_turbo.json`: `3,956 -> 3,785`
- both are now better than C `zstd -1`:
  - `3,785` vs `3,826`
- refreshed retained baseline:
  - `71` fixtures
  - vs C `zstd -1`: `51 / 16 / 4` better / worse / equal
  - `2,049` total bytes above C on losing fixtures

Largest remaining losses:
- `repo_Cargo.lock`: `9,114` vs `8,088` (`+1,026`)
- `generated_composer.lock`: `4,336` vs `3,766` (`+570`)
- `decodecorpus_z000079`: `7,322` vs `7,221` (`+101`)
- `generated_tsconfig.json`: `2,492` vs `2,391` (`+101`)
- `generated_deno.json`: `2,492` vs `2,391` (`+101`)
- `generated_nx.json`: `2,492` vs `2,391` (`+101`)

Conclusion:
- package-style JSON did want a denser sequence search, but only through a real probe-step
  change, not another repeat/window scoring tweak
- the tsconfig-style JSON family did not move, so it remains a separate sequence-shape problem

## 2026-06-01 - Retained tsconfig-style JSON ConfigText wider probe step

Change notes:
- Added a tsconfig-style JSON `ConfigText` parser branch in
  `ruzstd/src/encoding/match_generator.rs`.
- Scope:
  - `CompressionFileType::ConfigText`
  - short-line structured JSON
  - content-detected tsconfig-style objects (`compilerOptions` / `paths`)
- Change:
  - keep this family on the wider text no-match probe step `3`
  - instead of the retained package-style structured-JSON dense step `1`
- Added focused matcher tests for:
  - tsconfig-style JSON detection
  - wider probe-step selection for tsconfig-style JSON blocks

Focused result vs the retained `jsonconfig-step1` baseline:
- unchanged:
  - `generated_package.json`: `3,785`
  - `generated_turbo.json`: `3,785`
- improved:
  - `generated_tsconfig.json`: `2,492 -> 2,489`
  - `generated_deno.json`: `2,492 -> 2,489`
  - `generated_nx.json`: `2,492 -> 2,489`

Broad-local result:
- only three fixtures moved:
  - `generated_tsconfig.json`: `2,492 -> 2,489`
  - `generated_deno.json`: `2,492 -> 2,489`
  - `generated_nx.json`: `2,492 -> 2,489`
- refreshed retained baseline:
  - `71` fixtures
  - vs C `zstd -1`: `51 / 16 / 4` better / worse / equal
  - `2,040` total bytes above C on losing fixtures

Largest remaining losses:
- `repo_Cargo.lock`: `9,114` vs `8,088` (`+1,026`)
- `generated_composer.lock`: `4,336` vs `3,766` (`+570`)
- `decodecorpus_z000079`: `7,322` vs `7,221` (`+101`)
- `generated_tsconfig.json`: `2,489` vs `2,391` (`+98`)
- `generated_deno.json`: `2,489` vs `2,391` (`+98`)
- `generated_nx.json`: `2,489` vs `2,391` (`+98`)

Conclusion:
- the JSON-config family really does split:
  - package-style JSON wants dense probing
  - tsconfig-style JSON wants the wider text stride
- this is a real subfamily parser difference, not just more local repeat/window scoring

## 2026-06-01 - Retained composer-style DictionaryText wider probe step

Change notes:
- Added a composer-style `DictionaryText` parser branch in
  `ruzstd/src/encoding/match_generator.rs`.
- Scope:
  - `CompressionFileType::DictionaryText`
  - short-line text
  - content-detected composer-style JSON lockfiles
- Change:
  - keep this family on the wider text no-match probe step `3`
  - instead of the generic non-lockfile `DictionaryText` dense step `1`
- Added focused matcher tests for:
  - composer-style dictionary text detection
  - wider probe-step selection for composer-style blocks

Focused result vs the retained `tsconfig-step3` baseline:
- improved:
  - `generated_composer.lock`: `4,336 -> 4,332`
- unchanged focused controls:
  - `generated_pipfile.lock`: `2,811`
  - `generated_package-lock.json`: `4,392`
  - `generated_go.sum`: `151`
  - `repo_Cargo.lock`: `9,114`

Broad-local result:
- only one fixture moved:
  - `generated_composer.lock`: `4,336 -> 4,332`
- refreshed retained baseline:
  - `71` fixtures
  - vs C `zstd -1`: `51 / 16 / 4` better / worse / equal
  - `2,036` total bytes above C on losing fixtures

Largest remaining losses:
- `repo_Cargo.lock`: `9,114` vs `8,088` (`+1,026`)
- `generated_composer.lock`: `4,332` vs `3,766` (`+566`)
- `decodecorpus_z000079`: `7,322` vs `7,221` (`+101`)
- `generated_tsconfig.json`: `2,489` vs `2,391` (`+98`)
- `generated_deno.json`: `2,489` vs `2,391` (`+98`)
- `generated_nx.json`: `2,489` vs `2,391` (`+98`)

Conclusion:
- composer-style lockfiles did want a real parser-shape change, but it was another probe-step
  split rather than a local repeat/window scoring tweak
- the remaining composer gap is still large, but this confirms the family has useful
  subfamily-specific slack beyond the earlier partition win

## 2026-06-01 - Rejected composer-style DictionaryText probe step 4

Change notes:
- Starting from the retained composer-style `DictionaryText` wider probe-step point,
  tested whether that family wanted an even wider no-match stride:
  - composer-style `DictionaryText` probe step `4` instead of `3`

Focused result:
- regression on the target:
  - `generated_composer.lock`: `4,332 -> 4,336`
- unchanged controls:
  - `generated_pipfile.lock`: `2,811`
  - `generated_package-lock.json`: `4,392`
  - `generated_go.sum`: `151`
  - `repo_Cargo.lock`: `9,114`

Conclusion:
- the composer probe-step family is now bounded:
  - step `3` is the retained best point
  - step `4` is worse

## 2026-06-01 - Retained tsconfig-style JSON ConfigText probe step 4

Change notes:
- Starting from the retained tsconfig-style JSON wider probe-step point, tested whether that
  subfamily wanted an even wider text stride:
  - tsconfig-style JSON `ConfigText` probe step `4` instead of `3`

Focused result vs the retained `composer-step3` baseline:
- unchanged:
  - `generated_package.json`: `3,785`
  - `generated_turbo.json`: `3,785`
- improved:
  - `generated_tsconfig.json`: `2,489 -> 2,486`
  - `generated_deno.json`: `2,489 -> 2,486`
  - `generated_nx.json`: `2,489 -> 2,486`

Broad-local result:
- only three fixtures moved:
  - `generated_tsconfig.json`: `2,489 -> 2,486`
  - `generated_deno.json`: `2,489 -> 2,486`
  - `generated_nx.json`: `2,489 -> 2,486`
- refreshed retained baseline:
  - `71` fixtures
  - vs C `zstd -1`: `51 / 16 / 4` better / worse / equal
  - `2,027` total bytes above C on losing fixtures

Largest remaining losses:
- `repo_Cargo.lock`: `9,114` vs `8,088` (`+1,026`)
- `generated_composer.lock`: `4,332` vs `3,766` (`+566`)
- `decodecorpus_z000079`: `7,322` vs `7,221` (`+101`)
- `generated_tsconfig.json`: `2,486` vs `2,391` (`+95`)
- `generated_deno.json`: `2,486` vs `2,391` (`+95`)
- `generated_nx.json`: `2,486` vs `2,391` (`+95`)

Conclusion:
- tsconfig-style JSON still wanted a slightly wider stride than the earlier retained point
- package-style JSON and tsconfig-style JSON are now bounded on distinct retained parser shapes

## 2026-06-01 - Rejected fastest-only raw composer package-boundary split

Change notes:
- Tested a structural composer-family branch in `ruzstd/src/encoding/frame_compressor.rs`:
  - for fastest-level composer-style `DictionaryText`, split the raw input at package-object
    boundaries before calling `compress_fastest`
- Because the worktree has moved beyond the older retained binary, compared the branch against a
  binary built from the current restored source tree instead of the older retained artifact.

Focused result vs current source baseline:
- regression on the target:
  - `generated_composer.lock`: `4,332 -> 4,352`
- unchanged controls:
  - `generated_pipfile.lock`: `2,811`
  - `generated_package-lock.json`: `4,392`
  - `generated_go.sum`: `151`
  - `repo_Cargo.lock`: `9,114`

Artifacts:
- [branch A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-rawsplit-sourcecmp.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-source-restore.md)

Conclusion:
- composer-style lockfiles do not benefit from a fastest-only raw package-boundary split in this
  form
- the next composer branch should stay away from raw multi-block structural splitting

## 2026-06-01 - Retained tsconfig-style JSON ConfigText probe step 5

Change notes:
- Stayed on the retained tsconfig-style JSON `ConfigText` parser line in
  `ruzstd/src/encoding/match_generator.rs`
- widened the tsconfig-style no-match probe stride from `4` to `5`
- this only affects the `compilerOptions` / `paths` structured-JSON subfamily

Focused result vs the current source baseline:
- unchanged package-style controls:
  - `generated_package.json`: `3,785`
  - `generated_turbo.json`: `3,785`
- improved:
  - `generated_tsconfig.json`: `2,486 -> 2,485`
  - `generated_deno.json`: `2,486 -> 2,485`
  - `generated_nx.json`: `2,486 -> 2,485`

Broad-local result:
- only the tsconfig/deno/nx family moved
- refreshed retained baseline:
  - `71` fixtures
  - vs C `zstd -1`: `51 / 16 / 4` better / worse / equal
  - `2,024` total bytes above C on losing fixtures

Largest remaining losses:
- `repo_Cargo.lock`: `9,114` vs `8,088` (`+1,026`)
- `generated_composer.lock`: `4,332` vs `3,766` (`+566`)
- `decodecorpus_z000079`: `7,322` vs `7,221` (`+101`)
- `generated_tsconfig.json`: `2,485` vs `2,391` (`+94`)
- `generated_deno.json`: `2,485` vs `2,391` (`+94`)
- `generated_nx.json`: `2,485` vs `2,391` (`+94`)

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tsconfig-step5-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-tsconfig-step5-broad-local.md)
- [current broad-local baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-tsconfig-step5-retained)

Conclusion:
- tsconfig-style JSON still has a little stride slack beyond the previous retained point
- package-style JSON, tsconfig-style JSON, and composer-style lockfiles remain distinct parser
  subfamilies rather than one generic JSON/config family

## 2026-06-01 - Rejected lockfile-like DictionaryText probe step 3

Change notes:
- Starting from the retained current source baseline, tested whether the active lockfile parser
  shape wanted a wider no-match stride:
  - lockfile-like `DictionaryText` probe step `3` instead of `2`

Focused result vs current source baseline:
- exact byte-for-byte no-op on the focused lockfile family:
  - `repo_Cargo.lock`: stayed `9,114`
  - `generated_go.sum`: stayed `151`
  - `generated_poetry.lock`: stayed `359`
  - `generated_yarn.lock`: stayed `383`

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-step3-currentsource.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-step3-restore.md)

Conclusion:
- the current lockfile parser still does not want a wider text stride in this family
- lockfile probe-step widening is still closed beyond the retained `step 2` point

## 2026-06-01 - Rejected composer repeat-aware same-start preference

Change notes:
- Tested a composer-family scoring branch in `ruzstd/src/encoding/match_generator.rs`:
  - for composer-style `DictionaryText`, prefer a repeat-offset candidate over a non-repeat
    candidate when both start at the same byte and the repeat loses at most `1` match byte

Focused result vs current source baseline:
- exact byte-for-byte no-op on the focused composer family:
  - `generated_composer.lock`: stayed `4,332`
  - `generated_pipfile.lock`: stayed `2,811`
  - `generated_package-lock.json`: stayed `4,392`
  - `generated_go.sum`: stayed `151`
  - `repo_Cargo.lock`: stayed `9,114`

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-repeataware-focused.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-repeataware-restore.md)

Conclusion:
- the remaining composer gap is not waiting on same-start repeat promotion in this form
- composer repeat-bias work should move away from this local candidate-scoring family

## 2026-06-01 - Rejected DictionaryText OF repeat-table window 1024

Change notes:
- Tested a block-encoder branch in `ruzstd/src/encoding/blocks/compressed.rs`:
  - for fastest-level `DictionaryText`, widen the OF repeat-table reuse window from `64` to
    `1024` sequences while leaving LL/ML behavior unchanged

Focused result vs current source baseline:
- exact byte-for-byte no-op on the focused composer/lockfile family:
  - `generated_composer.lock`: stayed `4,332`
  - `generated_pipfile.lock`: stayed `2,811`
  - `generated_package-lock.json`: stayed `4,392`
  - `generated_go.sum`: stayed `151`
  - `repo_Cargo.lock`: stayed `9,114`

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-ofrepeat-focused.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-ofrepeat-restore.md)

Conclusion:
- the remaining composer and lockfile gaps are not waiting on an OF-only repeat-table reuse window
  in this form

## 2026-06-01 - Rejected pubspec.lock remap to ConfigText

Change notes:
- Tested a known-file-type mapping change in `ruzstd/src/encoding/mod.rs`:
  - remap `pubspec.lock` from `DictionaryText` to `ConfigText`

Focused result vs current source baseline:
- exact byte-for-byte no-op on the focused family:
  - `generated_pubspec.lock`: stayed `233`
  - `generated_pubspec.yaml`: stayed `187`
  - `generated_go.sum`: stayed `151`
  - `generated_poetry.lock`: stayed `359`

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/pubspeclock-config-focused.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/pubspeclock-config-restore.md)

Conclusion:
- `pubspec.lock` is not improved by remapping it from `DictionaryText` to `ConfigText`
- this small known-file mapping family is closed in that direction

## 2026-06-01 - Retained composer repeat-kind preference at same start

Change notes:
- Kept a composer-style `DictionaryText` matcher branch in
  `ruzstd/src/encoding/match_generator.rs`:
  - when two current-position repeat candidates start at the same byte, prefer the composer
    repeat kind that matches the encoder's existing repeat-code order if it loses at most `1`
    match byte

Focused result vs current source baseline:
- `generated_composer.lock`: `4,332 -> 4,160`
- unchanged controls:
  - `generated_pipfile.lock`: `2,811`
  - `generated_package-lock.json`: `4,392`
  - `generated_go.sum`: `151`
  - `repo_Cargo.lock`: `9,114`

Useful matcher evidence:
- `generated_composer.lock`:
  - `total_sequences`: `2676 -> 2673`
  - `repeat_current`: `[946, 518, 739] -> [909, 664, 687]`
  - `repeat_current_zero_literals`: `[0, 438, 602] -> [0, 647, 445]`
  - `window_current_newest[0]`: `209 -> 148`

Broad-local result:
- only `generated_composer.lock` moved
- refreshed retained baseline vs C `zstd -1`:
  - `71` fixtures
  - `51 / 16 / 4` better / worse / equal
  - `2,024 -> 1,852` bytes above C on losing fixtures

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-repeatkind-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-composer-repeatkind-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-composer-repeatkind-retained)

Conclusion:
- the remaining composer gap did want a repeat-family parser-shape change after all
- this is materially different from the earlier rejected same-start repeat-vs-nonrepeat branch

## 2026-06-01 - Retained lockfile repeat-kind preference at same start

Change notes:
- Kept a lockfile-like `DictionaryText` matcher branch in
  `ruzstd/src/encoding/match_generator.rs`:
  - when two current-position repeat candidates start at the same byte, prefer the repeat kind
    that matches the encoder's repeat-code order if it loses at most `1` match byte

Focused result vs current retained baseline:
- `repo_Cargo.lock`: `9,114 -> 9,111`
- unchanged controls:
  - `generated_go.sum`: `151`
  - `generated_poetry.lock`: `359`
  - `generated_yarn.lock`: `383`
  - `generated_composer.lock`: `4,160`

Useful matcher evidence:
- `repo_Cargo.lock`:
  - `repeat_current`: `[65, 24, 10] -> [71, 19, 9]`
  - `repeat_best_before_window`: `[67, 25, 11] -> [73, 20, 10]`
  - `window_current_newest[0]`: stayed `421`
  - `window_current_second_newest[0]`: stayed `105`
  - `window_current_oldest[0]`: stayed `196`

Broad-local result:
- only `repo_Cargo.lock` moved
- refreshed retained baseline vs C `zstd -1`:
  - `71` fixtures
  - `51 / 16 / 4` better / worse / equal
  - `1,852 -> 1,849` bytes above C on losing fixtures

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-repeatkind-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-repeatkind-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-lockfile-repeatkind-retained)

Conclusion:
- the active `Cargo.lock` path still had a small repeat-family win available
- this closes a narrow part of the remaining lockfile gap without moving the rest of the suite

## 2026-06-01 - Rejected lockfile repeat-kind preference match-loss 2

Change notes:
- Tested a follow-up to the retained lockfile repeat-kind branch in
  `ruzstd/src/encoding/match_generator.rs`:
  - widen the allowed match-length loss from `1` to `2`

Focused result vs current retained baseline:
- exact byte-for-byte no-op on the focused lockfile family:
  - `repo_Cargo.lock`: stayed `9,111`
  - `generated_go.sum`: stayed `151`
  - `generated_poetry.lock`: stayed `359`
  - `generated_yarn.lock`: stayed `383`
  - `generated_composer.lock`: stayed `4,160`

Useful matcher evidence:
- `repo_Cargo.lock` matcher diagnostics stayed byte-for-byte identical to the retained
  `match-loss 1` point

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-repeatkind2-focused.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-repeatkind-restore.md)

Conclusion:
- the retained lockfile repeat-kind family is now bounded in this direction
- widening the match-loss budget beyond `1` did not buy any additional parse movement

## 2026-06-01 - Rejected fastest lockfile partition-path retest

Change notes:
- Tested a structural branch in `ruzstd/src/encoding/levels/fastest.rs`:
  - let lockfile-like `DictionaryText` reach the existing fastest-level partition candidate path,
    matching the composer-only path already in the code

Focused result vs current retained baseline:
- exact byte-for-byte no-op on the focused lockfile family:
  - `repo_Cargo.lock`: stayed `9,111`
  - `generated_go.sum`: stayed `151`
  - `generated_poetry.lock`: stayed `359`
  - `generated_yarn.lock`: stayed `383`
  - `generated_composer.lock`: stayed `4,160`

Useful matcher evidence:
- `repo_Cargo.lock` live matcher diagnostics were identical to the retained baseline

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-partitionpath-focused.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-final.md)

Conclusion:
- the current lockfile path is not waiting on the existing fastest partition machinery in this form
- that structural family stays closed on the active retained baseline

## 2026-06-01 - Retained lockfile same-end smaller-offset preference

Change notes:
- Kept a lockfile-like `DictionaryText` matcher branch in
  `ruzstd/src/encoding/match_generator.rs`:
  - when two non-repeat candidates end at the same byte, prefer the smaller-offset candidate if
    it loses at most `1` match byte and saves at least `2` offset-code bits

Focused result vs current retained baseline:
- `repo_Cargo.lock`: `9,111 -> 9,109`
- unchanged controls:
  - `generated_go.sum`: `151`
  - `generated_poetry.lock`: `359`
  - `generated_yarn.lock`: `383`
  - `generated_composer.lock`: `4,160`

Useful matcher evidence:
- `repo_Cargo.lock`:
  - `repeat_current`: `[71, 19, 9] -> [72, 19, 9]`
  - `repeat_best_before_window`: `[73, 20, 10] -> [74, 20, 10]`
  - `window_current_newest[0]`: `421 -> 422`
  - `window_current_second_newest[0]`: `105 -> 103`

Broad-local result:
- only `repo_Cargo.lock` moved
- refreshed retained baseline vs C `zstd -1`:
  - `71` fixtures
  - `51 / 16 / 4` better / worse / equal
  - `1,849 -> 1,847` bytes above C on losing fixtures

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-sameend-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-sameend-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-lockfile-sameend-retained)

Conclusion:
- the active `Cargo.lock` path still had a small same-end parse-shape win available
- this is another clean reduction of the dominant remaining known-file-type gap

## 2026-06-01 - Rejected lockfile same-end smaller-offset match-loss 2

Change notes:
- Tested a follow-up to the retained lockfile same-end branch in
  `ruzstd/src/encoding/match_generator.rs`:
  - widen the allowed same-end match-length loss from `1` to `2`

Focused result vs current retained baseline:
- exact byte-for-byte no-op on the focused lockfile family:
  - `repo_Cargo.lock`: stayed `9,109`
  - `generated_go.sum`: stayed `151`
  - `generated_poetry.lock`: stayed `359`
  - `generated_yarn.lock`: stayed `383`
  - `generated_composer.lock`: stayed `4,160`

Useful matcher evidence:
- `repo_Cargo.lock` live matcher diagnostics stayed byte-for-byte identical to the retained
  `match-loss 1` point

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-sameend2-focused.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-sameend-restore.md)

Conclusion:
- the retained lockfile same-end family is bounded in this direction
- widening the same-end match-loss budget beyond `1` did not buy any additional parse movement

## 2026-06-01 - Rejected lockfile OF table max-log 6

Change notes:
- Tested a lockfile-specific encoder branch across the fastest-level whole-block and partition
  paths:
  - when the block is lockfile-like `DictionaryText`, lower OF table max-log from `7` to `6`

Focused result vs current retained baseline:
- hard regression on the target:
  - `repo_Cargo.lock`: `9,109 -> 9,145`
- unchanged controls:
  - `generated_go.sum`: `151`
  - `generated_poetry.lock`: `359`
  - `generated_yarn.lock`: `383`
  - `generated_composer.lock`: `4,160`

Useful matcher evidence:
- `repo_Cargo.lock` live matcher diagnostics stayed byte-for-byte identical to the retained point
- so this branch was pure entropy-side damage, not a parser change

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-oflog6-focused.md)
- [restore check](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-oflog6-restore2.md)

Conclusion:
- the active `Cargo.lock` path does not want a smaller OF table max-log in this form
- this closes another encoder-side OF-table family

## 2026-06-01 - Added focused matcher tuner and retained tsconfig step 6

Change notes:
- Added a focused tuning harness:
  - [tools/tune_matcher_family.py](/home/bsutton/git/zstd-rs/tools/tune_matcher_family.py)
- Added runtime-tunable matcher overrides under `feature = "std"` in
  [ruzstd/src/encoding/match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs)
  for:
  - lockfile probe step
  - composer probe step
  - structured-JSON probe step
  - tsconfig-style JSON probe step
  - dictionary same-start smaller-offset thresholds
  - lockfile same-end smaller-offset thresholds
  - lockfile/composer repeat-kind match-loss thresholds

Focused tuner results:
- `cargo-lock` family:
  - baseline total bytes: `10,002`
  - best searched candidates matched baseline exactly
  - useful conclusion:
    - the current retained `Cargo.lock` local knob surface is already at its best searched point
- `composer` family:
  - baseline total bytes: `11,514`
  - best searched candidates matched baseline exactly
  - useful conclusion:
    - the current retained composer local knob surface is already at its best searched point
- `structured-json` family:
  - baseline total bytes: `7,570`
  - best searched candidate stayed at probe step `1`
- `tsconfig-json` family:
  - baseline total bytes: `7,455`
  - probe step `6` improved to `7,452`

Retained change:
- raised `TSCONFIG_JSON_TEXT_NO_MATCH_PROBE_STEP` from `5` to `6`

Focused result vs current retained baseline:
- `generated_tsconfig.json`: `2,485 -> 2,484`
- `generated_deno.json`: `2,485 -> 2,484`
- `generated_nx.json`: `2,485 -> 2,484`
- package-style JSON stayed unchanged:
  - `generated_package.json`: `3,785`
  - `generated_turbo.json`: `3,785`

Broad-local result:
- only the tsconfig-style JSON family moved
- refreshed retained baseline:
  - `71` fixtures
  - `51 / 16 / 4` better / worse / equal vs C
  - bytes above C on losers: `1,847 -> 1,844`

Artifacts:
- tuner reports:
  - [cargo-lock](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-cargo-lock.md)
  - [composer](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-composer.md)
  - [structured-json](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-structured-json.md)
  - [tsconfig-json](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-tsconfig-json.md)
- broad-local A/B:
  - [compare report](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-tsconfig-step6-broad-local.md)
- retained binary:
  - [ruzstd-cli-level1-tsconfig-step6-retained](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-tsconfig-step6-retained)

Conclusion:
- the tuner is now in place for focused family sweeps
- the current local knob surfaces for `Cargo.lock` and composer are exhausted at the searched
  points
- tsconfig-style JSON still had one clean retained step left at probe step `6`

## 2026-06-01 - Fixed tuner race and retained dependency-JSON-lockfile encoder config

Change notes:
- Fixed a focused-tuner reliability bug in
  [tools/tune_matcher_family.py](/home/bsutton/git/zstd-rs/tools/tune_matcher_family.py):
  - concurrent family sweeps were reusing the same temp output filenames in `benchmarks/tmp`
  - candidate outputs now include a stable hash of the env settings
- Added an encoder-side content detector in
  [ruzstd/src/encoding/util.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/util.rs):
  - `likely_dependency_json_lockfile_text()`
  - detects `package-lock.json` / `Pipfile.lock`-style large JSON lockfiles
- Added a scoped encoder config branch in
  [ruzstd/src/encoding/blocks/compressed.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/blocks/compressed.rs):
  - dependency-JSON lockfiles now use:
    - `HuffmanTableSearch::AllSections`
    - `repeat_table_max_sequences = 256`
    - `offset_table_max_log = 8`

Focused encoder evidence after fixing the tuner:
- `cargo-lock-encoder` searched surface:
  - best searched total: `10,001` vs baseline `10,002`
  - useful conclusion:
    - no meaningful retained encoder-side lockfile win on the active broad surface
- `composer-encoder` searched surface:
  - once the tuner race was fixed, the best searched totals were really coming from:
    - `generated_package-lock.json`
    - `generated_pipfile.lock`
  - not from `generated_composer.lock`

Retained focused result:
- `generated_package-lock.json`: `4,392 -> 4,388`
- `generated_pipfile.lock`: `2,811 -> 2,804`
- unchanged controls:
  - `generated_composer.lock`: `4,160`
  - `generated_go.sum`: `151`
  - `repo_Cargo.lock`: `9,109`

Broad-local result:
- only two fixtures moved:
  - `generated_package-lock.json`: `4,392 -> 4,388`
  - `generated_pipfile.lock`: `2,811 -> 2,804`
- refreshed retained baseline vs C stayed:
  - `71` fixtures
  - `51 / 16 / 4` better / worse / equal
  - `1,844` bytes above C on losing fixtures

Artifacts:
- focused tuner reports:
  - [cargo-lock-encoder](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-cargo-lock-encoder.md)
  - [composer-encoder](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-composer-encoder.md)
  - [tsconfig-json-encoder](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-tsconfig-json-encoder.md)
- broad-local A/B:
  - [compare report](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dependency-json-lockfile-encoder-broad-local.md)
- retained binary:
  - [ruzstd-cli-level1-dependency-json-lockfile-encoder-retained](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-dependency-json-lockfile-encoder-retained)

Conclusion:
- the tuner is now trustworthy for concurrent sweeps
- dependency-JSON lockfiles are a real encoder subfamily with a clean retained config win
- this improves known-file handling without reopening the already-exhausted `Cargo.lock` local
  matcher surface

## 2026-06-01 - Retained whole-file dependency-JSON profile

Change notes:
- Added an internal whole-file profile in
  [ruzstd/src/encoding/mod.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/mod.rs)
  and threaded it through:
  - [ruzstd/src/encoding/frame_compressor.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/frame_compressor.rs)
  - [ruzstd/src/encoding/levels/fastest.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/levels/fastest.rs)
  - [ruzstd/src/encoding/blocks/compressed.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/blocks/compressed.rs)
- The public `CompressionFileType` API stays unchanged.
- `package-lock.json` / `Pipfile.lock` style files now carry the dependency-JSON encoder profile
  across every block instead of relying on a block-local content guess.

Retained focused result vs the prior retained baseline:
- `generated_package-lock.json`: `4,388 -> 4,383`
- unchanged controls:
  - `generated_pipfile.lock = 2,804`
  - `generated_composer.lock = 4,160`
  - `generated_go.sum = 151`
  - `repo_Cargo.lock = 9,109`

Broad-local result:
- only one fixture moved:
  - `generated_package-lock.json`: `4,388 -> 4,383`
- refreshed retained baseline vs C stayed:
  - `71` fixtures
  - `51 / 16 / 4` better / worse / equal
  - `1,844` bytes above C on losing fixtures

Artifacts:
- focused report:
  - [dependency-json profile focused](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/dependency-json-profile-focused.md)
- broad-local A/B:
  - [dependency-json profile broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-dependency-json-profile-broad-local.md)
- refreshed current baseline:
  - [current broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- retained binary:
  - [ruzstd-cli-level1-dependency-json-profile-retained](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-dependency-json-profile-retained)

Conclusion:
- the missing `package-lock.json` tuner win was a whole-file hinting problem, not a new local
  matcher or entropy threshold
- dependency-JSON lockfiles now use the intended encoder profile on every block

## 2026-06-01 - Retained composer probe step 5

Change notes:
- In
  [ruzstd/src/encoding/match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs),
  composer-style `DictionaryText` now defaults to no-match probe step `5` instead of the generic
  text step `3`.
- This came from extending the existing runtime tuner surface beyond the earlier searched
  `3/4` composer points.

Retained focused result vs prior retained baseline:
- `generated_composer.lock`: `4,160 -> 4,159`
- unchanged controls:
  - `generated_pipfile.lock = 2,804`
  - `generated_package-lock.json = 4,383`
  - `generated_go.sum = 151`
  - `repo_Cargo.lock = 9,109`

Broad-local result:
- only one fixture moved:
  - `generated_composer.lock`: `4,160 -> 4,159`
- refreshed retained baseline vs C:
  - `71` fixtures
  - `51 / 16 / 4` better / worse / equal
  - `1,843` bytes above C on losing fixtures

Closed this turn:
- single-segment frame-header branch: reject
  - broad-local regressed many fixtures
- composer whole-vs-split comparison for text blocks: exact no-op
- tsconfig probe steps `7` and `8`: no-op
- composer probe step `6`: regression to `4,171`

Artifacts:
- focused report:
  - [composer step5 focused](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-step5-current-focused.md)
- broad-local A/B:
  - [composer step5 broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-composer-step5-broad-local.md)
- refreshed current baseline:
  - [current broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- retained binary:
  - [ruzstd-cli-level1-composer-step5-retained](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-composer-step5-retained)

Conclusion:
- composer’s local probe-step family was not actually exhausted at `3/4`
- the new retained best point is `5`
- the next best target is still `repo_Cargo.lock`

## 2026-06-01 - Bounded composer partition-cap family

Change notes:
- Added a runtime-tunable composer partition-budget override in
  [ruzstd/src/encoding/levels/fastest.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/levels/fastest.rs):
  - `RUZSTD_TUNE_COMPOSER_MAX_PARTITIONS`
- Added a matching tuner preset in
  [tools/tune_matcher_family.py](/home/bsutton/git/zstd-rs/tools/tune_matcher_family.py):
  - `composer-partitions`

Focused sweep result against the current retained baseline:
- `generated_composer.lock`
  - cap `1`: `4,255`
  - cap `2`: `4,194`
  - caps `3..8`: all `4,159`
- unchanged focused controls at all non-regressing best points:
  - `generated_pipfile.lock = 2,804`
  - `generated_package-lock.json = 4,383`
  - `generated_go.sum = 151`
  - `repo_Cargo.lock = 9,109`

Default-path verification:
- with the override unset, focused outputs are byte-identical to the retained baseline
- report:
  - [composer cap default](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-cap-default-focused.md)

Closed this turn:
- composer partition-cap family is now bounded:
  - retained best region is already reached at cap `3`
  - larger caps do not help beyond the current retained `step 5` point
  - smaller caps regress

Conclusion:
- the remaining composer gap is not waiting on partition-count tuning
- next highest-value work returns to `repo_Cargo.lock`

## 2026-06-01 - Expanded Cargo.lock tuner surface still flat on Cargo.lock

Change notes:
- Expanded the `cargo-lock` and `cargo-lock-combined` presets in
  [tools/tune_matcher_family.py](/home/bsutton/git/zstd-rs/tools/tune_matcher_family.py) to
  cover the previously unsearched lower lockfile offset-bias settings:
  - `RUZSTD_TUNE_LOCKFILE_REPEAT_KIND_MATCH_LOSS_MAX = 0/1/2`
  - `RUZSTD_TUNE_LOCKFILE_SAME_END_MATCH_LOSS_MAX = 0/1/2`
  - `RUZSTD_TUNE_LOCKFILE_SAME_END_BITS_GAIN_MIN = 1/2/3`

Focused sweep results on the current retained source baseline:
- `cargo-lock` matcher-only sweep:
  - baseline focused family total: `10,002`
  - best searched candidate: `10,002`
  - no candidate beat the current retained local surface
- `cargo-lock-combined` matcher+encoder sweep:
  - baseline focused family total: `10,002`
  - best searched candidate: `10,001`
  - the `-1` did **not** move `repo_Cargo.lock`
  - it only moved:
    - `generated_poetry.lock`: `359 -> 358`
  - focused controls stayed unchanged:
    - `repo_Cargo.lock = 9,109`
    - `generated_go.sum = 151`
    - `generated_yarn.lock = 383`

Follow-up isolation:
- The only winning lever in that combined candidate was:
  - `RUZSTD_TUNE_OFFSET_PREDEFINED_MAX_SEQUENCES = 64`
- Broad-local spot check with only that encoder override was not clean:
  - wins:
    - `generated_poetry.lock`: `359 -> 358`
    - `generated_pubspec.lock`: `233 -> 230`
    - `dict_quotaon.service`: `412 -> 411`
  - but multiple regressions elsewhere, including:
    - `repo_ci.yml`: `556 -> 562`
    - `repo_ruzstd_Cargo.toml`: `730 -> 734`
    - `repo_cli_Cargo.toml`: `489 -> 492`
    - `dict_systemd-journal-gatewayd.service`: `622 -> 627`

Result:
- no new retained runtime compression change
- retained baseline stays:
  - `71` fixtures
  - `51 / 16 / 4` better / worse / equal vs C
  - `1,843` bytes above C on losing fixtures

Artifacts:
- [expanded cargo-lock matcher sweep](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-cargo-lock-expanded.md)
- [expanded cargo-lock combined sweep](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-cargo-lock-combined-expanded.md)

Conclusion:
- the current `Cargo.lock` local threshold surface is flatter than the earlier search suggested
- lower same-end / repeat-kind thresholds do not create real `repo_Cargo.lock` movement
- the only new searched `-1` is a side win on `generated_poetry.lock`, and it is not broad-local
  safe in its searched form
- the next credible `Cargo.lock` branch should move away from local threshold tuning and toward a
  different literal/sequence representation

## 2026-06-01 - New Cargo.lock literal encoder surface is flat; zero-literal second-newest is parse-only

Change notes:
- Added a new focused tuner preset in
  [tools/tune_matcher_family.py](/home/bsutton/git/zstd-rs/tools/tune_matcher_family.py):
  - `cargo-lock-literal-encoder`
- Searched the previously untested encoder surface for the lockfile family:
  - `RUZSTD_TUNE_HUFFMAN_TABLE_SEARCH = filetype/heuristic/allsections`
  - `RUZSTD_TUNE_FILE_TYPE_SINGLE_STREAM_HUFFMAN_MAX_LITERALS = none/1024/2048/4096/8192/16384`
  - `RUZSTD_TUNE_FILE_TYPE_SMALL_SEQUENCE_PREDEFINED_LLML_MAX_SEQUENCES = none/64/128/256/512`

Focused result:
- baseline focused family total: `10,002`
- best searched candidate: `10,002`
- no candidate beat the retained lockfile family baseline

I also added a tune-only structural matcher gate in
[ruzstd/src/encoding/match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs):
- `RUZSTD_TUNE_LOCKFILE_SECOND_NEWEST_ZERO_LITERALS`
- when set false, lockfile-like `DictionaryText` skips zero-literal `second_newest` probes

Focused byte result with the gate disabled:
- `repo_Cargo.lock = 9,109`
- `generated_go.sum = 151`
- `generated_poetry.lock = 359`
- `generated_yarn.lock = 383`
- exact no-op on bytes

But live matcher diagnostics changed substantially on `repo_Cargo.lock`:
- sequences: `821 -> 842`
- `window_current_second_newest_zero_literals: 66 -> 1`
- `window_current_second_newest: 103 -> 38`
- `window_current_newest: 422 -> 476`
- `window_current_oldest: 196 -> 225`

Interpretation:
- the zero-literal `second_newest` family is real parse movement, but it is byte-neutral under the
  current encoder surface
- the newly searched lockfile literal encoder surface is flat in its current form

Result:
- no new retained runtime compression change
- retained baseline stays:
  - `71` fixtures
  - `51 / 16 / 4` better / worse / equal vs C
  - `1,843` bytes above C on losing fixtures

Artifacts:
- [lockfile literal-encoder sweep](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-cargo-lock-literal-encoder.md)

Conclusion:
- the next credible `Cargo.lock` branch is narrower again:
  - not the searched literal-encoder knob surface
  - not simple zero-literal `second_newest` suppression by itself
- future lockfile work should target a broader sequence/literal representation change rather than
  another local encoder or source-order toggle

## 2026-06-01 - Composer zero-literal repeat-kind scope is flat

Change notes:
- Added a new focused tuner preset in
  [tools/tune_matcher_family.py](/home/bsutton/git/zstd-rs/tools/tune_matcher_family.py):
  - `composer-repeat-zero-literals`
- Added a tune-only matcher override in
  [ruzstd/src/encoding/match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs):
  - `RUZSTD_TUNE_COMPOSER_REPEAT_KIND_ZERO_LITERALS_ONLY`

Why this family:
- live fastest-level matcher diagnostics on `generated_composer.lock` are overwhelmingly repeat-side:
  - `repeat_current = [909, 663, 687]`
  - `repeat_current_zero_literals = [0, 647, 445]`
  - current-window wins are relatively small:
    - `window_current_newest = [147, 26, ...]`
    - `window_current_oldest = [229, 11, ...]`
- so the next credible composer branch was to scope the retained repeat-kind preference to the
  dominant zero-literal subfamily

Focused sweep result:
- baseline focused family total: `11,497`
- best searched candidate: `11,497`
- searched surface:
  - `RUZSTD_TUNE_COMPOSER_REPEAT_KIND_MATCH_LOSS_MAX = 0/1/2`
  - `RUZSTD_TUNE_COMPOSER_REPEAT_KIND_ZERO_LITERALS_ONLY = 0/1`

Result:
- no new retained runtime compression change
- scoping the composer repeat-kind rule to zero-literal repeats does not improve the focused family

Artifacts:
- [composer zero-literal repeat-kind sweep](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-composer-repeat-zero-literals.md)

Conclusion:
- the dominant composer repeat-side pattern is real, but this zero-literal-only scoping is flat
- the next credible composer branch should move away from repeat-kind scope toggles and toward a
  different sequence or block/entropy representation

## 2026-06-01 - Strong lockfile zero-literal window suppression is parse-only

Change notes:
- Added a tune-only matcher override in
  [ruzstd/src/encoding/match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs):
  - `RUZSTD_TUNE_LOCKFILE_ZERO_LITERAL_WINDOW_DISABLE`
- This is a stronger structural version of the earlier lockfile zero-literal window family:
  - reject all zero-literal non-repeat window candidates for lockfile-like `DictionaryText`

Focused byte result:
- exact no-op on the focused lockfile family:
  - `repo_Cargo.lock = 9,109`
  - `generated_go.sum = 151`
  - `generated_poetry.lock = 359`
  - `generated_yarn.lock = 383`

But live matcher diagnostics on `repo_Cargo.lock` changed materially:
- sequences: `821 -> 760`
- `window_current_newest: 422 -> 396`
- `window_current_second_newest_zero_literals: 66 -> 47`
- `window_current_oldest: 196 -> 151`
- repeat-side counts rose correspondingly

Combined follow-up:
- paired this structural matcher branch with the earlier best nearby lockfile encoder settings:
  - still no `repo_Cargo.lock` movement
  - the only focused `-1` remained `generated_poetry.lock: 359 -> 358`

Conclusion:
- this stronger zero-literal window family is a real parse-shape lever
- but, like the weaker lockfile zero-literal branches, it is not a compressed-size win under the
  current encoder surface
- the whole zero-literal window suppression family should be considered bounded for `Cargo.lock`

## 2026-06-01 - Composer non-repeat window suppression is flat

Change notes:
- Added a tune-only matcher override in
  [ruzstd/src/encoding/match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs):
  - `RUZSTD_TUNE_COMPOSER_WINDOW_DISABLE`
- Added a focused reproducible preset in
  [tools/tune_matcher_family.py](/home/bsutton/git/zstd-rs/tools/tune_matcher_family.py):
  - `composer-window-disable`

Why this family:
- live fastest-level composer diagnostics showed the family was overwhelmingly repeat-driven, so a
  coarse “disable all non-repeat window candidates” probe was a reasonable structural test

Focused result:
- baseline focused family total: `11,497`
- best searched candidate: `11,497`
- no searched combination moved:
  - `generated_composer.lock`
  - `generated_pipfile.lock`
  - `generated_package-lock.json`
  - `generated_go.sum`

Even the parse-heavy version was byte-flat:
- with `RUZSTD_TUNE_COMPOSER_WINDOW_DISABLE=1`, live matcher diagnostics on
  `generated_composer.lock` shifted heavily toward repeat candidates and sequence count changed
  substantially, but compressed bytes did not move

Artifacts:
- [composer window-disable sweep](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-composer-window-disable.md)

Conclusion:
- the coarse composer window-suppression family is flat
- the remaining composer gap is not waiting on simply removing non-repeat window candidates

## 2026-06-01 - Composer zero-literal repeat-candidate limit is flat

Change notes:
- Added a tune-only matcher override in
  [ruzstd/src/encoding/match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs):
  - `RUZSTD_TUNE_COMPOSER_ZERO_LITERAL_REPEAT_CANDIDATE_LIMIT`
- Added a focused preset in
  [tools/tune_matcher_family.py](/home/bsutton/git/zstd-rs/tools/tune_matcher_family.py):
  - `composer-zero-literal-repeat-limit`

Why this family:
- composer zero-literal repeat candidates are probed in a fixed order:
  - `second`, `third`, `first-1`
- live composer diagnostics show zero-literal repeat traffic is concentrated there, so limiting how
  many of those candidate kinds are even considered was the next clean structural probe

Focused sweep result:
- baseline focused family total: `11,497`
- best searched candidate: `11,497`
- searched surface:
  - `RUZSTD_TUNE_COMPOSER_ZERO_LITERAL_REPEAT_CANDIDATE_LIMIT = 1/2/3`
  - `RUZSTD_TUNE_COMPOSER_REPEAT_KIND_MATCH_LOSS_MAX = 0/1/2`

Result:
- no new retained runtime compression change
- limiting composer zero-literal repeat kinds does not improve the focused family

Artifacts:
- [composer zero-literal repeat-limit sweep](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-composer-zero-literal-repeat-limit.md)

Conclusion:
- the composer zero-literal repeat-candidate-limit family is now bounded
- the remaining composer gap is not waiting on simple repeat-kind suppression or repeat-kind scope
  changes

## 2026-06-01 - Lockfile fastest split/whole-compare family is flat

Change notes:
- Added tune-only fastest-path overrides in
  [ruzstd/src/encoding/levels/fastest.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/levels/fastest.rs):
  - `RUZSTD_TUNE_LOCKFILE_FASTEST_SPLITS`
  - `RUZSTD_TUNE_LOCKFILE_COMPARE_WHOLE_TEXT`
- Added a focused preset in
  [tools/tune_matcher_family.py](/home/bsutton/git/zstd-rs/tools/tune_matcher_family.py):
  - `cargo-lock-splits`

Why this family:
- the fastest path already has a composer-only structural branch that reaches the split-search path
- lockfile text was still excluded from whole-vs-partition comparison by the generic `likely_text`
  gate, so this was the cleanest remaining structural block-representation probe in the live code
  path

Focused sweep result:
- baseline focused family total: `10,002`
- every searched combination stayed `10,002`
- searched surface:
  - `RUZSTD_TUNE_LOCKFILE_FASTEST_SPLITS = 0/1`
  - `RUZSTD_TUNE_LOCKFILE_COMPARE_WHOLE_TEXT = 0/1`

Result:
- no new retained runtime compression change
- the lockfile fastest split / whole-compare family is flat in the current searched form

Artifacts:
- [lockfile splits sweep](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-cargo-lock-splits.md)

Conclusion:
- the lockfile structural split/whole-compare family is now bounded
- the remaining `Cargo.lock` gap is not waiting on simply letting the fastest path reuse the
  current split machinery for text blocks

## 2026-06-01 - Lockfile post-parse zero-literal match dropping is flat

Change notes:
- Added tune-only post-`prepare_block()` overrides in
  [ruzstd/src/encoding/blocks/compressed.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/blocks/compressed.rs):
  - `RUZSTD_TUNE_LOCKFILE_DROP_ZERO_LITERAL_MATCH_MAX_LEN`
  - `RUZSTD_TUNE_LOCKFILE_DROP_ZERO_LITERAL_MATCH_MIN_OF_CODE`
- Added a focused preset in
  [tools/tune_matcher_family.py](/home/bsutton/git/zstd-rs/tools/tune_matcher_family.py):
  - `cargo-lock-drop-zero-literal-match`

Why this family:
- current `Cargo.lock` still has more sequences than C while also leaving a much worse literal
  stream
- this branch was the first direct post-match representation probe:
  - downgrade selected zero-literal lockfile matches into literals before entropy coding

Focused sweep result:
- baseline focused family total: `10,002`
- best searched candidate: `10,002`
- searched surface:
  - `RUZSTD_TUNE_LOCKFILE_DROP_ZERO_LITERAL_MATCH_MAX_LEN = 5/6/7/8`
  - `RUZSTD_TUNE_LOCKFILE_DROP_ZERO_LITERAL_MATCH_MIN_OF_CODE = 7/8/9/10`

Aggressive spot-check:
- `max_len=8`, `min_of_code=7`
- archive inspection on `repo_Cargo.lock` was byte-for-byte identical to the retained baseline:
  - `compressed_bytes=9,109`
  - `literals_payload=6,888`
  - `sequence_count=821`
  - `sequence_payload=2,201`
  - `of_extra_bits=6,876`

Result:
- no new retained runtime compression change
- this post-parse zero-literal match-dropping family did not trigger a useful representation shift
  on the live `Cargo.lock` data

Artifacts:
- [lockfile drop-zero-literal-match sweep](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-cargo-lock-drop-zero-literal-match.md)

Conclusion:
- the direct post-parse zero-literal match-dropping family is bounded for `Cargo.lock`
- the next credible `Cargo.lock` branch has to be a different representation change again, not a
  nearby short-match downgrade rule

## 2026-06-01 - Corrected: lockfile post-parse zero-literal match-dropping branch was invalid

Correction to the prior note above:
- after rebuilding `target/release/ruzstd-cli`, the post-`prepare_block()` lockfile
  zero-literal-match-dropping branch was **not** merely flat
- it was invalid

What changed:
- the experimental tune-only overrides in `prepare_block()` were causing later repeat-offset
  history to diverge from the matcher-produced raw offsets
- focused sweep with the rebuilt binary then failed decode verification:
  - `tools/tune_matcher_family.py` raised `decoded output did not match original`

Aggressive rebuilt spot-check:
- `RUZSTD_TUNE_LOCKFILE_DROP_ZERO_LITERAL_MATCH_MAX_LEN=255`
- `RUZSTD_TUNE_LOCKFILE_DROP_ZERO_LITERAL_MATCH_MIN_OF_CODE=0`
- focused bytes regressed heavily before decode validation:
  - `repo_Cargo.lock`: `9,109 -> 14,639`

Resolution:
- removed the experimental tune-only branch from:
  - [ruzstd/src/encoding/blocks/compressed.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/blocks/compressed.rs)
  - [tools/tune_matcher_family.py](/home/bsutton/git/zstd-rs/tools/tune_matcher_family.py)
- do not treat this as a viable representation family in its current form

Conclusion:
- direct post-parse sequence dropping is unsafe unless repeat-offset history and later repeat-coded
  sequences are recomputed consistently
- this branch should be considered rejected as invalid, not retained and not merely “flat”

## 2026-06-01 - Lockfile sequence-cost tuner family regressed everywhere

Change notes:
- Added a tune-only lockfile candidate-scoring branch in
  [ruzstd/src/encoding/match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs):
  - `RUZSTD_TUNE_LOCKFILE_SEQUENCE_COST_LITERAL_WEIGHT`
  - `RUZSTD_TUNE_LOCKFILE_SEQUENCE_COST_OFFSET_WEIGHT`
  - `RUZSTD_TUNE_LOCKFILE_SEQUENCE_COST_MARGIN`
- Added a focused preset in
  [tools/tune_matcher_family.py](/home/bsutton/git/zstd-rs/tools/tune_matcher_family.py):
  - `cargo-lock-sequence-cost`

Why this family:
- after exhausting the nearby same-end, repeat-kind, and offset-threshold surfaces, the next local
  probe was a broader tune-only candidate score:
  - prefer same-start or same-end lockfile candidates with a lower estimated literal-plus-offset
    cost

Focused sweep result:
- baseline focused family total: `10,002`
- best searched candidate: `10,054`
- searched surface:
  - `RUZSTD_TUNE_LOCKFILE_SEQUENCE_COST_LITERAL_WEIGHT = 1/2/3/4`
  - `RUZSTD_TUNE_LOCKFILE_SEQUENCE_COST_OFFSET_WEIGHT = 1/2/3/4`
  - `RUZSTD_TUNE_LOCKFILE_SEQUENCE_COST_MARGIN = 0/1/2`

Resolution:
- removed the tune-only scorer branch from:
  - [ruzstd/src/encoding/match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs)
  - [tools/tune_matcher_family.py](/home/bsutton/git/zstd-rs/tools/tune_matcher_family.py)
- restore against the retained baseline is exact:
  - [cargolock-sequence-cost-restore.md](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/cargolock-sequence-cost-restore.md)

Conclusion:
- the lockfile sequence-cost tuner family is bounded and actively harmful in the searched space
- `Cargo.lock` still needs a broader literal/sequence representation change, not another local
  candidate-scoring rule

## 2026-06-01 - Retained Cargo.lock zero-literal next-position lazy parse

Change notes:
- Added a broader `Cargo.lock` parser branch in
  [ruzstd/src/encoding/match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs):
  - on lockfile-like `DictionaryText`
  - at zero literals only
  - compare the current candidate against the best candidate at `ip+1`
  - score that one-step lazy-parse choice using a local literal/match/offset cost model
- Added a focused sweep preset in
  [tools/tune_matcher_family.py](/home/bsutton/git/zstd-rs/tools/tune_matcher_family.py):
  - `cargo-lock-next-position`

Focused sweep result:
- baseline focused family total: `10,002`
- best searched candidate: `9,999`
- retained tuned point:
  - `max_current_match_len=7`
  - `literal_weight=6`
  - `match_reward=2`
  - `offset_weight=1`
  - `margin=1`

Focused A/B:
- `repo_Cargo.lock`: `9,109 -> 9,106`
- unchanged controls:
  - `generated_go.sum = 151`
  - `generated_poetry.lock = 359`
  - `generated_yarn.lock = 383`

Live `Cargo.lock` archive signal at the retained point:
- `sequence_count`: `821 -> 817`
- `sequence_payload_bytes`: `2208 -> 2195`
- `of_extra_bits`: `6898 -> 6830`
- `decoded_literals`: `9932 -> 9938`
- `literal_section_bytes`: `6886 -> 6891`

Broad-local result:
- only `repo_Cargo.lock` moved
- current retained baseline vs C `zstd -1`:
  - `71` fixtures
  - `51 / 16 / 4` better / worse / equal
  - `1,839` bytes above C on the losing fixtures

Current biggest losses:
- `repo_Cargo.lock`: `9,106` vs `8,088` = `+1,018`
- `generated_composer.lock`: `4,159` vs `3,766` = `+393`
- `decodecorpus_z000079`: `7,322` vs `7,221` = `+101`

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-next-position-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-next-position-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-lockfile-next-position-retained)

Conclusion:
- this is the first retained `Cargo.lock` move from a genuine one-step lazy-parse representation
  branch rather than another same-position threshold
- it cuts a few more bytes by removing four sequences and lowering offset payload

## 2026-06-01 - Tiny-literal extension of Cargo.lock lazy parse is flat

Change notes:
- Extended the retained lockfile one-step lazy-parse family with a tune-only tiny-literal reach:
  - allow the same `ip` vs `ip+1` lockfile candidate comparison beyond strict zero-literal
    positions
- Added and then removed a focused preset in
  [tools/tune_matcher_family.py](/home/bsutton/git/zstd-rs/tools/tune_matcher_family.py):
  - `cargo-lock-next-position-literals`

Focused sweep result:
- baseline focused family total: `9,999`
- best searched candidate: `9,999`
- searched surface included:
  - `max_literal_len = 0/1/2/3`
  - nearby current-match and local cost weights around the retained zero-literal point

Resolution:
- removed the tune-only tiny-literal extension and preset
- restore against the retained baseline is exact:
  - [lockfile-next-position-restore.md](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-next-position-restore.md)

Conclusion:
- the productive lockfile lazy-parse family is currently bounded at strict zero literals
- widening it to tiny literal runs did not improve the focused lockfile family

## 2026-06-01 - Retained Cargo.lock two-byte lazy-parse skip

Change notes:
- Extended the retained lockfile lazy-parse family in
  [ruzstd/src/encoding/match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs):
  - still only on zero-literal lockfile positions
  - but now compare the current candidate against the best candidate after skipping up to `2`
    literal bytes
- Added a focused sweep preset in
  [tools/tune_matcher_family.py](/home/bsutton/git/zstd-rs/tools/tune_matcher_family.py):
  - `cargo-lock-next-position-skip`

Focused sweep result:
- baseline focused family total: `9,999`
- best searched candidate: `9,998`
- retained tuned point:
  - `max_skip_literals=2`
  - `max_current_match_len=7`
  - `literal_weight=6`
  - `match_reward=2`
  - `offset_weight=2`
  - `margin=1`

Focused A/B:
- `repo_Cargo.lock`: `9,106 -> 9,105`
- unchanged controls:
  - `generated_go.sum = 151`
  - `generated_poetry.lock = 359`
  - `generated_yarn.lock = 383`

Live `Cargo.lock` archive signal at the retained point:
- `sequence_count`: `817 -> 810`
- `sequence_payload_bytes`: `2195 -> 2184`
- `of_extra_bits`: `6830 -> 6777`
- `decoded_literals`: `9938 -> 9952`
- `literal_section_bytes`: `6891 -> 6901`

Broad-local result:
- only `repo_Cargo.lock` moved
- current retained baseline vs C `zstd -1`:
  - `71` fixtures
  - `51 / 16 / 4` better / worse / equal
  - `1,838` bytes above C on the losing fixtures

Current biggest losses:
- `repo_Cargo.lock`: `9,105` vs `8,088` = `+1,017`
- `generated_composer.lock`: `4,159` vs `3,766` = `+393`
- `decodecorpus_z000079`: `7,322` vs `7,221` = `+101`

Artifacts:
- [focused sweep](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-cargo-lock-next-position-skip.md)
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-next-position-skip-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-lockfile-next-position-skip-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-lockfile-next-position-skip-retained)

Conclusion:
- the lockfile lazy-parse family still has small productive headroom beyond the one-byte skip point
- this retained step removes seven more sequences and lowers offset payload again

## 2026-06-01 - Retained whole-file ComposerLock profile

Change notes:
- Added an internal `CompressionFileProfile::ComposerLock` in
  [ruzstd/src/encoding/mod.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/mod.rs)
- Profiled `composer.lock` paths now carry that hint across the frame:
  - matcher path in
    [ruzstd/src/encoding/match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs)
  - fastest-level structural path in
    [ruzstd/src/encoding/levels/fastest.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/levels/fastest.rs)
- This replaces repeated per-block composer-content guessing with a whole-file known-profile hook.

Focused A/B:
- `generated_composer.lock`: `4,159 -> 4,119`
- unchanged controls:
  - `generated_package-lock.json = 4,381`
  - `generated_pipfile.lock = 2,804`
  - `generated_go.sum = 151`

Broad-local result:
- only `generated_composer.lock` moved
- current retained baseline vs C `zstd -1`:
  - `71` fixtures
  - `51 / 16 / 4` better / worse / equal
  - `1,798` bytes above C on the losing fixtures

Current biggest losses:
- `repo_Cargo.lock`: `9,105` vs `8,088` = `+1,017`
- `generated_composer.lock`: `4,119` vs `3,766` = `+353`
- `decodecorpus_z000079`: `7,322` vs `7,221` = `+101`

Live `generated_composer.lock` inspect:
- current retained:
  - `blocks=4`
  - `literal_section_bytes=687`
  - `sequence_payload_bytes=3386`
  - `decoded_literals=1503`
  - `sequences=2655`
- previous retained point:
  - `blocks=4`
  - `literal_section_bytes=681`
  - `sequence_payload_bytes=3432`
  - `decoded_literals=1495`
  - `sequences=2672`
- C `zstd -1` remains:
  - `blocks=2`
  - `literal_section_bytes=1597`
  - `sequence_payload_bytes=2137`
  - `decoded_literals=3294`
  - `sequences=2752`

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-profile-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-composer-profile-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-composer-profile-retained)

Conclusion:
- the whole-file composer profile is a real known-file-type win
- it does not change the block count yet, but it cuts sequence payload and sequence count across the composer path
- `repo_Cargo.lock` remains the dominant gap, with composer now materially reduced again

## 2026-06-01 - Retained Cargo.lock equal-length lazy-parse compare

Change notes:
- Extended the productive `Cargo.lock` lazy-parse family in
  [ruzstd/src/encoding/match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs):
  - still only on zero-literal lockfile positions
  - still compare against future candidates after skipping up to `2` literal bytes
  - but now allow an equal-length future candidate to win on local parse cost
- The retained point also raises the lockfile lazy-parse offset weight:
  - `offset_weight: 2 -> 3`

Focused tuner result:
- preset:
  - `cargo-lock-next-position-loss`
- baseline focused family total: `9,998`
- best searched candidate: `9,997`
- retained tuned point:
  - `max_skip_literals=2`
  - `max_current_match_len=7`
  - `max_match_loss=0`
  - `literal_weight=6`
  - `match_reward=2`
  - `offset_weight=3`
  - `margin=1`

Focused A/B:
- `repo_Cargo.lock`: `9,105 -> 9,104`
- unchanged controls:
  - `generated_go.sum = 151`
  - `generated_poetry.lock = 359`
  - `generated_yarn.lock = 383`

Live `Cargo.lock` archive signal at the retained point:
- previous retained point:
  - `sequence_count=810`
  - `sequence_payload_bytes=2184`
  - `of_extra_bits=6777`
  - `decoded_literals=9952`
  - `literal_section_bytes=6901`
- current retained point:
  - `sequence_count=810`
  - `sequence_payload_bytes=2182`
  - `of_extra_bits=6773`
  - `decoded_literals=9953`
  - `literal_section_bytes=6902`

Broad-local result:
- only `repo_Cargo.lock` moved
- current retained baseline vs C `zstd -1`:
  - `71` fixtures
  - `51 / 16 / 4` better / worse / equal
  - `1,797` bytes above C on the losing fixtures

Current biggest losses:
- `repo_Cargo.lock`: `9,104` vs `8,088` = `+1,016`
- `generated_composer.lock`: `4,119` vs `3,766` = `+353`
- `decodecorpus_z000079`: `7,322` vs `7,221` = `+101`

Artifacts:
- [focused sweep](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/tune-cargo-lock-next-position-loss.md)
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/cargo-lock-next-position-loss-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-cargo-lock-next-position-loss-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-cargo-lock-next-position-loss-retained)

Conclusion:
- the productive `Cargo.lock` lazy-parse family still has headroom beyond the first two retained points
- this step is smaller than the earlier skip win, but it is broad-safe and it cuts offset-side payload again without increasing sequence count

## 2026-06-01 - Rejected Cargo.lock two-step follow-up lazy parse

Change notes:
- Added then removed a tune-only extension of the productive lockfile lazy-parse family in
  [ruzstd/src/encoding/match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs):
  - estimate a two-step local path cost
  - compare `current` vs `skip->future` using an optional follow-up candidate after the first
    chosen match
- Added then removed focused preset:
  - `cargo-lock-next-position-followup`

Focused sweep result:
- baseline focused family total: `9,997`
- best searched candidate: `9,997`
- all top candidates kept `RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_USE_FOLLOWUP=0`

Conclusion:
- the broader two-step follow-up estimate did not beat the retained one-step lazy-parse point
- this closes another nearby `Cargo.lock` lookahead family

## 2026-06-01 - Rejected composer whole-vs-partition forced compare

Change notes:
- Added then removed a tune-only fastest-path switch in
  [ruzstd/src/encoding/levels/fastest.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/levels/fastest.rs):
  - force whole-vs-partition comparison even for composer text blocks
- Added then removed focused preset:
  - `composer-whole-compare`

Focused sweep result:
- baseline focused family total: `11,455`
- best searched candidate: `11,455`
- forcing whole compare stayed byte-identical across composer partition caps `3..8`

Conclusion:
- the retained whole-file composer profile did not expose a hidden whole-block win in the current
  fastest split machinery
- the remaining composer gap still points past this structural toggle

## 2026-06-01 - Retained small-text lockfile profile

Change notes:
- Added an internal `CompressionFileProfile::SmallTextLockfile` in
  [ruzstd/src/encoding/mod.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/mod.rs)
- Routed exact known small text lockfile names to that profile:
  - `poetry.lock`
  - `pubspec.lock`
- Applied a narrow encoder tuning in
  [ruzstd/src/encoding/blocks/compressed.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/blocks/compressed.rs):
  - `HuffmanTableSearch::AllSections`
  - `repeat_table_max_sequences = 256`
  - `offset_table_max_log = 7`
  - `offset_predefined_max_sequences = 64`

Focused A/B:
- `generated_poetry.lock`: `359 -> 358`
- `generated_pubspec.lock`: `232 -> 229`
- unchanged controls:
  - `generated_Gemfile.lock = 239`
  - `generated_go.sum = 151`
  - `generated_pubspec.yaml = 187`

Broad-local result:
- only two fixtures moved:
  - `generated_poetry.lock`: `359 -> 358`
  - `generated_pubspec.lock`: `232 -> 229`
- current retained baseline vs C `zstd -1`:
  - `71` fixtures
  - `51 / 16 / 4` better / worse / equal
  - `1,794` bytes above C on the losing fixtures

Current biggest losses:
- `repo_Cargo.lock`: `9,104` vs `8,088` = `+1,016`
- `generated_composer.lock`: `4,119` vs `3,766` = `+353`
- `decodecorpus_z000079`: `7,322` vs `7,221` = `+101`

Artifacts:
- [focused A/B](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/small-text-lockfile-focused.md)
- [broad-local A/B](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-compare-level1-small-text-lockfile-broad-local.md)
- [current baseline](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-current-level1-broad-local.md)
- [retained binary](/home/bsutton/git/zstd-rs/benchmarks/reports/ruzstd-cli-level1-small-text-lockfile-retained)

Conclusion:
- the subfamily is real and safe
- this is a clean extension-based known-file-type win, not a global encoder retune
- the dominant remaining work is still `repo_Cargo.lock`, then `generated_composer.lock`
