---
title: SD13-E3-R2 Execution Handoff — Fighter levels 2-3 milestone progression uplift with Rogue negative-control preservation
handoff_id: HANDOFF-CODEX-SD13-E3-R2-FIGHTER-LEVELS-2-3-MILESTONES-2026-07-01
stc_id: STC-CODEX-SD-13
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: ready-for-claude-launch
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e3-r2-martial-and-skill-driven-level-10-progression-execution-handoff-2026-07-01.md
source_stc: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md
source_epic_breakdown: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md
source_readiness_closure: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e3-r1-martial-and-skill-driven-level-10-progression-readiness-closure-2026-07-01.md
selected_slice: SD13-E3 first code slice — Fighter levels 2-3 milestone progression uplift with Rogue negative-control preservation and Human race-seam regression protection
run_in: Claude Code only
code_authority: true
authority_dependencies:
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-design.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/level-10-progression-validation-contract.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e2-r1-core-race-semantic-readiness-closure-2026-06-30.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e3-r1-martial-and-skill-driven-level-10-progression-readiness-closure-2026-07-01.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  expected_base_sha_at_creation: 25765e8c2cb4ed50bd936183b24a2f2189977bc0
  compare_base_ref: origin/develop
  compare_base_sha_at_creation: 25765e8c2cb4ed50bd936183b24a2f2189977bc0
  upstream_review_surface: https://github.com/electricm0nk/codex/pull/41
  recommended_branch: feat/sd13-e3-f5-fighter-level2-3-milestones
  pr_target: develop
allowed_write_scope:
  - src/rules_core/pilot_compute.rs
  - src/rules_core/support_state_matrix.rs
  - tests/sd13_support_state_matrix.rs
  - tests/ge06_pilot_total_saves.rs
  - tests/ge06_pilot_combat_baseline.rs
  - tests/ge06_pilot_selected_skill_modifiers.rs
  - tests/ge06_pilot_headless_receipt.rs
  - tests/ge06_failure_classifier.rs
  - tests/ge06_pilot_view_model.rs
  - tests/sd13_fighter_level2_level3_progression.rs
  - tests/fixtures/rules_core/pf1_human_fighter_level2_sd13_deterministic_input.txt
  - tests/fixtures/rules_core/pf1_human_fighter_level3_sd13_deterministic_input.txt
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
  - tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
  - tests/fixtures/rules_core/pf1_human_fighter_level1_minimal_character_input.txt
  - tests/ge08_*
  - AGENTS.md
  - CLAUDE.md
completion_class: pr-created
reviewed_at: 2026-07-01
---

# SD13-E3-R2 Execution Handoff — Fighter levels 2-3 milestone progression uplift with Rogue negative-control preservation

## Status
This is the stage-specific code-authorizing brief for the first honest repo-facing SD13-E3 lane.

It grants code authority for one bounded slice only: widen the current Human Fighter level-1 deterministic seam into a Human Fighter levels-2-and-3 milestone seam, preserve Rogue as an explicit blocked negative control, and keep the accepted Human race explanation/diagnostic truth intact across receipt, classifier, and view-model surfaces.

Board-visible verdict:
- this handoff is ready for a governed Claude Code lane now
- this artifact is documentary authorization only until the later CODE lane executes it
- later implementation truth exists only if the CODE lane leaves a durable `claude-execution-receipt`

## Run in
Claude Code only.

Do not substitute Hermes file editing or another coding harness as the primary implementation path. If Claude Code cannot be launched truthfully, block the downstream CODE lane instead of coding through Hermes.

## Core problem
Accepted repo truth on 2026-07-01 is still narrower than the epic name:
- sanctioned base truth is `origin/develop` at `25765e8c2cb4ed50bd936183b24a2f2189977bc0`, the merge commit for PR `#41`
- the shared checkout at `/home/ubuntu/workspace/repos/codex` is not that sanctioned base right now; after `git fetch origin --prune` it still reports branch `feat/sd13-e6-f11-support-state-debt-presentation` at `122de6a60609d9452de53c6d3ad406aeb81c2a82` with untracked `apps/desktop/src-tauri/gen/` content
- current accepted code still hard-gates the live computed class seam at Fighter level 1 and still claim-blocks both Rogue level 1 and Fighter level 2 in the GE-06 regression surface
- the accepted Human race seam is now explicit and must be preserved, but it does not authorize broad martial closure, spell burden closure, or a fake “level-10 supported” claim

