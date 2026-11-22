---
stc_id: STC-CODEX-GE-08
stc_kind: source-requirements
template_version: 2
work_type: planning-only
workflow_route: planning
readiness: planning-ready
status: active
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/README.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch: current local repo state observed 2026-06-21 is branch `ge06-e2-f3-headless-receipt-path` at commit `2deb11b` with in-flight GE-06 work; future GE-08 implementation handoffs must not assume this branch is the authoring baseline and must instead ground against then-current accepted dependency state before naming a write branch
  write_scope: source STC itself grants none; any future GE-08 implementation handoff must declare exact repo paths, dependency baseline, and whether the slice is headless-only or UI-facing
review_state: draft
last_reviewed_at: 2026-06-21
parent_scopes:
  - programs/codex
source_artifacts:
  - programs/codex/plans/spec-domains/GE-08-homebrew-authoring-and-rules-studio.md
  - programs/codex/plans/roadmaps/codex-spec-domain-roadmap-2026-06-18.md
  - programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md
  - programs/codex/requirements/GE-00-program-governance-and-scope/README.md
  - programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/README.md
  - programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/technical-requirements.md
  - programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/canonical-model-specification.md
  - programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/expression-language-decision-criteria.md
  - programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/provenance-source-map-specification.md
  - programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/content-validation-and-diagnostics-specification.md
  - programs/codex/requirements/GE-04-rules-engine-and-explainability-core/README.md
  - programs/codex/requirements/GE-04-rules-engine-and-explainability-core/technical-requirements.md
  - programs/codex/requirements/GE-04-rules-engine-and-explainability-core/artifacts/diagnostic-schema.md
  - programs/codex/requirements/GE-04-rules-engine-and-explainability-core/artifacts/explanation-graph-schema.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md
  - programs/codex/plans/spec-domains/GE-07-desktop-shell-and-modern-ux.md
  - programs/codex/doctrine/program-doctrine-and-scope-charter.md
  - programs/codex/doctrine/documentation-control-plane.md
  - programs/codex/doctrine/quality-gate-policy.md
related_artifacts:
  - programs/codex/requirements/README.md
  - programs/codex/plans/spec-domains/README.md
  - programs/codex/plans/spec-domains/GE-09-expansion-packaging-and-release-governance.md
upstream_targets:
  - programs/codex/README.md
  - programs/codex/requirements/README.md
  - programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md
expected_output_artifacts:
  - path: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/homebrew-authoring-surface-specification.md
    completion_rule: Defines the minimum safe structured authoring surfaces, required authored object classes, edit operations, and proof boundary for ordinary homebrew without collapsing into a broad studio claim.
  - path: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/rules-studio-surface-definition.md
    completion_rule: Defines the required product/documentary surfaces for the future authoring workbench while refusing to let UI/editor flow become rules authority ahead of GE-07 source-STC grounding.
  - path: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/validation-and-preview-workflow-requirements.md
    completion_rule: Defines the authoring-to-validation-to-preview workflow, required diagnostic gates, compile-preview boundary, and explanation-preview obligations for safe rule authoring.
  - path: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/safe-expression-authoring-constraints.md
    completion_rule: Defines the permitted constrained-expression posture, refusal boundary against arbitrary scripting, and diagnostic/escalation rules for unsupported semantics.
  - path: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/package-file-lifecycle-requirements.md
    completion_rule: Defines the package create/edit/save/diff/import/export lifecycle, provenance expectations, and portability/versioning rules for authored content.
  - path: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/initial-homebrew-acceptance-cases.md
    completion_rule: Names the first bounded homebrew cases that later implementation must satisfy to prove value without requiring full studio breadth or plugin-first escape hatches.
  - path: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e1-minimum-proof-object-selection-2026-06-22.md
    completion_rule: Selects the first truthful homebrew proof object, closes the GE-06-derived pilot variant it rides on, and refuses broader rules-studio claims before that proof exists.
  - path: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/plugin-exception-boundary.md
    completion_rule: Defines when plugins are exceptional and allowed, what ordinary customization must achieve without them, and which open questions remain explicitly non-authorizing.
supersedes: []
superseded_by: []
tags:
  - codex
  - ge-08
  - homebrew
  - authoring
  - rules-studio
  - diagnostics
  - provenance
  - expression-language
  - plugin-boundary
---

# GE-08 — Homebrew Authoring and Rules Studio

## Objective
Define the constrained early source requirements for safe, structured, inspectable homebrew authoring so Codex can replace routine PCGen LST editing without defaulting to arbitrary scripting or premature UI-first studio claims.

## Deliverable Type
`planning-only`

