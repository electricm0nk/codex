# APG Oracle — Per-Class Coverage (SD-24 Epic 4, criterion 4.2)

| Field | Value |
|---|---|
| `class_name` | Oracle |
| `book` | APG |
| `feature_table_path` | `rules_core::rules_tables::apg::class_oracle` (chassis); no named-feature module exists yet |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_players_guide/apg_classes.lst` (chassis) + `apg_abilities_class.lst` (named features) |
| `class_features_expected` | 19 (distinct `KEY:Oracle ~ ...` records in `apg_abilities_class.lst`) |
| `class_features_wired` | 0 |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | No — `compute_class_chassis` in `pilot_compute.rs` only recognizes Fighter/Wizard |
| `level_up_wired` | No — no `level_up::oracle` module exists |
| `gap_priority` | P1 |

## Gap features (from `apg_abilities_class.lst`, corpus commit above)

Battle Mystery, Bone Mystery, Clouded Vision, Cure Wounds, Deaf, Flame Mystery, Haunted, Heavens Mystery, Inflict Wounds, Lame, Life Mystery, Lore Mystery, Nature Mystery, Orisons, Stone Mystery, Tongues, Wasting, Waves Mystery, Winds Mystery.

Note: this list already names the seven Mysteries plus the Oracle's Curses (Clouded Vision, Deaf, Haunted, Lame, Tongues, Wasting) — but each Mystery's own per-level Revelation choices (~8 per Mystery) are a separate selectable-list layer, not counted in the 19 above.

## Verification

- Chassis math (three-quarter BAB; good Will; poor Fortitude/Reflex) cross-checked against `apg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens for `CLASS:Oracle` — matches, no defect.
- `tests/sd24_apg_class_coverage_audit.rs` exercises this row via `rules_tables::apg::class_coverage(ApgClassId::Oracle)` and the live `compute_pilot_base_chassis` seam.
