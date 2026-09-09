---
canonical: true
owner: god-emporer
bundle_id: SD-35
date: 2026-09-07
board: local-file (Hermes board retired 2026-08-01, SD-30 Decision 14a)
---

# SD-35 Kanban

One row per acceptance criterion. **32 rows covering 30 criteria** — row 29 carries both
remaining closure criteria; row 30 is AT-35-E2-005's disposition cycle and row 31 is Epic 2's
wrap-up gate + correction cycle and row 32 is Epic 3's (`workflow-instruction.md §5`: one row per extra cycle;
`§10` steps 0-3), none of them an additional criterion. A cycle marks its row from inside the dispatched agent
(`workflow-instruction.md §6` step 8), from the mechanical receipt rows, never from effort.

**Status vocabulary:** `not-started` | `in-progress` | `complete` | `blocked-escalated`.

A cycle that closes part of its population and **names the rest by refused token type, with
counts that sum** leaves its row at `in-progress` and reports `partial`; the next cycle takes
the remainder — bundled back up to the 500-unit floor (`decisions.md §2`). **Needing more
cycles is never `blocked-escalated`.**

**There is no `returned-to-backlog` and no `deferred`.** A blocker on the Definition of Done is
cleared or escalated (`../../governance/blocker-closure-doctrine.md`).

**Row hygiene:** per-cycle narrative goes in `progress.md` and the cycle receipt, **not the
Notes column.** Notes here are a pointer, never a story.

