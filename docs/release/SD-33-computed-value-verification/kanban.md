---
canonical: true
owner: god-emporer
bundle_id: SD-33
date: 2026-08-24
board: local-file (Hermes board retired 2026-08-01, SD-30 Decision 14a)
---

# SD-33 Kanban

One row per acceptance criterion. **21 rows.** A cycle marks its row `complete` from inside the dispatched agent (`workflow-instruction.md §6` step 8).

**Status vocabulary:** `not-started` | `in-progress` | `complete` | `blocked-escalated`.

**There is no `returned-to-backlog` and no `deferred`.** A blocker on the Definition of Done is cleared or escalated (`../../governance/blocker-closure-doctrine.md`). `blocked-escalated` means an operator ruling has been requested and the bundle is **paused** — it is not a closure state and it does not satisfy AT-33-E6-001.

**Row hygiene (SD-32 hazard):** keep per-cycle narrative in `progress.md` and the cycle receipt, **not in the Notes column.** SD-32's rows grew into 32KB single physical lines that no editor could safely modify and that broke every naive parser. Notes here are a pointer, never a story.

| # | Card | Epic | Criterion | Status | Notes (pointer only) |
|---|---|---|---|---|---|
| 1 | `box-partition` | 1 | AT-33-E1-001 | complete | `artifacts/epic-1-instruments/AT-33-E1-001_cycle_receipt.md` |
| 2 | `box-fail-closed` | 1 | AT-33-E1-002 | complete | `artifacts/epic-1-instruments/AT-33-E1-002_cycle_receipt.md` |
| 3 | `probe-surface-census` | 1 | AT-33-E1-003 | complete | `artifacts/epic-1-instruments/AT-33-E1-003_cycle_receipt.md` |
| 4 | `denominator-gate` | 1 | AT-33-E1-004 | complete | `artifacts/epic-1-instruments/AT-33-E1-004_cycle_receipt.md`; remediation (gate was RED at `AT-33-E6-001`'s scan) — `artifacts/epic-1-instruments/AT-33-E1-004-remediation_cycle_receipt.md`; scope-widening (wave 3, `AT-33-E6-001` attempt 3's unscanned-bundle-root finding) — `artifacts/epic-1-instruments/AT-33-E1-004-scope-widening_cycle_receipt.md` |
| 5 | `oracle-path-a-feasibility` | 2 | AT-33-E2-001 | complete | `artifacts/epic-2-oracle-harness/AT-33-E2-001_cycle_receipt.md` |
| 6 | `oracle-character-roundtrip` | 2 | AT-33-E2-002 | complete | `artifacts/epic-2-oracle-harness/AT-33-E2-002_cycle_receipt.md` |
| 7 | `oracle-comparison-harness` | 2 | AT-33-E2-003 | complete | `artifacts/epic-2-oracle-harness/AT-33-E2-003_cycle_receipt.md` |
| 8 | `oracle-path-ruling` | 2 | AT-33-E2-004 | complete | `artifacts/epic-2-oracle-harness/AT-33-E2-004_cycle_receipt.md` |
| 9 | `coverage-gap-rootcause` | 3 | AT-33-E3-001 | complete | `artifacts/epic-3-engine-coverage/AT-33-E3-001..004_cycle_receipt.md` |
| 10 | `coverage-f1` | 3 | AT-33-E3-002 | complete | `artifacts/epic-3-engine-coverage/AT-33-E3-001..004_cycle_receipt.md` |
| 11 | `coverage-f2-f9` | 3 | AT-33-E3-003 | complete | `artifacts/epic-3-engine-coverage/AT-33-E3-001..004_cycle_receipt.md` |
| 12 | `coverage-100-with-denominator` | 3 | AT-33-E3-004 | complete | `artifacts/epic-3-engine-coverage/AT-33-E3-001..004_cycle_receipt.md` |
| 13 | `unknown-rootcause` | 4 | AT-33-E4-001 | complete | `artifacts/epic-4-unknown-classification/AT-33-E4-001_cycle_receipt.md` |
| 14 | `unknown-to-zero` | 4 | AT-33-E4-002 | complete | `artifacts/epic-4-unknown-classification/AT-33-E4-002_cycle_receipt.md` |
| 15 | `no-effort-named-buckets` | 4 | AT-33-E4-003 | complete | `artifacts/epic-4-unknown-classification/AT-33-E4-003_cycle_receipt.md` |
| 16 | `reverify-fixture-verified` | 5 | AT-33-E5-001 | complete | `artifacts/epic-5-reverification/AT-33-E5-001_cycle_receipt.md`. **Finalized by `AT-33-E5-finalize`**: `fixture-verified.combined-oracle-results.json` carries **1,741 of 1,741** rows (100% of the `fixture-verified` population — 1,128 prior + 598 `spell-remainder` + 15 `charbuild-remainder`, 0 duplicate `unit_id`s), 396 agree / 1,345 unverifiable (each with a populated `reason`) / **0 disagree**. Re-derive: `python3 -c "import json,collections; d=json.load(open('artifacts/epic-5-reverification/fixture-verified.combined-oracle-results.json')); print(len(d['results'])); print(collections.Counter(r['verdict'] for r in d['results']))"` → `1741`, `Counter({'unverifiable': 1345, 'agree': 396})`. **Confirmed unaffected by remediation wave 3** (`AT-33-E5-finalize-wave3`): no wave-3 lane examined a `fixture-verified` unit; still `1741`/`1741`, still stays `complete`. |
| 17 | `reverify-literal-verified` | 5 | AT-33-E5-002 | in-progress | `artifacts/epic-5-reverification/AT-33-E5-002_cycle_receipt.md`. **Finalized by `AT-33-E5-finalize-wave3`** (supersedes `AT-33-E5-finalize`'s own 6,198-row figure below): `literal-verified.oracle-results.json` now carries **6,514 of 6,589** rows (6,198 prior + 331 raw rows from wave 3's `var`/`combat`/`stat-save-tail` lanes, minus 15 real cross-lane duplicate `unit_id`s root-caused and merged — see `progress.md`'s `AT-33-E5-finalize-wave3` entry and `artifacts/epic-5-reverification/finalize-wave3-duplicate-unit-ids.json`), 339 agree / 6,149 unverifiable (each with a populated `reason`) / **26 disagree** (all newly root-caused this wave, none fixed — see row 18). Re-derive: `python3 -c "import json,collections; d=json.load(open('artifacts/epic-5-reverification/literal-verified.oracle-results.json')); print(len(d['results'])); print(collections.Counter(r['verdict'] for r in d['results']))"` → `6514`, `Counter({'unverifiable': 6149, 'agree': 339, 'disagree': 26})`. **75 of 6,589 remain genuinely unrowed** — re-derived directly from `docs/work-inventory.json`'s own `literal-verified` id set minus every id in the merged file (not inferred from a count), classified by real corpus record read: `WEAPON` 23, `SKILL` 17, `WEAPONPROF` 15, `COMBAT` 7, `VAR` (equipment_modifier) 5, `EQMWEAPON` 3, `SITUATION` 2, `EQM` 1, `MOVEADD` 1, `STAT` 1 — full detail `artifacts/epic-5-reverification/finalize-wave3-missing-literal-shapes.json`. **Not marked complete**: 75 short of the 6,589 denominator is a real, named gap, not a false 100%. (Superseded prior text, kept for history: "6,198 of 6,589 rows ... 391 of 6,589 remain genuinely unrowed".) |
| 18 | `disagreement-resolution` | 5 | AT-33-E5-003 | in-progress | `artifacts/epic-5-reverification/AT-33-E5-003_cycle_receipt.md`. **Finalized by `AT-33-E5-finalize-wave3`** (supersedes `AT-33-E5-finalize`'s own 0-disagree figure below — wave 3's lanes introduced 26 real, new disagreements after that prior finalize ran): `AT-33-E5-003.combined-oracle-results.json` — **26 of 8,255 examined units disagree** (re-derive: `python3 -c "import json,collections; d=json.load(open('artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json')); print(len(d['results'])); print(collections.Counter(r['verdict'] for r in d['results']))"` → `8255`, `Counter({'unverifiable': 7494, 'agree': 735, 'disagree': 26})`). **All 26 root-caused this cycle, per-unit table in `progress.md`'s `AT-33-E5-finalize-wave3` entry; none fixed this cycle** — 21 share one named engine gap (`compute_arms_armor_effect`/`compute_var_effect` do not resolve and sum a base item's `EQMOD:`-referenced modifier record's own separate `BONUS:` chain — the same gap `AT-33-E5-remainder-equipment` first named, now confirmed recurring), 3 are a harness baseline-diff methodology limitation, 1 is an unhandled `PRE`-gated conditional chain, 1 is not yet individually diagnosed. Real engine fix scoped as next-cycle's top item, not attempted rushed inside this merge/finalize cycle — box_ledger.py's fail-closed gate independently confirms all 26 (`oracle_disagreement=26`, exit 1), proving disagree-detection capability on the current, unmodified batch path. **Kept `in-progress`, not `complete`**: 26 real unresolved disagreements plus row 17's own 75-unit gap (a unit with no row has not been checked for disagreement either way — this row's own long-standing inheritance rule). (Superseded prior text, kept for history: the wave-2 finalize's own 2-fixed-disagreement/0-remaining figure, now stale.) |
| 19 | `final-acceptance-scan` | 6 | AT-33-E6-001 | blocked-escalated | attempt 1 `artifacts/epic-6-closure/AT-33-E6-001_cycle_receipt.md`; attempt 2 `artifacts/epic-6-closure/AT-33-E6-001-attempt2_cycle_receipt.md`; attempt 3 `artifacts/epic-6-closure/AT-33-E6-001-attempt3_cycle_receipt.md` (gate FAIL — row 16 CLOSED at 1,741 of 1,741 rows; row 17 short at 6,198 of 6,589 rows, row 18 inherits at 7,939 of 8,330; denominator gate + deferral posture re-verified green) |
| 20 | `retrospective-written-and-cited` | 6 | AT-33-E6-002 | not-started | |
| 21 | `sweep-archdocs-graphify-pr` | 6 | AT-33-E6-003 | not-started | |

## Gating

```
Epic 1 (rows 1-4)
   ├──> Epic 2 (rows 5-8)   ─┐
   ├──> Epic 3 (rows 9-12)   ├─ parallel, worktree-isolated
   └──> Epic 4 (rows 13-15) ─┘
                 Epic 2 ──> Epic 5 (rows 16-18)
        all of the above ──> Epic 6 (rows 19-21)
```

**Rows 5–15 run concurrently.** Every agent in that wave gets `isolation: 'worktree'` (`workflow-instruction.md §3`).
