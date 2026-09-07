//! SD18 Rogue level-18 widening grounding proof.
//!
//! Widens the accepted Human Rogue level-1..level-17 chassis
//! (`tests/sd18_rogue_level17_widening.rs`, the loop's most recent Rogue
//! ceiling) to Rogue level 18, the loop's SIXTH §3.2 level-18 landing
//! (after Wizard, Cleric, Paladin, Fighter, and Barbarian), mirroring the
//! sibling-class level-range-gate idiom (`supported_rogue_level` is
//! generalized from `1..=17` to `1..=18` via `MAX_SUPPORTED_ROGUE_LEVEL =
//! 18`, exactly as every prior level-11..17 cycle widened its own
//! `MAX_SUPPORTED_<CLASS>_LEVEL`). Both PF1 CRB primary sources (d20pfsrd
//! and the Archives of Nethys aonprd.com mirror) were read directly before
//! writing any code or test, fetching the full levels-16-through-19 block
//! in one pass so the level-18 row's neighbors were visible in context
//! (guards against level-misattribution), and both agree byte-for-byte:
//!
//! - level 16: BAB +12/+7/+2, Fort +5, Ref +10, Will +5, Special "Rogue
//!   talent"
//! - level 17: BAB +12/+7/+2, Fort +5, Ref +10, Will +5, Special "Sneak
//!   attack +9d6"
//! - level 18: BAB +13/+8/+3, Fort +6, Ref +11, Will +6, Special "Rogue
//!   talent, trap sense +6"
//! - level 19: BAB +14/+9/+4, Fort +6, Ref +11, Will +6, Special "Sneak
//!   attack +10d6"
//!
//! At level 18: base attack bonus GENUINELY RISES to +13 (`18 * 3 / 4 =
//! 13`, up from +12 at level 17) and all three base saves GENUINELY RISE
//! (good Reflex to +11 via `18 / 2 + 2`, poor Fortitude/Will both to +6 via
//! `18 / 3`, up from +5 at level 17) -- all four checked directly against
//! both primary sources, not assumed. The level-18 "Special" column reads
//! "Rogue talent, trap sense +6" -- TWO entries, both tier-rises/repeats on
//! already-grounded pillars, not a new class feature: 18 IS a rogue-talent
//! cadence level (talents land at 2/4/6/8/10/12/14/16/18), so a NINTH
//! numbered choice-recognition slot (`choice:rogue_talent_9`) is added,
//! mirroring the proven open-ended raw-string idiom used at slots 1-8
//! exactly -- no talent-effect engine invented; and the pre-existing Trap
//! Sense flat-magnitude formula (`level / 3`) genuinely rises to +6, up
//! from +5 at level 17 (the doc comment on `ROGUE_TRAP_SENSE_LEVEL`'s
//! sibling formula already anticipated this: "this bonus rises further at
//! 9th/12th/15th/18th rogue level"). Sneak attack stays 9d6 (`(18 + 1) / 2
//! = 9`, an integer-division coincidence with level 17, next rise at level
//! 19); Trapfinding genuinely rises to 9 (`max(18/2, 1) = 9`, up from 8 at
//! level 17, via the pre-existing formula -- this rise is not named in the
//! level-18 "Special" column, exactly like its silent rise at level 16);
//! Evasion, Uncanny Dodge, and Improved Uncanny Dodge all stay granted, not
//! re-derived.
//!
//! This widening needs ZERO new tier constants for base attack/saves (all
//! already level-generic formulas) and ZERO new formula changes for Trap
//! Sense (already level-generic) -- the ONLY production-code changes are
//! raising `MAX_SUPPORTED_ROGUE_LEVEL` from 17 to 18 and appending a ninth
//! numbered talent slot to the existing tuple-array idiom, mirroring the
//! Barbarian level-18 rage-power-slot landing exactly (the loop's fifth
//! prior repeat of this same numbered-choice-slot shape, now the ninth
//! repeat of Rogue's own talent-slot idiom specifically).
//!
//! This slice deliberately does not touch the rogue-talent tree's own
//! effects (standard or advanced), any check-execution engine, or
//! sneak-attack damage application (all stay named-but-unproven, unchanged
//! from levels 1-17), and it does not ground Rogue level 19+. It also
//! preserves the accepted Rogue level-1..level-17 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const ROGUE_LEVEL17_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level17_sd18_widening_deterministic_input.txt"
);

const ROGUE_LEVEL18_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level18_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const ROGUE_EVASION_ID: &str = "class_feature.rogue.evasion";
const ROGUE_TRAP_SENSE_ID: &str = "class_feature.rogue.trap_sense";
const ROGUE_UNCANNY_DODGE_ID: &str = "class_feature.rogue.uncanny_dodge";
const ROGUE_IMPROVED_UNCANNY_DODGE_ID: &str = "class_feature.rogue.improved_uncanny_dodge";

