# SD-31 State, Goals, and Lessons

## State at package creation (2026-08-14, split from SD-30)

This package inherits SD-30's state at the moment of the split, restated for its own scope:

- `class_feature`: 15,472 units, 25 done (0.2%), 88 held. 25 of 28 archetype-bearing classes
  hand-verified (263 wired-able / 475 named slots, 175 mechanisms). 3 classes (Oracle, Arcanist,
  Sorcerer) unmeasured, need the chooser-interaction primitive.
- `monster`/`spell`/`race`/`race_trait`: folded from SD-29's closed corpus-wide lanes, each at a
  *measured* ceiling, not exhausted — see `scope-draft.md`'s inherited-figures table.
- 7 `future_state` books not yet in the engine at all: `occult_adventures`, `adventurers_guide`,
  `mythic_adventures`, `inner_sea_magic`, `inner_sea_temples`, `inner_sea_taverns`, `inner_sea_faiths`.

## Goals

Drive the above to `done` as this package's contribution to the joint SD-30→SD-31→SD-32 100% dashboard
mandate. This is genuinely the largest-volume lane of the three split packages — most of the corpus's
remaining `not-started` mass lives inside this package's scope, not SD-30's (gates/process) or SD-32's
(capability builds).

## Lessons inherited, not re-derived

Every operating lesson `SD-30-class-feature-archetype-bundle/state-goals-and-lessons.md` and
`decisions.md §44` recorded from the SD-29 handoff — raw-remainder splitting, pre-cycle screening,
corpus-shape hard stops (zero-monster books, negated PCC gates), the PI-gate discipline, the
"validate a proxy where it makes its confident claim" standard — carries forward to this package
unchanged. This file does not re-litigate them; read the origin package's file first, per
`README.md`'s "Source STC contents" convention.

## What this package must not do

Per the anti-gaming rule (`decisions.md` Decision 1(a)): no reclassifying a unit into an easier
wiring_class, no loosening a check, no counting `held` as `done`, no widening a bucket definition to
make progress look better. A cycle that finds itself tempted to do any of these to close the gap
between this package's ceiling and the 100% mandate stops and reports instead.
