---
canonical: true
owner: god-emporer
bundle_id: SD-34
date: 2026-08-26
board: local-file (Hermes board retired 2026-08-01, SD-30 Decision 14a)
---

# SD-34 Kanban

One row per acceptance criterion. **27 rows covering 28 criteria** — the last row carries both remaining closure criteria. A cycle marks its row `complete` from inside the dispatched agent (`workflow-instruction.md §6` step 8), and only when the row-count on its own artifact says so (`decisions.md §4`).

**Status vocabulary:** `not-started` | `in-progress` | `complete` | `blocked-escalated`.

A cycle that closes part of its population and **names every remaining unit by sub-cause, with
populations that sum exactly** leaves its row at `in-progress` and reports `partial` — the
dispatch continues and a later cycle takes the remainder (`decisions.md §15`). **Needing more
cycles is never `blocked-escalated`.**

**There is no `returned-to-backlog` and no `deferred`.** A blocker on the Definition of Done is cleared or escalated (`../../governance/blocker-closure-doctrine.md`). `blocked-escalated` pauses the bundle and does not satisfy AT-34-E6-001.

**Row hygiene:** per-cycle narrative goes in `progress.md` and the cycle receipt, **not the Notes column.** Notes here are a pointer, never a story.

| # | Card | Epic | Criterion | Status | Notes (pointer only) |
|---|---|---|---|---|---|
| 1 | `atlas-one-bucket-per-unit` | 1 | AT-34-E1-001 | complete | `artifacts/epic-1-atlas/AT-34-E1-001_cycle_receipt.md` |
| 2 | `atlas-fails-closed` | 1 | AT-34-E1-002 | complete | `artifacts/epic-1-atlas/AT-34-E1-002_cycle_receipt.md` |
| 3 | `missing-tables-and-book-coverage` | 1 | AT-34-E1-003 | complete | `artifacts/epic-1-atlas/AT-34-E1-003_cycle_receipt.md` |
| 4 | `shape-engine-boundary-stated` | 1 | AT-34-E1-004 | complete | `artifacts/epic-1-atlas/AT-34-E1-004_cycle_receipt.md` |
| 5 | `rename-not-ingested-field` | 1 | AT-34-E1-005 | complete | `artifacts/epic-1-atlas/AT-34-E1-005_cycle_receipt.md` |
| 6 | `figure-provenance-gate` | 1 | AT-34-E1-006 | complete | `artifacts/epic-1-atlas/AT-34-E1-006_cycle_receipt.md` |
| 7 | `corpus-trap-audit-stage` | 1 | AT-34-E1-007 | complete | `artifacts/epic-1-atlas/AT-34-E1-007_cycle_receipt.md`, `artifacts/epic-1-atlas/AT-34-E1-007_re-verification_receipt.md` |
| 8 | `wiring-class-mismatch-to-zero` | 1 | AT-34-E1-008 | complete | `artifacts/epic-1-atlas/AT-34-E1-008_G1_cycle_receipt.md` .. `_G4_cycle_receipt.md`, verified by `artifacts/epic-1-atlas/AT-34-E1-007_re-verification_receipt.md` |
| 9 | `build-eight-tables` | 2 | AT-34-E2-001 | complete | `artifacts/epic-2-tables/AT-34-E2-001_cycle_receipt.md`, `artifacts/epic-2-tables/AT-34-E2-001_table_transcript.txt` |
| 10 | `tables-fail-closed` | 2 | AT-34-E2-002 | complete | `artifacts/epic-2-tables/fail-closed-proofs.md`, `artifacts/epic-2-tables/AT-34-E2-002_cycle_receipt.md` |
| 11 | `table-build-rate-measured` | 2 | AT-34-E2-003 | complete | `artifacts/epic-2-tables/table-build-rate.json`, `artifacts/epic-2-tables/AT-34-E2-003_cycle_receipt.md` |
| 12 | `bucket-a-zero-both-books` | 2 | AT-34-E2-004 | complete | `artifacts/epic-2-tables/AT-34-E2-004_cycle_receipt.md` |
| 13 | `core-bucket-b-zero` | 3 | AT-34-E3-001 | in-progress | Escalation cleared (`decisions.md §14`, 9 mechanisms, 1006 total). `artifacts/epic-3-core-rulebook/AT-34-E3-001_cycle_receipt.md` — 29/1035 (mechanism: reattribution). `artifacts/epic-3-core-rulebook/AT-34-E3-001_domain_cycle_receipt.md` — 1/1 (mechanism: `domain`). `artifacts/epic-3-core-rulebook/AT-34-E3-001_race_trait_absent_cycle_receipt.md` — 9/9 (mechanism: `race_trait_absent_from_race_traits`). `artifacts/epic-3-core-rulebook/AT-34-E3-001_class_absent_cycle_receipt.md` — 17/17 (mechanism: `class_absent_from_ClassId_ALL_and_book_class_id_enums`). `artifacts/epic-3-core-rulebook/AT-34-E3-001_deity_absent_cycle_receipt.md` — 21/21 (mechanism: `deity_content_absent_from_deity_table_in_core_rulebook`). `artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_cycle_receipt.md` — Cycle 1 6/63, Cycle 2 2/57 (multi-DESC ingest truncation sub-cause), NOT closed (mechanism: `class_feature_option_pool_record_not_held_by_engine`; 55 remain, 5 named sub-causes, see receipt). `artifacts/epic-3-core-rulebook/AT-34-E3-001_companion_absent_cycle_receipt.md` — 72/100 (mechanism: `companion_absent_from_core_rulebook_companion_tables`; Shape 7 book-wide-grant ownership added to `companion_chassis`'s transcriber; 28 remain, 3 named sub-causes — 12 zero-content, 2 monster-class definitions, 14 cross-book familiar-pool rows), NOT closed. `artifacts/epic-3-core-rulebook/AT-34-E3-001_race_trait_race_not_modelled_cycle_receipt.md` — 132/132 (mechanism: `race_trait_race_not_modelled`; `classify()`'s `Kind::RaceTrait` arm gained a `race_trait_generic/` table fallback, reusing `simple_kind_verdict`; corpus-wide side effect 1413→90), CLOSED. `core_rulebook` bucket B (atlas-real partition) 762/6701 remains (`python3 scripts/completion_atlas.py --by-book`), 5 mechanisms fully closed, 2 partially closed (55 of 63, 28 of 100 remain), 2 not started. `artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt.md` — 0/346 (mechanism: `class_feature_owner_matched_by_name_but_record_not_held_by_engine`; re-derived and named an EXACT 7-way sub-cause partition, proven by a committed passing regression test — 143 null-description internal bookkeeping, 121 real engine-effect tokens, 67 catalog-served-but-wiring-class-gated, 6 class-level-scaled phrase, 5 dropped pcgen args, 3 unregenerated multi-DESC branches, 1 bare percent reference; every gate refusing these is pre-existing Decision-7 safety architecture, none a narrow catalog-widening bug — all need real engine wiring or new ingest work), NOT closed, 0 units moved. `core_rulebook` bucket B unchanged at 762/6701; 1 mechanism newly fully named but unstarted (this cycle's 346), 1 unstarted (`class_feature_option_pool_record_with_magnitude_not_held_by_engine`, 333). `artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_with_magnitude_cycle_receipt.md` — 5/333 (mechanism: `class_feature_option_pool_record_with_magnitude_not_held_by_engine`; new `probe_domain_power_effect_wiring` live-probes cleric's 5 real-formula domains, closing `Domain Power ~ {Battle Rage, Destructive Smite, Strength Surge, Touch of Good, Touch of Glory}`; 328 remain, sum-exact 129-group sub-cause partition, largest named: `Domain Power` 56, `Domain Base` 33, `Favored Enemy/Terrain Bonus` 42, `Bardic Performance`/`Draconic Bloodline Choice`/`Secret Lore`/`New Arcana` 39, wizard-school cluster ~34), NOT closed. `core_rulebook` bucket B (atlas-real partition) 757/6701 remains, 5 mechanisms fully closed, 3 partially closed (55 of 63, 28 of 100, 328 of 333 remain), 1 unstarted (`class_feature_owner_matched_by_name_but_record_not_held_by_engine`, 346). `artifacts/epic-3-core-rulebook/AT-34-E3-001_companion_absent_cycle_receipt_2.md` — cycle 2 on `companion_absent_from_core_rulebook_companion_tables`: dispatch instruction to re-derive (not inherit) the 28-unit remainder's judgement, take a narrower fix if one closes it. Re-derivation found none: the 2 monster-class rows confirmed a genuinely different (level-progression) record shape; the 14 familiar-pool rows' true owners (11 familiar creatures) confirmed already shipping as registered `CompanionRecord`s under `beastiary`, not `core_rulebook` — a real cross-book split baked into the actual books, needing Shape 8 (a corpus-wide invariant widening) not a narrow fix. 0/28 closed, judgement confirmed correct with new corpus proof, partition now backed by a committed passing regression test. `core_rulebook` bucket B (atlas-real partition) unchanged at 757/6701 |
| 14 | `core-bucket-c-zero` | 3 | AT-34-E3-002 | not-started | |
| 15 | `core-buckets-m-v-d-u-x-zero` | 3 | AT-34-E3-003 | not-started | |
| 16 | `core-step-cost-ledger` | 3 | AT-34-E3-004 | not-started | |
| 17 | `core-rulebook-zero-remaining` | 3 | AT-34-E3-005 | not-started | |
| 18 | `atlas-defects-recorded` | 3 | AT-34-E3-006 | not-started | |
| 19 | `uc-non-a-tail-resolved` | 4 | AT-34-E4-001 | not-started | |
| 20 | `ultimate-campaign-zero-remaining` | 4 | AT-34-E4-002 | not-started | |
| 21 | `second-cost-measurement` | 4 | AT-34-E4-003 | not-started | |
| 22 | `forward-plan-per-book-per-bucket` | 5 | AT-34-E5-001 | not-started | |
| 23 | `capability-register` | 5 | AT-34-E5-002 | not-started | |
| 24 | `power-table-costed` | 5 | AT-34-E5-003 | not-started | |
| 25 | `plan-ordered-single-bucket-flagged` | 5 | AT-34-E5-004 | not-started | |
| 26 | `final-acceptance-scan` | 6 | AT-34-E6-001 | not-started | |
| 27 | `retro-sweep-archdocs-pr` | 6 | AT-34-E6-002 + AT-34-E6-003 | not-started | |

