---
title: GE-03 Technical Requirements
stc_id: STC-CODEX-GE-03
artifact_type: technical-requirements
status: draft
scope: programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance
source_stc: ./README.md
source_artifacts:
  - ../../plans/spec-domains/GE-03-pcgen-import-pipeline-and-provenance.md
  - ../GE-01-legacy-corpus-and-conversion-matrix/README.md
  - ../GE-01-legacy-corpus-and-conversion-matrix/technical-design.md
  - ../../plans/spec-domains/GE-02-canonical-rules-model-and-content-packages.md
  - ../GE-02-canonical-rules-model-and-content-packages/README.md
  - ../GE-02-canonical-rules-model-and-content-packages/artifacts/canonical-model-specification.md
  - ../GE-02-canonical-rules-model-and-content-packages/artifacts/content-package-layout-specification.md
  - ../GE-02-canonical-rules-model-and-content-packages/artifacts/provenance-source-map-specification.md
  - ../GE-02-canonical-rules-model-and-content-packages/artifacts/content-validation-and-diagnostics-specification.md
  - ../GE-00-program-governance-and-scope/README.md
---

# GE-03 Technical Requirements

## Objective
Define the normative requirements for the Codex PCGen import bridge: parser stages, structured parse outputs, semantic-token handling, provenance/source maps, unsupported-token diagnostics, conversion reporting, and fixture-driven validation for the pilot slice.

## Normative language
- **MUST** means required for GE-03 completion.
- **SHOULD** means expected unless a later decision surface records a justified deviation.
- **MUST NOT** means prohibited for this STC.

## TR-03-001 — Importer posture
Codex MUST treat the GE-03 importer as all of the following:

- a compatibility bridge from PCGen source material into Codex canonical content
- a provenance-preserving translation surface
- a diagnostic surface that makes unsupported and lossy behavior visible
- a bounded subsystem downstream of GE-01 and dependent on GE-02

Codex MUST NOT treat the importer as:

- the new authoring substrate
- a substitute for the canonical rules model
- a best-effort regex migration shortcut
- permission to claim import coverage without explicit evidence

## TR-03-002 — Upstream dependency truth
The GE-03 source STC MUST state the following dependencies explicitly:

- GE-01 owns the legacy-source discovery, conversion-matrix control plane, unsupported-token posture, and pilot-corpus grounding
- GE-02 owns the canonical target-model planning surface through its accepted source STC and generated artifacts
- GE-00 contributes non-negotiables including headless core first, PCGen as oracle not architecture, conversion matrix as control plane, and no unsupported-token silence

The GE-03 source STC MUST cite GE-02 artifacts for canonical target homes and MUST NOT treat GE-02 planning readiness as final production schema, expression/evaluator, runtime engine, or implementation readiness.

## TR-03-003 — Pilot parse-target boundary
GE-03 MUST bound its first importer requirements to the PF1 Core Rulebook Human Fighter level 1 pilot path while remaining explicit about adjacent syntax or token families that are discovered but not yet required for the first slice.

The source STC MUST distinguish between:

- pilot-critical syntax and token families
- adjacent but non-critical syntax discovered during planning
- explicitly deferred syntax families
- unknown syntax or semantics that still block later implementation work

## TR-03-004 — Parser stage requirements
The source STC MUST define parser-stage requirements covering at minimum:

- PCC lexical and structural parsing obligations relevant to pilot entry files and include semantics
- LST lexical and structural parsing obligations relevant to pilot source files
- source-span capture requirements for tokens, lines, or equivalent structured locations when available
- representation of parse errors and unsupported syntax as explicit diagnostics rather than silent failure

The parser requirements MUST make clear that parsing and semantic conversion are separate stages.

## TR-03-005 — Structured parse representation
The source STC MUST require a structured parse representation capable of supporting later semantic conversion.

That representation MUST be able to preserve, at minimum:

- source file identity
- source ordering or span information available at the time of parse
- token family or syntactic construct identity
- include or containment relationships where applicable
- enough structure to route behavior to the correct token registry entry or conversion handler

A future implementation MAY choose token stream, AST, typed AST, or semantic-event layers, but GE-03 MUST preserve the requirement that the representation be structured and provenance-bearing.

## TR-03-006 — Token registry boundary
The source STC MUST define a token registry requirement for pilot token families.

For each token family or semantic class, the registry requirement MUST be able to record:

- legacy token or construct family
- applicable source kind or object class
- human meaning
- handler ownership boundary
- risk or ambiguity notes
- validation obligations
- unsupported or deferred posture when no handler exists yet

The registry MUST NOT collapse high-risk constructs such as prerequisites, formulas, bonuses, or choice semantics into undifferentiated generic handling.

