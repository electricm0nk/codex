//! SD-20 Epic 7 (Level Up grant model): Bard — second core class per
//! Step 2's stated order (`scope-draft.md` §1.7, `technical-design.md`
//! §2.6), after Barbarian (`8813eb8`).
//!
//! Mirrors `barbarian.rs`'s exact composition pattern: composes with two
//! already-grounded, already-landed sources rather than re-deriving
//! either.
//!
//! 1. `rules_tables::crb::class_tables::class_tables()` (SD-19's
//!    foundation slice) for the class-generic BAB/save progression — the
//!    class-generic pillars this table already carries.
//! 2. `pilot_compute::compute_pilot_base_chassis`'s own
//!    `explain_bard_level1_spell_baseline` output (SD13/SD18, read
//!    read-only via the public `compute_pilot_base_chassis` seam and its
//!    `explanations` field — this module never touches `pilot_compute.rs`
//!    itself, staying inside Epic 7's file-touch partition) for the
//!    class-specific pillars `class_tables.rs` deliberately does not
//!    carry: Bardic Knowledge, the Bardic Performance rounds-per-day
//!    budget, the Inspire Courage flat magnitude, the Fascinate flat
//!    Will-save DC and affected-creature-count formulas, Well-Versed,
//!    Inspire Competence, Jack-of-All-Trades, Lore Master, Soothing
//!    Performance, Frightening Tune's DC, Versatile Performance choice
//!    slots (named-but-unproven upstream, so never surfaced here either),
//!    Inspire Heroics' flat magnitudes, and Deadly Performance (the
//!    20th-level Bard class capstone).
//!
//! A level transition's automatic feature grants are computed as a
//! **diff** between the chassis snapshot at `from_level` and at
//! `to_level` (both computed against the same character, varying only the
//! class-level number), not by re-deriving PF1's own bard
//! class-progression rules a second time — identical algorithm to
//! `barbarian.rs`'s `append_class_feature_grants`, parameterized to the
//! Bard's own explanation-id namespace (`class_chassis.bard.*` /
//! `class_feature.bard.*`) and its own resource pool
//! (`bardic_performance_rounds_per_day` in place of
//! `rage_rounds_per_day`). Two signals drive the diff, exactly as they do
//! for Barbarian:
//!
//! - **Value change** (e.g. base attack bonus 0 -> 1, Fascinate DC 12 ->
//!   13): catches every magnitude-rising pillar.
//! - **Grant-state change** for level-gated features whose explanation
//!   record is either absent entirely below the gate (Deadly Performance,
//!   which `explain_bard_level1_spell_baseline` pushes no record for at
//!   all below bard level 20) or present with an explicit "correctly
//!   absent" marker text below the gate (Well-Versed, Inspire Competence):
//!   both shapes are handled identically by treating a missing
//!   `from_explanations` match the same as an explicit absence marker (see
//!   `is_absent_marker` below, unchanged from `barbarian.rs`).
//!
//! `pick_from_lists` stays empty: no PF1 Core Rulebook Bard pick-list
//! feature (e.g. a spells-known selection, or any open-ended choice) has
//! a real candidate catalog anywhere in `rules_tables::crb` to enumerate
//! from — the identical "no catalog to enumerate" boundary `barbarian.rs`
//! documented for the Rage Power list, here scoped to Bard's own
//! choice-list surface. This is a documented, bounded scope note, not a
//! blocker on this cycle's `LevelUpPlan`: every other field lands for
//! real.

use crate::rules_core::character_input::{CharacterClassLevel, CharacterInput};
use crate::rules_core::level_up::{Grant, GrantEffect, LevelUpPlan, ResourcePoolDelta};
use crate::rules_core::pilot_compute::{compute_pilot_base_chassis, ComputationExplanation};
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::class_tables::{class_tables, ClassId, ClassTableRow};
use crate::rules_core::rules_tables::RuleSetId;

