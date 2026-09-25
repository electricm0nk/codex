# F3b — hand-worked PF1 values for the four multiclass mixes (oracle first)

Written BEFORE `tests/sd36_multiclass_any_class.rs` was run against the engine. A mismatch
between the engine and this sheet is investigated in the engine; the expectation here is not
edited to match.

## Character

The shared deterministic fixture every census sweep uses
(`tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`), with only
`class_levels` replaced by the mix (first-listed class = the class taken at character level 1)
and each class's canonical seeds merged (`class_seeds::canonical_seeds_for`), the same builder
shape as the census's prestige-mix sweep.

| Ability | Score | Modifier | Note |
|---|---:|---:|---|
| Str | 16 + 2 (Human, chosen) = 18 | +4 | CRB p.21 Human +2 to one score |
| Dex | 14 | +2 | |
| Con | 14 | +2 | HP per level |
| Int | 10 | +0 | skill points per level |
| Wis | 12 | +1 | |
| Cha | 8 | -1 | |

No Toughness, no favored-class bonus chosen (the fixture chooses none), so HP and skill points
below are class-only terms (CRB p.30 favored class; p.31 Human skilled +1 rank/level is racial,
not class, and not in the class-only figure).

## Rules applied (with citations)

- **BAB:** CRB p.30 "Multiclassing": add the base attack bonuses of each class. Per-class BAB
  from each class table (full = level; 3/4 = floor(3 x level / 4); 1/2 = floor(level / 2)).
