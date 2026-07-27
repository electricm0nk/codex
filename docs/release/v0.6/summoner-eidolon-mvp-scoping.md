# Summoner (#17) — Bounded Eidolon MVP Scoping

> The lead ruled the path: **bounded Eidolon MVP, not a full evolution-point
> economy** — consistent with Oracle's Tier-1-only, Bloodrager's
> levels-4-20-only, and especially Hunter's canonical-Wolf companion. This
> doc picks one canonical base form, verifies its real stat-block math, and
> finds the smallest non-fabricated set of evolutions.
>
> **Result: the MVP is genuinely tractable, and better-shaped than expected —
> a canonical Quadruped Eidolon's entire stat block is fully determined with
> ZERO player evolution choices, because every base-form evolution is granted
> `AUTOMATIC`.** But the reuse story is thinner than the Hunter-Wolf framing
> implies, and that needs saying before backend plans around it.

## Recommended canonical base form: Quadruped

Three forms exist (`apg_abilities_companion.lst:61-63`). Verified in full:

| | Biped | **Quadruped** | Serpentine |
|---|---|---|---|
| Ability bonuses | Str +6, Dex +2 | **Str +4, Dex +4** | Str +2, Dex +6 |
| Good saves | Fort, Will | **Fort, Reflex** | Will, Reflex |
| Automatic evolutions | Arms, Clawed Hands, Legs | **Bite, Legs, Legs** | Bite, Climb, Reach (Bite), Tail, Tail Slap |

**Quadruped is the right pick**: it is the closest structural analogue to
the already-grounded Wolf companion (a bite attack, four legs, good
Fort+Reflex), and it carries the fewest automatic evolutions — three, two of
which are the same record.

## The whole stat block is corpus-derivable — every formula verified

- **Eidolon level** = `EidolonCompanionLVL = SummonerLVL`.
- **Class chassis** (`apg_classes.lst:207`): `HD:10`, **full BAB**
  (`classlevel`), saves configurable per base form via
  `EidolonFortGood`/`EidolonReflexGood`/`EidolonWillGood`.
- **Base race** (`apg_races_companion.lst:7`): Size M, `MOVE:Walk,20`,
  `REACH:5`, `BONUS:STAT|CON|2`, `BONUS:STAT|INT|-4`,
  `BONUS:VAR|AC_Natural_Armor|2`, `LEGS:0`, `HANDS:7`.
- **Evolution pool**:
  `3+(SL>=2)+(SL>=3)+if(SL>=4,2,0)+(SL>=5)+…+if(SL>=19,2,0)+(SL>=20)`
  → **3, 4, 5, 7, 8, 9, 10, 11, 13, 14, 15, 16, 17, 19, 20, 21, 22, 23, 25,
  26**. Re-derived level by level; matches the published table exactly.
- **Natural armor bonus**:
  `if(ML>=2,2,0)+if(ML>=5,2,0)+if(ML>=7,2,0)+if(ML>=10,2,0)+if(ML>=12,2,0)+
  if(ML>=15,2,0)+if(ML>=17,2,0)+if(ML>=20,2,0)`
  → **+0, +2 (L2), +4 (L5), +6 (L7), +8 (L10), +10 (L12), +12 (L15),
  +14 (L17), +16 (L20)**, on top of the racial `AC_Natural_Armor 2`.
- **Max natural attacks**: `3+(ML>=4)+(ML>=9)+(ML>=14)+(ML>=19)` →
  **3 / 4 / 5 / 6 / 7**.
- **Skill points**: `EidolonSkillPoints = 6+INT` per HD.
- **Speed**: base `Walk 20` + `BONUS:MOVEADD|TYPE.Walk|10` per `Legs`
  evolution. Quadruped's two Legs → **40 ft**, matching RAW.
- **Bite**: `NATURALATTACKS:1 Bite,…,*1,1d6` — **1d6**, the same die as the
  Wolf's own bite, so the existing natural-attack damage idiom applies
  (primary natural attack, 1.5× Str floored).

**The key structural finding:** every base-form evolution is granted
`ABILITY:Eidolon Evolution|AUTOMATIC|…` — *fixed, not chosen*. So a
Quadruped Eidolon at any level has a completely determined stat block
without the player spending a single evolution point.

## The MVP shape

Ground the chassis and the **pool size**, and defer the **spending** — the
same split already used for Warpriest's Fervor pool, Panache, and Cavalier's
Challenge uses/day, where the quantity is a verified standalone fact and the
expenditure mechanic is not modelled.

That yields a real, non-fabricated Eidolon: HD, full BAB, form-derived
saves, ability scores, natural armor, speed, max attacks, skill points, and
one 1d6 bite — all corpus-derived, nothing invented, no stub.

## The reuse is thinner than the Hunter-Wolf framing implies

This is worth stating plainly before backend plans around it. Hunter could
reuse `ground_wolf_companion_stat_block` because a hunter's companion
genuinely *is* a druid animal companion. **An Eidolon is a different
creature chassis**, and the existing function does not fit:

