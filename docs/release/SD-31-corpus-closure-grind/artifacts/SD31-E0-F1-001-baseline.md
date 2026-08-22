---
cycle-id: SD31-E0-F1-001
card: epic-0-reachability-audit
commit: eadb263f7d6b7f124a45547aa0a5a6f77ab2db9c
generated: 2026-08-15
oracle_pin: 7f818006e371188e5717fd18d74d18a420747fc6
---

# SD31-E0 — Reachability Audit Baseline Run

**Commit this baseline was produced at:** `eadb263f7d6b7f124a45547aa0a5a6f77ab2db9c` (`feat(sd31):
SD31-E0-F1 — scripts/reachability_audit.py, standing gate`, `tranche/11`).

**Exact command:**

```
python3 scripts/reachability_audit.py \
  --json-out docs/release/SD-31-corpus-closure-grind/artifacts/SD31-E0-F1-001-baseline.json \
  > docs/release/SD-31-corpus-closure-grind/artifacts/SD31-E0-F1-001-baseline.txt 2>&1
```

Full human-readable output: `SD31-E0-F1-001-baseline.txt`. Machine-readable result:
`SD31-E0-F1-001-baseline.json`. Source document: `docs/work-inventory.json`
(`generated_at 2026-08-15T01:34:18Z`, cross-checked against this package's own `progress.md` "Board
baseline re-derive" entry — same snapshot, unchanged since).

## Headline

**Reachable ceiling: 94.53 % (36,412 / 38,521)** — matches `decisions.md §5`'s cited figure exactly,
now independently reproduced by the audit script itself rather than carried by source pending the
script's existence.

Per-kind:

| kind | reachable ceiling |
|---|---:|
| equipment_modifier | 100.00 % |
| monster | 99.69 % |
| class | 99.46 % |
| equipment | 99.05 % |
| companion | 98.53 % |
| monster_ability | 98.42 % |
| spell | 97.82 % |
| feat | 96.70 % |
| class_feature | 92.99 % |
| race_trait | 79.98 % |
| race | 52.43 % |

**Caution on the `race`/`race_trait` per-kind figures above.** These are the grid's own number —
"does this unit's wiring_class have ANY status that reaches `done`" — and it reads deceptively high
for `race`/`race_trait` because most of those units carry a non-`ambiguous` wiring_class (e.g.
`static`, `display`), which DOES have a done-reaching status in the grid. The grid cannot see the
actual blocker: `SD-30 decisions.md §44`'s missing `RaceCorpus` chassis (`RaceCorpus::resolve`
returning `None`) is a kind-specific structural gap entirely outside the `wiring_class`/`status`
two-axis model, not a grid dead end. The **known populations** section below is the honest number for
these two kinds — re-derived directly, not through the grid.

## Dead-end cells (grid-based)

All 9 non-zero-or-zero dead-end cells found are in the `ambiguous|*` row — `wiring_class == ambiguous`
never reaches `done` at any status, confirmed live (not transcribed) by evaluating
`_doneness_verdict_uncapped` over the full 5×9 grid:

| cell | reason | on-board units |
|---|---|---:|
| `ambiguous\|not-ingested` | no-done-path | 1,501 |
| `ambiguous\|grounded` | no-done-path | 278 |
| `ambiguous\|unknown` | no-done-path | 119 |
| `ambiguous\|text-complete` | no-done-path | 94 |
| `ambiguous\|not-started` | no-done-path | 89 |
| `ambiguous\|ingested-magnitude` | no-done-path | 28 |
| `ambiguous\|deferred-with-reason` | no-done-path | 0 |
| `ambiguous\|fixture-verified` | no-done-path | 0 |
| `ambiguous\|literal-verified` | no-done-path | 0 |

