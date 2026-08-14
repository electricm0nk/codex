---
canonical: true
owner: god-emporer
status: planning-ready (operator directive 2026-08-10 -- re-scoped to class_feature/archetype)
date: 2026-08-10
canonical_branch: tranche/10 (unchanged)
kanban_board: retired (operator directive 2026-08-01) — see kanban.md
companion_to: ./scope-draft.md
mirror_of: ./scope-draft.md
build_version_target: 0.10.<build>
---

# SD-30 — `class_feature` / Archetype Bundle

## Purpose

**Re-scoped 2026-08-10** (operator directive; `decisions.md §33`). SD-30 is
no longer a sixteen-book content bundle. SD-29's re-scope to corpus-wide kind
lanes (`SD-29-corpus-wide-catch-up-lanes/decisions.md §38`) claims every kind
corpus-wide **except `class_feature`** — SD-29's own `§38.4` names
`class_feature` as staying out of its scope, funded instead through
`docs/release/corpus-work-channels.md §9.1`'s per-class archetype-measurement effort,
unassigned to any SD number until now. **SD-30 is that assignment.**

SD-30's new scope is the one kind lane SD-29 cannot take:
**`class_feature`, 15,472 units across 23 books, 40.2% of the 38,536-unit
corpus, 109 grounded (0.7%).** The sixteen-book list this package carried
before 2026-08-10 has dissolved — every one of those books is already inside
SD-29's corpus-wide scope for every kind except `class_feature`, and this
bundle now follows the record, not the book, across all 23 books that carry
`class_feature` units (not just the sixteen).

This is a **bundle**, not a lane, because SD-28 `§63` proved the work cannot
be scheduled by extrapolation: four hand-verified classes spanned 5%-70% of
named archetype slots wire-able, with no formula connecting sample to
population. SD-30 inherits that measurement and the larger one that followed
it (`§64`: 25 of 28 archetype-bearing classes hand-verified, 175 mechanisms,
~5,775 lines, two distinct wiring shapes named) as its starting state, not a
cold start — see `decisions.md §34`.

## Source STC contents

- `scope-draft.md` — re-scoped shape: `class_feature` corpus-wide, dependency-ordered epics.
- `decisions.md` — decisions 1-32 (book-bundle era, retained as history) plus the 2026-08-10
  re-scope decisions `§33` onward (this is the operative scope from `§33` forward).
- `loop-instruction.md` — per-cycle procedure; local-file dispatch via `kanban.md`/`progress.md`.
- `state-goals-and-lessons.md` — **read this first.** State at the SD-29→SD-30 handoff
  (2026-08-14), SD-30's goals and honest ceiling, and the retrospective lessons that session paid
  for — including the live hazards a successor inherits.
- `forward-scope-register.md` — successor work depending on SD-30's output; book-specific C2.x
  entries retired (moot — no book list to defer from).
- `epic-breakdown.md` — dependency-ordered epics (measurement gates mechanism gates chassis sweep).
- `technical-requirements.md` — pre-loop prerequisites + normative requirements + out-of-scope.
- `technical-design.md` — architectural surface for `class_feature` ingestion + archetype wiring.
- `acceptance-and-verification.md` — Given/When/Then per criterion.
- `progress.md` — per-cycle receipt log.
- `release-notes.md` — release-notes template; populated at closure.
- `kanban.md` — local-file work queue (replaces Hermes board), re-cut to the new epics.
- `risks-and-open-questions.md` — primary risks + open questions, including the closed SD-29
  collision.
- `artifacts/` — per-cycle receipts + finding logs.

## Authority surface

Canonical (repo-resident) home: `docs/release/SD-30-class-feature-archetype-bundle/` (renamed via
`git mv` 2026-08-10 from `SD-30-occult-and-companion-content-ingestion`, history preserved). The
move-not-copy publish already landed under the old name; the rename does not re-run it.

## Objective

