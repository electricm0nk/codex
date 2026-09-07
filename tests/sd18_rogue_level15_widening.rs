//! SD18 Rogue level-15 widening grounding proof.
//!
//! Widens the accepted Human Rogue level-1..level-14 chassis
//! (`tests/sd18_rogue_level14_widening.rs`, the loop's most recent Rogue
//! ceiling) to Rogue level 15, the loop's Rogue level-15 sweep landing,
//! mirroring the sibling-class level-range-gate idiom (`supported_rogue_level`
//! is generalized from `1..=14` to `1..=15` via `MAX_SUPPORTED_ROGUE_LEVEL =
//! 15`, exactly as every prior level-11/12/13/14 cycle widened its own
//! `MAX_SUPPORTED_<CLASS>_LEVEL`). Both PF1 CRB primary sources (d20pfsrd and
//! the Archives of Nethys aonprd.com mirror) were read directly before
//! writing any code or test, and both agree byte-for-byte:
//!
//! - level 15 base attack bonus genuinely RISES to +11 (`15 * 3 / 4 = 11`,
//!   up from 10 at level 14); Fortitude genuinely RISES to +5 (`15 / 3 = 5`,
//!   up from 4 at level 14); Reflex STAYS +9 (`15 / 2 + 2 = 9`, an
//!   integer-division coincidence with level 14); Will genuinely RISES to +5
//!   (`15 / 3 = 5`, up from 4 at level 14) -- all four checked directly
//!   against both primary sources, not assumed.
//! - the PF1 Core Rulebook Rogue class table's level-15 "Special" column
//!   reads only "Sneak attack +8d6, trap sense +5" (both primary sources
//!   agree byte-for-byte): the sneak-attack die-count formula
//!   (`(level + 1) / 2`) genuinely RISES to 8 (`16 / 2 = 8`, i.e. 8d6, up
//!   from 7d6 at level 14) via the same pre-existing formula, not a new
//!   record; Trap Sense genuinely RISES to +5 (`15 / 3 = 5`, up from +4 at
//!   level 14) via the same pre-existing formula, not a new record. Level 15
//!   is NOT a rogue-talent level (talents land at 2/4/6/8/10/12/14/16), so
//!   no eighth numbered choice-recognition slot appears here.
//! - Trapfinding STAYS +7 (`max(15/2, 1) = 7`, an integer-division
//!   coincidence with level 14, via its own independent formula -- not named
//!   in the level-15 "Special" column); Evasion, Uncanny Dodge, and Improved
//!   Uncanny Dodge all stay granted, not re-derived.
//!
//! Because every one of these formulas (base attack, all three base saves,
//! sneak attack, trap sense, trapfinding) is already level-generic in
//! `pilot_compute.rs`, and level 15 grants no new talent slot, this
//! widening needs ZERO new tier constants, ZERO new record types, and ZERO
//! new choice slots -- the cleanest possible landing shape, mirroring the
//! Barbarian level-15 landing (cycle-2026-07-15T2800) exactly: the ONLY
//! production-code change is raising `MAX_SUPPORTED_ROGUE_LEVEL` from 14 to
//! 15.
//!
//! This slice deliberately does not touch the rogue-talent tree's own
//! effects (standard or advanced), any check-execution engine, or
//! sneak-attack damage application (all stay named-but-unproven, unchanged
//! from levels 1-14), and it does not ground Rogue level 16+. It also
//! preserves the accepted Rogue level-1..level-14 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const ROGUE_LEVEL14_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level14_sd18_widening_deterministic_input.txt"
);

const ROGUE_LEVEL15_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level15_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const ROGUE_EVASION_ID: &str = "class_feature.rogue.evasion";
const ROGUE_TRAP_SENSE_ID: &str = "class_feature.rogue.trap_sense";
const ROGUE_UNCANNY_DODGE_ID: &str = "class_feature.rogue.uncanny_dodge";
const ROGUE_IMPROVED_UNCANNY_DODGE_ID: &str = "class_feature.rogue.improved_uncanny_dodge";

// ----- Base attack bonus, Fortitude, and Will genuinely rise at level 15; Reflex stays -----

