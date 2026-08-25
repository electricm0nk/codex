---
canonical: true
owner: god-emporer
bundle_id: SD-33
status: in progress — Epic 1 dispatched, rows 1-2 (AT-33-E1-001, AT-33-E1-002) complete
date: 2026-08-24
---

# SD-33 Progress

Live cycle-by-cycle record. Cycles **prepend** their entry (newest first) and update `kanban.md` in the same commit, via `workflow-instruction.md §5`'s retry protocol.

## Status

**Launch gates passed 2026-08-25** (`technical-requirements.md §1`, `workflow-instruction.md §1`):

1. SD-32's closure PR merged to `develop` — PR #376 MERGED, `origin/develop` = `f53b8e32da`
2. SD-32's instrument debt closed **inside SD-32** — 29 total / 0 open deferrals, `EXCLUDED_BOOKS = frozenset()`
3. `tranche/13` cut from `develop` and pushed — `origin/tranche/13` = `f652db7ac7`

Epic 1 dispatched; cycles 1-2 (`AT-33-E1-001` row 1, `AT-33-E1-002` row 2) landed.

**Cards complete: 2 / 21** (`jq` re-derive: count `complete` rows in `kanban.md`'s table body).

## Cycle entry schema

Each entry states, at minimum:

- criterion ID and card number
- commit SHA(s)
- **every figure with the command that produces it and its denominator** (`decisions.md §2`)
- **movement in four buckets** — closure / reclassification / reachability / instrument-correction
- receipt path

## Open blockers

None. **This section is not a parking lot.** An entry here is a request for an operator ruling and it **pauses the bundle** (`../../governance/blocker-closure-doctrine.md`). It is never a disposition, never a closure path, and no later cycle may proceed past a blocked card on its own authority.

## Cycles

### Cycle AT-33-E1-002 — box-fail-closed (row 2, Epic 1)

- **Criterion:** `AT-33-E1-002` — `box_ledger.py` fails closed on all five conditions.
- **Files:** `scripts/box_ledger.py` (extended), `scripts/tests/test_box_ledger.py` (extended), `THE-BOX.md` (extended — `"unverifiable"` field on every ledger group).
- **What landed:** conditions 3-5 (oracle disagreement / an `unverifiable` unit dispositioned `done` / `derived_at` staleness gate) added to the same `box_ledger.py` `AT-33-E1-001` built; conditions 1-2 (uncovered/overlap) were already implemented and are re-verified here, not re-implemented.
- **Figures:**
  - `box_ledger.py --check` on the committed `THE-BOX.md` → `uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False` (`python3 scripts/box_ledger.py --check`)
  - Unit test suite: 25 passed, 0 failed (`python3 -m unittest scripts.tests.test_box_ledger -v`) — 9 carried from the prior cycle, 16 new
  - `unknown` (unverifiable) group population, used in two of the five mutation proofs: 4,224 of 49,438 (`jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json`)
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 / instrument-correction 0 — this cycle extends the instrument, moves no unit.
- **RED→GREEN:** 12 `AttributeError`s + 3 `AssertionError`s before implementation (intended reason: the three new mechanisms didn't exist); 25/25 green after. **Five live mutation proofs, one per condition, against the real committed `THE-BOX.md` (or a temp copy with exactly one mutation)** — full transcripts in the receipt: (1) `unknown` group deleted → `uncovered=4224`, exit 1; (2) a colliding group added → `overlap=5099`, exit 1; (3) a real `--oracle-results` fixture with one `verdict: disagree` record → `oracle_disagreement=1`, exit 1, corrected to `agree` → exit 0; (4) the real `unknown` group's disposition changed `unverifiable`→`done` (its own `unverifiable: true` flag left on — reproduces SD-32's exact over-claim) → `unverifiable_done=4224`, exit 1; (5) `derived_at` replaced with a fabricated SHA → `STALE:` + exit 1, committed file's real SHA → exit 0. Every RED case's corresponding GREEN is the untouched committed file, exit 0.
- **Notes:** oracle-disagreement check is wired now (reads `AT-33-E2-003`'s harness output shape), not deferred — it activates automatically once Epic 2 lands `oracle-results.json`, no second cycle needed. `"unverifiable"` is a ledger-group-level flag, not a per-unit field, because `uncovered==0 overlap==0` already makes a unit's disposition equal to its one group's disposition.
- **Test scoping:** ran `scripts/tests/test_box_ledger.py` only (25/25 green) — this criterion's whole file-touch set. Did not run `scripts/verify.sh` (`AT-33-E1-004`'s stage, not yet wired), the Rust workspace, or `apps/desktop/src-tauri` (no files in either changed).
- **Receipt:** `artifacts/epic-1-instruments/AT-33-E1-002_cycle_receipt.md`.

### Cycle AT-33-E1-001 — box-partition (row 1, Epic 1)

- **Criterion:** `AT-33-E1-001` — `THE-BOX.md` exists as a living partition of the full inventory.
- **Files:** `scripts/box_ledger.py` (new), `scripts/tests/test_box_ledger.py` (new), `THE-BOX.md` (new).
- **Figures:**
  - population = 49,438 (`jq '.units | length' docs/work-inventory.json`; cross-checked against `jq '.totals.units' docs/work-inventory.json`, both agree — no correction needed)
  - `box_ledger.py --check` → `uncovered=0 overlap=0 population=49438` (`python3 scripts/box_ledger.py --check`)
  - 9 groups partition the population by the inventory's `status` field (already exhaustive and non-overlapping — 9 distinct non-null values, 0 duplicate unit ids); group counts: `grounded` 3,234, `literal-verified` 6,589, `fixture-verified` 1,741, `ingested-magnitude` 1,543, `text-complete` 5,099, `deferred-with-reason` 46, `not-ingested` 26,943, `not-started` 19, `unknown` 4,224 — each command in `THE-BOX.md`'s table, sum = 49,438.
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 / instrument-correction 0 — this cycle builds the instrument, moves no unit.
- **Explicit `unverifiable` bucket** (`decisions.md §7`): the `unknown` group, 4,224 units — owned by Epic 4 to move.
- **RED→GREEN:** `python3 -m unittest scripts.tests.test_box_ledger` failed with `ModuleNotFoundError` before `box_ledger.py` existed (intended reason); 9/9 green after, including the live-corpus acceptance case. Mutation proof: `THE-BOX.md` copy with the `unknown` group deleted correctly failed closed (`uncovered=4224`, exit 1); committed file untouched.
- **Receipt:** `artifacts/epic-1-instruments/AT-33-E1-001_cycle_receipt.md`.
