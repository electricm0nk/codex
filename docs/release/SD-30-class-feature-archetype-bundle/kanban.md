---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01; re-scoped 2026-08-10)
date: 2026-08-10
canonical_branch: tranche/10 (operator directive 2026-08-01)
build_version_target: 0.10.<build>
---

# SD-30 — Local-file Work Queue (replaces Hermes board `codex-tranche-10`)

Per operator directive 2026-08-01, the Hermes board is retired. SD-30's
work queue is a local-file Markdown table. The supervisor reads this
file at top of each cycle tick to identify the next ready card; the
file-touch partition ensures only one cycle claims a card at a time.

**Re-cut 2026-08-10** (`decisions.md §33-38`). The sixteen per-book cards this
file previously carried are retired — that book list dissolved into SD-29's
corpus-wide scope. Cards now match `epic-breakdown.md`'s 9 dependency-ordered
epics: measurement (epic-4) gates mechanism (epic-5) and chassis-sweep
(epic-6) **per class**, not bundle-wide.

## Status legend

- `READY` — not yet claimed. Cycle can pick up.
- `READY (gated on ...)` — not claimable until every named card is `COMPLETE`. The gate is part of the card's state: a cycle that claims a gated card while its gate is open is out of protocol.
- `READY (per-class, gated on epic-4 for the target class)` — epic-5/epic-6 specific: the card as a
  whole opens once its predecessor epic is under way, but any individual class-scoped cycle inside it
  still needs that class's own epic-4 (and, for epic-6, epic-5) clearance before it can be claimed.
- `IN-FLIGHT` — claimed by a cycle, in progress.
- `BLOCKED` — cycle claims the block, captures the gap, surfaces in `progress.md` as a blocker.
- `COMPLETE` — cycle receipt in `progress.md` closes the card.

## Cards (one row per epic; epic-5/epic-6 dispatch further per-class inside their own card)

Rows are in claim-priority order, top-down, matching `loop-instruction.md`'s
"Epic ordering": Epic 1 first, then Epic 2, then Epic 3, then Epic 4 (which
never fully "completes" in the sense of blocking dispatch — it clears classes
incrementally and epic-5/epic-6 cycles begin per class as soon as their class
is cleared).

| ID | Status | Epic | Cycle-type | Claimed-by | Claimed-at | Cycle-id |
|----|--------|------|-----------|------------|------------|----------|
| `epic-1-identifier` | READY | Code-Side Identifier Cleanup | identifier-discipline audit pass | — | — | — |
| `epic-2-prelaunch` | READY (gated on epic-1) | Operator Pre-Launch | local-file dispatch readiness + cycle-0 trap-report + work-inventory (23-book `class_feature` re-derivation) | — | — | — |
| `epic-3-pi-gate` | READY (gated on epic-1, epic-2) | PI-Screening Provenance Gate | per-class PI-blacklist sweep (SD30-E3-F1) + declared-PI reader wired into class_feature ingest (SD30-E3-F2, `decisions.md §39`) + corpus-wide declared-PI backfill (SD30-E3-F3) + regression gate (SD30-E3-F4) — F2 hard-blocks epic-6, no chassis-sweep cycle may claim a class before F2 is COMPLETE | — | — | — |
| `epic-4-measurement` | READY (gated on epic-1, epic-2) | Per-Class Archetype Measurement | class inventory + per-class hand-verification + chooser-primitive design + `unknown`-bucket characterization | — | — | — |
| `epic-5-mechanism` | READY (per-class, gated on epic-3, epic-4 clearing the target class) | Archetype Mechanism | supersession-shape wiring per cleared class; chooser-shape wiring once epic-4-F3 lands | — | — | — |
| `epic-6-chassis-sweep` | READY (per-class, gated on epic-3, epic-4 and epic-5 clearing the target class) | Per-Class Chassis Sweep | per-class `class_feature` ingest across the 23 in-scope books, reach-gate claim per record | — | — | — |
| `epic-7-version` | READY (gated on epic-1) | Build Version Numbering | first concrete value `0.10.<build>` | — | — | — |
| `epic-8-code-review` | READY (gated on epic-5, epic-6, epic-7) | Bundle Code Review | full-bundle diff review vs. branch point (`decisions.md §26`) | — | — | — |
| `epic-9-closure` | READY (gated on every other card) | Closure Epilogue | tranche promotion PR | — | — | — |

## Retired cards (sixteen-book era, 2026-08-01 to 2026-08-10) — historical record, not claimable

`epic-3-oa` through `epic-18-bd2` (Occult Adventures, Horror Adventures, Mythic Adventures, Monster
Codex, Book of the Damned ×2, the ten Inner Sea modules), plus the old `epic-20-version` /
`epic-21-code-review` / `epic-19-closure` numbering, are retired by `decisions.md §35`. None of these
IDs are claimable; a cycle that finds one of them cited in prior doctrine resolves it to the current
card covering the same functional role (Build Version Numbering -> `epic-7-version`; Bundle Code
Review -> `epic-8-code-review`; Closure -> `epic-9-closure`; every per-book content card -> retired
outright, no successor card, since the underlying kinds moved to SD-29 and `class_feature` is now
tracked per-class inside `epic-6-chassis-sweep`, not per-book).

## Cycle claims (cycle-supervisor protocol)

When a cycle claims a card:

1. Edit the card's `Status` to `IN-FLIGHT`.
2. Edit `Claimed-by` to the cycle's harness identifier.
3. Edit `Claimed-at` to the cycle's ISO-8601 timestamp.
4. Edit `Cycle-id` to the cycle's audit ID (e.g., `SD30-E4-F2-<class>-001`).
5. Append the cycle's per-cycle facts to `progress.md`.
6. On cycle completion, edit `Status` to `COMPLETE` and append the
   completion receipt to `progress.md`.
7. For `epic-5-mechanism` and `epic-6-chassis-sweep`: a cycle claiming a
   per-class slice inside the card names the class explicitly in `Cycle-id`
   and confirms (cites the receipt) that class's `epic-4-measurement`
   clearance before claiming — the card-level `IN-FLIGHT`/`COMPLETE` status
   tracks the epic as a whole; individual class slices are tracked in
   `progress.md`.

## Ordering check (2026-08-13, `decisions.md §41-§42`)

Re-verified: `epic-3-pi-gate` (PI-screening, including the 2026-08-13 declared-PI cards
SD30-E3-F2/F3/F4) still hard-blocks `epic-6-chassis-sweep` in the table above, and `epic-4-measurement`
still gates both `epic-5-mechanism` and `epic-6-chassis-sweep` per class. SD-32's corpus-wide
`static`/`derived` gates (`decisions.md §41`) do not change this order — they are consumed by running
`./scripts/verify.sh` per `AT-30-002`, already a standing per-cycle requirement, not a new card. No
reordering needed.

## Operator override slot

Operator may add or remove cards directly by editing this file. Cycle
dispatch honors the post-edit state.

## Resolution to operator directives

This file is the load-bearing replacement for the Hermes `codex-tranche-10`
board (operator-confirmed 2026-08-01). When a Hermes board card is
referenced from prior doctrine (`decisions.md`, `scope-draft.md`,
`loop-instruction.md`, etc.), the reference resolves to a `kanban.md`
card id at the time of cycle dispatch — for cards retired 2026-08-10, resolve
per the "Retired cards" table above.
