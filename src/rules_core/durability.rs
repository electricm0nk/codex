//! Durability (character survivability) — v0.6 alpha swarm, wave-2 item 2.
//!
//! Per the lead's ruling (`docs/release/v0.6/risks-and-open-questions.md`
//! open question 4): durability = max/current/temporary HP, nonlethal
//! damage tracking, and dying/unconscious/death thresholds — distinct from
//! "level-up hit points" (the level-up increment calc, `level_up.rs`).
//! Before this file, the only grounded HP number anywhere in this crate was
//! Fighter's own level-1 value (`pilot_compute.rs`'s
//! `explain_fighter_level1_hit_points`); no running total, no per-level
//! formula beyond level 1, and no threshold classification existed for any
//! class.
//!
//! Max HP formula (standard PF1 rule, open game content, not itself a
//! PCGen-sourced data point — the *inputs* to it, hit die size per class,
//! are sourced from the same `cr_classes.lst` `HD:` token
//! `rules_tables::crb::class_tables::CLASS_META` already cites for every
//! other per-class field): level 1 uses the maximum value of the class's
//! hit die; every level after that uses the average value (half the die
//! size, rounded up — `average_hit_die_value`, the PF1 Core Rulebook's own
//! named non-rolling default), each level's contribution floored at a
//! minimum of 1 HP regardless of a Constitution penalty. Scoped to
//! single-class Fighter/Wizard/Rogue (the classes `table_class_id`
//! recognizes) for the same reason `pilot_compute.rs`'s own dispatch is
//! scoped there — deliberately not attempted for multiclass here:
//! `CharacterClassLevel` records each class's *cumulative* level, not the
//! order individual levels were taken in, so "which single level was
//! character-level 1 (and gets the maximized die)" is genuinely ambiguous
//! for a multiclass build from this data shape alone, not an oversight.
//!
//! Explicitly out of scope, per QA's own spec appendix in
//! `SWARM_REPORT.md`, flagged for a future scope decision rather than
//! guessed at here: temporary HP (spell/effect-granted, not chassis math)
//! and the Favored Class Bonus HP choice (recognized as a choice slot in
//! `pilot_compute.rs`'s `explain_fighter_favored_class_bonus_choice` but
//! never summed into any total there either — this file does not change
//! that).

use crate::rules_core::character_input::CharacterClassLevel;
use crate::rules_core::pilot_compute::table_class_id;
use crate::rules_core::rules_tables::acg::{self, AcgClassId};
use crate::rules_core::rules_tables::apg::{self, ApgClassId};
use crate::rules_core::rules_tables::crb::class_tables::hit_die_for;

/// Half the die size, rounded up — the PF1 Core Rulebook's own named
/// non-rolling default for hit points gained after 1st level (e.g. a d10
/// averages to 6, a d6 to 4).
pub fn average_hit_die_value(hit_die_size: u8) -> i16 {
    i16::from(hit_die_size) / 2 + 1
}

/// Computes a single-class character's max HP: the maximized hit die at
/// level 1, the average hit die at every level after that, each level's
/// contribution floored at 1 HP even against a Constitution penalty, summed
/// across the character's level. Returns `None` for a multiclass
/// `class_levels` (see module doc comment for why) or an unrecognized
/// class id.
pub fn compute_max_hp(class_levels: &[CharacterClassLevel], constitution_modifier: i16) -> Option<i16> {
    let [class_level] = class_levels else {
        return None;
    };
    // v0.6 alpha swarm, risks item 8: falls back to the APG, then ACG,
    // hit-die table when the class isn't one of the 5 `table_class_id`
    // recognizes -- neither `ApgClassId::from_class_id_str` nor
    // `AcgClassId::from_class_id_str` is registered with `table_class_id`,
    // so this stays single-class-only by the same `[class_level]`
    // destructure above, not a new multiclass path.
    let hit_die_size = match table_class_id(&class_level.class_id).and_then(hit_die_for) {
        Some(hit_die_size) => hit_die_size,
        None => match ApgClassId::from_class_id_str(&class_level.class_id) {
            Some(apg_class_id) => apg::hit_die_for(apg_class_id),
            None => AcgClassId::from_class_id_str(&class_level.class_id)
                .map(acg::hit_die_for)?,
        },
    };

    let mut total = 0_i16;
    for level in 1..=class_level.level {
        let die_value = if level == 1 {
            i16::from(hit_die_size)
        } else {
            average_hit_die_value(hit_die_size)
        };
        total += (die_value + constitution_modifier).max(1);
    }
    Some(total)
}

/// A character's current survivability state, in ascending order of
/// severity (see `classify_durability`'s doc comment for the exact
/// threshold rules, per QA's SRD-sourced spec appendix).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurabilityStatus {
    Normal,
    Staggered,
    Disabled,
    Unconscious,
    Dying,
    Dead,
}

