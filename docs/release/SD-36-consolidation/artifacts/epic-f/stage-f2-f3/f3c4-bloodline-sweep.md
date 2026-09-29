# F3c4 bloodline sweep: sorcerer bloodlines beyond Arcane (SD-36 Epic F3)

Measured on `sd36/epic-f2-f3` at `a9d41d21e8`, which is the F3c3 package: records 49,450 and
`rules_written` 73,070. No engine or converter change is in this measurement.

**Commands:**

- The sweep: `scripts/f3c4_bloodline_sweep.rs`, copied into `tests/` as a temporary test and run
  with `cargo test --locked -j 8 --test zz_probe_f3c4 -- --nocapture --test-threads=8`. Its output
  is in `f3c4-bloodline-sweep.log`.
- The gates: `python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/f3c4_bloodline_gates.py`.
  Its output is in `f3c4-bloodline-gates.txt`, and a re-run is byte-identical.

**Posture:** the census fixture (`class_seeds::input_for(fixture, "sorcerer", level)`) with
`choice:sorcerer_bloodline -> bloodline:<x>`. The Arcane Bond pick is removed for a non-Arcane
bloodline, and Arcane keeps its canonical seed. Each bloodline is swept over levels 1..=20.

**Held-set probe:** `HeldSeed { classes: [(sorcerer, 20)], rule_ids: [<record>] }`, compared
against the same seed without the record. Siblings `<record>#*` are excluded.

## Denominator

**32** converted principal records named `<book>:class_feature:sorcerer_bloodline_<x>`, with
bloodline-feat picks excluded. They come from 6 books: core_rulebook 10, advanced_players_guide 10,
ultimate_magic 7, advanced_race_guide 2, occult_adventures 2 and monster_codex 1. Arcane is 1 of
the 32, and 31 are bloodlines the bespoke module does not model.

## Result

