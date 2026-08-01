---
stc_id: STC-CODEX-GE-05
stc_kind: source-requirements
template_version: 2
work_type: planning-only
workflow_route: planning
readiness: planning-ready
status: active
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/README.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch: current local checkout observed 2026-06-20 is ge04-e1-f1-character-input-record-shape; GE-05 execution branch/worktree remains unresolved until a later implementation handoff
  write_scope: source STC itself grants none; any future GE-05 implementation handoff must declare bounded repo paths explicitly
review_state: draft
last_reviewed_at: 2026-06-20
parent_scopes:
  - programs/codex
source_artifacts:
  - programs/codex/plans/spec-domains/GE-05-oracle-validation-and-parity-harness.md
  - programs/codex/plans/roadmaps/codex-spec-domain-roadmap-2026-06-18.md
  - programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/README.md
  - programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/references/oracle-surface-inventory.md
  - programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/README.md
  - programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/technical-requirements.md
  - programs/codex/requirements/GE-04-rules-engine-and-explainability-core/README.md
  - programs/codex/requirements/GE-04-rules-engine-and-explainability-core/technical-requirements.md
  - programs/codex/requirements/GE-04-rules-engine-and-explainability-core/artifacts/pilot-golden-computation-fixture-requirements.md
  - programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md
  - programs/codex/doctrine/quality-gate-policy.md
related_artifacts:
  - programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/README.md
  - programs/codex/requirements/GE-00-program-governance-and-scope/README.md
  - programs/codex/doctrine/program-doctrine-and-scope-charter.md
  - programs/codex/doctrine/documentation-control-plane.md
  - programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f1-handoff-readiness-closure-2026-06-20.md
  - programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/research-handoff.md
upstream_targets:
  - programs/codex/README.md
  - programs/codex/requirements/README.md
expected_output_artifacts:
  - path: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/oracle-strategy-specification-requirements.md
    completion_rule: Defines the oracle strategy, trust tiers, command-discovery obligations, and evidence boundaries without inventing a final PCGen invocation.
  - path: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/golden-case-fixture-format.md
    completion_rule: Defines the fixture schema for reproducible old-vs-new comparison cases, including pilot case identity, inputs, output references, normalization declarations, and known-gap linkage.
  - path: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/parity-report-format.md
    completion_rule: Defines the report schema for comparisons, diffs, evidence references, claim tier, diagnostics, and known gaps.
  - path: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/initial-human-fighter-l1-expected-output-source-requirements.md
    completion_rule: Defines source requirements for the first PF1 Human Fighter level 1 expected-output fixture without fabricating final legacy or new-system values.
  - path: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/known-gap-policy.md
    completion_rule: Defines how non-comparable, unsupported, undesirable, or oracle-unavailable behavior is recorded, routed, and prevented from becoming a false parity claim.
supersedes: []
superseded_by: []
tags:
  - codex
  - ge-05
  - oracle
  - parity
  - validation
  - golden-fixtures
  - pcgen
  - pf1
---

# GE-05 — Oracle Validation and Parity Harness

## Objective
Define the source requirements construct for using legacy PCGen as a bounded behavior oracle so Codex can compare loaded content, choices, derived values, diagnostics, and pilot export/stat-block outputs against real legacy evidence rather than plausible-looking new-system behavior.

## Deliverable Type
`planning-only`

## Workflow Route
`planning`

## Readiness
`planning-ready`

Why this readiness is accurate:
- the strategic GE-05 spec domain exists and explicitly frames this work as a boundary object, not an implementation prompt
- GE-01 provides a grounded oracle-surface inventory and preserves the warning that runtime character-generation output has not yet been grounded
- GE-03 defines the importer/provenance requirements that future new-system output must preserve before parity can be claimed
- GE-04 defines the deterministic rules-computation and golden-fixture requirements that GE-05 must compare against PCGen evidence
- this bundle creates the GE-05 control documents and same-epic documentary artifact specifications required by the spec domain
- the local Codex implementation checkout is grounded, but this STC grants no code write authority and no GE-05 implementation branch, write scope, final PCGen command, or verification command set has been authorized

## Closure State
GE-05 is generated as a planning-ready source STC for the oracle-validation and parity-harness requirements boundary as of 2026-06-20. It defines what evidence must exist before Codex may claim behavior is oracle-checked. It does **not** prove parity, choose a final PCGen execution route, authorize GUI automation, settle final output normalization, or create a code-authorizing handoff.

## Authority and Scope
- owner: `Todd Hintzmann`
- scope: `program`
- canonical: `true`
- canonical path: `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/README.md`
- parent scopes:
  - `programs/codex`

