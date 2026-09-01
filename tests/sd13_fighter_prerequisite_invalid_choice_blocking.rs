//! SD13-E5-F9 deterministic Human Fighter prerequisite / invalid-choice blocking proof.
//!
//! Proves the first truthful SD13-E5 cross-cutting slice: the accepted deterministic
//! Human Fighter level-1/2/3 feat-choice path stays computed exactly as it is on accepted
//! `origin/develop`, while non-canonical mutations of that same already-grounded feat-choice
//! seam become explicit claim-blocking evidence instead of riding through as if they were
//! legal computed builds.
//!
//! It is intentionally not a general feat legality or prerequisite engine. It grounds no
//! alternative feat effect, no alternative prerequisite truth, and no alternative Human
//! ability-bonus target. It only preserves the canonical selections and claim-blocks
//! deviations on the bounded Human Fighter seam, naming the offending choice identity and
//! stating plainly that broader feat/prerequisite legality is outside this bounded proof
//! without a general engine.
//!
//! Invalid-choice cases are expressed by safe in-test mutation of the accepted Fighter
//! fixtures; no new fixture file is introduced.

use codex::rules_core::pilot_compute::{
    HeadlessReceiptStatus, PilotBaseChassisComputation, build_pilot_headless_receipt,
    compute_pilot_base_chassis,
};
use codex::rules_core::pilot_failure::PrimaryOwner;
use codex::rules_core::pilot_view_model::PilotViewModel;
mod common;
use common::load;

const LEVEL_1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");
const LEVEL_2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level2_sd13_deterministic_input.txt");
const LEVEL_3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level3_sd13_deterministic_input.txt");

/// Return the claim-blocking diagnostics that name the non-canonical feat-choice seam.
fn feat_choice_blocks(
    computation: &PilotBaseChassisComputation,
) -> Vec<&codex::rules_core::pilot_compute::ComputationDiagnostic> {
    computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking && d.id.starts_with("feat_choice."))
        .collect()
}

fn has_claim_block(computation: &PilotBaseChassisComputation) -> bool {
    computation.diagnostics.iter().any(|d| d.claim_blocking)
}

// ----- Preservation: the canonical deterministic path stays computed -----

#[test]
fn canonical_level_1_2_3_paths_stay_computed_and_unblocked() {
    for fixture in [LEVEL_1_FIXTURE, LEVEL_2_FIXTURE, LEVEL_3_FIXTURE] {
        let input = load(fixture);
        let computation = compute_pilot_base_chassis(&input);

        assert!(
            !has_claim_block(&computation),
            "canonical deterministic Human Fighter path must not be claim-blocked: {:?}",
            computation.diagnostics
        );
        // No non-canonical feat-choice diagnostic may fire on the accepted selections.
        assert!(
            feat_choice_blocks(&computation).is_empty(),
            "canonical feat-choice selections must not raise a non-canonical block: {:?}",
            computation.diagnostics
        );

        let receipt = build_pilot_headless_receipt(&input);
        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "canonical deterministic Human Fighter path must report a computed receipt"
        );
    }
}

#[test]
fn canonical_path_preserves_selected_skill_and_derived_outputs() {
    // Level 1: Chain Shirt armor-check penalty -2, so Climb/Swim = 6, Intimidate = 3.
    // CG-03 fix: STR modifier is now +4 (base 16 + 2 Human racial), not +3.
    let level_1 = compute_pilot_base_chassis(&load(LEVEL_1_FIXTURE));
    assert_eq!(level_1.selected_skill_modifiers.climb, 6);
    assert_eq!(level_1.selected_skill_modifiers.intimidate, 3);
    assert_eq!(level_1.selected_skill_modifiers.swim, 6);
    assert_eq!(level_1.baseline_melee_attack_bonus, 6);
    assert_eq!(level_1.baseline_armor_class, 17);

    // Level 3: armor training reduces the penalty to -1, so Climb/Swim rise to 7.
    let level_3 = compute_pilot_base_chassis(&load(LEVEL_3_FIXTURE));
    assert_eq!(level_3.selected_skill_modifiers.climb, 7);
    assert_eq!(level_3.selected_skill_modifiers.swim, 7);
    assert_eq!(level_3.selected_skill_modifiers.intimidate, 3);
    assert_eq!(level_3.baseline_melee_attack_bonus, 8);
}

// ----- Invalid-choice blocking: level-1 character feat -----

#[test]
fn non_canonical_level_1_character_feat_is_claim_blocked_and_named() {
    // Mutate only the level-1 character-feat choice slot away from feat:power_attack.
    // The feat=feat:power_attack line and the Power Attack equipment posture are left
    // intact, so nothing but the feat-choice legality seam can block this build.
    let mutated = LEVEL_1_FIXTURE.replace(
        "choice=choice:level_1_character_feat:feat:power_attack",
        "choice=choice:level_1_character_feat:feat:iron_will",
    );
    assert!(
        mutated.contains("choice=choice:level_1_character_feat:feat:iron_will"),
        "test setup should have mutated the level-1 character-feat choice slot"
    );
    let input = load(&mutated);
    let computation = compute_pilot_base_chassis(&input);

    let blocks = feat_choice_blocks(&computation);
    assert!(
        !blocks.is_empty(),
        "non-canonical level-1 character feat must be claim-blocked, got: {:?}",
        computation.diagnostics
    );
    // The block must name the offending choice-set identity, not hide it in a generic bucket.
    assert!(
        blocks
            .iter()
            .any(|d| d.message.contains("choice:level_1_character_feat")),
        "feat-choice block must name the offending choice:level_1_character_feat slot: {:?}",
        blocks
    );
    // The block must make clear that alternative feat/prerequisite legality is outside
    // this bounded proof without a general engine.
    assert!(
        blocks
            .iter()
            .any(|d| d.message.contains("feat-effect") || d.message.contains("prerequisite")),
        "feat-choice block must state it grounds no general feat/prerequisite engine: {:?}",
        blocks
    );
}

