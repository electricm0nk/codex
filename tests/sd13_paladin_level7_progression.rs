//! SD13-E5 Paladin level-7 progression grounding proof.
//!
//! Widens the accepted Paladin level-1/level-2/level-3/level-4/level-5/level-6
//! chassis-and-spell-burden separation
//! (`tests/sd13_paladin_level1_chassis_and_spell_burden_separation.rs`,
//! `tests/sd13_paladin_level2_lay_on_hands_divine_grace.rs`,
//! `tests/sd13_paladin_base_attack_and_saves.rs`,
//! `tests/sd13_paladin_level3_mercy.rs`,
//! `tests/sd13_paladin_level4_progression.rs`,
//! `tests/sd13_paladin_level5_progression.rs`,
//! `tests/sd13_paladin_level6_progression.rs`) to paladin level 7, mirroring
//! the Fighter/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Sorcerer/Wizard/Ranger
//! level-range-gate idiom (`supported_paladin_level` is generalized from
//! `1..=6` to `1..=7` via `MAX_SUPPORTED_PALADIN_LEVEL = 7`). Both PF1 CRB
//! primary sources (d20pfsrd and legacy.aonprd.com Paladin class table) were
//! read directly before writing any code or test: level 7 base attack bonus
//! is +7/+2, Fortitude/Will are +5, Reflex is +2, and the level-7 "Special"
//! column reads "Smite evil 3/day" only (Aura of Resolve does not land until
//! 8th level; the next repeat Mercy grant does not land until 9th level). It
//! proves:
//!
//! - base attack bonus at level 7 is grounded by the same full-BAB formula
//!   already grounded at levels 1-6: `7` (the iterative-attack notation
//!   +7/+2 is not modeled anywhere in this codebase; only the flat leading
//!   number is grounded, mirroring every other full-BAB class row).
//! - base saves at level 7 are grounded by the same good-Fortitude/good-Will/
//!   poor-Reflex formulas already grounded at levels 1-6, and all three stay
//!   NUMERICALLY UNCHANGED from level 6: Fortitude/Will `7 / 2 + 2 = 5`
//!   (unchanged from 5 at level 6) and Reflex `7 / 3 = 2` (unchanged from 2
//!   at level 6) -- integer-division coincidences, verified independently
//!   against both primary sources' level-7 table row, not signs either
//!   formula stopped scaling.
//! - Smite Evil's uses-per-day formula `1 + (paladin level - 1) / 3`
//!   GENUINELY increases to 3/day at level 7 (`1 + 6/3 = 3`), up from 2/day
//!   at level 6; both primary sources' level-7 "Special" column reads
//!   "Smite evil 3/day", verified independently rather than assumed to stay
//!   at 2 (the PF1 CRB rule text: "At 4th level, and at every three levels
//!   thereafter, the paladin may smite evil one additional time per day").
//!   Attack bonus stays the Charisma modifier and damage bonus keeps scaling
//!   via the same pre-existing formula (damage bonus = paladin level = 7).
//! - lay on hands and divine grace stay NUMERICALLY UNCHANGED at level 7 via
//!   the same pre-existing formulas: uses/day = 1/2 paladin level + Charisma
//!   modifier = `7/2 + 2 = 5` (unchanged from 5 at level 6) and heal dice =
//!   paladin level / 2 = `7/2 = 3` (unchanged from 3 at level 6) --
//!   integer-division coincidences, not stalled formulas. Divine grace save
//!   bonus stays the positive Charisma modifier, unchanged.
//! - the effective caster level gate GENUINELY changes at level 7:
//!   `max(7 - 3, 0) = 4` (PF1 Core Rulebook: paladin spells begin at level
//!   4, effective caster level = paladin level - 3) -- a real value change
//!   from `3` at level 6, grounded via the same pre-existing formula (no
//!   re-derivation). This grounds only the caster-level gate arithmetic; it
//!   fabricates no spells known, no spells per day, no bonus spell slots,
//!   and no spell save DCs, and the partial-caster spell blocker keeps
//!   firing.
//! - Channel Positive Energy's flat die count GENUINELY increases at level 7:
//!   `ceil(paladin level / 2) = ceil(7/2) = 4`, up from 3 at level 6 --
//!   verified independently rather than assumed, mirroring the Cleric
//!   Channel Energy die-count widening idiom exactly.
//! - Mercy, granted once at 3rd level, stays granted (not re-derived) at
//!   level 7: the grant and single choice-recognition record both persist
//!   unchanged (`mercy:shaken`). This cycle was specifically briefed to
//!   check whether Paladin gains an actual new class feature at 7th level:
//!   verified independently against d20pfsrd and legacy.aonprd.com, the
//!   level-7 "Special" column reads "Smite evil 3/day" only -- the repeat
//!   Mercy grant (PF1 CRB: "At 3rd level, and every three levels
//!   thereafter") lands at 3, 6, 9, 12... and level 7 is NOT one of those
//!   levels, so there is nothing new to leave named-but-unproven for Mercy
//!   at level 7 (unlike level 6, which WAS one of those levels).
//! - Divine Bond (the level-5 grant) stays not fabricated at level 7,
//!   unaffected by this widening.
//!
//! It deliberately does not implement any lay-on-hands execution engine, any
//! channel-positive-energy healing/damage-resolution engine, any Divine Bond
//! execution engine, any mercy-list-growth/multi-slot mechanism, and it does
//! not ground Paladin level 8+ or the partial-caster spell burden's actual
//! content (spells known/prepared, spells per day, bonus spell slots, spell
//! save DCs -- the effective-caster-level gate widening to 4 does NOT mean
//! spell slots are grounded here). It also preserves the accepted Paladin
//! level-1/level-2/level-3/level-4/level-5/level-6 truth (unchanged), the F6
//! hybrid baseline, the Ranger negative control, and the Fighter negative
//! control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{SupportState, seeded_current_truth};

