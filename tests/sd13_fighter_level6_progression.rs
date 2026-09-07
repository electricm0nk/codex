//! SD13-E3 Fighter level 6 milestone progression proof.
//!
//! Widens the accepted deterministic Human Fighter pilot seam from the bounded
//! levels 1-5 tranche to level 6: base attack / base save progression continues
//! generically, and the level-6 Fighter bonus-feat slot is surfaced as an
//! explicit progression seam, mirroring the level-2/level-4 pattern (the Fighter
//! bonus-feat cadence continues at 1, 2, 4, 6, 8, 10, ...). Armor Training 2 is
//! not gained until level 7, so armor class and the armor-check-dependent skill
//! totals carry over unchanged from level 3-5.
//!
//! It is intentionally not a broad martial engine. It grounds only the level-6
//! base attack / base save delta and the level-6 bonus-feat slot. It asserts no
//! level-7+ Fighter burden (Armor Training 2 begins at level 7), no spell burden,
//! no non-Fighter positive support, and no general feat/prerequisite engine.

use codex::rules_core::pilot_compute::{
    HeadlessReceiptStatus,
    build_pilot_headless_receipt,
    compute_pilot_base_chassis,
};
use codex::rules_core::pilot_failure::PrimaryOwner;
use codex::rules_core::pilot_view_model::PilotViewModel;
use codex::rules_core::support_state_matrix::{
    EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const LEVEL_6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level6_sd13_deterministic_input.txt");
const LEVEL_5_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level5_sd13_deterministic_input.txt");

// ----- Milestone: level 6 is no longer blanket-blocked -----

#[test]
fn level_6_human_fighter_produces_non_blocked_bounded_evidence() {
    let input = load(LEVEL_6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic level-6 Fighter must not block claims: {:?}",
        computation.diagnostics
    );

    // Fighter level-6 base chassis: full BAB +6, good Fortitude +5, poor Reflex/Will +2.
    assert_eq!(computation.base_attack_bonus, 6);
    assert_eq!(computation.base_saves.fortitude, 5);
    assert_eq!(computation.base_saves.reflex, 2);
    assert_eq!(computation.base_saves.will, 2);

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(bab.value, 6);
    assert!(
        bab.detail.contains("level 6"),
        "level-6 BAB explanation must name the level-6 delta: {}",
        bab.detail
    );

    // Total saves: base + relevant ability modifier (CON +2, DEX +2, WIS +1).
    assert_eq!(computation.total_saves.fortitude, 7);
    assert_eq!(computation.total_saves.reflex, 4);
    assert_eq!(computation.total_saves.will, 3);

    // Baseline combat: +6 BAB + STR +3 + Weapon Focus +1 + Weapon Training +1 = 11.
    assert_eq!(computation.baseline_melee_attack_bonus, 12);
    // Armor class is unchanged since level 3 (Armor Training 2 begins at level 7).
    assert_eq!(computation.baseline_armor_class, 17);

    // Selected skills are unchanged since level 3 (same armor-training rank). CG-03 fix:
    // STR modifier is now +4 (base 16 + 2 Human racial), not +3.
    assert_eq!(computation.selected_skill_modifiers.climb, 7);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    assert_eq!(computation.selected_skill_modifiers.swim, 7);
}

#[test]
fn level_6_bonus_feat_progression_seam_is_explicit_and_bounded() {
    let input = load(LEVEL_6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let seam = explanation(&computation, "class_feature.fighter.level_6_bonus_feat");
    assert_eq!(
        seam.value, 0,
        "level-6 bonus-feat seam must not fabricate a feat-effect value: {seam:?}"
    );
    assert!(
        seam.detail.contains("choice:fighter_bonus_feat_6"),
        "level-6 bonus-feat seam must name the chosen selection: {}",
        seam.detail
    );
    assert!(
        seam.detail.contains("feat-effect") || seam.detail.contains("prerequisite"),
        "level-6 bonus-feat seam must state it grounds no general feat/prerequisite engine: {}",
        seam.detail
    );
}

#[test]
fn level_6_still_carries_every_earlier_seam() {
    let input = load(LEVEL_6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        "class_feature.fighter.level_2_bonus_feat",
        "class_feature.fighter.armor_training",
        "class_feature.fighter.level_4_bonus_feat",
        "class_feature.fighter.weapon_training",
    ] {
        assert!(
            has_explanation(&computation, id),
            "level-6 Fighter must still carry the earlier seam '{id}': {:?}",
            computation.explanations
        );
    }
}

// ----- Milestone: downstream propagation of the widened bounded truth -----

#[test]
fn level_6_propagates_computed_receipt_and_view_model() {
    let input = load(LEVEL_6_FIXTURE);
    let receipt = build_pilot_headless_receipt(&input);

    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Computed,
        "widened level-6 Fighter tranche must propagate a computed receipt"
    );

    assert!(
        has_explanation(&receipt.computation, "race.human.ability_bonus_target"),
        "widened level-6 receipt must preserve the Human ability-bonus race seam: {:?}",
        receipt.computation.explanations
    );

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Computed);
    assert_eq!(view_model.primary_owner, PrimaryOwner::OracleGap);
    let snapshot = view_model
        .snapshot
        .as_ref()
        .expect("computed level-6 receipt must yield a snapshot");
    assert_eq!(
        snapshot.base_attack_bonus, receipt.computation.base_attack_bonus,
        "view-model snapshot must carry the widened level-6 base attack bonus"
    );
}

