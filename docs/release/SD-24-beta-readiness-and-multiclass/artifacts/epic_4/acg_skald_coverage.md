# ACG Skald — Per-Class Coverage (SD-24 Epic 4, criterion 4.3)

| Field | Value |
|---|---|
| `class_name` | Skald |
| `book` | ACG |
| `feature_table_path` | `rules_core::rules_tables::acg::class_skald` (chassis); no named-feature module exists yet |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_class_guide/acg_classes.lst` (chassis) + `acg_abilities_class.lst` (named features) |
| `class_features_expected` | 20 (distinct `KEY:Skald ~ ...` records in `acg_abilities_class.lst`) |
| `class_features_wired` | 1 (Inspired Rage — see update below) |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | Yes — `compute_class_chassis`/`compute_acg_class_chassis` in `pilot_compute.rs` recognize single-class Skald (v0.6 alpha swarm) |
| `level_up_wired` | No — no `level_up::skald` module exists |
| `gap_priority` | P1 |

## Gap features (from `acg_abilities_class.lst`, corpus commit above)

Bardic Knowledge-analog, Class Skills, ~~Inspired Rage~~ (wired below), Iron Will, Class-skill spellcasting recognition, Raging Song (base rounds-per-day only, wired alongside Inspired Rage), Rage Powers (shared list access), Spell Kenning, Versatile Performance, War Chant.

## Verification

- Chassis math cross-checked against `acg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens (see `class_skald.rs`'s own doc comment for the exact transcribed tokens) — matches, no defect.
- `tests/sd24_acg_class_coverage_audit.rs` exercises this row via `rules_tables::acg::class_coverage(AcgClassId::Skald)` and the live `compute_pilot_base_chassis` seam.

## Update (v0.6 alpha swarm, risks item 8, first APG/ACG class-specific closure, 2026-07-25)

Skald's own chassis-integration gate (`is_supported_skald_single_class` in `pilot_compute.rs`) and 1st-level Raging Song song type, Inspired Rage, are now genuinely wired: STR/CON/Will morale bonuses and the AC penalty apply to `compute_total_saves`/`compute_combat_baseline`/ability modifiers when a valid, active, in-budget activation is present, verified against the PCGen corpus DESC text and `BONUS:VAR` formulas. Skald still cannot reach `Computed` -- spellcasting (Skald casts from the Bard spell list, but its own spells-known/per-day numbers are not yet independently verified) and every other named feature above remain claim-blocked via the new `class_feature.acg.skald.spellcasting_deferred.unsupported` diagnostic, which replaced the old generic `class_feature.acg.skald.unsupported` diagnostic for Skald specifically (retired because its "no named class-feature computation... grounded anywhere" claim became false). See `pilot_compute.rs`'s `skald_dispatch_widening_safety_tests` module and `docs/release/v0.6/skald-acg-first-class-scoping.md` for the full scoping and adversarial-review record.

