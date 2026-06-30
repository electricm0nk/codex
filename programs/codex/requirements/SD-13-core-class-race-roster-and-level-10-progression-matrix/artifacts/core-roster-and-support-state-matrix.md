# SD-13 Core Class/Race Roster and Level-10 Progression Matrix

**Artifact type:** Support-state seed authority  
**Slice:** SD-13-E1-F1  
**Status:** Active — breadth roster (unverified rows pending per-subject uplift slices)

---

## Race Roster

| Row ID | Subject ID | Dimension | Notes |
|--------|-----------|-----------|-------|
| `race.human.pilot_semantics` | `race:human` | Bounded pilot race semantics | Partially grounded by GE-06 deterministic proof |
| `race.dwarf.bounded_semantics` | `race:dwarf` | Bounded race semantics | Unverified — awaiting SD13-E2 race-semantic slice |
| `race.elf.bounded_semantics` | `race:elf` | Bounded race semantics | Unverified — awaiting SD13-E2 race-semantic slice |
| `race.gnome.bounded_semantics` | `race:gnome` | Bounded race semantics | Unverified — awaiting SD13-E2 race-semantic slice |
| `race.half_elf.bounded_semantics` | `race:half-elf` | Bounded race semantics | Unverified — awaiting SD13-E2 race-semantic slice |
| `race.half_orc.bounded_semantics` | `race:half-orc` | Bounded race semantics | Unverified — awaiting SD13-E2 race-semantic slice |
| `race.halfling.bounded_semantics` | `race:halfling` | Bounded race semantics | Unverified — awaiting SD13-E2 race-semantic slice |

---

## Class Roster (Level-10 Progression)

| Row ID | Subject ID | Dimension | Notes |
|--------|-----------|-----------|-------|
| `class.fighter.level_1_pilot` | `class:fighter` | Class progression through level 1 deterministic pilot surface | Partially grounded by GE-06 tests |
| `class.fighter.levels_2_10` | `class:fighter` | Class progression through levels 2-10 | Blocked — GE-06 claim-blocking diagnostics |
| `class.rogue.bounded_progression` | `class:rogue` | Bounded class progression | Blocked — GE-06 claim-blocking diagnostics |
| `class.barbarian.bounded_progression` | `class:barbarian` | Bounded class progression | Unverified — awaiting SD13-E3 martial progression slice |
| `class.bard.progression_and_spell_burden` | `class:bard` | Bounded class progression and spell burden | Unverified — awaiting SD13-E4 spellcasting slice |
| `class.cleric.progression_and_spell_burden` | `class:cleric` | Bounded class progression and spell burden | Unverified — awaiting SD13-E4 spellcasting slice |
| `class.druid.progression_and_spell_burden` | `class:druid` | Bounded class progression and spell burden | Unverified — awaiting SD13-E4 spellcasting slice |
| `class.monk.bounded_progression` | `class:monk` | Bounded class progression | Unverified — awaiting SD13-E3 martial progression slice |
| `class.paladin.hybrid_chassis_and_spell_burden` | `class:paladin` | Bounded class progression and hybrid spell burden | Unverified — awaiting SD13-E3 then SD13-E4 |
| `class.ranger.hybrid_chassis_and_spell_burden` | `class:ranger` | Bounded class progression and hybrid spell burden | Unverified — awaiting SD13-E3 then SD13-E4 |
| `class.sorcerer.progression_and_spell_burden` | `class:sorcerer` | Bounded class progression and spell burden | Unverified — awaiting SD13-E4 spellcasting slice |
| `class.wizard.progression_and_spell_burden` | `class:wizard` | Bounded class progression and spell burden | Unverified — awaiting SD13-E4 spellcasting slice |

---

## Interaction Rows

| Row ID | Subject ID | Dimension | Notes |
|--------|-----------|-----------|-------|
| `interaction.human_bonus_feat_ability_bonus.pilot_pressure` | `interaction:human-bonus-feat-ability-bonus` | Race/class interaction pressure on the deterministic pilot path | Partially grounded by GE-06 deterministic proof |
| `interaction.non_human_any_class.progression_pressure` | `interaction:non-human-any-class-progression` | Race/class interaction pressure beyond the pilot | Unverified — add named rows only where separate race and class rows are insufficient |

---

## Authority

This document is the SD-13 source-of-truth roster authority for `Unverified` + `Observed` breadth rows in `src/oracle_validation/support_state_matrix.rs`. Rows citing this artifact as `grounding_ref` have been included in the bounded roster but not yet verified by direct test evidence.
