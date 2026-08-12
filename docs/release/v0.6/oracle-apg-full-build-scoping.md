# APG Oracle — Full Class Build Scoping (8th ACG/APG Closure Target)

> Directed by the lead: the 6 remaining untouched classes are all
> genuinely harder than what's already closed (Shaman/Witch need a
> brand-new Familiar subsystem, Summoner needs an Eidolon, Investigator's
> Inspiration is the hardest chooser-list on the roster, Swashbuckler's
> Panache/Deeds is similar scale) -- but Oracle's own real cost is "a big
> spontaneous spells-known table to verify carefully," no new subsystem
> required. Scoping it the same way as Arcanist/Warpriest/Slayer before
> building.

## Corpus findings (verified against `apg_classes.lst` / `apg_abilities_class.lst`)

- **Chassis**: 3/4 BAB, good Will, poor Fortitude/Reflex. HD 8.
  `SPELLSTAT:CHA`, `MEMORIZE:NO` (spontaneous, like Sorcerer/Bard --
  **NOT** like Cleric/Wizard/Arcanist/Warpriest, all of which are
  prepared). `SPELLLIST:2|Cleric|Oracle` -- reuses Cleric's real
  spell-list content PLUS a separate "Oracle" list (Mystery-granted
  bonus spells).
- **A real structural correction to my own earlier framing**: because
  Oracle is spontaneous, it needs the FULL Sorcerer/Bard-shaped
  known-spell validation (a known-spell-count cap per level, checked
  against the real spell list, PLUS a separate per-day-cast cap) --
  genuinely more machinery than Arcanist's/Warpriest's own PREPARED-shape
  validation (record + prepare, no separate "known" cap at all).
  Confirmed by reading `unmet_sorcerer_known_spell_conditions`/
  `sorcerer_spells_known_table`/`ground_sorcerer_known_spells`
  (`pilot_compute.rs:19791-19934`) as the real shape to mirror, not
  Wizard's/Arcanist's/Warpriest's own simpler prepared-spellbook
  functions. This is a real, structural difference from the last two
  closures, not just "a bigger table" -- worth naming honestly before
  building, the same discipline that corrected the Arcanist/Wizard
  spell-list-reuse framing earlier this session.
- **Real per-level table**, found directly under Oracle's own "Level
  progression" block (`apg_classes.lst:114-134`, real `CAST:`/`KNOWN:`
  rows, not derived formulas): level 1 `CAST:0,3` / `KNOWN:4,2`, level 2
  `CAST:0,4` / `KNOWN:5,2`, level 3 `CAST:0,5` / `KNOWN:5,3`. The `0` in
  the CAST column's first slot is the corpus's own sentinel for
  "orisons cast at will, no daily cap" (the same real PF1 rule
  Sorcerer/Bard/Oracle all share, unlike Cleric/Wizard/Warpriest's own
  prepared orisons, which DO have a real daily-prepare cap) -- not a
  literal zero-casts value.
- **Cross-verification note**: a legacy.aonprd.com fetch attempt for
  this table returned an internally self-contradictory result (claimed
  "no 1st-level spells until 4th level" in one section while showing 2
  known 1st-level spells at level 1 in another) -- the same kind of
  unreliable-fetch failure mode already seen once this session for
  Arcanist's own table. Did NOT trust it. A separate, real web search
  independently confirmed "an oracle begins play knowing four 0-level
  spells and two 1st-level spells" -- matching the raw corpus `KNOWN:4,2`
  exactly. Trusting the corpus (a primary, direct transcription of the
  published rules) over the one unreliable fetch, consistent with this
  session's own "corpus is the tiebreaker" discipline.
- **Class skills**: `CSKILL:TYPE=Craft|Diplomacy|Heal|Knowledge
  (History)|Knowledge (Planes)|Knowledge (Religion)|TYPE=Profession|
  Sense Motive|Spellcraft` -- includes NONE of Climb/Intimidate/Swim,
  same shape as Wizard/Arcanist (no class-skill-bonus bug to fix here,
  unlike Warpriest/Slayer).
