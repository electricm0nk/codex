---
canonical: true
owner: god-emporer
bundle_id: SD-33
status: in progress — Epic 1 dispatched, rows 1-3 (AT-33-E1-001, AT-33-E1-002, AT-33-E1-003) complete
date: 2026-08-24
---

# SD-33 Progress

Live cycle-by-cycle record. Cycles **prepend** their entry (newest first) and update `kanban.md` in the same commit, via `workflow-instruction.md §5`'s retry protocol.

## Status

**Launch gates passed 2026-08-25** (`technical-requirements.md §1`, `workflow-instruction.md §1`):

1. SD-32's closure PR merged to `develop` — PR #376 MERGED, `origin/develop` = `f53b8e32da`
2. SD-32's instrument debt closed **inside SD-32** — 29 total / 0 open deferrals, `EXCLUDED_BOOKS = frozenset()`
3. `tranche/13` cut from `develop` and pushed — `origin/tranche/13` = `f652db7ac7`

Epic 1 dispatched; cycles 1-3 (`AT-33-E1-001` row 1, `AT-33-E1-002` row 2, `AT-33-E1-003` row 3) landed.

**Bundle-level figure (`AT-33-E1-003`'s own evidence bar, not a footnote):** of the corpus's **19**
distinct `kind` values (`jq -r '.units[].kind' docs/work-inventory.json | sort -u | wc -l`), **8**
carry a probe capable of verifying a computed magnitude and **11** do not
(`python3 scripts/probe_surface_census.py --check` → `kinds_with_probe=8 kinds_without_probe=11`).
Of the 11: 8 have no engine table at all (`ability`, `template`, `deity`, `power`, `domain`,
`skill`, `language`, `trait`), and 3 have an engine table but only a presence/lookup check, never a
computed-delta observation (`monster`, `monster_ability`, `companion`) — see
`artifacts/epic-1-instruments/probe-surface-census.json` and its cycle receipt for the full
per-kind table and the source citations.

**Cards complete: 3 / 21** (`jq` re-derive: count `complete` rows in `kanban.md`'s table body).

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

### Cycle AT-33-E1-003 — probe-surface-census (row 3, Epic 1)

- **Criterion:** `AT-33-E1-003` — the probe surface is enumerated for real.
- **Files:** `scripts/probe_surface_census.py` (new), `scripts/tests/test_probe_surface_census.py`
  (new), `artifacts/epic-1-instruments/probe-surface-census.json` (new, generated).
- **What landed:** every corpus `kind` (19, live) enumerated by reading `src/bin/v06_work_inventory.rs`'s
  exhaustive verdict match arm-by-arm, cross-checked against live evidence strings for every claim
  (not from memory or prior prose, per `decisions.md §7`). 8 kinds carry a probe function that
  changes an input and observes a delta on a rendered computed snapshot (`class`, `class_feature`,
  `feat`, `spell`, `equipment`, `equipment_modifier`, `race`, `race_trait`); 11 do not — 8 with no
  engine table at all, 3 with an engine table but only a presence/lookup check (`monster`,
  `monster_ability`, `companion`).
- **Figures:**
  - Distinct corpus `kind` count: 19 (`jq -r '.units[].kind' docs/work-inventory.json | sort -u | wc -l`)
  - Kinds with a magnitude probe: **8 of 19**; without: **11 of 19**
    (`python3 scripts/probe_surface_census.py --check` → `kinds_with_probe=8 kinds_without_probe=11`)
  - Units covered by a probe-bearing kind: 34,246 of 49,438
    (`jq '[.units[] | select(.kind | IN("class","class_feature","feat","spell","equipment","equipment_modifier","race","race_trait"))] | length' docs/work-inventory.json`)
  - Units in a no-probe kind: 15,192 of 49,438
    (`jq '[.units[] | select(.kind | IN("monster","monster_ability","companion","ability","template","deity","power","domain","skill","language","trait"))] | length' docs/work-inventory.json`)
  - Per-kind probe-fire confirmation (live, execution-derived): `class`=28, `class_feature`=26,
    `feat`=108, `spell`=966, `equipment`+`equipment_modifier`=605, `race`=39, `race_trait`=309 real
    units each — proving each claimed probe genuinely fired, not merely exists in source. All 11
    no-probe kinds confirmed at 0.
  - Full per-kind table with unit counts and re-derive commands: cycle receipt.
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 /
  instrument-correction 0 — this cycle builds and runs an enumeration instrument; it moves no
  unit's status.
- **RED→GREEN:** `ModuleNotFoundError: No module named 'probe_surface_census'` before the module
  existed (intended reason); 11/11 green after, including 3 live-corpus acceptance tests run
  against the real `docs/work-inventory.json`. 3 mutation proofs on `--check`'s fail-closed gate:
  an unmapped-kind unit, a `probe_exists:true` kind whose only unit never fires the probe, and a
  `probe_exists:false` kind carrying probe-shaped evidence — all three correctly detected and
  reported by name. Regression: `scripts/tests/test_box_ledger.py` re-run, 25/25 still green
  (untouched this cycle).
- **Notes:** presence-only lookups (`monster`/`monster_ability`/`companion`) are deliberately NOT
  counted as magnitude probes even though the inventory's own vocabulary calls their result
  `grounded` — the criterion's bar is "can verify a computed magnitude", and a `holds_key` table
  lookup answers a different, weaker question. See the receipt's Notes for the full reasoning and
  the recursive-find hazard check (confirmed no probe implementation exists outside
  `v06_work_inventory.rs`'s sibling `v06_content_state_dump.rs`, and confirmed `data/corpus/`'s
  extra `*_generic`/`_parity` directory names are storage-layout artifacts, not a 20th kind).
- **Test scoping:** ran `scripts/tests/test_probe_surface_census.py` (11/11) and
  `scripts/tests/test_box_ledger.py` (25/25, regression). Did not run `scripts/verify.sh` (any
  stage — `AT-33-E1-004` owns wiring this cycle's files into it), the Rust workspace, or
  `apps/desktop/src-tauri` (no `.rs` file touched).
- **Receipt:** `artifacts/epic-1-instruments/AT-33-E1-003_cycle_receipt.md`.

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
