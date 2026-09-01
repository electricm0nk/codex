//! SD13-E5 Rogue talent choice-slot grounding proof — the first talent
//! recognition, opening the rogue's repeat-grant queue on the shape the
//! monk second-bonus-feat slice proved (a grant is a choice slot with a
//! level gate; repeats are numbered slots). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Rogue page) were read this cycle
//! before writing any code, and both state the rule identically:
//! "Starting at 2nd level, a rogue gains one rogue talent. She gains an
//! additional rogue talent for every 2 levels of rogue attained after
//! 2nd level. A rogue cannot select an individual talent more than once."
//!
//! A new choice slot `choice:rogue_talent`, gated to rogue level >= 2, is
//! recognized as a bounded +0 record (`class_chassis.rogue.talent_choice`)
//! naming whichever raw talent string was actually selected — the
//! open-ended (non-restricted-list) recognition idiom of
//! `choice:ranger_favored_enemy` / `choice:ranger_favored_terrain` /
//! `choice:paladin_mercy`, deliberately NOT an encoding of the talent
//! list. The selected talent's own EFFECT stays entirely unproven: the
//! talent tree is the new-subsystem-shaped burden the level-2 slice
//! documented, and this recognition fabricates nothing from it — no
//! talent-effect engine exists in this codebase. The additional talents
//! at levels 4/6/8/10 stay named-but-unproven as future NUMBERED slots
//! per the proven monk-second-bonus-feat idiom.
//!
//! The deterministic proof fixture selects talent:trap_spotter as chosen
//! input only. Absent the selection, nothing is fabricated (the plain
//! level-2 fixture computes exactly as before); below the level-2 gate
//! the selection is not recognized even when present.
//!
//! It preserves the Fighter negative control, the multiclass negative
//! control, and the level-4 sibling's pin that the matrix note keeps
//! naming rogue talents (their effects) as unproven.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::load;

const ROGUE_LEVEL2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level2_sd13_deterministic_input.txt");

const ROGUE_TALENT_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level2_sd13_talent_choice_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const TALENT_CHOICE_ID: &str = "class_chassis.rogue.talent_choice";

fn talent_detail(fixture: &str) -> Option<String> {
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);
    computation
        .explanations
        .iter()
        .find(|e| e.id == TALENT_CHOICE_ID)
        .map(|e| {
            assert_eq!(e.value, 0, "talent recognition must carry no fabricated value");
            e.detail.clone()
        })
}

// ----- The level-2 talent selection is recognized -----

#[test]
fn rogue_level2_talent_selection_is_recognized() {
    let detail = talent_detail(ROGUE_TALENT_FIXTURE)
        .expect("the talent choice slot must be recognized at level 2");
    assert!(
        detail.contains("talent:trap_spotter") && detail.contains("choice:rogue_talent"),
        "the recognition must name the raw chosen talent string: {detail}"
    );
    assert!(
        detail.contains("cannot select an individual talent more than once"),
        "the recognition must cite the verified rule text: {detail}"
    );
    assert!(
        detail.contains("no talent-effect engine") || detail.contains("not computed"),
        "the recognition must state the talent's own effect stays unproven: {detail}"
    );
}

// ----- Absent the selection, nothing is fabricated -----

#[test]
fn rogue_level2_without_the_selection_fabricates_no_talent_record() {
    assert!(
        talent_detail(ROGUE_LEVEL2_FIXTURE).is_none(),
        "the plain level-2 fixture carries no talent selection, so no talent record may \
         exist"
    );
}

// ----- Below the level-2 gate the selection is not recognized -----

#[test]
fn rogue_level1_does_not_recognize_the_talent_selection() {
    let level_1 = ROGUE_TALENT_FIXTURE.replace("class:rogue:2", "class:rogue:1");
    assert!(
        talent_detail(&level_1).is_none(),
        "the 2nd-level talent grant must not be recognized below the PF1 CRB gate even \
         when present in chosen input"
    );
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_talent_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation.explanations.iter().any(|e| e.id == TALENT_CHOICE_ID),
        "the Fighter chassis must not surface a rogue talent record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Rogue is not promoted -----

#[test]
fn multiclass_rogue_does_not_gain_talent_recognition() {
    let multiclass = ROGUE_TALENT_FIXTURE.replace(
        "class_level=class:rogue:2",
        "class_level=class:rogue:2\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation.explanations.iter().any(|e| e.id == TALENT_CHOICE_ID),
        "multiclass Rogue must not gain a talent record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Rogue must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the talent-choice grounding -----

#[test]
fn matrix_rogue_row_names_the_talent_choice_grounding() {
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
        rogue.grounding_ref.contains("sd13_rogue_talent_choice"),
        "rogue row must cite the live talent-choice proof surface: {}",
        rogue.grounding_ref
    );
    assert!(
        rogue.blocker_or_lossiness_note.contains("talent_choice"),
        "rogue partial note must name the grounded talent-choice record: {}",
        rogue.blocker_or_lossiness_note
    );
    assert!(
        rogue.blocker_or_lossiness_note.contains("rogue talent"),
        "rogue partial note must keep naming the talent tree's effects as unproven: {}",
        rogue.blocker_or_lossiness_note
    );
}
