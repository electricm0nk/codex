# ACG Skald — First APG/ACG Class-Specific Closure (Risks Item 8) — Scoping Plan

> Requested by the lead after Bard's closure, as a first look into the
> completely untouched APG/ACG bucket (16 classes, nothing beyond generic
> BAB/save/HD dispatch built for any of them). This doc sizes Skald's
> Inspired Rage as the entry point: a genuine, well-verified early win in
> the same vein as Sorcerer's Arcane Bond, but one that also requires a
> real, first-of-its-kind architectural extension (recognizing an ACG
> class in the shared "supported class chassis" gate at all) — flagging
> that prerequisite explicitly rather than treating it as free.

## Current state, confirmed by direct investigation (not assumed)

- `compute_apg_class_chassis`/`compute_acg_class_chassis`
  (`pilot_compute.rs:7092-7260`) are the ONLY things built for all 16 APG/
  ACG classes: each emits exactly 4 explanations (base attack bonus,
  three base saves) from a shared table, then unconditionally pushes its
  own `class_feature.{apg,acg}.<class>.unsupported` diagnostic. Zero
  class-specific content (spell lists, class features, choice
  recognition) exists for any of the 16 — confirmed by grep, zero hits for
  every one of the 16 classes' own id prefixes anywhere in
  `pilot_compute.rs`.
- **Critical prerequisite finding**: `has_supported_class_chassis`
  (`pilot_compute.rs:6828`) — the gate every integrated pillar
  (`compute_total_saves`, `compute_combat_baseline`,
  `compute_selected_skill_modifiers`) actually depends on — only checks
  Fighter, Wizard, and `is_supported_generic_single_class`, which itself
  only calls `multiclass_class_level_supported`, which is `table_class_id`
  (CRB) only. **APG and ACG classes are not recognized by this gate at
  all today**, regardless of anything class-specific being built. This is
  a genuinely new architectural extension, not something "table_class_id
  widening" already covers — the first time this session extends the
  integration gate beyond CRB.

## Why Skald specifically

PF1's Advanced Class Guide hybrid classes each combine two CRB classes'
mechanics. Skald (Bard + Barbarian) turned out to share DNA with two
mechanisms already built this session:

- **Raging Song** is explicitly "the bard's bardic performance special
  ability for any effect that affects bardic performances" (verified via
  the PCGen corpus `acg_abilities_class.lst`, `TYPE:...Bardic Performance`)
  — same rounds-per-day shape as Bard's own Bardic Performance (3 + Cha
  modifier at 1st level, +2 per level after, per the corpus DESC text),
  just a different base (3, not 4).
