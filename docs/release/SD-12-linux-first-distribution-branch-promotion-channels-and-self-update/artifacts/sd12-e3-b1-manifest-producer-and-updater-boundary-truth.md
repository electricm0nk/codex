# SD12-E3-B1 Manifest Producer and Updater Boundary Truth Repair

## Card outcome
- evidence_class: `repaired-by-documentary-revision`
- repair_path: `revised the SD-12 authority surfaces so the manifest/update lane stops implying executable repo boundaries that do not exist yet`
- live manifest producer truth after repair: `none yet`
- live desktop consumer truth after repair: `none yet`
- current desktop update truth that remains real:
  - SD-11 still exposes a hard-coded `not-yet-supported` update state
  - existing Tauri seams are limited to GE-08 authoring and pilot-shell snapshot loading
- SD12-E3-R2 verdict: `blocked`

## Live repo truth grounded on 2026-06-29
- `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml` is still the only live workflow file under `.github/workflows/`
- no `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml` file exists yet
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/` contains only:
  - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts`
  - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadPilotShellSnapshot.ts`
- both existing boundary files are bounded GE-08 / pilot-shell invoke wrappers, not manifest or updater boundaries
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs` registers exactly two commands:
  - `load_pilot_shell_snapshot`
  - `load_ge08_authoring_workbench_snapshot`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts` still hard-codes:
  - `state: 'not-yet-supported'`
  - `label: 'Update checks not yet wired in this slice'`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` and `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx` only consume and present that bounded SD-11 status truth
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml` carries no updater/plugin dependency and `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json` still sets `bundle.active` to `false`

## Decisive judgment
The problem was not that the manifest contract lacked detail. The problem was that the documentary contract still left room for readers to infer a live executable producer/consumer lane where the repo has none.

I reviewed the two honest repair paths:
1. name a real manifest producer surface and a real dedicated desktop consumer boundary
2. revise the documentary contract so it explicitly stops claiming those executable seams exist yet

The first path is not available on live evidence. There is no publication workflow that emits manifest data, no dedicated boundary file under `apps/desktop/src/boundary/`, and no Tauri command or runtime seam that owns manifest fetch/check/apply behavior. Naming one now would counterfeit repo truth.

The second path is the truthful one. The SD-12 authority surfaces now state explicitly that the manifest/update lane remains documentary-only until a live manifest producer and a dedicated desktop consumer boundary are grounded in repo truth.

## Authoritative surfaces after repair
### Live repo authority
- `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadPilotShellSnapshot.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json`

### Documentary authority surfaces revised in this repair
- `/home/ubuntu/workspace/programs/codex/plans/spec-domains/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/technical-requirements.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/technical-design.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/acceptance-and-verification.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/epic-breakdown.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/self-update-transport-and-manifest-contract.md`

## Exact producer and consumer candidate paths
These paths are the only honest candidates visible today. They are not proof of an implementation lane by themselves.

### Candidate future manifest-producer surface
1. `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml`
   - future candidate workflow path for publication plus manifest emission
   - not live today
   - cannot be treated as current repo truth until the file exists and owns manifest output explicitly

### Candidate future dedicated desktop consumer surfaces
2. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/`
   - future home for a new dedicated manifest/update boundary file
   - existing files in this directory are read-only pattern references only
3. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
   - candidate composition surface that may consume dedicated manifest/update truth later
   - not itself the dedicated boundary
4. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts`
   - candidate runtime wiring surface for a later dedicated boundary loader
5. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
   - current truthful status surface
   - must stop hard-coding `not-yet-supported` only after the dedicated consumer boundary exists
6. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`
   - verification companion for any future status-surface change
7. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
   - candidate presentation surface for user-visible update state once dedicated consumer truth exists
8. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`
   - candidate Tauri registration surface if the dedicated consumer path needs a new read-only command
   - not a release or publication surface

## Exact allowed write scope for a future E3 execution handoff
If E3 is ever unblocked for implementation, the first honest write scope should stay narrow:
- `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/**` for a new dedicated manifest/update boundary only
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`

If future work cannot stay inside those paths, it must stop and run another readiness/truth repair instead of widening scope by assumption.

## Exact non-goals preserved by this repair
- do not treat the documentary manifest contract as proof that a live producer/consumer seam exists
- do not repurpose `loadGe08AuthoringWorkbench.ts` or `loadPilotShellSnapshot.ts` as pseudo-updater boundaries
- do not treat hard-coded SD-11 status copy as a dedicated manifest/update consumer
- do not activate packaging or updater claims merely because manifest fields are documented
- do not widen into rollback, provenance, or broader SD-11 issue-flow work under the cover of E3
- do not author an E3 execution handoff while the repo still lacks the exact producer and consumer seams named above

## Why SD12-E3-R2 stays blocked
`SD12-E3-R2 FLOW: Update manifest and desktop-consumer handoff artifact` remains blocked because the live repo still cannot name both ends of the executable lane honestly.

The exact unblock condition is:
- a real manifest-emission surface exists and is named exactly in repo/workflow truth
- a real dedicated desktop consumer boundary exists and is named exactly in repo/runtime truth
- both surfaces have a bounded allowed write scope and verification surface

Until those three statements are true, an E3 handoff artifact would still have to invent implementation authority.

## Required verification surfaces for downstream use
```bash
git -C /home/ubuntu/workspace/repos/codex branch --list
git -C /home/ubuntu/workspace/repos/codex branch -r
```

```text
/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml
/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts
/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadPilotShellSnapshot.ts
/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts
/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts
/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs
```

## Explicit refusals preserved by this repair
- do not let a detailed manifest schema be mistaken for a live implementation surface
- do not let existing GE-08 / pilot-shell seams absorb updater meaning they do not own
- do not let downstream handoff work outrun repo truth just because higher-order SD-12 doctrine is now explicit
