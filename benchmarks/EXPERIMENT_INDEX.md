# Experiment Index

Purpose: keep a fast lookup of retained and rejected experiment families so we do not rediscover the same failure mode through a slightly different threshold or gate.

Use this alongside:
- [BENCHMARK_HISTORY.md](/home/bsutton/git/zstd-rs/BENCHMARK_HISTORY.md) for chronological detail
- [CODEX_WORKPLAN.md](/home/bsutton/git/zstd-rs/CODEX_WORKPLAN.md) for longer reasoning

## 2026-06-01 - Newly retained Cargo.lock current-entry thirteenth-newest sidecar

- Cargo.lock current-entry thirteenth-newest sidecar
  - Tried:
    - extend the retained lockfile current-entry recent-candidate representation one more step
    - probe a new thirteenth-newest candidate after the retained twelfth-newest path
    - rerun the retained `cargo-lock-next-position-loss` lazy-parse surface on the current
      baseline as the comparison branch
  - What happened:
    - the alternative lazy-parse surface stayed flat at its focused baseline
    - broad-local stayed clean
    - one retained win:
      - `repo_Cargo.lock`: `9,012 -> 9,010`
    - unchanged controls:
      - `generated_go.sum`: stayed `151`
      - `generated_poetry.lock`: stayed `358`
      - `generated_yarn.lock`: stayed `383`
      - `generated_composer.lock`: stayed `4,112`
    - current broad-local vs C `zstd -1` became:
      - `51 / 16 / 4` better / worse / equal
      - `1,693` bytes above C on losing fixtures
  - Keep this branch.
  - Important conclusion:
    - the alternative `Cargo.lock` lazy-parse surface is flat on the current baseline
    - the current-entry recency family is still the only live local `Cargo.lock` family

## 2026-06-01 - Newly retained Cargo.lock current-entry twelfth-newest sidecar

- Cargo.lock current-entry twelfth-newest sidecar
  - Tried:
    - extend the retained lockfile current-entry recent-candidate representation one more step
    - probe a new twelfth-newest candidate after the retained eleventh-newest path
  - What happened:
    - broad-local stayed clean
    - one retained win:
      - `repo_Cargo.lock`: `9,014 -> 9,012`
    - unchanged controls:
      - `generated_go.sum`: stayed `151`
      - `generated_poetry.lock`: stayed `358`
      - `generated_yarn.lock`: stayed `383`
      - `generated_composer.lock`: stayed `4,112`
    - current broad-local vs C `zstd -1` became:
      - `51 / 16 / 4` better / worse / equal
      - `1,695` bytes above C on losing fixtures
  - Keep this branch.
  - Important conclusion:
    - the broadened current-entry recency family is still live on `Cargo.lock`
    - the main `Cargo.lock` gap is now below `+925` bytes

## 2026-06-01 - Newly retained Cargo.lock current-entry eleventh-newest sidecar

- Cargo.lock current-entry eleventh-newest sidecar
  - Tried:
    - extend the retained lockfile current-entry recent-candidate representation one more step
    - probe a new eleventh-newest candidate after the retained tenth-newest path
  - What happened:
    - broad-local stayed clean
    - one retained win:
      - `repo_Cargo.lock`: `9,021 -> 9,014`
    - unchanged controls:
      - `generated_go.sum`: stayed `151`
      - `generated_poetry.lock`: stayed `358`
      - `generated_yarn.lock`: stayed `383`
      - `generated_composer.lock`: stayed `4,112`
    - current broad-local vs C `zstd -1` became:
      - `51 / 16 / 4` better / worse / equal
      - `1,697` bytes above C on losing fixtures
  - Keep this branch.
  - Important conclusion:
    - the broadened current-entry recency family is still live on `Cargo.lock`
    - the main `Cargo.lock` gap is now below `+930` bytes

## 2026-06-01 - Newly retained Cargo.lock current-entry tenth-newest sidecar

- Cargo.lock current-entry tenth-newest sidecar
  - Tried:
    - extend the retained lockfile current-entry recent-candidate representation one more step
    - probe a new tenth-newest candidate after the retained ninth-newest path
  - What happened:
    - broad-local stayed clean
    - one retained win:
      - `repo_Cargo.lock`: `9,025 -> 9,021`
    - unchanged controls:
      - `generated_go.sum`: stayed `151`
      - `generated_poetry.lock`: stayed `358`
      - `generated_yarn.lock`: stayed `383`
      - `generated_composer.lock`: stayed `4,112`
    - current broad-local vs C `zstd -1` became:
      - `51 / 16 / 4` better / worse / equal
      - `1,704` bytes above C on losing fixtures
  - Keep this branch.
  - Important conclusion:
    - the broadened current-entry recency family is still live on `Cargo.lock`
    - the main `Cargo.lock` gap is now below `+935` bytes

## 2026-06-01 - Newly retained Cargo.lock current-entry ninth-newest sidecar

- Cargo.lock current-entry ninth-newest sidecar
  - Tried:
    - extend the retained lockfile current-entry recent-candidate representation one more step
    - probe a new ninth-newest candidate after the retained eighth-newest path
  - What happened:
    - broad-local stayed clean
    - one retained win:
      - `repo_Cargo.lock`: `9,032 -> 9,025`
    - unchanged controls:
      - `generated_go.sum`: stayed `151`
      - `generated_poetry.lock`: stayed `358`
      - `generated_yarn.lock`: stayed `383`
      - `generated_composer.lock`: stayed `4,112`
    - current broad-local vs C `zstd -1` became:
      - `51 / 16 / 4` better / worse / equal
      - `1,708` bytes above C on losing fixtures
  - Keep this branch.
  - Important conclusion:
    - the broadened current-entry recency family is still live on `Cargo.lock`
    - the main `Cargo.lock` gap is now below `+940` bytes

## 2026-06-01 - Newly retained Cargo.lock current-entry eighth-newest sidecar

- Cargo.lock current-entry eighth-newest sidecar
  - Tried:
    - extend the retained lockfile current-entry recent-candidate representation one more step
    - probe a new eighth-newest candidate after the retained seventh-newest path
  - What happened:
    - broad-local stayed clean
    - one retained win:
      - `repo_Cargo.lock`: `9,042 -> 9,032`
    - unchanged controls:
      - `generated_go.sum`: stayed `151`
      - `generated_poetry.lock`: stayed `358`
      - `generated_yarn.lock`: stayed `383`
      - `generated_composer.lock`: stayed `4,112`
    - current broad-local vs C `zstd -1` became:
      - `51 / 16 / 4` better / worse / equal
      - `1,715` bytes above C on losing fixtures
  - Keep this branch.
  - Important conclusion:
    - the broadened current-entry recency family is still live on `Cargo.lock`
    - the main `Cargo.lock` gap is now below `+945` bytes

## 2026-06-01 - Newly retained Cargo.lock current-entry seventh-newest sidecar

- Cargo.lock current-entry seventh-newest sidecar
  - Tried:
    - extend the retained lockfile current-entry recent-candidate representation one more step
    - probe a new seventh-newest candidate after the retained sixth-newest path
  - What happened:
    - broad-local stayed clean
    - one retained win:
      - `repo_Cargo.lock`: `9,051 -> 9,042`
    - unchanged controls:
      - `generated_go.sum`: stayed `151`
      - `generated_poetry.lock`: stayed `358`
      - `generated_yarn.lock`: stayed `383`
      - `generated_composer.lock`: stayed `4,112`
    - current broad-local vs C `zstd -1` became:
      - `51 / 16 / 4` better / worse / equal
      - `1,725` bytes above C on losing fixtures
  - Keep this branch.
  - Important conclusion:
    - the broadened current-entry recency family is still live on `Cargo.lock`
    - the main `Cargo.lock` gap is now below `+955` bytes

## 2026-06-01 - Newly retained Cargo.lock current-entry sixth-newest sidecar

- Cargo.lock current-entry sixth-newest sidecar
  - Tried:
    - extend the retained lockfile current-entry recent-candidate representation one more step
    - probe a new sixth-newest candidate after the retained fifth-newest path
  - What happened:
    - broad-local stayed clean
    - one retained win:
      - `repo_Cargo.lock`: `9,065 -> 9,051`
    - unchanged controls:
      - `generated_go.sum`: stayed `151`
      - `generated_poetry.lock`: stayed `358`
      - `generated_yarn.lock`: stayed `383`
      - `generated_composer.lock`: stayed `4,112`
    - current broad-local vs C `zstd -1` became:
      - `51 / 16 / 4` better / worse / equal
      - `1,734` bytes above C on losing fixtures
  - Keep this branch.
  - Important conclusion:
    - the broadened current-entry recency family is still live on `Cargo.lock`
    - the main `Cargo.lock` gap is now below `+965` bytes

## 2026-06-01 - Newly retained Cargo.lock current-entry fifth-newest sidecar

- Cargo.lock current-entry fifth-newest sidecar
  - Tried:
    - extend the retained lockfile current-entry recent-candidate representation one more step
    - probe a new fifth-newest candidate after the retained fourth-newest path
  - What happened:
    - broad-local stayed clean
    - one retained win:
      - `repo_Cargo.lock`: `9,073 -> 9,065`
    - unchanged controls:
      - `generated_go.sum`: stayed `151`
      - `generated_poetry.lock`: stayed `358`
      - `generated_yarn.lock`: stayed `383`
      - `generated_composer.lock`: stayed `4,112`
    - current broad-local vs C `zstd -1` became:
      - `51 / 16 / 4` better / worse / equal
      - `1,748` bytes above C on losing fixtures
  - Keep this branch.
  - Important conclusion:
    - the broadened current-entry recency family still has real lockfile headroom
    - the main `Cargo.lock` gap is now below `+980` bytes

## 2026-06-01 - Newly retained Cargo.lock current-entry fourth-newest sidecar

- Cargo.lock current-entry fourth-newest sidecar
  - Tried:
    - extend the retained lockfile current-entry recent-candidate representation one more step
    - probe a new fourth-newest candidate after the retained third-newest path
  - What happened:
    - broad-local stayed clean
    - one retained win:
      - `repo_Cargo.lock`: `9,087 -> 9,073`
    - unchanged controls:
      - `generated_go.sum`: stayed `151`
      - `generated_poetry.lock`: stayed `358`
      - `generated_yarn.lock`: stayed `383`
      - `generated_composer.lock`: stayed `4,112`
    - current broad-local vs C `zstd -1` became:
      - `51 / 16 / 4` better / worse / equal
      - `1,756` bytes above C on losing fixtures
  - Keep this branch.
  - Important conclusion:
    - the broadened current-entry recency family still has real lockfile headroom
    - the main `Cargo.lock` gap is now below `+1000` bytes

## 2026-06-01 - Newly retained Cargo.lock current-entry third-newest sidecar

- Cargo.lock current-entry third-newest sidecar
  - Tried:
    - extend the retained lockfile current-entry recent-candidate representation with a third
      sidecar
    - probe the new third-newest candidate after `second_newest` and before `newest`
  - What happened:
    - broad-local stayed clean
    - one retained win:
      - `repo_Cargo.lock`: `9,104 -> 9,087`
    - unchanged controls:
      - `generated_go.sum`: stayed `151`
      - `generated_poetry.lock`: stayed `358`
      - `generated_yarn.lock`: stayed `383`
      - `generated_composer.lock`: stayed `4,112`
    - current broad-local vs C `zstd -1` became:
      - `51 / 16 / 4` better / worse / equal
      - `1,770` bytes above C on losing fixtures
  - Keep this branch.
  - Important conclusion:
    - the current-entry recent-candidate representation still had real lockfile headroom
    - `Cargo.lock` is now below a `+1000` byte gap to C on the retained baseline

## 2026-06-01 - Newly closed Cargo.lock local-parse current-window search

- Cargo.lock local-parse current-window search
  - Tried:
    - first a small local parse with simulated repeat history
    - then a widened local current-window search scoring several nearby window alternatives
  - What happened:
    - focused sweep stayed flat at `9,996`
    - the branch was reverted and the temporary tuner preset was removed
  - Do not retry this branch in the same form.
  - Important conclusion:
    - the next `Cargo.lock` move was not hidden in a slightly broader local parse around the same
      active current-window candidates
    - the retained win came from a broader current-entry recent-candidate representation instead

## 2026-06-01 - Newly closed Cargo.lock byte-class literal model and known-size frame branch

- Cargo.lock byte-class lazy-parse literal model
  - Tried:
    - inside the retained zero-literal lockfile next-position compare, replace the flat skipped
      literal penalty with a lockfile-specific byte-class literal cost model
  - What happened:
    - focused sweep stayed flat at `9,996`
    - the tune-only branch was removed
  - Do not retry this branch in the same form.
  - Important conclusion:
    - the productive lockfile lazy-parse family is not missing an obvious “literals are cheaper”
      bias in this local form

- Cargo.lock encoder surface refresh
  - Tried:
    - rerun the existing `cargo-lock-encoder` surface on the current retained baseline
  - What happened:
    - focused sweep stayed flat at `10,006`
  - Important conclusion:
    - the current Cargo.lock encoder-side nearby table/threshold surface is still exhausted

- known-size single-segment frame headers
  - Tried:
    - plumb exact content size through the file-path CLI compression flow
    - emit single-segment frames when size is known
  - What happened:
    - direct spot checks regressed across representative fixtures, including:
      - `repo_Cargo.lock`: `9,104 -> 9,105`
      - `generated_composer.lock`: `4,112 -> 4,115`
      - `generated_poetry.lock`: `358 -> 361`
      - `generated_pubspec.lock`: `229 -> 232`
    - the branch was reverted immediately
  - Do not retry this branch in the same form.
  - Important conclusion:
    - the current unknown-size frame form is already smaller than a known-size single-segment
      frame on these file inputs

## 2026-06-01 - Newly bounded wider Cargo.lock lazy-parse and composer repeat-kind >2 surfaces

- Cargo.lock widened lazy-parse family
  - Tried:
    - widen the searched retained lazy-parse surface to:
      - `MAX_SKIP_LITERALS = 2/3/4`
      - `MAX_CURRENT_MATCH_LEN = 7/9/12`
      - `MATCH_LOSS_MAX = 0/1`
      - `LITERAL_WEIGHT = 6/8`
      - `OFFSET_WEIGHT = 3/4`
      - `MARGIN = 0/1/2`
    - then combine the retained lazy-parse family with nearby retained lockfile tie-breaks:
      - repeat-kind loss `1/2`
      - same-end loss `1/2`
      - same-end bits gain `2/3`
  - What happened:
    - both focused sweeps stayed flat at `9,996`
  - Do not retry these local surfaces in the same form.
  - Important conclusion:
    - the productive `Cargo.lock` lazy-parse family is bounded again on the wider searched local
      surface

- composer repeat-kind wider loss budget
  - Tried:
    - keep the retained composer probe step `5`
    - widen the same-start repeat-kind match-loss budget to `3` and `4`
    - also recheck probe step `6`
  - What happened:
    - focused family stayed flat at `11,448` for step `5`
    - step `6` still regressed
    - direct per-fixture check confirmed `loss=3` is an exact alias of the retained `loss=2`
      point
  - Do not retry this branch in the same form.
  - Important conclusion:
    - the retained composer same-start repeat-kind family is bounded upward: `2`, `3`, and `4`
      are equivalent on the live focused family

## 2026-06-01 - Newly retained wider composer repeat-kind same-start branch

- composer same-start repeat-kind preference
  - Tried:
    - keep the retained composer-profile same-start repeat-kind branch
    - widen the allowed match loss from `1` to `2`
    - only for composer-profiled files
  - What happened:
    - broad-local stayed clean
    - one retained win:
      - `generated_composer.lock`: `4,119 -> 4,112`
    - unchanged controls:
      - `generated_pipfile.lock`: stayed `2,804`
      - `generated_package-lock.json`: stayed `4,381`
      - `generated_go.sum`: stayed `151`
    - current broad-local vs C `zstd -1` became:
      - `51 / 16 / 4` better / worse / equal
      - `1,787` bytes above C on losing fixtures
  - Keep this branch.
  - Important conclusion:
    - the whole-file composer profile still had headroom in the same-start repeat-kind family
    - the remaining composer gap is still sequence-section heavy, not a classifier problem

## 2026-06-01 - Kept Cargo.lock matcher-profile plumbing, closed broader exact LL/ML candidate window

- Cargo.lock matcher-profile plumbing
  - Kept:
    - the internal `Cargo.lock` profile now reaches the matcher as well as the encoder
    - `Cargo.lock` blocks can take the exact profile path on the parse side without relying only
      on content heuristics
  - What happened:
    - focused restore compare returned exact equality to the retained
      `lockfamily-encoded-maxlog` baseline
  - Keep this groundwork.

- Cargo.lock broader exact LL/ML candidate window
  - Tried:
    - for `Cargo.lock`, widen the exact sequence-mode LL/ML candidate set to include predefined
      LL/ML tables up to `1024` sequences
  - What happened:
    - exact byte-for-byte no-op on the focused lockfile family
    - the branch was reverted
  - Do not retry this branch in the same form.
  - Important conclusion:
    - `Cargo.lock` still does not move on a broader nearby exact LL/ML table-candidate family

## 2026-06-01 - Kept Cargo.lock profile scaffold, closed zero-literal rewrite family

- Cargo.lock profile scaffold
  - Kept:
    - a dedicated internal `CompressionFileProfile::CargoLock`
    - named files like `repo_Cargo.lock` now carry a specific profile hook for future
      extension-based compression work
  - What happened:
    - output bytes stayed exactly on the retained baseline
    - restore check returned exact equality afterward
  - Keep this scaffold.

- lockfile zero-literal post-parse rewrite
  - Tried:
    - rebuild the prepared `Cargo.lock` block after converting selected short zero-literal
      matches into literals
    - keep repeat-history consistency by rebuilding the prepared representation end to end
  - Swept:
    - `RUZSTD_TUNE_LOCKFILE_DROP_ZERO_LITERAL_MATCH_MAX_LEN = 5/6/7/8`
    - `RUZSTD_TUNE_LOCKFILE_DROP_ZERO_LITERAL_MATCH_MIN_OF_CODE = 7/8/9/10`
  - Spot-checked:
    - `(4,10)`, `(4,9)`, `(3,9)`
  - What happened:
    - `max_len >= 5` regressed
    - `max_len 3/4` was exact byte-for-byte flat
    - the rewrite branch was removed
    - restore check returned exact equality to the retained `lockfamily-encoded-maxlog`
      baseline
  - Do not retry this family in the same form.
  - Important conclusion:
    - the first valid post-parse short-match literalization family is bounded already
    - the next credible `Cargo.lock` branch still needs a different literal/sequence
      representation

## 2026-06-01 - Newly closed lockfile lazy-parse family

- lockfile-family one-step lazy parse
  - Tried:
    - add a tune-only one-step lazy parse for lockfile-like `DictionaryText`
    - compare the current candidate against the best next-position candidate with a cheap local
      score and configurable gain margin
  - Swept:
    - `RUZSTD_TUNE_LOCKFILE_LAZY_SCORE_DIVISOR = 1/2/3/4/5/6`
    - `RUZSTD_TUNE_LOCKFILE_LAZY_MIN_GAIN = 0/1/2/3`
  - What happened:
    - focused sweep stayed flat at `10,002`
    - the branch was reverted
    - restore check returned exact equality to the retained `lockfamily-encoded-maxlog` baseline
  - Do not retry this family in the same form.
  - Important conclusion:
    - `Cargo.lock` is not waiting on a one-step lazy-parse family around the current matcher

## 2026-06-01 - Newly closed lockfile non-repeat offset-score family

- lockfile-family non-repeat offset-score comparer
  - Tried:
    - add a tune-only broader lockfile candidate scorer using:
      - `match_len * divisor - offset_code_bits`
    - sweep divisors `1..=6`
  - What happened:
    - focused sweep stayed flat at `10,002`
    - the branch was reverted
    - restore check returned exact equality to the retained `lockfamily-encoded-maxlog` baseline
  - Do not retry this family in the same form.
  - Important conclusion:
    - `Cargo.lock` is not waiting on a broader non-repeat offset-score rule either

## 2026-06-01 - Newly closed generic smaller-offset lockfile family

- lockfile-family generic smaller-offset matcher preference
  - Tried:
    - add a general non-repeat smaller-offset preference for lockfile-like `DictionaryText`
    - sweep broader loss/bit-gain thresholds than the already-retained same-start and same-end
      special cases
  - What happened:
    - focused sweep stayed flat at `10,002`
    - the default branch was also an exact byte-for-byte no-op on broad-local against the
      retained `lockfamily-encoded-maxlog` baseline
    - restore check returned exact equality afterward
  - Do not retry this family in the same form.
  - Important conclusion:
    - `Cargo.lock` is not waiting on a generic smaller-offset matcher preference either

## 2026-06-01 - Newly closed exact encoded-table normalization branch

- lockfile-family exact encoded-table normalization variants
  - Tried:
    - keep the retained exact encoded-table max-log search
    - for encoded LL/ML/OF candidates, also compare the alternate valid `avoid_0_numbit`
      normalization setting
  - What happened:
    - exact byte-for-byte no-op on broad-local against the retained
      `lockfamily-encoded-maxlog` baseline
    - restore check returned exact equality afterward
  - Do not retry this branch in the same form.
  - Important conclusion:
    - the retained lockfile-family exact sequence search is not missing another nearby FSE
      normalization variant

## 2026-06-01 - Newly retained broader exact encoded-table log branch

- lockfile-family exact encoded-table log search
  - Tried:
    - keep the retained exact LL/ML/OF sequence-mode comparison
    - but, for encoded LL/ML/OF candidates, compare additional valid encoded-table max-log
      choices in the `7..=max_log` range
  - What happened:
    - broad-local stayed clean
    - one retained win:
      - `generated_package-lock.json`: `4,383 -> 4,381`
    - unchanged key controls:
      - `repo_Cargo.lock`: stayed `9,109`
      - `generated_composer.lock`: stayed `4,159`
      - `generated_pipfile.lock`: stayed `2,804`
  - Keep this branch.
  - Important conclusion:
    - broader exact encoded-table log search is valid and useful for dependency-JSON lockfiles
    - it still does not unlock `Cargo.lock` or composer

## 2026-06-01 - Newly closed lockfile next-position lookahead branch

- lockfile-family next-position window/repeat lookahead
  - Tried:
    - enable next-position window lookahead for lockfile-like `DictionaryText`
    - compare next-position repeat candidates even when a current repeat candidate already exists
  - What happened:
    - exact byte-for-byte no-op on broad-local against the retained `lockfamily-exact-seq`
      baseline
    - restore check returned exact equality afterward
  - Do not retry this branch in the same form.
  - Important conclusion:
    - `Cargo.lock` is not waiting on next-position window or repeat lookahead

## 2026-06-01 - Newly closed lockfile long-hash branch

