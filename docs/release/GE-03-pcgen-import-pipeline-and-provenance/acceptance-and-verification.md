---
title: GE-03 Acceptance and Verification
stc_id: STC-CODEX-GE-03
artifact_type: acceptance-and-verification
status: draft
scope: programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance
source_stc: ./README.md
---

# GE-03 Acceptance and Verification

These checks prove that the Codex GE-03 source STC is complete enough to govern later importer planning and to block premature implementation authority.

## AT-03-001 — Source STC bundle completeness
**Given** the canonical Codex GE-03 source STC directory  
**When** the bundle is reviewed  
**Then** it contains `README.md`, `technical-requirements.md`, `technical-design.md`, `acceptance-and-verification.md`, `risks-and-open-questions.md`, and `epic-breakdown.md`.

Evidence:
- source-STC directory listing or direct file reads
- parent index link from `programs/codex/requirements/README.md`

## AT-03-002 — Importer boundary truth
**Given** the GE-03 README and technical requirements  
**When** the STC is inspected  
**Then** the importer is described as a compatibility bridge, the canonical-model dependency on accepted GE-02 artifacts is explicit, and no part of the STC treats this planning bundle as code authority.

Evidence:
- `README.md`
- `technical-requirements.md` sections TR-03-001 and TR-03-002

## AT-03-003 — Parser-stage requirements exist
**Given** the technical requirements and design  
**When** parse-stage obligations are reviewed  
**Then** the STC defines PCC/LST parsing requirements, structured representation requirements, and a separation between parsing and semantic conversion.

Evidence:
- `technical-requirements.md` sections TR-03-004 and TR-03-005
- `technical-design.md` PCC Parser Boundary, LST Parser Boundary, and Structured Parse Representation sections

## AT-03-004 — Registry and handler boundaries are explicit
**Given** the technical requirements and design  
**When** semantic handling is reviewed  
**Then** the STC defines token-registry and conversion-handler boundaries, including risk, validation, and dependency-on-GE-02 posture.

Evidence:
- `technical-requirements.md` sections TR-03-006 and TR-03-007
- `technical-design.md` Token Registry and Conversion Handlers sections

## AT-03-005 — Provenance and source-map obligations are explicit
**Given** the technical requirements and design  
**When** provenance expectations are reviewed  
**Then** the STC requires preserved lineage from source files and constructs through handler identity and canonical outcomes, with an explicit downgrade path if full source precision is unavailable.

Evidence:
- `technical-requirements.md` section TR-03-008
- `technical-design.md` Provenance / Source-Map Contract and schema notes

## AT-03-006 — Unsupported behavior cannot disappear silently
**Given** the technical requirements, design, and risks file  
**When** unsupported or lossy semantics are reviewed  
**Then** the STC requires explicit unsupported-token diagnostics and forbids silent dropping of unresolved behavior.

Evidence:
- `technical-requirements.md` section TR-03-009
- `technical-design.md` Diagnostics / Reporting Surface section
- `risks-and-open-questions.md` primary risks table

## AT-03-007 — Conversion reporting and fixture posture exist
**Given** the technical requirements and design  
**When** coverage and validation posture are reviewed  
**Then** the STC defines conversion-report expectations, fixture-driven verification requirements, and room for later oracle-backed comparison without claiming current automation success.

Evidence:
- `technical-requirements.md` sections TR-03-010 and TR-03-011
- `technical-design.md` Diagnostics / Reporting Surface section

## AT-03-008 — GE-02 artifact usage is recorded honestly
**Given** the canonical README, technical requirements, design, and risks file  
**When** the GE-02 dependency posture is reviewed  
**Then** the STC cites the accepted GE-02 source STC and generated artifacts for canonical target-model inputs without treating those planning artifacts as final schema, runtime, evaluator, or code authority.

Evidence:
- `README.md` Readiness and Required Reads sections
- `technical-requirements.md` sections TR-03-002 and TR-03-013
- `technical-design.md` Context and constraints plus External dependencies and references
- `risks-and-open-questions.md` remaining deferred model/runtime questions

## AT-03-009 — Downstream implementation work is bounded
**Given** the epic breakdown  
**When** later work is routed  
**Then** the STC decomposes importer work into bounded downstream epics such as PCC parser, LST parser, token registry, token handlers, source-map/provenance, report CLI, and fixture/parity work without becoming an execution prompt.

Evidence:
- `epic-breakdown.md`
- `README.md` next-stage rule

## AT-03-010 — Future runtime facts are not fabricated
**Given** the canonical README  
**When** target-runtime fields are reviewed  
**Then** known facts are named, unknown execution facts remain explicit, and code-authorizing handoff derivation is blocked until a later bounded implementation slice grounds repo/workdir/branch/write-scope/verification details.

Evidence:
- `README.md` Target Runtime section
- `README.md` Blockers / Forbidden Assumptions section

## AT-03-011 — GE03-E1-F1 execution-readiness closure exists
**Given** a future attempt to derive `execution-handoff.md` for the first GE-03 implementation slice  
**When** GE03-E1-F1 readiness is reviewed  
**Then** the reviewer must consult `artifacts/ge03-e1-f1-execution-readiness-closure-2026-06-19.md` and preserve its branch policy, allowed write scope, verification commands, stop conditions, and non-goals in the derived handoff.

Evidence:
- `artifacts/ge03-e1-f1-execution-readiness-closure-2026-06-19.md`
- `epic-breakdown.md` GE03-E1-F1 execution-readiness status
- `risks-and-open-questions.md` OQ-03-007

## Exit gate checklist
- [ ] Full rich source-STC bundle exists.
- [ ] Parent requirements index links to GE-03.
- [ ] Importer boundary and accepted GE-02 artifact usage are explicit.
- [ ] Parser-stage requirements are explicit.
- [ ] Structured parse representation requirements are explicit.
- [ ] Token-registry and handler boundaries are explicit.
- [ ] Provenance/source-map obligations are explicit.
- [ ] Unsupported-token diagnostics and report posture are explicit.
- [ ] Downstream epic routing exists.
- [ ] No code-authorizing implementation handoff has been derived prematurely.
- [ ] GE03-E1-F1 `execution-handoff.md`, if created, preserves the resolved execution-readiness closure gates and does not broaden scope.
