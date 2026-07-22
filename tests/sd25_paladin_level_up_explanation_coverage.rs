//! SD-25 Epic 7 (criterion 7.9 — per-class residue audit for Paladin).
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
//! SD-25's 7.6 cycle found the identical shape recurred in Bard
//! (`class_chassis.spell_baseline.bard` missing from `bard.rs`'s filter).
//!
//! **This audit's own intake row (`progress.md` §`## DISCOVERED`, 7.1)
//! cited two ids as the suspected gap: `class_spell.paladin.partial_caster.
//! unsupported` (`pilot_compute.rs:8592`) and `class_spell.hybrid.paladin.
//! unsupported` (`pilot_compute.rs:7606`). Both are VERIFIED NEGATIVE on
//! that specific claim** — reading `pilot_compute.rs` end-to-end confirms
//! both are pushed via `diagnostics.push(...)`, landing on the
//! structurally separate `ComputationDiagnostic` `Vec`
//! (`PilotBaseChassisComputation.diagnostics`), never merged into
//! `.explanations`. `paladin.rs`'s `paladin_chassis_explanations` reads
//! only `.explanations`, so neither id was ever reachable by the filter
//! regardless of what prefix it admits — the same shape as Cleric's 7.2,
//! Sorcerer's 7.4, and Bard's 7.6 resolutions on their own intake-cited
//! diagnostic ids.
//!
//! **But mirroring Bard's 7.6 finding (a real gap the intake row's own
//! cited id did NOT name) turned up a genuine bug here too.**
//! `pilot_compute.rs`'s `explain_hybrid_level1_chassis` — called
//! unconditionally from `compute_pilot_base_chassis` for every input, and
//! internally gated on `hybrid_level1_class` (matches a single-class
//! Paladin or Ranger at EXACTLY class level 1) — pushes a bounded +0
//! recognition record `"class_chassis.hybrid_baseline.paladin"` onto
//! `.explanations` for a level-1 Human Paladin. That id's second segment
//! is `"hybrid_baseline"`, not `"paladin"`, so it matches NEITHER of
//! `paladin.rs`'s two admitted prefixes (`"class_chassis.paladin."` /
//! `"class_feature.paladin."`) — `append_class_feature_grants`'s
//! `is_paladin_class_feature_id` check silently `continue`s past it. On
//! the very first Paladin level transition (0 -> 1, i.e. initial class
//! entry — the same shape Bard's 0 -> 1 transition exercised), the id
//! changes `None -> Some(0)`, a genuine change under this module's own
//! diffing rule, yet produces no `Grant` in the `LevelUpPlan`. This is the
//! exact Wizard SD-24 bug shape (a real, grounded `ComputationExplanation`
//! silently dropped by an explanation-id filter never widened to admit
//! it), just on the hybrid-baseline recognition id rather than a
//! `class_spell.*` one.
//!
//! `every_changed_paladin_explanation_produces_a_grant` walks the FULL
//! from/to explanation diff generically (by id, not a hand-picked
//! allowlist) starting at `from_level = 0` (uncovering exactly the missed
//! 0 -> 1 recognition transition) through the full supported level range,
//! and asserts every changed, filter-admitted id surfaces a matching
//! `Grant`/`ResourcePoolDelta`. Confirmed to FAIL (RED) against
//! pre-fix `paladin.rs` (the hybrid-baseline recognition explanation
//! changes `None -> Some(0)` on the 0 -> 1 transition but has no matching
//! `Grant`) and to PASS (GREEN) after `paladin.rs` gains a
//! `PALADIN_HYBRID_BASELINE_RECOGNITION_ID` whitelist entry mirroring
//! Wizard's/Sorcerer's/Bard's own fix shape.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState,
};
use codex::rules_core::level_up::paladin::compute_paladin_level_up_grants;
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;

const PALADIN_HYBRID_BASELINE_RECOGNITION_ID: &str = "class_chassis.hybrid_baseline.paladin";
const PALADIN_EXPLANATION_PREFIX: &str = "class_chassis.paladin.";
const PALADIN_FEATURE_PREFIX: &str = "class_feature.paladin.";
const PARTIAL_CASTER_UNSUPPORTED_DIAGNOSTIC_ID: &str =
    "class_spell.paladin.partial_caster.unsupported";
const HYBRID_SPELL_UNSUPPORTED_DIAGNOSTIC_ID: &str = "class_spell.hybrid.paladin.unsupported";
const HYBRID_FEATURE_UNSUPPORTED_DIAGNOSTIC_ID: &str = "class_feature.hybrid.paladin.unsupported";