- lockfile-family current-entry long-hash path
  - Tried:
    - enable the existing current-entry long-hash matcher path for lockfile-like `DictionaryText`
  - What happened:
    - exact byte-for-byte no-op on broad-local against the retained `lockfamily-exact-seq` baseline
    - restore check returned exact equality afterward
  - Do not retry this branch in the same form.
  - Important conclusion:
    - the dormant current-entry long-hash path is not the missing `Cargo.lock` representation

## 2026-06-01 - Newly retained exact LL/ML/OF sequence-mode branch

- lockfile-family exact LL/ML/OF sequence-mode comparison
  - Tried:
    - start from the existing heuristic chooser family for LL, ML, and OF
    - for fastest-level `DictionaryText` and dependency-JSON-lockfile profiles, exactly re-encode the sequence section across the valid LL/ML/OF combinations
    - keep the smallest valid combination
  - What happened:
    - broad-local stayed clean
    - one retained win:
      - `generated_pubspec.lock`: `233 -> 232`
    - unchanged key controls:
      - `repo_Cargo.lock`: stayed `9,109`
      - `generated_composer.lock`: stayed `4,159`
      - `generated_package-lock.json`: stayed `4,383`
      - `generated_pipfile.lock`: stayed `2,804`
    - current broad-local vs C `zstd -1` now is:
      - `51 / 16 / 4` better / worse / equal
      - `1,842` bytes above C on losing fixtures
  - Keep this branch.
  - Important conclusion:
    - exact full sequence-mode comparison is broad-safe, but still not the `Cargo.lock` / composer unlock
    - the next credible branch remains a broader literal/sequence representation change

## 2026-06-01 - Newly retained exact OF-mode search branch

- lockfile-family exact OF-mode sequence-section comparison
  - Tried:
    - for fastest-level `DictionaryText` and dependency-JSON-lockfile profiles, do not trust the OF threshold chooser alone
    - instead, enumerate the valid OF table modes, exactly re-encode the sequence section, and keep the smallest OF choice
  - What happened:
    - broad-local stayed clean
    - one retained win:
      - `generated_Gemfile.lock`: `240 -> 239`
    - unchanged key controls:
      - `repo_Cargo.lock`: stayed `9,109`
      - `generated_composer.lock`: stayed `4,159`
      - `generated_package-lock.json`: stayed `4,383`
      - `generated_pipfile.lock`: stayed `2,804`
    - current broad-local vs C `zstd -1` remains:
      - `51 / 16 / 4` better / worse / equal
      - `1,843` bytes above C on losing fixtures
  - Keep this branch.
  - Important conclusion:
    - exact OF-mode comparison is broad-safe, but it is not the missing `Cargo.lock` / composer breakthrough
    - the next credible branch is still a broader sequence/literal representation change

## File-Type Starting Points

### Current state

- The public API and CLI accept path/file-type hints without exposing tuning knobs.
- Important: the live source tree no longer matches some older `DictionaryText` retained reports.
  - the drift is now reconciled
  - the corrected retained live result for `dict_dictionary.bin` is `20,160`
  - the corrected retained live result for `decodecorpus_z000079` is `7,321`
- Current broad-local level-1 summary vs C `zstd -1`:
  - better / worse / equal: `43 / 10 / 4`
  - bytes-above-C on losing fixtures: `1,725`
- The broad-local suite now includes more explicit known-file-type fixtures from the repo:
  - unique `Cargo.toml` fixtures instead of silent collisions
  - `Cargo.lock`
  - `.github/workflows/ci.yml`
  - root and fuzz `.gitignore`
  - additional `Cargo.toml` files
  - extra Rust/Python source files
  - a broader spread of `.service` files
  - generated lockfile/config fixtures:
    - `generated_package-lock.json`
    - `generated_composer.lock`
    - `generated_pipfile.lock`
    - `generated_Gemfile`
    - `generated_Gemfile.lock`
    - `generated_yarn.lock`
    - `generated_poetry.lock`
    - `generated_go.sum`
    - `generated_go.mod`
    - `generated_requirements.txt`
- Retained runtime starting points now exist for block entropy decisions:
  - `DictionaryText`: exact Huffman table search for all literal sections
  - Huffman weight tables longer than `16` symbols compare FSE table encodings at max-log `6` and `5`, keeping the shorter byte sequence
  - large composer-style `DictionaryText` blocks at level 1 now use the existing partitioned-block candidate path
  - that composer partitioned-block path now uses the live fastest-level file-type block config instead of the generic `Best` block config
  - `CodeText`: exact Huffman table search for small literal sections
  - `CodeText`: dense short-line probing up to `96 KiB`
  - `CodeText`: short-line blocks up to `16 KiB` use a `5`-byte non-repeat floor
  - `ConfigText`: exact Huffman table search for small literal sections
  - `ConfigText`: predefined LL/ML tables for compressed-literals blocks up to `64` sequences
  - `ConfigText`: compressed literal sections up to `1024` literals may force the single-stream Huffman path
  - named-file matching now also accepts suffix matches with a separator, so synthetic and prefixed names like `repo_.gitignore` and `repo_Cargo.lock` still land on their intended named-file families
- Retained matcher specialization now also exists for one path/file-type family:
  - `Unknown` non-text at level 1: extend the Fastest current-entry `second_newest` path up to `128 KiB`
  - `Unknown` non-text at level 1: disable long-repeat early-exit for large blocks
- `DictionaryText` now also has a retained matcher specialization:
  - fully dense no-match probe step (`1`)
- retained named-file starting-point specialization now also exists:
  - `Cargo.lock` -> `DictionaryText`
- retained lockfile-like matcher specialization now also exists:
  - `Cargo.lock`-like `DictionaryText` text blocks enable the current-entry `second_newest` sidecar at level 1
  - `Cargo.lock`-like `DictionaryText` now probes current-entry `second_newest` before `newest`
  - `Cargo.lock`-like `DictionaryText` now keeps the current candidate over `oldest` unless `oldest` gains at least `2` match bytes
  - both adjacent bounds are rejected: `1` and `3`
  - small-sequence LL/ML max-log `8` and `7` are both rejected as no-ops
  - the next bound at `3` is rejected
  - zero-literal repeat-margin bonus is rejected
  - current-vs-`second_newest` displacement is rejected
  - wider same-start smaller-offset rule is rejected again on the new parser shape
- retained `second_newest` probe-admission fix now also exists:
  - tracked `second_newest` sidecars are now actually consulted through `should_track_second_newest_for_current_entry()`
  - lockfile classification is cached per block so the gate does not rescan hot `DictionaryText` blocks
- The matcher path itself is still shared; retained file-type specialization is currently on the literal-section encoding side, not the match-search side.

### Newly retained composer structural branch

- composer-style `DictionaryText` partitioned-block path
  - Tried:
    - on level 1 for large composer-style `DictionaryText`, use the existing `compress_best_with_estimated_splits()` path instead of the plain Fastest single-block path
    - gate it with `likely_composer_lockfile_text()`
  - What happened:
    - first real retained composer-family win:
      - `generated_composer.lock`: `4,461 -> 4,340`
    - unchanged nearby controls:
      - `generated_pipfile.lock`: stayed `2,811`
      - `generated_package-lock.json`: stayed `4,392`
      - `generated_go.sum`: stayed `151`
      - `repo_Cargo.lock`: stayed `9,114`
    - broad-local bytes-above-C on losers moved:
      - `1,850 -> 1,729`
  - Keep this branch. It is the live retained starting point for large composer-style lockfiles.

- composer partition candidates use fastest-level file-type block config
  - Tried:
    - on the retained composer partition path, use `BlockCompressionConfig::for_level_and_file_type(CompressionLevel::Fastest, state.file_type_hint)` instead of the generic `Best` block config while encoding the partition candidates
  - What happened:
    - focused composer-family win:
      - `generated_composer.lock`: `4,340 -> 4,336`
    - unchanged nearby controls:
      - `generated_pipfile.lock`: stayed `2,811`
      - `generated_package-lock.json`: stayed `4,392`
      - `generated_go.sum`: stayed `151`
      - `repo_Cargo.lock`: stayed `9,114`
    - broad-local bytes-above-C on losers moved:
      - `1,729 -> 1,725`
  - Keep this branch. It is now part of the live composer partition starting point.

### Newly closed composer partition sequence-table branches

- composer-style `DictionaryText` min non-repeat floor `5`
  - Tried:
    - use minimum non-repeat match length `5` for composer-style `DictionaryText` blocks
    - motivated by the fact that live composer text was staying on the generic text floor `8`, so several earlier `min_match_len == 5` branches had no chance to fire
  - What happened:
    - exact byte-for-byte no-op on the focused composer/lockfile family
    - `generated_composer.lock`: stayed `4,336`
  - Do not retry this lower non-repeat floor branch for composer in the same form.

- composer-style `DictionaryText` no-match probe step `2`
  - Tried:
    - use no-match probe step `2` for composer-style `DictionaryText` blocks
    - motivated by the retained `Cargo.lock` win from a less-dense lockfile probe step
  - What happened:
    - exact byte-for-byte no-op on the focused composer/lockfile family
    - `generated_composer.lock`: stayed `4,336`
  - Do not retry this probe-density transfer from `Cargo.lock` to composer in the same form.

- composer-style `DictionaryText` second_newest-before-newest probing
  - Tried:
    - probe current-entry `second_newest` before `newest` for composer-style `DictionaryText` blocks
    - motivated by the retained `Cargo.lock` win from the same ordering family
  - What happened:
    - exact byte-for-byte no-op on the focused composer/lockfile family
    - `generated_composer.lock`: stayed `4,336`
  - Do not retry this lockfile-style `second_newest` probe ordering on composer in the same form.

- composer-style `DictionaryText` without the special text-repeat pipeline
  - Tried:
    - keep composer-style `DictionaryText` off the special text-repeat pipeline while leaving the rest of `DictionaryText` unchanged
    - motivated by the remaining composer gap still being overwhelmingly sequence/offset-side
  - What happened:
    - exact byte-for-byte no-op on the focused composer/lockfile family
    - `generated_composer.lock`: stayed `4,336`
  - Do not retry this pipeline distinction in the same form.

- composer actual encoded partition-budget search across `1..=8`
  - Tried:
    - for composer-style `DictionaryText`, compare the actually encoded candidates produced by partition budgets `1` through `8` and keep the smallest one
    - motivated by the remaining gap between the retained `4`-block shape and C's `2`-block shape
  - What happened:
    - exact byte-for-byte no-op on the focused composer/lockfile family
    - `generated_composer.lock`: stayed `4,336`
  - Do not retry this broader actual-budget search over the current partition-tree family in the same form.

- strict partition-budget enforcement for estimated split recursion
  - Tried:
    - fix `derive_best_partitions()` so the requested partition budget is enforced strictly instead of letting the left subtree consume the budget and still appending the right half
    - add a focused unit test that proves the helper cannot exceed a `2`-partition budget on a recursively splittable synthetic block
  - What happened:
    - the bug was real
    - focused restore versus the retained `composer-filetypeconfig` binary was exact:
      - `generated_composer.lock`: stayed `4,336`
      - `generated_pipfile.lock`: stayed `2,811`
      - `generated_package-lock.json`: stayed `4,392`
      - `generated_go.sum`: stayed `151`
      - `repo_Cargo.lock`: stayed `9,114`
  - Keep this fix. It is a correctness fix with test coverage, and it leaves the live retained composer family byte-identical in its default `8`-partition form.

- composer partition path capped at `2` partitions
  - Tried:
    - after fixing the partition-budget bug, force the composer-style `DictionaryText` partition path down to `2` partitions
  - What happened:
    - focused regression:
      - `generated_composer.lock`: `4,336 -> 4,389`
    - unchanged nearby controls:
      - `generated_pipfile.lock`: stayed `2,811`
      - `generated_package-lock.json`: stayed `4,392`
      - `generated_go.sum`: stayed `151`
      - `repo_Cargo.lock`: stayed `9,114`
  - Do not retry this hard `2`-partition composer cap in the same form.

- composer partition candidates repeat previous FSE tables up to `1024` sequences
  - Tried:
    - on the retained composer partition path, raise the repeat-table ceiling so the partition blocks can reuse previous FSE tables instead of emitting new ones
  - What happened:
    - hard regression on the focused composer family:
      - `generated_composer.lock`: `4,336 -> 4,524`
  - Do not retry this broad repeat-table reuse branch in the same form.

- composer partition candidates predefined OF up to `1024` sequences
  - Tried:
    - on the retained composer partition path, enlarge the predefined-OF window so the partition blocks can skip emitting custom OF tables
  - What happened:
    - hard regression on the focused composer family:
      - `generated_composer.lock`: `4,336 -> 5,025`
  - Do not retry this wide predefined-OF branch in the same form.

### Newly closed Cargo.lock branches

- lockfile partition path with fastest-level file-type block config
  - Tried:
    - on lockfile-like `DictionaryText`, retest the partitioned-block candidate path after composer proved that the live fastest-level file-type block config mattered there
  - What happened:
    - exact no-op on the focused lockfile family
    - `repo_Cargo.lock`: stayed `9,114`
  - Do not retry this exact lockfile partition-path retest in the same form.

- lockfile current-over-`oldest` when a two-byte `oldest` gain is offset-expensive
  - Tried:
    - keep the current candidate over a farther `oldest` non-repeat candidate when the `oldest` candidate gains exactly two bytes but costs at least two more offset-code bits
  - What happened:
    - focused regression:
      - `repo_Cargo.lock`: `9,114 -> 9,116`
  - Do not retry this exact lockfile `oldest` bits branch in the same form.

- lockfile current-over-`newest` when a two-byte `newest` gain is offset-expensive
  - Tried:
    - keep the current candidate over a farther `newest` non-repeat candidate when the `newest` candidate gains exactly two bytes but costs at least two more offset-code bits
  - What happened:
    - exact no-op on the focused lockfile family
    - `repo_Cargo.lock`: stayed `9,114`
  - Do not retry this exact lockfile `newest` bits branch in the same form.

### Newly closed `ConfigText` / mapping variants

### Newly closed `DictionaryText` literal-layout variant

- `DictionaryText` adaptive single-stream vs four-stream Huffman literal layout
  - Tried:
    - on the `DictionaryText` literal path, compare single-stream vs four-stream Huffman layouts and keep the smaller estimated encoding
    - motivated by the remaining `Cargo.lock` literal-stream gap versus C
  - What happened:
    - exact byte-for-byte no-op on the focused dictionary/lockfile family
    - `repo_Cargo.lock`: stayed `9,114`
    - `dict_dictionary.bin`: stayed `19,668`
    - `generated_go.sum`: stayed `151`
    - `generated_poetry.lock`: stayed `359`
    - `generated_yarn.lock`: stayed `383`
  - Do not retry this stream-layout comparison family in the same form.

- lockfile smaller dense post-match insertion limit
  - Tried:
    - for lockfile-like `DictionaryText`, reduce the dense post-match suffix insertion limit from `128` to `64`
    - motivated by `repo_Cargo.lock` still being slightly over-sequenced versus C
  - What happened:
    - `repo_Cargo.lock`: `9,114 -> 9,116`
    - rest of the focused lockfile family stayed flat
  - Do not retry this smaller dense post-match insertion family in the same form.

- lockfile larger dense post-match insertion limit
  - Tried:
    - for lockfile-like `DictionaryText`, increase the dense post-match suffix insertion limit from `128` to `256`
    - to bound the opposite side of the same parse-representation family after `64` regressed
  - What happened:
    - exact byte-for-byte no-op on the focused lockfile family
    - `repo_Cargo.lock`: stayed `9,114`
  - Do not retry this larger dense post-match insertion family in the same form.

- `DictionaryText` predefined OF up to `1024` sequences
  - Tried:
    - on the `DictionaryText` path, let OF use predefined tables up to `1024` sequences instead of the generic `16`
    - motivated by the remaining `Cargo.lock` offset-code shape mismatch versus C
  - What happened:
    - exact byte-for-byte no-op on the focused dictionary/lockfile family
    - `repo_Cargo.lock`: stayed `9,114`
    - `dict_dictionary.bin`: stayed `19,668`
  - Do not retry this predefined-OF window in the same form.

- lockfile package-boundary raw-data multi-block split
  - Tried:
    - on the fastest path for lockfile-like `DictionaryText`, split the raw input into package-aligned segments before matching and emit multiple compressed blocks
    - verified the helper would split `repo_Cargo.lock` into four segments: `8193 / 8217 / 8197 / 7251`
  - What happened:
    - exact byte-for-byte no-op on the focused dictionary/lockfile family
    - `repo_Cargo.lock`: stayed `9,114`
  - Do not retry this package-aligned raw-data multi-block split in the same form.

- `composer.lock` / `Pipfile.lock` -> `JsonText`
  - Why it looked plausible:
    - the expanded known-file-type corpus exposed `generated_composer.lock` as a large loser
    - both lockfiles are JSON-shaped formats
  - What happened:
    - focused regression:
      - `generated_composer.lock`: `4,461 -> 4,482`
      - `generated_pipfile.lock`: `2,811 -> 2,885`
      - `repo_Cargo.lock`: unchanged at `9,114`
  - Do not retry the plain `JsonText` starting point for these lockfiles in the same form.

- small short-line `ConfigText` current-over-`oldest` displacement
  - Tried:
    - for small short-line `ConfigText`, keep the current candidate over a farther `oldest` non-repeat candidate unless `oldest` gains at least `2` match bytes
    - motivated by `repo_ruzstd_Cargo.toml` still having fewer sequences and more literals than C
  - What happened:
    - exact byte-for-byte no-op on the focused `ConfigText` family
    - `repo_ruzstd_Cargo.toml`: stayed `730`
    - `repo_Cargo.toml`: stayed `68`
    - `repo_ci.yml`: stayed `556`
    - `repo_.gitignore`: stayed `166`
  - Do not retry this current-vs-`oldest` `ConfigText` displacement family in the same form.

- `CompressionFileType::ConfigText` tiny literals -> keep smallest-table Huffman search active even for flat distributions
  - Tried:
    - only on the tiny `ConfigText` literal path that still trails C
    - removed the current flat-distribution early return from the exact-search branch in that narrow case
  - Why it looked plausible:
    - fresh archive inspection showed `repo_.gitignore` matching C on parse, stream count, and sequence modes
    - the remaining gap was pure literal-payload-side, and the current “exact” search still does not search flat distributions
  - What happened:
    - exact byte-for-byte no-op
    - `repo_.gitignore`: stayed `172`
    - `dict_talk.service`: stayed `160`
    - `repo_Cargo.toml`: stayed `730`
  - Do not retry this flat-distribution exhaustive-search family in the same form.

- `.toml` extension -> `CodeText`
  - Why it looked plausible:
    - known file types matter more than `Unknown`
    - `repo_Cargo.toml` was still a named known-family loser
    - this was a pure extension-based starting-point experiment, aligned with the public API design
  - What happened:
    - only moved the target, and moved it the wrong way:
      - `repo_Cargo.toml`: `730 -> 732`
  - Do not retry the plain `.toml -> CodeText` remap in the same form.

- `Cargo.lock` filename -> `CodeText`
  - Why it looked plausible:
    - the corrected broad-local suite exposed `repo_Cargo.lock` as the largest known-file-type loser
    - fresh archive inspection showed a broader parse mismatch, not just a tiny literal tail
    - this was the narrowest extension/name-based starting-point experiment for that file family
  - Updated result after retaining suffix-based named-file matching:
    - `repo_Cargo.lock`: `9,255 -> 9,240`
  - This is no longer a dead no-op, but `DictionaryText` is the retained policy fit for `Cargo.lock`.
  - Do not keep a separate `Cargo.lock -> CodeText` policy point while `DictionaryText` gives the same measured byte win.

- `Cargo.lock` filename -> `DictionaryText`
  - Why it looked plausible:
    - the `Cargo.lock` archive is over-sequenced and paying more offset and literal payload than C
    - among existing file-type families, `DictionaryText` is the only retained path with stronger offset-side behavior
  - Updated result after retaining suffix-based named-file matching:
    - `repo_Cargo.lock`: `9,255 -> 9,240`
    - nothing else moved
  - This is retained as the current `Cargo.lock` starting point.

- dedicated public `LockfileText` family
  - Why it looked plausible:
    - `repo_Cargo.lock` became the largest known-file-type loser after fixing duplicate fixture-name collisions in `broad-local`
    - `Cargo.lock` looked broad enough to justify more than a plain remap into an existing family
  - Tried:
    - new public `CompressionFileType::LockfileText`
    - dense probing, exact Huffman search, `offset_table_max_log = 7`, stronger short-line floor
  - What happened:
    - target regressed:
      - `repo_Cargo.lock`: `9,240 -> 9,288`
    - corrected broad-local total bytes-above-C on losers regressed:
      - `1,411 -> 1,459`
  - Do not retry this public-family split in the same form. Keep the narrower retained policy:
    - suffix-based named-file matching
    - `Cargo.lock -> DictionaryText`

- lockfile-like `DictionaryText` -> raise short-line non-repeat floor from `5` to `7`
  - Why it looked plausible:
    - `repo_Cargo.lock` is still overpaying both literal and sequence payload versus C
    - a stronger floor was the narrowest way to reduce local over-sequencing without another public family split
  - What happened:
    - target regressed:
      - `repo_Cargo.lock`: `9,240 -> 9,288`
  - Do not retry this “higher lockfile floor” cut in the current matcher representation.

- lockfile-like `DictionaryText` -> admit next-position window lookahead
  - Why it looked plausible:
    - the retained lockfile parser now has real `second_newest` wins
    - adjacent-position families have paid off elsewhere
    - this was the narrowest way to test whether lockfile text wanted the same `ip+1` normal-window shape
  - What happened:
    - exact byte-for-byte no-op
    - `repo_Cargo.lock`: stayed `9,170`
  - Do not retry this lockfile next-position window branch in the current matcher representation.

- lockfile-like `DictionaryText` -> fixed newline-aligned multi-block split around `8 KiB`
  - Why it looked plausible:
    - the remaining `Cargo.lock` gap is still mostly literal-side
    - a simple structural split could have changed the coded literal stream without another matcher threshold
  - What happened:
    - exact byte-for-byte no-op across the focused lockfile family:
      - `repo_Cargo.lock`: stayed `9,114`
      - `generated_go.sum`: stayed `151`
      - `generated_poetry.lock`: stayed `359`
      - `generated_yarn.lock`: stayed `383`
  - Do not retry this fixed newline-aligned multi-block split family in the same form.

- lockfile-like `DictionaryText` -> require `6` bytes for zero-literal non-repeat window matches
  - Why it looked plausible:
    - live `Cargo.lock` histograms still show too many zero-literal sequences versus C
    - this was the narrowest way to cut only the shortest zero-literal non-repeat window matches without changing repeat handling or with-literal matches
  - What happened:
    - target regressed:
      - `repo_Cargo.lock`: `9,114 -> 9,143`
    - the rest of the focused lockfile family stayed unchanged:
      - `generated_go.sum`: `151`
      - `generated_poetry.lock`: `359`
      - `generated_yarn.lock`: `383`
  - Do not retry this blunt zero-literal extra-floor family in the same form.

