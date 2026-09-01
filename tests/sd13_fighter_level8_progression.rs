//! SD13-E3 Fighter level 8 milestone progression proof.
//!
//! Widens the accepted deterministic Human Fighter pilot seam from the bounded
//! levels 1-7 tranche to level 8: base attack / base save progression continues
//! generically, and the level-8 bonus-feat progression seam is surfaced as an
//! explicit progression seam, mirroring the level-2/level-4/level-6 bonus-feat
//! pattern exactly (Fighter's bonus-feat cadence is 1, 2, 4, 6, 8, 10, ...). Level
//! 8 introduces no new armor-training or weapon-training milestone: Armor Training
//! 2 stayed at level 7 and Weapon Training 1 stayed at level 5, so neither seam
//! changes at level 8.
//!
//! It is intentionally not a broad martial engine. It grounds only the level-8
//! base attack / base save delta and the level-8 bonus-feat seam. It asserts no
//! level-9+ Fighter burden, no spell burden, no non-Fighter positive support, and
//! no general feat/prerequisite engine.

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

const LEVEL_8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level8_sd13_deterministic_input.txt");
const LEVEL_7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level7_sd13_deterministic_input.txt");

// ----- Milestone: level 8 is no longer blanket-blocked -----

#[test]
fn level_8_human_fighter_produces_non_blocked_bounded_evidence() {
    let input = load(LEVEL_8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic level-8 Fighter must not block claims: {:?}",
        computation.diagnostics
    );

    // Fighter level-8 base chassis: full BAB +8, good Fortitude +6, poor Reflex/Will +2.
    assert_eq!(computation.base_attack_bonus, 8);
    assert_eq!(computation.base_saves.fortitude, 6);
    assert_eq!(computation.base_saves.reflex, 2);
    assert_eq!(computation.base_saves.will, 2);

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(bab.value, 8);
    assert!(
        bab.detail.contains("level 8"),
        "level-8 BAB explanation must name the level-8 delta: {}",
        bab.detail
    );

    // Total saves: base + relevant ability modifier (CON +2, DEX +2, WIS +1).
    assert_eq!(computation.total_saves.fortitude, 8);
    assert_eq!(computation.total_saves.reflex, 4);
    assert_eq!(computation.total_saves.will, 3);

    // Baseline combat: +8 BAB + STR +3 + Weapon Focus +1 + Weapon Training +1 = 13.
    assert_eq!(computation.baseline_melee_attack_bonus, 14);
    // Armor class is unchanged from level 7: no new armor-training milestone lands
    // at level 8, and the deterministic +2 Dexterity contribution is already below
    // both the Armor Training 1 and Armor Training 2 max-Dex caps.
    assert_eq!(computation.baseline_armor_class, 17);

    // No new armor-training milestone lands at level 8, so the Climb/Intimidate/Swim
    // selected-skill totals stay exactly as they were at level 7 (CG-03 fix: 8, not 7,
    // since STR modifier is now +4).
    assert_eq!(computation.selected_skill_modifiers.climb, 8);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    assert_eq!(computation.selected_skill_modifiers.swim, 8);
}

#[test]
fn level_8_bonus_feat_seam_is_explicit_and_bounded() {
    let input = load(LEVEL_8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let seam = explanation(&computation, "class_feature.fighter.level_8_bonus_feat");
    assert_eq!(
        seam.value, 0,
        "level-8 bonus-feat seam must contribute no fabricated mechanical value: {seam:?}"
    );
    assert!(
        seam.detail.contains("choice:fighter_bonus_feat_8"),
        "level-8 bonus-feat seam must name the level-8 bonus-feat choice set: {}",
        seam.detail
    );
}

#[test]
fn level_8_still_carries_every_earlier_seam() {
    let input = load(LEVEL_8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        "class_feature.fighter.level_2_bonus_feat",
        "class_feature.fighter.level_4_bonus_feat",
        "class_feature.fighter.level_6_bonus_feat",
        "class_feature.fighter.armor_training",
        "class_feature.fighter.weapon_training",
    ] {
        assert!(
            has_explanation(&computation, id),
            "level-8 Fighter must still carry the earlier seam '{id}': {:?}",
            computation.explanations
        );
    }

    // Armor Training 2 (not a fresh Armor Training 3) is still the seam named at
    // level 8, since no new armor-training rank lands beyond level 7.
    let armor_training = explanation(&computation, "class_feature.fighter.armor_training");
    assert_eq!(armor_training.value, 2);
}

// ----- Milestone: downstream propagation of the widened bounded truth -----

#[test]
fn level_8_propagates_computed_receipt_and_view_model() {
    let input = load(LEVEL_8_FIXTURE);
    let receipt = build_pilot_headless_receipt(&input);

    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Computed,
        "widened level-8 Fighter tranche must propagate a computed receipt"
    );

    assert!(
        has_explanation(&receipt.computation, "race.human.ability_bonus_target"),
        "widened level-8 receipt must preserve the Human ability-bonus race seam: {:?}",
        receipt.computation.explanations
    );

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Computed);
    assert_eq!(view_model.primary_owner, PrimaryOwner::OracleGap);
    let snapshot = view_model
        .snapshot
        .as_ref()
        .expect("computed level-8 receipt must yield a snapshot");
    assert_eq!(
        snapshot.base_attack_bonus, receipt.computation.base_attack_bonus,
        "view-model snapshot must carry the widened level-8 base attack bonus"
    );
}

