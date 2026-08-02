---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01)
date: 2026-08-01
canonical_branch: tranche/10 (operator directive 2026-08-01)
build_version_target: 0.10.<build>
---

# SD-30 — Local-file Work Queue (replaces Hermes board `codex-tranche-10`)

Per operator directive 2026-08-01, the Hermes board is retired. SD-30's
work queue is a local-file Markdown table. The supervisor reads this
file at top of each cycle tick to identify the next ready card; the
file-touch partition ensures only one cycle claims a card at a time.

## Status legend

- `READY` — not yet claimed. Cycle can pick up.
- `READY (gated on ...)` — not claimable until every named card is `COMPLETE`. The gate is part of the card's state: a cycle that claims a gated card while its gate is open is out of protocol.
- `IN-FLIGHT` — claimed by a cycle, in progress.
- `BLOCKED` — cycle claims the block, captures the gap, surfaces in `progress.md` as a blocker.
- `COMPLETE` — cycle receipt in `progress.md` closes the card.

## Cards (one row per per-book epic cycle-batch)

Rows are in claim-priority order, top-down, matching `loop-instruction.md`'s
"Epic ordering": Epic 1 first, then Epic 2, and no per-book epic until both
are `COMPLETE`.

| ID | Status | Book | Cycle-type | Claimed-by | Claimed-at | Cycle-id |
|----|--------|------|-----------|------------|------------|----------|
| `epic-1-identifier` | READY | Identifier Cleanup | identifier-discipline audit pass | — | — | — |
| `epic-2-prelaunch` | READY (gated on epic-1) | Operator Pre-Launch | local-file dispatch readiness + cycle-0 trap-report (16 books) | — | — | — |
| `epic-3-oa` | READY (gated on epic-1, epic-2) | Occult Adventures | per-class / per-monster-block / per-psychic-discipline | — | — | — |
| `epic-4-ha` | READY (gated on epic-1, epic-2) | Horror Adventures | per-monster-block / per-haunt-block / per-corruption-mechanic | — | — | — |
| `epic-5-my` | READY (gated on epic-1, epic-2) | Mythic Adventures | per-mythic-path / per-monster-block (gated on reach-surface per `decisions.md §18`) | — | — | — |
| `epic-6-mc` | READY (gated on epic-1, epic-2) | Monster Codex | per-monster-block | — | — | — |
| `epic-7-iswg` | READY (gated on epic-1, epic-2) | Inner Sea World Guide | per-trait / per-feat / per-region | — | — | — |
| `epic-8-iscb` | READY (gated on epic-1, epic-2) | Inner Sea Combat | per-trait / per-option | — | — | — |
| `epic-9-isf` | READY (gated on epic-1, epic-2) | Inner Sea Faiths | per-deity / per-trait / per-option | — | — | — |
| `epic-10-isg` | READY (gated on epic-1, epic-2) | Inner Sea Gods | per-deity / per-domain | — | — | — |
| `epic-11-ism` | READY (gated on epic-1, epic-2) | Inner Sea Magic | per-spell / per-magic-trait | — | — | — |
| `epic-12-isr` | READY (gated on epic-1, epic-2) | Inner Sea Races | per-race / per-archetype | — | — | — |
| `epic-13-ist` | READY (gated on epic-1, epic-2) | Inner Sea Temples | per-temple / per-trait | — | — | — |
| `epic-14-isv` | READY (gated on epic-1, epic-2) | Inner Sea Taverns | per-tavern / per-event | — | — | — |
| `epic-15-isb` | READY (gated on epic-1, epic-2) | Inner Sea Bestiary | per-monster-block | — | — | — |
| `epic-16-isi` | READY (gated on epic-1, epic-2) | Inner Sea Intrigue | per-trait / per-faction / per-rule | — | — | — |
| `epic-17-bd1` | READY (gated on epic-1, epic-2) | Book of the Damned Vol. 1 | per-archetype / per-monster-block / per-tactic | — | — | — |
| `epic-18-bd2` | READY (gated on epic-1, epic-2) | Book of the Damned Vol. 2 | per-archetype / per-monster-block / per-tactic | — | — | — |
| `epic-20-version` | READY (gated on epic-1) | Build Version Numbering | first concrete value `0.10.<build>` | — | — | — |
| `epic-21-code-review` | READY (gated on epic-20 + all content epics 3-18) | Bundle Code Review | full-bundle diff review vs. branch point (`decisions.md §26`) | — | — | — |
| `epic-19-closure` | READY (gated on every other card) | Closure Epilogue | tranche promotion PR | — | — | — |

## Cycle claims (cycle-supervisor protocol)

When a cycle claims a card:

1. Edit the card's `Status` to `IN-FLIGHT`.
2. Edit `Claimed-by` to the cycle's harness identifier.
3. Edit `Claimed-at` to the cycle's ISO-8601 timestamp.
4. Edit `Cycle-id` to the cycle's audit ID (e.g., `SD30-E3-F1-001`).
5. Append the cycle's per-cycle facts to `progress.md`.
6. On cycle completion, edit `Status` to `COMPLETE` and append the
   completion receipt to `progress.md`.

## Operator override slot

Operator may add or remove cards directly by editing this file. Cycle
dispatch honors the post-edit state.

## Resolution to operator directives

This file is the load-bearing replacement for the Hermes `codex-tranche-10`
board (operator-confirmed 2026-08-01). When a Hermes board card is
referenced from prior doctrine (`decisions.md`, `scope-draft.md`,
`loop-instruction.md`, etc.), the reference resolves to a `kanban.md`
card id at the time of cycle dispatch.