| # | Card | Epic | Criterion | Status | Notes (pointer only) |
|---|---|---|---|---|---|
| 1 | `batch-floor-gate` | 1 | AT-35-E1-001 | complete | `artifacts/epic-1-tax-cut/AT-35-E1-001_cycle1_receipt.md` — `1d821cdc8d` |
| 2 | `content-anchored-citations` | 1 | AT-35-E1-002 | complete | `artifacts/epic-1-tax-cut/AT-35-E1-002_cycle1_receipt.md`; `artifacts/epic-1-tax-cut/citation-anchor-proofs.md`; cycle 2 re-verify + stale-pin repair: `artifacts/epic-1-tax-cut/AT-35-E1-002_cycle2_receipt.md` |
| 3 | `test-families-table-driven` | 1 | AT-35-E1-003 | complete | `artifacts/epic-1-tax-cut/AT-35-E1-003_cycle1_receipt.md`; `build-time.json`, `test-list-diff.txt` — `03072aea0c` |
| 4 | `ratio-row-and-gate-scope` | 1 | AT-35-E1-004 | complete | `artifacts/epic-1-tax-cut/AT-35-E1-004_cycle1_receipt.md` — `2bf452b038`; cycle 2 closes the last unscanned SD-35 `.md`: `artifacts/epic-1-tax-cut/AT-35-E1-004_cycle2_receipt.md` — `dd48f73d0f` |
| 5 | `pcgen-residue-gate` | 1 | AT-35-E1-005 | complete | `artifacts/epic-1-tax-cut/AT-35-E1-005_cycle1_receipt.md`; baseline `live_files=260 live_hits=12736` |
| 6 | `sd34-closure-folded` | 1 | AT-35-E1-006 | complete | `artifacts/epic-1-tax-cut/AT-35-E1-006_cycle1_receipt.md`; `docs/retro/sd34-book-completion-retrospective.md`; `artifacts/epic-1-tax-cut/sd34-open-row-map.json` (1,590 of 1,590), `sd34-deferral-dispositions.json` (29 of 29) |
| 7 | `sheet-rule-converter` | 2 | AT-35-E2-001 | complete | `artifacts/epic-2-sheet-rule/AT-35-E2-001_cycle1_receipt.md`; `data/sheet_rules/_refused.json` — `72ad0be010`; cycle 2 re-dispatch re-verifies at HEAD and corrects cycle 1's audit row: `artifacts/epic-2-sheet-rule/AT-35-E2-001_cycle2_receipt.md` |
| 8 | `live-evaluator-and-sheet-section` | 2 | AT-35-E2-002 | complete | `artifacts/epic-2-sheet-rule/AT-35-E2-002_cycle1_receipt.md`; `docs/retro/events/at-35-e2-002.jsonl` — `909bb0837c`; cycle 2 re-dispatch re-verifies at HEAD and corrects cycle 1's fixture line-count figure: `artifacts/epic-2-sheet-rule/AT-35-E2-002_cycle2_receipt.md` |
| 9 | `sheet-complete-status` | 2 | AT-35-E2-003 | complete | `artifacts/epic-2-sheet-rule/AT-35-E2-003_cycle1_receipt.md`; `docs/retro/events/at-35-e2-003.jsonl` — `a81c2a005c`; cycle 2 re-dispatch re-verifies at HEAD and corrects cycle 1's consumer-census after-count: `artifacts/epic-2-sheet-rule/AT-35-E2-003_cycle2_receipt.md` |
| 10 | `token-coverage-ledger` | 2 | AT-35-E2-004 | complete | `artifacts/epic-2-sheet-rule/AT-35-E2-004_cycle1_receipt.md`; `artifacts/epic-2-sheet-rule/token-coverage.json`; `data/sheet_rules/_tokens.json` — `344f18d1e1`; cycle 2 re-dispatch re-verifies at HEAD and corrects cycle 1's batch-floor figure (44 → 7 types ≥500 non-DONE): `artifacts/epic-2-sheet-rule/AT-35-E2-004_cycle2_receipt.md` |
| 11 | `first-corpus-wide-conversion` | 2 | AT-35-E2-005 | complete | Against the amended bar — `epic-breakdown.md` `### AT-35-E2-005` amendment 2026-09-08 + `### AT-35-E2-005-DISPOSITION` hand-off table; `decisions.md §16`; `artifacts/epic-2-sheet-rule/AT-35-E2-005-DISPOSITION_handoff.json` (1,404 of 1,404 owned); receipts `AT-35-E2-005_cycle1..4_receipt.md`; `oracle-parity/sheet-parity.json` — `cd3d64e578`; cycle 5 re-dispatch re-verifies the amended bar at HEAD and corrects the cycle-4 receipt's `cmp`-on-`ours.json` stability test: `artifacts/epic-2-sheet-rule/AT-35-E2-005_cycle5_receipt.md` — `77fa8a0d31` |
| 30 | `e2-005-disposition` | 2 | AT-35-E2-005-DISPOSITION (row 11's disposition cycle, `§5` one row per extra cycle) | complete | `artifacts/epic-2-sheet-rule/AT-35-E2-005-DISPOSITION_cycle1_receipt.md`; `AT-35-E2-005-DISPOSITION_handoff.py` / `.json`; `docs/retro/events/at-35-e2-005-disposition.jsonl`; cycle 2 re-dispatch re-derives the hand-off at HEAD and confirms all five obligations unchanged (`owned_sum=1404 unowned=0`, no discoveries): `artifacts/epic-2-sheet-rule/AT-35-E2-005-DISPOSITION_cycle2_receipt.md` |
| 31 | `epic-2-wrapup-gate` | 2 | Epic 2 wrap-up (`workflow-instruction.md §10` steps 0-3, `§5` one row per extra cycle) — not a criterion | complete | Gate run by the isolated read-only worker at `a542652c5e`: **FAIL, 45 of 48 PASS**, `artifacts/epic-2-sheet-rule/EPIC-2_wrapup_gate_report.md`; `docs/retro/events/at-35-e2-wrapup.jsonl`. Correction cycle fixed all three red stages and the `disk-full` incident-key defect, full gate re-run GREEN: `artifacts/epic-2-sheet-rule/EPIC-2_wrapup_fix_cycle_receipt.md`; `docs/retro/events/at-35-e2-wrapup-fix.jsonl` |
| 12 | `class-feature-b-zero` | 3 | AT-35-E3-001 | complete | `artifacts/epic-3-place-and-surface/AT-35-E3-001_cycle2_receipt.md` — `406003afc3`; `class_feature` B = 0 (`completion_atlas.py --by-kind`). Cycle 1's `blocked-escalated` superseded by the cycle-2 bundle |
| 13 | `other-kinds-b-zero` | 3 | AT-35-E3-002 | complete | `artifacts/epic-3-place-and-surface/AT-35-E3-002_cycle1_receipt.md` — `26bdfa8d5b`; bucket B 2 → 0 in every kind (`completion_atlas.py --by-kind`). Whole-remainder cycle: `cycle_scope_gate.py --min 500` → `scoped=786 verdict=PASS` |
| 14 | `bucket-c-zero` | 3 | AT-35-E3-003 | complete | `artifacts/epic-3-place-and-surface/AT-35-E3-001_cycle2_receipt.md` — `406003afc3`; bucket C 79 → 0, emptied by the same bundled cycle (`completion_atlas.py --check`). Own verification cycle at HEAD, C = 0 in all 19 kinds + SD-34 register C1.8 dispositioned: `artifacts/epic-3-place-and-surface/AT-35-E3-003_cycle1_receipt.md` — `0e0298d7fe` |
| 15 | `epic-3-rate-ledger` | 3 | AT-35-E3-004 | complete | `artifacts/epic-3-place-and-surface/AT-35-E3-004_cycle1_receipt.md` — `9a728d891f` (cycle start `697b7780ea`); `rate-ledger.json` all 5 Epic 3 cycles, one row each, re-verified against their receipts (0 discrepancies); `builds_recorded` 0/3/3/1/0, `pcgen_live_files` 260 throughout |
| 32 | `epic-3-wrapup-gate` | 3 | Epic 3 wrap-up (`workflow-instruction.md §10` steps 0-3, `§5` one row per extra cycle) — not a criterion | complete | Gate run by the isolated read-only worker at `07e29075b4`: **FAIL, 46 of 48 PASS**, `artifacts/epic-3-place-and-surface/EPIC-3_wrapup_gate_report.md`; `docs/retro/events/at-35-e3-wrapup.jsonl`, `docs/retro/events/epic-3-wrapup-gate.jsonl` (all three committed by this correction cycle — the gate worker pushes nothing). Correction cycle fixed both red stages (`site-dashboard-check` by `./scripts/publish-site-dashboard.sh`; `figure-provenance` by rewriting 16 unsourced figure lines — 14 named plus 2 that landed in `AT-35-E4-001_cycle1_receipt.md` after the gate ran) and raised `BASELINE_ROOT_FULL_TESTS` 8724 → 8727 on a derived attribution. Full gate re-run GREEN, **48 of 48 PASS**: `artifacts/epic-3-place-and-surface/EPIC-3_wrapup_fix_cycle_receipt.md`; `docs/retro/events/at-35-e3-wrapup-fix.jsonl` |
| 16 | `bucket-m-zero` | 4 | AT-35-E4-001 | complete | `artifacts/epic-4-resolve-and-verify/AT-35-E4-001_cycle1_receipt.md` — `9bae2cfa1f` (cycle start `07e29075b4`); M = 0 and refused non-DONE = 0 carried from `AT-35-E3-002_cycle1_receipt.md` (`26bdfa8d5b`); this cycle closed the third Evidence clause — 24 mapping rows + 1 head alias took `token-coverage.json` `unmapped_token_types` 25 → 0, degraded records 974 → 603, oracle lines 42/41 → 146/145 with 0 new disagreements |
| 17 | `bucket-v-oracle-once` | 4 | AT-35-E4-002 | in-progress | `artifacts/epic-3-place-and-surface/AT-35-E3-002_cycle1_receipt.md` — V 392 → 0 (`completion_atlas.py --check`); the corpus-wide `scripts/oracle_harness/` run is NOT done (needs a PCGen BatchExporter export) — deferral `1788922132640-at-35-e3-002-ac4da5` |
| 18 | `epic-4-rate-ledger` | 4 | AT-35-E4-003 | not-started | |
| 19 | `bucket-a-two-tables` | 5 | AT-35-E5-001 | complete | `artifacts/epic-3-place-and-surface/AT-35-E3-001_cycle2_receipt.md` — `406003afc3`; `missing_engine_tables.py --check` → `population=0`, bucket A 1 → 0, emptied by the same bundled cycle |
| 20 | `bucket-d-zero` | 5 | AT-35-E5-002 | complete | `artifacts/epic-3-place-and-surface/AT-35-E3-001_cycle2_receipt.md` — `406003afc3`; bucket D 43 → 0, every sub-cause named in `progress.md`, emptied by the same bundled cycle |
| 21 | `buckets-u-z-zero` | 5 | AT-35-E5-003 | complete | `artifacts/epic-3-place-and-surface/AT-35-E3-002_cycle1_receipt.md` — `26bdfa8d5b`; U 202 → 0, Z 19 → 0; `corpus_literal_sweep` examined-count unmoved (48,706 of 51,476, CLEAN) because the `beginner_box` delta is 0 — see `progress.md` |
| 22 | `bucket-x-choice-filter` | 5 | AT-35-E5-004 | in-progress | `artifacts/epic-3-place-and-surface/AT-35-E3-002_cycle1_receipt.md` — X 168 → 0 under `workflow-instruction.md §8`; the desktop per-character choice filter on the level-up IPC is NOT built — deferral `1788922132640-at-35-e3-002-ac4da5` |
| 23 | `corpus-49438-of-49438` | 5 | AT-35-E5-005 | in-progress | `artifacts/epic-3-place-and-surface/AT-35-E3-002_cycle1_receipt.md` — `completion_atlas.py --check` → `DONE=49438 of 49438`, every other bucket 0; `artifacts/epic-5-residues/completion-manifest.json` and the re-derived `capability-register.json` are NOT written |
| 24 | `formula-evaluator-leaves-live` | 6 | AT-35-E6-001 | not-started | |
| 25 | `generators-leave-rules-core` | 6 | AT-35-E6-002 | not-started | |
| 26 | `desktop-and-prose-leave-pcgen` | 6 | AT-35-E6-003 | not-started | |
| 27 | `pcgen-residue-zero` | 6 | AT-35-E6-004 | not-started | |
| 28 | `final-acceptance-scan` | 7 | AT-35-E7-001 | not-started | |
| 29 | `retro-sweep-archdocs-pr` | 7 | AT-35-E7-002 + AT-35-E7-003 | not-started | |

## Gating

```
Epic 1 — Tax cut (rows 1-6)                   cheaper builds; the two counters exist from cycle 1; SD-34's closure folded
   |   (rows 3, 6, and 1-2-5 in three worktrees; row 4 after all)
   v
Epic 2 — Sheet rule (rows 7-11)               converter + our schema; live evaluator; first corpus-wide conversion, oracle-checked
   |
   v
Epic 3 — Place and surface (rows 12-15)       B and C to zero
   |
   v
Epic 4 — Resolve and verify (rows 16-18)      M to zero by token family; V through the oracle once
   |
   v
Epic 5 — Residues (rows 19-23)                A, D, U, Z, X; corpus at 49,438 of 49,438
   |
   v
Epic 6 — PCGen exit (rows 24-27)              the old path comes out; residue gate reads zero; oracle agrees before and after
   |
   v
Epic 7 — Closure epilogue (rows 28-29)
```

**Epics 2–6 are strictly sequential** — every one of them writes `src/bin/v06_work_inventory.rs`,
`docs/work-inventory.json`, or `src/rules_core/` (`decisions.md §9` L12). Speed comes from
batch size, not lane count.

**Row 11 (`first-corpus-wide-conversion`) is the bundle's measurement**, and its oracle
comparison is the first proof that the rewrite is solid. **Row 27 (`pcgen-residue-zero`) is the
operator's boundary ruling made mechanical** (`decisions.md §11`). **Row 6
(`sd34-closure-folded`) is the debt SD-34 left at its merge**, cleared here rather than carried.