## TR-03-007 — Conversion-handler boundary
The source STC MUST define a conversion-handler boundary between structured legacy input and canonical Codex targets.

For each handler class, the source STC MUST require the ability to describe:

- accepted legacy input family
- intended canonical target concept
- provenance obligations carried through conversion
- diagnostic behavior for unsupported, partial, lossy, or intentionally ignored cases
- downstream dependence on GE-02 model decisions when applicable

The source STC MUST NOT permit a handler model that hides unresolved semantics behind vague “best effort” language.

## TR-03-008 — Provenance and source-map obligations
GE-03 MUST define provenance requirements strong enough to support debugging, coverage review, and parity claims.

The required provenance model MUST preserve, when available:

- source PCC include path
- source LST file identity
- line, token span, or equivalent structured location
- token-registry entry or handler identity
- canonical target object or field written
- whether behavior was exact, partial, lossy, unsupported, deferred, or intentionally ignored

If future tooling cannot capture the strongest span precision immediately, the STC MUST require an explicit downgrade path rather than pretending the missing precision does not matter.

## TR-03-009 — Unsupported-token diagnostics
The source STC MUST require unsupported or lossy behavior to surface as first-class diagnostics.

At minimum, the importer requirements MUST support:

- explicit unsupported-token reports
- linkage back to GE-01 ledger/matrix posture
- severity or blocking classification
- source references sufficient for remediation
- no silent dropping of unresolved semantics

This requirement inherits the GE-00 rule against unsupported-token silence.

## TR-03-010 — Conversion report requirements
The source STC MUST define a conversion-report requirement that can communicate importer coverage honestly.

At minimum, the report requirements MUST support:

- imported object counts or equivalent coverage summaries
- mapped-exact / partial / unsupported / intentionally ignored posture
- provenance or diagnostic linkage for failures and caveats
- machine-readable and human-auditable output expectations
- validation evidence references tied to fixtures or later oracle checks

The report MUST be designed to block vague claims such as “Pathfinder imports” when token-level or object-level evidence does not exist.

## TR-03-011 — Fixture-driven verification posture
The source STC MUST define a fixture-driven verification strategy.

At minimum, fixture requirements MUST support:

- pilot PCC/LST parse targets with expected structured outcomes
- token-family mapping or explicit diagnostic expectations
- provenance/source-map proof obligations
- conversion-report expectations
- room for future golden comparisons against legacy PCGen behavior

The GE-03 source STC MUST treat unknown oracle automation capability as a documented question, not as permission to fabricate validation steps.

## TR-03-012 — Authority surface boundary
During GE-03 source-STC generation, writable scope MUST remain limited to the Codex documentation authority surface.

The work MUST NOT:

- modify `/home/ubuntu/workspace/repos/pcgen`
- write implementation code into `/home/ubuntu/workspace/repos/codex`
- substitute repo scaffolding for requirement truth
- claim that a planning-only source STC is already an implementation handoff

## TR-03-013 — GE-02 dependency recording rule
Where the importer requirements depend on the canonical model, GE-03 MUST record the dependency explicitly rather than fabricating resolved model structure.

At minimum, the source STC MUST preserve open dependency on:

- canonical object homes for imported concepts
- formula and prerequisite representation
- choice-set structure
- final content-package and IR boundary decisions
- any target schema detail not yet grounded by accepted GE-02 artifacts or an equivalent later decision record

## TR-03-014 — Downstream routing rule
GE-03 MUST route later implementation work into bounded downstream epics rather than smearing code intent across the requirements files.

At minimum, the source STC MUST decompose follow-on work into:

- PCC parser implementation
- LST parser implementation
- token registry implementation
- pilot token-handler implementation
- source-map/provenance writer work
- conversion-report CLI work
- unsupported-token ledger integration
- fixture and parity-harness work

## TR-03-015 — Produced artifacts
GE-03 MUST produce a source-STC bundle containing:

- `README.md`
- `technical-requirements.md`
- `technical-design.md`
- `acceptance-and-verification.md`
- `risks-and-open-questions.md`
- `epic-breakdown.md`

This bundle MUST live under `programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/`.

## Success definition
GE-03 succeeds when Codex has an importer-boundary planning surface strong enough to say:

- how pilot PCC/LST files are expected to parse into structured outputs
- how token families are routed into explicit registry and handler boundaries
- how unsupported and lossy behavior is surfaced instead of buried
- how provenance and source maps are preserved through the bridge
- how conversion coverage will later be reported and verified
- which downstream implementation epics exist and which dependencies still belong to GE-02

If those answers still require invention, GE-03 is not complete.
