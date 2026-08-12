---
title: SD13-E4-R2 Execution Handoff — Sorcerer level-1 spontaneous spell-burden baseline blockers
handoff_id: HANDOFF-CODEX-SD13-E4-R2-SORCERER-LEVEL1-SPELL-BASELINE-2026-07-01
stc_id: STC-CODEX-SD-13
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: ready-for-claude-launch
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e4-r2-spellcasting-and-hybrid-level-10-progression-execution-handoff-2026-07-01.md
source_stc: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md
source_epic_breakdown: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md
source_readiness_closure: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e4-r1-spellcasting-and-hybrid-level-10-progression-readiness-closure-2026-07-01.md
selected_slice: SD13-E4 first code slice — Sorcerer level-1 spontaneous spell-burden baseline blockers with hybrid-baseline preservation and explicit matrix-carrier alignment
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
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e3-r5-paladin-and-ranger-hybrid-chassis-baseline-execution-handoff-2026-07-01.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e4-r1-spellcasting-and-hybrid-level-10-progression-readiness-closure-2026-07-01.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  expected_base_sha_at_creation: 8e48056c1fc5fc2f1af772a4a90c9e73ce2144c5
  compare_base_ref: origin/develop
  compare_base_sha_at_creation: 8e48056c1fc5fc2f1af772a4a90c9e73ce2144c5
  upstream_review_surface: https://github.com/electricm0nk/codex/pull/44
  recommended_branch: feat/sd13-e4-f7-sorcerer-level1-spell-baseline
  pr_target: develop
allowed_write_scope:
  - src/rules_core/pilot_compute.rs
  - src/rules_core/support_state_matrix.rs
  - src/oracle_validation/support_state_matrix.rs
  - tests/sd13_support_state_matrix.rs
  - tests/ge06_pilot_headless_receipt.rs
  - tests/ge06_failure_classifier.rs
  - tests/ge06_pilot_view_model.rs
  - tests/sd13_sorcerer_level1_spell_baseline.rs
  - tests/fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt
