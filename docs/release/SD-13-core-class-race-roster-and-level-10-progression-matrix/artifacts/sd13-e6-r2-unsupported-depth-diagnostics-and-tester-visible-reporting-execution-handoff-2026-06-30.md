---
title: SD13-E6-F11 Execution Handoff — Support-state and debt presentation contract
handoff_id: HANDOFF-CODEX-SD13-E6-F11-SUPPORT-STATE-DEBT-PRESENTATION-2026-06-30
stc_id: STC-CODEX-SD-13
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: ready-for-claude-launch
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e6-r2-unsupported-depth-diagnostics-and-tester-visible-reporting-execution-handoff-2026-06-30.md
source_stc: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md
source_epic_breakdown: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md
source_readiness_closure: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e6-r1-unsupported-depth-diagnostics-readiness-closure-2026-06-30.md
selected_slice: SD13-E6-F11 — Support-state and debt presentation contract
run_in: Claude Code only
code_authority: true
authority_dependencies:
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/adjacent-lane-boundary-and-breadth-claim-contract.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/tester-facing-support-language-contract.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e6-r1-unsupported-depth-diagnostics-readiness-closure-2026-06-30.md
  - programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md
  - programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/tester-workbench-surface-specification.md
  - repos/codex/src/rules_core/support_state_matrix.rs
  - repos/codex/tests/sd13_support_state_matrix.rs
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  observed_local_branch: feat/sd13-e1-f1-rules-core-support-state-matrix
  observed_local_head: 3827378a5bfe6dda22ad18695140d7f4fa723a5f
  expected_base_ref: origin/develop
  expected_base_sha_at_handoff_creation: c2cea5c6baeb3ca34077b85331214c4b42a4809c
  recommended_branch: feat/sd13-e6-f11-support-state-debt-presentation
  pr_target: develop
completion_class: pr-created
reviewed_at: 2026-06-30
---

# SD13-E6-F11 Execution Handoff — Support-state and debt presentation contract

## Status
This is the stage-specific code-authorizing brief for the already-routed downstream story `SD13-E6-F11 CODE: Support-state and debt presentation contract`.

It grants code authority only for the bounded slice below. It does not itself prove Claude execution, a pushed branch, a PR, or a merge. That truth belongs to the governed CODE lane and its durable `claude-execution-receipt`.

Board routing must remain:
- current documentary handoff artifact: `t_41128571`
- downstream CODE lane: `t_adfddfad`

## Run in
Claude Code only.

Do not execute this implementation primarily through Hermes file-editing tools. If Claude Code cannot be launched truthfully, block the CODE lane instead of silently coding through Hermes.

## Core problem
SD-13 now has a machine-usable support-state matrix and an explicit debt ledger, but the current SD-11 tester workbench does not surface that truth anywhere testers can see it.

The live desktop app already renders real bounded workflow state, diagnostics, blocked claims, explanation references, provenance references, support-tier posture, and governed feedback scaffolding. But none of those surfaces currently consume `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`. If the first SD13-E6 code slice improvises support labels inside the UI, hides debt behind generic diagnostics, or couples roster-state display to issue-transport work, the workbench becomes a second requirements authority and the lane starts lying.

The decisive move is smaller than evidence coupling and smaller than breadth implementation: add one read-only desktop bridge over the existing SD-13 matrix carrier, then render that truth visibly inside the current SD-11 workbench with SD-13-approved wording only.

## Objective
Implement the smallest truthful SD13-E6 bridge between the existing `rules_core` support-state matrix and the existing SD-11 tester workbench.

The result must prove all of the following:
1. the desktop runtime can read the seeded SD-13 support-state matrix through a dedicated read-only Tauri boundary
2. the SD-11 workbench can render SD-13 state, evidence tier, debt reason, grounding reference, and next uplift without inventing local support rules
3. the tester-visible wording comes from the SD-13 support-language contract, not from new UI-local optimism
4. visible debt remains visible when a row is `partial`, `blocked`, `lossy`, or `unverified`
5. `SD13-E6-F12 — Evidence and issue-capture coupling` remains explicitly deferred in both code and verification scope
6. no broader roster support, release/update truth, persistence truth, or GitHub transport behavior is inferred from this slice

This slice stops at truthful presentation of existing SD-13 matrix/debt truth. It does not yet wire that truth into evidence capture or issue composition.

