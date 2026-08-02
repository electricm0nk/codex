# SD-28 — Local-file Work Queue (replaces Hermes board `codex-tranche-8`)

Per operator directive 2026-08-01, the Hermes board is retired. SD-28's
work queue is a local-file Markdown table. The supervisor reads this file
at top of each cycle tick to identify the next ready card; the
file-touch partition ensures only one cycle claims a card at a time.

## Status legend

- `READY` — not yet claimed. Cycle can pick up once every `Depends-on` card is `COMPLETE`.
- `IN-FLIGHT` — claimed by a cycle, in progress. Other cycles must wait.
- `BLOCKED` — cycle claims the block, captures the gap, surfaces in `progress.md` as a blocker.
- `COMPLETE` — cycle receipt in `progress.md` closes the card.

**Dispatch tiebreak:** next card = lowest `Order` among `READY` cards whose
every `Depends-on` card is `COMPLETE`. A card whose `Depends-on` is not
fully `COMPLETE` is not eligible regardless of `Order` or `Status`.

## Cards (one row per per-book epic cycle-batch), in dispatch order

| Order | ID | Status | Book | Cycle-type | Depends-on | Claimed-by | Claimed-at | Cycle-id |
|---|----|--------|------|-----------|------------|------------|------------|----------|
| 1 | `epic-1-identifier` | COMPLETE | Identifier Cleanup | identifier-discipline audit pass | none | sd28-epic1 | 2026-08-02T02:25:56Z | SD28-E1-F1-001 |
| 2 | `epic-2-prelaunch` | COMPLETE | Operator Pre-Launch | local-file dispatch readiness + license precheck | `epic-1-identifier` | sd28-epic2 | 2026-08-01T00:00:00Z | SD28-E2-F1-001 |
| 3 | `epic-3-uc` | IN-FLIGHT | Ultimate Combat | per-class / per-chooser | `epic-2-prelaunch` | epic-3-uc | 2026-08-01T00:00:00Z | SD28-E3-F1-001 |
| 4 | `epic-4-um` | IN-FLIGHT | Ultimate Magic | per-class / per-spell-subsystem | `epic-2-prelaunch` | epic-4-um | 2026-08-02T03:40:27Z | SD28-E4-F1-001 |
| 5 | `epic-5-ue` | IN-FLIGHT | Ultimate Equipment | per-equipment-entry | `epic-2-prelaunch` | epic-5-ue | 2026-08-01T00:00:00Z | SD28-E5-F1-001 |
| 6 | `epic-6-ui` | IN-FLIGHT | Ultimate Intrigue | per-class / per-social-rule | `epic-2-prelaunch` | epic-6-ui | 2026-08-01T00:00:00Z | SD28-E6-F1-001 |
| 7 | `epic-7-ucam` | IN-FLIGHT | Ultimate Campaign | per-system-subsystem | `epic-2-prelaunch` | epic-7-ucam | 2026-08-01T00:00:00Z | SD28-E7-F1-001 |
| 8 | `epic-8-uw` | READY | Ultimate Wilderness | per-class / per-Companion-rule | `epic-2-prelaunch` | — | — | — |
| 9 | `epic-9-upsi` | READY | Ultimate Psionics (Dreamscarred Press tier) | per-class / per-power, license-gated | `epic-2-prelaunch` | — | — | — |
| 10 | `epic-11-version` | COMPLETE | Build Version Numbering | first concrete value `0.8.<build>` | `epic-1-identifier` | sd28-epic11 | 2026-08-02T03:00:00Z | SD28-E11-F1-001 |
| 11 | `epic-12-code-review` | READY | Bundle Code Review | full-bundle diff review vs. branch point (`decisions.md §26`) | `epic-3-uc`, `epic-4-um`, `epic-5-ue`, `epic-6-ui`, `epic-7-ucam`, `epic-8-uw`, `epic-9-upsi`, `epic-11-version` | — | — | — |
| 12 | `epic-10-closure` | READY | Closure Epilogue | tranche promotion PR | `epic-1-identifier`, `epic-2-prelaunch`, `epic-3-uc`, `epic-4-um`, `epic-5-ue`, `epic-6-ui`, `epic-7-ucam`, `epic-8-uw`, `epic-9-upsi`, `epic-11-version`, `epic-12-code-review` (everything else) | — | — | — |

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
