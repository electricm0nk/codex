//! SD13-E5 Sorcerer level-8 progression grounding proof.
//!
//! Widens the accepted Sorcerer level-1..level-7 spontaneous spell-burden
//! baseline (`tests/sd13_sorcerer_level1_spell_baseline.rs`,
//! `tests/sd13_sorcerer_base_attack_and_saves.rs`,
//! `tests/sd13_sorcerer_bloodline_class_skill_choice.rs`,
//! `tests/sd13_sorcerer_level2_progression.rs`,
//! `tests/sd13_sorcerer_level3_progression.rs`,
//! `tests/sd13_sorcerer_level4_progression.rs`,
//! `tests/sd13_sorcerer_level5_progression.rs`,
//! `tests/sd13_sorcerer_level6_progression.rs`,
//! `tests/sd13_sorcerer_level7_progression.rs`) to Sorcerer level 8, mirroring
//! the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Ranger/Wizard
//! level-range-gate idiom (`supported_sorcerer_level` is generalized from
//! `1..=7` to `1..=8` via `MAX_SUPPORTED_SORCERER_LEVEL = 8`). Both PF1 CRB
//! primary sources (d20pfsrd and legacy.aonprd.com Sorcerer class table) were
//! read directly before writing any code or test:
//!
//! - level 8 base attack bonus is +4 (`8 / 2 = 4`, the Sorcerer's own
//!   1/2-BAB progression, UNLIKE the 3/4 BAB shared by
//!   Rogue/Monk/Druid/Cleric/Bard) — a genuinely NEW value, up from +3 at
//!   level 7 — and base saves are +2 Fortitude (poor, `8 / 3 = 2`,
//!   numerically unchanged from level 7, an integer-division coincidence),
//!   +2 Reflex (poor, `8 / 3 = 2`, likewise unchanged), and +6 Will (good,
//!   `8 / 2 + 2 = 6`, genuinely risen from +5) — confirmed by the same
//!   formulas already grounded at levels 1-7, not re-derived.
//! - the bloodline choice recognition and the Arcane bloodline's class-skill
//!   choice recognition are not level-gated (a sorcerer's bloodline does not
//!   change by level), so both still fire at level 8 for the same fixture
//!   selections, confirmed neither is accidentally hardcoded to fire only at
//!   a lower level.
//! - the PF1 Core Rulebook Sorcerer class table's level-8 "Special" column
//!   is blank (verified independently against both primary sources, checked
//!   rather than assumed away) — like levels 2, 4, and 6, and UNLIKE the
//!   level-7 "Bloodline feat, bloodline spell" row, Sorcerer gains no new
//!   named class feature at 8th level. The first 4th-level spell slots
//!   arrive at 8th (3/day per the class table), but spells per day belong
//!   to the spontaneous spell burden this tranche deliberately leaves
//!   named-but-unproven, so no new pillar record is grounded at level 8.
//!
//! It deliberately does not touch Arcane Bond, bloodline arcana, the
//! bloodline powers or bloodline spells gained at 3rd/5th/7th level, the
//! 7th-level bloodline feat, bonus spells/feats at 3rd+, or the spontaneous
//! spell burden (all stay named-but-unproven, unchanged from levels 1-7),
//! and it does not ground Sorcerer level 9+. It also preserves the accepted
//! Sorcerer level-1..level-7 truth (unchanged), the Fighter negative
//! control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const SORCERER_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level7_sd13_deterministic_input.txt");

const SORCERER_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level8_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 8 -----

