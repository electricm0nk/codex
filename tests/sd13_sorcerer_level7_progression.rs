//! SD13-E5 Sorcerer level-7 progression grounding proof.
//!
//! Widens the accepted Sorcerer level-1/level-2/level-3/level-4/level-5/level-6
//! spontaneous spell-burden baseline (`tests/sd13_sorcerer_level1_spell_baseline.rs`,
//! `tests/sd13_sorcerer_base_attack_and_saves.rs`,
//! `tests/sd13_sorcerer_bloodline_class_skill_choice.rs`,
//! `tests/sd13_sorcerer_level2_progression.rs`,
//! `tests/sd13_sorcerer_level3_progression.rs`,
//! `tests/sd13_sorcerer_level4_progression.rs`,
//! `tests/sd13_sorcerer_level5_progression.rs`,
//! `tests/sd13_sorcerer_level6_progression.rs`) to Sorcerer level 7, mirroring
//! the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Ranger/Wizard
//! level-range-gate idiom (`supported_sorcerer_level` is generalized from
//! `1..=6` to `1..=7` via `MAX_SUPPORTED_SORCERER_LEVEL = 7`). Both PF1 CRB
//! primary sources (d20pfsrd and legacy.aonprd.com Sorcerer class table) were
//! read directly before writing any code or test:
//!
//! - level 7 base attack bonus is +3 (`7 / 2 = 3`, the Sorcerer's own
//!   1/2-BAB progression, UNLIKE the 3/4 BAB shared by
//!   Rogue/Monk/Druid/Cleric/Bard) and base saves are +2 Fortitude (poor,
//!   `7 / 3 = 2`), +2 Reflex (poor, `7 / 3 = 2`), +5 Will (good,
//!   `7 / 2 + 2 = 5`) — confirmed by the same formulas already grounded at
//!   levels 1-6, not re-derived. Every one of these four values is
//!   numerically unchanged from level 6, an integer-division coincidence,
//!   not a sign any formula stopped scaling (mirroring the level 4 -> 5
//!   coincidence).
//! - the bloodline choice recognition and the Arcane bloodline's class-skill
//!   choice recognition are not level-gated (a sorcerer's bloodline does not
//!   change by level), so both still fire at level 7 for the same fixture
//!   selections, confirmed neither is accidentally hardcoded to fire only at
//!   a lower level.
//! - the PF1 Core Rulebook Sorcerer class table's level-7 "Special" column
//!   reads "Bloodline feat, bloodline spell" (verified independently against
//!   both primary sources, checked rather than assumed away) — UNLIKE the
//!   blank level-6 column, Sorcerer gains two new named entries at 7th
//!   level: a bloodline feat (chosen from a list specific to each bloodline)
//!   and a third bloodline spell (bloodline-specific, e.g. the Arcane
//!   bloodline's own 7th-level bloodline spell is dispel magic). Both
//!   entries are bloodline-specific and not flat/identity-shaped the way
//!   Rogue's Trap Sense or Monk's Still Mind are, so this slice grounds no
//!   new pillar for level 7 either, mirroring exactly how the level-3
//!   "Bloodline power, bloodline spell" entry and the level-5 "Bloodline
//!   spell" entry were left unproven.
//!
//! It deliberately does not touch Arcane Bond, bloodline arcana, the
//! bloodline powers or bloodline spells gained at 3rd/5th/7th level, the new
//! 7th-level bloodline feat, bonus spells/feats at 3rd+, or the spontaneous
//! spell burden (all stay named-but-unproven, unchanged from level
//! 1/2/3/4/5/6), and it does not ground Sorcerer level 8+. It also preserves
//! the accepted Sorcerer level-1/level-2/level-3/level-4/level-5/level-6
//! truth (unchanged), the Fighter negative control, and the multiclass
//! negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const SORCERER_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level6_sd13_deterministic_input.txt");

const SORCERER_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level7_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 7 -----

