//! SD13-E5 Sorcerer level-10 progression grounding proof.
//!
//! Widens the accepted Sorcerer level-1..level-9 spontaneous spell-burden
//! baseline (most recently `tests/sd13_sorcerer_level9_progression.rs`) to
//! Sorcerer level 10 — the tranche's declared ceiling — mirroring the
//! sibling-class level-range-gate idiom (`supported_sorcerer_level` is
//! generalized from `1..=9` to `1..=10` via
//! `MAX_SUPPORTED_SORCERER_LEVEL = 10`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Sorcerer class table) were read directly
//! before writing any code or test:
//!
//! - level 10 base attack bonus is +5 (`10 / 2 = 5`, genuinely risen from
//!   +4 at level 9) and base saves are +3 Fortitude and +3 Reflex (both
//!   poor, `10 / 3 = 3`, numerically unchanged from level 9,
//!   integer-division coincidences) and +7 Will (good, `10 / 2 + 2 = 7`,
//!   genuinely risen from +6) — confirmed by the same formulas already
//!   grounded at levels 1-9, not re-derived.
//! - the bloodline choice recognition and the Arcane bloodline's class-skill
//!   choice recognition are not level-gated, so both still fire at level 10
//!   for the same fixture selections.
//! - the PF1 Core Rulebook Sorcerer class table's level-10 "Special" column
//!   is genuinely BLANK (verified independently against both primary
//!   sources, checked rather than assumed away) — like levels 2, 4, 6, and
//!   8, and UNLIKE the level-9 "Bloodline power, bloodline spell" row —
//!   so no new class feature is gained at 10th level. The first 5th-level
//!   spell slots arrive at 10th, but spells per day belong to the
//!   spontaneous spell burden that stays named-but-unproven, so no new
//!   pillar record is grounded at level 10.
//!
//! It deliberately does not touch Arcane Bond, bloodline arcana, the
//! bloodline powers/spells/feats, bonus spells at 3rd+, or the spontaneous
//! spell burden (all stay named-but-unproven, unchanged from levels 1-9),
//! and it does not ground Sorcerer level 11+. It also preserves the accepted
//! Sorcerer level-1..level-9 truth (unchanged), the Fighter negative
//! control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const SORCERER_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level9_sd13_deterministic_input.txt");

const SORCERER_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 9 -----

