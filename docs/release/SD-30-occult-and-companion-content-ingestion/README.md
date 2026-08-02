---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01)
date: 2026-08-01
canonical_branch: tranche/10 (operator directive 2026-08-01)
kanban_board: retired (operator directive 2026-08-01) — see kanban.md
companion_to: ./scope-draft.md
mirror_of: ./scope-draft.md
build_version_target: 0.10.<build>
---

# SD-30 — Occult + Companion Content Ingestion

## Purpose

End-to-end content-source ingest for the sixteen books on SD-30's scope
(occult + mythic + Monster Codex + Inner Sea World Guide + Inner Sea
series ×9 modules + Book of the Damned ×2). NPC Codex and Planar
Adventures are deferred to `forward-scope-register.md C2.x` per the
2026-08-01 absent-book rule; Occult Origins and Haunted Heroes Handbook
(present in the corpus under `player_companion/`) are deferred there by
explicit operator choice 2026-08-01.
Per-book ingest cycles produce canonical records in
`src/rules_core/rules_tables/<book>/` that satisfy the reach gate
(`apps/desktop/src-tauri/src/reach_gate.rs`) — a record is not done
until it reaches a player surface.

## Source STC contents

- `scope-draft.md` — committed scope shape, sixteen books confirmed.
- `decisions.md` — 31 decisions (numbered 1-31 plus 14a) including the operator-pinned amendments of 2026-08-01 (book list, tranche/10, build 0.10.x, no-Hermes-board, cross-book conflict rule with "recently published takes precident" precedence, reach-gate-DoD doctrine as the prime rule).
- `loop-instruction.md` — per-cycle procedure; local-file dispatch via `kanban.md`/`progress.md`.
- `forward-scope-register.md` — successor work depending on SD-30's output.
- `epic-breakdown.md` — 21 epics (matching `kanban.md`'s 21 cards); Closure fires LAST.
- `technical-requirements.md` — pre-loop prerequisites + normative requirements + out-of-scope.
- `technical-design.md` — architectural surface for the sixteen books.
- `acceptance-and-verification.md` — Given/When/Then per criterion.
- `progress.md` — per-cycle receipt log.
- `release-notes.md` — release-notes template; populated at closure.
- `kanban.md` — local-file work queue (replaces Hermes board).
- `risks-and-open-questions.md` — primary risks + open questions.
- `artifacts/` — per-cycle receipts + finding logs.

## Authority surface

Canonical (repo-resident) home:

`docs/release/SD-30-occult-and-companion-content-ingestion/`. The
move-not-copy publish has landed: this directory IS the source-of-record,
and the workspace source tree was removed on the publish commit per the
move-not-copy doctrine (`forward-scope-register.md` Class 0 anchor;
`acceptance-and-verification.md AT-30-011`).

## Objective

Per-cycle, ingest one canonical record from one book into
`src/rules_core/rules_tables/<book>/`, with the record reaching a player
surface via the reach gate. Each per-book content-source-ingest epic
produces the per-class / per-monster-block / per-psychic-discipline /
per-haunt-block / per-tactic / per-trait / per-region / per-deity /
per-domain / per-spell / per-race / per-temple / per-event / per-faction
cycles named in `scope-draft.md §"Book list"`.

## In scope

- Sixteen Paizo books on scope: Occult Adventures, Horror Adventures, Mythic Adventures, Monster Codex, Book of the Damned ×2 volumes, Inner Sea World Guide, Inner Sea Combat, Inner Sea Faiths, Inner Sea Gods, Inner Sea Magic, Inner Sea Races, Inner Sea Temples, Inner Sea Taverns, Inner Sea Bestiary, Inner Sea Intrigue.
- Reach-gate satisfaction for every record ingested (the reach gate is the definition of done per `decisions.md §18`, the prime rule).
- Cross-book conflict resolution per `decisions.md §16` (newer book = doctrine, older book = errata; cross-bundle precedence for SD-28/SD-29's already-published surfaces).
- The four shared classes (Occultist, Spiritualist, Medium, Mesmerist) that appear in both Ultimate Intrigue (SD-28's territory) and Occult Adventures (SD-30's territory) — SD-30 owns canonical class definitions per the class-grant doctrine, SD-28 references.

## Out of scope

- NPC Codex (deferred — corpus directory absent).
- Planar Adventures (deferred — corpus directory absent).
- Occult Origins (deferred by operator choice 2026-08-01 — present in the corpus at `player_companion/occult_origins`; the 07-30 "absent" finding was a bad check, see `forward-scope-register.md C2.3`).
- Haunted Heroes Handbook (deferred by operator choice 2026-08-01 — present in the corpus at `player_companion/haunted_heroes_handbook`, see `forward-scope-register.md C2.4`).
- Real-time execution engines (RNG, opponent state, turn sequencing). Per `decisions.md §18`, real-time engines remain out of scope; rules-data engines are in scope only when strictly necessary.
- Hermes-board operations. The board is retired; SD-30 dispatches via local file per `decisions.md §14a`.

## Produced artifacts

- `src/rules_core/rules_tables/occult_adventures/` — per-class / per-monster-block / per-psychic-discipline records.
- `src/rules_core/rules_tables/horror_adventures/` — per-monster-block / per-haunt-block / per-corruption-mechanic records.
- `src/rules_core/rules_tables/mythic_adventures/` — per-mythic-path / per-monster-block records.
- `src/rules_core/rules_tables/monster_codex/` — per-monster-block records.
- `src/rules_core/rules_tables/book_of_the_damned_volume_1/` + `book_of_the_damned_volume_2/` — per-archetype / per-monster-block / per-tactic records.
- `src/rules_core/rules_tables/inner_sea_world_guide/` — per-trait / per-feat / per-region records.
- `src/rules_core/rules_tables/inner_sea_{combat, faiths, gods, magic, races, temples, taverns, bestiary, intrigue}/` — per-type cycles per Inner Sea module.
- `data/corpus/<book>/` — Shape B cache for each of the sixteen books.

## Dependency position

- **Depends on:** SD-22 (closed, per-book ingest pipeline); SD-27 (closed, Shape B schema + license-stripping); SD-28 (published 2026-08-01, sister bundle's 7 books already in scope — cross-bundle precedence applies per §16); SD-29 (published 2026-08-01, sister bundle's 4 bestiaries already in scope — cross-bundle precedence applies).
- **Unblocks:** post-tranche consumer (whatever bundle picks up after `tranche/10`).
- **Blocks:** None in-cycle.

## Exit statement

SD-30 is complete when each of the sixteen books' records reaches a
player surface (via reach gate), the Closure Epilogue fires, and
`0.10.<last_build>` is the post-closure value. The bundle's move-not-
copy publish has already landed at `docs/release/SD-30-occult-and-
companion-content-ingestion/`.
