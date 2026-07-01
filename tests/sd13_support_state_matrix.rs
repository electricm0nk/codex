//! SD13-E1-F1 support-state matrix carrier proof.
//!
//! Proves the seeded SD-13 support-state matrix carries the exact current-truth
//! rows authorized by the SD-13 packet and the live GE-06 evidence, with support
//! state and evidence tier preserved as separate axes. It deliberately asserts
//! only documentary/control-plane truth: no rules computation, no promotion logic,
//! no serialization, and no broader roster support claim.

use codex::rules_core::support_state_matrix::{
    EvidenceTier, MatrixSubjectType, SupportState, SupportStateMatrix, SupportStateRow,
    seeded_sd13_e1_f1_current_truth,
};

/// The exact, ordered set of seeded row ids. The seed must expose these and no
/// others, with Fighter level 1 and Fighter levels 2-10 kept as separate rows.
const EXPECTED_ROW_IDS: [&str; 21] = [
    "race.human.pilot_semantics",
    "race.dwarf.bounded_semantics",
    "race.elf.bounded_semantics",
    "race.gnome.bounded_semantics",
    "race.half_elf.bounded_semantics",
    "race.half_orc.bounded_semantics",
    "race.halfling.bounded_semantics",
    "class.fighter.level_1_pilot",
    "class.fighter.levels_2_10",
    "class.rogue.bounded_progression",
    "class.barbarian.bounded_progression",
    "class.bard.progression_and_spell_burden",
    "class.cleric.progression_and_spell_burden",
    "class.druid.progression_and_spell_burden",
    "class.monk.bounded_progression",
    "class.paladin.hybrid_chassis_and_spell_burden",
    "class.ranger.hybrid_chassis_and_spell_burden",
    "class.sorcerer.progression_and_spell_burden",
    "class.wizard.progression_and_spell_burden",
    "interaction.human_bonus_feat_ability_bonus.pilot_pressure",
    "interaction.non_human_any_class.progression_pressure",
];

fn matrix() -> SupportStateMatrix {
    seeded_sd13_e1_f1_current_truth()
}

fn row<'a>(matrix: &'a SupportStateMatrix, row_id: &str) -> &'a SupportStateRow {
    matrix
        .row(row_id)
        .unwrap_or_else(|| panic!("expected seeded row '{row_id}'"))
}

#[test]
fn seed_contains_exactly_twenty_one_rows() {
    let matrix = matrix();
    assert_eq!(
        matrix.rows.len(),
        21,
        "seed must contain exactly 21 rows, got {}",
        matrix.rows.len()
    );
}

#[test]
fn seed_has_seven_race_twelve_class_two_interaction_rows() {
    let matrix = matrix();
    let count = |subject_type: MatrixSubjectType| {
        matrix
            .rows
            .iter()
            .filter(|r| r.subject_type == subject_type)
            .count()
    };
    assert_eq!(count(MatrixSubjectType::Race), 7, "expected 7 race rows");
    assert_eq!(count(MatrixSubjectType::Class), 12, "expected 12 class rows");
    assert_eq!(
        count(MatrixSubjectType::Interaction),
        2,
        "expected 2 interaction rows"
    );
}

#[test]
fn seed_exposes_exact_row_ids_and_no_extras() {
    let matrix = matrix();

    let actual: Vec<&str> = matrix.rows.iter().map(|r| r.row_id).collect();
    assert_eq!(
        actual.len(),
        EXPECTED_ROW_IDS.len(),
        "row count must match the expected id set"
    );

    for expected in EXPECTED_ROW_IDS {
        assert!(
            matrix.rows.iter().any(|r| r.row_id == expected),
            "missing expected row id '{expected}'"
        );
    }

    for actual_id in &actual {
        assert!(
            EXPECTED_ROW_IDS.contains(actual_id),
            "unexpected row id '{actual_id}' is not in the authorized seed"
        );
    }

    // No duplicate ids may sneak in to inflate breadth truth.
    let mut sorted = actual.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), actual.len(), "row ids must be unique");
}