/// Classifies a character's current survivability state from current HP,
/// accumulated nonlethal damage, and Constitution score — standard PF1/d20
/// SRD thresholds:
/// - `current_hp <= -constitution_score` -> `Dead`.
/// - `current_hp < 0` (and not dead) -> `Dying` (unconscious, losing 1 HP/round
///   unless stabilized).
/// - `current_hp == 0` -> `Disabled` (can take a single move or standard
///   action per round; a standard action causes 1 more point of nonlethal
///   damage).
/// - `current_hp > 0` and `nonlethal_damage > current_hp` -> `Unconscious`
///   (stable, not dying — the excess is nonlethal).
/// - `current_hp > 0` and `nonlethal_damage == current_hp` -> `Staggered`.
/// - otherwise -> `Normal`.
pub fn classify_durability(current_hp: i16, nonlethal_damage: i16, constitution_score: i16) -> DurabilityStatus {
    if current_hp <= -constitution_score {
        DurabilityStatus::Dead
    } else if current_hp < 0 {
        DurabilityStatus::Dying
    } else if current_hp == 0 {
        DurabilityStatus::Disabled
    } else if nonlethal_damage > current_hp {
        DurabilityStatus::Unconscious
    } else if nonlethal_damage == current_hp {
        DurabilityStatus::Staggered
    } else {
        DurabilityStatus::Normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIGHTER_CLASS_ID: &str = "class:fighter";
    const WIZARD_CLASS_ID: &str = "class:wizard";
    const ROGUE_CLASS_ID: &str = "class:rogue";
    const CLERIC_CLASS_ID: &str = "class:cleric";

    fn class_level(class_id: &str, level: u8) -> CharacterClassLevel {
        CharacterClassLevel { class_id: class_id.to_owned(), level }
    }

    #[test]
    fn average_hit_die_value_rounds_up_for_every_crb_die_size() {
        assert_eq!(average_hit_die_value(6), 4);
        assert_eq!(average_hit_die_value(8), 5);
        assert_eq!(average_hit_die_value(10), 6);
        assert_eq!(average_hit_die_value(12), 7);
    }

    #[test]
    fn compute_max_hp_for_fighter_level1_matches_the_already_grounded_level1_formula() {
        // Fighter d10, maximized at level 1: 10 + CON mod. Mirrors
        // pilot_compute.rs's own explain_fighter_level1_hit_points formula.
        let max_hp = compute_max_hp(&[class_level(FIGHTER_CLASS_ID, 1)], 2);
        assert_eq!(max_hp, Some(12));
    }

    #[test]
    fn compute_max_hp_sums_maximized_level1_plus_average_for_later_levels() {
        // Fighter d10, level 3, CON mod +2: level 1 maximized (10+2=12),
        // levels 2-3 average (6+2=8 each) = 12 + 8 + 8 = 28.
        let max_hp = compute_max_hp(&[class_level(FIGHTER_CLASS_ID, 3)], 2);
        assert_eq!(max_hp, Some(28));
    }

    #[test]
    fn compute_max_hp_floors_each_level_at_one_hp_against_a_constitution_penalty() {
        // Wizard d6, level 2, CON mod -5: level 1 would be 6-5=1 (already
        // at the floor), level 2 average (4-5=-1) floors up to 1. Total 2.
        let max_hp = compute_max_hp(&[class_level(WIZARD_CLASS_ID, 2)], -5);
        assert_eq!(max_hp, Some(2));
    }

    #[test]
    fn compute_max_hp_works_for_rogue_too_not_just_fighter_and_wizard() {
        // Rogue d8, level 1, CON mod +1: 8 + 1 = 9.
        let max_hp = compute_max_hp(&[class_level(ROGUE_CLASS_ID, 1)], 1);
        assert_eq!(max_hp, Some(9));
    }

    #[test]
    fn compute_max_hp_returns_none_for_a_multiclass_build() {
        let max_hp = compute_max_hp(
            &[class_level(FIGHTER_CLASS_ID, 2), class_level(ROGUE_CLASS_ID, 1)],
            2,
        );
        assert_eq!(max_hp, None);
    }

    #[test]
    fn compute_max_hp_returns_none_for_an_unrecognized_class() {
        let max_hp = compute_max_hp(&[class_level(CLERIC_CLASS_ID, 1)], 2);
        assert_eq!(max_hp, None);
    }

    #[test]
    fn classify_durability_normal_when_healthy() {
        assert_eq!(classify_durability(20, 0, 14), DurabilityStatus::Normal);
    }

    #[test]
    fn classify_durability_staggered_when_nonlethal_exactly_equals_current_hp() {
        assert_eq!(classify_durability(10, 10, 14), DurabilityStatus::Staggered);
    }

    #[test]
    fn classify_durability_unconscious_when_nonlethal_exceeds_current_hp() {
        assert_eq!(classify_durability(10, 11, 14), DurabilityStatus::Unconscious);
    }

    #[test]
    fn classify_durability_disabled_at_exactly_zero_hp() {
        assert_eq!(classify_durability(0, 0, 14), DurabilityStatus::Disabled);
    }

    #[test]
    fn classify_durability_dying_between_zero_and_negative_constitution() {
        assert_eq!(classify_durability(-3, 0, 14), DurabilityStatus::Dying);
    }

    #[test]
    fn classify_durability_dead_at_or_below_negative_constitution_score() {
        assert_eq!(classify_durability(-14, 0, 14), DurabilityStatus::Dead);
        assert_eq!(classify_durability(-20, 0, 14), DurabilityStatus::Dead);
    }
}