## Workflow Route
`planning`

## Readiness
`planning-ready`

Why this readiness is accurate:
- the strategic GE-08 spec domain exists and clearly defines this work as a boundary object rather than an implementation prompt
- GE-02 already grounds the canonical package/model, stable-ID, provenance, diagnostics, and authoring-versus-compiled-IR boundaries that GE-08 must consume rather than reinvent
- GE-04 already grounds diagnostics, explanation outputs, and the headless compute/preview truth that authored rules must ultimately feed
- GE-06 exists as the integrated pilot proof surface, but the stack has not yet earned broad authoring claims; this STC therefore stays in a constrained early posture rather than pretending the pilot substrate is already fully proven
- GE-07 now exists as a planning-ready source STC, but it still does not authorize product-visible editor implementation; GE-08 therefore treats GE-07 as a real upstream authority surface while still refusing to authorize final editor UX, command-boundary, or desktop-shell implementation from GE-08 alone
- this bundle includes both the control documents and the same-epic documentary output artifacts needed to keep the authoring problem concrete instead of recursive

## Closure State
GE-08 is generated as a planning-ready source STC in a constrained early posture as of 2026-06-22. It defines what ordinary homebrew must be able to do without LST and without plugins as the default path. `artifacts/ge08-e1-minimum-proof-object-selection-2026-06-22.md` now fixes the first truthful proof case as a package-local feat-like authored object riding one bounded GE-06 pilot variant. This source STC still does not authorize implementation code, final editor UX, final expression-authoring ergonomics, public package sharing, or a plugin ABI.

## Authority and Scope
- owner: `Todd Hintzmann`
- scope: `program`
- canonical: `true`
- canonical path: `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/README.md`
- parent scopes:
  - `programs/codex`

This STC governs the Codex authoring foundation: structured homebrew package shape, safe edit surfaces, validation/preview posture, explanation-preview expectations, file lifecycle, and plugin-exception posture. GE-02 owns canonical model/package truth. GE-04 owns diagnostics, explanation, and compute truth. GE-06 owns integrated pilot proof. GE-07 now owns planning-ready desktop-shell/editor presentation truth, but that does not grant GE-08 product-visible implementation authority by implication.

## Target Runtime
- repo: `/home/ubuntu/workspace/repos/codex`
- workdir: `/home/ubuntu/workspace/repos/codex`
- branch/worktree: `current local repo state observed 2026-06-21 is branch ge06-e2-f3-headless-receipt-path at commit 2deb11b with in-flight GE-06 work; future GE-08 implementation handoffs must ground against then-current accepted dependency state rather than reusing this branch implicitly`
- allowed write scope: `none during source STC generation; any future GE-08 implementation handoff must declare exact repo paths, dependency baseline, and whether the slice is headless-only or UI-facing`

This bundle is an authority surface under `programs/codex/requirements/`, not a repo-local implementation brief.

## Document Map
- `technical-requirements.md` — normative requirements for ordinary homebrew authoring, structured package editing, constrained expressions, validation, preview, explanation, provenance, import/export, and plugin exception boundaries
- `technical-design.md` — architecture/design response describing the intended authoring pipeline and subsystem boundaries separately from the normative requirements
- `acceptance-and-verification.md` — observable checks proving the GE-08 source STC and same-epic documentary outputs define a falsifiable authoring foundation without counterfeit implementation claims
- `risks-and-open-questions.md` — preserves unresolved questions about expression ergonomics, GE-07 UX binding, package sharing, migration posture, and plugin exceptions after the first proof object was fixed
- `epic-breakdown.md` — downstream implementation-facing decomposition for later bounded readiness closure and handoff derivation
- `references/upstream-dependency-contract.md` — compact contract mapping what GE-00, GE-02, GE-04, GE-06, and GE-07 do and do not authorize for authoring work
- `artifacts/homebrew-authoring-surface-specification.md` — concrete ordinary-homebrew scope and edit-surface definition
- `artifacts/rules-studio-surface-definition.md` — concrete future workbench surface definition without granting UI authority
- `artifacts/validation-and-preview-workflow-requirements.md` — concrete workflow contract for validate/preview/explain loops
- `artifacts/safe-expression-authoring-constraints.md` — concrete expression-authoring safety and escalation rules
- `artifacts/package-file-lifecycle-requirements.md` — concrete package lifecycle and portability requirements
- `artifacts/initial-homebrew-acceptance-cases.md` — first bounded acceptance cases proving authoring value
- `artifacts/ge08-e1-minimum-proof-object-selection-2026-06-22.md` — documentary closure that selects the first feat-like proof object and binds it to a bounded GE-06 pilot variant plus GE-02/GE-04 obligations
- `artifacts/plugin-exception-boundary.md` — explicit plugin-as-exception boundary and escalation criteria

