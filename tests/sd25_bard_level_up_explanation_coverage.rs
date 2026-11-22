//! SD-25 Epic 7 (criterion 7.6 — per-class residue audit for Bard).
//!
//! **Background**: SD-24 Epic 4 found and fixed a bug in
//! `src/rules_core/level_up/wizard.rs`: a later `pilot_compute.rs`
//! grounding (SD-21 E6b.2's `ground_wizard_prepared_spellbook`) landed
//! real `class_spell.wizard.*` explanations into
//! `compute_pilot_base_chassis`'s `.explanations` vector, but Wizard's own
//! `LevelUpPlan` explanation-id filter was never widened to admit the new
//! prefix, so those real facts were silently dropped from every
//! `LevelUpPlan` (see `tests/sd24_wizard_level_up_spell_coverage.rs`).
//! Wizard's fix added an explicit `WIZARD_RECOGNITION_ID` whitelist entry
//! (`"class_chassis.spell_baseline.wizard"`, which does NOT match the
//! `"class_chassis.wizard."` prefix shape) alongside the prefix widening.
//! SD-25's own 7.4 cycle confirmed Sorcerer's module already carries the
//! identical `SORCERER_RECOGNITION_ID` whitelist entry.
//!
//! **This audit's finding for Bard (verified, real bug — NOT a negative
//! finding): Bard's `bard.rs` is missing the exact same whitelist entry.**
//! `pilot_compute.rs`'s `explain_bard_level1_spell_baseline` pushes a
//! `"class_chassis.spell_baseline.bard"` recognition explanation (a bounded
//! +0 "Recognized deterministic Human Bard ... spell-bearing baseline"
//! record, the identical shape as Wizard's and Sorcerer's own recognition
//! records) onto `.explanations`. That id does NOT match either of
//! `bard.rs`'s two admitted prefixes (`"class_chassis.bard."` /
//! `"class_feature.bard."` — the id's own second segment is
//! `"spell_baseline"`, not `"bard"`), so `append_class_feature_grants`'s
//! `is_bard_class_feature_id` check silently `continue`s past it on every
//! transition, including the very first Bard level (0 -> 1, i.e. initial
//! class entry), where the id transitions `None -> Some(0)` — a genuine
//! change under this module's own diffing rule — yet produces no `Grant` in
//! the `LevelUpPlan`. This is the exact Wizard SD-24 bug shape: a real
//! `ComputationExplanation` record (confirmed via `compute_pilot_base_chassis`
//! reading only `.explanations`, never `.diagnostics`) silently dropped by
//! an explanation-id filter that was never widened to admit it.
//!
//! `every_changed_bard_explanation_produces_a_grant` walks the FULL
//! from/to explanation diff generically (by id, not a hand-picked
//! allowlist) starting at `from_level = 0` (uncovering exactly the missed
//! 0 -> 1 recognition transition Wizard's own bug hid in) through the full
//! supported level range, and asserts every changed, filter-admitted id
//! surfaces a matching `Grant`. Confirmed to fail (RED) against `bard.rs`
//! before this cycle's fix — the recognition explanation
//! (`class_chassis.spell_baseline.bard`) changes `None -> Some(0)` on the
//! 0 -> 1 transition but has no matching `Grant` — and to pass (GREEN)
//! after `bard.rs` gains the `BARD_RECOGNITION_ID` whitelist entry mirroring
//! Wizard's/Sorcerer's own fix.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState,
};
use codex::rules_core::level_up::bard::compute_bard_level_up_grants;
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;

const BARD_RECOGNITION_ID: &str = "class_chassis.spell_baseline.bard";
const BARD_EXPLANATION_PREFIX: &str = "class_chassis.bard.";
const BARD_FEATURE_PREFIX: &str = "class_feature.bard.";
const PERFORMANCE_EXECUTION_ROUNDS_EXCEEDED_DIAGNOSTIC_ID: &str =
    "class_feature.bard.bardic_performance_execution.rounds_exceeded";
const PERFORMANCE_EXECUTION_NOT_PERFORMING_EXPLANATION_ID: &str =
    "class_feature.bard.bardic_performance_execution.not_performing";
const SPONTANEOUS_UNSUPPORTED_DIAGNOSTIC_ID: &str =
    "class_spell.bard.spontaneous_known_and_per_day.unsupported";

