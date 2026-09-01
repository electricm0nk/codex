//! SD13-E5 Ranger level-8 progression grounding proof.
//!
//! Widens the accepted Ranger level-1/level-2/level-3/level-4/level-5/level-6/
//! level-7 per-pillar decomposition
//! (`tests/sd13_ranger_level1_chassis_and_class_feature_separation.rs`,
//! `tests/sd13_ranger_base_attack_and_saves.rs`, `tests/sd13_ranger_level2_progression.rs`,
//! `tests/sd13_ranger_level3_progression.rs`, `tests/sd13_ranger_favored_terrain_choice.rs`,
//! `tests/sd13_ranger_level4_progression.rs`, `tests/sd13_ranger_level5_progression.rs`,
//! `tests/sd13_ranger_level6_progression.rs`, `tests/sd13_ranger_level7_progression.rs`) to
//! Ranger level 8, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Sorcerer/Wizard
//! level-range-gate idiom (`supported_ranger_level` is generalized from `1..=7`
//! to `1..=8` via `MAX_SUPPORTED_RANGER_LEVEL = 8`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Ranger class table) were read directly before
//! writing any code or test:
//!
//! - level 8 base attack bonus is +8 (full BAB), base Fortitude/Reflex genuinely
//!   rise to +6 (good, `8/2+2 = 6`, a new value up from +5 at level 7), and base
//!   Will stays +2 (poor, `8/3 = 2`, numerically unchanged from level 7 -- an
//!   integer-division coincidence, re-verified rather than assumed) -- all via
//!   the same formulas already grounded at levels 1-7, not re-derived.
//! - Track genuinely rises to `max(8/2, 1) = 4` at level 8, up from 3 at level
//!   7 -- via the same pre-existing formula, a real value change, re-verified
//!   rather than assumed.
//! - the Favored Enemy rule's next interval (10th level) has not yet arrived, so
//!   both favored enemies' bonuses stay exactly as they were at level 7 (no new
//!   favored-enemy selection, no new bonus-increase choice) -- re-verified
//!   against both primary sources rather than assumed.
//! - the Combat Style Feat rule's own bonus-feat cadence (2nd, 6th, 10th, 14th,
//!   18th level) confirms 8th level grants no new combat-style bonus feat --
//!   both recognized bonus feats (2nd- and 6th-level) stay granted unchanged.
//! - Endurance, Favored Terrain (still only the first terrain, at its
//!   unchanged +2), combat style (both bonus feats), both favored enemies,
//!   Hunter's Bond, and Woodland Stride all stay granted at level 8, not
//!   re-derived.
//! - the PF1 Core Rulebook Ranger class table's level-8 "Special" column names
//!   TWO entries (verified independently against both primary sources): "Swift
//!   tracker" and "2nd favored terrain". Swift Tracker ("a ranger can move at
//!   his normal speed while using Survival to follow tracks without taking the
//!   normal -5 penalty. He takes only a -10 penalty (instead of the normal
//!   -20) when moving at up to twice normal speed while tracking") only
//!   modifies a tracking-while-moving penalty resolution that does not exist
//!   anywhere in this codebase (this codebase grounds only the flat Track
//!   skill-bonus magnitude, never a check-execution/movement-penalty engine),
//!   so -- exactly like Woodland Stride modifying an undergrowth-movement
//!   resolution this codebase also does not have -- it is a genuinely
//!   flat/identity-shaped, no-choice, no-magnitude grant. This slice grounds
//!   it as a bounded grant-only identity record
//!   (`class_feature.ranger.swift_tracker`, value 0), mirroring the Woodland
//!   Stride idiom exactly. "2nd favored terrain" mirrors the Favored Enemy
//!   5th-level idiom (a second terrain-type selection plus a
//!   bonus-increase-target choice) already grounded in this codebase, but
//!   grounding it is a multi-record burden of its own (a new choice slot, a
//!   restricted bonus-increase-target choice, and a flat magnitude) --
//!   deliberately left named-but-unproven this slice per the operator brief's
//!   one-new-record cap on this bounded widening; it is a real, newly
//!   discovered burden for a future slice, not an invented one.
//!
//! It deliberately does not touch the favored-enemy conditional-application
//! engine, either combat-style bonus feat's own mechanics, the 2nd Favored
//! Terrain selection/bonus-increase-target choice, Hunter's Bond ally-bonus
//! application, the animal-companion form, Swift Tracker's own tracking-speed
//! penalty application, the ranger spell burden, or Ranger level 9+ (all stay
//! named-but-unproven, unchanged from level 7), and it preserves the accepted
//! Ranger level-1 through level-7 truth, the Fighter negative control, and the
//! multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const RANGER_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level8_sd13_deterministic_input.txt");

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
const SWIFT_TRACKER_ID: &str = "class_feature.ranger.swift_tracker";

