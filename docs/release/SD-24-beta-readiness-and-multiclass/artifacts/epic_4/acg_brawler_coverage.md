# ACG Brawler — Per-Class Coverage (SD-24 Epic 4, criterion 4.3)

| Field | Value |
|---|---|
| `class_name` | Brawler |
| `book` | ACG |
| `feature_table_path` | `rules_core::rules_tables::acg::class_brawler` (chassis); no named-feature module exists yet |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_class_guide/acg_classes.lst` (chassis) + `acg_abilities_class.lst` (named features) |
| `class_features_expected` | 14 (distinct `KEY:Brawler ~ ...` records in `acg_abilities_class.lst`) |
| `class_features_wired` | 0 |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | No — `compute_class_chassis` in `pilot_compute.rs` only recognizes Fighter/Wizard |
| `level_up_wired` | No — no `level_up::brawler` module exists |
| `gap_priority` | P1 |

## Gap features (from `acg_abilities_class.lst`, corpus commit above)

AC Bonus, Brawler's Cunning, Brawler's Flurry, Brawler Weapon Training, Class Skills, Close Weapon Mastery, Improved Unarmed Strike, Knockout, Martial Flexibility, Maneuver Training, Unarmed Strike progression.

## Verification

- Chassis math cross-checked against `acg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens (see `class_brawler.rs`'s own doc comment for the exact transcribed tokens) — matches, no defect.
- `tests/sd24_acg_class_coverage_audit.rs` exercises this row via `rules_tables::acg::class_coverage(AcgClassId::Brawler)` and the live `compute_pilot_base_chassis` seam.

