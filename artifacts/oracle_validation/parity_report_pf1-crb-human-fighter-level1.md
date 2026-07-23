# Oracle parity report: pf1-crb-human-fighter-level1

## Summary

- Matches: 8
- Mismatches: 1
- Result: FAIL

## Per-Dimension Comparison

| Dimension | PCGen | Codex | Match | Notes |
|---|---|---|---|---|
| character.identity | pf1-crb-human-fighter-level1 | pf1-crb-human-fighter-level1 | yes | — |
| defense.baseline_armor_class | 17 | 17 | yes | — |
| defense.total_save.fortitude | 4 | 4 | yes | — |
| defense.total_save.reflex | 2 | 2 | yes | — |
| defense.total_save.will | 1 | 1 | yes | — |
| skill.selected_modifier.climb | 6 | 6 | yes | — |
| skill.selected_modifier.intimidate | 3 | 3 | yes | — |
| skill.selected_modifier.swim | 6 | 6 | yes | — |
| combat.baseline_melee_attack_bonus | 5 | 6 | no | — |

## Normalization Rules Used

- trailing-whitespace-strip (per `normalization.rs`)
- integer-coercion (per `normalization.rs`)

## Discovered Deltas

- `combat.baseline_melee_attack_bonus` — PCGen: 5, Codex: 6 (value mismatch)
