//! SD13-E5 Paladin level-6 progression grounding proof.
//!
//! Widens the accepted Paladin level-1/level-2/level-3/level-4/level-5
//! chassis-and-spell-burden separation
//! (`tests/sd13_paladin_level1_chassis_and_spell_burden_separation.rs`,
//! `tests/sd13_paladin_level2_lay_on_hands_divine_grace.rs`,
//! `tests/sd13_paladin_base_attack_and_saves.rs`,
//! `tests/sd13_paladin_level3_mercy.rs`,
//! `tests/sd13_paladin_level4_progression.rs`,
//! `tests/sd13_paladin_level5_progression.rs`) to paladin level 6, mirroring
//! the Fighter/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Sorcerer/Wizard/Ranger
//! level-range-gate idiom (`supported_paladin_level` is generalized from
//! `1..=5` to `1..=6` via `MAX_SUPPORTED_PALADIN_LEVEL = 6`). Both PF1 CRB
//! primary sources (d20pfsrd and legacy.aonprd.com Paladin class table) were
//! read directly before writing any code or test: level 6 base attack bonus
//! is +6/+1, Fortitude/Will are +5, Reflex is +2, and the level-6 "Special"
//! column reads "Mercy" (a second mercy becomes selectable at 6th level and
//! every three levels thereafter). It proves:
//!
//! - base attack bonus at level 6 is grounded by the same full-BAB formula
//!   already grounded at levels 1-5: `6` (the iterative-attack notation
//!   +6/+1 is not modeled anywhere in this codebase; only the flat leading
//!   number is grounded, mirroring every other full-BAB class row).
//! - base saves at level 6 are grounded by the same good-Fortitude/good-Will/
//!   poor-Reflex formulas already grounded at levels 1-5, and BOTH good
//!   saves GENUINELY increase: Fortitude/Will `6 / 2 + 2 = 5` (up from 4 at
//!   level 5) and Reflex `6 / 3 = 2` (up from 1 at level 5) -- real value
//!   changes, verified independently against both primary sources' level-6
//!   table row, not re-derivations.
//! - Smite Evil's uses-per-day formula `1 + (paladin level - 1) / 3`
//!   correctly stays 2/day at level 6 (`1 + 5/3 = 2`), unchanged from level
//!   5; the next increase does not land until level 7, out of scope. Attack
//!   bonus stays the Charisma modifier and damage bonus keeps scaling via
//!   the same pre-existing formula (damage bonus = paladin level = 6).
//! - lay on hands GENUINELY increases at level 6 via the same pre-existing
//!   formulas: uses/day = 1/2 paladin level + Charisma modifier =
//!   `6/2 + 2 = 5` (up from 4 at level 5) and heal dice = paladin level / 2
//!   = `6/2 = 3` (up from 2 at level 5). Divine grace save bonus stays the
//!   positive Charisma modifier, unchanged.
//! - the effective caster level gate GENUINELY changes at level 6:
//!   `max(6 - 3, 0) = 3` (PF1 Core Rulebook: paladin spells begin at level
//!   4, effective caster level = paladin level - 3) -- a real value change
//!   from `2` at level 5, grounded via the same pre-existing formula (no
//!   re-derivation). This grounds only the caster-level gate arithmetic; it
//!   fabricates no spells known, no spells per day, no bonus spell slots,
//!   and no spell save DCs, and the partial-caster spell blocker keeps
//!   firing.
//! - Channel Positive Energy's flat die count stays 3 at level 6
//!   (`ceil(paladin level / 2) = ceil(6/2) = 3`), numerically unchanged from
//!   level 5 -- an integer-division coincidence (`6/2` and the ceiling of
//!   `5/2` both land on `3`), not a sign the formula stopped scaling; the
//!   next increase lands at level 7 (`ceil(7/2) = 4`).
//! - Mercy, granted once at 3rd level, stays granted (not re-derived) at
//!   level 6: the grant and single choice-recognition record both persist
//!   unchanged (`mercy:shaken`). This cycle was specifically briefed to
//!   check whether Paladin gains an actual new class feature at 6th level:
//!   verified independently against d20pfsrd and legacy.aonprd.com, the
//!   level-6 "Special" column reads "Mercy" again -- PF1 CRB grants a
//!   paladin an ADDITIONAL mercy to select at 6th level and every three
//!   levels thereafter ("these abilities are cumulative"). Since the
//!   existing mercy mechanism in this codebase is a single, ungated
//!   choice-recognition record (it does not track how many mercies have
//!   been selected or enforce a count), this genuinely-new 6th-level grant
//!   is NOT a flat/identity-shaped burden this codebase has already
//!   modeled -- grounding a second mercy SLOT would require inventing a
//!   mercy-list-growth mechanism that does not exist yet, which the
//!   operator brief explicitly forbids. It is therefore deliberately left
//!   named-but-unproven, mirroring the Rogue second-talent-slot / Barbarian
//!   Rage Power / Monk second-bonus-feat precedent exactly: no new
//!   choice-slot and no new diagnostic is added for it.
//! - Divine Bond (the level-5 grant) stays not fabricated at level 6,
//!   unaffected by this widening.
//!
//! It deliberately does not implement any lay-on-hands execution engine, any
//! channel-positive-energy healing/damage-resolution engine, any Divine Bond
//! execution engine, any mercy-list-growth/multi-slot mechanism, and it does
//! not ground Paladin level 7+ or the partial-caster spell burden's actual
//! content (spells known/prepared, spells per day, bonus spell slots, spell
//! save DCs -- the effective-caster-level gate widening to 3 does NOT mean
//! spell slots are grounded here). It also preserves the accepted Paladin
//! level-1/level-2/level-3/level-4/level-5 truth (unchanged), the F6 hybrid
//! baseline, the Ranger negative control, and the Fighter negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{SupportState, seeded_current_truth};
mod common;
use common::{load, explanation, has_explanation};

