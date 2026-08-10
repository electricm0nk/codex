# SD-29 — Local-file Work Queue (replaces Hermes board `codex-tranche-9`)

Per operator directive 2026-08-01, the Hermes board is retired. SD-29's
work queue is a local-file Markdown table. The supervisor reads this file
at top of each cycle tick to identify the next ready card; the
file-touch partition ensures only one cycle claims a card at a time.

**Re-cut 2026-08-10 (`decisions.md §37`).** Cards are now lane-scoped
(kind, or merged-kind-pair), not per-book. A lane epic's cycle-batches
fan out per book internally — see `epic-breakdown.md` for each lane's
per-book unit counts.

## Status legend

- `READY` — not yet claimed. Cycle can pick up once every `Depends-on` card is `COMPLETE`.
- `IN-FLIGHT` — claimed by a cycle, in progress. Other cycles must wait.
- `BLOCKED` — cycle claims the block, captures the gap, surfaces in `progress.md` as a blocker.
- `COMPLETE` — cycle receipt in `progress.md` closes the card.

**Dispatch tiebreak:** next card = lowest `Order` among `READY` cards whose
every `Depends-on` card is `COMPLETE`. A card whose `Depends-on` is not
fully `COMPLETE` is not eligible regardless of `Order` or `Status`.

## Cards (one row per lane epic cycle-batch), in dispatch order

| Order | ID | Status | Lane / Scope | Cycle-type | Depends-on | Claimed-by | Claimed-at | Cycle-id |
|---|----|--------|------|-----------|------------|------------|------------|----------|
| 1 | `epic-1-identifier` | READY | Identifier Cleanup | identifier-discipline audit pass | none | — | — | — |
| 2 | `epic-2-prelaunch` | READY | Operator Pre-Launch | corpus-wide (7-book) cycle-0 trap-report + work-inventory | `epic-1-identifier` | — | — | — |
| 3 | `epic-3-provenance` | READY | Provenance Gate | PI-screening wired into each lane's extraction step; license-matrix citation for OGL/attribution | `epic-2-prelaunch` | — | — | — |
| 4 | `epic-4-monster-lane-pilot` | READY | Monster / Monster-Ability Chassis Lane — pilot | Bonus Bestiary end-to-end (14 monster + 17 monster_ability + 3 class) | `epic-3-provenance` | — | — | — |
| 5 | `epic-4-monster-lane-extend` | READY | Monster / Monster-Ability Chassis Lane — extend | Bestiary 2, 3, 4, 5, 6, Monster Codex (2,142 remaining units) | `epic-4-monster-lane-pilot` | — | — | — |
| 6 | `epic-5-race-trait-lane` | READY | Race-Trait Lane | classifier defect fix + per-book ingest, 1,124 units across all 7 books | `epic-3-provenance` | — | — | — |
| 7 | `epic-6-companion-lane` | READY | Companion Lane | mechanism-build + per-book ingest, 275 units across all 7 books | `epic-3-provenance` | — | — | — |
| 8 | `epic-7-residual-lane` | READY | Residual Proven-Path Content Lane | spell/equipment/feat/race/equipment_modifier/class, 203 units (excludes `class_feature`, see `epic-breakdown.md` Epic 7 note) | `epic-3-provenance` | — | — | — |
| 9 | `epic-9-version` | READY | Build Version Numbering | first concrete value `0.9.<build>` | `epic-1-identifier` | — | — | — |
| 10 | `epic-8-toolkit` | READY | DM Toolkit extension | consume Epic 4's monster records (optional; safe default retrofit per `successor-forward-scope-register.md C3.1`) | `epic-4-monster-lane-pilot` | — | — | — |
| 11 | `epic-10-review` | READY | Bundle Code Review | full-bundle diff review vs. branch point (`decisions.md §27`) | `epic-4-monster-lane-extend`, `epic-5-race-trait-lane`, `epic-6-companion-lane`, `epic-7-residual-lane`, `epic-9-version`, `epic-8-toolkit` (COMPLETE or `decision-blocked`) | — | — | — |
| 12 | `epic-11-closure` | READY | Closure Epilogue | tranche promotion PR | `epic-1-identifier`, `epic-2-prelaunch`, `epic-3-provenance`, `epic-4-monster-lane-pilot`, `epic-4-monster-lane-extend`, `epic-5-race-trait-lane`, `epic-6-companion-lane`, `epic-7-residual-lane`, `epic-9-version`, `epic-8-toolkit` (COMPLETE or `decision-blocked`), `epic-10-review` | — | — | — |

## Cycle claims (cycle-supervisor protocol)

When a cycle claims a card:

1. Edit the card's `Status` to `IN-FLIGHT`.
2. Edit `Claimed-by` to the cycle's harness identifier.
3. Edit `Claimed-at` to the cycle's ISO-8601 timestamp.
4. Edit `Cycle-id` to the cycle's audit ID (e.g., `SD29-E4-F1-001`).
5. Append the cycle's per-cycle facts to `progress.md` (write to
   `progress.md` after writing the kanban claim; the supervisor reads
   progress.md to verify the prior cycle complete before claiming the
   next).
6. On cycle completion, edit `Status` to `COMPLETE` and append the
   completion receipt to `progress.md`.

## Operator override slot

Operator may add or remove cards directly by editing this file. Cycle
dispatch honors the post-edit state.

## Resolution to operator directives

This file is the load-bearing replacement for the Hermes `codex-tranche-9`
board (operator-confirmed 2026-08-01). When a Hermes board card is
referenced from prior doctrine (`decisions.md`, `scope-draft.md`,
`loop-instruction.md`, etc.), the reference resolves to a `kanban.md`
card id at the time of cycle dispatch.
