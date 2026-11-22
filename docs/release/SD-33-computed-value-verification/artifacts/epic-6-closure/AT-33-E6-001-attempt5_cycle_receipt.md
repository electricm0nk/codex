# Cycle AT-33-E6-001 (attempt 5) — epic-6-closure / AT-33-E6-001

- **Rust suite:** `cargo test --locked --lib equipment_effects` (the only `src/rules_core/` surface changed this wave, at `abc72f75ec`) → `test result: ok. 70 passed; 0 failed; 0 ignored; 2770 filtered out` — **70 of 70** green.
- **Commit SHA:** recorded below at push time (`sd33-r4-acceptance-scan`, remediation wave 4)
- **Files touched:** `docs/release/SD-33-computed-value-verification/artifacts/epic-6-closure/AT-33-E6-001-attempt5_cycle_receipt.md`, `docs/release/SD-33-computed-value-verification/kanban.md` (row 19 note only), `retro/events/*.jsonl`
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (scan-only cycle; no `src/` or corpus writes)
- **Wired-integration audit result:** OK_NO_TOKENS (scan-only cycle)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "Every criterion `AT-33-E1-001` … `AT-33-E5-003` is `complete`, and every `kanban.md` card is `complete`. **A card at `returned-to-backlog`, `in-progress`, or `complete`-with-a-deferred-half blocks closure.** **If anything is short, the cycle stops here** — no retrospective, no sweep, **no PR**. Report what is short with the command that shows it. That is a correct outcome, not a failure."

## Gate result: **FAIL** (attempt 5). Two shortfalls, both pre-existing and both narrowed since attempt 4.

No retrospective, no sweep, no PR. Kanban row 19 stays `blocked-escalated`.

## Shortfall 1 — 67 of 8,330 blessed units carry no oracle row

Down from 75 of 8,330 at attempt 4 (and 391 of 8,330 at attempt 3). This is a
membership check, not a count check: the missing ids were derived as
`docs/work-inventory.json`'s own `literal-verified` + `fixture-verified` id set
minus every `unit_id` present in the merged results file.

Command:

```
python3 -c "import json
wi=json.load(open('docs/work-inventory.json'))['units']
pop={u['id'] for u in wi if u.get('status') in ('literal-verified','fixture-verified')}
c=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json'))['results']
print(len(pop), len(c), len(pop-{x['unit_id'] for x in c}))"
```

Output: `8330 8263 67` — population 8,330 of 8,330 blessed units; 8,263 of 8,330
rowed; **67 of 8,330 missing**. Composition: 56 of 67 `equipment`, 11 of 67
`equipment_modifier`, spanning 10 books; the full per-shape breakdown is in the
`AT-33-E5-last75` lane's own receipt (that lane rowed 8 of its 75-unit
population and reported `blocked-escalated`, correctly).

## Shortfall 2 — 4 of 8,263 examined units still `disagree`

Down from 26 of 8,255 at attempt 4. Command:

```
python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json; echo $?
```

Output: `uncovered=0 overlap=0 population=49438 oracle_disagreement=4 unverifiable_done=0 stale=False`, then
`ORACLE_DISAGREEMENT: advanced_class_guide:equipment:full_plate_of_the_corpse, inner_sea_world_guide:equipment:field_plate, inner_sea_world_guide:equipment:stoneplate, ultimate_equipment:equipment:snakeskin_tunic`, exit `1`.

`AT-33-E5-003` is therefore unmet, and its kanban row 18 is `blocked-escalated`;
row 17 is `in-progress`. Both states block closure by the criterion's own words.

## The 22 fixes were REAL, not hidden — this attempt's primary check

All 26 attempt-4 disagreements were diffed unit-by-unit against the previous
committed revision of the merged file (`git log -n1 --skip=1 -- <path>`):

```
python3 -c "<prev-vs-current diff, prints prev oracle/ours vs current verdict/oracle/ours per unit>"
```

