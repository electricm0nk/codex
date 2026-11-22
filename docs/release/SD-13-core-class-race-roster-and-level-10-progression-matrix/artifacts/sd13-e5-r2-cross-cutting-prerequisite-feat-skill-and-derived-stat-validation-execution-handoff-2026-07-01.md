---
title: SD13-E5-R2 Execution Handoff — Deterministic Human Fighter levels 1-3 prerequisite and invalid-choice blocking
handoff_id: HANDOFF-CODEX-SD13-E5-R2-HUMAN-FIGHTER-PREREQ-INVALID-CHOICE-2026-07-01
stc_id: STC-CODEX-SD-13
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: ready-for-claude-launch
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e5-r2-cross-cutting-prerequisite-feat-skill-and-derived-stat-validation-execution-handoff-2026-07-01.md
source_stc: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md
source_epic_breakdown: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md
source_readiness_closure: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e5-r1-cross-cutting-prerequisite-feat-skill-and-derived-stat-validation-readiness-closure-2026-07-01.md
selected_slice: SD13-E5 first code slice — deterministic Human Fighter levels 1-3 prerequisite and invalid-choice blocking with skill and derived-output pressure preserved
run_in: Claude Code only
code_authority: true
authority_dependencies:
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-design.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/acceptance-and-verification.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/level-10-progression-validation-contract.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e5-r1-cross-cutting-prerequisite-feat-skill-and-derived-stat-validation-readiness-closure-2026-07-01.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  expected_base_sha_at_creation: 454a92ed67578124d88232b130a832de6ed571df
  compare_base_ref: origin/develop
  compare_base_sha_at_creation: 454a92ed67578124d88232b130a832de6ed571df
  upstream_review_surface: https://github.com/electricm0nk/codex/pull/45
  recommended_branch: feat/sd13-e5-f9-human-fighter-prereq-invalid-choice
  pr_target: develop
allowed_write_scope:
  - src/rules_core/pilot_compute.rs
  - src/rules_core/support_state_matrix.rs
  - src/oracle_validation/support_state_matrix.rs
  - tests/sd13_support_state_matrix.rs
  - tests/ge06_pilot_headless_receipt.rs
  - tests/ge06_failure_classifier.rs
  - tests/ge06_pilot_view_model.rs
  - tests/sd13_fighter_prerequisite_invalid_choice_blocking.rs
