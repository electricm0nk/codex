//! SD13-E5 Barbarian level-5 progression grounding proof.
//!
//! Widens the accepted Barbarian level-1/level-2/level-3/level-4 martial
//! chassis baseline (`tests/sd13_barbarian_level1_chassis_baseline.rs`,
//! `tests/sd13_barbarian_level2_progression.rs`,
//! `tests/sd13_barbarian_level3_progression.rs`,
//! `tests/sd13_barbarian_level4_progression.rs`) to barbarian level 5,
//! mirroring the Fighter/Paladin/Rogue/Monk level-range-gate idiom
//! (`supported_barbarian_level` is generalized from `1..=4` to `1..=5` via
//! `MAX_SUPPORTED_BARBARIAN_LEVEL = 5`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Barbarian class table) were read directly
//! before writing any code or test: level 5 base attack bonus is +5, saves
//! are Fort +4 / Ref +1 / Will +1, and the level-5 "Special" column reads
//! "Improved uncanny dodge" -- a genuinely new class feature, not another
//! Rage Power grant. It proves:
//!
//! - base attack bonus at level 5 is grounded by the same full-BAB formula
//!   (`classlevel`) already grounded at levels 1-4: `5`.
//! - base saves at level 5 are grounded by the same good-Fortitude/poor-
//!   Reflex/poor-Will formulas already grounded at levels 1-4, extended to
//!   level 5: Fortitude `5 / 2 + 2 = 4`, Reflex `5 / 3 = 1`, Will `5 / 3 = 1`.
//! - fast movement stays the flat +10 ft. value at level 5, confirmed via the
//!   same formula, not a new record -- the PF1 Core Rulebook fast-movement
//!   bonus does not scale with level.
//! - rage rounds per day at level 5 grows by the PF1 Core Rulebook Rage rule
//!   ("at each level after 1st, she can rage for 2 additional rounds"):
//!   `4 + Constitution modifier + 2 * (level - 1)`. On the Con 16 fixture
//!   (modifier +3) this is `4 + 3 + 2 * 4 = 15` at level 5 (was `13` at level
//!   4) -- i.e. Con modifier + 12, matching the PF1 CRB's own progression.
//! - the four flat while-raging constants (+4 Str, +4 Con, +2 Will, -2 AC)
//!   stay exactly the same magnitudes at level 5, confirmed via the same
//!   formula, not new records -- the PF1 Core Rulebook Rage constants do not
//!   scale with level.
//! - the illiteracy-absence rules-correction record still applies,
//!   unconditionally, at level 5.
//! - Uncanny Dodge stays granted at level 5 (not re-derived), grounded as the
//!   same bounded identity/recognition record already grounded at levels
//!   2-4.
//! - Trap Sense stays granted at level 5 with the SAME +1 magnitude already
//!   grounded at levels 3-4 (barbarian level / 3, floor: `5 / 3 = 1`), not
//!   re-derived -- verified independently against both primary sources that
//!   the Trap Sense bonus does NOT increase again until barbarian level 6.
//!
//! It also verifies (per the operator brief) whether Barbarian gains an
//! actual new class feature at 5th level. Both primary sources' level-5
//! "Special" column reads "Improved uncanny dodge" -- verified independently
//! against d20pfsrd and legacy.aonprd.com: "At 5th level and higher, a
//! barbarian can no longer be flanked. This defense denies a rogue the
//! ability to sneak attack the barbarian by flanking her, unless the
//! attacker has at least four more rogue levels than the target has
//! barbarian levels." The GRANT itself is flat/identity-shaped -- a pure
//! rule-text acknowledgment, exactly mirroring Uncanny Dodge's own bounded
//! identity/recognition record -- so it is newly grounded here
//! (`class_feature.barbarian.improved_uncanny_dodge`, value 0, level-gate
//! absence below level 5, granted-but-unexecuted at or above it). The
//! CONDITIONAL flanking-resolution piece (comparing the attacking rogue's
//! own levels against the barbarian's own levels to decide whether the
//! immunity is actually pierced) is NOT computed: no flanking-resolution
//! engine, no attacker-level-comparison engine, and no sneak-attack-trigger
//! engine exists anywhere in this codebase, so this slice grounds only the
//! bounded grant, never applying it to any actual flanking/sneak-attack
//! resolution.
//!
//! It deliberately does not implement the rage-state execution engine
//! (activation, round consumption, fatigue, stat application), weapon
//! familiarity, the Rage Power choice-list feature (a genuinely open-ended
//! choice-list feature, a new-subsystem-shaped burden), the Improved Uncanny
//! Dodge flanking-resolution/rogue-level-comparison engine, flat-footed-state
//! tracking, Armor Class computation, invisibility detection, or level-6+
//! Barbarian progression. It also preserves the accepted Barbarian
//! level-1/level-2/level-3/level-4 truth (unchanged), the Fighter negative
//! control, and the multiclass negative control.

