# F3a — `ClassChassis::save_shape` work list and HP / skill-point Unknown (SD-36 Epic F3, acceptance F3.0)

Spec: `epic-f-class-completion.md` §5 (review findings 3 and 11). Code:
`src/rules_core/pilot_compute/class_chassis_sheet_rules.rs`.

## Population

The test iterates **every class chassis record the class registry knows**: for each of
`class_census::census()`'s **137 ids** (every canonical registry, the prestige sweep, and
`generic_class_chassis` including the CRB/APG prestige rows F2a appended), the converted record
in each book that census row names.

| Figure | Count | Denominator |
|---|---:|---|
| chassis records resolved | 135 | 132 of 137 census ids (3 ids have a record in two books: `cyphermage`, `hellknight`, `red_mantis_assassin`) |
| ids with no chassis record | 5 | of 137: `class:monk` (CRB principal degraded to words, no `BaseAttack` row), `class:unchained_barbarian`, `class:unchained_monk`, `class:unchained_rogue`, `class:unchained_summoner` (each converted record is one rule with no `BaseAttack`/`BaseSave` target; BAB/saves come from the bespoke Pathfinder Unchained tables) |
| save slots classified | 405 | 135 records x 3 |

Pinned by `registry_ids_without_a_chassis_record_are_named` and the `records.len() == 135`
guard in `every_generic_class_save_shape_is_recognized_or_named`.

## RED -> GREEN

Command: `cargo test --locked -j 8 --lib save_shape -- --test-threads=8`

| Stage | Not Good/Poor | Of |
|---|---:|---|
| RED: classifier with the spec's two base-class forms only (`level/2+2`, `level/3`) | 227 | 405 slots (77 records) |
| Classifier fixed generically: PF1's prestige-class table forms `(level+1)/2` (good) and `(level+1)/3` (poor) added | 14 | 405 slots (6 records) |
| GREEN: the 14 pinned by name (`NAMED_UNRECOGNIZED_SAVES`) | 0 unnamed, 0 vanished | 405 |

Mechanism of the 213 slots the fix moved: CRB prestige tables print good saves as +1 at 1st to
+5 at 10th and poor as +0 to +3, i.e. `(level+1)/2` and `(level+1)/3`. These are PF1 class-level
save forms the base-only classifier missed, not per-class exceptions. The classifier compares the
`Expr`'s truncated value at every level `1..=max_level` against each closed form. It does that only
after two structural checks: degradation is ruled out first, and the `Expr` must read nothing but
constants and the class's own `ClassLevel`.

## Distribution of the 405 slots (GREEN)

Re-derivation below. It walks `census-f2a.json`'s `classes` + `prestige` rows and the converted
records, and agrees with the test's 135 records / 405 slots:

```sh
python3 - <<'PY'
import json,os,re,collections
d=json.load(open('docs/release/SD-36-consolidation/artifacts/epic-f/census-f2a.json'))
rows=[(r,False) for r in d['classes']]+[(r,True) for r in d['prestige']]
recs=[];seen=set()
for r,pr in rows:
    slug=r['class_id'][6:]
    for b in r['books']:
        f='data/sheet_rules/%s/class/%s.json'%(b,slug)
        if (b,slug) in seen or not os.path.exists(f): continue
        rules=json.load(open(f)); saves={}
        for x in rules:
            t=x.get('target')
            if isinstance(t,dict) and 'BaseSave' in t and t['BaseSave'] not in saves and isinstance(x['value'],dict) and 'Number' in x['value']:
                saves[t['BaseSave']]=x['value']['Number']
        if not any(x.get('target')=='BaseAttack' for x in rules) or len(saves)<3: continue
        seen.add((b,slug)); recs.append((pr,saves))
norm=lambda e: re.sub(r'\{"ClassLevel": "[^"]*"\}','L',json.dumps(e))
c=collections.Counter(('prestige' if pr else 'non-prestige', norm(sv[k])) for pr,sv in recs for k in ('Fortitude','Reflex','Will'))
print(len(recs), sum(c.values())); [print(v,k) for k,v in sorted(c.items())]
PY
```

| Records | Shape | Classified | Slots |
|---|---|---|---:|
| non-prestige (58 records) | `level/2+2` | Good | 88 |
| non-prestige | `level/3` | Poor | 86 |
| prestige (77 records) | `(level+1)/2` | Good | 100 |
| prestige | `(level+1)/3` | Poor | 113 |
| prestige | `level/3` | Poor | 4 |
| prestige | any other shape | **Unrecognized** | 14 |
| any | words-not-`Expr` symptom | **Degraded** | 0 |

The 4 prestige slots with the base poor form are Evangelist Fortitude and Will and Mammoth Rider
Reflex and Will. The shape matches PF1's base poor progression exactly, so they classify Poor. The
F3b fold reads each class's own `Expr` values for totals, so the table form behind a category
does not change any printed number.

## The 14 named Unrecognized slots (stay Blocked in a mix, F3b)

Each converted `Expr` is a **faithful** conversion of the source LST formula
(`data/corpus/<book>/class/<slug>.json`, `SAVE|BASE.<save>|...`). The source formula itself is not
a PF1 class-level save progression. Most look like missing parentheses: `classlevel+3/2` is
level + 1.5, so Fortitude at 10th would be 11. That makes this a source-data issue, not a
converter or classifier bug. No converter change is made in this batch (`data/**` untouched).

