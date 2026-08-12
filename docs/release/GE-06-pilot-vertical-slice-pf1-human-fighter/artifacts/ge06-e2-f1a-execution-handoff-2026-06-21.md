---
title: GE06-E2-F1a Execution Handoff — Deterministic Pilot Input Contract Fixture Load Gate
handoff_id: HANDOFF-CODEX-GE-06-E2-F1A-CODING-2026-06-21
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
readiness_closure: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f1-execution-readiness-closure-2026-06-21.md
merge_receipt: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f1a-merge-receipt-2026-06-21.md
selected_slice: GE06-E2-F1a — Deterministic pilot input contract fixture load gate
run_in: Claude Code or equivalent frontier coding harness
code_authority: true
created_at: 2026-06-21
completed_at: 2026-06-21
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  recommended_branch: ge06-e2-f1a-deterministic-pilot-input-contract
  pr_target: develop
allowed_write_scope:
  - tests/ge06_pilot_input_contract.rs
  - tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
  - src/rules_core/character_input.rs
  - src/rules_core/mod.rs
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen
  - src/oracle_validation/**
  - src/pcgen_import/**
  - UI or desktop shell paths
  - programs/codex/**
  - Cargo.toml
  - Cargo.lock
---

# GE06-E2-F1a Execution Handoff — Deterministic Pilot Input Contract Fixture Load Gate

## Run in
Claude Code or an equivalent frontier coding harness.

This is a code-authorizing handoff. Hermes generated it; Hermes should not execute the code work unless Todd explicitly overrides the harness route.

## Core problem
Codex has a GE-04 character-input loader and a GE-05 golden fixture schema, but it does not yet have a headless tested fixture path that represents the **accepted GE-06 deterministic pilot input contract**.

## Objective
Create the smallest headless rules-core fixture/test path that proves Codex can represent and validate the GE-06 deterministic PF1 Core Rulebook Human Fighter level 1 pilot input contract.

The output is not a computed character. The output is a validated chosen-input record carrying the final GE-06 selections.

## Required reads
Read these before editing code:

1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f1-execution-readiness-closure-2026-06-21.md`
3. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`
4. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md`
5. `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
6. `/home/ubuntu/workspace/repos/codex/tests/character_input_record.rs`
7. `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_minimal_character_input.txt`
8. `/home/ubuntu/workspace/repos/codex/src/oracle_validation/golden_fixture.rs` — read only for GE-05 boundary awareness; do not edit.

## Branch setup
From `/home/ubuntu/workspace/repos/codex`:

```bash
git fetch origin --prune
git switch develop
git reset --hard origin/develop
git switch -c ge06-e2-f1a-deterministic-pilot-input-contract
```

Expected base at handoff creation:

```text
origin/develop = a2c7e88
```

If `origin/develop` has advanced, use the current fetched `origin/develop` and record the actual SHA in the final report.

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
- Your final scope audit must list only files you changed or created for this handoff.

## Allowed write scope
You may write only:

```text
tests/ge06_pilot_input_contract.rs
tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
src/rules_core/character_input.rs
```

You may also write this only if strictly needed for module exposure:

```text
src/rules_core/mod.rs
```

If you need any other file, stop and report the blocker.

## Required implementation behavior

The new GE-06 fixture/test path must prove the loader can represent these input facts from the final deterministic contract.

### Identity

```yaml
case_id: pf1-crb-human-fighter-level1
source_package_id: pf1.core_rulebook
race_id: race:human
class_level: class:fighter:1
```

If the existing loader does not support `case_id`, add the smallest representation needed, but do not turn this into a generic metadata framework.

### Ability scores

```yaml
strength: 16
dexterity: 14
constitution: 14
intelligence: 10
wisdom: 12
charisma: 8
human_ability_bonus:
  target: strength
  bonus: 2
```

The GE-04 historical fixture has CON 13. Do not mutate that file to hide history. Create a GE-06 fixture with CON 14.

### Feats and choice slots

Represent these selected feats:

```text
feat:power_attack
feat:dodge
feat:weapon_focus
```

Represent enough selected choices to make the slot decisions visible:

```text
choice:level_1_character_feat:feat:power_attack
choice:human_bonus_feat:feat:dodge
choice:fighter_bonus_feat:feat:weapon_focus:weapon:longsword
choice:human_ability_bonus:ability:strength
```

The exact string shape may follow existing parser conventions, but the test must assert that the slot/selection intent is not lost.

### Skill ranks

Represent exactly:

```text
skill:climb = 1
skill:intimidate = 1
skill:swim = 1
```

Do not compute final skill modifiers.

### Equipment and active states

Represent at least:

```text
item:chain_shirt = equipped/worn/active equivalent
item:longsword = equipped/primary/active equivalent
shield = none / absent equivalent
power_attack = selected but inactive for baseline outputs
```

If the existing `EquipmentSelection` boolean is insufficient to distinguish equipped/active/absent/inactive states, extend it minimally. Do not create a full inventory engine.

### Provenance

The loaded input must retain provenance/source references including the GE-06 deterministic input contract path or a stable identifier for it.

At minimum, the test should assert one provenance entry points to:

```text
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md
```

## Strict TDD sequence

You must follow this sequence.

### RED

1. Create `tests/ge06_pilot_input_contract.rs` and `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt` first.
2. The test must assert the GE-06 contract facts above.
3. Run:

```bash
cargo test --test ge06_pilot_input_contract --quiet
```

4. Confirm it fails for the expected reason: the current loader/model cannot yet represent the new GE-06 contract fully.

If the test passes immediately, the test is too weak. Strengthen it until it fails for a real missing capability.

### GREEN

Implement the smallest changes in `src/rules_core/character_input.rs` required to make the test pass.

Acceptable minimal changes may include:

- adding optional `case_id` or similar identity field
- allowing additional selected choices without losing slot identity
- representing equipment/active-state distinctions more explicitly than a single boolean
- preserving multiple provenance entries

Unacceptable changes:

- computing derived Pathfinder values
- adding a rules evaluator
- importing PCGen files
- adding a comparator
- adding UI state
- changing GE-05 fixture schema or historical GE-04 fixture to fake alignment

### VERIFY

Run:

```bash
cargo test --test ge06_pilot_input_contract --quiet
cargo test --test character_input_record --quiet
cargo test --quiet
```

All must pass.

### SCOPE AUDIT

Run:

```bash
git diff --name-only
git ls-files --others --exclude-standard
```

Confirm every new or modified file is inside the allowed write scope, ignoring pre-existing untracked `AGENTS.md`, `CLAUDE.md`, and `Cargo.lock` only if they were already present and untouched.

## Acceptance criteria

The handoff is complete only if all are true:

- [ ] A GE-06-specific test exists at `tests/ge06_pilot_input_contract.rs`.
- [ ] A GE-06-specific deterministic fixture exists at `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`.
- [ ] The test first failed before production code changes.
- [ ] The final fixture represents Human Fighter level 1 with STR 16, DEX 14, CON 14, INT 10, WIS 12, CHA 8.
- [ ] The final fixture represents Power Attack, Dodge, and Weapon Focus (Longsword).
- [ ] The final fixture represents Climb 1, Intimidate 1, and Swim 1.
- [ ] The final fixture represents Chain Shirt, Longsword, no shield, and Power Attack inactive for baseline outputs.
- [ ] The loaded record retains provenance pointing to the GE-06 deterministic input contract.
- [ ] No derived BAB/save/AC/attack/skill-modifier values are computed.
- [ ] No oracle parity, normalization, report writer, PCGen runner, or UI work is added.
- [ ] All verification commands pass.
- [ ] Scope audit shows only authorized files changed or newly created.

## Non-goals

Do not implement:

- BAB calculation
- saving throw calculation
- armor class calculation
- attack bonus calculation
- skill modifier calculation
- armor-check penalty calculation
- feat prerequisite evaluation
- source import expansion
- PCGen execution
- oracle comparison
- normalization engine
- parity report writer
- UI/view-model/export sheet work
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

This handoff exists to make the GE-06 pilot input contract executable as chosen input. It does not authorize the system to prove Pathfinder math yet. That comes next, after this foothold is real.
