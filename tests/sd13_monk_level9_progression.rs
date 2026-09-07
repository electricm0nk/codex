//! SD13-E5 Monk level-9 progression grounding proof.
//!
//! Widens the accepted Monk level-1..level-8 martial chassis baseline (most
//! recently `tests/sd13_monk_level8_progression.rs`) to Monk level 9,
//! mirroring the sibling-class level-range-gate idiom
//! (`supported_monk_level` is generalized from `1..=8` to `1..=9` via
//! `MAX_SUPPORTED_MONK_LEVEL = 9`). Both PF1 CRB primary sources (d20pfsrd
//! and legacy.aonprd.com Monk class table) were read directly before writing
//! any code or test:
//!
//! - level 9 base attack bonus is +6 (`9 * 3 / 4 = 6`, the Monk's 3/4-BAB
//!   progression, numerically unchanged from level 8 — an integer-division
//!   coincidence; the class table's own "+6/+1" iterative notation is not
//!   modeled anywhere in this codebase, only the flat base value) and all
//!   three base saves are +6 (all good, `9 / 2 + 2 = 6`, likewise unchanged
//!   from level 8, integer-division coincidences) — confirmed by the same
//!   formulas already grounded at levels 1-8, not re-derived.
//! - the unarmed strike damage die STAYS 1d10 (the 1d10 band spans monk
//!   levels 8-11 per both primary sources' unarmed-damage column, checked
//!   rather than assumed).
//! - the Flurry of Blows flat attack modifier GENUINELY RISES to +7 (monk
//!   level - 2 = 9 - 2, via the same pre-existing formula) while the attack
//!   count STAYS 3 (the rule text grants the third attack at 8th level "as
//!   if using Improved Two-Weapon Fighting"; the next count change does not
//!   land until the Greater Two-Weapon Fighting upgrade at 15th).
//! - the ki pool STAYS 7 (`9 / 2 + Wisdom modifier 3 = 4 + 3`, numerically
//!   unchanged from level 8, an integer-division coincidence), and Slow
//!   Fall's reach STAYS 40 ft (the next reach increase lands at 10th level,
//!   checked rather than assumed).
//! - Still Mind stays the flat +2; Purity of Body and Evasion stay granted
//!   +0 identity records, not re-derived; the level-1 bonus-feat choice
//!   recognition (Deflect Arrows, per the SD13-E5 list correction) still fires and its execution burden still
//!   claim-blocks.
//! - the PF1 Core Rulebook Monk class table's level-9 "Special" column
//!   reads "Improved evasion" (verified independently against both primary
//!   sources, checked rather than assumed away) — a genuinely NEW named
//!   entry at 9th level, and an upgrade of the already-recognized 2nd-level
//!   Evasion identity ("He still takes no damage on a successful Reflex
//!   saving throw against attacks, but henceforth he takes only half damage
//!   on a failed save"). This slice grounds it as a +0 identity/recognition
//!   record ONLY (`class_feature.monk.improved_evasion`), exactly mirroring
//!   how Evasion itself (2nd) and the Rogue's Improved Uncanny Dodge (8th)
//!   were grounded: no Reflex-save damage-resolution engine exists in this
//!   codebase, so no damage math is fabricated from the record.
//!
//! It deliberately does not touch the ki-power execution burden, the
//! level-2/level-6 repeat bonus-feat grants, High Jump, Wholeness of Body,
//! or any fall-damage/attack-resolution engine (all stay named-but-unproven,
//! unchanged from levels 1-8), and it does not ground Monk level 10+. It
//! also preserves the accepted Monk level-1..level-8 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const MONK_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level8_sd13_deterministic_input.txt");

const MONK_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level9_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const MONK_EVASION_ID: &str = "class_feature.monk.evasion";
const MONK_IMPROVED_EVASION_ID: &str = "class_feature.monk.improved_evasion";
const MONK_STILL_MIND_ID: &str = "class_feature.monk.still_mind";
const MONK_KI_POOL_ID: &str = "class_chassis.monk.ki_pool_size";
const MONK_PURITY_OF_BODY_ID: &str = "class_chassis.monk.purity_of_body";
const MONK_SLOW_FALL_ID: &str = "class_chassis.monk.slow_fall";

