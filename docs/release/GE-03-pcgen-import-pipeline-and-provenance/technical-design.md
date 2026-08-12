---
title: GE-03 Technical Design
stc_id: STC-CODEX-GE-03
artifact_type: technical-design
status: draft
scope: programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance
source_stc: ./README.md
---

# GE-03 Technical Design

## Purpose
This design operationalizes the GE-03 source STC without collapsing it into implementation code. It defines the importer-bridge structure Codex should use to parse legacy PCC/LST sources, preserve provenance, route semantics through explicit handlers, emit auditable diagnostics, and remain subordinate to the canonical-model boundary owned by GE-02.

## Design posture
- architecture style: `staged import bridge with explicit parser -> semantic conversion -> diagnostics/report boundaries`
- migration posture: `pilot-first, provenance-preserving, evidence-backed`
- canonical dependency posture: `import bridge is downstream of accepted GE-02 planning artifacts and must preserve remaining target-model gaps honestly`
- diagnostics posture: `strict about unsupported or lossy visibility; no silent downgrade`

## Context and constraints
- Codex is a new project. The legacy PCGen repo is reference material and oracle substrate, not the implementation architecture.
- GE-01 already owns the documentary control plane for legacy-source inventory, token taxonomy, conversion-matrix posture, unsupported-token ledger posture, and oracle-surface discovery.
- GE-03 is the first spec domain that owns importer execution planning, but this source STC still does not authorize code.
- The local Codex repo checkout is grounded at `/home/ubuntu/workspace/repos/codex`, yet branch/worktree/write-scope facts for code-authorizing work remain handoff-specific.
- GE-02 now exists as a planning-ready accepted source STC with generated artifacts that ground importer-facing canonical model homes, package layout, provenance/source-map obligations, and validation/diagnostic classes; remaining final-schema, expression/evaluator, and runtime-engine decisions must stay explicit unresolveds.

## Proposed system shape
GE-03 should be treated as an import-bridge design composed of seven documentary entities:

1. **PCC Parser Boundary** — defines how entry files and include semantics become structured inputs.
2. **LST Parser Boundary** — defines how list-file constructs become structured inputs with source identity.
3. **Structured Parse Representation** — preserves spans, ordering, and construct identity for later semantic handling.
4. **Token Registry** — maps token families and construct classes to explicit semantic ownership.
5. **Conversion Handlers** — translate structured legacy inputs into canonical Codex targets or explicit diagnostics.
6. **Provenance / Source-Map Contract** — preserves lineage from source files and tokens to canonical outcomes.
7. **Diagnostics / Reporting Surface** — makes conversion posture, unsupported behavior, and validation evidence auditable.

These entities belong in the requirements and planning surface first. Later implementation may mirror them in code, but must not invent materially different subsystem boundaries without review.

## Data flow
1. **Legacy source intake**
   - inspect pilot PCC/LST inputs already bounded by GE-01
2. **Lexical / structural parsing**
   - parse files into structured representations with source identity and spans when available
3. **Semantic routing**
   - resolve token families or constructs through explicit registry entries and conversion-handler boundaries
4. **Canonical projection**
   - emit canonical target intents, source-map records, and unsupported/lossy diagnostics using accepted GE-02 model homes while preserving remaining unresolved GE-02-adjacent details
5. **Coverage / validation output**
   - emit conversion reports, unsupported-token reports, fixture expectations, and future oracle comparison hooks
6. **Future implementation handoff**
   - once GE-03 is reviewed and dependencies are honest, later code-authorizing handoffs can target bounded importer epics rather than coding from this source STC directly

## Component boundaries

### PCC Parser Boundary
- responsibilities:
  - represent campaign entry files, includes, and package-level structural semantics relevant to the pilot
  - preserve source identity and include relationships for later provenance
- inputs:
  - GE-01 corpus inventory
  - pilot PCC files
  - loader/reference findings
- outputs:
  - structured campaign or package-entry parse results
- must not own:
  - canonical model design
  - token-family semantic conversion

### LST Parser Boundary
- responsibilities:
  - represent object records, fields, and legacy constructs relevant to pilot LST files
  - preserve ordering or span context when available
- inputs:
  - pilot LST files
  - token-family documentation
  - loader/reference findings
- outputs:
  - structured list-file parse results consumable by semantic handlers
- must not own:
  - final conversion semantics
  - coverage-claim logic by itself

### Structured Parse Representation
- responsibilities:
  - provide stable intermediate shape between syntax and semantic conversion
  - preserve source provenance and construct identity strongly enough for debugging
- inputs:
  - PCC/LST parse results
- outputs:
  - parse nodes, events, or equivalent structured records
- must not own:
  - canonical-target policy
  - silent loss handling

### Token Registry
- responsibilities:
  - classify legacy token families and construct classes by meaning, handler ownership, and risk
  - define the routing layer between parsed constructs and semantic conversion handlers
- inputs:
  - GE-01 token taxonomy and matrix posture
  - structured parse representation
  - legacy token documentation
- outputs:
  - explicit handler-routing metadata and validation obligations
- must not own:
  - conversion-report summarization by itself

### Conversion Handlers
- responsibilities:
  - interpret structured legacy inputs and project them into canonical target intents or explicit diagnostics
  - preserve provenance and support disposition through conversion
- inputs:
  - structured parse records
  - token-registry entries
  - accepted GE-02 source STC artifacts where importer-facing canonical targets are grounded
- outputs:
  - canonical target intents
  - unsupported/lossy diagnostics
  - source-map events
