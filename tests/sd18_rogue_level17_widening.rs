//! SD18 Rogue level-17 widening grounding proof.
//!
//! Widens the accepted Human Rogue level-1..level-16 chassis
//! (`tests/sd18_rogue_level16_widening.rs`, the loop's most recent Rogue
//! ceiling) to Rogue level 17, the loop's THIRD §3.2 level-17 sweep landing
//! (after Ranger and Bard), mirroring the sibling-class level-range-gate
//! idiom (`supported_rogue_level` is generalized from `1..=16` to `1..=17`
//! via `MAX_SUPPORTED_ROGUE_LEVEL = 17`, exactly as every prior level-11..16
//! cycle widened its own `MAX_SUPPORTED_<CLASS>_LEVEL`). Both PF1 CRB
//! primary sources (d20pfsrd and the Archives of Nethys aonprd.com mirror)
//! were read directly before writing any code or test, fetching the full
//! levels-14-through-18 block in one pass so the level-17 row's neighbors
//! were visible in context (guards against level-misattribution), and both
//! agree byte-for-byte:
//!
//! - level 14: BAB +10/+5, Fort +4, Ref +9, Will +4, Special "Rogue talent"
//! - level 15: BAB +11/+6/+1, Fort +5, Ref +9, Will +5, Special "Sneak
//!   attack +8d6, trap sense +5"
//! - level 16: BAB +12/+7/+2, Fort +5, Ref +10, Will +5, Special "Rogue
//!   talent"
//! - level 17: BAB +12/+7/+2, Fort +5, Ref +10, Will +5, Special "Sneak
//!   attack +9d6"
//! - level 18: BAB +13/+8/+3, Fort +6, Ref +11, Will +6, Special "Rogue
//!   talent, trap sense +6"
//!
//! At level 17: base attack bonus STAYS +12 (`17 * 3 / 4 = 12`, an
//! integer-division coincidence with level 16); Fortitude STAYS +5
//! (`17 / 3 = 5`, an integer-division coincidence with level 16); Reflex
//! STAYS +10 (`17 / 2 + 2 = 10`, an integer-division coincidence with level
//! 16); Will STAYS +5 (`17 / 3 = 5`, an integer-division coincidence with
//! level 16) -- all four checked directly against both primary sources, not
//! assumed. The level-17 "Special" column reads only "Sneak attack +9d6" --
//! level 17 is NOT a rogue-talent cadence level (talents land at
//! 2/4/6/8/10/12/14/16, next at 18), so no ninth talent slot is grounded or
//! fabricated; the entry is a tier-rise on the already-grounded
//! sneak-attack die-count formula, not a new class feature: the
//! sneak-attack die-count formula (`(level + 1) / 2`) genuinely rises to 9
//! (`18 / 2 = 9`, i.e. 9d6), up from 8d6 at level 16. Trap Sense stays +5
//! (`17 / 3 = 5`, next rise at level 18, not named in the level-17 "Special"
//! column); Trapfinding stays 8 (`max(17/2, 1) = 8`, an integer-division
//! coincidence with level 16, via its own independent formula, not named in
//! the level-17 "Special" column); Evasion, Uncanny Dodge, and Improved
//! Uncanny Dodge all stay granted, not re-derived.
//!
//! This widening needs ZERO new tier constants and ZERO new choice slots for
//! any pillar (base attack/saves/sneak-attack/trap-sense/trapfinding are all
//! already level-generic, and level 17 is not a talent-cadence level) -- the
//! ONLY production-code change is raising `MAX_SUPPORTED_ROGUE_LEVEL` from
//! 16 to 17. This is the cleanest possible widening shape checked in the
//! Rogue sweep so far, mirroring the Barbarian/Rogue level-15 landings'
//! "zero new record types" shape exactly.
//!
//! This slice deliberately does not touch the rogue-talent tree's own
//! effects (standard or advanced), any check-execution engine, or
//! sneak-attack damage application (all stay named-but-unproven, unchanged
//! from levels 1-16), and it does not ground Rogue level 18+. It also
//! preserves the accepted Rogue level-1..level-16 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const ROGUE_LEVEL16_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level16_sd18_widening_deterministic_input.txt"
);

const ROGUE_LEVEL17_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level17_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const ROGUE_EVASION_ID: &str = "class_feature.rogue.evasion";
const ROGUE_TRAP_SENSE_ID: &str = "class_feature.rogue.trap_sense";
const ROGUE_UNCANNY_DODGE_ID: &str = "class_feature.rogue.uncanny_dodge";
const ROGUE_IMPROVED_UNCANNY_DODGE_ID: &str = "class_feature.rogue.improved_uncanny_dodge";