- **Inspired Rage** (Raging Song's 1st-level song type) grants the
  affected target +STR/+CON morale, +Will morale, and an AC penalty —
  the same four-value shape as Barbarian's own Rage. Verified directly
  against the PCGen corpus DESC text and `BONUS:VAR` formulas: STR/CON
  bonus `2 + floor(level/8)*2` (+2 at 1-7, +4 at 8-15, +6 at 16+), Will
  bonus `1 + floor(level/4)` (+1 at 1-3, rising every 4 levels), AC
  penalty a flat -1 (not tiered, unlike the STR/CON/Will values).
- **Critically, the corpus text itself resolves the "does this affect the
  skald personally" question**: the Inspired Rage description reads
  "allies **other than the skald** cannot use any Charisma-, Dexterity-,
  or Intelligence-based skills..." — carving out an explicit exception
  FOR the skald specifically implies she IS among the affected targets by
  default (there would be no reason to name her exception to a rule that
  doesn't apply to her at all). This is NOT the same "raw-secondary-use"
  narrowing Touch of Good needed — self-application here is the ability's
  own default behavior, not a scoped-down alternate use, since a skald
  raging alongside her allies is the ability's ordinary use case, not an
  edge case. Worth independent confirmation before coding regardless,
  since this reading rests on one corpus sentence's implication rather
  than an explicit "including the skald" clause.
- **Skald's own class chassis** (`acg_classes.lst`, `CLASS:Skald`):
  3/4 BAB, good Fortitude AND Will, poor Reflex — verified DIFFERENT from
  Bard's own save shape (poor Fort, good Ref, good Will) and DIFFERENT
  from Barbarian's (good Fort only) — not assumed from either.
- **Skald casts from the Bard spell list** (`SPELLLIST:1|Bard` in the
  corpus) — meaning `bard_spell_list::BARD_SPELL_LIST` (already built and
  corpus-verified this session) could be reused directly for a future
  Skald known-spell validation slice, once Skald's own spells-known/
  per-day table numbers are separately verified (Skald is very likely a
  4-column partial spontaneous caster like Bard, not confirmed yet).

## Proposed scope for this FIRST slice (deliberately narrow)

Mirrors exactly how Barbarian's own first cycle was scoped (chassis
widening + Rage only, spellcasting classes' spell math landed in later,
separate cycles):

1. **Widen the chassis-integration gate for Skald specifically** (not all
   16 APG/ACG classes at once — the same "one class at a time" discipline
   `table_class_id` used for CRB): add an `AcgClassId::from_class_id_str`
   check to `is_supported_generic_single_class` (or a parallel helper),
   gated to exactly Skald, mirroring how each CRB class was individually
   added to `table_class_id` rather than widening to all 11 in one pass.
   This is the genuinely new architectural piece -- recommend explicit
   review given it's a first-of-its-kind extension, not a repeat of an
   already-reviewed pattern.
2. **Hoist a new `ground_or_block_skald_inspired_rage`** function, mirroring
   `ground_or_block_barbarian_rage` structurally: class-ownership-gated
   (Skald only), `ClassAbilityActivation` with a new `ability_id` (e.g.
   `"inspired_rage"`), self-application only (per the corpus finding
   above), rounds-per-day validation (claim-blocking if over budget, same
   as every other activation this session), non-blocking note for
   anything left unmodeled.
3. **Pillar integration**: apply_skald_inspired_rage_bonuses-equivalent
   layered onto `compute_ability_modifiers`'s output (STR/CON), plus
   Will-save and AC-penalty layering into `compute_total_saves`/
   `compute_combat_baseline` -- literally the same three call sites
   Barbarian's Rage already touches, extended with one more `unwrap_or(0)`
   term each, the same mechanical shape as adding Bard/Cleric's own terms
   alongside Barbarian's.
4. **Explicitly defer Skald's own spellcasting** (a real, separate,
   permanent diagnostic naming it as out of scope for this slice) --
   mirrors exactly how Barbarian's own first cycle didn't need this (not
   a caster) and how every caster class's spell math was its own later
   cycle, not bundled with the class's first non-spell feature.
5. New tests mirroring Barbarian's `barbarian_dispatch_widening_safety_tests`
   shape: not-raging reaches as-far-as-this-slice-allows (still Blocked on
   the deferred spellcasting diagnostic, unlike Barbarian which reached
   full Computed), actively-raging-in-budget applies the real STR/CON/
   Will/AC values, over-budget claim-blocks, class-ownership gate holds
   for non-Skald characters, and the NEW chassis-integration gate doesn't
   silently admit some OTHER unrelated APG/ACG class.

## Open questions for the lead / adversarial review

- **The chassis-integration gate widening is the one piece of this slice
  without a directly-reviewed precedent this session** (CRB's
  `table_class_id` widening was reviewed once, early, and reused
  ever since; this would be the first time the SAME KIND of gate is
  extended to a completely different class family). Recommend a review
  pass specifically on this piece, even though the Rage-mirroring pillar
  work itself is now a well-proven, un-reviewed-again pattern.
- **Self-application default vs. narrowing**: flagging for a second
  opinion whether the corpus's "other than the skald" phrasing is
  suffficient evidence that Inspired Rage defaults to including the
  skald, or whether this needs the same explicit "self-application only,
  named honestly" framing Touch of Good required (leaning toward: this
  one doesn't need the disclaimer, since it's the ability's actual default
  behavior, not a narrowed alternate use — but wanted this checked before
  writing the explanation text either way).
- **Scope of the spell-list reuse**: not attempting Skald's own spell
  math this slice at all (deferred, named honestly) -- confirming that's
  the right call given Bard's own numbers can't be assumed to transfer
  (different class, even sharing a spell list) without independent
  verification of Skald's specific spells-known/per-day table.

## Revision after adversarial review (2026-07-25)