const PALADIN_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level6_sd13_deterministic_input.txt");

const PALADIN_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level7_sd13_deterministic_input.txt");

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

fn load(fixture: &str) -> CharacterInput {
    let result = load_character_input_fixture(fixture);
    assert!(
        result.diagnostics.is_empty(),
        "fixture should load cleanly: {:?}",
        result.diagnostics
    );
    result
        .character_input
        .expect("valid fixture should produce a character input record")
}

fn has_explanation(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
}

fn explanation<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationExplanation {
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected explanation id '{id}', got {:?}",
                computation.explanations
            )
        })
}

// ----- Base attack / base save at level 7 (both good saves stay numerically unchanged) -----

#[test]
fn paladin_level7_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(PALADIN_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 7,
        "Paladin level 7 full BAB (classlevel) must equal 7: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(
        fortitude.value, 5,
        "Paladin level 7 good Fortitude (7/2+2) must equal 5, numerically unchanged from level 6"
    );

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(
        reflex.value, 2,
        "Paladin level 7 poor Reflex (7/3) must equal 2, numerically unchanged from level 6"
    );

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(
        will.value, 5,
        "Paladin level 7 good Will (7/2+2) must equal 5, numerically unchanged from level 6"
    );
}

// ----- Smite Evil GENUINELY increases to 3/day at level 7 -----

#[test]
fn paladin_level7_smite_evil_increases_to_3_per_day() {
    let input = load(PALADIN_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 3,
        "smite evil uses per day must genuinely increase to 3/day at level 7 (1 + (7-1)/3 = 3), \
         up from 2/day at level 6, matching the PF1 CRB level-7 \"Special\" column \"Smite evil \
         3/day\": {uses_per_day:?}"
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
        damage_bonus.value, 7,
        "smite evil damage bonus must equal paladin level (7 at level 7): {damage_bonus:?}"
    );
}

// ----- Lay on hands / divine grace stay numerically unchanged at level 7 -----

#[test]
fn paladin_level7_lay_on_hands_and_divine_grace_stay_unchanged() {
    let input = load(PALADIN_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, LAY_ON_HANDS_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 6,
        "lay on hands uses per day at level 7 (1/2 paladin level + Charisma modifier = 3 + 3) \
         must equal 6, numerically unchanged from level 6: {uses_per_day:?}"
    );

    let heal_amount = explanation(&computation, LAY_ON_HANDS_HEAL_AMOUNT_ID);
    assert_eq!(
        heal_amount.value, 3,
        "lay on hands heal amount at level 7 (paladin level / 2 = 3) must equal 3, numerically \
         unchanged from level 6: {heal_amount:?}"
    );

    let save_bonus = explanation(&computation, DIVINE_GRACE_SAVE_BONUS_ID);
    assert_eq!(
        save_bonus.value, 3,
        "divine grace save bonus at level 7 must still equal the positive Charisma modifier, \
         unchanged: {save_bonus:?}"
    );
}

// ----- Effective caster level GENUINELY becomes 4 at level 7 -----

#[test]
fn paladin_level7_effective_caster_level_genuinely_becomes_4() {
    let input = load(PALADIN_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let caster_level = explanation(&computation, EFFECTIVE_CASTER_LEVEL_ID);
    assert_eq!(
        caster_level.value, 4,
        "effective caster level at paladin level 7 (max(7 - 3, 0)) must genuinely become 4, up \
         from 3 at level 6: {caster_level:?}"
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
                 again at level 7: {daily_prep:?}"
            );
        }
    }
}

