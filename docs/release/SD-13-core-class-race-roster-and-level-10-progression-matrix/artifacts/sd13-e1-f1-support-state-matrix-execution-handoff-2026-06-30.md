---
title: SD13-E1-F1 Execution Handoff — Machine-usable support-state matrix and seeded current truth
handoff_id: HANDOFF-CODEX-SD-13-E1-F1-CODING-2026-06-30
stc_id: STC-CODEX-SD-13
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: autonomous-launch-authorized
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e1-f1-support-state-matrix-execution-handoff-2026-06-30.md
source_stc: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md
source_epic_breakdown: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md
selected_slice: SD13-E1-F1 — Matrix schema and seeded current-state rows
run_in: Claude Code only
code_authority: true
authority_dependencies:
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-design.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/coverage-evidence-and-fixture-plan.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: develop
  expected_base_sha_at_creation: c2cea5c6baeb3ca34077b85331214c4b42a4809c
  recommended_branch: feat/sd13-e1-f1-support-state-matrix
  pr_target: develop
allowed_write_scope:
  - src/oracle_validation/mod.rs
  - src/oracle_validation/support_state_matrix.rs
  - tests/sd13_support_state_matrix.rs
  - tests/fixtures/sd13/**
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen/**
  - programs/codex/**
  - Cargo.toml
  - Cargo.lock
  - src/lib.rs
  - src/rules_core/**
  - src/homebrew_authoring/**
  - src/pcgen_import/**
  - src/oracle_validation/golden_fixture.rs
  - src/oracle_validation/selected_parity_dimensions.rs
  - tests/character_input_record.rs
  - tests/ge06_pilot_input_contract.rs
  - tests/ge06_pilot_total_saves.rs
  - tests/ge06_selected_parity_dimensions.rs
  - tests/golden_case_fixture_schema.rs
  - AGENTS.md
  - CLAUDE.md
reviewed_at: 2026-06-30
---

# SD13-E1-F1 Execution Handoff — Machine-usable support-state matrix and seeded current truth

## Status
This is the stage-specific code-authorizing brief for SD13-E1-F1.

It carries `code_authority: true` for the first SD-13 implementation slice only. It does not claim branch, PR, merge, or Claude execution evidence itself. That truth belongs to the downstream governed CODE lane and its durable `claude-execution-receipt`.

Board routing already exists and must remain separate:
- handoff-artifact card: `t_8abe6d84`
- downstream Claude-only CODE lane: `t_0dfc10b1`

Legacy note: `t_e5dfc059` was created under the older launch-review doctrine and should be retired once this handoff is reconciled with the live CODE lane.

## Run in
Claude Code only.

Do not execute this implementation primarily through Hermes file editing. Hermes authored this handoff. Claude Code implements it. If Claude Code is unavailable, block the lane instead of substituting another coding harness.

## Core problem
SD-13 now has documentary truth, but not machine-usable truth.

The repo currently exposes only two oracle-validation carriers:
- `src/oracle_validation/golden_fixture.rs` for the GE-05 golden-case schema
- `src/oracle_validation/selected_parity_dimensions.rs` for the GE-06 selected-parity-dimension carrier

There is no typed Codex surface yet for the SD-13 support-state matrix. The current bounded breadth truth exists only in the SD-13 documents. If the first code lane improvises row shape, state vocabulary, evidence posture, or seed content ad hoc, every later breadth claim will inherit counterfeit authority.

The first honest move is therefore narrower than “implement more classes and races.” Establish the machine-usable matrix schema and seed the current truthful rows exactly as already grounded by the SD-13 packet and the existing GE-06 repo evidence.

## Objective
Implement the smallest truthful Rust-first SD-13 control-plane slice.

The result must prove all of the following:
1. `src/oracle_validation/support_state_matrix.rs` defines a typed machine-usable SD-13 support-state matrix surface
2. the surface keeps support state and evidence tier as separate axes
3. the surface encodes only the three SD-13 row types (`race`, `class`, `interaction`)
4. the surface seeds the exact current truthful rows already documented for SD13-E1-F1 instead of inventing broader support
5. the seeded matrix keeps the Human/Fighter pilot ceiling, Fighter level-2+ block, Rogue block, and unverified breadth rows explicit
6. no seeded row is silently promoted to `supported`
7. no 7 x 11 combination theater, claim-composition engine, UI/reporting layer, or broader rules implementation is smuggled into this slice

This slice stops at typed schema plus seeded current truth. It does not claim broader roster execution.

## Why this route is authorized now
The source STC and epic breakdown already fix SD13-E1-F1 as the first truthful executable slice. The live repo also fixes the exact bounded seam:
- `epic-breakdown.md` explicitly says the first execution slice should begin from matrix schema and seeded current-state rows, not a fake breadth sprint
- `artifacts/core-roster-and-support-state-matrix.md` already names the support-state taxonomy, evidence-tier axis, row model, and seeded current rows
- `artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md` already fixes the visible debt posture that the matrix must not hide
- `src/oracle_validation/mod.rs` currently exports only `golden_fixture` and `selected_parity_dimensions`, proving the new matrix surface does not exist yet
- live repo grounding on 2026-06-30 shows the local checkout is still `sd11-f10-update-action-surface` at `a0a90dcdf75cd0cc09d4b71e9fb7d3b440aaf293`, while `origin/develop` is `c2cea5c6baeb3ca34077b85331214c4b42a4809c`; the current local branch is not the truthful base for this slice
- the existing GE-06 tests already provide the only accepted computed pilot evidence that the seed is allowed to elevate above `Observed`

## Target repo / workdir
```text
repo:    /home/ubuntu/workspace/repos/codex
workdir: /home/ubuntu/workspace/repos/codex
```

Current grounded repo facts during handoff authoring:
- current local branch: `sd11-f10-update-action-surface`
- current local `HEAD`: `a0a90dcdf75cd0cc09d4b71e9fb7d3b440aaf293`
- current accepted remote base for this lane: `origin/develop` at `c2cea5c6baeb3ca34077b85331214c4b42a4809c`
- `AGENTS.md` and `CLAUDE.md` are read-only conduct surfaces for this lane

## Branch policy
Launch from a fresh `origin/develop`-based branch, not from the current local checkout branch.

Use this exact setup:
```bash
git fetch origin --prune
git switch develop --track origin/develop 2>/dev/null || git switch develop
git pull --ff-only origin develop
git switch -c feat/sd13-e1-f1-support-state-matrix
```

If `feat/sd13-e1-f1-support-state-matrix` already exists, use it only after confirming it still belongs exclusively to this slice.

Do not continue implementation on `sd11-f10-update-action-surface`.

## Required reads before coding
Read these first, in order:
1. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
2. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
3. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e1-f1-support-state-matrix-execution-handoff-2026-06-30.md`
4. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md`
5. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md`
6. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-design.md`
7. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md`
8. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
9. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
10. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/coverage-evidence-and-fixture-plan.md`
11. `/home/ubuntu/workspace/repos/codex/src/oracle_validation/mod.rs`
12. `/home/ubuntu/workspace/repos/codex/src/oracle_validation/golden_fixture.rs`
13. `/home/ubuntu/workspace/repos/codex/src/oracle_validation/selected_parity_dimensions.rs`
14. `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
15. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
16. `/home/ubuntu/workspace/repos/codex/tests/character_input_record.rs`
17. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
18. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
19. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
20. `/home/ubuntu/workspace/repos/codex/tests/ge06_selected_parity_dimensions.rs`
21. `/home/ubuntu/workspace/repos/codex/tests/golden_case_fixture_schema.rs`

Use these as bounded authority surfaces, not as permission to widen scope.

## Conditional reads
Read these only if the corresponding condition actually occurs:
1. `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
   - only if you need the exact chosen-input tokens behind the Human/Fighter pilot rows beyond what the GE-06 tests already assert
2. `/home/ubuntu/workspace/repos/codex/Cargo.toml`
   - only if you think a new dependency or crate-topology change is required
   - if this file would need to change, stop and block the lane instead of widening scope
3. extra files under `/home/ubuntu/workspace/repos/codex/src/oracle_validation/**`
   - only to confirm read-only neighboring style or comments
   - reading them is not permission to edit them

## Exact allowed write scope
You may create or modify only these paths under `/home/ubuntu/workspace/repos/codex`:
```text
src/oracle_validation/mod.rs
src/oracle_validation/support_state_matrix.rs
tests/sd13_support_state_matrix.rs
tests/fixtures/sd13/**
```

Write-scope interpretation:
- `src/oracle_validation/mod.rs` may change only to expose the new support-state-matrix surface
- `src/oracle_validation/support_state_matrix.rs` is the only implementation surface for this slice
- `tests/sd13_support_state_matrix.rs` is the only required new test surface for this slice
- `tests/fixtures/sd13/**` is optional support scope for deterministic SD-13 fixture text if it materially helps test the seed or a negative case without widening the design

Do not modify any other repo file.

## Forbidden write scope and explicit non-goals
This handoff does not authorize:
- edits under `src/rules_core/**`, `src/homebrew_authoring/**`, `src/pcgen_import/**`, or any other module outside `src/oracle_validation/support_state_matrix.rs`
- edits to `src/oracle_validation/golden_fixture.rs` or `src/oracle_validation/selected_parity_dimensions.rs`
- edits under `programs/codex/**`
- edits to `Cargo.toml`, `Cargo.lock`, `src/lib.rs`, `AGENTS.md`, or `CLAUDE.md`
- edits to existing GE-05 or GE-06 integration tests
- implementing race semantics, class progression, spellcasting burden, prerequisite logic, UI rendering, tester wording, distribution/update behavior, persistence behavior, or claim-composition policy
- a generalized matrix persistence framework, serde-based import/export system, or broad DSL for future rows
- any 7 x 11 race/class combination matrix or synthetic “core support complete” posture
- any row state or evidence tier not already authorized by the SD-13 packet

If you need any of the above to make the slice pass, stop and block the lane rather than widening it.

## Contract to implement
Implement one new bounded `oracle_validation::support_state_matrix` module that introduces a typed machine-usable SD-13 current-truth matrix carrier.

### Required module shape
The preferred minimal shape is:
```text
src/oracle_validation/support_state_matrix.rs
```

Expose the module from:
```text
src/oracle_validation/mod.rs
```

Do not create extra implementation files unless you must place deterministic text fixtures under `tests/fixtures/sd13/**` for test support.

### Required typed schema
The new module must expose a typed surface equivalent to the following minimum contract:
- `SupportState`
  - `Supported`
  - `Partial`
  - `Lossy`
  - `Blocked`
  - `Unverified`
- `EvidenceTier`
  - `Observed`
  - `Parsed`
  - `Converted`
  - `Computed`
  - `OracleChecked`
  - `ProductVisible`
- `MatrixSubjectType`
  - `Race`
  - `Class`
  - `Interaction`
- `SupportStateRow`
  - stable `row_id`
  - `subject_type`
  - `subject_id`
  - `dimension`
  - `support_state`
  - `evidence_tier`
  - `grounding_ref`
  - `blocker_or_lossiness_note` (`Option<String>` or equally explicit equivalent)
  - `next_required_uplift`
- `SupportStateMatrix`
  - `rows: Vec<SupportStateRow>`
- one deterministic constructor or accessor equivalent to `seeded_sd13_e1_f1_current_truth()`
- one narrow lookup helper equivalent to `row(&self, row_id: &str) -> Option<&SupportStateRow>`

Critical rules:
- keep support state and evidence tier as separate fields
- do not add a cross-cutting row type in this slice; the matrix row types stay limited to `Race`, `Class`, and `Interaction`
- do not add claim-composition logic, promotion rules, or mutable update orchestration in this slice
- do not add serialization dependencies or a generic persistence layer for future matrix editing

### Required seeded current-truth rows
The deterministic seed must contain exactly 21 rows: 7 race rows, 12 class rows, and 2 interaction rows.

Use these exact row ids and semantic postures:

| row_id | subject_type | subject_id | dimension | state | evidence | grounding minimum | next_required_uplift |
|---|---|---|---|---|---|---|---|
| `race.human.pilot_semantics` | `Race` | `race:human` | bounded pilot race semantics actually exercised by the GE-06 deterministic proof | `Partial` | `Computed` | GE-06 deterministic pilot evidence | classify remaining Human race semantics explicitly |
| `race.dwarf.bounded_semantics` | `Race` | `race:dwarf` | bounded race semantics | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E2 race-semantic slice |
| `race.elf.bounded_semantics` | `Race` | `race:elf` | bounded race semantics | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E2 race-semantic slice |
| `race.gnome.bounded_semantics` | `Race` | `race:gnome` | bounded race semantics | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E2 race-semantic slice |
| `race.half_elf.bounded_semantics` | `Race` | `race:half-elf` | bounded race semantics | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E2 race-semantic slice |
| `race.half_orc.bounded_semantics` | `Race` | `race:half-orc` | bounded race semantics | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E2 race-semantic slice |
| `race.halfling.bounded_semantics` | `Race` | `race:halfling` | bounded race semantics | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E2 race-semantic slice |
| `class.fighter.level_1_pilot` | `Class` | `class:fighter` | class progression through level 1 deterministic pilot surface | `Partial` | `Computed` | GE-06 deterministic fixture plus bounded save/combat/selected-dimension evidence | widen beyond level 1 and classify mandatory level-10 milestones |
| `class.fighter.levels_2_10` | `Class` | `class:fighter` | class progression through levels 2-10 | `Blocked` | `Computed` | GE-06 tests explicitly claim-block Fighter level 2 | SD13-E3 martial progression slice |
| `class.rogue.bounded_progression` | `Class` | `class:rogue` | bounded class progression | `Blocked` | `Computed` | GE-06 total-save test explicitly claim-blocks Rogue level 1 | SD13-E3 martial progression slice |
| `class.barbarian.bounded_progression` | `Class` | `class:barbarian` | bounded class progression | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E3 martial progression slice |
| `class.bard.progression_and_spell_burden` | `Class` | `class:bard` | bounded class progression and spell burden | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E4 spellcasting slice |
| `class.cleric.progression_and_spell_burden` | `Class` | `class:cleric` | bounded class progression and spell burden | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E4 spellcasting slice |
| `class.druid.progression_and_spell_burden` | `Class` | `class:druid` | bounded class progression and spell burden | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E4 spellcasting slice |
| `class.monk.bounded_progression` | `Class` | `class:monk` | bounded class progression | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E3 martial progression slice |
| `class.paladin.hybrid_chassis_and_spell_burden` | `Class` | `class:paladin` | bounded class progression and hybrid spell burden | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E3 then SD13-E4 |
| `class.ranger.hybrid_chassis_and_spell_burden` | `Class` | `class:ranger` | bounded class progression and hybrid spell burden | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E3 then SD13-E4 |
| `class.sorcerer.progression_and_spell_burden` | `Class` | `class:sorcerer` | bounded class progression and spell burden | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E4 spellcasting slice |
| `class.wizard.progression_and_spell_burden` | `Class` | `class:wizard` | bounded class progression and spell burden | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E4 spellcasting slice |
| `interaction.human_bonus_feat_ability_bonus.pilot_pressure` | `Interaction` | `interaction:human-bonus-feat-ability-bonus` | race/class interaction pressure on the deterministic pilot path | `Partial` | `Computed` | GE-06 deterministic Human Fighter pilot selections | SD13-E2 / SD13-E3 coupling |
| `interaction.non_human_any_class.progression_pressure` | `Interaction` | `interaction:non-human-any-class-progression` | race/class interaction pressure beyond the pilot | `Unverified` | `Observed` | no accepted repo evidence yet | add named interaction rows only where separate race and class rows are insufficient |

Additional seed rules:
- there must be no `Supported` rows in this initial seed
- only the Human pilot race row, Fighter level-1 row, Fighter levels-2-10 row, Rogue row, and Human interaction row may rise above `Observed`
- `grounding_ref` must cite real doc or repo evidence, not chat prose or invented receipts
- `blocker_or_lossiness_note` must be populated for every `Blocked` row and every `Lossy` row if one appears later; in this seed there should be no `Lossy` rows
- the seed must not collapse Fighter level 1 and Fighter levels 2-10 into one row
- the seed must not invent rows for multiclassing, prestige classes, or non-core books

### Grounding-ref expectations
Use real references from the current repo/docs. Minimum truthful posture:
- observed-only rows may cite the SD-13 artifact or technical-requirements path that authorizes the roster member
- computed rows must cite GE-06 evidence paths such as:
  - `tests/ge06_pilot_input_contract.rs`
  - `tests/ge06_pilot_total_saves.rs`
  - `tests/ge06_pilot_combat_baseline.rs`
  - `tests/ge06_selected_parity_dimensions.rs`
  - `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
- blocked rows must cite the exact GE-06 test that claim-blocks the posture

### Optional fixture posture
You may use `tests/fixtures/sd13/**` only if it helps you keep the seed or a negative-case expectation deterministic.

Allowed uses:
- a checked-in snapshot of expected seeded row ids or expected grounding refs
- a tiny negative-case text sample if you add a narrow constructor/validator that needs one

Not allowed:
- inventing a general import format for future SD-13 updates
- adding serde, JSON, YAML, TOML, or other new dependency-driven persistence just to hold the seed
- moving the source of truth out of the typed Rust surface into a broad editable external file format

## TDD requirement
TDD is mandatory.

Execution order:
1. create `tests/sd13_support_state_matrix.rs` first
2. if you use deterministic SD-13 fixture support files, create them under `tests/fixtures/sd13/**` before production code
3. run the new SD-13 test target and capture the real RED failure
4. implement the smallest code required inside `src/oracle_validation/support_state_matrix.rs` and `src/oracle_validation/mod.rs`
5. rerun the SD-13 test target to green
6. rerun the regression verification commands below

Important RED rule:
- a missing test target error is not sufficient
- the RED phase must express the intended seed shape and row-state expectations explicitly, even if the first failure is a compile failure against the missing module or missing constructor

### Minimum RED assertions
The failing tests should prove at least:
1. `seeded_sd13_e1_f1_current_truth()` (or equivalent) returns exactly 21 rows
2. the seed exposes the exact row ids above and no extras
3. the Human pilot race row is `Partial` + `Computed`
4. the Fighter level-1 row is `Partial` + `Computed`
5. the Fighter levels-2-10 row is `Blocked` + `Computed` with a non-empty blocker note
6. the Rogue row is `Blocked` + `Computed` with a non-empty blocker note
7. every remaining non-Human race row is `Unverified` + `Observed`
8. every remaining non-Fighter/non-Rogue class row is `Unverified` + `Observed`
9. there are no `Supported` rows in the seed
10. the lookup helper can retrieve at least the Human, Fighter, Rogue, and Human-interaction rows by id

## Verification commands
Run these at minimum.

From `/home/ubuntu/workspace/repos/codex`:
```bash
cargo test --test sd13_support_state_matrix
cargo test --test character_input_record --test ge06_pilot_input_contract --test ge06_pilot_total_saves --test ge06_pilot_combat_baseline --test ge06_selected_parity_dimensions --test golden_case_fixture_schema --test sd13_support_state_matrix
```

From `/home/ubuntu/workspace/repos/codex/apps/desktop`:
```bash
npm run tauri:check
```

Verification interpretation:
- the first command proves the new SD-13 slice itself
- the second command proves the new slice did not regress the exact upstream GE-04 / GE-05 / GE-06 evidence surfaces it depends on
- `tauri:check` proves the crate still remains acceptable to the desktop consumer without authorizing desktop changes

## Stop conditions
Stop and block the lane instead of widening it if any of these occur:
- truthful implementation requires edits outside `src/oracle_validation/mod.rs`, `src/oracle_validation/support_state_matrix.rs`, `tests/sd13_support_state_matrix.rs`, or `tests/fixtures/sd13/**`
- truthful implementation requires dependency changes in `Cargo.toml` or `Cargo.lock`
- truthful implementation appears to require changing existing GE-05 or GE-06 implementation/tests instead of consuming them as read-only grounding
- truthful implementation appears to require claim-composition logic, UI/reporting surfaces, matrix persistence infrastructure, or broader class/race semantics
- the repo cannot be refreshed to a clean `origin/develop`-based execution branch
- `cargo test --test character_input_record --test ge06_pilot_input_contract --test ge06_pilot_total_saves --test ge06_pilot_combat_baseline --test ge06_selected_parity_dimensions --test golden_case_fixture_schema --test sd13_support_state_matrix` or `npm run tauri:check` fails after the bounded change

If any stop condition lands, do not improvise. Block the card with the exact broader surface now required.

## Expected completion class
This lane is complete only at `pr-created` truth:
- fresh branch launched from `origin/develop`
- bounded changes confined to the allowed write scope
- branch pushed to `origin`
- normal PR opened against `develop`
- durable Claude receipt attached to the governed CODE card

This handoff does not authorize merge to `develop` or `main`.

## Required Claude receipt
Before the downstream CODE card completes, add a durable `claude-execution-receipt` comment that records:
- exact handoff path
- invocation mode
- repo/workdir
- branch and base SHA at launch
- durable Claude session/process handle when available, or `unknown`
- model identity when available, or `unknown`
- files changed
- RED failure summary
- verification commands run and their real results
- resulting commit and PR handle
- final completion class (`pr-created` or truthful blocker)

Without that receipt, this lane must not be described as Claude-executed.

## Merge authority boundary
This handoff authorizes only the bounded implementation slice above.

It does not authorize:
- merging the branch or PR
- landing code onto `develop` or `main`
- broadening into SD13-E2 race semantics, SD13-E3 progression burden, SD13-E4 spell burden, SD13-E6 tester reporting, or any other later lane
- any write outside the exact allowed scope

Stop at verified `pr-created` state and return control to Todd through the governed review surface.
