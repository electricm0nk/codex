# APG Cavalier's Mount — First APG Class-Specific Closure (Risks Item 8) — Scoping Plan

> Directed by the lead after the Monk remaining-feats closure landed,
> picking up the one flagged-but-not-built candidate from the earlier
> APG/ACG scan. This is the first widening of `has_supported_class_chassis`
> to any APG class -- confirmed by the lead that `ApgClassId::from_class_id_str`
> already mirrors `AcgClassId`'s own structure exactly, so the gate-widening
> mechanism itself needs no new design (same `== Some(ApgClassId::Cavalier)`
> exact-match discipline used 4 times now for ACG). The one piece needing
> real rigor is Horse's own stat block, verified below against 2+
> independent primary sources before any code, per the lead's explicit ask.

## Corpus findings (verified against `apg_classes.lst` / `apg_abilities_class.lst`)

- **Chassis**: full BAB, good Fortitude only, poor Reflex/Will (verified
  directly against `apg_classes.lst:42`). No `SPELLSTAT` line at all --
  Cavalier is a pure martial, non-caster class (same shape as Brawler),
  confirmed rather than assumed.
- **Mount** (`KEY:Cavalier ~ Mount`): "A cavalier gains the service of a
  loyal and trusty steed... This mount functions as a druid's animal
  companion, using the cavalier's level as his effective druid level...
  A Medium cavalier can select a camel or a horse. A Small cavalier can
  select a pony or wolf... A cavalier does not take an armor check
  penalty on Ride checks while riding his mount. The mount is always
  considered combat trained and begins play with Light Armor Proficiency
  as a bonus feat. **A cavalier's mount does not gain the share spells
  special ability.**" (already quoted and independently re-confirmed by
  the lead in the prior scan).

## Horse's stat block, verified against 2 independent primary sources
## plus the PCGen corpus as tiebreaker (mirrors the Wolf-natural-armor/
## Trip resolution methodology exactly)

Fetched both `aonprd.com/DruidCompanions.aspx?ItemName=Horse` (Archives
of Nethys, the same site Wolf's own build cites) and
`d20pfsrd.com/classes/core-classes/druid/animal-companions/` directly.
**Ability scores agree completely, zero disagreement**: Str 16, Dex 13,
Con 15, Int 2, Wis 12, Cha 6. Size (Large) also agrees on both.

**Two real disagreements found, resolved the same way Wolf's own
natural-armor/Trip question was** (corpus-backed majority over an
uncorroborated single-source reading):

- **Natural armor**: aonprd says +4; d20pfsrd says +1. The PCGen corpus's
  own `Companion (Horse)` entry (`cr_races_companion.lst:21`) states
  `BONUS:VAR|AC_Natural_Armor|4|TYPE=Base` directly -- a real, load-
  bearing hit confirming +4. Resolved: **+4 natural armor**, aonprd +
  corpus vs. d20pfsrd alone.
- **Speed**: aonprd says 50 ft.; d20pfsrd says 60 ft. The PCGen corpus's
  own base `Horse` race entry (`bestiary/b1_races.lst:235`) states
  `MOVE:Walk,50` directly. Resolved: **50 ft.**, aonprd + corpus vs.
  d20pfsrd alone.
- **Attack die sizes** (a minor, lower-stakes disagreement, given this
  codebase's own Wolf precedent only ever grounds ONE flat attack-bonus
  record, not a full multi-attack breakdown): aonprd says bite (1d4), 2
  hooves (1d6); d20pfsrd says 2 hooves (1d4), bite (1d3). Following the
  same aonprd + corpus preference established above, this slice grounds
  the hoof attack as Horse's primary natural attack (a horse's natural
  means of attack is kicking, mirroring how Wolf's own bite was chosen
  as ITS primary attack), at 1d6 per aonprd's reading -- named honestly
  as a die, not rolled, mirroring Wolf's own bite-attack record exactly.
- **4th-level advancement** (out of scope for this level-1-only slice,
  same deferral Druid's own Wolf build already established, noted here
  only for completeness): aonprd says Str+2/Con+2; d20pfsrd adds Dex+2.
  Not resolved or built this slice either way.

