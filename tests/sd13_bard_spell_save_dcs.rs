//! SD13-E5 Bard spell-save-DC grounding proof — the second spell-DC slice,
//! mirroring the Sorcerer DC slice (`tests/sd13_sorcerer_spell_save_dcs.rs`)
//! record-for-record on top of the grounded bard access ladder and base
//! per-day counts. Both PF1 CRB primary sources (d20pfsrd and
//! legacy.aonprd.com Bard page) were read this cycle before writing any
//! code, and both state the DC rule identically: "The Difficulty Class
//! for a saving throw against a bard's spell is 10 + the spell level +
//! the bard's Charisma modifier." — flat arithmetic over values already
//! grounded on the seam (the Charisma modifier, already used by the
//! grounded Bardic Performance rounds-per-day and Fascinate DC records,
//! and the access ladder).
//!
//! One record per ACCESSIBLE spell level (per the grounded access
//! ladder): `class_chassis.bard.spontaneous.spell_save_dc.spell_level_<N>`
//! with value `10 + N + Charisma modifier`. On the deterministic fixtures
//! (Charisma 15, modifier +2) that is DC 13/14/15/16 for spell levels 1-4
//! as they become accessible. The arithmetic is live, not hardcoded: a
//! lowered fixture Charisma lowers the DCs (asserted below). This is a
//! DIFFERENT formula family from the already-grounded Fascinate DC
//! (10 + 1/2 bard level + Charisma modifier — a performance DC keyed to
//! bard level, not spell level); both coexist as separate records.
//!
//! This grounds only the base DC formula: no saving-throw resolution, no
//! target, no spell selection, and no feat DC modifiers are computed.
//! Spells KNOWN and Charisma bonus slots stay unproven — the
//! `class_spell.bard.spontaneous_known_and_per_day.unsupported` blocker
//! stays claim-blocking at every supported level (its message is updated
//! to stop deferring the now-grounded base DC arithmetic, keeping it
//! honest; no test pins its wording).
//!
//! Sibling allow-list extensions (the new records fire at every
//! supported level): the level-1 baseline's spell-tag allow-list gains
//! the DC family by prefix, and the level-4/5/6/7/8 `known_bard_ids`
//! controls gain the family's exact ids — the established pattern.
//!
//! It reuses the accepted per-level bard fixtures (no new fixture) and
//! preserves the Fighter negative control and the multiclass negative
//! control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const BARD_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level1_sd13_deterministic_input.txt");
const BARD_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level4_sd13_deterministic_input.txt");
const BARD_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level7_sd13_deterministic_input.txt");
const BARD_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const DC_PREFIX: &str = "class_chassis.bard.spontaneous.spell_save_dc.";
const SPONTANEOUS_BLOCKER_ID: &str = "class_spell.bard.spontaneous_known_and_per_day.unsupported";

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

// ----- Level 1: DC 13 for the single accessible spell level -----

#[test]
fn bard_level1_first_level_dc_is_thirteen() {
    let input = load(BARD_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc = computation
        .explanations
        .iter()
        .find(|e| e.id == id(1))
        .expect("level-1 bard must carry the 1st-level spell-save-DC record");
    assert_eq!(
        dc.value, 13,
        "DC = 10 + spell level 1 + Charisma modifier 2 (fixture Charisma 15) = 13: {}",
        dc.detail
    );
    assert!(
        dc.detail.contains("10 + ") && dc.detail.contains("Charisma"),
        "the record must state the PF1 DC formula with the Charisma modifier: {}",
        dc.detail
    );
    assert_eq!(
        dc_values(BARD_LEVEL1_FIXTURE).len(),
        1,
        "only the 1st spell level is accessible at level 1"
    );
}

// ----- The DC ladder tracks the access ladder at every step -----

#[test]
fn bard_spell_save_dcs_track_the_access_ladder() {
    assert_eq!(
        dc_values(BARD_LEVEL4_FIXTURE),
        vec![(id(1), 13), (id(2), 14)],
        "level 4: spell levels 1-2 accessible, DCs 13/14"
    );
    assert_eq!(
        dc_values(BARD_LEVEL7_FIXTURE),
        vec![(id(1), 13), (id(2), 14), (id(3), 15)],
        "level 7: spell levels 1-3 accessible, DCs 13/14/15"
    );
    assert_eq!(
        dc_values(BARD_LEVEL10_FIXTURE),
        vec![(id(1), 13), (id(2), 14), (id(3), 15), (id(4), 16)],
        "level 10: spell levels 1-4 accessible, DCs 13-16"
    );
}

// ----- The arithmetic is live: a lower Charisma lowers the DCs -----

#[test]
fn bard_spell_save_dcs_track_the_charisma_modifier() {
    let lowered = BARD_LEVEL10_FIXTURE.replace("ability=charisma:15", "ability=charisma:10");
    let input = load(&lowered);
    let computation = compute_pilot_base_chassis(&input);

    let dc = computation
        .explanations
        .iter()
        .find(|e| e.id == id(4))
        .expect("the 4th-level DC record must exist at level 10");
    assert_eq!(
        dc.value, 14,
        "DC = 10 + spell level 4 + Charisma modifier 0 (lowered Charisma 10) = 14 — the \
         formula is live arithmetic over the chosen ability score, not a hardcoded table"
    );
}

// ----- Base DC formula only; the blocker persists and stops deferring DCs -----

#[test]
fn bard_level10_blocker_stays_and_stops_deferring_the_dc_arithmetic() {
    let input = load(BARD_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc = computation
        .explanations
        .iter()
        .find(|e| e.id == id(4))
        .expect("the 4th-level DC record must exist at level 10");
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
        !blocker.message.contains("no spell DCs"),
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
fn fighter_does_not_gain_bard_dc_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(DC_PREFIX)),
        "the Fighter chassis must not surface any bard spell-save-DC record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Bard is not promoted -----

#[test]
fn multiclass_bard_does_not_gain_dc_records() {
    let multiclass = BARD_LEVEL10_FIXTURE.replace(
        "class_level=class:bard:10",
        "class_level=class:bard:10\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(DC_PREFIX)),
        "multiclass Bard must not gain any spell-save-DC record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Bard must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the DC grounding -----

#[test]
fn matrix_bard_row_names_the_dc_grounding() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard progression_and_spell_burden row must exist");

    assert_eq!(bard.support_state, SupportState::Partial);
    assert_eq!(bard.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        bard.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        bard.grounding_ref.contains("sd13_bard_spell_save_dcs"),
        "bard row must cite the live spell-save-DC proof surface: {}",
        bard.grounding_ref
    );
    assert!(
        bard.blocker_or_lossiness_note.contains("spell_save_dc"),
        "bard partial note must name the grounded DC records: {}",
        bard.blocker_or_lossiness_note
    );
}
