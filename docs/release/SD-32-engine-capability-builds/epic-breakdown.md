---
canonical: true
owner: god-emporer
status: planning-ready (split from SD-30, operator ruling 2026-08-14)
date: 2026-08-14
canonical_branch: tranche/10
build_version_target: 0.10.<build>
companion_to: ./scope-draft.md, ./decisions.md
---

# SD-32 Epic Breakdown

**Moved from `SD-30-class-feature-archetype-bundle/epic-breakdown.md` Epics 12 and 13 (operator ruling
2026-08-14, "split phase 3 and phase 4 into their own SD's"), plus a scoped copy of Epic 14.**
Feature-seed content below is reproduced from the origin epics with IDs renumbered
`SD30-E{12,13}` → `SD32-E{1,2}`; each section's opening line records its SD-30 origin.

## Epic 1 (SD32-E1) — Race Chassis, 100% mandate (moved from SD30-E12)

**Objective:** build the missing race chassis that Decision §44 (citing `SD-29 §44.4/§45.1/§49.2`)
found absent for ~2,894 of the corpus's 3,447 `race_trait` units, plus the `race` kind itself
(103 units, 0% done). That absence was previously ruled structurally unreachable; this epic reverses
that ruling by building the capability rather than accepting the ceiling.

**Derived from:** `SD-30-class-feature-archetype-bundle/decisions.md §45` (the 100%-mandate ruling,
item 1); `decisions.md §44` (the original chassis-absence finding, `RaceCorpus::resolve` returning
`None` without a chassis).

**Verification:** DoD-8 on-screen verification is mandatory for this epic — a chassis claim is not
accepted from static/derived instrument output alone. This is unchanged from SD-30's original mandate
for this epic and is not weakened by the split.

### Feature seeds

#### SD32-E1-F1 — Chassis design: what makes a race "modeled"

Acceptance:

- A direct enumeration (grepped, not estimated) of the ~2,894-unit chassis-blind population's source
  races — which named races the corpus's `race_trait` rows reference that the engine's 18 modeled races
  do not cover.
- A design decision on the chassis shape: does each new race need a full `RaceCorpus` entry (ability
  score modifiers, size, speed, languages, the works) or a narrower "recognized name, traits resolve"
  shim — recorded with its tradeoffs, not assumed.
- The design decision cites `RaceCorpus::resolve`'s current signature and what changes to accommodate
  the new races, without breaking the 18 already-modeled races' resolution.

#### SD32-E1-F2 — Chassis build, per race (or race batch)

Acceptance:

- Each new race's chassis entry lands with the same rigor as the 18 existing ones — no placeholder
  ability scores, no stub trait list.
- `RaceCorpus::resolve` returns a real value (not `None`) for every race this feature seed adds.
- DoD-8 on-screen verification: a character sheet built with the new race shows its real ability score
  modifiers, size, speed — not a default/blank value silently substituted.

#### SD32-E1-F3 — `race`/`race_trait` reachability handoff to SD-31

Acceptance:

- Once a batch of races lands a real chassis, `SD-31-corpus-closure-grind`'s Epic 4-F3/F4 cards are
  notified (via this package's `progress.md` receipt, cited by SD-31's next cycle touching those cards)
  that their ceiling has moved — this package does not run the ingest itself.
- No `race`/`race_trait` unit is marked `done` by this package directly; `done` is SD-31's Epic 4
  ingest-cycle claim, made once the chassis this epic built lets a real record ground.

## Epic 2 (SD32-E2) — Verdict-Path Capability, 100% mandate (moved from SD30-E13)

**Objective:** give every currently-unmeasurable unit a real, non-placeholder verdict — the ~3,547
`unknown`/unmeasurable population, including the 2,109-unit `ambiguous` bucket, that the pre-widening
framing treated as a structural floor.

**Derived from:** `SD-30-class-feature-archetype-bundle/decisions.md §45` (the 100%-mandate ruling,
item 2).

**Constraint:** classifier/instrument work under this epic is bound by `decisions.md` Decision 1(b)
("the wiring-class classifier is accepted on accuracy, not on movement") — a verdict path is validated
against known-correct cases before it is trusted to move counts.

