//! SD13-E5 Barbarian rage power slots grounding proof — all five numbered
//! slots (barbarian levels 2/4/6/8/10) in one slice, the largest
//! application of the proven repeat-grant idiom and the discharge of the
//! rage-power-choice-list deferrals recorded across the barbarian level
//! slices. Both PF1 CRB primary sources (d20pfsrd and legacy.aonprd.com
//! Barbarian page) were read this cycle before writing any code, and both
//! state the rule identically: "Starting at 2nd level, a barbarian gains a
//! rage power. She gains another rage power for every two levels of
//! barbarian attained after 2nd level." / "Unless otherwise noted, a
//! barbarian cannot select an individual power more than once." / "A
//! barbarian gains the benefits of rage powers only while raging..."
//!
//! Five numbered choice slots (`choice:barbarian_rage_power` and
//! `_2`..`_5`, gates 2/4/6/8/10), each a bounded +0 record
//! (`class_chassis.barbarian.rage_power_choice`, `rage_power_2_choice`,
//! ...) via the open-ended recognition idiom (whichever raw power string
//! was selected, no power-list validation — the d20pfsrd superset caution
//! applies to rage power lists exactly as it did to mercies, and the
//! open-ended idiom sidesteps list encoding entirely). NOTHING about any
//! power's effect is computed: rage powers function only while raging, and
//! the rage-state execution engine (activation, round tracking, the
//! while-raging constants' application) is precisely the named engine
//! burden this row has carried since the rage slice — these recognitions
//! fabricate nothing from it.
//!
//! The proof fixture selects five distinct powers, honoring the no-repeat
//! clause. Gate ladder pinned: slot 1 silent at level 1; slot 3 silent at
//! level 5 while slot 2 fires; slot 5 silent at level 9 while slot 4
//! fires. Absent selections fabricate nothing; the Fighter and multiclass
//! negative controls are preserved.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::load;

const BARBARIAN_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_barbarian_level10_sd13_deterministic_input.txt");

const BARBARIAN_FIVE_POWERS_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level10_sd13_five_rage_powers_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

fn slot_id(n: u8) -> String {
    if n == 1 {
        "class_chassis.barbarian.rage_power_choice".to_owned()
    } else {
        format!("class_chassis.barbarian.rage_power_{n}_choice")
    }
}

fn detail_of(fixture: &str, id: &str) -> Option<String> {
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .map(|e| {
            assert_eq!(e.value, 0, "rage power recognitions must carry no fabricated value");
            e.detail.clone()
        })
}

// ----- All five numbered slots are recognized at level 10 -----

#[test]
fn barbarian_level10_recognizes_all_five_rage_power_slots() {
    let expectations = [
        (1u8, "power:animal_fury"),
        (2, "power:clear_mind"),
        (3, "power:guarded_stance"),
        (4, "power:knockback"),
        (5, "power:powerful_blow"),
    ];
    for (slot, power) in expectations {
        let detail = detail_of(BARBARIAN_FIVE_POWERS_FIXTURE, &slot_id(slot))
            .unwrap_or_else(|| panic!("slot {slot} must be recognized at level 10"));
        assert!(
            detail.contains(power),
            "slot {slot} must name its own raw chosen power string ({power}): {detail}"
        );
    }
}

// ----- The slots cite the rule and the while-raging non-fabrication posture -----

#[test]
fn barbarian_slots_cite_the_rule_text_and_the_rage_engine_posture() {
    for slot in [1u8, 3, 5] {
        let detail = detail_of(BARBARIAN_FIVE_POWERS_FIXTURE, &slot_id(slot))
            .unwrap_or_else(|| panic!("slot {slot} must be recognized"));
        assert!(
            detail.contains("every two levels")
                && detail.contains("cannot select an individual power more than once"),
            "slot {slot} must cite the verified rule text: {detail}"
        );
        assert!(
            detail.contains("only while raging")
                && (detail.contains("rage-state") || detail.contains("no rage")),
            "slot {slot} must state that powers function only while raging and the \
             rage-state engine stays the named burden: {detail}"
        );
    }
}

// ----- The gate ladder: each slot silent below its own gate -----

#[test]
fn barbarian_rage_power_slots_respect_their_own_level_gates() {
    let level_1 = BARBARIAN_FIVE_POWERS_FIXTURE.replace("class:barbarian:10", "class:barbarian:1");
    assert!(
        detail_of(&level_1, &slot_id(1)).is_none(),
        "slot 1 (gate 2) must be silent at level 1 even with the selection present"
    );

    let level_5 = BARBARIAN_FIVE_POWERS_FIXTURE.replace("class:barbarian:10", "class:barbarian:5");
    assert!(detail_of(&level_5, &slot_id(2)).is_some(), "slot 2 must fire at level 5");
    assert!(
        detail_of(&level_5, &slot_id(3)).is_none(),
        "slot 3 (gate 6) must be silent at level 5 even with the selection present"
    );

    let level_9 = BARBARIAN_FIVE_POWERS_FIXTURE.replace("class:barbarian:10", "class:barbarian:9");
    assert!(detail_of(&level_9, &slot_id(4)).is_some(), "slot 4 must fire at level 9");
    assert!(
        detail_of(&level_9, &slot_id(5)).is_none(),
        "slot 5 (gate 10) must be silent at level 9 even with the selection present"
    );
}

// ----- Absent selections fabricate nothing -----

#[test]
fn barbarian_level10_without_selections_fabricates_no_slot_records() {
    for slot in 1u8..=5 {
        assert!(
            detail_of(BARBARIAN_LEVEL10_FIXTURE, &slot_id(slot)).is_none(),
            "the plain level-10 fixture carries no rage power selections, so no slot-{slot} \
             record may exist"
        );
    }
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_barbarian_rage_power_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.rage_power")),
        "the Fighter chassis must not surface barbarian rage power records: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Barbarian is not promoted -----

#[test]
fn multiclass_barbarian_does_not_gain_rage_power_records() {
    let multiclass = BARBARIAN_FIVE_POWERS_FIXTURE.replace(
        "class_level=class:barbarian:10",
        "class_level=class:barbarian:10\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.rage_power")),
        "multiclass Barbarian must not gain rage power records: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Barbarian must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the completed slot family -----

#[test]
fn matrix_barbarian_row_names_the_completed_rage_power_family() {
    let matrix = seeded_current_truth();
    let barbarian = matrix
        .row("class.barbarian.bounded_progression")
        .expect("barbarian bounded_progression row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(barbarian.support_state, SupportState::Supported);
    assert_eq!(barbarian.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        barbarian.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        barbarian
            .grounding_ref
            .contains("sd13_barbarian_rage_power_slots"),
        "barbarian row must cite the live rage-power-slot proof surface: {}",
        barbarian.grounding_ref
    );
    assert!(
        barbarian
            .blocker_or_lossiness_note
            .contains("rage_power_5"),
        "barbarian partial note must name the completed five-slot family: {}",
        barbarian.blocker_or_lossiness_note
    );
}
