---
title: SD12-E1-F1 Execution Handoff — First Linux artifact set
stc_id: STC-CODEX-SD-12
artifact_type: execution-handoff
stc_kind: execution-handoff
template_version: 1
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: active
owner: Todd Hintzmann
scope: repo
code_authority: true
source_stc: ../README.md
source_readiness_closure: ./sd12-e1-r1-execution-readiness-closure-2026-06-29.md
selected_slice: SD12-E1-F1 — First Linux artifact set
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex/apps/desktop
  base_branch: develop
  execution_branch: feat/sd12-e1-f1-linux-artifact-set
  write_scope:
    - apps/desktop/src-tauri/tauri.conf.json
    - apps/desktop/package.json
    - apps/desktop/src-tauri/Cargo.toml
reviewed_at: 2026-06-29
---

# SD12-E1-F1 Execution Handoff — First Linux artifact set

## Deliverable Type
`implementation-ready`

## Execution Readiness
`codex-ready`

## Exact objective
Implement the smallest honest Linux packaging slice that turns the current desktop shell from build-only proof into a bounded Linux artifact emitter without widening into publication, updater, rollback, provenance, or UI work.

This slice selects the first Linux artifact set explicitly as:
- install-oriented artifact path: `.deb`
- recovery/manual artifact path: `AppImage`

Why this is the correct minimum slice:
- the SD-12 matrix requires one install-oriented Linux path and one recovery/manual path
- the current Tauri CLI in this repo exposes Linux bundle targets `deb`, `rpm`, and `appimage`
- `rpm` would widen the first slice without adding required truth for the Linux-first tester program
- live verification on 2026-06-29 showed `npx tauri build --bundles deb,appimage --ci` already emits a real `.deb` bundle, while the `AppImage` leg currently fails with `couldn't find a square icon to use as AppImage icon`
- that failure is still honest for this bounded slice because the repo already contains `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/icons/icon.png`, and the observed asset is square (`512x512`); the missing truth is configuration inside the allowed `tauri.conf.json` surface, not a broader publication or UI dependency

The goal is therefore not “all Linux packaging.” The goal is this one bounded artifact set, and nothing beyond it.

## Target repo / workdir

```text
repo:    /home/ubuntu/workspace/repos/codex
workdir: /home/ubuntu/workspace/repos/codex/apps/desktop
```