### Feature seeds

#### SD32-E2-F1 — Hand-labelled ground-truth sample (the gate that runs first)

Acceptance:

- At least 100 units, stratified across the five wiring classes and across at least four kinds, are
  hand-labelled from the corpus record — the whole record, not a field-filtered grep.
- Labels are committed. The labeller records the token evidence for each label.
- No classifier code is written before this sample is committed.

#### SD32-E2-F2 — Classifier build and acceptance

Acceptance:

- The classifier's acceptance criterion is its agreement rate against the F1 sample, reported per class
  and per kind, plus its full confusion matrix.
- Movement is reported in both directions — units moved toward AND away from `done`-producing cells.
- If F1's sample shows the current classifier substantially correct and any contradiction rare, F2 is
  **not dispatched** — this epic closes at F1, with the affected units reported "examined, correctly
  classified, left alone." That is `COMPLETE`, per Decision 1(b) item 4.

#### SD32-E2-F3 — `ambiguous`/`unknown` disposition handoff to SD-31

Acceptance:

- Units the classifier resolves to a real wiring class get their new verdict recorded, cited by
  `SD-31-corpus-closure-grind`'s Epic 1-F4/Epic 3-F3 for their next `unknown`-bucket disposition cycle.
- Units the classifier confirms are genuinely unreachable (no chooser code, no chassis, no verdict path
  possible even after this epic's build) are named findings, not silently left in `unknown` without a
  record of why.

## Epic 3 (SD32-E3) — Cloud Fan-Out Protocol, scoped to capability-build lane shapes (moved in part from SD30-E14)

**Objective:** the local-proof-then-cloud-scale protocol, scoped to whichever of this package's own
build work is self-contained enough to fan out (e.g., once Epic 1's chassis design is proven on one
race, rolling out the remaining races in parallel).

**Derived from:** `SD-30-class-feature-archetype-bundle/decisions.md §47`.

**Rules carried into every cycle dispatched under this epic** (identical to `SD-31-corpus-closure-grind
/epic-breakdown.md`'s Epic 6 — the two packages carry independent copies because their lane shapes
never overlap, not because the rules differ):

1. Every cloud agent works its own branch — never two writers on one branch.
2. The local orchestrator owns all merges to `tranche/10`, verified by content, not commit count.
3. DoD-8 on-screen verification and dashboard-producer work stay local — no cloud agent runs either
   (load-bearing here specifically: Epic 1's DoD-8 mandate cannot be satisfied by a cloud agent).

**Not in this epic:** Epic 1/Epic 2's design and first-proof work — this epic is the dispatch protocol
for scaling proven capability-build work, not the design work itself.

## Recommended sequencing

```
E1-F1 (chassis design) -> E1-F2 (chassis build, per race/batch, DoD-8 verified) -> E1-F3 (handoff)
E2-F1 (hand-labelled sample, gate) -> E2-F2 (classifier build+accept, or close at F1) -> E2-F3 (handoff)
E1 and E2 are independent (different engine surfaces: race-chassis data model vs. wiring-class
   classifier) and can run concurrently.
E3 (cloud fan-out) is available to either epic's build work once one race/one classifier-design proves
   out locally.
```

## Completion gate

SD-32 closes when:

- Epic 1 has landed a race chassis for the chassis-blind population (or named a successor for whichever
  races remain unmodelled, with evidence capability-building is genuinely impossible for that subset,
  per the same evidentiary bar `SD-30-.../decisions.md §44` applied to ingest-lane closures) — with
  DoD-8 on-screen verification recorded for every race added.
- Epic 2 has landed a hand-labelled ground-truth sample and either a classifier validated against it or
  a documented decision to close at F1 (units examined, correctly classified, left alone).
- Epic 3 has run at least one local-proof-then-cloud-scale cycle for whichever build work qualified.
- `progress.md` carries the closure receipt; the handoffs to `SD-31-corpus-closure-grind` (Epic 1-F3,
  Epic 2-F3) are cited with the SD-31 receipts that consumed them, not merely asserted as sent.
