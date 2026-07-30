# ACG Skald Spellcasting Closure (Risks Item 8) — Scoping Plan

> Directed by the lead after the ACG/APG scan and Monk cycle wrapped, per
> the comparative scoping request: of the 3 partial casters this session
> already gave pillar integration (Skald, Bloodrager, Alchemist), Skald
> is dramatically cheaper to push toward `Computed` because it reuses
> Bard's spell list wholesale (confirmed: zero "Skald" hits in either
> `cr_spells.lst` or `acg_spells.lst`, vs. 202/122 real per-spell
> `CLASSES:` tags for Bloodrager/Alchemist respectively). This doc goes
> one step further than that comparison: Skald's own spells-known AND
> spells-per-day progression tables turned out to be BYTE-IDENTICAL to
> Bard's own already-built tables, not merely similar -- verified against
> two independent primary sources before writing a line of code.

## The core finding: Skald doesn't just share Bard's spell list -- it
## shares Bard's entire numeric progression too

Fetched Skald's own "Spells Known" and "Spells per Day" tables from both
`aonprd.com` and `d20pfsrd.com` independently. Both agree with each
other, and BOTH match this codebase's own already-shipped
`bard_spells_known_table`/`bard_base_spells_per_day` (in
`pilot_compute.rs`) row for row, level for level:

| Level | Spells Known (0/1st/2nd/3rd/4th) | Spells/Day (1st/2nd/3rd/4th) |
|---|---|---|
| 1 | 4/2/-/-/- | 1/-/-/- |
| 2 | 5/3/-/-/- | 2/-/-/- |
| 3 | 6/4/-/-/- | 3/-/-/- |
| 4 | 6/4/2/-/- | 3/1/-/- |
| 5 | 6/4/3/-/- | 4/2/-/- |
| 6 | 6/4/4/-/- | 4/3/-/- |
| 7 | 6/5/4/2/- | 4/3/1/- |
| 8 | 6/5/4/3/- | 4/4/2/- |
| 9 | 6/5/4/4/- | 5/4/3/- |
| 10 | 6/5/5/4/2 | 5/4/3/1 |

This is exactly `bard_spells_known_table`'s and `bard_base_spells_per_day`'s
own existing match arms, unchanged. (One fetch artifact worth naming
honestly: both web sources' own rendered "Spells per Day" table showed a
leading "0" column that didn't match a 0-level entry existing anywhere
-- resolved by recognizing the numbers align exactly with Bard's own
4-column, no-0th-level `bard_base_spells_per_day` table once the header
is read as 1st-4th rather than 0-4th; the underlying digits are
identical either way, and Bard's own table was independently verified
against 2 sources when it was originally built, so this is the
trustworthy anchor.) This is not a coincidence: several PF1 Advanced
Class Guide hybrid classes share this exact "4+2/5+3/6+4/..." spontaneous
spellcasting chart as a common template -- Skald and Bard both use it
verbatim.

## Proposed scope

Mirrors `ground_or_block_bard_known_spells`'s own shape exactly, since
the underlying data and validation logic are the same:

1. **Reuse, don't duplicate, the two pure lookup functions**:
   `bard_spell_level_access(level)` and `bard_spells_known_table(level)`
   are pure, side-effect-free numeric lookups (no explanations/
   diagnostics pushed) -- calling them directly for Skald introduces
   zero risk to Bard's own behavior (unlike the Wolf/Horse case, where
   the shared functions PUSH explanation records and needed parallel
   copies to avoid any chance of touching Druid's/Hunter's own output).
   Recommend calling `bard_spell_level_access`/`bard_spells_known_table`
   directly from Skald's own validation function, not duplicating
   identical match arms under a new name -- the "don't repeat identical
   logic" principle applies cleanly here since there's no shared-mutable-
   state risk to guard against.
2. **Reuse `bard_spell_list::BARD_SPELL_LIST` directly** for spell-list-
   membership validation (Skald's own `SPELLLIST:1|Bard` confirms this is
   the same list, not merely a similar one).