- **22 of 26 now `agree`, every one by `ours` moving to the oracle's already-recorded value.** e.g. `hallowed_chain_greater` 6→9 against an unchanged oracle 9; `diviner_s_blight` 2→6 against unchanged oracle 6; `sea_knife` −2→0 against unchanged oracle 0; all 10 `stalking_armor_*` 3→5 against unchanged oracle 5.
- **0 of 26 dropped from the file.** **0 of 26 reclassified to `unverifiable`.**
- **1 of 26 showed a moved top-level `oracle` value** — `panoply_of_the_fierani_knight`, prev `oracle=3 ours=6`, now `oracle=11 ours=11`. Investigated and cleared: the unit is a merged multi-shape record; its `combat-weapon` shape's `oracle=11` **predates wave 4** (committed at `f66ae64320`, then `ours=9 oracle=11 disagree`), and its `var-bonus` shape is now `3/3` (ours moved 6→3). The top-level value changed because the merge rule surfaces a different shape, not because an expected value was edited. Command: `git show f66ae64320:docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-shape-combat.oracle-results.json`.
- The compute fix is real code at `abc72f75ec` — new `equipment_effects::eqmod_referenced_records` resolver + `apply_eqmod_armor_class_bonus` / `apply_eqmod_var_bonus`, plus a `TYPE=Circumstance` exclusion. `grep -n "eqmod_referenced_records" src/rules_core/equipment_effects.rs` → lines 92, 540, 551.
- Route taken was **our-compute**, not harness — so the "harness fixed but prior verdicts not re-run" illegitimate path does not apply. The 4 remaining are the harness-limitation route, and they are **escalated and still visible as `disagree`**, not hidden.
- **26 of 26 disagreements have a `progress.md` entry.** Command: `python3 -c "…prev disagree ids vs progress.md text…"` → `disagreements 26 without a progress.md mention 0 []`.

## `disagree` capability re-proven live on the current batch path

Two live probes, both removed afterwards (in-memory / `/tmp` only, nothing committed):

1. Batch comparison path — `scripts.oracle_harness.compare.run_comparison` fed a known-mismatching pair returned `[{"unit_id":"probe:unit:a","ours":5,"oracle":9,"verdict":"disagree"},{"unit_id":"probe:unit:b","ours":7,"oracle":9,"verdict":"disagree"}]`.
2. Ledger detector — a `/tmp` copy of the merged file with `sea_knife` mutated to `ours=999 verdict=disagree` produced `oracle_disagreement=5` naming `sea_knife` alongside the 4, exit 1. The unmutated file gives 4.

## Re-verified CLOSED (not re-litigated)

