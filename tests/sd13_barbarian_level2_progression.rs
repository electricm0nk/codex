//! SD13-E5 Barbarian level-2 progression grounding proof.
//!
//! Widens the accepted Barbarian level-1 martial chassis baseline
//! (`tests/sd13_barbarian_level1_chassis_baseline.rs`) to barbarian level 2,
//! mirroring the Fighter `supported_fighter_level` / Paladin
//! `supported_paladin_level` / Rogue `supported_rogue_level` level-range-gate
//! idiom (the level-1-only gate `martial_level1_class` is generalized to
//! `supported_barbarian_level`, an `Option<u8>` helper gated by
//! `MAX_SUPPORTED_BARBARIAN_LEVEL = 2`). It proves:
//!
//! - base attack bonus at level 2 is grounded by the same full-BAB formula
//!   (`classlevel`) already grounded at level 1: `2`.
//! - base saves at level 2 are grounded by the same good-Fortitude/poor-Reflex/
//!   poor-Will formulas already grounded at level 1, extended to level 2:
//!   Fortitude `2 / 2 + 2 = 3`, Reflex `2 / 3 = 0`, Will `2 / 3 = 0`.
//! - fast movement stays the flat +10 ft. value at level 2, confirmed via the
//!   same formula, not a new record — the PF1 Core Rulebook fast-movement
//!   bonus does not scale with level.
//! - rage rounds per day at level 2 grows by the PF1 Core Rulebook Rage rule
//!   ("at each level after 1st, she can rage for 2 additional rounds"):
//!   `4 + Constitution modifier + 2 * (level - 1)`. On the Con 16 fixture
//!   (modifier +3) this is `4 + 3 + 2 * 1 = 9` at level 2 (was `7` at level
//!   1). The claim-blocking-instead-of-fabricating behavior for a
//!   non-positive sum is preserved at level 2.
//! - the four flat while-raging constants (+4 Str, +4 Con, +2 Will, -2 AC)
//!   stay exactly the same magnitudes at level 2, confirmed via the same
//!   formula, not new records — the PF1 Core Rulebook Rage constants do not
//!   scale with level.
//! - the illiteracy-absence rules-correction record still applies,
//!   unconditionally, at level 2.
//!
//! - Uncanny Dodge, the PF1 Core Rulebook Barbarian's 2nd-level "Special"
//!   class table entry (verified independently against d20pfsrd and
//!   legacy.aonprd.com: the level-2 row reads "Rage power, uncanny dodge"),
//!   is grounded as a bounded identity/recognition record only (value 0),
//!   mirroring exactly how Rogue's/Monk's own Evasion and Druid's Woodland
//!   Stride were grounded: a level-gate-absence record below level 2, a
//!   granted-but-unexecuted rule-text recognition record at or above it, with
//!   no flat-footed-state tracking, no Armor Class computation, and no
//!   invisibility-detection engine implemented anywhere in this codebase to
//!   apply it. The level-2 row's OTHER named entry, a Rage Power choice (a
//!   genuinely open-ended choice-list feature, a new-subsystem-shaped
//!   burden), is deliberately left named-but-unproven this slice, mirroring
//!   how the Monk level-2 bonus feat grant and the Bard Versatile
//!   Performance were each deliberately left unrecognized by their own
//!   level-2 widening slices: no new choice-slot and no new diagnostic was
//!   added for it.
//!
//! It deliberately does not implement the rage-state execution engine
//! (activation, round consumption, fatigue, stat application), weapon
//! familiarity, flat-footed-state tracking, Armor Class computation,
//! invisibility detection, the Rage Power choice-list feature, or level-3+
//! Barbarian progression. It also preserves the accepted Barbarian level-1
//! truth (unchanged), the Fighter/Paladin/Ranger negative controls, and the
//! multiclass negative control.

use codex::rules_core::pilot_compute::{
    compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARBARIAN_LEVEL1_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level1_sd13_deterministic_input.txt"
);

const BARBARIAN_LEVEL2_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level2_sd13_deterministic_input.txt"
);

const BARBARIAN_LEVEL1_LOW_CONSTITUTION_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level1_low_constitution_sd13_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BARBARIAN_UNCANNY_DODGE_ID: &str = "class_feature.barbarian.uncanny_dodge";

// ----- Base attack bonus at level 2 -----

