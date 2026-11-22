---
stc_id: STC-CODEX-SD-14
stc_kind: source-requirements
template_version: 2
work_type: planning-only
workflow_route: planning
readiness: planning-ready
status: active
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/README.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch: current live repo state observed 2026-06-30 is branch `sd11-f10-update-action-surface` at commit `a0a90dcdf75cd0cc09d4b71e9fb7d3b440aaf293`; current accepted distributed-tester/update baseline remains `origin/develop` at commit `c9471f4d7c62236f9afe389e88c1173682f993e2`; accepted repo truth includes GE-08 authored-package persistence and SD-11/SD-12 update surfaces, but no accepted character save/load, revision, or migration boundary exists yet
  write_scope: documentary-only updates inside this source STC bundle plus control-plane sync in `programs/codex/requirements/README.md`; no repo implementation-code authority
review_state: draft
last_reviewed_at: 2026-06-30
parent_scopes:
  - programs/codex
source_artifacts:
  - programs/codex/plans/spec-domains/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision.md
  - programs/codex/plans/spec-domains/README.md
  - programs/codex/plans/roadmaps/codex-spec-domain-roadmap-2026-06-18.md
  - programs/codex/requirements/README.md
  - programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md
  - programs/codex/requirements/GE-10-demo-proof-and-onboarding/README.md
  - programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md
  - programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/tester-feedback-evidence-capture-matrix.md
  - programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/rollback-withdrawal-and-downgrade-policy.md
  - programs/codex/doctrine/program-doctrine-and-scope-charter.md
  - programs/codex/doctrine/documentation-control-plane.md
  - programs/codex/doctrine/quality-gate-policy.md
  - repos/codex/README.md
  - repos/codex/src/rules_core/character_input.rs
  - repos/codex/src/homebrew_authoring/package_store.rs
  - repos/codex/apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts
  - repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts
related_artifacts:
  - programs/codex/README.md
  - programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md
upstream_targets:
  - programs/codex/requirements/README.md
  - programs/codex/plans/roadmaps/codex-execution-status-ledger-2026-06-21.md
expected_output_artifacts:
  - path: programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/artifacts/persisted-character-state-contract.md
    completion_rule: Defines the authoritative persisted character envelope, mandatory identity/version/provenance fields, and the explicit split between authoritative saved choices and recomputable derived state.
  - path: programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/artifacts/character-lifecycle-operations-contract.md
    completion_rule: Defines the create/open/save/reopen/duplicate/archive/delete lifecycle rules for bounded local character artifacts and the blocked outcomes that must remain visible.
  - path: programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/artifacts/revision-autosave-and-recovery-policy.md
    completion_rule: Defines what counts as the latest authoritative save, the minimum autosave/backup posture, interrupted-write recovery behavior, and how unsaved work remains visible.
  - path: programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/artifacts/upgrade-migration-and-compatibility-contract.md
    completion_rule: Defines the schema/app/content/provenance compatibility checks, migration outcome states, and the upgrade-safe survival rules that later SD-12 update flows must preserve.
  - path: programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/artifacts/corrupt-incompatible-and-missing-dependency-diagnostics.md
    completion_rule: Defines the required diagnostic classes, user-visible outcomes, and evidence-capture posture for corrupt, incompatible, partial, or missing-dependency saved state.
supersedes: []
superseded_by: []
tags:
  - codex
  - sd-14
  - persistence
  - lifecycle
  - migration
  - upgrade-safety
  - saved-state
---

# SD-14 — Character lifecycle, persistence, and upgrade-safe revision

## Objective
Define the local-first, single-user saved-character authority surface that lets bounded Codex-supported characters survive close/reopen, revision, and governed app/content upgrades without silent reset, counterfeit durability claims, or campaign-management sprawl.

## Deliverable Type
`planning-only`

## Workflow Route
`planning`

## Readiness
`planning-ready`

