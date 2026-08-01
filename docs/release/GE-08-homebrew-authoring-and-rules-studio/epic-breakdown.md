---
title: GE-08 Epic Breakdown
stc_id: STC-CODEX-GE-08
artifact_type: epic-breakdown
status: draft
scope: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio
source_stc: ./README.md
---

# GE-08 Epic Breakdown

## Objective
Decompose GE-08 into future bounded work without turning the source STC into a code prompt.

## Downstream epics / slices

### GE08-E1 — Minimum homebrew proof object selection and fixture closure
Purpose:
- choose the narrowest first authoring case
- bind it to GE-02 object homes and GE-04/GE-06 preview obligations

Current closure:
- fixed by `artifacts/ge08-e1-minimum-proof-object-selection-2026-06-22.md` as a package-local feat-like authored object substituting for the GE-06 Human bonus feat `Dodge` in one bounded pilot variant

Likely route first:
- Hermes planning/review/readiness closure

### GE08-E2 — Authored package schema and lifecycle slice
Purpose:
- implement the bounded package create/edit/save/load/diff/export substrate for the first proof object

Preconditions:
- exact repo paths
- exact proof object
- explicit serialization and validation boundaries

### GE08-E3 — Validation and diagnostics slice
Purpose:
- implement structural validation and actionable diagnostics for the first proof object
- preserve machine-readable diagnostics and claim-blocking posture

### GE08-E4 — Preview and explanation bridge slice
Purpose:
- connect authored content to bounded preview/explanation outputs grounded in GE-04 and GE-06 truth

Constraint:
- remain headless unless a grounded GE-07 authority surface exists

### GE08-E5 — Product-visible editor/workbench slice
Purpose:
- implement editor-facing or workbench-facing surfaces only after GE-07 exists as a source STC

Constraint:
- must not own rules semantics
- must cite GE-07 and GE-08 authority surfaces together

### GE08-E6 — Plugin exception research / ADR lane
Purpose:
- evaluate genuine exception cases that the ordinary path cannot safely express

Constraint:
- research/review first; no plugin runtime implementation by implication alone

## Routing rule
No item above is code-authorizing by itself.

A future code-producing handoff must:
- identify the exact slice
- state exact repo paths and write scope
- state whether the slice is headless-only or UI-facing
- cite the exact proof artifact(s) and verification commands
- preserve plugin-exception posture
