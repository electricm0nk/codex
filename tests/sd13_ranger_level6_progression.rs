//! SD13-E5 Ranger level-6 progression grounding proof.
//!
//! Widens the accepted Ranger level-1/level-2/level-3/level-4/level-5 per-pillar
//! decomposition (`tests/sd13_ranger_level1_chassis_and_class_feature_separation.rs`,
//! `tests/sd13_ranger_base_attack_and_saves.rs`, `tests/sd13_ranger_level2_progression.rs`,
//! `tests/sd13_ranger_level3_progression.rs`, `tests/sd13_ranger_favored_terrain_choice.rs`,
//! `tests/sd13_ranger_level4_progression.rs`, `tests/sd13_ranger_level5_progression.rs`) to
//! Ranger level 6, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Sorcerer/Wizard
//! level-range-gate idiom (`supported_ranger_level` is generalized from `1..=5`
//! to `1..=6` via `MAX_SUPPORTED_RANGER_LEVEL = 6`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Ranger class table) were read directly before
//! writing any code or test:
//!
//! - level 6 base attack bonus is +6 (full BAB), base Fortitude/Reflex are +5
//!   (good, `6/2+2 = 5`, a genuinely new value, up from +4 at level 5), base
//!   Will is +2 (poor, `6/3 = 2`, also a genuinely new value, up from +1) -- all
//!   via the same formulas already grounded at levels 1-5, not re-derived.
//! - Track becomes `max(6/2, 1) = 3` at level 6, a genuinely new value, up from
//!   2 at levels 4-5.
//! - the PF1 Core Rulebook Ranger class table's level-6 "Special" column reads
//!   "Combat style feat" only (verified independently against both primary
//!   sources) -- no OTHER new class feature is gained at 6th level (checked
//!   rather than assumed, mirroring the Rogue/Bard/Cleric/Sorcerer/Wizard
//!   precedent of confirming a table's "Special" column rather than assuming it
//!   away).
//! - the Combat Style Feat rule's own bonus-feat cadence (both sources): "The
//!   ranger's expertise manifests in the form of bonus feats at 2nd, 6th, 10th,
//!   14th, and 18th level" -- 6th level is the very next milestone after 2nd,
//!   confirming a prior cycle's check that the next combat-style feat is NOT at
//!   3rd/4th/5th level. This slice grounds the SECOND combat-style bonus feat as
//!   a restricted-list choice recognition, gated on the same style already
//!   chosen at 2nd level, mirroring the first bonus feat's own grounding idiom
//!   exactly: Archery's own 6th-level list is Improved Precise Shot and
//!   Manyshot; Two-Weapon Combat's own 6th-level list is Improved Two-Weapon
//!   Fighting and Two-Weapon Defense (verified independently against both
//!   primary sources). Only the choice/grant recognition is grounded (+0); no
//!   feat's own mechanical effect is computed.
//! - Endurance, Favored Terrain, combat style, Hunter's Bond, and both favored
//!   enemies (all granted at or before level 5) all stay granted at level 6, not
//!   re-derived.
//!
//! It deliberately does not touch the favored-enemy conditional-application
//! engine, either combat-style bonus feat's own mechanics, the Favored Terrain
//! level-8th/13th/18th breadth, Hunter's Bond ally-bonus application, the
//! animal-companion form, the ranger spell burden, or Ranger level 7+ (all stay
//! named-but-unproven, unchanged from level 5), and it preserves the accepted
//! Ranger level-1/level-2/level-3/level-4/level-5 truth, the Fighter negative
//! control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const RANGER_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level6_sd13_deterministic_input.txt");

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

// ----- Base attack bonus at level 6 -----

#[test]
fn ranger_level6_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(RANGER_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 6,
        "Ranger level 6 full-BAB progression (classlevel) must equal 6: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 6 (good Fortitude/Reflex, poor Will; genuinely new values) -----

#[test]
fn ranger_level6_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(RANGER_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 5, "Ranger level 6 good Fortitude (6/2+2) must equal 5");

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 5, "Ranger level 6 good Reflex (6/2+2) must equal 5");

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 2, "Ranger level 6 poor Will (6/3) must equal 2");
}

// ----- Track at level 6 (genuinely new value, up from 2 at levels 4-5) -----

#[test]
fn ranger_level6_track_rises_to_three() {
    let input = load(RANGER_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let track = explanation(&computation, TRACK_ID);
    assert_eq!(
        track.value, 3,
        "Ranger level 6 Track bonus (max(6/2, 1)) must equal 3: {}",
        track.detail
    );
}

// ----- Combat style and its FIRST bonus feat stay granted, unchanged -----

#[test]
fn ranger_level6_keeps_combat_style_and_first_bonus_feat_grounded() {
    let input = load(RANGER_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, COMBAT_STYLE_CHOICE_ID),
        "combat-style recognition must stay granted at level 6"
    );
    let first_feat = explanation(&computation, COMBAT_STYLE_BONUS_FEAT_CHOICE_ID);
    assert_eq!(first_feat.value, 0);
    assert!(first_feat.detail.contains("Point-Blank Shot"));
}