const BARD_CLASS_ID: &str = "class:bard";
const HUMAN_RACE_ID: &str = "race:human";
/// SD-25 Epic 7 (criterion 7.6 per-class residue audit) finding: the exact
/// Wizard SD-24 bug shape (see `wizard.rs`'s own `WIZARD_RECOGNITION_ID` doc
/// comment) reproduced here. `pilot_compute.rs`'s
/// `explain_bard_level1_spell_baseline` pushes this id as a bounded +0
/// recognition record onto `.explanations`, but its own second segment
/// (`spell_baseline`) never matched either of this module's two admitted
/// prefixes (`"class_chassis.bard."` / `"class_feature.bard."`), so it was
/// silently dropped from the `LevelUpPlan` even on the very first Bard
/// level (0 -> 1). Added by this audit cycle, mirroring `wizard.rs`'s
/// `WIZARD_RECOGNITION_ID` and `sorcerer.rs`'s `SORCERER_RECOGNITION_ID`,
/// both of which already carry the identical whitelist entry.
const BARD_RECOGNITION_ID: &str = "class_chassis.spell_baseline.bard";
/// PF1 Core Rulebook Bard capstone: Deadly Performance, granted at 20th
/// level (verified against `pilot_compute.rs`'s own
/// `BARD_DEADLY_PERFORMANCE_LEVEL` gate and its doc comment naming
/// "Deadly Performance" as the class capstone — that constant is private
/// to `pilot_compute.rs`, so this mirrors its already-verified value
/// rather than importing it, the same boundary `barbarian.rs`'s own
/// `BARBARIAN_CAPSTONE_LEVEL` respects).
const BARD_CAPSTONE_LEVEL: u8 = 20;

const BARDIC_PERFORMANCE_ROUNDS_PER_DAY_EXPLANATION_ID: &str =
    "class_chassis.bard.bardic_performance_rounds_per_day";

/// Explanation ids that `append_class_table_grants` already covers from
/// `class_tables()` (the more authoritative, class-generic source per
/// this module's own doc comment). `pilot_compute.rs`'s chassis also
/// happens to carry standalone base-attack/base-save explanation records
/// for the same facts (grounded independently, pre-dating
/// `class_tables.rs`); skipping them here avoids reporting the identical
/// fact twice under two different names/provenances.
const CLASS_TABLE_COVERED_EXPLANATION_IDS: [&str; 4] = [
    "class_chassis.bard.base_attack_bonus",
    "class_chassis.bard.base_save.fortitude",
    "class_chassis.bard.base_save.reflex",
    "class_chassis.bard.base_save.will",
];

/// Composes a Bard `LevelUpPlan` for the transition from `from_level` to
/// `to_level`. Bounded to single-class Human Bard inputs, mirroring
/// `pilot_compute.rs`'s own `supported_bard_level` gate (the source this
/// module reads from carries the identical bound, so widening this gate
/// independently would only ever surface an empty chassis snapshot, not
/// real data).
pub fn compute_bard_level_up_grants(
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) -> LevelUpPlan {
    let mut plan = LevelUpPlan::default();

    let is_single_class_human_bard = character.chosen.race_id == HUMAN_RACE_ID
        && matches!(
            character.chosen.class_levels.as_slice(),
            [class_level] if class_level.class_id == BARD_CLASS_ID
        );
    if !is_single_class_human_bard {
        return plan;
    }

    append_class_table_grants(&mut plan, from_level, to_level);
    append_class_feature_grants(&mut plan, character, from_level, to_level);

    plan.capstone_threshold = to_level >= BARD_CAPSTONE_LEVEL;

    plan
}

fn class_table_row(level: u8) -> Option<ClassTableRow> {
    class_tables()
        .into_iter()
        .find(|row| row.class_id == ClassId::Bard && row.level == level)
}

/// Grants sourced from `rules_tables::crb::class_tables::class_tables()`
/// — the class-generic BAB/save progression pillars.
fn append_class_table_grants(plan: &mut LevelUpPlan, from_level: u8, to_level: u8) {
    let Some(to_row) = class_table_row(to_level) else {
        return;
    };
    let from_row = class_table_row(from_level);

    let columns: [(&str, i16); 4] = [
        ("base_attack_bonus", to_row.base_attack_bonus),
        ("fort_save", to_row.fort_save),
        ("ref_save", to_row.ref_save),
        ("will_save", to_row.will_save),
    ];
    let from_columns: Option<[i16; 4]> = from_row.map(|row| {
        [
            row.base_attack_bonus,
            row.fort_save,
            row.ref_save,
            row.will_save,
        ]
    });

    for (index, (column_key, to_value)) in columns.into_iter().enumerate() {
        let from_value = from_columns.map(|values| values[index]);
        if from_value == Some(to_value) {
            continue;
        }
        plan.automatic_features.push(Grant {
            name: format!("Bard {column_key} at level {to_level}"),
            source_table: TableCellRef {
                rule_set: RuleSetId::Crb,
                table: "class_tables".to_owned(),
                row_key: format!("bard:{to_level}"),
                column_key: column_key.to_owned(),
            },
            effects: vec![GrantEffect {
                description: format!(
                    "{column_key} rises to {to_value} at bard level {to_level}"
                ),
                value: to_value,
            }],
        });
    }
}

