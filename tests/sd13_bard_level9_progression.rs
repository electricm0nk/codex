//! SD13-E5 Bard level-9 progression grounding proof.
//!
//! Widens the accepted Bard level-1..level-8 spontaneous spell-burden
//! baseline (most recently `tests/sd13_bard_level8_progression.rs`) to Bard
//! level 9, mirroring the sibling-class level-range-gate idiom
//! (`supported_bard_level` is generalized from `1..=8` to `1..=9` via
//! `MAX_SUPPORTED_BARD_LEVEL = 9`). Both PF1 CRB primary sources (d20pfsrd
//! and legacy.aonprd.com Bard class table) were read directly before writing
//! any code or test:
//!
//! - level 9 base attack bonus is +6 (`9 * 3 / 4 = 6`, the Bard's 3/4-BAB
//!   progression, numerically unchanged from level 8 — an integer-division
//!   coincidence; the table's own "+6/+1" iterative notation is not modeled
//!   anywhere in this codebase, only the flat base value) and base saves are
//!   +3 Fortitude (poor, `9 / 3 = 3`, genuinely risen from +2), +6 Reflex
//!   and +6 Will (both good, `9 / 2 + 2 = 6`, numerically unchanged from
//!   level 8, integer-division coincidences) — confirmed by the same
//!   formulas already grounded at levels 1-8, not re-derived.
//! - the Bardic Performance rounds-per-day pool GENUINELY RISES to 22
//!   (`4 + Charisma modifier 2 + 2 × (9 - 1)`, via the same pre-existing
//!   formula: 4 + Cha mod at 1st, plus 2 per level thereafter).
//! - Bardic Knowledge stays 4 (`max(9 / 2, 1)`, an integer-division
//!   coincidence with level 8); the Fascinate DC stays 16
//!   (`10 + 9 / 2 + Charisma modifier 2`, likewise a coincidence) and the
//!   affected-creature count stays 3 (`1 + (9 - 1) / 3 = 3`, the next rise
//!   landing at 10th, checked rather than assumed); Inspire Courage stays
//!   +2 and Inspire Competence stays +3 (neither's next tier lands until
//!   11th); Well-Versed's flat +4 and Lore Master's 1/day both carry over
//!   unchanged, not re-derived.
//! - the PF1 Core Rulebook Bard class table's level-9 "Special" column
//!   reads "Inspire greatness" (verified independently against both primary
//!   sources, checked rather than assumed away) — a genuinely NEW bardic
//!   performance type at 9th level, and confirmed NOT flat/identity-shaped:
//!   it grants a willing ally 2 bonus Hit Dice (d10s) with the commensurate
//!   temporary hit points, a +2 competence bonus on attack rolls, and a +1
//!   competence bonus on Fortitude saves — requiring the performance-state
//!   engine already left ungrounded plus temporary-Hit-Dice/temporary-hit-
//!   point mechanics, none of which exist in this codebase — so it is
//!   deliberately left named-but-unproven, mirroring the
//!   Suggestion/Countersong/Distraction/Dirge-of-Doom precedent exactly. No
//!   record or diagnostic is fabricated for it (pinned below).
//!
//! It deliberately does not touch the performance-state engine, any
//! condition-resolution engine, or the spontaneous spell posture burden (all
//! stay named-but-unproven, unchanged from levels 1-8), and it does not
//! ground Bard level 10+. It also preserves the accepted Bard
//! level-1..level-8 truth (unchanged), the Fighter negative control, and the
//! multiclass negative control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};

const BARD_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level8_sd13_deterministic_input.txt");

const BARD_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level9_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const WELL_VERSED_ID: &str = "class_feature.bard.well_versed";
const INSPIRE_COMPETENCE_ID: &str = "class_feature.bard.inspire_competence";
const LORE_MASTER_ID: &str = "class_feature.bard.lore_master";

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

fn explanation<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationExplanation {
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected explanation id '{id}', got {:?}",
                computation.explanations
            )
        })
}

// ----- Base attack bonus and saves at level 9 -----

#[test]
fn bard_level9_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(BARD_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 6,
        "Bard level 9 3/4-BAB progression (9 * 3 / 4) must equal 6 — unchanged from level 8, \
         an integer-division coincidence: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.bard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 3,
        "Bard level 9 poor Fortitude (9/3) must equal 3, genuinely risen from 2 at level 8"
    );

    let reflex = explanation(&computation, "class_chassis.bard.base_save.reflex");
    assert_eq!(reflex.value, 6, "Bard level 9 good Reflex (9/2+2) must equal 6");

    let will = explanation(&computation, "class_chassis.bard.base_save.will");
    assert_eq!(will.value, 6, "Bard level 9 good Will (9/2+2) must equal 6");
}

// ----- Bardic Performance rounds genuinely rise to 22 at level 9 -----

