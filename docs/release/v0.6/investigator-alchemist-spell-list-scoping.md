# Investigator (#8) Spellcasting Subsystem — Shared Alchemist Spell List Scoping

> Task #8's main body: build the shared Alchemist formula spell list
> (unblocking Alchemist's own deferred spellcasting, #4, as a side
> effect) and close Investigator's prepared-extract spellcasting.
> Written up before coding per the established "scope a genuinely new
> subsystem before building it" discipline (matches Arcanist's/
> Warpriest's/Skald's own precedent).

## The spell list — 104 records, corpus-native, no external lookup needed

Extracted directly from `apg_spells.lst` (not hand-transcribed — verified
via a Python pass over the raw file, cross-checked against the file's own
per-level `Alchemist=N` token count):

- 13 genuinely new Alchemist-only spells named directly in `apg_spells.lst`
  (Absorbing Touch, Alchemical Allocation, Amplify Elixir, Bloodhound,
  Bomber's Eye, Delayed Consumption, Elude Time, Fluid Form, Resurgent
  Transformation, Thorn Body, Transmute Potion to Poison, Twin Form,
  Universal Formula).
- 91 `.MOD` records that graft `Alchemist=N` onto existing Core Rulebook
  spells (e.g. `Cure Light Wounds.MOD`, `Haste.MOD`, `Restoration.MOD`).
  The real spell name is the base name with `.MOD` stripped; the level
  Alchemist gets is read directly off the same `.MOD` line's own
  `CLASSES:` token (no cross-file lookup into `cr_spells.lst` needed for
  this purpose — that file would only matter for school/description
  metadata, which this closure's `(name, level)`-only shape mirrors
  `CLERIC_SPELL_LIST`/`BARD_SPELL_LIST` in not needing).
- 0 duplicate names after stripping `.MOD`. Level breakdown: 14/25/20/16/
  14/15 across levels 1-6 (sums to 104, matching the file's own raw
  `Alchemist=N` token count exactly).
- Scoped to `apg_spells.lst` only, not the wider PCGen corpus (which also
  tags `Alchemist=N` on spells in Ultimate Magic/Combat/Wilderness/
  Intrigue/Horror Adventures supplements) -- matches the real PF1 rule
  text itself, which cites "the alchemist formula list (Pathfinder RPG
  *Advanced Player's Guide* 32)" as the canonical source, the same single-
  book-source discipline every other spell list in this codebase already
  uses (Wizard/Cleric/Bard/Druid/Ranger are all CRB-only, not aggregated
  across every later splatbook that adds more spells to their lists).

