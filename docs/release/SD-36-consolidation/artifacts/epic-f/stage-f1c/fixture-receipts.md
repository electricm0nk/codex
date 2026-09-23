# Stage F1c — fixture receipts

Protocol: every failing test is classified (A) pinned count/fixture moved by the package change — re-baselined only with a per-line receipt and PF1 citation; (B) genuine defect — fixed at the root, RED test first; (C) unrelated pre-existing — attributed from git.

## f1c:suite-root (root crate), 2026-09-23, at 161ed73de3 on sd36/epic-f1c

| Gate | Command | Result |
|---|---|---|
| Root tests | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` | EXIT=0; 287 test binaries; 6,306 passed / 0 failed / 28 ignored (denominator: all tests in the root crate's 287 test-result lines) |
| Root clippy | `cargo clippy --locked -j 8 --all-targets -- -D warnings` | EXIT=0; zero warnings |

Failing tests: 0 of 6,334 (6,306 passed + 28 ignored). Classification A: 0 re-baselined. Classification B: 0 defects fixed. Classification C: 0. Changed sheet values without a PF1 citation: none. STOPs: none.