#[test]
fn barbarian_level2_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(BARBARIAN_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(
        base_attack.value, 2,
        "Barbarian level 2 full-BAB progression (classlevel) must equal 2: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("level 2"),
        "barbarian base-attack explanation must name level 2: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 2 -----

#[test]
fn barbarian_level2_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(BARBARIAN_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(fortitude.value, 3, "Barbarian level 2 good Fortitude (2/2+2) must equal 3");
    assert!(
        fortitude.detail.to_lowercase().contains("good"),
        "barbarian Fortitude explanation must name it as a good save: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.barbarian.base_save.reflex");
    assert_eq!(reflex.value, 0, "Barbarian level 2 poor Reflex (2/3) must equal 0");
    assert!(
        reflex.detail.to_lowercase().contains("poor"),
        "barbarian Reflex explanation must name it as a poor save: {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.barbarian.base_save.will");
    assert_eq!(will.value, 0, "Barbarian level 2 poor Will (2/3) must equal 0");
    assert!(
        will.detail.to_lowercase().contains("poor"),
        "barbarian Will explanation must name it as a poor save: {}",
        will.detail
    );
}

// ----- Fast movement stays the flat +10 ft. value at level 2 -----

#[test]
fn barbarian_level2_fast_movement_stays_flat_ten_feet() {
    let input = load(BARBARIAN_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fast_movement = explanation(&computation, "class_chassis.barbarian.fast_movement");
    assert_eq!(
        fast_movement.value, 10,
        "Barbarian fast movement must stay +10 ft. at level 2, not a new record: {}",
        fast_movement.detail
    );
}

// ----- Rage rounds per day grows by the PF1 CRB +2-rounds-per-level rule -----

#[test]
fn barbarian_level2_rage_rounds_per_day_grows_by_two() {
    let input = load(BARBARIAN_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Con 16 -> modifier +3. Level 2: 4 + 3 + 2 * (2 - 1) = 9 (was 7 at level 1).
    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 9,
        "Barbarian level 2 rage rounds per day must be 4 + Con modifier (+3) + 2 * (level - 1) \
         = 9: {}",
        rage_rounds.detail
    );
    assert!(
        rage_rounds.detail.contains("2 additional")
            || rage_rounds.detail.contains("2 * (level"),
        "rage rounds per day at level 2 must document the +2-rounds-per-level-after-1st rule: {}",
        rage_rounds.detail
    );
}

#[test]
fn barbarian_level2_claim_blocks_rage_rounds_per_day_at_low_constitution() {
    // Con 1 -> modifier -5. Level 2: 4 + (-5) + 2 * (2 - 1) = 1, which IS a positive
    // count at level 2 (unlike level 1, where the same Con score claim-blocks). This
    // proves the level-2 formula is a real generalization, not a copy-pasted level-1
    // constant.
    let level_2_low_con =
        BARBARIAN_LEVEL1_LOW_CONSTITUTION_FIXTURE.replace("class:barbarian:1", "class:barbarian:2");
    let input = load(&level_2_low_con);
    let computation = compute_pilot_base_chassis(&input);

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 1,
        "Barbarian level 2 rage rounds per day at Con modifier -5 must be 4 + (-5) + 2 = 1: {}",
        rage_rounds.detail
    );
}

// ----- Flat rage constants are unchanged at level 2 -----

#[test]
fn barbarian_level2_flat_rage_constants_are_unchanged() {
    let input = load(BARBARIAN_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for (id, expected) in [
        ("class_chassis.barbarian.rage.strength_morale_bonus", 4),
        ("class_chassis.barbarian.rage.constitution_morale_bonus", 4),
        ("class_chassis.barbarian.rage.will_save_morale_bonus", 2),
        ("class_chassis.barbarian.rage.armor_class_penalty", -2),
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, expected,
            "rage constant '{id}' must stay {expected} at level 2, not a new record"
        );
    }
}

// ----- Illiteracy-absence record still applies at level 2 -----

#[test]
fn barbarian_level2_illiteracy_absence_still_applies() {
    let input = load(BARBARIAN_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let illiteracy_absent = explanation(&computation, "class_chassis.barbarian.illiteracy_absent");
    assert_eq!(
        illiteracy_absent.value, 0,
        "the illiteracy-absent record documents a rules correction; it carries no mechanical value"
    );
}

// ----- Uncanny Dodge is granted at level 2, as an identity/recognition record only -----

#[test]
fn barbarian_level2_grounds_uncanny_dodge_as_identity_record_only() {
    let input = load(BARBARIAN_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uncanny_dodge = explanation(&computation, BARBARIAN_UNCANNY_DODGE_ID);
    assert_eq!(
        uncanny_dodge.value, 0,
        "Uncanny Dodge must carry no fabricated mechanical value: {}",
        uncanny_dodge.detail
    );
    assert!(
        uncanny_dodge.detail.contains("Uncanny Dodge"),
        "uncanny dodge explanation must name the Uncanny Dodge class feature: {}",
        uncanny_dodge.detail
    );
    assert!(
        uncanny_dodge.detail.to_lowercase().contains("flat-footed")
            && uncanny_dodge.detail.to_lowercase().contains("invisible"),
        "uncanny dodge explanation must state the actual rule text (cannot be caught \
         flat-footed, retains Dexterity bonus to AC even against an invisible attacker): {}",
        uncanny_dodge.detail
    );
    assert!(
        uncanny_dodge.detail.to_lowercase().contains("immobilized"),
        "uncanny dodge explanation must disclaim the immobilized exception (still loses the \
         Dexterity bonus to AC if immobilized): {}",
        uncanny_dodge.detail
    );
    assert!(
        uncanny_dodge.detail.contains("flat-footed-state")
            && uncanny_dodge.detail.contains("Armor Class")
            && uncanny_dodge.detail.contains("invisibility-detection"),
        "uncanny dodge explanation must disclaim a flat-footed-state tracking engine, an Armor \
         Class computation, and an invisibility-detection engine: {}",
        uncanny_dodge.detail
    );
    assert!(
        uncanny_dodge.detail.to_lowercase().contains("granted"),
        "uncanny dodge explanation at level 2 must state it is granted, not absent: {}",
        uncanny_dodge.detail
    );
}

#[test]
fn barbarian_level1_uncanny_dodge_is_a_correct_level_gate_absence() {
    let input = load(BARBARIAN_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uncanny_dodge = explanation(&computation, BARBARIAN_UNCANNY_DODGE_ID);
    assert_eq!(
        uncanny_dodge.value, 0,
        "Uncanny Dodge at level 1 must be a correct level-gate absence, value 0: {}",
        uncanny_dodge.detail
    );
    assert!(
        uncanny_dodge.detail.to_lowercase().contains("absent"),
        "uncanny dodge explanation at level 1 must state it is correctly absent: {}",
        uncanny_dodge.detail
    );
}

// ----- Still blocked: rage-state execution engine and generic diagnostics -----

#[test]
fn barbarian_level2_stays_blocked_on_rage_execution() {
    let input = load(BARBARIAN_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.barbarian.rage_execution.rounds_exceeded")
    {
        Some(blocker) => assert!(blocker.claim_blocking, "if the blocker fires, it must be claim-blocking"),
        None => {
            let not_raging = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.barbarian.rage_execution.not_raging");
            assert!(
                not_raging.is_some(),
                "level-2 Barbarian must ground an honest not-raging record when no rage \
                 posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
    assert_eq!(
        computation.base_attack_bonus, 2,
        "barbarian is now recognized by table_class_id; level 2 full BAB is +2"
    );
}

// ----- Negative control: level 3 was later widened into the supported tranche -----
//
// This control originally asserted level 3 stayed unrecognized by the level-2
// widening slice. A later SD13-E5 slice (tests/sd13_barbarian_level3_progression.rs)
// widened `supported_barbarian_level` to 1..=3 and grounds level 3 for real, so this
// control is renamed to document that widening rather than assert a now-false
// negative; the level-4 negative control moved to the new level-3 test file,
// mirroring the Rogue/Monk/Cleric/Bard/Druid/Sorcerer/Wizard/Ranger precedent.

#[test]
fn barbarian_level_3_was_later_widened_into_the_supported_tranche() {
    let level_3 = BARBARIAN_LEVEL2_FIXTURE.replace("class:barbarian:2", "class:barbarian:3");
    let input = load(&level_3);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")),
        "level-3 Barbarian was later widened into the supported tranche by \
         tests/sd13_barbarian_level3_progression.rs and must now gain bounded barbarian chassis \
         explanations: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the barbarian path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_barbarian_level2_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")),
        "the Fighter chassis must not surface any barbarian-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Barbarian is not promoted -----

#[test]
fn multiclass_barbarian_level2_is_not_promoted_by_this_slice() {
    let multiclass = BARBARIAN_LEVEL2_FIXTURE.replace(
        "class_level=class:barbarian:2",
        "class_level=class:barbarian:2\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")),
        "multiclass Barbarian must not gain any bounded barbarian chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Barbarian must stay claim-blocked in this slice"
    );
}

// ----- Barbarian level 1 stays unchanged -----

#[test]
fn barbarian_level1_truth_is_unchanged_by_the_level2_widening() {
    let input = load(BARBARIAN_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(base_attack.value, 1, "Barbarian level 1 full BAB must stay +1");

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 7,
        "Barbarian level 1 rage rounds per day must stay 4 + Con modifier (+3) = 7"
    );
}

// ----- Control plane: the matrix note names the level-2 widening -----

#[test]
fn matrix_barbarian_row_names_level_2_widening() {
    let matrix = seeded_current_truth();
    let barbarian = matrix
        .row("class.barbarian.bounded_progression")
        .expect("barbarian bounded_progression row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(barbarian.support_state, SupportState::Supported);
    assert_eq!(barbarian.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        barbarian.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        barbarian
            .grounding_ref
            .contains("sd13_barbarian_level2_progression"),
        "barbarian row must cite the live SD13-E5 level-2 proof surface: {}",
        barbarian.grounding_ref
    );
    let note = barbarian.blocker_or_lossiness_note;
    assert!(
        note.contains("level 2") || note.contains("level-2"),
        "barbarian partial note must name the level-2 widening: {note}"
    );
    assert!(
        note.contains("rage execution") || note.contains("rage-state execution"),
        "barbarian partial note must keep naming the rage-state execution engine as unproven: \
         {note}"
    );
    assert!(
        note.to_lowercase().contains("uncanny dodge"),
        "barbarian partial note must name Uncanny Dodge as newly grounded: {note}"
    );
    assert!(
        note.to_lowercase().contains("rage power"),
        "barbarian partial note must keep naming the Rage Power choice-list feature as \
         unproven: {note}"
    );
}
