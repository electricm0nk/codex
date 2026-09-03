//! SD18 Rogue level-14 widening grounding proof.
//!
//! Widens the accepted Human Rogue level-1..level-13 chassis
//! (`tests/sd18_rogue_level13_widening.rs`, the loop's most recent Rogue
//! ceiling) to Rogue level 14, the loop's Rogue level-14 sweep landing,
//! mirroring the sibling-class level-range-gate idiom (`supported_rogue_level`
//! is generalized from `1..=13` to `1..=14` via `MAX_SUPPORTED_ROGUE_LEVEL =
//! 14`, exactly as every prior level-11/12/13 cycle widened its own
//! `MAX_SUPPORTED_<CLASS>_LEVEL`). Both PF1 CRB primary sources (d20pfsrd and
//! the Archives of Nethys aonprd.com mirror) were read directly before
//! writing any code or test, and both agree byte-for-byte:
//!
//! - level 14 base attack bonus genuinely RISES to +10 (`14 * 3 / 4 = 10`,
//!   up from 9 at level 13); Fortitude STAYS +4 (`14 / 3 = 4`, an
//!   integer-division coincidence with level 13); Reflex genuinely RISES to
//!   +9 (`14 / 2 + 2 = 9`, up from 8 at level 13); Will STAYS +4
//!   (`14 / 3 = 4`, an integer-division coincidence) -- all four checked
//!   directly against both primary sources, not assumed.
//! - the PF1 Core Rulebook Rogue class table's level-14 "Special" column
//!   reads only "Rogue talent" (both primary sources agree byte-for-byte):
//!   level 14 IS a rogue-talent cadence level (talents land at
//!   2/4/6/8/10/12/14), so a SEVENTH numbered choice-recognition slot
//!   (`choice:rogue_talent_7`) is added, mirroring the proven open-ended
//!   raw-string idiom used at slots 1-6 exactly -- no talent-list
//!   validation, no talent-effect engine.
//! - the sneak-attack die-count formula (`(level + 1) / 2`) STAYS 7
//!   (`15 / 2 = 7`, an integer-division coincidence with level 13, matching
//!   the level-14 "Special" column naming no sneak-attack rise); Trap Sense
//!   STAYS +4 (`14 / 3 = 4`, unchanged from level 13, next rise at level
//!   15); Trapfinding genuinely RISES to 7 (`max(14/2, 1) = 7`, up from 6 at
//!   level 13, via its own independent formula -- this rise is not named in
//!   the level-14 "Special" column); Evasion, Uncanny Dodge, and Improved
//!   Uncanny Dodge all stay granted, not re-derived.
//!
//! This slice deliberately does not touch the rogue-talent tree's own
//! effects (standard or advanced), any check-execution engine, or
//! sneak-attack damage application (all stay named-but-unproven, unchanged
//! from levels 1-13), and it does not ground Rogue level 15+. It also
//! preserves the accepted Rogue level-1..level-13 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const ROGUE_LEVEL13_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level13_sd18_widening_deterministic_input.txt"
);

const ROGUE_LEVEL14_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level14_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const ROGUE_EVASION_ID: &str = "class_feature.rogue.evasion";
const ROGUE_TRAP_SENSE_ID: &str = "class_feature.rogue.trap_sense";
const ROGUE_UNCANNY_DODGE_ID: &str = "class_feature.rogue.uncanny_dodge";
const ROGUE_IMPROVED_UNCANNY_DODGE_ID: &str = "class_feature.rogue.improved_uncanny_dodge";

// ----- Base attack bonus and Reflex genuinely rise at level 14; Fortitude/Will stay -----

