//! SD13-E5 Sorcerer level-9 progression grounding proof.
//!
//! Widens the accepted Sorcerer level-1..level-8 spontaneous spell-burden
//! baseline (most recently `tests/sd13_sorcerer_level8_progression.rs`) to
//! Sorcerer level 9, mirroring the sibling-class level-range-gate idiom
//! (`supported_sorcerer_level` is generalized from `1..=8` to `1..=9` via
//! `MAX_SUPPORTED_SORCERER_LEVEL = 9`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Sorcerer class table) were read directly
//! before writing any code or test:
//!
//! - level 9 base attack bonus is +4 (`9 / 2 = 4`, the Sorcerer's 1/2-BAB
//!   progression, numerically unchanged from level 8 — an integer-division
//!   coincidence) and base saves are +3 Fortitude (poor, `9 / 3 = 3`,
//!   genuinely risen from +2), +3 Reflex (poor, `9 / 3 = 3`, genuinely
//!   risen from +2), and +6 Will (good, `9 / 2 + 2 = 6`, numerically
//!   unchanged from level 8, a coincidence) — confirmed by the same
//!   formulas already grounded at levels 1-8, not re-derived.
//! - the bloodline choice recognition and the Arcane bloodline's class-skill
//!   choice recognition are not level-gated, so both still fire at level 9
//!   for the same fixture selections.
//! - the PF1 Core Rulebook Sorcerer class table's level-9 "Special" column
//!   reads "Bloodline power, bloodline spell" (verified independently
//!   against both primary sources, checked rather than assumed away) —
//!   UNLIKE the blank level-8 column, Sorcerer gains two new named entries
//!   at 9th level: the second bloodline power (the Arcane bloodline's own
//!   9th-level power is New Arcana) and the fourth bloodline spell (the
//!   Arcane bloodline's own 9th-level bloodline spell is overland flight).
//!   Both entries are bloodline-specific and not flat/identity-shaped, so
//!   this slice grounds no new pillar for level 9 either, mirroring exactly
//!   how the level-3 "Bloodline power, bloodline spell", level-5 "Bloodline
//!   spell", and level-7 "Bloodline feat, bloodline spell" entries were left
//!   unproven — both stay named by the pre-existing
//!   `arcane_bond_and_bloodline_progression.unsupported` diagnostic's
//!   language, unchanged. A dedicated negative test pins that no
//!   bloodline-power/bloodline-spell record is fabricated.
//!
//! It deliberately does not touch Arcane Bond, bloodline arcana, the
//! bloodline powers or bloodline spells gained at 3rd/5th/7th/9th level, the
//! 7th-level bloodline feat, bonus spells/feats at 3rd+, or the spontaneous
//! spell burden (all stay named-but-unproven, unchanged from levels 1-8),
//! and it does not ground Sorcerer level 10+. It also preserves the accepted
//! Sorcerer level-1..level-8 truth (unchanged), the Fighter negative
//! control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const SORCERER_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level8_sd13_deterministic_input.txt");

const SORCERER_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level9_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 9 -----

