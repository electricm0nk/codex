# Oracle parity report: pf1-crb-human-wizard-level1

## Summary

- Matches: 13
- Mismatches: 1
- Result: FAIL

## Per-Dimension Comparison

| Dimension | PCGen | Codex | Match | Notes |
|---|---|---|---|---|
| character.identity | pf1-crb-human-wizard-level1 | pf1-crb-human-wizard-level1 | yes | — |
| defense.baseline_armor_class | 17 | 17 | yes | — |
| defense.total_save.fortitude | 2 | 2 | yes | — |
| defense.total_save.reflex | 2 | 2 | yes | — |
| defense.total_save.will | 3 | 3 | yes | — |
| skill.selected_modifier.climb | 3 | 3 | yes | — |
| skill.selected_modifier.intimidate | 0 | 0 | yes | — |
| skill.selected_modifier.swim | 3 | 3 | yes | — |
| encumbrance.carrying_capacity.light_max_lbs | 100 | 100 | yes | — |
| encumbrance.carrying_capacity.medium_max_lbs | 200 | 200 | yes | — |
| encumbrance.carrying_capacity.heavy_max_lbs | 300 | 300 | yes | — |
| encumbrance.total_carried_weight_lbs | 29 | 29 | yes | — |
| durability.max_hp | 8 | 8 | yes | — |
| combat.baseline_melee_attack_bonus | 4 | 5 | no | — |

## Normalization Rules Used

- trailing-whitespace-strip (per `normalization.rs`)
- integer-coercion (per `normalization.rs`)

## Discovered Deltas

- `combat.baseline_melee_attack_bonus` — PCGen: 4, Codex: 5 (value mismatch)