The decisive move is not “implement all martial and skill-driven classes.” It is smaller and honest: prove the first post-pilot Fighter milestone tranche only.

Why levels 2 and 3 are the first truthful tranche:
1. level 2 is the first post-pilot chassis widening, where total saves, combat posture, and selected-skill surfaces must stop treating every non-level-1 Fighter as blocked
2. level 2 also forces the first bounded proof that Fighter bonus-feat progression can advance without pretending a general feat engine exists
3. level 3 is the first Fighter armor-training milestone, which creates a real derived-output seam for armor class and armor-check-pressure on selected skills
4. stopping at level 3 keeps this first slice below later burdens that would otherwise counterfeit broader closure: repeated bonus-feat cadence, level-4 ability-score progression posture, weapon training, later armor-training ranks, and any claim about levels 4-10 as a finished surface

## Objective
Implement the smallest truthful SD13-E3 progression slice.

The result must prove all of the following:
1. `src/rules_core/pilot_compute.rs` can compute and explain bounded Human Fighter levels 2 and 3 deterministic outputs without fabricating levels 4-10 truth
2. `src/rules_core/support_state_matrix.rs` can move `class.fighter.levels_2_10` from a fully blocked posture to a partial posture only if the row still names exactly what remains unproven after this slice
3. Rogue remains an explicit blocked negative-control seam; widening Fighter must not silently make Rogue look interchangeable with Fighter
4. the accepted Human race seam and named Human interaction pressure remain intact; no regression may weaken the already-accepted Human explanation/diagnostic truth
5. downstream receipt, failure-classifier, and view-model surfaces remain aligned with the widened but still bounded Fighter truth
6. the slice stops before level-4-and-beyond burdens, spell burden, non-Fighter positive support, or general engine claims

## Why this route is authorized now
This route is authorized because accepted repo truth already exposes one narrow expansion surface and one honest blocker surface:
- `src/rules_core/pilot_compute.rs` is already the live compute seam for Fighter base chassis, total saves, baseline combat, selected skills, headless receipt, and blocker diagnostics
- `src/rules_core/support_state_matrix.rs` and `tests/sd13_support_state_matrix.rs` already carry explicit rows for `class.fighter.level_1_pilot`, `class.fighter.levels_2_10`, and `class.rogue.bounded_progression`
- `tests/ge06_pilot_total_saves.rs` and `tests/ge06_pilot_combat_baseline.rs` explicitly claim-block Fighter level 2 today, which means the repo already names the first truthful move instead of leaving it folkloric
- `tests/ge06_pilot_selected_skill_modifiers.rs` already proves that armor-check pressure is part of the bounded surface, making level-3 armor training a legitimate derived-output seam
- `tests/ge06_pilot_headless_receipt.rs`, `tests/ge06_failure_classifier.rs`, and `tests/ge06_pilot_view_model.rs` already enforce downstream propagation of bounded truth and blockers
- the accepted Human race semantics from SD13-E2 are already live inside the current bounded compute path and must now be preserved while Fighter widens

What is still not authorized:
- any positive support claim for Barbarian, Monk, Rogue, Paladin, Ranger, Bard, Cleric, Druid, Sorcerer, or Wizard
- any claim that Fighter is now solved through level 10
- any widening into level 4+ Fighter burden, weapon training, spellcasting, multiclassing, archetypes, or non-core scope
- any change that requires new parser/input-shape authority, a rewritten chosen-input model, or editing accepted governance/program files

## Target repo / workdir
```text
repo:    /home/ubuntu/workspace/repos/codex
workdir: /home/ubuntu/workspace/repos/codex
```

## Branch policy and launch substrate
Do not launch from the shared checkout as-is.

At handoff creation time, live shell facts were:
- `git rev-parse origin/develop` -> `25765e8c2cb4ed50bd936183b24a2f2189977bc0`
- current local branch in the shared checkout -> `feat/sd13-e6-f11-support-state-debt-presentation`
- current local `HEAD` in the shared checkout -> `122de6a60609d9452de53c6d3ad406aeb81c2a82`
- `git status --short` in the shared checkout -> `?? apps/desktop/src-tauri/gen/`

