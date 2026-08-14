---
canonical: true
owner: god-emporer
status: planning-ready (split from SD-30, operator ruling 2026-08-14)
date: 2026-08-14
---

# SD-32 Acceptance and Verification

Given/When/Then criteria for this package's own epics. General discipline (anti-gaming rule, table-sheet
doctrine) is inherited from `SD-30-class-feature-archetype-bundle/` via `decisions.md` Decision 1 and
not re-derived here.

## AT-32-001 — Race chassis: DoD-8 on-screen verification mandatory (moved from SD30-E12's standing mandate)

Given a new race chassis entry (Epic 1).

When the chassis lands and a character sheet is built with that race.

Then a real, on-screen character sheet (not a static/derived instrument report alone) shows the race's
true ability score modifiers, size, speed, and languages — DoD-8 verification is not satisfied by a
passing unit test or a green dashboard cell alone.

## AT-32-002 — Race chassis does not break the 18 already-modeled races

Given any chassis-build change under Epic 1.

When the change lands.

Then `RaceCorpus::resolve` still returns the correct value for every one of the 18 previously-modeled
races — a regression test proves this, not an assertion.

## AT-32-003 — Classifier accepted on accuracy against a hand-labelled sample, never on movement (moved from the former SD-32 package's Decision 3, via `decisions.md` Decision 1(b))

Given the verdict-path classifier (Epic 2).

When its output is evaluated for acceptance.

Then:

- The acceptance criterion is agreement rate against a ≥100-unit hand-labelled sample (stratified
  across the five wiring classes and at least four kinds, labelled before the classifier existed),
  reported per class and per kind, with a full confusion matrix.
- Movement is reported in both directions; a net-negative effect on `done` is a passing outcome if the
  sample supports it.
- There is no target count of units moved anywhere in this epic's acceptance. A classifier presented
  with "we moved N units to done" as its primary evidence is rejected pending the sample-agreement
  evidence instead.

## AT-32-004 — Handoff to SD-31 is cited, not merely asserted

Given a completed Epic 1 or Epic 2 slice that unblocks an `SD-31-corpus-closure-grind` card.

When this package's `progress.md` records the completion receipt.

Then the receipt names the specific SD-31 card (`epic-4-ingest-lanes` F3/F4, or `epic-1-measurement`
F4 / `epic-3-chassis-sweep` F3) it unblocks; the handoff is verified once SD-31's own `progress.md`
cites this package's receipt in the cycle that consumes it — a one-sided assertion of "unblocked" is
not sufficient.

## Exit gate checklist

- [ ] AT-32-001 — DoD-8 on-screen verification recorded for every race chassis entry added.
- [ ] AT-32-002 — no regression in the 18 already-modeled races.
- [ ] AT-32-003 — classifier acceptance evidence is sample-agreement, not movement count; confusion
      matrix recorded.
- [ ] AT-32-004 — every handoff to SD-31 cited on both sides.
- [ ] `forward-scope-register.md` reviewed for successor work.
- [ ] `release-notes.md` populated.
