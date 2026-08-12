---
title: SD12-E2-R2 Execution Handoff — GitHub publication and promotion control plane
handoff_id: HANDOFF-CODEX-SD12-E2-R2-CODING-2026-06-29
stc_id: STC-CODEX-SD-12
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: merged
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e2-r2-github-publication-and-promotion-handoff-2026-06-29.md
source_stc: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md
source_epic_breakdown: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/epic-breakdown.md
readiness_closure: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e2-r1-execution-readiness-closure-2026-06-29.md
promotion_truth_repair: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e2-b1-promotion-truth-repair-closure-2026-06-29.md
selected_slice: SD12-F3 + SD12-F4 — First honest GitHub publication and promotion lane
run_in: Claude Code only
code_authority: true
created_at: 2026-06-29
authority_dependencies:
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/github-artifact-publication-and-promotion-contract.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/distribution-platform-support-and-channel-matrix.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e1-f1-execution-handoff-2026-06-29.md
  - /home/ubuntu/workspace/repos/codex/CLAUDE.md
  - /home/ubuntu/workspace/repos/codex/AGENTS.md
  - /home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: develop
  expected_base_sha_at_creation: da66e2286ba0f8e0e5d9ba61386e79f3bfe236e5
  recommended_branch: sd12-e2-github-publication-promotion
  pr_target: develop
allowed_write_scope:
  - .github/workflows/allow-only-develop-into-main.yml
  - .github/workflows/publish-tester-release.yml