/// Deterministic single-class Human Paladin input at `level` (Charisma 14,
/// mirroring `sd20_levelup_paladin.rs`'s own fixture spread). `level: 0` is
/// a deliberately out-of-range probe value (mirrors
/// `paladin_chassis_explanations`'s own probe-at-level-0 construction for
/// the very first level transition) -- both `supported_paladin_level`'s
/// and `hybrid_level1_class`'s own gates reject it, producing zero
/// paladin-specific explanations, which is exactly the "before" state
/// this file's 0 -> 1 transition exercises.
fn paladin_at_level(level: u8) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd25_paladin_residue_audit".to_string()),
        source_package_id: "sd25_paladin_residue_audit".to_string(),
        chosen: ChosenCharacterState {
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:paladin".to_string(),
                level,
            }],
            ability_scores: AbilityScores {
                strength: 16,
                dexterity: 12,
                constitution: 14,
                intelligence: 8,
                wisdom: 10,
                charisma: 14,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

fn is_paladin_pillar_id(id: &str) -> bool {
    id == PALADIN_HYBRID_BASELINE_RECOGNITION_ID
        || id.starts_with(PALADIN_EXPLANATION_PREFIX)
        || id.starts_with(PALADIN_FEATURE_PREFIX)
}

// ----- Positive proof: the filter drops nothing real, at any level transition -----

#[test]
fn every_changed_paladin_explanation_produces_a_grant() {
    // Starts at from_level = 0 (unlike a supported-range-only loop) so the
    // 0 -> 1 initial-class-entry transition -- exactly where Paladin's
    // hybrid-baseline recognition explanation changes `None -> Some(0)` --
    // is exercised, mirroring `sd25_bard_level_up_explanation_coverage.rs`'s
    // own loop shape.
    for from_level in 0..20u8 {
        let to_level = from_level + 1;
        let character = paladin_at_level(to_level);

        let from_computation = compute_pilot_base_chassis(&paladin_at_level(from_level));
        let to_computation = compute_pilot_base_chassis(&character);
        let plan = compute_paladin_level_up_grants(&character, from_level, to_level);

        for to_explanation in &to_computation.explanations {
            if !is_paladin_pillar_id(&to_explanation.id) {
                // Not a Paladin-owned pillar -- out of this module's own
                // scope, exactly as `paladin.rs`'s own filter documents.
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
                // The four class-table-covered ids are re-granted under a
                // different column_key by `append_class_table_grants`
                // (`class_tables`'s own generic column names), per
                // `paladin.rs`'s own `CLASS_TABLE_COVERED_EXPLANATION_IDS`.
                || matches!(
                    to_explanation.id.as_str(),
                    "class_chassis.paladin.base_attack_bonus"
                        | "class_chassis.paladin.base_save.fortitude"
                        | "class_chassis.paladin.base_save.reflex"
                        | "class_chassis.paladin.base_save.will"
                );
            let has_matching_pool_delta = matches!(
                to_explanation.id.as_str(),
                "class_chassis.paladin.smite_evil_uses_per_day"
                    | "class_chassis.paladin.lay_on_hands_uses_per_day"
            ) && plan.resource_pool_change.pools.iter().any(|pool| {
                pool.to_value == to_explanation.value
                    && (pool.pool_id == "smite_evil_uses_per_day"
                        || pool.pool_id == "lay_on_hands_uses_per_day")
            });
            // DISCOVERED (out of THIS cycle's scope, forwarded to
            // progress.md): `append_resource_pool_change` requires BOTH
            // `from_value` and `to_value` to be `Some` before pushing a
            // `ResourcePoolDelta` (see `paladin.rs`). Whenever a pool's
            // explanation id doesn't exist yet on the from-side --
            // `smite_evil_uses_per_day` at the 0 -> 1 initial-class-entry
            // transition (`supported_paladin_level`'s own gate excludes
            // level 0), and `lay_on_hands_uses_per_day` at the 1 -> 2
            // transition (below its own level-2 gate the id is instead
            // `class_chassis.paladin.level_gate.lay_on_hands`, a flat +0
            // "correctly absent" marker, not the real uses/day id) -- the
            // pool never emits a delta on that one transition. A real,
            // verified gap, but a DIFFERENT bug mechanism than the
            // explanation-id filter this cycle (7.9) targets (the id
            // passes the filter and is correctly excluded from
            // `automatic_features` as "covered elsewhere"; the drop
            // happens one step later, inside the pool-diff's own
            // both-sides-`Some` requirement). Not fixed here — see this
            // cycle's receipt / progress.md `## DISCOVERED` for the
            // forwarding note.
            let is_documented_newly_introduced_pool_gap = from_value.is_none()
                && matches!(
                    to_explanation.id.as_str(),
                    "class_chassis.paladin.smite_evil_uses_per_day"
                        | "class_chassis.paladin.lay_on_hands_uses_per_day"
                );

            assert!(
                has_matching_grant
                    || has_matching_pool_delta
                    || is_documented_newly_introduced_pool_gap,
                "paladin level {from_level} -> {to_level}: explanation '{}' changed from {:?} \
                 to {} but no matching Grant/ResourcePoolDelta was found in the LevelUpPlan (the \
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
/// `tests/sd24_wizard_level_up_spell_coverage.rs`'s own named-id
/// assertions and `sd25_bard_level_up_explanation_coverage.rs`'s
/// `bard_recognition_explanation_produces_a_grant_on_first_level`.
#[test]
fn hybrid_baseline_recognition_explanation_produces_a_grant_on_first_level() {
    let character = paladin_at_level(1);
    let plan = compute_paladin_level_up_grants(&character, 0, 1);
    assert!(
        plan.automatic_features
            .iter()
            .any(|grant| grant.source_table.column_key == PALADIN_HYBRID_BASELINE_RECOGNITION_ID),
        "the paladin 0 -> 1 LevelUpPlan must carry a Grant for \
         '{PALADIN_HYBRID_BASELINE_RECOGNITION_ID}' (the recognition record pilot_compute.rs \
         grounds on initial class entry via explain_hybrid_level1_chassis); grants present: \
         {:#?}",
        plan.automatic_features
    );
}

// ----- Verified negative finding: the intake's cited diagnostics never leak in -----

#[test]
fn partial_caster_unsupported_diagnostic_never_leaks_into_explanations_or_grants() {
    for from_level in 1..20u8 {
        let to_level = from_level + 1;
        let character = paladin_at_level(to_level);
        let computation = compute_pilot_base_chassis(&character);

        let diagnostic = computation
            .diagnostics
            .iter()
            .find(|d| d.id == PARTIAL_CASTER_UNSUPPORTED_DIAGNOSTIC_ID)
            .unwrap_or_else(|| {
                panic!(
                    "paladin level {to_level} must still carry the partial-caster \
                     claim-blocking diagnostic: {:?}",
                    computation.diagnostics
                )
            });
        assert!(
            diagnostic.claim_blocking,
            "the partial-caster diagnostic must stay claim-blocking at level {to_level}"
        );
        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id == PARTIAL_CASTER_UNSUPPORTED_DIAGNOSTIC_ID),
            "the partial-caster diagnostic id must never appear in .explanations at level \
             {to_level}"
        );

        let plan = compute_paladin_level_up_grants(&character, from_level, to_level);
        assert!(
            !plan.automatic_features.iter().any(|grant| grant
                .source_table
                .column_key
                == PARTIAL_CASTER_UNSUPPORTED_DIAGNOSTIC_ID),
            "the LevelUpPlan for paladin {from_level} -> {to_level} must never fabricate a \
             Grant from the partial-caster claim-blocking diagnostic: {:#?}",
            plan.automatic_features
        );
    }
}

#[test]
fn hybrid_spell_and_feature_unsupported_diagnostics_never_leak_into_explanations_or_grants() {
    let character = paladin_at_level(1);
    let computation = compute_pilot_base_chassis(&character);

    for diagnostic_id in [
        HYBRID_SPELL_UNSUPPORTED_DIAGNOSTIC_ID,
        HYBRID_FEATURE_UNSUPPORTED_DIAGNOSTIC_ID,
    ] {
        let diagnostic = computation
            .diagnostics
            .iter()
            .find(|d| d.id == diagnostic_id)
            .unwrap_or_else(|| {
                panic!(
                    "paladin level 1 must still carry the '{diagnostic_id}' claim-blocking \
                     diagnostic: {:?}",
                    computation.diagnostics
                )
            });
        assert!(
            diagnostic.claim_blocking,
            "'{diagnostic_id}' must stay claim-blocking at paladin level 1"
        );
        assert!(
            !computation.explanations.iter().any(|e| e.id == diagnostic_id),
            "'{diagnostic_id}' must never appear in .explanations at paladin level 1"
        );
    }

    let plan = compute_paladin_level_up_grants(&character, 0, 1);
    for diagnostic_id in [
        HYBRID_SPELL_UNSUPPORTED_DIAGNOSTIC_ID,
        HYBRID_FEATURE_UNSUPPORTED_DIAGNOSTIC_ID,
    ] {
        assert!(
            !plan
                .automatic_features
                .iter()
                .any(|grant| grant.source_table.column_key == diagnostic_id),
            "the paladin 0 -> 1 LevelUpPlan must never fabricate a Grant from '{diagnostic_id}': \
             {:#?}",
            plan.automatic_features
        );
    }
}
