# Cycle AT-33-E6-001 (attempt 6) — epic-6-closure / AT-33-E6-001

- **Rust suite:** `cargo test --locked --lib` → `test result: FAILED. 2824 passed; 4 failed; 14 ignored; 0 measured` — **2,824 of 2,828 executed lib tests green, 4 of 2,828 RED**. See Shortfall 4. (Attempt 5 ran only the narrow `--lib equipment_effects` filter, 70 of 70; the wider run this wave changed `src/rules_core/corpus_loader.rs`, `damage_total.rs`, `equipment_effects.rs`, `equipment_effects/equipmods.rs` was the correct scope and it is red.)
- **Commit SHA:** recorded below at push time (`sd33-r5-acceptance-scan`, remediation wave 5)
- **Files touched:** `docs/release/SD-33-computed-value-verification/artifacts/epic-6-closure/AT-33-E6-001-attempt6_cycle_receipt.md`, `docs/release/SD-33-computed-value-verification/kanban.md` (row 19 note only), `docs/retro/events/*.jsonl`
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (scan-only cycle; no `src/`, `scripts/` or corpus writes)
- **Wired-integration audit result:** OK_NO_TOKENS (scan-only cycle)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "Every criterion `AT-33-E1-001` … `AT-33-E5-003` is `complete`, and every `kanban.md` card is `complete`. **A card at `returned-to-backlog`, `in-progress`, or `complete`-with-a-deferred-half blocks closure.** **If anything is short, the cycle stops here** — no retrospective, no sweep, **no PR**. Report what is short with the command that shows it. That is a correct outcome, not a failure."

## Gate result: **FAIL** (attempt 6). Sixth consecutive correct halt. Four shortfalls.

No retrospective, no sweep, no PR. Kanban row 19 stays `blocked-escalated`.

Both of attempt 5's shortfalls **narrowed but did not close**. Two further
shortfalls are named for the first time this attempt — one of them
(Shortfall 4) was masked at attempt 5 by a test filter narrower than the
surface that wave had actually changed.

### Figures

| Figure | Value | Denominator | Re-derive command |
|---|---|---|---|
| Blessed units carrying an oracle row | 8,291 | of 8,330 blessed units | CHECK 1 command below |
| Blessed units carrying **no** oracle row | 39 | of 8,330 blessed units | CHECK 1 command below |
| `fixture-verified` rows | 1,741 | of 1,741 `fixture-verified` units | CHECK 1 command below |
| `literal-verified` rows | 6,550 | of 6,589 `literal-verified` units | CHECK 1 command below |
| Examined units at `disagree` | 1 | of 8,291 examined units | CHECK 2 command below |
| Reasonless `unverifiable` | 0 | of 7,504 `unverifiable` rows | CHECK 1 command below |
| Duplicate `unit_id`s | 0 | of 8,291 rows | CHECK 1 command below |
| Denominator-gate files clean | 46 | of 46 scanned package markdown files | `bash scripts/verify.sh --only denominator-gate` |
| work-inventory units at `unknown` | 0 | of 49,438 units | `jq '[.units[]\|select(.status=="unknown")]\|length' docs/work-inventory.json` |
| Lib tests green | 2,824 | of 2,828 executed lib tests | `cargo test --locked --lib` |
| 66-unit harness re-run rows reflected in the closure artifact | 64 | of 66 re-run rows | Shortfall 3 command below |

### Four buckets

| Bucket | Count | Denominator |
|---|---|---|
| `agree` | 786 | of 8,291 rowed units |
| `unverifiable` (each reasoned) | 7,504 | of 8,291 rowed units |
| `disagree` | 1 | of 8,291 rowed units |
| unrowed (not examined at all) | 39 | of 8,330 blessed units |

---

## Shortfall 1 — 39 of 8,330 blessed units carry no oracle row

Down from 67 of 8,330 at attempt 5, 75 at attempt 4, 391 at attempt 3. This is a
membership check, not a count check.

