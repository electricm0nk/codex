# SD-29 — Local-file Work Queue (replaces Hermes board `codex-tranche-9`)

Per operator directive 2026-08-01, the Hermes board is retired. SD-29's
work queue is a local-file Markdown table. The supervisor reads this file
at top of each cycle tick to identify the next ready card; the
file-touch partition ensures only one cycle claims a card at a time.

**Re-cut 2026-08-10 (`decisions.md §37`).** Cards are now lane-scoped
(kind, or merged-kind-pair), not per-book. A lane epic's cycle-batches
fan out per book internally — see `epic-breakdown.md` for each lane's
per-book unit counts.

**Re-scoped corpus-wide 2026-08-10 (`decisions.md §38`).** Every lane below now fans out across
all 37 in-scope books, not the retired seven-book set. Epic 4 (Tier 1, proven-path) is split into
three cards since it now covers six kinds at corpus scale; Epics 5-7 (Tier 2, mechanism-gated)
each pilot on one small book before extending corpus-wide.

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
| 1 | `epic-1-identifier` | COMPLETE | Identifier Cleanup | identifier-discipline audit pass | none | sd29-e1-identifier | 2026-08-10T00:00:00Z | SD29-E1-F1-001 |
| 2 | `epic-2-prelaunch` | COMPLETE | Operator Pre-Launch | corpus-wide (37-book) cycle-0 trap-report + work-inventory | `epic-1-identifier` | sd29-e2-prelaunch | 2026-08-10T23:56:00Z | SD29-E2-F1-001 |
| 2.5 | `epic-1b-naming-sweep` | COMPLETE | Function-Based Naming Sweep | corpus-source rename sweep: SD-NN + GE-NN tags out of file names, directory names, and identifiers (operator directive 2026-08-11) | `epic-1-identifier` | sd29-e1b-naming | 2026-08-11T00:00:00Z | SD29-E1B-F1-001 |
| 3 | `epic-3-provenance` | COMPLETE | Provenance Gate | PI-screening wired into each lane's extraction step; license-matrix citation for OGL/attribution, corpus-wide | `epic-2-prelaunch` | sd29-e3-provenance | 2026-08-11T00:00:00Z | SD29-E3-F1-001 |
| 4 | `epic-4-proven-equip-mod` | IN-FLIGHT | Proven-Path Lanes — equipment + equipment_modifier | corpus-wide, 1,144 + 812 remaining units (equipment corrected from 1,163 by Epic 2 — the old figure counted `beginner_box`'s 19 excluded units; see `corpus-shape-37-books.md` §3) | `epic-3-provenance` | sd29-e4-equip | 2026-08-11T00:00:00Z | SD29-E4-F1-001 |
| 5 | `epic-4-proven-spell` | READY | Proven-Path Lanes — spell | corpus-wide, 1,754 remaining units | `epic-3-provenance` | — | — | — |
| 6 | `epic-4-proven-feat-race-class` | READY | Proven-Path Lanes — feat + race + class | corpus-wide, 1,348 + 96 + 158 remaining units (feat: the prior 1,350 counted the kind's 2 `deferred-with-reason` units as remaining — predicate difference, not an arithmetic error; see `corpus-shape-37-books.md` §3) | `epic-3-provenance` | — | — | — |
| 7 | `epic-5-monster-lane-pilot` | READY | Monster / Monster-Ability Chassis Lane — pilot | Bonus Bestiary end-to-end (14 monster + 17 monster_ability) | `epic-3-provenance` | — | — | — |
| 8 | `epic-5-monster-lane-extend` | READY | Monster / Monster-Ability Chassis Lane — extend | corpus-wide, every remaining book (1,224 monster + 3,107 monster_ability minus the pilot's 31) | `epic-5-monster-lane-pilot` | — | — | — |
| 9 | `epic-6-race-trait-lane-pilot` | READY | Race-Trait Lane — pilot | classifier defect fix + `inner_sea_intrigue` (9 units) | `epic-3-provenance` | — | — | — |
| 10 | `epic-6-race-trait-lane-extend` | READY | Race-Trait Lane — extend | corpus-wide, 27 books, 3,412 remaining units minus the pilot's 9 | `epic-6-race-trait-lane-pilot` | — | — | — |
| 11 | `epic-7-companion-lane-pilot` | READY | Companion Lane — pilot | mechanism-build + `inner_sea_combat` (10 units) | `epic-3-provenance` | — | — | — |
| 12 | `epic-7-companion-lane-extend` | READY | Companion Lane — extend | corpus-wide, 17 books, 1,683 remaining units minus the pilot's 10 | `epic-7-companion-lane-pilot` | — | — | — |
| 13 | `epic-9-version` | READY | Build Version Numbering | first concrete value `0.9.<build>` | `epic-1-identifier` | — | — | — |
| 14 | `epic-8-toolkit` | READY | DM Toolkit extension | consume Epic 5's monster records (optional; safe default retrofit per `successor-forward-scope-register.md C3.1`) | `epic-5-monster-lane-pilot` | — | — | — |
| 15 | `epic-10-review` | READY | Bundle Code Review | full-bundle diff review vs. branch point (`decisions.md §27`) | `epic-4-proven-equip-mod`, `epic-4-proven-spell`, `epic-4-proven-feat-race-class`, `epic-5-monster-lane-extend`, `epic-6-race-trait-lane-extend`, `epic-7-companion-lane-extend`, `epic-9-version`, `epic-8-toolkit` (COMPLETE or `decision-blocked`) | — | — | — |
| 16 | `epic-11-closure` | READY | Closure Epilogue | tranche promotion PR | all cards above (COMPLETE or `decision-blocked`) | — | — | — |

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
