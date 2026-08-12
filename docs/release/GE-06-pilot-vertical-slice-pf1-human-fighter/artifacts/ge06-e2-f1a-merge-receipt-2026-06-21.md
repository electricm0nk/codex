---
title: GE06-E2-F1a Merge Receipt
artifact_type: implementation-merge-receipt
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_handoff: ./ge06-e2-f1a-execution-handoff-2026-06-21.md
selected_slice: GE06-E2-F1a — Deterministic pilot input contract fixture load gate
workflow_route: coding
status: merged
merge_date: 2026-06-21
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE06-E2-F1a Merge Receipt

## Verdict
GE06-E2-F1a is complete and merged into `develop`.

## Verified repository state

Observed after `git fetch origin --prune`, `git pull --ff-only origin develop`, and baseline test execution:

```text
repo: /home/ubuntu/workspace/repos/codex
branch: develop
HEAD: 9f3cb93
origin/develop: 9f3cb93
merge: Merge pull request #7 from electricm0nk/ge06-e2-f1a-deterministic-pilot-input-contract
feature branch: origin/ge06-e2-f1a-deterministic-pilot-input-contract deleted upstream
```

## Landed files

```text
src/rules_core/character_input.rs
tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
tests/ge06_pilot_input_contract.rs
```

## Verified behavior

The merged slice establishes:

- GE-06 deterministic pilot input fixture exists in repo tests
- loader supports optional `case_id`
- loader supports active-state distinctions for equipped/active, absent, and selected-inactive
- test asserts GE-06 ability scores including CON 14
- test asserts Power Attack, Dodge, Weapon Focus
- test asserts Climb 1, Intimidate 1, Swim 1
- test asserts Chain Shirt, Longsword, no shield, and inactive Power Attack baseline state
- provenance points at the GE-06 final deterministic input contract

## Verification command

```bash
cargo test --quiet
```

Observed result: pass.

## Remaining boundary

This merge does not compute Pathfinder values. It advances the claim tier only to:

```text
selected pilot input contract: represented
derived Pathfinder outputs: not yet computed
oracle parity: not checked
UI truth: not product-visible
```

## Next truthful move

Derive a new bounded handoff for the first computation foothold: base ability modifiers plus Fighter level-1 class chassis values with explanation records, excluding equipment, attacks, skills, parity, and UI.
