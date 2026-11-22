---
stc_id: STC-CODEX-SD-15
stc_kind: source-requirements
template_version: 2
work_type: planning-only
workflow_route: planning
readiness: planning-ready
status: active
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/README.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch: current live repo state observed 2026-06-30 is branch `sd11-f10-update-action-surface` at commit `a0a90dcdf75cd0cc09d4b71e9fb7d3b440aaf293`; current accepted operator-facing tester baseline remains `origin/develop` at commit `c2cea5c6baeb3ca34077b85331214c4b42a4809c`; accepted planning truth now includes SD-11 tester-workbench and GitHub-intake authority, SD-12 distribution/update authority, the SD-13 roster/progression spec-domain boundary, and the SD-14 persistence authority, but no accepted SD-15 operations/regression/closure authority surface exists yet
  write_scope: documentary-only updates inside this source STC bundle plus control-plane sync in `programs/codex/requirements/README.md`; no repo implementation-code authority
review_state: draft
last_reviewed_at: 2026-06-30
parent_scopes:
  - programs/codex
source_artifacts:
  - programs/codex/plans/spec-domains/SD-15-test-program-operations-regression-and-triage.md
  - programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md
  - programs/codex/requirements/README.md
  - programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md
  - programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/tester-feedback-evidence-capture-matrix.md
  - programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/rollback-withdrawal-and-downgrade-policy.md
  - programs/codex/plans/spec-domains/SD-13-core-class-race-roster-and-level-10-progression-matrix.md
  - programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/README.md
  - programs/codex/doctrine/program-doctrine-and-scope-charter.md
  - programs/codex/doctrine/documentation-control-plane.md
  - programs/codex/doctrine/quality-gate-policy.md
  - programs/codex/README.md
  - repos/codex/README.md
related_artifacts:
  - programs/codex/README.md
  - repos/codex/README.md
  - programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md
upstream_targets:
  - programs/codex/requirements/README.md
  - programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md
expected_output_artifacts:
  - path: programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/triage-class-dictionary.md
    completion_rule: Defines the bounded SD-15 issue classes, their evidence thresholds, adjacent-authority references, and the visible treatment for unsupported, partial, blocked, not-yet-verified, and status-drift outcomes.
  - path: programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/intake-to-triage-mapping.md
    completion_rule: Defines the operator-side bridge from SD-11 intake into SD-15 triage, preserving the tester-supplied versus auto-captured evidence split and separating operator-added classification data.
  - path: programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/regression-receipt-schema.md
    completion_rule: Defines the receipt-grade regression and defect-state schema, including build/channel/provenance identity, workflow/support/persistence context, reproduction status, diagnostics, and attachment/redaction posture.
  - path: programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/evidence-freshness-and-verdict-rules.md
    completion_rule: Defines how SD-15 evidence is judged as current, stale, partial, insufficient, not-reproduced, or otherwise bounded so missing or aging proof cannot masquerade as fix or closure truth.
  - path: programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/tranche-2-install-and-use-matrix.md
    completion_rule: Defines the exact tranche-2 install and use path across channel, platform, build identity, bounded workflow surface, and evidence obligations so install/use claims stop depending on folklore.
  - path: programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/tranche-2-clean-machine-validation-report.md
    completion_rule: Defines the clean-machine validation receipt/report surface, including environment identity, build under test, executed steps, captured evidence, triage class for failures, and final verdict.
  - path: programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/tranche-2-external-test-cycle-plan.md
    completion_rule: Defines the external tester cohort, target builds/channels/platforms, missions, evidence requirements, stop conditions, and operator triage cadence before any external cycle is launched.
  - path: programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/tranche-2-external-test-cycle-report.md
    completion_rule: Defines the durable result surface for what external testers actually did, what failed, what remained unsupported, and what tranche-closure implications follow.
  - path: programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/tranche-2-project-status-truth-reconciliation-checklist.md
    completion_rule: Defines the exact repo/workspace/execution-ledger fields that must reconcile before tranche-2 closure claims are truthful and records the permitted drift and escalation states.
  - path: programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/sd15-e7-release-alpha-test-defect-repair-handoff-2026-07-02.md
    completion_rule: Defines the code-authorizing repair bundle for confirmed released-Linux-alpha tester defects: missing packaged GE08 proof-package root, checksum-manifest filename mismatch, and safe no-transport manual issue-draft posture without hardcoded credentials.
  - path: programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/sd15-e8-feedback-output-context-sanitization-handoff-2026-07-02.md
    completion_rule: Defines the code-authorizing repair bundle for preventing internal memory/system-context blocks from entering user-visible update evidence, preserved drafts, or manual filing payloads while preserving correct no-official-release update truth.
