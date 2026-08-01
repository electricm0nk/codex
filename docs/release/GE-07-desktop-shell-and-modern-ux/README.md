---
stc_id: STC-CODEX-GE-07
stc_kind: source-requirements
template_version: 2
work_type: planning-only
workflow_route: planning
readiness: planning-ready
status: active
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/README.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch: future GE-07 implementation handoffs should branch from current develop only after GE-06 viability is explicitly accepted or a bounded pre-viability spike is separately authorized; the repo was re-observed on 2026-06-24 with local branch `ge07-e1-desktop-shell-scaffold` at `48892249d5573927bf23a7e47a6d7d6a742da664` and `origin/develop` at `7bc89e8c1edf8f1d1a6d490a0ad28ac72fc6f104`; that grounds GE07-E1 as branch-ready but not yet merged into `origin/develop`
  write_scope: source STC generation grants no repo writes; any future GE-07 implementation handoff must declare exact repo paths and whether it touches desktop scaffold, frontend shell, command adapters, packaging, or diagnostic/explanation surfaces
review_state: draft
last_reviewed_at: 2026-06-24
parent_scopes:
  - programs/codex
source_artifacts:
  - programs/codex/plans/spec-domains/GE-07-desktop-shell-and-modern-ux.md
  - programs/codex/plans/roadmaps/codex-spec-domain-roadmap-2026-06-18.md
  - programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md
  - programs/codex/requirements/README.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md
  - programs/codex/requirements/GE-04-rules-engine-and-explainability-core/README.md
  - programs/codex/requirements/GE-04-rules-engine-and-explainability-core/technical-requirements.md
  - programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/technical-requirements.md
  - programs/codex/doctrine/program-doctrine-and-scope-charter.md
  - programs/codex/doctrine/quality-gate-policy.md
  - programs/codex/research/codex-reference-architecture-2026-06-17.md
related_artifacts:
  - programs/codex/plans/spec-domains/GE-00-program-governance-and-scope.md
  - programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/README.md
  - programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e1-shell-scaffold-receipt-2026-06-22.md
  - programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e1-runtime-boundary-adr-input-2026-06-22.md
  - programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e2-execution-readiness-closure-2026-06-22.md
  - programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e3-execution-readiness-closure-2026-06-22.md
  - programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e3-ui-truth-verification-receipt-2026-06-22.md
  - programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e4-execution-readiness-closure-2026-06-22.md
  - programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e4-explanation-diagnostics-visibility-receipt-2026-06-22.md
  - programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e5-execution-readiness-closure-2026-06-22.md
  - programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e6-platform-risk-receipt-2026-06-22.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md
  - repos/codex/AGENTS.md
upstream_targets:
  - programs/codex/README.md
  - programs/codex/requirements/README.md
expected_output_artifacts:
  - path: programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/pilot-shell-architecture-requirements.md
    completion_rule: Defines the pilot desktop shell frame, top-level navigation, panel layout obligations, and the rule that shell orchestration may not own rules semantics.
  - path: programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ui-information-architecture-requirements.md
    completion_rule: Defines the information architecture for the pilot character workspace, rules library, diagnostics, explanations, and source-package inspection surfaces.
  - path: programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/pilot-ux-flow-requirements.md
    completion_rule: Defines the user-visible pilot flow from loading the pilot case through explanation and diagnostics inspection using real domain outputs rather than mock state.
  - path: programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/component-surface-inventory.md
    completion_rule: Enumerates the required GE-07 surfaces, their backing data contracts, primary user/job, and explicit non-goals so later UI slices stay bounded.
  - path: programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ui-command-boundary-requirements.md
    completion_rule: Defines the UI-to-core command boundary, required read models, provenance/explanation payload duties, and what the UI is forbidden to compute locally.
  - path: programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/cross-platform-build-constraint-questions.md
    completion_rule: Records the Linux, Windows, and macOS packaging/signing/open-question ledger that must be resolved or consciously deferred before code-authorizing desktop implementation work.
supersedes: []
superseded_by: []
tags:
  - codex
  - ge-07
  - desktop-shell
  - ux
  - tauri
  - react
  - explainability
  - diagnostics
---

# GE-07 — Desktop Shell and Modern UX

