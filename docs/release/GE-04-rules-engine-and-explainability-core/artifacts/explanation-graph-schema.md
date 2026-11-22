---
title: GE-04 Explanation Graph Schema
stc_id: STC-CODEX-GE-04
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-04-rules-engine-and-explainability-core/artifacts
source_stc: ../README.md
---

# GE-04 Explanation Graph Schema

## Purpose
Define the conceptual explanation graph schema required before Codex can claim that computed values, unavailable choices, or failed prerequisites are explainable.

This is not final serialized schema authority. It is the minimum semantic contract a later implementation must satisfy.

## Required node kinds

| Node kind | Meaning |
|---|---|
| `character_input` | A user/fixture-provided selection, ability score, equipment state, level, feat, skill, or other input. |
| `source_package` | Canonical package or dependency graph source. |
| `canonical_object` | Race, class, feat, skill, equipment, proficiency, save, ability, or similar semantic object. |
| `effect` | Rule contribution or modifier attached to a canonical object or selected state. |
| `formula` | Evaluated expression used to compute a value or prerequisite. |
| `prerequisite` | Eligibility or activation condition. |
| `choice_set` | Selectable option set or selector. |
| `choice_option` | Candidate or selected option. |
| `derived_value` | Computed output such as save, attack value, armor/equipment-influenced value, skill-related value, or prerequisite outcome. |
| `diagnostic` | Structured issue affecting computation or claim level. |
| `provenance` | Source-map or lineage record for imported/canonical content. |

## Required edge kinds

| Edge kind | Meaning |
|---|---|
| `depends_on` | A formula, prerequisite, effect, or value consulted another value/object/input. |
| `contributes_to` | An effect, formula, input, or object contributes to a derived value. |
| `grants` | A canonical object or effect grants another object, effect, proficiency, feature, or option. |
| `modifies` | An effect modifies a value or calculation context. |
| `checks` | A prerequisite or selector checks an input/value/object. |
| `blocks` | A failed prerequisite, invalid input, diagnostic, or unresolved dependency blocks a choice/effect/value/claim. |
| `selects` | Character input selects a choice option or object. |
| `sourced_from` | A canonical object/effect/formula/diagnostic traces to provenance/source-map evidence. |
| `diagnoses` | A diagnostic attaches to the thing it explains or blocks. |

## Minimum graph obligations
For every tested derived value, the graph MUST support a path from character input plus canonical objects/effects/formulas to derived value.

For every tested failed prerequisite or unavailable choice, the graph MUST support a path from choice/prerequisite to checked expected condition to actual observed state or diagnostic to block reason.

For imported content affecting behavior, the graph SHOULD support a path from derived value or blocked choice to effect/prerequisite/formula to canonical object to provenance/source-map record.

## Required graph record fields
A future implementation SHOULD be able to represent graph nodes with `id`, `kind`, `label`, `ref`, optional `value`, optional `status`, and `metadata`.

Edges SHOULD be able to represent `from`, `to`, `kind`, optional `reason`, and `metadata`.

## Granularity rule
The graph must be granular enough for tests to assert that an output is explained by its real contributors. It must not collapse all effects into one generic “rules engine calculated this” node.

## UI boundary
GE-07 may later render or transform explanation graph output, but GE-04 owns the headless explanation data contract. UI rendering is not proof that the graph exists.