#[test]
fn human_race_row_is_partial_and_computed() {
    let matrix = matrix();
    let human = row(&matrix, "race.human.pilot_semantics");
    assert_eq!(human.subject_type, MatrixSubjectType::Race);
    assert_eq!(human.subject_id, "race:human");
    assert_eq!(human.support_state, SupportState::Partial);
    assert_eq!(human.evidence_tier, EvidenceTier::Computed);
    assert!(
        !human.grounding_ref.is_empty(),
        "computed Human row must cite grounding evidence"
    );
}

#[test]
fn fighter_level_1_row_is_partial_and_computed() {
    let matrix = matrix();
    let fighter = row(&matrix, "class.fighter.level_1_pilot");
    assert_eq!(fighter.subject_type, MatrixSubjectType::Class);
    assert_eq!(fighter.subject_id, "class:fighter");
    assert_eq!(fighter.support_state, SupportState::Partial);
    assert_eq!(fighter.evidence_tier, EvidenceTier::Computed);
    assert!(
        !fighter.grounding_ref.is_empty(),
        "computed Fighter level-1 row must cite grounding evidence"
    );
}

#[test]
fn fighter_levels_2_10_row_is_partial_and_computed_and_names_what_remains() {
    let matrix = matrix();
    let partial = row(&matrix, "class.fighter.levels_2_10");
    assert_eq!(partial.subject_type, MatrixSubjectType::Class);
    assert_eq!(partial.subject_id, "class:fighter");
    // The SD13-E3 tranche moves the row from Blocked to a bounded Partial posture,
    // but it must never be silently promoted to Supported.
    assert_eq!(partial.support_state, SupportState::Partial);
    assert_ne!(partial.support_state, SupportState::Supported);
    assert_eq!(partial.evidence_tier, EvidenceTier::Computed);
    assert!(
        !partial.blocker_or_lossiness_note.is_empty(),
        "partial Fighter levels-2-10 row must carry a non-empty note on what remains unproven"
    );
    // The note must explicitly name that levels 4-10 remain out of proof after the slice.
    assert!(
        partial.blocker_or_lossiness_note.contains("4-10"),
        "partial Fighter row must name the still-unproven levels 4-10: {}",
        partial.blocker_or_lossiness_note
    );
    assert!(
        partial.grounding_ref.contains("sd13_fighter_level2_level3_progression"),
        "partial Fighter row must cite the SD13-E3 tranche proof surface: {}",
        partial.grounding_ref
    );
}

#[test]
fn rogue_row_is_blocked_and_computed_with_blocker_note() {
    let matrix = matrix();
    let rogue = row(&matrix, "class.rogue.bounded_progression");
    assert_eq!(rogue.subject_type, MatrixSubjectType::Class);
    assert_eq!(rogue.subject_id, "class:rogue");
    assert_eq!(rogue.support_state, SupportState::Blocked);
    assert_eq!(rogue.evidence_tier, EvidenceTier::Computed);
    assert!(
        !rogue.blocker_or_lossiness_note.is_empty(),
        "blocked Rogue row must carry a non-empty blocker note"
    );
    assert!(
        rogue.grounding_ref.contains("ge06_pilot_total_saves"),
        "blocked Rogue row must cite the GE-06 total-save test that claim-blocks it: {}",
        rogue.grounding_ref
    );
}

#[test]
fn fighter_level_1_and_levels_2_10_remain_separate_rows() {
    let matrix = matrix();
    let level_1 = row(&matrix, "class.fighter.level_1_pilot");
    let levels_2_10 = row(&matrix, "class.fighter.levels_2_10");
    assert_ne!(
        level_1.row_id, levels_2_10.row_id,
        "Fighter level 1 and levels 2-10 must not collapse into one row"
    );
    // After the SD13-E3 tranche both rows are Partial/Computed, but they must remain
    // distinct rows describing distinct bounded progression dimensions: the level-1
    // pilot surface versus the levels-2-3 milestone proof with levels 4-10 unproven.
    assert_ne!(
        level_1.dimension, levels_2_10.dimension,
        "Fighter level-1 and levels-2-10 rows must keep distinct progression dimensions"
    );
}

