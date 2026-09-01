//! SD13-E5 Bard Versatile Performance slots grounding proof — the LAST row
//! of the repeat-grant queue, closing the tranche's slice-shaped frontier.
//! All three numbered slots (bard levels 2/6/10) in one slice on the
//! proven idiom. Both PF1 CRB primary sources (d20pfsrd and
//! legacy.aonprd.com Bard page) were read this cycle before writing any
//! code, and both state the rule identically: "At 2nd level, a bard can
//! choose one type of Perform skill. He can use his bonus in that skill in
//! place of his bonus in associated skills... At 6th level, and every 4
//! levels thereafter, the bard can select an additional type of Perform to
//! substitute." — with the same nine-type list on both sources: Act
//! (Bluff, Disguise), Comedy (Bluff, Intimidate), Dance (Acrobatics, Fly),
//! Keyboard Instruments (Diplomacy, Intimidate), Oratory (Diplomacy, Sense
//! Motive), Percussion (Handle Animal, Intimidate), Sing (Bluff, Sense
//! Motive), String (Bluff, Diplomacy), Wind (Diplomacy, Handle Animal).
//!
//! Three numbered choice slots (`choice:bard_versatile_performance`,
//! `_2`, `_3`, gates 2/6/10), each a bounded +0 RESTRICTED-LIST
//! recognition (`class_chassis.bard.versatile_performance_choice`,
//! `versatile_performance_2_choice`, `versatile_performance_3_choice`)
//! over the verified nine types, with the selected type's associated
//! skill pair named in the detail. The skill-SUBSTITUTION engine (using
//! the Perform bonus in place of the associated skills' bonuses) is
//! exactly the choice-gated engine burden the bard's performance blocker
//! has always named — these recognitions modify no skill total and
//! fabricate nothing from it.
//!
//! The proof fixture selects three distinct types (oratory/sing/dance).
//! Gate ladder pinned: slot 1 silent at level 1; slot 2 silent at level 5
//! while slot 1 fires; slot 3 silent at level 9 while slot 2 fires.
//! Absent selections fabricate nothing; an out-of-list selection grounds
//! no type identity; the Fighter and multiclass negative controls are
//! preserved.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::load;

const BARD_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level10_sd13_deterministic_input.txt");

const BARD_THREE_VP_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_bard_level10_sd13_three_versatile_performances_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

fn slot_id(n: u8) -> String {
    if n == 1 {
        "class_chassis.bard.versatile_performance_choice".to_owned()
    } else {
        format!("class_chassis.bard.versatile_performance_{n}_choice")
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
            assert_eq!(e.value, 0, "versatile performance recognitions must carry no value");
            e.detail.clone()
        })
}

// ----- All three slots recognized at level 10, with the verified pairs -----

#[test]
fn bard_level10_recognizes_all_three_versatile_performance_slots() {
    let slot_1 = detail_of(BARD_THREE_VP_FIXTURE, &slot_id(1))
        .expect("slot 1 must be recognized at level 10");
    assert!(
        slot_1.contains("perform:oratory")
            && slot_1.contains("Diplomacy")
            && slot_1.contains("Sense Motive"),
        "slot 1 must name Oratory and its verified associated pair: {slot_1}"
    );

    let slot_2 = detail_of(BARD_THREE_VP_FIXTURE, &slot_id(2))
        .expect("slot 2 must be recognized at level 10");
    assert!(
        slot_2.contains("perform:sing")
            && slot_2.contains("Bluff")
            && slot_2.contains("Sense Motive"),
        "slot 2 must name Sing and its verified associated pair: {slot_2}"
    );

    let slot_3 = detail_of(BARD_THREE_VP_FIXTURE, &slot_id(3))
        .expect("slot 3 must be recognized at level 10");
    assert!(
        slot_3.contains("perform:dance")
            && slot_3.contains("Acrobatics")
            && slot_3.contains("Fly"),
        "slot 3 must name Dance and its verified associated pair: {slot_3}"
    );
    assert!(
        slot_3.contains("substitution") && slot_3.contains("no skill total"),
        "slot 3 must state the substitution engine stays unproven and no skill total is \
         modified: {slot_3}"
    );
}

// ----- The gate ladder: each slot silent below its own gate -----

#[test]
fn bard_versatile_performance_slots_respect_their_own_level_gates() {
    let level_1 = BARD_THREE_VP_FIXTURE.replace("class:bard:10", "class:bard:1");
    assert!(
        detail_of(&level_1, &slot_id(1)).is_none(),
        "slot 1 (gate 2) must be silent at level 1 even with the selection present"
    );

    let level_5 = BARD_THREE_VP_FIXTURE.replace("class:bard:10", "class:bard:5");
    assert!(detail_of(&level_5, &slot_id(1)).is_some(), "slot 1 must fire at level 5");
    assert!(
        detail_of(&level_5, &slot_id(2)).is_none(),
        "slot 2 (gate 6) must be silent at level 5 even with the selection present"
    );

    let level_9 = BARD_THREE_VP_FIXTURE.replace("class:bard:10", "class:bard:9");
    assert!(detail_of(&level_9, &slot_id(2)).is_some(), "slot 2 must fire at level 9");
    assert!(
        detail_of(&level_9, &slot_id(3)).is_none(),
        "slot 3 (gate 10) must be silent at level 9 even with the selection present"
    );
}

// ----- An out-of-list selection grounds no type identity -----

#[test]
fn bard_out_of_list_perform_type_grounds_no_identity() {
    let kazoo = BARD_THREE_VP_FIXTURE
        .replace("bard_versatile_performance:perform:oratory", "bard_versatile_performance:perform:kazoo");
    let detail = detail_of(&kazoo, &slot_id(1))
        .expect("an out-of-list selection must still surface as present");
    assert!(
        !detail.contains("associated"),
        "an out-of-list Perform type must not be recognized with an associated pair: {detail}"
    );
}

// ----- Absent selections fabricate nothing -----

#[test]
fn bard_level10_without_selections_fabricates_no_slot_records() {
    for slot in 1u8..=3 {
        assert!(
            detail_of(BARD_LEVEL10_FIXTURE, &slot_id(slot)).is_none(),
            "the plain level-10 fixture carries no versatile performance selections, so no \
             slot-{slot} record may exist"
        );
    }
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_bard_versatile_performance_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.bard.versatile_performance")),
        "the Fighter chassis must not surface versatile performance records: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Bard is not promoted -----

#[test]
fn multiclass_bard_does_not_gain_versatile_performance_records() {
    let multiclass = BARD_THREE_VP_FIXTURE.replace(
        "class_level=class:bard:10",
        "class_level=class:bard:10\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.bard.versatile_performance")),
        "multiclass Bard must not gain versatile performance records: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Bard must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the completed slot family -----

#[test]
fn matrix_bard_row_names_the_completed_versatile_performance_family() {
    let matrix = seeded_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard progression_and_spell_burden row must exist");

    assert_eq!(bard.support_state, SupportState::Supported);
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        bard.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        bard.grounding_ref
            .contains("sd13_bard_versatile_performance_slots"),
        "bard row must cite the live versatile-performance proof surface: {}",
        bard.grounding_ref
    );
    assert!(
        bard.blocker_or_lossiness_note
            .contains("versatile_performance_3"),
        "bard partial note must name the completed three-slot family: {}",
        bard.blocker_or_lossiness_note
    );
}
