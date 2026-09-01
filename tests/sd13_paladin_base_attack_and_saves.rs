//! SD13-E5 Paladin level-1/level-2 base attack bonus and base save progression proof.
//!
//! Grounds the one foundational martial pillar every other class row in this matrix
//! (Fighter, Barbarian, Monk, Rogue, Druid, Cleric, Bard, Sorcerer, Wizard, and by the
//! immediately preceding cycle, Ranger) already has and Paladin has never had, despite
//! Paladin already supporting a level-range gate (1..=2, widened by the SD13-E5 lay on
//! hands / divine grace cycle) unlike Ranger's level-1-only gate. Both formulas are
//! verified against the PF1 Core Rulebook Paladin class table (d20pfsrd and the legacy
//! Paizo PRD mirror, reading the raw level 1-5 table rows directly) before writing any
//! code:
//! - base attack bonus: FULL BAB, the same formula shape as Fighter/Barbarian/Ranger
//!   (`classlevel`), +1 at level 1 and +2 at level 2 (level 4: +4, level 5: +5 --
//!   confirms full, not 3/4, which would show +3/+3 at those levels);
//! - base save progression: good Fortitude, good Will, poor Reflex
//!   (`classlevel/2+2` for the two good saves, `classlevel/3` for the one poor save),
//!   +2 / +0 / +2 at level 1 and +3 / +0 / +3 at level 2 -- NOT the same save shape as
//!   Ranger (good Fortitude/Reflex, poor Will), verified independently rather than
//!   assumed from Ranger's superficially similar hybrid-class chassis.
//!
//! Both are grounded as flat, standalone `ComputationExplanation` records mirroring the
//! exact "standalone, not wired into the integrated `PilotBaseChassisComputation`"
//! idiom already used for Barbarian's/Monk's/Druid's/Cleric's/Bard's/Sorcerer's/
//! Wizard's/Ranger's own base-attack/base-save grounding: neither record is wired into
//! `PilotBaseChassisComputation.base_attack_bonus`, `compute_total_saves`, or
//! `compute_combat_baseline`, so the integrated pilot surface still reports a blocked
//! posture on this input.
//!
//! Grounded inside `explain_paladin_level1_chassis_and_spell_burden_separation` (the
//! Paladin-only decomposition function), NOT the shared `explain_hybrid_level1_chassis`
//! (which also serves Ranger). This slice does NOT touch the F6 hybrid chassis pair,
//! the partial-caster spell burden, mercy, or Paladin level 3+ -- those stay
//! named-but-unproven exactly as before.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{SupportState, seeded_current_truth};
mod common;
use common::{load, explanation, has_explanation};

const PALADIN_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt");
const PALADIN_LEVEL2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level2_sd13_deterministic_input.txt");

const BASE_ATTACK_ID: &str = "class_chassis.paladin.base_attack_bonus";
const BASE_SAVE_FORTITUDE_ID: &str = "class_chassis.paladin.base_save.fortitude";
const BASE_SAVE_REFLEX_ID: &str = "class_chassis.paladin.base_save.reflex";
const BASE_SAVE_WILL_ID: &str = "class_chassis.paladin.base_save.will";

// ----- Grounded: the base attack bonus pillar, at both supported levels -----

#[test]
fn paladin_level1_grounds_base_attack_bonus() {
    let input = load(PALADIN_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Paladin class table: full BAB, same formula shape as
    // Fighter/Barbarian/Ranger (classlevel). At level 1: 1.
    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 1,
        "Paladin level 1 base attack bonus (full BAB) must be +1: {base_attack:?}"
    );
    assert!(
        base_attack.detail.contains("full") || base_attack.detail.contains("classlevel = 1"),
        "base attack detail must cite the full-BAB formula: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("standalone"),
        "base attack detail must state it is a standalone record: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("base_attack_bonus")
            || base_attack.detail.contains("compute_combat_baseline"),
        "base attack detail must name the integrated field/seam it is NOT wired into: {}",
        base_attack.detail
    );
}

#[test]
fn paladin_level2_grounds_base_attack_bonus() {
    let input = load(PALADIN_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Paladin class table: full BAB. At level 2: 2.
    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 2,
        "Paladin level 2 base attack bonus (full BAB) must be +2: {base_attack:?}"
    );
}

// ----- Grounded: the base save progression pillar, at both supported levels -----

#[test]
fn paladin_level1_grounds_base_save_progression() {
    let input = load(PALADIN_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Paladin class table: good Fortitude, good Will, poor Reflex.
    // At level 1: Fortitude/Will = classlevel/2+2 = 2, Reflex = classlevel/3 = 0.
    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 2, "Paladin level 1 good Fortitude save must be +2");
    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 2, "Paladin level 1 good Will save must be +2");
    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 0, "Paladin level 1 poor Reflex save must be +0");

    for (record, label) in [
        (fortitude, "Fortitude"),
        (will, "Will"),
        (reflex, "Reflex"),
    ] {
        assert!(
            record.detail.contains("standalone"),
            "{label} base-save detail must state it is a standalone record: {}",
            record.detail
        );
        assert!(
            record.detail.contains("compute_total_saves"),
            "{label} base-save detail must name compute_total_saves as the seam it is NOT wired \
             into: {}",
            record.detail
        );
    }
}

