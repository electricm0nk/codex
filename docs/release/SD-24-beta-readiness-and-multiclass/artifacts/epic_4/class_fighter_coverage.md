# Epic 4 — Class Coverage Audit: Fighter (CRB, criterion 4.1)

## DISCOVERED — plan-vs-reality path correction

`content-unit-inventory.md` §3.1 names the wired-feature-count location as
`src/rules_core/rules_tables/<book>/class_<name>.rs` (e.g. a hypothetical
`class_fighter.rs`). No such file exists. The real per-class wiring for
Fighter is split across two already-landed modules:

- `src/rules_core/rules_tables/crb/class_tables.rs` — the class-generic
  BAB / base-save progression table (`class_tables()`), shared by all 11
  CRB classes.
- `src/rules_core/level_up/fighter.rs` — the class-specific `LevelUpPlan`
  composer (`compute_fighter_level_up_grants`), which reads both
  `class_tables()` and `pilot_compute::compute_pilot_base_chassis`'s
  Fighter-specific explanation records.

This audit is scored against those two real modules, not the stale
planning-doc path. (See also `progress.md` `## DISCOVERED` for the
bundle-level entry.)

## Canonical feature source

PF1 Core Rulebook Fighter class table + class features (Bravery, Armor
Training, Weapon Training, Armor Mastery, Weapon Mastery, Bonus Feats),
cross-checked against `pilot_compute.rs`'s own already-primary-source-cited
grounding (`cr_classes.lst:139` for BAB/saves; `explain_fighter_class_features`
for the named pillars). No PCGen LST corpus file was independently opened
for this audit — the canonical-source claims below are `pilot_compute.rs`'s
own, already-verified-per-cycle citations, read read-only.

## Feature inventory (`class_features_expected`) vs. wiring (`class_features_wired`)

| # | Pillar | `pilot_compute.rs` explanation id | Wired into `LevelUpPlan`? |
|---|---|---|---|
| 1 | Base attack bonus (full BAB) | `class_tables()` column `base_attack_bonus` | Yes — `append_class_table_grants` |
| 2 | Fortitude / Reflex / Will base saves | `class_tables()` columns `fort_save`/`ref_save`/`will_save` | Yes — `append_class_table_grants` |
| 3 | Bravery (Will vs. fear, levels 2/6/10/14/18) | `class_feature.fighter.bravery` | Yes — `append_class_feature_grants` |
| 4 | Bonus Feat slots (levels 1/2/4/6/8/10/12/14/16/18/20) | `class_feature.fighter.level_{2,4,6,8,10,12,14,16,18,20}_bonus_feat` (10 slots; level-1 slot is a character-creation grant, out of Level-Up scope by design) | Yes, for all 10 level-up slots — `append_class_feature_grants` |
| 5 | Armor Training (ranks 1-4, levels 3/7/11/15) | `class_feature.fighter.armor_training` (4 magnitudes) | Yes |
| 6 | Weapon Training (rank 1 + 2nd/3rd/4th chosen group, levels 5/9/13/17) | `class_feature.fighter.weapon_training` + `.weapon_training_group_{2,3,4}` | Yes |
| 7 | Armor Mastery (level 19, DR 5/—) | `class_feature.fighter.armor_mastery` | Yes |
| 8 | Weapon Mastery (level 20 capstone) | `class_feature.fighter.weapon_mastery` | Yes |
| 9 | Level-1 Hit Points / Favored Class Bonus choice | `class_chassis.fighter.level_1_hit_points`, `.favored_class_bonus_choice` | N/A for Level-Up — both are gated to `supported_fighter_level(input) == Some(1)`, a character-creation-only grant; a Level-Up transition's `to_level` is always ≥ 2, so these ids can never appear in a `to_explanations` snapshot this module diffs. Not a gap. |
| 10 | Bonus Feat *candidate list* (which feats are selectable) | n/a — no candidate-catalog cross-check exists | No — `pick_from_lists` stays empty. Documented, bounded scope note (predates this cycle): composing a real Combat-Feats-eligible candidate list against Epic 3's `feat_prereqs` evaluator is a real design surface of its own, out of this audit's remediation scope. |

`class_features_expected` = 8 grounded pillars (rows 3-8, with row 4/5/6
each a multi-magnitude single pillar) + the 2 generic table columns (row
1-2) = **10 wireable pillars**. `class_features_wired` = **10 of 10**
(100%). Row 9 is correctly out of scope (character-creation-only, not a
Level-Up concern). Row 10 (candidate-list enumeration) is a separate,
already-documented boundary, not a "feature not wired" gap — no fabricated
list is preferable to a fabricated one per `AGENTS.md`.

## Audit verdict

**No gap found.** Fighter's `LevelUpPlan` composer
(`src/rules_core/level_up/fighter.rs`) already surfaces every named
class-feature pillar `pilot_compute.rs` grounds for Fighter. No code
change was required for Fighter this cycle.

## Remediation plan (criterion 4.4 input)

None — 0 missing features.

## Gap priority

N/A (no gaps).