Launch this slice from a fresh isolated worktree off accepted `origin/develop` instead:

```bash
git -C /home/ubuntu/workspace/repos/codex fetch origin --prune
WT=/home/ubuntu/workspace/worktrees/codex-sd13-e3-f5-level2-3
git -C /home/ubuntu/workspace/repos/codex worktree add -b feat/sd13-e3-f5-fighter-level2-3-milestones "$WT" origin/develop
cd "$WT"
```

If `feat/sd13-e3-f5-fighter-level2-3-milestones` already exists, reuse it only after confirming:
- it still belongs exclusively to this slice
- it still starts from sanctioned `origin/develop` truth
- it does not carry unrelated changes outside the bounded write scope

Record the actual launch branch, base SHA, commit handles, and PR handle in the final `claude-execution-receipt`.

## Exact required reads before coding
Read these first, in order:
1. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
2. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
3. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e3-r2-martial-and-skill-driven-level-10-progression-execution-handoff-2026-07-01.md`
4. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e3-r1-martial-and-skill-driven-level-10-progression-readiness-closure-2026-07-01.md`
5. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md`
6. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md`
7. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md`
8. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-design.md`
9. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
10. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/level-10-progression-validation-contract.md`
11. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
12. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e2-r1-core-race-semantic-readiness-closure-2026-06-30.md`
13. `/home/ubuntu/workspace/repos/codex/README.md`
14. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
15. `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
16. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
17. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
18. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
19. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_selected_skill_modifiers.rs`
20. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_headless_receipt.rs`
21. `/home/ubuntu/workspace/repos/codex/tests/ge06_failure_classifier.rs`
22. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_view_model.rs`
23. `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`
24. `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`

Use them as bounded authority surfaces, not as permission to widen scope.

## Exact allowed write scope
You may create or modify only these repo paths:

```text
/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs
/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs
/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs
/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs
/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs
/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_selected_skill_modifiers.rs
/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_headless_receipt.rs
/home/ubuntu/workspace/repos/codex/tests/ge06_failure_classifier.rs
/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_view_model.rs
/home/ubuntu/workspace/repos/codex/tests/sd13_fighter_level2_level3_progression.rs
/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level2_sd13_deterministic_input.txt
/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level3_sd13_deterministic_input.txt
```

Write-scope interpretation:
- `pilot_compute.rs` is the only live compute seam authorized to widen bounded Fighter progression truth
- `support_state_matrix.rs` is the only control-plane truth surface authorized to reclassify the Fighter levels-2-10 row after bounded new proof exists
- the listed GE-06 tests are regression sentinels and may be updated only where the bounded levels-2-and-3 truth changes what they should explicitly compute or block
- `sd13_support_state_matrix.rs` must pin the exact final matrix posture after this slice
- `sd13_fighter_level2_level3_progression.rs` is the dedicated new TDD proof surface for the tranche this handoff authorizes
- the two new deterministic fixtures are the only new fixture files authorized in this slice

No other repo file is in write scope.

## Read-only grounding seams
These files are grounding truth for this lane and may not be edited under this handoff:
- `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
- `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_view_model.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
- program-level packet docs listed above

If truthful completion would require write authority over any of these, stop and block instead of widening silently.

## Contract to implement
Implement one bounded Fighter milestone tranche over the already-accepted Human deterministic pilot seam.

### Exact milestone levels under claim
This slice may claim only these new Fighter milestone levels:
- `class:fighter:2`
- `class:fighter:3`

No other new Fighter level may be promoted by this handoff.

### Exact burden under claim
The slice may prove only these additional burdens:
1. Fighter base attack and base save progression for levels 2 and 3
2. total save, baseline combat, and selected-skill outputs for levels 2 and 3 where the deterministic Human loadout still grounds them
3. bounded preservation of a level-2 Fighter bonus-feat progression seam
4. bounded first armor-training seam at level 3 where that seam materially affects combat or selected-skill outputs
5. propagation of the widened bounded truth through headless receipt, failure-classifier, and view-model surfaces

