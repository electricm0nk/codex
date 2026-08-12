# ACG Brawler — Third APG/ACG Class-Specific Closure (Risks Item 8) — Scoping Plan

> Proposed after the lead's own check-in flagged Brawler and Warpriest as
> plausible next candidates; independent scan already ruled out
> Brawler's own Martial Flexibility (needs arbitrary-feat-prerequisite
> modeling, genuinely complex) and Warpriest's Blessings/Fervor (Domain-
> shaped choice + real healing math, comparable effort to Cleric's own
> build) before this doc was written. Brawler's OWN "AC Bonus" feature
> turned out to be a clean, structurally NEW (no activation gate, no
> choice gate at all -- a pure function of level + armor posture) but
> low-risk win. Built directly per the lead's "your call, no pressure"
> framing; this doc written for the record and to flag the one genuinely
> new wrinkle (a provably-vacuous armor-type precondition) explicitly.

## Corpus findings (verified against `acg_classes.lst` / `acg_abilities_class.lst`)

- **Chassis**: full BAB, good Fortitude/Reflex, poor Will -- one of the
  four real full-BAB ACG classes (matches `EXPECTED_LEVEL_1`'s
  `("class:brawler", 1, 2, 2, 0, 10)` row). `ROLE:None`, no `SPELLSTAT` --
  Brawler is a pure martial class with **zero spellcasting** at all,
  unlike Skald/Bloodrager (both partial casters). This means Brawler's
  own permanent "still missing" diagnostic cannot be named
  `spellcasting_deferred` -- named `other_features_deferred` instead,
  listing the real remaining named-feature bucket (Brawler's Flurry,
  Knockout, Martial Flexibility, Awesome Blow, Improved Awesome Blow,
  Brawler's Cunning, Martial Training, Bonus Feats, Close Weapon Mastery,
  Brawler's Strike, Maneuver Training -- 11 of Brawler's 14 corpus-counted
  features, once AC Bonus is wired).
- **AC Bonus** (`KEY:Brawler ~ AC Bonus`): "When wearing light or no
  armor, a brawler adds %1 AC as a dodge bonus to her Armor Class. If a
  brawler is helpless or immobilized, she loses this bonus." Corpus
  formula: `ACProgression = (level>3) + (level>8) + (level>12) +
  (level>17)` -- i.e. +0 at levels 1-3, +1 at 4-8, +2 at 9-12, +3 at
  13-17, +4 at 18-20. A pure function of level with NO activation state
  and NO choice -- structurally simpler than every other class built this
  session (Barbarian/Skald/Bloodrager all needed a
  `class_ability_activations` entry; Sorcerer/Cleric/Druid needed a
  `selected_choices` entry).

## Why the armor-type precondition is provably vacuous, not merely unmodeled

The corpus condition is `!PREEQUIP:1,ARMORTYPE=Medium,ARMORTYPE=Heavy` --
"not wearing Medium or Heavy armor." This codebase has **no generalized
armor-type representation at all** -- confirmed by direct grep: the only
armor item this codebase's `CharacterInput`/`pilot_compute.rs` can ever
express is Chain Shirt (`CHAIN_SHIRT_ITEM_ID`), which is itself light
armor. There is no Medium- or Heavy-armor item id anywhere in this repo
for any class to equip. This is the same "provably zero, not merely
unmodeled" shape Sorcerer's Arcane Bond casting-precondition and Druid's
Link/Share Spells needed: the precondition depends on a property of the
character's OWN fixed input (what armor they're wearing) that this
codebase CAN represent, and is provably always satisfied (never Medium/
Heavy) for every input this bounded slice can construct -- not an
opponent-controlled or third-party event. Additionally, the shared
combat-baseline posture gate (`unmet_combat_posture_conditions`) already
requires Chain Shirt `EquippedActive` unconditionally for every class
reaching `compute_combat_baseline` via this fixture, so the "or no
armor" branch never exercises in this bounded slice either -- only the
"light armor" branch does, and it is always true.

**Helpless/immobilized**: this codebase has no transient combat-state
representation at all (confirmed by grep -- no "helpless"/"immobilized"
concept anywhere), the same category as Barbarian Rage's own post-rage
Fatigue gap. Named honestly as a non-blocking, not-modeled diagnostic,
mirroring `class_feature.barbarian.rage_execution.fatigue_not_modeled`'s
own pattern exactly -- not silently assumed absent, not fabricated.

## Proposed scope

1. `is_supported_brawler_single_class` -- exact `AcgClassId::Brawler`
   match, mirroring `is_supported_skald_single_class`/
   `is_supported_bloodrager_single_class` exactly, added to
   `has_supported_class_chassis`.
2. `brawler_ac_bonus(level: u8) -> i16` -- pure function, the
   `ACProgression` formula above. No activation/ownership-gated query
   function is needed (unlike Rage-shaped mechanics) since this is
   always-on, not a togglable ability -- ground it directly wherever
   Brawler-ownership is confirmed.
3. Ground the value inside `compute_acg_class_chassis`'s new Brawler
   branch as a standalone explanation record (mirrors the shape of
   Monk's own `class_chassis.monk.ac_bonus`, but a level-driven flat
   value rather than Wisdom-driven), PLUS integrate the SAME value into
   the shared `compute_combat_baseline`'s Armor Class total (since
   Brawler is newly admitted to `has_supported_class_chassis` and reaches
   that shared pillar, unlike Monk which uses its own separate,
   non-integrated fixture path) -- a new dodge-bonus term chained
   alongside Dodge/Rage/Inspired-Rage/Bloodrage penalties there.
4. Push the honest, non-blocking helpless/immobilized-not-modeled
   diagnostic alongside the grounded value (only when the bonus is
   actually nonzero and would matter -- mirrors the Fatigue pattern's own
   "only relevant while a state that could be lost is active" framing;
   named unconditionally is also acceptable and arguably more honest
   since the loss-condition could theoretically arise at level 1 too even
   though the bonus itself is 0 there -- lean toward unconditional for
   simplicity and consistency with the Rage-family pattern).
5. New, narrower `class_feature.acg.brawler.other_features_deferred.unsupported`
   diagnostic replacing the generic one for Brawler specifically, naming
   the 11 remaining named features (not spellcasting -- Brawler has none).
6. Tests: AC Bonus value at levels 1 (0), 4 (1), 9 (2), 13 (3), 18 (4) --
   a pure-function test needs no activation-state matrix the way Rage-
   shaped mechanics did. Plus the same negative-leak coverage (carve
   Brawler out of the all-10-classes loop in both `pilot_compute.rs` and
   `tests/sd24_acg_class_coverage_audit.rs`; the "other N classes produce
   zero pillar output" test becomes "other 7"; `named_features_wired`
   becomes 3 total across Skald/Bloodrager/Brawler).

## Reachability note (per the lead's explicit ask)

AC Bonus needs neither a `class_ability_activations` entry nor a
`selected_choices` entry -- it is unconditional given class ownership and
level, the same "always computed, never toggled" shape as base attack
bonus or base saves themselves. This sits in the SAFEST bucket of the
three (choice-gated / activation-gated / always-on), with zero product-
reachability risk of the kind frontend just flagged for Sorcerer/Cleric/
Druid's own choice mechanics.