forbidden_write_scope:
  - programs/codex/**
  - apps/desktop/**
  - apps/desktop/src-tauri/gen/**
  - Cargo.toml
  - Cargo.lock
  - src/lib.rs
  - src/rules_core/character_input.rs
  - src/rules_core/pilot_view_model.rs
  - tests/ge06_pilot_base_computation.rs
  - tests/ge06_pilot_input_contract.rs
  - tests/ge06_pilot_total_saves.rs
  - tests/ge06_pilot_combat_baseline.rs
  - tests/ge06_pilot_selected_skill_modifiers.rs
  - tests/sd13_hybrid_level1_chassis_baseline.rs
  - tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
  - tests/fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt
  - tests/fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt
  - tests/ge08_*
  - AGENTS.md
  - CLAUDE.md
completion_class: pr-created
reviewed_at: 2026-07-01
---

# SD13-E4-R2 Execution Handoff — Sorcerer level-1 spontaneous spell-burden baseline blockers

## Status
This is the stage-specific code-authorizing brief for the first honest repo-facing SD13-E4 lane.

It grants code authority for one bounded slice only: add deterministic Human Sorcerer level-1 runtime recognition as a blocked/computed spell-bearing baseline, name Sorcerer's class-specific burden explicitly, preserve the accepted Human/Fighter/hybrid truth already on `origin/develop`, and keep both support-state matrix carriers aligned instead of letting one advance while the other lies.

Board-visible verdict:
- this handoff is ready for a governed Claude Code lane now
- this artifact is documentary authorization only until the later CODE lane executes it
- later implementation truth exists only if the CODE lane leaves a durable `claude-execution-receipt`

## Run in
Claude Code only.

Do not substitute Hermes file editing or another coding harness as the primary implementation path. If Claude Code cannot be launched truthfully, block the downstream CODE lane instead of coding through Hermes.

## Core problem
Accepted repo truth on 2026-07-01 is now sharp enough to separate hybrid baseline truth from true spell-bearing work, but it still contains no direct full-caster runtime evidence:
- sanctioned base truth is `origin/develop` at `8e48056c1fc5fc2f1af772a4a90c9e73ce2144c5`, the merge commit for PR `#44`
- the shared checkout at `/home/ubuntu/workspace/repos/codex` is not that sanctioned base right now; after `git fetch origin --prune` it still reports branch `feat/sd13-e6-f11-support-state-debt-presentation` at `122de6a60609d9452de53c6d3ad406aeb81c2a82`, with upstream tracking gone and untracked `apps/desktop/src-tauri/gen/` content
- the accepted evidence worktree `/home/ubuntu/workspace/worktrees/codex-sd13-e4-verdict` is clean detached `HEAD` at `8e48056c1fc5fc2f1af772a4a90c9e73ce2144c5`
- `src/rules_core/pilot_compute.rs` already recognizes Paladin and Ranger only at the deterministic Human level-1 hybrid chassis boundary and keeps both explicitly blocked on separate non-spell and later-spell burdens
- `src/rules_core/support_state_matrix.rs` now reflects that hybrid truth as `Blocked` / `Computed`, but Bard, Sorcerer, and Wizard remain `Unverified` / `Observed`
- `src/oracle_validation/support_state_matrix.rs` still lags the accepted hybrid truth and therefore must be handled deliberately rather than ignored
- no deterministic Bard, Sorcerer, or Wizard fixture exists yet in the accepted rules-core fixture set

The decisive move is not “open all arcane casters.” It is smaller and honest: choose one spell-bearing class whose first runtime movement can stay a baseline blocker surface instead of counterfeiting spell support.

## Why Sorcerer is the first truthful SD13-F7 slice
The later readiness closure deliberately refused to counterfeit this last narrowing decision. That decision is now frozen here.

Sorcerer is first because it is the narrowest honest arcane spell-bearing burden among the `SD13-F7` family:
1. Bard carries bardic-performance and support-feature burden in addition to spell posture. Starting there would entangle spell work with a second non-spell subsystem immediately.
2. Wizard carries prepared-casting, spellbook, and school-or-bonded-item branch pressure. Starting there would force the first `F7` move through a heavier branch surface.
3. Sorcerer still has real class-specific burden, but it is the cleanest first spell-bearing burden: bloodline plus spontaneous known-spell and slot posture.
4. A deterministic Human Sorcerer level-1 blocked baseline mirrors the accepted Paladin/Ranger level-1 hybrid pattern: direct runtime recognition, explicit burden naming, zero fabricated spell math, and a matrix uplift only to `Blocked` / `Computed`.
5. Choosing Human preserves the already-accepted Human race and Human interaction truth rather than mixing the first spell-bearing move with a new race seam.

This handoff therefore freezes the first code-authorizing `SD13-F7` tranche as:
- class: `class:sorcerer`
- level boundary: level 1 only
- support-state target: `Blocked` / `Computed`, not `Partial` and not `Supported`
- burden posture: explicit bloodline blocker plus explicit spontaneous spell posture blocker

## Objective
Implement the smallest truthful SD13-E4 progression slice.

The result must prove all of the following:
1. `src/rules_core/pilot_compute.rs` can recognize a deterministic Human Sorcerer level-1 input on the live rules-core seam without fabricating spell slots, spells known, spell choices, prepared posture, or general caster math
2. Sorcerer runtime output stays claim-blocked for the right reasons, with separate diagnostics for bloodline burden and spontaneous spell posture burden
3. `src/rules_core/support_state_matrix.rs` can move `class.sorcerer.progression_and_spell_burden` from `Unverified` / `Observed` to `Blocked` / `Computed` only if the blocker text names exactly what remains out of proof after this slice
4. `src/oracle_validation/support_state_matrix.rs` is updated in the same slice so the repo does not keep contradictory Sorcerer row truth in two matrix carriers
5. Bard and Wizard remain exactly where accepted `origin/develop` leaves them: `Unverified` / `Observed`
6. accepted Paladin/Ranger hybrid blocked/computed truth remains intact and is not flattened into spell-support closure
7. accepted Human race, Human interaction, Fighter 1-3, Rogue blocked, receipt, classifier, and view-model truth remain intact
8. the slice stops before Wizard, Bard, Cleric, Druid, Paladin, Ranger, level-2+ Sorcerer, or general spell-engine burden

## Why this route is authorized now
This route is authorized because accepted repo truth already exposes both the pattern and the missing spell-bearing gap:
- `src/rules_core/pilot_compute.rs` is already the live seam that surfaced hybrid baseline recognition while withholding fabricated support
- `tests/sd13_hybrid_level1_chassis_baseline.rs` already proves the exact style of baseline this lane should mimic: direct acknowledgement plus explicit blockers
- `tests/ge06_pilot_headless_receipt.rs`, `tests/ge06_failure_classifier.rs`, and `tests/ge06_pilot_view_model.rs` already enforce downstream propagation of blocked/computed truth
- `epic-breakdown.md` explicitly isolates `SD13-F7` from `SD13-F8`
- `artifacts/level-10-progression-validation-contract.md` explicitly names Sorcerer's distinctive burden as bloodline plus spontaneous spell posture
- there is no accepted full-caster runtime surface yet, so the first honest move is a baseline blocker tranche, not a counterfeit support promotion

What is still not authorized:
- any positive support claim for Bard, Wizard, Cleric, Druid, Paladin, Ranger, or any non-Sorcerer class
- any claim that Sorcerer is now partially or fully supported through level 10
- any level-2+ Sorcerer uplift
- any Bardic Performance, Wizard school/spellbook, Cleric domain, Druid nature-bond, Paladin spell, or Ranger spell work
- any general spell engine, generic slot resolver, spellbook model, or parser/input-shape widening
- any governance/program-file edits or any desktop/Tauri work

## Target repo / workdir
```text
repo:    /home/ubuntu/workspace/repos/codex
workdir: /home/ubuntu/workspace/repos/codex
```

## Branch policy and launch substrate
Do not launch from the shared checkout as-is.

At handoff creation time, live shell facts were:
- `git rev-parse origin/develop` -> `8e48056c1fc5fc2f1af772a4a90c9e73ce2144c5`
- current local branch in the shared checkout -> `feat/sd13-e6-f11-support-state-debt-presentation`
- current local `HEAD` in the shared checkout -> `122de6a60609d9452de53c6d3ad406aeb81c2a82`
- `git status --short --branch` in the shared checkout -> upstream `[gone]` plus `?? apps/desktop/src-tauri/gen/`

Launch this slice from a fresh isolated worktree off accepted `origin/develop` instead:

```bash
git -C /home/ubuntu/workspace/repos/codex fetch origin --prune
WT=/home/ubuntu/workspace/worktrees/codex-sd13-e4-f7-sorcerer-level1
git -C /home/ubuntu/workspace/repos/codex worktree add -b feat/sd13-e4-f7-sorcerer-level1-spell-baseline "$WT" origin/develop
cd "$WT"
```

If `feat/sd13-e4-f7-sorcerer-level1-spell-baseline` already exists, reuse it only after confirming:
- it still belongs exclusively to this slice
- it still starts from sanctioned `origin/develop` truth
- it carries no unrelated changes outside the bounded write scope

Record the actual launch branch, base SHA, commit handles, and PR handle in the final `claude-execution-receipt`.

## Exact required reads before coding
Read these first, in order:
1. `/home/ubuntu/workspace/worktrees/codex-sd13-e4-f7-sorcerer-level1/CLAUDE.md`
2. `/home/ubuntu/workspace/worktrees/codex-sd13-e4-f7-sorcerer-level1/AGENTS.md`
3. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e4-r2-spellcasting-and-hybrid-level-10-progression-execution-handoff-2026-07-01.md`
4. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e4-r1-spellcasting-and-hybrid-level-10-progression-readiness-closure-2026-07-01.md`
5. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e3-r5-paladin-and-ranger-hybrid-chassis-baseline-execution-handoff-2026-07-01.md`
6. `/home/ubuntu/workspace/worktrees/codex-sd13-e4-f7-sorcerer-level1/README.md`
7. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md`
8. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md`
9. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-design.md`
10. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/acceptance-and-verification.md`
11. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md`
12. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
13. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/level-10-progression-validation-contract.md`
14. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
15. `/home/ubuntu/workspace/worktrees/codex-sd13-e4-f7-sorcerer-level1/src/rules_core/pilot_compute.rs`
16. `/home/ubuntu/workspace/worktrees/codex-sd13-e4-f7-sorcerer-level1/src/rules_core/support_state_matrix.rs`
17. `/home/ubuntu/workspace/worktrees/codex-sd13-e4-f7-sorcerer-level1/src/oracle_validation/support_state_matrix.rs`
18. `/home/ubuntu/workspace/worktrees/codex-sd13-e4-f7-sorcerer-level1/tests/sd13_support_state_matrix.rs`
19. `/home/ubuntu/workspace/worktrees/codex-sd13-e4-f7-sorcerer-level1/tests/sd13_hybrid_level1_chassis_baseline.rs`
20. `/home/ubuntu/workspace/worktrees/codex-sd13-e4-f7-sorcerer-level1/tests/ge06_pilot_base_computation.rs`
21. `/home/ubuntu/workspace/worktrees/codex-sd13-e4-f7-sorcerer-level1/tests/ge06_pilot_input_contract.rs`
22. `/home/ubuntu/workspace/worktrees/codex-sd13-e4-f7-sorcerer-level1/tests/ge06_pilot_total_saves.rs`
23. `/home/ubuntu/workspace/worktrees/codex-sd13-e4-f7-sorcerer-level1/tests/ge06_pilot_combat_baseline.rs`
24. `/home/ubuntu/workspace/worktrees/codex-sd13-e4-f7-sorcerer-level1/tests/ge06_pilot_selected_skill_modifiers.rs`
25. `/home/ubuntu/workspace/worktrees/codex-sd13-e4-f7-sorcerer-level1/tests/ge06_pilot_headless_receipt.rs`
26. `/home/ubuntu/workspace/worktrees/codex-sd13-e4-f7-sorcerer-level1/tests/ge06_failure_classifier.rs`
27. `/home/ubuntu/workspace/worktrees/codex-sd13-e4-f7-sorcerer-level1/tests/ge06_pilot_view_model.rs`
28. `/home/ubuntu/workspace/worktrees/codex-sd13-e4-f7-sorcerer-level1/tests/fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt`
29. `/home/ubuntu/workspace/worktrees/codex-sd13-e4-f7-sorcerer-level1/tests/fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt`

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
/home/ubuntu/workspace/repos/codex/tests/sd13_sorcerer_level1_spell_baseline.rs
/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt
```

Write-scope interpretation:
- `pilot_compute.rs` is the only live rules-core seam authorized to surface Sorcerer baseline recognition and explicit blockers
- `support_state_matrix.rs` is the authoritative live matrix carrier authorized to reclassify the Sorcerer row after bounded proof exists
- `src/oracle_validation/support_state_matrix.rs` is intentionally in scope so the repo does not leave Sorcerer row truth split between `rules_core` and a stale oracle mirror
- `sd13_support_state_matrix.rs` must pin the exact final matrix posture after this slice
- `ge06_pilot_headless_receipt.rs`, `ge06_failure_classifier.rs`, and `ge06_pilot_view_model.rs` are the only downstream propagation surfaces authorized for adjustment if the new Sorcerer blocked/computed posture changes what those consumers must report
- `sd13_sorcerer_level1_spell_baseline.rs` is the dedicated tranche-specific proof surface for this handoff
- the Sorcerer fixture named above is the only new fixture file authorized in this slice

No other repo file is in write scope.

## Read-only grounding seams
These files are grounding truth for this lane and may not be edited under this handoff:
- `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
- `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_view_model.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_base_computation.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_selected_skill_modifiers.rs`
- `/home/ubuntu/workspace/repos/codex/tests/sd13_hybrid_level1_chassis_baseline.rs`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt`
- program-level packet docs listed above

If truthful completion would require write authority over any of these, stop and block instead of widening silently.

## Contract to implement
Implement one bounded Sorcerer baseline tranche over the already-accepted Human deterministic seam.

### Exact level and class under claim
This slice may claim only this new spell-bearing baseline:
- `race:human`
- `class:sorcerer:1`

No Bard, Wizard, Cleric, Druid, Paladin, Ranger, Rogue, Monk, Barbarian, or level-2+ Sorcerer surface may be promoted by this handoff.

### Exact burden under claim
The slice may prove only these bounded things:
1. direct runtime recognition that deterministic Human Sorcerer level 1 is a known spell-bearing class identity on the rules-core seam
2. explicit claim-blocking visibility for Sorcerer's bloodline burden
3. explicit claim-blocking visibility for Sorcerer's spontaneous known-spell / slot posture burden
4. matrix uplift for Sorcerer only from `Unverified` / `Observed` to `Blocked` / `Computed`
5. truthful propagation of that blocked/computed posture through receipt, classifier, and view-model surfaces when they consume the underlying diagnostics
6. matching Sorcerer row truth in the oracle-validation matrix carrier

### Required result shape
The final implementation must satisfy all of the following:
1. deterministic Human Sorcerer level-1 input leaves one explicit bounded recognition explanation carrying no fabricated mechanical value
2. deterministic Human Sorcerer level-1 input leaves two distinct claim-blocking diagnostics: one for bloodline burden and one for spontaneous spell posture burden
3. no spell slots, spells known, spell DCs, bonus spells, prepared posture, school choice, or general spell totals are fabricated
4. `class.sorcerer.progression_and_spell_burden` may move only to `Blocked` / `Computed`, and its blocker text must still name the missing burden rather than imply partial closure
5. `class.bard.progression_and_spell_burden` and `class.wizard.progression_and_spell_burden` must remain exactly `Unverified` / `Observed`
6. `class.paladin.hybrid_chassis_and_spell_burden` and `class.ranger.hybrid_chassis_and_spell_burden` must remain `Blocked` / `Computed`
7. the accepted Human row and Human interaction row must not be weakened, relabeled downward, or hidden
8. Fighter 1-3, Rogue blocked negative-control truth, and accepted hybrid baseline truth must remain intact

### Deterministic fixture posture
The new fixture must remain tightly bounded:
- it must be a Human Sorcerer case only
- it must add only the minimum choice data needed to make the Sorcerer class identity, bloodline burden, and spontaneous spell posture burden explicit
- it must not invent a broader spell list engine, level-up engine, or multi-level caster progression surface
- it must preserve the accepted Human seam rather than mixing the first spell-bearing move with a new race burden

If truthful completion would require level-2+ Sorcerer fixtures, non-Human Sorcerer fixtures, Bard fixtures, Wizard fixtures, or edits to the accepted Fighter/Paladin/Ranger fixtures, stop and block.

## TDD requirement
TDD is mandatory.

Execution order:
1. create `tests/sd13_sorcerer_level1_spell_baseline.rs` first
2. create `tests/fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt` only as needed to make the RED expectations concrete
3. run the targeted new test command below and capture a real RED state for the intended Sorcerer delta
4. implement the smallest code changes inside `pilot_compute.rs`, `support_state_matrix.rs`, and the oracle mirror only as needed
5. rerun the targeted test to green
6. rerun the focused regression floor
7. rerun full `cargo test`

RED discipline:
- a vague compile failure is not enough by itself
- the failing tests must explicitly name Sorcerer baseline recognition and the two distinct blocker families
- if RED reveals a need to touch any file outside the exact allowed write scope, stop and block instead of widening

Minimum RED assertions in `tests/sd13_sorcerer_level1_spell_baseline.rs`:
1. deterministic Human Sorcerer level-1 input produces one explicit bounded recognition explanation carrying `+0` fabricated value
2. deterministic Human Sorcerer level-1 input produces a claim-blocking bloodline diagnostic
3. deterministic Human Sorcerer level-1 input produces a separate claim-blocking spontaneous known-spell / slot posture diagnostic
4. Sorcerer matrix row is no longer pure `Unverified` / `Observed` after this slice, but is still not `Partial` or `Supported`
5. Bard and Wizard rows remain `Unverified` / `Observed`
6. Paladin and Ranger hybrid rows remain `Blocked` / `Computed`
7. no test assertion depends on fabricated slot math or general spell resolution

## Exact non-goals
This handoff does not authorize:
- any claim that Codex now supports Sorcerer through level 10 as a finished surface
- any promotion of Sorcerer above `Blocked` / `Computed`
- any Bard, Wizard, Cleric, Druid, Paladin, or Ranger positive-support work
- any level-2+ Sorcerer progression claim
- any Bardic Performance implementation
- any Wizard school, bonded-item, spellbook, or prepared-casting implementation
- any Cleric domain, Druid nature-bond, Paladin non-spell class-feature, or Ranger non-spell class-feature implementation
- any use of accepted Paladin/Ranger hybrid level-1 evidence as proof that their spell burden is closed
- any combined `SD13-F7` + `SD13-F8` tranche
- any general spell engine, generic slot resolver, spellbook model, or parser/input-model widening
- any edits under `/home/ubuntu/workspace/programs/codex/**`
- any edits under `/home/ubuntu/workspace/repos/codex/apps/desktop/**`
- any edits to `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_view_model.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_base_computation.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_selected_skill_modifiers.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/tests/sd13_hybrid_level1_chassis_baseline.rs`
- any new fixture file other than the one exact Sorcerer file named above
- any rewrite of `README.md`, `AGENTS.md`, `CLAUDE.md`, or governance docs as a substitute for repo evidence
- any normalization, deletion, or inclusion of unrelated `apps/desktop/src-tauri/gen/**` content

## Forbidden widening / stop conditions
Stop and block the CODE lane if any of these become true:
1. truthful completion requires editing `character_input.rs`, `pilot_view_model.rs`, `ge06_pilot_base_computation.rs`, `ge06_pilot_input_contract.rs`, `ge06_pilot_total_saves.rs`, `ge06_pilot_combat_baseline.rs`, `ge06_pilot_selected_skill_modifiers.rs`, or `sd13_hybrid_level1_chassis_baseline.rs`
2. truthful completion requires any new fixture beyond the exact Sorcerer file named in this handoff
3. truthful completion requires level-2+ Sorcerer work, Bard work, Wizard work, or any `SD13-F8` divine/hybrid burden work
4. truthful completion requires a general spell engine, prepared-casting model, spellbook model, or input-shape widening
5. truthful completion requires touching any path outside the exact allowed write scope
6. truthful completion would promote Sorcerer beyond `Blocked` / `Computed` or would promote any non-Sorcerer spell-bearing row
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
gh pr view 44 --repo electricm0nk/codex --json number,state,mergedAt,mergeCommit,headRefName,baseRefName,url
git -C "$WT" branch --show-current
git -C "$WT" rev-parse HEAD
git -C "$WT" status --short
```

### Write-scope confirmation
```bash
git -C "$WT" diff --name-only -- src/rules_core/pilot_compute.rs src/rules_core/support_state_matrix.rs src/oracle_validation/support_state_matrix.rs tests/sd13_support_state_matrix.rs tests/ge06_pilot_headless_receipt.rs tests/ge06_failure_classifier.rs tests/ge06_pilot_view_model.rs tests/sd13_sorcerer_level1_spell_baseline.rs tests/fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt
git -C "$WT" diff --unified=0 -- src/rules_core/pilot_compute.rs src/rules_core/support_state_matrix.rs src/oracle_validation/support_state_matrix.rs tests/sd13_support_state_matrix.rs tests/ge06_pilot_headless_receipt.rs tests/ge06_failure_classifier.rs tests/ge06_pilot_view_model.rs tests/sd13_sorcerer_level1_spell_baseline.rs tests/fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt
```

### Focused regression floor
```bash
cd "$WT" && . "$HOME/.cargo/env" && cargo test --test sd13_sorcerer_level1_spell_baseline
cd "$WT" && . "$HOME/.cargo/env" && cargo test --test ge06_pilot_base_computation --test ge06_pilot_input_contract --test ge06_pilot_total_saves --test ge06_pilot_combat_baseline --test ge06_pilot_selected_skill_modifiers --test ge06_pilot_headless_receipt --test ge06_failure_classifier --test ge06_pilot_view_model --test sd13_support_state_matrix --test sd13_hybrid_level1_chassis_baseline
cd "$WT" && . "$HOME/.cargo/env" && cargo test
```

Verification interpretation:
- `sd13_sorcerer_level1_spell_baseline` is the mandatory tranche-specific RED/GREEN proof surface
- `ge06_pilot_base_computation` preserves the accepted Wizard-backed non-Fighter negative-control posture as Sorcerer gets its own explicit baseline lane
- `ge06_pilot_input_contract`, `ge06_pilot_total_saves`, `ge06_pilot_combat_baseline`, and `ge06_pilot_selected_skill_modifiers` are read-only regression sentinels for the accepted Human/Fighter deterministic seam this slice must not disturb
- `ge06_pilot_headless_receipt`, `ge06_failure_classifier`, and `ge06_pilot_view_model` prove downstream truth propagation and blocker ownership stay honest
- `sd13_support_state_matrix` is the control-plane truth gate for the final matrix posture
- `sd13_hybrid_level1_chassis_baseline` proves the accepted Paladin/Ranger blocked/computed split remains intact and unflattened
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
- broadening into later SD13-E4 slices, SD13-E5 cross-cutting closure, or any adjacent lane
- any write outside the exact allowed scope

Stop at verified `pr-created` state and return control through the governed review surface.
