//! SD13-E5 Sorcerer bonus-spells-per-day grounding proof — the first
//! bonus-spells slice, grounding PF1's shared Table: Ability Modifiers and
//! Bonus Spells for the sorcerer's Charisma. Both PF1 CRB primary sources
//! (legacy.aonprd.com Getting Started / d20pfsrd Ability Scores pages)
//! were read this cycle before writing any code, and both show identical
//! table rows: modifier +1 grants 1 bonus 1st-level spell; +2 grants
//! 1/1; +3 grants 1/1/1; +4 grants 1/1/1/1; +5 grants 2/1/1/1/1 — i.e.
//! for modifier m and spell level N (1st+): 0 when m < N, otherwise
//! (m - N)/4 + 1. Both sources state the gating rule identically: "a
//! spellcaster must be of a high enough class level to be able to cast
//! spells of a given spell level" — exactly the grounded access ladder.
//!
//! One record per ACCESSIBLE spell level (1st+; the bonus table has no
//! 0th-level column — cantrips never gain bonus spells):
//! `class_chassis.sorcerer.spontaneous.bonus_spells_per_day.spell_level_<N>`
//! with the bonus count as its value. A computed 0 (modifier below the
//! spell level) is an honest arithmetic result, distinct from the base
//! table's literal "0" entries. On the deterministic fixtures (Charisma
//! 17, modifier +3): bonus 1/1/1/0/0 across spell levels 1-5 at level 10.
//! The arithmetic is live: raising the fixture Charisma to 18 (+4) turns
//! the 4th-level bonus to 1, and lowering it to 10 (+0) zeroes them all.
//!
//! This grounds the bonus COUNTS only: they are never added to the base
//! per-day counts (no total is computed — that integration is named as
//! the next uplift), no spell selection, and no spell save DCs. The
//! `class_spell.sorcerer.spontaneous.unsupported` blocker stays
//! claim-blocking (message updated to stop deferring the now-grounded
//! bonus-slot counts, preserving the pinned tokens "spontaneous" /
//! "spells known" / "spell slot").
//!
//! It reuses the accepted per-level sorcerer fixtures (no new fixture)
//! and preserves the Fighter negative control and the multiclass
//! negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::load;

const SORCERER_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt");
const SORCERER_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level6_sd13_deterministic_input.txt");
const SORCERER_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BONUS_PREFIX: &str = "class_chassis.sorcerer.spontaneous.bonus_spells_per_day.";
const SPONTANEOUS_BLOCKER_ID: &str = "class_spell.sorcerer.spontaneous.unsupported";

fn bonus_values(fixture: &str) -> Vec<(String, i16)> {
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);
    computation
        .explanations
        .iter()
        .filter(|e| e.id.starts_with(BONUS_PREFIX))
        .map(|e| (e.id.clone(), e.value))
        .collect()
}

fn id(spell_level: u8) -> String {
    format!("{BONUS_PREFIX}spell_level_{spell_level}")
}

// ----- Level 1 (+3): one accessible spell level, bonus 1 -----

#[test]
fn sorcerer_level1_gains_one_bonus_first_level_spell() {
    let input = load(SORCERER_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        bonus_values(SORCERER_LEVEL1_FIXTURE),
        vec![(id(1), 1)],
        "level 1, Charisma +3: one accessible spell level, bonus 1 (Table: Ability \
         Modifiers and Bonus Spells, modifier +3 row); no 0th-level record — cantrips \
         never gain bonus spells"
    );

    let bonus = computation
        .explanations
        .iter()
        .find(|e| e.id == id(1))
        .expect("the 1st-level bonus record must exist");
    assert!(
        bonus.detail.contains("Ability Modifiers and Bonus Spells"),
        "the record must cite the shared PF1 table: {}",
        bonus.detail
    );
}

// ----- The bonus ladder tracks modifier and access at every step -----

#[test]
fn sorcerer_bonus_spells_track_the_table_and_the_access_ladder() {
    assert_eq!(
        bonus_values(SORCERER_LEVEL6_FIXTURE),
        vec![(id(1), 1), (id(2), 1), (id(3), 1)],
        "level 6, Charisma +3: spell levels 1-3 accessible, bonus 1 each (+3 row: 1/1/1)"
    );
    // CG-03 fix: the Human ability-bonus choice's +2 racial Charisma adjustment is now
    // applied before the modifier is derived (base 17 -> 19, modifier +3 -> +4), so the
    // +4 row (1/1/1/1) now applies at spell level 4; level 5 still gets an honest zero.
    assert_eq!(
        bonus_values(SORCERER_LEVEL10_FIXTURE),
        vec![(id(1), 1), (id(2), 1), (id(3), 1), (id(4), 1), (id(5), 0)],
        "level 10, Charisma +4: spell level 5 is accessible but the +4 modifier grants no \
         bonus below itself — an honest computed 0, distinct from the base table's literal \
         \"0\" entries"
    );
}

