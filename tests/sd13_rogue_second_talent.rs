//! SD13-E5 Rogue SECOND-talent grounding proof — the first numbered-slot
//! mirror in the rogue's repeat-grant queue, following the proven
//! monk-second-bonus-feat idiom on top of the grounded 2nd-level talent
//! slot (`tests/sd13_rogue_talent_choice.rs`). The rule text was verified
//! identically on both primary sources this session: "Starting at 2nd
//! level, a rogue gains one rogue talent. She gains an additional rogue
//! talent for every 2 levels of rogue attained after 2nd level. A rogue
//! cannot select an individual talent more than once." — the first
//! additional talent lands at rogue level 4.
//!
//! A new numbered choice slot `choice:rogue_talent_2`, gated to rogue
//! level >= 4, is recognized as a bounded +0 record
//! (`class_chassis.rogue.talent_2_choice`) via the same open-ended
//! recognition idiom as slot 1. The deterministic proof fixture selects
//! talent:fast_stealth for slot 2 alongside slot 1's talent:trap_spotter
//! — distinct talents, honoring the no-repeat-selection clause. Neither
//! talent's own effect is computed (the talent tree stays the named
//! new-subsystem burden); the 6th/8th/10th additional talents stay
//! named-but-unproven as future numbered slots.
//!
//! Absent the slot-2 selection, nothing is fabricated; below the level-4
//! gate the selection is not recognized even when present (while slot 1
//! still fires); the Fighter and multiclass negative controls are
//! preserved.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::load;

const ROGUE_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level4_sd13_deterministic_input.txt");

const ROGUE_SECOND_TALENT_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level4_sd13_second_talent_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const SLOT_1_ID: &str = "class_chassis.rogue.talent_choice";
const SLOT_2_ID: &str = "class_chassis.rogue.talent_2_choice";

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

// ----- Both numbered slots are recognized at level 4 -----

#[test]
fn rogue_level4_second_talent_selection_is_recognized_alongside_the_first() {
    let slot_1 = detail_of(ROGUE_SECOND_TALENT_FIXTURE, SLOT_1_ID)
        .expect("the 2nd-level talent slot must still fire at level 4");
    assert!(
        slot_1.contains("talent:trap_spotter"),
        "slot 1 must keep naming its own selection: {slot_1}"
    );

    let slot_2 = detail_of(ROGUE_SECOND_TALENT_FIXTURE, SLOT_2_ID)
        .expect("the second talent slot must be recognized at level 4");
    assert!(
        slot_2.contains("talent:fast_stealth") && slot_2.contains("choice:rogue_talent_2"),
        "slot 2 must name the raw chosen talent string: {slot_2}"
    );
    assert!(
        slot_2.contains("every 2 levels")
            && slot_2.contains("cannot select an individual talent more than once"),
        "slot 2 must cite the verified repeat-grant rule text: {slot_2}"
    );
    assert!(
        slot_2.contains("no talent-effect engine") || slot_2.contains("not computed"),
        "slot 2 must state the talent's own effect stays unproven: {slot_2}"
    );
}

// ----- Absent the slot-2 selection, nothing is fabricated -----

#[test]
fn rogue_level4_without_the_selection_fabricates_no_slot_2_record() {
    assert!(
        detail_of(ROGUE_LEVEL4_FIXTURE, SLOT_2_ID).is_none(),
        "the plain level-4 fixture carries no slot-2 selection, so no slot-2 record may \
         exist"
    );
}

// ----- Below the level-4 gate slot 2 is not recognized (slot 1 still fires) -----

#[test]
fn rogue_level3_does_not_recognize_the_second_talent_selection() {
    let level_3 = ROGUE_SECOND_TALENT_FIXTURE.replace("class:rogue:4", "class:rogue:3");
    assert!(
        detail_of(&level_3, SLOT_2_ID).is_none(),
        "the level-4 repeat grant must not be recognized below the PF1 CRB gate even when \
         present in chosen input"
    );
    assert!(
        detail_of(&level_3, SLOT_1_ID).is_some(),
        "the 2nd-level slot must still fire at level 3"
    );
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_slot_2_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation.explanations.iter().any(|e| e.id == SLOT_2_ID),
        "the Fighter chassis must not surface a rogue slot-2 record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Rogue is not promoted -----

#[test]
fn multiclass_rogue_does_not_gain_slot_2_recognition() {
    let multiclass = ROGUE_SECOND_TALENT_FIXTURE.replace(
        "class_level=class:rogue:4",
        "class_level=class:rogue:4\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation.explanations.iter().any(|e| e.id == SLOT_2_ID),
        "multiclass Rogue must not gain a slot-2 record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Rogue must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the slot-2 grounding -----

#[test]
fn matrix_rogue_row_names_the_second_talent_grounding() {
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
        rogue.grounding_ref.contains("sd13_rogue_second_talent"),
        "rogue row must cite the live second-talent proof surface: {}",
        rogue.grounding_ref
    );
    assert!(
        rogue.blocker_or_lossiness_note.contains("talent_2"),
        "rogue partial note must name the grounded slot-2 record: {}",
        rogue.blocker_or_lossiness_note
    );
}