// ----- Base attack bonus at level 8 -----

#[test]
fn ranger_level8_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(RANGER_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 8,
        "Ranger level 8 full-BAB progression (classlevel) must equal 8: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 8 (Fort/Ref genuinely rise, Will unchanged) -----

#[test]
fn ranger_level8_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(RANGER_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 6, "Ranger level 8 good Fortitude (8/2+2) must equal 6");

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 6, "Ranger level 8 good Reflex (8/2+2) must equal 6");

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(
        will.value, 2,
        "Ranger level 8 poor Will (8/3) must equal 2, unchanged from level 7"
    );
}

// ----- Track genuinely rises to 4 at level 8 -----

#[test]
fn ranger_level8_track_rises_to_four() {
    let input = load(RANGER_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let track = explanation(&computation, TRACK_ID);
    assert_eq!(
        track.value, 4,
        "Ranger level 8 Track bonus (max(8/2, 1)) must equal 4, up from 3 at level 7: {}",
        track.detail
    );
}

// ----- Combat style and BOTH bonus feats stay granted, unchanged -----

#[test]
fn ranger_level8_keeps_combat_style_and_both_bonus_feats_grounded() {
    let input = load(RANGER_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, COMBAT_STYLE_CHOICE_ID),
        "combat-style recognition must stay granted at level 8"
    );
    let first_feat = explanation(&computation, COMBAT_STYLE_BONUS_FEAT_CHOICE_ID);
    assert_eq!(first_feat.value, 0);
    assert!(first_feat.detail.contains("Point-Blank Shot"));

    let second_feat = explanation(&computation, COMBAT_STYLE_BONUS_FEAT_2_CHOICE_ID);
    assert_eq!(second_feat.value, 0);
    assert!(second_feat.detail.contains("Manyshot"));
}

// ----- No THIRD combat-style bonus feat is fabricated at level 8 -----

#[test]
fn ranger_level8_does_not_fabricate_a_third_combat_style_bonus_feat() {
    let input = load(RANGER_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(
            &computation,
            "class_chassis.ranger.combat_style_bonus_feat_3_choice"
        ),
        "the PF1 CRB grants no THIRD combat-style bonus feat at level 8 (next is level 10); \
         no such record must be fabricated: {:?}",
        computation.explanations
    );
}

// ----- Favored Enemy, Endurance, Favored Terrain, and Hunter's Bond stay granted -----

