---
title: GE06-E2-F3 Execution Handoff — End-to-End Headless Receipt Path
handoff_id: HANDOFF-CODEX-GE-06-E2-F3-CODING-2026-06-21
stc_id: STC-CODEX-GE-06
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: awaiting-todd-launch
owner: Todd Hintzmann
scope: program
canonical: false
canonical_path: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/execution-handoff.md
source_stc: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md
readiness_closure: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f3-execution-readiness-closure-2026-06-21.md
selected_slice: GE06-E2-F3 — End-to-end command and receipt path
run_in: Claude Code or equivalent frontier coding harness
code_authority: true
created_at: 2026-06-21
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  expected_base_sha_at_creation: 2deb11b
  recommended_branch: ge06-e2-f3-headless-receipt-path
  pr_target: develop
allowed_write_scope:
  - src/rules_core/pilot_compute.rs
  - tests/ge06_pilot_headless_receipt.rs
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen
  - src/main.rs
  - src/bin/**
  - src/rules_core/character_input.rs
  - src/oracle_validation/**
  - src/pcgen_import/**
  - tests/ge06_pilot_input_contract.rs
  - tests/ge06_pilot_base_computation.rs
  - tests/ge06_pilot_combat_baseline.rs
  - tests/ge06_pilot_total_saves.rs
  - tests/ge06_pilot_selected_skill_modifiers.rs
  - tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
  - programs/codex/**
  - Cargo.toml
  - Cargo.lock
  - AGENTS.md
  - CLAUDE.md
---

# GE06-E2-F3 Execution Handoff — End-to-End Headless Receipt Path

## Status
This is the active stage-specific code-authorizing brief for GE06-E2-F3. It must not overwrite any prior F1a, F2a, F2b, F2c, or F2d handoff artifact.

## Run in
Claude Code or an equivalent frontier coding harness.

This handoff carries `code_authority: true` for GE06-E2-F3 only.

## Core problem
Codex can already load the accepted deterministic GE-06 pilot fixture and compute the bounded outputs proven across F2a through F2d, but it still lacks one integrated headless receipt path that later parity or UI consumers can depend on. There is no current CLI/bin surface and no current structured receipt shape, so the next truthful slice is to expose one narrow library-level receipt path and prove it through one focused test.

## Objective
Extend the existing GE-06 pilot compute surface to produce one bounded headless receipt/result for the accepted deterministic pilot path.

The result must preserve, at minimum:

1. case identity
2. source package identity
3. one simple status that distinguishes computed evidence from blocked posture
4. the currently supported computed output set already grounded by F2a/F2b/F2c/F2d
5. explanation records or references needed downstream
6. claim-blocking diagnostics when the path is unsupported or blocked

This handoff does **not** authorize a production CLI, dependency additions, oracle comparison, report-writer architecture, UI work, importer expansion, or broad framework design.

## Required reads
Read these before editing code:

1. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
2. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
3. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f3-execution-readiness-closure-2026-06-21.md`
4. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2d-merge-receipt-2026-06-21.md`
5. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-post-f2d-handoff-rack-2026-06-21.md`
6. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/technical-requirements.md`
7. `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
8. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
9. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
10. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_base_computation.rs`
11. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
12. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
13. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_selected_skill_modifiers.rs`
14. `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
15. `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/technical-design.md` sections describing headless new-system output capture as later comparison input.

## Branch setup
From `/home/ubuntu/workspace/repos/codex`:

```bash
git fetch origin --prune
git switch develop
git reset --hard origin/develop
git switch -c ge06-e2-f3-headless-receipt-path
```

Expected base at handoff creation:

```text
origin/develop = 2deb11b
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

- Read `AGENTS.md` / `CLAUDE.md` as instruction surfaces.
- Do not add, delete, or modify `AGENTS.md`, `CLAUDE.md`, or `Cargo.lock` for this handoff.
- Do not treat pre-existing residue as part of your implementation diff.
- Your final scope audit must list only files you changed or created for this handoff.

## Merge authority boundary
This handoff does **not** authorize the coding harness to merge anything.

- Do not merge the branch or PR.
- Do not land code directly onto `develop` or `main`.
- Stop at a verified branch state and hand control back to Todd for the launch/review/merge decisions.

## Allowed write scope
You may write only:

```text
src/rules_core/pilot_compute.rs
tests/ge06_pilot_headless_receipt.rs
```

If you need any other file, stop and report the blocker.

Do not modify the deterministic input fixture, `character_input.rs`, existing proof tests, dependency manifests, or repo-root instruction files.

## Required implementation behavior
Keep the implementation inside the existing `compute` surface unless a compile blocker proves that a smaller truthful shape is impossible.

Preferred outcome:

- extend `src/rules_core/pilot_compute.rs` with one bounded receipt/result shape for the accepted deterministic GE-06 pilot path
- keep the receipt path library-first and headless
- prove it through one focused integration test
- add no CLI and no new dependency
- preserve already-merged F2a/F2b/F2c/F2d behavior exactly

Suggested bounded surface:

```rust
pub enum HeadlessReceiptStatus {
    Computed,
    Blocked,
}

pub struct PilotHeadlessReceipt {
    pub case_id: Option<String>,
    pub source_package_id: String,
    pub status: HeadlessReceiptStatus,
    pub computation: PilotBaseChassisComputation,
}

pub fn build_pilot_headless_receipt(input: &CharacterInput) -> PilotHeadlessReceipt
```

Equivalent naming is allowed only if it is smaller and clearer while remaining machine-checkable in tests and reusable by later parity/UI consumers.

## Expected computed evidence for the accepted deterministic fixture
Using `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`, the receipt must preserve the already-grounded outputs:

```yaml
case_id: pf1-crb-human-fighter-level1
source_package_id: pf1.core_rulebook
ability_modifiers:
  strength: 3
  dexterity: 2
  constitution: 2
  intelligence: 0
  wisdom: 1
  charisma: -1
base_attack_bonus: 1
base_saves:
  fortitude: 2
  reflex: 0
  will: 0
baseline_melee_attack_bonus: 5
baseline_armor_class: 17
total_saves:
  fortitude: 4
  reflex: 2
  will: 1
selected_skill_modifiers:
  climb: 5
  intimidate: 3
  swim: 5
```

The receipt must not relabel these as oracle-checked parity. They are computed headless evidence only.

## Blocker behavior
The same receipt path must also support a clear blocker posture.

Minimum blocker rule:

- if the supported deterministic posture is broken, the receipt must return a blocked status
- the receipt must preserve claim-blocking diagnostics
- the receipt must not fabricate a success state
- the receipt may preserve whatever already-supported subcomputations still exist, but the overall receipt must clearly show that the integrated path is blocked

A truthful blocker case may be created by mutating one supported prerequisite in-memory during the test, such as:

- changing `class_level=class:fighter:1` to `class_level=class:rogue:1`, or
- changing `equipment=item:chain_shirt:equipped_worn_active` to `equipment=item:chain_shirt:absent`

Use one blocker case only if it proves the integrated status transition cleanly.

## Explanation and diagnostic requirements
The receipt path must preserve access to the existing machine-checkable explanation and diagnostic surfaces.

At minimum, tests must prove the integrated receipt still exposes explanation ids already grounded by prior slices, including examples such as:

```text
ability_modifier.strength
class_chassis.base_attack_bonus
combat.baseline_melee_attack_bonus
defense.baseline_armor_class
defense.total_save.fortitude
skill.selected_modifier.climb
```

The receipt does not need a new explanation schema. It must not hide or discard the existing one.

## Strict TDD sequence
This handoff requires the following sequence.

### RED
1. Create `tests/ge06_pilot_headless_receipt.rs` first.
2. Load the accepted deterministic fixture through the existing loader and attempt to call the new receipt entry point.
3. Assert the success-path receipt preserves:
   - `case_id`
   - `source_package_id`
   - computed/evidence status
   - the already-grounded output values listed above
   - presence of representative explanation ids
   - absence of claim-blocking diagnostics for the supported deterministic fixture
4. Add one blocker-path test by mutating the fixture text in memory and asserting:
   - blocked status
   - presence of at least one claim-blocking diagnostic
   - no counterfeit success state
5. Run:

```bash
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_headless_receipt --quiet
```

6. Confirm it fails for the expected reason: the new receipt type/entry point does not exist yet.

If the test passes immediately, it is too weak or not testing new behavior.

### GREEN
Implement the smallest change needed to pass:

- extend `src/rules_core/pilot_compute.rs`
- do not modify `character_input.rs`
- do not modify the deterministic fixture or prior proof tests
- do not introduce a CLI, dependency, or broad reporting abstraction

### VERIFY
Run:

```bash
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_headless_receipt --quiet
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_input_contract --quiet
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_base_computation --quiet
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_combat_baseline --quiet
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_total_saves --quiet
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_selected_skill_modifiers --quiet
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

- [ ] A new GE-06 headless receipt proof test exists at `tests/ge06_pilot_headless_receipt.rs`.
- [ ] The new test fails before production code changes.
- [ ] `src/rules_core/pilot_compute.rs` is the only production file changed.
- [ ] The receipt preserves `case_id` and `source_package_id` for the deterministic fixture.
- [ ] The receipt reports a computed/evidence status for the supported deterministic fixture.
- [ ] The receipt exposes the already-grounded bounded outputs without changing their values.
- [ ] The receipt preserves access to representative explanation ids from prior slices.
- [ ] The supported deterministic fixture yields no claim-blocking diagnostic in the integrated receipt.
- [ ] A blocked fixture mutation yields blocked status and at least one claim-blocking diagnostic.
- [ ] No CLI, dependency addition, oracle comparison, UI work, importer expansion, or broad reporting framework is added.
- [ ] All verification commands pass.
- [ ] Scope audit shows only authorized files changed or newly created.

## Non-goals
Do not implement:

- `src/main.rs`, `src/bin/**`, or any new command crate surface
- `serde`, `serde_json`, `clap`, `anyhow`, or any new dependency
- generic receipt/report infrastructure beyond this first bounded pilot receipt path
- oracle comparison or claim `Oracle-checked`
- normalization engine
- parity report writer
- PCGen execution or exporter capture
- UI/view-model/export-sheet work
- importer expansion
- changes to `character_input.rs`
- changes to the deterministic fixture
- rewrites of prior proof tests
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
- whether the branch is ready for Todd to open the PR
- any blockers or deviations

## Final rule
This handoff exists to prove the first integrated GE-06 headless receipt path over the already-grounded deterministic pilot. Expose one small structured receipt surface, prove it by test, preserve existing computations/explanations/diagnostics, refuse counterfeit CLI or framework expansion, and stop there.