// ----- Invalid-choice blocking: level-2 fighter bonus feat -----

#[test]
fn non_canonical_fighter_bonus_feat_2_is_claim_blocked_and_named() {
    // Mutate only choice:fighter_bonus_feat_2 away from feat:toughness on the level-2
    // fixture. Toughness contributes no bounded computed value, so before this slice the
    // mutation rode through as a computed success; it must now be claim-blocked.
    let mutated = LEVEL_2_FIXTURE.replace(
        "choice=choice:fighter_bonus_feat_2:feat:toughness",
        "choice=choice:fighter_bonus_feat_2:feat:iron_will",
    );
    assert!(
        mutated.contains("choice=choice:fighter_bonus_feat_2:feat:iron_will"),
        "test setup should have mutated the level-2 bonus-feat choice slot"
    );
    let input = load(&mutated);
    let computation = compute_pilot_base_chassis(&input);

    let blocks = feat_choice_blocks(&computation);
    assert!(
        !blocks.is_empty(),
        "non-canonical choice:fighter_bonus_feat_2 must be claim-blocked, got: {:?}",
        computation.diagnostics
    );
    assert!(
        blocks
            .iter()
            .any(|d| d.message.contains("choice:fighter_bonus_feat_2")),
        "feat-choice block must name the offending choice:fighter_bonus_feat_2 slot: {:?}",
        blocks
    );
}

// ----- Invalid-choice blocking: Human bonus feat choice (the silent-ride lie) -----

#[test]
fn non_canonical_human_bonus_feat_choice_is_claim_blocked_even_when_feat_line_stays() {
    // Mutate only the human_bonus_feat choice slot while leaving the feat=feat:dodge line
    // intact. The combat posture only checks the selected feat list, so before this slice
    // this deviation rode through as computed even though the named Human bonus-feat choice
    // no longer matched the accepted seam.
    let mutated = LEVEL_1_FIXTURE.replace(
        "choice=choice:human_bonus_feat:feat:dodge",
        "choice=choice:human_bonus_feat:feat:iron_will",
    );
    assert!(
        mutated.contains("choice=choice:human_bonus_feat:feat:iron_will")
            && mutated.contains("feat=feat:dodge"),
        "test setup should mutate only the choice slot and keep the feat:dodge selection"
    );
    let input = load(&mutated);
    let computation = compute_pilot_base_chassis(&input);

    let blocks = feat_choice_blocks(&computation);
    assert!(
        !blocks.is_empty(),
        "non-canonical choice:human_bonus_feat must be claim-blocked, got: {:?}",
        computation.diagnostics
    );
    assert!(
        blocks
            .iter()
            .any(|d| d.message.contains("choice:human_bonus_feat")),
        "feat-choice block must name the offending choice:human_bonus_feat slot: {:?}",
        blocks
    );
}

// ----- Downstream propagation of the blocked posture -----

#[test]
fn blocked_feat_choice_propagates_through_receipt_classifier_and_view_model() {
    let mutated = LEVEL_2_FIXTURE.replace(
        "choice=choice:fighter_bonus_feat_2:feat:toughness",
        "choice=choice:fighter_bonus_feat_2:feat:iron_will",
    );
    let input = load(&mutated);
    let receipt = build_pilot_headless_receipt(&input);

    // Headless receipt status must reflect the blocked posture honestly.
    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Blocked,
        "non-canonical feat-choice build must produce a blocked receipt"
    );
    assert!(
        receipt
            .computation
            .diagnostics
            .iter()
            .any(|d| d.claim_blocking),
        "blocked receipt must preserve a claim-blocking diagnostic: {:?}",
        receipt.computation.diagnostics
    );

    // Primary-owner classification must reflect the blocked engine posture.
    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    // The blocked path must not fabricate a computed success snapshot.
    assert_eq!(
        view_model.snapshot, None,
        "blocked feat-choice build must not project a success snapshot"
    );
}

// ----- Bounded honesty: the ability-bonus target stays read-only canonical truth -----

#[test]
fn canonical_human_ability_bonus_target_remains_computed_truth() {
    // This slice does not widen into alternate Human ability-bonus support: the accepted
    // choice:human_ability_bonus -> ability:strength selection stays computed truth and is
    // surfaced as the Human ability-bonus race explanation without a claim block.
    let computation = compute_pilot_base_chassis(&load(LEVEL_1_FIXTURE));
    let ability = computation
        .explanations
        .iter()
        .find(|e| e.id == "race.human.ability_bonus_target")
        .expect("canonical path must surface the Human ability-bonus target explanation");
    // CG-03 fix: the Human ability-bonus choice's +2 racial Strength adjustment is now
    // applied before the modifier is derived, raising it from +3 to +4.
    assert_eq!(
        ability.value, 4,
        "canonical Strength ability-bonus target must keep its computed +4 modifier"
    );
    assert!(
        !has_claim_block(&computation),
        "the canonical ability-bonus target must not be claim-blocked: {:?}",
        computation.diagnostics
    );
}
