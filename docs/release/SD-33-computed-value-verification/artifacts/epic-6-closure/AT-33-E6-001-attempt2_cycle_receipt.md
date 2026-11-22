# Cycle AT-33-E6-001 (attempt 2) — Epic 6 Closure / final-acceptance scan, re-run

- **Commit SHA:** recorded by the commit that lands this receipt on `tranche/13` (scan base: `3fdac906bf`)
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-6-closure/AT-33-E6-001-attempt2_cycle_receipt.md` (new)
  - `docs/release/SD-33-computed-value-verification/kanban.md` (row 19 Notes pointer -> attempt-2 receipt; status stays `blocked-escalated`)
  - `docs/retro/events/sd33-r-acceptance-scan.jsonl` (one `incident`)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — this cycle's diff is docs plus one retro event; no `src/`, `scripts/`, `apps/`, or `data/` change.
- **Wired-integration audit result:** `OK_NO_TOKENS` — no shipping code touched. This is a read-only scan.
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**

  > ### AT-33-E6-001 — final-acceptance scan
  >
  > Every criterion `AT-33-E1-001` … `AT-33-E5-003` is `complete`, and every `kanban.md` card is `complete`. **A card at `returned-to-backlog`, `in-progress`, or `complete`-with-a-deferred-half blocks closure.**
  >
  > **If anything is short, the cycle stops here** — no retrospective, no sweep, **no PR**. Report what is short with the command that shows it. That is a correct outcome, not a failure.

## Gate result: **FAIL** (second consecutive)

The bundle stops here again. **No retrospective was written. No worktree sweep was run. No PR was opened.** Rows 20 and 21 were left `not-started` and untouched. The scanner did not fix anything it found — it is the scanner, not an executor.

Two of the first scan's four shortfalls are genuinely closed. Two are not.

## The first scan's four shortfalls, re-verified one at a time

### Shortfall 1 — rows 16 and 17 `complete` over their FULL populations: **NOT CLOSED**

Row 16 is still `in-progress`. Row 17 reads `complete` but its per-unit row file does not cover its population.

```
$ grep -E '^\| 1[6-8] \|' docs/release/SD-33-computed-value-verification/kanban.md | cut -c1-96
| 16 | `reverify-fixture-verified` | 5 | AT-33-E5-001 | in-progress | ...
| 17 | `reverify-literal-verified` | 5 | AT-33-E5-002 | complete | ...
| 18 | `disagreement-resolution` | 5 | AT-33-E5-003 | complete | ...
```

The lane receipts agree with the board and did not over-claim:

```
$ for f in docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-00{1,2,3}_cycle_receipt.md; do printf "%-34s " "$(basename $f)"; grep -m1 -E '^## Status' "$f"; done
AT-33-E5-001_cycle_receipt.md      ## Status: in-progress
AT-33-E5-002_cycle_receipt.md      ## Status: complete
AT-33-E5-003_cycle_receipt.md      ## Status: complete
```

**The per-unit rows were COUNTED, not read off a receipt's summary line** — this is the check the criterion asks for by name:

```
$ python3 -c "
import json
for p,pop in [('fixture-verified.combined-oracle-results.json',1741),('literal-verified.oracle-results.json',6589),('AT-33-E5-003.combined-oracle-results.json',8330)]:
    d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/'+p))
    k='results' if 'results' in d else [x for x in d if isinstance(d[x],list)][0]
    rows=d[k]; ids={r.get('unit_id') for r in rows}
    from collections import Counter; c=Counter(r.get('verdict') or r.get('status') for r in rows)
    print(p,'rows=',len(rows),'distinct_unit_id=',len(ids),'population=',pop,dict(c))"
