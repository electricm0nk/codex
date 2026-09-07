//! SD13-E5 Sorcerer level-2 progression grounding proof.
//!
//! Widens the accepted Sorcerer level-1 spontaneous spell-burden baseline
//! (`tests/sd13_sorcerer_level1_spell_baseline.rs`,
//! `tests/sd13_sorcerer_base_attack_and_saves.rs`,
//! `tests/sd13_sorcerer_bloodline_class_skill_choice.rs`) to Sorcerer level 2,
//! mirroring the Fighter `supported_fighter_level` / Paladin `supported_paladin_level`
//! / Rogue `supported_rogue_level` / Barbarian `supported_barbarian_level` / Monk
//! `supported_monk_level` / Cleric `supported_cleric_level` / Bard
//! `supported_bard_level` / Druid `supported_druid_level` level-range-gate idiom (the
//! level-1-only gate `is_single_class_sorcerer_level1` is generalized to
//! `supported_sorcerer_level`, an `Option<u8>` helper gated by
//! `MAX_SUPPORTED_SORCERER_LEVEL = 2`). Both PF1 CRB primary sources (d20pfsrd and
//! legacy.aonprd.com Sorcerer class table) were read directly before writing any code
//! or test:
//!
//! - level 2 base attack bonus is +1 (`2 / 2 = 1`, the Sorcerer's own 1/2-BAB
//!   progression, UNLIKE the 3/4 BAB shared by Rogue/Monk/Druid/Cleric/Bard) and base
//!   saves are +0 Fortitude (poor, `2 / 3 = 0`), +0 Reflex (poor, `2 / 3 = 0`), +3 Will
//!   (good, `2 / 2 + 2 = 3`) — confirmed by the same formulas already grounded at
//!   level 1, not re-derived.
//! - the bloodline choice recognition and the Arcane bloodline's class-skill choice
//!   recognition are not level-gated (a sorcerer's bloodline does not change by level),
//!   so both still fire at level 2 for the same fixture selections, confirmed neither
//!   is accidentally hardcoded to fire only when `class_level.level ==
//!   SORCERER_BASELINE_LEVEL` (the level-1-only gate this slice widens away).
//! - the Sorcerer class table's level-2 "Special" column is blank: verified
//!   independently against both primary sources (d20pfsrd and legacy.aonprd.com) that
//!   Sorcerer gains no new class feature at 2nd level (unlike Rogue/Monk/Druid's
//!   Evasion/Woodland Stride, but like Cleric), so no new pillar burden is added this
//!   slice — only the existing pillars are widened.
//!
//! It deliberately does not touch Arcane Bond, bloodline arcana, bonus spells/feats at
//! 3rd+, or the spontaneous spell burden (all four stay named-but-unproven, unchanged
//! from level 1), and it does not ground Sorcerer level 3+. It also preserves the
//! accepted Sorcerer level-1 truth (unchanged), the Fighter negative control, and the
//! multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const SORCERER_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt");

const SORCERER_LEVEL2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level2_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 2 -----