#[test]
fn rogue_level15_base_attack_bonus_fortitude_and_will_genuinely_rise() {
    let input = load(ROGUE_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(
        base_attack.value, 11,
        "Rogue level 15 3/4-BAB progression (15 * 3 / 4) must genuinely rise to 11, up from 10 \
         at level 14: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(
        fortitude.value, 5,
        "Rogue level 15 poor Fortitude (15/3) must genuinely rise to 5, up from 4 at level 14"
    );

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(
        reflex.value, 9,
        "Rogue level 15 good Reflex (15/2+2) must stay 9, unchanged from level 14, an \
         integer-division coincidence"
    );

    let will = explanation(&computation, "class_chassis.rogue.base_save.will");
    assert_eq!(
        will.value, 5,
        "Rogue level 15 poor Will (15/3) must genuinely rise to 5, up from 4 at level 14"
    );
}

// ----- Sneak attack and Trap Sense both genuinely rise (the level-15 Special column) -----

#[test]
fn rogue_level15_sneak_attack_and_trap_sense_genuinely_rise() {
    let input = load(ROGUE_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 8,
        "Rogue level 15 sneak attack die count ((15 + 1) / 2) must genuinely rise to 8 (8d6), \
         up from 7d6 at level 14, per the PF1 Core Rulebook Rogue class table's level-15 \
         'Special' column reading 'Sneak attack +8d6, trap sense +5': {}",
        sneak_attack.detail
    );

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 5,
        "Rogue level 15 Trap Sense (15 / 3) must genuinely rise to +5, up from +4 at level 14: \
         {}",
        trap_sense.detail
    );
}

// ----- Trapfinding stays unchanged (an integer-division coincidence with level 14) -----

#[test]
fn rogue_level15_trapfinding_stays_unchanged() {
    let input = load(ROGUE_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(
        trapfinding.value, 7,
        "Rogue level 15 Trapfinding (max(15/2, 1)) must stay 7, unchanged from level 14, an \
         integer-division coincidence: {}",
        trapfinding.detail
    );
}

// ----- No eighth talent slot appears at level 15 (not a rogue-talent level) -----

#[test]
fn rogue_level15_does_not_surface_an_eighth_talent_slot() {
    let input = load(ROGUE_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.rogue.talent_8_choice"),
        "level 15 is NOT a rogue-talent level (talents land at 2/4/6/8/10/12/14/16); no eighth \
         talent slot must appear: {:?}",
        computation.explanations
    );

    // The seventh slot, selected at level 14, stays recognized (not re-derived).
    let slot_7 = explanation(&computation, "class_chassis.rogue.talent_7_choice");
    assert_eq!(slot_7.value, 0, "the seventh talent slot must stay a +0 recognition record");
}

// ----- Granted features stay granted at level 15 -----

#[test]
fn rogue_level15_still_recognizes_the_granted_feature_records() {
    let input = load(ROGUE_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        ROGUE_EVASION_ID,
        ROGUE_UNCANNY_DODGE_ID,
        ROGUE_IMPROVED_UNCANNY_DODGE_ID,
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, 0,
            "'{id}' must stay a granted +0 identity record at level 15"
        );
    }
}

// ----- Negative control: the level-14 fixture is unaffected by this widening -----

#[test]
fn rogue_level14_truth_is_unchanged_by_this_slice() {
    let input = load(ROGUE_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(sneak_attack.value, 7, "Rogue level 14 sneak attack must stay 7d6");

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(base_attack.value, 10, "Rogue level 14 base attack bonus must stay 10");

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(trap_sense.value, 4, "Rogue level 14 Trap Sense must stay +4");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.rogue.talent_8_choice"),
        "the level-14 fixture must not surface an eighth talent slot"
    );
}

// ----- Negative control: the rogue path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_level15_recognition() {
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
fn multiclass_rogue_level15_is_not_promoted_by_this_slice() {
    let multiclass = ROGUE_LEVEL15_FIXTURE.replace(
        "class_level=class:rogue:15",
        "class_level=class:rogue:15\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-15 widening -----

#[test]
fn matrix_rogue_row_names_level_15_widening() {
    let matrix = seeded_current_truth();
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue bounded_progression row must exist");

    assert_eq!(rogue.support_state, SupportState::Supported); // promoted by SD-19 Class Progression Catalog browser
    assert_eq!(rogue.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(rogue.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        rogue.grounding_ref.contains("sd18_rogue_level15_widening"),
        "matrix grounding_ref must name the level-15 widening test: {}",
        rogue.grounding_ref
    );
}
