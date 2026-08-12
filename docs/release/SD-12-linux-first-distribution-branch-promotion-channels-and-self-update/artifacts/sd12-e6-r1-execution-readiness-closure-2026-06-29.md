# SD12-E6-R1 Execution Readiness Closure — SD-11 and SD-12 truth synchronization

## Card outcome
- evidence_class: `blocked`
- readiness_verdict: `blocked-on-missing-control-plane-backed-status-and-issue-payload-sync-boundary`
- route truth: this card closes as a documentary readiness artifact only; no PR, branch-ready commit, or merge evidence is expected from this card itself
- next board move if accepted:
  - create `SD12-E6-B1 FLOW: Resolve SD-11 status-surface and issue-payload control-plane truth coupling`
  - create `SD12-E6-R2 FLOW: SD-11 and SD-12 truth-synchronization handoff artifact` gated behind that blocker repair

## Live repo truth grounded on 2026-06-29
- `git branch --show-current` reports `feat/sd11-enhancement-request-composer`
- `git rev-parse HEAD` reports `8f3a627655f490551ff23746293cde1622085e97`
- `git branch --list` shows local `develop` and `main`, but no local `uat` branch
- `git branch -r` shows `origin/develop` and `origin/main`, but no remote `origin/uat` branch
- `/home/ubuntu/workspace/repos/codex/.github/workflows/` currently contains exactly one workflow file: `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml`
- the authoritative SD-11 and SD-12 documentary surfaces now preserve live operator truth as `develop -> main` and reserve `beta` until a governed candidate backing surface exists:
  - `/home/ubuntu/workspace/programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md`
  - `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/distribution-platform-support-and-channel-matrix.md`
  - `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md`
- the live desktop status source still hard-codes stale operator promotion truth as `develop -> uat -> main` inside `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`, and repo search shows the same stale string persists in the directly coupled SD-11 status/evidence tests:
  - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`
  - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts`
  - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts`
  - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.test.ts`
  - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/submitBugReport.test.ts`
  - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.test.ts`
  - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/enhancement/submitEnhancementRequest.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` builds the SD-11 surface by calling `createSd11WorkbenchStatus(context)` and then layering GE-08 preview or pilot fallback data on top; it does not read any accepted SD-12 publication, manifest, release-unit, or trust-gate boundary
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx` renders the update/support posture and issue-payload status directly from `surface.status.*`, including:
  - `surface.status.channel.testerFacingLabel`
  - `surface.status.support.tierMatrixLabel`
  - `surface.status.update.label`
  - `surface.status.issueCapture.testerFacingChannelSupportLabel`
  - `surface.status.channel.operatorPromotionPath`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts` auto-captures build label, channel/support label, platform, workflow, diagnostics, explanation references, and provenance references from the current SD-11 workbench surface only; it does not capture SD-12 manifest fields such as `releaseId`, `updateEligibilityState`, `replacementReleaseId`, or a release-control-plane provenance handle
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/evidenceFields.ts` defines the shared feedback backbone and currently exposes no field for manifest-backed channel state, release identity, update eligibility, or official publication provenance
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.ts` and `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.ts` render the GitHub draft metadata from the current surface-local evidence backbone; they do not yet couple issue payloads to SD-12 release/control-plane truth
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json` still exposes only `npm run typecheck`, `npm run build`, and `npm run tauri:check`; there is no release-truth sync, manifest-reader, or provenance-coupling script surface
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json` still sets `bundle.active` to `false`, proving the desktop runtime is not already backed by an accepted packaging/updater configuration surface
- `SD12-E3-R1` already proved there is no governed manifest producer or dedicated desktop updater/manifest-consumer boundary yet
- `SD12-E5-R1` already proved there is no governed release-unit identity, checksum/provenance emission, or Linux trust-gate boundary yet

## Actual verification run during this closure
The following commands were run successfully on the live repo during this card:

```bash
git -C /home/ubuntu/workspace/repos/codex branch --show-current
git -C /home/ubuntu/workspace/repos/codex rev-parse HEAD
git -C /home/ubuntu/workspace/repos/codex branch --list
git -C /home/ubuntu/workspace/repos/codex branch -r
find /home/ubuntu/workspace/repos/codex/.github/workflows -maxdepth 1 -type f | sort
git -C /home/ubuntu/workspace/repos/codex status --short --ignored -- apps/desktop/src-tauri/target
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run typecheck && npm run build && npm run tauri:check
```

Observed result:
- current working branch is `feat/sd11-enhancement-request-composer`
- current commit is `8f3a627655f490551ff23746293cde1622085e97`
- local and remote branch inventory includes `develop` and `main` but no `uat`
- `.github/workflows/` still contains only `allow-only-develop-into-main.yml`
- ignored local build residue remains under `apps/desktop/src-tauri/target/` and must not be mistaken for accepted release truth
- desktop verification still passes:
  - `npm run typecheck` passed
  - `npm run build` passed
  - `npm run tauri:check` passed

This is the decisive fact. The documentary contract already says the SD-11 surfaces must synchronize against SD-12 control-plane truth, but the live repo still exposes no accepted control-plane source for manifest state or release provenance, while the existing SD-11 UI and issue-payload backbone continue to derive their truth from a local hard-coded status model plus GE-08 preview evidence. A handoff authored now would couple the UI and issue flows to assumptions, not to real control-plane truth.

## Exact required reads for the blocker-repair lane
The blocker-repair successor should require reading exactly these surfaces:
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/epic-breakdown.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/distribution-platform-support-and-channel-matrix.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/self-update-transport-and-manifest-contract.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/provenance-integrity-and-update-eligibility.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e3-r1-execution-readiness-closure-2026-06-29.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e5-r1-execution-readiness-closure-2026-06-29.md`
- Upstream blocker task `t_dc6a389e` (`SD12-E3-B1 FLOW: Resolve manifest producer and desktop updater boundary truth`)
- Upstream blocker task `t_b3cd10b7` (`SD12-E5-B1 FLOW: Resolve provenance emission and platform trust-gate execution truth`)
- `/home/ubuntu/workspace/programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md`
- `/home/ubuntu/workspace/repos/codex/AGENTS.md`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/evidenceFields.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.test.ts`
- `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml`
- `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml` once that file exists from the upstream publication/provenance repair chain
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs` only if the upstream repairs prove the accepted SD-12 truth enters the desktop through a dedicated Tauri command boundary
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml` only if the upstream repairs prove that dedicated command boundary requires a Rust dependency surface
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/capabilities/default.json` only if the upstream repairs prove that dedicated command boundary requires an explicit capability surface