## Objective
Define the authoritative desktop-shell and modern-UX contract for Codex so the pilot character experience presents proven domain behavior, explanations, provenance, and diagnostics without the UI owning rules semantics.

## Execution Posture
- **Eventual delivery mode:** mixed; documentary/planning plus bounded code-producing slices
- **Current pass mode:** GE07-E1 branch-ready scaffold awaiting Todd review/merge
- **Run in now:** no fresh GE-07 coding launch is justified by default; Hermes owns documentary route sync while Todd reviews or merges the existing GE07-E1 branch result
- **Code authority now:** yes, but only in `artifacts/ge07-e1-execution-handoff-2026-06-22.md`; this README and root route surface do not authorize code by themselves, and the truthful control state is now `awaiting-todd-merge`
- **Active route artifact:** `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/execution-handoff.md`
- **Active stage-specific handoff:** `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e1-execution-handoff-2026-06-22.md`
- **Operator note:** the prior GE-07 board cards produced documentary closures; the repaired GE07-E1 lane has now produced a pushed feature branch `ge07-e1-desktop-shell-scaffold` at `48892249d5573927bf23a7e47a6d7d6a742da664`, while `origin/develop` still lacks that scaffold

## Deliverable Type
`planning-only`

## Workflow Route
`planning`

## Readiness
`planning-ready`

Why this readiness is accurate:
- Todd explicitly authorized early GE-07 source-STC drafting on 2026-06-21 even though GE-06 viability is not yet settled, clearing the soft documentary blocker while leaving the implementation gate intact.
- the GE-07 spec domain exists and already frames the UI as a consumer of proven domain behavior rather than the owner of rules semantics.
- GE-00 doctrine and the quality-gate policy already define the headless-core-first rule and the UI-truth gate this STC must inherit.
- GE-06 exists as the integrated pilot truth boundary, so GE-07 can define the shell and UX surface that will consume that behavior even though GE-06 has not yet proven final viability.
- GE-06 now also has an explicit viability/domain-confidence decision fixing the current downstream posture at `computed-but-not-oracle-checked`, which clears the stale "no GE-06 decision exists" blocker without authorizing product-visible UI truth.
- the Tauri 2 plus TypeScript/React posture is grounded as the current architecture preference in accepted roadmap/research surfaces, while final implementation details remain explicitly unresolved here rather than guessed.
- this bundle includes the GE-07 control documents plus the same-epic documentary outputs the spec domain requires, without fabricating a coding route.
- GE07-E1 documentary spike artifacts ground the smallest additive shell scaffold shape and the first runtime-boundary answer; the repaired GE07-E1 readiness closure and execution handoff then produced a branch-ready scaffold on `ge07-e1-desktop-shell-scaffold` at `48892249d5573927bf23a7e47a6d7d6a742da664`, while merge into `origin/develop` remains unproven.
- the GE07-E2 readiness closure now records the next honest stop condition: exact scaffold prerequisite paths are known, but no code-authorizing boundary handoff exists because the scaffold is still absent on `origin/develop` even though a branch-ready GE07-E1 implementation now exists, and the upstream GE06-E4-F1 consumer bridge remains unlaunched.
- the GE07-E3 readiness closure and paired UI-truth verification receipt now define the minimum pilot workspace state over real pilot data, while still refusing counterfeit shell code authority before scaffold and merged snapshot-bridge truth exist.
- the GE07-E4 readiness closure and paired visibility receipt now ground live explanation detail plus rules-core, validation, and importer diagnostic payloads, while still refusing counterfeit shell code authority before scaffold truth, the merged consumer bridge, and a real invalid-choice reason lane exist.
- the GE07-E5 readiness closure now grounds the raw pilot rules/source-package carriers and the cross-link obligation back into the active character path, while still refusing counterfeit inspection code authority before scaffold, merged workspace bridge, and inspection-projection truth exist.
- the GE07-E6 platform-risk receipt and refreshed cross-platform constraint ledger now ground Linux, Windows, and macOS packaging/signing blocker classes while explicitly refusing ship-readiness claims before a real shell slice exists.