```
python3 -c "import json,collections
wi=json.load(open('docs/work-inventory.json'))['units']
pop={u['id'] for u in wi if u.get('status') in ('literal-verified','fixture-verified')}
for p,n in [('fixture-verified.combined-oracle-results.json',1741),
            ('literal-verified.oracle-results.json',6589),
            ('AT-33-E5-003.combined-oracle-results.json',8330)]:
  d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/'+p))
  k='results' if 'results' in d else [x for x in d if isinstance(d[x],list)][0]
  r=d[k]; ids=[x.get('unit_id') for x in r]
  print(p,'rows',len(r),'distinct',len(set(ids)),'pop',n,dict(collections.Counter(x.get('verdict') for x in r)))
c=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json'))['results']
miss=sorted(pop-{x['unit_id'] for x in c}); print('MISSING',len(miss))"
```

Output:

```
fixture-verified.combined-oracle-results.json rows 1741 distinct 1741 pop 1741 {'agree': 396, 'unverifiable': 1345}
literal-verified.oracle-results.json rows 6550 distinct 6550 pop 6589 {'agree': 390, 'unverifiable': 6159, 'disagree': 1}
AT-33-E5-003.combined-oracle-results.json rows 8291 distinct 8291 pop 8330 {'agree': 786, 'unverifiable': 7504, 'disagree': 1}
MISSING 39
```

**`fixture-verified` is CLOSED** — 1,741 of 1,741, 0 duplicates, 0 `disagree`
(row 16, re-verified not re-investigated). The 39 are entirely in the
`literal-verified` population: 6,550 of 6,589.

The 39 missing `unit_id`s, in full — every one named by the three wave-5 lanes'
own shape tables (23 weapon-shape + 9 skill-combat-shape + 7 eqm-shape = 39):

```
advanced_class_guide:equipment:brawler_s_flurry
advanced_class_guide:equipment:duelist_s_comate
advanced_class_guide:equipment:rapier_of_battlefield_movement
advanced_class_guide:equipment:swashbuckler_s_rapier
advanced_race_guide:equipment:heartstake_bolts_5
advanced_race_guide:equipment_modifier:material_darkleaf_cloth_clothing
core_rulebook:equipment:amulet_of_mighty_fists_1
core_rulebook:equipment:amulet_of_mighty_fists_2
core_rulebook:equipment:amulet_of_mighty_fists_3
core_rulebook:equipment:amulet_of_mighty_fists_4
core_rulebook:equipment:amulet_of_mighty_fists_5
core_rulebook:equipment:rod_alertness
core_rulebook:equipment:stone_of_good_luck_luckstone
core_rulebook:equipment_modifier:draco
core_rulebook:equipment_modifier:dragonhide
core_rulebook:equipment_modifier:material_dragonhide
core_rulebook:equipment_modifier:special_quality_spikes_shieldbash
core_rulebook:equipment_modifier:special_quality_wield_size_1_step_greater_no_penalty
core_rulebook:equipment_modifier:special_quality_wield_size_2_steps_greater_no_penalty
core_rulebook:equipment_modifier:special_quality_wield_size_3_steps_greater_no_penalty
core_rulebook:equipment_modifier:spike_sb
ultimate_combat:equipment:arrow_iron_tipped_distance_20
ultimate_equipment:equipment:belt_of_teeth
ultimate_equipment:equipment:cursed_sword_2
ultimate_equipment:equipment:gunfighter_s_poncho
ultimate_equipment:equipment:horseshoes_of_crushing_blows_1
ultimate_equipment:equipment:horseshoes_of_crushing_blows_2
ultimate_equipment:equipment:horseshoes_of_crushing_blows_3
ultimate_equipment:equipment:horseshoes_of_crushing_blows_4
ultimate_equipment:equipment:horseshoes_of_crushing_blows_5
ultimate_equipment:equipment:robe_of_vermin
ultimate_equipment:equipment:scattershot_bracers
ultimate_equipment:equipment:staff_of_the_hierophant
ultimate_equipment:equipment:talons_of_leng
ultimate_psionics:equipment:companion_stone_far_sight
ultimate_psionics:equipment:flurry_of_fists
ultimate_psionics:equipment:flurry_of_strikes
ultimate_psionics:equipment_modifier:special_quality_dissonance_enhancement_bonus_alt
ultimate_psionics:equipment_modifier:special_quality_dissonance_enhancement_bonus_main
```