// ----- Negative control: level 7 was later widened into the supported tranche -----

#[test]
fn level_7_fighter_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 7 (Armor Training 2) was the
    // next unproven milestone and stayed claim-blocked. A later SD13-E3 slice
    // (tests/sd13_fighter_level7_progression.rs) widened level 7 into the
    // supported tranche; this negative control is superseded, not violated —
    // pin the new truth here too so this file stays internally consistent.
    let level_7 = LEVEL_6_FIXTURE.replace("class:fighter:6", "class:fighter:7");
    let input = load(&level_7);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-7 Fighter is supported since the SD13-E3 level-7 slice: {:?}",
        computation.diagnostics
    );
    assert!(
        has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "level-7 Fighter must surface a computed base-attack-bonus explanation"
    );
}

// ----- Control plane: the matrix widens the levels-2-10 row's proven range to level 6 -----

#[test]
fn matrix_levels_2_10_names_level_6_as_proven() {
    // The row's proven range has since widened past level 6 (to level 7 —
    // tests/sd13_fighter_level7_progression.rs), so this test only asserts that
    // level 6's own proof landed, rather than the exact current grounding_ref or
    // "remaining" range, which a later slice is free to move forward.
    let matrix = seeded_current_truth();
    let row = matrix
        .row("class.fighter.levels_2_10")
        .expect("row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(row.support_state, SupportState::Supported);
    assert_eq!(row.evidence_tier, EvidenceTier::ProductVisible);
    assert!(
        row.blocker_or_lossiness_note.contains("level-6 bonus-feat")
            || row.blocker_or_lossiness_note.contains("level 6"),
        "levels-2-10 row blocker note must name the level-6 milestone: {}",
        row.blocker_or_lossiness_note
    );
}

#[test]
fn matrix_preserves_fighter_level_1_and_other_accepted_rows() {
    let matrix = seeded_current_truth();

    let level_1 = matrix
        .row("class.fighter.level_1_pilot")
        .expect("level-1 row must exist");
    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(level_1.support_state, SupportState::Supported);

    assert!(
        !matrix
            .rows
            .iter()
            // school.abjuration/illusion.spell_reachability were later promoted to
            // Supported/Product-visible by SD-19's operator-driven UI-surfacing work
            // (2026-07-16) -- excluded here, not an unintended promotion by this slice.
            .any(|r| (r.support_state == SupportState::Supported
                && r.row_id != "school.abjuration.spell_reachability"
                && r.row_id != "school.illusion.spell_reachability"
                && r.row_id != "school.conjuration.spell_reachability"
                && r.row_id != "school.divination.spell_reachability"
                && r.row_id != "school.enchantment.spell_reachability"
                && r.row_id != "school.evocation.spell_reachability"
                && r.row_id != "school.necromancy.spell_reachability"
                && r.row_id != "school.transmutation.spell_reachability"
                && r.row_id != "school.universal.spell_reachability"
                && r.row_id != "equipment.arms_armor.equipment_reachability"
                && r.row_id != "equipment.general.equipment_reachability"
                && r.row_id != "equipment.magic_items.equipment_reachability"
                && r.row_id != "race.human.pilot_semantics"
                && r.row_id != "race.dwarf.bounded_semantics"
                && r.row_id != "race.elf.bounded_semantics"
                && r.row_id != "race.gnome.bounded_semantics"
                && r.row_id != "race.half_elf.bounded_semantics"
                && r.row_id != "race.half_orc.bounded_semantics"
                && r.row_id != "race.halfling.bounded_semantics"
                && r.row_id != "class.fighter.level_1_pilot"
                && r.row_id != "class.fighter.levels_2_10"
                && r.row_id != "class.monk.bounded_progression"
                && r.row_id != "class.druid.progression_and_spell_burden"
                && r.row_id != "class.barbarian.bounded_progression"
                && r.row_id != "class.cleric.progression_and_spell_burden"
                && r.row_id != "class.wizard.progression_and_spell_burden"
                && r.row_id != "class.rogue.bounded_progression"
                && r.row_id != "class.sorcerer.progression_and_spell_burden"
                && r.row_id != "class.bard.progression_and_spell_burden"
                && r.row_id != "class.paladin.hybrid_chassis_and_spell_burden"
                && r.row_id != "class.ranger.hybrid_chassis_and_spell_burden"
                && r.row_id != "interaction.human_bonus_feat_ability_bonus.pilot_pressure"
                && r.row_id != "equipment.equipmods.equipment_reachability")
                || r.support_state == SupportState::Lossy),
        "the level-6 slice must not promote any row to Supported or Lossy"
    );
}

// Preserve the level-5-only fixture as a still-valid, unchanged reference point.
#[test]
fn level_5_fixture_still_loads_and_computes_unaffected_by_the_level_6_widening() {
    let input = load(LEVEL_5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    assert_eq!(computation.base_attack_bonus, 5);
    assert_eq!(computation.baseline_melee_attack_bonus, 11);
    assert!(!computation.diagnostics.iter().any(|d| d.claim_blocking));
}
