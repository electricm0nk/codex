//! SD13-E5 Paladin mercies 2-3 grounding proof — the level-6 and level-9
//! repeat mercy grants, landed together on the proven numbered-slot idiom
//! and finally discharging the "mercy-list-growth mechanism" deferrals the
//! level-6 and level-9 chassis slices recorded. The CRB mercy tiers were
//! verified this cycle: legacy.aonprd.com (Core Rulebook only) gives 3rd =
//! Fatigued/Shaken/Sickened, 6th ADDS Dazed/Diseased/Staggered, 9th ADDS
//! Cursed/Exhausted/Frightened/Nauseated/Poisoned; d20pfsrd's lists are
//! SUPERSETS whose CRB subset matches exactly (its extra entries —
//! Deceived, Riled, Enfeebled, Haunted, Targeted, Confused, Injured,
//! Restorative — are non-CRB expansions outside this seam's
//! pf1.core_rulebook source package, documented rather than encoded).
//!
//! Two numbered choice slots, mirroring slot 1's open-ended recognition
//! idiom: `choice:paladin_mercy_2` (level >= 6,
//! `class_chassis.paladin.mercy_2_choice`) and `choice:paladin_mercy_3`
//! (level >= 9, `class_chassis.paladin.mercy_3_choice`), each a bounded
//! +0 record naming whichever raw mercy string was selected, with the CRB
//! tier lists cited in the detail. The proof fixture's chain is coherent:
//! slot 1 shaken, slot 2 staggered (a 6th-tier addition), slot 3
//! frightened (a 9th-tier addition whose rule text requires the shaken
//! mercy — named as chosen input, prerequisite NOT validated, exactly as
//! slot 1 never validated tier membership). No mercy's effect is computed
//! (no lay-on-hands execution engine exists), and nothing is fabricated.
//!
//! Non-fabrication: absent selections fabricate nothing (every existing
//! per-level paladin fixture carries only slot 1); each slot is silent
//! below its own gate (slot 2 at level 5, slot 3 at level 8 while slot 2
//! fires); the Fighter and multiclass negative controls are preserved;
//! and the level-6/9 sibling controls (pinning slot 1's values) stay
//! green untouched.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::load;

const PALADIN_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level10_sd13_deterministic_input.txt");

const PALADIN_THREE_MERCIES_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_paladin_level10_sd13_three_mercies_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const SLOT_2_ID: &str = "class_chassis.paladin.mercy_2_choice";
const SLOT_3_ID: &str = "class_chassis.paladin.mercy_3_choice";

fn detail_of(fixture: &str, id: &str) -> Option<String> {
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .map(|e| {
            assert_eq!(e.value, 0, "mercy recognitions must carry no fabricated value");
            e.detail.clone()
        })
}

// ----- Both repeat slots are recognized at level 10 -----

#[test]
fn paladin_level10_recognizes_both_repeat_mercy_slots() {
    let slot_2 = detail_of(PALADIN_THREE_MERCIES_FIXTURE, SLOT_2_ID)
        .expect("the 6th-level mercy slot must be recognized at level 10");
    assert!(
        slot_2.contains("mercy:staggered") && slot_2.contains("choice:paladin_mercy_2"),
        "slot 2 must name the raw chosen mercy string: {slot_2}"
    );
    assert!(
        slot_2.contains("Dazed") && slot_2.contains("Diseased") && slot_2.contains("Staggered"),
        "slot 2 must cite the verified 6th-level CRB tier additions: {slot_2}"
    );

    let slot_3 = detail_of(PALADIN_THREE_MERCIES_FIXTURE, SLOT_3_ID)
        .expect("the 9th-level mercy slot must be recognized at level 10");
    assert!(
        slot_3.contains("mercy:frightened") && slot_3.contains("choice:paladin_mercy_3"),
        "slot 3 must name the raw chosen mercy string: {slot_3}"
    );
    assert!(
        slot_3.contains("Cursed")
            && slot_3.contains("Exhausted")
            && slot_3.contains("Nauseated")
            && slot_3.contains("Poisoned"),
        "slot 3 must cite the verified 9th-level CRB tier additions: {slot_3}"
    );
    assert!(
        slot_3.contains("prerequisite") || slot_3.contains("requires the shaken"),
        "slot 3 must name (without validating) the frightened mercy's shaken prerequisite: \
         {slot_3}"
    );
}

// ----- The gate ladder: each slot silent below its own gate -----

#[test]
fn paladin_mercy_slots_respect_their_own_level_gates() {
    let level_5 = PALADIN_THREE_MERCIES_FIXTURE.replace("class:paladin:10", "class:paladin:5");
    assert!(
        detail_of(&level_5, SLOT_2_ID).is_none(),
        "slot 2 (gate 6) must be silent at level 5 even with the selection present"
    );

    let level_8 = PALADIN_THREE_MERCIES_FIXTURE.replace("class:paladin:10", "class:paladin:8");
    assert!(
        detail_of(&level_8, SLOT_2_ID).is_some(),
        "slot 2 must fire at level 8"
    );
    assert!(
        detail_of(&level_8, SLOT_3_ID).is_none(),
        "slot 3 (gate 9) must be silent at level 8 even with the selection present"
    );
}

// ----- Absent selections fabricate nothing -----

#[test]
fn paladin_level10_without_selections_fabricates_no_repeat_slot_records() {
    assert!(
        detail_of(PALADIN_LEVEL10_FIXTURE, SLOT_2_ID).is_none()
            && detail_of(PALADIN_LEVEL10_FIXTURE, SLOT_3_ID).is_none(),
        "the plain level-10 fixture carries only slot 1, so no repeat-slot records may exist"
    );
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_paladin_mercy_slot_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == SLOT_2_ID || e.id == SLOT_3_ID),
        "the Fighter chassis must not surface paladin mercy slot records: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Paladin is not promoted -----

#[test]
fn multiclass_paladin_does_not_gain_mercy_slot_records() {
    let multiclass = PALADIN_THREE_MERCIES_FIXTURE.replace(
        "class_level=class:paladin:10",
        "class_level=class:paladin:10\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == SLOT_2_ID || e.id == SLOT_3_ID),
        "multiclass Paladin must not gain mercy slot records: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Paladin must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the completed mercy slot family -----

#[test]
fn matrix_paladin_row_names_the_completed_mercy_family() {
    let matrix = seeded_current_truth();
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin hybrid_chassis_and_spell_burden row must exist");

    assert_eq!(paladin.support_state, SupportState::Supported);
    assert_eq!(paladin.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        paladin.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        paladin
            .grounding_ref
            .contains("sd13_paladin_mercies_two_and_three"),
        "paladin row must cite the live mercy-slot proof surface: {}",
        paladin.grounding_ref
    );
    assert!(
        paladin.blocker_or_lossiness_note.contains("mercy_3"),
        "paladin partial note must name the completed three-slot mercy family: {}",
        paladin.blocker_or_lossiness_note
    );
}
