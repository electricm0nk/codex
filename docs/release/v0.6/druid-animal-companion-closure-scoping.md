# Druid Animal Companion Closure (Risks Item 8 Follow-On) — Scoping Plan

> Requested by the lead after Cleric's Touch of Good closure: with no ROI
> edge remaining between Druid's animal-companion subsystem and Monk's
> four remaining opponent-dependent feats, a quick re-scope of Druid
> turned up the same shape of finding Sorcerer's Arcane Bond and Cleric's
> Touch of Good both had — two of the animal companion's three named
> pieces are provably vacuous under this codebase's model, leaving only a
> genuinely new but SMALLER-than-feared piece: a standalone companion stat
> block, never integrated into the acting character's own totals at all
> (PF1 companions are a separate creature, not a buff on the druid).

## The central finding: Link and Share Spells are vacuous the same way
## Arcane Bond's spell-cast was; only the stat block is real new work

The existing diagnostic (`class_feature.druid.animal_companion.unsupported`)
names three things as unimplemented when an animal companion bond is
recognized: "the companion's stat block, its advancement, and its link and
share spells abilities." Verified independently (d20pfsrd, cross-checked):

- **Link (Ex)**: "A druid can handle her animal companion as a free
  action, or push it as a move action, even if she doesn't have any ranks
  in the Handle Animal skill." This is an exemption from a Handle Animal
  skill CHECK — and this codebase computes exactly three selected skills
  (Climb, Intimidate, Swim; confirmed by direct grep, the only two Handle
  Animal string hits in `pilot_compute.rs` are Bard Versatile Performance
  display-name prose, not a computed value). No Handle Animal check exists
  anywhere for Link's exemption to ever matter.
- **Share Spells (Ex)**: "The druid may cast a spell with a target of
  'You' on her animal companion (as a touch range spell) instead of on
  herself." Same structural gap as Arcane Bond's "cast a spell known":
  this codebase has zero spell-casting-resolution engine anywhere, for any
  class — no spell is ever actually cast, so retargeting one is never
  triggerable here regardless of build.

Both mirror the Sorcerer/Cleric pattern exactly: a property of what this
codebase's model represents (or doesn't), not an opponent-dependent event
— the same "provably zero, not merely unmodeled" shape, not the Deflect-
Arrows/Combat-Reflexes shape that was correctly ruled out earlier.

**What's genuinely new, and smaller than first feared**: the companion's
own stat block. PF1 Core Rulebook uses one shared "Animal Companion Base
Statistics" progression table (HD, BAB, Fort/Ref/Will, skills, feats,
natural armor adjustment, Str/Dex adjustment — by EFFECTIVE companion
level, verified against d20pfsrd) applied on top of a small per-species
baseline (size, speed, starting AC, attack, ability scores — verified for
Wolf: Str 15, Dex 16, Con 13, 1d6 bite plus grab, +2 natural armor, 50 ft.
speed). Critically, **the companion is a wholly separate creature with its
own combat stats — none of this ever touches the acting Druid's own
`compute_combat_baseline`/`compute_total_saves`/skill modifiers** (unlike
Touch of Good, which had to layer onto three existing pillars). This
means the whole slice is a set of NEW, STANDALONE explanation records,
the same "grounded value, not integrated into any existing total" shape
Fascinate's DC or Trap Sense's magnitude already use — no new pillar-
integration risk, just new IDs.

## Proposed approach (level 1 only, Wolf only — mirrors every other
## class's own "start at level 1, one canonical case" discipline)

1. Recognize the animal-companion nature bond exactly as already done
   (`DRUID_NATURE_BOND_ANIMAL_COMPANION_SELECTION_ID`, pre-existing).
