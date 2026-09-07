//! SD13-E5 Fighter level 9 and level 10 milestone progression proof.
//!
//! Widens the accepted deterministic Human Fighter pilot seam from the bounded
//! levels 1-8 tranche to levels 9 and 10: base attack / base save progression
//! continues generically, level 9 grants Weapon Training 2 (the first weapon
//! group's attack bonus rises to +2 and a second weapon group is chosen at +1 —
//! the earlier "no new PF1 milestone at level 9" claim was false and is
//! corrected here), and level 10 grants the next bonus feat in the cadence
//! (1, 2, 4, 6, 8, 10), canonically Greater Weapon Focus, whose prerequisites
//! (Weapon Focus (longsword) and fighter level 8) the canonical loadout
//! honestly meets.
//!
//! It is intentionally not a broad martial engine. It grounds only the level-9
//! and level-10 base attack / base save deltas, the Weapon Training 2
//! attack-roll half, and the level-10 bonus-feat seam. The Weapon Training
//! damage-roll half stays unproven (no damage total is computed anywhere in
//! this codebase), Bravery stays unproven (no Will-vs-fear total exists), and
//! it asserts no level-11+ Fighter burden, no spell burden, no non-Fighter
//! positive support, and no general feat/prerequisite engine.

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

const LEVEL_9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level9_sd13_deterministic_input.txt");
const LEVEL_10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level10_sd13_deterministic_input.txt");
const LEVEL_8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level8_sd13_deterministic_input.txt");

// ----- Milestone: level 9 is no longer blanket-blocked and carries Weapon Training 2 -----

#[test]
fn level_9_human_fighter_produces_non_blocked_bounded_evidence() {
    let input = load(LEVEL_9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic level-9 Fighter must not block claims: {:?}",
        computation.diagnostics
    );

    // Fighter level-9 base chassis: full BAB +9, good Fortitude +6, poor Reflex/Will +3.
    assert_eq!(computation.base_attack_bonus, 9);
    assert_eq!(computation.base_saves.fortitude, 6);
    assert_eq!(computation.base_saves.reflex, 3);
    assert_eq!(computation.base_saves.will, 3);

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(bab.value, 9);
    assert!(
        bab.detail.contains("level 9"),
        "level-9 BAB explanation must name the level-9 delta: {}",
        bab.detail
    );

    // Total saves: base + relevant ability modifier (CON +2, DEX +2, WIS +1).
    assert_eq!(computation.total_saves.fortitude, 8);
    assert_eq!(computation.total_saves.reflex, 5);
    assert_eq!(computation.total_saves.will, 4);

    // Baseline combat: +9 BAB + STR +3 + Weapon Focus +1 + Weapon Training 2 (+2) = 15.
    assert_eq!(computation.baseline_melee_attack_bonus, 16);
    // Armor class is unchanged from level 8: no new armor-training milestone lands
    // at level 9, and the deterministic +2 Dexterity contribution is already below
    // both the Armor Training 1 and Armor Training 2 max-Dex caps.
    assert_eq!(computation.baseline_armor_class, 17);

    // No new armor-training milestone lands at level 9, so the Climb/Intimidate/Swim
    // selected-skill totals stay exactly as they were at level 8 (CG-03 fix: 8, not 7,
    // since STR modifier is now +4).
    assert_eq!(computation.selected_skill_modifiers.climb, 8);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    assert_eq!(computation.selected_skill_modifiers.swim, 8);
}