forbidden_write_scope:
  - programs/codex/**
  - apps/desktop/**
  - apps/desktop/src-tauri/gen/**
  - Cargo.toml
  - Cargo.lock
  - src/lib.rs
  - src/rules_core/character_input.rs
  - src/rules_core/pilot_view_model.rs
  - tests/ge06_pilot_input_contract.rs
  - tests/ge06_pilot_total_saves.rs
  - tests/ge06_pilot_combat_baseline.rs
  - tests/ge06_pilot_selected_skill_modifiers.rs
  - tests/sd13_fighter_level2_level3_progression.rs
  - tests/sd13_hybrid_level1_chassis_baseline.rs
  - tests/sd13_sorcerer_level1_spell_baseline.rs
  - tests/fixtures/rules_core/**
  - AGENTS.md
  - CLAUDE.md
completion_class: pr-created
reviewed_at: 2026-07-01
---

# SD13-E5-R2 Execution Handoff — Deterministic Human Fighter levels 1-3 prerequisite and invalid-choice blocking

## Status
This is the stage-specific code-authorizing brief for the first honest repo-facing SD13-E5 lane.

It grants code authority for one bounded slice only: preserve the accepted deterministic Human Fighter level-1/2/3 computed path exactly as it exists on accepted `origin/develop`, while converting non-canonical feat-choice deviations on that same seam into explicit claim-blocking evidence instead of fabricated legal-build truth. Skill and derived-output pressure must remain visible and preserved on the accepted canonical path; hybrid and spell-bearing class-family baselines remain regression boundaries only.

Board-visible verdict:
- this handoff is ready for a governed Claude Code lane now
- this artifact is documentary authorization only until the later CODE lane executes it
- later implementation truth exists only if the CODE lane leaves a durable `claude-execution-receipt`

## Run in
Claude Code only.

Do not substitute Hermes file editing or another coding harness as the primary implementation path. If Claude Code cannot be launched truthfully, block the downstream CODE lane instead of coding through Hermes.

## Core problem
Accepted repo truth on 2026-07-01 is now sharp enough to expose the first honest cross-cutting validation seam, but the repo still lacks any general feat-effect or prerequisite engine:
- sanctioned base truth is `origin/develop` at `454a92ed67578124d88232b130a832de6ed571df`, the merge commit for PR `#45`
- PR `#45` is merged: `https://github.com/electricm0nk/codex/pull/45`
- the shared checkout at `/home/ubuntu/workspace/repos/codex` is not that sanctioned base right now; after fresh `git fetch origin --prune` it still reports branch `feat/sd13-e6-f11-support-state-debt-presentation` at `122de6a60609d9452de53c6d3ad406aeb81c2a82`, with upstream tracking gone, local modification in `README.md`, and untracked `apps/desktop/src-tauri/gen/` content
- the accepted evidence worktree `/home/ubuntu/workspace/worktrees/codex-sd13-e5-verdict` is clean detached `HEAD` at `454a92ed67578124d88232b130a832de6ed571df`
- `src/rules_core/pilot_compute.rs` already names the exact deterministic Human/Fighter choice seams: `choice:human_bonus_feat`, `choice:human_ability_bonus`, `choice:fighter_bonus_feat`, and `choice:fighter_bonus_feat_2`
- the same live compute seam already preserves selected-skill outputs, propagated explanations, and the level-3 armor-training pressure, but still says plainly that it grounds no general feat-effect or prerequisite engine
- `src/rules_core/support_state_matrix.rs` already holds `class.fighter.levels_2_10` as `Partial` / `Computed` and explicitly names the missing general feat-effect/prerequisite engine inside the blocker note
- accepted hybrid and Sorcerer rows remain bounded regression truth only: Paladin/Ranger and Sorcerer are `Blocked` / `Computed`, not positive support surfaces for the first E5 slice

The decisive move is not “build general feat legality.” It is smaller and honest: preserve the canonical deterministic Human Fighter choice selections exactly, and when that known seam is mutated away from accepted truth, claim-block the result with structured diagnostics instead of pretending unsupported alternative choices are legal.

## Why SD13-F9 is the first truthful SD13-E5 slice
The readiness closure deliberately refused to counterfeit this narrowing decision. That decision is frozen here.

`SD13-F9` is first because it is the smallest slice that repairs the exact remaining lie-risk in accepted repo truth:
1. accepted code already computes the canonical Human Fighter level-1/2/3 path, including the level-2 bonus-feat seam and level-3 armor-training effect on selected skills
2. accepted code already propagates explanations, receipts, failure classification, and view-model state; those surfaces do not need a new subsystem before the first E5 slice can be honest
3. what is still unsafe is alternative choice pressure on that seam: the repo can name the choice slots, but it cannot yet validate broader feat legality or broader prerequisite truth across arbitrary alternatives
4. authoring `SD13-F10` explanation-pressure work before `SD13-F9` would counterfeit legality, because alternative feat choices could still appear inside a computed path without a truthful claim-blocking boundary
5. hybrid and spell-bearing rows are exactly the wrong place to start: they remain accepted regression boundaries, not the first positive cross-cutting support surface

This handoff therefore freezes the first code-authorizing `SD13-F9` tranche as:
- race/class seam: deterministic Human Fighter levels 1-3 only
- positive path: preserve the accepted canonical selections and outputs exactly
- negative path: block non-canonical feat-choice deviations on that same seam with structured claim-blocking diagnostics
- support-state posture: no broad support promotion; no general feat/prerequisite engine claim

## Objective
Implement the smallest truthful SD13-E5 slice.

The result must prove all of the following:
1. the accepted deterministic Human Fighter level-1/2/3 fixtures still compute exactly as they do on accepted `origin/develop`, with no new claim-blocking diagnostics on the canonical path
2. non-canonical mutations of the already-grounded feat-choice seam no longer ride through as if they were legal computed builds
3. blocked choice mutations become explicit claim-blocking evidence on the live rules-core seam rather than vague “not ready” folklore
4. the blocked posture propagates through headless receipt, failure classifier, and view-model surfaces honestly
5. `src/rules_core/support_state_matrix.rs` and `src/oracle_validation/support_state_matrix.rs` remain intentionally aligned instead of drifting again
6. accepted Human race truth, Human interaction truth, Fighter levels-2-10 bounded progression truth, selected-skill pressure, Paladin/Ranger hybrid blocked baselines, and Sorcerer blocked baseline remain intact
7. the slice stops before any general feat engine, general prerequisite engine, non-canonical ability-bonus support, non-Fighter positive support, or hybrid/spell-bearing uplift

## Why this route is authorized now
This route is authorized because accepted repo truth already exposes both the positive seam and the exact unsupported choice-pressure gap:
- `tests/ge06_pilot_input_contract.rs` already freezes the accepted level-1 feat/choice set identities
- `tests/sd13_fighter_level2_level3_progression.rs` already proves the level-2 bonus-feat seam and the level-3 armor-training seam on the deterministic Human Fighter path
- `tests/ge06_pilot_selected_skill_modifiers.rs` already proves that the accepted derived skill outputs are real computed outputs, not future aspirations
- `tests/ge06_pilot_headless_receipt.rs`, `tests/ge06_failure_classifier.rs`, and `tests/ge06_pilot_view_model.rs` already prove downstream propagation surfaces for blocked/computed truth
- `epic-breakdown.md` explicitly separates `SD13-F9` prerequisite/invalid-choice blocking from `SD13-F10` explanation-pressure and derived-output work
- `src/rules_core/support_state_matrix.rs` already names the missing general feat-effect/prerequisite engine in the Fighter levels-2-10 row, so this slice can narrow around that burden without inventing a broader engine contract

What is still not authorized:
- any claim that Codex now has a general feat legality or prerequisite engine
- any support for alternative Human ability-bonus targets beyond the accepted canonical deterministic selection
- any non-Human race support or non-Fighter positive support
- any level-4+ Fighter uplift
- any Paladin, Ranger, Sorcerer, Bard, Cleric, Druid, or Wizard positive-support work
- any parser/input-model widening or new fixture library surface
- any governance/program-file edits or any desktop/Tauri work

## Target repo / workdir
```text
repo:    /home/ubuntu/workspace/repos/codex
workdir: /home/ubuntu/workspace/repos/codex
```

## Branch policy and launch substrate
Do not launch from the shared checkout as-is.

At handoff creation time, live shell facts were:
- `git rev-parse origin/develop` -> `454a92ed67578124d88232b130a832de6ed571df`
- current local branch in the shared checkout -> `feat/sd13-e6-f11-support-state-debt-presentation`
- current local `HEAD` in the shared checkout -> `122de6a60609d9452de53c6d3ad406aeb81c2a82`
- `git status --short --branch` in the shared checkout -> upstream `[gone]`, `M README.md`, and `?? apps/desktop/src-tauri/gen/`

Launch this slice from a fresh isolated worktree off accepted `origin/develop` instead:

```bash
git -C /home/ubuntu/workspace/repos/codex fetch origin --prune
WT=/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice
git -C /home/ubuntu/workspace/repos/codex worktree add -b feat/sd13-e5-f9-human-fighter-prereq-invalid-choice "$WT" origin/develop
cd "$WT"
```

If `feat/sd13-e5-f9-human-fighter-prereq-invalid-choice` already exists, reuse it only after confirming:
- it still belongs exclusively to this slice
- it still starts from sanctioned `origin/develop` truth
- it carries no unrelated changes outside the bounded write scope

Record the actual launch branch, base SHA, commit handles, and PR handle in the final `claude-execution-receipt`.

## Exact required reads before coding
Read these first, in order:
1. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/CLAUDE.md`
2. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/AGENTS.md`
3. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e5-r2-cross-cutting-prerequisite-feat-skill-and-derived-stat-validation-execution-handoff-2026-07-01.md`
4. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e5-r1-cross-cutting-prerequisite-feat-skill-and-derived-stat-validation-readiness-closure-2026-07-01.md`
5. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/README.md`
6. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md`
7. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md`
8. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-design.md`
9. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/acceptance-and-verification.md`
10. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md`
11. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
12. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/level-10-progression-validation-contract.md`
13. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
14. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/src/rules_core/pilot_compute.rs`
15. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/src/rules_core/support_state_matrix.rs`
16. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/src/oracle_validation/support_state_matrix.rs`
17. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/tests/ge06_pilot_input_contract.rs`
18. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/tests/ge06_pilot_total_saves.rs`
19. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/tests/ge06_pilot_combat_baseline.rs`
20. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/tests/ge06_pilot_selected_skill_modifiers.rs`
21. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/tests/ge06_pilot_headless_receipt.rs`
22. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/tests/ge06_failure_classifier.rs`
23. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/tests/ge06_pilot_view_model.rs`
24. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/tests/sd13_fighter_level2_level3_progression.rs`
25. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/tests/sd13_hybrid_level1_chassis_baseline.rs`
26. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/tests/sd13_sorcerer_level1_spell_baseline.rs`
27. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/tests/sd13_support_state_matrix.rs`
28. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
29. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/tests/fixtures/rules_core/pf1_human_fighter_level2_sd13_deterministic_input.txt`
30. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/tests/fixtures/rules_core/pf1_human_fighter_level3_sd13_deterministic_input.txt`
31. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/tests/fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt`
32. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/tests/fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt`
33. `/home/ubuntu/workspace/worktrees/codex-sd13-e5-f9-prereq-invalid-choice/tests/fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt`

Use them as bounded authority surfaces, not as permission to widen scope.

## Exact allowed write scope
You may create or modify only these repo paths:

```text
/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs
/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs
/home/ubuntu/workspace/repos/codex/src/oracle_validation/support_state_matrix.rs
/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs
/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_headless_receipt.rs
/home/ubuntu/workspace/repos/codex/tests/ge06_failure_classifier.rs
/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_view_model.rs
/home/ubuntu/workspace/repos/codex/tests/sd13_fighter_prerequisite_invalid_choice_blocking.rs
```

Write-scope interpretation:
- `pilot_compute.rs` is the only live rules-core seam authorized to preserve the canonical deterministic Human Fighter feat/choice path and claim-block non-canonical deviations on that same seam
- `support_state_matrix.rs` is the authoritative live matrix carrier authorized only for bounded note/alignment changes required by this slice
- `src/oracle_validation/support_state_matrix.rs` is intentionally in scope so this slice cannot widen the rules-core truth while leaving the oracle mirror contradictory
- `sd13_support_state_matrix.rs` must pin the exact final matrix posture after this slice
- `ge06_pilot_headless_receipt.rs`, `ge06_failure_classifier.rs`, and `ge06_pilot_view_model.rs` are the only downstream propagation surfaces authorized for adjustment if the new blocked-vs-computed posture changes what those consumers must report
- `sd13_fighter_prerequisite_invalid_choice_blocking.rs` is the dedicated tranche-specific proof surface for this handoff
- no new fixture file is authorized in this slice; invalid-choice and prerequisite-pressure cases must be expressed by safe in-test mutation of the accepted Fighter fixtures already present

No other repo file is in write scope.

## Read-only grounding seams
These files are grounding truth for this lane and may not be edited under this handoff:
- `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
- `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_view_model.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_selected_skill_modifiers.rs`
- `/home/ubuntu/workspace/repos/codex/tests/sd13_fighter_level2_level3_progression.rs`
- `/home/ubuntu/workspace/repos/codex/tests/sd13_hybrid_level1_chassis_baseline.rs`
- `/home/ubuntu/workspace/repos/codex/tests/sd13_sorcerer_level1_spell_baseline.rs`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level2_sd13_deterministic_input.txt`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level3_sd13_deterministic_input.txt`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt`
- program-level packet docs listed above

If truthful completion would require write authority over any of these, stop and block instead of widening silently.

## Contract to implement
Implement one bounded `SD13-F9` tranche over the already-accepted Human/Fighter deterministic seam.

### Exact class/level surface under claim
This slice may claim only this bounded positive seam:
- `race:human`
- `class:fighter:1`
- `class:fighter:2`
- `class:fighter:3`

No non-Human race, no non-Fighter positive support, and no level-4+ Fighter surface may be promoted by this handoff.

### Exact choice-pressure surface under claim
The slice may preserve and validate only these already-grounded canonical selections:
- `choice:level_1_character_feat` -> `feat:power_attack`
- `choice:human_bonus_feat` -> `feat:dodge`
- `choice:fighter_bonus_feat` -> `feat:weapon_focus:weapon:longsword`
- `choice:fighter_bonus_feat_2` -> `feat:toughness`

The accepted Human ability-bonus target remains read-only grounding truth in this slice:
- `choice:human_ability_bonus` -> `ability:strength`

Do not widen into alternate Human ability-bonus target support here. The first truthful E5 slice is feat-choice legality pressure on the accepted deterministic seam, not general alternate-stat support.

### Exact burden under claim
The slice may prove only these bounded things:
1. the canonical selections above remain the only accepted computed feat-choice path for the deterministic Human Fighter level-1/2/3 seam
2. non-canonical mutations of those feat-choice selections become explicit claim-blocking evidence instead of silently preserving computed success
3. blocked choice mutations name the offending choice seam and the fact that alternative feat/prerequisite legality is outside current bounded proof without a general engine
4. the accepted selected-skill and derived-output pressure remains intact on the canonical path and is not counterfeited on the blocked path
5. `class.fighter.levels_2_10` remains bounded and not `Supported`
6. the Human interaction row does not get weakened or hidden
7. Paladin, Ranger, and Sorcerer rows remain regression boundaries only

### Required result shape
The final implementation must satisfy all of the following:
1. the accepted deterministic Human Fighter level-1/2/3 fixtures still produce their current computed receipt posture with no claim-blocking diagnostics and no regression in selected-skill outputs
2. at least one non-canonical level-1 feat-choice mutation and at least one non-canonical level-2 bonus-feat mutation become claim-blocking on the live rules-core seam
3. the blocked path must identify the exact offending choice set in diagnostics or explanation-adjacent evidence; “generic unsupported build” alone is not enough
4. the blocked path must not fabricate a computed success posture for unsupported alternative feat choices
5. headless receipt status, primary-owner classification, and view-model state must reflect the blocked posture honestly
6. `class.fighter.levels_2_10` must remain below `Supported`, and any matrix note change must stay bounded to this choice-pressure truth rather than implying general feat legality
7. `interaction.human_bonus_feat_ability_bonus.pilot_pressure` must remain visible and must not be downgraded below accepted `Partial` / `Computed`
8. Paladin, Ranger, and Sorcerer rows must remain exactly their current accepted states
9. no new fixture file may be introduced

### Deterministic fixture posture
No new deterministic fixture file is authorized in this slice.

The tranche-specific proof file must derive its invalid-choice cases by safe in-test mutation of the accepted Fighter fixtures already present:
- `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
- `tests/fixtures/rules_core/pf1_human_fighter_level2_sd13_deterministic_input.txt`
- `tests/fixtures/rules_core/pf1_human_fighter_level3_sd13_deterministic_input.txt`

That means:
- preserve the canonical fixtures as read-only evidence
- express invalid-choice cases inside the new proof test by mutating only the necessary chosen feat/choice line(s)
- do not create a parallel fixture library for unsupported alternatives

If truthful completion would require a new fixture file, stop and block.

## TDD requirement
TDD is mandatory.

Execution order:
1. create `tests/sd13_fighter_prerequisite_invalid_choice_blocking.rs` first
2. express non-canonical choice-pressure cases by in-test mutation of the accepted Fighter fixtures; do not create new fixture files
3. run the targeted new test command below and capture a real RED state for the intended invalid-choice delta
4. implement the smallest code changes inside `pilot_compute.rs`, `support_state_matrix.rs`, and the oracle mirror only as needed
5. rerun the targeted test to green
6. rerun the focused regression floor
7. rerun full `cargo test`

RED discipline:
- a vague compile failure is not enough by itself
- the failing tests must explicitly show that the current repo does not yet claim-block the bounded non-canonical choice mutations in the intended way
- if RED reveals a need to touch any file outside the exact allowed write scope, stop and block instead of widening

Minimum RED assertions in `tests/sd13_fighter_prerequisite_invalid_choice_blocking.rs`:
1. the canonical deterministic Human Fighter level-1/2/3 fixtures still produce computed truth on the accepted path
2. mutating one accepted level-1 feat-choice slot away from its canonical selection yields blocked truth instead of computed truth
3. mutating `choice:fighter_bonus_feat_2` away from `feat:toughness` yields blocked truth instead of computed truth
4. the blocked path names the offending choice-set identity rather than hiding the reason in a generic failure bucket
5. the blocked path propagates through receipt/classifier/view-model surfaces honestly
6. no assertion depends on a new fixture file or on a general feat/prerequisite engine existing

## Exact non-goals
This handoff does not authorize:
- any claim that Codex now has a general feat legality engine
- any claim that Codex now has a general prerequisite engine
- any promotion of `class.fighter.levels_2_10` to `Supported`
- any non-canonical Human ability-bonus target support
- any non-Human race support or non-Fighter positive support
- any level-4+ Fighter progression claim
- any Paladin, Ranger, Sorcerer, Bard, Cleric, Druid, Wizard, Rogue, Monk, or Barbarian positive-support work
- any broad `SD13-F10` explanation-pressure tranche
- any parser/input-model widening
- any new deterministic fixture file
- any edits under `/home/ubuntu/workspace/programs/codex/**`
- any edits under `/home/ubuntu/workspace/repos/codex/apps/desktop/**`
- any edits to `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_view_model.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_selected_skill_modifiers.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/tests/sd13_fighter_level2_level3_progression.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/tests/sd13_hybrid_level1_chassis_baseline.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/tests/sd13_sorcerer_level1_spell_baseline.rs`
- any edits to any file under `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/**`
- any rewrite of `README.md`, `AGENTS.md`, `CLAUDE.md`, or governance docs as a substitute for repo evidence
- any normalization, deletion, or inclusion of unrelated `apps/desktop/src-tauri/gen/**` content

## Forbidden widening / stop conditions
Stop and block the CODE lane if any of these become true:
1. truthful completion requires editing `character_input.rs`, `pilot_view_model.rs`, `ge06_pilot_input_contract.rs`, `ge06_pilot_total_saves.rs`, `ge06_pilot_combat_baseline.rs`, `ge06_pilot_selected_skill_modifiers.rs`, `sd13_fighter_level2_level3_progression.rs`, `sd13_hybrid_level1_chassis_baseline.rs`, or `sd13_sorcerer_level1_spell_baseline.rs`
2. truthful completion requires any new fixture file or any edit to an existing fixture file
3. truthful completion requires support for alternate Human ability-bonus targets, non-Human races, non-Fighter positive support, or level-4+ Fighter progression
4. truthful completion requires a general feat-effect engine, general prerequisite engine, parser/input-model widening, or broad `SD13-F10` explanation-pressure work
5. truthful completion requires touching any path outside the exact allowed write scope
6. truthful completion would promote any support row beyond the bounded posture authorized here
7. truthful completion depends on UI/workbench/reporting work, desktop/Tauri cleanup, distribution/update work, or governance/doc rewrites
8. the repo cannot be refreshed to a clean `origin/develop`-based execution worktree
9. any mandatory verification command below fails after the bounded change

If a stop condition lands, do not improvise. Block the CODE lane with the exact broader surface now required.

## Verification commands
Run these at minimum.

### Preflight truth confirmation
```bash
git -C /home/ubuntu/workspace/repos/codex fetch origin --prune
git -C /home/ubuntu/workspace/repos/codex rev-parse origin/develop
gh pr view 45 --repo electricm0nk/codex --json number,state,mergedAt,mergeCommit,headRefName,baseRefName,url,title
git -C "$WT" branch --show-current
git -C "$WT" rev-parse HEAD
git -C "$WT" status --short
```

### Write-scope confirmation
```bash
git -C "$WT" diff --name-only -- src/rules_core/pilot_compute.rs src/rules_core/support_state_matrix.rs src/oracle_validation/support_state_matrix.rs tests/sd13_support_state_matrix.rs tests/ge06_pilot_headless_receipt.rs tests/ge06_failure_classifier.rs tests/ge06_pilot_view_model.rs tests/sd13_fighter_prerequisite_invalid_choice_blocking.rs
git -C "$WT" diff --unified=0 -- src/rules_core/pilot_compute.rs src/rules_core/support_state_matrix.rs src/oracle_validation/support_state_matrix.rs tests/sd13_support_state_matrix.rs tests/ge06_pilot_headless_receipt.rs tests/ge06_failure_classifier.rs tests/ge06_pilot_view_model.rs tests/sd13_fighter_prerequisite_invalid_choice_blocking.rs
```

### Focused regression floor
```bash
cd "$WT" && . "$HOME/.cargo/env" && cargo test --test sd13_fighter_prerequisite_invalid_choice_blocking
cd "$WT" && . "$HOME/.cargo/env" && cargo test --test ge06_pilot_input_contract --test ge06_pilot_total_saves --test ge06_pilot_combat_baseline --test ge06_pilot_selected_skill_modifiers --test ge06_pilot_headless_receipt --test ge06_failure_classifier --test ge06_pilot_view_model --test sd13_fighter_level2_level3_progression --test sd13_hybrid_level1_chassis_baseline --test sd13_sorcerer_level1_spell_baseline --test sd13_support_state_matrix
cd "$WT" && . "$HOME/.cargo/env" && cargo test
```

Verification interpretation:
- `sd13_fighter_prerequisite_invalid_choice_blocking` is the mandatory tranche-specific RED/GREEN proof surface
- the GE06 regression tests preserve the accepted Human/Fighter deterministic seam the tranche must not disturb
- `sd13_fighter_level2_level3_progression` preserves the accepted bounded level-2 bonus-feat seam and level-3 armor-training seam while this slice adds choice-pressure honesty
- `sd13_hybrid_level1_chassis_baseline` and `sd13_sorcerer_level1_spell_baseline` are regression boundaries proving hybrid and spell-bearing families remain blocked/computed baselines rather than counterfeit positive support
- `sd13_support_state_matrix` is the control-plane truth gate for the final matrix posture
- full `cargo test` is a smoke/regression sweep only; it does not by itself upgrade any SD13 breadth claim

## Expected completion class
This lane is complete only at `pr-created` truth:
- fresh branch launched from accepted `origin/develop`
- bounded changes confined to the exact allowed write scope
- branch pushed to `origin`
- normal PR opened against `develop`
- durable Claude execution receipt attached to the governed CODE card

This handoff does not authorize merge to `develop` or `main`.

## Required Claude receipt
Before the downstream CODE card completes, add a durable `claude-execution-receipt` comment that records:
- exact handoff path
- invocation mode
- repo/workdir
- launch branch and base SHA
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
- broadening into later SD13-E5 slices, SD13-E6 diagnostics/reporting work, or any adjacent lane
- any write outside the exact allowed scope

Stop at verified `pr-created` state and return control through the governed review surface.