- **Mystery** (`KEY:Oracle ~ <Name> Mystery`, 10 real types: Battle,
  Bone, Flame, Heavens, Life, Lore, Nature, Stone, Waves, Winds) --
  structurally identical to Cleric's domain / Warpriest's Blessing
  choice, gets the same "pick ONE canonical, self-scoped option"
  narrowing. **Life Mystery's own Healing Hands revelation** ("+4 bonus
  on Heal checks... may provide first aid to two people...") is the
  proposed MVP: the core `+4 Heal check bonus` is flat and self-scoped
  (the "treat two people" clause extends to other creatures, not
  modeled, same shape as Slayer's own standalone-fact precedent -- Heal
  isn't among the three skills `compute_selected_skill_modifiers`
  tracks either way, so this grounds as a standalone flat record like
  Trapfinding/Track).
- **Curse** (5 real types: Clouded Vision, Deaf, Haunted, Lame, Wasting)
  -- **Clouded Vision** is the proposed MVP: "cannot see beyond
  `OracleCloudedVisionRange` feet (flat `30`, verified directly against
  the corpus `BONUS:VAR`), but you can see as if you had darkvision" --
  a genuinely self-contained, flat, no-target-creature mechanic (a real
  restriction-plus-benefit pair, not opponent-dependent at all).
- **Orisons/Cure Wounds/Inflict Wounds/Tongues**: the remaining 4 of 19
  real `KEY:Oracle ~ ...` records. Orisons folds into the general
  spellcasting build (same "not separately implemented" reasoning that
  already excluded Arcanist's Cantrips and Warpriest's Orisons from
  their own counts). Cure Wounds/Inflict Wounds (a spontaneous cure/
  harm conversion, mirroring Cleric's own unmodeled spontaneous-
  conversion gap) and Tongues (a language-related Curse benefit) stay
  deferred.

## Proposed scope

1. `is_supported_oracle_single_class` -- exact `ApgClassId::Oracle`
   match, mirroring the six existing ACG/APG gates exactly.
2. Real spontaneous known-spell validation, mirroring
   `unmet_sorcerer_known_spell_conditions`/`sorcerer_spells_known_table`/
   `ground_sorcerer_known_spells`'s own shape exactly (own, independently
   verified per-level table; validated against Cleric's own already-
   built spell list, `cleric_spell_list::CLERIC_SPELL_LIST`, reused
   directly -- the Oracle-specific bonus-spell list portion stays
   explicitly out of scope, named honestly, since a bare/no-Mystery-
   specific-spell posture is a genuinely valid test case).
3. Mystery: recognize a `choice:oracle_mystery` selection naming
   `mystery:life`; if recognized, ground Healing Hands' flat +4 Heal
   bonus (unconditional -- Life Mystery's own revelation is always-on
   once chosen, not activation-gated, unlike Destructive Attacks/Touch
   of Good) and replace the claim-blocking mystery-powers diagnostic
   with a non-blocking note naming the other 9 Mysteries as deferred.
4. Curse: recognize a `choice:oracle_curse` selection naming
   `curse:clouded_vision`; if recognized, ground the flat 30-foot vision
   cap (unconditional) and replace the claim-blocking curse diagnostic
   with a non-blocking note naming the other 4 Curses as deferred.
5. New, narrower `class_feature.apg.oracle.other_features_deferred
   .unsupported` diagnostic naming Cure Wounds/Inflict Wounds
   (spontaneous conversion), Tongues, the other 9 Mysteries, and the
   other 4 Curses.

## What stays explicitly out of scope, named honestly

- The other 9 Mystery types and their own revelations (real mechanical
  variety, named but not built).
- The other 4 Curse types.
- Oracle's own Mystery-granted bonus-spell list portion of
  `SPELLLIST:2|Cleric|Oracle` (the Cleric portion is reused directly;
  the Oracle-specific portion stays deferred).
- Cure Wounds/Inflict Wounds spontaneous conversion (mirrors Cleric's
  own unmodeled gap).
- Tongues.

## Open questions for the lead

1. Is `Life Mystery` / `Healing Hands` the right canonical Mystery+
   Revelation pick, or is there a reason to prefer a different Mystery
   (all 10 are structurally similar chooser-list entries; Life was
   picked because its Revelation is the cleanest flat, self-scoped
   value of the ones checked)?
2. Given Oracle needs the FULL spontaneous known+per-day validation
   shape (mirroring Sorcerer/Bard, not Wizard/Arcanist/Warpriest's
   simpler prepared shape), this is genuinely a bigger single task than
   the last two closures -- worth confirming you're comfortable with
   that scope before I start, or whether splitting spellcasting from
   the Mystery/Curse choices into two separate commits makes more sense
   given the size.
