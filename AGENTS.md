# Agent Workflow

## Current Restart Checkpoint

When resuming the zstd C compressor port on branch
`faithful-c-compressor-port`, read
`ruzstd/src/encoding/levels/c_port/README.md` first. The current checkpoint is
the target compressed block size / superblock port.

As of July 16, 2026, target mode dispatch is threaded through the CCtx, frame
state, optimal frame path, and hash-chain frame path. The active target encoder
tries literal-only RLE superblocks, non-empty sequence sub-blocks with
Huffman compressed or treeless literal metadata, and basic literal sub-blocks
with all-RLE, all-repeat, or all-compressed sequence metadata before falling
back to raw blocks. The resolved `targetCBlockSize` value is consumed by the
multi-sub-block path, which can write a full-superblock Huffman literal table
once, use treeless literal sections later, write sequence entropy once, and use
repeat sequence metadata for later sub-blocks.

Next implementation step: extend the multi-sub-block path to use the full
superblock FSE entropy tables, so compressed sequence metadata matches
`ZSTD_compressSubBlock_multi()` across sub-blocks.

## Operating Principle

Use delegation to remove waiting, log scraping, and mechanical follow-through
from the main agent. Keep code ownership, architectural judgment, benchmark
interpretation, and PR positioning with the main agent.

The default sidecar mode is no-edit. A worker should only change files when the
main agent gives an exact file list, exact command sequence, and exact expected
outcome. Prefer narrow prompts over sharing broad conversation history.

## Performance and Validation Delegation

Use a cheaper sidecar agent for repeatable validation and benchmark runs when the
main task is active code work, porting, or review. The main agent should keep
ownership of code changes, interpretation of results, and final commit decisions.

Preferred split:

- Main agent: inspect C/Rust behavior, make scoped code changes, review diffs,
  decide whether to keep or revert experiments, and write commits.
- Sidecar worker: run deterministic checks and benchmark commands, then report
  exact pass/fail status and numbers. Sidecar workers must not edit files or
  revert changes.

Use a low-cost worker model for this validation loop when available. Give the
worker a self-contained prompt rather than forking full history if model
overrides are needed.

Recommended worker scopes:

- `validation-worker`: runs `cargo fmt`, focused tests, clippy, and build
  commands; reports exact failures and artifact paths.
- `benchmark-worker`: runs fixed benchmark commands; reports CSV/Markdown paths
  and summary lines without interpreting whether the PR is worthwhile.
- `ci-worker`: collects GitHub Actions status and failing log excerpts; does not
  choose fixes.
- `hygiene-worker`: reports `git status`, changed-file lists, diff stats,
  generated artifacts, and large-file or Rust file-size checks.
- `commit-worker`: stages an exact file list and commits an exact message after
  the main agent has decided both.

## Cost Control

Delegation should reduce the expensive model's time spent waiting on commands
and reading large logs. It will not reduce CPU time, wall-clock time for the
benchmark itself, or the cost of running the benchmark machine.

Use a sidecar when all of these are true:

- the task is command-heavy or log-heavy,
- the expected output can be summarized in a few lines,
- the worker does not need to make design decisions,
- the main agent can verify the result from the reported command and artifact
  paths.

Do not delegate when the prompt would need large parts of the repository
history, when interpreting the result requires code ownership context, or when
the worker would need to choose between competing implementation approaches.

Do not use delegation merely to avoid thinking about a result. A sidecar should
produce evidence; the main agent should decide what that evidence means.

Keep sidecar prompts narrow. Include the current branch, exact command, expected
artifact paths, and report format. Avoid sending full prior conversation
history unless the worker genuinely needs it.

## Other Good Delegation Targets

Delegate tasks that are mechanical, bounded, and easy for the main agent to
verify. Good sidecar tasks include:

- validation runs,
- benchmark summarisation and CSV comparison,
- worktree hygiene checks,
- diff stats and changed-file summaries,
- Rust file-size reports,
- documentation consistency checks,
- CI log collection and failure summaries,
- no-edit code searches,
- post-change smoke tests.

Delegation is most likely to reduce cost when the task is long-running,
command-heavy, low-reasoning, and easy to summarize. It does not reduce machine
time, and it can add overhead if prompts or reports are too broad.

Keep design decisions, risky code changes, benchmark interpretation, PR
positioning, and final keep-or-revert decisions in the main agent.

## Sidecar Prompt Patterns

Validation prompt:

```text
You are a no-edit validation worker in /home/bsutton/git/zstd-rs on branch
<branch>. Do not modify files. Run the following commands exactly, capture
pass/fail and the important error lines, and report latest commit plus
git status --short:

<commands>
```

Benchmark prompt:

```text
You are a no-edit benchmark worker in /home/bsutton/git/zstd-rs on branch
<branch>. Do not modify files. Run the following benchmark command exactly.
Then summarize the resulting CSV with the supplied awk command and report the
artifact paths, exact summary lines, latest commit, and git status --short:

<benchmark command>
<summary command>
```

Commit prompt:

```text
You are a commit worker in /home/bsutton/git/zstd-rs on branch <branch>. Stage
only these files:

<files>

Inspect git diff --cached and commit with this exact message:

<message>

Report the new commit hash and git status --short. Do not amend, rebase, reset,
revert, or stage any other files.
```

## Delegating Commits

Commit execution can be delegated only after the main agent has decided the
exact commit scope and message. The sidecar prompt must name the exact files to
stage and the exact commit message.

For delegated commits, require the worker to:

- confirm the latest commit and worktree status,
- stage only the named files,
- inspect `git diff --cached`,
- commit with the exact requested message,
- report the new commit hash.

Sidecar workers must not decide what belongs in a commit, stage unrelated files,
amend commits, rebase, reset, or revert changes.

Standard validation command set:

```sh
cargo fmt --check
cargo test -p ruzstd --quiet
cargo clippy -p ruzstd --all-targets -- -D warnings
cargo build --release -p zstd-rs-tools --bin profile_c_port --bin benchmark_c_port --bin profile_c_api
```

Focused benchmark:

```sh
target/release/profile_c_port benchmarks/archive/tmp/realworld-100/corpus_z000033 16 80
```

Broad benchmark:

```sh
target/release/benchmark_c_port --fixtures benchmarks/archive/tmp/realworld-100 --levels 8,16,19 --runs 1 --c-backend api --csv-output benchmarks/tmp/agent-validation.csv --md-output benchmarks/tmp/agent-validation.md --no-sync
```

Broad benchmark summary:

```sh
awk -F, 'NR>1{rows[$2]++; c[$2]+=$4; r[$2]+=$5; ccpu[$2]+=$7; rcpu[$2]+=$8; gap=$5-$4; if(!(($2) in worst) || gap>worst[$2]){worst[$2]=gap; worstf[$2]=$1}} END{for (l in rows) printf "L%s rows=%d c_bytes=%d rust_bytes=%d gap=%+d gap_pct=%+.3f c_cpu=%.4f rust_cpu=%.4f ratio=%.2f worst=%s %+d\n", l, rows[l], c[l], r[l], r[l]-c[l], (r[l]-c[l])*100/c[l], ccpu[l], rcpu[l], rcpu[l]/ccpu[l], worstf[l], worst[l]}' benchmarks/tmp/agent-validation.csv
```

Sidecar reports should include:

- latest commit and worktree status,
- pass/fail for each validation command,
- exact focused benchmark output for each run,
- broad benchmark summary lines,
- any command failures.