## Exact candidate repo paths and allowed write scope once the blocker is repaired
This lane must stay narrower than `repos/codex/**`. Once the upstream E3 and E5 blocker repairs ground a real control-plane truth source, the later E6 handoff should authorize only these exact desktop status/payload coupling surfaces unless a new readiness pass widens scope explicitly.

### Primary status-surface write paths
1. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
   - primary status-source surface that currently hard-codes operator promotion truth and update posture
   - must become a consumer of accepted SD-12 truth rather than a local doctrine fork

2. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`
   - required verification companion for any status-source change
   - must prove the live `develop -> main` mapping, reserved `beta` posture, Linux/macOS/Windows asymmetry, and update-state wording remain honest

3. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
   - primary composition surface that should map accepted SD-12 control-plane truth into the bounded SD-11 model
   - must not absorb publication logic, manifest authorship, or provenance-policy ownership

4. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts`
   - runtime wiring surface for whichever accepted release-truth bridge the blocker-repair lane grounds

5. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts`
   - required composition-level verification companion proving the workbench surface reflects the accepted synchronized truth rather than stale constants

6. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
   - bounded presentation surface for update/support posture, issue-payload status, and visible operator provenance
   - must continue to keep tester-facing language primary and branch lineage secondary

### Primary issue-payload coupling write paths
7. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts`
   - shared evidence backbone that currently auto-captures only the local workbench surface fields
   - must be the first place where accepted SD-12 release/control-plane handles enter both bug and enhancement flows without schema drift

8. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts`
   - required verification companion proving the shared evidence payload carries the synchronized truth and does not silently regress

9. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/evidenceFields.ts`
   - candidate field-catalog surface if the accepted control-plane truth requires explicit new evidence fields rather than reuse of the current local-only backbone
   - should change only if the blocker-repair lane proves a new field is necessary

10. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.ts`
    - candidate bug-payload rendering surface if synchronized build/channel/provenance/update fields must become visible in the governed GitHub draft

11. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.test.ts`
    - required verification companion for any bug-payload metadata change

12. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.ts`
    - candidate enhancement-payload rendering surface if synchronized build/channel/provenance/update fields must become visible in the governed GitHub draft

13. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.test.ts`
    - required verification companion for any enhancement-payload metadata change

### Bridge surfaces that remain conditional until upstream truth exists
14. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/`
    - a future dedicated release-truth bridge file may be added here if the accepted SD-12 source of truth enters the UI through a TypeScript boundary loader
    - the existing `loadGe08AuthoringWorkbench.ts` and `loadPilotShellSnapshot.ts` are read-only pattern references for this readiness pass; they must not be repurposed to smuggle release/control-plane semantics into unrelated commands

15. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`
16. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml`
17. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/capabilities/default.json`
    - these remain conditional only if the upstream blocker repairs prove the accepted SD-12 truth must be exposed through a new read-only Tauri command seam
    - they are not part of the first expected write scope by default