## Closure State
GE-07 is now an authoritative planning-ready source STC with one active bounded coding packet in `awaiting-todd-merge` state. The active packet is GE07-E1 only: scaffold creation under `apps/desktop/**` via `artifacts/ge07-e1-execution-handoff-2026-06-22.md`. The repo truth currently shows a branch-ready scaffold on `ge07-e1-desktop-shell-scaffold` at `48892249d5573927bf23a7e47a6d7d6a742da664`, but no merge receipt because `origin/develop` does not yet contain that work. GE-07 still does not authorize broad UI implementation, product-visible truth claims, final command transport, framework finality, packaging/signing, or downstream E2/E3/E4/E5 work. GE07-E4 makes the current explanation/diagnostics truth burden explicit, but still does not authorize shell-side rendering work while the scaffold, merged consumer bridge, and invalid-choice payload lane remain unresolved. GE07-E6 likewise makes the packaging/signing blocker set explicit without authorizing ship-readiness claims or platform build assertions.

## Authority and Scope
- owner: `Todd Hintzmann`
- scope: `program`
- canonical: `true`
- canonical path: `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/README.md`
- parent scopes:
  - `programs/codex`

This STC governs the Codex desktop-shell and modern-UX planning surface. GE-03 owns importer diagnostics and provenance outputs. GE-04 owns rules computation, explanation generation, and invalid-choice truth. GE-05 owns parity-comparison truth. GE-06 owns the integrated pilot proof contract that GE-07 must consume. GE-07 owns only the presentation-layer requirements, shell boundaries, and UX/documentary outputs needed to present that truth honestly.

## Target Runtime
- repo: `/home/ubuntu/workspace/repos/codex`
- workdir: `/home/ubuntu/workspace/repos/codex`
- branch/worktree: `future GE-07 implementation handoffs should branch from current develop only after a later readiness closure names the exact route truth; the repo was re-observed on 2026-06-24 with checked-out branch ge07-e1-desktop-shell-scaffold at 48892249d5573927bf23a7e47a6d7d6a742da664 and origin/develop at 7bc89e8c1edf8f1d1a6d490a0ad28ac72fc6f104, which grounds GE07-E1 as branch-ready but not yet merged into origin/develop`
- allowed write scope: `none during source STC generation; future GE-07 implementation handoffs must declare exact repo paths and whether they are shell scaffold, frontend-shell, command-adapter, packaging, or diagnostics/explanation changes`

The repo is grounded only as the future implementation surface. This package is a requirements authority surface under `programs/codex/requirements/`, not a repo-local implementation brief.

