//! SD13-E1-F1 support-state matrix carrier proof.
//!
//! Proves the seeded SD-13 support-state matrix carries the exact current-truth
//! rows authorized by the SD-13 packet and the live GE-06 evidence, with support
//! state and evidence tier preserved as separate axes. It deliberately asserts
//! only documentary/control-plane truth: no rules computation, no promotion logic,
//! no serialization, and no broader roster support claim.

use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, MatrixSubjectType, SupportState, SupportStateMatrix,
    SupportStateRow, seeded_sd13_e1_f1_current_truth,
};

/// The exact, ordered set of seeded row ids. The seed must expose these and no
/// others, with Fighter level 1 and Fighter levels 2-10 kept as separate rows.
///
/// SD-19 widening: `school.abjuration.spell_reachability` is the first
/// loop-routed §2.4 row landed on top of the fixed SD-13 21-row baseline;
/// `school.conjuration.spell_reachability` is the second;
/// `school.divination.spell_reachability` is the third;
/// `school.enchantment.spell_reachability` is the fourth;
/// `school.evocation.spell_reachability` is the fifth;
/// `school.illusion.spell_reachability` is the sixth;
/// `school.necromancy.spell_reachability` is the seventh;
/// `school.transmutation.spell_reachability` is the eighth;
/// `school.universal.spell_reachability` is the ninth, closing the full
/// §2.4 spell-school sweep; `equipment.arms_armor.equipment_reachability`
/// is the first loop-routed §2.5 row; later SD-19 cycles append one
/// equipment row each, growing this list, never rewriting an existing
/// SD-13 row's identity.
const EXPECTED_ROW_IDS: [&str; 31] = [
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
    "school.abjuration.spell_reachability",
    "school.conjuration.spell_reachability",
    "school.divination.spell_reachability",
    "school.enchantment.spell_reachability",
    "school.evocation.spell_reachability",
    "school.illusion.spell_reachability",
    "school.necromancy.spell_reachability",
    "school.transmutation.spell_reachability",
    "school.universal.spell_reachability",
    "equipment.arms_armor.equipment_reachability",
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
fn seed_contains_exactly_thirty_one_rows() {
    let matrix = matrix();
    assert_eq!(
        matrix.rows.len(),
        31,
        "seed must contain exactly 31 rows, got {}",
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
    assert_eq!(
        count(MatrixSubjectType::Class),
        12,
        "expected 12 class rows"
    );
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
    // The SD13-E5 hit-point slice grounds level-1 hit points: the row must cite
    // both the mandatory-milestone classification proof and the hit-point
    // baseline proof, and the note must carry the hit-point milestone on the
    // proven side while keeping the favored-class residue named as unproven.
    assert!(
        fighter
            .grounding_ref
            .contains("sd13_fighter_level1_mandatory_milestone_classification"),
        "Fighter level-1 row must keep citing the mandatory-milestone proof surface: {}",
        fighter.grounding_ref
    );
    assert!(
        fighter
            .grounding_ref
            .contains("sd13_fighter_level1_hit_point_baseline"),
        "Fighter level-1 row must cite the SD13-E5 hit-point baseline proof surface: {}",
        fighter.grounding_ref
    );
    assert!(
        fighter
            .blocker_or_lossiness_note
            .contains("level-1 hit points"),
        "Fighter level-1 row note must name the grounded level-1 hit-point milestone: {}",
        fighter.blocker_or_lossiness_note
    );
    assert!(
        fighter.blocker_or_lossiness_note.contains("favored-class"),
        "Fighter level-1 row note must keep the unproven favored-class hit-point residue named: {}",
        fighter.blocker_or_lossiness_note
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
    assert_eq!(partial.evidence_tier, EvidenceTier::Computed);
    assert!(
        !partial.blocker_or_lossiness_note.is_empty(),
        "partial Fighter levels-2-10 row must carry a non-empty note on what remains unproven"
    );
    // After the SD13-E5 level-9/level-10 widening slice the whole 2-10 level range
    // is proven; the note must explicitly name the honest remaining class-feature
    // burdens instead: the Weapon Training damage-roll half and Bravery.
    assert!(
        partial.blocker_or_lossiness_note.contains("damage"),
        "partial Fighter row must name the still-unproven Weapon Training damage-roll half: {}",
        partial.blocker_or_lossiness_note
    );
    assert!(
        partial.blocker_or_lossiness_note.contains("Bravery"),
        "partial Fighter row must name the still-unproven Bravery milestone: {}",
        partial.blocker_or_lossiness_note
    );
    assert!(
        partial
            .grounding_ref
            .contains("sd13_fighter_level9_level10_progression"),
        "partial Fighter row must cite the SD13-E5 level-9/level-10 proof surface: {}",
        partial.grounding_ref
    );
}

#[test]
fn rogue_row_is_partial_and_computed_with_blocker_note() {
    // The SD13-E3 Rogue chassis recognition slice promoted the row from
    // Blocked to Partial; the SD13-E5 slice grounds the fourth and final
    // named pillar (trapfinding), leaving the row Partial with the
    // check-execution / rogue-talent / integration remainder named.
    let matrix = matrix();
    let rogue = row(&matrix, "class.rogue.bounded_progression");
    assert_eq!(rogue.subject_type, MatrixSubjectType::Class);
    assert_eq!(rogue.subject_id, "class:rogue");
    assert_eq!(rogue.support_state, SupportState::Partial);
    assert_eq!(rogue.evidence_tier, EvidenceTier::Computed);
    assert!(
        !rogue.blocker_or_lossiness_note.is_empty(),
        "partial Rogue row must carry a non-empty blocker note"
    );
    assert!(
        rogue.blocker_or_lossiness_note.contains("trapfinding"),
        "partial Rogue row note must name the now-grounded trapfinding pillar: {}",
        rogue.blocker_or_lossiness_note
    );
    assert!(
        !rogue
            .blocker_or_lossiness_note
            .contains("only trapfinding remains unproven"),
        "partial Rogue row note must not repeat the stale pre-E5 trapfinding-unproven claim: {}",
        rogue.blocker_or_lossiness_note
    );
    assert!(
        rogue
            .grounding_ref
            .contains("sd13_rogue_level1_chassis_baseline"),
        "partial Rogue row must cite the live SD13-E3/E5 recognition test surface: {}",
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
fn every_non_human_race_row_now_carries_runtime_evidence() {
    // Dwarf, Elf, Gnome, Half-Elf, Half-Orc, and Halfling were all promoted to
    // Partial/Computed by their own SD13-E2 recognition slices. With this
    // final race slice landed, no core race row is a pure roster-scope
    // placeholder any longer.
    let matrix = matrix();
    let recognized = [
        "race.dwarf.bounded_semantics",
        "race.elf.bounded_semantics",
        "race.gnome.bounded_semantics",
        "race.half_elf.bounded_semantics",
        "race.half_orc.bounded_semantics",
        "race.halfling.bounded_semantics",
    ];
    let non_human_races: Vec<&SupportStateRow> = matrix
        .rows
        .iter()
        .filter(|r| r.subject_type == MatrixSubjectType::Race)
        .filter(|r| r.row_id != "race.human.pilot_semantics")
        .collect();

    assert_eq!(
        non_human_races.len(),
        recognized.len(),
        "there must be exactly {} non-Human race rows",
        recognized.len()
    );
    for race in non_human_races {
        assert!(
            recognized.contains(&race.row_id),
            "unexpected non-Human race row '{}'",
            race.row_id
        );
        assert_eq!(
            race.support_state,
            SupportState::Partial,
            "non-Human race row '{}' must be Partial",
            race.row_id
        );
        assert_eq!(
            race.evidence_tier,
            EvidenceTier::Computed,
            "non-Human race row '{}' must be Computed",
            race.row_id
        );
    }
}

#[test]
fn paladin_hybrid_row_is_partial_and_computed_with_named_burdens() {
    // SD13-E3-F6 moved the Paladin hybrid row off the pure Unverified/Observed
    // placeholder to a blocked computed posture; the SD13-E4 slice grounded
    // Smite Evil for real, and the SD13-E5 level-gate slice grounds lay on
    // hands / divine grace / mercy as correct PF1 CRB level-gate absences and
    // promotes the row from Blocked to Partial. The hybrid chassis pair and
    // the partial-caster spell burden stay named and unproven — never
    // Supported.
    let matrix = matrix();
    let hybrid = row(&matrix, "class.paladin.hybrid_chassis_and_spell_burden");
    assert_eq!(hybrid.subject_type, MatrixSubjectType::Class);
    assert_eq!(hybrid.subject_id, "class:paladin");
    assert_eq!(hybrid.support_state, SupportState::Partial);
    assert_ne!(hybrid.support_state, SupportState::Blocked);
    assert_ne!(hybrid.support_state, SupportState::Supported);
    assert_eq!(hybrid.evidence_tier, EvidenceTier::Computed);
    assert!(
        hybrid
            .grounding_ref
            .contains("sd13_hybrid_level1_chassis_baseline"),
        "paladin hybrid row must cite the SD13-F6 proof surface: {}",
        hybrid.grounding_ref
    );
    assert!(
        !hybrid.blocker_or_lossiness_note.is_empty(),
        "partial paladin hybrid row must carry a non-empty lossiness note"
    );
    assert!(
        hybrid.blocker_or_lossiness_note.contains("spell"),
        "paladin hybrid row note must name the later spell burden: {}",
        hybrid.blocker_or_lossiness_note
    );
    for token in ["smite", "lay on hands", "divine grace", "mercy"] {
        assert!(
            hybrid.blocker_or_lossiness_note.contains(token),
            "paladin hybrid row note must name the '{token}' burden: {}",
            hybrid.blocker_or_lossiness_note
        );
    }
}

#[test]
fn ranger_hybrid_row_is_partial_and_computed_with_named_burdens() {
    // The SD13-E3 Ranger decomposition slice grounds Track for real and
    // promotes this row from Blocked to Partial; the SD13-E5 slice further
    // grounds the Favored Enemy flat surface. Combat style, the favored-enemy
    // conditional-application engine, and the later spell burden remain named
    // and unproven.
    let matrix = matrix();
    let hybrid = row(&matrix, "class.ranger.hybrid_chassis_and_spell_burden");
    assert_eq!(hybrid.subject_type, MatrixSubjectType::Class);
    assert_eq!(hybrid.subject_id, "class:ranger");
    assert_eq!(hybrid.support_state, SupportState::Partial);
    assert_ne!(hybrid.support_state, SupportState::Blocked);
    assert_ne!(hybrid.support_state, SupportState::Supported);
    assert_eq!(hybrid.evidence_tier, EvidenceTier::Computed);
    assert!(
        hybrid
            .grounding_ref
            .contains("sd13_hybrid_level1_chassis_baseline"),
        "ranger hybrid row must still cite the SD13-F6 proof surface: {}",
        hybrid.grounding_ref
    );
    assert!(
        hybrid
            .grounding_ref
            .contains("sd13_ranger_level1_chassis_and_class_feature_separation"),
        "ranger hybrid row must cite its own decomposition proof surface: {}",
        hybrid.grounding_ref
    );
    assert!(
        !hybrid.blocker_or_lossiness_note.is_empty(),
        "partial ranger hybrid row must carry a non-empty note"
    );
    assert!(
        hybrid
            .blocker_or_lossiness_note
            .contains("partial_caster.spell_level_access"),
        "ranger hybrid row note must name the grounded partial-caster pair (the original \
         SD13-E4 wholesale deferral was superseded by the further SD13-E5 slice): {}",
        hybrid.blocker_or_lossiness_note
    );
    // Combat style stays a named, unproven burden; the favored-enemy flat
    // surface is grounded but its conditional-application engine stays named
    // and unproven.
    for token in ["favored enemy", "combat style", "conditional-application"] {
        assert!(
            hybrid.blocker_or_lossiness_note.contains(token),
            "ranger hybrid row note must name the '{token}' burden truth: {}",
            hybrid.blocker_or_lossiness_note
        );
    }
    assert!(
        hybrid.blocker_or_lossiness_note.contains("grounds")
            || hybrid.blocker_or_lossiness_note.contains("grounded"),
        "ranger hybrid row note must record the grounded Track / favored-enemy flat surface: {}",
        hybrid.blocker_or_lossiness_note
    );
}

#[test]
fn every_core_class_row_now_carries_runtime_evidence() {
    // After SD13-E3-F6, SD13-E4-F7, SD13-E4-R3, SD13-E4 (Cleric, Druid), and the
    // SD13-E3 Barbarian/Monk martial-chassis slices, all twelve core class rows
    // carry runtime evidence. No core class row is a pure roster-scope
    // placeholder any longer, though several stay Blocked or Partial rather than
    // Supported.
    let matrix = matrix();
    let proven_subjects = [
        "class:fighter",
        "class:rogue",
        "class:barbarian",
        "class:paladin",
        "class:ranger",
        "class:sorcerer",
        "class:bard",
        "class:wizard",
        "class:cleric",
        "class:druid",
        "class:monk",
    ];
    let class_rows: Vec<&SupportStateRow> = matrix
        .rows
        .iter()
        .filter(|r| r.subject_type == MatrixSubjectType::Class)
        .collect();

    // Twelve class rows total: eleven distinct class subjects, with Fighter alone
    // carrying two rows (level-1 pilot and levels-2-10).
    assert_eq!(class_rows.len(), 12, "there must be exactly 12 core class rows");
    for class in class_rows {
        assert!(
            proven_subjects.contains(&class.subject_id),
            "unexpected class row '{}' with subject_id '{}'",
            class.row_id,
            class.subject_id
        );
        assert_ne!(
            class.evidence_tier,
            EvidenceTier::Observed,
            "class row '{}' must carry runtime evidence, not stay a roster-scope placeholder",
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
    // The row must also cite the SD13-E6-F3a trait-bundle proof surface: it is the
    // live, re-runnable evidence that the four trait dimensions are classified.
    assert!(
        human.grounding_ref.contains("sd13_human_full_trait_bundle"),
        "Human race row must also ground on the trait-bundle proof surface: {}",
        human.grounding_ref
    );
    // The dimension must name the two grounded Human pilot pressures explicitly so the
    // seam is legible rather than incidental.
    assert!(
        human.dimension.contains("ability-bonus") && human.dimension.contains("bonus-feat"),
        "Human race row dimension must name the ability-bonus and bonus-feat seam: {}",
        human.dimension
    );
    // The note must reflect the classified SD13-E6-F3a trait bundle, not the
    // pre-F3a claim that size/speed/senses/extra-skill-ranks are still unverified.
    let note = human.blocker_or_lossiness_note;
    assert!(
        !note.is_empty(),
        "Human race row must keep a non-empty note about the remaining Human burden"
    );
    assert!(
        note.contains("classified explicitly"),
        "Human race row note must record that the trait bundle is classified explicitly: {note}"
    );
    assert!(
        !note.contains("are still unverified"),
        "Human race row note must no longer carry the pre-F3a 'still unverified' trait-bundle claim: {note}"
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
        interaction.dimension.contains("bonus-feat")
            && interaction.dimension.contains("ability-bonus"),
        "interaction row dimension must explicitly name both named Human pressures: {}",
        interaction.dimension
    );
}

#[test]
fn human_interaction_row_is_partial_and_product_visible() {
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
    // SD18-PRELOOP: bumped from Partial/Computed to
    // Partial/Product-visible. The consumer-side composer
    // (rules_core::composed_input) accepts a chosen-state
    // CharacterInput plus a corpus-side SourcePackageContent
    // and produces a ComposedCharacterInput that reaches
    // pilot_compute::compute_pilot_base_chassis without panic;
    // tests/sd18_preloop_consumer_compose.rs proves the
    // end-to-end path against a synthetic Core Rulebook PCC.
    assert_eq!(interaction.support_state, SupportState::Partial);
    assert_eq!(interaction.evidence_tier, EvidenceTier::ProductVisible);
    assert!(
        interaction
            .blocker_or_lossiness_note
            .contains("composed_input"),
        "the row's blocker/lossiness note must name the composer that grounds it: {}",
        interaction.blocker_or_lossiness_note
    );
    assert!(
        interaction
            .blocker_or_lossiness_note
            .contains("sd18_preloop_consumer_compose"),
        "the row's blocker/lossiness note must name the SD18-PRELOOP test that grounds it: {}",
        interaction.blocker_or_lossiness_note
    );
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
        "class.paladin.hybrid_chassis_and_spell_burden",
        "class.ranger.hybrid_chassis_and_spell_burden",
        "class.sorcerer.progression_and_spell_burden",
        "class.bard.progression_and_spell_burden",
        "class.wizard.progression_and_spell_burden",
        "class.cleric.progression_and_spell_burden",
        "class.druid.progression_and_spell_burden",
        "class.monk.bounded_progression",
        "interaction.human_bonus_feat_ability_bonus.pilot_pressure",
        "school.abjuration.spell_reachability",
        "school.conjuration.spell_reachability",
        "school.divination.spell_reachability",
        "school.enchantment.spell_reachability",
        "school.evocation.spell_reachability",
        "school.illusion.spell_reachability",
        "school.necromancy.spell_reachability",
        "school.transmutation.spell_reachability",
        "school.universal.spell_reachability",
        "equipment.arms_armor.equipment_reachability",
    ];

    assert_eq!(
        above_observed.len(),
        expected_above_observed.len(),
        "only the pilot-grounded, hybrid-baseline, Barbarian martial-baseline, spell-baseline, \
         and SD-19 loop-routed school/equipment rows may rise above Observed, got {above_observed:?}"
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
        let found = matrix
            .row(id)
            .unwrap_or_else(|| panic!("lookup must find '{id}'"));
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

#[test]
fn stale_generic_uplift_pointers_are_reconciled() {
    // The generic uplift pointers seeded before the tranche 2.6 closeout named
    // slices that have since landed WITHOUT covering these rows ("SD13-E3
    // martial progression slice" landed as the Barbarian baseline; "SD13-E4
    // spellcasting slice" landed as the arcane Sorcerer/Bard/Wizard baselines;
    // "SD13-E2 / SD13-E3 coupling" landed without generalizing the interaction
    // model). Each remaining unproven row must point at a subject-specific
    // uplift so the debt surface stays actionable instead of naming completed
    // slices.
    let matrix = matrix();
    let expectations = [
        ("class.rogue.bounded_progression", "Rogue"),
        ("class.monk.bounded_progression", "Monk"),
        ("class.cleric.progression_and_spell_burden", "Cleric"),
        ("class.druid.progression_and_spell_burden", "Druid"),
        (
            "interaction.human_bonus_feat_ability_bonus.pilot_pressure",
            "interaction",
        ),
    ];
    for (row_id, subject_token) in expectations {
        let r = row(&matrix, row_id);
        let uplift = r.next_required_uplift;
        assert!(
            uplift.contains(subject_token),
            "row '{row_id}' next_required_uplift must name its own subject ('{subject_token}'): {uplift}"
        );
        for stale in [
            "SD13-E3 martial progression slice",
            "SD13-E4 spellcasting slice",
            "SD13-E2 / SD13-E3 coupling",
        ] {
            assert_ne!(
                uplift.trim(),
                stale,
                "row '{row_id}' next_required_uplift must no longer be the generic '{stale}' pointer"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// SD13-E7-F13 evidence-freshness / breadth-claim audit truth.
//
// The carrier is the authority surface for freshness truth. This first honest
// slice records no calendar timestamp or SLA. It carries only the conservative
// distinction the seeded truth can actually prove per row: whether a row is
// anchored to a live, re-runnable proof surface it could be refreshed from, or
// whether it rests only on bounded roster-scope naming with no runtime evidence
// yet. Neither posture asserts a row is currently fresh; the whole surface stays
// refresh-required until a later slice records real refresh checkpoints.
// ---------------------------------------------------------------------------

/// The rows anchored to a live, re-runnable proof surface. These are exactly the
/// pilot-grounded, hybrid-baseline, Barbarian martial-baseline, spell-baseline,
/// and SD-19 loop-routed school/equipment rows that rise above `Observed` evidence.
const EXPECTED_REFRESHABLE_FROM_LIVE_PROOF: [&str; 30] = [
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
    "class.paladin.hybrid_chassis_and_spell_burden",
    "class.ranger.hybrid_chassis_and_spell_burden",
    "class.sorcerer.progression_and_spell_burden",
    "class.bard.progression_and_spell_burden",
    "class.wizard.progression_and_spell_burden",
    "class.cleric.progression_and_spell_burden",
    "class.druid.progression_and_spell_burden",
    "class.monk.bounded_progression",
    "interaction.human_bonus_feat_ability_bonus.pilot_pressure",
    "school.abjuration.spell_reachability",
    "school.conjuration.spell_reachability",
    "school.divination.spell_reachability",
    "school.enchantment.spell_reachability",
    "school.evocation.spell_reachability",
    "school.illusion.spell_reachability",
    "school.necromancy.spell_reachability",
    "school.transmutation.spell_reachability",
    "school.universal.spell_reachability",
    "equipment.arms_armor.equipment_reachability",
];

#[test]
fn every_row_carries_an_evidence_freshness_posture() {
    let matrix = matrix();
    for r in &matrix.rows {
        assert!(
            matches!(
                r.evidence_freshness,
                EvidenceFreshness::RefreshableFromLiveProof
                    | EvidenceFreshness::AwaitingInitialEvidence
            ),
            "row '{}' must carry a bounded evidence-freshness posture",
            r.row_id
        );
    }
}

#[test]
fn pilot_grounded_rows_are_refreshable_from_live_proof() {
    let matrix = matrix();
    let refreshable: Vec<&str> = matrix
        .rows
        .iter()
        .filter(|r| r.evidence_freshness == EvidenceFreshness::RefreshableFromLiveProof)
        .map(|r| r.row_id)
        .collect();

    assert_eq!(
        refreshable.len(),
        EXPECTED_REFRESHABLE_FROM_LIVE_PROOF.len(),
        "only the pilot-grounded rows may be refreshable from a live proof, got {refreshable:?}"
    );
    for id in EXPECTED_REFRESHABLE_FROM_LIVE_PROOF {
        assert!(
            refreshable.contains(&id),
            "expected row '{id}' to be refreshable from a live proof surface"
        );
    }
}

#[test]
fn observed_rows_await_initial_evidence() {
    let matrix = matrix();
    for r in &matrix.rows {
        if r.evidence_tier == EvidenceTier::Observed {
            assert_eq!(
                r.evidence_freshness,
                EvidenceFreshness::AwaitingInitialEvidence,
                "roster-only row '{}' has no runtime evidence to refresh and must await initial evidence",
                r.row_id
            );
        }
    }
}

#[test]
fn freshness_tracks_live_proof_grounding_not_optimism() {
    // The refreshable-from-live-proof set must coincide exactly with the rows that
    // rise above `Observed` evidence: freshness is grounded in whether a live proof
    // exists, never invented to imply readiness.
    let matrix = matrix();
    for r in &matrix.rows {
        let above_observed = r.evidence_tier != EvidenceTier::Observed;
        let refreshable = r.evidence_freshness == EvidenceFreshness::RefreshableFromLiveProof;
        assert_eq!(
            above_observed, refreshable,
            "row '{}' freshness must track its live-proof grounding: above_observed={above_observed}, refreshable={refreshable}",
            r.row_id
        );
    }
}

#[test]
fn no_row_claims_confirmed_fresh_evidence() {
    // This first slice can prove no completed refresh checkpoint for any row, so the
    // carrier must not expose any posture that implies a row is currently fresh /
    // refresh-confirmed. Both allowed postures are explicitly refresh-required.
    let matrix = matrix();
    for r in &matrix.rows {
        assert!(
            !r.evidence_freshness.is_refresh_confirmed(),
            "row '{}' must not claim confirmed-fresh evidence in the first audit slice",
            r.row_id
        );
    }
}

// ---------------------------------------------------------------------------
// SD13-E2-F15 non-Human interaction seam verdict.
//
// This block records the first-slice decision for
// `interaction.non_human_any_class.progression_pressure`: at this stage, no named
// non-Human interaction row is required. The verdict is pinned in two places —
// the carrier's blocker/lossiness note (which now carries the explicit
// non-named-row rationale instead of staying empty) and these tests, which
// assert the audit basis directly from the carrier so a future slice cannot
// silently reintroduce a named non-Human interaction row without first
// overturning the audit.
// ---------------------------------------------------------------------------

/// The SD13-E2-F15 verdict token: phrases that, taken together, prove the
/// blocker note names both the absence of any named non-Human interaction row
/// and the warrant condition under which one would become required. If a
/// future slice removes the non-named-row verdict, every one of these tokens
/// should disappear from the note — and these tests will fire.
const NON_HUMAN_NO_NAMED_ROW_VERDICT_TOKENS: &[&str] = &[
    "no named non-Human interaction row",
    "Unverified/Observed",
    "Human deterministic pilot surface",
    "Human interaction row",
    "becomes warranted only when",
];

/// Audit-basis tokens that name the specific rows the verdict rests on. Each
/// non-Human race row id and each Computed class row id must appear so the
/// verdict is traceable to a real row, not generic prose.
const NON_HUMAN_VERDICT_AUDIT_BASIS_TOKENS: &[&str] = &[
    "race.dwarf",
    "race.elf",
    "race.gnome",
    "race.half_elf",
    "race.half_orc",
    "race.halfling",
    "class.fighter.level_1_pilot",
    "class.fighter.levels_2_10",
    "class.rogue.bounded_progression",
    "class.barbarian.bounded_progression",
    "class.monk.bounded_progression",
    "class.paladin.hybrid_chassis_and_spell_burden",
    "class.ranger.hybrid_chassis_and_spell_burden",
    "class.sorcerer.progression_and_spell_burden",
    "class.bard.progression_and_spell_burden",
    "class.wizard.progression_and_spell_burden",
    "class.cleric.progression_and_spell_burden",
    "class.druid.progression_and_spell_burden",
];

#[test]
fn non_human_interaction_row_carries_non_empty_explicit_verdict_note() {
    // The verdict must be visible in the carrier itself, not only in prose
    // elsewhere. A future slice that empties the note is silently overturning
    // the audit and must be caught here.
    let matrix = matrix();
    let non_human = row(
        &matrix,
        "interaction.non_human_any_class.progression_pressure",
    );
    assert!(
        !non_human.blocker_or_lossiness_note.is_empty(),
        "non-Human interaction row must carry a non-empty blocker/lossiness note \
         recording the explicit no-named-row verdict, got an empty note"
    );
}

#[test]
fn non_human_interaction_row_verdict_names_the_no_named_row_decision() {
    let matrix = matrix();
    let non_human = row(
        &matrix,
        "interaction.non_human_any_class.progression_pressure",
    );
    for token in NON_HUMAN_NO_NAMED_ROW_VERDICT_TOKENS {
        assert!(
            non_human.blocker_or_lossiness_note.contains(token),
            "non-Human interaction row blocker note must name verdict token '{token}': {}",
            non_human.blocker_or_lossiness_note
        );
    }
}

#[test]
fn non_human_interaction_row_verdict_traces_to_real_audit_basis_rows() {
    // The verdict must rest on real matrix rows, not invented rationale. Every
    // non-Human race row id and every Computed class row id must appear in the
    // blocker note so a future reader can trace each claim back to a real row.
    let matrix = matrix();
    let non_human = row(
        &matrix,
        "interaction.non_human_any_class.progression_pressure",
    );
    for token in NON_HUMAN_VERDICT_AUDIT_BASIS_TOKENS {
        assert!(
            non_human.blocker_or_lossiness_note.contains(token),
            "non-Human interaction row blocker note must name real audit-basis row '{token}': {}",
            non_human.blocker_or_lossiness_note
        );
    }
}

#[test]
fn non_human_interaction_row_verdict_cites_human_interaction_row_as_already_named() {
    // The Human interaction row is the only race/class pressure the
    // deterministic compute surface exposes today. The verdict must explicitly
    // name it so a future slice cannot claim the Human interaction row was
    // overlooked when deciding no non-Human interaction row is required.
    let matrix = matrix();
    let non_human = row(
        &matrix,
        "interaction.non_human_any_class.progression_pressure",
    );
    let human_interaction = row(
        &matrix,
        "interaction.human_bonus_feat_ability_bonus.pilot_pressure",
    );
    assert!(
        non_human
            .blocker_or_lossiness_note
            .contains(human_interaction.row_id),
        "non-Human interaction row blocker note must cite the existing Human interaction \
         row id '{}' as the only already-named race/class pressure: {}",
        human_interaction.row_id,
        non_human.blocker_or_lossiness_note
    );
}

#[test]
fn non_human_interaction_row_stays_unverified_and_observed() {
    // The verdict is "no named row is required at this stage", which means the
    // row itself remains the Unverified/Observed placeholder. A future slice
    // that flips the row to Computed without first overturning the verdict in
    // the blocker note would be silently claiming a named interaction exists,
    // and must be caught here.
    let matrix = matrix();
    let non_human = row(
        &matrix,
        "interaction.non_human_any_class.progression_pressure",
    );
    assert_eq!(
        non_human.support_state,
        SupportState::Unverified,
        "non-Human interaction row must stay Unverified while the verdict stands"
    );
    assert_eq!(
        non_human.evidence_tier,
        EvidenceTier::Observed,
        "non-Human interaction row must stay Observed while the verdict stands"
    );
    assert_eq!(
        non_human.evidence_freshness,
        EvidenceFreshness::AwaitingInitialEvidence,
        "non-Human interaction row must stay AwaitingInitialEvidence while the verdict stands"
    );
}

#[test]
fn non_human_interaction_row_subject_id_remains_a_generic_placeholder() {
    // The verdict forbids adding a named non-Human interaction row at this
    // stage. The existing row's subject_id is therefore still the generic
    // "non-human-any-class-progression" placeholder, not a specific named
    // race/class pair (e.g. race.dwarf:race.dwarf+class.fighter.level_1_pilot).
    let matrix = matrix();
    let non_human = row(
        &matrix,
        "interaction.non_human_any_class.progression_pressure",
    );
    assert_eq!(
        non_human.subject_id, "interaction:non-human-any-class-progression",
        "non-Human interaction row subject_id must remain the generic placeholder until a \
         future slice proves a specific non-Human race/class pair at the compute surface: {}",
        non_human.subject_id
    );
}

#[test]
fn non_human_interaction_row_verdict_does_not_silently_add_a_named_pair_row() {
    // The interaction row count must stay at exactly 2: the Human interaction
    // row plus this generic non-Human placeholder. If a future slice adds a
    // named race/class interaction row without first overturning the verdict,
    // the count check (combined with the subject_id check) catches the
    // silent introduction.
    let matrix = matrix();
    let interaction_rows: Vec<&SupportStateRow> = matrix
        .rows
        .iter()
        .filter(|r| r.subject_type == MatrixSubjectType::Interaction)
        .collect();

    assert_eq!(
        interaction_rows.len(),
        2,
        "interaction row count must stay at 2 while the verdict stands, got {}: {:?}",
        interaction_rows.len(),
        interaction_rows
            .iter()
            .map(|r| r.row_id)
            .collect::<Vec<_>>()
    );

    let row_ids: std::collections::BTreeSet<&str> =
        interaction_rows.iter().map(|r| r.row_id).collect();
    assert!(
        row_ids.contains("interaction.human_bonus_feat_ability_bonus.pilot_pressure"),
        "Human interaction row must remain one of the two interaction rows"
    );
    assert!(
        row_ids.contains("interaction.non_human_any_class.progression_pressure"),
        "non-Human interaction row must remain one of the two interaction rows"
    );

    // No third interaction row may claim a specific non-Human race/class pair
    // while the verdict stands. Pin the absence explicitly so a future slice
    // that adds one is forced to overturn the verdict first.
    for r in &interaction_rows {
        let subject_is_specific_non_human_pair = r.subject_id.starts_with("interaction:")
            && r.subject_id != "interaction:human-bonus-feat-ability-bonus"
            && r.subject_id != "interaction:non-human-any-class-progression";
        assert!(
            !subject_is_specific_non_human_pair,
            "no additional named non-Human interaction row is permitted while the verdict \
             stands; found subject_id '{}' on row '{}'",
            r.subject_id, r.row_id
        );
    }
}

#[test]
fn non_human_interaction_row_next_uplift_states_the_warrant_condition() {
    // The next_required_uplift must encode the future warrant condition: a
    // named non-Human interaction row becomes required only when a non-Human
    // race trait is proven at the compute surface AND a class row exposes a
    // distinct non-Human race x class pressure the separate rows do not
    // already absorb. This pins the doctrinal boundary so a future slice that
    // weakens the warrant (e.g. "add rows later") is caught here.
    let matrix = matrix();
    let non_human = row(
        &matrix,
        "interaction.non_human_any_class.progression_pressure",
    );
    let uplift = non_human.next_required_uplift;
    for token in [
        "named non-Human interaction row",
        "SD13-E2",
        "non-Human race trait",
        "compute surface",
        "separate race and class rows",
    ] {
        assert!(
            uplift.contains(token),
            "non-Human interaction row next_required_uplift must name warrant token '{token}': {}",
            uplift
        );
    }
}

#[test]
fn non_human_interaction_row_verdict_keeps_row_anchored_to_roster_matrix_doc() {
    // The verdict is documentary/control-plane truth, not a runtime result.
    // The grounding_ref must continue to cite the SD-13 roster matrix
    // authority — i.e. no compute seam has been introduced for a non-Human
    // race/class interaction while the verdict stands.
    let matrix = matrix();
    let non_human = row(
        &matrix,
        "interaction.non_human_any_class.progression_pressure",
    );
    assert!(
        non_human
            .grounding_ref
            .contains("core-roster-and-support-state-matrix.md"),
        "non-Human interaction row must stay anchored to the SD-13 roster matrix doc while \
         the verdict stands, got: {}",
        non_human.grounding_ref
    );
}
