//! SD18 Rogue level-11 sneak-attack widening grounding proof.
//!
//! Widens the accepted deterministic Human Rogue level-1..level-10 chassis
//! (`tests/sd13_rogue_level10_progression.rs`, the SD13 tranche's declared
//! ceiling) to Rogue level 11 — the ninth SD-18 §3.2 class-row widening,
//! mirroring the sibling-class level-range-gate idiom
//! (`supported_rogue_level` is generalized from `1..=10` to `1..=11` via
//! `MAX_SUPPORTED_ROGUE_LEVEL = 11`, exactly as prior cycles widened
//! `MAX_SUPPORTED_BARBARIAN_LEVEL`, `MAX_SUPPORTED_BARD_LEVEL`,
//! `MAX_SUPPORTED_CLERIC_LEVEL`, `MAX_SUPPORTED_DRUID_LEVEL`,
//! `MAX_SUPPORTED_FIGHTER_LEVEL`, `MAX_SUPPORTED_MONK_LEVEL`,
//! `MAX_SUPPORTED_PALADIN_LEVEL`, and `MAX_SUPPORTED_WIZARD_LEVEL`, all from
//! 10 to 11). §3.1 race rows are confirmed genuinely exhausted this run
//! (Dwarf, Elf, Gnome, Half-Elf, Half-Orc, Halfling, and Human all CLOSED
//! under the current seam shape), and §3.3 interaction rows were
//! re-checked live this cycle and remain non-advanceable (no class row yet
//! branches its compute path on a specific non-Human race identity), so
//! this cycle picks the next fully-untouched §3.2 class row. Both PF1 CRB
//! primary sources (d20pfsrd and the Archives of Nethys aonprd.com mirror)
//! were read directly before writing any code or test, and both agree:
//!
//! - level 11 base attack bonus is +8 (`11 * 3 / 4 = 8`, the Rogue's
//!   3/4-BAB progression, genuinely risen from +7 at level 10) and base
//!   saves are +3 Fortitude (poor, `11 / 3 = 3`), +7 Reflex (good,
//!   `11 / 2 + 2 = 7`), and +3 Will (poor, `11 / 3 = 3`) — all three saves
//!   numerically IDENTICAL to level 10, integer-division coincidences, not
//!   a sign any formula stopped scaling, confirmed by the same formulas
//!   already grounded at levels 1-10, not re-derived.
//! - the PF1 Core Rulebook Rogue class table's level-11 "Special" column
//!   reads only "Sneak attack +6d6" (verified independently against both
//!   primary sources, checked rather than assumed away) — the pre-existing
//!   sneak-attack die-count formula (`(level + 1) / 2`) genuinely rises to
//!   `6` (i.e. `6d6`) at level 11, up from `5` (`5d6`) at level 10, via the
//!   same formula, not a new record.
//! - Trap Sense stays +3 (`11 / 3 = 3`, unchanged from level 10, its next
//!   rise lands at 12th) and Trapfinding stays 5 (`max(11 / 2, 1) = 5`,
//!   unchanged from level 10, another integer-division coincidence);
//!   Evasion, Uncanny Dodge, and Improved Uncanny Dodge all stay granted,
//!   not re-derived.
//! - level 11 is NOT a rogue-talent level (talents land at 2, 4, 6, 8, 10,
//!   12, ...; the next slot is level 12), verified independently rather than
//!   assumed, so no new talent choice-slot record is grounded or fabricated
//!   at level 11 either.
//!
//! It deliberately does not touch the rogue-talent tree (standard or
//! advanced), any check-execution engine, or sneak-attack damage
//! application (all stay named-but-unproven, unchanged from levels 1-10),
//! and it does not ground Rogue level 12+. It also preserves the accepted
//! Rogue level-1..level-10 truth (unchanged), the Fighter negative control,
//! and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const ROGUE_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level10_sd13_deterministic_input.txt");

const ROGUE_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level11_sd18_sneak_attack_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const ROGUE_EVASION_ID: &str = "class_feature.rogue.evasion";
const ROGUE_TRAP_SENSE_ID: &str = "class_feature.rogue.trap_sense";
const ROGUE_UNCANNY_DODGE_ID: &str = "class_feature.rogue.uncanny_dodge";
const ROGUE_IMPROVED_UNCANNY_DODGE_ID: &str = "class_feature.rogue.improved_uncanny_dodge";

// ----- Base attack bonus genuinely rises at level 11 -----

