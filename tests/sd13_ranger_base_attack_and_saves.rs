//! SD13-E5 Ranger level-1 base attack bonus and base save progression proof.
//!
//! Grounds the one foundational martial pillar every other class row in this matrix
//! (Fighter, Barbarian, Monk, Rogue, Paladin, and by now Druid/Cleric/Bard/Sorcerer/
//! Wizard) already has and Ranger has never had: the base attack bonus and base save
//! progression at Ranger level 1. Both formulas are verified against the PF1 Core
//! Rulebook Ranger class table (d20pfsrd and the legacy Paizo PRD mirror, reading the
//! raw level 1-5 table rows directly and cross-checking the level 4/5 base-attack-bonus
//! values to disambiguate the fraction, since level 1 alone floors a 3/4 progression to
//! +0 while a full progression already shows +1) before writing any code:
//! - base attack bonus: FULL BAB, the same formula shape as Fighter/Barbarian/Paladin
//!   (`classlevel`), +1 at level 1 (level 4: +4, level 5: +5 -- confirms full, not 3/4,
//!   which would show +3/+3 at those levels);
//! - base save progression: good Fortitude, good Reflex, poor Will
//!   (`classlevel/2+2` for the two good saves, `classlevel/3` for the one poor save),
//!   +2 / +2 / +0 at level 1 (level 4: +4/+4/+1, confirming the same formula shape).
//!
//! Both are grounded as flat, standalone `ComputationExplanation` records mirroring the
//! exact "standalone, not wired into the integrated `PilotBaseChassisComputation`"
//! idiom already used for Barbarian's/Monk's/Druid's/Cleric's/Bard's/Sorcerer's/
//! Wizard's own base-attack/base-save grounding: neither record is wired into
//! `PilotBaseChassisComputation.base_attack_bonus`, `compute_total_saves`, or
//! `compute_combat_baseline`, so the integrated pilot surface still reports a blocked
//! posture on this input.
//!
//! This slice does NOT touch the combat-style pillar's actual bonus-feat grant, the
//! ranger spell burden, or Ranger level 2+ -- those stay named-but-unproven exactly as
//! before. Track, Favored Enemy, and the combat-style level-gate absence record
//! (grounded by earlier SD13-E3/E5 slices) are unaffected.
//!
//! **Superseded (v0.6 alpha swarm, risks item 8):** `table_class_id` was widened to
//! recognize Ranger via the shared table-driven `compute_generic_table_chassis`
//! dispatch, giving Ranger its own real, integrated `class_chassis.*` computation
//! (not just the standalone `class_chassis.ranger.*` pillar records this file's
//! original slice grounded). This makes the paragraph above's "standalone, not wired
//! into `PilotBaseChassisComputation`" claim stale: Ranger's base-attack/base-save ARE
//! now wired into the integrated `class_chassis.base_attack_bonus` /
//! `class_chassis.base_save.*` explanations, mirroring the identical Rogue-widening
//! flip in `sd13_rogue_level1_chassis_baseline.rs`. See the test marked "(v0.6 swarm
//! update)" below for the exact current truth.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{SupportState, seeded_current_truth};
mod common;
use common::{load, explanation, has_explanation};

const RANGER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt");

const BASE_ATTACK_ID: &str = "class_chassis.ranger.base_attack_bonus";
const BASE_SAVE_FORTITUDE_ID: &str = "class_chassis.ranger.base_save.fortitude";
const BASE_SAVE_REFLEX_ID: &str = "class_chassis.ranger.base_save.reflex";
const BASE_SAVE_WILL_ID: &str = "class_chassis.ranger.base_save.will";

// ----- Grounded: the base attack bonus pillar -----

#[test]
fn ranger_level1_grounds_base_attack_bonus() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Ranger class table: full BAB, same formula shape as
    // Fighter/Barbarian/Paladin (classlevel). At level 1: 1.
    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 1,
        "Ranger level 1 base attack bonus (full BAB) must be +1: {base_attack:?}"
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

// ----- Grounded: the base save progression pillar -----

