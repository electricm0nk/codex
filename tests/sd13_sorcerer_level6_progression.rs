//! SD13-E5 Sorcerer level-6 progression grounding proof.
//!
//! Widens the accepted Sorcerer level-1/level-2/level-3/level-4/level-5
//! spontaneous spell-burden baseline (`tests/sd13_sorcerer_level1_spell_baseline.rs`,
//! `tests/sd13_sorcerer_base_attack_and_saves.rs`,
//! `tests/sd13_sorcerer_bloodline_class_skill_choice.rs`,
//! `tests/sd13_sorcerer_level2_progression.rs`,
//! `tests/sd13_sorcerer_level3_progression.rs`,
//! `tests/sd13_sorcerer_level4_progression.rs`,
//! `tests/sd13_sorcerer_level5_progression.rs`) to Sorcerer level 6, mirroring
//! the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Ranger/Wizard
//! level-range-gate idiom (`supported_sorcerer_level` is generalized from
//! `1..=5` to `1..=6` via `MAX_SUPPORTED_SORCERER_LEVEL = 6`). Both PF1 CRB
//! primary sources (d20pfsrd and legacy.aonprd.com Sorcerer class table) were
//! read directly before writing any code or test:
//!
//! - level 6 base attack bonus is +3 (`6 / 2 = 3`, the Sorcerer's own
//!   1/2-BAB progression, UNLIKE the 3/4 BAB shared by
//!   Rogue/Monk/Druid/Cleric/Bard) and base saves are +2 Fortitude (poor,
//!   `6 / 3 = 2`), +2 Reflex (poor, `6 / 3 = 2`), +5 Will (good,
//!   `6 / 2 + 2 = 5`) — confirmed by the same formulas already grounded at
//!   levels 1-5, not re-derived. Every one of these four values is a
//!   genuinely NEW value, up from +2/+1/+1/+4 at level 5.
//! - the bloodline choice recognition and the Arcane bloodline's class-skill
//!   choice recognition are not level-gated (a sorcerer's bloodline does not
//!   change by level), so both still fire at level 6 for the same fixture
//!   selections, confirmed neither is accidentally hardcoded to fire only at
//!   a lower level.
//! - the PF1 Core Rulebook Sorcerer class table's level-6 "Special" column is
//!   genuinely BLANK (verified independently against both primary sources,
//!   checked rather than assumed away) — UNLIKE the level-3 "Bloodline
//!   power, bloodline spell" entry and the level-5 "Bloodline spell" entry,
//!   Sorcerer gains no new class feature at 6th level, mirroring exactly how
//!   the blank level-2 and level-4 "Special" columns were left as pure
//!   arithmetic widenings — so this slice grounds no new pillar for level 6
//!   either, only the existing pillars are widened.
//!
//! It deliberately does not touch Arcane Bond, bloodline arcana, the
//! bloodline powers or bloodline spells gained at 3rd/5th level, bonus
//! spells/feats at 3rd+, or the spontaneous spell burden (all stay
//! named-but-unproven, unchanged from level 1/2/3/4/5), and it does not
//! ground Sorcerer level 7+. It also preserves the accepted Sorcerer
//! level-1/level-2/level-3/level-4/level-5 truth (unchanged), the Fighter
//! negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const SORCERER_LEVEL5_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level5_sd13_deterministic_input.txt");

const SORCERER_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level6_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 6 -----