New module: `src/rules_core/rules_tables/apg/alchemist_spell_list.rs`
(location matches `apg::spell_list`'s own book), `pub const
ALCHEMIST_SPELL_LIST: &[(&str, u8)]` + `pub fn alchemist_spell_level(spell_key:
&str) -> Option<u8>`, mirroring `CLERIC_SPELL_LIST`/`cleric_spell_level`'s
exact shape (the established minimal shape for a spell-list-membership +
level lookup, no school/description needed).

## Investigator's own casting shape

`acg_classes.lst`'s real `CLASS:Investigator` record: `SPELLSTAT:INT
MEMORIZE:YES SPELLBOOK:YES SPELLLIST:1|Alchemist` -- a **prepared** caster
(Wizard/Arcanist/Warpriest shape, not Bard/Sorcerer/Skald/Hunter's
spontaneous shape), reusing the Alchemist list directly (confirmed, not
assumed). No per-level `CAST:`/`KNOWN:` rows exist anywhere in
`acg_classes.lst` for Investigator -- the same external-source caveat
Hunter/Arcanist/Warpriest already had, not a parsing gap.

**Extracts Prepared table, verified via 3 independent sources** (aonprd.com's
own Investigator page, d20pfsrd.com's own Investigator page, and
d20pfsrd.com's own separate Alchemist page -- the last one both a genuine
cross-check of the numbers AND independent confirmation of the "Investigator
extracts-per-day exactly mirrors Alchemist's own table" claim a web search
turned up before I went looking for the raw numbers). All three agree
byte-for-byte:

| Level | 1st | 2nd | 3rd | 4th | 5th | 6th |
|---|---|---|---|---|---|---|
| 1 | 1 | - | - | - | - | - |
| 2 | 2 | - | - | - | - | - |
| 3 | 3 | - | - | - | - | - |
| 4 | 3 | 1 | - | - | - | - |
| 5 | 4 | 2 | - | - | - | - |
| 6 | 4 | 3 | - | - | - | - |
| 7 | 4 | 3 | 1 | - | - | - |
| 8 | 4 | 4 | 2 | - | - | - |
| 9 | 5 | 4 | 3 | - | - | - |
| 10 | 5 | 4 | 3 | 1 | - | - |
| 11 | 5 | 4 | 4 | 2 | - | - |
| 12 | 5 | 5 | 4 | 3 | - | - |
| 13 | 5 | 5 | 4 | 3 | 1 | - |
| 14 | 5 | 5 | 4 | 4 | 2 | - |
| 15 | 5 | 5 | 5 | 4 | 3 | - |
| 16 | 5 | 5 | 5 | 4 | 3 | 1 |
| 17 | 5 | 5 | 5 | 4 | 4 | 2 |
| 18 | 5 | 5 | 5 | 5 | 4 | 3 |
| 19 | 5 | 5 | 5 | 5 | 5 | 4 |
| 20 | 5 | 5 | 5 | 5 | 5 | 5 |

Intelligence bonus extracts stack on top via the same universal PF1
"bonus spells from ability score" table `ability_bonus_spells` already
implements for Arcanist/Wizard/Warpriest -- reused directly, no new table.

## One deliberate deviation from the Wizard/Arcanist precedent, flagged explicitly

Wizard and Arcanist both bound their own prepared-spellbook grounding to
levels 1-3 (`WIZARD_SPELLBOOK_SUPPORTED_MAX_LEVEL`/
`ARCANIST_SPELLBOOK_SUPPORTED_MAX_LEVEL`), an idiom inherited from the
project's original GE-06 deterministic pilot slice (Fighter/Wizard levels
1-3), not a genuine verification limit for Arcanist specifically (its own
doc comment reads "mirrors wizard's own bounded-scope discipline", not
"the formula is unverified past level 3").

**Investigator is different**: its base BAB/save chassis is already fully
wired for all 20 levels (unlike Wizard/Fighter's own originally-bounded
pilot), and I now have the complete, 3-source-verified Extracts Prepared
table for every level 1-20, not just 1-3. Bounding this closure to level 3
would be an arbitrary, unjustified restriction given the data is
genuinely there and genuinely verified -- the "don't fabricate" doctrine
cuts the other way here (grounding all 20 levels isn't fabrication, it's
using real, sourced data fully). **Proposing to ground all 20 levels**,
not bound to 3, unless there's a reason to prefer matching the
Wizard/Arcanist convention that I'm not seeing. Flagging this explicitly
since it's a real precedent deviation, not a silent one.

## Validation shape (mirrors `ground_arcanist_prepared_spellbook`/
## `unmet_arcanist_spellbook_conditions` almost exactly)

Investigator has no arcane-school mechanic (confirmed: no "School" record
in its own `KEY:Investigator ~ ...` list), so like Arcanist, every
prepared extract costs exactly 1 slot -- no opposed-school double-cost
rule to model.

1. New `investigator_base_extracts_per_day(level) -> [Option<i16>; 6]`
   (the table above, `None` past level 6's max extract level or below an
   inaccessible extract level for the given character level).
2. New `parse_investigator_extract_id(spell_id: &str) -> Option<u8>` --
   looks up `alchemist_spell_level` directly (mirrors
   `parse_wizard_spellbook_spell_id`'s own SPELL_LIST-first-then-fallback
   shape, but simpler: no school to resolve, just a level).
3. New `unmet_investigator_extract_conditions`/
   `ground_investigator_prepared_extracts`, mirroring
   `unmet_arcanist_spellbook_conditions`/`ground_arcanist_prepared_spellbook`
   line-for-line: recorded (`AcquisitionMode::Known`) vs. prepared
   (`AcquisitionMode::Prepared`) extracts, every prepared extract already
   recorded, no spell level's prepared count exceeding base+INT-bonus
   slots.
4. Replace `class_feature.acg.investigator.spellcasting_deferred` (folded
   into the existing `other_features_deferred` diagnostic today) with a
   real validation: a valid extract posture (including zero known
   extracts, a valid PF1 posture per Bard's/Arcanist's own precedent)
   stops claim-blocking on spellcasting specifically; the diagnostic
   narrows further to name only what's still missing (Inspiration's
   spend, Investigator Talents, Studied Combat/Strike, etc.).
5. Tests mirroring Arcanist's own shape: valid known+prepared posture,
   an unrecorded prepared extract stays blocked, over-prepared at a
   level stays blocked, a prepared extract targeting an inaccessible
   level stays blocked, zero known extracts is a valid posture, a
   non-Investigator character's spoofed extracts are ignored.

## What stays explicitly out of scope, named honestly

- Actual extract-drinking execution (slot consumption tracking, casting
  resolution) -- out of scope regardless, the same boundary every other
  spellcasting closure respects.
- Investigator's own spell save DC -- real formula exists
  (`10 + extract level + INT modifier`), groundable alongside the rest if
  wanted; proposing to include it as a small addition (mirrors Arcanist's
  own DC grounding), not a separate task.
- Alchemist's own consumption of this shared spell list (task #4) -- the
  list module itself is built here and becomes immediately available,
  but wiring Alchemist's own prepared-extract validation is a separate
  closure, not bundled into this one (Alchemist's own casting shape,
  mutagen interaction, and bomb mechanic are a different class's scope).
- Investigator Talents, Studied Combat/Strike, Inspiration's actual spend
  -- already-deferred, unaffected by this closure.

## Open question for the lead

Ground the full 1-20 Extracts Prepared table (my recommendation, given
it's genuinely verified and Investigator's own chassis already supports
all 20 levels), or bound to levels 1-3 to match Wizard's/Arcanist's own
established convention? Everything else in this scoping is ready to build
either way -- this is the one real judgment call before I start coding.
