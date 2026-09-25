# F3c -- prestige carrier-mix blocker histogram (before F3c)

Command: `cargo run --locked -j 8 --bin class_census -- --json <scratch>/census-f3c-0.json` on
`sd36/epic-f2-f3` at `f7f1488ecf` (F3b3). Denominator: the 74 prestige ids of 137 census ids.
Headline: `prestige_mix_computed=59`, `prestige_mix_unknown=11`, Blocked 4.

A row is counted once per id (a class with two carrier mixes counts once). A Blocked row counts under
every claim-blocking diagnostic id any of its mixes carries, so the Blocked ids overlap (4 classes,
5 ids each).

| blocking id / state | count of 74 | examples (up to 3) | mechanism |
|---|---:|---|---|
| Unknown: no nameable carrier (`carrier_unknown_reason`) | 11 | dark_tempest, harrower, pure_legion_enforcer | carrier chooser reads only mandatory top-level Arcane/Divine terms; see split below |
| `multiclass.save_shape.unrecognized` | 4 | exalted, mammoth_rider, sentinel | oracle-data operator-precedence defect in the class line's `BONUS:SAVE` formula (F3b3 §3) |
| `class_chassis.unsupported` | 4 | exalted, mammoth_rider, sentinel | same 4 rows, consequence of the save-shape refusal |
| `combat.baseline_unsupported` | 4 | exalted, mammoth_rider, sentinel | same 4 rows |
| `defense.total_save.unsupported` | 4 | exalted, mammoth_rider, sentinel | same 4 rows |
| `skill.selected_modifier.unsupported` | 4 | exalted, mammoth_rider, sentinel | same 4 rows |
| Computed | 59 | -- | -- |

The four Blocked rows are one mechanism: the gate refuses the class (`multiclass.save_shape.unrecognized`),
and the other four ids follow from that refusal. The fourth class in every row is ulfen_guard.

## Unknown: 11 of 74, by the shape of the caster term

| shape | count | classes |
|---|---:|---|
| mandatory top-level `HighestSpellLevel(Any) >= n` (no Arcane/Divine carrier rule for `Any`) | 6 | dark_tempest, elocater, hellknight_signifer, psion_uncarnate, storm_kindler, thrallherd |
| caster term only inside an `AtLeast` | 4 | dragon_disciple, evangelist, harrower, pathfinder_savant |
| caster term only inside a `Not` (a prohibition) | 1 | pure_legion_enforcer |

These 11 rows never reach the engine: `mixes` is empty, so no blocking id is measured for them yet.

## Movement

One fix, in the census carrier chooser (`src/rules_core/class_census.rs` `choose_carriers` /
`translate_clause`), applied to every prestige gate with no class named:

- An `AtLeast { n, of }` clause takes the first `n` branches, in oracle order, that translate to a carrier
  + level.
- `HighestSpellLevel(Any)` is read the same way, as "at least 1 of [Arcane, Divine]". The result is a
  wizard, or a cleric when the gate forbids arcane casting.
- A `Not` of a spell-kind term is a prohibition, and the fighter floor meets it.
- The same translation feeds the entry-gate level solve, so a branch's numeric terms count toward the
  carrier level.

| run | command | prestige_mix_computed | Unknown | Blocked |
|---|---|---:|---:|---:|
| before (`census-f3c-0`) | `cargo run --locked -j 8 --bin class_census -- --json <scratch>/census-f3c-0.json` | 59 of 74 | 11 | 4 |
| after (`census-f3c`, committed) | same, `--json <scratch>/census-f3c-1.json`, copied to `artifacts/epic-f/census-f3c.json` | **67 of 74** | 1 | 6 |

Of the 11 rows that had no carrier before:

- **Now Computed, 8.** Wizard is the carrier for all eight: dark_tempest, elocater,
  hellknight_signifer, psion_uncarnate, storm_kindler and thrallherd (through `Any`), plus harrower and
  pathfinder_savant (branch 1 of their Arcane-or-Divine `AtLeast`).
- **Now carried, still Blocked, 2.** Evangelist gets the fighter floor through its BAB +5 branch, and
  pure_legion_enforcer gets it through the prohibition. Both mixes then stop at
  `multiclass.save_shape.unrecognized`, the same oracle-formula defect as the four already Blocked.
- **Still Unknown, 1: dragon_disciple.** No branch translates.

The 59 already Computed kept their carrier. Seven of them now have a higher or better-described carrier
level, because a skill-rank `AtLeast` joined the level solve:

- argent_dramaturge 5 -> 6;
- body_snatcher 5 -> 10;
- demoniac 5 -> 7;
- master_spy 5 -> 7;
- cerebremancer, pathfinder_chronicler and pathfinder_delver keep level 5, and their entry gate moves
  from unmet to met.

All 59 are still Computed. Mammoth Rider's carrier moved 6 -> 9, and it is still Blocked for the same
reason as before.

After the fix, the Blocked histogram is one mechanism across 6 of 74 rows. Each row carries
`multiclass.save_shape.unrecognized`, `class_chassis.unsupported`, `combat.baseline_unsupported`,
`defense.total_save.unsupported` and `skill.selected_modifier.unsupported`. Examples: evangelist,
exalted and pure_legion_enforcer.
