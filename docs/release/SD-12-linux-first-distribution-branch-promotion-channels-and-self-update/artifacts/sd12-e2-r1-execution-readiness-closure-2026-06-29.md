# SD12-E2-R1 Execution Readiness Closure — GitHub publication and promotion control plane

## Card outcome
- evidence_class: `blocked`
- readiness_verdict: `blocked-on-missing-uat-promotion-surface`
- route truth: this card closes as a documentary readiness artifact only; no PR, branch-ready commit, or merge evidence is expected from this card itself
- next board move if accepted:
  - create `SD12-E2-B1 FLOW: Resolve missing uat publication and promotion truth`
  - create `SD12-E2-R2 FLOW: GitHub publication and promotion handoff artifact` gated behind that blocker repair

## Live repo truth grounded on 2026-06-29
- `git branch --show-current` reports `feat/sd11-enhancement-request-composer`
- `git rev-parse HEAD` reports `8f3a627655f490551ff23746293cde1622085e97`
- `git branch --list` shows local `develop` and `main`, but no local `uat` branch
- `git branch -r` shows `origin/develop` and `origin/main`, but no remote `origin/uat` branch
- `/home/ubuntu/workspace/repos/codex/.github/` currently contains exactly one workflow file: `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml`
- that workflow proves only one live promotion-control rule today: pull requests into `main` must come from `develop`; it does not express a `uat` promotion stage and it does not publish release assets, checksums, provenance receipts, or manifest metadata
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json` exposes only `npm run typecheck`, `npm run build`, and `npm run tauri:check`; it contains no release, publish, or updater scripts
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json` still sets `bundle.active` to `false`, which proves packaging/publication outputs are not already enabled silently
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml` remains a bounded Tauri shell surface at version `0.0.0`, not a release-automation authority surface
- `/home/ubuntu/workspace/repos/codex/README.md` still states that Linux onboarding/build proof is real while release packaging remains unfinished
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`, `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`, and `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` still encode the tester-facing truth boundary: `alpha` over `develop -> uat -> main`, Linux first-class / macOS second-class / Windows third-class, and `Update checks not yet wired in this slice`

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

This is the decisive fact. The documentary contract says publication states are backed by `develop`, `uat`, and `main`, but the live repo/workflow truth exposes only `develop` and `main` as real branch-control surfaces today. A handoff that pretends the `beta` backing branch already exists would be counterfeit.

## Exact required reads for the blocker-repair lane
The blocker-repair successor should require reading exactly these surfaces:
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/epic-breakdown.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/technical-requirements.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/technical-design.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/risks-and-open-questions.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/distribution-platform-support-and-channel-matrix.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/github-artifact-publication-and-promotion-contract.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md`
- `/home/ubuntu/workspace/repos/codex/AGENTS.md`
- `/home/ubuntu/workspace/repos/codex/README.md`
- `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`

## Exact candidate repo paths and allowed write scope once the blocker is repaired
The publication/promotion lane is narrower than `repos/codex/**`. If the missing `uat` control-plane truth is repaired, the next handoff should authorize only this candidate write surface unless a later readiness pass widens it explicitly:

### Primary candidate write paths
1. `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml`
   - writable only if the three-stage promotion contract remains authoritative and this existing branch-governance workflow must be widened, renamed, or otherwise normalized so `develop -> uat -> main` is explicit and auditable in repo truth
   - not a grant to rewrite unrelated branch policy, repository automation, or merge doctrine

2. `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml`
   - new workflow candidate for the first official publication lane
   - should become the authoritative GitHub-backed publication surface that maps the operator promotion states to tester-facing `alpha`, `beta`, and `stable` release units while attaching required assets, checksums, provenance/build receipt, and manifest reference
   - must refuse feature-branch artifacts as official tester-channel publication

### Read-only comparison surfaces for this lane
These files should be read for truth preservation but are not part of the first expected write scope:
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`

### Explicit scope boundary
If the later handoff discovers that publication/promotion cannot be implemented inside the workflow surfaces above, it must stop and route back through a new readiness closure before touching broader surfaces such as:
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/**`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/**` beyond read-only packaging truth checks
- `/home/ubuntu/workspace/repos/codex/src/**`
- updater-manifest consumer surfaces that belong to SD12-E3
- rollback/withdrawal surfaces that belong to SD12-E4
- provenance/integrity enforcement surfaces that belong to SD12-E5

## Exact non-goals for the first GitHub publication and promotion lane
The next handoff must state these non-goals plainly:
- no silent invention of a `uat` stage that does not exist in live repo truth
- no rewording of SD-11 tester-facing channel or support-tier copy
- no Linux package-format selection, bundle activation, or broader packaging work under `apps/desktop/**`
- no update-manifest generation or desktop manifest-consumer implementation
- no rollback, withdrawal, downgrade, or provenance/integrity implementation beyond publication-surface placeholders explicitly required by the contract
- no feature-branch publication masquerading as `alpha`, `beta`, or `stable`
- no PR, merge, or branch-ready evidence from this documentary card

## Exact verification surfaces for the blocker-repair lane and later handoff
The blocker-repair lane must verify the promotion truth directly, not by prose alone:

```bash
git -C /home/ubuntu/workspace/repos/codex branch --list
git -C /home/ubuntu/workspace/repos/codex branch -r
```

Required workflow/file verification surfaces:
- `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml`
- `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml` once it exists

Repo verification commands that must remain available for the eventual publication handoff:

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run typecheck
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run build
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run tauri:check
```

Verification interpretation:
- the git branch inventory proves whether `uat` exists as a real control-plane surface instead of mere documentary intent
- the workflow files prove whether the repo has an auditable publication lane rather than a branch-policy fragment mistaken for release automation
- `npm run typecheck` and `npm run build` protect the existing desktop shell proof surface from collateral drift while publication automation is introduced
- `npm run tauri:check` stays a required read-through verification surface if later workflow work touches packaging metadata or depends on Tauri packaging posture

## Readiness verdict
This lane is not yet ready for a handoff artifact.

Why it is blocked:
- the accepted SD-11 and SD-12 documentary surfaces define three promotion-backed publication states: `alpha` from `develop`, `beta` from `uat`, and `stable` from `main`
- the live repo has no local or remote `uat` branch, so the `beta` backing surface is not real yet
- the only live GitHub workflow proves a `develop -> main` gate and branch restoration, not a three-stage promotion model and not a release/publication lane
- authoring `SD12-E2-R2` now would either invent nonexistent repo truth or smuggle branch-governance repair into what should be a bounded publication handoff

Why the closure still matters:
- it narrows the future write surface to `.github/workflows/**` instead of letting publication work sprawl into desktop UI or updater-consumer files
- it identifies the exact control-plane mismatch that must be repaired before a code-authorizing or handoff-authorizing lane can be honest
- it preserves the existing desktop verification surface and SD-11 authority as read-only comparison truth instead of accidental implementation scope

## Successor truth
The real blocker is missing live `uat` promotion truth. The next truthful board move is therefore a blocker-repair pair:
- `SD12-E2-B1 FLOW: Resolve missing uat publication and promotion truth`
  - unblock condition: either a real `uat` branch and corresponding promotion-governance evidence exist in repo truth, or the SD-11/SD-12 documentary contract is revised explicitly so the tester-channel model no longer claims a three-stage `develop -> uat -> main` publication path
- `SD12-E2-R2 FLOW: GitHub publication and promotion handoff artifact`
  - this card should remain gated behind `SD12-E2-B1` and should only author the stage-specific handoff after the promotion topology is truthful again
