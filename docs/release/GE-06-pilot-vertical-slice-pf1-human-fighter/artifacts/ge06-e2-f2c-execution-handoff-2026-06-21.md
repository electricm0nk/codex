---
title: GE06-E2-F2c Execution Handoff — Total Fortitude, Reflex, and Will Saving Throws Under Deterministic Ability Scores
handoff_id: HANDOFF-CODEX-GE-06-E2-F2C-CODING-2026-06-21
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
readiness_closure: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2c-execution-readiness-closure-2026-06-21.md
selected_slice: GE06-E2-F2c — Total Fortitude, Reflex, and Will saving throws under deterministic ability scores
run_in: Claude Code or equivalent frontier coding harness
code_authority: true
created_at: 2026-06-21
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  expected_base_sha_at_creation: 75c26ce
  recommended_branch: ge06-e2-f2c-total-saving-throws
  pr_target: develop
allowed_write_scope:
  - src/rules_core/pilot_compute.rs
  - tests/ge06_pilot_total_saves.rs
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen
  - src/rules_core/character_input.rs
  - src/oracle_validation/**
  - src/pcgen_import/**
  - tests/ge06_pilot_base_computation.rs
  - tests/ge06_pilot_combat_baseline.rs
  - tests/ge06_pilot_input_contract.rs
  - tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
  - UI or desktop shell paths
  - programs/codex/**
  - Cargo.toml
  - Cargo.lock
---

# GE06-E2-F2c Execution Handoff — Total Fortitude, Reflex, and Will Saving Throws Under Deterministic Ability Scores

## Status
This is the active stage-specific code-authorizing brief for GE06-E2-F2c. It must not overwrite any prior F1a, F2a, or F2b handoff artifact.

## Run in
Claude Code or an equivalent frontier coding harness.

This handoff carries `code_authority: true` for GE06-E2-F2c only.

## Core problem
GE06-E2-F2b established deterministic combat-facing totals for baseline melee attack bonus and baseline armor class. Codex still cannot compute the first deterministic **total** saving throws for the accepted pilot, and `src/rules_core/pilot_compute.rs` still contains stale F2a-era prose that falsely says the surface does not compute armor class or attack bonus.

## Objective
Extend the existing GE-06 pilot compute surface to produce and explain only:

1. total Fortitude save for the deterministic pilot
2. total Reflex save for the deterministic pilot
3. total Will save for the deterministic pilot
4. claim-blocking diagnostics when unsupported chassis/input posture would make those totals dishonest
5. truthful module/struct/file prose in `src/rules_core/pilot_compute.rs` so the comments match the already-merged F2b behavior and the newly added F2c behavior

This handoff does **not** authorize feat-based save modifiers, item-based save modifiers, conditional or situational modifiers, damage, active Power Attack math, initiative, skills, parity, UI, or broad rules-engine expansion.

## Required reads
Read these before editing code:

1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2c-execution-readiness-closure-2026-06-21.md`
3. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2b-merge-receipt-2026-06-21.md`
4. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2b-execution-handoff-2026-06-21.md`
5. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`
6. `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
7. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
8. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_base_computation.rs`
9. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
10. `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
11. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst:139-143` — Fighter level-1 base save evidence only; do not edit.
12. `programs/codex/requirements/GE-04-rules-engine-and-explainability-core/technical-requirements.md` TR-04-010 and TR-04-011.

## Branch setup
From `/home/ubuntu/workspace/repos/codex`:

```bash
git fetch origin --prune
git switch develop
git reset --hard origin/develop
git switch -c ge06-e2-f2c-total-saving-throws
```

Expected base at handoff creation:

```text
origin/develop = 75c26ce
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
tests/ge06_pilot_total_saves.rs
```

If you need any other file, stop and report the blocker.

Do not modify the deterministic input fixture, `character_input.rs`, or the prior F2a/F2b proof tests. Those are already grounded and must remain stable.

## Required implementation behavior
Keep the implementation inside the existing `compute_pilot_base_chassis` / `PilotBaseChassisComputation` surface unless a compile blocker proves that a smaller truthful shape is impossible.

Preferred outcome:

- extend `PilotBaseChassisComputation` with one bounded total-save surface
- keep one headless entry point for the GE-06 pilot base-compute path
- add no new module unless required for clarity and still within scope
- repair stale file/module/struct prose in `pilot_compute.rs` as part of the same minimal edit

Suggested new field:

```rust
total_saves: BaseSaves
```

Equivalent naming is allowed only if it is smaller and clearer while remaining machine-checkable in tests.

## Expected computed values
Using the accepted deterministic input fixture:

### Total Fortitude save
Expected total:

```yaml
fortitude: 4
```

Bounded contributors:

```text
Fighter base Fortitude: +2
Constitution modifier: +2
```

So the total is:

```text
2 + 2 = 4
```

### Total Reflex save
Expected total:

```yaml
reflex: 2
```

Bounded contributors:

```text
Fighter base Reflex: +0
Dexterity modifier: +2
```

So the total is:

```text
0 + 2 = 2
```

### Total Will save
Expected total:

```yaml
will: 1
```

Bounded contributors:

```text
Fighter base Will: +0
Wisdom modifier: +1
```

So the total is:

```text
0 + 1 = 1
```

These values are local computed outputs only. They are not oracle-checked parity.

## Support boundary
This slice may support only the exact bounded total-save posture needed for the accepted deterministic pilot and the already-grounded Fighter level-1 chassis.

That means:

- it may rely on the already-computed Fighter level-1 base saves
- it may rely on the already-computed ability modifiers
- it must **not** broaden into feat-, item-, or condition-based save modifiers
- it must **not** generalize into a broad defense engine
- if the Fighter level-1 chassis is absent or unsupported, it must refuse to fabricate total saves and must emit a claim-blocking diagnostic

It is acceptable for this slice to leave unsupported cases at zero totals when paired with explicit claim-blocking diagnostics and withheld total-save explanations.

## Mandatory prose synchronization
This handoff explicitly requires repairing the stale prose in `src/rules_core/pilot_compute.rs`.

Minimum required truth repair:

- remove or rewrite F2a-only language at the file/module/struct level
- stop claiming that armor class and attack bonus are unsupported
- truthfully describe the surface as supporting:
  - ability modifiers
  - Fighter level-1 base attack bonus
  - Fighter level-1 base saves
  - baseline melee attack bonus
  - baseline armor class
  - total Fortitude/Reflex/Will saves
- preserve explicit non-goals so the file still does not pretend to be a full rules engine

Do **not** treat this as optional cleanup. The file must leave the handoff more truthful than it entered.

## Explanation requirements
The computation result must include explanation records sufficient for tests to assert why each new bounded total exists.

At minimum, add machine-checkable explanation ids or equivalent fields for:

```text
defense.total_save.fortitude
defense.total_save.reflex
defense.total_save.will
```

The explanation detail for each total save must mention:

- the grounded Fighter base save value
- the relevant ability modifier added to that base save
- the final total

Tests must assert contributors by ids and/or detail content, not by vague prose only.

## Diagnostic requirements
The result must preserve explicit diagnostics for unsupported inputs that would make this narrow total-save slice dishonest.

Minimum diagnostic behavior:

- if the deterministic Fighter level-1 chassis is absent, do not silently compute total saves
- do not broaden into feat-, item-, or condition-based save modifiers
- when total saves are unsupported, do not emit total-save explanation records that pretend those totals were grounded

You may preserve already-supported F2a and F2b outputs and explanations where appropriate.

## Strict TDD sequence
This handoff requires the following sequence.

### RED
1. Create `tests/ge06_pilot_total_saves.rs` first.
2. Load `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt` through the existing loader and call the existing GE-06 pilot compute entry point.
3. Assert total Fortitude `4`, total Reflex `2`, total Will `1`, and explanation details proving the exact bounded contributors.
4. Add at least one unsupported-chassis test by mutating the fixture text from `class_level=class:fighter:1` to `class_level=class:rogue:1`, then prove claim-blocking diagnostics appear and total-save explanations are withheld.
5. Run:

```bash
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_total_saves --quiet
```

6. Confirm it fails for the expected reason: the new total-save field/behavior and explanation ids do not exist yet.

If the test passes immediately, it is too weak or not testing new behavior.

### GREEN
Implement the smallest change needed to pass:

- extend `src/rules_core/pilot_compute.rs`
- repair the stale prose in that same file before declaring GREEN
- do not modify `character_input.rs`
- do not modify the deterministic fixture or prior proof tests

### VERIFY
Run:

```bash
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_total_saves --quiet
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_base_computation --quiet
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_combat_baseline --quiet
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

- [ ] A new GE-06 total-saves test exists at `tests/ge06_pilot_total_saves.rs`.
- [ ] The new test fails before production code changes.
- [ ] `src/rules_core/pilot_compute.rs` is the only production file changed.
- [ ] The compute surface returns total Fortitude `4` for the deterministic fixture.
- [ ] The compute surface returns total Reflex `2` for the deterministic fixture.
- [ ] The compute surface returns total Will `1` for the deterministic fixture.
- [ ] Total-save explanation records exist for Fortitude, Reflex, and Will and each cites the base-save contributor, the relevant ability modifier, and the final total.
- [ ] Unsupported chassis/input posture produces claim-blocking diagnostics rather than fabricated total saves.
- [ ] Unsupported chassis/input posture withholds total-save explanation records.
- [ ] `pilot_compute.rs` file/module/struct prose is updated so it no longer falsely describes the surface as F2a-only or claims armor class / attack bonus are unsupported.
- [ ] No feat/item/conditional save modifiers, damage, initiative, skills, parity, report, importer, PCGen runner, or UI work is added.
- [ ] All verification commands pass.
- [ ] Scope audit shows only authorized files changed or newly created.

## Non-goals
Do not implement:

- feat-based save modifiers
- cloak/ring/item-based save modifiers
- temporary or situational bonuses
- condition-based modifiers
- damage rolls
- active Power Attack penalties or bonuses
- initiative
- skill modifiers
- armor-check penalties
- encumbrance
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
This handoff exists to prove the first deterministic total saving throws from the accepted pilot input while repairing already-known prose drift in the touched compute file. It does not authorize “defense support” in the abstract. Compute total Fortitude, Reflex, and Will saves, keep the change narrow, make the file truthful, and stop there.
