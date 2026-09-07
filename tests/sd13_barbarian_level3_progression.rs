//! SD13-E5 Barbarian level-3 progression grounding proof.
//!
//! Widens the accepted Barbarian level-1/level-2 martial chassis baseline
//! (`tests/sd13_barbarian_level1_chassis_baseline.rs`,
//! `tests/sd13_barbarian_level2_progression.rs`) to barbarian level 3,
//! mirroring the Fighter/Paladin/Rogue/Monk level-range-gate idiom
//! (`supported_barbarian_level` is generalized from `1..=2` to `1..=3` via
//! `MAX_SUPPORTED_BARBARIAN_LEVEL = 3`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Barbarian class table) were read directly
//! before writing any code or test: level 3 base attack bonus is +3, saves
//! are Fort +3 / Ref +1 / Will +1, and the level-3 "Special" column reads
//! "Trap sense +1". It proves:
//!
//! - base attack bonus at level 3 is grounded by the same full-BAB formula
//!   (`classlevel`) already grounded at levels 1-2: `3`.
//! - base saves at level 3 are grounded by the same good-Fortitude/poor-
//!   Reflex/poor-Will formulas already grounded at levels 1-2, extended to
//!   level 3: Fortitude `3 / 2 + 2 = 3`, Reflex `3 / 3 = 1`, Will `3 / 3 = 1`.
//! - fast movement stays the flat +10 ft. value at level 3, confirmed via the
//!   same formula, not a new record — the PF1 Core Rulebook fast-movement
//!   bonus does not scale with level.
//! - rage rounds per day at level 3 grows by the PF1 Core Rulebook Rage rule
//!   ("at each level after 1st, she can rage for 2 additional rounds"):
//!   `4 + Constitution modifier + 2 * (level - 1)`. On the Con 16 fixture
//!   (modifier +3) this is `4 + 3 + 2 * 2 = 11` at level 3 (was `9` at level
//!   2, `7` at level 1) — i.e. Con modifier + 8, matching the PF1 CRB's own
//!   progression.
//! - the four flat while-raging constants (+4 Str, +4 Con, +2 Will, -2 AC)
//!   stay exactly the same magnitudes at level 3, confirmed via the same
//!   formula, not new records — the PF1 Core Rulebook Rage constants do not
//!   scale with level.
//! - the illiteracy-absence rules-correction record still applies,
//!   unconditionally, at level 3.
//! - Uncanny Dodge stays granted at level 3 (not re-derived), grounded as the
//!   same bounded identity/recognition record already grounded at level 2.
//! - Trap Sense, the PF1 Core Rulebook's 3rd-level Barbarian class feature
//!   (verified independently against d20pfsrd and legacy.aonprd.com: "at 3rd
//!   level, a barbarian gains a +1 bonus on Reflex saves made to avoid traps
//!   and a +1 dodge bonus to AC against attacks made by traps; these bonuses
//!   increase by +1 every three barbarian levels thereafter"), is grounded as
//!   a bounded flat-magnitude record only (barbarian level / 3, floor; `+1`
//!   at level 3) — mirroring the Fighter Bravery / Paladin Divine Grace /
//!   Rogue Trap Sense idiom: the magnitude is never applied to any actual
//!   Reflex-save total or AC total, since no saving-throw-resolution or
//!   armor-class-resolution engine exists in this codebase, and no
//!   trap-detection or trap-triggering engine exists to decide when it would
//!   apply.
//!
//! It deliberately does not implement the rage-state execution engine
//! (activation, round consumption, fatigue, stat application), weapon
//! familiarity, the Rage Power choice-list feature (a genuinely open-ended
//! choice-list feature, a new-subsystem-shaped burden), flat-footed-state
//! tracking, Armor Class computation, invisibility detection, or level-4+
//! Barbarian progression. It also preserves the accepted Barbarian
//! level-1/level-2 truth (unchanged), the Fighter negative control, and the
//! multiclass negative control.

use codex::rules_core::pilot_compute::{
    compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const BARBARIAN_LEVEL2_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level2_sd13_deterministic_input.txt"
);

const BARBARIAN_LEVEL3_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level3_sd13_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BARBARIAN_UNCANNY_DODGE_ID: &str = "class_feature.barbarian.uncanny_dodge";
const BARBARIAN_TRAP_SENSE_ID: &str = "class_feature.barbarian.trap_sense";

// ----- Base attack bonus at level 3 -----

