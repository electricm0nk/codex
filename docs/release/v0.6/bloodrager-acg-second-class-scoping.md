# ACG Bloodrager — Second APG/ACG Class-Specific Closure (Risks Item 8) — Scoping Plan

> Proposed to the lead immediately after Skald's closure landed (d2eb0798).
> Unlike Skald, this class needed no novel architectural question at all --
> the chassis-integration gate widening pattern is already twice-proven
> (Barbarian's original review, Skald's re-confirmation), and Bloodrage
> itself turns out to be a near-exact mechanical clone of Barbarian's own
> Rage. Treated as a repeat of an already-reviewed pattern, per the lead's
> own "no second review needed on a pattern-setter" standing guidance --
> built directly, this doc written for the record.

## Corpus findings (verified directly against `acg_classes.lst` /
## `acg_abilities_class.lst`, PCGen corpus commit
## `7f818006e371188e5717fd18d74d18a420747fc6`)

- **Chassis**: full BAB (`classlevel`), good Fortitude
  (`classlevel/2+2`), poor Reflex/Will (`classlevel/3`) -- one of the four
  real full-BAB ACG classes, matching `pilot_compute.rs`'s own
  `EXPECTED_LEVEL_1` reference row (`("class:bloodrager", 1, 2, 0, 0, 10)`).
  `SPELLSTAT:CHA`, casts from its own `SPELLLIST:1|Bloodrager` (not the
  Bard list Skald uses) -- spellcasting stays deferred, same as every
  other caster class's own first cycle.
- **Bloodrage** (`KEY:Bloodrager ~ Bloodrage`): +4 Strength / +4
  Constitution morale bonus, +2 Will morale bonus, -2 Armor Class penalty
  at base tier -- the exact same four values as Barbarian's own base Rage.
  Rounds-per-day: `2 + CON modifier + 2*BloodragerLVL` where
  `BloodragerLVL` is the character's own Bloodrager level (not
  level-minus-one) -- algebraically identical to Barbarian's
  `4 + CON modifier + 2*(level-1)` (both reduce to `2 + CON + 2*level`).
  Corpus text states outright: "Bloodrage counts as the barbarian's rage
  class feature for the purpose of feat prerequisites, feat abilities,
  magic item abilities, and spell effects" -- the clearest possible
  confirmation this is the same mechanic, not merely similar.
- **Greater Bloodrage** (11th level): +2/+2/+1 further morale bonus (base
  4/4/2 -> 6/6/3) -- identical threshold level AND identical magnitude
  jump to Barbarian's own Greater Rage
  (`BARBARIAN_GREATER_RAGE_LEVEL: u8 = 11`, 4/4/2 -> 6/6/3).
- **Mighty Bloodrage** (20th level): another +2/+2/+1 (6/6/3 -> 8/8/4) --
  identical threshold AND magnitude to Barbarian's own Mighty Rage
  (`BARBARIAN_MIGHTY_RAGE_LEVEL: u8 = 20`, 6/6/3 -> 8/8/4).
- **Tireless Bloodrage** (17th level): fatigue immunity -- identical
  threshold to Barbarian's own Tireless Rage
  (`BARBARIAN_TIRELESS_RAGE_LEVEL: u8 = 17`).
- **Self-application**: Bloodrage is explicitly self-only by RAW ("A
  bloodrager can enter a bloodrage...he gains...") -- no exception-clause
  inference needed the way Skald's Inspired Rage required; this removes
  the one genuinely novel piece of Skald's own build.
- **Not reused this slice, named honestly**: Greater Bloodrage's
  11th-level "apply a known 2nd-level-or-lower touch/personal spell to
  himself" clause requires real spellcasting resolution (which this
  codebase has none of, for any class) -- deferred alongside spellcasting
  generally, the same "opponent/engine-dependent, not vacuous" shape as
  every other spell-execution gap this session has named rather than
  built around.

## Proposed scope (mirrors Skald's own build almost exactly)

1. Reuse `is_supported_skald_single_class`'s exact shape for a new
   `is_supported_bloodrager_single_class`, gated on
   `AcgClassId::Bloodrager` specifically (same exact-match discipline,
   not a broad `.is_some()`).
2. `ground_or_block_bloodrager_bloodrage`, `bloodrager_bloodrage_rounds_per_day`,
   `bloodrager_bloodrage_tier`, `active_bloodrager_bloodrage_bonus`,
   `apply_bloodrager_bloodrage_ability_bonuses` -- literal structural
   copies of the Barbarian Rage functions (four-value tier, rounds budget,
   fatigue-not-modeled honesty note below `BARBARIAN_TIRELESS_RAGE_LEVEL`-
   equivalent), reusing the same three shared level constants
   (`BARBARIAN_GREATER_RAGE_LEVEL`/`BARBARIAN_MIGHTY_RAGE_LEVEL`/
   `BARBARIAN_TIRELESS_RAGE_LEVEL`) directly rather than duplicating them,
   since the thresholds are independently verified identical above.
3. New, narrower `class_feature.acg.bloodrager.spellcasting_deferred.unsupported`
   diagnostic replacing the generic one for Bloodrager only, mirroring
   Skald's own diagnostic-honesty fix exactly.
4. Same test shape as Skald's `skald_dispatch_widening_safety_tests`
   (not-raging, actively-raging-in-budget with real values, over-budget,
   non-Bloodrager spoof-ignored), plus the same negative-leak coverage
   (carve Bloodrager out of the all-10-classes loop in both
   `pilot_compute.rs` and `tests/sd24_acg_class_coverage_audit.rs`, update
   `named_features_wired` to 2 total, i.e. Skald's 1 + Bloodrager's 1).

## Why no separate review pass

The one piece of Skald's build that genuinely needed independent review
(the exact-match chassis-gate widening, to guard against silently
admitting all 10 ACG classes) is not a new question here -- it is the
identical gate-widening code shape, applied to one more class, with its
own dedicated negative-leak test proving the same discrimination property
per class. The one piece that was genuinely novel to Skald (the
self-application evidentiary question) does not arise for Bloodrager at
all, since Bloodrage is self-only by RAW with no exception-clause
inference required. Flagging this reasoning explicitly rather than
silently skipping review -- happy to take a review pass anyway if the
lead disagrees this qualifies as a repeat pattern.