Why this readiness is accurate:
- the SD-14 strategic spec domain already exists and explicitly frames this lane as a boundary object rather than an implementation prompt
- adjacent accepted planning truth already exists: GE-06 grounds bounded character input/computation, GE-10 grounds the current developer-proof desktop posture, SD-11 grounds tester workbench/evidence surfaces, and SD-12 grounds distribution/update/rollback truth
- the live repo proves an adjacent persistence pattern in `src/homebrew_authoring/package_store.rs` and `apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts`, but that persistence is for authored proof packages rather than character lifecycle truth; this packet can therefore ground what exists without pretending character save/load already exists
- the current tester-workbench evidence model already recognizes `save-file` attachments, proving downstream evidence capture is expected, while still leaving character-state durability itself undefined
- this bundle includes both the control documents and concrete same-epic documentary artifacts needed to keep persistence, revision, migration, and failure posture explicit instead of recursive

## Closure State
SD-14 is generated as a planning-ready source STC on 2026-06-30. It defines the saved-character authority surface for local-first single-user persistence, lifecycle operations, revision/autosave, compatibility/migration, and failure diagnostics that later implementation must preserve. It does not authorize repo implementation code, storage-technology selection, cloud sync, campaign-management breadth, or silent update-time mutation. Current repo truth remains that Codex has deterministic character-input and computation surfaces plus GE-08 authored-package persistence, but no accepted character save/load, character revision, or character-migration boundary yet exists.

## Authority and Scope
- owner: `Todd Hintzmann`
- scope: `program`
- canonical: `true`
- canonical path: `programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/README.md`
- parent scopes:
  - `programs/codex`

This STC governs bounded saved-character continuity: persisted character identity, authoritative versus derived state boundaries, save/load/reopen/revise/duplicate/archive/delete behavior, revision/autosave and recovery posture, compatibility/migration rules, and diagnostic/evidence obligations. GE-06 owns the current bounded character-computation truth. SD-11 owns tester workbench and evidence-capture UX. SD-12 owns distribution/update/rollback transport truth. SD-14 owns whether local character state survives those adjacent flows honestly.

## Target Runtime
- repo: `/home/ubuntu/workspace/repos/codex`
- workdir: `/home/ubuntu/workspace/repos/codex`
- branch/worktree: `current live repo state observed 2026-06-30 is branch sd11-f10-update-action-surface at commit a0a90dcdf75cd0cc09d4b71e9fb7d3b440aaf293; current accepted distributed-tester/update baseline remains origin/develop at c9471f4d7c62236f9afe389e88c1173682f993e2; current repo truth includes GE-08 package persistence and SD-11/SD-12 update surfaces but no accepted character save/load, revision, or migration boundary`
- allowed write scope: `none during source STC generation beyond this packet and control-plane sync docs; future SD-14 execution handoffs must declare exact repo paths, exact write scope, exact required reads, and exact verification commands before repo files may change`

This bundle is an authority surface under `programs/codex/requirements/`, not a repo-local implementation brief.

## Document Map
- `technical-requirements.md` — normative requirements for saved-character classes, lifecycle behavior, authoritative-versus-derived state boundaries, compatibility vectors, and failure/recovery rules
- `technical-design.md` — architecture/design response describing how local state, revisions, migration gates, diagnostics, and adjacent SD-11/SD-12 surfaces should cooperate without turning the desktop shell into persistence authority
- `acceptance-and-verification.md` — observable checks proving the SD-14 packet and same-epic documentary outputs remain concrete and reviewable
- `risks-and-open-questions.md` — unresolved storage-format, migration-strategy, autosave-depth, missing-content, and progression-breadth questions isolated from the main contract
- `epic-breakdown.md` — downstream epic/feature decomposition for later same-domain execution-story minting
- `references/upstream-dependency-contract.md` — compact contract naming what GE-06, GE-10, SD-11, SD-12, and the live repo do and do not authorize for SD-14
- `artifacts/persisted-character-state-contract.md` — concrete saved-character envelope and authoritative-state contract
- `artifacts/character-lifecycle-operations-contract.md` — concrete lifecycle-operation rules for bounded local characters
- `artifacts/revision-autosave-and-recovery-policy.md` — concrete revision/autosave/restore posture
- `artifacts/upgrade-migration-and-compatibility-contract.md` — concrete compatibility/migration and upgrade-survival contract
- `artifacts/corrupt-incompatible-and-missing-dependency-diagnostics.md` — concrete failure-mode and diagnostic contract

