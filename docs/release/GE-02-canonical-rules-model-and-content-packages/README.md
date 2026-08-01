---
stc_id: STC-CODEX-GE-02
stc_kind: source-requirements
template_version: 2
work_type: planning-only
workflow_route: planning
readiness: planning-ready
status: active
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/README.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch: main is the current grounded checkout; execution branch/worktree remains handoff-specific
  write_scope: none during source STC generation; any future code-authorizing GE-02 handoff must declare bounded repo paths explicitly
review_state: accepted
last_reviewed_at: 2026-06-19
parent_scopes:
  - programs/codex
source_artifacts:
  - programs/codex/plans/spec-domains/GE-02-canonical-rules-model-and-content-packages.md
  - programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/README.md
  - programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-corpus-inventory.csv
  - programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-token-taxonomy.csv
  - programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv
  - programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv
  - programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/references/oracle-surface-inventory.md
  - programs/codex/requirements/GE-00-program-governance-and-scope/README.md
  - programs/codex/research/codex-reference-architecture-2026-06-17.md
related_artifacts:
  - programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md
  - programs/codex/plans/spec-domains/GE-03-pcgen-import-pipeline-and-provenance.md
  - programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/README.md
  - programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/artifacts/ge02-dependency-reconciliation-2026-06-19.md
upstream_targets:
  - programs/codex/README.md
  - programs/codex/requirements/README.md
supersedes: []
superseded_by: []
tags:
  - codex
  - ge-02
  - canonical-model
  - content-packages
  - rules-model
  - provenance
  - pf1
---

# GE-02 — Canonical Rules Model and Content Packages

## Objective
Define the source requirements construct for the Codex canonical rules substrate: versioned content packages, stable object IDs, pilot object homes, declarative effects, prerequisites, formulas, choice sets, diagnostics, compiled runtime IR boundaries, and legacy provenance lineage.

## Deliverable Type
`planning-only`

## Workflow Route
`planning`

## Readiness
`planning-ready`

Why this readiness is accurate:
- the strategic GE-02 spec domain exists and explicitly identifies this work as a boundary object, not implementation authorization
- GE-01's accepted documentary outputs exist and provide the governed pilot inventory, taxonomy, conversion matrix, unsupported-token ledger, and oracle surfaces required to seed GE-02
- the local Codex implementation checkout is grounded at `/home/ubuntu/workspace/repos/codex`, but this STC grants no repo-code write authority
- this bundle names canonical model homes for the PF1 Human Fighter level 1 pilot without pretending that final schemas, expression language, runtime engine, or parser implementation are solved
- future implementation work can now derive bounded planning or coding handoffs without inventing the canonical-model dependency that previously blocked GE-03

## Closure State
GE-02 is generated and closed as a planning-ready source STC for the PF1 Core Rulebook Human Fighter level 1 pilot boundary as of 2026-06-19. It uses GE-01's accepted governed inputs as its source truth. It does **not** authorize implementation code, final schema publication, expression-language selection, runtime engine work, or broad Pathfinder support.

## Authority and Scope
- owner: `Todd Hintzmann`
- scope: `program`
- canonical: `true`
- canonical path: `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/README.md`
- parent scopes:
  - `programs/codex`

This STC governs the Codex canonical rules-model planning surface. PCGen remains the heritage corpus and oracle substrate. GE-02 defines the clean target semantics that future importer, engine, validation, and UI work must aim at rather than cloning PCGen LST syntax.

## Target Runtime
- repo: `/home/ubuntu/workspace/repos/codex`
- workdir: `/home/ubuntu/workspace/repos/codex`
- branch/worktree: `main is currently grounded locally; execution branch/worktree remains unresolved until a later implementation handoff`
- allowed write scope: `none during source STC generation; future GE-02 implementation handoff must bound repo paths explicitly`

This bundle is an authority surface under `programs/codex/requirements/`, not a repo-local implementation brief.

## Document Map
- `technical-requirements.md` — normative requirements for package identity, stable IDs, pilot object model, effects, prerequisites, formulas, choice sets, provenance, IR boundary, diagnostics, and validation posture
- `technical-design.md` — architecture/design response describing the model surface and subsystem boundaries separately from normative requirements
- `acceptance-and-verification.md` — observable checks proving GE-02 can represent the pilot conceptually without LST syntax cloning and with source lineage preserved
- `risks-and-open-questions.md` — unresolved modeling questions, expression-language tradeoffs, package format uncertainty, and schema-boundary gaps
- `epic-breakdown.md` — downstream implementation-facing epics and feature seeds derived from the source STC while remaining upstream of any code-authorizing handoff
- `references/ge01-governed-inputs.md` — trace from GE-01 inventory, taxonomy, matrix, ledger, and oracle surfaces into GE-02 requirements
- `references/ge03-importer-dependency-contract.md` — importer-facing dependency contract that maps GE-01 evidence and GE-02 generated artifacts into GE-03 obligations without granting implementation authority
- `artifacts/canonical-model-specification.md` — concrete GE-02 canonical model homes and required relationships
- `artifacts/content-package-layout-specification.md` — concrete package layout, manifest, section, and validation expectations
- `artifacts/pilot-object-examples.yaml` — documentary pilot object skeletons for source package, Human, Fighter, equipment, effects, prerequisites, formulas, choices, source maps, and diagnostics
- `artifacts/provenance-source-map-specification.md` — concrete lineage/source-map fields, diagnostic linkage, downgrade policy, and oracle-linkage posture
- `artifacts/expression-language-decision-criteria.md` — decision criteria for later prerequisite/formula expression technology
- `artifacts/compiled-ir-boundary-definition.md` — source-package versus compiled-runtime-IR authority boundary
- `artifacts/content-validation-and-diagnostics-specification.md` — validation classes, diagnostic classes, and parity-prohibition requirements
- `review-handoff.md` — derived non-code review handoff for reconciling GE-03 after GE-02 closure; not a coding execution handoff

