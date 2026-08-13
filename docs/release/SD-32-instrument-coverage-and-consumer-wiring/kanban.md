---
canonical: true
owner: god-emporer
status: planning-ready (2026-08-13)
date: 2026-08-13
companion_to: ./epic-breakdown.md
---

# SD-32 — Local-file Work Queue

Per the standing operator directive (2026-08-01), there is no Hermes board.
SD-32's queue is this table. A cycle reads it at tick, claims the top `READY`
card, and the file-touch partition keeps one claimant per card.

## Status legend

- `READY` — claimable now.
- `READY (gated on ...)` — not claimable until every named card is `COMPLETE`.
  The gate is part of the card's state; claiming a gated card is out of protocol.
- `BLOCKED (decision)` — **not claimable at all** until an operator decision
  lands. Distinct from `BLOCKED`: nothing a cycle can do opens it.
- `IN-FLIGHT` — claimed.
- `BLOCKED` — the cycle claims the block, captures the gap, surfaces it in `progress.md`.
- `COMPLETE` — receipt in `progress.md` closes the card. **A card whose cycle moved
  fewer units than its ceiling, with a correct account of why, closes `COMPLETE`**
  (`decisions.md §1.3`).

## Cards

Rows are in claim-priority order, matching `scope-draft.md §5`'s dispatch order.

| ID | Status | Epic | Ceiling (units) | Rank / R | Claimed-by | Claimed-at | Cycle-id |
|----|--------|------|---:|---|------------|------------|----------|
| `inventory-determinism` | COMPLETE | Work-Inventory Identity and Scan-Order Fix | 0 (by design — instrument fix) | — | probe-determinism | 2026-08-13 | `5fb94067` |
| `e1-measurement-gate` | READY | Measurement-Gate Decision Request | 0 (gates 7,479) | — | — | — | — |
| `e2-probe-coverage` | COMPLETE | Equipment-Effect Probe Coverage Extension | 358 (real ceiling 63) | **1** / 18 moved | doneness-inprogress | 2026-08-13 | `5ed6bdc0` |
| `e4-classifier-calibration` | READY | Classifier — hand-labelled sample (E4-F1, gate) | 0 (gates 1,776) | 2a | — | — | — |
| `e3-effect-wiring` | READY (gated on e2-probe-coverage) | Equipment Effect Wiring for Inert Items | 375 | **3** / 125 | — | — | — |
| `e4-classifier` | READY (gated on e4-classifier-calibration opening the gate) | Wiring-Class Classifier (E4-F2, E4-F3) | 1,776 | **2** / 444 ceiling | — | — | — |
| `e5-static-sweep` | BLOCKED (decision) — `decisions.md §2` | Static Corpus-Literal Byte-Equality Sweep | 4,805 | 0 today (1,602 if gate opens) | — | — | — |
| `e6-derived-check` | BLOCKED (decision) — `decisions.md §2` | Derived Evaluator-vs-Fixture Check | 2,674 | 0 today (535 if gate opens) | — | — | — |
| `e7-structural-report` | READY | Structural-Block Report | 0 (by design) | — | — | — | — |
| `e8-code-review` | READY (gated on e2, e3, e4, e7) | Bundle Code Review | — | — | — | — | — |
| `e9-closure` | READY (gated on every other claimable card) | Closure | — | — | — | — | — |
| `spell-consumer-delta-probe` | COMPLETE | Spell Consumer-Delta Probe (instrument only) | 0 (by design — instrument build) | — | probe-spell | 2026-08-13 | `aafd492c` |
| `ground-spell-units` | COMPLETE | Apply the Spell Consumer-Delta Probe | 623 keys wired (real ceiling **46** to `done`) | — / 623 grounded, 46 to `done` | probe-spell-ground | 2026-08-13 | `90bd9975` |
| `verify-this-run` | COMPLETE | Adversarial Verification of the Determinism + Spell-Probe Run | 0 (by design — verification only) | — / 0 moved, 0 reversed | probe-verify | 2026-08-13 | `verify-this-run` |

### A note on `inventory-determinism`

It is not one of the scope-authored epics; it was added when the
wiring-classifier cycle's near-miss made every before/after comparison on this
board suspect. Its ceiling is **0 units by design** — it fixes the instrument
that the other cards' numbers are read from, and a cycle that fixed a measuring
tool and also moved the thing being measured would have proven nothing. It
closes `COMPLETE` having moved zero units and having *declined* the one (+1
`grounded`) that a side effect of its own change offered. See `progress.md`.

### A note on the two spell cards

Both were added by their own receipts, and both existed because
`decisions.md §5` was wrong (see the `[SUPERSEDED]` banner on that decision).
They are split deliberately: `spell-consumer-delta-probe` built and proved the
instrument while leaving `classify()` byte-identical, so no unit could move on
an unproven instrument; `ground-spell-units` then applied it. The second card's
ceiling is stated two ways because only one of them is `done`: **623** keys
clear the probe's bar, but only the **46** whose `wiring_class` is `computed`
reach `done` under the producer's verdict table. The other 577 land on `held`,
which is not `done`.

### A note on `verify-this-run`

Adversarial verification of `inventory-determinism`,
`spell-consumer-delta-probe` and `ground-spell-units`, run against the whole
window `0dbbcf4d..21cf3998` rather than against their receipts. Ceiling **0 by
design**: a cycle that verifies the board and also moves it proves nothing.
**Nothing was reversed** — every `grounded` unit in the window was admitted on
an observed consumer delta whose refusal paths were made to fire by deliberate
mutation, no bar moved anywhere in the window, and the run's true net is
`done` **+46** / `held` **−46** / everything else **+0**, independently
re-derived through the producer's own `doneness_verdict()`. Five findings
recorded, none of them a reversal, headed by `F-DASHBOARD-STALE` (the board was
reading a checkout 12 commits behind, so the earned +46 was not visible on it).
See `progress.md`.

## Not on this board, deliberately

- ~~A spell consumer-delta probe. `decisions.md §5`: it moves 178 units into a
  *worse* bucket and none to `done`.~~ **RETRACTED 2026-08-13** by the
  `ground-spell-units` receipt. The 178 is the `NO_GROUNDING_PROBE` cap
  population, which only a *producer* edit could move; grounding never passes
  through that cell. Measured: `done` +46, `held` −46, **0** units into any
  worse bucket. Both cards are on the board above and both are `COMPLETE`.
- Any card that edits the producer, the dashboard JSON, `doneness_meaning`,
  `NO_GROUNDING_PROBE` or `EXCLUDED_BOOKS`. `decisions.md §1`, `§2`, `§6`.
- The 3,547 `unmeasurable` units. `forward-scope-register.md F1`.
- `not-started` content ingestion. `decisions.md §7`.

## Cycle claims

1. Set `Status` to `IN-FLIGHT`.
2. Fill `Claimed-by`, `Claimed-at`, `Cycle-id`.
3. Commit the claim before any other write, so a concurrent cycle sees it.
4. On close, set `COMPLETE` and land the receipt in `progress.md` in the same commit.