**`unmapped_cells_with_units`: empty.** Zero cells raise `ValueError` with on-board units — confirms,
against the live corpus rather than only the fabricated grid `test_pf1e_dashboard_producer.py`'s
`test_full_grid_yields_no_unmapped_cells` already checks, that the S5-drift remediation (`(ambiguous,
literal-verified|fixture-verified) -> held`, blocker B6, commit `d636c922d`) closed every unmapped
cell that could carry real units.

**Sum check:** 1,501 + 278 + 119 + 94 + 89 + 28 + 0 + 0 + 0 = **2,109**, matching
`ambiguous_wiring_class_units` below exactly (every `ambiguous` unit, at any status, is dead-end —
consistent with the grid computing "no status for this wiring_class reaches done").

## Known populations (re-derived this cycle, not transcribed)

| population | authoring-time figure (`decisions.md §4`, `epic-breakdown.md` Epic 0-F2) | re-derived this cycle |
|---|---:|---:|
| `wiring_class == ambiguous` (no path to `done` at any status) | ~2,109 | **2,109** — exact match |
| `unmeasurable` / `status == unknown` | 3,989 (`class_feature` 3,622 + `feat` 367) | **3,989** (`class_feature` **3,622** + `feat` **367**) — exact match |
| `race` not-done | 103 units at 0 % | **103 / 103 not-done (0.00 % done)** — exact match |
| `race_trait` not-done | ~3,284 | **3,181** — **drifted from the authoring-time estimate**, see correction below |

## Correction: `race_trait` not-done drifted from the authoring-time ~3,284 estimate

The authoring-time figure (`epic-breakdown.md` Epic 0-F2, `decisions.md §4`) states `race_trait`
~3,284 not-done. Re-deriving live against `docs/work-inventory.json` at this cycle's HEAD:

```
python3 -c "
import json, sys
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d = json.load(open('docs/work-inventory.json'))
U = [u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS and u.get('kind')=='race_trait']
notdone = sum(1 for u in U if P.doneness_verdict(u.get('wiring_class') or 'ambiguous', u.get('status') or 'unknown', 'race_trait') != 'done')
print(len(U), 'total,', notdone, 'not-done,', len(U)-notdone, 'done')
"
# -> 3447 total, 3181 not-done, 266 done
```

**Actual: 3,447 total, 266 done, 3,181 not-done** — 103 fewer not-done units than the ~3,284
authoring-time estimate (real progress landed on `race_trait` between authoring and this cycle: 266
units are now `done`, not 0). `retro.py correction` emitted for this figure (see progress.md).

## Ownership — every dead-end / known-gap population assigned, none unowned

Per `kanban.md` "Deferral is not available to a cycle" and `decisions.md §3`: every population this
baseline reports is already assigned to a named epic in `epic-breakdown.md` — **no Structural
Exclusion Register proposal is needed this cycle**:

| population | owning epic | citation |
|---|---|---|
| `ambiguous` wiring_class (2,109 units, all 9 grid dead-end cells) | **Epic 2 — Verdict-Path Capability** | `kanban.md` row: "`ambiguous` dead-end closed or registered"; `decisions.md §2` item 4: Epic 2's target is the union of `unmeasurable` + `ambiguous`, ~5,979 units |
| `unmeasurable`/`status == unknown` (3,989: `class_feature` 3,622 + `feat` 367) | **Epic 2 — Verdict-Path Capability** | same citation — the other half of Epic 2's ~5,979-unit union target |
| `race` not-done (103/103) | **Epic 1 — Race Chassis, 100 % mandate** | `epic-breakdown.md` Epic 1 objective: "the `race` kind itself (103 units, 0 % done)" |
| `race_trait` not-done (3,181) | **Epic 1 — Race Chassis, 100 % mandate** | `epic-breakdown.md` Epic 1 objective: "~2,894 of the corpus's 3,447 `race_trait` units" (chassis-absence population); the remainder of the not-done population is covered as Epic 1's ceiling-release (Epic 1-F3) opens Epic 6-F4 ingest per race batch |

No row in this table is unowned; no SER proposal logged this cycle.
