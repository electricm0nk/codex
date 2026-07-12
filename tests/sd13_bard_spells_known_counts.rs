//! SD13-E5 Bard base spells-known count grounding proof — the first
//! spells-KNOWN slice, complementing the completed flat spell surface
//! (access ladder + base per-day counts + base save DCs) with the OTHER
//! bard spell table, per the Cleric domain-slot-count literal-table
//! doctrine (a lookup table, not arithmetic — no formula is invented).
//! Both PF1 CRB primary sources (d20pfsrd and legacy.aonprd.com Bard
//! Spells Known table) were read this cycle before writing any code, and
//! both show identical raw rows for levels 1-10 (0th through 4th spell
//! level):
//!
//! - level 1: `4/2/—/—/—`   - level 2: `5/3/—/—/—`  - level 3: `6/4/—/—/—`
//! - level 4: `6/4/2/—/—`   - level 5: `6/4/3/—/—`  - level 6: `6/4/4/—/—`
//! - level 7: `6/5/4/2/—`   - level 8: `6/5/4/3/—`  - level 9: `6/5/4/4/—`
//! - level 10: `6/5/5/4/2`
//!
//! UNLIKE the spells-per-day table, the spells-known table INCLUDES the
//! 0th level (cantrips) — this is exactly where the access-ladder slice's
//! "cantrips are spells known only" note lands: a level-1 bard carries
//! TWO known-count records (0th and 1st) but only ONE per-day record
//! (1st). One record per spell level with a non-"—" known column:
//! `class_chassis.bard.spontaneous.spells_known.spell_level_<N>` (N from
//! 0) with the raw known count as its value; "—" spell levels get no
//! record at all.
//!
//! This grounds the base known COUNTS only: the selection of WHICH spells
//! are known (from the Bard list) is never computed — no spell-list
//! content, no spell identities, and no swap/retraining rules — and the
//! `class_spell.bard.spontaneous_known_and_per_day.unsupported` blocker
//! stays claim-blocking at every supported level (its message is updated
//! to defer the which-spells selection rather than the now-grounded
//! counts, preserving the pinned tokens "spontaneous" / "spells known" /
//! "spells per day").
//!
//! Sibling allow-list extensions: the established pattern (level-1
//! baseline by prefix; level-4/5/6/7/8 `known_bard_ids` by exact ids).
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
const BARD_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level6_sd13_deterministic_input.txt");
const BARD_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level7_sd13_deterministic_input.txt");
const BARD_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const KNOWN_PREFIX: &str = "class_chassis.bard.spontaneous.spells_known.";
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
fn bard_level1_knows_cantrips_and_first_level_spells() {
    let input = load(BARD_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        known_values(BARD_LEVEL1_FIXTURE),
        vec![(id(0), 4), (id(1), 2)],
        "level 1 (`4/2/—/—/—`): TWO known-count records — the known table includes the 0th \
         level (cantrips), unlike the per-day table"
    );

    let cantrips = computation
        .explanations
        .iter()
        .find(|e| e.id == id(0))
        .expect("the 0th-level (cantrip) known-count record must exist");
    assert!(
        cantrips.detail.contains("known count") || cantrips.detail.contains("known COUNT"),
        "the record must state it grounds the base known count only: {}",
        cantrips.detail
    );
}

// ----- The literal table rows hold at every grounded step -----

#[test]
fn bard_spells_known_match_the_raw_table_rows() {
    assert_eq!(
        known_values(BARD_LEVEL4_FIXTURE),
        vec![(id(0), 6), (id(1), 4), (id(2), 2)],
        "level 4 (`6/4/2/—/—`)"
    );
    assert_eq!(
        known_values(BARD_LEVEL6_FIXTURE),
        vec![(id(0), 6), (id(1), 4), (id(2), 4)],
        "level 6 (`6/4/4/—/—`)"
    );
    assert_eq!(
        known_values(BARD_LEVEL7_FIXTURE),
        vec![(id(0), 6), (id(1), 5), (id(2), 4), (id(3), 2)],
        "level 7 (`6/5/4/2/—`)"
    );
    assert_eq!(
        known_values(BARD_LEVEL10_FIXTURE),
        vec![(id(0), 6), (id(1), 5), (id(2), 5), (id(3), 4), (id(4), 2)],
        "level 10 (`6/5/5/4/2`)"
    );
}

// ----- Known counts only; the blocker defers the which-spells selection -----

#[test]
fn bard_level10_blocker_stays_and_defers_the_which_spells_selection() {
    let input = load(BARD_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

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
        blocker.message.contains("WHICH spells") || blocker.message.contains("which spells"),
        "the blocker must defer the which-spells selection now that the counts are \
         grounded: {}",
        blocker.message
    );
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_bard_known_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(KNOWN_PREFIX)),
        "the Fighter chassis must not surface any bard spells-known record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Bard is not promoted -----

#[test]
fn multiclass_bard_does_not_gain_known_records() {
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
            .any(|e| e.id.starts_with(KNOWN_PREFIX)),
        "multiclass Bard must not gain any spells-known record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Bard must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the known-count grounding -----

#[test]
fn matrix_bard_row_names_the_known_count_grounding() {
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
        bard.grounding_ref
            .contains("sd13_bard_spells_known_counts"),
        "bard row must cite the live spells-known proof surface: {}",
        bard.grounding_ref
    );
    assert!(
        bard.blocker_or_lossiness_note.contains("spells_known"),
        "bard partial note must name the grounded known-count records: {}",
        bard.blocker_or_lossiness_note
    );
}
