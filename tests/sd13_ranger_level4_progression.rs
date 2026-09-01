//! SD13-E5 Ranger level-4 progression grounding proof.
//!
//! Widens the accepted Ranger level-1/level-2/level-3 per-pillar decomposition
//! (`tests/sd13_ranger_level1_chassis_and_class_feature_separation.rs`,
//! `tests/sd13_ranger_base_attack_and_saves.rs`,
//! `tests/sd13_ranger_level2_progression.rs`,
//! `tests/sd13_ranger_level3_progression.rs`,
//! `tests/sd13_ranger_favored_terrain_choice.rs`) to Ranger level 4, mirroring
//! the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Sorcerer/Wizard
//! level-range-gate idiom (`supported_ranger_level` is generalized from `1..=3`
//! to `1..=4` via `MAX_SUPPORTED_RANGER_LEVEL = 4`). Both PF1 CRB primary
//! sources (d20pfsrd and legacy.aonprd.com Ranger class table) were read
//! directly before writing any code or test:
//!
//! - level 4 base attack bonus is +4 (full BAB), base Fortitude/Reflex are +4
//!   (good), base Will is +1 (poor) -- confirmed by the same formulas already
//!   grounded at levels 1-3 (`classlevel` and `classlevel/2+2` / `classlevel/3`),
//!   not re-derived.
//! - Track becomes `max(4/2, 1) = 2` at level 4, the formula's first genuinely
//!   new value since level 1.
//! - the Favored Enemy flat surface (choice recognition, +2 skill bonus, +2
//!   attack/damage bonus) is confirmed unchanged at level 4 -- PF1 Core
//!   Rulebook only increases the Favored Enemy bonus at 5th ranger level and
//!   beyond (verified independently against both primary sources), so level 4
//!   stays at the flat +2 via the same formulas, not new records.
//! - the combat-style choice/bonus-feat recognition (a one-time 2nd-level
//!   grant) stays granted at level 4, not re-derived; the next combat-style
//!   feat is not granted until 6th level (verified independently against both
//!   primary sources), so nothing new is recognized at level 4.
//! - Endurance and Favored Terrain (both granted at 3rd level) both stay
//!   granted at level 4, not re-derived.
//! - the PF1 Core Rulebook Ranger class table's level-4 "Special" column reads
//!   "Hunter's bond" (verified independently against d20pfsrd and
//!   legacy.aonprd.com). Hunter's Bond is newly grounded: a restricted
//!   two-option choice recognition (`choice:ranger_hunters_bond` -> `form:bond`
//!   or `form:companion`, mirroring the combat-style choice idiom) as a bounded
//!   `+0` record, an unconditional grant-only identity record (mirroring the
//!   Endurance/Favored Terrain idiom), and -- only for the "bond" form -- a
//!   flat, non-applied magnitude equal to half the already-grounded Favored
//!   Enemy bonus (the ally-bonus alternative's own flat-shaped component). No
//!   move-action/action-economy engine, no ally-range-and-perception check, and
//!   no favored-enemy target-type matching is implemented; the "companion"
//!   form's own animal-companion stat block/advancement subsystem is
//!   deliberately left named-but-unproven, since it does not exist anywhere in
//!   this codebase.
//!
//! It deliberately does not touch the favored-enemy conditional-application
//! engine, the combat-style bonus feat's own mechanics, the Favored Terrain
//! level-8th/13th/18th breadth, Hunter's Bond ally-bonus application, the
//! animal-companion form, the ranger spell burden, or Ranger level 5+ (all stay
//! named-but-unproven, unchanged from level 3), and it preserves the accepted
//! Ranger level-1/level-2/level-3 truth, the Fighter negative control, and the
//! multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const RANGER_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level3_sd13_deterministic_input.txt");

const RANGER_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level4_sd13_deterministic_input.txt");

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
const FAVORED_TERRAIN_ID: &str = "class_feature.ranger.favored_terrain";
const HUNTERS_BOND_ID: &str = "class_feature.ranger.hunters_bond";
const HUNTERS_BOND_CHOICE_ID: &str = "class_chassis.ranger.hunters_bond_choice";
const HUNTERS_BOND_ALLY_BONUS_ID: &str = "class_chassis.ranger.hunters_bond_ally_bonus";

// ----- Base attack bonus at level 4 -----

#[test]
fn ranger_level4_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(RANGER_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 4,
        "Ranger level 4 full-BAB progression (classlevel) must equal 4: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 4 (good Fortitude/Reflex, poor Will) -----

#[test]
fn ranger_level4_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(RANGER_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 4, "Ranger level 4 good Fortitude (4/2+2) must equal 4");

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 4, "Ranger level 4 good Reflex (4/2+2) must equal 4");

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 1, "Ranger level 4 poor Will (4/3) must equal 1");
}

// ----- Track at level 4 (a genuinely new value) -----

