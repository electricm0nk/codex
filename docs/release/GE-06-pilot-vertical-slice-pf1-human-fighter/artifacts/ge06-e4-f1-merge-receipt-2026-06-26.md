---
title: GE06-E4-F1 Merge Receipt
artifact_type: implementation-merge-receipt
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_handoff: ./ge06-e4-f1-execution-handoff-2026-06-22.md
selected_slice: GE06-E4-F1 — Pilot view-model contract from real outputs
workflow_route: coding
status: merged
merge_date: 2026-06-26
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE06-E4-F1 Merge Receipt

## Verdict
GE06-E4-F1 is complete and merged into `develop`.

## Verified repository state
Observed after `git fetch origin --prune`, detached verification of current `origin/develop` ancestry, and GitHub PR inspection:

```text
repo: /home/ubuntu/workspace/repos/codex
current origin/develop: cc4e1a55caad07af83768a036d4b0f5fffbf99c9
verified merge commit: a11f7a4d016fd58324518bb07de8edbbf486ad0c
merge: Merge pull request #19 from electricm0nk/ge06-e4-f1-pilot-view-model-contract
feature branch on origin: not present after merge
previous develop anchor: d7d2604f4d4361a94e059522f19b9aef97836a08
implementation commit: 1840cd93321f1c7860f26df0e775d76a55571d76
github pr: https://github.com/electricm0nk/codex/pull/19
```

## Landed files

```text
src/rules_core/mod.rs
src/rules_core/pilot_view_model.rs
tests/ge06_pilot_view_model.rs
```

Diff footprint observed from `d7d2604..a11f7a4`:

```text
3 files changed, 224 insertions(+)
```

## Verified behavior
The merged slice establishes:

- a bounded `PilotViewModel` projection from the real GE-06 headless receipt into a machine-checkable UI-consumer boundary
- preserved pilot identity (`case_id`, `source_package_id`), preserved status, and preserved primary failure owner from the failure-classifier lane
- a real computed snapshot when the receipt status is `Computed`, including deterministic ability modifiers, base attack bonus, base saves, baseline melee attack bonus, baseline armor class, total saves, selected skill modifiers, explanations, and non-claim-blocking diagnostics
- explicit refusal of a faux success snapshot when the receipt status is `Blocked`; the blocked view-model keeps real claim-blocking diagnostics instead of fabricating success values
- module export wiring through `src/rules_core/mod.rs` so the merged adapter is reachable as part of the rules-core surface

## Verification commands

```bash
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_view_model --quiet
"$HOME/.cargo/bin/cargo" test --quiet
```

Observed result: targeted GE06 pilot view-model test passed (`2 passed`), and the full test suite passed on current repo state after the merge was present on `origin/develop`.

## Remaining boundary
This merge advances GE-06 to a bounded UI-consumer projection over real pilot outputs while preserving the existing documentary and parity boundaries:

```text
selected pilot input contract: represented
base ability modifiers and Fighter class chassis: computed
baseline melee attack bonus / armor class: computed
total Fortitude / Reflex / Will saves: computed
selected deterministic Climb / Intimidate / Swim modifiers: computed
integrated headless receipt path: computed
selected parity-dimension adapter: computed
failure classifier and owner mapping: computed
pilot view-model contract from real outputs: merged
oracle parity: not checked
product-visible desktop shell: merged separately in GE-07, but GE-06 still remains computed-but-not-oracle-checked rather than pilot-viable
```

## Next truthful move
Retire the root GE-06 route surface from `awaiting-todd-launch` to `no-active-handoff`, preserve GE06-E4-F1 as merged historical authority, and rerun the downstream documentary route-sync so README, spec-domain, and ledger surfaces no longer pretend the lane is still awaiting launch.
