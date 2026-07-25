# APG Alchemist — Per-Class Coverage (SD-24 Epic 4, criterion 4.2)

| Field | Value |
|---|---|
| `class_name` | Alchemist |
| `book` | APG |
| `feature_table_path` | `rules_core::rules_tables::apg::class_alchemist` (chassis); no named-feature module exists yet |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_players_guide/apg_classes.lst` (chassis) + `apg_abilities_class.lst` (named features) |
| `class_features_expected` | 24 (distinct `KEY:Alchemist ~ ...` records in `apg_abilities_class.lst`) |
| `class_features_wired` | 1 (Mutagen — see update below) |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | Yes — `compute_class_chassis`/`compute_apg_class_chassis` in `pilot_compute.rs` recognize single-class Alchemist (v0.6 alpha swarm) |
| `level_up_wired` | No — no `level_up::alchemist` module exists |
| `gap_priority` | P1 |

## Gap features (from `apg_abilities_class.lst`, corpus commit above)

Alchemy, Bomb, Brew Potion, Class Skills, Discovery, Formulae, Grand Discovery, Instant Alchemy, ~~Mutagen~~ (wired below; its three stat-selection sub-choices are recognized as part of this wiring), Persistent Mutagen, Poison Resistance, Poison Use, Swift Alchemy, Swift Poisoning, Throw Anything.

Note: individual Discoveries (e.g. Precise Bombs, Explosive Bombs, ~20 more) and individual Grand Discoveries are a separate selectable-list layer, not counted in the 24 above — they live under `apg_abilities.lst`'s own ability categories, not `apg_abilities_class.lst`'s `KEY:Alchemist ~ ...` records. A future ingest cycle scoping the Discovery/Grand-Discovery chooser lists would need to audit that file too.

## Verification

- Chassis math (three-quarter BAB; good Fortitude/Reflex; poor Will) cross-checked against `apg_classes.lst:11`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens — matches, no defect.
- `tests/sd24_apg_class_coverage_audit.rs` exercises this row via `rules_tables::apg::class_coverage(ApgClassId::Alchemist)` and the live `compute_pilot_base_chassis` seam.

## Update (v0.6 alpha swarm, risks item 8, Alchemist Mutagen closure, second APG class-specific closure, 2026-07-25)

Alchemist's own chassis-integration gate (`is_supported_alchemist_single_class` in `pilot_compute.rs`) and Mutagen are now genuinely wired -- the second widening of `has_supported_class_chassis` to an APG class. Unlike every prior Rage-shaped closure (which affects a fixed set of stats), Mutagen genuinely requires a choice recognized via a new `choice:alchemist_mutagen_stat` choice set (Strength/Dexterity/Constitution), combined with activation-gating (`class_ability_activations` entry `"mutagen"`) for the first time this session -- the +4 alchemical bonus to the chosen physical score, the corresponding -2 mental-score penalty (Str->Int, Dex->Wis, Con->Cha, verified verbatim against the corpus DESC), and the +2 natural armor bonus are all genuinely applied to the integrated ability modifiers and Armor Class when a character is actively mutated with a recognized stat choice. An active mutagen with no recognized stat choice is a genuine posture violation and claim-blocks (mirrors Sorcerer's own "recognized bloodline but no bond choice given" shape), confirmed by direct edge-case review not to misclassify any real gameplay path. Confirmed via direct grep of `CharacterSheet.tsx` before scoping: this closure lands in the same headless-only reachability bucket as Sorcerer/Cleric/Druid's own choice-gated mechanics (no generic choice-picker exists in the creation/level-up UI). Alchemist still does not reach `Computed` -- spellcasting (extracts, genuinely cast via `SPELLSTAT:INT`) and every other named feature remain claim-blocked via the new `class_feature.apg.alchemist.spellcasting_deferred.unsupported` diagnostic, which replaced the generic `class_feature.apg.alchemist.unsupported` diagnostic for Alchemist specifically. See `pilot_compute.rs`'s `alchemist_dispatch_widening_safety_tests` module and `docs/release/v0.6/alchemist-mutagen-apg-second-class-scoping.md` for the full scoping record.