supersedes: []
superseded_by: []
tags:
  - codex
  - sd-15
  - test-program-operations
  - triage
  - regression
  - clean-machine-validation
  - external-testing
  - tranche-closure
---

# SD-15 — Test-program operations, regression, and triage

## Objective
Define the tester-program operations authority surface that turns tranche-2 from a merely buildable tester lane into an evidence-bearing operating lane with bounded issue classification, regression provenance, clean-machine validation, external test-cycle discipline, and cross-surface status truth reconciliation.

## Deliverable Type
`planning-only`

## Workflow Route
`planning`

## Readiness
`planning-ready`

Why this readiness is accurate:
- the SD-15 strategic spec domain already exists and explicitly frames this lane as evidence work rather than an implementation prompt
- adjacent planning truth already exists: SD-11 defines tester-facing workbench and GitHub intake posture, SD-12 defines distribution/update/rollback posture, SD-13 defines bounded roster/progression support truth, and SD-14 defines persistence and upgrade-safe lifecycle truth
- the live repo and program readme surfaces already give the packet concrete status and onboarding material to reconcile rather than forcing abstract process theater
- the execution status ledger already exposes the route-legibility problem this packet must help close, but it is not yet a tester-operations authority surface by itself
- this bundle includes both the control documents and concrete same-epic documentary artifacts for install/use, clean-machine proof, external testing, and status reconciliation, so the lane stays concrete instead of recursive

## Closure State
SD-15 is generated as a planning-ready source STC on 2026-06-30. It defines the tester-program operations contract for issue triage, regression evidence, build/platform/channel provenance, clean-machine validation, external test-cycle planning/results, and project-status truth reconciliation across repo, workspace, and operator ledger surfaces. It does not authorize repo implementation code, public-release operations, telemetry/SRE sprawl, or support-process automation by itself. Current accepted truth remains that Codex has a buildable bounded tester surface plus adjacent planning lanes for tester UX, distribution, breadth, and persistence, but no accepted operator-closure surface yet proves tranche-2 install/use reality, regression posture, or status reconciliation over time.

## Authority and Scope
- owner: `Todd Hintzmann`
- scope: `program`
- canonical: `true`
- canonical path: `programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/README.md`
- parent scopes:
  - `programs/codex`

This STC governs the tester-program operations lane: issue-triage taxonomy, regression evidence fields, clean-machine proof, external test-cycle planning/results, and closure-truth reconciliation. SD-11 owns tester-facing workbench and GitHub issue-flow UX. SD-12 owns distribution/update/rollback transport truth. SD-13 owns bounded roster/progression support-state truth. SD-14 owns saved-state and upgrade-survival truth. SD-15 owns how the operator classifies, proves, audits, and closes those adjacent truths without collapsing them into one undifferentiated status claim.

## Target Runtime
- repo: `/home/ubuntu/workspace/repos/codex`
- workdir: `/home/ubuntu/workspace/repos/codex`
- branch/worktree: `current live repo state observed 2026-06-30 is branch sd11-f10-update-action-surface at commit a0a90dcdf75cd0cc09d4b71e9fb7d3b440aaf293; current accepted operator-facing tester baseline remains origin/develop at c2cea5c6baeb3ca34077b85331214c4b42a4809c; current accepted truth includes adjacent planning surfaces for tester UX, distribution, breadth, and persistence, but no accepted SD-15 operator lane yet exists`
- allowed write scope: `none during source STC generation beyond this packet and control-plane sync docs; future SD-15 handoffs must declare exact repo paths, exact write scope, exact required reads, and exact verification commands before repo or workflow files may change`

This bundle is an authority surface under `programs/codex/requirements/`, not a repo-local implementation brief.