| measure | value |
|---|---|
| records reaching Computed at every level 1..=20, with their canonical seed | **1 of 32** (Arcane, the bespoke module) |
| records Blocked at every level 1..=20 | **31 of 32**, each on `class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported` (20 of 20 levels) and on nothing else |
| lines the record grants (`granted_by` edges naming it) | 287 in total: 9 per record, and 8 for Arcane |
| of those edges, gated on a `Var` | **287 of 287** |
| lines the held set reaches beyond the record itself, at sorcerer 20 | **0 of 287** (every record, Arcane included) |
| records whose oracle pick row is an unresolved `Sorcerer Bloodline\|<label>` reference | 27 of 32 (65 rows of `_defects/unresolved-references.json`'s 7,119). The other 5 are Ectoplasm, Ghoul, Imperious, Kobold and Psychic, whose pick rows sit in `support/*_um.lst`. Every var provenance names them `outside_corpus_rows`. |

## Why no bloodline line is held (the mechanism)

Take Draconic as the worked row (`f3c4-bloodline-gates.txt`).

The oracle record `Sorcerer Bloodline ~ Draconic` (`cr_abilities_class.lst:2434`) grants its class
skill, arcana, bonus spells and powers, each behind a variable:

- `ABILITY:Class Skill|AUTOMATIC|Perception|PREVARGTEQ:Sorcerer_Draconic_BloodlineClassSkill1,1`
- `...Bloodline Arcana|PREVARGTEQ:...Arcana1,1`
- `...Power LVL 01|PREVARGTEQ:...Power1,1`
- and so on.

Those variables are raised to 1 by the bloodline's PICK row, `Draconic Bloodline`
`CATEGORY:Sorcerer Bloodline` (`cr_abilities_class.lst:2435`). For example, that row carries:

- `BONUS:VAR|Sorcerer_Draconic_BloodlineClassSkill1|if(Sorcerer_CF_BloodlineClassSkill==0,1,0)`
- `BONUS:VAR|...BloodlineProgressionLVL|BloodlineProgressionLVL`

The pick row is outside the 49,450-unit inventory. It is mechanism E, `forward-scope-register.md`
FS-12, and every `_vars` file for these variables records it as `outside_corpus_rows`.

The converted record therefore carries the nine gated edges, but nothing a plain sorcerer holds
contributes to their variables:

- 166 edge-vars are fed only by Ultimate Magic wildblooded/crossblooded records.
- 112 edge-vars have no contribution at all.
- 9 (Draconic only) are fed by Dragon Disciple's `Draconic Bloodline ~ Standard`
  (`cr_abilities_class.lst:2976`). It sets `ClassSkill1` to `if(SorcererLVL>=1,0,-1)`, which is 0
  for a sorcerer.

So `held_set` reaches none of the lines: no class skill joins the union, and no power, arcana or
bonus-spell line prints.

## Per record

| # | converted record | granted lines (edges) | lines held beyond the record, sorcerer 20 | unresolved `Sorcerer Bloodline\|<label>` refs | levels Computed (of 20) | claim-blocking diagnostics |
|---:|---|---:|---:|---:|---:|---|
| 1 | `core_rulebook:class_feature:sorcerer_bloodline_aberrant` | 9 | 0 | 3 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 2 | `core_rulebook:class_feature:sorcerer_bloodline_abyssal` | 9 | 0 | 3 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 3 | `ultimate_magic:class_feature:sorcerer_bloodline_accursed` | 9 | 0 | 2 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 4 | `advanced_players_guide:class_feature:sorcerer_bloodline_aquatic` | 9 | 0 | 2 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 5 | `core_rulebook:class_feature:sorcerer_bloodline_arcane` | 8 | 0 | 3 | 20 | none |
| 6 | `advanced_players_guide:class_feature:sorcerer_bloodline_boreal` | 9 | 0 | 2 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 7 | `core_rulebook:class_feature:sorcerer_bloodline_celestial` | 9 | 0 | 3 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 8 | `advanced_players_guide:class_feature:sorcerer_bloodline_deep_earth` | 9 | 0 | 2 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 9 | `core_rulebook:class_feature:sorcerer_bloodline_destined` | 9 | 0 | 3 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 10 | `ultimate_magic:class_feature:sorcerer_bloodline_djinni` | 9 | 0 | 2 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 11 | `core_rulebook:class_feature:sorcerer_bloodline_draconic` | 9 | 0 | 4 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 12 | `advanced_players_guide:class_feature:sorcerer_bloodline_dreamspun` | 9 | 0 | 2 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 13 | `occult_adventures:class_feature:sorcerer_bloodline_ectoplasm` | 9 | 0 | 0 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 14 | `ultimate_magic:class_feature:sorcerer_bloodline_efreeti` | 9 | 0 | 2 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 15 | `core_rulebook:class_feature:sorcerer_bloodline_elemental` | 9 | 0 | 3 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 16 | `core_rulebook:class_feature:sorcerer_bloodline_fey` | 9 | 0 | 3 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 17 | `monster_codex:class_feature:sorcerer_bloodline_ghoul` | 9 | 0 | 0 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 18 | `advanced_race_guide:class_feature:sorcerer_bloodline_imperious` | 9 | 0 | 0 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 19 | `core_rulebook:class_feature:sorcerer_bloodline_infernal` | 9 | 0 | 3 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 20 | `advanced_race_guide:class_feature:sorcerer_bloodline_kobold` | 9 | 0 | 0 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 21 | `ultimate_magic:class_feature:sorcerer_bloodline_maestro` | 9 | 0 | 2 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 22 | `ultimate_magic:class_feature:sorcerer_bloodline_marid` | 9 | 0 | 2 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 23 | `advanced_players_guide:class_feature:sorcerer_bloodline_protean` | 9 | 0 | 2 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 24 | `occult_adventures:class_feature:sorcerer_bloodline_psychic` | 9 | 0 | 0 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 25 | `ultimate_magic:class_feature:sorcerer_bloodline_rakshasa` | 9 | 0 | 2 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 26 | `advanced_players_guide:class_feature:sorcerer_bloodline_serpentine` | 9 | 0 | 2 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 27 | `advanced_players_guide:class_feature:sorcerer_bloodline_shadow` | 9 | 0 | 2 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 28 | `ultimate_magic:class_feature:sorcerer_bloodline_shaitan` | 9 | 0 | 2 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 29 | `advanced_players_guide:class_feature:sorcerer_bloodline_starsoul` | 9 | 0 | 2 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 30 | `advanced_players_guide:class_feature:sorcerer_bloodline_stormborn` | 9 | 0 | 2 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 31 | `core_rulebook:class_feature:sorcerer_bloodline_undead` | 9 | 0 | 3 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
| 32 | `advanced_players_guide:class_feature:sorcerer_bloodline_verdant` | 9 | 0 | 2 | 0 | `arcane_bond_and_bloodline_progression.unsupported` (20 of 20) |