#[test]
fn level_9_weapon_training_2_raises_the_first_group_attack_bonus_to_2() {
    let input = load(LEVEL_9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let seam = explanation(&computation, "class_feature.fighter.weapon_training");
    assert_eq!(
        seam.value, 2,
        "Weapon Training 2 must carry the +2 attack-bonus value it folds into \
         the baseline melee attack bonus for the first weapon group: {seam:?}"
    );
    assert!(
        seam.detail.contains("heavy_blades") || seam.detail.contains("Heavy Blades"),
        "weapon training seam must name the chosen first weapon group: {}",
        seam.detail
    );
    assert!(
        seam.detail.contains("damage"),
        "weapon training seam must keep naming the unproven damage-roll half: {}",
        seam.detail
    );
}

#[test]
fn level_9_second_weapon_training_group_is_an_explanation_only_seam() {
    let input = load(LEVEL_9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let seam = explanation(&computation, "class_feature.fighter.weapon_training_group_2");
    assert_eq!(
        seam.value, 1,
        "the second weapon-training group gets the rank-minus-one +1 bonus: {seam:?}"
    );
    assert!(
        seam.detail.contains("group:bows") || seam.detail.contains("Bows"),
        "second weapon-training group seam must name the canonical bows selection: {}",
        seam.detail
    );

    // The +1 for the bows group must NOT leak into the Longsword baseline: the
    // baseline melee attack bonus uses only the heavy-blades rank (+2).
    assert_eq!(computation.baseline_melee_attack_bonus, 16);
}

#[test]
fn non_canonical_second_weapon_training_group_is_claim_blocked() {
    let non_canonical = LEVEL_9_FIXTURE.replace(
        "choice:fighter_weapon_training_group_2:group:bows",
        "choice:fighter_weapon_training_group_2:group:axes",
    );
    let input = load(&non_canonical);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.claim_blocking && d.id.contains("fighter_weapon_training_group_2")),
        "non-canonical second weapon-training-group selection must be claim-blocked: {:?}",
        computation.diagnostics
    );
}

// ----- Milestone: level 10 is no longer blanket-blocked and carries the bonus feat -----

#[test]
fn level_10_human_fighter_produces_non_blocked_bounded_evidence() {
    let input = load(LEVEL_10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic level-10 Fighter must not block claims: {:?}",
        computation.diagnostics
    );

    // Fighter level-10 base chassis: full BAB +10, good Fortitude +7, poor Reflex/Will +3.
    assert_eq!(computation.base_attack_bonus, 10);
    assert_eq!(computation.base_saves.fortitude, 7);
    assert_eq!(computation.base_saves.reflex, 3);
    assert_eq!(computation.base_saves.will, 3);

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(bab.value, 10);
    assert!(
        bab.detail.contains("level 10"),
        "level-10 BAB explanation must name the level-10 delta: {}",
        bab.detail
    );

    // Total saves: base + relevant ability modifier (CON +2, DEX +2, WIS +1).
    assert_eq!(computation.total_saves.fortitude, 9);
    assert_eq!(computation.total_saves.reflex, 5);
    assert_eq!(computation.total_saves.will, 4);

    // Baseline combat: +10 BAB + STR +3 + Weapon Focus +1 + Weapon Training 2 (+2) = 16.
    assert_eq!(computation.baseline_melee_attack_bonus, 17);
    assert_eq!(computation.baseline_armor_class, 17);

    // No new armor-training milestone lands at level 10, so the selected-skill
    // totals stay exactly as they were at level 9 (CG-03 fix: 8, not 7).
    assert_eq!(computation.selected_skill_modifiers.climb, 8);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    assert_eq!(computation.selected_skill_modifiers.swim, 8);
}

#[test]
fn level_10_bonus_feat_seam_is_explicit_and_bounded() {
    let input = load(LEVEL_10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let seam = explanation(&computation, "class_feature.fighter.level_10_bonus_feat");
    assert_eq!(
        seam.value, 0,
        "level-10 bonus-feat seam must contribute no fabricated mechanical value: {seam:?}"
    );
    assert!(
        seam.detail.contains("choice:fighter_bonus_feat_10"),
        "level-10 bonus-feat seam must name the level-10 bonus-feat choice set: {}",
        seam.detail
    );
}

#[test]
fn non_canonical_level_10_bonus_feat_is_claim_blocked() {
    let non_canonical = LEVEL_10_FIXTURE.replace(
        "choice:fighter_bonus_feat_10:feat:greater_weapon_focus",
        "choice:fighter_bonus_feat_10:feat:vital_strike",
    );
    let input = load(&non_canonical);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.claim_blocking && d.id.contains("fighter_bonus_feat_10")),
        "non-canonical level-10 bonus-feat selection must be claim-blocked: {:?}",
        computation.diagnostics
    );
}

