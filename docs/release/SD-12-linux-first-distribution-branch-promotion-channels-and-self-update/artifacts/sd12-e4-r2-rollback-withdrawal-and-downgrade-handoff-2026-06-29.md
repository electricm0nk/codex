---
title: SD12-E4-R2 Execution Handoff — Rollback, withdrawal, and downgrade recovery surface
handoff_id: HANDOFF-CODEX-SD12-E4-R2-CODING-2026-06-29
stc_id: STC-CODEX-SD-12
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: blocked
status: draft
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e4-r2-rollback-withdrawal-and-downgrade-handoff-2026-06-29.md
source_stc: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md
source_epic_breakdown: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/epic-breakdown.md
readiness_closure: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e4-r1-execution-readiness-closure-2026-06-29.md
boundary_truth_repair: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e4-b1-rollback-withdrawal-and-downgrade-boundary-truth-2026-06-29.md
selected_slice: SD12-E4 — first bounded rollback/withdrawal/downgrade recovery surface
run_in: Claude Code only once unblocked
code_authority: false
created_at: 2026-06-29
authority_dependencies:
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/rollback-withdrawal-and-downgrade-policy.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/self-update-transport-and-manifest-contract.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e4-r1-execution-readiness-closure-2026-06-29.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e4-b1-rollback-withdrawal-and-downgrade-boundary-truth-2026-06-29.md
  - /home/ubuntu/workspace/repos/codex/AGENTS.md
  - /home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml
  - /home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts
  - /home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts
  - /home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts
  - /home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts
  - /home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx
  - /home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: develop
  recommended_branch: sd12-e4-rollback-recovery-surface
  pr_target: develop
