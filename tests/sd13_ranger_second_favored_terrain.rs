//! SD13-E5 Ranger SECOND-favored-terrain grounding proof — the first
//! priority-2 burden slice after the level-10 band closed.
//!
//! Grounds the PF1 Core Rulebook Ranger level-8 "2nd favored terrain" burden
//! that `tests/sd13_ranger_level8_progression.rs` discovered and deliberately
//! deferred ("a real, newly discovered burden for a future slice"). Both PF1
//! CRB primary sources (d20pfsrd and legacy.aonprd.com Ranger page) were read
//! directly before writing any code, and both state the exact rule text: "At
//! 8th level and every five levels thereafter, the ranger may select an
//! additional favored terrain. In addition, at each such interval, the skill
//! bonus and initiative bonus in any one favored terrain (including the one
//! just selected, if so desired), increases by +2." — the exact structural
//! mirror of the Favored Enemy 5th-level interval already grounded in this
//! codebase (`choice:ranger_favored_enemy_2` +
//! `choice:ranger_favored_enemy_bonus_increase_target`), which this slice
//! mirrors record-for-record:
//!
//! - a SECOND terrain-type selection (`choice:ranger_favored_terrain_2`,
//!   open-ended raw-string recognition, no restricted-list validation) is
//!   recognized as a bounded `+0` identity record at ranger level 8+, and its
//!   own flat magnitude is grounded as a standalone
//!   `class_feature.ranger.favored_terrain_2` record (base +2, or +4 when
//!   this interval's bonus-increase target names the second terrain);
//! - the bonus-increase TARGET selection
//!   (`choice:ranger_favored_terrain_bonus_increase_target`, restricted to
//!   the pair `terrain:first` / `terrain:second`) is recognized as a bounded
//!   `+0` record at ranger level 8+, and the targeted terrain's flat
//!   magnitude genuinely rises to +4 while the untargeted one stays +2;
//! - absent either selection in chosen input, nothing is fabricated: the
//!   level-8 baseline fixture (which carries no second-terrain choices)
//!   computes exactly as before — first terrain +2, no second-terrain
//!   records — preserving every existing level-8/9/10 sibling truth;
//! - below the 8th-level interval gate the selections are correctly not
//!   recognized even when present in chosen input.
//!
//! It deliberately does not touch the terrain-detection /
//! conditional-application engine (no Initiative total or skill-check total
//! is modified by any of these records — no such engine exists in this
//! codebase), the favored-enemy records, Swift Tracker, the ranger spell
//! burden, or Ranger level 11+; and it preserves the Fighter negative
//! control and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const RANGER_LEVEL8_BASELINE_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level8_sd13_deterministic_input.txt");

const RANGER_SECOND_TERRAIN_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level8_sd13_second_favored_terrain_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const FAVORED_TERRAIN_ID: &str = "class_feature.ranger.favored_terrain";
const FAVORED_TERRAIN_2_ID: &str = "class_feature.ranger.favored_terrain_2";
const FAVORED_TERRAIN_2_CHOICE_ID: &str = "class_chassis.ranger.favored_terrain_2_choice";
const FAVORED_TERRAIN_BONUS_INCREASE_CHOICE_ID: &str =
    "class_chassis.ranger.favored_terrain_bonus_increase_choice";

// ----- The second terrain selection is recognized at level 8 -----

#[test]
fn ranger_level8_second_favored_terrain_selection_is_recognized() {
    let input = load(RANGER_SECOND_TERRAIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, FAVORED_TERRAIN_2_CHOICE_ID);
    assert_eq!(
        choice.value, 0,
        "the second favored-terrain selection must be a +0 recognition record only"
    );
    assert!(
        choice.detail.contains("mountain"),
        "the recognition record must name the raw chosen terrain string (terrain:mountain): {}",
        choice.detail
    );
}

// ----- The second terrain's own flat magnitude is grounded -----

