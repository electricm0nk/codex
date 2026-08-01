---
title: GE06-E2-F2b Execution Handoff — Baseline Melee Attack Bonus and Armor Class Under Deterministic Loadout
handoff_id: HANDOFF-CODEX-GE-06-E2-F2B-CODING-2026-06-21
stc_id: STC-CODEX-GE-06
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: active
owner: Todd Hintzmann
scope: program
canonical: false
canonical_path: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/execution-handoff.md
source_stc: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md
readiness_closure: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2b-execution-readiness-closure-2026-06-21.md
selected_slice: GE06-E2-F2b — Baseline melee attack bonus and armor class under deterministic loadout
run_in: Claude Code or equivalent frontier coding harness
code_authority: true
created_at: 2026-06-21
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  expected_base_sha_at_creation: 760c9b0
  recommended_branch: ge06-e2-f2b-baseline-combat-values
  pr_target: develop
allowed_write_scope:
  - src/rules_core/pilot_compute.rs
  - tests/ge06_pilot_combat_baseline.rs
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen
  - src/rules_core/character_input.rs
  - src/oracle_validation/**
  - src/pcgen_import/**
  - tests/ge06_pilot_base_computation.rs
  - tests/ge06_pilot_input_contract.rs
  - tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
  - UI or desktop shell paths
  - programs/codex/**
  - Cargo.toml
  - Cargo.lock
---

# GE06-E2-F2b Execution Handoff — Baseline Melee Attack Bonus and Armor Class Under Deterministic Loadout

## Status
This is the active stage-specific code-authorizing brief for GE06-E2-F2b. It must not overwrite any prior F1a or F2a handoff artifact.

## Run in
Claude Code or an equivalent frontier coding harness.

This handoff carries `code_authority: true` for GE06-E2-F2b only.

## Core problem
GE06-E2-F2a established ability modifiers and Fighter level-1 base chassis. Codex still cannot compute the first deterministic combat-facing totals from the accepted loadout: baseline melee attack bonus and baseline armor class.

## Objective
Extend the existing GE-06 pilot compute surface to produce and explain only:

1. baseline melee attack bonus for the deterministic Longsword-primary loadout with Power Attack selected but inactive
2. baseline armor class for Chain Shirt worn, DEX 14, Dodge selected, and no shield

This handoff does **not** authorize weapon damage, active Power Attack math, initiative, skills, armor-check penalties, encumbrance, parity, UI, or broad proficiency systems.

## Required reads
Read these before editing code:

1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2b-execution-readiness-closure-2026-06-21.md`
3. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2a-merge-receipt-2026-06-21.md`
4. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2a-execution-handoff-2026-06-21.md`
5. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`
6. `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
7. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
8. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_base_computation.rs`
9. `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
10. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst:139-143` — source evidence only; do not edit.
11. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst:40` — Chain Shirt armor bonus and `MAXDEX:4`; source evidence only.
12. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst:165` — Longsword base row; source evidence only.
13. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_profs_weapon.lst:57` — Longsword proficiency identity; source evidence only.
14. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_feats.lst:53` — Dodge +1 AC; source evidence only.
15. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_feats.lst:184` — Weapon Focus selected-weapon +1 to-hit; source evidence only.
16. `programs/codex/requirements/GE-04-rules-engine-and-explainability-core/technical-requirements.md` TR-04-010 and TR-04-011.

## Branch setup
From `/home/ubuntu/workspace/repos/codex`:

```bash
git fetch origin --prune
git switch develop
git reset --hard origin/develop
git switch -c ge06-e2-f2b-baseline-combat-values
```

Expected base at handoff creation:

```text
origin/develop = 760c9b0
```

If `origin/develop` has advanced, use the current fetched `origin/develop` and record the actual base SHA in the final report.

## Toolchain posture
Observed in the current Hermes shell:

```text
cargo is not on the default PATH
$HOME/.cargo/bin/cargo is present and works
```

Use the explicit cargo path in commands below unless your shell already resolves `cargo` and you record that fact in the final report.

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
src/rules_core/pilot_compute.rs
tests/ge06_pilot_combat_baseline.rs
```

If you need any other file, stop and report the blocker.

Do not modify the deterministic input fixture, `character_input.rs`, or the prior F2a proof test. Those are already grounded and must remain stable.

## Required implementation behavior
Keep the implementation inside the existing `compute_pilot_base_chassis` / `PilotBaseChassisComputation` surface unless a compile blocker proves that a smaller truthful shape is impossible.

Preferred outcome:

- extend `PilotBaseChassisComputation` with two new bounded outputs
- keep one headless entry point for the GE-06 pilot base-compute path
- add no new module unless required for clarity and still within scope

Suggested new fields:

```rust
baseline_melee_attack_bonus: i16
baseline_armor_class: i16
```

Equivalent naming is allowed only if it is smaller and clearer while remaining machine-checkable in tests.

## Expected computed values
Using the accepted deterministic input fixture:

### Baseline melee attack bonus

Expected total:

```yaml
baseline_melee_attack_bonus: 5
```

Bounded contributors:

```text
Fighter base attack bonus: +1
Strength modifier: +3
Weapon Focus (Longsword): +1
Power Attack: selected but inactive -> contributes 0 and must be mentioned as inactive
```

So the total is:

```text
1 + 3 + 1 = 5
```

### Baseline armor class

Expected total:

```yaml
baseline_armor_class: 17
```

Bounded contributors:

```text
base 10
Chain Shirt armor bonus: +4
Dexterity contribution: +2 (DEX 14, within MAXDEX 4)
Dodge: +1
shield: absent -> +0
```

So the total is:

```text
10 + 4 + 2 + 1 = 17
```

## Support boundary
This slice may support only the exact deterministic posture needed for the GE-06 pilot baseline.

That means it is acceptable to compute these values only when all of the following are true:

- Fighter level 1 chassis is present
- `item:longsword` is `EquippedActive`
- `item:chain_shirt` is `EquippedActive`
- `item:shield` is `Absent`
- `power_attack` is `SelectedInactive`
- `feat:dodge` is selected
- `feat:weapon_focus` is selected
- the selected Fighter bonus feat choice is `feat:weapon_focus:weapon:longsword`

If those conditions are not met, the correct behavior for this slice is to emit a claim-blocking diagnostic and refuse to fabricate combat totals.

Do **not** generalize into a broad combat engine here.

## Explanation requirements
The computation result must include explanation records sufficient for tests to assert why each new bounded total exists.

At minimum, add machine-checkable explanation ids or equivalent fields for:

```text
combat.baseline_melee_attack_bonus
defense.baseline_armor_class
```

The explanation detail for attack must mention:

- Fighter BAB
- Strength modifier
- Weapon Focus (Longsword)
- Power Attack inactive posture

The explanation detail for armor class must mention:

- base 10
- Chain Shirt armor bonus
- Dexterity contribution
- `MAXDEX:4` limit context
- Dodge bonus
- shield absent posture

Tests must assert contributors by ids and/or detail content, not by vague prose only.

## Diagnostic requirements
The result must preserve explicit diagnostics for unsupported inputs that would make this narrow baseline dishonest.

Minimum diagnostic behavior:

- if the deterministic longsword/armor/feat/choice posture is missing or changed, do not silently compute the totals
- emit at least one claim-blocking diagnostic identifying the unsupported combat baseline posture
- when combat totals are unsupported, do not emit combat explanation records that pretend those totals were grounded

You may still preserve the already-supported F2a outputs and explanations where appropriate.

## Strict TDD sequence
This handoff requires the following sequence.

### RED
1. Create `tests/ge06_pilot_combat_baseline.rs` first.
2. Load `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt` through the existing loader and call the existing GE-06 pilot compute entry point.
3. Assert baseline melee attack bonus `5`, baseline armor class `17`, and explanation details proving the exact bounded contributors.
4. Add at least one unsupported-posture test that proves claim-blocking diagnostics appear and combat explanations are withheld when the exact deterministic posture is not met.
5. Run:

```bash
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_combat_baseline --quiet
```

6. Confirm it fails for the expected reason: the new combat fields/behavior do not exist yet.

If the test passes immediately, it is too weak or not testing new behavior.

### GREEN
Implement the smallest change needed to pass:

- extend `src/rules_core/pilot_compute.rs`
- do not modify `character_input.rs`
- do not modify the deterministic fixture or prior F2a proof test

### VERIFY
Run:

```bash
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_combat_baseline --quiet
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_base_computation --quiet
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_input_contract --quiet
"$HOME/.cargo/bin/cargo" test --quiet
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

- [ ] A new GE-06 combat-baseline test exists at `tests/ge06_pilot_combat_baseline.rs`.
- [ ] The new test fails before production code changes.
- [ ] `src/rules_core/pilot_compute.rs` is the only production file changed.
- [ ] The compute surface returns baseline melee attack bonus `5` for the deterministic fixture.
- [ ] The compute surface returns baseline armor class `17` for the deterministic fixture.
- [ ] Attack explanation records mention Fighter BAB, STR modifier, Weapon Focus (Longsword), and Power Attack inactive posture.
- [ ] Armor-class explanation records mention base 10, Chain Shirt armor bonus, DEX contribution, `MAXDEX:4`, Dodge, and shield absent posture.
- [ ] Unsupported loadout/choice posture produces claim-blocking diagnostics rather than fabricated combat totals.
- [ ] No damage, active Power Attack, initiative, skill, parity, report, importer, PCGen runner, or UI work is added.
- [ ] All verification commands pass.
- [ ] Scope audit shows only authorized files changed or newly created.

## Non-goals
Do not implement:

- damage rolls
- active Power Attack penalties or damage bonuses
- iterative attacks / multiattack logic
- attack-of-opportunity logic
- initiative
- skill modifiers
- armor-check penalties
- encumbrance
- shield breadth beyond the exact absent posture needed for the deterministic baseline
- broad armor or weapon proficiency systems
- feat prerequisite evaluation beyond the exact deterministic posture needed to reject unsupported input
- source import expansion
- PCGen execution
- oracle comparison
- normalization engine
- parity report writer
- UI/view-model/export sheet work
- broad Pathfinder support

## Final report required from coding harness
Return a concise factual report with:

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
This handoff exists to prove the first deterministic combat totals from the accepted pilot loadout. It does not authorize “combat support” in the abstract. Compute baseline Longsword attack bonus and baseline armor class. Stop there.