#[test]
fn ranger_level4_track_becomes_two() {
    let input = load(RANGER_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let track = explanation(&computation, TRACK_ID);
    assert_eq!(
        track.value, 2,
        "Ranger level 4 Track bonus (max(4/2, 1)) must equal 2: {}",
        track.detail
    );
}

// ----- Favored Enemy flat surface at level 4 (unchanged) -----

#[test]
fn ranger_level4_favored_enemy_flat_surface_is_unchanged() {
    let input = load(RANGER_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let skill = explanation(&computation, FAVORED_ENEMY_SKILL_BONUS_ID);
    assert_eq!(
        skill.value, 2,
        "Ranger level 4 favored-enemy skill bonus must stay the flat +2 (PF1 CRB increases only \
         at 5th ranger level): {}",
        skill.detail
    );

    let attack_damage = explanation(&computation, FAVORED_ENEMY_ATTACK_DAMAGE_BONUS_ID);
    assert_eq!(
        attack_damage.value, 2,
        "Ranger level 4 favored-enemy attack/damage bonus must stay the flat +2: {}",
        attack_damage.detail
    );
}

// ----- Combat style stays granted at level 4, not re-derived -----

#[test]
fn ranger_level4_keeps_combat_style_grounded() {
    let input = load(RANGER_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let style_choice = explanation(&computation, COMBAT_STYLE_CHOICE_ID);
    assert_eq!(
        style_choice.value, 0,
        "combat-style choice recognition must carry no fabricated mechanical value at level 4: {}",
        style_choice.value
    );
    assert!(
        has_explanation(&computation, COMBAT_STYLE_BONUS_FEAT_CHOICE_ID),
        "combat-style bonus-feat recognition must stay granted at level 4: {:?}",
        computation.explanations
    );
}

// ----- Endurance and Favored Terrain both stay granted at level 4, not re-derived -----

#[test]
fn ranger_level4_keeps_endurance_grounded() {
    let input = load(RANGER_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let endurance = explanation(&computation, ENDURANCE_ID);
    assert_eq!(
        endurance.value, 0,
        "Endurance must carry no fabricated mechanical value at level 4: {}",
        endurance.detail
    );
    assert!(
        endurance.detail.to_lowercase().contains("granted"),
        "endurance explanation at level 4 must state it is granted, not absent: {}",
        endurance.detail
    );
}

#[test]
fn ranger_level4_keeps_favored_terrain_grounded() {
    let input = load(RANGER_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let favored_terrain = explanation(&computation, FAVORED_TERRAIN_ID);
    assert_eq!(
        favored_terrain.value, 2,
        "Favored Terrain must keep its flat +2 magnitude grounded at level 4: {}",
        favored_terrain.detail
    );
    assert!(
        favored_terrain.detail.to_lowercase().contains("granted"),
        "favored terrain explanation at level 4 must state it is granted, not absent: {}",
        favored_terrain.detail
    );
}

// ----- Hunter's Bond is newly grounded at level 4 -----

#[test]
fn ranger_level4_grounds_hunters_bond_choice_recognition() {
    let input = load(RANGER_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, HUNTERS_BOND_CHOICE_ID);
    assert_eq!(
        choice.value, 0,
        "Hunter's Bond form choice recognition must carry no fabricated mechanical value: {}",
        choice.detail
    );
    assert!(
        choice.detail.contains("hunting companions"),
        "hunter's bond choice explanation must name the selected form: {}",
        choice.detail
    );
}

#[test]
fn ranger_level4_grounds_hunters_bond_as_grant_only_identity_record() {
    let input = load(RANGER_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let hunters_bond = explanation(&computation, HUNTERS_BOND_ID);
    assert_eq!(
        hunters_bond.value, 0,
        "Hunter's Bond must carry no fabricated mechanical value at level 4: {}",
        hunters_bond.detail
    );
    assert!(
        hunters_bond.detail.contains("Hunter's Bond"),
        "hunter's bond explanation must name the Hunter's Bond class feature: {}",
        hunters_bond.detail
    );
    assert!(
        hunters_bond.detail.to_lowercase().contains("granted"),
        "hunter's bond explanation at level 4 must state it is granted, not absent: {}",
        hunters_bond.detail
    );
}

#[test]
fn ranger_level3_hunters_bond_is_a_correct_level_gate_absence() {
    let input = load(RANGER_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let hunters_bond = explanation(&computation, HUNTERS_BOND_ID);
    assert_eq!(
        hunters_bond.value, 0,
        "Hunter's Bond at level 3 must be a correct level-gate absence, value 0: {}",
        hunters_bond.detail
    );
    assert!(
        hunters_bond.detail.to_lowercase().contains("absent"),
        "hunter's bond explanation at level 3 must state it is correctly absent: {}",
        hunters_bond.detail
    );
    assert!(
        !has_explanation(&computation, HUNTERS_BOND_CHOICE_ID),
        "level-3 Ranger must not gain the Hunter's Bond choice explanation"
    );
    assert!(
        !has_explanation(&computation, HUNTERS_BOND_ALLY_BONUS_ID),
        "level-3 Ranger must not gain the Hunter's Bond ally-bonus explanation"
    );
}

#[test]
fn ranger_level4_grounds_hunters_bond_ally_bonus_for_the_bond_form_only() {
    let input = load(RANGER_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let ally_bonus = explanation(&computation, HUNTERS_BOND_ALLY_BONUS_ID);
    assert_eq!(
        ally_bonus.value, 1,
        "Hunter's Bond ally-bonus magnitude (half the favored-enemy bonus, 2 / 2) must equal 1: {}",
        ally_bonus.detail
    );
    assert!(
        ally_bonus.detail.to_lowercase().contains("move action"),
        "hunter's bond ally-bonus explanation must name the move-action grant: {}",
        ally_bonus.detail
    );
    assert!(
        ally_bonus.detail.to_lowercase().contains("no move-action")
            || ally_bonus.detail.to_lowercase().contains("action-economy"),
        "hunter's bond ally-bonus explanation must disclaim an action-economy engine: {}",
        ally_bonus.detail
    );
}

#[test]
fn ranger_hunters_bond_companion_form_grounds_no_ally_bonus() {
    let companion_form = RANGER_LEVEL4_FIXTURE.replace(
        "choice=choice:ranger_hunters_bond:form:bond",
        "choice=choice:ranger_hunters_bond:form:companion",
    );
    let input = load(&companion_form);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, HUNTERS_BOND_ALLY_BONUS_ID),
        "the 'companion' Hunter's Bond form must not fabricate an ally-bonus magnitude that only \
         applies to the 'bond' form: {:?}",
        computation.explanations
    );
    let choice = explanation(&computation, HUNTERS_BOND_CHOICE_ID);
    assert!(
        choice.detail.contains("animal companion"),
        "hunter's bond choice explanation must name the 'companion' form when selected: {}",
        choice.detail
    );
    assert!(
        has_explanation(&computation, HUNTERS_BOND_ID),
        "the grant-only Hunter's Bond identity record must still be present regardless of form"
    );
}

// ----- Historical control: level 5 was later widened into the supported tranche -----
//
// This test previously asserted level 5 stayed unrecognized by this slice. A later
// SD13-E5 slice (`tests/sd13_ranger_level5_progression.rs`) widened
// `supported_ranger_level` to include level 5, so the negative-control coverage moved
// there (including the new level-6 negative control). This test is retained, renamed,
// to document that the widening happened and to keep the level-4 fixture's own
// baseline behavior pinned.
#[test]
fn ranger_level_5_was_later_widened_into_the_supported_tranche() {
    let level_5 = RANGER_LEVEL4_FIXTURE.replace("class:ranger:4", "class:ranger:5");
    let input = load(&level_5);
    let computation = compute_pilot_base_chassis(&input);
    // Level 5 IS now recognized (widened by a later slice); the level-4 fixture's
    // ranger_favored_terrain/hunters_bond selections still carry over cleanly, so the
    // base chassis pillars are still recognized at level 5 too.
    assert!(
        has_explanation(&computation, ENDURANCE_ID),
        "level-5 Ranger (mutated from the level-4 fixture) is recognized by the later \
         level-5-widening slice: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the grounding must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_level4_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")
                || e.id == ENDURANCE_ID
                || e.id == FAVORED_TERRAIN_ID
                || e.id == HUNTERS_BOND_ID),
        "the Fighter chassis must not surface any ranger-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Ranger is not promoted -----

#[test]
fn multiclass_ranger_level4_is_not_promoted_by_this_slice() {
    let multiclass = RANGER_LEVEL4_FIXTURE.replace(
        "class_level=class:ranger:4",
        "class_level=class:ranger:4\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")
                || e.id == ENDURANCE_ID
                || e.id == FAVORED_TERRAIN_ID
                || e.id == HUNTERS_BOND_ID),
        "multiclass Ranger must not gain any bounded ranger chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Ranger must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-4 widening and Hunter's Bond -----

#[test]
fn matrix_ranger_row_names_level_4_widening_and_hunters_bond() {
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
        ranger.grounding_ref.contains("sd13_ranger_level4_progression"),
        "ranger row must cite the live SD13-E5 level-4 proof surface: {}",
        ranger.grounding_ref
    );

    let note = ranger.blocker_or_lossiness_note;
    assert!(
        note.contains("level 4") || note.contains("level-4"),
        "ranger partial note must name the level-4 widening: {note}"
    );
    assert!(
        note.to_lowercase().contains("hunter's bond") || note.to_lowercase().contains("hunters bond"),
        "ranger partial note must name Hunter's Bond as newly grounded: {note}"
    );
    // The still-unproven burdens stay named.
    for token in ["spell", "conditional-application", "companion"] {
        assert!(
            note.to_lowercase().contains(token),
            "ranger partial note must still name the unproven '{token}' burden: {note}"
        );
    }
}
