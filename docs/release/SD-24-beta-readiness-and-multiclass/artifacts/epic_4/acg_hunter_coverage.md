# ACG Hunter — Per-Class Coverage (SD-24 Epic 4, criterion 4.3)

| Field | Value |
|---|---|
| `class_name` | Hunter |
| `book` | ACG |
| `feature_table_path` | `rules_core::rules_tables::acg::class_hunter` (chassis); no named-feature module exists yet |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_class_guide/acg_classes.lst` (chassis) + `acg_abilities_class.lst` (named features) |
| `class_features_expected` | 21 (distinct `KEY:Hunter ~ ...` records in `acg_abilities_class.lst`) |
| `class_features_wired` | 3 (Animal Companion + Wild Empathy + Animal Focus (Bull) — see updates below) |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | Yes — `compute_class_chassis`/`compute_acg_class_chassis` in `pilot_compute.rs` recognize single-class Hunter (v0.6 alpha swarm) |
| `level_up_wired` | No — no `level_up::hunter` module exists |
| `gap_priority` | P1 |

## Gap features (from `acg_abilities_class.lst`, corpus commit above)

~~Animal Companion~~ (wired below), ~~Animal Focus~~ (Bull wired below, other 12 options deferred), Class Skills, Hunter Tactics, Hunter's Trick, Nature Training (no numeric BONUS token at all -- correctly deferred, not a missed win), Nature's Bond, Precise Strike (shared shape), Second Skin, Skirmisher, Swift Companion, Track, ~~Wild Empathy~~ (wired below), Woodland Stride.

## Verification

- Chassis math cross-checked against `acg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens (see `class_hunter.rs`'s own doc comment for the exact transcribed tokens) — matches, no defect.
- `tests/sd24_acg_class_coverage_audit.rs` exercises this row via `rules_tables::acg::class_coverage(AcgClassId::Hunter)` and the live `compute_pilot_base_chassis` seam.
Note: this ACG Hunter class is distinct from the CRB Ranger's already-grounded Hunter's Bond feature (`class_feature.ranger.hunters_bond`, per `level_up/ranger.rs`); the two share the word "Hunter" but not a feature source.

## Update (v0.6 alpha swarm, risks item 8, fourth APG/ACG class-specific closure, 2026-07-25)

Hunter's own chassis-integration gate (`is_supported_hunter_single_class` in `pilot_compute.rs`) and 1st-level Animal Companion are now genuinely wired. The corpus text confirms this is mechanically identical to Druid's own Animal Companion progression ("the hunter's effective druid level is equal to her hunter level"), so the closure reuses the exact Wolf companion stat-block math Druid's own closure already independently verified (Str 13/Con 15/natural armor +2/d8 HD, cross-checked against 3 sources) via new shared helpers (`ground_wolf_companion_stat_block`, `ground_wolf_companion_link_and_share_spells_vacuous`), rather than re-deriving or copy-pasting it -- Druid's own call site was refactored to use the same helpers with byte-identical output, verified by Druid's existing 15-test suite passing unchanged. Unlike Druid's own Nature Bond (a genuine choice between an animal companion and a domain), Hunter's Animal Companion is unconditional on class ownership and level alone -- no `selected_choices` or `class_ability_activations` entry is needed, since every Hunter gets one automatically at 1st level per the corpus text (not gated behind an alternative the way Druid's own bond-type choice is). The species choice the corpus also names ("any of the animals on the druid list") is handled the same way Druid's own was: Wolf is assumed as the canonical species, since this codebase models no species-selection input at all for either class. Hunter still does not reach `Computed` -- spellcasting (a restricted Summon Nature's Ally-only known-spell list from the Druid/Ranger spell lists, not yet independently verified) and every other named feature remain claim-blocked via the new `class_feature.acg.hunter.spellcasting_deferred.unsupported` diagnostic, which replaced the old generic `class_feature.acg.hunter.unsupported` diagnostic for Hunter specifically. See `pilot_compute.rs`'s `hunter_stays_blocked_with_the_new_narrower_diagnostic_not_the_retired_one` test and `docs/release/v0.6/hunter-acg-fourth-class-scoping.md` for the full scoping record.

## Second update (v0.6 alpha swarm, risks item 8, Hunter deepening, 2026-07-26, task #2)

Wild Empathy and Animal Focus (Bull) are now also genuinely wired, per `docs/release/v0.6/hunter-remaining-features-scoping.md`'s own corrected-bar analysis. Wild Empathy (`BONUS:VAR|HunterWildEmpatyBonus|CHA+HunterLVL`) is a flat, unconditional check-modifier fact -- no "wild empathy check" total exists anywhere in this codebase, so it grounds as a standalone record, the same corrected shape Inquisitor's Monster Lore/Skald's Bardic Knowledge already established. Animal Focus (`KEY:Hunter ~ Animal Focus`, "usable %1 minutes per day" via a swift action) IS a real activation-gated ability -- unlike the prior scoping pass's own "chooser-list, defer it" call, each of its 13 real options (`KEY:Hunter Animal Focus ~ ...`) is a flat, self-scoped, verifiable magnitude, so it grounds exactly like Oracle's Mystery/Shaman's Spirit canonical-narrowing: Bull (`+2` STR enhancement bonus, rising to `+4` at level 8 and `+6` at level 15) is the one canonical option grounded, mirroring `ground_or_block_inquisitor_judgment`'s three-branch shape with a genuinely enforced per-day minutes budget (`HunterAnimalFocusMinutes = HunterLVL`). The other 12 options (Bat, Bear, Falcon, Frog, Monkey, Mouse, Owl, Snake, Stag, Tiger, plus the "No Ability" sentinel) stay named-but-unbuilt. Nature Training was re-examined under the corrected bar too and confirmed to correctly stay deferred for a different, honest reason: it carries zero numeric `BONUS` token of any kind in the corpus (a feat/option-qualification flag only, no magnitude to ground at all) -- not a missed win. Hunter's own spellcasting (a restricted Summon Nature's Ally-only known-spell list reusing the already-built Druid/Ranger spell lists, plus the existing spontaneous known-spell machinery, with an external-source slot-table caveat like Arcanist/Warpriest) remains deliberately deferred to its own follow-on slice. `named_features_wired` rises from 1 to 3 (Animal Companion + Wild Empathy + Animal Focus). See `pilot_compute.rs`'s `hunter_dispatch_widening_safety_tests` module (9 new tests) for the full test coverage.
