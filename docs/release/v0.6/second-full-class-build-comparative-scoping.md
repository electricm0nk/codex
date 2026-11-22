# Second Full Class Build — Comparative Scoping Across the Remaining 7 ACG/APG Classes

> Directed by the lead after Warpriest landed: comparative scoping
> across the 7 fully-untouched classes (Investigator, Shaman, Slayer,
> Swashbuckler, Oracle, Summoner, Witch), same discipline as the
> Arcanist/Warpriest picks. Recommendation: **Slayer**, with one real
> correction to the original comparison doc's own claim about Shaman.

## Correction to `full-class-build-comparative-scoping.md`

That doc's own findings table said Shaman's Spirit Animal "could reuse
Wolf-companion code, proven pattern." **This is wrong, caught before
building anything on it.** Checked directly against
`acg_abilities_class.lst`'s own `KEY:Shaman ~ Spirit Animal` record:
`TYPE:Shaman Class Feature.SpecialQuality.Extraordinary.Familiar` --
Shaman's own companion is a **Familiar**, not an Animal Companion. This
is the SAME unbuilt infrastructure gap Witch's own Familiar needs
(confirmed earlier: no familiar stat-block code exists anywhere in this
codebase, only Sorcerer's Arcane Bond recognizing `bond:familiar` as an
identity choice with no real stat block). Shaman's real cost is
genuinely HIGHER than the original comparison estimated -- not a
disqualifying finding on its own, but worth correcting before it became
a stale assumption a future build relied on.

## New findings: Slayer (verified directly against `acg_abilities_class.lst`)

Slayer is a **non-caster** (confirmed: no `SPELLSTAT` token on
`CLASS:Slayer` at all) -- zero spellcasting scope, unlike every closure
so far this segment except Cavalier/Brawler/Hunter. Its remaining named
features, checked one at a time:

- **Sneak Attack**: dice count `SlayerLVL/3` -- a real, flat formula.
- **Trap Sense**: `max(1, SlayerLVL/3)` -- a real, flat formula.
- **Trapfinding**: `+SlayerLVL/2` on Perception (to locate traps) and
  Disable Device -- a real, flat formula.
- **Track**: `+max(SlayerLVL/2, 1)` on Survival (to follow tracks) -- a
  real, flat formula.
- **Studied Target** (Slayer's own real class-feature name -- there is
  no separate "Quarry" record; my own earlier scoping doc's use of
  "Quarry/Studied Target" interchangeably was imprecise): confirmed
  genuinely opponent-dependent ("study an opponent... bonuses against
  THAT opponent... until the opponent is dead") -- the same "no
  opponent representation exists anywhere in this codebase" wall that
  already correctly excluded it from the single-ability scan. **A full
  build still hits this wall**, same as this doc's own earlier
  confirmation.
- **Slayer Talents**: a chooser-list (mirrors Rogue Talents/Rage
  Powers/Discoveries) -- defer, named but not built.

**None of Sneak Attack/Trap Sense/Trapfinding/Track integrate into any
existing total in this codebase** (this engine has no "trap AC/save"
pillar, no "sneak attack damage" total, and Perception/Disable
Device/Survival aren't among the three skills
`compute_selected_skill_modifiers` tracks). This sounds like a gap, but
it is **already an established, precedented idiom, not a compromise
unique to this closure**: Barbarian's own Trap Sense
(`class_feature.barbarian.trap_sense`, `pilot_compute.rs:16045-16062`)
and Rogue's own Trap Sense (`class_feature.rogue.trap_sense`,
`pilot_compute.rs:18259-18270`) are BOTH already grounded as standalone
flat explanation records with no further integration -- the same shape
this closure would use for Slayer's own four sub-features.

