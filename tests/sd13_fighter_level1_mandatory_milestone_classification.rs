//! SD13-E3-F5 Fighter level-1 mandatory milestone classification proof.
//!
//! Proves the dedicated tranche this handoff authorizes: the accepted deterministic
//! Human Fighter level-1 pilot seam is reclassified into a bounded
//! level-1 mandatory-milestone classification that names which level-1 mandatory
//! milestones are proven (computed) on the deterministic pilot surface and which
//! remain unproven for the level-10 progression matrix, while preserving the
//! existing L2-10 row, the Rogue blocked negative control, the Human race seam,
//! and the Human interaction pressure row.
//!
//! It is intentionally not a level-2+ Fighter engine, not a general feat engine,
//! and not a level-10 progression engine. It only widens the existing L1 row's
//! blocker note into a concrete milestone enumeration, points the row's grounding
//! ref at this proof surface, and asserts the matrix reclassification without
//! silently promoting the row to Supported.
//!
//! Non-goals:
//! - do not claim Fighter L1 is fully supported (level-10 progression remains unclassified)
//! - do not collapse the L1 row into the L2-10 row (they keep distinct dimensions)
//! - do not weaken the Rogue blocked negative control or the Human race seam

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const LEVEL_1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

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

// ----- L1 deterministic pilot surface preserves its proven (computed) seams -----

#[test]
fn level_1_pilot_surface_still_emits_proven_base_chassis_explanations() {
    let input = load(LEVEL_1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    // The Fighter L1 deterministic pilot surface must still emit every proven
    // (computed) level-1 milestone explanation that the matrix note names.
    let required = [
        "class_chassis.base_attack_bonus",
        "class_chassis.base_save.fortitude",
        "class_chassis.base_save.reflex",
        "class_chassis.base_save.will",
        "defense.total_save.fortitude",
        "defense.total_save.reflex",
        "defense.total_save.will",
        "combat.baseline_melee_attack_bonus",
        "defense.baseline_armor_class",
    ];
    for id in required {
        assert!(
            explanation(&computation, id).id == id,
            "Fighter L1 must still emit the proven '{id}' explanation"
        );
    }
}

#[test]
fn level_1_pilot_surface_emits_ability_modifier_and_selected_skill_seams() {
    let input = load(LEVEL_1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    // At least one ability modifier explanation must surface (proves the
    // ability-modifier milestone is grounded for the chosen scores).
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "ability_modifier.strength"),
        "Fighter L1 must surface an ability modifier explanation"
    );
    // The selected skill modifier seam (climb/intimidate/swim per the L1 fixture)
    // proves the selected-skill milestone is grounded for the canonical picks.
    for id in [
        "skill.selected_modifier.climb",
        "skill.selected_modifier.intimidate",
        "skill.selected_modifier.swim",
    ] {
        assert!(
            explanation(&computation, id).id == id,
            "Fighter L1 must still emit the selected skill modifier '{id}'"
        );
    }
}

#[test]
fn level_1_base_attack_bonus_still_equals_fighter_chassis_value() {
    let input = load(LEVEL_1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(
        bab.value, 1,
        "Fighter level 1 BAB must remain +1 on the deterministic pilot surface"
    );
}

// ----- Control plane: the matrix reclassifies the L1 row to bounded Partial -----

#[test]
fn matrix_level_1_row_is_partial_and_computed_and_cites_f5_proof_surface() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let level_1 = matrix
        .row("class.fighter.level_1_pilot")
        .expect("level-1 pilot row must exist");

    // The slice keeps the row at Partial/Computed. It must NOT silently promote
    // to Supported because level-10 progression remains out of proof.
    assert_eq!(level_1.support_state, SupportState::Partial);
    assert_ne!(
        level_1.support_state,
        SupportState::Supported,
        "Fighter L1 must not be silently promoted to Supported by this slice"
    );
    assert_eq!(level_1.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        level_1.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );

    // The slice must point the L1 row's grounding ref at its own proof surface,
    // not the GE-06 view-model proof, so the audit trail names this tranche.
    assert!(
        level_1
            .grounding_ref
            .contains("sd13_fighter_level1_mandatory_milestone_classification"),
        "Fighter L1 row must cite the SD13-F5 mandatory-milestone proof surface: {}",
        level_1.grounding_ref
    );
}