fixture-verified.combined-oracle-results.json rows= 1128 distinct_unit_id= 1128 population= 1741 {'agree': 382, 'unverifiable': 746}
literal-verified.oracle-results.json          rows= 5812 distinct_unit_id= 5812 population= 6589 {'agree': 41, 'unverifiable': 5771}
AT-33-E5-003.combined-oracle-results.json     rows= 6940 distinct_unit_id= 6940 population= 8330 {'agree': 423, 'unverifiable': 6517}
```

- **Row 16: 1,128 of 1,741** fixture-verified units carry a per-unit row. **613 of 1,741 carry none at all.** Board status `in-progress`; the dispatched lane itself returned `blocked-escalated`. Both values block closure under the criterion's own vocabulary.
- **Row 17: 5,812 of 6,589** literal-verified units carry a per-unit row. **777 of 6,589 carry none at all.** The row is marked `complete` while a named 777-unit remainder of its own population is undispositioned — this is exactly the `complete`-with-a-deferred-half shape the criterion names as blocking. The remainder is honestly named per-shape in the lane's own receipt, which is why this is a scoping shortfall and not a false claim.

Real movement since attempt 1 is large and is recorded here rather than flattened: fixture-verified examined went **11 of 1,741 to 1,128 of 1,741**; literal-verified went **21 of 6,589 to 5,812 of 6,589**. Neither reached its population.

### Shortfall 2 — row 18 `complete` over the full examined population: **PARTIALLY CLOSED, still blocks**

The disagreement half is genuinely closed. The 103 disagreements the fixture lane surfaced were root-caused and fixed at `dded72f0b4`, and **0 of 6,940** examined units now disagree — counted from the committed file above, not quoted from the receipt.

What still blocks is the denominator underneath it. Row 18's own examined population is **6,940 of 8,330** (8,330 = 1,741 fixture-verified + 6,589 literal-verified, the Epic 5 population named in `epic-breakdown.md`); **1,390 of 8,330** units have no oracle row anywhere and therefore cannot have been examined for disagreement.

```
$ python3 -c "print('E5 population 8330; examined 6940; unexamined', 8330-6940)"
E5 population 8330; examined 6940; unexamined 1390
```

Row 18 cannot stand `complete` over Epic 5's population while rows 16 and 17 — the cards that would supply the other 1,390 — are `in-progress` and `complete`-over-a-partial respectively. Row 18's internal arithmetic is sound (6,940 = 1,128 + 5,812); it is short only because its inputs are.

### Shortfall 3 — denominator gate green AND detection power still real: **CLOSED**

The stage passes:

```
$ scripts/verify.sh --only denominator-gate
==> denominator-gate — python3 scripts/denominator_gate.py --check
    PASS  denominator-gate  (files_checked=18 violations=0)
RESULT: PASS
$ python3 scripts/denominator_gate.py --check >/dev/null 2>&1; echo $?
0
```

**The gate was NOT fixed by being blinded — re-proven live by this scanner against the real CLI, inside the real default scope, not by reading the unit test.** A throwaway receipt was written into `artifacts/epic-6-closure/` (which `DEFAULT_GLOBS` covers) and removed afterwards:

```
counterfactual A — bare percentages, no denominator (must FAIL):
VIOLATION .../ZZ-scanner-counterfactual_cycle_receipt.md:3: Coverage reached 97.9% this cycle.
VIOLATION .../ZZ-scanner-counterfactual_cycle_receipt.md:5: Agreement was 100% and disagreement 0%.
files_checked=19 violations=2   REAL_EXIT=1

counterfactual B — anti-shadowing, real figure sharing a line with the exempted idiom (must FAIL):
VIOLATION .../ZZ-scanner-counterfactual_cycle_receipt.md:3: Not a false 100% shape, but coverage was 88.2% anyway.
violations=1   REAL_EXIT=1

counterfactual C — same figure, denominator stated in the same construct (must PASS):
files_checked=19 violations=0   REAL_EXIT=0

