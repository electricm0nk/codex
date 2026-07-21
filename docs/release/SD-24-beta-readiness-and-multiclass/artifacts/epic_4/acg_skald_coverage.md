# ACG Skald — Per-Class Coverage (SD-24 Epic 4, criterion 4.3)

| Field | Value |
|---|---|
| `class_name` | Skald |
| `book` | ACG |
| `feature_table_path` | `rules_core::rules_tables::acg::class_skald` (chassis); no named-feature module exists yet |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_class_guide/acg_classes.lst` (chassis) + `acg_abilities_class.lst` (named features) |
| `class_features_expected` | 20 (distinct `KEY:Skald ~ ...` records in `acg_abilities_class.lst`) |
| `class_features_wired` | 0 |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | No — `compute_class_chassis` in `pilot_compute.rs` only recognizes Fighter/Wizard |
| `level_up_wired` | No — no `level_up::skald` module exists |
| `gap_priority` | P1 |

## Gap features (from `acg_abilities_class.lst`, corpus commit above)

Bardic Knowledge-analog, Class Skills, Inspired Rage, Iron Will, Class-skill spellcasting recognition, Raging Song, Rage Powers (shared list access), Spell Kenning, Versatile Performance, War Chant.

## Verification

- Chassis math cross-checked against `acg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens (see `class_skald.rs`'s own doc comment for the exact transcribed tokens) — matches, no defect.
- `tests/sd24_acg_class_coverage_audit.rs` exercises this row via `rules_tables::acg::class_coverage(AcgClassId::Skald)` and the live `compute_pilot_base_chassis` seam.

