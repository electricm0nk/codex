---
title: SD13-E3-R5 Execution Handoff — Paladin and Ranger level-1 hybrid chassis baseline with explicit class-feature and spell blockers
handoff_id: HANDOFF-CODEX-SD13-E3-R5-PALADIN-RANGER-HYBRID-LEVEL1-BASELINE-2026-07-01
stc_id: STC-CODEX-SD-13
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: ready-for-claude-launch
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e3-r5-paladin-and-ranger-hybrid-chassis-baseline-execution-handoff-2026-07-01.md
source_stc: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md
source_epic_breakdown: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md
source_readiness_closure: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e3-r1-martial-and-skill-driven-level-10-progression-readiness-closure-2026-07-01.md
source_prior_execution_handoff: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e3-r2-martial-and-skill-driven-level-10-progression-execution-handoff-2026-07-01.md
selected_slice: SD13-F6 first code slice — combined Paladin and Ranger level-1 hybrid chassis baseline with explicit non-spell class-feature blockers and no spell-burden closure
authority_dependencies:
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-design.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/acceptance-and-verification.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/level-10-progression-validation-contract.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e3-r1-martial-and-skill-driven-level-10-progression-readiness-closure-2026-07-01.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e3-r2-martial-and-skill-driven-level-10-progression-execution-handoff-2026-07-01.md
run_in: Claude Code only
code_authority: true
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  expected_base_sha_at_creation: 33e419ea9ab0c7d6c7b63906e88130f1dcc155ce
  compare_base_ref: origin/develop
  compare_base_sha_at_creation: 33e419ea9ab0c7d6c7b63906e88130f1dcc155ce
  upstream_review_surface: https://github.com/electricm0nk/codex/pull/43
  recommended_branch: feat/sd13-e3-f6-hybrid-level1-baseline
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
  - tests/sd13_hybrid_level1_chassis_baseline.rs
  - tests/fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt
  - tests/fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt
