//! SD13-E5 Bard bonus-spells-per-day grounding proof — the second
//! bonus-spells slice, mirroring the Sorcerer bonus slice
//! (`tests/sd13_sorcerer_bonus_spells.rs`) record-for-record against
//! PF1's shared Table: Ability Modifiers and Bonus Spells, verified this
//! session against both primary sources' ability-scores pages
//! (legacy.aonprd.com Getting Started / d20pfsrd Ability Scores, quoted
//! identically): modifier +1 grants 1 bonus 1st-level spell; +2 grants
//! 1/1; +3 grants 1/1/1; +4 grants 1/1/1/1; +5 grants 2/1/1/1/1 — for
//! modifier m and spell level N (1st+): 0 when m < N, otherwise
//! (m - N)/4 + 1, gated by "a spellcaster must be of a high enough class
//! level to be able to cast spells of a given spell level" — the
//! grounded bard access ladder.
//!
//! One record per ACCESSIBLE spell level:
//! `class_chassis.bard.spontaneous.bonus_spells_per_day.spell_level_<N>`.
//! On the deterministic fixtures (Charisma 15, modifier +2): bonus
//! 1/1/0/0 across spell levels 1-4 at level 10 — the +2 row grants
//! nothing at spell levels 3-4, honest computed zeros. Live arithmetic:
//! raising the fixture Charisma to 16 (+3) turns the 3rd-level bonus to
//! 1. Cantrips never gain bonus spells (no 0th-level column).
//!
//! Bonus COUNTS only: never added to the base per-day counts (the
//! base+bonus TOTAL integration stays deferred), no spell selection, no
//! DCs. The `class_spell.bard.spontaneous_known_and_per_day.unsupported`
//! blocker stays claim-blocking (message updated to defer the TOTAL
//! integration instead of the now-grounded bonus counts, preserving the
//! pinned tokens "spontaneous" / "spells known" / "spells per day").
//!
//! Sibling allow-lists: the established pattern (level-1 baseline by
//! prefix; level-4/5/6/7/8 `known_bard_ids` by exact ids).
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
const BARD_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level7_sd13_deterministic_input.txt");
const BARD_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BONUS_PREFIX: &str = "class_chassis.bard.spontaneous.bonus_spells_per_day.";
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

// ----- Level 1 (+2): one accessible spell level, bonus 1 -----

#[test]
fn bard_level1_gains_one_bonus_first_level_spell() {
    let input = load(BARD_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        bonus_values(BARD_LEVEL1_FIXTURE),
        vec![(id(1), 1)],
        "level 1, Charisma +2: one accessible spell level, bonus 1 (+2 row: 1/1); no \
         0th-level record — cantrips never gain bonus spells"
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
fn bard_bonus_spells_track_the_table_and_the_access_ladder() {
    assert_eq!(
        bonus_values(BARD_LEVEL7_FIXTURE),
        vec![(id(1), 1), (id(2), 1), (id(3), 0)],
        "level 7, Charisma +2: spell levels 1-3 accessible, +2 row grants 1/1 and an \
         honest computed 0 at 3rd"
    );
    assert_eq!(
        bonus_values(BARD_LEVEL10_FIXTURE),
        vec![(id(1), 1), (id(2), 1), (id(3), 0), (id(4), 0)],
        "level 10, Charisma +2: bonus 1/1/0/0 across the four accessible spell levels"
    );
}

// ----- The arithmetic is live -----

#[test]
fn bard_bonus_spells_track_the_charisma_modifier() {
    let raised = BARD_LEVEL10_FIXTURE.replace("ability=charisma:15", "ability=charisma:16");
    let input = load(&raised);
    let computation = compute_pilot_base_chassis(&input);
    let third = computation
        .explanations
        .iter()
        .find(|e| e.id == id(3))
        .expect("the 3rd-level bonus record must exist");
    assert_eq!(
        third.value, 1,
        "raising Charisma to 16 (+3) turns the 3rd-level bonus to 1 (+3 row: 1/1/1)"
    );
}

// ----- Bonus counts only; the blocker defers the TOTAL integration -----

#[test]
fn bard_level10_blocker_stays_and_defers_the_total_integration() {
    let input = load(BARD_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = computation
        .explanations
        .iter()
        .find(|e| e.id == id(1))
        .expect("the 1st-level bonus record must exist");
    assert!(
        bonus.detail.contains("never added") || bonus.detail.contains("no total"),
        "the record must state the bonus is never added to the base counts: {}",
        bonus.detail
    );

    let blocker = computation
        .diagnostics
        .iter()
        .find(|d| d.id == SPONTANEOUS_BLOCKER_ID)
        .expect("the spontaneous blocker must still exist at level 10");
    assert!(blocker.claim_blocking, "the blocker must stay claim-blocking");
    assert!(
        blocker.message.contains("spontaneous")
            && blocker.message.contains("spells known")
            && blocker.message.contains("spells per day"),
        "the blocker must keep its pinned tokens: {}",
        blocker.message
    );
    assert!(
        blocker.message.contains("TOTAL") || blocker.message.contains("total"),
        "the blocker must defer the base+bonus TOTAL integration now that the bonus counts \
         are grounded: {}",
        blocker.message
    );
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_bard_bonus_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(BONUS_PREFIX)),
        "the Fighter chassis must not surface any bard bonus-spells record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Bard is not promoted -----

#[test]
fn multiclass_bard_does_not_gain_bonus_records() {
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
            .any(|e| e.id.starts_with(BONUS_PREFIX)),
        "multiclass Bard must not gain any bonus-spells record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Bard must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the bonus-spells grounding -----

#[test]
fn matrix_bard_row_names_the_bonus_spells_grounding() {
    let matrix = seeded_sd13_e1_f1_current_truth();
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
        bard.grounding_ref.contains("sd13_bard_bonus_spells"),
        "bard row must cite the live bonus-spells proof surface: {}",
        bard.grounding_ref
    );
    assert!(
        bard.blocker_or_lossiness_note
            .contains("bonus_spells_per_day"),
        "bard partial note must name the grounded bonus-spells records: {}",
        bard.blocker_or_lossiness_note
    );
}
