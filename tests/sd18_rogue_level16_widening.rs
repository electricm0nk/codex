//! SD18 Rogue level-16 widening grounding proof.
//!
//! Widens the accepted Human Rogue level-1..level-15 chassis
//! (`tests/sd18_rogue_level15_widening.rs`, the loop's most recent Rogue
//! ceiling) to Rogue level 16, the loop's Rogue level-16 sweep landing (the
//! FOURTH §3.2 level-16 landing, after Barbarian, Fighter, and Wizard),
//! mirroring the sibling-class level-range-gate idiom (`supported_rogue_level`
//! is generalized from `1..=15` to `1..=16` via `MAX_SUPPORTED_ROGUE_LEVEL =
//! 16`, exactly as every prior level-11..15 cycle widened its own
//! `MAX_SUPPORTED_<CLASS>_LEVEL`). Both PF1 CRB primary sources (d20pfsrd and
//! the Archives of Nethys aonprd.com mirror) were read directly before
//! writing any code or test, and both agree byte-for-byte:
//!
//! - level 16 base attack bonus genuinely RISES to +12 (`16 * 3 / 4 = 12`,
//!   up from 11 at level 15); Fortitude STAYS +5 (`16 / 3 = 5`, an
//!   integer-division coincidence with level 15); Reflex genuinely RISES to
//!   +10 (`16 / 2 + 2 = 10`, up from 9 at level 15); Will STAYS +5
//!   (`16 / 3 = 5`, an integer-division coincidence with level 15) -- all
//!   four checked directly against both primary sources, not assumed.
//! - the PF1 Core Rulebook Rogue class table's level-16 "Special" column
//!   reads only "Rogue talent" (both primary sources agree byte-for-byte,
//!   the identical shape to the level-14 landing): level 16 IS a
//!   rogue-talent cadence level (talents land at 2/4/6/8/10/12/14/16), so an
//!   EIGHTH numbered choice-recognition slot (`choice:rogue_talent_8`) is
//!   added, mirroring the proven open-ended raw-string idiom used at slots
//!   1-7 exactly -- no talent-effect engine invented.
//! - the sneak-attack die-count formula (`(level + 1) / 2`) STAYS at 8
//!   (`17 / 2 = 8`, i.e. 8d6, an integer-division coincidence with level 15,
//!   next rise at level 17) and the Trap Sense flat-magnitude formula
//!   (`level / 3`) STAYS at +5 (`16 / 3 = 5`, an integer-division
//!   coincidence with level 15, next rise at level 18) -- neither is named
//!   in the level-16 "Special" column, consistent with both staying
//!   unchanged.
//! - Trapfinding genuinely RISES to +8 (`max(16/2, 1) = 8`, up from +7 at
//!   level 15, via its own independent formula -- not named in the level-16
//!   "Special" column); Evasion, Uncanny Dodge, and Improved Uncanny Dodge
//!   all stay granted, not re-derived.
//!
//! This widening needs ZERO new tier constants for base attack/saves/sneak
//! attack/trap sense/trapfinding (all already level-generic), and exactly
//! ONE new numbered choice slot (the eighth rogue-talent slot, added to the
//! existing `additional_talent_slots` array via the same tuple idiom used
//! for slots 3-7) -- the same low-risk shape as every prior talent-cadence
//! landing (levels 6/8/10/12/14).
//!
//! This slice deliberately does not touch the rogue-talent tree's own
//! effects (standard or advanced), any check-execution engine, or
//! sneak-attack damage application (all stay named-but-unproven, unchanged
//! from levels 1-15), and it does not ground Rogue level 17+. It also
//! preserves the accepted Rogue level-1..level-15 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const ROGUE_LEVEL15_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level15_sd18_widening_deterministic_input.txt"
);

const ROGUE_LEVEL16_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level16_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const ROGUE_EVASION_ID: &str = "class_feature.rogue.evasion";
const ROGUE_TRAP_SENSE_ID: &str = "class_feature.rogue.trap_sense";
const ROGUE_UNCANNY_DODGE_ID: &str = "class_feature.rogue.uncanny_dodge";
const ROGUE_IMPROVED_UNCANNY_DODGE_ID: &str = "class_feature.rogue.improved_uncanny_dodge";

// ----- Base attack bonus and Reflex genuinely rise at level 16; Fort/Will stay -----

