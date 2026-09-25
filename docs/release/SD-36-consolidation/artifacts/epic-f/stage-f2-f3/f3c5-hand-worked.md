# F3c5: hand-worked PF1 values for Sorcerer 5 / Dragon Disciple 3 (oracle first)

Written BEFORE the new oracle row in `tests/sd36_multiclass_any_class.rs` first ran. If the engine
and this sheet disagree, the engine is investigated. The expected values here are not edited to
match it.

## Character

This uses the same fixture and builder as `f3b-hand-worked.md`:

- The fixture is `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`.
- `class_levels` is replaced with Sorcerer 5, then Dragon Disciple 3. Sorcerer is the class taken
  at character level 1.
- The canonical seeds for each class are merged in.

Ability modifiers: Str +4, Dex +2, Con +2, Int +0, Wis +1, Cha -1. The character has no Toughness
and no favored-class bonus.

## Class rows used

| Class | Source (record SOURCEPAGE) | Hit die | BAB | Fort / Ref / Will | Skill ranks / level |
|---|---|---|---|---|---:|
| Sorcerer | CRB p.70 | d6 | 1/2 | poor / poor / good | 2 |
| Dragon Disciple (prestige) | CRB p.380 | d12 | 3/4 | good / poor / good (prestige forms) | 2 |

The rules are the same as in `f3b-hand-worked.md`, with the same citations:

- BAB is summed across classes.
- Base saves use the fractional rule, floored once. A base class's good save is `level/2 + 2` and
  its poor save is `level/3`. A prestige class's good save is `(level+1)/2` and its poor save is
  `(level+1)/3`.
- HP is the maximum die at character level 1, then `die/2 + 1` per level after that, plus the Con
  modifier at every level.

**Dragon Disciple features at 3rd level** (CRB p.380-382):

- Natural armor +1 at 1st level. This affects AC, not saves or HP.
- Blood of Dragons.
- The first ability boost is Str +2 at 2nd level. It does not move HP, saves or skill points.
- Dragon Bite at 2nd level. This is a natural attack, not a proficiency.
- Nothing in these features adds to a save or to HP.

**Weapon proficiency:** CRB p.380 says "Dragon disciples gain no proficiency with any weapon or
armor." Sorcerer grants simple weapons (CRB p.71). So the union is Sorcerer's grant, and Dragon
Disciple adds nothing to it.

## Sorcerer 5 / Dragon Disciple 3

| Figure | Working | Value |
|---|---|---:|
| BAB | floor(5/2) = 2; floor(3 x 3/4) = 2 | **4** |
| Fort (base) | 5/3 = 1.667; (3+1)/2 = 2.0; sum 3.667, floored | **3** (CRB core rounding: 1 + 2 = 3) |
| Ref (base) | 5/3 = 1.667; (3+1)/3 = 1.333; sum 3.0 | **3** (CRB core rounding: 1 + 1 = 2; the adopted fractional rule gives 3) |
| Will (base) | 5/2 + 2 = 4.5; (3+1)/2 = 2.0; sum 6.5, floored | **6** (CRB core rounding: 4 + 2 = 6) |
| Fort / Ref / Will total | 3+2 / 3+2 / 6+1 | **5 / 5 / 7** |
| HP | Sorcerer: 6+2 = 8, then 4 x (4+2) = 24, for 32; Dragon Disciple: 3 x (7+2) = 27 | **59** |
| Skill points (class) | 5 x (2+0) + 3 x (2+0) | **16** |
