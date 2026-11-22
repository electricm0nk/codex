---
title: GE06-E2-F2a Execution Handoff — Base Ability Modifiers and Fighter Class Chassis Computation
handoff_id: HANDOFF-CODEX-GE-06-E2-F2A-CODING-2026-06-21
stc_id: STC-CODEX-GE-06
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: completed
owner: Todd Hintzmann
scope: program
canonical: false
canonical_path: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/execution-handoff.md
source_stc: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md
readiness_closure: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2a-execution-readiness-closure-2026-06-21.md
merge_receipt: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2a-merge-receipt-2026-06-21.md
selected_slice: GE06-E2-F2a — Base ability modifiers and Fighter class chassis computation
run_in: Claude Code or equivalent frontier coding harness
code_authority: true
created_at: 2026-06-21
completed_at: 2026-06-21
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  expected_base_sha_at_creation: 9f3cb93
  recommended_branch: ge06-e2-f2a-base-chassis-computation
  pr_target: develop
  merged_commit: 760c9b0
  merged_pr: 8
allowed_write_scope:
  - src/rules_core/pilot_compute.rs
  - src/rules_core/mod.rs
  - tests/ge06_pilot_base_computation.rs
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen
  - src/oracle_validation/**
  - src/pcgen_import/**
  - tests/ge06_pilot_input_contract.rs
  - tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
  - UI or desktop shell paths
  - programs/codex/**
  - Cargo.toml
  - Cargo.lock
---

# GE06-E2-F2a Execution Handoff — Base Ability Modifiers and Fighter Class Chassis Computation

## Status
This historical handoff is complete and merged. It remains the stable stage-specific brief for GE06-E2-F2a and must not be retargeted to another slice.

## Run in
Claude Code or an equivalent frontier coding harness.

This was a code-authorizing handoff when executed. It is preserved here as the exact bounded brief that authorized GE06-E2-F2a.

## Core problem
GE06-E2-F1a made the deterministic pilot input contract executable as chosen input. Codex still could not compute even the first base rules-core outputs from that loaded input.

## Objective
Create the smallest headless rules-core computation surface that computes and explains only:

1. ability modifiers for the GE-06 deterministic pilot input
2. Fighter level-1 base attack bonus
3. Fighter level-1 base Fortitude, Reflex, and Will save bonuses

This handoff did **not** authorize armor class, attack bonus, skill modifiers, feat prerequisites, equipment effects, oracle parity, import conversion, UI, or export-sheet work.

## Required reads
Read these before editing code:

1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2a-execution-readiness-closure-2026-06-21.md`
3. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f1a-merge-receipt-2026-06-21.md`
4. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`
5. `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
6. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
7. `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
8. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst:139-141` — source evidence only; do not edit.
9. `programs/codex/requirements/GE-04-rules-engine-and-explainability-core/technical-requirements.md` TR-04-010 and TR-04-011.

## Branch setup
From `/home/ubuntu/workspace/repos/codex`:

```bash
git fetch origin --prune
git switch develop
git reset --hard origin/develop
git switch -c ge06-e2-f2a-base-chassis-computation
```

Expected base at handoff creation:

```text
origin/develop = 9f3cb93
```

If `origin/develop` had advanced, the coding harness was required to use the current fetched `origin/develop` and record the actual SHA in the final report.

## Baseline repo residue
Before this handoff, the repo reported these untracked files:

```text
?? AGENTS.md
?? CLAUDE.md
?? Cargo.lock
```

Rules:

- Read `AGENTS.md` / `CLAUDE.md` if useful as instruction surfaces.
- Do not add, delete, or modify `AGENTS.md`, `CLAUDE.md`, or `Cargo.lock` for this handoff.
- Do not treat pre-existing residue as part of your implementation diff.
- Your final scope audit had to list only files you changed or created for this handoff.

## Allowed write scope
You could write only:

```text
src/rules_core/pilot_compute.rs
src/rules_core/mod.rs
tests/ge06_pilot_base_computation.rs
```

If you needed any other file, the correct action was to stop and report the blocker.

Do not modify the GE06-E2-F1a input fixture or its tests. They are the prior foothold and must remain stable.

## Required implementation behavior
Create a minimal rules-core computation module that consumes a loaded `CharacterInput` and returns a computation result with structured explanations.

Suggested API shape, adjustable only if the final code remained smaller and clearer:

```rust
pub fn compute_pilot_base_chassis(input: &CharacterInput) -> PilotBaseChassisComputation
```

The result had to expose at least:

```rust
ability_modifiers: AbilityModifiers
base_attack_bonus: i16
base_saves: BaseSaves
explanations: Vec<ComputationExplanation>
diagnostics: Vec<ComputationDiagnostic>
```

Keep the model small. This was not the final rules engine architecture.

## Expected computed values
Using the GE-06 deterministic input fixture:

### Ability modifiers

Use this local deterministic formula:

```text
floor(score / 2) - 5
```

Expected values:

```yaml
strength: 3
dexterity: 2
constitution: 2
intelligence: 0
wisdom: 1
charisma: -1
```

### Fighter class chassis

Grounding source: `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst:139`.

Relevant source formulas:

```text
BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")
BONUS:SAVE|BASE.Fortitude|classlevel("APPLIEDAS=NONEPIC")/2+2
BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel("APPLIEDAS=NONEPIC")/3
```

For Fighter level 1, expected base class chassis values:

```yaml
base_attack_bonus: 1
base_fortitude_save: 2
base_reflex_save: 0
base_will_save: 0
```

These are class/base values only. Do not add ability modifiers to saves in this slice.

## Explanation requirements
The computation result had to include explanation records sufficient for tests to assert why each value exists.

At minimum, explanations had to include:

- one explanation for each ability modifier, referencing the source ability score and formula
- one explanation for base attack bonus, referencing Fighter level 1 and the class source formula
- one explanation for each base save, referencing Fighter level 1 and the class source formula

The exact Rust struct shape was up to the coding harness, but the test had to assert explanation presence by stable ids or equivalent machine-checkable fields, not by vague prose only.

Suggested ids:

```text
ability_modifier.strength
ability_modifier.dexterity
ability_modifier.constitution
ability_modifier.intelligence
ability_modifier.wisdom
ability_modifier.charisma
class_chassis.base_attack_bonus
class_chassis.base_save.fortitude
class_chassis.base_save.reflex
class_chassis.base_save.will
```

## Diagnostic requirements
The result had to preserve explicit diagnostics for unsupported inputs that would make this narrow computation unsafe.

Minimum diagnostic behavior:

- if the input has no Fighter level 1 class level, computation must not silently produce Fighter chassis values
- the result must contain a claim-blocking diagnostic identifying the missing or unsupported class chassis input

Do not add broad class support. Supporting only `class:fighter` level 1 was acceptable for this slice.

## Strict TDD sequence
The handoff required this sequence.

### RED
1. Create `tests/ge06_pilot_base_computation.rs` first.
2. The test must load `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt` through the existing loader and then call the wished-for computation API.
3. The test must assert expected ability modifiers, Fighter base BAB/saves, explanation records, and diagnostic behavior for unsupported/missing class chassis.
4. Run:

```bash
cargo test --test ge06_pilot_base_computation --quiet
```

5. Confirm it fails for the expected reason: the computation API/module does not exist yet.

If the test passes immediately, it is too weak or not testing new behavior.

### GREEN
Implement the smallest changes needed to pass:

- add `src/rules_core/pilot_compute.rs`
- expose it from `src/rules_core/mod.rs`
- implement only the base computation and explanation/diagnostic records required by the test

Do not modify the prior GE06-E2-F1a fixture or tests.

### VERIFY
Run:

```bash
cargo test --test ge06_pilot_base_computation --quiet
cargo test --test ge06_pilot_input_contract --quiet
cargo test --quiet
```

All had to pass.

### SCOPE AUDIT
Run:

```bash
git diff --name-only
git ls-files --others --exclude-standard
```

Confirm every new or modified file is inside the allowed write scope, ignoring pre-existing untracked `AGENTS.md`, `CLAUDE.md`, and `Cargo.lock` only if they were already present and untouched.

## Acceptance criteria
The handoff was complete only if all were true:

- [x] A GE-06-specific base computation test exists at `tests/ge06_pilot_base_computation.rs`.
- [x] A minimal computation module exists at `src/rules_core/pilot_compute.rs`.
- [x] The test first failed before production code changes.
- [x] The computation consumes the GE06-E2-F1a deterministic input fixture via the existing loader.
- [x] Ability modifiers are computed as STR +3, DEX +2, CON +2, INT +0, WIS +1, CHA -1.
- [x] Fighter base BAB is computed as +1.
- [x] Fighter base saves are computed as Fort +2, Reflex +0, Will +0.
- [x] Base save values are class/base values only and do not include ability modifiers.
- [x] Explanation records exist for each computed value using machine-checkable ids or equivalent fields.
- [x] Unsupported or missing class chassis input produces a claim-blocking diagnostic rather than fake values.
- [x] No armor, attack, skill modifier, feat prerequisite, import, parity, report, PCGen runner, or UI work is added.
- [x] All verification commands pass.
- [x] Scope audit shows only authorized files changed or newly created.

## Non-goals
Do not implement:

- armor class
- melee attack bonus
- weapon damage
- skill modifiers
- armor-check penalties
- encumbrance
- feat prerequisite evaluation
- choice availability
- source import expansion
- PCGen execution
- oracle comparison
- normalization engine
- parity report writer
- UI/view-model/export sheet work
- broad class support beyond Fighter level 1
- broad Pathfinder support

## Final report required from coding harness
Return a concise report with:

- branch name
- base SHA actually used
- files changed
- tests added or updated
- RED command and failure summary
- GREEN command and pass summary
- full verification command output summary
- scope audit result
- any blockers or deviations

## Final rule
This handoff existed to prove the first narrow computation layer. It did not authorize “the rules engine.” Compute the base chassis. Stop there.