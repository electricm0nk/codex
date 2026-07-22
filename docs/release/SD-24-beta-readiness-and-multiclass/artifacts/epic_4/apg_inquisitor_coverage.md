# APG Inquisitor — Per-Class Coverage (SD-24 Epic 4, criterion 4.2)

| Field | Value |
|---|---|
| `class_name` | Inquisitor |
| `book` | APG |
| `feature_table_path` | `rules_core::rules_tables::apg::class_inquisitor` (chassis); no named-feature module exists yet |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_players_guide/apg_classes.lst` (chassis) + `apg_abilities_class.lst` (named features) |
| `class_features_expected` | 19 (distinct `KEY:Inquisitor ~ ...` records in `apg_abilities_class.lst`) |
| `class_features_wired` | 0 |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | No — `compute_class_chassis` in `pilot_compute.rs` only recognizes Fighter/Wizard |
| `level_up_wired` | No — no `level_up::inquisitor` module exists |
| `gap_priority` | P1 |

## Gap features (from `apg_abilities_class.lst`, corpus commit above)

Bane, Class Skills, Cunning Initiative, Detect Alignment, Discern Lies, Exploit Weakness, Greater Bane, Monster Lore, Orisons, Profane Judgment, Sacred Judgment, Second Judgment, Slayer, Solo Tactics, Stalwart, Stern Gaze, Third Judgment, Track, True Judgment.

Note: the individual Judgments available to select (Destruction, Healing, Justice, Piercing, Protection, Purity, Resiliency, Resistance, Sacred/Fell Judgment "Smiting", Sight, Smiting) are a separate selectable-list layer, not counted in the 19 above.

## Verification

- Chassis math (three-quarter BAB; good Fortitude/Will; poor Reflex) cross-checked against `apg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens for `CLASS:Inquisitor` — matches, no defect.
- `tests/sd24_apg_class_coverage_audit.rs` exercises this row via `rules_tables::apg::class_coverage(ApgClassId::Inquisitor)` and the live `compute_pilot_base_chassis` seam.
