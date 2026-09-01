//! SD18 Rogue level-13 widening grounding proof.
//!
//! Widens the accepted Human Rogue level-1..level-12 chassis
//! (`tests/sd18_rogue_level12_widening.rs`, the loop's most recent Rogue
//! ceiling) to Rogue level 13 -- the first §3.2 level-13 widening attempted
//! across any of the 11 core classes (all 11 landed level 12 as of
//! cycle-2026-07-14T2244; this cycle opens the level-13 frontier), mirroring
//! the sibling-class level-range-gate idiom (`supported_rogue_level` is
//! generalized from `1..=12` to `1..=13` via `MAX_SUPPORTED_ROGUE_LEVEL =
//! 13`, exactly as every prior level-11/level-12 cycle widened its own
//! `MAX_SUPPORTED_<CLASS>_LEVEL`). Both PF1 CRB primary sources (d20pfsrd and
//! the Archives of Nethys aonprd.com mirror) were read directly before
//! writing any code or test, and both agree byte-for-byte:
//!
//! - level 13 base attack bonus STAYS +9 (`13 * 3 / 4 = 9`, an
//!   integer-division coincidence with level 12) and all three base saves
//!   also STAY unchanged: Fortitude +4 (`13 / 3 = 4`), Reflex +8
//!   (`13 / 2 + 2 = 8`), Will +4 (`13 / 3 = 4`) -- all four checked directly
//!   against both primary sources, not assumed.
//! - the PF1 Core Rulebook Rogue class table's level-13 "Special" column
//!   reads only "Sneak attack +7d6" (both primary sources agree) -- a
//!   tier-rise on the already-grounded sneak-attack die-count formula
//!   (`(level + 1) / 2`), which genuinely rises to `7` (i.e. `7d6`) at level
//!   13, up from `6` (`6d6`) at level 12, via the same formula, not a new
//!   record.
//! - Trap Sense STAYS +4 (`13 / 3 = 4`, unchanged from level 12, its next
//!   rise lands at 15th) and Trapfinding STAYS 6 (`max(13 / 2, 1) = 6`,
//!   unchanged from level 12, another integer-division coincidence);
//!   Evasion, Uncanny Dodge, and Improved Uncanny Dodge all stay granted,
//!   not re-derived.
//! - level 13 is NOT a rogue-talent level (talents land at 2, 4, 6, 8, 10,
//!   12, ...; the next slot is level 14), verified independently rather than
//!   assumed, so no seventh talent choice-slot record is grounded or
//!   fabricated at level 13 either.
//!
//! This is the cleanest possible widening shape: the ONLY value that
//! genuinely changes at level 13 is the sneak-attack die count, and that
//! rise runs entirely through the pre-existing `(level + 1) / 2` formula --
//! zero new record types, zero new named pillars, zero new choice slots.
//! It deliberately does not touch the rogue-talent tree (standard or
//! advanced), any check-execution engine, or sneak-attack damage
//! application (all stay named-but-unproven, unchanged from levels 1-12),
//! and it does not ground Rogue level 14+. It also preserves the accepted
//! Rogue level-1..level-12 truth (unchanged), the Fighter negative control,
//! and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const ROGUE_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level12_sd18_widening_deterministic_input.txt"
);

const ROGUE_LEVEL13_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level13_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const ROGUE_EVASION_ID: &str = "class_feature.rogue.evasion";
const ROGUE_TRAP_SENSE_ID: &str = "class_feature.rogue.trap_sense";
const ROGUE_UNCANNY_DODGE_ID: &str = "class_feature.rogue.uncanny_dodge";
const ROGUE_IMPROVED_UNCANNY_DODGE_ID: &str = "class_feature.rogue.improved_uncanny_dodge";

// ----- Base attack bonus and base saves stay unchanged at level 13 -----