#[test]
fn sorcerer_level9_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(SORCERER_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(
        base_attack.value, 4,
        "Sorcerer level 9 1/2-BAB progression (9 / 2) must equal 4 — numerically unchanged \
         from level 8, an integer-division coincidence: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 9 (good Will only, poor Fortitude/Reflex) -----

#[test]
fn sorcerer_level9_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(SORCERER_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.sorcerer.base_save.fortitude");
    assert_eq!(
        fortitude.value, 3,
        "Sorcerer level 9 poor Fortitude (9/3) must equal 3, genuinely risen from 2 at level 8"
    );

    let reflex = explanation(&computation, "class_chassis.sorcerer.base_save.reflex");
    assert_eq!(
        reflex.value, 3,
        "Sorcerer level 9 poor Reflex (9/3) must equal 3, genuinely risen from 2 at level 8"
    );

    let will = explanation(&computation, "class_chassis.sorcerer.base_save.will");
    assert_eq!(will.value, 6, "Sorcerer level 9 good Will (9/2+2) must equal 6");
}

// ----- Bloodline choice recognition still fires at level 9 -----

#[test]
fn sorcerer_level9_still_recognizes_the_bloodline_choice() {
    let input = load(SORCERER_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.sorcerer.bloodline_choice");
    assert_eq!(
        choice.value, 0,
        "bloodline choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Arcane"),
        "bloodline choice recognition must still name the Arcane bloodline at level 9: {}",
        choice.detail
    );
}

// ----- Arcane bloodline class-skill choice recognition still fires at level 9 -----

#[test]
fn sorcerer_level9_still_recognizes_the_bloodline_class_skill_choice() {
    let input = load(SORCERER_LEVEL9_FIXTURE);
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
         9: {}",
        choice.detail
    );
}

// ----- The two existing burden diagnostics still fire at level 9 -----

#[test]
fn sorcerer_level9_still_claim_blocks_arcane_bond_and_spontaneous_spell_burdens() {
    let input = load(SORCERER_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"
            && d.claim_blocking),
        "level-9 Sorcerer must still claim-block on the Arcane Bond / bloodline \
         progression burden (which also names the 9th-level bloodline power/spell): {:?}",
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
fn sorcerer_level9_still_recognizes_the_spell_bearing_baseline() {
    let input = load(SORCERER_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.spell_baseline.sorcerer"),
        "level-9 Sorcerer must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
}

// ----- No bloodline-power/spell record is fabricated at level 9 -----

#[test]
fn sorcerer_level9_does_not_fabricate_the_ninth_level_bloodline_entries() {
    let input = load(SORCERER_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // SD-34 bucket-B batch cycle: found stale, unrelated to this cycle's own three
    // mechanisms -- same fix as `sd13_sorcerer_level10_progression.rs`'s own sibling test:
    // the pre-existing (SD-32 T12 Epic 8, `decisions.md §17`) generic bloodline-power pass
    // legitimately emits real, citation-backed `class_feature.sorcerer.bloodline.generic.*`
    // ids (including real PCGen var names containing "new_arcana"/"bloodline_power"-shaped
    // substrings) for the fixture's selected Arcane bloodline -- never a fabricated
    // per-character magnitude. Scoped by prefix, same shape as that sibling test's own fix.
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| !e.id.starts_with("class_feature.sorcerer.bloodline.generic.")
                && (e.id.to_lowercase().contains("new_arcana")
                    || e.id.to_lowercase().contains("bloodline_power")
                    || e.id.to_lowercase().contains("bloodline_spell"))),
        "level-9 Sorcerer must not fabricate any bloodline-power/bloodline-spell record (both \
         9th-level entries are bloodline-specific, not flat): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the level-8 fixture is unaffected by this widening -----

#[test]
fn sorcerer_level8_truth_is_unchanged_by_this_slice() {
    let input = load(SORCERER_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(base_attack.value, 4, "Sorcerer level 8 base attack bonus must stay 4");

    let fortitude = explanation(&computation, "class_chassis.sorcerer.base_save.fortitude");
    assert_eq!(fortitude.value, 2, "Sorcerer level 8 poor Fortitude must stay 2");
}

// ----- Level 10 was later widened into the supported tranche by a further slice -----

#[test]
fn sorcerer_level_10_was_later_widened_into_the_supported_tranche() {
    let level_10 = SORCERER_LEVEL9_FIXTURE.replace("class:sorcerer:9", "class:sorcerer:10");
    let input = load(&level_10);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.sorcerer.")),
        "level-10 Sorcerer is now recognized by the later level-10 widening slice \
         (tests/sd13_sorcerer_level10_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the sorcerer path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_sorcerer_level9_recognition() {
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
fn multiclass_sorcerer_level9_is_not_promoted_by_this_slice() {
    let multiclass = SORCERER_LEVEL9_FIXTURE.replace(
        "class_level=class:sorcerer:9",
        "class_level=class:sorcerer:9\nclass_level=class:fighter:1",
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
fn matrix_sorcerer_row_names_level_9_widening() {
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
            .contains("sd13_sorcerer_level9_progression"),
        "sorcerer row must cite the live SD13-E5 level-9 proof surface: {}",
        sorcerer.grounding_ref
    );
    let note = sorcerer.blocker_or_lossiness_note;
    assert!(
        note.contains("level 9") || note.contains("level-9"),
        "sorcerer partial note must name the level-9 widening: {note}"
    );
}
