---
title: GE-06 Pilot Character Fixture Requirements
stc_id: STC-CODEX-GE-06
artifact_type: generated-documentary-output
status: accepted
scope: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts
source_stc: ../README.md
related:
  - ../../../plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md
  - ../../GE-04-rules-engine-and-explainability-core/artifacts/pilot-golden-computation-fixture-requirements.md
  - ../../GE-05-oracle-validation-and-parity-harness/technical-requirements.md
  - ./ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md
---

# GE-06 Pilot Character Fixture Requirements

## Purpose
Define the integrated pilot character fixture boundary GE-06 needs before later implementation can prove the vertical slice honestly.

This artifact defines the fixture boundary. The accepted final deterministic input choices for the first bounded pilot case are closed in `ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`. Final computed values and parity results remain evidence-gated and are not fabricated here.

## Case identity
The first integrated GE-06 case MUST remain:

```text
pf1-crb-human-fighter-level1
```

## Grounded selections from accepted/planned authority surfaces
GE-06 may treat the following as grounded inputs today:
- race: Human
- class: Fighter
- level: 1
- ability scores: STR 16, DEX 14, CON 14, INT 10, WIS 12, CHA 8
- at least one named feat path: `power_attack`
- required output categories include base attack bonus, Fortitude save, Reflex save, Will save, melee attack bonus, armor class, skill ranks, and equipment effects

## Grounded candidate content anchors now closed for the first pilot
Existing GE-01 inputs ground representative pilot content surfaces such as:
- Chain Shirt equipment row
- Longsword equipment row
- Fighter class-skill and proficiency grant surfaces
- Human starting-feat and ability-pool entitlement surfaces

The final deterministic contract settles the first pilot loadout and full choice set as Chain Shirt worn, Longsword primary, no shield, Human +2 STR, Power Attack, Dodge, Weapon Focus (Longsword), Climb 1, Intimidate 1, Swim 1, and one headless summary receipt boundary.

## Closed selection debt for the first bounded pilot
The following inputs are no longer unresolved for the first GE-06 pilot case:
- Human ability target: STR
- feat slots: Power Attack, Dodge, Weapon Focus (Longsword)
- skill allocation: Climb 1, Intimidate 1, Swim 1
- equipment and active state: Chain Shirt worn, Longsword primary, no shield, Power Attack selected but inactive for baseline outputs
- export boundary: one headless summary receipt

Any later change to these selections is scope-bearing and must update the deterministic input contract and review the pilot charter/ADR triggers.

## Required fixture schema
A later implementation-facing fixture MUST be able to represent at minimum:

```yaml
case_id: pf1-crb-human-fighter-level1
source_package: pf1.crb
character:
  race: human
  class_levels:
    fighter: 1
  ability_scores:
    str: 16
    dex: 14
    con: 14
    int: 10
    wis: 12
    cha: 8
  feats:
    grounded:
      - power_attack
      - dodge
      - weapon_focus_longsword
    unresolved_additional_entitlements: false
  skills:
    Climb: 1
    Intimidate: 1
    Swim: 1
  equipment:
    worn_armor: Chain Shirt
    primary_weapon: Longsword
    shield: none
  active_states:
    power_attack: selected_inactive_for_baseline_outputs
required_outputs:
  - base_attack_bonus
  - fortitude_save
  - reflex_save
  - will_save
  - melee_attack_bonus
  - armor_class
  - skill_ranks
  - equipment_effects
```

## Explanation obligations
For each selected derived value under test, the later integrated fixture SHOULD be able to assert:
- character input contributions
- canonical object/effect/formula contributions
- prerequisite or choice-state contributions where relevant
- provenance/source-map references when imported content contributes
- diagnostics or known-gap references when behavior is blocked, deferred, or unsupported

## Oracle boundary
GE-06 may require selected comparison targets for viability, but this artifact does **not** fabricate final old-system or new-system expected values.

Those values require:
- GE-04-backed computed output
- GE-05-backed comparison standards and evidence

## Final rule
The pilot fixture input is no longer allowed to be unresolved for the first bounded GE-06 case. Runtime values may still be unresolved until computed and oracle evidence exists, but input ambiguity is closed.
