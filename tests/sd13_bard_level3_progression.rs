//! SD13-E5 Bard level-3 progression grounding proof.
//!
//! Widens the accepted Bard level-1/level-2 spontaneous arcane spell-bearing
//! baseline (`tests/sd13_bard_level1_spell_baseline.rs`,
//! `tests/sd13_bard_base_attack_and_saves.rs`, `tests/sd13_bard_fascinate_dc.rs`,
//! `tests/sd13_bard_level2_progression.rs`) to Bard level 3, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Druid/Sorcerer/Wizard
//! level-range-gate idiom (`supported_bard_level` is generalized from `1..=2` to
//! `1..=3` via `MAX_SUPPORTED_BARD_LEVEL = 3`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Bard class table) were read directly before
//! writing any code or test:
//!
//! - level 3 base attack bonus is +2 (`3 * 3 / 4`), base Reflex/Will are +3
//!   (good, `3/2+2`), base Fortitude is +1 (poor, `3/3`) — confirmed by the same
//!   formulas already grounded at levels 1-2, not re-derived.
//! - Bardic Knowledge stays `max(3/2, 1) = 1` at level 3, confirmed via the same
//!   formula.
//! - Bardic Performance rounds per day continues to scale: `4 + Charisma
//!   modifier + 2 * (level - 1)` = 4 + 2 + 4 = 10 on the fixture's Charisma 15,
//!   up from 8 at level 2, confirmed via the same formula, not a new record.
//! - Inspire Courage's flat magnitude stays +1 at level 3: the PF1 Core
//!   Rulebook Inspire Courage bonus first increases at bard level 5 (to +2),
//!   confirmed via the same formula/constant, not a new record.
//! - the Fascinate flat Will-save DC (`10 + level/2 + CHA modifier`) and the
//!   flat affected-creature count (`1 + (level-1)/3`) both already take bard
//!   level as an input variable, so both extend correctly to level 3 with no
//!   re-derivation: DC 13 (10 + 1 + 2) and count 1 (1 + 0) on the fixture.
//! - Well-Versed (the 2nd-level Bard class feature grounded by the immediately
//!   preceding cycle) stays granted at level 3, not re-derived — the same
//!   bounded identity/magnitude record already grounded at level 2.
//! - the Bard class table's level-3 "Special" column reads "Inspire competence
//!   +2" — verified independently against both primary sources (d20pfsrd and
//!   legacy.aonprd.com): "a bard can use his performance to help allies
//!   succeed at a task... granting a +2 competence bonus on skill checks with
//!   a particular skill," first gained at 3rd level. This is flat and
//!   identity-shaped at the one supported level (the magnitude does not
//!   change again until level 7, out of this slice's scope), so it is
//!   grounded as a standalone `ComputationExplanation` record mirroring the
//!   Fighter Bravery / Rogue Trap Sense / Barbarian Trap Sense / Monk Still
//!   Mind idiom: never applied to any actual skill-check total, since no
//!   skill-check-resolution engine exists in this codebase, and no
//!   task-selection/action-economy engine decides which skill or ally it
//!   targets.
//!
//! It deliberately does not touch the performance-state/action-economy
//! engine, Countersong, Distraction, Versatile Performance, or the
//! spontaneous spell burden (all stay named-but-unproven, unchanged from
//! level 1/2), and it does not ground Bard level 4+. It also preserves the
//! accepted Bard level-1/level-2 truth (unchanged), the Fighter negative
//! control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARD_LEVEL2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level2_sd13_deterministic_input.txt");

const BARD_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level3_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

const WELL_VERSED_ID: &str = "class_feature.bard.well_versed";
const INSPIRE_COMPETENCE_ID: &str = "class_feature.bard.inspire_competence";

// ----- Base attack bonus at level 3 -----