All three wave-5 remainder lanes returned `blocked-escalated` and each left a
complete per-shape table with the concrete blocker — that is correct conduct and
is what makes this list actionable. It is not closure. Kanban row 17
(`AT-33-E5-002`) is `in-progress`, which blocks by the criterion's own words.

## Shortfall 2 — 1 of 8,291 examined units still `disagree`

Down from 4 of 8,263 at attempt 5, 26 of 8,255 at attempt 4. The four attempt-5
disagreements were genuinely resolved (see the audit below); a **new** one was
surfaced by wave 5's weapon lane examining previously-unrowed units.

```
python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json; echo $?
```

Output:

```
uncovered=0 overlap=0 population=49438 oracle_disagreement=1 unverifiable_done=0 stale=False
ORACLE_DISAGREEMENT: advanced_race_guide:equipment:rending_claw_blades
EXIT=1
```

The dispatch brief for this attempt stated the check "must print
`oracle_disagreement=0` and exit 0". It prints `oracle_disagreement=1` and exits
`1`. `advanced_race_guide:equipment:rending_claw_blades` (ours 0, oracle 1,
dimension `DAMAGE`) is root-caused by the weapon lane to a corpus-extraction
`.MOD`-attached-EQMOD gap and **escalated under `progress.md`'s
`## Open blockers`, not fixed**. Per `AGENTS.md`'s Blocker Discipline and this
bundle's own `closure-criteria-must-not-permit-open-cards` precedent, a filed
`## Open blockers` entry is not a disposition and not a closure path. Kanban row
18 (`AT-33-E5-003`) is `blocked-escalated`, which blocks.

### Audit — the four attempt-5 disagreements were RESOLVED, not hidden

Each of the four is still **present** as a row, now `agree`, and traced to
`a68fbeea3d` whose diff was read in full.

| unit_id | attempt-5 (ours vs oracle) | now (ours vs oracle) | oracle moved? | commit | verdict |
|---|---|---|---|---|---|
| `advanced_class_guide:equipment:full_plate_of_the_corpse` | 9 vs 10 | 11 vs 11 | yes, 10 → 11 | `a68fbeea3d` | real-fix |
| `inner_sea_world_guide:equipment:field_plate` | 7 vs 6 | 7 vs 7 | yes, 6 → 7 | `a68fbeea3d` | real-fix |
| `inner_sea_world_guide:equipment:stoneplate` | 9 vs 8 | 9 vs 9 | yes, 8 → 9 | `a68fbeea3d` | real-fix |
| `ultimate_equipment:equipment:snakeskin_tunic` | 1 vs 2 | 1 vs 1 | yes, 2 → 1 | `a68fbeea3d` | real-fix |

**The oracle value moved on all four.** That is the shape of blocking shortfall
(a) — "an expected/oracle value edited to match our output" — so it was audited
directly rather than accepted from the lane's report, and it is **not** shortfall
(a). The distinction, established from the commit diff and not from the lane's
prose:

- The prior oracle values (10, 6, 8, 2) were **derived**, not observed: they came
  from `combat-shape-work/ac_build_results.py`'s
  `oracle_value = int(item_AC.Total) - int(baseline_AC.Total)` — a
  whole-character AC diff. That derivation cannot separate the item's own
  `COMBAT|AC` chain from a `MAXDEX`-cap Dex loss (`field_plate`, `stoneplate`,
  `full_plate_of_the_corpse`) or a co-located `STAT|DEX|2|TYPE=Enhancement`
  chain on the same record (`snakeskin_tunic`).
