---
canonical: true
owner: god-emporer
status: planning-ready (SD-32 absorbed, epics re-sequenced, operator ruling 2026-08-15)
date: 2026-08-15
canonical_branch: tranche/10
---

# SD-31 — Local-file Work Queue

Same local-file dispatch convention as `SD-30-class-feature-archetype-bundle/kanban.md` (Hermes board
retired, operator directive 2026-08-01).

**Origin.** Cards `epic-3-measurement` through `epic-8-cloud-fanout` were moved from SD-30's
`epic-4-measurement`, `epic-5-mechanism`, `epic-6-chassis-sweep`, `epic-10-ingest-lanes`,
`epic-11-book-onboarding` and `epic-14-cloud-fanout` rows (operator ruling 2026-08-14, `SD-30
decisions.md §51`). Cards `epic-1-race-chassis` and `epic-2-verdict-paths` were moved from
`SD-32-engine-capability-builds/kanban.md`, which was absorbed and deleted by operator ruling
2026-08-15 (`decisions.md §2`). `epic-0-reachability-audit` and `epic-9-closure` are new in that same
ruling. No card was `IN-FLIGHT` at either move — verified against both source boards immediately
before each.

**Claim-priority order is the table order, top-down**, and it is not the old order: capability
(Epics 1-2) now precedes the lanes that consume it, because 8,524 units — 22.1 % of the board — cannot
reach `done` without it (`decisions.md §2`).

## Status legend

Identical to `SD-30-class-feature-archetype-bundle/kanban.md`'s legend — `READY`, `READY (gated on
...)`, `IN-FLIGHT`, `BLOCKED`, `COMPLETE`. See that file for the full definitions; not reproduced here
to avoid drift between two copies of the same legend text.

## Cards

| ID | Status | Epic | Cycle-type | Claimed-by | Claimed-at | Cycle-id |
|----|--------|------|-----------|------------|------------|----------|
| `epic-0-reachability-audit` | READY | **Order 1 — Reachability Audit (standing gate)** | build `scripts/reachability_audit.py`, prove it can fail, commit a baseline run, assign every dead-end to an epic or propose it to the Structural Exclusion Register | — | — | — |
| `epic-1-race-chassis` | READY | Race Chassis, 100 % mandate | chassis design → per-race (or batch) build with DoD-8 on-screen verification → ceiling release to `epic-6` per race batch | — | — | — |
| `epic-2-verdict-paths` | READY | Verdict-Path Capability, 100 % mandate | hand-labelled ground-truth sample (gate) → classifier build/accept or close-at-F1 → **`ambiguous` dead-end closed or registered** | — | — | — |
| `epic-3-measurement` | READY (per-class; F4 gated on `epic-2`) | Per-Class Archetype Measurement | class inventory + per-class hand-verification + chooser-primitive design + `unknown`-bucket characterization (F4) | — | — | — |
| `epic-4-mechanism` | READY (per-class, gated on `epic-3` clearing the target class) | Archetype Mechanism | supersession-shape wiring per cleared class; chooser-shape wiring once `epic-3`-F3 lands | — | — | — |
| `epic-5-chassis-sweep` | READY (per-class, gated on `epic-3` + `epic-4` for the target class; F3 additionally gated on `epic-2` and `epic-3`-F4) | Per-Class Chassis Sweep | per-class `class_feature` ingest across the 23 in-scope books, reach-gate claim per record | — | — | — |
| `epic-6-ingest-lanes` | READY for F1/F2; **F3 and F4 gated on `epic-1` per race batch** | Corpus-Wide Ingest Lanes, folded from SD-29 | per-kind ingest: F1 `monster`, F2 `spell`, F3 `race`, F4 `race_trait` — each runs the raw-vs-workable split + pre-cycle classifier screen before claiming a book | — | — | — |
| `epic-7-book-onboarding` | READY | Book Onboarding, 100 % mandate | onboard the 7 `future_state` books — PI screen cited clean per book before any record is written | — | — | — |
| `epic-8-cloud-fanout` | READY (per lane shape, after one local proof cycle) | Cloud Fan-Out Protocol (grind **and** capability lanes) | local-proof-then-cloud-scale protocol; local orchestrator owns all `tranche/10` merges; DoD-8 and dashboard-producer work stay local | — | — | — |
| `epic-9-closure` | READY (gated on every other card) | Closure and the 100 % Exit Gate | `epic-0` audit at closing tip → reachable ceiling 100 % or signed register entries → closure receipt + promotion PR (opened, not merged) | — | — | — |

## The two gates that exist because of the merge

`decisions.md §2` inverted an ordering in which the capability builds were scheduled *after* the lanes
depending on them. Both dependencies were cross-package handoffs; both are now **internal hard gates**,
and a cycle claiming across an open one is out of protocol exactly as a PI-gate violation would be:

1. **`epic-1-race-chassis` → `epic-6-ingest-lanes` F3/F4.** No `race` or `race_trait` ingest cycle
   claims a book before Epic 1 has landed a chassis covering the races that book's rows reference. The
   gate opens **per race batch**, not all-or-nothing — Epic 1-F3 names each landed batch here as it
   lands, so ingest starts as soon as any chassis is real.
2. **`epic-2-verdict-paths` → `epic-3-measurement` F4 and `epic-5-chassis-sweep` F3.** No
   `unknown`-bucket characterization or disposal cycle claims before Epic 2 is `COMPLETE`.

## Cross-SD gate discipline (SD-30's PI gate — satisfied, still cited)

`SD-30-class-feature-archetype-bundle`'s `epic-3-pi-gate` closed `COMPLETE` on 2026-08-14 (all of
F1-F4; SD-30 closed the same day, PR #363 open). The hard block it imposed on `epic-5-chassis-sweep`,
`epic-6-ingest-lanes` and `epic-7-book-onboarding` is therefore **discharged at package level** — but
per-book citation is still required: a cycle claiming a book cites SD-30's `progress.md` receipt for
that book's screen, and calls the documented invocation contracts (`SD-30 decisions.md §52.3` for the
blacklist sweep, `§53.5` for the declared-PI reader) from the production ingest path before writing any
generated record. Discharged is not the same as absent: a cycle that writes records without calling the
readers is out of protocol.

## Deferral is not available to a cycle

The phrase "or named a successor for the remainder" is struck from this package (`decisions.md §2`
item 5). A unit leaves the 100 % denominator only through the **Structural Exclusion Register**
(`acceptance-and-verification.md AT-31-100`), which requires the proving command, the named missing
capability with why building it is impossible rather than merely expensive, an `epic-0` audit run
reproducing it, and **operator sign-off**. A cycle may propose an exclusion; only the operator grants
one. An unsigned proposal leaves the unit in the denominator and its epic open.

## Cycle claims (cycle-supervisor protocol)

Identical procedure to `SD-30-class-feature-archetype-bundle/kanban.md`'s "Cycle claims" section
(edit `Status`→`IN-FLIGHT`, `Claimed-by`, `Claimed-at`, `Cycle-id`; append to `progress.md`; on
completion edit `Status`→`COMPLETE` and append the receipt). `epic-4-mechanism` and
`epic-5-chassis-sweep` cycles name the class explicitly in `Cycle-id`; `epic-1-race-chassis` cycles
name the race batch; `epic-6-ingest-lanes` cycles name the kind and book.

## Operator override slot

Operator may add or remove cards directly by editing this file. Cycle dispatch honors the post-edit
state.
