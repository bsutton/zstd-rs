# Agent Workflow

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
