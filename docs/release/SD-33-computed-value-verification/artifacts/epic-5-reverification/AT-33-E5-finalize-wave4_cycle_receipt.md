# Cycle AT-33-E5-finalize-wave4 — Epic 5 Re-verification / totals + kanban call (rows 16, 17, 18)

- **Commit SHA:** recorded on landing (see `progress.md` entry `AT-33-E5-finalize-wave4`)
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/finalize-wave4-merge.py` (new — the merge script, run for real, output below)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/literal-verified.oracle-results.json` (merged in place)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json` (merged in place)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixture-verified.combined-oracle-results.json` (verified UNTOUCHED — 0 overlap with either wave-4 lane)
  - `docs/release/SD-33-computed-value-verification/progress.md`, `kanban.md` (updated in place)
  - This receipt
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to `finalize-wave4-merge.py` + the two merged JSON files, diffed against `git merge-base HEAD origin/develop`)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope)
- **Acceptance criteria owned this cycle (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-001 — the 1,741 `fixture-verified` units are re-examined against the oracle
  > **Evidence:** per-unit `(ours, oracle, verdict)` rows committed; agreement and disagreement counts both stated, with the denominator.
  >
  > ### AT-33-E5-002 — the 6,589 `literal-verified` units are re-examined
  > **Evidence:** as above.
  >
  > ### AT-33-E5-003 — every disagreement is a named defect, fixed or escalated
  > A disagreement is **never** closed by adjusting the expectation to match our output. Each is root-caused: either our computation is wrong (fix it) or the oracle comparison is wrong (fix the harness, and re-run everything it already judged). **Evidence:** one entry per disagreement in `progress.md`, each resolved to a commit or an operator escalation. **A filed blocker does not satisfy this criterion.**

## What this cycle owns

Total Epic 5 across wave 4's two lanes (`AT-33-E5-003-disagreement-fixes`: 26→4 disagreements;
`AT-33-E5-last75`: 8 of 75 unrowed units rowed) and own the kanban call on rows 16/17/18. This
cycle writes no `src/rules_core/` code and runs no new PCGen invocations — it merges committed
lane outputs into the three canonical files and re-derives every figure independently, per the
dispatch brief's "reports are not evidence" instruction.

## Step 1 — Merge every lane result file into the canonical artifacts

Both lanes' commits (`abc72f75ec`, `36133b4bc0`, `9d3e0c88da` on the disagreements side;
`41f4982fa2`, `019257b74c` on the last75 side) were already on `origin/tranche/13` at this cycle's
start (`git merge-base --is-ancestor origin/tranche/13 9d3e0c88da` confirmed fast-forward
possible). This tree rebased onto `origin/tranche/13` first.

`finalize-wave4-merge.py` (committed alongside this receipt) merges:

1. `disagreement-fixes.oracle-results.json` (22 rows, all `agree`) — **supersedes** the matching
   stale `disagree` row for the same `unit_id`. Verified before overwrite that all 22 target rows
   were `disagree` pre-merge (never an unrelated overwrite):
   ```
   $ python3 -c "import json,collections
   lit=json.load(open('.../literal-verified.oracle-results.json'))['results']
   dis=json.load(open('.../disagreement-fixes.oracle-results.json'))['results']
   dis_ids={r['unit_id'] for r in dis}
   before=[r for r in lit if r['unit_id'] in dis_ids]
   print(collections.Counter(r['verdict'] for r in before), len(before))"
   Counter({'disagree': 22}) 22
   ```
2. `equipment-last75.oracle-results.json` (8 rows) — pure **addition**. Verified 0 overlap with
   `literal-verified`, `fixture-verified`, or the combined file before writing (the script raises
   and refuses to write on any overlap or post-merge duplicate — see script source).

`fixture-verified.combined-oracle-results.json` — **verified untouched**: neither lane's unit_ids
intersect the fixture-verified population.