## Gating

```
Epic 1 — Completion Atlas (rows 1-8)          THE DELIVERABLE; gates everything
   |
   +--> Epic 2 — Build 8 of 9 tables (rows 9-12)
            |
            +--> Epic 3 — Core Rulebook to zero (rows 13-18)   deep book, every bucket
            |
            +--> Epic 4 — Ultimate Campaign to zero (rows 19-21)  shallow book, one bucket
                     |
            (3 and 4 both) --> Epic 5 — Price 35 books (rows 22-25)
                                   |
                                   +--> Epic 6 — Closure (rows 26-27)
```

**Epics 3 and 4 are the only pair that could run concurrently** — different books, disjoint corpus subtrees, both gated only on Epic 2. Whether they do is decided after Epic 2 closes by `workflow-instruction.md §4`'s **disjointness check** (the `git diff --name-only` block there): both touch `src/rules_core/` and `src/bin/`, so unless that check proves **file-level** disjointness, they run **sequentially, Core Rulebook first**. If they do run in parallel, each agent gets `isolation: 'worktree'`.

**Everything else is strictly sequential.** Each epic's output is the next one's input: the atlas names the tables, the tables unblock the books, the books measure the rates, the rates price the plan.

**Row 18 (`atlas-defects-recorded`) is the operator's "three more things" guard.** An empty defects file is an excellent result. An absent one is a failure.
