---
title: SD-18 — Epic Breakdown
status: draft (operator review required)
date: 2026-07-12
companion_to: /home/ubuntu/workspace/SD-18-core-rules-breadth-scope-draft.md
---

# SD-18 — Epic Breakdown

Maps the 34 acceptance criteria from the scope doc's §3 to execution lanes and cycle units.

## Execution lane split

```
Pre-loop gate (card-routed, tech-priest)
└── §1.1 Consumer-side composition                                     [1 criterion]

Loop-routed coverage (claude-code loop, one per cycle)
├── §3.1 Race rows (7 criteria, levels 1-20)                          [7 criteria]
├── §3.2 Class rows (11 criteria, levels 1-20)                        [11 criteria]
├── §3.3 Interaction rows (2 criteria)                                [2 criteria]
├── §3.4 Spell school cards (9 criteria: Abjuration, Conjuration,
│         Divination, Enchantment, Evocation, Illusion, Necromancy,
│         Transmutation, Universal)                                   [9 criteria]
└── §3.5 Equipment category cards (4 criteria: arms_armor, general,
          magic_items, equipmods)                                     [4 criteria]
```

Total: **34 acceptance criteria.** 1 pre-loop (tech-priest) + 33 loop-routed (claudi-code cycle).

## Cycle ordering (operator-prioritized)

Per operator directive 2026-07-12, the loop's first cycles land the two SD-13 interaction rows:

```
Cycle 1+  §3.3 Interaction row 1: Human bonus feat / ability-bonus seam
Cycle 2+  §3.3 Interaction row 2: non-Human race × class interaction pressure
```

After interactions:

```
Cycle 3-9  §3.1 Race rows × 7 (Dwarf through Human)
Cycle 10-20  §3.2 Class rows × 11 (Barbarian through Wizard)
Cycle 21-29  §3.4 Spell schools × 9 (per strict school alphabet)
Cycle 30-33  §3.5 Equipment categories × 4 (per corpus-natural category)
```

This is the *suggested* order. The loop's actual progression is data-dependent — see §4 of the scope doc for the loop's reading rules. The loop may reorder as needed based on per-cycle feasibility.

## Cycle unit definition

A single loop cycle lands one acceptance criterion. Each cycle:

1. Picks one criterion from the progress doc's "open" list (§4.3 of scope doc).
2. Creates a feature branch off `tranche/3` (e.g. `loop/tranche3-cycle-2026-07-13T0900-dwarf-favored-class`).
3. Lands the bounded work (code, tests, fixtures, handoff doc).
4. Auto-merges to `tranche/3`.
5. Self-heals inline or exits `FAIL`.
6. Deletes feature branch from local and origin.
7. Mints a kanban card on `codex-tranche-3` per the §4.3 schema.
8. Updates the progress doc.
9. Exits.

A cycle is a *unit of post-mortem*, not a unit of delivered scope. One cycle, one criterion, one card. The cycle log in the progress doc plus the cards on the board let a 3-day-later operator reconstruct any specific cycle.

## What the breakdown does NOT specify

- Per-criterion implementation approach (the loop picks the smallest change that satisfies the criterion).
- Per-criterion TDD structure (inherits from the matured SD-13 model's red-green-refactor pattern; see `references/sd13-loop-model-excerpt.md`).
- Per-criterion timing (depends on corpus size, parser friction, and behavior complexity; the loop's self-healing handles friction; volume is incidental).

## Cross-reference

- `decisions.md` §6 (pre-loop vs loop lane split).
- `technical-design.md` (per-cycle mechanics, parser/race/class seam function inventory, corpus paths, branch lifecycle command sequence).
- `risks-and-open-questions.md` (per-criterion risks and blockers).
- `/home/ubuntu/workspace/SD-18-core-rules-breadth-scope-draft.md` (canonical handoff doc; acceptance criteria with concrete corpus/code pointers).