#[test]
fn sorcerer_level2_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(SORCERER_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(
        base_attack.value, 1,
        "Sorcerer level 2 1/2-BAB progression (2 / 2) must equal 1: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 2 (good Will only, poor Fortitude, poor Reflex) -----

#[test]
fn sorcerer_level2_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(SORCERER_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.sorcerer.base_save.fortitude");
    assert_eq!(fortitude.value, 0, "Sorcerer level 2 poor Fortitude (2/3) must equal 0");

    let reflex = explanation(&computation, "class_chassis.sorcerer.base_save.reflex");
    assert_eq!(reflex.value, 0, "Sorcerer level 2 poor Reflex (2/3) must equal 0");

    let will = explanation(&computation, "class_chassis.sorcerer.base_save.will");
    assert_eq!(will.value, 3, "Sorcerer level 2 good Will (2/2+2) must equal 3");
}

// ----- Bloodline choice recognition still fires at level 2 -----

#[test]
fn sorcerer_level2_still_recognizes_the_bloodline_choice() {
    let input = load(SORCERER_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.sorcerer.bloodline_choice");
    assert_eq!(
        choice.value, 0,
        "bloodline choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Arcane"),
        "bloodline choice recognition must still name the Arcane bloodline at level 2: {}",
        choice.detail
    );
}

// ----- Arcane bloodline class-skill choice recognition still fires at level 2 -----

#[test]
fn sorcerer_level2_still_recognizes_the_bloodline_class_skill_choice() {
    let input = load(SORCERER_LEVEL2_FIXTURE);
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
         2: {}",
        choice.detail
    );
}

// ----- The two existing burden diagnostics still fire at level 2 -----

#[test]
fn sorcerer_level2_still_claim_blocks_arcane_bond_and_spontaneous_spell_burdens() {
    let input = load(SORCERER_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"
            && d.claim_blocking),
        "level-2 Sorcerer must still claim-block on the Arcane Bond / bloodline \
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

// ----- The chassis recognition record is still present at level 2 -----

#[test]
fn sorcerer_level2_still_recognizes_the_spell_bearing_baseline() {
    let input = load(SORCERER_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.spell_baseline.sorcerer"),
        "level-2 Sorcerer must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
}

// ----- The accepted Sorcerer level-1 truth is unaffected -----

#[test]
fn sorcerer_level1_truth_is_unchanged_by_this_widening() {
    let input = load(SORCERER_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(base_attack.value, 0, "Sorcerer level 1 base attack bonus must stay 0");

    let will = explanation(&computation, "class_chassis.sorcerer.base_save.will");
    assert_eq!(will.value, 2, "Sorcerer level 1 good Will save must stay 2");
}

// ----- Negative control: level 3 was later widened into the supported tranche -----

#[test]
fn sorcerer_level_3_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 3 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_sorcerer_level3_progression.rs) widened the level-range gate
    // to level 3 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Ranger
    // level-range gate idiom); this negative control is superseded, not
    // violated — pin the new truth here too so this file stays internally
    // consistent. Level 4 was in turn widened by a further SD13-E5 slice,
    // covered by `sorcerer_level_4_was_later_widened_into_the_supported_tranche`
    // below, and level 5 by a further slice still, covered by
    // `sorcerer_level_5_was_later_widened_into_the_supported_tranche` below.
    let level_3 = SORCERER_LEVEL2_FIXTURE.replace("class:sorcerer:2", "class:sorcerer:3");
    let input = load(&level_3);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, "class_chassis.sorcerer.base_attack_bonus"),
        "level-3 Sorcerer is supported since the SD13-E5 level-3 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "class_chassis.sorcerer.bloodline_choice"),
        "level-3 Sorcerer must keep the bloodline choice recognition grounded at level 2"
    );
}

// ----- Negative control: level 4 was later widened into the supported tranche -----

#[test]
fn sorcerer_level_4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_sorcerer_level4_progression.rs) widened the level-range gate
    // to level 4 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Ranger
    // level-range gate idiom); this negative control is superseded, not
    // violated — pin the new truth here too so this file stays internally
    // consistent. Level 5 was in turn widened by a further SD13-E5 slice,
    // covered by `sorcerer_level_5_was_later_widened_into_the_supported_tranche`
    // below.
    let level_4 = SORCERER_LEVEL2_FIXTURE.replace("class:sorcerer:2", "class:sorcerer:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, "class_chassis.sorcerer.base_attack_bonus"),
        "level-4 Sorcerer is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "class_chassis.sorcerer.bloodline_choice"),
        "level-4 Sorcerer must keep the bloodline choice recognition grounded at level 2"
    );
}

// ----- Negative control: level 5 was later widened into the supported tranche -----

#[test]
fn sorcerer_level_5_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 5 was not yet the frontier (level 3
    // was). A further SD13-E5 slice (tests/sd13_sorcerer_level5_progression.rs) widened
    // the level-range gate to level 5 (mirroring the Fighter/Paladin/Rogue/Barbarian/
    // Monk/Cleric/Bard/Druid/Ranger level-range gate idiom); pin the new truth here too
    // so this file stays internally consistent.
    let level_5 = SORCERER_LEVEL2_FIXTURE.replace("class:sorcerer:2", "class:sorcerer:5");
    let input = load(&level_5);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, "class_chassis.sorcerer.base_attack_bonus"),
        "level-5 Sorcerer is supported since the SD13-E5 level-5 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "class_chassis.sorcerer.bloodline_choice"),
        "level-5 Sorcerer must keep the bloodline choice recognition grounded at level 2"
    );
}

// ----- Negative control: the sorcerer path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_sorcerer_level2_recognition() {
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
fn multiclass_sorcerer_level2_is_not_promoted_by_this_slice() {
    let multiclass = SORCERER_LEVEL2_FIXTURE.replace(
        "class_level=class:sorcerer:2",
        "class_level=class:sorcerer:2\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-2 widening -----

#[test]
fn matrix_sorcerer_row_names_level_2_widening() {
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
            .contains("sd13_sorcerer_level2_progression"),
        "sorcerer row must cite the live SD13-E5 level-2 proof surface: {}",
        sorcerer.grounding_ref
    );
    let note = sorcerer.blocker_or_lossiness_note;
    assert!(
        note.contains("level 2") || note.contains("level-2"),
        "sorcerer partial note must name the level-2 widening: {note}"
    );
}
