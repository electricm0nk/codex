---
stc_id: STC-CODEX-GE-04
stc_kind: source-requirements
template_version: 2
work_type: planning-only
workflow_route: planning
readiness: planning-ready
status: active
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/GE-04-rules-engine-and-explainability-core/README.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch: GE04-E1-F1 execution derives from clean current `develop`; see execution-handoff.md for the code-authorizing execution branch
  write_scope: source STC itself grants none; bounded repo paths for GE04-E1-F1 are declared in execution-handoff.md
review_state: draft
last_reviewed_at: 2026-06-20
parent_scopes:
  - programs/codex
source_artifacts:
  - programs/codex/plans/spec-domains/GE-04-rules-engine-and-explainability-core.md
  - programs/codex/plans/spec-domains/GE-02-canonical-rules-model-and-content-packages.md
  - programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/README.md
  - programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/canonical-model-specification.md
  - programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/expression-language-decision-criteria.md
  - programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/compiled-ir-boundary-definition.md
  - programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/content-validation-and-diagnostics-specification.md
  - programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/provenance-source-map-specification.md
  - programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/README.md
  - programs/codex/plans/spec-domains/GE-05-oracle-validation-and-parity-harness.md
  - programs/codex/plans/spec-domains/GE-06-pilot-vertical-slice-pf1-human-fighter.md
  - programs/codex/requirements/GE-00-program-governance-and-scope/README.md
related_artifacts:
  - programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md
  - programs/codex/doctrine/program-doctrine-and-scope-charter.md
  - programs/codex/doctrine/documentation-control-plane.md
  - programs/codex/doctrine/quality-gate-policy.md
  - programs/codex/requirements/GE-04-rules-engine-and-explainability-core/artifacts/ge04-e1-f1-execution-readiness-closure-2026-06-20.md
upstream_targets:
  - programs/codex/README.md
  - programs/codex/requirements/README.md
supersedes: []
superseded_by: []
tags:
  - codex
  - ge-04
  - rules-engine
  - explainability
  - diagnostics
  - expression-language
  - pf1
---

# GE-04 — Rules Engine and Explainability Core

## Objective
Define the source requirements construct for the Codex headless rules core: deterministic character-state computation from canonical content, effect evaluation, prerequisites, formulas, choice availability, diagnostics, explanation graphs, and non-UI test/CLI entry points for the PF1 pilot path.

## Deliverable Type
`planning-only`

## Workflow Route
`planning`

## Readiness
`planning-ready`

Why this readiness is accurate:
- the strategic GE-04 spec domain exists and explicitly frames this work as a boundary object, not an implementation prompt
- GE-02 has an accepted planning-ready canonical-model source STC and generated artifacts that define model homes, expression criteria, diagnostic posture, provenance obligations, and compiled-IR boundaries
- GE-03 exists as the importer bridge source STC and identifies how canonical pilot content is expected to arrive with provenance and diagnostics
- this bundle creates the concrete GE-04 documentary artifacts required by the spec domain: rules-engine specification requirements, evaluation order, expression runtime requirements, explanation graph schema, diagnostic schema, and pilot golden computation fixture requirements
- the local Codex implementation checkout is grounded, but this STC grants no code write authority and no GE-04 implementation branch, write scope, or verification command set has been authorized

## Closure State
GE-04 is generated as a planning-ready source STC for the rules-engine and explainability requirements boundary as of 2026-06-20. It defines what later implementation must prove before engine behavior can be called computed, explainable, diagnostic-rich, or suitable for oracle comparison. It does **not** authorize implementation code, choose a final expression evaluator, settle all stacking/circular-dependency semantics, or claim PCGen parity.

## Authority and Scope
- owner: `Todd Hintzmann`
- scope: `program`
- canonical: `true`
- canonical path: `programs/codex/requirements/GE-04-rules-engine-and-explainability-core/README.md`
- parent scopes:
  - `programs/codex`

This STC governs the Codex rules-engine and explainability planning surface. GE-02 owns canonical source-package model truth. GE-03 owns import/provenance bridge truth. GE-05 owns oracle parity harness truth. GE-06 owns the integrated pilot slice contract. GE-04 owns the computation and explanation contract that sits between canonical content and parity/UI consumers.