## Document Map
- `technical-requirements.md` — normative requirements for issue classification, regression evidence, install/use proof, external test cycles, and status reconciliation
- `technical-design.md` — architecture/design response describing how GitHub intake, triage, regression evidence, validation receipts, external test-cycle surfaces, and status truth should cooperate without becoming faux release ops
- `acceptance-and-verification.md` — observable checks proving the SD-15 packet and same-epic documentary outputs remain concrete and reviewable
- `risks-and-open-questions.md` — unresolved taxonomy, coverage freshness, clean-machine cadence, external tester logistics, and reconciliation-governance questions isolated from the main contract
- `epic-breakdown.md` — downstream epic/feature decomposition for later same-domain documentary and execution-story minting
- `references/upstream-dependency-contract.md` — compact contract naming what SD-11, SD-12, SD-13, SD-14, the execution ledger, and live status surfaces do and do not authorize for SD-15
- `artifacts/triage-class-dictionary.md` — concrete SD-15 issue taxonomy with evidence thresholds, adjacent-authority references, and visible outcome handling
- `artifacts/intake-to-triage-mapping.md` — concrete bridge from SD-11 intake evidence into SD-15 operator classification without rewriting tester UX
- `artifacts/regression-receipt-schema.md` — concrete receipt schema for build/channel/provenance identity, workflow/support/persistence context, reproduction status, diagnostics, and attachment posture
- `artifacts/evidence-freshness-and-verdict-rules.md` — concrete freshness, sufficiency, and verdict rules that keep `not-reproduced` and other bounded outcomes from masquerading as fix or closure proof
- `artifacts/tranche-2-install-and-use-matrix.md` — concrete install/use matrix contract for the bounded tester tranche
- `artifacts/tranche-2-clean-machine-validation-report.md` — concrete clean-machine receipt/report contract
- `artifacts/tranche-2-external-test-cycle-plan.md` — concrete external-test-cycle planning contract
- `artifacts/tranche-2-external-test-cycle-report.md` — concrete external-test-cycle result contract
- `artifacts/tranche-2-lnx-a-testing-instructional-brief-2026-07-02.md` — actual row-ready tester packet for the grounded Linux alpha path, decoupling execution instructions from tester identity while preserving evidence burden, mission bounds, and issue-intake rules
- `artifacts/tranche-2-project-status-truth-reconciliation-checklist.md` — concrete reconciliation contract for repo/workspace/ledger truth before closure
- `artifacts/sd15-e6-automation-helper-boundary-handoff-2026-06-30.md` — stage-specific SD15-E6 handoff that decides whether any repo helper lane is warranted and, if so, freezes the exact headless code boundary, reads, verification floor, non-goals, and Claude-only receipt requirements
- `artifacts/sd15-e7-release-alpha-test-defect-repair-handoff-2026-07-02.md` — stage-specific SD15-E7 code-authorizing handoff for the released Linux alpha tester-defect repair bundle discovered through LNX-A execution evidence
- `artifacts/sd15-e8-feedback-output-context-sanitization-handoff-2026-07-02.md` — stage-specific SD15-E8 code-authorizing handoff for internal-context sanitization in feedback/update reportable output surfaces