**Superseded unit_ids (22, the one sanctioned overwrite):**
`inner_sea_races:equipment:armor_of_grim_triumph`, `inner_sea_races:equipment:coat_of_shells`,
`inner_sea_races:equipment:gnome_scrap_armor`, `inner_sea_races:equipment:hallowed_chain`,
`inner_sea_races:equipment:hallowed_chain_greater`, `inner_sea_races:equipment:hide_of_grim_triumph`,
`inner_sea_races:equipment:mail_of_sly_steps`, `inner_sea_races:equipment:panoply_of_the_fierani_knight`,
`advanced_class_guide:equipment:hero_s_hauberk`, `advanced_class_guide:equipment:stalking_armor_{cold,
desert,forest,jungle,mountain,plains,swamp,underground,urban,water}` (10),
`advanced_class_guide:equipment:tireless_tracking_hide`, `advanced_race_guide:equipment:sea_knife`,
`ultimate_intrigue:equipment:diviner_s_blight`. All 22 moved `disagree` → `agree`.

**Unexpected duplicates found:** 0. The merge script asserts no internal duplicates in either
source file and no post-merge duplicate in either canonical file; it raised nothing.

**Real run output:**
```
$ python3 artifacts/epic-5-reverification/finalize-wave4-merge.py
fixture-verified.combined-oracle-results.json: UNTOUCHED, rows=1741 (verified 0 overlap)
literal-verified: rows=6522 distinct=6522 population=6589 superseded=22 added=8 verdicts={'agree': 362, 'unverifiable': 6156, 'disagree': 4}
combined (AT-33-E5-003): rows=8263 distinct=8263 population=8330 superseded=22 added=8 verdicts={'agree': 758, 'unverifiable': 7501, 'disagree': 4}
```

## Step 2 — Derive the unexamined SET, not its size

```
$ python3 -c "
import json,collections
wi=json.load(open('docs/work-inventory.json'))['units']
lit={u['id'] for u in wi if u.get('status')=='literal-verified'}
fix={u['id'] for u in wi if u.get('status')=='fixture-verified'}
comb=json.load(open('artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json'))['results']
comb_ids={r['unit_id'] for r in comb}
miss=sorted((lit|fix)-comb_ids)
print('lit',len(lit),'fix',len(fix),'missing',len(miss))
print(dict(collections.Counter(m.split(':')[1] for m in miss)))"
lit 6589 fix 1741 missing 67
{'equipment': 56, 'equipment_modifier': 11}
```

