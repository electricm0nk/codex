# SD13-E6-F11 Implementation Contract — Support-state and debt presentation

## Authority

This contract is extracted from:
- `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e6-r2-unsupported-depth-diagnostics-and-tester-visible-reporting-execution-handoff-2026-06-30.md`
- `repos/codex/CLAUDE.md`
- `repos/codex/AGENTS.md`
- `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md`
- `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md`

The coding implementer should not need to reread the full handoff for scope. If a detail here conflicts with the handoff, the handoff remains the higher authority.

## Objective

Implement one read-only desktop bridge from the existing SD-13 support-state matrix carrier into the existing SD-11 tester workbench, then render that truth visibly for testers.

The change is read-only support-state and debt presentation only. It must present existing SD-13 row state, evidence tier, debt reason, grounding reference, approved tester-facing wording, and next uplift. It must not create, promote, recompute, persist, submit, or infer support truth.

## Target repo and branch

Repo/workdir:
- `/home/ubuntu/workspace/repos/codex`

Recommended launch branch:
- `feat/sd13-e6-f11-support-state-debt-presentation`

Recommended launch sequence:

```bash
git fetch origin --prune
git switch develop --track origin/develop 2>/dev/null || git switch develop
git pull --ff-only origin develop
git switch -c feat/sd13-e6-f11-support-state-debt-presentation
```

If the branch already exists, reuse it only after confirming it belongs exclusively to this slice and is based on current `origin/develop`.

Live observation during this contract extraction: `/home/ubuntu/workspace/repos/codex` is already on `feat/sd13-e6-f11-support-state-debt-presentation` at `7ce891fba6b75f3ac4cd87c775c94708d9247dad`; `origin/develop` resolves to `60973f94ba91b3af8f918f655a9f21e679d97b17`; untracked `?? .claude/` and `?? apps/desktop/src-tauri/gen/` are present and are not part of the authorized write scope.

## Exact repo write scope

Only these repo paths may be created or modified:

1. `apps/desktop/src/boundary/loadSd13SupportStateMatrix.ts`
   - New dedicated frontend boundary file.
   - It must invoke the new Tauri command and expose typed support-state snapshot data to the desktop code.

