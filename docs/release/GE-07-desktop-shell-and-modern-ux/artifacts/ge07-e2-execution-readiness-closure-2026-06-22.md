---
title: GE07-E2 Execution Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-07
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE07-E2 — UI-to-core command boundary contract
workflow_route: readiness-closure
readiness: blocked
handoff_created: false
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE07-E2 Execution Readiness Closure

## Verdict
GE07-E2 is not yet grounded enough to mint a code-authorizing boundary-adapter handoff.

This pass did recover the real repo/workdir, current `origin/develop` base, the live upstream domain payload carriers, and the exact prerequisite scaffold path set named by GE07-E1. That is the useful part. The decisive blocker is architectural, not informational: `origin/develop` still contains no desktop scaffold at all, and the upstream rules-core UI-consumer bridge that GE07-E2 would otherwise consume is still only an awaiting-Todd-launch GE06-E4-F1 handoff rather than merged repo truth.

Minting a GE07-E2 handoff now would do one of two wrong things:
1. silently combine GE07-E1 scaffold work and GE07-E2 boundary work into one broader slice, or
2. duplicate the still-unlaunched GE06-E4-F1 view-model lane inside a second boundary adapter.

Both are counterfeit readiness. Therefore this closure records the grounded contract and stops before code authority.

## Core problem
GE-07 now knows what the shell must ask of the core and what the UI must never own locally, but the repo does not yet contain the two implementation footholds that make a narrow E2 coding lane honest:

1. an executed desktop scaffold under `apps/desktop/`
2. a merged rules-core UI-consumer projection on `origin/develop` that is narrower than raw receipt internals and broader than shell-only mock state

Without those footholds, a supposed “boundary adapter” handoff would either be scaffold-plus-adapter scope creep or a duplicate read-model lane.

## Selected bounded slice
```text
GE07-E2 — UI-to-core command boundary contract
```

Intended responsibility when it eventually becomes code-ready:
- bind the desktop shell to real domain payloads through an explicit boundary
- preserve blocked/computed honesty, diagnostics visibility, and explanation/provenance references
- refuse frontend-owned rules semantics

What it must not become:
- the initial scaffold lane
- the rules-core read-model invention lane already claimed by GE06-E4-F1
- a broad “build the UI” handoff

## Required source evidence recovered
| Gate | Evidence |
|---|---|
| Target repo/workdir exists | `/home/ubuntu/workspace/repos/codex` exists and remains the future implementation surface for Codex. |
| Current base truth is grounded | `git rev-parse origin/develop` returned `7bc89e8c1edf8f1d1a6d490a0ad28ac72fc6f104`. The checked-out branch remains `ge06-e3-f2-classifier-impl` at `cc45f2c84b0c6bd3b3a7886f9f3068ece8b58e48`, which is residue and not GE-07 execution authority. |
| Root repo is still headless-core only | `find . -maxdepth 2` in the repo shows `Cargo.toml`, `src/`, `tests/`, repo conduct files, and no desktop subtree. `Cargo.toml` is still a single-package crate with no workspace configuration. |
| No scaffold exists on `origin/develop` | `git ls-tree -r --name-only origin/develop` showed `apps_desktop_entries=0` and `src_tauri_entries=0`. |
| No UI-consumer view-model bridge exists on `origin/develop` | `git ls-tree -r --name-only origin/develop` showed `pilot_view_model_entries=0` and `snapshot_entries=0`. `src/rules_core/mod.rs` still exposes only `character_input`, `pilot_compute`, and `pilot_failure`. |
| Real upstream payload carriers do exist | `src/rules_core/pilot_compute.rs` exposes `PilotHeadlessReceipt`, `HeadlessReceiptStatus`, and `build_pilot_headless_receipt`; `src/rules_core/pilot_failure.rs` exposes `FailureClassifier` and the `PrimaryOwner` vocabulary including `UiGap`. |
| Command-boundary requirements are already documentary truth | `artifacts/ui-command-boundary-requirements.md` names the required boundary capabilities and explicit UI prohibitions; `artifacts/component-surface-inventory.md` ties those duties to concrete GE-07 surfaces. |
| The narrow rules-core consumer bridge is already claimed elsewhere | `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md` and the paired `ge06-e4-f1-execution-handoff-2026-06-22.md` already define the next honest rules-core view-model lane, but that lane is still awaiting Todd launch rather than merged repo truth. |
| First runtime-boundary doctrine is already grounded | `artifacts/ge07-e1-runtime-boundary-adr-input-2026-06-22.md` fixes the smallest truthful transport answer at direct Tauri command invocation over the headless GE-06 receipt path. |

