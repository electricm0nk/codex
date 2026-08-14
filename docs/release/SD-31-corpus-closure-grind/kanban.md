---
canonical: true
owner: god-emporer
status: planning-ready (split from SD-30, operator ruling 2026-08-14)
date: 2026-08-14
canonical_branch: tranche/10
---

# SD-31 — Local-file Work Queue

Same local-file dispatch convention as `SD-30-class-feature-archetype-bundle/kanban.md` (Hermes board
retired, operator directive 2026-08-01). Cards below are **moved** from that file's
`epic-4-measurement`, `epic-5-mechanism`, `epic-6-chassis-sweep`, `epic-10-ingest-lanes`,
`epic-11-book-onboarding`, and `epic-14-cloud-fanout` rows (operator ruling 2026-08-14, split). Their
`Status`/`Claimed-by`/`Claimed-at`/`Cycle-id` state at the moment of the split was `READY` for every
row (no card was `IN-FLIGHT` at split time — verified against SD-30's kanban.md immediately before this
move) — carried forward unchanged below.

## Status legend

Identical to `SD-30-class-feature-archetype-bundle/kanban.md`'s legend — `READY`, `READY (gated on
...)`, `IN-FLIGHT`, `BLOCKED`, `COMPLETE`. See that file for the full definitions; not reproduced here
to avoid drift between two copies of the same legend text.

## Cards

| ID | Status | Epic | Cycle-type | Claimed-by | Claimed-at | Cycle-id |
|----|--------|------|-----------|------------|------------|----------|
| `epic-1-measurement` | READY (moved from SD-30 `epic-4-measurement`) | Per-Class Archetype Measurement | class inventory + per-class hand-verification + chooser-primitive design + `unknown`-bucket characterization | — | — | — |
| `epic-2-mechanism` | READY (per-class, gated on epic-1 clearing the target class) (moved from SD-30 `epic-5-mechanism`) | Archetype Mechanism | supersession-shape wiring per cleared class; chooser-shape wiring once epic-1-F3 lands | — | — | — |
| `epic-3-chassis-sweep` | READY (per-class, gated on epic-1 and epic-2 clearing the target class; cross-SD gated on `SD-30-class-feature-archetype-bundle`'s `epic-3-pi-gate`) (moved from SD-30 `epic-6-chassis-sweep`) | Per-Class Chassis Sweep | per-class `class_feature` ingest across the 23 in-scope books, reach-gate claim per record | — | — | — |
| `epic-4-ingest-lanes` | READY (cross-SD gated on `SD-30-class-feature-archetype-bundle`'s `epic-1-identifier`, `epic-2-prelaunch`, `epic-3-pi-gate` per book) (moved from SD-30 `epic-10-ingest-lanes`) | Corpus-Wide Ingest Lanes, folded from SD-29 | per-kind ingest: SD31-E4-F1 `monster`, F2 `spell`, F3 `race`, F4 `race_trait` — each runs the raw-vs-workable split + pre-cycle classifier screen before claiming a book; F3/F4 partially cross-SD gated on `SD-32-engine-capability-builds`'s race chassis for the beyond-553-unit remainder | — | — | — |
| `epic-5-book-onboarding` | READY (cross-SD gated on `SD-30-class-feature-archetype-bundle`'s `epic-3-pi-gate`) (moved from SD-30 `epic-11-book-onboarding`) | Book Onboarding, 100% mandate | onboard the 7 `future_state` books — PI screen clean per book before any record is written | — | — | — |
| `epic-6-cloud-fanout` | READY (moved from SD-30 `epic-14-cloud-fanout`, scoped to this package's lane shapes) | Cloud Fan-Out Protocol | local-proof-then-cloud-scale protocol for epic-4/epic-5 lane shapes; local orchestrator owns all `tranche/10` merges, DoD-8/dashboard-producer work stays local | — | — | — |

## Cycle claims (cycle-supervisor protocol)

Identical procedure to `SD-30-class-feature-archetype-bundle/kanban.md`'s "Cycle claims" section
(edit `Status`→`IN-FLIGHT`, `Claimed-by`, `Claimed-at`, `Cycle-id`; append to `progress.md`; on
completion edit `Status`→`COMPLETE` and append the receipt). `epic-2-mechanism` and
`epic-3-chassis-sweep` cycles name the class explicitly in `Cycle-id`, mirroring SD-30's own
`epic-5-mechanism`/`epic-6-chassis-sweep` convention.

## Cross-SD gate discipline

Every card above that cites a gate on `SD-30-class-feature-archetype-bundle`'s `epic-3-pi-gate` is a
**hard block, not a courtesy note**: a cycle claiming `epic-3-chassis-sweep`, `epic-4-ingest-lanes`, or
`epic-5-book-onboarding` for a specific book must first read SD-30's `kanban.md` and confirm that
book's declared-PI screen (SD30-E3-F2/F3) shows `COMPLETE` for the book in question, citing the SD-30
`progress.md` receipt. A cycle that claims without that citation is out of protocol, identical to how
SD-30's own Epic 6/Epic 10 cycles were gated before the split.

## Operator override slot

Operator may add or remove cards directly by editing this file. Cycle dispatch honors the post-edit
state.