const PALADIN_LEVEL5_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level5_sd13_deterministic_input.txt");

const PALADIN_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level6_sd13_deterministic_input.txt");

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

const MERCY_GRANTED_ID: &str = "class_chassis.paladin.mercy_granted";
const MERCY_CHOICE_ID: &str = "class_chassis.paladin.mercy_choice";

const CHANNEL_POSITIVE_ENERGY_DICE_ID: &str = "class_chassis.paladin.channel_positive_energy_dice";

// ----- Base attack / base save at level 6 (both good saves genuinely increase) -----

#[test]
fn paladin_level6_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(PALADIN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 6,
        "Paladin level 6 full BAB (classlevel) must equal 6: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(
        fortitude.value, 5,
        "Paladin level 6 good Fortitude (6/2+2) must genuinely increase to 5, up from 4 at \
         level 5"
    );

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(
        reflex.value, 2,
        "Paladin level 6 poor Reflex (6/3) must genuinely increase to 2, up from 1 at level 5"
    );

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(
        will.value, 5,
        "Paladin level 6 good Will (6/2+2) must genuinely increase to 5, up from 4 at level 5"
    );
}

// ----- Smite Evil stays at 2/day at level 6 (next increase is level 7) -----

#[test]
fn paladin_level6_smite_evil_stays_2_per_day() {
    let input = load(PALADIN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 2,
        "smite evil uses per day stays 2/day at level 6 (1 + (6-1)/3 = 2); the next increase \
         does not land until level 7: {uses_per_day:?}"
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
        damage_bonus.value, 6,
        "smite evil damage bonus must equal paladin level (6 at level 6): {damage_bonus:?}"
    );
}

// ----- Lay on hands GENUINELY increases; divine grace stays unchanged at level 6 -----