**Widened 2026-08-13** (operator ruling; `decisions.md §43`, correcting `decisions.md §42`). SD-30's
charter is no longer `class_feature`-only. SD-32 (a separate package covering corpus-wide instrument
coverage and consumer wiring) should never have existed as its own bundle — that was a dispatch error
— and its package folds into SD-30, which now owns driving **all kinds, corpus-wide, to `done`** (not
`grounded`, not `ingested` — see `decisions.md §43` for the `grounded` != `done` distinction and the
re-derived per-kind figures). SD-32's already-merged content (the corpus-literal sweep, the
derived-evaluator check, the spell consumer-delta probe, the `wiring_class` fix, the inventory-
determinism fix) is unreverted and stands. The `class_feature` objective below is the superset's first
and largest-funded lane, unchanged and still operative in full; it does not shrink or get
deprioritized by the widening.

Close the collision with SD-29, then execute `class_feature` in dependency order:

1. **Per-class measurement** (gates everything else, per `§63`'s proof that sizing cannot be
   extrapolated) — extend SD-28's 25/28-class hand-verification to the remaining classes across all
   23 `class_feature`-bearing books, resolve the 3 choice-based classes' (Oracle, Arcanist, Sorcerer)
   unproven chooser-interaction wiring shape, and characterize the 2,958-unit `unknown` bucket.
2. **Archetype mechanism** — build out the measured 175-mechanism / ~5,775-line supersession shape
   (`archetype_claims_slot`, proven on Alchemist and Fighter) for the 25 measured classes; design and
   size the chooser-interaction shape for the 3 excluded classes once Epic 4 resolves it.
3. **Per-class chassis sweep** — the per-book `class_feature` ingest cycles across all 23 books,
   scoped and sequenced by what Epic 4's measurement finds wireable, not by a blended estimate.
4. Non-content epics: identifier cleanup, operator pre-launch, the PI-screening provenance gate
   (mirroring SD-29 Epic 3 — SD-30 writes the same `rules_tables/*.rs` pipeline and needs the same
   gate), build version numbering, bundle code review, closure.

## In scope

**Widened 2026-08-13 (`decisions.md §43`):** all kinds, corpus-wide, driven to `done` — `class`,
`class_feature`, `companion`, `equipment`, `equipment_modifier`, `feat`, `monster`, `monster_ability`,
`race`, `race_trait`, `spell` (the live kind roster per `docs/work-inventory.json`, re-derived this
session). This absorbs SD-32's former scope (corpus-wide instrument coverage: the `computed`-bucket
consumer-delta probes not yet built, the `static`/`derived` missing `done` rung, and any further
instrument work) directly — see `scope-draft.md`'s "Widened charter" section for the operative
per-kind figures, honest ceiling, and epic ordering. Everything below this point in this section was
written for the `class_feature`-only era and remains fully in scope, unchanged, as the widened
charter's first and largest-funded lane:

- `class_feature`, corpus-wide: 23 books (`advanced_class_guide`, `advanced_players_guide`,
  `ultimate_psionics`, `ultimate_combat`, `ultimate_magic`, `occult_adventures`, `core_rulebook`,
  `ultimate_wilderness`, `ultimate_intrigue`, `adventurers_guide`, `advanced_race_guide`,
  `pathfinder_unchained`, `horror_adventures`, `inner_sea_combat`, `inner_sea_magic`,
  `book_of_the_damned_volume_2`, `inner_sea_world_guide`, `inner_sea_intrigue`, `monster_codex`,
  `bestiary_6`, `inner_sea_taverns`, `book_of_the_damned_volume_1`, `bestiary_4`).
- Reach-gate satisfaction for every record ingested (prime rule, `decisions.md §18`).
- Cross-book conflict resolution per `decisions.md §16`.
- The four shared classes (Occultist, Spiritualist, Medium, Mesmerist) — SD-30 owns canonical class
  definitions per the class-grant doctrine (`decisions.md §5`, unchanged by the re-scope).
- The archetype-swap primitive (`archetype_resolver::archetype_claims_slot`) and its two wiring
  shapes, inherited whole from SD-28.
- The 2,958-unit `unknown` `class_feature` bucket's characterization (Epic 4) and disposition.