// ----- Base attack bonus and saves at level 9 -----

#[test]
fn monk_level9_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(MONK_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.monk.base_attack_bonus");
    assert_eq!(
        base_attack.value, 6,
        "Monk level 9 3/4-BAB progression (9 * 3 / 4) must equal 6 — numerically unchanged \
         from level 8, an integer-division coincidence: {}",
        base_attack.detail
    );

    for (id, label) in [
        ("class_chassis.monk.base_save.fortitude", "Fortitude"),
        ("class_chassis.monk.base_save.reflex", "Reflex"),
        ("class_chassis.monk.base_save.will", "Will"),
    ] {
        let save = explanation(&computation, id);
        assert_eq!(
            save.value, 6,
            "Monk level 9 good {label} (9/2+2) must equal 6 — unchanged from level 8, an \
             integer-division coincidence"
        );
    }
}

// ----- Unarmed strike damage die stays 1d10 at level 9 -----

#[test]
fn monk_level9_unarmed_strike_die_stays_d10() {
    let input = load(MONK_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let unarmed = explanation(&computation, "class_chassis.monk.unarmed_strike_damage_die");
    assert_eq!(
        unarmed.value, 10,
        "Monk level 9 unarmed strike die must stay 1d10 (the 1d10 band spans levels 8-11 per \
         both primary sources): {}",
        unarmed.detail
    );
}

// ----- Flurry: attack bonus rises to +7, attack count stays 3 -----

#[test]
fn monk_level9_flurry_bonus_rises_and_count_stays_three() {
    let input = load(MONK_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let attack_bonus = explanation(
        &computation,
        "class_chassis.monk.flurry_of_blows_attack_bonus",
    );
    assert_eq!(
        attack_bonus.value, 7,
        "Monk level 9 Flurry flat attack modifier (monk level - 2) must equal 7, genuinely \
         risen from 6 at level 8: {}",
        attack_bonus.detail
    );

    let attack_count = explanation(
        &computation,
        "class_chassis.monk.flurry_of_blows_attack_count",
    );
    assert_eq!(
        attack_count.value, 3,
        "Monk level 9 Flurry attack count must stay 3 (the next count change lands at 15th \
         via the Greater Two-Weapon Fighting upgrade): {}",
        attack_count.detail
    );
}

// ----- Ki pool and Slow Fall stay at their level-8 values -----

#[test]
fn monk_level9_ki_pool_and_slow_fall_stay_at_level8_values() {
    let input = load(MONK_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // CG-03 fix: Wisdom modifier is now +4 (base 17 + 2 Human racial), not +3.
    let ki_pool = explanation(&computation, MONK_KI_POOL_ID);
    assert_eq!(
        ki_pool.value, 8,
        "Monk level 9 ki pool (9/2 + Wisdom modifier 4) must stay 8 — an integer-division \
         coincidence with level 8: {}",
        ki_pool.detail
    );

    let slow_fall = explanation(&computation, MONK_SLOW_FALL_ID);
    assert_eq!(
        slow_fall.value, 0,
        "Slow Fall recognition must carry no fabricated mechanical value at level 9"
    );
    assert!(
        slow_fall.detail.contains("40"),
        "Slow Fall's reach must stay 40 ft at level 9 (the next reach increase lands at \
         10th): {}",
        slow_fall.detail
    );
}

// ----- Improved Evasion is newly grounded as a +0 identity record at level 9 -----

#[test]
fn monk_level9_grounds_improved_evasion_as_identity_recognition_only() {
    let input = load(MONK_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let improved_evasion = explanation(&computation, MONK_IMPROVED_EVASION_ID);
    assert_eq!(
        improved_evasion.value, 0,
        "Improved Evasion must be grounded as a +0 identity/recognition record only — no \
         Reflex-save damage-resolution engine exists in this codebase: {}",
        improved_evasion.detail
    );
    assert!(
        improved_evasion.detail.contains("half damage"),
        "Improved Evasion's record must carry the rule's own half-damage-on-failed-save \
         identity: {}",
        improved_evasion.detail
    );

    let evasion = explanation(&computation, MONK_EVASION_ID);
    assert_eq!(
        evasion.value, 0,
        "the base Evasion recognition must still fire alongside Improved Evasion at level 9"
    );
}

// ----- Still Mind / Purity of Body stay unchanged at level 9 -----

#[test]
fn monk_level9_still_mind_and_purity_stay_unchanged() {
    let input = load(MONK_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let still_mind = explanation(&computation, MONK_STILL_MIND_ID);
    assert_eq!(still_mind.value, 2, "Still Mind must stay the flat +2 at level 9");

    let purity = explanation(&computation, MONK_PURITY_OF_BODY_ID);
    assert_eq!(
        purity.value, 0,
        "Purity of Body recognition must carry no fabricated mechanical value at level 9"
    );
}

// ----- The bonus-feat choice recognition and burden still fire at level 9 -----

#[test]
fn monk_level9_still_recognizes_the_bonus_feat_choice_and_burden() {
    let input = load(MONK_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.monk.bonus_feat_choice");
    assert_eq!(
        choice.value, 0,
        "bonus-feat choice recognition must carry no fabricated mechanical value"
    );

    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.monk.bounded_progression.bonus_feat.unsupported"
            && d.claim_blocking),
        "level-9 Monk must still claim-block on the bonus-feat execution burden: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the level-8 fixture is unaffected by this widening -----

#[test]
fn monk_level8_truth_is_unchanged_by_this_slice() {
    let input = load(MONK_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let attack_bonus = explanation(
        &computation,
        "class_chassis.monk.flurry_of_blows_attack_bonus",
    );
    assert_eq!(attack_bonus.value, 6, "Monk level 8 Flurry attack modifier must stay 6");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == MONK_IMPROVED_EVASION_ID),
        "level-8 Monk must NOT gain the Improved Evasion record — it is a 9th-level feature: \
         {:?}",
        computation.explanations
    );
}

// ----- Level 10 was later widened into the supported tranche by a further slice -----

#[test]
fn monk_level_10_was_later_widened_into_the_supported_tranche() {
    let level_10 = MONK_LEVEL9_FIXTURE.replace("class:monk:9", "class:monk:10");
    let input = load(&level_10);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.monk.")),
        "level-10 Monk is now recognized by the later level-10 widening slice \
         (tests/sd13_monk_level10_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the monk path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_monk_level9_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.monk.")
                || e.id.starts_with("class_feature.monk.")),
        "the Fighter chassis must not surface any monk-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Monk is not promoted -----

#[test]
fn multiclass_monk_level9_is_not_promoted_by_this_slice() {
    let multiclass = MONK_LEVEL9_FIXTURE.replace(
        "class_level=class:monk:9",
        "class_level=class:monk:9\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.monk.")
                || e.id.starts_with("class_feature.monk.")),
        "multiclass Monk must not gain any bounded monk explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Monk must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-9 widening -----

#[test]
fn matrix_monk_row_names_level_9_widening() {
    let matrix = seeded_current_truth();
    let monk = matrix
        .row("class.monk.bounded_progression")
        .expect("monk bounded_progression row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(monk.support_state, SupportState::Supported);
    assert_eq!(monk.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        monk.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        monk.grounding_ref.contains("sd13_monk_level9_progression"),
        "monk row must cite the live SD13-E5 level-9 proof surface: {}",
        monk.grounding_ref
    );
    let note = monk.blocker_or_lossiness_note;
    assert!(
        note.contains("level 9") || note.contains("level-9"),
        "monk partial note must name the level-9 widening: {note}"
    );
}