## Expected Output Artifacts
| Artifact | Completion rule |
|---|---|
| `artifacts/homebrew-authoring-surface-specification.md` | Defines the minimum safe structured authoring surfaces, required authored object classes, edit operations, and proof boundary for ordinary homebrew without broad studio inflation. |
| `artifacts/rules-studio-surface-definition.md` | Defines the documentary/product surfaces the future authoring workbench must expose while refusing to let UI/editor flow become rules authority ahead of GE-07 grounding. |
| `artifacts/validation-and-preview-workflow-requirements.md` | Defines authoring-to-validation-to-preview workflow, required diagnostics, compile-preview boundary, and explanation-preview obligations. |
| `artifacts/safe-expression-authoring-constraints.md` | Defines constrained-expression posture, anti-scripting boundary, and escalation rules for unsupported semantics. |
| `artifacts/package-file-lifecycle-requirements.md` | Defines create/edit/save/diff/import/export lifecycle, provenance expectations, and portability/versioning rules for authored packages. |
| `artifacts/initial-homebrew-acceptance-cases.md` | Names the first bounded homebrew cases that later implementation must satisfy to prove value without full studio breadth or plugin-first escape hatches. |
| `artifacts/ge08-e1-minimum-proof-object-selection-2026-06-22.md` | Selects the first truthful homebrew proof object and closes the GE-06-derived pilot variant and preview obligations it must use. |
| `artifacts/plugin-exception-boundary.md` | Defines when plugins are exceptional and allowed, what ordinary customization must achieve without them, and which open questions remain explicitly non-authorizing. |

## Required Reads
- `../../plans/spec-domains/GE-08-homebrew-authoring-and-rules-studio.md` — primary strategic authority for this source STC
- `../../plans/roadmaps/codex-spec-domain-roadmap-2026-06-18.md` — roadmap dependency and stage authority for GE-08
- `../GE-00-program-governance-and-scope/README.md` — inherited anti-LST-cloning, no-counterfeit-completion, and no-arbitrary-scripting posture
- `../GE-02-canonical-rules-model-and-content-packages/README.md` — canonical package/model planning boundary for authored content
- `../GE-02-canonical-rules-model-and-content-packages/technical-requirements.md` — normative package, stable-ID, provenance, expression, validation, and IR-boundary rules that authoring must inherit
- `../GE-02-canonical-rules-model-and-content-packages/artifacts/canonical-model-specification.md` — required canonical object homes and relationships that authored content must target
- `../GE-02-canonical-rules-model-and-content-packages/artifacts/expression-language-decision-criteria.md` — decision criteria that constrain future expression-authoring affordances
- `../GE-02-canonical-rules-model-and-content-packages/artifacts/provenance-source-map-specification.md` — provenance/source-map obligations for authored content and later debugging
- `../GE-02-canonical-rules-model-and-content-packages/artifacts/content-validation-and-diagnostics-specification.md` — validation and diagnostic classes authoring must preserve
- `../GE-04-rules-engine-and-explainability-core/README.md` — compute/explainability planning boundary that authored content must eventually feed
- `../GE-04-rules-engine-and-explainability-core/technical-requirements.md` — normative compute, diagnostics, explanation, and headless-preview obligations downstream of authoring
- `../GE-04-rules-engine-and-explainability-core/artifacts/diagnostic-schema.md` — concrete diagnostic taxonomy authoring flows must surface instead of hiding
- `../GE-04-rules-engine-and-explainability-core/artifacts/explanation-graph-schema.md` — explanation surface that preview flows must preserve
- `../GE-06-pilot-vertical-slice-pf1-human-fighter/README.md` — current integrated pilot proof boundary and current evidence ceiling
- `artifacts/ge08-e1-minimum-proof-object-selection-2026-06-22.md` — fixed first proof-object selection and GE-06-derived fixture closure for later bounded GE-08 slices
- `../GE-07-desktop-shell-and-modern-ux/README.md` — planning-ready UI/editor authority surface that GE-08 must align with before any product-visible authoring route is derived
- `../../plans/spec-domains/GE-07-desktop-shell-and-modern-ux.md` — strategic UI/editor boundary and original GE-07 problem statement
- `../../doctrine/quality-gate-policy.md` — claim-tier and evidence-gate doctrine for what authoring is allowed to claim at each stage

