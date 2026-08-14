---
canonical: true
owner: god-emporer
status: planning-ready (split from SD-30, operator ruling 2026-08-14)
date: 2026-08-14
canonical_branch: tranche/10 (unchanged, inherited from SD-30)
build_version_target: 0.10.<build> (shared with SD-30/SD-31 for the remainder of the tranche/10 program)
companion_to: ./scope-draft.md, ./decisions.md
split_from: SD-30-class-feature-archetype-bundle (decisions.md §51)
---

# SD-32 — Engine Capability Builds

**Disambiguation note (read first).** This package is **unrelated** to
`SD-32-instrument-coverage-and-consumer-wiring`, a different package that was **deleted from the repo
tree on 2026-08-14** (`SD-30-class-feature-archetype-bundle/decisions.md §50`, operator ruling: "as far
as i'm concerned, SD-32 should be deleted and any required work needs to be brought into scope of
SD-30"). That package's content was absorbed into SD-30 before deletion; its git history is preserved
at the pre-deletion SHA `b88b18fa3700125f992e67b0ae29e1d5b70de3c0`. The number `SD-32` was reused, by
the same operator's later direction the same day, to name **this** package instead — the split of
SD-30's Phase 4 ("capability builds"). Any reference elsewhere in this repo to
`SD-32-instrument-coverage-and-consumer-wiring` predates this reuse and names the deleted package, not
this one.

## Purpose

**Split from `SD-30-class-feature-archetype-bundle`, operator ruling 2026-08-14** (verbatim: "ok,
let's split phase 3 and phase 4 into their own SD's. SD-31 and SD-32."). SD-30's launch plan
(`SD-30-class-feature-archetype-bundle/decisions.md §45`) authorized **capability-building, not
descoping** as the route to the 100% dashboard mandate — building the race chassis and real verdict
paths the pre-widening framing had ruled structurally unreachable. That capability-build work is
**Phase 4**, and it moves here. Phase 3 ("the grind" — measurement/mechanism/chassis-sweep, ingest
lanes, book onboarding) moved to `SD-31-corpus-closure-grind/` instead.

SD-32 is **net-new engine capability**, not content ingest: it builds the modeling/classification
capability that Phase 3's ingest work (this package's sibling, SD-31) and SD-30's own instrument work
(Epic 0) then apply. This is the smallest-volume, highest-leverage split package of the three — a
handful of engine changes unblock thousands of otherwise-unreachable units in SD-31's ingest lanes.

## Source STC contents

Same chassis shape as `SD-30-class-feature-archetype-bundle/` — see that package's `README.md` for the
file-set convention. This package's files:

- `scope-draft.md` — the capability-build scope: race chassis, verdict-path classifier.
- `decisions.md` — Decision 1 records the split and reproduces the binding rules this package's cycles
  need (anti-gaming rule, classifier accuracy-not-movement rule, table-sheet doctrine, concurrency/cloud
  protocol). Decisions 2+ are this package's own.
- `loop-instruction.md` — short; governed by `SD-30-class-feature-archetype-bundle/loop-instruction.md`
  by reference, with this package's overrides only.
- `epic-breakdown.md` — Epics 1-2 here, moved and renumbered from SD-30's Epic 12 (race chassis) and
  Epic 13 (verdict-path capability), plus this package's own copy of the cloud fan-out protocol scoped
  to its own lane shapes (moved in part from SD-30's former Epic 14).
- `acceptance-and-verification.md` — AT-32-* criteria, including DoD-8 on-screen verification mandatory
  for the race chassis (unchanged from SD-30's original mandate) and the classifier accuracy-not-
  movement acceptance bar.
- `kanban.md` — the cards moved from SD-30's kanban, `IN-FLIGHT`/`READY` state preserved.
- `progress.md` — fresh; SD-30's own per-cycle receipts for this scope stay in SD-30's `progress.md`
  as history, cross-referenced here.
- `forward-scope-register.md`, `risks-and-open-questions.md`, `release-notes.md`,
  `technical-requirements.md`, `technical-design.md` — fresh, scoped to this package's charter.
- `artifacts/` — this package's own cycle receipts; shares SD-30's instrument tooling by path reference,
  same convention as SD-31.

## Epic renumber map (origin: `SD-30-class-feature-archetype-bundle/epic-breakdown.md`)

| SD-32 epic | Moved from (SD-30) | Objective |
|---|---|---|
| Epic 1 (SD32-E1) | SD30-E12 | Race Chassis, 100% mandate |
| Epic 2 (SD32-E2) | SD30-E13 | Verdict-Path Capability, 100% mandate |
| Epic 3 (SD32-E3) | SD30-E14 (capability-build lane shapes only) | Cloud Fan-Out Protocol, scoped to this package |

SD-30's own Epic 0/1/2/3/7/8/9 are **not** moved — see `SD-30-class-feature-archetype-bundle/README.md`'s
narrowed scope, decisions.md §51. SD-31's Epic 4/5 (ingest lanes, book onboarding) are **not** moved
here either — they stay SD-31's, this package only builds the capability they consume.

## Authority surface

Canonical (repo-resident) home: `docs/release/SD-32-engine-capability-builds/`.

## Objective

Build the two engine capabilities the pre-widening framing (`SD-30-.../state-goals-and-lessons.md`
§2.3) ruled genuinely unreachable, per the 100%-mandate ruling
(`SD-30-class-feature-archetype-bundle/decisions.md §45`):

1. **Race chassis** (Epic 1) — for the ~2,894 chassis-blocked `race_trait` units plus the `race` kind
   itself (103 units, 0% done). `RaceCorpus::resolve` currently returns `None` for races the engine has
   no chassis for; this epic builds the missing chassis rather than accepting that as a ceiling.
2. **Verdict-path capability** (Epic 2) — real, non-placeholder verdict paths for the ~3,547
   unmeasurable units, including the 2,109-unit `ambiguous` bucket, that the pre-correction framing
   treated as a structural floor. Classifier work here is bound by the accuracy-not-movement rule
   (Decision 1(b)) — a verdict path is validated against known-correct cases before it is trusted to
   move counts.
3. **Cloud fan-out** (Epic 3) — the same protocol as SD-31's Epic 6, scoped to this package's own
   build-heavy lane shapes (large-scale chassis rollout once the design proves out locally).

## In scope

- The race chassis engine capability: whatever data model, resolver logic, and corpus-mapping change
  gives `RaceCorpus::resolve` a real answer for the currently-chassis-blind races.
- The verdict-path classifier: resolving `ambiguous` (360+ units per the pre-widening sample) and
  re-examining `display`+`grounded` contradictions, judged on agreement with a hand-labelled sample, not
  on how many units it moves.
- DoD-8 on-screen verification for the race chassis (mandatory, not optional — unchanged from SD-30's
  original Epic 12 mandate).

## Out of scope

- Applying the chassis/classifier to actually ingest `race`/`race_trait` content, or to reclassify
  `class_feature`'s `unknown` bucket — that consumption happens in `SD-31-corpus-closure-grind`'s Epic
  4 and Epic 1-F4 respectively. This package builds the capability; it does not run the resulting ingest
  cycles.
- Instrument-application work (`static`/`derived` `done` rung, consumer-delta probes) — SD-30's Epic 0.
- Identifier cleanup, pre-launch, PI-screening-gate ownership, version numbering, bundle code review,
  closure epilogue — stay SD-30's Epic 1/2/3/7/8/9.

## Dependency position

- **Depends on:** `SD-30-class-feature-archetype-bundle` Epic 1 (identifier cleanup), Epic 2
  (pre-launch). Not gated on SD-30's Epic 3 (PI-screening) — this package builds engine capability, it
  does not ingest corpus content, so the PI gate (which governs content landing in `rules_tables/`) does
  not apply to it directly; a cycle that finds itself writing corpus content rather than engine code has
  drifted out of this package's scope.
- **Unblocks:** `SD-31-corpus-closure-grind`'s Epic 4-F3/F4 (race/race_trait ingest ceiling) once Epic 1
  lands; `SD-31-corpus-closure-grind`'s Epic 1-F4 (`unknown`-bucket disposal for genuinely-unreachable
  units) once Epic 2 lands.
- **Blocks:** None in-cycle. Runs concurrently with SD-31 and with SD-30's Epic 0 (file-disjoint — this
  package touches race-chassis/classifier engine code, not `rules_tables/` content or dashboard-producer
  code).

## Exit statement

SD-32 is complete when: Epic 1 has landed the race chassis with DoD-8 on-screen verification, Epic 2 has
landed a verdict-path classifier validated against a hand-labelled sample per the accuracy-not-movement
rule (Decision 1(b)), and Epic 3 has run at least one local-proof-then-cloud-scale cycle for whichever
of Epic 1/Epic 2's build work is self-contained enough to fan out. The **joint SD-30→SD-31→SD-32 100%
dashboard mandate** (`SD-30-class-feature-archetype-bundle/decisions.md §45`, restated by `§51`) is this
package's own exit contribution, not a bar it clears alone — its contribution is specifically *unlocking*
SD-31's ceiling, not moving `done` counts directly itself.