**A third real class-skill-bonus bug, same shape as Warpriest's own**:
Slayer's own class-skill list (`CSKILL:Acrobatics|Bluff|Climb|...|
Intimidate|...|Swim|...`) genuinely includes Climb, Intimidate, AND
Swim. `selected_skill_class_skill_bonus_applies` would need widening to
include Slayer too, once `has_supported_class_chassis` admits it --
this closure needs to check and likely fix this the same way Warpriest's
closure did, not assume it's already correct.

## Findings table (updating the original comparison for the 7 remaining classes)

| Class | Spellcasting scope | Marquee feature(s) | Real buildable content | Verdict |
|---|---|---|---|---|
| **Slayer** | None (non-caster) | Studied Target -- opponent-dependent, confirmed still blocked at full-build scale | Sneak Attack dice, Trap Sense, Trapfinding, Track -- 4 real flat formulas, all following the ALREADY-precedented "flat record, no total-integration" idiom (Barbarian's/Rogue's own Trap Sense) | **Cheapest remaining -- recommended** |
| Shaman | Own list, fresh build (like Witch) | Spirit (domain-like, could narrow to one canonical type like Blessings/Domain) + Spirit Animal (a **Familiar**, not Animal Companion -- corrected above, same unbuilt gap as Witch) + Hex (chooser-list) | Real, but the Familiar gap raises this class's real cost above the original estimate | Not recommended this round |
| Oracle | Own spontaneous 9th-level table (biggest table-verification task of the 9) | Mystery (10 choices) + Curse (5 choices) -- could narrow to one canonical pair, mirroring Blessings/Domain | Real, but the spontaneous 9th-level spells-known table is the single biggest verification task among all remaining classes | Not recommended this round |
| Investigator | `SPELLLIST:1|Alchemist` -- confirmed false shortcut (Alchemist's own casting isn't built) | Inspiration -- a chooser-list, already flagged hardest of the whole roster | 95 named features (dominant outlier) | Not recommended |
| Swashbuckler | None (non-caster) | Panache pool gating 20+ Deeds | Confirmed not cheap at full-build scale too | Not recommended |
| Summoner | Own short list, fresh build | Eidolon -- a whole separate creature with its own evolution-point system | Effectively a second character sheet | Not recommended |
| Witch | Own list, fresh build | Familiar (confirmed no stat-block infrastructure exists) + Hex (chooser-list) | Smallest raw feature count (7) but no real reuse anywhere | Not recommended this round |

## Proposed scope for Slayer

1. `is_supported_slayer_single_class` -- exact `AcgClassId::Slayer`
   match, mirroring the six existing ACG/APG gates exactly.
2. `slayer_sneak_attack_dice(level) -> i16` (`level/3`),
   `slayer_trap_sense_bonus(level) -> i16` (`max(1, level/3)`),
   `slayer_trapfinding_bonus(level) -> i16` (`level/2`),
   `slayer_track_bonus(level) -> i16` (`max(level/2, 1)`) -- four flat,
   pure functions, each grounded as its own standalone explanation
   record (`class_feature.acg.slayer.sneak_attack_dice`,
   `.trap_sense_bonus`, `.trapfinding_bonus`, `.track_bonus`), mirroring
   Barbarian's/Rogue's own Trap Sense precedent exactly -- no
   activation gate needed (these are all always-on class features, not
   Rage-shaped).
3. New, narrower `class_feature.acg.slayer.other_features_deferred
   .unsupported` diagnostic naming Studied Target (opponent-dependent)
   and Slayer Talents (chooser-list) as the genuinely still-missing
   pieces.
4. **Check and likely fix** `selected_skill_class_skill_bonus_applies`
   to include Slayer -- do not assume it already answers correctly,
   verify with a dedicated test the same way Warpriest's closure did.

## What stays explicitly out of scope, named honestly

- Studied Target (opponent-dependent, no target-creature representation
  exists anywhere in this codebase).
- Slayer Talents (chooser-list, real mechanical variety, named but not
  built).

## Open question for the lead

None of the four flat Slayer formulas integrate into an existing total
-- I'm treating this as fine, matching Barbarian's/Rogue's own Trap
Sense precedent, but flagging it explicitly since it means this closure
adds four standalone facts rather than improving any existing pillar
(unlike Arcanist's/Warpriest's own spellcasting, which added a genuinely
new integrated pillar). If you'd rather this closure be skipped in favor
of something with more product-visible integration (or Arcanist's own
Exploits, per your own alternative), your call -- I don't think this
changes the "Slayer is cheapest" verdict, just the shape of its payoff.
