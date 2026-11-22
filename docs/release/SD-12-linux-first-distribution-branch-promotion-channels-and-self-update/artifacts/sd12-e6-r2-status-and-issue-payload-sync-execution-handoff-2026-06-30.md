---
title: SD12-E6-F11/F12 Execution Handoff — Status-surface and issue-payload truth synchronization
handoff_id: HANDOFF-CODEX-SD12-E6-F11-F12-STATUS-ISSUE-TRUTH-SYNC-2026-06-30
stc_id: STC-CODEX-SD-12
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: merged
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e6-r2-status-and-issue-payload-sync-execution-handoff-2026-06-30.md
source_stc: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md
source_epic_breakdown: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/epic-breakdown.md
source_readiness_closure: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e6-r1-execution-readiness-closure-2026-06-29.md
source_blocker_repair: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e6-b1-status-and-issue-payload-coupling-truth.md
selected_slice: SD12-E6-F11/F12 — Status-surface and issue-payload truth synchronization
run_in: Claude Code
code_authority: false
authority_dependencies:
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/distribution-platform-support-and-channel-matrix.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/self-update-transport-and-manifest-contract.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/provenance-integrity-and-update-eligibility.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e6-r1-execution-readiness-closure-2026-06-29.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e6-b1-status-and-issue-payload-coupling-truth.md
  - programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md
  - repos/codex/AGENTS.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  observed_local_branch: feat/sd13-e6-f11-support-state-debt-presentation
  observed_local_head: 122de6a60609d9452de53c6d3ad406aeb81c2a82
  observed_origin_develop: 60973f94ba91b3af8f918f655a9f21e679d97b17
  historical_execution_branch: feat/sd12-e6-release-truth-bridge
  historical_branch_base_sha: a42859ae12dfafb917d2bf25f0e6e7ef951e13b9
  implementation_commit: 13484b1a1d510c6c186d73fa715ba23e447e532b
  merged_commit: 60973f94ba91b3af8f918f655a9f21e679d97b17
  merged_pr: 39
  pr_target: develop
completion_class: merged
reviewed_at: 2026-06-30
allowed_write_scope:
  - apps/desktop/src/boundary/loadSd12ReleaseTruth.ts
  - apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts
  - apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts
  - apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts
  - apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts
  - apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts
  - apps/desktop/src/sd11/feedback/evidence/evidenceFields.ts
  - apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts
  - apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts
  - apps/desktop/src/sd11/feedback/bug/composeBugReport.ts
  - apps/desktop/src/sd11/feedback/bug/composeBugReport.test.ts
  - apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.ts
  - apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.test.ts