#[test]
fn rogue_level14_base_attack_bonus_and_reflex_genuinely_rise() {
    let input = load(ROGUE_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(
        base_attack.value, 10,
        "Rogue level 14 3/4-BAB progression (14 * 3 / 4) must genuinely rise to 10, up from 9 \
         at level 13: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(
        fortitude.value, 4,
        "Rogue level 14 poor Fortitude (14/3) must stay 4, unchanged from level 13"
    );

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(
        reflex.value, 9,
        "Rogue level 14 good Reflex (14/2+2) must genuinely rise to 9, up from 8 at level 13"
    );

    let will = explanation(&computation, "class_chassis.rogue.base_save.will");
    assert_eq!(
        will.value, 4,
        "Rogue level 14 poor Will (14/3) must stay 4, unchanged from level 13"
    );
}

// ----- Sneak attack stays 7d6 (matches the level-14 Special column naming only "Rogue talent") -----

#[test]
fn rogue_level14_sneak_attack_stays_unchanged() {
    let input = load(ROGUE_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 7,
        "Rogue level 14 sneak attack die count ((14 + 1) / 2) must stay 7 (7d6), unchanged \
         from level 13, an integer-division coincidence, per the PF1 Core Rulebook Rogue class \
         table's level-14 'Special' column naming only 'Rogue talent': {}",
        sneak_attack.detail
    );
}

// ----- Trap Sense stays unchanged; Trapfinding genuinely rises at level 14 -----

#[test]
fn rogue_level14_trap_sense_stays_and_trapfinding_genuinely_rises() {
    let input = load(ROGUE_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 4,
        "Rogue level 14 Trap Sense (14 / 3) must stay +4, unchanged from level 13 -- the next \
         rise lands at level 15: {}",
        trap_sense.detail
    );

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(
        trapfinding.value, 7,
        "Rogue level 14 Trapfinding (max(14/2, 1)) must genuinely rise to 7, up from 6 at \
         level 13: {}",
        trapfinding.detail
    );
}

// ----- The seventh talent choice slot appears at level 14 (a talent level) -----

#[test]
fn rogue_level14_surfaces_the_seventh_talent_slot() {
    let input = load(ROGUE_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot_7 = explanation(&computation, "class_chassis.rogue.talent_7_choice");
    assert_eq!(slot_7.value, 0, "the seventh talent slot must be a +0 recognition record");

    // The sixth slot, selected at level 12, stays recognized (not re-derived).
    let slot_6 = explanation(&computation, "class_chassis.rogue.talent_6_choice");
    assert_eq!(slot_6.value, 0, "the sixth talent slot must stay a +0 recognition record");
}

// ----- Granted features stay granted at level 14 -----

#[test]
fn rogue_level14_still_recognizes_the_granted_feature_records() {
    let input = load(ROGUE_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        ROGUE_EVASION_ID,
        ROGUE_UNCANNY_DODGE_ID,
        ROGUE_IMPROVED_UNCANNY_DODGE_ID,
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, 0,
            "'{id}' must stay a granted +0 identity record at level 14"
        );
    }
}

// ----- Negative control: the level-13 fixture is unaffected by this widening -----

#[test]
fn rogue_level13_truth_is_unchanged_by_this_slice() {
    let input = load(ROGUE_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(sneak_attack.value, 7, "Rogue level 13 sneak attack must stay 7d6");

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(base_attack.value, 9, "Rogue level 13 base attack bonus must stay 9");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.rogue.talent_7_choice"),
        "the level-13 fixture must not surface the seventh talent slot"
    );
}

// ----- Negative control: level 18 stays claim-blocked (beyond the bounded L1-14 row) -----
//
// SD18 widening (cycle-2026-07-15T2900, tests/sd18_rogue_level15_widening.rs)
// now genuinely recognizes level 15, so this boundary control moved to
// level 16. A further SD18 widening (cycle-2026-07-15T5200,
// tests/sd18_rogue_level16_widening.rs) now genuinely recognizes level 16
// too, so this boundary control moved again, to level 17. A further SD18
// widening (cycle-2026-07-15T8100, tests/sd18_rogue_level17_widening.rs) now
// genuinely recognizes level 17 too, so this boundary control moved again,
// to level 18. A further SD18 widening (cycle-2026-07-16T0212,
// tests/sd18_rogue_level18_widening.rs) now genuinely recognizes level 18
// too, so this boundary control moved again, to level 19. A further SD18
// widening (cycle-2026-07-16T3600, tests/sd18_rogue_level19_widening.rs)
// now genuinely recognizes level 19 too, so this boundary control moves
// again, to level 20. A further SD18 widening (cycle-2026-07-16T1431,
// tests/sd18_rogue_level20_widening.rs) now genuinely recognizes level 20
// too, so this boundary control moves again, to level 21 (PF1 has no 21st
// character level; this is a pure implementation-gate check).

#[test]
fn rogue_level_21_stays_claim_blocked() {
    let level_21 = ROGUE_LEVEL14_FIXTURE.replace("class:rogue:14", "class:rogue:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-21 Rogue must stay claim-blocked beyond the bounded levels-1-20 row: {:?}",
        computation.diagnostics
    );
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.rogue.base_attack_bonus"),
        "level-21 Rogue must not fabricate a base-attack-bonus explanation"
    );
}

// ----- Negative control: the rogue path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_level14_recognition() {
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
fn multiclass_rogue_level14_is_not_promoted_by_this_slice() {
    let multiclass = ROGUE_LEVEL14_FIXTURE.replace(
        "class_level=class:rogue:14",
        "class_level=class:rogue:14\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-14 widening -----

#[test]
fn matrix_rogue_row_names_level_14_widening() {
    let matrix = seeded_current_truth();
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue bounded_progression row must exist");

    assert_eq!(rogue.support_state, SupportState::Supported); // promoted by SD-19 Class Progression Catalog browser
    assert_eq!(rogue.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(rogue.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        rogue.grounding_ref.contains("sd18_rogue_level14_widening"),
        "rogue row must cite the live SD18 level-14 widening proof surface: {}",
        rogue.grounding_ref
    );
    let note = rogue.blocker_or_lossiness_note;
    assert!(
        note.contains("level 14") || note.contains("level-14"),
        "rogue partial note must name the level-14 widening: {note}"
    );
}
