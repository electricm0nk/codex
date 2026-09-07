//! SD13-E5 Monk SECOND-bonus-feat grounding proof — the repeat-grant
//! PROVING slice. Every repeat-grant deferral in the campaign (rogue 2nd
//! talent, barbarian 2nd rage power, monk 2nd/6th/10th bonus feats,
//! paladin level-6/9 mercies, bard versatile-performance repeats) cited a
//! missing "list-growth mechanism"; the design decision this slice proves
//! is that no such mechanism is needed — a repeat grant grounds as a
//! NUMBERED CHOICE SLOT with its own level gate, exactly the idiom the
//! ranger slices already established (`choice:ranger_favored_enemy_2`,
//! `choice:ranger_favored_terrain_2`, `choice:ranger_combat_style_bonus_feat_2`).
//!
//! The instance: PF1 CRB Monk Bonus Feat, verified identically on both
//! primary sources this session: "At 1st level, 2nd level, and every 4
//! levels thereafter, a monk may select a bonus feat." The 2nd-level
//! repeat grant draws from the SAME corrected seven-feat list as slot 1
//! (Catch Off-Guard, Combat Reflexes, Deflect Arrows, Dodge, Improved
//! Grapple, Scorpion Style, Throw Anything — the list additions land at
//! 6th/10th, not 2nd). A new choice slot `choice:monk_bonus_feat_2`,
//! gated to monk level >= 2, is recognized as a bounded +0 record
//! (`class_chassis.monk.bonus_feat_2_choice`) with the same
//! automatic-grant exclusions (Improved Unarmed Strike, Stunning Fist)
//! and the same present-but-unrecognized branch.
//!
//! The deterministic proof fixture selects feat:combat_reflexes (slot 1
//! takes Deflect Arrows; the Human bonus feat takes Dodge — three
//! distinct feats). Absent the selection, nothing is fabricated (the
//! plain level-2 fixture computes exactly as before); below the level-2
//! gate the selection is not recognized even when present. The level-6/10
//! repeat grants stay named-but-unproven (their slots are future slices
//! of this same proven shape, with their own widened lists to verify).
//!
//! This slice also fixes a leftover from the list-correction slice: the
//! bonus-feat blocker's no-recognized-selection fallback branch still
//! carried the old wrong list text (no test covered that branch); it now
//! names the corrected list.
//!
//! It preserves the Fighter negative control and the multiclass negative
//! control, and the level-6/7/8 sibling controls (which pin exactly one
//! slot-1 record) stay green untouched — the slot-2 record has its own id
//! and their fixtures carry no slot-2 selection.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::load;

const MONK_LEVEL2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level2_sd13_deterministic_input.txt");

const MONK_SECOND_FEAT_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_monk_level2_sd13_second_bonus_feat_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const SLOT_2_ID: &str = "class_chassis.monk.bonus_feat_2_choice";

fn slot_2_detail(fixture: &str) -> Option<String> {
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);
    computation
        .explanations
        .iter()
        .find(|e| e.id == SLOT_2_ID)
        .map(|e| {
            assert_eq!(e.value, 0, "slot-2 recognition must carry no fabricated value");
            e.detail.clone()
        })
}

// ----- The level-2 repeat grant is recognized as a numbered slot -----

#[test]
fn monk_level2_second_bonus_feat_selection_is_recognized() {
    let detail = slot_2_detail(MONK_SECOND_FEAT_FIXTURE)
        .expect("the second bonus-feat slot must be recognized at level 2");
    assert!(
        detail.contains("Combat Reflexes") && detail.contains("choice:monk_bonus_feat_2"),
        "the slot-2 recognition must name the recognized selection: {detail}"
    );
    assert!(
        detail.contains("Catch Off-Guard")
            && detail.contains("Scorpion Style")
            && detail.contains("Throw Anything"),
        "the slot-2 recognition must cite the corrected seven-feat 1st/2nd-level list \
         (the 6th/10th-level additions are not yet in the list at 2nd level): {detail}"
    );
    assert!(
        detail.contains("every 4 levels thereafter") || detail.contains("2nd level"),
        "the slot-2 recognition must cite the repeat-grant rule text: {detail}"
    );
}

// ----- Absent the selection, nothing is fabricated -----

