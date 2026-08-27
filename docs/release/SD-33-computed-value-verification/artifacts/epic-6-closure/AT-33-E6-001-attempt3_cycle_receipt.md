# Cycle AT-33-E6-001-attempt3 — Epic 6 Closure epilogue / AT-33-E6-001

- **Commit SHA:** recorded on this receipt's own landing commit (see `git log -1 -- docs/release/SD-33-computed-value-verification/artifacts/epic-6-closure/AT-33-E6-001-attempt3_cycle_receipt.md`)
- **Files touched:** `docs/release/SD-33-computed-value-verification/artifacts/epic-6-closure/AT-33-E6-001-attempt3_cycle_receipt.md` (this file); `kanban.md` row 19 pointer only. **No code, no data, no instrument was modified** — this is a scan, not an executor cycle.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (scoped to this cycle's touched files; the only changes are two markdown files)
- **Wired-integration audit result:** OK_NO_TOKENS (scoped to this cycle's touched files)
- **Acceptance criterion (verbatim, `workflow-instruction.md §11` step 1):** "Final-acceptance scan. Every criterion and every `kanban.md` card at `complete`. **Never 'complete *or* filed under `## Open blockers`'.** If anything is short, **stop** — no retrospective, no sweep, **no PR**; report what is short with the command that shows it."
- **Status:** blocked-escalated
- **Gate result: FAIL (attempt 3).** The bundle is **not** closable. No retrospective was written, no worktree sweep was run, no PR was opened — per §11 step 1, correctly.

---

## 1. The row count — the check that matters most

Run for real at `f8f82a61fb`:

```
$ python3 -c "import json,collections
for p,pop in [('fixture-verified.combined-oracle-results.json',1741),
              ('literal-verified.oracle-results.json',6589),
              ('AT-33-E5-003.combined-oracle-results.json',8330)]:
  d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/'+p))
  k='results' if 'results' in d else [x for x in d if isinstance(d[x],list)][0]
  r=d[k]; print(p,'rows',len(r),'distinct',len({x.get('unit_id') for x in r}),'pop',pop,
    dict(collections.Counter(x.get('verdict') or x.get('status') for x in r)))"

fixture-verified.combined-oracle-results.json rows 1741 distinct 1741 pop 1741 {'agree': 396, 'unverifiable': 1345}
literal-verified.oracle-results.json           rows 6198 distinct 6198 pop 6589 {'agree': 207, 'unverifiable': 5991}
AT-33-E5-003.combined-oracle-results.json      rows 7939 distinct 7939 pop 8330 {'agree': 603, 'unverifiable': 7336}
```

| File | Rows carried | Denominator | Short by |
|---|---|---|---|
| `fixture-verified.combined-oracle-results.json` | 1,741 | 1,741 | **0 — CLOSED** |
| `literal-verified.oracle-results.json` | 6,198 | 6,589 | **391 of 6,589** |
| `AT-33-E5-003.combined-oracle-results.json` | 7,939 | 8,330 | **391 of 8,330** |

`fixture-verified` reached its full denominator this wave — that is a real closure and attempt 2's row-16 half of the shortfall is genuinely gone. `literal-verified` did not.

## 2. Prior shortfalls — what this wave closed and what it did not

| Attempt-2 shortfall | Closed? | Command |
|---|---|---|
| Row 16 / `AT-33-E5-001` short of 1,741 (1,128 of 1,741 rows) | **YES** — 1,741 of 1,741 rows, 0 duplicate `unit_id` | the §1 command, line 1 |
| Row 17 / `AT-33-E5-002` short of 6,589 | **NO** — 6,198 of 6,589 rows; **391 of 6,589 units carry no oracle row at all** | the §1 command, line 2 |
| Row 18 / `AT-33-E5-003` inherits row 17 | **NO** — 7,939 of 8,330 rows; a unit with no row has not been checked for disagreement either way | the §1 command, line 3 |
| Denominator gate green + detection live | **YES** (re-verified this attempt, §4) | `scripts/verify.sh --only denominator-gate` |
| Deferral posture: 2 open of 8, both genuine capability deferrals with revisit conditions, 0 covering DoD scope | **YES** (re-verified this attempt, §5) | `scripts/retro.py summary --since 2026-08-24 --json` |

## 3. The blocking shortfall, named

**391 of 6,589 `literal-verified` units carry no `(ours, oracle, verdict)` row.** They are the `equipment-remainder` lane's own named, un-attempted `other_bonus_shape` / `equipment_modifier` shapes, decomposed in `AT-33-E5-remainder-equipment_cycle_receipt.md`'s shape census: `VAR` 108, `COMBAT` 92, `STAT_multi_or_other_slot` 43, `SITUATION` 34, `SAVE` 24, `WEAPON` 18, `WEAPONPROF=*` 15, plus 11 smaller shapes totalling 20 units. That lane examined 103 of its own 494-unit population and named the rest as not attempted.

Board state, re-read at scan time:

```
$ grep -nE '^\| 1[678] ' docs/release/SD-33-computed-value-verification/kanban.md | cut -c1-120
| 16 | `reverify-fixture-verified`  | 5 | AT-33-E5-001 | complete    | ...
| 17 | `reverify-literal-verified`  | 5 | AT-33-E5-002 | in-progress | ...
| 18 | `disagreement-resolution`    | 5 | AT-33-E5-003 | in-progress | ...
```

```
$ grep -nE '^\- \*\*Status' docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-finalize_cycle_receipt.md
167:- **Status:** in-progress
```

Rows 17 and 18 are `in-progress`, and the finalize cycle's own receipt reports `in-progress`. Under `kanban.md`'s status vocabulary and §11 step 1, `in-progress` is not a closure state. **This blocks.** There is no "complete OR filed under Open blockers" path.

## 4. Everything else the scan checked — all green

| Check | Command | Result |
|---|---|---|
| No reasonless `unverifiable` | per-file scan of `verdict=='unverifiable'` with empty/missing `reason` | 0 of 1,741, 0 of 6,198, 0 of 7,939 |
| No unresolved `disagree` | per-file scan of `verdict=='disagree'` | 0 of 1,741, 0 of 6,198, 0 of 7,939 |
| Historical disagreements traced | `ring_of_the_sea_strider` and `monk_ac_bonus` both now `agree` (16/16 and 7/7), fixed at `9de465ee12` | traced to a real engine commit |
| No duplicate `unit_id` across merged files | `fixture ∩ literal` | overlap 0; union 7,939 equals the combined file's 7,939 rows exactly |
| Disagree-detection path still live | `box_ledger.py --check --oracle-results <synthetic disagree>` | `oracle_disagreement=1`, `ORACLE_DISAGREEMENT: probe:equipment:known_case`, exit 1; probe removed |
| `box_ledger.py --check` on the real combined file | `python3 scripts/box_ledger.py --check` | `uncovered=0 overlap=0 population=49438 oracle_disagreement=0 stale=False`, exit 0 |
| Denominator gate green | `scripts/verify.sh --only denominator-gate` | `PASS (files_checked=23 violations=0)`, exit 0 |
| Denominator gate still catches a bare percentage | in-scope probe receipt containing a bare percentage with no denominator | `VIOLATION ...:3`, `files_checked=24 violations=1`, exit 1; probe removed, gate back to `files_checked=23 violations=0` exit 0 |
| `work-inventory.json` unknowns | `jq '[.units[]\|select(.status=="unknown")]\|length' docs/work-inventory.json` | 0 |
| Epic 3 artifact at the SD-33 path | `ls .../artifacts/epic-3-engine-coverage/formula_interpreter.corpus-wide.json` | present, 18,860 bytes |
| SD-32's `gate-2-engines/` untouched | `git log -1 --date=short -- docs/release/SD-32-*/artifacts/gate-2-engines/` | last touched `d5cbf1f801`, 2026-08-22 — before `tranche/13` was cut; untouched by SD-33 |
| Carve-outs in closure instruments | grep for exclusion/skip/blacklist lists in `box_ledger.py`, `denominator_gate.py`, `pf1e_dashboard_producer.py` | `EXCLUDED_BOOKS = frozenset()`, `EXCLUDED_BOOKS_REASONS = {}`; `denominator_gate.py` and `box_ledger.py` carry none. The gate's narrow default target set is documented in its own module docstring as deliberate scope, not a carve-out |

**One scope note, not a defect.** `denominator_gate.py`'s `DEFAULT_GLOBS` covers this bundle's `artifacts/**/*_cycle_receipt.md` plus `progress.md` — it does **not** scan `kanban.md` or the bundle's planning prose. That narrowing is stated in the script's own docstring with its rationale. A first probe placed at the bundle root was silently not scanned; the detection re-proof in the table above was therefore re-run with an **in-scope** probe, which the gate caught. Recording this because a probe outside the scanned set returning green is exactly the shape of a false all-clear.

## 5. Deferral posture — green

```
$ python3 scripts/retro.py summary --since 2026-08-24 --json | python3 -c "import json,sys; d=json.load(sys.stdin)['deferrals']; print('total:',d['total'],'open:',d['open'])"
total: 8 open: 2
```

Both open deferrals enumerated, both from `sd33-e4-unknown`:

1. **Widen `REGISTERED_POOL_GROUPS`** in `src/rules_core/class_feature_pool_catalog.rs` to resolve more of the 3,052 not-ingested `class_feature` units to a real class owner. Revisit condition: *"a future cycle with corpus-research time budgeted specifically for class_feature pool ownership."* A capability deferral — pool ownership was never in SD-33's Definition of Done.
2. **Recognize `status=="unmeasurable"`** in `pf1e_dashboard_producer.py`'s `_doneness_verdict_uncapped()`. Revisit condition: *"whichever cycle next touches `scripts/observer/pf1e_dashboard_producer.py`."* Out of `AT-33-E4`'s granted write scope; the function raises loudly on the unrecognized status by design, so it is disclosed rather than silently swallowed.

Both carry a named revisit condition. **Neither defers SD-33 DoD scope.** This half of attempt 2's finding stays closed.

## 6. Figures + their re-derive commands

| Figure | Denominator | Re-derive command |
|---|---|---|
| 1,741 rows carried | of 1,741 `fixture-verified` units | §1's command, line 1 |
| 6,198 rows carried | of 6,589 `literal-verified` units | §1's command, line 2 |
| 7,939 rows carried | of 8,330 Epic 5 units | §1's command, line 3 |
| 391 units with no oracle row | of 6,589 `literal-verified` units | `6589 - 6198`, both from §1's command |
| 603 agree | of 7,939 examined units | §1's command, line 3 |
| 7,336 unverifiable, each with a populated reason | of 7,939 examined units | §1's command, line 3, plus the reasonless-scan in §4 |
| 0 disagree | of 7,939 examined units | §1's command, line 3 |
| 0 duplicate `unit_id` | of 7,939 merged rows | the union/overlap check in §4 |
| 2 open deferrals | of 8 total in the SD-33 window | §5's command |
| 0 unknown units | of `docs/work-inventory.json`'s units | `jq '[.units[]\|select(.status=="unknown")]\|length' docs/work-inventory.json` |
| 23 files checked, 0 violations | denominator gate's documented default target set | `scripts/verify.sh --only denominator-gate` |

## 7. Movement, four buckets

- **Closure:** row 16 / `AT-33-E5-001` reached its full 1,741-of-1,741 denominator. The disagreement bucket reached 0 of 7,939 examined units with both new disagreements fixed by real engine commits at `9de465ee12`. The reasonless-`unverifiable` bucket reached 0 of 7,939.
- **Reclassification:** none this cycle. This is a scan; it moved no unit between buckets.
- **Reachability:** none this cycle.
- **Instrument-correction:** one recorded — the denominator gate's default target set does not cover the bundle root, so an out-of-scope probe returns a false green. Detection was re-proven with an in-scope probe. No instrument was modified.

## 8. Notes — judgment calls

- **Reports were not accepted as evidence.** Every figure in the three-lane wave report and in the `AT-33-E5-finalize` receipt was re-derived from the JSON files directly. The lane reports' own numbers reconcile with the files, but the `literal-verified` shortfall the wave left open is not visible from any single lane's summary line — only from counting the merged rows against the population.
- **`fixture-verified` closing is real and worth banking.** Attempt 2 failed on two halves; one is genuinely gone. Reporting it as still-short would be as wrong as reporting the bundle closed.
- **Both prior scans were right to halt, and so is this one.** The remaining 391 of 6,589 is a sequencing problem with an already-written decomposition, not an exemption. `AT-33-E5-remainder-equipment_cycle_receipt.md`'s next-cycle plan names the cheapest lever first (`SAVE`, 24 units, mapping to the already-proven `CHECK.<i>.TOTAL`/`.NAME` loop) and names one `src/rules_core` fix that unlocks two shapes at once (splitting a comma-joined qualifier, clearing the 27 multi-skill/`ALL SKILL` units and the 43 `STAT_multi_or_other_slot` units together).
- **The equipment lane named its own throughput ceiling.** Each `./gradlew run` spins its own Gradle daemon, so `-P15` did not reach the `-P20` direct-java speedup a sibling lane achieved. A direct-java wrapper is the named lever for the next wave and is likely a precondition for clearing `VAR` (108) and `COMBAT` (92) at reasonable cost.

## 9. Next-cycle plan

**Do not re-run this scan until `literal-verified.oracle-results.json` carries 6,589 of 6,589 rows.** The next dispatch is an executor wave, not a scanner:

1. **`SAVE` (24 of 391)** — maps directly to `AT-33-E2-002`'s proven `CHECK.<i>.TOTAL`/`.NAME` loop. Cheapest lever, do it first.
2. **`STAT_multi_or_other_slot` (43 of 391) + the 27 multi-skill/`ALL SKILL` units** — one `src/rules_core` fix (split the comma-joined qualifier in `compute_magic_items_effect`) unlocks both.
3. **A direct-java wrapper** replacing the per-unit `./gradlew run` invocation, to lift the lane's throughput ceiling before the two large shapes.
4. **`VAR` (108 of 391) and `COMBAT` (92 of 391)** — the two largest shapes, each needing its own per-sub-shape census first.
5. **`SITUATION` (34 of 391), `WEAPON` (18 of 391), `WEAPONPROF=*` (15 of 391), and the 11 smaller shapes (20 of 391 combined)** — several are likely genuine `unverifiable` shapes once examined, which is a first-class disposition and still requires a real row with a populated reason.
6. **Then** re-run `AT-33-E6-001` as attempt 4.

Rows 17 and 18 move to `complete` only when row counts equal their denominators exactly. Row 19 stays `blocked-escalated`.