## Expected Output Artifacts
| Artifact | Completion rule |
|---|---|
| `artifacts/persisted-character-state-contract.md` | Defines the authoritative persisted character envelope, mandatory identity/version/provenance fields, and the explicit split between authoritative saved choices and recomputable derived state. |
| `artifacts/character-lifecycle-operations-contract.md` | Defines the create/open/save/reopen/duplicate/archive/delete lifecycle rules for bounded local character artifacts and the blocked outcomes that must remain visible. |
| `artifacts/revision-autosave-and-recovery-policy.md` | Defines what counts as the latest authoritative save, the minimum autosave/backup posture, interrupted-write recovery behavior, and how unsaved work remains visible. |
| `artifacts/upgrade-migration-and-compatibility-contract.md` | Defines the schema/app/content/provenance compatibility checks, migration outcome states, and the upgrade-safe survival rules that later SD-12 update flows must preserve. |
| `artifacts/corrupt-incompatible-and-missing-dependency-diagnostics.md` | Defines the required diagnostic classes, user-visible outcomes, and evidence-capture posture for corrupt, incompatible, partial, or missing-dependency saved state. |

## Required Reads
- `../../plans/spec-domains/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision.md` — primary strategic authority for this source STC
- `../GE-06-pilot-vertical-slice-pf1-human-fighter/README.md` — bounded character-input and computation truth this lane must preserve over time
- `../GE-10-demo-proof-and-onboarding/README.md` — current developer-proof desktop posture and onboarding truth this lane must not overstate
- `../SD-11-test-user-workbench-and-github-feedback-intake/README.md` — tester-workbench and evidence-capture authority that SD-14 must support without replacing
- `../SD-11-test-user-workbench-and-github-feedback-intake/artifacts/tester-feedback-evidence-capture-matrix.md` — current evidence-capture expectations including save-file handling
- `../SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md` — current operator-versus-tester vocabulary this lane must not contradict
- `../SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md` — distribution/update/rollback authority this lane must inherit at the saved-state seam
- `../SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/rollback-withdrawal-and-downgrade-policy.md` — adjacent rollback and recovery posture this packet must align with
- `../../doctrine/program-doctrine-and-scope-charter.md` — local-first scope, headless-core-first, and anti-sprawl doctrine
- `../../doctrine/documentation-control-plane.md` — control-plane and authority-surface doctrine
- `../../doctrine/quality-gate-policy.md` — evidence and anti-counterfeit-completion doctrine
- `/home/ubuntu/workspace/repos/codex/README.md` — current repo state and bounded product claims
- `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs` — current bounded character-input substrate that future persistence must preserve rather than reinterpret
- `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/package_store.rs` — current adjacent persistence implementation proving only authored-package lifecycle, not character-state lifecycle
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts` — current adjacent desktop lifecycle-gate surface (`saveAllowed`, `exportAllowed`, `diffMode`) that must not be misread as character persistence truth
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` — current tester-workbench consumer posture that later persistence surfaces may need to feed

