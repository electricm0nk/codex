//! SD-20 Epic 7 (Level Up grant model): Cleric — third core class per
//! Step 2's stated order (barbarian, bard, cleric, druid, fighter, monk,
//! paladin, ranger, rogue, sorcerer, wizard; `scope-draft.md` §1.7,
//! `technical-design.md` §2.6).
//!
//! **Deviation from Barbarian's own composition precedent, documented in
//! full**: Barbarian's `LevelUpPlan` (`level_up/barbarian.rs`) composes
//! its base-attack/base-save pillars from
//! `rules_tables::crb::class_tables::class_tables()` (SD-19's
//! class-generic BAB/save progression table), calling it "the more
//! authoritative, class-generic source". This module does NOT do the
//! same for Cleric, because `class_tables()`'s `CLASS_META` row for
//! `ClassId::Cleric` encodes `good_saves.fortitude: false` — but the PF1
//! Core Rulebook Cleric class table's good saves are Fortitude AND Will
//! (poor Reflex only; verified against d20pfsrd and legacy.aonprd.com),
//! exactly as this codebase's own already-landed
//! `pilot_compute.rs::explain_cleric_level1_spell_baseline` already
//! independently grounds (that function primary-source-verified this
//! fact against the same two sources before ever landing — see its own
//! doc comment). `class_tables()`'s `ClassId::Druid` row carries the
//! identical defect (also missing `fortitude: true`). Composing this
//! cycle's Cleric `LevelUpPlan` with the buggy `class_tables()` row would
//! silently fabricate an incorrect Fortitude-save grant (reporting it as
//! an unchanging poor save), which this codebase's no-fabrication rule
//! (`AGENTS.md`) does not permit. Fixing `class_tables.rs` itself is out
//! of this cycle's file-touch partition (SD-20's cycles may not touch
//! `rules_tables/crb/*`; SD-19 owns the table store) — so instead, EVERY
//! automatic-feature pillar for Cleric (base attack bonus, all three base
//! saves, Channel Energy's die count and uses-per-day, the flat domain
//! spell slot count, and the Good/Healing domain-power magnitudes) is
//! composed from `pilot_compute::compute_pilot_base_chassis`'s own
//! already-grounded, already-primary-source-verified Cleric explanations
//! instead — a single, internally consistent, already-verified source,
//! mirroring exactly how Barbarian sources ITS OWN class-specific pillars
//! (the ones `class_tables()` does not carry at all) from the identical
//! seam. `rules_tables::crb::class_tables` is not imported by this file
//! at all. A future SD-19 cycle should fix `class_tables.rs`'s Cleric
//! (and Druid) `good_saves` records; this cycle's own `LevelUpPlan` does
//! not depend on that fix landing.
//!
//! A level transition's automatic feature grants are computed as a
//! **diff** between the chassis snapshot at `from_level` and at
//! `to_level` (both computed against the same character, varying only
//! the class-level number), not by re-deriving PF1's own Cleric
//! class-progression rules a second time — the identical technique
//! Barbarian's own `append_class_feature_grants` uses. Unlike Barbarian,
//! Cleric has no level-gated on/off "identity" features (no Uncanny
//! Dodge-shaped absent/granted marker text): every one of
//! `explain_cleric_level1_spell_baseline`'s explanations is emitted at
//! every supported level (1-20) uniformly, so a pure value-change diff
//! (no "newly granted" branch) suffices.
//!
//! `resource_pool_change` stays genuinely empty for every Cleric
//! level-up transition: Cleric's three daily-use pools (Channel Energy
//! uses/day, Touch of Good uses/day, Rebuke Death uses/day) are all flat
//! `3 + ability modifier` formulas with no level term at all, so none of
//! them ever change size between `from_level` and `to_level` for a fixed
//! character — unlike Barbarian's rage-rounds-per-day pool, which does
//! scale with level. This is a factual property of the grounded formulas,
//! not an unfilled scope note.
//!
//! Cleric has no distinct named capstone class feature at 20th level (the
//! PF1 Core Rulebook Cleric class table's level-20 "Special" column is
//! genuinely blank — verified independently against d20pfsrd and the
//! Archives of Nethys aonprd.com mirror, per `pilot_compute.rs`'s own doc
//! comment on `MAX_SUPPORTED_CLERIC_LEVEL`), unlike Barbarian's Mighty
//! Rage. `capstone_threshold` still flags `to_level >= 20` (PF1's
//! universal character-level cap, which every class reaches even without
//! a distinctly named ability there), but no separate named grant is
//! fabricated for it — the ordinary base-attack / base-save /
//! Touch-of-Good pillars simply keep rising through the same generic diff
//! used at every other level.
//!
//! `pick_from_lists` stays empty: Cleric's domain spells (which would
//! fill the grounded domain spell slot count) have no domain-spell-list
//! candidate catalog anywhere in `rules_tables::crb` to enumerate real
//! candidates from — the identical "no catalog to enumerate" boundary
//! Barbarian's Rage Power list hit, here scoped to Cleric's own
//! choice-list feature. A documented, bounded scope note, not a blocker
//! on this cycle's `LevelUpPlan`: every other field lands for real.

use crate::rules_core::character_input::{CharacterClassLevel, CharacterInput};
use crate::rules_core::level_up::{Grant, GrantEffect, LevelUpPlan};
use crate::rules_core::pilot_compute::{compute_pilot_base_chassis, ComputationExplanation};
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::RuleSetId;

