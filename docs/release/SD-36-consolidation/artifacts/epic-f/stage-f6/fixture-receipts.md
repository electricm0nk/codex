# Stage F6 — fixture receipts

Protocol (f6:suite-root): every failing test classed (A) pinned count/status moved by this batch —
re-baselined with a line here; (B) genuine defect — fixed at the root, RED first; (C) unrelated
pre-existing — attributed from git.

## f6:suite-root — root crate at `db359567c5`

| Instrument | Command | Result |
|---|---|---|
| Root suite | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` | 294 test binaries/doc-test groups; 6,415 passed, 0 failed, 27 ignored; `EXIT=0` (`f6-suite-root.log`) |
| Root clippy | `cargo clippy --locked -j 8 --all-targets -- -D warnings` | `EXIT=0`, no warnings (`f6-clippy-root.log`) |

Failing tests: 0 of 6,415 run. Re-baselines (A): none. Defects (B): none. Pre-existing (C): none.
No fixture, pinned count or printed sheet value was changed in this step.

Denominator: the counts sum the `test result:` lines of the root-crate run (294 lines); the desktop
crate (`apps/desktop/src-tauri`, a separate workspace) is not in this figure.
