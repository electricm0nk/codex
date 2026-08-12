---
title: GE06-E2-F2b Execution Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE06-E2-F2b — Baseline melee attack bonus and armor class under deterministic loadout
workflow_route: readiness-closure
readiness: codex-ready
handoff_created: true
created_handoff: ./ge06-e2-f2b-execution-handoff-2026-06-21.md
review_date: 2026-06-21
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE06-E2-F2b Execution Readiness Closure

## Verdict
GE-06 may advance to the next narrow code-producing handoff.

The active handoff is:

```text
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2b-execution-handoff-2026-06-21.md
```

This closure itself is not code authority. It records why the separate stage-specific handoff may carry `code_authority: true`.

## Core problem
GE06-E2-F2a proved only the first base compute foothold: ability modifiers plus Fighter level-1 class chassis. GE-06 still cannot compute the first deterministic combat-facing totals from the accepted pilot loadout.

## Selected bounded slice

```text
GE06-E2-F2b — Baseline melee attack bonus and armor class under deterministic loadout
```

This slice is deliberately narrow. It computes only:

- baseline melee attack bonus for the deterministic Longsword-primary loadout
- baseline armor class for Chain Shirt worn, DEX 14, Dodge selected, and no shield
- explanation records for each bounded output naming the exact contributors
- explicit claim-blocking diagnostics when the required deterministic loadout/choice posture is absent or unsupported

This slice does **not** compute damage, active Power Attack penalties/bonuses, initiative, skill modifiers, armor-check penalties, encumbrance, parity, UI, or broader proficiency/choice systems.

## Required source evidence recovered

| Gate | Evidence |
|---|---|
| Prior merged foothold | GE06-E2-F2a merged into `origin/develop` at `760c9b0`. |
| Target repo exists | `/home/ubuntu/workspace/repos/codex`. |
| Branch policy | Start from current `origin/develop`, target PR to `develop`. |
| Baseline tests | `"$HOME/.cargo/bin/cargo" test --quiet` passes on a detached worktree at `origin/develop` commit `760c9b0`. |
| Existing compute surface | `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`. |
| Existing prior-proof test | `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_base_computation.rs`. |
| Input fixture | `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`. |
| Active-state model | `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs` with `EquippedActive`, `Absent`, and `SelectedInactive`. |
| Fighter chassis grounding | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst:139-143`. |
| Chain Shirt grounding | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst:40`. |
| Longsword grounding | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst:165` and `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_profs_weapon.lst:57`. |
| Dodge grounding | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_feats.lst:53`. |
| Weapon Focus grounding | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_feats.lst:184`. |
| Deterministic Weapon Focus choice | fixture line `choice=choice:fighter_bonus_feat:feat:weapon_focus:weapon:longsword`. |
| GE-04 computation requirements | `../GE-04-rules-engine-and-explainability-core/technical-requirements.md` TR-04-010 and TR-04-011. |

## Grounded expected values

Using the accepted deterministic input fixture and keeping Power Attack selected but inactive:

### Baseline melee attack bonus

```text
Fighter BAB (+1) + STR modifier (+3) + Weapon Focus (Longsword) (+1) = +5
```

### Baseline armor class

```text
10 + Chain Shirt armor bonus (+4) + DEX contribution (+2, within MAXDEX 4) + Dodge (+1) + no shield (+0) = 17
```

These values are local computed outputs only. They are not oracle-checked parity.

## Gate table

| Gate | Status | Resolution |
|---|---|---|
| Prior foothold merged | pass | GE06-E2-F2a is on `origin/develop` at `760c9b0`. |
| Bounded implementation slice selected | pass | GE06-E2-F2b is limited to baseline melee attack bonus and armor class under the deterministic loadout only. |
| Target repo/workdir exists | pass | `/home/ubuntu/workspace/repos/codex`. |
| Branch policy explicit | pass | Branch from current `origin/develop`, PR to `develop`. |
| Allowed write scope explicit | pass | Extend `pilot_compute.rs` and add one new GE-06 combat-baseline test only. |
| Runtime instruction surface exists | pass | Repo `AGENTS.md` exists and requires strict TDD. |
| Toolchain grounded | pass | Full test suite passes when invoked as `"$HOME/.cargo/bin/cargo" test --quiet`; current shell PATH does not expose `cargo` by default. |
| Verification commands known | pass | Exact per-test and full-suite commands are named below. |
| Non-goals explicit | pass | Excludes damage, active Power Attack math, skills, parity, importer expansion, and UI. |
| Harness route explicit | pass | Stage-specific execution handoff runs in Claude Code / equivalent frontier coding harness. |

## Authorized write scope for the derived handoff

The derived handoff may authorize writes only to:

```text
src/rules_core/pilot_compute.rs
tests/ge06_pilot_combat_baseline.rs
```

It may read but must not modify the prior GE06-E2-F2a proof files:

```text
tests/ge06_pilot_base_computation.rs
tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
src/rules_core/character_input.rs
```

If a compile break proves another file is required, stop and report the blocker rather than widening silently.

## Required TDD posture

The coding harness must:

1. create the failing GE-06 combat-baseline test first
2. run the specific test and capture RED
3. implement the smallest additions inside the existing `pilot_compute.rs` surface needed to pass
4. run the specific test and capture GREEN
5. re-run the prior F2a proof test
6. run full `"$HOME/.cargo/bin/cargo" test --quiet`
7. run a file-granular scope audit

## Explicit non-goals

The derived handoff must not authorize:

- weapon damage
- active Power Attack penalties or bonuses
- iterative attacks
- ranged attacks
- initiative
- skill modifiers
- armor-check penalty application
- encumbrance or inventory breadth
- feat prerequisite evaluation beyond the exact deterministic posture needed to refuse unsupported input
- imported source-package conversion
- oracle comparison or claim `Oracle-checked`
- normalization engine
- parity report writer
- PCGen execution
- UI, view-model, desktop shell, or export-sheet work

## Claim tier after this slice

If the handoff succeeds, GE-06 may claim:

```text
selected pilot input contract: represented
ability modifiers: computed with explanations
Fighter base BAB/save chassis: computed with explanations
baseline melee attack bonus: computed with explanation
baseline armor class: computed with explanation
weapon damage / active Power Attack / skills / parity / UI: not yet
```

## Completion rule

This closure is complete when `artifacts/ge06-e2-f2b-execution-handoff-2026-06-21.md` exists, carries `code_authority: true`, names GE06-E2-F2b, enforces strict TDD, lists exact allowed repo paths, excludes broader combat/parity/UI scope, and gives runnable verification commands using the truthful cargo-path posture.