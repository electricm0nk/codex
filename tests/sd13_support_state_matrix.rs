//! SD13-E1-F1 — Machine-usable support-state matrix and seeded current truth.
//!
//! Pins the smallest typed SD-13 control-plane surface: a support-state matrix
//! whose rows keep support state and evidence tier as separate axes, limited to
//! the three SD-13 row types (`race`, `class`, `interaction`). These tests pin
//! the deterministic current-truth seed exactly as grounded by the SD-13 packet
//! and the existing GE-06 repo evidence. They do not assert race semantics,
//! class progression, spell burden, claim composition, or any broader breadth.

use codex::oracle_validation::support_state_matrix::{
    seeded_sd13_e1_f1_current_truth, EvidenceTier, MatrixSubjectType, SupportState,
    SupportStateMatrix,
};

/// Every seeded row id, in roster order: 7 race, 12 class, 2 interaction.
const EXPECTED_ROW_IDS: [&str; 21] = [
    // race rows
    "race.human.pilot_semantics",
    "race.dwarf.bounded_semantics",
    "race.elf.bounded_semantics",
    "race.gnome.bounded_semantics",
    "race.half_elf.bounded_semantics",
    "race.half_orc.bounded_semantics",
    "race.halfling.bounded_semantics",
    // class rows
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
    // interaction rows
    "interaction.human_bonus_feat_ability_bonus.pilot_pressure",
    "interaction.non_human_any_class.progression_pressure",
];

fn seed() -> SupportStateMatrix {
    seeded_sd13_e1_f1_current_truth()
}

#[test]
fn seed_returns_exactly_twenty_one_rows_split_seven_twelve_two() {
    let matrix = seed();
    assert_eq!(matrix.rows.len(), 21, "seed must hold exactly 21 rows");

    let races = matrix
        .rows
        .iter()
        .filter(|r| r.subject_type == MatrixSubjectType::Race)
        .count();
    let classes = matrix
        .rows
        .iter()
        .filter(|r| r.subject_type == MatrixSubjectType::Class)
        .count();
    let interactions = matrix
        .rows
        .iter()
        .filter(|r| r.subject_type == MatrixSubjectType::Interaction)
        .count();

    assert_eq!(races, 7, "expected 7 race rows");
    assert_eq!(classes, 12, "expected 12 class rows");
    assert_eq!(interactions, 2, "expected 2 interaction rows");
}

#[test]
fn seed_exposes_exact_row_ids_and_no_extras() {
    let matrix = seed();

    let actual: Vec<&str> = matrix.rows.iter().map(|r| r.row_id.as_str()).collect();
    assert_eq!(
        actual, EXPECTED_ROW_IDS,
        "seed row ids must match the SD-13 handoff exactly, in roster order"
    );

    // No duplicate row ids.
    let mut sorted = actual.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), actual.len(), "row ids must be unique");
}

#[test]
fn human_pilot_race_row_is_partial_and_computed() {
    let matrix = seed();
    let row = matrix
        .row("race.human.pilot_semantics")
        .expect("Human pilot race row must exist");

    assert_eq!(row.subject_type, MatrixSubjectType::Race);
    assert_eq!(row.subject_id, "race:human");
    assert_eq!(row.support_state, SupportState::Partial);
    assert_eq!(row.evidence_tier, EvidenceTier::Computed);
    assert!(
        !row.grounding_ref.trim().is_empty(),
        "grounding_ref must cite real evidence"
    );
}

#[test]
fn fighter_level_1_row_is_partial_and_computed() {
    let matrix = seed();
    let row = matrix
        .row("class.fighter.level_1_pilot")
        .expect("Fighter level-1 row must exist");

    assert_eq!(row.subject_type, MatrixSubjectType::Class);
    assert_eq!(row.subject_id, "class:fighter");
    assert_eq!(row.support_state, SupportState::Partial);
    assert_eq!(row.evidence_tier, EvidenceTier::Computed);
}

#[test]
fn fighter_levels_2_10_row_is_blocked_computed_with_nonempty_blocker_note() {
    let matrix = seed();
    let row = matrix
        .row("class.fighter.levels_2_10")
        .expect("Fighter levels-2-10 row must exist");

    assert_eq!(row.subject_type, MatrixSubjectType::Class);
    assert_eq!(row.subject_id, "class:fighter");
    assert_eq!(row.support_state, SupportState::Blocked);
    assert_eq!(row.evidence_tier, EvidenceTier::Computed);

    let note = row
        .blocker_or_lossiness_note
        .as_deref()
        .expect("Blocked row must carry a blocker note");
    assert!(!note.trim().is_empty(), "blocker note must be non-empty");
}

#[test]
fn rogue_row_is_blocked_computed_with_nonempty_blocker_note() {
    let matrix = seed();
    let row = matrix
        .row("class.rogue.bounded_progression")
        .expect("Rogue row must exist");

    assert_eq!(row.subject_type, MatrixSubjectType::Class);
    assert_eq!(row.subject_id, "class:rogue");
    assert_eq!(row.support_state, SupportState::Blocked);
    assert_eq!(row.evidence_tier, EvidenceTier::Computed);

    let note = row
        .blocker_or_lossiness_note
        .as_deref()
        .expect("Blocked row must carry a blocker note");
    assert!(!note.trim().is_empty(), "blocker note must be non-empty");
}