## Target Runtime
- repo: `/home/ubuntu/workspace/repos/codex`
- workdir: `/home/ubuntu/workspace/repos/codex`
- branch/worktree: `current local checkout observed 2026-06-20 is ge03-e1-f1-pcc-entry-parser; GE-04 execution branch/worktree remains unresolved until a later implementation handoff`
- allowed write scope: `none during source STC generation; future GE-04 implementation handoff must bound repo paths explicitly`

The target repo is grounded only as the future implementation surface. This package is a requirements authority surface under `programs/codex/requirements/`, not a repo-local implementation brief.

## Document Map
- `technical-requirements.md` — normative requirements for character input, effect evaluation, prerequisites, formulas, choice availability, derived stats, diagnostics, explanation graphs, headless entry points, deterministic fixtures, and downstream boundaries
- `technical-design.md` — architecture/design response describing the rules-core computation pipeline and subsystem boundaries separately from the normative requirements
- `acceptance-and-verification.md` — observable checks proving the GE-04 source STC and generated artifacts define a falsifiable rules-correctness gate without UI or oracle counterfeit claims
- `risks-and-open-questions.md` — unresolved evaluation-order, expression-semantics, stacking, circularity, diagnostic taxonomy, explanation-granularity, and fixture-selection questions
- `epic-breakdown.md` — downstream implementation-facing epics and feature seeds for later bounded handoff derivation
- `artifacts/rules-engine-technical-specification.md` — concrete GE-04 rules-engine technical specification requirements
- `artifacts/evaluation-order-definition.md` — concrete pilot evaluation-order definition and unresolved escalation points
- `artifacts/expression-language-runtime-requirements.md` — concrete runtime requirements for formula/prerequisite expression evaluation without choosing an evaluator
- `artifacts/explanation-graph-schema.md` — concrete conceptual explanation graph schema for derived values, choices, prerequisites, diagnostics, and provenance
- `artifacts/diagnostic-schema.md` — concrete rules-engine diagnostic schema and taxonomy
- `artifacts/pilot-golden-computation-fixture-requirements.md` — concrete requirements for the first deterministic Human Fighter computation fixture
- `artifacts/ge04-e1-f1-execution-readiness-closure-2026-06-20.md` — execution-readiness closure recording why GE04-E1-F1 is codex-ready and pointing at the derived code-authorizing handoff
- `execution-handoff.md` — bounded code-authorizing execution brief for GE04-E1-F1 only

## Required Reads
- `../../plans/spec-domains/GE-04-rules-engine-and-explainability-core.md` — primary strategic authority for this source STC
- `../GE-02-canonical-rules-model-and-content-packages/README.md` — accepted canonical-model source STC and generated documentary artifact index
- `../GE-02-canonical-rules-model-and-content-packages/artifacts/canonical-model-specification.md` — canonical model homes for effects, prerequisites, formulas, choices, diagnostics, provenance, and compiled runtime IR
- `../GE-02-canonical-rules-model-and-content-packages/artifacts/expression-language-decision-criteria.md` — inherited expression-language safety, determinism, structure, provenance, and diagnostics criteria
- `../GE-02-canonical-rules-model-and-content-packages/artifacts/compiled-ir-boundary-definition.md` — source-package versus compiled-runtime-IR authority boundary
- `../GE-02-canonical-rules-model-and-content-packages/artifacts/content-validation-and-diagnostics-specification.md` — validation and diagnostic classes that rules execution must respect
- `../GE-02-canonical-rules-model-and-content-packages/artifacts/provenance-source-map-specification.md` — provenance and diagnostic linkage obligations for imported content
- `../GE-03-pcgen-import-pipeline-and-provenance/README.md` — importer bridge posture and provenance/diagnostic outputs expected before engine execution claims
- `../GE-00-program-governance-and-scope/README.md` — inherited non-negotiables including headless core first, explainability as product behavior, PCGen as oracle not architecture, and no unsupported-token silence
- `../../plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md` — pilot target boundary for later deterministic computation fixture selection

