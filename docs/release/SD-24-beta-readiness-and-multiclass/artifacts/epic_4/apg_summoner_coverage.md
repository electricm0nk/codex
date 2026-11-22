# APG Summoner — Per-Class Coverage (SD-24 Epic 4, criterion 4.2)

| Field | Value |
|---|---|
| `class_name` | Summoner |
| `book` | APG |
| `feature_table_path` | `rules_core::rules_tables::apg::class_summoner` (chassis); no named-feature module exists yet |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_players_guide/apg_classes.lst` (chassis) + `apg_abilities_class.lst` (named features) |
| `class_features_expected` | 17 (distinct `KEY:Summoner ~ ...` records in `apg_abilities_class.lst`) |
| `class_features_wired` | 0 |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | No — `compute_class_chassis` in `pilot_compute.rs` only recognizes Fighter/Wizard |
| `level_up_wired` | No — no `level_up::summoner` module exists |
| `gap_priority` | P1 |

## Gap features (from `apg_abilities_class.lst`, corpus commit above)

Aspect, Bond Senses, Cantrips, Eidolon, Gate, Greater Aspect, Greater Shield Ally, Life Bond, Life Link, Maker's Call, Merge Forms, Shield Ally, Standard Class, Summon Monster, Transposition, Twin Eidolon, Weapon and Armor Proficiency.

Note: the Eidolon's own evolution-point-driven build (base forms, evolutions) is a separate, much larger content layer (`apg_companionmods.lst` / `apg_kits_companion.lst`), not counted in the 17 above and not audited by this cycle.

## Verification

- Chassis math (three-quarter BAB; good Will; poor Fortitude/Reflex) cross-checked against `apg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens for `CLASS:Summoner` — matches, no defect.
- `tests/sd24_apg_class_coverage_audit.rs` exercises this row via `rules_tables::apg::class_coverage(ApgClassId::Summoner)` and the live `compute_pilot_base_chassis` seam.