#[test]
fn every_non_human_race_row_is_unverified_and_observed() {
    let matrix = matrix();
    let non_human_races: Vec<&SupportStateRow> = matrix
        .rows
        .iter()
        .filter(|r| r.subject_type == MatrixSubjectType::Race)
        .filter(|r| r.row_id != "race.human.pilot_semantics")
        .collect();

    assert_eq!(
        non_human_races.len(),
        6,
        "there must be 6 non-Human race rows"
    );
    for race in non_human_races {
        assert_eq!(
            race.support_state,
            SupportState::Unverified,
            "non-Human race row '{}' must be Unverified",
            race.row_id
        );
        assert_eq!(
            race.evidence_tier,
            EvidenceTier::Observed,
            "non-Human race row '{}' must be Observed",
            race.row_id
        );
    }
}

#[test]
fn every_non_fighter_non_rogue_class_row_is_unverified_and_observed() {
    let matrix = matrix();
    let other_classes: Vec<&SupportStateRow> = matrix
        .rows
        .iter()
        .filter(|r| r.subject_type == MatrixSubjectType::Class)
        .filter(|r| r.subject_id != "class:fighter" && r.subject_id != "class:rogue")
        .collect();

    assert_eq!(
        other_classes.len(),
        9,
        "there must be 9 non-Fighter/non-Rogue class rows"
    );
    for class in other_classes {
        assert_eq!(
            class.support_state,
            SupportState::Unverified,
            "class row '{}' must be Unverified",
            class.row_id
        );
        assert_eq!(
            class.evidence_tier,
            EvidenceTier::Observed,
            "class row '{}' must be Observed",
            class.row_id
        );
    }
}

#[test]
fn human_race_row_makes_named_pilot_seam_explicit() {
    let matrix = matrix();
    let human = row(&matrix, "race.human.pilot_semantics");
    // The Human race row must ground to the live compute surface that now makes the
    // race seam explicit, not merely to chosen-input fixture text.
    assert!(
        human.grounding_ref.contains("pilot_compute"),
        "Human race row must ground to the compute surface that makes the seam explicit: {}",
        human.grounding_ref
    );
    // The dimension must name the two grounded Human pilot pressures explicitly so the
    // seam is legible rather than incidental.
    assert!(
        human.dimension.contains("ability-bonus") && human.dimension.contains("bonus-feat"),
        "Human race row dimension must name the ability-bonus and bonus-feat seam: {}",
        human.dimension
    );
    // The broader Human racial burden must stay visibly unverified.
    assert!(
        !human.blocker_or_lossiness_note.is_empty(),
        "Human race row must keep a non-empty note about the still-unverified Human burden"
    );
}

#[test]
fn human_interaction_row_names_both_pressures_and_stays_distinct_from_race() {
    let matrix = matrix();
    let interaction = row(
        &matrix,
        "interaction.human_bonus_feat_ability_bonus.pilot_pressure",
    );
    let human = row(&matrix, "race.human.pilot_semantics");
    // The named interaction seam must stay a distinct subject from the Human race row.
    assert_eq!(interaction.subject_type, MatrixSubjectType::Interaction);
    assert_ne!(
        interaction.subject_id, human.subject_id,
        "named interaction truth must stay distinct from the Human race row"
    );
    // The dimension itself must explicitly name both Human pressures, not leave them
    // implied only by the blocker note.
    assert!(
        interaction.dimension.contains("bonus-feat") && interaction.dimension.contains("ability-bonus"),
        "interaction row dimension must explicitly name both named Human pressures: {}",
        interaction.dimension
    );
}

