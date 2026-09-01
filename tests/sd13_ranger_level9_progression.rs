//! SD13-E5 Ranger level-9 progression grounding proof.
//!
//! Widens the accepted Ranger level-1..level-8 hybrid chassis baseline (most
//! recently `tests/sd13_ranger_level8_progression.rs`) to Ranger level 9,
//! mirroring the sibling-class level-range-gate idiom
//! (`supported_ranger_level` is generalized from `1..=8` to `1..=9` via
//! `MAX_SUPPORTED_RANGER_LEVEL = 9`). Both PF1 CRB primary sources (d20pfsrd
//! and legacy.aonprd.com Ranger class table) were read directly before
//! writing any code or test:
//!
//! - level 9 base attack bonus is +9 (full BAB, genuinely risen from +8 —
//!   the table's own "+9/+4" iterative notation is not modeled anywhere in
//!   this codebase, only the flat base value) and base saves are +6
//!   Fortitude and +6 Reflex (both good, `9 / 2 + 2 = 6`, numerically
//!   unchanged from level 8, integer-division coincidences) and +3 Will
//!   (poor, `9 / 3 = 3`, genuinely risen from +2) — confirmed by the same
//!   formulas already grounded at levels 1-8, not re-derived.
//! - the PF1 Core Rulebook Ranger class table's level-9 "Special" column
//!   reads "Evasion" (verified independently against both primary sources,
//!   checked rather than assumed away) — a genuinely NEW class feature at
//!   9th level ("If he makes a successful Reflex saving throw against an
//!   attack that normally deals half damage on a successful save, he
//!   instead takes no damage"). This slice grounds it as a +0
//!   identity/recognition record ONLY (`class_feature.ranger.evasion`),
//!   exactly mirroring how Rogue's and Monk's own Evasion records were
//!   grounded: no Reflex-save damage-resolution engine exists in this
//!   codebase, so no damage math is fabricated from the record.
//! - Track stays 4 (`max(9 / 2, 1) = 4`, numerically unchanged from level
//!   8, an integer-division coincidence); the favored-enemy skill bonuses
//!   stay +4/+2 and the favored-terrain count stays at its second selection
//!   (the next favored-enemy grant lands at 10th and the next
//!   favored-terrain grant at 13th, both checked rather than assumed);
//!   Hunter's Bond and its +2 ally bonus, Endurance, Woodland Stride, Swift
//!   Tracker, the combat-style choice, and both combat-style bonus-feat
//!   choice recognitions all carry over unchanged, not re-derived.
//!
//! It deliberately does not touch the hybrid spell burden, the favored-enemy
//! attack/damage application, or any check-execution engine (all stay
//! named-but-unproven, unchanged from levels 1-8), and it does not ground
//! Ranger level 10+. It also preserves the accepted Ranger level-1..level-8
//! truth (unchanged), the Fighter negative control, and the multiclass
//! negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const RANGER_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level8_sd13_deterministic_input.txt");

const RANGER_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level9_sd13_deterministic_input.txt");

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
const HUNTERS_BOND_ALLY_BONUS_ID: &str = "class_chassis.ranger.hunters_bond_ally_bonus";
const ENDURANCE_ID: &str = "class_feature.ranger.endurance";
const FAVORED_TERRAIN_ID: &str = "class_feature.ranger.favored_terrain";
const WOODLAND_STRIDE_ID: &str = "class_feature.ranger.woodland_stride";
const SWIFT_TRACKER_ID: &str = "class_feature.ranger.swift_tracker";
const RANGER_EVASION_ID: &str = "class_feature.ranger.evasion";

// ----- Base attack bonus and saves at level 9 -----

#[test]
fn ranger_level9_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(RANGER_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 9,
        "Ranger level 9 full-BAB progression must equal 9, genuinely risen from 8: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 6, "Ranger level 9 good Fortitude (9/2+2) must equal 6");

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 6, "Ranger level 9 good Reflex (9/2+2) must equal 6");

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(
        will.value, 3,
        "Ranger level 9 poor Will (9/3) must equal 3, genuinely risen from 2 at level 8"
    );
}

// ----- Evasion is newly grounded as a +0 identity record at level 9 -----