forbidden_write_scope:
  - programs/codex/**
  - apps/desktop/**
  - src/**
  - tests/**
  - README.md
  - AGENTS.md
  - CLAUDE.md
---

# SD12-E2-R2 Execution Handoff — GitHub publication and promotion control plane

## Status
This is the stage-specific code-authorizing brief for the first honest SD12 publication/promotion lane.

It carries `code_authority: true` for the bounded GitHub workflow surface only and now records historical accepted truth: the bounded publication/promotion lane landed in accepted repo state via PR #32 rather than remaining launch-pending.

Historical acceptance surface:
- merged PR: `#32`
- accepted repo truth includes `.github/workflows/publish-tester-release.yml`

## Run in
Claude Code only.

Do not run this lane in Hermes as a documentary card. Hermes authored the handoff. Claude Code implements it and must leave a durable execution receipt. If Claude Code cannot be launched truthfully, block the downstream CODE lane instead of coding through Hermes.

## Core problem
The missing `uat` fiction is gone. Live promotion truth is now explicit and honest: `develop -> main` is the only governed operator path, `alpha` is backed by `develop`, `stable` is backed by `main`, and `beta` remains reserved until a real candidate promotion surface exists.

The next honest move is therefore not to invent a third branch. It is to create the first bounded GitHub publication control plane that can:
1. publish official tester-channel units from `develop` and `main` only
2. preserve `alpha` and `stable` semantics without exposing raw branch names as the product surface
3. refuse feature-branch publication as official channel truth
4. keep all first-pass write scope inside `.github/workflows/**`

The decisive constraint is equally important: this lane is publication/promotion control-plane work, not packaging, updater-manifest, rollback, or provenance/integrity completion. If the workflow cannot be authored truthfully inside the bounded workflow files, the lane must stop and report the broader surface now required.

## Objective
Implement the smallest honest GitHub publication and promotion workflow surface for SD-12.

The result must prove:
1. official tester-channel publication can originate only from governed promotion truth, never from feature branches
2. `develop` maps to official `alpha` publication posture
3. `main` maps to official `stable` publication posture
4. `beta` remains explicitly unavailable until a real candidate promotion surface exists in repo/workflow truth
5. every official publication unit is shaped to carry the release-unit contents already required by the SD-12 contract: platform artifact bundle, checksum artifact, provenance/build receipt artifact, and manifest payload or manifest reference
6. the lane stops at branch-ready or PR-ready control-plane evidence only; it does not claim live GitHub release proof from this slice by itself

## Branch policy
Do not launch this lane from the currently checked-out local branch merely because it exists.

Observed local repo truth at handoff creation:

```text
current local branch: feat/sd11-enhancement-request-composer
current local head:   8f3a627655f490551ff23746293cde1622085e97
origin/develop:       da66e2286ba0f8e0e5d9ba61386e79f3bfe236e5
live workflow files:  /home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml only
```

That local feature branch is not the truthful base for this lane.

Launch from `origin/develop` instead:

```bash
git fetch origin --prune
git switch -c sd12-e2-github-publication-promotion origin/develop
```

Record the actual branch name and base SHA in the final execution receipt.

## Exact allowed write scope
You may create or modify only these paths:

```text
/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml
/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml
```

Write-scope interpretation:
- `allow-only-develop-into-main.yml` may be edited only if a narrow normalization is required to preserve the repaired two-stage promotion truth or to keep the publication workflow aligned with it
- `publish-tester-release.yml` is the new bounded workflow candidate for official tester-channel publication
- if `allow-only-develop-into-main.yml` is already sufficient, leave it unchanged and add only `publish-tester-release.yml`
- do not write any other `.github/**` file, any repo code file, any packaging config file, or any documentation file in this lane

## Required reads before coding
Read these first, in order:
1. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
2. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
3. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e2-r2-github-publication-and-promotion-handoff-2026-06-29.md`
4. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e2-r1-execution-readiness-closure-2026-06-29.md`
5. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e2-b1-promotion-truth-repair-closure-2026-06-29.md`
6. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md`
7. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/epic-breakdown.md`
8. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/github-artifact-publication-and-promotion-contract.md`
9. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/distribution-platform-support-and-channel-matrix.md`
10. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e1-f1-execution-handoff-2026-06-29.md`
11. `/home/ubuntu/workspace/repos/codex/README.md`
12. `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml`
13. `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json`
14. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json`
15. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml`

Use the desktop and packaging files as read-only comparison truth only. They do not enter write scope here.

## Conditional reads
Read these only if the corresponding condition actually occurs:
1. `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml`
   - after you create it, to verify the final bounded workflow surface exactly as written
2. any future SD12-E1 branch-ready receipt or PR evidence
   - only if such evidence exists by launch time and you need the exact emitted Linux artifact paths or filenames rather than the handoff-level contract
3. GitHub Actions / GitHub Releases documentation
   - only if you need syntax details for workflow triggers, permissions, release/prerelease creation, or asset upload steps
   - those docs must not be used to widen product contract, invent `beta`, or bypass the repaired branch truth

## Contract to implement
Implement a bounded GitHub publication control plane. Do not implement the rest of SD-12.

### Minimum workflow result
The workflow surface must establish all of the following:
- official tester-channel publication is branch-governed and auditable
- only `develop` and `main` may create official tester-channel publication units
- `develop` yields the governed `alpha` publication posture
- `main` yields the governed `stable` publication posture
- `beta` remains reserved and unavailable; there is no `uat`, candidate, or hidden third-stage publication path in this lane
- feature branches may still exist for development, but they must not masquerade as official `alpha`, `beta`, or `stable` publication sources

### Required release-unit obligations the workflow must preserve
The workflow may choose exact implementation steps, but it must preserve the SD-12 contract that every official publication unit carries or links:
1. the allowed platform artifact bundle
2. checksum output covering the distributed assets
3. a provenance/build receipt artifact linked to source revision and publication event
4. manifest payload or manifest reference resolvable from the GitHub-backed publication unit
5. human-readable release notes stating tester-facing channel, platform scope, and any manual-only or unsupported posture

If truthful implementation of one of those obligations would require editing non-workflow repo files, stop and report the blocker. Do not absorb the missing slice here.

### E1 dependency boundary
`SD12-E1-F1` defines the first Linux artifact-production contract. This publication lane may consume its documented artifact-emission assumptions, but it may not rewrite them.

Therefore:
- use `sd12-e1-f1-execution-handoff-2026-06-29.md` as the authority for the first Linux artifact-generation command and artifact-role assumptions
- if the publication workflow cannot be authored honestly without changing `apps/desktop/package.json`, `src-tauri/tauri.conf.json`, `Cargo.toml`, or other packaging surfaces, stop and report the blocker instead of widening into E1 work
- if exact asset filenames or locations cannot be grounded from accepted E1 authority or live repo truth, fail explicitly and name the missing truth in the final receipt

### Existing branch-governance workflow rule
The repaired truth must remain visible:
- pull requests into `main` must still come from `develop`
- no edit in this lane may introduce `uat`, candidate-branch folklore, or a fake `beta` backing branch
- this lane may normalize the existing branch-governance workflow only to preserve or clarify that repaired truth

## Exact non-goals
This handoff does not authorize:
- updater-manifest schema generation or desktop manifest-consumer work
- rollback, withdrawal, downgrade, or provenance/integrity implementation beyond bounded publication-surface placeholders explicitly required by the existing contract
- SD-11 tester-facing wording changes
- packaging-surface edits under `apps/desktop/**`
- root Rust or application-code edits under `src/**`
- additional `.github/**` work outside the two allowed workflow files
- repo settings, branch protection UI changes, GitHub environment changes, or secret-management changes outside what can be expressed in the allowed workflow files
- a fake `beta` or candidate publication path
- live merge, live release publication, or live promotion proof from this documentary-to-code handoff alone

## Forbidden widening / stop conditions
Stop and report the blocker if any of these become true:
1. truthful implementation requires edits outside the two allowed workflow files
2. truthful implementation requires introducing a real candidate-stage branch or a `uat` surface
3. truthful implementation depends on undocumented package output names, paths, or manifest/provenance assets that cannot be grounded from the accepted E1 handoff or live repo truth
4. truthful implementation requires repo settings, secrets, or environment wiring that cannot be represented or at least stubbed honestly inside the bounded workflow file surface
5. the workflow would need to change SD-11 wording or desktop code to stay coherent
6. the scope drifts into updater, rollback, provenance/integrity, or general release-engineering doctrine rather than this first bounded control-plane slice

If a stop condition lands, do not improvise. Return a blocker naming the exact broader surface now required.

## Verification commands
Run these at minimum:

```bash
git -C /home/ubuntu/workspace/repos/codex branch --list
git -C /home/ubuntu/workspace/repos/codex branch -r
git -C /home/ubuntu/workspace/repos/codex diff --name-only -- .github/workflows/allow-only-develop-into-main.yml .github/workflows/publish-tester-release.yml
git -C /home/ubuntu/workspace/repos/codex diff --unified=0 -- .github/workflows/allow-only-develop-into-main.yml .github/workflows/publish-tester-release.yml
find /home/ubuntu/workspace/repos/codex/.github/workflows -maxdepth 1 -type f | sort
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run typecheck
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run build
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run tauri:check
```

## Verification interpretation
- `branch --list` and `branch -r` must continue to prove the repaired two-stage truth: `develop` and `main` exist, `uat` does not
- `diff --name-only` must show that only the allowed workflow files changed
- `diff --unified=0` is the review surface for confirming that no `uat`/candidate fiction was reintroduced and that only `alpha`/`stable` publication paths are claimed as live
- `find .github/workflows` must show the bounded workflow surface, including the new `publish-tester-release.yml` if it was created
- `typecheck`, `build`, and `tauri:check` preserve the existing desktop proof surface from collateral drift even though this lane is workflow-only

## Workflow proof surfaces
The later coding lane must treat these as the authoritative proof surfaces for this slice:
- `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml`
- `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml`
- `git -C /home/ubuntu/workspace/repos/codex branch --list`
- `git -C /home/ubuntu/workspace/repos/codex branch -r`

Those surfaces, not prose alone, determine whether official publication truth stays honest.

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
- whether `allow-only-develop-into-main.yml` changed or remained untouched
- whether `publish-tester-release.yml` was created or updated
- exact verification commands and results
- the final evidence class: `branch-ready` or `pr-ready`
- whether any obligation had to remain stubbed/guarded because E1 output truth or GitHub runtime wiring was not yet fully grounded

Without that receipt, this lane must not be described as Claude-executed.
