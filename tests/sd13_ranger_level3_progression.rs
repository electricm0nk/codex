//! SD13-E5 Ranger level-3 progression grounding proof.
//!
//! Widens the accepted Ranger level-1/level-2 per-pillar decomposition
//! (`tests/sd13_ranger_level1_chassis_and_class_feature_separation.rs`,
//! `tests/sd13_ranger_base_attack_and_saves.rs`,
//! `tests/sd13_ranger_level2_progression.rs`) to Ranger level 3, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Sorcerer/Wizard
//! level-range-gate idiom (`supported_ranger_level` is generalized from `1..=2`
//! to `1..=3` via `MAX_SUPPORTED_RANGER_LEVEL = 3`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Ranger class table) were read directly before
//! writing any code or test:
//!
//! - level 3 base attack bonus is +3 (full BAB), base Fortitude/Reflex are +3
//!   (good), base Will is +1 (poor) — confirmed by the same formulas already
//!   grounded at levels 1-2 (`classlevel` and `classlevel/2+2` / `classlevel/3`),
//!   not re-derived.
//! - Track stays `max(ranger level / 2, 1) = 1` at level 3, reached naturally
//!   (`max(3/2, 1) = 1`), not via the level-1 floor.
//! - the Favored Enemy flat surface (choice recognition, +2 skill bonus, +2
//!   attack/damage bonus) is confirmed unchanged at level 3 — PF1 Core Rulebook
//!   only increases the Favored Enemy bonus at 4th ranger level and beyond, so
//!   level 3 stays at the flat +2 via the same formulas, not new records.
//! - the combat-style choice/bonus-feat recognition (a one-time 2nd-level grant)
//!   stays granted at level 3, not re-derived.
//! - the PF1 Core Rulebook Ranger class table's level-3 "Special" column reads
//!   "Endurance, 1st favored terrain" (verified independently against d20pfsrd
//!   and legacy.aonprd.com). Endurance is grounded as a bounded grant-only
//!   identity record (value 0, non-fabricated): the ranger gains Endurance as a
//!   bonus feat automatically, with no player choice involved, mirroring the
//!   Wizard Scribe Scroll / Barbarian Uncanny Dodge idiom — no feat-effect
//!   execution engine exists anywhere in this codebase. Favored Terrain is
//!   deliberately left named-but-unproven this slice: it is a player choice of
//!   terrain type with a flat +2 bonus on Initiative/Knowledge (geography)/
//!   Perception/Stealth/Survival checks made in that terrain, which would
//!   require a NEW choice-slot with no existing fixture selection.
//!
//! It deliberately does not touch the favored-enemy conditional-application
//! engine, the combat-style bonus feat's own mechanics, Favored Terrain, the
//! ranger spell burden, or Ranger level 4+ (all stay named-but-unproven,
//! unchanged from level 2), and it preserves the accepted Ranger level-1/level-2
//! truth, the Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const RANGER_LEVEL2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level2_sd13_deterministic_input.txt");

const RANGER_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level3_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BASE_ATTACK_ID: &str = "class_chassis.ranger.base_attack_bonus";
const BASE_SAVE_FORTITUDE_ID: &str = "class_chassis.ranger.base_save.fortitude";
const BASE_SAVE_REFLEX_ID: &str = "class_chassis.ranger.base_save.reflex";
const BASE_SAVE_WILL_ID: &str = "class_chassis.ranger.base_save.will";
const TRACK_ID: &str = "class_chassis.ranger.track";
const FAVORED_ENEMY_SKILL_BONUS_ID: &str = "class_chassis.ranger.favored_enemy_skill_bonus";
const FAVORED_ENEMY_ATTACK_DAMAGE_BONUS_ID: &str =
    "class_chassis.ranger.favored_enemy_attack_damage_bonus";
const COMBAT_STYLE_CHOICE_ID: &str = "class_chassis.ranger.combat_style_choice";
const COMBAT_STYLE_BONUS_FEAT_CHOICE_ID: &str =
    "class_chassis.ranger.combat_style_bonus_feat_choice";
const ENDURANCE_ID: &str = "class_feature.ranger.endurance";

// ----- Base attack bonus at level 3 -----

#[test]
fn ranger_level3_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(RANGER_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 3,
        "Ranger level 3 full-BAB progression (classlevel) must equal 3: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 3 (good Fortitude/Reflex, poor Will) -----

#[test]
fn ranger_level3_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(RANGER_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 3, "Ranger level 3 good Fortitude (3/2+2) must equal 3");

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 3, "Ranger level 3 good Reflex (3/2+2) must equal 3");

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 1, "Ranger level 3 poor Will (3/3) must equal 1");
}

// ----- Track at level 3 (stays 1, reached naturally not via the level-1 floor) -----

