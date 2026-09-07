//! SD13-E5 Paladin level-4 progression grounding proof.
//!
//! Widens the accepted Paladin level-1/level-2/level-3 chassis-and-spell-burden
//! separation (`tests/sd13_paladin_level1_chassis_and_spell_burden_separation.rs`,
//! `tests/sd13_paladin_level2_lay_on_hands_divine_grace.rs`,
//! `tests/sd13_paladin_base_attack_and_saves.rs`,
//! `tests/sd13_paladin_level3_mercy.rs`) to paladin level 4, mirroring the
//! Fighter/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Sorcerer/Wizard/Ranger
//! level-range-gate idiom (`supported_paladin_level` is generalized from
//! `1..=3` to `1..=4` via `MAX_SUPPORTED_PALADIN_LEVEL = 4`). It proves,
//! each formula independently re-verified against the PF1 Core Rulebook
//! Paladin class table (d20pfsrd and legacy.aonprd.com) before writing any
//! code:
//!
//! - base attack bonus at level 4 is grounded by the same full-BAB formula
//!   already grounded at levels 1-3: `4`.
//! - base saves at level 4 are grounded by the same good-Fortitude/good-Will/
//!   poor-Reflex formulas already grounded at levels 1-3, extended to level 4:
//!   Fortitude/Will `4 / 2 + 2 = 4`, Reflex `4 / 3 = 1`.
//! - Smite Evil's uses-per-day GENUINELY increases at level 4: the PF1 CRB
//!   class table's level-4 "Special" column reads "Channel positive energy,
//!   smite evil 2/day" -- verified independently rather than assumed to stay
//!   at 1/day. The uses-per-day formula is generalized to
//!   `1 + (paladin level - 1) / 3`, which correctly yields 1 at levels 1-3 and
//!   2 at level 4 (the next increase does not land until level 7). Attack
//!   bonus and damage bonus keep scaling via the same pre-existing formulas
//!   (damage bonus = paladin level = 4).
//! - lay on hands / divine grace still scale correctly at level 4 via the same
//!   pre-existing formulas: uses/day = 1/2 paladin level + Charisma modifier =
//!   2 + 2 = 4, heal dice = paladin level / 2 = 2, divine grace save bonus
//!   stays the positive Charisma modifier.
//! - the effective caster level gate GENUINELY changes at level 4: `max(4 - 3,
//!   0) = 1` (PF1 Core Rulebook: paladin spells begin at level 4) -- a real
//!   value change from 0, grounded via the same pre-existing formula (no
//!   re-derivation). This grounds only the caster-level gate arithmetic; it
//!   fabricates no spells known, no spells per day, no bonus spell slots, and
//!   no spell save DCs, and the partial-caster spell blocker keeps firing.
//! - Mercy, granted once at 3rd level, stays granted (not re-derived) at level
//!   4: the grant and choice-recognition records both persist unchanged.
//! - Channel Positive Energy, the PF1 Core Rulebook's OTHER 4th-level Paladin
//!   class feature (verified independently against legacy.aonprd.com's Core
//!   Rulebook Paladin page: "When a paladin reaches 4th level, she gains the
//!   supernatural ability to channel positive energy like a cleric. Using
//!   this ability consumes two uses of her lay on hands ability. A paladin
//!   uses her level as her effective cleric level when channeling positive
//!   energy."), transitions from a level-gate ABSENCE record (value 0) at
//!   levels 1-3 to a bounded, flat-magnitude record at level 4, grounding only
//!   the channel-energy die count via the same ceil(effective level / 2)
//!   formula already grounded for Cleric's own Channel Energy: 2d6 at
//!   paladin level 4. No healing/damage-resolution execution, no
//!   heal-vs-harm target selection, and no lay-on-hands-resource-consumption
//!   bookkeeping is computed.
//!
//! It deliberately does not implement any lay-on-hands execution engine, any
//! channel-positive-energy healing/damage-resolution engine, and it does not
//! ground Paladin level 5+ or the partial-caster spell burden's actual
//! content (spells known/prepared, spells per day, bonus spell slots, spell
//! save DCs -- the effective-caster-level gate becoming nonzero does NOT mean
//! spell slots are grounded here). It also preserves the accepted Paladin
//! level-1/level-2/level-3 truth (unchanged), the F6 hybrid baseline, the
//! Ranger negative control, and the Fighter negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{SupportState, seeded_current_truth};
mod common;
use common::{load, explanation, has_explanation};

