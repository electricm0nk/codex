# ACG Brawler — Per-Class Coverage (SD-24 Epic 4, criterion 4.3)

**Updated (v0.6 alpha swarm, risks item 8, Brawler deepening, 2026-07-26,
directed by the operator to complete in-progress classes rather than
start new ones):** Brawler's Cunning (flat, unconditional Intelligence
floor for combat-feat prerequisites) and Brawler's Strike (a real
level-gated DR-bypass progression, honestly inert below level 5) are now
also genuinely wired, narrowing `class_features_wired` from 1 to 3. Also
fixed the SIXTH instance of the class-skill-bonus widening bug — found
incidentally while in this file for an unrelated reason — Brawler's own
real class-skill list genuinely includes Climb/Intimidate/Swim. See the
Update section below for the full record. **Also corrects a stale/
inaccurate "Gap features" list** (the prior version invented entries --
"Brawler Weapon Training", "Improved Unarmed Strike", "Unarmed Strike
progression" -- that are not real `KEY:Brawler ~ ...` records).

| Field | Value |
|---|---|
| `class_name` | Brawler |
| `book` | ACG |
| `feature_table_path` | `rules_core::rules_tables::acg::class_brawler` (chassis); `pilot_compute::ground_brawler_ac_bonus_and_defer_the_rest` (AC Bonus, Cunning, Strike) |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_class_guide/acg_classes.lst` (chassis) + `acg_abilities_class.lst` (named features) |
| `class_features_expected` | 14 (distinct `KEY:Brawler ~ ...` records in `acg_abilities_class.lst`) |
| `class_features_wired` | 3 (AC Bonus, Brawler's Cunning, Brawler's Strike) |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | Yes — `compute_class_chassis`/`compute_acg_class_chassis` in `pilot_compute.rs` recognize single-class Brawler (v0.6 alpha swarm) |
| `level_up_wired` | No — no `level_up::brawler` module exists |
| `gap_priority` | P2 (real, verified partial coverage; permanently `Blocked` on the remaining named features, all genuinely out of scope for a bounded slice) |

## Gap features (from `acg_abilities_class.lst`, corpus commit above)

Brawler's Flurry (full-attack iterative-attack mechanic), Knockout (opponent-must-save-or-fall-unconscious), Martial Flexibility (arbitrary feat-prerequisite modeling, already ruled out in `brawler-acg-third-class-scoping.md`), Awesome Blow / Improved Awesome Blow (combat maneuver vs. an opponent), Martial Training (a flat fact, deferred alongside the others this slice), Bonus Feats (feat-selection chooser), Close Weapon Mastery (needs an unarmed-damage-by-level table this class doesn't have transcribed), Maneuver Training (this codebase has no CMB pillar for any of the ~10 sub-bonuses), and Brawler's Strike's own Alignment Selection chooser (unlocked at progression tier 3+, level 12).

## Verification

- Chassis math cross-checked against `acg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens (see `class_brawler.rs`'s own doc comment for the exact transcribed tokens) — matches, no defect.
- Brawler's Cunning's `max(13,INTSCORE)` formula verified directly against `acg_abilities_class.lst`'s own `BONUS:VAR|CombatFeatIntRequirement|max(13,INTSCORE)` — no `PREVARGTEQ`/`PRECLASS` gate at all, genuinely unconditional from level 1.
- Brawler's Strike's own progression (`(level>=5)+(level>=9)+(level>=12)+(level>=17)`) verified directly against `acg_abilities_class.lst`'s own `BONUS:VAR|BrawlersStrikeProgression|...` formula and its two real `DESC:` lines (tier 1: magic; tier 2+: magic/cold iron/silver) — confirmed genuinely inert below level 5 (a real "not yet gained" gate, the same honesty already established for Swashbuckler's Charmed Life), not a formula that merely evaluates to zero.
- Brawler's own real class-skill list confirmed to include all three of Climb/Intimidate/Swim (`CSKILL:Acrobatics|Climb|...|Intimidate|...|Swim`) — the sixth class needing the class-skill-bonus widening fix.
- `tests/sd24_acg_class_coverage_audit.rs` exercises this row via `rules_tables::acg::class_coverage(AcgClassId::Brawler)` and the live `compute_pilot_base_chassis` seam; `pilot_compute.rs`'s own `brawler_cunning_grounds_the_effective_intelligence_floor`/`brawler_strike_is_honestly_not_yet_gained_below_level_5_then_progresses`/`brawler_other_features_deferred_acknowledges_cunning_and_strike_as_grounded` tests exercise the new dispatch directly.

## Update (v0.6 alpha swarm, risks item 8, third APG/ACG class-specific closure, 2026-07-25)

Brawler's own chassis-integration gate (`is_supported_brawler_single_class` in `pilot_compute.rs`) and AC Bonus are now genuinely wired: a level-driven dodge bonus to Armor Class (`(level>3)+(level>8)+(level>12)+(level>17)`, verified against the corpus `BONUS:VAR` formula), integrated into the shared `compute_combat_baseline` Armor Class total. Unlike Skald/Bloodrager's Rage-shaped mechanics, AC Bonus needs no `class_ability_activations` entry at all — it is a pure function of level and class ownership, since the "not wearing Medium/Heavy armor" precondition is provably vacuous in this codebase (no Medium/Heavy armor item id exists anywhere; the only armor this codebase can express, Chain Shirt, is itself light armor and is required `EquippedActive` unconditionally by the shared combat-baseline posture). Brawler is a pure martial class (no `SPELLSTAT`) — its remaining named-feature bucket (Brawler's Flurry, Knockout, Martial Flexibility, Awesome Blow, Improved Awesome Blow, Brawler's Cunning, Martial Training, Bonus Feats, Close Weapon Mastery, Brawler's Strike, Maneuver Training) stays claim-blocked via the new `class_feature.acg.brawler.other_features_deferred.unsupported` diagnostic (not `spellcasting_deferred`, since Brawler casts nothing), replacing the old generic `class_feature.acg.brawler.unsupported` diagnostic for Brawler specifically. See `pilot_compute.rs`'s `brawler_ac_bonus_progression_matches_the_corpus_formula_at_higher_levels` test and `docs/release/v0.6/brawler-acg-third-class-scoping.md` for the full scoping record.

