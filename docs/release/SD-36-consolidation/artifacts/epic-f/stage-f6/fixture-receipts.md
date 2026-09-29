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

## f6:suite-ingest+desktop — ingest crate and desktop workspace at `f4b4de1e21`

| Instrument | Command | Result |
|---|---|---|
| Ingest suite | `cargo test --locked -j 8 -p codex-ingest --no-fail-fast -- --test-threads=8` | 167 `test result:` lines; 1,765 passed, 0 failed, 43 ignored; `EXIT=0` (`f6-suite-ingest.log`) |
| Desktop Rust suite | `cargo test --locked -j 8 --no-fail-fast --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8` | 1 `test result:` line (`codex_desktop` unittests); 626 passed, 0 failed, 0 ignored; `EXIT=0` (`f6-suite-desktop.log`) |
| Desktop typecheck | `cd apps/desktop && npm run typecheck` (`tsc --noEmit`) | exit 0, no diagnostics (`f6-desktop-npm.log`) |
| Desktop JS tests | `cd apps/desktop && npm test` (`node scripts/run-tests.mjs`) | 127 of 127 test files passed; exit 0 (`f6-desktop-npm.log`) |

Failing tests: 0 of 1,765 (ingest), 0 of 626 (desktop Rust), 0 of 127 test files (desktop JS).
Re-baselines (A): none. Defects (B): none. Pre-existing (C): none.
No fixture, pinned count or printed sheet value was changed in this step.

Denominators: ingest counts sum the 167 `test result:` lines of the `-p codex-ingest` run; desktop
Rust counts are the single `codex_desktop` unittest binary of the separate `apps/desktop/src-tauri`
workspace; the JS figure counts test files, as `run-tests.mjs` reports them (per-file assertion
counts are printed by each file, not totalled). Logs are condensed to Running/result/exit lines;
the full logs lived on tmpfs scratch.
