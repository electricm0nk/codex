# F3c -- prestige carrier-mix remainder: 7 of 74

Source: `artifacts/epic-f/census-f3c.json`, from
`cargo run --locked -j 8 --bin class_census -- --json <path>` on `sd36/epic-f2-f3`, 2026-09-24.
Denominator: 74 prestige ids of 137 census ids. Result: `prestige_mix_computed=67`,
`prestige_mix_unknown=1`, Blocked 6. A remainder here is a number, never an exemption, and each row
below is named by its mechanism.

| mechanism | count of 74 | classes | what closes it |
|---|---:|---|---|
| Oracle `BONUS:SAVE` formula defect: the class line's save formula matches no PF1 save form. The mix refuses with `multiclass.save_shape.unrecognized`, never a silent poor save. | 6 | evangelist, exalted, mammoth_rider, pure_legion_enforcer, sentinel, ulfen_guard | A data correction checked against the book's table, or the converter carrying the oracle's own `ClassSave{Good,Poor}_<Save>` declaration (not in the converted records today). Both are outside an engine batch (`data/sheet_rules/**` is frozen here). |
| Caster clause with no translatable branch: the carrier stays Unknown and `carrier_unknown_reason` names the clause | 1 | dragon_disciple | A carrier set wider than wizard/cleric/fighter (a sorcerer carrier with a recorded bloodline choice), or a spontaneous-caster axis in the translator |

## The save-formula rows

The first four are F3b3 §3, pinned by
`multiclass_fold::tests::the_four_unrecognized_prestige_saves_are_oracle_formula_defects_not_a_missed_shape`.
Evangelist and Pure Legion Enforcer reach the save gate for the first time in F3c, because the chooser
now names a carrier for them. They are pinned by
`multiclass_fold::tests::two_more_prestige_saves_the_f3c_carriers_reach_are_the_same_oracle_formula_defect`:

- **Pure Legion Enforcer** (Inner Sea Combat). The formulas are `classlevel()+3/2`,
  `classlevel()+1/3` and `classlevel()+3/2`. That is level + 3/2, the same shape as Ulfen Guard, giving
  +11/+10/+11 at 10th. Every slot is Unrecognized.
- **Evangelist** (Inner Sea Gods). The Reflex formula is `classlevel()/3+1`, under the class's own
  `ClassSaveGood_Reflex` declaration. That gives +4 at 10th, which matches none of PF1's save forms at
  10th (base good 7, base poor 3, prestige good 5, prestige poor 3). Fortitude and Will are `L/3`,
  which classifies as Poor.

Printing either class's number would put a save on the sheet that the book does not state. They stay
Blocked.

## dragon_disciple

Its converted gate states the caster requirement as
`AtLeast{1, [Holds eldritch_scion_spells, AtLeast{3, [Situational "requires a spontaneous caster", HighestSpellLevel(Arcane) >= 1, Not(ClassLevel(sorcerer) >= 1)]}, AtLeast{2, [ClassLevel(sorcerer) >= 1, Holds sorcerer_bloodline_draconic]}]}`.
None of the three branches translates:

- **Branch 1** is a class feature.
- **Branch 2** needs all three of its terms, and one is a situational spontaneous-caster clause.
- **Branch 3** needs a sorcerer level and a bloodline choice.

Naming the wizard here would repeat the F0-check finding 1 defect, where the wizard is a prepared caster
that the gate never asks for.
