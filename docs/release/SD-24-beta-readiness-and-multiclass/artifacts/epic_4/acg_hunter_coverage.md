# ACG Hunter — Per-Class Coverage (SD-24 Epic 4, criterion 4.3)

| Field | Value |
|---|---|
| `class_name` | Hunter |
| `book` | ACG |
| `feature_table_path` | `rules_core::rules_tables::acg::class_hunter` (chassis); no named-feature module exists yet |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_class_guide/acg_classes.lst` (chassis) + `acg_abilities_class.lst` (named features) |
| `class_features_expected` | 21 (distinct `KEY:Hunter ~ ...` records in `acg_abilities_class.lst`) |
| `class_features_wired` | 1 (Animal Companion — see update below) |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | Yes — `compute_class_chassis`/`compute_acg_class_chassis` in `pilot_compute.rs` recognize single-class Hunter (v0.6 alpha swarm) |
| `level_up_wired` | No — no `level_up::hunter` module exists |
| `gap_priority` | P1 |

## Gap features (from `acg_abilities_class.lst`, corpus commit above)

~~Animal Companion~~ (wired below), Class Skills, Hunter Tactics, Hunter's Trick, Nature's Bond, Precise Strike (shared shape), Second Skin, Skirmisher, Swift Companion, Track, Wild Empathy, Woodland Stride.

## Verification

- Chassis math cross-checked against `acg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens (see `class_hunter.rs`'s own doc comment for the exact transcribed tokens) — matches, no defect.
- `tests/sd24_acg_class_coverage_audit.rs` exercises this row via `rules_tables::acg::class_coverage(AcgClassId::Hunter)` and the live `compute_pilot_base_chassis` seam.
Note: this ACG Hunter class is distinct from the CRB Ranger's already-grounded Hunter's Bond feature (`class_feature.ranger.hunters_bond`, per `level_up/ranger.rs`); the two share the word "Hunter" but not a feature source.

## Update (v0.6 alpha swarm, risks item 8, fourth APG/ACG class-specific closure, 2026-07-25)

Hunter's own chassis-integration gate (`is_supported_hunter_single_class` in `pilot_compute.rs`) and 1st-level Animal Companion are now genuinely wired. The corpus text confirms this is mechanically identical to Druid's own Animal Companion progression ("the hunter's effective druid level is equal to her hunter level"), so the closure reuses the exact Wolf companion stat-block math Druid's own closure already independently verified (Str 13/Con 15/natural armor +2/d8 HD, cross-checked against 3 sources) via new shared helpers (`ground_wolf_companion_stat_block`, `ground_wolf_companion_link_and_share_spells_vacuous`), rather than re-deriving or copy-pasting it -- Druid's own call site was refactored to use the same helpers with byte-identical output, verified by Druid's existing 15-test suite passing unchanged. Unlike Druid's own Nature Bond (a genuine choice between an animal companion and a domain), Hunter's Animal Companion is unconditional on class ownership and level alone -- no `selected_choices` or `class_ability_activations` entry is needed, since every Hunter gets one automatically at 1st level per the corpus text (not gated behind an alternative the way Druid's own bond-type choice is). The species choice the corpus also names ("any of the animals on the druid list") is handled the same way Druid's own was: Wolf is assumed as the canonical species, since this codebase models no species-selection input at all for either class. Hunter still does not reach `Computed` -- spellcasting (a restricted Summon Nature's Ally-only known-spell list from the Druid/Ranger spell lists, not yet independently verified) and every other named feature remain claim-blocked via the new `class_feature.acg.hunter.spellcasting_deferred.unsupported` diagnostic, which replaced the old generic `class_feature.acg.hunter.unsupported` diagnostic for Hunter specifically. See `pilot_compute.rs`'s `hunter_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one` test and `docs/release/v0.6/hunter-acg-fourth-class-scoping.md` for the full scoping record.