- The method was **replaced**, not the numbers: a new BatchExporter template
  reads PCGen's own per-type bonus token
  (`AC.ISOLATED = BONUS.COMBAT.AC.TOTAL.!BASE.!Ability.!Size`) directly off the
  **same already-committed, unmodified `.pcg` fixtures**. No baseline character
  is involved, so a Dex/`MAXDEX` term cannot leak in structurally.
- The new oracle values are cross-checked two ways in the diff: `AC.ISOLATED`
  equals the independent per-type breakdown sum (`AC.ARMOR` + `AC.ARMORENH`) in
  all 4, and equals `AC.TOTAL − AC.BASE − AC.ABILITY − AC.SIZE` in all 4.
- Our side moved on exactly one of the four (`full_plate_of_the_corpse`,
  9 → 11) and that movement was a **stale row**, not a new code path:
  `abc72f75ec`'s EQMOD resolver already summed the referenced `+2`
  unconditionally; the unit had simply never been re-run.

This is `AT-33-E5-003`'s own "fix the harness, and re-run everything it already
judged" route, taken correctly on the first limb. The second limb is Shortfall 3.

No unit was dropped (all 4 present), and none was reclassified to
`unverifiable`. Reasonless `unverifiable` is 0 of 7,504.

## Shortfall 3 — the method change's re-run did not fully reach the closure artifact

`AT-33-E5-003`'s harness route obliges a re-run of everything the changed method
already judged. The re-run **did happen** — `full-rerun-wave5.oracle-results.json`
carries 66 rows, 66 of 66 `agree`. But **2 of those 66 re-run rows were not
merged** into `AT-33-E5-003.combined-oracle-results.json`, which still carries
the pre-re-run values for them.

```
python3 -c "
import json,os
os.chdir('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification')
r=json.load(open('full-rerun-wave5.oracle-results.json'))['results']
cm={x['unit_id']:x for x in json.load(open('AT-33-E5-003.combined-oracle-results.json'))['results']}
bad=[(x['unit_id'],cm[x['unit_id']].get('ours'),cm[x['unit_id']].get('oracle'),x.get('ours'),x.get('oracle'))
     for x in r if x['unit_id'] in cm and (cm[x['unit_id']].get('ours')!=x.get('ours') or cm[x['unit_id']].get('oracle')!=x.get('oracle'))]
print('rerun rows',len(r),'not reflected in combined:',len(bad))
[print(' ',b) for b in bad]"
```

Output:

```
rerun rows 66 not reflected in combined: 2
  ('inner_sea_gods:equipment:ring_of_unquenchable_passions', 5, 5, 1, 1)
  ('inner_sea_races:equipment:goblin_plate', 6, 6, 10, 10)
```

Both remain `agree` on both sides, so **no verdict is wrong** — this does not
change any bucket count. It is still a real shortfall: the closure artifact,
which is the file `box_ledger.py` and every downstream figure read, carries stale
`ours`/`oracle` magnitudes for 2 of the 66 units the corrected method re-judged.
`goblin_plate` is specifically the unit `a68fbeea3d`'s own commit message calls
out as having moved (a coincidental double-error the full re-run caught) — its
corrected value 10 is in the re-run file and 6 is in the closure file. The merge
step, not the re-run, is what is incomplete. `method_change_rerun_verified` is
therefore recorded **false**: the re-run was performed and verified, its
propagation was not.

## Shortfall 4 — the Rust lib suite is RED: 4 of 2,828 executed lib tests fail

Attempt 5 ran `cargo test --locked --lib equipment_effects` (70 of 70 green).
This wave changed four `src/rules_core/` files
(`corpus_loader.rs`, `damage_total.rs`, `equipment_effects.rs`,
`equipment_effects/equipmods.rs`), so the wider run is the correct scope, and it
is red.

```
cargo test --locked --lib
```

Output (tail):