after removing the probe file, baseline restored:
files_checked=18 violations=0   BASELINE_EXIT=0
```

Counterfactual A proves a bare figure still fails, including a bare all-of-it figure written as one hundred percent.
Counterfactual B is the important one: the remediation exempted the bundle's "false hundred-percent" idiom, and B proves that exemption blanks only the idiom's own token — a genuine separate figure sharing the line is still caught in full.
The exemption's whole implementation is one narrow regex, applied by substitution to a scratch copy of the line before `PERCENT_RE` runs, so it cannot suppress anything else:

```
FALSE_100_IDIOM_RE = re.compile(r"\bfalse[\s-]100%", re.IGNORECASE)
```

**Scanner's judgment: this is a matcher-precision fix, not a blinding.**

*(This receipt was itself caught by the gate on first write — the sentence above originally quoted a bare hundred-percent token, and `denominator_gate.py --check` flagged it at exit 1. The prose was reworded; the gate was not touched. That is a fourth, unplanned live proof of its detection power, and it fired on the scanner's own output.)*


### Shortfall 4 — no open deferral defers Definition-of-Done scope: **CLOSED**

Enumerated, not counted:

```
$ python3 scripts/retro.py summary --since 2026-08-24 --json | python3 -c "import json,sys; d=json.load(sys.stdin)['deferrals']; print('total',d['total'],'open',d['open'],'resolved',d['resolved']); [print(i['id'],'|',i['actor'],'|',i['revisit'][:70]) for i in d['open_items']]"
total 8 open 2 resolved 6
1787633115006-sd33-e4-unknown-136912 | sd33-e4-unknown | a future cycle with corpus-research time budgeted specifically ...
1787633121875-sd33-e4-unknown-58d073 | sd33-e4-unknown | whichever cycle next touches scripts/observer/pf1e_dashboard_producer.py
```

**2 open of 8 total**, denominator = deferrals emitted since the SD-33 launch date 2026-08-24. The two that deferred DoD scope at attempt 1 (`sd33-e5-fixture`, `sd33-e5-literal`, covering the then-unexamined 1,730 and 6,568 units) are now **resolved**, moving open from 4 of 8 to 2 of 8. Both survivors carry a named revisit condition, and both are genuine capability deferrals outside this bundle's Definition of Done: widening `REGISTERED_POOL_GROUPS` for class_feature pool ownership, and teaching the dashboard producer to recognise `status == unmeasurable`. Neither gates any AT-33 criterion.

Note for the record: a resolved deferral event is a bookkeeping fact, not evidence the deferred work landed. The units those two deferrals covered were checked directly above in Shortfall 1, by counting rows — 613 of 1,741 and 777 of 6,589 remain undispositioned regardless of the events' state.

## Full scan — everything else, re-run by this scanner

| Check | Result | Denominator | Command |
|---|---|---|---|
| kanban rows 1–15 | all 15 `complete` | of 21 rows; rows 19–21 are Epic 6's own | `grep -cE '^\| [0-9]+ \|.*\| complete \|' docs/release/SD-33-computed-value-verification/kanban.md` |
| kanban rows 16–18 | **row 16 `in-progress`; row 17 `complete` over a partial population; row 18 `complete` over a partial population** — BLOCKS | of 3 Epic 5 rows | `grep -E '^\| 1[6-8] \|' .../kanban.md` |
| `box_ledger.py --check` | `uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False`, exit **0** | population 49,438 = whole inventory | `python3 scripts/box_ledger.py --check` |
| inventory units at `status: unknown` | **0 of 49,438** | total units `jq '.units\|length'` -> 49438 | `jq '[.units[]\|select(.status=="unknown")]\|length' docs/work-inventory.json` |
| Epic 3 corpus-wide coverage | **11,652 of 11,652** (10,626 recognised + 240 refused + 786 unjoined) | of 11,652 formula-bearing units, scope `F1..F9` | `jq '{total_population,total_recognised_units,total_refused_units,total_unjoined_units,scope}' .../artifacts/epic-3-engine-coverage/formula_interpreter.corpus-wide.json` |
| Epic 3 artifact at the SD-33 path | present | — | `ls .../SD-33-.../artifacts/epic-3-engine-coverage/` |
| SD-32 `gate-2-engines/` file UNTOUCHED | last touched by `25dbee17aa feat(sd32): Gate 2 corpus-wide run …` — **no SD-33 commit** | — | `git log --oneline -3 -- docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-2-engines/formula_interpreter.corpus-wide.json` |
| carve-out sweep of the closure **instruments** (code, not prose) | no hardcoded exclusion list in any closure instrument | 6 instruments scanned | `grep -nE '(EXCLUD\|SKIP_\|IGNORE_\|_ALLOWLIST\|_DENYLIST\|EXEMPT)' scripts/box_ledger.py scripts/denominator_gate.py scripts/coverage_ledger.py scripts/shape_ledger.py scripts/retro.py src/bin/v06_work_inventory.rs` |
| `EXCLUDED_BOOKS` still empty | `frozenset()`, `EXCLUDED_BOOKS_REASONS = {}` | — | `grep -n "^EXCLUDED_BOOKS" scripts/observer/pf1e_dashboard_producer.py` |
| receipts exist at kanban-stated paths | **17 of 17** receipt files found | Epic 3's four criteria share one receipt; row 4 carries an extra remediation receipt | `find .../artifacts -name '*_cycle_receipt.md' \| sort` |
| §7 figures row + four-buckets row | present on **17 of 17** receipts | of 17 receipt files | `for f in $(find … -name '*_cycle_receipt.md'); do grep -c 'Figures + their re-derive commands' "$f"; grep -c 'Movement, four buckets' "$f"; done` |

Two observations carried forward from attempt 1, still true, still blocking nothing:

1. `coverage_ledger.py:207` takes `excluded_books: frozenset[str] = frozenset(P.EXCLUDED_BOOKS)` — a parameterised default reading the producer's now-empty set, not a hardcoded list. Clean today; named because it is the shape that concealed `beginner_box` in SD-32.
2. `box_ledger.py --check` prints `INFO: no oracle-results at .../epic-2-oracle-harness/oracle-results.json`. The check is wired and was exercised by AT-33-E5-003 against an explicit `--oracle-results` path; the default path simply holds no file.

Also noted and deliberately not committed: two untracked orchestrator scripts, `artifacts/sd-33-dispatch.workflow.js` and `artifacts/sd-33-remediation.workflow.js`, left as-is per `workflow-instruction.md §5`.

## Test scoping

This cycle ran **no cargo suites** — it is a read-only scan and touched no Rust. Executed: `scripts/verify.sh --only denominator-gate` (one stage, plus three live counterfactual runs of `scripts/denominator_gate.py --check` inside the real default scope), `python3 scripts/box_ledger.py --check`, `jq` over `docs/work-inventory.json` and the Epic 3 artifact, `python3 scripts/retro.py summary`, direct row-counting over the three Epic 5 oracle-result JSON files, and `git log`/`git show`. **Not run:** the root `cargo test` sweep, `scripts/verify.sh` in full mode, and the separate `apps/desktop/src-tauri` cargo workspace — none was assumed covered by anything else. Attempt 1's remediation receipt reports `scripts/verify.sh` in full mode stalling at stage 8 of 40 (`site-dashboard-check`) with no verdict; that remains **unknown**, not cleared, and this scanner did not re-attempt it.

- **Figures + their re-derive commands:** every figure above sits in a table row or a fenced block carrying its denominator and the command that produces it. The blocking figures: **row 16 at `in-progress` of 21 kanban rows**; **1,128 of 1,741** fixture-verified units with a per-unit row (**613 of 1,741** with none); **5,812 of 6,589** literal-verified units with a per-unit row (**777 of 6,589** with none); **6,940 of 8,330** Epic 5 units examined for disagreement (**1,390 of 8,330** unexamined). The closed figures: **0 violations of 18 files checked** by the denominator gate, with detection power re-proven by three live counterfactuals; **2 open of 8 total** deferrals since 2026-08-24, **0 of 2** covering Definition-of-Done scope.
- **Status:** blocked-escalated
- **Movement, four buckets:**
  - **closure:** 0 — a scan moves no unit.
  - **reclassification:** 0
  - **reachability:** 0
  - **instrument-correction:** 0 — the denominator gate's return to green was performed by AT-33-E1-004's remediation cycle at `3fdac906bf`; this scanner only re-proved it, and corrected no instrument itself.
- **Notes:**
  - The criterion forbids "complete *or* filed under `## Open blockers`". Nothing here was filed as an open blocker to route around; the two surviving shortfalls are reported as short.
  - Row 19 stays **`blocked-escalated`**. Per `kanban.md`'s own vocabulary note that pauses the bundle and requests a ruling; it does not satisfy AT-33-E6-001.
  - Rows 20 and 21 remain `not-started`, deliberately untouched — they are this criterion's downstream and are excluded from the scan's own population.
  - This scanner fixed nothing it found, by design.
