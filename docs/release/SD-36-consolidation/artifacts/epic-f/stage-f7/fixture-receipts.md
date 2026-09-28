# Stage F7 — fixture receipts

Protocol: every failing test is classified (A) pinned count/status moved by this batch — re-baselined
with a line here; (B) genuine defect — fixed at the root, RED first; (C) unrelated pre-existing —
attributed from git.

## f7:suite-root+ingest (worktree HEAD 30affb322c, branch sd36/epic-f7-sheet-visible)

| Run | Command | Result |
|---|---|---|
| Root crate suite | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` | EXIT=0; 294 test binaries; 6,429 passed, 0 failed, 27 ignored (`f7-root-suite.log`) |
| Root crate clippy | `cargo clippy --locked -j 8 --all-targets -- -D warnings` | EXIT=0; 0 warnings (`f7-root-clippy.log`) |
| Ingest crate suite | `cargo test --locked -j 8 -p codex-ingest --no-fail-fast -- --test-threads=8` | EXIT=0; 167 test binaries; 1,767 passed, 0 failed, 43 ignored (`f7-ingest-suite.log`) |

Counts: sum of the `test result:` lines in each log (denominator = every test binary cargo ran for that command).

Failing tests: 0 of 6,429 + 1,767. Class A re-baselines: none. Class B fixes: none. Class C attributions: none.
No fixture was changed in this step; no printed sheet value moved.