#[test]
fn rogue_level11_base_attack_bonus_genuinely_rises() {
    let input = load(ROGUE_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(
        base_attack.value, 8,
        "Rogue level 11 3/4-BAB progression (11 * 3 / 4) must equal 8, genuinely risen from 7 \
         at level 10: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 11 stay numerically unchanged -----

#[test]
fn rogue_level11_base_saves_stay_numerically_unchanged() {
    let input = load(ROGUE_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(fortitude.value, 3, "Rogue level 11 poor Fortitude (11/3) must stay 3");

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(
        reflex.value, 7,
        "Rogue level 11 good Reflex (11/2+2) must stay 7 — an integer-division coincidence \
         with level 10"
    );

    let will = explanation(&computation, "class_chassis.rogue.base_save.will");
    assert_eq!(will.value, 3, "Rogue level 11 poor Will (11/3) must stay 3");
}

// ----- Sneak attack die count genuinely rises to 6 at level 11 -----

#[test]
fn rogue_level11_sneak_attack_die_count_genuinely_rises() {
    let input = load(ROGUE_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 6,
        "Rogue level 11 sneak attack die count ((11 + 1) / 2) must genuinely rise to 6 (6d6), \
         up from 5 (5d6) at level 10, per the PF1 Core Rulebook Rogue class table's level-11 \
         'Special' column: {}",
        sneak_attack.detail
    );
}

// ----- Trap Sense and Trapfinding both stay unchanged at level 11 -----

#[test]
fn rogue_level11_trap_sense_and_trapfinding_stay_unchanged() {
    let input = load(ROGUE_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 3,
        "Rogue level 11 Trap Sense (11 / 3) must stay +3 — its next rise lands at 12th: {}",
        trap_sense.detail
    );

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(
        trapfinding.value, 5,
        "Rogue level 11 Trapfinding (max(11/2, 1)) must stay 5, an integer-division \
         coincidence with level 10: {}",
        trapfinding.detail
    );
}

// ----- Granted features stay granted at level 11 -----

#[test]
fn rogue_level11_still_recognizes_the_granted_feature_records() {
    let input = load(ROGUE_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        ROGUE_EVASION_ID,
        ROGUE_UNCANNY_DODGE_ID,
        ROGUE_IMPROVED_UNCANNY_DODGE_ID,
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, 0,
            "'{id}' must stay a granted +0 identity record at level 11"
        );
    }
}

// ----- No talent record is fabricated at level 11 -----

#[test]
fn rogue_level11_does_not_fabricate_talent_records() {
    let input = load(ROGUE_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("talent")),
        "level-11 Rogue must not fabricate any rogue-talent or advanced-talent record (level \
         11 is not a rogue-talent level; talents land at 2/4/6/8/10/12): {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.to_lowercase().contains("talent")),
        "level-11 Rogue must not fabricate any talent diagnostic either: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the level-10 fixture is unaffected by this widening -----

#[test]
fn rogue_level10_truth_is_unchanged_by_this_slice() {
    let input = load(ROGUE_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(base_attack.value, 7, "Rogue level 10 base attack bonus must stay 7");

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(sneak_attack.value, 5, "Rogue level 10 sneak attack die count must stay 5");

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(trapfinding.value, 5, "Rogue level 10 Trapfinding must stay 5");
}

// ----- Negative control: level 15 stays unrecognized by this slice -----
//
// SD18 widening (cycle-2026-07-15T0800, tests/sd18_rogue_level12_widening.rs)
// now genuinely recognizes level 12, so this boundary control moved to
// level 13. A further SD18 widening (cycle-2026-07-15T1100,
// tests/sd18_rogue_level13_widening.rs) now genuinely recognizes level 13
// too, so this boundary control moved again, to level 14. A further SD18
// widening (cycle-2026-07-15T2000, tests/sd18_rogue_level14_widening.rs) now
// genuinely recognizes level 14 too, so this boundary control moved again,
// to level 16, per SD18 cycle-2026-07-15T2900 (tests/sd18_rogue_level15_widening.rs).
// A further SD18 widening (cycle-2026-07-15T5200,
// tests/sd18_rogue_level16_widening.rs) now genuinely recognizes level 16
// too, so this boundary control moved again, to level 17. A further SD18
// widening (cycle-2026-07-15T8100, tests/sd18_rogue_level17_widening.rs) now
// genuinely recognizes level 17 too, so this boundary control moved again,
// to level 18. A further SD18 widening (cycle-2026-07-16T0212,
// tests/sd18_rogue_level18_widening.rs) now genuinely recognizes level 18
// too, so this boundary control moved again, to level 19. A further SD18
// widening (cycle-2026-07-16T3600, tests/sd18_rogue_level19_widening.rs)
// now genuinely recognizes level 19 too, so this boundary control moves
// again, to level 21 (PF1 has no 21st character level; this is a pure
// implementation-gate check).

#[test]
fn rogue_level_21_is_not_promoted_by_this_slice() {
    let level_21 = ROGUE_LEVEL11_FIXTURE.replace("class:rogue:11", "class:rogue:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")
                || e.id.starts_with("class_feature.rogue.")),
        "level-21 Rogue must not gain any bounded rogue explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the rogue path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_level11_recognition() {
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
fn multiclass_rogue_level11_is_not_promoted_by_this_slice() {
    let multiclass = ROGUE_LEVEL11_FIXTURE.replace(
        "class_level=class:rogue:11",
        "class_level=class:rogue:11\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-11 widening -----

#[test]
fn matrix_rogue_row_names_level_11_widening() {
    let matrix = seeded_current_truth();
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue bounded_progression row must exist");

    assert_eq!(rogue.support_state, SupportState::Supported); // promoted by SD-19 Class Progression Catalog browser
    assert_eq!(rogue.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(rogue.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        rogue.grounding_ref.contains("sd18_rogue_level11_sneak_attack"),
        "rogue row must cite the live SD18 level-11 widening proof surface: {}",
        rogue.grounding_ref
    );
    let note = rogue.blocker_or_lossiness_note;
    assert!(
        note.contains("level 11") || note.contains("level-11"),
        "rogue partial note must name the level-11 widening: {note}"
    );
}