```
failures:
    rules_core::equipment_resolver::tests::catalog_rows_span_every_ingested_book_with_their_real_counts
    rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::a_subset_run_trips_the_population_mismatch_check
    rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::corpus_wide_scan_population_matches_the_closed_gate1_census
    rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::f1_population_matches_the_current_true_formula_bearing_count_not_the_stale_sd32_census

test result: FAILED. 2824 passed; 4 failed; 14 ignored; 0 measured; 0 filtered out
```

Attributed per `AGENTS.md`'s "attribute every `test result: FAILED` line" rule —
not bucketed as environmental:

**3 of 4 are caused by SD-33's own Epic 4.** All three
`formula_interpreter_corpus_wide` failures share one panic:

```
ValueError: doneness: unmapped 'ambiguous' + 'unmeasurable'
  scripts/observer/pf1e_dashboard_producer.py:4031 _doneness_verdict_uncapped
  <- scripts/coverage_ledger.py:202 unit_verdict
  <- scripts/shape_ledger.py:981 main
```

The `(wiring_class='ambiguous', status='unmeasurable')` combination exists on
**11 of 49,438** work-inventory units:

```
python3 -c "import json,collections
u=json.load(open('docs/work-inventory.json'))['units']
print(collections.Counter((x.get('wiring_class'),x.get('status')) for x in u if x.get('status')=='unmeasurable' or x.get('wiring_class')=='ambiguous'))"
```
```
Counter({('ambiguous', 'not-ingested'): 336, ('display', 'unmeasurable'): 310, ('ambiguous', 'text-complete'): 112, ('ambiguous', 'ingested-magnitude'): 43, ('ambiguous', 'grounded'): 42, ('ambiguous', 'unmeasurable'): 11, ('ambiguous', 'not-started'): 1})
```

`docs/work-inventory.json` has exactly one commit on this branch —
`00ca087775 fix(sd33): AT-33-E4-002 -- 4,224 unknown units reclassified to zero`
(`git log --oneline -1 -- docs/work-inventory.json`), which is SD-33's own Epic 4
deliverable. Kanban row 14 (`unknown-to-zero`, `AT-33-E4-002`) is marked
`complete` over a suite this bundle's own commit turned red. This is a
`complete`-with-a-consequence-not-carried, and it is exactly the
`count-change-needs-a-sweep-not-just-a-build` shape: the reclassification
compiled clean and left another module's mapper unmapped.

**1 of 4 is inherited, not SD-33-caused.**
`catalog_rows_span_every_ingested_book_with_their_real_counts` asserts
`left: 8119, right: 8100` at `src/rules_core/equipment_resolver.rs:863` — a
hardcoded catalog count 19 short of the live corpus. `data/corpus/**` is
byte-identical to the `tranche/13` cut (`git diff --stat f652db7ac7..HEAD -- data/corpus`
→ empty) and `git status --porcelain data/` is empty (no untracked litter), so
this test was already red at the cut and is inherited from `develop`. It is
still red and still blocks a green-suite claim; naming it here rather than
carrying it as "the known environmental failure".

---

## Re-verified CLOSED (re-verified, not re-investigated)

| Item | Result | Command |
|---|---|---|
| Row 16 `fixture-verified` | 1,741 of 1,741 rows, 1,741 distinct, 396 agree / 1,345 unverifiable / **0 disagree** | CHECK 1 command |
| Duplicate `unit_id`s | 0 of 8,291 rows (8,291 rows, 8,291 distinct) | CHECK 1 command |
| Reasonless `unverifiable` | **0 of 7,504** | `python3 -c "import json; c=json.load(open('.../AT-33-E5-003.combined-oracle-results.json'))['results']; print(sum(1 for x in c if x.get('verdict')=='unverifiable' and not (x.get('reason') or x.get('note'))))"` → `0` |
| work-inventory `unknown` | **0 of 49,438 units** | `jq '[.units[]\|select(.status=="unknown")]\|length' docs/work-inventory.json` → `0` |
| Denominator gate | PASS, **46 of 46** files checked, 0 violations | `bash scripts/verify.sh --only denominator-gate` → `PASS denominator-gate (files_checked=46 violations=0)`, `RESULT: PASS` |
| Epic 3 artifact at the SD-33 path | present | `ls artifacts/epic-3-engine-coverage/` → `formula_interpreter.corpus-wide.json` present |
| SD-32's `gate-2-engines` file untouched | untouched | `git log --oneline -3 -- "*gate-2-engines*"` → newest is `d5cbf1f801 docs(sd32)…`; no SD-33 commit touches it |
| No hardcoded exclusion lists in closure instruments | none | `grep -nE "EXCLUDED\|SKIP_\|exclude_list\|EXEMPT\|BLOCKLIST\|blacklist" scripts/box_ledger.py scripts/verify.sh src/bin/v06_work_inventory.rs` → 0 hits in `box_ledger.py`; `verify.sh` hits are the unrelated `pi-sweep` PI term list; `v06_work_inventory.rs` hits are documented per-record rules, not a book/unit carve-out |

