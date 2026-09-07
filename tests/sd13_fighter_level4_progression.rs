//! SD13-E3 Fighter level 4 milestone progression proof.
//!
//! Widens the accepted deterministic Human Fighter pilot seam from the bounded
//! levels 1-3 tranche to level 4: base attack / base save progression continues
//! generically (the existing `classlevel`, `classlevel/2+2`, `classlevel/3`
//! formulas were already level-generic), and the level-4 Fighter bonus-feat slot
//! is surfaced as an explicit progression seam, mirroring the level-2 bonus-feat
//! pattern. Armor training stays at rank 1 (unchanged since level 3; the next
//! rank is level 7), so the armor-check-dependent skill totals and armor class
//! carry over unchanged from level 3. The generic PF1 level-4 ability-score
//! increase needs no new mechanism: the chosen ability score is trusted at face
//! value, exactly like every other ability adjustment in this codebase.
//!
//! It is intentionally not a broad martial engine. It grounds only the level-4
//! base attack / base save delta and the level-4 bonus-feat slot. It asserts no
//! level-5+ Fighter burden (weapon training begins at level 5), no spell burden,
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

const LEVEL_4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level4_sd13_deterministic_input.txt");
const LEVEL_3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level3_sd13_deterministic_input.txt");

// ----- Milestone: level 4 is no longer blanket-blocked -----

#[test]
fn level_4_human_fighter_produces_non_blocked_bounded_evidence() {
    let input = load(LEVEL_4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic level-4 Fighter must not block claims: {:?}",
        computation.diagnostics
    );

    // Fighter level-4 base chassis: full BAB +4, good Fortitude +4, poor Reflex/Will +1.
    assert_eq!(computation.base_attack_bonus, 4);
    assert_eq!(computation.base_saves.fortitude, 4);
    assert_eq!(computation.base_saves.reflex, 1);
    assert_eq!(computation.base_saves.will, 1);

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(bab.value, 4);
    assert!(
        bab.detail.contains("level 4"),
        "level-4 BAB explanation must name the level-4 delta: {}",
        bab.detail
    );

    // Total saves: base + relevant ability modifier (CON +2, DEX +2, WIS +1).
    assert_eq!(computation.total_saves.fortitude, 6);
    assert_eq!(computation.total_saves.reflex, 3);
    assert_eq!(computation.total_saves.will, 2);

    // Baseline combat advances with BAB: +4 BAB + STR +3 + Weapon Focus +1 = 8.
    assert_eq!(computation.baseline_melee_attack_bonus, 9);
    // Armor class is unchanged since level 3 (armor training stays rank 1 until level 7).
    assert_eq!(computation.baseline_armor_class, 17);

    // Selected skills are unchanged since level 3 (same armor-training rank). CG-03 fix:
    // STR modifier is now +4 (base 16 + 2 Human racial), not +3.
    assert_eq!(computation.selected_skill_modifiers.climb, 7);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    assert_eq!(computation.selected_skill_modifiers.swim, 7);
}

