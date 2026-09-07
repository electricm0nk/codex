//! SD13-E3 Fighter level 5 milestone progression proof.
//!
//! Widens the accepted deterministic Human Fighter pilot seam from the bounded
//! levels 1-4 tranche to level 5: base attack / base save progression continues
//! generically, and Weapon Training 1 is surfaced as an explicit progression seam.
//! Weapon Training grants +1 to both attack rolls and damage rolls with weapons of
//! the chosen weapon group in PF1 Core Rulebook. This slice grounds only the
//! attack-roll half: the deterministic Longsword falls under the Heavy Blades
//! group, so the canonical `choice:fighter_weapon_training_group ->
//! group:heavy_blades` selection folds +1 into the baseline melee attack bonus,
//! validated by the same canonical-choice machinery that already guards the
//! bonus-feat slots. The damage-roll half stays explicitly unproven: no damage
//! total is computed anywhere in this codebase for any Fighter level, so this is
//! not a new gap introduced by this slice.
//!
//! It is intentionally not a broad martial engine. It grounds only the level-5
//! base attack / base save delta and the Weapon Training 1 attack-bonus half. It
//! asserts no level-6+ Fighter burden (armor training's next rank is level 7), no
//! spell burden, no non-Fighter positive support, and no general
//! feat/prerequisite engine.

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

const LEVEL_5_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level5_sd13_deterministic_input.txt");
const LEVEL_4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level4_sd13_deterministic_input.txt");

// ----- Milestone: level 5 is no longer blanket-blocked -----

#[test]
fn level_5_human_fighter_produces_non_blocked_bounded_evidence() {
    let input = load(LEVEL_5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic level-5 Fighter must not block claims: {:?}",
        computation.diagnostics
    );

    // Fighter level-5 base chassis: full BAB +5, good Fortitude +4, poor Reflex/Will +1.
    assert_eq!(computation.base_attack_bonus, 5);
    assert_eq!(computation.base_saves.fortitude, 4);
    assert_eq!(computation.base_saves.reflex, 1);
    assert_eq!(computation.base_saves.will, 1);

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(bab.value, 5);
    assert!(
        bab.detail.contains("level 5"),
        "level-5 BAB explanation must name the level-5 delta: {}",
        bab.detail
    );

    // Total saves: base + relevant ability modifier (CON +2, DEX +2, WIS +1).
    assert_eq!(computation.total_saves.fortitude, 6);
    assert_eq!(computation.total_saves.reflex, 3);
    assert_eq!(computation.total_saves.will, 2);

    // Baseline combat: +5 BAB + STR +3 + Weapon Focus +1 + Weapon Training +1 = 10.
    assert_eq!(computation.baseline_melee_attack_bonus, 11);
    // Armor class is unchanged since level 3 (armor training stays rank 1 until level 7).
    assert_eq!(computation.baseline_armor_class, 17);

    // Selected skills are unchanged since level 3 (same armor-training rank). CG-03 fix:
    // STR modifier is now +4 (base 16 + 2 Human racial), not +3.
    assert_eq!(computation.selected_skill_modifiers.climb, 7);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    assert_eq!(computation.selected_skill_modifiers.swim, 7);
}

#[test]
fn level_5_weapon_training_seam_folds_attack_bonus_and_names_unproven_damage() {
    let input = load(LEVEL_5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let seam = explanation(&computation, "class_feature.fighter.weapon_training");
    assert_eq!(
        seam.value, 1,
        "weapon training seam must carry the +1 attack-bonus value it folds into \
         the baseline melee attack bonus: {seam:?}"
    );
    assert!(
        seam.detail.contains("heavy_blades") || seam.detail.contains("Heavy Blades"),
        "weapon training seam must name the chosen weapon group: {}",
        seam.detail
    );
    assert!(
        seam.detail.contains("damage"),
        "weapon training seam must explicitly name the still-unproven damage-roll half: {}",
        seam.detail
    );

    // The melee attack bonus explanation itself must cite the weapon-training +1.
    let melee = explanation(&computation, "combat.baseline_melee_attack_bonus");
    assert!(
        melee.detail.contains("Weapon Training") || melee.detail.contains("weapon training"),
        "level-5 melee attack bonus explanation must cite the Weapon Training contribution: {}",
        melee.detail
    );
}

#[test]
fn non_canonical_weapon_training_group_is_claim_blocked() {
    let non_canonical =
        LEVEL_5_FIXTURE.replace("group:heavy_blades", "group:double_weapons");
    let input = load(&non_canonical);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.claim_blocking && d.id.contains("fighter_weapon_training_group")),
        "non-canonical weapon-training-group selection must be claim-blocked: {:?}",
        computation.diagnostics
    );
}