## Expected Output Artifacts
| Artifact | Completion rule |
|---|---|
| `artifacts/triage-class-dictionary.md` | Defines the bounded SD-15 issue classes, their evidence thresholds, adjacent-authority references, and the visible treatment for unsupported, partial, blocked, not-yet-verified, and status-drift outcomes. |
| `artifacts/intake-to-triage-mapping.md` | Defines the operator-side bridge from SD-11 intake into SD-15 triage, preserving the tester-supplied versus auto-captured evidence split and separating operator-added classification data. |
| `artifacts/regression-receipt-schema.md` | Defines the receipt-grade regression and defect-state schema, including build/channel/provenance identity, workflow/support/persistence context, reproduction status, diagnostics, and attachment/redaction posture. |
| `artifacts/evidence-freshness-and-verdict-rules.md` | Defines how SD-15 evidence is judged as current, stale, partial, insufficient, not-reproduced, or otherwise bounded so missing or aging proof cannot masquerade as fix or closure truth. |
| `artifacts/tranche-2-install-and-use-matrix.md` | Defines the exact tranche-2 install and use path across channel, platform, build identity, bounded workflow surface, and evidence obligations so install/use claims stop depending on folklore. |
| `artifacts/tranche-2-clean-machine-validation-report.md` | Defines the clean-machine validation receipt/report surface, including environment identity, build under test, executed steps, captured evidence, triage class for failures, and final verdict. |
| `artifacts/tranche-2-external-test-cycle-plan.md` | Defines the external tester cohort, target builds/channels/platforms, missions, evidence requirements, stop conditions, and operator triage cadence before any external cycle is launched. |
| `artifacts/tranche-2-external-test-cycle-report.md` | Defines the durable result surface for what external testers actually did, what failed, what remained unsupported, and what tranche-closure implications follow. |
| `artifacts/tranche-2-lnx-a-testing-instructional-brief-2026-07-02.md` | Defines the real tester-facing execution brief for the grounded Linux alpha row, including governed artifact identity, exact install/launch instructions, mission ordering, evidence capture, issue-filing rules, and the correction that tester identity is not a prerequisite for issuing instructions. |
| `artifacts/tranche-2-project-status-truth-reconciliation-checklist.md` | Defines the exact repo/workspace/execution-ledger fields that must reconcile before tranche-2 closure claims are truthful and records the permitted drift and escalation states. |
| `artifacts/sd15-e7-release-alpha-test-defect-repair-handoff-2026-07-02.md` | Defines the exact coding brief for repairing confirmed released-Linux-alpha tester defects while preserving manual issue-draft/no-transport safety and forbidding hardcoded GitHub credentials. |
| `artifacts/sd15-e8-feedback-output-context-sanitization-handoff-2026-07-02.md` | Defines the exact coding brief for stripping internal memory/system-context blocks from copyable/reportable tester output while preserving honest local-build update semantics. |

## Required Reads
- `../../plans/spec-domains/SD-15-test-program-operations-regression-and-triage.md` — primary strategic authority for this source STC
- `../../plans/roadmaps/codex-execution-status-ledger-2026-06-21.md` — operator route/status truth this packet must reconcile rather than contradict
- `../SD-11-test-user-workbench-and-github-feedback-intake/README.md` — tester-workbench and GitHub issue-flow authority this packet must preserve
- `../SD-11-test-user-workbench-and-github-feedback-intake/artifacts/tester-feedback-evidence-capture-matrix.md` — current evidence-capture fields and save/log attachment posture
- `../SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md` — current operator-versus-tester channel/support truth that regression and closure evidence must carry forward
- `../SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md` — distribution/update/rollback authority that install/use and clean-machine proof must inherit honestly
- `../SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/rollback-withdrawal-and-downgrade-policy.md` — recovery and withdrawn-build vocabulary SD-15 must use instead of folklore
- `../../plans/spec-domains/SD-13-core-class-race-roster-and-level-10-progression-matrix.md` — bounded roster/progression support-state truth that triage and external testing must classify honestly
- `../SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/README.md` — persistence, migration, and saved-state failure authority that SD-15 must preserve during triage and clean-machine proof
- `../../doctrine/program-doctrine-and-scope-charter.md` — bounded product/tranche doctrine
- `../../doctrine/documentation-control-plane.md` — authority-surface and control-plane doctrine
- `../../doctrine/quality-gate-policy.md` — anti-counterfeit-completion and evidence doctrine
- `/home/ubuntu/workspace/programs/codex/README.md` — workspace-facing project-status truth surface
- `/home/ubuntu/workspace/repos/codex/README.md` — repo-facing onboarding/current-state truth surface

## Conditional Reads
- `/home/ubuntu/workspace/repos/codex/AGENTS.md` — only when a later SD-15 execution or automation handoff is being prepared for repo-facing work
- GitHub Issues API or template docs — only when a later handoff implements exact issue-routing transport or issue-form surfaces
- CI/test-runner/regression-harness docs — only when a later handoff automates regression evidence collection instead of preserving the documentary contract
- VM/imaging/provisioning docs — only when a later handoff selects the exact clean-machine environment rather than keeping the proof obligation abstractly defined
- messaging or survey tooling docs — only when a later handoff selects the exact external tester coordination mechanism