Current grounded repo facts:
- the local shell that grounded readiness was on `feat/sd11-enhancement-request-composer` at `8f3a627655f490551ff23746293cde1622085e97`
- `git fetch origin --prune` on 2026-06-29 showed that branch has already been merged into `origin/develop`, which is now at `da66e22`
- `git diff --name-only origin/develop...HEAD` returned empty, so the current target surfaces are already represented in merged `origin/develop`
- `npm run typecheck`, `npm run build`, and `npm run tauri:check` all passed again on 2026-06-29 from `apps/desktop`
- `npx tauri build --bundles deb,appimage --ci` is a real command for this repo/toolchain and is the bounded artifact-emission verification step for this slice
- that live build emitted `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/target/release/bundle/deb/Codex Desktop Shell Scaffold_0.0.0_amd64.deb`
- the same live build then aborted with `couldn't find a square icon to use as AppImage icon`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/icons/icon.png` already exists and was verified as square (`512x512`), which keeps the required repair inside the bounded configuration surface rather than forcing a wider asset-creation lane

`AGENTS.md` is the repo-root conduct surface. Follow it.

## Branch / launch policy
Before implementation:

```bash
git fetch origin --prune
git switch develop --track origin/develop 2>/dev/null || git switch develop
git pull --ff-only origin develop
git switch -c feat/sd12-e1-f1-linux-artifact-set
```

Do not continue implementation on the stale local `feat/sd11-enhancement-request-composer` branch.

If `feat/sd12-e1-f1-linux-artifact-set` already exists, use it only after confirming it still belongs exclusively to this slice.

## Exact allowed write scope
You may create or modify only these paths:

```text
/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json
/home/ubuntu/workspace/repos/codex/apps/desktop/package.json
/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml
```

Write-scope interpretation:
- `src-tauri/tauri.conf.json` is the primary authority surface for enabling the Linux bundle set and declaring the package-output posture
- `package.json` may be changed only for narrowly required script or package-surface adjustments that make the bounded Linux verification path repeatable
- `src-tauri/Cargo.toml` may be changed only if Tauri-side package metadata must be aligned to emit the bounded Linux artifact set honestly

Do not write outside `/home/ubuntu/workspace/repos/codex`.

## Exact required implementation shape
The slice must do only what is needed to make the first Linux artifact set real inside the allowed write scope.

Required outcome:
1. enable the bounded Linux packaging surface
2. configure the first Linux artifact set as `.deb` plus `AppImage`
3. keep Linux first-class while leaving macOS and Windows untouched except for preserving their already accepted posture
4. preserve the existing truth that update checks are not yet wired in this slice

Acceptable implementation moves inside the allowed write scope:
- set `bundle.active` truthfully for this slice
- declare Linux bundle targets for `.deb` and `AppImage`
- wire the existing square icon asset at `src-tauri/icons/icon.png` into the Tauri bundle configuration if needed for AppImage emission
- add only the minimum metadata/script changes needed to make the bundle command reproducible and internally consistent

If the current placeholder metadata (`Codex Desktop Shell Scaffold`, `0.0.0`, current identifier) blocks honest bounded artifact emission, you may make the smallest aligned metadata update inside the allowed files only. Do not turn this into a branding or versioning overhaul.

## Explicitly forbidden scope
Do not implement or modify:

```text
/home/ubuntu/workspace/repos/codex/.github/**
/home/ubuntu/workspace/repos/codex/apps/desktop/src/**
/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/**
/home/ubuntu/workspace/repos/codex/src/**
programs/codex/**
```

Also forbidden in this slice:
- GitHub release or prerelease publication work
- manifest generation or desktop manifest-consumer work
- rollback, withdrawal, downgrade, or provenance/integrity implementation
- SD-11 UI wording, issue-payload, or status-surface synchronization changes
- macOS or Windows parity expansion
- updater-library selection or updater enablement
- merge, branch-promotion, or release-hosting actions

This handoff authorizes code only inside the bounded write surface above.

## Required reads before coding
Read these first:
1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e1-r1-execution-readiness-closure-2026-06-29.md`
3. `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md`
4. `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/epic-breakdown.md`
5. `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/distribution-platform-support-and-channel-matrix.md`
6. `/home/ubuntu/workspace/repos/codex/README.md`
7. `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json`
8. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json`
9. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml`
10. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/icons/icon.png`
11. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
12. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
13. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
14. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`

Use the SD-11 files as read-only truth-preservation surfaces, not as permission to edit them.

## Upstream evidence this slice must preserve
- Linux remains the first-class platform in this tranche
- macOS remains second-class but real, and Windows remains explicitly third-class
- tester-facing channel truth still layers `alpha`, `beta`, and `stable` over the operator path `develop -> uat -> main`
- the desktop UI remains a consumer of release truth, not its source
- “update checks not yet wired in this slice” remains true after this work; packaging is not updater enablement
- `.github/**`, manifest generation, rollback, and provenance lanes belong to later SD12-E2 through SD12-E5 slices

## Acceptance criteria
This slice is complete only when all of the following are true:
1. the repo still passes the existing desktop verification surface
2. the bounded Linux artifact set is explicitly configured as `.deb` plus `AppImage`
3. the artifact-emission verification run produces the Linux bundle outputs or a truthful blocker explaining why one of the two bounded outputs cannot be emitted from the allowed scope
4. no files outside the allowed write scope are modified
5. no UI/status/update-language source file is changed
6. no GitHub publication, manifest, rollback, or provenance behavior is implemented or implied

## Verification commands
Run from `/home/ubuntu/workspace/repos/codex/apps/desktop`:

```bash
npm run typecheck
npm run build
npm run tauri:check
npx tauri build --bundles deb,appimage --ci
find src-tauri/target/release/bundle -maxdepth 2 -type f \( -name '*.deb' -o -name '*.AppImage' \) | sort
```

Verification interpretation:
- `typecheck`, `build`, and `tauri:check` preserve the existing desktop proof surface
- `tauri build --bundles deb,appimage --ci` is the bounded artifact-emission proof for this slice
- the final `find` output must show the Linux bundle files actually emitted by the build

## Stop conditions
Stop and report without widening scope if any of these occur:
- the bounded Linux artifact set cannot be implemented inside `tauri.conf.json`, `package.json`, and `src-tauri/Cargo.toml`
- a truthful implementation would require `.github/**`, manifest, updater, rollback, provenance, or UI/status-file writes
- the build emits only one of the two required Linux artifact roles and the missing role cannot be recovered without broadening scope
- the repo cannot be refreshed to a clean `origin/develop`-based execution branch
- the required verification commands fail after the bounded config changes

If any stop condition lands, do not improvise. Return a truthful blocker that names the exact broader surface now required.

## Merge / launch authority boundary
This handoff is code-authorizing for the bounded repo slice only.

It does not authorize:
- merge to `develop`, `uat`, or `main`
- GitHub release creation
- asset publication
- updater enablement
- manifest generation
- provenance/checksum policy completion
- any write outside the bounded three-file surface

For the governed board, the later execution lane must run through Claude Code and leave a durable `claude-execution-receipt` before closeout.
