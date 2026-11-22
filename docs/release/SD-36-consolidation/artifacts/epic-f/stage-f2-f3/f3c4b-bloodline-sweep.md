# F3c4b bloodline sweep: sorcerer bloodline lines through the converted pick option (SD-36 Epic F3)

Measured on `sd36/epic-f2-f3` with the F3c4b package (records 49,450, `rules_written` 73,360). It
re-runs F3c4's sweep (`f3c4-bloodline-sweep.md`, 0 of 287 lines reachable) against the package
where each bloodline's `CATEGORY:Sorcerer Bloodline` pick row is a `pool_option` rule granted by
`Standard Bloodline`'s choice (`f3c4b-receipt.md` §2).

**Command:** `scripts/f3c4b_bloodline_sweep.rs`, copied into `tests/` as the temporary test
`tests/zz_probe_f3c4b.rs` and run with
`cargo test --locked -j 8 --test zz_probe_f3c4b -- --nocapture --test-threads=8`, then deleted. Its
rows are in `f3c4b-bloodline-sweep.log`.

**Probes:**

- **Via the pick (new).** `HeldSeed { race, classes: [(sorcerer, 20)] }`, with the character's choice
  `core_rulebook:class_feature:sorcerer_standard_bloodline_selection -> <the bloodline's
  pool_option>`. A line counts as reachable when the held set holds it.
