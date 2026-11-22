---
stc_id: STC-CODEX-SD-12
stc_kind: source-requirements
template_version: 2
work_type: planning-only
workflow_route: planning
readiness: planning-ready
status: active
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch: current live repo state observed 2026-06-29 is branch `sd11-f10-update-action-surface` at commit `c9471f4d7c62236f9afe389e88c1173682f993e2`; live accepted operator promotion truth for distributed tester builds is still `develop -> main`; accepted repo truth now includes `.github/workflows/publish-tester-release.yml` on `origin/develop` via merged PR `#32`, but no accepted runtime `load_sd12_release_truth` or equivalent desktop release-truth boundary exists yet in `apps/desktop/src-tauri/src/main.rs` or `apps/desktop/src/boundary/`, and rollback/recovery presentation remains absent beyond the static SD-11 status/evidence surface
  write_scope: documentary-only updates inside this source STC bundle plus control-plane sync in `programs/codex/requirements/README.md` and the SD-12 strategic surface; no repo implementation-code authority
review_state: draft
last_reviewed_at: 2026-06-29
parent_scopes:
  - programs/codex
source_artifacts:
  - programs/codex/plans/spec-domains/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update.md
  - programs/codex/plans/spec-domains/README.md
  - programs/codex/plans/roadmaps/codex-spec-domain-roadmap-2026-06-18.md
  - programs/codex/requirements/README.md
  - programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md
  - programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md
  - programs/codex/doctrine/program-doctrine-and-scope-charter.md
  - programs/codex/doctrine/documentation-control-plane.md
  - programs/codex/doctrine/quality-gate-policy.md
  - repos/codex/README.md
  - repos/codex/.github/workflows/allow-only-develop-into-main.yml
  - repos/codex/apps/desktop/package.json
  - repos/codex/apps/desktop/src-tauri/tauri.conf.json
  - repos/codex/apps/desktop/src/App.tsx
  - repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts
  - repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts
related_artifacts:
  - programs/codex/README.md
  - programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md
upstream_targets:
  - programs/codex/requirements/README.md
  - programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md
expected_output_artifacts:
  - path: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/distribution-platform-support-and-channel-matrix.md
    completion_rule: Defines the Linux-first, macOS-second-class, and Windows-third-class distribution/support matrix plus the operator-branch to tester-channel mapping SD-11 and later updater work must preserve.
  - path: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/github-artifact-publication-and-promotion-contract.md
    completion_rule: Defines the authoritative GitHub release/prerelease surfaces, artifact classes, promotion rules, and branch-backed publication governance for tester builds.
  - path: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/self-update-transport-and-manifest-contract.md
    completion_rule: Defines the machine-readable update-manifest contract, per-platform update eligibility rules, and the GitHub-backed discovery/retrieval model.
  - path: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/rollback-withdrawal-and-downgrade-policy.md
    completion_rule: Defines the required withdrawn-build, rollback, downgrade, and recovery behavior for distributed tester builds.
  - path: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/provenance-integrity-and-update-eligibility.md
    completion_rule: Defines the minimum build identity, checksum/provenance publication, and platform-specific integrity gates that must pass before self-update claims are honest.
supersedes: []
superseded_by: []
tags:
  - codex
  - sd-12
  - distribution
  - updater
  - release-governance
  - linux-first
---

# SD-12 — Linux-first Distribution, Branch-Promotion Channels, and Self-Update

## Objective
Define the Linux-first distribution, GitHub-backed publication, self-update, rollback, and branch-promotion authority surface that turns Codex tester builds into bounded deliverables without flattening platform asymmetry or leaking raw branch mechanics into product language.

## Deliverable Type
`planning-only`

## Workflow Route
`planning`

## Readiness
`planning-ready`

