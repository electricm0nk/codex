# Brawler (#5) — Fresh Scope of the 9 Remaining Features

> Expected to come back thin: backend already deepened Brawler
> (`4555a862`, `named_features_wired` 1→3), so the cheap wins should be
> picked over.
>
> **It did not come back thin.** Six of the nine remaining features carry
> real, flat, self-scoped `BONUS:VAR` magnitudes. Three are correctly
> blocked. More usefully, three of my own working assumptions were wrong and
> were caught by checking rather than reasoning — including one that would
> have shipped a fabricated hazard to backend.

Already grounded: AC Bonus, Brawler's Cunning, Brawler's Strike (plus the
class-skill widening fix). 14 `KEY:Brawler ~ …` records total.

## Verification result: every formula agrees with its own grant level

Checked each remaining feature's formula against the class table's actual
grant level — the check that caught Shaman's level-8-gated Healer's Touch:

| feature | granted | formula yields at that level |
|---|---|---|
| Martial Flexibility | 1 | `max(1,L/2)+3` → 4 ✓ |
| Martial Training | 1 | level-equivalence, no gate ✓ |
| Bonus Feats | 2 | `(1+L)/3` → 1 ✓ |
| Brawler's Flurry | 2 | `min((L+6)/7,3)` → 1 ✓ |
| Maneuver Training | 3 | `(L>2)+…` → 1 ✓ |
| Knockout | **4** | `(L+2)/6` → 1 ✓ |

**No gate mismatches anywhere.** Worth stating positively, since this is the
first class this segment where that check came back completely clean.

## Real missed wins

### Knockout (granted 4th) — three real tokens

- `BONUS:VAR|KnockoutTimes|(BrawlerLVL+2)/6` — uses/day.
- `BONUS:VAR|KnockoutStatBonus|max(STR,DEX)`.
- `BONUS:VAR|KnockoutDC|(BrawlerLVL/2)+10+KnockoutStatBonus` — a flat save
  DC, the Bomb-DC / Channel-DC idiom.

The *effect* (target falls unconscious on a failed Fortitude save) is
opponent-directed and defers; the **DC and the per-day pool are self-scoped
and ground**, the same split already accepted for Shaman's Channel DC and
Alchemist's Bomb DC.

### Brawler's Flurry (granted 2nd) — two real tokens

- `BONUS:VAR|BrawlersFlurryExtraAttacks|min((BrawlersFlurryLVL+6)/7,3)` →
  **1 / 2 / 3** extra attacks, capped at 3.