This STC governs the Codex oracle-parity planning surface. GE-01 owns legacy corpus and candidate oracle-surface discovery. GE-03 owns import/provenance bridge truth. GE-04 owns new-system computation and explanation output truth. GE-05 owns the comparison standard, claim boundaries, report shape, diff obligations, and known-gap policy that prevent counterfeit compatibility claims. GE-06 owns the integrated pilot vertical slice that consumes this evidence.

## Target Runtime
- repo: `/home/ubuntu/workspace/repos/codex`
- workdir: `/home/ubuntu/workspace/repos/codex`
- branch/worktree: `current local checkout observed 2026-06-20 is ge04-e1-f1-character-input-record-shape; GE-05 execution branch/worktree remains unresolved until a later implementation handoff`
- allowed write scope: `none during source STC generation; future GE-05 implementation handoff must bound repo paths explicitly`

The target repo is grounded only as the future implementation surface. This package is a requirements authority surface under `programs/codex/requirements/`, not a repo-local implementation brief.

## Document Map
- `technical-requirements.md` — normative requirements for oracle invocation, golden-case fixtures, PCGen output capture, new-system output capture, normalization, comparison dimensions, parity reports, actionable diffs, known gaps, and claim tiers
- `technical-design.md` — architecture/design response describing the parity-harness pipeline, subsystem boundaries, evidence flow, normalizer posture, and claim-control model separately from normative requirements
- `acceptance-and-verification.md` — observable checks proving the GE-05 source STC and generated artifacts define a falsifiable parity standard without inventing PCGen runtime evidence
- `risks-and-open-questions.md` — unresolved oracle-command, GUI-driving, normalization, licensing, known-gap, and non-comparable-output questions
- `epic-breakdown.md` — downstream implementation-facing epics and feature seeds for later bounded handoff derivation
- `research-handoff.md` — active non-code discovery handoff for GE05-E1-F1 candidate PCGen oracle-route inventory
- `collection-handoff.md` — active non-code collection handoff for GE05-E1-F2 first reproducible old-system output route
- `execution-handoff.md` — stable GE-05 coding route surface; currently `no-active-handoff`, so it does not itself authorize Claude Code or any other coding harness
- `references/upstream-dependency-contract.md` — compact contract mapping GE-01, GE-03, GE-04, the pilot charter, and the quality gate policy into GE-05 obligations
- `artifacts/oracle-strategy-specification-requirements.md` — concrete GE-05 oracle strategy and evidence-standard requirements
- `artifacts/golden-case-fixture-format.md` — concrete golden-case fixture format requirements
- `artifacts/parity-report-format.md` — concrete parity-report schema and diff-reporting requirements
- `artifacts/initial-human-fighter-l1-expected-output-source-requirements.md` — concrete source requirements for the first Human Fighter level 1 expected-output fixture
- `artifacts/known-gap-policy.md` — concrete policy for non-comparable, undesirable, unsupported, and oracle-unavailable behavior
- `artifacts/ge05-e1-f1-handoff-readiness-closure-2026-06-20.md` — handoff-readiness closure establishing that GE05-E1-F1 is ready only as a non-code discovery/research handoff and that GE05-E1-F2/runtime-output work remains blocked until Java/runtime and oracle-route facts are grounded
- `artifacts/ge05-e1-f2-handoff-readiness-closure-2026-06-20.md` — handoff-readiness closure establishing that GE05-E1-F2 is collection-ready, that retention and write-scope policy are now grounded, and that the derived collection handoff may use one explicitly provisional pilot `.pcg` if no authoritative pilot file exists
- `artifacts/ge05-e1-f2-runtime-output-attempt-2026-06-20.md` — runtime-output receipt for the first GE05-E1-F2 collection attempt; PCGen started headless batch export under Java 25, but no XML was produced because the provisional `.pcg` used older/incorrect campaign and game-mode identity strings
- `artifacts/ge05-e1-f2-runtime-output-attempt-2-2026-06-20.md` — follow-up runtime-output receipt proving that `CAMPAIGN:Core Rulebook` loads sources, while `GAMEMODE:Pathfinder` is rejected by the character loader
- `artifacts/ge05-e1-f2-runtime-output-attempt-3-2026-06-20.md` — successful runtime-output receipt proving the headless PCGen XML export path with `CAMPAIGN:Core Rulebook` and `GAMEMODE:Pathfinder_RPG`; raw XML remains local/generated and is referenced by SHA-256 only
- `artifacts/ge05-e2-f1-execution-readiness-closure-2026-06-20.md` — execution-readiness closure that grounded GE05-E2-F1 for a code-authorizing handoff without granting code authority by itself
- `artifacts/ge05-e2-f1-execution-handoff-2026-06-20.md` — preserved historical code-authorizing handoff for the merged GE05-E2-F1 golden-case fixture schema slice
- `artifacts/ge05-e2-f1-merge-receipt-2026-06-21.md` — verified merge receipt proving GE05-E2-F1 landed on `develop`, passed detached cargo tests, and retired active code authority back to the route surface
- `artifacts/pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg` — provisional pilot input created for GE05-E1-F2 runtime evidence only; it is not canonical GE-06 pilot truth