#[test]
fn paladin_level2_grounds_base_save_progression() {
    let input = load(PALADIN_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Paladin class table: good Fortitude, good Will, poor Reflex.
    // At level 2: Fortitude/Will = classlevel/2+2 = 3, Reflex = classlevel/3 = 0.
    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 3, "Paladin level 2 good Fortitude save must be +3");
    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 3, "Paladin level 2 good Will save must be +3");
    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 0, "Paladin level 2 poor Reflex save must be +0");
}

// ----- The grounded records are standalone: not wired into any integrated total -----

#[test]
fn paladin_base_attack_and_saves_are_not_wired_into_integrated_totals() {
    let input = load(PALADIN_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The grounded standalone records exist...
    assert!(has_explanation(&computation, BASE_ATTACK_ID));
    assert!(has_explanation(&computation, BASE_SAVE_FORTITUDE_ID));
    assert!(has_explanation(&computation, BASE_SAVE_REFLEX_ID));
    assert!(has_explanation(&computation, BASE_SAVE_WILL_ID));

    // (v0.6 alpha swarm, risks item 8, third slice, 2026-07-25) Paladin's base
    // attack bonus is now genuinely integrated via the table-driven
    // `compute_generic_table_chassis` dispatch (`table_class_id` widened to
    // recognize Paladin) -- the value 1 is Paladin's real full-BAB
    // progression at level 1, not a fabricated absence, and the integrated
    // explanation now legitimately exists alongside the standalone record
    // asserted above.
    assert_eq!(
        computation.base_attack_bonus, 1,
        "paladin level 1's real full-BAB progression (classlevel) is now genuinely integrated"
    );
    assert!(
        has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "paladin base-attack bonus is now a genuinely integrated chassis explanation, not a \
         standalone-only record"
    );
}

// ----- Existing Paladin grounded pillars and blockers are unaffected -----

#[test]
fn paladin_level1_base_attack_and_saves_do_not_disturb_existing_pillars_or_blockers() {
    let input = load(PALADIN_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Smite Evil, the level-1 lay-on-hands/divine-grace level-gate absences, mercy,
    // and the partial-caster effective-caster-level gate all stay grounded exactly
    // as before.
    assert!(has_explanation(
        &computation,
        "class_chassis.paladin.smite_evil_uses_per_day"
    ));
    assert!(has_explanation(
        &computation,
        "class_chassis.paladin.smite_evil_attack_bonus"
    ));
    assert!(has_explanation(
        &computation,
        "class_chassis.paladin.smite_evil_damage_bonus"
    ));
    assert!(has_explanation(
        &computation,
        "class_chassis.paladin.level_gate.lay_on_hands"
    ));
    assert!(has_explanation(
        &computation,
        "class_chassis.paladin.level_gate.divine_grace"
    ));
    assert!(has_explanation(
        &computation,
        "class_chassis.paladin.level_gate.mercy"
    ));
    assert!(has_explanation(
        &computation,
        "class_chassis.paladin.partial_caster.effective_caster_level"
    ));

    // The former hybrid class-feature blocker (`class_feature.hybrid.paladin.
    // unsupported`) is retired: it flatly claimed Smite Evil / lay on hands /
    // divine grace / mercy were unimplemented, which the grounded records
    // asserted immediately above (Smite Evil's real numbers, the lay on hands /
    // divine grace / mercy level-gate absences) contradict. See
    // `tests/hybrid_diagnostic_grounded_contradiction.rs`. Only the partial-caster
    // spell burden still fires; this slice grounds no spell math.
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.hybrid.paladin.unsupported"),
        "the retired hybrid class-feature blocker must not reappear: {:?}",
        computation.diagnostics
    );
    // (v0.6 alpha swarm, risks item 8, third slice, 2026-07-25)
    // `class_spell.paladin.partial_caster.unsupported` is no longer
    // unconditional: at level 1 no paladin spell level is accessible at all
    // (spells begin at level 4), so a bare fixture with zero prepared spells
    // has a genuinely valid (empty) posture, and the blocker correctly does
    // not fire.
    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_spell.paladin.partial_caster.unsupported")
    {
        Some(spell_blocker) => assert!(spell_blocker.claim_blocking),
        None => {
            let daily_prep = explanation(&computation, "class_spell.paladin.daily_preparation");
            assert_eq!(daily_prep.value, 0, "no spells are fabricated at paladin level 1");
        }
    }
}

#[test]
fn paladin_level2_base_attack_and_saves_do_not_disturb_existing_pillars_or_blockers() {
    let input = load(PALADIN_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Smite Evil, lay on hands, divine grace, mercy, and the partial-caster
    // effective-caster-level gate all stay grounded exactly as before.
    assert!(has_explanation(
        &computation,
        "class_chassis.paladin.smite_evil_uses_per_day"
    ));
    assert!(has_explanation(
        &computation,
        "class_chassis.paladin.lay_on_hands_uses_per_day"
    ));
    assert!(has_explanation(
        &computation,
        "class_chassis.paladin.divine_grace_save_bonus"
    ));
    assert!(has_explanation(
        &computation,
        "class_chassis.paladin.level_gate.mercy"
    ));
    assert!(has_explanation(
        &computation,
        "class_chassis.paladin.partial_caster.effective_caster_level"
    ));

    // (v0.6 alpha swarm, risks item 8, third slice, 2026-07-25)
    // `class_spell.paladin.partial_caster.unsupported` is no longer
    // unconditional: at level 2 no paladin spell level is accessible yet
    // (spells begin at level 4), so a bare fixture with zero prepared spells
    // has a genuinely valid (empty) posture, and the blocker correctly does
    // not fire. (The hybrid class-feature/spell blockers from
    // `explain_hybrid_level1_chassis` are gated to level 1 only, so they do
    // not fire here -- unaffected by this slice, unchanged from before.)
    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_spell.paladin.partial_caster.unsupported")
    {
        Some(spell_blocker) => assert!(spell_blocker.claim_blocking),
        None => {
            let daily_prep = explanation(&computation, "class_spell.paladin.daily_preparation");
            assert_eq!(daily_prep.value, 0, "no spells are fabricated at paladin level 2");
        }
    }
}

// ----- Negative control (retired): Paladin level 3 was later widened into the -----
// ----- supported tranche; see tests/sd13_paladin_level3_mercy.rs               -----

#[test]
fn paladin_level_3_was_later_widened_into_the_supported_tranche() {
    // A subsequent SD13-E5 slice (tests/sd13_paladin_level3_mercy.rs) widened
    // `supported_paladin_level` from 1..=2 to 1..=3, so level 3 now gains this
    // slice's base-attack/base-save grounding via the same formulas. This control
    // now asserts the widened truth rather than the original (now-stale) "level 3
    // stays unrecognized" claim; the equivalent negative control for the current
    // boundary (level 4) lives in tests/sd13_paladin_level3_mercy.rs's
    // `paladin_level_4_is_not_promoted_by_this_slice`.
    let level_3 = PALADIN_LEVEL2_FIXTURE.replace("class:paladin:2", "class:paladin:3");
    let input = load(&level_3);
    let computation = compute_pilot_base_chassis(&input);
    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 3,
        "level-3 Paladin must now gain the widened base-attack grounding (classlevel = 3): {:?}",
        base_attack
    );
    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(
        fortitude.value, 3,
        "level-3 Paladin must now gain the widened base-save grounding (classlevel/2+2 = 3): {:?}",
        fortitude
    );
}

// ----- Negative control: the grounding must not leak onto other classes -----

#[test]
fn fighter_and_ranger_do_not_gain_paladin_base_attack_or_save_grounding() {
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(!has_explanation(&fighter_computation, BASE_ATTACK_ID));
    assert!(!has_explanation(&fighter_computation, BASE_SAVE_FORTITUDE_ID));

    let ranger_fixture = PALADIN_LEVEL1_FIXTURE.replace("class:paladin:1", "class:ranger:1");
    let ranger = load(&ranger_fixture);
    let ranger_computation = compute_pilot_base_chassis(&ranger);
    assert!(!has_explanation(&ranger_computation, BASE_ATTACK_ID));
    assert!(!has_explanation(&ranger_computation, BASE_SAVE_FORTITUDE_ID));
}

// ----- Control plane: the matrix row's note names the newly grounded pillar -----

#[test]
fn matrix_paladin_row_note_names_base_attack_and_base_save_as_grounded() {
    let matrix = seeded_current_truth();
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin row must exist");

    assert_eq!(paladin.support_state, SupportState::Supported);
    for token in ["base attack", "base save", "standalone"] {
        assert!(
            paladin.blocker_or_lossiness_note.contains(token),
            "paladin blocker note must name '{token}' now that base attack/base save are \
             grounded: {}",
            paladin.blocker_or_lossiness_note
        );
    }
    // The still-unproven burdens stay named.
    for token in ["hybrid", "spell"] {
        assert!(
            paladin.blocker_or_lossiness_note.contains(token),
            "paladin blocker note must still name the unproven '{token}' burden: {}",
            paladin.blocker_or_lossiness_note
        );
    }
}