Why this readiness is accurate:
- the SD-12 strategic spec domain already exists and explicitly frames this lane as a boundary object rather than an implementation prompt
- the durable accepted repo truth now includes both the branch-promotion rule in `repos/codex/.github/workflows/allow-only-develop-into-main.yml` and the merged Linux tester publication workflow in `repos/codex/.github/workflows/publish-tester-release.yml`, but the repo still has no accepted runtime release-truth seam in `repos/codex/apps/desktop/src-tauri/src/main.rs` or `repos/codex/apps/desktop/src/boundary/`
- `repos/codex/apps/desktop/src-tauri/tauri.conf.json` still has `bundle.active: false`, which proves packaging/updater behavior is not already silently complete for end-user desktop consumption
- the repo still has no dedicated TypeScript desktop manifest/update consumer boundary and no dedicated recovery-consumer boundary; current SD-11 desktop truth remains a local status/evidence model layered over GE-08/pilot seams rather than a synchronized SD-12 control-plane consumer
- SD-11 already fixes the tester-facing channel/support vocabulary and status surfaces, so this packet can anchor the underlying artifact/update contract without rewriting the UI lane
- this bundle includes both the control documents and same-epic documentary artifacts needed to keep packaging, publication, integrity, and rollback concrete instead of recursive

## Closure State
SD-12 is generated as a planning-ready source STC on 2026-06-29. It defines the Linux-first tester distribution contract, the GitHub-backed publication/update topology, rollback and withdrawal obligations, and the operator-versus-tester truth boundary that later implementation must preserve. It does not authorize packaging code, CI/release automation changes, updater library selection, or repo-local release engineering by itself. The durable accepted repo truth now includes the buildable desktop proof surface, the branch-promotion rule for `develop -> main`, and the merged Linux tester publication workflow in `.github/workflows/publish-tester-release.yml`. What the repo still does not have is an accepted runtime release-truth seam, a dedicated TypeScript desktop consumer boundary that wires release truth through SD-11 status/evidence surfaces, or a dedicated recovery-consumer/presentation chain. Until those seams are grounded durably, the E3, E4, and E6 executable desktop-consumer lanes remain documentary-only.

## Authority and Scope
- owner: `Todd Hintzmann`
- scope: `program`
- canonical: `true`
- canonical path: `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md`
- parent scopes:
  - `programs/codex`

This STC governs the first bounded tester distribution lane: packaging artifact classes, GitHub publication surfaces, channel/promotion mapping, self-update transport contract, rollback/recovery, provenance/integrity thresholds, and platform-tier promises. SD-11 owns the tester-facing workbench/status surface. SD-12 owns the artifact/update machinery and authority rules underneath it. Today accepted repo truth includes the Linux publication workflow, but the repo still lacks an accepted runtime release-truth seam plus the dedicated TypeScript desktop consumer boundary and rollback/recovery presentation chain that honest downstream desktop coupling would require.

## Target Runtime
- repo: `/home/ubuntu/workspace/repos/codex`
- workdir: `/home/ubuntu/workspace/repos/codex`
- branch/worktree: `current live repo state observed 2026-06-29 is branch sd11-f10-update-action-surface at commit c9471f4d7c62236f9afe389e88c1173682f993e2; official live tester-distribution promotion truth is develop -> main; accepted repo truth now includes the merged publish-tester-release workflow on origin/develop; any future beta/candidate stage remains reserved until a governed backing surface exists in repo/workflow truth`
- allowed write scope: `none during source STC generation beyond this packet and control-plane sync docs; future SD-12 execution handoffs must declare exact repo paths, exact write scope, exact verification commands, and exact release-surface authority before repo or CI files may change`

This bundle is an authority surface under `programs/codex/requirements/`, not a repo-local release runbook.

