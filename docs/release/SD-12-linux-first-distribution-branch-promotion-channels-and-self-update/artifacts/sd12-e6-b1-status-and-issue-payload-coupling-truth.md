# SD12-E6-B1 — Status-surface and issue-payload coupling truth

## Outcome
- accepted upstream SD-12 control-plane truth after this repair:
  - `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml`
  - `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml` on `origin/develop`, merged via PR #32 (`48cf50df0f17ac05ae88a4ceadf0b5e726cf0a55`)
- accepted desktop/evidence bridge truth after this repair: `none yet on origin/develop`; candidate bridge truth now exists on `feat/sd12-e6-release-truth-bridge` / PR #39 (`13484b1a1d510c6c186d73fa715ba23e447e532b`) but is not yet merged accepted repo truth
- SD12-E6-R2 verdict: `blocked on merge authority, not on implementation substance`

## Exact remaining unblock condition for SD12-E6-R2
`SD12-E6-R2 FLOW: SD-11 and SD-12 truth-synchronization handoff artifact` must remain blocked until all of the following are true at the same time:

As of 2026-06-30, PR #39 (`feat: bridge SD12 release truth into SD11 evidence`) implements the missing bridge at candidate-branch scope and verifies locally, but the current credential surface can create/comment on PRs while failing both GraphQL and REST merge attempts with `Resource not accessible by personal access token`. Until a merge-capable authority lands that PR on `origin/develop`, predicates 1-4 remain false at accepted-repo-truth scope even though the branch implementation exists.

