---
title: GE06-E2-F2b Merge Receipt
artifact_type: implementation-merge-receipt
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_handoff: ./ge06-e2-f2b-execution-handoff-2026-06-21.md
selected_slice: GE06-E2-F2b — Baseline melee attack bonus and armor class under deterministic loadout
workflow_route: coding
status: merged
merge_date: 2026-06-21
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE06-E2-F2b Merge Receipt

## Verdict
GE06-E2-F2b is complete and merged into `develop`.

## Verified repository state
Observed after `git fetch origin --prune` and detached verification of `origin/develop`:

```text
repo: /home/ubuntu/workspace/repos/codex
local_branch: ge06-e2-f2b-baseline-combat-values
local_HEAD: a93269a
origin/develop: 75c26ce
merge: Merge pull request #9 from electricm0nk/ge06-e2-f2b-baseline-combat-values
feature branch: deleted upstream
```

## Landed files

```text
src/rules_core/pilot_compute.rs
tests/ge06_pilot_combat_baseline.rs
```

## Verified behavior
The merged slice establishes:

- GE-06 deterministic baseline melee attack bonus computes as `+5`
- GE-06 deterministic baseline armor class computes as `17`
- explanation ids exist for `combat.baseline_melee_attack_bonus` and `defense.baseline_armor_class`
- explanation detail cites Fighter BAB, Strength modifier, Weapon Focus (Longsword), Chain Shirt armor bonus, Dexterity contribution with `MAXDEX:4`, Dodge, absent shield posture, and inactive Power Attack posture
- an unsupported posture such as equipping the shield yields a claim-blocking diagnostic instead of fabricated combat totals
- the slice remains bounded to baseline combat totals only and does not claim damage, active Power Attack math, initiative, skills, parity, or UI truth

## Verification command

```bash
"$HOME/.cargo/bin/cargo" test --quiet
```

Observed result: pass in a detached worktree created from `origin/develop`.

## Remaining boundary
This merge advances GE-06 to the first deterministic combat totals only:

```text
selected pilot input contract: represented
base ability modifiers and Fighter class chassis: computed
baseline melee attack bonus / armor class: computed
total Fortitude / Reflex / Will saves: not yet computed
oracle parity: not checked
UI truth: not product-visible
```

## Next truthful move
Derive a new bounded handoff for total Fortitude, Reflex, and Will saving throws under the deterministic ability scores only: base Fortitude/Reflex/Will plus Constitution/Dexterity/Wisdom modifiers, excluding feat-, item-, condition-, parity-, and UI-scope expansion.