| | Wolf companion (existing code) | Eidolon |
|---|---|---|
| BAB | 3/4 (`hd*3/4`) | **full** (`classlevel`) |
| HD | fixed 2, and `wolf_companion_hit_dice` carries `debug_assert_eq!(companion_level, 1)` | **= Summoner level, all 20** |
| Saves | hardcoded good Fort/Ref, poor Will | **configurable per base form** |
| Ability scores | hardcoded species constants | **race + form bonuses** |

**The pattern reuses; the function does not.** What genuinely carries over:
the standalone-records-never-folded-into-the-owner's-totals shape (including
the `id_prefix` / `owner_class_label` signature), plus
`durability::average_hit_die_value` and `ability_modifier`. This is the same
category of correction as Shaman's Spirit Animal turning out to be a
Familiar rather than an Animal Companion — right idiom, wrong function.

Note also that the `debug_assert_eq!(companion_level, 1)` boundary Hunter
worked around is a hard blocker for any attempt to route the Eidolon through
the Wolf path, since the Eidolon needs all 20 levels by construction.

## Build-time hazards

1. **`Evolution ~ Legs` carries `COST:2` but is granted free.** Quadruped
   gets it twice automatically. An implementation that sums `COST:` over
   granted evolutions would charge **4 points against a 3-point level-1
   pool** and produce an impossible negative. The `AUTOMATIC` grants do not
   draw on the pool at all.
2. **Saves are per-form, not fixed.** Quadruped's good Fort+Reflex happens
   to match the Wolf's, which makes it easy to hardcode by accident and then
   be silently wrong for Biped (Fort+Will) and Serpentine (Will+Reflex).
3. **A Favored Class Bonus term inflates the pool** —
   `BONUS:VAR|EidolonEvolution|EidolonFavoredClassBonusEvolutionPointsEveryFour/4`.
   Confirm it is provably vacuous here, the same check that cleared
   Alchemist's Gnome-only `BonusBombCount`.
4. **`mastervar()` indirection.** Several Eidolon vars mirror the master's
   (`mastervar("EidolonEvolution")`, `mastervar("EidolonNaturalArmorBonus")`)
   — the values are computed on the Summoner and mirrored onto the
   companion. Don't compute both and double-count.
5. **Aspect deducts from the pool.** `FOLLOWER:EidolonAspect=1/2/3` each
   apply `BONUS:ABILITYPOOL|Eidolon Evolution|-1`. Aspect should be deferred;
   make sure the deduction doesn't fire when it is.
6. **104 real `KEY:Evolution ~ …` records, alongside ~102 parallel
   `Temp Evolution ~ …` records.** Do not sweep the latter — that was my own
   earlier "353" error, already corrected once.
7. **Bite is form-restricted.** Its `TYPE:` carries
   `EvolutionQuadruped.EvolutionSerpentine` — Biped does not get it. Another
   reason not to generalize from whichever form gets built first.

## Explicitly deferred and named

The full evolution point-buy economy (104 records, costs 1-4, with base-form
and level prerequisites); the other two base forms; Aspect / Greater Aspect;
Life Link; Bond Senses; Shield Ally / Greater Shield Ally; Maker's Call;
Transposition; Twin Eidolon; the Summon Monster spell-like ability; and
Summoner's own spontaneous CHA spellcasting (`HD:8`, `SPELLSTAT:CHA`,
`MEMORIZE:NO`, its own list) — a separate subsystem on the scale of
Bloodrager's, not part of this MVP.

**Honest status expectation:** Summoner stays **Blocked** — on unspent
evolutions and on its own deferred spellcasting. `named_features_wired`
0 → 1 or 2 depending on whether the pool counts as its own mechanism
alongside the Eidolon slot. This does not reach Computed, and it should not
be described as "the Eidolon subsystem built" — it is one canonical form's
chassis, honestly bounded.

## Open questions for the lead

1. **Is "evolution points unspent" an honest posture, or an incomplete
   character?** Unlike "not currently raging" (a genuinely valid PF1
   posture), RAW expects the points to be allocated. My read is that this is
   the Wizard-unchosen-spells shape — ground the real facts, keep a
   claim-blocking `evolutions_deferred` diagnostic, and let Summoner stay
   Blocked (which is the expected outcome regardless). Worth confirming
   rather than assuming, since it decides whether the MVP is honest.
2. **Quadruped vs Biped as the canonical form.** I picked Quadruped for the
   Wolf-analogue reuse and the smaller automatic-evolution set. Biped is
   arguably the more iconic summoner eidolon, if you'd rather optimize for
   recognizability over structural proximity.
3. **How far into the stat block should the MVP go?** The Wolf grounds HP,
   attack bonus, and damage as well as the chassis. The Eidolon can too —
   the 1d6 bite die and full-BAB math are both verified — but that is
   meaningfully more surface than grounding the chassis and pool alone. Your
   call on where the bound sits.