**Not empty.** 67 of 8,330 units carry no oracle row (56 `equipment` + 11 `equipment_modifier`,
consistent with the `AT-33-E5-last75` lane's own 75-in/8-rowed/67-remaining shape table — see that
receipt's shape table for the per-shape breakdown of these exact 67 ids). This is the honest
remainder, not a new figure — it is the same population `AT-33-E6-001` attempt 4 named at 75,
minus the 8 the last75 lane rowed.

## Step 3 — Zero unresolved `disagree` — NOT achieved, re-derived

```
$ python3 scripts/box_ledger.py --check --oracle-results artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json
uncovered=0 overlap=0 population=49438 oracle_disagreement=4 unverifiable_done=0 stale=False
ORACLE_DISAGREEMENT: advanced_class_guide:equipment:full_plate_of_the_corpse, inner_sea_world_guide:equipment:field_plate, inner_sea_world_guide:equipment:stoneplate, ultimate_equipment:equipment:snakeskin_tunic
$ echo $?
1
```

**4 of 26 remain, and they are not new.** All 4 are the `baseline_diff_harness_limitation` bucket
the disagreement-fixes lane named and escalated this same wave (`AT-33-E5-003-disagreement-fixes_
cycle_receipt.md`) — the harness's whole-character `AC.TOTAL` diff conflates the item's own AC
bonus with a second-order `MAXDEX`-cap Dex loss or a co-located Dex-enhancement chain, undercounting
the item by exactly the Dex delta. Each is arithmetic-verified against already-committed raw PCGen
exports in that receipt (e.g. `full_plate_of_the_corpse`: `22 (item AC.TOTAL) - 12 (baseline
AC.TOTAL) = 10` naive diff, vs `10 (base) + 11 (armor: 9 base + 2 EQMOD enhancement) + 1 (Dex,
capped 2→1 by MAXDEX:1) = 22` real composition — this engine's `11` is correct, the recorded
`10` is the harness artifact).

**Not suppressed, not closed by moving the expectation.** The doctrine's route for a harness-side
disagreement is "fix the harness, and re-run everything it already judged" — building an
`AC.Armor`-isolating (or fixed-baseline) oracle probe in `scripts/oracle_harness/` and re-running
the full 8,263-row population through it. That is real, live-PCGen, multi-hour work (prior lanes'
own measured throughput: ~20s/invocation even at high parallelism) that does not fit this cycle's
one-turn budget alongside the merge/derivation/receipt work already done. Per `AGENTS.md`'s
Blocker Discipline: this is disposition 2, "raise your hand" — named precisely, with the exact
fix required, not deferred vaguely. **Escalated, not fixed.**

## Step 4 — Zero reasonless `unverifiable`, zero duplicate `unit_id` — CONFIRMED across all three files

```
$ python3 -c "
import json,collections
for f,pop in [('fixture-verified.combined-oracle-results.json',1741),
              ('literal-verified.oracle-results.json',6589),
              ('AT-33-E5-003.combined-oracle-results.json',8330)]:
    d=json.load(open('artifacts/epic-5-reverification/'+f))['results']
    ids=[r['unit_id'] for r in d]
    bad=[r for r in d if r['verdict']=='unverifiable' and not (r.get('reason') or '').strip()]
    print(f,'rows',len(d),'distinct',len(set(ids)),'pop',pop,'reasonless',len(bad),
          'dupes',len(ids)-len(set(ids)),dict(collections.Counter(r['verdict'] for r in d)))"
fixture-verified.combined-oracle-results.json rows 1741 distinct 1741 pop 1741 reasonless 0 dupes 0 {'agree': 396, 'unverifiable': 1345}
literal-verified.oracle-results.json rows 6522 distinct 6522 pop 6589 reasonless 0 dupes 0 {'agree': 362, 'unverifiable': 6156, 'disagree': 4}
AT-33-E5-003.combined-oracle-results.json rows 8263 distinct 8263 pop 8330 reasonless 0 dupes 0 {'agree': 758, 'unverifiable': 7501, 'disagree': 4}
```

## Step 5 — Re-prove `disagree` capability on the CURRENT batch path

A known-agreeing row in the just-merged combined file, mutated to a deliberately-wrong `ours`
value and re-checked through `box_ledger.py` unmodified — proving the comparator still returns
`disagree` on the post-merge file, not just on a pre-merge lane's own isolated output:

```
$ python3 -c "
import json
d = json.load(open('artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json'))
row = next(r for r in d['results'] if r['verdict']=='agree')
row['verdict']='disagree'; row['ours']=999999
json.dump(d, open('/tmp/probe-wave4-disagree.oracle-results.json','w'))
print('mutated unit_id:', row['unit_id'])"
mutated unit_id: ultimate_equipment:equipment:belt_of_mighty_hurling_greater
$ python3 scripts/box_ledger.py --check --oracle-results /tmp/probe-wave4-disagree.oracle-results.json
uncovered=0 overlap=0 population=49438 oracle_disagreement=5 unverifiable_done=0 stale=False
ORACLE_DISAGREEMENT: ultimate_equipment:equipment:belt_of_mighty_hurling_greater, advanced_class_guide:equipment:full_plate_of_the_corpse, inner_sea_world_guide:equipment:field_plate, inner_sea_world_guide:equipment:stoneplate, ultimate_equipment:equipment:snakeskin_tunic
$ echo $?
1
$ rm -f /tmp/probe-wave4-disagree.oracle-results.json
```

5th disagreement (the probe) plus the 4 real ones — the mutated file correctly returns exit 1 and
names both. Probe file lived only under `/tmp`, never committed. `disagree_capability_reproven_
on_batch_path=true`.

## Step 6 — Denominator gate stays green

```
$ bash scripts/verify.sh --only denominator-gate
    PASS  denominator-gate  (files_checked=39 violations=0)
RESULT: PASS
```

Scope was not narrowed to reach this. `files_checked` rose from attempt 4's 36/37 to 39 because
this cycle's own new prose (this receipt + the two lane receipts already on the branch) is inside
the widened glob, and each states every figure with its denominator.

## The kanban call

**Row 16 (`reverify-fixture-verified`, `AT-33-E5-001`): stays `complete`.** 1,741 of 1,741 rowed,
0 disagree, 0 reasonless `unverifiable`. Confirmed unaffected by wave 4 (0 overlap, both lanes).

**Row 17 (`reverify-literal-verified`, `AT-33-E5-002`): `in-progress`, NOT `complete`.**
6,522 of 6,589 rowed — 67 short, real and named (Step 2). A short population, however honestly
labeled, does not satisfy "the 6,589 ... units are re-examined."

**Row 18 (`disagreement-resolution`, `AT-33-E5-003`): `blocked-escalated`, NOT `complete`.**
4 of the original 26 disagreements remain unresolved. They are root-caused (not "undiagnosed" —
attempt 4's residual bucket closes) and escalated with the exact fix named, but "fixed or
escalated" still means the criterion's own text — "each resolved to a commit or an operator
escalation" — is met by an actual escalation record, not by this receipt alone. This receipt
**is** that escalation: the harness-fix-and-rerun scope is named, the population it must cover
(8,263 rows) is named, and the reason it did not happen this cycle (live-PCGen throughput vs.
one-turn budget) is named. Marked `blocked-escalated` rather than `in-progress` because the
remaining work is not "more of the same lane's throughput" — it needs a harness change and a full
re-run, a materially different shape of cycle, matching the disagreement-fixes lane's own status.

## Figures + their re-derive commands

- 8,330 blessed units total (1,741 fixture + 6,589 literal) — `docs/work-inventory.json` status
  counts, command in Step 2.
- 8,263 of 8,330 rowed (99.2%, stated with denominator) — combined file row count, command in
  Step 1's merge output.
- 67 of 8,330 unrowed — Step 2's set-difference command, not inferred from a count.
- 4 of 26 original disagreements remain unresolved — `box_ledger.py --check`, Step 3.
- 22 of 26 fixed this wave (26→4) — the merge's `superseded=22` count, Step 1.
- 0 reasonless `unverifiable` across 8,263+1,741 = 10,004 total rows examined this bundle-wide —
  Step 4.
- 0 unexpected duplicate `unit_id` — the merge script's own refuse-to-write guard, never tripped.
- 39 of 39 gate-scoped files at 0 violations — Step 6.

## Status: blocked-escalated

## Movement, four buckets

- **Closure:** 0 units' `docs/work-inventory.json` `status` field changed — this cycle merges
  oracle-result files, not the inventory.
- **Reclassification:** 0.
- **Reachability:** 8 units (the last75 lane's own new rows) move from "no oracle row" to a real
  disposition — re-confirmed here, not re-earned.
- **Instrument-correction:** 22 units' recorded `ours` value corrected from a base-only reading to
  the real EQMOD-summed total (the disagreement-fixes lane's own fix, re-confirmed by the merge's
  `superseded=22` figure matching its manifest exactly).

## Notes

Both wave-4 lanes reported `blocked-escalated` honestly at their own real row counts (22 of 26,
8 of 75) rather than rounding up, and this cycle's independent re-derivation confirms both figures
exactly — no report was taken on trust. `AT-33-E6-001` will still find rows 17/18 short on its
next attempt; that is correct, not a regression, since 67 units and 4 disagreements are real
remaining work, not a scanning artifact.

## Next-cycle plan

1. Build the `AC.Armor`-isolating (or fixed-baseline) oracle probe in `scripts/oracle_harness/`,
   then re-run the full 8,263-row population per `AT-33-E5-003`'s own re-run clause — resolves the
   4 remaining disagreements to 0.
2. Row the 67 still-unexamined units, per the `AT-33-E5-last75` receipt's own shape table and
   next-cycle plan (two harness defects blocking 17, two fixture-engineering gaps covering 27, five
   genuinely new engine shapes covering the remaining 23).
3. Once both land, re-run `AT-33-E6-001` as attempt 5.