#[test]
fn sorcerer_level7_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(SORCERER_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(
        base_attack.value, 3,
        "Sorcerer level 7 1/2-BAB progression (7 / 2) must equal 3: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 7 (good Will only, poor Fortitude, poor Reflex) -----

#[test]
fn sorcerer_level7_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(SORCERER_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.sorcerer.base_save.fortitude");
    assert_eq!(fortitude.value, 2, "Sorcerer level 7 poor Fortitude (7/3) must equal 2");

    let reflex = explanation(&computation, "class_chassis.sorcerer.base_save.reflex");
    assert_eq!(reflex.value, 2, "Sorcerer level 7 poor Reflex (7/3) must equal 2");

    let will = explanation(&computation, "class_chassis.sorcerer.base_save.will");
    assert_eq!(will.value, 5, "Sorcerer level 7 good Will (7/2+2) must equal 5");
}

// ----- Bloodline choice recognition still fires at level 7 -----

#[test]
fn sorcerer_level7_still_recognizes_the_bloodline_choice() {
    let input = load(SORCERER_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.sorcerer.bloodline_choice");
    assert_eq!(
        choice.value, 0,
        "bloodline choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Arcane"),
        "bloodline choice recognition must still name the Arcane bloodline at level 7: {}",
        choice.detail
    );
}

// ----- Arcane bloodline class-skill choice recognition still fires at level 7 -----

#[test]
fn sorcerer_level7_still_recognizes_the_bloodline_class_skill_choice() {
    let input = load(SORCERER_LEVEL7_FIXTURE);
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
         7: {}",
        choice.detail
    );
}

// ----- The two existing burden diagnostics still fire at level 7 -----

#[test]
fn sorcerer_level7_still_claim_blocks_arcane_bond_and_spontaneous_spell_burdens() {
    let input = load(SORCERER_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"
            && d.claim_blocking),
        "level-7 Sorcerer must still claim-block on the Arcane Bond / bloodline \
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

// ----- The chassis recognition record is still present at level 7 -----

#[test]
fn sorcerer_level7_still_recognizes_the_spell_bearing_baseline() {
    let input = load(SORCERER_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.spell_baseline.sorcerer"),
        "level-7 Sorcerer must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the level-6 fixture is unaffected by this widening -----

#[test]
fn sorcerer_level6_truth_is_unchanged_by_this_slice() {
    let input = load(SORCERER_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(base_attack.value, 3, "Sorcerer level 6 base attack bonus must stay 3");

    let will = explanation(&computation, "class_chassis.sorcerer.base_save.will");
    assert_eq!(will.value, 5, "Sorcerer level 6 good Will save must stay 5");
}

// ----- Level 8 was later widened into the supported tranche by a further slice -----

#[test]
fn sorcerer_level_8_was_later_widened_into_the_supported_tranche() {
    let level_8 = SORCERER_LEVEL7_FIXTURE.replace("class:sorcerer:7", "class:sorcerer:8");
    let input = load(&level_8);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.sorcerer.")),
        "level-8 Sorcerer is now recognized by the later level-8 widening slice \
         (tests/sd13_sorcerer_level8_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the sorcerer path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_sorcerer_level7_recognition() {
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
fn multiclass_sorcerer_level7_is_not_promoted_by_this_slice() {
    let multiclass = SORCERER_LEVEL7_FIXTURE.replace(
        "class_level=class:sorcerer:7",
        "class_level=class:sorcerer:7\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-7 widening -----

#[test]
fn matrix_sorcerer_row_names_level_7_widening() {
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
            .contains("sd13_sorcerer_level7_progression"),
        "sorcerer row must cite the live SD13-E5 level-7 proof surface: {}",
        sorcerer.grounding_ref
    );
    let note = sorcerer.blocker_or_lossiness_note;
    assert!(
        note.contains("level 7") || note.contains("level-7"),
        "sorcerer partial note must name the level-7 widening: {note}"
    );
}
