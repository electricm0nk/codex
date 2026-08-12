# APG Cavalier — Per-Class Coverage (SD-24 Epic 4, criterion 4.2)

| Field | Value |
|---|---|
| `class_name` | Cavalier |
| `book` | APG |
| `feature_table_path` | `rules_core::rules_tables::apg::class_cavalier` (chassis); no named-feature module exists yet |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_players_guide/apg_classes.lst` (chassis) + `apg_abilities_class.lst` (named features) |
| `class_features_expected` | 16 (distinct `KEY:Cavalier ~ ...` records in `apg_abilities_class.lst`) |
| `class_features_wired` | 1 (Mount — see update below) |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | Yes — `compute_class_chassis`/`compute_apg_class_chassis` in `pilot_compute.rs` recognize single-class Cavalier (v0.6 alpha swarm) |
| `level_up_wired` | No — no `level_up::cavalier` module exists |
| `gap_priority` | P1 |

## Gap features (from `apg_abilities_class.lst`, corpus commit above)

Banner, Bonus Feat, Cavalier's Charge, Challenge, Class Skills, Demanding Challenge, Expert Trainer, Greater Banner, Greater Tactician, Master Tactician, Mighty Charge, ~~Mount~~ (wired below), Order, Supreme Charge, Tactician, Tactician Feat.

Note: the specific Cavalier's Order a player selects (each with its own Edicts/Challenge bonus/skills/Order-granted-abilities) is a separate selectable-list layer under a different ability category, not counted in the 16 above.

## Verification

- Chassis math (full BAB; good Fortitude; poor Reflex/Will) cross-checked against `apg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens for `CLASS:Cavalier` — matches, no defect.
- `tests/sd24_apg_class_coverage_audit.rs` exercises this row via `rules_tables::apg::class_coverage(ApgClassId::Cavalier)` and the live `compute_pilot_base_chassis` seam.

## Update (v0.6 alpha swarm, risks item 8, Cavalier Mount closure, first APG class-specific closure, 2026-07-25)

Cavalier's own chassis-integration gate (`is_supported_cavalier_single_class` in `pilot_compute.rs`) and 1st-level Mount are now genuinely wired — the first widening of `has_supported_class_chassis` to any APG class. The corpus confirms the Mount is mechanically identical to Druid's/Hunter's own animal companion ("This mount functions as a druid's animal companion, using the cavalier's level as his effective druid level"), so the closure reuses the shared "Animal Companion Base Statistics" progression math via new, parallel Horse-specific helpers (`ground_horse_companion_stat_block`, `ground_horse_companion_link_vacuous`) — kept deliberately separate from Druid's/Hunter's own Wolf-specific functions rather than genericized, so this closure carries zero risk to their already-shipped output (verified: their own test suites pass unchanged). Horse's stat block (Str 16, Dex 13, Con 15, +4 natural armor) was independently verified against aonprd.com and d20pfsrd, with two real disagreements (natural armor, speed) resolved in aonprd's favor via the PCGen corpus as tiebreaker, the same methodology as Wolf's own natural-armor/Trip resolution. Unlike Druid's own choice-gated Nature Bond, the Mount is unconditional on class ownership and level alone — every Cavalier gets one automatically, mirroring Hunter's own Animal Companion shape. Per the PF1 Core Rulebook's own Mount description, the Mount does NOT gain the Share Spells special ability at all (unlike Druid's/Hunter's own companions) — no vacuous-correction record is grounded for an ability the Mount never has in the first place. Cavalier still does not reach `Computed` — its other named features (Challenge, Order, Tactician, Cavalier's Charge, and the rest) remain claim-blocked via the new `class_feature.apg.cavalier.other_features_deferred.unsupported` diagnostic, which replaced the old generic `class_feature.apg.cavalier.unsupported` diagnostic for Cavalier specifically. See `pilot_compute.rs`'s `cavalier_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one` test and `docs/release/v0.6/cavalier-mount-apg-first-class-scoping.md` for the full scoping and verification record.
