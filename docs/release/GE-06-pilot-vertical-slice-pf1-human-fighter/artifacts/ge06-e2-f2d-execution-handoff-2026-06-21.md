---
title: GE06-E2-F2d Execution Handoff — Selected Deterministic Skill Modifiers and Chain Shirt Armor-Check Effects
handoff_id: HANDOFF-CODEX-GE-06-E2-F2D-CODING-2026-06-21
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
readiness_closure: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2d-execution-readiness-closure-2026-06-21.md
selected_slice: GE06-E2-F2d — Selected deterministic skill modifiers and Chain Shirt armor-check effects
run_in: Claude Code or equivalent frontier coding harness
code_authority: true
created_at: 2026-06-21
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  expected_base_sha_at_creation: 1b44c07
  recommended_branch: ge06-e2-f2d-selected-skill-modifiers
  pr_target: develop
allowed_write_scope:
  - src/rules_core/pilot_compute.rs
  - tests/ge06_pilot_selected_skill_modifiers.rs
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen
  - src/rules_core/character_input.rs
  - src/oracle_validation/**
  - src/pcgen_import/**
  - tests/ge06_pilot_base_computation.rs
  - tests/ge06_pilot_combat_baseline.rs
  - tests/ge06_pilot_total_saves.rs
  - tests/ge06_pilot_input_contract.rs
  - tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
  - UI or desktop shell paths
  - programs/codex/**
  - Cargo.toml
  - Cargo.lock
---

# GE06-E2-F2d Execution Handoff — Selected Deterministic Skill Modifiers and Chain Shirt Armor-Check Effects

## Status
This is the active stage-specific code-authorizing brief for GE06-E2-F2d. It must not overwrite any prior F1a, F2a, F2b, or F2c handoff artifact.

## Run in
Claude Code or an equivalent frontier coding harness.

This handoff carries `code_authority: true` for GE06-E2-F2d only.

## Core problem
GE06-E2-F2c closed deterministic total saving throws. Codex still cannot compute the next truthful bounded skill/equipment-effect surface from the accepted pilot contract: the selected deterministic Climb, Intimidate, and Swim modifiers, with the already-grounded Chain Shirt armor-check penalty applied only where it belongs.

## Objective
Extend the existing GE-06 pilot compute surface to produce and explain only:

1. selected deterministic Climb modifier
2. selected deterministic Intimidate modifier
3. selected deterministic Swim modifier
4. claim-blocking diagnostics when the chosen Fighter class posture, chosen skill-rank posture, or deterministic Chain Shirt posture is absent or widened beyond this slice

This handoff does **not** authorize a broad Pathfinder skill system. It does **not** authorize unselected skills, feat-based skill modifiers, racial skill modifiers, item bonuses beyond the deterministic Chain Shirt armor-check penalty, encumbrance breadth, speed-dependent adjustments, parity, UI, or broader rules-engine expansion.

## Required reads
Read these before editing code:

1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2d-execution-readiness-closure-2026-06-21.md`
3. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2c-merge-receipt-2026-06-21.md`
4. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2c-execution-handoff-2026-06-21.md`
5. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`
6. `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
7. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
8. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_base_computation.rs`
9. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
10. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
11. `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
12. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst:2835` — Fighter class-skill evidence only; do not edit.
13. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_skills.lst:10`, `:42`, and `:102` — Climb / Intimidate / Swim grounding only; do not edit.
14. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst:40` — Chain Shirt `ACCHECK:-2` evidence only; do not edit.
15. `programs/codex/requirements/GE-04-rules-engine-and-explainability-core/technical-requirements.md` TR-04-010 and TR-04-011.

## Branch setup
From `/home/ubuntu/workspace/repos/codex`:

```bash
git fetch origin --prune
git switch develop
git reset --hard origin/develop
git switch -c ge06-e2-f2d-selected-skill-modifiers
```

Expected base at handoff creation:

```text
origin/develop = 1b44c07
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

## Merge authority boundary
This handoff does **not** authorize the coding harness to merge anything.

- Do not merge the branch or PR.
- Do not land code directly onto `develop` or `main`.
- Stop at a verified branch state and hand control back to Todd for the merge decision and merge action.

## Allowed write scope
You may write only:

```text
src/rules_core/pilot_compute.rs
tests/ge06_pilot_selected_skill_modifiers.rs
```

If you need any other file, stop and report the blocker.

Do not modify the deterministic input fixture, `character_input.rs`, or the prior F2a/F2b/F2c proof tests. Those are already grounded and must remain stable.

## Required implementation behavior
Keep the implementation inside the existing `compute_pilot_base_chassis` / `PilotBaseChassisComputation` surface unless a compile blocker proves that a smaller truthful shape is impossible.

Preferred outcome:

- extend `PilotBaseChassisComputation` with one bounded selected-skill-modifier surface
- keep one headless entry point for the GE-06 pilot base-compute path
- add no new module unless required for clarity and still within scope
- preserve already-merged F2a, F2b, and F2c behavior exactly

Suggested new field:

```rust
selected_skill_modifiers: SelectedSkillModifiers
```

Suggested bounded shape:

```rust
pub struct SelectedSkillModifiers {
    pub climb: i16,
    pub intimidate: i16,
    pub swim: i16,
}
```

Equivalent naming is allowed only if it is smaller and clearer while remaining machine-checkable in tests.

## Expected computed values
Using the accepted deterministic input fixture:

### Climb
Expected total:

```yaml
climb: 5
```

Bounded contributors:

```text
rank 1
Strength modifier +3
class-skill bonus +3
Chain Shirt armor-check penalty -2
```

So the total is:

```text
1 + 3 + 3 - 2 = 5
```

### Intimidate
Expected total:

```yaml
intimidate: 3
```

Bounded contributors:

```text
rank 1
Charisma modifier -1
class-skill bonus +3
```

So the total is:

```text
1 - 1 + 3 = 3
```

### Swim
Expected total:

```yaml
swim: 5
```

Bounded contributors:

```text
rank 1
Strength modifier +3
class-skill bonus +3
Chain Shirt armor-check penalty -2
```

So the total is:

```text
1 + 3 + 3 - 2 = 5
```

These values are local computed outputs only. They are not oracle-checked parity.

## Support boundary
This slice may support only the exact bounded skill/equipment-effect posture needed for the accepted deterministic pilot.

That means:

- it may rely on the already-computed ability modifiers
- it may rely on the already-grounded Fighter class posture
- it may rely on the already-chosen deterministic rank allocations in the accepted fixture
- it may apply only the already-grounded Chain Shirt armor-check penalty
- it must **not** broaden into a general skill engine
- it must **not** generalize to arbitrary classes, arbitrary equipment effects, feat-based skill bonuses, racial skill bonuses, or encumbrance breadth
- if the grounded selected posture is absent or widened, it must refuse to fabricate selected skill totals and must emit claim-blocking diagnostics

It is acceptable for this slice to leave unsupported cases at zero totals when paired with explicit claim-blocking diagnostics and withheld selected-skill explanation records.

## Explanation requirements
The computation result must include explanation records sufficient for tests to assert why each new bounded skill total exists.

At minimum, add machine-checkable explanation ids or equivalent fields for:

```text
skill.selected_modifier.climb
skill.selected_modifier.intimidate
skill.selected_modifier.swim
```

The explanation detail for each selected skill total must mention:

- the chosen rank allocation
- the key ability modifier used
- the class-skill bonus when present
- the Chain Shirt armor-check penalty when present
- the final total

Tests must assert contributors by ids and/or detail content, not by vague prose only.

## Diagnostic requirements
The result must preserve explicit diagnostics for unsupported inputs that would make this narrow skill slice dishonest.

Minimum diagnostic behavior:

- if the deterministic Fighter level-1 chassis is absent, do not silently compute selected skill totals
- if the required selected skill allocation is missing or widened beyond this slice, do not silently compute selected skill totals
- if the deterministic Chain Shirt posture needed for Climb/Swim is absent or unsupported, do not silently compute selected skill totals that pretend the equipment-effect surface is grounded
- when selected skill totals are unsupported, do not emit selected-skill explanation records that pretend those totals were grounded

You may preserve already-supported F2a, F2b, and F2c outputs and explanations where appropriate.

## Strict TDD sequence
This handoff requires the following sequence.

### RED
1. Create `tests/ge06_pilot_selected_skill_modifiers.rs` first.
2. Load `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt` through the existing loader and call the existing GE-06 pilot compute entry point.
3. Assert Climb `5`, Intimidate `3`, Swim `5`, and explanation details proving the exact bounded contributors.
4. Add at least two unsupported-posture tests by mutating fixture text in memory:
   - one that removes or widens a selected skill allocation (for example, remove `skill=skill:swim:1`)
   - one that breaks the deterministic Chain Shirt posture (for example, change `equipment=item:chain_shirt:equipped_worn_active` to `equipment=item:chain_shirt:absent`)
5. Prove claim-blocking diagnostics appear and selected-skill explanations are withheld for those unsupported cases.
6. Run:

```bash
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_selected_skill_modifiers --quiet
```

7. Confirm it fails for the expected reason: the new selected-skill field/behavior and explanation ids do not exist yet.

If the test passes immediately, it is too weak or not testing new behavior.

### GREEN
Implement the smallest change needed to pass:

- extend `src/rules_core/pilot_compute.rs`
- do not modify `character_input.rs`
- do not modify the deterministic fixture or prior proof tests
- do not add broad skill-engine helpers that generalize beyond Climb / Intimidate / Swim

### VERIFY
Run:

```bash
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_selected_skill_modifiers --quiet
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_base_computation --quiet
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_combat_baseline --quiet
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_total_saves --quiet
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

- [ ] A new GE-06 selected-skill proof test exists at `tests/ge06_pilot_selected_skill_modifiers.rs`.
- [ ] The new test fails before production code changes.
- [ ] `src/rules_core/pilot_compute.rs` is the only production file changed.
- [ ] The compute surface returns Climb `5` for the deterministic fixture.
- [ ] The compute surface returns Intimidate `3` for the deterministic fixture.
- [ ] The compute surface returns Swim `5` for the deterministic fixture.
- [ ] Selected-skill explanation records exist for Climb, Intimidate, and Swim and each cites rank, key ability modifier, class-skill bonus when present, armor-check penalty when present, and the final total.
- [ ] Unsupported selected-skill posture produces claim-blocking diagnostics rather than fabricated totals.
- [ ] Unsupported selected-skill posture withholds selected-skill explanation records.
- [ ] No broad skill engine, feat/racial/item-skill bonus logic, encumbrance breadth, parity, report, importer, PCGen runner, or UI work is added.
- [ ] All verification commands pass.
- [ ] Scope audit shows only authorized files changed or newly created.

## Non-goals
Do not implement:

- Acrobatics, Stealth, Ride, or any skill outside Climb / Intimidate / Swim
- broad class-skill resolution for arbitrary classes
- favored-class skill-rank changes
- feat-based skill modifiers
- racial skill modifiers
- temporary or situational modifiers
- speed-dependent Swim or Climb rules
- armor-check effects from armor other than the deterministic Chain Shirt posture
- shield penalties or broader equipment-effect propagation
- attack, damage, initiative, armor class, or save changes beyond preserving already merged behavior
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
- whether the branch is ready for Todd to open or merge the PR
- any blockers or deviations

## Final rule
This handoff exists to prove the first deterministic selected skill modifiers from the accepted pilot input while keeping the equipment-effect surface narrow and honest. Compute only Climb, Intimidate, and Swim with the already-grounded Chain Shirt armor-check effect, preserve prior merged behavior, refuse widened postures, and stop there.
