---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01)
date: 2026-08-01
canonical_branch: tranche/8 (operator directive 2026-08-01)
kanban_board: retired (operator directive 2026-08-01) — see kanban.md
companion_to: ./scope-draft.md
mirror_of: ./scope-draft.md
build_version_target: 0.8.<build>
---

# SD-28 — Ultimate Book Content Ingestion

## Purpose

End-to-end content-source ingest for the seven Ultimate-line books on SD-28's
scope (six Paizo hardcover + one Dreamscarred Press hardcover). Per-book ingest
cycles produce canonical records in `src/rules_core/rules_tables/<book>/` that
satisfy the reach gate (`apps/desktop/src-tauri/src/reach_gate.rs`) — a record
is not done until it reaches a player surface.

## Source STC contents

- `scope-draft.md` — committed scope shape, seven books confirmed.
- `decisions.md` — 30 decisions (plus §15a, §17a) including the operator-pinned amendments of 2026-08-01 (book list, tranche/8, build 0.8.x, no-Hermes-board, cross-book conflict rule, reach-gate doctrine, third-party tier).
- `loop-instruction.md` — per-cycle procedure; local-file dispatch via `kanban.md`/`progress.md`.
- `forward-scope-register.md` — successor work depending on SD-28's output.
- `epic-breakdown.md` — 12 epics × ~3 = ~36 criteria; Closure Epilogue fires LAST.
- `technical-requirements.md` — pre-loop prerequisites + normative requirements + out-of-scope.
- `technical-design.md` — architectural surface for the seven books, including the third-party (Dreamscarred Press) tier license gate.
- `acceptance-and-verification.md` — Given/When/Then per criterion.
- `progress.md` — per-cycle receipt log.
- `release-notes.md` — release-notes template; populated at closure.
- `kanban.md` — local-file work queue (replaces Hermes board).
- `artifacts/` — per-cycle receipts + finding logs.

## Authority surface

Canonical (repo-resident) home:

`docs/release/SD-28-ultimate-book-content-ingestion/` (after the move-not-copy
publish landing this cycle). Source-of-record (this directory) is removed on
the publish commit per `decisions.md §22`.

## Objective

Per-cycle, ingest one canonical record from one Ultimate book into
`src/rules_core/rules_tables/<book>/`, with the record reaching a player
surface via the reach gate. Each per-book content-source-ingest epic produces
the per-class / per-monster-block / per-equipment-entry cycles named in
`scope-draft.md §"Book list"`.

## In scope

- Six Paizo hardcovers: Ultimate Combat, Ultimate Magic, Ultimate Equipment, Ultimate Intrigue, Ultimate Campaign, Ultimate Wilderness.
- One Dreamscarred Press hardcover: Ultimate Psionics (third-party tier; license-gated per `decisions.md §17`).
- Reach-gate satisfaction for every record ingested (the reach gate is the definition of done per `decisions.md §18`).
- Cross-book conflict resolution per `decisions.md §16` (newer book = doctrine, older book = errata).

## Out of scope

- SD-29 Bestiary 2-5 (separate bundle).
- SD-30 Occult Adventures + companions (separate bundle).
- SD-22's APG/ACG/Bestiary 1/DM toolkit (closed).
- Real-time execution engines (RNG, opponent state, turn sequencing). Per `decisions.md §18`, real-time engines remain out of scope; rules-data engines are in scope only when strictly necessary.
- Hermes-board operations. Per `decisions.md §15a`, the board is retired; SD-28 dispatches via local file.

## Produced artifacts

- `src/rules_core/rules_tables/ultimate_combat/` — per-class / per-chooser slice records for Combat maneuvers (Gunslinger, Ninja, Samurai), martial rules (panache, grit), archetype variant rules.
- `src/rules_core/rules_tables/ultimate_magic/` — per-class / per-spell-subsystem records for new casting variants, spell subsystems (words of power, truename), class features.
- `src/rules_core/rules_tables/ultimate_equipment/` — per-equipment-entry records.
- `src/rules_core/rules_tables/ultimate_intrigue/` — per-class / per-social-rule records for Vigilante, Mesmerist (where canonical id is owned by SD-30 per `decisions.md §5`), social combat, intrigue subsystems.
- `src/rules_core/rules_tables/ultimate_campaign/` — player-options subsystems (downtime, kingdom-building, traits, retraining).
- `src/rules_core/rules_tables/ultimate_wilderness/` — per-class / per-Companion-rules records.
- `src/rules_core/rules_tables/ultimate_psionics/` — third-party tier; license-verified.
- `data/corpus/<book>/` — Shape B cache for each of the seven books.

## Dependency position

- **Depends on:** SD-22 (closed, doctrine-of-record for per-book ingest pipeline and reach-gate mechanic); SD-27 (closed, Shape B schema + license-stripping pattern); local `~/workspace/governance/pcgen-licenses.md` (forthcoming, license-conformance surface).
- **Unblocks:** SD-29 (Bestiary 2-5) and SD-30 (Occult Adventures + companions) inherit the per-book ingest pipeline shape from SD-28.
- **Blocks:** None in-cycle; the post-tranche consumer is whatever bundle picks up after SD-30.

## Exit statement

SD-28 is complete when each of the seven books' records reaches a player surface
(via reach gate), the Closure Epilogue fires, and `0.8.<last_build>` is the
post-closure value. The bundle's move-not-copy publish has already landed at
`docs/release/SD-28-ultimate-book-content-ingestion/`.