- **Base saves:** the repo's adopted fractional rule (SD-21 E7.29,
  `compute_multiclass_base_chassis`, `docs/architecture/rules-engine.md` "Multiclass base-chassis
  dispatch"): each class's UN-ROUNDED save value is summed and the total is floored ONCE. A base
  class's good save is `level/2 + 2`, poor `level/3` (CRB p.30 Table 3-1 shapes); a prestige
  class's good save is `(level+1)/2`, poor `(level+1)/3` (CRB p.374+ prestige tables: good +1 at
  1st .. +5 at 10th, poor +0 .. +3). The per-class value used is exactly the class's own converted
  `Expr`, evaluated without truncation.
  CRB p.30's core rule (add each class's already-rounded save) is shown beside it; where the two
  differ the row says so. Spec §5 keeps the fractional rule unchanged ("Fractional rule and
  floor-once unchanged"), so the test asserts the fractional column.
- **Total saves:** base + Con (Fort) / Dex (Ref) / Wis (Will); none of these mixes has a class
  feature that adds to a save at these levels (Barbarian Indomitable Will is 14th, Loremaster and
  Arcane Archer tables carry none).
- **HP:** CRB p.30 / p.12: maximum hit die at character level 1 (the first-listed class), the
  non-rolling average `die/2 + 1` at every later level (d6 4, d8 5, d10 6, d12 7;
  `durability::average_hit_die_value`), + Con modifier each level.
- **Skill points (class term):** CRB p.30: each class's skill ranks per level + Int modifier (min 1)
  x its levels.

## Class rows used

| Class | Source (record SOURCEPAGE) | Hit die | BAB | Fort / Ref / Will | Skill ranks / level |
|---|---|---|---|---|---:|
| Barbarian | CRB p.31 | d12 | full | good / poor / poor | 4 |
| Fighter | CRB p.55 | d10 | full | good / poor / poor | 2 |
| Wizard | CRB p.77 | d6 | 1/2 | poor / poor / good | 2 |
| Arcane Archer (prestige) | CRB p.374 | d10 | full | good / good / poor (prestige forms) | 4 |
| Loremaster (prestige) | CRB p.385 | d6 | 1/2 | poor / poor / good (prestige forms) | 4 |
| Magus | Ultimate Magic p.9 | d8 | 3/4 | good / poor / good | 2 |
| Samurai | Ultimate Combat (record p.9) | d10 | full | good / poor / poor | 4 |

Hit dice and progressions re-derived from `data/corpus/<book>/class/<slug>.json` (`HD`,
`SOURCEPAGE`) and the converted `BaseAttack` / `BaseSave` `Expr`s in
`data/sheet_rules/<book>/class/<slug>.json`. Skill ranks per level are the printed book values;
**no record in this repo carries them** (see "Skill points" below).

## 1. Barbarian 12 / Fighter 1

| Figure | Working | Value |
|---|---|---:|
| BAB | 12 + 1 | **13** |
| Fort (base) | 12/2+2 = 8.0 ; 1/2+2 = 2.5 ; 10.5 floor | **10** (CRB core: 8 + 2 = 10) |
| Ref (base) | 12/3 = 4.0 ; 1/3 = 0.333 ; 4.333 floor | **4** (core 4) |
| Will (base) | same as Ref | **4** (core 4) |
| Fort / Ref / Will total | 10+2 / 4+2 / 4+1 | **12 / 6 / 5** |
| HP | Barbarian: 12+2 = 14, then 11 x (7+2) = 99 -> 113; Fighter 1 (not char. level 1): 6+2 = 8 | **121** |
| Skill points (class) | 12 x (4+0) + 1 x (2+0) | **50** |

## 2. Fighter 6 / Arcane Archer 3

| Figure | Working | Value |
|---|---|---:|
| BAB | 6 + 3 | **9** |
| Fort | 6/2+2 = 5.0 ; (3+1)/2 = 2.0 ; 7.0 | **7** (core 5 + 2 = 7) |
| Ref | 6/3 = 2.0 ; (3+1)/2 = 2.0 ; 4.0 | **4** (core 4) |
| Will | 6/3 = 2.0 ; (3+1)/3 = 1.333 ; 3.333 floor | **3** (core 2 + 1 = 3) |
| Totals | 7+2 / 4+2 / 3+1 | **9 / 6 / 4** |
| HP | Fighter: 10+2 = 12, then 5 x (6+2) = 40 -> 52; Arcane Archer: 3 x (6+2) = 24 | **76** |
| Skill points (class) | 6 x 2 + 3 x 4 | **24** |

## 3. Magus 4 / Samurai 2

| Figure | Working | Value |
|---|---|---:|
| BAB | Magus floor(3 x 4 / 4) = 3 ; Samurai 2 | **5** |
| Fort | 4/2+2 = 4.0 ; 2/2+2 = 3.0 ; 7.0 | **7** (core 7) |
| Ref | 4/3 = 1.333 ; 2/3 = 0.667 ; exactly 2 | **2** (**core 1 + 0 = 1: the two rules differ here**) |
| Will | 4/2+2 = 4.0 ; 2/3 = 0.667 ; 4.667 floor | **4** (core 4) |
| Totals | 7+2 / 2+2 / 4+1 | **9 / 4 / 5** |
| HP | Magus: 8+2 = 10, then 3 x (5+2) = 21 -> 31; Samurai: 2 x (6+2) = 16 | **47** |
| Skill points (class) | 4 x 2 + 2 x 4 | **16** |

The Ref sum is exactly 2 (4/3 + 2/3). Floating-point would put it at 1.999... on some operand
orders; the fold must sum exact rationals.

## 4. Wizard 5 / Loremaster 2

| Figure | Working | Value |
|---|---|---:|
| BAB | floor(5/2) = 2 ; floor(2/2) = 1 | **3** |
| Fort | 5/3 = 1.667 ; (2+1)/3 = 1.0 ; 2.667 floor | **2** (core 1 + 1 = 2) |
| Ref | same as Fort | **2** (core 2) |
| Will | 5/2+2 = 4.5 ; (2+1)/2 = 1.5 ; 6.0 | **6** (**core 4 + 1 = 5: the two rules differ here**) |
| Totals | 2+2 / 2+2 / 6+1 | **4 / 4 / 7** |
| HP | Wizard: 6+2 = 8, then 4 x (4+2) = 24 -> 32; Loremaster: 2 x (4+2) = 12 | **44** |
| Skill points (class) | 5 x 2 + 2 x 4 | **18** |

## The two rules that disagree (named, not silently chosen)

Mix 3 Ref (fractional 2, core 1) and mix 4 Will (fractional 6, core 5) are where the repo's
fractional rule and CRB p.30's core rule give different printed numbers. The test asserts the
fractional column because that is the rule the engine documents and the spec keeps unchanged.
Two further notes, recorded for an operator ruling and NOT changed in F3b:

1. The CRB core rule (sum of per-class rounded saves) is PF1's default; the fractional rule is an
   optional variant.
2. The engine's fractional rule adds a base class's good-save `+2` once PER CLASS. The published
   optional rule (Pathfinder Unchained, "Fractional Base Bonuses") grants that `+2` only once per
   save, however many good-save classes the character has. That is recalled from the book, not
   read from a source in this repo. Under it, mix 1 Fort would be 6 + 0.5 + 2 = 8.5 -> 8 and mix 3
   Fort 2 + 1 + 2 = 5. The pre-existing `sd21` / `sd24` pins (Fighter / Wizard, one good save per
   class, so no double +2) do not exercise the difference.

## Skill points: the data cannot support the number

The class-term skill points above (50 / 24 / 16 / 18) are the PF1 values. The engine cannot reach
them: `ClassChassis::skill_ranks_per_level` is `None` for 135 of 135 chassis records (F3a,
`f3a-save-shapes.md`), no converted class file carries a skill-ranks row (189 of 189), and the
corpus carries no `STARTSKILLPTS` token at all (`grep -rl STARTSKILLPTS data/corpus | wc -l` -> 0).
So the fold prints skill points **Unknown** with the named diagnostic
`class_chassis.skill_points.unknown` for every class in every mix, never 0 and never a guessed
value. Reaching the numbers needs an ingest/converter change (carry `STARTSKILLPTS` through to the
converted class record), which touches `data/corpus/**` / `data/sheet_rules/**` and is outside
this batch. The test asserts the Unknown, and keeps these four values beside it as the target that
change must meet.
