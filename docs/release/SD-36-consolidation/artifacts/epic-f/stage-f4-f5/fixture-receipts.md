# Stage F4–F5 fixture receipts

Fixture protocol (per suite step): every failing test is classified (A) pinned count/status moved
by this batch — re-baselined only with a receipt line here; (B) genuine defect — fixed at the root,
RED test first; (C) unrelated pre-existing — attributed from git. A changed printed sheet value
without a PF1 citation is a STOP.

## f4:suite-root (worktree `sd36/epic-f4-f5` at `2a17695258`)

| what | command | result | receipt |
|---|---|---|---|
| root suite | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` | exit 0; 293 `test result` lines, **6,391 passed, 0 failed**, 27 ignored | `f4-suite-root-test.log` (condensed: Running + result lines) |
| root clippy | `cargo clippy --locked -j 8 --all-targets -- -D warnings` | exit 0, no warnings | `f4-suite-root-clippy.log` |

Before / after. Before: the last full root run on this branch (`f4pre-full-root.log`, after the
F4pre converter step) was 293 result lines, 6,387 passed, 2 failed, 27 ignored. Both failures were
class (A) pins moved by F4pre and were re-baselined in that step with PF1 citations (CRB p.46 Plant,
CRB p.40 Animal; `f4pre-receipt.md` §9). After (this step, HEAD `2a17695258` = F4pre + F4a–F4d):
6,391 passed, 0 failed, 27 ignored. The 6,391 − 6,389 = +2 net new passing tests come from root tests added after that run (F4pre re-run through F4d); denominator is
the sum of the 293 `test result` lines, `grep '^test result:' f4-suite-root-test.log`.

Failing tests this step: **0**. Classified: (A) 0, (B) 0, (C) 0. Re-baselined tests: none.
Fixed defects: none. Printed sheet values changed: none. STOPs: none.
