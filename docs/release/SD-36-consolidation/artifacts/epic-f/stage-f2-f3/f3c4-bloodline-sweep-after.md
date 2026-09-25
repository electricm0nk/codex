# F3c4 bloodline sweep, after the engine link (SD-36 Epic F3, step F3c4)

Measured on `sd36/epic-f2-f3` (F3c4b package, records 49,450, `rules_written` 73,360, unchanged) with the F3c4 engine change: the character's Path-A pick `choice:sorcerer_bloodline -> bloodline:<x>` is linked to the converted pick option (`sheet_rule::link_path_a_picks`), and the Sorcerer module yields to the converted record for any bloodline it does not model.

**Command:** `docs/release/SD-36-consolidation/artifacts/epic-f/scripts/f3c4b_bloodline_sweep.rs` (the F3c4b sweep, unchanged), copied to `tests/zz_probe_f3c4.rs`, run with `cargo test --locked -j 8 --test zz_probe_f3c4 -- --nocapture --test-threads=8`, then deleted. Rows: `f3c4-bloodline-sweep-after.log`. The census probe is `class_seeds::input_for(fixture, "sorcerer", level)` with `choice:sorcerer_bloodline -> bloodline:<x>` (and, for any bloodline but Arcane, the Arcane Bond pick removed), levels 1..=20.

## Denominator

**32** converted principal records `<book>:class_feature:sorcerer_bloodline_<x>`, 287 `granted_by` edges naming them.

## Result

| measure | F3c4 (`713bcfba74`) | F3c4b (`07f02c6387`) | F3c4 engine (this step) |
|---|---:|---:|---:|
| bloodlines Computed single-class at every level 1..=20 | 1 of 32 | 1 of 32 | **30 of 32** |
| bloodline lines reachable through the pick at sorcerer 20 | 0 of 287 | 269 of 287 | 269 of 287 (no converter change) |
| lines held from the level their own gate states | -- | 267 of 287 checked, 20 not | 267 of 287, 20 not (unchanged) |

## The remainder, by mechanism

- **2 of 32 bloodlines Blocked: the pick option's own gate excludes the character (FS-19).** Imperious and Kobold link to their option, but the option is race-gated through `Holds <book>:template:is<race>`, a template the package's race never grants (`f3c4b-receipt.md` §5). The held set holds none of their lines, so the Sorcerer module names it: `class_feature.sorcerer.bloodline.converted_option_not_held` (claim-blocking), alongside the bespoke `arcane_bond_and_bloodline_progression.unsupported`. They are never passed as Computed with an empty bloodline.
- **The 18 unreachable lines are those same two bloodlines' lines (FS-19).**
- **2 lines held late (FS-18):** Draconic and Abyssal Claws print from sorcerer 7, where CRB p.75 grants them at 1 (CONV-02 single-line principal). No sheet total reads them.

## Per record

