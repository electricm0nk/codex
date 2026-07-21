# APG Witch — Per-Class Coverage (SD-24 Epic 4, criterion 4.2)

| Field | Value |
|---|---|
| `class_name` | Witch |
| `book` | APG |
| `feature_table_path` | `rules_core::rules_tables::apg::class_witch` (chassis); no named-feature module exists yet |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_players_guide/apg_classes.lst` (chassis) + `apg_abilities_class.lst` (named features) |
| `class_features_expected` | 7 (distinct `KEY:Witch ~ ...` records in `apg_abilities_class.lst`) |
| `class_features_wired` | 0 |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | No — `compute_class_chassis` in `pilot_compute.rs` only recognizes Fighter/Wizard |
| `level_up_wired` | No — no `level_up::witch` module exists |
| `gap_priority` | P1 |

## Gap features (from `apg_abilities_class.lst`, corpus commit above)

Cantrips, Class Skills, Familiar, Familiar Touch Spells, Hex, Patron Spells, Weapon Proficiencies.

Note: the ~20 individual Hexes (Cackle, Evil Eye, Slumber, Ward, ...) and Major/Grand Hexes a Witch selects from are a separate selectable-list layer under a different `CATEGORY:Special Ability` chooser in `apg_abilities.lst`, not counted in the 7 above — this is the largest gap between "named feature slots wired" and "actual playable feature surface" of the six APG classes, since almost all of Witch's build-defining choices live in that unaudited chooser list.

## Verification

- Chassis math (half BAB; good Will; poor Fortitude/Reflex) cross-checked against `apg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens for `CLASS:Witch` — matches, no defect.
- `tests/sd24_apg_class_coverage_audit.rs` exercises this row via `rules_tables::apg::class_coverage(ApgClassId::Witch)` and the live `compute_pilot_base_chassis` seam.
