---
title: GE-02 Canonical Model Specification
stc_id: STC-CODEX-GE-02
artifact_type: generated-documentary-output
status: accepted
scope: programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts
source_stc: ../README.md
source_inputs:
  - ../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-corpus-inventory.csv
  - ../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-token-taxonomy.csv
  - ../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv
  - ../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv
---

# GE-02 Canonical Model Specification

## Purpose
Define the concrete documentary model homes GE-02 requires before importer, engine, or UI work can claim a canonical target.

This is a GE-02 output artifact. It is not merely a requirement for the STC to inspect itself.

## Grounded source basis
- GE-01 pilot corpus inventory rows: 66
- GE-01 pilot token taxonomy rows: 26
- GE-01 conversion matrix rows: 29
- GE-01 unsupported-token ledger rows: 13

## Non-negotiable model rule
Codex canonical content MUST be semantic. It MUST NOT mirror PCGen LST token syntax as the final authored content model.

Raw PCGen token text may appear only as:
- parser input
- provenance/source-map evidence
- unresolved expression text pending conversion
- diagnostic context

## Canonical model homes

| Model home | Required purpose | GE-01 pressure |
|---|---|---|
| `SourcePackage` | Versioned source package identity, game system, source/book, dependency/include graph, validation state. | PCC root and include directives from `core_rulebook.pcc`. |
| `StableId` | Deterministic package/object identity independent of absolute local paths. | Imported object names and paths need durable aliases, not path-based identity. |
| `Race` | Race identity, display name, lineage, trait composition references. | Human `RACE` entry. |
| `RaceTrait` | Human trait carrier/composition rows, default traits, replacement gates. | Human ability carrier and trait replacement ledger entries. |
| `Class` | Fighter class identity, level progression, BAB/save/skill budget surface. | Fighter `CLASS` row and formula-bearing class entries. |
| `ClassFeature` | Named class grants and feature carriers separate from class identity. | Fighter proficiencies and class-skill carrier rows. |
| `Feat` | Selectable feat identity with prerequisites, effects, and choice sets. | Martial Weapon Proficiency and Power Attack-adjacent feat surfaces. |
| `Skill` | Skill identity, key ability, armor-check posture, class-skill interaction. | `SKILL`, `CSKILL`, and class-skill package surfaces. |
| `Equipment` | Armor/weapon/item identity, fields, proficiency references, and mechanical effects. | Chain Shirt and Longsword rows. |
| `Proficiency` | Weapon/armor/shield proficiency catalogs and grouped selectors. | `WEAPONPROF / ARMORPROF / SHIELDPROF`. |
| `AbilityScore` | Base stat definitions and derived variable/formula hooks. | `cr__stats.lst` formula/variable ledger entry. |
| `Save` | Save identity and save-to-stat bindings. | `cr__saves.lst` and Fighter save progression. |
| `Effect` | Automatic grants, numeric modifiers, trait grants/removals, equipment modifiers. | `ABILITY`, `AUTO`, `BONUS`, `CSKILL`, trait/equipment rows. |
| `Prerequisite` | Structured predicates and eligibility gates. | `PRE*`, `PREMULT`, `PREPROFWITH*`, `PREFACT`. |
| `Formula` | Symbolic value expressions and derived-value definitions. | `BONUS`, `DEFINE`, `VAR`, base-stat and skill-point formulas. |
| `ChoiceSet` | Selectable/repeatable option definitions with selectors/cardinality. | `CHOOSE + MULT` Martial Weapon Proficiency. |
| `Selector` | Type/category/group selection distinct from legacy raw `TYPE`. | `TYPE=Craft`, proficiency groups, equipment categories. |
| `Diagnostic` | Machine/human-readable unresolved, lossy, unsupported, deferred, or invalid behavior. | Unsupported-token ledger and conversion matrix dispositions. |
| `ProvenanceRecord` / `SourceMapEntry` | Lineage from package/file/entry/span/token to canonical object/effect/field. | Matrix provenance requirements and oracle surface inventory. |
| `CompiledRuntimeIR` | Derived runtime cache/IR boundary, not source-of-truth authored content. | GE-02 authoring-vs-runtime boundary requirement. |

## Required relationships
- `SourcePackage` owns many canonical objects.
- `StableId` identifies every package object and major rule record.
- `Race`, `Class`, `Feat`, `Skill`, `Equipment`, `Proficiency`, `AbilityScore`, and `Save` are semantic objects.
- `Effect`, `Prerequisite`, `Formula`, `ChoiceSet`, and `Selector` attach to semantic objects but remain first-class model records.
- `Diagnostic` and `ProvenanceRecord` attach to imported objects, effects, fields, formulas, prerequisites, and choices.
- `CompiledRuntimeIR` is derived from validated source-package content and must be traceable back to the source content.

## Pilot minimum object set
For the first pilot, GE-02 requires enough model coverage to represent:
- source package: PF1 Core Rulebook package and include graph
- race: Human
- class: Fighter level 1
- ability scores: six PF1 ability scores plus stat formula/value hooks
- saves: Fortitude, Reflex, Will and stat/progression bindings
- skills: representative Fighter-relevant skills including class-skill behavior
- feats: at least one selected feat path and adjacent Martial Weapon Proficiency choice debt
- equipment: representative armor and weapon with proficiency references
- effects: Fighter proficiencies, class-skill grants, race traits, equipment effects, BAB/save/skill effects
- prerequisites: feat/proficiency and trait/condition gates where discovered
- formulas: Fighter progression, base-stat, and skill-point formulas carried as structured values or explicit deferred diagnostics
- provenance and diagnostics for every imported or deferred construct

## What this artifact does not decide
- final production schema syntax
- final expression/evaluator technology
- final engine execution semantics
- full Pathfinder object coverage
- public registry or plugin ABI