**Verified Horse Animal Companion starting statistics (companion level 1,
i.e. companion's own base 2 HD, mirroring `wolf_companion_hit_dice`'s own
"a 1st-level [class]'s animal companion always starts at 2 HD" framing)**:
Size Large; Str 16, Dex 13, Con 15; +4 natural armor; primary attack 2
hooves (1d6 each); Speed 50 ft.

## Proposed scope

1. **Genericize `ground_wolf_companion_stat_block`/
   `ground_wolf_companion_link_and_share_spells_vacuous`** (currently
   Wolf-specific via the hardcoded `WOLF_COMPANION_*` constants) into
   species-parameterized versions, OR add a parallel
   `ground_horse_companion_stat_block` using new `HORSE_COMPANION_*`
   constants mirroring the exact same structure. Leaning toward the
   latter (a parallel function, not a generic species-agnostic one) to
   avoid risking any behavior change to Druid's/Hunter's own already-
   shipped, already-tested Wolf output -- a genericization refactor
   touching two already-verified closures is a bigger, separate risk
   than this slice needs to take on. Will size this precisely once
   in the file and can compare both approaches directly.
2. **Skip Share Spells entirely for Cavalier's Mount** -- ground Link's
   own vacuous-correction note (same reasoning: this codebase computes
   no Handle Animal check, so Link's exemption never matters), but do
   NOT ground a Share-Spells vacuous-correction note at all, since the
   corpus explicitly states the Mount never gains Share Spells in the
   first place -- grounding a vacuous-correction note for an ability the
   Mount doesn't even have would misrepresent what's being corrected.
3. **`is_supported_cavalier_single_class`** -- exact `ApgClassId::Cavalier`
   match, mirroring all 4 ACG gates exactly, added to
   `has_supported_class_chassis`.
4. Ground the Mount unconditionally on class ownership and level alone,
   the same "no choice, no activation" shape Hunter's own Animal
   Companion already established (every Cavalier gets a Mount
   automatically, no bond-type choice the way Druid's own works) --
   Medium-cavalier-only (this codebase's fixture is Human/Medium), Horse
   assumed as the canonical species (mirrors Wolf's/Druid's own
   "assumed, no species-selection input modeled" precedent, named the
   same honest way).
5. New, narrower `class_feature.apg.cavalier.other_features_deferred
   .unsupported` diagnostic (Cavalier has no `SPELLSTAT` at all, confirmed
   directly -- a pure martial class like Brawler, so this mirrors
   Brawler's own "other_features_deferred" naming, not
   "spellcasting_deferred") replacing the generic
   `class_feature.apg.cavalier.unsupported` diagnostic, naming Cavalier's
   OTHER remaining named features (Challenge, Order, Tactician,
   Cavalier's Charge, ...).
6. Tests mirroring Hunter's own shape: dedicated diagnostic-swap test
   (verifying the Horse stat block's own exact values), positive-leak
   test, and the same negative-leak test extended to cover the first-ever
   APG-side admission (the other 5 APG classes must still produce zero
   pillar output under the same satisfying posture).

## What stays explicitly out of scope, named honestly

- Camel as an alternate Medium-cavalier species choice (this codebase
  models no species-selection input at all, mirroring Wolf's/Horse's own
  "assumed, not chosen" framing).
- Small-cavalier Pony/Wolf options (this codebase's fixture is Human,
  Medium-only).
- Mount advancement past companion level 1.
- Light Armor Proficiency and "combat trained"/no-armor-check-penalty-
  on-Ride grants (real PF1 mechanics, but this codebase computes no Ride
  check and no armor-proficiency-gated-benefit engine at all -- deferred
  the same honest way Link's Handle-Animal-check exemption is).
- Cavalier's own spellcasting posture (pending confirmation it even
  exists) and every other named Cavalier feature beyond the Mount.

## Verification requested before build

Per the lead's explicit process: pinging for a quick corpus spot-check
on Horse's numbers above (ability scores, +4 natural armor, 50 ft.
speed) before writing any code, mirroring every prior class's own
pre-build check.