#[test]
fn human_interaction_row_is_partial_and_computed() {
    let matrix = matrix();
    let interaction = row(
        &matrix,
        "interaction.human_bonus_feat_ability_bonus.pilot_pressure",
    );
    assert_eq!(interaction.subject_type, MatrixSubjectType::Interaction);
    assert_eq!(
        interaction.subject_id,
        "interaction:human-bonus-feat-ability-bonus"
    );
    assert_eq!(interaction.support_state, SupportState::Partial);
    assert_eq!(interaction.evidence_tier, EvidenceTier::Computed);
}

#[test]
fn non_human_interaction_row_is_unverified_and_observed() {
    let matrix = matrix();
    let interaction = row(
        &matrix,
        "interaction.non_human_any_class.progression_pressure",
    );
    assert_eq!(interaction.subject_type, MatrixSubjectType::Interaction);
    assert_eq!(interaction.support_state, SupportState::Unverified);
    assert_eq!(interaction.evidence_tier, EvidenceTier::Observed);
}

#[test]
fn seed_contains_no_supported_rows() {
    let matrix = matrix();
    assert!(
        !matrix
            .rows
            .iter()
            .any(|r| r.support_state == SupportState::Supported),
        "the initial seed must not silently promote any row to Supported"
    );
}

#[test]
fn seed_contains_no_lossy_rows() {
    let matrix = matrix();
    assert!(
        !matrix
            .rows
            .iter()
            .any(|r| r.support_state == SupportState::Lossy),
        "the initial seed must not contain any Lossy rows"
    );
}

#[test]
fn only_pilot_grounded_rows_rise_above_observed() {
    let matrix = matrix();
    let above_observed: Vec<&str> = matrix
        .rows
        .iter()
        .filter(|r| r.evidence_tier != EvidenceTier::Observed)
        .map(|r| r.row_id)
        .collect();

    let expected_above_observed = [
        "race.human.pilot_semantics",
        "class.fighter.level_1_pilot",
        "class.fighter.levels_2_10",
        "class.rogue.bounded_progression",
        "interaction.human_bonus_feat_ability_bonus.pilot_pressure",
    ];

    assert_eq!(
        above_observed.len(),
        expected_above_observed.len(),
        "only the pilot-grounded rows may rise above Observed, got {above_observed:?}"
    );
    for id in expected_above_observed {
        assert!(
            above_observed.contains(&id),
            "expected row '{id}' to rise above Observed"
        );
    }
}

#[test]
fn every_blocked_row_carries_a_non_empty_blocker_note() {
    let matrix = matrix();
    for blocked in matrix
        .rows
        .iter()
        .filter(|r| r.support_state == SupportState::Blocked)
    {
        assert!(
            !blocked.blocker_or_lossiness_note.is_empty(),
            "blocked row '{}' must carry a non-empty blocker note",
            blocked.row_id
        );
    }
}

#[test]
fn lookup_helper_retrieves_anchor_rows_by_id() {
    let matrix = matrix();
    for id in [
        "race.human.pilot_semantics",
        "class.fighter.level_1_pilot",
        "class.rogue.bounded_progression",
        "interaction.human_bonus_feat_ability_bonus.pilot_pressure",
    ] {
        let found = matrix.row(id).unwrap_or_else(|| panic!("lookup must find '{id}'"));
        assert_eq!(found.row_id, id, "lookup must return the requested row");
    }

    assert!(
        matrix.row("class.nonexistent.row").is_none(),
        "lookup must return None for an unknown row id"
    );
}

#[test]
fn every_row_carries_grounding_and_next_uplift() {
    let matrix = matrix();
    for r in &matrix.rows {
        assert!(
            !r.grounding_ref.is_empty(),
            "row '{}' must cite a grounding reference",
            r.row_id
        );
        assert!(
            !r.dimension.is_empty(),
            "row '{}' must name a semantic/progression dimension",
            r.row_id
        );
        assert!(
            !r.next_required_uplift.is_empty(),
            "row '{}' must name its next required uplift",
            r.row_id
        );
    }
}