#[test]
fn matrix_level_1_blocker_note_enumerates_proven_milestones() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let level_1 = matrix
        .row("class.fighter.level_1_pilot")
        .expect("level-1 pilot row must exist");
    let note = level_1.blocker_or_lossiness_note;

    assert!(
        !note.is_empty(),
        "partial Fighter L1 row must carry a non-empty blocker note"
    );

    // Proven (computed) level-1 surface tokens. The note must name each one
    // explicitly so downstream readers can see exactly what is grounded.
    for token in [
        "ability modifier",
        "base attack bonus",
        "base saving throws",
        "total saves",
        "baseline melee attack",
        "defense.baseline_armor_class",
        "selected skill modifier",
        "pilot view-model",
        "prerequisite",
        "invalid-choice",
        "claim-blocking",
    ] {
        assert!(
            note.contains(token),
            "Fighter L1 note must name the proven level-1 milestone token '{token}': {note}"
        );
    }
}

#[test]
fn matrix_level_1_blocker_note_enumerates_remaining_unproven_milestones() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let level_1 = matrix
        .row("class.fighter.level_1_pilot")
        .expect("level-1 pilot row must exist");
    let note = level_1.blocker_or_lossiness_note;

    // Remaining unproven Fighter level-1 mandatory milestones for the level-10
    // progression matrix. The note must name each one explicitly so the gap is
    // visible instead of hidden behind "unclassified".
    for token in [
        "hit point",
        "general class skill rank",
        "general feat",
        "equipment",
        "armor",
        "bonus-feat",
        "armor-training",
        "weapon-training",
        "level-10",
    ] {
        assert!(
            note.contains(token),
            "Fighter L1 note must name the remaining unproven level-1 milestone token '{token}': {note}"
        );
    }
}

#[test]
fn matrix_level_1_row_keeps_a_concrete_next_required_uplift() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let level_1 = matrix
        .row("class.fighter.level_1_pilot")
        .expect("level-1 pilot row must exist");

    // The next_required_uplift must stay non-empty AND must name the bounded
    // Fighter surface widening toward level-10 milestones (not a vague "widen").
    assert!(
        !level_1.next_required_uplift.is_empty(),
        "partial Fighter L1 row must keep a non-empty next_required_uplift"
    );
    assert!(
        level_1.next_required_uplift.contains("level-10"),
        "Fighter L1 next_required_uplift must name the level-10 progression widening: {}",
        level_1.next_required_uplift
    );
}

// ----- Control plane: peers remain stable after the L1 widening -----

#[test]
fn matrix_level_1_and_levels_2_10_remain_distinct_rows() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let level_1 = matrix
        .row("class.fighter.level_1_pilot")
        .expect("level-1 pilot row must exist");
    let levels_2_10 = matrix
        .row("class.fighter.levels_2_10")
        .expect("levels-2-10 row must exist");

    assert_ne!(
        level_1.row_id, levels_2_10.row_id,
        "Fighter L1 and L2-10 must not collapse into one row"
    );
    assert_ne!(
        level_1.dimension, levels_2_10.dimension,
        "Fighter L1 and L2-10 must keep distinct progression dimensions"
    );
    // The slice must NOT have weakened the L2-10 row's enforcement of "levels
    // beyond the proven range remain unproven"; the L2-10 row is a separate
    // concern with its own grounding ref, now the more specific SD13-E3 level-7
    // proof surface.
    assert!(
        levels_2_10
            .grounding_ref
            .contains("sd13_fighter_level7_progression"),
        "L2-10 row must keep its own SD13-E3 tranche grounding ref: {}",
        levels_2_10.grounding_ref
    );
}

#[test]
fn matrix_keeps_rogue_partial_after_its_own_recognition_slice() {
    // Rogue was later promoted to Partial/Computed by its own SD13-E3 chassis
    // recognition slice; this Fighter-L1-widening snapshot preserves that.
    let matrix = seeded_sd13_e1_f1_current_truth();
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("Rogue row must exist");
    assert_eq!(
        rogue.support_state,
        SupportState::Partial,
        "Rogue must remain Partial after the Fighter L1 widening"
    );
}

#[test]
fn matrix_keeps_human_race_partial_seam() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let human = matrix
        .row("race.human.pilot_semantics")
        .expect("Human race row must exist");
    assert_eq!(
        human.support_state,
        SupportState::Partial,
        "Human race seam must remain Partial after the Fighter L1 widening"
    );
    assert_eq!(human.evidence_tier, EvidenceTier::Computed);
}
