//! SD13-E5 Ranger level-7 progression grounding proof.
//!
//! Widens the accepted Ranger level-1/level-2/level-3/level-4/level-5/level-6
//! per-pillar decomposition
//! (`tests/sd13_ranger_level1_chassis_and_class_feature_separation.rs`,
//! `tests/sd13_ranger_base_attack_and_saves.rs`, `tests/sd13_ranger_level2_progression.rs`,
//! `tests/sd13_ranger_level3_progression.rs`, `tests/sd13_ranger_favored_terrain_choice.rs`,
//! `tests/sd13_ranger_level4_progression.rs`, `tests/sd13_ranger_level5_progression.rs`,
//! `tests/sd13_ranger_level6_progression.rs`) to Ranger level 7, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Sorcerer/Wizard
//! level-range-gate idiom (`supported_ranger_level` is generalized from `1..=6`
//! to `1..=7` via `MAX_SUPPORTED_RANGER_LEVEL = 7`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Ranger class table) were read directly before
//! writing any code or test:
//!
//! - level 7 base attack bonus is +7 (full BAB), base Fortitude/Reflex are +5
//!   (good, `7/2+2 = 5`, numerically unchanged from level 6 -- an
//!   integer-division coincidence, re-verified rather than assumed), and base
//!   Will is +2 (poor, `7/3 = 2`, also numerically unchanged from level 6) --
//!   all via the same formulas already grounded at levels 1-6, not re-derived.
//! - Track stays `max(7/2, 1) = 3` at level 7, numerically unchanged from level
//!   6 -- another integer-division coincidence, re-verified rather than assumed.
//! - the Favored Enemy rule's next interval (10th level) has not yet arrived, so
//!   both favored enemies' bonuses stay exactly as they were at level 6 (no new
//!   favored-enemy selection, no new bonus-increase choice) -- re-verified
//!   against both primary sources rather than assumed.
//! - the Combat Style Feat rule's own bonus-feat cadence (2nd, 6th, 10th, 14th,
//!   18th level) confirms 7th level grants no new combat-style bonus feat --
//!   both recognized bonus feats (2nd- and 6th-level) stay granted unchanged.
//! - the PF1 Core Rulebook Ranger class table's level-7 "Special" column reads
//!   "Woodland stride" only (verified independently against both primary
//!   sources). Woodland Stride ("a ranger may move through any sort of
//!   undergrowth ... at his normal speed and without taking damage or suffering
//!   any other impairment ... magically manipulated undergrowth ... still
//!   affects him normally") is an automatic, no-choice class feature with no
//!   numeric magnitude of its own -- unlike Track or Favored Terrain, it is a
//!   pure boolean grant, mirroring the Endurance grant-only identity idiom
//!   exactly (not the Barbarian Damage-Reduction flat-magnitude idiom, since
//!   there is no magnitude to record). This slice grounds ONLY the grant
//!   identity (+0): no terrain-detection or movement-resolution engine exists
//!   in this codebase to determine whether the ranger is actually moving
//!   through undergrowth, so the rule's own effect is never applied.
//! - Endurance, Favored Terrain, combat style (both bonus feats), both favored
//!   enemies, and Hunter's Bond all stay granted at level 7, not re-derived.
//!
//! It deliberately does not touch the favored-enemy conditional-application
//! engine, either combat-style bonus feat's own mechanics, the Favored Terrain
//! level-8th/13th/18th breadth, Hunter's Bond ally-bonus application, the
//! animal-companion form, the ranger spell burden, or Ranger level 8+ (all stay
//! named-but-unproven, unchanged from level 6), and it preserves the accepted
//! Ranger level-1 through level-6 truth, the Fighter negative control, and the
//! multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const RANGER_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level7_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BASE_ATTACK_ID: &str = "class_chassis.ranger.base_attack_bonus";
const BASE_SAVE_FORTITUDE_ID: &str = "class_chassis.ranger.base_save.fortitude";
const BASE_SAVE_REFLEX_ID: &str = "class_chassis.ranger.base_save.reflex";
const BASE_SAVE_WILL_ID: &str = "class_chassis.ranger.base_save.will";
const TRACK_ID: &str = "class_chassis.ranger.track";
const FAVORED_ENEMY_SKILL_BONUS_ID: &str = "class_chassis.ranger.favored_enemy_skill_bonus";
const FAVORED_ENEMY_2_SKILL_BONUS_ID: &str = "class_chassis.ranger.favored_enemy_2_skill_bonus";
const COMBAT_STYLE_CHOICE_ID: &str = "class_chassis.ranger.combat_style_choice";
const COMBAT_STYLE_BONUS_FEAT_CHOICE_ID: &str =
    "class_chassis.ranger.combat_style_bonus_feat_choice";