const PALADIN_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level3_sd13_deterministic_input.txt");

const PALADIN_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level4_sd13_deterministic_input.txt");

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

// The level-1/level-2/level-3-only channel positive energy absence gate; must
// not appear at level 4.
const CHANNEL_POSITIVE_ENERGY_GATE_ID: &str = "class_chassis.paladin.level_gate.channel_positive_energy";

// The newly grounded level-4 channel positive energy dice-count record.
const CHANNEL_POSITIVE_ENERGY_DICE_ID: &str = "class_chassis.paladin.channel_positive_energy_dice";

// ----- Base attack / base save at level 4 -----

#[test]
fn paladin_level4_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(PALADIN_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 4,
        "Paladin level 4 full BAB (classlevel) must equal 4: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 4, "Paladin level 4 good Fortitude (4/2+2) must equal 4");

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 1, "Paladin level 4 poor Reflex (4/3) must equal 1");

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 4, "Paladin level 4 good Will (4/2+2) must equal 4");
}

// ----- Smite Evil genuinely increases to 2/day at level 4 -----

#[test]
fn paladin_level4_smite_evil_uses_per_day_increases_to_2() {
    let input = load(PALADIN_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 2,
        "smite evil uses per day genuinely increases to 2/day at level 4 (PF1 CRB level-4 \
         \"Special\" column: \"smite evil 2/day\"): {uses_per_day:?}"
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
        damage_bonus.value, 4,
        "smite evil damage bonus must equal paladin level (4 at level 4): {damage_bonus:?}"
    );
}

// ----- Lay on hands / divine grace still scale correctly at level 4 -----

#[test]
fn paladin_level4_lay_on_hands_and_divine_grace_scale_correctly() {
    let input = load(PALADIN_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, LAY_ON_HANDS_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 5,
        "lay on hands uses per day at level 4 (1/2 paladin level + Charisma modifier = 2 + 3) \
         must equal 5: {uses_per_day:?}"
    );

    let heal_amount = explanation(&computation, LAY_ON_HANDS_HEAL_AMOUNT_ID);
    assert_eq!(
        heal_amount.value, 2,
        "lay on hands heal amount at level 4 (paladin level / 2 = 2) must equal 2: {heal_amount:?}"
    );

    let save_bonus = explanation(&computation, DIVINE_GRACE_SAVE_BONUS_ID);
    assert_eq!(
        save_bonus.value, 3,
        "divine grace save bonus at level 4 must still equal the positive Charisma modifier: \
         {save_bonus:?}"
    );
}

// ----- Effective caster level GENUINELY becomes 1 at level 4 -----

#[test]
fn paladin_level4_effective_caster_level_genuinely_becomes_1() {
    let input = load(PALADIN_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let caster_level = explanation(&computation, EFFECTIVE_CASTER_LEVEL_ID);
    assert_eq!(
        caster_level.value, 1,
        "effective caster level at paladin level 4 (max(4 - 3, 0)) must genuinely become 1 (PF1 \
         Core Rulebook: paladin spells begin at level 4): {caster_level:?}"
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
                "no spells are fabricated merely because the effective-caster-level gate is now \
                 nonzero: {daily_prep:?}"
            );
        }
    }
}

// ----- Mercy stays granted (not re-derived) at level 4 -----