#[test]
fn ranger_level9_grounds_evasion_as_identity_recognition_only() {
    let input = load(RANGER_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let evasion = explanation(&computation, RANGER_EVASION_ID);
    assert_eq!(
        evasion.value, 0,
        "Ranger Evasion must be grounded as a +0 identity/recognition record only — no \
         Reflex-save damage-resolution engine exists in this codebase: {}",
        evasion.detail
    );
    assert!(
        evasion.detail.contains("no damage"),
        "Ranger Evasion's record must carry the rule's own no-damage-on-successful-save \
         identity: {}",
        evasion.detail
    );
}

// ----- Track stays 4 at level 9 (integer-division coincidence) -----

#[test]
fn ranger_level9_track_stays_four() {
    let input = load(RANGER_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let track = explanation(&computation, TRACK_ID);
    assert_eq!(
        track.value, 4,
        "Ranger level 9 Track (max(9/2, 1)) must stay 4 — an integer-division coincidence \
         with level 8: {}",
        track.detail
    );
}

// ----- Favored-enemy / favored-terrain / hunter's-bond facets carry over -----

#[test]
fn ranger_level9_favored_facets_carry_over_unchanged() {
    let input = load(RANGER_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let first_skill = explanation(&computation, FAVORED_ENEMY_SKILL_BONUS_ID);
    assert_eq!(
        first_skill.value, 4,
        "the first favored enemy's skill bonus must stay +4 at level 9 (the next \
         favored-enemy grant lands at 10th)"
    );

    let second_skill = explanation(&computation, FAVORED_ENEMY_2_SKILL_BONUS_ID);
    assert_eq!(
        second_skill.value, 2,
        "the second favored enemy's skill bonus must stay +2 at level 9"
    );

    let favored_terrain = explanation(&computation, FAVORED_TERRAIN_ID);
    assert_eq!(
        favored_terrain.value, 2,
        "the favored-terrain count must stay 2 at level 9 (the next grant lands at 13th): {}",
        favored_terrain.detail
    );

    let ally_bonus = explanation(&computation, HUNTERS_BOND_ALLY_BONUS_ID);
    assert_eq!(
        ally_bonus.value, 2,
        "Hunter's Bond's ally bonus (half the favored-enemy bonus) must stay +2 at level 9"
    );
}

// ----- Granted features and choice recognitions carry over at level 9 -----

#[test]
fn ranger_level9_granted_features_and_choices_carry_over() {
    let input = load(RANGER_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [ENDURANCE_ID, WOODLAND_STRIDE_ID, SWIFT_TRACKER_ID] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, 0,
            "'{id}' must stay a granted +0 identity record at level 9"
        );
    }

    let style = explanation(&computation, COMBAT_STYLE_CHOICE_ID);
    assert_eq!(
        style.value, 0,
        "the combat-style choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        style.detail.to_lowercase().contains("archery"),
        "the combat-style recognition must still name the Archery selection at level 9: {}",
        style.detail
    );
}

// ----- Negative control: the level-8 fixture is unaffected by this widening -----

#[test]
fn ranger_level8_truth_is_unchanged_by_this_slice() {
    let input = load(RANGER_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(base_attack.value, 8, "Ranger level 8 base attack bonus must stay 8");

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 2, "Ranger level 8 poor Will must stay 2");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == RANGER_EVASION_ID),
        "level-8 Ranger must NOT gain the Evasion record — it is a 9th-level feature: {:?}",
        computation.explanations
    );
}

// ----- Level 10 was later widened into the supported tranche by a further slice -----

#[test]
fn ranger_level_10_was_later_widened_into_the_supported_tranche() {
    let level_10 = RANGER_LEVEL9_FIXTURE.replace("class:ranger:9", "class:ranger:10");
    let input = load(&level_10);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")),
        "level-10 Ranger is now recognized by the later level-10 widening slice \
         (tests/sd13_ranger_level10_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the ranger path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_level9_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")
                || e.id.starts_with("class_feature.ranger.")),
        "the Fighter chassis must not surface any ranger-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Ranger is not promoted -----

#[test]
fn multiclass_ranger_level9_is_not_promoted_by_this_slice() {
    let multiclass = RANGER_LEVEL9_FIXTURE.replace(
        "class_level=class:ranger:9",
        "class_level=class:ranger:9\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")
                || e.id.starts_with("class_feature.ranger.")),
        "multiclass Ranger must not gain any bounded ranger explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Ranger must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-9 widening -----

#[test]
fn matrix_ranger_row_names_level_9_widening() {
    let matrix = seeded_current_truth();
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger hybrid_chassis_and_spell_burden row must exist");

    assert_eq!(ranger.support_state, SupportState::Supported);
    assert_eq!(ranger.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        ranger.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        ranger.grounding_ref.contains("sd13_ranger_level9_progression"),
        "ranger row must cite the live SD13-E5 level-9 proof surface: {}",
        ranger.grounding_ref
    );
    let note = ranger.blocker_or_lossiness_note;
    assert!(
        note.contains("level 9") || note.contains("level-9"),
        "ranger partial note must name the level-9 widening: {note}"
    );
}