### Required result shape
The final implementation must satisfy all of the following:
1. Human Fighter level 2 is no longer blanket-blocked when the deterministic posture for this slice is present
2. Human Fighter level 3 is no longer blanket-blocked when the deterministic posture for this slice is present
3. level-2 bonus-feat progression is proven only as far as this deterministic slice can ground it; this handoff does not authorize a general feat-effect or prerequisite engine
4. level-3 armor-training truth is explicit and bounded; if it changes armor class or armor-check pressure, the explanations must say so directly
5. `class.fighter.levels_2_10` may move from `Blocked` / `Computed` to `Partial` / `Computed` only if its blocker note explicitly names what remains out of proof after this slice, including levels 4-10 and later Fighter burden families
6. `class.rogue.bounded_progression` must remain `Blocked` / `Computed` and must continue to act as a negative-control seam
7. the accepted Human row and Human interaction row must not be weakened, relabeled downward, or hidden
8. non-Fighter positive-support rows must remain exactly where accepted `origin/develop` leaves them

### Deterministic fixture posture
The two new fixtures must remain tightly bounded:
- both fixtures must be Human Fighter cases only
- both fixtures must stay inside the existing deterministic Longsword / Chain Shirt / no-shield posture unless a narrower documented reason inside the allowed write scope proves otherwise
- both fixtures must preserve the accepted Human race-choice seam rather than replacing it with a different race or a looser interaction path
- the level-2 fixture may add only the minimum additional Fighter bonus-feat selection needed for this bounded tranche
- the level-3 fixture may add only the minimum additional level-3 progression posture needed to expose the first armor-training seam

If truthful completion would require new level-4+ fixtures, non-Human fixtures, or a rewritten accepted level-1 fixture, stop and block.

## TDD requirement
TDD is mandatory.

Execution order:
1. create `tests/sd13_fighter_level2_level3_progression.rs` first
2. create the two new deterministic fixtures only as needed to make the RED expectations concrete
3. run the targeted new test command below and capture a real RED state for the intended levels-2-and-3 delta
4. implement the smallest code changes inside `pilot_compute.rs` and/or `support_state_matrix.rs`
5. rerun the targeted test to green
6. rerun the focused regression floor
7. rerun full `cargo test`

RED discipline:
- a vague compile failure is not enough by itself
- the failing tests must explicitly name the intended level-2 and level-3 Fighter truth delta
- if RED reveals a need to touch any file outside the exact allowed write scope, stop and block instead of widening

Minimum RED assertions in `tests/sd13_fighter_level2_level3_progression.rs`:
1. level-2 deterministic Human Fighter input produces explicit non-blocked bounded evidence
2. level-3 deterministic Human Fighter input produces explicit non-blocked bounded evidence
3. Rogue remains blocked when it replaces the Fighter chassis
4. the matrix still keeps `class.fighter.level_1_pilot` and `class.fighter.levels_2_10` as separate rows
5. after this slice, the Fighter levels-2-10 row is still not `Supported`
6. any new armor-training explanation is explicit rather than implicit folklore

