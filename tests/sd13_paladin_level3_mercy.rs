//! SD13-E5 Paladin level-3 progression grounding proof.
//!
//! Widens the accepted Paladin level-1/level-2 chassis-and-spell-burden
//! separation (`tests/sd13_paladin_level1_chassis_and_spell_burden_separation.rs`,
//! `tests/sd13_paladin_level2_lay_on_hands_divine_grace.rs`,
//! `tests/sd13_paladin_base_attack_and_saves.rs`) to paladin level 3,
//! mirroring the Fighter/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Sorcerer/
//! Wizard/Ranger level-range-gate idiom (`supported_paladin_level` is
//! generalized from `1..=2` to `1..=3` via `MAX_SUPPORTED_PALADIN_LEVEL = 3`).
//! It proves:
//!
//! - base attack bonus at level 3 is grounded by the same full-BAB formula
//!   (`classlevel`) already grounded at levels 1-2: `3`.
//! - base saves at level 3 are grounded by the same good-Fortitude/good-Will/
//!   poor-Reflex formulas already grounded at levels 1-2, extended to level 3:
//!   Fortitude/Will `3 / 2 + 2 = 3`, Reflex `3 / 3 = 1`.
//! - Smite Evil still scales correctly at level 3: uses/day stays 1 (below
//!   level 4), attack bonus stays the positive Charisma modifier, damage bonus
//!   = paladin level = 3.
//! - lay on hands / divine grace still scale correctly at level 3: uses/day =
//!   1/2 paladin level + Charisma modifier = 3, heal dice = paladin level / 2
//!   = 1, divine grace save bonus stays the positive Charisma modifier.
//! - the effective caster level gate stays a correct absence at level 3:
//!   `max(3 - 3, 0) = 0` (PF1 Core Rulebook: paladin spells begin at level 4).
//! - Mercy, the PF1 Core Rulebook's 3rd-level Paladin class feature (verified
//!   independently against legacy.aonprd.com's Core Rulebook Paladin page:
//!   "At 3rd level, and every three levels thereafter, a paladin can select
//!   one mercy. Each mercy adds an effect to the paladin's lay on hands
//!   ability."; the first, 3rd-level tier of the mercy list is Fatigued,
//!   Shaken, and Sickened), transitions from a level-gate ABSENCE record
//!   (value 0) at levels 1-2 to a bounded GRANT-only identity record at level
//!   3, mirroring the Barbarian Uncanny Dodge / Ranger Endurance idiom, plus a
//!   choice-recognition record naming whichever mercy was selected
//!   (mirroring the Ranger Favored Terrain / Sorcerer bloodline choice-slot
//!   idiom): no restricted-list validation is performed, and the mercy's own
//!   effect (curing the named condition when lay on hands is used) is NOT
//!   computed, since no lay-on-hands execution engine exists in this
//!   codebase.
//!
//! It deliberately does not implement any lay-on-hands execution engine, any
//! condition-curing engine, and it does not ground Paladin level 4+ (where
//! Smite Evil's uses/day would first increase) or the partial-caster spell
//! burden's actual content (spells known/prepared, spells per day, bonus
//! spell slots, spell save DCs). It also preserves the accepted Paladin
//! level-1/level-2 truth (unchanged), the F6 hybrid baseline, the Ranger
//! negative control, and the Fighter negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{SupportState, seeded_current_truth};
mod common;
use common::{load, explanation, has_explanation};

const PALADIN_LEVEL2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level2_sd13_deterministic_input.txt");

const PALADIN_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level3_sd13_deterministic_input.txt");

const RANGER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

const BASE_ATTACK_ID: &str = "class_chassis.paladin.base_attack_bonus";
const BASE_SAVE_FORTITUDE_ID: &str = "class_chassis.paladin.base_save.fortitude";
const BASE_SAVE_REFLEX_ID: &str = "class_chassis.paladin.base_save.reflex";
const BASE_SAVE_WILL_ID: &str = "class_chassis.paladin.base_save.will";

const SMITE_EVIL_USES_PER_DAY_ID: &str = "class_chassis.paladin.smite_evil_uses_per_day";
const SMITE_EVIL_ATTACK_BONUS_ID: &str = "class_chassis.paladin.smite_evil_attack_bonus";
const SMITE_EVIL_DAMAGE_BONUS_ID: &str = "class_chassis.paladin.smite_evil_damage_bonus";

const LAY_ON_HANDS_USES_PER_DAY_ID: &str = "class_chassis.paladin.lay_on_hands_uses_per_day";
const LAY_ON_HANDS_HEAL_AMOUNT_ID: &str = "class_chassis.paladin.lay_on_hands_heal_amount";
const DIVINE_GRACE_SAVE_BONUS_ID: &str = "class_chassis.paladin.divine_grace_save_bonus";

const EFFECTIVE_CASTER_LEVEL_ID: &str = "class_chassis.paladin.partial_caster.effective_caster_level";
const PARTIAL_CASTER_BLOCKER_ID: &str = "class_spell.paladin.partial_caster.unsupported";

