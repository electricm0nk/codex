//! SD13-E5 Monk bonus feats 3-4 grounding proof — the remaining numbered
//! bonus-feat slots (monk levels 6/10 per "At 1st level, 2nd level, and
//! every 4 levels thereafter, a monk may select a bonus feat"), landed
//! together as one slice on the proven numbered-slot idiom. Both list
//! ADDITIONS were verified identically on both primary sources this
//! session:
//!
//! - at 6th level: "Gorgon's Fist, Improved Bull Rush, Improved Disarm,
//!   Improved Feint, Improved Trip, and Mobility" join the base
//!   seven-feat list (13 recognizable feats at slot 3);
//! - at 10th level: "Improved Critical, Medusa's Wrath, Snatch Arrows,
//!   and Spring Attack" join (17 recognizable feats at slot 4).
//!
//! Two numbered choice slots: `choice:monk_bonus_feat_3` (level >= 6,
//! `class_chassis.monk.bonus_feat_3_choice`) and
//! `choice:monk_bonus_feat_4` (level >= 10,
//! `class_chassis.monk.bonus_feat_4_choice`), each a bounded +0
//! restricted-list recognition over its own widened list, with the same
//! automatic-grant exclusions (Improved Unarmed Strike, Stunning Fist)
//! and present-but-unrecognized branch as slots 1-2. With this slice the
//! monk's full four-slot bonus-feat count at the tranche ceiling is
//! recognized.
//!
//! THE PAYOFF PIN: slot 3 selects Improved Trip — the very feat the
//! list-correction slice removed from slots 1/2 — because at 6th level it
//! genuinely joins the list. The wrongness of the old slot-1 list and the
//! rightness of the slot-3 list are two sides of the same verified rule.
//!
//! Non-fabrication: absent selections fabricate nothing; each slot is
//! silent below its own gate (slot 3 at level 5, slot 4 at level 9) while
//! earlier slots still fire; a slot-4 out-of-list selection is surfaced
//! without a feat identity; the Fighter and multiclass negative controls
//! are preserved; and the level-6/7/8 sibling controls (pinning exactly
//! one slot-1 record) stay green untouched.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::load;

const MONK_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level10_sd13_deterministic_input.txt");

const MONK_FOUR_FEATS_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_monk_level10_sd13_four_bonus_feats_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const SLOT_3_ID: &str = "class_chassis.monk.bonus_feat_3_choice";
const SLOT_4_ID: &str = "class_chassis.monk.bonus_feat_4_choice";

fn detail_of(fixture: &str, id: &str) -> Option<String> {
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .map(|e| {
            assert_eq!(e.value, 0, "bonus-feat recognitions must carry no fabricated value");
            e.detail.clone()
        })
}

// ----- Slot 3 recognizes Improved Trip — legal at 6th, the correction's twin -----

#[test]
fn monk_level10_slot_3_recognizes_improved_trip_from_the_widened_list() {
    let detail = detail_of(MONK_FOUR_FEATS_FIXTURE, SLOT_3_ID)
        .expect("slot 3 must be recognized at level 10");
    assert!(
        detail.contains("Improved Trip") && detail.contains("choice:monk_bonus_feat_3"),
        "slot 3 must name Improved Trip — the feat the list-correction slice removed from \
         slots 1/2, genuinely legal at 6th: {detail}"
    );
    assert!(
        detail.contains("Gorgon's Fist") && detail.contains("Mobility"),
        "slot 3 must cite the 6th-level list additions: {detail}"
    );
}

// ----- Slot 4 recognizes Snatch Arrows from the 10th-level list -----

#[test]
fn monk_level10_slot_4_recognizes_snatch_arrows_from_the_widened_list() {
    let detail = detail_of(MONK_FOUR_FEATS_FIXTURE, SLOT_4_ID)
        .expect("slot 4 must be recognized at level 10");
    assert!(
        detail.contains("Snatch Arrows") && detail.contains("choice:monk_bonus_feat_4"),
        "slot 4 must name the recognized 10th-level selection: {detail}"
    );
    assert!(
        detail.contains("Medusa's Wrath") && detail.contains("Spring Attack"),
        "slot 4 must cite the 10th-level list additions: {detail}"
    );
}

// ----- The gate ladder: each slot silent below its own gate -----

#[test]
fn monk_bonus_feat_slots_respect_their_own_level_gates() {
    let level_5 = MONK_FOUR_FEATS_FIXTURE.replace("class:monk:10", "class:monk:5");
    assert!(
        detail_of(&level_5, SLOT_3_ID).is_none(),
        "slot 3 (gate 6) must be silent at level 5 even with the selection present"
    );

    let level_9 = MONK_FOUR_FEATS_FIXTURE.replace("class:monk:10", "class:monk:9");
    assert!(
        detail_of(&level_9, SLOT_3_ID).is_some(),
        "slot 3 must fire at level 9"
    );
    assert!(
        detail_of(&level_9, SLOT_4_ID).is_none(),
        "slot 4 (gate 10) must be silent at level 9 even with the selection present"
    );
}

// ----- Out-of-list selections fabricate no feat identity -----

#[test]
fn monk_slot_4_out_of_list_selection_grounds_no_identity() {
    let toughness = MONK_FOUR_FEATS_FIXTURE
        .replace("monk_bonus_feat_4:feat:snatch_arrows", "monk_bonus_feat_4:feat:toughness");
    let detail = detail_of(&toughness, SLOT_4_ID)
        .expect("an out-of-list slot-4 selection must still surface as present");
    assert!(
        !detail.contains("This character's selection names"),
        "an out-of-list selection must not be recognized as a list member: {detail}"
    );
}

// ----- Absent selections fabricate nothing -----

#[test]
fn monk_level10_without_selections_fabricates_no_new_slot_records() {
    assert!(
        detail_of(MONK_LEVEL10_FIXTURE, SLOT_3_ID).is_none()
            && detail_of(MONK_LEVEL10_FIXTURE, SLOT_4_ID).is_none(),
        "the plain level-10 fixture carries no slot-3/4 selections, so no records may exist"
    );
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_monk_slot_3_or_4_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == SLOT_3_ID || e.id == SLOT_4_ID),
        "the Fighter chassis must not surface monk slot-3/4 records: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Monk is not promoted -----

#[test]
fn multiclass_monk_does_not_gain_slot_3_or_4_recognition() {
    let multiclass = MONK_FOUR_FEATS_FIXTURE.replace(
        "class_level=class:monk:10",
        "class_level=class:monk:10\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == SLOT_3_ID || e.id == SLOT_4_ID),
        "multiclass Monk must not gain slot-3/4 records: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Monk must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the completed slot family -----

#[test]
fn matrix_monk_row_names_the_completed_bonus_feat_family() {
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
        monk.grounding_ref
            .contains("sd13_monk_bonus_feats_three_and_four"),
        "monk row must cite the live slot-3/4 proof surface: {}",
        monk.grounding_ref
    );
    assert!(
        monk.blocker_or_lossiness_note.contains("bonus_feat_4"),
        "monk partial note must name the completed four-slot family: {}",
        monk.blocker_or_lossiness_note
    );
}
