//! SD13-E5 Monk level-3 progression grounding proof.
//!
//! Widens the accepted Monk level-1/level-2 martial chassis baseline
//! (`tests/sd13_monk_level1_chassis_baseline.rs`,
//! `tests/sd13_monk_level2_progression.rs`) to monk level 3, mirroring the
//! Fighter/Paladin/Rogue/Barbarian level-range-gate idiom (`supported_monk_level`
//! is generalized from `1..=2` to `1..=3` via `MAX_SUPPORTED_MONK_LEVEL = 3`).
//! Both PF1 CRB primary sources (d20pfsrd and legacy.aonprd.com Monk class
//! table) were read directly before writing any code or test: level 3 base
//! attack bonus is +2, all three base saves are +3, unarmed damage stays
//! 1d6, the Flurry of Blows attack bonus improves to +1/+1 (two attacks),
//! and the level-3 special feature list is "Fast movement, maneuver
//! training, still mind." It proves:
//!
//! - base attack bonus at level 3 is grounded by the same 3/4-BAB formula
//!   (`level * 3 / 4`) already grounded at levels 1-2: `3 * 3 / 4 = 2`.
//! - base saves at level 3 are grounded by the same all-good formula
//!   (`level / 2 + 2`) already grounded at levels 1-2: `3 / 2 + 2 = 3` for
//!   Fortitude, Reflex, and Will alike (Monk is unusual in having all three
//!   saves good).
//! - the unarmed strike damage die stays `6` (i.e. `1d6`) at level 3 — the
//!   PF1 Core Rulebook Monk class table only increases the die at level 4
//!   (to 1d8); this is confirmed via the same formula/value, not a new
//!   record.
//! - the Flurry of Blows flat attack bonus is grounded by the same
//!   `level - 2` formula already grounded at levels 1-2, correctly
//!   producing `1` at level 3 (`3 - 2 = 1`, matching the primary source's
//!   "+1/+1"), and the attack count stays `2` at level 3 (Monk gains a
//!   third flurry attack only at a much higher level), confirmed via the
//!   same formula, not a new record.
//! - Evasion stays granted at level 3 (not re-derived), grounded as the
//!   same bounded identity/recognition record already grounded at level 2.
//! - Still Mind, the PF1 Core Rulebook's 3rd-level Monk class feature
//!   (verified independently against d20pfsrd and legacy.aonprd.com: "a
//!   monk of 3rd level or higher gains a +2 bonus on saving throws against
//!   enchantment spells and effects"), is grounded as a bounded
//!   flat-magnitude record only (a flat `+2`, not level-scaled) — mirroring
//!   the Fighter Bravery / Paladin Divine Grace / Rogue Trap Sense idiom:
//!   the magnitude is never applied to any actual save total, since no
//!   saving-throw-resolution engine exists in this codebase.
//!
//! It deliberately does not implement Fast Movement or Maneuver Training,
//! the PF1 CRB's other two 3rd-level Monk "Special" column entries (Fast
//! Movement is a speed bonus with no speed-total engine to attach to;
//! Maneuver Training is a CMB/CMD-substitution rule with no CMB/CMD engine
//! anywhere in this codebase) — both stay named but unproven. It also does
//! not implement the recognized bonus feat's own mechanics (unchanged from
//! level 1), the level-2 bonus feat grant (PF1 grants monks a SEPARATE
//! bonus feat at 2nd level; not recognized by this or any prior slice), the
//! ki pool, any attack-resolution or damage-resolution engine, and it does
//! not ground Monk level 4+. It also preserves the accepted Monk
//! level-1/level-2 truth (unchanged), the Fighter negative control, and the
//! multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const MONK_LEVEL2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level2_sd13_deterministic_input.txt");

const MONK_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level3_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const MONK_EVASION_ID: &str = "class_feature.monk.evasion";
const MONK_STILL_MIND_ID: &str = "class_feature.monk.still_mind";

// ----- Base attack bonus at level 3 -----