forbidden_write_scope:
  - programs/codex/**
  - .github/workflows/**
  - apps/desktop/src/App.tsx
  - apps/desktop/package.json
  - apps/desktop/tsconfig.json
  - apps/desktop/src-tauri/**
  - apps/desktop/src/sd11/feedback/bug/submitBugReport.test.ts
  - apps/desktop/src/sd11/feedback/enhancement/submitEnhancementRequest.test.ts
  - apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts
  - apps/desktop/src/boundary/loadPilotShellSnapshot.ts
  - src/**
  - AGENTS.md
  - CLAUDE.md
---

# SD12-E6-F11/F12 Execution Handoff — Status-surface and issue-payload truth synchronization

## Status
This is the preserved stage-specific handoff for the bounded SD12-E6 synchronization slice that is now accepted repo truth on `origin/develop` via merged PR `#39` (`60973f94ba91b3af8f918f655a9f21e679d97b17`).

It exists as historical bounded authority, not as a live launch packet. No downstream `CODE` card should be opened from this artifact alone, because the exact slice it authorizes is already merged. If later regression or follow-on work is needed, mint a new readiness closure and a new slice-specific handoff rather than reusing this one.

This artifact does not claim a durable Claude execution receipt for the already-merged implementation. It preserves the exact bounded brief the merge now proves, nothing more.

Current documentary artifact card: `t_ac9ffc3e`.

## Run in
Claude Code.

If this slice ever has to be reissued, do not silently implement it through Hermes file-editing tools. Reissue a fresh code-authorizing handoff plus governed `CODE` card from current `origin/develop` truth.

## Core problem
SD-12 documentary truth had already standardized the operator promotion posture as `develop -> main`, reserved `beta` until a governed backing surface exists, and required SD-11 to consume artifact/update control-plane truth instead of improvising it locally.

The live desktop repo had been lying in two places at once:
- `createSd11WorkbenchStatus.ts` still carried stale `develop -> uat -> main` doctrine and a local-only update posture
- the shared SD-11 issue-evidence and draft payload chain still captured only local workbench facts, not release-truth handles from the accepted publication/update surface

The correct slice was not to widen into SD12-E3 manifest ownership, SD12-E4 rollback ownership, SD12-E5 provenance/trust-gate ownership, or broader UI rewrites. The correct slice was one bounded bridge that consumes accepted release truth and threads it through the existing SD-11 status, surface, evidence, and issue-draft chain.

## Objective
Implement one narrow SD12-E6 synchronization slice that proves all of the following without broadening ownership:

1. SD-11 status truth consumes accepted SD-12 release/control-plane truth instead of forking doctrine locally.
2. The workbench surface and shared issue-evidence backbone expose the same synchronized release-truth fields.
3. Bug and enhancement draft composition preserve those synchronized fields identically where the shared evidence layer already applies.
4. Tester-facing channel/support language stays primary, while operator branch lineage remains auditable but secondary.
5. The slice stops at the already-accepted SD-11 status/evidence/payload surfaces plus the accepted release-truth bridge file, and does not sprawl into workflows, Rust/Tauri, manifest production, rollback semantics, provenance emission, or unrelated SD-11 transport UX.

## Why this route was authorized
The blocker was documentary until accepted repo truth could prove a real bridge existed. That proof now exists.

Grounded facts on 2026-06-30:
- `origin/develop` is `60973f94ba91b3af8f918f655a9f21e679d97b17`, the merge commit for PR `#39` (`feat: bridge SD12 release truth into SD11 evidence`).
- The implementing commit is `13484b1a1d510c6c186d73fa715ba23e447e532b`, based from `a42859ae12dfafb917d2bf25f0e6e7ef951e13b9`.
- The accepted diff is confined to exactly the thirteen files in `allowed_write_scope`.
- No `.github/workflows/**`, no `apps/desktop/src-tauri/**`, and no `apps/desktop/src/App.tsx` changes were required.
- The existing workbench UI already rendered `surface.status.*`, so the truthful move was to fix the status/evidence/payload inputs rather than invent a second UI doctrine surface.
- The following commands are green on the live repo right now:
  - `npx --yes tsx src/sd11/status/createSd11WorkbenchStatus.test.ts`
  - `npx --yes tsx src/sd11/loadSd11TesterWorkbenchSurface.test.ts`
  - `npx --yes tsx src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts`
  - `npx --yes tsx src/sd11/feedback/bug/composeBugReport.test.ts`
  - `npx --yes tsx src/sd11/feedback/enhancement/composeEnhancementRequest.test.ts`
  - `npm run typecheck`
  - `npm run build`
  - `npm run tauri:check`

That is the decisive boundary. The slice is real, bounded, verified, and already merged.

## Target repo and branch policy
```text
repo:    /home/ubuntu/workspace/repos/codex
workdir: /home/ubuntu/workspace/repos/codex
```

Observed workspace facts while authoring this preserved handoff:
- current local branch: `feat/sd13-e6-f11-support-state-debt-presentation`
- current local `HEAD`: `122de6a60609d9452de53c6d3ad406aeb81c2a82`
- current remote base: `origin/develop` at `60973f94ba91b3af8f918f655a9f21e679d97b17`
- unrelated local residue exists as untracked `apps/desktop/src-tauri/gen/`

Historical execution facts for the merged slice:
- implementation branch: `feat/sd12-e6-release-truth-bridge`
- implementation base: `a42859ae12dfafb917d2bf25f0e6e7ef951e13b9`
- implementing commit: `13484b1a1d510c6c186d73fa715ba23e447e532b`
- merged to `develop` by PR `#39`

If a future repair is needed, do not revive `feat/sd12-e6-release-truth-bridge`. Launch a fresh branch from the then-current `origin/develop` and issue a new handoff.

## Exact required reads before coding
Read these first, in order, before touching code for any reissued version of this slice:
1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md`
3. `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/epic-breakdown.md`
4. `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e6-r1-execution-readiness-closure-2026-06-29.md`
5. `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e6-b1-status-and-issue-payload-coupling-truth.md`
6. `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/distribution-platform-support-and-channel-matrix.md`
7. `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/self-update-transport-and-manifest-contract.md`
8. `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/provenance-integrity-and-update-eligibility.md`
9. `/home/ubuntu/workspace/programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md`
10. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadSd12ReleaseTruth.ts`
11. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
12. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
13. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts`
14. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/evidenceFields.ts`
15. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts`
16. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.ts`
17. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.ts`
18. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`
19. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts`
20. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts`
21. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.test.ts`
22. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.test.ts`

Use them as bounded authority surfaces, not as permission to widen the lane.

## Exact allowed write scope
You may create or modify only these repo paths:
- `apps/desktop/src/boundary/loadSd12ReleaseTruth.ts`
- `apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
- `apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`
- `apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
- `apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts`
- `apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts`
- `apps/desktop/src/sd11/feedback/evidence/evidenceFields.ts`
- `apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts`
- `apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts`
- `apps/desktop/src/sd11/feedback/bug/composeBugReport.ts`
- `apps/desktop/src/sd11/feedback/bug/composeBugReport.test.ts`
- `apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.ts`
- `apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.test.ts`

Write-scope interpretation:
- the new boundary file is the only new release-truth bridge surface authorized here
- `createSd11WorkbenchStatus.ts` remains the sole status model home for channel/support/update posture
- `loadSd11TesterWorkbenchSurface.ts` remains the sole SD-11 composition surface authorized to combine the release-truth bridge with the existing workbench model
- `loadSd11TesterWorkbenchSurfaceRuntime.ts` may change only to thread the bridge into the existing runtime seam
- the shared evidence and issue composers may change only to carry and render synchronized release-truth fields already owned by the shared SD-11 issue backbone
- the five listed `*.test.ts` files are the only direct proof surfaces authorized for this slice

No other repo file is in write scope.

## Forbidden write scope and explicit non-goals
This handoff does not authorize:
- any edits under `/home/ubuntu/workspace/programs/codex/**`
- any edits under `/home/ubuntu/workspace/repos/codex/.github/workflows/**`
- any edits under `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/**`
- any edits to `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
- any edits to `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json` or `/home/ubuntu/workspace/repos/codex/apps/desktop/tsconfig.json`
- any edits to `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts`
- any edits to `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadPilotShellSnapshot.ts`
- any edits to `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/submitBugReport.test.ts`
- any edits to `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/enhancement/submitEnhancementRequest.test.ts`
- any edits under `/home/ubuntu/workspace/repos/codex/src/**`
- any manifest-producer ownership, workflow-publication ownership, provenance emission, checksum generation, trust-gate computation, rollback logic, withdrawn-build handling, or recovery-surface implementation
- any claim that `beta` is live-backed
- any UI rewrite that makes raw branch names primary tester-facing language
- any widening into GitHub transport/auth, attachment handling, redaction handling, or unrelated SD-11 workflow UX

If truthful completion would require touching a forbidden surface, stop and route back through a new readiness closure.

## Contract to implement
Implement one read-only synchronization bridge from the accepted SD-12 release/update control plane into the existing SD-11 status/evidence/payload chain.

### Required release-truth bridge
Required file surface:
```text
apps/desktop/src/boundary/loadSd12ReleaseTruth.ts
```

The bridge must:
- consume the already-accepted `sd11_update_action` surface rather than inventing a new Rust/Tauri command for this slice
- normalize a single bounded snapshot containing `truth`, `updateAction`, and `issueCapture`
- preserve, at minimum, these issue-capture fields when available:
  - `releaseUnitId`
  - `sourceRevision`
  - `manifestPath`
  - `updateEligibilityState`
  - `trustGateStatus`
  - `replacementReleaseId`
  - `officialSurface`
  - `localBuildAuthority`
- stay read-only and documentary; it must not become a manifest producer, updater runner, release publisher, or rollback engine

### Required status and surface synchronization
Required file surfaces:
```text
apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts
apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts
apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts
```

The synchronized status/surface lane must:
- derive tester-facing channel/support/update truth from the accepted release-truth snapshot when present
- preserve the operator mapping as `develop -> main`
- keep `beta` explicitly reserved/unavailable until governed backing truth exists
- keep Linux first-class, macOS second-class, and Windows third-class posture honest
- thread the synchronized release-truth handle into `surface.status.issueCapture.releaseTruth`
- keep the existing SD-11 workbench structure intact instead of inventing a new presentation model

Important preservation rule:
- `App.tsx` already renders `surface.status.*`; this slice must fix the underlying status/source truth, not widen into a second UI doctrine surface

### Required shared evidence and issue-payload coupling
Required file surfaces:
```text
apps/desktop/src/sd11/feedback/evidence/evidenceFields.ts
apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts
apps/desktop/src/sd11/feedback/bug/composeBugReport.ts
apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.ts
```

The shared issue-evidence lane must:
- auto-capture the synchronized release-truth fields from the SD-11 surface once, in the shared evidence backbone
- ensure bug and enhancement flows inherit identical auto-captured release-truth values
- keep tester-entered fields distinct from auto-captured release-control-plane truth
- render release-truth context in the composed draft bodies where the shared evidence layer already governs structured metadata
- avoid creating separate bug-only or enhancement-only truth forks

### Required test posture
Required proof surfaces:
```text
apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts
apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts
apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts
apps/desktop/src/sd11/feedback/bug/composeBugReport.test.ts
apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.test.ts
```

Those tests must be the contract carriers for this slice. The slice is not complete because the implementation "looks right." It is complete only when these proof files force the synchronized truth.

## TDD requirement
TDD is mandatory.

### RED
Update the existing proof files first so they fail without the bridge:
1. `createSd11WorkbenchStatus.test.ts` must fail until the status model reflects `develop -> main` plus synchronized release-truth issue-capture fields.
2. `loadSd11TesterWorkbenchSurface.test.ts` must fail until the runtime/composition path threads the accepted release-truth snapshot into the workbench surface.
3. `captureFeedbackEvidence.test.ts` must fail until the shared evidence backbone carries the synchronized auto-captured release-truth fields identically for bug and enhancement flows.
4. `composeBugReport.test.ts` must fail until bug draft composition preserves the synchronized release-truth context.
5. `composeEnhancementRequest.test.ts` must fail until enhancement draft composition preserves the synchronized release-truth context.

Run the targeted proof commands after making the failing assertions.

### GREEN
Implement the smallest change necessary to make those proof files pass:
- add the bridge file
- thread it through the existing SD-11 runtime/surface/status chain
- extend the shared evidence catalog only as far as needed for synchronized release-truth capture
- update bug and enhancement composition only where the shared evidence contract already governs the output

Do not change `App.tsx`, workflow files, Rust/Tauri surfaces, or manifest/provenance/rollback ownership.

### VERIFY
Run these commands at minimum:

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npx --yes tsx src/sd11/status/createSd11WorkbenchStatus.test.ts
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npx --yes tsx src/sd11/loadSd11TesterWorkbenchSurface.test.ts
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npx --yes tsx src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npx --yes tsx src/sd11/feedback/bug/composeBugReport.test.ts
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npx --yes tsx src/sd11/feedback/enhancement/composeEnhancementRequest.test.ts
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run typecheck
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run build
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run tauri:check
```

### SCOPE AUDIT
Run:

```bash
git -C /home/ubuntu/workspace/repos/codex diff --name-only
git -C /home/ubuntu/workspace/repos/codex ls-files --others --exclude-standard
```

Confirm every changed or newly created product file is inside the allowed write scope.

## Acceptance criteria
This preserved handoff is historically satisfied only because all of the following are now true in accepted repo truth:

- [x] `apps/desktop/src/boundary/loadSd12ReleaseTruth.ts` exists as the bounded release-truth bridge.
- [x] `createSd11WorkbenchStatus.ts` and `createSd11WorkbenchStatus.test.ts` now preserve `develop -> main` rather than stale `develop -> uat -> main` doctrine.
- [x] `loadSd11TesterWorkbenchSurface.ts`, `loadSd11TesterWorkbenchSurfaceRuntime.ts`, and `loadSd11TesterWorkbenchSurface.test.ts` now thread synchronized release truth into the SD-11 surface.
- [x] `evidenceFields.ts`, `captureFeedbackEvidence.ts`, and `captureFeedbackEvidence.test.ts` now carry shared synchronized release-truth fields for both issue flows.
- [x] `composeBugReport.ts` plus its test preserve the synchronized release-truth context in bug drafts.
- [x] `composeEnhancementRequest.ts` plus its test preserve the synchronized release-truth context in enhancement drafts.
- [x] The accepted merged diff is confined to the thirteen authorized files and does not widen into workflows, `App.tsx`, `src-tauri`, or upstream program docs.
- [x] The five targeted TypeScript proof commands are green on the live repo.
- [x] `npm run typecheck`, `npm run build`, and `npm run tauri:check` are green on the live repo.
- [x] PR `#39` merged this bounded slice into `develop` as commit `60973f94ba91b3af8f918f655a9f21e679d97b17`.

## Stop conditions for any future reissue
Stop and create a new readiness closure instead of widening the slice if any of these become true:
- truthful completion would require edits outside the thirteen allowed repo files
- a new Rust/Tauri command, workflow change, manifest producer, provenance emitter, rollback state, or recovery-state UI becomes necessary
- `beta` must become live-backed without a governed upstream promotion surface
- the shared issue-evidence lane can no longer express the needed release truth without expanding into broader SD-11 transport ownership
- the verification commands above fail after the bounded change

## Merge authority boundary
This preserved artifact records a slice that is already merged.

If reissued in the future, the execution lane would still stop short of merge authority:
- branch and verification are in scope
- merge to `develop` or `main` is not in scope
- any future run must return control through a governed review/merge surface

## Final rule
The slice is smaller than release engineering and smaller than UI redesign. Consume accepted release truth through the thirteen-file SD-11 status/evidence/payload lane, keep tester language honest, keep branch lineage secondary, and stop there.
