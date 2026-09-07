//! SD-25 Epic 7 (criterion 7.10 — per-class residue audit for Ranger).
//!
//! **Background**: SD-24 Epic 4 found and fixed a bug in
//! `src/rules_core/level_up/wizard.rs`: a later `pilot_compute.rs`
//! grounding (SD-21 E6b.2's `ground_wizard_prepared_spellbook`) landed real
//! `class_spell.wizard.*` explanations into `compute_pilot_base_chassis`'s
//! `.explanations` vector, but Wizard's own `LevelUpPlan` explanation-id
//! filter was never widened to admit the new prefix, so those real facts
//! were silently dropped from every `LevelUpPlan` (see
//! `tests/sd24_wizard_level_up_spell_coverage.rs`). Wizard's fix added an
//! explicit `WIZARD_RECOGNITION_ID` whitelist entry
//! (`"class_chassis.spell_baseline.wizard"`, which does NOT match the
//! `"class_chassis.wizard."` prefix shape) alongside the prefix widening.
//! SD-25's own 7.6 cycle found and fixed the identical bug shape for Bard
//! (`BARD_RECOGNITION_ID` = `"class_chassis.spell_baseline.bard"`).
//!
//! **This audit's finding for Ranger (verified, real bug — NOT a negative
//! finding, mirroring Bard's 7.6 outcome rather than Cleric's/Druid's/
//! Sorcerer's own verified-negative outcomes): `ranger.rs` was missing the
//! analogous whitelist entry for a different, Ranger/Paladin-shared
//! recognition seam.** `pilot_compute.rs`'s `explain_hybrid_level1_chassis`
//! (the function that also grounds Paladin's
//! `class_chassis.hybrid_baseline.paladin`) pushes a bounded +0
//! `"class_chassis.hybrid_baseline.ranger"` recognition record onto
//! `.explanations` for a single-class Human Ranger at level 1 (it fires
//! unconditionally alongside
//! `explain_ranger_level1_chassis_and_class_feature_separation` for that
//! exact input shape — confirmed by reading `compute_pilot_base_chassis`'s
//! call sequence directly). That id's own second segment
//! (`hybrid_baseline`) never matched either of `ranger.rs`'s two admitted
//! prefixes (`"class_chassis.ranger."` / `"class_feature.ranger."`), so
//! `append_class_feature_grants`'s `is_ranger_class_feature_id` check
//! silently `continue`s past it on every transition, including the very
//! first Ranger level (0 -> 1, i.e. initial class entry), where the id
//! transitions `None -> Some(0)` — a genuine change under this module's own
//! diffing rule — yet produced no `Grant` in the `LevelUpPlan` before this
//! cycle's fix. This is the exact Wizard SD-24 bug shape: a real
//! `ComputationExplanation` record (confirmed via `compute_pilot_base_chassis`
//! reading only `.explanations`, never `.diagnostics`) silently dropped by
//! an explanation-id filter that was never widened to admit it.
//!
//! Every OTHER Ranger-specific explanation id `pilot_compute.rs` grounds —
//! including the full `class_chassis.ranger.partial_caster.*` spellcasting-
//! burden family (`effective_caster_level`, `spell_level_access`,
//! `base_spells_per_day.spell_level_N`, `bonus_spells_per_day.spell_level_N`,
//! `total_spells_per_day.spell_level_N`, `spell_save_dc.spell_level_N`,
//! newly opening at ranger level 4) — was independently confirmed (by this
//! file's own generalized `every_changed_ranger_explanation_produces_a_grant`
//! sweep) to already carry the `"class_chassis.ranger."` prefix
//! `ranger.rs`'s filter already admitted, so no `class_spell.ranger.*`-shaped
//! defect (the literal bug shape the 7.1 intake's blanket characterization
//! named) exists for Ranger — the real gap was the recognition-id shape
//! SD-25's 7.6 (Bard) cycle already found, not the 7.1 intake's own cited
//! `class_spell.hybrid.ranger.unsupported` id (verified below to be a
//! `ComputationDiagnostic`, never reachable by any filter regardless of
//! prefix, mirroring Cleric's/Druid's/Sorcerer's own diagnostic-vs-
//! explanation findings for their own `.unsupported`-suffixed ids).
//!
//! `every_changed_ranger_explanation_produces_a_grant` walks the FULL
//! from/to explanation diff generically (by id, not a hand-picked
//! allowlist) starting at `from_level = 0` (uncovering exactly the missed
//! 0 -> 1 recognition transition Wizard's own bug hid in) through the full
//! supported level range, and asserts every changed, filter-admitted id
//! surfaces a matching `Grant`. Confirmed to fail (RED) against a
//! deliberately reverted copy of `ranger.rs` (the pre-fix filter shape)
//! before this cycle's fix, and to pass (GREEN) against the real,
//! fixed module.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState,
};
use codex::rules_core::level_up::ranger::compute_ranger_level_up_grants;
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;

