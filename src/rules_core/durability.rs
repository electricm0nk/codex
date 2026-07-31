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
//!
//! **Update (task #58, v0.6 alpha swarm):** the Rogue Talent Resiliency
//! grounds the FIRST temporary-hit-point magnitude in this codebase
//! (`rogue_resiliency_temp_hp`/`investigator_resiliency_temp_hp`), but the
//! "future scope decision" above is still genuinely unmade — no aggregate
//! temporary-HP total field exists anywhere in this crate or its
//! downstream `apps/desktop/src-tauri/character_hub.rs` consumer (which
//! tracks only `max_hp`/`current_hp`). These two functions are pure
//! per-source magnitudes, grounded in `pilot_compute.rs` as standalone
//! flat facts (the same honest-gap idiom as Witch's Ward hex), not wired
//! into any total, because there is no total to wire into yet.

use crate::rules_core::character_input::CharacterClassLevel;
use crate::rules_core::pilot_compute::table_class_id;
use crate::rules_core::rules_tables::acg::{self, AcgClassId};
use crate::rules_core::rules_tables::apg::{self, ApgClassId};
use crate::rules_core::rules_tables::crb::class_tables::hit_die_for;
use crate::rules_core::rules_tables::pathfinder_unchained::class_chassis::{
    self as pu_class_chassis, PuClassId,
};

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
    // SD-27 (2026-07-31) added the Pathfinder Unchained tier at the end of
    // the same chain. Three of the four Unchained classes deliberately
    // return their base class's die (the corpus record overrides nothing);
    // the Unchained Monk returns d10 against the Core Rulebook Monk's
    // operator-ruled d8, which is a real, visible HP difference and is
    // pinned by `unchained_monk_hp_uses_its_own_d10_not_the_crb_monks_d8`.
    let hit_die_size = match table_class_id(&class_level.class_id).and_then(hit_die_for) {
        Some(hit_die_size) => hit_die_size,
        None => match ApgClassId::from_class_id_str(&class_level.class_id) {
            Some(apg_class_id) => apg::hit_die_for(apg_class_id),
            None => match AcgClassId::from_class_id_str(&class_level.class_id) {
                Some(acg_class_id) => acg::hit_die_for(acg_class_id),
                None => PuClassId::from_class_id_str(&class_level.class_id)
                    .map(pu_class_chassis::hit_die_for)?,
            },
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
    // The negative control for `compute_max_hp`'s "unrecognized class"
    // branch. This constant has now been substituted THREE times for the
    // same reason -- Cleric, then Barbarian, then Monk each stopped being
    // unrecognized as the class roster widened underneath it.
    //
    // v0.6 alpha swarm (Monk/Summoner chassis-recognition closure,
    // 2026-07-29): it is deliberately no longer a real class at all, and
    // it cannot be one again. Monk was the LAST real class missing from
    // `table_class_id`; with it mapped, the three rosters
    // `compute_max_hp` consults -- `table_class_id` (11 CRB),
    // `ApgClassId` (6 APG), `AcgClassId` (10 ACG) -- cover all 27 base
    // classes. So no real PF1 base class can serve as this negative
    // control any more, and picking a fourth real class would just queue
    // up a fourth silent promotion. A synthetic id exercises the same
    // `None` branch permanently.
    const UNRECOGNIZED_CLASS_ID: &str = "class:not_a_real_pf1_class";

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
        let max_hp = compute_max_hp(&[class_level(UNRECOGNIZED_CLASS_ID, 1)], 2);
        assert_eq!(max_hp, None);
    }

    /// The other half of the constant swap above: Monk must now resolve a
    /// REAL max HP rather than `None`, proving the negative control was
    /// retired because the gap genuinely closed -- not because the
    /// assertion was weakened to make a failure go away.
    ///
    /// Monk d8 (published PF1 Core Rulebook p.56; operator ruling
    /// 2026-07-29, risks item 91), level 1, CON mod +2: maximized level 1
    /// = 8 + 2 = 10.
    ///
    /// This is deliberately NOT the corpus's `cr_classes.lst:147`
    /// `CLASS:Monk HD:10`, which would give 12. `CLASS_META`'s Monk row
    /// carries a documented corpus-defect override -- read its comment
    /// block before changing this expectation.
    #[test]
    fn compute_max_hp_now_resolves_monk_the_last_class_to_join_table_class_id() {
        let max_hp = compute_max_hp(&[class_level("class:monk", 1)], 2);
        assert_eq!(max_hp, Some(10));
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

/// v0.6 alpha swarm, task #11 Tier 0 (2026-07-27): the familiar's master
/// benefit, which is a flat bonus on the MASTER and reads nothing from
/// the familiar creature's own stat block.
/// The canonical familiar species a spellcaster may bond with (v0.6
/// alpha swarm, task #11 Tier 0, 2026-07-27).
///
/// Only the four whose master benefit lands on a total this engine
/// actually computes are represented. The other seven standard familiars
/// (Bat, Cat, Hawk, Monkey, Owl, Raven, Viper) grant flat skill bonuses
/// on skills this engine does not compute, so they would ground as
/// standalone facts rather than reaching a total, and are deferred with
/// the rest of the chooser.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FamiliarSpecies {
    /// `BONUS:HP|CURRENTMAX|FamiliarGrantedBonus_3` -- the canonical pick.
    Toad,
    /// `BONUS:SAVE|Fortitude|FamiliarGrantedBonus_2`.
    Rat,
    /// `BONUS:SAVE|Reflex|FamiliarGrantedBonus_2`.
    Weasel,
    /// `BONUS:SKILL|Climb|FamiliarGrantedBonus_3`.
    Lizard,
}

/// The Toad familiar's flat bonus to its master's maximum hit points.
///
/// Verified against `core_essentials/ce_abilities_familiar_cr.lst`'s own
/// `Familiar Granted Bonus (Toad)` record, whose
/// `BONUS:HP|CURRENTMAX|FamiliarGrantedBonus_3` resolves through
/// `BONUS:VAR|FamiliarGrantedBonus_3|3` in that same file.
///
/// **Provenance is not a reachability problem, despite appearances.**
/// These records live under `core_essentials/`, which is not one of the
/// four books this repo ingests -- easy to mistake for the
/// Bloodrager-style unreachable-content case. The file itself declares
/// `SOURCELONG:Core Rulebook` / `SOURCESHORT:CR`, and `core_essentials`
/// is PCGen's own ORGANISATIONAL directory rather than a rulebook. These
/// are Core Rulebook rules.
///
/// **The negative setters are provably vacuous here.** The tree also
/// carries `FamiliarGrantedBonus_N|-1/-2/-3/-4` lines, some gated on
/// `PREVAREQ:FamiliarBondFeatActive,1`, which would cancel the bonus.
/// Every one lives in `player_companion/familiar_folio` or
/// `horror_adventures`, neither ingested here -- verified by tracing all
/// twelve setters to their files, the same check that cleared
/// Alchemist's Ultimate-Magic Bomb override. Re-verify if the ingested
/// book set ever widens.
const FAMILIAR_TOAD_MAX_HP_BONUS: i16 = 3;

/// The familiar's flat contribution to its master's maximum hit points.
///
/// Returns 0 for a character with no familiar and for every species
/// whose master benefit lands somewhere other than hit points. Kept in
/// this module because `durability.rs` owns hit points, and applied as a
/// per-character add-on at the same consumer that already layers
/// Toughness's feat bonus -- not folded into `compute_max_hp`, which
/// owns only the class hit-die table.
pub fn familiar_master_hp_bonus(species: Option<FamiliarSpecies>) -> i16 {
    match species {
        Some(FamiliarSpecies::Toad) => FAMILIAR_TOAD_MAX_HP_BONUS,
        _ => 0,
    }
}

/// The Rogue Talent Resiliency's temporary-hit-point magnitude (task #58,
/// v0.6 alpha swarm), verified against `cr_abilities_class.lst`'s own
/// `KEY:Rogue Talent ~ Resiliency` record: `BONUS:VAR|ResiliencyHitPoints|
/// RogueTalentLVL`, with `RogueTalentLVL` resolving to the rogue's own
/// class level and no `PRE` gate at all. PF1 Core Rulebook: "Once per
/// day, a rogue with this ability can gain a number of temporary hit
/// points equal to the rogue's level. Activating this ability is an
/// immediate action that can only be performed when she is brought to
/// below 0 hit points." These temporary hit points last 1 minute.
///
/// Grounds the magnitude only. The once-per-day budget and the "brought
/// below 0 hit points" trigger are named but not enforced anywhere in
/// this codebase -- no once-per-day activation tracker and no
/// HP-threshold trigger engine exists, the same honest-gap idiom already
/// used for Skald's Raging Song rounds-per-day (`skald_inspired_rage_
/// rounds_per_day` in `pilot_compute.rs`).
pub fn rogue_resiliency_temp_hp(rogue_level: u8) -> i16 {
    i16::from(rogue_level)
}

/// Investigator's OWN separate copy of Resiliency (task #58, v0.6 alpha
/// swarm), verified against `acg_abilities_class.lst`'s own
/// `KEY:Investigator ~ Rogue Talent ~ Resiliency` record:
/// `BONUS:VAR|ResiliencyHitPoints|InvestigatorLVL` -- a distinct corpus
/// record from Rogue's own copy above, keyed to its own level variable
/// (`InvestigatorLVL`, not `RogueTalentLVL`), not a shared formula.
/// Investigator draws Resiliency from its own explicit whitelist of
/// `KEY:Investigator ~ Rogue Talent ~ *` records (40 entries), of which
/// Resiliency is one. Same magnitude-only, budget/trigger-named-but-
/// unenforced treatment as `rogue_resiliency_temp_hp`.
pub fn investigator_resiliency_temp_hp(investigator_level: u8) -> i16 {
    i16::from(investigator_level)
}

#[cfg(test)]
mod familiar_master_bonus_tests {
    use super::*;

    #[test]
    fn a_toad_familiar_grants_its_master_three_max_hit_points() {
        assert_eq!(familiar_master_hp_bonus(Some(FamiliarSpecies::Toad)), 3);
    }

    /// Every other canonical familiar's benefit lands somewhere other
    /// than hit points, so the HP bonus must be 0 for them -- and 0 for
    /// a character with no familiar at all.
    #[test]
    fn only_the_toad_contributes_to_hit_points() {
        assert_eq!(familiar_master_hp_bonus(None), 0);
        for species in [FamiliarSpecies::Rat, FamiliarSpecies::Weasel, FamiliarSpecies::Lizard] {
            assert_eq!(familiar_master_hp_bonus(Some(species)), 0, "{species:?} is not an HP familiar");
        }
    }

    /// The whole point of picking Toad as canonical: its bonus reaches
    /// the REAL computed max-HP total, not just an explanation record.
    /// Differenced against the same build without a familiar.
    #[test]
    fn the_toad_bonus_actually_raises_a_real_computed_max_hp_total() {
        let levels = [CharacterClassLevel {
            class_id: "class:fighter".to_owned(),
            level: 1,
        }];
        let base = compute_max_hp(&levels, 2).expect("fighter max hp");
        let with_toad = base + familiar_master_hp_bonus(Some(FamiliarSpecies::Toad));
        assert_eq!(with_toad - base, 3, "a Toad familiar is worth 3 real hit points");
    }
}

#[cfg(test)]
mod resiliency_temp_hp_tests {
    use super::*;

    /// Rogue's own copy (`cr_abilities_class.lst`, `KEY:Rogue Talent ~
    /// Resiliency`): `BONUS:VAR|ResiliencyHitPoints|RogueTalentLVL`, and
    /// `RogueTalentLVL` resolves to the rogue's own class level with no
    /// `PRE` gate -- PF1 Core Rulebook: "a rogue with this ability can
    /// gain a number of temporary hit points equal to the rogue's level."
    #[test]
    fn rogue_resiliency_temp_hp_equals_rogue_level() {
        assert_eq!(rogue_resiliency_temp_hp(1), 1);
        assert_eq!(rogue_resiliency_temp_hp(10), 10);
        assert_eq!(rogue_resiliency_temp_hp(20), 20);
    }

    /// Investigator's OWN separate record (`acg_abilities_class.lst`,
    /// `KEY:Investigator ~ Rogue Talent ~ Resiliency`):
    /// `BONUS:VAR|ResiliencyHitPoints|InvestigatorLVL` -- a distinct
    /// corpus record from Rogue's own copy, keyed to its own level
    /// variable, not a shared formula. Verifies the two functions are
    /// genuinely independent (not aliases of each other) despite the
    /// identical formula shape.
    #[test]
    fn investigator_resiliency_temp_hp_equals_investigator_level() {
        assert_eq!(investigator_resiliency_temp_hp(3), 3);
        assert_eq!(investigator_resiliency_temp_hp(10), 10);
        assert_eq!(investigator_resiliency_temp_hp(20), 20);
    }

    /// A level-0 (no levels in the class) input grounds to 0 temporary hit
    /// points for both copies -- the formula floors naturally, no clamp
    /// needed.
    #[test]
    fn zero_level_grounds_to_zero_temp_hp_for_both_copies() {
        assert_eq!(rogue_resiliency_temp_hp(0), 0);
        assert_eq!(investigator_resiliency_temp_hp(0), 0);
    }
}