forbidden_write_scope:
  - programs/codex/**
  - apps/desktop/**
  - apps/desktop/src-tauri/gen/**
  - Cargo.toml
  - Cargo.lock
  - src/lib.rs
  - src/oracle_validation/**
  - src/rules_core/character_input.rs
  - src/rules_core/pilot_view_model.rs
  - tests/ge06_pilot_input_contract.rs
  - tests/sd13_fighter_level2_level3_progression.rs
  - tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
  - tests/fixtures/rules_core/pf1_human_fighter_level1_minimal_character_input.txt
  - tests/fixtures/rules_core/pf1_human_fighter_level2_sd13_deterministic_input.txt
  - tests/fixtures/rules_core/pf1_human_fighter_level3_sd13_deterministic_input.txt
  - tests/ge08_*
  - AGENTS.md
  - CLAUDE.md
completion_class: pr-created
reviewed_at: 2026-07-01
---

# SD13-E3-R5 Execution Handoff — Paladin and Ranger level-1 hybrid chassis baseline with explicit class-feature and spell blockers

## Status
This is the stage-specific code-authorizing brief for the first truthful SD13-F6 repo-facing lane.

It grants code authority for one bounded slice only: establish direct runtime evidence that the live rules-core surface can recognize and explain a deterministic Human Paladin level-1 chassis and a deterministic Human Ranger level-1 chassis, then reclassify their SD13 rows from pure packet placeholders to explicitly blocked computed baselines that still name the missing non-spell class-feature burden and the later spell burden.

Board-visible verdict:
- the downstream CODE lane is knowable now
- it may stay combined only at the shared level-1 hybrid chassis boundary
- if the worker finds itself trying to implement Paladin level 2+, Ranger level 2+, class-specific feature math, or spell posture in the same slice, the lane is overbroad and must block rather than improvise
- the truthful matrix target for this first slice is `Blocked` / `Computed`, not `Partial` or `Supported`

## Run in
Claude Code only.

Do not substitute Hermes file editing or another coding harness as the primary implementation path. If Claude Code cannot be launched truthfully, block the downstream CODE lane instead of coding through Hermes.

## Core problem
Accepted repo truth on 2026-07-01 has moved since the earlier SD13-E3 documentary work, but the hybrid class rows are still documentary placeholders:
- sanctioned base truth is now `origin/develop` at `33e419ea9ab0c7d6c7b63906e88130f1dcc155ce`, the merge commit for PR `#43` (`SD13-E7` evidence-refresh audit)
- the shared checkout at `/home/ubuntu/workspace/repos/codex` is still not that sanctioned base right now; after `git fetch origin --prune` it reports branch `feat/sd13-e6-f11-support-state-debt-presentation` at `122de6a60609d9452de53c6d3ad406aeb81c2a82` with untracked `apps/desktop/src-tauri/gen/` content
- accepted `origin/develop` now truthfully supports the bounded Human Fighter levels 1-3 seam, but `src/rules_core/support_state_matrix.rs` still leaves `class.paladin.hybrid_chassis_and_spell_burden` and `class.ranger.hybrid_chassis_and_spell_burden` at `Unverified` / `Observed` with empty blocker notes
- the packet is explicit that Paladin and Ranger are not ordinary martial rows: `epic-breakdown.md` says SD13-F6 begins with hybrid chassis/class-feature truth before later spell burden closure, and `level-10-progression-validation-contract.md` says a generic martial shell does not justify a Paladin or Ranger support claim
- accepted `pilot_compute.rs` is still structurally Fighter-shaped: `supported_fighter_level(...)` gates the live class chassis, the GE-06 regression floor remains centered on Fighter-derived combat/save/selected-skill outputs, and no accepted hybrid row yet carries direct runtime evidence

The decisive move is not “support Paladin and Ranger through their early feature ladders.” That would counterfeit closure. The decisive move is smaller and honest: split the first shared hybrid chassis boundary away from the Fighter-only seam, surface direct computed evidence for level-1 Paladin and Ranger inputs, and keep both rows explicitly blocked on the missing class-feature and later spell burdens.

## Why this exact tranche is the first truthful SD13-F6 move
1. `epic-breakdown.md` names SD13-F6 as a hybrid chassis baseline lane, not a full hybrid spell-completion lane. The first move should therefore expose chassis truth without pretending SD13-E4 is done.
2. `core-roster-and-support-state-matrix.md` already says Paladin and Ranger should classify chassis burden first, then spell burden explicitly. A direct move from `Unverified` / `Observed` to `Supported` would violate that doctrine.
3. `level-10-progression-validation-contract.md` is explicit that Paladin cannot be considered supported from a partial martial shell and that Ranger cannot be marked supported from a generic martial shell alone. The first truthful promotion is therefore to a still-blocked computed posture, not to support.
4. the only safe combined lane is the shared level-1 baseline. Once the work reaches Paladin-specific burden (smite, lay on hands, divine grace, mercy or similar) or Ranger-specific burden (favored enemy, combat style, skill/tracking burden), the slice stops being a shared substrate lane and becomes class-family-specific work that needs its own later tranche.
5. stopping at level 1 keeps this first slice below the point where a worker would need to counterfeit feature-family completion, healing/resource handling, target/alignment logic, or any spell posture.

## Objective
Implement the smallest truthful SD13-F6 slice.

The result must prove all of the following:
1. the live rules-core surface can ingest deterministic Human `class:paladin:1` and `class:ranger:1` inputs without treating them as mere undocumented packet scope
2. the code leaves direct computed evidence on the accepted runtime path for those two hybrid level-1 cases
3. both hybrid matrix rows move off pure `Unverified` / `Observed` placeholders only if the final posture remains explicitly blocked on the missing class-feature burden and later spell burden
4. the accepted Fighter 1-3 truth, Rogue blocked negative-control truth, and Human race/interaction truth remain intact
5. no row becomes `Supported`, no row becomes `Lossy`, and no broader “hybrid classes now work” claim appears anywhere
6. the slice stops before Paladin/Ranger level 2+, feature-family execution, spellcasting posture, or a general class engine

## Why this route is authorized now
This route is authorized because the accepted repo and packet now jointly expose one honest hybrid move:
- `epic-breakdown.md` names SD13-F6 directly and constrains it to hybrid chassis/class-feature truth before spell closure
- `technical-requirements.md` and the validation contract keep spell-bearing/hybrid truth separate from non-caster truth and require later spell posture classification for Paladin and Ranger
- `core-roster-and-support-state-matrix.md` already says both rows should classify chassis burden first and spell burden later
- `unsupported-partial-lossy-and-unverified-semantics-ledger.md` still records both rows as `unverified` with no direct chassis or spell evidence, which means a runtime-evidence slice is now the missing step
- accepted `pilot_compute.rs`, `support_state_matrix.rs`, and the GE-06/SD13 tests already provide the live seam where direct evidence and blocker honesty can be made explicit
- earlier documentary work already proved the repo must use clean `origin/develop` truth instead of a stale shared branch; this handoff simply freezes the exact hybrid tranche on that accepted base

What is still not authorized:
- any `Supported` claim for Paladin or Ranger
- any positive claim for Paladin level 2+, Ranger level 2+, or any later hybrid milestone
- any spell-slot, spell-source, spells-known/prepared, or partial-caster burden work
- any target/alignment/creature-type combat logic for smite or favored enemy
- any healing/resource surface for lay on hands or mercy
- any general feat/prerequisite engine, broad skill engine, class-engine rewrite, multiclassing, archetype, or non-core scope expansion
- any non-Human hybrid uplift or any broad fighter/rogue/barbarian/monk reconsideration beyond regression preservation

## Target repo / workdir
```text
repo:    /home/ubuntu/workspace/repos/codex
workdir: /home/ubuntu/workspace/repos/codex
```

## Branch policy and launch substrate
Do not launch from the shared checkout as-is.

At handoff creation time, live shell facts were:
- `git rev-parse origin/develop` -> `33e419ea9ab0c7d6c7b63906e88130f1dcc155ce`
- `git log --oneline -1 origin/develop` -> `33e419e Merge pull request #43 from electricm0nk/feat/sd13-e7-f13-evidence-refresh-audit`
- current local branch in the shared checkout -> `feat/sd13-e6-f11-support-state-debt-presentation`
- current local `HEAD` in the shared checkout -> `122de6a60609d9452de53c6d3ad406aeb81c2a82`
- `git status --short` in the shared checkout -> `?? apps/desktop/src-tauri/gen/`

Launch this slice from a fresh isolated worktree off accepted `origin/develop` instead:

```bash
git -C /home/ubuntu/workspace/repos/codex fetch origin --prune
WT=/home/ubuntu/workspace/worktrees/codex-sd13-e3-f6-hybrid-level1-baseline
git -C /home/ubuntu/workspace/repos/codex worktree add -b feat/sd13-e3-f6-hybrid-level1-baseline "$WT" origin/develop
cd "$WT"
```

If `feat/sd13-e3-f6-hybrid-level1-baseline` already exists, reuse it only after confirming:
- it still belongs exclusively to this slice
- it still starts from sanctioned `origin/develop` truth
- it does not carry unrelated changes outside the bounded write scope

Record the actual launch branch, base SHA, commit handles, and PR handle in the final `claude-execution-receipt`.

## Exact required reads before coding
Read these first, in order:
1. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
2. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
3. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e3-r5-paladin-and-ranger-hybrid-chassis-baseline-execution-handoff-2026-07-01.md`
4. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e3-r2-martial-and-skill-driven-level-10-progression-execution-handoff-2026-07-01.md`
5. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e3-r1-martial-and-skill-driven-level-10-progression-readiness-closure-2026-07-01.md`
6. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md`
7. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md`
8. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md`
9. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-design.md`
10. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/acceptance-and-verification.md`
11. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
12. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/level-10-progression-validation-contract.md`
13. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
14. `/home/ubuntu/workspace/repos/codex/README.md`
15. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
16. `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
17. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
18. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
19. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
20. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_selected_skill_modifiers.rs`
21. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_headless_receipt.rs`
22. `/home/ubuntu/workspace/repos/codex/tests/ge06_failure_classifier.rs`
23. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_view_model.rs`
24. `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`
25. `/home/ubuntu/workspace/repos/codex/tests/sd13_fighter_level2_level3_progression.rs`
26. `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
27. `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level2_sd13_deterministic_input.txt`
28. `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level3_sd13_deterministic_input.txt`

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
/home/ubuntu/workspace/repos/codex/tests/sd13_hybrid_level1_chassis_baseline.rs
/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt
/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt
```

Write-scope interpretation:
- `pilot_compute.rs` is the only live compute seam authorized to surface direct hybrid level-1 runtime evidence and explicit blocker diagnostics
- `support_state_matrix.rs` is the only control-plane truth surface authorized to reclassify the Paladin and Ranger hybrid rows after direct new proof exists
- the listed GE-06 tests are regression sentinels and may be updated only where the bounded hybrid baseline changes what they should explicitly compute, propagate, or claim-block
- `sd13_support_state_matrix.rs` must pin the exact final matrix posture after this slice
- `sd13_hybrid_level1_chassis_baseline.rs` is the dedicated new TDD proof surface for the tranche this handoff authorizes
- the two new deterministic fixtures are the only new fixture files authorized in this slice

No other repo file is in write scope.

## Read-only grounding seams
These files are grounding truth for this lane and may not be edited under this handoff:
- `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
- `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_view_model.rs`
- `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
- `/home/ubuntu/workspace/repos/codex/tests/sd13_fighter_level2_level3_progression.rs`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level2_sd13_deterministic_input.txt`
- `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level3_sd13_deterministic_input.txt`
- program-level packet docs listed above

If truthful completion would require write authority over any of these, stop and block instead of widening silently.

## Contract to implement
Implement one bounded hybrid baseline tranche over the already-accepted Human deterministic substrate.

### Exact milestone levels under claim
This slice may claim only these new hybrid milestone levels:
- `class:paladin:1`
- `class:ranger:1`

No other Paladin or Ranger level may be promoted by this handoff.

### Exact matrix posture under claim
The matrix posture this slice may earn is narrowly constrained:
- `class.paladin.hybrid_chassis_and_spell_burden` may move from `Unverified` / `Observed` to `Blocked` / `Computed`
- `class.ranger.hybrid_chassis_and_spell_burden` may move from `Unverified` / `Observed` to `Blocked` / `Computed`
- neither row may move to `Partial`
- neither row may move to `Supported`

The rationale is doctrinal, not cosmetic: the validation contract explicitly says a partial martial shell is not enough to prove Paladin support and a generic martial shell is not enough to prove Ranger support.

### Exact burden under claim
The slice may prove only these new burdens:
1. a direct runtime chassis baseline exists for deterministic Human Paladin level 1 and Human Ranger level 1 inputs on the accepted rules-core seam
2. those hybrid level-1 cases are no longer mere packet placeholders; the runtime and matrix must now name what is still blocking them
3. Paladin remains explicitly blocked on its non-spell feature family and later spell burden, with the blocker note and/or compute diagnostics naming at least the smite / lay-on-hands / divine-grace / mercy-or-similar burden family rather than hiding behind a generic “unsupported hybrid” label
4. Ranger remains explicitly blocked on its non-spell feature family and later spell burden, with the blocker note and/or compute diagnostics naming at least the favored-enemy / combat-style / skill-tracking burden family rather than hiding behind a generic “unsupported hybrid” label
5. the accepted Fighter 1-3, Rogue blocked, and Human race/interaction truths are preserved without relabeling or accidental downgrade

### Required result shape
The final implementation must satisfy all of the following:
1. the new dedicated tranche test proves that `class:paladin:1` and `class:ranger:1` now leave direct runtime evidence on the bounded compute path
2. that direct runtime evidence is still claim-blocked for honest reasons tied to missing hybrid burden, not because the repo simply refuses to parse or acknowledge the class identity at all
3. both hybrid matrix rows carry non-empty blocker notes that explicitly name their still-missing non-spell class-feature burden and the later spell burden
4. neither hybrid row becomes `Partial` or `Supported`
5. the Fighter level-1 row, Fighter levels-2-10 row, Rogue row, Human race row, and Human interaction row remain at their accepted support/evidence posture unless a preservation-aligned wording update is strictly required by the new tranche
6. no non-Human row, no other class row, and no interaction row is silently promoted

### Deterministic fixture posture
The two new fixtures must remain tightly bounded:
- both fixtures must be Human, single-class, and level 1 only
- both fixtures must retain the named Human race-choice seam rather than routing around it
- both fixtures must avoid spell selections, prepared/known posture, domains, animal companions, favored-terrain breadth, alignment-target resolution, healing resource accounting, or other late burden surfaces
- the fixtures may add only the minimum feat, equipment, and selected-skill posture needed to make the bounded hybrid baseline legible
- if truthful completion would require a new chosen-input shape, parser/schema authority, or mutation of the accepted Fighter fixtures, stop and block

## TDD requirement
TDD is mandatory.

Execution order:
1. create `tests/sd13_hybrid_level1_chassis_baseline.rs` first
2. create the two new deterministic fixtures only as needed to make the RED expectations concrete
3. run the targeted new test command below and capture a real RED state for the intended Paladin/Ranger level-1 delta
4. implement the smallest code changes inside `pilot_compute.rs` and/or `support_state_matrix.rs`
5. rerun the targeted new test to green
6. rerun the focused regression floor
7. rerun full `cargo test`

RED discipline:
- a vague compile failure is not enough by itself
- the failing tests must explicitly name the intended hybrid level-1 evidence and blocker delta
- if the RED state reveals that truthful completion requires out-of-scope authority, stop there and block rather than pushing through GREEN by widening the brief

## Exact verification commands
### Preflight grounding commands
These are not success gates by themselves, but the worker must run them so the lane does not operate from stale branch truth.

```bash
cd /home/ubuntu/workspace/repos/codex && git fetch origin --prune && git rev-parse --abbrev-ref HEAD && git rev-parse HEAD && git rev-parse origin/develop
cd /home/ubuntu/workspace/repos/codex && git diff --name-only origin/develop -- src/rules_core/pilot_compute.rs src/rules_core/support_state_matrix.rs tests/ge06_pilot_input_contract.rs tests/ge06_pilot_total_saves.rs tests/ge06_pilot_combat_baseline.rs tests/ge06_pilot_selected_skill_modifiers.rs tests/ge06_pilot_headless_receipt.rs tests/ge06_failure_classifier.rs tests/ge06_pilot_view_model.rs tests/sd13_support_state_matrix.rs tests/sd13_fighter_level2_level3_progression.rs tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt tests/fixtures/rules_core/pf1_human_fighter_level2_sd13_deterministic_input.txt tests/fixtures/rules_core/pf1_human_fighter_level3_sd13_deterministic_input.txt
```

Interpretation:
- if the target working copy still lags `origin/develop` on the listed files, the worker must sync to accepted `develop` truth or use a clean worktree before claiming SD13-F6 evidence

### Required RED command
```bash
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test --test sd13_hybrid_level1_chassis_baseline
```

### Required regression / acceptance commands
```bash
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test --test ge06_pilot_input_contract --test ge06_pilot_total_saves --test ge06_pilot_combat_baseline --test ge06_pilot_selected_skill_modifiers --test ge06_pilot_headless_receipt --test ge06_failure_classifier --test ge06_pilot_view_model --test sd13_support_state_matrix --test sd13_fighter_level2_level3_progression --test sd13_hybrid_level1_chassis_baseline
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test
```

Verification interpretation:
- the dedicated new tranche test is mandatory because this slice is not merely a matrix edit; it must prove real runtime evidence
- the focused regression bundle is mandatory because it protects the accepted Human race seam, Fighter 1-3 truth, Rogue blocker truth, downstream receipt/classifier/view-model propagation, and the matrix posture the hybrid slice is allowed to touch
- full `cargo test` is a smoke/regression sweep only; it does not upgrade any SD13 support-state claim by itself

## Explicit stop-and-reroute conditions
Stop and block the CODE lane if any of the following becomes necessary:
1. edits outside the allowed write scope
2. changes to `src/rules_core/character_input.rs` or any input-schema/parser authority surface
3. edits under `src/oracle_validation/**`, `src/lib.rs`, `apps/desktop/**`, `tests/ge08_*`, or program/governance docs
4. any attempt to rescue the slice by using Paladin level 2+, Ranger level 2+, non-Human fixtures, multiclass fixtures, or spell-bearing fixtures
5. any need for target/alignment/creature typing, healing accounting, resource tracking, favored-enemy category resolution, combat-style engine behavior, mercy handling, spell-slot logic, known/prepared spell posture, domains, companions, or other broader hybrid/spell surfaces
6. any need to reclassify the hybrid rows above `Blocked` / `Computed` without direct class-feature proof
7. any change that weakens the accepted Human race/interaction rows, Fighter 1-3 rows, or Rogue blocked row rather than preserving them

If any stop condition triggers, the worker must block with a concise explanation naming the exact missing authority surface.

## Exact non-goals
This handoff does not authorize any of the following:
- no `Supported` claim for Paladin or Ranger
- no `Partial` claim for Paladin or Ranger
- no Paladin level 2+ or Ranger level 2+ progression claim
- no smite math, lay-on-hands resource handling, divine-grace computation, mercy behavior, or similar Paladin feature execution beyond explicit blocker visibility
- no favored-enemy resolution, combat-style execution, tracking engine, or similar Ranger feature execution beyond explicit blocker visibility
- no spellcasting burden implementation, spell slots, spell-source lineage, prepared/known posture, or partial-caster closure
- no general class-engine rewrite, no general feat/prerequisite engine, no general skill engine, and no broad “martial classes now work” claim
- no multiclassing, archetypes, prestige classes, or non-core expansion
- no non-Human uplift beyond preserving the already-accepted Human interaction seam
- no UI/workbench/reporting/distribution/persistence work under SD-11, SD-12, or SD-14 authority

## Expected final delivery from the CODE lane
The downstream Claude Code lane should finish only when it can provide all of the following:
1. a PR against `develop`
2. a `claude-execution-receipt` naming:
   - launch worktree path
   - base SHA
   - branch name
   - changed files
   - exact commands run
   - actual RED -> GREEN evidence for `sd13_hybrid_level1_chassis_baseline`
   - focused regression and full-suite results
3. an explicit statement that Paladin and Ranger remain blocked on named class-feature and spell burdens after this slice

## Readiness verdict
This lane is ready for a governed CODE successor now.

Why it is ready:
- accepted `origin/develop` truth is now sharp enough to isolate the first honest hybrid move
- the packet and matrix already define the required classification posture: chassis first, spell burden later
- the repo already contains the exact surfaces where direct runtime evidence and blocker honesty can be made explicit
- the combined tranche is still small enough to remain truthful only at the shared level-1 boundary

Why the tranche is not broader:
- Paladin- and Ranger-specific burden diverge immediately after that shared boundary
- trying to implement level-2+ or feature-family behavior in the same slice would force new authority surfaces and counterfeit closure
- the validation contract forbids collapsing hybrid support into a generic martial-shell claim

## Successor truth
The earned successor should be:
- `SD13-E3-F6 CODE: Paladin and Ranger level-1 hybrid chassis baseline blockers`

That successor should execute this handoff, leave the hybrid rows at an explicitly blocked computed posture, and preserve the later route into SD13-E4 for actual spell burden closure.