## Out of scope

- **Correction, 2026-08-13 (`decisions.md §43`):** the bullet below ("every other kind... SD-29's
  corpus-wide lanes own them") described the pre-widening scope split and is now only partially true.
  SD-29's per-book *content-ingest* ownership for non-`class_feature` kinds is unchanged — SD-30 does
  not take over SD-29's ingest cycles. What SD-30 now additionally owns is driving those kinds'
  already-`held`/already-`grounded` units to `done` via instrument application (the former SD-32
  territory), which is a different axis from SD-29's ingest lane and does not collide with it. See
  `scope-draft.md`'s "Widened charter" section for the boundary as it now stands.
  - **Correction, 2026-08-13, later same day (`decisions.md §44`):** the correction above is itself
    now superseded in part. The operator ruled "yes, fold the ingest lanes into SD-30 too" — SD-29's
    per-book ingest ownership is **no longer unchanged**. SD-29 is closed
    (`SD-29-corpus-wide-catch-up-lanes/decisions.md §70`) and its ingest lanes had no live owner;
    SD-30 inherits them by default, carrying forward SD-29's operating lessons (raw-remainder
    splitting, pre-cycle screening, corpus-shape hard stops, the PI gate staying hard-blocking). See
    `decisions.md §44` for the full ruling, reasoning, and the new Epic 10 ingest-lane cards this
    creates in `epic-breakdown.md`/`kanban.md`.
- Every other kind's *ingest* — **superseded 2026-08-13, `decisions.md §44`: SD-30 now owns this
  too**, inherited from SD-29's closed corpus-wide lanes. The original claim below is left visible as
  history: SD-29's corpus-wide lanes own that (`SD-29-corpus-wide-catch-up-lanes/decisions.md §38`).
- The former sixteen-book content-ingest scope for kinds other than `class_feature` — dissolved,
  see `decisions.md §35` (collision closure).
- Epic 14's harness widening (`ingested-magnitude` ceiling for `spell`/`equipment`) — not
  `class_feature`-shaped, stays outside this bundle; see `decisions.md §36`.
- Real-time execution engines (RNG, opponent state, turn sequencing) — unchanged, `decisions.md §18`.
- Hermes-board operations — unchanged, `decisions.md §14a`.

## Produced artifacts

- `src/rules_core/rules_tables/<book>/` per-class `class_feature` canonical records, across all 23
  in-scope books (not limited to the four occult/mythic/Inner Sea books the old scope named).
- `archetype_resolver.rs` supersession wiring for the 175 measured mechanisms.
- The chooser-interaction primitive (net-new, if Epic 4/5 fund it) for Oracle/Arcanist/Sorcerer.
- Per-class measurement receipts, one per class, never blended (per `§63`'s standing discipline).

## Dependency position

- **Depends on:** SD-22 (closed, per-book ingest pipeline); SD-27 (closed, Shape B schema +
  license-stripping); SD-28 (published, `§60`/`§63`/`§64` measurement + primitive inherited whole);
  SD-29 (published 2026-08-10 as corpus-wide, cedes `class_feature` to SD-30 at `§38.4`).
- **Gated on:** its own Epic 4 (per-class measurement) reaching enough classes to schedule Epic 6's
  chassis sweep with confidence — not merely sequenced after SD-28/SD-29, genuinely blocked until
  measured. See `decisions.md §37` (launch order).
- **Unblocks:** post-tranche consumer (whatever bundle picks up after `tranche/10`).
- **Blocks:** None in-cycle.

## Exit statement

SD-30 is complete when: the per-class measurement (Epic 4) has covered every `class_feature`-bearing
class across the 23 books (or named its own successor for what remains), the archetype mechanism
(Epic 5) has landed the 175-mechanism supersession shape and resolved or explicitly deferred the
chooser-interaction shape, the per-class chassis sweep (Epic 6) has ingested and reach-gated the
`class_feature` records it scoped, the PI-screening gate has run clean on every touched book, the
Closure Epilogue fires, and `0.10.<last_build>` is the post-closure value.