- lockfile-like `DictionaryText` -> disable backward match extension
  - Why it looked plausible:
    - the remaining `Cargo.lock` loss is still mostly in coded literals versus C
    - backward extension is one of the remaining parse operations that directly shrinks literal runs before entropy coding
  - What happened:
    - exact byte-for-byte no-op across the focused lockfile family:
      - `repo_Cargo.lock`: stayed `9,114`
      - `generated_go.sum`: stayed `151`
      - `generated_poetry.lock`: stayed `359`
      - `generated_yarn.lock`: stayed `383`
  - Do not retry this no-backward-extension branch in the same form.

- small short-line `ConfigText` -> enable current-entry `second_newest`
  - Why it looked plausible:
    - fresh `repo_ruzstd_Cargo.toml` evidence showed a parser-side gap versus C:
      - Rust: `821` literals, `51` sequences
      - C: `732` literals, `71` sequences
    - this was the narrowest known-file-type parser representation branch that could increase short-line config match density without remapping the family
  - What happened:
    - exact byte-for-byte no-op across `broad-local`
    - no fixture bytes moved at all
  - Do not retry this small-`ConfigText` current-entry `second_newest` family in the same form.

- small short-line `ConfigText` -> enable next-position window lookahead
  - Why it looked plausible:
    - fresh `repo_ruzstd_Cargo.toml` matcher diagnostics still showed only current-position window wins:
      - `window_current_newest = 22`
      - `window_current_oldest = 28`
      - `window_next_position_* = 0`
    - this was the narrowest way to test whether the under-sequenced small ConfigText path wanted the existing `ip+1` normal-window branch
  - What happened:
    - exact byte-for-byte no-op across `broad-local`
    - no fixture bytes moved at all
  - Do not retry this small-`ConfigText` next-position window family in the same form.

- tiny single-stream `ConfigText` -> choose exact Huffman tables by actual encoded bytes
  - Why it looked plausible:
    - fresh `repo_.gitignore` inspection still showed a pure literal-side tail with the same parse as C:
      - Rust: `literals_payload=131`, `literals_stream=109`
      - C: `129`, `105`
    - this was the narrowest way to test whether the current exact-table candidate set was being mis-ranked by the byte estimate
  - What happened:
    - exact byte-for-byte no-op across `broad-local`
    - no fixture bytes moved at all
  - Do not retry this tiny-ConfigText actual-byte table-ranking family in the same form.

- lockfile-like `DictionaryText` -> keep the current floor but stop forcing dense probe step `1`
  - Why it looked plausible:
    - if the lockfile gap came from overly eager candidate discovery, reducing only the probe density was the next narrow cut
  - What happened:
    - target still regressed:
      - `repo_Cargo.lock`: `9,240 -> 9,255`
  - Do not retry this “less-dense lockfile probing” cut in the current matcher representation.

- lockfile-like `DictionaryText` -> same-start smaller-offset preference with up to 2 bytes of match loss
  - Why it looked plausible:
    - `repo_Cargo.lock` still pays a large offset-side gap versus C
    - the retained generic dictionary smaller-offset rule only allows 1 byte of match loss
  - What happened:
    - target still regressed:
      - `repo_Cargo.lock`: `9,240 -> 9,243`
  - Do not retry this wider lockfile-specific smaller-offset rule in the current matcher representation.

- `DictionaryText` text blocks -> enable the best-text repeat pipeline at level 1
  - Why it looked plausible:
    - it was the last remaining text-side parser shape in the current matcher that had not been tried on the lockfile family
  - What happened:
    - exact byte-for-byte no-op on corrected `broad-local`
    - `repo_Cargo.lock` stayed `9,240`
    - `dict_dictionary.bin` stayed `20,160`
  - Do not retry this `DictionaryText` text-repeat-pipeline enable in the current form.

- known-size file compression -> emit single-segment frame headers with frame content size
  - Why it looked plausible:
    - current file-archive inspections still differed from C at the frame header level
  - What happened:
    - broadly made outputs 1 to 3 bytes larger
    - examples:
      - `repo_Cargo.lock`: `9,240 -> 9,241`
      - `repo_compressed.rs`: `12,946 -> 12,949`
      - `decodecorpus_z000079`: `7,321 -> 7,324`
  - Do not retry this known-size/single-segment header branch in the current encoder shape.

- small-sequence `DictionaryText` -> lower OF max-log from `7` to `6`
  - Why it looked plausible:
    - `repo_Cargo.lock` has a large offset-side gap versus C
    - `repo_Cargo.lock` has only `836` sequences, while `dict_dictionary.bin` has over `4,000`, so a sequence-count gate looked like a clean discriminator
  - What happened:
    - target regressed hard:
      - `repo_Cargo.lock`: `9,240 -> 9,292`
  - Do not retry this small-sequence `DictionaryText oflog6` branch.

- lockfile-like `DictionaryText` -> enable current-entry `second_newest`
  - Why it looked plausible:
    - `repo_Cargo.lock` remained the dominant corrected-suite known-file-type loser after the suite collision fix
    - the obvious local matcher threshold and OF-log cuts were already closed
    - the next untried structural branch was a different current-entry candidate representation
  - What happened:
    - retained win:
      - `repo_Cargo.lock`: `9,240 -> 9,197`
    - every other corrected-suite fixture stayed byte-identical
    - corrected broad-local bytes-above-C on losers improved:
      - `1,307 -> 1,264`
  - This is retained. Do not retry the older threshold-only Cargo.lock cuts instead of this path.

- tracked `second_newest` sidecar -> gate actual probe sites by `should_track_second_newest_for_current_entry()`
  - Why it looked plausible:
    - direct matcher inspection showed the retained lockfile sidecar was being tracked but not producing visible `second_newest` wins
    - code inspection showed the probe sites were still guarded by `use_second_newest_probe` alone
    - that meant the retained lockfile path, and the older Fastest small-block `second_newest` path, were not being admitted through the intended probe gate
  - What happened:
    - retained win:
      - `repo_Cargo.lock`: `9,197 -> 9,185`
      - `build_ruzstd-cli`: `862,752 -> 854,529`
      - `decodecorpus_z000028`: `98,381 -> 95,230`
      - `decodecorpus_z000033`: `532,632 -> 530,433`
      - `decodecorpus_z000079`: `7,321 -> 7,322`
    - corrected broad-local bytes-above-C on losers improved:
      - `1,264 -> 1,253`
    - after caching the per-block lockfile classification, the temporary `dict_dictionary.bin` CPU drift disappeared from the repeat screen
  - This is retained. Treat this as a real bug fix in the `second_newest` family, not another optional threshold branch.

- lockfile-like `DictionaryText` -> probe step `2` on top of the retained `second_newest` gate-fix baseline
  - Why it looked plausible:
    - fresh retained `Cargo.lock` diagnostics after the gate fix still showed too many sequences and too much offset-side cost versus C
    - the old pre-gate-fix rejection of step `2` was no longer the same experiment, because the active lockfile-sidecar path had changed the parser shape materially
  - What happened:
    - retained win:
      - `repo_Cargo.lock`: `9,185 -> 9,170`
      - corrected broad-local bytes-above-C on losers: `1,253 -> 1,238`
      - every other corrected-suite fixture stayed byte-identical
    - new retained lockfile archive shape:
      - `sequences`: `883 -> 848`
      - `sequence_payload_bytes`: `2,418 -> 2,258`
      - `of_extra_bits`: `8,218 -> 7,164`
      - `of_codes 0`: now `70`
  - This is retained. Do not treat the older pre-gate-fix step-2 rejection as blocking this newer retained point.

- lockfile-like `DictionaryText` -> probe step `3` on top of the retained `step 2` point
  - Why it looked plausible:
    - after retaining `step 2`, the obvious next question was whether the lockfile family still wanted a wider search stride
    - this cleanly bounds the family around the new retained point
  - What happened:
    - target regressed:
      - `repo_Cargo.lock`: `9,170 -> 9,223`

- lockfile-like `DictionaryText` -> repeat margin `+1` on top of the retained `step 2` point
  - Why it looked plausible:
    - the active post-gate-fix `Cargo.lock` parser now has materially more repeat-current wins than the older rejected baseline
    - this was the narrowest repeat-side retest on the new parser shape
  - What happened:
    - exact byte-for-byte no-op
    - `repo_Cargo.lock`: stayed `9,170`
  - Do not retry this blanket repeat-margin branch on the current lockfile parser shape.

- lockfile-like `DictionaryText` -> same-start repeat-aware scoring on top of the retained `step 2` point
  - Why it looked plausible:
    - the retained `Cargo.lock` archive now has a real `of_code=0` population
    - the natural next repeat-side retest was to prefer repeats only when they start at the same byte and the competing non-repeat clearly pays extra offset-code bits
  - What happened:
    - exact byte-for-byte no-op
    - `repo_Cargo.lock`: stayed `9,170`
  - Do not retry this same-start repeat-aware scoring branch in the current form.

- lockfile-like `DictionaryText` -> raise short-line non-repeat floor from `5` to `6` on top of the retained `step 2` point
  - Why it looked plausible:
    - the older higher-floor rejection was on a different pre-gate-fix parser shape
    - this retest checked whether the active lockfile-sidecar path now wanted a slightly stricter local floor
  - What happened:
    - hard regression:
      - `repo_Cargo.lock`: `9,170 -> 9,246`
    - it also suppressed the retained `second_newest` lockfile wins entirely:
      - `window_current_second_newest[0]`: `44 -> 0`
  - Do not retry stronger lockfile floors on the active post-gate-fix parser shape.

- lockfile-like `DictionaryText` -> current-entry long-hash with the gate and allocation actually enabled on top of the retained `step 2` point
  - Why it looked plausible:
    - the older lockfile long-hash branch had been benchmarked before the active parser-shape changes and before checking that the path was really admitted
    - code inspection showed the live lockfile path still was not actually allocating or admitting current-entry long-hash
  - What happened:
    - after wiring the gate and allocation through, the target was still an exact no-op:
      - `repo_Cargo.lock`: stayed `9,170`
    - matcher diagnostics closed it cleanly:
      - `current_long_hash_found`: stayed `0`
  - Do not retry this long-hash family in the current matcher representation.

- lockfile-like `DictionaryText` -> bypass the special repeat-only text pipeline and use the general text parser path
  - Why it looked plausible:
    - this was the remaining parser-shape toggle that had not been isolated on the active lockfile parser
    - if the special repeat-only text path was the reason `Cargo.lock` stayed over-sequenced, this would have shown it directly
  - What happened:
    - exact byte-for-byte no-op
    - `repo_Cargo.lock`: stayed `9,170`
    - matcher diagnostics stayed byte-for-byte identical too
  - Do not retry this text-pipeline toggle in the current form.

- `DictionaryText` -> broaden predefined LL/ML table eligibility up to `1024` sequences
  - Why it looked plausible:
    - `repo_Cargo.lock` is still the dominant known-file-type loser
    - its remaining gap has a large sequence-payload component
    - the narrowest entropy-side test was to see whether the default LL/ML tables were already good enough if we just allowed them at a higher sequence count
  - What happened:
    - hard regression:
      - `repo_Cargo.lock`: `9,170 -> 9,408`
    - code histograms stayed identical
    - only sequence payload changed:
    - `2,258 -> 2,496`
  - Do not retry this broader predefined-LL/ML window for `DictionaryText`.

- `DictionaryText` -> compare 1-stream vs 4-stream Huffman up to `16 KiB` of literals
  - Why it looked plausible:
    - after closing the broader predefined-LL/ML branch, the next literal-side question was whether `Cargo.lock` was simply on the wrong stream mode
    - this was a stricter version of the earlier single-stream idea because it compared both modes and kept the smaller estimate
  - What happened:
    - exact byte-for-byte no-op
    - `repo_Cargo.lock`: stayed `9,170`
    - archive metrics stayed byte-for-byte identical too
  - Do not retry this 1-stream vs 4-stream literal-choice family in the current form.

- lockfile-like `DictionaryText` -> prefer repeat-offset matches that end at the same byte as a non-repeat
  - Why it looked plausible:
    - `Cargo.lock` still trails C heavily on `of_code=0` population
    - the narrowest remaining repeat-side hypothesis was that some useful repeat candidates were being discarded only because they started later while still covering the same tail
  - What happened:
    - exact byte-for-byte no-op
    - `repo_Cargo.lock`: stayed `9,170`
    - matcher diagnostics stayed byte-for-byte identical too
  - Do not retry this same-end repeat-promotion family in the current matcher representation.

- lockfile-like `DictionaryText` -> add a `third_newest` current-entry sidecar
  - Why it looked plausible:
    - after retaining the lockfile-specific `second_newest` path, the next structural representation question was whether the useful candidate was still one slot deeper in the current-entry history
    - this was a real representation change, not another threshold tweak
  - What happened:
    - exact byte-for-byte no-op
    - `repo_Cargo.lock`: stayed `9,170`
    - matcher diagnostics stayed byte-for-byte identical too
  - Do not retry this `third_newest` current-entry family in the current matcher design.

- lockfile-like `DictionaryText` -> oldest-first current-window probing
  - Why it looked plausible:
    - after local current-entry representation changes failed, the next structural parser-order question was whether the lockfile family wanted a more best-like current-window probe order
  - What happened:
    - target regressed:
      - `repo_Cargo.lock`: `9,170 -> 9,180`
  - Do not retry oldest-first current-window probing on the lockfile path.
  - Do not retry lockfile probe step `3` on this active parser shape. The family is now bounded:
    - `1` worse
    - `2` retained
    - `3` worse

- lockfile-like `DictionaryText` -> fully dense post-match suffix insertion
  - Why it looked plausible:
    - after the retained `second_newest` gate fix, `Cargo.lock` still had a large residual gap
    - sparse post-match insertion could have been suppressing additional current-entry candidate shapes and repeat-offset reuse
  - What happened:
    - exact byte-for-byte no-op
    - `repo_Cargo.lock`: stayed `9,185`
  - Do not retry this dense-insertion branch in the same form.

- lockfile-like `DictionaryText` -> override `offset_table_max_log` back to `8`
  - Why it looked plausible:
    - the remaining `Cargo.lock` gap still carries a large OF-side cost versus C
    - the retained `DictionaryText oflog7` choice was driven by `dict_dictionary.bin`, so the lockfile family might have wanted the broader table
  - What happened:
    - exact byte-for-byte no-op
    - `repo_Cargo.lock`: stayed `9,185`
  - Do not retry this lockfile-only `oflog8` override in the same form.

- lockfile-like `DictionaryText` -> enable current-entry long-hash on top of retained `second_newest`
  - Why it looked plausible:
    - after the retained `second_newest` win, the next untried current-entry representation was the existing long-hash machinery
  - What happened:
    - exact byte-for-byte no-op on corrected `broad-local`
    - `repo_Cargo.lock`: stayed `9,197`
  - Do not retry this current-entry long-hash branch in the same form.

- lockfile-like `DictionaryText` -> keep closer current candidate over farther `newest` / `oldest`
  - Why it looked plausible:
    - after the retained `second_newest` win, the next untried structural scoring rule was a large-`Unknown`-style displacement guard for the lockfile family
  - What happened:
    - exact byte-for-byte no-op on corrected `broad-local`
    - `repo_Cargo.lock`: stayed `9,197`
  - Do not retry this displacement family in the same form.

- lockfile-like `DictionaryText` -> widen repeat-match margin by 1
  - Why it looked plausible:
    - retained `Cargo.lock` sequence histograms still differ strongly from C on offset-code usage
    - C shows many `of_code=0` sequences while the retained Rust path shows none
  - What happened:
    - exact byte-for-byte no-op on corrected `broad-local`
    - `repo_Cargo.lock`: stayed `9,197`
  - Do not retry this repeat-bias family in the same form.

- lockfile-like `DictionaryText` -> enable `ip+1` repeat lookahead
  - Why it looked plausible:
    - retained `Cargo.lock` sequence histograms still differ strongly from C on repeat-offset usage
    - file-type-aware matcher diagnostics on the retained lockfile path showed very few repeat wins and zero next-position repeat promotions
  - What happened:
    - exact byte-for-byte no-op on corrected `broad-local`
    - `repo_Cargo.lock`: stayed `9,197`
  - Do not retry this lockfile next-position repeat branch in the same form.

- lockfile-like `DictionaryText` -> prefer same-start repeat over non-repeat when offset-code savings are material
  - Why it looked plausible:
    - retained lockfile diagnostics show very sparse repeat wins and a strong repeat-offset mismatch versus C
    - a same-start repeat tie-break is narrower than the already-rejected global repeat-margin changes
  - What happened:
    - exact byte-for-byte no-op on corrected `broad-local`
    - `repo_Cargo.lock`: stayed `9,197`
  - Do not retry this repeat-aware tie-break in the same form.

### Newly closed `DictionaryText` scoring variants

- `CompressionFileType::DictionaryText` -> keep the current non-repeat candidate over farther `oldest` when offset bits are much cheaper
  - Tried:
    - only for `DictionaryText`
    - only against `WindowCandidateKind::Oldest`
    - keep the closer current non-repeat candidate when the farther `oldest` gains less than `2` match bytes and costs at least `4` more offset-code bits
  - Why it looked plausible:
    - fresh retained current-vs-C archive inspection still shows the dictionary block over-sequenced and paying much larger offset payload than C
    - earlier blunt `oldest` penalties were too coarse, so an offset-gated version was the next narrow scoring check
  - What happened:
    - target moved the wrong way:
      - `dict_dictionary.bin`: `20,160 -> 20,161`
    - everything else stayed exact
  - Do not retry this current-vs-`oldest` displacement rule in the same offset-bit-gated form.

### Newly closed `Unknown` scoring variants

- large `Unknown` `RepeatNextPosition`-only repeat margin bonus
  - Motivation:
    - `decodecorpus_z000079` is still dominated by `repeat_next_position_selected_without_current_candidate`
    - the generic large-`Unknown` repeat margin had already helped, so the next narrow idea was to boost only the `ip+1` repeat family
  - Result:
    - `decodecorpus_z000079`: `7,321 -> 7,331`
    - `build_ruzstd-cli`: `855,679 -> 855,745`
  - Conclusion:
    - the remaining large-`Unknown` gap is not another local `RepeatNextPosition` repeat-bias problem
  - Do not retry this family in the same form.

- large `Unknown` same-start smaller-offset preference
  - Motivation:
    - `decodecorpus_z000079` is still a current-window `newest` / `oldest` fight with no long-hash or `second_newest`
    - a dictionary-style smaller-offset preference looked like the next plausible scoring tweak
  - Result:
    - exact byte-for-byte no-op
    - `decodecorpus_z000079` stayed `7,321`
  - Do not retry this family in the same form.

- tiny `Unknown` displacement (`<=4 KiB`)
  - Motivation:
    - the older all-size `Unknown` displacement experiment improved `decodecorpus_z000059` by `2` bytes, but hit global CPU
    - narrowing it to tiny blocks looked like a way to isolate the small-file win
  - Result:
    - `decodecorpus_z000059`: `711 -> 709`
    - but regressed:
      - `decodecorpus_z000031`: `112 -> 113`
      - `decodecorpus_z000053`: `304 -> 305`
    - and still drifted `json_logs_32m.jsonl` CPU the wrong way on the fast screen
  - Do not retry this size-only narrowing in the same form.

- large `Unknown` smaller-offset rescoring without the same-start restriction
  - Motivation:
    - fresh current-vs-C `z000079` histograms show the Rust path is still paying too much offset payload with too few sequences
    - a broader cheaper-offset preference looked like the first scoring rule that matched that evidence directly
  - Result:
    - exact byte-for-byte no-op
    - `decodecorpus_z000079` stayed `7,321`
  - Do not retry this rescoring family in the same form.

- large `Unknown` half-window
  - Motivation:
    - current Rust `z000079` uses much farther offsets than C in the first large compressed blocks
    - narrowing the effective search window looked like a structural way to force closer matches
  - Result:
    - `decodecorpus_z000079`: unchanged at `7,321`
    - `build_ruzstd-cli`: `855,679 -> 865,333`
  - Do not retry this blunt large-Unknown window-size cut in the same form.

- large `Unknown` `ip+1` newest-repeat-only
  - Motivation:
    - fresh matcher diagnostics still show `repeat_next_position_selected_without_current_candidate` dominating `z000079`
    - the oldest repeat-history slots were the most plausible stale candidates to cut first
  - Result:
    - hard regression:
      - `decodecorpus_z000079`: `7,321 -> 7,606`
      - `build_ruzstd-cli`: `855,679 -> 856,949`
  - Conclusion:
    - second and third repeat-history slots are still doing real work on `z000079`
  - Do not retry this family in the same form.

- large `Unknown` no backward match extension
  - Motivation:
    - fresh `z000079` archive evidence showed the Rust path is still materially under-sequenced with too few literals
    - disabling backward extension was the narrowest structural way to reduce match coalescing
  - Result:
    - `decodecorpus_z000079`: `7,321 -> 7,360`
    - `build_ruzstd-cli`: `855,679 -> 866,828`
    - `decodecorpus_z000033`: `532,632 -> 537,783`
  - Conclusion:
    - under-sequencing is real, but backward extension alone is not the culprit in a keepable form
  - Do not retry this blunt cut in the same form.

- large `Unknown` denser post-match insertion after `RepeatNextPosition`
  - Motivation:
    - live diagnostics still show `repeat_next_position_selected_without_current_candidate` dominating `z000079`
    - denser post-match suffix insertion was the narrowest way to give that dominant path more future candidate coverage
  - Variants tried:
    - dense limit `256`
    - fully dense post-match insertion
  - Result:
    - both were exact no-ops on `decodecorpus_z000079` (`7,321`)
    - only CPU drift/noise remained on the fast screen
  - Conclusion:
    - the dominant large-`Unknown` `RepeatNextPosition` path is not bottlenecked by sparse post-match suffix insertion
  - Do not retry this family in the same form.

- level-1 `Unknown` fixed `96 KiB` block size
  - Motivation:
    - `z000079` still differs from C in both block and parse shape
    - this was the first direct block-structure experiment on the `Unknown` path
  - Result:
    - `decodecorpus_z000079`: `7,321 -> 7,772`
    - `build_ruzstd-cli`: `855,679 -> 884,939`
    - repeated-text fixtures also regressed badly
  - Conclusion:
    - a smaller fixed block size is the wrong direction for this family
  - Do not retry a blunt 96 KiB `Unknown` cap in the same form.

### Retained

