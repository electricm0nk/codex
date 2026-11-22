# SD-36 Epic F2b — prestige class alone states the game rule

Spec: `epic-f-class-completion.md` §4; acceptance F2.2 (`epic-breakdown.md`).

## Rule

A character whose ONLY class is a prestige class (converted record tagged
`Prestige`; `generic_class_chassis::is_prestige`) gets one claim-blocking
diagnostic:

- id: `prestige_class.requires_base_class_levels`
- text: `A prestige class cannot be a character's first class. Add levels in a base class first.`

No chassis number is emitted for that build (no `class_chassis.base_*`
explanation; the printed `sheet.base_attack_bonus` / `sheet.save.*` cells render
`Blocked`), and `class_chassis.unsupported` no longer fires (the class is known).
One branch at the top of `compute_class_chassis`'s single-class section
(`src/rules_core/pilot_compute/class_occult_and_psionic.rs`), keyed on the tag;
no class is named. The entry-requirement report
(`class_chassis.prestige_entry_gate.{met,unmet}`) still prints beside it.

## Counts

Command: `cargo run --locked -j 8 --bin class_census -- --json <path>`
(snapshot: `census-f2b.json` beside this directory), stdout:

```
ids=137 computed=63 blocked=0
prestige_swept=74 prestige_alone_blocked=74 prestige_mix_computed=0 prestige_mix_unknown=11
mix_panel_swept=185 mix_panel_computed=185 mix_panel_blocked=0
```

Read from `prestige[].alone_blocking_diagnostics`:

| Figure | Value | Denominator |
|---|---|---|
| Prestige ids Blocked alone (`prestige_alone_blocked`) | 74 | of 74 prestige ids |
| ...carrying `prestige_class.requires_base_class_levels` | 74 | of 74 |
| ...carrying it at EVERY level `1..=max_level` | 74 | of 74 |
| ...carrying `class_chassis.unsupported` | 0 | of 74 |
| ...carrying `class_chassis.prestige_entry_gate.unmet` (fixture has no prestige feats) | 74 | of 74 |
| ...carrying `combat.baseline_unsupported` / `defense.total_save.unsupported` / `skill.selected_modifier.unsupported` | 74 each | of 74 |

Unchanged against `census-f2a.json` (field-by-field equality, python
`json.load` compare): `classes` (63 non-prestige rows), `mix_panel` (185 rows),
every top-level count, and every prestige row's `mixes`. Before F2b the same
74 rows carried `class_chassis.prestige_entry_gate.unmet` + the three
downstream ids and NO rule id — and each emitted a converted chassis row alone
(a half sheet).

## RED → GREEN

`cargo test --locked -j 8 --lib prestige_alone -- --test-threads=8`

- RED (before the branch): `a_prestige_class_alone_states_the_game_rule` —
  148 failures = 74 of 74 missing the rule id + 74 of 74 emitting
  `class_chassis.base_attack_bonus` (half sheet).
- GREEN: 4 passed — `a_prestige_class_alone_states_the_game_rule` (74 of 74 at
  level 1, denominator asserted), `a_prestige_class_alone_prints_no_chassis_cell`
  (Arcane Archer, War Mind: BAB and three save cells `Blocked`),
  `a_base_class_alone_never_gets_the_prestige_alone_rule` (Fighter, Kineticist,
  Ninja, Ex-Antipaladin), and the census's
  `prestige_alone_is_blocked_with_the_game_rule` (every level of all 74;
  previously `#[ignore = "RED until Epic F2"]` with a placeholder id).

Flipped: `untabled_base_class_features::a_prestige_class_id_still_fails_the_gate`
(same name and meaning, now asserts the rule id and no `class_chassis.unsupported`);
`prestige_class_features` — the three CRB feature tests assert no chassis
explanation alone (features still ground), and the Arcane Archer dispatch test
became `prestige_class_alone_dispatch_states_the_game_rule_and_the_shared_gate_refuses_it`.

Desktop `character_hub::create_character_at_root_grants_no_wealth_when_the_build_is_blocked`:
its roster members Samurai and Magus were already Computed (F1/F1c) and the
guard already used a prestige-alone Eldritch Knight; it now also asserts the
Blocked diagnostics carry the rule id. Non-vacuity: with the rule's
`diagnostics.push` disabled, that test FAILS (Eldritch Knight Blocked only by
`class_chassis.unsupported` et al.); restored, it passes.
