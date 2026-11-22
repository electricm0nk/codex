---
title: GE06-E2-F2c Merge Receipt
artifact_type: implementation-merge-receipt
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_handoff: ./ge06-e2-f2c-execution-handoff-2026-06-21.md
selected_slice: GE06-E2-F2c — Total Fortitude, Reflex, and Will saving throws under deterministic ability scores
workflow_route: coding
status: merged
merge_date: 2026-06-21
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE06-E2-F2c Merge Receipt

## Verdict
GE06-E2-F2c is complete and merged into `develop`.

## Verified repository state
Observed after `git fetch origin --prune` and detached verification of `origin/develop`:

```text
repo: /home/ubuntu/workspace/repos/codex
origin/develop: 1b44c07
merge: Merge pull request #10 from electricm0nk/ge06-e2-f2c-total-saving-throws
feature branch: deleted upstream
previous develop anchor: 75c26ce
```

## Landed files

```text
src/rules_core/pilot_compute.rs
tests/ge06_pilot_total_saves.rs
```

## Verified behavior
The merged slice establishes:

- GE-06 deterministic total Fortitude save computes as `4`
- GE-06 deterministic total Reflex save computes as `2`
- GE-06 deterministic total Will save computes as `1`
- explanation ids exist for `defense.total_save.fortitude`, `defense.total_save.reflex`, and `defense.total_save.will`
- explanation detail cites the grounded Fighter base save, the relevant ability modifier, and the final total for each bounded save
- unsupported chassis such as mutating the fixture from Fighter 1 to Rogue 1 yields a claim-blocking diagnostic and withholds total-save explanations instead of fabricating values
- `src/rules_core/pilot_compute.rs` prose now truthfully describes the post-F2b/post-F2c supported outputs rather than falsely claiming armor class and attack bonus are unsupported
- the slice remains bounded to deterministic total saves only and does not claim feat-, item-, or condition-based save modifiers, damage, initiative, skills, parity, or UI truth

## Verification command

```bash
"$HOME/.cargo/bin/cargo" test --quiet
```

Observed result: pass in a detached worktree created from `origin/develop`.

## Remaining boundary
This merge advances GE-06 to deterministic ability modifiers, Fighter base chassis, baseline combat totals, and deterministic total saving throws only:

```text
selected pilot input contract: represented
base ability modifiers and Fighter class chassis: computed
baseline melee attack bonus / armor class: computed
total Fortitude / Reflex / Will saves: computed
selected skill modifiers and armor-check effects: not yet computed
oracle parity: not checked
UI truth: not product-visible
```

## Next truthful move
Derive a new bounded handoff for selected deterministic skill modifiers under the accepted rank allocation and Chain Shirt posture only:

- Climb `5` = rank `1` + STR modifier `+3` + class-skill bonus `+3` + armor-check penalty `-2`
- Intimidate `3` = rank `1` + CHA modifier `-1` + class-skill bonus `+3`
- Swim `5` = rank `1` + STR modifier `+3` + class-skill bonus `+3` + armor-check penalty `-2`

Keep the slice narrow: compute and explain only the selected skill totals plus the equipment-effect contribution of Chain Shirt armor-check penalty where applicable, excluding parity, UI, and broader skill-system expansion.