- `CompressionFileType::CodeText` -> dense short-line probing up to `64 KiB`
  - Why: after broadening the known-file-type corpus, `repo_compressed.rs` became the largest known-file-type loss, and archive inspection against C showed it was still matcher-side:
    - Rust: `literal_section_bytes=4658`, `sequence_payload_bytes=8161`, `sequences=3408`
    - C: `literal_section_bytes=4392`, `sequence_payload_bytes=8339`, `sequences=3576`
  - Broad-local result versus the retained `codeprobe10k` baseline:
    - `repo_compressed.rs`: `12,839 -> 12,695`
    - `repo_match_generator.rs`: `26,253 -> 26,192`
    - `decodecorpus_z000079`: unchanged at `7,321`
    - `dict_dictionary.bin`: unchanged at `20,160`
  - Current live broad-local summary vs C after retaining it:
    - better / worse / equal: `27 / 11 / 3`
    - bytes-above-C on losing fixtures: `185`
  - This is the retained larger-code-block `CodeText` starting point.

- `CompressionFileType::CodeText` -> short-line blocks up to `16 KiB` use a `5`-byte non-repeat floor
  - Why: fresh archive inspection on `repo_progress.rs` showed the remaining gap was still under-sequenced on the small short-line code path:
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
  - Broad-local result versus the retained `codeprobe64k` baseline:
    - `repo_progress.rs`: `3,147 -> 3,125`
    - `repo_benchmark_zstd.py`: `2,846 -> 2,814`
    - `repo_main.rs`: `2,128 -> 2,125`
    - unchanged:
      - `repo_compressed.rs`: `12,695`
      - `repo_match_generator.rs`: `26,192`
      - `decodecorpus_z000079`: `7,321`
      - `dict_dictionary.bin`: `20,160`
  - Current live broad-local summary vs C after retaining it:
    - better / worse / equal: `28 / 10 / 3`
    - bytes-above-C on losing fixtures: `162`
  - This is the retained smaller-code-block `CodeText` starting point.

- `CompressionFileType::ConfigText` -> compressed literal sections up to `1024` literals may force the single-stream Huffman path
  - Why: after the retained matcher-side work, the remaining `ConfigText` losses were no longer moving on small matcher rules, and `repo_Cargo.toml` still trailed C on a one-block compressed-literals shape.
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
    - unchanged:
      - `decodecorpus_z000079`: `7,321`
      - `dict_dictionary.bin`: `20,160`
      - `repo_.gitignore`: `172`
  - Current live broad-local summary vs C after retaining it:
    - better / worse / equal: `30 / 9 / 2`
    - bytes-above-C on losing fixtures: `154`
  - This is the retained small-literal `ConfigText` entropy starting point.

- `CompressionFileType::DictionaryText` -> exact Huffman table search for all literal sections
  - Why: archive inspection showed the dictionary fixture was paying too much sequence payload versus C and needed a literal-side correction, not a wider text search step.
  - Current result:
    - `dict_dictionary.bin`: `20,667`
    - broad-local bytes-above-C on worse fixtures stays at or below the retained non-file-type baseline.
  - This is the retained dictionary-specific starting point.

- `CompressionFileType::CodeText` / `CompressionFileType::ConfigText` -> exact Huffman table search for small literal sections
  - Why: small source/config residuals were still losing to C after the retained matcher work, and broadening exact Huffman search on the literal side improved them without touching JSON or binary paths.
  - Broad-local result versus the retained dense-smallbin baseline:
    - better / worse / equal vs C: `15 / 14 / 3 -> 16 / 13 / 3`
    - bytes-above-C on worse fixtures: `1,005 -> 984`
  - Main wins:
    - `dict_NetworkManager-dispatcher.service`: `395 -> 391`
    - `dict_fstrim.service`: `312 -> 308`
    - `dict_ftpd.service`: `172 -> 168`
    - `dict_netctl@.service`: `212 -> 206`
    - `repo_main.rs`: `2,141 -> 2,137`
    - `repo_match_generator.rs`: `22,591 -> 22,587`
  - No regressions on the broad-local suite.

- `CompressionFileType::CodeText` -> exact Huffman table search for all literal sections
  - Motivation:
    - after archive inspection, `repo_compressed.rs` looked like it might still have literal-side slack versus C
  - Result:
    - exact byte-for-byte no-op on the expanded broad-local suite
    - `repo_compressed.rs`: stayed `12,839`
    - `repo_progress.rs`: stayed `3,147`
    - `repo_benchmark_zstd.py`: stayed `2,846`
  - Conclusion:
    - this `CodeText` family is not waiting on a broader exact-Huffman literal search
  - Do not retry this literal-side expansion in the same form.

- `CompressionFileType::ConfigText` -> predefined LL/ML tables for compressed-literals blocks up to `64` sequences
  - Why: after the retained small-literal Huffman win, the remaining config/service losses still looked like tiny sequence-table overhead, especially `dict_kmod-static-nodes.service`.
  - Retained gate:
    - level 1 only
    - `ConfigText` only
    - compressed-literals blocks only
    - at most `64` sequences
  - Broad-local result versus the retained `Unknown predef64 compressed-literals` baseline:
    - better / worse / equal vs C: `16 / 13 / 3 -> 18 / 11 / 3`
    - bytes-above-C on losing fixtures: `192 -> 155`
  - Main wins:
    - `dict_kmod-static-nodes.service`: `497 -> 486`
    - `dict_NetworkManager-dispatcher.service`: `391 -> 381`
    - `dict_fstrim.service`: `308 -> 299`
    - `dict_systemd-coredump@.service`: `686 -> 682`
    - `dict_systemd-udev-settle.service`: `568 -> 560`
  - Guardrails stayed exact:
    - `decodecorpus_pack.bin`: `5,319,265`
    - `json_logs_32m.jsonl`: `690,084`
    - `decodecorpus_z000079`: `7,321`
    - `dict_dictionary.bin`: `20,160`
  - This is retained.

- `CompressionFileType::ConfigText` -> predefined OF tables on top of the retained compressed-literals LL/ML gate
  - Tried first at `<=64` sequences, then narrowed to `<=24`.
  - Why it looked plausible: `dict_kmod-static-nodes.service` still differed from C only by literals payload and OF table mode.
  - What happened:
    - `<=64`: no `kmod` improvement, plus regressions on other services:
      - `dict_systemd-coredump@.service`: `682 -> 688`
      - `dict_systemd-udev-settle.service`: `560 -> 562`
    - `<=24`: still no `kmod` improvement; only `dict_fstrim.service` moved `299 -> 298`
  - Do not retry this `ConfigText` OF predefined-table family in the same form.

- `CompressionFileType::ConfigText` -> same-start smaller-offset preference on small blocks
  - Tried:
    - `ConfigText`
    - up to `16 KiB`
    - same-start only
    - save at least 2 offset-code bits
    - lose at most 1 match byte
  - Result:
    - exact no-op on the expanded broad-local suite
    - `repo_Cargo.toml`: stayed `737`
    - `repo_.gitignore`: stayed `172`
  - Conclusion:
    - the remaining small `ConfigText` losses were not waiting on this dictionary-style matcher rule
  - Do not retry in the same form.

- `CompressionFileType::ConfigText` -> text repeat pipeline on small blocks
  - Tried:
    - `ConfigText`
    - up to `16 KiB`
  - Result:
    - byte no-op on the expanded broad-local suite
    - `repo_Cargo.toml`: stayed `737`
    - `repo_.gitignore`: stayed `172`
    - fast-screen CPU drifted slightly on already-winning binaries
  - Conclusion:
    - this family is not waiting on the text repeat parser branch either
  - Do not retry in the same form.

- `CompressionFileType::ConfigText` -> treat small blocks (`<=1024` bytes) as text for matching
  - Motivation:
    - `repo_.gitignore` and `dict_talk.service` sit below the generic `likely_text()` cutoff, so none of the retained text-side matcher logic runs for them today.
  - Result:
    - no gain on the target fixtures:
      - `repo_.gitignore`: stayed `172`
      - `dict_talk.service`: stayed `160`
      - `repo_Cargo.toml`: stayed `730`
    - one already-winning config fixture regressed slightly:
      - `dict_glustereventsd.service`: `285 -> 286`
  - Conclusion:
    - the remaining smallest `ConfigText` losses are not solved by forcing them onto the text matcher path
  - Do not retry in the same form.

- `CompressionFileType::ConfigText` -> prefer single-stream Huffman, but fall back to normal 4-stream when estimated smaller
  - Motivation:
    - `repo_Cargo.toml` still trailed C by `4` bytes
    - archive inspection showed C using a 4-stream literal section there while the retained Rust path used single-stream
  - Result:
    - exact byte-for-byte no-op on the retained `config-singlestream` baseline
    - `repo_Cargo.toml`: stayed `730`
    - `repo_.gitignore`: stayed `172`
    - `dict_talk.service`: stayed `160`
    - no broad-local fixture moved at all
  - Conclusion:
    - the remaining `ConfigText` tail is not fixed by letting the single-stream override fall back to 4-stream on current literal-size estimates
  - Do not retry this `ConfigText` literal stream-choice family in the same form.

- Huff0 weight-table description -> adaptively choose smaller of direct vs FSE weight encoding
  - Motivation:
    - after the retained `ConfigText` single-stream Huffman rule, `repo_.gitignore` was still `+8` vs C with the same parse and the same sequence payload
    - that made Huffman weight-table description overhead the next plausible literal-side target
  - Result:
    - exact byte-for-byte no-op on the retained `config-singlestream` baseline
    - `repo_.gitignore`: stayed `172`
    - `dict_talk.service`: stayed `160`
    - `repo_Cargo.toml`: stayed `730`
    - no broad-local fixture moved at all
  - Conclusion:
    - the remaining `.gitignore` gap is not explained by direct-vs-FSE Huffman weight-table selection
  - Do not retry this Huff0 table-description family in the same form.

- `CompressionFileType::Unknown` non-text -> extend the Fastest current-entry `second_newest` path up to `128 KiB`
  - Why: `decodecorpus_z000079` and a few neighboring `decodecorpus_z...` fixtures were still the main unknown-family residuals, and the earlier broad Fastest `second_newest` experiment was the only matcher direction that improved them.
  - Important implementation detail:
    - the first attempt widened sidecar allocation only and benchmarked as a no-op
    - the retained point required matching the probe gate in `should_track_second_newest_for_current_entry()`
  - Live broad-local result versus the live baseline:
    - better / worse / equal vs C stayed `16 / 13 / 3`
    - bytes-above-C on worse fixtures: `4,188 -> 4,166`
  - Main wins:
    - `decodecorpus_z000079`: `7,540 -> 7,518`
    - `decodecorpus_z000033`: `544,266 -> 532,424`
    - `decodecorpus_z000028`: `100,250 -> 98,656`
    - `decodecorpus_z000003`: `52,134 -> 51,006`
    - `build_ruzstd-cli`: `860,072 -> 856,479`
  - Guardrails stayed exact:
    - `decodecorpus_pack.bin`: `5,319,265`
    - `json_logs_32m.jsonl`: `690,084`
  - This is a retained live-tree improvement, but it sits on top of the currently drifted live baseline, not the older dictionary-retained reports.

- `CompressionFileType::Unknown` non-text -> disable long-repeat early-exit on large Fastest blocks
  - Why: `decodecorpus_z000079` remained the largest corrected-baseline loser, and diagnostics showed it was a repeat-heavy current-window case. The plausible remaining lever was allowing window candidates to keep competing after long repeat hits instead of exiting early.
  - Result on top of the corrected retained live baseline:
    - `decodecorpus_z000079`: `7,518 -> 7,344`
    - `build_ruzstd-cli`: `856,479 -> 855,908`
    - `decodecorpus_z000028`: `98,656 -> 98,592`
    - `decodecorpus_z000033`: `532,424 -> 532,333`
  - Broad-local summary vs C:
    - better / worse / equal stayed `16 / 13 / 3`
    - bytes-above-C on worse fixtures: `470 -> 296`
  - Guardrails stayed exact:
    - `decodecorpus_pack.bin`: `5,319,265`
    - `json_logs_32m.jsonl`: `690,084`
  - Important interpretation:
    - this improved `z000079` without making the archive more C-like
    - retained archive inspection shows even fewer literals and sequences than the prior retained point
    - so the win is from a better parse under our own compressed-block shape, not from converging on C’s shape

- `CompressionFileType::DictionaryText` -> broader smaller-offset preference without same-start
  - Tried:
    - smaller offset may win even without the same-start condition
    - save at least 4 offset-code bits
    - lose at most 1 match byte
  - Motivation:
    - retained archive inspection still shows `dict_dictionary.bin` paying much higher offset payload than C
  - Result:
    - `dict_dictionary.bin`: `20,160 -> 20,161`
  - Conclusion:
    - the retained dictionary same-start smaller-offset rule is already at the useful edge
  - Do not retry in the same form.

- `CompressionFileType::Unknown` -> small-literal exact Huffman search at level 1
  - Why: after the retained no-repeat early-exit win, the remaining `z000079` gap looked small enough that a literal-side entropy tweak might still matter.
  - Result:
    - `decodecorpus_z000079`: `7,344 -> 7,340`
    - broad-local bytes-above-C on losing fixtures: `296 -> 292`
  - Important interpretation:
    - archive inspection shows this is purely a literal-side gain:
      - `literal_section_bytes`: `823 -> 819`
      - `sequence_payload_bytes`: unchanged at `6480`
  - This is retained.

- `CompressionFileType::DictionaryText` -> remove the threshold-8 matcher override
  - Why: direct archive comparison against the checked-in retained binaries showed the live `DictionaryText` threshold-8 override had regressed badly.
  - Live regression shape before the rollback:
    - `dict_dictionary.bin`: `23,871`
    - `decoded_literals=22,688`
    - `sequences=1,974`
  - After removing the override:
    - `dict_dictionary.bin`: `20,667`
  - This rollback is retained.

- `CompressionFileType::DictionaryText` -> fully dense no-match probe step (`1`)
  - Why: after removing the threshold-8 override, the dictionary fixture was still the largest remaining broad-local loser.
  - Result:
    - `dict_dictionary.bin`: `20,667 -> 20,175`
    - C `zstd -1`: `20,145`
  - Broad-local summary on the corrected retained baseline:
    - better / worse / equal vs C: `16 / 13 / 3`
    - bytes-above-C on worse fixtures: `470`
  - Guardrails stayed exact:
    - `decodecorpus_pack.bin`: `5,319,265`
    - `json_logs_32m.jsonl`: `690,084`
  - This is the current retained dictionary-specific matcher starting point.

- `CompressionFileType::DictionaryText` -> same-start smaller-offset preference for non-repeat candidates
  - Why: after the retained dense probe step, `dict_dictionary.bin` still looked offset-side in both archive inspection and matcher diagnostics:
    - too many sequences and too many offset extra bits versus C
    - mostly a current-entry `newest` vs `oldest` fight
  - Retained rule:
    - only for `DictionaryText`
    - only for non-repeat candidates
    - only when the two candidates begin at the same `start_idx`
    - prefer the smaller offset when it saves at least 2 offset-code bits for at most a 1-byte match loss
  - Direct live-tree A/B result:
    - `dict_dictionary.bin`: `20,175 -> 20,162`
    - everything else in broad-local stayed byte-identical
    - fast-guardrail CPU stayed in the same band
  - Broad-local summary vs C:
    - better / worse / equal stayed `16 / 13 / 3`
    - bytes-above-C on losing fixtures: `230 -> 217`
  - This is retained.

- `CompressionFileType::DictionaryText` -> offset FSE max-log `8 -> 7`
  - Why: after the retained dictionary same-start smaller-offset rule, the remaining dictionary gap was only `17` bytes and looked narrow enough that a tiny entropy-side offset-table adjustment might still close a few more bytes.
  - Retained result:
    - `dict_dictionary.bin`: `20,162 -> 20,160`
    - every other broad-local fixture stayed byte-identical
    - fast guardrails stayed byte-identical
  - Broad-local summary vs C:
    - better / worse / equal stayed `16 / 13 / 3`
    - bytes-above-C on losing fixtures: `217 -> 215`
  - This is retained and bounds the useful point in this family.

- `CompressionFileType::DictionaryText` -> threshold `7`
  - Why it looked plausible: the current retained dictionary archive is still over-sequenced versus C, so a slightly stronger floor looked like the cleanest matcher-side correction.
  - What happened:
    - hard regression on the target:
      - `dict_dictionary.bin`: `20,175 -> 21,619`
  - Do not retry this threshold in the current retained dictionary-step-1 path.

- `CompressionFileType::DictionaryText` -> skip `oldest` entirely
  - Why it looked plausible: dictionary diagnostics showed the path is dominated by current-entry `newest` / `oldest`, and the archive still looked overmatched versus C.
  - What happened:
    - hard regression on the target:
      - `dict_dictionary.bin`: `20,175 -> 22,657`
  - Do not retry this blunt `oldest` removal in the current dictionary family.

- `CompressionFileType::DictionaryText` -> make `oldest` require a bigger plain match gain
  - Tried:
    - require `oldest` to beat the current non-repeat candidate by more than 1 byte
  - What happened:
    - target moved the wrong way:
      - `dict_dictionary.bin`: `20,175 -> 20,177`
  - Do not retry this blunt `oldest` penalty in the same form.

- `CompressionFileType::DictionaryText` -> same-start smaller-offset preference with up to 2 bytes of match loss
  - Why it looked plausible: the retained 1-byte-loss point finally moved the dictionary target, so the next obvious check was whether a slightly wider trade could close more of the gap.
  - What happened:
    - exact same outcome as the retained point:
      - `dict_dictionary.bin`: `20,175 -> 20,162`
    - nothing else moved
  - Do not keep this wider point as a separate setting. The narrower retained rule already captures the whole gain.

- `CompressionFileType::DictionaryText` -> smaller-offset preference with near-same coverage
  - Tried: widen the retained same-start rule so the smaller-offset candidate can also win when it starts no later and covers to within 1 byte of the farther match.
  - What happened:
    - exact byte-for-byte no-op on the retained baseline
    - `dict_dictionary.bin` stayed `20,162`
  - Do not retry this wider coverage rule in the same form. The retained same-start rule already captured the useful gain.

- `CompressionFileType::DictionaryText` -> offset FSE max-log `7 -> 6`
  - Why it looked plausible: the retained `8 -> 7` point did improve the target without touching anything else, so the next stronger point needed to be bounded explicitly.
  - What happened:
    - direct sequential `6` vs `7` A/B regressed only the target:
      - `dict_dictionary.bin`: `20,160 -> 20,432`
    - everything else stayed exact
  - Do not retry this stronger point. The family is bounded at `7`.

### Rejected

- `CompressionFileType::ConfigText` -> `offset_table_max_log = 7`
  - Why it looked plausible: both `DictionaryText` and `Unknown` retained real wins at `7`, and `ConfigText` still had small residual losses versus C.
  - What happened: exact no-op on the current live baseline.
  - Main screen:
    - `dict_kmod-static-nodes.service`: stayed `486`
    - `dict_fstrim.service`: stayed `299`
    - `dict_systemd-udev-settle.service`: stayed `560`
    - `dict_NetworkManager-dispatcher.service`: stayed `381`
  - Guardrails stayed exact too:
    - `build_ruzstd-cli`: `855,679`
    - `decodecorpus_z000079`: `7,321`
    - `dict_dictionary.bin`: `20,160`
    - `repo_main.rs`: `2,105`
  - Do not retry this exact file-type entropy move.

- `CompressionFileType::CodeText` / `CompressionFileType::ConfigText` -> exact Huffman table search for all literal sections
  - Why it looked plausible: the retained small-literal exact-Huffman change was a clean win, so the obvious follow-up was to broaden it further.
  - What happened: exact no-op versus the retained small-literal policy on both the focused and broad-local suites.
  - Do not retry this exact broadening; the useful signal is already exhausted at the current small-literal gate.

- `CompressionFileType::ConfigText` tiny short-line threshold `5 -> 4`
  - Tried: for short-line `ConfigText` blocks below `1 KiB`, lower the non-repeat floor from `5` to `4`.
  - What happened:
    - exact byte-for-byte no-op on the retained baseline
    - none of the remaining service-file residuals moved
  - Do not retry this tiny-config threshold cut in the same form.

- `CompressionFileType::ArchiveLike` -> dense non-text probing across block sizes
  - Why it looked plausible: earlier broad binary experiments showed real wins on build artifacts and some archive-like binaries.
  - What happened: it improved `build_libruzstd.rlib` from `611,155 -> 600,329`, but repeat CPU moved `0.03s -> 0.04s`.
  - Broad-local compression gap vs C did not improve at all:
    - better / worse / equal stayed `15 / 14 / 3`
    - bytes-above-C on worse fixtures stayed `1,005`
  - Do not retain this as a level-1 starting point in the same form.

- `CompressionFileType::DictionaryText` -> wider text no-match probe step
  - Why it looked plausible: archive inspection showed `dict_dictionary.bin` was overmatching versus C, with too many sequences and too few literals.
  - What happened: it made the target file worse:
    - `dict_dictionary.bin`: `20,667 -> 21,302`
  - Broad-local bytes-above-C on worse fixtures got worse:
    - `1,005 -> 1,640`
  - Do not retry this exact dictionary-specific probe-step widening.

- `CompressionFileType::DictionaryText` -> threshold-8 matcher override
  - This was previously treated as promising, but the live source-tree reconciliation disproved it.
  - Failure mode:
    - `dict_dictionary.bin`: `20,667 -> 23,871`
    - archive shape shifted to far too many literals and too few sequences
  - Do not treat this as retained; the rollback is the retained point.

- `CompressionFileType::Unknown` non-text -> non-repeat floor `5 -> 6`
  - Why it looked plausible: `decodecorpus_z000079` looked overmatched versus C, with too few literals and too few sequences.
  - What happened: it made the whole `decodecorpus_z...` family worse.
  - Main regressions:
    - `decodecorpus_z000079`: `7,540 -> 7,556`
    - `decodecorpus_z000033`: `544,266 -> 559,261`
    - `build_ruzstd-cli`: `860,072 -> 866,219`
  - Do not retry this family-wide Unknown-binary threshold raise.

- `CompressionFileType::Unknown` non-text -> dense long-match index limit `128 -> 64`
  - Why it looked plausible: the remaining unknown-family loser still looked overindexed and overmatched versus C.
  - What happened: the target got slightly worse and the rest of the family only moved by noise.
  - Main regression:
    - `decodecorpus_z000079`: `7,540 -> 7,548`
  - Do not retry this exact Unknown-family dense-limit cut.

- `CompressionFileType::Unknown` non-text -> dense probe step up to `128 KiB`
  - Why it looked plausible: the remaining main unknown-family loser, `decodecorpus_z000079`, lives on `128 KiB` blocks and was still worse than C after the retained `second_newest` extension.
  - What happened:
    - some neighboring `decodecorpus_z...` fixtures improved
    - but the target got worse:
      - `decodecorpus_z000079`: `7,518 -> 7,531`
    - broad-local bytes-above-C on worse fixtures worsened:
      - `470 -> 483`
  - Do not retry this exact dense-probe extension.