const CLERIC_CLASS_ID: &str = "class:cleric";
const HUMAN_RACE_ID: &str = "race:human";
/// PF1's universal character-level cap. Cleric has no distinct named
/// capstone feature here (see this module's own doc comment) — this
/// constant only gates the boolean `capstone_threshold` flag, not a
/// fabricated grant.
const CLERIC_CHARACTER_LEVEL_CAP: u8 = 20;

/// Every explanation id `pilot_compute.rs::explain_cleric_level1_spell_baseline`
/// emits is either this exact recognition id or carries this prefix. Used
/// to scope the diff to Cleric's own pillars only (never another class's
/// or the class-independent race/ability explanations that also appear
/// in the same chassis snapshot).
const CLERIC_RECOGNITION_ID: &str = "class_chassis.spell_baseline.cleric";
const CLERIC_EXPLANATION_PREFIX: &str = "class_chassis.cleric.";

/// Composes a Cleric `LevelUpPlan` for the transition from `from_level`
/// to `to_level`. Bounded to single-class Human Cleric inputs, mirroring
/// `pilot_compute.rs`'s own `supported_cleric_level` gate (the source
/// this module reads from carries the identical bound, so widening this
/// gate independently would only ever surface an empty chassis snapshot,
/// not real data).
pub fn compute_cleric_level_up_grants(
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) -> LevelUpPlan {
    let mut plan = LevelUpPlan::default();

    let is_single_class_human_cleric = character.chosen.race_id == HUMAN_RACE_ID
        && matches!(
            character.chosen.class_levels.as_slice(),
            [class_level] if class_level.class_id == CLERIC_CLASS_ID
        );
    if !is_single_class_human_cleric {
        return plan;
    }

    append_class_feature_grants(&mut plan, character, from_level, to_level);

    plan.capstone_threshold = to_level >= CLERIC_CHARACTER_LEVEL_CAP;

    plan
}

/// Reads `pilot_compute::compute_pilot_base_chassis`'s Cleric-specific
/// explanations for a synthetic single-class Cleric input at `level`,
/// keeping every other chosen field (race, ability scores, domain
/// choices) from `character` unchanged. Read-only: `pilot_compute.rs`
/// itself is never touched or re-derived, only its already-public output
/// is read twice (once per level) to diff — the identical technique
/// Barbarian's `barbarian_chassis_explanations` uses.
fn cleric_chassis_explanations(
    character: &CharacterInput,
    level: u8,
) -> Vec<ComputationExplanation> {
    let mut probe = character.clone();
    probe.chosen.class_levels = vec![CharacterClassLevel {
        class_id: CLERIC_CLASS_ID.to_owned(),
        level,
    }];
    compute_pilot_base_chassis(&probe).explanations
}

/// Grants sourced from `pilot_compute::compute_pilot_base_chassis`'s
/// Cleric-specific explanations — every pillar this cycle grounds (base
/// attack bonus, all three base saves, Channel Energy, the domain spell
/// slot count, and the domain-power magnitudes), per this module's own
/// doc comment on why `class_tables()` is not used here. A pure
/// value-change diff: Cleric carries no level-gated on/off "identity"
/// feature whose value stays 0 whether granted or absent (unlike
/// Barbarian's Uncanny Dodge), so no separate "newly granted" branch is
/// needed.
fn append_class_feature_grants(
    plan: &mut LevelUpPlan,
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) {
    let from_explanations = cleric_chassis_explanations(character, from_level);
    let to_explanations = cleric_chassis_explanations(character, to_level);

    for to_explanation in &to_explanations {
        let is_cleric_pillar = to_explanation.id == CLERIC_RECOGNITION_ID
            || to_explanation.id.starts_with(CLERIC_EXPLANATION_PREFIX);
        if !is_cleric_pillar {
            continue;
        }

        let from_value = from_explanations
            .iter()
            .find(|explanation| explanation.id == to_explanation.id)
            .map(|explanation| explanation.value);
        if from_value == Some(to_explanation.value) {
            // Either genuinely unchanged (the common case for Cleric --
            // most pillars are flat ability-modifier formulas or share an
            // integer-division coincidence between adjacent levels), or
            // present-and-identical at both ends (the +0 recognition
            // record and the domain-choice seam, whenever domain
            // selections are present at both levels, which they always
            // are for a fixed character).
            continue;
        }

        plan.automatic_features.push(Grant {
            name: friendly_name(&to_explanation.id),
            source_table: TableCellRef {
                rule_set: RuleSetId::Crb,
                table: "pilot_compute::explain_cleric_level1_spell_baseline".to_owned(),
                row_key: format!("cleric:{to_level}"),
                column_key: to_explanation.id.clone(),
            },
            effects: vec![GrantEffect {
                description: to_explanation.detail.clone(),
                value: to_explanation.value,
            }],
        });
    }
}

/// Derives a human-readable grant name mechanically from a grounded
/// explanation id (e.g. `"class_chassis.cleric.channel_energy_dice"` ->
/// `"channel energy dice"`) — never a hand-typed label naming CRB rule
/// text this module has not itself verified. Mirrors Barbarian's own
/// `friendly_name` helper.
fn friendly_name(id: &str) -> String {
    id.trim_start_matches(CLERIC_EXPLANATION_PREFIX)
        .trim_start_matches("class_chassis.spell_baseline.")
        .replace(['_', '.'], " ")
}
