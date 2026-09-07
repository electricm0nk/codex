//! SD-24 Epic 4 (Per-class audit, criterion 4.1): Wizard coverage audit.
//!
//! `src/rules_core/level_up/wizard.rs` composes two sources into a
//! `LevelUpPlan`: `rules_tables::crb::class_tables::class_tables()` (the
//! class-generic BAB/save pillars) and `pilot_compute::
//! compute_pilot_base_chassis`'s Wizard-specific explanations, filtered
//! to ids equal to `class_chassis.spell_baseline.wizard` or prefixed
//! `class_chassis.wizard.`.
//!
//! That filter silently drops every `class_spell.wizard.*` explanation
//! `pilot_compute.rs` grounds once a wizard's prepared-spellbook posture
//! is met (SD-21 E6b.2 `ground_wizard_prepared_spellbook`:
//! `class_spell.wizard.spellbook_contents`, `.daily_preparation`, and the
//! per-spell-level `.base_spells_per_day.spell_level_N` /
//! `.intelligence_bonus_spells_per_day.spell_level_N` /
//! `.total_spells_per_day.spell_level_N` records) — a class-feature
//! coverage gap this audit surfaces and remediates.
//!
//! Reproduces `tests/sd21_wizard_prepared_spellbook.rs`'s own populated
//! spellbook fixture (a Human Wizard, canonical Evocation specialization,
//! opposed Necromancy/Transmutation) but drives it through
//! `compute_wizard_level_up_grants` instead of `compute_pilot_base_chassis`
//! directly, to prove the *Level Up grant* surface — not just the base
//! chassis compute — carries these real spell-per-day facts.
//!
//! The from/to levels (2 -> 3) are chosen so the posture is UNMET at
//! `from_level` (the populated spellbook's one 2nd-level spell,
//! `evocation.2.scorching_ray`, targets a spell level not yet accessible
//! at wizard level 2 per `wizard_base_spells_per_day`) and MET at
//! `to_level` (wizard level 3 grants 2nd-level spell access) — so every
//! `class_spell.wizard.*` record is absent at `from_level` and present at
//! `to_level`, a clean "newly granted" transition per this module's own
//! diff idiom.

use codex::rules_core::character_input::{AcquisitionMode, CharacterInput, SpellSelection};
use codex::rules_core::level_up::wizard::compute_wizard_level_up_grants;
mod common;
use common::load;

const WIZARD_LEVEL_3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level3_sd13_deterministic_input.txt");

fn wizard_spell(spell_id: &str, mode: AcquisitionMode) -> SpellSelection {
    SpellSelection {
        spell_id: spell_id.to_string(),
        source_class_id: "class:wizard".to_string(),
        acquisition_mode: mode,
    }
}

/// Mirrors `tests/sd21_wizard_prepared_spellbook.rs`'s own
/// `wizard_level_3_with_populated_spellbook_and_daily_prep` fixture.
fn wizard_with_populated_spellbook_and_daily_prep() -> CharacterInput {
    let mut input = load(WIZARD_LEVEL_3_FIXTURE);
    input.chosen.spells_selected = vec![
        wizard_spell("evocation.1.magic_missile", AcquisitionMode::Known),
        wizard_spell("evocation.1.burning_hands", AcquisitionMode::Known),
        wizard_spell("evocation.2.scorching_ray", AcquisitionMode::Known),
        wizard_spell("evocation.1.magic_missile", AcquisitionMode::Prepared),
        wizard_spell("evocation.2.scorching_ray", AcquisitionMode::Prepared),
    ];
    input
}

#[test]
fn leveling_up_into_a_supported_spellbook_posture_grants_the_real_spell_facts() {
    let character = wizard_with_populated_spellbook_and_daily_prep();
    let plan = compute_wizard_level_up_grants(&character, 2, 3);

    let has_grant_from_id_suffix = |suffix: &str| {
        plan.automatic_features
            .iter()
            .any(|grant| grant.source_table.column_key.ends_with(suffix))
    };

    assert!(
        has_grant_from_id_suffix("class_spell.wizard.spellbook_contents"),
        "expected a grant sourced from class_spell.wizard.spellbook_contents when the wizard's \
         prepared-spellbook posture becomes met at to_level, got: {:#?}",
        plan.automatic_features
    );
    assert!(
        has_grant_from_id_suffix("class_spell.wizard.daily_preparation"),
        "expected a grant sourced from class_spell.wizard.daily_preparation, got: {:#?}",
        plan.automatic_features
    );
    assert!(
        has_grant_from_id_suffix("class_spell.wizard.total_spells_per_day.spell_level_2"),
        "expected a grant for the newly-accessible spell level 2 total spells per day, got: {:#?}",
        plan.automatic_features
    );
}