#[test]
fn paladin_level4_mercy_stays_granted_unchanged() {
    let input = load(PALADIN_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let granted = explanation(&computation, MERCY_GRANTED_ID);
    assert_eq!(
        granted.value, 0,
        "mercy grant record still carries no fabricated mechanical value at level 4: {granted:?}"
    );

    let choice = explanation(&computation, MERCY_CHOICE_ID);
    assert!(
        choice.detail.contains("mercy:shaken"),
        "mercy choice detail must still name the selected mercy (mercy:shaken) at level 4: {}",
        choice.detail
    );
}

// ----- Channel Positive Energy transitions from a level-gate absence to a grounded dice count -----

#[test]
fn paladin_level4_grounds_channel_positive_energy_dice_count() {
    let input = load(PALADIN_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, CHANNEL_POSITIVE_ENERGY_GATE_ID),
        "level-4 Paladin must not gain the level-1/2/3-only channel positive energy absence \
         gate: {:?}",
        computation.explanations
    );

    let dice = explanation(&computation, CHANNEL_POSITIVE_ENERGY_DICE_ID);
    assert_eq!(
        dice.value, 2,
        "channel positive energy dice count at level 4 (ceil(paladin level / 2) = ceil(4/2)) \
         must equal 2: {dice:?}"
    );
    assert!(
        dice.detail.to_lowercase().contains("4th-level")
            || dice.detail.to_lowercase().contains("4th level"),
        "channel positive energy detail must name the verified 4th-level PF1 CRB grant: {}",
        dice.detail
    );
    assert!(
        dice.detail.to_lowercase().contains("two uses")
            || dice.detail.to_lowercase().contains("lay on hands"),
        "channel positive energy detail must name the lay-on-hands resource cost rule: {}",
        dice.detail
    );
    assert!(
        dice.detail.to_lowercase().contains("no healing")
            || dice.detail.to_lowercase().contains("not computed")
            || dice.detail.to_lowercase().contains("no execution"),
        "channel positive energy detail must disclaim computing the actual healing/damage \
         execution: {}",
        dice.detail
    );
}

// ----- Negative control: level 3 keeps the channel positive energy absence gate, unaffected -----

#[test]
fn paladin_level3_channel_positive_energy_absence_gate_is_unaffected_by_the_level4_widening() {
    let input = load(PALADIN_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let gate = explanation(&computation, CHANNEL_POSITIVE_ENERGY_GATE_ID);
    assert_eq!(
        gate.value, 0,
        "level 3 channel positive energy must remain a correct absence: {gate:?}"
    );
    assert!(
        !has_explanation(&computation, CHANNEL_POSITIVE_ENERGY_DICE_ID),
        "level-3 Paladin must not gain the level-4-only channel positive energy dice record"
    );
}

// ----- Negative control (superseded): level 5 was later widened into the supported tranche -----

#[test]
fn paladin_level_5_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 5 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_paladin_level5_progression.rs) widened the level-range
    // gate to level 5 (mirroring the Fighter/Rogue/Barbarian/Monk/Cleric/
    // Bard/Druid/Sorcerer/Wizard/Ranger level-range-gate idiom) and grounded
    // the genuine effective-caster-level and Channel Positive Energy dice
    // increases; this negative control is superseded, not violated -- pin
    // the new truth here too so this file stays internally consistent. The
    // frontier this file's own slice actually drew is now level 6, covered
    // by `paladin_level_6_is_not_promoted_by_this_slice` in
    // `tests/sd13_paladin_level5_progression.rs`.
    let level_5 = PALADIN_LEVEL4_FIXTURE.replace("class:paladin:4", "class:paladin:5");
    let input = load(&level_5);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, BASE_ATTACK_ID),
        "level-5 Paladin is supported since the SD13-E5 level-5 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, CHANNEL_POSITIVE_ENERGY_DICE_ID),
        "level-5 Paladin's channel positive energy dice count is supported since the SD13-E5 \
         level-5 slice: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the grounding must not leak onto other classes -----

#[test]
fn fighter_and_ranger_do_not_gain_paladin_level4_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(!has_explanation(&fighter_computation, CHANNEL_POSITIVE_ENERGY_DICE_ID));

    let ranger_computation = compute_pilot_base_chassis(&load(RANGER_FIXTURE));
    assert!(!has_explanation(&ranger_computation, CHANNEL_POSITIVE_ENERGY_DICE_ID));
}

// ----- Control plane: the matrix row's note names the level-4 widening -----

#[test]
fn matrix_paladin_row_names_level_4_widening_and_channel_positive_energy() {
    let matrix = seeded_current_truth();
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin row must exist");

    assert_eq!(paladin.support_state, SupportState::Supported);
    assert!(
        paladin.grounding_ref.contains("sd13_paladin_level4_progression"),
        "paladin row must cite the live SD13-E5 level-4 progression proof surface: {}",
        paladin.grounding_ref
    );
    let note = paladin.blocker_or_lossiness_note;
    assert!(
        note.to_lowercase().contains("channel positive energy"),
        "paladin partial note must name channel positive energy as newly grounded at level 4: \
         {note}"
    );
}
