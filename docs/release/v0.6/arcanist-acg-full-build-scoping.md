# APG/ACG Arcanist — Full Class Build Scoping (First Non-CRB Class Attempting `Computed`)

> Directed by the lead after the full-class-build comparison picked
> Arcanist as the cheapest target: this is the actual build scoping doc,
> same rigor as the original CRB Cleric/Druid/Sorcerer builds, not a
> quick single-ability slice. Arcanist is an **ACG** class (not APG --
> correcting my own loose phrasing in the comparison doc, which didn't
> matter for the comparison itself but matters here for the real gate
> placement).

## Correction to the comparison doc's own framing (verify-before-build, caught here)

The comparison doc characterized Arcanist's `SPELLLIST:1|Wizard` reuse
as "the cleanest spell-list-and-shape match this session" without
checking the actual per-day numbers. Checked now, against two
independent sources:

- **PCGen corpus** (`acg_abilities_class.lst`'s own `Arcanist Spells
  Prepared` record): `ArcanistPreparedLVL_0 = 4+(CastingLVL>=2)+(>=4)+
  (>=6)+(>=8)+(>=10)`, `ArcanistPreparedLVL_1 = 2+(>=3)+(>=5)+(>=7)`,
  `ArcanistPreparedLVL_2 = (>=4)+(>=5)+(>=7)+(>=9)+(>=11)`, etc. At
  level 1: 4 cantrips, 2 first-level spells, 0 second-level (not yet
  accessible).
- **legacy.aonprd.com's own "Table: Arcanist Spells Prepared"**
  (fetched directly, cross-checked against a separate web search that
  independently stated "at 1st level, an arcanist can prepare four
  0-level spells and two 1st-level spells"): levels 1-5 read `4/2/-/-`,
  `5/2/-/-`, `5/3/-/-`, `6/3/1/-`, `6/4/2/-` -- matches the corpus
  formula exactly at every level checked (verified by hand-evaluating
  the formula at CastingLVL 1-5).

**This does NOT match Wizard's own table** (`wizard_base_spells_per_day`:
level 1 = `3/1/-/-`, level 2 = `4/2/-/-`, level 3 = `4/2/1/-` --
verified in `pilot_compute.rs:19457-19464`). Arcanist prepares MORE
spells per day at every level, and its 2nd-level access ceiling is
level 4, not level 3 like Wizard's. **The spell-LIST content and casting
SHAPE (prepared, spellbook-gated, no known-spells cap) are genuinely
shared with Wizard; the per-day NUMBERS and access ladder are NOT --
Arcanist needs its own real table, built and verified independently,
the same as Bloodrager/Alchemist needed despite conceptual similarity to
other precedents.** This is a real correction to the comparison doc's
framing, not a disqualifying finding -- Arcanist is still the cheapest
target (list reuse cuts real per-spell corpus-content verification work
even though the numeric table needs its own build), just not quite as
free as first framed.

One simplification Arcanist genuinely has that Wizard does NOT: no
arcane-school/specialization mechanic exists in Arcanist's own KEY list
at all (confirmed: Arcane Reservoir, Arcanist Exploits, Cantrips, Class
Skills, Consume Spells, Greater Arcanist Exploits, Magical Supremacy,
Spells Prepared, Weapon and Armor Proficiency -- no "School" record
anywhere). Arcanist's own prepared-spellbook validation would need NO
opposed-school gate at all, unlike `unmet_wizard_spellbook_conditions`'s
own `wizard_has_canonical_specialization_selections` check
(`pilot_compute.rs:19572-19579`) -- genuinely simpler than Wizard's own
validation in this one respect.

## Proposed scope

1. **Gate widening**: `is_supported_arcanist_single_class` -- exact
   `AcgClassId::Arcanist` match, mirroring Skald's/Bloodrager's/
   Brawler's/Hunter's own ACG gates exactly (`pilot_compute.rs:7420-
   7488`'s existing four). Added to `has_supported_class_chassis`. No
   separate gate-ordering/hoisting fix needed -- confirmed directly
   against `compute_acg_class_chassis`'s own doc comment
   (`pilot_compute.rs:8605-8619`): this function is only ever called
   from `compute_class_chassis`'s single-class-only section, and
   `AcgClassId::from_class_id_str` stays deliberately unregistered with
   `multiclass_class_level_supported`, the same as every prior ACG
   closure.
2. **Arcane Reservoir** (flat pool, no choice): `ARCANIST_RESERVOIR_MAX
   (level) -> i16` (`3 + level`), `ARCANIST_RESERVOIR_DAILY_FILL(level)
   -> i16` (`3 + level/2`). No `class_ability_activations` entry needed
   at all -- this is a passive daily resource, not an activation-gated
   posture like Rage/Mutagen/Judgment. Grounds as a flat explanation
   record, mirroring Cleric's own `channel_energy_dice`/
   `channel_energy_uses_per_day`'s "flat, no gate" shape.
3. **Arcanist spellcasting** (the bulk of the work):
   - `arcanist_spell_level_access(level) -> i16` (own access ladder:
     1st at level 1, 2nd at level 4, 3rd at level 7, 4th at level 10,
     5th at level 13, 6th at level 16, 7th at level 17, 8th at level 18,
     9th at level 19 -- verified above).
   - `arcanist_base_spells_per_day(level) -> [Option<i16>; 4]` (own
     table, bounded to the same bootstrap scope Wizard's own build used
     -- `WIZARD_SPELLBOOK_SUPPORTED_MAX_LEVEL = 3` -- so Arcanist levels
     1-3 first: `[4,2,None,None]`, `[5,2,None,None]`, `[5,3,None,None]`,
     widened later the same incremental way every other per-level table
     in this codebase already is).
   - Reuses `SPELL_LIST`/Wizard's own spell-list content directly (the
     genuine reuse) for the prepared-spellbook validation, mirroring
     `parse_wizard_spellbook_spell_id`'s own resolution logic --
     confirm during build whether a direct function reuse (like Skald's
     direct reuse of `bard_spell_level_access`) is safe here, or whether
     Arcanist's own access ladder being genuinely different requires a
     parallel (not shared) function -- likely parallel, since the
     ACCESS ladder differs even though the LIST CONTENT is shared.
   - `unmet_arcanist_spellbook_conditions` mirroring
     `unmet_wizard_spellbook_conditions`'s shape MINUS the school-
     specialization check (Arcanist has none).
4. **Exploits deferral**: new `class_feature.acg.arcanist.exploits_deferred
   .unsupported` diagnostic (mirroring Cavalier's/Alchemist's own
   diagnostic-narrowing pattern) naming Arcanist Exploits, Greater
   Arcanist Exploits, Consume Spells, and Magical Supremacy as the
   genuinely still-missing named features -- a chooser-list (Exploits)
   plus two lower-priority capstone-shaped abilities, all deliberately
   out of scope for this slice.
5. **Class-skill list**: 8 skills (`CSKILL:Appraise|TYPE=Craft|Fly|
   TYPE=Knowledge|Linguistics|TYPE=Profession|Spellcraft|Use Magic
   Device`, verified directly against the corpus's own `Class Skills`
   record) -- wire into `compute_selected_skill_modifiers`'s existing
   `has_supported_class_chassis`-gated dispatch, mirroring how Rogue's/
   Wizard's own class-skill sets are already wired.

## Whether this reaches `Computed`

This is the first APG/ACG closure with a real chance, but not for free:
mirroring Wizard's own bootstrap-deadlock history, an EMPTY prepared
spellbook (zero spells recorded) is very likely a genuine posture
violation for Arcanist too, the same reasoning Wizard's own
`unmet_wizard_spellbook_conditions` uses (`recorded.is_empty()` blocks).
Wizard's own path to `Computed` needed a SEPARATE follow-on cycle
(`compose_character_input`'s canonical starter-spell seed, this
session's own "bootstrap-deadlock fix") after the engine build itself
landed. Expect the same shape here: the ENGINE build (this scope) likely
lands Arcanist at `Blocked` on the new, narrower `exploits_deferred`
diagnostic alone for a bare/no-spells posture (mirroring Cavalier's/
Alchemist's own "stays Blocked" shape), with a REAL, valid,
`Computed`-reaching posture achievable only once a `class_ability_activations`-
free but spell-seeded fixture is constructed (proving the pillar
reachable) -- and a genuine product-reachability path needs its own
Path-A-style `compose_character_input` seed as a distinct follow-on,
the same two-cycle shape Wizard's own history had. Naming this
explicitly now rather than assuming a single build cycle reaches full
product-visible `Computed`, matching this session's own "don't conflate
the engine closing with the product being reachable" discipline (the
same distinction the choice-picker Path A work formalized for Sorcerer/
Cleric/Druid).

## What stays explicitly out of scope, named honestly

- Arcanist Exploits / Greater Arcanist Exploits (chooser-list, real
  variety -- named but not built).
- Consume Spells (real formula, Charisma-modifier-gated resource
  conversion) / Magical Supremacy (capstone) -- lower priority than the
  core chassis/skills/spellcasting/Reservoir scope, a real follow-on,
  not blocking this slice.
- Levels above 3 for the spells-per-day table, mirroring Wizard's own
  bounded `WIZARD_SPELLBOOK_SUPPORTED_MAX_LEVEL = 3` scope exactly --
  widened later the same incremental way, not attempted in one slice.
- The product-reachability (`compose_character_input` seeding) question
  -- a distinct, separate follow-on cycle, not assumed free.

## Open question for the lead

Given the doc comment on `compute_acg_class_chassis` (built for Skald/
Bloodrager/Brawler/Hunter) already anticipates "no separate gate-
ordering/hoisting fix... the way CRB classes required," I don't expect
a novel architectural risk on the gate-widening piece itself. The one
piece I'd flag for your own judgment on review depth: whether reusing
`SPELL_LIST`'s Wizard-tagged entries directly for Arcanist's own
prepared-spellbook validation (content reuse) while building a fully
separate, real per-day/access-ladder table (no reuse) is the right
combination to attempt in one slice, or whether the spellbook-validation
piece alone is big enough to warrant its own follow-on cycle after
Reservoir + chassis + skills land first. Your call on sequencing.
