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

# SD-31 — Corpus Closure Grind, scope

See `README.md` for Purpose/In-scope/Out-of-scope/Dependency-position/Exit-statement — this file adds
the operative figures and the ceiling math this package inherits from SD-30's decisions §43/§44
(re-derived 2026-08-13/14, not re-run at split time; the next cycle that opens a card must re-derive
before claiming, per this program's standing "generated, never hand-maintained" rule).

## Inherited figures at split time (2026-08-14), cited not re-derived by this file

Per `SD-30-class-feature-archetype-bundle/decisions.md §43` (per-kind `grounded`/`done` table) and
`§44`/`acceptance-and-verification.md AT-30-015` (per-kind `done` floors):

| kind | total units | done (2026-08-14) | held | done floor | closure instrument |
|---|---:|---:|---:|---:|---|
| class_feature | 15,472 | 25 (0.2%) | 88 | measured per-class via Epic 1 (this package) | SD-31 Epic 1/2/3 |
| monster | 1,270 | 7 (0.6%) | 1,235 | 1,242 (97.8%, done+held) | SD-31 Epic 4-F1 + SD-30 Epic 0's static/derived rung |
| spell | 2,843 | 47 (1.7%) | 1,235 | 1,282 (45.1%, done+held, `NO_GROUNDING_PROBE`-capped) | SD-31 Epic 4-F2 + SD-30 Epic 0's spell probe |
| race | 103 | 0 (0.0%) | 7 | 103 (100%) — **blocked on SD-32's race chassis** | SD-31 Epic 4-F3, gated on SD-32 Epic 1 |
| race_trait | 3,447 | 266 (7.7%) | 247 | 513 (14.9%, done+held ceiling) *without* chassis; up to 3,447 *with* SD-32's chassis | SD-31 Epic 4-F4, partially gated on SD-32 Epic 1 |

**Cross-SD note on `race`/`race_trait`:** this package's Epic 4-F3/F4 ingest cards can move `race`/
`race_trait` up to the *chassis-blind* ceiling (513 of 3,447 `race_trait` units, 0 of 103 `race`
units — the ~2,894-unit remainder has no modeled chassis, per `SD-30-.../decisions.md §44` lesson 1).
Closing `race`/`race_trait` past that ceiling requires `SD-32-engine-capability-builds/`'s race-chassis
epic; SD-31's ingest cards do not duplicate or block on that work — they run their own ceiling now and
re-run once SD-32 lands the chassis (recorded as a named cross-SD dependency, not a blocker to route
around).

## Combined ceiling (instruments + ingest together), inherited from `SD-30-.../decisions.md §44`

The ceiling via instrument-application alone (SD-30 Epic 0) is 12,919 of 38,521 units (33.5%) —
`done` 3,464 + `held` 9,455 at the 2026-08-13 stamp. This package's ingest work (Epic 3/4/5) is what
closes the remaining `not-started`/`not-ingested` gap (21,303 units) and, together with SD-32's
capability builds (chassis + verdict paths), is what makes the joint program's 100% mandate
(`SD-30-.../decisions.md §45`) reachable rather than merely aspirational. Neither this package nor
SD-30 nor SD-32 claims the full ceiling alone.

## Binding rules this package inherits (copied, not by reference — see `decisions.md` Decision 1)

- The anti-gaming rule (`SD-30-.../decisions.md §50(a)`, originally SD-32 Decision 1).
- The table-sheet doneness doctrine (`SD-30-.../decisions.md §49`).
- The PI-gate hard-block: no ingest card in this package's Epic 3/4/5 claims a book before
  `SD-30-class-feature-archetype-bundle`'s Epic 3 (PI-screening) is `COMPLETE` for that book —
  Epic 3 stays owned by SD-30; this package is gated on it, not a co-owner.
- The concurrency/cloud fan-out protocol (`SD-30-.../decisions.md §47`).

## What this file does not restate

Everything else — the SD-29 operating lessons (raw-remainder splitting, pre-cycle screening, corpus
shape traps, PI-gate discipline), the reach-gate prime rule, cross-book conflict resolution, and the
class-grant boundary with SD-28 — is inherited unchanged from `SD-30-class-feature-archetype-bundle/`
and is not re-derived here; cite the SD-30 decision directly.