allowed_write_scope:
  - .github/workflows/publish-tester-release.yml
  - apps/desktop/src/boundary/**
  - apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts
  - apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts
  - apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts
  - apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts
  - apps/desktop/src/App.tsx
  - apps/desktop/src-tauri/src/main.rs
forbidden_write_scope:
  - programs/codex/**
  - apps/desktop/package.json
  - apps/desktop/src-tauri/Cargo.toml
  - apps/desktop/src-tauri/tauri.conf.json
  - apps/desktop/src/** outside the exact paths named above
  - apps/desktop/src-tauri/** outside src/main.rs
  - src/**
  - tests/** outside apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts
  - README.md
  - AGENTS.md
  - CLAUDE.md
---

# SD12-E4-R2 Execution Handoff — Rollback, withdrawal, and downgrade recovery surface

## Status
This is a stage-specific prebuild handoff, not a launch-authorizing code brief yet.

It exists because the boundary truth is now narrow enough to define exact repo/workdir, exact write scope, exact reads, exact non-goals, and exact verification commands without inventing hidden surfaces. It remains `blocked` and carries `code_authority: false` because the current producer/consumer substrate is only partially grounded and is not yet durable enough to authorize launch.

## Run in
Claude Code only once the unblock conditions below are satisfied.

Do not run this lane in Hermes as though it were already an executable code packet. Hermes authored the handoff. Claude Code should execute it only after the substrate is accepted as durable repo truth and the final receipt can name the actual branch/base/evidence cleanly. If Claude Code cannot be launched truthfully once unblocked, keep the lane blocked instead of substituting another harness.

## Core problem
The documentary contract now has a bounded home for the recovery lane, but the repo still does not provide a fully accepted end-to-end rollback/withdrawal/downgrade surface.

Current grounded state as of this handoff pass:
1. A recovery-state publication candidate now exists as `.github/workflows/publish-tester-release.yml` and it emits a manifest/update record with `updateEligibilityState: manual-only` plus `replacementReleaseId` support.
2. A runtime-facing read-only release-truth candidate now exists in `apps/desktop/src-tauri/src/main.rs` via `load_sd12_release_truth`.
3. The tester-facing SD-11 surface still remains hard-coded around `not-yet-supported` in `createSd11WorkbenchStatus.ts`, and no dedicated TypeScript boundary file under `apps/desktop/src/boundary/**` currently consumes the release truth or recovery metadata.
4. The publication candidate is still only local workspace residue (`git status --short` shows `.github/workflows/publish-tester-release.yml` as untracked), and the runtime consumer candidate in `src-tauri/src/main.rs` is still an uncommitted modification.

That is the decisive constraint. The lane is now classifiable, but not yet durably launchable. The producer side is workflow-backed and manifest-backed only as a local candidate. The consumer side is partially Tauri-command-backed, but it is not yet a complete desktop recovery boundary with dedicated TypeScript wiring and tester-facing SD-11 status/App integration.

## Grounded path classification
State this plainly and do not improvise beyond it:

- publication/release-state producer path: workflow-backed and manifest-backed
  - candidate file: `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml`
  - grounded behavior seen in the file: emits release-unit metadata, `manifestVersion`, `updateEligibilityState`, `replacementReleaseId`, checksum/provenance assets, and release notes
  - current truth limit: this file is still untracked local workspace residue, not yet durable branch truth

- runtime consumer path: partially Tauri-command-backed
  - candidate file: `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`
  - grounded behavior seen in the file: exposes `load_sd12_release_truth` and derives a read-only trust/update snapshot from release metadata env vars
  - current truth limit: there is still no dedicated TypeScript boundary file under `apps/desktop/src/boundary/**`, no proven runtime loader chain consuming this command, and no tester-facing SD-11 status/App surface yet presenting rollback/withdrawal/downgrade truth from it

- current tester-facing truth: still static and not yet recovery-capable
  - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts` still hard-codes `state: 'not-yet-supported'`
  - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx` does not yet carry a bounded rollback/withdrawal/downgrade guidance surface for testers

Therefore this lane is not custom-fetch-backed today, and it is not yet a fully grounded dedicated boundary chain. It is a workflow-backed/manifest-backed publication candidate plus a partial Tauri-command-backed runtime candidate, with the dedicated desktop consumer still incomplete.

## Objective once unblocked
Implement the smallest honest rollback/withdrawal/downgrade recovery surface for SD-12 without widening into unrelated publication, packaging, updater, provenance, or SD-11 feature work.

The eventual result must prove all of the following:
1. the desktop surface can consume grounded release truth without pretending local build residue is official publication truth
2. tester-facing update/recovery posture can distinguish at least `manual-only`, `unsupported`, `withdrawn`, `blocked`, and `superseded` behavior when the publication truth actually supplies those states
3. rollback/downgrade guidance is explicit and user-visible when automatic update is not approved
4. operator branch provenance remains secondary to tester-facing channel/support language
5. the lane refuses counterfeit silent-success claims for rollback, downgrade, withdrawal, or recovery

## Why this handoff is blocked instead of live
This handoff intentionally stops short of code authority because the repo still lacks durable accepted truth for both ends of the lane.

Exact remaining unblock conditions:
1. `.github/workflows/publish-tester-release.yml` must stop being mere local residue and become accepted repo truth on a clean branch pushed to `origin` or equivalent durable evidence surface
2. the runtime consumer must stop being a partial `src-tauri/src/main.rs` candidate and become a named accepted desktop recovery boundary with a clear chain from producer truth -> runtime read surface -> SD-11 status/composition -> App presentation
3. the substrate must stay within the exact write scope listed above; if it needs broader files, another readiness/truth pass is required before launch

Until those are true, launching a coding harness would force it to improvise around dirty residue and incomplete desktop ownership.

## Branch policy
Do not launch from the currently checked-out local branch merely because it contains candidate files.

Observed local repo truth during this handoff pass:

```text
current local branch: feat/sd11-enhancement-request-composer
current local head:   8f3a627655f490551ff23746293cde1622085e97
workspace residue:    .github/workflows/publish-tester-release.yml is untracked
workspace residue:    apps/desktop/src-tauri/src/main.rs is modified
workspace residue:    apps/desktop/package.json is modified
```

That checkout is not a truthful execution base for this lane.

When the lane is unblocked, launch from clean durable branch truth instead:

```bash
git fetch origin --prune
git switch -c sd12-e4-rollback-recovery-surface origin/develop
```

If the accepted producer/consumer substrate lands on a different pushed base first, record that exact base SHA and stack policy explicitly in the execution receipt. Do not inherit a dirty shared checkout by assumption.

## Exact allowed write scope
You may create or modify only these paths:

```text
/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml
/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/**
/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts
/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts
/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts
/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts
/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx
/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs
```

Write-scope interpretation:
- the workflow file may be edited only to the extent required to carry truthful recovery-facing release state
- `src/boundary/**` is the preferred home for the dedicated desktop recovery consumer once it is introduced
- `createSd11WorkbenchStatus.ts` and its test are the bounded status-model surfaces that may widen beyond `not-yet-supported`
- `loadSd11TesterWorkbenchSurface.ts`, `loadSd11TesterWorkbenchSurfaceRuntime.ts`, and `App.tsx` are the bounded composition/presentation surfaces for truthful tester-facing recovery guidance
- `src-tauri/src/main.rs` is the only allowed Tauri-side write surface in this lane
- if truthful implementation requires `package.json`, `Cargo.toml`, `tauri.conf.json`, broader `src/**`, or any other surface, stop and route back through readiness instead of widening locally

## Required reads before coding
Read these first, in order:
1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. this handoff file
3. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e4-r1-execution-readiness-closure-2026-06-29.md`
4. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e4-b1-rollback-withdrawal-and-downgrade-boundary-truth-2026-06-29.md`
5. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/rollback-withdrawal-and-downgrade-policy.md`
6. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/self-update-transport-and-manifest-contract.md`
7. `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml`
8. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`
9. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
10. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`
11. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
12. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts`
13. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
14. `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json` as read-only dependency/tooling context only

## Conditional reads
Read these only if the corresponding condition occurs:
1. any file introduced under `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/**`
   - only once a dedicated recovery boundary exists or is created for this lane
2. any branch-ready receipt or PR evidence that makes `.github/workflows/publish-tester-release.yml` durable
   - only when you need the accepted base SHA, branch, or exact publication truth provenance
3. any future manifest/update consumer test files
   - only if this lane introduces them inside the allowed write scope

## Exact non-goals
This handoff does not authorize:
- inventing `uat`, a hidden candidate branch, or any publication topology beyond the already grounded SD-12 publication truth
- treating untracked or uncommitted workspace residue as accepted branch-ready evidence
- broad package/build-tooling edits in `apps/desktop/package.json`
- dependency/config edits in `apps/desktop/src-tauri/Cargo.toml` or `apps/desktop/src-tauri/tauri.conf.json`
- using GE-08 preview `blocked` or `unsupported` diagnostics as though they already satisfy release/update recovery truth
- silent success claims for rollback, downgrade, withdrawal, or recovery
- rewriting tester-facing SD-11 wording so operator branch names become the primary product surface
- widening into provenance/integrity work that belongs to SD12-E5
- widening into unrelated feedback-intake, enhancement-request, or other SD-11 workbench flows
- PR, merge, or release publication claims from this documentary handoff itself

## Forbidden widening / stop conditions
Stop and report the blocker if any of these become true:
1. the workflow-backed producer cannot carry the required recovery state truth inside `.github/workflows/publish-tester-release.yml`
2. the desktop consumer cannot be implemented inside the exact allowed files above
3. truthful implementation requires `package.json`, `Cargo.toml`, `tauri.conf.json`, or broader repo surfaces
4. the only available consumer path remains an uncommitted local `main.rs` change with no dedicated boundary/UI/status chain
5. the lane would have to claim rollback, downgrade, or automatic recovery success without grounded publication truth and visible tester guidance
6. the lane would need to rewrite publication topology, manifest carrier doctrine, or branch semantics rather than consuming the repaired truth

If a stop condition lands, do not improvise. Return a blocker naming the exact broader surface now required.

## Verification commands
Run these at minimum when the lane actually launches:

```bash
git -C /home/ubuntu/workspace/repos/codex status --short
git -C /home/ubuntu/workspace/repos/codex branch --list
git -C /home/ubuntu/workspace/repos/codex branch -r
git -C /home/ubuntu/workspace/repos/codex diff --name-only -- \
  .github/workflows/publish-tester-release.yml \
  apps/desktop/src/boundary \
  apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts \
  apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts \
  apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts \
  apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts \
  apps/desktop/src/App.tsx \
  apps/desktop/src-tauri/src/main.rs
git -C /home/ubuntu/workspace/repos/codex diff --unified=0 -- \
  .github/workflows/publish-tester-release.yml \
  apps/desktop/src/boundary \
  apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts \
  apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts \
  apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts \
  apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts \
  apps/desktop/src/App.tsx \
  apps/desktop/src-tauri/src/main.rs
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run typecheck
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run build
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run tauri:check
```

## Verification interpretation
- `git status --short` must show whether the substrate is clean durable branch truth or still dirty local residue
- `branch --list` and `branch -r` confirm the launch base and whether the branch authority remains grounded
- `diff --name-only` must prove that only the allowed recovery surfaces changed
- `diff --unified=0` is the review surface for confirming that the lane did not smuggle in publication-topology edits, hidden updater claims, or scope creep
- `typecheck`, `build`, and `tauri:check` protect the desktop proof surface while the recovery lane is introduced

## Merge authority boundary
This handoff does not authorize merging.

- Do not merge the branch or PR.
- Do not land code directly onto `develop` or `main`.
- Stop at verified branch-ready or PR-ready evidence and hand control back to Todd.

## Final report requirements
When this handoff eventually becomes live and is executed, the final receipt must include:
- exact handoff path
- actual branch name
- actual base SHA
- files changed
- whether `.github/workflows/publish-tester-release.yml` remained aligned or required bounded edits
- the exact recovery-consumer path chosen (`src/boundary/**`, `src-tauri/src/main.rs`, or both)
- whether the tester-facing status surface stayed truthful and kept operator provenance secondary
- exact verification commands and actual results
- the final evidence class: `branch-ready` or `pr-ready`
- any remaining guardrails that still keep automatic update blocked or manual-only

Without that receipt, this lane must not be described as frontier-harness executed.