// ----- The SECOND combat-style bonus feat is newly grounded at level 6 -----

#[test]
fn ranger_level6_grounds_second_combat_style_bonus_feat_choice_recognition() {
    let input = load(RANGER_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let second_feat = explanation(&computation, COMBAT_STYLE_BONUS_FEAT_2_CHOICE_ID);
    assert_eq!(
        second_feat.value, 0,
        "the second combat-style bonus feat recognition must carry no fabricated mechanical \
         value: {}",
        second_feat.detail
    );
    assert!(
        second_feat.detail.contains("Manyshot"),
        "the second combat-style bonus feat explanation must name the selected feat: {}",
        second_feat.detail
    );
    assert!(
        second_feat.detail.contains("Archery"),
        "the second combat-style bonus feat explanation must name the style it was drawn from: {}",
        second_feat.detail
    );
}

#[test]
fn ranger_level6_second_combat_style_bonus_feat_absent_below_the_gate() {
    let level_5 = RANGER_LEVEL6_FIXTURE
        .replace("class:ranger:6", "class:ranger:5")
        .lines()
        .filter(|line| {
            !line.starts_with("choice=choice:ranger_combat_style_bonus_feat_2")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let input = load(&level_5);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, COMBAT_STYLE_BONUS_FEAT_2_CHOICE_ID),
        "the second combat-style bonus feat must not be recognized below the level-6 gate"
    );
}

// ----- Favored Enemy, Endurance, Favored Terrain, and Hunter's Bond stay granted -----

#[test]
fn ranger_level6_keeps_favored_enemy_endurance_favored_terrain_and_hunters_bond_grounded() {
    let input = load(RANGER_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let first_skill = explanation(&computation, FAVORED_ENEMY_SKILL_BONUS_ID);
    assert_eq!(
        first_skill.value, 4,
        "the first favored enemy's bonus must stay at the widened +4 (targeted at level 5) \
         unchanged at level 6: {}",
        first_skill.detail
    );

    let second_skill = explanation(&computation, FAVORED_ENEMY_2_SKILL_BONUS_ID);
    assert_eq!(
        second_skill.value, 2,
        "the second favored enemy's bonus must stay at the flat +2 unchanged at level 6: {}",
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
        "Hunter's Bond ally-bonus magnitude must stay unchanged at level 6 (still half the \
         first favored enemy's +4 bonus): {}",
        ally_bonus.detail
    );
}

// ----- Negative control: level 7 was later widened into the supported tranche -----
//
// This test previously asserted that level 7 stayed unrecognized by this slice. A
// later SD13-E5 slice (`tests/sd13_ranger_level7_progression.rs`) widened
// `supported_ranger_level` to include level 7 and grounded Woodland Stride there.
// Renamed to reflect that widened truth, mirroring the
// Rogue/Barbarian/Monk/Bard/Cleric/Druid/Sorcerer/Wizard level-7/level-6
// precedent, rather than deleting sibling coverage outright.

#[test]
fn ranger_level_7_was_later_widened_into_the_supported_tranche() {
    let level_7 = RANGER_LEVEL6_FIXTURE.replace("class:ranger:6", "class:ranger:7");
    let input = load(&level_7);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")),
        "level-7 Ranger was later widened into the supported tranche and must now gain \
         bounded ranger chassis explanations: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, ENDURANCE_ID),
        "level-7 Ranger was later widened and must now gain the Endurance explanation"
    );
    assert!(
        has_explanation(&computation, FAVORED_TERRAIN_ID),
        "level-7 Ranger was later widened and must now gain the Favored Terrain explanation"
    );
    assert!(
        has_explanation(&computation, HUNTERS_BOND_ID),
        "level-7 Ranger was later widened and must now gain the Hunter's Bond explanation"
    );
}

// ----- Negative control: the grounding must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_level6_recognition() {
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
fn multiclass_ranger_level6_is_not_promoted_by_this_slice() {
    let multiclass = RANGER_LEVEL6_FIXTURE.replace(
        "class_level=class:ranger:6",
        "class_level=class:ranger:6\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-6 widening -----

#[test]
fn matrix_ranger_row_names_level_6_widening_and_second_combat_style_bonus_feat() {
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
        ranger.grounding_ref.contains("sd13_ranger_level6_progression"),
        "ranger row must cite the live SD13-E5 level-6 proof surface: {}",
        ranger.grounding_ref
    );

    let note = ranger.blocker_or_lossiness_note;
    assert!(
        note.contains("level 6") || note.contains("level-6"),
        "ranger partial note must name the level-6 widening: {note}"
    );
    assert!(
        note.to_lowercase().contains("second combat-style bonus feat")
            || note.to_lowercase().contains("second combat style bonus feat"),
        "ranger partial note must name the second combat-style bonus feat as newly grounded: \
         {note}"
    );
    // The still-unproven burdens stay named.
    for token in ["spell", "conditional-application", "companion"] {
        assert!(
            note.to_lowercase().contains(token),
            "ranger partial note must still name the unproven '{token}' burden: {note}"
        );
    }
}