## Exact non-goals
This handoff does not authorize:
- any claim that Codex now supports Fighter through level 10 as a finished surface
- any positive support claim for Barbarian, Monk, Rogue, Paladin, Ranger, Bard, Cleric, Druid, Sorcerer, or Wizard
- any level-4-or-higher Fighter progression claim, including ability-score progression posture, repeated bonus-feat cadence, weapon training, later armor-training ranks, or level-10 closure
- any spellcasting burden work
- any multiclassing, prestige-class, archetype, alternate-racial-trait, or non-core expansion work
- any general feat engine, prerequisite engine, skill engine, equipment engine, or parser/input-model widening
- any edits under `/home/ubuntu/workspace/programs/codex/**`
- any edits under `/home/ubuntu/workspace/repos/codex/apps/desktop/**`
- any edits to `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_view_model.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
- any edits to the accepted level-1 fixtures
- any new fixture file other than the two exact level-2 and level-3 files named above
- any rewrite of `README.md`, `AGENTS.md`, `CLAUDE.md`, or governance docs as a substitute for repo evidence
- any normalization, deletion, or inclusion of unrelated `apps/desktop/src-tauri/gen/**` content

## Forbidden widening / stop conditions
Stop and block the CODE lane if any of these become true:
1. truthful completion requires editing `character_input.rs`, `pilot_view_model.rs`, `ge06_pilot_input_contract.rs`, or either accepted level-1 fixture
2. truthful completion requires any new fixture beyond the exact two level-2 and level-3 Human Fighter files named in this handoff
3. truthful completion requires a general feat-effect engine, prerequisite engine, or input-shape widening to express level-2 or level-3 proof
4. truthful completion requires touching any path outside the exact allowed write scope
5. truthful completion would promote Fighter level 4+, any non-Fighter class-positive-support row, or any broad “martial classes” claim
6. truthful completion depends on UI/workbench/reporting work, desktop/Tauri cleanup, distribution/update work, or governance/doc rewrites
7. the repo cannot be refreshed to a clean `origin/develop`-based execution worktree
8. any mandatory verification command below fails after the bounded change

If a stop condition lands, do not improvise. Block the CODE lane with the exact broader surface now required.

## Verification commands
Run these at minimum.

### Preflight truth confirmation
```bash
git -C /home/ubuntu/workspace/repos/codex fetch origin --prune
git -C /home/ubuntu/workspace/repos/codex rev-parse origin/develop
gh pr view 41 --repo electricm0nk/codex --json number,state,mergedAt,mergeCommit,headRefName,baseRefName,url
git -C "$WT" branch --show-current
git -C "$WT" rev-parse HEAD
git -C "$WT" status --short
```

### Write-scope confirmation
```bash
git -C "$WT" diff --name-only -- src/rules_core/pilot_compute.rs src/rules_core/support_state_matrix.rs tests/sd13_support_state_matrix.rs tests/ge06_pilot_total_saves.rs tests/ge06_pilot_combat_baseline.rs tests/ge06_pilot_selected_skill_modifiers.rs tests/ge06_pilot_headless_receipt.rs tests/ge06_failure_classifier.rs tests/ge06_pilot_view_model.rs tests/sd13_fighter_level2_level3_progression.rs tests/fixtures/rules_core/pf1_human_fighter_level2_sd13_deterministic_input.txt tests/fixtures/rules_core/pf1_human_fighter_level3_sd13_deterministic_input.txt
git -C "$WT" diff --unified=0 -- src/rules_core/pilot_compute.rs src/rules_core/support_state_matrix.rs tests/sd13_support_state_matrix.rs tests/ge06_pilot_total_saves.rs tests/ge06_pilot_combat_baseline.rs tests/ge06_pilot_selected_skill_modifiers.rs tests/ge06_pilot_headless_receipt.rs tests/ge06_failure_classifier.rs tests/ge06_pilot_view_model.rs tests/sd13_fighter_level2_level3_progression.rs tests/fixtures/rules_core/pf1_human_fighter_level2_sd13_deterministic_input.txt tests/fixtures/rules_core/pf1_human_fighter_level3_sd13_deterministic_input.txt
```

### Focused regression floor
```bash
cd "$WT" && . "$HOME/.cargo/env" && cargo test --test sd13_fighter_level2_level3_progression
cd "$WT" && . "$HOME/.cargo/env" && cargo test --test ge06_pilot_input_contract --test ge06_pilot_total_saves --test ge06_pilot_combat_baseline --test ge06_pilot_selected_skill_modifiers --test ge06_pilot_headless_receipt --test ge06_failure_classifier --test ge06_pilot_view_model --test sd13_support_state_matrix
cd "$WT" && . "$HOME/.cargo/env" && cargo test
```

Verification interpretation:
- `sd13_fighter_level2_level3_progression` is the mandatory tranche-specific RED/GREEN proof surface
- `ge06_pilot_input_contract` remains a read-only regression sentinel for the accepted Human race seam this slice must preserve
- `ge06_pilot_total_saves`, `ge06_pilot_combat_baseline`, and `ge06_pilot_selected_skill_modifiers` prove the widened bounded Fighter outputs and any honest armor-training effects
- `ge06_pilot_headless_receipt`, `ge06_failure_classifier`, and `ge06_pilot_view_model` prove downstream truth propagation and blocker ownership stay honest
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
- broadening into later SD13-E3 slices, SD13-E4 spell burden work, SD13-E5 cross-cutting closure, or any adjacent lane
- any write outside the exact allowed scope

Stop at verified `pr-created` state and return control through the governed review surface.