### Denominator-gate scope widened, not blinded — re-proven live this cycle

Detection was re-proven by planting a probe receipt inside the gate's scope,
observing the violation, and removing the probe.

```
$ printf '# probe\n\nCoverage is 100%% and 47 units are complete.\n' > artifacts/epic-6-closure/AT-33-E6-001-attempt6-PROBE_cycle_receipt.md
$ python3 scripts/denominator_gate.py --check
VIOLATION .../AT-33-E6-001-attempt6-PROBE_cycle_receipt.md:3: Coverage is 100% and 47 units are complete.
files_checked=47
violations=1
$ rm .../AT-33-E6-001-attempt6-PROBE_cycle_receipt.md
$ python3 scripts/denominator_gate.py --check
files_checked=46
violations=0
```

The scanned set moved 40 → 46 files between attempt 5 and attempt 6 (new wave-5
receipts entering scope), the bare-hundred-percent matcher still fires, and the
scan's own draft receipt is inside the scanned set.

### `disagree` capability re-proven on the current batch path

Injected a known-disagreeing case into a **copy** of the current closure artifact
and fed it through the live `box_ledger.py --check` route, then removed the probe.

```
$ cp .../AT-33-E5-003.combined-oracle-results.json /tmp/probe-combined.json
$ python3 -c "...set field_plate ours=99, verdict=disagree in /tmp/probe-combined.json..."
probe injected: field_plate ours=99 verdict=disagree
$ python3 scripts/box_ledger.py --check --oracle-results /tmp/probe-combined.json; echo $?
uncovered=0 overlap=0 population=49438 oracle_disagreement=2 unverifiable_done=0 stale=False
ORACLE_DISAGREEMENT: advanced_race_guide:equipment:rending_claw_blades, inner_sea_world_guide:equipment:field_plate
1
$ rm /tmp/probe-combined.json
```

The count moved 1 → 2 and the injected unit was named. The real artifact was
never modified.

## Check 3 — audits of wave 5's new judgment calls

Each of these is a place a false `agree` could be manufactured. Audited against
the rows and the receipts, not the lane reports.

