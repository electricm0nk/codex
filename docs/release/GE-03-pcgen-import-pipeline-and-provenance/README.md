---
stc_id: STC-CODEX-GE-03
stc_kind: source-requirements
template_version: 2
work_type: planning-only
readiness: planning-ready
status: active
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/README.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch: main is the current grounded checkout; execution branch/worktree remains handoff-specific
  write_scope: none during source STC generation; any future code-authorizing GE-03 handoff must declare bounded repo paths explicitly
review_state: accepted
last_reviewed_at: 2026-06-19
parent_scopes:
  - programs/codex
source_artifacts:
  - programs/codex/plans/spec-domains/GE-03-pcgen-import-pipeline-and-provenance.md
  - programs/codex/plans/spec-domains/GE-02-canonical-rules-model-and-content-packages.md
  - programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/README.md
  - programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/canonical-model-specification.md
  - programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/content-package-layout-specification.md
  - programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/provenance-source-map-specification.md
  - programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/content-validation-and-diagnostics-specification.md
  - programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/README.md
  - programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/technical-design.md
  - programs/codex/requirements/GE-00-program-governance-and-scope/README.md
  - programs/codex/research/codex-reference-architecture-2026-06-17.md
related_artifacts:
  - programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/execution-handoff.md
  - programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/execution-handoff.md
  - programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md
  - programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/artifacts/ge02-dependency-reconciliation-2026-06-19.md
  - programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/artifacts/ge03-e1-f1-execution-readiness-closure-2026-06-19.md
upstream_targets:
  - programs/codex/README.md
  - programs/codex/requirements/README.md
supersedes: []
superseded_by: []
tags:
  - codex
  - ge-03
  - import-pipeline
  - provenance
  - parser
  - pcgen
  - pf1
---

# GE-03 — PCGen Import Pipeline and Provenance

## Objective
Define the source requirements construct for the Codex import bridge: parser stages, structured parse representation, token-registry and conversion-handler boundaries, provenance/source-map obligations, unsupported-token diagnostics, conversion reporting, and fixture strategy for the PF1 pilot path.

## Deliverable Type
`planning-only`

## Readiness
`planning-ready`

Why this readiness is accurate:
- the strategic GE-03 source artifact exists and explicitly scopes this work as the import-pipeline bridge
- GE-01 already exists as the upstream legacy-source control plane and documentary migration map
- the local Codex checkout is grounded, so future implementation target facts are no longer purely hypothetical
- GE-02 now exists as a planning-ready accepted source STC, with generated artifacts that provide importer-facing canonical model homes, package layout expectations, provenance/source-map obligations, and validation/diagnostic classes
- this bundle has passed GE-02 dependency reconciliation review and is ready for further planning or later bounded handoff derivation, but it still does not authorize implementation code
- GE03-E1-F1 has been selected as the first candidate implementation slice and now has a derived `execution-handoff.md`; code authority for implementation begins only in that bounded handoff

## Authority and Scope
- owner: `Todd Hintzmann`
- scope: `program`
- canonical: `true`
- canonical path: `programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/README.md`
- parent scopes:
  - `programs/codex`

This STC governs the Codex importer-boundary planning surface. It does not own the legacy-source discovery plane already defined by GE-01, and it does not own the canonical rules-model design in full, which remains a GE-02 dependency.

## Target Runtime
- repo: `/home/ubuntu/workspace/repos/codex`
- workdir: `/home/ubuntu/workspace/repos/codex`
- branch/worktree: `main is currently grounded locally; execution branch/worktree remains unresolved until a later implementation handoff`
- allowed write scope: `none during source STC generation; future GE-03 implementation handoff must bound repo paths explicitly`

The target repo is grounded, but code-authorizing execution authority is intentionally absent. This bundle defines the importer problem truthfully without authorizing parser or converter code.

## Document Map
- `technical-requirements.md` — normative requirements for parser stages, structured parse outputs, token registry, conversion handlers, provenance/source maps, unsupported-token reporting, conversion-report shape, and fixture strategy
- `technical-design.md` — architecture/design response describing the import pipeline shape and subsystem boundaries separately from the normative requirements
- `acceptance-and-verification.md` — observable checks proving the GE-03 source STC is complete enough for later planning and bounded implementation handoff derivation
- `risks-and-open-questions.md` — unresolved importer questions, remaining GE-02-adjacent model/implementation uncertainties, and anti-hallucination boundaries
- `epic-breakdown.md` — implementation-facing epics and feature seeds derived from the GE-03 source STC while remaining upstream of any code-authorizing handoff
- `execution-handoff.md` — code-authorizing, bounded handoff for GE03-E1-F1 only
- `artifacts/ge03-e1-f1-execution-readiness-closure-2026-06-19.md` — slice-specific closure receipt selecting GE03-E1-F1 and recording the branch policy, write scope, Rust/Codex runtime substrate, verification commands, and provenance threshold required for the handoff

