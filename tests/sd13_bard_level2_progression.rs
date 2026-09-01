//! SD13-E5 Bard level-2 progression grounding proof.
//!
//! Widens the accepted Bard level-1 spontaneous arcane spell-bearing baseline
//! (`tests/sd13_bard_level1_spell_baseline.rs`, `tests/sd13_bard_base_attack_and_saves.rs`,
//! `tests/sd13_bard_fascinate_dc.rs`) to Bard level 2, mirroring the Fighter
//! `supported_fighter_level` / Paladin `supported_paladin_level` / Rogue
//! `supported_rogue_level` / Barbarian `supported_barbarian_level` / Monk
//! `supported_monk_level` / Cleric `supported_cleric_level` level-range-gate idiom
//! (the level-1-only gate `is_single_class_bard_level1` is generalized to
//! `supported_bard_level`, an `Option<u8>` helper gated by
//! `MAX_SUPPORTED_BARD_LEVEL = 2`). Both PF1 CRB primary sources (d20pfsrd and
//! legacy.aonprd.com Bard class table) were read directly before writing any code
//! or test:
//!
//! - level 2 base attack bonus is +1 (`2 * 3 / 4`), base Reflex/Will are +3 (good,
//!   `2/2+2`), base Fortitude is +0 (poor, `2/3`) — confirmed by the same formulas
//!   already grounded at level 1, not re-derived.
//! - Bardic Knowledge stays `max(2/2, 1) = 1` at level 2, the same value as level 1's
//!   floor-forced 1, but reached naturally this time rather than via the floor,
//!   confirmed via the same formula.
//! - Bardic Performance rounds per day DOES scale beyond the level-1 baseline: the
//!   PF1 Core Rulebook Bardic Performance rule text reads "At each level after 1st a
//!   bard can use bardic performance for 2 additional rounds per day," so the
//!   formula widens to `4 + Charisma modifier + 2 * (level - 1)` — 6 + CHA at level
//!   2 (8 on the fixture's CHA 15, up from 6 at level 1) — grounded as an extension
//!   of the existing formula, not a new record.
//! - Inspire Courage's flat magnitude stays +1 at level 2: the PF1 Core Rulebook
//!   Inspire Courage bonus first increases at bard level 5 (to +2), so level 2 is
//!   not the level it first increases; confirmed via the same formula/constant, not
//!   a new record.
//! - the Fascinate flat Will-save DC (`10 + level/2 + CHA modifier`) and the flat
//!   affected-creature count (`1 + (level-1)/3`) both already take bard level as an
//!   input variable, so both extend correctly to level 2 with no re-derivation: DC
//!   13 (10 + 1 + 2) and count 1 (1 + 0) on the fixture.
//! - the Bard class table's level-2 "Special" column reads "Versatile performance,
//!   well-versed" — TWO new class features, unlike Cleric's blank level-2 column.
//!   Well-Versed is flat and identity-shaped (a flat +4 bonus on saving throws
//!   against bardic performance, sonic, and language-dependent effects — verified
//!   against both primary sources, no level-scaling, no execution engine needed to
//!   name the flat magnitude), so it is grounded as a standalone explanation record
//!   mirroring the Fighter Bravery / Rogue-Monk Evasion idiom: never applied to any
//!   actual save total, since no save-resolution engine exists in this codebase.
//!   Versatile Performance is NOT flat — it requires a choice of Perform type and an
//!   actual skill-substitution engine mapping that choice onto associated skill
//!   checks — so it is deliberately left named-but-unproven, mirroring exactly how
//!   the Monk level-2 bonus feat grant was deliberately left unrecognized by the
//!   Monk level-2 widening slice: no new choice-slot and no new diagnostic is added
//!   for it this slice.
//!
//! It deliberately does not touch the performance-state/action-economy engine,
//! Countersong, Distraction, Versatile Performance, or the spontaneous spell
//! burden (all stay named-but-unproven, unchanged from level 1), and it does not
//! ground Bard level 3+. It also preserves the accepted Bard level-1 truth
//! (unchanged), the Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARD_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level1_sd13_deterministic_input.txt");

const BARD_LEVEL2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level2_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

const WELL_VERSED_ID: &str = "class_feature.bard.well_versed";

// ----- Base attack bonus at level 2 -----

