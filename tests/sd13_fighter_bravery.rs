//! SD13-E5 Fighter Bravery milestone proof.
//!
//! Grounds Bravery as a flat, non-fabricated numeric magnitude only: PF1 Core
//! Rulebook Fighter Bravery grants +1 Will save vs fear at level 2, and another
//! +1 for every four fighter levels thereafter (level 6 -> +2, level 10 -> +3).
//! This mirrors the Weapon Training attack-bonus rank idiom exactly — the bonus
//! magnitude is recorded as a `ComputationExplanation`, but no fear-condition or
//! save-resolution engine exists anywhere in this codebase, so the Will-vs-fear
//! total itself stays unproven and unclaimed here. It is intentionally not a
//! save-resolution engine: it asserts no fear-condition handling, no rolled Will
//! save, and no leakage onto non-Fighter classes.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const LEVEL_1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");
const LEVEL_2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level2_sd13_deterministic_input.txt");
const LEVEL_5_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level5_sd13_deterministic_input.txt");
const LEVEL_6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level6_sd13_deterministic_input.txt");
const LEVEL_9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level9_sd13_deterministic_input.txt");
const LEVEL_10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level10_sd13_deterministic_input.txt");

const BRAVERY_ID: &str = "class_feature.fighter.bravery";

// ----- Milestone: level 1 Fighter has no Bravery bonus yet -----

#[test]
fn level_1_fighter_has_no_bravery_seam() {
    let input = load(LEVEL_1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, BRAVERY_ID),
        "level-1 Fighter must not carry a Bravery seam (Bravery is gained at level 2): {:?}",
        computation.explanations
    );
}

// ----- Milestone: level 2 grants +1 Bravery -----

#[test]
fn level_2_fighter_bravery_is_plus_1() {
    let input = load(LEVEL_2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let seam = explanation(&computation, BRAVERY_ID);
    assert_eq!(
        seam.value, 1,
        "Fighter level 2 Bravery must be +1 Will save vs fear: {seam:?}"
    );
    assert!(
        seam.detail.to_lowercase().contains("fear"),
        "Bravery seam must name the Will-vs-fear bonus it explains: {}",
        seam.detail
    );
}

// ----- Milestone: level 5 still carries +1 Bravery (no new rank until level 6) -----

#[test]
fn level_5_fighter_bravery_is_still_plus_1() {
    let input = load(LEVEL_5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let seam = explanation(&computation, BRAVERY_ID);
    assert_eq!(
        seam.value, 1,
        "Fighter level 5 Bravery must still be +1 (next rank is level 6): {seam:?}"
    );
}

// ----- Milestone: level 6 raises Bravery to +2 -----

#[test]
fn level_6_fighter_bravery_is_plus_2() {
    let input = load(LEVEL_6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let seam = explanation(&computation, BRAVERY_ID);
    assert_eq!(
        seam.value, 2,
        "Fighter level 6 Bravery must rise to +2: {seam:?}"
    );
}

// ----- Milestone: level 9 still carries +2 Bravery (no new rank until level 10) -----

#[test]
fn level_9_fighter_bravery_is_still_plus_2() {
    let input = load(LEVEL_9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let seam = explanation(&computation, BRAVERY_ID);
    assert_eq!(
        seam.value, 2,
        "Fighter level 9 Bravery must still be +2 (next rank is level 10): {seam:?}"
    );
}

// ----- Milestone: level 10 raises Bravery to +3 -----

#[test]
fn level_10_fighter_bravery_is_plus_3() {
    let input = load(LEVEL_10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let seam = explanation(&computation, BRAVERY_ID);
    assert_eq!(
        seam.value, 3,
        "Fighter level 10 Bravery must rise to +3: {seam:?}"
    );

    // Sibling seams from the level-9/level-10 slice must still be present alongside
    // the new Bravery seam.
    for id in [
        "class_feature.fighter.level_10_bonus_feat",
        "class_feature.fighter.weapon_training",
        "class_feature.fighter.weapon_training_group_2",
    ] {
        assert!(
            has_explanation(&computation, id),
            "level-10 Fighter must still carry the earlier seam '{id}' alongside Bravery: {:?}",
            computation.explanations
        );
    }
}

// ----- Negative control: no rolled Will save or fear-condition total exists -----

#[test]
fn bravery_seam_contributes_no_fabricated_will_save_total() {
    let input = load(LEVEL_10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The Bravery seam is a standalone magnitude record only. It must not be
    // folded into the generic Will save total, since Bravery only applies
    // against fear effects, and no fear-condition engine exists in this
    // codebase to gate that conditionally.
    let bravery = explanation(&computation, BRAVERY_ID);
    assert_eq!(computation.total_saves.will, computation.base_saves.will + 1);
    assert_ne!(
        computation.total_saves.will,
        computation.base_saves.will + bravery.value,
        "Bravery must not be silently folded into the unconditional Will save total \
         (it only applies vs fear, which is not modeled here)"
    );
}

// ----- Negative control: non-Fighter classes don't leak the Bravery seam -----

#[test]
fn non_fighter_class_does_not_leak_bravery_seam() {
    let paladin_fixture = LEVEL_2_FIXTURE
        .replace("class:fighter:2", "class:paladin:1")
        .replace(
            "case_id=pf1-crb-human-fighter-level2",
            "case_id=pf1-crb-human-paladin-level1",
        );
    let input = load(&paladin_fixture);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, BRAVERY_ID),
        "non-Fighter class must not leak the Fighter-only Bravery seam: {:?}",
        computation.explanations
    );
}

// ----- Control plane: the matrix names Bravery as grounded, not unproven -----

#[test]
fn matrix_levels_2_10_names_bravery_as_grounded() {
    let matrix = seeded_current_truth();
    let row = matrix
        .row("class.fighter.levels_2_10")
        .expect("row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(row.support_state, SupportState::Supported);
    assert_eq!(row.evidence_tier, EvidenceTier::ProductVisible);
    assert!(
        row.blocker_or_lossiness_note.contains("Bravery"),
        "levels-2-10 row note must still name Bravery: {}",
        row.blocker_or_lossiness_note
    );
    assert!(
        !row.blocker_or_lossiness_note
            .contains("Bravery stays unproven"),
        "levels-2-10 row must drop the now-false 'Bravery stays unproven' claim: {}",
        row.blocker_or_lossiness_note
    );
    // The Weapon Training damage-roll half and the general feat-effect engine
    // must remain honestly named as still-unproven.
    assert!(
        row.blocker_or_lossiness_note.contains("damage"),
        "levels-2-10 row must keep naming the unproven Weapon Training damage-roll half: {}",
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
        "the Bravery slice must not promote any row to Supported or Lossy"
    );
}