/// Reads `pilot_compute::compute_pilot_base_chassis`'s bard-specific
/// explanations for a synthetic single-class Bard input at `level`,
/// keeping every other chosen field (race, ability scores) from
/// `character` unchanged. Read-only: `pilot_compute.rs` itself is never
/// touched or re-derived, only its already-public output is read twice
/// (once per level) to diff.
fn bard_chassis_explanations(
    character: &CharacterInput,
    level: u8,
) -> Vec<ComputationExplanation> {
    let mut probe = character.clone();
    probe.chosen.class_levels = vec![CharacterClassLevel {
        class_id: BARD_CLASS_ID.to_owned(),
        level,
    }];
    compute_pilot_base_chassis(&probe).explanations
}

/// `explain_bard_level1_spell_baseline`'s level-gated explanations
/// (Well-Versed, Inspire Competence) word their "not yet granted" branch
/// with this exact marker phrase. Reading it is reading an
/// already-computed, already-grounded fact — not re-deriving the level
/// gate that produced it. Unchanged from `barbarian.rs`'s identical
/// helper.
fn is_absent_marker(detail: &str) -> bool {
    detail.contains("correctly absent")
}

/// Grants (and the one resource-pool change) sourced from
/// `pilot_compute::compute_pilot_base_chassis`'s bard-specific
/// explanations — the class-specific pillars `class_tables.rs` does not
/// carry.
fn append_class_feature_grants(
    plan: &mut LevelUpPlan,
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) {
    let from_explanations = bard_chassis_explanations(character, from_level);
    let to_explanations = bard_chassis_explanations(character, to_level);

    for to_explanation in &to_explanations {
        let is_bard_class_feature_id = to_explanation.id == BARD_RECOGNITION_ID
            || to_explanation.id.starts_with("class_chassis.bard.")
            || to_explanation.id.starts_with("class_feature.bard.");
        let is_covered_elsewhere = to_explanation.id
            == BARDIC_PERFORMANCE_ROUNDS_PER_DAY_EXPLANATION_ID
            || CLASS_TABLE_COVERED_EXPLANATION_IDS.contains(&to_explanation.id.as_str());
        if !is_bard_class_feature_id || is_covered_elsewhere {
            // Bardic performance rounds per day is a resource pool
            // (handled separately below); the class-table-covered ids
            // are already granted by `append_class_table_grants`.
            continue;
        }

        let from_match = from_explanations
            .iter()
            .find(|explanation| explanation.id == to_explanation.id);
        let from_granted = from_match.map(|explanation| !is_absent_marker(&explanation.detail));
        let to_granted = !is_absent_marker(&to_explanation.detail);
        let value_changed = from_match.map(|explanation| explanation.value) != Some(to_explanation.value);
        let newly_granted = from_granted != Some(true) && to_granted;

        if !value_changed && !newly_granted {
            continue;
        }

        plan.automatic_features.push(Grant {
            name: friendly_name(&to_explanation.id),
            source_table: TableCellRef {
                rule_set: RuleSetId::Crb,
                table: "pilot_compute::explain_bard_level1_spell_baseline".to_owned(),
                row_key: format!("bard:{to_level}"),
                column_key: to_explanation.id.clone(),
            },
            effects: vec![GrantEffect {
                description: to_explanation.detail.clone(),
                value: to_explanation.value,
            }],
        });
    }

    let from_rounds = from_explanations
        .iter()
        .find(|explanation| explanation.id == BARDIC_PERFORMANCE_ROUNDS_PER_DAY_EXPLANATION_ID)
        .map(|explanation| explanation.value);
    let to_rounds = to_explanations
        .iter()
        .find(|explanation| explanation.id == BARDIC_PERFORMANCE_ROUNDS_PER_DAY_EXPLANATION_ID)
        .map(|explanation| explanation.value);
    if let (Some(from_value), Some(to_value)) = (from_rounds, to_rounds)
        && from_value != to_value
    {
        plan.resource_pool_change.pools.push(ResourcePoolDelta {
            pool_id: "bardic_performance_rounds_per_day".to_owned(),
            from_value,
            to_value,
            source_table: TableCellRef {
                rule_set: RuleSetId::Crb,
                table: "pilot_compute::explain_bard_level1_spell_baseline".to_owned(),
                row_key: format!("bard:{to_level}"),
                column_key: BARDIC_PERFORMANCE_ROUNDS_PER_DAY_EXPLANATION_ID.to_owned(),
            },
        });
    }
}

/// Derives a human-readable grant name mechanically from a grounded
/// explanation id (e.g. `"class_feature.bard.well_versed"` -> `"well
/// versed"`) — never a hand-typed label naming CRB rule text this module
/// has not itself verified.
fn friendly_name(id: &str) -> String {
    id.trim_start_matches("class_chassis.bard.")
        .trim_start_matches("class_feature.bard.")
        .trim_start_matches("class_chassis.spell_baseline.")
        .replace(['_', '.'], " ")
}