## Document Map
- `technical-requirements.md` — normative requirements for shell architecture, the UI-to-core boundary, real-data truth, explanation/diagnostic visibility, surface inventory, local-state boundaries, and packaging constraints.
- `technical-design.md` — architecture/design response for composing a Tauri desktop shell over the headless Codex substrate without allowing UI-first drift.
- `acceptance-and-verification.md` — planning-level acceptance criteria and future UI-truth verification obligations.
- `risks-and-open-questions.md` — unresolved command-boundary, framework finality, packaging, explanation, and diagnostics questions isolated from the main contract.
- `epic-breakdown.md` — later implementation-facing decomposition for narrow shell, boundary, flow, and packaging slices.
- `references/upstream-dependency-contract.md` — compact contract mapping what GE-00, GE-03, GE-04, GE-06, doctrine, and reference architecture do and do not authorize.
- `artifacts/pilot-shell-architecture-requirements.md` — shell-frame and panel-layout requirements for the pilot.
- `artifacts/ui-information-architecture-requirements.md` — information architecture for pilot character, rules library, source-package, diagnostics, and explanation surfaces.
- `artifacts/pilot-ux-flow-requirements.md` — required user-visible pilot flows grounded in real domain outputs.
- `artifacts/component-surface-inventory.md` — bounded inventory of required UI surfaces and their backing data duties.
- `artifacts/ui-command-boundary-requirements.md` — contract for UI commands/read models and explicit UI non-ownership of rules semantics.
- `artifacts/cross-platform-build-constraint-questions.md` — packaging/signing/OS-constraint ledger for Linux, Windows, and macOS.
- `artifacts/ge07-e6-platform-risk-receipt-2026-06-22.md` — execution-backed receipt grounding the current repo stop condition, host/tooling gaps, and later GE-09 release-governance decision inputs for Linux, Windows, and macOS.
- `artifacts/ge07-e1-shell-scaffold-receipt-2026-06-22.md` — live repo/toolchain receipt proving that GE07-E1 is still a non-production scaffold candidate and naming the smallest additive shell path shape.
- `artifacts/ge07-e1-runtime-boundary-adr-input-2026-06-22.md` — ADR input for the first read-only shell boundary over the real GE-06 headless receipt path.
- `artifacts/ge07-e1-execution-readiness-closure-2026-06-22.md` — repaired readiness closure converting the missing scaffold stop condition into a bounded code-ready GE07-E1 lane.
- `artifacts/ge07-e1-execution-handoff-2026-06-22.md` — active stage-specific code-authorizing handoff whose first run produced the additive desktop scaffold on a feature branch and now awaits Todd merge.
- `execution-handoff.md` — non-authorizing root route surface reflecting the current `awaiting-todd-merge` state for GE07-E1.
- `artifacts/ge07-e2-execution-readiness-closure-2026-06-22.md` — readiness closure grounding the real repo/dependency posture for the command-boundary lane and explicitly refusing counterfeit code authority before scaffold and upstream view-model prerequisites exist.
- `artifacts/ge07-e3-execution-readiness-closure-2026-06-22.md` — readiness closure grounding the pilot workspace shell lane, defining the minimum workspace state over real pilot outputs, and refusing shell code authority before scaffold and merged snapshot-bridge truth exist.
- `artifacts/ge07-e3-ui-truth-verification-receipt-2026-06-22.md` — execution-backed receipt capturing current selections plus computed and blocked route examples from the deterministic pilot fixture.
- `artifacts/ge07-e4-execution-readiness-closure-2026-06-22.md` — readiness closure grounding the live explanation/diagnostics payload burden and explicitly refusing counterfeit code authority before scaffold, merged consumer-bridge, projection, and invalid-choice prerequisites exist.
- `artifacts/ge07-e4-explanation-diagnostics-visibility-receipt-2026-06-22.md` — execution-backed receipt capturing real explanation details plus blocked-route, validation, and importer diagnostics from live Codex surfaces.
- `artifacts/ge07-e5-execution-readiness-closure-2026-06-22.md` — readiness closure grounding the current pilot rules/source-package inspection burden and refusing counterfeit inspection code authority before scaffold, workspace bridge, and inspection-projection truth exist.

## Expected Output Artifacts
| Artifact | Completion rule |
|---|---|
| `artifacts/pilot-shell-architecture-requirements.md` | Defines the pilot desktop shell frame, top-level navigation, panel layout obligations, and the rule that shell orchestration may not own rules semantics. |
| `artifacts/ui-information-architecture-requirements.md` | Defines the information architecture for the pilot character workspace, rules library, diagnostics, explanations, and source-package inspection surfaces. |
| `artifacts/pilot-ux-flow-requirements.md` | Defines the user-visible pilot flow from loading the pilot case through explanation and diagnostics inspection using real domain outputs rather than mock state. |
| `artifacts/component-surface-inventory.md` | Enumerates the required GE-07 surfaces, their backing data contracts, primary user/job, and explicit non-goals so later UI slices stay bounded. |
| `artifacts/ui-command-boundary-requirements.md` | Defines the UI-to-core command boundary, required read models, provenance/explanation payload duties, and what the UI is forbidden to compute locally. |
| `artifacts/cross-platform-build-constraint-questions.md` | Records the Linux, Windows, and macOS packaging/signing/open-question ledger that must be resolved or consciously deferred before code-authorizing desktop implementation work. |