| Item | Command | Output |
|---|---|---|
| Row 16 population | `python3 -c "…len fixture-verified.combined…"` | `rows 1741 distinct 1741 pop 1741`, 396 agree / 1,345 unverifiable / 0 disagree — **1,741 of 1,741** |
| Duplicate `unit_id`s | dupe counter over the merged file | `dupes 0` of 8,263 rows |
| Reasonless `unverifiable` | same script | `reasonless_unverifiable 0` of 7,501 unverifiable rows |
| `box_ledger.py` structural | as above | `uncovered=0 overlap=0 population=49438` |
| work-inventory `unknown` | `jq '[.units[]\|select(.status=="unknown")]\|length' docs/work-inventory.json` | `0` |
| Denominator gate green | `scripts/verify.sh --only denominator-gate` | `PASS denominator-gate (files_checked=40 violations=0)`, `RESULT: PASS`, exit 0 |
| Gate scope still wide | `DEFAULT_GLOBS` read at `scripts/denominator_gate.py:105` | receipts under `artifacts/**/*_cycle_receipt.md` **plus** 8 root package documents (`README`, `decisions`, `epic-breakdown`, `release-notes`, `scope-draft`, `kanban`, `THE-BOX`, `progress`) — **40 of 40 files checked** |
| Gate not blinded | live probe: a temporary in-scope `_probe_cycle_receipt.md` carrying one bare hundred-percent token (spelled out here so this receipt does not itself trip the gate) plus "26 units were fixed" | `VIOLATION …_probe_cycle_receipt.md:3` on that exact line; `files_checked=41 violations=1`; probe removed, back to `files_checked=40 violations=0`. Confirmed a second time when this very receipt's first draft quoted the probe line verbatim and the gate flagged **1 of 41** files. |
| No code carve-outs in closure instruments | `grep -nE "EXCLUD\|SKIP_\|IGNORE_\|_ALLOWLIST\|BLESSED\|beginner_box" scripts/box_ledger.py scripts/denominator_gate.py scripts/oracle_harness/*.py` | no hits; and `EXCLUDED_BOOKS: frozenset[str] = frozenset()` (empty) at `scripts/observer/pf1e_dashboard_producer.py:3519` |
| Epic 3 artifact at the SD-33 path | `ls artifacts/epic-3-engine-coverage/` | `formula_interpreter.corpus-wide.json` present |
| SD-32's file untouched | `git log --oneline -1 -- docs/release/SD-32-*/artifacts/gate-2-engines/` | `d5cbf1f801 docs(sd32): …` — an SD-32 commit; no SD-33 commit touches it |
| Receipts exist at kanban-stated paths | existence loop over every `` `artifacts/…_cycle_receipt.md` `` in `kanban.md` | no `MISSING` lines |

## Deferral posture

`python3 scripts/retro.py query --type deferral` enumerates the open set. One
SD-33 deferral **does** cover DoD scope and is therefore itself part of shortfall
1, not an accepted deferral: `2026-08-25T11:25:41Z sd33-r4-last75 — "67 of the 75
last-unrowed equipment/equipment_modifier units remain unexamined after this
cycle"`. It carries a named revisit condition (row the remaining 67) and is
correctly reported rather than folded into done, but it is a gap, not a closure.

- **Figures + their re-derive commands:** every figure above is in a row or block that carries both its denominator and its command. Headline: **8,263 of 8,330** blessed units rowed; **67 of 8,330** unrowed; **4 of 8,263** examined units `disagree` (was 26 of 8,255); **22 of 26** attempt-4 disagreements fixed at the compute; **1,741 of 1,741** fixture-verified rows; **0 of 8,263** duplicate `unit_id`s; **40 of 40** documents scanned by the denominator gate.
- **Status:** blocked-escalated
- **Movement, four buckets:**
  - *closure*: 22 of 26 disagreements genuinely fixed at the compute (`abc72f75ec`); 8 of 75 previously-unrowed units rowed.
  - *reclassification*: none this cycle. Verified none happened in the fixes either — 0 of 26 moved to `unverifiable`, 0 dropped.
  - *reachability*: none.
  - *instrument-correction*: none this cycle; the 4 remaining disagreements are a **named instrument limitation** (`baseline_diff_harness_limitation` — a whole-character `AC.TOTAL` diff cannot separate an item's own AC bonus from a `MAXDEX`-cap Dex loss or a co-located Dex-enhancement chain), not an engine defect, and the correcting instrument (an `AC.Armor`-isolating oracle probe) is not yet built.
- **Notes:** Attempt 5 is the fifth consecutive correct halt. Both remaining shortfalls are the *bundle working*: 26 real wrong computed values were found and 22 were fixed at the source. The scan deliberately did not attempt either remaining fix — an acceptance scanner that executes is no longer an acceptance scanner.
- **Next-cycle plan:** (1) build the `AC.Armor`-isolating (or fixed-baseline) oracle probe in `scripts/oracle_harness/` and re-run the full population per `AT-33-E5-003`'s own re-run clause, resolving the 4; (2) row the remaining 67 of 8,330; (3) re-run `AT-33-E6-001` as attempt 6.