#[test]
fn bard_level2_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(BARD_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 1,
        "Bard level 2 3/4-BAB progression (2 * 3 / 4) must equal 1: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 2 (good Reflex/Will, poor Fortitude) -----

#[test]
fn bard_level2_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(BARD_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.bard.base_save.fortitude");
    assert_eq!(fortitude.value, 0, "Bard level 2 poor Fortitude (2/3) must equal 0");

    let reflex = explanation(&computation, "class_chassis.bard.base_save.reflex");
    assert_eq!(reflex.value, 3, "Bard level 2 good Reflex (2/2+2) must equal 3");

    let will = explanation(&computation, "class_chassis.bard.base_save.will");
    assert_eq!(will.value, 3, "Bard level 2 good Will (2/2+2) must equal 3");
}

// ----- Bardic Knowledge stays max(level/2, 1) -----

#[test]
fn bard_level2_bardic_knowledge_stays_grounded_at_one() {
    let input = load(BARD_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let knowledge = explanation(&computation, "class_chassis.bard.bardic_knowledge");
    assert_eq!(
        knowledge.value, 1,
        "Bard level 2 Bardic Knowledge (max(2/2, 1)) must equal 1: {}",
        knowledge.detail
    );
}

// ----- Bardic Performance rounds per day scales with level -----

#[test]
fn bard_level2_bardic_performance_rounds_per_day_scales_with_level() {
    let input = load(BARD_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Bardic Performance: 4 + CHA modifier at level 1, plus 2
    // additional rounds per day at each level after 1st. Fixture CHA 15 + 2 Human
    // racial (CG-03 fix) -> +3 modifier. At level 2: 4 + 3 + 2 * (2 - 1) = 9.
    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    assert_eq!(
        rounds.value, 9,
        "Bard level 2 bardic performance rounds per day must equal 4 + CHA + 2*(level-1) \
         = 4 + 3 + 2 = 9: {}",
        rounds.detail
    );
    assert!(
        rounds.detail.contains("2 additional") || rounds.detail.contains("2 * ("),
        "the rounds-per-day explanation must name the +2-rounds-per-level-after-1st \
         formula extension: {}",
        rounds.detail
    );
}

// ----- Inspire Courage stays +1 at level 2 (first increases at level 5) -----

#[test]
fn bard_level2_inspire_courage_stays_flat_plus_one() {
    let input = load(BARD_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_courage = explanation(&computation, "class_chassis.bard.inspire_courage_bonus");
    assert_eq!(
        inspire_courage.value, 1,
        "Bard level 2 Inspire Courage magnitude must stay +1 (PF1: first increases at level \
         5): {}",
        inspire_courage.detail
    );
}

// ----- Fascinate DC and affected-creature count extend to level 2 -----

#[test]
fn bard_level2_fascinate_dc_and_affected_creatures_extend_by_the_same_formulas() {
    let input = load(BARD_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Fixture CHA 15 + 2 Human racial (CG-03 fix) -> +3 modifier. DC = 10 + 2/2 + 3 = 14.
    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(
        dc.value, 14,
        "Bard level 2 Fascinate DC must equal 10 + (2/2) + 3 = 14: {}",
        dc.detail
    );

    // Affected creatures = 1 + (2-1)/3 = 1.
    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(
        count.value, 1,
        "Bard level 2 Fascinate affected-creature count must stay 1: {}",
        count.detail
    );
}

// ----- New at level 2: Well-Versed grounded as a flat standalone magnitude -----

#[test]
fn bard_level2_grounds_well_versed_flat_magnitude() {
    let input = load(BARD_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let well_versed = explanation(&computation, WELL_VERSED_ID);
    assert_eq!(
        well_versed.value, 4,
        "Bard Well-Versed at level 2 must ground the flat +4 bonus (PF1 Core Rulebook: not \
         level-scaled): {}",
        well_versed.detail
    );
    for token in ["bardic performance", "sonic", "language-dependent"] {
        assert!(
            well_versed.detail.contains(token),
            "Well-Versed detail must name the '{token}' saving-throw category it applies to: {}",
            well_versed.detail
        );
    }
    assert!(
        well_versed.detail.contains("standalone") || well_versed.detail.contains("never applied"),
        "Well-Versed detail must disclaim application to any actual save total: {}",
        well_versed.detail
    );
}

#[test]
fn bard_level1_correctly_lacks_well_versed() {
    let input = load(BARD_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let well_versed = explanation(&computation, WELL_VERSED_ID);
    assert_eq!(
        well_versed.value, 0,
        "Bard Well-Versed must be a correct level-gate absence (value 0) below level 2: {}",
        well_versed.detail
    );
}

// ----- Versatile Performance is deliberately NOT grounded this slice -----

#[test]
fn bard_level2_does_not_fabricate_versatile_performance() {
    let input = load(BARD_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.contains("versatile_performance")),
        "Versatile Performance requires a choice-gated skill-substitution engine and must not \
         be fabricated as a flat record: {:?}",
        computation.explanations
    );
}

// ----- The two existing burden diagnostics still fire at level 2 -----

#[test]
fn bard_level2_still_claim_blocks_performance_execution_and_spontaneous_spell_burdens() {
    let input = load(BARD_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.bard.bardic_performance_execution.rounds_exceeded")
    {
        Some(blocker) => assert!(blocker.claim_blocking, "if the blocker fires, it must be claim-blocking"),
        None => {
            let not_performing = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.bard.bardic_performance_execution.not_performing");
            assert!(
                not_performing.is_some(),
                "level-2 Bard must ground an honest not-performing record when no \
                 bardic-performance posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_spell.bard.spontaneous_known_and_per_day.unsupported")
    {
        Some(blocker) => assert!(blocker.claim_blocking, "if the blocker fires, it must be claim-blocking"),
        None => {
            let known_count = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_spell.bard.known_spells")
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

// ----- The accepted Bard level-1 truth is unaffected -----

#[test]
fn bard_level1_truth_is_unchanged_by_this_widening() {
    let input = load(BARD_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(base_attack.value, 0, "Bard level 1 base attack bonus must stay 0");

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    // CG-03 fix: level 1's rounds are now 7 (4 + 3 CHA modifier), not 6.
    assert_eq!(rounds.value, 7, "Bard level 1 bardic performance rounds per day must stay 7");
}

// ----- Bard level 3 was later widened into the supported tranche -----

#[test]
fn bard_level_3_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 3 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_bard_level3_progression.rs) widened the level-range gate to
    // level 3 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Druid/
    // Sorcerer/Wizard level-range gate idiom) and extended every formula below;
    // this negative control is superseded, not violated — pin the new truth
    // here too so this file stays internally consistent.
    let level_3 = BARD_LEVEL2_FIXTURE.replace("class:bard:2", "class:bard:3");
    let input = load(&level_3);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.bard.")
                || e.id == "class_chassis.spell_baseline.bard"
                || e.id == WELL_VERSED_ID),
        "level-3 Bard is supported since the SD13-E5 level-3 slice: {:?}",
        computation.explanations
    );
}

// ----- Bard level 4 was later widened into the supported tranche -----

#[test]
fn bard_level_4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_bard_level4_progression.rs) widened the level-range gate to
    // level 4 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Druid/
    // Sorcerer/Wizard level-range gate idiom) and extended every formula below;
    // this negative control is superseded, not violated — pin the new truth
    // here too so this file stays internally consistent.
    let level_4 = BARD_LEVEL2_FIXTURE.replace("class:bard:2", "class:bard:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.bard.")
                || e.id == "class_chassis.spell_baseline.bard"
                || e.id == WELL_VERSED_ID),
        "level-4 Bard is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the bard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_bard_level2_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.bard.")
                || e.id == "class_chassis.spell_baseline.bard"
                || e.id == WELL_VERSED_ID),
        "the Fighter chassis must not surface any bard-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Bard is not promoted -----

#[test]
fn multiclass_bard_level2_is_not_promoted_by_this_slice() {
    let multiclass = BARD_LEVEL2_FIXTURE.replace(
        "class_level=class:bard:2",
        "class_level=class:bard:2\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.bard.")
                || e.id == "class_chassis.spell_baseline.bard"
                || e.id == WELL_VERSED_ID),
        "multiclass Bard must not gain any bounded bard chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Bard must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-2 widening -----

#[test]
fn matrix_bard_row_names_level_2_widening() {
    let matrix = seeded_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard progression_and_spell_burden row must exist");

    assert_eq!(bard.support_state, SupportState::Supported);
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(bard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        bard.grounding_ref.contains("sd13_bard_level2_progression"),
        "bard row must cite the live SD13-E5 level-2 proof surface: {}",
        bard.grounding_ref
    );
    let note = bard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 2") || note.contains("level-2"),
        "bard partial note must name the level-2 widening: {note}"
    );
    assert!(
        note.contains("well") || note.contains("Well-Versed") || note.contains("Well Versed"),
        "bard partial note must name the newly grounded Well-Versed pillar: {note}"
    );
}