#[test]
fn sorcerer_level8_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(SORCERER_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(
        base_attack.value, 4,
        "Sorcerer level 8 1/2-BAB progression (8 / 2) must equal 4, genuinely risen from 3 at \
         level 7: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 8 (good Will only, poor Fortitude, poor Reflex) -----

#[test]
fn sorcerer_level8_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(SORCERER_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.sorcerer.base_save.fortitude");
    assert_eq!(fortitude.value, 2, "Sorcerer level 8 poor Fortitude (8/3) must equal 2");

    let reflex = explanation(&computation, "class_chassis.sorcerer.base_save.reflex");
    assert_eq!(reflex.value, 2, "Sorcerer level 8 poor Reflex (8/3) must equal 2");

    let will = explanation(&computation, "class_chassis.sorcerer.base_save.will");
    assert_eq!(
        will.value, 6,
        "Sorcerer level 8 good Will (8/2+2) must equal 6, genuinely risen from 5 at level 7"
    );
}

// ----- Bloodline choice recognition still fires at level 8 -----

#[test]
fn sorcerer_level8_still_recognizes_the_bloodline_choice() {
    let input = load(SORCERER_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.sorcerer.bloodline_choice");
    assert_eq!(
        choice.value, 0,
        "bloodline choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Arcane"),
        "bloodline choice recognition must still name the Arcane bloodline at level 8: {}",
        choice.detail
    );
}

// ----- Arcane bloodline class-skill choice recognition still fires at level 8 -----

#[test]
fn sorcerer_level8_still_recognizes_the_bloodline_class_skill_choice() {
    let input = load(SORCERER_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(
        &computation,
        "class_chassis.sorcerer.bloodline_class_skill_choice",
    );
    assert_eq!(
        choice.value, 0,
        "class-skill choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Knowledge (arcana)"),
        "class-skill choice recognition must still name the selected Knowledge skill at level \
         8: {}",
        choice.detail
    );
}

// ----- The two existing burden diagnostics still fire at level 8 -----

#[test]
fn sorcerer_level8_still_claim_blocks_arcane_bond_and_spontaneous_spell_burdens() {
    let input = load(SORCERER_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"
            && d.claim_blocking),
        "level-8 Sorcerer must still claim-block on the Arcane Bond / bloodline \
         progression burden: {:?}",
        computation.diagnostics
    );
    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_spell.sorcerer.spontaneous.unsupported")
    {
        Some(spell_blocker) => assert!(
            spell_blocker.claim_blocking,
            "if the spell blocker fires at all, it must be claim-blocking"
        ),
        None => {
            // (v0.6 alpha swarm, risks item 8) class_spell.sorcerer.spontaneous.unsupported
            // is no longer unconditional -- it's a real, conditional validation of
            // AcquisitionMode::Known selections. This fixture predates spells_selected
            // (zero known spells), so the posture is genuinely valid and the blocker
            // correctly does not fire -- confirmed via the real known-spell count being
            // honestly 0, not fabricated.
            let known_count = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_spell.sorcerer.known_spells")
                .map(|e| e.value)
                .unwrap_or(-1);
            assert_eq!(
                known_count, 0,
                "no spells are fabricated merely because the spell blocker stopped firing: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- The chassis recognition record is still present at level 8 -----

#[test]
fn sorcerer_level8_still_recognizes_the_spell_bearing_baseline() {
    let input = load(SORCERER_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.spell_baseline.sorcerer"),
        "level-8 Sorcerer must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the level-7 fixture is unaffected by this widening -----

#[test]
fn sorcerer_level7_truth_is_unchanged_by_this_slice() {
    let input = load(SORCERER_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(base_attack.value, 3, "Sorcerer level 7 base attack bonus must stay 3");

    let will = explanation(&computation, "class_chassis.sorcerer.base_save.will");
    assert_eq!(will.value, 5, "Sorcerer level 7 good Will save must stay 5");
}

// ----- Level 9 was later widened into the supported tranche by a further slice -----

#[test]
fn sorcerer_level_9_was_later_widened_into_the_supported_tranche() {
    let level_9 = SORCERER_LEVEL8_FIXTURE.replace("class:sorcerer:8", "class:sorcerer:9");
    let input = load(&level_9);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.sorcerer.")),
        "level-9 Sorcerer is now recognized by the later level-9 widening slice \
         (tests/sd13_sorcerer_level9_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the sorcerer path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_sorcerer_level8_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.sorcerer.")
                || e.id == "class_chassis.spell_baseline.sorcerer"),
        "the Fighter chassis must not surface any sorcerer-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Sorcerer is not promoted -----

#[test]
fn multiclass_sorcerer_level8_is_not_promoted_by_this_slice() {
    let multiclass = SORCERER_LEVEL8_FIXTURE.replace(
        "class_level=class:sorcerer:8",
        "class_level=class:sorcerer:8\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.sorcerer.")
                || e.id == "class_chassis.spell_baseline.sorcerer"),
        "multiclass Sorcerer must not gain any bounded sorcerer chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Sorcerer must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-8 widening -----

#[test]
fn matrix_sorcerer_row_names_level_8_widening() {
    let matrix = seeded_current_truth();
    let sorcerer = matrix
        .row("class.sorcerer.progression_and_spell_burden")
        .expect("sorcerer progression_and_spell_burden row must exist");

    assert_eq!(sorcerer.support_state, SupportState::Supported);
    assert_eq!(sorcerer.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        sorcerer.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        sorcerer
            .grounding_ref
            .contains("sd13_sorcerer_level8_progression"),
        "sorcerer row must cite the live SD13-E5 level-8 proof surface: {}",
        sorcerer.grounding_ref
    );
    let note = sorcerer.blocker_or_lossiness_note;
    assert!(
        note.contains("level 8") || note.contains("level-8"),
        "sorcerer partial note must name the level-8 widening: {note}"
    );
}