/// Deterministic single-class Human Bard input at `level` (canonical
/// ability-score spread), mirroring `sd25_sorcerer_level_up_explanation_coverage.rs`'s
/// own `sorcerer_at_level` builder. `level: 0` is a deliberately
/// out-of-range probe value (mirrors `bard_chassis_explanations`'s own
/// probe-at-level-0 construction for the very first level transition) —
/// `explain_bard_level1_spell_baseline`'s own `supported_bard_level` gate
/// rejects it, producing zero explanations, which is exactly the "before"
/// state this file's 0 -> 1 transition exercises.
fn bard_at_level(level: u8) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd25_bard_residue_audit".to_string()),
        source_package_id: "sd25_bard_residue_audit".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:bard".to_string(),
                level,
            }],
            ability_scores: AbilityScores {
                strength: 10,
                dexterity: 14,
                constitution: 12,
                intelligence: 10,
                wisdom: 8,
                charisma: 16,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

fn is_bard_pillar_id(id: &str) -> bool {
    id == BARD_RECOGNITION_ID
        || id.starts_with(BARD_EXPLANATION_PREFIX)
        || id.starts_with(BARD_FEATURE_PREFIX)
}

// ----- Positive proof: the filter drops nothing real, at any level transition -----

#[test]
fn every_changed_bard_explanation_produces_a_grant() {
    // Starts at from_level = 0 (unlike a supported-range-only loop) so the
    // 0 -> 1 initial-class-entry transition -- exactly where Bard's
    // recognition explanation changes `None -> Some(0)` -- is exercised.
    // Then walks the full supported level-1..20 range, mirroring
    // `sd25_sorcerer_level_up_explanation_coverage.rs`'s own loop shape.
    for from_level in 0..20u8 {
        let to_level = from_level + 1;
        let character = bard_at_level(to_level);

        let from_computation = compute_pilot_base_chassis(&bard_at_level(from_level));
        let to_computation = compute_pilot_base_chassis(&character);
        let plan = compute_bard_level_up_grants(&character, from_level, to_level);

        for to_explanation in &to_computation.explanations {
            if !is_bard_pillar_id(&to_explanation.id) {
                // Not a Bard-owned pillar -- out of this module's own
                // scope, exactly as `bard.rs`'s own filter documents.
                continue;
            }

            let from_value = from_computation
                .explanations
                .iter()
                .find(|explanation| explanation.id == to_explanation.id)
                .map(|explanation| explanation.value);
            if from_value == Some(to_explanation.value) {
                // Genuinely unchanged at this transition -- not this
                // test's concern (Wizard's own bug only ever manifested on
                // a CHANGED value being silently dropped).
                continue;
            }

            let has_matching_grant = plan
                .automatic_features
                .iter()
                .any(|grant| grant.source_table.column_key == to_explanation.id)
                // Bardic performance rounds-per-day is re-granted as a
                // ResourcePoolDelta, not a Grant (handled below); the
                // four class-table-covered ids are re-granted under a
                // different column_key by `append_class_table_grants`.
                || to_explanation.id
                    == "class_chassis.bard.bardic_performance_rounds_per_day"
                || matches!(
                    to_explanation.id.as_str(),
                    "class_chassis.bard.base_attack_bonus"
                        | "class_chassis.bard.base_save.fortitude"
                        | "class_chassis.bard.base_save.reflex"
                        | "class_chassis.bard.base_save.will"
                );
            let has_matching_pool_delta = to_explanation.id
                == "class_chassis.bard.bardic_performance_rounds_per_day"
                && plan.resource_pool_change.pools.iter().any(|pool| {
                    pool.pool_id == "bardic_performance_rounds_per_day"
                        && pool.to_value == to_explanation.value
                });

            assert!(
                has_matching_grant || has_matching_pool_delta,
                "bard level {from_level} -> {to_level}: explanation '{}' changed from {:?} to \
                 {} but no matching Grant/ResourcePoolDelta was found in the LevelUpPlan (the \
                 exact Wizard SD-24 bug shape -- a real grounded fact silently dropped by the \
                 explanation-id filter). Grants present: {:#?}; pool deltas: {:#?}",
                to_explanation.id,
                from_value,
                to_explanation.value,
                plan.automatic_features,
                plan.resource_pool_change.pools
            );
        }
    }
}

/// Names the recognition record explicitly by id, so a future accidental
/// narrowing of the whitelist is caught by name too, mirroring
/// `tests/sd24_wizard_level_up_spell_coverage.rs`'s own named-id assertions.
#[test]
fn bard_recognition_explanation_produces_a_grant_on_first_level() {
    let character = bard_at_level(1);
    let plan = compute_bard_level_up_grants(&character, 0, 1);
    assert!(
        plan.automatic_features
            .iter()
            .any(|grant| grant.source_table.column_key == BARD_RECOGNITION_ID),
        "the bard 0 -> 1 LevelUpPlan must carry a Grant for '{BARD_RECOGNITION_ID}' (the \
         recognition record pilot_compute.rs grounds on initial class entry); grants present: \
         {:#?}",
        plan.automatic_features
    );
}

// ----- Verified negative finding: both "unsupported" diagnostics are correctly excluded -----

#[test]
fn bardic_performance_execution_diagnostic_never_leaks_into_explanations_or_grants() {
    // (v0.6 alpha swarm, risks item 8) The old unconditional "unsupported"
    // diagnostic is retired -- ground_or_block_bard_bardic_performance is
    // now a real, conditional engine. These bare fixtures have no bardic
    // performance activation, a genuinely valid posture, so
    // rounds_exceeded correctly never fires across the whole sweep; the
    // honest not_performing record grounds instead.
    for from_level in 1..20u8 {
        let to_level = from_level + 1;
        let character = bard_at_level(to_level);
        let computation = compute_pilot_base_chassis(&character);

        assert!(
            !computation
                .diagnostics
                .iter()
                .any(|d| d.id == PERFORMANCE_EXECUTION_ROUNDS_EXCEEDED_DIAGNOSTIC_ID),
            "a genuinely valid not-performing posture must not claim-block on rounds_exceeded at \
             level {to_level}: {:?}",
            computation.diagnostics
        );
        assert!(
            computation
                .explanations
                .iter()
                .any(|e| e.id == PERFORMANCE_EXECUTION_NOT_PERFORMING_EXPLANATION_ID),
            "bard level {to_level} must ground the honest not-performing record: {:?}",
            computation.explanations
        );
        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id == PERFORMANCE_EXECUTION_ROUNDS_EXCEEDED_DIAGNOSTIC_ID),
            "the rounds_exceeded diagnostic id must never appear in .explanations at level \
             {to_level}"
        );

        let plan = compute_bard_level_up_grants(&character, from_level, to_level);
        assert!(
            !plan.automatic_features.iter().any(|grant| grant
                .source_table
                .column_key
                == PERFORMANCE_EXECUTION_ROUNDS_EXCEEDED_DIAGNOSTIC_ID),
            "the LevelUpPlan for bard {from_level} -> {to_level} must never fabricate a Grant \
             from the bardic performance-execution claim-blocking diagnostic: {:#?}",
            plan.automatic_features
        );
    }
}