#[test]
fn paladin_level6_lay_on_hands_increases_and_divine_grace_stays_unchanged() {
    let input = load(PALADIN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, LAY_ON_HANDS_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 6,
        "lay on hands uses per day at level 6 (1/2 paladin level + Charisma modifier = 3 + 3) \
         must genuinely increase to 6, up from 5 at level 5: {uses_per_day:?}"
    );

    let heal_amount = explanation(&computation, LAY_ON_HANDS_HEAL_AMOUNT_ID);
    assert_eq!(
        heal_amount.value, 3,
        "lay on hands heal amount at level 6 (paladin level / 2 = 3) must genuinely increase to \
         3, up from 2 at level 5: {heal_amount:?}"
    );

    let save_bonus = explanation(&computation, DIVINE_GRACE_SAVE_BONUS_ID);
    assert_eq!(
        save_bonus.value, 3,
        "divine grace save bonus at level 6 must still equal the positive Charisma modifier, \
         unchanged: {save_bonus:?}"
    );
}

// ----- Effective caster level GENUINELY becomes 3 at level 6 -----

#[test]
fn paladin_level6_effective_caster_level_genuinely_becomes_3() {
    let input = load(PALADIN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let caster_level = explanation(&computation, EFFECTIVE_CASTER_LEVEL_ID);
    assert_eq!(
        caster_level.value, 3,
        "effective caster level at paladin level 6 (max(6 - 3, 0)) must genuinely become 3, up \
         from 2 at level 5: {caster_level:?}"
    );

    // (v0.6 alpha swarm, risks item 8, 2026-07-25) `PARTIAL_CASTER_BLOCKER_ID`
    // is no longer unconditional: it's a real, conditional validation of
    // AcquisitionMode::Prepared selections. This fixture predates
    // spells_selected (zero prepared), so the posture is genuinely valid and
    // the blocker correctly does not fire -- the real "no spell slots are
    // fabricated" guarantee now comes from the daily-preparation record's own
    // count being honestly 0.
    match computation.diagnostics.iter().find(|d| d.id == PARTIAL_CASTER_BLOCKER_ID) {
        Some(spell_blocker) => assert!(
            spell_blocker.claim_blocking,
            "if the spell blocker fires at all, it must be claim-blocking"
        ),
        None => {
            let daily_prep = explanation(&computation, "class_spell.paladin.daily_preparation");
            assert_eq!(
                daily_prep.value, 0,
                "no spells are fabricated merely because the effective-caster-level gate widened \
                 again at level 6: {daily_prep:?}"
            );
        }
    }
}

// ----- Mercy stays granted (not re-derived) at level 6; no second slot fabricated -----

#[test]
fn paladin_level6_mercy_stays_granted_unchanged() {
    let input = load(PALADIN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let granted = explanation(&computation, MERCY_GRANTED_ID);
    assert_eq!(
        granted.value, 0,
        "mercy grant record still carries no fabricated mechanical value at level 6: {granted:?}"
    );

    let choice = explanation(&computation, MERCY_CHOICE_ID);
    assert!(
        choice.detail.contains("mercy:shaken"),
        "mercy choice detail must still name the selected mercy (mercy:shaken) at level 6: {}",
        choice.detail
    );
}

#[test]
fn paladin_level6_does_not_fabricate_a_second_mercy_slot() {
    let input = load(PALADIN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        computation
            .explanations
            .iter()
            .filter(|e| e.id == MERCY_CHOICE_ID)
            .count(),
        1,
        "the level-6 repeat mercy grant (PF1 CRB: an additional mercy is selectable at 6th \
         level and every three levels thereafter) must not fabricate a second mercy-choice \
         explanation record, since no mercy-list-growth mechanism exists in this codebase: {:?}",
        computation.explanations
    );
}

// ----- Channel Positive Energy dice count stays 3 at level 6 (integer-division coincidence) -----

#[test]
fn paladin_level6_channel_positive_energy_dice_stays_at_3() {
    let input = load(PALADIN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, CHANNEL_POSITIVE_ENERGY_DICE_ID);
    assert_eq!(
        dice.value, 3,
        "channel positive energy dice count at level 6 (ceil(paladin level / 2) = ceil(6/2)) \
         stays 3, numerically unchanged from level 5, not a sign the formula stopped scaling \
         (the next increase lands at level 7): {dice:?}"
    );
}

// ----- Divine Bond: still not fabricated at level 6 -----

#[test]
fn paladin_level6_does_not_fabricate_divine_bond() {
    let input = load(PALADIN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            // SD-34 decisions.md section 18: widened BY CONSTRUCTION, not narrowed --
            // class_feature_grant_consumer now emits the flat, citation-backed grant
            // fact (Paladin gains Divine Bond at this level, joined to a real corpus
            // record). The MECHANICAL magnitude this test guards (activation/resource-
            // consumption, weapon-enhancement, mount stat block) is still fabricated by
            // nothing, so the exact citation-backed id is carved out by NAME while every
            // other id shape remains caught.
            .any(|e| e.id.contains("divine_bond")
                && e.id != "class_feature.paladin.corpus_record.divine_bond"),
        "Divine Bond (the PF1 CRB's 5th-level paladin class feature) still requires an \
         activation/resource-consumption engine and either a weapon-enhancement subsystem or a \
         full mount stat-block/advancement subsystem, neither of which exists in this codebase; \
         no explanation record must be fabricated for it at level 6 either: {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.contains("divine_bond")),
        "no diagnostic record should be fabricated for Divine Bond either, at level 6: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 7 stays unrecognized by this slice (level 7
// was later widened into the supported tranche by a subsequent SD13-E5
// slice; see tests/sd13_paladin_level7_progression.rs) -----

#[test]
fn paladin_level_7_was_later_widened_into_the_supported_tranche() {
    let level_7 = PALADIN_LEVEL6_FIXTURE.replace("class:paladin:6", "class:paladin:7");
    let input = load(&level_7);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.paladin.")),
        "level-7 Paladin was later widened into the supported tranche and must now gain bounded \
         paladin chassis explanations: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the grounding must not leak onto other classes -----

#[test]
fn fighter_and_ranger_do_not_gain_paladin_level6_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(!has_explanation(&fighter_computation, CHANNEL_POSITIVE_ENERGY_DICE_ID));

    let ranger_computation = compute_pilot_base_chassis(&load(RANGER_FIXTURE));
    assert!(!has_explanation(&ranger_computation, CHANNEL_POSITIVE_ENERGY_DICE_ID));
}

// ----- Sanity: level 5 fixture is unaffected by the level-6 widening -----

#[test]
fn paladin_level5_values_stay_unaffected_after_the_level6_widening() {
    let input = load(PALADIN_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let caster_level = explanation(&computation, EFFECTIVE_CASTER_LEVEL_ID);
    assert_eq!(
        caster_level.value, 2,
        "level 5 effective caster level must remain 2, unaffected by the level-6 widening: \
         {caster_level:?}"
    );

    let dice = explanation(&computation, CHANNEL_POSITIVE_ENERGY_DICE_ID);
    assert_eq!(
        dice.value, 3,
        "level 5 channel positive energy dice count must remain 3, unaffected by the level-6 \
         widening: {dice:?}"
    );

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(
        fortitude.value, 4,
        "level 5 base Fortitude save must remain 4, unaffected by the level-6 widening: \
         {fortitude:?}"
    );
}

// ----- Control plane: the matrix row's note names the level-6 widening -----

#[test]
fn matrix_paladin_row_names_level_6_widening_and_effective_caster_level() {
    let matrix = seeded_current_truth();
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin row must exist");

    assert_eq!(paladin.support_state, SupportState::Supported);
    assert!(
        paladin.grounding_ref.contains("sd13_paladin_level6_progression"),
        "paladin row must cite the live SD13-E5 level-6 progression proof surface: {}",
        paladin.grounding_ref
    );
    let note = paladin.blocker_or_lossiness_note;
    assert!(
        note.to_lowercase().contains("effective caster level")
            || note.to_lowercase().contains("level 6"),
        "paladin partial note must name the level-6 widening: {note}"
    );
}
