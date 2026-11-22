# F3d — sabotage parity for the status-parity negative controls (F3.3, re-measured)

Ruling: `decisions.md` §14.2. Population: the 187 tests of `f3d-sites.tsv` (64
`MULTICLASS_NEG_ROWS` + 59 `multiclass_negative_controls!` rows + 64 hand-written), each now
asserting `common::assert_multiclass_status_parity` (mix receipt status == class-alone status
AND mix claim-blocking set, `multiclass.<class>.` re-scope stripped, == class-alone set;
vacuity guard: the mix loads >= 2 classes and more than the class alone).

## Sabotage

Mechanism (the one the ruling names): disable the fold's carry-over of each class's own
claim-blocking class lines into the mix -- `src/rules_core/pilot_compute/multiclass_fold.rs`,
`explain_multiclass_fold`, the isolated-run diagnostic loop's guard became
`if true // F3D-SABOTAGE ... || !diagnostic.claim_blocking || ...` (every carried line skipped).

Command (the 45 binaries, filter `multiclass_`, which matches all 187 plus 28 other
`multiclass_*` tests in those binaries):

    cargo test --locked -j 8 --no-fail-fast --test sd18_widening --test sd13_progression --test sd13_barbarian_level1_chassis_baseline --test sd13_barbarian_rage_power_slots --test sd13_bard_bonus_spells --test sd13_bard_spell_level_thresholds --test sd13_bard_spell_save_dcs --test sd13_bard_spells_known_counts --test sd13_bard_spells_per_day_counts --test sd13_bard_total_spells_per_day --test sd13_bard_versatile_performance_slots --test sd13_monk_bonus_feats_three_and_four --test sd13_monk_level1_chassis_baseline --test sd13_monk_second_bonus_feat --test sd13_paladin_bonus_spells --test sd13_paladin_mercies_two_and_three --test sd13_paladin_spell_level_thresholds --test sd13_paladin_spell_save_dcs --test sd13_paladin_spells_per_day_counts --test sd13_paladin_total_spells_per_day --test sd13_ranger_bonus_spells --test sd13_ranger_second_favored_terrain --test sd13_ranger_spell_level_thresholds --test sd13_ranger_spell_save_dcs --test sd13_ranger_spells_per_day_counts --test sd13_ranger_third_favored_enemy --test sd13_ranger_total_spells_per_day --test sd13_rogue_level1_chassis_baseline --test sd13_rogue_second_talent --test sd13_rogue_talent_choice --test sd13_rogue_talents_three_through_five --test sd13_sorcerer_bonus_spells --test sd13_sorcerer_spell_level_thresholds --test sd13_sorcerer_spell_save_dcs --test sd13_sorcerer_spells_known_counts --test sd13_sorcerer_spells_per_day_counts --test sd13_sorcerer_total_spells_per_day --test sd18_barbarian_level11_greater_rage --test sd18_monk_level11_diamond_body --test sd18_paladin_level11_aura_of_justice --test sd18_ranger_level11_quarry --test sd18_ranger_level16_improved_evasion --test sd18_ranger_level17_hide_in_plain_sight --test sd18_rogue_level11_sneak_attack --test v06_druid_level16_to_20_widening -- multiclass_ --test-threads=8

| Run | Of the 187 | All `multiclass_` matches (215) | Exit |
|---|---|---|---|
| sabotaged | **14 red**, 173 green | 14 failed, 201 passed | 101 |
| restored (`git checkout -- src/rules_core/pilot_compute/multiclass_fold.rs`) | **0 red**, 187 green | 0 failed, 215 passed | 0 |

The 14 red are exactly the 14 Monk mixes whose class-alone set carries
`class_feature.monk.bounded_progression.bonus_feat.unsupported` (set-diff of the failing names
against `f3d-probe-alone-vs-mix.tsv` rows with a `multiclass.monk.` blocker: identical):

- `multiclass_monk_does_not_gain_slot_2_recognition`
- `multiclass_monk_does_not_gain_slot_3_or_4_recognition`
- `multiclass_monk_is_not_promoted_by_this_slice`
- `multiclass_monk_level10_is_not_promoted_by_this_slice`
- `multiclass_monk_level11_is_not_promoted_by_this_slice`
- `multiclass_monk_level12_is_not_promoted_by_this_slice`
- `multiclass_monk_level2_is_not_promoted_by_this_slice`
- `multiclass_monk_level3_is_not_promoted_by_this_slice`
- `multiclass_monk_level4_is_not_promoted_by_this_slice`
- `multiclass_monk_level5_is_not_promoted_by_this_slice`
- `multiclass_monk_level6_is_not_promoted_by_this_slice`
- `multiclass_monk_level7_is_not_promoted_by_this_slice`
- `multiclass_monk_level8_is_not_promoted_by_this_slice`
- `multiclass_monk_level9_is_not_promoted_by_this_slice`

Every one failed on the set clause, status unchanged (still Blocked on the pillars), e.g.:

    multiclass Monk: the mix's claim-blocking set (multiclass.<class>. re-scope stripped) must equal the class alone's
      left: {"combat.baseline_unsupported", "skill.selected_modifier.unsupported"}
     right: {"class_feature.monk.bounded_progression.bonus_feat.unsupported", "combat.baseline_unsupported", "skill.selected_modifier.unsupported"}

## The 173 this sabotage cannot turn red, by mechanism

Their class-alone blocker set (column 4 of `f3d-probe-alone-vs-mix.tsv`) holds no line the
carry-over is responsible for:

| Mixes | Class-alone blockers | Why the carry-over is not what keeps parity |
|---|---|---|
| 129 | `combat.baseline_unsupported`, `skill.selected_modifier.unsupported` | character-level pillars: the fold deliberately does not carry them; the mix raises them itself (the GE-06 posture gates run on the mix) |
| 25 | + `class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported` | the mix raises the same id itself, unprefixed; the carry-over skips a line the mix already raised |
| 19 | + `class_feature.cleric.healing_domain.rebuke_death.unsupported` | same: raised by the mix under its own id |
| **14** | + `class_feature.monk.bounded_progression.bonus_feat.unsupported` | only the carry-over supplies it (`multiclass.monk.`-scoped) -> red under sabotage |

129 + 25 + 19 + 14 = 187. So "sabotage parity" for this population is 14 red of 187 under the
carry-over sabotage, 0 red restored; the other 173 are guarded by the status/set assertion
against any change to the pillars or the mix's own class-feature checks, not by the carry-over.

Raw logs were scratch (tmpfs); the counts above are from them: sabotaged run `test result` sum
passed 201 failed 14 (EXIT=101); restored passed 215 failed 0 (EXIT=0).
