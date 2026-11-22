---
title: GE-04 Technical Requirements
stc_id: STC-CODEX-GE-04
artifact_type: technical-requirements
status: draft
scope: programs/codex/requirements/GE-04-rules-engine-and-explainability-core
source_stc: ./README.md
source_artifacts:
  - ../../plans/spec-domains/GE-04-rules-engine-and-explainability-core.md
  - ../GE-02-canonical-rules-model-and-content-packages/README.md
  - ../GE-02-canonical-rules-model-and-content-packages/artifacts/canonical-model-specification.md
  - ../GE-02-canonical-rules-model-and-content-packages/artifacts/expression-language-decision-criteria.md
  - ../GE-02-canonical-rules-model-and-content-packages/artifacts/compiled-ir-boundary-definition.md
  - ../GE-02-canonical-rules-model-and-content-packages/artifacts/content-validation-and-diagnostics-specification.md
  - ../GE-02-canonical-rules-model-and-content-packages/artifacts/provenance-source-map-specification.md
  - ../GE-03-pcgen-import-pipeline-and-provenance/README.md
  - ../GE-00-program-governance-and-scope/README.md
---

# GE-04 Technical Requirements

## Objective
Define the normative requirements for the Codex rules-engine and explainability core: deterministic character computation from canonical content, effect evaluation, prerequisite/formula evaluation, choice availability, derived stat output, diagnostics, explanation graphs, headless entry points, and pilot computation fixtures.

## Normative language
- **MUST** means required for GE-04 completion.
- **SHOULD** means expected unless a later decision surface records a justified deviation.
- **MUST NOT** means prohibited for this STC.

## TR-04-001 — Rules-core posture
Codex MUST treat the GE-04 rules core as all of the following:

- a headless computation engine downstream of canonical content packages
- an explainability engine that can report why each tested value, choice, or failed prerequisite has its observed result
- a diagnostic producer that keeps invalid content, invalid character state, unsupported semantics, and engine defects visible
- a deterministic subsystem whose behavior can be tested without desktop UI involvement

Codex MUST NOT treat the rules core as:

- a UI display helper
- a black-box calculator that returns only numbers
- an oracle comparison harness
- a place to silently complete unresolved importer or canonical-model semantics

## TR-04-002 — Upstream dependency truth
The GE-04 source STC MUST state these dependencies explicitly:

- GE-02 owns canonical model homes, expression criteria, diagnostics, provenance, source-package authority, and compiled-IR boundary posture
- GE-03 owns import/provenance bridge requirements and is the expected source of imported canonical content plus diagnostics
- GE-05 owns oracle comparison evidence and parity claims
- GE-06 owns the integrated pilot vertical-slice contract
- GE-00 contributes non-negotiables including headless core first, explainability as product behavior, PCGen as oracle not architecture, and quality-gate evidence rules

GE-04 MUST cite accepted GE-02 artifacts for canonical model inputs and MUST NOT redefine the canonical source-package model locally.

## TR-04-003 — Character input model requirements
GE-04 MUST define the minimum character input shape needed for deterministic pilot computation.

At minimum, the character input requirements MUST cover:

- game system/source package identity
- character ancestry/race selection
- class and level selection
- ability score input
- feat selections
- skill choices or allocations when required by the pilot
- equipment selections and equipped/active state where a derived value depends on them
- any choice-set resolution needed before effect evaluation
- provenance or user-selection source sufficient for explanation output

The source STC MUST distinguish between canonical content inputs, user/character choices, imported diagnostics, and derived runtime state.

## TR-04-004 — Source-package and validation preconditions
Before rules computation can claim correctness, GE-04 MUST require source package content to be in a known validation state.

At minimum, the computation preconditions MUST include:

- source package identity and dependency graph are known
- stable IDs resolve for objects touched by the character fixture
- referenced effects, prerequisites, formulas, choices, selectors, and diagnostics are available or explicitly diagnosed as unavailable
- imported content carries provenance/source-map records when relevant
- unresolved diagnostics that affect the claimed behavior block higher-tier claims

Rules execution MUST NOT silently run through unsupported or lossy imported semantics as if they were accepted behavior.

## TR-04-005 — Effect evaluation pipeline requirements
GE-04 MUST define an effect evaluation pipeline for the pilot scope.

At minimum, the pipeline requirements MUST describe:

- how candidate effects are collected from race, class, feats, skills, equipment, selected choices, and source-package defaults
- how prerequisites or activation conditions gate effects
- how active effects contribute to derived values or choice availability
- how ordering, dependency discovery, and repeat evaluation are recorded when values depend on earlier computations
- how invalid, unsupported, circular, or unresolved effects become diagnostics

The pipeline MUST preserve enough structure to explain each contribution rather than collapsing all effects into final values only.

## TR-04-006 — Evaluation order requirements
GE-04 MUST produce a documented pilot evaluation-order definition.

The evaluation-order definition MUST cover at minimum:

- package load/validation preconditions
- character input normalization
- baseline state creation
- static grants and always-on effects
- prerequisite and formula evaluation
- choice availability calculation
- selected equipment and active-condition handling
- derived stat calculation
- diagnostic and explanation graph emission

Any still-unresolved order issue MUST appear in `risks-and-open-questions.md` rather than being hidden in confident prose.

## TR-04-007 — Formula evaluation requirements
GE-04 MUST define formula evaluation requirements without prematurely selecting the final evaluator.

The formula substrate MUST support:

- deterministic and sandboxed evaluation
- typed or otherwise validated inputs and outputs
- dependency capture for explanation and cycle detection
- provenance from canonical content and imported source maps
- diagnostics for invalid, unsupported, unresolved, or partially converted formulas
- test fixture assertions for pilot formulas