#[test]
fn ranger_level1_grounds_base_save_progression() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Ranger class table: good Fortitude, good Reflex, poor Will.
    // At level 1: Fortitude/Reflex = classlevel/2+2 = 2, Will = classlevel/3 = 0.
    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 2, "Ranger level 1 good Fortitude save must be +2");
    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 2, "Ranger level 1 good Reflex save must be +2");
    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 0, "Ranger level 1 poor Will save must be +0");

    for (record, label) in [
        (fortitude, "Fortitude"),
        (reflex, "Reflex"),
        (will, "Will"),
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

// ----- (v0.6 swarm update, risks item 8): the standalone records now ALSO have a genuinely wired integrated total -----

#[test]
fn ranger_level1_base_attack_and_saves_are_now_wired_into_integrated_totals() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The grounded standalone records still exist...
    assert!(has_explanation(&computation, BASE_ATTACK_ID));
    assert!(has_explanation(&computation, BASE_SAVE_FORTITUDE_ID));
    assert!(has_explanation(&computation, BASE_SAVE_REFLEX_ID));
    assert!(has_explanation(&computation, BASE_SAVE_WILL_ID));

    // ...and, since `table_class_id` was widened to recognize Ranger via the
    // shared table-driven `compute_generic_table_chassis` dispatch, the
    // integrated chassis compute path now genuinely computes Ranger too: a
    // real, non-fabricated base_attack_bonus (Ranger's full-BAB progression,
    // 1 at level 1), and the generic `class_chassis.base_attack_bonus`
    // explanation now legitimately appears alongside the standalone
    // `class_chassis.ranger.*` records above. Mirrors the identical Rogue
    // widening flip in `sd13_rogue_level1_chassis_baseline.rs`.
    assert_eq!(
        computation.base_attack_bonus, 1,
        "ranger level 1's real full-BAB progression (classlevel) is now genuinely integrated"
    );
    assert!(
        has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "ranger base-attack bonus is now a genuinely integrated chassis explanation, not a \
         standalone-only record"
    );
}

// ----- Existing Ranger grounded pillars and blockers are unaffected -----

#[test]
fn ranger_level1_base_attack_and_saves_do_not_disturb_existing_pillars_or_blockers() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Track, the favored-enemy flat surface, and the combat-style level-gate absence
    // stay grounded exactly as before.
    assert!(has_explanation(&computation, "class_chassis.ranger.track"));
    assert!(has_explanation(
        &computation,
        "class_chassis.ranger.favored_enemy_skill_bonus"
    ));
    assert!(has_explanation(
        &computation,
        "class_chassis.ranger.favored_enemy_attack_damage_bonus"
    ));
    assert!(has_explanation(
        &computation,
        "class_chassis.ranger.level_gate.combat_style"
    ));

    // The former hybrid class-feature blocker (`class_feature.hybrid.ranger.
    // unsupported`) is retired: it flatly claimed favored enemy / combat style /
    // tracking were unimplemented, which the grounded records asserted immediately
    // above (Track, the Favored Enemy flat surface, the combat-style level-gate
    // absence) contradict. See `tests/hybrid_diagnostic_grounded_contradiction.rs`.
    // Both blanket hybrid burden blockers are now retired; this slice grounds no
    // combat-style feat mechanics and no spell math, but neither absence is a
    // claim-blocking gap. (The later-spell one went 2026-07-28: Rangers have no
    // `CAST:` row in `cr_classes.lst` before class level 4, so a level-1 Ranger
    // having no spell posture is the correct computed answer. Real spell-posture
    // violations are still claim-blocked by
    // `class_spell.ranger.partial_caster.unsupported`. See
    // `tests/v06_hybrid_level1_no_spellcasting_is_computed.rs`.)
    for retired in [
        "class_feature.hybrid.ranger.unsupported",
        "class_spell.hybrid.ranger.unsupported",
    ] {
        assert!(
            !computation.diagnostics.iter().any(|d| d.id == retired),
            "the retired hybrid blocker '{retired}' must not reappear: {:?}",
            computation.diagnostics
        );
    }
}

// ----- Negative control: Ranger level 2 was later widened into the supported tranche -----

