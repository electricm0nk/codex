# ACG Brawler — Per-Class Coverage (SD-24 Epic 4, criterion 4.3)

| Field | Value |
|---|---|
| `class_name` | Brawler |
| `book` | ACG |
| `feature_table_path` | `rules_core::rules_tables::acg::class_brawler` (chassis); no named-feature module exists yet |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_class_guide/acg_classes.lst` (chassis) + `acg_abilities_class.lst` (named features) |
| `class_features_expected` | 14 (distinct `KEY:Brawler ~ ...` records in `acg_abilities_class.lst`) |
| `class_features_wired` | 1 (AC Bonus — see update below) |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | Yes — `compute_class_chassis`/`compute_acg_class_chassis` in `pilot_compute.rs` recognize single-class Brawler (v0.6 alpha swarm) |
| `level_up_wired` | No — no `level_up::brawler` module exists |
| `gap_priority` | P1 |

## Gap features (from `acg_abilities_class.lst`, corpus commit above)

~~AC Bonus~~ (wired below), Brawler's Cunning, Brawler's Flurry, Brawler Weapon Training, Class Skills, Close Weapon Mastery, Improved Unarmed Strike, Knockout, Martial Flexibility, Maneuver Training, Unarmed Strike progression.

## Verification

- Chassis math cross-checked against `acg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens (see `class_brawler.rs`'s own doc comment for the exact transcribed tokens) — matches, no defect.
- `tests/sd24_acg_class_coverage_audit.rs` exercises this row via `rules_tables::acg::class_coverage(AcgClassId::Brawler)` and the live `compute_pilot_base_chassis` seam.

## Update (v0.6 alpha swarm, risks item 8, third APG/ACG class-specific closure, 2026-07-25)

Brawler's own chassis-integration gate (`is_supported_brawler_single_class` in `pilot_compute.rs`) and AC Bonus are now genuinely wired: a level-driven dodge bonus to Armor Class (`(level>3)+(level>8)+(level>12)+(level>17)`, verified against the corpus `BONUS:VAR` formula), integrated into the shared `compute_combat_baseline` Armor Class total. Unlike Skald/Bloodrager's Rage-shaped mechanics, AC Bonus needs no `class_ability_activations` entry at all — it is a pure function of level and class ownership, since the "not wearing Medium/Heavy armor" precondition is provably vacuous in this codebase (no Medium/Heavy armor item id exists anywhere; the only armor this codebase can express, Chain Shirt, is itself light armor and is required `EquippedActive` unconditionally by the shared combat-baseline posture). Brawler is a pure martial class (no `SPELLSTAT`) — its remaining named-feature bucket (Brawler's Flurry, Knockout, Martial Flexibility, Awesome Blow, Improved Awesome Blow, Brawler's Cunning, Martial Training, Bonus Feats, Close Weapon Mastery, Brawler's Strike, Maneuver Training) stays claim-blocked via the new `class_feature.acg.brawler.other_features_deferred.unsupported` diagnostic (not `spellcasting_deferred`, since Brawler casts nothing), replacing the old generic `class_feature.acg.brawler.unsupported` diagnostic for Brawler specifically. See `pilot_compute.rs`'s `brawler_ac_bonus_progression_matches_the_corpus_formula_at_higher_levels` test and `docs/release/v0.6/brawler-acg-third-class-scoping.md` for the full scoping record.