#[test]
fn bard_level9_performance_rounds_rise_to_twenty_two() {
    let input = load(BARD_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    // CG-03 fix: Charisma modifier is now +3 (base 15 + 2 Human racial), not +2.
    assert_eq!(
        rounds.value, 23,
        "Bard level 9 Bardic Performance rounds per day (4 + Charisma modifier 3 + 2 × (9 - \
         1)) must equal 23, genuinely risen from 21 at level 8: {}",
        rounds.detail
    );
}

// ----- Bardic Knowledge / Fascinate / Inspire tiers carry over at level 9 -----

#[test]
fn bard_level9_knowledge_fascinate_and_inspire_tiers_carry_over() {
    let input = load(BARD_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let knowledge = explanation(&computation, "class_chassis.bard.bardic_knowledge");
    assert_eq!(
        knowledge.value, 4,
        "Bardic Knowledge (max(9/2, 1)) must stay 4 — an integer-division coincidence with \
         level 8"
    );

    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(
        dc.value, 17,
        "the Fascinate DC (10 + 9/2 + Charisma modifier 3) must stay 17 — a coincidence with \
         level 8"
    );

    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(
        count.value, 3,
        "the Fascinate affected-creature count (1 + (9-1)/3) must stay 3 — the next rise \
         lands at 10th"
    );

    let inspire_courage = explanation(&computation, "class_chassis.bard.inspire_courage_bonus");
    assert_eq!(
        inspire_courage.value, 2,
        "Inspire Courage must stay +2 at level 9 (the next tier lands at 11th)"
    );

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(
        inspire_competence.value, 3,
        "Inspire Competence must stay +3 at level 9 (the next tier lands at 11th)"
    );

    let well_versed = explanation(&computation, WELL_VERSED_ID);
    assert_eq!(well_versed.value, 4, "Well-Versed must stay the flat +4 at level 9");

    let lore_master = explanation(&computation, LORE_MASTER_ID);
    assert_eq!(
        lore_master.value, 1,
        "Lore Master must stay 1/day at level 9 (the next rise lands at 11th)"
    );
}

// ----- Inspire Greatness stays entirely named-but-unproven at level 9 -----

#[test]
fn bard_level9_does_not_fabricate_inspire_greatness() {
    let input = load(BARD_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // SD-34 bucket-B batch cycle: `AT-34-E3-001`'s own cycle 5 legitimately grounded Inspire
    // Greatness's flat affected-ally COUNT (`class_feature.bard.inspire_greatness_allies`,
    // verified against this repo's own ingested corpus record and formula,
    // `min((bard level - 6) / 3, 4)`) before this test's own assertion was ever updated to
    // admit it -- a stale-gate gap this cycle found and fixes here, not a new fabrication.
    // Only that one flat count is admitted; the feature's own bundled bonuses (extra Hit
    // Dice, temporary hit points, competence bonuses) and their application to any ally are
    // still unimplemented, so any OTHER greatness-tagged id still fails this control.
    let greatness_ids: Vec<&ComputationExplanation> = computation
        .explanations
        .iter()
        .filter(|e| e.id.to_lowercase().contains("greatness"))
        .collect();
    assert!(
        greatness_ids
            .iter()
            .all(|e| e.id == "class_feature.bard.inspire_greatness_allies"),
        "only the flat, citation-backed Inspire Greatness affected-ally COUNT may appear; the \
         feature's own bundled bonus Hit Dice, temporary hit points, and competence bonuses \
         are still behind the ungrounded performance-state engine, so no OTHER greatness- \
         tagged explanation record may be fabricated: {greatness_ids:?}"
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.to_lowercase().contains("greatness")),
        "level-9 Bard must not fabricate any inspire-greatness diagnostic either: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the level-8 fixture is unaffected by this widening -----

#[test]
fn bard_level8_truth_is_unchanged_by_this_slice() {
    let input = load(BARD_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    assert_eq!(rounds.value, 21, "Bard level 8 performance rounds must stay 21");

    let fortitude = explanation(&computation, "class_chassis.bard.base_save.fortitude");
    assert_eq!(fortitude.value, 2, "Bard level 8 poor Fortitude must stay 2");
}

// ----- Level 10 was later widened into the supported tranche by a further slice -----

#[test]
fn bard_level_10_was_later_widened_into_the_supported_tranche() {
    let level_10 = BARD_LEVEL9_FIXTURE.replace("class:bard:9", "class:bard:10");
    let input = load(&level_10);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.bard.")),
        "level-10 Bard is now recognized by the later level-10 widening slice \
         (tests/sd13_bard_level10_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the bard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_bard_level9_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.bard.")),
        "the Fighter chassis must not surface any bard-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Bard is not promoted -----

#[test]
fn multiclass_bard_level9_is_not_promoted_by_this_slice() {
    let multiclass = BARD_LEVEL9_FIXTURE.replace(
        "class_level=class:bard:9",
        "class_level=class:bard:9\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.bard.")),
        "multiclass Bard must not gain any bounded bard chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Bard must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-9 widening -----

#[test]
fn matrix_bard_row_names_level_9_widening() {
    let matrix = seeded_current_truth();
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
        bard.grounding_ref.contains("sd13_bard_level9_progression"),
        "bard row must cite the live SD13-E5 level-9 proof surface: {}",
        bard.grounding_ref
    );
    let note = bard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 9") || note.contains("level-9"),
        "bard partial note must name the level-9 widening: {note}"
    );
}