#[test]
fn barbarian_level3_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(BARBARIAN_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(
        base_attack.value, 3,
        "Barbarian level 3 full-BAB progression (classlevel) must equal 3: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("level 3"),
        "barbarian base-attack explanation must name level 3: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 3 -----

#[test]
fn barbarian_level3_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(BARBARIAN_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(fortitude.value, 3, "Barbarian level 3 good Fortitude (3/2+2) must equal 3");
    assert!(
        fortitude.detail.to_lowercase().contains("good"),
        "barbarian Fortitude explanation must name it as a good save: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.barbarian.base_save.reflex");
    assert_eq!(reflex.value, 1, "Barbarian level 3 poor Reflex (3/3) must equal 1");
    assert!(
        reflex.detail.to_lowercase().contains("poor"),
        "barbarian Reflex explanation must name it as a poor save: {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.barbarian.base_save.will");
    assert_eq!(will.value, 1, "Barbarian level 3 poor Will (3/3) must equal 1");
    assert!(
        will.detail.to_lowercase().contains("poor"),
        "barbarian Will explanation must name it as a poor save: {}",
        will.detail
    );
}

// ----- Fast movement stays the flat +10 ft. value at level 3 -----

#[test]
fn barbarian_level3_fast_movement_stays_flat_ten_feet() {
    let input = load(BARBARIAN_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fast_movement = explanation(&computation, "class_chassis.barbarian.fast_movement");
    assert_eq!(
        fast_movement.value, 10,
        "Barbarian fast movement must stay +10 ft. at level 3, not a new record: {}",
        fast_movement.detail
    );
}

// ----- Rage rounds per day grows by the PF1 CRB +2-rounds-per-level rule -----

#[test]
fn barbarian_level3_rage_rounds_per_day_grows_by_two_again() {
    let input = load(BARBARIAN_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Con 16 -> modifier +3. Level 3: 4 + 3 + 2 * (3 - 1) = 11 (was 9 at level 2,
    // 7 at level 1) -- i.e. Con modifier + 8, matching the PF1 CRB's own
    // "+2 rounds per level after 1st" progression through level 3.
    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 11,
        "Barbarian level 3 rage rounds per day must be 4 + Con modifier (+3) + 2 * (level - 1) \
         = 11: {}",
        rage_rounds.detail
    );
    assert!(
        rage_rounds.detail.contains("2 additional")
            || rage_rounds.detail.contains("2 * (level"),
        "rage rounds per day at level 3 must document the +2-rounds-per-level-after-1st rule: {}",
        rage_rounds.detail
    );
}

// ----- Flat rage constants are unchanged at level 3 -----

#[test]
fn barbarian_level3_flat_rage_constants_are_unchanged() {
    let input = load(BARBARIAN_LEVEL3_FIXTURE);
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
            "rage constant '{id}' must stay {expected} at level 3, not a new record"
        );
    }
}

// ----- Illiteracy-absence record still applies at level 3 -----

#[test]
fn barbarian_level3_illiteracy_absence_still_applies() {
    let input = load(BARBARIAN_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let illiteracy_absent = explanation(&computation, "class_chassis.barbarian.illiteracy_absent");
    assert_eq!(
        illiteracy_absent.value, 0,
        "the illiteracy-absent record documents a rules correction; it carries no mechanical value"
    );
}

// ----- Uncanny Dodge stays granted at level 3, not re-derived -----

#[test]
fn barbarian_level3_keeps_uncanny_dodge_grounded() {
    let input = load(BARBARIAN_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uncanny_dodge = explanation(&computation, BARBARIAN_UNCANNY_DODGE_ID);
    assert_eq!(
        uncanny_dodge.value, 0,
        "Uncanny Dodge must carry no fabricated mechanical value at level 3: {}",
        uncanny_dodge.detail
    );
    assert!(
        uncanny_dodge.detail.to_lowercase().contains("granted"),
        "uncanny dodge explanation at level 3 must state it is granted, not absent: {}",
        uncanny_dodge.detail
    );
}

// ----- Trap Sense is granted at level 3, as a flat +1 magnitude record -----

#[test]
fn barbarian_level3_grounds_trap_sense_as_a_flat_plus_one_magnitude() {
    let input = load(BARBARIAN_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 1,
        "Trap Sense must ground the PF1 Core Rulebook's barbarian-level/3 formula (+1 at level \
         3): {}",
        trap_sense.detail
    );
    assert!(
        trap_sense.detail.contains("Trap Sense"),
        "trap sense explanation must name the Trap Sense class feature: {}",
        trap_sense.detail
    );
    assert!(
        trap_sense.detail.to_lowercase().contains("reflex")
            && trap_sense.detail.to_lowercase().contains("ac"),
        "trap sense explanation must state the actual rule text (Reflex save bonus vs. traps and \
         an equal dodge bonus to AC against attacks made by traps): {}",
        trap_sense.detail
    );
    assert!(
        trap_sense.detail.to_lowercase().contains("granted"),
        "trap sense explanation at level 3 must state it is granted, not absent: {}",
        trap_sense.detail
    );
    assert!(
        trap_sense.detail.to_lowercase().contains("never applied")
            || trap_sense.detail.to_lowercase().contains("not applied")
            || trap_sense.detail.to_lowercase().contains("no saving-throw-resolution"),
        "trap sense explanation must disclaim being applied to any actual save or AC total: {}",
        trap_sense.detail
    );
}

#[test]
fn barbarian_level2_trap_sense_is_a_correct_level_gate_absence() {
    let input = load(BARBARIAN_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 0,
        "Trap Sense at level 2 must be a correct level-gate absence, value 0: {}",
        trap_sense.detail
    );
    assert!(
        trap_sense.detail.to_lowercase().contains("absent"),
        "trap sense explanation at level 2 must state it is correctly absent: {}",
        trap_sense.detail
    );
}

// ----- Still blocked: rage-state execution engine and generic diagnostics -----

#[test]
fn barbarian_level3_stays_blocked_on_rage_execution() {
    let input = load(BARBARIAN_LEVEL3_FIXTURE);
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
                "level-3 Barbarian must ground an honest not-raging record when no rage \
                 posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
    assert_eq!(
        computation.base_attack_bonus, 3,
        "barbarian is now recognized by table_class_id; level 3 full BAB is +3"
    );
}

// ----- Negative control: level 4 was later widened into the supported tranche -----
//
// This control originally asserted level 4 stayed unrecognized by the level-3
// widening slice. A later SD13-E5 slice (tests/sd13_barbarian_level4_progression.rs)
// widened `supported_barbarian_level` to 1..=4 and grounds level 4 for real, so this
// control is renamed to document that widening rather than assert a now-false
// negative; the level-5 negative control moved to the new level-4 test file,
// mirroring the Rogue/Monk/Cleric/Bard/Druid/Sorcerer/Wizard/Ranger precedent.

#[test]
fn barbarian_level_4_was_later_widened_into_the_supported_tranche() {
    let level_4 = BARBARIAN_LEVEL3_FIXTURE.replace("class:barbarian:3", "class:barbarian:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")),
        "level-4 Barbarian was later widened into the supported tranche by \
         tests/sd13_barbarian_level4_progression.rs and must now gain bounded barbarian chassis \
         explanations: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, BARBARIAN_UNCANNY_DODGE_ID),
        "level-4 Barbarian must keep the Uncanny Dodge explanation grounded"
    );
    assert!(
        has_explanation(&computation, BARBARIAN_TRAP_SENSE_ID),
        "level-4 Barbarian must keep the Trap Sense explanation grounded"
    );
}

// ----- Negative control: the barbarian path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_barbarian_level3_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")
                || e.id == BARBARIAN_UNCANNY_DODGE_ID
                || e.id == BARBARIAN_TRAP_SENSE_ID),
        "the Fighter chassis must not surface any barbarian-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Barbarian is not promoted -----

#[test]
fn multiclass_barbarian_level3_is_not_promoted_by_this_slice() {
    let multiclass = BARBARIAN_LEVEL3_FIXTURE.replace(
        "class_level=class:barbarian:3",
        "class_level=class:barbarian:3\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")
                || e.id == BARBARIAN_UNCANNY_DODGE_ID
                || e.id == BARBARIAN_TRAP_SENSE_ID),
        "multiclass Barbarian must not gain any bounded barbarian chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Barbarian must stay claim-blocked in this slice"
    );
}

