---
title: SD11-F10 Execution Handoff — Check/Update Action Surface from Accepted SD-12 Authority
handoff_id: HANDOFF-CODEX-SD-11-F10-CODING-2026-06-29
stc_id: STC-CODEX-SD-11
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: pr-created
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/sd11-update-action-execution-handoff.md
source_stc: programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md
source_epic_breakdown: programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/epic-breakdown.md
selected_slice: SD11-F10 — Check/update action surface
run_in: Claude Code only
code_authority: true
created_at: 2026-06-29
authority_dependencies:
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/technical-design.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/github-artifact-publication-and-promotion-contract.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/self-update-transport-and-manifest-contract.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/rollback-withdrawal-and-downgrade-policy.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/provenance-integrity-and-update-eligibility.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: develop
  expected_base_sha_at_creation: 5d131d0a8c98358cae8d0f15ff770a25dc109408
  recommended_branch: sd11-f10-update-action-surface
  pr_target: develop
allowed_write_scope:
  - apps/desktop/src/App.tsx
  - apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts
  - apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts
  - apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts
  - apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts
  - apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts
  - apps/desktop/src/sd11/update/**
  - apps/desktop/src/boundary/loadSd11UpdateAction.ts
  - apps/desktop/src-tauri/src/main.rs
  - apps/desktop/src-tauri/src/sd11_update_action.rs
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen/**
  - programs/codex/**
  - .github/**
  - apps/desktop/package.json
  - apps/desktop/package-lock.json
  - apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts
  - apps/desktop/src/boundary/loadPilotShellSnapshot.ts
  - apps/desktop/src-tauri/Cargo.toml
  - apps/desktop/src-tauri/Cargo.lock
  - apps/desktop/src-tauri/tauri.conf.json
  - apps/desktop/src-tauri/icons/**
  - src/**
  - tests/**
  - AGENTS.md
  - CLAUDE.md
---

# SD11-F10 Execution Handoff — Check/Update Action Surface from Accepted SD-12 Authority

## Status
This is the stage-specific code-authorizing brief for SD11-F10.

It carries `code_authority: true` for the bounded SD-11 check/update action surface only and now records the executed lane truth: the governed CODE card ran, branch evidence was preserved, and PR #34 became the terminal review surface.

Historical execution surface:
- governed CODE card: `kanban://codex-phase-2/t_a8dc147b`
- PR: `https://github.com/electricm0nk/codex/pull/34`
- branch: `sd11-f10-update-action-surface`

## Run in
Claude Code only.

Do not run this in Hermes as a documentary card. Hermes produced this handoff; Claude Code implements it. If Claude Code cannot be launched truthfully, block the downstream CODE lane instead of coding through Hermes.

## Core problem
SD-11 already exposes build/channel/support truth, but the updater surface is still hard-coded to `not-yet-supported`. SD-12 now fixes the authoritative publication, manifest, rollback, and integrity rules tightly enough that SD-11 can implement a bounded check/update action surface without inventing release doctrine locally.

The decisive constraint is equally important: SD-11 is a consumer of SD-12 truth, not the release control plane itself. This slice must add an honest tester-facing action surface without widening into GitHub publication automation, manifest-schema invention, package/installer work, or fake parity across Linux, macOS, and Windows.

## Objective
Implement the smallest truthful SD-11 updater action surface over accepted SD-12 authority.

The result must prove:
1. the workbench no longer stops at the static `Update checks not yet wired in this slice` posture
2. a tester can trigger a bounded check/update action from the SD-11 surface
3. the action result distinguishes `up-to-date`, `update-available`, `manual-only`, `blocked`, `withdrawn`, `unsupported`, and `check-failed` style outcomes instead of flattening everything into success/failure prose
4. Linux-first, macOS-second-class, and Windows-third-class truth remains explicit in both status and action outcomes
5. feature branches and non-official builds do not masquerade as governed tester-channel update sources
6. rollback / withdrawal / recovery state stays visible when SD-12 authority says a build is superseded, withdrawn, blocked, or recovery-preferred

## Why this route is now authorized
The accepted SD-12 authority surface fixed the product truth that SD-11 must consume:
- `technical-design.md` fixes the desktop surface as a consumer of publication/manifest truth, not its author
- `github-artifact-publication-and-promotion-contract.md` fixes official release units as the only governed update origin
- `self-update-transport-and-manifest-contract.md` fixes the minimum outcome vocabulary and manifest obligations
- `rollback-withdrawal-and-downgrade-policy.md` fixes the required withdrawn/superseded/blocked/recovery behavior
- `provenance-integrity-and-update-eligibility.md` fixes when `automatic` may be claimed and explicitly forbids cross-platform trust flattening

That is enough to authorize an SD-11 consumer slice. It is not enough to authorize release automation, package topology changes, updater-plugin activation, or CI/release workflow work.

## Branch policy
Do not launch this lane from the currently checked-out local branch merely because it exists.

Observed local repo truth at handoff creation:

```text
current local branch: feat/sd11-enhancement-request-composer
current local head:   8f3a627655f490551ff23746293cde1622085e97
merge-base with origin/develop: 5d131d0a8c98358cae8d0f15ff770a25dc109408
remote branch of same name: none
```

That branch is not the updater lane and is not the truthful base for SD11-F10.

Launch from `origin/develop` instead:

```bash
git fetch origin --prune
git switch -c sd11-f10-update-action-surface origin/develop
```

Record the actual branch and base SHA in the final receipt.

## Required reads before coding
Read these first, in order:
1. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
2. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
3. `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/sd11-update-action-execution-handoff.md`
4. `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md`
5. `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/technical-requirements.md`
6. `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/technical-design.md`
7. `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/epic-breakdown.md`
8. `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md`
9. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/technical-design.md`
10. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/github-artifact-publication-and-promotion-contract.md`
11. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/self-update-transport-and-manifest-contract.md`
12. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/rollback-withdrawal-and-downgrade-policy.md`
13. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/provenance-integrity-and-update-eligibility.md`
14. `/home/ubuntu/workspace/repos/codex/README.md`
15. `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json`
16. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
17. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
18. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`
19. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
20. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts`
21. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts`
22. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts`
23. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadPilotShellSnapshot.ts`
24. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`

## Conditional reads
Read these only if the corresponding condition actually occurs:
1. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml`
   - only if you truly need to verify existing Tauri/Rust dependency availability before wiring `sd11_update_action.rs`
   - if the implementation would require changing this file, stop and report the blocker instead of widening the lane
2. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json`
   - only if you suspect the slice is drifting into bundle/updater-plugin activation work
   - reading it here is a guardrail, not permission to edit it
3. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.ts`
4. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.ts`
   - only if you need to thread the new update-state labels into existing issue payload copy without changing their schemas
5. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/technical-requirements.md`
   - only if you need the full prose restatement of minimum manifest fields or support-tier rules beyond the four SD-12 authority artifacts above
6. Tauri updater documentation or GitHub release API documentation
   - only if the code path is already fully bounded by this handoff and you need API syntax details
   - those docs must not be used to change the product contract, widen scope, or bypass the SD-12 authority packet

## Contract to implement
Implement a bounded SD-11 update-action consumer surface. Do not implement the SD-12 control plane.

### Minimum UI/interaction result
The workbench must gain a tester-visible update action and result area that:
- keeps current build/channel/support state visible
- lets the tester trigger a bounded update check
- reports the resulting state explicitly
- preserves platform/support asymmetry in the result copy
- does not expose raw branch names as the primary product language

### Required outcome vocabulary
The implementation may choose exact internal naming, but the visible/result model must preserve at least these distinctions:
- `up-to-date`
- `update-available`
- `manual-only`
- `blocked`
- `withdrawn`
- `unsupported`
- `check-failed`
- `no-official-release-for-this-build` or an equally explicit equivalent for non-governed local/feature builds

### SD-12 authority rules the code must enforce
1. Official update truth comes only from governed GitHub-backed release units, never from feature branches.
2. A build may be treated as `automatic` only when the SD-12 integrity gate is satisfied for that platform.
3. Linux may surface the strongest action posture first.
4. macOS must not be promoted to fake parity. Until a later explicit slice proves otherwise, its truthful outcomes are `manual-only`, `blocked`, `withdrawn`, `unsupported`, or `up-to-date` against already-governed releases.
5. Windows must remain explicitly bounded in this tranche. No automatic-update claim.
6. Withdrawn, superseded, blocked, or recovery-preferred states must stay visible instead of collapsing into generic failure.

### Recommended boundary shape
A narrow new boundary file is authorized:

```text
apps/desktop/src/boundary/loadSd11UpdateAction.ts
```

A narrow new runtime module is also authorized:

```text
apps/desktop/src-tauri/src/sd11_update_action.rs
```

But the implementation must stay read-only from the product point of view. This slice is about truthful check/action state, not applying installers, mutating releases, or building packages.

### Current repo anchor points
The action surface should extend these existing facts rather than replacing them:
- `createSd11WorkbenchStatus.ts` currently hard-codes `not-yet-supported`
- `loadSd11TesterWorkbenchSurface.ts` currently projects status/update text into the workbench surface
- `App.tsx` already shows the bounded workbench frame and status cards
- `loadGe08AuthoringWorkbench.ts` and `loadPilotShellSnapshot.ts` already model the preferred Tauri-boundary plus explicit-fallback pattern
- `src-tauri/src/main.rs` already exposes bounded Tauri commands and a repo-relative path pattern

## Required implementation constraints
- keep the slice inside `apps/desktop/**`
- consume SD-12 truth as a client/consumer surface; do not author release truth locally
- preserve the tester-facing channel labels `alpha`, `beta`, `stable` as layered language over operator promotion truth
- keep failure and blocked-path truth attributable to build/channel/platform state
- if the current build is not an official tester release unit, say so explicitly instead of pretending a normal update check succeeded
- if the runtime boundary cannot prove official update truth, surface a guarded failure/manual-only result rather than silent success

## Exact non-goals
This handoff does not authorize:
- GitHub release creation, publication-state mutation, or CI/release workflow edits
- manifest-schema invention beyond consuming the SD-12 minimum contract
- Tauri bundle activation, installer packaging, signing, notarization, or Windows trust work
- applying downloaded binaries or claiming end-to-end self-update completion
- changes under `.github/**`, `src/**`, `tests/**`, or `programs/codex/**`
- broad GE-07 shell rewrites, GE-08 workflow changes, or general character-builder expansion
- exposing raw `develop`, `uat`, or `main` as the default tester-facing update language

## Forbidden widening
Stop and report the blocker if the implementation appears to require any of these:
- editing `apps/desktop/package.json` or adding a new JS test framework
- editing `apps/desktop/src-tauri/Cargo.toml`, `Cargo.lock`, `tauri.conf.json`, or any icons/bundle config
- editing `.github/workflows/**`
- editing root Rust code under `src/**`
- inventing a new release backend or bypassing GitHub-backed release truth
- turning macOS or Windows into automatic-update surfaces by implication

## Verification commands
Run these at minimum:

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop
npm run typecheck
npm run build
```

If any Rust/Tauri command boundary file was touched, also run:

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop
npm run tauri:check
```

Important truth surfaced during handoff authoring:
- `npm run typecheck` passes in the live repo
- direct execution of the current ad hoc `.test.ts` assertion files via Node failed on ESM import-resolution, so those files are not a sufficient launch verifier by themselves in their current form

Interpretation:
- treat `typecheck`, `build`, and conditional `tauri:check` as the minimum acceptance proof for this slice
- if you add new assertion-style test files under the allowed write scope, they may supplement these commands, but they do not replace the required commands above

## Failure / rollback behavior for this slice
The implemented action surface must never counterfeit success.

At minimum:
- manifest/release lookup failure -> explicit `check-failed` with actionable detail
- governed but non-automatic platform -> explicit `manual-only` or `unsupported`
- withdrawn build -> explicit warning + recovery target or replacement direction
- blocked build -> explicit blocked reason; no apply path
- superseded build -> explicit preferred replacement
- local feature build or non-governed build -> explicit `no-official-release-for-this-build` equivalent, not normal `up-to-date`

## Merge authority boundary
This handoff does not authorize the coding harness to merge anything.

- Do not merge the branch or PR.
- Do not land code directly onto `develop` or `main`.
- Stop at verified branch-ready or PR-ready state and hand control back to Todd.

## Final report requirements
The final execution receipt must include:
- exact handoff path
- actual branch name
- actual base SHA
- files changed
- whether the implementation stayed fully TypeScript-side or also touched the Tauri/Rust boundary
- final verification commands and results
- the exact visible outcome vocabulary implemented
- whether the lane stopped at branch-ready or PR-ready

Without that receipt, this lane must not be described as Claude-executed.