- **Next-cycle plan:** **Not** AT-33-E6-002. The bundle re-enters Epic 5 for a second remediation wave, now much smaller than the first. Two named populations remain, both already decomposed by their own lanes into concrete sub-populations with structural reasons rather than throughput excuses:
  1. **AT-33-E5-001, 613 of 1,741** — 598 spell units with no casting-ability mapping in this engine (the lane's committed `spell_unresolved` array is the exact worklist; first establish whether any consumer for their magnitude is modelled at all before attempting oracle comparison), plus 15 class_feature units needing a probe binary against the full pilot-compute pipeline and one L20 `.pcg` per source class.
  2. **AT-33-E5-002, 777 of 6,589** — 448 equipment other-bonus-shape units (SKILL 124 is the cheapest next lever, reusing the existing `CHECK.<i>.TOTAL` export token; then COMBAT 94, VAR 134, WEAPON 22, SAVE 29, SITUATION 44, multi-ability/other-slot STAT 43, 18 smaller shapes) plus 329 non-equipment probe-bearing units (spell 217, equipment_modifier 46, race 36, class_feature 17, race_trait 13).

  Then re-run AT-33-E5-003 over the full 8,330, then re-run AT-33-E6-001 a third time. Per `../../../governance/blocker-closure-doctrine.md` neither remainder is an exemption candidate — each is a sequencing problem with a named decomposition, and the decomposition already exists.