- **Record alone (F3c4's probe, kept for comparison).** The record held outright, no pick.
- **At its own level.** Each line whose gate states `<bloodline> Progression LVL >= N` (1 when it
  states none) is probed again at sorcerer N via the pick. This checks the level the package holds
  a line from against the level its own gate (and the book) states.
- **Race.** The census fixture is human. A pick option gated on another race (`Holds
  template:is<race>`) is probed with that race.
- **Census.** Unchanged from F3c4: `class_seeds::input_for(fixture, "sorcerer", level)` with
  `choice:sorcerer_bloodline -> bloodline:<x>`, levels 1..=20.

## Denominator

**32** converted principal records `<book>:class_feature:sorcerer_bloodline_<x>`, 287 `granted_by`
edges naming them (9 each, 8 for Arcane). Each record now has exactly one pick option in pool
`sorcerer_bloodline` that grants it: **32 of 32**.

## Result

| measure | F3c4 | F3c4b |
|---|---:|---:|
| bloodline lines reachable at sorcerer 20 | 0 of 287 | **269 of 287** |
| of those, held from the level their own gate states | -- | 267 of 287 checked, 20 not |
| bloodlines Computed single-class, every level 1..=20 | 1 of 32 (Arcane) | **1 of 32** (Arcane) |
| lines held with the record alone (no pick) | 0 of 287 | 0 of 287 |
| unresolved `Sorcerer Bloodline\|<X> Bloodline` references | 65 | **0** |

## The remainder, by mechanism

- **18 of 287 lines unreachable: race-gated pick rows.** Imperious (`arg_abilities_class.lst:13`,
  `PREFACT:1,TEMPLATES,IsHuman=true`) and Kobold (`:631`, the kobold twin) convert with the gate
  `Holds <book>:template:is<race>`. In the package that template is granted only through
  `template:race_<race>` (by three race traits for human), never by the race itself. The oracle
  race row states `TEMPLATE:Human` (`core_essentials/races/human/human_races.lst:6`), and that edge
  does not reach the package's race. So a human (or a kobold) never holds the gate, and 9 + 9 lines
  stay off. This is a race-template conversion gap, not a pick-row one, and it is named as FS-19.
- **2 of 287 lines held late: a single-line record keeps its conditional line as its principal.**
  Draconic Claws (`cr_abilities_class.lst:2444`) and Abyssal Claws convert to ONE line, their
  claw-size bonus gated `Power1LVL >= 7`. SD-36 Epic E CONV-02 keeps a one-line record's line as its
  principal, so the record is held from sorcerer 7 where CRB p.75 grants it at 1. Named as FS-18.
- **31 of 32 bloodlines still Blocked single-class: the engine does not link the pick.** The
  Sorcerer module (`pilot_compute/class_sorcerer_wizard.rs`) still refuses every non-Arcane
  bloodline on `class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported`.
  Three things keep it refusing:
  - The character's Path-A `choice:sorcerer_bloodline -> bloodline:<x>` is not linked to the
    converted option.
  - The class-skill reader (`class_skill_sheet_rules`) reads canonical seeds per class, not the
    character's own pick. A flipped Aquatic sorcerer would print Swim 3 where PF1 gives 6
    (`f3c4-receipt.md` §3).
  - The SD-32 generic pool-group magnitude pass is still level-ungated (`f3c4-receipt.md` §6).

  This step changed no engine code, so the count is 1 of 32 exactly as before. The converter half
  of FS-17 is closed; the engine half stays open.

## Per record

| # | converted record | probe race | granted lines (edges) | pick option | lines held via the pick, sorcerer 20 | lines held with the record alone (F3c4 probe) | not held at the level its own gate states | levels Computed (of 20) | claim-blocking diagnostics |
|---:|---|---|---:|---|---:|---:|---|---:|---|
| 1 | `core_rulebook:class_feature:sorcerer_bloodline_aberrant` | human | 9 | `core_rulebook:pool_option:sorcerer_bloodline_aberrant_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 2 | `core_rulebook:class_feature:sorcerer_bloodline_abyssal` | human | 9 | `core_rulebook:pool_option:sorcerer_bloodline_abyssal_bloodline` | 9 | 0 | `core_rulebook:class_feature:abyssal_bloodline_claws@1` | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 3 | `ultimate_magic:class_feature:sorcerer_bloodline_accursed` | human | 9 | `ultimate_magic:pool_option:sorcerer_bloodline_accursed_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 4 | `advanced_players_guide:class_feature:sorcerer_bloodline_aquatic` | human | 9 | `advanced_players_guide:pool_option:sorcerer_bloodline_aquatic_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 5 | `core_rulebook:class_feature:sorcerer_bloodline_arcane` | human | 8 | `core_rulebook:pool_option:sorcerer_bloodline_arcane_bloodline` | 8 | 0 | none | 20 | none |
| 6 | `advanced_players_guide:class_feature:sorcerer_bloodline_boreal` | human | 9 | `advanced_players_guide:pool_option:sorcerer_bloodline_boreal_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 7 | `core_rulebook:class_feature:sorcerer_bloodline_celestial` | human | 9 | `core_rulebook:pool_option:sorcerer_bloodline_celestial_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 8 | `advanced_players_guide:class_feature:sorcerer_bloodline_deep_earth` | human | 9 | `advanced_players_guide:pool_option:sorcerer_bloodline_deep_earth_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 9 | `core_rulebook:class_feature:sorcerer_bloodline_destined` | human | 9 | `core_rulebook:pool_option:sorcerer_bloodline_destined_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 10 | `ultimate_magic:class_feature:sorcerer_bloodline_djinni` | human | 9 | `ultimate_magic:pool_option:sorcerer_bloodline_djinni_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 11 | `core_rulebook:class_feature:sorcerer_bloodline_draconic` | human | 9 | `core_rulebook:pool_option:sorcerer_bloodline_draconic_bloodline` | 9 | 0 | `core_rulebook:class_feature:draconic_bloodline_claws@1` | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 12 | `advanced_players_guide:class_feature:sorcerer_bloodline_dreamspun` | human | 9 | `advanced_players_guide:pool_option:sorcerer_bloodline_dreamspun_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 13 | `occult_adventures:class_feature:sorcerer_bloodline_ectoplasm` | human | 9 | `occult_adventures:pool_option:sorcerer_bloodline_ectoplasm_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 14 | `ultimate_magic:class_feature:sorcerer_bloodline_efreeti` | human | 9 | `ultimate_magic:pool_option:sorcerer_bloodline_efreeti_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 15 | `core_rulebook:class_feature:sorcerer_bloodline_elemental` | human | 9 | `core_rulebook:pool_option:sorcerer_bloodline_elemental_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 16 | `core_rulebook:class_feature:sorcerer_bloodline_fey` | human | 9 | `core_rulebook:pool_option:sorcerer_bloodline_fey_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 17 | `monster_codex:class_feature:sorcerer_bloodline_ghoul` | human | 9 | `monster_codex:pool_option:sorcerer_bloodline_ghoul_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 18 | `advanced_race_guide:class_feature:sorcerer_bloodline_imperious` | human | 9 | `advanced_race_guide:pool_option:sorcerer_bloodline_imperious_bloodline` | 0 | 0 | `core_rulebook:ability:perform_oratory@1`, `advanced_race_guide:class_feature:imperious_bloodline_bloodline_arcana@1`... | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 19 | `core_rulebook:class_feature:sorcerer_bloodline_infernal` | human | 9 | `core_rulebook:pool_option:sorcerer_bloodline_infernal_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 20 | `advanced_race_guide:class_feature:sorcerer_bloodline_kobold` | kobold | 9 | `advanced_race_guide:pool_option:sorcerer_bloodline_kobold_bloodline` | 0 | 0 | `core_rulebook:ability:disable_device@1`, `advanced_race_guide:class_feature:kobold_bloodline_arcane_ambush@9`, `adva... | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 21 | `ultimate_magic:class_feature:sorcerer_bloodline_maestro` | human | 9 | `ultimate_magic:pool_option:sorcerer_bloodline_maestro_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 22 | `ultimate_magic:class_feature:sorcerer_bloodline_marid` | human | 9 | `ultimate_magic:pool_option:sorcerer_bloodline_marid_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 23 | `advanced_players_guide:class_feature:sorcerer_bloodline_protean` | human | 9 | `advanced_players_guide:pool_option:sorcerer_bloodline_protean_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 24 | `occult_adventures:class_feature:sorcerer_bloodline_psychic` | human | 9 | `occult_adventures:pool_option:sorcerer_bloodline_psychic_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 25 | `ultimate_magic:class_feature:sorcerer_bloodline_rakshasa` | human | 9 | `ultimate_magic:pool_option:sorcerer_bloodline_rakshasa_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 26 | `advanced_players_guide:class_feature:sorcerer_bloodline_serpentine` | human | 9 | `advanced_players_guide:pool_option:sorcerer_bloodline_serpentine_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 27 | `advanced_players_guide:class_feature:sorcerer_bloodline_shadow` | human | 9 | `advanced_players_guide:pool_option:sorcerer_bloodline_shadow_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 28 | `ultimate_magic:class_feature:sorcerer_bloodline_shaitan` | human | 9 | `ultimate_magic:pool_option:sorcerer_bloodline_shaitan_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 29 | `advanced_players_guide:class_feature:sorcerer_bloodline_starsoul` | human | 9 | `advanced_players_guide:pool_option:sorcerer_bloodline_starsoul_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 30 | `advanced_players_guide:class_feature:sorcerer_bloodline_stormborn` | human | 9 | `advanced_players_guide:pool_option:sorcerer_bloodline_stormborn_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 31 | `core_rulebook:class_feature:sorcerer_bloodline_undead` | human | 9 | `core_rulebook:pool_option:sorcerer_bloodline_undead_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
| 32 | `advanced_players_guide:class_feature:sorcerer_bloodline_verdant` | human | 9 | `advanced_players_guide:pool_option:sorcerer_bloodline_verdant_bloodline` | 9 | 0 | none | 0 | arcane_bond_and_bloodline_progression.unsupported (20 of 20) |