## Exact repo paths recovered this pass
The exact prerequisite scaffold paths already named by GE07-E1 are:

```text
apps/desktop/package.json
apps/desktop/src/main.tsx
apps/desktop/src/App.tsx
apps/desktop/src-tauri/Cargo.toml
apps/desktop/src-tauri/tauri.conf.json
apps/desktop/src-tauri/src/main.rs
```

These paths are the minimum additive shell shape the current doctrine recognizes. They are still absent on `origin/develop`.

That matters because any GE07-E2 coding lane launched now would necessarily have to create some or all of those files first, which means it would no longer be an E2-only boundary-adapter slice.

## Write-scope posture
### What can be named honestly now
The only exact repo paths that can be named without invention are the prerequisite GE07-E1 scaffold paths above and the already-grounded read-only upstream domain files:

```text
src/rules_core/mod.rs
src/rules_core/pilot_compute.rs
src/rules_core/pilot_failure.rs
```

### What cannot be named honestly yet
An exact GE07-E2-only writable file list inside the desktop subtree cannot yet be named truthfully, because the subtree does not exist and the intended boundary should not absorb scaffold creation.

If this pass were to invent file names deeper inside a non-existent scaffold just to force a handoff, that would be fiction masquerading as governance.

## Gate table
| Gate | Status | Resolution |
|---|---|---|
| Target repo/workdir grounded | pass | `/home/ubuntu/workspace/repos/codex` is explicit. |
| Branch-base truth grounded | pass | future GE-07 work must branch from fetched `origin/develop`, not the stale checked-out GE-06 topic branch. |
| Upstream payload carriers grounded | pass | receipt and failure-classifier surfaces exist in `src/rules_core/`. |
| Command-boundary duties grounded | pass | GE-07 documentary artifacts already define required capabilities and UI prohibitions. |
| Exact prerequisite scaffold paths named | pass | the six `apps/desktop/...` paths above are explicit and still absent. |
| Executed scaffold exists on repo base | fail | `origin/develop` has no `apps/desktop/` or `src-tauri/` entries. |
| Narrow UI-consumer rules-core bridge exists on repo base | fail | no `pilot_view_model`/`snapshot` layer exists on `origin/develop`; the only grounded candidate remains the unlaunched GE06-E4-F1 handoff. |
| Exact GE07-E2-only writable file list is grounded | fail | any list would currently smuggle scaffold creation or duplicate E4-F1 work. |
| Verification commands for a truthful E2 coding slice are runnable | fail | until the scaffold and upstream bridge are real, there is no honest E2-specific RED/GREEN command set to authorize. |
| Code-authorizing handoff justified | fail | prerequisites missing; `handoff_created: false`. |

## Branch and dependency posture
If GE-07 later resumes toward execution, the branch rule remains:

```bash
git fetch origin --prune
git switch develop
git reset --hard origin/develop
```

But no GE07-E2 branch should be created yet, because the future boundary lane would immediately depend on two truths not yet present on that base:
- an actual desktop scaffold subtree
- a merged rules-core UI-consumer bridge, or an explicit decision to collapse that responsibility into a newly bounded replacement slice

## Shortest honest next move
1. Decide whether the program still wants the GE06-E4-F1 rules-core view-model lane. If yes, Todd must launch it or otherwise land an equivalent merged read-model bridge on `origin/develop`.
2. If early desktop work is still desired after that, derive a separate GE07-E1 execution-readiness closure and bounded execution handoff for scaffold creation only.
3. After a real scaffold exists and the rules-core consumer bridge is merged, rerun GE07-E2 readiness against the live tree.
4. Only then mint a stage-specific GE07-E2 handoff whose writable file list is exact, narrow, and free of scaffold duplication.

## Explicit non-goals for this pass
This closure does not authorize:
- `apps/desktop/**` creation by implication
- Tauri, React, TypeScript, or package-manager implementation work
- a duplicate rules-core snapshot/view-model lane alongside GE06-E4-F1
- frontend-owned rules or explanation logic
- a broad “build the UI” packet

## Completion rule
This readiness closure is complete because it does all of the honest work GE07-E2 can support today:
- grounds the live repo base and upstream payload carriers
- names the exact prerequisite scaffold repo paths already authorized at the documentary level
- records why no exact E2-only write scope exists yet
- refuses to mint counterfeit code authority while E1 scaffold truth and the GE06-E4-F1 read-model dependency remain unresolved
