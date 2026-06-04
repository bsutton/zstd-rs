# Codex Workplan

## Current Goal

Branch: `compression/huffman-maxheight`

Implement and verify the two follow-up compression improvements after merging the two-candidate matcher:

1. Encoder-side Zstd repeat-offset history for sequence offsets.
2. Conservative sequence FSE table selection, using the C zstd implementation as the guide.

Keep the branch correct against the local Rust decoder and C zstd decoder, then benchmark against upstream and C zstd. Treat excellent test coverage as a first-class part of the goal: a performance or compression win is not complete unless its correctness invariants are covered, emitted bitstreams are round-tripped where relevant, or the workplan explicitly justifies why benchmark-only coverage is appropriate.

Quality constraints:

- Keep the Rust implementation high quality, well structured, and idiomatic.
- Avoid `unsafe` code. Treat safe Rust as a goal constraint, not just a preference.
- Keep fastest-level work conservative on CPU. If an experiment gives materially better compression but costs too much CPU for level 1, record it as a candidate for a future higher compression level instead of discarding the knowledge.
- Treat future compression levels as first-class design space: level 1 should stay comparable to C zstd's fast level tradeoffs, while higher levels can spend more CPU for better parsing, larger searches, stronger entropy choices, or more exact cost modeling.
- Do not force every compression win into level 1. If a change gives an impressive size result but has an obvious CPU cost, preserve the benchmark and implementation notes as a higher-level candidate, then keep level 1 on the current fast-parser budget.
- When evaluating those higher-level candidates, compare against C zstd at the corresponding levels as well as `-1`, so the CPU/ratio tradeoff is judged against the right target.
- Prefer clear state machines and small helpers over clever code that is harder to verify.
- Prefer explicit typed state over manual bit packing when the measured cost is acceptable. For matcher suffix candidates, the chosen direction is a small struct with two `Option<NonZeroU32>` values rather than packing two indexes into one `NonZeroU64`.
- Keep `usize` for Rust slice/window positions while searching, because slice indexing and lengths are naturally `usize`. Convert to `u32`/`NonZeroU32` only at bounded storage or bitstream boundaries, with checked conversions and cold invariant panics where the bound is guaranteed by the compressor window.
- Avoid `unwrap()`/`expect()` in production matcher code. Use explicit `match` branches with clear invariant messages on cold panic paths; do not replace these with `unsafe`.
- Maintain excellent test coverage for each compression feature; retained changes should have tests that make the behavior hard to regress.
- Cover private invariants with focused unit tests, especially compact matcher state such as suffix candidates and repeat-offset history.
- Cover emitted bitstreams with end-to-end tests through the Rust decoder and the C zstd decoder; helper-level tests alone are not enough.
- Every new compression heuristic should get either a regression test for the intended behavior or an explicit workplan note explaining why benchmark-only coverage is appropriate.

## 2026-06-01 - Retained Cargo.lock current-entry thirteenth-newest sidecar

- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added a thirteenth current-entry recent sidecar for lockfile-profiled text
  - probe the new thirteenth-newest candidate after the retained twelfth-newest path
- Added focused unit coverage:
  - `lockfile_sidecar_tracks_thirteenth_newest_for_current_entry`
- Reran the retained `cargo-lock-next-position-loss` lazy-parse surface on the current baseline:
  - it stayed flat and did not beat the current source

- Result:
  - focused:
    - `repo_Cargo.lock`: `9,012 -> 9,010`
    - unchanged:
      - `generated_go.sum = 151`
      - `generated_poetry.lock = 358`
      - `generated_yarn.lock = 383`
      - `generated_composer.lock = 4,112`
      - `generated_package-lock.json = 4,381`
      - `generated_pipfile.lock = 2,804`
  - broad-local:
    - only `repo_Cargo.lock` moved
  - current broad-local vs C `zstd -1`:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal
    - `1,693` bytes above C on losing fixtures

- Useful conclusion:
  - the alternative `Cargo.lock` lazy-parse surface is flat on the current baseline
  - the current-entry recency family is still the only live local `Cargo.lock` family
  - the next turn should decide whether to try one final recency slot or pivot to a broader
    non-local representation branch

## 2026-06-01 - Retained Cargo.lock current-entry twelfth-newest sidecar

- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added a twelfth current-entry recent sidecar for lockfile-profiled text
  - probe the new twelfth-newest candidate after the retained eleventh-newest path
- Added focused unit coverage:
  - `lockfile_sidecar_tracks_twelfth_newest_for_current_entry`

- Result:
  - focused:
    - `repo_Cargo.lock`: `9,014 -> 9,012`
    - unchanged:
      - `generated_go.sum = 151`
      - `generated_poetry.lock = 358`
      - `generated_yarn.lock = 383`
      - `generated_composer.lock = 4,112`
      - `generated_package-lock.json = 4,381`
      - `generated_pipfile.lock = 2,804`
  - broad-local:
    - only `repo_Cargo.lock` moved
  - current broad-local vs C `zstd -1`:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal
    - `1,695` bytes above C on losing fixtures

- Useful conclusion:
  - the current-entry recent-candidate family is still live on `Cargo.lock`
  - the main `Cargo.lock` gap is now `+924`
  - the next turn should explicitly decide whether to keep extending this family or pivot back to
    a different lockfile representation branch

## 2026-06-01 - Retained Cargo.lock current-entry eleventh-newest sidecar

- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added an eleventh current-entry recent sidecar for lockfile-profiled text
  - probe the new eleventh-newest candidate after the retained tenth-newest path
- Added focused unit coverage:
  - `lockfile_sidecar_tracks_eleventh_newest_for_current_entry`

- Result:
  - focused:
    - `repo_Cargo.lock`: `9,021 -> 9,014`
    - unchanged:
      - `generated_go.sum = 151`
      - `generated_poetry.lock = 358`
      - `generated_yarn.lock = 383`
      - `generated_composer.lock = 4,112`
      - `generated_package-lock.json = 4,381`
      - `generated_pipfile.lock = 2,804`
  - broad-local:
    - only `repo_Cargo.lock` moved
  - current broad-local vs C `zstd -1`:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal
    - `1,697` bytes above C on losing fixtures

- Useful conclusion:
  - the current-entry recent-candidate family is still live on `Cargo.lock`
  - the main `Cargo.lock` gap is now `+926`
  - the next turn should explicitly decide whether to keep extending this family or pivot back to
    a different lockfile representation branch

## 2026-06-01 - Retained Cargo.lock current-entry tenth-newest sidecar

- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added a tenth current-entry recent sidecar for lockfile-profiled text
  - probe the new tenth-newest candidate after the retained ninth-newest path
- Added focused unit coverage:
  - `lockfile_sidecar_tracks_tenth_newest_for_current_entry`

- Result:
  - focused:
    - `repo_Cargo.lock`: `9,025 -> 9,021`
    - unchanged:
      - `generated_go.sum = 151`
      - `generated_poetry.lock = 358`
      - `generated_yarn.lock = 383`
      - `generated_composer.lock = 4,112`
      - `generated_package-lock.json = 4,381`
      - `generated_pipfile.lock = 2,804`
  - broad-local:
    - only `repo_Cargo.lock` moved
  - current broad-local vs C `zstd -1`:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal
    - `1,704` bytes above C on losing fixtures

- Useful conclusion:
  - the current-entry recent-candidate family is still live on `Cargo.lock`
  - the main `Cargo.lock` gap is now `+933`
  - the next turn should explicitly decide whether to keep extending this family or pivot back to
    a different lockfile representation branch

## 2026-06-01 - Retained Cargo.lock current-entry ninth-newest sidecar

- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added a ninth current-entry recent sidecar for lockfile-profiled text
  - probe the new ninth-newest candidate after the retained eighth-newest path
- Added focused unit coverage:
  - `lockfile_sidecar_tracks_ninth_newest_for_current_entry`

- Result:
  - focused:
    - `repo_Cargo.lock`: `9,032 -> 9,025`
    - unchanged:
      - `generated_go.sum = 151`
      - `generated_poetry.lock = 358`
      - `generated_yarn.lock = 383`
      - `generated_composer.lock = 4,112`
      - `generated_package-lock.json = 4,381`
      - `generated_pipfile.lock = 2,804`
  - broad-local:
    - only `repo_Cargo.lock` moved
  - current broad-local vs C `zstd -1`:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal
    - `1,708` bytes above C on losing fixtures

- Useful conclusion:
  - the current-entry recent-candidate family is still live on `Cargo.lock`
  - the main `Cargo.lock` gap is now `+937`
  - the next turn should explicitly decide whether to keep extending this family or pivot back to
    a different lockfile representation branch

## 2026-06-01 - Retained Cargo.lock current-entry eighth-newest sidecar

- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added an eighth current-entry recent sidecar for lockfile-profiled text
  - probe the new eighth-newest candidate after the retained seventh-newest path
- Added focused unit coverage:
  - `lockfile_sidecar_tracks_eighth_newest_for_current_entry`

- Result:
  - focused:
    - `repo_Cargo.lock`: `9,042 -> 9,032`
    - unchanged:
      - `generated_go.sum = 151`
      - `generated_poetry.lock = 358`
      - `generated_yarn.lock = 383`
      - `generated_composer.lock = 4,112`
      - `generated_package-lock.json = 4,381`
      - `generated_pipfile.lock = 2,804`
  - broad-local:
    - only `repo_Cargo.lock` moved
  - current broad-local vs C `zstd -1`:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal
    - `1,715` bytes above C on losing fixtures

- Useful conclusion:
  - the current-entry recent-candidate family is still live on `Cargo.lock`
  - the main `Cargo.lock` gap is now `+944`
  - the next turn should explicitly decide whether to keep extending this family or pivot back to
    a different lockfile representation branch

## 2026-06-01 - Retained Cargo.lock current-entry seventh-newest sidecar

- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added a seventh current-entry recent sidecar for lockfile-profiled text
  - probe the new seventh-newest candidate after the retained sixth-newest path
- Added focused unit coverage:
  - `lockfile_sidecar_tracks_seventh_newest_for_current_entry`

- Result:
  - focused:
    - `repo_Cargo.lock`: `9,051 -> 9,042`
    - unchanged:
      - `generated_go.sum = 151`
      - `generated_poetry.lock = 358`
      - `generated_yarn.lock = 383`
      - `generated_composer.lock = 4,112`
      - `generated_package-lock.json = 4,381`
      - `generated_pipfile.lock = 2,804`
  - broad-local:
    - only `repo_Cargo.lock` moved
  - current broad-local vs C `zstd -1`:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal
    - `1,725` bytes above C on losing fixtures

- Useful conclusion:
  - the current-entry recent-candidate family is still live on `Cargo.lock`
  - the main `Cargo.lock` gap is now `+954`
  - the next turn should explicitly decide whether to keep extending this family or pivot back to
    a different lockfile representation branch

## 2026-06-01 - Retained Cargo.lock current-entry sixth-newest sidecar

- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added a sixth current-entry recent sidecar for lockfile-profiled text
  - probe the new sixth-newest candidate after the retained fifth-newest path
- Added focused unit coverage:
  - `lockfile_sidecar_tracks_sixth_newest_for_current_entry`

- Result:
  - focused:
    - `repo_Cargo.lock`: `9,065 -> 9,051`
    - unchanged:
      - `generated_go.sum = 151`
      - `generated_poetry.lock = 358`
      - `generated_yarn.lock = 383`
      - `generated_composer.lock = 4,112`
      - `generated_package-lock.json = 4,381`
      - `generated_pipfile.lock = 2,804`
  - broad-local:
    - only `repo_Cargo.lock` moved
  - current broad-local vs C `zstd -1`:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal
    - `1,734` bytes above C on losing fixtures

- Useful conclusion:
  - the current-entry recent-candidate family is still live on `Cargo.lock`
  - the main `Cargo.lock` gap is now `+963`
  - the next decision should be whether one more recency slot is still worth the added state, or
    whether this is the right point to pivot back to a different lockfile representation family

## 2026-06-01 - Retained Cargo.lock current-entry fifth-newest sidecar

- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added a fifth current-entry recent sidecar for lockfile-profiled text
  - probe the new fifth-newest candidate after the retained fourth-newest path
- Added focused unit coverage:
  - `lockfile_sidecar_tracks_fifth_newest_for_current_entry`

- Result:
  - focused:
    - `repo_Cargo.lock`: `9,073 -> 9,065`
    - unchanged:
      - `generated_go.sum = 151`
      - `generated_poetry.lock = 358`
      - `generated_yarn.lock = 383`
      - `generated_composer.lock = 4,112`
      - `generated_package-lock.json = 4,381`
      - `generated_pipfile.lock = 2,804`
  - broad-local:
    - only `repo_Cargo.lock` moved
  - current broad-local vs C `zstd -1`:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal
    - `1,748` bytes above C on losing fixtures

- Useful conclusion:
  - the current-entry recent-candidate family still has lockfile headroom, but the return per
    extra slot is shrinking
  - the main `Cargo.lock` gap is now `+977`
  - the next branch should probably check whether another added recency slot is still worth the
    extra state before continuing to hand-unroll this family

## 2026-06-01 - Retained Cargo.lock current-entry fourth-newest sidecar

- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added a fourth current-entry recent sidecar for lockfile-profiled text
  - probe the new fourth-newest candidate after the retained third-newest path
- Added focused unit coverage:
  - `lockfile_sidecar_tracks_fourth_newest_for_current_entry`

- Result:
  - focused:
    - `repo_Cargo.lock`: `9,087 -> 9,073`
    - unchanged:
      - `generated_go.sum = 151`
      - `generated_poetry.lock = 358`
      - `generated_yarn.lock = 383`
      - `generated_composer.lock = 4,112`
      - `generated_package-lock.json = 4,381`
      - `generated_pipfile.lock = 2,804`
  - broad-local:
    - only `repo_Cargo.lock` moved
  - current broad-local vs C `zstd -1`:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal
    - `1,756` bytes above C on losing fixtures

- Useful conclusion:
  - the lockfile current-entry recent-candidate family still has measurable headroom
  - the main `Cargo.lock` gap is now `+985`
  - the next branch can still stay in this broader current-entry representation family, but
    the diminishing returns from each extra recency slot should now be checked carefully

## 2026-06-01 - Retained Cargo.lock current-entry third-newest sidecar

- Extended the retained lockfile current-entry recent-candidate representation in
  `ruzstd/src/encoding/match_generator.rs`:
  - added a third current-entry recent sidecar for lockfile-profiled text
  - probe the new third-newest candidate after `second_newest` and before `newest`
  - keep the diagnostics surface unchanged by reusing the existing second-newest bookkeeping in
    test-only candidate-source reporting
- Added focused unit coverage:
  - `lockfile_sidecar_tracks_third_newest_for_current_entry`

- Result:
  - focused:
    - `repo_Cargo.lock`: `9,104 -> 9,087`
    - unchanged:
      - `generated_go.sum = 151`
      - `generated_poetry.lock = 358`
      - `generated_yarn.lock = 383`
      - `generated_composer.lock = 4,112`
      - `generated_package-lock.json = 4,381`
      - `generated_pipfile.lock = 2,804`
  - broad-local:
    - only `repo_Cargo.lock` moved
  - current broad-local vs C `zstd -1`:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal
    - `1,770` bytes above C on losing fixtures

- Useful `Cargo.lock` signal before the change:
  - `compressed_bytes=9104`
  - `literal_section_bytes=6902`
  - `sequence_payload_bytes=2182`
  - `decoded_literals=9953`
  - `sequences=810`
  - `match_bytes=21905`
  - `of_extra_bits=6773`

- Useful conclusion:
  - the lockfile current-entry recent-candidate family still had real headroom
  - `Cargo.lock` is now below a `+1000` byte gap to C on the retained baseline
  - the next `Cargo.lock` branch should continue from this broader current-entry representation
    point rather than return to already-bounded local threshold families

## 2026-06-01 - Rejected Cargo.lock local-parse current-window search

- Tested one broader lockfile parser-side family in
  `ruzstd/src/encoding/match_generator.rs`:
  - first a small local parse with simulated repeat history
  - then a widened local current-window search scoring several nearby window alternatives
- Added a temporary focused tuner preset:
  - `cargo-lock-local-parse`

- Result:
  - focused sweep stayed flat:
    - baseline focused family: `9,996`
    - best searched candidate: `9,996`
  - the branch was reverted and the temporary tuner preset was removed

- Useful conclusion:
  - the next productive `Cargo.lock` move was not hidden in a slightly broader local parse around
    the same active current-window candidates
  - do not retry this family in the same form

## 2026-06-01 - Rejected Cargo.lock byte-class lazy-parse literals and known-size single-segment frames

- Tested one new tune-only `Cargo.lock` lazy-parse branch:
  - replace the flat skipped-literal penalty inside the retained lockfile next-position compare
    with a byte-class literal model for lockfile syntax and common text bytes
- Result:
  - focused `cargo-lock-next-position-byteclass` sweep stayed flat at `9,996`
  - the tune-only branch was removed

- Re-ran the current `cargo-lock-encoder` surface on the live retained baseline:
  - baseline focused family: `10,006`
  - best searched candidate: `10,006`

- Tested a broader frame-level branch:
  - plumb exact known content size through the file-path CLI compression flow
  - emit single-segment frames when size is known
- Result:
  - direct spot checks regressed and the branch was reverted:
    - `repo_Cargo.lock`: `9,104 -> 9,105`
    - `generated_composer.lock`: `4,112 -> 4,115`
    - `generated_poetry.lock`: `358 -> 361`
    - `generated_pubspec.lock`: `229 -> 232`

- Useful conclusion:
  - the current `Cargo.lock` encoder-side nearby surface is still flat
  - the productive lockfile lazy-parse family is not missing an obvious byte-class literal bias in
    this local form
  - the current unknown-size frame form is already smaller than a known-size single-segment frame
    on these file inputs
  - the next credible branch still needs a broader parse/sequence representation change

## 2026-06-01 - Bounded wider Cargo.lock lazy-parse surface and composer repeat-kind >2

- Added focused tuner presets only:
  - `cargo-lock-next-position-wide`
  - `cargo-lock-combined-lazy`
  - `composer-repeatkind-wide`

- Results:
  - `cargo-lock-next-position-wide`
    - baseline focused family: `9,996`
    - best searched candidate: `9,996`
  - `cargo-lock-combined-lazy`
    - baseline focused family: `9,996`
    - best searched candidate: `9,996`
  - `composer-repeatkind-wide`
    - baseline focused family: `11,448`
    - best searched candidate: `11,448`
    - direct per-fixture check confirmed `loss=3` is an exact alias of the retained composer
      `loss=2` point

- Useful conclusion:
  - the productive `Cargo.lock` lazy-parse family is bounded again on the wider searched local
    surface
  - the retained composer same-start repeat-kind family is bounded upward: `2`, `3`, and `4`
    are equivalent on the live focused family
  - the next credible work should move away from nearby local matcher-threshold combinations and
    back to broader representation changes

## 2026-06-01 - Retained composer repeat-kind match-loss 2

- Widened the retained composer same-start repeat-kind preference in
  `ruzstd/src/encoding/match_generator.rs`:
  - when two current-position repeat candidates start at the same byte on a composer-profiled
    file, allow the repeat kind that better matches the encoder repeat-code order to win even
    when it loses up to `2` match bytes instead of `1`

- Result:
  - focused:
    - `generated_composer.lock`: `4,119 -> 4,112`
    - unchanged:
      - `generated_pipfile.lock = 2,804`
      - `generated_package-lock.json = 4,381`
      - `generated_go.sum = 151`
  - broad-local:
    - only `generated_composer.lock` moved
  - current broad-local vs C `zstd -1`:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal
    - `1,787` bytes above C on losing fixtures

- Useful signal:
  - same block count and same sequence count on `generated_composer.lock`
  - `sequence_payload_bytes: 3386 -> 3375`
  - `literal_section_bytes: 687 -> 691`

- Useful conclusion:
  - the whole-file `ComposerLock` profile exposed one more real improvement from the same-start
    repeat-kind family
  - the remaining composer gap is still mostly sequence-section side
  - the next highest-value target remains `repo_Cargo.lock`

Test coverage bar:

- Correctness fixes require a failing or gap-focused regression test.
- Parser/matcher heuristics require private invariant tests plus at least one emitted-stream round trip when the emitted sequence stream can change.
- Entropy-mode or bitstream changes require Rust decoder and C zstd decoder coverage.
- Performance-only refactors may use benchmark-only coverage only when output bytes are proven unchanged and the workplan records that rationale.

## 2026-06-01 - Kept Cargo.lock matcher-profile plumbing, rejected broader exact LL/ML candidate window

- Extended the internal `Cargo.lock` profile so it now reaches the matcher as well as the encoder:
  - added an internal file-profile hint hook to the matcher API
  - threaded the `Cargo.lock` profile through `FrameCompressor` into `MatchGenerator`
  - `Cargo.lock` blocks can now take the exact profile path on the parse side without relying
    only on content heuristics

- Tested one broader exact sequence-mode branch in
  `ruzstd/src/encoding/blocks/compressed.rs`:
  - for `Cargo.lock`, widen the exact LL/ML candidate set to include predefined LL/ML tables up to
    `1024` sequences

- Result:
  - the broader exact LL/ML branch was an exact no-op on the focused lockfile family and was
    reverted:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/cargolock-llml.md`
  - the retained matcher-profile plumbing is output-neutral against the retained
    `lockfamily-encoded-maxlog` baseline:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/cargolock-profile-matcher.md`

- Useful conclusion:
  - `Cargo.lock` now has exact profile plumbing on both the encoder and matcher sides
  - widening the nearby exact LL/ML table-candidate window did not move the live lockfile family
  - the next credible `Cargo.lock` branch still needs a more substantive literal/sequence
    representation change

## 2026-06-01 - Kept Cargo.lock profile scaffold, rejected zero-literal rewrite family

- Added a dedicated internal `CompressionFileProfile::CargoLock` in
  `ruzstd/src/encoding/mod.rs`.
- `repo_Cargo.lock`-style named files now carry a specific profile hook for future
  extension-based compression work.

- Tested one broader post-parse representation family in
  `ruzstd/src/encoding/blocks/compressed.rs`:
  - tune-only rewrite of short zero-literal `Cargo.lock` matches into literals
  - rebuild the prepared block so repeat history stays consistent end to end
- Swept:
  - `RUZSTD_TUNE_LOCKFILE_DROP_ZERO_LITERAL_MATCH_MAX_LEN = 5/6/7/8`
  - `RUZSTD_TUNE_LOCKFILE_DROP_ZERO_LITERAL_MATCH_MIN_OF_CODE = 7/8/9/10`
- Spot-checked narrower cases:
  - `(4,10)`
  - `(4,9)`
  - `(3,9)`

- Result:
  - the swept family regressed for `max_len >= 5`
  - the narrower `max_len 3/4` cases were exact no-ops
  - the rewrite branch was removed
  - keeping only the `Cargo.lock` profile scaffold restored exact equality to the retained
    `lockfamily-encoded-maxlog` baseline:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/cargolock-profile-restore.md`

- Useful conclusion:
  - `Cargo.lock` now has dedicated profile plumbing for future extension-specific algorithms
  - this first valid post-parse zero-literal rewrite family is bounded:
    - `max_len >= 5` regresses
    - `max_len 3/4` is flat
  - the next credible `Cargo.lock` branch still needs a different literal/sequence
    representation

## 2026-06-01 - Rejected lockfile lazy-parse family

- Tested one broader parser-side `Cargo.lock` family in
  `ruzstd/src/encoding/match_generator.rs`:
  - add a tune-only one-step lazy parse for lockfile-like `DictionaryText`
  - score the current candidate against the best next-position candidate with a cheap local
    score and configurable gain margin
- Swept:
  - `RUZSTD_TUNE_LOCKFILE_LAZY_SCORE_DIVISOR = 1/2/3/4/5/6`
  - `RUZSTD_TUNE_LOCKFILE_LAZY_MIN_GAIN = 0/1/2/3`

- Result:
  - focused tuner sweep stayed flat:
    - baseline `10,002`
    - best searched candidate `10,002`
  - the runtime branch was reverted
  - restore check returned exact equality to the retained `lockfamily-encoded-maxlog` baseline:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-lazy-restore.md`

- Useful conclusion:
  - `Cargo.lock` is not waiting on this one-step lazy-parse family either
  - that closes another broader parser-side representation attempt around the current matcher

## 2026-06-01 - Rejected lockfile non-repeat offset-score family

- Tested one broader parser-side `Cargo.lock` family in
  `ruzstd/src/encoding/match_generator.rs`:
  - add a tune-only non-repeat offset scorer for lockfile-like `DictionaryText`
  - compare candidates by:
    - `match_len * divisor - offset_code_bits`
- Swept:
  - `RUZSTD_TUNE_LOCKFILE_NONREPEAT_OFFSET_PENALTY_DIVISOR = 1/2/3/4/5/6`

- Result:
  - focused tuner sweep stayed flat:
    - baseline `10,002`
    - best searched candidate `10,002`
  - the runtime branch was reverted
  - restore check returned exact equality to the retained `lockfamily-encoded-maxlog` baseline:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-general-offset-restore.md`

- Useful conclusion:
  - `Cargo.lock` is not waiting on a broader non-repeat offset-bit scoring rule either
  - that closes another parser-side cost-model family around the current candidate comparer

## 2026-06-01 - Rejected generic smaller-offset lockfile matcher family

- Tested one broader parser-side lockfile family in `ruzstd/src/encoding/match_generator.rs`:
  - add a generic smaller-offset preference for non-repeat lockfile candidates
  - do not limit offset-aware tie-breaking to the already-retained same-start and same-end
    special cases
- Swept:
  - `RUZSTD_TUNE_LOCKFILE_SMALLER_OFFSET_MATCH_LOSS_MAX = 0/1/2/3`
  - `RUZSTD_TUNE_LOCKFILE_SMALLER_OFFSET_BITS_GAIN_MIN = 1/2/3/4`

- Result:
  - focused tuner sweep stayed flat:
    - baseline `10,002`
    - best searched candidate `10,002`
  - the default branch itself was also an exact byte-for-byte no-op on broad-local against the
    retained `lockfamily-encoded-maxlog` baseline
  - restore check returned exact equality afterward:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-general-offset-restore.md`

- Useful conclusion:
  - `Cargo.lock` is not waiting on a generic smaller-offset matcher preference either
  - that closes another broader parser-side offset-scoring family

## 2026-06-01 - Rejected exact encoded-table normalization variant search for lockfile families

- Tested one broader follow-up inside the retained exact sequence-mode search in
  `ruzstd/src/encoding/blocks/compressed.rs`:
  - for exact-sequence lockfile-family searches, encoded LL/ML/OF candidates also compared the
    alternate valid `avoid_0_numbit` normalization setting while still scanning encoded-table
    max-log choices

- Result:
  - exact byte-for-byte no-op on broad-local against the retained
    `lockfamily-encoded-maxlog` baseline
  - restore check returned exact equality afterward:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfamily-avoidzero-restore.md`

- Useful conclusion:
  - the retained lockfile-family exact sequence search is not missing another nearby FSE
    normalization variant in this form
  - the next credible `Cargo.lock` branch still needs a different literal/sequence
    representation, not another exact-sequence-table normalization toggle

## 2026-06-01 - Retained broader exact encoded-table log search for lockfile families

- Expanded the retained exact sequence-mode search in
  `ruzstd/src/encoding/blocks/compressed.rs`:
  - for fastest-level `DictionaryText` and dependency-JSON-lockfile profiles, exact
    sequence-mode search now compares not just table modes but also additional encoded-table
    max-log choices in the valid `7..=max_log` range
  - this keeps the broader search inside the families already paying for exact sequence
    re-encoding

- Result:
  - broad-local A/B against the retained `lockfamily-exact-seq` binary stayed clean
  - moved fixture:
    - `generated_package-lock.json`: `4,383 -> 4,381`
  - unchanged key controls:
    - `repo_Cargo.lock = 9,109`
    - `generated_composer.lock = 4,159`
    - `generated_pipfile.lock = 2,804`
    - `generated_pubspec.lock = 232`
    - `generated_Gemfile.lock = 239`

- Current broad-local vs C `zstd -1` remains:
  - `71` fixtures
  - `51 / 16 / 4` better / worse / equal
  - `1,842` bytes above C on losing fixtures

- Useful conclusion:
  - broader exact encoded-table log search is valid and broad-safe on the retained lockfile
    families
  - it further improves dependency-JSON lockfiles, but still does not move the dominant
    `Cargo.lock` or composer gaps

## 2026-06-01 - Rejected lockfile next-position lookahead branch

- Tested two matcher branches in `ruzstd/src/encoding/match_generator.rs`:
  - enable next-position window lookahead for lockfile-like `DictionaryText`
  - compare next-position repeat candidates even when a current repeat candidate already exists
  - keep the retained exact-sequence encoder baseline unchanged

- Result:
  - exact byte-for-byte no-op on broad-local against the retained `lockfamily-exact-seq` baseline
  - restore check returned exact equality afterward:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-nextpos-restore.md`

- Useful conclusion:
  - `Cargo.lock` is not waiting on next-position window or repeat lookahead in this form
  - next credible `Cargo.lock` work remains a broader literal/sequence representation change,
    not another nearby lookahead toggle

## 2026-06-01 - Rejected lockfile current-entry long-hash branch

- Tested a matcher branch in `ruzstd/src/encoding/match_generator.rs`:
  - enable the existing current-entry long-hash path for lockfile-like `DictionaryText`
  - keep the retained exact-sequence encoder baseline unchanged

- Result:
  - exact byte-for-byte no-op on broad-local against the retained `lockfamily-exact-seq` baseline
  - restore check returned exact equality afterward:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-longhash-restore.md`

- Useful conclusion:
  - the dormant current-entry long-hash path is not the missing `Cargo.lock` representation in this form
  - next credible `Cargo.lock` work remains a broader literal/sequence representation change, not another nearby matcher-toggle family

## 2026-06-01 - Retained exact LL/ML/OF sequence-mode search for lockfile families

- Kept one broader encoder-side representation change in `ruzstd/src/encoding/blocks/compressed.rs`:
  - for fastest-level `DictionaryText` and dependency-JSON-lockfile profiles, enumerate the valid LL, ML, and OF table modes from the existing heuristic family
  - exactly re-encode the sequence section across the valid combinations
  - keep the smallest valid LL/ML/OF combination
- Added focused unit coverage for:
  - which file families enable the exact sequence-mode search
  - the invariant that the exact chooser never emits a larger sequence section than the threshold path

- Result:
  - broad-local A/B against the retained `lockfamily-exact-of` binary stayed clean
  - moved fixture:
    - `generated_pubspec.lock`: `233 -> 232`
  - unchanged key controls:
    - `repo_Cargo.lock = 9,109`
    - `generated_composer.lock = 4,159`
    - `generated_package-lock.json = 4,383`
    - `generated_pipfile.lock = 2,804`

- Refreshed current broad-local baseline vs C `zstd -1`:
  - `71` fixtures
  - `51 / 16 / 4` better / worse / equal
  - `1,842` bytes above C on losing fixtures

- Useful conclusion:
  - exact full sequence-mode comparison is valid and slightly stronger than OF-only search
  - it still does not move the dominant `Cargo.lock` / composer gaps
  - next credible work remains a broader literal/sequence representation change, not another nearby FSE table threshold family

## 2026-06-01 - Retained exact OF-mode sequence-section search for lockfile families

- Kept one new encoder-side representation change in `ruzstd/src/encoding/blocks/compressed.rs`:
  - for fastest-level `DictionaryText` and dependency-JSON-lockfile profiles, keep the threshold LL/ML/OF starting point but exactly compare the valid OF table modes by re-encoding the sequence section and keeping the smallest
- Added focused unit coverage for:
  - which file families enable the exact search
  - the invariant that the exact chooser never emits a larger sequence section than the threshold path

- Result:
  - broad-local A/B against the pre-change current binary was clean
  - only one fixture moved:
    - `generated_Gemfile.lock`: `240 -> 239`
  - unchanged key controls:
    - `repo_Cargo.lock = 9,109`
    - `generated_composer.lock = 4,159`
    - `generated_package-lock.json = 4,383`
    - `generated_pipfile.lock = 2,804`

- Refreshed current broad-local baseline vs C `zstd -1`:
  - `71` fixtures
  - `51 / 16 / 4` better / worse / equal
  - `1,843` bytes above C on losing fixtures

- Useful conclusion:
  - exact OF-mode comparison is valid and broad-safe
  - it is still not the missing `Cargo.lock` / composer unlock
  - next credible work remains a broader sequence/literal representation change, not another local OF threshold tweak

## 2026-06-01 - Rejected composer-style min non-repeat floor 5

- Stayed on the retained `composer-filetypeconfig` baseline:
  - `generated_composer.lock = 4,336`
  - `generated_pipfile.lock = 2,811`
  - `generated_package-lock.json = 4,392`
  - `generated_go.sum = 151`
  - `repo_Cargo.lock = 9,114`

- Rejected:
  - use minimum non-repeat match length `5` for composer-style `DictionaryText` blocks

- Result:
  - exact byte-for-byte no-op on the focused composer/lockfile family
  - restore check confirmed the retained baseline exactly:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-floor5-restore.md`

- Useful conclusion:
  - the remaining composer gap is not waiting on a lower non-repeat floor either
  - the floor-5 family is now closed with direct focused evidence

## 2026-06-01 - Rejected composer-style probe step 2

- Stayed on the retained `composer-filetypeconfig` baseline:
  - `generated_composer.lock = 4,336`
  - `generated_pipfile.lock = 2,811`
  - `generated_package-lock.json = 4,392`
  - `generated_go.sum = 151`
  - `repo_Cargo.lock = 9,114`

- Rejected:
  - use no-match probe step `2` for composer-style `DictionaryText` blocks

- Result:
  - exact byte-for-byte no-op on the focused composer/lockfile family
  - restore check confirmed the retained baseline exactly:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-step2-restore.md`

- Useful conclusion:
  - the remaining composer gap is not waiting on a less-dense current-window probe step either
  - the retained `Cargo.lock` step-2 win does not transfer to composer in this form

## 2026-06-01 - Rejected composer-style second_newest-before-newest probing

- Stayed on the retained `composer-filetypeconfig` baseline:
  - `generated_composer.lock = 4,336`
  - `generated_pipfile.lock = 2,811`
  - `generated_package-lock.json = 4,392`
  - `generated_go.sum = 151`
  - `repo_Cargo.lock = 9,114`

- Rejected:
  - probe current-entry `second_newest` before `newest` for composer-style `DictionaryText` blocks

- Result:
  - exact byte-for-byte no-op on the focused composer/lockfile family
  - restore check confirmed the retained baseline exactly:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-secondnewestfirst-restore.md`

- Useful conclusion:
  - the remaining composer gap is not waiting on lockfile-style `second_newest` probe ordering
  - current-entry `second_newest` does not look like the missing representation for this family in the current matcher shape

## 2026-06-01 - Rejected two more composer-family structural branches

- Stayed on the retained `composer-filetypeconfig` baseline:
  - `generated_composer.lock = 4,336`
  - `generated_pipfile.lock = 2,811`
  - `generated_package-lock.json = 4,392`
  - `generated_go.sum = 151`
  - `repo_Cargo.lock = 9,114`

- Rejected:
  - disable the special text-repeat pipeline for composer-style `DictionaryText`
  - search actual encoded composer partition candidates across partition budgets `1..=8` and keep the smallest

- Result:
  - both were exact byte-for-byte no-ops on the focused composer/lockfile family
  - restore check confirmed the retained baseline exactly:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-restore-after-branches.md`

- Useful conclusion:
  - the remaining composer gap is not moving on this text-repeat pipeline distinction
  - it is also not waiting on a broader actual-budget search over the current partition-tree family

## 2026-06-01 - Retained partition-budget fix, rejected composer max-2 partition cap

- Fixed `derive_best_partitions()` in `ruzstd/src/encoding/levels/fastest.rs` so the requested partition budget is enforced strictly.
  - the old recursion could let the left subtree consume the budget and still append the right half
  - added focused unit coverage proving the helper cannot exceed a `2`-partition budget on a recursively splittable synthetic block
- Verified the retained live composer/lockfile family stays byte-identical with the fix when the default `8`-partition composer path is restored:
  - `generated_composer.lock = 4,336`
  - `generated_pipfile.lock = 2,811`
  - `generated_package-lock.json = 4,392`
  - `generated_go.sum = 151`
  - `repo_Cargo.lock = 9,114`
  - report:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-budgetfix-focused.md`

- Rejected the direct follow-up:
  - cap the composer-style `DictionaryText` partition path at `2` partitions
- Result:
  - `generated_composer.lock`: `4,336 -> 4,389`
  - unchanged nearby controls:
    - `generated_pipfile.lock = 2,811`
    - `generated_package-lock.json = 4,392`
    - `generated_go.sum = 151`
    - `repo_Cargo.lock = 9,114`
  - report:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-cap2-focused.md`

- Useful conclusion:
  - the partition-budget bug was real, but the live retained composer path was already byte-identical on the focused family once the default `8`-partition budget is restored
  - the next composer branch should not be a blunt smaller partition cap

## 2026-06-01 - Rejected two composer partition sequence-entropy branches

- Stayed on the retained `composer-filetypeconfig` baseline:
  - `generated_composer.lock = 4,336`
  - `repo_Cargo.lock = 9,114`
- Screened the composer/lockfile family only:
  - `generated_composer.lock`
  - `generated_pipfile.lock`
  - `generated_package-lock.json`
  - `generated_go.sum`
  - `repo_Cargo.lock`

- Rejected:
  - allow composer partition candidates to reuse previous FSE tables up to `1024` sequences
  - allow composer partition candidates to use predefined OF tables up to `1024` sequences

- Result:
  - repeat previous FSE tables: hard regression
    - `generated_composer.lock`: `4,336 -> 4,524`
  - predefined OF tables: hard regression
    - `generated_composer.lock`: `4,336 -> 5,025`
  - restore check confirmed the retained baseline exactly:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-entropy-restore2.md`

- Useful conclusion:
  - the current composer partition path does not want broader sequence-table reuse or a wider predefined-OF window
  - next composer work should stay away from these sequence-table families

## 2026-06-01 - Rejected three more Cargo.lock-focused branches

- Stayed on the retained `composer-filetypeconfig` baseline:
  - `repo_Cargo.lock = 9,114`
  - `generated_composer.lock = 4,336`
- Screened the known lockfile family only:
  - `repo_Cargo.lock`
  - `generated_composer.lock`
  - `generated_pipfile.lock`
  - `generated_package-lock.json`
  - `generated_go.sum`

- Rejected:
  - lockfile-like `DictionaryText` partitioned-block candidate path with fastest-level file-type block config
  - lockfile current-over-`oldest` displacement when a two-byte `oldest` gain still costs at least two more offset-code bits
  - lockfile current-over-`newest` displacement when a two-byte `newest` gain still costs at least two more offset-code bits

- Result:
  - partition path: exact no-op
    - `repo_Cargo.lock`: stayed `9,114`
  - `oldest` bits branch: regression
    - `repo_Cargo.lock`: `9,114 -> 9,116`
  - `newest` bits branch: exact no-op
    - `repo_Cargo.lock`: stayed `9,114`
  - restore check confirmed the retained baseline exactly:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-bits-restore.md`

- Useful conclusion:
  - the current `Cargo.lock` gap is not moving on another partition-path retest or another offset-bit-aware current-window displacement rule
  - next credible work should stay away from these local window-comparison branches

## 2026-06-01 - Retained file-type block config for composer partition candidates

- Kept one more composer-specific known-file-type compression change:
  - in `ruzstd/src/encoding/levels/fastest.rs`, the composer-style `DictionaryText` partitioned-block path now uses the live fastest-level file-type block config instead of the generic `Best` block config
- Refreshed the retained current broad-local baseline:
  - `57` fixtures
  - `43 / 10 / 4` better / worse / equal vs C
  - `1,725` bytes above C on the losing fixtures

- Result:
  - focused composer-family win:
    - `generated_composer.lock`: `4,340 -> 4,336`
  - unchanged nearby controls:
    - `generated_pipfile.lock`: stayed `2,811`
    - `generated_package-lock.json`: stayed `4,392`
    - `generated_go.sum`: stayed `151`
    - `repo_Cargo.lock`: stayed `9,114`
  - broad-local losing-byte total:
    - `1,729 -> 1,725`

- Current largest losses:
  - `repo_Cargo.lock`: `+1,026`
  - `generated_composer.lock`: `+570`
  - `decodecorpus_z000079`: `+101`

- Useful conclusion:
  - the composer partition family still had a small retained gain left once it used the live `DictionaryText` fastest-level block config
  - next known-file-type priority remains `repo_Cargo.lock`, with `generated_composer.lock` still second

## 2026-06-01 - Retained composer-style `DictionaryText` partitioned-block path

- Kept one new known-file-type compression change:
  - in `ruzstd/src/encoding/levels/fastest.rs`, large composer-style `DictionaryText` blocks at level 1 now use the existing `compress_best_with_estimated_splits()` path
  - the gate uses `likely_composer_lockfile_text()` in `ruzstd/src/encoding/util.rs`
- Refreshed the retained current broad-local baseline:
  - `57` fixtures
  - `43 / 10 / 4` better / worse / equal vs C
  - `1,729` bytes above C on the losing fixtures

- Result:
  - focused known-file-type win:
    - `generated_composer.lock`: `4,461 -> 4,340`
  - unchanged nearby controls:
    - `generated_pipfile.lock`: stayed `2,811`
    - `generated_package-lock.json`: stayed `4,392`
    - `generated_go.sum`: stayed `151`
    - `repo_Cargo.lock`: stayed `9,114`
  - broad-local losing-byte total:
    - `1,850 -> 1,729`

- Current largest losses:
  - `repo_Cargo.lock`: `+1,026`
  - `generated_composer.lock`: `+574`
  - `decodecorpus_z000079`: `+101`

- Useful conclusion:
  - this is the first retained `generated_composer.lock` win after the public remaps and small composer matcher branches all failed
  - the next best known-file-type targets remain `repo_Cargo.lock` first and `generated_composer.lock` second

## 2026-06-01 - Expanded known-file-type corpus and rejected `composer.lock` / `Pipfile.lock` -> `JsonText`

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

- Tried one public starting-point correction in `ruzstd/src/encoding/mod.rs`:
  - map `composer.lock` and `Pipfile.lock` to `JsonText`

- Result:
  - focused regression:
    - `generated_composer.lock`: `4,461 -> 4,482`
    - `generated_pipfile.lock`: `2,811 -> 2,885`
    - `repo_Cargo.lock`: unchanged at `9,114`

- Useful conclusion:
  - the corpus expansion is retained and valuable
  - these two lockfiles are now covered by benchmarks
  - but the plain `JsonText` starting point is wrong for both of them in this form

## 2026-06-01 - Rejected lockfile package-boundary raw-data multi-block split

- Stayed on the retained live baseline for the dictionary/lockfile family:
  - `repo_Cargo.lock = 9,114`
  - `dict_dictionary.bin = 19,668`
  - `generated_go.sum = 151`
  - `generated_poetry.lock = 359`
  - `generated_yarn.lock = 383`
- Tried one structural literal-side branch in `ruzstd/src/encoding/frame_compressor.rs`:
  - for lockfile-like `DictionaryText` on the fastest path, split the raw input into package-aligned segments before matching and emit multiple compressed blocks
- Verified the helper would actually split `repo_Cargo.lock` into four package-aligned segments:
  - `8193 / 8217 / 8197 / 7251`

- Result:
  - exact byte-for-byte no-op on the focused dictionary/lockfile family

- Useful conclusion:
  - even a real package-aligned multi-block split does not move the lockfile path in this form
  - this closes another structural literal-context family while leaving the retained baseline unchanged

## 2026-06-01 - Rejected `DictionaryText` predefined OF up to 1024 sequences

- Stayed on the retained live baseline for the dictionary/lockfile family:
  - `repo_Cargo.lock = 9,114`
  - `dict_dictionary.bin = 19,668`
  - `generated_go.sum = 151`
  - `generated_poetry.lock = 359`
  - `generated_yarn.lock = 383`
- Tried one sequence-entropy branch in `ruzstd/src/encoding/blocks/compressed.rs`:
  - on the `DictionaryText` path, let OF use predefined tables up to `1024` sequences instead of the generic `16`

- Result:
  - exact byte-for-byte no-op on the focused dictionary/lockfile family

- Useful conclusion:
  - the obvious `DictionaryText` predefined-OF window does not move the lockfile path in this form
  - this closes another sequence-entropy family while leaving the retained baseline unchanged

## 2026-06-01 - Rejected lockfile larger dense-match insertion limit

- Stayed on the retained live baseline for the lockfile family:
  - `repo_Cargo.lock = 9,114`
  - `dict_dictionary.bin = 19,668`
  - `generated_go.sum = 151`
  - `generated_poetry.lock = 359`
  - `generated_yarn.lock = 383`
- Tried one parse-representation branch in `ruzstd/src/encoding/match_generator.rs`:
  - for lockfile-like `DictionaryText`, increase the dense post-match suffix insertion limit from `128` to `256`

- Result:
  - exact byte-for-byte no-op on the focused lockfile family

- Useful conclusion:
  - the lockfile dense post-match insertion family is now bounded on both sides:
    - `64` is worse
    - `256` is a no-op
  - this closes another `Cargo.lock` parse-representation family while leaving the retained baseline unchanged

## 2026-06-01 - Rejected lockfile smaller dense-match insertion limit

- Stayed on the retained live baseline for the rest of the lockfile family:
  - `dict_dictionary.bin = 19,668`
  - `generated_go.sum = 151`
  - `generated_poetry.lock = 359`
  - `generated_yarn.lock = 383`
- Tried one parse-representation branch in `ruzstd/src/encoding/match_generator.rs`:
  - for lockfile-like `DictionaryText`, reduce the dense post-match suffix insertion limit from `128` to `64`

- Result:
  - `repo_Cargo.lock`: `9,114 -> 9,116`

- Useful conclusion:
  - the lockfile path does not want a smaller dense post-match insertion limit
  - this closes another lockfile parse-representation family while leaving the retained baseline unchanged

## 2026-06-01 - Rejected small short-line `ConfigText` current-over-`oldest` displacement

- Stayed on the retained live baseline:
  - `repo_ruzstd_Cargo.toml = 730`
  - `repo_Cargo.toml = 68`
  - `repo_ci.yml = 556`
  - `repo_.gitignore = 166`
- Fresh `repo_ruzstd_Cargo.toml` evidence still showed a parser-side gap versus C:
  - Rust: `literal_section_bytes=570`, `sequence_payload_bytes=142`, `sequences=51`
  - C: `520`, `187`, `71`
- Tried one narrow parser branch in `ruzstd/src/encoding/match_generator.rs`:
  - for small short-line `ConfigText`, keep the current candidate over a farther `oldest` non-repeat candidate unless `oldest` gains at least `2` match bytes

- Result:
  - exact byte-for-byte no-op on the focused `ConfigText` family

- Useful conclusion:
  - the remaining small `ConfigText` / TOML tail is not waiting on this current-vs-`oldest` displacement rule
  - this closes another small `ConfigText` parser family while leaving the retained baseline unchanged

## 2026-06-01 - Rejected `DictionaryText` adaptive single-stream vs four-stream Huffman literals

- Stayed on the retained live baseline:
  - `repo_Cargo.lock = 9,114`
  - `dict_dictionary.bin = 19,668`
  - `generated_go.sum = 151`
  - `generated_poetry.lock = 359`
  - `generated_yarn.lock = 383`
- Fresh `Cargo.lock` archive inspection still showed the remaining lockfile gap is literal-stream-side:
  - Rust: `literals_payload=6886`, `literals_stream=6855`
  - C: `5975`, `5930`
- Tried one literal-side branch in `ruzstd/src/encoding/blocks/compressed.rs`:
  - on the `DictionaryText` path, compare single-stream vs four-stream Huffman literal layouts and keep the smaller estimated encoding

- Result:
  - exact byte-for-byte no-op on the focused dictionary/lockfile family

- Useful conclusion:
  - the remaining `Cargo.lock` gap is not waiting on single-stream vs four-stream Huffman layout selection in this form
  - this closes another literal-layout family while leaving the retained baseline unchanged

## 2026-06-01 - Rejected small short-line `ConfigText` next-position window lookahead

- Stayed on the retained live baseline:
  - `repo_ruzstd_Cargo.toml = 730`
  - `repo_ci.yml = 556`
  - `repo_.gitignore = 166`
- Fresh matcher diagnostics on `repo_ruzstd_Cargo.toml` showed only current-position window wins:
  - `window_current_newest = 22`
  - `window_current_oldest = 28`
  - `window_next_position_* = 0`
- Tried one known-file-type parser branch in `ruzstd/src/encoding/match_generator.rs`:
  - enable next-position window lookahead for small short-line `ConfigText`

- Result:
  - exact byte-for-byte no-op across `broad-local`
  - no fixture bytes moved at all

- Useful conclusion:
  - the remaining small `ConfigText` / TOML tail is not waiting on next-position window lookahead in this form
  - this closes another known-file-type parser family while leaving the retained baseline unchanged

## 2026-06-01 - Rejected tiny single-stream `ConfigText` actual-byte Huffman table search

- Stayed on the retained live baseline:
  - `repo_.gitignore = 166`
  - `repo_ruzstd_Cargo.toml = 730`
- Fresh `.gitignore` archive inspection still showed a pure literal-side tail:
  - Rust: `literals_payload=131`, `literals_table_desc=22`, `literals_stream=109`
  - C: `129`, `24`, `105`
- Tried one literal-side branch in `ruzstd/src/huff0/huff0_encoder.rs` and `ruzstd/src/encoding/blocks/compressed.rs`:
  - for tiny single-stream `ConfigText` literals, choose exact Huffman tables by actual encoded bytes instead of the estimate

- Result:
  - exact byte-for-byte no-op across `broad-local`
  - no fixture bytes moved at all

- Useful conclusion:
  - the remaining tiny `ConfigText` literal tail is not waiting on actual-byte re-ranking of the current exact Huffman candidate set
  - this closes another literal-selection family while leaving the retained baseline unchanged

## 2026-06-01 - Rejected small short-line `ConfigText` current-entry `second_newest`

- Stayed on the retained live baseline:
  - `repo_ruzstd_Cargo.toml = 730`
  - `repo_ci.yml = 556`
  - `repo_.gitignore = 166`
- Tried one known-file-type parser branch in `ruzstd/src/encoding/match_generator.rs`:
  - enable current-entry `second_newest` for small short-line `ConfigText` blocks

- Result:
  - exact byte-for-byte no-op across `broad-local`
  - no fixture bytes moved at all

- Useful conclusion:
  - the remaining small `ConfigText` / TOML tail is not waiting on current-entry `second_newest` in this form
  - this closes another known-file-type parser family while leaving the retained baseline unchanged

## 2026-06-01 - Rejected lockfile-only no-backward-extension parse branch

- Stayed on the retained lockfile-specific baseline:
  - `repo_Cargo.lock = 9,114`
- Tried one lockfile-specific parse-shape branch in `ruzstd/src/encoding/match_generator.rs`:
  - disable backward match extension for lockfile-like `DictionaryText`

- Result:
  - exact byte-for-byte no-op across the focused lockfile family:
    - `repo_Cargo.lock`: stayed `9,114`
    - `generated_go.sum`: stayed `151`
    - `generated_poetry.lock`: stayed `359`
    - `generated_yarn.lock`: stayed `383`

- Restore:
  - focused restore confirmed the source tree is back on the retained point:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-nobackextend-restore.md`
    - `repo_Cargo.lock = 9,114`

- Useful conclusion:
  - the retained `Cargo.lock` gap is not waiting on backward match extension in this form
  - this closes another parse-shape branch on the active lockfile path

## 2026-06-01 - Rejected lockfile-only zero-literal nonrepeat extra floor

- Stayed on the retained lockfile-specific baseline:
  - `repo_Cargo.lock = 9,114`
- Tried one narrow lockfile-specific matcher rule in `ruzstd/src/encoding/match_generator.rs`:
  - zero-literal, non-repeat window candidates on the lockfile path must be `6` bytes long instead of `5`

- Result:
  - focused lockfile family regression:
    - `repo_Cargo.lock`: `9,114 -> 9,143`
    - `generated_go.sum`: stayed `151`
    - `generated_poetry.lock`: stayed `359`
    - `generated_yarn.lock`: stayed `383`

- Restore:
  - focused restore confirmed the source tree is back on the retained point:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-zero-literal-floor-restore.md`
    - `repo_Cargo.lock = 9,114`

- Useful conclusion:
  - the lockfile histogram still points at too many zero-literal sequences versus C
  - but a blunt extra minimum length for zero-literal non-repeat window matches over-cuts the retained parser and makes compression worse

## 2026-06-01 - Rejected fixed newline-aligned multi-block split for lockfile-like `DictionaryText`

- Stayed on the retained lockfile-specific baseline:
  - `repo_Cargo.lock = 9,114`
- Tried one structural lockfile branch in `ruzstd/src/encoding/levels/fastest.rs`:
  - split large lockfile-like `DictionaryText` blocks at fixed newline-aligned segment boundaries around `8 KiB`

- Result:
  - exact byte-for-byte no-op on the focused lockfile family:
    - `repo_Cargo.lock`: stayed `9,114`
    - `generated_go.sum`: stayed `151`
    - `generated_poetry.lock`: stayed `359`
    - `generated_yarn.lock`: stayed `383`

- Restore:
  - sequential restore check confirmed the source tree is back on the retained point:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-fixedsplit-restore-seq.md`
    - `repo_Cargo.lock = 9,114`

- Useful conclusion:
  - the retained `Cargo.lock` gap is not waiting on a simple fixed multi-block split
  - this closes another structural split family on the active lockfile parser shape

## 2026-06-01 - Rejected `DictionaryText` small-sequence LL/ML max-log lockfile branches

- Stayed on the retained lockfile-specific `oldest +2` baseline:
  - `repo_Cargo.lock = 9,114`
- Tried two narrow sequence-entropy variants for lockfile-scale `DictionaryText` blocks:
  1. LL/ML FSE max-log `8`
  2. LL/ML FSE max-log `7`

- Result:
  - both were exact byte-for-byte no-ops on focused `Cargo.lock`

- Restore:
  - sequential restore check confirmed the source tree is back on the retained point:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-dictllmlmaxlog-seq.md`
    - `repo_Cargo.lock = 9,114`

- Useful conclusion:
  - the lockfile gap is not waiting on a smaller LL/ML FSE max-log in this family
  - this small-sequence LL/ML max-log line is closed in the tested points

## 2026-06-01 - Fully bounded the retained lockfile current-vs-`oldest` family: `+1` and `+3` are both worse

- Stayed on the retained lockfile-specific `oldest +2` baseline:
  - `repo_Cargo.lock = 9,114`
- After already rejecting `+3`, tested the other side of the same family:
  - require `oldest` to gain at least `1` match byte instead of `2`

- Result:
  - target regressed:
    - `repo_Cargo.lock`: `9,114 -> 9,116`

- Restore:
  - sequential restore check confirmed the source tree is back on the retained point:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-oldestgain1-seq.md`
    - `repo_Cargo.lock = 9,114`

- Useful conclusion:
  - the lockfile current-vs-`oldest` family is now fully bounded on the active parser shape:
    - `+1` is worse
    - `+2` is the retained best point
    - `+3` is worse

## 2026-06-01 - Rejected wider same-start smaller-offset rule on the retained lockfile parser

- Stayed on the retained lockfile-specific `oldest +2` baseline:
  - `repo_Cargo.lock = 9,114`
- Retested one older lockfile family against the current parser shape:
  - widen the same-start smaller-offset rule from `1` byte of allowed match loss to `2`
  - scope: lockfile-like `DictionaryText` only

- Result:
  - target regressed:
    - `repo_Cargo.lock`: `9,114 -> 9,117`

- Restore:
  - sequential restore check confirmed the source tree is back on the retained point:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-offset2-retest-seq.md`
    - `repo_Cargo.lock = 9,114`

- Useful conclusion:
  - this older offset-aware family still does not become valid on the new lockfile parser shape
  - the retained same-start smaller-offset rule is still the useful edge

## 2026-06-01 - Rejected two more narrow lockfile branches on the retained `oldest +2` baseline

- Stayed on the retained lockfile-specific `oldest +2` baseline:
  - `repo_Cargo.lock = 9,114`
- Tried two narrower follow-ups on the active parser shape:
  1. a zero-literal repeat-margin bonus for lockfile text
  2. a current-vs-`second_newest` displacement rule

- Result:
  - both were exact no-ops in matcher diagnostics on live `Cargo.lock`
  - no focused size run was worth promoting after the diagnostic no-op

- Restore:
  - sequential restore check confirmed the source tree is back on the retained point:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-secondnewest-noop-seq.md`
    - `repo_Cargo.lock = 9,114`

- Useful conclusion:
  - the active lockfile parser does not want another narrow repeat-side bonus
  - it also does not want a current-vs-`second_newest` displacement rule in this form

## 2026-06-01 - Bounded the retained lockfile current-vs-`oldest` family: `+2` is good, `+3` is worse

- Stayed on the retained lockfile-specific `oldest +2` baseline:
  - `repo_Cargo.lock = 9,114`
- Tried the next obvious bound on the same active parser shape:
  - require `oldest` to gain at least `3` match bytes instead of `2`

- Result:
  - target regressed:
    - `repo_Cargo.lock`: `9,114 -> 9,117`

- Restore:
  - sequential restore check confirmed the source tree is back on the retained point:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-oldestgain3-seq.md`
    - `repo_Cargo.lock = 9,114`

- Useful conclusion:
  - the lockfile `oldest`-displacement family is now bounded on the active parser shape:
    - `+2` retained best point
    - `+3` is worse

## 2026-06-01 - Retained lockfile-specific current-vs-`oldest` displacement after `second_newest`-first probing

- Stayed on the retained lockfile-specific `second_newest`-first baseline:
  - `repo_Cargo.lock = 9,116`
- Tried one narrower current-window scoring rule on the active lockfile parser:
  - keep the current non-repeat candidate over a farther `oldest` candidate unless `oldest` gains at least `2` match bytes

- Result:
  - `repo_Cargo.lock`: `9,116 -> 9,114`
  - every other corrected-suite fixture stayed byte-identical
  - corrected broad-local bytes-above-C on losers:
    - `1,184 -> 1,182`

- Useful matcher result:
  - sequences stayed `821`
  - `window_current_oldest[0]`: `211 -> 196`
  - `window_current_second_newest[0]`: `100 -> 105`
  - `window_current_newest[0]`: `413 -> 421`

- Useful inspect result:
  - `sequence_payload_bytes`: `2,213 -> 2,208`
  - `of_extra_bits`: `6,944 -> 6,898`
  - `literal_section_bytes`: `6,883 -> 6,886`

- Conclusion:
  - the active lockfile parser still had a small useful `oldest`-side scoring gain left
  - any more work in this family needs to stay narrow and evidence-driven

## 2026-06-01 - Retained lockfile-specific current-window probe order: `second_newest` before `newest`

- Stayed on the retained lockfile-specific `step 2` baseline:
  - `repo_Cargo.lock = 9,170`
- Tried one narrower current-window parser branch for the active lockfile path:
  - probe current-entry `second_newest` before `newest`
  - keep `oldest` last

- Result:
  - `repo_Cargo.lock`: `9,170 -> 9,116`
  - every other corrected-suite fixture stayed byte-identical
  - corrected broad-local bytes-above-C on losers:
    - `1,238 -> 1,184`

- Useful matcher result:
  - sequences: `848 -> 821`
  - `window_current_second_newest[0]`: `44 -> 100`
  - `window_current_newest[0]`: `475 -> 413`

- Useful inspect result:
  - `sequence_payload_bytes`: `2,258 -> 2,213`
  - `of_extra_bits`: `7,164 -> 6,944`
  - `literal_section_bytes`: `6,892 -> 6,883`

- Conclusion:
  - the lockfile parser still had a real current-window ordering win available
  - `second_newest` is now strong enough on this family that it should be probed before `newest`

## 2026-06-01 - Rejected lockfile-specific next-position window lookahead

- Stayed on the retained lockfile-specific `step 2` baseline:
  - `repo_Cargo.lock = 9,170`
- Tried one more adjacent-position parser branch:
  - let lockfile-like `DictionaryText` reach the existing next-position window lookahead gate

- Result:
  - exact byte-for-byte no-op
  - `repo_Cargo.lock`: stayed `9,170`

- Restore:
  - rebuilt restore check confirmed the source tree is back on the retained point:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-nextwindow.md`
    - `repo_Cargo.lock = 9,170`

- Useful conclusion:
  - the lockfile path does not benefit from next-position window lookahead either
  - this closes another adjacent-position family on the retained lockfile parser shape

## 2026-06-01 - Rejected lockfile oldest-first current-window probing

- Stayed on the retained lockfile-specific `step 2` baseline:
  - `repo_Cargo.lock = 9,170`
- Tried one best-level parser behavior selectively on the lockfile path:
  - current-window oldest-first probing

- Result:
  - target regressed:
    - `repo_Cargo.lock`: `9,170 -> 9,180`

- Restore:
  - rebuilt restore check confirmed the source tree is back on the retained point:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-oldestfirst.md`
    - `repo_Cargo.lock = 9,170`

- Useful conclusion:
  - the lockfile path does not want oldest-first current-window probing
  - this closes another best-level parser behavior on that family

## 2026-06-01 - Rejected lockfile-only `third_newest` current-entry sidecar

- Stayed on the retained lockfile-specific `step 2` baseline:
  - `repo_Cargo.lock = 9,170`
- Tried one structural current-entry representation change:
  - add a lockfile-only `third_newest` sidecar and probe it after the retained `second_newest` path

- Result:
  - exact byte-for-byte no-op
  - `repo_Cargo.lock`: stayed `9,170`

- Useful matcher result:
  - diagnostics stayed byte-for-byte identical too
  - so the extra current-entry representation never changed the chosen parse on live `Cargo.lock`

- Restore:
  - rebuilt restore check confirmed the source tree is back on the retained point:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-thirdnewest.md`
    - `repo_Cargo.lock = 9,170`

- Useful conclusion:
  - the lockfile gap is not waiting on a `third_newest` current-entry representation
  - current-entry `newest` / `second_newest` / `oldest` is effectively bounded for this family in the current matcher design

## 2026-06-01 - Rejected lockfile same-end repeat promotion

- Stayed on the retained lockfile-specific `step 2` baseline:
  - `repo_Cargo.lock = 9,170`
- Tried one more narrow offset-side branch:
  - for lockfile-like `DictionaryText`, prefer a repeat-offset candidate when it ends at the same byte as a non-repeat and only loses at most one match byte

- Result:
  - exact byte-for-byte no-op
  - `repo_Cargo.lock`: stayed `9,170`

- Useful matcher result:
  - diagnostics stayed byte-for-byte identical too
  - so this rule never fired on the live lockfile data

- Restore:
  - rebuilt restore check confirmed the source tree is back on the retained point:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-repeat-sameend.md`
    - `repo_Cargo.lock = 9,170`

- Useful conclusion:
  - the remaining lockfile `of_code=0` gap is not waiting on this same-end repeat-promotion family
  - next credible lockfile work still needs a different parse or entropy representation

## 2026-06-01 - Rejected `DictionaryText` 1-stream vs 4-stream literal choice up to 16 KiB

- Stayed on the retained lockfile-specific `step 2` baseline:
  - `repo_Cargo.lock = 9,170`
- Tried one more literal-side branch:
  - for `DictionaryText`, compare 1-stream vs 4-stream Huffman up to `16 KiB` of literals and keep the smaller estimate

- Result:
  - exact byte-for-byte no-op
  - `repo_Cargo.lock`: stayed `9,170`

- Useful inspect result:
  - the whole archive stayed byte-identical:
    - `literal_section_bytes`: `6,892`
    - `sequence_payload_bytes`: `2,258`
    - `sequences`: `848`
  - so the current encoder already kept the same literal-stream choice on this file

- Restore:
  - rebuilt restore check confirmed the source tree is back on the retained point:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-streamchoice16k.md`
    - `repo_Cargo.lock = 9,170`

- Useful conclusion:
  - the lockfile gap is not another stream-mode choice issue
  - next credible lockfile literal-side work must be a different table or representation family

## 2026-06-01 - Rejected broader predefined LL/ML tables for `DictionaryText`

- Stayed on the retained lockfile-specific `step 2` baseline:
  - `repo_Cargo.lock = 9,170`
- Tried one sequence-entropy branch:
  - allow `DictionaryText` LL/ML predefined tables up to `1024` sequences

- Result:
  - hard regression:
    - `repo_Cargo.lock`: `9,170 -> 9,408`

- Useful inspect result:
  - code histograms stayed identical
  - only sequence payload changed:
    - `2,258 -> 2,496`
  - so this branch only inflated table-description cost; it did not improve the parse at all

- Restore:
  - rebuilt restore check confirmed the source tree is back on the retained point:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-predef1024.md`
    - `repo_Cargo.lock = 9,170`

- Useful conclusion:
  - the lockfile gap is not another broader predefined-table window
  - next credible lockfile entropy work must be a different table or representation choice

## 2026-06-01 - Closed two more structural `Cargo.lock` branches with focused A/Bs

- Stayed on the retained lockfile-specific `step 2` baseline:
  - `repo_Cargo.lock = 9,170`
- Tested two structural parser families that were still ambiguous on the active parser shape:
  1. lockfile-specific current-entry long-hash, but with the gate and allocation actually enabled
  2. bypass the special DictionaryText repeat-only text pipeline and use the general text parser path

- Results:
  - both were exact byte-for-byte no-ops on the focused `repo_Cargo.lock` screen
  - useful closure on the long-hash family:
    - even with the path admitted, `current_long_hash_found = 0`
  - useful closure on the text-pipeline family:
    - matcher diagnostics stayed byte-for-byte identical too

- Useful conclusion:
  - the older lockfile long-hash rejection is now fully closed with stronger evidence
  - the remaining `Cargo.lock` gap is also not hiding behind the special DictionaryText text-repeat pipeline
  - the next credible branch is now even narrower:
    - sequence-entropy or a different parse representation
    - not another current-entry long-hash or text-pipeline toggle

## 2026-06-01 - Rejected three more focused `Cargo.lock` follow-ups; restored the retained `step 2` baseline

- Stayed on the retained lockfile-specific `step 2` point:
  - `repo_Cargo.lock = 9,170`
- Retested three narrow ideas against the active post-gate-fix parser shape:
  1. lockfile-specific repeat margin `+1`
  2. lockfile-specific same-start repeat-aware scoring
  3. lockfile-specific short-line non-repeat floor `5 -> 6`

- Results:
  - repeat margin `+1`: exact no-op
  - same-start repeat-aware scoring: exact no-op
  - floor `6`: hard regression
    - `repo_Cargo.lock`: `9,170 -> 9,246`
    - matcher diagnostics showed why:
      - `window_current_second_newest[0]`: `44 -> 0`

- Restore:
  - rebuilt restore check confirmed the source tree is back on the retained point:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-floor6.md`
    - `repo_Cargo.lock = 9,170`

- Useful conclusion:
  - the active post-gate-fix lockfile parser still does not want blanket repeat-margin tweaks
  - it still does not want same-start repeat-aware scoring
  - stronger lockfile floors are now clearly harmful because they suppress the retained `second_newest` wins

## 2026-06-01 - Bounded the retained lockfile probe-step family

- After retaining lockfile-specific probe step `2`, tested the next obvious point:
  - lockfile-specific probe step `3`
- Result:
  - `repo_Cargo.lock`: `9,170 -> 9,223`
- Rebuilt restore check confirmed the source tree is back on the retained point:
  - `repo_Cargo.lock = 9,170`

- Useful conclusion:
  - the active lockfile probe-density family is now bounded on the real parser shape:
    - dense step `1` is worse than retained
    - step `2` is the retained best point
    - step `3` is worse

## 2026-06-01 - Retained lockfile-specific probe step `2` after the `second_newest` gate fix

- Revisited the older lockfile probe-step family, but only after the retained `second_newest` gate fix changed the active parser shape for `Cargo.lock`.
- Narrow runtime change:
  - for content-detected `Cargo.lock`-like `DictionaryText`
  - keep the retained lockfile `second_newest` path
  - but relax the no-match probe step from dictionary-dense `1` back to `2`

- Focused result:
  - `repo_Cargo.lock`: `9,185 -> 9,170`
- Corrected broad-local result:
  - every other fixture stayed byte-identical
  - bytes-above-C on losers: `1,253 -> 1,238`

- Fresh diagnostics explain why this worked:
  - matcher:
    - `total_sequences`: `883 -> 848`
    - `repeat_current`: `48 -> 99`
  - archive:
    - `sequence_payload_bytes`: `2,418 -> 2,258`
    - `of_extra_bits`: `8,218 -> 7,164`
    - `of_codes 0`: now `70`
  - tradeoff:
    - literal payload rose a bit (`6,747 -> 6,892`)
    - but the reduced sequence/offset cost won overall

- Useful conclusion:
  - the old pre-gate-fix rejection of lockfile step `2` does not carry over unchanged
  - once the lockfile `second_newest` path is really active, a slightly less dense no-match probe is the better parser balance for that family

## 2026-06-01 - Rejected two more lockfile-specific follow-ups on top of the retained `second_newest` gate fix

- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,185`
- Tried one current-block indexing follow-up and one narrow OF-table follow-up, both only for the lockfile family.

- Rejected:
  1. fully dense post-match suffix insertion for `Cargo.lock`-like `DictionaryText`
  2. lockfile-only `offset_table_max_log = 8`

- Results:
  - both were exact byte-for-byte no-ops on the focused `repo_Cargo.lock` screen
  - `repo_Cargo.lock` stayed `9,185` in both cases

- Useful conclusion:
  - the remaining `Cargo.lock` gap is not waiting on denser post-match current-block indexing
  - it is also not fixed by simply broadening the OF table back from the retained `DictionaryText oflog7` point
  - next credible lockfile work still has to be a different parse/entropy representation, not another local insertion-density or OF-log toggle

## 2026-06-01 - Retained `second_newest` probe-admission bug fix, then cached the lockfile gate

- Found a real mismatch in the retained `second_newest` family:
  - the lockfile-specific `DictionaryText` sidecar was being tracked
  - the older Fastest small-block `second_newest` sidecar was also being tracked
  - but the actual probe sites were still guarded by `use_second_newest_probe` alone
- At level 1 Fastest, that meant the tracked sidecars were not being consulted through the intended gate.

- Fixed the probe sites to use `should_track_second_newest_for_current_entry()`.
- That immediately produced the expected lockfile movement on a focused A/B:
  - `repo_Cargo.lock`: `9,197 -> 9,185`

- The first version also caused a large `dict_dictionary.bin` CPU drift.
  - Root cause:
    - the new gate called `uses_dictionary_lockfile_second_newest_path()`
    - that rescanned the whole current `DictionaryText` block for lockfile markers at hot matcher sites
- Fixed that by caching the block-local lockfile classification in `add_data()`.

- Fresh retained diagnostics after the fix finally show the intended lockfile sidecar wins:
  - `window_current_second_newest[0] = 55`
  - `window_current_second_newest_zero_literals[0] = 26`
  - `window_current_second_newest_with_literals[0] = 29`

- Corrected `broad-local` result versus the retained `lockfile-secondnewest` baseline:
  - `repo_Cargo.lock`: `9,197 -> 9,185`
  - `build_ruzstd-cli`: `862,752 -> 854,529`
  - `decodecorpus_z000028`: `98,381 -> 95,230`
  - `decodecorpus_z000033`: `532,632 -> 530,433`
  - `decodecorpus_z000079`: `7,321 -> 7,322`
  - `dict_dictionary.bin`: unchanged at `20,160`
- New corrected-suite broad-local summary vs C `zstd -1`:
  - better / worse / equal: `32 / 11 / 4`
  - bytes-above-C on losing fixtures: `1,264 -> 1,253`

- Useful conclusion:
  - this was not another optional heuristic; it was a real bug in how the retained `second_newest` family was admitted at the probe sites
  - the lockfile-specific sidecar is now actually active
  - the older Fastest small-block `second_newest` path is now also active again through the intended gate

## 2026-06-01 - Rejected two more structural `Cargo.lock` matcher follow-ups after the retained `second_newest` win

- Refreshed the live retained archive shape for `repo_Cargo.lock`:
  - Rust retained:
    - `compressed_bytes=9197`
    - `literal_section_bytes=6766`
    - `sequence_payload_bytes=2411`
    - `decoded_literals=9756`
    - `sequences=879`
    - `match_bytes=22102`
  - C `zstd -1`:
    - `compressed_bytes=8088`
    - `literal_section_bytes=5975`
    - `sequence_payload_bytes=2092`
    - `decoded_literals=10360`
    - `sequences=784`
    - `match_bytes=21498`
- That confirms the retained `second_newest` branch reduced literals, but the remaining `Cargo.lock` gap is still broad.

- Tried the next two structural current-entry/current-window follow-ups:
  1. current-entry long-hash on the same lockfile path
  2. keep a closer current non-repeat candidate over farther `newest` / `oldest` window hits unless the farther match gains at least 2 bytes

- Results:
  - both were exact byte-for-byte no-ops on corrected `broad-local`
  - `repo_Cargo.lock` stayed `9,197` in both cases
- Restored the retained baseline and verified:
  - `benchmarks/reports/zstd-bench-restore-level1-lockfile-secondnewest-broad-local.md`

- Useful conclusion:
  - the retained `second_newest` branch was real
  - nearby current-entry long-hash and displacement rules are closed in this form
  - the next credible `Cargo.lock` branch is more likely sequence-entropy or a different parse representation, not another small current-window rule

## 2026-06-01 - Rejected lockfile-specific repeat-margin increase

- Compared retained `Cargo.lock` sequence histograms against C:
  - retained Rust still has no `of_code=0` population
  - C has a large one
- Tried the narrowest repeat-side follow-up:
  - only for `Cargo.lock`-like `DictionaryText`
  - increase the repeat-vs-normal match margin by `1`

- Result:
  - exact byte-for-byte no-op on corrected `broad-local`
  - `repo_Cargo.lock`: stayed `9,197`

- Useful conclusion:
  - the lockfile gap is not another local repeat-margin problem
  - the next credible branch remains sequence-entropy or a different parse representation

## 2026-06-01 - Rejected lockfile `ip+1` repeat lookahead after direct matcher inspection

- Added a useful test-only diagnostics improvement:
  - the ignored matcher inspectors now accept `RUZSTD_MATCHER_FILE_TYPE`
  - that made it possible to inspect the live retained `Cargo.lock` path as `DictionaryText`
- Fresh retained matcher diagnostics for `repo_Cargo.lock` showed:
  - repeat wins are sparse:
    - `repeat_current`: `18 / 24 / 8`
  - zero `repeat_next_position`
  - zero current-entry long-hash activity
  - emitted window wins are still dominated by current-entry `newest` / `oldest`
- Because the retained archive histograms still show a large repeat-offset mismatch versus C, tried the narrowest repeat-side follow-up:
  - enable the existing `ip+1` repeat-lookahead path for `Cargo.lock`-like `DictionaryText`

- Result:
  - exact byte-for-byte no-op on corrected `broad-local`
  - `repo_Cargo.lock`: stayed `9,197`

- Useful conclusion:
  - the missing repeat-offset wins are not unlocked by just turning on the existing `ip+1` repeat-lookahead machinery for lockfile text
  - next `Cargo.lock` work should stay on sequence-entropy or a different parse representation, not more small repeat-lookahead toggles

## 2026-06-01 - Rejected lockfile-specific repeat-vs-non-repeat same-start tie-break

- Stayed on the repeat-offset clue from the retained `Cargo.lock` histograms and matcher diagnostics.
- Tried a narrower scoring rule than the rejected repeat-margin changes:
  - only for `Cargo.lock`-like `DictionaryText`
  - if a repeat and non-repeat candidate start at the same place, prefer the repeat when it loses at most 1 match byte and saves at least 2 offset-code bits

- Result:
  - exact byte-for-byte no-op on corrected `broad-local`
  - `repo_Cargo.lock`: stayed `9,197`

- Useful conclusion:
  - the remaining lockfile gap is not another same-start repeat-vs-non-repeat scoring issue
  - the next credible `Cargo.lock` branch still has to move away from small matcher scoring tweaks and toward sequence-entropy or a different parse representation

## 2026-06-01 - Retained lockfile-like `DictionaryText` current-entry `second_newest`

- Stayed on the corrected `broad-local` suite and targeted the dominant remaining known-file-type loss:
  - `repo_Cargo.lock`
- The earlier obvious branches were already closed:
  - plain `Cargo.lock` remaps
  - lockfile-specific floor/probe cuts
  - wider same-start smaller-offset rule
  - `DictionaryText` text-repeat pipeline
  - small-sequence `DictionaryText oflog6`
- Moved to the first new structural matcher branch for that family:
  - keep the public file-type API unchanged
  - inside `DictionaryText`, detect `Cargo.lock`-like text by content:
    - repeated `[[package]]`
    - `name =`
    - `version =`
    - `checksum =`
  - only for that path, enable the existing current-entry `second_newest` sidecar at level 1
- Added focused matcher tests:
  - lockfile-like `DictionaryText` tracks the sidecar
  - binary/non-lockfile `DictionaryText` does not

- Result against the retained `codeprobe96k` corrected-suite baseline:
  - `repo_Cargo.lock`: `9,240 -> 9,197`
  - every other corrected-suite fixture stayed byte-identical
- New corrected-suite broad-local summary vs C `zstd -1`:
  - better / worse / equal: `32 / 11 / 4`
  - bytes-above-C on losing fixtures: `1,264`
- Current top losses:
  - `repo_Cargo.lock`: `+1,109`
  - `decodecorpus_z000079`: `+100`
  - `dict_dictionary.bin`: `+15`

- Useful conclusion:
  - the remaining `Cargo.lock` gap was not another threshold problem
  - a different current-entry representation does help this family
  - the next Cargo.lock branch should build on representation or sequence/offset structure again, not go back to the rejected threshold-only cuts

## 2026-06-01 - Retained suffix-based named-file matching plus `Cargo.lock -> DictionaryText`

- After correcting the broad-local suite, found that the next blocker in the file-type work was not the runtime itself but the named-file matcher:
  - synthetic benchmark filenames like `repo_.gitignore` and `repo_Cargo.lock` were not hitting exact filename rules at all
  - that meant some earlier “no-op” filename-policy experiments were being judged against the wrong classification path
- Retained a mapper improvement in `encoding/mod.rs`:
  - well-known named-file families now match by suffix when preceded by a clear separator (`_`, `-`, or `.`)
  - this preserves exact real filename matches and also makes synthetic benchmark names exercise the intended named-file policy
- On top of that retained helper, re-ran the `Cargo.lock` family:
  - `Cargo.lock -> DictionaryText`
  - `Cargo.lock -> CodeText`
- Both now move the target, unlike the earlier invalid no-op result:
  - `repo_Cargo.lock`: `9,255 -> 9,240`
- Kept `Cargo.lock -> DictionaryText` as the retained point:
  - it matches the measured byte win
  - it is the better semantic fit for the current stronger offset-aware starting point
- Rejected `Cargo.lock -> CodeText` as a separate retained point:
  - same byte gain, no broader benefit

- Refreshed current broad-local baseline after the retained point:
  - better / worse / equal: `31 / 12 / 4`
  - bytes-above-C on losing fixtures: `1,411`
- Current top losses:
  - `repo_Cargo.lock`: `+1,152`
  - `repo_compressed.rs`: `+104`
  - `decodecorpus_z000079`: `+100`

## 2026-06-01 - Corrected broad-local fixture collisions and exposed the real known-file-type gap

- Fixed the `broad-local` generator so repo-source fixture names are unique instead of silently overwriting each other.
  - Before:
    - manifest had duplicate `repo_Cargo.toml` entries
    - fixture directory only kept the last one written
  - After:
    - distinct fixtures now exist for:
      - `repo_Cargo.toml`
      - `repo_cli_Cargo.toml`
      - `repo_ruzstd_Cargo.toml`
      - `repo_ruzstd_fuzz_Cargo.toml`
      - `repo_ruzstd_fuzz_.gitignore`
- Added more known-file-type fixtures at the same time:
  - `repo_Cargo.lock`
  - `repo_ci.yml`
- Regenerated:
  - `benchmarks/fixtures/broad-local`
  - `benchmarks/manifests/broad-local.json`
- Refreshed live baselines:
  - `benchmarks/reports/zstd-bench-current-level1-broad-local.md`
  - `benchmarks/reports/zstd-bench-current-level1-fast.md`

- Corrected current broad-local level-1 summary vs C `zstd -1`:
  - better / worse / equal: `31 / 12 / 4`
  - bytes-above-C on losing fixtures: `1,426`
- Largest corrected-suite losses:
  - `repo_Cargo.lock`: `9,255` vs `8,088` (`+1,167`)
  - `repo_compressed.rs`: `13,111` vs `13,007` (`+104`)
  - `decodecorpus_z000079`: `7,321` vs `7,221` (`+100`)

- This changed the immediate priority:
  - the old known-file-type tail was not the real remaining gap
  - the corrected suite says `Cargo.lock` is now the biggest known-file-type miss by far

## 2026-06-01 - Rejected `Cargo.lock -> CodeText` and `Cargo.lock -> DictionaryText`

- After the corrected suite exposed `repo_Cargo.lock`, generated fresh archive inspections:
  - Rust current:
    - `compressed_bytes=9255`
    - `literal_section_bytes=6981`
    - `sequence_payload_bytes=2254`
    - `decoded_literals=10061`
    - `sequences=836`
    - `match_bytes=21797`
  - C `zstd -1`:
    - `compressed_bytes=8088`
    - `literal_section_bytes=5975`
    - `sequence_payload_bytes=2092`
    - `decoded_literals=10360`
    - `sequences=784`
    - `match_bytes=21498`
- That showed a broad mismatch:
  - Rust is over-sequenced
  - Rust also still pays much more literal payload
- Tried two pure filename-policy remaps in `encoding/mod.rs`:
  1. `Cargo.lock -> CodeText`
  2. `Cargo.lock -> DictionaryText`
- Results:
  - both were exact no-ops on the corrected baseline
  - `repo_Cargo.lock` stayed `9,255` in both cases
- Conclusion:
  - the `Cargo.lock` gap is real and important
  - but it is not solved by reusing the retained `CodeText` or `DictionaryText` starting points as-is

## 2026-05-31 - Rejected tiny ConfigText flat-distribution Huffman follow-up and plain `.toml -> CodeText` remap

- Stayed on known-file-type-first work after the rejected dictionary displacement rule.
- Refreshed current-vs-C archive inspections for the three remaining tiny config-side losers:
  - `repo_.gitignore`
    - Rust and C matched on:
      - `sequence_count=6`
      - `sequence_payload=17`
      - `literals_streams=1`
      - `ll/of/ml = predefined/predefined/predefined`
    - only the literal payload differed:
      - Rust `137`
      - C `129`
  - `dict_talk.service`
    - still a tiny parse-shape difference:
      - Rust `4` sequences / `130` literal bytes
      - C `3` sequences / `127` literal bytes
  - `repo_Cargo.toml`
    - still a broader parse difference:
      - Rust `51` sequences / `570` literal bytes / `1` stream
      - C `71` sequences / `520` literal bytes / `4` streams

- Tried a pure extension-based starting-point experiment first:
  - map `.toml` to `CodeText` instead of generic `ConfigText`
  - Result:
    - `repo_Cargo.toml`: `730 -> 732`
    - nothing else moved
  - Conclusion:
    - the retained `CodeText` starting point is not a better fit for `.toml` in this plain form

- Then tried a narrower literal-side follow-up for tiny `ConfigText`:
  - keep the smallest-table Huffman search active even for flat literal distributions
  - scope was limited to the tiny `ConfigText` literal path that still trails C
  - Result:
    - exact byte-for-byte no-op
    - `repo_.gitignore`: stayed `172`
    - `dict_talk.service`: stayed `160`
    - `repo_Cargo.toml`: stayed `730`
  - Conclusion:
    - the remaining tiny `ConfigText` literal gap is not hidden behind the current flat-distribution early return
    - do not retry this family in the same form

## 2026-05-31 - Rejected DictionaryText current-over-oldest offset-bit displacement rule

- Switched back to known-file-type-first work after the recent `ConfigText` literal no-ops.
- Refreshed current-vs-C archive inspection for `dict_dictionary.bin`:
  - Rust current:
    - `compressed_bytes=20175`
    - `literal_section_bytes=8973`
    - `sequence_payload_bytes=11182`
    - `decoded_literals=11071`
    - `sequences=4285`
    - `of_extra_bits=47698`
  - C `zstd -1`:
    - `compressed_bytes=20145`
    - `literal_section_bytes=11042`
    - `sequence_payload_bytes=9082`
    - `decoded_literals=16116`
    - `sequences=3461`
    - `of_extra_bits=36827`
- That kept the same interpretation:
  - the live dictionary gap is still over-sequenced and still too expensive on offsets
  - but the earlier blunt `oldest` penalties were already shown to be too coarse
- Tried a narrower `DictionaryText` scoring rule in `match_generator.rs`:
  - only when comparing a farther `oldest` window candidate against the current non-repeat candidate
  - keep the closer current candidate when:
    - the farther `oldest` gains less than `2` match bytes
    - and it costs at least `4` more offset-code bits
- Result versus the rebuilt retained live baseline:
  - `dict_dictionary.bin`: `20,160 -> 20,161`
  - every other broad-local fixture stayed exact
- Conclusion:
  - this current-vs-`oldest` displacement family is not the missing dictionary correction
  - the retained same-start smaller-offset rule is still the useful edge
  - do not retry this offset-bit-gated `oldest` displacement variant in the same form

## 2026-05-31 - Rejected `ConfigText` single-stream-vs-4-stream literal candidate selection

- After the retained small-`ConfigText` single-stream rule, refreshed the remaining tiny config losses:
  - `repo_Cargo.toml`: `730` vs C `726`
  - `repo_.gitignore`: `172` vs C `164`
  - `dict_talk.service`: `160` vs C `154`
- Fresh archive inspection said:
  - `repo_Cargo.toml` was the strongest candidate for another literal-mode refinement
  - C used a 4-stream literal section there while the retained Rust path used single-stream
- Tried a narrower literal-side change in `compressed.rs`:
  - keep the `ConfigText` small-literal single-stream rule as a preferred candidate
  - also estimate the normal 4-stream path
  - choose whichever estimated literal section is smaller
- Result:
  - exact byte-for-byte no-op on the retained `config-singlestream` baseline
  - `repo_Cargo.toml`: stayed `730`
  - `repo_.gitignore`: stayed `172`
  - `dict_talk.service`: stayed `160`
  - no broad-local fixture moved at all
- Conclusion:
  - the remaining `ConfigText` tail is not fixed by letting the retained single-stream override fall back to 4-stream on current literal-size estimates
  - do not retry this `ConfigText` literal stream-choice family without new evidence

## 2026-05-31 - Rejected adaptive Huffman weight-table description selection

- After the retained small-`ConfigText` single-stream rule, checked the remaining `.gitignore` gap more closely:
  - Rust and C already matched on:
    - sequence count
    - decoded literals
    - sequence payload bytes
  - so the remaining `+8` looked like literal-table-description overhead
- Tried a narrow Huff0 follow-up in `ruzstd/src/huff0/huff0_encoder.rs`:
  - build both existing weight-table encodings:
    - direct nibble encoding
    - FSE-compressed weights
  - emit whichever byte vector is smaller
- Validation:
  - focused Huff0 tests passed
  - broad-local A/B against the retained `config-singlestream` binary was an exact no-op
- Key results:
  - `repo_.gitignore`: stayed `172`
  - `dict_talk.service`: stayed `160`
  - `repo_Cargo.toml`: stayed `730`
  - `decodecorpus_z000079`: stayed `7,321`
  - `build_ruzstd-cli`: stayed `866,649`
- Conclusion:
  - the remaining `.gitignore` literal gap is not explained by direct-vs-FSE Huffman weight-table selection
  - do not retry this Huff0 table-description family without new evidence

## 2026-05-31 - Retained wider dense probing for short-line `CodeText` up to 64 KiB

- First checked `repo_compressed.rs`, now the largest known-file-type loser on the expanded broad-local suite.
- Fresh archive inspection against C on `repo_compressed.rs` showed:
  - Rust: `literal_section_bytes=4658`, `sequence_payload_bytes=8161`, `sequences=3408`
  - C: `literal_section_bytes=4392`, `sequence_payload_bytes=8339`, `sequences=3576`
- That said the residual was not a literal-table-search issue:
  - `CodeText -> exact Huffman search for all literal sections` was an exact no-op on the broad-local suite.
- Retained the more plausible matcher-side lever instead:
  - widened short-line `CodeText` dense probing from `10 KiB` to `64 KiB`
  - `ConfigText` stays at `8 KiB`
- Added a focused matcher test:
  - `large_code_text_blocks_keep_dense_probe_step`
- Broad-local retained result versus the `codeprobe10k` baseline:
  - `repo_compressed.rs`: `12,839 -> 12,695`
  - `repo_match_generator.rs`: `26,253 -> 26,192`
  - unchanged:
    - `decodecorpus_z000079`: `7,321`
    - `dict_dictionary.bin`: `20,160`
    - `repo_progress.rs`: `3,147`
    - `repo_benchmark_zstd.py`: `2,846`
- Refreshed current broad-local live summary vs C `zstd -1`:
  - better / worse / equal: `27 / 11 / 3`
  - bytes-above-C on losing fixtures: `185`
- Current top remaining losses:
  - `decodecorpus_z000079`: `+100`
  - `repo_progress.rs`: `+23`
  - `dict_dictionary.bin`: `+15`
  - `decodecorpus_z000059`: `+13`
  - `repo_Cargo.toml`: `+11`
- Reports:
  - retained A/B:
    - `benchmarks/reports/zstd-bench-compare-level1-codeprobe64k-broad-local.md`
  - refreshed live baselines:
    - `benchmarks/reports/zstd-bench-current-level1-broad-local.md`
    - `benchmarks/reports/zstd-bench-current-level1-fast.md`
  - retained binary:
    - `benchmarks/reports/ruzstd-cli-level1-codeprobe64k-retained`
- Conclusion:
  - the `CodeText` family still had meaningful room above `10 KiB`
  - `repo_compressed.rs` was matcher-side after all, but only for larger short-line code blocks

## 2026-05-31 - Retained lower non-repeat floor for small short-line `CodeText`

- Next checked `repo_progress.rs`, the next largest known-file-type loser after the retained `repo_compressed.rs` win.
- Fresh archive inspection against C on `repo_progress.rs` showed it was still matcher-side and under-sequenced:
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
- That pointed to a narrower parse change instead of another literal-side entropy change.
- Retained matcher change:
  - for short-line `CodeText` blocks up to `16 KiB`, use a `5`-byte non-repeat floor instead of `6`
  - larger `CodeText` blocks stay on the retained `6`-byte floor
- Added focused matcher coverage:
  - `small_code_text_blocks_use_lower_non_repeat_floor`
  - `large_code_text_blocks_keep_code_non_repeat_floor`
- Broad-local result versus the retained `codeprobe64k` baseline:
  - `repo_progress.rs`: `3,147 -> 3,125`
  - `repo_benchmark_zstd.py`: `2,846 -> 2,814`
  - `repo_main.rs`: `2,128 -> 2,125`
  - unchanged:
    - `repo_compressed.rs`: `12,695`
    - `repo_match_generator.rs`: `26,192`
    - `decodecorpus_z000079`: `7,321`
    - `dict_dictionary.bin`: `20,160`
- Refreshed current broad-local live summary vs C `zstd -1`:
  - better / worse / equal: `28 / 10 / 3`
  - bytes-above-C on losing fixtures: `162`
- Current top remaining losses:
  - `decodecorpus_z000079`: `+100`
  - `dict_dictionary.bin`: `+15`
  - `decodecorpus_z000059`: `+13`
  - `repo_Cargo.toml`: `+11`
  - `repo_.gitignore`: `+8`
- Reports:
  - retained A/B:
    - `benchmarks/reports/zstd-bench-compare-level1-code-smallfloor5-broad-local.md`
  - refreshed live baselines:
    - `benchmarks/reports/zstd-bench-current-level1-broad-local.md`
    - `benchmarks/reports/zstd-bench-current-level1-fast.md`
  - retained binary:
    - `benchmarks/reports/ruzstd-cli-level1-code-smallfloor5-retained`
- Conclusion:
  - the remaining small `CodeText` residuals were still short-line matcher-side
  - the useful split is now:
    - small short-line `CodeText` gets the `5`-byte floor
    - larger short-line `CodeText` stays on the retained `6`-byte floor

## 2026-05-31 - Rejected two small-`ConfigText` matcher-side follow-ups

- Tried a dictionary-style same-start smaller-offset preference on small `ConfigText` blocks:
  - scope: `ConfigText`, up to `16 KiB`, same-start only, save at least 2 offset-code bits, lose at most 1 match byte
  - result: complete no-op on the expanded broad-local suite
  - `repo_Cargo.toml`: stayed `737`
  - `repo_.gitignore`: stayed `172`
- Tried enabling the text repeat pipeline on small `ConfigText` blocks:
  - scope: `ConfigText`, up to `16 KiB`
  - result: byte no-op on the expanded broad-local suite and slight fast-screen CPU drift on already-winning binaries
  - `repo_Cargo.toml`: stayed `737`
  - `repo_.gitignore`: stayed `172`
  - `build_ruzstd-cli` CPU drifted `0.06s -> 0.07s`
- Conclusion:
  - the remaining small `ConfigText` losses were not waiting on another matcher-side rule
  - the next credible move on this family was literal-side

## 2026-05-31 - Retained single-stream Huffman for small `ConfigText` literal sections

- After the matcher-side no-ops, inspected the residual `ConfigText` shape:
  - `repo_Cargo.toml` was still under C by `11` bytes
  - `repo_.gitignore` was still under C by `8` bytes
- Retained literal-side change:
  - for `CompressionFileType::ConfigText` at level 1, compressed literal sections up to `1024` literals may force the single-stream Huffman path
- Added focused block-encoder coverage:
  - `fastest_config_text_enables_small_single_stream_huffman_override`
  - `forced_single_stream_huffman_uses_single_stream_size_format`
- Broad-local result versus the retained `code-smallfloor5` baseline:
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
- Unchanged:
  - `decodecorpus_z000079`: `7,321`
  - `dict_dictionary.bin`: `20,160`
  - `repo_progress.rs`: `3,125`
  - `repo_.gitignore`: `172`
- Refreshed current broad-local live summary vs C `zstd -1`:
  - better / worse / equal: `30 / 9 / 2`
  - bytes-above-C on losing fixtures: `154`
- Current top remaining losses:
  - `decodecorpus_z000079`: `+100`
  - `dict_dictionary.bin`: `+15`
  - `decodecorpus_z000059`: `+13`
  - `repo_.gitignore`: `+8`
  - `dict_talk.service`: `+6`
  - `decodecorpus_z000053`: `+5`
  - `repo_Cargo.toml`: `+4`
- Conclusion:
  - the `ConfigText` family still had literal-side room even after the matcher path looked exhausted
  - `repo_Cargo.toml` is now nearly closed, and many small service files moved from “behind C” to “ahead of C”

## 2026-05-31 - Rejected two more follow-ups after the retained small-`ConfigText` single-stream literal rule

- Refreshed the two smallest remaining known-file-type losses on the retained tree:
  - `repo_.gitignore`
    - Rust:
      - `literal_section_bytes=137`
      - `sequence_payload_bytes=17`
      - `decoded_literals=180`
      - `sequences=6`
    - C:
      - `literal_section_bytes=129`
      - `sequence_payload_bytes=17`
      - `decoded_literals=180`
      - `sequences=6`
    - conclusion:
      - exact same parse and table modes
      - the remaining `+8` is literal-payload-side only
  - `dict_talk.service`
    - Rust:
      - `literal_section_bytes=130`
      - `sequence_payload_bytes=12`
      - `decoded_literals=152`
      - `sequences=4`
    - C:
      - `literal_section_bytes=127`
      - `sequence_payload_bytes=9`
      - `decoded_literals=159`
      - `sequences=3`
    - conclusion:
      - only a very small parse difference remains

- Rejected:
  1. treat small `ConfigText` blocks (`<=1024` bytes) as text for matching
     - no gain on the target fixtures:
       - `repo_.gitignore`: stayed `172`
       - `dict_talk.service`: stayed `160`
       - `repo_Cargo.toml`: stayed `730`
     - one already-winning config fixture regressed by 1 byte:
       - `dict_glustereventsd.service`: `285 -> 286`
  2. broader dictionary smaller-offset rule without the same-start requirement
     - scope:
       - `DictionaryText`
       - smaller offset must save at least 4 offset-code bits
       - can lose at most 1 match byte
     - result:
       - `dict_dictionary.bin`: `20,160 -> 20,161`

- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-config-forcedtext-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-dict-broader-offset-broad-local.md`

- Conclusion:
  - the remaining `repo_.gitignore` gap is not another matcher-path miss; it is literal-payload-side
  - the retained dictionary same-start smaller-offset rule is already at the useful edge

## 2026-05-31 - Rejected `CodeText` exact Huffman search for all literal sections

- Tried:
  - `CompressionFileType::CodeText` -> exact Huffman table search for all literal sections at level 1
- Motivation:
  - `repo_compressed.rs` was paying `266` more literal-section bytes than C on archive inspection
- Result:
  - exact byte-for-byte no-op on the expanded broad-local suite
  - `repo_compressed.rs`: stayed `12,839`
  - `repo_progress.rs`: stayed `3,147`
  - `repo_benchmark_zstd.py`: stayed `2,846`
- Report:
  - `benchmarks/reports/zstd-bench-compare-level1-code-allsections-broad-local.md`
- Conclusion:
  - `repo_compressed.rs` was not blocked on literal Huffman table search
  - do not retry this `CodeText` literal-side expansion in the same form

## 2026-05-31 - Retained wider dense probing for short-line `CodeText` up to 10 KiB

- Widened the retained dense short-line `CodeText` probe cutoff:
  - `CodeText`: `8 KiB -> 10 KiB`
  - `ConfigText`: unchanged at `8 KiB`
- Motivation:
  - after expanding the broad-local corpus, two explicit `CodeText` losers landed just above the old cutoff:
    - `repo_progress.rs` (`8,784` bytes)
    - `repo_benchmark_zstd.py` (`8,997` bytes)
- Fast screen result:
  - `repo_progress.rs`: `3,168 -> 3,147`
  - `repo_benchmark_zstd.py`: `2,865 -> 2,846`
  - unchanged:
    - `build_ruzstd-cli`: `866,649`
    - `decodecorpus_z000079`: `7,321`
    - `dict_dictionary.bin`: `20,160`
    - `repo_compressed.rs`: `12,839`
    - `repo_main.rs`: `2,128`
- Broad-local result:
  - `benchmarks/reports/zstd-bench-compare-level1-codeprobe10k-broad-local.md`
  - bytes-above-C on losing fixtures: `312 -> 272`
  - better / worse / equal stayed `26 / 12 / 3`
- Refreshed current live reports:
  - `benchmarks/reports/zstd-bench-current-level1-broad-local.md`
  - `benchmarks/reports/zstd-bench-current-level1-fast.md`
- Conclusion:
  - the retained dense short-line `CodeText` path still had room slightly above `8 KiB`
  - this is a `CodeText`-specific win, not a new `ConfigText` or `Unknown` direction

## 2026-05-31 - Rejected `ConfigText` `offset_table_max_log = 7` and expanded the known-file-type corpus

- Tried one more explicit file-type entropy move:
  - `CompressionFileType::ConfigText` uses `offset_table_max_log = 7` at level 1
- Result on the config-heavy fast screen:
  - exact no-op
  - `dict_kmod-static-nodes.service`: `486`
  - `dict_fstrim.service`: `299`
  - `dict_systemd-udev-settle.service`: `560`
  - `dict_NetworkManager-dispatcher.service`: `381`
  - guardrails unchanged:
    - `build_ruzstd-cli`: `855,679`
    - `decodecorpus_z000079`: `7,321`
    - `dict_dictionary.bin`: `20,160`
    - `repo_main.rs`: `2,105`
- Report:
  - `benchmarks/reports/zstd-bench-compare-level1-config-oflog7-fast.md`
- Conclusion:
  - `ConfigText` is not waiting on the same offset-FSE-log reduction that helped `DictionaryText` and `Unknown`

- Also expanded `tools/prepare_benchmark_suites.py` so `broad-local` covers more explicit known-file-type fixtures:
  - `.gitignore`
  - extra `Cargo.toml` files
  - `cli/src/progress.rs`
  - `ruzstd/src/encoding/blocks/compressed.rs`
  - benchmark tooling scripts
  - a larger spread of `.service` files
- Regenerated the suite and refreshed the current live broad-local baseline:
  - report: `benchmarks/reports/zstd-bench-current-level1-broad-local.md`
  - current summary vs C `zstd -1` on the expanded suite:
    - better / worse / equal: `26 / 12 / 3`
    - bytes-above-C on the losing fixtures: `312`
- Largest remaining losses are now:
  - `decodecorpus_z000079`: `+100`
  - `repo_compressed.rs`: `+87`
  - `repo_progress.rs`: `+44`
  - `repo_benchmark_zstd.py`: `+20`
  - `dict_dictionary.bin`: `+15`

## 2026-05-31 - Retained predefined LL/ML table eligibility for small compressed-literals `ConfigText` blocks

- Extended the retained level-1 small compressed-literals predefined LL/ML table gate from `CompressionFileType::Unknown` to also cover `CompressionFileType::ConfigText`.
- Scope:
  - level 1 only
  - `ConfigText` only
  - compressed-literals blocks only
  - at most `64` sequences
- Motivation:
  - the remaining service/config residuals still looked like sequence-table overhead rather than matcher drift
  - `dict_kmod-static-nodes.service` was the clearest signal
- Retained result versus the retained `Unknown predef64 compressed-literals` baseline:
  - `dict_kmod-static-nodes.service`: `497 -> 486`
  - `dict_NetworkManager-dispatcher.service`: `391 -> 381`
  - `dict_fstrim.service`: `308 -> 299`
  - `dict_systemd-coredump@.service`: `686 -> 682`
  - `dict_systemd-udev-settle.service`: `568 -> 560`
  - broad-local better / worse / equal vs C: `16 / 13 / 3 -> 18 / 11 / 3`
  - broad-local bytes-above-C on losing fixtures: `192 -> 155`
- Fast guardrails stayed exact:
  - `decodecorpus_pack.bin`: `5,319,265`
  - `json_logs_32m.jsonl`: `690,084`
  - `decodecorpus_z000079`: `7,321`
  - `dict_dictionary.bin`: `20,160`
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-config-predef64-complit-vsretained-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-config-predef64-complit-vsretained-fast.md`
  - retained binary: `benchmarks/reports/ruzstd-cli-level1-config-predef64-complit-retained`

## 2026-05-31 - Rejected a large-`Unknown` `RepeatNextPosition`-only repeat-margin bonus

- Tried one narrower follow-up inside the dominant retained large-`Unknown` repeat family:
  - keep the retained large-`Unknown` repeat-vs-normal margin normally
  - add one extra margin point only when the repeat candidate is the `ip+1` `RepeatNextPosition` case
- Motivation:
  - live matcher diagnostics still show `repeat_next_position_selected_without_current_candidate` dominating `decodecorpus_z000079`
  - this was the narrowest way to distinguish that family without broadening the generic repeat bias
- Result on the fast screen:
  - `decodecorpus_z000079`: `7,321 -> 7,331`
  - `build_ruzstd-cli`: `855,679 -> 855,745`
  - `dict_dictionary.bin`: unchanged at `20,160`
  - `repo_main.rs`: unchanged at `2,105`
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-nextrepmargin-fast.md`
  - restore:
    - `benchmarks/reports/zstd-bench-restore-level1-unknown-nextrepmargin-fast.md`
- Conclusion:
  - the remaining `z000079` gap is not waiting on another local `RepeatNextPosition` repeat-bias increase
  - the next credible move still needs a different large-`Unknown` parse or sequence/offset representation

## 2026-05-31 - Rejected three follow-ups after the retained `ConfigText` LL/ML gate

- Tried:
  1. large `Unknown` equal-length smaller-offset tie-break
  2. `ConfigText` compressed-literals OF predefined-table gate up to `64` sequences
  3. narrower `ConfigText` compressed-literals OF predefined-table gate up to `24` sequences
- Results:
  - large `Unknown` tie-break:
    - exact byte-for-byte no-op
    - `decodecorpus_z000079` stayed `7,321`
  - `ConfigText` OF `<=64`:
    - no `dict_kmod-static-nodes.service` improvement
    - regressed:
      - `dict_systemd-coredump@.service`: `682 -> 688`
      - `dict_systemd-udev-settle.service`: `560 -> 562`
  - `ConfigText` OF `<=24`:
    - still no `dict_kmod-static-nodes.service` improvement
    - only `dict_fstrim.service` moved `299 -> 298`
- Conclusion:
  - the remaining `kmod`-style config residual is not waiting on OF predefined-table eligibility
  - the remaining large-`Unknown` gap is not waiting on an equal-length smaller-offset tie-break

## 2026-05-31 - Rejected two more large-`Unknown` follow-ups

- Tried:
  1. widen the retained `Unknown` compressed-literals predefined LL/ML gate from `64` to `256` sequences
  2. keep the retained large-`Unknown` `newest +2` rule normally, but require `+3` when the farther `newest` candidate also cost at least 4 more offset-code bits
- Results:
  - `Unknown` predef256 compressed-literals:
    - `decodecorpus_z000059`: `711 -> 826`
    - `decodecorpus_z000079`: unchanged at `7,321`
  - conditional stronger `newest` rule:
    - `decodecorpus_z000079`: unchanged at `7,321`
    - `build_ruzstd-cli`: `855,679 -> 855,725`
- Conclusion:
  - the retained `Unknown` compressed-literals LL/ML gate is bounded at `64`
  - the remaining large-`Unknown` gap still does not move on another `newest`-side local threshold

## C zstd Guidance Used

- Repeat offsets: mirror the decoder/spec rules and the C compressor's repeat-code choice/update behavior.
- FSE table modes: follow the conservative fast-path idea from C zstd: use predefined tables for tiny sequence counts, repeat previous tables only when the symbols are valid, and avoid broad heuristics without cost modeling.
- Literal compression: follow C zstd's fast-level guardrails for small/repeated literals, single-stream Huffman below 256 bytes, and the `(srcSize >> 6) + 2` minimum literal gain before accepting a Huffman literal section.
- Huffman work remains C-guided but not C-cloned: compare table-depth choices, literal mode selection, repeat-table reuse, and payload-cost guards against C zstd, then keep the Rust shape idiomatic and covered.
- C zstd's level table and strategy selection are used as level-design guidance: `ZSTD_getCParams_internal()` maps levels to parameter rows, and the compressor then selects fast, greedy, lazy, row-matchfinder, or optimal paths from those parameters. The local C source inspected is from `zstd-sys-2.0.16+zstd.1.5.7` under Cargo's registry cache.
- C zstd's literal compressor tightens literal-compression thresholds as strategy increases and enables optimal Huffman depth at higher strategies. The Rust branch now mirrors that direction by keeping level 1 conservative and moving broad exact Huffman table-depth search to `CompressionLevel::Best`.
- C zstd `-4` on `decodecorpus_pack.bin` reports `windowLog 21` and `strategy ZSTD_dfast`, while `-1` reports `windowLog 19` and `strategy ZSTD_fast`. Higher Rust levels should therefore explore larger history and double-fast/secondary-candidate ideas, but only with guardrails so higher levels do not regress binary-like inputs unnecessarily.
- SIMD/hardware-vector work should target the matcher later, especially match extension/comparison. The repeat-offset and table-selection paths are scalar/control-heavy and are not good SIMD candidates.
- On the current `rustc 1.94.1` toolchain, `std::simd`/portable SIMD is still unstable, so direct SIMD in the encoder would require nightly, an additional dependency, or unsafe target intrinsics. Those conflict with the current safe-Rust/no-new-risk constraints unless a future change explicitly revisits that tradeoff.

## Completed

- Added encoder repeat-offset state to `CompressState`.
- Reset repeat-offset state and previous FSE table state at the start of each frame.
- Encoded offsets using repeat-code values when they match the current offset history.
- Added tests for:
  - repeat offsets with non-zero literal lengths,
  - shifted repeat offsets with zero literal lengths,
  - new offset history updates,
  - tiny-sequence predefined FSE table selection,
  - small-block repeat FSE table selection.
- Avoided per-block temporary code vectors for FSE table selection.
- Added a scalar `OffsetHistory` struct instead of a `[u32; 3]` array in encoder state.
- Found and fixed a correctness issue in raw fallback: if `compress_fastest` builds a compressed block and then discards it as raw, it now restores FSE and repeat-offset encoder history because the decoder will not see the discarded compressed block.
- Deferred committing a new Huffman table until `compress_fastest` knows the compressed block will actually be emitted. This avoids cloning `HuffmanTable`, which is not `Clone`, and avoids committing entropy history for a discarded raw fallback block.
- Added encoder support for RLE sequence table modes and a decoder-facing RLE bitstream round-trip test.
- Increased the default fastest matcher window from one 128 KiB block to four 128 KiB blocks, matching the 512 KiB level-1 window observed from C zstd on the repeated-text fixture.
- Fixed match-length code 52 encoding, which had the wrong baseline and corrupted streams once cross-block matches exposed match lengths above 65,538 bytes.
- Added a cross-block repetitive-data test that verifies both the Rust decoder and C zstd decoder can decode the emitted stream and that compression stays compact.
- Added a sampled incompressibility gate so random-looking blocks skip expensive match search and are emitted raw while still being committed for future history.
- Added a matcher fast path for incompressible raw blocks that marks the block processed without indexing every suffix. This preserves safe behavior for custom matchers via a default trait method while allowing the default matcher to avoid wasted history work.
- Added shortest-form raw literals headers for 0-31 and 32-4095 byte raw literal sections, matching the Zstd format and reducing per-block overhead.
- Enabled overlapping same-block match extension using the current block's original bytes. This lets the matcher emit long matches with small offsets instead of artificial doubling sequences.
- Reworked overlapping match extension to compare contiguous slices in chunks instead of one byte at a time, staying in safe Rust and adding focused tests for same-block overlap, previous-window lookups, and chunk-boundary mismatches.
- Added matcher-side repeat-offset probing. The default matcher now stays synchronized with the encoder repeat-offset history, probes repeat offsets even when no suffix entry exists, and restores matcher repeat-offset state when a compressed attempt is discarded as raw.
- Added safe backward extension for hash-table match candidates, following C zstd's fast parser behavior of moving a match start back toward the current sequence anchor when the preceding bytes also match.
- Replaced the rough Huffman previous-table reuse heuristic with an exact encoded-size estimate. The estimator is covered against the real single-stream and four-stream Huffman encoders, and avoids full trial encoding of the literal payload.
- Added a text-like block classifier that uses a longer non-repeat match threshold on mostly printable blocks while preserving short non-repeat matches for binary-looking blocks. Thresholds 7, 8, 9, 10, and 12 were benchmarked; 10 was the best measured aggregate point on the current fixture set.
- Broadened predefined FSE table use from tiny blocks to small non-RLE sequence blocks. A broad "always predefined when possible" experiment heavily regressed JSON, so this stays intentionally narrow.
- Added repeat-offset-biased match selection. Repeat candidates may win when they are only slightly shorter than a normal match; margins 1, 2, 3, and 4 were benchmarked, and margin 2 was the best measured aggregate point.
- Added matcher-internal tests for the compact two-candidate suffix store so the `oldest`/`newest` invariant is covered directly.
- Added fastest-compressor mixed-frame regression tests that round-trip through both the Rust decoder and C zstd decoder. These cover text-like compressed blocks, binary-looking blocks, incompressible/raw blocks, and reuse of repetitive history after an incompressible block.
- Added a repeat-offset early-exit heuristic: if a repeat-offset candidate already reaches the block end, or has at least 10 bytes, the matcher skips hash-table search for that position. Thresholds 5, 10, 16, and 64 were benchmarked; 10 gave the best measured CPU/size balance while keeping every fixture smaller than C zstd.
- Added focused tests for the repeat-offset early-exit decision.
- Added sparse suffix indexing for long emitted matches, following C zstd's fast parser shape of maintaining only a few hash entries around a long match instead of indexing every byte skipped by the match. Dense limits 64, 128, and 256 were benchmarked; 128 kept the JSON size/CPU balance from the repeat early-exit work and made the repeated-text fixture effectively C-speed.
- Added focused tests that short match ranges are still indexed densely while long match ranges are indexed sparsely.
- Added a repeat-offset minimum-match precheck. Obvious repeat misses now compare the first minimum-match bytes before entering the full cross-window match-length loop; boundary-crossing cases fall back to the existing matcher path. This keeps output bytes unchanged while reducing repeat-probe CPU.
- Added focused tests for repeat-offset precheck accept/reject behavior.
- Inlined the small hot-path match extension helpers after profiling showed the repeat-offset precheck and relative-slice lookup visible as separate symbols. This keeps output bytes unchanged and reduces matcher call overhead in the release build.
- Added an exact minimum-match precheck for hash-table candidates, matching C zstd's fast-parser shape of checking candidate bytes before full match extension. Hash collisions now avoid the offset-based match-length path. This keeps output bytes unchanged and reduces decodecorpus CPU.
- Added focused tests for hash-candidate precheck accept/reject behavior.
- Added a C-fast-style no-match probe step. After a miss, the matcher indexes the next suffix and advances by two probes when doing so would not skip over an immediate repeat-offset match. This trades a small decodecorpus size loss for materially lower decodecorpus CPU and better JSON size.
- Added focused coverage that the no-match step does not skip the next repeat-offset match.
- Replaced hot repeat-offset candidate iterator chains in the matcher with fixed arrays and explicit loops. This keeps output bytes unchanged while reducing iterator overhead in repeat probing and no-match skip guards.
- Simplified the no-match skip guard so it checks repeat-offset bytes directly instead of building a temporary match context. This keeps output bytes unchanged; benchmark medians were neutral, but the hot path is simpler and avoids unnecessary context construction.
- Inlined the remaining hot candidate helpers after profiling showed `match_candidate` and `repeat_offset_can_match_at` as visible call targets. This keeps output bytes unchanged and reduces decodecorpus CPU slightly.
- Switched deterministic FSE/Huffman table-construction sorts to unstable sorting where explicit keys fully define the order. This keeps output bytes unchanged and avoids stable-sort overhead in entropy setup.
- Added a text-only wider no-match probe step. Text-like blocks now use step 3 while binary-looking blocks keep step 2; this keeps the global step-3 decodecorpus regression out while recovering its JSON size win.
- Added focused tests for text and binary no-match probe step selection.
- Added focused Huffman tests for equal-count rank-limited weight assignment and deterministic length-limited code lengths. These lock down tie behavior before any future table-construction optimization.
- Removed `unwrap`/`expect`-style invariant handling from production matcher code. The suffix index and committed-window invariants now use explicit safe `match` branches with cold panic paths, keeping the hot path clear without introducing `unsafe`.
- Removed a redundant literal-statistics pass by tracking the largest symbol count while building the histogram, and simplified Huffman encoded-length estimation to a direct loop. This keeps output bytes unchanged while reducing entropy-path bookkeeping.
- Cached the encoder Huffman table's actual maximum code length when building the table, avoiding a scan over `codes` each time table-description weights are generated. Added a focused invariant test so the cached value stays tied to the generated codes.
- Removed the temporary compressed-block buffer in the fastest path. Compressed attempts now write directly behind a 3-byte header placeholder in the caller's output buffer, then either patch the compressed header or truncate and emit raw fallback after restoring entropy/repeat state.
- Added conservative initial capacities for per-block literals and sequence buffers to avoid early growth in compressed blocks without reserving a full 128 KiB block.
- Kept repeat-offset probe candidate selection in `usize` until sequence emission. The repeat-candidate order only depends on whether the current literal length is zero, so this avoids a hot checked `u32` conversion while preserving checked conversion at the bitstream boundary. Added focused tests for zero-literal and non-zero-literal repeat-candidate ordering.
- Changed suffix-store reuse to clear only touched hash slots instead of resizing the whole slot table to `None` for each returned block. This follows C zstd's long-lived hash-table shape while keeping stale entries impossible in safe Rust. Touched slot indexes are stored as `u32`, and a 32K touched-slot threshold falls back to full sequential clearing for dense blocks to limit RSS growth. Focused tests cover stale-candidate removal, reinsertion after clear, full-clear mode, and the threshold switch.
- Added a direct repeat-history update path for the matcher. The matcher only needs to keep its repeat probes synchronized, so it now updates the three repeat offsets directly instead of calling `encode_offset_value()` and discarding the encoded value. The sequence encoder still uses `encode_offset_value()` at the bitstream boundary. Added equivalence tests against the encoded update path for literal and zero-literal matches.
- Changed negative relative window lookups to scan previous window entries newest-first, skipping the current block because non-negative relative lookups already handle current-block matches. Cross-block repeat and hash matches most often target the most recent previous block, matching the C fast parser's prefix-oriented shape. Added a focused test with two previous entries so most-recent previous-window matching stays covered.
- Cached the encoder FSE table accuracy log at table construction time and used that cached value when flushing sequence states. Added a focused invariant test for the predefined tables so `acc_log` stays tied to `table_size`.
- Changed sparse indexing after long matches to store the final sparse hash at `match_end - 2`, matching the C fast parser's post-match hash fill shape. Added focused coverage that long matches index exactly the start, start+2, and end-2 positions.
- Carried verified minimum-match prefixes into full match-length scans so accepted repeat and hash candidates do not compare the first five bytes twice. Added focused tests for the normal skipped-prefix case and the previous-window boundary fallback.
- Replaced repeated stable sorting in Huffman length-limited tree construction with a deterministic min-heap. Existing Huffman tie-behavior tests cover the ordering invariants, and a same-window direct benchmark against the previous commit confirmed exact bytes with decodecorpus CPU slightly better and JSON neutral.
- Cached resolved FSE table references once per sequence section before encoding FSE states, avoiding repeated table-mode enum matching in the per-sequence hot path. Existing compressed-block and end-to-end tests cover the emitted bitstream.
- Cached the common literal-length and match-length sequence code ranges in const lookup tables. Added an exhaustive helper-level test over the cached ranges plus the next uncached boundary, using independently stated spec ranges so table-generation mistakes are caught.
- Removed the redundant modulo from suffix hash lookup. The shifted multiplicative hash is already bounded by `len_log`, matching C zstd's mask-style hash-table indexing while staying safe for the existing non-power-of-two test capacities.
- Added a same-block forward match-length fast path. When the match source is already in the current block, the matcher now compares the current-block source and target slices directly with the existing safe chunked prefix comparison instead of repeatedly resolving relative window slices. This follows C zstd's contiguous `ZSTD_count()` shape while preserving overlap behavior.
- Preallocated a modest 1024 entries for suffix-store touched-slot tracking. This avoids first-use growth in the touched-slot clearing path while staying well below the 32K full-clear threshold and adding only a small bounded allocation per suffix store.
- Added focused matcher coverage for same-block minimum-match precheck hits and misses. This locks down repeat-offset precheck behavior even though the generic relative-window implementation remains the fastest measured runtime path.
- Replaced the suffix-candidate iterator chain in the matcher window-search loop with explicit newest/oldest candidate checks through a small helper. This preserves the existing two-candidate order, avoids iterator setup in the dominant matcher loop, and adds focused helper coverage for best-candidate updates and block-end early exits.
- Replaced sequence-emission repeat-offset encoding's temporary repeat-candidate array search with direct branch checks. Existing offset-history tests cover the repeat-code semantics, and the now-unused array helper was removed.
- Inlined the two safe `usize` to `u32` offset boundary helpers used during sequence emission. This preserves the public `Sequence` offset type and matcher `usize` internals while letting the optimizer fold the checked conversions into their callers.
- Hoisted the current block length into a local scalar in the matcher loop, mirroring the C fast parser's local `iend`/`ilimit` style and avoiding repeated `last_entry.data.len()` reads in the dominant path.
- Removed production `unwrap()` calls from Huffman length-limited tree construction and rank-limited weight distribution. These invariants now use explicit branches with a cold panic helper, while tests cover deterministic tied-count behavior plus representative bounded/prefix-free Huffman tables.
- Added encoder emission for RLE literal sections when the literal payload for a compressed block is one repeated byte. This matches an existing decoder-supported Zstd literal section mode and adds both literal-section decoder coverage and a full frame round-trip through the Rust and C zstd decoders.
- Lowered the fastest encoder's literal-compression threshold from more than 1024 literals to more than 63 literals, matching C zstd's `COMPRESS_LITERALS_SIZE_MIN` heuristic. Added a focused small-literal compressed-block test that verifies Huffman literal emission and round-trips the full frame through both the Rust and C zstd decoders.
- Lowered the literal-compression threshold to more than 6 literals when a previous Huffman table is available, matching C zstd's repeat-table `minLitSize` heuristic. Added a full-frame Rust/C decoder round-trip showing small repeated literal payloads can use RLE once the previous table makes the smaller threshold valid.
- Added an early raw-literals fallback when the exact Huffman encoded-size estimate plus literal-section header cannot beat raw literals. This mirrors C zstd's no-gain entropy decision and avoids doing the actual Huffman encode only to rewind. Added a full-frame Rust/C decoder round-trip for a high-alphabet payload that passes the histogram gate but stays raw by estimate.
- Fixed frame-header descriptor emission for 8-byte frame content sizes. The encoder now writes FCS flag 3 when `find_min_size()` returns 8, and focused coverage verifies that a large single-segment frame content size round-trips through the local frame-header parser.
- Added one-byte read lookahead in frame compression so exact block-sized inputs mark the full block as final instead of emitting an extra empty raw block. Focused tests cover both the no-empty-final-block case and preserving the lookahead byte as the first byte of the next block.
- Added a hot BitWriter path for writes that exactly fill the 64-bit staging buffer, avoiding the cold overflow helper in that common boundary case. Added focused bit-level coverage that exact 64-bit fills flush correctly and preserve following writes.
- Split suffix hashing into a precomputed five-byte key value plus per-store slot mapping. Window search now reads the current suffix bytes once and reuses that value across all window entries, matching C zstd's local-hash shape while preserving each store's own table size. Added focused coverage that precomputed key lookup matches normal suffix lookup.
- Changed sequence bitstream encoding's reverse walk from a reversed range iterator to an explicit countdown loop, matching C zstd's indexed reverse-loop shape. Existing compressed-block and end-to-end Rust/C decoder tests cover the emitted bitstream.
- Tightened focused coverage after the coverage audit: suffix-store zero index candidates now round-trip through the one-based `NonZeroU32` representation, empty committed matcher entries emit no sequences and can be followed by normal data, and mixed predefined sequence modes round-trip through the decoder.
- Hardened `compress_fastest()` for empty direct block inputs before the RLE probe indexes byte 0. The normal frame compressor already handles empty frames before block compression, so this is a focused correctness/idiomatic-safety guard rather than a fixture benchmark change. Added focused coverage that an empty fastest block emits a valid raw block without panicking.
- Inlined the common literal-length and match-length code helpers after refreshed JSON profiles showed both as visible sequence-side symbols. Existing exhaustive helper-level spec tests cover these tables and the first uncached boundary, and a follow-up profile confirmed the helpers are folded into `encode_sequences`.
- Added a specialized matcher path for whole-block RLE history. Instead of indexing every suffix in a constant block, the default matcher stores only the first and last useful suffix for the single repeated key, preserving future matches while avoiding duplicate suffix insertion work. Custom matchers keep the existing default behavior through a new provided trait method.
- Hardened matcher suffix-store sizing for short-frame reuse: fresh driver stores are sized from the configured slice size, and the private store constructor has a minimum backing size. Added focused tests for zero-capacity store construction and reusing a one-byte frame's store for a later larger frame, plus an end-to-end reused-compressor Fastest test that compresses a one-byte frame followed by a larger compressible frame and round-trips both through the Rust and C zstd decoders.
- Added C-style repeat-offset availability pruning before repeat probes enter relative-window lookup. Repeat offsets that point before the retained window are now skipped explicitly, matching C fast's invalid-repcode guard while keeping the safe relative lookup for valid boundary-crossing matches.
- Reduced the default suffix hash table to the C zstd level-1 fast-parser scale: a 128 KiB block now uses 8 Ki hash slots. This gives back some retained compression headroom but keeps every PR fixture smaller than C zstd while materially reducing CPU and RSS. Added focused coverage for the driver sizing invariant and for keeping `Option<Candidates>` compact with the two-`NonZeroU32` representation.
- Changed hash-candidate window search to scan newest entries first and to stop once a candidate reaches the block end. This mirrors C fast's most-recent hash-table behavior and avoids continuing after the maximum possible match length is found. Added focused helper coverage for non-offset-1 block-end early exit and full matcher coverage that the newest previous block-end candidate wins.
- Switched Huffman-compressed literal payloads below 256 bytes to single-stream encoding, matching C zstd's `singleStream = srcSize < 256` selection while keeping 4-stream encoding for larger payloads. Added an emitted-bitstream test that verifies the single-stream literal header and round-trips the frame through both the Rust and C zstd decoders.
- Added C-style minimum-gain rejection for Huffman literal sections. Fast-level literals now require a compressed payload gain of `(srcSize >> 6) + 2` before emitting Huffman, preserving all PR fixtures smaller than C zstd while avoiding narrow literal wins that cost CPU. Added focused coverage for the exact boundary plus Rust/C decoder round-trip coverage for the emitted raw fallback.
- Disabled CLI progress-bar updates for non-terminal stderr. This keeps interactive progress behavior but removes `indicatif` update/tick overhead from redirected benchmark runs, making the CLI comparison with quiet C zstd cleaner. Added focused CLI coverage that the hidden progress monitor still reads through all input bytes.
- Added C-style single-stream literal encoding when reusing a previous Huffman table for sub-1 KiB literal payloads, but only when the exact estimated size is no larger than a newly generated table. This preserves decodecorpus size, improves JSON by 7 bytes on the PR fixture table, and avoids blindly preferring a worse previous table. Added Huffman-table symbol coverage plus a two-block emitted-bitstream test that round-trips the treeless single-stream repeat-table path through both the Rust and C zstd decoders.
- Removed the redundant previous-Huffman-table compatibility scan against the newly generated Huffman table. The literal-count compatibility check already proves the previous table can encode every symbol in the payload, so the repeat-table decision now avoids a second equivalent symbol scan while preserving exact fixture bytes.
- Added an all-RLE sequence encoding fast path. When literal-length, match-length, and offset sequence modes are all already RLE, the encoder now writes only the per-sequence additional bits and final padding instead of passing through the generic optional FSE-state update path. This does not change table selection or emitted bytes. Added decoder round-trip coverage with varying additional bits under identical RLE symbols.
- Added a conservative compressed-block output reservation in the fastest path before writing directly into the frame output buffer. This preserves exact emitted bytes and reduces output-buffer growth churn after the temporary compressed-block buffer was removed.
- Added exact Huffman table-depth selection for small literal sections with at most two sequences. This recovers C-style small-boundary compression wins without applying the expensive search to every level-1 Huffman literal section. Focused tests cover the selected table's real gain and Rust/C decoder round trips.
- Added an internal block-compression config keyed by `CompressionLevel` so higher levels can diverge from level 1 without slowing the fastest path. Public levels `Default`, `Better`, and `Best` now encode instead of panicking; `Best` opts into the previously benchmarked broad exact Huffman table-depth search for every new Huffman literal table. Focused tests cover all implemented levels through Rust and C zstd decoder round trips, and cover `Best` using exact Huffman search beyond level-1's small-literal-section limit.
- Added a `Best`-level larger advertised matcher window, matching C `-4`'s larger-window direction, while keeping the active search history at the level-1 size for binary-like blocks. Text-like blocks can use sixteen 128 KiB blocks of history; binary-like blocks stay at four 128 KiB blocks. Focused matcher tests cover both the level-specific advertised window and the text-only active-window guard.
- Added `Best`-level C-sized suffix hash tables, following C `-4`'s `hashLog 18` direction while preserving level 1's C-fast 8 Ki slot scale. The matcher resizes reused suffix stores when a level switch changes the desired table size. Focused matcher coverage locks down the Best-level store size.
- Added `tools/generate_zstd_fixtures.py` for deterministic expanded local benchmark fixtures and ignored Python bytecode cache files.
- Added C-style FSE normalization for sequence tables, using a safe Rust implementation of the reference table-log and normalized-count selection while retaining the older normalizer for Huffman weight tables. Focused tests cover the table-log policy, normalized-count totals, and large balanced sequence tables.
- Added bounded direct FSE state lookup tables for small encoder tables. This avoids linear state-range searches in `next_state()` for sequence tables after the C-style normalizer raised their table logs, while large generic/fuzz tables keep the old search path to avoid excessive memory use.

## Verification So Far

Latest successful commands:

- `cargo fmt --all --check`
- `cargo test -q -p ruzstd bit_writer`
- `cargo test -q -p ruzstd fse`
- `cargo test -q -p ruzstd encoding::blocks::compressed`
- `cargo test -q -p ruzstd all_rle_sequence_modes_preserve_additional_bits`
- `cargo test -q -p ruzstd encoding::blocks::compressed::tests::literal_min_gain_boundary_uses_raw_literals_and_round_trips`
- `cargo test -q -p ruzstd encoding::blocks::compressed::tests::small_huffman_literals_use_single_stream_and_round_trip`
- `cargo test -q -p ruzstd encoding::blocks::compressed::tests::small_literals_prefer_previous_huffman_table_and_single_stream`
- `cargo test -q -p ruzstd encoding::match_generator`
- `cargo test -q -p ruzstd encoding::frame_compressor::tests::fastest_reused_compressor_handles_tiny_then_compressible_frame`
- `cargo test -q -p ruzstd encoding::levels::fastest_tests`
- `cargo test -q -p ruzstd fastest_reuses_history_across_blocks`
- `cargo test -q -p ruzstd can_encode_counts_checks_symbols_without_building_table`
- `cargo test -q -p ruzstd huff0::huff0_encoder::encoded_len`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q -p ruzstd`
- `cargo test -q --workspace`
- `cargo build --release -p ruzstd-cli`
- `cargo test -q -p ruzstd-cli progress`
- `cargo clippy -q -p ruzstd-cli -- -D warnings`
- `python3 /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/zstd_bench_current_branch.py`
- `python3 tools/benchmark_zstd.py --csv-output /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-no-progress.csv --md-output /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-no-progress.md`
- `python3 tools/benchmark_zstd.py --csv-output /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-repeat-huff-single-when-smaller.csv --md-output /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-repeat-huff-single-when-smaller.md`
- `python3 tools/benchmark_zstd.py --csv-output /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-repeat-huff-count-check.csv --md-output /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-repeat-huff-count-check.md`
- `python3 tools/benchmark_zstd.py --csv-output /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-rle-sequence-fast-path.csv --md-output /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-rle-sequence-fast-path.md`
- `python3 tools/benchmark_zstd.py --csv-output /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-output-reserve.csv --md-output /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-output-reserve.md`
- `perf record -F 999 -g -o /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-decodecorpus-after-usize-rep.perf.data -- /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-huffman-maxheight compress /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-decodecorpus-profile.zst -l 1`
- `perf report --stdio -i /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-decodecorpus-after-usize-rep.perf.data --sort=symbol --no-children`
- `perf record -F 999 -g -o /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-json-touched-u32-clear.perf.data -- /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-huffman-maxheight compress /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/json_logs_32m.jsonl /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-json-profile.zst -l 1`
- `perf report --stdio -i /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-json-touched-u32-clear.perf.data --sort=symbol --no-children`
- `perf record -F 999 -g -o /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-json-direct-repeat-update.perf.data -- /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-huffman-maxheight compress /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/json_logs_32m.jsonl /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-json-profile.zst -l 1`
- `perf report --stdio -i /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-json-direct-repeat-update.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-decodecorpus-heap-huff.perf.data -- /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-huffman-maxheight compress /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-decodecorpus-profile.zst -l 1`
- `perf report --stdio -i /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-decodecorpus-heap-huff.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-decodecorpus-after-bitwriter.perf.data -- /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-huffman-maxheight compress /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-decodecorpus-profile.zst -l 1`
- `perf report --stdio -i /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-decodecorpus-after-bitwriter.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-json-after-bitwriter.perf.data -- /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-huffman-maxheight compress /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/json_logs_32m.jsonl /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-json-profile.zst -l 1`
- `perf report --stdio -i /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-json-after-bitwriter.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-decodecorpus-after-keyvalue.perf.data -- /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-huffman-maxheight compress /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-decodecorpus-profile.zst -l 1`
- `perf report --stdio -i /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-decodecorpus-after-keyvalue.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-json-after-keyvalue.perf.data -- /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-huffman-maxheight compress /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/json_logs_32m.jsonl /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-json-profile.zst -l 1`
- `perf report --stdio -i /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-json-after-keyvalue.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-decodecorpus-current.perf.data -- /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-huffman-maxheight compress /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-decodecorpus-profile.zst -l 1`
- `perf report --stdio -i /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-decodecorpus-current.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-json-current.perf.data -- /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-huffman-maxheight compress /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/json_logs_32m.jsonl /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-json-profile.zst -l 1`
- `perf report --stdio -i /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-json-current.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-decodecorpus-f9735b7.perf.data -- /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-huffman-maxheight compress /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-decodecorpus-profile.zst -l 1`
- `perf report --stdio -i /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-decodecorpus-f9735b7.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-json-f9735b7.perf.data -- /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-huffman-maxheight compress /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/json_logs_32m.jsonl /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-json-profile.zst -l 1`
- `perf report --stdio -i /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-json-f9735b7.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-decodecorpus-after-c-hash.perf.data -- /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-huffman-maxheight compress /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-decodecorpus-profile.zst -l 1`
- `perf report --stdio -i /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-decodecorpus-after-c-hash.perf.data --sort=symbol --no-children`
- `perf record -m 64 -F 999 -g -o /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-json-after-c-hash.perf.data -- /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-huffman-maxheight compress /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/json_logs_32m.jsonl /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-json-profile.zst -l 1`
- `perf report --stdio -i /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-json-after-c-hash.perf.data --sort=symbol --no-children`

## Latest Benchmark Snapshot

Script: `tools/benchmark_zstd.py`

This script benchmarks fixtures from `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures` one output at a time because `/tmp` is nearly full. The verifier decodes each compressed output with C zstd and compares the decoded bytes against the original fixture bytes; benchmark rows therefore prove both decode success and byte-for-byte identity. The latest saved byte-verified outputs are `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-output-reserve.csv` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-output-reserve.md`.

Last run after the larger window, match-length fix, RLE sequence modes, incompressibility gate, raw-block no-index fast path, compact raw literals headers, overlapping match extension, chunked slice comparison, matcher-side repeat-offset probing, hash-match backward extension, exact Huffman table reuse estimates, text-aware non-repeat match threshold, small-block predefined FSE tables, repeat-offset-biased match selection, the 10-byte repeat-offset search early exit, sparse suffix indexing for matches longer than 128 bytes, repeat-offset and hash-candidate minimum-match prechecks, verified-prefix match-length scans, hot helper inlining, the repeat-aware no-match probe step, fixed repeat-candidate loops, candidate-helper inlining, deterministic unstable entropy sorts, text-only wider no-match probing, `usize` repeat-candidate selection, touched-slot suffix-store clearing, direct matcher repeat-history updates, previous-entry-only newest-first cross-window lookup, cached encoder FSE `acc_log`, C-style end-2 sparse match indexing, heap-based Huffman tree construction, cached sequence FSE table references, cached common sequence length-code tables, suffix-hash modulo removal, same-block forward match-length fast path, modest touched-slot preallocation, explicit suffix-candidate checks, direct repeat-offset encoding branches, inlined offset boundary conversions, matcher block-length hoisting, C-style small literal-compression threshold, exact-block EOF lookahead, BitWriter exact-fill fast path, precomputed suffix key values, countdown sequence encoding, inlined literal/match length-code helpers, sparse RLE history indexing, hardened suffix-store sizing, repeat-offset availability pruning, C-sized suffix hash tables, newest-first block-end hash search, C-style single-stream Huffman literals below 256 bytes, C-style minimum-gain rejection for Huffman literal sections, C-style single-stream repeat-Huffman literals below 1 KiB when estimated no larger than a new table, the all-RLE sequence encoding fast path, and conservative compressed-block output reservation:

| Fixture | Upstream bytes | Current bytes | C zstd -1 bytes | Upstream CPU | Current CPU | C zstd -1 CPU |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | 5,976,095 | 5,371,424 | 5,385,951 | 0.13s | 0.17s | 0.04s |
| `json_logs_32m.jsonl` | 3,392,237 | 742,720 | 1,138,701 | 0.18s | 0.11s | 0.04s |
| `repeated_text_32m.txt` | 31,757 | 2,874 | 3,116 | 0.12s | 0.00s | 0.02s |
| `xorshift_32m.bin` | 33,555,213 | 33,555,210 | 33,555,214 | 0.59s | 0.02s | 0.06s |

Peak RSS from the same run:

| Fixture | Upstream RSS | Current RSS | C zstd -1 RSS |
| --- | ---: | ---: | ---: |
| `decodecorpus_pack.bin` | 6,532 KB | 5,352 KB | 22,048 KB |
| `json_logs_32m.jsonl` | 5,764 KB | 4,656 KB | 19,080 KB |
| `repeated_text_32m.txt` | 5,596 KB | 4,240 KB | 17,768 KB |
| `xorshift_32m.bin` | 6,236 KB | 4,504 KB | 25,612 KB |

Interpretation:

- Size improved materially on `decodecorpus_pack.bin`, `json_logs_32m.jsonl`, and `repeated_text_32m.txt`; the current branch remains smaller than C zstd on all three compressible fixtures and four bytes smaller on xorshift.
- Conservative compressed-block output reservation preserved exact fixture byte counts. The refreshed table measured decodecorpus at 0.16s, JSON at 0.11s, repeated text at 0.00s, and xorshift at 0.02s, with current RSS still well below C zstd on every fixture.
- Narrow exact Huffman table-depth selection for small literal sections preserved exact PR fixture byte counts and kept CPU in the retained band on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-huff-smallest-small-seq.md`: decodecorpus 0.16s, JSON 0.11s, repeated text 0.00s, and xorshift 0.02s. The expanded-suite report `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-expanded-huff-smallest-small-seq.md` improved the repeated boundary cases from 130/136/137/137 bytes to 122/128/129/129 bytes, making all four smaller than C zstd's 125/132/133/135 byte outputs. `repeated_text_001m.txt` improved from 216 bytes to 208 bytes. `json_logs_001m.jsonl` remains larger than C zstd at 61,359 bytes versus 59,118 bytes.
- Broad exact Huffman table-depth selection for every new Huffman literal table improved PR fixture sizes by 67 bytes on decodecorpus and 4 bytes on JSON, but repeatedly moved decodecorpus CPU to 0.18-0.19s. Do not use it for level 1; keep it recorded as a higher-level compression candidate.
- C-style sequence FSE normalization closed the remaining expanded 1 MiB JSON gap: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-expanded-c-normalize-seq.md` measured `json_logs_001m.jsonl` at 58,767 bytes versus C zstd's 59,118 and the previous Rust 61,359. The PR table `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-c-normalize-seq-repeat.md` improved `decodecorpus_pack.bin` from 5,371,424 to 5,324,267 bytes and `json_logs_32m.jsonl` from 742,720 to 690,084 bytes. CPU cost increased versus the previous retained level-1 snapshot, with repeat medians at decodecorpus 0.18s and JSON 0.15s; keep this as a C-guided compression-quality win, then profile sequence table construction/encoding before further CPU work.
- Bounded direct FSE state lookup preserved exact compressed bytes after C-style sequence normalization and recovered most of the JSON CPU cost. Two PR table runs saved to `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-fse-direct-lookup.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-fse-direct-lookup-repeat.md` measured `json_logs_32m.jsonl` at 0.12s instead of 0.15s, with `decodecorpus_pack.bin` at 0.17s. The expanded report `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-expanded-fse-direct-lookup.md` preserved the 58,767 byte 1 MiB JSON result. The follow-up profile `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-json-fse-direct-lookup.perf.data` moved `encode_sequences` down from about 26.8% to about 4.5%; matcher search is again dominant at about 68%.
- Level-specific compression config preserved the retained level-1 size table on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-level-config-l1.md`: decodecorpus `5,324,267`, JSON `690,084`, repeated text `2,874`, and xorshift `33,555,210`. The same byte-verified run measured level-1 CPU at decodecorpus `0.17s`, JSON `0.12s`, repeated text `0.01s`, and xorshift `0.01s`.
- `CompressionLevel::Best` now applies broad exact Huffman table-depth selection. The focused level-4 benchmark `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-level-config-l4-current-vs-c.md` used the current binary as the upstream placeholder because older upstream cannot encode level 4. It measured `Best` at decodecorpus `5,324,200` and JSON `690,080`, recovering the known broad-Huffman byte wins versus level 1 while costing extra decodecorpus CPU (`0.18s` versus level-1 `0.17s`). C zstd `-4` measured `4,789,813` on decodecorpus, `1,361,274` on JSON, `3,128` on repeated text, and `33,555,214` on xorshift, so the next higher-level work should target parser/search improvements for decodecorpus rather than more literal-only tuning.
- `CompressionLevel::Best` text-only larger active history improved JSON further without worsening decodecorpus versus the previous Best snapshot. `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-text-window-l4.md` measured decodecorpus `5,324,200`, JSON `600,017`, repeated text `2,874`, and xorshift `33,555,210`. This is a higher-level compression win, not a level-1 change: JSON improves by about 90 KiB versus the first Best implementation and about 90 KiB versus level 1, while level-1 bytes remain unchanged on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-text-window-l1.md`.
- `CompressionLevel::Best` C-sized suffix hash tables produced a substantial higher-level compression win on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-hashlog18-retained-l4.md`: decodecorpus `5,107,302`, JSON `561,407`, repeated text `2,874`, and xorshift `33,555,210`. This closes much of the C `-4` decodecorpus gap while keeping JSON far smaller than C `-4`'s `1,361,274`. CPU cost is higher than level 1, as expected for a higher level: decodecorpus `0.22s` and JSON `0.23s` on the retained table run. Level 1 remained byte-stable on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-level-progression-l1.md`.
- Tested mapping `Default` and `Better` to intermediate C-style window/hash scales (`windowLog`/`hashLog` progression for C levels 2 and 3). The benchmark exposed invalid decodecorpus output for levels 2 and 3, so that intermediate matcher scaling was reverted. `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-level2-reverted-check.md` confirms level 2 is valid again and currently uses the level-1 matcher behavior. Revisit intermediate levels only with a focused decodecorpus regression fixture that catches this failure.
- The all-RLE sequence encoding fast path preserved exact fixture byte counts. The refreshed table measured JSON at 0.10s, decodecorpus at 0.17s, repeated text at 0.00s, and xorshift at 0.02s. Keep it as a covered sequence-side simplification for blocks whose sequence table modes are already RLE, not as a table-selection heuristic.
- C-style single-stream repeat-Huffman literals below 1 KiB, gated by exact estimated size versus a newly generated table, preserved decodecorpus/repeated/xorshift byte counts and improved JSON by 7 bytes. The same table run measured JSON CPU in the retained 0.11s band and decodecorpus at 0.17s; treat the decodecorpus CPU as noise risk rather than a proven regression because byte counts are unchanged and the change is literal-path only. A broader blind-prefer-previous-table variant was rejected after it regressed JSON by 7,340 bytes and decodecorpus by 522 bytes despite improving JSON CPU to 0.10s.
- Removing the redundant previous-Huffman compatibility scan preserved exact fixture byte counts. The refreshed table measured CPU in the retained band: decodecorpus 0.17s, JSON 0.11s, repeated 0.00s, xorshift 0.01s.
- Disabling CLI progress updates for non-terminal benchmark runs preserved exact fixture byte counts and kept CPU medians in the existing band. A follow-up JSON perf sample no longer showed `indicatif` progress symbols near the top; matcher search remained dominant at about 69% and sequence encoding remained secondary at about 9%.
- C-style minimum-gain rejection for Huffman literal sections regressed `decodecorpus_pack.bin` by 2,901 bytes versus the previous retained snapshot, but still keeps it 14,527 bytes smaller than C zstd. Two runs measured decodecorpus CPU at 0.16s instead of the previous 0.17s band, with the other fixture byte counts unchanged. Keep it because it matches C zstd's fast literal acceptance guardrail and has focused emitted-bitstream Rust/C coverage.
- Single-stream Huffman literals below 256 bytes improved `decodecorpus_pack.bin` by 23 bytes, preserved the other PR fixture byte counts, and kept CPU in the existing noise band. Keep it because it matches C zstd's literal-stream selection and has full Rust/C emitted-bitstream coverage.
- C-sized suffix hash tables trade retained compression headroom for a large CPU/RSS win. Divisors 2, 4, 8, 16, and 32 were tested. Divisor 16 keeps `decodecorpus_pack.bin` 17,428 bytes smaller than C zstd and keeps JSON 395,974 bytes smaller than C while reducing current RSS below upstream on the table run. Divisor 32 improved CPU further but regressed decodecorpus to 5,488,132 bytes, larger than C zstd, so it was rejected.
- Newest-first block-end hash search preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.17s both times, JSON at 0.10s then 0.11s, repeated text at 0.00s, and xorshift at 0.02s. Keep it as a C-shaped control-flow cleanup with focused block-end early-exit coverage.
- Repeat-offset availability pruning preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.21s then 0.20s, JSON at 0.12s both times, repeated text at 0.00s/0.01s, and xorshift at 0.02s both times. Keep it as covered C-style stale-repeat pruning rather than a fixture-specific speed win.
- Hardened suffix-store sizing preserved exact fixture byte counts. The table run measured decodecorpus at 0.21s, JSON at 0.12s, repeated text at 0.00s, and xorshift at 0.02s. Keep it as a covered matcher-pool correctness fix rather than a fixture-specific speed win.
- Sparse RLE history indexing preserved exact PR fixture byte counts. Two table runs measured decodecorpus at 0.21s then 0.20s, JSON at 0.11s then 0.12s, repeated text at 0.01s/0.00s, and xorshift at 0.02s both times. The PR fixtures do not heavily exercise whole-block RLE history, so keep this as covered C-shaped RLE-path CPU cleanup rather than a fixture-specific speed win.
- Inlining the literal-length and match-length code helpers preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.21s then 0.20s, JSON at 0.11s both times, repeated text at 0.01s/0.00s, and xorshift at 0.02s both times. The follow-up JSON profile no longer showed `encode_literal_length` or `encode_match_len` as separate top-level symbols, so keep this narrow sequence-side cleanup.
- The empty fastest-block guard preserved exact fixture byte counts. The refreshed table run measured decodecorpus at 0.21s and JSON at 0.12s, within the existing noise band for this branch. Keep it as a covered direct-block correctness guard; normal frame compression already handles empty frames before calling `compress_fastest()`.
- Countdown sequence encoding preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.20s both times and JSON at 0.11s both times; keep it as a small sequence-side cleanup that matches C zstd's indexed reverse-loop shape.
- Precomputed suffix key values preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.20s then 0.21s and JSON at 0.11s both times; keep it as a small safe matcher cleanup that avoids re-reading the current five-byte suffix for every window entry during hash lookup.
- Refreshed profiles after suffix-key reuse still show matcher search as the dominant cost: about 73% of decodecorpus samples and 66% of JSON samples. JSON also showed sequence encoding around 11%, so sequence-side scalar cleanup remains a secondary target when matcher experiments stop moving.
- Exact-block EOF lookahead removed the extra empty final raw block for exact block-multiple inputs. This improved `repeated_text_32m.txt` and `xorshift_32m.bin` by 3 bytes each, with decodecorpus and JSON byte-identical and CPU in the existing noise band across two runs.
- BitWriter exact-fill flushing preserved exact fixture byte counts. Two table runs kept decodecorpus at 0.21s, JSON at 0.11s, and repeated/xorshift in their existing bands; retain it because it removes a cold helper call from a common bitstream boundary and has focused bit-level coverage.
- Refreshed profiles after the BitWriter exact-fill change still show matcher search as the dominant CPU cost: about 70% of decodecorpus samples and 76% of JSON samples. The former `write_bits_64_cold` hotspot dropped to about 0.5% of decodecorpus samples, so further CPU work should stay focused on matcher search/counting unless future profiles shift.
- Lowering the literal-compression threshold to C zstd's 63-byte heuristic improved JSON size by 80,942 bytes and decodecorpus by 1,164 bytes versus the previous retained snapshot. Two runs kept JSON CPU at 0.11s, repeated/xorshift unchanged, and decodecorpus at 0.22s then 0.20s; keep it as a clear compression win with covered bitstream behavior.
- Lowering the repeat-Huffman literal threshold to C zstd's 6-byte heuristic preserved exact PR fixture bytes. Two table runs measured decodecorpus at 0.20s both times, JSON at 0.12s then 0.11s, and repeated/xorshift unchanged; keep it because it is covered C-guided compression-quality work for small literal payloads after Huffman history exists.
- Early raw-literals fallback from the exact Huffman estimate preserved exact PR fixture bytes. Three table runs measured decodecorpus at 0.20s, 0.21s, then 0.20s and JSON at 0.11s, 0.12s, then 0.11s; keep it because it avoids known no-gain Huffman encode work and has focused emitted-bitstream coverage for the estimate-raw path.
- The repeat-offset search early exit, sparse long-match indexing, no-match probe step, and text-only wider probing trade about 50 KiB of decodecorpus compression and 2 bytes of repeated-text compression for a large CPU improvement, while JSON is now materially smaller than before the CPU parser shortcuts. The repeat/hash prechecks, hot helper inlining, fixed repeat-candidate loops, candidate-helper inlining, suffix-hash modulo removal, same-block forward match-length fast path, modest touched-slot preallocation, explicit suffix-candidate checks, direct repeat-offset encoding branches, inlined offset boundary conversions, and matcher block-length hoisting keep output sizes unchanged and improve or simplify CPU hot paths further. The measured current aggregate CPU is now about 0.34s versus about 0.90s before these CPU-focused parser shortcuts.
- Text-only wider no-match probing improved JSON size from 849,901 bytes to 826,471 bytes with only a 370-byte decodecorpus size cost, avoiding the much larger global step-3 decodecorpus regression.
- Candidate-helper inlining improved decodecorpus CPU from about 0.26s to 0.25s with no size change on the fixture set.
- Deterministic unstable entropy sorts preserved output sizes and benchmarked neutral-to-slightly-better; keep them because they remove unnecessary stable-sort work in a profiled setup path.
- Fixed repeat-candidate loops improved decodecorpus CPU from about 0.27s to 0.26s and JSON CPU from about 0.19s to 0.18s with no size change on the fixture set.
- The repeat-aware no-match probe step improved decodecorpus CPU from about 0.31s to 0.27s, improved JSON size from 950,143 bytes to 849,901 bytes, and kept all fixture sizes smaller than C zstd.
- The hash-candidate precheck improved decodecorpus CPU from about 0.33s to 0.31s with no size change on the fixture set.
- Hot helper inlining improved decodecorpus CPU from about 0.36s to 0.33s and JSON CPU from about 0.19s to 0.18s with no size change on the fixture set.
- The repeat-offset precheck improved decodecorpus CPU from about 0.42s to 0.36s and JSON CPU from about 0.21s to 0.19s with no size change on the fixture set.
- Sparse long-match indexing is the largest repeated-text CPU improvement so far, dropping that fixture from about 0.20s to about 0.02s while keeping it smaller than C zstd.
- Backward extension is a net size win across the fixture set, but it slightly worsened JSON size versus repeat-offset probing alone; keep that tradeoff visible when evaluating future match selection changes.
- The incompressible fixture is now near C zstd CPU after the no-index raw fast path.
- The larger window, overlapping extension, and repeat-offset probing improve compression but raise CPU and RSS on compressible fixtures; next work should continue reducing matcher search cost without giving back the remaining compression advantage over C zstd.
- Perf sample on `repeated_text_32m.txt` showed time dominated by `MatchGeneratorDriver::start_matching`; the repeat-offset callback is inlined into that symbol, but the larger future CPU opportunity is still matcher logic.
- Profiling after the repeat-offset bias still shows `MatchGeneratorDriver::start_matching` and `match_len_at_offset` as the dominant CPU cost. A safe hand-written `u64` prefix mismatch loop was tested, but it was slower than the existing chunk-iterator comparison and was not kept.
- Tested C zstd level-1's `minMatch = 7` setting. It improved JSON slightly but regressed decodecorpus much more, so the current global `MIN_MATCH_LEN = 5` remains the better fixture-wide choice.
- Tested rejecting short non-repeat matches below length 7 with offset cutoffs 64, 1024, 4096, and 16384. This improved JSON size by up to about 15 KiB and CPU modestly, but regressed decodecorpus by more than the JSON gain at every cutoff, so it was not kept.
- Tested repeat-offset search early-exit thresholds 5, 10, 16, and 64. Threshold 64 had no useful CPU benefit, thresholds 5/10/16 all reduced JSON CPU sharply, and threshold 10 gave the best measured aggregate CPU/size balance.
- Tested sparse long-match indexing dense limits 64, 128, and 256. Limit 64 hurt JSON size more than needed, 256 lost JSON CPU compared with 128, and 128 was the best measured aggregate balance.
- Tested widening the repeat-aware no-match probe step from 2 to 3. It improved JSON size but worsened decodecorpus size by about 52 KiB and did not improve CPU, so step 2 remains the better aggregate point.
- Tested changing the remaining Huffman stable sorts to unstable sorts with explicit tie-breakers. Output sizes were unchanged, but decodecorpus CPU repeatedly regressed from the 0.26-0.27s band to about 0.30s, so the runtime change was not kept. The added tie-behavior tests were kept.
- Removing production matcher `unwrap`/`expect` calls preserved output sizes and benchmarked neutral: decodecorpus stayed at 0.26s, JSON stayed at 0.18s, and repeated text stayed at about 0.01-0.02s.
- Tested replacing suffix `windows(MIN_MATCH_LEN)` insertion with direct index-based key extraction. Output sizes were unchanged, but JSON CPU repeatedly regressed from the 0.18-0.19s band to about 0.20-0.21s, so the change was not kept.
- Tested replacing the hot `bounded_u32` `TryFrom` path with an explicit checked branch plus cold panic path. Output sizes were unchanged, but decodecorpus and JSON CPU drifted worse across two runs, so the change was not kept.
- Tested C-fast-style immediate zero-literal repeat early exits. Threshold 5 slightly improved decodecorpus size but regressed JSON size, threshold 6 regressed JSON and did not beat the baseline on decodecorpus, and threshold 7 was worse than both endpoints. The family was not kept because it trades away text compression without a reliable CPU win.
- Entropy-path bookkeeping cleanup preserved exact fixture byte counts. Benchmarks were neutral within noise: decodecorpus measured 0.26s then 0.28s, JSON stayed at 0.19s, and repeated text stayed at 0.01s.
- Tested wider safe match-extension chunks as a SIMD-adjacent experiment. Chunk widths 16 and 32 preserved exact output bytes, but neither improved decodecorpus CPU over the existing 8-byte chunk shape, so the existing safe chunk width remains in place.
- Tested combining the repeated FSE table-selection scans into one pass. Output bytes were unchanged, but JSON CPU repeatedly regressed to about 0.20s, so the original separate scans were kept.
- Caching the Huffman max code length preserved exact fixture byte counts and benchmarked neutral-to-slightly-positive within noise: decodecorpus measured 0.26s then 0.27s in clean runs, JSON measured 0.19s then 0.18s.
- Direct compressed-block output preserved exact fixture byte counts and removed one allocation/copy from accepted compressed blocks. Clean benchmark passes measured decodecorpus at 0.26s and JSON at 0.18s.
- Conservative literals/sequences preallocation preserved exact fixture byte counts and improved decodecorpus CPU to 0.25s across two clean runs while keeping JSON at 0.18s. RSS stayed within the existing measurement band.
- Tested moving per-block literal and sequence scratch buffers into `CompressState` for reuse across compressed blocks. Output bytes stayed unchanged and RSS was slightly lower, but decodecorpus CPU repeatedly measured about 0.27s instead of the 0.25s seen with simple per-block preallocation, so the added state complexity was not kept.
- Keeping repeat-offset probe candidate selection in `usize` preserved exact fixture byte counts, reduced decodecorpus CPU to 0.25s on the table run, and moved `bounded_u32` from a visible multi-percent matcher cost to a small residual emission-side cost in the follow-up profile.
- Touched-slot suffix-store clearing preserved exact fixture byte counts. It reduced JSON CPU from the 0.18-0.19s band to 0.17s, reduced xorshift CPU to 0.02s, and moved JSON `commit_space` from about 7% to about 1.1% in perf. A pure touched-slot vector raised decodecorpus RSS to about 11.4 MB, so the retained implementation switches to full sequential clearing after 32K touched slots; that kept the JSON CPU win while lowering decodecorpus RSS to about 10.7 MB. A 64K threshold was tested and rejected after it regressed decodecorpus and JSON CPU on the table run.
- Direct matcher repeat-history updates preserved exact fixture byte counts, kept JSON at 0.17s, and moved decodecorpus back to 0.25s on the table run. The follow-up JSON profile removed the previously visible matcher-side `OffsetHistory::encode_offset_value` cost, leaving only small residual repeat-history samples around sequence emission.
- Newest-first cross-window lookup preserved exact fixture byte counts. The initial newest-first change was neutral-to-positive; additionally skipping the impossible current-block check for negative relative lookups improved decodecorpus to 0.24s across two runs, while JSON drifted to 0.18s. The aggregate CPU stayed slightly better, so the C-shaped lookup order was kept with focused coverage for most-recent previous-window matches.
- Tested replacing the no-match skip guard's small range iterator with explicit branches for probe steps 2 and 3. Output bytes were unchanged, but decodecorpus drifted to 0.26s and JSON to 0.18s on the table run, so the original iterator-shaped guard was kept.
- Tested replacing FSE `SymbolStates::get()`'s iterator search with an explicit indexed loop and cold panic path. Output bytes were unchanged, but decodecorpus drifted to 0.26s and JSON to 0.18s on the table run, so the original iterator/`unwrap` form was kept for this hot lookup.
- Tested splitting the common literal-length and match-length code ranges into explicit early-return branches before the larger range matches. Output bytes were unchanged, but decodecorpus drifted to 0.26s and JSON to 0.18s on the table run, so the original single-match form was kept.
- Tested scanning hash-candidate window entries newest-first. Output bytes were unchanged and one run improved JSON to 0.16s, but the repeat run returned JSON to 0.18s while decodecorpus stayed at 0.25s, so the change was treated as noise and not kept.
- Caching the encoder FSE `acc_log` preserved exact fixture byte counts. The table run measured decodecorpus unchanged at 0.24s and JSON at 0.16s; treat the JSON improvement cautiously as run-to-run noise, but keep the change because it removes repeated `ilog2` work and has focused invariant coverage.
- Moving the sparse long-match tail hash from `match_end - MIN_MATCH_LEN` to `match_end - 2`, matching the C fast parser, improved decodecorpus by 65 bytes with no size change on the other fixtures. Two benchmark runs kept decodecorpus at 0.24s and JSON in the 0.16-0.17s noise band, so the C-shaped indexing position was kept.
- Carrying a verified minimum-match prefix into full match-length scans preserved exact fixture byte counts. The benchmark stayed neutral in the current noise band, but the change avoids rechecking the first five bytes for accepted candidates while preserving the full scan for previous-window boundary cases.
- Tested probing only the first two repeat-offset candidates in matcher search, closer to C fast's active repeat-offset checks. It regressed decodecorpus from 5,160,978 bytes to 5,166,985 bytes and JSON from 826,471 bytes to 854,443 bytes with no measurable CPU win, so the three-candidate matcher probe was kept.
- Replacing repeated stable sorts in Huffman length-limited tree construction with a deterministic min-heap preserved exact fixture byte counts. The full table runs were noisy, but a direct five-run comparison against the previous commit showed decodecorpus median CPU improving from about 0.24-0.25s to 0.23-0.24s and JSON staying neutral. The follow-up perf sample removed the previously visible `core::slice::sort::stable::drift::sort` symbol, leaving Huffman table construction around 1.2% of decodecorpus samples.
- Tested a same-block fast path for backward match extension using contiguous prefix slices instead of the existing byte walk through `slice_at_relative()`. Output bytes were unchanged, but the table run stayed neutral-to-worse and the follow-up perf sample still showed `extend_match_backwards` around 2.5-3%, so the simpler byte walk was kept.
- Caching sequence FSE table references preserved exact fixture byte counts and reduced repeated enum matching in `encode_sequences`. Two table runs measured decodecorpus at 0.23s, with JSON neutral at 0.17s; keep this as a small sequence-encoder CPU improvement.
- Tested replacing the safe chunk-iterator common-prefix comparison with a direct indexed chunk loop. Output bytes were unchanged, but decodecorpus regressed from the 0.23s band to 0.24-0.26s across two runs, so the iterator-shaped chunk comparison remains the better safe-Rust implementation.
- Tested guarding sequence additional-bit writes so zero-width writes skip `BitWriter::write_bits()`. Output bytes were unchanged, but table runs were mixed: decodecorpus measured 0.23s then 0.25s while JSON measured 0.18s then 0.16s. The unstable tradeoff was not enough to justify the extra branch, so the original direct writes were kept.
- Caching common literal-length and match-length sequence code ranges preserved exact fixture byte counts. Three table runs kept decodecorpus at 0.23s, with JSON in the 0.16-0.18s noise band; retain it as a small safe sequence-code cleanup distinct from the rejected explicit branch-split experiment.
- Removing the redundant suffix-hash modulo preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.22s then 0.20s and JSON at 0.12s both times; retain it because it removes hot-path integer division-style work and matches C zstd's bounded hash-table indexing shape.
- Same-block forward match-length fast path preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.21s then 0.20s and JSON at 0.11s then 0.12s; retain it because it removes repeated relative-window resolution for the common current-block case while keeping the existing safe chunked comparison.
- Modest touched-slot preallocation preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.21s then 0.20s, JSON at 0.11s both times, and RSS stayed in the existing band; retain it because it removes first-use growth in the touched-slot clear path with a small bounded allocation.
- Tested caching each sequence's derived literal-length, match-length, and offset FSE symbols/add-bits in a local `EncodedSequence` representation. Output bytes were unchanged and focused tests passed, but decodecorpus CPU regressed to 0.22s across two table runs and RSS rose to about 12.1 MB, so the runtime change was not kept.
- Tested a previous-window first-segment fast path for match-length scans, mirroring C zstd's two-segment count shape. Output bytes were unchanged and focused tests passed, but decodecorpus regressed to 0.21s then 0.22s while JSON stayed neutral, so the simpler generic relative-window loop remains better.
- Tested increasing the per-block sequence vector initial capacity from 256 to 512 after profiles showed a small `Vec::grow_one` sample on JSON. Output bytes were unchanged, but decodecorpus measured 0.23s then 0.20s and JSON worsened to 0.12s then 0.13s, so the original conservative capacity remains better.
- Tested replacing suffix-store `TryFrom`/checked-add index packing with an explicit upper-bound branch and cold panic path. Output bytes were unchanged and focused tests passed, but decodecorpus stayed at 0.22s across two runs with no clear aggregate CPU win, so the original checked conversion remains.
- Tested reusing the already-computed current suffix hash when inserting the current suffix after a no-match probe miss. Output bytes were unchanged and focused tests passed, but JSON regressed to 0.13s across two runs while decodecorpus stayed in its normal 0.20s-0.21s band, so the simpler insertion path remains better.
- Tested replacing FSE probability-normalization `unwrap()` calls with explicit helper loops and cold invariant panics. After matching iterator tie behavior, output bytes were unchanged, but decodecorpus measured 0.22s then 0.21s and JSON stayed at 0.12s, so the original iterator form remains better for this setup path.
- Tested C-fast-style sparse miss indexing that only stores the current suffix when the no-match probe skips ahead. CPU improved slightly on decodecorpus, but size regressed badly: decodecorpus grew to 5,387,388 bytes, JSON to 862,050 bytes, and repeated text to 2,965 bytes. The retained parser continues indexing skipped miss positions to preserve compression.
- Tested replacing Huffman `build_from_weights()`'s temporary sorted `Vec` with a fixed 256-entry stack array sorted over the filled prefix. Output bytes were unchanged and Huffman tests passed, but decodecorpus measured 0.23s then 0.21s with no clear win, so the existing temporary `Vec` remains better.
- Tested adding same-block direct paths to the repeat-offset minimum-match prechecks, mirroring the retained same-block full match-length fast path. Output bytes were unchanged and focused tests passed, but decodecorpus regressed to 0.22s then 0.21s and JSON regressed to 0.12s across both runs, so the generic relative-window precheck remains better.
- Explicit suffix-candidate checks preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.19s then 0.20s and JSON at 0.11s both times, so retain the helper as a small safe hot-loop cleanup.
- Direct repeat-offset encoding branches preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.20s both times and JSON at 0.11s both times; a follow-up JSON profile no longer showed `OffsetHistory::encode_offset_value` as a separate symbol, so keep the smaller direct branch shape.
- Tested a small cached offset-code table, mirroring the retained literal/match length code caches. Output bytes were unchanged and focused spec-range tests passed, but decodecorpus regressed to 0.21s then 0.22s, so the direct `ilog2` offset-code calculation remains better.
- Inlining boundary offset conversions preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.20s then 0.21s and JSON at 0.11s both times; a follow-up JSON profile no longer showed `offset_to_u32` or matcher `bounded_u32` as separate symbols, so keep this as a small neutral cleanup.
- Tested increasing the per-block literals vector initial capacity from 1024 to 2048 after profiles showed occasional allocation growth in compressed block assembly. Output bytes were unchanged, but decodecorpus regressed to 0.22s then 0.21s and JSON regressed to 0.12s both runs, so the existing conservative literal capacity remains better.
- Hoisting the matcher block length preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.21s then 0.20s and JSON at 0.12s then 0.11s; keep it as a small neutral hot-loop cleanup that reduces repeated length reads without changing search behavior.
- Tested forcing `encode_offset()` inline after JSON profiles showed it as a small sequence-side symbol. Output bytes were unchanged, but decodecorpus measured 0.23s on the first run and only returned to 0.20s on the repeat, so the optimizer's original inlining decision remains better.
- Tested replacing matcher prefix comparison's `chunks_exact(N)` shape with stable fixed-array `as_chunks::<N>()` as a safe SIMD-adjacent experiment. Output bytes were unchanged, but decodecorpus measured 0.22s then 0.20s and JSON measured 0.11s then 0.12s, so the existing `chunks_exact` shape remains better.
- Tested a C-fast-style end-of-block search cleanup that stops probing the last few bytes of large blocks. It improved decodecorpus size by 833 bytes but regressed JSON size by 4,048 bytes and did not improve CPU, so the retained matcher still searches down to the minimum match-length tail.
- Tested direct branches for common offset codes 1, 2, and 3 as a narrower alternative to the rejected offset-code cache. Output bytes were unchanged, but decodecorpus measured 0.23s on the first run and only recovered to 0.20s on the repeat, so the branch-free `ilog2` offset-code path remains better.
- Removing Huffman tree-construction `unwrap()` calls preserved exact fixture byte counts. Two table runs measured decodecorpus at 0.20s both times and JSON at 0.11s both times, so keep the explicit invariant handling as a safe, benchmark-neutral cleanup.
- Tested a conservative C-fast-style no-match acceleration step that increased the probe distance after each 128 bytes since the current anchor while still checking skipped repeat-offset positions and indexing skipped suffixes. Decodecorpus grew by 1,438 bytes with no CPU improvement and JSON drifted to 0.11s CPU, so the fixed step-2/step-3 matcher remains better.
- Tested specializing sequence FSE state updates for the common all-table case to remove per-symbol `Option` checks, mirroring C's resolved-state shape. Output bytes were unchanged, but decodecorpus regressed to 0.21s and JSON did not improve, so the existing compact `Option`-based state update remains better.
- Tested comparing previous/new Huffman literal table reuse lengths in one pass over the literal payload instead of separate `encoded_len()` scans. Output bytes were unchanged, but decodecorpus regressed to 0.22s and JSON to 0.12s, so the existing separate estimator scans remain better.
- Tested forcing the safe chunked prefix helper itself inline after profiles kept the matcher body dominant. Output bytes were unchanged, but the repeat table run drifted decodecorpus to 0.21s and did not improve JSON, so the optimizer's existing inlining choice remains better.
- Tested packing a short suffix fingerprint into the high bits of each stored two-candidate suffix index so hash-slot collisions can be filtered before candidate byte comparison, then narrowed it to binary-looking blocks only. Output bytes stayed unchanged and decodecorpus improved to 0.19s, but JSON regressed to 0.12s across runs, so the untagged two-candidate store remains the best aggregate choice.
- RLE literal-section emission did not change the current PR fixture byte counts, so those fixtures do not exercise all-same large literal payloads. Two table runs stayed in the current noise band with decodecorpus at 0.20s/0.19s and JSON at 0.12s both times; keep the change as covered format/compression-quality work rather than as a fixture-specific benchmark win.
- Tested raising the previous-FSE-table repeat threshold from the current small-block limit of 64 sequences to C fast's broader 1000-sequence heuristic, while keeping the all-symbols-encodable guard. Decodecorpus grew by 107 bytes and CPU regressed to 0.22s with no JSON size benefit, so the local narrower repeat-table policy remains better.
- Tested preallocating the temporary buffer used to estimate Huffman table-description length after the small-literal threshold made literal compression more visible in profiles. Fixture bytes were unchanged, but two table runs were CPU-neutral and JSON drifted from 0.11s to 0.12s on the repeat, so the original minimal `Vec::new()` remains better.
- Tested C zstd's cheap `HUF_optimalTableLog()` depth reduction for Huffman table construction after the small-literal threshold made more blocks eligible for Huffman compression. Focused helper tests passed, but fixture sizes regressed: decodecorpus grew by 311 bytes and JSON by 342 bytes, with decodecorpus CPU also drifting to 0.22s, so the retained encoder keeps the 11-bit Huffman cap.
- Tested replacing exact-EOF one-byte lookahead with a full pending-block lookahead to avoid tiny reads on full-block streams. Output bytes were unchanged, but decodecorpus repeatedly regressed to 0.22s and RSS rose slightly, so the simpler one-byte lookahead remains better.
- Tested replacing hot five-byte minimum-match slice comparisons with an explicit safe byte-by-byte helper, as a safe approximation of C zstd's fixed-width `MEM_read32` prechecks. Output bytes stayed unchanged and focused matcher tests passed, but decodecorpus regressed to 0.24s then 0.23s and JSON to 0.12s across two table runs, so the compiler's original slice-comparison shape remains better.
- Tested reducing the matcher text-classifier sample count from 256 to 128 to lower per-block classifier work while preserving the text/binary parser split. Output bytes stayed unchanged and matcher classifier tests passed, but the measured CPU band was indistinguishable from the 256-sample A/B run, so the original more conservative 256-sample classifier remains.
- Tested precomputing the non-zero-literal repeat-candidate array for the no-match skip guard, since skipped probe positions always have at least one literal before them. Output bytes stayed unchanged and focused matcher tests passed, but JSON stayed at 0.12s and decodecorpus drifted to 0.22s on the repeat run, so the original per-position helper remains better.
- Tested forcing the sequence FSE state helper functions inline after JSON profiles showed sequence encoding as a secondary CPU target. Output bytes stayed unchanged and focused tests passed, but JSON regressed to 0.12s across two table runs, so the optimizer's original helper inlining decisions remain better.
- Tested rewriting the whole-block RLE check to load the first byte once and scan the rest of the block. Output bytes stayed unchanged and focused fastest/compressed-block tests passed, but decodecorpus drifted to 0.21s on the repeat run with no clear fixture-wide CPU win, so the original compact iterator check remains.
- Tested using RLE FSE table modes for two repeated sequence codes instead of only three-or-more repeated codes. Output bytes stayed unchanged on the PR fixtures and CPU had no stable win, so the existing small-block predefined-table preference remains better.
- Tested reducing the fastest incompressibility gate sample count from 256 to 128 after fresh profiles showed the gate as a small JSON cost. Focused gate tests passed and xorshift stayed raw, but decodecorpus grew from 5,159,814 bytes to 5,223,327 bytes, so 128 samples are too weak for mixed binary/text inputs and the 256-sample gate remains.
- Tested a less aggressive 192-sample incompressibility gate as a compromise after the 128-sample version regressed decodecorpus size. Output bytes stayed unchanged, but JSON stayed at 0.12s across both table runs and decodecorpus only improved on the first run before returning to 0.21s, so the 256-sample gate remains the best measured point.
- Tested hoisting the current-block byte slice out of the backward-extension loop. Output bytes stayed unchanged and focused matcher tests passed, but decodecorpus regressed to 0.23s on the first table run and only recovered to the normal band on repeat, so the original loop shape remains.
- Tested deferring the current suffix key-value computation until after repeat-offset probing, so repeat matches that skip hash-table search would avoid one five-byte key load. Output bytes stayed unchanged, but two table runs measured decodecorpus at 0.21s with no JSON CPU improvement, so the existing eager key computation remains.
- Tested C-style precomputed sequence code byte arrays for literal length, match length, and offset code selection/encoding. Focused compressed-block, fastest-frame, and clippy checks passed and output bytes stayed unchanged, but the first table run regressed JSON CPU to 0.12s and the repeat showed no stable improvement while adding three side vectors, so the simpler recompute-on-use path remains.
- Tested carrying the current block slice in `MatchCandidateContext` so hot match helpers could avoid reacquiring the last window entry. Focused matcher and fastest-frame tests plus clippy passed and output bytes stayed unchanged, but two table runs held decodecorpus at 0.22s and JSON drifted to 0.12s on the repeat, so the current narrower context remains better.
- Tested replacing BitWriter cold-path per-byte pushes with a single slice copy for full spilled bytes after a 64-bit flush. Focused BitWriter, compressed-block, fastest-frame, and clippy checks passed and output bytes stayed unchanged, but the repeat table run regressed decodecorpus to 0.22s with no stable JSON win, so the original compact byte loop remains.
- Tested replacing the incompressibility gate's sampled-key linear duplicate search with a fixed 512-slot open-addressed set. Output bytes stayed unchanged and focused fastest tests plus clippy passed, but two table runs showed no stable CPU win: decodecorpus measured 0.20s then 0.21s, JSON stayed at 0.11s, and xorshift stayed at 0.02s. The simpler linear scan remains.
- Refreshed profiles after C-sized suffix hash tables. Matcher search still dominates at about 71% of decodecorpus samples and 74% of JSON samples; sequence encoding is a smaller secondary target around 6%.
- Retested suffix-candidate key tags after the smaller C-sized hash table increased collision pressure. Full tag filtering preserved bytes and improved decodecorpus to 0.16s on one run, but JSON regressed badly to 0.14s. Narrowing tags to binary-looking blocks still regressed JSON to 0.14s with no decodecorpus win over the retained C-sized hash baseline, so the untagged two-candidate store remains better.
- Tested a text-only 4 Ki effective suffix hash table after the global 4 Ki table was rejected. It improved decodecorpus by 5 bytes but regressed JSON from 742,727 bytes to 748,825 bytes with no CPU gain, so text-like blocks keep the retained 8 Ki C-fast hash scale.
- Tested narrowing the no-match skip guard so it only protects the primary repeat-offset candidate from being skipped. Decodecorpus grew by 544 bytes and JSON grew by 8,536 bytes with no JSON CPU improvement, so the guard continues checking all three repeat candidates.
- Tested removing the hot-path temporary repeat-candidate array from matcher probing by expanding the three repeat-offset checks into direct branches. Output bytes stayed unchanged, but decodecorpus CPU repeatedly measured 0.17s instead of the retained 0.16s band, so the compact fixed-array helper remains better.
- Tested raising the text-like non-repeat match threshold from 10 to 12 after the expanded 1 MiB JSON fixture showed higher sequence cost than C zstd. The expanded benchmark `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-expanded-text-threshold-12.md` regressed `json_logs_001m.jsonl` from 61,359 bytes to 61,369 bytes with no other useful change, so the retained threshold remains 10.
- Tested a text-only repeat-offset minimum match length of 7 to mirror C zstd level-1's reported `minMatch=7` more closely. It left `json_logs_001m.jsonl` unchanged at 61,359 bytes on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-expanded-text-repeat-min7.md`, so repeat matches are not the remaining 1 MiB JSON gap.
- Note: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-expanded-text-repeat-min7.md` includes a stray `json_logs_001m.jsonl.zst` row because `zstd --show-default-cparams` wrote into the fixture directory. Use a fresh generated fixture directory for future expanded-suite runs.
- Compared first-block sequence tables on `json_logs_001m.jsonl`: C zstd's literal-length FSE table starts with table log 9, while the current Rust builder selected log 6 after its crude probability normalization. A blanket max-log switch broke the existing FSE round-trip test, so the path forward is better FSE normalization/table-log selection, not forcing max log.
- Tested choosing FSE table log from the original symbol count before the current probability reduction, which is closer to C zstd's `FSE_optimalTableLog(FSELog, nbSeq, max)` input. FSE tests passed, but `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-expanded-fse-total-log.md` regressed `json_logs_001m.jsonl` from 61,359 to 64,447 bytes and added one byte to several small boundary fixtures, so the current normalizer cannot simply use the larger log without a better distribution.
- Applying the new C-style FSE normalizer to Huffman weight table descriptions broke literal round trips, so the retained change intentionally applies it only to sequence FSE tables (`max_log > 6`). Huffman weight FSE still uses the older proven normalizer.

## Next Steps

1. Finish committing and pushing retained progress on the Huffman branch, including the `.gitignore`/benchmark hygiene state where relevant.
2. Use `tools/generate_zstd_fixtures.py` to keep a deterministic expanded local fixture suite under `/tmp`; this avoids committing large binaries while preserving reproducibility.
3. Investigate the expanded-suite gaps versus C zstd, starting with small repeated boundary files and `json_logs_001m.jsonl`, where the current branch is still larger than C.
4. Review C zstd's compression-level strategy and map likely Rust levels onto the same tradeoff space: fast levels should avoid expensive cost searches, while higher levels may use broader parsing and entropy optimization.
5. Profile matcher search and extension paths again on `decodecorpus_pack.bin` and `json_logs_32m.jsonl`; current samples still show matcher search as the dominant cost and sequence encoding as the secondary JSON target.
6. Investigate further safe early-exit or candidate-pruning heuristics in match selection; keep compression-ratio guardrails in tests and benchmarks and compare every retained idea against C zstd's fast parser shape.
7. Continue C-guided Huffman work only where it improves compression, CPU, or correctness clarity. Candidate areas are table-log/depth selection, literal mode selection, repeat-table reuse, and cost estimation, but previously rejected C heuristics should not be retried without new evidence.
8. Keep adding focused helper-level tests plus emitted-bitstream/Rust-decoder/C-decoder interoperability tests for each compression change; excellent coverage is a hard acceptance criterion for retained work.
9. SIMD remains a matcher byte-comparison topic, but current stable safe-Rust options have not beaten the retained chunked comparison; avoid unsafe/nightly SIMD unless the project explicitly accepts that tradeoff.

## Higher Compression Level Candidates

Compression level policy:

- Level 1 remains the current priority. It should stay close to C zstd fast-level behavior: low parser/search cost, conservative entropy decisions, and no broad exact searches unless they pay for themselves on the PR fixtures and the expanded suite.
- Higher levels are explicitly allowed to spend more CPU for better compression. Promising ideas include larger candidate searches, stronger lazy parsing, broader exact entropy-table selection, more precise block cost modeling, and level-specific match thresholds.
- Any experiment that materially improves size but costs too much CPU for level 1 should be recorded here with its benchmark paths, byte deltas, CPU deltas, and likely target level. Do not discard those notes just because the change is not suitable for the fastest level.
- Benchmark higher-level candidates against C zstd at matching levels, not only against C zstd `-1`. Use C's reported parameter sets as guidance for acceptable tradeoffs: fast levels use cheap hash/search settings, while mid/higher levels can use larger tables, deeper search, greedy/lazy parsing, and stronger entropy choices.
- Keep the same quality bar for higher levels: safe idiomatic Rust, no `unsafe` unless the project explicitly accepts a separate tradeoff, and focused tests plus Rust/C decoder round trips for emitted-bitstream changes.

Current candidates that are too expensive or too broad for the current fastest-level acceptance bar, but should be reconsidered when adding higher compression levels:

- Broad exact Huffman table-depth selection: applying `build_smallest_from_counts()` to every new Huffman literal table improved PR fixture sizes by 67 bytes on `decodecorpus_pack.bin` and 4 bytes on `json_logs_32m.jsonl`, and improved several expanded repeated-text boundary cases, but moved decodecorpus CPU from the retained ~0.16s band to ~0.18-0.19s. This is now wired to `CompressionLevel::Best`; keep benchmarking whether it should also apply to `Better` after parser/search level policies exist.
- Broader parser searches that were rejected for level 1 due to CPU cost or small fast-level regressions may be useful above level 1 if guarded by a clear level policy and benchmarked against C zstd's corresponding levels.
- Future level design should record both compression and CPU/RSS deltas against C zstd at matching levels, not only against C zstd `-1`.

## Design Rules From PR Review

- Suffix candidate storage should stay readable and typed: use two `Option<NonZeroU32>` fields for oldest/newest candidate indexes rather than reintroducing the packed `Option<NonZeroU64>` representation.
- Keep one-based `NonZeroU32` only inside compact suffix-store storage. Convert back to `usize` before indexing slices/windows, and keep those conversions checked.
- Access to the current/last committed window entry should go through small helpers or explicit `match` branches so empty-window invariants are obvious. Avoid raw `len() - 1` indexing in new matcher code unless the non-empty invariant is immediately established.
- A window entry may have empty data in edge cases, but suffix insertion must not assume a non-empty committed window or a minimum payload length. Empty and shorter-than-min-match entries should return without indexing suffixes.
- `unwrap()`/`expect()` is acceptable in tests, but production matcher/compression logic should use explicit invariant handling. `unsafe` is not a better alternative to `unwrap()` for these paths.
- Keep casts between `usize` and `u32` intentional. `usize` being 64-bit on the current target is not a problem for local slice positions; the memory savings come from compact stored candidate indexes, not from forcing every transient index into `u32`.

## Benchmark And Reporting Rules

- Keep `tools/benchmark_zstd.py` as the canonical local benchmark harness. It must decode each Rust and C compressed output with C zstd and byte-compare the decompressed bytes with the original fixture.
- Save benchmark reports under `/tmp` unless the user asks for a committed artifact. Current retained report paths are `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-no-progress.csv` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-no-progress.md`.
- Markdown benchmark reports should use a fixed-width table with these columns: `fixture`, `upstream bytes`, `C bytes`, `new bytes`, `% improvement`, `upstream cpu`, `C cpu`, `new cpu`, `% improvement`.
- Treat the PR's four fixtures as the stable review table, not as sufficient proof of compressor quality. Any major parser/Huffman change should also be checked against the broader fixture expansion plan below.
- Compare against both upstream Rust and C zstd `-1`. Compression wins should state whether they are against upstream, C, or both; CPU regressions should be called out even when size improves.
- For profile runs, prefer `perf record -m 64 -F 999 -g` followed by `perf report --stdio --sort=symbol --no-children`, and record in this file which symbols remain dominant.

## Rejected Ideas To Avoid Repeating

- Directly expanding the matcher repeat-probe fixed-array helper into three explicit branches preserved bytes but repeatedly regressed decodecorpus CPU from the retained 0.16s band to about 0.17s.
- Searching only the newest suffix candidate when a hash slot has both oldest and newest candidates regressed compression: decodecorpus grew to 5,437,724 bytes, larger than C zstd's 5,385,951 bytes, with no CPU win.
- Text-only 4 Ki effective suffix hash tables regressed JSON size with no CPU gain; keep the retained 8 Ki C-fast hash scale.
- Narrowing the no-match skip guard to only the primary repeat offset regressed both decodecorpus and JSON without a JSON CPU improvement.
- Forcing sequence/FSE helpers inline, forcing `encode_offset()` inline, specializing all-table FSE state updates, and caching offset-code tables all failed to produce stable CPU wins.
- Stable safe SIMD-adjacent rewrites tried so far, including wider chunk comparisons and `as_chunks::<N>()`, did not beat the retained `chunks_exact` comparison shape.

## Fixture Expansion Plan

The current four PR fixtures are useful for a stable review table, but they are not broad enough to prove best-quality compressor behavior. Keep them for the PR comparison and add a broader byte-verified local validation suite without committing large binary fixtures.

Committed fixture support:

- `tools/generate_zstd_fixtures.py` writes deterministic small and medium fixtures to `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/expanded-fixtures` by default.
- The initial generated suite covers empty/tiny boundaries, 128/256/4096/128 KiB repeated-text boundaries, 1 MiB RLE, repeated text, JSON logs, cross-block repetition, and xorshift data.
- The first byte-verified expanded-suite report was saved to `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-expanded-benchmark.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-expanded-benchmark.csv`.

Initial expanded-suite findings:

- Current branch matches upstream and C exactly on 0, 1, 6, and 63 byte boundary fixtures.
- Current branch is still larger than C on repeated boundary cases: 128 bytes is 130 versus C's 125, 256 bytes is 136 versus C's 132, 4 KiB is 137 versus C's 133, and 128 KiB is 137 versus C's 135.
- Current branch is still larger than C on `json_logs_001m.jsonl`: 61,359 bytes versus C's 59,118 bytes.
- Current branch is smaller than C on `cross_block_001m.bin`, `repeated_text_001m.txt`, `rle_001m.bin`, and `xorshift_001m.bin`.

Planned fixture coverage:

- Silesia corpus or an equivalent mixed real-world corpus, matching the C zstd benchmark style.
- C zstd `datagen` matrix across input sizes and compressibility levels.
- Many small files and tiny payloads, including 0, 1, 6, 63, 128, 256, 4 KiB, and 128 KiB boundary cases.
- Dictionary compression cases, including small samples and dictionary reuse.
- Long-distance and cross-block repetition cases.
- Already-compressed/high-entropy formats to exercise raw fallback and incompressibility gates.
- Generated decodecorpus frame/data pairs where the original data is available for byte comparison.
- Streaming/chunked input cases to catch behavior that whole-buffer tests miss.

Benchmark acceptance rule:

- Every benchmarked compressed output must be decoded with C zstd and compared byte-for-byte against the original fixture.
- The committed PR fixture table may remain small, but optimization decisions should be checked against the broader suite before considering the C-comparison work complete.

## 2026-05-30 - Rejected split short-line probe step by code-like vs config-like text

- Tried to keep the retained code/config threshold split, but make the denser short-line probe step apply only to non-code short-line text.
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-shortline-probestep-split-main.md`
  - `benchmarks/reports/zstd-bench-compare-level1-shortline-probestep-split-broad-local.md`
- Main level-1 guardrails stayed exact:
  - `decodecorpus_pack.bin`: `5,323,478`
  - `json_logs_32m.jsonl`: `690,084`
- Broad-local result versus the retained code/config threshold split baseline:
  - bytes-above-C on worse fixtures moved the wrong way:
    - `1,909 -> 2,035`
  - the dictionary/config wins were preserved, but the code-file wins regressed:
    - `repo_match_generator.rs`: `22,591 -> 22,883`
    - `repo_main.rs`: `2,141 -> 2,181`
- Decision:
  - Reject and revert.
  - The denser probe step is helping both sides of the current short-line path enough that this split is not worth keeping.

## 2026-05-30 - Retained level-1 code-like short-line text split

- Refined the retained short-line text path by splitting it into:
  - code-like short-line text: threshold `6`
  - non-code short-line text: threshold `5`
  - long-line text: threshold `8`
- This was aimed directly at the tradeoff from the retained `6 -> 5` point, where `dict_dictionary.bin` improved but `repo_match_generator.rs` crossed back to slightly worse than C.
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-shortline-code6-vs5-main.md`
  - `benchmarks/reports/zstd-bench-compare-level1-shortline-code6-vs5-broad-local.md`
  - retained binary: `benchmarks/reports/ruzstd-cli-level1-shortline-code6-retained`
- Main level-1 guardrails stayed exact:
  - `decodecorpus_pack.bin`: `5,323,478`
  - `json_logs_32m.jsonl`: `690,084`
- Broad-local result versus the retained short-line-threshold-5 baseline:
  - better / worse / equal vs C moved from `13 / 16 / 3` to `14 / 15 / 3`
  - total bytes above C on the fixtures where we still lose improved:
    - `2,196 -> 1,909`
  - key recovery:
    - `repo_match_generator.rs`: `23,085 -> 22,591`
    - C: `22,797`
  - preserved wins:
    - `dict_dictionary.bin`: `20,667`
    - `dict_systemd-logind.service`: `1,134`
    - `dict_systemd-coredump@.service`: `692`
  - small tradeoff:
    - `repo_main.rs`: `2,140 -> 2,141`
- Decision:
  - Keep this change.
  - It is a better balanced retained point than the pure `6 -> 5` threshold, because it recovers the source-text regression while preserving the broader dictionary/config-text gain.

## 2026-05-30 - Retained level-1 short-line text threshold `6 -> 5`

- Built directly on the retained short-line threshold `6` plus short-line probe-step baseline.
- Lowered the short-line text non-repeat threshold from `6` to `5`, while keeping:
  - the short-line text gate
  - the denser short-line probe step
  - the long-line JSON path unchanged
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-shortline5-vs6step2-main.md`
  - `benchmarks/reports/zstd-bench-compare-level1-shortline5-vs6step2-broad-local.md`
  - retained binary: `benchmarks/reports/ruzstd-cli-level1-shortline5-step2-retained`
- Main level-1 guardrails stayed exact:
  - `decodecorpus_pack.bin`: `5,323,478`
  - `json_logs_32m.jsonl`: `690,084`
- Broad-local result versus the retained short-line-threshold-6 baseline:
  - better / worse / equal vs C moved from `14 / 15 / 3` to `13 / 16 / 3`
  - but total bytes above C on the fixtures where we still lose improved:
    - `2,411 -> 2,196`
  - biggest gains:
    - `dict_dictionary.bin`: `21,157 -> 20,667`
    - `dict_systemd-logind.service`: `1,144 -> 1,134`
    - `dict_systemd-coredump@.service`: `694 -> 692`
    - `repo_main.rs`: `2,141 -> 2,140`
  - main tradeoff:
    - `repo_match_generator.rs`: `22,591 -> 23,085`
    - C is `22,797`, so this point crosses back to slightly worse there
- Decision:
  - Keep this change for now.
  - It is a better total broad-local compression point, but it also means the next move should probably target that `repo_match_generator.rs` regression specifically rather than pushing this threshold line any further.

## 2026-05-30 - Retained level-1 short-line text threshold `7 -> 6`

- Built directly on the retained short-line text threshold plus short-line probe-step baseline.
- Lowered the short-line text non-repeat threshold from `7` to `6`, while keeping:
  - the short-line text gate
  - the denser short-line probe step
  - the long-line JSON path unchanged
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-shortline6-vs7step2-main.md`
  - `benchmarks/reports/zstd-bench-compare-level1-shortline6-vs7step2-broad-local.md`
  - retained binary: `benchmarks/reports/ruzstd-cli-level1-shortline6-step2-retained`
- Main level-1 guardrails stayed exact:
  - `decodecorpus_pack.bin`: `5,323,478`
  - `json_logs_32m.jsonl`: `690,084`
- Broad-local result versus the retained short-line-threshold-7 baseline:
  - better / worse / equal vs C stayed `14 / 15 / 3`
  - total bytes above C on the fixtures where we still lose improved:
    - `3,408 -> 2,411`
  - biggest gains:
    - `dict_dictionary.bin`: `22,101 -> 21,157`
    - `repo_main.rs`: `2,174 -> 2,141`
    - `dict_systemd-coredump@.service`: `708 -> 694`
    - `dict_systemd-logind.service`: `1,150 -> 1,144`
  - main tradeoff:
    - `repo_match_generator.rs`: `22,469 -> 22,591`
    - but C is still `22,797`, so this point remains on the better-than-C side there
- Decision:
  - Keep this change.
  - This is the best retained level-1 broad-local point so far.

## 2026-05-30 - Retained level-1 short-line text probe step `3 -> 2`

- Built directly on the retained short-line text threshold gate.
- Kept the retained short-line `7`-byte threshold and additionally used the denser no-match probe step only on short-line text blocks.
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-shortline7-step2-vsstep3-main.md`
  - `benchmarks/reports/zstd-bench-compare-level1-shortline7-step2-vsstep3-broad-local.md`
  - retained binary: `benchmarks/reports/ruzstd-cli-level1-shortline7-step2-retained`
- Main level-1 guardrails stayed exact:
  - `decodecorpus_pack.bin`: `5,323,478`
  - `json_logs_32m.jsonl`: `690,084`
- Broad-local result versus the retained short-line-threshold baseline:
  - better / worse / equal vs C moved to `14 / 15 / 3`
  - total bytes above C on the fixtures where we still lose improved:
    - `4,059 -> 3,408`
  - biggest gains:
    - `dict_dictionary.bin`: `22,634 -> 22,101`
    - `repo_match_generator.rs`: `22,876 -> 22,469`
    - `repo_main.rs`: `2,211 -> 2,174`
    - `dict_systemd-logind.service`: `1,152 -> 1,150`
- Decision:
  - Keep this change.
  - This is the best retained level-1 broad-local point so far, and it achieves it without reopening the main JSON failure.

## 2026-05-30 - Retained level-1 short-line text threshold `8 -> 7`

- Replaced the earlier `64 KiB` size gate with a text-shape gate.
- The matcher now uses `7` for short-line text blocks, rather than only for text blocks below a size cutoff.
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-shortline7-vs64k-main.md`
  - `benchmarks/reports/zstd-bench-compare-level1-shortline7-vs64k-broad-local.md`
  - retained binary: `benchmarks/reports/ruzstd-cli-level1-shortline7-retained`
- Main level-1 guardrails stayed exact:
  - `decodecorpus_pack.bin`: `5,323,478`
  - `json_logs_32m.jsonl`: `690,084`
- Broad-local result versus the retained `64 KiB` gate:
  - better / worse / equal vs C stayed `13 / 16 / 3`
  - total bytes above C on the fixtures where we still lose improved:
    - `4,908 -> 4,059`
  - main gain:
    - `repo_match_generator.rs`: `23,725 -> 22,876`
    - C `zstd -1`: `22,797`
  - earlier retained wins stayed in place:
    - `dict_dictionary.bin`: `22,634`
    - `repo_main.rs`: `2,211`
    - `dict_systemd-logind.service`: `1,152`
    - `dict_systemd-coredump@.service`: `708`
- Decision:
  - Keep this change.
  - It is the cleanest retained step so far toward closing the remaining broad-local source-text gap without disturbing JSON.

## 2026-05-30 - Retained level-1 small-text threshold `8 -> 7` for text blocks under `64 KiB`

- Untangled the matcher’s text-path policy from the retained `TEXT_MIN_NON_REPEAT_MATCH_LEN = 8` constant:
  - text classification now owns the text probe-step and large text window behavior
  - the non-repeat threshold can vary without silently switching those other text-path behaviors off
- Kept a narrower level-1 text-path change on top of that refactor:
  - use `7` only for text blocks smaller than `64 KiB`
  - keep `8` on the normal `128 KiB` streaming text blocks
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-smalltext7-64k-vs8-main.md`
  - `benchmarks/reports/zstd-bench-compare-level1-smalltext7-64k-vs8-broad-local.md`
  - retained binary: `benchmarks/reports/ruzstd-cli-level1-smalltext7-64k-retained`
- Main level-1 guardrails stayed exact:
  - `decodecorpus_pack.bin`: `5,323,478`
  - `json_logs_32m.jsonl`: `690,084`
- Broad-local result versus the retained threshold-8 baseline:
  - better / worse / equal vs C stayed `13 / 16 / 3`
  - but total bytes above C on the fixtures where we still lose improved:
    - `6,578 -> 4,908`
  - biggest wins:
    - `dict_dictionary.bin`: `24,237 -> 22,634`
    - `repo_main.rs`: `2,249 -> 2,211`
    - `dict_systemd-logind.service`: `1,175 -> 1,152`
    - `dict_systemd-coredump@.service`: `722 -> 708`
  - known tradeoff:
    - `repo_match_generator.rs`: `23,717 -> 23,725`
- Decision:
  - Keep this change.
  - It is the strongest retained level-1 compression step since the original `10 -> 8` threshold change, and it does it without reopening the main JSON failure.

## 2026-05-30 - Rejected level-1 small-text threshold `8 -> 7` for text blocks under `256 KiB`

- First attempted to salvage the earlier global `8 -> 7` win by restricting it to text blocks under `256 KiB`.
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-smalltext7-vs8-main.md`
  - `benchmarks/reports/zstd-bench-compare-level1-smalltext7-vs8-broad-local.md`
- Broad-local direction looked strong:
  - `dict_dictionary.bin`: `24,237 -> 22,634`
  - `repo_match_generator.rs`: `23,717 -> 22,876`
  - `dict_systemd-logind.service`: `1,175 -> 1,152`
- But it still failed the main JSON guardrail because the compressor works in `128 KiB` blocks, so a `256 KiB` cutoff still hit every large JSON block:
  - `json_logs_32m.jsonl`: `690,084 -> 809,823`
  - `decodecorpus_pack.bin`: `5,323,478 -> 5,324,210`
- Decision:
  - Reject and narrow the cutoff below the block size.

## Archive Inspection Findings

- Added an ignored internal diagnostic test, `inspect_archive_from_env`, in `ruzstd/src/tests/mod.rs`. It parses a `.zst` frame with the existing decoder internals and reports frame size, block counts, literal-section payload bytes, sequence payload bytes, decoded literal bytes, and match bytes. Use it with:
  - `env RUZSTD_INSPECT_FRAME=/path/to/archive.zst cargo test -q -p ruzstd inspect_archive_from_env -- --ignored --nocapture`
  - `RUZSTD_INSPECT_VERBOSE_BLOCKS=1` only when a per-block dump is needed.
- Comparing `decodecorpus_pack.bin` at C `-4` versus current Rust level 4 showed that we have not reached parity on that fixture:
  - C `-4`: `4,789,813` bytes, `564` blocks, `438` compressed, `123` raw, `3` RLE, `literal_section_bytes=2,534,696`, `sequence_payload_bytes=1,242,862`.
  - Rust level 4: `5,107,302` bytes, `89` blocks, `83` compressed, `6` raw, `0` RLE, `literal_section_bytes=3,109,308`, `sequence_payload_bytes=1,210,679`.
- The decisive gap on that fixture is not sequence entropy coding. Rust's sequence payload is already slightly smaller than C's, while Rust spends about `575` KiB more in literal payload bytes. C is winning by isolating incompressible or weakly compressible regions into many smaller raw/compressed/RLE blocks, not by some hidden FSE/Huffman mode we are missing.
- The first per-block comparison confirms the same shape: C quickly falls back to many `8 KiB`-scale blocks and mixes raw/RLE decisions, while Rust keeps almost everything in `128 KiB` blocks. That strongly points to adaptive block splitting or block-size selection as the next best implementation path for `Best`.
- The level-1 control run does not show the same problem. On `decodecorpus_pack.bin`, Rust level 1 is already smaller than C `-1` (`5,324,267` versus `5,385,951`) even though Rust uses fewer, larger blocks. That suggests the remaining gap is mainly a higher-level block modeling decision, not a universal low-level entropy deficiency.
- Retained a `Best`-only adaptive block partitioner driven by sustained `8 KiB` binary-like incompressible runs. Split sub-blocks still attempt full compression first and only fall back to raw if the compressed block loses, which avoids the heavy regressions from the rejected eager-raw split variants.
- Current retained `Best` benchmark: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-adaptive-split-l4.md`
  - `decodecorpus_pack.bin`: `5,061,522` bytes versus previous retained `5,107,302` and C `4,789,813`
  - `json_logs_32m.jsonl`: `561,407`
  - `repeated_text_32m.txt`: `2,874`
  - `xorshift_32m.bin`: `33,555,210`
  - CPU tradeoff: decodecorpus `0.69s` versus previous retained `0.30s`; JSON unchanged at `0.24s`
- Level 1 guardrail remained exact on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-adaptive-split-l1.md`: decodecorpus `5,324,267`, JSON `690,084`, repeated text `2,874`, and xorshift `33,555,210`, with the same CPU band as the retained level-1 baseline.
- Refactored the compressed-block encoder so match/literal collection is separated from bitstream emission. `PreparedBlock` and `PreparedSequence` now preserve raw offsets until encode time, which makes post-sequence block splitting implementable without unsafe code and without rebuilding matcher output for every candidate.
- Retained a `Best`-level exact midpoint post-sequence split candidate on top of that refactor, now guarded by a cheap entropy-style gate over literal bytes and sequence code distributions. This stays closer to C zstd's estimate-then-split shape than the earlier raw-byte heuristics while avoiding obviously homogeneous blocks. After that, removed avoidable trial-path copies by switching the split candidate to borrowed block data and borrowed prepared-block views instead of cloning the committed block and both split halves, and then narrowed split-trial FSE snapshots so they clone only the mutable previous-table state instead of the immutable default tables. Latest benchmark reports: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-exact-mid-split-prevonly-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-exact-mid-split-prevonly-l1.md`.
  - `decodecorpus_pack.bin`: `5,019,141` bytes versus adaptive split `5,061,522` and C `4,789,813`
  - `json_logs_32m.jsonl`: `549,139` bytes versus adaptive split `561,407` and C `1,361,274`
  - `repeated_text_32m.txt`: unchanged at `2,874`
  - `xorshift_32m.bin`: unchanged at `33,555,210`
  - CPU tradeoff after borrowed inputs plus previous-table-only snapshots: decodecorpus `0.74s`, JSON `0.27s`, repeated text `0.02s`
  - Level 1 remained byte-stable and CPU-stable on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-exact-mid-split-prevonly-l1.md`.
- Replaced the one-shot exact midpoint duel with a recursive estimate-driven partitioner for `Best`, following C zstd's `deriveBlockSplits()` shape more closely: recurse only when the split estimate beats the unsplit estimate, then emit the chosen partitions once instead of exact-encoding unsplit and split candidates for every trial. The first uncapped version (`/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-estimated-recursive-l4.md`) substantially improved compression again but pushed decodecorpus CPU back up to `0.79s`: `decodecorpus_pack.bin` `4,910,850`, `json_logs_32m.jsonl` `546,249`.
- Archive inspection of that uncapped recursive result showed why the CPU and overhead were still off: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-decodecorpus-recursive-l4.zst` had `1,021` blocks (`897` compressed / `109` raw / `15` RLE) versus C `-4`'s `564`, even though Rust's literal payload (`2,299,229`) and sequence payload (`1,218,256`) were already smaller than C's. That points to entropy/table overhead on many small compressed partitions, not a missing matcher payload win.
- Retained a level-aware FSE repeat-table policy for `Best`: split-heavy higher-level blocks may now reuse the previous FSE table up to `256` sequences instead of the level-1 `64`-sequence cap. This reduced the recursive-split overhead without changing level 1. Benchmark reports: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-repeat256-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-repeat256-l1.md`.
  - `decodecorpus_pack.bin`: `4,963,785` bytes versus previous retained `5,019,141` and C `4,789,813`
  - `json_logs_32m.jsonl`: `546,249` bytes versus previous retained `549,139` and C `1,361,274`
  - `repeated_text_32m.txt`: unchanged at `2,874`
  - `xorshift_32m.bin`: unchanged at `33,555,210`
  - CPU tradeoff: decodecorpus stayed in the previous retained band at `0.74s` while JSON improved to `0.25s`
  - Level 1 remained byte-stable on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-repeat256-l1.md`: decodecorpus `5,324,267`, JSON `690,084`, repeated text `2,874`, xorshift `33,555,210`
- Rejected a `Best`-only dual-hash matcher experiment modeled after C `ZSTD_dfast`'s `hashSmall`/`hashLong` split. The Rust version added a second long-key suffix store and searched it before the existing small-key store. Compression improved, but the current matcher architecture paid far too much CPU for that extra state and search.
  - Full dual-hash search across all window entries: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-dualhash-l4.md`
    - `decodecorpus_pack.bin`: `4,925,507` bytes, but CPU regressed to `1.30s`
    - `json_logs_32m.jsonl`: `540,965` bytes, but CPU regressed to `0.43s`
  - Narrowing the long-hash search to only the two most recent window entries reduced the regression but was still not viable: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-dualhash-limit2-l4.md`
    - `decodecorpus_pack.bin`: `4,931,334` bytes, CPU `1.04s`
    - `json_logs_32m.jsonl`: `544,926` bytes, CPU `0.38s`
  - Conclusion: the extra hash by itself is not enough. C `dfast` couples its dual-hash state with a much tighter parser loop and cheaper table updates than the current Rust matcher. Porting only the second hash store into the existing per-entry search shape is not a keeper.
- Rejected shallow `Best`-level window-entry search caps as another attempted approximation of C `ZSTD_dfast`'s `searchLog=1`. They did reduce CPU on `decodecorpus_pack.bin`, but every version gave back far too much binary-like compression to keep.
  - Newest-entry-only search was immediately non-viable: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-newest-only-l4.md`
    - `decodecorpus_pack.bin`: `5,469,638` bytes, CPU `0.79s`
    - `json_logs_32m.jsonl`: `605,890` bytes, CPU `0.22s`
  - Capping all blocks to the two most recent window entries improved decodecorpus CPU materially, but compression collapsed: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-searchlimit2-l4.md`
    - `decodecorpus_pack.bin`: `5,223,089` bytes, CPU `0.45s`
    - `json_logs_32m.jsonl`: `677,548` bytes, CPU `0.22s`
  - Restricting the cap to binary-like blocks preserved JSON but still lost too much decodecorpus compression:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-searchlimit2-binaryonly-l4.md`: decodecorpus `5,223,649` at `0.48s`, JSON `546,247` at `0.24s`
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-searchlimit3-binaryonly-l4.md`: decodecorpus `5,174,288` at `0.50s`, JSON `546,252` at `0.24s`
  - Conclusion: the current Rust matcher cannot approximate C `dfast` by simply scanning fewer prior window entries. The retained `Best` baseline stays uncapped, and the next matcher step should target a more faithful parser/probe loop rather than another shallow search cap.
- Retained a `Best`-only adaptive binary no-match probe step modeled after C fast/dfast's growing skip distance after long literal runs. Instead of always probing binary-like blocks at a fixed step of 2, `Best` now grows the binary miss step by `literal_run_len >> 8` while text-like blocks keep the existing step-3 path. This reduces `Best` matcher CPU without changing level 1 and without another broad search-cap heuristic.
  - Retained reports: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-adaptive-binary-step-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-adaptive-binary-step-l1.md`
  - Compared to the previous retained `Best` table `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-repeat256-l4.md`:
    - `decodecorpus_pack.bin`: `4,963,785` -> `4,965,276` bytes, CPU `0.74s` -> `0.68s`
    - `json_logs_32m.jsonl`: unchanged at `546,249` bytes, CPU `0.25s` -> `0.23s`
    - `repeated_text_32m.txt`: unchanged at `2,874` bytes, CPU remained in the `0.02s`/`0.03s` noise band
    - `xorshift_32m.bin`: unchanged at `33,555,210` bytes, CPU stayed at `0.05s`
  - Level 1 remained byte-stable on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-adaptive-binary-step-l1.md`: decodecorpus `5,324,267`, JSON `690,084`, repeated text `2,874`, xorshift `33,555,210`
  - Rejected a less aggressive threshold (`literal_run_len >> 9`) after benchmarking `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-adaptive-binary-step-threshold9-l4.md`: it drifted decodecorpus slightly further to `4,965,486` bytes with no meaningful CPU improvement versus the retained `>> 8` variant.
- Retained a narrow `Best`-only next-position window-match lookahead modeled after C `ZSTD_dfast`'s `ip+1` long-match check. When a binary-like `Best` block finds only a minimum-length normal hash match at the current position, the matcher now probes one byte ahead and prefers the next-position window match if it is strictly longer. This is the first parser-shape change beyond skip tuning that materially improves decodecorpus compression again.
  - Retained reports: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-nextpos-repeat-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-nextpos-l1.md`
  - Compared to the previous retained adaptive-step table `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-adaptive-binary-step-l4.md`:
    - `decodecorpus_pack.bin`: `4,965,276` -> `4,958,219` bytes, with repeat CPU holding in the same band at `0.68s` -> `0.65s`
    - `json_logs_32m.jsonl`: unchanged at `546,249` bytes, repeat CPU `0.23s` -> `0.24s`
    - `repeated_text_32m.txt`: unchanged at `2,874`
    - `xorshift_32m.bin`: unchanged at `33,555,210`
  - Level 1 remained byte-stable on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-nextpos-l1.md`: decodecorpus `5,324,267`, JSON `690,084`, repeated text `2,874`, xorshift `33,555,210`
  - The first level-4 run (`/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-nextpos-l4.md`) showed decodecorpus at `0.70s` and JSON at `0.25s`, but the repeat table stabilized to `0.65s` / `0.24s`. Keep the repeated measurement as the retained benchmark and treat the first run as noise.
- Retained a second narrow `Best`-only parser decision from C `ZSTD_dfast`: an `ip+1` repeat-offset lookahead. When a binary-like `Best` block has no current candidate, or only a minimum-length normal window match at the current position, the matcher now probes repeat offsets one byte ahead and can prefer that shifted repeat match. This closes another chunk of the decodecorpus gap while leaving level 1 untouched.
  - Retained reports: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-nextrep-repeat-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-nextrep-l1.md`
  - Compared to the previous retained next-position window-match table `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-nextpos-repeat-l4.md`:
    - `decodecorpus_pack.bin`: `4,958,219` -> `4,945,062` bytes, with repeat CPU `0.65s` -> `0.70s`
    - `json_logs_32m.jsonl`: `546,249` -> `546,250` bytes, repeat CPU unchanged at `0.24s`
    - `repeated_text_32m.txt`: unchanged at `2,874`
    - `xorshift_32m.bin`: unchanged at `33,555,210`
  - Level 1 remained byte-stable on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-nextrep-l1.md`: decodecorpus `5,324,267`, JSON `690,084`, repeated text `2,874`, xorshift `33,555,210`
  - The first level-4 run (`/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-nextrep-l4.md`) measured decodecorpus at `0.69s` and the repeated run at `0.70s`; treat `0.70s` as the retained CPU figure.
- Retained a `Best`-only complementary end insertion in the sparse post-match indexer, mirroring more of C `ZSTD_dfast`'s post-match table updates (`curr`, `curr+2`, `ip-2`, plus a higher-level-only `ip-1` equivalent). The Rust matcher now keeps the extra `match_end - 1` suffix only for `Best`, which preserves the higher-level decodecorpus gain without moving level 1.
  - Retained reports: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-complementary-bestonly-repeat-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-complementary-bestonly-repeat-l1b.md`
  - Compared to the previous retained next-repeat table `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-nextrep-repeat-l4.md`:
    - `decodecorpus_pack.bin`: `4,945,062` -> `4,942,962` bytes, with repeat CPU `0.70s` -> `0.68s`
    - `json_logs_32m.jsonl`: unchanged at `546,250` bytes, repeat CPU `0.24s` -> `0.24s`
    - `repeated_text_32m.txt`: unchanged at `2,874`
    - `xorshift_32m.bin`: unchanged at `33,555,210`
  - Level 1 remained byte- and CPU-stable on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-complementary-bestonly-repeat-l1b.md`: decodecorpus `5,324,267` at `0.17s`, JSON `690,084` at `0.12s`, repeated text `2,874`, xorshift `33,555,210`
  - An unrestricted version that applied the extra end insertion to every level was not kept. It improved decodecorpus bytes at both levels, but it also moved level-1 decodecorpus CPU into the `0.18s` band. Keep the narrower `Best`-only variant as the retained tradeoff.
- Rejected a cached immediate repeat-chain path intended to mirror C `ZSTD_dfast`'s post-match zero-literal repcode loop. The implementation cached the first zero-literal repeat candidate after a stored match so the next `next_sequence()` call could emit it directly instead of rediscovering it through the normal repeat/window search.
  - Fresh benchmark reports: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-repeatchain-fresh-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-repeatchain-fresh-l1.md`
  - It was not a CPU-only cleanup in practice. `decodecorpus_pack.bin` drifted backward from the retained `4,942,962` to `4,946,573`, and `json_logs_32m.jsonl` regressed sharply from `546,250` to `602,790` bytes at `Best`. Level 1 bytes stayed stable, but decodecorpus CPU drifted into the `0.18s` band.
  - Conclusion: the cached chain changed parser decisions in the wrong places rather than just removing rediscovery overhead, so it was fully reverted.
- Rejected a repeat-first lazy hash-key computation cleanup. The matcher was changed to defer `SuffixStore::key_value()` until after repeat probing so repeat-heavy positions that never need window search would skip the hash-key work.
  - Benchmark reports: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-lazykey-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-lazykey-l1.md`
  - Output bytes stayed unchanged, but CPU moved the wrong way: `decodecorpus_pack.bin` drifted from the retained `0.68s`/`0.17s` bands to `0.75s` at `Best` and `0.19s` at level 1.
  - Conclusion: the extra control-flow shape was worse than the always-compute baseline, so the change was reverted.
- Retained a broader `Best`-only next-position window lookahead, extending the earlier `ip+1` normal-match check to cover the no-candidate case as well. This is closer to the adjacent-position search shape in C fast/dfast: when the current position has no candidate at all, `Best` now also checks for a normal window match at `ip+1` before falling back to the miss path.
  - Retained reports: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-nextposnone-repeat-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-nextposnone-guarded-l1.md`
  - Compared to the previous retained `Best` table `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-complementary-bestonly-repeat-l4.md`:
    - `decodecorpus_pack.bin`: `4,942,962` -> `4,902,872` bytes, with repeat CPU moving from `0.68s` to `0.76s`
    - `json_logs_32m.jsonl`: unchanged at `546,250` bytes, repeat CPU stayed in the `0.24s` band
    - `repeated_text_32m.txt`: unchanged at `2,874`
    - `xorshift_32m.bin`: unchanged at `33,555,210`
  - The first implementation left the `Best`-only lookahead helper calls on the hot path even when disabled, and the level-1 control run drifted upward. Guarding those call sites behind the `Best` flags restored the level-1 control band on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-nextposnone-guarded-l1.md`: decodecorpus `5,324,267` at `0.17s`, JSON `690,084` at `0.12s`, repeated text `2,874`, xorshift `33,555,210`.
  - Keep the guarded version as the retained tradeoff: materially better decodecorpus compression for `Best`, no byte movement at level 1, and no always-on overhead for disabled lookahead paths.
- Rejected a `Best`-only sparse `ip+1` insertion in the post-match indexer. This was meant to mirror C fast/dfast's safe `ip1` hash write before jumping past a match, by adding `start+1` alongside the existing sparse `start`, `start+2`, and tail insertions.
  - Benchmark reports: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-ip1insert-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-ip1insert-l1.md`
  - Result: essentially no compression movement beyond noise (`decodecorpus_pack.bin` only shifted from `4,902,872` to `4,902,869`) while CPU stayed worse than the retained no-candidate next-position lookahead (`0.78s` vs `0.76s` on decodecorpus at `Best`, and level-1 decodecorpus stayed in the `0.18s` band with JSON drifting to `0.13s`).
  - Conclusion: the retained `Best` no-candidate `ip+1` search win is real, but adding the extra sparse `start+1` insertion on top of it is not paying for itself, so it was reverted.
- Rejected a one-pass paired window scan for the retained `Best` no-candidate `ip+1` lookahead. The idea was to compute current and next-position window candidates in one pass over the history window, instead of scanning once for the current position and then again for the `ip+1` normal-match lookahead.
  - Benchmark reports: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-pairscan-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-pairscan-l1.md`
  - Output bytes stayed identical to the retained no-candidate `ip+1` parser result, but CPU regressed: `decodecorpus_pack.bin` moved from `0.76s` to `0.80s` at `Best`, and the level-1 control run drifted from `0.17s` to `0.18s`.
  - Conclusion: the extra coupled-control-flow shape was worse than the simpler two-pass retained implementation, so it was reverted.
- Tested raising the recursive partition cap back to `16` after the broader FSE repeat-table reuse. Report: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-repeat256-cap16-l4.md`. This recovered the stronger uncapped decodecorpus size (`4,910,654`) but moved decodecorpus CPU back to `0.78s`, so the retained `Best` configuration keeps the lower partition cap.
- Tested removing the older `Best` pre-segmentation layer once recursive post-sequence splitting existed. The existing mixed-block regression immediately failed because the fixture stopped splitting at all, so the pre-segmentation layer was restored and remains part of the retained `Best` path for now.
- The ungated exact midpoint split (`/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-exact-mid-split-l4.md`) stayed slightly smaller at `5,018,491` / `548,113` but cost more CPU (`0.81s` / `0.32s`). Keep the entropy-gated variant as the better current tradeoff.
- Tested raising the entropy-gate threshold to `1024` and `2048` estimated bits. Reports: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-exact-mid-split-threshold1024-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-exact-mid-split-threshold2048-l4.md`. Both reduced the JSON compression gain materially for only marginal CPU movement on decodecorpus and JSON, so the retained threshold remains `512`.
- Tested an additional literal-ratio gate for the exact midpoint split. The gated run (`/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-exact-mid-split-gated-l4.md`) reduced the JSON gain back to zero while still leaving decodecorpus CPU high, so the gate was rejected.
- `perf` on the exact midpoint split path (`/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-decodecorpus-best-exact-mid.perf.data` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-json-best-exact-mid.perf.data`) confirmed that matcher search is still the dominant cost even at `Best` (`~66-72%` of samples), with the new splitter overhead showing up secondarily in FSE-table cloning, Huffman length-limited work, and candidate re-encoding. Borrowing the split inputs and narrowing the FSE snapshots removed some of that secondary cost, but matcher search still dominates.
- A fresh profile on the retained recursive/cap-8 `Best` candidate (`/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-decodecorpus-best-recursive-cap8.perf.data`) showed the same overall shape: matcher search still dominated decodecorpus at about `65%`, while `best_mid_split_estimate()` itself was only about `1.6%` of samples. The more meaningful non-matcher costs were Huffman work and FSE table handling, which is why the broader `Best` repeat-table reuse was a better next CPU lever than optimizing the split estimator itself.
- Rejected a safe Rust port of C's `zstd_preSplit.c` pre-block fingerprint splitter as the next retained step. Benchmark reports are `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-c-presplit-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-c-presplit-l1.md`. On level 4 it moved `decodecorpus_pack.bin` to `5,067,556` bytes with decodecorpus CPU back down to `0.30s`, but it was still worse than the retained adaptive split on bytes (`5,061,522`) and it badly regressed `repeated_text_32m.txt` CPU from `0.02s` to `0.11s` with no size win. Level 1 stayed byte-identical. Source review also showed why: C `-4` (`ZSTD_dfast`) does not auto-enable this pre-split path; the automatic block splitting path in current C zstd is the post-sequence splitter (`postBlockSplitter`) used for higher strategies such as `btopt` and above. Keep the result as evidence that the next parity step should target post-sequence block splitting or equivalent block-cost modeling, not `zstd_preSplit.c` alone.
- Refreshed the current level-4 archive inspection on `decodecorpus_pack.bin` after the retained parser work. Compared with C `zstd -4`, the remaining gap is no longer about literal entropy. The current retained Rust frame had `843` blocks versus C's `564`, with `719` compressed blocks versus `438` and much more data pushed into split-heavy tiny blocks. Even though Rust's compressed literal-section bytes were already smaller than C's (`2,254,119` versus `2,534,696`), the overall frame was larger because the split policy emitted too many blocks and too much raw/RLE material. That redirected the next work from matcher heuristics back to block-cost policy.
- Retained a `Best`-only exact whole-block fallback for cap-saturated non-text recursive split candidates. When `derive_best_partitions()` hits the full `BEST_SPLIT_MAX_PARTITIONS` budget on a non-text block, the compressor now exact-encodes the unsplit prepared block once and keeps it if the partitioned candidate was a false win. The retained implementation defers the FSE previous-table snapshot unless that fallback is actually needed, so text-heavy `Best` blocks avoid the extra clone cost.
  - Retained reports: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-saturated-nontext-whole-compare-fse-lazy-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-saturated-nontext-whole-compare-fse-lazy-l1.md`
  - Compared to the previous retained `Best` table `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-nextposnone-repeat-l4.md`:
    - `decodecorpus_pack.bin`: `4,902,872` -> `4,894,877` bytes, with CPU holding in the same band on the latest timed run (`0.85s` -> `0.77s` in that run; earlier repeats were `0.79-0.80s` for both old and new)
    - `json_logs_32m.jsonl`: unchanged at `546,250` bytes, with CPU moving only slightly from `0.24s` to `0.25s`
    - `repeated_text_32m.txt`: unchanged at `2,874`
    - `xorshift_32m.bin`: unchanged at `33,555,210`
  - Level 1 remained byte- and CPU-stable on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-saturated-nontext-whole-compare-fse-lazy-l1.md`: decodecorpus `5,324,267` at `0.17s`, JSON `690,084` at `0.12s`, repeated text `2,874`, xorshift `33,555,210`
  - Fresh archive inspection on the retained decodecorpus candidate reduced block count from `843` to `815` and compressed size from `4,902,872` to `4,894,877`, confirming that the fallback is trimming the worst over-split cases rather than changing match generation.
- Rejected broader exact whole-vs-split comparisons and a lower recursive partition cap from the same investigation:
  - Whole-vs-split comparison on every split block: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-whole-vs-split-l4.md` improved decodecorpus further to `4,892,549`, but decodecorpus CPU regressed from `0.80s` to `0.84s` and JSON CPU regressed from `0.24s` to `0.29s`.
  - Lowering the recursive partition cap to `6` without the exact fallback cut decodecorpus block count sharply, but compression regressed to `4,916,065`; with the exact fallback it recovered only to `4,902,575`, still worse than the retained cap-8 non-text fallback path.
  - Running the cap-saturated exact fallback while eagerly cloning the previous FSE tables on every `Best` block was also rejected: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-saturated-nontext-whole-compare-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-saturated-nontext-whole-compare-repeat-l4.md` held the decodecorpus gain, but JSON CPU stayed stuck around `0.26s`. Deferring that snapshot until the fallback is actually used recovered most of the unnecessary text-block overhead.
- Retained a decompressed-byte-balanced recursive split midpoint for `Best`. Instead of always splitting recursive candidates at `sequence_count / 2`, the estimator now chooses the sequence boundary whose cumulative literal+match bytes are closest to half of the prepared block's decompressed size. This is closer to C's byte-range reasoning than the old count-only midpoint and avoids generating lopsided recursive halves when one side carries most of the literals or match bytes.
  - Retained reports: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-byte-mid-split-l4.md`, `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-byte-mid-split-repeat-l4.md`, and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-byte-mid-split-l1.md`
  - Compared to the previous retained `Best` table `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-nextposnone-repeat-l4.md`:
    - `decodecorpus_pack.bin`: `4,902,872` -> `4,877,832` bytes, with repeat CPU improving from `0.81s` to `0.78s`
    - `json_logs_32m.jsonl`: `546,250` -> `545,174` bytes, with CPU holding in the same `0.24s/0.25s` band
    - `repeated_text_32m.txt`: unchanged at `2,874`
    - `xorshift_32m.bin`: unchanged at `33,555,210`
  - Level 1 remained byte- and CPU-stable on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-byte-mid-split-l1.md`: decodecorpus `5,324,267` at `0.17s`, JSON `690,084` at `0.12s`, repeated text `2,874`, xorshift `33,555,210`
  - Fresh archive inspection on the retained decodecorpus candidate moved the frame to `4,877,832` bytes. Compared with the previous retained non-text whole-block fallback, compressed-block count fell from `691` to `682`, decoded literal bytes fell from `2,614,738` to `2,549,340`, and raw block count rose modestly from `109` to `116`. The important result is that the better byte-balanced split point produced fewer literals and smaller compressed blocks even without reducing total block count much.
- Retained a lower `Best` pre-segmentation threshold for incompressible runs: `BEST_SPLIT_MIN_INCOMPRESSIBLE_RUN_CHUNKS` now triggers after `2` consecutive `8 KiB` binary-like chunks instead of `3`. This leaves level 1 untouched but gives `Best` a more C-like willingness to isolate medium-length incompressible streaks before they distort the later block-cost decisions.
  - Retained reports: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-runlen2-byte-mid-l4.md`, `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-runlen2-byte-mid-l1.md`, and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-runlen2-byte-mid-repeat-l1.md`
  - Compared to the previous retained `Best` table `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-byte-mid-split-repeat-l4.md`:
    - `decodecorpus_pack.bin`: `4,877,832` -> `4,846,130` bytes, with CPU holding in the same band at `0.78s`
    - `json_logs_32m.jsonl`: unchanged in the same band but slightly smaller, `545,174` bytes at `0.24s`
    - `repeated_text_32m.txt`: unchanged at `2,874`
    - `xorshift_32m.bin`: unchanged at `33,555,210`
  - Level 1 remained byte-stable and the repeat guardrail restored the prior CPU band on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-runlen2-byte-mid-repeat-l1.md`: decodecorpus `5,324,267` at `0.17s`, JSON `690,084` at `0.12s`, repeated text `2,874`, xorshift `33,555,210`
  - Fresh archive inspection on decodecorpus after the retained change moved the frame to `4,846,130` bytes. Compared with the previous retained byte-balanced split, compressed-block bytes fell from `3,425,514` to `3,393,166`, decoded literal bytes fell from `2,549,340` to `2,516,986`, and match bytes increased from `7,162,880` to `7,391,171`. The raw-byte total rose slightly, so the gain came from giving the surrounding compressed blocks better structure, not from directly shrinking raw regions.
- Retained a further `Best` pre-segmentation step: isolate even a single `8 KiB` binary-like incompressible chunk at the top level. `BEST_SPLIT_MIN_INCOMPRESSIBLE_RUN_CHUNKS` is now `1`. This is the first change in this line that materially closes the decodecorpus gap to C: it pushes `decodecorpus_pack.bin` to within about `5.5 KiB` of C `-4`, while level 1 remains unchanged.
  - Retained reports: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-runlen1-clean-l4.md`, `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-runlen1-byte-mid-l1.md`
  - Compared to the previous retained `Best` table `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-runlen2-byte-mid-l4.md`:
    - `decodecorpus_pack.bin`: `4,846,130` -> `4,795,318` bytes, with CPU moving from `0.78s` to `0.86s`
    - `json_logs_32m.jsonl`: unchanged at `545,174` bytes, with CPU still in the `0.24s/0.25s` band
    - `repeated_text_32m.txt`: unchanged at `2,874`
    - `xorshift_32m.bin`: unchanged at `33,555,210`
  - Level 1 remained byte-stable on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-runlen1-byte-mid-l1.md`: decodecorpus `5,324,267`, JSON `690,084`, repeated text `2,874`, xorshift `33,555,210`; CPU stayed in the same band (`decodecorpus 0.17s`, JSON 0.11s/0.12s).
  - Fresh archive inspection on decodecorpus after the retained change moved the frame to `4,795,318` bytes. Relative to the run-length-2 retained state, raw input bytes fell from `1,450,525` to `1,284,189`, decoded literals rose back toward C (`2,516,986` -> `2,606,833`), and match bytes rose further (`7,391,171` -> `7,508,620`). The overall gain came from isolating more single-chunk incompressible regions so the adjacent compressed regions could emit significantly more match bytes.
- Fresh `perf` samples on the retained run-length-1 state confirm that `MatchGeneratorDriver::start_matching` is still the dominant encode CPU cost at `Best`: about `66-68%` of `decodecorpus_pack.bin` samples and about `70-71%` of `json_logs_32m.jsonl` samples, saved in `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-decodecorpus-best-runlen1.perf.data` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/perf-data/ruzstd-json-best-runlen1.perf.data`.
- Rejected decompressed child-size floors on recursive split candidates after comparing them against the retained byte-balanced midpoint:
  - `8 KiB` minimum child size: decodecorpus block count dropped sharply to `479` compressed blocks, but bytes regressed to `4,936,825`.
  - `4 KiB` minimum child size: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-saturated-nontext-plus-min4k-l4.md` improved CPU slightly on decodecorpus (`0.82s` -> `0.79s`) but only reached `4,897,262` bytes, worse than the retained byte-balanced midpoint, with no JSON size gain.
  - Conclusion: the over-splitting problem was more about where we split than a simple minimum child size. Choosing a better split boundary by decompressed bytes was the better fix.
- Follow-up inspection of the retained byte-balanced split result showed C still had zero raw blocks above `8 KiB`, while Rust still had `13` such raw blocks. Experiments aimed directly at those large raw fallbacks did not produce independent retained gains:
  - A targeted whole-block comparison based on large raw fallback blocks did not change the core fixture outputs beyond the already-retained byte-balanced split result.
  - A one-shot raw-split rescue path for large raw fallback blocks likewise produced no measurable movement on the retained core fixtures.
  - Reducing the top-level compressible-run segment cap for non-text runs also failed to help. A `64 KiB` cap regressed decodecorpus to `4,846,915`, and a `32 KiB` cap regressed it much further to about `4.68 MiB`, so those experiments were reverted.
  - Conclusion: the more effective lever was earlier top-level segmentation around incompressible runs, not a late rescue once a partition had already fallen back to raw or a smaller generic cap on non-text compressible runs.
- Rejected two cheap `Best`-matcher CPU knob changes after profiling the retained run-length-1 state:
  - More aggressive adaptive binary no-match probe growth (`BEST_BINARY_NO_MATCH_SEARCH_STRENGTH = 7`): `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-runlen1-step7-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-runlen1-step7-l1.md`. It reduced decodecorpus CPU only slightly (`0.86s` -> `0.83s`) while giving back about `1.2 KiB` on decodecorpus (`4,795,318` -> `4,796,558`) and drifting level-1 JSON CPU into the `0.13s` band.
  - Skipping the `ip+1` window lookahead when the current position already has a minimum-length normal match: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-runlen1-nocurrnextwin-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-runlen1-nocurrnextwin-l1.md`. It preserved output bytes, but decodecorpus CPU regressed slightly (`0.86s` -> `0.87s`), so it was fully reverted.
  - Widening `common_prefix_len()` from `8`-byte chunk compares to `16`-byte chunk compares: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-runlen1-prefix16-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-runlen1-prefix16-l1.md`. It preserved output bytes exactly, but decodecorpus CPU regressed again (`0.86s` -> `0.87s`) and level-1 JSON drifted back to `0.12s`.
  - Adding a same-block fast path for repeat-offset prefix verification likewise preserved bytes but regressed CPU in the full tables (`decodecorpus 0.86s` -> `0.88s`, JSON `0.25s` -> `0.26s`), so it was reverted without keeping report files as a retained comparison point.
  - Cross-entry match-length source walking for verified-prefix scans: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-runlen1-crossentry-fastpath-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-runlen1-crossentry-fastpath-l1.md`. It kept the core bytes exact, but decodecorpus CPU regressed (`0.86s` -> `0.88s`) and level-1 JSON drifted back to `0.12s`, so the generic relative-window scan remains.
  - Skipping the same entry's `oldest` candidate after a nontrivial `newest` hit in `best_window_candidate()`: this was the unfinished parser experiment left in the tree after the crash. `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-restore-check-l4.md` showed the regression signature clearly: decodecorpus worsened from the retained `4,795,318` to `4,813,166`, and JSON shifted from `545,174` to `545,001`, with no CPU win. Reverting that skip restored the retained bytes on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-restore-check2-l4.md`.
  - A more aggressive `Best`-only recent-entry early exit after a long enough match in the newest history block: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-recent-entry-early-exit-l4.md`. It combined the same decodecorpus regression (`4,813,166`) with slower CPU (`0.87s`), so it was fully reverted.
  - A `Best`-only tail search cutoff modeled on C `dfast` cleanup: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-tail-cutoff-l4.md`. It badly regressed JSON compression (`545,174` -> `608,101`) while also worsening decodecorpus to `4,813,455`, so it was fully reverted.
  - Backward-extension pruning based on a forward-match upper bound against the current winner: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-forward-prune-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-forward-prune2-l4.md`. Even the corrected upper bound still changed parser choices through the newest/oldest-candidate control flow and reproduced the same bad decodecorpus regression (`4,813,166`), so the matcher continues to backward-extend every surviving normal candidate.
  - A narrower `Best`-only immediate repeat drain that mirrored C `dfast`'s post-match `rep_offset2` loop more literally than the earlier cached repeat-chain experiment. Instead of caching any zero-literal repeat candidate, the driver drained only `offset_history.second` immediately after each emitted match. `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-second-rep-drain-l4b.md` showed that this still reproduced the same failure shape as the broader cached chain: decodecorpus moved from the retained `4,795,318` to `4,796,474`, and JSON regressed sharply from `545,174` to `602,798`, even though JSON CPU improved. Restoring the old path returned to the retained baseline on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-restore-check3-l4.md`.
  - Added a test-only matcher diagnostics hook and ran it on the retained `Best` parser:
    - `decodecorpus_pack.bin`: `543,807` emitted sequences, with the vast majority of normal-window winners coming from entry distances `0..=3`, and with both newest and oldest candidates in those entries contributing heavily. Current-position and next-position window matches both mattered in those same recent entries.
    - `json_logs_32m.jsonl`: `737,212` emitted sequences, of which `736,617` were current-position repeat-offset winners. Window matches were almost irrelevant there.
    - These were collected with `RUZSTD_MATCHER_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/<fixture> cargo test -q -p ruzstd inspect_best_matcher_from_env -- --ignored --nocapture`.
  - Tested a `Best`-only conditional recent-entry cap based on those diagnostics: once four recent entries had been scanned and any candidate existed, stop instead of walking older entries. `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-recent-limit-after-hit-l4.md` showed it was not safe in the current parser shape: decodecorpus regressed to `4,803,619`, JSON collapsed to `622,654`, and decodecorpus CPU got worse (`0.92s`). So the diagnostics are useful, but they do not justify a simple early-exit cap.
  - Tightened that same idea to only stop after four recent entries when the current normal-window candidate was already a C-shaped long hit (`>= 8` bytes). `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-recent-limit-longhit-l4.md` showed the same failure mode: decodecorpus still regressed (`4,797,548`), JSON still collapsed (`622,654`), and decodecorpus CPU got even worse (`0.95s`). So the threshold was not the issue; the current parser simply cannot replace older-entry competition with a recent-entry cap without changing the chosen parse in bad ways.
  - Tried the same “long hit” idea at a smaller scope: within one history entry, skip `oldest` only when `newest` from that same entry had already installed a long normal match (`>= 8` bytes). `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-same-entry-long-newest-l4b.md` showed that this narrower gate preserved JSON but still regressed decodecorpus (`4,803,189`) and made decodecorpus CPU worse (`0.91s`). So even inside one entry, the older candidate is still doing necessary work often enough that the current representation cannot cheap out there.
  - Tried a three-candidate per-slot layout (`oldest`, `second_newest`, `newest`) to capture more of the recent-history shape that the matcher diagnostics suggested. It materially improved `Best` compression: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-second-newest-layout-l4.md` reached `4,765,769` on `decodecorpus_pack.bin` and `539,951` on `json_logs_32m.jsonl`, which even beat C `-4` on decodecorpus. But it also made `Best` much slower (`decodecorpus 0.99s`, `json 0.29s`) and regressed level-1 CPU simply by enlarging the hot suffix slot from `8` bytes to `12` bytes: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-second-newest-layout-l1.md` kept bytes exact but moved level-1 decodecorpus to `0.19s` and JSON to `0.13s`. So the direct three-candidate layout was rejected.
  - Reworked that idea into a `Best`-only sidecar vector for `second_newest` candidates so non-`Best` levels could keep the compact `oldest/newest` slot layout. This remained functionally correct, but it did not solve the tradeoff. The sidecar version still regressed `Best` CPU (`/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-second-newest-sidecar-l4.md`: `decodecorpus 1.03s`, `json 0.29s`) and the first non-`Best` fast path only partly recovered the level-1 drift (`/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-second-newest-sidecar-fastpath-l1.md`: decodecorpus stayed around `0.18s`, JSON stayed around `0.13s`). Since neither version improved the CPU story enough, the whole second-newest experiment was reverted.
  - Tried a much narrower version of that same idea: keep the compact two-candidate slot layout for everyone, but add a `Best`-only `second_newest` sidecar and probe it only on binary-like blocks for the four most recent history entries. This preserved JSON and targeted exactly the recent-entry normal-window region that the diagnostics say matters on `decodecorpus_pack.bin`.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-recent-second-newest-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-recent-second-newest-l1.md`
    - Key result, compared against the known retained branch state:
      - `decodecorpus_pack.bin`: improved materially from retained `4,792,291` to `4,765,913`
      - `json_logs_32m.jsonl`: stayed flat at `688,174`
      - but CPU was still too expensive:
        - `Best` decodecorpus moved into the `0.91s` band
        - `Best` JSON moved into the `0.24s` band
        - level-1 decodecorpus drifted to `0.20s` and JSON to `0.13s`
    - Conclusion:
      - Narrowing the third-candidate search to recent binary entries is enough to keep the JSON byte profile stable and still recover most of the decodecorpus compression win.
      - It is still not an acceptable retained tradeoff because the CPU cost remains too high and level 1 is not clean. The useful signal is that additional recent candidates are genuinely valuable for binary compression; the missing piece is a cheaper representation/control-flow shape, not a broader search scope.
  - Tightened that second-newest idea again: still `Best` only, but probe the extra recent candidate only for current-position normal-window search, not for the retained `ip+1` lookahead path, and only within the two most recent binary history entries. This was aimed at cutting the doubled candidate checks while preserving the part that should matter most on decodecorpus.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-recent-second-newest-currentonly-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-recent-second-newest-currentonly-l1.md`
    - Key result, compared against the actual current retained source baseline (`/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-current-retained`):
      - `decodecorpus_pack.bin`: `4,792,291 -> 4,774,153`
      - `json_logs_32m.jsonl`: unchanged at `688,174`
      - CPU still failed the bar:
        - `Best` decodecorpus `0.75s -> 0.92s`
        - `Best` JSON `0.19s -> 0.24s`
        - level-1 decodecorpus `0.17s -> 0.19s`
        - level-1 JSON `0.12s -> 0.13s`
    - Conclusion:
      - Restricting the extra candidate to current-position search and only two recent entries is still not enough. The compression gain remains real and even stronger on decodecorpus, but the CPU tax comes from maintaining and consulting the extra-candidate representation at all, not just from probing it too widely.
      - This narrows the design space further: any future revisit needs a cheaper representation than a sidecar second-newest table.
  - Replaced the sidecar with a packed 8-byte triple-candidate slot (`oldest`, `second_newest`, `newest`) so the hot suffix-table entry stayed cache-sized instead of growing beyond the existing 8-byte slot. The goal was to test whether representation size, rather than the extra candidate itself, was the real problem.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-packed-triple-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-packed-triple-l1.md`
    - Key result, compared against the verified current retained baseline `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-current-retained`:
      - `decodecorpus_pack.bin`: `4,792,291 -> 4,765,024`
      - `json_logs_32m.jsonl`: unchanged at `688,174`
      - `Best` CPU improved materially versus the sidecar variants, but still regressed versus the retained baseline:
        - decodecorpus `0.74s -> 0.80s`
        - JSON `0.19s -> 0.21s`
      - level 1 was still not acceptable:
        - decodecorpus `0.17s -> 0.21s`
        - JSON `0.12s -> 0.16s`
    - Conclusion:
      - Packing the third candidate back into an 8-byte slot does reduce the `Best` CPU penalty substantially relative to the sidecar experiments, so representation size was a real part of the cost.
      - It is still not sufficient, because the universal slot-update cost hits level 1 too hard. That points the next structural step toward a genuinely separate `Best` parser/matcher representation rather than further changes to the shared suffix-store layout.
  - Retained a `Best`-only external second-newest sidecar that leaves the shared `SuffixStore` layout untouched and only tracks/probes the extra recent candidate for binary current entries. This follows the same compression direction as the earlier second-newest work, but moves the state completely out of the shared suffix-store hot path so lower levels do not pay the universal slot-layout cost.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-external-sidecar-binaryonly-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-external-sidecar-binaryonly-repeat-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-external-sidecar-binaryonly-l1.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-external-sidecar-binaryonly-repeat-l1.md`
    - Verification:
- `cargo fmt --all --check`
- `cargo clippy -q -p ruzstd --lib -- -D warnings`
- `cargo test -q --workspace`

2026-05-31 follow-up: implemented the public path/file-type compression API shape in `encoding` and routed the CLI through it, but rejected the first two runtime starting-point overrides after benchmarking:
- `ArchiveLike` dense non-text probing across block sizes:
  - real compression win on `build_libruzstd.rlib` (`611,155 -> 600,329`)
  - not retained because repeat CPU moved `0.03s -> 0.04s` and broad-local bytes-above-C stayed `1,005`
- `DictionaryText` wider text no-match probe step:
  - targeted `dict_dictionary.bin` based on archive inspection
  - rejected because it regressed `20,667 -> 21,302` and broad-local bytes-above-C worsened `1,005 -> 1,640`
- Restored runtime behavior to the retained baseline after both rejects. The file-type plumbing remains in place for future experiments without exposing public tuning knobs.
- Follow-up isolation check: renaming `dict_dictionary.bin` to `not_dictionary.bin` and recompressing with the current CLI still produced `21,302` bytes in both cases, so the remaining one-file drift versus the older retained binary is not caused by the new path/file-type API itself. It is a pre-existing source-tree/runtime divergence that still needs separate isolation.
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd best_level_external_fixture_round_trips_with_rust_and_c_decoders_from_env -- --ignored --nocapture`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd-cli best_level_progress_monitor_round_trips_external_fixture_from_env -- --ignored --nocapture`
    - Key repeated result versus the verified retained baseline `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-current-retained`:
      - `decodecorpus_pack.bin`: `4,792,291 -> 4,692,418`
      - `json_logs_32m.jsonl`: unchanged at `688,174`
      - `repeated_text_32m.txt`: unchanged at `3,127`
      - `xorshift_32m.bin`: unchanged at `33,555,210`
      - `Best` CPU moved from `0.75s` to `0.88s` on decodecorpus, while JSON stayed in the retained `0.19s` band on the repeat run
      - level 1 stayed byte-identical, with decodecorpus moving from `0.17s` to `0.19s` and JSON staying in the `0.11s`/`0.12s` band
    - Interpretation:
      - This is the first second-newest-style matcher path that preserves the JSON byte profile, keeps the shared suffix-store representation intact, passes the release external round-trip checks, and still delivers a very large decodecorpus compression win.
      - The remaining downside is a modest level-1 decodecorpus CPU drift and a `Best` decodecorpus CPU increase, but the tradeoff is now substantially better than any prior second-newest variant. This is a plausible retained `Best` compression step and a better foundation for subsequent CPU-focused work than the sidecar or packed-slot prototypes.
  - Tried a CPU-focused gate on top of that retained sidecar: skip the extra second-newest probe whenever the current same-entry normal candidate was already at least `8` bytes long. Result: reject.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-external-sidecar-threshold8-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-external-sidecar-threshold8-l1.md`
    - Key result relative to the retained external-sidecar baseline:
      - `decodecorpus_pack.bin`: `4,692,418 -> 4,695,669`
      - `json_logs_32m.jsonl`: unchanged at `688,174`
      - `Best` decodecorpus CPU improved only marginally (`0.88s -> 0.87s`)
      - level 1 showed no meaningful improvement (`decodecorpus 0.19s`, `json 0.13s`)
    - Conclusion:
      - The extra second-newest candidate is still paying for itself even when an 8-byte same-entry candidate already exists.
      - This local gating cut gives back too much binary compression for too little CPU benefit, so the retained sidecar baseline stays unchanged.
  - Restore checks after reverting the second-newest experiment returned the branch to the retained baseline byte profile: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-restore-check7-l4.md` restored `decodecorpus_pack.bin` to `4,795,318` and `json_logs_32m.jsonl` to `545,174`; `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-restore-check7-l1.md` restored level 1 to `5,324,267` / `690,084` with CPU back in the normal `0.17s` / `0.12s` band.
  - Tried a text-only structural cap for the window search after a repeat-offset candidate had already been found: on text-like blocks, search only the two most recent history entries instead of the full text window when a repeat candidate exists. This was aimed directly at the JSON diagnostics, where nearly all emitted matches were repeat-offset wins.
    - Benchmark report: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-text-repeat-window-limit-l4.md`
    - Result: it was a clear reject. `decodecorpus_pack.bin` regressed slightly on bytes (`4,795,318` -> `4,795,354`) and got slower (`0.85s` -> `0.90s` in that run), while `json_logs_32m.jsonl` stayed byte-identical at `545,174` with only noise-level CPU movement (`0.26s` -> `0.25s`). So the cap did not actually buy a useful reduction in text cost, and it disturbed the non-text path enough to make decodecorpus worse.
    - Conclusion: even though window winners are rare on JSON, the cost of threading a conditional search limit through the current parser loop outweighs the saved scans. This is another signal that the remaining CPU gap needs a deeper parser representation change, not just a dynamic search cutoff.
  - Tried a behavior-preserving fast path in `slice_at_relative()` that checked the three most recent previous entries before falling back to the generic reverse scan. This targeted the hot relative-window lookups used by prefix checks, repeat verification, and cross-entry match extension.
    - Benchmark report: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-relative-slice-fastpath-l4.md`
    - Result: it preserved the retained bytes exactly (`decodecorpus_pack.bin` stayed `4,795,318`, `json_logs_32m.jsonl` stayed `545,174`), but decodecorpus got slower anyway (`0.85s` -> `0.89s` in that run) and JSON did not improve (`0.26s` stayed `0.26s`). The extra branchy fast-path logic cost more than the saved generic loop.
    - Conclusion: recent-entry dominance is real for parser decisions, but it does not automatically imply that hand-specializing the relative-slice resolver helps. This path is not where the remaining CPU gap should be attacked.
  - Tried a deeper but still behavior-preserving rewrite of the cross-entry match-length loop: resolve the initial previous-window source position once, then only rescan the window mapping when the source crossed an entry boundary, instead of calling the generic relative-slice lookup on every iteration of `match_len_at_offset_with_prefix()`.
    - Benchmark report: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-relative-position-loop-l4.md`
    - Result: bytes stayed exact (`decodecorpus_pack.bin` remained `4,795,318`, `json_logs_32m.jsonl` remained `545,174`), but decodecorpus again got slower (`0.85s` -> `0.87s` in that run) and JSON stayed in the same `0.25s/0.26s` band. So the extra state tracking in the loop still cost more than it saved.
    - Conclusion: the expensive part is not simply “too many generic relative-slice lookups” in isolation. This points even more strongly toward a parser representation change rather than incremental reshaping of the current helper stack.
  - Tried a C-shaped short collision-tag sidecar for the suffix store, modeled after zstd fast/dfast's tagged hash-table path. The idea was to reject most hash collisions cheaply before touching candidate bytes, while keeping the `Candidates` slot itself compact.
    - Benchmark report: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-tag-sidecar-l4.md`
    - Result: it was not behavior-preserving in practice, which is enough to reject it. `decodecorpus_pack.bin` moved from the retained `4,795,318` to `4,796,835` and decodecorpus CPU worsened from `0.85s` to `0.95s`; `json_logs_32m.jsonl` stayed at `545,174` bytes but CPU also worsened (`0.26s` -> `0.28s`). Since a supposedly safe collision filter changed bytes at all, the experiment was fully reverted rather than tuned further on the retained branch.
    - Conclusion: even the more C-like tagged-hash representation is not a drop-in win inside the current matcher shape. Any future revisit would need much tighter investigation of why the filter changed parse output before it could be considered again.
  - Tried a more faithful fast/dfast-style 4-byte precheck before full match counting. C fast/dfast tests `MEM_read32()` first and only counts further after that, whereas the current Rust matcher verified 5 bytes before entering the full count. The experiment switched both repeat and window prechecks to 4-byte verification while keeping final acceptance at 5 bytes.
    - Benchmark report: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-precheck4-l4.md`
    - Result: it was also a reject. `decodecorpus_pack.bin` regressed from `4,795,318` to `4,795,870`, decodecorpus CPU worsened from `0.85s` to `0.87s`, and JSON stayed flat in both bytes and CPU. The looser precheck admitted more false-positive candidates into full counting without producing a compensating speedup.
    - Conclusion: matching C's 4-byte precheck in isolation is not enough. In C it is paired with a much tighter parser pipeline; copied into the current Rust loop, it only creates more work.
  - Retained a behavior-preserving precomputed slot-key fast path for window candidate lookup. The hot `best_window_candidate()` loop was recomputing the same five-byte hash-to-slot mapping for every history entry even though all live suffix stores on a given level share the same table size. The matcher now computes the slot key once from the current block's suffix-store layout and reuses it across same-sized history entries, falling back to the old per-entry path only if a store with a different `len_log` is encountered.
    - Retained reports: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-precomputed-slotkey-l4.md` and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-precomputed-slotkey-l1.md`
    - Compared to the restored retained baseline `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-restore-check7-l4.md`:
      - `decodecorpus_pack.bin`: unchanged at `4,795,318`, CPU improved from `0.85s` to `0.84s`
      - `json_logs_32m.jsonl`: unchanged at `545,174`, CPU improved from `0.26s` to `0.23s`
      - `repeated_text_32m.txt`: unchanged at `2,874`, CPU stayed in the same `0.02s` band
      - `xorshift_32m.bin`: unchanged at `33,555,210`, CPU stayed in the same `0.05s` band
    - Level 1 stayed byte-identical and CPU-stable on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-precomputed-slotkey-l1.md`: decodecorpus `5,324,267` at `0.17s`, JSON `690,084` at `0.12s`, repeated text `2,874`, xorshift `33,555,210`
    - Added focused coverage that precomputed slot keys produce the same lookup result as the existing value-based path and that the fast path is skipped safely when suffix stores have different capacities.
  - Retained a second behavior-preserving fast path on top of that slot-key work: track whether all live suffix stores in the current matcher window share one `len_log`, and when they do, skip the per-entry capacity branch entirely inside `best_window_candidate()`. The matcher now updates `uniform_suffix_len_log` conservatively as blocks are committed; if any mixed capacity is ever observed it falls back to the older precomputed-slot-key path until reset.
    - Retained reports: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-uniform-slotkey-l4.md`, `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-uniform-slotkey-repeat-l4.md`, and `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-uniform-slotkey-l1.md`
    - Compared to the previous retained precomputed-slot-key baseline:
      - `decodecorpus_pack.bin`: unchanged at `4,795,318`, CPU improved from `0.84s` to `0.79s` on both level-4 runs
      - `json_logs_32m.jsonl`: unchanged at `545,174`, CPU stayed in the `0.23s` to `0.24s` noise band across the two level-4 runs
      - `repeated_text_32m.txt`: unchanged at `2,874`, CPU stayed in the same `0.02s` to `0.03s` band
      - `xorshift_32m.bin`: unchanged at `33,555,210`, CPU stayed at `0.05s`
    - Level 1 stayed byte-identical on `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-uniform-slotkey-l1.md`: decodecorpus `5,324,267`, JSON `690,084`, repeated text `2,874`, xorshift `33,555,210`; decodecorpus stayed at `0.17s`, while JSON remained in the same `0.11s`/`0.12s` band.
    - Added focused coverage that the matcher tracks uniform suffix-store capacity across same-sized blocks and disables the branch-elision path when different capacities are committed into one window. Also fixed the older mixed-capacity slot-key test to match the current `SlotKey` representation, which now stores only the precomputed slot index.
  - Added external-fixture `Best` regression coverage after a confusing CLI verification detour:
    - `ruzstd/src/encoding/levels/fastest_tests.rs` now has an ignored env-driven test, `best_level_external_fixture_round_trips_with_rust_and_c_decoders_from_env`, which validates a local fixture from `RUZSTD_BEST_FIXTURE` through three retained `Best` paths:
      - `compress_to_vec(data.as_slice(), CompressionLevel::Best)`
      - `compress(BufReader<File>, &mut Vec<u8>, CompressionLevel::Best)`
      - `compress(BufReader<File>, &mut File, CompressionLevel::Best)`
    - `cli/src/progress.rs` now has an ignored env-driven test, `best_level_progress_monitor_round_trips_external_fixture_from_env`, which validates the CLI-side `ProgressMonitor::without_progress(BufReader<File>, len)` reader path at `Best`, including an external `/usr/bin/zstd -d -c` decode of a temp output file.
    - These checks were run successfully against `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin` in release mode:
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd best_level_external_fixture_round_trips_with_rust_and_c_decoders_from_env -- --ignored --nocapture`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd-cli best_level_progress_monitor_round_trips_external_fixture_from_env -- --ignored --nocapture`
    - While adding that coverage, a test-only matcher diagnostics bug was found and fixed: large external fixtures can emit normal-window winners with `entry_distance >= BEST_WINDOW_BLOCKS`, so diagnostics now saturate those counts instead of panicking on out-of-bounds indexes.
    - The direct `ruzstd-cli` shell repro that initially looked like a truncated frame turned out to be a false lead once rerun against the rebuilt binary and rechecked via subprocess capture. The retained conclusion is that the current release library and the CLI-side `ProgressMonitor` path both round-trip `decodecorpus_pack.bin`; the missing piece was external-fixture coverage in the test suite, not a confirmed retained corruption bug.
  - Retried a bounded release-only matcher cleanup: compile `WindowCandidateMeta` / `WindowCandidateKind` and the related `best_window_candidate()` metadata plumbing only under `#[cfg(test)]`, so the release hot path does not carry entry-distance bookkeeping that exists purely for diagnostics.
    - Validation passed before benchmarking:
      - `cargo test -q -p ruzstd match_generator -- --nocapture`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd best_level_external_fixture_round_trips_with_rust_and_c_decoders_from_env -- --ignored --nocapture`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd-cli best_level_progress_monitor_round_trips_external_fixture_from_env -- --ignored --nocapture`
    - A direct before/after benchmark was then run against a validated copied baseline CLI binary because the stock harness behaved inconsistently with stale `/tmp` state:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-testmeta-elision-timed-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-testmeta-elision-timed-l1.md`
    - Result: reject. Bytes stayed identical everywhere, level 1 stayed flat, JSON stayed flat, but level-4 decodecorpus regressed slightly on CPU (`0.82s` -> `0.84s`). That is not enough to keep extra conditional compilation and signature complexity in the hot matcher path.
  - Tried a more structural `Best`-level adjacent-position parser experiment: cache the already-computed `ip+1` repeat/window lookahead from one miss iteration and reuse it on the next single-byte step, instead of rescanning prior history from scratch. This was meant to move the current loop one step closer to C fast/dfast's adjacent-position pipeline without changing the match model.
    - The idea failed before benchmarking. Focused matcher tests immediately exposed parse changes:
      - `repeat_offset_probe_finds_match_without_suffix_index` regressed from the expected `literals: b\"x\", offset: 10, match_len: 9` to `literals: b\"xM\", offset: 10, match_len: 8`
      - `matches` regressed by missing a retained tail match and emitting trailing literals instead
    - Those failures showed that the cached `ip+1` state was not a safe substitute for recomputing the current-position parser state on the next iteration. Even when the previous lookahead had already searched old history, the interaction with same-block suffix insertion and candidate ordering was still strong enough to change emitted sequences.
    - The experiment was fully reverted without benchmarking. Keep the result as evidence that a true adjacent-position rewrite cannot be staged in by simply memoizing the current `ip+1` helper outputs; it needs a more faithful parser/control-flow redesign.
  - Tried a smaller `Best`-only reuse on the miss path: when the parser had already probed repeat offsets at `ip+1` for the retained next-position repeat lookahead, reuse that result inside `can_skip_next_probe()` instead of rescanning `repeat_offset_can_match_at(ip+1, ...)`.
    - First version reused the existence of a full `ip+1` repeat candidate. That was not equivalent to `repeat_offset_can_match_at()` and gave back `92` bytes on `decodecorpus_pack.bin`.
    - Second version tightened the reuse to the prefix-match predicate, matching the skip guard more closely. It preserved all focused matcher tests and the release external-fixture checks, but the timed manual before/after benchmark still showed the same `92`-byte decodecorpus regression:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-repeatprobe-prefixreuse-manual-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-repeatprobe-prefixreuse-manual-l1.md`
    - Result: reject. Level-4 decodecorpus CPU improved slightly (`0.80s` -> `0.79s`) and JSON improved slightly (`0.24s` -> `0.23s`), but giving back bytes on the main retained `Best` compression fixture is not acceptable for this branch.
    - Conclusion: even this apparently local reuse is entangled with the parser's no-match stepping semantics. The repeat probe result at `ip+1` cannot be substituted into the skip guard without changing the eventual parse on decodecorpus, so the current miss-step logic stays as-is.
  - Tried a current-entry fast path in `best_window_candidate()`: handle the current window entry outside the generic reverse scan and then iterate only older entries. This preserved focused matcher tests and the release external-fixture checks, but it was not a retained win.
    - The timed before/after run against the copied pre-change binary produced identical decodecorpus bytes and only noise-band CPU movement:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-current-entry-fastpath-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-current-entry-fastpath-l1.md`
    - More importantly, that run exposed that the copied pre-change binary was already not the last retained `Best` state, so the fast path itself was not trustworthy enough to keep. The change was fully reverted.
  - Refreshed the benchmark view against the rebuilt source tree and found that the workspace is no longer on the last retained `Best` baseline. The current source binary is valid and byte-verified, but its level-4 tradeoff has shifted materially:
    - `decodecorpus_pack.bin`: `4,792,424` bytes at `0.79s` (better than the retained `4,795,318 @ 0.79s`)
    - `json_logs_32m.jsonl`: `688,165` bytes at `0.20s` (much worse than the retained `545,174 @ 0.23s`)
    - `repeated_text_32m.txt`: `3,127` bytes at `0.02s` (worse than the retained `2,874`)
    - `xorshift_32m.bin`: unchanged bytes, slightly faster CPU
    - Clean summary report: `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-source-current-summary.md`
    - Source reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-currentonly-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-source-current-l1.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-rs-benchmark-best-uniform-slotkey-repeat-l4.md`
    - The level-1 source build remains in the expected band. The current problem is specifically the rebuilt source tree's `Best` behavior, which now trades a small decodecorpus gain for a large JSON regression. That needs to be reconciled before more `Best` experiments are trusted.
  - Tried two more C-guided `Best` parser/block-budget reductions and rejected both:
    - Skip the recursive `Best` partitioner entirely for text-like blocks. Result: reject. It regressed both `decodecorpus_pack.bin` and `json_logs_32m.jsonl` for only small CPU movement.
      - Reports:
        - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-nosplit-l4.md`
        - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-nosplit-l1.md`
    - Shrink the active `Best` text matcher window from the current 16-block budget to an 8-block budget. Result: reject. It preserved decodecorpus bytes but made JSON both larger and slightly slower.
      - Reports:
        - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-window8-l4.md`
        - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-window8-l1.md`
  - Retained a `Best` literal-Huffman CPU reduction in `ruzstd/src/encoding/blocks/compressed.rs`:
    - `CompressionLevel::Best` no longer forces exhaustive `build_smallest_from_counts()` search for every new Huffman table.
    - It now uses the same `SmallLiteralSections` gate as the lower levels, so exhaustive table minimization is only attempted for sequence-empty or very small literal sections.
    - This follows the current C-guided direction better than the previous unconditional `AllNewTables` policy for `Best`: the hot cost was visible in `perf` (`length_limited_code_lengths` and related heap work around 6% on decodecorpus), while the compression benefit on large blocks was weak.
    - Retained reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-huffsmall-final-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-huffsmall-final-l1.md`
    - Result versus the verified pre-change `Best` baseline (`/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-before-best-text-nosplit`):
      - `decodecorpus_pack.bin`: `4,792,424 -> 4,794,772`, CPU `0.80s -> 0.71s`
      - `json_logs_32m.jsonl`: `688,165 -> 688,174`, CPU unchanged at `0.20s`
      - `repeated_text_32m.txt`: unchanged at `3,127`
      - `xorshift_32m.bin`: unchanged at `33,555,210`
    - Level 1 stayed byte-identical:
      - `decodecorpus 5,324,267`
      - `json 690,084`
      - `repeated 2,874`
      - `xorshift 33,555,210`
    - Validation after retaining this change:
      - `cargo fmt --all --check`
      - `cargo clippy -q -p ruzstd --lib -- -D warnings`
      - `cargo test -q --workspace`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd best_level_external_fixture_round_trips_with_rust_and_c_decoders_from_env -- --ignored --nocapture`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd-cli best_level_progress_monitor_round_trips_external_fixture_from_env -- --ignored --nocapture`
  - Refreshed `Best` matcher diagnostics on the retained Huffman-reduced state:
    - `decodecorpus_pack.bin` still gets almost all normal-window winners from entry distances `0..=3`, and `oldest` wins more often than `newest` at every one of those distances.
    - `json_logs_32m.jsonl` remains overwhelmingly repeat-offset driven (`repeat_current` and `repeat_next_position` dominate; normal-window winners are effectively absent).
    - Those diagnostics were gathered with:
      - `env RUZSTD_MATCHER_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test -q -p ruzstd inspect_best_matcher_from_env -- --ignored --nocapture`
      - `env RUZSTD_MATCHER_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/json_logs_32m.jsonl cargo test -q -p ruzstd inspect_best_matcher_from_env -- --ignored --nocapture`
  - Tried a text-path short-circuit in `next_sequence()`: on text blocks, if a repeat-offset candidate already existed, skip normal window search. Result: reject.
    - It preserved most bytes but made CPU slightly worse at both levels.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-shortcircuit-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-shortcircuit-l1.md`
  - Retained a `Best`-only window-probe order change in `ruzstd/src/encoding/match_generator.rs`:
    - Added an explicit `prefer_oldest_first_window_probe` flag, enabled only for `CompressionLevel::Best`.
    - Inside each suffix-store slot, `Best` now probes the `oldest` candidate before `newest`, matching the current decodecorpus winner distribution more closely.
    - Retained reports against the actual retained pre-change baseline (`/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-before-text-repeat-shortcircuit`):
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-oldest-first-retainedbase-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-oldest-first-retainedbase-l1.md`
    - Result versus the retained Huffman-reduced baseline:
      - `decodecorpus_pack.bin`: `4,794,772 -> 4,794,724`, CPU `0.73s -> 0.71s`
      - `json_logs_32m.jsonl`: unchanged at `688,174`, CPU `0.20s -> 0.19s`
      - `repeated_text_32m.txt`: unchanged at `3,127`
      - `xorshift_32m.bin`: unchanged bytes, CPU `0.05s -> 0.04s`
    - Level 1 stayed byte-identical:
      - `decodecorpus 5,324,267`
      - `json 690,084`
      - `repeated 2,874`
      - `xorshift 33,555,210`
    - Validation after retaining this change:
      - `cargo fmt --all --check`
      - `cargo clippy -q -p ruzstd --lib -- -D warnings`
      - `cargo test -q --workspace`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd best_level_external_fixture_round_trips_with_rust_and_c_decoders_from_env -- --ignored --nocapture`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd-cli best_level_progress_monitor_round_trips_external_fixture_from_env -- --ignored --nocapture`
  - Tried a narrower C-style sparse-index cut: when `Best` uses complementary end insertion, drop the older `start + 2` sparse suffix and keep only `start`, `end - 1`, and `end - 2`. Result: reject.
    - It preserved bytes on most fixtures but made JSON slower and did not improve decodecorpus CPU.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-sparse-cstyle-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-sparse-cstyle-l1.md`
  - Retained a bounded adjacent-position sparse-index detail in `ruzstd/src/encoding/match_generator.rs`:
    - when a next-position candidate wins and the match is long enough to use sparse indexing, explicitly preserve `start_idx` before the existing sparse end-of-match inserts
    - this is the nearest safe-Rust analogue so far to C `dfast`'s `hash1` write before resuming after an `ip+1` match decision
    - added focused invariant coverage:
      - `sparse_next_position_match_preserves_start_index`
    - Retained reports against the actual retained oldest-first baseline (`/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-before-text-repeat-shortcircuit`):
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-nextpos-ip1-sparse-retainedbase-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-nextpos-ip1-sparse-retainedbase-l1.md`
    - Result versus the retained oldest-first state:
      - `decodecorpus_pack.bin`: `4,794,724 -> 4,794,719`, CPU `0.72s -> 0.71s`
      - `json_logs_32m.jsonl`: unchanged at `688,174`, CPU `0.20s -> 0.19s`
      - `repeated_text_32m.txt`: unchanged at `3,127`
      - `xorshift_32m.bin`: unchanged bytes
    - Level 1 guardrail:
      - benchmark table showed byte-identical output with decodecorpus in the `0.17/0.18s` band and JSON slightly faster
      - a dedicated 5-run check confirmed the medians stayed on the retained baseline:
        - `decodecorpus_pack.bin`: `0.17s` median both before and after
        - `json_logs_32m.jsonl`: `0.11s` median both before and after
    - Validation after retaining this change:
      - `cargo fmt --all --check`
      - `cargo clippy -q -p ruzstd --lib -- -D warnings`
      - `cargo test -q --workspace`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd best_level_external_fixture_round_trips_with_rust_and_c_decoders_from_env -- --ignored --nocapture`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd-cli best_level_progress_monitor_round_trips_external_fixture_from_env -- --ignored --nocapture`
  - Tried a stricter C-shaped per-entry matcher cut: after probing `oldest` first, if that candidate improved the current best, skip probing `newest` for the same suffix-store entry. Result: reject.
    - This was too aggressive. It badly regressed `decodecorpus_pack.bin` and also worsened CPU at both levels.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-single-oldest-entry-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-single-oldest-entry-l1.md`
    - Key result:
      - `decodecorpus_pack.bin`: `4,794,724 -> 4,897,672`, CPU `0.72s -> 0.81s`
    - Conclusion: even with the retained oldest-first ordering, the secondary candidate inside the same entry is still important enough that collapsing to one winner per entry does not match the current Rust parser geometry safely.
  - Retained a bounded adjacent-position detail in `ruzstd/src/encoding/match_generator.rs`:
    - when a next-position candidate wins and the match is long enough to use sparse indexing, preserve `start_idx` explicitly before the existing sparse match-end inserts
    - this is the closest safe Rust analogue so far to C `dfast`'s `hash1` write before resuming after an `ip+1` decision
    - added focused invariant coverage:
      - `sparse_next_position_match_preserves_start_index`
    - Retained reports against the actual retained oldest-first baseline (`/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-before-text-repeat-shortcircuit`):
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-nextpos-ip1-sparse-retainedbase-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-nextpos-ip1-sparse-retainedbase-l1.md`
    - Result versus the retained oldest-first state:
      - `decodecorpus_pack.bin`: `4,794,724 -> 4,794,719`, CPU `0.72s -> 0.71s`
      - `json_logs_32m.jsonl`: unchanged at `688,174`, CPU `0.20s -> 0.19s`
      - `repeated_text_32m.txt`: unchanged at `3,127`
      - `xorshift_32m.bin`: unchanged bytes
    - Level 1 guardrail:
      - benchmark table showed byte-identical output with `decodecorpus` in the `0.17/0.18s` band and JSON slightly faster
      - a dedicated 5-run check confirmed the medians stayed on the retained baseline:
        - `decodecorpus_pack.bin`: `0.17s` median both before and after
        - `json_logs_32m.jsonl`: `0.11s` median both before and after
    - Validation after retaining this change:
      - `cargo fmt --all --check`
      - `cargo clippy -q -p ruzstd --lib -- -D warnings`
      - `cargo test -q --workspace`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd best_level_external_fixture_round_trips_with_rust_and_c_decoders_from_env -- --ignored --nocapture`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd-cli best_level_progress_monitor_round_trips_external_fixture_from_env -- --ignored --nocapture`
  - Re-read the actual C `zstd_fast.c` / `ZSTD_dfast` pipeline comment and the surrounding loop shape. The meaningful architectural gap is now clearer than the local experiments:
    - C explicitly pipelines adjacent positions (`ip0`, `ip1`, `ip2`, `ip3`) so hash, table lookup, repcode check, and compare are interleaved instead of fully sequenced per position.
    - The current Rust matcher still performs a full repeat probe set plus a full per-entry window search for one position at a time, then repeats that control flow for the next byte.
    - C also writes a flat hash table entry for the current position as part of the pipeline and resumes from a known adjacent position after each miss or match, whereas the current Rust code re-enters the whole helper stack from scratch every iteration.
  - Extended the test-only matcher diagnostics so repeat winners are now split by repcode slot rather than only “current” vs “next-position”.
    - `json_logs_32m.jsonl` on the retained baseline:
      - `repeat_current`: first `3,527`, second `118,055`, third `491,491`
      - `repeat_current_zero_literals`: first `0`, second `116,715`, third `488,888`
      - `repeat_current_with_literals`: first `3,527`, second `1,340`, third `2,603`
      - `repeat_next_position`: first `144,907`, second `107`, third `54,426`
      - `repeat_next_position_zero_literals`: all `0`
      - `repeat_next_position_with_literals`: first `144,907`, second `107`, third `54,426`
      - normal-window wins remain effectively zero
    - `decodecorpus_pack.bin` on the retained baseline:
      - `repeat_current`: first `9,507`, second `13,622`, third `11,909`
      - `repeat_current_zero_literals`: first `7,402`, second `12,222`, third `10,886`
      - `repeat_current_with_literals`: first `2,105`, second `1,400`, third `1,023`
      - `repeat_next_position`: first `15,024`, second `11,538`, third `10,167`
      - `repeat_next_position_zero_literals`: all `0`
      - `repeat_next_position_with_literals`: first `15,024`, second `11,538`, third `10,167`
      - normal-window wins are still concentrated in entry distances `0..=3`
    - Conclusion: a naive `dfast`-style “only probe newest/current rep first” cut is not safe in the current Rust parser shape. On JSON, the dominant current-repeat winner is actually the third repcode and it is overwhelmingly zero-literal chaining, while next-position repeats are all literal-carrying and dominated by the first repcode.
    - Inspection commands:
      - `env RUZSTD_MATCHER_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/json_logs_32m.jsonl cargo test -q -p ruzstd inspect_best_matcher_from_env -- --ignored --nocapture`
      - `env RUZSTD_MATCHER_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test -q -p ruzstd inspect_best_matcher_from_env -- --ignored --nocapture`
  - Tried a `Best`-only text repeat-priority shortcut based on those diagnostics: probe current repeat candidates in second/third/first order for text blocks and stop once the repeat is already long enough to skip window search. Result: reject.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-priority-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-priority-l1.md`
    - Key result:
      - `decodecorpus_pack.bin`: `4,794,719 -> 4,794,741`, CPU `0.74s -> 0.75s`
      - `json_logs_32m.jsonl`: `688,174 -> 699,878`, CPU `0.19s -> 0.21s`
      - level 1 stayed byte-identical and CPU-flat
    - Conclusion: even with the new repeat-winner split, collapsing the current text repeat probe order and adding an early exit changes the parse badly on JSON. The useful part is the diagnostics, not the shortcut.
  - Tried an even narrower `Best`-only zero-literal repeat cut: keep the existing repcode order, but once a zero-literal current repeat is already long enough to skip window search, stop probing the remaining repcodes. Result: reject.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-zero-literal-repeat-cut-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-zero-literal-repeat-cut-l1.md`
    - Key result:
      - `decodecorpus_pack.bin`: `4,794,719 -> 4,795,119`, CPU `0.73s -> 0.74s`
      - `json_logs_32m.jsonl`: `688,174 -> 699,878`, CPU `0.20s -> 0.22s`
      - level 1 stayed byte-identical but JSON CPU drifted `0.12s -> 0.13s`
    - Conclusion: even this much tighter zero-literal cut is not safe. The current parser still needs the full repcode competition, so the diagnostics are useful evidence against more local repeat-loop cuts rather than support for one.
  - Tried a narrower adjacent-position cut aimed only at the `ip+1` window lookahead: for `Best`, cap the secondary next-position window search to the four most recent history entries, since the retained diagnostics show all emitted next-position window winners come from entry distances `0..=3`. Result: reject.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-nextpos-window-cap4-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-nextpos-window-cap4-l1.md`
    - Key result:
      - `decodecorpus_pack.bin`: `4,794,719 -> 4,803,252`, CPU `0.72s -> 0.71s`
      - `json_logs_32m.jsonl`: unchanged at `688,174`, CPU `0.20s -> 0.21s`
      - level 1 stayed byte-identical but decodecorpus and JSON CPU both drifted upward (`0.18s -> 0.19s`, `0.12s -> 0.13s`)
    - Conclusion: even though emitted next-position window winners never came from farther entries, older-entry competition still influences the parse enough that capping only the `ip+1` search is not safe in the current matcher shape. This reinforces the same conclusion as the repeat-loop experiments: the next meaningful step has to be a deeper adjacent-position parser rewrite, not another local cap.
  - Tried a more structural but still bounded adjacent-position rewrite: when `Best` had no repeat candidate yet and `ip+1` lookahead was eligible, scan current-position and next-position window candidates together in one conditional pass over the history window instead of doing the current scan and then a separate `ip+1` scan. Result: reject.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-conditional-integrated-nextscan-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-conditional-integrated-nextscan-l1.md`
    - Key result:
      - `decodecorpus_pack.bin`: unchanged at `4,794,719`, but CPU regressed `0.72s -> 0.80s`
      - `json_logs_32m.jsonl`: unchanged at `688,174`, CPU unchanged at `0.19s`
      - level 1 stayed byte-identical but decodecorpus CPU drifted `0.17s -> 0.18s`
    - Conclusion: even the conditional integrated scan carries enough extra control-flow and bookkeeping overhead to lose in the current matcher representation. This is stronger evidence that the remaining gap is not fixable by rearranging the existing helper stack; it needs a different parser representation or control-flow shape closer to C fast/dfast from the ground up.
  - Tried a Best-only flat recent-entry window fast path inside the retained `best_window_candidate()` search: scan the four recent history entries that dominate actual emitted wins by direct index first, then fall back to the generic reverse iterator for older entries. Result: reject.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-recent-flat-window-fastpath-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-recent-flat-window-fastpath-l1.md`
    - Key result:
      - `decodecorpus_pack.bin`: unchanged at `4,794,719`, but CPU regressed `0.73s -> 0.77s`
      - `json_logs_32m.jsonl`: unchanged at `688,174`, CPU unchanged at `0.20s`
      - level 1 stayed byte-identical but decodecorpus CPU drifted `0.18s -> 0.19s`
    - Conclusion: even a flat recent-entry fast path that preserves the exact parse still adds enough branching and indexing overhead to lose on CPU. This reinforces the same conclusion as the conditional integrated scan: the remaining gap is not in the surface shape of `best_window_candidate()`, but in the deeper matcher representation and parser loop.
  - Tried a more aggressive C-shaped post-match indexing change just for `Best` zero-literal matches: instead of dense indexing below the normal `128`-byte threshold, always use the sparse C-fast/dfast-style inserts (`start`, `start+2`, `end-1`, `end-2`) when a match carries no literals. Result: not retained on `Best`, but worth recording as a higher-compression candidate.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-zero-literal-sparse-all-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-zero-literal-sparse-all-l1.md`
    - Key result:
      - `decodecorpus_pack.bin`: `4,794,719 -> 4,775,704`, CPU `0.71s -> 0.77s`
      - `json_logs_32m.jsonl`: unchanged at `688,174`, CPU `0.19s -> 0.20s`
      - `repeated_text_32m.txt`: unchanged bytes, CPU `0.04s -> 0.03s`
      - level 1 stayed byte-identical but CPU drifted upward in the benchmark run (`decodecorpus 0.17s -> 0.18s`, `json 0.12s -> 0.13s`)
    - Interpretation:
      - This is the first recent experiment that materially improved compression rather than just reshaping CPU. On decodecorpus it even beat C `zstd -4` (`4,789,813`) by about `14 KiB`.
      - It is not acceptable as the retained `Best` behavior because CPU moved the wrong way on the main binary fixture and the level-1 guardrail drifted.
      - It should be kept in mind as a future higher-compression-level candidate if we add a level above the current `Best`, or if later CPU work can offset its cost.
  - Tried a narrower version of that same idea: lower the dense-to-sparse threshold only for `Best` zero-literal matches, from `128` down to `64`, instead of forcing sparse indexing for every zero-literal match. Result: reject.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-zero-literal-sparse64-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-zero-literal-sparse64-l1.md`
    - Key result:
      - `decodecorpus_pack.bin`: `4,794,719 -> 4,795,455`
      - `json_logs_32m.jsonl`: unchanged at `688,174`
      - CPU stayed flat in the benchmark run rather than recovering the lost time (`decodecorpus 0.77s -> 0.77s`, JSON `0.20s -> 0.20s`)
    - Conclusion: the compression gain from “all zero-literal sparse” does not survive a partial rollout at a `64`-byte threshold, and the CPU cost does not come back either. So the effect is not a smooth threshold tradeoff; it looks more like a specific parse shift that only appears when zero-literal matches are made sparse aggressively.
  - Kept a narrower `Best`-only follow-up from that direction: when a zero-literal repeat-offset match is emitted, preserve the sparse C-fast/dfast-style inserts (`start`, `start+2`, `end-1`, `end-2`) regardless of match length, while leaving non-repeat zero-literal matches on the existing retained path.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-zero-literal-repeat-sparse-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-zero-literal-repeat-sparse-repeat-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-zero-literal-repeat-sparse-l1.md`
    - Verification:
      - `cargo fmt --all --check`
      - `cargo clippy -q -p ruzstd --lib -- -D warnings`
      - `cargo test -q -p ruzstd match_generator -- --nocapture`
      - `cargo test -q --workspace`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd best_level_external_fixture_round_trips_with_rust_and_c_decoders_from_env -- --ignored --nocapture`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd-cli best_level_progress_monitor_round_trips_external_fixture_from_env -- --ignored --nocapture`
    - Key result:
      - `decodecorpus_pack.bin`: `4,794,719 -> 4,792,291`, with repeat CPU `0.73s -> 0.75s`
      - `json_logs_32m.jsonl`: unchanged at `688,174`, with repeat CPU `0.20s -> 0.19s`
      - `repeated_text_32m.txt`: unchanged at `3,127`
      - `xorshift_32m.bin`: unchanged at `33,555,210`
      - level 1 stayed byte-identical and CPU-flat in the benchmark run (`decodecorpus 0.18s -> 0.18s`, `json 0.12s -> 0.12s`)
    - Interpretation:
      - This is a modest but real retained `Best` compression win on the main binary fixture, without the level-1 drift that disqualified the broader zero-literal-sparse experiments.
      - The CPU cost is localized to `Best` on `decodecorpus` and stays in a narrow band; the release external-fixture round-trip checks through both the library and CLI paths passed, so this emitted-bitstream change is safe to keep.
  - Tried a narrower zero-literal non-repeat follow-up: keep the retained repeat-only sparse path, but lower the dense-index limit for zero-literal non-repeat `Best` matches from `128` to `32` so only medium and longer current-position window matches use sparse tail inserts. Result: reject.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-zero-literal-nonrepeat32-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-zero-literal-nonrepeat32-l1.md`
    - Key result:
      - `decodecorpus_pack.bin`: `4,794,719 -> 4,796,566`, CPU `0.76s -> 0.73s`
      - `json_logs_32m.jsonl`: unchanged at `688,174`, CPU unchanged at `0.19s`
      - level 1 stayed byte-identical but CPU drifted (`decodecorpus 0.17s -> 0.18s`, `json 0.11s -> 0.13s`)
    - Conclusion:
      - The useful zero-literal signal is not concentrated in the `33..=128` non-repeat region by itself.
      - This loses too much decodecorpus compression and is not level-1 clean, so the retained path stays on repeat-only sparse indexing for zero-literal matches.
  - Tightened that same non-repeat experiment to a `16`-byte dense limit, so only zero-literal non-repeat `Best` matches longer than `16` bytes would switch to sparse tail inserts. Result: reject.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-zero-literal-nonrepeat16-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-zero-literal-nonrepeat16-l1.md`
    - Key result:
      - `decodecorpus_pack.bin`: `4,794,719 -> 4,797,718`, CPU `0.76s -> 0.76s`
      - `json_logs_32m.jsonl`: unchanged at `688,174`, CPU `0.19s -> 0.18s`
      - level 1 stayed byte-identical, but JSON CPU drifted from `0.11s` to `0.12s`
    - Conclusion:
      - The remaining compression win from all-zero-literal sparse indexing is not recoverable by simple non-repeat dense-limit tuning.
      - Both `32` and `16` confirm that this local threshold path is tapped out; the retained branch should keep the repeat-only sparse rule and spend effort on deeper parser control-flow changes instead.
  - That makes the next credible rewrite target concrete: not another helper optimization, but a `Best`-only parser path closer to C fast/dfast's adjacent-position pipeline. The most plausible bounded experiment is a recent-history flat candidate cache or adjacent-position pipeline over the current prefix plus the most recent few history entries, paired with the existing retained block-splitting logic. Any such rewrite should be benchmarked first on `decodecorpus_pack.bin` and `json_logs_32m.jsonl` before broader adoption.
  - Tried a narrower `ip+1` sparse-index analogue from C `dfast`: when a next-position sparse match wins, explicitly add the current position before the existing sparse inserts. Result: reject.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-nextpos-current-index-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-nextpos-current-index-l1.md`
    - Key result:
      - `decodecorpus_pack.bin`: `4,794,719 -> 4,794,718`, but CPU regressed `0.72s -> 0.73s`
      - `json_logs_32m.jsonl`: unchanged at `688,174` with CPU unchanged at `0.19s`
      - level 1 stayed byte-identical and CPU-flat
    - Conclusion: this path was already mostly covered. `add_suffixes_for_match()` already preserves the sparse current start and `start+2`, so the extra insert mostly re-touched an existing slot. The missing C behavior is not “current-position sparse index preservation” for the retained next-position emit path.
  - Kept a small shared-path CPU cleanup on top of the retained external second-newest sidecar baseline: guard all four `best_second_newest_candidate()` callsites behind `use_second_newest_probe`, while keeping the sidecar vector aligned with `window`. Result: retain.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-external-sidecar-guarded-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-external-sidecar-guarded-repeat-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-external-sidecar-guarded-l1.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-external-sidecar-guarded-repeat-l1.md`
    - Verification:
      - `cargo test -q -p ruzstd match_generator -- --nocapture`
      - `cargo build --release -q -p ruzstd-cli`
    - Key result:
      - bytes stayed identical to the retained external-sidecar baseline
      - `Best decodecorpus_pack.bin`: `0.89s -> 0.87s`, repeat `0.87s -> 0.86s`
      - `Best json_logs_32m.jsonl`: `0.20s -> 0.19s`, repeat stayed in the `0.19s/0.20s` band
      - level 1 stayed byte-identical and decodecorpus moved `0.19s -> 0.18s` on both runs
    - Interpretation:
      - This is a real shared-path win. The extra no-op second-newest probe plumbing was still costing CPU at both levels even when the feature was disabled.
      - The sidecar storage itself still needs to stay aligned with `window`; only the callsite guards are retained from this cleanup.
  - Also tried to remove the sidecar/window alignment overhead completely by pushing sidecar storage only for binary `Best` entries and skipping the placeholder entries elsewhere. Result: reject immediately.
    - Failure:
      - `target/release/ruzstd-cli compress /home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin ... -l 4` panicked at `ruzstd/src/encoding/match_generator.rs:1740` with `index out of bounds: the len is 1 but the index is 1`
    - Cause:
      - `best_second_newest_sidecars[last_entry_idx]` assumes the sidecar vector is indexed exactly like `window`
      - removing empty placeholders broke that invariant as soon as a text/history entry preceded a binary current entry
    - Conclusion:
      - sidecar storage can maybe be redesigned later, but not by simply dropping the placeholder entries in the current representation.
  - Tried a bounded `Best`-only binary immediate repcode path modeled directly on C `ZSTD_dfast`'s post-match `offset_2` loop: after every emitted binary-like match, the next `next_sequence()` call first checked only `offset_history.second` at the current zero-literal position before running the full repeat/window search. Result: reject.
    - Rationale:
      - In C `double_fast`, the immediate post-match rep loop checks only `offset_2`, which corresponds to the current Rust zero-literal `RepeatCandidateKind::Second`.
      - The earlier ungated `offset_2` drain hurt JSON badly. The bounded experiment here tried the same idea only on binary-like `Best` blocks, since the retained diagnostics show JSON's zero-literal chaining is dominated by the third repcode while decodecorpus is much more mixed.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-binary-secondrep-vsretained-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-binary-secondrep-vsretained-l1.md`
    - Verification before benchmark:
      - `cargo test -q -p ruzstd match_generator -- --nocapture`
      - `cargo build --release -q -p ruzstd-cli`
    - Key result versus `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-best-external-sidecar-retained`:
      - `decodecorpus_pack.bin`: `4,692,418 -> 4,693,175`, CPU `0.88s -> 0.89s`
      - `json_logs_32m.jsonl`: unchanged at `688,174`, CPU unchanged at `0.19s`
      - level 1 stayed byte-identical, but JSON CPU drifted from `0.12s` to `0.13s`
    - Conclusion:
      - Binary-only gating removes the catastrophic JSON byte regression from the earlier ungated `offset_2` drain, but it still gives back decodecorpus bytes and does not improve CPU.
      - The useful lesson is narrower: the C immediate rep loop is not the missing win by itself in the current Rust parser geometry, even when restricted to binary `Best` blocks.
  - Tried a C-shaped selective-maintenance cut on the retained external second-newest sidecar: keep the main suffix-table inserts exact, but stop updating the `Best` second-newest sidecar for skipped no-match positions added by the adaptive miss-step loop. Result: reject.
    - Rationale:
      - C `double_fast` does not fully maintain both hash tables at every auxiliary position; it uses selective insertion (`fastHashFillStep`) for the secondary structure.
      - In the current Rust shape, the closest bounded analogue is to preserve all suffix-table inserts but avoid the extra sidecar bookkeeping on skipped miss-step positions that were not actually parsed.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-sidecar-skipmaint-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-sidecar-skipmaint-l1.md`
    - Verification before benchmark:
      - `cargo test -q -p ruzstd match_generator -- --nocapture`
      - `cargo build --release -q -p ruzstd-cli`
    - Key result versus `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-best-external-sidecar-retained`:
      - `decodecorpus_pack.bin`: `4,692,418 -> 4,692,443`, CPU `0.88s -> 0.86s`
      - `json_logs_32m.jsonl`: unchanged at `688,174`, CPU `0.19s -> 0.20s`
      - level 1 stayed byte-identical, but decodecorpus stayed at `0.19s` and JSON drifted `0.12s -> 0.13s`
    - Conclusion:
      - This does save a little decodecorpus CPU, but not beyond the existing repeat-run noise band of the retained guarded baseline, and it pushes level-1 JSON the wrong way.
      - The useful lesson is that sidecar maintenance on skipped miss-step positions is not the dominant remaining cost. The next credible CPU move still needs to target parser control flow more fundamentally.
  - Tightened the retained external second-newest sidecar probe scope from the two most recent prior entries down to only the most recent prior entry. Result: retain.
    - Rationale:
      - This is the cleanest untested C-shaped cut on the retained path. C `dfast` does not keep our broader recent-candidate expansion, so shrinking the extra probe radius toward a single adjacent-history competitor is closer to its control flow than preserving two-entry lookback indefinitely.
      - Unlike the earlier `threshold8` gate, this reduces sidecar competition by scope rather than by current-match length.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-secondnewest-limit1-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-secondnewest-limit1-repeat-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-secondnewest-limit1-l1.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-secondnewest-limit1-repeat-l1.md`
    - Verification before full suite:
      - `cargo test -q -p ruzstd match_generator -- --nocapture`
      - `cargo build --release -q -p ruzstd-cli`
    - Repeated result versus `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-best-external-sidecar-retained`:
      - `decodecorpus_pack.bin`: `4,692,418 -> 4,693,766`, CPU `0.90s -> 0.87s`
      - `json_logs_32m.jsonl`: unchanged at `688,174`, CPU stayed in the `0.19s/0.20s` band
      - `repeated_text_32m.txt`: unchanged at `3,127`
      - `xorshift_32m.bin`: unchanged at `33,555,210`
      - level 1 stayed byte-identical on both runs, with decodecorpus moving `0.19s -> 0.18s` and JSON staying in the `0.12s` band
    - Interpretation:
      - This gives back only about `1.3 KiB` on decodecorpus while keeping Rust far ahead of C `-4` there (`4,693,766` vs `4,789,813`), and it recovers a real slice of the second-newest CPU tax.
      - This is a better trade for the branch objective than the wider two-entry probe: still clearly better than C on compression for the tracked fixtures, and measurably closer on CPU.
  - Tried a repeat-first gate on top of that retained one-entry sidecar: if a repeat candidate already existed, skip probing `second_newest`. Result: reject as a no-op.
    - Rationale:
      - This was a closer match to fast/dfast’s repcode-first bias than the earlier raw match-length threshold.
      - If the extra recent candidate was mostly competing with live repeat candidates, this should have reduced CPU without a meaningful byte change.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-secondnewest-repeatgate-vsretained-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-secondnewest-repeatgate-vsretained-l1.md`
    - Key result versus `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-best-external-sidecar-retained`:
      - identical to the retained one-entry sidecar table within the benchmark noise band:
        - `decodecorpus_pack.bin`: `4,693,766` at `0.87s`
        - `json_logs_32m.jsonl`: `688,174` at `0.19s`
        - level 1 unchanged at `5,324,267` / `690,084`
    - Conclusion:
      - In the retained path, `second_newest` is effectively not being consulted in the cases where a repeat candidate already exists.
      - That is useful evidence, but not a retained code change.
  - Tried a stricter one-entry sidecar gate: probe `second_newest` only when `oldest` produced no candidate at all. Result: reject.
    - Rationale:
      - This is the next logical C-shaped tightening after the one-entry scope cut: use the extra recent candidate only on same-entry misses, instead of whenever `oldest` merely loses.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-secondnewest-oldestmiss-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-secondnewest-oldestmiss-l1.md`
    - Key result versus `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-best-external-sidecar-retained`:
      - `decodecorpus_pack.bin`: `4,692,418 -> 4,710,351`, CPU `0.85s -> 0.86s`
      - `json_logs_32m.jsonl`: unchanged at `688,174`, CPU `0.19s -> 0.20s`
      - level 1 stayed byte-identical
    - Conclusion:
      - The sidecar still matters even when `oldest` already has some candidate.
      - This gives back too much decodecorpus compression for no CPU win, so the retained state stays on the broader one-entry probe.
  - Revisited the old match-length threshold idea, but on top of the retained one-entry sidecar instead of the older two-entry path: skip `second_newest` when the current same-entry normal candidate is already at least 8 bytes long. Result: reject.
    - Rationale:
      - This is materially different from the earlier threshold experiment because the baseline has already been tightened to a one-entry probe radius.
      - With the branch now explicitly prioritizing movement toward C-like CPU while remaining comfortably ahead of C on compression, this was worth retesting.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-secondnewest-threshold8-limit1-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-secondnewest-threshold8-limit1-l1.md`
    - Key result versus `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-best-external-sidecar-retained`:
      - `decodecorpus_pack.bin`: `4,692,418 -> 4,696,689`, CPU `0.86s -> 0.86s`
      - `json_logs_32m.jsonl`: unchanged at `688,174`, CPU `0.19s -> 0.19s`
      - level 1 stayed byte-identical, but JSON drifted from `0.12s` to `0.13s`
    - Comparison to the retained one-entry sidecar baseline:
      - retained one-entry probe was already better: `decodecorpus_pack.bin 4,693,766 @ 0.87s`
      - this thresholded version only gives back more bytes without a reliable CPU improvement
    - Conclusion:
      - Even on the narrowed one-entry sidecar path, the length-threshold gate does not buy enough CPU to justify the extra byte loss and level-1 JSON drift.
      - The retained state stays on the plain one-entry probe with no additional length threshold.
  - Tried a slightly tighter version of that same idea on the retained one-entry sidecar: skip `second_newest` once the current same-entry normal candidate reaches 6 bytes instead of 8. Result: reject.
    - Rationale:
      - This is the most plausible remaining “weak first hit only” gate. It is closer to the C fast/dfast idea of spending extra search only when the primary candidate is short.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-secondnewest-threshold6-limit1-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-secondnewest-threshold6-limit1-repeat-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-secondnewest-threshold6-limit1-l1.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-secondnewest-threshold6-limit1-repeat-l1.md`
    - Key results versus `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-best-external-sidecar-retained`:
      - first run:
        - `decodecorpus_pack.bin`: `4,692,418 -> 4,705,180`, CPU `0.87s -> 0.86s`
        - `json_logs_32m.jsonl`: unchanged at `688,174`, CPU `0.20s -> 0.19s`
      - repeat run:
        - `decodecorpus_pack.bin`: `4,692,418 -> 4,705,180`, CPU `0.85s -> 0.88s`
        - `json_logs_32m.jsonl`: unchanged at `688,174`, CPU unchanged at `0.19s`
      - level 1 stayed byte-identical on both runs
    - Conclusion:
      - This gate is too unstable. At best it buys about `0.01s` on decodecorpus while giving back more than `11 KiB`; on the repeat run it actually gets slower.
      - The retained one-entry sidecar baseline is still the better trade.
  - Retained a `Best`-only size gate on the one-entry external second-newest sidecar: only enable the extra recent-candidate path for binary blocks at least `16 KiB` long.
    - Rationale:
      - This is a better C-shaped activation cut than the earlier candidate-length gates. It does not guess from the current match quality; it simply reserves the extra recent-candidate machinery for larger binary blocks where the decodecorpus gain is most likely to matter.
      - Smaller binary blocks keep the shared suffix-table behavior but skip the added sidecar allocation, updates, and probes.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-secondnewest-min16k-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-secondnewest-min16k-repeat-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-secondnewest-min16k-l1.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-secondnewest-min16k-repeat-l1.md`
    - Focused coverage:
      - `best_sidecar_tracks_second_newest_for_current_entry`
      - `best_sidecar_is_disabled_for_small_blocks`
    - Repeated result versus `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-best-external-sidecar-retained`:
      - `decodecorpus_pack.bin`: `4,692,418 -> 4,698,198`, CPU `0.86s -> 0.81s`
      - `json_logs_32m.jsonl`: unchanged at `688,174`, CPU stayed in the `0.19s` band
      - `repeated_text_32m.txt`: unchanged at `3,127`
      - `xorshift_32m.bin`: unchanged at `33,555,210`, with a small CPU regression from `0.05s` to `0.06s`
      - level 1 stayed byte-identical on both runs, with decodecorpus moving `0.19s -> 0.18s` and JSON staying in the `0.12s/0.13s` band
    - Interpretation:
      - This gives back about `5.8 KiB` on decodecorpus relative to the wider one-entry sidecar, but recovers roughly `0.05s` to `0.06s` of `Best` CPU there.
      - It still leaves Rust comfortably ahead of C `-4` on decodecorpus compression (`4,698,198` vs `4,789,813`) and keeps the JSON byte profile unchanged.
      - For the current branch objective, this is a better compression/CPU trade than paying the full sidecar cost on every binary block.
  - Tried raising that retained size gate further from `16 KiB` to `32 KiB`. Result: reject.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-secondnewest-min32k-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-secondnewest-min32k-l1.md`
    - Key result versus `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-best-external-sidecar-retained`:
      - `decodecorpus_pack.bin`: `4,692,418 -> 4,699,057`, CPU `0.86s -> 0.81s`
      - `json_logs_32m.jsonl`: unchanged at `688,174`, CPU unchanged at `0.19s`
      - level 1 stayed byte-identical
    - Comparison to the retained `16 KiB` gate:
      - retained `16 KiB`: `decodecorpus 4,698,198 @ 0.81s`
      - `32 KiB`: `decodecorpus 4,699,057 @ 0.81s`
    - Conclusion:
      - The larger gate gives back another ~859 bytes on decodecorpus without recovering any additional CPU.
      - So `16 KiB` remains the better retained cutoff.
  - Retained a structural cleanup of the retained `16 KiB` sidecar path: collapse the extra `second_newest` state from a per-window vector of sidecars down to a single current-entry sidecar.
    - Rationale:
      - With the retained probe radius now fixed at `1`, the extra recent candidate is only ever consulted for the current entry.
      - The older aligned `Vec<Vec<Option<NonZeroU32>>>` shape only existed to support the earlier broader probe radius. Keeping that representation after the current-entry-only cut was unnecessary bookkeeping.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-currentsidecar-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-currentsidecar-repeat-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-currentsidecar-l1.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-currentsidecar-repeat-l1.md`
    - Focused coverage:
      - `best_sidecar_tracks_second_newest_for_current_entry`
      - `best_sidecar_is_disabled_for_small_blocks`
    - Repeated result versus `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-best-external-sidecar-retained`:
      - bytes stayed identical to the retained `16 KiB` gate
      - `decodecorpus_pack.bin`: `4,698,198`, CPU improved from the retained `0.81s` band to `0.80s` / `0.79s`
      - `json_logs_32m.jsonl`: unchanged at `688,174`, CPU stayed in the `0.19s` / `0.20s` noise band
      - level 1 stayed byte-identical, with decodecorpus at `0.18s` and JSON at `0.12s`
    - Interpretation:
      - This is the structural cleanup that the retained one-entry design wanted all along: same emitted bytes, simpler state, less window bookkeeping.
      - The practical win is a small but repeatable decodecorpus CPU improvement with no compression loss and no meaningful level-1 regression.
  - Tried removing repeat-candidate array materialization from the hot path by replacing the three loops over `repeat_offset_candidates()` with direct ordered probing helpers. Result: reject.
    - Rationale:
      - Fresh `perf annotate` on the retained `16 KiB`/current-sidecar state still showed hot stack traffic around repeat-probe setup inside `MatchGenerator::next_sequence`.
      - The bounded cleanup target was to remove the small runtime array construction without changing repeat ordering or emitted bytes.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-repeat-directprobe-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-repeat-directprobe-l1.md`
    - Result versus the latest retained current-sidecar baseline:
      - Level 4 bytes stayed identical everywhere, but CPU drifted the wrong way:
        - `decodecorpus_pack.bin`: `4,698,198 -> 4,698,198`, CPU `0.79s -> 0.80s`
        - `json_logs_32m.jsonl`: `688,174 -> 688,174`, CPU `0.20s -> 0.21s`
      - Level 1 also stayed byte-identical but did not improve:
        - `decodecorpus_pack.bin`: `5,324,267 -> 5,324,267`, CPU `0.18s -> 0.18s`
        - `json_logs_32m.jsonl`: `690,084 -> 690,084`, CPU `0.12s -> 0.14s`
    - Interpretation:
      - The array materialization visible in `perf` is not the real limiting cost by itself.
      - Replacing it with direct helper calls adds enough control-flow or inlining cost that the runtime gets slightly worse even though bytes stay exact.
  - Tried a more structural adjacent-position repeat pipeline in `next_sequence()`: probe current-position and `ip+1` repeat candidates together in one pass, instead of doing the current repeat loop and then a separate `best_repeat_candidate_at(ip+1, ...)` pass. Result: reject.
    - Rationale:
      - Fresh `perf` on the retained current-sidecar state still showed `MatchGenerator::next_sequence` dominating (`decodecorpus` about `73%` children / `68%` self; `json` about `54%` children / `49%` self).
      - The hot assembly around the repeat loops and adjacent-position repeat lookahead made this the most plausible bounded parser/control-flow experiment that still follows C `dfast`'s adjacent-position shape.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-repeat-pipeline-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-repeat-pipeline-l1.md`
    - Result versus the previously retained current-sidecar baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,698,198 -> 4,715,344`, CPU `0.79s -> 0.79s`
        - `json_logs_32m.jsonl`: `688,174 -> 656,943`, CPU `0.20s -> 0.19s`
        - `repeated_text_32m.txt`: `3,127 -> 2,874`, CPU flat
      - Level 1:
        - bytes stayed identical, but CPU regressed (`decodecorpus 0.18s -> 0.19s`, `json 0.12s -> 0.14s`)
    - Interpretation:
      - The adjacent-position repeat pipeline clearly helps the repeat-dominated text fixtures.
      - But as a shared-path rewrite it gives back too much decodecorpus compression and still perturbs lower-level codegen in the wrong direction.
  - Narrowed that same idea to a `Best` text-only path, leaving the binary `Best` and lower-level repeat loops on the old code. Result: reject.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-repeat-text-pipeline-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-repeat-text-pipeline-l1.md`
    - Result versus the previously retained current-sidecar baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,698,198 -> 4,703,526`, CPU `0.79s -> 0.81s`
        - `json_logs_32m.jsonl`: `688,174 -> 656,943`, CPU flat at `0.20s`
        - `repeated_text_32m.txt`: `3,127 -> 2,874`, CPU flat
      - Level 1:
        - bytes still stayed identical, but CPU remained worse (`decodecorpus 0.18s -> 0.19s`, `json 0.12s -> 0.13s`)
    - Interpretation:
      - The text compression gain is real and repeat-driven.
      - But even after narrowing the runtime branch, the code motion still changes lower-level performance enough that it does not clear the shared-path bar for retention.
  - After reverting the repeat-pipeline experiments, benchmarked the current source tree again to establish where the live workspace now stands.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-restore-current-sidecar-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-restore-current-sidecar-l1.md`
    - Current source-tree result:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,698,198`
        - `json_logs_32m.jsonl`: `688,174`
        - `repeated_text_32m.txt`: `3,127`
        - `xorshift_32m.bin`: `33,555,210`
      - Level 1:
        - `decodecorpus_pack.bin`: `5,324,267`
        - `json_logs_32m.jsonl`: `690,084`
        - `repeated_text_32m.txt`: `2,874`
        - `xorshift_32m.bin`: `33,555,210`
    - Interpretation:
      - The matcher experiments were reverted cleanly, and the live source-tree benchmark is back on the retained current-sidecar output shape.
      - Treat these restore-check reports as the authoritative fresh source-tree benchmark state for subsequent work, and re-baseline future comparisons from them rather than from the older external retained binary snapshot.
  - Tried a larger structural split aligned with C's separate `fast` / `dfast` strategy functions: route plain levels through a simpler candidate path and keep the fuller path only for `Best`. Result: reject.
    - Rationale:
      - The current Rust matcher still carries all `Best` candidate-path control flow in one function, even when only the plain path is active.
      - Splitting the candidate generation logic by strategy is a direct analogue of C keeping separate parser functions for `fast` and `double_fast`.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-strategy-split-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-strategy-split-l1.md`
    - Result versus the fresh restore baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,698,198 -> 4,698,198`, CPU `0.82s -> 0.84s`
        - `json_logs_32m.jsonl`: `688,174 -> 688,174`, CPU `0.20s -> 0.21s`
        - `repeated_text_32m.txt`: `3,127 -> 3,127`, CPU flat
      - Level 1:
        - bytes stayed identical
        - `decodecorpus_pack.bin`: `0.19s -> 0.18s`
        - `json_logs_32m.jsonl`: `0.13s -> 0.13s`
    - Interpretation:
      - The split preserved bytes against the actual restored baseline, so the earlier “fresh restore” drift was an artifact of benchmarking during the adjacent-position repeat experiments, not a retained source difference.
      - But the CPU movement is not strong or consistent enough: level-4 `decodecorpus` and JSON got slightly worse, while level-1 only improved by one small decodecorpus step.
      - This means simple strategy outlining alone is not enough; the remaining gain still needs a better internal parser representation, not just different function boundaries.
  - Retained a separate `Best`-text repeat helper instead of embedding the adjacent-position repeat pipeline into the shared matcher path.
    - Rationale:
      - The earlier inline text-only repeat pipeline showed a real compression win on repeat-dominated text, but its code motion polluted the shared `next_sequence()` path and caused level-1 CPU drift.
      - This version isolates the text-only adjacent-position repeat logic behind a dedicated helper, so lower levels and the plain `Best` binary path stay on the old inline repeat loop.
    - Focused coverage:
      - Added `best_text_repeat_helper_can_prefer_next_position_repeat_match`.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-repeat-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-l1.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-repeat-l1.md`
    - First run versus the fresh restore baseline:
      - `decodecorpus_pack.bin`: `4,698,198 -> 4,702,547`, CPU `0.82s -> 0.83s`
      - `json_logs_32m.jsonl`: `688,174 -> 602,826`, CPU `0.20s -> 0.23s`
      - `repeated_text_32m.txt`: `3,127 -> 2,874`, CPU flat at `0.03s`
      - level 1 stayed byte-identical and CPU-stable
    - Repeat run:
      - `decodecorpus_pack.bin`: `4,698,198 -> 4,702,547`, CPU `0.82s -> 0.82s`
      - `json_logs_32m.jsonl`: `688,174 -> 602,826`, CPU `0.20s -> 0.25s`
      - `repeated_text_32m.txt`: `3,127 -> 2,874`, CPU flat
      - level 1 stayed byte-identical and CPU-stable again
    - Interpretation:
      - This is a real retained `Best`-level compression win on the text-heavy fixtures without the earlier level-1 drift.
      - The cost is small but real on `decodecorpus`, and moderate on `json` CPU, so this belongs in the higher-compression `Best` strategy rather than the plain path.
      - It also gives the next CPU task a clearer target: optimize the isolated text-repeat helper itself instead of contaminating the shared matcher loop.
  - Current live source-tree benchmark state after retaining the isolated `Best`-text repeat helper:
    - Level 4:
      - `decodecorpus_pack.bin`: `4,702,547`
      - `json_logs_32m.jsonl`: `602,826`
      - `repeated_text_32m.txt`: `2,874`
      - `xorshift_32m.bin`: `33,555,210`
    - Level 1:
      - `decodecorpus_pack.bin`: `5,324,267`
      - `json_logs_32m.jsonl`: `690,084`
      - `repeated_text_32m.txt`: `2,874`
      - `xorshift_32m.bin`: `33,555,210`
  - Tried narrowing the retained `Best`-text repeat helper to small literal runs only (`literal_len <= 8`). Result: reject.
    - Rationale:
      - The helper's direct `ip+1` repeat wins on JSON are rare under the retained parser shape, so a small-literal gate looked like a plausible way to keep the parser-shape gain while reducing speculative helper work.
    - Focused coverage:
      - Added a temporary helper-activation test, then removed it after reverting the runtime gate.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-lit8-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-lit8-l1.md`
    - Result versus the fresh restore baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,698,198 -> 4,702,542`, CPU `0.82s -> 0.85s`
        - `json_logs_32m.jsonl`: `688,174 -> 602,826`, CPU `0.20s -> 0.24s`
        - `repeated_text_32m.txt`: `3,127 -> 2,874`, CPU flat at `0.03s`
        - `xorshift_32m.bin`: unchanged bytes, CPU `0.06s -> 0.08s`
      - Level 1:
        - bytes and CPU stayed flat
    - Interpretation:
      - The literal-run gate did not preserve the earlier decodecorpus CPU band and made xorshift noisier while leaving JSON CPU materially above baseline.
      - So the retained helper should stay unconditional within the `Best` text path; this threshold is another local gate that does not recover enough CPU to justify its complexity.
  - Tried splitting the retained `Best`-text repeat helper into separate current-repeat and next-position-repeat passes, with the goal of preserving bytes while reducing branchy helper work. Result: reject.
    - Rationale:
      - The retained helper currently interleaves current-position and `ip+1` repeat probing in one loop.
      - A split current-pass / next-pass structure looked like a behavior-preserving cleanup that could reduce the hot branch on `current_candidate.is_some()`.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-l1.md`
    - Result versus the fresh restore baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,698,198 -> 4,703,526`, CPU `0.82s -> 0.86s`
        - `json_logs_32m.jsonl`: `688,174 -> 656,943`, CPU flat at `0.20s`
        - `repeated_text_32m.txt`: `3,127 -> 2,874`, CPU flat
        - `xorshift_32m.bin`: unchanged bytes, CPU `0.06s -> 0.09s`
      - Level 1:
        - bytes stayed identical
        - `decodecorpus_pack.bin`: `0.19s -> 0.20s`
    - Interpretation:
      - This was not behavior-preserving in practice. It gave back too much JSON compression and drifted level-1 decodecorpus CPU upward.
      - The helper must therefore stay in its interleaved form for now; even seemingly local control-flow reshapes are still affecting the parse.
  - Tried removing the persistent `base_offset` bookkeeping from window entries and computing the byte distance during the reverse scan instead. Result: reject.
    - Rationale:
      - `add_data()` currently updates every live window entry on each block just to keep `base_offset` current.
      - This looked like a pure bookkeeping cost under `commit_space()`, so replacing it with reverse-scan accumulation was a plausible byte-stable CPU cleanup.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-no-baseoffset-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-no-baseoffset-l1.md`
    - Result versus the fresh restore baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,698,198 -> 4,702,547`, CPU `0.82s -> 0.92s`
        - `json_logs_32m.jsonl`: `688,174 -> 602,826`, CPU `0.20s -> 0.25s`
        - `repeated_text_32m.txt`: `3,127 -> 2,874`, CPU `0.03s -> 0.05s`
      - Level 1:
        - bytes stayed identical
        - CPU stayed flat on the main fixtures
    - Interpretation:
      - The rewrite preserved bytes on the retained helper path, but the CPU result was clearly worse.
      - So the per-entry `base_offset` updates are not the practical bottleneck they looked like in `commit_space()`, or the accumulated offset calculation cost is more expensive in the hot scan than the bookkeeping it replaces.
  - Retained a separate `next_sequence_best_text()` parser loop for the `Best` text-repeat helper path, instead of keeping that control flow inside the shared `next_sequence()` function.
    - Rationale:
      - C keeps separate fast/dfast parser functions instead of one shared hot loop with strategy branches.
      - The retained text-repeat helper was already a real higher-level compression win, but its CPU tax sat inside the generic parser loop.
      - Splitting the `Best` text path into its own parser entry point lets the default path compile without the helper branch while preserving the retained helper semantics exactly.
    - Coverage:
      - Existing matcher tests plus release Rust/C round-trip checks were rerun.
      - This is a performance-only refactor with exact output bytes on the benchmark fixtures, so benchmark-backed verification is appropriate.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-textpath-split-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-textpath-split-l1.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-textpath-split-repeat-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-textpath-split-repeat-l1.md`
    - Result versus the fresh restore baseline:
      - Level 4, first run:
        - `decodecorpus_pack.bin`: `4,698,198 -> 4,702,547`, CPU `0.82s -> 0.84s`
        - `json_logs_32m.jsonl`: `688,174 -> 602,826`, CPU `0.20s -> 0.23s`
        - `repeated_text_32m.txt`: `3,127 -> 2,874`, CPU flat
        - `xorshift_32m.bin`: unchanged bytes, CPU `0.06s -> 0.10s`
      - Level 4, repeat run:
        - `decodecorpus_pack.bin`: `4,698,198 -> 4,702,547`, CPU `0.82s -> 0.81s`
        - `json_logs_32m.jsonl`: `688,174 -> 602,826`, CPU `0.20s -> 0.22s`
        - `repeated_text_32m.txt`: `3,127 -> 2,874`, CPU flat
        - `xorshift_32m.bin`: unchanged bytes, CPU `0.06s -> 0.06s`
      - Level 1:
        - bytes stayed identical on both runs
        - `decodecorpus_pack.bin`: `0.19s -> 0.18s`
        - `json_logs_32m.jsonl`: `0.13s -> 0.13s` then `0.14s`
    - Interpretation:
      - The split keeps the retained level-4 bytes exact while recovering a useful slice of CPU on the two important `Best` fixtures on the repeat run.
      - Level-1 bytes remain exact and CPU stays in the existing noise band; the only visible drift is a one-step JSON timing wobble, which does not contradict the change being a codegen/layout refactor rather than a semantic change.
  - Verification on the retained split parser path:
    - `cargo fmt --all --check`
    - `cargo clippy -q -p ruzstd --lib -- -D warnings`
    - `cargo test -q --workspace`
    - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd best_level_external_fixture_round_trips_with_rust_and_c_decoders_from_env -- --ignored --nocapture`
    - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd-cli best_level_progress_monitor_round_trips_external_fixture_from_env -- --ignored --nocapture`
  - Tried inlining `best_text_repeat_candidate()` into the retained split `Best` text parser path. Result: reject.
    - Rationale:
      - Fresh profiles on the retained split path still show `best_text_repeat_candidate()` as a large self-time contributor on `json_logs_32m.jsonl`.
      - Since the helper now only lives under `next_sequence_best_text()`, inlining it was a safe candidate for codegen improvement without affecting the default parser path.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-texthelper-inline-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-texthelper-inline-l1.md`
    - Result versus the fresh restore baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,698,198 -> 4,702,547`, CPU `0.82s -> 0.87s`
        - `json_logs_32m.jsonl`: `688,174 -> 602,826`, CPU `0.20s -> 0.23s`
        - `repeated_text_32m.txt`: `3,127 -> 2,874`, CPU flat
        - `xorshift_32m.bin`: unchanged bytes, CPU `0.06s -> 0.09s`
      - Level 1:
        - bytes and CPU stayed flat
    - Interpretation:
      - The helper remains a hotspot, but forcing it inline made codegen worse on the retained `Best` path.
      - Keep the helper outlined under the split text parser path.
  - Tried a dedicated `Best` binary parser loop, mirroring the retained text split on the decodecorpus path. Result: reject.
    - Rationale:
      - After splitting the `Best` text path, `decodecorpus_pack.bin` still runs through the shared parser loop.
      - A separate `Best` binary parser path was the next closest analogue to C keeping separate fast/dfast-style loops, with the goal of removing the remaining mixed-path branches from decodecorpus.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-binarypath-split-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-binarypath-split-l1.md`
    - Result versus the fresh restore baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,698,198 -> 4,702,547`, CPU `0.82s -> 0.87s`
        - `json_logs_32m.jsonl`: `688,174 -> 602,826`, CPU `0.20s -> 0.24s`
        - `repeated_text_32m.txt`: `3,127 -> 2,874`, CPU flat
        - `xorshift_32m.bin`: unchanged bytes, CPU `0.06s -> 0.10s`
      - Level 1:
        - bytes and CPU stayed flat
    - Interpretation:
      - The split preserved bytes, but it did not produce the hoped-for CPU win on decodecorpus and instead moved the whole level-4 table into the same worse timing band as the rejected helper-inline experiment.
      - So the remaining decodecorpus cost is not just “shared parser branches” at this level of structure.
  - Tried reusing the retained `Best` second-newest sidecar buffer across same-sized blocks instead of reallocating it in `add_data()`. Result: reject.
    - Rationale:
      - Fresh profiles still show `commit_space()` and matcher setup in the hot path.
      - Reusing the sidecar allocation for the retained `Best` current-entry sidecar looked like a safe way to shave setup cost without changing any parser decisions.
    - Validation:
      - `cargo fmt --all --check`
      - `cargo test -q -p ruzstd match_generator -- --nocapture`
      - `cargo build --release -q -p ruzstd-cli`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd best_level_external_fixture_round_trips_with_rust_and_c_decoders_from_env -- --ignored --nocapture`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd-cli best_level_progress_monitor_round_trips_external_fixture_from_env -- --ignored --nocapture`
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-l1.md`
    - Result versus the fresh restore baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,698,198 -> 4,702,547`, CPU `0.82s -> 0.84s`
        - `json_logs_32m.jsonl`: `688,174 -> 602,826`, CPU `0.20s -> 0.23s`
        - `repeated_text_32m.txt`: `3,127 -> 2,874`, CPU flat
        - `xorshift_32m.bin`: unchanged bytes, CPU `0.06s -> 0.10s`
      - Level 1:
        - bytes stayed identical
        - `decodecorpus_pack.bin`: `0.19s -> 0.18s`
        - `json_logs_32m.jsonl`: `0.13s -> 0.13s`
    - Interpretation:
      - The reuse path preserved bytes and all release round-trip checks, but it did not recover CPU on the retained level-4 path and instead moved `decodecorpus` and `json` into a slightly worse timing band.
      - Keep the simpler fresh-allocation path for now.
  - Tried an explicit three-probe rewrite of `best_text_repeat_candidate()`, replacing the small repeat loop with a manually ordered current-then-next probe path. Result: reject.
    - Rationale:
      - Fresh profiles on the retained split text path still show `best_text_repeat_candidate()` as the dominant user-space hotspot on `json_logs_32m.jsonl`.
      - The repeat-candidate set is fixed at three entries, so removing the iterator/loop shape while keeping the retained helper isolated looked like the cleanest remaining codegen experiment in that hotspot.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-l1.md`
    - Result versus the fresh restore baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,698,198 -> 4,703,526`, CPU `0.82s -> 0.81s`
        - `json_logs_32m.jsonl`: `688,174 -> 656,943`, CPU `0.20s -> 0.19s`
        - `repeated_text_32m.txt`: `3,127 -> 2,874`, CPU flat
        - `xorshift_32m.bin`: unchanged bytes, CPU `0.06s -> 0.05s`
      - Level 1:
        - bytes stayed identical
        - `decodecorpus_pack.bin`: `0.19s -> 0.19s`
        - `json_logs_32m.jsonl`: `0.13s -> 0.13s`
    - Interpretation:
      - The rewrite was not semantics-preserving in practice; it materially gave back the retained JSON compression win and moved `decodecorpus` bytes the wrong way too.
      - The retained text helper is sensitive to its existing interleaved repeat-probe ordering, so future work here needs a genuinely different parser strategy rather than a loop-shape rewrite.
  - Re-ran a clean current-entry fast path in `best_window_candidate()` against the corrected retained baseline. Result: reject.
    - Rationale:
      - The retained `second_newest` sidecar is now current-entry-only, so handling the current history entry outside the generic reverse scan looked like the cleanest structural match to the actual parser representation.
      - This revisits an older experiment that had been discarded because the copied baseline binary was no longer trustworthy at the time.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-l1.md`
    - Result versus the fresh restore baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,698,198 -> 4,703,526`, CPU `0.82s -> 0.81s`
        - `json_logs_32m.jsonl`: `688,174 -> 1,745,442`, CPU `0.20s -> 0.19s`
        - `repeated_text_32m.txt`: `3,127 -> 2,899`, CPU `0.03s -> 0.03s`
        - `xorshift_32m.bin`: unchanged bytes, CPU `0.06s -> 0.05s`
      - Level 1:
        - bytes stayed identical
        - `decodecorpus_pack.bin`: `0.19s -> 0.19s`
        - `json_logs_32m.jsonl`: `0.13s -> 0.13s`
    - Interpretation:
      - The fast path is not just a neutral layout tweak; it materially changes parse behavior on the retained `Best` path and catastrophically loses the retained JSON compression win.
      - That closes this avenue with trustworthy numbers: even a current-entry-only split around `best_window_candidate()` is not safe in the existing matcher geometry.
  - Tried a text-only same-block prefix-verification fast path inside the retained `Best` text-repeat helper. Result: reject.
    - Rationale:
      - The retained split text path is still dominated by repeat probing.
      - `match_len_at_offset_with_prefix()` already has a same-block fast path, but `verified_min_match_prefix_len()` still resolves the source through the generic relative-window path. Restricting a direct-slice prefix check to `best_text_repeat_candidate()` looked like the cleanest parse-preserving CPU attempt that avoided touching binary and lower-level paths.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-l1.md`
    - Result versus the fresh restore baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,698,198 -> 4,705,194`, CPU `0.82s -> 0.82s`
        - `json_logs_32m.jsonl`: `688,174 -> 1,777,573`, CPU `0.20s -> 0.74s`
        - `repeated_text_32m.txt`: `3,127 -> 2,896`, CPU flat
        - `xorshift_32m.bin`: unchanged bytes and CPU
      - Level 1:
        - bytes stayed identical
        - `decodecorpus_pack.bin`: `0.19s -> 0.18s`
        - `json_logs_32m.jsonl`: `0.13s -> 0.12s`
    - Interpretation:
      - Even though the change looked semantically equivalent, it dramatically altered the retained `Best` text parse and made JSON much larger and slower.
      - That rules out this direct-slice prefix path in the current helper. The retained text-repeat helper is more sensitive to seemingly equivalent source-resolution changes than expected.
  - Tried a fixed-order rewrite of the retained `Best` text-repeat helper that preserved the original interleaved current/next probe semantics but replaced the tiny dynamic repeat-candidate loop with separate straight-line zero-literal and nonzero-literal paths. Result: reject.
    - Rationale:
      - The previous helper rewrite failed because it changed the current/next interleaving. This follow-up kept that interleaving exactly, with the goal of testing whether the remaining cost was just loop/code-shape overhead around a fixed three-candidate set.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-l1.md`
    - Result versus the fresh restore baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,698,198 -> 4,705,082`, CPU `0.82s -> 0.82s`
        - `json_logs_32m.jsonl`: `688,174 -> 767,985`, CPU `0.20s -> 0.44s`
        - `repeated_text_32m.txt`: `3,127 -> 2,896`, CPU flat
        - `xorshift_32m.bin`: unchanged bytes and CPU
      - Level 1:
        - bytes stayed identical
        - `decodecorpus_pack.bin`: `0.19s -> 0.18s`
        - `json_logs_32m.jsonl`: `0.13s -> 0.13s`
    - Interpretation:
      - Even with the original interleaving preserved, changing the helper into fixed straight-line paths still perturbed the retained `Best` text parse and made JSON both larger and slower.
      - That rules out “just change the loop shape” inside the retained helper. Future work here needs a different parser representation, not another code-shape rewrite of the same probe logic.
  - Tried a unified repeat-match helper for the retained `Best` text-repeat path: compute repeat match length in one helper instead of doing `verified_min_match_prefix_len()` and `match_len_at_offset_with_prefix()` as two separate steps. Result: reject.
    - Rationale:
      - The retained text path is still dominated by repeat probing.
      - This was meant to keep the parser model unchanged while removing duplicate source resolution, especially for same-block repeats where the existing helper verifies the prefix and then re-enters the same-block fast path in `match_len_at_offset_with_prefix()`.
    - Validation:
      - Added focused helper equivalence checks for a same-block repeat case and a short-previous-entry repeat case; both passed.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-l1.md`
    - Result versus the fresh restore baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,698,198 -> 4,705,194`, CPU `0.82s -> 0.82s`
        - `json_logs_32m.jsonl`: `688,174 -> 1,777,573`, CPU `0.20s -> 0.72s`
        - `repeated_text_32m.txt`: `3,127 -> 2,896`, CPU flat
        - `xorshift_32m.bin`: unchanged bytes and CPU
      - Level 1:
        - bytes stayed identical
        - `decodecorpus_pack.bin`: `0.19s -> 0.18s`
        - `json_logs_32m.jsonl`: `0.13s -> 0.12s`
    - Interpretation:
      - The narrow equivalence checks were not sufficient. Despite looking equivalent on the targeted cases, the unified helper still changed the retained `Best` text parse catastrophically on JSON.
      - That closes off this family too: deduplicating repeat prefix verification and full match counting inside the current helper is not safe in this matcher representation.
  - Tried moving the retained `Best` text-repeat helper to a new `CompressionLevel::Max` / CLI level `5` so `Best` could stay closer to C CPU while the more expensive parser lived at a higher level. Result: reject and fully revert.
    - Rationale:
      - The user explicitly wanted CPU-expensive compression wins recorded as higher-level candidates where appropriate.
      - This was the cleanest way to test that policy without discarding the retained text-helper compression work.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-best-text-repeat-helper-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-rs-benchmark-max-l5.md`
    - Result:
      - `Best` returned to the pre-helper baseline exactly, which confirmed the level split itself worked on that side:
        - `decodecorpus_pack.bin`: `4,698,198`
        - `json_logs_32m.jsonl`: `688,174`
      - The new `Max` level did not reproduce the retained helper behavior and instead matched the known-bad unified-helper shape:
        - `decodecorpus_pack.bin`: `4,705,194`
        - `json_logs_32m.jsonl`: `1,777,573`
    - Interpretation:
      - The level split exposed that the worktree still contained a broken text-helper rewrite from the rejected unified-helper experiment.
      - Keeping `Max` would have added API surface around a broken parser path, so the entire split was reverted.
  - Restored the retained `Best` text-repeat baseline after the failed `Max` experiment. Result: keep.
    - Root cause:
      - `best_text_repeat_candidate()` was still computing both `current_candidate` and `next_candidate` but returning only `next_candidate`.
      - The existing next-position helper test also failed to enable `use_text_repeat_pipeline`, so it never exercised the retained helper path and could not catch this regression.
    - Fix:
      - Restore `current_candidate.or(next_candidate)` in `best_text_repeat_candidate()`.
      - Revert the `CompressionLevel::Max` / CLI level `5` split so the retained helper lives under `Best` again.
      - Strengthen coverage so the helper is actually exercised:
        - `best_text_repeat_helper_can_prefer_next_position_repeat_match`
        - `best_text_repeat_helper_keeps_current_repeat_match`
    - Verification:
      - `cargo test -q -p ruzstd match_generator -- --nocapture`
      - `cargo build --release -q -p ruzstd-cli`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd best_level_external_fixture_round_trips_with_rust_and_c_decoders_from_env -- --ignored --nocapture`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd-cli best_level_progress_monitor_round_trips_external_fixture_from_env -- --ignored --nocapture`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-spot-l4.md`
    - Restored retained benchmark state:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,702,547`
        - `json_logs_32m.jsonl`: `602,826`
      - Level 1:
        - `decodecorpus_pack.bin`: `5,324,267`
        - `json_logs_32m.jsonl`: `690,084`
    - Interpretation:
      - This re-established the actual retained `Best` baseline and closed a real regression gap in the test suite.
      - The branch is back on the text-helper state that beats C by a wide margin on JSON and still stays smaller than C on decodecorpus.
  - Tried a `Best` text-block early return in `best_block_segment_lengths()` so obviously text-like blocks would skip the pre-split chunk scan. Result: reject.
    - Rationale:
      - Fresh perf on the restored baseline showed `likely_incompressible` still consuming visible time on `json_logs_32m.jsonl`, even though text blocks do not benefit from the binary-oriented pre-split scan.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-spot-l4.md`
    - Result:
      - Bytes stayed exact on the two-fixture spot check, but JSON CPU did not move and decodecorpus only changed within noise.
    - Interpretation:
      - There is not enough evidence to keep another behavior change in the split policy for this.
      - Leave pre-splitting unchanged and keep looking deeper in the parser path.
  - Tried a two-phase rewrite of the retained `Best` text-repeat helper: scan current-position repeat offsets first, and only scan next-position repeats if no current-position repeat exists at all. Result: reject.
    - Rationale:
      - This looked semantically equivalent because any surviving current-position repeat wins over next-position repeats in the retained helper, so avoiding early next-position work seemed like the cleanest remaining CPU cut in `best_text_repeat_candidate()`.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-spot-l4.md`
    - Result:
      - The rewrite reproduced the old rejected helper regression shape:
        - `decodecorpus_pack.bin`: `4,703,526`
        - `json_logs_32m.jsonl`: `656,943`
      - Reverting it restored the retained spot-check baseline:
        - `decodecorpus_pack.bin`: `4,702,547`
        - `json_logs_32m.jsonl`: `602,826`
    - Interpretation:
      - Even changes that appear algebraically equivalent inside the retained text helper are not safe in this parser shape.
      - That narrows the next step again: further progress needs a different parser representation, not another local rewrite of the helper's repeat search.
  - Added test-only diagnostics to distinguish `second_newest` window wins from ordinary `newest` wins, then re-ran the fixture inspection on the restored retained baseline.
    - Verification:
      - `env RUZSTD_MATCHER_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test -q -p ruzstd inspect_best_matcher_from_env -- --ignored --nocapture`
      - `env RUZSTD_MATCHER_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/json_logs_32m.jsonl cargo test -q -p ruzstd inspect_best_matcher_from_env -- --ignored --nocapture`
    - Useful result:
      - On `json_logs_32m.jsonl`, `second_newest` wins are effectively absent:
        - `window_current_second_newest`: all zero
        - `window_next_position_second_newest`: all zero
      - On `decodecorpus_pack.bin`, `second_newest` matters only for current-position current-entry matches:
        - `window_current_second_newest[0] = 37,497`
        - `window_next_position_second_newest`: all zero
      - Most of those decodecorpus wins are zero-literal:
        - `window_current_second_newest_zero_literals[0] = 33,295`
        - `window_current_second_newest_with_literals[0] = 4,202`
    - Interpretation:
      - The retained `second_newest` path is a decodecorpus-specific binary parser feature, not a text/helper feature.
      - That sharply narrows where future CPU work should focus: current-position binary matching on the current entry, not the retained text helper or next-position paths.
  - Corrected a diagnostics-only labeling bug in the uniform-capacity `best_window_candidate()` fast path.
    - Root cause:
      - One ordinary `newest` branch in the uniform-capacity path was still tagged as `WindowCandidateKind::SecondNewest`, which overstated the apparent emitted importance of the retained sidecar path.
    - Verification:
      - `cargo test -q -p ruzstd match_generator -- --nocapture`
      - `env RUZSTD_MATCHER_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test -q -p ruzstd inspect_best_matcher_from_env -- --ignored --nocapture`
    - Corrected decodecorpus reading:
      - `window_current_newest[0] = 117,261`
      - `window_current_second_newest[0] = 37,497`
      - `window_current_oldest[0] = 112,445`
    - Interpretation:
      - `second_newest` still matters materially, but less overwhelmingly than the mis-labeled diagnostics suggested.
      - The remaining binary-side candidate competition is effectively a three-way current-entry fight between newest, second-newest, and oldest, which points more strongly toward a dedicated current-entry candidate representation than another boolean gate.
  - Tried a binary-only current-entry fast path in `best_window_candidate()`, but validated it with a true local A/B binary comparison instead of noisy spot checks. Result: reject.
    - Rationale:
      - The corrected diagnostics show that `second_newest` only matters on current-position, current-entry binary matches. That made a binary-only current-entry split the narrowest plausible structural CPU cut that avoids the retained text helper entirely.
    - Validation method:
      - Built a candidate release binary with the current-entry fast path.
      - Reverted just that patch, rebuilt a baseline release binary, then benchmarked the two binaries directly with 3-run medians.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-compare-current-entry-fastpath.md`
    - Result:
      - `decodecorpus_pack.bin`: bytes unchanged at `4,702,547`, CPU `0.83s -> 0.84s`
      - `json_logs_32m.jsonl`: bytes unchanged at `602,826`, CPU flat at `0.21s`
    - Interpretation:
      - The current-entry extraction is behavior-preserving on the retained binary path, but it does not buy CPU.
      - That closes off this structural split as well. The remaining gap is deeper than “pull the current entry out of the reverse scan.”
  - Tried a deeper dedicated current-entry probe for the binary `Best` path: specialize the current-entry `newest` / `second_newest` / `oldest` fight instead of routing it through the generic metadata path. Result: reject.
    - Rationale:
      - The corrected diagnostics showed that the sidecar only matters for current-position, current-entry binary matches. A dedicated current-entry probe was the next structural step after the simpler current-entry extraction failed.
    - Validation method:
      - Built a candidate release binary with the dedicated probe.
      - Compared it directly against the rebuilt baseline release binary with 3-run medians and C-zstd decode verification.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-compare-current-entry-specialized.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-spot-restore.md`
    - Result:
      - Candidate vs baseline:
        - `decodecorpus_pack.bin`: `4,702,547 -> 4,702,539`, CPU `0.80s -> 0.83s`
        - `json_logs_32m.jsonl`: `602,826 -> 602,826`, CPU `0.21s -> 0.22s`
      - After reverting the experiment, the restore spot check returned to the retained baseline:
        - `decodecorpus_pack.bin`: `4,702,547`
        - `json_logs_32m.jsonl`: `602,826`
    - Interpretation:
      - The dedicated current-entry probe is not behavior-preserving and still loses CPU.
      - That closes off the “specialize the current-entry candidate ordering inside the existing suffix-store representation” path. The remaining binary-side gap likely needs a different candidate representation, not just a different probe function.
  - Tried that next representation step directly: a dedicated current-entry candidate array for binary `Best` blocks, tracking `oldest`, `newest`, and `second_newest` separately from the generic suffix-store slot. Result: reject.
    - Rationale:
      - The corrected diagnostics show the remaining binary-side fight is specifically between current-entry `oldest`, `newest`, and `second_newest`.
      - After both current-entry extraction and a dedicated probe failed, the next credible move was to change the candidate representation itself instead of adding more probe logic around the existing slot layout.
    - Validation method:
      - Built a candidate release binary with the current-entry candidate array.
      - Reverted the experiment cleanly back to the retained baseline, rebuilt a baseline release binary, and compared the two binaries directly with 3-run medians and C-zstd decode verification.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-entry-array.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-spot-restore.md`
    - Result:
      - Candidate vs baseline:
        - `decodecorpus_pack.bin`: `4,702,547 -> 4,702,539`, CPU `0.83s -> 0.92s`
        - `json_logs_32m.jsonl`: `602,826 -> 602,826`, CPU `0.22s -> 0.24s`
      - After reverting the experiment, the restore spot check returned to the retained baseline:
        - `decodecorpus_pack.bin`: `4,702,547`
        - `json_logs_32m.jsonl`: `602,826`
    - Interpretation:
      - A different current-entry representation inside the existing parser shape is still not enough.
      - It is not byte-stable and it is materially slower, so the next credible move is no longer “another current-entry representation tweak.” It needs a broader parser/search-structure change for binary `Best`.
  - Tried a narrower runtime gate from that evidence: only probe `second_newest` on zero-literal current positions. Result: reject.
    - Rationale:
      - About 89% of observed `second_newest` wins on decodecorpus are zero-literal, so this looked like the narrowest plausible CPU cut.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-spot-l4.md`
    - Result:
      - `decodecorpus_pack.bin`: `4,702,547 -> 4,703,315`
      - `json_logs_32m.jsonl`: unchanged at `602,826`
      - CPU stayed inside noise on the two-fixture spot check.
    - Interpretation:
      - The literal-carrying tail still matters enough that this gate is not worth keeping.
      - Runtime change reverted; keep only the diagnostics.
  - Tried a more direct `double_fast` control-flow analogue for binary `Best`: check `rep1` at `ip+1` before the normal current-position search and commit immediately if it matches. Result: reject.
    - Rationale:
      - Local C `zstd_double_fast.c` does an early `offset_1` repcode check at `ip+1` before the long/short current-position search.
      - Our retained binary `Best` path only did a later, conditional next-position repeat lookahead, so this was the closest remaining parser-control-flow gap to close without rewriting the whole loop.
    - Validation method:
      - Implemented the early `ip+1` `rep1` branch only for binary `Best`, leaving the retained text path unchanged.
      - Built a candidate release binary and compared it directly against the retained baseline release binary with 3-run medians and C-zstd decode verification.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-binary-nextrep1-early.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-restore-binary-nextrep1.md`
    - Result:
      - Candidate vs baseline:
        - `decodecorpus_pack.bin`: `4,702,547 -> 4,708,725`, CPU `0.81s -> 0.81s`
        - `json_logs_32m.jsonl`: `602,826 -> 602,826`, CPU `0.21s -> 0.21s`
      - After reverting the experiment, the restore spot check returned to the retained baseline:
        - `decodecorpus_pack.bin`: `4,702,547`
        - `json_logs_32m.jsonl`: `602,826`
    - Interpretation:
      - The direct `ip+1 rep1` branch from `double_fast` is not the missing win in our parser shape.
      - It gives back decodecorpus compression without improving CPU, so the remaining binary gap is not just the ordering of the next-position repeat probe.
  - Kept a more faithful `double_fast`-shaped binary current-entry signal: a dedicated 8-byte current-entry long-hash candidate for `Best`, with activation only on binary blocks at least `32 KiB` long.
    - Rationale:
      - Local C `zstd_double_fast.c` keeps both a short hash and a long hash, and current-position long matches are checked before the short-match fallback.
      - Our retained binary path previously only had the fixed 5-byte suffix-store candidate set plus the `second_newest` sidecar. The next plausible gap was a real current-entry long-hash candidate, not another probe ordering tweak.
    - Implementation:
      - Added `current_long_hash` in `ruzstd/src/encoding/match_generator.rs`.
      - Updated `add_suffix_at()` to maintain that table only when the `Best` binary current-entry path is active and at least 8 bytes remain.
      - Seeded `best_window_candidate()` with a current-entry 8-byte candidate before the generic suffix-store scan.
      - Tightened allocation so the long-hash table is only allocated when the block length reaches the same `32 KiB` activation threshold.
      - Added focused coverage:
        - `best_current_long_hash_tracks_latest_current_entry_index`
        - `best_current_long_hash_is_disabled_below_threshold`
    - Threshold tuning:
      - First cut at `16 KiB`:
        - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-longhash.md`
        - `decodecorpus_pack.bin`: `4,702,547 -> 4,670,533`, CPU `0.82s -> 0.86s`
      - Rejected `64 KiB` threshold:
        - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-longhash-64k.md`
        - `decodecorpus_pack.bin`: `4,702,547 -> 4,673,806`, CPU `0.82s -> 0.84s`
      - Retained `32 KiB` threshold:
        - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-longhash-32k.md`
        - `decodecorpus_pack.bin`: `4,702,547 -> 4,670,553`, CPU `0.82s -> 0.85s`
    - Final retained full-fixture reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-current-longhash-final-l4.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-current-longhash-final-l1.md`
    - Final retained result versus the previous retained baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,702,547 -> 4,670,553`, CPU `0.87s -> 0.84s`
        - `json_logs_32m.jsonl`: `602,826 -> 602,826`, CPU `0.22s -> 0.23s`
        - `repeated_text_32m.txt`: unchanged at `2,874`
        - `xorshift_32m.bin`: unchanged at `33,555,210`
      - Level 1:
        - all four fixtures stayed byte-identical
    - Verification:
      - `cargo fmt --all --check`
      - `cargo clippy -q -p ruzstd --lib -- -D warnings`
      - `cargo test -q --workspace`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd best_level_external_fixture_round_trips_with_rust_and_c_decoders_from_env -- --ignored --nocapture`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd-cli best_level_progress_monitor_round_trips_external_fixture_from_env -- --ignored --nocapture`
    - Interpretation:
      - This is the first retained binary-side matcher representation change in a while that materially improves decodecorpus compression without disturbing level 1 or JSON bytes.
      - The remaining binary gap is now narrower, and the next useful move should probably target CPU around the retained long-hash path rather than going back to more local repeat-probe ordering tweaks.
  - Added test-only diagnostics for retained long-hash wins and used them to inspect the new current-entry long-hash path on the real fixtures.
    - Verification:
      - `env RUZSTD_MATCHER_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test -q -p ruzstd inspect_best_matcher_from_env -- --ignored --nocapture`
      - `env RUZSTD_MATCHER_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/json_logs_32m.jsonl cargo test -q -p ruzstd inspect_best_matcher_from_env -- --ignored --nocapture`
    - Useful result:
      - `json_logs_32m.jsonl`:
        - `window_current_long_hash = 0`
      - `decodecorpus_pack.bin`:
        - `window_current_long_hash = 134,801`
        - `window_current_long_hash_zero_literals = 117,376`
        - `window_current_long_hash_with_literals = 17,425`
    - Interpretation:
      - The retained long-hash path is entirely a decodecorpus-style binary feature, not a text-path feature.
      - Most wins are zero-literal, but the literal-carrying tail is still large enough that it cannot simply be discarded.
  - Tried the narrowest CPU cut from those diagnostics: only probe the retained long-hash candidate on zero-literal positions. Result: reject.
    - Rationale:
      - About 87% of emitted long-hash wins on decodecorpus were zero-literal, so this was the narrowest plausible runtime gate.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-longhash-zero-lit.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-longhash-zero-lit-l1.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-restore-current-longhash.md`
    - Result versus the retained `32 KiB` long-hash baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,670,553 -> 4,672,564`
        - `json_logs_32m.jsonl`: unchanged at `602,826`
      - Level 1:
        - bytes stayed identical
      - CPU did not produce a clean win in the full tables.
    - Interpretation:
      - The literal-carrying long-hash wins still matter enough that this gate is not worth keeping.
      - Runtime change reverted; keep only the diagnostics.
  - Kept a CPU-oriented cleanup around the retained long-hash path: once the current-entry 8-byte long-hash candidate exists, skip the redundant current-entry 5-byte suffix-store scan and continue only with older-entry competition.
    - Rationale:
      - The retained long-hash candidate is already the closest analogue to `double_fast`'s current-position long-hash hit.
      - After that candidate exists, rescanning the same current entry via `oldest` / `newest` / `second_newest` is the most obvious remaining redundant work in our current parser shape.
    - Validation method:
      - Built a candidate release binary with the current-entry skip.
      - Benchmarked it directly against the retained `32 KiB` long-hash release binary with 3-run medians and C-zstd decode verification.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-longhash-skip-current-entry.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-longhash-skip-current-entry-l1.md`
    - Result versus the retained `32 KiB` long-hash baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,670,553 -> 4,675,681`
        - `json_logs_32m.jsonl`: unchanged at `602,826`
        - `repeated_text_32m.txt`: unchanged at `2,874`
        - `xorshift_32m.bin`: unchanged at `33,555,210`
        - CPU:
          - `decodecorpus_pack.bin`: `0.89s -> 0.82s`
          - `json_logs_32m.jsonl`: `0.22s -> 0.21s`
      - Level 1:
        - all four fixtures stayed byte-identical
    - Verification:
      - `cargo fmt --all --check`
      - `cargo clippy -q -p ruzstd --lib -- -D warnings`
      - `cargo test -q --workspace`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd best_level_external_fixture_round_trips_with_rust_and_c_decoders_from_env -- --ignored --nocapture`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd-cli best_level_progress_monitor_round_trips_external_fixture_from_env -- --ignored --nocapture`
    - Interpretation:
      - This gives back about `5 KiB` of decodecorpus compression but recovers about `8%` of its level-4 CPU while leaving level 1 exact and leaving JSON bytes unchanged.
      - Since the branch still remains materially smaller than C `-4` on decodecorpus, this is a reasonable move toward the CPU side of the parity target.
  - Kept a second CPU-oriented cut on top of that retained long-hash path: if the current-entry 8-byte long-hash candidate is already at least `16` bytes long, skip the older-entry scan entirely.
    - Rationale:
      - After the previous retained cleanup removed redundant current-entry short-candidate work, the next remaining cost was older-entry competition against a current long-hash hit.
      - This is the narrowest length-based analogue to trusting `double_fast`'s strong current-position long-hash signal enough to stop searching earlier.
    - Validation method:
      - Built a candidate release binary with the `16`-byte early return.
      - Compared it directly against the actual retained baseline binary (`/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-best-current-longhash-skip-current-entry`) with 3-run medians and C-zstd decode verification.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-longhash-skipolder16-vsretained.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-longhash-skipolder16-vsretained-l1.md`
    - Result versus the current retained baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,675,681 -> 4,676,147`
        - `json_logs_32m.jsonl`: unchanged at `602,826`
        - `repeated_text_32m.txt`: unchanged at `2,874`
        - `xorshift_32m.bin`: unchanged at `33,555,210`
        - CPU:
          - `decodecorpus_pack.bin`: `0.86s -> 0.83s`
          - `json_logs_32m.jsonl`: stayed `0.21s`
      - Level 1:
        - all four fixtures stayed byte-identical
    - Verification:
      - `cargo fmt --all --check`
      - `cargo clippy -q -p ruzstd --lib -- -D warnings`
      - `cargo test -q --workspace`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd best_level_external_fixture_round_trips_with_rust_and_c_decoders_from_env -- --ignored --nocapture`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd-cli best_level_progress_monitor_round_trips_external_fixture_from_env -- --ignored --nocapture`
    - Interpretation:
      - This gives back only about `466` bytes on decodecorpus while recovering another ~`3.5%` of level-4 CPU there.
      - JSON bytes and level 1 remain unchanged, so this is a good additional move toward CPU parity on top of the retained long-hash compression win.
  - Tuned that retained long-hash early-exit threshold upward from `16` to `24` and kept the new threshold.
    - Rationale:
      - After retaining the `16`-byte cut, the next practical question was whether a slightly higher trust threshold could recover some decodecorpus bytes without giving back the CPU win.
    - Validation method:
      - Built a candidate release binary with `BEST_CURRENT_LONG_HASH_SKIP_OLDER_LEN = 24`.
      - Compared it directly against the actual retained `16`-byte baseline binary (`/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/artifacts/ruzstd-cli-best-current-longhash-skipolder16`) with 3-run medians and C-zstd decode verification.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-longhash-skipolder24-vs16.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-longhash-skipolder24-vs16-l1.md`
    - Result versus the `16`-byte retained baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,676,147 -> 4,675,942`
        - `json_logs_32m.jsonl`: unchanged at `602,826`
        - `repeated_text_32m.txt`: unchanged at `2,874`
        - `xorshift_32m.bin`: unchanged at `33,555,210`
        - CPU:
          - `decodecorpus_pack.bin`: `0.87s -> 0.83s`
          - `json_logs_32m.jsonl`: `0.21s -> 0.22s`
      - Level 1:
        - all four fixtures stayed byte-identical
        - `decodecorpus_pack.bin`: `0.19s -> 0.18s`
    - Verification:
      - `cargo fmt --all --check`
      - `cargo clippy -q -p ruzstd --lib -- -D warnings`
      - `cargo test -q --workspace`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd best_level_external_fixture_round_trips_with_rust_and_c_decoders_from_env -- --ignored --nocapture`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd-cli best_level_progress_monitor_round_trips_external_fixture_from_env -- --ignored --nocapture`
    - Interpretation:
      - `24` is a slightly better retained point than `16`: decodecorpus gets a little smaller again, level 1 stays exact, and the CPU shape stays in the same band.
      - The next useful work remains CPU reductions after the retained long-hash hit, but now from this `24`-byte threshold baseline.
  - Retuned that retained long-hash early-exit threshold upward again from `24` to `32` and kept the new threshold.
    - Rationale:
      - `24` was the best retained point so far, but the threshold itself was still the simplest remaining knob on the long-hash path.
      - This was worth retesting directly against the actual retained `24`-byte release binary instead of assuming the earlier `16`/`24` result had already found the local optimum.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-longhash-skipolder32.csv`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-longhash-skipolder32-l1.csv`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-longhash-skipolder32-repeat.csv`
    - Result versus the retained `24`-byte long-hash baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,675,942 -> 4,675,858`
        - `json_logs_32m.jsonl`: unchanged at `602,826`
        - `repeated_text_32m.txt`: unchanged at `2,874`
        - `xorshift_32m.bin`: unchanged at `33,555,210`
      - Repeat check on the two main fixtures:
        - `decodecorpus_pack.bin`: bytes stayed at `4,675,858`; CPU medians moved from `0.85s` to `0.84s`
        - `json_logs_32m.jsonl`: bytes stayed at `602,826`; CPU medians moved from `0.21s` to `0.22s`
      - Level 1:
        - all fixture bytes stayed identical
        - the two main fixture CPU medians stayed in band:
          - `decodecorpus_pack.bin`: `0.19s -> 0.19s`
          - `json_logs_32m.jsonl`: `0.13s -> 0.13s`
    - Verification:
      - `cargo fmt --all --check`
      - `cargo clippy -q -p ruzstd --lib -- -D warnings`
      - `cargo test -q -p ruzstd match_generator -- --nocapture`
      - `cargo test -q --workspace`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd best_level_external_fixture_round_trips_with_rust_and_c_decoders_from_env -- --ignored --nocapture`
      - `env RUZSTD_BEST_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test --release -q -p ruzstd-cli best_level_progress_monitor_round_trips_external_fixture_from_env -- --ignored --nocapture`
    - Interpretation:
      - `32` is a slightly better retained point than `24`: decodecorpus gets a bit smaller again, the main decodecorpus CPU median stays in the improved band, and level 1 remains exact.
      - JSON CPU is slightly worse in the repeat check, but still in the same narrow band, so this is a reasonable retained move for the current objective.
  - Added test-only diagnostics for retained long-hash overrides and used them to inspect which older-entry paths actually beat a current-entry long-hash candidate.
    - Verification:
      - `env RUZSTD_MATCHER_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin cargo test -q -p ruzstd inspect_best_matcher_from_env -- --ignored --nocapture`
      - `env RUZSTD_MATCHER_FIXTURE=/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/json_logs_32m.jsonl cargo test -q -p ruzstd inspect_best_matcher_from_env -- --ignored --nocapture`
    - Useful result on the retained `24`-byte baseline:
      - `json_logs_32m.jsonl`:
        - `current_long_hash_found = 0`
        - `current_long_hash_overridden = 0`
      - `decodecorpus_pack.bin`:
        - `current_long_hash_found = 172,191`
        - `current_long_hash_overridden = 9,414`
        - override buckets:
          - `current_long_hash_overridden_by_newest[1] = 3,450`
          - `current_long_hash_overridden_by_newest[2] = 586`
          - `current_long_hash_overridden_by_newest[3] = 141`
          - `current_long_hash_overridden_by_oldest[1] = 3,370`
          - `current_long_hash_overridden_by_oldest[2] = 1,313`
          - `current_long_hash_overridden_by_oldest[3] = 554`
          - no observed overrides from `second_newest`
    - Interpretation:
      - Older-entry overrides are relatively rare compared with total long-hash wins, and all observed overrides were from distances `1..=3`.
      - That made an older-entry scan cap look plausible enough to test directly.
  - Tried capping older-entry competition after a long-hash hit to only entry distances `1..=3`. Result: reject.
    - Rationale:
      - The diagnostics showed no observed overrides beyond distance `3`, so this was the narrowest evidence-backed cap to test.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-longhash-entrycap3.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-longhash-entrycap3-l1.md`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-restore-current-longhash3.md`
    - Result versus the retained `24`-byte long-hash baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,675,942 -> 4,676,173`
        - `json_logs_32m.jsonl`: unchanged at `602,826`
        - CPU did not improve enough to justify the byte loss
      - Level 1:
        - bytes stayed identical
      - After reverting and rebuilding the release CLI, the restore spot check returned to the retained baseline:
        - `decodecorpus_pack.bin`: `4,675,942`
        - `json_logs_32m.jsonl`: `602,826`
    - Interpretation:
      - Even though all observed overrides came from `1..=3`, that is not sufficient to make a hard entry-distance cap safe.
      - The remaining CPU work after the retained long-hash hit is not just “stop at the deepest observed winner distance.”
  - Tried a narrower cut from the same diagnostics: when a current-entry long-hash candidate already exists, skip only the older-entry `second_newest` probe while keeping `newest` and `oldest` competition intact. Result: reject.
    - Rationale:
      - The retained long-hash override diagnostics showed no observed older-entry overrides from `second_newest` at all, while `newest` and `oldest` still mattered at entry distances `1..=3`.
      - This was the narrowest remaining way to reduce older-entry work without changing the observed winning paths.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-longhash-nosecondolder.csv`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-longhash-nosecondolder-l1.csv`
    - Result versus the retained `24`-byte long-hash baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: unchanged at `4,675,942`
        - `json_logs_32m.jsonl`: unchanged at `602,826`
        - CPU regressed:
          - `decodecorpus_pack.bin`: `0.80s -> 0.85s`
          - `json_logs_32m.jsonl`: `0.21s -> 0.22s`
      - Level 1:
        - all fixture bytes stayed identical
        - CPU drifted slightly the wrong way on the two main fixtures:
          - `decodecorpus_pack.bin`: `0.18s -> 0.19s`
          - `json_logs_32m.jsonl`: `0.12s -> 0.13s`
    - Interpretation:
      - The older-entry `second_newest` probe is not a measurable CPU cost center by itself on this retained path.
      - The remaining long-hash CPU work is therefore not in “remove one more candidate kind” cuts; it is deeper in the parser/search structure.
  - Tried a more faithful `double_fast`-shaped `ip+1` long-hash promotion on the binary `Best` path: widen the retained next-position window lookahead so it also runs when the current non-repeat candidate is shorter than the long-hash minimum, not only when it is the exact minimum match. Result: reject.
    - Rationale:
      - In C `double_fast`, once a short current-position hit is found, the parser checks the long hash at `ip+1` and promotes it if that longer match wins.
      - The retained Rust path already had an `ip+1` window promotion, but it only ran when the current non-repeat match length was exactly `MIN_MATCH_LEN`. That left an obvious control-flow gap versus C for short-but-not-minimum current matches.
    - Variants tested:
      - current non-repeat match length `< 8`
      - current non-repeat match length `< 7`
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-nextpos-lt8.csv`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-nextpos-lt8-l1.csv`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-nextpos-lt7.csv`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-nextpos-lt7-l1.csv`
    - Result versus the retained `skipolder32` baseline:
      - `< 8` variant:
        - Level 4:
          - `decodecorpus_pack.bin`: `4,675,858 -> 4,675,761`
          - `json_logs_32m.jsonl`: unchanged at `602,826`
          - CPU regressed materially on `decodecorpus_pack.bin`: `0.82s -> 0.88s`
        - Level 1:
          - all fixture bytes stayed identical
          - decodecorpus and JSON CPU drifted slightly better, but not enough to offset the level-4 regression
      - `< 7` variant:
        - Level 4:
          - `decodecorpus_pack.bin`: `4,675,858 -> 4,676,044`
          - `json_logs_32m.jsonl`: unchanged at `602,826`
          - CPU also regressed on `decodecorpus_pack.bin`: `0.81s -> 0.88s`
        - Level 1:
          - all fixture bytes stayed identical
    - Interpretation:
      - The C-shaped gap was real in terms of parser behavior: the broader `< 8` promotion did improve decodecorpus compression.
      - But in the current Rust matcher shape, widening the `ip+1` promotion beyond the exact-minimum case adds too much CPU on the main binary fixture.
      - The remaining binary gap is therefore not just “probe `ip+1` more often”; it needs a cheaper parser/search structure for that promotion.
  - Tried another C-shaped `ip+1` cut on the retained binary path: when a short current-position non-repeat match already exists, replace the full next-position window rescan with a current-entry long-hash-only probe at `ip+1`. Result: reject.
    - Rationale:
      - In C `double_fast`, the `ip+1` promotion after a short current hit is specifically a long-hash check, not a full older-entry competition pass.
      - The retained Rust path still rescanned the full window in that case, so the next obvious experiment was to keep the no-current-candidate path unchanged but narrow the short-current-hit promotion to the current-entry long-hash only.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-nextpos-longhashonly.csv`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-nextpos-longhashonly-l1.csv`
    - Result versus the retained `skipolder32` baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,675,858 -> 4,677,267`
        - `json_logs_32m.jsonl`: unchanged at `602,826`
        - CPU did not improve materially:
          - `decodecorpus_pack.bin`: `0.84s -> 0.87s`
          - `json_logs_32m.jsonl`: stayed `0.21s`
      - Level 1:
        - all fixture bytes stayed identical
        - `decodecorpus_pack.bin` CPU drifted `0.18s -> 0.19s`
    - Interpretation:
      - The older-entry part of the retained `ip+1` promotion is still doing meaningful work even in the short-current-hit case.
      - So a direct “long-hash-only at `ip+1`” analogue is too narrow in the current matcher representation; it gives back too much decodecorpus compression without buying back enough CPU.
  - Tried a narrower byte-stable cleanup inside the retained `ip+1` path: disable only the `second_newest` probe for next-position window promotion, while leaving current-position search unchanged. Result: reject.
    - Rationale:
      - The retained diagnostics on both decodecorpus and JSON showed `window_next_position_second_newest = 0`, so this looked like the narrowest removable piece of the `ip+1` promotion.
      - Because it only touched the next-position helper, it was also expected to be behavior-preserving if those diagnostics were complete.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-nextpos-no2nd.csv`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-nextpos-no2nd-l1.csv`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-nextpos-no2nd-repeat.csv`
    - Result versus the retained `skipolder32` baseline:
      - Full table run:
        - all fixture bytes stayed identical at both levels
        - level-4 CPU was effectively flat on decodecorpus and JSON
      - Repeat check on the two main fixtures:
        - `decodecorpus_pack.bin`: byte-identical; CPU medians stayed flat in the `0.84s` band
        - `json_logs_32m.jsonl`: byte-identical; CPU medians moved slightly the wrong way (`0.21s -> 0.22s`)
        - level 1 moved slightly better on decodecorpus, but stayed in the same general noise band
    - Interpretation:
      - The diagnostics were not enough to justify keeping extra helper structure. Even with exact bytes, the CPU result is at best noise and slightly worse on the main level-4 text fixture.
      - So the retained branch should stay on the simpler shared helper rather than preserving a special next-position no-second-newest path.
  - Tried a `double_fast`-style large-hash maintenance cut on the retained binary path: during no-match skip stepping, auxiliary inserted positions only filled an empty current-entry long-hash slot instead of overwriting it. Result: reject.
    - Rationale:
      - In C `double_fast`, auxiliary positions in the fill step always update the short hash table, but only populate the large-hash table when the large-hash slot is empty.
      - The current Rust long-hash path overwrote the current-entry long-hash slot on every inserted skip position, so this was the closest bounded analogue to C’s selective large-hash maintenance.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-longhash-auxfill.csv`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-longhash-auxfill-l1.csv`
    - Result versus the retained `skipolder32` baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,675,858 -> 4,675,925`
        - `json_logs_32m.jsonl`: unchanged at `602,826`
        - CPU regressed:
          - `decodecorpus_pack.bin`: `0.81s -> 0.86s`
          - `json_logs_32m.jsonl`: `0.21s -> 0.22s`
      - Level 1:
        - all fixture bytes stayed identical
        - `decodecorpus_pack.bin` CPU drifted `0.18s -> 0.19s`
    - Interpretation:
      - This confirms that the retained current-entry long-hash path is sensitive to overwrite recency on skipped positions.
      - The C selective-fill idea does not transfer cleanly into the current Rust representation; in this matcher shape it hurts both bytes and CPU.
  - Retested the previously rejected `16 KiB` current-entry long-hash activation point on top of the newer retained `skipolder32` baseline. Result: reject again.
    - Rationale:
      - Earlier in the branch history, `16 KiB` was rejected before the later retained CPU-side cuts around the long-hash path.
      - It was worth rechecking whether the cheaper current retained path made that earlier threshold viable again.
    - Reports:
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-longhash-16k-retest.csv`
      - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench-manual/zstd-bench-compare-current-longhash-16k-retest-l1.csv`
    - Result versus the retained `skipolder32` baseline:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,675,858 -> 4,675,862`
        - `json_logs_32m.jsonl`: unchanged at `602,826`
        - CPU regressed on decodecorpus:
          - `decodecorpus_pack.bin`: `0.81s -> 0.87s`
      - Level 1:
        - all fixture bytes stayed identical
        - CPU stayed in the same general band
    - Interpretation:
      - The newer retained long-hash path does not rescue the old `16 KiB` activation point.
      - The `32 KiB` activation threshold is still the better retained point for the current parser shape.
  - Conclusion: the obvious local matcher toggles are not enough. The remaining CPU gap needs a more structural `start_matching` improvement, not another boolean gate on the existing parser path.

## Coverage Audit

- Current branch has focused unit tests for matcher suffix candidates, repeat-offset candidate ordering, repeat-history updates, prechecks, sparse indexing, no-match step selection, FSE table selection, FSE `acc_log` caching, and Huffman length/weight invariants.
- Current branch has focused coverage that suffix hash keys stay inside the slot table without a final modulo, including a non-power-of-two capacity.
- Current branch has focused coverage that suffix lookup through a precomputed five-byte key value matches ordinary suffix lookup.
- Current branch has focused coverage that the default driver allocates suffix stores at the retained C-fast hash-table scale.
- Current branch has focused coverage that touched-slot preallocation stays modest and below the full-clear threshold.
- Current branch has focused coverage that same-block match-length scanning with a verified prefix still handles overlapping matches by comparing against the generic relative-window scanner.
- Current branch has focused coverage for the explicit suffix-candidate helper, including best-candidate replacement and non-offset-1 block-end early exit.
- Current branch has focused coverage that unavailable repeat offsets, including zero and offsets before the retained window, are rejected before repeat probing.
- Current branch has exhaustive helper-level coverage for the cached common literal-length and match-length sequence code tables, including the first uncached boundary for each table. This also covers the retained explicit inlining of those helper paths.
- Current branch has helper-level coverage for offset-code generation across the small repeat-code range plus the first uncached boundary from the rejected offset-code cache experiment.
- Current branch's sequence bitstream loop shape is covered by compressed-block tests, end-to-end Rust/C decoder round-trips, and a mixed predefined-mode sequence-section decoder round-trip that uses varied literal-length, match-length, and offset symbols.
- Current branch has focused coverage that the all-RLE sequence encoding fast path preserves additional bits even when equal RLE symbols cover different represented values.
- Current branch has focused coverage for the one-based `NonZeroU32` suffix candidate representation, including index zero.
- Current branch has focused coverage that an empty committed matcher entry emits no sequence and does not prevent processing a following entry.
- Current branch has emitted-bitstream tests that round-trip fastest compression through the Rust decoder and the C zstd decoder, including mixed text/binary/random frames, history reuse after incompressible blocks, and cross-block repetitive data.
- Current branch has focused fastest-level coverage that whole-block RLE emission uses an RLE block header and round-trips through both the Rust decoder and the C zstd decoder.
- Current branch has focused matcher coverage that whole-block RLE history indexes only the extreme suffixes for the repeated key and still matches a following repeated block when repeat offsets are out of range.
- Current branch has emitted-bitstream coverage that the previous-Huffman-table literal threshold allows a small repeated literal payload to use an RLE literal section and round-trip through both the Rust decoder and the C zstd decoder.
- Current branch has emitted-bitstream coverage that a high-alphabet literal payload with no estimated Huffman gain uses raw literals and round-trips through both the Rust decoder and the C zstd decoder.
- Current branch has emitted-bitstream coverage that Huffman-compressed literal payloads below 256 bytes use the single-stream header and round-trip through both the Rust decoder and the C zstd decoder.
- Current branch has Huffman-table helper coverage that previous-table symbol checks can be done from counts, plus emitted-bitstream coverage that a sub-1 KiB treeless repeat-Huffman literal section uses the single-stream header after an earlier block establishes the table and round-trips through both the Rust decoder and the C zstd decoder.
- Current branch has frame-header coverage for 8-byte frame content sizes so large known-size frames serialize a valid descriptor.
- Current branch has frame-level coverage that exact full-block inputs do not emit an extra empty final block and that one-byte EOF lookahead preserves the first byte of the following block.
- Current branch has focused fastest-block coverage that empty direct block inputs emit a valid raw block without indexing byte 0.
- Current branch has focused `Best`-level frame coverage that fully compressible blocks stay whole, mixed blocks with sustained incompressible runs split into multiple blocks and still round-trip through both the Rust decoder and the C zstd decoder, and fully incompressible blocks stay unsplit.
- Current branch has focused top-level `Best` pre-segmentation coverage that two consecutive `8 KiB` incompressible chunks are now recognized as a split-worthy run.
- Current branch has focused top-level `Best` pre-segmentation coverage that even a single `8 KiB` incompressible chunk is now recognized as a split-worthy run.
- Current branch has focused helper coverage that `Best`'s recursive split midpoint follows decompressed-byte balance rather than raw sequence count, including a lopsided-sequence case and an even sequence case.
- Current branch still has the existing encode/decode corpus tests and fuzz targets for encode/decode/FSE/Huff0 interop. These are not a replacement for focused regression tests, but they are useful broad coverage.
- Acceptance rule for future retained changes: add a focused unit/regression test for the changed invariant or an end-to-end Rust+C decode test for emitted-bitstream behavior. If a change is purely a benchmark-only micro-optimization, document why in this file.

## 2026-05-30 - Current long-hash skip-older threshold 40 retained

- Kept a small retune on the retained binary `Best` long-hash early-exit path: `BEST_CURRENT_LONG_HASH_SKIP_OLDER_LEN` is now `40` instead of `32`.
  - Rationale:
    - The retained `32`-byte cut was already a good trade, but the remaining hot path still spends time scanning older entries after a strong current-entry long-hash hit.
    - Raising the threshold was the cheapest remaining structural knob to test before deeper parser work.
  - Reports:
    - `benchmarks/reports/zstd-bench-compare-current-longhash-skipolder40.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-skipolder40-l1.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-skipolder40-repeat-l4.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-skipolder40-repeat-l1.csv`
  - Result versus the retained `skipolder32` baseline:
    - Level 4:
      - `decodecorpus_pack.bin`: `4,675,858 -> 4,675,820`
      - `json_logs_32m.jsonl`: unchanged at `602,826`
      - repeat-run CPU on the main fixtures:
        - `decodecorpus_pack.bin`: `0.89s -> 0.84s`
        - `json_logs_32m.jsonl`: `0.21s -> 0.22s`
    - Level 1:
      - all fixture bytes stayed identical
      - repeat-run CPU on the main fixtures:
        - `decodecorpus_pack.bin`: `0.20s -> 0.19s`
        - `json_logs_32m.jsonl`: stayed `0.13s`
  - Interpretation:
    - This is a better retained point than `32`.
    - It preserves the binary long-hash compression win, slightly improves `decodecorpus` bytes again, and keeps CPU direction favorable enough on the main binary fixture to be worth retaining.
    - The remaining gap is still broader binary parser/search structure after the retained long-hash hit, not another small local gate.

## 2026-05-30 - Current long-hash skip-older threshold 48 retained

- Kept another small retune on the retained binary `Best` long-hash early-exit path: `BEST_CURRENT_LONG_HASH_SKIP_OLDER_LEN` is now `48` instead of `40`.
  - Rationale:
    - The retained `40`-byte cut continued the same favorable trend as `24 -> 32 -> 40`.
    - It was worth checking one more step before moving on to a broader parser/search-structure change.
  - Reports:
    - `benchmarks/reports/zstd-bench-compare-current-longhash-skipolder48.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-skipolder48-l1.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-skipolder48-repeat-l4.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-skipolder48-repeat-l1.csv`
  - Result versus the retained `skipolder40` baseline:
    - Level 4:
      - `decodecorpus_pack.bin`: `4,675,820 -> 4,675,797`
      - `json_logs_32m.jsonl`: unchanged at `602,826`
      - repeat-run CPU on the main fixtures:
        - `decodecorpus_pack.bin`: `0.90s -> 0.84s`
        - `json_logs_32m.jsonl`: stayed `0.22s`
    - Level 1:
      - all fixture bytes stayed identical
      - repeat-run CPU on the main fixtures stayed flat:
        - `decodecorpus_pack.bin`: `0.19s`
        - `json_logs_32m.jsonl`: `0.13s`
  - Interpretation:
    - This is another slightly better retained point than `40`.
    - The compression gain is small, but it is still in the right direction, and the focused repeat check keeps the decodecorpus CPU improvement intact.
    - The next credible move is still broader binary parser/search work after the retained current-entry long-hash hit, not more tiny threshold drift unless the trend breaks.

## 2026-05-30 - Current long-hash skip-older threshold 56 retained

- Kept another small retune on the retained binary `Best` long-hash early-exit path: `BEST_CURRENT_LONG_HASH_SKIP_OLDER_LEN` is now `56` instead of `48`.
  - Rationale:
    - The retained `48`-byte cut still held the same favorable pattern, so it was worth checking one more step before abandoning this line.
    - This is the last cheap threshold move worth taking seriously before switching to a broader binary parser/search change.
  - Reports:
    - `benchmarks/reports/zstd-bench-compare-current-longhash-skipolder56.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-skipolder56-l1.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-skipolder56-repeat-l4.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-skipolder56-repeat-l1.csv`
  - Result versus the retained `skipolder48` baseline:
    - Level 4:
      - `decodecorpus_pack.bin`: `4,675,797 -> 4,675,782`
      - `json_logs_32m.jsonl`: unchanged at `602,826`
      - repeat-run CPU on the main fixtures:
        - `decodecorpus_pack.bin`: `0.91s -> 0.83s`
        - `json_logs_32m.jsonl`: stayed `0.21s`
    - Level 1:
      - all fixture bytes stayed identical
      - repeat-run CPU on the main fixtures stayed flat:
        - `decodecorpus_pack.bin`: `0.18s`
        - `json_logs_32m.jsonl`: `0.13s`
  - Interpretation:
    - This is another slightly better retained point than `48`.
    - The absolute gain is small, but it is still positive on the main binary fixture and does not cost level-1 guardrails in the repeat check.
    - The next move should now be broader binary parser/search structure; if another threshold is tested later, it should justify itself against the diminishing returns shown here.

## 2026-05-30 - Rejected older-entry 8-byte prefix prune after current long-hash hit

- Tried a broader binary `Best` pruning change: once a current-entry long-hash candidate exists, require older-entry window candidates to clear an 8-byte prefix match before paying full match expansion.
  - Rationale:
    - After a retained current-entry long-hash hit, any competing non-repeat candidate must also reach at least 8 bytes to beat or tie it.
    - The intent was to convert many older-entry 5-byte collisions into a cheap reject and reduce the remaining hot path in `best_window_candidate()`.
  - Reports:
    - `benchmarks/reports/zstd-bench-compare-current-longhash-prefix8-prune.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-prefix8-prune-l1.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-prefix8-prune-repeat-l4.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-prefix8-prune-repeat-l1.csv`
    - restore check:
      - `benchmarks/reports/zstd-bench-restore-skipolder56-prefix8-prune.csv`
  - Result versus the retained `skipolder56` baseline:
    - Initial full-table run looked promising:
      - Level 4:
        - `decodecorpus_pack.bin`: `4,675,782 -> 4,675,559`
        - `json_logs_32m.jsonl`: unchanged at `602,826`
        - `decodecorpus_pack.bin` CPU: `1.07s -> 0.91s`
      - Level 1:
        - all bytes stayed identical
        - `decodecorpus_pack.bin` CPU drifted `0.20s -> 0.21s`
    - But the focused repeat check rejected it:
      - Level 4:
        - `decodecorpus_pack.bin`: stayed `4,675,559`
        - `decodecorpus_pack.bin` CPU: `0.97s -> 0.98s`
        - `json_logs_32m.jsonl` CPU improved slightly: `0.23s -> 0.22s`
      - Level 1:
        - repeat-run medians stayed flat
  - Interpretation:
    - The prune is not robust enough on the main binary fixture. It keeps the compression win but fails the decodecorpus CPU guardrail on the focused repeat run.
    - That strongly suggests the older-entry cost is not just cheap 5-byte false positives; some of the extra 8-byte checks and altered expansion path offset the expected win.
    - The next credible move is a different older-entry search structure after the current long-hash hit, not another prefix-gating variation on the same loop.

## 2026-05-30 - Retained distant-newest prune after current long-hash hit

- Kept a broader binary `Best` search-shape change: once a current-entry long-hash candidate exists, older-entry `newest` candidates are only probed for entry distance `1`; for entry distances `>= 2`, only `oldest` remains in competition.
  - Rationale:
    - Retained diagnostics on decodecorpus showed `current_long_hash` overrides only from `newest`/`oldest` at entry distances `1..=3`, with `newest` dropping sharply after distance `1`:
      - `current_long_hash_overridden_by_newest = [0, 3480, 586, 141, ...]`
      - `current_long_hash_overridden_by_oldest = [0, 3421, 1319, 555, ...]`
    - So after a strong current-entry long-hash hit, the costly distant `newest` probes were the first broader older-entry candidate class worth pruning.
  - Reports:
    - `benchmarks/reports/zstd-bench-compare-current-longhash-distant-newest-prune.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-distant-newest-prune-l1.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-distant-newest-prune-repeat-l4.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-distant-newest-prune-repeat-l1.csv`
  - Result versus the retained `skipolder56` baseline:
    - Level 4:
      - `decodecorpus_pack.bin`: `4,675,782 -> 4,675,636`
      - `json_logs_32m.jsonl`: unchanged at `602,826`
      - initial full-table CPU:
        - `decodecorpus_pack.bin`: `0.87s -> 0.84s`
        - `json_logs_32m.jsonl`: `0.22s -> 0.21s`
      - repeat-run CPU on the main fixtures:
        - `decodecorpus_pack.bin`: `0.84s -> 0.82s`
        - `json_logs_32m.jsonl`: stayed `0.21s`
    - Level 1:
      - all fixture bytes stayed identical
      - initial full-table CPU stayed in band
      - repeat-run CPU on the main fixtures stayed flat:
        - `decodecorpus_pack.bin`: `0.19s`
        - `json_logs_32m.jsonl`: `0.13s`
  - Interpretation:
    - This is the first retained broader older-entry search-structure win after the current-entry long-hash hit.
    - Unlike the rejected 8-byte prefix prune, it improved decodecorpus bytes and held the CPU win on the focused repeat check.
    - The next credible move is now more of the same class: prune or reorganize older-entry competition using the override diagnostics, not more tiny threshold tuning.

## 2026-05-30 - Retained entry-distance-1 newest-first override after current long-hash hit

- Kept a narrower binary `Best` probe-order change: when a current-entry long-hash candidate is already active and the older-entry probe is at entry distance `1`, probe `newest` before `oldest`.
  - Rationale:
    - Fresh diagnostics on the retained distant-`newest` baseline showed that the remaining current-long-hash overrides on decodecorpus were concentrated in:
      - `current_long_hash_overridden_by_newest = [0, 3517, 0, ...]`
      - `current_long_hash_overridden_by_oldest = [0, 3446, 1471, 604, ...]`
    - So at entry distance `1`, `newest` was slightly stronger than `oldest`; beyond that, `oldest` still mattered more.
    - This made entry-distance `1` the narrowest credible place to try a probe-order override without reopening the rejected broader distance or prefix gates.
  - Reports:
    - `benchmarks/reports/zstd-bench-compare-current-longhash-entry1-newest-first.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-entry1-newest-first-l1.csv`
    - focused repeat checks:
      - `benchmarks/reports/zstd-bench-compare-current-longhash-entry1-newest-first-repeat-l4.csv`
      - `benchmarks/reports/zstd-bench-compare-current-longhash-entry1-newest-first-repeat-l1.csv`
  - Result versus the retained distant-`newest` baseline:
    - First pass:
      - Level 4:
        - `decodecorpus_pack.bin`: stayed `4,675,636`, CPU `0.91s -> 0.88s`
        - `json_logs_32m.jsonl`: stayed `602,826`, CPU stayed `0.21s`
      - Level 1:
        - all fixture bytes stayed identical
        - `decodecorpus_pack.bin` CPU improved `0.20s -> 0.18s`
    - Focused repeat check:
      - Level 4:
        - `decodecorpus_pack.bin`: stayed `4,675,636`, CPU `0.91s -> 0.84s`
        - `json_logs_32m.jsonl`: stayed `602,826`, CPU stayed `0.22s`
      - Level 1:
        - `decodecorpus_pack.bin`: stayed `5,324,267`, CPU `0.19s -> 0.18s`
        - `json_logs_32m.jsonl`: stayed `690,084`, CPU `0.13s -> 0.12s`
  - Interpretation:
    - This is the first retained probe-order-only win in the binary post-long-hash path.
    - Unlike the rejected distance and prefix cuts, it kept bytes exact against the retained baseline while improving the main decodecorpus CPU path and not hurting JSON.
    - The next credible move remains in the same binary area, but the evidence now points more toward candidate-quality or representation changes than further entry-distance pruning.

## 2026-05-30 - Added broader local benchmark suite and validated the retained baseline

- Added `tools/prepare_benchmark_suites.py` to create reproducible in-repo benchmark suites.
  - Current suite support:
    - `broad-local`: generated text/binary fixtures plus repo-local corpora, source files, and build artifacts
    - `broad-c-zstd`: optional import path for a local C `zstd` checkout when available
- Generated:
  - `benchmarks/fixtures/broad-local`
  - `benchmarks/manifests/broad-local.json`
- Benchmarked the retained entry-distance-1 newest-first baseline against the previous retained distant-newest baseline on the broader local suite.
  - Reports:
    - `benchmarks/reports/zstd-bench-broad-local-entry1-newest-first-l4.md`
    - `benchmarks/reports/zstd-bench-broad-local-entry1-newest-first-l1.md`
  - Result:
    - 32 fixtures total
    - 0 byte changes at level 4
    - 0 byte changes at level 1
    - Level 4 showed several CPU wins and no measured regressions:
      - `build_ruzstd-cli`: `0.23s -> 0.20s`
      - `build_libruzstd.rlib`: `0.12s -> 0.11s`
      - `decodecorpus_z000033`: `0.13s -> 0.12s`
      - `generated_json_logs_001m.jsonl`: `0.02s -> 0.01s`
    - Level 1 stayed flat across the suite at this resolution.
  - Interpretation:
    - This reduces the risk that the retained entry-distance-1 newest-first win is merely overfit to the tiny inner-loop fixture pair.
    - The broader local suite is still not a substitute for curated C-repo fixtures, but it is a better promotion gate than the previous tiny set.
    - Future retained micro-optimizations should be screened on the small inner-loop set and promoted only after this broader suite stays acceptable.

## 2026-05-30 - Re-ran broad-local level 1 on the live tree to test whether compression is now broadly comparable to C

- Re-ran the 32-fixture `broad-local` suite at level 1 on the current source tree with:
  - `target/release/ruzstd-cli` as current
  - `benchmarks/reports/ruzstd-cli-entry1-newest-first-baseline` as upstream
  - C `zstd -1` as the external comparison point
- Reports:
  - `benchmarks/reports/zstd-bench-broad-local-current-l1.md`
  - `benchmarks/reports/zstd-bench-broad-local-current-l1.csv`
- Result versus C on compression:
  - better on 13 fixtures
  - worse on 16 fixtures
  - equal on 3 fixtures
- Biggest wins versus C:
  - `decodecorpus_z000033`: `544,118` vs `571,529`
  - `build_ruzstd-cli`: `870,526` vs `894,099`
  - `build_libruzstd.rlib`: `619,650` vs `635,879`
- Biggest losses versus C:
  - `dict_dictionary.bin`: `25,598` vs `20,145`
  - `repo_match_generator.rs`: `24,884` vs `22,797`
  - `decodecorpus_z000079`: `8,358` vs `7,221`
- Interpretation:
  - Level 1 is not yet broadly compression-comparable to C across the wider local corpus.
  - The current level-1 strengths are still larger binary/build-artifact and generated-JSON style inputs.
  - The current level-1 weaknesses are small dictionary/service/source-style text inputs.
  - This means the branch should not pivot to CPU-only level-1 work yet. The next level-1 move should either close these compression gaps first, or expand the corpus further with a curated C-repo-derived suite before making a CPU-first call.

## 2026-05-30 - Rejected level-1 text classifier minimum `1024 -> 512`

- Tried broadening the level-1 text classification gate so printable blocks start using the text matcher heuristics at `512` bytes instead of `1024`.
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-textclass512-vs1024-main.md`
  - `benchmarks/reports/zstd-bench-compare-level1-textclass512-vs1024-broad-local.md`
  - `benchmarks/reports/zstd-bench-restore-level1-textclass1024-after-textclass512.md`
- Main level-1 guardrails were unchanged:
  - `decodecorpus_pack.bin`: unchanged at `5,323,478`
  - `json_logs_32m.jsonl`: unchanged at `690,084`
- Broad-local movement was too small and net-negative:
  - improvements:
    - `dict_NetworkManager-dispatcher.service`: `400 -> 398`
    - `dict_kmod-static-nodes.service`: `499 -> 496`
  - regressions:
    - `dict_e2scrub_reap.service`: `383 -> 386`
    - `dict_systemd-udev-settle.service`: `572 -> 576`
  - total bytes above C on the fixtures where we still lose moved from `6,578 -> 6,580`
- Decision:
  - Reject and revert.
  - This was too close to noise and moved the broad-local gap slightly the wrong way.
  - The next level-1 text-side experiment should be more selective than another global text-classifier range change.

## 2026-05-30 - Retained level-1 text non-repeat threshold `10 -> 8`

- Captured direct in-process profiles on two representative broad-local level-1 loss cases using the new ignored harness `tests::profile_level1_fixture_from_env`:
  - `benchmarks/reports/perf-level1-repo_match_generator-direct.txt`
  - `benchmarks/reports/perf-level1-dict_dictionary-direct.txt`
- The important signal from both direct profiles is the same:
  - `repo_match_generator.rs`: `MatchGenerator::next_sequence` dominates user-space samples at `62.61%`
  - `dict_dictionary.bin`: `MatchGenerator::next_sequence` dominates user-space samples at `66.32%`
- Based on that, tested a narrow level-1 text-side parser change: lower `TEXT_MIN_NON_REPEAT_MATCH_LEN` from `10` to `8`.
- Main level-1 screen:
  - `benchmarks/reports/zstd-bench-compare-level1-textmin8-main.md`
  - `decodecorpus_pack.bin`: `5,324,267 -> 5,323,478`, CPU `0.18s -> 0.19s`
  - `json_logs_32m.jsonl`: unchanged at `690,084`, CPU unchanged at `0.13s`
- Broad-local level-1 suite:
  - `benchmarks/reports/zstd-bench-compare-level1-textmin8-broad-local.md`
  - better/worse/equal counts vs C stayed `13 / 16 / 3`, so this is not full parity yet
  - but the magnitude of the remaining losses improved materially:
    - total bytes above C on the fixtures where we are still worse: `9,279 -> 6,578`
  - largest improvements:
    - `dict_dictionary.bin`: `25,598 -> 24,237`
    - `repo_match_generator.rs`: `24,884 -> 23,717`
    - `repo_main.rs`: `2,402 -> 2,249`
    - `dict_systemd-logind.service`: `1,206 -> 1,175`
    - `build_ruzstd-cli`: `870,526 -> 867,739`
  - only measured regression:
    - `decodecorpus_z000079`: `8,358 -> 8,372`
- Decision:
  - Keep this change. It is the first level-1 text-path tweak in this line that materially shrinks the broader compression gap without disrupting JSON and while still improving the main decodecorpus fixture.
  - CPU is slightly worse on main decodecorpus, so this is not a CPU win. But it moves the branch closer to compression equivalence on the broader level-1 corpus, which is still the higher-priority gap before a CPU-only pivot.

## 2026-05-30 - Rejected level-1 text non-repeat threshold `8 -> 7`

- Tried a narrower follow-up on the retained text-threshold change: lower `TEXT_MIN_NON_REPEAT_MATCH_LEN` again from `8` to `7`.
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-textmin7-vs8-main.md`
  - `benchmarks/reports/zstd-bench-compare-level1-textmin7-vs8-broad-local.md`
  - `benchmarks/reports/zstd-bench-restore-level1-textmin8-after-textmin7.md`
- Broad-local direction was superficially attractive:
  - `sum(bytes above C on worse fixtures)` improved from `6,578 -> 4,065`
  - more text-side gaps shrank again:
    - `dict_dictionary.bin`: `24,237 -> 22,634`
    - `repo_match_generator.rs`: `23,717 -> 22,876`
    - `repo_main.rs`: `2,249 -> 2,211`
- But it failed the main level-1 guardrail badly:
  - `json_logs_32m.jsonl`: `690,084 -> 809,823`
  - `decodecorpus_pack.bin`: `5,323,478 -> 5,324,210`, CPU `0.19s -> 0.22s`
- Decision:
  - Reject and revert.
  - The threshold-8 retained point appears to be near the safe edge for this heuristic. Going lower starts to change the JSON parse in unacceptable ways.
  - The next level-1 text-side experiment should not be another global threshold drop on `TEXT_MIN_NON_REPEAT_MATCH_LEN`.

## 2026-05-30 - Rejected level-1 text no-match probe step `3 -> 2`

- Tried a denser text-path search by lowering `TEXT_NO_MATCH_PROBE_STEP` from `3` to `2`.
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-textstep2-vs3-main.md`
  - `benchmarks/reports/zstd-bench-compare-level1-textstep2-vs3-broad-local.md`
  - `benchmarks/reports/zstd-bench-restore-level1-textstep3-after-textstep2.md`
- Broad-local direction looked useful on some text fixtures:
  - `dict_dictionary.bin`: `24,237 -> 23,871`
  - `repo_match_generator.rs`: `23,717 -> 22,693`
  - `generated_json_logs_001m.jsonl`: `58,767 -> 57,109`
  - total bytes above C on the worse fixtures improved from `6,578 -> 5,279`
- But it failed the main level-1 guardrail:
  - `json_logs_32m.jsonl`: `690,084 -> 713,323`
  - `decodecorpus_pack.bin`: `5,323,478 -> 5,323,528`, CPU `0.18s -> 0.20s`
- Decision:
  - Reject and revert.
  - The retained threshold-8 baseline stays in place.
  - The next level-1 text-side experiment should not be another blanket increase in text search density.

## 2026-05-30 - Rejected distance-4 oldest prune even after stronger intermediate-improvement diagnostics

- Added stronger test-only diagnostics for the long-hash-active binary search path:
  - `current_long_hash_improved_by_newest`
  - `current_long_hash_improved_by_second_newest`
  - `current_long_hash_improved_by_oldest`
- Fresh decodecorpus inspection on the retained entry-distance-1 newest-first baseline showed:
  - `current_long_hash_improved_by_newest = [0, 3910, 0, 0, ...]`
  - `current_long_hash_improved_by_oldest = [0, 3507, 1485, 604, 0, ...]`
  - So there were no intermediate improvement steps from `oldest` beyond entry distance `3`, not just no final overrides beyond `3`.
- Re-tested the distance-4 `oldest` prune on top of the retained entry-distance-1 newest-first baseline.
  - Reports:
    - `benchmarks/reports/zstd-bench-compare-entry1-newest-first-plus-distant-oldest4-l4.md`
    - `benchmarks/reports/zstd-bench-compare-entry1-newest-first-plus-distant-oldest4-l1.md`
    - restore checks:
      - `benchmarks/reports/zstd-bench-restore-entry1-newest-first-1-l4.md`
      - `benchmarks/reports/zstd-bench-restore-entry1-newest-first-1-l1.md`
  - Result versus the retained entry-distance-1 newest-first baseline:
    - Level 4:
      - `decodecorpus_pack.bin`: `4,675,636 -> 4,675,913`
      - `json_logs_32m.jsonl`: unchanged at `602,826`
      - `decodecorpus_pack.bin` CPU: `0.99s -> 0.85s`
      - `json_logs_32m.jsonl` CPU drifted `0.22s -> 0.23s`
    - Level 1:
      - all fixture bytes stayed identical
      - `decodecorpus_pack.bin` CPU drifted `0.19s -> 0.20s`
  - Interpretation:
    - This is the strongest negative result so far against distance-based `oldest` pruning.
    - Even after proving that `oldest@4+` never matters as an intermediate improvement on decodecorpus in the current retained baseline, removing it still gives back decodecorpus bytes.
    - That strongly suggests the remaining win is not in more entry-distance pruning. The next credible direction should be a different candidate-quality signal or a different post-long-hash search representation entirely.

## 2026-05-30 - Rejected distant-oldest prune after current long-hash hit

- Tried the next older-entry cut in the same family: once a current-entry long-hash candidate exists, skip older-entry `oldest` candidates for entry distances `>= 3`.
  - Rationale:
    - The retained diagnostics after the long-hash hit showed `oldest` overrides still matter strongly at distances `1` and `2`, but fall off again at `3`:
      - `current_long_hash_overridden_by_oldest = [0, 3421, 1319, 555, ...]`
    - So after retaining the distant-`newest` prune, the next plausible structural cut was the distance-3 `oldest` tail.
  - Reports:
    - `benchmarks/reports/zstd-bench-compare-current-longhash-distant-oldest-prune.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-distant-oldest-prune-l1.csv`
    - restore check:
      - `benchmarks/reports/zstd-bench-restore-distant-newest-prune.csv`
  - Result versus the retained distant-`newest` baseline:
    - Level 4:
      - `decodecorpus_pack.bin`: `4,675,636 -> 4,675,684`
      - `json_logs_32m.jsonl`: unchanged at `602,826`
      - `decodecorpus_pack.bin` CPU looked better on the first pass: `1.05s -> 0.82s`
    - Level 1:
      - all fixture bytes stayed identical
      - `decodecorpus_pack.bin` CPU drifted `0.19s -> 0.21s`
  - Interpretation:
    - This is not a keep. It gives back decodecorpus compression immediately and hurts the level-1 guardrail.
    - The current retained older-entry competition still needs `oldest` at distance `3` often enough that pruning it is too aggressive.
    - The next credible move should stay in the “diagnostic-guided search shape” family, but it needs to be more selective than a blanket distance-3 `oldest` cut.

## 2026-05-30 - Rejected selective distant-oldest prune gated by strong long-hash

- Tried a narrower variant of the rejected distance-3 `oldest` cut: after a current-entry long-hash hit of at least 16 bytes, skip older-entry `oldest` candidates for entry distances `>= 3`.
  - Rationale:
    - The blanket distance-3 `oldest` prune was too aggressive.
    - The next plausible refinement was to keep distance-3 `oldest` for weaker long-hash hits, but drop it once the current-entry long-hash match was already clearly strong.
  - Reports:
    - `benchmarks/reports/zstd-bench-compare-current-longhash-selective-distant-oldest-prune.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-selective-distant-oldest-prune-l1.csv`
    - restore check:
      - `benchmarks/reports/zstd-bench-restore-distant-newest-prune-2.csv`
  - Result versus the retained distant-`newest` baseline:
    - Level 4:
      - `decodecorpus_pack.bin`: `4,675,636 -> 4,675,683`
      - `json_logs_32m.jsonl`: unchanged at `602,826`
      - `decodecorpus_pack.bin` CPU: `0.86s -> 0.85s`
    - Level 1:
      - all fixture bytes stayed identical
      - `decodecorpus_pack.bin` CPU drifted `0.19s -> 0.20s`
  - Interpretation:
    - This is still not selective enough. It gives back decodecorpus compression immediately and still moves the level-1 decodecorpus guardrail the wrong way.
    - So the remaining useful cuts in this family are probably not another simple “distance >= N” gate on `oldest`, even when conditioned on current long-hash length.

## 2026-05-30 - Rejected zero-literal selective distant-oldest prune

- Tried an even narrower variant of the distance-3 `oldest` cut: after a current-entry long-hash hit of at least 16 bytes with zero literals, skip older-entry `oldest` candidates for entry distances `>= 3`.
  - Rationale:
    - Retained diagnostics showed current-entry long-hash wins are mostly zero-literal.
    - The hope was that distance-3 `oldest` only becomes disposable on that narrower, stronger binary path, while literal-carrying long-hash hits would keep the old behavior.
  - Reports:
    - `benchmarks/reports/zstd-bench-compare-current-longhash-selective-distant-oldest-prune.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-selective-distant-oldest-prune-l1.csv`
    - restore check:
      - `benchmarks/reports/zstd-bench-restore-distant-newest-prune-3.csv`
  - Result versus the retained distant-`newest` baseline:
    - Level 4:
      - `decodecorpus_pack.bin`: `4,675,636 -> 4,675,663`
      - `json_logs_32m.jsonl`: unchanged at `602,826`
      - `decodecorpus_pack.bin` CPU: `0.86s -> 0.85s`
    - Level 1:
      - all fixture bytes stayed identical
      - `decodecorpus_pack.bin` CPU: stayed `0.20s` in that run
  - Interpretation:
    - This is still not a keep. It gives back decodecorpus compression immediately.
    - Zero-literal gating alone is not enough to make the distance-3 `oldest` cut safe.
    - The next useful move in the diagnostic-guided family is likely candidate-quality or probe-order based, not another distance gate layered on `oldest`.

## 2026-05-30 - Rejected distant-oldest prefix-6 gate after current long-hash hit

- Tried a more selective older-entry quality filter: after a current-entry long-hash hit, older-entry `oldest` candidates at entry distances `>= 3` were only expanded if they matched a 6-byte prefix first.
  - Rationale:
    - The blanket and selectively gated distance-3 `oldest` cuts were too aggressive.
    - The next narrower attempt was to keep distant `oldest` structurally present, but prune the weaker ones before paying full match expansion.
  - Reports:
    - `benchmarks/reports/zstd-bench-compare-current-longhash-distant-oldest-prefix6.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-distant-oldest-prefix6-l1.csv`
    - focused repeat check:
      - `benchmarks/reports/zstd-bench-compare-current-longhash-distant-oldest-prefix6-repeat-l4.csv`
    - restore checks:
      - `benchmarks/reports/zstd-bench-restore-distant-newest-prune-4.csv`
      - `benchmarks/reports/zstd-bench-restore-distant-newest-prune-4-l1.csv`
  - Result versus the retained distant-`newest` baseline:
    - First pass:
      - Level 4:
        - `decodecorpus_pack.bin`: stayed `4,675,636`, CPU `0.89s -> 0.84s`
        - `json_logs_32m.jsonl`: stayed `602,826`, CPU stayed `0.21s`
      - Level 1:
        - all fixture bytes stayed identical
        - `json_logs_32m.jsonl` CPU improved slightly: `0.13s -> 0.12s`
    - Focused repeat check:
      - Level 4:
        - `decodecorpus_pack.bin`: stayed `4,675,636`, but CPU regressed `0.86s -> 0.96s`
        - `json_logs_32m.jsonl`: stayed `602,826`, but CPU drifted `0.25s -> 0.26s`
  - Interpretation:
    - This is not stable enough to keep. The first full-table run looked promising, but the main binary repeat check failed hard on CPU while preserving bytes.
    - So the useful search-shape direction remains “selective older-entry competition after the long-hash hit,” but this specific prefix gate is too noisy.
    - The next credible move is likely candidate-quality or probe-order based inside the retained older-entry path, not another prefix-length gate on the same distant `oldest` branch.

## 2026-05-30 - Rejected newest@1-gated distant-oldest prune after current long-hash hit

- Tried a narrower probe-shape cut: after a current-entry long-hash hit, skip older-entry `oldest` candidates at entry distances `>= 3` only after `newest` at entry distance `1` has already improved the candidate.
  - Rationale:
    - Retained diagnostics on decodecorpus showed that long-hash overrides come from `newest@1` and `oldest@1..3`, with `newest@1` slightly ahead of `oldest@1`.
    - The idea was to keep the retained distant-`newest` prune, then weaken the distant `oldest` tail only once the strongest nearby `newest` override had already happened.
  - Reports:
    - `benchmarks/reports/zstd-bench-compare-current-longhash-newest1-gates-oldest.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-newest1-gates-oldest-l1.csv`
    - restore checks:
      - `benchmarks/reports/zstd-bench-restore-distant-newest-prune-5.csv`
      - `benchmarks/reports/zstd-bench-restore-distant-newest-prune-5-l1.csv`
  - Result versus the retained distant-`newest` baseline:
    - Level 4:
      - `decodecorpus_pack.bin`: `4,675,636 -> 4,675,688`
      - `json_logs_32m.jsonl`: unchanged at `602,826`
      - `decodecorpus_pack.bin` CPU: `0.91s -> 0.90s`
    - Level 1:
      - all fixture bytes stayed identical
      - `decodecorpus_pack.bin` CPU drifted `0.20s -> 0.21s`
  - Interpretation:
    - This is not a keep. The selectivity was better than the earlier blanket distance gates, but it still gave back decodecorpus bytes immediately and nudged the level-1 decodecorpus guardrail the wrong way.
    - That suggests the remaining useful cuts are not just “distance-3 oldest after some earlier event”; they likely need a more direct candidate-quality signal or a different post-long-hash search representation.

## 2026-05-30 - Rejected distance-4 distant-oldest prune after current long-hash hit

- Tried the narrowest pure distance cut in this family: after a current-entry long-hash hit, skip older-entry `oldest` candidates only at entry distances `>= 4`.
  - Rationale:
    - Fresh diagnostics on the retained distant-`newest` baseline showed no current-long-hash overrides from `oldest` beyond entry distance `3`:
      - `current_long_hash_overridden_by_oldest = [0, 3446, 1471, 604, 0, ...]`
    - So this was the first distance gate directly supported by the current data rather than an inferred heuristic.
  - Reports:
    - `benchmarks/reports/zstd-bench-compare-current-longhash-distant-oldest-prune4.csv`
    - `benchmarks/reports/zstd-bench-compare-current-longhash-distant-oldest-prune4-l1.csv`
    - restore checks:
      - `benchmarks/reports/zstd-bench-restore-distant-newest-prune-6.md`
      - `benchmarks/reports/zstd-bench-restore-distant-newest-prune-6-l1.md`
  - Result versus the retained distant-`newest` baseline:
    - Level 4:
      - `decodecorpus_pack.bin`: `4,675,636 -> 4,675,913`
      - `json_logs_32m.jsonl`: unchanged at `602,826`
      - `decodecorpus_pack.bin` CPU: stayed `0.89s`
      - `json_logs_32m.jsonl` CPU drifted `0.21s -> 0.22s`
    - Level 1:
      - all fixture bytes stayed identical
      - `decodecorpus_pack.bin` CPU drifted `0.19s -> 0.20s`
  - Interpretation:
    - This is still not a keep. Even the first distance cut that matched the override buckets exactly gave back decodecorpus bytes and did not buy CPU.
    - That is strong evidence that the remaining binary-path win is not another distance gate on `oldest`, even at the exact observed boundary.
    - The next credible move should shift away from entry-distance pruning and toward a different candidate-quality signal or a different post-long-hash search representation.

## 2026-05-30 - Rejected level-1 binary oldest-first window probing for non-text blocks

- Tried a selective level-1 binary-path probe-order experiment in `MatchGenerator::best_window_candidate()`:
  - keep the retained short-line text code/config split unchanged
  - for `CompressionLevel::Fastest`, use oldest-first window probing only on non-text blocks
- Rationale:
  - the retained text path is already heavily refined and indexed
  - the largest remaining broad-local losses include binary decodecorpus fixtures such as `decodecorpus_z000079`
  - this was the cheapest way to test whether a `Best`-style older-first search order helps level-1 binary blocks without disturbing the retained text wins
- Reports:
  - broad-local:
    - `benchmarks/reports/zstd-bench-compare-level1-fastest-binary-oldestfirst-broad-local.md`
    - `benchmarks/reports/zstd-bench-compare-level1-fastest-binary-oldestfirst-broad-local.csv`
  - focused repeat:
    - `benchmarks/reports/zstd-bench-compare-level1-fastest-binary-oldestfirst-repeat.md`
    - `benchmarks/reports/zstd-bench-compare-level1-fastest-binary-oldestfirst-repeat.csv`
- Result versus the retained code/config level-1 baseline:
  - broad-local bytes were identical on all 32 fixtures
  - the first pass showed only two possible CPU wins:
    - `build_ruzstd-cli`: `0.05s -> 0.04s`
    - `decodecorpus_z000033`: `0.02s -> 0.01s`
  - focused repeat check on exactly those two fixtures collapsed back to flat:
    - `build_ruzstd-cli`: `0.04s -> 0.04s`
    - `decodecorpus_z000033`: `0.02s -> 0.02s`
  - Interpretation:
    - This is not a retained win. It is byte-stable, but the apparent CPU improvement was only timing noise.
    - That closes the simple “Fastest binary oldest-first probe order” variant.
    - The next credible level-1 binary move needs a stronger signal or a different representation, not just swapping newest/oldest probe order in the current window search.

## 2026-05-30 - Retained level-1 binary `ip+1` repeat lookahead for non-text blocks

- Tried the next narrower `double_fast`-shaped adjacent-position experiment on the level-1 binary path:
  - keep the retained short-line text code/config split unchanged
  - for `CompressionLevel::Fastest`, enable `ip+1` repeat lookahead only on non-text blocks
- Rationale:
  - the simple oldest-first probe-order cut was a no-op on bytes and only produced CPU noise
  - the remaining broad-local binary losses, especially `decodecorpus_z000079`, are more plausibly about missing adjacent-position repeat promotions than about current-position window probe order
- Reports:
  - main screen:
    - `benchmarks/reports/zstd-bench-compare-level1-fastest-binary-nextrep-main.md`
    - repeat check:
      - `benchmarks/reports/zstd-bench-compare-level1-fastest-binary-nextrep-main-repeat.md`
  - broad-local:
    - `benchmarks/reports/zstd-bench-compare-level1-fastest-binary-nextrep-broad-local.md`
    - focused binary repeat:
      - `benchmarks/reports/zstd-bench-compare-level1-fastest-binary-nextrep-repeat.md`
  - retained binary:
    - `benchmarks/reports/ruzstd-cli-level1-fastest-binary-nextrep-retained`
- Result versus the retained code/config level-1 baseline:
  - Main:
    - `decodecorpus_pack.bin`: `5,323,478 -> 5,319,265`
    - `json_logs_32m.jsonl`: unchanged at `690,084`
    - `repeated_text_32m.txt`: unchanged at `2,874`
    - `xorshift_32m.bin`: unchanged at `33,555,210`
  - Main repeat check confirmed the same bytes exactly, with `decodecorpus_pack.bin` CPU drifting from `0.18s` to `0.20s`
  - Broad-local:
    - better / worse / equal vs C stayed `14 / 15 / 3`
    - total bytes above C on fixtures where we still lose improved `1,909 -> 1,073`
    - biggest wins:
      - `build_libruzstd.rlib`: `619,650 -> 611,155`
      - `build_ruzstd-cli`: `867,739 -> 860,072`
      - `decodecorpus_z000079`: `8,372 -> 7,540`
- Interpretation:
  - This is a real retained compression win on the level-1 binary path.
  - It is not a CPU win; the main binary guardrail and one broad-local build artifact drift slightly slower.
  - That is still acceptable at the current phase because the branch is not yet broadly compression-equivalent to C, and this materially closes that gap without reopening JSON.

## 2026-05-30 - Rejected three Fastest non-text follow-ups on top of the retained binary `ip+1` repeat win

- Added test-only diagnostics to split retained `RepeatNextPosition` wins by reason:
  - `NoCurrentCandidate`
  - `BeatsCurrentMinNonRepeat`
- Added an ignored `inspect_fastest_matcher_from_env` test so we can inspect Fastest-level matcher diagnostics on real fixtures.
- Diagnostic result on both:
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/system-tmp/zstd-bench/fixtures/decodecorpus_pack.bin`
  - `benchmarks/fixtures/broad-local/build_ruzstd-cli`
  - every retained `RepeatNextPosition` win came from `NoCurrentCandidate`
  - none came from `BeatsCurrentMinNonRepeat`

- Follow-up 1: zero-literal gate on retained Fastest non-text `ip+1` repeat lookahead
  - Reports:
    - `benchmarks/reports/zstd-bench-compare-level1-fastest-binary-nextrep-zerolit-main.md`
    - `benchmarks/reports/zstd-bench-compare-level1-fastest-binary-nextrep-zerolit-broad-local.md`
  - Result:
    - rejected
    - main regression:
      - `decodecorpus_pack.bin`: `5,319,265 -> 5,321,178`
    - broad-local regressions:
      - `build_libruzstd.rlib`: `611,155 -> 618,065`
      - `build_ruzstd-cli`: `860,072 -> 866,916`
      - `decodecorpus_z000079`: `7,540 -> 8,376`
  - Interpretation:
    - final zero-literal `RepeatNextPosition` counts were not a safe gating signal
    - zero-literal probes still perturb the eventual parse enough to matter

- Follow-up 2: Fastest non-text `ip+1` window lookahead
  - Reports:
    - `benchmarks/reports/zstd-bench-compare-level1-fastest-binary-nextwindow-main.md`
    - `benchmarks/reports/zstd-bench-compare-level1-fastest-binary-nextwindow-broad-local.md`
  - Result:
    - complete no-op on both main and broad-local
  - Interpretation:
    - enabling the existing next-position window helper for Fastest non-text blocks does not move the retained level-1 binary baseline in any meaningful way

- Follow-up 3: dedicated Fastest non-text parser loop
  - Reports:
    - `benchmarks/reports/zstd-bench-compare-level1-fastbinary-split-main.md`
    - `benchmarks/reports/zstd-bench-compare-level1-fastbinary-split-broad-local.md`
  - Result:
    - rejected
    - byte-identical, but no CPU win
    - broad-local even nudged `build_ruzstd-cli` from `0.04s` to `0.05s`
  - Interpretation:
    - simply splitting the Fastest non-text loop shape is not enough
    - the next credible move needs a stronger representation or search-structure change than this path split

## 2026-05-30 - Rejected Fastest current-entry second-newest candidate on the level-1 binary path

- Tried a broader compression-oriented current-entry representation change for large Fastest non-text blocks:
  - reuse the existing current-entry `second_newest` sidecar machinery on the level-1 binary path
  - first as the full current-entry extra candidate
  - then narrowed to only probe that extra candidate when no candidate existed yet
- Reports:
  - full version:
    - `benchmarks/reports/zstd-bench-compare-level1-fastest-secondnewest-main.md`
    - `benchmarks/reports/zstd-bench-compare-level1-fastest-secondnewest-broad-local.md`
  - narrowed no-candidate version:
    - `benchmarks/reports/zstd-bench-compare-level1-fastest-secondnewest-nocand-main.md`
- Result:
  - strong compression signal:
    - main:
      - `decodecorpus_pack.bin`: `5,319,265 -> 5,259,216`
    - broad-local:
      - `build_libruzstd.rlib`: `611,155 -> 609,561`
      - `build_ruzstd-cli`: `860,072 -> 856,479`
      - `decodecorpus_z000033`: `544,266 -> 532,424`
  - CPU cost was too large:
    - `decodecorpus_pack.bin`: `0.21s -> 0.25s`
    - `build_libruzstd.rlib`: `0.03s -> 0.04s`
    - `build_ruzstd-cli`: `0.05s -> 0.06s`
  - narrowing it to the “no candidate yet” case did not reduce the cost or the compression result on the main binary guardrail
- Interpretation:
  - this is a real higher-compression direction for the level-1 binary path
  - but the current sidecar representation is still too expensive
  - the next credible move, if we revisit this family, needs a cheaper current-entry representation rather than another probe gate on the same sidecar

## 2026-05-30 - Rejected Fastest no-candidate current-entry long-hash gate on the level-1 binary path

- Tried a narrower follow-up to the rejected Fastest current-entry long-hash path:
  - enable the current-entry long-hash candidate for large Fastest non-text blocks
  - but only probe it when the current position has no candidate yet
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-fastest-longhash-nocand-main.md`
  - `benchmarks/reports/zstd-bench-compare-level1-fastest-longhash-nocand-broad-local.md`
  - restore:
    - `benchmarks/reports/zstd-bench-restore-level1-fastest-binary-nextrep-after-longhash-nocand.md`
- Result:
  - main guardrail improved again:
    - `decodecorpus_pack.bin`: `5,319,265 -> 5,301,816`
  - but CPU still regressed:
    - `decodecorpus_pack.bin`: `0.20s -> 0.24s`
  - broad-local regressed overall:
    - bytes-above-C on worse fixtures: `1,073 -> 1,147`
    - `build_libruzstd.rlib`: `611,155 -> 611,997`
    - `build_ruzstd-cli`: `860,072 -> 860,496`
    - `decodecorpus_z000079`: `7,540 -> 7,614`
- Interpretation:
  - this closes off the obvious “only use long-hash when the current position is empty” gate
  - the remaining useful binary-side move is not another local current-entry long-hash condition on the same representation

## 2026-05-30 - Rejected Fastest no-candidate 4-byte current-entry hash on the level-1 binary path

- Tried a different current-entry representation than the rejected long-hash family:
  - add a Fastest-only current-entry 4-byte hash for non-text blocks
  - only probe it when the current position has no candidate
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-fastest-shorthash-nocand-main.md`
  - `benchmarks/reports/zstd-bench-compare-level1-fastest-shorthash-nocand-broad-local.md`
  - restore:
    - `benchmarks/reports/zstd-bench-restore-level1-fastest-binary-nextrep-after-shorthash-nocand.md`
- Result:
  - strong main guardrail improvement:
    - `decodecorpus_pack.bin`: `5,319,265 -> 5,282,033`
  - CPU still regressed:
    - `decodecorpus_pack.bin`: `0.20s -> 0.23s`
  - broad-local did not improve overall:
    - bytes-above-C on worse fixtures: `1,073 -> 1,098`
    - wins:
      - `build_ruzstd-cli`: `860,072 -> 855,340`
      - `build_libruzstd.rlib`: `611,155 -> 608,526`
      - `decodecorpus_z000028`: `100,250 -> 98,140`
      - `decodecorpus_z000033`: `544,266 -> 541,477`
    - but remaining losers still moved the wrong way:
      - `decodecorpus_z000079`: `7,540 -> 7,565`
      - `dict_dictionary.bin`: unchanged at `20,667`
- Interpretation:
  - this closes off the obvious C-shaped short-hash current-entry representation in its simplest no-candidate form
  - the next credible move is not another local current-entry hash enable on the same no-candidate rule

## 2026-05-30 - Retained Fastest small-block current-entry second-newest probe on the level-1 binary path

- Tried a narrow same-block binary-path search improvement on top of the retained Fastest non-text `ip+1` repeat baseline:
  - non-text blocks only
  - block size up to `64 KiB`
  - probe current-entry `second_newest` only when the current position has no candidate
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-fastest-small-secondnewest-main.md`
  - `benchmarks/reports/zstd-bench-compare-level1-fastest-small-secondnewest-main-repeat.md`
  - `benchmarks/reports/zstd-bench-compare-level1-fastest-small-secondnewest-broad-local.md`
  - retained binary:
    - `benchmarks/reports/ruzstd-cli-level1-fastest-small-secondnewest-retained`
- Result:
  - main level-1 guardrails stayed exact:
    - `decodecorpus_pack.bin`: `5,319,265`
    - `json_logs_32m.jsonl`: `690,084`
  - repeat run kept CPU in the same band:
    - `decodecorpus_pack.bin`: `0.20s -> 0.21s`
    - `json_logs_32m.jsonl`: `0.16s -> 0.16s`
  - broad-local gap vs C improved:
    - better / worse / equal stayed `14 / 15 / 3`
    - bytes-above-C on worse fixtures: `1,073 -> 1,022`
  - notable wins:
    - `decodecorpus_z000030`: `13,545 -> 13,463`
    - `decodecorpus_z000054`: `9,756 -> 9,628`
    - `decodecorpus_z000059`: `717 -> 702`
    - `decodecorpus_z000080`: `2,669 -> 2,635`
    - `dict_NetworkManager-dispatcher.service`: `398 -> 396`
- Interpretation:
  - this is a modest but real retained level-1 binary compression step
  - it does not solve the remaining big losers, but it proves a narrower same-block extra-candidate path can pay its way when kept away from the large-block binary path

## 2026-05-31 - Rejected Fastest small-block oldest-first current-window probing on the level-1 binary path

- Tried a probe-order-only follow-up to the retained small-block current-entry `second_newest` baseline:
  - Fastest non-text blocks only
  - block size up to `64 KiB`
  - only when the current position has no candidate
  - prefer `oldest` before `newest` in current-window probing
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-fastest-small-oldestfirst-main.md`
  - `benchmarks/reports/zstd-bench-compare-level1-fastest-small-oldestfirst-broad-local.md`
  - restore:
    - `benchmarks/reports/zstd-bench-restore-level1-fastest-small-secondnewest-after-oldestfirst.md`
- Result:
  - rejected
  - byte-identical on both main and broad-local
  - main CPU moved the wrong way:
    - `decodecorpus_pack.bin`: `0.22s -> 0.27s`
    - `json_logs_32m.jsonl`: `0.17s -> 0.22s`
- Interpretation:
  - this closes off the small-block no-candidate oldest-first probe-order variant
  - the remaining move for `dict_dictionary.bin` is not another current-window order tweak; it needs a different current-block signal or representation

## 2026-05-31 - Retained dense no-match probing for small Fastest non-text blocks

- Tried a narrow binary-path search-density follow-up on top of the retained Fastest non-text `ip+1` repeat and small-block `second_newest` baseline:
  - Fastest non-text blocks only
  - block size up to `64 KiB`
  - when no candidate exists yet, probe byte-by-byte instead of stepping by `2`
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-fastest-dense-smallbin-main.md`
  - `benchmarks/reports/zstd-bench-compare-level1-fastest-dense-smallbin-main-repeat.md`
  - `benchmarks/reports/zstd-bench-compare-level1-fastest-dense-smallbin-broad-local.md`
  - retained binary:
    - `benchmarks/reports/ruzstd-cli-level1-fastest-dense-smallbin-retained`
- Result:
  - retained
  - main level-1 guardrails stayed exact:
    - `decodecorpus_pack.bin`: `5,319,265`
    - `json_logs_32m.jsonl`: `690,084`
  - repeat run kept main CPU in the same band:
    - `decodecorpus_pack.bin`: `0.22s -> 0.21s`
    - `json_logs_32m.jsonl`: `0.16s -> 0.16s`
  - broad-local gap vs C improved:
    - better / worse / equal: `14 / 15 / 3 -> 15 / 14 / 3`
    - bytes-above-C on worse fixtures: `1,022 -> 1,005`
  - notable wins:
    - `decodecorpus_z000030`: `13,463 -> 13,152`
    - `decodecorpus_z000031`: `116 -> 112`
    - `decodecorpus_z000054`: `9,628 -> 9,567`
    - `decodecorpus_z000080`: `2,635 -> 2,603`
  - notable non-moves / losses:
    - `dict_dictionary.bin`: unchanged at `20,667`
    - `decodecorpus_z000079`: unchanged at `7,540`
    - `decodecorpus_z000059`: `702 -> 711`
    - `dict_fstrim.service`: `304 -> 312`
- Interpretation:
  - this is a modest but real retained level-1 binary compression step
  - it improves the broader suite without disturbing the main guardrails or the CPU band
  - it does not address the two stubborn remaining binary losers, so the next move still needs a different signal or representation for those cases

## 2026-05-31 - Rejected weak-current widening of the Fastest small-block `second_newest` probe

- Tried the narrowest current-candidate follow-up to the retained small-block `second_newest` baseline:
  - keep the same Fastest non-text block-size gate (`<= 64 KiB`)
  - keep the same current-entry sidecar
  - but allow `second_newest` to compete when the current candidate is a weak minimum-length non-repeat match, not just when there is no candidate
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-fastest-small-secondnewest-weakcurrent-main.md`
  - `benchmarks/reports/zstd-bench-compare-level1-fastest-small-secondnewest-weakcurrent-broad-local.md`
- Result:
  - rejected
  - complete no-op on both suites
  - main guardrails stayed exact:
    - `decodecorpus_pack.bin`: `5,319,265`
    - `json_logs_32m.jsonl`: `690,084`
  - broad-local stayed exact:
    - better / worse / equal vs C: `15 / 14 / 3`
    - bytes-above-C on worse fixtures: `1,005`
- Interpretation:
  - this closes off the obvious “weak current min-length non-repeat” widening of the retained small-block `second_newest` path
  - the remaining binary gap is not waiting on another local expansion of this sidecar condition

## 2026-05-31 - Rejected compressibility-gated dense probing for Fastest non-text blocks

- Tried a broader conditioning signal for the retained dense-probe family:
  - keep text excluded
  - keep xorshift-like incompressible blocks excluded
  - but use byte-by-byte no-match probing for all compressible Fastest non-text blocks, not just blocks up to `64 KiB`
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-fastest-dense-compressible-main.md`
  - `benchmarks/reports/zstd-bench-compare-level1-fastest-dense-compressible-broad-local.md`
- Result:
  - rejected
  - strong compression signal:
    - `decodecorpus_pack.bin`: `5,319,265 -> 5,219,513`
    - `build_libruzstd.rlib`: `611,155 -> 600,329`
    - `build_ruzstd-cli`: `860,072 -> 846,556`
    - `decodecorpus_z000033`: `544,266 -> 533,010`
  - but main CPU regressed far too much:
    - `decodecorpus_pack.bin`: `0.21s -> 0.28s`
  - and broad-local total moved the wrong way because one stubborn loser regressed:
    - `decodecorpus_z000079`: `7,540 -> 7,565`
    - bytes-above-C on worse fixtures: `1,005 -> 1,030`
- Interpretation:
  - this is a real higher-compression direction for the level-1 binary path
  - but it is not retainable as a broad “compressible non-text everywhere” gate
  - the next credible move, if we stay in this family, must be more selective than block compressibility alone

## 2026-05-31 - Rejected C-shaped `ip + step` repeat probe for Fastest non-text blocks

- Went back to the original C `ZSTD_fast` path in `lib/compress/zstd_fast.c` and tried the closest narrow analogue:
  - for Fastest non-text blocks
  - only when the current position has no candidate
  - probe repeat at `ip + step` instead of only the retained `ip + 1` lookahead
- Report:
  - `benchmarks/reports/zstd-bench-compare-level1-fastest-step-repeat-main.md`
- Result:
  - rejected immediately on the main guardrail
  - `decodecorpus_pack.bin`: `5,319,265 -> 5,330,232`
  - CPU: `0.22s -> 0.26s`
- Interpretation:
  - this specific `ZSTD_fast` adjacent-position repeat shape does not transfer cleanly into the current Rust matcher representation
  - the useful C hint was real, but not this literal step-probe analogue

## 2026-05-31 - Retained small CodeText/ConfigText exact-Huffman literal search as a level-1 file-type starting point

- Kept the matcher path unchanged.
- Broadened exact Huffman table search only for:
  - `CompressionFileType::CodeText`
  - `CompressionFileType::ConfigText`
- Shape:
  - `DictionaryText` still searches all literal sections exactly
  - `CodeText` / `ConfigText` now search exactly when the literal section is small (`<= 4 KiB`)
  - `JsonText` stays on the retained long-line heuristic path
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-code-config-smallhuff-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-code-config-smallhuff-mainish.md`
  - retained binary:
    - `benchmarks/reports/ruzstd-cli-level1-filetype-smallhuff-retained`
- Result versus the retained dense-smallbin baseline:
  - retained
  - broad-local improved:
    - better / worse / equal vs C: `15 / 14 / 3 -> 16 / 13 / 3`
    - bytes-above-C on worse fixtures: `1,005 -> 984`
  - wins:
    - `dict_NetworkManager-dispatcher.service`: `395 -> 391`
    - `dict_fstrim.service`: `312 -> 308`
    - `dict_ftpd.service`: `172 -> 168`
    - `dict_netctl@.service`: `212 -> 206`
    - `dict_systemd-coredump@.service`: `692 -> 690`
    - `dict_systemd-udev-settle.service`: `569 -> 568`
    - `repo_main.rs`: `2,141 -> 2,137`
    - `repo_match_generator.rs`: `22,591 -> 22,587`
  - unchanged guardrails:
    - `build_libruzstd.rlib`: `611,155`
    - `build_ruzstd-cli`: `860,072`
    - `dict_dictionary.bin`: `20,667`
    - `generated_json_logs_001m.jsonl`: `58,767`
- Interpretation:
  - this is the first retained file-type starting-point family beyond the dictionary-specific literal policy
  - the useful extension/path hint here is on block entropy policy, not the matcher
  - next file-type experiments should keep exploiting that separation instead of reopening the shared level-1 matcher path immediately

## 2026-05-31 - Rejected follow-up entropy and framing experiments after the retained CodeText/ConfigText small-literal Huffman win

- Tried four follow-ups:
  - widen `CodeText` / `ConfigText` exact-Huffman search from small literal sections to all literal sections
  - raise Fastest repeat-table reuse threshold from `64` to `2048`
  - split long trailing single-byte runs into separate Fastest RLE blocks
  - use the known CLI file size to emit a single-segment frame header
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-code-config-allhuff-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-fse-repeat2048-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-trailing-rle-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-fcs-broad-local.md`
- Results:
  - `CodeText` / `ConfigText` all-literal exact-Huffman:
    - exact no-op versus the retained small-literal policy
  - repeat-table reuse `64 -> 2048`:
    - exact no-op on bytes across both focused and broad-local suites
  - trailing-RLE split:
    - `decodecorpus_z000079`: `7,540 -> 8,338`
    - broad-local bytes-above-C on worse fixtures: `984 -> 1,782`
  - CLI frame-content-size single-segment hint:
    - broadly made outputs 1 to 3 bytes larger
    - examples:
      - `dict_dictionary.bin`: `20,667 -> 20,668`
      - `repo_main.rs`: `2,137 -> 2,138`
      - `build_ruzstd-cli`: `860,072 -> 860,075`
- Interpretation:
  - the small-literal file-type Huffman policy is the useful retained point; broadening it further does not buy more
  - the remaining big losers are not waiting on simple entropy-table reuse or framing-header changes
  - `decodecorpus_z000079` also rejects the obvious C-inspired trailing-RLE block split, so the archive difference there needs a subtler parse/block-shape explanation

## 2026-05-31 - Retained Unknown non-text `second_newest` extension on the live level-1 baseline

- While continuing the file-type starting-point work, I found that the live source tree no longer matches some older retained `DictionaryText` reports:
  - direct CLI output on `benchmarks/fixtures/broad-local/dict_dictionary.bin` is currently `23,871`, not the older reported `19,988`
  - until that is reconciled, new experiments should use the live tree as the authoritative baseline
- I then tested three `CompressionFileType::Unknown` non-text matcher variants against that live baseline:
  - raise non-repeat floor from `5` to `6`: reject
  - lower dense long-match index limit from `128` to `64`: reject
  - extend the retained Fastest small-block `second_newest` path up to `128 KiB`: retained, but only after fixing a probe-gating bug
- Important implementation detail:
  - the first `Unknown` `second_newest` attempt widened sidecar allocation in `add_data()` but left `should_track_second_newest_for_current_entry()` on the old `<= 64 KiB` limit, so it benchmarked as a no-op
  - the retained point required widening both the allocation gate and the probe gate
- Retained reports:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-secondnewest-fixed-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-secondnewest-fixed-fast.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-secondnewest-fixed-mainish.md`
  - retained binary:
    - `benchmarks/reports/ruzstd-cli-level1-unknown-secondnewest-retained`
- Result versus the live baseline:
  - broad-local better / worse / equal vs C stayed `16 / 13 / 3`
  - bytes-above-C on losing fixtures improved:
    - `4,188 -> 4,166`
  - wins:
    - `decodecorpus_z000079`: `7,540 -> 7,518`
    - `decodecorpus_z000033`: `544,266 -> 532,424`
    - `decodecorpus_z000028`: `100,250 -> 98,656`
    - `decodecorpus_z000003`: `52,134 -> 51,006`
    - `build_ruzstd-cli`: `860,072 -> 856,479`
  - fast guardrails stayed exact:
    - `decodecorpus_pack.bin`: `5,319,265`
    - `json_logs_32m.jsonl`: `690,084`
    - `xorshift_32m.bin`: `33,555,210`
  - CPU stayed in-band on the fast fixture set; `build_ruzstd-cli` drifted `0.05s -> 0.06s`
- Rejected `Unknown` variants:
  - threshold `6`:
    - `decodecorpus_z000079`: `7,540 -> 7,556`
    - `decodecorpus_z000033`: `544,266 -> 559,261`
    - `build_ruzstd-cli`: `860,072 -> 866,219`
  - dense limit `64`:
    - `decodecorpus_z000079`: `7,540 -> 7,548`
- Interpretation:
  - the remaining unknown-family gap still responds to a better candidate set more than to a broader non-repeat floor or sparser long-match indexing
  - any future follow-up in this family should start from the corrected `Unknown` `second_newest` retained point, not from the two rejected gate ideas

## 2026-05-31 - Reconciled the DictionaryText drift and kept a denser dictionary-only probe step

- Direct current CLI output on `benchmarks/fixtures/broad-local/dict_dictionary.bin` exposed a live regression:
  - current live source before the fix: `23,871`
  - checked-in retained binaries: `20,667`
- Archive inspection showed the live regression was matcher-side under-matching:
  - live `23,871` archive:
    - `decoded_literals=22,688`
    - `sequences=1,974`
    - `sequence_payload_bytes=5,903`
  - older retained `20,667` archive:
    - `decoded_literals=12,603`
    - `sequences=3,988`
    - `sequence_payload_bytes=10,493`
- The direct cause was the `DictionaryText` threshold-8 matcher override.
- Removing that override restored the retained dictionary behavior:
  - `dict_dictionary.bin`: `23,871 -> 20,667`
  - broad-local bytes-above-C on losing fixtures: `4,166 -> 962`
  - fast guardrails stayed exact
- Then tested one narrower dictionary-only follow-up:
  - `DictionaryText` gets a fully dense text no-match probe step of `1`
- Retained reports:
  - rollback:
    - `benchmarks/reports/zstd-bench-compare-level1-dictionary-no8-vs-live-broad-local.md`
    - `benchmarks/reports/zstd-bench-compare-level1-dictionary-no8-vs-live-fast.md`
  - dense step-1:
    - `benchmarks/reports/zstd-bench-compare-level1-dictionary-step1-vs-no8-broad-local.md`
    - `benchmarks/reports/zstd-bench-compare-level1-dictionary-step1-vs-no8-fast.md`
  - retained binary:
    - `benchmarks/reports/ruzstd-cli-level1-dictionary-step1-retained`
- Retained result on the corrected live baseline:
  - `dict_dictionary.bin`: `20,667 -> 20,175`
  - C `zstd -1`: `20,145`
  - broad-local summary vs C:
    - better / worse / equal: `16 / 13 / 3`
    - bytes-above-C on worse fixtures: `470`
  - fast guardrails stayed exact:
    - `decodecorpus_pack.bin`: `5,319,265`
    - `json_logs_32m.jsonl`: `690,084`
    - `xorshift_32m.bin`: `33,555,210`
- Also tested and rejected one more unknown-family follow-up from that corrected baseline:
  - extend the retained dense non-text probe step to `CompressionFileType::Unknown` blocks up to `128 KiB`
  - result:
    - `decodecorpus_z000079`: `7,518 -> 7,531`
    - broad-local bytes-above-C on worse fixtures: `470 -> 483`
  - reports:
    - `benchmarks/reports/zstd-bench-compare-level1-unknown-dense128-vs-dictstep1-broad-local.md`
    - `benchmarks/reports/zstd-bench-compare-level1-unknown-dense128-vs-dictstep1-fast.md`
- Interpretation:
  - the dictionary path is now close enough to C that it is no longer the main problem
  - the remaining meaningful level-1 gap is back to `decodecorpus_z000079`, not `dict_dictionary.bin`

## 2026-05-31 - Rejected two more `decodecorpus_z000079`-focused follow-ups on the corrected baseline

- Tested a narrower parser-side widening on top of the corrected retained baseline:
  - let the retained `Unknown` non-text `second_newest` path also compete against a weak current min-length non-repeat match
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-secondnewest-weakcurrent-vs-dictstep1-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-secondnewest-weakcurrent-vs-dictstep1-fast.md`
- Result:
  - exact byte-for-byte no-op on the corrected retained baseline
  - fast CPU nudged the wrong way:
    - `decodecorpus_pack.bin`: `0.22s -> 0.23s`
- Interpretation:
  - the useful `Unknown`-family `second_newest` signal is already captured by the retained no-candidate path
  - widening it to weak-current cases is exhausted

- Revisited the earlier trailing-RLE idea in a more faithful C-shaped form:
  - split exactly one trailing `32 KiB` RLE block when a full Fastest `Unknown` non-text block ends with a `32 KiB` repeated suffix that clearly continues into the next block
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-aligned-rle-split-vs-dictstep1-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-aligned-rle-split-vs-dictstep1-fast.md`
- Result:
  - it did reproduce the C-like 5-block / 2-RLE block shape on `decodecorpus_z000079`
  - but it still made the target slightly worse:
    - `decodecorpus_z000079`: `7,518 -> 7,522`
    - broad-local bytes-above-C on worse fixtures: `470 -> 474`
- Interpretation:
  - the remaining gap is not mainly “missing the extra 32 KiB RLE split”
  - once the block shape is close to C, the difference is still inside the compressed-block parse and entropy, not the suffix framing itself

## 2026-05-31 - Retained no-repeat early-exit disable for large Fastest Unknown blocks

- Reinspected the corrected live retained baseline on the remaining `decodecorpus_z000079` gap using:
  - archive inspection
  - the ignored Fastest matcher diagnostics on both `decodecorpus_z000079` and nearby winner `decodecorpus_z000033`
- Useful diagnostic result:
  - `z000079` is repeat-heavy and current-window-heavy
  - it has no `second_newest` or long-hash wins at all
  - unlike C, it still emits far fewer literals and sequences
  - but the obvious “be more C-like” subpaths were still wrong:
    - complementary end-of-match insertion made `z000079` worse: `7,518 -> 7,541`
    - reducing repeat-match bias from `2` to `1` did not move `z000079` at all and regressed fast CPU
- That narrowed the next credible move to repeat search control flow rather than candidate scoring or framing.
- Retained change:
  - for large `CompressionFileType::Unknown` non-text Fastest blocks, do not let a long repeat match skip window search early
  - block-end repeats can still terminate early; the change only removes the length-based early exit on that path
- Retained reports:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-no-repeat-earlyexit-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-no-repeat-earlyexit-fast.md`
  - retained binary:
    - `benchmarks/reports/ruzstd-cli-level1-unknown-no-repeat-earlyexit-retained`
- Result versus the corrected retained live baseline:
  - `decodecorpus_z000079`: `7,518 -> 7,344`
  - `build_ruzstd-cli`: `856,479 -> 855,908`
  - `decodecorpus_z000028`: `98,656 -> 98,592`
  - `decodecorpus_z000033`: `532,424 -> 532,333`
  - broad-local better / worse / equal vs C stayed `16 / 13 / 3`
  - bytes-above-C on worse fixtures improved: `470 -> 296`
  - fast guardrails stayed exact:
    - `decodecorpus_pack.bin`: `5,319,265`
    - `json_logs_32m.jsonl`: `690,084`
    - `xorshift_32m.bin`: `33,555,210`
- Important archive result:
  - retained new `z000079` output is `7,344`
  - but it still has fewer literals and sequences than the prior retained point
  - so the win is not “matching C’s block shape”; it is a better compressed-block parse under our own shape

## 2026-05-31 - Rejected two follow-ups on the retained large-Unknown no-repeat early-exit path

- Tried a smaller-offset non-repeat bias on the same large Fastest `Unknown` path:
  - if two non-repeat candidates started at the same position, let the smaller offset win when it was at most 1 byte shorter
- Why:
  - `z000079` still had much larger offsets and far fewer sequences than C, so this looked like the narrowest way to reduce far-match preference without changing the wider parser shape
- Result: reject
  - `decodecorpus_z000079`: unchanged at `7,344`
  - some already-winning unknown-family fixtures improved slightly
  - fast CPU moved the wrong way:
    - `decodecorpus_pack.bin`: `0.23s -> 0.25s`
- Conclusion:
  - the remaining `z000079` gap is not simply “prefer smaller offsets more often”

- Then tried the direct follow-up to the retained no-repeat early-exit win:
  - remove the last remaining repeat early-exit too, including block-end exits
- Result: reject
  - exact byte-for-byte no-op versus the retained point
  - fast CPU drifted the wrong way:
    - `decodecorpus_pack.bin`: `0.22s -> 0.25s`
- Conclusion:
  - the useful win in the repeat-early-exit family is exhausted at the retained length-only cut

## 2026-05-31 - Rejected large-Unknown split-family follow-ups after the retained no-repeat early-exit win

- Tested a new family aimed at the remaining `decodecorpus_z000079` gap:
  - compare the whole block against a midpoint split candidate on the large Fastest `Unknown` non-text path
  - first with the existing estimate gate
  - then with an exact whole-vs-midpoint comparison that bypassed the gate
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-split-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-split-fast.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-exactsplit-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-exactsplit-fast.md`
- Result:
  - both variants behaved the same
  - `decodecorpus_z000079` stayed unchanged at `7,344`
  - broad-local bytes-above-C on losing fixtures stayed `296`
  - already-winning unknown-family fixtures improved:
    - `build_ruzstd-cli`: `855,908 -> 850,952`
    - `decodecorpus_z000028`: `98,592 -> 97,095`
    - `decodecorpus_z000033`: `532,333 -> 525,509`
  - but the fast CPU screen drifted the wrong way:
    - `decodecorpus_pack.bin`: `0.22s -> 0.23s`
- Decision:
  - Reject and revert.
  - The remaining `z000079` gap is not waiting on a simple whole-vs-midpoint split candidate, and bypassing the estimate gate does not change that.

## 2026-05-31 - Rejected dictionary-threshold and tiny-code follow-ups, retained an Unknown entropy tweak

- First tried to reduce the current dictionary overmatching directly by raising the `DictionaryText` matcher floor from `5` to `7`.
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-dictionary-threshold7-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-dictionary-threshold7-fast.md`
- Result:
  - hard regression on the target:
    - `dict_dictionary.bin`: `20,175 -> 21,619`
- Decision:
  - Reject and revert.
  - The current retained dictionary-step-1 path does not want a stronger floor.

- Then tried a very narrow `CodeText` split:
  - keep the retained code-like short-line floor of `6`
  - but lower it to `5` only for code-like files up to `8 KiB`
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-small-code5-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-small-code5-fast.md`
- Result:
  - `repo_main.rs`: `2,137 -> 2,136`
  - everything else stayed exact
- Decision:
  - Reject and revert.
  - One byte is not enough to justify another persistent code-path split.

- Retained instead a narrower entropy-side tweak:
  - at level 1, `CompressionFileType::Unknown` now uses the small-literal exact Huffman table search
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-smallhuff-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-smallhuff-fast.md`
  - archive inspection:
    - `benchmarks/reports/archive-inspect/decodecorpus_z000079.unknown_smallhuff.l1.inspect.txt`
- Result on the current retained baseline:
  - `decodecorpus_z000079`: `7,344 -> 7,340`
  - broad-local bytes-above-C on losing fixtures: `296 -> 292`
  - fast guardrails stayed exact
- Useful conclusion:
  - there was still a tiny entropy-side gain left on the unknown-family path
  - it is literal-side only:
    - `literal_section_bytes`: `823 -> 819`
    - `sequence_payload_bytes`: unchanged at `6480`

- Also re-ran the old large-block `second_newest` family under the current file-type split:
  - extend the retained small-block Fastest `Unknown` no-candidate `second_newest` path to all `Unknown` non-text blocks
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-large-secondnewest-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-large-secondnewest-fast.md`
- Result:
  - exact byte-for-byte no-op on the retained `unknown-smallhuff` baseline
  - fast CPU still drifted the wrong way:
    - `decodecorpus_pack.bin`: `0.22s -> 0.23s`
    - `json_logs_32m.jsonl`: `0.16s -> 0.17s`
- Decision:
  - Reject and revert.
  - The old large-block `second_newest` family is still exhausted, even after narrowing it to `Unknown` through the file-type split.

- Added richer archive diagnostics in `ruzstd/src/tests/mod.rs`:
  - per-block `ll_extra_bits`, `ml_extra_bits`, `of_extra_bits`
  - top code histograms for LL / ML / OF symbols
- Re-ran them on:
  - `benchmarks/reports/archive-inspect/decodecorpus_z000079.c.l1.zst`
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/archive-inspect/current/decodecorpus_z000079.unknown_smallhuff.l1.zst`
- Useful result:
  - the retained `z000079` gap is overwhelmingly offset-bit cost
  - C totals:
    - `sequence_payload_bytes=5722`
    - `decoded_literals=2799`
    - `sequences=4354`
  - C block 1 / 2:
    - `of_extra_bits=2205`, `2298`
  - retained Rust totals:
    - `sequence_payload_bytes=6480`
    - `decoded_literals=1455`
    - `sequences=2804`
  - retained Rust block 1 / 2:
    - `of_extra_bits=12110`, `4363`
  - the retained Rust path is still choosing much larger offset codes, especially in block 1

- Tried two direct follow-ups from that clue:
  1. current-entry cutoff on large Fastest `Unknown` blocks
     - stop before older-entry search when the current entry already has a 16-byte non-repeat candidate
     - reports:
       - `benchmarks/reports/zstd-bench-compare-level1-unknown-current-entry-cutoff-broad-local.md`
       - `benchmarks/reports/zstd-bench-compare-level1-unknown-current-entry-cutoff-fast.md`
     - result:
       - `decodecorpus_z000079`: `7,340 -> 7,524`
     - decision: reject

  2. offset-aware same-start non-repeat comparison on large Fastest `Unknown` blocks
     - prefer a smaller-offset non-repeat candidate when it saves at least 4 offset-code bits for at most a 2-byte match loss
     - reports:
       - `benchmarks/reports/zstd-bench-compare-level1-unknown-offsetaware-broad-local.md`
       - `benchmarks/reports/zstd-bench-compare-level1-unknown-offsetaware-fast.md`
     - result:
       - exact byte-for-byte no-op on the retained baseline
       - fast CPU still drifted the wrong way
     - decision: reject

- Conclusion from this cycle:
  - we now have strong evidence that `z000079` is paying in offset bits
  - but the remaining fix is not a simple local cutoff or same-start scoring rule
  - the next credible move needs a deeper current-entry / repeat / offset-history representation change, or a more explicit sequence-entropy-side intervention than these local candidate gates

## 2026-05-31 - Retained a stronger repeat-vs-normal bias for large Fastest Unknown blocks

- Starting from the retained `unknown-smallhuff` baseline and the new `z000079` offset-bit diagnostics, tested whether the remaining gap would shrink if large Fastest `Unknown` non-text blocks let repeat matches beat normal matches with a wider length margin.
- Retained change:
  - only on large Fastest `CompressionFileType::Unknown` non-text blocks
  - widen the repeat-vs-normal match-length margin from `2` to `4`
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-repeatbias-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-repeatbias-fast.md`
  - retained binary:
    - `benchmarks/reports/ruzstd-cli-level1-unknown-repeatbias-retained`
  - archive inspect:
    - `benchmarks/reports/archive-inspect/decodecorpus_z000079.unknown_repeatbias.l1.inspect.txt`
- Result:
  - `decodecorpus_z000079`: `7,340 -> 7,329`
  - broad-local better / worse / equal vs C stayed `16 / 13 / 3`
  - bytes-above-C on losing fixtures improved: `292 -> 281`
  - fast guardrails stayed exact where they matter:
    - `decodecorpus_pack.bin`: `5,319,265`
    - `json_logs_32m.jsonl`: `690,084`
    - `dict_dictionary.bin`: `20,175`
    - `repo_main.rs`: `2,137`
  - known givebacks on already-winning binaries:
    - `build_ruzstd-cli`: `855,908 -> 855,996`
    - `decodecorpus_z000033`: `532,333 -> 532,528`
- Useful archive result:
  - the win is real but narrow
  - `sequence_payload_bytes`: `6480 -> 6469`
  - `decoded_literals`: `1455 -> 1460`
  - block 2 `of_extra_bits`: `4363 -> 4326`
  - block 1 `of_extra_bits` stayed flat at `12110`
- Interpretation:
  - the remaining `z000079` gap still is not a framing issue
  - but a stronger repeat preference can buy back a little offset cost on the retained large-`Unknown` path

## 2026-05-31 - Rejected the narrower `2 -> 3` repeat-bias bump on the same large-Unknown path

- Followed the retained large-`Unknown` repeat-bias win with the narrower version:
  - increase the repeat-vs-normal margin from `2` to `3` instead of `4`
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-repeatbiasplus1-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-repeatbiasplus1-fast.md`
- Result:
  - `decodecorpus_z000079`: stayed `7,340`
  - broad-local better / worse / equal vs C stayed `16 / 13 / 3`
  - bytes-above-C on losing fixtures stayed `292`
  - already-winning binaries still regressed:
    - `build_ruzstd-cli`: `855,908 -> 856,018`
    - `decodecorpus_z000033`: `532,333 -> 532,439`
- Interpretation:
  - the retained signal starts at the stronger `2 -> 4` bump
  - the narrower `2 -> 3` version is just collateral without target benefit

## 2026-05-31 - Retained the next stronger large-Unknown repeat-bias point and rejected the one after it

- Continued the same large Fastest `Unknown` family from the retained `2 -> 4` point.
- First tested the next stronger setting:
  - widen the repeat-vs-normal margin from `4` to `5`
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-repeatbiasplus3-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-repeatbiasplus3-fast.md`
  - retained binary:
    - `benchmarks/reports/ruzstd-cli-level1-unknown-repeatbiasplus3-retained`
  - archive inspect:
    - `benchmarks/reports/archive-inspect/decodecorpus_z000079.unknown_repeatbiasplus3.l1.inspect.txt`
- Result:
  - `decodecorpus_z000079`: `7,329 -> 7,326`
  - broad-local better / worse / equal vs C stayed `16 / 13 / 3`
  - bytes-above-C on losing fixtures improved: `281 -> 278`
  - already-winning binaries gave back a little more:
    - `build_ruzstd-cli`: `855,996 -> 856,110`
    - `decodecorpus_z000033`: `532,528 -> 532,592`
  - fast guardrails stayed exact where they matter:
    - `decodecorpus_pack.bin`: `5,319,265`
    - `json_logs_32m.jsonl`: `690,084`
    - `dict_dictionary.bin`: `20,175`
    - `repo_main.rs`: `2,137`
- Useful archive result:
  - the win is still sequence/offset-side
  - `sequence_payload_bytes`: `6469 -> 6464`
  - `decoded_literals`: `1460 -> 1464`
  - block 2 `of_extra_bits`: `4326 -> 4290`
  - block 1 `of_extra_bits` stayed flat at `12110`
- Decision:
  - Retain. This is the best point found in the family so far.

- Then tested one stronger point:
  - widen the repeat-vs-normal margin from `5` to `6`
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-repeatbiasplus4-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-repeatbiasplus4-fast.md`
- Result:
  - `decodecorpus_z000079`: `7,329 -> 7,333`
  - broad-local bytes-above-C on losing fixtures worsened: `281 -> 285`
  - already-winning binaries regressed more:
    - `build_ruzstd-cli`: `855,996 -> 856,134`
    - `decodecorpus_z000033`: `532,528 -> 532,616`
- Decision:
  - Reject and revert.
  - This bounds the family: `2 -> 5` is currently the best retained point; `2 -> 6` is worse.

## 2026-05-31 - Retained dense short-line probing for tiny CodeText and ConfigText

- After bounding the large-`Unknown` repeat-bias family, split the remaining level-1 losses by behavior:
  - `repo_main.rs` was still a small text/code residual
  - `dict_systemd-logind.service` and `dict_systemd-coredump@.service` were small config/service residuals
  - the binary holdouts (`decodecorpus_z000079`, `dict_dictionary.bin`) were already isolated
- Retained change:
  - for `CompressionFileType::CodeText` and `CompressionFileType::ConfigText`
  - only on short-line text blocks up to `8 KiB`
  - force dense no-match probing with step `1`
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-codeprobe1-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-codeconfigprobe1-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-codeconfigprobe1-fast.md`
  - retained binary:
    - `benchmarks/reports/ruzstd-cli-level1-codeconfigprobe1-retained`
- Result:
  - `repo_main.rs`: `2,137 -> 2,105`
  - C `zstd -1`: `2,101`
  - `dict_systemd-logind.service`: `1,134 -> 1,122`
  - `dict_systemd-coredump@.service`: `690 -> 686`
  - broad-local better / worse / equal vs C stayed `16 / 13 / 3`
  - bytes-above-C on losing fixtures improved: `278 -> 230`
  - important unchanged holdouts:
    - `decodecorpus_z000079`: `7,326`
    - `dict_dictionary.bin`: `20,175`
    - `build_ruzstd-cli`: `856,110`
- Interpretation:
  - the remaining small text/config residuals still wanted denser local search, but only under a very narrow file-type and size gate
  - this does not reopen the binary path and it does not disturb JSON

## 2026-05-31 - Retained dictionary-only same-start smaller-offset preference after direct live-tree A/B

- Starting from the retained `codeconfigprobe1` live baseline, the remaining `dict_dictionary.bin` gap still looked offset-side:
  - archive inspection still showed too many sequences and too many offset extra bits versus C
  - Fastest matcher diagnostics showed the dictionary path is mainly a current-entry `newest` vs `oldest` fight, with no repeat, `second_newest`, or long-hash signal
- Tried a new `DictionaryText`-only candidate comparison:
  - non-repeat candidates only
  - same `start_idx` only
  - prefer the smaller offset when it saves at least 2 offset-code bits for at most a 1-byte match loss
- I validated it with a direct live-tree A/B instead of comparing against older retained binaries:
  - saved the candidate binary with the new rule
  - disabled only that rule in source
  - rebuilt a baseline binary from the live tree
  - benchmarked candidate vs baseline directly
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-dict-offsetaware-live-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-dict-offsetaware-live-fast.md`
- Result:
  - `dict_dictionary.bin`: `20,175 -> 20,162`
  - C `zstd -1`: `20,145`
  - every other broad-local fixture stayed byte-identical
  - fast guardrails stayed byte-identical, with repeat CPU in the same band:
    - `decodecorpus_pack.bin`: `0.22s -> 0.22s`
    - `json_logs_32m.jsonl`: `0.16s -> 0.16s`
  - broad-local better / worse / equal vs C stayed `16 / 13 / 3`
  - total bytes-above-C on losing fixtures improved: `230 -> 217`
- Decision:
  - Retain. This is the first dictionary-specific current-entry candidate-selection rule that improves the target without disturbing the rest of the live suite.

- Also tested the wider variant in the same family:
  - allow up to a 2-byte match loss with the same 2-bit offset-code saving threshold
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-dict-offsetaware2-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-dict-offsetaware2-fast.md`
- Result:
  - exact same dictionary outcome as the retained point:
    - `dict_dictionary.bin`: `20,175 -> 20,162`
  - no other fixtures moved
- Decision:
  - Reject as a separate setting. The narrower retained point already captures the full gain.

## 2026-05-31 - Rejected two more `Unknown`-family follow-ups after the retained dictionary offset-aware point

- Tried a small-`Unknown` analogue of the retained dictionary offset-aware rule:
  - non-repeat candidates only
  - same `start_idx` only
  - prefer the smaller offset when it saves at least 2 offset-code bits for at most a 1-byte match loss
  - gated to `CompressionFileType::Unknown` non-text blocks up to `128 KiB`
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-small-unknown-offsetaware-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-small-unknown-offsetaware-fast.md`
- Result:
  - hard regression across the `decodecorpus_z...` family:
    - `decodecorpus_z000079`: `7,326 -> 7,946`
    - `build_ruzstd-cli`: `856,110 -> 878,123`
    - `decodecorpus_z000059`: `711 -> 747`
- Decision:
  - Reject and revert.
  - This closes the “reuse the dictionary same-start offset-aware rule on small Unknown blocks” family.

- Then tried a narrow entropy-side offset lever for `Unknown`:
  - lower offset FSE table max-log from `8` to `7` at level 1
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-oflog7-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-oflog7-fast.md`
- Result:
  - tiny wins only:
    - `decodecorpus_z000079`: `7,326 -> 7,325`
    - `decodecorpus_z000059`: `711 -> 709`
    - `build_ruzstd-cli`: `856,110 -> 855,482`
  - but the gain is too small to justify another file-type entropy knob
  - the repeat fast run also drifted the wrong way on `build_ruzstd-cli` (`0.06s -> 0.07s`)
- Decision:
  - Reject and revert.
  - This suggests the remaining `Unknown` gap is not going to fall to another tiny offset-FSE-table-log tweak.

## 2026-05-31 - Rejected two more narrow no-op follow-ups after the retained dictionary offset-aware point

- Re-ran two narrow file-type follow-ups sequentially against the retained dictionary-offset-aware baseline:
  1. tiny `ConfigText` short-line threshold `5 -> 4` below `1 KiB`
  2. widen the retained dictionary same-start smaller-offset rule to “starts no later and covers to within 1 byte”
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-tiny-config4-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-tiny-config4-fast.md`
  - `benchmarks/reports/zstd-bench-compare-level1-dict-offsetcoverage-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-dict-offsetcoverage-fast.md`
- Result:
  - both were exact byte-for-byte no-ops on the retained baseline
  - `dict_dictionary.bin` stayed `20,162`
  - the remaining service-file residuals did not move at all
- Decision:
  - Reject both and revert.
  - This closes one more local family:
    - the config/service residuals are not waiting on a tiny short-line threshold cut
    - the retained dictionary same-start rule already captured the useful offset-side gain; widening it to near-same coverage buys nothing

## 2026-05-31 - Retained `DictionaryText` offset FSE max-log `7` and rejected `6`

- The in-flight dictionary entropy branch had drifted past the last trustworthy benchmark point, so I bounded the family explicitly with a direct sequential A/B:
  - built and saved the current `DictionaryText offset_table_max_log = 6` candidate
  - patched back to `7`
  - rebuilt and benchmarked `6` vs `7` directly
- Reports:
  - retained stepping-stone `8 -> 7`:
    - `benchmarks/reports/zstd-bench-compare-level1-dict-oflog7-broad-local.md`
    - `benchmarks/reports/zstd-bench-compare-level1-dict-oflog7-fast.md`
  - direct family bound `6 vs 7`:
    - `benchmarks/reports/zstd-bench-compare-level1-dict-oflog6-vs7-broad-local.md`
    - `benchmarks/reports/zstd-bench-compare-level1-dict-oflog6-vs7-fast.md`
- Result:
  - `7` is the retained point:
    - `dict_dictionary.bin`: `20,162 -> 20,160`
    - every other broad-local fixture stayed exact
    - broad-local bytes-above-C on losing fixtures: `217 -> 215`
  - `6` is a hard regression:
    - `dict_dictionary.bin`: `20,160 -> 20,432`
    - everything else stayed exact
- Decision:
  - Retain `DictionaryText offset_table_max_log = 7`
  - Reject `6`
  - This family is now bounded:
    - `8 -> 7` helps slightly
    - `7 -> 6` is clearly worse

## 2026-05-31 - Rejected a wider large-`Unknown` `ip+1` normal-window promotion

- Tried a narrower C-shaped follow-up on the retained `dict oflog7` baseline:
  - only on large Fastest `CompressionFileType::Unknown` non-text blocks
  - widen the next-position normal-window promotion from exact-minimum current non-repeat hits to short current non-repeat hits up to `8` bytes
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-nextwindowwide-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-nextwindowwide-fast.md`
- Result:
  - exact byte-for-byte no-op on both suites:
    - `decodecorpus_z000079`: stayed `7,326`
    - `build_ruzstd-cli`: stayed `856,110`
  - no other broad-local fixture moved
- Decision:
  - Reject and revert.
  - This closes another adjacent-position family on the current matcher shape: widening the `ip+1` normal-window probe condition does not move the retained `Unknown` binary baseline.

## 2026-05-31 - Retained a large-`Unknown` oldest-displacement rule and rejected the stronger point

- Tried a new current-window scoring rule on top of the retained `dict oflog7` baseline:
  - only on large Fastest `CompressionFileType::Unknown` non-text blocks
  - when an `oldest` candidate tries to displace an already-found closer non-repeat candidate, require at least a 2-byte match gain
- Reports:
  - retained `+2` point:
    - `benchmarks/reports/zstd-bench-compare-level1-unknown-oldestgain2-broad-local.md`
    - `benchmarks/reports/zstd-bench-compare-level1-unknown-oldestgain2-fast.md`
  - bounded `+3` point:
    - `benchmarks/reports/zstd-bench-compare-level1-unknown-oldestgain3-vs2-broad-local.md`
    - `benchmarks/reports/zstd-bench-compare-level1-unknown-oldestgain3-vs2-fast.md`
- Result:
  - `+2` is retained:
    - `decodecorpus_z000079`: `7,326 -> 7,324`
    - `decodecorpus_z000028`: `98,567 -> 98,388`
    - `decodecorpus_z000033`: `532,592 -> 532,546`
    - `build_ruzstd-cli`: `856,110 -> 855,782`
    - fast guardrails stayed exact on the important fixtures
    - broad-local bytes-above-C on losing fixtures: `215 -> 213`
  - `+3` is worse:
    - `decodecorpus_z000079`: `7,324 -> 7,335`
    - `build_ruzstd-cli`: `855,782 -> 856,067`
    - fast-guardrail CPU drifted slightly worse
- Decision:
  - Retain the `+2` rule.
  - Reject `+3`.
  - This is the first current-window scoring rule on the large `Unknown` path that moved the remaining `z000079` gap without disturbing CPU.

## 2026-05-31 - Rejected a selective stronger large-`Unknown` oldest-displacement rule

- Tried a more selective stronger follow-up on top of the retained `+2` oldest-displacement rule:
  - still only on large Fastest `CompressionFileType::Unknown` non-text blocks
  - keep the retained `+2` rule normally
  - require `+3` only when the farther `oldest` candidate also costs at least 4 more offset-code bits
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-oldestgain2bits-vs2-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-oldestgain2bits-vs2-fast.md`
- Result:
  - target regressed:
    - `decodecorpus_z000079`: `7,324 -> 7,328`
  - part of the retained unknown-family win regressed too:
    - `build_ruzstd-cli`: `855,782 -> 855,829`
  - CPU stayed effectively flat
- Decision:
  - Reject and revert.
  - The retained `+2` rule is already near the useful edge; strengthening it only on big offset-bit gaps still over-penalizes some worthwhile `oldest` wins.

## 2026-05-31 - Rejected a distance-based stronger large-`Unknown` oldest-displacement rule

- Tried another structural follow-up on the retained `+2` oldest-displacement rule:
  - keep the retained `+2` rule inside the current entry
  - require `+3` only when `oldest` comes from the previous entry (`entry_distance >= 1`)
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-oldestdist1-vs2-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-oldestdist1-vs2-fast.md`
- Result:
  - target did not improve:
    - `decodecorpus_z000079`: stayed `7,324`
  - part of the retained unknown-family win regressed:
    - `build_ruzstd-cli`: `855,782 -> 855,926`
  - CPU stayed effectively flat
- Decision:
  - Reject and revert.
  - Entry-distance alone is not a strong enough signal to sharpen the retained `+2` rule.

## 2026-05-31 - Retained `Unknown` offset FSE max-log `7` and rejected `6`

- Revisited the earlier `Unknown oflog7` entropy idea on the newer retained baseline that already includes the large-`Unknown` oldest-displacement `+2` rule.
- Reports:
  - retained `7` point:
    - `benchmarks/reports/zstd-bench-compare-level1-unknown-oflog7-current-vsretained-broad-local.md`
    - `benchmarks/reports/zstd-bench-compare-level1-unknown-oflog7-current-vsretained-fast.md`
  - bounded `6` point:
    - `benchmarks/reports/zstd-bench-compare-level1-unknown-oflog6-vs7-current-broad-local.md`
    - `benchmarks/reports/zstd-bench-compare-level1-unknown-oflog6-vs7-current-fast.md`
- Result:
  - `7` is retained:
    - `decodecorpus_z000079`: `7,324 -> 7,321`
    - `build_ruzstd-cli`: `855,782 -> 855,822`
    - `decodecorpus_z000033`: `532,546 -> 532,650`
    - broad-local bytes-above-C on losing fixtures: `213 -> 210`
    - fast guardrails stayed in the same CPU band
  - `6` is a hard regression:
    - `decodecorpus_z000079`: `7,321 -> 7,413`
    - `build_ruzstd-cli`: `855,822 -> 860,261`
    - `decodecorpus_z000033`: `532,650 -> 536,771`
- Decision:
  - Retain `Unknown offset_table_max_log = 7` for level 1.
  - Reject `6`.
  - This family is now bounded on the current retained baseline.

## 2026-05-31 - Rejected small-sequence-only `Unknown oflog6`

- Tried a block-local entropy variant on top of the retained `Unknown oflog7` point:
  - keep retained `oflog7` normally
  - use `oflog6` only when the `Unknown` level-1 block has at most `1,536` sequences
- Motivation:
  - retained archive inspection showed `decodecorpus_z000079` is a much smaller-sequence-count Unknown case than already-winning build-artifact blocks like `build_ruzstd-cli`
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-oflog6-smallseq-vs7-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-oflog6-smallseq-vs7-fast.md`
- Result:
  - target regressed hard:
    - `decodecorpus_z000079`: `7,321 -> 7,413`
  - neighboring decodecorpus samples also regressed:
    - `decodecorpus_z000053`: `322 -> 324`
    - `decodecorpus_z000054`: `9,567 -> 9,589`
    - `decodecorpus_z000059`: `711 -> 714`
  - `build_ruzstd-cli` stayed flat at `855,822`
- Decision:
  - Reject and revert.
  - Sequence-count alone is not a safe gate for a stronger `Unknown` offset entropy table.

## 2026-05-31 - Retained `Unknown` newest-displacement `+2` and rejected `+3`

- Tried the missing sibling to the retained large-`Unknown` `oldest +2` scoring rule:
  - only on large Fastest `CompressionFileType::Unknown` non-text blocks
  - when a farther `newest` current-window candidate tries to displace an already-found closer non-repeat candidate, require at least a 2-byte match gain
- Reports:
  - retained `+2` point:
    - `benchmarks/reports/zstd-bench-compare-level1-unknown-newestgain2-vsretained-broad-local.md`
    - `benchmarks/reports/zstd-bench-compare-level1-unknown-newestgain2-vsretained-fast.md`
  - bounded `+3` point:
    - `benchmarks/reports/zstd-bench-compare-level1-unknown-newestgain3-vs2-broad-local.md`
- Result:
  - `+2` is retained:
    - `build_ruzstd-cli`: `855,822 -> 855,679`
    - `decodecorpus_z000033`: `532,650 -> 532,632`
    - `decodecorpus_z000079`: stayed `7,321`
    - fast guardrails stayed in the same CPU band
    - broad-local bytes-above-C on losing fixtures stayed `210`
  - `+3` is worse:
    - `build_ruzstd-cli`: `855,679 -> 855,915`
    - `decodecorpus_z000079`: stayed `7,321`
- Decision:
  - Retain `newest +2`.
  - Reject `+3`.
  - This bounds the large-`Unknown` current-window scoring family on the `newest` side.

## 2026-05-31 - Rejected extending the retained `Unknown` displacement rules to all Unknown sizes

- Tried a narrower family extension after retaining the large-`Unknown` `oldest +2` and `newest +2` rules:
  - keep both retained displacement rules
  - remove the `128 KiB` size gate so they apply to all Fastest `CompressionFileType::Unknown` non-text blocks
- Motivation:
  - the smaller remaining Unknown losers (`decodecorpus_z000053`, `decodecorpus_z000059`) show the same `newest` / `oldest` current-window shape as `z000079`
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-allsize-displacement-vsretained-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-allsize-displacement-vsretained-fast.md`
- Result:
  - small Unknown fixtures improved:
    - `decodecorpus_z000059`: `711 -> 709`
    - `decodecorpus_z000054`: `9,567 -> 9,565`
    - `decodecorpus_z000080`: `2,603 -> 2,602`
    - `decodecorpus_z000003`: `51,012 -> 51,001`
  - but total gap improvement was too small:
    - broad-local bytes-above-C on losing fixtures: `210 -> 208`
  - and the main fast guardrails drifted the wrong way:
    - `decodecorpus_pack.bin` CPU: `0.22s -> 0.23s`
    - `json_logs_32m.jsonl` CPU: `0.16s -> 0.17s`
- Decision:
  - Reject and revert.
  - The displacement rules are worth keeping only on the large-Unknown path in the current matcher shape.

## 2026-05-31 - Rejected tiny-sequence-only `Unknown oflog6`

- After inspecting the smaller Unknown losers (`decodecorpus_z000053`, `decodecorpus_z000059`), tried a narrower follow-up to the rejected small-sequence entropy family:
  - keep retained `Unknown oflog7` normally
  - use `oflog6` only when the `Unknown` level-1 block has at most `256` sequences
- Motivation:
  - `decodecorpus_z000053` and `decodecorpus_z000059` are single-block Unknown cases with only `36` and `246` sequences, far below `z000079`
- Report:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-tinyseq-oflog6-vsretained-broad-local.md`
- Result:
  - regressed the exact targets:
    - `decodecorpus_z000053`: `322 -> 324`
    - `decodecorpus_z000059`: `711 -> 714`
  - and still nudged `decodecorpus_z000079` the wrong way:
    - `7,321 -> 7,325`
- Decision:
  - Reject and revert.
  - Sequence-count alone is not a safe gate for stronger `Unknown` offset entropy tables, even at much lower thresholds.

## 2026-05-31 - Retained Unknown predefined LL/ML gate on small compressed-literals blocks

- After inspecting the smaller remaining Unknown losers, tried a sequence-entropy-side change instead of another matcher gate:
  - level 1 only
  - `CompressionFileType::Unknown`
  - if the block is in the compressed-literals path and has at most `64` sequences, allow predefined LL/ML tables
- Motivation:
  - `decodecorpus_z000053` was still losing by `23` bytes even though sequence count already matched C
  - archive inspection showed C used `ll_mode=predefined`, `ml_mode=predefined`, while Rust still encoded both tables
- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-predef64-complit-vsretained-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-predef64-complit-vsretained-fast.md`
  - restore:
    - `benchmarks/reports/zstd-bench-restore-level1-unknown-predef64-complit.md`
- Result:
  - `decodecorpus_z000053`: `322 -> 304`
  - `decodecorpus_z000031`: stays `112` because the compressed-literals condition excludes the raw-literals case that regressed in the unrestricted version
  - broad-local bytes-above-C on losing fixtures: `210 -> 192`
  - fast guardrails stayed exact on bytes and in the same CPU band
- Useful archive result:
  - on `decodecorpus_z000053`, Rust now matches C’s LL/ML table-mode shape:
    - before: `ll_mode=fse`, `ml_mode=fse`, `sequence_payload_bytes=106`
    - after: `ll_mode=predefined`, `ml_mode=predefined`, `sequence_payload_bytes=88`
- Decision:
  - Retain.
  - This is the first recent win on the small Unknown losers that comes from sequence-table mode instead of matcher gating.

## 2026-05-31 - Rejected predefined OF-table eligibility on top of the retained Unknown LL/ML gate

- Followed the retained Unknown small compressed-literals LL/ML predefined-table gate with the obvious entropy-side sibling:
  - keep the retained LL/ML gate
  - also allow predefined OF tables on the same small Unknown block family
- Report:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-predef64-complit-of-vsllmlonly-broad-local.md`
- Result:
  - regressed the same small Unknown targets immediately:
    - `decodecorpus_z000053`: `304 -> 305`
    - `decodecorpus_z000059`: `711 -> 747`
- Decision:
  - Reject and revert.
  - The retained Unknown small-sequence predefined-table win is specifically LL/ML-side, not OF-side.

## 2026-05-31 - Rejected two more `Unknown` current-window scoring variants after refreshing live matcher diagnostics

- Refreshed ignored matcher diagnostics directly on the live retained baseline for:
  - `decodecorpus_z000079`
  - `dict_dictionary.bin`
  - `decodecorpus_z000059`
  - `decodecorpus_z000053`
- Current signal is still:
  - `z000079` dominated by `repeat_next_position_selected_without_current_candidate`
  - current-window wins almost entirely `newest` / `oldest`
  - no long-hash and no `second_newest`

- Rejected:
  - large `Unknown` same-start smaller-offset preference
    - exact byte no-op on broad-local and fast guardrails
    - `decodecorpus_z000079` stayed `7,321`
  - tiny `Unknown` displacement (`<=4 KiB`)
    - `decodecorpus_z000059`: `711 -> 709`
    - but regressed:
      - `decodecorpus_z000031`: `112 -> 113`
      - `decodecorpus_z000053`: `304 -> 305`
    - fast JSON CPU still drifted the wrong way

- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-samestart-offset-vsretained-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-samestart-offset-vsretained-fast.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-smalldisplacement-vsretained-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-smalldisplacement-vsretained-fast.md`

- Conclusion:
  - the remaining `z000079` gap is not a same-start smaller-offset fight
  - the older all-size `Unknown` displacement family does not become safe just by narrowing it to tiny blocks
  - the next credible move is still a more structural large-`Unknown` sequence/offset decision

## 2026-05-31 - Refreshed live `z000079` archive evidence and rejected three more structural large-`Unknown` branches

- Refreshed live current-vs-C `decodecorpus_z000079` archive inspections with sequence histograms.
- Current Rust still differs sharply from C:
  - Rust:
    - `compressed_bytes=7321`
    - `literal_section_bytes=820`
    - `sequence_payload_bytes=6460`
    - `decoded_literals=1463`
    - `sequences=2806`
    - `match_bytes=391753`
  - C:
    - `compressed_bytes=7221`
    - `literal_section_bytes=1449`
    - `sequence_payload_bytes=5722`
    - `decoded_literals=2799`
    - `sequences=4354`
    - `match_bytes=357649`

- Rejected:
  - large `Unknown` smaller-offset rescoring without same-start restriction
    - exact byte no-op
  - large `Unknown` half-window
    - `decodecorpus_z000079`: unchanged at `7,321`
    - `build_ruzstd-cli`: `855,679 -> 865,333`
  - large `Unknown` `ip+1` newest-repeat-only
    - `decodecorpus_z000079`: `7,321 -> 7,606`
    - `build_ruzstd-cli`: `855,679 -> 856,949`

- Reports:
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-offsetscore-vsretained-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-halfwindow-vsretained-broad-local.md`
  - `benchmarks/reports/zstd-bench-compare-level1-unknown-nextrep-firstonly-vsretained-broad-local.md`

- Conclusion:
  - the remaining `z000079` gap is still sequence/offset-side and materially under-sequenced
  - it is not waiting on:
    - a broader cheaper-offset scoring preference
    - a smaller effective search window
    - removing second/third repeat-history slots from the `ip+1` repeat probe
  - the next credible move needs a different large-`Unknown` parse representation or sequence/offset structure

## 2026-05-31 - Rejected disabling backward match extension on the large-`Unknown` Fastest path

- Tried a direct structural cut aimed at the under-sequenced `z000079` shape:
  - on the large `Unknown` Fastest path, disable backward match extension entirely
- Why:
  - fresh current-vs-C archive evidence still showed:
    - Rust `sequences=2806`, `decoded_literals=1463`
    - C `sequences=4354`, `decoded_literals=2799`
- Result:
  - `decodecorpus_z000079`: `7,321 -> 7,360`
  - `build_ruzstd-cli`: `855,679 -> 866,828`
  - `decodecorpus_z000033`: `532,632 -> 537,783`
- Conclusion:
  - backward extension alone is not the keepable cause of the under-sequenced large-`Unknown` shape
  - the next credible move still needs a different parse representation or sequence/offset structure, not this blunt cut

## 2026-05-31 - Rejected denser post-match insertion on the large-`Unknown` `RepeatNextPosition` path

- Tried two structural variants on the dominant `z000079` family:
  - denser post-match suffix insertion limit `256`
  - fully dense post-match suffix insertion
- Scope:
  - only large `Unknown` Fastest blocks
  - only after `RepeatNextPosition` wins
- Why:
  - live matcher diagnostics still show `repeat_next_position_selected_without_current_candidate` dominating `z000079`
  - if the remaining gap were from too little future candidate availability after those wins, denser post-match insertion should have changed the parse
- Result:
  - both variants were exact byte no-ops on `decodecorpus_z000079` (`7,321`)
  - only CPU drift/noise remained on the fast screen
- Conclusion:
  - the dominant large-`Unknown` `RepeatNextPosition` path is not bottlenecked by sparse post-match suffix insertion
  - the next credible move still needs a different parse/representation change before or during candidate selection

## 2026-05-31 - Rejected a 96 KiB block-size cap for level-1 `Unknown`

- Tried a block-structure change rather than another matcher-local tweak:
  - for `CompressionFileType::Unknown` at level 1, cap block reads at `96 KiB` instead of `128 KiB`
- Why:
  - `decodecorpus_z000079` still differs materially from C in both block and parse shape
- Result:
  - `decodecorpus_z000079`: `7,321 -> 7,772`
  - `build_ruzstd-cli`: `855,679 -> 884,939`
  - repeated-text fixtures regressed badly too
  - `decodecorpus_pack.bin` CPU drifted `0.22s -> 0.25s`
- Conclusion:
  - a smaller fixed block size is the wrong direction for this family
  - any future block-structure move has to be much more selective than a blunt 96 KiB cap

## 2026-06-01 - Corrected `broad-local` suite and rejected dedicated `LockfileText`

- Fixed a benchmark-suite bug in [tools/prepare_benchmark_suites.py](/home/bsutton/git/zstd-rs/tools/prepare_benchmark_suites.py):
  - repo-source fixtures now get unique names instead of silently overwriting collisions such as repeated `Cargo.toml`
  - added more explicit known-file-type fixtures:
    - `repo_Cargo.lock`
    - `repo_ci.yml`
    - `repo_ruzstd_fuzz_.gitignore`
    - `repo_ruzstd_fuzz_Cargo.toml`
- Corrected broad-local baseline exposed a much larger known-file-type gap than the pre-fix suite:
  - current retained summary vs C `zstd -1`:
    - `31 / 12 / 4` better / worse / equal
    - `1,411` total bytes above C on losing fixtures
  - top corrected-suite losses:
    - `repo_Cargo.lock`: `9,240` vs `8,088` (`+1,152`)
    - `repo_compressed.rs`: `13,111` vs `13,007` (`+104`)
    - `decodecorpus_z000079`: `7,321` vs `7,221` (`+100`)
- Retained policy improvement:
  - suffix-based named-file matching in `compression_file_type_for_path()`
  - keeps synthetic benchmark names like `repo_.gitignore` and `repo_Cargo.lock` on the intended file-type path
- Retained `Cargo.lock` starting point:
  - `Cargo.lock -> DictionaryText`
  - `repo_Cargo.lock`: `9,255 -> 9,240`
- Rejected follow-up:
  - dedicated public `CompressionFileType::LockfileText`
  - regressed the target:
    - `repo_Cargo.lock`: `9,240 -> 9,288`
  - widened corrected-suite bytes-above-C on losers:
    - `1,411 -> 1,459`
  - restore check matched the retained baseline exactly after revert
- Current next target:
  - stay on known file types first
  - focus `Cargo.lock` with a narrower lockfile/TOML-family internal strategy rather than another public family split

## 2026-06-01 - Rejected two narrow lockfile-like matcher cuts, retained a larger `CodeText` dense-probe cutoff

- Tested two content-detected matcher cuts inside the retained `Cargo.lock -> DictionaryText` policy:
  1. raise the short-line non-repeat floor from `5` to `7`
  2. keep the retained floor but use short-line probe step `2` instead of dense step `1`
- Both were clean rejects:
  - floor `7`:
    - `repo_Cargo.lock`: `9,240 -> 9,288`
  - probe step `2`:
    - `repo_Cargo.lock`: `9,240 -> 9,255`
- Conclusion:
  - the obvious “make lockfile text less eager” matcher cuts are closed
  - the remaining `Cargo.lock` gap needs a different internal representation or entropy-side decision, not a blunter floor/probe reduction

- Pivoted to the next biggest known-file-type miss:
  - `repo_compressed.rs`
- Fresh archive comparison versus C:
  - us:
    - `literal_section_bytes=4678`
    - `sequence_payload_bytes=8413`
    - `sequences=3505`
  - C:
    - `literal_section_bytes=4413`
    - `sequence_payload_bytes=8571`
    - `sequences=3672`
- Key read:
  - `repo_compressed.rs` was slightly under-sequenced, not over-sequenced
  - it sits just above the retained `CodeText` dense-probe cutoff

- Retained change:
  - widened `CodeText` dense short-line probing cutoff from `64 KiB` to `96 KiB`
- Result:
  - `repo_compressed.rs`: `13,111 -> 12,946`
  - C `zstd -1`: `13,007`
  - corrected broad-local summary vs C improved:
    - `31 / 12 / 4 -> 32 / 11 / 4`
    - bytes-above-C on losers: `1,411 -> 1,307`

- Current top corrected-suite losses after this retained point:
  - `repo_Cargo.lock`: `9,240` vs `8,088` (`+1,152`)
  - `decodecorpus_z000079`: `7,321` vs `7,221` (`+100`)
  - `dict_dictionary.bin`: `20,160` vs `20,145` (`+15`)

- One more lockfile-only follow-up was rejected after that:
  - widen the retained dictionary same-start smaller-offset rule from 1 byte of allowed match loss to 2 bytes, but only for Cargo-lock-like text
  - result:
    - `repo_Cargo.lock`: `9,240 -> 9,243`
    - corrected broad-local bytes-above-C on losers: `1,307 -> 1,310`
  - conclusion:
    - the lockfile-specific smaller-offset family is closed in the current matcher representation

- Two more follow-ups also closed cleanly after that:
  1. enable the best-text repeat pipeline for `DictionaryText` text blocks at level 1
     - exact byte-for-byte no-op on corrected `broad-local`
     - `repo_Cargo.lock` stayed `9,240`
  2. emit known-size single-segment frame headers for whole-file CLI compression
     - broadly made outputs 1 to 3 bytes larger
     - examples:
       - `repo_Cargo.lock`: `9,240 -> 9,241`
       - `repo_compressed.rs`: `12,946 -> 12,949`
       - `decodecorpus_z000079`: `7,321 -> 7,324`

- Restore check after rejecting those branches matched the retained `CodeText 96 KiB` baseline exactly:
  - [restore broad-local](/home/bsutton/git/zstd-rs/benchmarks/reports/zstd-bench-restore-level1-codeprobe96k-broad-local.md)

- Also rejected one narrower entropy-side `Cargo.lock` branch:
  - lower `DictionaryText offset_table_max_log` from `7` to `6`, but only for small-sequence blocks (`<= 1024`)
  - rationale:
    - `repo_Cargo.lock` has `836` sequences and a large offset-side gap
    - `dict_dictionary.bin` has over `4,000` sequences, so this gate should have left the large dictionary case untouched
  - result:
    - `repo_Cargo.lock`: `9,240 -> 9,292`
    - corrected broad-local bytes-above-C on losers: `1,307 -> 1,359`
  - conclusion:
    - small-sequence OF-table tightening is the wrong direction for the lockfile family too

## 2026-06-01 - Rejected lockfile-specific repeat block-end early-exit change

- Stayed on the retained lockfile-specific `oldest +2` baseline:
  - `repo_Cargo.lock = 9,114`
- Tried one more narrow structural cut on the active parser shape:
  - disable repeat block-end early-exit only for lockfile-like `DictionaryText`

- Result:
  - focused `Cargo.lock` size was an exact no-op:
    - `repo_Cargo.lock`: `9,114 -> 9,114`
  - matcher diagnostics moved slightly, but the compressed bytes did not

- Restore:
  - sequential restore check confirmed the source tree is back on the retained point:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-norepeatblockend-seq.md`
    - `repo_Cargo.lock = 9,114`

- Useful conclusion:
  - the remaining `Cargo.lock` gap is not waiting on repeat block-end early-exit
  - do not retry this branch in the current lockfile parser shape

## 2026-06-01 - Expanded file-type coverage and `broad-local` corpus for additional lockfiles/config files

- Expanded path/file-name classification in `ruzstd/src/encoding/mod.rs`:
  - `DictionaryText` now also recognizes:
    - `yarn.lock`
    - `poetry.lock`
    - `pipfile.lock`
    - `gemfile.lock`
    - `composer.lock`
    - `podfile.lock`
    - `mix.lock`
    - `go.sum`
    - `bun.lock`
  - `ConfigText` now also recognizes:
    - `.dockerignore`
    - `.npmrc`
    - `.prettierignore`
    - `.eslintignore`
    - `requirements.txt`
    - `go.mod`
    - `Gemfile`
    - `Pipfile`

- Added generated `broad-local` fixtures in `tools/prepare_benchmark_suites.py`:
  - `generated_yarn.lock`
  - `generated_poetry.lock`
  - `generated_go.sum`
  - `generated_requirements.txt`

- Refreshed `broad-local` current baseline:
  - `51` fixtures total
  - `34 / 13 / 4` better / worse / equal vs C `zstd -1`
  - `1,216` bytes-above-C on losers

- New useful signal:
  - the suite now exposes more lockfile-family misses directly:
    - `generated_poetry.lock`: `386` vs `362` (`+24`)
    - `generated_yarn.lock`: `403` vs `393` (`+10`)

- Useful conclusion:
  - file-type surface coverage is materially better
  - the next lockfile branch should be judged against more than `Cargo.lock`

## 2026-06-01 - Rejected Poetry-style lockfile detector widening; retained evidence-based lockfile-name trimming

- After expanding the corpus, two new lockfile-family misses were visible:
  - `generated_poetry.lock`: `386` vs C `362` (`+24`)
  - `generated_yarn.lock`: `403` vs C `393` (`+10`)

- Tried one internal content-detector widening:
  - broaden `likely_lockfile_text()` so Poetry-style `[[package]]` / `files = [` blocks would enter the retained Cargo-lock-specific parser path

- Result:
  - this did not improve either file at all on the active mapped path
  - `generated_poetry.lock` stayed `386`
  - `generated_yarn.lock` stayed `403`

- Useful read:
  - the real issue was not missing detector admission
  - it was public over-classification:
    - both `yarn.lock` and `poetry.lock` were worse as `DictionaryText` than on the generic path

- Retained fix:
  - remove `yarn.lock` and `poetry.lock` from the `DictionaryText` named-file mapping

- Result:
  - `generated_poetry.lock`: `386 -> 371`
  - `generated_yarn.lock`: `403 -> 398`
  - no other fixture moved
  - current broad-local summary vs C `zstd -1`:
    - `34 / 13 / 4` better / worse / equal
    - `1,196` bytes-above-C on losers

- Useful conclusion:
  - keep lockfile-name expansion evidence-based
  - `Cargo.lock` and `go.sum` stay mapped specially
  - the right next question is not “generic vs DictionaryText” anymore; it is whether those two names want another existing text family

## 2026-06-01 - Retained `poetry.lock` and `yarn.lock` as `ConfigText`

- After trimming the harmful `DictionaryText` mapping for `poetry.lock` and `yarn.lock`, tested the next narrow public-policy move:
  - map both names to `ConfigText`

- Result:
  - `generated_poetry.lock`: `371 -> 362`
  - `generated_yarn.lock`: `398 -> 390`
  - no other fixture moved

- Current broad-local summary vs C `zstd -1`:
  - `35 / 11 / 5` better / worse / equal
  - `1,182` bytes-above-C on losers

- Useful conclusion:
  - those two names are better served by the existing `ConfigText` path than by either `DictionaryText` or the generic path
  - this is a clean retained file-type-policy improvement, not another heuristic branch

## 2026-06-01 - Rejected `Cargo.lock` as `ConfigText` and `CodeText`

- Re-tested the public starting family for `Cargo.lock` directly on the current retained parser shape.
- Tried:
  1. `Cargo.lock -> ConfigText`
  2. `Cargo.lock -> CodeText`

- Result:
  - both are materially worse than the retained `DictionaryText` path:
    - `ConfigText`: `repo_Cargo.lock 9,114 -> 9,255`
    - `CodeText`: `repo_Cargo.lock 9,114 -> 9,240`

- Useful matcher read:
  - both generic text families lose the retained lockfile-specific current-entry behavior
  - `second_newest` activity disappears entirely

- Restore:
  - sequential restore check after the `CodeText` reject confirmed the source tree is back on the retained point:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-cargolock-code-seq.md`
    - `repo_Cargo.lock = 9,114`

- Useful conclusion:
  - the public-family branch for `Cargo.lock` is now closed
  - `DictionaryText` remains the correct starting family for that file on the current design

## 2026-06-01 - Rejected two more internal lockfile parser branches

- Stayed on the retained `Cargo.lock` `DictionaryText` path and tested:
  1. require one extra match byte for repeat candidates on the lockfile-specific path
  2. retest dense lockfile probing (`step 1`) on the current retained parser shape

- Result:
  - repeat-floor branch regressed:
    - `repo_Cargo.lock`: `9,114 -> 9,117`
  - step-1 retest also regressed:
    - `repo_Cargo.lock`: `9,114 -> 9,118`

- Restore:
  - sequential restore check confirmed the source tree is back on the retained point:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-restore-after-step1-retest-seq.md`
    - `repo_Cargo.lock = 9,114`

- Useful conclusion:
  - the active retained lockfile parser does not want a stricter repeat floor
  - and it still does not want dense probe step `1`

- Small retained test hygiene:
  - kept missing `#[test]` annotations fixed on two focused matcher tests in `match_generator.rs`

## 2026-06-01 - Rejected stronger repeat-vs-normal margin on the retained lockfile path

- Stayed on the retained `Cargo.lock` parser and widened the repeat-vs-normal scoring margin only for the lockfile-specific `DictionaryText` path.

- Result:
  - exact byte-for-byte no-op
  - `repo_Cargo.lock` stayed `9,114`

- Useful conclusion:
  - the current lockfile path is not waiting on a larger repeat-vs-normal margin either

## 2026-06-01 - Rejected wider lockfile `second_newest` recent-entry reach

- Stayed on the retained `Cargo.lock` parser shape and widened the lockfile-only `second_newest` recent-entry limit from `2` to `3`.
- Added a focused matcher test first, then benchmarked `repo_Cargo.lock` directly against the retained baseline binary.

- Result:
  - exact byte-for-byte no-op on focused `Cargo.lock`
  - `repo_Cargo.lock`: stayed `9,114`

- Useful conclusion:
  - the retained lockfile `second_newest` path does not benefit from reaching one more older current-window entry
  - the next credible `Cargo.lock` branch is not a wider recent-entry reach in this family

## 2026-06-01 - Rejected three more focused `Cargo.lock` branches

- Stayed on the retained `Cargo.lock` parser shape and tested:
  1. lockfile-only fastest whole-vs-estimated-split candidate using the existing best-level partition machinery
  2. lockfile-only zero-literal non-repeat floor `+1`
  3. `DictionaryText` exact Huffman search also evaluating flat-distribution max-bit variants

- Result:
  - partition candidate: exact no-op
    - `repo_Cargo.lock`: stayed `9,114`
  - zero-literal floor `+1`: regression
    - `repo_Cargo.lock`: `9,114 -> 9,143`
  - flat-distribution exact search: exact no-op
    - `repo_Cargo.lock`: stayed `9,114`

- Useful conclusion:
  - the current `Cargo.lock` path is not waiting on:
    - a best-level-style split candidate
    - a stricter zero-literal non-repeat floor
    - flat-distribution exact Huffman table search

## 2026-06-01 - Rejected TOML single-stream exception and DictionaryText `newest` displacement

- Tried a narrow `ConfigText` entropy policy exception:
  - keep the retained small-config single-stream Huffman rule generally
  - but skip it when the literal payload looks like a TOML manifest, so `Cargo.toml` can use the normal 4-stream path
- Also tried a narrow `DictionaryText` matcher rule:
  - keep the current non-repeat candidate over a farther `newest` window hit when the farther hit costs at least 4 more offset-code bits and gains less than 2 match bytes

- Result:
  - TOML single-stream exception regressed:
    - `repo_ruzstd_Cargo.toml`: `730 -> 737`
  - Dictionary `newest` displacement was an exact no-op:
    - `dict_dictionary.bin`: stayed `20,160`

- Useful conclusion:
  - the retained `ConfigText` single-stream policy should stay intact for now
  - the remaining dictionary gap is not waiting on this `newest`-side current-window rule

## 2026-06-01 - Rejected generic DictionaryText probe step `1 -> 2`

- Re-tested the generic `DictionaryText` probe-density family with the lockfile path left unchanged.
- Changed only non-lockfile `DictionaryText` short-line probing from dense step `1` to step `2`.

- Result:
  - hard regression on the target dictionary fixture:
    - `dict_dictionary.bin`: `20,160 -> 20,667`

- Useful conclusion:
  - generic `DictionaryText` still wants fully dense probe step `1`

## 2026-06-01 - Retained adaptive Huffman weight-table FSE max-log choice

- Kept the existing `huff0` helper in `ruzstd/src/huff0/huff0_encoder.rs`:
  - for Huffman weight tables longer than `16` symbols, compare FSE weight-table encodings at max-log `6` and `5`
  - emit the shorter byte sequence
- Added focused unit coverage for the helper.

- Focused tiny known-file-type literals stayed byte-identical:
  - `repo_.gitignore`: `172`
  - `dict_talk.service`: `160`
  - `repo_ruzstd_Cargo.toml`: `730`
  - `repo_ci.yml`: `556`

- Corrected `broad-local` A/B versus a fixed-max-log-6 revert:
  - `build_ruzstd-cli`: `866,125 -> 866,118`
  - `repo_match_generator.rs`: `27,879 -> 27,877`
  - corrected broad-local bytes-above-C on losers stayed `1,182`

- Reports:
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/huffweight-focused.md`
  - `benchmarks/reports/zstd-bench-compare-level1-huffweight-maxlog5-broad-local.md`

- Why keep it:
  - it is broad-local clean
  - it slightly improves two already-winning fixtures
  - it does not disturb the known-file-type gap accounting

## 2026-06-01 - Rejected lockfile package-boundary partition candidates

- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,114`
- Tested two structural partition branches inside the retained `DictionaryText` lockfile path:
  1. split once at the `[[package]]` boundary nearest the midpoint
  2. split at the `[[package]]` boundaries nearest the quartiles

- Result:
  - exact byte-for-byte no-op on the focused lockfile family
  - `repo_Cargo.lock`: stayed `9,114`
  - `generated_go.sum`: stayed `151`
  - `generated_poetry.lock`: stayed `362`
  - `generated_yarn.lock`: stayed `390`

- Reports:
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-split-focused.md`
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-split-restore.md`

- Useful conclusion:
  - the retained `Cargo.lock` gap is not waiting on package-boundary block partitioning in this form
  - that closes another structural parse family, not just another matcher threshold

## 2026-06-01 - Rejected Huffman weight-table FSE max-log `7`

- Stayed on the retained live baseline with adaptive Huffman weight-table FSE max-log `5/6`.
- Tested one narrow literal-side branch in `ruzstd/src/huff0/huff0_encoder.rs`:
  - extend the retained weight-table search to also consider FSE max-log `7`

- Result:
  - this produced invalid output on the focused lockfile family
  - `tools/benchmark_zstd.py` failed decode verification on `generated_go.sum.current.zst`

- Restore:
  - after reverting the branch, the focused lockfile family returned to the retained baseline:
    - `repo_Cargo.lock = 9,114`
    - `generated_go.sum = 151`
    - `generated_poetry.lock = 362`
    - `generated_yarn.lock = 390`
  - restore report:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-huffweight7-restore.md`

- Useful conclusion:
  - the retained Huffman weight-table FSE search should stay bounded at max-log `5/6`
  - do not retry `>6` in this branch without evidence that the emitted representation is valid

## 2026-06-01 - Rejected lockfile zero-literal window displacement rule and kept literal-payload diagnostics

- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,114`
- Added a retained archive-inspector diagnostic in `ruzstd/src/tests/mod.rs`:
  - compressed literal blocks now print:
    - `literals_table_desc`
    - `literals_stream`
- Tested one narrow lockfile-specific matcher rule:
  - keep the current non-repeat candidate over a zero-literal non-repeat window candidate unless the zero-literal candidate gains at least `2` bytes

- Result:
  - exact no-op on focused `Cargo.lock`
  - `repo_Cargo.lock`: stayed `9,114`
  - restore report:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-zero-lit-restore.md`

- Useful new evidence from the retained inspector:
  - current `Cargo.lock`:
    - `literals_payload=6886`
    - `literals_table_desc=25`
    - `literals_stream=6855`
  - C `Cargo.lock`:
    - `literals_payload=5975`
    - `literals_table_desc=39`
    - `literals_stream=5930`

- Useful conclusion:
  - the `Cargo.lock` literal gap is not a Huffman table-description problem
  - C actually spends more bytes on the table description
  - the remaining loss is in the coded literal stream itself, so the next credible branch is still parse/literal-shape oriented, not another table-header tweak

## 2026-06-01 - Rejected lockfile zero-literal `second_newest` ordering change

- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,114`
- Tested one narrow lockfile-specific parse rule:
  - keep the retained `second_newest-before-newest` ordering only when literals are pending
  - on zero-literal positions, fall back to the normal `newest`-first order

- Result:
  - focused `Cargo.lock` regressed immediately:
    - `9,114 -> 9,164`
  - restore report:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-zero-lit-restore.md`

- Useful conclusion:
  - zero-literal `second_newest` wins are not the main reason `Cargo.lock` lags C
  - this closes another zero-literal parse-shape branch on the retained lockfile path

## 2026-06-01 - Rejected rank-limited candidate in DictionaryText exact Huffman search

- Stayed on the retained live baseline for the `DictionaryText` family.
- Tested one deeper literal-model branch in `ruzstd/src/huff0/huff0_encoder.rs`:
  - when `build_smallest_from_counts()` searches exact Huffman tables for non-flat distributions, also consider the `rank_limited_weights()` candidate

- Result:
  - exact byte-for-byte no-op across the focused live `DictionaryText` family:
    - `repo_Cargo.lock`: stayed `9,114`
    - `dict_dictionary.bin`: stayed `20,160`
    - `generated_go.sum`: stayed `151`
  - restore report:
    - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/dictionary-huff-restore.md`

- Useful conclusion:
  - the remaining `DictionaryText` gap is not waiting on the rank-limited weight family being added to the current exact Huffman search
  - this closes another literal-model branch for both `Cargo.lock` and `dict_dictionary.bin`

## 2026-06-01 - Rejected lockfile zero-literal high-offset filter

- Stayed on the retained live `Cargo.lock` baseline:
  - `repo_Cargo.lock = 9,114`
- Tested one narrow lockfile-only matcher branch in `ruzstd/src/encoding/match_generator.rs`:
  - reject zero-literal non-repeat window candidates when they are only `5` bytes long and cost at least `11` offset-code bits

- Result:
  - focused lockfile family regressed only on the main target:
    - `repo_Cargo.lock`: `9,114 -> 9,132`
    - `generated_go.sum`: stayed `151`
    - `generated_poetry.lock`: stayed `362`
    - `generated_yarn.lock`: stayed `390`

- Report:
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-zero-literal-high-offset-focused.md`

- Useful conclusion:
  - the retained `Cargo.lock` parser does not want this high-offset zero-literal filter
  - the remaining lockfile gap is not improved by suppressing these short high-offset non-repeat window matches in this form

## 2026-06-01 - Retained rank-limited exact Huffman candidate

- Reintroduced a shared literal-model branch in `ruzstd/src/huff0/huff0_encoder.rs`:
  - when `build_smallest_from_counts()` searches exact Huffman tables, also evaluate the `rank_limited_weights()` candidate and keep it if the fully encoded literal section is shorter
- This was screened first on the tiny config-like and lockfile sentinels, then promoted to `broad-local`.

- Focused result:
  - `repo_.gitignore`: `172 -> 166`
  - `dict_talk.service`: `160 -> 151`
  - `generated_poetry.lock`: `362 -> 359`
  - `repo_Cargo.lock`: stayed `9,114`
  - `repo_ruzstd_Cargo.toml`: stayed `730`

- Live literal evidence:
  - `repo_.gitignore`
    - before: `literals_payload=137`, `literals_table_desc=32`, `literals_stream=105`
    - after: `literals_payload=131`, `literals_table_desc=22`, `literals_stream=109`
    - C: `literals_payload=129`, `literals_table_desc=24`, `literals_stream=105`
  - `dict_talk.service`
    - before: `literals_payload=130`, `literals_table_desc=36`, `literals_stream=94`
    - after: `literals_payload=121`, `literals_table_desc=25`, `literals_stream=96`

- Broad-local A/B vs the retained `huffweight-maxlog5` baseline:
  - no regressions
  - notable wins:
    - `dict_talk.service`: `160 -> 151`
    - `repo_.gitignore`: `172 -> 166`
    - `generated_yarn.lock`: `390 -> 383`
    - `generated_poetry.lock`: `362 -> 359`
    - `dict_git-daemon@.service`: `241 -> 237`
    - `dict_glustereventsd.service`: `285 -> 281`
    - `dict_gpm.service`: `191 -> 190`

- Fast guardrails stayed flat:
  - `build_ruzstd-cli`: `854,529`
  - `decodecorpus_z000079`: `7,322`
  - `dict_dictionary.bin`: `20,160`
  - `generated_json_logs_001m.jsonl`: `58,767`
  - `repo_Cargo.lock`: `9,114`

- Current corrected `broad-local` summary vs C:
  - `37 / 10 / 4` better / worse / equal
  - `1,170` total bytes above C on the losing fixtures

- Useful conclusion:
  - this is a real retained literal-model win for known file types
  - it improves the tiny literal-header tail without disturbing the active lockfile path
  - the next best branch is still `Cargo.lock`, but the tiny config/config-like literal tail is materially better now

## 2026-06-01 - Rejected lockfile stream-first Huffman search

- Stayed on the retained `config-ranklimited` baseline:
  - `repo_Cargo.lock = 9,114`
- Tested one narrow lockfile-only literal branch:
  - for `Cargo.lock`-like `DictionaryText`, keep the current exact-table candidate set but choose the new Huffman table by coded stream size instead of total literal payload size
- This remained internal to `compress_literals()` and still competed normally against the repeat-table path on total estimated size.

- Result:
  - exact byte-for-byte no-op on the focused lockfile family:
    - `repo_Cargo.lock`: stayed `9,114`
    - `generated_go.sum`: stayed `151`
    - `generated_poetry.lock`: stayed `359`
    - `generated_yarn.lock`: stayed `383`

- Reports:
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-streamfirst-focused.md`
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-streamfirst-restore.md`

- Useful conclusion:
  - the retained lockfile gap is not waiting on a stream-first re-ranking of the current exact Huffman candidate set
  - this closes another lockfile literal-model family without changing the retained baseline

## 2026-06-01 - Retained generic DictionaryText current-entry second_newest

- In `ruzstd/src/encoding/match_generator.rs`, non-lockfile text-like `DictionaryText` blocks now track and probe the current-entry `second_newest` sidecar at level 1.
- This uses the existing current-entry `second_newest` machinery already retained for `Cargo.lock`, but it leaves the special lockfile ordering alone.

- Focused result:
  - `dict_dictionary.bin`: `20,160 -> 19,668`
  - `repo_Cargo.lock`: stayed `9,114`
  - `generated_go.sum`: stayed `151`
  - `generated_poetry.lock`: stayed `359`
  - `generated_yarn.lock`: stayed `383`

- Live matcher evidence on `dict_dictionary.bin`:
  - before:
    - `window_current_second_newest[0] = 0`
    - `window_current_newest[0] = 2527`
    - `window_current_oldest[0] = 1710`
  - after:
    - `window_current_second_newest[0] = 604`
    - `window_current_second_newest_zero_literals[0] = 378`
    - `window_current_newest[0] = 2417`
    - `window_current_oldest[0] = 1330`

- Broad-local A/B vs the retained `config-ranklimited` baseline:
  - only one fixture moved:
    - `dict_dictionary.bin`: `20,160 -> 19,668`

- Fast guardrails:
  - flat outside that same fixture
  - `build_ruzstd-cli`: `854,529`
  - `decodecorpus_z000079`: `7,322`
  - `repo_Cargo.lock`: `9,114`

- Current corrected `broad-local` summary vs C:
  - `38 / 9 / 4` better / worse / equal
  - `1,155` total bytes above C on the losing fixtures

- Useful conclusion:
  - this is a clean retained DictionaryText parser win
  - it turns `dict_dictionary.bin` from a small loser into a meaningful win versus C
  - the remaining known-file-type gap is now even more concentrated in `Cargo.lock`

## 2026-06-01 - Rejected lockfile structural midpoint split

- Stayed on the retained `dict-secondnewest` baseline:
  - `repo_Cargo.lock = 9,114`
- Tested one structural lockfile-only branch in `ruzstd/src/encoding/levels/fastest.rs`:
  - if a large `Cargo.lock`-like `DictionaryText` block is seen at level 1, split it once at the newline nearest the midpoint and compress the two halves as separate blocks

- Result:
  - exact byte-for-byte no-op on the focused lockfile family:
    - `repo_Cargo.lock`: stayed `9,114`
    - `generated_go.sum`: stayed `151`
    - `generated_poetry.lock`: stayed `359`
    - `generated_yarn.lock`: stayed `383`

- Reports:
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-structuralsplit-focused.md`
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-structuralsplit-restore-seq.md`

- Useful conclusion:
  - the retained `Cargo.lock` gap is not waiting on a forced two-block midpoint split in this form
  - this closes another structural lockfile representation family without changing the retained baseline

## 2026-06-01 - Rejected lockfile zero-literal short second_newest filter

- Stayed on the retained `dict-secondnewest` baseline:
  - `repo_Cargo.lock = 9,114`
- Tested one narrow lockfile-only matcher branch in `ruzstd/src/encoding/match_generator.rs`:
  - reject lockfile `second_newest` window candidates when they are zero-literal, non-repeat, and only `5` bytes long

- Result:
  - exact byte-for-byte no-op on the focused lockfile family:
    - `repo_Cargo.lock`: stayed `9,114`
    - `generated_go.sum`: stayed `151`
    - `generated_poetry.lock`: stayed `359`
    - `generated_yarn.lock`: stayed `383`

- Reports:
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-secondnewest-zerolit-floor-focused.md`
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-secondnewest-zerolit-floor-restore-seq.md`

- Useful conclusion:
  - the retained lockfile gap is not waiting on a higher minimum length for zero-literal `second_newest` wins in this form
  - this closes another narrow lockfile zero-literal matcher family without changing the retained baseline

## 2026-06-01 - Rejected Cargo.toml -> CodeText

- Stayed on the retained `dict-secondnewest` baseline.
- Tested one narrow known-file-type mapping branch in `ruzstd/src/encoding/mod.rs`:
  - map `Cargo.toml` filenames to `CodeText` instead of `ConfigText`

- Result:
  - exact byte-for-byte no-op on the focused `Cargo.toml` family:
    - `repo_Cargo.toml`: stayed `68`
    - `repo_cli_Cargo.toml`: stayed `489`
    - `repo_ruzstd_Cargo.toml`: stayed `730`
    - `repo_ruzstd_fuzz_Cargo.toml`: stayed `340`

- Reports:
  - `benchmarks/archive/tmp/cargotoml-code-focused.md`
  - `benchmarks/archive/tmp/cargotoml-code-restore-seq.md`

- Useful conclusion:
  - on the current retained baseline, `Cargo.toml` is not improved by switching from `ConfigText` to `CodeText`
  - this closes another known-file-type mapping branch without changing the retained baseline

## 2026-06-01 - Rejected composer.lock / Pipfile.lock -> ConfigText

- Stayed on the retained `dict-secondnewest` baseline:
  - `repo_Cargo.lock = 9,114`
- Tested one narrow known-file-type mapping branch in `ruzstd/src/encoding/mod.rs`:
  - map `composer.lock` and `Pipfile.lock` to `ConfigText` instead of `DictionaryText`

- Focused result:
  - regression on both remapped targets:
    - `generated_composer.lock`: `4,461 -> 4,469`
    - `generated_pipfile.lock`: `2,811 -> 2,879`
  - unchanged nearby controls:
    - `generated_package-lock.json`: stayed `4,392`
    - `generated_go.sum`: stayed `151`
    - `repo_Cargo.lock`: stayed `9,114`

- Reports:
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/configlock-focused.md`

- Useful conclusion:
  - the current `ConfigText` path is the wrong public starting point for `composer.lock` and `Pipfile.lock`
  - `generated_composer.lock` remains a real known-file-type target, but it needs a different family or an internal dictionary/lockfile path change rather than this plain remap

## 2026-06-01 - Rejected three focused composer.lock internal parser branches

- Stayed on the retained `dict-secondnewest` baseline:
  - `generated_composer.lock = 4,461`
- Screened the focused lockfile family only:
  - `generated_composer.lock`
  - `generated_pipfile.lock`
  - `generated_package-lock.json`
  - `generated_go.sum`
  - `repo_Cargo.lock`

- Rejected:
  - treat large composer-style JSON lockfiles as the retained lockfile parser path inside `DictionaryText`
  - raise the non-repeat floor to `6` for large composer-style `DictionaryText`
  - prefer smaller non-repeat offsets for composer-style `DictionaryText` when the farther match only gains a byte

- Result:
  - all three were exact byte-for-byte no-ops on the focused family:
    - `generated_composer.lock`: stayed `4,461`
    - `generated_pipfile.lock`: stayed `2,811`
    - `generated_package-lock.json`: stayed `4,392`
    - `generated_go.sum`: stayed `151`
    - `repo_Cargo.lock`: stayed `9,114`

- Reports:
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-lockpath-focused.md`
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-floor6-focused.md`
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-offset-focused.md`
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-restore.md`

- Useful conclusion:
  - `generated_composer.lock` is not moving on another small internal parser threshold
  - the gap still points at a different sequence/repeat representation, not a public remap and not these local matcher-floor or smaller-offset variants

## 2026-06-01 - Rejected composer current-entry long-hash path

- Stayed on the retained `dict-secondnewest` baseline:
  - `generated_composer.lock = 4,461`
- Tested one focused internal matcher branch in `ruzstd/src/encoding/match_generator.rs`:
  - enable the current-entry long-hash path for large composer-style `DictionaryText`

- Result:
  - exact byte-for-byte no-op on the focused family:
    - `generated_composer.lock`: stayed `4,461`
    - `generated_pipfile.lock`: stayed `2,811`
    - `generated_package-lock.json`: stayed `4,392`
    - `generated_go.sum`: stayed `151`
    - `repo_Cargo.lock`: stayed `9,114`

- Reports:
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-longhash-focused.md`
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-longhash-restore.md`

- Useful conclusion:
  - the remaining composer gap is not waiting on the current-entry long-hash family either
  - this tightens the read that the live matcher search space is largely exhausted for composer on the current representation

## 2026-06-01 - Rejected composer zero-literal rep1-1 ordering

- Stayed on the retained `dict-secondnewest` baseline:
  - `generated_composer.lock = 4,461`
- Tested one focused internal repeat branch in `ruzstd/src/encoding/match_generator.rs`:
  - for composer-style `DictionaryText`, try the zero-literal `rep1-1` candidate first instead of last

- Result:
  - exact byte-for-byte no-op on the focused family:
    - `generated_composer.lock`: stayed `4,461`
    - `generated_pipfile.lock`: stayed `2,811`
    - `generated_package-lock.json`: stayed `4,392`
    - `generated_go.sum`: stayed `151`
    - `repo_Cargo.lock`: stayed `9,114`

- Reports:
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-repfirst-focused.md`
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-repfirst-restore.md`

- Useful conclusion:
  - the remaining composer gap is not waiting on zero-literal repcode ordering either
  - this is another sign that the live matcher search space is largely exhausted for composer on the current representation

## 2026-06-01 - Rejected composer ip+1 repeat comparison branch

- Stayed on the retained `dict-secondnewest` baseline:
  - `generated_composer.lock = 4,461`
- Tested one focused internal repeat branch in `ruzstd/src/encoding/match_generator.rs`:
  - for composer-style `DictionaryText`, allow `ip+1` repeat candidates to be compared even when a current-position repeat candidate already exists

- Result:
  - exact byte-for-byte no-op on the focused family:
    - `generated_composer.lock`: stayed `4,461`
    - `generated_pipfile.lock`: stayed `2,811`
    - `generated_package-lock.json`: stayed `4,392`
    - `generated_go.sum`: stayed `151`
    - `repo_Cargo.lock`: stayed `9,114`

- Reports:
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-nextrep-focused.md`
  - `/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/composer-nextrep-restore.md`

- Useful conclusion:
  - the remaining composer gap is not waiting on the `ip+1` repeat family either
  - the repeat-search space is now very tight on the current representation

## 2026-06-01 - Pivoted to classifier expansion and sampled fallback

- Retained in `ruzstd/src/encoding/mod.rs`:
  - indexed exact-match classification helpers for file names and extensions
  - broader known extension and named-file coverage
  - compound extension support
  - `compression_file_type_for_path_and_data()`
  - `compress_with_path()` now samples up to `32 KiB` before falling back to `Unknown`

- Retained in `tools/prepare_benchmark_suites.py`:
  - new known-file fixtures:
    - `generated_package.json`
    - `generated_tsconfig.json`
    - `generated_pyproject.toml`
    - `generated_pom.xml`
    - `generated_Dockerfile`

- Refreshed `broad-local` baseline:
  - `62` fixtures
  - `43 / 15 / 4` better / worse / equal vs C
  - `2,421` bytes above C on the losing fixtures

- Main remaining losses after the classifier/corpus expansion:
  - `repo_Cargo.lock`: `+1,026`
  - `generated_composer.lock`: `+570`
  - `decodecorpus_z000079`: `+309`
  - `generated_package.json`: `+130`
  - `repo_prepare_benchmark_suites.py`: `+125`

- Current direction from user:
  - prioritize expanding known-file classification
  - keep scaling the extension/name surface
  - use sampled fallback only when path matching misses

## 2026-06-01 - Second classifier pass

- Retained:
  - confidence-based sampled fallback in `ruzstd/src/encoding/mod.rs`
    - removed generic plain-text fallback to `ConfigText`
    - ambiguous sampled text now stays `Unknown`
  - extra special-name coverage:
    - `BUILD.bazel`
    - `MODULE.bazel`
    - `WORKSPACE`
    - `pubspec.yaml`
    - `pubspec.lock`
    - `melos.yaml`
    - `Podfile`
    - `Brewfile`

- Added new known-file fixtures:
  - `generated_pubspec.yaml`
  - `generated_pubspec.lock`
  - `generated_BUILD.bazel`
  - `generated_WORKSPACE`

- Refreshed `broad-local` baseline:
  - `66` fixtures
  - `46 / 16 / 4` better / worse / equal vs C
  - `2,122` bytes above C on the losing fixtures

- Important recovery from the fallback tightening:
  - `decodecorpus_z000079`: back to `7,322` from the overly eager fallback `7,530`

- Current largest losses:
  - `repo_Cargo.lock`: `+1,026`
  - `generated_composer.lock`: `+570`
  - `repo_prepare_benchmark_suites.py`: `+145`
  - `generated_package.json`: `+130`
  - `decodecorpus_z000079`: `+101`

- Next likely direction:
  - keep expanding exact path/name coverage
  - avoid weak sample-based claims
  - start tuning the newly exposed known JSON/config tails (`generated_package.json`, `generated_tsconfig.json`, `repo_prepare_benchmark_suites.py`) alongside the larger retained lockfile gaps

## 2026-06-01 - Closed two JSON-config branches

- Rejected plain family remap:
  - `package.json` / `tsconfig.json` / `jsconfig.json` / `composer.json` -> `JsonText`
  - result on exposed fixtures:
    - `generated_package.json`: no change at `3,956`
    - `generated_tsconfig.json`: no change at `2,492`

- Rejected matcher floor branch:
  - structured-JSON `ConfigText` short-line non-repeat floor `7`
  - result:
    - `generated_package.json`: `3,956 -> 3,960`
    - `generated_tsconfig.json`: `2,492 -> 3,292`

- Restored retained source confirmed:
  - `generated_package.json = 3,956`
  - `generated_tsconfig.json = 2,492`

- Current read:
  - the JSON-config tail is parser/sequence-shape related
  - not another plain family remap
  - not a blunt stronger short-line floor

## 2026-06-01 - Closed repeat-code-rank-only JSON branch

- Rejected:
  - zero-literal structured-JSON `ConfigText` cheaper-repeat-code bias

- Why it was plausible:
  - `generated_tsconfig.json` was overwhelmingly choosing the third repeat candidate
  - C heavily favors the cheapest repeat code there

- What happened:
  - matcher diagnostics moved a lot:
    - `repeat_current[2]`: `4521 -> 3`
    - `repeat_current[1]`: `2 -> 3015`
    - `repeat_current[0]`: `21 -> 1526`
  - but bytes did not move:
    - `generated_package.json`: stayed `3,956`
    - `generated_tsconfig.json`: stayed `2,492`

- Current read is narrower:
  - repeat-code rank alone is not the missing piece
  - the JSON-config tail needs a deeper zero-literal sequence-shape change, not just a different repeat-code choice inside the same shape

## 2026-06-01 - Closed two more structured-JSON repeat branches

- Rejected:
  1. disable repeat-length early exit on structured JSON `ConfigText`
  2. reject minimum-length zero-literal repeat matches on structured JSON `ConfigText`

- Why they were plausible:
  - `generated_tsconfig.json` was dominated by zero-literal repeats
  - the goal was to let longer window candidates compete and/or force more literal accumulation

- What happened:
  - early-exit disable: exact no-op on bytes
  - zero-literal repeat floor `6`:
    - matcher diagnostics changed
    - `generated_tsconfig.json` total sequences dropped `4567 -> 4522`
    - but bytes still stayed flat at `2,492`

- Current read is tighter again:
  - even reducing the repeat-sequence count slightly was not enough
  - the JSON-config tail needs a more material sequence-shape change than these local repeat gates

## 2026-06-01 - Retained classifier surface expansion

- Retained:
  - broader exact-name classification in `ruzstd/src/encoding/mod.rs`
  - broader extension coverage in `ruzstd/src/encoding/mod.rs`
  - broader `broad-local` fixture coverage in `tools/prepare_benchmark_suites.py`

- New exact-name families now covered:
  - JSON/TOML/YAML configs:
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

- New fixture coverage added:
  - `generated_turbo.json`
  - `generated_deno.json`
  - `generated_nx.json`
  - `generated_wrangler.toml`
  - `generated_buf.yaml`

- Refreshed retained `broad-local` baseline:
  - `71` fixtures
  - `48 / 19 / 4` better / worse / equal vs C
  - `2,449` bytes above C on losing fixtures

- Current largest losses:
  - `repo_Cargo.lock`: `+1,026`
  - `generated_composer.lock`: `+570`
  - `repo_prepare_benchmark_suites.py`: `+140`
  - `generated_package.json`: `+130`
  - `generated_turbo.json`: `+130`
  - `generated_tsconfig.json`: `+101`
  - `generated_deno.json`: `+101`
  - `generated_nx.json`: `+101`
  - `decodecorpus_z000079`: `+101`

- Next likely direction:
  - keep the classifier expansion work, but do not confuse it with compression movement
  - target the exposed known-family tails directly:
    - `Cargo.lock`
    - composer-style lockfiles
    - JSON-config parse shape (`package.json` / `turbo.json`, `tsconfig.json` / `deno.json` / `nx.json`)

## 2026-06-01 - Retained medium-size CodeText second_newest path

- Retained:
  - `CodeText` short-line current-entry `second_newest` probing in
    `ruzstd/src/encoding/match_generator.rs`
  - scope:
    - short-line `CodeText`
    - `16 KiB ..= 64 KiB` blocks

- Why it stays:
  - it fixed a real code-family parser gap on medium-size files:
    - `repo_prepare_benchmark_suites.py`: `7,221 -> 6,827`
    - `repo_match_generator.rs`: `28,078 -> 27,845`
  - no movement on nearby controls:
    - `repo_benchmark_zstd.py`
    - `repo_compressed.rs`
    - `repo_main.rs`

- Useful matcher evidence on `repo_prepare_benchmark_suites.py`:
  - `window_current_second_newest[0]`: `0 -> 72`
  - `window_current_second_newest_zero_literals[0]`: `0 -> 32`
  - `window_current_oldest[0]`: `430 -> 372`

- Refreshed retained `broad-local` baseline:
  - `71` fixtures
  - `49 / 18 / 4` better / worse / equal vs C
  - `2,309` bytes above C on losing fixtures

- Current largest remaining losses:
  - `repo_Cargo.lock`: `+1,026`
  - `generated_composer.lock`: `+570`
  - `generated_package.json`: `+130`
  - `generated_turbo.json`: `+130`
  - `generated_tsconfig.json`: `+101`
  - `generated_deno.json`: `+101`
  - `generated_nx.json`: `+101`
  - `decodecorpus_z000079`: `+101`

- Next likely direction:
  - stay on the exposed known-family tails
  - JSON-config still looks over-sequenced / offset-heavy
  - `Cargo.lock` and composer-style lockfiles are still the largest known-file gaps

## 2026-06-01 - Closed stronger structured-JSON repeat gating branch

- Rejected:
  - structured JSON `ConfigText` zero-literal repeat gating
  - required longer matches on the heavier repeat slots:
    - second repeat candidate min len `8`
    - third repeat candidate min len `10`

- Focused result:
  - no benefit on `package.json` / `turbo.json`
  - hard regressions on the `tsconfig` family:
    - `generated_tsconfig.json`: `2,492 -> 3,446`
    - `generated_deno.json`: `2,492 -> 3,446`
    - `generated_nx.json`: `2,492 -> 3,446`

- Restored retained source confirmed:
  - `generated_package.json = 3,956`
  - `generated_turbo.json = 3,956`
  - `generated_tsconfig.json = 2,492`
  - `generated_deno.json = 2,492`
  - `generated_nx.json = 2,492`

- Current read:
  - JSON-config still looks over-sequenced and offset-heavy
  - but direct repeat-length gating is the wrong lever
  - next JSON-config work should target a different sequence-shape mechanism

## 2026-06-01 - Closed composer current-window offset-choice branch

- Rejected:
  - composer-style `DictionaryText` current-window offset-choice rule
  - keep the current smaller-offset non-repeat candidate over farther `newest` / `oldest`
    current-window candidates unless the farther one gains at least `2` bytes and wins on
    offset-code bits too

- Focused result:
  - exact no-op on the composer family:
    - `generated_composer.lock = 4,336`
    - `generated_pipfile.lock = 2,811`
    - `generated_package-lock.json = 4,392`
    - `generated_go.sum = 151`

- Restored retained source confirmed:
  - `generated_composer.lock = 4,336`
  - `generated_pipfile.lock = 2,811`
  - `generated_package-lock.json = 4,392`
  - `generated_go.sum = 151`
  - `repo_Cargo.lock = 9,114`

- Current read:
  - composer current-window `newest` / `oldest` scoring tweaks are effectively closed in this form
  - the remaining composer gap still points away from local current-window scoring and toward a
    different sequence or entropy representation

## 2026-06-01 - Closed package-style JSON next-position repeat branch

- Rejected:
  - package-style JSON `ConfigText` next-position repeat-lookahead
  - intended to let a short current-position match yield to a longer `ip+1` repeat on
    `package.json` / `turbo.json`-like content

- Focused result:
  - exact no-op on the JSON-config family:
    - `generated_package.json = 3,956`
    - `generated_turbo.json = 3,956`
    - `generated_tsconfig.json = 2,492`
    - `generated_deno.json = 2,492`
    - `generated_nx.json = 2,492`

- Restored retained source confirmed:
  - `generated_package.json = 3,956`
  - `generated_turbo.json = 3,956`
  - `generated_tsconfig.json = 2,492`
  - `generated_deno.json = 2,492`
  - `generated_nx.json = 2,492`

- Current read:
  - package-style JSON is not moving on this `ip+1` repeat family
  - the next credible JSON-config branch still needs a different sequence-shape mechanism, not
    another repeat-lookahead toggle

## 2026-06-01 - Closed package-style JSON current-entry second_newest branch

- Rejected:
  - package-style JSON `ConfigText` current-entry `second_newest`
  - intended to identify `package.json` / `turbo.json`-like content and give it a retained
    `CodeText`-style current-entry `second_newest` sidecar path

- Focused result:
  - exact no-op on the JSON-config family:
    - `generated_package.json = 3,956`
    - `generated_turbo.json = 3,956`
    - `generated_tsconfig.json = 2,492`
    - `generated_deno.json = 2,492`
    - `generated_nx.json = 2,492`

- Restored retained source confirmed:
  - `generated_package.json = 3,956`
  - `generated_turbo.json = 3,956`
  - `generated_tsconfig.json = 2,492`
  - `generated_deno.json = 2,492`
  - `generated_nx.json = 2,492`

- Current read:
  - package-style JSON is not moving on current-entry `second_newest` either
  - the next credible JSON-config branch still needs a different sequence-shape mechanism, not
    another current-entry sidecar variant

## 2026-06-01 - Retained structured-JSON ConfigText dense probe step

- Kept:
  - a structured-JSON `ConfigText` parser branch in `ruzstd/src/encoding/match_generator.rs`
  - for short-line JSON object configs up to `128 KiB`, use dense no-match probe step `1`
    instead of the generic short-line text step `2`

- Focused result vs retained baseline:
  - `generated_package.json`: `3,956 -> 3,785`
  - `generated_turbo.json`: `3,956 -> 3,785`
  - `generated_tsconfig.json`: unchanged at `2,492`
  - `generated_deno.json`: unchanged at `2,492`
  - `generated_nx.json`: unchanged at `2,492`

- Broad-local result:
  - only two fixtures moved:
    - `generated_package.json`: `3,956 -> 3,785`
    - `generated_turbo.json`: `3,956 -> 3,785`
  - both are now better than C
  - refreshed retained baseline:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal vs C
    - `2,049` bytes above C on losing fixtures

- Current read:
  - package-style JSON did want a real probe-step change
  - tsconfig-style JSON still did not move, so that family remains a separate sequence-shape
    problem
  - next JSON-config work should split by JSON subfamily instead of treating all structured JSON
    configs as the same parser problem

## 2026-06-01 - Retained tsconfig-style JSON ConfigText wider probe step

- Kept:
  - a tsconfig-style JSON `ConfigText` parser branch in
    `ruzstd/src/encoding/match_generator.rs`
  - detect the `compilerOptions` / `paths` JSON family and keep it on the wider text
    no-match probe step `3`

- Focused result vs `jsonconfig-step1` retained baseline:
  - `generated_package.json`: unchanged at `3,785`
  - `generated_turbo.json`: unchanged at `3,785`
  - `generated_tsconfig.json`: `2,492 -> 2,489`
  - `generated_deno.json`: `2,492 -> 2,489`
  - `generated_nx.json`: `2,492 -> 2,489`

- Broad-local result:
  - only the tsconfig/deno/nx family moved
  - refreshed retained baseline:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal vs C
    - `2,040` bytes above C on losing fixtures

- Current read:
  - JSON-config is now clearly split by subfamily:
    - package-style JSON wants dense probing
    - tsconfig-style JSON wants the wider text stride
  - next JSON-config work should stay subfamily-specific rather than treating all JSON configs
    as one parser line

## 2026-06-01 - Rejected fastest-only raw composer package-boundary split

- Rejected:
  - a structural composer-family branch in `ruzstd/src/encoding/frame_compressor.rs`
  - for composer-style `DictionaryText` at fastest level, split the raw input at package-object
    boundaries before calling `compress_fastest`

- Focused result vs current source baseline:
  - `generated_composer.lock`: `4,332 -> 4,352`
  - unchanged controls:
    - `generated_pipfile.lock = 2,811`
    - `generated_package-lock.json = 4,392`
    - `generated_go.sum = 151`
    - `repo_Cargo.lock = 9,114`

- Current read:
  - composer is not waiting on a raw package-boundary multi-block split
  - next composer work should stay away from this structural family

## 2026-06-01 - Retained tsconfig-style JSON ConfigText probe step 5

- Kept:
  - widened the tsconfig-style JSON `ConfigText` no-match probe step from `4` to `5` in
    `ruzstd/src/encoding/match_generator.rs`

- Focused result vs current source baseline:
  - unchanged package-style controls:
    - `generated_package.json = 3,785`
    - `generated_turbo.json = 3,785`
  - improved:
    - `generated_tsconfig.json`: `2,486 -> 2,485`
    - `generated_deno.json`: `2,486 -> 2,485`
    - `generated_nx.json`: `2,486 -> 2,485`

- Broad-local result:
  - only the tsconfig/deno/nx family moved
  - refreshed retained baseline:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal vs C
    - `2,024` bytes above C on losing fixtures

- Current read:
  - tsconfig-style JSON still has a small amount of parser slack on the wider-stride family
  - the next high-value branches are still `repo_Cargo.lock`, `generated_composer.lock`, and then
    more tsconfig-family shape work only if a non-threshold mechanism appears

## 2026-06-01 - Rejected lockfile-like DictionaryText probe step 3

- Rejected:
  - widened the active lockfile no-match probe step from `2` to `3`

- Focused result vs current source baseline:
  - exact byte-for-byte no-op on the focused lockfile family:
    - `repo_Cargo.lock = 9,114`
    - `generated_go.sum = 151`
    - `generated_poetry.lock = 359`
    - `generated_yarn.lock = 383`

- Current read:
  - the active lockfile parser still does not want a wider text stride
  - next `Cargo.lock` work should stay away from more local probe-step widening

## 2026-06-01 - Rejected composer repeat-aware same-start preference

- Rejected:
  - for composer-style `DictionaryText`, prefer a repeat-offset candidate over a non-repeat
    candidate when both start at the same byte and the repeat loses at most `1` match byte

- Focused result vs current source baseline:
  - exact byte-for-byte no-op on the focused composer family:
    - `generated_composer.lock = 4,332`
    - `generated_pipfile.lock = 2,811`
    - `generated_package-lock.json = 4,392`
    - `generated_go.sum = 151`
    - `repo_Cargo.lock = 9,114`

- Current read:
  - the remaining composer gap is not waiting on this local repeat-aware scoring branch
  - next composer work should stay away from this same-start repeat-promotion family

## 2026-06-01 - Rejected DictionaryText OF repeat-table window 1024

- Rejected:
  - for fastest-level `DictionaryText`, widen the OF repeat-table reuse window from `64` to
    `1024` sequences while leaving LL/ML behavior unchanged

- Focused result vs current source baseline:
  - exact byte-for-byte no-op on the focused composer/lockfile family:
    - `generated_composer.lock = 4,332`
    - `generated_pipfile.lock = 2,811`
    - `generated_package-lock.json = 4,392`
    - `generated_go.sum = 151`
    - `repo_Cargo.lock = 9,114`

- Current read:
  - the remaining composer and lockfile gaps are not waiting on this OF-only repeat-table reuse
    family in this form

## 2026-06-01 - Rejected pubspec.lock remap to ConfigText

- Rejected:
  - remap `pubspec.lock` from `DictionaryText` to `ConfigText`

- Focused result vs current source baseline:
  - exact byte-for-byte no-op:
    - `generated_pubspec.lock = 233`
    - `generated_pubspec.yaml = 187`
    - `generated_go.sum = 151`
    - `generated_poetry.lock = 359`

- Current read:
  - `pubspec.lock` is not improved by this remap
  - this small known-file mapping family is closed in that direction

## 2026-06-01 - Retained composer-style DictionaryText wider probe step

- Kept:
  - a composer-style `DictionaryText` parser branch in
    `ruzstd/src/encoding/match_generator.rs`
  - detect composer-style JSON lockfiles and keep them on the wider text no-match probe step `3`

- Focused result vs `tsconfig-step3` retained baseline:
  - `generated_composer.lock`: `4,336 -> 4,332`
  - `generated_pipfile.lock`: unchanged at `2,811`
  - `generated_package-lock.json`: unchanged at `4,392`
  - `generated_go.sum`: unchanged at `151`
  - `repo_Cargo.lock`: unchanged at `9,114`

- Broad-local result:
  - only `generated_composer.lock` moved
  - refreshed retained baseline:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal vs C
    - `2,036` bytes above C on losing fixtures

- Current read:
  - composer-style lockfiles did want a real parser-shape change after all
  - like JSON-config, the useful composer move was a probe-step split rather than another local
    repeat/window scoring tweak

## 2026-06-01 - Rejected composer-style DictionaryText probe step 4

- Rejected:
  - composer-style `DictionaryText` probe step `4`
  - starting from the retained composer-specific text stride `3`, widened that family to `4`

- Focused result:
  - `generated_composer.lock`: `4,332 -> 4,336`
  - unchanged controls:
    - `generated_pipfile.lock = 2,811`
    - `generated_package-lock.json = 4,392`
    - `generated_go.sum = 151`
    - `repo_Cargo.lock = 9,114`

- Current read:
  - the composer probe-step family is now bounded
  - retained best point stays at step `3`

## 2026-06-01 - Retained tsconfig-style JSON ConfigText probe step 4

- Kept:
  - widened the retained tsconfig-style JSON text stride from `3` to `4`

- Focused result vs `composer-step3` retained baseline:
  - `generated_package.json`: unchanged at `3,785`
  - `generated_turbo.json`: unchanged at `3,785`
  - `generated_tsconfig.json`: `2,489 -> 2,486`
  - `generated_deno.json`: `2,489 -> 2,486`
  - `generated_nx.json`: `2,489 -> 2,486`

- Broad-local result:
  - only the tsconfig/deno/nx family moved
  - refreshed retained baseline:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal vs C
    - `2,027` bytes above C on losing fixtures

- Current read:
  - tsconfig-style JSON still wanted a slightly wider stride than the earlier retained point
  - package-style JSON and tsconfig-style JSON are now separately tuned rather than sharing one
    generic structured-JSON parser shape

## 2026-06-01 - Retained composer repeat-kind preference at same start

- Kept:
  - a composer-style `DictionaryText` matcher branch in
    `ruzstd/src/encoding/match_generator.rs`
  - when two current-position repeat candidates start at the same byte, prefer the repeat kind
    that matches the encoder repeat-code order if it loses at most `1` match byte

- Focused result vs current source baseline:
  - `generated_composer.lock`: `4,332 -> 4,160`
  - unchanged controls:
    - `generated_pipfile.lock = 2,811`
    - `generated_package-lock.json = 4,392`
    - `generated_go.sum = 151`
    - `repo_Cargo.lock = 9,114`

- Useful matcher shift on `generated_composer.lock`:
  - `total_sequences`: `2676 -> 2673`
  - `repeat_current`: `[946, 518, 739] -> [909, 664, 687]`
  - `repeat_current_zero_literals`: `[0, 438, 602] -> [0, 647, 445]`
  - `window_current_newest[0]`: `209 -> 148`

- Broad-local result:
  - only `generated_composer.lock` moved
  - refreshed retained baseline:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal vs C
    - bytes above C on losers: `2,024 -> 1,852`

- Current read:
  - composer did want a repeat-family parser-shape change after all
  - next highest-value target remains `repo_Cargo.lock`

## 2026-06-01 - Retained lockfile repeat-kind preference at same start

- Kept:
  - a lockfile-like `DictionaryText` matcher branch in
    `ruzstd/src/encoding/match_generator.rs`
  - when two current-position repeat candidates start at the same byte, prefer the repeat kind
    that matches the encoder repeat-code order if it loses at most `1` match byte

- Focused result vs current retained baseline:
  - `repo_Cargo.lock`: `9,114 -> 9,111`
  - unchanged controls:
    - `generated_go.sum = 151`
    - `generated_poetry.lock = 359`
    - `generated_yarn.lock = 383`
    - `generated_composer.lock = 4,160`

- Useful matcher shift on `repo_Cargo.lock`:
  - `repeat_current`: `[65, 24, 10] -> [71, 19, 9]`
  - `repeat_best_before_window`: `[67, 25, 11] -> [73, 20, 10]`
  - current-window wins stayed flat:
    - `window_current_newest[0] = 421`
    - `window_current_second_newest[0] = 105`
    - `window_current_oldest[0] = 196`

- Broad-local result:
  - only `repo_Cargo.lock` moved
  - refreshed retained baseline:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal vs C
    - bytes above C on losers: `1,852 -> 1,849`

- Current read:
  - the active lockfile path still had a small repeat-family win available
  - next highest-value target is still `repo_Cargo.lock`, but not another broad current-window
    probe-step change

## 2026-06-01 - Rejected lockfile repeat-kind match-loss 2

- Rejected:
  - widen the retained lockfile repeat-kind match-loss allowance from `1` to `2`

- Focused result vs current retained baseline:
  - exact byte-for-byte no-op:
    - `repo_Cargo.lock = 9,111`
    - `generated_go.sum = 151`
    - `generated_poetry.lock = 359`
    - `generated_yarn.lock = 383`
    - `generated_composer.lock = 4,160`

- Current read:
  - this retained lockfile repeat-kind family is bounded in that direction
  - widening the match-loss budget does not create additional parse movement

## 2026-06-01 - Rejected fastest lockfile partition-path retest

- Rejected:
  - let lockfile-like `DictionaryText` reach the existing fastest-level partition candidate path
    in `ruzstd/src/encoding/levels/fastest.rs`

- Focused result vs current retained baseline:
  - exact byte-for-byte no-op:
    - `repo_Cargo.lock = 9,111`
    - `generated_go.sum = 151`
    - `generated_poetry.lock = 359`
    - `generated_yarn.lock = 383`
    - `generated_composer.lock = 4,160`

- Current read:
  - the active lockfile path is not waiting on the existing fastest partition machinery in this
    form
  - this structural family remains closed on the current retained baseline

## 2026-06-01 - Retained lockfile same-end smaller-offset preference

- Kept:
  - a lockfile-like `DictionaryText` matcher branch in
    `ruzstd/src/encoding/match_generator.rs`
  - when two non-repeat candidates end at the same byte, prefer the smaller-offset candidate if
    it loses at most `1` match byte and saves at least `2` offset-code bits

- Focused result vs current retained baseline:
  - `repo_Cargo.lock`: `9,111 -> 9,109`
  - unchanged controls:
    - `generated_go.sum = 151`
    - `generated_poetry.lock = 359`
    - `generated_yarn.lock = 383`
    - `generated_composer.lock = 4,160`

- Useful matcher shift on `repo_Cargo.lock`:
  - `repeat_current`: `[71, 19, 9] -> [72, 19, 9]`
  - `repeat_best_before_window`: `[73, 20, 10] -> [74, 20, 10]`
  - `window_current_newest[0]`: `421 -> 422`
  - `window_current_second_newest[0]`: `105 -> 103`

- Broad-local result:
  - only `repo_Cargo.lock` moved
  - refreshed retained baseline:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal vs C
    - bytes above C on losers: `1,849 -> 1,847`

- Current read:
  - the active `Cargo.lock` path still had a small same-end parse-shape win available
  - next highest-value target remains `repo_Cargo.lock`, but not the already-closed repeat-threshold
    or fastest-partition families

## 2026-06-01 - Rejected lockfile same-end smaller-offset match-loss 2

- Rejected:
  - widen the retained lockfile same-end match-loss allowance from `1` to `2`

- Focused result vs current retained baseline:
  - exact byte-for-byte no-op:
    - `repo_Cargo.lock = 9,109`
    - `generated_go.sum = 151`
    - `generated_poetry.lock = 359`
    - `generated_yarn.lock = 383`
    - `generated_composer.lock = 4,160`

- Current read:
  - this retained lockfile same-end family is bounded in that direction
  - widening the same-end match-loss budget does not create additional parse movement

## 2026-06-01 - Rejected lockfile OF table max-log 6

- Rejected:
  - when the block is lockfile-like `DictionaryText`, lower OF table max-log from `7` to `6`
    across the fastest-level whole-block and partition paths

- Focused result vs current retained baseline:
  - `repo_Cargo.lock`: `9,109 -> 9,145`
  - unchanged controls:
    - `generated_go.sum = 151`
    - `generated_poetry.lock = 359`
    - `generated_yarn.lock = 383`
    - `generated_composer.lock = 4,160`

- Current read:
  - this was pure entropy-side damage
  - the active `Cargo.lock` path does not want a smaller OF table max-log in this form

## 2026-06-01 - Added tuner and retained tsconfig step 6

- Added:
  - `tools/tune_matcher_family.py`
  - runtime-tunable focused matcher override surface in
    `ruzstd/src/encoding/match_generator.rs`

- Focused tuner results:
  - `cargo-lock`:
    - baseline `10,002`
    - best searched candidate `10,002`
    - local searched surface is exhausted at the retained point
  - `composer`:
    - baseline `11,514`
    - best searched candidate `11,514`
    - local searched surface is exhausted at the retained point
  - `structured-json`:
    - baseline `7,570`
    - best searched candidate `7,570`
    - retained best point stays probe step `1`
  - `tsconfig-json`:
    - baseline `7,455`
    - best searched candidate `7,452`

- Retained:
  - raised tsconfig-style JSON probe step from `5` to `6`

- Broad-local result:
  - only `generated_tsconfig.json`, `generated_deno.json`, and `generated_nx.json` changed
  - refreshed retained baseline:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal vs C
    - bytes above C on losers: `1,847 -> 1,844`

- Current biggest losses:
  - `repo_Cargo.lock`: `9,109` vs `8,088` = `+1,021`
  - `generated_composer.lock`: `4,160` vs `3,766` = `+394`
  - `decodecorpus_z000079`: `7,322` vs `7,221` = `+101`
  - `generated_tsconfig.json`: `2,484` vs `2,391` = `+93`
  - `generated_deno.json`: `2,484` vs `2,391` = `+93`
  - `generated_nx.json`: `2,484` vs `2,391` = `+93`

- Current read:
  - the tuner now gives us an evidence-backed way to stop reopening exhausted local families
  - next highest-value tuner expansion should move away from the already-exhausted local
    `Cargo.lock` / composer knob surfaces and into new sequence/literal representation branches

## 2026-06-01 - Retained dependency-JSON-lockfile encoder config

- Fixed:
  - `tools/tune_matcher_family.py` temp-output race
  - concurrent sweeps now isolate candidate outputs by env hash

- Added:
  - `likely_dependency_json_lockfile_text()` in `ruzstd/src/encoding/util.rs`
  - scoped encoder config in `ruzstd/src/encoding/blocks/compressed.rs`

- Retained:
  - dependency-JSON lockfiles (`package-lock.json` / `Pipfile.lock` style) now use:
    - `HuffmanTableSearch::AllSections`
    - `repeat_table_max_sequences = 256`
    - `offset_table_max_log = 8`

- Focused result vs prior retained baseline:
  - `generated_package-lock.json`: `4,392 -> 4,388`
  - `generated_pipfile.lock`: `2,811 -> 2,804`
  - unchanged controls:
    - `generated_composer.lock = 4,160`
    - `generated_go.sum = 151`
    - `repo_Cargo.lock = 9,109`

- Broad-local result:
  - only those two fixtures moved
  - refreshed retained baseline vs C stayed:
    - `71` fixtures
    - `51 / 16 / 4` better / worse / equal
    - `1,844` bytes above C on losing fixtures

- Current biggest losses remain:
  - `repo_Cargo.lock`: `9,109` vs `8,088` = `+1,021`
  - `generated_composer.lock`: `4,160` vs `3,766` = `+394`
  - `decodecorpus_z000079`: `7,322` vs `7,221` = `+101`
  - `generated_tsconfig.json`: `2,484` vs `2,391` = `+93`
  - `generated_deno.json`: `2,484` vs `2,391` = `+93`
  - `generated_nx.json`: `2,484` vs `2,391` = `+93`

- Current read:
  - known-file handling improved again with a clean encoder-side subfamily
  - `Cargo.lock` still needs a different representation family
  - composer still needs a true composer-specific sequence/entropy branch, not the dependency-JSON
    lockfile encoder path

## 2026-06-01 - Retained whole-file dependency-JSON profile

- Retained:
  - added an internal whole-file profile path for dependency-JSON lockfiles
  - public `CompressionFileType` stays unchanged
  - dependency-JSON encoder tuning now applies across every block instead of depending on the
    first-block content guess

- Files:
  - `ruzstd/src/encoding/mod.rs`
  - `ruzstd/src/encoding/frame_compressor.rs`
  - `ruzstd/src/encoding/levels/fastest.rs`
  - `ruzstd/src/encoding/blocks/compressed.rs`

- Focused result vs prior retained baseline:
  - `generated_package-lock.json`: `4,388 -> 4,383`
  - unchanged controls:
    - `generated_pipfile.lock = 2,804`
    - `generated_composer.lock = 4,160`
    - `generated_go.sum = 151`
    - `repo_Cargo.lock = 9,109`

- Broad-local result:
  - only `generated_package-lock.json` moved
  - retained baseline vs C stayed:
    - `71` fixtures
    - `51 / 16 / 4`
    - `1,844` bytes above C on losers

- Current biggest losses remain:
  - `repo_Cargo.lock`: `+1,021`
  - `generated_composer.lock`: `+394`
  - `decodecorpus_z000079`: `+101`
  - `generated_tsconfig.json`: `+93`
  - `generated_deno.json`: `+93`
  - `generated_nx.json`: `+93`

- Current read:
  - the dependency-JSON path is now correctly whole-file
  - the next high-value tuner expansion should move away from local dependency-JSON knobs and back
    to new representation branches for `repo_Cargo.lock` and the remaining composer gap

## 2026-06-01 - Retained composer probe step 5

- Retained:
  - composer-style `DictionaryText` now defaults to no-match probe step `5`
  - this came from extending the tuner surface beyond the earlier searched `3/4` composer points

- Files:
  - `ruzstd/src/encoding/match_generator.rs`

- Focused result vs prior retained baseline:
  - `generated_composer.lock`: `4,160 -> 4,159`
  - unchanged controls:
    - `generated_pipfile.lock = 2,804`
    - `generated_package-lock.json = 4,383`
    - `generated_go.sum = 151`
    - `repo_Cargo.lock = 9,109`

- Broad-local result:
  - only `generated_composer.lock` moved
  - retained baseline vs C:
    - `71` fixtures
    - `51 / 16 / 4`
    - `1,843` bytes above C on losers

- Closed this turn:
  - single-segment frame headers: reject
  - composer whole-vs-split text comparison: no-op
  - tsconfig probe steps `7` and `8`: no-op
  - composer probe step `6`: regression

- Current biggest losses:
  - `repo_Cargo.lock`: `+1,021`
  - `generated_composer.lock`: `+393`
  - `decodecorpus_z000079`: `+101`
  - `generated_tsconfig.json`: `+93`
  - `generated_deno.json`: `+93`
  - `generated_nx.json`: `+93`

- Current read:
  - the composer local probe-step family is now bounded with retained best point at `5`
  - next highest-value work is still `repo_Cargo.lock`, then the remaining composer gap

## 2026-06-01 - Composer partition-cap family bounded

- Added search-surface plumbing:
  - `RUZSTD_TUNE_COMPOSER_MAX_PARTITIONS` in `ruzstd/src/encoding/levels/fastest.rs`
  - `composer-partitions` preset in `tools/tune_matcher_family.py`

- Focused sweep against the current retained baseline:
  - `generated_composer.lock`
    - cap `1`: `4,255`
    - cap `2`: `4,194`
    - caps `3..8`: all `4,159`

- Result:
  - no new retained compression change
  - default path is byte-identical to the retained `composer step 5` baseline when the override is
    unset

- Current read:
  - composer partition count is not the missing lever
  - next best target remains `repo_Cargo.lock`

## 2026-06-01 - Cargo.lock local tuner surface widened and re-bounded

- Expanded the `cargo-lock` and `cargo-lock-combined` presets in
  `tools/tune_matcher_family.py` to cover:
  - `RUZSTD_TUNE_LOCKFILE_REPEAT_KIND_MATCH_LOSS_MAX = 0/1/2`
  - `RUZSTD_TUNE_LOCKFILE_SAME_END_MATCH_LOSS_MAX = 0/1/2`
  - `RUZSTD_TUNE_LOCKFILE_SAME_END_BITS_GAIN_MIN = 1/2/3`

- Focused search results on the current retained source baseline:
  - matcher-only `cargo-lock` sweep:
    - baseline `10,002`
    - best searched candidate `10,002`
  - combined `cargo-lock-combined` sweep:
    - baseline `10,002`
    - best searched candidate `10,001`
    - that `-1` did not move `repo_Cargo.lock`
    - it only improved `generated_poetry.lock: 359 -> 358`

- Follow-up isolation:
  - the only winning lever in that combined candidate was
    `RUZSTD_TUNE_OFFSET_PREDEFINED_MAX_SEQUENCES = 64`
  - broad-local spot check was not clean:
    - wins:
      - `generated_poetry.lock: 359 -> 358`
      - `generated_pubspec.lock: 233 -> 230`
    - regressions:
      - `repo_ci.yml: 556 -> 562`
      - `repo_ruzstd_Cargo.toml: 730 -> 734`
      - `dict_systemd-journal-gatewayd.service: 622 -> 627`

- Result:
  - no new retained runtime change
  - retained baseline stays:
    - `71` fixtures
    - `51 / 16 / 4`
    - `1,843` bytes above C on losers

- Updated read:
  - the current `Cargo.lock` local threshold surface is effectively exhausted
  - lower same-end / repeat-kind thresholds do not create real `repo_Cargo.lock` movement
  - the next credible `Cargo.lock` branch should move to a different literal/sequence
    representation rather than more local threshold tuning

## 2026-06-01 - Lockfile literal encoder surface flat; zero-literal second-newest is parse-only

- Added a new focused tuner preset in `tools/tune_matcher_family.py`:
  - `cargo-lock-literal-encoder`
  - searched:
    - `RUZSTD_TUNE_HUFFMAN_TABLE_SEARCH`
    - `RUZSTD_TUNE_FILE_TYPE_SINGLE_STREAM_HUFFMAN_MAX_LITERALS`
    - `RUZSTD_TUNE_FILE_TYPE_SMALL_SEQUENCE_PREDEFINED_LLML_MAX_SEQUENCES`

- Focused result on the current retained source baseline:
  - baseline `10,002`
  - best searched candidate `10,002`
  - no improvement anywhere on that new literal encoder surface

- Added a tune-only matcher override in `ruzstd/src/encoding/match_generator.rs`:
  - `RUZSTD_TUNE_LOCKFILE_SECOND_NEWEST_ZERO_LITERALS`
  - when disabled, lockfile-like `DictionaryText` skips zero-literal `second_newest` probes

- Focused byte result with that gate disabled:
  - exact no-op:
    - `repo_Cargo.lock = 9,109`
    - `generated_go.sum = 151`
    - `generated_poetry.lock = 359`
    - `generated_yarn.lock = 383`

- But live matcher diagnostics on `repo_Cargo.lock` moved a lot:
  - sequences `821 -> 842`
  - `window_current_second_newest_zero_literals: 66 -> 1`
  - `window_current_newest: 422 -> 476`
  - `window_current_oldest: 196 -> 225`

- Updated read:
  - zero-literal `second_newest` is a real parse-shape lever, but not a size win by itself
  - the newly searched lockfile literal encoder surface is flat
  - the next `Cargo.lock` branch should be a broader sequence/literal representation change, not
    another nearby encoder knob or source-order toggle

## 2026-06-01 - Composer zero-literal repeat-kind scope bounded

- Added a new focused tuner preset in `tools/tune_matcher_family.py`:
  - `composer-repeat-zero-literals`
- Added a tune-only matcher override in `ruzstd/src/encoding/match_generator.rs`:
  - `RUZSTD_TUNE_COMPOSER_REPEAT_KIND_ZERO_LITERALS_ONLY`

- Reason for the branch:
  - live fastest-level composer diagnostics are overwhelmingly repeat-side, especially at
    zero literals:
    - `repeat_current = [909, 663, 687]`
    - `repeat_current_zero_literals = [0, 647, 445]`
  - so the next credible composer follow-up was to scope the retained repeat-kind preference to
    that dominant zero-literal subfamily

- Focused search result:
  - baseline `11,497`
  - best searched candidate `11,497`
  - searched surface:
    - `RUZSTD_TUNE_COMPOSER_REPEAT_KIND_MATCH_LOSS_MAX = 0/1/2`
    - `RUZSTD_TUNE_COMPOSER_REPEAT_KIND_ZERO_LITERALS_ONLY = 0/1`

- Updated read:
  - the dominant composer repeat-side pattern is real
  - but zero-literal-only scoping of the retained repeat-kind rule is flat
  - the next composer branch should move away from repeat-kind scope toggles and toward a
    different sequence or block/entropy representation

## 2026-06-01 - Strong lockfile zero-literal suppression and coarse composer window suppression both bounded

- Added tune-only matcher overrides in `ruzstd/src/encoding/match_generator.rs`:
  - `RUZSTD_TUNE_LOCKFILE_ZERO_LITERAL_WINDOW_DISABLE`
  - `RUZSTD_TUNE_COMPOSER_WINDOW_DISABLE`
- Added a reproducible focused preset in `tools/tune_matcher_family.py`:
  - `composer-window-disable`

- Lockfile result:
  - disabling all zero-literal non-repeat window candidates on lockfile-like `DictionaryText`
    changed the live `repo_Cargo.lock` parse a lot:
    - sequences `821 -> 760`
    - `window_current_oldest: 196 -> 151`
    - `window_current_newest: 422 -> 396`
  - but bytes stayed flat:
    - `repo_Cargo.lock = 9,109`
  - pairing it with the nearby best lockfile encoder settings still did not move `repo_Cargo.lock`

- Composer result:
  - disabling all non-repeat composer window candidates is also flat in the focused searched space:
    - baseline `11,497`
    - best searched candidate `11,497`
  - even the parse-heavy disabled-window variant did not move bytes on
    `generated_composer.lock`

- Updated read:
  - the strong lockfile zero-literal suppression family is bounded
  - the coarse composer window-suppression family is bounded
  - the remaining top gaps now point even more strongly to a different literal/sequence
    representation, not another local source-family suppression toggle

## 2026-06-01 - Composer zero-literal repeat-candidate family bounded

- Added a tune-only matcher override in `ruzstd/src/encoding/match_generator.rs`:
  - `RUZSTD_TUNE_COMPOSER_ZERO_LITERAL_REPEAT_CANDIDATE_LIMIT`
- Added a focused preset in `tools/tune_matcher_family.py`:
  - `composer-zero-literal-repeat-limit`

- Reason for the branch:
  - composer zero-literal repeat candidates are tried in a fixed order:
    - `second`, `third`, `first-1`
  - live composer diagnostics showed that zero-literal repeat traffic is concentrated in that
    subfamily, so a direct candidate-count limit was the next structural probe

- Focused result:
  - baseline `11,497`
  - best searched candidate `11,497`
  - searched surface:
    - `RUZSTD_TUNE_COMPOSER_ZERO_LITERAL_REPEAT_CANDIDATE_LIMIT = 1/2/3`
    - `RUZSTD_TUNE_COMPOSER_REPEAT_KIND_MATCH_LOSS_MAX = 0/1/2`

- Updated read:
  - the composer zero-literal repeat-candidate-limit family is bounded
  - the remaining composer gap is not waiting on simple repeat-kind suppression, repeat-kind scope
    gating, or coarse window suppression

## 2026-06-01 - Lockfile fastest split / whole-compare family bounded

- Added tune-only fastest-path overrides in `ruzstd/src/encoding/levels/fastest.rs`:
  - `RUZSTD_TUNE_LOCKFILE_FASTEST_SPLITS`
  - `RUZSTD_TUNE_LOCKFILE_COMPARE_WHOLE_TEXT`
- Added a focused preset in `tools/tune_matcher_family.py`:
  - `cargo-lock-splits`

- Reason for the branch:
  - the fastest path already had a composer-only structural split branch
  - lockfile text was still excluded from whole-vs-partition comparison by the generic
    `likely_text` gate
  - so this was the cleanest remaining fastest-path structural representation probe for
    `Cargo.lock`

- Focused result:
  - baseline `10,002`
  - best searched candidate `10,002`
  - searched surface:
    - `RUZSTD_TUNE_LOCKFILE_FASTEST_SPLITS = 0/1`
    - `RUZSTD_TUNE_LOCKFILE_COMPARE_WHOLE_TEXT = 0/1`

- Updated read:
  - the lockfile fastest split / whole-compare family is bounded
  - the remaining `Cargo.lock` gap is not waiting on simply letting the fastest path reuse the
    current split machinery for text blocks

## 2026-06-01 - Lockfile post-parse zero-literal match-dropping family bounded

- Added tune-only post-`prepare_block()` overrides in `ruzstd/src/encoding/blocks/compressed.rs`:
  - `RUZSTD_TUNE_LOCKFILE_DROP_ZERO_LITERAL_MATCH_MAX_LEN`
  - `RUZSTD_TUNE_LOCKFILE_DROP_ZERO_LITERAL_MATCH_MIN_OF_CODE`
- Added a focused preset in `tools/tune_matcher_family.py`:
  - `cargo-lock-drop-zero-literal-match`

- Reason for the branch:
  - current `Cargo.lock` still has more sequences than C while also leaving a much worse literal
    stream
  - this was the first direct post-match representation probe:
    - downgrade selected zero-literal lockfile matches into literals before entropy coding

- Focused result:
  - baseline `10,002`
  - best searched candidate `10,002`
  - aggressive spot-check (`max_len=8`, `min_of_code=7`) was archive-identical to the retained
    baseline

- Updated read:
  - correction after rebuilding the release binary:
    - direct post-parse short-match downgrading was invalid, not flat
  - removing individual sequences after matching breaks later repeat-offset assumptions unless the
    downstream repeat history is recomputed consistently
  - the next credible `Cargo.lock` branch must still be a different representation change, but not
    this unsafe post-parse sequence-dropping approach

## 2026-06-01 - Lockfile sequence-cost tuner family rejected

- Added and tested a broader tune-only local scorer for lockfile candidates:
  - `RUZSTD_TUNE_LOCKFILE_SEQUENCE_COST_LITERAL_WEIGHT`
  - `RUZSTD_TUNE_LOCKFILE_SEQUENCE_COST_OFFSET_WEIGHT`
  - `RUZSTD_TUNE_LOCKFILE_SEQUENCE_COST_MARGIN`
- Focused preset:
  - `cargo-lock-sequence-cost`
- Result:
  - baseline `10,002`
  - best searched candidate `10,054`
  - every reported point regressed the focused lockfile family
- Resolution:
  - removed the scorer branch and tuner preset
  - exact restore vs retained baseline:
    - [cargolock-sequence-cost-restore.md](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/cargolock-sequence-cost-restore.md)
- Updated read:
  - `Cargo.lock` is not waiting on another broader local candidate-scoring rule
  - next credible work remains a more substantive literal/sequence representation change

## 2026-06-01 - Retained Cargo.lock zero-literal next-position lazy parse

- Added a broader `Cargo.lock` representation branch in
  [ruzstd/src/encoding/match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs):
  - at zero literals, compare the current lockfile candidate against the best candidate at `ip+1`
  - use a local literal/match/offset cost model to decide whether to delay by one byte
- Added a focused sweep preset in
  [tools/tune_matcher_family.py](/home/bsutton/git/zstd-rs/tools/tune_matcher_family.py):
  - `cargo-lock-next-position`

- Focused sweep:
  - baseline `10,002`
  - best `9,999`
  - retained point:
    - `max_current_match_len=7`
    - `literal_weight=6`
    - `match_reward=2`
    - `offset_weight=1`
    - `margin=1`

- Retained focused result:
  - `repo_Cargo.lock`: `9,109 -> 9,106`
  - unchanged:
    - `generated_go.sum = 151`
    - `generated_poetry.lock = 359`
    - `generated_yarn.lock = 383`

- Live `Cargo.lock` inspect at the retained point:
  - `sequence_count`: `821 -> 817`
  - `sequence_payload_bytes`: `2208 -> 2195`
  - `of_extra_bits`: `6898 -> 6830`
  - `decoded_literals`: `9932 -> 9938`
  - `literal_section_bytes`: `6886 -> 6891`

- Broad-local:
  - only `repo_Cargo.lock` moved
  - retained baseline vs C `zstd -1`:
    - `71` fixtures
    - `51 / 16 / 4`
    - `1,839` bytes above C on losers

- Updated read:
  - this is the first retained `Cargo.lock` move from a genuine one-step lazy-parse
    representation branch rather than another same-position threshold
  - it is small, but it is the right family
  - next work should keep exploring broader parse/literal representation changes from here

## 2026-06-01 - Tiny-literal extension of Cargo.lock lazy parse rejected

- Tested a wider version of the retained lockfile lazy-parse family:
  - allow the `ip` vs `ip+1` comparison on tiny literal runs, not just strict zero literals
- Focused preset:
  - `cargo-lock-next-position-literals`
- Result:
  - baseline `9,999`
  - best searched candidate `9,999`
  - widening the family to `max_literal_len = 1/2/3` never beat the retained zero-literal point
- Resolution:
  - removed the tune-only tiny-literal extension and preset
  - exact restore vs retained baseline:
    - [lockfile-next-position-restore.md](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-next-position-restore.md)
- Updated read:
  - the productive lockfile lazy-parse family is currently bounded at strict zero literals
  - next work should widen representation in a different direction, not just extend this branch to
    adjacent literal lengths

## 2026-06-01 - Retained Cargo.lock two-byte lazy-parse skip

- Extended the productive lockfile lazy-parse family in
  [ruzstd/src/encoding/match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs):
  - still zero-literal only
  - but compare the current candidate against the best candidate after skipping up to `2`
    literal bytes
- Focused preset:
  - `cargo-lock-next-position-skip`

- Focused sweep:
  - baseline `9,999`
  - best `9,998`
  - retained point:
    - `max_skip_literals=2`
    - `max_current_match_len=7`
    - `literal_weight=6`
    - `match_reward=2`
    - `offset_weight=2`
    - `margin=1`

- Retained focused result:
  - `repo_Cargo.lock`: `9,106 -> 9,105`
  - unchanged:
    - `generated_go.sum = 151`
    - `generated_poetry.lock = 359`
    - `generated_yarn.lock = 383`

- Live `Cargo.lock` inspect at the retained point:
  - `sequence_count`: `817 -> 810`
  - `sequence_payload_bytes`: `2195 -> 2184`
  - `of_extra_bits`: `6830 -> 6777`
  - `decoded_literals`: `9938 -> 9952`
  - `literal_section_bytes`: `6891 -> 6901`

- Broad-local:
  - only `repo_Cargo.lock` moved
  - retained baseline vs C `zstd -1`:
    - `71` fixtures
    - `51 / 16 / 4`
    - `1,838` bytes above C on losers

- Updated read:
  - the productive lockfile lazy-parse family still has small headroom beyond the one-byte skip
    point
  - it is still reducing offset-side payload while trading a few bytes back to literals

## 2026-06-01 - Retained whole-file ComposerLock profile

- Added an internal `CompressionFileProfile::ComposerLock` in
  [ruzstd/src/encoding/mod.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/mod.rs)
- Routed the known `composer.lock` profile through:
  - [ruzstd/src/encoding/match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs)
  - [ruzstd/src/encoding/levels/fastest.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/levels/fastest.rs)
- Goal:
  - stop relying only on per-block composer-content heuristics
  - let the known-file-type path stay active across the whole frame

- Focused result:
  - `generated_composer.lock`: `4,159 -> 4,119`
  - unchanged:
    - `generated_package-lock.json = 4,381`
    - `generated_pipfile.lock = 2,804`
    - `generated_go.sum = 151`

- Broad-local:
  - only `generated_composer.lock` moved
  - retained baseline vs C `zstd -1`:
    - `71` fixtures
    - `51 / 16 / 4`
    - `1,798` bytes above C on losers

- Live composer inspect:
  - current retained:
    - `blocks=4`
    - `literal_section_bytes=687`
    - `sequence_payload_bytes=3386`
    - `decoded_literals=1503`
    - `sequences=2655`
  - previous retained composer point:
    - `literal_section_bytes=681`
    - `sequence_payload_bytes=3432`
    - `decoded_literals=1495`
    - `sequences=2672`
  - C still uses `2` blocks and much lower sequence payload

- Updated read:
  - the whole-file composer profile is a real known-file-type win
  - it improves composer without touching the dependency-JSON lockfile family
  - the remaining composer gap still looks structural/block-representation-side, but smaller
  - highest-value next target remains `repo_Cargo.lock`

## 2026-06-01 - Retained Cargo.lock equal-length lazy-parse compare

- Extended the productive `Cargo.lock` lazy-parse family in
  [ruzstd/src/encoding/match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs):
  - still zero-literal only
  - still compare against future candidates after skipping up to `2` literal bytes
  - but now allow an equal-length future candidate to win on local parse cost
- Also raised the retained lockfile local offset weight:
  - `offset_weight: 2 -> 3`
- Focused preset:
  - `cargo-lock-next-position-loss`

- Focused sweep:
  - baseline `9,998`
  - best `9,997`
  - retained point:
    - `max_skip_literals=2`
    - `max_current_match_len=7`
    - `max_match_loss=0`
    - `literal_weight=6`
    - `match_reward=2`
    - `offset_weight=3`
    - `margin=1`

- Retained focused result:
  - `repo_Cargo.lock`: `9,105 -> 9,104`
  - unchanged:
    - `generated_go.sum = 151`
    - `generated_poetry.lock = 359`
    - `generated_yarn.lock = 383`

- Live `Cargo.lock` inspect at the retained point:
  - `sequence_count`: `810 -> 810`
  - `sequence_payload_bytes`: `2184 -> 2182`
  - `of_extra_bits`: `6777 -> 6773`
  - `decoded_literals`: `9952 -> 9953`
  - `literal_section_bytes`: `6901 -> 6902`

- Broad-local:
  - only `repo_Cargo.lock` moved
  - retained baseline vs C `zstd -1`:
    - `71` fixtures
    - `51 / 16 / 4`
    - `1,797` bytes above C on losers

- Updated read:
  - the productive lockfile lazy-parse family still has headroom
  - this point is smaller than the earlier skip win, but it is broad-safe and still trims
    offset-side sequence cost
  - highest-value next target remains `repo_Cargo.lock`, but the next branch should probably be a
    wider representation change than another nearby local cost tweak

## 2026-06-01 - Rejected Cargo.lock two-step follow-up lazy parse

- Tested a broader version of the productive lockfile lazy-parse family in
  [ruzstd/src/encoding/match_generator.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/match_generator.rs):
  - estimate a two-step local path cost with an optional follow-up candidate after the first
    chosen match
- Focused preset:
  - `cargo-lock-next-position-followup`
- Result:
  - baseline `9,997`
  - best `9,997`
  - all top candidates kept `RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_USE_FOLLOWUP=0`
- Updated read:
  - the retained one-step lockfile lazy-parse point still dominates this nearby broader lookahead
    family

## 2026-06-01 - Rejected composer whole-vs-partition forced compare

- Tested a tune-only fastest-path switch in
  [ruzstd/src/encoding/levels/fastest.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/levels/fastest.rs):
  - force whole-vs-partition comparison for composer text blocks even though they are likely text
- Focused preset:
  - `composer-whole-compare`
- Result:
  - baseline `11,455`
  - best `11,455`
  - byte-identical across composer partition caps `3..8`
- Updated read:
  - the retained whole-file composer profile did not expose a hidden whole-block win in the
    current fastest split machinery
  - the remaining composer gap still points past this structural toggle

## 2026-06-01 - Retained small-text lockfile profile

- Added an internal `CompressionFileProfile::SmallTextLockfile` in
  [ruzstd/src/encoding/mod.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/mod.rs)
- Routed exact known names to that profile:
  - `poetry.lock`
  - `pubspec.lock`
- Applied a narrow encoder tuning in
  [ruzstd/src/encoding/blocks/compressed.rs](/home/bsutton/git/zstd-rs/ruzstd/src/encoding/blocks/compressed.rs):
  - `HuffmanTableSearch::AllSections`
  - `repeat_table_max_sequences = 256`
  - `offset_table_max_log = 7`
  - `offset_predefined_max_sequences = 64`

- Focused result:
  - `generated_poetry.lock`: `359 -> 358`
  - `generated_pubspec.lock`: `232 -> 229`
  - unchanged:
    - `generated_Gemfile.lock = 239`
    - `generated_go.sum = 151`
    - `generated_pubspec.yaml = 187`

- Broad-local:
  - only two fixtures moved:
    - `generated_poetry.lock`: `359 -> 358`
    - `generated_pubspec.lock`: `232 -> 229`
  - retained baseline vs C `zstd -1`:
    - `71` fixtures
    - `51 / 16 / 4`
    - `1,794` bytes above C on losers

- Updated read:
  - this is a clean extension-based known-file-type win
  - it is not a global encoder retune; the profile stays narrow
  - the highest-value remaining work is still `repo_Cargo.lock`, then `generated_composer.lock`