## Expected Output Artifacts
| Artifact | Completion rule |
|---|---|
| `artifacts/oracle-strategy-specification-requirements.md` | Defines oracle surfaces, trust tiers, command-discovery obligations, claim tiers, and evidence boundaries without inventing the final PCGen invocation. |
| `artifacts/golden-case-fixture-format.md` | Defines a reproducible fixture schema for old-vs-new comparison cases, including inputs, output references, normalization declarations, and known-gap linkage. |
| `artifacts/parity-report-format.md` | Defines a report schema that records compared dimensions, old/new values or references, diffs, evidence, diagnostics, claim tier, and known gaps. |
| `artifacts/initial-human-fighter-l1-expected-output-source-requirements.md` | Defines the source requirements for the first PF1 Human Fighter level 1 expected-output fixture without fabricating final expected values. |
| `artifacts/known-gap-policy.md` | Defines how non-comparable, unsupported, undesirable, oracle-unavailable, or normalization-blocked outputs are recorded and prevented from becoming false parity claims. |

## Required Reads
- `../../plans/spec-domains/GE-05-oracle-validation-and-parity-harness.md` — primary strategic authority for this source STC
- `../../plans/roadmaps/codex-spec-domain-roadmap-2026-06-18.md` — roadmap dependency and exit-gate authority for Stage C behavior proof
- `../GE-01-legacy-corpus-and-conversion-matrix/README.md` — accepted legacy-corpus and oracle-surface discovery posture
- `../GE-01-legacy-corpus-and-conversion-matrix/references/oracle-surface-inventory.md` — grounded and candidate PCGen oracle surfaces, including the ungrounded runtime-output warning
- `../GE-03-pcgen-import-pipeline-and-provenance/README.md` — importer/provenance bridge posture and diagnostics expected before new-system outputs can be trusted
- `../GE-03-pcgen-import-pipeline-and-provenance/technical-requirements.md` — provenance, conversion-report, fixture, and diagnostic obligations inherited by parity comparison
- `../GE-04-rules-engine-and-explainability-core/README.md` — rules-engine and explanation boundary that GE-05 compares against PCGen
- `../GE-04-rules-engine-and-explainability-core/technical-requirements.md` — deterministic computation, explanation, diagnostics, fixture, and headless-entry obligations inherited by parity comparison
- `../GE-04-rules-engine-and-explainability-core/artifacts/pilot-golden-computation-fixture-requirements.md` — initial new-system golden computation fixture requirements and GE-05 boundary
- `../../plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md` — first pilot case boundary and initial acceptance target
- `../../doctrine/quality-gate-policy.md` — oracle parity gate and compatibility claim-tier doctrine

