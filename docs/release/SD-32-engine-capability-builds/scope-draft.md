---
canonical: true
owner: god-emporer
status: planning-ready (split from SD-30, operator ruling 2026-08-14)
date: 2026-08-14
canonical_branch: tranche/10
companion_to: ./README.md
mirror_of: ./README.md
build_version_target: 0.10.<build>
---

# SD-32 — Engine Capability Builds, scope

See `README.md` for Purpose/In-scope/Out-of-scope/Dependency-position/Exit-statement. This file adds
the operative figures this package inherits from SD-30's decisions §44/§45 (re-derived 2026-08-13/14,
cited not re-run at split time).

## Inherited figures at split time (2026-08-14)

**Race chassis (Epic 1):** per `SD-30-.../decisions.md §44` lesson 1 (citing
`SD-29-corpus-wide-catch-up-lanes/decisions.md §44.4/§45.1/§49.2`): of the corpus's 3,447 `race_trait`
units, only 553 carry a `TYPE:<Race> Racial Trait` component naming one of the 18 races the engine
currently models. The other ~2,894 belong to races with no modeled chassis at all —
`RaceCorpus::resolve` returns `None` for them. The `race` kind itself is 103 units, 0 done (0.0%), 7
grounded.

**Verdict-path capability (Epic 2):** per `SD-30-.../decisions.md §45` item 2: ~3,547 unmeasurable
units corpus-wide, including a 2,109-unit `ambiguous` bucket. The classifier accuracy rule this epic's
work is bound by (`decisions.md` Decision 1(b) in this package, reproduced from the former SD-32
package's Decision 3 via `SD-30-.../decisions.md §50(c)`) requires a ≥100-unit hand-labelled sample,
stratified across the five wiring classes and at least four kinds, labelled **before** the classifier
is written.

## What this package's capability builds unlock (not this package's own work to run)

- SD-31's Epic 4-F3/F4 (`race`/`race_trait` ingest) — ceiling rises from 513/3,447 `race_trait` toward
  the full population once the chassis lands, and from 0/103 `race` toward 103 (subject to the same
  anti-gaming discipline: a unit is only `done` once it legitimately clears its bar with the new
  chassis, not because the chassis definition was loosened to count more).
- SD-31's Epic 1-F4 (`class_feature`'s `unknown`-bucket genuinely-unreachable subset) — real verdict
  paths give these units a non-placeholder answer, which SD-31's Epic 1-F4/Epic 3-F3 then apply.

## Binding rules this package inherits (copied, not by reference — see `decisions.md` Decision 1)

- The anti-gaming rule (`SD-30-.../decisions.md §50(a)`).
- The classifier accuracy-not-movement rule (`SD-30-.../decisions.md §50(c)`), load-bearing on Epic 2
  specifically — the classifier is accepted or rejected on agreement with a hand-labelled sample, never
  on how many units it moves, and a net-negative movement (units reclassified OUT of a `done`-producing
  cell) is a passing outcome if the sample supports it.
- The table-sheet doneness doctrine (`SD-30-.../decisions.md §49`).
- The concurrency/cloud fan-out protocol (`SD-30-.../decisions.md §47`).

## What this file does not restate

The reach-gate prime rule, DoD-8 on-screen verification discipline, and the class-grant boundary with
SD-28 are inherited unchanged from `SD-30-class-feature-archetype-bundle/` and are not re-derived here.