## Conditional Reads
- future GE-07 readiness closures or execution handoffs — mandatory before any product-visible editor or desktop-shell handoff is derived beyond the current planning-ready GE-07 source STC
- `../GE-06-pilot-vertical-slice-pf1-human-fighter/execution-handoff.md` — only when a later GE-08 slice needs to bind authoring proof to an active pilot implementation lane
- `../GE-05-oracle-validation-and-parity-harness/README.md` — only when authoring outputs are being routed into parity or old-vs-new comparison claims
- `/home/ubuntu/workspace/repos/codex/AGENTS.md` — only if a later session proposes repo-facing implementation work
- legacy PCGen homebrew or extension examples under `/home/ubuntu/workspace/repos/pcgen` — only when a later documentary or research pass needs concrete migration examples; this source STC does not claim those examples as the new model

## In Scope
- Codex GE-08 source-STC documents under `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/`
- concrete GE-08 same-epic documentary artifacts under `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/`
- data-first homebrew package authoring requirements
- structured package editing and file lifecycle requirements
- constrained-expression authoring posture for formulas and prerequisites
- validation, compile-preview, and explanation-preview requirements
- provenance, diffability, portability, and import/export requirements for authored content
- plugin exception boundaries and escalation criteria
- downstream epic decomposition for later bounded readiness closure and handoff derivation

## Out of Scope
- writing implementation code in `/home/ubuntu/workspace/repos/codex`
- modifying `/home/ubuntu/workspace/repos/pcgen`
- final desktop-shell/editor UX architecture, command boundary, or design system scope owned by GE-07
- public package registry, marketplace, or trust/distribution system
- arbitrary scripting as the ordinary authoring path
- final plugin ABI, plugin sandbox, or plugin marketplace
- broad multi-system support or full-path Pathfinder breadth
- claiming pilot viability, end-user usability, or parity from this planning bundle alone

## Acceptance Summary
The acceptance criteria in `acceptance-and-verification.md` are satisfied for the GE-08 planning-ready source-STC boundary when this bundle and its named same-epic documentary artifacts exist, resolve, and remain internally coherent.

Compact summary:
- ordinary homebrew is defined as a structured package-editing problem rather than an LST or plugin-first problem
- authoring inherits GE-02 canonical-model, provenance, diagnostics, and compiled-IR boundaries instead of cloning them locally
- validation, compile preview, and explanation preview are explicit authoring obligations rather than downstream afterthoughts
- plugins remain exceptional, not the required path for routine customization
- GE-07 and harder GE-06 evidence are acknowledged as real constraints instead of being silently bypassed

## Allowed Assumptions
- the PF1 Core Rulebook Human Fighter level 1 pilot remains the first concrete proof target unless a higher-order decision surface changes it
- GE-02 accepted artifacts remain authoritative planning inputs for package/model, stable-ID, provenance, diagnostics, and expression-boundary posture
- GE-04 remains authoritative for downstream compute/explanation/diagnostic truth; GE-08 may define preview requirements but not engine behavior
- GE-07 now exists as a planning-ready source STC, so GE-08 may align its product-surface expectations to that authority while still refusing to claim final UI/editor implementation authority from GE-08 alone
- ordinary customization should remain data-first, reviewable, diffable, and portable unless a later accepted decision record narrows that rule

## Blockers / Forbidden Assumptions
- stop if a future handoff treats this source STC as code write authority without exact repo paths, dependency baseline, allowed write scope, and verification commands
- do not let arbitrary scripting become the ordinary homebrew path merely because expression authoring is difficult
- do not treat GE-07's still-planning posture as permission to improvise final editor flow, command boundary, or desktop-shell architecture inside GE-08
- do not claim authored rules are safe unless validation, diagnostics, preview, and explanation obligations remain visible
- do not claim ordinary homebrew requires plugins; plugin use must remain an explicit exception path
- do not broaden the problem into public sharing, marketplace, or ecosystem policy without a higher-order decision surface

## Next Stage Rule
- GE-08 is planning-ready because both the source-STC control bundle and its same-epic documentary output artifacts now exist.
- `artifacts/ge08-e1-minimum-proof-object-selection-2026-06-22.md` now fixes the first truthful proof object and the bounded GE-06-derived pilot variant it must ride on.
- GE-08 has no active `execution-handoff.md`; this source STC does not authorize code by itself.
- The next truthful move is to derive narrow future readiness closures for non-UI GE08-E2/E3/E4 slices against the selected proof object, exact repo paths, dependency baseline, verification commands, and non-goals rather than reopening the object-selection question.
- Any future GE-08 code-authorizing handoff must remain narrower than this spec domain and must preserve the plugin-exception rule, exact write scope, required reads, expected receipts, and whether the slice is headless-only or UI-facing before implementation begins.
