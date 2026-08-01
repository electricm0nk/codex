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

# SD-29 — Bestiary 2-3-4-5 Content Ingestion

## Purpose

End-to-end content-source ingest for the four bestiaries on SD-29's scope
(Bestiary 2-5). Per-bestiary ingest cycles produce canonical monster (or
player-options for Bestiary 5) records in `src/rules_core/rules_tables/beastiary<N>/`
that satisfy the reach gate (`apps/desktop/src-tauri/src/reach_gate.rs`) — a
record is not done until it reaches a player surface.

## Source STC contents

- `scope-draft.md` — committed scope shape, four bestiaries confirmed.
- `decisions.md` — 21 decisions including the operator-pinned amendments of 2026-08-01 (bestiary list, tranche/9, build 0.9.x, no-Hermes-board, Bestiary 5 shape-resolution, cross-book conflict rule, reach-gate-doD doctrine).
- `loop-instruction.md` — per-cycle procedure; local-file dispatch via `kanban.md`/`progress.md`.
- `forward-scope-register.md` — successor work depending on SD-29's output.
- `epic-breakdown.md` — 9 epics × ~3 criteria = ~30 criteria; Closure Epilogue fires LAST.
- `technical-requirements.md` — pre-loop prerequisites + normative requirements + out-of-scope.
- `technical-design.md` — architectural surface for the four bestiaries, including the Bestiary 5 shape-resolution.
- `acceptance-and-verification.md` — Given/When/Then per criterion.
- `progress.md` — per-cycle receipt log.
- `release-notes.md` — release-notes template; populated at closure.
- `kanban.md` — local-file work queue (replaces Hermes board).
- `risks-and-open-questions.md` — primary risks + open questions.
- `artifacts/` — per-cycle receipts + finding logs.

## Authority surface

Canonical (repo-resident) home:

`docs/release/SD-29-bestiary-2-3-4-5-content-ingestion/` (after the move-not-
copy publish landing this cycle). Source-of-record (this directory) is
removed on the publish commit per `decisions.md §13` + the SD-27 / SD-28
move-not-copy precedent (operator directive 2026-08-01).

## Objective

Per-cycle, ingest one canonical record from one bestiary into
`src/rules_core/rules_tables/beastiary<N>/`, with the record reaching a
player surface via the reach gate. Each per-book content-source-ingest
epic produces the per-monster-block cycles named in `scope-draft.md §"Book
list"` for Bestiary 2-4 and the per-race / per-feat / per-companion-mod
cycles for Bestiary 5 (operator-pinned per cycle-0 trap-report output).

## In scope

- **Bestiary 2, 3, 4** — per-monster-block cycles. Cycles produce canonical monster stat-block slices that the rules-core compute path can read.
- **Bestiary 5** — player-options cycles (race / feat / companion-mod). Operator-pinned per cycle-0 trap-report output: if the inventory surfaces zero `monster` units, Epic 5's cycle runs the player-options cycles instead.
- Reach-gate satisfaction for every record ingested (the reach gate is the definition of done per `decisions.md §19`).
- Cross-book conflict resolution per `decisions.md §16` (newer book = doctrine, older book = errata).

## Out of scope

- Bestiary 1 (closed in SD-22).
- SD-28's Ultimate books (separate bundle).
- SD-30's Occult + companions (separate bundle).
- Mythic monster appendices (not in any current SD).
- NPC codex (not in any current SD).
- Real-time execution engines (RNG, opponent state, turn sequencing). Per `decisions.md §19`, real-time engines remain out of scope; rules-data engines are in scope only when strictly necessary.
- Hermes-board operations. Per `decisions.md §14a`, the board is retired; SD-29 dispatches via local file.

## Produced artifacts

- `src/rules_core/rules_tables/beastiary2/` — per-monster-block records.
- `src/rules_core/rules_tables/beastiary3/` — per-monster-block records.
- `src/rules_core/rules_tables/beastiary4/` — per-monster-block records.
- `src/rules_core/rules_tables/beastiary5/` — per-race / per-feat / per-companion-mod records (gated on cycle-0 trap-report output).
- `data/corpus/beastiary{2,3,4,5}/` — Shape B cache per book.

## Dependency position

- **Depends on:** SD-22 (closed, Bestiary 1 ingest pipeline; reach-gate mechanic); SD-27 (closed, Shape B schema).
- **Unblocks:** SD-30 (separate bundle, no SD-29 dependency).
- **Blocks:** None in-cycle; the post-tranche consumer is whatever bundle picks up after SD-30.

## Exit statement

SD-29 is complete when each of the four bestiaries' records reaches a player
surface (via reach gate), the Closure Epilogue fires, and `0.9.<last_build>`
is the post-closure value. The bundle's move-not-copy publish has already
landed at `docs/release/SD-29-bestiary-2-3-4-5-content-ingestion/`.