3. **New `unmet_skald_known_spell_conditions`/`ground_skald_known_spells`/
   `ground_or_block_skald_known_spells`** -- these need their own bodies
   (Skald-specific class-id filtering, diagnostic IDs/messages), but the
   internal logic is a straight mirror of Bard's own three-function
   shape: known spells filtered by `SKALD_CLASS_ID`, each checked against
   `BARD_SPELL_LIST` (not a new Skald-specific list), the shared
   `bard_spell_level_access` ceiling, and the shared
   `bard_spells_known_table` per-level cap.
4. **Replace the existing `class_feature.acg.skald.spellcasting_deferred
   .unsupported` diagnostic's scope**: it currently claim-blocks
   spellcasting unconditionally. Once known-spell validation is real,
   this narrows to a genuine validation (mirrors Bard's own
   `class_spell.bard.spontaneous_known_and_per_day.unsupported` shape) --
   a Skald with a valid known-spell posture (including zero known spells,
   a valid PF1 posture per Bard's own precedent) no longer trips it,
   while an invalid one (off-list spell, over spell-level access, over
   the known-count cap) still does.
5. **Base spells-per-day table**: ground the flat per-day counts as
   standalone explanation records too (mirrors Bard's own
   `class_chassis.bard.spontaneous.base_spells_per_day.spell_level_N`
   records) -- these are informational/flat, not part of the known-spell
   validation itself, and reuse the identical numeric table.
6. Tests mirroring Bard's own known-spell-posture test shape (valid
   known-spell selection reaches as far as this slice allows; an off-list
   spell stays blocked; over-access-ceiling stays blocked; over-known-cap
   stays blocked; zero known spells is a valid posture) plus confirming
   Skald's own Inspired Rage closure from the earlier cycle is
   unaffected.

## What stays explicitly out of scope, named honestly

- Actual spontaneous-casting execution (slot consumption, tracking,
  casting resolution) -- out of scope regardless, the same "no
  spell-casting-resolution engine exists anywhere in this codebase"
  boundary every other spellcasting closure this session respects.
- Charisma bonus spell slots (a real PF1 mechanic, not built for Bard
  either -- named the same gap, not newly introduced here).
- Raging Song's own spellcasting-adjacent interactions, if any exist
  beyond Inspired Rage (not found in the corpus read for this doc; would
  need its own check if a future cycle explores Raging Song further).

## Correction: Skald will NOT reach full `Computed` this closure

Initially assumed Skald would reach full `Computed` the way Bard does,
by analogy. Checked directly and this is wrong: Bard reaches `Computed`
not because unbuilt features never block, but because Bard already has
roughly 15 OTHER named features individually grounded from a much
earlier SD13-E5 cycle (predating this v0.6 swarm entirely) --
`class_chassis.bard.bardic_knowledge`, `class_feature.bard.lore_master`,
`jack_of_all_trades`, `well_versed`, `inspire_heroics_*`,
`deadly_performance_dc`, `soothing_performance`, `inspire_competence`,
and more. Known-spell-posture and bardic-performance-execution are just
the LAST two gates closing on an otherwise near-complete class. Skald
has none of that prior investment -- only Inspired Rage is built.
Bardic Knowledge-analog, Iron Will, Rage Powers shared-list access,
Spell Kenning, Versatile Performance, and War Chant remain completely
unbuilt for Skald specifically. **Skald must keep its own "other named
features still missing" diagnostic even after this closure** -- retitled
to drop "spellcasting" from what it claims is missing (since that
becomes real), but still claim-blocking, naming the real remaining gap,
mirroring Brawler's own `other_features_deferred` naming rather than
Bard's absence of one.

## Open questions for the lead

- **Is reusing `bard_spell_level_access`/`bard_spells_known_table`
  directly (rather than writing parallel Skald-named copies) the right
  call**, given the established Wolf/Horse precedent leaned toward
  parallel functions? The stated reasoning above (pure lookups, zero
  shared-mutation risk) is different from the Wolf/Horse case (which
  pushed explanation records), so recommend direct reuse here -- flagging
  for confirmation since it's a related-but-distinct judgment call.
