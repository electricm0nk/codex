---
canonical: true
owner: god-emporer
bundle_id: SD-33
date: 2026-08-24
board: local-file (Hermes board retired 2026-08-01, SD-30 Decision 14a)
---

# SD-33 Kanban

One row per acceptance criterion. **21 rows.** A cycle marks its row `complete` from inside the dispatched agent (`workflow-instruction.md §6` step 8).

**Status vocabulary:** `not-started` | `in-progress` | `complete` | `blocked-escalated`.

**There is no `returned-to-backlog` and no `deferred`.** A blocker on the Definition of Done is cleared or escalated (`../../governance/blocker-closure-doctrine.md`). `blocked-escalated` means an operator ruling has been requested and the bundle is **paused** — it is not a closure state and it does not satisfy AT-33-E6-001.

**Row hygiene (SD-32 hazard):** keep per-cycle narrative in `progress.md` and the cycle receipt, **not in the Notes column.** SD-32's rows grew into 32KB single physical lines that no editor could safely modify and that broke every naive parser. Notes here are a pointer, never a story.

| # | Card | Epic | Criterion | Status | Notes (pointer only) |
|---|---|---|---|---|---|
| 1 | `box-partition` | 1 | AT-33-E1-001 | complete | `artifacts/epic-1-instruments/AT-33-E1-001_cycle_receipt.md` |
| 2 | `box-fail-closed` | 1 | AT-33-E1-002 | not-started | |
| 3 | `probe-surface-census` | 1 | AT-33-E1-003 | not-started | |
| 4 | `denominator-gate` | 1 | AT-33-E1-004 | not-started | |
| 5 | `oracle-path-a-feasibility` | 2 | AT-33-E2-001 | not-started | |
| 6 | `oracle-character-roundtrip` | 2 | AT-33-E2-002 | not-started | |
| 7 | `oracle-comparison-harness` | 2 | AT-33-E2-003 | not-started | |
| 8 | `oracle-path-ruling` | 2 | AT-33-E2-004 | not-started | |
| 9 | `coverage-gap-rootcause` | 3 | AT-33-E3-001 | not-started | |
| 10 | `coverage-f1` | 3 | AT-33-E3-002 | not-started | |
| 11 | `coverage-f2-f9` | 3 | AT-33-E3-003 | not-started | |
| 12 | `coverage-100-with-denominator` | 3 | AT-33-E3-004 | not-started | |
| 13 | `unknown-rootcause` | 4 | AT-33-E4-001 | not-started | |
| 14 | `unknown-to-zero` | 4 | AT-33-E4-002 | not-started | |
| 15 | `no-effort-named-buckets` | 4 | AT-33-E4-003 | not-started | |
| 16 | `reverify-fixture-verified` | 5 | AT-33-E5-001 | not-started | |
| 17 | `reverify-literal-verified` | 5 | AT-33-E5-002 | not-started | |
| 18 | `disagreement-resolution` | 5 | AT-33-E5-003 | not-started | |
| 19 | `final-acceptance-scan` | 6 | AT-33-E6-001 | not-started | |
| 20 | `retrospective-written-and-cited` | 6 | AT-33-E6-002 | not-started | |
| 21 | `sweep-archdocs-graphify-pr` | 6 | AT-33-E6-003 | not-started | |

## Gating

```
Epic 1 (rows 1-4)
   ├──> Epic 2 (rows 5-8)   ─┐
   ├──> Epic 3 (rows 9-12)   ├─ parallel, worktree-isolated
   └──> Epic 4 (rows 13-15) ─┘
                 Epic 2 ──> Epic 5 (rows 16-18)
        all of the above ──> Epic 6 (rows 19-21)
```

**Rows 5–15 run concurrently.** Every agent in that wave gets `isolation: 'worktree'` (`workflow-instruction.md §3`).