## Required Reads
- `../../plans/spec-domains/GE-03-pcgen-import-pipeline-and-provenance.md` — primary strategic authority for the importer bridge and its required requirement classes
- `../GE-01-legacy-corpus-and-conversion-matrix/README.md` — upstream migration-control plane defining pilot corpus, conversion-matrix posture, unsupported-token visibility, and provenance expectations
- `../GE-01-legacy-corpus-and-conversion-matrix/technical-design.md` — explicit statement that GE-01 is documentary control plane and GE-03 owns importer execution authority
- `../GE-00-program-governance-and-scope/README.md` — inherited non-negotiables including headless core first, PCGen as oracle not architecture, conversion matrix as control plane, and no unsupported-token silence
- `../GE-02-canonical-rules-model-and-content-packages/README.md` — accepted planning-ready canonical-model source STC that grounds GE-03 target-model dependency posture
- `../GE-02-canonical-rules-model-and-content-packages/artifacts/canonical-model-specification.md` — canonical model homes and relationships that importer conversion planning must target
- `../GE-02-canonical-rules-model-and-content-packages/artifacts/content-package-layout-specification.md` — package and manifest layout expectations that importer output planning must respect
- `../GE-02-canonical-rules-model-and-content-packages/artifacts/provenance-source-map-specification.md` — lineage and source-map obligations that importer provenance planning must preserve
- `../GE-02-canonical-rules-model-and-content-packages/artifacts/content-validation-and-diagnostics-specification.md` — validation and diagnostic classes that importer reports must feed

## Conditional Reads
- `../../research/codex-reference-architecture-2026-06-17.md` — only if parser/import pipeline shape, conversion-matrix posture, or provenance/source-map guidance needs deeper architectural grounding
- `../GE-01-legacy-corpus-and-conversion-matrix/technical-requirements.md` — only if exact upstream language is needed for provenance, ledger, or parity-prohibition wording
- `/home/ubuntu/workspace/repos/codex/AGENTS.md` — only if a later session proposes repo-facing implementation work or an actual GE-03 code-authorizing handoff

## In Scope
- Codex GE-03 source-STC documents under `programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/`
- parser-stage requirements for pilot PCC/LST files
- structured parse representation and source-span preservation requirements
- token registry and conversion-handler boundary requirements
- source-map, provenance, unsupported-token-report, and conversion-report requirements
- fixture-driven verification posture and downstream epic decomposition for importer implementation planning
- explicit linkage to the accepted GE-02 source STC and generated artifacts wherever canonical target-model posture matters

## Out of Scope
- writing parser, importer, token-handler, source-map, conversion-report CLI, or test code in `/home/ubuntu/workspace/repos/codex`
- modifying `/home/ubuntu/workspace/repos/pcgen`
- redefining the GE-01 discovery/control-plane boundary
- final canonical rules-model design beyond what must be referenced from GE-02
- broad GE-04, GE-05, or GE-06 replanning except where a dependency must be named
- deriving a GE-03 code-authorizing implementation handoff in this run

## Acceptance Summary
This STC is complete only when the acceptance criteria in `acceptance-and-verification.md` are satisfied.

Compact summary:
- the importer is documented as a compatibility bridge, not a new authoring substrate
- parser, AST/structured parse, token-registry, handler, provenance, diagnostics, and report obligations are explicit
- unsupported or lossy behavior is structurally visible rather than implied away
- GE-02 artifact usage is recorded explicitly instead of letting importer planning invent canonical targets
- downstream implementation work can be decomposed into bounded epics without inventing runtime facts, final schema details, expression/evaluator choices, or code authority

## Allowed Assumptions
- the PF1 Core Rulebook Human Fighter level 1 pilot remains the first proof target unless a higher-order decision surface changes it
- the local Codex repo at `/home/ubuntu/workspace/repos/codex` remains the future implementation surface for importer work, but no code write authority is granted here
- the GE-02 source STC and generated artifacts are authoritative planning inputs for importer-facing canonical target dependencies, but they do not settle final production schemas, expression/evaluator implementation, runtime engine behavior, branch/worktree, write scope, or verification commands

## Blockers / Forbidden Assumptions
- stop if the GE-03 spec domain is missing or materially contradicts the structure claimed here
- do not assume the importer may override or redefine GE-02 canonical model homes
- do not assume unsupported tokens may be dropped silently pending later cleanup
- do not claim parser coverage, conversion success, provenance completeness, or parity success from this source STC alone
- do not let this source STC or its filename become counterfeit write authority for repo code
- do not create `execution-handoff.md` for GE03-E1-F1 unless it preserves the branch policy, write scope, verification commands, stop conditions, and non-goals recorded in `artifacts/ge03-e1-f1-execution-readiness-closure-2026-06-19.md`

## Next Stage Rule
- GE-03 is `planning-ready` after GE-02 dependency reconciliation because it truthfully captures the importer bridge, GE-01 dependency, and accepted GE-02 planning inputs without invention.
- GE03-E1-F1 is the selected first implementation-slice candidate and has a derived `execution-handoff.md` under the policy and environment facts recorded in the execution-readiness closure receipt.
- Do not derive any code-authorizing GE-03 implementation handoff until the bounded implementation slice is chosen and repo/workdir/branch/write-scope/verification facts are explicit for that slice.
- A future GE-03 implementation handoff must remain narrower than the spec domain even though GE-03 as a spec domain will eventually drive multiple downstream importer epics.
- If a later handoff attempts to authorize implementation directly from this source STC, it must first prove exact target paths, branch/worktree policy, allowed write scope, and verification commands for the selected slice.