The final expression-language implementation choice remains deferred unless a later decision record or spike accepts it.

## TR-04-008 — Prerequisite evaluation requirements
GE-04 MUST define prerequisite evaluation requirements for both eligibility and explanation.

At minimum, prerequisite evaluation MUST produce:

- pass/fail result
- expected condition
- actual observed value or state
- source object and prerequisite identity
- dependencies consulted
- diagnostics when a prerequisite cannot be evaluated safely

Failed prerequisites MUST be explainable as expected-versus-actual outcomes, not merely boolean `false`.

## TR-04-009 — Choice availability requirements
GE-04 MUST define how available choices are calculated for the pilot.

The choice availability requirements MUST cover:

- source choice set identity
- candidate options
- prerequisite or selector filters
- already-selected or repeatability constraints when relevant
- unavailable choice explanations
- diagnostics for unsupported selectors, unresolved references, or deferred semantics

Choice availability MUST be computable headlessly and MUST NOT depend on desktop UI state.

## TR-04-010 — Derived stat output requirements
GE-04 MUST define the pilot derived-value output categories that rules execution must eventually calculate.

At minimum, the output requirements MUST include pilot-relevant categories such as:

- ability modifiers and ability-dependent values
- hit points or hit-point inputs where pilot fixture scope permits
- base attack bonus and attack-related values for selected weapon/equipment cases
- armor class or armor/equipment-influenced values for selected equipment cases
- saving throws
- skill-related values required by the pilot fixture
- prerequisite/choice availability outcomes for at least one feat or proficiency path

Exact final pilot values remain fixture-dependent and must be grounded by the future GE-06 character fixture and GE-05 oracle evidence before parity claims.

## TR-04-011 — Explanation graph requirements
GE-04 MUST define an explanation graph output schema.

The explanation graph MUST support:

- nodes for character inputs, canonical objects, effects, formulas, prerequisites, choices, derived values, diagnostics, and provenance/source-map records
- edges for contributes-to, depends-on, grants, modifies, checks, blocks, selects, and sourced-from relationships or equivalent accepted vocabulary
- enough detail to explain why a derived value has its value
- enough detail to explain why an unavailable choice or failed prerequisite is unavailable
- references to diagnostics and provenance where behavior depends on imported or unresolved content

The explanation graph schema MUST remain conceptual until implementation handoff grounds the exact serialized shape.

## TR-04-012 — Rules diagnostic schema requirements
GE-04 MUST define a diagnostic schema for rules execution.

The diagnostic schema MUST distinguish at least:

- invalid canonical content
- invalid character input or invalid character choice
- unsupported imported semantics
- unresolved references
- invalid or unsupported expressions
- circular or unstable dependencies
- engine defects
- provenance gaps that block explanation or parity claims

Diagnostics MUST be machine-readable enough for tests and human-readable enough for debugging.

## TR-04-013 — Headless CLI/test entry-point requirements
GE-04 MUST require non-UI execution paths for rules behavior.

At minimum, future implementation handoffs MUST preserve the ability to run rules-core behavior through tests and/or CLI commands that:

- load a deterministic fixture
- compute pilot outputs without desktop UI
- emit derived values
- emit explanations
- emit diagnostics
- fail when expected values, explanation edges, or diagnostic expectations are not satisfied

No UI screenshot may satisfy the GE-04 rules-correctness gate.

## TR-04-014 — Pilot golden computation fixture requirements
GE-04 MUST produce requirements for deterministic pilot computation fixtures.

The fixture requirements MUST name:

- required input classes
- selected pilot character dimensions still awaiting GE-06 finalization
- expected-output categories
- explanation assertions
- diagnostic assertions
- provenance/source-map expectations when imported content contributes to behavior
- oracle-comparison readiness boundary for GE-05

The fixture artifact MUST NOT fabricate final values before the character fixture and oracle comparison surfaces are grounded.

## TR-04-015 — Produced artifacts
GE-04 MUST produce a source-STC bundle containing:

- `README.md`
- `technical-requirements.md`
- `technical-design.md`
- `acceptance-and-verification.md`
- `risks-and-open-questions.md`
- `epic-breakdown.md`

GE-04 MUST also produce same-epic documentary artifacts containing:

- `artifacts/rules-engine-technical-specification.md`
- `artifacts/evaluation-order-definition.md`
- `artifacts/expression-language-runtime-requirements.md`
- `artifacts/explanation-graph-schema.md`
- `artifacts/diagnostic-schema.md`
- `artifacts/pilot-golden-computation-fixture-requirements.md`

This package MUST live under `programs/codex/requirements/GE-04-rules-engine-and-explainability-core/`.

## TR-04-016 — Downstream routing rule
GE-04 MUST route later implementation work into bounded downstream epics rather than treating the entire rules engine as one handoff.

At minimum, downstream decomposition MUST include:

- character input model
- effect evaluation core
- formula and prerequisite evaluator
- choice availability engine
- diagnostic emitter
- explanation graph builder
- rules-core CLI/test entry point
- pilot golden computation fixture

## Success definition
GE-04 succeeds when Codex has a rules-engine planning surface strong enough to say:

- what canonical content and character input the engine requires
- how effects, formulas, prerequisites, choices, and derived values are expected to flow through a deterministic headless pipeline
- how failed prerequisites and unavailable choices explain expected versus actual state
- how diagnostics distinguish content, character, importer, expression, dependency, provenance, and engine failures
- how explanation graphs preserve source contributions
- what evidence a later implementation handoff must produce before GE-05 can compare outputs and GE-06 can integrate the pilot

If those answers still require invention, GE-04 is not complete.