| Shape | Finding |
|---|---|
| `WEAPON\|DAMAGEMULT` fractional | **No silent truncation.** `advanced_players_guide:equipment:sword_cane` (`BONUS:WEAPON\|DAMAGEMULT:2\|-0.5`) is `ours=None, oracle=None, unverifiable`, reason `no_comparable_export_token`. The weapon receipt states the decision explicitly (lines 220-221, 263-265): "recorded `unverifiable` rather than truncated to an integer (truncating -0.5 to 0 or -1 would…)". **1 of 4 `DAMAGEMULT` units examined**; the other 3 are in Shortfall 1's unrowed 39. |
| bare `WEAPON\|TOHIT,DAMAGE,ATTACKS` | **Comparable magnitude stated and applied consistently on every row that exists.** The receipt (line 260) fixes `TOHIT` as the single comparable magnitude for `TYPE=`-less bare chains. Both examined rows (`crossbow_double` oracle `-4`, `rod_withering` oracle `1`) are `unverifiable`/`no_resolver` with `ours=None` — the engine deliberately excludes these chains, so no comparison was manufactured. **2 of 6 examined**; "consistently across all 6" is NOT yet provable — 4 of 6 are in the unrowed 39. |
| `EQM*` modifiers | **Nothing to audit — nothing was rowed.** The `eqm-modifier-family` lane returned `blocked-escalated` with **0 of 7 rowed**; its own `red_green` field states the live-oracle host-application mechanism "never produced a trustworthy oracle value", confirmed on 2 independent shapes/hosts/export-tokens. No host item is named per modifier because no comparison was made. All 7 are in the unrowed 39. This is honest and correct conduct, and it is an open population. |
| psionics | **Wave 4's finding holds — no unit called `unverifiable` for "book not in oracle".** All 14 `ultimate_psionics` rows the skill-combat lane wrote are `agree` with real matched magnitudes on both sides (4/4, 10/10, 9/9, 1/1, 8/8, 20/20, …). `grep -niE "book not in oracle\|not in the oracle"` over that lane's receipt and results file → 0 hits. The lane additionally landed a real harness fix (campaign-KEY-vs-display-name) and a real engine fix (`equipment_id_resolve` OUTPUTNAME-divergent identity, RED→GREEN). |

## Criterion / card status at attempt 6

| Card | Criterion | Status | Blocks? |
|---|---|---|---|
| 1-4 | `AT-33-E1-001..004` | complete | no |
| 5-8 | `AT-33-E2-001..004` | complete | no |
| 9-12 | `AT-33-E3-001..004` | complete | no |
| 13, 15 | `AT-33-E4-001`, `AT-33-E4-003` | complete | no |
| 14 | `AT-33-E4-002` | complete | **yes** — complete over a suite its own commit `00ca087775` turned red (Shortfall 4) |
| 16 | `AT-33-E5-001` | complete | no — 1,741 of 1,741, 0 disagree |
| 17 | `AT-33-E5-002` | **in-progress** | **yes** — 6,550 of 6,589 (Shortfall 1) |
| 18 | `AT-33-E5-003` | **blocked-escalated** | **yes** — 1 of 8,291 disagree, escalated not fixed (Shortfall 2) |

Rows 19-21 are Epic 6's own and are not counted against this gate.

## Deferral posture

`python3 scripts/retro.py summary --since 2026-08-24 --json` → `total 12 open 6`.
Enumerated: none of the 6 open deferrals defers DoD scope; each carries a revisit
condition. The one entry that *does* touch DoD scope is not a deferral at all —
it is `progress.md`'s `## Open blockers` entry for `rending_claw_blades`, which
is counted as Shortfall 2, not excused as a deferral.

## Disposition

**Gate FAIL.** Stop here. No retrospective, no sweep, no PR. Row 19 stays
`blocked-escalated`.

The next attempt needs, in order:

1. **Row 17** — the 39 named units. All three lanes left concrete next-cycle
   plans; the eqm family (7) needs the live-oracle equipmod-attachment
   round-trip proven at all before any of its 7 can be rowed, and the weapon
   lane independently found the same failure, so that is one root cause worth
   one owner (`dont-dispatch-two-lanes-at-one-diagnosis`).
2. **Row 18** — `rending_claw_blades`'s corpus-extraction `.MOD`-attached-EQMOD
   gap needs an owner with corpus-extractor write scope. It is escalated, which
   is not a closure path.
3. **Shortfall 3** — re-merge `full-rerun-wave5.oracle-results.json`'s 2
   unpropagated rows into the closure artifact. Cheap, mechanical, and it makes
   the closure file actually equal to what the corrected method judged.
4. **Shortfall 4** — map `('ambiguous','unmeasurable')` in
   `pf1e_dashboard_producer.py::_doneness_verdict_uncapped` (SD-33's own debt,
   3 tests), and resolve `equipment_resolver.rs:863`'s `8100` against the live
   `8119` (inherited, 1 test). A red suite is not a background condition.