- must not own:
  - invention of final schema, expression/evaluator, or runtime-engine details beyond accepted GE-02 planning artifacts

### Provenance / Source-Map Contract
- responsibilities:
  - define minimum lineage fields preserved through parse and conversion
  - define downgrade behavior when only partial source precision is available
- inputs:
  - GE-01 provenance posture
  - parse-stage outputs
  - reference-architecture guidance
- outputs:
  - explicit provenance obligations for downstream importer work
- must not own:
  - final storage format choice

### Diagnostics / Reporting Surface
- responsibilities:
  - define how coverage, unsupported behavior, and caveats become auditable outputs
  - support later fixture-driven and oracle-backed validation without overstating success
- inputs:
  - conversion outcomes
  - unsupported diagnostics
  - validation expectations
- outputs:
  - conversion reports
  - unsupported-token reports
  - validation evidence expectations
- must not own:
  - parity claims unsupported by evidence

## Data and schema notes
Key documentary entities:

- **ParseRecord**
  - `source_path`
  - `source_kind`
  - `record_kind`
  - `span_or_location`
  - `raw_construct`
  - `normalized_fields`

- **RegistryEntry**
  - `legacy_family`
  - `applicable_source_kind`
  - `meaning`
  - `handler_owner`
  - `risk_level`
  - `validation_obligation`

- **ConversionIntent**
  - `legacy_reference`
  - `target_concept`
  - `support_disposition`
  - `lossiness_class`
  - `dependency_on_ge02`
  - `notes`

- **SourceMapEntry**
  - `pcc_path`
  - `lst_path`
  - `span_or_location`
  - `registry_entry`
  - `handler_identity`
  - `canonical_target`
  - `diagnostic_outcome`

- **UnsupportedDiagnostic**
  - `legacy_construct`
  - `source_reference`
  - `severity`
  - `reason`
  - `recommended_owner`
  - `linked_ge01_ledger_posture`

- **ConversionReportRecord**
  - `coverage_scope`
  - `mapped_exact`
  - `mapped_partial`
  - `unsupported`
  - `intentionally_ignored`
  - `validation_evidence`
  - `caveats`

## External dependencies and references
- `programs/codex/plans/spec-domains/GE-03-pcgen-import-pipeline-and-provenance.md` — strategic source artifact
- `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/README.md` — accepted canonical-model source STC
- `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/canonical-model-specification.md` — importer-facing canonical model homes and relationships
- `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/content-package-layout-specification.md` — package layout expectations
- `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/provenance-source-map-specification.md` — provenance/source-map obligations
- `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/content-validation-and-diagnostics-specification.md` — validation and diagnostic classes
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/README.md` — upstream control-plane input
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/technical-design.md` — importer-boundary and provenance posture inherited from GE-01
- `programs/codex/requirements/GE-00-program-governance-and-scope/README.md` — non-negotiables, including headless core first and no unsupported-token silence
- `programs/codex/research/codex-reference-architecture-2026-06-17.md` — staged parser/import pipeline and provenance/reference guidance

## Design decisions already fixed
- Codex replaces legacy project naming in the new authority surface.
- The PCGen repo is read-only reference/oracle material.
- GE-03 is the first importer-boundary spec domain, but this source STC is still not an implementation handoff.
- The importer is a compatibility bridge, not a new authoring substrate.
- Unsupported, lossy, deferred, or intentionally ignored behavior must remain visible as first-class diagnostics.
- Parser stages and semantic conversion stages remain separate.

## Deferred design decisions
- exact structured representation form: token stream, AST, typed AST, semantic events, or layered combination
- exact production schema details beyond GE-02 pilot model homes
- exact formula and prerequisite evaluator implementation pending later model/runtime decisions
- exact report serialization format and CLI surface
- exact source-span precision achievable in the first parser milestone
- exact scope of the first parity-harness automation pass

## Failure modes and observability
- **Failure mode:** the importer recreates PCGen syntax in a new container instead of aiming at the canonical model.
  - **Required signal:** every handler boundary references accepted GE-02 canonical target concepts and remaining GE-02-adjacent gaps stay explicit.
- **Failure mode:** unsupported behavior disappears during parsing or translation.
  - **Required signal:** every unresolved construct becomes an explicit unsupported diagnostic and remains linkable to GE-01 control-plane posture.
- **Failure mode:** provenance collapses before debugging or parity review.
  - **Required signal:** source-map obligations are explicit before implementation begins and downgrade behavior is named.
- **Failure mode:** registry and handler boundaries blur until responsibility is unreviewable.
  - **Required signal:** token-family routing, handler ownership, and validation obligations are named separately.
- **Failure mode:** GE-03 starts writing code from a planning bundle.
  - **Required signal:** no code-authorizing handoff exists until branch/worktree/write-scope and bounded implementation slice are explicit.

## Verification implications
`acceptance-and-verification.md` must prove that this design yields:

- a complete GE-03 source-STC bundle
- explicit parser-stage and structured-representation obligations
- explicit registry, handler, provenance, diagnostic, and reporting boundaries
- honest use of accepted GE-02 artifacts with remaining model/runtime gaps preserved
- an implementation decomposition that routes importer work into bounded downstream epics
- no counterfeit code authority

## Change constraints
- Do not assume the canonical model is resolved beyond what the accepted GE-02 source STC and artifacts actually state.
- Do not collapse parse, semantic conversion, provenance, and reporting into one vague subsystem.
- Do not collapse requirements into design or design into implementation handoff.
- Do not claim parity or importer success from planning artifacts.
- Do not let this STC drift into repo-local implementation documentation before a real implementation handoff exists.
