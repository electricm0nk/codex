---
title: SD13-E1-F1 Execution Handoff — Matrix schema and seeded current-state rows
handoff_id: HANDOFF-CODEX-SD13-E1-F1-RULES-CORE-MATRIX-2026-06-30
stc_id: STC-CODEX-SD-13
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: ready-for-claude-launch
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e1-r2-matrix-schema-and-seeded-current-state-execution-handoff-2026-06-30.md
source_stc: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md
source_epic_breakdown: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md
source_readiness_closure: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e1-r1-execution-readiness-closure-2026-06-30.md
selected_slice: SD13-E1-F1 — Matrix schema and seeded current-state rows
run_in: Claude Code only
code_authority: true
authority_dependencies:
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/references/upstream-dependency-contract.md
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e1-r1-execution-readiness-closure-2026-06-30.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  observed_local_branch: sd11-f10-update-action-surface
  observed_local_head: a0a90dcdf75cd0cc09d4b71e9fb7d3b440aaf293
  expected_base_ref: origin/develop
  expected_base_sha_at_handoff_creation: c2cea5c6baeb3ca34077b85331214c4b42a4809c
  recommended_branch: feat/sd13-e1-f1-rules-core-support-state-matrix
  pr_target: develop
completion_class: pr-created
reviewed_at: 2026-06-30
---

# SD13-E1-F1 Execution Handoff — Matrix schema and seeded current-state rows

## Status
This is the stage-specific code-authorizing brief for the already-routed downstream story `SD13-E1-F1 CODE: Matrix schema and seeded current-state rows`.

It grants code authority only for the bounded slice below. It does not itself prove Claude execution, a pushed branch, a PR, or a merge. That truth belongs to the governed CODE lane and its durable `claude-execution-receipt`.

Board routing must remain:
- current documentary handoff artifact: `t_07312a8f`
- downstream CODE lane: `t_767dd7fb`

## Run in
Claude Code only.

Do not execute this implementation primarily through Hermes file-editing tools. If Claude Code cannot be launched truthfully, block the CODE lane instead of silently coding through Hermes.

## Core problem
SD-13 now has documentary matrix truth but no machine-usable repo surface that carries that truth.

The live repo currently exposes only these `rules_core` surfaces:
- `character_input`
- `pilot_compute`
- `pilot_failure`
- `pilot_view_model`

There is no typed support-state matrix module in `rules_core`, no seeded row carrier for the SD-13 current posture, and no focused proof file for that carrier. If the first SD-13 code slice improvises shape, vocabulary, or seed content ad hoc, every later breadth claim inherits counterfeit authority.

The decisive move is smaller than breadth implementation: add one typed `rules_core` matrix carrier plus one focused proof file, seeded only with truth already grounded by the SD-13 packet and the current GE-06 repo evidence.

## Objective
Implement the smallest truthful Rust-first SD-13 control-plane slice inside `rules_core`.

The result must prove all of the following:
1. `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs` defines a typed machine-usable SD-13 support-state matrix surface.
2. support state and evidence tier remain separate axes.
3. the row subject types remain limited to `race`, `class`, and `interaction`.
4. the surface seeds only the current truthful rows already authorized by SD-13 and GE-06 grounding.
5. the seed preserves the Human/Fighter pilot ceiling, the Fighter level-2+ block, the Rogue block, and the broader unverified roster explicitly.
6. no row is silently promoted to `supported`.
7. no parser, file format, UI/reporting surface, rules computation, or fake breadth sprint is smuggled into this slice.

This slice stops at typed schema plus seeded current truth. It does not claim broader roster execution.

## Why this route is authorized now
This handoff is authorized because the live repo and the readiness closure now agree on one narrow truthful seam:
- `epic-breakdown.md` explicitly says the first execution slice starts from matrix schema and seeded current-state rows, not from a fake breadth sprint.
- `artifacts/core-roster-and-support-state-matrix.md` already fixes the roster, the support-state taxonomy, the evidence-tier axis, and the seeded current-truth posture.
- `artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md` already fixes the visible debt posture that the code seed must not hide.
- `references/upstream-dependency-contract.md` fixes what upstream surfaces do and do not authorize.
- the readiness closure grounded the exact candidate write surface to `rules_core`, not UI, release, persistence, or oracle-reporting surfaces.
- the live repo still has an empty `[dependencies]` section in `Cargo.toml`, which means the smallest truthful implementation is a typed Rust module rather than a new serializer or data-carrier subsystem.
- the live GE-06 tests are the only accepted computed evidence allowed to elevate any seed rows above `Observed`.

## Target repo and branch policy
```text
repo:    /home/ubuntu/workspace/repos/codex
workdir: /home/ubuntu/workspace/repos/codex
```

Grounded repo facts at handoff creation time:
- current local branch: `sd11-f10-update-action-surface`
- current local `HEAD`: `a0a90dcdf75cd0cc09d4b71e9fb7d3b440aaf293`
- grounded remote base for this lane: `origin/develop` at `c2cea5c6baeb3ca34077b85331214c4b42a4809c`

