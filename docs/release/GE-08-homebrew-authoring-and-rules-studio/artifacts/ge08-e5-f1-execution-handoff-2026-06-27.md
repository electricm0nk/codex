---
title: GE08-E5-F1 Execution Handoff — Product-Visible Desktop Workbench Proof for First Homebrew Package
handoff_id: HANDOFF-CODEX-GE-08-E5-F1-CODING-2026-06-27
stc_id: STC-CODEX-GE-08
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: awaiting-todd-launch
owner: Todd Hintzmann
scope: program
canonical: false
canonical_path: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/execution-handoff.md
source_stc: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/README.md
readiness_closure: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e5-r2-desktop-authoring-boundary-and-native-verification-closure-2026-06-27.md
selected_slice: GE08-E5-F1 — Product-visible workbench proof for the first homebrew package
run_in: Claude Code or equivalent frontier coding harness
code_authority: true
created_at: 2026-06-27
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: ge08-e4-f1-preview-and-explanation-bridge
  expected_base_sha_at_creation: 9c883dc1ee3e8a7de35ecea9cf84bd6c9611cb1f
  recommended_branch: ge08-e5-f1-desktop-authoring-workbench
  pr_target: develop
allowed_write_scope:
  - apps/desktop/src/App.tsx
  - apps/desktop/src/boundary/**
  - apps/desktop/src-tauri/src/**
  - apps/desktop/src-tauri/Cargo.toml
  - apps/desktop/src-tauri/Cargo.lock
  - apps/desktop/src-tauri/tauri.conf.json
  - apps/desktop/src-tauri/icons/**
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen/**
  - programs/codex/**
  - src/homebrew_authoring/**
  - src/lib.rs
  - src/oracle_validation/**
  - src/pcgen_import/**
  - src/rules_core/**
  - apps/desktop/package.json
  - apps/desktop/package-lock.json
  - Cargo.toml
  - Cargo.lock
  - AGENTS.md
  - CLAUDE.md
---

# GE08-E5-F1 Execution Handoff — Product-Visible Desktop Workbench Proof for First Homebrew Package

## Status
This is the live stage-specific code-authorizing brief for GE08-E5-F1.

It carries `code_authority: true` for GE08-E5-F1 only and is currently `awaiting-todd-launch`.

## Run in
Claude Code or an equivalent frontier coding harness.

Do not run this in Hermes as a documentary card. Hermes produced this handoff; the coding harness implements it.

## Core problem
GE08-E4-F1 proved the headless package, validation, preview, explanation, provenance, and lifecycle substrate. The desktop shell still does not consume that truth. It only exposes a `load_pilot_shell_snapshot` placeholder seam and therefore cannot honestly demonstrate GE08 product-visible authoring behavior.

The next honest move is narrower than “build the rules studio.” Create a bounded internal-pilot desktop workbench proof that loads the real first-proof package through Tauri, surfaces the real GE08 headless result families, and keeps blocked-path truth visible instead of inventing UI-local semantics.

## Objective
Create the smallest truthful desktop workbench proof over the existing GE08 headless substrate.

The result must prove:
1. the shell can load the bounded guard-stance source package from the stacked GE08-E4 base
2. the shell can surface package state, diagnostics, preview status, selected-slot resolution, provenance refs, explanation refs, and lifecycle gate state without reinterpretation
3. a valid package yields the bounded success path
4. malformed or widened package variants yield blocked/unsupported shell truth instead of counterfeit success
5. the workbench remains a consumer of GE08/GE07 authority, not a second rules engine or a hidden rewrite of `src/homebrew_authoring/**`

## Branch policy
Do not reset this lane to `origin/develop`. The truthful base is still the stacked GE08-E4 work because the desktop lane must consume the live headless substrate before that branch is merged.

The default execution surface for this lane is a feature branch in the canonical repo:

```text
repo root: /home/ubuntu/workspace/repos/codex
branch: ge08-e5-f1-desktop-authoring-workbench
base branch: ge08-e4-f1-preview-and-explanation-bridge
base commit: 9c883dc1ee3e8a7de35ecea9cf84bd6c9611cb1f
```

If you need to recreate the branch in the canonical repo, the equivalent setup is:

```bash
git fetch origin --prune
git switch ge08-e4-f1-preview-and-explanation-bridge
git pull --ff-only origin ge08-e4-f1-preview-and-explanation-bridge
git switch -c ge08-e5-f1-desktop-authoring-workbench
```

Use a worktree only if Todd explicitly wants parallel isolated checkout behavior or the canonical repo cannot be kept clean. If that exception occurs, justify it in the final receipt.

Record the actual launch repo root, branch, and base SHA in the final receipt.

## Required reads before coding
Read these first, in order:
1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
3. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e5-r2-desktop-authoring-boundary-and-native-verification-closure-2026-06-27.md`
4. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/execution-handoff.md`
5. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/rules-studio-surface-definition.md`
6. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/validation-and-preview-workflow-requirements.md`
7. `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e4-f1-execution-handoff-2026-06-27.md`
8. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
9. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadPilotShellSnapshot.ts`
10. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`
11. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml`
12. `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/mod.rs`
13. `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/package_manifest.rs`
14. `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/package_store.rs`
15. `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/preview_bridge.rs`
16. `/home/ubuntu/workspace/repos/codex/tests/ge08_preview_bridge.rs`

## Contract to implement
Do not invent a new semantic surface. Implement the contract named by the R2 closure.

### Tauri command
Introduce a truthful command boundary named:

```text
load_ge08_authoring_workbench_snapshot
```

### Request
```ts
type Ge08AuthoringWorkbenchRequest = {
  packageRoot: string;
  activeRecordRef?: string | null;
};
```

Required request behavior:
- treat `packageRoot` as repo-root-relative in this lane
- use the deterministic first-proof package root `tests/fixtures/ge08/guard-stance-package` for the success path
- use the invalid/widened fixture roots for blocked-path proof
- refuse unknown roots instead of inventing placeholder data

### Response obligations
Return a shell-consumer snapshot that surfaces, at minimum:
- `packageState`
- manifest identity/version/dependency truth
- current authored feat/effect/prerequisite records
- `previewStatus`
- `selectedSlotResolution`
- `baselineArmorClass` computed or blocked marker
- diagnostics with claim-blocking posture preserved
- provenance refs
- explanation refs
- lifecycle gate state (`saveAllowed`, `previewAllowed`, `exportAllowed`, `diffMode`)
- explicit `dataSource` / `note` truth when Tauri is unavailable

This desktop route must be a projection over `PackageStore` + `PreviewBridge`, not a second semantic engine.

## Preflight verification truth
Hermes already repaired the host-level Tauri prerequisites. The current remaining native failure is repo-local:

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop
npm run tauri:check
```

Current failure:
- `apps/desktop/src-tauri/icons/icon.png` missing during `tauri::generate_context!()`

Treat that as a bounded lane-local prerequisite inside the allowed write scope. Repair it first and record the preflight failure in the final report. Do not treat host setup as the blocker anymore.

## Required RED -> GREEN execution pattern
TDD is mandatory here.

### Preflight fix before RED
If `npm run tauri:check` still fails on the missing icon/config asset before your test can even compile, repair that exact preflight issue first inside the allowed write scope and record it separately as environment-to-repo unblocking, not as the business change.

### RED first
Before changing the desktop boundary behavior, add a focused failing Rust test surface under `apps/desktop/src-tauri/src/**` that proves the current shell adapter still lacks the truthful GE08 workbench contract.

At minimum, the failing assertions must cover:
- valid guard-stance package -> `packageState: valid`, `previewStatus: success`, real selected-slot resolution, non-empty provenance refs, non-empty explanation refs
- missing-effect or widened fixture -> blocked/unsupported status with diagnostics and blocked claims preserved
- lifecycle gate state refuses export/preview when the underlying package is invalid/deferred/unsupported
- non-Tauri or command-failure fallback does not masquerade as successful GE08 product truth

Capture the RED result in the final report.

### GREEN second
After the intended RED failure, make the smallest change necessary inside the allowed write scope to satisfy the desktop contract.

## Allowed write scope
You may write only:

```text
apps/desktop/src/App.tsx
apps/desktop/src/boundary/**
apps/desktop/src-tauri/src/**
apps/desktop/src-tauri/Cargo.toml
apps/desktop/src-tauri/Cargo.lock
apps/desktop/src-tauri/tauri.conf.json
apps/desktop/src-tauri/icons/**
```

If you need any other file, stop and report the blocker.

## Required implementation constraints
- consume the existing GE08 headless substrate; do not modify `src/homebrew_authoring/**`
- keep the proof bounded to the first package and its invalid/widened variants
- keep blocked-path truth visible; do not blank the shell because preview failed
- do not widen into save implementation, import flows, plugin runtime, or general authoring UX
- do not invent local rules computations in React or Tauri glue code
- if you need a path dependency from `apps/desktop/src-tauri` to the root `codex` crate, keep it minimal and justified in the final receipt

## Forbidden widening
This handoff does not authorize:
- edits under `src/homebrew_authoring/**`, `src/rules_core/**`, `src/oracle_validation/**`, or `src/pcgen_import/**`
- general editor breadth beyond the single proof package surface
- NPM dependency changes or frontend test-harness introduction unless a blocker proves the packet incomplete
- any write under `/home/ubuntu/workspace/repos/pcgen`
- merge, rebase, or landing work onto `develop` or `main`

## Verification commands
Run at minimum:

```bash
export PATH=/home/ubuntu/.cargo/bin:$PATH
cd /home/ubuntu/workspace/repos/codex/apps/desktop
npm run typecheck
npm run build
npm run tauri:check
cargo test --manifest-path src-tauri/Cargo.toml -- --nocapture
```

If you narrow `cargo test` to a named test module during RED/GREEN iteration, still end with a full `cargo test --manifest-path src-tauri/Cargo.toml -- --nocapture` before reporting done.

## Merge authority boundary
This handoff does not authorize the coding harness to merge anything.

- Do not merge the branch or PR.
- Do not land code directly onto `develop` or `main`.
- Stop at verified branch-ready or PR-ready state and hand control back to Todd.

## Final report requirements
The final execution receipt must include:
- exact handoff path
- exact worktree path used
- actual branch name
- actual base SHA
- files changed
- preflight native-fix summary if the icon/config issue was repaired
- RED failure summary
- final verification commands and results
- whether the lane stopped at branch-ready or PR-ready

Without that receipt, this lane must not be described as Claude-executed.
