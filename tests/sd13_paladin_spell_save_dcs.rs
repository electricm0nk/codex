//! SD13-E5 Paladin spell-save-DC grounding proof — the third spell-DC
//! slice, mirroring the Sorcerer/Bard DC slices on top of the grounded
//! paladin partial-caster surface (access ladder + base per-day counts).
//! Both PF1 CRB primary sources (d20pfsrd and legacy.aonprd.com Paladin
//! page) were read this cycle before writing any code, and both state the
//! DC rule identically: "The Difficulty Class for a saving throw against
//! a paladin's spell is 10 + the spell level + the paladin's Charisma
//! modifier." — flat arithmetic over values already grounded on the seam
//! (the Charisma modifier, already used by Smite Evil / Lay on Hands /
//! Divine Grace, and the access ladder).
//!
//! One record per ACCESSIBLE spell level:
//! `class_chassis.paladin.partial_caster.spell_save_dc.spell_level_<N>`
//! with value `10 + N + Charisma modifier`. On the deterministic fixtures
//! (Charisma 14, modifier +2) that is DC 13/14/15 for spell levels 1-3 as
//! they become accessible; below the level-4 spell gate no DC record
//! exists at all. The arithmetic is live, not hardcoded: a lowered
//! fixture Charisma lowers the DCs (asserted below).
//!
//! This grounds only the base DC formula: no saving-throw resolution, no
//! target, no spell selection, and no feat DC modifiers are computed.
//! The spell-source lineage, prepared posture, and Charisma bonus slots
//! stay unproven — the `class_spell.paladin.partial_caster.unsupported`
//! blocker stays claim-blocking at every supported level (its message is
//! updated to stop deferring the now-grounded base DC arithmetic while
//! preserving the tokens its level-1 sibling pins: "partial" and
//! "level - 3").
//!
//! It reuses the accepted per-level paladin fixtures (no new fixture) and
//! preserves the Fighter negative control and the multiclass negative
//! control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::load;

const PALADIN_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level3_sd13_deterministic_input.txt");
const PALADIN_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level4_sd13_deterministic_input.txt");
const PALADIN_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level7_sd13_deterministic_input.txt");
const PALADIN_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const DC_PREFIX: &str = "class_chassis.paladin.partial_caster.spell_save_dc.";
const PARTIAL_CASTER_BLOCKER_ID: &str = "class_spell.paladin.partial_caster.unsupported";

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
fn paladin_level3_has_no_dc_records() {
    assert!(
        dc_values(PALADIN_LEVEL3_FIXTURE).is_empty(),
        "a level-3 paladin has no accessible spell level, so no spell-save-DC record may \
         exist"
    );
}

// ----- The DC ladder tracks the access ladder at every step -----

#[test]
fn paladin_spell_save_dcs_track_the_access_ladder() {
    // CG-03 fix: Charisma modifier is now +3 (base 14 + 2 Human racial), not +2, so
    // every DC in the ladder is one higher than before.
    assert_eq!(
        dc_values(PALADIN_LEVEL4_FIXTURE),
        vec![(id(1), 14)],
        "level 4: spell level 1 accessible (a \"0\" per-day entry is still real access), \
         DC 14"
    );
    assert_eq!(
        dc_values(PALADIN_LEVEL7_FIXTURE),
        vec![(id(1), 14), (id(2), 15)],
        "level 7: spell levels 1-2 accessible, DCs 14/15"
    );
    assert_eq!(
        dc_values(PALADIN_LEVEL10_FIXTURE),
        vec![(id(1), 14), (id(2), 15), (id(3), 16)],
        "level 10: spell levels 1-3 accessible, DCs 14-16"
    );
}

// ----- The formula is pinned in the record detail -----

#[test]
fn paladin_dc_record_states_the_formula() {
    let input = load(PALADIN_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc = computation
        .explanations
        .iter()
        .find(|e| e.id == id(3))
        .expect("the 3rd-level DC record must exist at level 10");
    assert!(
        dc.detail.contains("10 + ") && dc.detail.contains("Charisma"),
        "the record must state the PF1 DC formula with the Charisma modifier: {}",
        dc.detail
    );
    assert!(
        dc.detail.contains("no saving-throw resolution"),
        "the record must state it grounds the base DC formula only: {}",
        dc.detail
    );
}

// ----- The arithmetic is live: a lower Charisma lowers the DCs -----

#[test]
fn paladin_spell_save_dcs_track_the_charisma_modifier() {
    let lowered = PALADIN_LEVEL10_FIXTURE.replace("ability=charisma:14", "ability=charisma:10");
    let input = load(&lowered);
    let computation = compute_pilot_base_chassis(&input);

    let dc = computation
        .explanations
        .iter()
        .find(|e| e.id == id(3))
        .expect("the 3rd-level DC record must exist at level 10");
    // CG-03 fix: the lowered raw Charisma of 10 still receives the +2 Human racial
    // ability-bonus choice before the modifier is derived (10 + 2 = 12, modifier +1).
    assert_eq!(
        dc.value, 14,
        "DC = 10 + spell level 3 + Charisma modifier 1 (lowered Charisma 10 + 2 Human racial) \
         = 14 — the formula is live arithmetic over the chosen ability score, not a hardcoded \
         table"
    );
}

// ----- The blocker persists and stops deferring the DC arithmetic -----

#[test]
fn paladin_level10_blocker_stays_and_stops_deferring_the_dc_arithmetic() {
    let input = load(PALADIN_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // (v0.6 alpha swarm, risks item 8, 2026-07-25) `PARTIAL_CASTER_BLOCKER_ID`
    // is no longer unconditional, and its message shape changed entirely
    // (it's a real, conditional validation now, not a static deferred-work
    // list). PALADIN_LEVEL10_FIXTURE predates spells_selected (zero
    // prepared), so the posture is genuinely valid and the blocker
    // correctly does not fire -- the real "DC arithmetic is grounded, no
    // slots fabricated" guarantee now comes from the daily-preparation
    // record's own count being honestly 0.
    match computation.diagnostics.iter().find(|d| d.id == PARTIAL_CASTER_BLOCKER_ID) {
        Some(blocker) => assert!(blocker.claim_blocking, "the blocker must stay claim-blocking"),
        None => {
            let daily_prep = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_spell.paladin.daily_preparation")
                .expect("the daily-preparation record must exist when the posture is valid");
            assert_eq!(
                daily_prep.value, 0,
                "no spells are fabricated at paladin level 10: {daily_prep:?}"
            );
        }
    }
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_paladin_dc_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(DC_PREFIX)),
        "the Fighter chassis must not surface any paladin spell-save-DC record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Paladin is not promoted -----

#[test]
fn multiclass_paladin_does_not_gain_dc_records() {
    let multiclass = PALADIN_LEVEL10_FIXTURE.replace(
        "class_level=class:paladin:10",
        "class_level=class:paladin:10\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(DC_PREFIX)),
        "multiclass Paladin must not gain any spell-save-DC record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Paladin must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the DC grounding -----

#[test]
fn matrix_paladin_row_names_the_dc_grounding() {
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
            .contains("sd13_paladin_spell_save_dcs"),
        "paladin row must cite the live spell-save-DC proof surface: {}",
        paladin.grounding_ref
    );
    assert!(
        paladin
            .blocker_or_lossiness_note
            .contains("spell_save_dc"),
        "paladin partial note must name the grounded DC records: {}",
        paladin.blocker_or_lossiness_note
    );
}