#[test]
fn monk_level2_without_the_selection_fabricates_no_slot_2_record() {
    assert!(
        slot_2_detail(MONK_LEVEL2_FIXTURE).is_none(),
        "the plain level-2 fixture carries no slot-2 selection, so no slot-2 record may \
         exist"
    );
}

// ----- Below the level-2 gate the selection is not recognized -----

#[test]
fn monk_level1_does_not_recognize_the_second_bonus_feat_selection() {
    let level_1 = MONK_SECOND_FEAT_FIXTURE.replace("class:monk:2", "class:monk:1");
    assert!(
        slot_2_detail(&level_1).is_none(),
        "the 2nd-level repeat grant must not be recognized below the PF1 CRB gate even \
         when present in chosen input"
    );
}

// ----- Out-of-list and automatic-grant branches -----

#[test]
fn monk_level2_slot_2_out_of_list_and_automatic_grant_branches() {
    let toughness =
        MONK_SECOND_FEAT_FIXTURE.replace("monk_bonus_feat_2:feat:combat_reflexes", "monk_bonus_feat_2:feat:toughness");
    let detail = slot_2_detail(&toughness)
        .expect("an out-of-list slot-2 selection must still surface as present");
    assert!(
        !detail.contains("drawn from the PF1 Core Rulebook restricted"),
        "an out-of-list selection must not be recognized as a list member: {detail}"
    );

    let stunning = MONK_SECOND_FEAT_FIXTURE.replace(
        "monk_bonus_feat_2:feat:combat_reflexes",
        "monk_bonus_feat_2:feat:stunning_fist",
    );
    let detail = slot_2_detail(&stunning)
        .expect("a stunning_fist slot-2 selection must still surface as present");
    assert!(
        detail.contains("Stunning Fist") && detail.contains("automatic"),
        "a stunning_fist selection must be surfaced as the automatic grant, not a list \
         member: {detail}"
    );
}

// ----- The blocker fallback carries the corrected list (leftover fix) -----

#[test]
fn monk_bonus_feat_blocker_fallback_names_the_corrected_list() {
    // Drive the fallback branch: slot 1 present but out-of-list.
    let unrecognized = MONK_LEVEL2_FIXTURE
        .replace("choice:monk_bonus_feat:feat:deflect_arrows", "choice:monk_bonus_feat:feat:toughness");
    let input = load(&unrecognized);
    let computation = compute_pilot_base_chassis(&input);
    let blocker = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.monk.bounded_progression.bonus_feat.unsupported")
        .expect("the bonus-feat blocker must fire");
    assert!(
        blocker.message.contains("Catch Off-Guard")
            && blocker.message.contains("Throw Anything")
            && !blocker.message.contains("Improved Trip, Stunning Fist)"),
        "the fallback branch must carry the corrected seven-feat list, not the old wrong \
         one (leftover from the list-correction slice): {}",
        blocker.message
    );
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_monk_slot_2_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation.explanations.iter().any(|e| e.id == SLOT_2_ID),
        "the Fighter chassis must not surface a monk slot-2 record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Monk is not promoted -----

#[test]
fn multiclass_monk_does_not_gain_slot_2_recognition() {
    let multiclass = MONK_SECOND_FEAT_FIXTURE.replace(
        "class_level=class:monk:2",
        "class_level=class:monk:2\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation.explanations.iter().any(|e| e.id == SLOT_2_ID),
        "multiclass Monk must not gain a slot-2 record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Monk must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the repeat-grant grounding -----

#[test]
fn matrix_monk_row_names_the_second_bonus_feat_grounding() {
    let matrix = seeded_current_truth();
    let monk = matrix
        .row("class.monk.bounded_progression")
        .expect("monk bounded_progression row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(monk.support_state, SupportState::Supported);
    assert_eq!(monk.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        monk.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        monk.grounding_ref.contains("sd13_monk_second_bonus_feat"),
        "monk row must cite the live second-bonus-feat proof surface: {}",
        monk.grounding_ref
    );
    assert!(
        monk.blocker_or_lossiness_note.contains("bonus_feat_2"),
        "monk partial note must name the grounded slot-2 record: {}",
        monk.blocker_or_lossiness_note
    );
}
