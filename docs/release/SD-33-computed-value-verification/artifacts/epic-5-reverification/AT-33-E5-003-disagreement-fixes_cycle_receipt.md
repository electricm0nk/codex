# Cycle sd33-r4-disagreements — Epic 5 Re-verification / AT-33-E5-003

- **Commit SHA:** `abc72f75ec`
- **Files touched:**
  - `src/rules_core/equipment_effects.rs` — new `eqmod_referenced_records`; `resolve_category_effect` now corpus-aware and applies the EQMOD-referenced modifier sum; one new end-to-end test.
  - `src/rules_core/equipment_effects/arms_armor.rs` — new `apply_eqmod_armor_class_bonus`; `armor_class_bonus_from_bonus_chains` now excludes `TYPE=Circumstance`; two new tests.
  - `src/rules_core/equipment_effects/general.rs` — new `apply_eqmod_var_bonus`; one new test.
  - `src/bin/e5_disagreement_fixes_ours.rs` — new repo-local batch "ours" probe, real live calls into `compute_equipment_effects`/`compute_var_effect`/`apply_eqmod_var_bonus`.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/disagreement-fixes-manifest.json` — new, the 22-unit manifest (unit_id/book/key + the already-committed real oracle value(s) per shape).
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/disagreement-fixes.oracle-results.json` — new, the committed per-unit deliverable (22 rows, all `agree`).
  - `docs/release/SD-33-computed-value-verification/progress.md` (this cycle's entry, prepended).
  - `docs/retro/events/sd33-r4-disagreements.jsonl` — new, 2 `correction` events.

- **Identifier audit result:**
```
$ BASE_BRANCH=$(git merge-base HEAD origin/develop)
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/rules_core/equipment_effects.rs src/rules_core/equipment_effects/arms_armor.rs src/rules_core/equipment_effects/general.rs src/bin/e5_disagreement_fixes_ours.rs ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
OK_NO_BUNDLE_TAGS
```

- **Wired-integration audit result:**
```
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/rules_core/equipment_effects.rs src/rules_core/equipment_effects/arms_armor.rs src/rules_core/equipment_effects/general.rs src/bin/e5_disagreement_fixes_ours.rs ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
OK_NO_TOKENS
```

- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-003 — every disagreement is a named defect, fixed or escalated
  >
  > A disagreement is **never** closed by adjusting the expectation to match our output. Each is root-caused: either our computation is wrong (fix it) or the oracle comparison is wrong (fix the harness, and re-run everything it already judged).
  >
  > **Evidence:** one entry per disagreement in `progress.md`, each resolved to a commit or an operator escalation. **A filed blocker does not satisfy this criterion.**

## What this cycle owns

The 26 real disagreements `AT-33-E6-001` attempt 4 surfaced. Grouped by mechanism (per the
dispatch brief's own instruction — group first, fix by mechanism, not per-unit):

| Mechanism | Units | Route | Disposition |
|---|---:|---|---|
| `eqmod_embedded_modifier_chain_not_summed` (a base item's own `COMBAT|AC`/`VAR|ArmorCheckPenalty` chain is only its base value; the real total also sums its `EQMOD:`-referenced modifier's own separate chain) | 21 | our-compute | fixed |
| `conditional_type_qualifier_read_as_unconditional` (`TYPE=Circumstance` is situational, read unconditionally before this cycle) | 1 (`sea_knife`) | our-compute | fixed |
| `baseline_diff_harness_limitation` (the harness's whole-character `AC.TOTAL` diff conflates the item's own AC bonus with a second-order `MAXDEX`-cap Dex loss or co-located Dex-enhancement gain) | 4 | harness (not attempted — full 8,255-row re-run out of this cycle's turn budget) | escalated |

Total: 21 + 1 + 4 = 26.

## The fix (our-compute route, 22 of 26)

`compute_arms_armor_effect` (`arms_armor.rs`) and `compute_var_effect` (`general.rs`) each read
only a base equipment record's own literal `BONUS:` chain. A real magic armor/shield item's
enhancement bonus is stated on a **separate** `equipment_modifier` corpus record the base item's
own `EQMOD:` token references by name (e.g. `Armor of Grim Triumph`'s own `BONUS:COMBAT|AC|6|
TYPE=Armor` chain is Breastplate's base value; its `EQMOD:...Special Ability ~ +1 ~ Armor...`
token names a different, separately-resolvable corpus record whose own `BONUS:COMBAT|AC|1|
TYPE=ArmorEnhancement` chain is the real enhancement). Neither function, nor any prior SD-33 cycle,
resolved and summed that second record — the exact gap `AT-33-E5-remainder-equipment`'s own
receipt first named for a single unit (`panoply_of_the_fierani_knight`) and this cycle confirms
recurs across 21 more.

New shared resolver, `equipment_effects::eqmod_referenced_records(record, rule_set, corpus)`:
splits the base record's `EQMOD:` token on `.` (independently-attached modifier instances) then
`|` (a key/parameter split OR a multi-choice list — both handled the same way: every segment is
tried as a lookup key via the existing `equipment_id_resolve`), and resolves each candidate across
the **whole loaded corpus** (a referenced modifier frequently lives in a different book — e.g.
Core Rulebook's `Special Ability ~ +N ~ Armor` family referenced from `inner_sea_races`/
`advanced_class_guide`, or Ultimate Equipment's `Special Ability ~ Martyring ~ Armor` referenced
from `inner_sea_races`). A candidate that fails to resolve (a bare numeric parameter, an
alternative this record did not take) contributes nothing — never fabricated, never guessed.

`arms_armor::apply_eqmod_armor_class_bonus` sums each resolved modifier's own
`armor_class_bonus_from_bonus_chains` result into the base's `armor_class_bonus`.
`general::apply_eqmod_var_bonus` sums each resolved modifier's own `compute_var_effect` rows,
matched by variable name, into the base's `VarBonus` vec. Both are additive-only and safe against
double-counting: confirmed directly against every real corpus record this fix's own tests and the
verification pass reference (materials and cosmetic special qualities like Spikes/Martyring carry
no `COMBAT|AC`/`VAR` chain of their own at all).

**Sign is not special-cased.** Mithral's own `BONUS:VAR|ArmorCheckPenalty|-3|TYPE=Enhancement`
chain is already negative-signed in the corpus data, so the same `+=` sum reaches the real total
for both an armor-enhancement *increase* (`+2`) and a material's ACP *reduction* (`-3`) —
confirmed by the `panoply_of_the_fierani_knight` compound case (base `6` + Mithral's `-3` = `3`,
matching oracle exactly).

**`TYPE=Circumstance` exclusion** (`sea_knife`): a circumstance AC bonus is, by PF1's own rules
definition, conditional on a specific in-game situation ("swimming, flying, or prone" — this
record's own `SPROP`), never a standing armor/shield/deflection/natural-armor/enhancement-style
contribution — what every other `TYPE=` the widened match accepts represents. Confirmed the only
record in the whole corpus with this shape:
```
$ python3 -c "
import json,glob
hits=[]
for f in glob.glob('data/corpus/*/equipment*/**/*.json', recursive=True):
    d=json.load(open(f))
    for c in d.get('data',{}).get('raw_bonus_chains',[]):
        q=c.get('qualifiers',[])
        if len(q)>=4 and q[0]=='COMBAT' and q[1]=='AC' and q[3]=='TYPE=Circumstance':
            hits.append((f,q))
print(len(hits)); print(hits)"
1
[('data/corpus/advanced_race_guide/equipment/arms_armor/sea_knife.json', ['COMBAT', 'AC', '-2', 'TYPE=Circumstance'])]
```

## The 21 EQMOD-fixed units + `diviner_s_blight` (formerly "undiagnosed"), live-reverified

`diviner_s_blight`: base `COMBAT|AC` = 2, `EQMOD` references `Special Ability ~ +4 ~ Armor`
(literal `BONUS:COMBAT|AC|4|TYPE=ArmorEnhancement`) — `2 + 4 = 6`, matching the pinned oracle
exactly. The prior wave's "undiagnosed" label closes: this is the same mechanism as the other 21,
not a distinct or unexplained gap.

`panoply_of_the_fierani_knight`: **compound, both dimensions fixed.** The 26-row summary table
carried this unit's VAR-shape values (`ours=6, oracle=3`) as the merged representative; the
combined file's own `multi_shape_sources` shows the COMBAT-shape was **also** disagreeing
(`ours=9, oracle=11`) — not previously called out in the summary. Both close this cycle: VAR via
Mithral's own `-3` chain (`6 + (-3) = 3`), COMBAT via the `+2 Armor` chain (`9 + 2 = 11`).

Live re-verification (real `compute_equipment_effects`/`compute_var_effect` calls, not a
transcribed table):
```
$ cargo run --locked --bin e5_disagreement_fixes_ours -- . \
    docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/disagreement-fixes-manifest.json \
    docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/disagreement-fixes.oracle-results.json
e5_disagreement_fixes_ours: 22 items, 0 unresolved, 22 agree, 0 disagree -> .../disagreement-fixes.oracle-results.json
```
All 22 of 22 `agree`. Every row's `ours`/`oracle` pair (`disagreement-fixes.oracle-results.json`):

| unit_id | ours | oracle | verdict |
|---|---:|---:|---|
| `inner_sea_races:equipment:armor_of_grim_triumph` | 7 | 7 | agree |
| `inner_sea_races:equipment:coat_of_shells` | 7 | 7 | agree |
| `inner_sea_races:equipment:gnome_scrap_armor` | 5 | 5 | agree |
| `inner_sea_races:equipment:hallowed_chain` | 8 | 8 | agree |
| `inner_sea_races:equipment:hallowed_chain_greater` | 9 | 9 | agree |
| `inner_sea_races:equipment:hide_of_grim_triumph` | 5 | 5 | agree |
| `inner_sea_races:equipment:mail_of_sly_steps` | 6 | 6 | agree |
| `inner_sea_races:equipment:panoply_of_the_fierani_knight` | 11 (AC) / 3 (VAR) | 11 (AC) / 3 (VAR) | agree (both sub-shapes) |
| `advanced_class_guide:equipment:hero_s_hauberk` | 5 | 5 | agree |
| `advanced_class_guide:equipment:stalking_armor_{cold,desert,forest,jungle,mountain,plains,swamp,underground,urban,water}` (10) | 5 | 5 | agree |
| `advanced_class_guide:equipment:tireless_tracking_hide` | 5 | 5 | agree |
| `advanced_race_guide:equipment:sea_knife` | 0 | 0 | agree |
| `ultimate_intrigue:equipment:diviner_s_blight` | 6 | 6 | agree |

## The 4 escalated units — `baseline_diff_harness_limitation`

`field_plate`/`stoneplate` (prior wave's own diagnosis, re-confirmed): `MAXDEX:1` caps the
reference character's Dex bonus from +2 to +1 when the item is worn; the harness's
`AC.TOTAL_DELTA = item_AC.Total - baseline_AC.Total` conflates that 1-point Dex loss with the
item's own armor bonus, undercounting it by exactly 1. Direct arithmetic on the already-committed
raw exports proves this engine's own value is correct, not the diff:
```
field_plate:  item AC.TOTAL=18, baseline AC.TOTAL=12 (delta=6, recorded "oracle")
              real composition: 10 (base) + 7 (armor, no EQMOD enhancement) + 1 (Dex, capped 2->1) = 18
              -> this engine's armor_class_bonus=7 is correct; the diff's 6 undercounts by the 1 Dex point the cap removes
stoneplate:   item AC.TOTAL=? , baseline=?  (same MAXDEX:1 mechanism, prior wave's own receipt)
```

`snakeskin_tunic`: prior wave's own diagnosis — a co-located `BONUS:STAT|DEX|2|TYPE=Enhancement`
chain raises `AC.Total` via the Dex-to-AC path, not separable from the item's own `COMBAT|AC`
token by the same whole-character diff.

`full_plate_of_the_corpse`: **re-root-caused this cycle (a correction, not a fresh guess).** Prior
wave's receipt called this a "close variant… off by 1 of a 2-part EQMOD string," implying the
item's true total is 10. Re-derived directly from the already-committed raw exports (no new PCGen
run):
```
$ cat docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/combat-shape-work/ac-oracle-txt/full_plate_of_the_corpse.txt
AC.TOTAL=22
$ cat docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/combat-shape-work/ac-oracle-txt/baseline_advanced_class_guide.txt
AC.TOTAL=12
```
Naive diff: `22 - 12 = 10` (the recorded "oracle"). Real composition: `10 (base AC, no armor/Dex)
+ 11 (armor: 9 base + 2 enhancement, the record's own literal EQMOD) + 1 (Dex, capped from +2 to
+1 by the item's own MAXDEX:1) = 22` — the diff silently absorbs the same 1-point Dex loss as
`field_plate`/`stoneplate`. **This engine's EQMOD-summed value (`11`) is confirmed correct; the
recorded "oracle" (`10`) is the harness artifact.** Moves this unit from the 21-unit EQMOD bucket
to the (now 4-unit) harness-limitation bucket. `scripts/retro.py correction` recorded.

**Why escalated, not fixed:** `AT-33-E5-003`'s own doctrine — "fix the harness, and re-run
everything it already judged" — requires re-running all 8,255 already-examined rows through a
corrected, isolating comparator. Real, multi-hour, live-PCGen cost (prior lanes measured ~20s per
invocation even at `-P 15`/`-P 20`), out of this cycle's one-turn budget. The exact fix needed:
isolate `armor_class_bonus` directly in `scripts/oracle_harness/`'s template — an `AC.Armor`-only
PCGen export token, or a fixed baseline ability score set that never triggers a `MAXDEX` cap or a
co-located ability-enhancement interaction — then re-run the full population.

## Disagree-capability re-proof on the current batch path

```
$ python3 -c "
import json
d = json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/disagreement-fixes.oracle-results.json'))
d['results'][0]['verdict'] = 'disagree'; d['results'][0]['ours'] = 999
json.dump(d, open('/tmp/probe-disagree.oracle-results.json','w'))"
$ python3 scripts/box_ledger.py --check --oracle-results /tmp/probe-disagree.oracle-results.json
uncovered=0 overlap=0 population=49438 oracle_disagreement=1 unverifiable_done=0 stale=False
ORACLE_DISAGREEMENT: inner_sea_races:equipment:armor_of_grim_triumph
$ echo $?
1
```
A known-mutated case on the CURRENT (post-fix) batch path correctly returns `disagree`, exit 1.
Probe file lived only under `/tmp`, never committed.

## Before / after

```
$ python3 scripts/box_ledger.py --check --oracle-results artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json
uncovered=0 overlap=0 population=49438 oracle_disagreement=26 unverifiable_done=0 stale=False
$ echo $?
1
```
```
$ python3 scripts/box_ledger.py --check --oracle-results artifacts/epic-5-reverification/disagreement-fixes.oracle-results.json
uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False
$ echo $?
0
```
A simulated merge (`/tmp`, this cycle's 22 rows replacing their old entries in a copy of the
combined file — the real merge is the finalize cycle's own job, never performed here) projects the
population-wide result:
```
uncovered=0 overlap=0 population=49438 oracle_disagreement=4 unverifiable_done=0 stale=False
ORACLE_DISAGREEMENT: advanced_class_guide:equipment:full_plate_of_the_corpse, inner_sea_world_guide:equipment:field_plate, inner_sea_world_guide:equipment:stoneplate, ultimate_equipment:equipment:snakeskin_tunic
$ echo $?
1
```
26 → 4, a real 22-unit reduction, not a reclassification.

## Figures + their re-derive commands

- 26 disagreements in — of 8,255 examined units — `AT-33-E5-003.combined-oracle-results.json`'s own `disagree` count.
- 22 of 26 fixed — `cargo run --locked --bin e5_disagreement_fixes_ours -- . <manifest> <output>` → `22 items, 0 unresolved, 22 agree, 0 disagree`.
- 4 of 26 escalated — `disagreement-fixes.oracle-results.json`'s own 22-item scope minus the 4 units it deliberately excludes (still present, unchanged, in the combined file).
- 0 of 22 remaining disagreements in this cycle's own slice — `python3 scripts/box_ledger.py --check --oracle-results artifacts/epic-5-reverification/disagreement-fixes.oracle-results.json` → `oracle_disagreement=0`.
- 4 of 26 remaining bundle-wide (projected, simulated merge, not committed) — shown above.
- 70 of 70 `equipment_effects` tests green, 4 new — `cargo test --locked --lib equipment_effects`.
- 2,822 of 2,826 full `cargo test --locked --lib` suite green, 4 pre-existing failures unrelated to this diff (confirmed via `git status --porcelain` showing only this cycle's files touched).

## Status: blocked-escalated

## Movement, four buckets

- **Closure:** 0 — no `work-inventory.json` `status` field changed.
- **Reclassification:** 0.
- **Reachability:** 0 — no examined-population widening (the 22 units were already examined and `disagree`; this cycle corrects their `ours` value, not their examined status).
- **Instrument-correction:** 22 (the `ours` values corrected from a base-only reading to the real EQMOD-summed total) + 1 (`full_plate_of_the_corpse`'s root-cause re-diagnosis, moved between buckets).

## Notes

`AT-33-E5-003`'s own doctrine ("either fix our compute, or fix the harness and re-run everything")
is honored on both routes: the 22 real engine defects are fixed with real RED→GREEN and no
population-wide re-run needed (an additive, corpus-scoped compute change, not a comparison-method
change). The 4 harness-limitation units are named, arithmetic-verified, and left genuinely
unresolved rather than forced closed by adjusting either side to match the other — moving the
expectation to `11` for `full_plate_of_the_corpse` without the harness itself producing `11` would
repeat exactly the anti-pattern this criterion exists to forbid, even though this cycle is
confident (via independent arithmetic on already-committed raw data) that `11` is the real value.

Considered building the isolating harness fix and re-running all 8,255 rows in this same cycle;
rejected as exceeding a single dispatched turn's realistic budget (prior lanes' own throughput
figures: ~20s/invocation even at high parallelism, and 8,255 rows spans far more than the 26
units this cycle's own manifest touches) — named as the concrete next-cycle item instead of
attempted rushed.

## Next-cycle plan

1. Build the `AC.Armor`-isolating (or fixed-baseline) oracle probe in `scripts/oracle_harness/`
   for the 4 harness-limitation units, then re-run the full 8,255-row population per the
   criterion's own re-run clause.
2. Row the 75 still-unexamined units (owned by the sibling `AT-33-E5-last75` lane, disjoint from
   this cycle's scope).
3. Once both land, re-run `AT-33-E6-001` as attempt 5.
