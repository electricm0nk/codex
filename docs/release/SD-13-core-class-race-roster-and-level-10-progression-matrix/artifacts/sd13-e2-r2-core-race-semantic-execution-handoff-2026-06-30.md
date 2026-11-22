---
title: SD13-E2-R2 Execution Handoff — Human pilot race semantics and named interaction substrate
handoff_id: HANDOFF-CODEX-SD-13-E2-R2-CODING-2026-06-30
stc_id: STC-CODEX-SD-13
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: ready-for-claude-launch
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e2-r2-core-race-semantic-execution-handoff-2026-06-30.md
source_stc: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md
source_epic_breakdown: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md
source_readiness_closure: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e2-r1-core-race-semantic-readiness-closure-2026-06-30.md
selected_slice: SD13-E2-R2 — Human pilot race semantics and named interaction substrate
run_in: Claude Code only
code_authority: true
authority_dependencies:
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-design.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/coverage-evidence-and-fixture-plan.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e2-r1-core-race-semantic-readiness-closure-2026-06-30.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  expected_base_sha_at_creation: 60973f94ba91b3af8f918f655a9f21e679d97b17
  compare_base_ref: origin/develop
  compare_base_sha_at_creation: 60973f94ba91b3af8f918f655a9f21e679d97b17
  upstream_review_surface: https://github.com/electricm0nk/codex/pull/37
  recommended_branch: feat/sd13-e2-f3-f4-human-race-semantic-substrate
  pr_target: develop
allowed_write_scope:
  - src/rules_core/support_state_matrix.rs
  - src/rules_core/pilot_compute.rs
  - tests/sd13_support_state_matrix.rs
  - tests/ge06_pilot_input_contract.rs
  - tests/ge06_pilot_selected_skill_modifiers.rs
  - tests/ge06_pilot_headless_receipt.rs
  - tests/ge06_failure_classifier.rs
  - tests/ge06_pilot_view_model.rs