use codex::rules_core::pilot_compute::{
    ComputationDiagnostic,
    PilotBaseChassisComputation,
    compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const BARBARIAN_LEVEL4_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level4_sd13_deterministic_input.txt"
);

const BARBARIAN_LEVEL5_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level5_sd13_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BARBARIAN_UNCANNY_DODGE_ID: &str = "class_feature.barbarian.uncanny_dodge";
const BARBARIAN_TRAP_SENSE_ID: &str = "class_feature.barbarian.trap_sense";
const BARBARIAN_IMPROVED_UNCANNY_DODGE_ID: &str =
    "class_feature.barbarian.improved_uncanny_dodge";

fn claim_blocking<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationDiagnostic {
    let diag = computation
        .diagnostics
        .iter()
        .find(|d| d.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected diagnostic id '{id}', got {:?}",
                computation.diagnostics
            )
        });
    assert!(
        diag.claim_blocking,
        "diagnostic '{id}' must be claim-blocking: {diag:?}"
    );
    diag
}

// ----- Base attack bonus at level 5 -----

#[test]
fn barbarian_level5_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(BARBARIAN_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(
        base_attack.value, 5,
        "Barbarian level 5 full-BAB progression (classlevel) must equal 5: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("level 5"),
        "barbarian base-attack explanation must name level 5: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 5 -----

#[test]
fn barbarian_level5_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(BARBARIAN_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(fortitude.value, 4, "Barbarian level 5 good Fortitude (5/2+2) must equal 4");
    assert!(
        fortitude.detail.to_lowercase().contains("good"),
        "barbarian Fortitude explanation must name it as a good save: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.barbarian.base_save.reflex");
    assert_eq!(reflex.value, 1, "Barbarian level 5 poor Reflex (5/3) must equal 1");
    assert!(
        reflex.detail.to_lowercase().contains("poor"),
        "barbarian Reflex explanation must name it as a poor save: {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.barbarian.base_save.will");
    assert_eq!(will.value, 1, "Barbarian level 5 poor Will (5/3) must equal 1");
    assert!(
        will.detail.to_lowercase().contains("poor"),
        "barbarian Will explanation must name it as a poor save: {}",
        will.detail
    );
}

// ----- Fast movement stays the flat +10 ft. value at level 5 -----

#[test]
fn barbarian_level5_fast_movement_stays_flat_ten_feet() {
    let input = load(BARBARIAN_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fast_movement = explanation(&computation, "class_chassis.barbarian.fast_movement");
    assert_eq!(
        fast_movement.value, 10,
        "Barbarian fast movement must stay +10 ft. at level 5, not a new record: {}",
        fast_movement.detail
    );
}

// ----- Rage rounds per day grows by the PF1 CRB +2-rounds-per-level rule -----

#[test]
fn barbarian_level5_rage_rounds_per_day_grows_by_two_again() {
    let input = load(BARBARIAN_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Con 16 -> modifier +3. Level 5: 4 + 3 + 2 * (5 - 1) = 15 (was 13 at level
    // 4, 11 at level 3, 9 at level 2, 7 at level 1) -- i.e. Con modifier + 12,
    // matching the PF1 CRB's own "+2 rounds per level after 1st" progression
    // through level 5.
    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 15,
        "Barbarian level 5 rage rounds per day must be 4 + Con modifier (+3) + 2 * (level - 1) \
         = 15: {}",
        rage_rounds.detail
    );
    assert!(
        rage_rounds.detail.contains("2 additional")
            || rage_rounds.detail.contains("2 * (level"),
        "rage rounds per day at level 5 must document the +2-rounds-per-level-after-1st rule: {}",
        rage_rounds.detail
    );
}

// ----- Flat rage constants are unchanged at level 5 -----

#[test]
fn barbarian_level5_flat_rage_constants_are_unchanged() {
    let input = load(BARBARIAN_LEVEL5_FIXTURE);
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
            "rage constant '{id}' must stay {expected} at level 5, not a new record"
        );
    }
}

// ----- Illiteracy-absence record still applies at level 5 -----

#[test]
fn barbarian_level5_illiteracy_absence_still_applies() {
    let input = load(BARBARIAN_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let illiteracy_absent = explanation(&computation, "class_chassis.barbarian.illiteracy_absent");
    assert_eq!(
        illiteracy_absent.value, 0,
        "the illiteracy-absent record documents a rules correction; it carries no mechanical value"
    );
}

// ----- Uncanny Dodge stays granted at level 5, not re-derived -----

#[test]
fn barbarian_level5_keeps_uncanny_dodge_grounded() {
    let input = load(BARBARIAN_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uncanny_dodge = explanation(&computation, BARBARIAN_UNCANNY_DODGE_ID);
    assert_eq!(
        uncanny_dodge.value, 0,
        "Uncanny Dodge must carry no fabricated mechanical value at level 5: {}",
        uncanny_dodge.detail
    );
    assert!(
        uncanny_dodge.detail.to_lowercase().contains("granted"),
        "uncanny dodge explanation at level 5 must state it is granted, not absent: {}",
        uncanny_dodge.detail
    );
}

// ----- Trap Sense stays granted at level 5, at the SAME +1 magnitude as level 3/4 -----

#[test]
fn barbarian_level5_trap_sense_stays_at_the_same_plus_one_magnitude() {
    let input = load(BARBARIAN_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 1,
        "Trap Sense at level 5 must stay at the PF1 Core Rulebook's barbarian-level/3 formula \
         value (+1, unchanged since level 3 -- the bonus does not rise again until level 6): {}",
        trap_sense.detail
    );
    assert!(
        trap_sense.detail.contains("Trap Sense"),
        "trap sense explanation must name the Trap Sense class feature: {}",
        trap_sense.detail
    );
    assert!(
        trap_sense.detail.to_lowercase().contains("granted"),
        "trap sense explanation at level 5 must state it is granted, not absent: {}",
        trap_sense.detail
    );
}

// ----- Improved Uncanny Dodge is newly granted at level 5 -----

#[test]
fn barbarian_level5_grounds_improved_uncanny_dodge_grant_only() {
    let input = load(BARBARIAN_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let improved_uncanny_dodge = explanation(&computation, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID);
    assert_eq!(
        improved_uncanny_dodge.value, 0,
        "Improved Uncanny Dodge must carry no fabricated mechanical value: {}",
        improved_uncanny_dodge.detail
    );
    assert!(
        improved_uncanny_dodge.detail.to_lowercase().contains("granted"),
        "improved uncanny dodge explanation at level 5 must state it is granted: {}",
        improved_uncanny_dodge.detail
    );
    assert!(
        improved_uncanny_dodge.detail.to_lowercase().contains("flank"),
        "improved uncanny dodge explanation must name the no-longer-flanked rule: {}",
        improved_uncanny_dodge.detail
    );
}

#[test]
fn barbarian_level4_does_not_yet_grant_improved_uncanny_dodge() {
    let input = load(BARBARIAN_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let improved_uncanny_dodge = explanation(&computation, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID);
    assert_eq!(
        improved_uncanny_dodge.value, 0,
        "Improved Uncanny Dodge absence record carries no mechanical value either: {}",
        improved_uncanny_dodge.detail
    );
    assert!(
        !improved_uncanny_dodge.detail.to_lowercase().contains("granted"),
        "improved uncanny dodge explanation at level 4 must state it is absent, not granted: {}",
        improved_uncanny_dodge.detail
    );
}

// ----- Still blocked: rage-state execution engine and generic diagnostics -----

#[test]
fn barbarian_level5_stays_blocked_on_rage_execution() {
    let input = load(BARBARIAN_LEVEL5_FIXTURE);
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
                "level-5 Barbarian must ground an honest not-raging record when no rage \
                 posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
    assert_eq!(
        computation.base_attack_bonus, 5,
        "barbarian is now recognized by table_class_id; level 5 full BAB is +5"
    );
}

// ----- Negative control: level 6 was later widened into the supported tranche -----

#[test]
fn barbarian_level_6_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 6 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_barbarian_level6_progression.rs) widened the level-range gate
    // to level 6 (mirroring the Rogue level-range gate idiom) and grounded Trap
    // Sense's own rise to +2; this negative control is superseded, not
    // violated — pin the new truth here too so this file stays internally
    // consistent. The frontier this file's own slice actually drew is now
    // level 7, covered by `barbarian_level_7_is_not_promoted_by_this_slice` in
    // `tests/sd13_barbarian_level6_progression.rs`.
    let level_6 = BARBARIAN_LEVEL5_FIXTURE.replace("class:barbarian:5", "class:barbarian:6");
    let input = load(&level_6);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, "class_chassis.barbarian.base_attack_bonus"),
        "level-6 Barbarian is supported since the SD13-E5 level-6 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, BARBARIAN_UNCANNY_DODGE_ID),
        "level-6 Barbarian must keep the Uncanny Dodge explanation grounded at level 2"
    );
    assert!(
        has_explanation(&computation, BARBARIAN_TRAP_SENSE_ID),
        "level-6 Barbarian must keep the Trap Sense explanation grounded at level 3"
    );
    assert!(
        has_explanation(&computation, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID),
        "level-6 Barbarian must keep the Improved Uncanny Dodge explanation grounded at level 5"
    );
}

// ----- Negative control: the barbarian path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_barbarian_level5_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")
                || e.id == BARBARIAN_UNCANNY_DODGE_ID
                || e.id == BARBARIAN_TRAP_SENSE_ID
                || e.id == BARBARIAN_IMPROVED_UNCANNY_DODGE_ID),
        "the Fighter chassis must not surface any barbarian-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Barbarian is not promoted -----

#[test]
fn multiclass_barbarian_level5_is_not_promoted_by_this_slice() {
    let multiclass = BARBARIAN_LEVEL5_FIXTURE.replace(
        "class_level=class:barbarian:5",
        "class_level=class:barbarian:5\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")
                || e.id == BARBARIAN_UNCANNY_DODGE_ID
                || e.id == BARBARIAN_TRAP_SENSE_ID
                || e.id == BARBARIAN_IMPROVED_UNCANNY_DODGE_ID),
        "multiclass Barbarian must not gain any bounded barbarian chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Barbarian must stay claim-blocked in this slice"
    );
}

// ----- Barbarian level 1/level 2/level 3/level 4 stays unchanged -----

#[test]
fn barbarian_level4_truth_is_unchanged_by_the_level5_widening() {
    let input = load(BARBARIAN_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(base_attack.value, 4, "Barbarian level 4 full BAB must stay +4");

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 13,
        "Barbarian level 4 rage rounds per day must stay 4 + Con modifier (+3) + 2 * 3 = 13"
    );

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(trap_sense.value, 1, "Barbarian level 4 Trap Sense must stay +1");
}

// ----- Control plane: the matrix note names the level-5 widening -----

#[test]
fn matrix_barbarian_row_names_level_5_widening() {
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
            .contains("sd13_barbarian_level5_progression"),
        "barbarian row must cite the live SD13-E5 level-5 proof surface: {}",
        barbarian.grounding_ref
    );
    let note = barbarian.blocker_or_lossiness_note;
    assert!(
        note.contains("level 5") || note.contains("level-5"),
        "barbarian partial note must name the level-5 widening: {note}"
    );
    assert!(
        note.to_lowercase().contains("improved uncanny dodge"),
        "barbarian partial note must name the newly grounded Improved Uncanny Dodge grant: {note}"
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
