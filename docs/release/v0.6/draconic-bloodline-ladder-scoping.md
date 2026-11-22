# Sorcerer Draconic Bloodline (#60) — Full Power-Ladder Scoping

> Dedicated workstream, not a quick canonical pick — bloodlines are
> single-class, chooser-in-chooser ladders (risks item 77).
>
> **The ladder is 7 real records. Two are groundable now, two more are
> Bomb-shaped, and three are correctly blocked. The anchor — Dragon
> Resistances — carries a genuine two-line stacking structure that a partial
> read gets wrong at level 15.**

## The full record set, verified fresh

`KEY:Draconic Bloodline ~ *` returns seven real records plus one plumbing
record (`~ Standard`, carrying `if(SorcererLVL>=1,0,-1)` enable gates).

| power | gate | token | verdict |
|---|---|---|---|
| Bloodline Arcana | 1 | none | **no magnitude** — a spell-damage modifier rule |
| Bloodline Powers | 1 | none | umbrella + **dragon-type chooser** (see below) |
| Claws | 1 | `Sorcerer_GenericClaws_SizeBonus = 1` | **blocked** — needs a natural-attack routine |
| **Dragon Resistances** | **3** | AC + energy resistance | **groundable — the anchor** |
| Breath Weapon | 9 | dice / DC / uses | groundable magnitudes, effect deferred |
| Wings | 15 | `Maneuverability = 3` | flat, but no computed total |
| Power of Wyrms | 20 | `BlindsenseRange = 60` | flat, but no computed total |

Gate levels are confirmed by the variable names themselves —
`Sorcerer_Draconic_BloodlinePower{1,3,9,15,20}LVL`, each set to
`Sorcerer_Draconic_BloodlineLVL + BloodlinePowerNLVLBonus`.

## The anchor: Dragon Resistances — and its stacking hazard

```
BONUS:COMBAT|AC|…_NaturalArmorBonus|TYPE=NaturalArmor.STACK
BONUS:VAR|…_ResistanceBonus  |min(floor((L-3)/6)+1,2)*5
BONUS:VAR|…_NaturalArmorBonus|min(floor((L-3)/6)+1,3)
BONUS:VAR|…_NaturalArmorBonus|1|PREVARGTEQ:…BloodlinePower3LVL,15     ← second line
```

Worked through level by level:

| level | resistance | natural armor |
|---|---|---|
| 3 | `min(1,2)*5` = **5** | `min(1,3)` = **1** |
| 9 | `min(2,2)*5` = **10** | `min(2,3)` = **2** |
| 15+ | 10 (capped) | `min(3,3)` **+ 1** = **4** |

**The natural-armor progression is 1 → 2 → 4, and it needs both lines to
produce it.** Reading only the first yields **3** at level 15. This is the
Sacred Weapon / Brawler Maneuver Training shape again — except here the
corpus is *correct* and the naive read is wrong, since the `min(…,3)` cap
plus a separate `+1` is precisely how the jump from 2 to 4 is expressed.

**`BONUS:COMBAT|AC|…|TYPE=NaturalArmor.STACK` lands on computed AC** — this
is the fact that makes the whole ladder worth building.

## A third nesting level the brief didn't anticipate

The energy type is **not** fixed by the bloodline. Dragon Resistances
branches on `Sorcerer_DraconicDamageType_{Acid, Cold, Electricity, Fire}`,
which is set by the **dragon-type choice** (chromatic/metallic — 10 dragon
types) made under Bloodline Powers.

So the nesting is **Sorcerer → Draconic bloodline → dragon type → energy
type**: three levels of chooser, one deeper than any prior narrowing this
session.

**Useful consequence:** the *magnitudes* are type-agnostic — resist 5/10 and
natural armor 1/2/4 regardless of dragon. Only the **energy label** needs the
dragon-type choice. So the ladder can be grounded with the numbers correct
and the energy type named as a further deferred choice, or a canonical dragon
picked (Red → fire is the conventional default) to name it concretely.

## Groundable now

1. **Dragon Resistances** — natural armor into computed AC, plus the energy
   resistance magnitude. The anchor.
2. **Wings** (`Maneuverability = 3`, fly 60 ft) and **Power of Wyrms**
   (`BlindsenseRange = 60`) — flat and self-scoped, but no computed total, so
   they ground standalone. Both are level 15/20, so inert across the level
   band tests exercise.
3. **Breath Weapon** — `Dice = BloodlinePower9LVL`,
   `DC = 10 + (L/2) + SpellStat`, `Times = max(floor((L-11)/3)…)`. Exactly
   the Bomb split already precedented: **ground the dice, DC and uses; defer
   the breath effect.**

## Correctly blocked

- **Claws** — grants two claw attacks; needs a natural-attack routine
  (attack rolls, iterative attacks, damage-by-size) this engine does not
  model. Same category as the Eidolon's own attack surface. The
  `GenericClaws_SizeBonus` var alone is a size step, not a usable magnitude.
- **Bloodline Arcana** — "whenever you cast a spell with the [energy]
  descriptor, it deals +1 damage per die." No token, and it modifies spell
  damage, which is not computed. Genuine no-op in the Nature Training sense.
- **Bloodline Powers** — umbrella record; its content is the dragon-type
  chooser.

## Recommended scope

**Slice: Dragon Resistances alone**, grounding the natural-armor bonus into
computed AC and the energy-resistance magnitude, with the energy type left
as a named deferred choice (or Red/fire as canonical if you want it
concrete). That is the one power in the ladder with a computed consumer, and
it carries the stacking subtlety worth getting right once.

**Optional add:** Breath Weapon's three magnitudes, on the Bomb precedent —
cheap, but level-9-gated and effect-deferred, so lower value.

**Honest expectation:** this grounds 1-2 of Draconic's 7 powers and leaves
the other 20 bloodlines entirely deferred. It is a genuine slice of the
largest remaining seam, not a closure of it.

## Open question

**Name the energy type or defer it?** Grounding "resist 5" without saying
*to what* is accurate but thin; picking Red/fire as canonical makes it
concrete at the cost of a third-level canonical choice. My lean is to ground
the numbers and defer the type — the type-agnostic magnitudes are the honest
common factor, and it avoids a canonical-within-canonical-within-canonical
pick — but this is exactly the kind of call worth your ruling rather than my
assumption.