## Required Reads
- `../../plans/spec-domains/GE-07-desktop-shell-and-modern-ux.md` — primary strategic authority for this desktop-shell source STC.
- `../../plans/roadmaps/codex-spec-domain-roadmap-2026-06-18.md` — Stage D roadmap posture and product-slice exit-gate authority.
- `../../plans/roadmaps/codex-execution-status-ledger-2026-06-21.md` — route-state authority proving GE-07 now has an active GE07-E1 code handoff while downstream GE-07 lanes remain non-authorizing.
- `../GE-06-pilot-vertical-slice-pf1-human-fighter/README.md` — integrated pilot truth boundary the UI must consume rather than redefine.
- `../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md` — current downstream posture authority fixing GE-06 at `computed-but-not-oracle-checked` and preserving the non-production rule for early UI work.
- `../GE-04-rules-engine-and-explainability-core/README.md` — computation/explainability planning boundary for derived values and invalid choices.
- `../GE-04-rules-engine-and-explainability-core/technical-requirements.md` — explanation, diagnostics, and character-state obligations inherited by the UI.
- `../GE-03-pcgen-import-pipeline-and-provenance/technical-requirements.md` — importer diagnostics and provenance visibility obligations inherited by the UI.
- `../../doctrine/program-doctrine-and-scope-charter.md` — headless-core-first and anti-UI-first doctrine.
- `../../doctrine/quality-gate-policy.md` — documentation gate and UI-truth gate authority.
- `../../research/codex-reference-architecture-2026-06-17.md` — grounded Tauri/TypeScript/React reference architecture and surface taxonomy.

## Conditional Reads
- `/home/ubuntu/workspace/repos/codex/AGENTS.md` — only when a future GE-07 implementation or spike handoff is prepared for the repo.
- `../GE-06-pilot-vertical-slice-pf1-human-fighter/execution-handoff.md` — only when a future GE-07 slice depends on the concrete integrated pilot runtime surface rather than the planning contract alone.
- `../GE-05-oracle-validation-and-parity-harness/README.md` — only when the UI must expose parity-report or known-gap surfaces directly.
- any future ADR under `programs/codex/doctrine/decisions/` for framework, boundary transport, or packaging choices — only once such a decision is accepted.
- runtime receipts produced by future GE-06 or GE-07 bounded slices — only when deriving a code-authorizing handoff that must prove real shell integration.

## In Scope
- Codex GE-07 source-STC documents under `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/`
- same-epic documentary outputs under `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/`
- shell architecture requirements for a local-first Tauri desktop shell over the pilot domain substrate
- UI information architecture for pilot character, rules library, source-package, explanation, and diagnostics surfaces
- the UI-to-core command boundary and local UI-state boundaries
- cross-platform packaging/build constraints and unanswered questions for Linux, Windows, and macOS
- the rule that UI must consume proven domain outputs, explanations, provenance, and diagnostics rather than fabricate them

## Out of Scope
- writing Tauri, React, Rust, packaging, or integration code in `/home/ubuntu/workspace/repos/codex`
- modifying `/home/ubuntu/workspace/repos/pcgen`
- expanding beyond the pilot user journey into full Pathfinder breadth, cloud sync, accounts, mobile, marketplace, or VTT integrations
- allowing the UI to implement or reinterpret rules semantics locally
- final visual-design polish or public-launch design-system breadth
- claiming GE-06 viability, parity, or product truth from documentary requirements alone
- deriving a GE-07 `execution-handoff.md` before the UI route is genuinely readiness-grounded

## Acceptance Summary
The acceptance criteria in `acceptance-and-verification.md` are satisfied for the planning-ready GE-07 boundary when this bundle and its named output artifacts exist, remain internally linked, and make the UI a consumer of domain truth rather than an owner of rules behavior.

Compact summary:
- the desktop shell is defined as a presentation layer over real domain outputs, not a substitute for them
- explanation, provenance, and diagnostics visibility are explicit product requirements
- shell surfaces, UX flow, command boundary, and packaging questions are all bounded into concrete documents
- this source STC remains planning-only, while bounded execution authority exists only in the GE07-E1 stage-specific handoff that now awaits Todd merge

## Allowed Assumptions
- the current architecture preference remains Tauri 2 plus a TypeScript UI because the roadmap and reference architecture both ground that posture, but the exact frontend binding and component stack can still require a later ADR or bounded spike
- GE-06 remains the first integrated pilot proof target and therefore the primary domain truth input for the first GE-07 implementation slice
- GE-03 and GE-04 continue to own importer diagnostics/provenance and rules/explanation truth respectively; GE-07 must consume rather than redefine them
- a future GE-07 implementation route should prefer `origin/develop` as its branch base once GE-06 viability and dependency posture are explicitly settled