// ----- Barbarian level 1/level 2 stays unchanged -----

#[test]
fn barbarian_level2_truth_is_unchanged_by_the_level3_widening() {
    let input = load(BARBARIAN_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(base_attack.value, 2, "Barbarian level 2 full BAB must stay +2");

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 9,
        "Barbarian level 2 rage rounds per day must stay 4 + Con modifier (+3) + 2 = 9"
    );
}

// ----- Control plane: the matrix note names the level-3 widening and Trap Sense -----

#[test]
fn matrix_barbarian_row_names_level_3_widening_and_trap_sense() {
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
            .contains("sd13_barbarian_level3_progression"),
        "barbarian row must cite the live SD13-E5 level-3 proof surface: {}",
        barbarian.grounding_ref
    );
    let note = barbarian.blocker_or_lossiness_note;
    assert!(
        note.contains("level 3") || note.contains("level-3"),
        "barbarian partial note must name the level-3 widening: {note}"
    );
    assert!(
        note.to_lowercase().contains("trap sense"),
        "barbarian partial note must name Trap Sense as newly grounded: {note}"
    );
    assert!(
        note.contains("rage execution") || note.contains("rage-state execution"),
        "barbarian partial note must keep naming the rage-state execution engine as unproven: \
         {note}"
    );
    assert!(
        note.to_lowercase().contains("rage power"),
        "barbarian partial note must keep naming the Rage Power choice-list feature as \
         unproven: {note}"
    );
}
