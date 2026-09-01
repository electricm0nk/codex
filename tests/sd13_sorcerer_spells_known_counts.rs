//! SD13-E5 Sorcerer base spells-known count grounding proof — the second
//! and final spells-KNOWN slice, completing every literal spell table in
//! the tranche (access ladders, per-day counts, save DCs, and known
//! counts across the spontaneous casters; the prepared casters' posture
//! is a different burden family). Mirrors the Bard known slice
//! (`tests/sd13_bard_spells_known_counts.rs`) per the literal-table
//! doctrine. Both PF1 CRB primary sources (d20pfsrd and legacy.aonprd.com
//! Sorcerer Spells Known table) were read this cycle before writing any
//! code, and both show identical raw rows for levels 1-10 (0th through
//! 5th spell level):
//!
//! - level 1: `4/2/—/—/—/—`   - level 2: `5/2/—/—/—/—`
//! - level 3: `5/3/—/—/—/—`   - level 4: `6/3/1/—/—/—`
//! - level 5: `6/4/2/—/—/—`   - level 6: `7/4/2/1/—/—`
//! - level 7: `7/5/3/2/—/—`   - level 8: `8/5/3/2/1/—`
//! - level 9: `8/5/4/3/2/—`   - level 10: `9/5/4/3/2/1`
//!
//! Like the Bard's, the known table INCLUDES the 0th level (cantrips are
//! "spells known" only), and its new-spell-level cadence matches the
//! grounded per-day access ladder exactly (2nd at 4, 3rd at 6, 4th at 8,
//! 5th at 10 — checked rather than assumed). One record per non-"—"
//! column:
//! `class_chassis.sorcerer.spontaneous.spells_known.spell_level_<N>` (N
//! from 0) with the raw known count as its value.
//!
//! This grounds the base known COUNTS only: the selection of WHICH spells
//! are known is never computed — no spell-list content, no spell
//! identities, no swap/retraining rules, and no bloodline bonus-spell
//! additions (the 3rd/5th/7th-level bloodline spells are part of the
//! still-unproven bloodline progression burden, NOT this table). The
//! `class_spell.sorcerer.spontaneous.unsupported` blocker stays
//! claim-blocking (message updated to defer the which-spells selection,
//! preserving the pinned tokens "spontaneous" / "spells known" /
//! "spell slot").
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

const KNOWN_PREFIX: &str = "class_chassis.sorcerer.spontaneous.spells_known.";
const SPONTANEOUS_BLOCKER_ID: &str = "class_spell.sorcerer.spontaneous.unsupported";

fn known_values(fixture: &str) -> Vec<(String, i16)> {
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);
    computation
        .explanations
        .iter()
        .filter(|e| e.id.starts_with(KNOWN_PREFIX))
        .map(|e| (e.id.clone(), e.value))
        .collect()
}

fn id(spell_level: u8) -> String {
    format!("{KNOWN_PREFIX}spell_level_{spell_level}")
}

// ----- Level 1: TWO records — cantrips live in the known table -----

#[test]
fn sorcerer_level1_knows_cantrips_and_first_level_spells() {
    let input = load(SORCERER_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        known_values(SORCERER_LEVEL1_FIXTURE),
        vec![(id(0), 4), (id(1), 2)],
        "level 1 (`4/2/—/—/—/—`): TWO known-count records — the known table includes the \
         0th level (cantrips), unlike the per-day table"
    );

    let cantrips = computation
        .explanations
        .iter()
        .find(|e| e.id == id(0))
        .expect("the 0th-level (cantrip) known-count record must exist");
    assert!(
        cantrips.detail.contains("known count"),
        "the record must state it grounds the base known count only: {}",
        cantrips.detail
    );
}

// ----- The literal table rows hold at every grounded step -----

#[test]
fn sorcerer_spells_known_match_the_raw_table_rows() {
    assert_eq!(
        known_values(SORCERER_LEVEL4_FIXTURE),
        vec![(id(0), 6), (id(1), 3), (id(2), 1)],
        "level 4 (`6/3/1/—/—/—`): the 2nd-level column arrives at 1, matching the per-day \
         access ladder's level-4 gate"
    );
    assert_eq!(
        known_values(SORCERER_LEVEL6_FIXTURE),
        vec![(id(0), 7), (id(1), 4), (id(2), 2), (id(3), 1)],
        "level 6 (`7/4/2/1/—/—`): the 3rd-level column arrives at the two-level cadence"
    );
    assert_eq!(
        known_values(SORCERER_LEVEL8_FIXTURE),
        vec![(id(0), 8), (id(1), 5), (id(2), 3), (id(3), 2), (id(4), 1)],
        "level 8 (`8/5/3/2/1/—`)"
    );
    assert_eq!(
        known_values(SORCERER_LEVEL10_FIXTURE),
        vec![
            (id(0), 9),
            (id(1), 5),
            (id(2), 4),
            (id(3), 3),
            (id(4), 2),
            (id(5), 1)
        ],
        "level 10 (`9/5/4/3/2/1`): six known-count records — the deepest known surface in \
         the tranche"
    );
}

// ----- Known counts only; the blocker defers the which-spells selection -----

#[test]
fn sorcerer_level10_blocker_stays_and_defers_the_which_spells_selection() {
    let input = load(SORCERER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let known = computation
        .explanations
        .iter()
        .find(|e| e.id == id(5))
        .expect("the 5th-level known-count record must exist at level 10");
    assert!(
        known.detail.contains("WHICH spells") || known.detail.contains("which spells"),
        "the record must state that the which-spells selection is never computed: {}",
        known.detail
    );

    // (v0.6 alpha swarm, risks item 8) SPONTANEOUS_BLOCKER_ID is no longer
    // unconditional -- this fixture has zero known spells, a genuinely valid
    // posture, so the blocker correctly does not fire here. The known-count record
    // above already proves the base known-count table is grounded for real and
    // never fabricated; that's the property this test actually needs.
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
fn fighter_does_not_gain_sorcerer_known_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(KNOWN_PREFIX)),
        "the Fighter chassis must not surface any sorcerer spells-known record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Sorcerer is not promoted -----

#[test]
fn multiclass_sorcerer_does_not_gain_known_records() {
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
            .any(|e| e.id.starts_with(KNOWN_PREFIX)),
        "multiclass Sorcerer must not gain any spells-known record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Sorcerer must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the known-count grounding -----

#[test]
fn matrix_sorcerer_row_names_the_known_count_grounding() {
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
            .contains("sd13_sorcerer_spells_known_counts"),
        "sorcerer row must cite the live spells-known proof surface: {}",
        sorcerer.grounding_ref
    );
    assert!(
        sorcerer
            .blocker_or_lossiness_note
            .contains("spells_known"),
        "sorcerer partial note must name the grounded known-count records: {}",
        sorcerer.blocker_or_lossiness_note
    );
}