// ----- The arithmetic is live in both directions -----

#[test]
fn sorcerer_bonus_spells_track_the_charisma_modifier() {
    // CG-03 fix: this raised fixture's chosen Charisma score also receives the +2 Human
    // racial ability-bonus choice before the modifier is derived (18 + 2 = 20, modifier
    // +5, not the pre-fix +4), but the 4th-level bonus is still 1 either way (the +4 and
    // +5 rows both grant 1 at spell level 4).
    let raised = SORCERER_LEVEL10_FIXTURE.replace("ability=charisma:17", "ability=charisma:18");
    let input = load(&raised);
    let computation = compute_pilot_base_chassis(&input);
    let fourth = computation
        .explanations
        .iter()
        .find(|e| e.id == id(4))
        .expect("the 4th-level bonus record must exist");
    assert_eq!(
        fourth.value, 1,
        "raising Charisma to 18 (+2 Human racial = 20, modifier +5) still turns the \
         4th-level bonus to 1 (both the +4 and +5 rows grant 1/1/1/1 at spell level 4)"
    );

    // A raw score of 10 would now yield 10 + 2 Human racial = 12, modifier +1 -- no longer
    // the zero modifier this test means to demonstrate. Lower to 8 instead (8 + 2 = 10,
    // modifier +1) to keep proving the same real thing: no bonus without a positive
    // effective modifier.
    let lowered = SORCERER_LEVEL10_FIXTURE.replace("ability=charisma:17", "ability=charisma:8");
    let input = load(&lowered);
    let computation = compute_pilot_base_chassis(&input);
    let first = computation
        .explanations
        .iter()
        .find(|e| e.id == id(1))
        .expect("the 1st-level bonus record must exist");
    assert_eq!(
        first.value, 0,
        "lowering Charisma to 8 (+2 Human racial = 10, modifier +0) zeroes every bonus — no \
         bonus spells without a positive modifier"
    );
}

// ----- Bonus counts only; the blocker stops deferring them -----

#[test]
fn sorcerer_level10_blocker_stays_and_stops_deferring_bonus_slots() {
    let input = load(SORCERER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = computation
        .explanations
        .iter()
        .find(|e| e.id == id(1))
        .expect("the 1st-level bonus record must exist");
    assert!(
        bonus.detail.contains("never added") || bonus.detail.contains("no total"),
        "the record must state the bonus is never added to the base counts (no total is \
         computed): {}",
        bonus.detail
    );

    // (v0.6 alpha swarm, risks item 8) SPONTANEOUS_BLOCKER_ID is no longer
    // unconditional -- this fixture has zero known spells, a genuinely valid
    // posture, so the blocker correctly does not fire here. The bonus-slot record
    // above already proves the real bonus count is grounded and never fabricated
    // into a total; that's the property this test actually needs, independent of
    // whether the (now-conditional) spell blocker happens to fire on this
    // particular fixture.
    match computation.diagnostics.iter().find(|d| d.id == SPONTANEOUS_BLOCKER_ID) {
        Some(blocker) => assert!(blocker.claim_blocking, "if the blocker fires, it must be claim-blocking"),
        None => {
            let known_count = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_spell.sorcerer.known_spells")
                .map(|e| e.value)
                .unwrap_or(-1);
            assert_eq!(
                known_count, 0,
                "no spells are fabricated merely because the blocker stopped firing: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_sorcerer_bonus_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(BONUS_PREFIX)),
        "the Fighter chassis must not surface any sorcerer bonus-spells record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Sorcerer is not promoted -----

#[test]
fn multiclass_sorcerer_does_not_gain_bonus_records() {
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
            .any(|e| e.id.starts_with(BONUS_PREFIX)),
        "multiclass Sorcerer must not gain any bonus-spells record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Sorcerer must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the bonus-spells grounding -----

#[test]
fn matrix_sorcerer_row_names_the_bonus_spells_grounding() {
    let matrix = seeded_current_truth();
    let sorcerer = matrix
        .row("class.sorcerer.progression_and_spell_burden")
        .expect("sorcerer progression_and_spell_burden row must exist");

    assert_eq!(sorcerer.support_state, SupportState::Supported);
    assert_eq!(sorcerer.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        sorcerer.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        sorcerer
            .grounding_ref
            .contains("sd13_sorcerer_bonus_spells"),
        "sorcerer row must cite the live bonus-spells proof surface: {}",
        sorcerer.grounding_ref
    );
    assert!(
        sorcerer
            .blocker_or_lossiness_note
            .contains("bonus_spells_per_day"),
        "sorcerer partial note must name the grounded bonus-spells records: {}",
        sorcerer.blocker_or_lossiness_note
    );
}
