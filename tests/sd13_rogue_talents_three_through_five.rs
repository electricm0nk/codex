//! SD13-E5 Rogue talents 3-5 grounding proof — the remaining numbered
//! talent slots (rogue levels 6/8/10), landed together as one slice: they
//! are three instances of the same proven numbered-slot idiom on the same
//! matrix row (the shape was proven at monk slot 2 and rogue slots 1-2,
//! and the goal file's one-row rule permits multiple grants of one row's
//! feature family in one slice). The rule text was verified identically
//! on both primary sources this session: "Starting at 2nd level, a rogue
//! gains one rogue talent. She gains an additional rogue talent for every
//! 2 levels of rogue attained after 2nd level. A rogue cannot select an
//! individual talent more than once." — additional talents land at rogue
//! levels 4, 6, 8, and 10.
//!
//! Three new numbered choice slots, each gated to its own level:
//! `choice:rogue_talent_3` (level >= 6, `class_chassis.rogue.talent_3_choice`),
//! `choice:rogue_talent_4` (level >= 8, `class_chassis.rogue.talent_4_choice`),
//! `choice:rogue_talent_5` (level >= 10, `class_chassis.rogue.talent_5_choice`)
//! — the same open-ended +0 recognition as slots 1-2. With this slice the
//! rogue's full level-10 talent slot count (five) is recognized; the
//! talent tree's EFFECTS stay the named new-subsystem burden, and nothing
//! is fabricated from any recognition.
//!
//! The deterministic proof fixture selects five distinct talents
//! (trap_spotter / fast_stealth / slow_reactions / stand_up /
//! ledge_walker), honoring the no-repeat clause. The gate ladder is
//! pinned: each slot is silent one level below its own gate while the
//! earlier slots still fire; absent selections fabricate nothing; the
//! Fighter and multiclass negative controls are preserved.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::load;

const ROGUE_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level10_sd13_deterministic_input.txt");

const ROGUE_FIVE_TALENTS_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level10_sd13_five_talents_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

fn slot_id(n: u8) -> String {
    if n == 1 {
        "class_chassis.rogue.talent_choice".to_owned()
    } else {
        format!("class_chassis.rogue.talent_{n}_choice")
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
            assert_eq!(e.value, 0, "talent recognitions must carry no fabricated value");
            e.detail.clone()
        })
}

// ----- All five numbered slots are recognized at level 10 -----

#[test]
fn rogue_level10_recognizes_all_five_talent_slots() {
    let expectations = [
        (1u8, "talent:trap_spotter"),
        (2, "talent:fast_stealth"),
        (3, "talent:slow_reactions"),
        (4, "talent:stand_up"),
        (5, "talent:ledge_walker"),
    ];
    for (slot, talent) in expectations {
        let detail = detail_of(ROGUE_FIVE_TALENTS_FIXTURE, &slot_id(slot))
            .unwrap_or_else(|| panic!("slot {slot} must be recognized at level 10"));
        assert!(
            detail.contains(talent),
            "slot {slot} must name its own raw chosen talent string ({talent}): {detail}"
        );
    }
}

// ----- The new slots cite the rule and the no-effect posture -----

#[test]
fn rogue_new_slots_cite_the_rule_text_and_no_effect_posture() {
    for slot in [3u8, 4, 5] {
        let detail = detail_of(ROGUE_FIVE_TALENTS_FIXTURE, &slot_id(slot))
            .unwrap_or_else(|| panic!("slot {slot} must be recognized"));
        assert!(
            detail.contains("every 2 levels")
                && detail.contains("cannot select an individual talent more than once"),
            "slot {slot} must cite the verified repeat-grant rule text: {detail}"
        );
        assert!(
            detail.contains("no talent-effect engine") || detail.contains("not computed"),
            "slot {slot} must state the talent's own effect stays unproven: {detail}"
        );
    }
}

// ----- The gate ladder: each slot silent one level below its gate -----

#[test]
fn rogue_talent_slots_respect_their_own_level_gates() {
    // Level 5: slots 1-2 fire, slot 3 silent.
    let level_5 = ROGUE_FIVE_TALENTS_FIXTURE.replace("class:rogue:10", "class:rogue:5");
    assert!(detail_of(&level_5, &slot_id(2)).is_some(), "slot 2 must fire at level 5");
    assert!(
        detail_of(&level_5, &slot_id(3)).is_none(),
        "slot 3 (gate 6) must be silent at level 5 even with the selection present"
    );

    // Level 7: slot 3 fires, slot 4 silent.
    let level_7 = ROGUE_FIVE_TALENTS_FIXTURE.replace("class:rogue:10", "class:rogue:7");
    assert!(detail_of(&level_7, &slot_id(3)).is_some(), "slot 3 must fire at level 7");
    assert!(
        detail_of(&level_7, &slot_id(4)).is_none(),
        "slot 4 (gate 8) must be silent at level 7 even with the selection present"
    );

    // Level 9: slot 4 fires, slot 5 silent.
    let level_9 = ROGUE_FIVE_TALENTS_FIXTURE.replace("class:rogue:10", "class:rogue:9");
    assert!(detail_of(&level_9, &slot_id(4)).is_some(), "slot 4 must fire at level 9");
    assert!(
        detail_of(&level_9, &slot_id(5)).is_none(),
        "slot 5 (gate 10) must be silent at level 9 even with the selection present"
    );
}

// ----- Absent selections fabricate nothing -----

#[test]
fn rogue_level10_without_selections_fabricates_no_new_slot_records() {
    for slot in [3u8, 4, 5] {
        assert!(
            detail_of(ROGUE_LEVEL10_FIXTURE, &slot_id(slot)).is_none(),
            "the plain level-10 fixture carries no slot-{slot} selection, so no record may \
             exist"
        );
    }
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_talent_slot_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    for slot in [3u8, 4, 5] {
        assert!(
            !computation.explanations.iter().any(|e| e.id == slot_id(slot)),
            "the Fighter chassis must not surface rogue slot-{slot} records: {:?}",
            computation.explanations
        );
    }
}

// ----- Negative control: multiclass Rogue is not promoted -----

#[test]
fn multiclass_rogue_does_not_gain_talent_slot_records() {
    let multiclass = ROGUE_FIVE_TALENTS_FIXTURE.replace(
        "class_level=class:rogue:10",
        "class_level=class:rogue:10\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    for slot in [3u8, 4, 5] {
        assert!(
            !computation.explanations.iter().any(|e| e.id == slot_id(slot)),
            "multiclass Rogue must not gain slot-{slot} records: {:?}",
            computation.explanations
        );
    }
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Rogue must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the completed slot family -----

#[test]
fn matrix_rogue_row_names_the_completed_talent_slot_family() {
    let matrix = seeded_current_truth();
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue bounded_progression row must exist");

    assert_eq!(rogue.support_state, SupportState::Supported); // promoted by SD-19 Class Progression Catalog browser
    assert_eq!(rogue.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        rogue.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        rogue
            .grounding_ref
            .contains("sd13_rogue_talents_three_through_five"),
        "rogue row must cite the live slot-3-5 proof surface: {}",
        rogue.grounding_ref
    );
    assert!(
        rogue.blocker_or_lossiness_note.contains("talent_5"),
        "rogue partial note must name the completed five-slot family: {}",
        rogue.blocker_or_lossiness_note
    );
}