## Conditional Reads
- `../../plans/spec-domains/GE-05-oracle-validation-and-parity-harness.md` — when defining outputs that GE-05 will compare against PCGen
- `../../plans/spec-domains/GE-06-pilot-vertical-slice-pf1-human-fighter.md` — when aligning the golden computation fixture with integrated pilot-slice viability
- `../GE-03-pcgen-import-pipeline-and-provenance/execution-handoff.md` — only if later engine work depends on actual importer implementation outputs or fixtures
- `/home/ubuntu/workspace/repos/codex/AGENTS.md` — only if a later session proposes repo-facing GE-04 implementation work

## In Scope
- Codex GE-04 source-STC documents under `programs/codex/requirements/GE-04-rules-engine-and-explainability-core/`
- concrete GE-04 generated documentary artifacts under `programs/codex/requirements/GE-04-rules-engine-and-explainability-core/artifacts/`
- character input requirements for the PF1 Human Fighter level 1 pilot
- deterministic effect evaluation pipeline requirements
- prerequisite and formula evaluation requirements
- choice availability requirements
- derived stat calculation requirements for pilot values
- explanation graph output requirements
- validation and rules-engine diagnostic schema requirements
- headless CLI/test entry-point requirements
- downstream epic decomposition for later bounded GE-04 implementation handoffs

## Out of Scope
- writing rules-engine implementation code in `/home/ubuntu/workspace/repos/codex`
- modifying `/home/ubuntu/workspace/repos/pcgen`
- final expression-language/evaluator selection
- full spellcasting engine, plugin ABI, or broad multi-system abstraction
- UI presentation beyond data structures that GE-07 may later consume
- oracle harness implementation owned by GE-05
- declaring PCGen parity, product-visible correctness, or broad Pathfinder support from this planning bundle alone

## Acceptance Summary
The acceptance criteria in `acceptance-and-verification.md` are satisfied for the planning-ready source-STC boundary when this bundle and its generated documentary artifacts exist and remain internally linked.

Compact summary:
- the rules engine is documented as a headless computation and explanation core, not a UI feature or oracle harness
- canonical content, validation state, diagnostics, provenance, and compiled-IR boundaries are inherited from GE-02 rather than redefined locally
- character input, effect evaluation, formula/prerequisite evaluation, choice availability, derived stats, explanation graphs, diagnostics, CLI/test entry points, and deterministic fixture obligations are explicit
- GE-04 explains what engine outputs must look like for GE-05 oracle comparison and GE-06 pilot integration without claiming either result yet
- downstream implementation work can be decomposed into bounded epics without inventing final evaluator choice, stacking behavior, circular dependency policy, branch/worktree, write scope, or verification commands

## Allowed Assumptions
- the PF1 Core Rulebook Human Fighter level 1 pilot remains the first proof target unless a higher-order decision surface changes it
- GE-02 accepted artifacts are authoritative planning inputs for canonical model homes, expression criteria, diagnostics, provenance, and compiled-IR boundaries
- GE-03 importer outputs are expected to provide canonical content, source maps, and diagnostics, but GE-04 must not assume the importer has already succeeded for any unproven scope
- future implementation may choose code structure and exact data representation, but it must preserve deterministic headless computation, diagnostic visibility, and source-contribution explainability unless a later decision record supersedes this STC

## Blockers / Forbidden Assumptions
- stop if a future handoff treats this source STC as code write authority without exact branch/worktree, allowed write scope, selected implementation slice, and verification commands
- do not claim rules correctness from a number without an explanation trail
- do not claim oracle parity; GE-05 owns comparison evidence
- do not choose a final expression evaluator here merely because formulas and prerequisites need one later
- do not silently resolve stacking, circular dependency, selected-equipment semantics, or explanation granularity questions not grounded by accepted upstream documents
- do not permit imported unsupported or lossy semantics to become executable behavior without diagnostics

## Next Stage Rule
- GE-04 is planning-ready as a source requirements construct because its control bundle and same-epic generated documentary outputs now exist.
- GE04-E1-F1 has passed execution-readiness validation. `artifacts/ge04-e1-f1-execution-readiness-closure-2026-06-20.md` records the grounded gate evidence, and `execution-handoff.md` is now the only code-authorizing brief for this slice.
- Any later GE-04 code-authorizing handoff must remain narrower than this spec domain and must identify exact repo paths, failing-first tests, fixture inputs, expected outputs, and non-goals before code work begins.