const COMBAT_STYLE_BONUS_FEAT_2_CHOICE_ID: &str =
    "class_chassis.ranger.combat_style_bonus_feat_2_choice";
const ENDURANCE_ID: &str = "class_feature.ranger.endurance";
const FAVORED_TERRAIN_ID: &str = "class_feature.ranger.favored_terrain";
const HUNTERS_BOND_ID: &str = "class_feature.ranger.hunters_bond";
const HUNTERS_BOND_ALLY_BONUS_ID: &str = "class_chassis.ranger.hunters_bond_ally_bonus";
const WOODLAND_STRIDE_ID: &str = "class_feature.ranger.woodland_stride";

// ----- Base attack bonus at level 7 -----

#[test]
fn ranger_level7_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(RANGER_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 7,
        "Ranger level 7 full-BAB progression (classlevel) must equal 7: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 7 (unchanged from level 6, integer-division coincidence) -----

#[test]
fn ranger_level7_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(RANGER_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 5, "Ranger level 7 good Fortitude (7/2+2) must equal 5");

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 5, "Ranger level 7 good Reflex (7/2+2) must equal 5");

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 2, "Ranger level 7 poor Will (7/3) must equal 2");
}

// ----- Track at level 7 (unchanged from level 6, integer-division coincidence) -----

#[test]
fn ranger_level7_track_stays_at_three() {
    let input = load(RANGER_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let track = explanation(&computation, TRACK_ID);
    assert_eq!(
        track.value, 3,
        "Ranger level 7 Track bonus (max(7/2, 1)) must equal 3, unchanged from level 6: {}",
        track.detail
    );
}

// ----- Combat style and BOTH bonus feats stay granted, unchanged -----

#[test]
fn ranger_level7_keeps_combat_style_and_both_bonus_feats_grounded() {
    let input = load(RANGER_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, COMBAT_STYLE_CHOICE_ID),
        "combat-style recognition must stay granted at level 7"
    );
    let first_feat = explanation(&computation, COMBAT_STYLE_BONUS_FEAT_CHOICE_ID);
    assert_eq!(first_feat.value, 0);
    assert!(first_feat.detail.contains("Point-Blank Shot"));

    let second_feat = explanation(&computation, COMBAT_STYLE_BONUS_FEAT_2_CHOICE_ID);
    assert_eq!(second_feat.value, 0);
    assert!(second_feat.detail.contains("Manyshot"));
}

// ----- No THIRD combat-style bonus feat is fabricated at level 7 -----

#[test]
fn ranger_level7_does_not_fabricate_a_third_combat_style_bonus_feat() {
    let input = load(RANGER_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(
            &computation,
            "class_chassis.ranger.combat_style_bonus_feat_3_choice"
        ),
        "the PF1 CRB grants no THIRD combat-style bonus feat at level 7 (next is level 10); \
         no such record must be fabricated: {:?}",
        computation.explanations
    );
}

// ----- Favored Enemy, Endurance, Favored Terrain, and Hunter's Bond stay granted -----