## Conditional Reads
- `/home/ubuntu/workspace/repos/codex/AGENTS.md` — only when a later SD-14 execution handoff is being prepared for repo-facing work
- Tauri/local storage docs, SQLite/file-format docs, or platform filesystem guidance — only when a later handoff selects a concrete persistence backend instead of preserving the contract abstractly
- repo-local desktop/runtime files under `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/` — only when a later handoff grounds the exact runtime save/load seam
- later SD-13 breadth artifacts — only when future persistence work must prove how expanded class/race/level coverage changes saved-state compatibility
- issue/telemetry/docs for corrupted files or migration failures — only when a later handoff fixes the exact repair/reporting transport

## In Scope
- Codex SD-14 source-STC documents under `programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/`
- same-epic documentary outputs under `programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/artifacts/`
- local-first single-user saved-character identity, lifecycle, revision, and migration contracts
- authoritative-versus-derived state boundaries for saved characters
- compatibility, rollback-interaction, corruption, missing-dependency, and recovery posture for saved state
- tester-visible evidence and diagnostic obligations for save/load and upgrade failures
- downstream epic decomposition for later same-domain story generation

## Out of Scope
- writing implementation code in `/home/ubuntu/workspace/repos/codex`
- cloud sync, accounts, remote collaboration, or cross-device state replication
- party/campaign/world-state management
- broad rules/content coverage expansion that belongs to SD-13 and beyond
- replacing SD-11 tester-workbench authority or SD-12 release/update authority
- claiming current repo already has character persistence because GE-08 authored-package persistence exists
- silent migration or silent reset behavior justified only by implementation convenience

## Acceptance Summary
The acceptance criteria in `acceptance-and-verification.md` are satisfied for the SD-14 planning-ready boundary when this bundle and its named same-epic documentary artifacts exist, remain internally coherent, and define saved-character continuity as a concrete governed surface rather than a future convenience promise.

Compact summary:
- saved-character identity, provenance, and version vectors are explicit enough to decide reopen, migrate, block, or recover honestly
- save/load/reopen/revise/duplicate/archive/delete behavior is defined as product-truth obligations, not folklore or UI convenience
- authoritative user-authored state is separated from recomputable derived state so later implementation cannot freeze accidental snapshots as canonical truth
- upgrade, rollback-adjacent, corrupt, incompatible, and missing-dependency outcomes remain visible and evidence-bearing instead of silent
- the lane stays local-first and single-user without drifting into cloud sync or campaign-management scope

## Allowed Assumptions
- Codex remains a local-first single-user product in this tranche; no account or cloud substrate is required to make saved characters meaningful
- GE-06 bounded character-input/computation truth is the correct current character-domain substrate; persistence must preserve that truth rather than replace it
- GE-08 package persistence may be treated as an adjacent implementation pattern for versioned local artifacts, but not as proof that character persistence already exists
- SD-11 remains the adjacent authority for tester-facing evidence capture and support wording, and SD-12 remains the adjacent authority for update/rollback transport truth

## Blockers / Forbidden Assumptions
- stop if a later handoff treats this source STC as repo-write authority without exact repo paths, exact write scope, and exact verification commands
- do not assume the current repo already supports character save/load merely because authored packages can be persisted
- do not assume every computed field must be serialized; derived state may be recomputed when the contract says it is non-authoritative
- do not assume upgrade transport success from SD-12 implies saved-character survival; SD-14 must prove the saved-state seam separately
- do not broaden this lane into roster management, cloud sync, or campaign-management scope by implication
- do not permit silent migration, silent field dropping, or silent reset on corrupted/incompatible state

## Next Stage Rule
- SD-14 is planning-ready because both the source-STC control bundle and its same-epic documentary output artifacts now exist.
- SD-14 has no `execution-handoff.md`; this source STC does not authorize code by itself.
- The next truthful move is the already-declared workflow card `SD-14 FLOW: Mint bounded execution stories from the SD-14 epic breakdown`, then stage-specific handoff artifacts only for the slices Todd explicitly releases.
- No later persistence handoff may claim upgrade-safe durability until it names exact compatibility vectors, exact migration/blocked/read-only outcomes, and exact verification evidence for close/reopen and version-change survival.