| Record | Save (index) | Source formula |
|---|---|---|
| `adventurers_guide:class:mammoth_rider` | Fortitude (0) | `(classlevel+2)/2` |
| `inner_sea_combat:class:pure_legion_enforcer` | Fortitude (0), Will (2) | `classlevel+3/2` |
| `inner_sea_combat:class:pure_legion_enforcer` | Reflex (1) | `classlevel+1/3` |
| `inner_sea_combat:class:ulfen_guard` | Fortitude (0), Will (2) | `classlevel+3/2` |
| `inner_sea_combat:class:ulfen_guard` | Reflex (1) | `classlevel+1/3` |
| `inner_sea_gods:class:evangelist` | Reflex (1) | `classlevel/3+1` |
| `inner_sea_gods:class:exalted` | Fortitude (0), Reflex (1) | `classlevel+1/3` |
| `inner_sea_gods:class:exalted` | Will (2) | `classlevel+1/2` |
| `inner_sea_gods:class:sentinel` | Fortitude (0) | `classlevel+1/2` |
| `inner_sea_gods:class:sentinel` | Reflex (1), Will (2) | `classlevel+1/3` |

Totals: 14 slots on 6 records, all prestige (6 of 77 prestige records). No non-prestige record is
affected.

**Follow-up (named, outside F3a):** these 6 records still produce a `row_at` progression today.
Any single-class reader that prints `ClassChassis::row_at` for them prints these values. Examples
are the desktop reference catalog (`class_catalog_generic`) and `generic_class_chassis::resolve`.
Prestige-alone is claim-blocked by F2b, so no computed character sheet prints them. The reference
catalog is F4 territory.

**Degraded:** 0 of 405. The words-not-`Expr` symptom documented at
`generic_class_chassis.rs` (the old record-wide degradation that printed a class's clean save
formula as words) is detected structurally in two cases: a `BaseSave` row whose value is not a
`Number`, beside a sibling that did convert; or an unresolved `Choice` term inside the `Expr`,
which prints as words. A record whose only row for a save is words has no chassis at all (unchanged
honest absence). Both arms are covered by `save_shape_names_the_words_not_expr_symptom_degraded`.

## F3.0 — HP / skill points Unknown, not zero

**Where HP and skill points are computed today (precise):** they are not computed from
`ClassChassis` anywhere before this step.

- HP: `durability::compute_max_hp` (consumed by the desktop `character_hub`) reads only the
  CRB/APG/ACG/Pathfinder Unchained hit-die tables. It covers single-class builds only and returns
  `None` otherwise.
- Skill points: no skill-point budget (ranks x levels + Int) exists anywhere in the crate.
  `skill_allocation::allocate_skill_ranks` applies caps and the class-skill bonus to ranks the
  input states. It never computes a budget.

This step adds the per-class terms the multiclass fold (F3b) will sum, on `ClassChassis`. That is
the one point where HP and skill points would be read off a class chassis:

- `ClassChassis::hit_points(levels, includes_first_character_level, con_mod)` uses the maximized die
  at character level 1 and the non-rolling average (`average_hit_die_value`) otherwise. Each level
  is +Con, floored at 1, the same per-level rule `compute_max_hp` applies. If `hit_die` is `None`
  it returns `Err(ChassisUnknown { id: "class_chassis.hit_points.unknown", message names the class })`.
- `ClassChassis::skill_points(levels, int_mod)` computes `levels x max(ranks + Int, 1)`. If
  `skill_ranks_per_level` is `None` it returns
  `Err(ChassisUnknown { id: "class_chassis.skill_points.unknown", ... })`.

Oracles (hand-worked PF1):

| Case | Expected | Result |
|---|---:|---|
| Warrior 3, including character level 1, Con +2 | 28 | Ok(28) |
| Warrior 3, not including character level 1, Con +2 | 24 | Ok(24) |
| Commoner 2, Con -3 (floor 1) | 4 | Ok(4) |
| 2 ranks, Int -2, 3 levels (min 1) | 3 | Ok(3) |
| 4 ranks, Int +1, 5 levels | 25 | Ok(25) |

**Named lists:**

| Field `None` | Over 135 registry chassis records | Over all converted class files (189*) |
|---|---:|---:|
| `hit_die` -> HP Unknown | **0 of 135** | 11 of 189 files carry no `StatBlock "Hit die"` row. None of them has a chassis: `bestiary/sorcerer_cleric_arcane`, `occult_adventures/psychic_detective`, `pathfinder_unchained/{unchained_barbarian, unchained_monk, unchained_rogue, unchained_summoner}`, `ultimate_intrigue/{vcabalist, vwarlock}`, `ultimate_psionics/{gifted_blade, gifted_blade_marksman_power_list, unlocked_talent}` |
| `skill_ranks_per_level` -> skill points Unknown | **135 of 135** (every one, named in the test's per-record walk) | 189 of 189: no class file carries a `StatBlock "Skill ranks per level"` row |

\* The spec text says 185 class records. The count is 189 today: F1c added the 4
`pathfinder_unchained` class files. Re-derive with
`ls data/sheet_rules/*/class/*.json | wc -l`. The hit-die and skill-ranks columns come from a
Python walk of each file's principal-rule `prose` `StatBlock` labels.

Consequence for F3b: until a converter reads a skill-ranks row, every chassis class's skill
points print **Unknown**, never 0. Adding that row is a converter and data change outside this batch.

Tests: `a_class_missing_hit_die_reports_hp_unknown` and
`a_class_missing_skill_ranks_reports_skill_points_unknown` (`--lib`). RED was a compile failure:
`hit_points`, `skill_points` and the two diagnostic ids did not exist. RED logs:
`f3a-red.log`; GREEN verify log: `f3a-verify.log` (both in this directory).