#[test]
fn sorcerer_level10_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(SORCERER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(
        base_attack.value, 5,
        "Sorcerer level 10 1/2-BAB progression (10 / 2) must equal 5, genuinely risen from 4 \
         at level 10: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 9 (good Will only, poor Fortitude/Reflex) -----

#[test]
fn sorcerer_level10_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(SORCERER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.sorcerer.base_save.fortitude");
    assert_eq!(
        fortitude.value, 3,
        "Sorcerer level 10 poor Fortitude (10/3) must equal 3 — unchanged from level 9, an \
         integer-division coincidence"
    );

    let reflex = explanation(&computation, "class_chassis.sorcerer.base_save.reflex");
    assert_eq!(reflex.value, 3, "Sorcerer level 10 poor Reflex (10/3) must equal 3");

    let will = explanation(&computation, "class_chassis.sorcerer.base_save.will");
    assert_eq!(
        will.value, 7,
        "Sorcerer level 10 good Will (10/2+2) must equal 7, genuinely risen from 6 at level 9"
    );
}

// ----- Bloodline choice recognition still fires at level 9 -----

#[test]
fn sorcerer_level10_still_recognizes_the_bloodline_choice() {
    let input = load(SORCERER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.sorcerer.bloodline_choice");
    assert_eq!(
        choice.value, 0,
        "bloodline choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Arcane"),
        "bloodline choice recognition must still name the Arcane bloodline at level 10: {}",
        choice.detail
    );
}

// ----- Arcane bloodline class-skill choice recognition still fires at level 9 -----

#[test]
fn sorcerer_level10_still_recognizes_the_bloodline_class_skill_choice() {
    let input = load(SORCERER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(
        &computation,
        "class_chassis.sorcerer.bloodline_class_skill_choice",
    );
    assert_eq!(
        choice.value, 0,
        "class-skill choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Knowledge (arcana)"),
        "class-skill choice recognition must still name the selected Knowledge skill at level \
         10: {}",
        choice.detail
    );
}

// ----- The two existing burden diagnostics still fire at level 9 -----

#[test]
fn sorcerer_level10_still_claim_blocks_arcane_bond_and_spontaneous_spell_burdens() {
    let input = load(SORCERER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"
            && d.claim_blocking),
        "level-10 Sorcerer must still claim-block on the Arcane Bond / bloodline \
         progression burden (which also names the bloodline burdens): {:?}",
        computation.diagnostics
    );
    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_spell.sorcerer.spontaneous.unsupported")
    {
        Some(spell_blocker) => assert!(
            spell_blocker.claim_blocking,
            "if the spell blocker fires at all, it must be claim-blocking"
        ),
        None => {
            // (v0.6 alpha swarm, risks item 8) class_spell.sorcerer.spontaneous.unsupported
            // is no longer unconditional -- it's a real, conditional validation of
            // AcquisitionMode::Known selections. This fixture predates spells_selected
            // (zero known spells), so the posture is genuinely valid and the blocker
            // correctly does not fire -- confirmed via the real known-spell count being
            // honestly 0, not fabricated.
            let known_count = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_spell.sorcerer.known_spells")
                .map(|e| e.value)
                .unwrap_or(-1);
            assert_eq!(
                known_count, 0,
                "no spells are fabricated merely because the spell blocker stopped firing: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- The chassis recognition record is still present at level 9 -----

#[test]
fn sorcerer_level10_still_recognizes_the_spell_bearing_baseline() {
    let input = load(SORCERER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.spell_baseline.sorcerer"),
        "level-10 Sorcerer must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
}

// ----- No bloodline-power/spell record is fabricated at level 9 -----

#[test]
fn sorcerer_level10_does_not_fabricate_the_ninth_level_bloodline_entries() {
    let input = load(SORCERER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // SD-34 bucket-B batch cycle: found stale, unrelated to this cycle's own three
    // mechanisms -- `push_generic_pool_group_selection_magnitude`'s pre-existing (SD-32
    // T12 Epic 8 row 18 cycle 5) generic bloodline pass legitimately emits one
    // `class_feature.sorcerer.bloodline.generic.<bloodline>.<power_slug>.<pcgen_var>` id per
    // real corpus power the character's selected bloodline carries (covering the 51
    // non-hand-modelled bloodlines "purely additive alongside the Arcane/Draconic
    // hand-modelled branches" -- this module's own doc comment at the push site), including
    // real New Arcana/bloodline-power-shaped var names for the Arcane bloodline this
    // fixture selects -- a REAL, citation-backed, per-bloodline-power roster fact (proven by
    // its own dedicated `pool_group_closure_census_across_all_six_pools` regression), never a
    // fabricated per-character MAGNITUDE (this generic pass never computes one; see its own
    // module doc). This negative control's substring checks were written before that generic
    // pass existed and were never updated to admit its id shape -- an additive, scoped
    // prefix carve-out fixes the stale gate without weakening it for anything outside that
    // one namespace.
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| !e.id.starts_with("class_feature.sorcerer.bloodline.generic.")
                && (e.id.to_lowercase().contains("new_arcana")
                    || e.id.to_lowercase().contains("bloodline_power")
                    || e.id.to_lowercase().contains("bloodline_spell"))),
        "level-10 Sorcerer must not fabricate any bloodline-power/bloodline-spell record (no bloodline entry exists at level 10 to ground — the column is blank): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the level-8 fixture is unaffected by this widening -----

#[test]
fn sorcerer_level9_truth_is_unchanged_by_this_slice() {
    let input = load(SORCERER_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(base_attack.value, 4, "Sorcerer level 9 base attack bonus must stay 4");

    let will = explanation(&computation, "class_chassis.sorcerer.base_save.will");
    assert_eq!(will.value, 6, "Sorcerer level 9 good Will must stay 6");
}

// ----- Negative control: level 21 stays unrecognized by this slice -----
//
// SD18 widened Sorcerer support to level 20, PF1's level cap
// (tests/sd18_sorcerer_level20_widening.rs), so this boundary moved to 21
// (which does not exist in PF1), mirroring the exact same boundary move
// every other Barbarian/Bard/Cleric/Fighter/Paladin/Rogue/Ranger/Wizard
// level-20 widening cycle made to its own sibling level-10/level-11/
// level-12 progression test.

#[test]
fn sorcerer_level_21_is_not_promoted_by_this_slice() {
    let level_21 = SORCERER_LEVEL10_FIXTURE.replace("class:sorcerer:10", "class:sorcerer:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.sorcerer.")
                || e.id == "class_chassis.spell_baseline.sorcerer"),
        "level-21 Sorcerer must not gain any bounded sorcerer chassis explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the sorcerer path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_sorcerer_level10_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.sorcerer.")
                || e.id == "class_chassis.spell_baseline.sorcerer"),
        "the Fighter chassis must not surface any sorcerer-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Sorcerer is not promoted -----

#[test]
fn multiclass_sorcerer_level10_is_not_promoted_by_this_slice() {
    let multiclass = SORCERER_LEVEL10_FIXTURE.replace(
        "class_level=class:sorcerer:10",
        "class_level=class:sorcerer:10\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.sorcerer.")
                || e.id == "class_chassis.spell_baseline.sorcerer"),
        "multiclass Sorcerer must not gain any bounded sorcerer chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Sorcerer must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-9 widening -----

#[test]
fn matrix_sorcerer_row_names_level_10_widening() {
    let matrix = seeded_current_truth();
    let sorcerer = matrix
        .row("class.sorcerer.progression_and_spell_burden")
        .expect("sorcerer progression_and_spell_burden row must exist");

    assert_eq!(sorcerer.support_state, SupportState::Supported);
    assert_eq!(sorcerer.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        sorcerer.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        sorcerer
            .grounding_ref
            .contains("sd13_sorcerer_level10_progression"),
        "sorcerer row must cite the live SD13-E5 level-10 proof surface: {}",
        sorcerer.grounding_ref
    );
    let note = sorcerer.blocker_or_lossiness_note;
    assert!(
        note.contains("level 10") || note.contains("level-10"),
        "sorcerer partial note must name the level-10 widening: {note}"
    );
}