The lead's review found two real gaps in the plan above -- confirmed
directly by re-reading the actual code -- plus a correction to the
self-application framing. Recorded here before any code, per the lead's
request.

**1. The chassis-gate widening must use an exact class match, not
`AcgClassId::from_class_id_str(...).is_some()`**: verified directly
(`rules_tables/acg/mod.rs:123-125`) that `from_class_id_str` searches
`Self::ALL` (all 10 ACG classes) and resolves to `Some` for ANY of
them -- Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman,
Skald, Slayer, Swashbuckler, Warpriest. A naive `.is_some()` check in the
new gate would silently admit all 10 into real
save/combat/skill-modifier computation, not just Skald -- the exact
Ranger-review class of over-widening bug, and one existing tests
wouldn't catch (they only assert "stays Blocked," not "produces zero
pillar output"). The gate must check
`AcgClassId::from_class_id_str(&class_level.class_id) ==
Some(AcgClassId::Skald)` explicitly. A new positive test is required:
the other 9 ACG classes must produce ZERO
`defense.total_save.*`/`combat.baseline_*`/`skill.selected_modifier.*`
explanations at level 1, not merely "stay Blocked" (which their existing
test already checks, but doesn't rule out a silent pillar-output leak).

**2. The existing `class_feature.acg.skald.unsupported` diagnostic and
its own test both need an explicit resolution, not left implicit**: the
pre-existing `all_ten_acg_classes_stay_blocked_with_the_real_unconditional_diagnostic`
test (`pilot_compute.rs:23335`) loops all 10 ACG classes and asserts each
gets `class_feature.acg.{name}.unsupported` unconditionally, with a
message claiming "no named class-feature computation... is grounded
anywhere." That claim becomes FALSE for Skald once Inspired Rage is
real -- keeping the same diagnostic id/message for Skald would be
dishonest, mirroring exactly why Sorcerer's own generic bloodline
diagnostic was narrowed rather than left as-is once Arcane Bond
resolved. Resolution: retire `class_feature.acg.skald.unsupported`
entirely for Skald (the other 9 classes keep it, untouched) and replace
it with a new, narrower `class_feature.acg.skald.spellcasting_deferred.unsupported`
diagnostic naming ONLY the genuinely still-missing piece (spellcasting
and the other named-but-unbuilt class features), the same
"narrow the message to what's actually still true" pattern used for
Sorcerer/Bard. The existing all-10-classes test must carve Skald out of
its loop (asserting the OTHER 9 unchanged) and gain a dedicated
Skald-specific assertion pair: not-raging still carries the new
narrower diagnostic (spellcasting still deferred), while the OLD generic
id never appears for Skald at any raging state.

**3. Self-application needs a stronger, not weaker, honesty note**:
corrected framing per the review -- Touch of Good's self-targeting was
RAW-explicit (unambiguously legal per the rules text itself), so its
disclaimer was about SIGNIFICANCE (a narrow/secondary use), not
legality. Here, self-application itself is inferred from a single
negative clause ("allies other than the skald") with no explicit
"including yourself" language anywhere in the source text (confirmed:
Bard's own Bardic Performance/Inspire Courage text explicitly says
"including yourself if desired" / "including yourself" -- Skald's
Raging Song/Inspired Rage text never uses that phrasing at all). This is
weaker evidence, not stronger, and needs a correspondingly more explicit
disclaimer in the grounded explanation text: "self-application is
modeled by inference from the 'allies other than the skald' exception
clause; the corpus carries no explicit self-inclusion language here,
unlike Bard's 'including yourself.'" Adopting self-application as the
grounded behavior (the inference is mechanically reasonable -- the
skill-restriction exception only matters if the skald herself is
affected), but stating the weaker evidentiary basis honestly rather than
implying it's as settled as Touch of Good's was.

**Net revised scope**: exact `AcgClassId::Skald` match (not a broad
ACG-family check) + a new negative-leak test for the other 9 classes +
a new, narrower `spellcasting_deferred` diagnostic replacing the generic
one for Skald specifically (with the all-10-classes test updated to
carve Skald out) + the corrected, more honest self-application framing
in the explanation text. Otherwise unchanged from the original plan
(Inspired Rage's pillar integration, rounds-per-day validation, and the
deferred spellcasting scope).
