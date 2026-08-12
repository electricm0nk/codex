---
title: GE-06 Required Token Family List Requirements
stc_id: STC-CODEX-GE-06
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts
source_stc: ../README.md
source_artifacts:
  - ../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-token-taxonomy.csv
  - ../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv
---

# GE-06 Required Token Family List Requirements

## Purpose
Enumerate the grounded GE-01 token families the integrated pilot slice depends on, and classify which are hard integration gates versus supporting-but-still-visible scope.

## Grounded basis
The GE-01 pilot token taxonomy currently carries 26 grounded pilot rows. GE-06 does not need to restate every nuance, but it **does** need an explicit gate list for the first integrated slice.

## Hard integration gates
These token families MUST be treated as hard gates for the first integrated pilot path.

| Token family / construct | Why GE-06 needs it | Primary owner |
|---|---|---|
| PCC include directives | The pilot cannot be loaded honestly without the package/include graph. | GE-03 importer |
| CLASS declarations | The slice requires Fighter identity and class progression. | GE-03 importer + GE-02 model |
| RACE declarations | The slice requires Human identity and race-linked entitlements. | GE-03 importer + GE-02 model |
| SKILL declarations | The slice requires skill ranks, governing stats, and class-skill behavior. | GE-03 importer + GE-04 engine |
| EQUIPMENT declarations | The slice requires equipment effects, armor class impact, and attack-related state. | GE-03 importer + GE-04 engine |
| ABILITY | The slice relies on grants and ability-bearing carriers for class/race behavior. | GE-03 importer + GE-02/GE-04 |
| AUTO | Automatic grants must remain visible rather than becoming invisible side effects. | GE-03 importer + GE-04 engine |
| BONUS | Derived numeric behavior depends on grounded bonus semantics. | GE-04 engine |
| PRE* prerequisite guards | Eligibility and gated behavior must be explainable, not flattened away. | GE-03 importer + GE-04 engine |
| CSKILL | The pilot requires class-skill behavior for skill outputs. | GE-03 importer + GE-04 engine |
| KEYSTAT | Skill and save relationships need governing-stat bindings. | GE-03 importer + GE-04 engine |
| STARTSKILLPTS | Fighter skill-budget behavior must be modeled for the pilot. | GE-04 engine |
| STATMOD / MODIFY | Ability-score modifiers and downstream stat links are core to the slice. | GE-04 engine |
| STARTFEATS | Human starting-feat entitlement affects deterministic fixture closure. | GE-04 engine |
| ABILITYPOOL | Human bonus feat / ability-score choice debt must remain explicit. | GE-04 engine |
| CHOOSE | Selector-driven choice spaces cannot be ignored if the pilot touches them. | GE-03 importer + GE-04 engine |
| PROFICIENCY and PROFICIENCY:ARMOR / PROFICIENCY:SHIELD | Equipment and class/feat grants must resolve to consistent proficiency references. | GE-03 importer + GE-02/GE-04 |
| PREVARGTEQ / PREVAREQ / PREFACT | Guarded class/race/equipment behavior must preserve predicate truth. | GE-03 importer + GE-04 engine |

## Supporting but still visible token families
These surfaces MAY be supporting rather than first-pass blockers, but they must remain visible if touched by the selected pilot path.

| Token family / construct | Why it still matters |
|---|---|
| WEAPONPROF / ARMORPROF / SHIELDPROF catalogs | They ground grouped proficiency references and selectors. |
| TYPE facets | They influence grouping, filtering, and selector semantics. |
| DEFINE / VAR | They provide named variables and progression anchors that BONUS/skill-budget logic may consume. |
| FACT / MOVE / TEMPLATE / RACETYPE / LEGS / HANDS | They carry race/body metadata that may matter for faithful model identity even when not first-pass blockers. |
| MULT | Repeatability semantics must remain explicit when choice-driven feat paths are involved. |

## Classification rule
GE-06 implementation or readiness work MUST classify every touched token family as one of:
- hard gate
- supporting dependency
- deferred risk with explicit owner

The slice MUST NOT use generic phrases like “import coverage” when the real issue is an unclosed token-family obligation.