#[test]
fn rogue_level16_base_attack_bonus_and_reflex_genuinely_rise() {
    let input = load(ROGUE_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(
        base_attack.value, 12,
        "Rogue level 16 3/4-BAB progression (16 * 3 / 4) must genuinely rise to 12, up from 11 \
         at level 15: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(
        fortitude.value, 5,
        "Rogue level 16 poor Fortitude (16/3) must stay 5, unchanged from level 15, an \
         integer-division coincidence"
    );

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(
        reflex.value, 10,
        "Rogue level 16 good Reflex (16/2+2) must genuinely rise to 10, up from 9 at level 15"
    );

    let will = explanation(&computation, "class_chassis.rogue.base_save.will");
    assert_eq!(
        will.value, 5,
        "Rogue level 16 poor Will (16/3) must stay 5, unchanged from level 15, an \
         integer-division coincidence"
    );
}

// ----- Sneak attack and Trap Sense both stay unchanged (not named in the level-16 Special column) -----

#[test]
fn rogue_level16_sneak_attack_and_trap_sense_stay_unchanged() {
    let input = load(ROGUE_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 8,
        "Rogue level 16 sneak attack die count ((16 + 1) / 2) must stay 8 (8d6), unchanged from \
         level 15, an integer-division coincidence -- the level-16 'Special' column reads only \
         'Rogue talent': {}",
        sneak_attack.detail
    );

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 5,
        "Rogue level 16 Trap Sense (16 / 3) must stay +5, unchanged from level 15, an \
         integer-division coincidence: {}",
        trap_sense.detail
    );
}

// ----- Trapfinding genuinely rises -----

#[test]
fn rogue_level16_trapfinding_genuinely_rises() {
    let input = load(ROGUE_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(
        trapfinding.value, 8,
        "Rogue level 16 Trapfinding (max(16/2, 1)) must genuinely rise to 8, up from 7 at level \
         15: {}",
        trapfinding.detail
    );
}

// ----- The eighth talent slot appears at level 16 (a rogue-talent level) -----

#[test]
fn rogue_level16_surfaces_an_eighth_talent_slot() {
    let input = load(ROGUE_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot_8 = explanation(&computation, "class_chassis.rogue.talent_8_choice");
    assert_eq!(slot_8.value, 0, "the eighth talent slot must be a bounded +0 recognition record");
    assert!(
        slot_8.detail.contains("offensive_defense"),
        "the eighth talent slot detail must name the selected talent's raw string: {}",
        slot_8.detail
    );

    // The seventh slot, selected at level 14, stays recognized (not re-derived).
    let slot_7 = explanation(&computation, "class_chassis.rogue.talent_7_choice");
    assert_eq!(slot_7.value, 0, "the seventh talent slot must stay a +0 recognition record");
}

// ----- Granted features stay granted at level 16 -----

#[test]
fn rogue_level16_still_recognizes_the_granted_feature_records() {
    let input = load(ROGUE_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        ROGUE_EVASION_ID,
        ROGUE_UNCANNY_DODGE_ID,
        ROGUE_IMPROVED_UNCANNY_DODGE_ID,
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, 0,
            "'{id}' must stay a granted +0 identity record at level 16"
        );
    }
}

// ----- Negative control: the level-15 fixture is unaffected by this widening -----

#[test]
fn rogue_level15_truth_is_unchanged_by_this_slice() {
    let input = load(ROGUE_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(sneak_attack.value, 8, "Rogue level 15 sneak attack must stay 8d6");

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(base_attack.value, 11, "Rogue level 15 base attack bonus must stay 11");

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(trap_sense.value, 5, "Rogue level 15 Trap Sense must stay +5");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.rogue.talent_8_choice"),
        "the level-15 fixture must not surface an eighth talent slot"
    );
}

// ----- Negative control: the rogue path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_level16_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")
                || e.id.starts_with("class_feature.rogue.")),
        "the Fighter chassis must not surface any rogue-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Rogue is not promoted -----

#[test]
fn multiclass_rogue_level16_is_not_promoted_by_this_slice() {
    let multiclass = ROGUE_LEVEL16_FIXTURE.replace(
        "class_level=class:rogue:16",
        "class_level=class:rogue:16\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| (e.id.starts_with("class_chassis.rogue.")
                || e.id.starts_with("class_feature.rogue."))
                // SD-34 wave 34 lane A (`docs/release/SD-34-book-completion/artifacts/
                // bucket-d-mining/wave34_laneA_weapon_and_armor_proficiency_cycle_
                // receipt.md`): Rogue's own Weapon and Armor Proficiency identity
                // grant is now genuinely grounded as a level-independent, always-on
                // +0 record (true since level 1, mirrors the same "no gate to lift"
                // idiom as Jack-of-All-Trades) -- not a bounded, level-gated feature
                // this slice's negative control is checking for.
                && e.id != "class_feature.rogue.weapon_and_armor_proficiency"),
        "multiclass Rogue must not gain any bounded rogue explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Rogue must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-16 widening -----

#[test]
fn matrix_rogue_row_names_level_16_widening() {
    let matrix = seeded_current_truth();
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue bounded_progression row must exist");

    assert_eq!(rogue.support_state, SupportState::Supported); // promoted by SD-19 Class Progression Catalog browser
    assert_eq!(rogue.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(rogue.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        rogue.grounding_ref.contains("sd18_rogue_level16_widening"),
        "matrix grounding_ref must name the level-16 widening test: {}",
        rogue.grounding_ref
    );
}
