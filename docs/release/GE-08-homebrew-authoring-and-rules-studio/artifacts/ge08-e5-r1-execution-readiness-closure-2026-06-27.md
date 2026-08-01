---
title: GE08-E5-R1 Execution Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-08
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE08-E5-R1 — Product-visible authoring route readiness closure
workflow_route: readiness-closure
readiness: blocked
handoff_created: false
review_date: 2026-06-27
owner: Todd Hintzmann
scope: program
code_authority: false
related_artifacts:
  - ./rules-studio-surface-definition.md
  - ./validation-and-preview-workflow-requirements.md
  - ./homebrew-authoring-surface-specification.md
  - ./ge08-e4-f1-branch-ready-receipt-2026-06-27.md
  - ./ge08-e4-f1-pr-created-receipt-2026-06-27.md
  - ../../GE-07-desktop-shell-and-modern-ux/README.md
  - ../../GE-07-desktop-shell-and-modern-ux/execution-handoff.md
---

# GE08-E5-R1 Execution Readiness Closure

## Verdict
GE08-E5 is **not yet ready** to mint a code-authorizing GE08-E5-F1 product-visible workbench handoff.

The lesser models would call this ready because the desktop scaffold exists and the GE08 headless substrate now exists on a real branch. That is noise. The missing truth is narrower and more dangerous:

1. the live GE-07 shell boundary still exposes only placeholder pilot snapshot commands rather than a GE08 authoring/validation/preview/explanation/provenance command family
2. this host cannot currently verify a Tauri-side boundary change, because native desktop Rust checks fail before the shell command layer can be exercised

That means the route is blocked by **command-boundary truth and native verification**, not by missing GE-07 architecture.

## What changed from the earlier stale blocker story
The older GE07-E5 closure is no longer an accurate blocker explanation for GE08-E5. Live repo truth now shows:

- `origin/develop` already contains the GE-07 desktop scaffold under `apps/desktop/**`
- `origin/develop` already contains the GE-06 pilot view-model bridge under `src/rules_core/pilot_view_model.rs` and `tests/ge06_pilot_view_model.rs`
- PR `#20` now exposes the GE08 E2/E3/E4 headless authoring substrate on top of that merged base

So the route is no longer blocked by absent shell scaffolding or absent GE06 UI-consumer bridge.

## Live evidence recovered this pass
| Gate | Status | Evidence |
|---|---|---|
| GE-07 shell scaffold exists on durable base | pass | `origin/develop@cc4e1a55caad07af83768a036d4b0f5fffbf99c9` contains `apps/desktop/package.json`, `apps/desktop/src/main.tsx`, `apps/desktop/src/App.tsx`, and `apps/desktop/src-tauri/Cargo.toml`. |
| GE-06 pilot view-model bridge exists on durable base | pass | `origin/develop` contains `src/rules_core/pilot_view_model.rs` and `tests/ge06_pilot_view_model.rs`. |
| GE08 headless authoring substrate exists on a real reviewable branch | pass | PR `#20` (`ge08-e4-f1-preview-and-explanation-bridge`) contains `src/homebrew_authoring/mod.rs`, `package_manifest.rs`, `package_store.rs`, `preview_bridge.rs`, deterministic GE08 fixtures, and bounded GE08 tests. |
| GE08 stacked branch is reviewable and mergeable | pass | GitHub reports PR `#20` as `OPEN`, non-draft, `MERGEABLE`. |
| GE08 headless proof still verifies at the PR head | pass | Detached worktree test rerun at `origin/ge08-e4-f1-preview-and-explanation-bridge@9c883dc1ee3e8a7de35ecea9cf84bd6c9611cb1f` passed `cargo test ge08_package_file_lifecycle`, `cargo test ge08_validation_and_diagnostics`, and `cargo test ge08_preview_bridge`. |
| Desktop frontend build exists | pass | `apps/desktop/package.json` defines `typecheck`, `build`, and `tauri:check`; `npm run build` succeeded on this host. |
| Real GE08 workbench command boundary exists | fail | `apps/desktop/src/boundary/loadPilotShellSnapshot.ts` still returns a scaffold placeholder outside Tauri and only invokes `load_pilot_shell_snapshot`; `apps/desktop/src-tauri/src/main.rs` still returns a hard-coded placeholder `PilotShellSnapshot`. No GE08 authoring command family is present. |
| GE08 payload families are consumable through the shell without reinterpretation | fail | The required GE08 families (`package_state`, diagnostics with blocked-claim posture, preview status, selected-slot resolution, provenance refs, explanation refs, lifecycle gate state) are not yet exposed through the desktop boundary. |
| Native desktop boundary verification runs on this host | fail | `cargo check` in `apps/desktop/src-tauri` failed with `glib-sys` build failure because `pkg-config` is unavailable, so a Tauri-side boundary change cannot currently be verified here. |
| Exact GE08-E5-F1 write scope is honest and executable | fail | Without a real authoring command contract and native boundary verification, any exact E5-F1 file list would smuggle ungrounded payload/schema decisions into a false code-authorizing packet. |
| Success-path and blocked-path verification contract for a product-visible slice exists | fail | The spec requires both, but a real product-visible blocked-path command/result loop cannot yet be verified because the desktop boundary still speaks only placeholder pilot-snapshot truth. |

