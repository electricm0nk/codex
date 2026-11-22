# SD12-E4-R1 Execution Readiness Closure — Rollback, withdrawal, and downgrade recovery

## Card outcome
- evidence_class: `blocked`
- readiness_verdict: `blocked-on-missing-recovery-state-producer-and-consumer-boundary`
- route truth: this card closes as a documentary readiness artifact only; no PR, branch-ready commit, or merge evidence is expected from this card itself
- next board move if accepted:
  - create `SD12-E4-B1 FLOW: Resolve rollback, withdrawal, and downgrade recovery boundary truth`
  - create `SD12-E4-R2 FLOW: Rollback, withdrawal, and downgrade handoff artifact` gated behind that blocker repair

## Live repo truth grounded on 2026-06-29
- `git branch --show-current` reports `feat/sd11-enhancement-request-composer`
- `git rev-parse HEAD` reports `8f3a627655f490551ff23746293cde1622085e97`
- `git branch --list` shows many local feature branches plus local `develop` and `main`, but no local `uat` branch
- `git branch -r` shows only `origin/develop` and `origin/main`; there is no remote `origin/uat` branch
- `/home/ubuntu/workspace/repos/codex/.github/workflows/` still has no `publish-tester-release.yml` or other grounded publication/manifests workflow for withdrawn, superseded, blocked, or recovery-preferred release state
- repo search for `publish-tester-release|manifestVersion|updateEligibilityState|replacementReleaseId|operatorPromotionPathReference` under `/home/ubuntu/workspace/repos/codex` returned no matches, which proves the SD-12 manifest/recovery contract is still documentary rather than repo-backed executable truth
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json` still exposes only `npm run typecheck`, `npm run build`, and `npm run tauri:check`; it contains no updater, manifest, recovery, rollback, or publication scripts
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json` still sets `bundle.active` to `false`, so packaged updater/recovery outputs are not already enabled silently
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml` still carries only `serde`, `serde_json`, `tauri`, and the repo-local `codex` crate; there is no updater/plugin or recovery-specific runtime dependency
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs` still registers exactly two Tauri commands: `load_pilot_shell_snapshot` and `load_ge08_authoring_workbench_snapshot`; there is no manifest-fetch, update-check, rollback, withdrawal, downgrade, or recovery command
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts` still defines the entire update state surface as `type Sd11UpdateState = 'not-yet-supported'` and hard-codes `Update checks not yet wired in this slice`; there is no governed state model yet for `automatic`, `manual-only`, `unsupported`, `withdrawn`, `blocked`, `superseded`, or recovery-preferred behavior
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts` verifies only the current `not-yet-supported` posture plus Linux/macOS/Windows support asymmetry; it proves no rollback/withdrawal/downgrade path exists yet
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` and `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts` still load only the GE-08 authoring workbench seam with pilot fallback and map the static SD-11 status surface; they do not load manifest truth or recovery metadata
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx` still renders one bounded "Update and support posture" card using the static `status.update` object plus operator provenance; it has no user-visible recovery guidance, downgrade target, withdrawn-build warning, or blocked-build remediation surface
- repo search for `updater|checkUpdate|installUpdate|plugin-updater` under `/home/ubuntu/workspace/repos/codex` found no executable updater surface; the only matches are narrative strings in `loadSd11TesterWorkbenchSurface.ts` that explicitly refuse to claim updater mechanics
- repo search for `rollback|withdrawn|superseded|manual-only` inside the desktop app surfaces found no governed update-recovery implementation surface; the only nearby state words currently in repo truth belong to unrelated GE-08 preview `blocked`/`unsupported` diagnostics rather than release/update recovery behavior
- the upstream E2 closure already proves the promotion topology is blocked on missing `uat` truth, and the upstream E3 closure already proves there is no honest manifest producer or dedicated desktop consumer boundary yet; E4 inherits both gaps directly because rollback/withdrawal/downgrade requires both a producer of recovery state and a consumer/presentation seam

## Actual verification run during this closure
The following commands were run successfully on the live repo during this card:

```bash
git -C /home/ubuntu/workspace/repos/codex branch --show-current
git -C /home/ubuntu/workspace/repos/codex rev-parse HEAD
git -C /home/ubuntu/workspace/repos/codex branch --list
git -C /home/ubuntu/workspace/repos/codex branch -r
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run typecheck && npm run build && npm run tauri:check
```