#[test]
fn spontaneous_known_and_per_day_diagnostic_never_leaks_into_explanations_or_grants() {
    for from_level in 1..20u8 {
        let to_level = from_level + 1;
        let character = bard_at_level(to_level);
        let computation = compute_pilot_base_chassis(&character);

        // (v0.6 alpha swarm, risks item 8, known-spell closure)
        // SPONTANEOUS_UNSUPPORTED_DIAGNOSTIC_ID is no longer unconditional --
        // these fixtures have zero known spells, a genuinely valid posture,
        // so the diagnostic correctly does not fire. If absent, confirm no
        // spell is fabricated merely because it stopped firing.
        match computation
            .diagnostics
            .iter()
            .find(|d| d.id == SPONTANEOUS_UNSUPPORTED_DIAGNOSTIC_ID)
        {
            Some(diagnostic) => assert!(
                diagnostic.claim_blocking,
                "the spontaneous known-spell / per-day diagnostic must stay claim-blocking at \
                 level {to_level}"
            ),
            None => {
                let known_count = computation
                    .explanations
                    .iter()
                    .find(|e| e.id == "class_spell.bard.known_spells")
                    .map(|e| e.value)
                    .unwrap_or(-1);
                assert_eq!(
                    known_count, 0,
                    "no spells are fabricated merely because the blocker stopped firing at \
                     level {to_level}: {:?}",
                    computation.diagnostics
                );
            }
        }
        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id == SPONTANEOUS_UNSUPPORTED_DIAGNOSTIC_ID),
            "the spontaneous known-spell / per-day diagnostic id must never appear in \
             .explanations at level {to_level}"
        );

        let plan = compute_bard_level_up_grants(&character, from_level, to_level);
        assert!(
            !plan.automatic_features.iter().any(|grant| grant
                .source_table
                .column_key
                == SPONTANEOUS_UNSUPPORTED_DIAGNOSTIC_ID),
            "the LevelUpPlan for bard {from_level} -> {to_level} must never fabricate a Grant \
             from the spontaneous known-spell / per-day claim-blocking diagnostic: {:#?}",
            plan.automatic_features
        );
    }
}
