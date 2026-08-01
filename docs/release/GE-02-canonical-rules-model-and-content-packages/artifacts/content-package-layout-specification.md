---
title: GE-02 Content Package Layout Specification
stc_id: STC-CODEX-GE-02
artifact_type: generated-documentary-output
status: accepted
scope: programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts
source_stc: ../README.md
---

# GE-02 Content Package Layout Specification

## Purpose
Prescribe the required documentary content-package layout for GE-02 so future import and authoring work aims at a concrete package substrate rather than a generic “canonical model” phrase.

## Package identity requirements
A Codex source package MUST carry:
- stable package ID
- package title
- game system ID
- package version or source revision
- source/book identity when applicable
- dependency/include edges
- supported object kinds
- validation state
- diagnostics summary
- provenance policy

## Required source package sections
A future authored/imported source package SHOULD be decomposable into these logical sections, even if implementation later chooses different physical files:

| Section | Purpose | Pilot requirement |
|---|---|---|
| `manifest` | Package identity, version, game system, dependencies/includes. | Represent Core Rulebook package and GE-01 PCC include graph. |
| `objects/races` | Race records and race-trait references. | Human race record. |
| `objects/classes` | Class records, levels, features, progression hooks. | Fighter level-1 class record and grants. |
| `objects/feats` | Feat records, prerequisites, choice sets, effects. | Pilot feat path plus Martial Weapon Proficiency choice debt. |
| `objects/skills` | Skill definitions and skill metadata. | Representative Fighter-relevant skills. |
| `objects/equipment` | Armor, weapon, and item records. | Chain Shirt and Longsword representative rows. |
| `objects/proficiencies` | Weapon/armor/shield proficiency catalogs. | Longsword/martial and armor/shield proficiency references. |
| `rules/effects` | Effect/grant records. | Fighter proficiencies, class skills, race traits, equipment effects. |
| `rules/prerequisites` | Structured eligibility predicates. | `PRE*`, `PREMULT`, proficiency and trait gates. |
| `rules/formulas` | Symbolic value expressions. | BAB, saves, stats, skill points, equipment values. |
| `rules/choices` | Choice sets and repeatability/cardinality. | `CHOOSE + MULT` proficiency choice debt. |
| `metadata/provenance` | Source maps and lineage records. | File/entry/span/token lineage for imported constructs. |
| `metadata/diagnostics` | Unsupported/lossy/deferred behavior. | Ledger-linked diagnostics. |

## Layout rule
The physical implementation MAY use YAML, JSON, TOML, directories, a database, or generated code later, but the package layout MUST preserve the logical separation above.

The package MUST NOT collapse into:
- one raw imported LST dump
- one opaque object table
- a runtime-only cache with no authored source representation

## Validation requirements
Before a package can be trusted, validators MUST check:
- manifest identity and dependency graph validity
- stable ID uniqueness
- object kind validity
- required fields for pilot object homes
- reference resolution across effects, prerequisites, formulas, choices, and selectors
- provenance presence for imported content
- diagnostics for unsupported/lossy/deferred behavior
- authoring-source versus compiled-IR separation

## Pilot package sketch
```text
package pf1.crb
  manifest
  objects
    races: Human
    classes: Fighter
    feats: selected pilot feat(s), Martial Weapon Proficiency as choice debt
    skills: representative Fighter skill targets
    equipment: Chain Shirt, Longsword representative objects
    proficiencies: weapon/armor/shield proficiency records
    stats: Strength, Dexterity, Constitution, Intelligence, Wisdom, Charisma
    saves: Fortitude, Reflex, Will
  rules
    effects: grants, modifiers, trait/equipment effects
    prerequisites: structured/deferred gates
    formulas: structured/deferred value expressions
    choices: repeatable/selectable options
  metadata
    provenance: source-map records
    diagnostics: unsupported/lossy/deferred records
  compiled_ir
    derived only; not source of truth
```
