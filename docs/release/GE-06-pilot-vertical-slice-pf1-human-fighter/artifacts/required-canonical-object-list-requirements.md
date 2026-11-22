---
title: GE-06 Required Canonical Object List Requirements
stc_id: STC-CODEX-GE-06
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts
source_stc: ../README.md
source_artifacts:
  - ../../GE-02-canonical-rules-model-and-content-packages/artifacts/canonical-model-specification.md
  - ../../GE-02-canonical-rules-model-and-content-packages/artifacts/pilot-object-examples.yaml
---

# GE-06 Required Canonical Object List Requirements

## Purpose
Enumerate the minimum GE-02 canonical model homes and support records the integrated pilot slice depends on.

## Required semantic objects
The integrated slice MUST be able to consume or validate at minimum the following semantic objects.

| Canonical object | Why GE-06 needs it |
|---|---|
| `SourcePackage` | The pilot must identify and load the PF1 Core Rulebook package boundary honestly. |
| `StableId` | Every imported or referenced object must have durable identity across import, compute, compare, and UI surfaces. |
| `Race` | The pilot requires Human identity. |
| `RaceTrait` | Human entitlements and default trait composition must remain explainable. |
| `Class` | The pilot requires Fighter level progression identity. |
| `ClassFeature` | Fighter class-grant behavior must remain separable from class identity. |
| `Feat` | The pilot names at least one explicit feat path and may carry additional feat debt. |
| `Skill` | The pilot requires skill-rank outputs and governing-stat/class-skill behavior. |
| `Equipment` | The pilot requires armor/weapon/item effects and selected equipment state. |
| `Proficiency` | Equipment and grants must resolve to stable proficiency concepts. |
| `AbilityScore` | The pilot starts from explicit ability scores and modifier-linked downstream behavior. |
| `Save` | The pilot requires Fortitude, Reflex, and Will outputs. |

## Required first-class support records
The integrated slice MUST also preserve the following support records as first-class model citizens rather than hidden implementation details.

| Support record | Why GE-06 needs it |
|---|---|
| `Effect` | The integrated slice must explain what contributes to derived values and grants. |
| `Prerequisite` | Failed or unavailable choices must be explainable as structured gates. |
| `Formula` | Derived-value computation and progression math must remain inspectable. |
| `ChoiceSet` | Human/Fighter selection debt and selector-driven choices must remain explicit. |
| `Selector` | Group or type-based choice filtering must not collapse into prose. |
| `Diagnostic` | Unsupported, invalid, blocked, or unresolved behavior must remain visible. |
| `ProvenanceRecord` / `SourceMapEntry` | Import, explanation, and parity claims need source lineage. |

## Required boundary artifact
GE-06 MUST preserve the GE-02 `CompiledRuntimeIR` boundary rule:
- authored source-package content remains the canonical authority
- runtime IR is derived and must be traceable back to source content

GE-06 must consume this boundary. It must not redefine it.

## Pilot minimum interpretation
For the first integrated proof, GE-06 assumes the GE-02 pilot minimum object set is still correct:
- PF1 Core Rulebook source package and include graph
- Human race and race-trait composition
- Fighter class and class-feature carriers
- selected feats, skills, equipment, saves, and ability-score surfaces
- effects, prerequisites, formulas, choices, diagnostics, and provenance for touched pilot behavior

If the pilot requires a semantic object not on this list, the correct next move is an upstream GE-02 review rather than local invention.