## Why this route is authorized now
This handoff is authorized because the live repo and the readiness closure now agree on one narrow truthful seam:
- `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs` already exists as the bounded SD-13 carrier, and `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs` proves the seeded current-truth rows.
- `cargo test --test sd13_support_state_matrix` is green on the live repo right now with 18/18 tests passing.
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` is already the SD-11 aggregation point for a real bounded tester surface.
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx` already renders the exact kind of truth-preserving panels this slice needs: diagnostics, blocked claims, explanation references, provenance references, update/support posture, and bounded-scope messaging.
- `npm run typecheck`, `npm run build`, `npm run tauri:check`, and `cargo test --manifest-path src-tauri/Cargo.toml` are all green on the live repo right now.
- the current direct Node execution of `src/sd11/loadSd11TesterWorkbenchSurface.test.ts` still fails with `ERR_MODULE_NOT_FOUND`, so the truthful way to drive TDD in this slice is on the runnable Rust/Tauri boundary rather than by pretending the existing TypeScript proof files are already wired to a governed runner.

## Target repo and branch policy
```text
repo:    /home/ubuntu/workspace/repos/codex
workdir: /home/ubuntu/workspace/repos/codex
```

Grounded repo facts at handoff creation time:
- current local branch: `feat/sd13-e1-f1-rules-core-support-state-matrix`
- current local `HEAD`: `3827378a5bfe6dda22ad18695140d7f4fa723a5f`
- grounded remote base for this lane: `origin/develop` at `c2cea5c6baeb3ca34077b85331214c4b42a4809c`

Launch from a fresh `origin/develop`-based feature branch, not from the current local feature branch.

Use this exact setup:
```bash
git fetch origin --prune
git switch develop --track origin/develop 2>/dev/null || git switch develop
git pull --ff-only origin develop
git switch -c feat/sd13-e6-f11-support-state-debt-presentation
```

If `feat/sd13-e6-f11-support-state-debt-presentation` already exists, reuse it only after confirming it still belongs exclusively to this slice.