const RANGER_RECOGNITION_ID: &str = "class_chassis.hybrid_baseline.ranger";
const RANGER_EXPLANATION_PREFIX: &str = "class_chassis.ranger.";
const RANGER_FEATURE_PREFIX: &str = "class_feature.ranger.";
const HYBRID_FEATURE_UNSUPPORTED_DIAGNOSTIC_ID: &str = "class_feature.hybrid.ranger.unsupported";
const HYBRID_SPELL_UNSUPPORTED_DIAGNOSTIC_ID: &str = "class_spell.hybrid.ranger.unsupported";

/// Deterministic single-class Human Ranger input at `level` (canonical
/// ability-score spread mirroring `tests/sd20_levelup_ranger.rs`'s own
/// fixture, minus the class-feature choice selections that module's wider
/// sweep needs -- this file only reads chassis-shaped explanation ids, none
/// of which are choice-gated). `level: 0` is a deliberately out-of-range
/// probe value (mirrors `ranger_chassis_explanations`'s own probe-at-
/// level-0 construction for the very first level transition) --
/// `explain_ranger_level1_chassis_and_class_feature_separation`'s own
/// `supported_ranger_level` gate rejects it, producing zero explanations
/// from that function, which is exactly the "before" state this file's
/// 0 -> 1 transition exercises.
fn ranger_at_level(level: u8) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd25_ranger_residue_audit".to_string()),
        source_package_id: "sd25_ranger_residue_audit".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:ranger".to_string(),
                level,
            }],
            ability_scores: AbilityScores {
                strength: 16,
                dexterity: 14,
                constitution: 14,
                intelligence: 10,
                wisdom: 12,
                charisma: 8,
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

fn is_ranger_pillar_id(id: &str) -> bool {
    id == RANGER_RECOGNITION_ID
        || id.starts_with(RANGER_EXPLANATION_PREFIX)
        || id.starts_with(RANGER_FEATURE_PREFIX)
}

// ----- Positive proof: the filter drops nothing real, at any level transition -----

#[test]
fn every_changed_ranger_explanation_produces_a_grant() {
    // Starts at from_level = 0 (unlike a supported-range-only loop) so the
    // 0 -> 1 initial-class-entry transition -- exactly where Ranger's
    // hybrid-baseline recognition explanation changes `None -> Some(0)` --
    // is exercised, mirroring `tests/sd25_bard_level_up_explanation_coverage.rs`'s
    // own loop shape.
    for from_level in 0..20u8 {
        let to_level = from_level + 1;
        let character = ranger_at_level(to_level);

        let from_computation = compute_pilot_base_chassis(&ranger_at_level(from_level));
        let to_computation = compute_pilot_base_chassis(&character);
        let plan = compute_ranger_level_up_grants(&character, from_level, to_level);

        for to_explanation in &to_computation.explanations {
            if !is_ranger_pillar_id(&to_explanation.id) {
                // Not a Ranger-owned pillar -- out of this module's own
                // scope, exactly as `ranger.rs`'s own filter documents.
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
                // different column_key by `append_class_table_grants`.
                || matches!(
                    to_explanation.id.as_str(),
                    "class_chassis.ranger.base_attack_bonus"
                        | "class_chassis.ranger.base_save.fortitude"
                        | "class_chassis.ranger.base_save.reflex"
                        | "class_chassis.ranger.base_save.will"
                );

            assert!(
                has_matching_grant,
                "ranger level {from_level} -> {to_level}: explanation '{}' changed from {:?} to \
                 {} but no matching Grant was found in the LevelUpPlan (the exact Wizard SD-24 \
                 bug shape -- a real grounded fact silently dropped by the explanation-id \
                 filter). Grants present: {:#?}",
                to_explanation.id,
                from_value,
                to_explanation.value,
                plan.automatic_features
            );
        }
    }
}

/// Names the recognition record explicitly by id, so a future accidental
/// narrowing of the whitelist is caught by name too, mirroring
/// `tests/sd24_wizard_level_up_spell_coverage.rs`'s own named-id assertions
/// and `tests/sd25_bard_level_up_explanation_coverage.rs`'s analogous test.
#[test]
fn ranger_recognition_explanation_produces_a_grant_on_first_level() {
    let character = ranger_at_level(1);
    let plan = compute_ranger_level_up_grants(&character, 0, 1);
    assert!(
        plan.automatic_features
            .iter()
            .any(|grant| grant.source_table.column_key == RANGER_RECOGNITION_ID),
        "the ranger 0 -> 1 LevelUpPlan must carry a Grant for '{RANGER_RECOGNITION_ID}' (the \
         hybrid-baseline recognition record pilot_compute.rs grounds on initial class entry); \
         grants present: {:#?}",
        plan.automatic_features
    );
}

/// Names the newly-opening partial-caster spellcasting-burden family
/// explicitly (ranger level 3 -> 4, where `pilot_compute.rs`'s
/// `RANGER_FIRST_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL` gate first opens), so a
/// future accidental narrowing of the `"class_chassis.ranger."` prefix
/// (the literal `class_spell.ranger.*`-shaped bug the 7.1 intake's blanket
/// characterization suspected) is caught by name, not just by the
/// generalized sweep above.
#[test]
fn ranger_partial_caster_family_newly_grants_at_the_level_4_spell_access_gate() {
    let character = ranger_at_level(4);
    let plan = compute_ranger_level_up_grants(&character, 3, 4);

    let has_grant_from_id = |id: &str| {
        plan.automatic_features
            .iter()
            .any(|grant| grant.source_table.column_key == id)
    };

    assert!(
        has_grant_from_id("class_chassis.ranger.partial_caster.effective_caster_level"),
        "expected a grant for the effective caster level rising 0 -> 1 at ranger level 4, got: \
         {:#?}",
        plan.automatic_features
    );
    assert!(
        has_grant_from_id("class_chassis.ranger.partial_caster.spell_level_access"),
        "expected a grant for spell-level access rising 0 -> 1 at ranger level 4, got: {:#?}",
        plan.automatic_features
    );
    assert!(
        has_grant_from_id(
            "class_chassis.ranger.partial_caster.base_spells_per_day.spell_level_1"
        ),
        "expected a grant for the newly-accessible spell level 1 base spells per day (a genuine \
         0 -> present record) at ranger level 4, got: {:#?}",
        plan.automatic_features
    );
    assert!(
        has_grant_from_id("class_chassis.ranger.partial_caster.spell_save_dc.spell_level_1"),
        "expected a grant for the newly-accessible spell level 1 spell save DC at ranger level \
         4, got: {:#?}",
        plan.automatic_features
    );
}

// ----- Verified negative finding: both hybrid "unsupported" diagnostics are correctly excluded -----

#[test]
fn hybrid_feature_unsupported_diagnostic_is_retired() {
    // `class_feature.hybrid.ranger.unsupported` used to fire unconditionally at
    // ranger level 1, flatly claiming favored enemy / combat style / tracking were
    // unimplemented. It was retired because the per-class decomposition
    // (`explain_ranger_level1_chassis_and_class_feature_separation`), dispatched
    // unconditionally on the exact same input, grounds Track and the Favored Enemy
    // flat surface for real -- making the blanket "not implemented" claim false,
    // not just redundant. See `tests/hybrid_diagnostic_grounded_contradiction.rs`
    // for the direct proof.
    let character = ranger_at_level(1);
    let computation = compute_pilot_base_chassis(&character);

    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == HYBRID_FEATURE_UNSUPPORTED_DIAGNOSTIC_ID),
        "the retired '{HYBRID_FEATURE_UNSUPPORTED_DIAGNOSTIC_ID}' diagnostic must not \
         reappear: {:?}",
        computation.diagnostics
    );

    let plan = compute_ranger_level_up_grants(&character, 0, 1);
    assert!(
        !plan
            .automatic_features
            .iter()
            .any(|grant| grant.source_table.column_key == HYBRID_FEATURE_UNSUPPORTED_DIAGNOSTIC_ID),
        "the ranger 0 -> 1 LevelUpPlan must never fabricate a Grant from the retired \
         '{HYBRID_FEATURE_UNSUPPORTED_DIAGNOSTIC_ID}': {:#?}",
        plan.automatic_features
    );
}

