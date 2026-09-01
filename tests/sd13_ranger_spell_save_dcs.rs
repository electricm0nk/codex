//! SD13-E5 Ranger spell-save-DC grounding proof — the fourth and final
//! spell-DC slice, completing the DC family (Sorcerer, Bard, Paladin
//! before it) and the flat spell surface across all four
//! partial/spontaneous casters in the tranche. Both PF1 CRB primary
//! sources (d20pfsrd and legacy.aonprd.com Ranger page) were read this
//! cycle before writing any code, and both state the DC rule identically:
//! "The Difficulty Class for a saving throw against a ranger's spell is
//! 10 + the spell level + the ranger's Wisdom modifier." — the family's
//! only WISDOM caster (the other three key to Charisma), flat arithmetic
//! over values already grounded on the seam (the chosen-ability Wisdom
//! modifier and the access ladder).
//!
//! One record per ACCESSIBLE spell level:
//! `class_chassis.ranger.partial_caster.spell_save_dc.spell_level_<N>`
//! with value `10 + N + Wisdom modifier`. On the deterministic fixtures
//! (Wisdom 12, modifier +1) that is DC 12/13/14 for spell levels 1-3 as
//! they become accessible; below the level-4 spell gate no DC record
//! exists at all. The arithmetic is live, not hardcoded: a raised fixture
//! Wisdom raises the DCs (asserted below).
//!
//! This grounds only the base DC formula: no saving-throw resolution, no
//! target, no spell selection, and no feat DC modifiers are computed.
//! The prepared posture, Wisdom bonus slots, and spell-source lineage
//! stay unproven; NO new claim-blocking diagnostic is added (the ranger
//! has no per-level spell blocker — the burden stays named by the F6
//! level-1 hybrid blocker and the matrix note), and the computation stays
//! claim-blocked overall via the generic chassis diagnostics.
//!
//! The sibling `tests/sd13_ranger_spell_level_thresholds.rs` pinned
//! "exactly two partial_caster records" (already excluding the
//! base_spells_per_day family); that control is extended to exclude the
//! new spell_save_dc family too, preserving its original claim.
//!
//! It reuses the accepted per-level ranger fixtures (no new fixture) and
//! preserves the Fighter negative control and the multiclass negative
//! control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::load;

const RANGER_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level3_sd13_deterministic_input.txt");
const RANGER_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level4_sd13_deterministic_input.txt");
const RANGER_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level7_sd13_deterministic_input.txt");
const RANGER_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const DC_PREFIX: &str = "class_chassis.ranger.partial_caster.spell_save_dc.";

fn dc_values(fixture: &str) -> Vec<(String, i16)> {
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);
    computation
        .explanations
        .iter()
        .filter(|e| e.id.starts_with(DC_PREFIX))
        .map(|e| (e.id.clone(), e.value))
        .collect()
}

fn id(spell_level: u8) -> String {
    format!("{DC_PREFIX}spell_level_{spell_level}")
}

// ----- Below the level-4 spell gate: no DC records at all -----

#[test]
fn ranger_level3_has_no_dc_records() {
    assert!(
        dc_values(RANGER_LEVEL3_FIXTURE).is_empty(),
        "a level-3 ranger has no accessible spell level, so no spell-save-DC record may exist"
    );
}

// ----- The DC ladder tracks the access ladder at every step -----

#[test]
fn ranger_spell_save_dcs_track_the_access_ladder() {
    assert_eq!(
        dc_values(RANGER_LEVEL4_FIXTURE),
        vec![(id(1), 12)],
        "level 4: spell level 1 accessible (a \"0\" per-day entry is still real access), \
         DC 12"
    );
    assert_eq!(
        dc_values(RANGER_LEVEL7_FIXTURE),
        vec![(id(1), 12), (id(2), 13)],
        "level 7: spell levels 1-2 accessible, DCs 12/13"
    );
    assert_eq!(
        dc_values(RANGER_LEVEL10_FIXTURE),
        vec![(id(1), 12), (id(2), 13), (id(3), 14)],
        "level 10: spell levels 1-3 accessible, DCs 12-14"
    );
}

// ----- The formula is pinned as WISDOM-keyed in the record detail -----

#[test]
fn ranger_dc_record_states_the_wisdom_formula() {
    let input = load(RANGER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc = computation
        .explanations
        .iter()
        .find(|e| e.id == id(3))
        .expect("the 3rd-level DC record must exist at level 10");
    assert!(
        dc.detail.contains("10 + ") && dc.detail.contains("Wisdom"),
        "the record must state the PF1 DC formula with the ranger's WISDOM modifier (the \
         family's only Wisdom caster): {}",
        dc.detail
    );
    assert!(
        dc.detail.contains("no saving-throw resolution"),
        "the record must state it grounds the base DC formula only: {}",
        dc.detail
    );
}

// ----- The arithmetic is live: a higher Wisdom raises the DCs -----

#[test]
fn ranger_spell_save_dcs_track_the_wisdom_modifier() {
    let raised = RANGER_LEVEL10_FIXTURE.replace("ability=wisdom:12", "ability=wisdom:18");
    let input = load(&raised);
    let computation = compute_pilot_base_chassis(&input);

    let dc = computation
        .explanations
        .iter()
        .find(|e| e.id == id(3))
        .expect("the 3rd-level DC record must exist at level 10");
    assert_eq!(
        dc.value, 17,
        "DC = 10 + spell level 3 + Wisdom modifier 4 (raised Wisdom 18) = 17 — the formula \
         is live arithmetic over the chosen ability score, not a hardcoded table"
    );
}

// ----- Still claim-blocked overall; exactly the grounded record families -----

#[test]
fn ranger_level10_stays_claim_blocked_overall() {
    let input = load(RANGER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "the level-10 Ranger must stay claim-blocked overall (prepared posture, Wisdom \
         bonus slots, and lineage are still unproven): {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_dc_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(DC_PREFIX)),
        "the Fighter chassis must not surface any ranger spell-save-DC record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Ranger is not promoted -----

#[test]
fn multiclass_ranger_does_not_gain_dc_records() {
    let multiclass = RANGER_LEVEL10_FIXTURE.replace(
        "class_level=class:ranger:10",
        "class_level=class:ranger:10\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(DC_PREFIX)),
        "multiclass Ranger must not gain any spell-save-DC record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Ranger must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the DC grounding -----

#[test]
fn matrix_ranger_row_names_the_dc_grounding() {
    let matrix = seeded_current_truth();
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger hybrid_chassis_and_spell_burden row must exist");

    assert_eq!(ranger.support_state, SupportState::Supported);
    assert_eq!(ranger.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        ranger.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        ranger
            .grounding_ref
            .contains("sd13_ranger_spell_save_dcs"),
        "ranger row must cite the live spell-save-DC proof surface: {}",
        ranger.grounding_ref
    );
    assert!(
        ranger
            .blocker_or_lossiness_note
            .contains("spell_save_dc"),
        "ranger partial note must name the grounded DC records: {}",
        ranger.blocker_or_lossiness_note
    );
}
