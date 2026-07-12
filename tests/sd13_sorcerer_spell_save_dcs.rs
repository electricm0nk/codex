//! SD13-E5 Sorcerer spell-save-DC grounding proof — the first spell-DC
//! slice, building on the grounded access ladder and base per-day counts
//! (`tests/sd13_sorcerer_spell_level_thresholds.rs`,
//! `tests/sd13_sorcerer_spells_per_day_counts.rs`). Both PF1 CRB primary
//! sources (d20pfsrd and legacy.aonprd.com Sorcerer page) were read this
//! cycle before writing any code, and both state the DC rule identically:
//! "The Difficulty Class for a saving throw against a sorcerer's spell is
//! 10 + the spell level + the sorcerer's Charisma modifier." — flat
//! arithmetic over values already grounded on the seam (the Charisma
//! modifier and the access ladder), mirroring the Bard Fascinate DC's
//! 10-plus-modifier idiom.
//!
//! One record per ACCESSIBLE spell level (per the grounded access
//! ladder): `class_chassis.sorcerer.spontaneous.spell_save_dc.spell_level_<N>`
//! with value `10 + N + Charisma modifier`. On the deterministic fixtures
//! (Charisma 17, modifier +3) that is DC 14/15/16/17/18 for spell levels
//! 1-5 as they become accessible. The arithmetic is live, not hardcoded:
//! a lowered fixture Charisma lowers the DCs (asserted below).
//!
//! This grounds only the base DC formula: no saving-throw resolution, no
//! target, no spell selection, and no bloodline-arcana or feat DC
//! modifiers are computed. Spells KNOWN and Charisma bonus slots stay
//! unproven — the `class_spell.sorcerer.spontaneous.unsupported` blocker
//! stays claim-blocking at every supported level (its message is updated
//! to stop deferring the now-grounded base DC arithmetic, keeping it
//! honest; no test pins its wording).
//!
//! It reuses the accepted per-level sorcerer fixtures (no new fixture)
//! and preserves the Fighter negative control and the multiclass
//! negative control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const SORCERER_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt");
const SORCERER_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level4_sd13_deterministic_input.txt");
const SORCERER_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level6_sd13_deterministic_input.txt");
const SORCERER_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level8_sd13_deterministic_input.txt");
const SORCERER_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const DC_PREFIX: &str = "class_chassis.sorcerer.spontaneous.spell_save_dc.";
const SPONTANEOUS_BLOCKER_ID: &str = "class_spell.sorcerer.spontaneous.unsupported";

fn load(fixture: &str) -> CharacterInput {
    let result = load_character_input_fixture(fixture);
    assert!(
        result.diagnostics.is_empty(),
        "fixture should load cleanly: {:?}",
        result.diagnostics
    );
    result
        .character_input
        .expect("valid fixture should produce a character input record")
}

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

// ----- Level 1: DC 14 for the single accessible spell level -----