#[test]
fn hybrid_spell_unsupported_diagnostic_never_leaks_into_explanations_or_grants() {
    let character = ranger_at_level(1);
    let computation = compute_pilot_base_chassis(&character);

    // v0.6 alpha swarm (2026-07-28): this blanket diagnostic is now retired --
    // Rangers have no `CAST:` row in `cr_classes.lst` before class level 4, so a
    // level-1 Ranger having no spell posture is a satisfied condition, not a gap.
    // The leak guard below is kept and strengthened: the id must not appear as a
    // diagnostic, an explanation, OR a fabricated grant.
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == HYBRID_SPELL_UNSUPPORTED_DIAGNOSTIC_ID),
        "the retired hybrid spell-burden blocker must not reappear at ranger level 1: {:?}",
        computation.diagnostics
    );
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == HYBRID_SPELL_UNSUPPORTED_DIAGNOSTIC_ID),
        "the hybrid spell-burden diagnostic id must never appear in .explanations at ranger \
         level 1"
    );

    let plan = compute_ranger_level_up_grants(&character, 0, 1);
    assert!(
        !plan
            .automatic_features
            .iter()
            .any(|grant| grant.source_table.column_key == HYBRID_SPELL_UNSUPPORTED_DIAGNOSTIC_ID),
        "the LevelUpPlan for ranger 0 -> 1 must never fabricate a Grant from the hybrid \
         spell-burden claim-blocking diagnostic: {:#?}",
        plan.automatic_features
    );
}
