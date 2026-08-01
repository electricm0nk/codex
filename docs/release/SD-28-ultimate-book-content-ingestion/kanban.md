# SD-28 — Local-file Work Queue (replaces Hermes board `codex-tranche-8`)

Per operator directive 2026-08-01, the Hermes board is retired. SD-28's
work queue is a local-file Markdown table. The supervisor reads this file
at top of each cycle tick to identify the next ready card; the
file-touch partition ensures only one cycle claims a card at a time.

## Status legend

- `READY` — not yet claimed. Cycle can pick up.
- `IN-FLIGHT` — claimed by a cycle, in progress. Other cycles must wait.
- `BLOCKED` — cycle claims the block, captures the gap, surfaces in `progress.md` as a blocker.
- `COMPLETE` — cycle receipt in `progress.md` closes the card.

## Cards (one row per per-book epic cycle-batch)

| ID | Status | Book | Cycle-type | Claimed-by | Claimed-at | Cycle-id |
|----|--------|------|-----------|------------|------------|----------|
| `epic-3-uc` | READY | Ultimate Combat | per-class / per-chooser | — | — | — |
| `epic-4-um` | READY | Ultimate Magic | per-class / per-spell-subsystem | — | — | — |
| `epic-5-ue` | READY | Ultimate Equipment | per-equipment-entry | — | — | — |
| `epic-6-ui` | READY | Ultimate Intrigue | per-class / per-social-rule | — | — | — |
| `epic-7-ucam` | READY | Ultimate Campaign | per-system-subsystem | — | — | — |
| `epic-8-uw` | READY | Ultimate Wilderness | per-class / per-Companion-rule | — | — | — |
| `epic-9-upsi` | READY | Ultimate Psionics (Dreamscarred Press tier) | per-class / per-power, license-gated | — | — | — |
| `epic-10-closure` | READY (gated) | Closure Epilogue | tranche promotion PR | — | — | — |
| `epic-11-version` | READY (gated) | Build Version Numbering | first concrete value `0.8.<build>` | — | — | — |
| `epic-12-code-review` | READY (gated) | Bundle Code Review | full-bundle diff review vs. branch point (`decisions.md §26`) | — | — | — |
| `epic-1-identifier` | READY | Identifier Cleanup | identifier-discipline audit pass | — | — | — |
| `epic-2-prelaunch` | READY | Operator Pre-Launch | local-file dispatch readiness + license precheck | — | — | — |

## Cycle claims (cycle-supervisor protocol)

When a cycle claims a card:

1. Edit the card's `Status` to `IN-FLIGHT`.
2. Edit `Claimed-by` to the cycle's harness identifier.
3. Edit `Claimed-at` to the cycle's ISO-8601 timestamp.
4. Edit `Cycle-id` to the cycle's audit ID (e.g., `SD28-E3-F1-001`).
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

This file is the load-bearing replacement for the Hermes `codex-tranche-8`
board (operator-confirmed 2026-08-01). When a Hermes board card is
referenced from prior doctrine (`decisions.md`, `scope-draft.md`,
`loop-instruction.md`, etc.), the reference resolves to a `kanban.md`
card id at the time of cycle dispatch.
