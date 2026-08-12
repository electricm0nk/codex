# ACG Bloodrager — Per-Class Coverage (SD-24 Epic 4, criterion 4.3)

| Field | Value |
|---|---|
| `class_name` | Bloodrager |
| `book` | ACG |
| `feature_table_path` | `rules_core::rules_tables::acg::class_bloodrager` (chassis); no named-feature module exists yet |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_class_guide/acg_classes.lst` (chassis) + `acg_abilities_class.lst` (named features) |
| `class_features_expected` | 19 (distinct `KEY:Bloodrager ~ ...` records in `acg_abilities_class.lst`) |
| `class_features_wired` | 1 (Bloodrage — see update below) |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | Yes — `compute_class_chassis`/`compute_acg_class_chassis` in `pilot_compute.rs` recognize single-class Bloodrager (v0.6 alpha swarm) |
| `level_up_wired` | No — no `level_up::bloodrager` module exists |
| `gap_priority` | P1 |

## Gap features (from `acg_abilities_class.lst`, corpus commit above)

Blood Sanctuary, Blood Casting, Bloodline (+ its per-bloodline power selections), ~~Bloodrage~~ (wired below), Bloodrager Bonus Feats, Class Skills, Damage Reduction, Fast Movement, Indomitable Will, Improved Uncanny Dodge, Uncanny Dodge, Tireless Rage-analog (Tireless Bloodrage, 17th level).

## Verification

- Chassis math cross-checked against `acg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens (see `class_bloodrager.rs`'s own doc comment for the exact transcribed tokens) — matches, no defect.
- `tests/sd24_acg_class_coverage_audit.rs` exercises this row via `rules_tables::acg::class_coverage(AcgClassId::Bloodrager)` and the live `compute_pilot_base_chassis` seam.

## Update (v0.6 alpha swarm, risks item 8, second APG/ACG class-specific closure, 2026-07-25)

Bloodrager's own chassis-integration gate (`is_supported_bloodrager_single_class` in `pilot_compute.rs`) and Bloodrage are now genuinely wired: STR/CON/Will morale bonuses and the AC penalty apply to `compute_total_saves`/`compute_combat_baseline`/ability modifiers when a valid, active, in-budget activation is present, verified against the PCGen corpus DESC text and `BONUS:VAR` formulas -- the corpus text itself states Bloodrage "counts as the barbarian's rage class feature for the purpose of feat prerequisites, feat abilities, magic item abilities, and spell effects," and the Greater/Tireless/Mighty Bloodrage thresholds (11th/17th/20th level) and magnitudes were independently verified identical to Barbarian's own Rage tiers. Bloodrager still cannot reach `Computed` -- spellcasting (Bloodrager casts from its own `SPELLLIST:1|Bloodrager`, not yet independently verified) and every other named feature above remain claim-blocked via the new `class_feature.acg.bloodrager.spellcasting_deferred.unsupported` diagnostic, which replaced the old generic `class_feature.acg.bloodrager.unsupported` diagnostic for Bloodrager specifically. See `pilot_compute.rs`'s `bloodrager_dispatch_widening_safety_tests` module and `docs/release/v0.6/bloodrager-acg-second-class-scoping.md` for the full scoping record.

