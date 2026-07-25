# APG Inquisitor — Per-Class Coverage (SD-24 Epic 4, criterion 4.2)

| Field | Value |
|---|---|
| `class_name` | Inquisitor |
| `book` | APG |
| `feature_table_path` | `rules_core::rules_tables::apg::class_inquisitor` (chassis); no named-feature module exists yet |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_players_guide/apg_classes.lst` (chassis) + `apg_abilities_class.lst` (named features) |
| `class_features_expected` | 19 (distinct `KEY:Inquisitor ~ ...` records in `apg_abilities_class.lst`) |
| `class_features_wired` | 1 (Justice judgment — see update below) |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | Yes — `compute_class_chassis`/`compute_apg_class_chassis` in `pilot_compute.rs` recognize single-class Inquisitor (v0.6 alpha swarm) |
| `level_up_wired` | No — no `level_up::inquisitor` module exists |
| `gap_priority` | P1 |

## Gap features (from `apg_abilities_class.lst`, corpus commit above)

Bane, Class Skills, Cunning Initiative, Detect Alignment, Discern Lies, Exploit Weakness, Greater Bane, Monster Lore, Orisons, Profane Judgment, Sacred Judgment (~~Judgment/Justice wired~~ — see update below), Second Judgment, Slayer, Solo Tactics, Stalwart, Stern Gaze, Third Judgment, Track, True Judgment.

Note: the individual Judgments available to select (Destruction, Healing, Justice, Piercing, Protection, Purity, Resiliency, Resistance, Sacred/Fell Judgment "Smiting", Sight, Smiting) are a separate selectable-list layer, not counted in the 19 above. Justice is the one judgment type this repo's Judgment closure grounds (see update below); the other 7 remain unbuilt.

## Verification

- Chassis math (three-quarter BAB; good Fortitude/Will; poor Reflex) cross-checked against `apg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens for `CLASS:Inquisitor` — matches, no defect.
- `tests/sd24_apg_class_coverage_audit.rs` exercises this row via `rules_tables::apg::class_coverage(ApgClassId::Inquisitor)` and the live `compute_pilot_base_chassis` seam.

## Update (v0.6 alpha swarm, risks item 8, Inquisitor Judgment closure, third APG class-specific closure, 2026-07-25)

Inquisitor's own chassis-integration gate (`is_supported_inquisitor_single_class` in `pilot_compute.rs`) and its Justice judgment are now genuinely wired -- the third widening of `has_supported_class_chassis` to an APG class. Judgment combines the activation-gating pattern (Barbarian/Skald/Bloodrager Rage-shaped mechanics) with the choice-recognition pattern (Cleric's domain choice / Alchemist's mutagen-stat choice), the same combination Alchemist's Mutagen already proved -- not a new architecture. A new `choice:inquisitor_judgment` choice set (currently recognizing one selection, `judgment:justice`) combined with activation-gating (`class_ability_activations` entry `"judgment"`) grounds a real +1+floor(level/5) sacred-or-profane bonus to the integrated baseline melee attack bonus when actively, validly judging with Justice recognized. Justice was picked as the one canonical judgment type this closure grounds because it is the only one of the 8 whose bonus is a pure numeric attack-roll bonus requiring no new engine state (Destruction needs a damage-roll total this codebase doesn't compute anywhere; Healing/Resiliency/Resistance/Protection need fast-healing/damage-reduction/energy-resistance state that doesn't exist yet; Piercing/Purity/Smiting are opponent- or effect-type-dependent). The numeric bonus is identical whether the inquisitor's own alignment grants the Sacred or Profane variant, so no alignment branching was needed for the value itself. An active judgment with no recognized Justice choice (or a request naming a different, unbuilt judgment type) is a genuine posture violation and claim-blocks, mirroring Alchemist's own "active but no recognized stat choice" shape. Inquisitor's Domain was confirmed, directly against the corpus's own `KEY:Inquisitor ~ ...` list, to grant no separate domain power (only spell-list access) -- unlike Cleric's domain, so it needed no dedicated domain-power closure and is folded into the deferred spellcasting bucket instead. Inquisitor still does not reach `Computed` -- spellcasting, the other 7 judgment types, and every other named feature remain claim-blocked via the new `class_feature.apg.inquisitor.other_features_deferred.unsupported` diagnostic, which replaced the generic `class_feature.apg.inquisitor.unsupported` diagnostic for Inquisitor specifically. See `pilot_compute.rs`'s `inquisitor_dispatch_widening_safety_tests` module for the full test coverage.