## Document Map
- `technical-requirements.md` — normative requirements for packaging artifact classes, GitHub publication, self-update transport, rollback/recovery, integrity thresholds, and platform-tier promises
- `technical-design.md` — architecture/design response describing how branch promotion, GitHub release surfaces, update manifests, and desktop update/status surfaces should cooperate without counterfeit parity claims
- `acceptance-and-verification.md` — observable checks proving the SD-12 packet and same-epic documentary outputs remain concrete and reviewable
- `risks-and-open-questions.md` — unresolved package-format, signing/notarization, private-distribution, and updater-library questions isolated from the main contract
- `epic-breakdown.md` — downstream epic/feature decomposition for later same-domain execution-story minting
- `references/upstream-dependency-contract.md` — compact contract naming what SD-11, doctrine, and the live repo release surface do and do not authorize for SD-12
- `artifacts/distribution-platform-support-and-channel-matrix.md` — concrete Linux/macOS/Windows support, delivery, and channel-mapping matrix
- `artifacts/github-artifact-publication-and-promotion-contract.md` — concrete GitHub-backed publication and promotion rules
- `artifacts/self-update-transport-and-manifest-contract.md` — concrete update-manifest and client-consumption contract
- `artifacts/rollback-withdrawal-and-downgrade-policy.md` — concrete withdrawn-build, rollback, and downgrade behavior
- `artifacts/provenance-integrity-and-update-eligibility.md` — concrete build-identity, checksum/provenance, and integrity-gate requirements
- `artifacts/sd12-e3-b1-manifest-producer-and-updater-boundary-truth.md` — documentary repair artifact recording that no live manifest producer or dedicated desktop consumer boundary exists yet and naming the exact unblock condition for later E3 execution work
- `artifacts/sd12-e4-b1-rollback-withdrawal-and-downgrade-boundary-truth-2026-06-29.md` — documentary repair artifact recording that no live recovery-state publication/manifests bridge or dedicated desktop recovery-consumer boundary exists yet and naming the exact unblock condition for later E4 execution work
- `artifacts/sd12-e6-b1-status-and-issue-payload-coupling-truth.md` — documentary repair artifact recording that the accepted publication workflow now exists but no accepted runtime/desktop evidence bridge consumes release truth yet, and naming the exact unblock condition for later E6 synchronization work

## Expected Output Artifacts
| Artifact | Completion rule |
|---|---|
| `artifacts/distribution-platform-support-and-channel-matrix.md` | Defines the Linux-first, macOS-second-class, and Windows-third-class distribution/support matrix plus the operator-branch to tester-channel mapping SD-11 and later updater work must preserve. |
| `artifacts/github-artifact-publication-and-promotion-contract.md` | Defines the authoritative GitHub release/prerelease surfaces, artifact classes, promotion rules, and branch-backed publication governance for tester builds. |
| `artifacts/self-update-transport-and-manifest-contract.md` | Defines the machine-readable update-manifest contract, per-platform update eligibility rules, and the GitHub-backed discovery/retrieval model. |
| `artifacts/rollback-withdrawal-and-downgrade-policy.md` | Defines the required withdrawn-build, rollback, downgrade, and recovery behavior for distributed tester builds. |
| `artifacts/provenance-integrity-and-update-eligibility.md` | Defines the minimum build identity, checksum/provenance publication, and platform-specific integrity gates that must pass before self-update claims are honest. |

