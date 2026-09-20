---
canonical: true
bundle_id: SD-36
---

# SD-36 Progress

Live cycle-by-cycle progress. Populated as the bundle runs.

---

## Summary

| Epic | Status | Cycles | Completed | Notes |
|---|---|---|---|---|
| B (freeze + retire) | **done** (corrected 2026-09-20 — row was stale at `open`/0 despite the epic being closed since 2026-09-17; see `docs/retro/sd36-retrospective.md` "What did not work") | 7 | 1 of 1 | Closed 2026-09-17 (round 7); 46/46 `verify.sh` PASS |
| E (SD-35 code-review correctness, folded in) | **done** (added to this table 2026-09-20 — had no row) | 2 | 1 of 1 | Closed 2026-09-17 (fix cycle 2); 16/22 findings fixed whole, 3 partial, 4 deferred with retro |
| A (crate wall) | done | 1 | 1 of 1 | Completed 2026-09-19; crate-wall verify stage green; A2 residue-gate `--closure` regression open, see release-notes.md |
| C (bloat cuts) | in progress | 2 | 1.5 of 2 | C1 completed 2026-09-20; C2 sub-criteria C2.3/C2.4/C2.5/C2.6 closed this pass (path helpers tracked, branch-promotion test moved, oracle tests re-run 52/52 green, verify-baselines.env re-synced — `receipts.md`); C2.1/C2.2 (the table-driven rewrite itself, ~75,000 lines / 186 files) still awaiting its own dispatch — its own acceptance command needs correcting first (retrospective Finding 1) |
| D (closure) | in progress | 1 | D1 of 6 items | D1 (architecture docs, full-set rewrite) completed 2026-09-20; D2 (retrospective) and D3 (release notes) written this pass; D4 (graphify), D5 (PR/merge), D6 (worktree sweep) await C2 completion per the epic order in `decisions.md §7` |

---

## Cycle-by-cycle log

(Cycles will be logged here as dispatch proceeds. Each cycle record includes: epic, cycle number, units closed, lines changed, test status, verify.sh result, and any rework needed.)

| Cycle | Epic | Scope | Units | Status | Notes |
|---|---|---|---|---|---|
| 1 | B | Freeze public status at 100%, retire v06_work_inventory/support_state_matrix/reach_gate + desktop bridge, 9 test files, dashboard cron jobs | B1–B9 | green | 2026-09-15 through 2026-09-17; 7 independent-verifier rounds; 46/46 `verify.sh` PASS at round 5 (`receipts/epic-b_receipt.md`) |
| 1 | E | SD-35 code-review correctness: 4 P1 converter findings, 4 engine findings, 2 desktop P1s, 2 gates, 2 desktop P2 findings | 22 findings | green (first pass) | 2026-09-15/16; commits `b69ba965e7`..`2b538ce6a4`; 15/22 fixed whole |
| 2 | E | Independent-verifier fix cycle: 14 findings against the epic-E receipt | fix cycle 1 | green | 2026-09-17; commit `fc64577116`; 16/22 fixed whole cumulative |
| 3 | E | Second independent-verifier fix cycle: 9 findings against fix cycle 1's own result | fix cycle 2 | green | 2026-09-17; commit `4f49ee1c1e`; PC4-1 resolved at source, GATE-02 widened 29→31 |
| 4 | A | Crate wall: path rewrites, ingest baselines, crate-wall verify stage | A1–A11 | green | 2026-09-19; baselines recorded; residue gate scope deferred per A2 incident |
| 5 | C1 | Source refactor: split pilot_compute into 42 submodules, consolidate path helpers, lock clippy warnings | C1.1–C1.5 | green | 2026-09-20; 47 files changed (42 new pilot_compute modules + 2 support modules); largest submodule 6,160 lines |
| 6 | D1 | Architecture-docs full-set rewrite per §10 operator ruling | D1 | green | 2026-09-20; 9 docs updated, 1 deleted, 2 added; `docs/architecture/README.md` "Last verified" header current |
| 7 | D2/D3 | Retrospective written, release notes re-derived and rewritten | D2, D3 | green (docs only, no code touched) | 2026-09-20; `docs/retro/sd36-retrospective.md` new; `release-notes.md` rewritten with re-derived figures and Known follow-ups |

---

