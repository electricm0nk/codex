---
title: GE-05 Golden-Case Fixture Format
stc_id: STC-CODEX-GE-05
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts
source_stc: ../README.md
related:
  - ../../../plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md
  - ../../GE-04-rules-engine-and-explainability-core/artifacts/pilot-golden-computation-fixture-requirements.md
---

# GE-05 Golden-Case Fixture Format

## Purpose
Define the fixture shape required to bind a PCGen oracle output and a Codex output into one reproducible comparison case.

This artifact defines the format. It does not populate final expected values.

## Required fixture fields
A golden-case fixture MUST be able to represent:

| Field | Purpose |
|---|---|
| `case_id` | Stable case identifier, e.g. `pf1-crb-human-fighter-level1`. |
| `case_version` | Version of the fixture contract or case data. |
| `scope` | Human-readable scope statement and out-of-scope boundary. |
| `source_package` | Game system, source package, campaign, or corpus identity. |
| `character_input` | Race, class/level, ability scores, feat choices, skill/equipment state, and selected choices. |
| `legacy_oracle` | PCGen route, raw output reference, capture metadata, and trust tier. |
| `codex_output` | Codex route, raw output reference, diagnostics, provenance, and explanation references. |
| `dimensions` | Compared output dimensions and pass/fail/blocked rules. |
| `normalization` | Explicit normalization rules and raw-value retention requirements. |
| `known_gaps` | Links to non-comparable, unsupported, divergent, or blocked dimensions. |
| `claim_target` | Desired claim tier, usually `Oracle-checked` for exact scoped dimensions. |

## Conceptual fixture skeleton

```yaml
case_id: pf1-crb-human-fighter-level1
case_version: 0
scope:
  statement: Pathfinder 1e Core Rulebook Human Fighter level 1 pilot comparison
  out_of_scope:
    - broad Pathfinder support
    - full export-sheet parity
    - all feats, equipment, archetypes, or classes
source_package:
  system: pathfinder-1e
  package: core_rulebook
character_input:
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
  skills: unresolved-until-fixture-grounded
  equipment: unresolved-until-fixture-grounded
legacy_oracle:
  route: unresolved-pending-ge05-e1
  raw_output_ref: null
  trust_tier: unknown
codex_output:
  route: unresolved-pending-ge03-ge04-implementation
  raw_output_ref: null
  requires:
    - diagnostics
    - provenance_or_source_map
    - explanation_references
normalization:
  rules_ref: unresolved-pending-ge05-e3
dimensions:
  - id: loaded_content_summary
    status: candidate
  - id: derived_values
    status: candidate
  - id: choice_or_prerequisite_outcome
    status: candidate
  - id: exportable_summary
    status: candidate_if_oracle_route_exists
known_gaps: []
claim_target: oracle-checked
```

## Fixture rules
- The fixture MAY carry inherited pilot inputs from the pilot charter.
- The fixture MUST NOT invent old-system or new-system expected values.
- Every compared dimension MUST name both old and new evidence sources before it can pass.
- Every non-comparable dimension MUST link to a known-gap entry.
- Raw output references MUST remain auditable after normalization.
- The fixture MUST be narrow enough for the first pilot; broader regression belongs to later expansion governance.

## Completion rule
The format is complete when it can represent the first Human Fighter case, unresolved oracle/new-system outputs, normalization rules, known gaps, and eventual evidence references without falsifying readiness.