2. `apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
   - The only SD-11 aggregation surface authorized to consume the new SD-13 snapshot.
   - Extend the workbench surface model with support/debt presentation data derived from the returned matrix rows.

3. `apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts`
   - May change only to thread the new boundary dependency into the existing runtime wrapper.

4. `apps/desktop/src/App.tsx`
   - The only tester-visible presentation surface authorized to render the new SD-13 support/debt section.
   - Render inside the existing single-page SD-11 tester workbench, not a new route/modal subsystem.

5. `apps/desktop/src-tauri/src/sd13_support_state_matrix.rs`
   - New Rust/Tauri read-only adapter over `codex::rules_core::support_state_matrix::seeded_sd13_e1_f1_current_truth()`.
   - Must expose a serializable snapshot with `rows` and preserve one object per seeded row.

6. `apps/desktop/src-tauri/src/main.rs`
   - May change only to register the new module/command and minimal glue required by that command.
   - Required command name: `load_sd13_support_state_matrix`.

No other repo file is in write scope.

## Required snapshot fields

The Tauri boundary must return a top-level snapshot containing `rows`. The row count must remain 21. No row filtering, aggregation, hiding, or suppression is authorized.

Each row must preserve at minimum:
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

The Rust adapter must consume `seeded_sd13_e1_f1_current_truth()` directly. It must not duplicate matrix literals or introduce new support-state truth.

## Required tester-facing wording

Use these exact labels when deriving `testerFacingStateLabel`:

- `supported` -> `Supported in the current bounded PF1 Core Rulebook roster slice for the named level band.`
- `partial` -> `Partially supported in the current bounded roster slice; some progression or semantic obligations remain explicitly limited.`
- `lossy` -> `Available only with lossy support in the current bounded roster slice; important semantics are simplified or approximated.`
- `blocked` -> `Blocked by known missing semantics in the current bounded roster slice.`
- `unverified` -> `Included in the bounded roadmap scope, but not yet verified for this support level.`

Do not use softer or stronger substitutes such as `ready`, `works`, `supported enough`, `parity`, or bare `Supported` for any non-`supported` row.

## UI presentation contract

The SD-11 workbench must render one explicit SD-13 support/debt section that keeps visible:
- subject identity or stable row identity
- claimed dimension
- raw support-state token
- evidence tier
- exact canonical tester-facing wording
- blocker/lossiness note when present
- grounding reference
- next required uplift

Rows may be grouped visually, but `partial`, `blocked`, `lossy`, and `unverified` rows must not be hidden for polish. If the UI trims verbosity, it must trim around the debt rather than away from it.

## TDD and verification contract

TDD is mandatory. Because the existing TypeScript proof files are not truthfully wired to a repo runner, the RED/GREEN cycle must happen first in the new Rust/Tauri adapter module.

Minimum RED assertions in `apps/desktop/src-tauri/src/sd13_support_state_matrix.rs` tests:
- snapshot contains exactly 21 rows
- Human pilot row remains `partial` + `computed`
- Fighter levels 2-10 row remains `blocked` + `computed`
- blocked rows preserve a non-empty blocker note
- every row preserves `groundingRef` and `nextRequiredUplift`
- canonical wording for `partial`, `blocked`, and `unverified` matches the exact phrases above
- no non-`supported` row collapses to bare `Supported`

Run at minimum:

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop && . "$HOME/.cargo/env" && cargo test --manifest-path src-tauri/Cargo.toml sd13_support_state_matrix
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test --test sd13_support_state_matrix
cd /home/ubuntu/workspace/repos/codex/apps/desktop && . "$HOME/.cargo/env" && cargo test --manifest-path src-tauri/Cargo.toml
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run typecheck
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run build
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run tauri:check
```

Do not claim the existing TypeScript files under `apps/desktop/src/sd11/**/*.test.ts` as passing verification unless a separately authorized runner lane makes them truthfully runnable.

## Explicit exclusions

This slice explicitly excludes:
- issue-capture/evidence coupling
- edits under `apps/desktop/src/sd11/feedback/**`
- pushing SD-13 rows into `captureFeedbackEvidence.ts`
- bug/enhancement payload schema changes
- auto-captured issue evidence schema changes
- GitHub transport, authentication, labels, submission behavior, attachment handling, or redaction behavior
- updater behavior, release/channel behavior, branch/channel behavior, or platform-tier behavior
- SD-14 persistence, save/load, lifecycle, migration, or upgrade-safe revision work
- broader SD-13 breadth work: support-state promotion/demotion/recomputation, new roster semantics, class progression, spellcasting burden, multiclassing, non-core content, archetypes, prestige classes, or level/breadth expansion
- edits to `src/rules_core/support_state_matrix.rs`
- edits to `tests/sd13_support_state_matrix.rs`
- edits to `apps/desktop/package.json`, `apps/desktop/tsconfig.json`, `apps/desktop/src-tauri/Cargo.toml`, `AGENTS.md`, or `CLAUDE.md`
- edits under `programs/codex/**`

If truthful completion requires any excluded surface, stop and block the CODE lane instead of widening scope.

## Completion expectation

The downstream CODE lane is complete only at verified `pr-created` state:
- fresh/reused-exclusive feature branch grounded against `origin/develop`
- bounded changes confined to the six allowed write paths
- TDD RED/GREEN evidence captured for the Rust/Tauri adapter
- full verification command set run with real results
- branch pushed
- PR opened against `develop`
- durable `claude-execution-receipt` comment added to the governed CODE card

This contract does not authorize merge to `develop` or `main`.
