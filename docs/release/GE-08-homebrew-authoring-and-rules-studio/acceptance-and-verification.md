---
title: GE-08 Acceptance and Verification
stc_id: STC-CODEX-GE-08
artifact_type: acceptance-and-verification
status: draft
scope: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio
source_stc: ./README.md
---

# GE-08 Acceptance and Verification

## Objective
Define the observable checks that make the GE-08 source STC planning-ready without pretending that authoring implementation or final UX already exists.

## Acceptance posture
This file verifies the documentary/planning boundary for GE-08. It does not verify implementation code. It verifies that the GE-08 packet is concrete enough to drive an honest later readiness closure.

## Acceptance criteria

### AC-08-001 — Control bundle completeness
The GE-08 source STC is acceptable only if the following files exist and remain internally coherent:
- `README.md`
- `technical-requirements.md`
- `technical-design.md`
- `acceptance-and-verification.md`
- `risks-and-open-questions.md`
- `epic-breakdown.md`
- `references/upstream-dependency-contract.md`

Verification:
- all files exist at the canonical GE-08 path
- `README.md` frontmatter names GE-08 as canonical source STC
- supporting files point back to `README.md` as `source_stc`

### AC-08-002 — Required output artifact contract exists materially
The GE-08 packet is acceptable only if the named same-epic documentary outputs exist materially, not merely as a promise in prose.

Required artifacts:
- `artifacts/homebrew-authoring-surface-specification.md`
- `artifacts/rules-studio-surface-definition.md`
- `artifacts/validation-and-preview-workflow-requirements.md`
- `artifacts/safe-expression-authoring-constraints.md`
- `artifacts/package-file-lifecycle-requirements.md`
- `artifacts/initial-homebrew-acceptance-cases.md`
- `artifacts/plugin-exception-boundary.md`

Verification:
- each artifact exists
- each artifact is linked from `README.md`
- each artifact expresses a concrete completion rule or bounded requirement surface

### AC-08-003 — Ordinary homebrew is defined without LST or plugins
The GE-08 packet is acceptable only if it defines ordinary homebrew as a structured content problem rather than an LST or plugin escape hatch.

Verification:
- `technical-requirements.md` names structured editing as a required ordinary path
- `technical-requirements.md` forbids arbitrary scripting as the default answer
- `artifacts/plugin-exception-boundary.md` makes plugins exceptional rather than routine

### AC-08-004 — Validation and preview are first-class requirements
The GE-08 packet is acceptable only if authoring is tied to validation, compile-preview, and explanation-preview duties.

Verification:
- `technical-requirements.md` includes explicit validation requirements
- `artifacts/validation-and-preview-workflow-requirements.md` defines the edit -> validate -> preview -> explain loop
- `technical-requirements.md` and `technical-design.md` both keep downstream GE-04 explanation/diagnostic truth visible

### AC-08-005 — Upstream authority boundaries remain intact
The GE-08 packet is acceptable only if it consumes GE-02, GE-04, GE-06, and GE-07 truth without counterfeit authority capture.

Verification:
- `references/upstream-dependency-contract.md` names what each upstream surface authorizes and does not authorize
- `README.md` names GE-07 as a planning-ready source STC while still refusing to treat it as GE-08 code authority for final editor/UI decisions
- GE-08 does not claim final compute semantics, pilot viability, or UI architecture ownership

### AC-08-006 — Initial proof cases are concrete
The GE-08 packet is acceptable only if the first homebrew proof cases are concrete enough for a later bounded implementation handoff to test without guessing.

Verification:
- `artifacts/ge08-e1-minimum-proof-object-selection-2026-06-22.md` fixes the first proof object, its GE-06-derived pilot variant, and its GE-02/GE-04 obligations concretely
- `artifacts/initial-homebrew-acceptance-cases.md` names bounded cases, expected authoring value, and anti-scope-creep rules
- `technical-requirements.md` requires validation-negative and preview/explanation cases in addition to a happy path

## Current verification approach
For this planning-stage packet, verification is documentary and structural:
1. verify file existence and canonical paths
2. verify internal link and source-STC coherence
3. verify that required boundaries are named explicitly
4. verify that open questions are quarantined rather than buried
5. verify that no code-authorizing handoff is created prematurely

## Future implementation proof obligations
A future GE-08 readiness closure or execution handoff must add:
- exact repo paths and write scope
- exact proof object under test
- exact verification commands
- expected receipts/artifacts
- explicit non-goals
- dependency baseline against GE-06/GE-07 truth at that time