#[test]
fn level_10_still_carries_every_earlier_seam() {
    let input = load(LEVEL_10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        "class_feature.fighter.level_2_bonus_feat",
        "class_feature.fighter.level_4_bonus_feat",
        "class_feature.fighter.level_6_bonus_feat",
        "class_feature.fighter.level_8_bonus_feat",
        "class_feature.fighter.armor_training",
        "class_feature.fighter.weapon_training",
        "class_feature.fighter.weapon_training_group_2",
    ] {
        assert!(
            has_explanation(&computation, id),
            "level-10 Fighter must still carry the earlier seam '{id}': {:?}",
            computation.explanations
        );
    }

    // Armor Training 2 (not a fresh Armor Training 3) is still the seam named at
    // level 10, since no new armor-training rank lands beyond level 7.
    let armor_training = explanation(&computation, "class_feature.fighter.armor_training");
    assert_eq!(armor_training.value, 2);
}

// ----- Milestone: downstream propagation of the widened bounded truth -----

#[test]
fn level_10_propagates_computed_receipt_and_view_model() {
    let input = load(LEVEL_10_FIXTURE);
    let receipt = build_pilot_headless_receipt(&input);

    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Computed,
        "widened level-10 Fighter tranche must propagate a computed receipt"
    );

    assert!(
        has_explanation(&receipt.computation, "race.human.ability_bonus_target"),
        "widened level-10 receipt must preserve the Human ability-bonus race seam: {:?}",
        receipt.computation.explanations
    );

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Computed);
    assert_eq!(view_model.primary_owner, PrimaryOwner::OracleGap);
    let snapshot = view_model
        .snapshot
        .as_ref()
        .expect("computed level-10 receipt must yield a snapshot");
    assert_eq!(
        snapshot.base_attack_bonus, receipt.computation.base_attack_bonus,
        "view-model snapshot must carry the widened level-10 base attack bonus"
    );
}

// ----- Negative control: level 21 stays blocked (PF1 has no 21st character level) -----
//
// SD18 (tests/sd18_fighter_level11_armor_training3.rs,
// tests/sd18_fighter_level12_widening.rs, tests/sd18_fighter_level13_widening.rs,
// tests/sd18_fighter_level14_widening.rs, tests/sd18_fighter_level15_widening.rs,
// tests/sd18_fighter_level16_widening.rs, tests/sd18_fighter_level17_widening.rs,
// tests/sd18_fighter_level18_widening.rs, tests/sd18_fighter_level19_widening.rs,
// tests/sd18_fighter_level20_widening.rs)
// widened the bounded tranche from level 10 to level 12 (Armor Training 3,
// then a sixth bonus-feat cadence slot), then to level 13 (Weapon Training
// 3), then to level 14 (a seventh bonus-feat cadence slot and the Bravery
// magnitude rise), then to level 15 (Armor Training 4), then to level 16
// (an eighth bonus-feat cadence slot), then to level 17 (Weapon Training
// 4), then to level 18 (a ninth bonus-feat cadence slot and a further
// Bravery magnitude rise), then to level 19 (the Armor Mastery
// flat-magnitude damage reduction record), and then to level 20 (a tenth
// bonus-feat cadence slot and the Weapon Mastery grant-only capstone
// record) -- the FINAL level within PF1's 1-20 character-level cap -- so
// this negative control now sits just above the current bound (level 21,
// which does not exist as a PF1 character level) rather than at level 12,
// level 13, level 14, level 15, level 16, level 17, level 18, level 19, or
// level 20.