#[test]
fn monk_level3_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(MONK_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.monk.base_attack_bonus");
    assert_eq!(
        base_attack.value, 2,
        "Monk level 3 3/4-BAB progression (3 * 3 / 4) must equal 2: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("3/4") || base_attack.detail.to_lowercase().contains("bab"),
        "monk base-attack explanation must name the 3/4-BAB progression: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 3 (all three good) -----

#[test]
fn monk_level3_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(MONK_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.monk.base_save.fortitude");
    assert_eq!(fortitude.value, 3, "Monk level 3 good Fortitude (3/2+2) must equal 3");
    assert!(
        fortitude.detail.to_lowercase().contains("good"),
        "monk Fortitude explanation must name it as a good save: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.monk.base_save.reflex");
    assert_eq!(reflex.value, 3, "Monk level 3 good Reflex (3/2+2) must equal 3");
    assert!(
        reflex.detail.to_lowercase().contains("good"),
        "monk Reflex explanation must name it as a good save: {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.monk.base_save.will");
    assert_eq!(will.value, 3, "Monk level 3 good Will (3/2+2) must equal 3");
    assert!(
        will.detail.to_lowercase().contains("good"),
        "monk Will explanation must name it as a good save: {}",
        will.detail
    );
}

// ----- Unarmed strike damage die stays 1d6 at level 3 -----

#[test]
fn monk_level3_unarmed_strike_damage_die_stays_one_d6() {
    let input = load(MONK_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let unarmed = explanation(&computation, "class_chassis.monk.unarmed_strike_damage_die");
    assert_eq!(
        unarmed.value, 6,
        "Monk level 3 unarmed strike damage die must stay 6 (i.e. 1d6), not a new record: {}",
        unarmed.detail
    );
    assert!(
        unarmed.detail.contains("1d6"),
        "monk unarmed-strike explanation must name the 1d6 damage die at level 3: {}",
        unarmed.detail
    );
}

// ----- Flurry of Blows flat attack bonus/count at level 3 -----

#[test]
fn monk_level3_flurry_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(MONK_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let attack_bonus = explanation(&computation, "class_chassis.monk.flurry_of_blows_attack_bonus");
    assert_eq!(
        attack_bonus.value, 1,
        "Monk level 3 Flurry of Blows flat attack modifier (3 - 2) must equal 1, matching the \
         PF1 CRB table's +1/+1 at level 3: {}",
        attack_bonus.detail
    );
}

#[test]
fn monk_level3_flurry_attack_count_stays_two() {
    let input = load(MONK_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let attack_count = explanation(&computation, "class_chassis.monk.flurry_of_blows_attack_count");
    assert_eq!(
        attack_count.value, 2,
        "Monk level 3 Flurry of Blows attack count must stay 2, not a new record: {}",
        attack_count.detail
    );
}

// ----- Evasion stays granted at level 3, not re-derived -----

#[test]
fn monk_level3_keeps_evasion_grounded() {
    let input = load(MONK_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let evasion = explanation(&computation, MONK_EVASION_ID);
    assert_eq!(
        evasion.value, 0,
        "Evasion must carry no fabricated mechanical value at level 3: {}",
        evasion.detail
    );
    assert!(
        evasion.detail.to_lowercase().contains("granted"),
        "evasion explanation at level 3 must state it is granted, not absent: {}",
        evasion.detail
    );
}

// ----- Still Mind is granted at level 3, as a flat +2 magnitude record -----

#[test]
fn monk_level3_grounds_still_mind_as_a_flat_plus_two_magnitude() {
    let input = load(MONK_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let still_mind = explanation(&computation, MONK_STILL_MIND_ID);
    assert_eq!(
        still_mind.value, 2,
        "Still Mind must ground the PF1 Core Rulebook's flat +2 bonus at level 3: {}",
        still_mind.detail
    );
    assert!(
        still_mind.detail.contains("Still Mind"),
        "still mind explanation must name the Still Mind class feature: {}",
        still_mind.detail
    );
    assert!(
        still_mind.detail.to_lowercase().contains("enchantment"),
        "still mind explanation must state the actual rule text (bonus vs. enchantment spells \
         and effects): {}",
        still_mind.detail
    );
    assert!(
        still_mind.detail.to_lowercase().contains("granted"),
        "still mind explanation at level 3 must state it is granted, not absent: {}",
        still_mind.detail
    );
    assert!(
        still_mind.detail.to_lowercase().contains("never applied")
            || still_mind.detail.to_lowercase().contains("not applied")
            || still_mind.detail.to_lowercase().contains("no saving-throw-resolution"),
        "still mind explanation must disclaim being applied to any actual save total: {}",
        still_mind.detail
    );
}

#[test]
fn monk_level2_still_mind_is_a_correct_level_gate_absence() {
    let input = load(MONK_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let still_mind = explanation(&computation, MONK_STILL_MIND_ID);
    assert_eq!(
        still_mind.value, 0,
        "Still Mind at level 2 must be a correct level-gate absence, value 0: {}",
        still_mind.detail
    );
    assert!(
        still_mind.detail.to_lowercase().contains("absent"),
        "still mind explanation at level 2 must state it is correctly absent: {}",
        still_mind.detail
    );
}

// ----- The existing bonus-feat-mechanics diagnostic still fires at level 3 -----

#[test]
fn monk_level3_still_claim_blocks_the_recognized_bonus_feat_mechanics() {
    let input = load(MONK_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.monk.bounded_progression.bonus_feat.unsupported"
                && d.claim_blocking),
        "level-3 Monk must still claim-block on the recognized bonus feat's own mechanics: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 4 was later widened into the supported tranche -----

#[test]
fn monk_level_4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_monk_level4_progression.rs) widened the level-range gate to
    // level 4 (mirroring the Fighter/Paladin/Rogue/Barbarian level-range gate
    // idiom) and grounded the ki pool's flat size and Slow Fall; this negative
    // control is superseded, not violated — pin the new truth here too so this
    // file stays internally consistent. The frontier this file's own slice
    // actually drew is now level 5, covered by
    // `tests/sd13_monk_level4_progression.rs::monk_level_5_is_not_promoted_by_this_slice`.
    let level_4 = MONK_LEVEL3_FIXTURE.replace("class:monk:3", "class:monk:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, "class_chassis.monk.base_attack_bonus"),
        "level-4 Monk is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, MONK_EVASION_ID),
        "level-4 Monk must keep the Evasion explanation grounded at level 2"
    );
    assert!(
        has_explanation(&computation, MONK_STILL_MIND_ID),
        "level-4 Monk must keep the Still Mind explanation grounded at level 3"
    );
}

// ----- Negative control: the monk path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_monk_level3_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.monk.")
                || e.id == MONK_EVASION_ID
                || e.id == MONK_STILL_MIND_ID),
        "the Fighter chassis must not surface any monk-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Monk is not promoted -----

#[test]
fn multiclass_monk_level3_is_not_promoted_by_this_slice() {
    let multiclass = MONK_LEVEL3_FIXTURE.replace(
        "class_level=class:monk:3",
        "class_level=class:monk:3\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.monk.")
                || e.id == MONK_EVASION_ID
                || e.id == MONK_STILL_MIND_ID),
        "multiclass Monk must not gain any bounded monk chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Monk must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-3 widening and Still Mind -----

#[test]
fn matrix_monk_row_names_level_3_widening_and_still_mind() {
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
        monk.grounding_ref.contains("sd13_monk_level3_progression"),
        "monk row must cite the live SD13-E5 level-3 proof surface: {}",
        monk.grounding_ref
    );
    let note = monk.blocker_or_lossiness_note;
    assert!(
        note.to_lowercase().contains("still mind"),
        "monk partial note must name Still Mind as newly grounded: {note}"
    );
    assert!(
        note.contains("bonus feat"),
        "monk partial note must keep naming the bonus feat's own mechanics as unproven: {note}"
    );
}
