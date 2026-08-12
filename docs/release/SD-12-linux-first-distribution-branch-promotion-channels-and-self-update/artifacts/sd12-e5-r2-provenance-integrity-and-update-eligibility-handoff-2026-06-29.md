---
title: SD12-E5-R2 Execution Handoff — Provenance, integrity, and update-eligibility gates
handoff_id: HANDOFF-CODEX-SD12-E5-R2-CODING-2026-06-29
stc_id: STC-CODEX-SD-12
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: blocked
status: draft
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e5-r2-provenance-integrity-and-update-eligibility-handoff-2026-06-29.md
source_stc: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md
source_epic_breakdown: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/epic-breakdown.md
readiness_closure: programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e5-r1-execution-readiness-closure-2026-06-29.md
boundary_truth_repair_task: t_b3cd10b7
selected_slice: SD12-E5 — release-unit-linked build identity, checksum/provenance publication, and Linux trust-gate surface
run_in: Claude Code only once unblocked
code_authority: false
created_at: 2026-06-29
authority_dependencies:
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/provenance-integrity-and-update-eligibility.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/self-update-transport-and-manifest-contract.md
  - programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e5-r1-execution-readiness-closure-2026-06-29.md
  - kanban:t_b3cd10b7
  - /home/ubuntu/workspace/repos/codex/AGENTS.md
  - /home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml
  - /home/ubuntu/workspace/repos/codex/apps/desktop/package.json
  - /home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs
  - /home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml
  - /home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts
  - /home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts
  - /home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json
  - /home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml
  - /home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/capabilities/default.json
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: develop
  expected_base_sha_at_creation: da66e2286ba0f8e0e5d9ba61386e79f3bfe236e5
  recommended_branch: sd12-e5-provenance-integrity-gates
  pr_target: develop
allowed_write_scope:
  - .github/workflows/publish-tester-release.yml
  - apps/desktop/package.json
  - apps/desktop/src-tauri/src/main.rs