#[test]
fn rogue_level13_base_attack_bonus_and_saves_stay_unchanged() {
    let input = load(ROGUE_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(
        base_attack.value, 9,
        "Rogue level 13 3/4-BAB progression (13 * 3 / 4) must stay 9, an integer-division \
         coincidence with level 12: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(
        fortitude.value, 4,
        "Rogue level 13 poor Fortitude (13/3) must stay 4, unchanged from level 12"
    );

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(
        reflex.value, 8,
        "Rogue level 13 good Reflex (13/2+2) must stay 8, unchanged from level 12"
    );

    let will = explanation(&computation, "class_chassis.rogue.base_save.will");
    assert_eq!(
        will.value, 4,
        "Rogue level 13 poor Will (13/3) must stay 4, unchanged from level 12"
    );
}

// ----- Sneak attack genuinely rises to 7d6 (matches the level-13 Special column) -----

#[test]
fn rogue_level13_sneak_attack_genuinely_rises() {
    let input = load(ROGUE_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 7,
        "Rogue level 13 sneak attack die count ((13 + 1) / 2) must genuinely rise to 7 (7d6), \
         up from 6 at level 12, per the PF1 Core Rulebook Rogue class table's level-13 \
         'Special' column ('Sneak attack +7d6'): {}",
        sneak_attack.detail
    );
}

// ----- Trap Sense and Trapfinding stay unchanged at level 13 -----

#[test]
fn rogue_level13_trap_sense_and_trapfinding_stay_unchanged() {
    let input = load(ROGUE_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 4,
        "Rogue level 13 Trap Sense (13 / 3) must stay +4, unchanged from level 12 -- the next \
         rise lands at level 15: {}",
        trap_sense.detail
    );

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(
        trapfinding.value, 6,
        "Rogue level 13 Trapfinding (max(13/2, 1)) must stay 6, unchanged from level 12: {}",
        trapfinding.detail
    );
}

// ----- No seventh talent choice slot at level 13 (not a talent level) -----

#[test]
fn rogue_level13_does_not_surface_a_seventh_talent_slot() {
    let input = load(ROGUE_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.rogue.talent_7_choice"),
        "level 13 is not a rogue-talent level (talents land at 2/4/6/8/10/12/14/...), so no \
         seventh talent choice slot may be fabricated: {:?}",
        computation.explanations
    );

    // The sixth slot, selected at level 12, stays recognized (not re-derived).
    let slot_6 = explanation(&computation, "class_chassis.rogue.talent_6_choice");
    assert_eq!(slot_6.value, 0, "the sixth talent slot must stay a +0 recognition record");
}

// ----- Granted features stay granted at level 13 -----

#[test]
fn rogue_level13_still_recognizes_the_granted_feature_records() {
    let input = load(ROGUE_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        ROGUE_EVASION_ID,
        ROGUE_UNCANNY_DODGE_ID,
        ROGUE_IMPROVED_UNCANNY_DODGE_ID,
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, 0,
            "'{id}' must stay a granted +0 identity record at level 13"
        );
    }
}

// ----- Negative control: the level-12 fixture is unaffected by this widening -----

#[test]
fn rogue_level12_truth_is_unchanged_by_this_slice() {
    let input = load(ROGUE_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(sneak_attack.value, 6, "Rogue level 12 sneak attack must stay 6d6");

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(base_attack.value, 9, "Rogue level 12 base attack bonus must stay 9");
}

// ----- Negative control: level 18 stays claim-blocked (beyond the bounded L1-13 row) -----
//
// SD18 widening (cycle-2026-07-15T2000, tests/sd18_rogue_level14_widening.rs)
// now genuinely recognizes level 14, so this boundary control moved to
// level 15; SD18 widening (cycle-2026-07-15T2900,
// tests/sd18_rogue_level15_widening.rs) now genuinely recognizes level 15
// too, so this boundary control moved again, to level 16. A further SD18
// widening (cycle-2026-07-15T5200, tests/sd18_rogue_level16_widening.rs) now
// genuinely recognizes level 16 too, so this boundary control moved again,
// to level 17. A further SD18 widening (cycle-2026-07-15T8100,
// tests/sd18_rogue_level17_widening.rs) now genuinely recognizes level 17
// too, so this boundary control moved again, to level 18. A further SD18
// widening (cycle-2026-07-16T0212, tests/sd18_rogue_level18_widening.rs)
// now genuinely recognizes level 18 too, so this boundary control moved
// again, to level 19. A further SD18 widening (cycle-2026-07-16T3600,
// tests/sd18_rogue_level19_widening.rs) now genuinely recognizes level 19
// too, so this boundary control moved again, to level 20. A further SD18
// widening (cycle-2026-07-16T1431, tests/sd18_rogue_level20_widening.rs)
// now genuinely recognizes level 20 too, so this boundary control moves
// again, to level 21 (PF1 has no 21st character level; this is a pure
// implementation-gate check).

#[test]
fn rogue_level_21_stays_claim_blocked() {
    let level_21 = ROGUE_LEVEL13_FIXTURE.replace("class:rogue:13", "class:rogue:21");
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
fn fighter_does_not_gain_rogue_level13_recognition() {
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
fn multiclass_rogue_level13_is_not_promoted_by_this_slice() {
    let multiclass = ROGUE_LEVEL13_FIXTURE.replace(
        "class_level=class:rogue:13",
        "class_level=class:rogue:13\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-13 widening -----

#[test]
fn matrix_rogue_row_names_level_13_widening() {
    let matrix = seeded_current_truth();
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue bounded_progression row must exist");

    assert_eq!(rogue.support_state, SupportState::Supported); // promoted by SD-19 Class Progression Catalog browser
    assert_eq!(rogue.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(rogue.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        rogue.grounding_ref.contains("sd18_rogue_level13_widening"),
        "rogue row must cite the live SD18 level-13 widening proof surface: {}",
        rogue.grounding_ref
    );
    let note = rogue.blocker_or_lossiness_note;
    assert!(
        note.contains("level 13") || note.contains("level-13"),
        "rogue partial note must name the level-13 widening: {note}"
    );
}
