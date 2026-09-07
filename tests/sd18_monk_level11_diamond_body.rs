//! SD18 Monk level-11 widening grounding proof.
//!
//! Widens the accepted Monk level-1..level-10 martial chassis baseline
//! (`tests/sd13_monk_level10_progression.rs`, the SD13 tranche's declared
//! ceiling) to Monk level 11 — the sixth SD-18 §3.2 class-row widening,
//! mirroring the sibling-class level-range-gate idiom
//! (`supported_monk_level` is generalized from `1..=10` to `1..=11` via
//! `MAX_SUPPORTED_MONK_LEVEL = 11`, exactly as `cycle-2026-07-13T1255`
//! widened `MAX_SUPPORTED_BARBARIAN_LEVEL`, `cycle-2026-07-13T1830` widened
//! `MAX_SUPPORTED_BARD_LEVEL`, `cycle-2026-07-13T2007` widened
//! `MAX_SUPPORTED_CLERIC_LEVEL`, `cycle-2026-07-13T1851` widened
//! `MAX_SUPPORTED_DRUID_LEVEL`, and `cycle-2026-07-13T1941` widened
//! `MAX_SUPPORTED_FIGHTER_LEVEL`, all from 10 to 11). Both PF1 CRB primary
//! sources (d20pfsrd and legacy.aonprd.com Monk class table) were read
//! directly before writing any code or test:
//!
//! - level 11 base attack bonus is +8 (`11 * 3 / 4 = 8`, genuinely risen
//!   from +7 at level 10 — the class table's own "+8/+3" iterative notation
//!   is not modeled anywhere in this codebase, only the flat base value)
//!   while all three base saves STAY +7 (all good, `11 / 2 + 2 = 7`, an
//!   integer-division coincidence with level 10) — confirmed by the same
//!   formulas already grounded at levels 1-10, not re-derived.
//! - the unarmed strike damage die STAYS 1d10 (the 1d10 band spans monk
//!   levels 8-11 per both primary sources).
//! - the Flurry of Blows flat attack modifier GENUINELY RISES to +9 (monk
//!   level - 2) while the attack count STAYS 3 (the next count change lands
//!   at 15th).
//! - the ki pool STAYS 8 (`11 / 2 + Wisdom modifier 3 = 8`, an
//!   integer-division coincidence with level 10), and Slow Fall's reach
//!   STAYS 50 ft (the level-11 "Special" column names no reach rise; the
//!   next rise, to 60 ft, lands at 12th per both primary sources).
//! - the PF1 Core Rulebook Monk class table's level-11 "Special" column
//!   reads "Diamond body" ONLY (verified independently against both
//!   primary sources, checked rather than assumed away) — a flat
//!   poison-immunity grant, grounded here as a new bounded grant-only
//!   identity record (`class_chassis.monk.diamond_body`, value 0,
//!   non-fabricated), mirroring exactly how Purity of Body's own
//!   disease-immunity grant was grounded at 5th level: no
//!   poison-resolution engine exists anywhere in this codebase to apply
//!   the immunity to.
//! - Still Mind, Evasion, Improved Evasion, and Purity of Body all stay
//!   granted +0 identity records; the level-1 bonus-feat choice
//!   recognition still fires and its execution burden still claim-blocks.
//!
//! It deliberately does not touch the ki-power execution burden, the repeat
//! bonus-feat grants, High Jump, Wholeness of Body, the ki-strike
//! (lawful) DR-bypass property, or any fall-damage/attack-resolution
//! engine (all stay named-but-unproven, unchanged from levels 1-10), and
//! it does not ground Monk level 12+. It also preserves the accepted Monk
//! level-1..level-10 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const MONK_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level10_sd13_deterministic_input.txt");

