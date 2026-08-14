---
canonical: true
owner: god-emporer
status: planning-ready (split from SD-30, operator ruling 2026-08-14)
date: 2026-08-14
canonical_branch: tranche/10
---

# SD-32 — Local-file Work Queue

Same local-file dispatch convention as `SD-30-class-feature-archetype-bundle/kanban.md`. Cards below
are **moved** from that file's `epic-12-race-chassis`, `epic-13-verdict-paths`, and (scoped) part of
`epic-14-cloud-fanout` rows (operator ruling 2026-08-14, split). Both `epic-12-race-chassis` and
`epic-13-verdict-paths` were `READY` (unclaimed, no gate) at split time — verified against SD-30's
`kanban.md` immediately before this move — carried forward unchanged below.

## Status legend

Identical to `SD-30-class-feature-archetype-bundle/kanban.md`'s legend. See that file for the full
definitions.

## Cards

| ID | Status | Epic | Cycle-type | Claimed-by | Claimed-at | Cycle-id |
|----|--------|------|-----------|------------|------------|----------|
| `epic-1-race-chassis` | READY (moved from SD-30 `epic-12-race-chassis`) | Race Chassis, 100% mandate | chassis design + per-race build, DoD-8 on-screen verification mandatory, handoff to SD-31's ingest cards | — | — | — |
| `epic-2-verdict-paths` | READY (moved from SD-30 `epic-13-verdict-paths`) | Verdict-Path Capability, 100% mandate | hand-labelled ground-truth sample (gate) + classifier build/accept or close-at-gate, bound by the accuracy-not-movement rule (`decisions.md` Decision 1(b)) | — | — | — |
| `epic-3-cloud-fanout` | READY (moved from SD-30 `epic-14-cloud-fanout`, scoped to this package's lane shapes) | Cloud Fan-Out Protocol | local-proof-then-cloud-scale protocol for epic-1/epic-2 build work once proven locally; local orchestrator owns all `tranche/10` merges, DoD-8/dashboard-producer work stays local | — | — | — |

## Cycle claims (cycle-supervisor protocol)

Identical procedure to `SD-30-class-feature-archetype-bundle/kanban.md`'s "Cycle claims" section.
`epic-1-race-chassis` cycles name the race (or race batch) explicitly in `Cycle-id`; `epic-2-verdict-
paths` cycles name whether they are claiming F1 (labelling gate) or F2 (classifier build).

## Handoff discipline

A cycle closing an `epic-1-race-chassis` or `epic-2-verdict-paths` slice records, in its completion
receipt, the exact `SD-31-corpus-closure-grind` card it unblocks (Epic 4-F3/F4 for chassis, Epic
1-F4/Epic 3-F3 for verdict paths) — a handoff asserted without the SD-31 side later citing it is not
verified.

## Operator override slot

Operator may add or remove cards directly by editing this file. Cycle dispatch honors the post-edit
state.
