---
canonical: true
owner: god-emporer
bundle_id: SD-34
date: 2026-08-26
board: local-file (Hermes board retired 2026-08-01, SD-30 Decision 14a)
---

# SD-34 Kanban

One row per acceptance criterion. **26 rows covering 27 criteria** — the last row carries both remaining closure criteria. A cycle marks its row `complete` from inside the dispatched agent (`workflow-instruction.md §6` step 8), and only when the row-count on its own artifact says so (`decisions.md §4`).

**Status vocabulary:** `not-started` | `in-progress` | `complete` | `blocked-escalated`.

**There is no `returned-to-backlog` and no `deferred`.** A blocker on the Definition of Done is cleared or escalated (`../../governance/blocker-closure-doctrine.md`). `blocked-escalated` pauses the bundle and does not satisfy AT-34-E6-001.

**Row hygiene:** per-cycle narrative goes in `progress.md` and the cycle receipt, **not the Notes column.** Notes here are a pointer, never a story.

| # | Card | Epic | Criterion | Status | Notes (pointer only) |
|---|---|---|---|---|---|
| 1 | `atlas-one-bucket-per-unit` | 1 | AT-34-E1-001 | complete | `artifacts/epic-1-atlas/AT-34-E1-001_cycle_receipt.md` |
| 2 | `atlas-fails-closed` | 1 | AT-34-E1-002 | complete | `artifacts/epic-1-atlas/AT-34-E1-002_cycle_receipt.md` |
| 3 | `missing-tables-and-book-coverage` | 1 | AT-34-E1-003 | not-started | |
| 4 | `shape-engine-boundary-stated` | 1 | AT-34-E1-004 | not-started | |
| 5 | `rename-not-ingested-field` | 1 | AT-34-E1-005 | not-started | |
| 6 | `figure-provenance-gate` | 1 | AT-34-E1-006 | not-started | |
| 7 | `corpus-trap-audit-stage` | 1 | AT-34-E1-007 | not-started | |
| 8 | `build-eight-tables` | 2 | AT-34-E2-001 | not-started | |
| 9 | `tables-fail-closed` | 2 | AT-34-E2-002 | not-started | |
| 10 | `table-build-rate-measured` | 2 | AT-34-E2-003 | not-started | |
| 11 | `bucket-a-zero-both-books` | 2 | AT-34-E2-004 | not-started | |
| 12 | `core-bucket-b-zero` | 3 | AT-34-E3-001 | not-started | |
| 13 | `core-bucket-c-zero` | 3 | AT-34-E3-002 | not-started | |
| 14 | `core-buckets-m-v-d-u-x-zero` | 3 | AT-34-E3-003 | not-started | |
| 15 | `core-step-cost-ledger` | 3 | AT-34-E3-004 | not-started | |
| 16 | `core-rulebook-zero-remaining` | 3 | AT-34-E3-005 | not-started | |
| 17 | `atlas-defects-recorded` | 3 | AT-34-E3-006 | not-started | |
| 18 | `uc-non-a-tail-resolved` | 4 | AT-34-E4-001 | not-started | |
| 19 | `ultimate-campaign-zero-remaining` | 4 | AT-34-E4-002 | not-started | |
| 20 | `second-cost-measurement` | 4 | AT-34-E4-003 | not-started | |
| 21 | `forward-plan-per-book-per-bucket` | 5 | AT-34-E5-001 | not-started | |
| 22 | `capability-register` | 5 | AT-34-E5-002 | not-started | |
| 23 | `power-table-costed` | 5 | AT-34-E5-003 | not-started | |
| 24 | `plan-ordered-single-bucket-flagged` | 5 | AT-34-E5-004 | not-started | |
| 25 | `final-acceptance-scan` | 6 | AT-34-E6-001 | not-started | |
| 26 | `retro-sweep-archdocs-pr` | 6 | AT-34-E6-002 + AT-34-E6-003 | not-started | |

## Gating

```
Epic 1 — Completion Atlas (rows 1-7)          THE DELIVERABLE; gates everything
   |
   +--> Epic 2 — Build 8 of 9 tables (rows 8-11)
            |
            +--> Epic 3 — Core Rulebook to zero (rows 12-17)   deep book, every bucket
            |
            +--> Epic 4 — Ultimate Campaign to zero (rows 18-20)  shallow book, one bucket
                     |
            (3 and 4 both) --> Epic 5 — Price 35 books (rows 21-24)
                                   |
                                   +--> Epic 6 — Closure (rows 25-26)
```

**Epics 3 and 4 are the only pair that could run concurrently** — different books, disjoint corpus subtrees, both gated only on Epic 2. Whether they do is decided after Epic 2 closes by `workflow-instruction.md §4`'s **disjointness check** (the `git diff --name-only` block there): both touch `src/rules_core/` and `src/bin/`, so unless that check proves **file-level** disjointness, they run **sequentially, Core Rulebook first**. If they do run in parallel, each agent gets `isolation: 'worktree'`.

**Everything else is strictly sequential.** Each epic's output is the next one's input: the atlas names the tables, the tables unblock the books, the books measure the rates, the rates price the plan.

**Row 17 (`atlas-defects-recorded`) is the operator's "three more things" guard.** An empty defects file is an excellent result. An absent one is a failure.