#[test]
fn ranger_level8_second_favored_terrain_magnitude_is_grounded_at_base_two() {
    let input = load(RANGER_SECOND_TERRAIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let second = explanation(&computation, FAVORED_TERRAIN_2_ID);
    assert_eq!(
        second.value, 2,
        "the second favored terrain must carry its own flat +2 magnitude (this fixture's \
         bonus-increase target names the FIRST terrain, so the second stays at base): {}",
        second.detail
    );
}

// ----- The bonus-increase target is recognized and boosts the first terrain -----

#[test]
fn ranger_level8_bonus_increase_target_boosts_the_first_terrain_to_four() {
    let input = load(RANGER_SECOND_TERRAIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let target = explanation(&computation, FAVORED_TERRAIN_BONUS_INCREASE_CHOICE_ID);
    assert_eq!(
        target.value, 0,
        "the bonus-increase target selection must be a +0 recognition record only"
    );
    assert!(
        target.detail.contains("first favored terrain"),
        "the recognition record must name the first favored terrain as the boosted one: {}",
        target.detail
    );

    let first = explanation(&computation, FAVORED_TERRAIN_ID);
    assert_eq!(
        first.value, 4,
        "the first favored terrain's flat magnitude must genuinely rise to +4 when the \
         interval's bonus-increase target names it: {}",
        first.detail
    );
}

// ----- Targeting the second terrain boosts it instead -----

#[test]
fn ranger_level8_bonus_increase_target_second_boosts_the_second_terrain() {
    let swapped = RANGER_SECOND_TERRAIN_FIXTURE.replace(
        "choice=choice:ranger_favored_terrain_bonus_increase_target:terrain:first",
        "choice=choice:ranger_favored_terrain_bonus_increase_target:terrain:second",
    );
    let input = load(&swapped);
    let computation = compute_pilot_base_chassis(&input);

    let first = explanation(&computation, FAVORED_TERRAIN_ID);
    assert_eq!(
        first.value, 2,
        "the first favored terrain must stay at the flat +2 when the bonus-increase target \
         names the second terrain"
    );

    let second = explanation(&computation, FAVORED_TERRAIN_2_ID);
    assert_eq!(
        second.value, 4,
        "the second favored terrain's flat magnitude must genuinely rise to +4 when the \
         interval's bonus-increase target names it (\"including the one just selected, if so \
         desired\"): {}",
        second.detail
    );
}

// ----- An unrecognized target string fabricates no boost -----

#[test]
fn ranger_level8_unrecognized_bonus_increase_target_boosts_nothing() {
    let unrecognized = RANGER_SECOND_TERRAIN_FIXTURE.replace(
        "choice=choice:ranger_favored_terrain_bonus_increase_target:terrain:first",
        "choice=choice:ranger_favored_terrain_bonus_increase_target:terrain:third",
    );
    let input = load(&unrecognized);
    let computation = compute_pilot_base_chassis(&input);

    let target = explanation(&computation, FAVORED_TERRAIN_BONUS_INCREASE_CHOICE_ID);
    assert_eq!(target.value, 0);
    assert!(
        target.detail.contains("restricted pair"),
        "an unrecognized target must be surfaced as outside the restricted pair, with no \
         target identity grounded: {}",
        target.detail
    );

    let first = explanation(&computation, FAVORED_TERRAIN_ID);
    assert_eq!(first.value, 2, "no boost may be fabricated from an unrecognized target");
    let second = explanation(&computation, FAVORED_TERRAIN_2_ID);
    assert_eq!(second.value, 2, "no boost may be fabricated from an unrecognized target");
}

// ----- Negative control: the baseline level-8 fixture is unchanged -----

#[test]
fn ranger_level8_baseline_without_the_selections_is_unchanged_by_this_slice() {
    let input = load(RANGER_LEVEL8_BASELINE_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, FAVORED_TERRAIN_2_CHOICE_ID),
        "absent a chosen second terrain, no second-terrain recognition may be fabricated"
    );
    assert!(
        !has_explanation(&computation, FAVORED_TERRAIN_2_ID),
        "absent a chosen second terrain, no second-terrain magnitude may be fabricated"
    );
    assert!(
        !has_explanation(&computation, FAVORED_TERRAIN_BONUS_INCREASE_CHOICE_ID),
        "absent a chosen bonus-increase target, no target recognition may be fabricated"
    );

    let first = explanation(&computation, FAVORED_TERRAIN_ID);
    assert_eq!(
        first.value, 2,
        "the baseline first favored terrain must stay at the flat +2, exactly as the \
         level-8 chassis slice left it"
    );
}

// ----- Negative control: below the 8th-level interval gate -----

#[test]
fn ranger_level7_does_not_recognize_the_second_terrain_selections() {
    let level_7 = RANGER_SECOND_TERRAIN_FIXTURE.replace("class:ranger:8", "class:ranger:7");
    let input = load(&level_7);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, FAVORED_TERRAIN_2_CHOICE_ID)
            && !has_explanation(&computation, FAVORED_TERRAIN_2_ID)
            && !has_explanation(&computation, FAVORED_TERRAIN_BONUS_INCREASE_CHOICE_ID),
        "the 8th-level interval selections must not be recognized below the PF1 CRB gate \
         even when present in chosen input: {:?}",
        computation.explanations
    );

    let first = explanation(&computation, FAVORED_TERRAIN_ID);
    assert_eq!(
        first.value, 2,
        "below the interval gate the first favored terrain must stay at the flat +2"
    );
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_second_favored_terrain_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !has_explanation(&computation, FAVORED_TERRAIN_2_CHOICE_ID)
            && !has_explanation(&computation, FAVORED_TERRAIN_2_ID)
            && !has_explanation(&computation, FAVORED_TERRAIN_BONUS_INCREASE_CHOICE_ID),
        "the Fighter chassis must not surface any second-favored-terrain record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Ranger is not promoted -----

#[test]
fn multiclass_ranger_does_not_gain_second_favored_terrain_recognition() {
    let multiclass = RANGER_SECOND_TERRAIN_FIXTURE.replace(
        "class_level=class:ranger:8",
        "class_level=class:ranger:8\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, FAVORED_TERRAIN_2_CHOICE_ID)
            && !has_explanation(&computation, FAVORED_TERRAIN_2_ID)
            && !has_explanation(&computation, FAVORED_TERRAIN_BONUS_INCREASE_CHOICE_ID),
        "multiclass Ranger must not gain any second-favored-terrain record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Ranger must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the second-favored-terrain grounding -----

#[test]
fn matrix_ranger_row_names_the_second_favored_terrain_grounding() {
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
        ranger
            .grounding_ref
            .contains("sd13_ranger_second_favored_terrain"),
        "ranger row must cite the live second-favored-terrain proof surface: {}",
        ranger.grounding_ref
    );
    assert!(
        ranger
            .blocker_or_lossiness_note
            .contains("favored_terrain_2"),
        "ranger partial note must name the grounded second-favored-terrain records: {}",
        ranger.blocker_or_lossiness_note
    );
}