## In Scope
- Codex SD-15 source-STC documents under `programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/`
- same-epic documentary outputs under `programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/`
- issue-triage taxonomy and operator routing rules for tester-reported defects and unsupported states
- regression evidence, build identity, platform/channel/support-state, and reproduction metadata requirements
- tranche-2 install/use matrix and clean-machine validation proof obligations
- external test-cycle planning, result capture, and verdict surfaces
- project-status truth reconciliation across `repos/codex/README.md`, `programs/codex/README.md`, and `programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md`
- downstream epic decomposition for later same-domain documentary and execution stories

## Out of Scope
- writing repo implementation code in `/home/ubuntu/workspace/repos/codex`
- public-release operations, marketplace/app-store posture, billing, licensing, or customer-support workflows
- generic telemetry/SRE infrastructure or production monitoring programs
- replacing SD-11 as GitHub issue-flow authority, SD-12 as distribution/update authority, SD-13 as breadth/support-state authority, or SD-14 as persistence authority
- claiming install/use or clean-machine success without real execution receipts
- treating repo/program readme prose as tranche-closure proof without explicit reconciliation and supporting evidence
- collapsing unsupported semantics, packaging failures, and install/use failures into the same generic bug bucket

## Acceptance Summary
The acceptance criteria in `acceptance-and-verification.md` are satisfied for the SD-15 planning-ready boundary when this bundle and its named same-epic documentary artifacts exist, remain internally coherent, and define tester-program operations as a concrete evidence-bearing control surface rather than vague future ops prose.

Compact summary:
- tester issues are classifiable against a bounded taxonomy instead of collapsing into an undifferentiated backlog
- regression claims carry enough build/platform/channel/support-state/persistence context to be reconstructable later
- tranche-2 install/use and clean-machine proof are mandatory named receipt surfaces, not assumptions from the authoring machine
- external testing remains a first-class planned and reported activity rather than an implied future phase
- repo-facing, workspace-facing, and operator-ledger status surfaces must reconcile before tranche closure can be claimed honestly

## Allowed Assumptions
- GitHub remains the authoritative intake destination for tester bug and enhancement submissions in this tranche unless a higher-order doctrine surface changes that rule
- Linux remains the first-class tester platform; macOS remains second-class and Windows explicitly third-class unless adjacent authority surfaces are revised
- tester-facing channel language continues to map to the live operator promotion truth `develop -> main`; any `beta` candidate track remains unavailable until a governed backing surface exists
- external testing in this tranche is bounded evidence work, not a public-support or mass-release program

## Blockers / Forbidden Assumptions
- stop if a later handoff treats this source STC as repo-write or workflow-write authority without exact repo paths, exact write scope, and exact verification commands
- do not assume that GitHub issue creation by itself equals usable triage or regression operations
- do not assume authoring-machine success proves clean-machine install/use reality
- do not assume unsupported semantics are generic bugs; they must remain classifiable against SD-13 support-state truth
- do not assume update success or rollback success proves saved-state continuity; SD-14 remains the authority there
- do not let repo README, workspace README, and execution-ledger claims drift independently at tranche closure time without an explicit reconciliation verdict

## Next Stage Rule
- SD-15 is planning-ready because both the source-STC control bundle and its same-epic documentary output artifacts now exist.
- SD-15 source STC does not authorize code by itself.
- Stage-specific code authority exists only where a derived artifact explicitly says `code_authority: true`; as of 2026-07-02 that applies to `artifacts/sd15-e7-release-alpha-test-defect-repair-handoff-2026-07-02.md` for the bounded released-Linux-alpha tester-defect repair bundle.
- Stage-specific code authority also exists for `artifacts/sd15-e8-feedback-output-context-sanitization-handoff-2026-07-02.md`, limited to internal-context sanitization in feedback/update reportable output surfaces.
- The next truthful move is the already-declared workflow card `SD-15 FLOW: Mint bounded execution stories from the SD-15 epic breakdown`, then stage-specific handoff artifacts only for the slices Todd explicitly releases.
- No later SD-15 execution or documentary handoff may claim tranche closure until it names exact evidence inputs, exact surfaces to update, exact stop conditions, and exact reconciliation rules for repo/workspace/ledger truth.
