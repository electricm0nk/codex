---
title: GE07-E3 UI-Truth Verification Receipt
artifact_type: verification-receipt
stc_id: STC-CODEX-GE-07
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE07-E3 — Pilot character workspace shell
workflow_route: planning
readiness: planning-ready
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE07-E3 UI-Truth Verification Receipt

## Objective
Prove the minimum pilot workspace truth burden over real Codex domain outputs before any shell slice claims product progress.

## Verification commands run
In `/home/ubuntu/workspace/repos/codex`:
- `git merge-base --is-ancestor HEAD origin/develop && echo HEAD_IS_ANCESTOR_OF_ORIGIN_DEVELOP`
- `"$HOME/.cargo/bin/cargo" test --test ge06_pilot_headless_receipt --quiet`

In the temporary probe workspace `/home/ubuntu/.hermes/kanban/boards/codex/workspaces/t_41c6b298/pilot_probe`:
- `"$HOME/.cargo/bin/cargo" run --quiet`

## Observed verification results
- `git merge-base --is-ancestor HEAD origin/develop` succeeded, so the live probe ran against code already contained by the current `origin/develop` lineage.
- `cargo test --test ge06_pilot_headless_receipt --quiet` passed with `2 passed; 0 failed`.
- the probe loaded the deterministic pilot fixture at `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
- the probe emitted both the supported computed route and a deliberately blocked route by mutating `class:fighter:1` to `class:rogue:1`

## Current selections recovered from the real pilot fixture
| Group | Real selection |
|---|---|
| Race | `race:human` |
| Class level | `class:fighter:1` |
| Ability scores | STR 16, DEX 14, CON 14, INT 10, WIS 12, CHA 8 |
| Feats | `feat:power_attack`, `feat:dodge`, `feat:weapon_focus` |
| Skill ranks | `skill:climb:1`, `skill:intimidate:1`, `skill:swim:1` |
| Equipment posture | Chain Shirt active/worn; Longsword active/primary; Shield absent; Power Attack selected but inactive |
| Slot choices | level-1 character feat = Power Attack; human bonus feat = Dodge; fighter bonus feat = Weapon Focus (Longsword); human ability bonus = Strength |

## Computed route truth recovered from the real receipt
| Value group | Real output |
|---|---|
| Identity | `case_id = pf1-crb-human-fighter-level1`; `source_package_id = pf1.core_rulebook` |
| Route framing | `status = Computed`; `primary_owner = OracleGap` |
| Ability modifiers | STR 3, DEX 2, CON 2, INT 0, WIS 1, CHA -1 |
| Base chassis | BAB 1; base saves Fort 2 / Ref 0 / Will 0 |
| Combat | baseline melee attack bonus 5 |
| Defense | baseline armor class 17; total saves Fort 4 / Ref 2 / Will 1 |
| Selected skills | Climb 5 / Intimidate 3 / Swim 5 |
| Diagnostics | none (`diagnostic_count = 0`) |

## Explanation hooks already available to the workspace
The computed route already carries stable explanation ids the shell may reveal without inventing local semantics:
- `ability_modifier.strength`
- `ability_modifier.dexterity`
- `ability_modifier.constitution`
- `ability_modifier.intelligence`
- `ability_modifier.wisdom`
- `ability_modifier.charisma`
- `class_chassis.base_attack_bonus`
- `class_chassis.base_save.fortitude`
- `class_chassis.base_save.reflex`
- `class_chassis.base_save.will`
- `combat.baseline_melee_attack_bonus`
- `defense.baseline_armor_class`
- `defense.total_save.fortitude`
- `defense.total_save.reflex`
- `defense.total_save.will`
- `skill.selected_modifier.climb`
- `skill.selected_modifier.intimidate`
- `skill.selected_modifier.swim`

## Blocked route truth recovered from the real receipt lane
The blocked probe example preserved the same pilot identity while switching the route truth to:
- `status = Blocked`
- `primary_owner = EngineFlaw`
- `diagnostic_count = 4`
- downstream groups collapsing to placeholder values rather than a successful computed snapshot: base chassis `0/0/0`, combat `0`, defense `0/0/0`, selected skills `0/0/0`

Real claim-blocking diagnostics observed:
1. `class_chassis.unsupported` — base class chassis is only supported for `class:fighter` level 1; the mutated Rogue chassis does not provide it.
2. `combat.baseline_unsupported` — baseline combat totals are only computed for the exact deterministic Longsword/Chain Shirt/Dodge/no-shield posture and the missing Fighter chassis blocks that proof.
3. `defense.total_save.unsupported` — total saves are only computed from the grounded Fighter level-1 base saves; the mutated chassis does not provide them.
4. `skill.selected_modifier.unsupported` — selected skill modifiers are only computed for the exact deterministic Fighter level-1 Climb/Intimidate/Swim posture with the grounded Chain Shirt armor-check penalty.

## Minimum UI-truth burden for the future workspace shell
A truthful pilot workspace must therefore do all of the following:
1. show the pilot identity and route posture (`Computed` or `Blocked`) from the real receipt path
2. present the current selections explicitly, not just the derived numbers
3. group the real computed values into at least identity, abilities, base chassis, combat, defense, and selected-skills sections
4. preserve explanation affordances by carrying or linking the real explanation ids
5. keep diagnostics visible; the blocked route must show real diagnostics instead of a faux clean state
6. avoid relabeling the computed route as oracle-checked parity, because the current computed path still classifies to `OracleGap`

## Why this receipt matters
This receipt answers the one question the shell must not improvise:

What exact pilot truth must exist on screen before a workspace shell is allowed to look successful?

The answer is now grounded by execution, not guesswork.

## Completion rule
This receipt is complete because it records executable proof over the real deterministic pilot fixture, captures both the computed and blocked route examples, and turns the abstract GE07-E3 acceptance language into a concrete UI-truth burden for later readiness and coding work.
