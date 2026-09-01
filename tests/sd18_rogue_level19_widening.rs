//! SD18 Rogue level-19 widening grounding proof.
//!
//! Widens the accepted Human Rogue level-1..level-18 chassis
//! (`tests/sd18_rogue_level18_widening.rs`, the loop's most recent Rogue
//! ceiling) to Rogue level 19, the loop's SEVENTH §3.2 level-19 landing
//! (after Barbarian, Cleric, Fighter, Bard, Paladin, and Ranger), mirroring
//! the sibling-class level-range-gate idiom (`supported_rogue_level` is
//! generalized from `1..=18` to `1..=19` via `MAX_SUPPORTED_ROGUE_LEVEL =
//! 19`, exactly as every prior level-11..18 cycle widened its own
//! `MAX_SUPPORTED_<CLASS>_LEVEL`). Both PF1 CRB primary sources (d20pfsrd
//! and the Archives of Nethys aonprd.com mirror) were read directly before
//! writing any code or test, fetching the full levels-15-through-20 block
//! in one pass (raw curl + Python tag-strip, not summarized WebFetch) so
//! the level-19 row's neighbors were visible in context (guards against
//! level-misattribution), and both agree byte-for-byte:
//!
//! - level 17: BAB +12/+7/+2, Fort +5, Ref +10, Will +5, Special "Sneak
//!   attack +9d6"
//! - level 18: BAB +13/+8/+3, Fort +6, Ref +11, Will +6, Special "Rogue
//!   talent, trap sense +6"
//! - level 19: BAB +14/+9/+4, Fort +6, Ref +11, Will +6, Special "Sneak
//!   attack +10d6"
//! - level 20: BAB +15/+10/+5, Fort +6, Ref +12, Will +6, Special "Master
//!   strike, rogue talent"
//!
//! At level 19: base attack bonus GENUINELY RISES to +14 (`19 * 3 / 4 =
//! 14`, up from +13 at level 18) -- checked directly against both primary
//! sources, not assumed. Good Reflex stays +11 (`19 / 2 + 2 = 11`, an
//! integer-division coincidence with level 18) and poor Fortitude/Will both
//! stay +6 (`19 / 3 = 6`, also integer-division coincidences with level
//! 18). The level-19 "Special" column reads only "Sneak attack +10d6" -- a
//! tier-rise on the already-grounded sneak-attack die-count formula, not a
//! new class feature: 19 is NOT a rogue-talent cadence level (talents land
//! at 2/4/6/8/10/12/14/16/18, next at 20), so no tenth numbered talent slot
//! is grounded or fabricated this cycle. The sneak-attack die-count formula
//! (`(level + 1) / 2`) genuinely rises to 10 (`10d6`, up from 9d6 at level
//! 18, via the pre-existing formula, not a new record -- this is the
//! formula's own final tier per the PF1 CRB, confirmed unchanged at level
//! 20 by the fetched neighbor row). Trap Sense stays +6 (`19 / 3`, an
//! integer-division coincidence with level 18, next rise beyond level 20's
//! own scope) and Trapfinding stays 9 (`max(19 / 2, 1)`, an
//! integer-division coincidence with level 18), neither named in the
//! level-19 "Special" column. Evasion, Uncanny Dodge, and Improved Uncanny
//! Dodge all stay granted, not re-derived.
//!
//! This widening needs ZERO new tier constants for base attack/saves/
//! sneak-attack/trap-sense/trapfinding (all already level-generic
//! formulas) and ZERO new choice slots -- the ONLY production-code change
//! is raising `MAX_SUPPORTED_ROGUE_LEVEL` from 18 to 19, the cleanest
//! possible widening shape checked in the Rogue sweep so far (mirroring
//! the level-17 cycle's own equally clean shape).
//!
//! This slice deliberately does not touch the rogue-talent tree's own
//! effects (standard or advanced), any check-execution engine, or
//! sneak-attack damage application (all stay named-but-unproven, unchanged
//! from levels 1-18), and it does not ground Rogue level 20 (Master
//! Strike, the tenth talent slot, and the capstone). It also preserves the
//! accepted Rogue level-1..level-18 truth (unchanged), the Fighter
//! negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const ROGUE_LEVEL18_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level18_sd18_widening_deterministic_input.txt"
);

const ROGUE_LEVEL19_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level19_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const ROGUE_EVASION_ID: &str = "class_feature.rogue.evasion";
const ROGUE_TRAP_SENSE_ID: &str = "class_feature.rogue.trap_sense";
const ROGUE_UNCANNY_DODGE_ID: &str = "class_feature.rogue.uncanny_dodge";
const ROGUE_IMPROVED_UNCANNY_DODGE_ID: &str = "class_feature.rogue.improved_uncanny_dodge";

