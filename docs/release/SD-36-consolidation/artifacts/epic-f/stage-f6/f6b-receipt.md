# F6b receipt: the HP source rule, and feat skill bonuses folded from the record

Stage F6, step F6b (`polish.json` items F6-3 and F6-2b). Branch `sd36/epic-f6-desktop-polish`,
on F6a `6cd4442559`. Out of scope here: F6-4, F6-5, F6-6, P1-P4, #28.

## 1. (a) One hit-die rule

**Rule** (`src/rules_core/pilot_compute/hit_die_source.rs`): the source that COMPUTES a class's
hit points is also the source whose die the sheet prints, and the roster offers a class only if
that source exists. `hit_die_source(class_id) -> Option<HitDieSource { die, source }>` checks two
tiers in order:

1. **The bespoke class module.** This is `durability::bespoke_hit_die`, pulled out of
   `compute_max_hp`'s existing chain: the CRB class table, then APG, ACG and Pathfinder Unchained.
   The CRB Monk is d8 here (CRB p.56; operator ruling 2026-07-29), and Unchained Monk is d10 (Pathfinder Unchained).
2. **The converted class record.** This is `class_chassis_sheet_rules::hit_die_from_package`: the
   principal's `StatBlock "Hit die"` row. The four Unchained records state no die. Their only row
   is `grants: [TakenOnClass(<base>)]`, so the reader follows that edge to the base class line the
   character holds (`SheetRulePackage::base_class_of`).

`None` from both tiers means Unknown (`class_chassis.hit_points.unknown`). No die is ever made up.

What reads it:

| Reader | Before | After |
|---|---|---|
| Census roster reason (`class_census::roster_hit_die`, `hit_die_absent`) | converted principal row | `hit_die_source` |
| Multiclass HP fold (`multiclass_fold::explain_multiclass_fold`) | the converted CHASSIS record's die (`None` for all five) | `hit_die_source::class_hit_points` |
| Desktop roster `hitDie` / `hitPointsDie`, `list_class_facts.hitDie` | principal row / chassis record | `hit_die_source`, both fields |
| Printed class line `Hit die:` prose (`with_sheet_rules`) | the converted record's rendered row (Monk `d10`) | `print_hit_die_lines`: `Hit die: d<source die>` |

`ClassChassis::hit_points` and the fold both call the same `hit_points_from_die`.

**FS-23.** The oracle line is still `CLASS:Monk HD:10 ... SOURCEPAGE:p.56`
(`core_rulebook/cr_classes.lst:147`), and the package still states `Hit die d10`. The sheet is
right because Monk's hit points come from the CRB table, which says d8. The class line prints
`Hit die: d8` and the d10 row is replaced, not printed. The test
`bespoke_and_converted_dice_agree_except_the_fs23_monk` checks the two tiers over all **137**
census ids. Both tiers answer for 31, only the converted tier for 106, only the bespoke tier for 0,
and neither for 0. They disagree on exactly one class: `class:monk: bespoke d8, converted d10`.
`every_chassis_record_die_is_unchanged_by_the_source_rule` shows that for **132 of 137** census ids
the fold reads the same die as before (the ids that have a chassis-record die). The five that
change are the five below. FS-23 is now marked sheet-side closed, and FS-24 is closed
(`forward-scope-register.md`).

**Hand-worked HP** on the census fixture (Human Fighter fixture, Con 14, so +2). Level 1 takes the
full die; levels 2-5 take the average (die/2 + 1); Con is added every level:

| Class 5 | Die (source) | HP |
|---|---|---|
| Monk | d8, CRB p.56 (`rules_tables::crb::class_tables`) | 8+2 + 4 x (5+2) = **38** |
| Unchained Barbarian | d12, Pathfinder Unchained class entry (`rules_tables::pathfinder_unchained::class_chassis`) | 12+2 + 4 x (7+2) = **50** |
| Unchained Monk | d10, Pathfinder Unchained class entry (same module) | 10+2 + 4 x (6+2) = **44** |
| Unchained Rogue | d8, Pathfinder Unchained class entry (same module) | 8+2 + 4 x (5+2) = **38** |
| Unchained Summoner | d8, Pathfinder Unchained class entry (same module) | 8+2 + 4 x (5+2) = **38** |

Also checked: Monk 4 / Rogue 3 mix = 31 + 21 = **52**
(`tests/sd36_f3_polish.rs::p4_monk_mix_hit_points_come_from_the_bespoke_d8`). Before F6b this mix
was HP Unknown.

**Roster.** Still **59** offered, with 0 `hit_die_absent`. **0 of 59** offered classes are now
HP Unknown (was 5 of 59). The census JSON (`class_census --json`) matches
`stage-f4-f5/census-f5.json` in every field except `generated_at`: ids 137, computed 63,
roster_offered 59, prestige_mix_computed 68, mix_panel 185/185. The HP change reaches only the
fold explanations, the roster wire and the printed class line, and the census JSON records none of
those. Wire diffs:

- `f4c-class-roster-wire.json`: Monk `hitDie` 10 -> 8, `hitPointsDie` null -> 8. The four Unchained
  classes go from `hitPointsDie` null to 12/10/8/8.
- `f6a-class-facts-wire.json`: the same five classes, `hitDie` null to their dice at levels 1 and 7.

## 2. (b) Feat skill bonuses, read from the held feat records

**Rule** (`src/rules_core/pilot_compute/feat_skill_bonus_sheet_rules.rs`):
`feat_skill_bonuses(package, rendered lines)` folds the same lines the sheet prints. A feat line
is added to a skill total when all three hold:

- its converted rule targets `Skill(<id>)` or `SkillGroup(<family>)`;
- it resolved to a number;
- it has no condition.

The fold stacks by bonus type the same way the package's `Var` fold does. Untyped, negative and
`STACKING_TYPES` bonuses sum; a second bonus of any other type takes the max. A conditional line is
printed and listed as `situational`. A line that resolves to words is listed as `unknown`. Neither
is added. The desktop sends the fold on the sheet response as `featSkillBonuses`
(`sheet_lines_for`). The Skills panel and the allocation dialog add `skills[<id>]` to each skill,
plus `groups[<family>]` to every skill in that family. A folded bonus whose skill has no panel row
(for example `craft_alchemy`) is printed as a note under the panel, so nothing is dropped.

**Population.** From `every_skill_bonus_feat_is_counted` (`--nocapture`, pinned). Each feat was
held alone as the census fixture's level-1 feat:

| | Feat slugs | |
|---|---|---|
| Carry a skill-bonus rule (denominator) | **43** (115 rules: 113 `Skill` + 2 `SkillGroup`; 53 feat records across books) | |
| **Fold into a skill total** | **40 of 43** | Alertness, Acrobatic, Athletic, Deceitful, Stealthy, Magical Aptitude, ... (list in `f6b-green.log`) |
| Situational only (printed, not added) | 2 of 43 | advanced_infiltrator_path, taldan_duelist |
| Unknown (words) | 0 of 43 | |
| No skill line on the fixture | 1 of 43 | sociable (held only by a half-elf with Cha 13+) |

Command: `cargo test --locked -j 8 --lib -- --test-threads=8 --nocapture feat_skill_bonus`.

A feat's own prerequisite (its rule's `applies`, e.g. Sea Legs' Profession (sailor) 5 ranks) is
still the job of the level-up option filter, as before. The fold adds exactly what the printed feat
line shows.

**Alertness** (CRB p.117): the census fixture with Alertness as its level-1 feat, in place of
Power Attack, folds `perception +2` and `sense_motive +2` and nothing else. That result is pinned
engine-side (`alertness_adds_two_to_perception_and_sense_motive_at_level_one`) and on the served
wire (`f6b-feat-skill-bonus-wire.json`, pinned by
`character_hub::tests::feat_skill_bonuses_wire_for_the_census_fixture_with_alertness_matches_the_committed_artifact`).
The Skills panel prints Perception +3: Wis +1, 0 ranks, Alertness +2 (`skillsModel.test.ts`).

**Remainder, by mechanism** (none of it is added, all of it is named):

- **`Chosen` targets.** 5 feat rules, Skill Focus among them, target the character's pick, not a
  fixed skill. The fold does not read the pick yet.
- **`SkillSituation` targets.** 28 feat rules print with their situation and are never added.
- **Engine-totalled rows.** On the GE-06 pilot posture (`realModifiers`), the Climb / Intimidate /
  Swim rows print the engine's pilot total. That total's feat term is `feat_effects`' own reader
  (Athletic, Persuasive, Intimidating Prowess, and Sure and Fleet through `arg_climb_bonus`). The
  panel does not add the fold on top, because that would count those four twice. As a result,
  7 of the 40 folding feats that also target those three skills are not in those three rows on
  that posture: scarred_legion, trlbal_scars_bearpelt, trlbal_scars_ice_chasm, monkey_moves,
  sea_legs, aquatic_combatant, improvisation. The fix is to make the pilot total read this fold
  instead of the 4-feat reader. That is an engine change to `compute_selected_skill_modifiers` and
  touches the census posture.
- **Non-feat records.** Traits, racial traits and class features that grant skill bonuses are not
  in this fold.

## 3. RED -> GREEN

`f6b-red.log`:

- `the_five_hp_unknown_classes_now_print_hp` against the pre-F6b rule (the chassis-record die, which
  the fold and `hitPointsDie` read): **5 of 5 HP Unknown**, one line per class naming it.
- `classRoster.test.ts` on the pre-F6b served wire:
  `Monk d8 (CRB p.56), from the source that computes its HP: expected 8, got null`.
- `skillsModel.test.ts` against the pre-F6b model: no `featSkillBonusFor` export. The behavioural
  probe: `FAIL Perception total with Alertness ...: expected +3, pre-F6b panel prints +1`.

`f6b-green.log`: the 8 new engine tests pass, with the counts printed. `classRoster`, `skillsModel`
and `characterProgression` tests exit 0.

## 4. Verification (one pass, after all changes): `f6b-verify.log`

| Check | Result |
|---|---|
| root lib `cargo test --locked -j 8 --lib -- --test-threads=8` (covers class_census, class_chassis_sheet_rules, skill*) | 2741 passed, 0 failed, 6 ignored |
| the 7 `tests/sd36_*.rs` files | 54 passed, 0 failed |
| root clippy `--tests` | exit 0, 0 warnings |
| desktop `cargo test` (roster, character_hub, class_facts) | 625 passed, 0 failed |
| desktop clippy `--tests` | exit 0, 0 warnings |
| frontend `npm run typecheck && npm test` | clean; 127/127 test files |
| `python3 scripts/pcgen_residue_gate.py --check --closure` | live_hits=0, PASS |
| `class_census --json` vs census-f5 | identical except `generated_at` |

`data/corpus/**`, `site/**` and `data/sheet_rules/**` were not touched. No converter step ran. No
app was launched.
