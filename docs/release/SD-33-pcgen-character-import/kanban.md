---
canonical: true
owner: god-emporer
status: planning-ready (operator directive 2026-08-11)
date: 2026-08-11
canonical_branch: tranche/11
---

# SD-31 — Local-file Work Queue

The supervisor reads this file at the top of each cycle to identify the next ready card. The
`TR-31-001` file-touch partition ensures only one cycle claims a card at a time.

## Status legend

- `READY` — not yet claimed.
- `READY (gated on ...)` — not claimable until every named card is `COMPLETE`.
- `IN-FLIGHT` — claimed by a cycle.
- `BLOCKED` — cycle claims the block, captures the gap, surfaces it in `progress.md`.
- `COMPLETE` — cycle receipt in `progress.md` closes the card.

## Cards

| # | Card | Epic | Status |
|---|---|---|---|
| 1 | Preflight: re-derive partition, cut `tranche/11`, fixture inventory | SD31-E1 | `READY` |
| 2 | Layer 1 — tokenizer + round-trip proof | SD31-E2 | `READY (gated on 1)` |
| 3 | Layer 1 — malformed-input behaviour | SD31-E2 | `READY (gated on 2)` |
| 4 | Layer 2 — core typed records | SD31-E3 | `READY (gated on 2)` |
| 5 | Layer 2 — `EQUIPSET` tree | SD31-E3 | `READY (gated on 4)` |
| 6 | Layer 2 — unknown-token survival | SD31-E3 | `READY (gated on 4)` |
| 7 | Layer 3 — resolver per token kind | SD31-E4 | `READY (gated on 4)` |
| 8 | Layer 3 — parameterized feats as a pair | SD31-E4 | `READY (gated on 7)` |
| 9 | Layer 3 — fidelity report | SD31-E4 | `READY (gated on 7)` |
| 10 | Layer 3 — `CreateCharacterRequest` construction + no-computed-values proof | SD31-E4 | `READY (gated on 7)` |
| 11 | IPC — `import_pcgen_character` | SD31-E5 | `READY (gated on 10)` |
| 12 | IPC — acknowledged lossy import | SD31-E5 | `READY (gated on 11)` |
| 13 | UI — import affordance | SD31-E6 | `READY (gated on 11)` |
| 14 | UI — mapping-review screen | SD31-E6 | `READY (gated on 13)` |
| 15 | UI — imported character reaches the sheet (live) | SD31-E6 | `READY (gated on 14)` |
| 16 | Oracle parity harness | SD31-E7 | `READY (gated on 11)` |
| 17 | Both fixtures at parity | SD31-E7 | `READY (gated on 16)` |
| 18 | Bundle code review (adversarial) | SD31-E8 | `READY (gated on 15, 17)` |
| 19 | Closure epilogue | SD31-E9 | `READY (gated on 18)` |

Card 16 does not wait on the UI cards — parity work can run in parallel with 13–15.