#[test]
fn sorcerer_level6_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(SORCERER_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(
        base_attack.value, 3,
        "Sorcerer level 6 1/2-BAB progression (6 / 2) must equal 3: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 6 (good Will only, poor Fortitude, poor Reflex) -----

#[test]
fn sorcerer_level6_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(SORCERER_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.sorcerer.base_save.fortitude");
    assert_eq!(fortitude.value, 2, "Sorcerer level 6 poor Fortitude (6/3) must equal 2");

    let reflex = explanation(&computation, "class_chassis.sorcerer.base_save.reflex");
    assert_eq!(reflex.value, 2, "Sorcerer level 6 poor Reflex (6/3) must equal 2");

    let will = explanation(&computation, "class_chassis.sorcerer.base_save.will");
    assert_eq!(will.value, 5, "Sorcerer level 6 good Will (6/2+2) must equal 5");
}

// ----- Bloodline choice recognition still fires at level 6 -----

#[test]
fn sorcerer_level6_still_recognizes_the_bloodline_choice() {
    let input = load(SORCERER_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.sorcerer.bloodline_choice");
    assert_eq!(
        choice.value, 0,
        "bloodline choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Arcane"),
        "bloodline choice recognition must still name the Arcane bloodline at level 6: {}",
        choice.detail
    );
}

// ----- Arcane bloodline class-skill choice recognition still fires at level 6 -----

#[test]
fn sorcerer_level6_still_recognizes_the_bloodline_class_skill_choice() {
    let input = load(SORCERER_LEVEL6_FIXTURE);
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
         6: {}",
        choice.detail
    );
}

// ----- The two existing burden diagnostics still fire at level 6 -----

#[test]
fn sorcerer_level6_still_claim_blocks_arcane_bond_and_spontaneous_spell_burdens() {
    let input = load(SORCERER_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"
            && d.claim_blocking),
        "level-6 Sorcerer must still claim-block on the Arcane Bond / bloodline \
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

// ----- The chassis recognition record is still present at level 6 -----

#[test]
fn sorcerer_level6_still_recognizes_the_spell_bearing_baseline() {
    let input = load(SORCERER_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.spell_baseline.sorcerer"),
        "level-6 Sorcerer must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the level-5 fixture is unaffected by this widening -----

#[test]
fn sorcerer_level5_truth_is_unchanged_by_this_slice() {
    let input = load(SORCERER_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(base_attack.value, 2, "Sorcerer level 5 base attack bonus must stay 2");

    let will = explanation(&computation, "class_chassis.sorcerer.base_save.will");
    assert_eq!(will.value, 4, "Sorcerer level 5 good Will save must stay 4");
}

// ----- Negative control (superseded): level 7 was later widened into the supported tranche -----

#[test]
fn sorcerer_level_7_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 7 was the next unproven milestone
    // and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_sorcerer_level7_progression.rs) widened the level-range gate to
    // level 7 (mirroring the Rogue/Barbarian/Monk level-range gate idiom), extending
    // the existing base-attack/base-save/bloodline-choice pillars without grounding
    // any new pillar; this negative control is superseded, not violated — pin the new
    // truth here too so this file stays internally consistent. The frontier this
    // file's own slice actually drew is now level 8, covered by
    // `sorcerer_level_8_is_not_promoted_by_this_slice` in
    // `tests/sd13_sorcerer_level7_progression.rs`.
    let level_7 = SORCERER_LEVEL6_FIXTURE.replace("class:sorcerer:6", "class:sorcerer:7");
    let input = load(&level_7);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, "class_chassis.sorcerer.base_attack_bonus"),
        "level-7 Sorcerer is supported since the SD13-E5 level-7 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "class_chassis.sorcerer.bloodline_choice"),
        "level-7 Sorcerer must keep the bloodline choice recognition grounded: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the sorcerer path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_sorcerer_level6_recognition() {
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
fn multiclass_sorcerer_level6_is_not_promoted_by_this_slice() {
    let multiclass = SORCERER_LEVEL6_FIXTURE.replace(
        "class_level=class:sorcerer:6",
        "class_level=class:sorcerer:6\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-6 widening -----

#[test]
fn matrix_sorcerer_row_names_level_6_widening() {
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
            .contains("sd13_sorcerer_level6_progression"),
        "sorcerer row must cite the live SD13-E5 level-6 proof surface: {}",
        sorcerer.grounding_ref
    );
    let note = sorcerer.blocker_or_lossiness_note;
    assert!(
        note.contains("level 6") || note.contains("level-6"),
        "sorcerer partial note must name the level-6 widening: {note}"
    );
}