#[test]
fn ranger_level8_keeps_favored_enemy_endurance_favored_terrain_and_hunters_bond_grounded() {
    let input = load(RANGER_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let first_skill = explanation(&computation, FAVORED_ENEMY_SKILL_BONUS_ID);
    assert_eq!(
        first_skill.value, 4,
        "the first favored enemy's bonus must stay at the widened +4 (targeted at level 5) \
         unchanged at level 8: {}",
        first_skill.detail
    );

    let second_skill = explanation(&computation, FAVORED_ENEMY_2_SKILL_BONUS_ID);
    assert_eq!(
        second_skill.value, 2,
        "the second favored enemy's bonus must stay at the flat +2 unchanged at level 8: {}",
        second_skill.detail
    );

    let endurance = explanation(&computation, ENDURANCE_ID);
    assert_eq!(endurance.value, 0);
    assert!(endurance.detail.to_lowercase().contains("granted"));

    let favored_terrain = explanation(&computation, FAVORED_TERRAIN_ID);
    assert_eq!(
        favored_terrain.value, 2,
        "the first favored terrain's bonus must stay at the flat +2, unchanged at level 8 -- \
         the 2nd favored terrain selection is deliberately left ungrounded this slice: {}",
        favored_terrain.detail
    );
    assert!(favored_terrain.detail.to_lowercase().contains("granted"));

    let hunters_bond = explanation(&computation, HUNTERS_BOND_ID);
    assert_eq!(hunters_bond.value, 0);
    assert!(hunters_bond.detail.to_lowercase().contains("granted"));

    let ally_bonus = explanation(&computation, HUNTERS_BOND_ALLY_BONUS_ID);
    assert_eq!(
        ally_bonus.value, 2,
        "Hunter's Bond ally-bonus magnitude must stay unchanged at level 8 (still half the \
         first favored enemy's +4 bonus): {}",
        ally_bonus.detail
    );

    let woodland_stride = explanation(&computation, WOODLAND_STRIDE_ID);
    assert_eq!(woodland_stride.value, 0);
    assert!(woodland_stride.detail.to_lowercase().contains("granted"));
}

// ----- Swift Tracker is newly grounded at level 8 as a grant-only identity record -----

#[test]
fn ranger_level8_grounds_swift_tracker_as_a_grant_only_identity_record() {
    let input = load(RANGER_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let swift_tracker = explanation(&computation, SWIFT_TRACKER_ID);
    assert_eq!(
        swift_tracker.value, 0,
        "Swift Tracker must carry no fabricated mechanical value: {}",
        swift_tracker.detail
    );
    assert!(
        swift_tracker.detail.to_lowercase().contains("granted"),
        "Swift Tracker must be recorded as granted at level 8: {}",
        swift_tracker.detail
    );
    assert!(
        swift_tracker.detail.to_lowercase().contains("track"),
        "Swift Tracker detail must name the tracking rule it modifies: {}",
        swift_tracker.detail
    );
}

#[test]
fn ranger_level8_swift_tracker_absent_below_the_gate() {
    let level_7 = RANGER_LEVEL8_FIXTURE.replace("class:ranger:8", "class:ranger:7");
    let input = load(&level_7);
    let computation = compute_pilot_base_chassis(&input);

    let swift_tracker = explanation(&computation, SWIFT_TRACKER_ID);
    assert_eq!(swift_tracker.value, 0);
    assert!(
        swift_tracker.detail.to_lowercase().contains("absent"),
        "Swift Tracker must be a correct level-gate absence below level 8: {}",
        swift_tracker.detail
    );
}

// ----- The 2nd Favored Terrain choice/bonus-increase burden stays unfabricated -----

#[test]
fn ranger_level8_does_not_fabricate_a_second_favored_terrain() {
    let input = load(RANGER_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(
            &computation,
            "class_chassis.ranger.favored_terrain_2_choice"
        ),
        "the PF1 CRB level-8 second favored-terrain selection is a real, newly discovered \
         burden but is deliberately left ungrounded this bounded slice; no such record must \
         be fabricated: {:?}",
        computation.explanations
    );
}

// ----- Level 9 was later widened into the supported tranche by a further slice -----
//
// This block previously superseded `tests/sd13_ranger_level7_progression.rs`'s
// level-8 negative control; its own coverage moves to
// `tests/sd13_ranger_level9_progression.rs`'s level-10 control now that level 9
// is itself promoted into the supported tranche.

#[test]
fn ranger_level_9_was_later_widened_into_the_supported_tranche() {
    let level_9 = RANGER_LEVEL8_FIXTURE.replace("class:ranger:8", "class:ranger:9");
    let input = load(&level_9);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")),
        "level-9 Ranger is now recognized by the later level-9 widening slice \
         (tests/sd13_ranger_level9_progression.rs carries its proof): {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, ENDURANCE_ID),
        "level-9 Ranger now carries the Endurance record via the widened gate"
    );
}

// ----- Negative control: the grounding must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_level8_recognition() {
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
                || e.id == WOODLAND_STRIDE_ID
                || e.id == SWIFT_TRACKER_ID),
        "the Fighter chassis must not surface any ranger-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Ranger is not promoted -----

#[test]
fn multiclass_ranger_level8_is_not_promoted_by_this_slice() {
    let multiclass = RANGER_LEVEL8_FIXTURE.replace(
        "class_level=class:ranger:8",
        "class_level=class:ranger:8\nclass_level=class:fighter:1",
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
                || e.id == WOODLAND_STRIDE_ID
                || e.id == SWIFT_TRACKER_ID),
        "multiclass Ranger must not gain any bounded ranger chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Ranger must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-8 widening -----

#[test]
fn matrix_ranger_row_names_level_8_widening_and_swift_tracker() {
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
        ranger.grounding_ref.contains("sd13_ranger_level8_progression"),
        "ranger row must cite the live SD13-E5 level-8 proof surface: {}",
        ranger.grounding_ref
    );

    let note = ranger.blocker_or_lossiness_note;
    assert!(
        note.contains("level 8") || note.contains("level-8"),
        "ranger partial note must name the level-8 widening: {note}"
    );
    assert!(
        note.to_lowercase().contains("swift tracker"),
        "ranger partial note must name Swift Tracker as newly grounded: {note}"
    );
    assert!(
        note.to_lowercase().contains("favored terrain"),
        "ranger partial note must still name the 2nd favored terrain as an unproven burden: {note}"
    );
    // The still-unproven burdens stay named.
    for token in ["spell", "conditional-application", "companion"] {
        assert!(
            note.to_lowercase().contains(token),
            "ranger partial note must still name the unproven '{token}' burden: {note}"
        );
    }
}
