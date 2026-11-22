---
title: GE06-E2-F2a Execution Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE06-E2-F2a — Base ability modifiers and Fighter class chassis computation
workflow_route: readiness-closure
readiness: codex-ready
handoff_created: true
created_handoff: ../execution-handoff.md
review_date: 2026-06-21
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE06-E2-F2a Execution Readiness Closure

## Verdict
GE-06 may advance to the next narrow code-producing handoff.

The active handoff is:

```text
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/execution-handoff.md
```

This closure itself is not code authority. It records why the separate handoff may carry `code_authority: true`.

## Core problem
GE06-E2-F1a made the deterministic pilot input executable as chosen input. GE-06 now needs the first actual computation foothold without collapsing into the full rules engine.

## Selected bounded slice

```text
GE06-E2-F2a — Base ability modifiers and Fighter class chassis computation
```

This is a deliberately narrow first computation slice under GE06-E2-F2. It computes only:

- ability modifiers from the loaded GE-06 pilot ability scores
- Fighter level-1 base attack bonus from the grounded Fighter class row
- Fighter level-1 base save bonuses from the grounded Fighter class row
- explanation records for each computed value

It does not compute armor class, attack bonus, skill modifiers, armor-check penalties, feat prerequisites, oracle comparison, import conversion, or UI output.

## Required source evidence recovered

| Gate | Evidence |
|---|---|
| Prior merged foothold | GE06-E2-F1a merged into `develop` at `9f3cb93`. |
| Target repo exists | `/home/ubuntu/workspace/repos/codex`. |
| Branch policy | Start from current `origin/develop`, target PR to `develop`. |
| Baseline tests | `cargo test --quiet` passes on `develop` at `9f3cb93`. |
| Input fixture | `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`. |
| Input loader | `src/rules_core/character_input.rs`. |
| GE-06 deterministic contract | `artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`. |
| Fighter class chassis source | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst:139-141`. |
| GE-04 computation requirements | `../GE-04-rules-engine-and-explainability-core/technical-requirements.md` TR-04-010 and TR-04-011. |

Grounded Fighter formulas from `cr_classes.lst:139`:

```text
BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")
BONUS:SAVE|BASE.Fortitude|classlevel("APPLIEDAS=NONEPIC")/2+2
BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel("APPLIEDAS=NONEPIC")/3
```

For Fighter level 1, this bounds the first class chassis expected values:

```yaml
base_attack_bonus: 1
base_fortitude_save: 2
base_reflex_save: 0
base_will_save: 0
```

Ability modifier formula for this slice:

```text
floor(score / 2) - 5
```

For the GE-06 pilot ability scores, this bounds:

```yaml
strength: 3
dexterity: 2
constitution: 2
intelligence: 0
wisdom: 1
charisma: -1
```

These values are local computed outputs only. They are not oracle-checked parity.

## Gate table

| Gate | Status | Resolution |
|---|---|---|
| Prior foothold merged | pass | GE06-E2-F1a is on `develop` at `9f3cb93`. |
| Bounded implementation slice selected | pass | GE06-E2-F2a is limited to ability modifiers and Fighter base class chassis values. |
| Target repo/workdir exists | pass | `/home/ubuntu/workspace/repos/codex`. |
| Branch policy explicit | pass | Branch from current `origin/develop`, PR to `develop`. |
| Allowed write scope explicit | pass | New computation module/test plus minimal module exposure only. |
| Runtime instruction surface exists | pass | Repo `AGENTS.md` exists and requires strict TDD. |
| Toolchain grounded | pass | `cargo test --quiet` passes. |
| Verification commands known | pass | Specific test command plus full `cargo test --quiet`. |
| Non-goals explicit | pass | Excludes equipment, attacks, skills, parity, importer expansion, and UI. |
| Harness route explicit | pass | `execution-handoff.md` is for Claude Code / frontier coding harness. |

## Authorized write scope for the derived handoff

The derived handoff may authorize writes only to:

```text
src/rules_core/pilot_compute.rs
src/rules_core/mod.rs
tests/ge06_pilot_base_computation.rs
```

It may read but must not modify the existing GE06-E2-F1a fixture/test unless a compile break proves a minimal import-path adjustment is required. If that happens, stop and report the blocker rather than widening silently.

## Required TDD posture

The coding harness must:

1. create the failing GE-06 base computation test first
2. run the specific test and capture RED
3. implement the smallest computation/explanation model needed to pass
4. run the specific test and capture GREEN
5. run full `cargo test --quiet`
6. run a file-granular scope audit

## Explicit non-goals

The derived handoff must not authorize:

- armor class
- melee/ranged attack bonus
- weapon damage
- skill modifiers
- armor-check penalty application
- encumbrance
- feat prerequisite evaluation
- choice availability
- imported source-package conversion
- oracle comparison or claim `Oracle-checked`
- normalization engine
- parity report writer
- PCGen execution
- UI, view-model, desktop shell, or export sheet work

## Claim tier after this slice

If the handoff succeeds, GE-06 may claim:

```text
selected pilot input contract: represented
ability modifiers: computed with explanations
Fighter base BAB/save chassis: computed with explanations
attack/armor/skill/equipment-derived outputs: not yet
oracle parity: not checked
UI truth: not product-visible
```

## Completion rule

This closure is complete when `execution-handoff.md` exists, carries `code_authority: true`, names GE06-E2-F2a, enforces strict TDD, lists exact allowed repo paths, excludes broader rules-engine work, and gives runnable verification commands.