#[test]
fn ranger_level_2_was_later_widened_into_the_supported_tranche() {
    // At the time this file was written, Ranger level 2+ progression was out of scope
    // (the level-1-only gate `is_single_class_ranger_level1` did not recognize it). A
    // later SD13-E5 slice (`tests/sd13_ranger_level2_progression.rs`) widened the gate
    // to a level-range gate (`supported_ranger_level`, 1..=2) and extended this exact
    // base-attack/base-save grounding to level 2 via the same formulas. This control
    // now pins the widened truth instead of the stale level-1-only absence, and a
    // level-3 negative control takes over the "still out of scope" role.
    let level_2 = RANGER_FIXTURE.replace("class:ranger:1", "class:ranger:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 2,
        "level-2 Ranger base attack bonus (full BAB) must be +2: {base_attack:?}"
    );
    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 3, "level-2 Ranger good Fortitude save must be +3");
}

// ----- Negative control: Ranger level 3 was later widened into the supported tranche -----

#[test]
fn ranger_level_3_was_later_widened_into_the_supported_tranche() {
    // At the time this file was written, Ranger level 3+ progression was out of
    // scope (the level-range gate `supported_ranger_level` only recognized 1..=2).
    // A later SD13-E5 slice (`tests/sd13_ranger_level3_progression.rs`) widened the
    // gate to 1..=3 and extended this exact base-attack/base-save grounding to
    // level 3 via the same formulas. This control now pins the widened truth
    // instead of the stale level-2-only absence, and a level-4 negative control
    // takes over the "still out of scope" role.
    let level_3 = RANGER_FIXTURE.replace("class:ranger:1", "class:ranger:3");
    let input = load(&level_3);
    let computation = compute_pilot_base_chassis(&input);
    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 3,
        "level-3 Ranger base attack bonus (full BAB) must be +3: {base_attack:?}"
    );
    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 3, "level-3 Ranger good Fortitude save must be +3");
}

// ----- Negative control: level 4 was later widened into the supported tranche -----

#[test]
fn ranger_level_4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_ranger_level4_progression.rs) widened the level-range gate to
    // level 4 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk level-range
    // gate idiom) and grounded Hunter's Bond; this negative control is
    // superseded, not violated — pin the new truth here too so this file stays
    // internally consistent.
    let level_4 = RANGER_FIXTURE.replace("class:ranger:1", "class:ranger:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, BASE_ATTACK_ID),
        "level-4 Ranger is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, BASE_SAVE_FORTITUDE_ID),
        "level-4 Ranger keeps the base-save grounding since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the grounding must not leak onto other classes -----

#[test]
fn fighter_and_paladin_do_not_gain_ranger_base_attack_or_save_grounding() {
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(!has_explanation(&fighter_computation, BASE_ATTACK_ID));
    assert!(!has_explanation(&fighter_computation, BASE_SAVE_FORTITUDE_ID));

    let paladin_fixture = RANGER_FIXTURE.replace("class:ranger:1", "class:paladin:1");
    let paladin = load(&paladin_fixture);
    let paladin_computation = compute_pilot_base_chassis(&paladin);
    assert!(!has_explanation(&paladin_computation, BASE_ATTACK_ID));
    assert!(!has_explanation(&paladin_computation, BASE_SAVE_FORTITUDE_ID));
}

// ----- Control plane: the matrix row's note names the newly grounded pillar -----

#[test]
fn matrix_ranger_row_note_names_base_attack_and_base_save_as_grounded() {
    let matrix = seeded_current_truth();
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger row must exist");

    assert_eq!(ranger.support_state, SupportState::Supported);
    for token in ["base attack", "base save", "standalone"] {
        assert!(
            ranger.blocker_or_lossiness_note.contains(token),
            "ranger blocker note must name '{token}' now that base attack/base save are \
             grounded: {}",
            ranger.blocker_or_lossiness_note
        );
    }
    // The still-unproven burdens stay named.
    for token in ["combat-style", "spell"] {
        assert!(
            ranger.blocker_or_lossiness_note.contains(token),
            "ranger blocker note must still name the unproven '{token}' burden: {}",
            ranger.blocker_or_lossiness_note
        );
    }
}
