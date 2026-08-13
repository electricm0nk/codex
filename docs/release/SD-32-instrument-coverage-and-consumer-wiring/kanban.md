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
| `e1-measurement-gate` | READY | Measurement-Gate Decision Request | 0 (gates 7,479) | — | — | — | — |
| `e2-probe-coverage` | READY | Equipment-Effect Probe Coverage Extension | 358 | **1** / 358 | — | — | — |
| `e4-classifier-calibration` | READY | Classifier — hand-labelled sample (E4-F1, gate) | 0 (gates 1,776) | 2a | — | — | — |
| `e3-effect-wiring` | READY (gated on e2-probe-coverage) | Equipment Effect Wiring for Inert Items | 375 | **3** / 125 | — | — | — |
| `e4-classifier` | READY (gated on e4-classifier-calibration opening the gate) | Wiring-Class Classifier (E4-F2, E4-F3) | 1,776 | **2** / 444 ceiling | — | — | — |
| `e5-static-sweep` | BLOCKED (decision) — `decisions.md §2` | Static Corpus-Literal Byte-Equality Sweep | 4,805 | 0 today (1,602 if gate opens) | — | — | — |
| `e6-derived-check` | BLOCKED (decision) — `decisions.md §2` | Derived Evaluator-vs-Fixture Check | 2,674 | 0 today (535 if gate opens) | — | — | — |
| `e7-structural-report` | READY | Structural-Block Report | 0 (by design) | — | — | — | — |
| `e8-code-review` | READY (gated on e2, e3, e4, e7) | Bundle Code Review | — | — | — | — | — |
| `e9-closure` | READY (gated on every other claimable card) | Closure | — | — | — | — | — |

## Not on this board, deliberately

- A spell consumer-delta probe. `decisions.md §5`: it moves 178 units into a
  *worse* bucket and none to `done`. `forward-scope-register.md F2`.
- Any card that edits the producer, the dashboard JSON, `doneness_meaning`,
  `NO_GROUNDING_PROBE` or `EXCLUDED_BOOKS`. `decisions.md §1`, `§2`, `§6`.
- The 3,547 `unmeasurable` units. `forward-scope-register.md F1`.
- `not-started` content ingestion. `decisions.md §7`.

## Cycle claims

1. Set `Status` to `IN-FLIGHT`.
2. Fill `Claimed-by`, `Claimed-at`, `Cycle-id`.
3. Commit the claim before any other write, so a concurrent cycle sees it.
4. On close, set `COMPLETE` and land the receipt in `progress.md` in the same commit.
