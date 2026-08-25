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
| 4 | `denominator-gate` | 1 | AT-33-E1-004 | complete | `artifacts/epic-1-instruments/AT-33-E1-004_cycle_receipt.md`; remediation (gate was RED at `AT-33-E6-001`'s scan) — `artifacts/epic-1-instruments/AT-33-E1-004-remediation_cycle_receipt.md` |
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
| 16 | `reverify-fixture-verified` | 5 | AT-33-E5-001 | complete | `artifacts/epic-5-reverification/AT-33-E5-001_cycle_receipt.md`. **Finalized by `AT-33-E5-finalize`**: `fixture-verified.combined-oracle-results.json` carries **1,741 of 1,741** rows (100% of the `fixture-verified` population — 1,128 prior + 598 `spell-remainder` + 15 `charbuild-remainder`, 0 duplicate `unit_id`s), 396 agree / 1,345 unverifiable (each with a populated `reason`) / **0 disagree**. Re-derive: `python3 -c "import json,collections; d=json.load(open('artifacts/epic-5-reverification/fixture-verified.combined-oracle-results.json')); print(len(d['results'])); print(collections.Counter(r['verdict'] for r in d['results']))"` → `1741`, `Counter({'unverifiable': 1345, 'agree': 396})`. |
| 17 | `reverify-literal-verified` | 5 | AT-33-E5-002 | in-progress | `artifacts/epic-5-reverification/AT-33-E5-002_cycle_receipt.md`. **Finalized by `AT-33-E5-finalize`**: `literal-verified.oracle-results.json` carries **6,198 of 6,589** rows (5,812 prior + 217 `spell-remainder` + 103 `equipment-remainder` + 66 `charbuild-remainder`), 207 agree / 5,991 unverifiable (each with a populated `reason`) / **0 disagree**. **391 of 6,589 remain genuinely unrowed** — the `equipment-remainder` lane's own named `other_bonus_shape`/`equipment_modifier` remainder (`VAR` 108, `COMBAT` 92, `STAT_multi_or_other_slot` 43, `SITUATION` 34, `SAVE` 24, `WEAPON` 18, +smaller shapes — `AT-33-E5-remainder-equipment_cycle_receipt.md`'s own next-cycle plan). Re-derive: `python3 -c "import json,collections; d=json.load(open('artifacts/epic-5-reverification/literal-verified.oracle-results.json')); print(len(d['results'])); print(collections.Counter(r['verdict'] for r in d['results']))"` → `6198`, `Counter({'unverifiable': 5991, 'agree': 207})`. **Not marked complete**: 391 short of the 6,589 denominator is a real, named gap, not a false 100%. |
| 18 | `disagreement-resolution` | 5 | AT-33-E5-003 | in-progress | `artifacts/epic-5-reverification/AT-33-E5-003_cycle_receipt.md`. **Finalized by `AT-33-E5-finalize`**: `AT-33-E5-003.combined-oracle-results.json` — **0 of 7,939 examined units disagree** (re-derive: `python3 -c "import json,collections; d=json.load(open('artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json')); print(len(d['results'])); print(collections.Counter(r['verdict'] for r in d['results']))"` → `7939`, `Counter({'unverifiable': 7336, 'agree': 603})`). This wave's 2 NEW disagreements were each root-caused and **fixed with a real engine commit** (`progress.md`'s two dedicated entries): `ring_of_the_sea_strider` (`src/rules_core/equipment_effects/general.rs`, PF1's swim-speed-grants-+8-racial-Swim-bonus auto-rule) and `monk_ac_bonus` (`src/rules_core/pilot_compute/mod.rs`, the level-4+ dodge-bonus progression). **Kept `in-progress`, not `complete`**: this criterion's "every disagreement" claim inherits row 17's 391-unit gap — a unit with no row at all has not been checked for disagreement either way, matching `AT-33-E6-001` attempt 2's own precedent ("row 18 inherits" row 17's shortfall). |
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