1. An accepted runtime release-truth surface is named exactly. `origin/develop` no longer carries the old local `load_sd12_release_truth` residue in `apps/desktop/src-tauri/src/main.rs`, but it also does not expose an accepted replacement there or elsewhere.
2. A dedicated TypeScript desktop release-truth boundary exists under `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/` and is named exactly. The narrowest truthful future home remains `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadSd12ReleaseTruth.ts`, but that file does not exist today and must not be treated as live truth.
3. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts` and `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` consume that bridge instead of wiring only GE-08 and pilot-shell loaders plus a locally manufactured status model.
4. The SD-11 status/evidence chain consumes the same bridge instead of forking local truth across these exact surfaces:
   - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
   - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/evidenceFields.ts`
   - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts`
   - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.ts`
   - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.ts`

Until those four statements are true, any E6 handoff would counterfeit an executable synchronization chain that the repo does not yet own.

## Verification run
Observed directly on 2026-06-30:

```text
repo branch:        feat/sd12-e6-release-truth-bridge
repo head:          13484b1a1d510c6c186d73fa715ba23e447e532b
origin/feature:     13484b1a1d510c6c186d73fa715ba23e447e532b
origin/develop:     a42859ae12dfafb917d2bf25f0e6e7ef951e13b9
working tree:       clean after commit `feat: bridge SD12 release truth into SD11 evidence`
merged publication surface:  #32 https://github.com/electricm0nk/codex/pull/32
open bridge PR surface: #39 https://github.com/electricm0nk/codex/pull/39
merged workflow commit: 48cf50df0f17ac05ae88a4ceadf0b5e726cf0a55
boundary directory:
  - /home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts
  - /home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadPilotShellSnapshot.ts
  - /home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadSd12ReleaseTruth.ts (candidate branch / PR #39 only; not yet on origin/develop)
origin/develop main.rs release-truth terms:
  - none for load_sd12_release_truth / ReleaseTruthSnapshot
candidate-branch bridge fields:
  - loadSd12ReleaseTruth / releaseUnitId / sourceRevision / manifestPath / updateEligibilityState / trustGateStatus / replacementReleaseId / officialSurface / localBuildAuthority all present in the candidate branch and exercised by SD-11 status/evidence tests
local verification commands:
  - `cd apps/desktop && npx --yes tsx src/sd11/status/createSd11WorkbenchStatus.test.ts`
  - `cd apps/desktop && npx --yes tsx src/sd11/loadSd11TesterWorkbenchSurface.test.ts`
  - `cd apps/desktop && npx --yes tsx src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts`
  - `cd apps/desktop && npx --yes tsx src/sd11/feedback/bug/composeBugReport.test.ts`
  - `cd apps/desktop && npx --yes tsx src/sd11/feedback/enhancement/composeEnhancementRequest.test.ts`
  - `cd apps/desktop && npm run typecheck`
  - `cd apps/desktop && npm run build`
  - `cd apps/desktop && npm run tauri:check`
merge attempts from this credential surface:
  - `gh pr merge 39 --squash --delete-branch` => `GraphQL: Resource not accessible by personal access token (mergePullRequest)`
  - `gh api -X PUT repos/electricm0nk/codex/pulls/39/merge ...` => HTTP 403 `Resource not accessible by personal access token`
```

## What accepted upstream truth now does prove
`/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml` is no longer speculative residue. It is merged accepted repo truth on `origin/develop`.

That accepted workflow now proves the repo owns a real Linux tester publication lane for:
- governed `develop`/`main` release publication
- Linux `.deb` and `.AppImage` release assets
- `checksums.sha256`
- `provenance.json`
- `update-manifest-stub.json`
- release notes that explicitly keep update checks manual-only for now

This narrows the blocker. The publication surface exists. The desktop/evidence bridge now exists on PR #39, but not yet as accepted `origin/develop` truth.

## Why no honest accepted executable bridge exists on `origin/develop` today

### 1. No accepted `origin/develop` runtime release-truth surface is named yet
`origin/develop` does not contain the earlier local `load_sd12_release_truth` / `ReleaseTruthSnapshot` residue in `apps/desktop/src-tauri/src/main.rs`, and it still does not contain the accepted replacement runtime/consumer seam. PR #39 deliberately bridges over the already-accepted `sd11_update_action` runtime surface, but that consumer naming is still candidate truth until merged.

### 2. No accepted `origin/develop` TypeScript boundary exists yet
`origin/develop` still lacks `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadSd12ReleaseTruth.ts`. That boundary now exists on PR #39, but it is not accepted repo truth until merged.

### 3. Accepted `origin/develop` runtime wiring still ignores release truth entirely
On `origin/develop`, `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts` still wires only `loadGe08AuthoringWorkbench` and `loadPilotShellSnapshot`, and `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` still calls `createSd11WorkbenchStatus(context)` without any SD-12 release-truth input. PR #39 repairs both files on branch scope.

### 4. Accepted `origin/develop` status truth is still locally hard-coded
`origin/develop` still hard-codes the stale operator promotion path, `not-yet-supported` update posture, and a purely local issue-capture model. PR #39 replaces that branch-local posture with bridge-derived update and issue-capture truth, but that repair is not yet accepted branch truth.

### 5. Accepted `origin/develop` evidence capture still cannot carry release-truth fields
On `origin/develop`, `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/evidenceFields.ts` still limits the shared auto-captured catalog to local SD-11 surface metadata such as:
- `buildLabel`
- `channelSupportLabel`
- `platformLabel`
- `currentWorkflow`
- `dataSourceIdentity`
- diagnostics / refs / attachments

It does not define accepted slots for release-truth fields such as:
- `releaseUnitId`
- `sourceRevision`
- `manifestPath`
- `updateEligibilityState`
- `trustGateStatus`
- `replacementReleaseId`
- `officialSurface`
- `localBuildAuthority`

`/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts` on `origin/develop` therefore still auto-captures only the current SD-11 local surface metadata. PR #39 extends both files with the release-truth backbone, but that remains branch-local until merged.

### 6. Accepted `origin/develop` GitHub issue payload composers still serialize only local SD-11 metadata
On `origin/develop`, both of these files still render the metadata section from the current local `AutoCapturedEvidence` shape:
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.ts`

Their accepted structured issue drafts still serialize build label, tester channel/support label, platform, bounded workflow, and local data-source identity, but not release-unit identity, manifest path, trust-gate state, or replacement/recovery truth from SD-12. PR #39 adds those fields on candidate branch truth.

## Documentary repair performed in this lane
The authoritative SD-12 documentary bundle was repaired so it no longer claims that the publication workflow is merely untracked residue or that `main.rs` still carries the old local release-truth candidate. Updated authority surfaces now reflect the narrower live blocker: accepted publication truth exists, but no accepted runtime/TypeScript/evidence bridge consumes it.

## Decisive conclusion
There is now an honest executable SD-11 ⇄ SD-12 synchronization bridge on candidate branch truth, but there is still no accepted `origin/develop` bridge truth today.

The old blocker story was too broad because the publication workflow is already merged accepted repo truth. The current blocker story is narrower: PR #39 implements the dedicated boundary and the SD-11 consumer bridge, but the available credential surface cannot merge it. The correct move is to keep `SD12-E6-R2` blocked on accepted-branch truth until a merge-capable authority lands PR #39, then re-run the gate and close it immediately if `origin/develop` reflects the candidate bridge unchanged.