#[test]
fn level_21_fighter_stays_claim_blocked() {
    let level_21 = LEVEL_10_FIXTURE.replace("class:fighter:10", "class:fighter:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-21 Fighter must stay claim-blocked beyond the bounded levels-2-20 row: {:?}",
        computation.diagnostics
    );
    assert!(
        !has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "level-21 Fighter must not fabricate a base-attack-bonus explanation"
    );
}

// ----- Negative control: non-Fighter classes don't leak the new explanations -----

#[test]
fn non_fighter_class_does_not_leak_level_10_seams() {
    // A single-class Paladin at the hybrid baseline level must never carry the
    // Fighter-only level-10 bonus-feat or second weapon-training-group seams,
    // even though both classes are recognized identities on this rules-core
    // surface.
    let paladin_fixture = LEVEL_10_FIXTURE
        .replace("class:fighter:10", "class:paladin:1")
        .replace(
            "case_id=pf1-crb-human-fighter-level10",
            "case_id=pf1-crb-human-paladin-level1",
        );
    let input = load(&paladin_fixture);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        "class_feature.fighter.level_10_bonus_feat",
        "class_feature.fighter.weapon_training_group_2",
    ] {
        assert!(
            !has_explanation(&computation, id),
            "non-Fighter class must not leak the Fighter-only seam '{id}': {:?}",
            computation.explanations
        );
    }
}

// ----- Control plane: the matrix widens the levels-2-10 row's proven range to level 10 -----

#[test]
fn matrix_levels_2_10_names_levels_9_and_10_as_proven_and_the_honest_remaining_burdens() {
    let matrix = seeded_current_truth();
    let row = matrix
        .row("class.fighter.levels_2_10")
        .expect("row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(row.support_state, SupportState::Supported);
    assert_eq!(row.evidence_tier, EvidenceTier::ProductVisible);
    assert!(
        row.grounding_ref
            .contains("sd13_fighter_level9_level10_progression"),
        "levels-2-10 row must cite the live SD13-E5 level-9/level-10 proof surface: {}",
        row.grounding_ref
    );
    assert!(
        row.blocker_or_lossiness_note.contains("Weapon Training 2"),
        "levels-2-10 row note must name the level-9 Weapon Training 2 milestone: {}",
        row.blocker_or_lossiness_note
    );
    assert!(
        row.blocker_or_lossiness_note.contains("level-10 bonus")
            || row.blocker_or_lossiness_note.contains("level 10 bonus"),
        "levels-2-10 row note must name the level-10 bonus-feat milestone: {}",
        row.blocker_or_lossiness_note
    );
    // The row must NOT keep the falsified "no new PF1 milestone at level 9" claim.
    assert!(
        !row.blocker_or_lossiness_note
            .contains("no new class-feature milestone at level 9"),
        "levels-2-10 row must drop the falsified level-9 no-milestone claim: {}",
        row.blocker_or_lossiness_note
    );
    // Honest remaining burdens: the Weapon Training damage-roll half and Bravery.
    assert!(
        row.blocker_or_lossiness_note.contains("damage"),
        "levels-2-10 row must keep naming the unproven Weapon Training damage-roll half: {}",
        row.blocker_or_lossiness_note
    );
    assert!(
        row.blocker_or_lossiness_note.contains("Bravery"),
        "levels-2-10 row must name the unproven Bravery milestone (+1 Will vs fear at L2, \
         +2 at L6, +3 at L10): {}",
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
        "the level-9/level-10 slice must not promote any row to Supported or Lossy"
    );
}

// Preserve the level-8-only fixture as a still-valid, unchanged reference point.
#[test]
fn level_8_fixture_still_loads_and_computes_unaffected_by_the_level_9_10_widening() {
    let input = load(LEVEL_8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    assert_eq!(computation.base_attack_bonus, 8);
    assert_eq!(computation.baseline_melee_attack_bonus, 14);
    assert!(!computation.diagnostics.iter().any(|d| d.claim_blocking));
}
