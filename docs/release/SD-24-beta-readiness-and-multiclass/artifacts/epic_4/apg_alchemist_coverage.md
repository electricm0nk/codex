# APG Alchemist — Per-Class Coverage (SD-24 Epic 4, criterion 4.2)

| Field | Value |
|---|---|
| `class_name` | Alchemist |
| `book` | APG |
| `feature_table_path` | `rules_core::rules_tables::apg::class_alchemist` (chassis); no named-feature module exists yet |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_players_guide/apg_classes.lst` (chassis) + `apg_abilities_class.lst` (named features) |
| `class_features_expected` | 24 (distinct `KEY:Alchemist ~ ...` records in `apg_abilities_class.lst`) |
| `class_features_wired` | 0 |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | No — `compute_class_chassis` in `pilot_compute.rs` only recognizes Fighter/Wizard |
| `level_up_wired` | No — no `level_up::alchemist` module exists |
| `gap_priority` | P1 |

## Gap features (from `apg_abilities_class.lst`, corpus commit above)

Alchemy, Bomb, Brew Potion, Class Skills, Discovery, Formulae, Grand Discovery, Instant Alchemy, Mutagen (+ its three stat-selection sub-choices), Persistent Mutagen, Poison Resistance, Poison Use, Swift Alchemy, Swift Poisoning, Throw Anything.

Note: individual Discoveries (e.g. Precise Bombs, Explosive Bombs, ~20 more) and individual Grand Discoveries are a separate selectable-list layer, not counted in the 24 above — they live under `apg_abilities.lst`'s own ability categories, not `apg_abilities_class.lst`'s `KEY:Alchemist ~ ...` records. A future ingest cycle scoping the Discovery/Grand-Discovery chooser lists would need to audit that file too.

## Verification

- Chassis math (three-quarter BAB; good Fortitude/Reflex; poor Will) cross-checked against `apg_classes.lst:11`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens — matches, no defect.
- `tests/sd24_apg_class_coverage_audit.rs` exercises this row via `rules_tables::apg::class_coverage(ApgClassId::Alchemist)` and the live `compute_pilot_base_chassis` seam.