- `CompressionFileType::Unknown` non-text -> let `second_newest` compete against a weak current min-length non-repeat match
  - Why it looked plausible: the older broad `second_newest` experiments had real compression signal, and the corrected `Unknown`-family path had only ever been allowed to run on “no current candidate” cases.
  - What happened:
    - exact byte-for-byte no-op on the corrected retained baseline
    - slight fast-fixture CPU drift the wrong way
  - Do not retry this exact weak-current widening; it is exhausted.

- `CompressionFileType::Unknown` non-text -> complementary end-of-match insertion on large Fastest blocks
  - Why it looked plausible: `z000079` was still under-sequenced versus C, so preserving one more post-match start looked like the cleanest way to create extra follow-up opportunities without changing framing.
  - What happened:
    - `decodecorpus_z000079`: `7,518 -> 7,541`
    - `build_ruzstd-cli`: `856,479 -> 856,462`
    - fast CPU drifted the wrong way
  - Do not retry this exact complementary-end variant for the unknown-family path.

- `CompressionFileType::Unknown` non-text -> reduce repeat-match advantage from `2` bytes to `1` on large Fastest blocks
  - Why it looked plausible: `z000079` still looked overmatched versus C, and repeat-favoring bias was a plausible reason window candidates were losing too often.
  - What happened:
    - `decodecorpus_z000079`: unchanged at `7,518`
    - a few already-winning unknown-family fixtures improved slightly
    - fast CPU regressed
  - Do not retry this exact repeat-margin cut. It does not address the actual `z000079` loss mechanism.

- `CompressionFileType::Unknown` non-text -> prefer smaller-offset non-repeat matches over slightly longer ones
  - Why it looked plausible: `z000079` still had much larger offsets and many fewer sequences than C, so a narrow smaller-offset bias looked like a way to reduce far-match preference without reopening the broader matcher.
  - What happened:
    - `decodecorpus_z000079`: unchanged at `7,344`
    - some already-winning fixtures improved slightly
    - fast CPU got worse
  - Do not retry this exact small-offset bias. It does not move the real remaining loser.

- `CompressionFileType::Unknown` non-text -> equal-length smaller-offset tie-break on the large Fastest path
  - Tried: when two non-repeat window candidates had equal match length, keep the closer current candidate if it saved meaningful offset-code bits.
  - What happened:
    - exact byte-for-byte no-op on broad-local
    - `decodecorpus_z000079` stayed `7,321`
  - Do not retry this equal-length tie-break in the same representation.

- `CompressionFileType::Unknown` non-text -> conditionally stronger `newest` displacement on large Fastest blocks
  - Tried: keep the retained `newest +2` rule normally, but require `+3` when the farther `newest` candidate also cost at least 4 more offset-code bits.
  - What happened:
    - `decodecorpus_z000079` stayed `7,321`
    - `build_ruzstd-cli`: `855,679 -> 855,725`
  - Do not retry this `newest` bits-gap threshold in the same form.

- `CompressionFileType::Unknown` non-text -> remove all repeat early-exit, including block-end exits
  - Why it looked plausible: the retained length-only no-repeat early-exit disable was a clean win, so the obvious follow-up was removing the last repeat early-exit too.
  - What happened:
    - exact byte-for-byte no-op versus the retained point
    - fast CPU drifted the wrong way
  - Do not retry this exact full repeat-early-exit removal. The useful win in this family is already exhausted at the length-only cut.

- `CompressionFileType::Unknown` non-text -> whole-vs-midpoint split candidate on large Fastest blocks
  - Why it looked plausible: `decodecorpus_z000079` still remained the largest corrected-baseline loser, and archive inspection had already shown that some unknown-family winners benefited from better compressed-block partitioning.
  - What happened:
    - both the estimate-gated and exact whole-vs-midpoint variants produced the same result
    - `decodecorpus_z000079` stayed unchanged at `7,344`
    - broad-local bytes-above-C on losing fixtures stayed `296`
    - some already-winning unknown-family fixtures improved, but fast CPU drifted the wrong way
  - Do not retry this split family in the same form. The remaining `z000079` gap is not waiting on a simple midpoint split choice.

- `CompressionFileType::Unknown` non-text -> extend small-block `second_newest` to all Unknown block sizes
  - Tried: keep the retained no-candidate `second_newest` path, but widen it from small `Unknown` blocks to all `Unknown` non-text blocks.
  - Why it looked plausible: this revisited the old large-block second-newest family under the current file-type split, so it no longer touched `.bin` guardrails like `decodecorpus_pack.bin`.
  - What happened:
    - exact byte-for-byte no-op on the retained `unknown-smallhuff` baseline
    - fast CPU still drifted the wrong way
  - Do not retry this widened Unknown-only second-newest path in the same representation.

- `CompressionFileType::Unknown` non-text -> stop after a solid current-entry non-repeat candidate
  - Tried: on large Fastest `Unknown` blocks, if the current entry already yields a 16-byte non-repeat candidate, stop before walking older entries.
  - Why it looked plausible: archive histograms showed the remaining `z000079` gap is dominated by offset-bit cost, so cutting older, farther matches looked like the narrowest structural fix.
  - What happened:
    - direct regression on the target:
      - `decodecorpus_z000079`: `7,340 -> 7,524`
  - Do not retry this current-entry cutoff in the same form.

- `CompressionFileType::Unknown` non-text -> offset-aware same-start non-repeat comparison
  - Tried: for large Fastest `Unknown` blocks, prefer a smaller-offset non-repeat candidate at the same start position when it saved at least 4 offset-code bits for at most a 2-byte match loss.
  - Why it looked plausible: archive histograms showed `z000079` is still paying heavily in offset codes.
  - What happened:
    - exact byte-for-byte no-op on the retained baseline
    - fast CPU still drifted the wrong way
  - Do not retry this local offset-aware comparison in the same form.

- `CompressionFileType::Unknown` non-text -> keep closer current candidate over farther `newest` unless `newest` gains 2 bytes
  - Why: after bounding the `oldest` displacement family, the remaining large-`Unknown` current-window scoring gap still had the symmetric `newest` path open.
  - Retained rule:
    - only for large Fastest `Unknown` non-text blocks
    - only when a farther `newest` candidate tries to displace an already-found closer non-repeat candidate
    - require at least a 2-byte match gain
  - Result:
    - `build_ruzstd-cli`: `855,822 -> 855,679`
    - `decodecorpus_z000033`: `532,650 -> 532,632`
    - `decodecorpus_z000079`: stayed `7,321`
    - fast guardrails stayed in the same CPU band
    - broad-local gap versus C stayed `210` bytes-above-C on losers
  - This is retained as a small safe broad-family improvement.

- `CompressionFileType::Unknown` non-text -> keep closer current candidate over farther `newest` unless `newest` gains 3 bytes
  - Why it looked plausible: the retained `newest +2` rule improved already-winning unknown-family fixtures without regressions, so the stronger point needed to be bounded.
  - What happened:
    - gave back the broad-family win:
      - `build_ruzstd-cli`: `855,679 -> 855,915`
    - did nothing for the target:
      - `decodecorpus_z000079`: stayed `7,321`
  - Do not retry this stronger `newest` displacement rule. The family is bounded at `+2`.

- `CompressionFileType::Unknown` non-text -> extend retained `oldest +2` and `newest +2` displacement rules to all Unknown block sizes
  - Tried:
    - keep the retained large-Unknown displacement rules
    - remove the `128 KiB` size gate so they apply to all Fastest `Unknown` non-text blocks
  - Why it looked plausible:
    - the remaining smaller Unknown losers (`decodecorpus_z000053`, `decodecorpus_z000059`) show the same `newest` / `oldest` current-window shape as `z000079`, just without the large-block gate
  - What happened:
    - small Unknown fixtures did improve:
      - `decodecorpus_z000059`: `711 -> 709`
      - `decodecorpus_z000054`: `9,567 -> 9,565`
      - `decodecorpus_z000080`: `2,603 -> 2,602`
      - `decodecorpus_z000003`: `51,012 -> 51,001`
    - but the total gap improvement was too small:
      - broad-local bytes-above-C on losing fixtures: `210 -> 208`
    - and fast guardrails drifted the wrong way:
      - `decodecorpus_pack.bin` CPU: `0.22s -> 0.23s`
      - `json_logs_32m.jsonl` CPU: `0.16s -> 0.17s`
  - Do not extend the displacement rules below the large-Unknown path in the current matcher shape.

- `CompressionFileType::Unknown` non-text -> keep closer current candidate over farther `oldest` unless `oldest` gains 2 bytes
  - Why: the remaining `z000079` loss still looked like too many farther current-window wins for small length gain, especially on the large `Unknown` Fastest path.
  - Retained rule:
    - only for large Fastest `Unknown` non-text blocks
    - only when an `oldest` candidate tries to displace an already-found closer non-repeat candidate
    - require at least a 2-byte match gain
  - Result:
    - `decodecorpus_z000079`: `7,326 -> 7,324`
    - `decodecorpus_z000028`: `98,567 -> 98,388`
    - `decodecorpus_z000033`: `532,592 -> 532,546`
    - `build_ruzstd-cli`: `856,110 -> 855,782`
    - fast guardrails stayed byte-identical on the main fixtures
  - Broad-local summary vs C:
    - better / worse / equal stayed `16 / 13 / 3`
    - bytes-above-C on losing fixtures: `215 -> 213`
  - This is retained.

- `CompressionFileType::Unknown` non-text -> keep closer current candidate over farther `oldest` unless `oldest` gains 3 bytes
  - Why it looked plausible: the retained `+2` oldest-displacement rule was the first current-window scoring change that moved `z000079`, so the next stronger point needed to be bounded.
  - What happened:
    - target regressed:
      - `decodecorpus_z000079`: `7,324 -> 7,335`
    - already-winning fixtures also moved the wrong way:
      - `build_ruzstd-cli`: `855,782 -> 856,067`
      - `decodecorpus_z000033`: `532,546 -> 532,577`
    - fast-guardrail CPU also drifted slightly worse
  - Do not retry this stronger point. The family is bounded at `+2`.

- `CompressionFileType::Unknown` non-text -> keep closer current candidate unless farther `oldest` gains 3 bytes only when it also costs 4 more offset-code bits
  - Why it looked plausible: the plain `+3` point was too broad, so the next sensible bound was to strengthen the retained `+2` rule only when the farther candidate also had a materially larger offset-code cost.
  - What happened:
    - target regressed:
      - `decodecorpus_z000079`: `7,324 -> 7,328`
    - part of the retained unknown-family win also regressed:
      - `build_ruzstd-cli`: `855,782 -> 855,829`
    - CPU stayed flat, but the compression direction was wrong
  - Do not retry this selective stronger point. The retained `+2` rule is already near the useful edge.

- `CompressionFileType::Unknown` non-text -> keep retained `+2` inside current entry, but require `+3` for `oldest` from the previous entry
  - Why it looked plausible: live diagnostics on `z000079` showed `oldest` winners only in the current entry and the immediately previous entry, so entry-distance was the next structural split worth testing.
  - What happened:
    - `decodecorpus_z000079`: stayed `7,324`
    - `build_ruzstd-cli`: `855,782 -> 855,926`
    - CPU stayed flat
  - Do not retry this distance-only stronger point. It gives back existing wins without moving the target.

- `CompressionFileType::Unknown` non-text -> wider `ip+1` normal-window promotion after short current hits
  - Tried: for large Fastest `Unknown` blocks, widen the next-position normal-window promotion from exact-minimum current non-repeat hits to short current non-repeat hits up to `8` bytes.
  - Why it looked plausible: this is the closest remaining safe-Rust analogue to C `double_fast` checking a long match at `ip+1` after a short current hit.
  - What happened:
    - exact byte-for-byte no-op on both broad-local and the fast guardrails
    - `decodecorpus_z000079` stayed `7,326`
  - Do not retry this wider `ip+1` normal-window probe in the same representation.

- `CompressionFileType::Unknown` small blocks -> same-start smaller-offset preference
  - Tried: reuse the retained dictionary same-start smaller-offset rule on non-text `Unknown` blocks at or below `128 KiB`.
  - Why it looked plausible: the remaining small `decodecorpus_z...` residuals are also offset-side, so a narrower non-repeat offset-cost preference looked like a possible bridge.
  - What happened:
    - hard regression across the family:
      - `decodecorpus_z000079`: `7,326 -> 7,946`
      - `build_ruzstd-cli`: `856,110 -> 878,123`
      - `decodecorpus_z000059`: `711 -> 747`
  - Do not retry this “dictionary offset-aware rule for small Unknown blocks” family in the same form.

- `CompressionFileType::Unknown` -> offset FSE max-log `8 -> 7`
  - Tried: lower only the offset FSE table max-log for `Unknown` level-1 blocks from `8` to `7`.
  - Why it looked plausible: the remaining `z000079` gap is still dominated by offset-code payload, and this is one of the narrowest remaining entropy-side levers.
  - What happened:
    - on the older retained baseline it produced only tiny wins:
      - `decodecorpus_z000079`: `7,326 -> 7,325`
      - `decodecorpus_z000059`: `711 -> 709`
      - `build_ruzstd-cli`: `856,110 -> 855,482`
    - that older point was rejected because the gain looked too small to justify another file-type entropy knob
    - on the newer retained baseline with the large-`Unknown` oldest-displacement `+2` rule:
      - `decodecorpus_z000079`: `7,324 -> 7,321`
      - `build_ruzstd-cli`: `855,782 -> 855,822`
      - `decodecorpus_z000033`: `532,546 -> 532,650`
      - broad-local bytes-above-C on losing fixtures: `213 -> 210`
      - fast guardrails stayed in the same CPU band
  - This newer `Unknown oflog7` point is retained.

- `CompressionFileType::Unknown` -> offset FSE max-log `7 -> 6`
  - Why it looked plausible: after the retained `Unknown oflog7` point finally moved `z000079` enough to matter, the stronger point needed to be bounded.
  - What happened:
    - hard regression on the target:
      - `decodecorpus_z000079`: `7,321 -> 7,413`
    - broad regressions on already-winning unknown fixtures:
      - `build_ruzstd-cli`: `855,822 -> 860,261`
      - `decodecorpus_z000033`: `532,650 -> 536,771`
  - Do not retry `6`. The family is bounded at `7`.

- `CompressionFileType::Unknown` -> offset FSE max-log `6` only for small-sequence-count blocks
  - Tried:
    - keep retained `Unknown oflog7` normally
    - use `oflog6` only when the block has at most `1,536` sequences
  - Why it looked plausible:
    - retained archive inspection showed `decodecorpus_z000079` is a much smaller-sequence-count Unknown case than already-winning build-artifact blocks like `build_ruzstd-cli`
  - What happened:
    - target regressed hard:
      - `decodecorpus_z000079`: `7,321 -> 7,413`
    - several neighboring decodecorpus samples also regressed:
      - `decodecorpus_z000053`: `322 -> 324`
      - `decodecorpus_z000054`: `9,567 -> 9,589`
      - `decodecorpus_z000059`: `711 -> 714`
    - `build_ruzstd-cli` stayed flat, so there was no balancing family-wide gain
  - Do not retry sequence-count alone as the gate for a stronger `Unknown` offset entropy table.

- `CompressionFileType::Unknown` -> offset FSE max-log `6` only for tiny-sequence-count blocks
  - Tried:
    - keep retained `Unknown oflog7` normally
    - use `oflog6` only when the block has at most `256` sequences
  - Why it looked plausible:
    - archive inspection showed the smaller Unknown losers (`decodecorpus_z000053`, `decodecorpus_z000059`) are single-block cases with `36` and `246` sequences, far below `z000079`
  - What happened:
    - it regressed the exact targets:
      - `decodecorpus_z000053`: `322 -> 324`
      - `decodecorpus_z000059`: `711 -> 714`
    - and still nudged the larger target the wrong way:
      - `decodecorpus_z000079`: `7,321 -> 7,325`
  - Do not retry sequence-count alone as the gate for stronger `Unknown` offset entropy tables, even at much lower thresholds.

- `CompressionFileType::Unknown` -> predefined LL/ML tables up to 64 sequences, but only on compressed-literals blocks
  - Why: the smaller remaining Unknown losers (`decodecorpus_z000053`, `decodecorpus_z000059`) are single-block cases where sequence counts are already close to C, but sequence payload is still too large. On `z000053`, C was using predefined LL/ML while Rust was still encoding both tables.
  - Retained rule:
    - level 1 only
    - `CompressionFileType::Unknown`
    - block must be in the compressed-literals path
    - block must have at most `64` sequences
  - Result:
    - `decodecorpus_z000053`: `322 -> 304`
    - `decodecorpus_z000031`: stays `112` because it is a raw-literals case
    - broad-local bytes-above-C on losing fixtures: `210 -> 192`
    - fast guardrails stayed exact on bytes and in the same CPU band
  - This is retained.

- `CompressionFileType::Unknown` -> predefined LL/ML tables for compressed-literals blocks up to `256` sequences
  - Why it looked plausible: this can reach `decodecorpus_z000059` while still staying off the large `z000079` compressed-literals blocks.
  - What happened:
    - hard regression on the target small Unknown fixture:
      - `decodecorpus_z000059`: `711 -> 826`
    - `decodecorpus_z000079` stayed `7,321`
  - Do not retry this broader compressed-literals gate. The retained `64`-sequence point is bounded.

- `CompressionFileType::Unknown` -> predefined OF tables on top of the retained small compressed-literals LL/ML gate
  - Why it looked plausible: after the retained `z000053` win from matching C’s LL/ML table-mode shape, the natural follow-up was to try the same broadening for OF tables on the same small Unknown block family.
  - What happened:
    - it regressed the same small Unknown targets immediately:
      - `decodecorpus_z000053`: `304 -> 305`
      - `decodecorpus_z000059`: `711 -> 747`
  - Do not broaden the retained Unknown small-sequence predefined-table gate to OF tables. This family is LL/ML-specific.

- `CompressionFileType::Unknown` -> predefined LL/ML tables up to 64 sequences on all blocks
  - Why it looked plausible: it was the direct minimal version of the retained Unknown small-sequence LL/ML idea.
  - What happened:
    - it improved `decodecorpus_z000053`: `322 -> 304`
    - but regressed a tiny raw-literals case:
      - `decodecorpus_z000031`: `112 -> 114`
  - Do not use the unrestricted gate. Keep the compressed-literals condition.

- `CompressionFileType::Unknown` non-text -> narrower repeat-vs-normal margin bump (`2 -> 3`)
  - Tried: keep the retained large-`Unknown` repeat-bias family, but increase the repeat-vs-normal margin by only one byte instead of two.
  - Why it looked plausible: the stronger retained branch improved `z000079`, so the next obvious check was whether a narrower bump could keep the target gain with less collateral.
  - What happened:
    - `decodecorpus_z000079` stayed unchanged at `7,340`
    - broad-local bytes-above-C on losing fixtures stayed at `292`
    - already-winning binaries still regressed:
      - `build_ruzstd-cli`: `855,908 -> 856,018`
      - `decodecorpus_z000033`: `532,333 -> 532,439`
  - Do not retry this narrower repeat-bias bump. In this family the signal starts at the stronger `2 -> 5` margin.

- `CompressionFileType::Unknown` non-text -> too-strong repeat-vs-normal margin bump (`5 -> 6`)
  - Tried: after the retained large-`Unknown` repeat-bias win, push the same family one step farther.
  - Why it looked plausible: the retained `2 -> 5` point still improved `z000079`, so it was worth checking whether one more step in the same direction still paid off.
  - What happened:
    - `decodecorpus_z000079`: `7,329 -> 7,333`
    - broad-local bytes-above-C on losing fixtures worsened `281 -> 285`
    - already-winning binaries also regressed more:
      - `build_ruzstd-cli`: `855,996 -> 856,134`
      - `decodecorpus_z000033`: `532,528 -> 532,616`
  - Do not retry this stronger repeat-bias bump. The family is bounded at the retained `2 -> 5` point.

## Level 1 Text Path

### Retained

- `TEXT_MIN_NON_REPEAT_MATCH_LEN: 10 -> 8`
  - Why: first broad-local win that materially reduced the level-1 text gap without disturbing main JSON.
  - Key result: broad-local bytes-above-C on worse fixtures `9,279 -> 6,578`.

- Short-line text threshold `8 -> 7`
  - Why: closes the remaining source-text gap without touching long-line JSON.
  - Current retained path:
    - `7` on short-line text
    - `8` on long-line text
  - Key result: broad-local bytes-above-C on worse fixtures `4,908 -> 4,059`.

- Short-line text probe step `3 -> 2`
  - Why: denser search helps the remaining short-line text and dictionary fixtures once JSON is excluded by the short-line gate.
  - Key result: broad-local bytes-above-C on worse fixtures `4,059 -> 3,408`.

- Short-line text threshold `7 -> 6`
  - Why: further improves the remaining dictionary/config-text gap while preserving the main fixture guardrails.
  - Key result: broad-local bytes-above-C on worse fixtures `3,408 -> 2,411`.
  - Tradeoff: `repo_match_generator.rs` gives back a little versus the previous retained point, but still remains smaller than C.

- Short-line text threshold `6 -> 5`
  - Why: further reduces the largest remaining dictionary-style gap.
  - Key result: broad-local bytes-above-C on worse fixtures `2,411 -> 2,196`.
  - Tradeoff: `repo_match_generator.rs` crosses back to slightly worse than C, so this is a more compression-total-oriented retained point, not a clean win on every source-text subcase.

- Code-like short-line text split
  - Why: recover the source-text regression without giving back the dictionary/config-text wins.
  - Current retained split:
    - code-like short-line text: threshold `6`
    - other short-line text: threshold `5`
    - long-line text: threshold `8`
  - Key result: broad-local bytes-above-C on worse fixtures `2,196 -> 1,909`.

- Tiny `CodeText` / `ConfigText` dense short-line probing
  - Why: the remaining small text-side losses had separated cleanly from the binary holdouts, so a tiny file-only probe-density bump could target them without touching JSON or the retained binary path.
  - Current retained shape:
    - `CompressionFileType::CodeText` or `CompressionFileType::ConfigText`
    - short-line text only
    - block size up to `8 KiB`
    - no-match probe step forced to `1`
  - Key result:
    - `repo_main.rs`: `2,137 -> 2,105`
    - `dict_systemd-logind.service`: `1,134 -> 1,122`
    - `dict_systemd-coredump@.service`: `690 -> 686`
    - broad-local bytes-above-C on worse fixtures `278 -> 230`
  - Tradeoff:
    - no broad-local binary regression; the main remaining binary losers stayed exact

