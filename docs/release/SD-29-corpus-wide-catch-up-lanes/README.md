---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01)
date: 2026-08-01
canonical_branch: tranche/9 (operator directive 2026-08-01)
kanban_board: retired (operator directive 2026-08-01) — see kanban.md
companion_to: ./scope-draft.md
mirror_of: ./scope-draft.md
build_version_target: 0.9.<build>
---

# SD-29 — Corpus-Wide Catch-Up Lanes

> **RE-SCOPED CORPUS-WIDE, 2026-08-10 (`decisions.md §38`).** SD-29 is no longer the bestiary
> line. Operator directive, verbatim: *"What I'm really after is establishing lanes that we can use
> to rapidly catch up all the books in parallel — both those we have touched and those we have not
> touched."* Every lane in `epic-breakdown.md` now runs across all 37 in-scope books
> (`../corpus-work-channels.md §10.2`; `beginner_box` is the sole exclusion), touched or untouched,
> superseding the seven-book pin `§34` set and `§37.5` carried forward. The directory itself is
> renamed from `SD-29-bestiary-line-book-ingestion` (via `git mv`, history preserved) so the
> package's own name stops re-asserting the retired boundary. Lanes are grouped in three tiers:
> **Tier 1 — proven-path, day-one parallel** (equipment, feat, spell, equipment_modifier, race,
> class — no mechanism needed, Epic 4); **Tier 2 — mechanism-build-then-sweep, pilot-then-extend**
> (monster+monster_ability chassis, race-trait, companion — Epics 5-7); **Tier 3 — blocked, out of
> scope** (`class_feature`, 40.2% of the corpus, behind the archetype engine — `decisions.md §38.4`).
> See `decisions.md §38` for every re-derived figure and the command behind each one, and
> `risks-and-open-questions.md` for the SD-30 book-list collision this re-scope surfaces but does
> not resolve.

