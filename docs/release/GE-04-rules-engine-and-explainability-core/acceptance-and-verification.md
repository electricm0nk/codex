---
title: GE-04 Acceptance and Verification
stc_id: STC-CODEX-GE-04
artifact_type: acceptance-and-verification
status: draft
scope: programs/codex/requirements/GE-04-rules-engine-and-explainability-core
source_stc: ./README.md
---

# GE-04 Acceptance and Verification

These checks prove that the Codex GE-04 source STC is complete enough to govern later rules-engine planning and to block premature implementation authority.

## AT-04-001 — Source STC bundle completeness
**Given** the canonical Codex GE-04 source STC directory  
**When** the bundle is reviewed  
**Then** it contains `README.md`, `technical-requirements.md`, `technical-design.md`, `acceptance-and-verification.md`, `risks-and-open-questions.md`, and `epic-breakdown.md`.

Evidence:
- source-STC directory listing or direct file reads
- parent index link from `programs/codex/requirements/README.md`

## AT-04-002 — Same-epic documentary outputs exist
**Given** the GE-04 spec domain required outputs  
**When** the generated artifact set is inspected  
**Then** the bundle contains `artifacts/rules-engine-technical-specification.md`, `artifacts/evaluation-order-definition.md`, `artifacts/expression-language-runtime-requirements.md`, `artifacts/explanation-graph-schema.md`, `artifacts/diagnostic-schema.md`, and `artifacts/pilot-golden-computation-fixture-requirements.md`.

Evidence:
- `README.md` Document Map
- generated artifact files under `artifacts/`

## AT-04-003 — Rules-core boundary truth
**Given** the README and technical requirements  
**When** the GE-04 posture is inspected  
**Then** the rules core is described as a headless computation and explanation subsystem, with GE-02, GE-03, GE-05, and GE-06 boundaries explicit, and no part of the STC grants code authority.

Evidence:
- `README.md` Authority and Scope, Target Runtime, and Out of Scope sections
- `technical-requirements.md` sections TR-04-001 and TR-04-002

## AT-04-004 — Character input and validation preconditions exist
**Given** the technical requirements and generated artifacts  
**When** the engine input posture is reviewed  
**Then** character input, source-package validation preconditions, imported diagnostics, provenance obligations, and invalid-input behavior are explicit.

Evidence:
- `technical-requirements.md` sections TR-04-003 and TR-04-004
- `artifacts/rules-engine-technical-specification.md`
- `artifacts/pilot-golden-computation-fixture-requirements.md`

## AT-04-005 — Evaluation order and effect pipeline are explicit
**Given** the technical requirements, design, and evaluation-order artifact  
**When** computation flow is reviewed  
**Then** the STC defines a pilot evaluation order, effect collection/activation pipeline, dependency discovery posture, and diagnostic behavior for unresolved or unstable evaluation.

Evidence:
- `technical-requirements.md` sections TR-04-005 and TR-04-006
- `technical-design.md` High-level pipeline and Component boundaries
- `artifacts/evaluation-order-definition.md`

## AT-04-006 — Formula and prerequisite requirements preserve explainability
**Given** the technical requirements and expression-language artifact  
**When** formula/prerequisite evaluation is reviewed  
**Then** formulas and prerequisites are deterministic, sandboxed, dependency-capturing, provenance-aware, and diagnostic-rich, with failed prerequisites expressed as expected-versus-actual outcomes.

Evidence:
- `technical-requirements.md` sections TR-04-007 and TR-04-008
- `artifacts/expression-language-runtime-requirements.md`

## AT-04-007 — Choice availability is headless and explainable
**Given** the technical requirements and design  
**When** choice availability is reviewed  
**Then** the STC requires candidate choices, filters, prerequisites, repeatability constraints, unavailable-choice explanations, and diagnostics without relying on UI state.

Evidence:
- `technical-requirements.md` section TR-04-009
- `technical-design.md` Choice availability engine

## AT-04-008 — Derived outputs have explanation and diagnostic obligations
**Given** the technical requirements and generated artifacts  
**When** pilot derived values are reviewed  
**Then** output categories are named, each tested derived value must have source-contribution explanation obligations, and diagnostics block overstated claims.

Evidence:
- `technical-requirements.md` sections TR-04-010, TR-04-011, and TR-04-012
- `artifacts/explanation-graph-schema.md`
- `artifacts/diagnostic-schema.md`

## AT-04-009 — Headless verification path is required
**Given** the GE-04 acceptance criteria  
**When** a later implementation handoff is proposed  
**Then** the handoff must include tests and/or CLI commands that load deterministic fixtures, compute outputs without UI, emit explanations and diagnostics, and fail on missing expected values or explanation edges.

Evidence:
- `technical-requirements.md` section TR-04-013
- `artifacts/pilot-golden-computation-fixture-requirements.md`
- `risks-and-open-questions.md` forbidden assumptions

## AT-04-010 — Downstream work is decomposed but not authorized
**Given** the epic breakdown  
**When** later work is routed  
**Then** the STC decomposes GE-04 into bounded downstream epics for character input, effect core, formula/prerequisite evaluator, choice availability, diagnostics, explanation graph, CLI/test entry point, and pilot fixtures without becoming an execution prompt.

Evidence:
- `epic-breakdown.md`
- `README.md` Next Stage Rule

## AT-04-011 — Future runtime facts are not fabricated
**Given** the canonical README  
**When** target-runtime fields are reviewed  
**Then** known local runtime facts are named, unresolved GE-04 execution branch/write-scope/verification facts remain explicit, and coding handoff derivation is blocked until a bounded slice is selected.

Evidence:
- `README.md` Target Runtime
- `README.md` Blockers / Forbidden Assumptions

## AT-04-012 — GE04-E1-F1 execution-readiness closure is honored
**Given** a future attempt to derive `execution-handoff.md` for the first GE-04 implementation slice  
**When** GE04-E1-F1 readiness is reviewed  
**Then** the reviewer must consult `artifacts/ge04-e1-f1-execution-readiness-closure-2026-06-20.md` and `execution-handoff.md`, and refuse any broader or differently based coding run unless it branches from clean current `develop` with the exact declared write scope.

Evidence:
- `artifacts/ge04-e1-f1-execution-readiness-closure-2026-06-20.md`
- `epic-breakdown.md` GE04-E1-F1 execution-readiness status
- `risks-and-open-questions.md` OQ-04-007 and OQ-04-008

## Exit gate checklist
- [ ] Full rich source-STC bundle exists.
- [ ] Parent requirements index links to GE-04.
- [ ] Same-epic generated documentary artifact set exists.
- [ ] GE-02/GE-03/GE-05/GE-06 boundaries are explicit.
- [ ] Character input and validation preconditions are explicit.
- [ ] Effect evaluation pipeline and evaluation-order posture are explicit.
- [ ] Formula and prerequisite requirements preserve deterministic, sandboxed, explainable behavior.
- [ ] Choice availability requirements are headless and explainable.
- [ ] Explanation graph schema exists.
- [ ] Rules diagnostic schema exists.
- [ ] Pilot golden computation fixture requirements exist without fabricated final values.
- [ ] Downstream epic routing exists.
- [ ] No code-authorizing implementation handoff has been derived prematurely.
- [ ] GE04-E1-F1 execution-readiness closure is consulted before creating any GE-04 `execution-handoff.md`.