- Wider `CodeText` dense short-line probing
  - Why: after expanding the broad-local suite, two new `CodeText` losers (`repo_progress.rs` and `repo_benchmark_zstd.py`) landed just above the retained `8 KiB` dense-probe cutoff.
  - Current retained shape:
    - `CompressionFileType::CodeText` only
    - short-line text only
    - block size up to `10 KiB`
    - `ConfigText` remains at the older `8 KiB` cutoff
  - Key result on the expanded broad-local suite:
    - `repo_progress.rs`: `3,168 -> 3,147`
    - `repo_benchmark_zstd.py`: `2,865 -> 2,846`
    - bytes-above-C on losing fixtures: `312 -> 272`
  - Guardrails stayed exact:
    - `build_ruzstd-cli`: `866,649`
    - `decodecorpus_z000079`: `7,321`
    - `dict_dictionary.bin`: `20,160`

### Rejected

- Split short-line probe step by code-like vs config-like text
  - Tried: denser short-line probe step only for non-code short-line text, while code-like short-line text kept the wider text step.
  - Failure mode: preserved the dictionary/config wins, but gave back the recovered code-file wins.
  - Broad-local result moved the wrong way:
    - bytes-above-C on worse fixtures `1,909 -> 2,035`
  - Main regressions:
    - `repo_match_generator.rs`: `22,591 -> 22,883`
    - `repo_main.rs`: `2,141 -> 2,181`
  - Do not retry this split in the same form.

- Tiny code-like short-line threshold `6 -> 5`
  - Tried: lower the code-like short-line non-repeat floor only for code files up to `8 KiB`.
  - Why it looked plausible: `repo_main.rs` is still a small residual loser while `repo_match_generator.rs` is already on the winning side.
  - What happened:
    - `repo_main.rs`: `2,137 -> 2,136`
    - everything else stayed exact
  - Do not keep this split in the same form. A 1-byte win is not enough to justify another code-path branch.

### Rejected

- Global text threshold `8 -> 7`
  - Failure mode: catastrophic main JSON regression.
  - Main result: `json_logs_32m.jsonl 690,084 -> 809,823`.
  - Do not retry globally.

- Global text probe step `3 -> 2`
  - Failure mode: helps small text, but regresses main JSON and decodecorpus CPU.
  - Main result: `json_logs_32m.jsonl 690,084 -> 713,323`.
  - Do not retry globally.

- Text classifier minimum `1024 -> 512`
  - Failure mode: effectively noise, slightly worsened broad-local gap.
  - Broad-local bytes-above-C on worse fixtures: `6,578 -> 6,580`.
  - Do not retry as a broad range change.

- Size-gated threshold `8 -> 7` under `256 KiB`
  - Failure mode: still hits every `128 KiB` JSON block, so it reproduces the global JSON failure.
  - Main result: `json_logs_32m.jsonl 690,084 -> 809,823`.
  - Do not use a size cutoff above the streaming block size to protect JSON.

- Size-gated threshold `8 -> 7` under `64 KiB`
  - Failure mode: safe, but too narrow to reach the main remaining large source-text loser.
  - What it missed: `repo_match_generator.rs` was still worse than C.
  - Replaced by the short-line gate.

## Level 1 Binary Path

### Retained

- Fastest non-text `ip+1` repeat lookahead
  - Why: first retained level-1 binary-side change that materially improved both the original main binary guardrail and the broader local suite without disturbing JSON.
  - Main result:
    - `decodecorpus_pack.bin`: `5,323,478 -> 5,319,265`
    - `json_logs_32m.jsonl`: unchanged at `690,084`
  - Broad-local result:
    - better / worse / equal vs C stayed `14 / 15 / 3`
    - bytes-above-C on worse fixtures improved `1,909 -> 1,073`
  - Tradeoff:
    - `decodecorpus_pack.bin` CPU drifted from `0.18s/0.19s` into the `0.20s` band on repeat.

- Fastest small-block current-entry `second_newest` probe
  - Why: a narrow same-block binary search improvement that leaves the large-block level-1 guardrails unchanged while reducing the broad-local residual compression gap.
  - Current retained shape:
    - non-text blocks only
    - block size up to `64 KiB`
    - probe `second_newest` only when the current position has no candidate
  - Main result:
    - `decodecorpus_pack.bin`: unchanged at `5,319,265`
    - `json_logs_32m.jsonl`: unchanged at `690,084`
  - Broad-local result:
    - better / worse / equal vs C stayed `14 / 15 / 3`
    - bytes-above-C on worse fixtures improved `1,073 -> 1,022`
  - Tradeoff:
    - main repeat CPU stayed in the same band, with `decodecorpus_pack.bin` moving `0.20s -> 0.21s`

- Fastest small-block dense no-match probing
  - Why: a narrow binary-path search-density increase that improves the small-block residual gap without changing the main level-1 guardrails.
  - Current retained shape:
    - non-text blocks only
    - block size up to `64 KiB`
    - no-match probe step forced to `1`
    - current-candidate paths unchanged
  - Main result:
    - `decodecorpus_pack.bin`: unchanged at `5,319,265`
    - `json_logs_32m.jsonl`: unchanged at `690,084`
  - Broad-local result:
    - better / worse / equal vs C improved `14 / 15 / 3 -> 15 / 14 / 3`
    - bytes-above-C on worse fixtures improved `1,022 -> 1,005`
  - Tradeoff:
    - main repeat CPU stayed in the same band, with `decodecorpus_pack.bin` moving `0.22s -> 0.21s`
    - it still does not move `dict_dictionary.bin` or `decodecorpus_z000079`

- Large `Unknown` Fastest blocks use a stronger repeat-vs-normal margin
  - Why: archive diagnostics on `decodecorpus_z000079` showed the remaining gap was dominated by offset-bit cost, and the retained path was still underusing cheap repeat-style matches.
  - Current retained shape:
    - `CompressionFileType::Unknown` only
    - non-text only
    - large Fastest blocks only
    - repeat-vs-normal match margin widened from `2` to `5`
  - Main result:
    - `decodecorpus_z000079`: `7,340 -> 7,326`
    - `decodecorpus_pack.bin`: unchanged at `5,319,265`
    - `json_logs_32m.jsonl`: unchanged at `690,084`
  - Broad-local result:
    - better / worse / equal vs C stayed `16 / 13 / 3`
    - bytes-above-C on worse fixtures improved `292 -> 278`
  - Tradeoff:
    - already-winning binaries gave back a little:
      - `build_ruzstd-cli`: `855,908 -> 856,110`
      - `decodecorpus_z000033`: `532,333 -> 532,592`

### Rejected

- Fastest non-text oldest-first window probing
  - Tried: keep the retained short-line text path unchanged, but probe window candidates oldest-first for non-text blocks at `CompressionLevel::Fastest`.
  - Failure mode: byte-identical across the broad-local suite, and the only apparent CPU wins disappeared on repeat.
  - Focused repeat result:
    - `build_ruzstd-cli`: `0.04s -> 0.04s`
    - `decodecorpus_z000033`: `0.02s -> 0.02s`
  - Do not retry this probe-order change in the same form.

- Fastest non-text next-repeat zero-literal gate
  - Tried: keep the retained `ip+1` repeat lookahead, but skip it at zero-literal positions.
  - Failure mode: final zero-literal `RepeatNextPosition` wins were zero in diagnostics, but the gate still gave back real compression.
  - Main regressions:
    - `decodecorpus_pack.bin`: `5,319,265 -> 5,321,178`
  - Broad-local regressions:
    - `build_libruzstd.rlib`: `611,155 -> 618,065`
    - `build_ruzstd-cli`: `860,072 -> 866,916`
    - `decodecorpus_z000079`: `7,540 -> 8,376`
  - Do not retry a gate based only on final zero-literal `RepeatNextPosition` counts.

- Fastest non-text `ip+1` window lookahead
  - Tried: enable the existing next-position window helper for `CompressionLevel::Fastest` non-text blocks on top of the retained `ip+1` repeat lookahead.
  - Failure mode: complete no-op on both the main screen and broad-local suite.
  - Do not retry this helper enable in the same form.

- Dedicated Fastest non-text parser loop split
  - Tried: route Fastest non-text blocks through a dedicated parser loop preserving the retained `ip+1` repeat behavior while removing shared-loop feature branches.
  - Failure mode: byte-identical, but no CPU improvement; broad-local even nudged `build_ruzstd-cli` from `0.04s` to `0.05s`.
  - Do not retry the same split without a stronger representation change.

- Fastest current-entry second-newest candidate
  - Tried: track a current-entry second-newest candidate for large Fastest non-text blocks on top of the retained `ip+1` repeat lookahead.
  - Compression signal was real:
    - `decodecorpus_pack.bin`: `5,319,265 -> 5,259,216`
    - `build_libruzstd.rlib`: `611,155 -> 609,561`
    - `build_ruzstd-cli`: `860,072 -> 856,479`
  - Failure mode: CPU cost was too large:
    - `decodecorpus_pack.bin`: `0.21s -> 0.25s`
    - `build_libruzstd.rlib`: `0.03s -> 0.04s`
    - `build_ruzstd-cli`: `0.05s -> 0.06s`
  - Narrowing it to the “no candidate yet” case did not reduce the CPU cost at all.
  - Do not retry this exact current-entry second-candidate shape without a cheaper representation.

- Fastest current-entry long-hash candidate
  - Tried: enable the existing current-entry long-hash candidate machinery for large Fastest non-text blocks on top of the retained `ip+1` repeat lookahead.
  - Compression signal was real on the main binary guardrail:
    - `decodecorpus_pack.bin`: `5,319,265 -> 5,304,504`
  - But broad-local was mixed and CPU cost was still too large:
    - `decodecorpus_pack.bin`: `0.20s -> 0.24s`
    - `build_ruzstd-cli`: `0.04s -> 0.05s`
    - `build_libruzstd.rlib`: `611,155 -> 612,668`
    - `decodecorpus_z000079`: `7,540 -> 7,579`
  - Do not retry this exact long-hash enable without a narrower condition or a cheaper follow-up search shape.

- Fastest current-entry long-hash candidate only when no candidate exists
  - Tried: narrow the rejected Fastest current-entry long-hash family so the long-hash probe only runs when the current position has no candidate yet.
  - Main result improved further:
    - `decodecorpus_pack.bin`: `5,319,265 -> 5,301,816`
  - Failure mode: still too much CPU cost and broader regression:
    - `decodecorpus_pack.bin`: `0.20s -> 0.24s`
    - broad-local bytes-above-C on worse fixtures: `1,073 -> 1,147`
    - `build_libruzstd.rlib`: `611,155 -> 611,997`
    - `build_ruzstd-cli`: `860,072 -> 860,496`
    - `decodecorpus_z000079`: `7,540 -> 7,614`
  - Do not retry this no-candidate long-hash gate in the same representation.

- Fastest current-entry 4-byte hash only when no candidate exists
  - Tried: add a Fastest-only current-entry 4-byte hash for non-text blocks and only probe it when the current position has no candidate.
  - Main result improved strongly:
    - `decodecorpus_pack.bin`: `5,319,265 -> 5,282,033`
  - Failure mode: still too much CPU cost and broad-local did not improve overall:
    - `decodecorpus_pack.bin`: `0.20s -> 0.23s`
    - broad-local bytes-above-C on worse fixtures: `1,073 -> 1,098`
    - `decodecorpus_z000079`: `7,540 -> 7,565`
    - `dict_dictionary.bin`: unchanged at `20,667`
  - It produced real wins on build artifacts and some decodecorpus samples, but not enough to clear the broader promotion bar.

- Fastest repeat-table reuse threshold `64 -> 2048`
  - Tried: an entropy-side follow-up aimed at large multi-block level-1 inputs like `decodecorpus_z000079`.
  - Failure mode: exact no-op on bytes across both the focused and broad-local suites.
  - Do not retry this simple repeat-table threshold raise in the same form.

- Fastest trailing-RLE block split
  - Tried: split long trailing single-byte runs into separate RLE blocks when the suffix run is at least `32 KiB`.
  - Why it looked plausible: C `zstd -1` splits `decodecorpus_z000079` into two trailing RLE blocks while the retained Rust path does not.
  - Failure mode: hard regression on the target fixture:
    - `decodecorpus_z000079`: `7,540 -> 8,338`
  - Broad-local bytes-above-C on worse fixtures worsened:
    - `984 -> 1,782`
  - Do not retry this naive trailing-RLE split in the same form.

## CLI / Framing

### Rejected

- CLI frame-content-size single-segment hint
  - Tried: use the known source file size on the CLI path to emit a single-segment frame header.
  - Failure mode: broadly made outputs 1 to 3 bytes larger.
  - Examples:
    - `dict_dictionary.bin`: `20,667 -> 20,668`
    - `repo_main.rs`: `2,137 -> 2,138`
    - `build_ruzstd-cli`: `860,072 -> 860,075`
  - Do not retry this framing change in the same form.
  - Do not retry this exact 4-byte current-entry hash shape without a narrower condition or a different follow-up search rule.

- Fastest small-block oldest-first current-window probing
  - Tried: for Fastest non-text blocks up to `64 KiB`, and only when the current position had no candidate, prefer `oldest` before `newest` in current-window probing.
  - Failure mode: byte-identical on both main and broad-local, but CPU got worse.
  - Main result:
    - `decodecorpus_pack.bin`: unchanged at `5,319,265`, CPU `0.22s -> 0.27s`
    - `json_logs_32m.jsonl`: unchanged at `690,084`, CPU `0.17s -> 0.22s`
  - Broad-local result:
    - no byte deltas at all versus the retained small-block `second_newest` baseline
  - Do not retry this probe-order-only change in the same small-block no-candidate form.

- Fastest small-block `second_newest` on weak current non-repeat matches
  - Tried: keep the retained small-block `second_newest` sidecar and block-size gate, but also probe it when the current candidate is a weak minimum-length non-repeat match instead of only when no candidate exists.
  - Failure mode: complete no-op.
  - Main result:
    - `decodecorpus_pack.bin`: unchanged at `5,319,265`
    - `json_logs_32m.jsonl`: unchanged at `690,084`
  - Broad-local result:
    - better / worse / equal vs C stayed `15 / 14 / 3`
    - bytes-above-C on worse fixtures stayed `1,005`
  - Do not retry this widening of the small-block `second_newest` condition in the same representation.

- Dense probing for all compressible Fastest non-text blocks
  - Tried: broaden the retained dense-probe family from “non-text blocks up to `64 KiB`” to “all compressible non-text blocks”, still excluding text and xorshift-like incompressible blocks.
  - Compression signal was strong:
    - `decodecorpus_pack.bin`: `5,319,265 -> 5,219,513`
    - `build_libruzstd.rlib`: `611,155 -> 600,329`
    - `build_ruzstd-cli`: `860,072 -> 846,556`
    - `decodecorpus_z000033`: `544,266 -> 533,010`
  - Failure mode:
    - main CPU regressed too hard:
      - `decodecorpus_pack.bin`: `0.21s -> 0.28s`
    - broad-local total moved the wrong way because one stubborn loser regressed:
      - `decodecorpus_z000079`: `7,540 -> 7,565`
      - bytes-above-C on worse fixtures: `1,005 -> 1,030`
  - Do not retry this dense-probe family as a simple “compressible non-text everywhere” gate.

- C-shaped Fastest repeat probe at `ip + step`
  - Tried: follow `ZSTD_fast` more literally by probing repeat at `ip + step` for Fastest non-text blocks when the current position has no candidate.
  - Failure mode: immediate main guardrail regression.
  - Main result:
    - `decodecorpus_pack.bin`: `5,319,265 -> 5,330,232`
    - CPU: `0.22s -> 0.26s`
  - Do not retry this exact `ip + step` repeat analogue in the current matcher representation.

## Level 4 / Best Binary Path

### Retained

- Current-entry long-hash candidate for binary `Best`
  - Why: strongest retained binary-side compression improvement on decodecorpus-style inputs.

- Skip redundant current-entry short scan after a long-hash hit
  - Why: real decodecorpus CPU recovery for a small byte trade.

- Older-entry skip threshold after long-hash hit
  - Current retained threshold line:
    - keep only the retained current point from history/workplan
  - Why: incremental CPU recovery with limited byte loss.

- Distant-`newest` prune after current long-hash hit
  - Why: first broader retained older-entry search-shape win after the long-hash hit.

### Rejected

- Distance-based `oldest` pruning (`>= 3`, `>= 4`, and conditional variants)
  - Failure mode: consistently gives back bytes, even when diagnostics suggest those buckets look inactive.
  - Do not retry as a pure entry-distance cut.

- Wider `ip+1` promotion after short current hit
  - Failure mode: can improve decodecorpus bytes, but costs too much CPU in the current matcher representation.
  - Do not retry as another threshold tweak on the same helper path.

- Long-hash zero-literal-only runtime gate
  - Failure mode: gives back bytes without a reliable CPU win.

- Long-hash-only `ip+1` promotion
  - Failure mode: too narrow; older-entry competition still matters.

## Working Rule

Before trying a new threshold, gate, or pruning cut:
1. check whether the family already appears here
2. if it does, change the representation or conditioning signal, not just the numeric cutoff
3. add the new result here once benchmarked

## Latest lockfile reject

- lockfile-like `DictionaryText` -> disable repeat block-end early-exit
  - Why it looked plausible:
    - if repeat block-end early-exit was suppressing a later current-window winner, removing it could have improved the remaining `Cargo.lock` offset-side gap
  - Result:
    - focused size was an exact no-op:
      - `repo_Cargo.lock`: stayed `9,114`
    - matcher diagnostics moved slightly, but output bytes did not
  - Do not retry this repeat block-end early-exit branch in the current lockfile parser shape.

## Latest file-type surface update

- Added named-file coverage for more lockfiles:
  - `yarn.lock`
  - `poetry.lock`
  - `pipfile.lock`
  - `gemfile.lock`
  - `composer.lock`
  - `podfile.lock`
  - `mix.lock`
  - `go.sum`
  - `bun.lock`
- Added named-file coverage for more config-like files:
  - `.dockerignore`
  - `.npmrc`
  - `.prettierignore`
  - `.eslintignore`
  - `requirements.txt`
  - `go.mod`
  - `Gemfile`
  - `Pipfile`
- Added generated `broad-local` fixtures to exercise those mappings:
  - `generated_yarn.lock`
  - `generated_poetry.lock`
  - `generated_go.sum`
  - `generated_requirements.txt`
- Immediate useful result:
  - two more lockfile-family losses are now visible on the suite:
    - `generated_poetry.lock`: `+9`
    - `generated_yarn.lock`: `+5`

- lockfile-name trimming after that expansion:
  - `yarn.lock` and `poetry.lock` were worse as `DictionaryText` than on the generic path
  - keep them off the `DictionaryText` named-file list
  - retained A/B:
    - `generated_poetry.lock`: `386 -> 371`
    - `generated_yarn.lock`: `403 -> 398`

- follow-up retained mapping:
  - `poetry.lock` -> `ConfigText`
  - `yarn.lock` -> `ConfigText`
  - retained A/B:
    - `generated_poetry.lock`: `371 -> 362`
    - `generated_yarn.lock`: `398 -> 390`
  - this is the currently retained public file-type starting point for those two names

- `Cargo.lock` public-family retests on the current retained parser shape:
  - `Cargo.lock -> ConfigText`
    - `repo_Cargo.lock`: `9,114 -> 9,255`
  - `Cargo.lock -> CodeText`
    - `repo_Cargo.lock`: `9,114 -> 9,240`
  - Do not retry those public-family remaps in the current design.
  - `Cargo.lock` should stay on the retained `DictionaryText` starting point.

- current retained `Cargo.lock` internal parser follow-ups:
  - stronger repeat-vs-normal margin on the lockfile-specific path
    - `repo_Cargo.lock`: stayed `9,114`
  - require one extra match byte for repeat candidates on the lockfile-specific path
    - `repo_Cargo.lock`: `9,114 -> 9,117`
  - retest dense lockfile probing (`step 1`) on the current retained parser shape
    - `repo_Cargo.lock`: `9,114 -> 9,118`
  - Do not retry either branch in the current lockfile parser shape.

- Poetry-style lockfile detector widening:
  - tried broadening `likely_lockfile_text()` to admit Poetry-style `[[package]]` / `files = [` blocks
  - result: no improvement at all on the harmful mapped path
  - do not retry this detector widening without a different public file-type starting point

- lockfile-only `second_newest` recent-entry limit `2 -> 3`
  - added a focused matcher test first
  - focused `repo_Cargo.lock` benchmark was an exact no-op:
    - `9,114 -> 9,114`
  - do not retry this wider recent-entry reach in the current lockfile parser shape

- lockfile-only fastest whole-vs-estimated-split candidate
  - focused `repo_Cargo.lock`: exact no-op
    - `9,114 -> 9,114`
  - do not retry this best-level-style partition branch in the current lockfile parser shape

- lockfile-only zero-literal non-repeat floor `+1`
  - focused `repo_Cargo.lock`: regression
    - `9,114 -> 9,143`
  - do not retry this stricter zero-literal floor in the current lockfile parser shape

- `DictionaryText` exact Huffman search also evaluating flat-distribution max-bit variants
  - focused `repo_Cargo.lock`: exact no-op
    - `9,114 -> 9,114`
  - do not retry this flat-distribution exact-search branch for the current lockfile gap

- TOML-like `ConfigText` skipping the retained single-stream Huffman override
  - focused `repo_ruzstd_Cargo.toml`: regression
    - `730 -> 737`
  - do not retry this TOML single-stream exception in the current config path

- `DictionaryText` current-over-`newest` displacement
  - focused `dict_dictionary.bin`: exact no-op
    - `20,160 -> 20,160`
  - do not retry this `newest`-side current-window rule in the current dictionary parser shape

- generic `DictionaryText` probe step `1 -> 2`
  - focused `dict_dictionary.bin`: hard regression
    - `20,160 -> 20,667`
  - do not retry this wider generic DictionaryText step in the current parser shape

- adaptive Huffman weight-table FSE max-log `6` vs `5`
  - focused tiny known-file-type literals: exact no-op
    - `repo_.gitignore`: stayed `172`
    - `dict_talk.service`: stayed `160`
    - `repo_ruzstd_Cargo.toml`: stayed `730`
  - corrected `broad-local`: only two already-winning fixtures improved
    - `build_ruzstd-cli`: `866,125 -> 866,118`
    - `repo_match_generator.rs`: `27,879 -> 27,877`
  - corrected broad-local bytes-above-C on losers stayed `1,182`
  - keep it: broad-local clean, slightly positive, now explicitly covered by tests

- adaptive Huffman weight-table FSE max-log `7`
  - tried extending the retained weight-table search from `5/6` to `5/6/7`
  - result: invalid output on the focused lockfile family
    - `tools/benchmark_zstd.py` failed decode verification on `generated_go.sum.current.zst`
  - restore check after reverting returned:
    - `repo_Cargo.lock = 9,114`
    - `generated_go.sum = 151`
    - `generated_poetry.lock = 362`
    - `generated_yarn.lock = 390`
  - do not retry `>6` in this weight-table FSE branch without evidence that the emitted representation is valid

