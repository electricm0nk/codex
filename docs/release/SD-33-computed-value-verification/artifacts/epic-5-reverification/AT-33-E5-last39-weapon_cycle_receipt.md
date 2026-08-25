# Cycle AT-33-E5-last39-weapon — Epic 5 Re-verification / AT-33-E5-002, AT-33-E5-003

- **Commit SHA:** recorded below at push time (`sd33-r6-weapon`, remediation wave 6)
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/last39-weapon.oracle-results.json` (new — this lane's committed deliverable, 23 rows)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-last39-weapon_cycle_receipt.md` (this file)
  - `scripts/oracle_harness/weapon-family.txt.ftl` (new — reusable `WEAPON.n` batch-dump BatchExporter template, generalized from `AT-33-E5-last67-weapon`'s uncommitted-scratch fixture per this cycle's write scope)
  - `docs/release/SD-33-computed-value-verification/progress.md` / `kanban.md` (updated in place, next commit)
  - `docs/retro/events/sd33-r6-weapon.jsonl` (new)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
  (`git diff --unified=0 f53b8e32da2ae1939b9ddb1b8375ba1baefd00ba...HEAD -- docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/last39-weapon.oracle-results.json scripts/oracle_harness/weapon-family.txt.ftl ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → no match)
- **Wired-integration audit result:** OK_NO_TOKENS (same diff, second pattern, no match)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-002 — the 6,589 `literal-verified` units are re-examined
  > **Evidence:** per-unit `(ours, oracle, verdict)` rows committed; agreement and disagreement counts both stated, with the denominator.
  >
  > ### AT-33-E5-003 — every disagreement is a named defect, fixed or escalated
  > A disagreement is **never** closed by adjusting the expectation to match our output. Each is root-caused: either our computation is wrong (fix it) or the oracle comparison is wrong (fix the harness, and re-run everything it already judged).

## Population re-derivation (first action, per the brief)

```
$ python3 -c "import json
wi=json.load(open('docs/work-inventory.json'))['units']
pop={u['id'] for u in wi if u.get('status') in ('literal-verified','fixture-verified')}
d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json'))['results']
miss=sorted(pop-{r['unit_id'] for r in d})
print(len(miss)); [print(m) for m in miss]"
39
<39 ids, unchanged from AT-33-E6-001 attempt 6's list>
```

This lane's population is **23 of the 39**, derived by set-subtracting the sibling
lanes' own named populations (9 skill-combat-shape, from
`AT-33-E5-last67-skill-combat_cycle_receipt.md`'s "The 9 NOT examined this
cycle" section; 7 eqm-shape, from `AT-33-E5-last67-eqm_cycle_receipt.md`'s
population table) from the 39:

```
$ python3 -c "
skill_combat = {'core_rulebook:equipment:rod_alertness','core_rulebook:equipment:stone_of_good_luck_luckstone','ultimate_equipment:equipment:gunfighter_s_poncho','ultimate_equipment:equipment:robe_of_vermin','ultimate_equipment:equipment:scattershot_bracers','ultimate_equipment:equipment:staff_of_the_hierophant','ultimate_psionics:equipment:companion_stone_far_sight','ultimate_psionics:equipment_modifier:special_quality_dissonance_enhancement_bonus_alt','ultimate_psionics:equipment_modifier:special_quality_dissonance_enhancement_bonus_main'}
eqm = {'core_rulebook:equipment_modifier:draco','core_rulebook:equipment_modifier:dragonhide','core_rulebook:equipment_modifier:material_dragonhide','core_rulebook:equipment_modifier:special_quality_spikes_shieldbash','core_rulebook:equipment_modifier:spike_sb','ultimate_combat:equipment:arrow_iron_tipped_distance_20','advanced_race_guide:equipment_modifier:material_darkleaf_cloth_clothing'}
print(len(skill_combat), len(eqm))
"
9 7
```

`39 - 9 - 7 = 23`, verified by direct set subtraction against the full 39-id
list (script in this receipt's own history; output below matches this lane's
committed rows exactly, one for one).

## Shape table

| Shape | Population | Examined | Verdicts |
|---|---:|---:|---|
| `WEAPONPROF=<x>` family, real wielded weapon (`compute_equipmods_effect`-covered, no natural-attack host needed) | 1 | 1 | agree: 1 |
| `WEAPONPROF=<x>` family, needs a natural-attack host (`TYPE.Natural`/`Bite`/`Hoof`/`Claw`) | 12 | 12 | unverifiable (`no_probe_surface`): 12 — real `ours`, oracle unreachable this cycle |
| `WEAPON\|DAMAGEMULT` fractional crit-multiplier | 3 | 3 | unverifiable (`no_comparable_export_token`): 3 |
| bare `WEAPON\|TOHIT`, no `TYPE=`, ammunition | 1 | 1 | unverifiable (`no_resolver`): 1 |
| bare `WEAPON\|TOHIT`, no `TYPE=`, wield-size equipment_modifier | 3 | 3 | unverifiable (`no_resolver`): 3 |
| `WEAPON\|ATTACKS` formula-valued flurry shape | 3 | 3 | unverifiable (`no_resolver`): 3 |
| **Total** | **23** | **23** | **agree 1, unverifiable 22 (0 reasonless)** |

Re-derive:
```
$ python3 -c "import json,collections
d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/last39-weapon.oracle-results.json'))['results']
print('rows', len(d)); print(collections.Counter(x['verdict'] for x in d))
print('reasonless', len([x for x in d if x['verdict']=='unverifiable' and not (x.get('reason') or '').strip()]))
ids=[x['unit_id'] for x in d]; print('dupes', len(ids)-len(set(ids)))"
rows 23
Counter({'unverifiable': 22, 'agree': 1})
reasonless 0
dupes 0

$ python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/last39-weapon.oracle-results.json
uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False
EXIT=0
```

## The 23 rows

Full rows (`ours`, `oracle`, `verdict`, `reason`/`note`) are in
`last39-weapon.oracle-results.json`. Summary:

| unit_id | ours | oracle | verdict | reason |
|---|---:|---:|---|---|
| `ultimate_equipment:equipment:cursed_sword_2` | -2 | -2 | **agree** | — |
| `core_rulebook:equipment:amulet_of_mighty_fists_1..5` (5) | 1..5 | null | unverifiable | `no_probe_surface` |
| `ultimate_equipment:equipment:belt_of_teeth` | 4 | null | unverifiable | `no_probe_surface` |
| `ultimate_equipment:equipment:horseshoes_of_crushing_blows_1..5` (5) | 1..5 | null | unverifiable | `no_probe_surface` |
| `ultimate_equipment:equipment:talons_of_leng` | 3 | null | unverifiable | `no_probe_surface` |
| `advanced_class_guide:equipment:duelist_s_comate` | null | null | unverifiable | `no_comparable_export_token` |
| `advanced_class_guide:equipment:rapier_of_battlefield_movement` | null | null | unverifiable | `no_comparable_export_token` |
| `advanced_class_guide:equipment:swashbuckler_s_rapier` | null | null | unverifiable | `no_comparable_export_token` |
| `advanced_race_guide:equipment:heartstake_bolts_5` | null | null | unverifiable | `no_resolver` |
| `core_rulebook:equipment_modifier:special_quality_wield_size_1/2/3_step(s)_greater_no_penalty` (3) | null | null | unverifiable | `no_resolver` |
| `advanced_class_guide:equipment:brawler_s_flurry` | null | null | unverifiable | `no_resolver` |
| `ultimate_psionics:equipment:flurry_of_fists` | null | null | unverifiable | `no_resolver` |
| `ultimate_psionics:equipment:flurry_of_strikes` | null | null | unverifiable | `no_resolver` |

## Method — real "ours" for all 23, real oracle where reached

**"ours"**: reused `src/bin/e5_last67_weapon_ours.rs` UNMODIFIED (per write
scope: "extend the existing equipment probe binaries; do not fork them" — no
fork needed, the existing manifest-driven binary already generalizes to any
unit given `book`+`key`). Built a 23-unit manifest (this lane's own scratch
file, not committed — reproducible from the population table above) and ran:

```
$ cargo build --locked --bin e5_last67_weapon_ours
   Finished (clean; only pre-existing unrelated warnings elsewhere in the crate)
$ cargo run --locked --bin e5_last67_weapon_ours -- <repo_root> manifest.json ours-output.json
e5_last67_weapon_ours: 23 units in manifest, 23 resolved, 0 unresolved -> ours-output.json
```

All 23 units resolved (including the 3 `equipment_modifier` records, found in
`data/corpus/core_rulebook/equipment/equipmods/`). 13 of 23 returned a real,
non-null `weapon_enhancement_bonus`; 10 of 23 returned `null` — both are real
probe outputs, not fabricated, and match this lane's shape classification
exactly (verified by reading each of the 23 units' full `raw_bonus_chains`
before classifying, not a BONUS-filtered view).

**oracle**: `scripts/oracle_harness/charbuild_remainder_run_one.sh`
(unmodified, reused) + this cycle's new `weapon-family.txt.ftl` against
hand-built `.pcg` fixtures (scratch, not committed — reproducible from this
receipt, same convention `AT-33-E5-last67-weapon` established), one L1 Human
Fighter (Core Rulebook + Ultimate Equipment + Core Essentials) wielding
`Cursed Sword (-2)` as its own weapon. Live oracle:
`WEAPON.0.MAGICHIT=-2, WEAPON.0.MAGICDAMAGE=-2` — agrees with `ours` on both
halves.

## The natural-attack investigation (12 units) — three real, live attempts, all named

This lane's largest sub-population (12 of 23) needs a character that actually
has the natural-attack weapon type (`TYPE.Natural`/`Bite`/`Hoof`/`Claw`) the
`WEAPONPROF=` bonus targets. Three independent, real, live attempts were made
this cycle (not "ran out of time" vaguely):

1. **Equip a NATURALATTACKS-self-granting item.** `Belt of Teeth`
   (`NATURALATTACKS:Bite,Weapon.Natural...,*1,1d6`) and `Talons of Leng`
   (`NATURALATTACKS:Claw,Weapon.Natural...,*2,1d4`) both carry their OWN
   natural-attack grant. Equipped both alongside `Cursed Sword (-2)` in one
   fixture (three different body slots — Belt/Glove/Weapon, no conflict):
   `WEAPON.COUNT=1` (only the literally-wielded Cursed Sword; Bite and Claw
   never appeared as `WEAPON.n` rows).
2. **`TEMPLATESAPPLIED:Hoof 2 (Medium)`** — a real, in-pin PCGen
   monster-building template (`core_essentials/ce_templates.lst:574`,
   `NATURALATTACKS:Hoof,Weapon.Natural...,*2,1d4`), tried in both the
   bracketed `TEMPLATESAPPLIED:[NAME:Hoof 2 (Medium)]` and bare
   `TEMPLATESAPPLIED:Hoof 2 (Medium)` `.pcg` forms
   (`pcgen.io.PCGVer2Parser.parseTemplateLine`, both branches read directly).
   Result: `TEMPLATE.COUNT=0` on export — the template key lookup itself did
   not resolve either way; root cause not isolated within this cycle's
   budget (not a corpus/data issue — `core_essentials` loaded cleanly, its
   own unrelated `IsOrc` FACT-token LST error is inherited/pre-existing and
   unrelated to this template).
3. **Monk wielding the corpus's own `Unarmed Strike`.** `Unarmed Strike`
   (`cr_equip_arms_armor.lst:296`) is a real, standalone Equipment record
   whose own `TYPE` list includes `Natural` — unlike (1)/(2), this is a
   normal wielded weapon, not a bare NATURALATTACKS grant, so it DOES appear
   cleanly under `WEAPON.n` (`WEAPON.0.OUTPUTNAME=Unarmed Strike,
   MAGICHIT=+0` baseline, proving the export mechanism itself works for a
   genuine natural-typed weapon). Adding `Amulet of Mighty Fists +1`
   alongside it left `MAGICHIT=+0` — no delta, meaning
   `WEAPONPROF=TYPE.Natural` did not apply here: `Unarmed Strike`'s own
   `WeaponProf` record (`cr_profs_weapon.lst:14`) carries no `Natural` TYPE
   token itself; only a conditional `.MOD` entry
   (`cr_profs_weapon.lst:202`, `TYPE:Monk.Natural.Weapon Group Monk`) adds
   it, and this cycle did not prove that `.MOD`'s live activation.

Root-caused as far as time allowed: PCGen's `NaturalWeaponFacet`/
`NaturalEquipSetFacet` chain (`code/src/java/pcgen/cdom/facet/`) listens for
CDOMObjects added via the generic `charObjectFacet` (confirmed in source —
`FacetInitialization.java:134`, `charObjectFacet.addDataFacetChangeListener
(naturalWeaponFacet)`), which is the channel Race/Template/Ability grants use.
Whether an Equipment record's own `NATURALATTACKS` sub-grant routes through
that same channel in this direct-`java` `BatchExporter` path was not
confirmed either way this cycle — attempt (1)'s null result is consistent
with "it does not" but is not proof of the internal mechanism, only of the
observed export output.

**This is a real, reproducible, well-evidenced probe-surface gap, named for
the next cycle — not a comparison this cycle skipped.** `ours` is real
(non-null) for all 12; `no_probe_surface` is the correct, established verdict
(distinct from `no_resolver`, where `ours` is also null) per this bundle's
verdict vocabulary.

## Verdict discipline

- `cursed_sword_2`: real `agree`, both `ours` and `oracle` from live sources,
  no hand-typing.
- The 12 natural-attack units: `ours` is real (the probe's own non-null
  output); `oracle` is genuinely unreachable this cycle (three real attempts,
  all documented above) — `unverifiable`/`no_probe_surface`, not a fabricated
  comparison.
- The 3 `DAMAGEMULT` (Advanced Class Guide) units: applied
  `AT-33-E5-last67-weapon`'s own established rule directly (`sword_cane`'s
  precedent: no PCGen export token isolates a fractional per-attack
  multiplier delta, proven live on that unit already) — **no truncation
  performed**, confirmed by inspection of this lane's own committed rows
  (`ours: null, oracle: null` for all 3, matching `sword_cane`'s row exactly
  in shape).
- The bare `WEAPON` chains (`heartstake_bolts_5`, the 3 wield-size units): the
  established rule (`AT-33-E5-last67-weapon` receipt, line 260) fixes `TOHIT`
  as the comparable magnitude for `TYPE=`-less bare `WEAPON` chains; `ours`
  confirmed `null` live via the probe (not assumed from the rule alone).
- The 3 flurry units: `ATTACKS` is the comparable magnitude per this lane's
  brief (an attack count, not a to-hit value) — confirmed the engine has
  **zero** resolver for `WEAPON|ATTACKS` chains at all (`grep -rn '"ATTACKS"'
  src/rules_core/equipment_effects*.rs src/rules_core/equipment_effects/*.rs`
  → no matches), so `ours` is genuinely `null`, not a missed comparison.

No `unverifiable` row was truncated, rounded, or otherwise manufactured into
a false `agree`/`disagree`. `oracle_disagreement=0` — this lane surfaced no
new disagreement (unlike wave 5's weapon lane, which surfaced 2 real ones);
`cursed_sword_2` was the one shape this cycle could fully close live.

## RED→GREEN

No `src/rules_core/` production code changed this cycle — every shape
examined resolves (or correctly does not resolve) via the existing, already-
tested `compute_equipmods_effect`. The only new production tooling is
`scripts/oracle_harness/weapon-family.txt.ftl` (a BatchExporter template, not
application code): RED — before this cycle, no committed, reusable `WEAPON.n`
batch-dump template existed in `scripts/oracle_harness/` (wave 5's own
equivalent was scratch, explicitly not committed per its own receipt). GREEN
— `weapon-family.txt.ftl` run live against `Cursed Sword (-2)` produces the
exact `WEAPON.0.MAGICHIT=-2/MAGICDAMAGE=-2` this receipt cites, confirmed via
a real `charbuild_remainder_run_one.sh` invocation (shown above), and the
`Unarmed Strike` control run confirms it correctly returns `+0`/`+0` for a
zero-bonus baseline weapon (not a hardcoded value — the same template, two
different live characters, two different correct outputs).

## Test scoping

Ran `cargo build --locked --bin e5_last67_weapon_ours` and
`cargo run --locked --bin e5_last67_weapon_ours` (23/23 resolved, shown
above). Ran `charbuild_remainder_run_one.sh` four times live against the
pinned oracle (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`,
resolved via `$PCGEN_REPO_DIR` — reused, not rebuilt, on-pin checkout
verified this cycle: `cat $PCGEN_REPO_DIR/.git/refs/heads/master` →
`7f818006e371188e5717fd18d74d18a420747fc6`, matching
`scripts/pcgen-oracle-pin.env`, the same "reused, not rebuilt" precedent
`AT-33-E5-002`/`AT-33-E5-last67-skill-combat` already established): the
cursed-sword control (agree), the natural-attack control (`WEAPON.COUNT=1`),
the `TEMPLATESAPPLIED` attempt (`TEMPLATE.COUNT=0`), and the Unarmed
Strike/amulet delta test (`MAGICHIT=+0`, no delta). Ran
`python3 scripts/box_ledger.py --check --oracle-results
last39-weapon.oracle-results.json` (shown above, exit 0). **Did not** run the
root `cargo test` sweep or `apps/desktop/src-tauri` — no `src/rules_core/`
file changed this cycle (a new `scripts/oracle_harness/*.ftl` template and a
results JSON only); the probe's own build+run against the existing,
already-tested `compute_equipment_effects`/`compute_equipmods_effect` is the
real, live verification of correctness for the `ours` side, and
`box_ledger.py --check`'s clean exit is the real, live verification for the
committed results file's shape.

## Status: complete

**All 23 of this lane's population carry a committed `(ours, oracle,
verdict)` row.** 1 real `agree` (live both sides); 22 `unverifiable`, every
one with a populated, real, non-generic reason — 12 with a real non-null
`ours` and a genuinely-unreached oracle (`no_probe_surface`, three documented
live attempts), 10 with a real, probe-confirmed `null` `ours`
(`no_comparable_export_token` ×3, `no_resolver` ×7). 0 `disagree`. 0
reasonless `unverifiable`. 0 duplicate `unit_id`s. This is `complete` under
this lane's own Definition of Done ("every one of your 23 units carries a
per-unit row") — it does **not** by itself close `AT-33-E5-002`/`003` at the
bundle level, since the sibling skill-combat/eqm lanes' own populations and
Shortfall 2/3 from `AT-33-E6-001` attempt 6 are outside this lane's scope.

## Movement, four buckets

- **Closure:** 23 units of this lane's 23-unit population get a real,
  committed oracle disposition for the first time (1 agree, 22
  unverifiable, each reasoned) — the full 23-unit remainder this wave's
  three sibling lanes' shape tables named as `weapon-shape-final`.
- **Reclassification:** none — no unit's `docs/work-inventory.json` `status`
  field changed.
- **Reachability:** `cursed_sword_2` (real weapon, `WEAPONPROF=<x>` family)
  needed zero new `src/rules_core/` code, matching wave 5's own finding for
  this family. The 12 natural-attack units' `ours` side is now reachable
  (real, non-null, probe-confirmed) even though the oracle side is not yet —
  this narrows the remaining gap from "shape unclassified" to "one specific,
  well-evidenced harness mechanism" for the next cycle.
- **Instrument-correction:** none this cycle (no prior finding was revised).
  Three new, real, well-evidenced findings recorded for next-cycle
  remediation (the `NaturalWeaponFacet`/`NaturalEquipSetFacet` Equipment-
  source gap, the `TEMPLATESAPPLIED` key-lookup failure, and the
  `Unarmed Strike` `.MOD`-conditional `Natural` TYPE non-activation) — none
  smoothed over into a false `agree`.

## Notes

Judgment calls made explicit per the brief:
- **Comparable magnitude for the 13-unit real/reachable `WEAPONPROF=<x>`
  family:** `DAMAGE` for `cursed_sword_2` (TOHIT and DAMAGE numerically
  identical, `DAMAGE` per this lane's inherited convention); the 12
  natural-attack units' `ours` values are reported as their single
  `tohit_bonus`/`damage_bonus` pair (identical for all 12 except
  `belt_of_teeth`, which is TOHIT-only) in each row's `note`, with no
  `oracle`/`verdict` fabricated for the missing comparison side.
- **`WEAPON|DAMAGEMULT` fractional:** inherited `AT-33-E5-last67-weapon`'s
  established rule directly (no re-derivation needed — that lane already
  proved live, on `sword_cane`, that no PCGen token isolates this shape); **no
  truncation performed**, confirmed in this lane's own committed rows.
- **Bare `WEAPON\|TOHIT,DAMAGE,ATTACKS`, no `TYPE=`:** `TOHIT` is the
  comparable magnitude for the TOHIT-shaped units (`heartstake_bolts_5`, the
  3 wield-size units), per the inherited rule; `ATTACKS` (an attack count,
  not a to-hit value) is the comparable magnitude for the 3 flurry units, per
  this lane's own brief — both confirmed `ours=null` live via the probe, not
  assumed from the rule text alone.
- **Wield-size no-penalty variants:** compared the comparable `TOHIT` half
  only, exactly as stated; the `WIELDCATEGORY` half stays
  unverifiable/non-scalar per `AT-33-E5-last75` and was not re-litigated.
  These are `equipment_modifier` records (EQMOD-attached, not standalone
  equippable items) — `AT-33-E5-last67-weapon`'s Finding 5 (proving the live
  `CUSTOMIZATION:[BASEITEM:...|DATA:EQMOD=...]` attachment syntax) remains
  open next-cycle scope; not reached this cycle either, since the verdict
  (`no_resolver`) is settled by `ours=null` regardless of oracle
  reachability.

## Next-cycle plan

1. Root-cause the `NaturalWeaponFacet`/`NaturalEquipSetFacet` Equipment-
   source gap (or the `TEMPLATESAPPLIED` key-lookup failure — either one
   would unblock the 12 `no_probe_surface` units) — the single largest
   remaining lever in this lane's own population, and likely shared by any
   future natural-attack-shaped unit elsewhere in the corpus.
2. Prove the `.MOD`-conditional `Natural` TYPE activation on `Unarmed
   Strike` for a Monk (an alternative, narrower path to the same 12 units,
   via `WEAPONPROF=TYPE.Natural` specifically, though it would not reach the
   `Bite`/`Hoof`/`Claw`-named subset).
3. Build the launcher+ammunition fixture pair for `heartstake_bolts_5`
   (`AT-33-E5-last67-weapon`'s own plan item 4, still open).
4. Prove the wield-size `CUSTOMIZATION:[BASEITEM:...|DATA:EQMOD=...]`
   attachment syntax (`AT-33-E5-last67-weapon`'s Finding 5, still open).
5. This lane's own 23-unit population is now fully rowed; the bundle-level
   remainder is the sibling lanes' own populations (skill-combat 9,
   eqm-modifier-family 7) plus `AT-33-E6-001` attempt 6's Shortfalls 2/3
   (the `rending_claw_blades` disagreement and the 2 unmerged
   `full-rerun-wave5` rows), all outside this lane's write scope.