Observed result:
- current working branch is `feat/sd11-enhancement-request-composer`
- current commit is `8f3a627655f490551ff23746293cde1622085e97`
- local and remote branch inventory still includes `develop` and `main` but no `uat`
- desktop verification still passes:
  - `npm run typecheck` passed
  - `npm run build` passed
  - `npm run tauri:check` passed

This is the decisive fact. The documentary contract defines withdrawn, superseded, blocked, and recovery-preferred behavior, but the live repo exposes neither a truthful release-state producer nor a dedicated desktop recovery-consumer seam. A handoff authored now would have to invent both the state emitter and the state presentation/recovery path.

## Exact required reads for the blocker-repair lane
The blocker-repair successor should require reading exactly these surfaces:
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/epic-breakdown.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/rollback-withdrawal-and-downgrade-policy.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/self-update-transport-and-manifest-contract.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e2-r1-execution-readiness-closure-2026-06-29.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e3-r1-execution-readiness-closure-2026-06-29.md`
- Upstream blocker task `t_d914a841` (`SD12-E2-B1 FLOW: Resolve missing uat publication and promotion truth`)
- Upstream blocker task `t_dc6a389e` (`SD12-E3-B1 FLOW: Resolve manifest producer and desktop updater boundary truth`)
- `/home/ubuntu/workspace/repos/codex/AGENTS.md`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`
- `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml` once that file exists from the repaired E2 publication lane
- any dedicated manifest/update boundary file introduced by the repaired E3 lane under `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/**`

## Exact candidate repo paths and allowed write scope once the blocker is repaired
This lane is narrower than `repos/codex/**`. If the missing recovery producer/consumer boundary truth is repaired, the later handoff should authorize only these exact candidate write surfaces unless a new readiness pass widens scope explicitly.

### Candidate release-state producer / publication bridge surfaces
1. `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml`
   - candidate future publication/manifests bridge for withdrawn, superseded, blocked, and recovery-preferred release metadata
   - not live today; must not be invented as already-real repo truth
   - remains blocked on the upstream E2 publication repair and E3 manifest-boundary repair because recovery state cannot outrun the release topology or manifest surface that carries it

### Candidate desktop recovery-consumer surfaces
2. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/`
   - future dedicated manifest/update/recovery boundary may add a new loader here once E3 has grounded it
   - the existing GE-08 and pilot-shell boundary files are read-only pattern references for this readiness pass and must not be repurposed implicitly into recovery semantics

3. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
   - candidate status-model surface that will need to widen beyond `not-yet-supported` to classify governed recovery states such as `manual-only`, `unsupported`, `withdrawn`, `blocked`, `superseded`, or recovery-preferred behavior
   - any change here must preserve SD-11 authority over tester-facing wording and keep operator provenance secondary to channel/support language

4. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`
   - required verification companion for any recovery-state model change
   - must remain the place that proves Linux/macOS/Windows asymmetry and recovery/update wording stay honest

5. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
   - candidate composition surface that may consume grounded manifest/recovery truth and map it into the bounded workbench model
   - should not absorb publication logic, branch repair logic, or provenance-policy authorship

6. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts`
   - candidate runtime wiring surface for any newly introduced manifest/update/recovery boundary loader

7. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
   - candidate UI presentation surface for withdrawn-build warnings, superseded/replacement guidance, blocked-build explanation, manual-only recovery posture, and downgrade/rollback instructions
   - must preserve bounded truth instead of implying silent recovery success

8. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`
   - candidate Tauri command registration surface only if the accepted recovery path requires a dedicated new read-only manifest/update/recovery command
   - should not become hidden release automation or a substitute for the publication bridge

9. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml`
   - read-only unless a later handoff explicitly proves a new dependency is needed for a dedicated manifest/update/recovery read path

10. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json`
   - read-only unless a later handoff explicitly selects a Tauri-native updater/config path rather than a custom bounded recovery-consumer seam

