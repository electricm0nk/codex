---
canonical: true
owner: god-emporer
bundle_id: SD-35
date: 2026-09-07
board: local-file (Hermes board retired 2026-08-01, SD-30 Decision 14a)
---

# SD-35 Kanban

One row per acceptance criterion. **28 rows covering 29 criteria** — the last row carries both
remaining closure criteria. A cycle marks its row from inside the dispatched agent
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
| 1 | `batch-floor-gate` | 1 | AT-35-E1-001 | not-started | |
| 2 | `content-anchored-citations` | 1 | AT-35-E1-002 | not-started | |
| 3 | `test-families-table-driven` | 1 | AT-35-E1-003 | not-started | |
| 4 | `ratio-row-and-gate-scope` | 1 | AT-35-E1-004 | not-started | |
| 5 | `pcgen-residue-gate` | 1 | AT-35-E1-005 | not-started | |
| 6 | `sheet-rule-converter` | 2 | AT-35-E2-001 | not-started | |
| 7 | `live-evaluator-and-sheet-section` | 2 | AT-35-E2-002 | not-started | |
| 8 | `sheet-complete-status` | 2 | AT-35-E2-003 | not-started | |
| 9 | `token-coverage-ledger` | 2 | AT-35-E2-004 | not-started | |
| 10 | `first-corpus-wide-conversion` | 2 | AT-35-E2-005 | not-started | |
| 11 | `class-feature-b-zero` | 3 | AT-35-E3-001 | not-started | |
| 12 | `other-kinds-b-zero` | 3 | AT-35-E3-002 | not-started | |
| 13 | `bucket-c-zero` | 3 | AT-35-E3-003 | not-started | |
| 14 | `epic-3-rate-ledger` | 3 | AT-35-E3-004 | not-started | |
| 15 | `bucket-m-zero` | 4 | AT-35-E4-001 | not-started | |
| 16 | `bucket-v-oracle-once` | 4 | AT-35-E4-002 | not-started | |
| 17 | `epic-4-rate-ledger` | 4 | AT-35-E4-003 | not-started | |
| 18 | `bucket-a-two-tables` | 5 | AT-35-E5-001 | not-started | |
| 19 | `bucket-d-zero` | 5 | AT-35-E5-002 | not-started | |
| 20 | `buckets-u-z-zero` | 5 | AT-35-E5-003 | not-started | |
| 21 | `bucket-x-choice-filter` | 5 | AT-35-E5-004 | not-started | |
| 22 | `corpus-49438-of-49438` | 5 | AT-35-E5-005 | not-started | |
| 23 | `formula-evaluator-leaves-live` | 6 | AT-35-E6-001 | not-started | |
| 24 | `generators-leave-rules-core` | 6 | AT-35-E6-002 | not-started | |
| 25 | `desktop-and-prose-leave-pcgen` | 6 | AT-35-E6-003 | not-started | |
| 26 | `pcgen-residue-zero` | 6 | AT-35-E6-004 | not-started | |
| 27 | `final-acceptance-scan` | 7 | AT-35-E7-001 | not-started | |
| 28 | `retro-sweep-archdocs-pr` | 7 | AT-35-E7-002 + AT-35-E7-003 | not-started | |

## Gating

```
Epic 1 — Tax cut (rows 1-5)                   cheaper builds; the two counters exist from cycle 1
   |   (row 3 in a worktree beside rows 1-2-5; row 4 after all)
   v
Epic 2 — Sheet rule (rows 6-10)               converter + our schema; live evaluator; first corpus-wide conversion, oracle-checked
   |
   v
Epic 3 — Place and surface (rows 11-14)       B and C to zero
   |
   v
Epic 4 — Resolve and verify (rows 15-17)      M to zero by token family; V through the oracle once
   |
   v
Epic 5 — Residues (rows 18-22)                A, D, U, Z, X; corpus at 49,438 of 49,438
   |
   v
Epic 6 — PCGen exit (rows 23-26)              the old path comes out; residue gate reads zero; oracle agrees before and after
   |
   v
Epic 7 — Closure (rows 27-28)
```

**Epics 2–6 are strictly sequential** — every one of them writes `src/bin/v06_work_inventory.rs`,
`docs/work-inventory.json`, or `src/rules_core/` (`decisions.md §9` L12). Speed comes from
batch size, not lane count.

**Row 10 (`first-corpus-wide-conversion`) is the bundle's measurement**, and its oracle
comparison is the first proof that the rewrite is solid. **Row 26 (`pcgen-residue-zero`) is the
operator's boundary ruling made mechanical** (`decisions.md §11`).