### Explicit scope boundary
If the later handoff discovers that status/payload synchronization cannot be implemented inside the surfaces above, it must stop and route back through a new readiness closure before touching broader surfaces such as:
- `/home/ubuntu/workspace/repos/codex/.github/workflows/**` beyond read-only comparison with the already accepted upstream publication/provenance repairs
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/**` beyond the exact files named above
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/**` beyond the conditional files named above
- `/home/ubuntu/workspace/repos/codex/src/**`
- manifest producer/consumer implementation ownership that belongs to SD12-E3
- provenance/build-receipt/trust-gate ownership that belongs to SD12-E5
- rollback/withdrawal execution surfaces that belong to SD12-E4
- unrelated SD-11 bug/enhancement transport work beyond the evidence/payload metadata surfaces above

## Exact non-goals for the later synchronization lane
The next handoff must state these non-goals plainly:
- no invention of publication, manifest, provenance, or trust-gate truth inside the SD-11 UI/payload layer
- no persistence of the stale `develop -> uat -> main` doctrine once the synchronized control-plane source is accepted
- no raw `develop` or `main` branch names as the primary tester-facing update UX
- no broadening into GitHub submission transport, attachment transport, or unrelated SD-11 issue-flow UX behavior
- no repurposing of GE-08 authoring provenance as though it were SD-12 release/build provenance
- no manifest-schema authorship, publication-workflow authorship, or checksum/provenance emission logic inside the E6 synchronization lane
- no widening into rollback, withdrawal, downgrade, or platform trust-threshold behavior beyond whatever synchronized labels the accepted upstream surfaces already expose
- no PR, merge, or branch-ready evidence from this documentary readiness card itself

## Exact verification surfaces for the blocker-repair lane and later handoff
Required repo-truth verification surfaces:

```bash
git -C /home/ubuntu/workspace/repos/codex branch --list
git -C /home/ubuntu/workspace/repos/codex branch -r
find /home/ubuntu/workspace/repos/codex/.github/workflows -maxdepth 1 -type f | sort
git -C /home/ubuntu/workspace/repos/codex status --short --ignored -- apps/desktop/src-tauri/target
```

Required desktop verification commands that must remain available for the eventual handoff:

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run typecheck
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run build
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run tauri:check
```

Required file-surfaces to inspect directly:
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/evidenceFields.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.ts`
- `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml` once it exists
- the accepted artifact outputs from `SD12-E3-B1` and `SD12-E5-B1` once they exist

Verification interpretation:
- the branch and workflow inventory prove whether the operator/publication truth source remains `develop -> main` only or has gained an explicitly governed expansion
- the desktop files prove whether the visible status surface still forks doctrine locally or has been coupled to accepted SD-12 truth
- the evidence/payload files prove whether GitHub issue drafts still capture only surface-local metadata or now carry the accepted release/control-plane handles
- the ignored-target check prevents counterfeit completion by confusing local bundle residue with official release truth
- the desktop verification commands protect the existing SD-11 proof surface from collateral drift while synchronization work lands

## Readiness verdict
This lane is not yet ready for a handoff artifact.

Why it is blocked:
- the accepted SD-12 contract says SD-11 must synchronize tester-facing status and issue-payload truth against the SD-12 control plane, but the upstream E3 and E5 lanes still have not grounded the manifest, release-identity, provenance, and trust-gate source surfaces that synchronization would consume
- the live SD-11 status source still forks doctrine locally by hard-coding stale `develop -> uat -> main` promotion truth even after the documentary repair standardized `develop -> main`
- the current issue-payload backbone captures workbench-local build/workflow/provenance context, but not SD-12 release-unit identity, manifest eligibility state, or official provenance/build-receipt handles
- authoring `SD12-E6-R2` now would either invent a control-plane source that does not exist, or merely spread stale/local status strings across more files without resolving the authority boundary

Why the closure still matters:
- it narrows the later E6 write surface to the exact SD-11 status, composition, evidence, and issue-draft files that should consume accepted SD-12 truth, instead of allowing the sync lane to sprawl into publication, manifest, provenance, or rollback ownership
- it identifies the hidden architectural split clearly: E6 is not the place to invent release truth; it is the place to consume already-governed truth once upstream surfaces exist
- it surfaces an immediately visible truth-drift defect in the current repo: documentary operator mapping is already `develop -> main`, while the desktop status and related tests still carry the stale three-stage path

## Successor truth
The real blocker is not a missing UI string. It is missing accepted control-plane truth for the UI and issue payloads to consume, plus a still-local status model that forks doctrine.

The next truthful board move is therefore a blocker-repair pair:
- `SD12-E6-B1 FLOW: Resolve SD-11 status-surface and issue-payload control-plane truth coupling`
  - unblock condition: the repo exposes accepted upstream SD-12 manifest/provenance/trust surfaces and an exact bounded desktop/evidence bridge for consuming them, or the documentary contract is revised explicitly so SD-12 no longer claims this executable synchronization lane yet
  - prerequisite truth: the upstream `SD12-E3-B1` and `SD12-E5-B1` repairs must land first, because E6 cannot couple to release/control-plane truth that does not exist yet
- `SD12-E6-R2 FLOW: SD-11 and SD-12 truth-synchronization handoff artifact`
  - this card should remain gated behind `SD12-E6-B1` and should only author the stage-specific handoff after the synchronized source/bridge boundary is truthful