Launch from a fresh `origin/develop`-based feature branch, not from the current local branch.

Use this exact setup:
```bash
git fetch origin --prune
git switch develop --track origin/develop 2>/dev/null || git switch develop
git pull --ff-only origin develop
git switch -c feat/sd13-e1-f1-rules-core-support-state-matrix
```

If `feat/sd13-e1-f1-rules-core-support-state-matrix` already exists, reuse it only after confirming it still belongs exclusively to this slice.

## Exact required reads before coding
Read these first, in order:
1. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
2. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
3. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e1-r2-matrix-schema-and-seeded-current-state-execution-handoff-2026-06-30.md`
4. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e1-r1-execution-readiness-closure-2026-06-30.md`
5. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md`
6. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md`
7. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
8. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
9. `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/references/upstream-dependency-contract.md`
10. `/home/ubuntu/workspace/repos/codex/README.md`
11. `/home/ubuntu/workspace/repos/codex/Cargo.toml`
12. `/home/ubuntu/workspace/repos/codex/src/rules_core/mod.rs`
13. `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
14. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
15. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_view_model.rs`
16. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
17. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
18. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
19. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_view_model.rs`
20. `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`

Use them as bounded authority surfaces, not as permission to widen scope.

## Exact allowed write scope
You may create or modify only these repo paths:
- `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
- `/home/ubuntu/workspace/repos/codex/src/rules_core/mod.rs`
- `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`

Write-scope interpretation:
- `support_state_matrix.rs` is the only implementation surface for this slice.
- `mod.rs` may change only to export the new module.
- `tests/sd13_support_state_matrix.rs` is the only new proof surface for this slice.

No other repo file is in write scope.

## Forbidden write scope and explicit non-goals
This handoff does not authorize:
- any edits under `/home/ubuntu/workspace/programs/codex/**`
- any edits under `/home/ubuntu/workspace/repos/codex/apps/desktop/**`
- any edits under `/home/ubuntu/workspace/repos/codex/src/oracle_validation/**`
- any edits under `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/**`
- any edits under `/home/ubuntu/workspace/repos/codex/src/pcgen_import/**`
- any edits to `/home/ubuntu/workspace/repos/codex/src/lib.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/Cargo.toml` or `/home/ubuntu/workspace/repos/codex/Cargo.lock`
- any edits to `/home/ubuntu/workspace/repos/codex/AGENTS.md` or `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
- any edits to `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
- any edits to `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_view_model.rs`
- any edits to existing GE-06 regression test files or existing fixtures
- any parser, serializer, file I/O surface, external schema format, or dependency addition
- any UI/workbench/status/reporting surface under SD-11
- any distribution/update/channel work under SD-12
- any persistence/lifecycle work under SD-14
- any race semantics implementation, class progression implementation, spell burden implementation, multiclassing, archetype, prestige-class, or non-core expansion work
- any claim-composition engine or 7 x 11 breadth-complete theater

If truthful completion would require touching any forbidden surface, stop and block the CODE lane instead of widening scope.

## Contract to implement
Implement one new bounded `rules_core::support_state_matrix` module exported from `rules_core::mod`.

### Required module shape
Required file surface:
```text
/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs
/home/ubuntu/workspace/repos/codex/src/rules_core/mod.rs
```

The module must remain documentary/control-plane truth only.
It must not compute character mechanics, parse external files, or project UI surfaces.

### Required typed schema
The module must expose a typed surface equivalent to this minimum contract:
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
  - `blocker_or_lossiness_note`
  - `next_required_uplift`
- `SupportStateMatrix`
  - `rows: Vec<SupportStateRow>`
- one deterministic constructor or accessor equivalent to `seeded_sd13_e1_f1_current_truth()`
- one narrow lookup helper equivalent to `row(&self, row_id: &str) -> Option<&SupportStateRow>`

Critical rules:
- keep support state and evidence tier as separate fields
- keep subject types limited to `Race`, `Class`, and `Interaction` for this slice
- do not add promotion logic, mutable update orchestration, claim assembly, or breadth scoring
- do not add serialization or persistence infrastructure

## Required seeded current-truth rows
The deterministic seed must contain exactly 21 rows: 7 race rows, 12 class rows, and 2 interaction rows.

Use these exact row ids and postures:

| row_id | subject_type | subject_id | dimension | state | evidence | grounding minimum | next_required_uplift |
|---|---|---|---|---|---|---|---|
| `race.human.pilot_semantics` | `Race` | `race:human` | bounded pilot race semantics actually exercised by the GE-06 deterministic proof | `Partial` | `Computed` | GE-06 deterministic pilot evidence | classify remaining Human race semantics explicitly |
| `race.dwarf.bounded_semantics` | `Race` | `race:dwarf` | bounded race semantics | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E2 race-semantic slice |
| `race.elf.bounded_semantics` | `Race` | `race:elf` | bounded race semantics | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E2 race-semantic slice |
| `race.gnome.bounded_semantics` | `Race` | `race:gnome` | bounded race semantics | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E2 race-semantic slice |
| `race.half_elf.bounded_semantics` | `Race` | `race:half-elf` | bounded race semantics | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E2 race-semantic slice |
| `race.half_orc.bounded_semantics` | `Race` | `race:half-orc` | bounded race semantics | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E2 race-semantic slice |
| `race.halfling.bounded_semantics` | `Race` | `race:halfling` | bounded race semantics | `Unverified` | `Observed` | SD-13 roster authority only | SD13-E2 race-semantic slice |
| `class.fighter.level_1_pilot` | `Class` | `class:fighter` | class progression through level 1 deterministic pilot surface | `Partial` | `Computed` | GE-06 deterministic fixture plus bounded save/combat/view-model evidence | widen beyond level 1 and classify mandatory level-10 milestones |
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
- `grounding_ref` must cite real doc or repo evidence, never chat prose or invented receipts
- `blocker_or_lossiness_note` must be populated for every `Blocked` row
- there should be no `Lossy` rows in this initial seed
- the seed must not collapse Fighter level 1 and Fighter levels 2-10 into one row
- the seed must not invent multiclass, prestige-class, archetype, or non-core rows

## Grounding-ref expectations
Use real references from the current repo/docs.

Minimum truthful posture:
- observed-only rows may cite the SD-13 packet paths that authorize the roster member
- computed rows must cite real GE-06 evidence such as:
  - `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_input_contract.rs`
  - `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs`
  - `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs`
  - `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_view_model.rs`
  - `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
- blocked rows must cite the exact GE-06 test that claim-blocks the posture

## TDD requirement
TDD is mandatory.

Execution order:
1. create `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs` first
2. run the new SD-13 test target and capture the real RED failure
3. implement the smallest code required inside `support_state_matrix.rs` and `mod.rs`
4. rerun the SD-13 test target to green
5. rerun the regression verification commands below

Important RED rule:
- a missing test target error alone is not sufficient
- the RED phase must express the intended seed shape and row-state expectations explicitly, even if the first failure is a compile failure against the missing module or missing constructor

Minimum RED assertions:
1. `seeded_sd13_e1_f1_current_truth()` returns exactly 21 rows
2. the seed exposes the exact row ids above and no extras
3. the Human race row is `Partial` plus `Computed`
4. the Fighter level-1 row is `Partial` plus `Computed`
5. the Fighter levels-2-10 row is `Blocked` plus `Computed` with a non-empty blocker note
6. the Rogue row is `Blocked` plus `Computed` with a non-empty blocker note
7. every remaining non-Human race row is `Unverified` plus `Observed`
8. every remaining non-Fighter/non-Rogue class row is `Unverified` plus `Observed`
9. there are no `Supported` rows in the seed
10. the lookup helper can retrieve at least the Human, Fighter, Rogue, and Human-interaction rows by id

## Exact verification commands
Run these at minimum from `/home/ubuntu/workspace/repos/codex`:
```bash
. "$HOME/.cargo/env" && cargo test --test sd13_support_state_matrix
. "$HOME/.cargo/env" && cargo test --test ge06_pilot_input_contract --test ge06_pilot_total_saves --test ge06_pilot_combat_baseline --test ge06_pilot_view_model
. "$HOME/.cargo/env" && cargo test
```

Verification interpretation:
- the first command proves the new matrix carrier itself
- the four focused GE-06 tests are mandatory regression protection because the seeded rows cite those exact current truths and blockers
- full `cargo test` is a smoke/regression sweep only; it does not upgrade SD-13 breadth claims by itself

## Stop conditions
Stop and block the CODE lane instead of widening it if any of these occur:
- truthful implementation requires edits outside the three allowed write paths
- truthful implementation requires dependency changes in `Cargo.toml` or `Cargo.lock`
- truthful implementation appears to require changing existing GE-06 implementation or tests instead of consuming them as read-only grounding
- truthful implementation appears to require UI/reporting work, persistence infrastructure, parser work, or broader class/race semantics
- the repo cannot be refreshed to a clean `origin/develop`-based execution branch
- any verification command above fails after the bounded change

## Expected completion class
This lane is complete only at `pr-created` truth:
- fresh branch launched from `origin/develop`
- bounded changes confined to the allowed write scope
- branch pushed to `origin`
- normal PR opened against `develop`
- durable Claude execution receipt attached to the governed CODE card

This handoff does not authorize merge to `develop` or `main`.

## Required Claude receipt
Before the downstream CODE card completes, add a durable `claude-execution-receipt` comment that records:
- exact handoff path
- invocation mode
- repo/workdir
- branch and base SHA at launch
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
- broadening into SD13-E2 race semantics, SD13-E3 progression burden, SD13-E4 spell burden, SD13-E6 tester reporting, or any later SD-13 lane
- any write outside the exact allowed scope

Stop at verified `pr-created` state and return control through the governed review surface.
