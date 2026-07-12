//! SD13-E5 Paladin bonus-spells-per-day grounding proof — the third
//! bonus-spells slice, mirroring the Sorcerer/Bard bonus slices against
//! PF1's shared Table: Ability Modifiers and Bonus Spells (verified this
//! session against both primary sources' ability-scores pages, quoted
//! identically: for modifier m and spell level N (1st+), 0 when m < N,
//! otherwise (m - N)/4 + 1). The paladin-specific rule text was verified
//! on both class pages this session too: "she receives bonus spells per
//! day if she has a high Charisma score", and the shared table's gating
//! rule ("a spellcaster must be of a high enough class level to be able
//! to cast spells of a given spell level") maps onto the grounded paladin
//! access ladder.
//!
//! One record per ACCESSIBLE spell level:
//! `class_chassis.paladin.partial_caster.bonus_spells_per_day.spell_level_<N>`.
//! On the deterministic fixtures (Charisma 14, modifier +2): bonus 1/1/0
//! across spell levels 1-3 at level 10; below the level-4 spell gate no
//! bonus record exists at all. THE KEY INTERPLAY, pinned below: at level
//! 4 the base per-day count is a literal "0" entry while the bonus is 1 —
//! this is exactly the bonus-spells-only access the PF1 "0 spells per
//! day" rule describes, now visible as two grounded records side by side.
//! Live arithmetic: lowering the fixture Charisma to 10 (+0) zeroes every
//! bonus.
//!
//! Bonus COUNTS only: never added to the base per-day counts (the
//! base+bonus TOTAL integration stays deferred), no spell selection, no
//! DCs. The `class_spell.paladin.partial_caster.unsupported` blocker
//! stays claim-blocking (message updated to defer the TOTAL integration
//! instead of the now-grounded bonus counts, preserving the pinned tokens
//! "partial" and "level - 3").
//!
//! It reuses the accepted per-level paladin fixtures (no new fixture) and
//! preserves the Fighter negative control and the multiclass negative
//! control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const PALADIN_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level3_sd13_deterministic_input.txt");
const PALADIN_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level4_sd13_deterministic_input.txt");
const PALADIN_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BONUS_PREFIX: &str = "class_chassis.paladin.partial_caster.bonus_spells_per_day.";
const BASE_FIRST_ID: &str =
    "class_chassis.paladin.partial_caster.base_spells_per_day.spell_level_1";
const PARTIAL_CASTER_BLOCKER_ID: &str = "class_spell.paladin.partial_caster.unsupported";

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

// ----- Below the level-4 spell gate: no bonus records at all -----

#[test]
fn paladin_level3_has_no_bonus_records() {
    assert!(
        bonus_values(PALADIN_LEVEL3_FIXTURE).is_empty(),
        "a level-3 paladin has no accessible spell level, so no bonus-spells record may \
         exist"
    );
}

// ----- Level 4: the "0"-base / 1-bonus interplay, side by side -----

#[test]
fn paladin_level4_zero_base_and_one_bonus_are_the_bonus_only_access() {
    let input = load(PALADIN_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base = computation
        .explanations
        .iter()
        .find(|e| e.id == BASE_FIRST_ID)
        .expect("the grounded base per-day record must exist at level 4");
    assert_eq!(base.value, 0, "the level-4 base count is the literal \"0\" table entry");

    let bonus = computation
        .explanations
        .iter()
        .find(|e| e.id == id(1))
        .expect("the 1st-level bonus record must exist at level 4");
    assert_eq!(
        bonus.value, 1,
        "Charisma +2 grants 1 bonus 1st-level spell — together with the \"0\" base entry \
         this is exactly the bonus-spells-only access the PF1 rule describes: {}",
        bonus.detail
    );
    assert!(
        bonus.detail.contains("Ability Modifiers and Bonus Spells"),
        "the record must cite the shared PF1 table: {}",
        bonus.detail
    );
}

// ----- The bonus ladder tracks modifier and access at every step -----

#[test]
fn paladin_bonus_spells_track_the_table_and_the_access_ladder() {
    assert_eq!(
        bonus_values(PALADIN_LEVEL4_FIXTURE),
        vec![(id(1), 1)],
        "level 4, Charisma +2: one accessible spell level, bonus 1"
    );
    assert_eq!(
        bonus_values(PALADIN_LEVEL10_FIXTURE),
        vec![(id(1), 1), (id(2), 1), (id(3), 0)],
        "level 10, Charisma +2: bonus 1/1/0 — the +2 row grants nothing at 3rd, an honest \
         computed 0"
    );
}

// ----- The arithmetic is live: no bonus without a positive modifier -----

#[test]
fn paladin_bonus_spells_track_the_charisma_modifier() {
    let lowered = PALADIN_LEVEL10_FIXTURE.replace("ability=charisma:14", "ability=charisma:10");
    let input = load(&lowered);
    let computation = compute_pilot_base_chassis(&input);
    let first = computation
        .explanations
        .iter()
        .find(|e| e.id == id(1))
        .expect("the 1st-level bonus record must exist");
    assert_eq!(
        first.value, 0,
        "lowering Charisma to 10 (+0) zeroes every bonus — live arithmetic over the chosen \
         ability score"
    );
}

// ----- Bonus counts only; the blocker defers the TOTAL integration -----

#[test]
fn paladin_level10_blocker_stays_and_defers_the_total_integration() {
    let input = load(PALADIN_LEVEL10_FIXTURE);
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
        .find(|d| d.id == PARTIAL_CASTER_BLOCKER_ID)
        .expect("the partial-caster blocker must still exist at level 10");
    assert!(blocker.claim_blocking, "the blocker must stay claim-blocking");
    assert!(
        blocker.message.contains("partial") && blocker.message.contains("level - 3"),
        "the blocker must keep the tokens its level-1 sibling pins: {}",
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
fn fighter_does_not_gain_paladin_bonus_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(BONUS_PREFIX)),
        "the Fighter chassis must not surface any paladin bonus-spells record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Paladin is not promoted -----

#[test]
fn multiclass_paladin_does_not_gain_bonus_records() {
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
            .any(|e| e.id.starts_with(BONUS_PREFIX)),
        "multiclass Paladin must not gain any bonus-spells record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Paladin must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the bonus-spells grounding -----

#[test]
fn matrix_paladin_row_names_the_bonus_spells_grounding() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin hybrid_chassis_and_spell_burden row must exist");

    assert_eq!(paladin.support_state, SupportState::Partial);
    assert_eq!(paladin.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        paladin.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        paladin
            .grounding_ref
            .contains("sd13_paladin_bonus_spells"),
        "paladin row must cite the live bonus-spells proof surface: {}",
        paladin.grounding_ref
    );
    assert!(
        paladin
            .blocker_or_lossiness_note
            .contains("bonus_spells_per_day"),
        "paladin partial note must name the grounded bonus-spells records: {}",
        paladin.blocker_or_lossiness_note
    );
}
