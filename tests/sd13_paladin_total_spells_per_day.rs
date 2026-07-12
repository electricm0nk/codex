//! SD13-E5 Paladin total-spells-per-day grounding proof — the third
//! TOTAL-integration slice, mirroring the Sorcerer/Bard total slices:
//! base table count + Charisma bonus count, the pure sum of two records
//! this codebase already grounds
//! (`tests/sd13_paladin_spells_per_day_counts.rs` +
//! `tests/sd13_paladin_bonus_spells.rs`), each carrying its own
//! two-source verification. No new rules content and no source fetch.
//!
//! One record per ACCESSIBLE spell level:
//! `class_chassis.paladin.partial_caster.total_spells_per_day.spell_level_<N>`.
//! On the deterministic fixtures (Charisma 14, modifier +2):
//!
//! - level 4: total 1 (base "0" entry + bonus 1) — the bonus-spells-only
//!   access, previously visible as two records side by side, now lands as
//!   arithmetic: the level-4 paladin genuinely casts 1 first-level spell
//!   per day;
//! - level 7: totals 2/1 (base 1/0 + bonus 1/1);
//! - level 10: totals 3/2/0 (base 2/1/0 + bonus 1/1/0) — the 3rd-level
//!   total is THE TRANCHE'S FIRST HONEST ZERO TOTAL: a "0" base entry
//!   plus a modifier-below-spell-level 0 bonus means this paladin can
//!   access 3rd-level spells but currently casts none per day, exactly
//!   what the PF1 rules produce for Charisma 14.
//!
//! Live end-to-end: raising the fixture Charisma to 16 (+3) turns the
//! 3rd-level total from 0 to 1. Counts only: no prepared-posture
//! selection, no casting execution, no slot consumption or tracking, no
//! save resolution. The `class_spell.paladin.partial_caster.unsupported`
//! blocker stays claim-blocking (message updated: the TOTAL integration
//! is no longer deferred — the remainder is spell-source lineage and the
//! prepared posture; the pinned tokens "partial" / "level - 3" and the
//! sibling pins on lineage/posture/"total" are preserved, the last via
//! the grounded-list wording).
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
const PALADIN_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level7_sd13_deterministic_input.txt");
const PALADIN_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const TOTAL_PREFIX: &str = "class_chassis.paladin.partial_caster.total_spells_per_day.";
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

fn total_values(fixture: &str) -> Vec<(String, i16)> {
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);
    computation
        .explanations
        .iter()
        .filter(|e| e.id.starts_with(TOTAL_PREFIX))
        .map(|e| (e.id.clone(), e.value))
        .collect()
}

fn id(spell_level: u8) -> String {
    format!("{TOTAL_PREFIX}spell_level_{spell_level}")
}

// ----- Below the level-4 spell gate: no total records at all -----

#[test]
fn paladin_level3_has_no_total_records() {
    assert!(
        total_values(PALADIN_LEVEL3_FIXTURE).is_empty(),
        "a level-3 paladin has no accessible spell level, so no total record may exist"
    );
}

// ----- Level 4: the bonus-only access lands as arithmetic (0 + 1 = 1) -----

#[test]
fn paladin_level4_total_is_one_from_the_bonus_alone() {
    let input = load(PALADIN_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let total = computation
        .explanations
        .iter()
        .find(|e| e.id == id(1))
        .expect("the 1st-level total record must exist at level 4");
    assert_eq!(
        total.value, 1,
        "total = base \"0\" entry + Charisma bonus 1 = 1 — the bonus-spells-only access as \
         arithmetic: the level-4 paladin genuinely casts one 1st-level spell per day: {}",
        total.detail
    );
    assert!(
        total.detail.contains("base") && total.detail.contains("bonus"),
        "the record must present itself as the sum of the grounded base and bonus records: {}",
        total.detail
    );
}

// ----- The total ladder, including the tranche's first honest zero total -----

#[test]
fn paladin_totals_are_the_sum_of_the_grounded_base_and_bonus_ladders() {
    assert_eq!(
        total_values(PALADIN_LEVEL7_FIXTURE),
        vec![(id(1), 2), (id(2), 1)],
        "level 7, Charisma +2: totals 2/1 (base 1/0 + bonus 1/1)"
    );
    assert_eq!(
        total_values(PALADIN_LEVEL10_FIXTURE),
        vec![(id(1), 3), (id(2), 2), (id(3), 0)],
        "level 10, Charisma +2: totals 3/2/0 — the 3rd-level total is the tranche's first \
         honest ZERO total (base \"0\" entry + modifier-below-spell-level 0 bonus): 3rd-level \
         spells are accessible but none are castable per day at Charisma 14"
    );
}

// ----- The arithmetic is live end-to-end -----

#[test]
fn paladin_totals_track_the_charisma_modifier() {
    let raised = PALADIN_LEVEL10_FIXTURE.replace("ability=charisma:14", "ability=charisma:16");
    let input = load(&raised);
    let computation = compute_pilot_base_chassis(&input);
    let third = computation
        .explanations
        .iter()
        .find(|e| e.id == id(3))
        .expect("the 3rd-level total record must exist");
    assert_eq!(
        third.value, 1,
        "raising Charisma to 16 (+3) turns the 3rd-level total from 0 to 1 (base 0 + bonus \
         1) — live end-to-end arithmetic"
    );
}

// ----- Counts only; the blocker defers the remaining burden -----

#[test]
fn paladin_level10_blocker_stays_and_stops_deferring_the_total() {
    let input = load(PALADIN_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let total = computation
        .explanations
        .iter()
        .find(|e| e.id == id(1))
        .expect("the 1st-level total record must exist");
    assert!(
        total.detail.contains("no slot consumption")
            || total.detail.contains("no prepared-posture"),
        "the record must state it grounds the count only, never execution or posture: {}",
        total.detail
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
        !blocker.message.contains("TOTAL integration are deferred")
            && !blocker.message.contains("TOTAL integration are \\
             deferred"),
        "the blocker must stop deferring the now-grounded total integration: {}",
        blocker.message
    );
    assert!(
        blocker.message.contains("spell-source lineage")
            && blocker.message.contains("prepared posture"),
        "the blocker must still name the genuinely-unproven remainder: {}",
        blocker.message
    );
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_paladin_total_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(TOTAL_PREFIX)),
        "the Fighter chassis must not surface any paladin total record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Paladin is not promoted -----

#[test]
fn multiclass_paladin_does_not_gain_total_records() {
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
            .any(|e| e.id.starts_with(TOTAL_PREFIX)),
        "multiclass Paladin must not gain any total record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Paladin must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the total grounding -----

#[test]
fn matrix_paladin_row_names_the_total_grounding() {
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
            .contains("sd13_paladin_total_spells_per_day"),
        "paladin row must cite the live total proof surface: {}",
        paladin.grounding_ref
    );
    assert!(
        paladin
            .blocker_or_lossiness_note
            .contains("total_spells_per_day"),
        "paladin partial note must name the grounded total records: {}",
        paladin.blocker_or_lossiness_note
    );
}
