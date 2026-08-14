---
canonical: true
owner: god-emporer
status: planning-ready (split from SD-30, operator ruling 2026-08-14)
date: 2026-08-14
canonical_branch: tranche/10 (unchanged, inherited from SD-30)
build_version_target: 0.10.<build> (shared with SD-30/SD-32 for the remainder of the tranche/10 program)
companion_to: ./scope-draft.md, ./decisions.md
split_from: SD-30-class-feature-archetype-bundle (decisions.md §51)
---

# SD-31 — Corpus Closure Grind

**Note on the `SD-31` number.** This package reuses the number `SD-31`, previously held by
`SD-31-pcgen-character-import`. That package was renamed to
`docs/release/SD-33-pcgen-character-import/` on the same operator ruling that created this package
(2026-08-14: "ok, let's split phase 3 and phase 4 into their own SD's. SD-31 and SD-32. Take the
existing SD-31 and rename it to SD-33"). This package is unrelated to PCGen import.

## Purpose

**Split from `SD-30-class-feature-archetype-bundle`, operator ruling 2026-08-14** (verbatim: "ok,
let's split phase 3 and phase 4 into their own SD's. SD-31 and SD-32."). SD-30's launch plan
(`SD-30-class-feature-archetype-bundle/decisions.md §45-§50`) named four phases: Phase 0-2
(remediation, done) and Epic 0 (instrument-apply) plus Epics 1-3/7-9 (gates and process epics) stay in
SD-30; **Phase 3 ("the grind")** — the expensive, high-volume corpus work — moves here. Phase 4
("capability builds") moves to `SD-32-engine-capability-builds/`.

SD-31 is **the grind**: the per-class `class_feature` measurement→mechanism→chassis-sweep chain, the
corpus-wide per-kind ingest lanes folded from SD-29, and the 7-book onboarding pass — the volume work
that makes the corpus-wide `done` count actually move, as opposed to SD-30's remaining gate/process
scope or SD-32's net-new engine-capability builds.

## Source STC contents

Same chassis shape as `SD-30-class-feature-archetype-bundle/` (this package's own origin package) —
see that package's `README.md` "Source STC contents" section for the convention. This package's files:

- `scope-draft.md` — the grind's scope: `class_feature` measurement/mechanism/chassis-sweep,
  corpus-wide per-kind ingest lanes, 7-book onboarding, cloud fan-out protocol.
- `decisions.md` — Decision 1 records the split itself and reproduces the binding rules this
  package's cycles need (anti-gaming rule, table-sheet doctrine, PI-gate cross-SD dependency,
  concurrency/cloud protocol). Decisions 2+ are this package's own, going forward.
- `loop-instruction.md` — short; governed by `SD-30-class-feature-archetype-bundle/loop-instruction.md`
  by reference, with this package's overrides only.
- `epic-breakdown.md` — Epics 1-6 here, moved and renumbered from SD-30's Epic 4/5/6/10/11 and the
  ingest-relevant half of SD-30's former Epic 14 (cloud fan-out) — see the renumber map below.
- `acceptance-and-verification.md` — AT-31-* criteria, including the moved per-kind `done`-floor table
  (formerly SD-30's AT-30-015).
- `kanban.md` — the cards moved from SD-30's kanban, `IN-FLIGHT`/`READY` state preserved.
- `progress.md` — fresh; SD-30's own per-cycle receipts for this scope stay in SD-30's `progress.md`
  as history, cross-referenced here.
- `forward-scope-register.md`, `risks-and-open-questions.md`, `release-notes.md`,
  `state-goals-and-lessons.md`, `technical-requirements.md`, `technical-design.md` — fresh, scoped to
  this package's charter.
- `artifacts/` — the instrument scripts this lane's cycles need (`derive-movable-mass.py` and friends)
  are shared with SD-30/SD-32 by path reference (`../SD-30-class-feature-archetype-bundle/artifacts/`)
  rather than duplicated — SD-30 remains the canonical home for the corpus-wide instrument tooling
  (Epic 0 stays there).

## Epic renumber map (origin: `SD-30-class-feature-archetype-bundle/epic-breakdown.md`)

| SD-31 epic | Moved from (SD-30) | Objective |
|---|---|---|
| Epic 1 (SD31-E1) | SD30-E4 | Per-Class Archetype Measurement (gates Epic 2/3) |
| Epic 2 (SD31-E2) | SD30-E5 | Archetype Mechanism |
| Epic 3 (SD31-E3) | SD30-E6 | Per-Class Chassis Sweep (`class_feature` ingest) |
| Epic 4 (SD31-E4) | SD30-E10 | Corpus-Wide Ingest Lanes (`monster`/`spell`/`race`/`race_trait`) |
| Epic 5 (SD31-E5) | SD30-E11 | Book Onboarding (7 `future_state` books) |
| Epic 6 (SD31-E6) | SD30-E14 | Cloud Fan-Out Protocol (grind lanes only — Epic 4/Epic 5 above) |

SD-30's own Epic 0/1/2/3/7/8/9 are **not** moved — see `SD-30-class-feature-archetype-bundle/README.md`'s
narrowed scope, decisions.md §51.

## Authority surface

Canonical (repo-resident) home: `docs/release/SD-31-corpus-closure-grind/`.

## Objective

Drive `class_feature` (23 books, 15,472 units) and the four folded SD-29 ingest kinds
(`monster`/`spell`/`race`/`race_trait`) to `done`, and onboard the 7 `future_state` books, in
dependency order:

1. **Per-class measurement** (Epic 1, gates Epic 2/3) — extend SD-28/SD-30's hand-verification to every
   remaining `class_feature`-bearing class.
2. **Archetype mechanism** (Epic 2) — the supersession shape (`archetype_claims_slot`) and the
   chooser-interaction shape once Epic 1 designs it.
3. **Per-class chassis sweep** (Epic 3) — the actual `class_feature` ingest, gated per class on 1/2.
4. **Corpus-wide ingest lanes** (Epic 4) — `monster`/`spell`/`race`/`race_trait`, folded from SD-29,
   hard-gated on SD-30's PI-screening gate (cross-SD dependency, see Decision 1 below).
5. **Book onboarding** (Epic 5) — the 7 `future_state` books, same PI gate.
6. **Cloud fan-out** (Epic 6) — the dispatch protocol scaling Epic 4/5 to cloud agents after one local
   proof cycle per lane shape.

## In scope

- `class_feature`, corpus-wide, all 23 books (unchanged list, `SD-30-class-feature-archetype-bundle/README.md`).
- The four SD-29-folded kind lanes: `monster`, `spell`, `race`, `race_trait`.
- The 7 `future_state` books: `occult_adventures`, `adventurers_guide`, `mythic_adventures`,
  `inner_sea_magic`, `inner_sea_temples`, `inner_sea_taverns`, `inner_sea_faiths`.
- The cloud fan-out dispatch protocol for the above (Epic 4/Epic 5 lane shapes only).

## Out of scope

- Instrument-application work (`static`/`derived` `done` rung, consumer-delta probes, `unknown`
  characterization) — stays SD-30's Epic 0.
- Race chassis and verdict-path capability builds — moved to `SD-32-engine-capability-builds/`, not
  this package (Phase 4, not Phase 3).
- Identifier cleanup, pre-launch, PI-screening-gate ownership, version numbering, bundle code review,
  closure epilogue — stay SD-30's Epic 1/2/3/7/8/9. This package's Epic 3/4/5 cycles consume SD-30's
  Epic 3 (PI gate) as a **cross-SD hard dependency** (see Decision 1) but do not own or re-run it.

## Dependency position

- **Depends on:** `SD-30-class-feature-archetype-bundle` Epic 1 (identifier cleanup), Epic 2
  (pre-launch), and — hard-blocking — Epic 3 (PI-screening provenance gate, specifically SD30-E3-F2/F3
  the declared-PI reader and corpus-wide backfill). No SD-31 ingest cycle (Epic 3/4/5) may claim a book
  before that book's declared-PI screen is `COMPLETE` in SD-30.
- **Unblocks:** SD-30's Epic 8 (Bundle Code Review) and Epic 9 (Closure) read this package's
  `done`-floor progress as part of the joint program exit criterion (Decision §51).
- **Blocks:** None in-cycle. Runs concurrently with `SD-32-engine-capability-builds` (file-disjoint —
  this package touches `class_feature`/ingest-lane content; SD-32 touches race-chassis/verdict-path
  engine code) and with SD-30's own Epic 0 (file-disjoint — Epic 0 touches dashboard-producer/instrument
  code).

## Exit statement

SD-31 is complete when: Epic 1 has measured every remaining `class_feature`-bearing class (or named a
successor for the remainder), Epic 2 has wired the supersession/chooser mechanisms for every measured
class, Epic 3 has ingested and reach-gated the `class_feature` records those measurements cleared, Epic
4's four per-kind ingest cards have each reached their measured workable-pool ceiling or named a
successor, Epic 5 has onboarded all 7 `future_state` books (PI-clean), and Epic 6's cloud fan-out
protocol has run at least one local-proof-then-cloud-scale cycle per lane shape it claims a role in. The
**joint SD-30→SD-31→SD-32 100% dashboard mandate** (`SD-30-class-feature-archetype-bundle/decisions.md
§45`, restated by `§51`) is this package's own exit contribution, not a bar it clears alone.
