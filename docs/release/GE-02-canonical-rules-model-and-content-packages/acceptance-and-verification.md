---
title: GE-02 Acceptance and Verification
stc_id: STC-CODEX-GE-02
artifact_type: acceptance-and-verification
status: accepted
scope: programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages
source_stc: ./README.md
---

# GE-02 Acceptance and Verification

## Objective
Define observable checks proving the GE-02 source STC establishes a canonical rules-model planning surface for the PF1 Core Rulebook Human Fighter level 1 pilot without direct LST syntax copying and with preserved source-lineage requirements.

## Acceptance criteria

### AV-02-001 — Required control bundle exists
The GE-02 source-STC control bundle MUST contain:
- `README.md`
- `technical-requirements.md`
- `technical-design.md`
- `acceptance-and-verification.md`
- `risks-and-open-questions.md`
- `epic-breakdown.md`
- `references/ge01-governed-inputs.md`
- `references/ge03-importer-dependency-contract.md`

Verification:
- inspect the directory under `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/`
- confirm every control file exists at the named path

### AV-02-001A — Required GE-02 output artifacts exist
GE-02 MUST contain its concrete generated documentary outputs:
- `artifacts/canonical-model-specification.md`
- `artifacts/content-package-layout-specification.md`
- `artifacts/pilot-object-examples.yaml`
- `artifacts/provenance-source-map-specification.md`
- `artifacts/expression-language-decision-criteria.md`
- `artifacts/compiled-ir-boundary-definition.md`
- `artifacts/content-validation-and-diagnostics-specification.md`

Verification:
- inspect `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/`
- confirm every generated output exists and is non-empty
- confirm `README.md` and `technical-requirements.md` link or name the generated output set

### AV-02-002 — GE-01 governed inputs are used explicitly
The source STC MUST cite and use all required GE-01 inputs:
- pilot corpus inventory
- token taxonomy
- conversion matrix
- unsupported-token ledger
- oracle surface inventory

Verification:
- `README.md` lists the inputs under `source_artifacts`
- `references/ge01-governed-inputs.md` summarizes row counts and maps each input surface to GE-02 requirements
- `technical-requirements.md` contains `TR-02-002 — Upstream GE-01 input contract`

### AV-02-003 — Anti-LST-clone rule is explicit
The source STC MUST prohibit treating PCGen LST syntax as the Codex canonical model.

Verification:
- `README.md`, `technical-requirements.md`, and `technical-design.md` each state that Codex must not directly mirror LST syntax
- the model homes in `technical-design.md` are semantic objects/effects/prerequisites/formulas/choices rather than raw token tables

### AV-02-004 — Pilot object homes are explicit
The source STC MUST provide canonical homes for the pilot-critical target concepts discovered by GE-01.

Required homes include:
- source package
- race and race trait/composition
- class and class feature/grant carrier
- feat
- skill
- equipment
- proficiency
- ability score
- save
- effect/grant
- prerequisite
- formula/value expression
- choice set
- provenance/source map
- diagnostic
- compiled runtime IR boundary

Verification:
- `TR-02-006` lists the homes
- `artifacts/canonical-model-specification.md` defines the homes as a concrete GE-02 output
- `technical-design.md` gives component boundaries and documentary schema skeletons
- `epic-breakdown.md` routes each home into downstream work

### AV-02-005 — Human Fighter pilot can be represented conceptually
The STC MUST be able to describe the PF1 Core Rulebook Human Fighter level 1 pilot in canonical model terms without copying LST syntax.

Verification:
- `artifacts/pilot-object-examples.yaml` includes documentary skeletons for package, race, class, equipment, effect, prerequisite, formula, choice-set, source-map, and diagnostic records
- `TR-02-015` states the Human Fighter representation requirement
- open formula/prerequisite/choice semantics are preserved as structured debt, not prose-only omissions

### AV-02-006 — Source lineage is preserved as a model obligation
The source STC MUST require provenance strong enough for later debugging, conversion coverage, and oracle review.

Verification:
- `TR-02-012` defines provenance/source-map requirements
- `artifacts/provenance-source-map-specification.md` defines required source-map fields and downgrade policy
- `references/ge01-governed-inputs.md` names GE-01 matrix and ledger provenance pressures

### AV-02-007 — Effects, prerequisites, formulas, and choice sets have explicit homes
The source STC MUST not bury high-risk rule behavior in a generic notes field.

Verification:
- `TR-02-007` through `TR-02-010` define effect, prerequisite, formula, and choice-set requirements
- `risks-and-open-questions.md` preserves unresolved expression/evaluation decisions
- `epic-breakdown.md` decomposes these as distinct downstream epics

### AV-02-008 — Authoring format and compiled IR are not conflated
The source STC MUST distinguish human-authored/source package content from compiled runtime IR/cache.

Verification:
- `TR-02-013` defines the boundary
- `artifacts/compiled-ir-boundary-definition.md` defines source package authority and compiled IR role
- no file in this bundle claims compiled IR is the only source of truth

### AV-02-009 — Unsupported and deferred behavior remains visible
The source STC MUST preserve unsupported, lossy, partial, deferred, or unresolved behavior as diagnostics or explicit open questions.

Verification:
- `TR-02-014` defines validation and diagnostics requirements
- `artifacts/content-validation-and-diagnostics-specification.md` defines validation and diagnostic classes
- `risks-and-open-questions.md` carries GE-01 ledger-derived unresolveds
- `references/ge01-governed-inputs.md` names high-risk ledger pressures

### AV-02-010 — No implementation authority is granted
The source STC MUST remain a planning authority surface, not a code-writing handoff.

Verification:
- `README.md` sets `work_type: planning-only`, `workflow_route: planning`, and `readiness: planning-ready`
- `README.md` target runtime write scope says no code write authority is granted
- no `execution-handoff.md` is created in this bundle

### AV-02-011 — GE-03 importer dependency contract is explicit
GE-02 MUST state what GE-03 importer planning may rely on from the accepted GE-02 artifacts and what remains unresolved for later handoffs.

Verification:
- `references/ge03-importer-dependency-contract.md` exists and is non-empty
- the contract names the relevant GE-01 evidence surfaces and GE-02 generated artifacts
- the contract maps GE-02 canonical model homes, package layout, provenance/source-map fields, diagnostics, expression-language constraints, and compiled-IR boundary into GE-03 importer obligations
- the contract explicitly refuses final schema, evaluator, runtime engine, branch/worktree, write scope, verification-command, and code-readiness assumptions

## Closure verification result
GE-02 passes the source-STC closure boundary when all named control-bundle files exist, the concrete generated documentary output set exists under `artifacts/`, all GE-01 input surfaces are cited, the model homes are explicit, the GE-03 importer dependency contract is present, and no implementation handoff is emitted.

As of 2026-06-19, this source STC is closed as `planning-ready`. It is ready to serve as a required read for downstream GE-03 re-audit, GE-03 importer dependency planning, and later bounded implementation handoffs. It is not `codex-ready` for code by itself because no code-authorizing slice, branch/worktree, write scope, or verification commands have been granted.