## Exact required reads before coding
Read these first, in order:
1. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
2. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
3. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e6-r2-unsupported-depth-diagnostics-and-tester-visible-reporting-execution-handoff-2026-06-30.md`
4. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e6-r1-unsupported-depth-diagnostics-readiness-closure-2026-06-30.md`
5. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md`
6. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md`
7. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
8. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
9. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/adjacent-lane-boundary-and-breadth-claim-contract.md`
10. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/tester-facing-support-language-contract.md`
11. `/home/ubuntu/workspace/programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md`
12. `/home/ubuntu/workspace/programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/tester-workbench-surface-specification.md`
13. `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
14. `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`
15. `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json`
16. `/home/ubuntu/workspace/repos/codex/apps/desktop/tsconfig.json`
17. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts`
18. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadPilotShellSnapshot.ts`
19. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
20. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts`
21. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
22. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts`
23. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts`
24. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts`
25. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
26. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml`
27. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`

Use them as bounded authority surfaces, not as permission to widen scope.

## Exact allowed write scope
You may create or modify only these repo paths:
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadSd13SupportStateMatrix.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/sd13_support_state_matrix.rs`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`

Write-scope interpretation:
- the new TypeScript boundary file is the only new frontend loader surface authorized here
- `loadSd11TesterWorkbenchSurface.ts` is the only SD-11 aggregation surface authorized to consume the new snapshot
- `loadSd11TesterWorkbenchSurfaceRuntime.ts` may change only to thread the new boundary dependency into the existing runtime wrapper
- `App.tsx` is the only tester-visible presentation surface authorized to render the new support/debt block
- the new Rust file is the only new Tauri-side adapter surface authorized here
- `main.rs` may change only to register the new read-only command/module and any minimal helper glue required by that command

No other repo file is in write scope.

## Forbidden write scope and explicit non-goals
This handoff does not authorize:
- any edits under `/home/ubuntu/workspace/programs/codex/**`
- any edits to `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts`
- any edits to `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadPilotShellSnapshot.ts`
- any edits to `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
- any edits to `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/**`
- any edits to `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/update/**`
- any edits to `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json` or `/home/ubuntu/workspace/repos/codex/apps/desktop/tsconfig.json`
- any edits to existing TypeScript `*.test.ts` proof files
- any edits to `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml`
- any edits to `/home/ubuntu/workspace/repos/codex/AGENTS.md` or `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
- any edits under `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/**`
- any edits under `/home/ubuntu/workspace/repos/codex/src/oracle_validation/**`
- any edits under `/home/ubuntu/workspace/repos/codex/src/pcgen_import/**`
- any issue-capture coupling, GitHub auth/transport work, bug/enhancement payload schema changes, or evidence-capture expansion
- any updater, branch/channel, release, or platform-tier behavior changes
- any persistence, save/load, or lifecycle work
- any support-state promotion, demotion, or recomputation logic
- any claim-composition engine, breadth scoring, or broader roster implementation
- any multiclassing, non-core, archetype, prestige-class, spellcasting-burden, or class-progression widening beyond presentation of current matrix truth

If truthful completion would require touching any forbidden surface, stop and block the CODE lane instead of widening scope.

## Contract to implement
Implement one new read-only desktop bridge over `rules_core::support_state_matrix::seeded_sd13_e1_f1_current_truth()` and render that bridge inside the existing SD-11 tester workbench.

### Required runtime boundary
Required new file surface:
```text
/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/sd13_support_state_matrix.rs
```

The new module must:
- consume `codex::rules_core::support_state_matrix::seeded_sd13_e1_f1_current_truth()` directly rather than duplicating matrix literals
- expose a serializable snapshot shape for the desktop boundary
- remain read-only and documentary/control-plane in nature
- avoid any rules computation, persistence, mutation, promotion, or issue-transport logic

Required snapshot posture:
- a top-level snapshot object containing `rows`
- one row object per seeded SD-13 row
- each row must preserve, at minimum:
  - `rowId`
  - `subjectType`
  - `subjectId`
  - `dimension`
  - `supportState`
  - `evidenceTier`
  - `testerFacingStateLabel`
  - `groundingRef`
  - `blockerOrLossinessNote`
  - `nextRequiredUplift`

The row count must remain 21 in this slice. No row filtering, aggregation, or hidden suppression is authorized in the Tauri command.

Register one new Tauri command in `main.rs`:
- command name: `load_sd13_support_state_matrix`

### Required canonical tester-facing wording
For determinism in this slice, use these exact SD-13-approved phrases when deriving `testerFacingStateLabel`:
- `supported` -> `Supported in the current bounded PF1 Core Rulebook roster slice for the named level band.`
- `partial` -> `Partially supported in the current bounded roster slice; some progression or semantic obligations remain explicitly limited.`
- `lossy` -> `Available only with lossy support in the current bounded roster slice; important semantics are simplified or approximated.`
- `blocked` -> `Blocked by known missing semantics in the current bounded roster slice.`
- `unverified` -> `Included in the bounded roadmap scope, but not yet verified for this support level.`

Do not invent alternate synonyms such as `ready`, `works`, `should work`, `supported enough`, or `parity`.

### Required TypeScript consumption surface
Required file surfaces:
```text
/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadSd13SupportStateMatrix.ts
/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts
/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts
```

The TypeScript side must:
- invoke the new Tauri command through a dedicated boundary file
- extend the SD-11 surface model with a bounded support/debt presentation structure derived from the returned rows
- keep the new structure separate from feedback evidence capture and separate from update/support-tier status
- avoid treating GE-08 workflow success, app build success, or platform status as proof that any roster row is `supported`

### Required UI presentation posture
Required visible file surface:
```text
/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx
```

The UI must render one explicit SD-13 support/debt section inside the existing tester workbench that, for each row it shows, keeps all of the following visible:
- the subject identity or stable row identity
- the dimension under claim
- the raw state token and evidence tier
- the exact canonical tester-facing wording above
- the blocker/lossiness note when present
- the grounding reference
- the next required uplift

Presentation rules:
- the section must live inside the current single-page SD-11 tester workbench, not in a new route or modal subsystem
- the section must not rewrite the existing diagnostics/blocked-claims panels into issue-capture machinery
- the section may visually group rows by subject type, but it must not hide `blocked`, `partial`, `lossy`, or `unverified` rows for polish
- if the UI trims verbosity anywhere, it must trim around the debt, not away from it

### Required preservation rules
This slice must preserve these truths:
- SD-13 owns state taxonomy, debt reason, evidence tier, grounding ref, next uplift, and approved wording
- SD-11 owns workbench structure and placement of the new section inside the tester surface
- the new roster section is read-only truth presentation, not a claim-composition engine
- feedback evidence capture remains exactly as-is in this slice
- GitHub submission transport remains exactly as-is in this slice

### Explicit F12 defer boundary
`SD13-E6-F12 — Evidence and issue-capture coupling` is not part of this slice.

That means this slice must not:
- add new fields to feedback payload assembly
- push SD-13 rows into `captureFeedbackEvidence.ts`
- change bug or enhancement composition
- alter auto-captured issue evidence schemas
- change issue labels, GitHub submission behavior, or attachment/redaction handling

If a desired improvement crosses that boundary, stop and route it to the later F12 lane.

## TDD requirement
TDD is mandatory.

Because the existing TypeScript proof files are not truthfully wired to a governed runner yet, the mandatory RED/GREEN cycle for this slice must happen on the new Rust/Tauri adapter contract first.

Execution order:
1. create failing Rust-side tests inside `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/sd13_support_state_matrix.rs` first
2. run the targeted src-tauri test command below and capture the real RED failure
3. implement the smallest adapter/command code required to satisfy the failing tests
4. rerun the targeted src-tauri test command to green
5. wire the TypeScript boundary and UI presentation surfaces
6. rerun the full verification command set below

Important RED rule:
- a missing module or missing command compile failure is acceptable only if the new tests already express the intended support-state presentation contract explicitly
- do not skip the tests because the UI-side TypeScript proof surfaces are not yet runner-wired

Minimum RED assertions in the new Rust module tests:
1. the snapshot contains exactly 21 rows
2. the Human pilot row remains `partial` plus `computed`
3. the Fighter levels 2-10 row remains `blocked` plus `computed`
4. blocked rows preserve a non-empty blocker note
5. every row preserves `groundingRef` and `nextRequiredUplift`
6. the canonical tester-facing wording for `partial`, `blocked`, and `unverified` rows matches the exact phrases named above
7. no non-`supported` row carries a `testerFacingStateLabel` that falsely collapses into bare `Supported`

## Evidence surfaces with current runner gap
These existing TypeScript proof files are real evidence surfaces but are not truthful mandatory verification gates for this slice unless a separately authorized runner lane exists:
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts`

Current live truth:
- direct execution of `node src/sd11/loadSd11TesterWorkbenchSurface.test.ts` from `/home/ubuntu/workspace/repos/codex/apps/desktop` fails with `ERR_MODULE_NOT_FOUND` while resolving the extensionless import for `loadSd11TesterWorkbenchSurface`
- this handoff does not authorize adding a TypeScript test runner, patching import strategy, or changing package/tooling surfaces merely to make these files runnable

Therefore:
- read those files as current behavior evidence and deferred-coupling context
- do not put them in write scope
- do not claim them as passing verification for this slice

## Exact verification commands
Run these at minimum:
```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop && . "$HOME/.cargo/env" && cargo test --manifest-path src-tauri/Cargo.toml sd13_support_state_matrix
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test --test sd13_support_state_matrix
cd /home/ubuntu/workspace/repos/codex/apps/desktop && . "$HOME/.cargo/env" && cargo test --manifest-path src-tauri/Cargo.toml
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run typecheck
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run build
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run tauri:check
```

Verification interpretation:
- the first command is the mandatory RED/GREEN proof surface for the new read-only desktop adapter contract
- the second command protects the upstream SD-13 carrier truth this slice must consume without changing
- the third command is the broader src-tauri regression sweep
- `npm run typecheck` proves the desktop TypeScript surface stays coherent
- `npm run build` proves the Vite/React workbench still compiles
- `npm run tauri:check` proves the desktop bridge still compiles as a whole

## Stop conditions
Stop and block the CODE lane instead of widening it if any of these occur:
- truthful implementation requires edits outside the six allowed write paths
- truthful implementation appears to require changes to feedback capture, bug/enhancement composition, GitHub transport, updater behavior, or platform support status
- truthful implementation appears to require changing the upstream SD-13 matrix carrier instead of consuming it read-only
- truthful implementation appears to require package/tooling changes to make TypeScript tests runnable
- the repo cannot be refreshed to a clean `origin/develop`-based execution branch
- any verification command above fails after the bounded change

## Expected completion class
This lane is complete only at `pr-created` truth:
- fresh branch launched from `origin/develop`
- bounded changes confined to the allowed write scope
- branch pushed to `origin`
- normal PR opened against `develop`
- durable Claude execution receipt attached to the governed CODE card

This handoff does not authorize merge to `develop` or `main`.

## Required Claude receipt
Before the downstream CODE card completes, add a durable `claude-execution-receipt` comment that records:
- exact handoff path
- invocation mode
- repo/workdir
- branch and base SHA at launch
- durable Claude session or process handle when available, or `unknown`
- model identity when available, or `unknown`
- files changed
- RED failure summary
- verification commands run and their real results
- resulting commit and PR handle
- final completion class (`pr-created` or truthful blocker)

Without that receipt, the lane must not be described as Claude-executed.

## Merge authority boundary
This handoff authorizes only the bounded implementation slice above.

It does not authorize:
- merging the branch or PR
- landing code onto `develop` or `main`
- broadening into `SD13-E6-F12`, SD13-E7, or any later SD-13 lane
- any write outside the exact allowed scope

Stop at verified `pr-created` state and return control through the governed review surface.