// The level-1/level-2-only mercy absence gate; must not appear at level 3.
const MERCY_GATE_ID: &str = "class_chassis.paladin.level_gate.mercy";

// The newly grounded level-3 mercy grant + choice-recognition records.
const MERCY_GRANTED_ID: &str = "class_chassis.paladin.mercy_granted";
const MERCY_CHOICE_ID: &str = "class_chassis.paladin.mercy_choice";

// ----- Base attack / base save at level 3 -----

#[test]
fn paladin_level3_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(PALADIN_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 3,
        "Paladin level 3 full BAB (classlevel) must equal 3: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 3, "Paladin level 3 good Fortitude (3/2+2) must equal 3");

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 1, "Paladin level 3 poor Reflex (3/3) must equal 1");

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 3, "Paladin level 3 good Will (3/2+2) must equal 3");
}

// ----- Smite Evil still scales correctly at level 3 -----

#[test]
fn paladin_level3_smite_evil_scales_correctly() {
    let input = load(PALADIN_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 1,
        "smite evil uses per day stays 1/day below level 4: {uses_per_day:?}"
    );

    // CG-03 fix: Charisma modifier is now +3 (base 14 + 2 Human racial), not +2.
    let attack_bonus = explanation(&computation, SMITE_EVIL_ATTACK_BONUS_ID);
    assert_eq!(
        attack_bonus.value, 3,
        "smite evil attack bonus must equal the Charisma modifier (+3 for CHA 14 + 2 Human \
         racial): {attack_bonus:?}"
    );

    let damage_bonus = explanation(&computation, SMITE_EVIL_DAMAGE_BONUS_ID);
    assert_eq!(
        damage_bonus.value, 3,
        "smite evil damage bonus must equal paladin level (3 at level 3): {damage_bonus:?}"
    );
}

// ----- Lay on hands / divine grace still scale correctly at level 3 -----