### Explicit scope boundary
If the later handoff discovers that rollback, withdrawal, downgrade, or recovery guidance cannot be implemented inside the surfaces above, it must stop and route back through a new readiness closure before touching broader surfaces such as:
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/**` beyond the exact files named above
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/**` beyond the exact files named above
- `/home/ubuntu/workspace/repos/codex/src/**`
- publication-topology repair surfaces that belong to SD12-E2 beyond the explicit workflow bridge
- manifest producer/consumer boundary work that belongs to SD12-E3 beyond the explicit repaired surfaces
- provenance/integrity enforcement surfaces that belong to SD12-E5
- feedback-intake or unrelated SD-11 workbench flows

## Exact non-goals for the later rollback/recovery lane
The next handoff must state these non-goals plainly:
- no silent invention of a `uat` publication stage, manifest emitter, or desktop updater boundary that does not exist in live repo truth
- no treating GE-08 preview `blocked`/`unsupported` diagnostics as though they already satisfy update recovery states
- no activation of `bundle.active` or broader packaging claims merely because rollback/recovery doctrine exists
- no raw `develop`, `uat`, or `main` branch names as the primary tester-facing recovery UX
- no widening into publication-topology repair, initial manifest-producer implementation, or provenance/integrity enforcement beyond what is minimally required to preserve truthful recovery state labels and guidance
- no counterfeit silent-success claims for rollback, downgrade, or recovery
- no PR, merge, or branch-ready evidence from this documentary readiness card itself

## Exact verification surfaces for the blocker-repair lane and later handoff
Required branch/workflow verification surfaces:

```bash
git -C /home/ubuntu/workspace/repos/codex branch --list
git -C /home/ubuntu/workspace/repos/codex branch -r
```

Required desktop verification commands that must remain available for the eventual handoff:

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run typecheck
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run build
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run tauri:check
```

Required file surfaces to inspect directly:
- `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml` once it exists
- the dedicated manifest/update/recovery boundary file under `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/**` once it exists
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`

Verification interpretation:
- the git branch inventory proves whether the accepted `develop -> uat -> main` publication backing surface is real instead of documentary wishfulness
- the workflow surface proves whether withdrawn/superseded/replacement state has an honest publication/manifests bridge instead of release folklore
- the boundary files prove whether a dedicated manifest/update/recovery consumer seam exists instead of overloading unrelated GE-08 or pilot-shell commands
- the status/composition/UI files prove whether testers will actually see governed recovery truth rather than a static `not-yet-supported` placeholder
- the desktop verification commands protect the existing SD-11 proof surface from collateral drift while the recovery lane is introduced

## Readiness verdict
This lane is not yet ready for a handoff artifact.

Why it is blocked:
- the accepted SD-12 rollback/withdrawal/downgrade policy requires state transitions such as `active`, `superseded`, `withdrawn`, and `blocked`, plus explicit recovery-preferred targets, but the live repo still has no grounded publication/manifests bridge that could emit those states
- the accepted SD-12 and SD-11 desktop contract requires user-visible recovery behavior, but the live repo still exposes only a static `not-yet-supported` update status and no dedicated recovery-consumer seam
- the upstream E2 lane is already blocked on missing `uat` publication truth, so any recovery state model named now would inherit counterfeit promotion topology
- the upstream E3 lane is already blocked on missing manifest producer/consumer truth, so any rollback/recovery handoff authored now would have to invent both the state carrier and the desktop consumer boundary
- authoring `SD12-E4-R2` now would either invent nonexistent repo truth or overload unrelated GE-08 / pilot-shell seams and static SD-11 status surfaces with recovery semantics they do not currently own

Why the closure still matters:
- it narrows the future write surface to one publication bridge plus a small set of desktop status/composition/UI files instead of letting rollback/recovery sprawl across the repo
- it makes the hidden blocker explicit: rollback, withdrawal, downgrade, and recovery are planning-real, but the release-state producer and user-visible recovery-consumer seams are not yet executable repo truth
- it preserves SD-11 authority over tester-facing wording while reserving rollback/recovery behavior for a dedicated bounded lane once the upstream publication and manifest boundaries are honest

## Successor truth
The real blocker is missing executable recovery producer/consumer boundary truth. The next truthful board move is therefore a blocker-repair pair:
- `SD12-E4-B1 FLOW: Resolve rollback, withdrawal, and downgrade recovery boundary truth`
  - unblock condition: the repo exposes an accepted publication/manifests bridge for withdrawn/superseded/recovery state and an accepted dedicated desktop recovery-consumer boundary, or the documentary contract is revised explicitly so SD-12 no longer claims this executable lane yet
  - prerequisite truth: the upstream E2 publication repair and E3 manifest/updater boundary repair must land first, because the first honest recovery lane cannot outrun either the promotion topology or the manifest producer/consumer seam that defines it
- `SD12-E4-R2 FLOW: Rollback, withdrawal, and downgrade handoff artifact`
  - this card should remain gated behind `SD12-E4-B1` and should only author the stage-specific handoff after the recovery boundary is truthful