- `BONUS:VAR|BrawlersFlurryAttackPenalty|-2` — a flat self-applied attack
  penalty, the same self-penalty shape as Bloodrage's and Challenge's AC
  penalties (and here it *does* have a real `BONUS:VAR` token, unlike
  Cavalier's Challenge).

### Bonus Feats (granted 2nd) — flat count

`BONUS:ABILITYPOOL|Brawler Bonus Feat|(1+BrawlerLVL)/3` → feats at levels
2, 5, 8, 11, 14, 17, 20 (7 by 20th). Same shape as Cavalier's
`CavalierLVL/6`.

### Martial Flexibility (granted 1st) — pool grounds, chooser defers

`BONUS:VAR|BrawlerMartialFlexibilityTimes|max(1,BrawlerMartialFlexibilityLVL/2)+3`
— a uses/day pool. Grounds. The ability itself ("gain the benefit of a
combat feat she doesn't possess") is a chooser over the whole combat-feat
list — that half defers, and under the ratified Skill Focus precedent it
would need an explicit recorded choice, never a silently seeded canonical
feat.

### Maneuver Training (granted 3rd) — count + one canonical maneuver

- Count: `(BrawlerLVL>2)+(BrawlerLVL>6)+(BrawlerLVL>10)+(BrawlerLVL>14)`,
  **plus a second `BONUS:VAR` line adding `1` when `BrawlerLVL>18`** →
  1 / 2 / 3 / 4 / 5 at levels 3 / 7 / 11 / 15 / 19.
- Per-maneuver bonus: `<Maneuver>ManeuverTrainingBonus =
  BrawlerManeuverTraining` for the first pick, `-1` for the second, `-2`
  third, `-3` fourth, `-4` fifth, across 10 maneuvers (Bull Rush, Dirty
  Trick, Disarm, Drag, Grapple, Overrun, Reposition, Steal, Sunder, Trip).

Narrow to one canonical maneuver exactly as Animal Focus narrowed to Bull.

### Martial Training (granted 1st) — real leverage, and a name collision

`BONUS:VAR|MonkFeatQualify|BrawlerLVL`, `BONUS:VAR|MonkLVL|BrawlerLVL`,
`BONUS:VAR|FighterWeaponQualifyLVL|BrawlerLVL` — brawler levels count as
fighter *and* monk levels for qualifying for feats. This has genuine
existing leverage: the repo already has `feat_prereqs` machinery (the module
Arcanist's Metamagic Knowledge reused).

**Do not carry the Alchemist verdict across by name.** *Alchemist's* Martial
Training was correctly ruled a genuine no-op with zero `BONUS` tokens.
**Brawler's identically-named feature has three real ones.** Same name,
different class, opposite verdict.

## Correctly blocked

- **Awesome Blow (16th) and Improved Awesome Blow (20th)** — zero tokens of
  any kind, and both are combat maneuvers resolved against an opponent.
  Genuine no-ops in the Nature Training sense, and gated far above any level
  the tests exercise.
- **Close Weapon Mastery (5th)** — DESC-only, and not a flat magnitude: it
  substitutes "the unarmed strike damage of a brawler 4 levels lower," a
  shifted table lookup. Monk's own unarmed-damage work may be reusable, but
  that is a table-reuse question, not a standalone fact.
- **The deferred halves** — Martial Flexibility's feat selection, Maneuver
  Training's other 9 maneuvers, Knockout's unconsciousness effect.

## Build-time hazards

1. **`INTSCORE` vs bare `STR`/`DEX` — the corpus disambiguates deliberately,
   so do not normalize them.** Brawler's Cunning is
   `max(13,INTSCORE)` — an explicit *score* token, which is why the repo's
   `brawler_cunning_effective_intelligence_score` correctly takes a score.
   Knockout is `max(STR,DEX)` — bare tokens, i.e. *modifiers* (a DC of
   `10 + L/2 + score` would be absurd). A Knockout implementation that
   copies the Cunning idiom would produce a wildly wrong DC.
2. **`BrawlerManeuverTraining` is set by TWO stacking `BONUS:VAR` lines.**
   The `(L>2)+(L>6)+(L>10)+(L>14)` line tops out at 4; a separate
   `1|PREVARGT:BrawlerLVL,18` line supplies the 5th. Reading only the first
   gives 4 at levels 19-20 instead of 5 — the same partial-read shape as the
   Sacred Weapon divisor bug.
3. **Maneuver Training's bonus degrades by pick order** (first pick gets the
   full count, second `-1`, and so on). Narrowing to one canonical maneuver
   means it is the *first* pick and takes the undegraded value — correct for
   the MVP, but do not hardcode that assumption if multiple picks are ever
   modelled.
4. **Knockout is granted at 4th level, not 5th.** I expected 5th from
   recollection; the class table says 4, and `(L+2)/6` yields exactly 1
   there. The corpus is internally consistent — do not "fix" the formula to
   match a remembered table.
5. **Do not conclude "corpus-incomplete" from a feature record alone.**
   `Brawler ~ Brawler's Flurry` shows three bare `DEFINE:`s and no setters
   on its own record — visually identical to the genuinely-unset
   Interstellar Void case. All three **are** set elsewhere in the file. I
   nearly reported this as an incompleteness; check the whole file for
   setters before calling a var unset.

## Recommended bounded MVP

All six, in rough cost order: Bonus Feats and Brawler's Flurry (flat
counts/penalty), Knockout (pool + DC), Martial Flexibility (pool), Maneuver
Training (count + one canonical maneuver), Martial Training (three
level-equivalence facts, with `feat_prereqs` leverage worth confirming
before committing to it).

**Honest status expectation:** stays Blocked — Awesome Blow, Close Weapon
Mastery, and the deferred chooser halves keep the
`other_features_deferred` diagnostic alive. `named_features_wired` 3 → ~9,
subject to your cluster-collapsing call (Martial Training's three vars are
one mechanism; Knockout's DC and pool arguably one).

## Open questions for the lead

1. **Is Martial Training worth building now or is the `feat_prereqs`
   integration bigger than it looks?** The three level-equivalence vars are
   trivially flat, but "brawler levels count as fighter and monk levels for
   feat qualification" only *means* anything once wired into prerequisite
   checking. Grounding the facts standalone is cheap and honest; wiring them
   is a real integration. I'd ground the facts and defer the wiring, but
   that split is worth your explicit call given it's the same
   fact-vs-consumer line the corrected bar already settled once.
2. **Close Weapon Mastery** — worth a look at whether Monk's unarmed-damage
   table is genuinely reusable with a 4-level shift, or is that a follow-on?
   I did not scope Monk's table in this pass, so I'm flagging it as unknown
   rather than asserting reuse is available.