#[test]
fn paladin_level3_lay_on_hands_and_divine_grace_scale_correctly() {
    let input = load(PALADIN_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // CG-03 fix: Charisma modifier is now +3 (base 14 + 2 Human racial), not +2.
    let uses_per_day = explanation(&computation, LAY_ON_HANDS_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 4,
        "lay on hands uses per day at level 3 (1/2 paladin level + Charisma modifier = 1 + 3) \
         must equal 4: {uses_per_day:?}"
    );

    let heal_amount = explanation(&computation, LAY_ON_HANDS_HEAL_AMOUNT_ID);
    assert_eq!(
        heal_amount.value, 1,
        "lay on hands heal amount at level 3 (paladin level / 2 = 1) must equal 1: {heal_amount:?}"
    );

    // CG-03 fix: Charisma modifier is now +3 (base 14 + 2 Human racial), not +2.
    let save_bonus = explanation(&computation, DIVINE_GRACE_SAVE_BONUS_ID);
    assert_eq!(
        save_bonus.value, 3,
        "divine grace save bonus at level 3 must still equal the positive Charisma modifier: \
         {save_bonus:?}"
    );
}

// ----- Effective caster level stays a correct absence at level 3 -----

#[test]
fn paladin_level3_effective_caster_level_stays_correctly_zero() {
    let input = load(PALADIN_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let caster_level = explanation(&computation, EFFECTIVE_CASTER_LEVEL_ID);
    assert_eq!(
        caster_level.value, 0,
        "effective caster level at paladin level 3 (max(3 - 3, 0)) must stay 0: {caster_level:?}"
    );

    // (v0.6 alpha swarm, risks item 8, 2026-07-25) `PARTIAL_CASTER_BLOCKER_ID`
    // is no longer unconditional: it's a real, conditional validation of
    // AcquisitionMode::Prepared selections. This fixture predates
    // spells_selected (zero prepared), so the posture is genuinely valid and
    // the blocker correctly does not fire -- the real "nothing fabricated"
    // guarantee now comes from the daily-preparation record's own count
    // being honestly 0.
    match computation.diagnostics.iter().find(|d| d.id == PARTIAL_CASTER_BLOCKER_ID) {
        Some(spell_blocker) => assert!(spell_blocker.claim_blocking),
        None => {
            let daily_prep = explanation(&computation, "class_spell.paladin.daily_preparation");
            assert_eq!(daily_prep.value, 0, "no spells are fabricated at paladin level 3: {daily_prep:?}");
        }
    }
}

// ----- Mercy transitions from a level-gate absence to a grant + choice record -----

#[test]
fn paladin_level3_grounds_mercy_as_a_grant_and_choice_recognition_record() {
    let input = load(PALADIN_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, MERCY_GATE_ID),
        "level-3 Paladin must not gain the level-1/level-2-only mercy absence gate: {:?}",
        computation.explanations
    );

    let granted = explanation(&computation, MERCY_GRANTED_ID);
    assert_eq!(
        granted.value, 0,
        "mercy grant record carries no fabricated mechanical value: {granted:?}"
    );
    assert!(
        granted.detail.to_lowercase().contains("3rd-level")
            || granted.detail.to_lowercase().contains("3rd level"),
        "mercy grant detail must name the 3rd-level PF1 CRB grant: {}",
        granted.detail
    );
    assert!(
        granted.detail.to_lowercase().contains("fatigued")
            && granted.detail.to_lowercase().contains("shaken")
            && granted.detail.to_lowercase().contains("sickened"),
        "mercy grant detail must name the verified 3rd-level mercy list (Fatigued, Shaken, \
         Sickened): {}",
        granted.detail
    );
    assert!(
        granted.detail.to_lowercase().contains("not computed")
            || granted.detail.to_lowercase().contains("no lay-on-hands execution"),
        "mercy grant detail must disclaim computing the mercy's actual effect: {}",
        granted.detail
    );

    let choice = explanation(&computation, MERCY_CHOICE_ID);
    assert_eq!(
        choice.value, 0,
        "mercy choice recognition carries no fabricated mechanical value: {choice:?}"
    );
    assert!(
        choice.detail.contains("mercy:shaken"),
        "mercy choice detail must name the selected mercy (mercy:shaken): {}",
        choice.detail
    );
}

// ----- Negative control: level 1/2 keep the mercy absence gate, unaffected -----

#[test]
fn paladin_level2_mercy_absence_gate_is_unaffected_by_the_level3_widening() {
    let input = load(PALADIN_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let mercy = explanation(&computation, MERCY_GATE_ID);
    assert_eq!(mercy.value, 0, "level 2 mercy must remain a correct absence: {mercy:?}");
    assert!(
        !has_explanation(&computation, MERCY_GRANTED_ID),
        "level-2 Paladin must not gain the level-3-only mercy grant record"
    );
    assert!(
        !has_explanation(&computation, MERCY_CHOICE_ID),
        "level-2 Paladin must not gain the level-3-only mercy choice record"
    );
}

// ----- Negative control (retired): Paladin level 4 was later widened into the -----
// ----- supported tranche; see tests/sd13_paladin_level4_progression.rs           -----

#[test]
fn paladin_level_4_was_later_widened_into_the_supported_tranche() {
    // A subsequent SD13-E5 slice (tests/sd13_paladin_level4_progression.rs) widened
    // `supported_paladin_level` from 1..=3 to 1..=4, so level 4 now gains this
    // slice's grounding via the same formulas (Smite Evil uses/day genuinely
    // increasing to 2/day, Channel Positive Energy newly grounded, and the
    // effective caster level genuinely becoming 1). This control now asserts the
    // widened truth rather than the original (now-stale) "level 4 stays
    // unrecognized" claim; the equivalent negative control for the current
    // boundary (level 5) lives in tests/sd13_paladin_level4_progression.rs's
    // `paladin_level_5_is_not_promoted_by_this_slice`. Mercy stays granted
    // (not re-derived) at level 4.
    let level_4 = PALADIN_LEVEL3_FIXTURE.replace("class:paladin:3", "class:paladin:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 4,
        "level-4 Paladin must now gain the widened base-attack grounding (classlevel = 4): {:?}",
        base_attack
    );
    assert!(
        has_explanation(&computation, MERCY_GRANTED_ID),
        "level-4 Paladin must still gain the Mercy grant explanation (granted once at level 3, \
         unchanged at level 4)"
    );
    assert!(
        has_explanation(&computation, MERCY_CHOICE_ID),
        "level-4 Paladin must still gain the Mercy choice explanation (unchanged at level 4)"
    );
}

// ----- Negative control: the grounding must not leak onto other classes -----

#[test]
fn fighter_and_ranger_do_not_gain_paladin_level3_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(!has_explanation(&fighter_computation, MERCY_GRANTED_ID));
    assert!(!has_explanation(&fighter_computation, MERCY_CHOICE_ID));

    let ranger_computation = compute_pilot_base_chassis(&load(RANGER_FIXTURE));
    assert!(!has_explanation(&ranger_computation, MERCY_GRANTED_ID));
    assert!(!has_explanation(&ranger_computation, MERCY_CHOICE_ID));
}

// ----- Control plane: the matrix row's note names the level-3 widening and mercy -----

#[test]
fn matrix_paladin_row_names_level_3_widening_and_mercy() {
    let matrix = seeded_current_truth();
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin row must exist");

    assert_eq!(paladin.support_state, SupportState::Supported);
    assert!(
        paladin.grounding_ref.contains("sd13_paladin_level3_mercy"),
        "paladin row must cite the live SD13-E5 level-3 mercy proof surface: {}",
        paladin.grounding_ref
    );
    let note = paladin.blocker_or_lossiness_note;
    assert!(
        note.to_lowercase().contains("mercy") && note.to_lowercase().contains("granted"),
        "paladin partial note must name mercy as newly granted at level 3: {note}"
    );
}
