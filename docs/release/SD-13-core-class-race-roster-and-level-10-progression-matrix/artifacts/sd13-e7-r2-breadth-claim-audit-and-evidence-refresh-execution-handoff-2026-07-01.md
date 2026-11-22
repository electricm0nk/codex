---
title: SD13-E7-R2 — Breadth-claim audit and evidence-refresh execution handoff
artifact_id: SD13-E7-R2
date: 2026-07-01
owner: Todd Hintzmann
scope: universal
authority_surface: execution-handoff
program: codex
requirement_pack: SD-13 core class/race roster and level-10 progression matrix
workflow_stage: CODE-ready handoff
workflow_route: SD13-E7-F13 CODE
status: ready-for-claude-launch
code_execution_harness: claude-code
source_readiness_artifact: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e7-r1-breadth-claim-audit-and-evidence-refresh-readiness-closure-2026-07-01.md
base_reference:
  repo: /home/ubuntu/workspace/repos/codex
  branch: origin/develop
  head_sha: 257c1376ae0edfa0c5f5b5f66f3e05b48a2594d5
observed_local_checkout:
  repo: /home/ubuntu/workspace/repos/codex
  branch: feat/sd13-e6-f11-support-state-debt-presentation
  head_sha: 1221ac243e36ecaa7767a465d5fe300083b93d11
  tracking: origin/feat/sd13-e6-f11-support-state-debt-presentation [gone]
  residue:
    - apps/desktop/src-tauri/gen/ (untracked)
launch_policy:
  mode: isolated-worktree-exception
  rationale: current shared checkout is not a clean execution substrate because it is parked on a gone feature branch and carries unrelated untracked generated residue
  worktree_path: /home/ubuntu/workspace/worktrees/codex-sd13-e7-f13-evidence-refresh-audit
  branch_name: feat/sd13-e7-f13-evidence-refresh-audit
  pr_base: develop
completion_class: pr-created
review_gate: human-review-on-pr
---

# 1. Core problem

The live SD-13 support-state carrier and SD-11 tester workbench can present support/debt truth, but they still lack any machine-usable freshness metadata or bounded stale-claim audit surface, which means breadth claims can drift away from current evidence without an honest runtime warning.

# 2. Decision

Freshness metadata must live in the typed SD-13 carrier and must be projected through the existing Tauri and TypeScript support-state surfaces unchanged.

Do not invent UI-local freshness heuristics as the source of truth. The carrier owns freshness truth; downstream layers may only project it and derive bounded audit presentation from it.

# 3. Objective

Implement the first honest SD13-E7-F13 slice by extending the existing SD-13 support-state truth path so the desktop tester workbench can show a bounded breadth-claim audit / evidence-refresh posture without promoting any unsupported row.

The slice is complete only when:

1. the typed SD-13 matrix can express per-row freshness posture explicitly,
2. the desktop bridge projects that freshness posture verbatim,
3. the SD-11 workbench derives a bounded audit presentation from the projected truth,
4. the app renders that audit posture visibly, and
5. all verification commands below pass from the fresh execution substrate.

# 4. Exact required reads

Read these in order before modifying code:

1. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
2. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
3. `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e7-r2-breadth-claim-audit-and-evidence-refresh-execution-handoff-2026-07-01.md`
4. `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e7-r1-breadth-claim-audit-and-evidence-refresh-readiness-closure-2026-07-01.md`
5. `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md`
6. `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md`
7. `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-design.md`
8. `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/acceptance-and-verification.md`
9. `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
10. `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/coverage-evidence-and-fixture-plan.md`
11. `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/adjacent-lane-boundary-and-breadth-claim-contract.md`
12. `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
13. `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/tester-facing-support-language-contract.md`
14. `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
15. `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`
16. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/sd13_support_state_matrix.rs`
17. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadSd13SupportStateMatrix.ts`
18. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
19. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
20. `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json`
21. Read `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts` and `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs` for grounding only if you need to prove existing wiring boundaries before changing any allowed file.

# 5. Current implementation truth that this slice must respect

As of 2026-07-01:

- `src/rules_core/support_state_matrix.rs` already carries typed row truth for support state, evidence tier, grounding ref, blocker/lossiness note, and next required uplift, but no row-level freshness field exists there yet.
- `tests/sd13_support_state_matrix.rs` already proves row count, row IDs, state separation, and grounding/uplift preservation for the seed.
- `apps/desktop/src-tauri/src/sd13_support_state_matrix.rs` projects the carrier into a serializable snapshot and derives tester-facing support wording, but it does not project any freshness/audit metadata yet.
- `apps/desktop/src/boundary/loadSd13SupportStateMatrix.ts` exposes only the projected support/debt fields.
- `apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` builds a read-only `supportDebt` presentation and unavailable notice, but no breadth-claim audit / freshness summary exists there yet.
- `apps/desktop/src/App.tsx` renders the support/debt section, row cards, and state tallies, but no stale-claim or refresh posture surface is rendered there yet.
- `apps/desktop/package.json` exposes `typecheck`, `build`, and `tauri:check`; there is no dedicated TypeScript unit-test runner wired for this lane, so do not counterfeit frontend TDD with imagined tests.

# 6. Exact allowed write scope

Modify only these files for this slice:

1. `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
2. `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`
3. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/sd13_support_state_matrix.rs`
4. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadSd13SupportStateMatrix.ts`
5. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
6. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`

If you conclude that `apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts` or `apps/desktop/src-tauri/src/main.rs` must change for this slice to compile, stop and report that blocker instead of silently widening scope.

# 7. Exact forbidden write scope

Do not modify:

- any file under `programs/codex/...`
- `apps/desktop/src/sd11/feedback/**`
- `apps/desktop/src/boundary/loadSd11UpdateAction.ts`
- `apps/desktop/src/sd11/update/**`
- release / CI / workflow files
- unrelated GE-08 authoring files
- unrelated SD-12 release-truth files
- any generated output under `apps/desktop/src-tauri/gen/`
- any file outside `/home/ubuntu/workspace/repos/codex`

# 8. Launch-from-origin/develop policy

Use Claude Code for the execution run.

Do not write from the current shared checkout.

Launch exactly like this:

```bash
git -C /home/ubuntu/workspace/repos/codex fetch origin --prune
rm -rf /home/ubuntu/workspace/worktrees/codex-sd13-e7-f13-evidence-refresh-audit
git -C /home/ubuntu/workspace/repos/codex worktree add \
  -b feat/sd13-e7-f13-evidence-refresh-audit \
  /home/ubuntu/workspace/worktrees/codex-sd13-e7-f13-evidence-refresh-audit \
  origin/develop
cd /home/ubuntu/workspace/worktrees/codex-sd13-e7-f13-evidence-refresh-audit
```

After launch, confirm:

```bash
git branch --show-current
git rev-parse HEAD
git rev-parse origin/develop
git status --short
```

Expected truth:

- branch: `feat/sd13-e7-f13-evidence-refresh-audit`
- base branch: `origin/develop`
- base SHA: `257c1376ae0edfa0c5f5b5f66f3e05b48a2594d5`
- `git status --short` clean before the first test edit

# 9. Implementation contract

## 9.1 Carrier ownership rule

The typed SD-13 carrier is the authority surface for freshness truth.

This slice must add explicit freshness metadata to the row truth carried by `SupportStateRow` or an equivalently tight typed structure owned by `src/rules_core/support_state_matrix.rs`.

That metadata must not be invented in the desktop bridge or UI.

## 9.2 First-slice honesty rule

Do not invent a broad calendar/SLA policy that the requirement pack does not authorize.

This first slice is allowed to be conservative. If freshness cannot be proven for a row from the seeded truth, the audit surface must present that row or the broader claim posture as refresh-required / stale / not-currently-audited rather than implying freshness.

The decisive point is honesty, not optimism.

## 9.3 Projection rule

Whatever freshness structure the carrier owns must be projected through:

- `apps/desktop/src-tauri/src/sd13_support_state_matrix.rs`
- `apps/desktop/src/boundary/loadSd13SupportStateMatrix.ts`

without UI-local reinterpretation.

## 9.4 Audit derivation rule

`apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` must derive a bounded audit presentation over the projected row truth.

At minimum, that presentation must:

- distinguish the support/debt matrix from the new audit/freshness posture,
- expose whether the current breadth claim surface is refresh-backed or refresh-required,
- avoid collapsing `partial`, `blocked`, `lossy`, or `unverified` rows into positive breadth claims,
- preserve visible grounding/evidence context instead of replacing it with vague labels.

## 9.5 UI surface rule

`apps/desktop/src/App.tsx` must render the new audit/freshness posture explicitly.

The UI result must be visibly subordinate to the row truth already shown in the support/debt section. It is an audit surface, not a promotion surface.

# 10. TDD and verification discipline

TDD is mandatory, but use the real runner surfaces available in this repo.

Required sequence:

1. Update or extend the Rust carrier tests in `tests/sd13_support_state_matrix.rs` so they fail for the intended freshness/audit gap.
2. Update or extend the Tauri bridge tests in `apps/desktop/src-tauri/src/sd13_support_state_matrix.rs` so they fail for the intended projected freshness/audit gap.
3. Implement the minimal production changes to make those tests pass.
4. Then wire the TypeScript/App projection and verify through `typecheck`, `build`, and `tauri:check`.

Do not add a new frontend test framework in this slice.

# 11. Required verification commands

Run these from the fresh worktree and report real outcomes:

```bash
cd /home/ubuntu/workspace/worktrees/codex-sd13-e7-f13-evidence-refresh-audit
cargo test --test sd13_support_state_matrix
cd /home/ubuntu/workspace/worktrees/codex-sd13-e7-f13-evidence-refresh-audit/apps/desktop
npm run typecheck
npm run build
npm run tauri:check
cargo test --manifest-path src-tauri/Cargo.toml sd13_support_state_matrix
```

Also run a write-scope audit before finalizing:

```bash
cd /home/ubuntu/workspace/worktrees/codex-sd13-e7-f13-evidence-refresh-audit
git diff --name-only -- \
  src/rules_core/support_state_matrix.rs \
  tests/sd13_support_state_matrix.rs \
  apps/desktop/src-tauri/src/sd13_support_state_matrix.rs \
  apps/desktop/src/boundary/loadSd13SupportStateMatrix.ts \
  apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts \
  apps/desktop/src/App.tsx
```

And a final repo truth check:

```bash
git status --short
git diff --stat
git rev-parse --abbrev-ref HEAD
git rev-parse HEAD
```

# 12. Exact non-goals

This slice does not authorize any of the following:

- changing support-state semantics beyond what is necessary to add freshness truth
- widening the SD-13 roster
- adding persistence, issue transport, feedback capture, or update coupling
- redesigning the GE-08 or SD-11 workbench architecture
- changing release-truth logic
- changing generated Tauri assets
- creating a second claim-audit system outside the existing support-state path
- inventing broad “all SD-13 is fresh” or “all SD-13 is stale forever” product copy divorced from per-row truth

# 13. Expected completion receipt

Completion is branch-ready only after:

1. a branch or worktree rooted at `feat/sd13-e7-f13-evidence-refresh-audit` exists off `origin/develop`,
2. the exact allowed write scope contains all code changes,
3. all required verification commands pass,
4. the branch is pushed to `origin`,
5. a PR targeting `develop` is opened, and
6. the execution report includes the PR link and a durable Claude Code execution receipt.

The receipt must include at least:

- Claude Code execution confirmation
- final branch name
- final HEAD SHA
- verification commands run
- verification results
- PR URL
- explicit note of any remaining bounded risk or none

# 14. Blockers that must stop the run

Stop and surface the blocker instead of guessing if any of the following occurs:

- the worktree cannot be created from `origin/develop`
- required freshness truth cannot be represented inside the current typed carrier without widening write scope
- `main.rs` or `loadSd11TesterWorkbenchSurfaceRuntime.ts` must change
- verification fails
- the only way forward is unrelated cleanup of the dirty shared checkout or generated residue

# 15. Why lesser approaches fail here

A UI-only badge would be fraud.
A repo-wide documentation sweep would be scope drift.
A fake freshness flag with no carrier ownership would become folklore.

This slice succeeds only if the freshness truth starts in the SD-13 carrier, survives projection unchanged, and reaches the tester workbench as an explicit bounded audit surface.