#[test]
fn ranger_level3_track_stays_one_reached_naturally() {
    let input = load(RANGER_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let track = explanation(&computation, TRACK_ID);
    assert_eq!(
        track.value, 1,
        "Ranger level 3 Track bonus (max(3/2, 1)) must equal 1: {}",
        track.detail
    );
}

// ----- Favored Enemy flat surface at level 3 (unchanged) -----

#[test]
fn ranger_level3_favored_enemy_flat_surface_is_unchanged() {
    let input = load(RANGER_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let skill = explanation(&computation, FAVORED_ENEMY_SKILL_BONUS_ID);
    assert_eq!(
        skill.value, 2,
        "Ranger level 3 favored-enemy skill bonus must stay the flat +2 (PF1 CRB increases only \
         at 4th ranger level): {}",
        skill.detail
    );

    let attack_damage = explanation(&computation, FAVORED_ENEMY_ATTACK_DAMAGE_BONUS_ID);
    assert_eq!(
        attack_damage.value, 2,
        "Ranger level 3 favored-enemy attack/damage bonus must stay the flat +2: {}",
        attack_damage.detail
    );
}

// ----- Combat style stays granted at level 3, not re-derived -----

#[test]
fn ranger_level3_keeps_combat_style_grounded() {
    let input = load(RANGER_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let style_choice = explanation(&computation, COMBAT_STYLE_CHOICE_ID);
    assert_eq!(
        style_choice.value, 0,
        "combat-style choice recognition must carry no fabricated mechanical value at level 3: {}",
        style_choice.value
    );
    assert!(
        has_explanation(&computation, COMBAT_STYLE_BONUS_FEAT_CHOICE_ID),
        "combat-style bonus-feat recognition must stay granted at level 3: {:?}",
        computation.explanations
    );
}

// ----- Endurance is granted at level 3, as a grant-only identity record -----

#[test]
fn ranger_level3_grounds_endurance_as_a_grant_only_identity_record() {
    let input = load(RANGER_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let endurance = explanation(&computation, ENDURANCE_ID);
    assert_eq!(
        endurance.value, 0,
        "Endurance must carry no fabricated mechanical value at level 3: {}",
        endurance.detail
    );
    assert!(
        endurance.detail.contains("Endurance"),
        "endurance explanation must name the Endurance class feature: {}",
        endurance.detail
    );
    assert!(
        endurance.detail.to_lowercase().contains("granted"),
        "endurance explanation at level 3 must state it is granted, not absent: {}",
        endurance.detail
    );
    assert!(
        endurance.detail.to_lowercase().contains("no player choice")
            || endurance.detail.to_lowercase().contains("automatically"),
        "endurance explanation must state the grant is automatic, not a player choice: {}",
        endurance.detail
    );
}

#[test]
fn ranger_level2_endurance_is_a_correct_level_gate_absence() {
    let input = load(RANGER_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let endurance = explanation(&computation, ENDURANCE_ID);
    assert_eq!(
        endurance.value, 0,
        "Endurance at level 2 must be a correct level-gate absence, value 0: {}",
        endurance.detail
    );
    assert!(
        endurance.detail.to_lowercase().contains("absent"),
        "endurance explanation at level 2 must state it is correctly absent: {}",
        endurance.detail
    );
}

// ----- Negative control: level 4 was later widened into the supported tranche -----

#[test]
fn ranger_level_4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_ranger_level4_progression.rs) widened the level-range gate to
    // level 4 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk level-range
    // gate idiom) and grounded Hunter's Bond; this negative control is
    // superseded, not violated — pin the new truth here too so this file stays
    // internally consistent. The frontier this file's own slice actually drew
    // is now level 5, covered by
    // `tests/sd13_ranger_level4_progression.rs::ranger_level_5_is_not_promoted_by_this_slice`.
    let level_4 = RANGER_LEVEL3_FIXTURE.replace("class:ranger:3", "class:ranger:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, BASE_ATTACK_ID),
        "level-4 Ranger is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, ENDURANCE_ID),
        "level-4 Ranger must keep the Endurance explanation grounded at level 3"
    );
}

// ----- Negative control: the grounding must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_level3_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.") || e.id == ENDURANCE_ID),
        "the Fighter chassis must not surface any ranger-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Ranger is not promoted -----

#[test]
fn multiclass_ranger_level3_is_not_promoted_by_this_slice() {
    let multiclass = RANGER_LEVEL3_FIXTURE.replace(
        "class_level=class:ranger:3",
        "class_level=class:ranger:3\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.") || e.id == ENDURANCE_ID),
        "multiclass Ranger must not gain any bounded ranger chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Ranger must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-3 widening and Endurance -----

#[test]
fn matrix_ranger_row_names_level_3_widening_and_endurance() {
    let matrix = seeded_current_truth();
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger row must exist");

    assert_eq!(ranger.support_state, SupportState::Supported);
    assert_eq!(ranger.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        ranger.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        ranger.grounding_ref.contains("sd13_ranger_level3_progression"),
        "ranger row must cite the live SD13-E5 level-3 proof surface: {}",
        ranger.grounding_ref
    );

    let note = ranger.blocker_or_lossiness_note;
    assert!(
        note.contains("level 3") || note.contains("level-3"),
        "ranger partial note must name the level-3 widening: {note}"
    );
    assert!(
        note.to_lowercase().contains("endurance"),
        "ranger partial note must name Endurance as newly grounded: {note}"
    );
    assert!(
        note.contains("Favored Terrain"),
        "ranger partial note must name Favored Terrain as still-unproven: {note}"
    );
    // The still-unproven burdens stay named.
    for token in ["spell", "conditional-application"] {
        assert!(
            note.contains(token),
            "ranger partial note must still name the unproven '{token}' burden: {note}"
        );
    }
}