## Required Reads
- `../../plans/spec-domains/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update.md` — primary strategic authority for this source STC
- `../../plans/spec-domains/SD-11-test-user-workbench-and-github-feedback-intake.md` — adjacent tester-workbench authority this packet must support without overwriting
- `../SD-11-test-user-workbench-and-github-feedback-intake/README.md` — accepted SD-11 planning surface
- `../SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md` — current operator-to-tester mapping this packet must preserve and deepen
- `../../doctrine/program-doctrine-and-scope-charter.md` — product-scope and evidence-first doctrine
- `../../doctrine/documentation-control-plane.md` — control-plane and authority-surface doctrine
- `../../doctrine/quality-gate-policy.md` — anti-counterfeit-completion and truth-surface doctrine
- `/home/ubuntu/workspace/repos/codex/README.md` — live current-state truth, including the statement that release packaging remains unfinished
- `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml` — current branch-promotion governance surface
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json` — current desktop package identity and toolchain surface
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json` — current packaging surface showing bundle output is not yet activated
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx` — current tester-facing status surface that SD-12 must support, not contradict
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts` — current Linux/macOS/Windows support-tier and channel wording expectations
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` — current bounded update-status language that still refuses to claim updater support

## Conditional Reads
- `/home/ubuntu/workspace/repos/codex/AGENTS.md` — only when a later SD-12 execution handoff is being prepared for repo-facing work
- GitHub Releases / release-assets documentation — only when a later handoff fixes the exact publication transport, token posture, or automation workflow
- Tauri updater/channel documentation — only when a later handoff selects a specific updater implementation path rather than keeping the transport contract abstract
- platform packaging/signing docs for AppImage, `.deb`, macOS notarization, or Windows signing — only when a later handoff fixes exact package formats or trust thresholds
- repo-local workflow/CI files under `/home/ubuntu/workspace/repos/codex/.github/` beyond `allow-only-develop-into-main.yml` — only when later execution creates them explicitly

## In Scope
- Codex SD-12 source-STC documents under `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/`
- same-epic documentary outputs under `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/`
- Linux-first distribution artifact classes and support-tier truth
- GitHub-backed artifact publication, promotion, and update discovery rules
- machine-readable update-manifest and update-eligibility contract
- rollback, withdrawal, downgrade, and recovery obligations
- provenance, integrity, signing/notarization gates, and build-identity requirements
- operator-branch versus tester-channel truth mapping and SD-11 coupling points
- downstream epic decomposition for later same-domain story generation

## Out of Scope
- writing repo implementation code in `/home/ubuntu/workspace/repos/codex`
- selecting a final updater library or CI implementation in this planning pass
- claiming cross-platform packaging parity
- public marketplace or app-store release posture
- cloud distribution, accounts, or non-GitHub artifact hosting
- broad release-management/product-operations doctrine beyond the bounded tester-distribution lane
- changing SD-11 issue intake scope, SD-13 breadth scope, or SD-14 persistence scope

## Acceptance Summary
The acceptance criteria in `acceptance-and-verification.md` are satisfied for the SD-12 planning-ready boundary when this bundle and its named same-epic documentary artifacts exist, remain internally coherent, and define the Linux-first distribution/update lane as a concrete governed surface rather than a vague future promise.

Compact summary:
- Linux remains the first-class packaging and self-update target
- macOS remains second-class but real, and Windows remains explicitly third-class
- live operator branch truth `develop -> main` stays separate from tester-facing channel/support language, and any future `beta`/candidate stage must gain a governed backing surface before it is claimed
- GitHub-backed publication, update discovery, rollback, and integrity rules become explicit enough for bounded execution-story minting
- the manifest/update contract remains documentary authority; accepted repo truth now includes the publication workflow, but executable desktop synchronization remains deferred until an accepted runtime release-truth seam and a dedicated consumer boundary exist and are wired through SD-11 status/evidence surfaces

## Allowed Assumptions
- Linux remains the first-class verified tester platform for this tranche; macOS is second-class and Windows remains explicitly third-class
- the live operator promotion path is `develop -> main`; `beta` or any future candidate-stage label remains reserved until a governed backing surface exists in repo/workflow truth, even if tester-facing UX continues to prefer channel labels such as `alpha`, `beta`, and `stable`
- GitHub remains the authoritative distribution and update-origin surface for this tranche unless a higher-order doctrine surface changes that rule
- SD-11 remains the adjacent authority for tester-facing status language and issue-payload vocabulary; SD-12 extends the underlying artifact/update contract rather than replacing that UI authority

## Blockers / Forbidden Assumptions
- stop if a later handoff treats this source STC as release-automation or updater-code authority without exact repo paths, exact write scope, exact verification commands, and exact platform scope
- do not assume the current branch-protection workflow equals a complete release pipeline
- do not assume `bundle.active: false` can be ignored as though packaging already exists
- do not treat the manifest contract documents themselves as proof that a live manifest producer or desktop consumer seam already exists in the repo
- do not expose raw branch names as primary tester-facing update language merely because the operator model is branch-based
- do not flatten Linux/macOS/Windows into fake distribution parity
- do not claim automatic update on a platform whose integrity gate has not been satisfied explicitly

## Next Stage Rule
- SD-12 is planning-ready because both the source-STC control bundle and its same-epic documentary output artifacts now exist.
- SD-12 has no `execution-handoff.md`; this source STC does not authorize code by itself.
- The next truthful move is the already-linked workflow card `SD-12 FLOW: Mint bounded execution stories from the SD-12 epic breakdown`, then stage-specific handoff artifacts only for the slices Todd explicitly releases.
- No E3 manifest/update handoff may be authored until a future lane can name a dedicated desktop consumer boundary that truthfully consumes accepted publication/release-truth surfaces, or until higher-order authority revises the claim again.
- No E4 rollback/withdrawal/downgrade handoff may be authored until a future lane can name both a live publication/manifests bridge for withdrawn/superseded/blocked/recovery-preferred state and a dedicated desktop recovery-consumer boundary in repo truth, or until higher-order authority revises the claim again.
- SD-11 updater/status implementation may now consume this packet as governing documentary authority plus the currently observed candidate publication/release-truth surfaces, but not as proof that the full desktop producer/consumer chain already exists.