| # | converted record | pick option | lines held via the pick, sorcerer 20 (of edges) | levels Computed (of 20) | claim-blocking diagnostics |
|---:|---|---|---:|---:|---|
| 1 | `core_rulebook:class_feature:sorcerer_bloodline_aberrant` | `core_rulebook:pool_option:sorcerer_bloodline_aberrant_bloodline` | 9 of 9 | 20 | none |
| 2 | `core_rulebook:class_feature:sorcerer_bloodline_abyssal` | `core_rulebook:pool_option:sorcerer_bloodline_abyssal_bloodline` | 9 of 9 | 20 | none |
| 3 | `ultimate_magic:class_feature:sorcerer_bloodline_accursed` | `ultimate_magic:pool_option:sorcerer_bloodline_accursed_bloodline` | 9 of 9 | 20 | none |
| 4 | `advanced_players_guide:class_feature:sorcerer_bloodline_aquatic` | `advanced_players_guide:pool_option:sorcerer_bloodline_aquatic_bloodline` | 9 of 9 | 20 | none |
| 5 | `core_rulebook:class_feature:sorcerer_bloodline_arcane` | `core_rulebook:pool_option:sorcerer_bloodline_arcane_bloodline` | 8 of 8 | 20 | none |
| 6 | `advanced_players_guide:class_feature:sorcerer_bloodline_boreal` | `advanced_players_guide:pool_option:sorcerer_bloodline_boreal_bloodline` | 9 of 9 | 20 | none |
| 7 | `core_rulebook:class_feature:sorcerer_bloodline_celestial` | `core_rulebook:pool_option:sorcerer_bloodline_celestial_bloodline` | 9 of 9 | 20 | none |
| 8 | `advanced_players_guide:class_feature:sorcerer_bloodline_deep_earth` | `advanced_players_guide:pool_option:sorcerer_bloodline_deep_earth_bloodline` | 9 of 9 | 20 | none |
| 9 | `core_rulebook:class_feature:sorcerer_bloodline_destined` | `core_rulebook:pool_option:sorcerer_bloodline_destined_bloodline` | 9 of 9 | 20 | none |
| 10 | `ultimate_magic:class_feature:sorcerer_bloodline_djinni` | `ultimate_magic:pool_option:sorcerer_bloodline_djinni_bloodline` | 9 of 9 | 20 | none |
| 11 | `core_rulebook:class_feature:sorcerer_bloodline_draconic` | `core_rulebook:pool_option:sorcerer_bloodline_draconic_bloodline` | 9 of 9 | 20 | none |
| 12 | `advanced_players_guide:class_feature:sorcerer_bloodline_dreamspun` | `advanced_players_guide:pool_option:sorcerer_bloodline_dreamspun_bloodline` | 9 of 9 | 20 | none |
| 13 | `occult_adventures:class_feature:sorcerer_bloodline_ectoplasm` | `occult_adventures:pool_option:sorcerer_bloodline_ectoplasm_bloodline` | 9 of 9 | 20 | none |
| 14 | `ultimate_magic:class_feature:sorcerer_bloodline_efreeti` | `ultimate_magic:pool_option:sorcerer_bloodline_efreeti_bloodline` | 9 of 9 | 20 | none |
| 15 | `core_rulebook:class_feature:sorcerer_bloodline_elemental` | `core_rulebook:pool_option:sorcerer_bloodline_elemental_bloodline` | 9 of 9 | 20 | none |
| 16 | `core_rulebook:class_feature:sorcerer_bloodline_fey` | `core_rulebook:pool_option:sorcerer_bloodline_fey_bloodline` | 9 of 9 | 20 | none |
| 17 | `monster_codex:class_feature:sorcerer_bloodline_ghoul` | `monster_codex:pool_option:sorcerer_bloodline_ghoul_bloodline` | 9 of 9 | 20 | none |
| 18 | `advanced_race_guide:class_feature:sorcerer_bloodline_imperious` | `advanced_race_guide:pool_option:sorcerer_bloodline_imperious_bloodline` | 0 of 9 | 0 | class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported (20 of 20); class_feature.sorcerer.bloodline.converted_option_not_held (20 of 20) |
| 19 | `core_rulebook:class_feature:sorcerer_bloodline_infernal` | `core_rulebook:pool_option:sorcerer_bloodline_infernal_bloodline` | 9 of 9 | 20 | none |
| 20 | `advanced_race_guide:class_feature:sorcerer_bloodline_kobold` | `advanced_race_guide:pool_option:sorcerer_bloodline_kobold_bloodline` | 0 of 9 | 0 | class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported (20 of 20); class_feature.sorcerer.bloodline.converted_option_not_held (20 of 20) |
| 21 | `ultimate_magic:class_feature:sorcerer_bloodline_maestro` | `ultimate_magic:pool_option:sorcerer_bloodline_maestro_bloodline` | 9 of 9 | 20 | none |
| 22 | `ultimate_magic:class_feature:sorcerer_bloodline_marid` | `ultimate_magic:pool_option:sorcerer_bloodline_marid_bloodline` | 9 of 9 | 20 | none |
| 23 | `advanced_players_guide:class_feature:sorcerer_bloodline_protean` | `advanced_players_guide:pool_option:sorcerer_bloodline_protean_bloodline` | 9 of 9 | 20 | none |
| 24 | `occult_adventures:class_feature:sorcerer_bloodline_psychic` | `occult_adventures:pool_option:sorcerer_bloodline_psychic_bloodline` | 9 of 9 | 20 | none |
| 25 | `ultimate_magic:class_feature:sorcerer_bloodline_rakshasa` | `ultimate_magic:pool_option:sorcerer_bloodline_rakshasa_bloodline` | 9 of 9 | 20 | none |
| 26 | `advanced_players_guide:class_feature:sorcerer_bloodline_serpentine` | `advanced_players_guide:pool_option:sorcerer_bloodline_serpentine_bloodline` | 9 of 9 | 20 | none |
| 27 | `advanced_players_guide:class_feature:sorcerer_bloodline_shadow` | `advanced_players_guide:pool_option:sorcerer_bloodline_shadow_bloodline` | 9 of 9 | 20 | none |
| 28 | `ultimate_magic:class_feature:sorcerer_bloodline_shaitan` | `ultimate_magic:pool_option:sorcerer_bloodline_shaitan_bloodline` | 9 of 9 | 20 | none |
| 29 | `advanced_players_guide:class_feature:sorcerer_bloodline_starsoul` | `advanced_players_guide:pool_option:sorcerer_bloodline_starsoul_bloodline` | 9 of 9 | 20 | none |
| 30 | `advanced_players_guide:class_feature:sorcerer_bloodline_stormborn` | `advanced_players_guide:pool_option:sorcerer_bloodline_stormborn_bloodline` | 9 of 9 | 20 | none |
| 31 | `core_rulebook:class_feature:sorcerer_bloodline_undead` | `core_rulebook:pool_option:sorcerer_bloodline_undead_bloodline` | 9 of 9 | 20 | none |
| 32 | `advanced_players_guide:class_feature:sorcerer_bloodline_verdant` | `advanced_players_guide:pool_option:sorcerer_bloodline_verdant_bloodline` | 9 of 9 | 20 | none |

`TOTAL|edges 287|held via pick 269|held record alone 0|level-checked 287|not held at the level its own gate states (1 when none) 20`

