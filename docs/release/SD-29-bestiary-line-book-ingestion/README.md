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

# SD-29 — Bestiary-Line Book Ingestion

> **⚠️ CONSOLIDATION NOTICE (2026-08-01).** This directory merges two SD-29 packages
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

End-to-end content-source ingest for the bestiary-line books (four per Package
A's committed scope-draft, seven per Package B's register — see "Unresolved:
scope width" above). Per-bestiary ingest cycles produce canonical monster (or
player-options, or racial-options for `monster_codex`) records that satisfy the
reach gate (`apps/desktop/src-tauri/src/reach_gate.rs`) — a record is not done
until it reaches a player surface.

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
- `epic-breakdown.md` — 13 epics × ~3-4 criteria = ~46 criteria; Closure
  Epilogue fires LAST.
- `technical-requirements.md` — pre-loop prerequisites + normative requirements +
  out-of-scope.
- `technical-design.md` — architectural surface for the four bestiaries, including
  the Bestiary 5 shape-resolution.
- `acceptance-and-verification.md` — Given/When/Then per criterion.
- `progress.md` — per-cycle receipt log.
- `release-notes.md` — release-notes template; populated at closure.
- `kanban.md` — local-file work queue (replaces Hermes board).
- `risks-and-open-questions.md` — primary risks + open questions.
- `README.md` — this file.

## Authority surface

Canonical (repo-resident) home:

`docs/release/SD-29-bestiary-line-book-ingestion/` (this directory, post-
consolidation, replacing `docs/release/SD-29-bestiary-2-3-4-5-content-ingestion/`
which is removed as part of the consolidation commit). See "Unresolved:
planning-readiness" above for the live contradiction over whether this package
is actually ready to execute from.

## Objective

Per-cycle, ingest one canonical record from one bestiary-line book, with the
record reaching a player surface via the reach gate. Package A's epics name
per-monster-block cycles for Bestiary 2-4 and per-race/per-feat/per-companion-mod
cycles for Bestiary 5 (operator-pinned per cycle-0 trap-report output); Package
B's register additionally scopes `bestiary_6`, `bonus_bestiary`, and
`monster_codex` — see "Unresolved: scope width" above.

## In scope

- **Bestiary 2, 3, 4** — per-monster-block cycles (322 / 261 / 220 verified
  base race-declaration rows respectively; sizing unit: `races.lst` rows,
  not monster stat blocks).
- **Bestiary 5** — player-options cycles (race / feat / companion-mod); confirmed
  0 base monster rows.
- Reach-gate satisfaction for every record ingested (the reach gate is the
  definition of done per `decisions.md §19`).
- Cross-book conflict resolution per `decisions.md §16` (newer book = doctrine,
  older book = errata).
- **Per Package B's register only** (not yet accepted into Package A's
  scope-draft — see "Unresolved: scope width"): `bestiary_6` (0), `bonus_bestiary`
  (14), and `monster_codex` (2, and the sole source of the flag that grants
  `Duergar ~ Spell-Like Ability ~ Invisibility`, the project's last open reach
  finding — see `forward-scope-register.md` §1.2).

## Out of scope

- Bestiary 1 (closed in SD-22 procedurally; **4.1% proven, 42/1,027 units** as measured 2026-08-02 — `decisions.md §35`, not a finished content foundation).
- SD-28's Ultimate books (separate bundle).
- SD-30's Occult + companions (separate bundle).
- Mythic monster appendices (not in any current SD).
- NPC codex (not in any current SD).
- Real-time execution engines (RNG, opponent state, turn sequencing). Per
  `decisions.md §19`, real-time engines remain out of scope; rules-data engines
  are in scope only when strictly necessary.
- Hermes-board operations. Per `decisions.md §14a`, the board is retired; SD-29
  dispatches via local file.
- The ~40 unrouted engine/UI deferrals in `docs/retro/events/` — real, but not
  SD-29's by any documentary authority (`forward-scope-register.md` Class 3).

## Produced artifacts

- `src/rules_core/rules_tables/beastiary2/` — per-monster-block records.
- `src/rules_core/rules_tables/beastiary3/` — per-monster-block records.
- `src/rules_core/rules_tables/beastiary4/` — per-monster-block records.
- `src/rules_core/rules_tables/beastiary5/` — per-race / per-feat /
  per-companion-mod records (gated on cycle-0 trap-report output).
- `data/corpus/beastiary{2,3,4,5}/` — Shape B cache per book.
- Per Package B's wider scope (pending resolution): `data/corpus/bestiary_6/`,
  `data/corpus/bonus_bestiary/`, `data/corpus/monster_codex/`.

## Dependency position

- **Depends on:** SD-22 (closed, Bestiary 1 ingest pipeline; reach-gate
  mechanic — but the pipeline's *output* is 4.1% proven, `decisions.md §35`); SD-27 (closed, Shape B schema).
- **Unblocks:** SD-30 (separate bundle, no SD-29 dependency).
- **Blocks:** None in-cycle; the post-tranche consumer is whatever bundle picks
  up after SD-30.

## Exit statement

SD-29 is complete when each in-scope bestiary-line book's records reach a
player surface (via reach gate), the Closure Epilogue fires, and
`0.9.<last_build>` is the post-closure value. **Whether "in-scope" means four
books or seven is unresolved — see above.** The consolidation of the two source
packages into this directory has landed; no cycle work has run yet.