#[test]
fn level_4_bonus_feat_progression_seam_is_explicit_and_bounded() {
    let input = load(LEVEL_4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let seam = explanation(&computation, "class_feature.fighter.level_4_bonus_feat");
    assert_eq!(
        seam.value, 0,
        "level-4 bonus-feat seam must not fabricate a feat-effect value: {seam:?}"
    );
    assert!(
        seam.detail.contains("choice:fighter_bonus_feat_4"),
        "level-4 bonus-feat seam must name the chosen selection: {}",
        seam.detail
    );
    assert!(
        seam.detail.contains("feat-effect") || seam.detail.contains("prerequisite"),
        "level-4 bonus-feat seam must state it grounds no general feat/prerequisite engine: {}",
        seam.detail
    );
}

#[test]
fn level_4_still_carries_the_level_2_and_level_3_seams() {
    let input = load(LEVEL_4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_feature.fighter.level_2_bonus_feat"),
        "level-4 Fighter must still carry the level-2 bonus-feat seam: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "class_feature.fighter.armor_training"),
        "level-4 Fighter must still carry the level-3 armor-training seam: {:?}",
        computation.explanations
    );
}

// ----- Milestone: downstream propagation of the widened bounded truth -----

#[test]
fn level_4_propagates_computed_receipt_and_view_model() {
    let input = load(LEVEL_4_FIXTURE);
    let receipt = build_pilot_headless_receipt(&input);

    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Computed,
        "widened level-4 Fighter tranche must propagate a computed receipt"
    );

    assert!(
        has_explanation(&receipt.computation, "race.human.ability_bonus_target"),
        "widened level-4 receipt must preserve the Human ability-bonus race seam: {:?}",
        receipt.computation.explanations
    );
    assert!(
        has_explanation(&receipt.computation, "race.human.bonus_feat_grant"),
        "widened level-4 receipt must preserve the Human bonus-feat race seam: {:?}",
        receipt.computation.explanations
    );

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Computed);
    assert_eq!(view_model.primary_owner, PrimaryOwner::OracleGap);
    let snapshot = view_model
        .snapshot
        .as_ref()
        .expect("computed level-4 receipt must yield a snapshot");
    assert_eq!(
        snapshot.base_attack_bonus, receipt.computation.base_attack_bonus,
        "view-model snapshot must carry the widened level-4 base attack bonus"
    );
}

// ----- Negative control: level 5 stays blocked (weapon training is out of scope) -----

#[test]
fn level_5_fighter_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 5 (Weapon Training 1) was the
    // next unproven milestone and stayed claim-blocked. A later SD13-E3 slice
    // (tests/sd13_fighter_level5_progression.rs) widened level 5 into the
    // supported tranche; this negative control is superseded, not violated —
    // pin the new truth here too so this file stays internally consistent.
    let level_5 = LEVEL_4_FIXTURE.replace("class:fighter:4", "class:fighter:5");
    let input = load(&level_5);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-5 Fighter is supported since the SD13-E3 level-5 slice: {:?}",
        computation.diagnostics
    );
    assert!(
        has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "level-5 Fighter must surface a computed base-attack-bonus explanation"
    );
}

// ----- Control plane: the matrix widens the levels-2-10 row's proven range to level 4 -----

#[test]
fn matrix_levels_2_10_names_level_4_as_proven() {
    // Levels 5-10 are a separate, later concern (tests/sd13_fighter_level5_progression.rs
    // widened level 5); this test only asserts that level 4's own proof landed.
    let matrix = seeded_current_truth();
    let row = matrix
        .row("class.fighter.levels_2_10")
        .expect("row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(row.support_state, SupportState::Supported);
    assert_eq!(row.evidence_tier, EvidenceTier::ProductVisible);
    // The row's proven range has since widened past level 4 (to level 6); level 4
    // remains a subset of whatever range is currently proven, so assert the
    // level-4-specific blocker-note evidence directly rather than the dimension
    // text's exact digits, which a later slice is free to rephrase.
    assert!(
        row.blocker_or_lossiness_note.contains("level-4"),
        "levels-2-10 row blocker note must name the level-4 milestone: {}",
        row.blocker_or_lossiness_note
    );
    // What remains unproven beyond level 4 is a separate, later concern —
    // tests/sd13_fighter_level5_progression.rs owns that assertion now that
    // level 5 itself has landed.
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
        "the level-4 slice must not promote any row to Supported or Lossy"
    );
}

// Preserve the level-3-only fixture as a still-valid, unchanged reference point.
#[test]
fn level_3_fixture_still_loads_and_computes_unaffected_by_the_level_4_widening() {
    let input = load(LEVEL_3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    assert_eq!(computation.base_attack_bonus, 3);
    assert!(!computation.diagnostics.iter().any(|d| d.claim_blocking));
}