#[test]
fn ranger_level7_keeps_favored_enemy_endurance_favored_terrain_and_hunters_bond_grounded() {
    let input = load(RANGER_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let first_skill = explanation(&computation, FAVORED_ENEMY_SKILL_BONUS_ID);
    assert_eq!(
        first_skill.value, 4,
        "the first favored enemy's bonus must stay at the widened +4 (targeted at level 5) \
         unchanged at level 7: {}",
        first_skill.detail
    );

    let second_skill = explanation(&computation, FAVORED_ENEMY_2_SKILL_BONUS_ID);
    assert_eq!(
        second_skill.value, 2,
        "the second favored enemy's bonus must stay at the flat +2 unchanged at level 7: {}",
        second_skill.detail
    );

    let endurance = explanation(&computation, ENDURANCE_ID);
    assert_eq!(endurance.value, 0);
    assert!(endurance.detail.to_lowercase().contains("granted"));

    let favored_terrain = explanation(&computation, FAVORED_TERRAIN_ID);
    assert_eq!(favored_terrain.value, 2);
    assert!(favored_terrain.detail.to_lowercase().contains("granted"));

    let hunters_bond = explanation(&computation, HUNTERS_BOND_ID);
    assert_eq!(hunters_bond.value, 0);
    assert!(hunters_bond.detail.to_lowercase().contains("granted"));

    let ally_bonus = explanation(&computation, HUNTERS_BOND_ALLY_BONUS_ID);
    assert_eq!(
        ally_bonus.value, 2,
        "Hunter's Bond ally-bonus magnitude must stay unchanged at level 7 (still half the \
         first favored enemy's +4 bonus): {}",
        ally_bonus.detail
    );
}

// ----- Woodland Stride is newly grounded at level 7 as a grant-only identity record -----

#[test]
fn ranger_level7_grounds_woodland_stride_as_a_grant_only_identity_record() {
    let input = load(RANGER_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let woodland_stride = explanation(&computation, WOODLAND_STRIDE_ID);
    assert_eq!(
        woodland_stride.value, 0,
        "Woodland Stride must carry no fabricated mechanical value: {}",
        woodland_stride.detail
    );
    assert!(
        woodland_stride.detail.to_lowercase().contains("granted"),
        "Woodland Stride must be recorded as granted at level 7: {}",
        woodland_stride.detail
    );
    assert!(
        woodland_stride.detail.to_lowercase().contains("undergrowth"),
        "Woodland Stride detail must name the undergrowth-movement rule: {}",
        woodland_stride.detail
    );
}

#[test]
fn ranger_level7_woodland_stride_absent_below_the_gate() {
    let level_6 = RANGER_LEVEL7_FIXTURE.replace("class:ranger:7", "class:ranger:6");
    let input = load(&level_6);
    let computation = compute_pilot_base_chassis(&input);

    let woodland_stride = explanation(&computation, WOODLAND_STRIDE_ID);
    assert_eq!(woodland_stride.value, 0);
    assert!(
        woodland_stride.detail.to_lowercase().contains("absent"),
        "Woodland Stride must be a correct level-gate absence below level 7: {}",
        woodland_stride.detail
    );
}

// ----- Level 8 was later widened into the supported tranche (see
// `tests/sd13_ranger_level8_progression.rs`) -----
//
// This supersedes what was originally this test's own negative control
// (`ranger_level_8_is_not_promoted_by_this_slice`), mirroring the
// Rogue/Barbarian/Monk precedent: once a level is itself promoted by a later
// slice, its former negative-control sibling flips to a positive assertion
// rather than staying stale.

#[test]
fn ranger_level_8_was_later_widened_into_the_supported_tranche() {
    let level_8 = RANGER_LEVEL7_FIXTURE.replace("class:ranger:7", "class:ranger:8");
    let input = load(&level_8);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")),
        "level-8 Ranger was later widened into the supported tranche and must now gain bounded \
         ranger chassis explanations: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, ENDURANCE_ID),
        "level-8 Ranger was later widened and must now carry the Endurance explanation"
    );
    assert!(
        has_explanation(&computation, FAVORED_TERRAIN_ID),
        "level-8 Ranger was later widened and must now carry the Favored Terrain explanation"
    );
    assert!(
        has_explanation(&computation, HUNTERS_BOND_ID),
        "level-8 Ranger was later widened and must now carry the Hunter's Bond explanation"
    );
    assert!(
        has_explanation(&computation, WOODLAND_STRIDE_ID),
        "level-8 Ranger was later widened and must now carry the Woodland Stride explanation"
    );
}

// ----- Negative control: the grounding must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_level7_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")
                || e.id == ENDURANCE_ID
                || e.id == FAVORED_TERRAIN_ID
                || e.id == HUNTERS_BOND_ID
                || e.id == WOODLAND_STRIDE_ID),
        "the Fighter chassis must not surface any ranger-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Ranger is not promoted -----

#[test]
fn multiclass_ranger_level7_is_not_promoted_by_this_slice() {
    let multiclass = RANGER_LEVEL7_FIXTURE.replace(
        "class_level=class:ranger:7",
        "class_level=class:ranger:7\nclass_level=class:fighter:1",
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
                || e.id == HUNTERS_BOND_ID
                || e.id == WOODLAND_STRIDE_ID),
        "multiclass Ranger must not gain any bounded ranger chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Ranger must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-7 widening -----

#[test]
fn matrix_ranger_row_names_level_7_widening_and_woodland_stride() {
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
        ranger.grounding_ref.contains("sd13_ranger_level7_progression"),
        "ranger row must cite the live SD13-E5 level-7 proof surface: {}",
        ranger.grounding_ref
    );

    let note = ranger.blocker_or_lossiness_note;
    assert!(
        note.contains("level 7") || note.contains("level-7"),
        "ranger partial note must name the level-7 widening: {note}"
    );
    assert!(
        note.to_lowercase().contains("woodland stride"),
        "ranger partial note must name Woodland Stride as newly grounded: {note}"
    );
    // The still-unproven burdens stay named.
    for token in ["spell", "conditional-application", "companion"] {
        assert!(
            note.to_lowercase().contains(token),
            "ranger partial note must still name the unproven '{token}' burden: {note}"
        );
    }
}