- lockfile zero-literal non-repeat window displacement
  - tried: keep the current non-repeat candidate over a zero-literal non-repeat window candidate unless the zero-literal candidate gains at least `2` bytes
  - focused result: exact no-op
    - `repo_Cargo.lock`: stayed `9,114`
  - do not retry this exact zero-literal displacement rule in the current lockfile parser shape

- lockfile zero-literal `second_newest` ordering change
  - tried: keep the retained `second_newest-before-newest` order only when literals are pending; use normal `newest`-first order on zero-literal positions
  - focused result: regression
    - `repo_Cargo.lock`: `9,114 -> 9,164`
  - do not retry this zero-literal ordering change in the current lockfile parser shape

- retained archive-inspector literal split
  - compressed literal block diagnostics now print:
    - `literals_table_desc`
    - `literals_stream`
  - key `Cargo.lock` result:
    - current: `literals_payload=6886`, `literals_table_desc=25`, `literals_stream=6855`
    - C: `literals_payload=5975`, `literals_table_desc=39`, `literals_stream=5930`
  - implication:
    - the remaining `Cargo.lock` literal gap is not in the Huffman table-description bytes
    - it is in the coded literal stream itself

- rank-limited candidate in `DictionaryText` exact Huffman search
  - tried: when `build_smallest_from_counts()` searches exact Huffman tables for non-flat distributions, also consider the `rank_limited_weights()` candidate
  - focused live `DictionaryText` family result: exact no-op
    - `repo_Cargo.lock`: stayed `9,114`
    - `dict_dictionary.bin`: stayed `20,160`
    - `generated_go.sum`: stayed `151`
  - do not retry this rank-limited exact-search branch in the current Huffman model

- lockfile package-boundary partition candidates
  - tried inside the retained `Cargo.lock` `DictionaryText` path:
    - one split at the `[[package]]` boundary nearest the midpoint
    - multiple splits at the `[[package]]` boundaries nearest the quartiles
  - focused lockfile-family result: exact no-op
    - `repo_Cargo.lock`: stayed `9,114`
    - `generated_go.sum`: stayed `151`
    - `generated_poetry.lock`: stayed `362`
    - `generated_yarn.lock`: stayed `390`
  - do not retry this package-boundary block-partition family in the current lockfile parser shape

- lockfile zero-literal high-offset filter
  - tried: reject zero-literal non-repeat window candidates when they are only `5` bytes long and cost at least `11` offset-code bits
  - focused lockfile-family result: regression
    - `repo_Cargo.lock`: `9,114 -> 9,132`
    - `generated_go.sum`: stayed `151`
    - `generated_poetry.lock`: stayed `362`
    - `generated_yarn.lock`: stayed `390`
  - do not retry this exact high-offset zero-literal filter in the current lockfile parser shape

- rank-limited candidate in exact Huffman search
  - kept globally in `build_smallest_from_counts()`
  - wins on the tiny literal-header tail without moving the sensitive lockfile path
  - focused:
    - `repo_.gitignore`: `172 -> 166`
    - `dict_talk.service`: `160 -> 151`
    - `generated_poetry.lock`: `362 -> 359`
    - `repo_Cargo.lock`: stayed `9,114`
  - broad-local additional wins:
    - `generated_yarn.lock`: `390 -> 383`
    - `dict_git-daemon@.service`: `241 -> 237`
    - `dict_glustereventsd.service`: `285 -> 281`
  - corrected broad-local summary vs C moved to:
    - `37 / 10 / 4`
    - `1,170` bytes above C on the losers
  - keep this branch

- lockfile stream-first Huffman search
  - tried: for `Cargo.lock`-like `DictionaryText`, keep the current exact-table candidate set but choose the new table by coded stream size instead of total literal payload size
  - focused lockfile-family result: exact no-op
    - `repo_Cargo.lock`: stayed `9,114`
    - `generated_go.sum`: stayed `151`
    - `generated_poetry.lock`: stayed `359`
    - `generated_yarn.lock`: stayed `383`
  - do not retry this exact stream-first re-ranking of the current Huffman candidate set on the retained lockfile path

- generic DictionaryText current-entry second_newest
  - kept for non-lockfile text-like `DictionaryText`
  - focused:
    - `dict_dictionary.bin`: `20,160 -> 19,668`
    - `repo_Cargo.lock`: stayed `9,114`
    - `generated_go.sum`: stayed `151`
  - broad-local:
    - only `dict_dictionary.bin` moved
  - corrected broad-local summary vs C moved to:
    - `38 / 9 / 4`
    - `1,155` bytes above C on the losers
  - keep this branch

- lockfile structural midpoint split
  - tried: for a large `Cargo.lock`-like `DictionaryText` block, split once at the newline nearest the midpoint and compress the halves as separate blocks
  - focused lockfile-family result: exact no-op
    - `repo_Cargo.lock`: stayed `9,114`
    - `generated_go.sum`: stayed `151`
    - `generated_poetry.lock`: stayed `359`
    - `generated_yarn.lock`: stayed `383`
  - do not retry this exact forced midpoint two-block split on the retained lockfile path

- lockfile zero-literal short second_newest filter
  - tried: reject lockfile `second_newest` window candidates when they are zero-literal, non-repeat, and only `5` bytes long
  - focused lockfile-family result: exact no-op
    - `repo_Cargo.lock`: stayed `9,114`
    - `generated_go.sum`: stayed `151`
    - `generated_poetry.lock`: stayed `359`
    - `generated_yarn.lock`: stayed `383`
  - do not retry this exact zero-literal short `second_newest` filter on the retained lockfile path

- Cargo.toml -> CodeText
  - tried: map `Cargo.toml` filenames to `CodeText` instead of `ConfigText`
  - focused Cargo.toml-family result: exact no-op
    - `repo_Cargo.toml`: stayed `68`
    - `repo_cli_Cargo.toml`: stayed `489`
    - `repo_ruzstd_Cargo.toml`: stayed `730`
    - `repo_ruzstd_fuzz_Cargo.toml`: stayed `340`
  - do not retry this exact `Cargo.toml -> CodeText` mapping branch on the current retained baseline

- composer.lock / Pipfile.lock -> ConfigText
  - tried: map `composer.lock` and `Pipfile.lock` to `ConfigText` instead of `DictionaryText`
  - focused lockfile-family result: regression
    - `generated_composer.lock`: `4,461 -> 4,469`
    - `generated_pipfile.lock`: `2,811 -> 2,879`
    - `generated_package-lock.json`: stayed `4,392`
    - `generated_go.sum`: stayed `151`
    - `repo_Cargo.lock`: stayed `9,114`
  - do not retry this exact `ConfigText` remap for those two lockfiles on the current retained baseline

- composer-style DictionaryText lockfile path
  - tried: treat large composer-style JSON lockfiles as the retained lockfile parser path inside `DictionaryText`
  - focused result: exact no-op
    - `generated_composer.lock`: stayed `4,461`
    - `generated_pipfile.lock`: stayed `2,811`
    - `generated_package-lock.json`: stayed `4,392`
    - `generated_go.sum`: stayed `151`
    - `repo_Cargo.lock`: stayed `9,114`
  - do not retry this exact lockfile-path remap inside `DictionaryText`

- composer-style DictionaryText non-repeat floor 6
  - tried: raise the non-repeat floor to `6` for large composer-style `DictionaryText`
  - focused result: exact no-op
    - `generated_composer.lock`: stayed `4,461`
    - `generated_pipfile.lock`: stayed `2,811`
  - do not retry this exact composer-floor branch

- composer-style DictionaryText broader smaller-offset preference
  - tried: prefer smaller non-repeat offsets for large composer-style `DictionaryText` when the farther match only gains a byte
  - focused result: exact no-op
    - `generated_composer.lock`: stayed `4,461`
    - `generated_pipfile.lock`: stayed `2,811`
  - do not retry this exact composer-specific smaller-offset branch

- composer-style DictionaryText current-entry long-hash
  - tried: enable the current-entry long-hash path for large composer-style `DictionaryText`
  - focused result: exact no-op
    - `generated_composer.lock`: stayed `4,461`
    - `generated_pipfile.lock`: stayed `2,811`
    - `generated_package-lock.json`: stayed `4,392`
    - `generated_go.sum`: stayed `151`
    - `repo_Cargo.lock`: stayed `9,114`
  - do not retry this exact composer-specific long-hash branch

- composer-style DictionaryText zero-literal rep1-1 first
  - tried: for composer-style `DictionaryText`, try the zero-literal `rep1-1` candidate first instead of last
  - focused result: exact no-op
    - `generated_composer.lock`: stayed `4,461`
    - `generated_pipfile.lock`: stayed `2,811`
    - `generated_package-lock.json`: stayed `4,392`
    - `generated_go.sum`: stayed `151`
    - `repo_Cargo.lock`: stayed `9,114`
  - do not retry this exact composer-specific zero-literal rep ordering branch

- composer-style DictionaryText ip+1 repeat comparison
  - tried: for composer-style `DictionaryText`, allow `ip+1` repeat candidates to be compared even when a current-position repeat candidate already exists
  - focused result: exact no-op
    - `generated_composer.lock`: stayed `4,461`
    - `generated_pipfile.lock`: stayed `2,811`
    - `generated_package-lock.json`: stayed `4,392`
    - `generated_go.sum`: stayed `151`
    - `repo_Cargo.lock`: stayed `9,114`
  - do not retry this exact composer-specific `ip+1` repeat branch

- indexed file-type lookup expansion + sampled fallback
  - retained in `ruzstd/src/encoding/mod.rs`
  - widened known extension and named-file coverage substantially
  - compound extensions now classify directly
  - unknown path misses now sample up to `32 KiB` before falling back to `Unknown`
  - added broad-local fixtures for:
    - `generated_package.json`
    - `generated_tsconfig.json`
    - `generated_pyproject.toml`
    - `generated_pom.xml`
    - `generated_Dockerfile`
  - refreshed broad-local baseline:
    - `62` fixtures
    - `43 / 15 / 4` better / worse / equal vs C
    - `2,421` bytes above C on losing fixtures

- confidence-based sampled fallback
  - retained in `ruzstd/src/encoding/mod.rs`
  - removed the generic “plain text => ConfigText” fallback
  - sample fallback now only claims:
    - archive signatures
    - binary signatures
    - JSON-like text
    - config-like text
    - code-like text
    - lockfile-like text
  - ambiguous plain text now stays `Unknown`
  - important recovery:
    - `decodecorpus_z000079`: `7,530 -> 7,322`

- additional special-name classifier coverage
  - retained in `ruzstd/src/encoding/mod.rs`
  - added:
    - `BUILD.bazel`
    - `MODULE.bazel`
    - `WORKSPACE`
    - `.bzl` / `.bazel`
    - `pubspec.yaml`
    - `pubspec.lock`
    - `melos.yaml`
    - `Podfile`
    - `Brewfile`
  - added broad-local fixtures:
    - `generated_pubspec.yaml`
    - `generated_pubspec.lock`
    - `generated_BUILD.bazel`
    - `generated_WORKSPACE`

- JSON-config family remap to `JsonText`
  - tried for:
    - `package.json`
    - `tsconfig.json`
    - `jsconfig.json`
    - `composer.json`
  - focused result: exact no-op on exposed fixtures
    - `generated_package.json`: stayed `3,956`
    - `generated_tsconfig.json`: stayed `2,492`
  - do not retry this plain remap as a standalone branch

- structured-JSON `ConfigText` short-line floor `7`
  - tried: detect structured JSON inside `ConfigText` and raise the short-line non-repeat floor
  - focused result: hard regression
    - `generated_package.json`: `3,956 -> 3,960`
    - `generated_tsconfig.json`: `2,492 -> 3,292`
  - do not retry this blunt structured-JSON floor branch

- structured-JSON cheaper repeat-code bias
  - tried: for zero-literal structured JSON `ConfigText`, prefer the cheaper repeat code when match loss is at most `2`
  - matcher diagnostics changed heavily for `generated_tsconfig.json`
  - focused result: exact byte-for-byte no-op
    - `generated_package.json`: stayed `3,956`
    - `generated_tsconfig.json`: stayed `2,492`
  - do not retry this repeat-code-rank-only branch

- structured-JSON repeat-length early-exit disable
  - tried: prevent repeat matches from short-circuiting window search on structured JSON `ConfigText`
  - focused result: exact no-op
    - `generated_package.json`: stayed `3,956`
    - `generated_tsconfig.json`: stayed `2,492`
  - do not retry this standalone early-exit branch

- structured-JSON zero-literal repeat min len `6`
  - tried: suppress minimum-length zero-literal repeat matches on structured JSON `ConfigText`
  - matcher diagnostics changed, including:
    - `generated_tsconfig.json` total sequences `4567 -> 4522`
  - focused result: exact byte-for-byte no-op
    - `generated_package.json`: stayed `3,956`
    - `generated_tsconfig.json`: stayed `2,492`
  - do not retry this standalone zero-literal repeat floor branch

- retained known-name classifier expansion
  - retained in `ruzstd/src/encoding/mod.rs`
  - added exact-name coverage for more config/build files, including:
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
  - added extension coverage for:
    - code-like: `.bicep`, `.nix`
    - config-like: `.pbxproj`, `.props`, `.resx`, `.tf`, `.tfvars`, `.xcconfig`, `.xconfig`
    - JSON-like: `.hjson`, `.xcstrings`
  - expanded broad-local fixtures with:
    - `generated_turbo.json`
    - `generated_deno.json`
    - `generated_nx.json`
    - `generated_wrangler.toml`
    - `generated_buf.yaml`
  - refreshed broad-local retained baseline:
    - `71` fixtures
    - `48 / 19 / 4` better / worse / equal vs C
    - `2,449` bytes above C on losing fixtures
  - newly exposed important tails:
    - `generated_turbo.json`: `+130`
    - `generated_deno.json`: `+101`
    - `generated_nx.json`: `+101`
  - current biggest losses remain:
    - `repo_Cargo.lock`: `+1,026`
    - `generated_composer.lock`: `+570`

- retained CodeText short-line current-entry `second_newest`
  - retained in `ruzstd/src/encoding/match_generator.rs`
  - scope:
    - `CodeText`
    - short-line text blocks
    - block size `16 KiB ..= 64 KiB`
  - focused A/B vs retained baseline:
    - `repo_prepare_benchmark_suites.py`: `7,221 -> 6,827`
    - `repo_match_generator.rs`: `28,078 -> 27,845`
    - unchanged:
      - `repo_benchmark_zstd.py`: `2,814`
      - `repo_compressed.rs`: `13,046`
      - `repo_main.rs`: `2,125`
  - useful matcher evidence on `repo_prepare_benchmark_suites.py`:
    - `window_current_second_newest[0]`: `0 -> 72`
    - `window_current_second_newest_zero_literals[0]`: `0 -> 32`
    - `window_current_oldest[0]`: `430 -> 372`
  - refreshed broad-local retained baseline:
    - `71` fixtures
    - `49 / 18 / 4` better / worse / equal vs C
    - `2,309` bytes above C on losing fixtures

- structured-JSON stronger zero-literal repeat floor
  - tried for structured JSON `ConfigText` only:
    - second repeat candidate minimum len `8`
    - third repeat candidate minimum len `10`
  - focused result:
    - `generated_package.json`: stayed `3,956`
    - `generated_turbo.json`: stayed `3,956`
    - `generated_tsconfig.json`: `2,492 -> 3,446`
    - `generated_deno.json`: `2,492 -> 3,446`
    - `generated_nx.json`: `2,492 -> 3,446`
  - do not retry this stronger repeat-length-gating family in this form

- composer current-window offset-choice
  - tried for composer-style `DictionaryText` only:
    - keep the current smaller-offset non-repeat candidate over farther current-window
      `newest` / `oldest` candidates unless the farther one gains at least `2` bytes and
      overcomes the offset-code-bit gap
  - focused result: exact byte-for-byte no-op
    - `generated_composer.lock`: stayed `4,336`
    - `generated_pipfile.lock`: stayed `2,811`
    - `generated_package-lock.json`: stayed `4,392`
    - `generated_go.sum`: stayed `151`
    - `repo_Cargo.lock`: stayed `9,114`
  - do not retry this composer current-window offset-choice family in this form

- package-style JSON next-position repeat lookahead
  - tried for package-style JSON `ConfigText` only:
    - let the existing `ip+1` repeat-lookahead path run for package-style JSON content
  - focused result: exact byte-for-byte no-op
    - `generated_package.json`: stayed `3,956`
    - `generated_turbo.json`: stayed `3,956`
    - `generated_tsconfig.json`: stayed `2,492`
    - `generated_deno.json`: stayed `2,492`
    - `generated_nx.json`: stayed `2,492`
  - do not retry this package-style JSON next-position repeat family in this form

- package-style JSON current-entry second_newest
  - tried for package-style JSON `ConfigText` only:
    - identify `package.json` / `turbo.json`-like content
    - track and probe a current-entry `second_newest` sidecar for that family
  - focused result: exact byte-for-byte no-op
    - `generated_package.json`: stayed `3,956`
    - `generated_turbo.json`: stayed `3,956`
    - `generated_tsconfig.json`: stayed `2,492`
    - `generated_deno.json`: stayed `2,492`
    - `generated_nx.json`: stayed `2,492`
  - do not retry this package-style JSON current-entry `second_newest` family in this form

- structured-JSON `ConfigText` dense probe step `1`
  - kept for structured JSON `ConfigText` only:
    - detect short-line JSON object configs
    - use dense no-match probe step `1` up to `128 KiB`
  - focused result:
    - `generated_package.json`: `3,956 -> 3,785`
    - `generated_turbo.json`: `3,956 -> 3,785`
    - `generated_tsconfig.json`: stayed `2,492`
    - `generated_deno.json`: stayed `2,492`
    - `generated_nx.json`: stayed `2,492`
  - broad-local result:
    - only `generated_package.json` and `generated_turbo.json` moved
    - both became better than C

- tsconfig-style JSON `ConfigText` wider probe step
  - kept for tsconfig-style JSON `ConfigText` only:
    - detect `compilerOptions` / `paths` structured JSON configs
    - keep this subfamily on the wider text no-match probe step `3`
  - focused result:
    - `generated_package.json`: stayed `3,785`
    - `generated_turbo.json`: stayed `3,785`
    - `generated_tsconfig.json`: `2,492 -> 2,489`
    - `generated_deno.json`: `2,492 -> 2,489`
    - `generated_nx.json`: `2,492 -> 2,489`
  - broad-local result:
    - only the tsconfig/deno/nx family moved

- composer-style `DictionaryText` wider probe step
  - kept for composer-style `DictionaryText` only:
    - detect composer-style JSON lockfiles
    - keep this family on the wider text no-match probe step `3`
  - focused result:
    - `generated_composer.lock`: `4,336 -> 4,332`
    - `generated_pipfile.lock`: stayed `2,811`
    - `generated_package-lock.json`: stayed `4,392`
    - `generated_go.sum`: stayed `151`
    - `repo_Cargo.lock`: stayed `9,114`
  - broad-local result:
    - only `generated_composer.lock` moved

- composer-style `DictionaryText` probe step `4`
  - tried for composer-style `DictionaryText` only:
    - widen the retained composer text stride from `3` to `4`
  - focused result:
    - `generated_composer.lock`: `4,332 -> 4,336`
    - `generated_pipfile.lock`: stayed `2,811`
    - `generated_package-lock.json`: stayed `4,392`
    - `generated_go.sum`: stayed `151`
    - `repo_Cargo.lock`: stayed `9,114`
  - do not retry this composer probe-step family past the retained `3` point

- tsconfig-style JSON `ConfigText` probe step `4`
  - kept for tsconfig-style JSON `ConfigText` only:
    - widen the retained tsconfig text stride from `3` to `4`
  - focused result:
    - `generated_package.json`: stayed `3,785`
    - `generated_turbo.json`: stayed `3,785`
    - `generated_tsconfig.json`: `2,489 -> 2,486`
    - `generated_deno.json`: `2,489 -> 2,486`
    - `generated_nx.json`: `2,489 -> 2,486`
  - broad-local result:
    - only the tsconfig/deno/nx family moved

- fastest-only raw composer package-boundary split
  - tried in `ruzstd/src/encoding/frame_compressor.rs`:
    - for composer-style `DictionaryText`, split the raw input at package-object boundaries
      before calling `compress_fastest`
  - because the worktree had moved beyond the older retained binary, compared this branch
    against a binary built from the current restored source tree
  - focused result vs current source baseline:
    - `generated_composer.lock`: `4,332 -> 4,352`
    - `generated_pipfile.lock`: stayed `2,811`
    - `generated_package-lock.json`: stayed `4,392`
    - `generated_go.sum`: stayed `151`
    - `repo_Cargo.lock`: stayed `9,114`
  - restore check returned exact equality to the current source baseline
  - do not retry this raw composer multi-block split family in this form

- tsconfig-style JSON `ConfigText` probe step `5`
  - kept for tsconfig-style JSON `ConfigText` only:
    - widen the retained tsconfig text stride from `4` to `5`
  - focused result vs current source baseline:
    - `generated_package.json`: stayed `3,785`
    - `generated_turbo.json`: stayed `3,785`
    - `generated_tsconfig.json`: `2,486 -> 2,485`
    - `generated_deno.json`: `2,486 -> 2,485`
    - `generated_nx.json`: `2,486 -> 2,485`
  - broad-local result:
    - only the tsconfig/deno/nx family moved

- lockfile-like `DictionaryText` probe step `3`
  - tried for the active lockfile parser shape:
    - widen the lockfile no-match stride from `2` to `3`
  - focused result vs current source baseline:
    - `repo_Cargo.lock`: stayed `9,114`
    - `generated_go.sum`: stayed `151`
    - `generated_poetry.lock`: stayed `359`
    - `generated_yarn.lock`: stayed `383`
  - restore check returned exact equality to the current source baseline
  - do not retry this lockfile probe-step widening family in this form

- composer repeat-aware same-start preference
  - tried for composer-style `DictionaryText` only:
    - prefer a repeat-offset candidate over a non-repeat candidate when both start at the same
      byte and the repeat loses at most `1` match byte
  - focused result vs current source baseline:
    - `generated_composer.lock`: stayed `4,332`
    - `generated_pipfile.lock`: stayed `2,811`
    - `generated_package-lock.json`: stayed `4,392`
    - `generated_go.sum`: stayed `151`
    - `repo_Cargo.lock`: stayed `9,114`
  - restore check returned exact equality to the current source baseline
  - do not retry this local composer repeat-aware scoring family in this form

