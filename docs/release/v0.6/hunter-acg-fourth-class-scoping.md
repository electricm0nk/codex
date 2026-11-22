# ACG Hunter — Fourth APG/ACG Class-Specific Closure (Risks Item 8) — Scoping Plan

> Found during a quick scan of the remaining untouched ACG classes
> (Hunter/Shaman/Swashbuckler) after Brawler's closure, at the lead's
> "your call, no pressure either way" framing. Hunter's 1st-level Animal
> Companion turned out to be mechanically identical to Druid's own
> already-built (and already 3-source-verified) companion system, not a
> new mechanic to design from scratch -- built directly per the lead's
> greenlight ("gate-widening pattern is proven four times now... the one
> substantive question (species/level scope) is a repeat of an
> already-answered one from Druid's own build, not new territory").

## Corpus findings (verified against `acg_classes.lst` / `acg_abilities_class.lst`)

- **Chassis**: 3/4 BAB, good Fortitude/Reflex, poor Will (matches
  `EXPECTED_LEVEL_1`'s `("class:hunter", 0, 2, 2, 0, 8)` row). `SPELLSTAT:
  WIS`, `SPELLLIST:2|Druid|Ranger`, but `KNOWNSPELLS` is restricted to
  "Summon Nature's Ally I" through "VI" only -- a narrower, structurally
  different known-spell list than either Druid's full prepared list or
  Ranger's own spontaneous list, not yet independently verified. Deferred
  along with every other named feature this slice, the same shape as
  Skald's/Bloodrager's own deferred spellcasting.
- **Animal Companion** (`KEY:Hunter ~ Animal Companion`): "At 1st level, a
  hunter forms a bond with an animal companion... The hunter's effective
  druid level is equal to her hunter level." This is not merely similar
  to Druid's own progression -- it explicitly reuses Druid's own
  effective-level mechanism, which is exactly what Druid's own closure
  already built and 3-source-verified (Wolf: Str 13, Dex 15, Con 15,
  Int 2, Wis 12, Cha 6; natural armor +2; Trip from 1st level;
  `wolf_companion_hit_dice`'s own progression formula). Confirmed
  directly by the lead independently before greenlighting.

## Why this is unconditional, unlike Druid's own choice-gated version

Druid's Nature Bond is a genuine PF1 choice: a Druid selects EITHER an
animal companion OR a domain at 1st level, and this codebase's own
`DRUID_NATURE_BOND_CHOICE_ID`/`DRUID_NATURE_BOND_ANIMAL_COMPANION_SELECTION_ID`
gate exists specifically to recognize which one was picked. Hunter has no
such alternative -- the corpus states "At 1st level, a hunter forms a
bond with an animal companion" as a plain fact, never framing it as one
of several options. There is nothing for a `selected_choices` entry to
disambiguate. Consequently, Hunter's own Animal Companion is grounded
purely on class ownership and level -- no `selected_choices` and no
`class_ability_activations` entry needed at all, the same "always on,
no gate" shape Brawler's own AC Bonus already established (though for a
different underlying reason: Brawler has no choice because its bonus is
passive; Hunter has no choice because its companion isn't optional).

**Species is handled exactly as Druid's own was**: the corpus also says
"a hunter may begin play with any of the animals on the druid list" --
worth naming honestly, since this IS a real choice PF1 offers. This
codebase has never modeled a species-selection input for either class
(confirmed by direct grep -- no `companion_species`/similar concept
exists anywhere), so Wolf is assumed as the canonical species for Hunter
too, the same "smallest defensible slice" choice, not a new gap this
closure introduces.

## Reachability note (per the lead's explicit ask, corrected after checking)

The lead's initial framing suggested this closure would land in the same
`headless-only` reachability bucket frontend confirmed for Druid's own
Nature Bond (no creation-UI picker exists for the bond-type choice).
**Checked directly and this is NOT quite the same bucket**: Druid's own
reachability gap is specifically about the missing UI for the animal-
companion-vs-domain CHOICE -- a real decision with no picker. Hunter has
no such choice to expose a picker for at all (the companion is automatic,
not opted into), so there is no missing-picker gap analogous to Druid's
own for the companion-vs-domain decision. The one real choice Hunter
shares with Druid -- WHICH SPECIES -- was never exposed via any picker
for Druid either (Wolf was simply assumed), so this closure doesn't
introduce a NEW missing-picker gap beyond what Druid's own build already
carried forward silently. Net: Hunter's Animal Companion sits closer to
Brawler's "always-on, no gate" reachability bucket than to Sorcerer/
Cleric/Druid's own choice-gated bucket -- worth stating precisely rather
than assuming the analogy holds by default.

## Proposed scope (built)

1. `is_supported_hunter_single_class` -- exact `AcgClassId::Hunter`
   match, mirroring the other three gates exactly.
2. Extracted `ground_wolf_companion_stat_block` and
   `ground_wolf_companion_link_and_share_spells_vacuous` from Druid's own
   inline implementation (byte-identical output for Druid, verified by
   Druid's existing 15-test suite passing unchanged after the refactor)
   so Hunter's own call site reuses the exact math rather than
   re-deriving or copy-pasting it.
3. `ground_hunter_animal_companion_and_defer_the_rest` -- grounds the
   companion unconditionally at Hunter level 1, plus Hunter's own
   advancement-absent diagnostic (non-blocking, mirroring Druid's) and
   the new, narrower `class_feature.acg.hunter.spellcasting_deferred
   .unsupported` diagnostic (naming spellcasting plus Hunter's other
   remaining named features: Animal Focus, Nature Training, Wild
   Empathy, Precise Companion), replacing the generic
   `class_feature.acg.hunter.unsupported` diagnostic for Hunter
   specifically.
4. No pillar integration needed at all (unlike every other class this
   session) -- the companion is a wholly separate creature, exactly like
   Druid's own; `compute_total_saves`/`compute_combat_baseline` need no
   new term.
5. Tests: dedicated diagnostic-swap test (verifying the reused companion
   HP value matches Druid's own math exactly), positive-leak test,
   negative-leak test reapplied (now "the other six").

## What stays explicitly out of scope, named honestly

- Hunter's own spellcasting (Summon Nature's Ally-restricted known-spell
  list, structurally distinct from both Druid's and Ranger's own tables,
  needing independent verification before any build).
- Animal Focus, Nature Training, Wild Empathy, Precise Companion, and
  every other named Hunter feature beyond the companion.
- Companion advancement past level 1 -- identical deferral to Druid's own.