forbidden_write_scope:
  - programs/codex/**
  - apps/desktop/**
  - apps/desktop/src-tauri/gen/**
  - Cargo.toml
  - Cargo.lock
  - src/lib.rs
  - src/rules_core/character_input.rs
  - src/rules_core/pilot_view_model.rs
  - tests/ge06_pilot_total_saves.rs
  - tests/ge06_pilot_combat_baseline.rs
  - tests/fixtures/**
  - AGENTS.md
  - CLAUDE.md
reviewed_at: 2026-06-30
---

# SD13-E2-R2 Execution Handoff — Human pilot race semantics and named interaction substrate

## Status
This is the stage-specific code-authorizing brief for the first repo-facing SD13-E2 lane.

It carries `code_authority: true` for one bounded slice only: make the already-grounded Human pilot race seam and its named interaction pressure explicit in repo truth without pretending that all core races, all race semantic families, or any broader class progression surface are now implemented.

Board-visible verdict:
- this handoff authoring is ready now
- this file is not code execution truth
- later truth exists only if a governed Claude Code lane executes this brief and leaves a durable `claude-execution-receipt`

## Run in
Claude Code only.

Do not substitute another coding harness and do not execute this lane primarily through Hermes file editing. Hermes authored the handoff. Claude Code implements it.

## Core problem
SD13-E1 now gives Codex a machine-usable support-state matrix and a live review surface, but the current repo still carries race semantics only as coarse control-plane truth plus indirect GE-06 evidence.

Live state re-grounded on 2026-06-30 before launch repair:
- `git rev-parse origin/develop` reports `60973f94ba91b3af8f918f655a9f21e679d97b17`
- PR `#35` (`feat/sd13-e1-f1-rules-core-support-state-matrix`) is merged into `develop` at `5568ddc5b024a92efb41e549f7206130acf73aee`
- PR `#37` (`feat/sd13-e1-f1-support-state-matrix`) is merged into `develop` at `a42859ae12dfafb917d2bf25f0e6e7ef951e13b9`
- the old E1 feature branches are no longer the truthful launch substrate for SD13-E2; this slice must now start from current `origin/develop`
- a detached `origin/develop` worktree at `60973f94ba91b3af8f918f655a9f21e679d97b17` passed the focused regression floor live: `sd13_support_state_matrix` (18), `ge06_pilot_input_contract` (2), `ge06_pilot_total_saves` (3), `ge06_pilot_combat_baseline` (4), `ge06_pilot_view_model` (2), `ge06_pilot_selected_skill_modifiers` (5), `ge06_pilot_headless_receipt` (2), `ge06_failure_classifier` (5)

The decisive constraint is what this lane must not do. It must not counterfeit:
- full seven-race semantic closure
- full class progression closure
- spellcasting or multiclass depth
- silent widening into new fixtures, new input surfaces, or view-model source changes

The first honest move is narrower: use the accepted E1 matrix branch as the upstream substrate and make the currently grounded Human race-linked seam explicit in typed matrix truth and compute/explanation truth, while leaving non-Human races and broader race families visibly unverified.

## Objective
Implement the smallest truthful SD13-E2 repo slice.

The result must prove all of the following:
1. `src/rules_core/support_state_matrix.rs` can represent the currently grounded Human race-semantic seam more explicitly than the coarse E1 seed without collapsing race rows, class rows, and interaction rows into one vague breadth claim
2. `src/rules_core/pilot_compute.rs` exposes explicit race-linked explanation and/or claim-blocking truth derived from the already-grounded deterministic Human selections, rather than leaving race semantics as an accidental side effect of unrelated numeric tests
3. all non-Human race truth remains visibly constrained by real evidence: no non-Human core race may be promoted above `Unverified` / `Observed` in this lane without new grounded proof
4. class truth remains separate from race truth: this lane may not promote Fighter progression, Rogue progression, or any other class posture beyond what SD13-E1 already grounded
5. named interaction truth remains explicit: the Human bonus-feat / ability-bonus pressure seam must stay distinct from the Human race row and from class progression rows
6. no new fixture family, parser/input-shape widening, UI/reporting surface, or governance/doc rewrite is smuggled in to simulate progress

This slice stops at a race-semantic and interaction substrate over the already-accepted Human pilot seam. It does not claim broad race coverage.

## Why this route is authorized now
This route is authorized because the repo already contains one narrow, truthful upstream substrate with a review surface:
- `src/rules_core/support_state_matrix.rs` and `tests/sd13_support_state_matrix.rs` now exist and prove the typed row carrier is live
- `src/rules_core/character_input.rs` already exposes `race_id`, `selected_choices`, `selected_feats`, `skill_allocations`, and `equipment_selections`, which is enough read-only chosen-input truth for the grounded Human seam
- `src/rules_core/pilot_compute.rs` is still the only live repo seam that can convert those chosen inputs into computed evidence, explanations, and claim-blocking diagnostics
- `tests/ge06_pilot_input_contract.rs` already proves the deterministic accepted input names `race:human`, `choice:human_bonus_feat -> feat:dodge`, and `choice:human_ability_bonus -> ability:strength`
- `tests/ge06_pilot_selected_skill_modifiers.rs`, `tests/ge06_pilot_headless_receipt.rs`, `tests/ge06_failure_classifier.rs`, and `tests/ge06_pilot_view_model.rs` already prove that changes in `pilot_compute.rs` propagate into skill-facing outputs, receipt truth, blocker ownership, and view-model projection
- the accepted E1 truth is no longer a stacked-branch dependency; it is merged sanctioned-base truth on current `origin/develop`

What is still not authorized:
- any claim that the repo now supports every core race
- any promotion of non-Human race rows above `Observed`
- any claim that Human size/speed, senses, full racial trait burden, or every interaction seam are already computed just because one deterministic Human Fighter path exists
- any change that requires widening the chosen-input model, fixture corpus, or source view-model projection without a new readiness pass

## Target repo / workdir
```text
repo:    /home/ubuntu/workspace/repos/codex
workdir: /home/ubuntu/workspace/repos/codex
```

Current grounded repo facts during handoff launch repair:
- sanctioned base: `origin/develop` at `60973f94ba91b3af8f918f655a9f21e679d97b17`
- accepted upstream matrix substrate is already merged on that base through PR `#35` and PR `#37`
- the old E1 feature branch is no longer a truthful launch base for this slice
- if the shared checkout is occupied by another active lane, launch this slice from a fresh isolated worktree rooted at `/home/ubuntu/workspace/repos/codex`

## Branch policy
Do not copy stale launch facts from earlier SD13 documentary surfaces.

The truthful substrate for this lane is current sanctioned `origin/develop`, not any older E1 feature branch.

Launch from a fresh isolated worktree off current `origin/develop` and create the bounded feature branch there:

```bash
git -C /home/ubuntu/workspace/repos/codex fetch origin --prune
WT=/home/ubuntu/workspace/worktrees/codex-sd13-e2-f3-f4
git -C /home/ubuntu/workspace/repos/codex worktree add -b feat/sd13-e2-f3-f4-human-race-semantic-substrate "$WT" origin/develop
cd "$WT"
```

If `feat/sd13-e2-f3-f4-human-race-semantic-substrate` already exists, reuse it only after confirming it still belongs exclusively to this slice and still starts from current sanctioned base truth.

Record the actual launch branch, upstream base SHA, and resulting PR/commit handles in the final `claude-execution-receipt`.

## Exact required reads before coding
Read these first, in order:
1. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
2. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
3. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e2-r2-core-race-semantic-execution-handoff-2026-06-30.md`
4. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e2-r1-core-race-semantic-readiness-closure-2026-06-30.md`
5. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md`
6. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md`
7. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md`
8. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-design.md`
9. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
10. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
11. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/coverage-evidence-and-fixture-plan.md`
12. `/home/ubuntu/workspace/repos/codex/README.md`
13. `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
14. `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
15. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
16. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_view_model.rs`
17. `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`
18. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
19. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
20. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
21. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_selected_skill_modifiers.rs`
22. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_headless_receipt.rs`
23. `/home/ubuntu/workspace/repos/codex/tests/ge06_failure_classifier.rs`
24. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_view_model.rs`
25. `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`

Use them as bounded authority surfaces, not as permission to widen scope.

## Exact allowed write scope
You may create or modify only these repo paths:

```text
/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs
/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs
/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs
/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs
/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_selected_skill_modifiers.rs
/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_headless_receipt.rs
/home/ubuntu/workspace/repos/codex/tests/ge06_failure_classifier.rs
/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_view_model.rs
```

Write-scope interpretation:
- `support_state_matrix.rs` is the authoritative control-plane truth surface for race rows, class rows, and named interaction rows
- `pilot_compute.rs` is the only live repo seam allowed to turn the already-grounded Human chosen-input seam into explicit race-linked explanations or blockers
- `tests/sd13_support_state_matrix.rs` is the mandatory matrix-truth gate and must pin the final exact row set/row semantics for this slice
- `ge06_pilot_input_contract.rs` may be updated only to assert the exact Human race-choice seam this slice relies on
- `ge06_pilot_selected_skill_modifiers.rs`, `ge06_pilot_headless_receipt.rs`, `ge06_failure_classifier.rs`, and `ge06_pilot_view_model.rs` may be updated only where the new race-linked explanation or blocker truth needs explicit regression coverage

No other repo file is in write scope.

## Read-only grounding seams
These files are grounding truth for this lane and may not be edited under this handoff:
- `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
- `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_view_model.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`

If truthful completion would require opening any of these for write authority, stop and block the lane instead of widening silently.

## Contract to implement
Implement one bounded race-semantic substrate over the already-grounded Human Fighter deterministic path.

### Exact evidence you are allowed to lean on
The only accepted computed race-linked seam available at launch time is the current deterministic Human pilot surface:
- `race_id = race:human`
- `choice:human_bonus_feat = feat:dodge`
- `choice:human_ability_bonus = ability:strength`
- downstream pressure already visible through current computed outputs and their tests

That is enough to make Human race pressure explicit.
It is not enough to claim:
- non-Human race semantics
- general Human trait completeness
- size/speed/senses closure
- every future race/class interaction seam

### Required result shape
The final implementation must satisfy all of the following:
1. race truth remains separate from class truth and from named interaction truth
2. the Human pilot seam becomes explicitly legible in machine-usable repo truth instead of being only implied by one coarse row and incidental numeric outputs
3. non-Human races remain visible and honest as unverified unless the existing deterministic evidence really grounds more
4. the Human bonus-feat / ability-bonus pressure seam remains a named interaction surface, not a hidden side effect of class computation
5. any new explanation or blocker wording added in `pilot_compute.rs` must be derived strictly from existing chosen input and current deterministic outputs

### Permitted matrix movement
You may update the E1 matrix seed only within these boundaries:
- you may refine or split the current Human race row if doing so is necessary to make the Human pilot seam explicit
- you may refine the existing Human interaction row if doing so makes the named interaction seam more honest
- you may update notes, grounding refs, and next-uplift text to match the more explicit race-semantic substrate
- you may not promote any non-Human race row above `Unverified` / `Observed`
- you may not promote any class row above the current E1 posture
- you may not introduce a 7 x 11 combination matrix or universal per-combination interaction grid

If the final slice changes the exact row set, `tests/sd13_support_state_matrix.rs` must pin the new exact shape explicitly.

### Permitted compute movement
You may update `pilot_compute.rs` only to make the grounded Human race-linked seam explicit.

Allowed examples:
- explanation records that make the Human ability-bonus pressure visible rather than merely inferable
- explanation or diagnostic wording that makes the Human bonus-feat interaction seam explicit where the deterministic pilot already proves it
- bounded blocker truth that clarifies why broader race semantics are still absent

Forbidden examples:
- widening to new race fixtures
- adding general race trait engines
- rewriting chosen-input parsing
- changing source view-model logic in `pilot_view_model.rs`
- widening class progression behavior
- adding spell, multiclass, prestige-class, archetype, or non-core logic

## TDD requirement
TDD is mandatory.

Execution order:
1. update the allowed test files first so the new race-semantic expectations are explicit
2. run the focused tests below and capture a real RED state for the intended change
3. implement the smallest code changes inside `support_state_matrix.rs` and/or `pilot_compute.rs`
4. rerun the focused tests to green
5. rerun the broader regression floor and then full `cargo test`

RED discipline:
- a vague compile failure is not enough by itself
- the failing tests must name the intended Human race-semantic or interaction-truth delta explicitly
- if the change requires touching any file outside the eight allowed write paths, stop and block instead of widening during RED

## Exact non-goals
This handoff does not authorize:
- any claim that the repo now supports the full seven-race by eleven-class roster
- any claim that any non-Human core race is already computed or supported
- any claim that Human size/speed, senses, or every racial trait family are now implemented unless the existing bounded evidence directly proves them
- any class progression broadening beyond the current Fighter level-1 / Fighter level-2+ blocked / Rogue blocked posture
- any spellcasting burden work
- any multiclassing, prestige-class, archetype, alternate-racial-trait, or non-core expansion work
- any edits under `/home/ubuntu/workspace/programs/codex/**`
- any edits under `/home/ubuntu/workspace/repos/codex/apps/desktop/**`
- any edits to `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_view_model.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
- any edits to any fixture file under `/home/ubuntu/workspace/repos/codex/tests/fixtures/**`
- any rewrite of `README.md`, `AGENTS.md`, `CLAUDE.md`, or governance docs as a substitute for repo evidence
- any normalization, deletion, or inclusion of the unrelated untracked files under `apps/desktop/src-tauri/gen/**`

## Forbidden widening / stop conditions
Stop and block the lane if any of these become true:
1. truthful completion requires editing `character_input.rs`, `pilot_view_model.rs`, `ge06_pilot_total_saves.rs`, `ge06_pilot_combat_baseline.rs`, or any fixture file
2. truthful completion requires creating a new race fixture family, alternate Human fixture, or non-Human fixture corpus under `tests/fixtures/**`
3. truthful completion requires promoting non-Human race rows above `Observed` or promoting any class row beyond the current E1 truth without new grounded evidence
4. truthful completion requires touching any path outside the eight allowed write paths
5. truthful completion depends on UI/workbench/reporting, desktop/Tauri generator cleanup, distribution/update work, or governance/doc rewrites
6. truthful completion cannot be launched honestly from current sanctioned `origin/develop` and instead would require some other unreviewed substrate or broadened authority surface

If a stop condition lands, do not improvise. Block the CODE lane with the exact broader surface now required.

## Verification commands
Run these at minimum.

### Preflight truth confirmation
```bash
git -C /home/ubuntu/workspace/repos/codex fetch origin --prune
git -C /home/ubuntu/workspace/repos/codex rev-parse origin/develop
gh pr view 35 --repo electricm0nk/codex --json number,state,mergedAt,mergeCommit,headRefName,baseRefName,url
gh pr view 37 --repo electricm0nk/codex --json number,state,mergedAt,mergeCommit,headRefName,baseRefName,url
git -C "$WT" branch --show-current
git -C "$WT" rev-parse HEAD
git -C "$WT" status --short
```

### Write-scope confirmation
```bash
git -C /home/ubuntu/workspace/repos/codex diff --name-only -- src/rules_core/support_state_matrix.rs src/rules_core/pilot_compute.rs tests/sd13_support_state_matrix.rs tests/ge06_pilot_input_contract.rs tests/ge06_pilot_selected_skill_modifiers.rs tests/ge06_pilot_headless_receipt.rs tests/ge06_failure_classifier.rs tests/ge06_pilot_view_model.rs
git -C /home/ubuntu/workspace/repos/codex diff --unified=0 -- src/rules_core/support_state_matrix.rs src/rules_core/pilot_compute.rs tests/sd13_support_state_matrix.rs tests/ge06_pilot_input_contract.rs tests/ge06_pilot_selected_skill_modifiers.rs tests/ge06_pilot_headless_receipt.rs tests/ge06_failure_classifier.rs tests/ge06_pilot_view_model.rs
```

### Focused regression floor
```bash
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test --test sd13_support_state_matrix
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test --test ge06_pilot_input_contract --test ge06_pilot_total_saves --test ge06_pilot_combat_baseline --test ge06_pilot_selected_skill_modifiers --test ge06_pilot_headless_receipt --test ge06_failure_classifier --test ge06_pilot_view_model
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test
```

Verification interpretation:
- `sd13_support_state_matrix` is the truth gate for the final exact row shape and the separation of race/class/interaction truth
- `ge06_pilot_input_contract` proves the exact Human chosen-input seam this slice is allowed to rely on
- `ge06_pilot_total_saves` and `ge06_pilot_combat_baseline` remain read-only regression sentinels because Human ability-bonus and bonus-feat pressure already propagate into those outputs
- `ge06_pilot_selected_skill_modifiers`, `ge06_pilot_headless_receipt`, `ge06_failure_classifier`, and `ge06_pilot_view_model` verify that any new race-linked explanation or blocker truth propagates honestly through the bounded downstream surfaces
- full `cargo test` is a smoke/regression sweep only; it does not by itself upgrade any breadth claim

## Success condition
This handoff is successful when a later governed Claude Code lane can implement a bounded Human race-semantic and interaction substrate on top of the live SD13-E1 PR-backed branch, inside the eight-path write surface, without counterfeiting non-Human race coverage or class breadth.

Until that governed CODE lane runs and leaves a durable receipt, this artifact remains documentary authorization only.