> **RE-CUT, 2026-08-10 (`decisions.md §37`, executing §36).** SD-29 now dispatches by **kind
> lane**, not by per-book epic. `../corpus-work-channels.md`'s channel analysis (three of SD-29's
> four dominant kinds — `monster_ability`, `race_trait`, `companion` — have no working ingest path
> anywhere in the corpus; the fourth, `monster`, has been exercised in exactly one book, and that
> book is SD-22's, not SD-29's) is now built into the package itself: `epic-breakdown.md` carries
> an 11-epic structure (Epic 3 provenance gate + Epics 4-7 kind lanes replacing the retired
> per-book Epics 3-6/11-13), `kanban.md`'s cards match it, and every cross-reference across this
> directory has been updated. Provenance — flagged as "blocking before the first channel runs" in
> `../corpus-work-channels.md §6` — is resolved for OGL/attribution (`docs/governance/
> license-matrix.md`, all 7 books) and gated as a new Epic 3 for PI-screening (unresolved anywhere
> in the pipeline SD-29's lanes write into). See `decisions.md §37` for the full re-cut, every
> re-derived figure, and the command behind each one.

> **⚠️ CONSOLIDATION NOTICE (2026-08-01, historical).** This directory merges two SD-29 packages
> that existed side by side in `docs/release/`, both authored 2026-08-01, both
> claiming a kind of authority, and **disagreeing with each other**:
>
> - **`SD-29-bestiary-2-3-4-5-content-ingestion`** (the frontmatter above, and every
>   file below except `forward-scope-register.md`) — the full ten-file chassis plus
>   `kanban.md`/`progress.md`, scoped to **four** books (Bestiary 2, 3, 4, 5) and
>   carrying `canonical: true` / `status: planning-ready`.
> - **`SD-29-bestiary-line-book-ingestion`** (this directory's original two files:
>   `forward-scope-register.md`, and this README's status banner + §"Chassis files"
>   below) — a scope pass only, explicitly **not** planning-ready, scoped to **seven**
>   books (adding `bestiary_6`, `bonus_bestiary`, `monster_codex` to the four above).
>
> Both bodies of work are preserved here in full — nothing was deleted, only moved
> and (where the corpus figures were disproven) corrected in place with the old
> value left visible. See §"Unresolved: planning-readiness" and §"Unresolved: scope
> width (4 books vs. 7 books)" below — **the operator must resolve both**; this
> consolidation does not pick a winner.

> **⚠️ Inherited status banner, as authored by the scope-pass package:**
> **SCOPE PASS ONLY — NOT PLANNING-READY, NOT EXECUTION-READY.** This package
> (or the parts of it that originated from the scope-pass side) was authored so that
> the work SD-27 routed forward had somewhere to land that was not a retro shard.
> Per `spec-domain-bundle-authoring` v1.2.0 and SD-27's own recorded pitfall —
> *"Don't author the `scope-draft.md` until the operator accepts this register"*
> (`../SD-27-future-state-book-content-ingestion/forward-scope-register.md`,
> "Pitfalls") — the register is meant to be the artifact the operator signs off on
> before anything downstream is authored. **The other package (now merged into this
> directory) did not wait for that sign-off**: it authored the full ten-file chassis
> anyway. That is the authority conflict this consolidation preserves rather than
> resolves — see below.

## RESOLVED: planning-readiness (operator directive 2026-08-02)

**Resolution:** Package B's register is signed off; the existing chassis
(Package A's `scope-draft.md`, `decisions.md`, `epic-breakdown.md`, and the
rest) is planning-ready at seven-book width — the chassis is retained and
widened in place, not re-authored from a blank sign-off cycle. See
`decisions.md §34`.

**Pre-resolution record, preserved below.** The two source packages made
contradictory claims and the original consolidation did not adjudicate
between them:

- **Package A** (`SD-29-bestiary-2-3-4-5-content-ingestion`, now merged into this
  directory's `scope-draft.md`, `decisions.md`, `epic-breakdown.md`, etc.) claims,
  in its own frontmatter (reproduced above): `canonical: true`,
  `status: planning-ready (operator directives 2026-08-01)`,
  `canonical_branch: tranche/9`, `build_version_target: 0.9.<build>`.
- **Package B** (`SD-29-bestiary-line-book-ingestion`, this directory's original
  `forward-scope-register.md` and README banner) claims the opposite: **scope pass
  only, not planning-ready, not execution-ready**, awaiting operator sign-off on the
  register before any of the downstream chassis files (`scope-draft.md`,
  `decisions.md`, `technical-design.md`, `technical-requirements.md`,
  `epic-breakdown.md`, `loop-instruction.md`, `progress.md`, `release-notes.md`,
  `acceptance-and-verification.md`) are even authored.

Both claims are preserved verbatim in this directory (frontmatter above; banner
above; `forward-scope-register.md` unchanged). **The operator must resolve this
contradiction** — this consolidation neither deletes either claim, nor averages
them, nor silently prefers one process over the other. Whichever way it resolves,
note that Package A's corpus-figure errors (below) are now corrected in place
regardless of which package's planning-readiness claim stands.

## RESOLVED: scope width (4 books vs. 7 books) (operator directive 2026-08-02)

**Resolution:** SD-29's scope is the wider seven-book cut — Bestiary 2, 3, 4,
5, 6, Bonus Bestiary, and Monster Codex (Package B's scope). See
`decisions.md §34` for the full ruling, verified per-book shapes, and the
sequential-after-SD-28 launch order.

**Pre-resolution record, preserved below.** Not flagged in the original
consolidation brief, found during this consolidation: the two packages did
not just disagree on numbers, they disagreed on **how many books are in
scope**.

- **Package A** scopes **four** books: Bestiary 2, 3, 4, 5 (see `scope-draft.md`
  "Book list", `decisions.md` Decision 1). Bestiary 6 and Bonus Bestiary appear only
  as a contingent operator-on-request swap-in *for* Bestiary 5, not as committed
  scope (`successor-forward-scope-register.md` C2.1/C3.2, `decisions.md` §"Bestiary
  5 shape resolution").
- **Package B** scopes **seven** books: the four above, plus `bestiary_6`,
  `bonus_bestiary`, and `monster_codex` — all seven cited as Tier-1 per SD-27
  `decisions.md §9` and `epic-breakdown.md:150` (`forward-scope-register.md` §0,
  §1.1).

This consolidation does not narrow or widen either package's stated scope. Both
scope statements are preserved as-authored in their respective files
(`scope-draft.md` for the four-book claim, `forward-scope-register.md` for the
seven-book claim). **The operator must decide which scope width SD-29 actually
carries** before cycle dispatch.

## Corpus-figure corrections applied during consolidation

Package A's `decisions.md` and `scope-draft.md` had already withdrawn their own
"~250-300 monsters each; total ~1,000-1,200" estimate (as of 2026-07-30,
predating this consolidation) after discovering Bestiary 5 has no monster file —
but neither file recorded what the *correct* figure actually was. Package B's
`forward-scope-register.md` §1.3 independently re-derived the exact counts
against the PCGen checkout on 2026-08-01. This consolidation adds those verified
numbers into Package A's files as inline, clearly-marked additions (old value
left visible, not silently overwritten):

| book | old estimate | verified count | file corrected |
|---|---|---:|---|
| bestiary_2 | ~250-300 | **322** | `decisions.md`, `scope-draft.md` |
| bestiary_3 | ~250-300 | **261** | `decisions.md`, `scope-draft.md` |
| bestiary_4 | ~250-300 | **220** | `decisions.md`, `scope-draft.md` |
| bestiary_5 | ~250-300 | **0** (player-options only, confirmed) | `decisions.md`, `scope-draft.md` |
| four-book total | ~1,000-1,200 | **803** (b2+b3+b4; b5 contributes 0) | `decisions.md`, `scope-draft.md` |

The authoritative source for these numbers, and for the full seven-book table
(adding `bestiary_6` **0**, `bonus_bestiary` **14**, `monster_codex` **2**, total
**819** across all seven), remains `forward-scope-register.md` §1.3 — do not
re-derive a third time; cite that file.

## Duplicate `forward-scope-register.md` — renamed, not deleted

Both source packages contained a file named `forward-scope-register.md`, with
**different purposes**, discovered during this consolidation (the dispatching
brief believed only one package had a register):

- **`forward-scope-register.md`** (kept as-is, unchanged) — Package B's register:
  routes work **into** SD-29 from SD-27's retro log, carries the corpus-figure
  corrections (§1.3), and is the operator sign-off gate per its own banner.
- **`successor-forward-scope-register.md`** (renamed from Package A's
  `forward-scope-register.md`) — tracks work **downstream of** SD-29 (SD-30 as
  named successor, DM Toolkit extension retrofit, Bestiary 6 + Bonus Bestiary
  drop-in candidacy). Every internal cross-reference to this file from the other
  merged files (`decisions.md`, `epic-breakdown.md`, `loop-instruction.md`,
  `progress.md`, `risks-and-open-questions.md`, `acceptance-and-verification.md`)
  was updated to the new filename so links stay live.

Neither register was deleted or merged into the other — they answer different
questions (what SD-29 inherits vs. what inherits from SD-29) and both remain
load-bearing for their respective source packages' internal cross-references.

## Purpose

**Current purpose (2026-08-10, `decisions.md §38`):** corpus-wide catch-up, by kind lane, across
all 37 in-scope books — touched and untouched alike. Each lane cycle produces canonical records for
its kind (equipment, feat, spell, equipment_modifier, race, class in Tier 1; monster +
monster_ability chassis, race_trait, companion in Tier 2) that satisfy the reach gate
(`apps/desktop/src-tauri/src/reach_gate.rs`) — a record is not done until it reaches a player
surface. `class_feature` is explicitly out of scope (`decisions.md §38.4`).

**Historical purpose (superseded), preserved for record:** end-to-end content-source ingest for the
bestiary-line books (four per Package A's committed scope-draft, seven per Package B's register —
see "Unresolved: scope width" above).

## Source STC contents (consolidated, 14 files)

- `scope-draft.md` — Package A's committed scope shape, four bestiaries confirmed.
- `decisions.md` — 35 decision headings (1–34, plus 14a) including the
  operator-pinned amendments of 2026-08-01/02, with consolidation-time
  figure corrections inline.
- `loop-instruction.md` — per-cycle procedure; local-file dispatch via
  `kanban.md`/`progress.md`.
- `forward-scope-register.md` — Package B's register: predecessor-deferral
  routing into SD-29, corpus-figure corrections (§1.3), operator sign-off gate.
- `successor-forward-scope-register.md` — Package A's register (renamed):
  successor work depending on SD-29's output.
- `epic-breakdown.md` — **11 epics** × ~3-4 criteria = ~40 criteria (re-cut
  2026-08-10, `decisions.md §37`: Epic 3 provenance gate + Epics 4-7 kind
  lanes replace the retired per-book Epics 3-6/11-13; re-scoped corpus-wide
  2026-08-10, `decisions.md §38`); Closure Epilogue (Epic 11) fires LAST.
- `technical-requirements.md` — pre-loop prerequisites + normative requirements +
  out-of-scope.
- `technical-design.md` — architectural surface for all 37 in-scope books, organized
  by kind lane (Tier 1 proven-path; Tier 2 monster/monster-ability chassis, race-trait,
  companion) rather than per-book.
- `acceptance-and-verification.md` — Given/When/Then per criterion.
- `progress.md` — per-cycle receipt log.
- `release-notes.md` — release-notes template; populated at closure.
- `kanban.md` — local-file work queue (replaces Hermes board).
- `risks-and-open-questions.md` — primary risks + open questions.
- `README.md` — this file.

## Authority surface

Canonical (repo-resident) home:

`docs/release/SD-29-corpus-wide-catch-up-lanes/` (renamed 2026-08-10 via `git mv` from
`SD-29-bestiary-line-book-ingestion`, `decisions.md §38`; that directory was itself a
post-consolidation rename from `docs/release/SD-29-bestiary-2-3-4-5-content-ingestion/` — the
consolidation commit removed that name). See "Unresolved: planning-readiness" above for the live
historical contradiction over whether this package was ready to execute from at consolidation time
(now moot — see `decisions.md §34` for the resolution).

## Objective

**Superseded by the 2026-08-10 corpus-wide re-scope** (`decisions.md §38`, executing over the
2026-08-10 kind-lane re-cut, `decisions.md §37`) — preserved below as historical record of the
consolidation's original per-book framing, current structure follows.

Per-cycle, ingest one canonical record from one bestiary-line book, with the
record reaching a player surface via the reach gate. Package A's epics name
per-monster-block cycles for Bestiary 2-4 and per-race/per-feat/per-companion-mod
cycles for Bestiary 5 (operator-pinned per cycle-0 trap-report output); Package
B's register additionally scopes `bestiary_6`, `bonus_bestiary`, and
`monster_codex` — see "Unresolved: scope width" above.

**Current objective (2026-08-10, `decisions.md §38`):** per lane cycle-batch, ingest one canonical
record of one kind (Tier 1 proven-path: equipment, feat, spell, equipment_modifier, race, class;
Tier 2 mechanism-build-then-sweep: monster+monster_ability chassis, race_trait, companion) from any
of the 37 in-scope books that carries units of that kind — touched or untouched by any prior
bundle — with the record reaching a player surface via the reach gate and clearing the Epic 3
provenance gate. See `epic-breakdown.md` for the full lane structure.

## In scope

**Re-derived per-kind, per `decisions.md §38.1`/`§38.3`** (supersedes both the per-book unit counts
below and the seven-book kind counts in `§37.0`, which sized these lanes to the retired seven-book
boundary):

- **Epic 4 — Proven-Path Content Lanes (Tier 1, day-one parallel)** — equipment (5,064 held / 1,163
  remaining), feat (1,260 / 1,350), spell (1,089 / 1,754), equipment_modifier (768 / 812), race (7 /
  96), class (27 / 158) — corpus-wide, no mechanism needed.
- **Epic 5 — Monster / Monster-Ability Chassis Lane (Tier 2)** — monster (46 held / 1,224 remaining,
  14 books) + monster_ability (0 held / 3,107 remaining, 24 books), merged per
  `../corpus-work-channels.md §9.2`. Pilot: Bonus Bestiary (31 remaining units), then extend
  corpus-wide.
- **Epic 6 — Race-Trait Lane (Tier 2)** — 44 held / 3,412 remaining `race_trait` units across 27
  books; fixes the `classify()` name-coincidence defect alongside the build
  (`../corpus-work-channels.md §9.3`). Pilot: `inner_sea_intrigue` (9 remaining units).
- **Epic 7 — Companion Lane (Tier 2)** — 0 held / 1,683 remaining `companion` units across 17
  books; no path exists anywhere in the corpus. Pilot: `inner_sea_combat` (10 remaining units).
- **Tier 3, out of scope:** `class_feature` — 15,472 units, 40.2% of the corpus, 109 held / 15,363
  remaining, blocked behind the archetype engine (SD-28 `§60`/`§63`). See "Out of scope" below.
- Reach-gate satisfaction for every record ingested (the reach gate is the
  definition of done per `decisions.md §19`).
- Epic 3's PI-screening sweep, wired into each lane's extraction step, before that lane's first
  content commit for any book, corpus-wide (`decisions.md §37.3`, widened `§38.6`).
- Cross-book conflict resolution per `decisions.md §16` (newer book = doctrine,
  older book = errata).
- `monster_codex` is the sole source of the flag that grants
  `Duergar ~ Spell-Like Ability ~ Invisibility`, the project's last open reach
  finding — Epic 5's Monster Codex cycle-batch's to retire (see `forward-scope-register.md` §1.2).

## Out of scope

- **`class_feature`** — 15,472 units, 40.2% of the corpus. Blocked behind the archetype mechanism
  and per-class chassis (SD-28 `§60`/`§63`: sizing cannot be extrapolated from a sample). Successor
  owner: whichever bundle executes `../corpus-work-channels.md §9.1`'s funded per-class
  hand-verification — not yet assigned an SD number. See `decisions.md §38.4`.
- **`beginner_box`** — 19 units, the sole book excluded from the 37-book product per
  `../corpus-work-channels.md §10.2`.
- Real-time execution engines (RNG, opponent state, turn sequencing). Per
  `decisions.md §19`, real-time engines remain out of scope; rules-data engines
  are in scope only when strictly necessary.
- Hermes-board operations. Per `decisions.md §14a`, the board is retired; SD-29
  dispatches via local file.
- The ~40 unrouted engine/UI deferrals in `docs/retro/events/` — real, but not
  SD-29's by any documentary authority (`forward-scope-register.md` Class 3).
- **Not out of scope, flagged instead:** SD-30's sixteen-book list is a live collision with SD-29's
  now-corpus-wide lanes, not a clean boundary — see `risks-and-open-questions.md` and
  `decisions.md §38.5`. This package does not resolve it; the operator does.

## Produced artifacts

- `src/rules_core/rules_tables/<book>/` for every one of the 37 in-scope books — canonical records
  populated lane by lane: Epic 4's proven-path records (equipment, feat, spell,
  equipment_modifier, race, class), Epic 5's monster/monster-ability chassis records, Epic 6's
  race-trait records, Epic 7's companion records.
- `data/corpus/<book>/` — Shape B cache per book, corpus-wide.
- A per-lane PI-screening sweep record per book, per `decisions.md §37.3`/`§38.6` (Epic 3).

## Dependency position

- **Depends on:** SD-22 (closed, Bestiary 1 ingest pipeline; reach-gate
  mechanic); SD-27 (closed, Shape B schema); SD-28 (sequential-before, per `decisions.md §34`);
  `docs/governance/license-matrix.md` (commit `314a7ad9`, provenance evidence Epic 3 cites, all 37
  books).
- **Collides with:** SD-30 (`docs/release/SD-30-class-feature-archetype-bundle/`) — its
  sixteen-book list is now a subset of SD-29's corpus-wide lane scope. Flagged, not resolved — see
  `decisions.md §38.5`, `risks-and-open-questions.md`.
- **Blocks:** None in-cycle; `class_feature`'s successor owner (unassigned SD number, see "Out of
  scope") is the only downstream dependency this package names.

## Exit statement

SD-29 is complete when every one of the 37 in-scope books' records reach a
player surface (via reach gate) across every kind lane (Epic 4-7) that book
carries remaining units of, the Closure Epilogue (Epic 11) fires, and `0.9.<last_build>`
is the post-closure value. **Book-list scope is corpus-wide, resolved 2026-08-10**
(`decisions.md §38`, superseding the seven-book pin `§34` set and `§37.5` carried forward). The
kind-lane partitioning itself (`decisions.md §37`) is unchanged by this re-scope. No cycle work
has run yet.