#[test]
fn fighter_level_1_and_levels_2_10_are_not_collapsed() {
    let matrix = seed();
    let level_1 = matrix.row("class.fighter.level_1_pilot").unwrap();
    let levels_2_10 = matrix.row("class.fighter.levels_2_10").unwrap();

    assert_ne!(
        level_1.row_id, levels_2_10.row_id,
        "Fighter level 1 and levels 2-10 must remain distinct rows"
    );
    assert_ne!(
        level_1.support_state, levels_2_10.support_state,
        "the two Fighter rows must carry distinct support states"
    );
}

#[test]
fn every_non_human_race_row_is_unverified_and_observed() {
    let matrix = seed();
    for row in matrix
        .rows
        .iter()
        .filter(|r| r.subject_type == MatrixSubjectType::Race)
        .filter(|r| r.subject_id != "race:human")
    {
        assert_eq!(
            row.support_state,
            SupportState::Unverified,
            "non-Human race row {} must be Unverified",
            row.row_id
        );
        assert_eq!(
            row.evidence_tier,
            EvidenceTier::Observed,
            "non-Human race row {} must be Observed",
            row.row_id
        );
    }
}

#[test]
fn every_non_fighter_non_rogue_class_row_is_unverified_and_observed() {
    let matrix = seed();
    for row in matrix
        .rows
        .iter()
        .filter(|r| r.subject_type == MatrixSubjectType::Class)
        .filter(|r| {
            r.row_id != "class.fighter.level_1_pilot"
                && r.row_id != "class.fighter.levels_2_10"
                && r.row_id != "class.rogue.bounded_progression"
        })
    {
        assert_eq!(
            row.support_state,
            SupportState::Unverified,
            "class row {} must be Unverified",
            row.row_id
        );
        assert_eq!(
            row.evidence_tier,
            EvidenceTier::Observed,
            "class row {} must be Observed",
            row.row_id
        );
    }
}

#[test]
fn human_interaction_row_is_partial_computed_and_non_human_is_unverified_observed() {
    let matrix = seed();

    let human = matrix
        .row("interaction.human_bonus_feat_ability_bonus.pilot_pressure")
        .expect("Human interaction row must exist");
    assert_eq!(human.subject_type, MatrixSubjectType::Interaction);
    assert_eq!(human.support_state, SupportState::Partial);
    assert_eq!(human.evidence_tier, EvidenceTier::Computed);

    let non_human = matrix
        .row("interaction.non_human_any_class.progression_pressure")
        .expect("non-Human interaction row must exist");
    assert_eq!(non_human.subject_type, MatrixSubjectType::Interaction);
    assert_eq!(non_human.support_state, SupportState::Unverified);
    assert_eq!(non_human.evidence_tier, EvidenceTier::Observed);
}

#[test]
fn seed_has_no_supported_rows() {
    let matrix = seed();
    assert!(
        !matrix
            .rows
            .iter()
            .any(|r| r.support_state == SupportState::Supported),
        "the initial seed must not silently promote any row to Supported"
    );
}

#[test]
fn only_pilot_truth_rows_rise_above_observed() {
    let matrix = seed();
    let allowed_above_observed = [
        "race.human.pilot_semantics",
        "class.fighter.level_1_pilot",
        "class.fighter.levels_2_10",
        "class.rogue.bounded_progression",
        "interaction.human_bonus_feat_ability_bonus.pilot_pressure",
    ];

    for row in &matrix.rows {
        if row.evidence_tier != EvidenceTier::Observed {
            assert!(
                allowed_above_observed.contains(&row.row_id.as_str()),
                "row {} must not rise above Observed",
                row.row_id
            );
        }
    }
}

#[test]
fn every_blocked_row_carries_a_nonempty_blocker_note() {
    let matrix = seed();
    for row in matrix
        .rows
        .iter()
        .filter(|r| r.support_state == SupportState::Blocked)
    {
        let note = row
            .blocker_or_lossiness_note
            .as_deref()
            .unwrap_or_else(|| panic!("Blocked row {} must carry a blocker note", row.row_id));
        assert!(
            !note.trim().is_empty(),
            "Blocked row {} blocker note must be non-empty",
            row.row_id
        );
    }
}

#[test]
fn every_row_carries_dimension_grounding_and_next_uplift() {
    let matrix = seed();
    for row in &matrix.rows {
        assert!(
            !row.dimension.trim().is_empty(),
            "row {} must declare a dimension",
            row.row_id
        );
        assert!(
            !row.grounding_ref.trim().is_empty(),
            "row {} must cite a grounding_ref",
            row.row_id
        );
        assert!(
            !row.next_required_uplift.trim().is_empty(),
            "row {} must declare a next_required_uplift",
            row.row_id
        );
    }
}

#[test]
fn lookup_helper_retrieves_key_rows_and_misses_unknown_ids() {
    let matrix = seed();

    assert!(matrix.row("race.human.pilot_semantics").is_some());
    assert!(matrix.row("class.fighter.level_1_pilot").is_some());
    assert!(matrix.row("class.rogue.bounded_progression").is_some());
    assert!(matrix
        .row("interaction.human_bonus_feat_ability_bonus.pilot_pressure")
        .is_some());

    assert!(
        matrix.row("class.fighter.level_99_unreal").is_none(),
        "lookup must return None for an unknown row id"
    );
}