// ----- Negative control: a Fighter above the bounded tranche stays blocked -----

#[test]
fn fighter_above_the_bounded_tranche_stays_claim_blocked() {
    // The bounded tranche has since widened past level 8 (to level 15 — SD13-E5
    // widened it to level 10 in tests/sd13_fighter_level9_level10_progression.rs,
    // and SD18 widened it further to level 11 in
    // tests/sd18_fighter_level11_armor_training3.rs, to level 12 in
    // tests/sd18_fighter_level12_widening.rs, to level 13 in
    // tests/sd18_fighter_level13_widening.rs, to level 14 in
    // tests/sd18_fighter_level14_widening.rs, to level 15 in
    // tests/sd18_fighter_level15_widening.rs, to level 16 in
    // tests/sd18_fighter_level16_widening.rs, to level 17 in
    // tests/sd18_fighter_level17_widening.rs, to level 18 in
    // tests/sd18_fighter_level18_widening.rs, to level 19 in
    // tests/sd18_fighter_level19_widening.rs, and to level 20 (the FINAL
    // level within PF1's 1-20 character-level cap) in
    // tests/sd18_fighter_level20_widening.rs), so this negative control now
    // sits just above the current bound (level 21, which does not exist as
    // a PF1 character level) rather than at level 9, level 11, level 12,
    // level 13, level 14, level 15, level 16, level 17, level 18, level 19,
    // or level 20.
    let level_21 = LEVEL_8_FIXTURE.replace("class:fighter:8", "class:fighter:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-21 Fighter must stay claim-blocked above the bounded tranche: {:?}",
        computation.diagnostics
    );
    assert!(
        !has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "level-21 Fighter must not fabricate a base-attack-bonus explanation"
    );
}

// ----- Negative control: non-Fighter classes don't leak the new explanation -----

#[test]
fn non_fighter_class_does_not_leak_level_8_bonus_feat_explanation() {
    // A single-class Paladin at the hybrid baseline level must never carry the
    // Fighter-only level-8 bonus-feat seam, even though both classes are
    // recognized identities on this rules-core surface.
    let paladin_fixture = LEVEL_8_FIXTURE
        .replace("class:fighter:8", "class:paladin:1")
        .replace("case_id=pf1-crb-human-fighter-level8", "case_id=pf1-crb-human-paladin-level1");
    let input = load(&paladin_fixture);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(
            &computation,
            "class_feature.fighter.level_8_bonus_feat"
        ),
        "non-Fighter class must not leak the Fighter level-8 bonus-feat explanation: {:?}",
        computation.explanations
    );
}

// ----- Control plane: the matrix widens the levels-2-10 row's proven range to level 8 -----

#[test]
fn matrix_levels_2_10_names_level_8_as_proven() {
    // The row's proven range has since widened past level 8 (to level 10 —
    // tests/sd13_fighter_level9_level10_progression.rs), so this test only asserts
    // that level 8's own proof landed, rather than the exact current grounding_ref
    // or "remaining" range, which a later slice is free to move forward.
    let matrix = seeded_current_truth();
    let row = matrix
        .row("class.fighter.levels_2_10")
        .expect("row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(row.support_state, SupportState::Supported);
    assert_eq!(row.evidence_tier, EvidenceTier::ProductVisible);
    assert!(
        row.blocker_or_lossiness_note.contains("level-8")
            || row.blocker_or_lossiness_note.contains("L8")
            || row.blocker_or_lossiness_note.contains("level 8"),
        "levels-2-10 row blocker note must name the level-8 bonus-feat milestone: {}",
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
        "the level-8 slice must not promote any row to Supported or Lossy"
    );
}

// Preserve the level-7-only fixture as a still-valid, unchanged reference point.
#[test]
fn level_7_fixture_still_loads_and_computes_unaffected_by_the_level_8_widening() {
    let input = load(LEVEL_7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    assert_eq!(computation.base_attack_bonus, 7);
    assert_eq!(computation.baseline_melee_attack_bonus, 13);
    assert!(!computation.diagnostics.iter().any(|d| d.claim_blocking));
}