// ----- Base attack bonus and base saves all stay unchanged at level 17 (integer-division coincidences) -----

#[test]
fn rogue_level17_base_attack_bonus_and_base_saves_stay_unchanged() {
    let input = load(ROGUE_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(
        base_attack.value, 12,
        "Rogue level 17 3/4-BAB progression (17 * 3 / 4) must stay 12, unchanged from level 16, \
         an integer-division coincidence: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(
        fortitude.value, 5,
        "Rogue level 17 poor Fortitude (17/3) must stay 5, unchanged from level 16, an \
         integer-division coincidence"
    );

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(
        reflex.value, 10,
        "Rogue level 17 good Reflex (17/2+2) must stay 10, unchanged from level 16, an \
         integer-division coincidence"
    );

    let will = explanation(&computation, "class_chassis.rogue.base_save.will");
    assert_eq!(
        will.value, 5,
        "Rogue level 17 poor Will (17/3) must stay 5, unchanged from level 16, an \
         integer-division coincidence"
    );
}

// ----- Sneak attack genuinely rises at level 17 (the level-17 "Special" column's only entry) -----

#[test]
fn rogue_level17_sneak_attack_genuinely_rises() {
    let input = load(ROGUE_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 9,
        "Rogue level 17 sneak attack die count ((17 + 1) / 2) must genuinely rise to 9 (9d6), \
         up from 8d6 at level 16 -- the level-17 'Special' column reads only 'Sneak attack \
         +9d6': {}",
        sneak_attack.detail
    );
}

// ----- Trap Sense and Trapfinding both stay unchanged (neither named in the level-17 Special column) -----

#[test]
fn rogue_level17_trap_sense_and_trapfinding_stay_unchanged() {
    let input = load(ROGUE_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 5,
        "Rogue level 17 Trap Sense (17 / 3) must stay +5, unchanged from level 16, an \
         integer-division coincidence: {}",
        trap_sense.detail
    );

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(
        trapfinding.value, 8,
        "Rogue level 17 Trapfinding (max(17/2, 1)) must stay 8, unchanged from level 16, an \
         integer-division coincidence: {}",
        trapfinding.detail
    );
}

// ----- No ninth talent slot appears at level 17 (not a rogue-talent cadence level) -----

#[test]
fn rogue_level17_does_not_surface_a_ninth_talent_slot() {
    let input = load(ROGUE_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.rogue.talent_9_choice"),
        "level 17 must not surface a ninth talent slot -- talents land at \
         2/4/6/8/10/12/14/16/18, not 17"
    );

    // The eighth slot, selected at level 16, stays recognized (not re-derived).
    let slot_8 = explanation(&computation, "class_chassis.rogue.talent_8_choice");
    assert_eq!(slot_8.value, 0, "the eighth talent slot must stay a +0 recognition record");
}

// ----- Granted features stay granted at level 17 -----

#[test]
fn rogue_level17_still_recognizes_the_granted_feature_records() {
    let input = load(ROGUE_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        ROGUE_EVASION_ID,
        ROGUE_UNCANNY_DODGE_ID,
        ROGUE_IMPROVED_UNCANNY_DODGE_ID,
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, 0,
            "'{id}' must stay a granted +0 identity record at level 17"
        );
    }
}

// ----- Negative control: the level-16 fixture is unaffected by this widening -----

#[test]
fn rogue_level16_truth_is_unchanged_by_this_slice() {
    let input = load(ROGUE_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(sneak_attack.value, 8, "Rogue level 16 sneak attack must stay 8d6");

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(base_attack.value, 12, "Rogue level 16 base attack bonus must stay 12");

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(trap_sense.value, 5, "Rogue level 16 Trap Sense must stay +5");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.rogue.talent_9_choice"),
        "the level-16 fixture must not surface a ninth talent slot"
    );
}

// ----- Negative control: the rogue path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_level17_recognition() {
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
fn multiclass_rogue_level17_is_not_promoted_by_this_slice() {
    let multiclass = ROGUE_LEVEL17_FIXTURE.replace(
        "class_level=class:rogue:17",
        "class_level=class:rogue:17\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-17 widening -----

#[test]
fn matrix_rogue_row_names_level_17_widening() {
    let matrix = seeded_current_truth();
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue bounded_progression row must exist");

    assert_eq!(rogue.support_state, SupportState::Supported); // promoted by SD-19 Class Progression Catalog browser
    assert_eq!(rogue.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(rogue.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        rogue.grounding_ref.contains("sd18_rogue_level17_widening"),
        "matrix grounding_ref must name the level-17 widening test: {}",
        rogue.grounding_ref
    );
}
