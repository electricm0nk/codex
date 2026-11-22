# APG/ACG Warpriest — Full Class Build Scoping (Runner-Up After Arcanist)

> Self-directed after the Arcanist follow-on dead-end (seeding alone
> can't reach `Computed` there -- Exploits blocks unconditionally).
> Moving to Warpriest, the documented runner-up from
> `full-class-build-comparative-scoping.md`. This is a scoping check
> before committing to the full build, matching the established rhythm
> for genuinely novel combinations -- flagging one real judgment call
> below before writing code.

## Corpus findings (verified directly against `acg_classes.lst` / `acg_abilities_class.lst`)

- **Chassis**: 3/4 BAB, good Fortitude/Will, poor Reflex. HD 8.
  `SPELLSTAT:WIS`, no `MEMORIZE:NO` (prepared, same shape as Cleric),
  `SPELLLIST:1|Cleric` (reuses Cleric's real spell-list content).
- **Spells-per-day table**: found directly under Warpriest's own
  "Level progression" block (`acg_classes.lst:386-405`, real `CAST:`
  rows, not a derived formula): level 1 `CAST:3,1`, level 2 `CAST:4,2`,
  level 3 `CAST:4,3`, level 4 `CAST:4,3,1`. **Correction applied before
  it became a doc claim this time** (learned from the Arcanist
  overclaim): checked directly against this codebase's own
  `cleric_base_spells_per_day_table` (`pilot_compute.rs:21092-21102`):
  Warpriest's own table matches Cleric's EXACTLY at levels 1-2 (`[3,1]`,
  `[4,2]`), but genuinely DIVERGES from level 3 onward -- Cleric gets a
  2nd-level spell slot at level 3 (`[4,2,1]`), Warpriest doesn't unlock
  2nd-level spells until level 4, and even then at a lower count
  (Warpriest level 4 `[4,3,1]` vs Cleric level 4 `[4,3,2]`). Same shape
  as Arcanist-vs-Wizard: the spell-LIST content is genuinely shared, the
  per-day NUMBERS are NOT -- Warpriest needs its own real, independently
  verified table, not a byte-identical reuse.
- **Level-gating of Warpriest's OTHER named features** (`acg_classes.lst:
  376-382`'s own "Level Progression Abilities" block, giving the exact
  level each feature first grants): Blessings, Sacred Weapon, Orisons,
  Spontaneous Casting, Aura, Focus Weapon, Bonus Languages all grant at
  level 1. **Fervor grants at level 2, Bonus Feats at level 3, Channel
  Energy at level 4, Sacred Armor at level 7, Aspect of War at level
  20.** This matters directly for scope: a level-1-bounded first slice
  (mirroring every other class's own "levels 1-3" bootstrap discipline)
  would only ever need Blessings + Sacred Weapon among the "big"
  features -- Channel Energy and Sacred Armor are structurally
  unreachable within levels 1-3 regardless, and Fervor only becomes
  relevant at level 2-3.
- **Blessings** (`KEY:Warpriest ~ Blessings`): "call upon the power of
  your blessings %1 times per day. DC is %2" -- uses-per-day formula
  `(WarpriestBlessingLVL/2)+3`, DC formula `(WarpriestBlessingLVL/2)+10+
  WIS` (or `+CHA` for a different sub-variant not relevant here). A
  Warpriest picks TWO Blessings at 1st level from a list matching
  Cleric's own ~20 domains (Air, Animal, Artifice, Chaos, Charm,
  Community, Darkness, Death, Destruction, Earth, ... verified via
  `grep -oE "KEY:[A-Za-z ]+Blessing ~ "`), each granting a Minor power
  (1st level) and Major power (later level). This is structurally
  identical to Cleric's own domain system, which this session already
  solved via the "pick ONE canonical, self-scoped-only option"
  narrowing (Good domain / Touch of Good).
- **Destruction Blessing's own Minor power** ("Destructive Attacks"):
  "You can touch an ally and bless it with the power of destruction. For
  1 minute, the ally gains a +[max(1,level/2)] morale bonus on weapon
  damage rolls." **This is structurally IDENTICAL to Cleric's own Touch
  of Good** (touch another creature, grant a flat bonus, target-creature
  representation this codebase doesn't have) -- the exact same
  "self-application only" narrowing Touch of Good already used applies
  cleanly here too. Proposing Destruction as the one canonical Blessing
  this closure grounds, for the identical reason Good was picked for
  Cleric: a pure, flat, self-scoped bonus with no new engine state.
- **Sacred Weapon**: two parts. (1) A passive base-damage-die upgrade,
  real formula (`WarpriestSacredWeaponBaseDice`d`...BaseDiceSize`,
  size-class-dependent via `PREBASESIZEEQ:M` for a Medium weapon like
  this codebase's own Longsword fixture): at level 1, `dice_size =
  if(LVL<5,6,...)` = 6, `dice_count = 1+min(1,LVL/20)` = 1 -- i.e. 1d6.
  Buildable as a flat formula, but genuinely near-zero VALUE at level 1
  for this fixture's Longsword (whose own native base damage is already
  1d8, better than Sacred Weapon's 1d6 at this level) -- the real
  benefit only kicks in at higher levels. Named honestly as a real,
  grounded-but-currently-inert value, mirroring Brawler's own "AC Bonus
  is genuinely +0 at level 1" precedent. (2) An active swift-action
  weapon-enhancement mechanic (granting a magic weapon enhancement bonus
  plus a menu of weapon special abilities -- Flaming, Frost, Keen,
  Shock, and several alignment-gated ones) -- this is a real magic-item-
  creation-shaped system, explicitly out of scope, the same "genuinely
  bigger scope" reasoning that already excluded Swashbuckler's Deeds and
  Investigator's Inspiration from the single-ability scan.

## Proposed scope (levels 1-3, mirroring the established bootstrap-bound discipline)

1. `is_supported_warpriest_single_class` -- exact `AcgClassId::Warpriest`
   match, mirroring the five existing ACG/APG gates exactly.
2. Real prepared-spellbook spellcasting, mirroring
   `ground_arcanist_prepared_spellbook`'s own shape exactly (same
   pattern proven twice now: Wizard's own validation minus whatever
   Warpriest doesn't have -- no opposed-school mechanic here either).
   Own, independently-verified spells-per-day table (see above), bounded
   to levels 1-3 the same way Wizard's/Arcanist's own tables are.
3. Blessings: uses-per-day (`(level/2)+3`) and DC
   (`(level/2)+10+WIS`) as flat explanation records (no gate, always
   grounded, mirroring Cleric's own Channel Energy dice/uses shape) plus
   Destruction Blessing's own Destructive Attacks minor power (self-
   application-only, activation-gated -- mirrors
   `ground_or_block_cleric_domain`'s exact three-branch shape: not-
   active / active-self-applied / active-without-recognized-choice).
   Every OTHER Blessing choice falls into a still-blocking catch-all,
   mirroring Cleric's own "no domain chosen" branch exactly.
4. Sacred Weapon's base-damage-die formula as a flat explanation record
   (real value, honestly near-zero for this fixture at level 1) -- the
   active enhancement mechanic stays explicitly out of scope.
5. New, narrower `class_feature.acg.warpriest.other_features_deferred
   .unsupported` diagnostic naming: the OTHER ~19 Blessing types, Sacred
   Weapon's own active enhancement mechanic, Fervor (not yet accessible
   within this bounded level range for a level-1 test but should still
   be named as a real future gap), Channel Energy, Bonus Feats/Sacred
   Armor/Aspect of War (all beyond level 3 anyway), Aura, Focus Weapon,
   Spontaneous Casting, Bonus Languages.
6. **Class-skill list: 14 skills, real code change needed here --
   UNLIKE Wizard/Arcanist.** Checked directly against
   `KEY:Warpriest ~ Class Skills`'s own `CSKILL:` tokens: Climb,
   Diplomacy, Handle Animal, Heal, Intimidate, Knowledge (Engineering),
   Knowledge (Religion), Profession, Ride, Sense Motive, Spellcraft,
   Survival, Swim, and Craft (via `TYPE=Craft`). Warpriest's own list
   genuinely INCLUDES all three of the skills
   `selected_skill_class_skill_bonus_applies` tracks (Climb/Intimidate/
   Swim) -- the opposite finding from Wizard/Arcanist, whose lists
   include none of the three. This function currently only recognizes
   Fighter/Rogue (`pilot_compute.rs:23396-23400`), so a Warpriest today
   would silently get ZERO class-skill bonus despite genuinely earning
   one per RAW -- the same shape bug this session's own operator-cited
   history already found and fixed once for Wizard (SWARM_TASKS.md:
   "Class-skill-modifier bug ... Climb/Intimidate/Swim bonus applied
   regardless of actual class"). This closure needs to widen
   `selected_skill_class_skill_bonus_applies` to include Warpriest too,
   with its own dedicated test (mirroring
   `fighter_still_gets_the_class_skill_bonus_on_all_three_skills`), not
   just verify the existing generic mechanism answers correctly for
   free.

## What stays explicitly out of scope, named honestly

- 19 of the 20 Blessing types (only Destruction is grounded).
- Sacred Weapon's active enhancement mechanic (a real magic-item-
  creation-shaped system).
- Fervor, Channel Energy, Sacred Armor, Aspect of War (each gated to a
  level beyond this slice's own bounded 1-3 range, or in Fervor's case,
  level 2 -- worth a follow-on slice, not blocking this one).
- Spontaneous Casting (Warpriest's own cure/inflict spell-swap
  mechanic, mirroring Cleric's own unmodeled spontaneous-conversion gap).
- Aura, Focus Weapon, Bonus Languages (flat/simple in principle, but
  not the priority scope for this slice -- named in the deferred
  diagnostic, not built).

## Open question for the lead

Before committing to the build: is grounding Destruction Blessing as the
one canonical, self-scoped Blessing (mirroring Good domain's own Touch
of Good precedent exactly) the right MVP choice, or is there a reason to
prefer a different Blessing type? Destruction was picked because its
Minor power is structurally identical to Touch of Good (touch an ally,
flat self-scoped bonus) with no alignment restriction narrower than any
other Blessing -- but flagging this as the one real judgment call here,
same discipline as every other closure this session.