forbidden_write_scope:
  - programs/codex/**
  - .github/workflows/allow-only-develop-into-main.yml
  - apps/desktop/src/**
  - apps/desktop/src-tauri/** outside src/main.rs
  - apps/desktop/src-tauri/Cargo.toml
  - apps/desktop/src-tauri/tauri.conf.json
  - apps/desktop/src-tauri/capabilities/default.json
  - apps/desktop/src-tauri/target/**
  - src/**
  - tests/**
  - README.md
  - AGENTS.md
  - CLAUDE.md
---

# SD12-E5-R2 Execution Handoff — Provenance, integrity, and update-eligibility gates

## Status
This is a stage-specific prebuild handoff, not a launch-authorizing code brief yet.

The blocker-repair lane restored truthful E5 repo/workflow surfaces and narrowed the write boundary to exactly three files. That part is complete. The reason this handoff remains `blocked` and carries `code_authority: false` is simpler: the repaired substrate is still local workspace residue on a dirty feature checkout rather than clean durable branch truth suitable for a governed Claude launch.

## Run in
Claude Code only once the unblock conditions below are satisfied.

Do not run this lane in Hermes as though it were already an executable code packet. Hermes authored the handoff. Claude Code should execute it only after the repaired E5 surfaces exist on a clean accepted launch base and the execution receipt can name the real branch/base/evidence truthfully. If Claude Code cannot be launched truthfully once unblocked, keep the lane blocked instead of substituting another harness.

## Core problem
The SD-12 contract now has a truthful bounded E5 home: one publication workflow surface, one desktop build-tooling surface, and one Tauri runtime truth surface. But the current repo state still presents those surfaces as local residue on `feat/sd11-enhancement-request-composer`, alongside unrelated dirty paths.

Current grounded state as of this handoff pass:
1. `.github/workflows/publish-tester-release.yml` now defines the first honest Linux tester publication candidate that emits bundle assets plus checksum, provenance/build receipt, and manifest outputs.
2. `apps/desktop/package.json` now exposes the bounded Linux build command `tauri:build:linux` required by that publication workflow.
3. `apps/desktop/src-tauri/src/main.rs` now exposes `load_sd12_release_truth`, a read-only runtime truth snapshot that keeps Linux automatic update blocked unless release-unit, checksum, provenance, manifest, and explicit trust-gate conditions are all present.
4. `apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts` and `apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` remain comparison surfaces only; E5 does not own the tester-facing desktop status/UI sync lane.
5. ignored local output under `apps/desktop/src-tauri/target/**` still exists and remains explicitly non-authoritative.
6. the repaired E5 surfaces are not yet clean durable branch truth:
   - `.github/workflows/publish-tester-release.yml` is untracked
   - `apps/desktop/package.json` is modified
   - `apps/desktop/src-tauri/src/main.rs` is modified
   - unrelated residue also exists in `apps/desktop/src-tauri/tests/ge08_workbench_integration.rs` and `apps/desktop/src-tauri/gen/`

That is the decisive constraint. The scope is finally bounded enough to hand off honestly, but not yet durably enough to authorize launch from the current checkout.

## Grounded path classification
State this plainly and do not improvise beyond it.

- publication / build-identity / provenance producer path: workflow-backed
  - candidate file: `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml`
  - grounded behavior seen in the file: branch-governed Linux publication from `develop` or `main`; release-unit metadata; checksum asset; provenance/build-receipt asset; machine-readable manifest asset; explicit `manual-only` update gate
  - current truth limit: the file is still untracked local workspace residue, not accepted pushed branch truth

- build/tooling bridge: package-script-backed
  - candidate file: `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json`
  - grounded behavior seen in the file: `npm run tauri:build:linux` exists and is the bounded Linux build entry used by the publication workflow
  - current truth limit: this remains a local modification only and does not authorize broader build-tooling churn

- runtime trust/update snapshot path: Tauri-command-backed
  - candidate file: `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`
  - grounded behavior seen in the file: `load_sd12_release_truth` exposes a read-only release-truth snapshot and refuses to classify Linux as automatic unless release-unit identity, checksum asset, provenance asset, manifest path, and explicit trust approval all exist
  - current truth limit: this is still a local modification and it is not the same thing as a dedicated desktop status/UI consumer surface

- current tester-facing desktop truth: still comparison-only and still not E5-owned
  - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts` still hard-codes `state: 'not-yet-supported'`
  - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` still presents SD-11 workbench truth without consuming the new E5 runtime snapshot

Therefore this lane is a bounded producer/runtime-truth substrate only. It is not the UI synchronization lane, it is not the manifest consumer lane, and it is not the rollback/withdrawal lane.

## Objective once unblocked
Implement the smallest honest E5 completion surface for SD-12 without widening into SD-11 status/UI sync, E3 manifest-consumer work, E4 rollback/withdrawal work, or cross-platform trust claims.

The eventual result must preserve all of the following:
1. every official Linux tester release unit remains machine-linkable to build label, version, source revision, operator promotion state, tester-facing channel, support tier, and publication event
2. every official Linux tester release unit continues to publish checksum, provenance/build-receipt, and manifest outputs from the governed workflow surface
3. the runtime-facing trust snapshot continues to refuse counterfeit automatic-update claims when any required gate is absent
4. ignored local residue under `apps/desktop/src-tauri/target/**` never counts as publication truth
5. no repo code widens into SD-11 presentation scope, rollback/recovery scope, or macOS/Windows trust posture

## Why this handoff is blocked instead of live
This handoff intentionally stops short of code authority because the repaired substrate is not yet on a clean accepted launch base.

Exact remaining unblock conditions:
1. `.github/workflows/publish-tester-release.yml`, `apps/desktop/package.json`, and `apps/desktop/src-tauri/src/main.rs` must stop being local residue on a dirty shared feature checkout and become accepted durable branch truth on a clean branch or equivalent governed receipt surface
2. the launch base must exclude unrelated dirty paths such as `apps/desktop/src-tauri/tests/ge08_workbench_integration.rs` and `apps/desktop/src-tauri/gen/`
3. the lane must remain inside the exact three-file write surface above; if truthful work now requires `.github/workflows/allow-only-develop-into-main.yml`, `apps/desktop/src/**`, `Cargo.toml`, `tauri.conf.json`, capabilities, or broader Tauri files, another readiness/truth pass is required before launch

Until those are true, launching a coding harness would force it to inherit a dirty checkout and counterfeit execution provenance.

## Branch policy
Do not launch this lane from the currently checked-out local branch merely because it contains the repaired surfaces.

Observed local repo truth during this handoff pass:

```text
current local branch: feat/sd11-enhancement-request-composer
current local head:   8f3a627655f490551ff23746293cde1622085e97
origin/develop:       da66e2286ba0f8e0e5d9ba61386e79f3bfe236e5
workspace residue:    ?? .github/workflows/publish-tester-release.yml
workspace residue:     M apps/desktop/package.json
workspace residue:     M apps/desktop/src-tauri/src/main.rs
workspace residue:     M apps/desktop/src-tauri/tests/ge08_workbench_integration.rs
workspace residue:    ?? apps/desktop/src-tauri/gen/
```

That checkout is not a truthful execution base for this lane.

When the lane is unblocked, launch from clean durable branch truth instead:

```bash
git fetch origin --prune
git switch -c sd12-e5-provenance-integrity-gates origin/develop
```

If the accepted E5 substrate lands on a different pushed base first, record that exact base SHA and stack policy explicitly in the execution receipt. Do not inherit a dirty shared checkout by assumption.

## Exact allowed write scope
You may create or modify only these paths:

```text
/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml
/home/ubuntu/workspace/repos/codex/apps/desktop/package.json
/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs
```

Write-scope interpretation:
- `publish-tester-release.yml` is the only allowed publication/provenance producer surface
- `package.json` is writable only for the bounded Linux build-tooling bridge already established by the repaired truth
- `src-tauri/src/main.rs` is the only allowed runtime truth/trust-gate surface in this lane
- if truthful implementation now requires any TypeScript consumer surface, any additional Rust file, any config/dependency file, or any other workflow file, stop and route back through readiness instead of widening locally

## Required reads before coding
Read these first, in order:
1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. this handoff file
3. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e5-r1-execution-readiness-closure-2026-06-29.md`
4. the completion handoff and metadata from kanban task `t_b3cd10b7`
5. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/provenance-integrity-and-update-eligibility.md`
6. `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/self-update-transport-and-manifest-contract.md`
7. `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml`
8. `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json`
9. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`
10. `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml` as read-only promotion-truth context only
11. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts` as read-only SD-11 status-authority context only
12. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` as read-only SD-11 consumer-boundary context only
13. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json` as read-only packaging/trust-boundary context only
14. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml` as read-only dependency-boundary context only
15. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/capabilities/default.json` as read-only capability-boundary context only

## Conditional reads
Read these only if the corresponding condition actually occurs:
1. any branch-ready receipt or PR evidence that makes the three repaired E5 files durable branch truth
   - only when you need the accepted base SHA, branch, or merge stack truth rather than local workspace evidence
2. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/target/**`
   - only to confirm ignored local residue remains non-authoritative; never as a publication authority surface
3. any new tests added strictly inside the allowed write surface in a later repaired brief
   - only if a future readiness pass explicitly expands the allowed write scope to include them

## Exact non-goals
This handoff does not authorize:
- widening into `apps/desktop/src/**`, `App.tsx`, or any SD-11 tester-facing status/UI wording work
- widening into manifest-consumer or desktop update-client work that belongs to SD12-E3
- widening into rollback, withdrawal, downgrade, replacement-release, or recovery-preference behavior that belongs to SD12-E4
- widening into macOS signing/notarization or Windows trust posture
- rewriting `allow-only-develop-into-main.yml` or broader promotion doctrine
- dependency/config changes in `Cargo.toml`, `tauri.conf.json`, or `capabilities/default.json`
- treating ignored local build residue under `apps/desktop/src-tauri/target/**` as official checksum, provenance, or release truth
- classifying Linux as `automatic` unless every gate defined in the runtime truth surface and documentary contract is explicitly satisfied
- PR, merge, or release-publication claims from this documentary handoff itself

## Forbidden widening / stop conditions
Stop and report the blocker if any of these become true:
1. truthful implementation requires changes outside the exact three allowed files
2. truthful Linux trust-gate behavior now requires `Cargo.toml`, `tauri.conf.json`, capability edits, or broader Rust module changes
3. truthful completion requires a TypeScript consumer/status surface rather than staying inside the bounded workflow/package/main.rs substrate
4. the only available execution base is still the dirty `feat/sd11-enhancement-request-composer` checkout or another dirty shared branch
5. the lane would need to claim automatic-update success, cross-platform trust parity, or rollback/recovery truth without the dedicated lanes that own those surfaces
6. the workflow/package/runtime surfaces stop agreeing on build label, release-unit identity, checksum/provenance asset naming, manifest naming, or manual-only gate semantics

If a stop condition lands, do not improvise. Return a blocker naming the exact broader surface now required.

## Verification commands
Run these at minimum when the lane actually launches:

```bash
git -C /home/ubuntu/workspace/repos/codex status --short
git -C /home/ubuntu/workspace/repos/codex branch --list
git -C /home/ubuntu/workspace/repos/codex branch -r
git -C /home/ubuntu/workspace/repos/codex status --short -- \
  .github/workflows/publish-tester-release.yml \
  apps/desktop/package.json \
  apps/desktop/src-tauri/src/main.rs
git -C /home/ubuntu/workspace/repos/codex diff --name-only -- \
  .github/workflows/publish-tester-release.yml \
  apps/desktop/package.json \
  apps/desktop/src-tauri/src/main.rs
git -C /home/ubuntu/workspace/repos/codex diff --unified=0 -- \
  .github/workflows/publish-tester-release.yml \
  apps/desktop/package.json \
  apps/desktop/src-tauri/src/main.rs
git -C /home/ubuntu/workspace/repos/codex status --short --ignored -- apps/desktop/src-tauri/target
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run typecheck
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run build
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run tauri:check
cd /home/ubuntu/workspace/repos/codex/apps/desktop && cargo test --manifest-path src-tauri/Cargo.toml linux_release_truth -- --nocapture
```

## Verification interpretation
- `git status --short` must prove whether the launch base is clean durable truth or still contaminated by unrelated residue
- `branch --list` and `branch -r` confirm the accepted launch base and preserve the repaired `develop`/`main` promotion truth without inventing `uat`
- path-scoped `status` and `diff --name-only` must prove that only the exact E5 surfaces changed
- `diff --unified=0` is the review surface for confirming that no hidden SD-11 UI sync, rollback behavior, cross-platform trust claims, or extra workflow/config widening was smuggled in
- the ignored-target check prevents counterfeit completion by mistaking local generated bundle output for publication truth
- `typecheck`, `build`, and `tauri:check` preserve the desktop proof surface while the bounded E5 substrate is finalized
- the targeted `linux_release_truth` cargo tests prove the runtime trust-gate logic still classifies Linux as `manual-only` when metadata is missing and only reports `automatic` when every gate is explicitly satisfied

## Release-surface truth this handoff depends on
The later coding lane must treat these as the authoritative proof surfaces for this slice:
- `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`
- `git -C /home/ubuntu/workspace/repos/codex branch --list`
- `git -C /home/ubuntu/workspace/repos/codex branch -r`
- `git -C /home/ubuntu/workspace/repos/codex status --short --ignored -- apps/desktop/src-tauri/target`

These surfaces, not prose alone, determine whether build identity, checksum/provenance publication, and Linux trust-gate truth remain honest.

## Merge authority boundary
This handoff does not authorize merging.

- Do not merge the branch or PR.
- Do not land code directly onto `develop` or `main`.
- Stop at verified branch-ready or PR-ready state and hand control back to Todd.

## Final report requirements
When this handoff eventually becomes live and is executed, the final receipt must include:
- exact handoff path
- actual branch name
- actual base SHA
- files changed
- whether `publish-tester-release.yml` remained the sole producer surface
- whether `package.json` changed and why
- whether `src-tauri/src/main.rs` changed and why
- whether Linux automatic update remained `manual-only` or what exact evidence satisfied every gate
- exact verification commands and actual results
- the final evidence class: `branch-ready` or `pr-ready`
- any remaining reasons macOS and Windows still remain non-automatic in this tranche

Without that receipt, this lane must not be described as frontier-harness executed.
