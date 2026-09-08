---
canonical: true
owner: god-emporer
bundle_id: SD-35
date: 2026-09-07
board: local-file (Hermes board retired 2026-08-01, SD-30 Decision 14a)
---

# SD-35 Kanban

One row per acceptance criterion. **30 rows covering 30 criteria** — row 29 carries both
remaining closure criteria; row 30 is AT-35-E2-005's disposition cycle (`workflow-instruction.md
§5`: one row per extra cycle), not a 31st criterion. A cycle marks its row from inside the dispatched agent
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
| 9 | `sheet-complete-status` | 2 | AT-35-E2-003 | complete | `artifacts/epic-2-sheet-rule/AT-35-E2-003_cycle1_receipt.md`; `docs/retro/events/at-35-e2-003.jsonl` — `a81c2a005c` |
| 10 | `token-coverage-ledger` | 2 | AT-35-E2-004 | complete | `artifacts/epic-2-sheet-rule/AT-35-E2-004_cycle1_receipt.md`; `artifacts/epic-2-sheet-rule/token-coverage.json`; `data/sheet_rules/_tokens.json` — `344f18d1e1` |
| 11 | `first-corpus-wide-conversion` | 2 | AT-35-E2-005 | complete | Against the amended bar — `epic-breakdown.md` `### AT-35-E2-005` amendment 2026-09-08 + `### AT-35-E2-005-DISPOSITION` hand-off table; `decisions.md §16`; `artifacts/epic-2-sheet-rule/AT-35-E2-005-DISPOSITION_handoff.json` (1,404 of 1,404 owned); receipts `AT-35-E2-005_cycle1..4_receipt.md`; `oracle-parity/sheet-parity.json` — `cd3d64e578` |
| 30 | `e2-005-disposition` | 2 | AT-35-E2-005-DISPOSITION (row 11's disposition cycle, `§5` one row per extra cycle) | complete | `artifacts/epic-2-sheet-rule/AT-35-E2-005-DISPOSITION_cycle1_receipt.md`; `AT-35-E2-005-DISPOSITION_handoff.py` / `.json`; `docs/retro/events/at-35-e2-005-disposition.jsonl` |
| 12 | `class-feature-b-zero` | 3 | AT-35-E3-001 | blocked-escalated | `artifacts/epic-3-place-and-surface/AT-35-E3-001_cycle1_receipt.md`; `docs/retro/events/at-35-e3-001.jsonl` (cycle 1 did not start: `scoped=214 … verdict=FAIL_UNDER_FLOOR`, all 214 converter-refused, 24 refused types; §8 → orchestrator re-scope to AT-35-E4-001's 623-unit refused bundle, no Open-blockers entry) |
| 13 | `other-kinds-b-zero` | 3 | AT-35-E3-002 | not-started | |
| 14 | `bucket-c-zero` | 3 | AT-35-E3-003 | not-started | |
| 15 | `epic-3-rate-ledger` | 3 | AT-35-E3-004 | not-started | |
| 16 | `bucket-m-zero` | 4 | AT-35-E4-001 | not-started | |
| 17 | `bucket-v-oracle-once` | 4 | AT-35-E4-002 | not-started | |
| 18 | `epic-4-rate-ledger` | 4 | AT-35-E4-003 | not-started | |
| 19 | `bucket-a-two-tables` | 5 | AT-35-E5-001 | not-started | |
| 20 | `bucket-d-zero` | 5 | AT-35-E5-002 | not-started | |
| 21 | `buckets-u-z-zero` | 5 | AT-35-E5-003 | not-started | |
| 22 | `bucket-x-choice-filter` | 5 | AT-35-E5-004 | not-started | |
| 23 | `corpus-49438-of-49438` | 5 | AT-35-E5-005 | not-started | |
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