#[test]
fn sorcerer_level1_first_level_dc_is_fourteen() {
    let input = load(SORCERER_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc = computation
        .explanations
        .iter()
        .find(|e| e.id == id(1))
        .expect("level-1 sorcerer must carry the 1st-level spell-save-DC record");
    assert_eq!(
        dc.value, 14,
        "DC = 10 + spell level 1 + Charisma modifier 3 (fixture Charisma 17) = 14: {}",
        dc.detail
    );
    assert!(
        dc.detail.contains("10 + ") && dc.detail.contains("Charisma"),
        "the record must state the PF1 DC formula with the Charisma modifier: {}",
        dc.detail
    );
    assert_eq!(
        dc_values(SORCERER_LEVEL1_FIXTURE).len(),
        1,
        "only the 1st spell level is accessible at level 1"
    );
}

// ----- The DC ladder tracks the access ladder at every step -----

#[test]
fn sorcerer_spell_save_dcs_track_the_access_ladder() {
    assert_eq!(
        dc_values(SORCERER_LEVEL4_FIXTURE),
        vec![(id(1), 14), (id(2), 15)],
        "level 4: spell levels 1-2 accessible, DCs 14/15"
    );
    assert_eq!(
        dc_values(SORCERER_LEVEL6_FIXTURE),
        vec![(id(1), 14), (id(2), 15), (id(3), 16)],
        "level 6: spell levels 1-3 accessible, DCs 14/15/16"
    );
    assert_eq!(
        dc_values(SORCERER_LEVEL8_FIXTURE),
        vec![(id(1), 14), (id(2), 15), (id(3), 16), (id(4), 17)],
        "level 8: spell levels 1-4 accessible, DCs 14-17"
    );
    assert_eq!(
        dc_values(SORCERER_LEVEL10_FIXTURE),
        vec![
            (id(1), 14),
            (id(2), 15),
            (id(3), 16),
            (id(4), 17),
            (id(5), 18)
        ],
        "level 10: spell levels 1-5 accessible, DCs 14-18"
    );
}

// ----- The arithmetic is live: a lower Charisma lowers the DCs -----

#[test]
fn sorcerer_spell_save_dcs_track_the_charisma_modifier() {
    let lowered = SORCERER_LEVEL10_FIXTURE.replace("ability=charisma:17", "ability=charisma:12");
    let input = load(&lowered);
    let computation = compute_pilot_base_chassis(&input);

    let dc = computation
        .explanations
        .iter()
        .find(|e| e.id == id(5))
        .expect("the 5th-level DC record must exist at level 10");
    assert_eq!(
        dc.value, 16,
        "DC = 10 + spell level 5 + Charisma modifier 1 (lowered Charisma 12) = 16 — the \
         formula is live arithmetic over the chosen ability score, not a hardcoded table"
    );
}

// ----- Base DC formula only; the blocker persists -----

#[test]
fn sorcerer_level10_blocker_stays_and_stops_deferring_the_dc_arithmetic() {
    let input = load(SORCERER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc = computation
        .explanations
        .iter()
        .find(|e| e.id == id(5))
        .expect("the 5th-level DC record must exist at level 10");
    assert!(
        dc.detail.contains("no saving-throw resolution"),
        "the record must state it grounds the base DC formula only: {}",
        dc.detail
    );

    let blocker = computation
        .diagnostics
        .iter()
        .find(|d| d.id == SPONTANEOUS_BLOCKER_ID)
        .expect("the spontaneous blocker must still exist at level 10");
    assert!(blocker.claim_blocking, "the blocker must stay claim-blocking");
    assert!(
        !blocker.message.contains("spell save DCs are out of scope"),
        "the blocker must stop deferring the now-grounded base DC arithmetic: {}",
        blocker.message
    );
    assert!(
        blocker.message.contains("spells known"),
        "the blocker must still name the genuinely-unproven remainder: {}",
        blocker.message
    );
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_sorcerer_dc_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(DC_PREFIX)),
        "the Fighter chassis must not surface any sorcerer spell-save-DC record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Sorcerer is not promoted -----

#[test]
fn multiclass_sorcerer_does_not_gain_dc_records() {
    let multiclass = SORCERER_LEVEL10_FIXTURE.replace(
        "class_level=class:sorcerer:10",
        "class_level=class:sorcerer:10\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(DC_PREFIX)),
        "multiclass Sorcerer must not gain any spell-save-DC record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Sorcerer must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the DC grounding -----

#[test]
fn matrix_sorcerer_row_names_the_dc_grounding() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let sorcerer = matrix
        .row("class.sorcerer.progression_and_spell_burden")
        .expect("sorcerer progression_and_spell_burden row must exist");

    assert_eq!(sorcerer.support_state, SupportState::Partial);
    assert_eq!(sorcerer.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        sorcerer.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        sorcerer
            .grounding_ref
            .contains("sd13_sorcerer_spell_save_dcs"),
        "sorcerer row must cite the live spell-save-DC proof surface: {}",
        sorcerer.grounding_ref
    );
    assert!(
        sorcerer
            .blocker_or_lossiness_note
            .contains("spell_save_dc"),
        "sorcerer partial note must name the grounded DC records: {}",
        sorcerer.blocker_or_lossiness_note
    );
}
