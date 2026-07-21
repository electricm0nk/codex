# APG Cavalier — Per-Class Coverage (SD-24 Epic 4, criterion 4.2)

| Field | Value |
|---|---|
| `class_name` | Cavalier |
| `book` | APG |
| `feature_table_path` | `rules_core::rules_tables::apg::class_cavalier` (chassis); no named-feature module exists yet |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_players_guide/apg_classes.lst` (chassis) + `apg_abilities_class.lst` (named features) |
| `class_features_expected` | 16 (distinct `KEY:Cavalier ~ ...` records in `apg_abilities_class.lst`) |
| `class_features_wired` | 0 |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | No — `compute_class_chassis` in `pilot_compute.rs` only recognizes Fighter/Wizard |
| `level_up_wired` | No — no `level_up::cavalier` module exists |
| `gap_priority` | P1 |

## Gap features (from `apg_abilities_class.lst`, corpus commit above)

Banner, Bonus Feat, Cavalier's Charge, Challenge, Class Skills, Demanding Challenge, Expert Trainer, Greater Banner, Greater Tactician, Master Tactician, Mighty Charge, Mount, Order, Supreme Charge, Tactician, Tactician Feat.

Note: the specific Cavalier's Order a player selects (each with its own Edicts/Challenge bonus/skills/Order-granted-abilities) is a separate selectable-list layer under a different ability category, not counted in the 16 above.

## Verification

- Chassis math (full BAB; good Fortitude; poor Reflex/Will) cross-checked against `apg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens for `CLASS:Cavalier` — matches, no defect.
- `tests/sd24_apg_class_coverage_audit.rs` exercises this row via `rules_tables::apg::class_coverage(ApgClassId::Cavalier)` and the live `compute_pilot_base_chassis` seam.
