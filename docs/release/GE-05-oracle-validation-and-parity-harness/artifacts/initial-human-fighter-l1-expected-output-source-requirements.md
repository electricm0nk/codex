---
title: GE-05 Initial Human Fighter Level 1 Expected-Output Source Requirements
stc_id: STC-CODEX-GE-05
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts
source_stc: ../README.md
related:
  - ../../../plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md
  - ../../GE-04-rules-engine-and-explainability-core/artifacts/pilot-golden-computation-fixture-requirements.md
---

# GE-05 Initial Human Fighter Level 1 Expected-Output Source Requirements

## Purpose
Define the source requirements for the first expected-output comparison case:

```text
Pathfinder 1e Core Rulebook Human Fighter level 1
```

This artifact does not contain final expected values. It defines what evidence must exist before those values can be accepted.

## Inherited pilot identity
The initial case inherits this pilot input from the pilot charter:

```yaml
case: pf1-crb-human-fighter-level1
sources:
  - pathfinder/core_rulebook
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
    - power_attack
expect:
  includes:
    - base_attack_bonus
    - fortitude_save
    - reflex_save
    - will_save
    - melee_attack_bonus
    - armor_class
    - skill_ranks
    - equipment_effects
```

Skills, equipment, and choice details remain unresolved until the fixture is grounded by GE-05/GE-06 work.

## Required expected-output categories
The first expected-output fixture SHOULD attempt to define, compare, or explicitly gap the following categories:

| Category | Requirement | Initial status |
|---|---|---|
| Source package / campaign loaded | Record PCGen and Codex source package identity and load evidence. | Candidate. |
| Loaded content summary | Compare object/count summaries where both systems expose comparable data. | Candidate, route-dependent. |
| Character identity | Race, class, level, and selected feature identity. | Candidate. |
| Ability modifiers | Derived modifiers from supplied ability scores. | Candidate; final values require old/new evidence. |
| Base attack and attack-related output | Pilot Fighter attack-relevant derived values under selected equipment path. | Candidate, equipment-dependent. |
| Saving throws | Fortitude, Reflex, and Will for the selected character state. | Candidate. |
| Armor/equipment-influenced values | Armor class or equivalent values when equipment is selected. | Candidate, equipment-dependent. |
| Skills | Skill ranks/class-skill handling sufficient for the pilot. | Candidate, skill allocation-dependent. |
| Feat/prerequisite or choice availability | At least one choice/prerequisite path such as Power Attack/proficiency context. | Candidate, final route-dependent. |
| Diagnostics and known gaps | Importer, rules, oracle, normalization, and fixture diagnostics. | Mandatory. |
| Exportable summary/stat-block | One limited export/summary if a PCGen output route exists. | Optional/known-gap candidate. |

## Evidence required per expected output
Each accepted expected-output field MUST record:

- field ID
- old-system source reference
- new-system source reference
- raw old value or reference
- raw new value or reference
- normalization rule, if any
- compared result
- diagnostics/provenance/explanation references
- known-gap or decision reference when not comparable or intentionally divergent

## Values are not accepted until grounded
Final expected values MUST NOT be accepted from:

- manual calculation without source evidence
- static PCGen source files alone
- GE-04 computed output alone
- UI screenshots without repeatable output capture
- broad Pathfinder assumptions

Final expected values MAY be accepted when they are tied to reproducible PCGen output, reproducible Codex output, explicit normalization, and a parity report.

## GE-03 and GE-04 inherited proof obligations
For every comparable Codex output, the fixture SHOULD preserve:

- GE-03 provenance/source-map references where imported content contributes
- GE-03 diagnostics for unsupported/lossy/deferred imported semantics
- GE-04 explanation references for derived values or failed choices
- GE-04 diagnostics for invalid content, unresolved references, expression issues, dependency instability, or engine defects

## Known-gap fallback
If a category cannot be compared, the fixture MUST record:

- category ID
- why it is not comparable
- whether the blocker is oracle route, new-system implementation, normalization, licensing, scope, or intentional divergence
- owning GE or later handoff
- review trigger

## Completion rule
The source requirements are complete when a future fixture author can populate old/new evidence without guessing, and can record blocked or known-gap dimensions without hiding them.