- DictionaryText OF repeat-table window `1024`
  - tried in `ruzstd/src/encoding/blocks/compressed.rs`:
    - for fastest-level `DictionaryText`, widen the OF repeat-table reuse window from `64` to
      `1024` sequences while leaving LL/ML behavior unchanged
  - focused result vs current source baseline:
    - `generated_composer.lock`: stayed `4,332`
    - `generated_pipfile.lock`: stayed `2,811`
    - `generated_package-lock.json`: stayed `4,392`
    - `generated_go.sum`: stayed `151`
    - `repo_Cargo.lock`: stayed `9,114`
  - restore check returned exact equality to the current source baseline
  - do not retry this OF-only repeat-table window family in this form

- `pubspec.lock -> ConfigText`
  - tried in `ruzstd/src/encoding/mod.rs`:
    - remap `pubspec.lock` from `DictionaryText` to `ConfigText`
  - focused result vs current source baseline:
    - `generated_pubspec.lock`: stayed `233`
    - `generated_pubspec.yaml`: stayed `187`
    - `generated_go.sum`: stayed `151`
    - `generated_poetry.lock`: stayed `359`
  - restore check returned exact equality to the current source baseline
  - do not retry this `pubspec.lock` remap in this direction

- composer repeat-kind preference at same start
  - kept in `ruzstd/src/encoding/match_generator.rs`:
    - for composer-style `DictionaryText`, when two current-position repeat candidates start at
      the same byte, prefer the repeat kind that matches the encoder's repeat-code order if it
      loses at most `1` match byte
  - focused result vs current source baseline:
    - `generated_composer.lock`: `4,332 -> 4,160`
    - `generated_pipfile.lock`: stayed `2,811`
    - `generated_package-lock.json`: stayed `4,392`
    - `generated_go.sum`: stayed `151`
    - `repo_Cargo.lock`: stayed `9,114`
  - broad-local result:
    - only `generated_composer.lock` moved
    - refreshed retained baseline:
      - `71` fixtures
      - `51 / 16 / 4` better / worse / equal vs C
      - `1,852` bytes above C on losing fixtures
  - useful matcher shift on `generated_composer.lock`:
    - `repeat_current`: `[946, 518, 739] -> [909, 664, 687]`
    - `repeat_current_zero_literals`: `[0, 438, 602] -> [0, 647, 445]`
  - this is the new retained best point for the composer repeat family

- lockfile repeat-kind preference at same start
  - kept in `ruzstd/src/encoding/match_generator.rs`:
    - for lockfile-like `DictionaryText`, when two current-position repeat candidates start at
      the same byte, prefer the repeat kind that matches the encoder repeat-code order if it
      loses at most `1` match byte
  - focused result vs current retained baseline:
    - `repo_Cargo.lock`: `9,114 -> 9,111`
    - `generated_go.sum`: stayed `151`
    - `generated_poetry.lock`: stayed `359`
    - `generated_yarn.lock`: stayed `383`
  - broad-local result:
    - only `repo_Cargo.lock` moved
    - refreshed retained baseline:
      - `71` fixtures
      - `51 / 16 / 4` better / worse / equal vs C
      - `1,849` bytes above C on losing fixtures
  - useful matcher shift on `repo_Cargo.lock`:
    - `repeat_current`: `[65, 24, 10] -> [71, 19, 9]`
    - `repeat_best_before_window`: `[67, 25, 11] -> [73, 20, 10]`
  - this is the new retained best point for the narrow lockfile repeat-kind family

- lockfile repeat-kind preference match-loss `2`
  - tried in `ruzstd/src/encoding/match_generator.rs`:
    - widen the retained lockfile repeat-kind match-loss allowance from `1` to `2`
  - focused result vs current retained baseline:
    - exact no-op:
      - `repo_Cargo.lock = 9,111`
      - `generated_go.sum = 151`
      - `generated_poetry.lock = 359`
      - `generated_yarn.lock = 383`
  - live matcher diagnostics on `repo_Cargo.lock` were identical to the retained point
  - do not widen this retained lockfile repeat-kind family in this direction

- fastest lockfile partition-path retest
  - tried in `ruzstd/src/encoding/levels/fastest.rs`:
    - let lockfile-like `DictionaryText` reach the existing fastest-level partition candidate path
  - focused result vs current retained baseline:
    - exact no-op:
      - `repo_Cargo.lock = 9,111`
      - `generated_go.sum = 151`
      - `generated_poetry.lock = 359`
      - `generated_yarn.lock = 383`
  - live matcher diagnostics on `repo_Cargo.lock` were identical to the retained point
  - do not revisit this structural family in the same form on the active retained baseline

- lockfile same-end smaller-offset preference
  - kept in `ruzstd/src/encoding/match_generator.rs`:
    - for lockfile-like `DictionaryText`, when two non-repeat candidates end at the same byte,
      prefer the smaller-offset candidate if it loses at most `1` match byte and saves at least
      `2` offset-code bits
  - focused result vs current retained baseline:
    - `repo_Cargo.lock`: `9,111 -> 9,109`
    - `generated_go.sum`: stayed `151`
    - `generated_poetry.lock`: stayed `359`
    - `generated_yarn.lock`: stayed `383`
  - broad-local result:
    - only `repo_Cargo.lock` moved
    - refreshed retained baseline:
      - `71` fixtures
      - `51 / 16 / 4` better / worse / equal vs C
      - `1,847` bytes above C on losing fixtures
  - useful matcher shift on `repo_Cargo.lock`:
    - `repeat_current`: `[71, 19, 9] -> [72, 19, 9]`
    - `window_current_newest[0]`: `421 -> 422`
    - `window_current_second_newest[0]`: `105 -> 103`
  - this is the new retained best point for the narrow lockfile same-end family

- lockfile same-end smaller-offset match-loss `2`
  - tried in `ruzstd/src/encoding/match_generator.rs`:
    - widen the retained lockfile same-end match-loss allowance from `1` to `2`
  - focused result vs current retained baseline:
    - exact no-op:
      - `repo_Cargo.lock = 9,109`
      - `generated_go.sum = 151`
      - `generated_poetry.lock = 359`
      - `generated_yarn.lock = 383`
  - live matcher diagnostics on `repo_Cargo.lock` were identical to the retained point
  - do not widen this retained lockfile same-end family in this direction

- lockfile OF table max-log `6`
  - tried across the fastest-level whole-block and partition paths:
    - when the block is lockfile-like `DictionaryText`, lower OF table max-log from `7` to `6`
  - focused result vs current retained baseline:
    - `repo_Cargo.lock`: `9,109 -> 9,145`
    - unchanged controls:
      - `generated_go.sum = 151`
      - `generated_poetry.lock = 359`
      - `generated_yarn.lock = 383`
  - live matcher diagnostics on `repo_Cargo.lock` were identical to the retained point
  - do not revisit this smaller-OF-max-log family in the same form

- focused matcher tuner
  - added:
    - `tools/tune_matcher_family.py`
  - runtime-tunable matcher surface now covers:
    - lockfile probe step
    - composer probe step
    - structured-json probe step
    - tsconfig-json probe step
    - dictionary same-start smaller-offset thresholds
    - lockfile same-end smaller-offset thresholds
    - lockfile/composer repeat-kind match-loss thresholds

- `cargo-lock` tuner sweep
  - focused family total:
    - baseline `10,002`
    - best searched candidate `10,002`
  - searched local surface did not beat the current retained point

- `composer` tuner sweep
  - focused family total:
    - baseline `11,514`
    - best searched candidate `11,514`
  - searched local surface did not beat the current retained point

- `structured-json` tuner sweep
  - focused family total:
    - baseline `7,570`
    - best searched candidate `7,570`
  - probe step `1` remains the retained best point

- tsconfig-style JSON probe step `6`
  - kept in `ruzstd/src/encoding/match_generator.rs`:
    - raise `TSCONFIG_JSON_TEXT_NO_MATCH_PROBE_STEP` from `5` to `6`
  - focused result vs current retained baseline:
    - `generated_tsconfig.json`: `2,485 -> 2,484`
    - `generated_deno.json`: `2,485 -> 2,484`
    - `generated_nx.json`: `2,485 -> 2,484`
  - broad-local result:
    - only the tsconfig-style JSON family moved
    - `1,847 -> 1,844` bytes above C on losing fixtures
  - retained best point for the tsconfig-style JSON subfamily is now step `6`

- tuner race fix
  - fixed `tools/tune_matcher_family.py` so concurrent sweeps no longer reuse the same temp
    output filenames
  - do not trust the earlier pre-fix encoder totals that showed impossible large wins

- dependency-JSON lockfile encoder config
  - added in `ruzstd/src/encoding/util.rs` and `ruzstd/src/encoding/blocks/compressed.rs`
  - detect `package-lock.json` / `Pipfile.lock`-style large JSON lockfiles
  - for that subfamily only:
    - use `HuffmanTableSearch::AllSections`
    - use `repeat_table_max_sequences = 256`
    - use `offset_table_max_log = 8`
  - focused result vs current retained baseline:
    - `generated_package-lock.json`: `4,392 -> 4,388`
    - `generated_pipfile.lock`: `2,811 -> 2,804`
    - `generated_composer.lock`: unchanged at `4,160`
  - broad-local result:
    - only those two fixtures moved
    - overall loser summary vs C stayed `51 / 16 / 4`, `1,844` bytes above C

- `cargo-lock-encoder` tuner sweep
  - focused family total:
    - baseline `10,002`
    - best searched candidate `10,001`
  - not worth retaining globally in the searched form

- `composer-encoder` tuner sweep
  - after fixing the tuner race, the searched win was really a dependency-JSON lockfile win
  - do not treat it as a composer-specific retained point

- whole-file dependency-JSON profile
  - added an internal file-profile layer in:
    - `ruzstd/src/encoding/mod.rs`
    - `ruzstd/src/encoding/frame_compressor.rs`
    - `ruzstd/src/encoding/levels/fastest.rs`
    - `ruzstd/src/encoding/blocks/compressed.rs`
  - public `CompressionFileType` stayed unchanged
  - the dependency-JSON encoder profile now applies across every block for:
    - `package-lock.json`
    - `Pipfile.lock`
  - focused result vs prior retained baseline:
    - `generated_package-lock.json`: `4,388 -> 4,383`
    - unchanged:
      - `generated_pipfile.lock = 2,804`
      - `generated_composer.lock = 4,160`
      - `generated_go.sum = 151`
      - `repo_Cargo.lock = 9,109`
  - broad-local result:
    - only `generated_package-lock.json` moved
    - retained summary vs C stayed:
      - `71` fixtures
      - `51 / 16 / 4`
      - `1,844` bytes above C on losers

- extended probe-step checks
  - `tsconfig-json`:
    - probe step `7`: no-op
    - probe step `8`: no-op
  - composer:
    - probe step `5`: retained best point
      - `generated_composer.lock`: `4,160 -> 4,159`
      - no other focused composer-family fixture moved
    - probe step `6`: regression
      - `generated_composer.lock`: `4,160 -> 4,171`

- rejected structural branches
  - single-segment frame headers when content size is known:
    - broad-local regression across many fixtures
    - do not revisit this as a default level-1 path in the same form
  - composer whole-vs-split comparison even for text blocks:
    - exact no-op on the focused family

- retained composer probe step `5`
  - changed default composer-style `DictionaryText` no-match probe step from `3` to `5`
  - broad-local result:
    - only `generated_composer.lock` moved
    - retained summary vs C became:
      - `71` fixtures
      - `51 / 16 / 4`
      - `1,843` bytes above C on losers

- composer partition-cap sweep
  - added runtime override:
    - `RUZSTD_TUNE_COMPOSER_MAX_PARTITIONS`
  - added tuner preset:
    - `composer-partitions`
  - focused result against the current retained baseline:
    - `generated_composer.lock`
      - cap `1`: `4,255`
      - cap `2`: `4,194`
      - caps `3..8`: all `4,159`
  - conclusion:
    - the composer partition-count family is now bounded
    - current retained point is already on the best plateau for this family

- expanded `cargo-lock` tuner sweeps
  - widened searched matcher surface to include lower lockfile same-end / repeat-kind thresholds:
    - `RUZSTD_TUNE_LOCKFILE_REPEAT_KIND_MATCH_LOSS_MAX = 0/1/2`
    - `RUZSTD_TUNE_LOCKFILE_SAME_END_MATCH_LOSS_MAX = 0/1/2`
    - `RUZSTD_TUNE_LOCKFILE_SAME_END_BITS_GAIN_MIN = 1/2/3`
  - `cargo-lock` matcher-only result:
    - baseline `10,002`
    - best searched candidate `10,002`
  - `cargo-lock-combined` result:
    - baseline `10,002`
    - best searched candidate `10,001`
    - the `-1` was only `generated_poetry.lock: 359 -> 358`
    - `repo_Cargo.lock` stayed `9,109`
  - isolated follow-up:
    - `RUZSTD_TUNE_OFFSET_PREDEFINED_MAX_SEQUENCES = 64` was the only winning lever in that
      combined candidate
    - broad-local spot check was not clean, so it was not retained

- `cargo-lock-literal-encoder` tuner sweep
  - new focused encoder-only preset for the previously unsearched lockfile literal surface:
    - `RUZSTD_TUNE_HUFFMAN_TABLE_SEARCH`
    - `RUZSTD_TUNE_FILE_TYPE_SINGLE_STREAM_HUFFMAN_MAX_LITERALS`
    - `RUZSTD_TUNE_FILE_TYPE_SMALL_SEQUENCE_PREDEFINED_LLML_MAX_SEQUENCES`
  - result:
    - baseline `10,002`
    - best searched candidate `10,002`
  - conclusion:
    - current `Cargo.lock` family did not improve anywhere on that searched literal encoder
      surface

- lockfile zero-literal `second_newest` gate
  - added tune-only matcher override:
    - `RUZSTD_TUNE_LOCKFILE_SECOND_NEWEST_ZERO_LITERALS`
  - disabling zero-literal `second_newest` probes:
    - changed live `Cargo.lock` matcher diagnostics materially
    - but was byte-for-byte no-op on the focused lockfile family
  - conclusion:
    - that source-order family is real parse movement, but not a compressed-size win under the
      current encoder surface

- `composer-repeat-zero-literals` tuner sweep
  - new focused family for the dominant live composer repeat-side pattern
  - searched:
    - `RUZSTD_TUNE_COMPOSER_REPEAT_KIND_MATCH_LOSS_MAX = 0/1/2`
    - `RUZSTD_TUNE_COMPOSER_REPEAT_KIND_ZERO_LITERALS_ONLY = 0/1`
  - result:
    - baseline `11,497`
    - best searched candidate `11,497`
  - conclusion:
    - narrowing the retained composer repeat-kind rule to zero-literal repeats does not improve
      the focused composer family

- strong lockfile zero-literal window suppression
  - tune-only matcher override:
    - `RUZSTD_TUNE_LOCKFILE_ZERO_LITERAL_WINDOW_DISABLE`
  - exact byte no-op on the focused lockfile family
  - but live `repo_Cargo.lock` matcher diagnostics changed materially
  - paired with nearby encoder settings, still no `repo_Cargo.lock` byte movement
  - conclusion:
    - the stronger zero-literal window suppression family is real parse movement, but not a size
      win under the current encoder surface

- `composer-window-disable` tuner sweep
  - coarse structural composer family:
    - `RUZSTD_TUNE_COMPOSER_WINDOW_DISABLE = 0/1`
    - searched with nearby OF/repeat encoder knobs
  - result:
    - baseline `11,497`
    - best searched candidate `11,497`
  - conclusion:
    - removing composer non-repeat window candidates is flat in the current searched space

- `composer-zero-literal-repeat-limit` tuner sweep
  - structural composer repeat family:
    - `RUZSTD_TUNE_COMPOSER_ZERO_LITERAL_REPEAT_CANDIDATE_LIMIT = 1/2/3`
    - `RUZSTD_TUNE_COMPOSER_REPEAT_KIND_MATCH_LOSS_MAX = 0/1/2`
  - result:
    - baseline `11,497`
    - best searched candidate `11,497`
  - conclusion:
    - limiting how many zero-literal composer repeat kinds are considered is flat in the current
      searched space

- `cargo-lock-splits` tuner sweep
  - structural fastest-path lockfile family:
    - `RUZSTD_TUNE_LOCKFILE_FASTEST_SPLITS = 0/1`
    - `RUZSTD_TUNE_LOCKFILE_COMPARE_WHOLE_TEXT = 0/1`
  - result:
    - baseline `10,002`
    - best searched candidate `10,002`
  - conclusion:
    - the current fastest split / whole-compare family is flat for the lockfile focus set

- lockfile post-parse zero-literal match dropping
  - correction after rebuilding the release binary:
    - this branch was invalid, not flat
  - the tune-only branch caused decode mismatches because later repeat-offset history no longer
    matched the matcher-produced raw offsets
  - aggressive spot-check also showed large focused regression:
    - `repo_Cargo.lock: 9,109 -> 14,639`
  - the experimental code and preset were removed
  - conclusion:
    - do not reuse this family without a full repeat-history recomputation strategy

- `cargo-lock-sequence-cost` tuner sweep
  - added tune-only matcher score overrides:
    - `RUZSTD_TUNE_LOCKFILE_SEQUENCE_COST_LITERAL_WEIGHT`
    - `RUZSTD_TUNE_LOCKFILE_SEQUENCE_COST_OFFSET_WEIGHT`
    - `RUZSTD_TUNE_LOCKFILE_SEQUENCE_COST_MARGIN`
  - result:
    - baseline `10,002`
    - best searched candidate `10,054`
  - action:
    - removed the tune-only scorer branch and preset
    - exact restore vs retained baseline:
      - [cargolock-sequence-cost-restore.md](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/cargolock-sequence-cost-restore.md)
  - conclusion:
    - broader local sequence-cost scoring regresses the focused `Cargo.lock` family

- `cargo-lock-next-position` tuner sweep
  - added a one-step zero-literal lazy-parse family for lockfile-like `DictionaryText`
  - searched:
    - `RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_CURRENT_MATCH_LEN = 5/6/7/8/9`
    - `RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_LITERAL_WEIGHT = 6/8/10`
    - `RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MATCH_REWARD = 1/2`
    - `RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_OFFSET_WEIGHT = 1/2`
    - `RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MARGIN = 0/1/2`
  - focused result:
    - baseline `10,002`
    - best `9,999`
  - retained point:
    - `max_current_match_len=7`
    - `literal_weight=6`
    - `match_reward=2`
    - `offset_weight=1`
    - `margin=1`
  - broad-local result:
    - only `repo_Cargo.lock` moved
    - `9,109 -> 9,106`
  - useful signal:
    - sequences `821 -> 817`
    - `of_extra_bits 6898 -> 6830`
    - `decoded_literals 9932 -> 9938`

- `cargo-lock-next-position-literals` tuner sweep
  - widened the retained lockfile lazy-parse family from strict zero literals to tiny literal runs
  - result:
    - baseline `9,999`
    - best searched candidate `9,999`
  - action:
    - removed the tune-only tiny-literal extension and preset
    - exact restore vs retained baseline:
      - [lockfile-next-position-restore.md](/home/bsutton/git/zstd-rs/benchmarks/archive/tmp/lockfile-next-position-restore.md)
  - conclusion:
    - the retained lockfile lazy-parse family is currently bounded at strict zero literals

- `cargo-lock-next-position-skip` tuner sweep
  - widened the retained lockfile lazy-parse family in a different direction:
    - compare the current zero-literal lockfile candidate against the best candidate after
      skipping up to `2` or `3` literal bytes
  - focused result:
    - baseline `9,999`
    - best `9,998`
  - retained point:
    - `max_skip_literals=2`
    - `max_current_match_len=7`
    - `literal_weight=6`
    - `match_reward=2`
    - `offset_weight=2`
    - `margin=1`
  - broad-local result:
    - only `repo_Cargo.lock` moved
    - `9,106 -> 9,105`
  - useful signal:
    - sequences `817 -> 810`
    - `sequence_payload_bytes 2195 -> 2184`
    - `of_extra_bits 6830 -> 6777`
    - `decoded_literals 9938 -> 9952`

- retained whole-file `ComposerLock` profile
  - added an internal `CompressionFileProfile::ComposerLock`
  - `composer.lock` paths now carry that hint through the matcher and fastest-level structural
    path instead of relying only on per-block content guesses
  - focused result:
    - `generated_composer.lock`: `4,159 -> 4,119`
    - unchanged:
      - `generated_package-lock.json = 4,381`
      - `generated_pipfile.lock = 2,804`
      - `generated_go.sum = 151`
  - broad-local result:
    - only `generated_composer.lock` moved
    - retained baseline vs C `zstd -1`:
      - `71` fixtures
      - `51 / 16 / 4`
      - `1,798` bytes above C on losers
  - useful signal:
    - current retained composer inspect:
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

- `cargo-lock-next-position-loss` tuner sweep
  - widened the productive lockfile lazy-parse family in a narrower way:
    - still zero-literal only
    - still skip up to `2` future literal bytes
    - but allow an equal-length future candidate to win on local parse cost
  - focused result:
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
  - broad-local result:
    - only `repo_Cargo.lock` moved
    - `9,105 -> 9,104`
  - useful signal:
    - `sequence_count 810 -> 810`
    - `sequence_payload_bytes 2184 -> 2182`
    - `of_extra_bits 6777 -> 6773`
    - `decoded_literals 9952 -> 9953`

- `cargo-lock-next-position-followup` tuner sweep
  - tried a broader two-step version of the productive lockfile lazy-parse family:
    - estimate local path cost with an optional follow-up candidate after the first chosen match
  - result:
    - baseline `9,997`
    - best `9,997`
    - all top candidates kept `RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_USE_FOLLOWUP=0`
  - conclusion:
    - the broader two-step follow-up estimate did not beat the retained one-step point

- `composer-whole-compare` tuner sweep
  - forced fastest-level whole-vs-partition comparison for composer text blocks
  - result:
    - baseline `11,455`
    - best `11,455`
    - byte-identical across partition caps `3..8`
  - conclusion:
    - the retained whole-file composer profile did not expose a hidden whole-block win in the
      current split machinery

- retained `SmallTextLockfile` profile
  - added an internal profile for:
    - `poetry.lock`
    - `pubspec.lock`
  - applied narrow encoder tuning:
    - `HuffmanTableSearch::AllSections`
    - `repeat_table_max_sequences = 256`
    - `offset_table_max_log = 7`
    - `offset_predefined_max_sequences = 64`
  - focused result:
    - `generated_poetry.lock`: `359 -> 358`
    - `generated_pubspec.lock`: `232 -> 229`
    - unchanged:
      - `generated_Gemfile.lock = 239`
      - `generated_go.sum = 151`
      - `generated_pubspec.yaml = 187`
  - broad-local result:
    - only `generated_poetry.lock` and `generated_pubspec.lock` moved
    - retained baseline vs C `zstd -1`:
      - `71` fixtures
      - `51 / 16 / 4`
      - `1,794` bytes above C on losers