// ----- Mercy stays granted (not re-derived) at level 7; no second slot fabricated -----

#[test]
fn paladin_level7_mercy_stays_granted_unchanged() {
    let input = load(PALADIN_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let granted = explanation(&computation, MERCY_GRANTED_ID);
    assert_eq!(
        granted.value, 0,
        "mercy grant record still carries no fabricated mechanical value at level 7: {granted:?}"
    );

    let choice = explanation(&computation, MERCY_CHOICE_ID);
    assert!(
        choice.detail.contains("mercy:shaken"),
        "mercy choice detail must still name the selected mercy (mercy:shaken) at level 7: {}",
        choice.detail
    );
}

#[test]
fn paladin_level7_does_not_fabricate_a_second_mercy_slot() {
    let input = load(PALADIN_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        computation
            .explanations
            .iter()
            .filter(|e| e.id == MERCY_CHOICE_ID)
            .count(),
        1,
        "level 7 is not one of the PF1 CRB's repeat-mercy-grant levels (3rd level and every \
         three levels thereafter: 3, 6, 9...); no second mercy-choice explanation record must be \
         fabricated: {:?}",
        computation.explanations
    );
}

// ----- Channel Positive Energy dice count GENUINELY increases to 4 at level 7 -----

#[test]
fn paladin_level7_channel_positive_energy_dice_increases_to_4() {
    let input = load(PALADIN_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, CHANNEL_POSITIVE_ENERGY_DICE_ID);
    assert_eq!(
        dice.value, 4,
        "channel positive energy dice count at level 7 (ceil(paladin level / 2) = ceil(7/2)) \
         must genuinely increase to 4, up from 3 at level 6: {dice:?}"
    );
}

// ----- Divine Bond: still not fabricated at level 7 -----

#[test]
fn paladin_level7_does_not_fabricate_divine_bond() {
    let input = load(PALADIN_LEVEL7_FIXTURE);
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
         no explanation record must be fabricated for it at level 7 either: {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.contains("divine_bond")),
        "no diagnostic record should be fabricated for Divine Bond either, at level 7: {:?}",
        computation.diagnostics
    );
}

// ----- Level 8 was later widened into the supported tranche by a further slice -----

#[test]
fn paladin_level_8_was_later_widened_into_the_supported_tranche() {
    let level_8 = PALADIN_LEVEL7_FIXTURE.replace("class:paladin:7", "class:paladin:8");
    let input = load(&level_8);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.paladin.")),
        "level-8 Paladin is now recognized by the later level-8 widening slice \
         (tests/sd13_paladin_level8_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the grounding must not leak onto other classes -----

#[test]
fn fighter_and_ranger_do_not_gain_paladin_level7_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(!has_explanation(&fighter_computation, CHANNEL_POSITIVE_ENERGY_DICE_ID));

    let ranger_computation = compute_pilot_base_chassis(&load(RANGER_FIXTURE));
    assert!(!has_explanation(&ranger_computation, CHANNEL_POSITIVE_ENERGY_DICE_ID));
}

// ----- Sanity: level 6 fixture is unaffected by the level-7 widening -----

#[test]
fn paladin_level6_values_stay_unaffected_after_the_level7_widening() {
    let input = load(PALADIN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let caster_level = explanation(&computation, EFFECTIVE_CASTER_LEVEL_ID);
    assert_eq!(
        caster_level.value, 3,
        "level 6 effective caster level must remain 3, unaffected by the level-7 widening: \
         {caster_level:?}"
    );

    let dice = explanation(&computation, CHANNEL_POSITIVE_ENERGY_DICE_ID);
    assert_eq!(
        dice.value, 3,
        "level 6 channel positive energy dice count must remain 3, unaffected by the level-7 \
         widening: {dice:?}"
    );

    let smite = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(
        smite.value, 2,
        "level 6 smite evil uses per day must remain 2, unaffected by the level-7 widening: \
         {smite:?}"
    );
}

// ----- Control plane: the matrix row's note names the level-7 widening -----

#[test]
fn matrix_paladin_row_names_level_7_widening_and_effective_caster_level() {
    let matrix = seeded_current_truth();
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin row must exist");

    assert_eq!(paladin.support_state, SupportState::Supported);
    assert!(
        paladin.grounding_ref.contains("sd13_paladin_level7_progression"),
        "paladin row must cite the live SD13-E5 level-7 progression proof surface: {}",
        paladin.grounding_ref
    );
    let note = paladin.blocker_or_lossiness_note;
    assert!(
        note.to_lowercase().contains("effective caster level")
            || note.to_lowercase().contains("level 7"),
        "paladin partial note must name the level-7 widening: {note}"
    );
}