## Conditional Reads
- `/home/ubuntu/workspace/repos/codex/AGENTS.md` — only if a later session proposes repo-facing GE-05 implementation work or an actual code-authorizing handoff
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc` — only if campaign-load or source-package count comparison details must be grounded
- `/home/ubuntu/workspace/repos/pcgen/docs/listfilepages/listfileimportanttoknow.html` — only if token or list-file documentation is needed to interpret a candidate oracle output
- PCGen runtime, export, validation, or script surfaces inside `/home/ubuntu/workspace/repos/pcgen` — only during a bounded oracle-command discovery run; this source STC does not assert the final command path
- `../../doctrine/decisions/` — when a comparison reveals known PCGen behavior that Codex intentionally should not preserve

## In Scope
- Codex GE-05 source-STC documents under `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/`
- concrete GE-05 generated documentary artifacts under `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/`
- oracle-runner and command-discovery requirements
- golden-case fixture format requirements
- PCGen output capture and new-system output capture requirements
- output normalization requirements and explicit non-comparable-output handling
- comparison dimensions for loaded content counts, choice availability, derived values, and limited export/stat-block outputs where practical
- parity report schema and actionable diff requirements
- known-gap policy for non-comparable, undesirable, unsupported, lossy, or oracle-unavailable behavior
- downstream epic decomposition for later bounded GE-05 implementation handoffs

## Out of Scope
- writing parity-harness implementation code in `/home/ubuntu/workspace/repos/codex`
- modifying `/home/ubuntu/workspace/repos/pcgen`
- claiming PCGen parity, oracle-checked behavior, or product-visible correctness from this planning bundle alone
- selecting a final PCGen command, validation task, export path, or GUI automation route without a bounded discovery pass
- perfect parity for every PCGen behavior or every Pathfinder path
- full export-sheet compatibility
- broad regression suite beyond the PF1 Human Fighter level 1 pilot and immediate expansion needs
- release governance owned by GE-09

## Acceptance Summary
The acceptance criteria in `acceptance-and-verification.md` are satisfied for the planning-ready source-STC boundary when this bundle and its generated documentary artifacts exist and remain internally linked.

Compact summary:
- PCGen is documented as a bounded oracle substrate, not as the Codex runtime architecture
- comparison claims must name exact scope, outputs, evidence, normalization, and known gaps
- the first pilot comparison path is framed around the PF1 Core Rulebook Human Fighter level 1 case without inventing final old-system or new-system values
- parity failures are required to produce actionable diffs, not vague failure banners
- non-comparable outputs are required to become known-gap records or decision records, not silent exclusions
- downstream implementation work can be decomposed into bounded epics without inventing final PCGen command, branch/worktree, write scope, verifier commands, GUI automation, normalization rules, or parity evidence

## Allowed Assumptions
- the PF1 Core Rulebook Human Fighter level 1 pilot remains the first oracle-check target unless a higher-order decision surface changes it
- legacy PCGen is an oracle substrate for bounded evidence, not an architecture to copy
- GE-01's oracle-surface inventory is sufficient to seed GE-05 planning but does not prove a runtime output path
- GE-03 importer outputs and GE-04 rules-engine outputs must preserve provenance, diagnostics, and explanation data strongly enough for comparison, but this STC must not assume those implementations are complete
- future implementation may choose exact serialization formats and comparator tooling, but it must preserve the report, diff, known-gap, and claim-tier obligations defined here unless a later decision record supersedes this STC

## Blockers / Forbidden Assumptions
- stop if a future handoff treats this source STC as code write authority without exact branch/worktree, allowed write scope, selected implementation slice, and verification commands
- do not claim parity from static PCGen source files alone
- do not claim oracle-checked behavior until a reproducible old-vs-new comparison artifact exists
- do not hide non-comparable outputs by omitting them from reports
- do not allow GUI automation to become the default oracle route unless no lower-friction headless/export/script route is available and the decision is recorded
- do not copy PCGen semantics into Codex merely because parity comparison revealed them; undesirable legacy behavior requires a decision record and a known-gap/intentional-divergence entry
- do not fabricate final Human Fighter expected values, output normalization rules, or comparison tolerances before oracle and new-system outputs are grounded

## Next Stage Rule
- GE-05 remains planning-ready as a source requirements construct because its control bundle and same-epic generated documentary outputs still govern the parity boundary.
- The original source-STC generation pass did not create code authority. Later readiness and collection work grounded the GE05-E2-F1 coding slice, and that slice has now merged.
- `artifacts/ge05-e2-f1-execution-handoff-2026-06-20.md` preserves the historical GE05-E2-F1 code-authorizing brief exactly as it was handed to the coding harness.
- `artifacts/ge05-e2-f1-merge-receipt-2026-06-21.md` now records that the merged slice landed on `develop`, the feature branch was deleted upstream, and detached cargo verification passed.
- `execution-handoff.md` is now a non-authorizing route surface with `status: running-under-card-triggered-harness`; it points at the live GE05-E2-F2 readiness/handoff pair and active Kanban card `t_0cdc64d0` rather than serving as code authority itself.
- `artifacts/ge05-e2-f2-execution-readiness-closure-2026-06-24.md` records why the next bounded GE-05 coding slice is now grounded.
- `artifacts/ge05-e2-f2-execution-handoff-2026-06-24.md` is the live stage-specific code-authorizing brief for `GE05-E2-F2 — PF1 Human Fighter level 1 governed fixture instance`.
- The current truthful next operator action is not more research and not Todd manually walking files; it is a governed Kanban CODE card that triggers Claude Code / frontier-harness execution automatically for the bounded E2-F2 coding packet.
- Any later GE-05 code-authorizing handoff must remain narrower than this spec domain and must identify exact repo paths, branch/worktree policy, allowed write scope, failing-first tests or discovery receipts, required legacy reads, expected report artifacts, and non-goals before code work begins.
- Any future GE-05 code authority must be minted as a **new** stage-specific artifact under `artifacts/`; do not retarget this root route surface into the next coding brief.