#[test]
fn level_5_still_carries_the_level_2_level_3_and_level_4_seams() {
    let input = load(LEVEL_5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        "class_feature.fighter.level_2_bonus_feat",
        "class_feature.fighter.armor_training",
        "class_feature.fighter.level_4_bonus_feat",
    ] {
        assert!(
            has_explanation(&computation, id),
            "level-5 Fighter must still carry the earlier seam '{id}': {:?}",
            computation.explanations
        );
    }
}

// ----- Milestone: downstream propagation of the widened bounded truth -----

#[test]
fn level_5_propagates_computed_receipt_and_view_model() {
    let input = load(LEVEL_5_FIXTURE);
    let receipt = build_pilot_headless_receipt(&input);

    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Computed,
        "widened level-5 Fighter tranche must propagate a computed receipt"
    );

    assert!(
        has_explanation(&receipt.computation, "race.human.ability_bonus_target"),
        "widened level-5 receipt must preserve the Human ability-bonus race seam: {:?}",
        receipt.computation.explanations
    );

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Computed);
    assert_eq!(view_model.primary_owner, PrimaryOwner::OracleGap);
    let snapshot = view_model
        .snapshot
        .as_ref()
        .expect("computed level-5 receipt must yield a snapshot");
    assert_eq!(
        snapshot.base_attack_bonus, receipt.computation.base_attack_bonus,
        "view-model snapshot must carry the widened level-5 base attack bonus"
    );
}

// ----- Negative control: level 6 stays blocked -----

#[test]
fn level_6_fighter_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 6 (another bonus feat) was the
    // next unproven milestone and stayed claim-blocked. A later SD13-E3 slice
    // (tests/sd13_fighter_level6_progression.rs) widened level 6 into the
    // supported tranche; this negative control is superseded, not violated —
    // pin the new truth here too so this file stays internally consistent.
    let level_6 = LEVEL_5_FIXTURE.replace("class:fighter:5", "class:fighter:6");
    let input = load(&level_6);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-6 Fighter is supported since the SD13-E3 level-6 slice: {:?}",
        computation.diagnostics
    );
    assert!(
        has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "level-6 Fighter must surface a computed base-attack-bonus explanation"
    );
}

// ----- Control plane: the matrix widens the levels-2-10 row's proven range to level 5 -----

#[test]
fn matrix_levels_2_10_names_level_5_as_proven() {
    // Levels 6-10 are a separate, later concern (tests/sd13_fighter_level6_progression.rs
    // widened level 6); this test only asserts that level 5's own proof landed.
    let matrix = seeded_current_truth();
    let row = matrix
        .row("class.fighter.levels_2_10")
        .expect("row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(row.support_state, SupportState::Supported);
    assert_eq!(row.evidence_tier, EvidenceTier::ProductVisible);
    assert!(
        row.blocker_or_lossiness_note.contains("Weapon Training")
            || row.blocker_or_lossiness_note.contains("weapon training"),
        "levels-2-10 row blocker note must name the weapon training milestone: {}",
        row.blocker_or_lossiness_note
    );
    assert!(
        row.blocker_or_lossiness_note.contains("damage"),
        "levels-2-10 row blocker note must name the still-unproven damage-roll half: {}",
        row.blocker_or_lossiness_note
    );
    // What remains unproven beyond level 5 is a separate, later concern —
    // tests/sd13_fighter_level6_progression.rs owns that assertion now that
    // level 6 itself has landed.
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
        "the level-5 slice must not promote any row to Supported or Lossy"
    );
}

// Preserve the level-4-only fixture as a still-valid, unchanged reference point.
#[test]
fn level_4_fixture_still_loads_and_computes_unaffected_by_the_level_5_widening() {
    let input = load(LEVEL_4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    assert_eq!(computation.base_attack_bonus, 4);
    assert_eq!(computation.baseline_melee_attack_bonus, 9);
    assert!(!computation.diagnostics.iter().any(|d| d.claim_blocking));
}