const MONK_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_monk_level11_sd18_diamond_body_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const MONK_STILL_MIND_ID: &str = "class_feature.monk.still_mind";
const MONK_EVASION_ID: &str = "class_feature.monk.evasion";
const MONK_IMPROVED_EVASION_ID: &str = "class_feature.monk.improved_evasion";
const MONK_PURITY_OF_BODY_ID: &str = "class_chassis.monk.purity_of_body";
const MONK_KI_POOL_ID: &str = "class_chassis.monk.ki_pool_size";
const MONK_SLOW_FALL_ID: &str = "class_chassis.monk.slow_fall";
const MONK_DIAMOND_BODY_ID: &str = "class_chassis.monk.diamond_body";

// ----- Base attack bonus rises, saves stay unchanged at level 11 -----

#[test]
fn monk_level11_base_attack_rises_and_saves_stay_unchanged() {
    let input = load(MONK_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.monk.base_attack_bonus");
    assert_eq!(
        base_attack.value, 8,
        "Monk level 11 3/4-BAB progression (11 * 3 / 4) must equal 8, genuinely risen from 7 \
         at level 10: {}",
        base_attack.detail
    );

    for (id, label) in [
        ("class_chassis.monk.base_save.fortitude", "Fortitude"),
        ("class_chassis.monk.base_save.reflex", "Reflex"),
        ("class_chassis.monk.base_save.will", "Will"),
    ] {
        let save = explanation(&computation, id);
        assert_eq!(
            save.value, 7,
            "Monk level 11 good {label} (11/2+2) must stay 7, an integer-division coincidence \
             with level 10"
        );
    }
}

// ----- Unarmed die and ki pool/slow fall stay unchanged; flurry bonus rises -----

#[test]
fn monk_level11_unarmed_die_and_ki_pool_stay_flurry_bonus_rises() {
    let input = load(MONK_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let unarmed = explanation(&computation, "class_chassis.monk.unarmed_strike_damage_die");
    assert_eq!(
        unarmed.value, 10,
        "Monk level 11 unarmed strike die must stay 1d10 (the band spans levels 8-11): {}",
        unarmed.detail
    );

    let attack_bonus = explanation(
        &computation,
        "class_chassis.monk.flurry_of_blows_attack_bonus",
    );
    assert_eq!(
        attack_bonus.value, 9,
        "Monk level 11 Flurry flat attack modifier (monk level - 2) must equal 9, genuinely \
         risen from 8 at level 10: {}",
        attack_bonus.detail
    );

    let attack_count = explanation(
        &computation,
        "class_chassis.monk.flurry_of_blows_attack_count",
    );
    assert_eq!(
        attack_count.value, 3,
        "Monk level 11 Flurry attack count must stay 3 (the next count change lands at 15th)"
    );

    // CG-03 fix: Wisdom modifier is now +4 (base 17 + 2 Human racial), not +3.
    let ki_pool = explanation(&computation, MONK_KI_POOL_ID);
    assert_eq!(
        ki_pool.value, 9,
        "Monk level 11 ki pool (11/2 + Wisdom modifier 4) must stay 9, an integer-division \
         coincidence with level 10: {}",
        ki_pool.detail
    );

    let slow_fall = explanation(&computation, MONK_SLOW_FALL_ID);
    assert!(
        slow_fall.detail.contains("50"),
        "Monk level 11 Slow Fall reach must stay 50 ft (the next rise, to 60 ft, lands at \
         12th): {}",
        slow_fall.detail
    );
}

// ----- Diamond Body is newly granted at level 11 -----

#[test]
fn monk_level11_diamond_body_is_newly_granted() {
    let input = load(MONK_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let diamond_body = explanation(&computation, MONK_DIAMOND_BODY_ID);
    assert_eq!(
        diamond_body.value, 0,
        "Diamond Body must be a bounded grant-only identity record (value 0, non-fabricated): {}",
        diamond_body.detail
    );
    assert!(
        diamond_body.detail.to_lowercase().contains("poison"),
        "Diamond Body's detail must name the poison-immunity grant: {}",
        diamond_body.detail
    );

    let level10_input = load(MONK_LEVEL10_FIXTURE);
    let level10_computation = compute_pilot_base_chassis(&level10_input);
    let level10_diamond_body = explanation(&level10_computation, MONK_DIAMOND_BODY_ID);
    assert_eq!(
        level10_diamond_body.value, 0,
        "Diamond Body must correctly be absent (value 0, level-gate placeholder) below level 11"
    );
    assert!(
        !level10_diamond_body.detail.to_lowercase().contains("granted"),
        "Diamond Body must not claim to be granted at level 10: {}",
        level10_diamond_body.detail
    );
}

// ----- Remaining granted features and choices carry over unchanged -----

#[test]
fn monk_level11_granted_features_and_choices_carry_over() {
    let input = load(MONK_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let still_mind = explanation(&computation, MONK_STILL_MIND_ID);
    assert_eq!(still_mind.value, 2, "Still Mind must stay the flat +2 at level 11");

    for id in [MONK_EVASION_ID, MONK_IMPROVED_EVASION_ID, MONK_PURITY_OF_BODY_ID] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, 0,
            "'{id}' must stay a granted +0 identity record at level 11"
        );
    }

    let choice = explanation(&computation, "class_chassis.monk.bonus_feat_choice");
    assert_eq!(
        choice.value, 0,
        "the level-1 bonus-feat choice recognition must still fire at level 11"
    );

    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.monk.bounded_progression.bonus_feat.unsupported"
            && d.claim_blocking),
        "level-11 Monk must still claim-block on the bonus-feat execution burden: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the level-10 fixture is unaffected by this widening -----

#[test]
fn monk_level10_truth_is_unchanged_by_this_slice() {
    let input = load(MONK_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.monk.base_attack_bonus");
    assert_eq!(base_attack.value, 7, "Monk level 10 base attack bonus must stay 7");

    let ki_pool = explanation(&computation, MONK_KI_POOL_ID);
    assert_eq!(ki_pool.value, 9, "Monk level 10 ki pool must stay 9");
}

// ----- Level 13 was later widened into the supported tranche (task #49) -----
//
// Formerly a "level 12 stays unrecognized" negative control; superseded when
// `tests/sd18_monk_level12_widening.rs` genuinely widened
// `MAX_SUPPORTED_MONK_LEVEL` to 12, then moved to "level 13 stays
// unrecognized". Superseded again when task #49 widened the ceiling all the
// way to 20 (the full PF1 capstone) -- flipped to a positive assertion,
// mirroring how `tests/sd13_monk_level9_progression.rs`'s own level-10
// boundary was flipped once a later slice widened past it.

#[test]
fn monk_level_13_was_later_widened_into_the_supported_tranche() {
    let level_13 = MONK_LEVEL11_FIXTURE.replace("class:monk:11", "class:monk:13");
    let input = load(&level_13);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.monk.")),
        "level-13 Monk is now recognized by the later task #49 level-20-capstone widening \
         slice (tests/sd49_monk_level20_capstone.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the monk path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_monk_level11_recognition() {
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
fn multiclass_monk_level11_is_not_promoted_by_this_slice() {
    let multiclass = MONK_LEVEL11_FIXTURE.replace(
        "class_level=class:monk:11",
        "class_level=class:monk:11\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-11 widening -----

#[test]
fn matrix_monk_row_names_level_11_widening() {
    let matrix = seeded_current_truth();
    let monk = matrix
        .row("class.monk.bounded_progression")
        .expect("monk bounded_progression row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(monk.support_state, SupportState::Supported);
    assert_eq!(monk.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(monk.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        monk.grounding_ref.contains("sd18_monk_level11_diamond_body"),
        "monk row must cite the live SD18 level-11 widening proof surface: {}",
        monk.grounding_ref
    );
    let note = monk.blocker_or_lossiness_note;
    assert!(
        note.contains("level 11") || note.contains("level-11"),
        "monk partial note must name the level-11 widening: {note}"
    );
}
