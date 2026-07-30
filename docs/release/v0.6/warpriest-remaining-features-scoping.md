# Warpriest (#9) — Fresh Scope (corrected bar; the low-priority tag is partly wrong)

> Directed by the lead: don't accept the "19 Blessings + Sacred Weapon
> active + Fervor + Channel Energy + Sacred Armor = large volume, low
> priority" tag at face value — apply the corrected standalone-grounding
> bar and check whether several remaining features are actually flat,
> self-scoped, non-opponent-dependent standalone facts (Bomb / Poison
> Resistance / Nimble shape). Specifically re-check Fervor and Sacred Armor
> (likely flat magnitudes). Read the real corpus records; flag genuine
> missed wins vs what's correctly still blocked. **Result: a nuanced answer,
> not 5-for-5. Fervor, Sacred Armor, and Channel Energy's DC ARE real flat
> missed wins the tag undersold — but unlike Skald/Hunter/Investigator/
> Alchemist, Warpriest also has a genuine hard core (touch-weapon/summon
> blessings + Sacred Weapon active) that stays correctly blocked on a
> weapon-enhancement/summon surface this engine doesn't have.**

## Real missed wins — flat magnitudes, groundable now

### Fervor — groundable (pool + heal dice, both flat)

- `BONUS:VAR|WarpriestFervorUses|WarpriestLVL/2+WIS` — a uses-per-day pool
  (self-contained quantity, Panache/Blessing-uses idiom).
- `BONUS:VAR|WarpriestFervorDice|1+max(0,min(20,WarpriestLVL)-2)/3` — the
  swift-action self-heal magnitude: 1d6 at level 2, +1d6 per 3 levels after
  (the same channel-dice shape as Cleric). A flat, verifiable heal
  magnitude, groundable standalone like Bomb's damage dice.
- **Both pieces ground as standalone facts** — the lead's instinct was
  right. (The "cast a spell as a swift action" clause is a separate
  activation not modelled, but the pool size and heal dice are flat facts.)

### Sacred Armor — groundable (AC enhancement + pool, both flat)

- `BONUS:VAR|WarpriestSacredArmorEnhancement|1+max(0,(min(20,WarpriestLVL)-7)/3)`
  — an armor enhancement bonus: +1 at level 7, +1 per 3 levels after. A
  flat magnitude, grounds standalone exactly like Sacred Weapon's base dice
  / Nimble's dodge bonus (no player-AC total needed — same documented gap).
- `BONUS:VAR|WarpriestSacredArmorUses|WarpriestLVL` — a uses-per-day pool.
- **Groundable missed win.** (The activation — spend a use to enhance armor
  for minutes — isn't modelled, but the enhancement value and pool are flat
  facts.)

### Channel Energy DC — flat, and reuses Fervor's dice

- `BONUS:VAR|WarpriestChannelEnergyDC|10+WarpriestLVL/2+WIS` — a flat save
  DC, groundable standalone (Blessing-DC / Bomb-DC idiom). And Warpriest's
  Channel draws its dice/uses from the Fervor pool we're already grounding,
  so the heal magnitude is the same Fervor dice. **More tractable than the
  original "full Channel Energy is a genuine gap" framing** — at least the
  DC and the (Fervor-shared) dice ground; only the channel-vs-Fervor
  resource-routing is the residual.

## Partially groundable — a subset of the 19 blessings

The blessings are **not a uniform bucket.** 66 blessing-power records
(33 blessings × a minor + major power each); Warpriest already grounds
Destruction Blessing's Destructive Attacks — an **activation-gated
flat-magnitude self-buff** (`max(1,level/2)`, self-applied). Blessings
whose minor power is that same shape (a swift-action flat self-buff)
ground the same way — narrow to one canonical blessing, ground its flat
magnitude, defer the rest, exactly as the original closure did for
Destruction. **A handful of the 19 are groundable this way** (self-buff
minor powers with a flat `BONUS:VAR`); this is a real, if bounded, slice.

## Correctly still blocked — a genuine hard core (not a corrected-bar miss)

Unlike the last four classes, Warpriest has a real subsystem-blocked
remainder — verified, not assumed:
- **~15 of the 66 blessing powers are touch-weapon "Strike" or summon
  "Companion" abilities** (e.g. Air/Zephyr's Gift, Good/Holy Strike, Fire
  Strike, Battle Companion) — they enhance a weapon with special abilities
  or summon a creature. Both need surfaces this engine lacks: a
  **weapon-enhancement activation surface** and a **summon subsystem**.
- **Sacred Weapon's active enhancement** — same weapon-enhancement gap
  (spend a use to grant the weapon a +X enhancement / weapon special
  abilities). The original assessment was right that this is a genuine gap;
  confirmed. (Note: Sacred Weapon's *base damage dice* are already grounded
  — and were the subject of the earlier `/20`→`/15` fix; only the *active
  enhancement* is blocked.)
- These stay correctly deferred — not standalone-fact-shaped, and blocked
  on real missing engine state (weapon enhancement / summoning), the honest
  kind of block (like opponent-dependent bonuses or the Familiar/Eidolon
  subsystems).

## Honest boundedness read

Warpriest is a **real medium slice with a genuinely-blocked remainder** —
not the uniformly-low bucket the tag implied, but not fully tractable
either:
- **Groundable now (missed wins):** Fervor (pool + heal dice), Sacred Armor
  (enhancement + pool), Channel Energy DC (+ Fervor-shared dice). Three
  real flat wins the low-priority tag undersold.
- **Groundable, bounded:** a subset of self-buff blessings (Destructive-
  Attacks shape) — narrow to one canonical, defer the rest.
- **Correctly blocked:** ~15 touch-weapon/summon blessing powers + Sacred
  Weapon active enhancement (weapon-enhancement/summon subsystem gap).

So the corrected bar *does* upgrade Warpriest — Fervor + Sacred Armor +
Channel DC are a real slice worth building — but it's honestly "4.5 of 5,"
with a hard core that stays blocked for a real reason, not a scoping
artifact. Worth re-tagging from "low" to "partial: a real Fervor/Sacred-
Armor slice available, weapon-enhancement remainder genuinely deferred."

## Open questions for the lead

1. Greenlight the **Fervor + Sacred Armor + Channel Energy DC** slice (three
   flat missed wins), plus one canonical self-buff blessing narrowed the
   Destructive-Attacks way? That's a real medium closure independent of the
   blocked weapon-enhancement remainder.
2. The **weapon-enhancement activation surface** (Sacred Weapon active +
   the ~15 touch-weapon blessings) is a genuine shared gap — worth a
   scoped design task of its own later (it also unblocks a chunk of
   blessings at once), or leave deferred? Flagging it as the Warpriest
   analog of the Familiar/Eidolon subsystem decisions.
3. Build-time note (Sacred Weapon lesson): Fervor's `…-2)/3` and Sacred
   Armor's `…-7)/3` step formulas and Channel DC want direct per-level
   re-derivation from their own records, not carried from Cleric's
   channel-dice by assumption.
