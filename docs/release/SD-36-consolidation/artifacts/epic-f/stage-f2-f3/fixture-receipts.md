# Stage F2/F3 — fixture receipts

## f2f3:suite-root (2026-09-25, branch sd36/epic-f2-f3 @ 18261a3315)

Command: `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` (root crate, full suite).
Per-binary result lines: `f2f3-suite-root.log` (this directory).

| measure | value | denominator |
|---|---|---|
| test binaries run | 291 | 291 `test result` lines |
| tests passed | 6,370 | 6,397 tests collected (6,370 passed + 0 failed + 27 ignored) |
| tests failed | 0 | 6,397 |
| tests ignored | 27 | 6,397 (pre-existing `#[ignore]`, none added this step) |
| cargo exit | 0 | — |
| clippy `--all-targets -D warnings` | exit 0, 0 warnings | root crate |
| `python3 scripts/pcgen_residue_gate.py --check --closure` | PASS (identifier 0, shipped 0 of 70,045 scanned, live 0) | — |

### Classification of failing tests

None failed, so no line needed classifying:

- (A) pinned count/status moved: 0 tests. Nothing re-baselined. The F3 flips from earlier steps
  (prestige mixes now Computed; alone-prestige carrying its game-rule diagnostic) were re-pinned in
  the commits that made them (see f3b..f3c5 receipts; negative-control parity in 18261a3315).
- (B) genuine defect exposed: 0 tests. No fixes made.
- (C) unrelated pre-existing failure: 0 tests.

No printed sheet value changed in this step, so no PF1 citation is needed and there are no STOPs.

## f2f3:suite-ingest (2026-09-25, branch sd36/epic-f2-f3 @ 4066c3a619)

Command: `cargo test --locked -j 8 -p codex-ingest --no-fail-fast -- --test-threads=8` (ingest crate, full suite).
Per-binary result lines: `f2f3-suite-ingest.log` (this directory).

| measure | value | denominator |
|---|---|---|
| test binaries run | 167 | 167 `test result` lines |
| tests passed | 1,763 | 1,806 tests collected (1,763 passed + 0 failed + 43 ignored) |
| tests failed | 0 | 1,806 |
| tests ignored | 43 | 1,806 (pre-existing `#[ignore]`, none added this step) |
| cargo exit | 0 | — |
| `python3 scripts/pcgen_residue_gate.py --check --closure` | PASS (identifier 0, shipped 0 of 70,045 scanned, live 0) | — |

### Classification of failing tests

None failed, so no line needed classifying:

- (A) pinned count/status moved: 0 tests. Nothing re-baselined.
- (B) genuine defect exposed: 0 tests. No fixes made.
- (C) unrelated pre-existing failure: 0 tests.

No printed sheet value changed in this step, so no PF1 citation is needed and there are no STOPs.
