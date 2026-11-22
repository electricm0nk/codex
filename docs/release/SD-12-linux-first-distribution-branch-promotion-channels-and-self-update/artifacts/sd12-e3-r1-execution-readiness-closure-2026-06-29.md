# SD12-E3-R1 Execution Readiness Closure — Update manifest and desktop-consumer contract

## Card outcome
- evidence_class: `blocked`
- readiness_verdict: `blocked-on-missing-manifest-producer-and-updater-boundary`
- route truth: this card closes as a documentary readiness artifact only; no PR, branch-ready commit, or merge evidence is expected from this card itself
- next board move if accepted:
  - create `SD12-E3-B1 FLOW: Resolve manifest producer and desktop updater boundary truth`
  - create `SD12-E3-R2 FLOW: Update manifest and desktop-consumer handoff artifact` gated behind that blocker repair

## Live repo truth grounded on 2026-06-29
- `git branch --show-current` reports `feat/sd11-enhancement-request-composer`
- `git rev-parse HEAD` reports `8f3a627655f490551ff23746293cde1622085e97`
- `git branch --list` shows local `develop` and `main`, but no local `uat` branch
- `git branch -r` shows `origin/develop` and `origin/main`, but no remote `origin/uat` branch
- `/home/ubuntu/workspace/repos/codex/.github/workflows/` currently contains exactly one workflow file: `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml`
- the accepted publication doctrine still says `develop -> uat -> main`, but the live repo exposes no workflow or branch-backed publication surface that could emit an official update manifest today
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json` exposes only `npm run typecheck`, `npm run build`, and `npm run tauri:check`; it contains no updater, release, or manifest-generation scripts
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json` still sets `bundle.active` to `false`, so packaging/update outputs are not already enabled silently
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml` carries only the bounded Tauri shell dependencies (`serde`, `serde_json`, `tauri`, and the repo-local `codex` crate); there is no updater/plugin or manifest-specific runtime dependency
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/` currently contains only `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts` and `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadPilotShellSnapshot.ts`
- both boundary files are thin `@tauri-apps/api/core` invoke wrappers around existing GE-08 / pilot-shell commands, not updater or manifest-consumer surfaces
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs` currently registers exactly two Tauri commands: `load_pilot_shell_snapshot` and `load_ge08_authoring_workbench_snapshot`; there is no manifest-fetch, manifest-parse, update-check, or update-apply command
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts` hard-codes the current updater truth as `state: 'not-yet-supported'` with label `Update checks not yet wired in this slice`, while preserving `develop -> uat -> main` only as operator provenance
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` and `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx` only render that bounded status truth; they do not fetch or consume update-manifest data
- repo search under `/home/ubuntu/workspace/repos/codex/apps/desktop/` found no `plugin-updater`, `checkUpdate`, `installUpdate`, or similar updater-consumer surface

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
- local and remote branch inventory includes `develop` and `main` but no `uat`
- desktop verification still passes:
  - `npm run typecheck` passed
  - `npm run build` passed
  - `npm run tauri:check` passed

This is the decisive fact. The documentary contract defines a manifest producer and a desktop consumer, but the live repo exposes neither a real publication/manifest emitter surface nor a dedicated updater boundary in the desktop runtime. A handoff authored now would have to invent both ends of the contract.

## Exact required reads for the blocker-repair lane
The blocker-repair successor should require reading exactly these surfaces:
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/epic-breakdown.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/self-update-transport-and-manifest-contract.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/provenance-integrity-and-update-eligibility.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e2-r1-execution-readiness-closure-2026-06-29.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md`
- `/home/ubuntu/workspace/repos/codex/AGENTS.md`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadPilotShellSnapshot.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`
- `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml`
- `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml` once that file exists from the repaired E2 publication lane

## Exact candidate repo paths and allowed write scope once the blocker is repaired
This lane is narrower than `repos/codex/**`. If the missing producer/consumer boundary truth is repaired, the later handoff should authorize only these exact candidate write surfaces unless a new readiness pass widens scope explicitly.

### Candidate manifest-producer / bridge surfaces
1. `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml`
   - candidate future manifest-emission surface for the first official tester release lane
   - not live today; must not be invented as already-real repo truth
   - remains blocked on the upstream E2 publication/promotion repair because a governed manifest producer cannot exist honestly before the publication topology is truthful

2. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/`
   - future manifest-consumer entry may add a new dedicated boundary file here (for example a manifest loader or update-status fetch boundary)
   - the existing `loadGe08AuthoringWorkbench.ts` and `loadPilotShellSnapshot.ts` are read-only pattern references for this readiness pass; they should not be repurposed to smuggle updater semantics into unrelated commands

### Candidate desktop-consumer write paths
3. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
   - candidate status-model surface that will need to stop hard-coding `not-yet-supported` once an accepted manifest/update boundary exists
   - any change here must preserve SD-11 authority over tester-facing wording and keep operator provenance secondary to channel language

4. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`
   - required verification companion for any status-surface change
   - must remain the place that proves Linux/macOS/Windows asymmetry and channel/provenance wording stay honest

5. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
   - candidate composition surface that may consume manifest/update truth and map it into the bounded workbench model
   - should not absorb publication logic, branch repair logic, or provenance-policy authorship

6. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts`
   - candidate runtime wiring surface for any newly introduced manifest boundary loader

7. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
   - candidate UI presentation surface for update status, manual-only posture, blocked/withdrawn state, and support-tier visibility
   - must preserve the existing bounded truth posture instead of claiming silent updater success

8. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`
   - candidate Tauri command registration surface if the accepted consumer path requires a new read-only manifest/update command
   - should not become a hidden release-automation or publication surface

9. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml`
   - read-only unless a later handoff explicitly proves a new dependency is needed for a dedicated manifest/update read path

10. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json`
   - read-only unless a later handoff explicitly selects a Tauri-native updater/config path rather than a custom bounded manifest-consumer seam

### Explicit scope boundary
If the later handoff discovers that manifest production or consumption cannot be implemented inside the surfaces above, it must stop and route back through a new readiness closure before touching broader surfaces such as:
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/**` beyond the exact files named above
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/**` beyond the exact files named above
- `/home/ubuntu/workspace/repos/codex/src/**`
- rollback/withdrawal execution surfaces that belong to SD12-E4
- provenance/integrity enforcement surfaces that belong to SD12-E5
- feedback intake surfaces that belong to SD-11 bug/enhancement lanes

## Exact non-goals for the later manifest/consumer lane
The next handoff must state these non-goals plainly:
- no silent invention of a publication workflow, `uat` branch, or tester release topology that does not exist in live repo truth
- no repurposing of GE-08 authoring or pilot-shell boundary commands as pseudo-updater surfaces without a dedicated new manifest/update boundary
- no activation of `bundle.active` or broader packaging claims merely because manifest fields exist in doctrine
- no raw `develop`, `uat`, or `main` branch names as the primary tester-facing update UX
- no rollback, downgrade, withdrawal, or provenance-policy implementation beyond what is minimally required to preserve accepted manifest fields and truthful state labels
- no widening into GitHub feedback intake, attachment transport, or unrelated SD-11 workbench flows
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

Required file-surfaces to inspect directly:
- `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml` once it exists
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/` for a dedicated manifest/update boundary file
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`

Verification interpretation:
- the git branch inventory proves whether the accepted `develop -> uat -> main` publication backing surface is real instead of documentary wishfulness
- the workflow surface proves whether a governed manifest producer exists instead of branch folklore
- the boundary files prove whether a dedicated manifest/update consumer seam exists instead of overloaded GE-08 or pilot-shell commands
- the desktop verification commands protect the existing SD-11 proof surface from collateral drift while the update lane is introduced

## Readiness verdict
This lane is not yet ready for a handoff artifact.

Why it is blocked:
- the accepted SD-12 contract requires a machine-readable manifest producer, but the live repo still has no publication workflow or other grounded manifest-emission surface
- the accepted SD-11/SD-12 desktop contract requires a consumer that can classify update truth, but the live repo still exposes only hard-coded `not-yet-supported` status plus two unrelated Tauri command seams
- the upstream E2 lane is already blocked on missing `uat` publication truth, so any manifest producer named now would inherit counterfeit publication topology
- authoring `SD12-E3-R2` now would either invent nonexistent repo truth or overload unrelated GE-08 / pilot-shell boundaries with updater meaning they do not currently own

Why the closure still matters:
- it narrows the future write surface to a small set of desktop status/composition/boundary files plus the eventual publication workflow, instead of allowing updater work to sprawl across the repo
- it makes the hidden blocker explicit: the manifest contract is planning-real, but the producer and consumer seams are not yet executable repo truth
- it preserves SD-11 authority over tester-facing wording while reserving the update-control-plane machinery for dedicated boundaries

## Successor truth
The real blocker is missing executable producer/consumer boundary truth. The next truthful board move is therefore a blocker-repair pair:
- `SD12-E3-B1 FLOW: Resolve manifest producer and desktop updater boundary truth`
  - unblock condition: the repo exposes an accepted manifest-emission surface and an accepted dedicated desktop consumer boundary, or the documentary contract is revised explicitly so SD-12 no longer claims this executable lane yet
  - prerequisite truth: the upstream E2 publication repair must land first, because the first honest manifest producer cannot outrun the publication topology that defines it
- `SD12-E3-R2 FLOW: Update manifest and desktop-consumer handoff artifact`
  - this card should remain gated behind `SD12-E3-B1` and should only author the stage-specific handoff after the producer/consumer boundaries are truthful
