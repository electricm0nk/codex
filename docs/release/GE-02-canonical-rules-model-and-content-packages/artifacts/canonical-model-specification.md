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

## Required field: `wiring_class` on every imported rule record

Added 2026-08-02. Every canonical object imported from a legacy record — every `Race`, `RaceTrait`, `Class`, `ClassFeature`, `Feat`, `Skill`, `Equipment`, and spell record — MUST carry two fields:

| Field | Required meaning |
|---|---|
| `wiring_class` | Exactly one of `display`, `static`, `derived`, `computed`, `ambiguous`. States what kind of evidence would prove this record done. |
| `wiring_class_signals` | The full, ordered set of signals the source record carried, e.g. `["derived:bonus", "computed:pre_guard"]`. Never collapsed away. |
| `upstream_implementation_marker` | Whether the legacy record carries an upstream not-implemented admission (PCGen writes `[Not Implemented]` into `DESC:`), plus the marker text. |

**The definitions and the determination rules are NOT restated here.** They are owned by GE-01 and live in `../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.md`. Duplicating them would create two authorities that drift.

**Determination reads the record's token closure, not one row.** A `<Name>.MOD` row modifies a base record rather than declaring one, so it produces no unit of its own — but it can carry the record's only magnitude. 8,234 `.MOD` rows corpus-wide carry a magnitude token, and 1,895 of the 9,828 held units have at least one `.MOD` row targeting them. The class MUST therefore be determined from the base row **plus every `.MOD` row targeting it**, per GE-01's token-closure rule. An importer that classifies from the base row alone will model records with real magnitudes as text-only.

**`upstream_implementation_marker` MUST NOT feed `wiring_class`, in either direction.** What PCGen did or did not implement upstream is a different claim from what evidence would prove *our* record done. `ultimate_campaign`'s *Accursed* is marked `[Not Implemented]` and still carries a fully specified benefit formula; conversely, a record must never be modelled as complete on the strength of a `[Not Implemented]` description alone. The two fields are reported side by side and never merged.

**Why the field belongs on the canonical model rather than only in a report.** The class is derived from the legacy record at import time and is a durable property of the imported object, not a view over it. A record whose magnitude is `(min(10,CASTERLEVEL))d6` requires different downstream treatment — a `Formula` object and an evaluation, rather than an `Effect` with bespoke engine logic — and that requirement must survive from import into the model, not be re-derived by every consumer.

**Relationship to the existing model homes.** `wiring_class` is a *classification of how a record's magnitude behaves*, not a new kind of magnitude. It does not replace `Formula`, `Effect`, `Prerequisite`, or `ChoiceSet`; it predicts which of them a record will need:

| `wiring_class` | expected canonical shape |
|---|---|
| `display` | no `Formula`, no numeric `Effect`; description text only |
| `static` | a literal-valued field or `Effect`; no `Formula` |
| `derived` | a `Formula` over declared dependencies, evaluated by GE-04 |
| `computed` | a `Formula` and/or `Effect` guarded by a `Prerequisite`, or driven by a `ChoiceSet` |
| `ambiguous` | a `Diagnostic`, mandatory — the record could not be classified and must not be silently modelled as one of the above |

**`ambiguous` MUST mint a `Diagnostic`.** This is the model-level statement of GE-01's no-silent-default rule: a determination failure is a first-class unresolved construct, exactly like an unsupported token, and is subject to the same "unsupported behavior must not disappear into prose" failure-mode signal already governing this bundle.

**`wiring_class_signals` is not decoration.** 470 of the 9,828 currently-held corpus units carry both a `derived` and a `computed` signal (re-derive with `python3 docs/release/GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.py HELD`, `dual-signal` line). `wiring_class` collapses those to `computed` by highest-bar-wins; without the retained signal set, a record with a formulaic main effect and a guarded rider is indistinguishable from one that is wholly bespoke, and neither the model nor GE-09's audit can tell them apart.

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