## Required Reads
- `../../plans/spec-domains/GE-02-canonical-rules-model-and-content-packages.md` — primary strategic authority for this canonical-model source STC
- `../GE-01-legacy-corpus-and-conversion-matrix/README.md` — accepted upstream closure state and migration-control posture
- `references/ge01-governed-inputs.md` — GE-01 input usage map for this STC
- `../GE-00-program-governance-and-scope/README.md` — inherited program non-negotiables and pilot boundary
- `../../plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md` — pilot target and downstream ownership split

## Conditional Reads
- `../GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-corpus-inventory.csv` — only when exact pilot source files, include edges, or object classes must be verified
- `../GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-token-taxonomy.csv` — only when token-family criticality or downstream owner must be verified
- `../GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv` — only when mapping from legacy construct to canonical target concept must be verified
- `../GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv` — only when unresolved prerequisite, formula, choice, trait, proficiency, or source-span debt must be grounded
- `../GE-01-legacy-corpus-and-conversion-matrix/references/oracle-surface-inventory.md` — only when source-truth, semantic documentation, or behavioral-comparison surfaces must be cited
- `references/ge03-importer-dependency-contract.md` — when evaluating or deriving GE-03 importer planning, parser/converter targets, source-map obligations, unsupported-token reporting, or downstream implementation boundaries
- `/home/ubuntu/workspace/repos/codex/AGENTS.md` — only if a later session proposes repo-facing implementation work

## In Scope
- Codex GE-02 source-STC documents under `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/`
- concrete GE-02 generated documentary artifacts under `programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/`
- package manifest and source-package identity requirements
- stable ID convention requirements for imported and future native-authored content
- pilot object homes for source package, race, class, feat, skill, equipment, ability score, save, proficiency, effect, prerequisite, formula, choice set, diagnostic, provenance/source map, and runtime IR boundary concepts
- requirements for representing the PF1 Core Rulebook Human Fighter level 1 pilot conceptually without direct LST syntax copying
- validation and diagnostic requirements that preserve unsupported/lossy/deferred behavior as first-class evidence
- downstream epic decomposition for later bounded implementation handoffs

## Out of Scope
- writing implementation code in `/home/ubuntu/workspace/repos/codex`
- modifying `/home/ubuntu/workspace/repos/pcgen`
- final expression-language selection or runtime evaluator implementation
- complete schema for all game systems or full Pathfinder support
- public package registry, plugin ABI, or advanced homebrew editor UX
- claiming import, engine, parity, or UI readiness from this planning bundle alone

## Acceptance Summary
The acceptance criteria in `acceptance-and-verification.md` are satisfied for the GE-02 source-STC closure boundary.

Compact summary:
- the concrete GE-02 generated documentary artifact set exists under `artifacts/`
- the canonical rules-model problem is now explicit and separated from PCGen syntax cloning
- every GE-01 pilot-critical target concept has a GE-02 model home or an explicit unresolved modeling debt entry
- effects, prerequisites, formulas, choice sets, source packages, provenance, diagnostics, authoring format, and compiled IR boundaries are named as first-class requirements
- the PF1 Human Fighter pilot can be described in model terms while preserving source-lineage obligations
- downstream GE-03 importer work no longer has to invent the canonical-target dependency from a spec domain alone
- downstream GE-03 importer work now has an explicit dependency contract that states which GE-02 artifacts it may rely on and which final schema/runtime facts remain unresolved

## Allowed Assumptions
- the PF1 Core Rulebook Human Fighter level 1 pilot remains the first proof target unless a higher-order decision surface changes it
- PCGen source rows are legacy evidence and oracle substrate, not the Codex canonical data model
- GE-01's accepted artifact set is sufficient as a governed input for GE-02 pilot-boundary planning
- future implementation may evolve exact code shape, but it must preserve the source requirements and provenance obligations unless a later decision record supersedes them

## Blockers / Forbidden Assumptions
- stop if a future handoff treats this source STC as code write authority without branch/worktree/write-scope and bounded implementation slice
- do not flatten formulas, prerequisites, choice sets, grant carriers, trait replacements, or type selectors into prose-only requirements
- do not clone LST token syntax into a new canonical container and call it the Codex model
- do not claim final schema, expression-language, engine, importer, validation, or parity readiness from this STC alone
- do not expand beyond the pilot slice without an explicit upstream decision surface

## Next Stage Rule
- GE-02 is planning-ready as a source requirements construct because its own generated documentary outputs now exist, not merely because the STC bundle exists.
- The next truthful downstream move is to re-audit GE-03 against this GE-02 source STC and its `artifacts/` outputs, then derive bounded implementation handoffs only after the target slice, write scope, verification commands, and handoff-specific runtime facts are explicit.
- Any future code-authorizing GE-02 handoff must be narrower than this spec domain and must name exact repo paths and tests before code work begins.