## Blockers / Forbidden Assumptions
- stop if a future handoff treats this source STC as permission for broad UI implementation without an explicit bounded slice, exact repo paths, branch/worktree policy, write scope, required reads, and verification commands
- do not fabricate the final command transport, local storage schema, packaging automation, or framework ADR outcome here
- do not allow mock state, screenshots, or hardcoded examples to satisfy the UI-truth gate
- do not let the UI compute rules answers, prerequisite results, or explanation logic that belongs to GE-04 or GE-06
- do not treat “modern UX” as permission to hide diagnostics, unsupported-token warnings, or provenance details from the operator/user surfaces that require them
- do not claim cross-platform readiness, signing, updater behavior, or product-polish completion from this planning bundle alone

## Next Stage Rule
- GE-07 is planning-ready because the control bundle and required same-epic documentary artifacts now exist.
- The active GE07-E1 route set is now `artifacts/ge07-e1-shell-scaffold-receipt-2026-06-22.md`, `artifacts/ge07-e1-runtime-boundary-adr-input-2026-06-22.md`, `artifacts/ge07-e1-execution-readiness-closure-2026-06-22.md`, and `artifacts/ge07-e1-execution-handoff-2026-06-22.md`; the final file is the only GE-07 artifact currently carrying stage-specific code authority, and it now sits in `awaiting-todd-merge` after producing a branch-ready scaffold on `ge07-e1-desktop-shell-scaffold`.
- The active GE07-E2 route artifact is `artifacts/ge07-e2-execution-readiness-closure-2026-06-22.md`; it records that no GE07-E2 code-authorizing handoff is justified yet because the scaffold is still absent on `origin/develop` even though a branch-ready GE07-E1 scaffold now exists off-develop, and the separately tracked GE06-E4-F1 consumer bridge remains unlaunched.
- The active GE07-E3 route artifact is `artifacts/ge07-e3-execution-readiness-closure-2026-06-22.md`; it records that the minimum pilot workspace truth burden is now explicit, but no code-authorizing shell handoff is justified yet because the scaffold is still absent on `origin/develop` even though a branch-ready GE07-E1 scaffold now exists off-develop, and the separately tracked GE06-E4-F1 consumer bridge remains unlaunched.
- The active GE07-E4 route artifacts are `artifacts/ge07-e4-execution-readiness-closure-2026-06-22.md` plus `artifacts/ge07-e4-explanation-diagnostics-visibility-receipt-2026-06-22.md`; they record the live explanation/diagnostics burden while explicitly refusing shell-side code authority before the scaffold, merged consumer bridge, bounded projection contract, and invalid-choice reason lane are real.
- The active GE07-E5 route artifact is `artifacts/ge07-e5-execution-readiness-closure-2026-06-22.md`; it records that current pilot rules/source-package carriers and the cross-link obligation back into the active character path are now grounded, but no code-authorizing inspection handoff is justified yet because the scaffold, merged workspace bridge, and dedicated inspection projection are still absent on `origin/develop`.
- The active GE07-E6 route artifacts are `artifacts/cross-platform-build-constraint-questions.md` and `artifacts/ge07-e6-platform-risk-receipt-2026-06-22.md`; they ground the packaging/signing blocker classes and later GE-09 decision inputs while explicitly refusing ship-readiness claims before a real shell slice exists and platform-specific receipts are produced.
- No additional GE-07 code-authorizing handoff should be derived until the intended UI slice is bounded and the route is honestly grounded by: the merge or accepted successor of the current scaffold subtree, exact repo path/write scope, runtime/toolchain checks, and merged upstream payload/consumer truth rather than documentary aspiration.
- The shortest honest precursor to a future GE07-E2 or GE07-E3 coding lane is the merge of the existing GE07-E1 scaffold branch plus the real merge of GE06-E4-F1, followed by a fresh readiness closure that proves the slice can stay narrow; do not smuggle scaffold work, rules-core view-model work, and shell presentation into one fake packet.
- If GE-06 viability becomes accepted and those prerequisite scaffold/consumer surfaces exist on `origin/develop`, the next truthful GE-07 artifact is a fresh narrow readiness closure for the first grounded shell slice rather than a broad UI implementation packet.