## Core problem
The first product-visible GE08 slice is only honest if it can let the user work with the bounded first-proof package while consuming the real GE08 headless truth families through the GE07 shell boundary.

The relevant specs require that the workbench support this loop:
1. select the first-proof package and authored object
2. expose package state
3. edit bounded manifest/object/effect/prerequisite surfaces
4. run validation with machine-readable blocked claims
5. run preview only when valid
6. inspect explanation and provenance without losing package context
7. save or diff honestly

The repo now has enough headless substrate to support that route in principle. It still does **not** have the shell boundary that makes the route executable truth instead of paper truth.

## Why the route is blocked now
### 1. The live shell boundary is still a placeholder seam
The only current desktop boundary is:
- TypeScript: `apps/desktop/src/boundary/loadPilotShellSnapshot.ts`
- Tauri command: `load_pilot_shell_snapshot` in `apps/desktop/src-tauri/src/main.rs`

That seam returns scaffold placeholder data for GE07-E1. It does not expose:
- package identity and package state
- authored record inventory
- validation diagnostics with blocked claims
- preview status (`success | blocked | unsupported`)
- selected-slot resolution for the Human bonus feat substitution
- provenance refs
- explanation refs
- lifecycle gate state for save/diff/export

A product-visible GE08 handoff that guessed those payloads now would counterfeit exactly the semantic honesty these specs were created to protect.

### 2. The host cannot verify Tauri-side command-boundary changes yet
The desktop Rust layer is not merely unimplemented for GE08. It is currently unverifiable on this machine:

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri
cargo check
```

Observed failure:
- exit `101`
- `glib-sys` build script failed because `pkg-config` was not found in PATH

Until that native verification path is repaired or an equivalent verified environment is designated explicitly, a code-authorizing GE08-E5-F1 handoff would be forced to rely on unverified shell-boundary changes.

## What can be named honestly now
The likely boundary-holding path classes are now visible, but they are **not yet code-authorized**:

```text
apps/desktop/src/App.tsx
apps/desktop/src/boundary/loadPilotShellSnapshot.ts
apps/desktop/src-tauri/src/main.rs
apps/desktop/src-tauri/Cargo.toml
src/homebrew_authoring/**
src/lib.rs
```

Interpretation:
- the desktop paths are the probable shell and Tauri seam for the first product-visible lane
- `src/homebrew_authoring/**` is the existing headless substrate the shell must consume rather than redefine
- this is still a boundary map, not an authorized write list, because the request/response contract and native verification surface are not yet grounded enough

## What cannot be named honestly yet
An exact GE08-E5-F1 code packet with fixed writable files, exact payload schema, and a runnable verification contract cannot yet be named truthfully.

Why not:
- the only live desktop command seam is still a GE07 scaffold placeholder
- the required GE08 payload families are not yet surfaced through that seam
- the Tauri-native check required to verify that seam fails on this host before GE08 boundary work can be proven

## Shortest honest next move
Create one more route-closure step before any E5 code handoff:

```text
GE08-E5-R2 FLOW: Desktop authoring boundary and native verification closure
```

Its job is to do exactly two things:
1. ground the first truthful GE08 desktop command/request-response contract over the existing headless substrate
2. repair or explicitly designate the native verification environment needed to run the Tauri-side check for that contract

Only after that closure succeeds should a stage-specific GE08-E5-F1 code-authorizing handoff be minted.

## Completion rule
This readiness closure is complete because it does the honest work this route required today:
- invalidates the stale “missing GE07 scaffold” blocker with live repo truth
- proves the GE08 headless substrate now exists on a real mergeable PR branch
- identifies the actual remaining blocker as command-boundary truth plus native verification
- refuses to mint counterfeit code authority until that blocker is closed explicitly