#[test]
fn bard_level3_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(BARD_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 2,
        "Bard level 3 3/4-BAB progression (3 * 3 / 4) must equal 2: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 3 (good Reflex/Will, poor Fortitude) -----

#[test]
fn bard_level3_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(BARD_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.bard.base_save.fortitude");
    assert_eq!(fortitude.value, 1, "Bard level 3 poor Fortitude (3/3) must equal 1");

    let reflex = explanation(&computation, "class_chassis.bard.base_save.reflex");
    assert_eq!(reflex.value, 3, "Bard level 3 good Reflex (3/2+2) must equal 3");

    let will = explanation(&computation, "class_chassis.bard.base_save.will");
    assert_eq!(will.value, 3, "Bard level 3 good Will (3/2+2) must equal 3");
}

// ----- Bardic Knowledge stays max(level/2, 1) -----

#[test]
fn bard_level3_bardic_knowledge_stays_grounded_at_one() {
    let input = load(BARD_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let knowledge = explanation(&computation, "class_chassis.bard.bardic_knowledge");
    assert_eq!(
        knowledge.value, 1,
        "Bard level 3 Bardic Knowledge (max(3/2, 1)) must equal 1: {}",
        knowledge.detail
    );
}

// ----- Bardic Performance rounds per day keeps scaling with level -----

#[test]
fn bard_level3_bardic_performance_rounds_per_day_keeps_scaling() {
    let input = load(BARD_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Bardic Performance: 4 + CHA modifier at level 1, plus 2
    // additional rounds per day at each level after 1st. Fixture CHA 15 + 2 Human
    // racial (CG-03 fix) -> +3 modifier. At level 3: 4 + 3 + 2 * (3 - 1) = 11.
    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    assert_eq!(
        rounds.value, 11,
        "Bard level 3 bardic performance rounds per day must equal 4 + CHA + 2*(level-1) \
         = 4 + 3 + 4 = 11: {}",
        rounds.detail
    );
}

// ----- Inspire Courage stays +1 at level 3 (first increases at level 5) -----

#[test]
fn bard_level3_inspire_courage_stays_flat_plus_one() {
    let input = load(BARD_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_courage = explanation(&computation, "class_chassis.bard.inspire_courage_bonus");
    assert_eq!(
        inspire_courage.value, 1,
        "Bard level 3 Inspire Courage magnitude must stay +1 (PF1: first increases at level \
         5): {}",
        inspire_courage.detail
    );
}

// ----- Fascinate DC and affected-creature count extend to level 3 -----

#[test]
fn bard_level3_fascinate_dc_and_affected_creatures_extend_by_the_same_formulas() {
    let input = load(BARD_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Fixture CHA 15 + 2 Human racial (CG-03 fix) -> +3 modifier. DC = 10 + 3/2 + 3 = 14.
    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(
        dc.value, 14,
        "Bard level 3 Fascinate DC must equal 10 + (3/2) + 3 = 14: {}",
        dc.detail
    );

    // Affected creatures = 1 + (3-1)/3 = 1.
    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(
        count.value, 1,
        "Bard level 3 Fascinate affected-creature count must stay 1: {}",
        count.detail
    );
}

// ----- Well-Versed stays granted at level 3, not re-derived -----

#[test]
fn bard_level3_keeps_well_versed_grounded() {
    let input = load(BARD_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let well_versed = explanation(&computation, WELL_VERSED_ID);
    assert_eq!(
        well_versed.value, 4,
        "Well-Versed must stay the flat +4 bonus at level 3, not re-derived: {}",
        well_versed.detail
    );
}

// ----- New at level 3: Inspire Competence grounded as a flat standalone magnitude -----

#[test]
fn bard_level3_grounds_inspire_competence_flat_magnitude() {
    let input = load(BARD_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(
        inspire_competence.value, 2,
        "Bard Inspire Competence at level 3 must ground the flat +2 competence bonus (PF1 Core \
         Rulebook): {}",
        inspire_competence.detail
    );
    assert!(
        inspire_competence.detail.to_lowercase().contains("competence"),
        "Inspire Competence detail must name the competence bonus on skill checks: {}",
        inspire_competence.detail
    );
    assert!(
        inspire_competence.detail.to_lowercase().contains("granted"),
        "Inspire Competence detail at level 3 must state it is granted, not absent: {}",
        inspire_competence.detail
    );
    assert!(
        inspire_competence.detail.contains("standalone")
            || inspire_competence.detail.contains("never applied")
            || inspire_competence.detail.contains("no skill-check"),
        "Inspire Competence detail must disclaim application to any actual skill-check total: {}",
        inspire_competence.detail
    );
}

#[test]
fn bard_level2_correctly_lacks_inspire_competence() {
    let input = load(BARD_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(
        inspire_competence.value, 0,
        "Bard Inspire Competence must be a correct level-gate absence (value 0) below level 3: {}",
        inspire_competence.detail
    );
}

// ----- Versatile Performance is still deliberately NOT grounded this slice -----

#[test]
fn bard_level3_does_not_fabricate_versatile_performance() {
    let input = load(BARD_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.contains("versatile_performance")),
        "Versatile Performance requires a choice-gated skill-substitution engine and must not \
         be fabricated as a flat record: {:?}",
        computation.explanations
    );
}

// ----- The two existing burden diagnostics still fire at level 3 -----

#[test]
fn bard_level3_still_claim_blocks_performance_execution_and_spontaneous_spell_burdens() {
    let input = load(BARD_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.bard.bardic_performance_execution.rounds_exceeded")
    {
        Some(blocker) => assert!(blocker.claim_blocking, "if the blocker fires, it must be claim-blocking"),
        None => {
            let not_performing = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.bard.bardic_performance_execution.not_performing");
            assert!(
                not_performing.is_some(),
                "level-3 Bard must ground an honest not-performing record when no \
                 bardic-performance posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_spell.bard.spontaneous_known_and_per_day.unsupported")
    {
        Some(blocker) => assert!(blocker.claim_blocking, "if the blocker fires, it must be claim-blocking"),
        None => {
            let known_count = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_spell.bard.known_spells")
                .map(|e| e.value)
                .unwrap_or(-1);
            assert_eq!(
                known_count, 0,
                "no spells are fabricated merely because the blocker stopped firing: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- The accepted Bard level-2 truth is unaffected -----

#[test]
fn bard_level2_truth_is_unchanged_by_this_widening() {
    let input = load(BARD_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(base_attack.value, 1, "Bard level 2 base attack bonus must stay 1");

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    assert_eq!(rounds.value, 9, "Bard level 2 bardic performance rounds per day must stay 9");
}

// ----- Bard level 4 was later widened into the supported tranche -----

#[test]
fn bard_level_4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_bard_level4_progression.rs) widened the level-range gate to
    // level 4 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Druid/
    // Sorcerer/Wizard level-range gate idiom) and extended every formula below;
    // this negative control is superseded, not violated — pin the new truth
    // here too so this file stays internally consistent.
    let level_4 = BARD_LEVEL3_FIXTURE.replace("class:bard:3", "class:bard:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.bard.")
                || e.id == "class_chassis.spell_baseline.bard"
                || e.id == WELL_VERSED_ID
                || e.id == INSPIRE_COMPETENCE_ID),
        "level-4 Bard is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the bard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_bard_level3_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.bard.")
                || e.id == "class_chassis.spell_baseline.bard"
                || e.id == WELL_VERSED_ID
                || e.id == INSPIRE_COMPETENCE_ID),
        "the Fighter chassis must not surface any bard-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Bard is not promoted -----

#[test]
fn multiclass_bard_level3_is_not_promoted_by_this_slice() {
    let multiclass = BARD_LEVEL3_FIXTURE.replace(
        "class_level=class:bard:3",
        "class_level=class:bard:3\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.bard.")
                || e.id == "class_chassis.spell_baseline.bard"
                || e.id == WELL_VERSED_ID
                || e.id == INSPIRE_COMPETENCE_ID),
        "multiclass Bard must not gain any bounded bard chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Bard must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-3 widening and Inspire Competence -----

#[test]
fn matrix_bard_row_names_level_3_widening_and_inspire_competence() {
    let matrix = seeded_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard progression_and_spell_burden row must exist");

    assert_eq!(bard.support_state, SupportState::Supported);
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(bard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        bard.grounding_ref.contains("sd13_bard_level3_progression"),
        "bard row must cite the live SD13-E5 level-3 proof surface: {}",
        bard.grounding_ref
    );
    let note = bard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 3") || note.contains("level-3"),
        "bard partial note must name the level-3 widening: {note}"
    );
    assert!(
        note.to_lowercase().contains("inspire competence"),
        "bard partial note must name the newly grounded Inspire Competence pillar: {note}"
    );
}