// ----- Base attack bonus and all three base saves genuinely rise at level 18 -----

#[test]
fn rogue_level18_base_attack_and_saves_rise() {
    let input = load(ROGUE_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(
        base_attack.value, 13,
        "Rogue level 18 3/4-BAB progression (18 * 3 / 4) must genuinely rise to 13, up from 12 \
         at level 17: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(
        fortitude.value, 6,
        "Rogue level 18 poor Fortitude (18/3) must genuinely rise to 6, up from 5 at level 17"
    );

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(
        reflex.value, 11,
        "Rogue level 18 good Reflex (18/2+2) must genuinely rise to 11, up from 10 at level 17"
    );

    let will = explanation(&computation, "class_chassis.rogue.base_save.will");
    assert_eq!(
        will.value, 6,
        "Rogue level 18 poor Will (18/3) must genuinely rise to 6, up from 5 at level 17"
    );
}

// ----- Trap Sense genuinely rises to +6 at level 18 -----

#[test]
fn rogue_level18_trap_sense_rises_to_six() {
    let input = load(ROGUE_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 6,
        "Rogue level 18 Trap Sense (18/3) must genuinely rise to +6, up from +5 at level 17: {}",
        trap_sense.detail
    );
}

// ----- Ninth talent slot is newly grounded at level 18 -----

#[test]
fn rogue_level18_gains_ninth_talent_slot() {
    let input = load(ROGUE_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot_nine = explanation(&computation, "class_chassis.rogue.talent_9_choice");
    assert_eq!(
        slot_nine.value, 0,
        "the ninth rogue talent slot must be a bounded +0 recognition record at level 18: {}",
        slot_nine.detail
    );
    assert!(
        slot_nine.detail.contains("weapon_training"),
        "the ninth talent slot detail must name the level-18 fixture's chosen selection: {}",
        slot_nine.detail
    );

    // The eighth slot (granted at level 16) still carries over.
    let slot_eight = explanation(&computation, "class_chassis.rogue.talent_8_choice");
    assert_eq!(
        slot_eight.value, 0,
        "the eighth talent slot must stay a bounded +0 recognition record at level 18: {}",
        slot_eight.detail
    );
}

// ----- Sneak attack stays unchanged; Trapfinding genuinely rises, though neither is named in the Special column -----

#[test]
fn rogue_level18_sneak_attack_stays_unchanged() {
    let input = load(ROGUE_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 9,
        "Rogue level 18 sneak attack die count ((18 + 1) / 2) must stay 9 (9d6), unchanged from \
         level 17, an integer-division coincidence -- next rise at level 19: {}",
        sneak_attack.detail
    );
}

#[test]
fn rogue_level18_trapfinding_genuinely_rises() {
    let input = load(ROGUE_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(
        trapfinding.value, 9,
        "Rogue level 18 Trapfinding (max(18/2, 1)) must genuinely rise to 9, up from 8 at level \
         17, via the pre-existing formula -- not named in the level-18 'Special' column: {}",
        trapfinding.detail
    );
}

// ----- Remaining pillars carry over unchanged -----

#[test]
fn rogue_level18_remaining_pillars_carry_over() {
    let input = load(ROGUE_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        ROGUE_EVASION_ID,
        ROGUE_UNCANNY_DODGE_ID,
        ROGUE_IMPROVED_UNCANNY_DODGE_ID,
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, 0,
            "'{id}' must stay a granted +0 identity record at level 18"
        );
    }
}

// ----- Negative control: the level-17 fixture is unaffected by this widening -----

#[test]
fn rogue_level17_truth_is_unchanged_by_this_slice() {
    let input = load(ROGUE_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(sneak_attack.value, 9, "Rogue level 17 sneak attack must stay 9d6");

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(base_attack.value, 12, "Rogue level 17 base attack bonus must stay 12");

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(trap_sense.value, 5, "Rogue level 17 Trap Sense must stay +5");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.rogue.talent_9_choice"),
        "Rogue level 17 must not gain a ninth talent slot: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the rogue path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_level18_recognition() {
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
fn multiclass_rogue_level18_is_not_promoted_by_this_slice() {
    let multiclass = ROGUE_LEVEL18_FIXTURE.replace(
        "class_level=class:rogue:18",
        "class_level=class:rogue:18\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-18 widening -----

#[test]
fn matrix_rogue_row_names_level_18_widening() {
    let matrix = seeded_current_truth();
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue bounded_progression row must exist");

    assert_eq!(rogue.support_state, SupportState::Supported); // promoted by SD-19 Class Progression Catalog browser
    assert_eq!(rogue.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(rogue.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        rogue.grounding_ref.contains("sd18_rogue_level18_widening"),
        "matrix grounding_ref must name the level-18 widening test: {}",
        rogue.grounding_ref
    );
}
