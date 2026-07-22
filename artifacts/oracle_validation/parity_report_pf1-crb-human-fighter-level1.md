# Oracle parity report: pf1-crb-human-fighter-level1

## Summary

- Matches: 0
- Mismatches: 9
- Result: FAIL

## Per-Dimension Comparison

| Dimension | PCGen | Codex | Match | Notes |
|---|---|---|---|---|
| character.identity | Florian Syrkov | pf1-crb-human-fighter-level1 | no | — |
| combat.baseline_melee_attack_bonus | 10 | 5 | no | — |
| defense.baseline_armor_class | 22 | 17 | no | — |
| defense.total_save.fortitude | 9 | 4 | no | — |
| defense.total_save.reflex | 5 | 2 | no | — |
| defense.total_save.will | 8 | 1 | no | — |
| skill.selected_modifier.climb | -1 | 5 | no | — |
| skill.selected_modifier.intimidate | 2 | 3 | no | — |
| skill.selected_modifier.swim | -1 | 5 | no | — |

## Normalization Rules Used

- trailing-whitespace-strip (per `normalization.rs`)
- integer-coercion (per `normalization.rs`)

## Discovered Deltas

- `character.identity` — PCGen: Florian Syrkov, Codex: pf1-crb-human-fighter-level1 (value mismatch)
- `combat.baseline_melee_attack_bonus` — PCGen: 10, Codex: 5 (value mismatch)
- `defense.baseline_armor_class` — PCGen: 22, Codex: 17 (value mismatch)
- `defense.total_save.fortitude` — PCGen: 9, Codex: 4 (value mismatch)
- `defense.total_save.reflex` — PCGen: 5, Codex: 2 (value mismatch)
- `defense.total_save.will` — PCGen: 8, Codex: 1 (value mismatch)
- `skill.selected_modifier.climb` — PCGen: -1, Codex: 5 (value mismatch)
- `skill.selected_modifier.intimidate` — PCGen: 2, Codex: 3 (value mismatch)
- `skill.selected_modifier.swim` — PCGen: -1, Codex: 5 (value mismatch)