2. Ground the companion's level-1 base statistics as new standalone
   explanation records (BAB, Fortitude/Reflex/Will saves, natural armor,
   bite attack, HP from HD 2 d8 — average-per-level mirrors this session's
   own maximized-first-level-plus-average idiom): a small, new,
   `class_chassis.druid.animal_companion.<field>` id family. Wolf chosen as
   the canonical species (most common PF1 companion choice, same
   "smallest defensible slice" discipline as Barbarian's fixed Longsword
   or Sorcerer's Arcane bloodline).
3. Ground Link and Share Spells as vacuous-correction records (mirrors the
   Bloodline Arcana / Arcane Bond precedent exactly): +0, non-fabricated,
   explaining precisely why each precondition can never arise here.
4. Advancement beyond companion level 1 is correctly absent at Druid level
   1 (the companion advances with the Druid's own effective level; this
   bounded slice only computes Druid level 1, so companion level 1 is the
   only real case) — future Druid level widening (already anticipated as
   its own separate task, same as every other class) would need to revisit
   this, named explicitly the same way Sorcerer's bonus-spells-at-3rd+
   split was.
5. Retire (or narrow) `class_feature.druid.animal_companion.unsupported`
   for the specific case of Human, single-class, Druid level 1, animal-
   companion nature bond recognized. A Druid with the DOMAIN nature-bond
   alternative, or an unrecognized/absent nature bond, falls through to
   the SAME unchanged catch-all — mirroring the Cleric review's
   catch-all-preservation requirement exactly (this seam has never
   recognized a domain-type nature bond at all, so that path is
   unaffected either way).
6. New tests mirroring the Barbarian/Sorcerer/Cleric shape: animal-
   companion-recognized-at-level-1 reaches `Computed` (assuming the
   Druid's own prepared-spell posture is also valid), companion stat block
   values match the verified table, an unrecognized/absent nature bond
   still falls through to the catch-all, a domain-type nature bond (not
   yet representable at all) also falls through to the catch-all.

## What stays explicitly out of scope, named honestly

- Companion advancement past level 1 (real PF1 mechanic, deferred the
  same way every other class's level-range widening was).
- Tricks known / the "Handle Animal to teach tricks" system entirely
  (no Handle Animal check exists in this codebase, per the Link finding
  above — tricks are a real, separate PF1 mechanic this slice doesn't
  touch at all, not silently assumed vacuous the way Link's specific
  exemption is).
- The domain-type nature-bond alternative (Air/Animal/Earth/Fire/Plant/
  Water/Weather domains) — this seam has never recognized it, and
  building it would inherit Cleric's own harder domain-power cost.
  Verified directly (not assumed) for two of the seven: Earth domain's
  1st-level power is Acid Dart ("unleash an acid dart targeting any foe
  within 30 feet as a ranged touch attack... 1d6 points of acid damage +
  1 per two cleric levels"), Fire domain's is Fire Bolt (identical shape,
  fire damage) — both real dice-roll damage powers targeting an opponent
  at range, the same non-vacuous, opponent-dependent shape Rebuke Death
  and Monk's remaining feats have. The other five domains weren't
  individually checked, but the pattern (elemental attack domains sharing
  Cleric's ranged-touch-damage template) makes it unlikely any is
  meaningfully cheaper; worth a targeted check only if this path is ever
  prioritized over the animal-companion one.

## Open questions for the lead / adversarial review

- **Is Wolf the right canonical species, or should the doc name a
  different one?** Wolf is PF1's most commonly chosen companion and has
  the simplest stat line (one natural attack, no special movement modes
  like burrow/swim/fly that would need their own representation) —
  recommend it as the cheapest, most defensible single case, same
  reasoning as every other "pick the simplest canonical example" decision
  this session made.
- **Companion HP**: mirrors this session's own maximized-first-HD-plus-
  average-thereafter idiom (already used for every class's own HP), or
  should it use straight average throughout since a companion isn't the
  player's own character? Recommend the same idiom for consistency unless
  there's a reason PF1 treats companion HP differently (not found in
  research so far).
- **Scale of adversarial review**: this is the third "provably vacuous
  piece(s) + one small genuinely-new piece" finding in a row (Arcane Bond,
  Touch of Good, now Link/Share Spells) — worth confirming whether the
  team wants a full review each time or whether the pattern itself is now
  trusted enough to move faster, similar to how the Ranger-pattern review
  discipline relaxed after the first few dispatch-widening slices.