// ----- Base attack bonus genuinely rises at level 19; saves stay put (coincidences) -----

#[test]
fn rogue_level19_base_attack_rises_saves_stay_put() {
    let input = load(ROGUE_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(
        base_attack.value, 14,
        "Rogue level 19 3/4-BAB progression (19 * 3 / 4) must genuinely rise to 14, up from 13 \
         at level 18: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(
        fortitude.value, 6,
        "Rogue level 19 poor Fortitude (19/3) must stay 6, an integer-division coincidence with \
         level 18"
    );

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(
        reflex.value, 11,
        "Rogue level 19 good Reflex (19/2+2) must stay 11, an integer-division coincidence with \
         level 18"
    );

    let will = explanation(&computation, "class_chassis.rogue.base_save.will");
    assert_eq!(
        will.value, 6,
        "Rogue level 19 poor Will (19/3) must stay 6, an integer-division coincidence with level \
         18"
    );
}

// ----- Sneak attack genuinely rises to 10d6 (its final PF1 CRB tier) -----

#[test]
fn rogue_level19_sneak_attack_rises_to_ten() {
    let input = load(ROGUE_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 10,
        "Rogue level 19 sneak attack die count ((19 + 1) / 2) must genuinely rise to 10 (10d6), \
         up from 9 at level 18: {}",
        sneak_attack.detail
    );
}

// ----- No tenth talent slot at level 19: 19 is not a rogue-talent cadence level -----

#[test]
fn rogue_level19_gains_no_tenth_talent_slot() {
    let input = load(ROGUE_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.rogue.talent_10_choice"),
        "Rogue level 19 must NOT gain a tenth talent slot: 19 is not a rogue-talent cadence \
         level (talents land at 2/4/6/8/10/12/14/16/18, next at 20): {:?}",
        computation.explanations
    );

    // The ninth slot (granted at level 18) still carries over unchanged.
    let slot_nine = explanation(&computation, "class_chassis.rogue.talent_9_choice");
    assert_eq!(
        slot_nine.value, 0,
        "the ninth talent slot must stay a bounded +0 recognition record at level 19: {}",
        slot_nine.detail
    );
}

// ----- Trap Sense and Trapfinding stay put at level 19 (integer-division coincidences) -----

#[test]
fn rogue_level19_trap_sense_and_trapfinding_stay_put() {
    let input = load(ROGUE_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 6,
        "Rogue level 19 Trap Sense (19/3) must stay +6, an integer-division coincidence with \
         level 18: {}",
        trap_sense.detail
    );

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(
        trapfinding.value, 9,
        "Rogue level 19 Trapfinding (max(19/2, 1)) must stay 9, an integer-division coincidence \
         with level 18, not named in the level-19 'Special' column: {}",
        trapfinding.detail
    );
}

// ----- Remaining pillars carry over unchanged -----

#[test]
fn rogue_level19_remaining_pillars_carry_over() {
    let input = load(ROGUE_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        ROGUE_EVASION_ID,
        ROGUE_UNCANNY_DODGE_ID,
        ROGUE_IMPROVED_UNCANNY_DODGE_ID,
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, 0,
            "'{id}' must stay a granted +0 identity record at level 19"
        );
    }
}

// ----- Negative control: the level-18 fixture is unaffected by this widening -----

#[test]
fn rogue_level18_truth_is_unchanged_by_this_slice() {
    let input = load(ROGUE_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(sneak_attack.value, 9, "Rogue level 18 sneak attack must stay 9d6");

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(base_attack.value, 13, "Rogue level 18 base attack bonus must stay 13");

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(trap_sense.value, 6, "Rogue level 18 Trap Sense must stay +6");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.rogue.talent_10_choice"),
        "Rogue level 18 must not gain a tenth talent slot: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the rogue path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_level19_recognition() {
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
fn multiclass_rogue_level19_is_not_promoted_by_this_slice() {
    let multiclass = ROGUE_LEVEL19_FIXTURE.replace(
        "class_level=class:rogue:19",
        "class_level=class:rogue:19\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")
                || e.id.starts_with("class_feature.rogue.")),
        "multiclass Rogue must not gain any bounded rogue explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Rogue must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-19 widening -----

#[test]
fn matrix_rogue_row_names_level_19_widening() {
    let matrix = seeded_current_truth();
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue bounded_progression row must exist");

    assert_eq!(rogue.support_state, SupportState::Supported); // promoted by SD-19 Class Progression Catalog browser
    assert_eq!(rogue.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(rogue.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        rogue.grounding_ref.contains("sd18_rogue_level19_widening"),
        "matrix grounding_ref must name the level-19 widening test: {}",
        rogue.grounding_ref
    );
}
