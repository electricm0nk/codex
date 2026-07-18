//! SD-20 Epic 7 (Level Up grant model): Rogue — ninth core class per
//! Step 2's stated order (barbarian, bard, cleric, druid, fighter, monk,
//! paladin already landed; `scope-draft.md` §1.7, `technical-design.md`
//! §2.6).
//!
//! Mirrors `barbarian.rs`'s exact composition pattern (NOT `druid.rs`'s/
//! `cleric.rs`'s deviation): composes with two already-grounded,
//! already-landed sources rather than re-deriving either.
//!
//! 1. `rules_tables::crb::class_tables::class_tables()` (SD-19's
//!    foundation slice) for the class-generic BAB/save progression.
//!    Per this cycle's own brief, Rogue's `CLASS_META` row was
//!    spot-checked against `pilot_compute.rs`'s own already-grounded
//!    `explain_rogue_level1_chassis` formulas BEFORE composing with it
//!    (the same discipline that caught the Cleric/Druid `good_saves.fortitude`
//!    defect, fixed at `28b0e88`): `class_tables.rs`'s Rogue row reads
//!    `bab: BabProgression::ThreeQuarter, good_saves: { fortitude: false,
//!    reflex: true, will: false }`. `pilot_compute.rs`'s own
//!    `explain_rogue_level1_chassis` computes `base_attack_bonus = level *
//!    3 / 4` (three-quarter BAB), `base_save.fortitude = level / 3` (poor),
//!    `base_save.reflex = level / 2 + 2` (good), `base_save.will = level /
//!    3` (poor) — and `class_tables.rs`'s own `base_attack_bonus` /
//!    `save_bonus` helper functions apply the identical formulas
//!    (`ThreeQuarter => (level * 3) / 4`; good `level / 2 + 2`, poor
//!    `level / 3`) to that exact `good_saves` row. The two sources agree at
//!    every level 1-20: **Rogue's `CLASS_META` row is CORRECT, no defect
//!    found** (unlike Cleric/Druid's now-fixed row).
//! 2. `pilot_compute::compute_pilot_base_chassis`'s own
//!    `explain_rogue_level1_chassis` output (SD13/SD18, read read-only via
//!    the public `compute_pilot_base_chassis` seam and its `explanations`
//!    field — this module never touches `pilot_compute.rs` itself, staying
//!    inside Epic 7's file-touch partition) for the class-specific pillars
//!    `class_tables.rs` deliberately does not carry: the Sneak Attack
//!    damage-die count, Trapfinding's flat numeric bonus, Evasion, Trap
//!    Sense, Uncanny Dodge, Improved Uncanny Dodge, the ten numbered Rogue
//!    Talent choice-slot recognitions, and Master Strike (the 20th-level
//!    capstone).
//!
//! A level transition's automatic feature grants are computed as a
//! **diff** between the chassis snapshot at `from_level` and at
//! `to_level` (both computed against the same character, varying only the
//! class-level number), not by re-deriving PF1's own rogue
//! class-progression rules a second time — identical algorithm to
//! `barbarian.rs`'s `append_class_feature_grants`, parameterized to the
//! Rogue's own explanation-id namespace (`class_chassis.rogue.*` /
//! `class_feature.rogue.*`). Two signals drive the diff, exactly as they
//! do for Barbarian:
//!
//! - **Value change** (e.g. base attack bonus 0 -> 1, Trapfinding +1 ->
//!   +2, Sneak Attack 1d6 -> 2d6): catches every magnitude-rising pillar.
//! - **Grant-state change** for level-gated features whose explanation
//!   record is present with an explicit "correctly absent" marker text
//!   below the gate (Evasion, Trap Sense, Uncanny Dodge, Improved Uncanny
//!   Dodge, Master Strike) — `pilot_compute.rs`'s own
//!   `explain_rogue_level1_chassis` words every one of these identically
//!   to Barbarian's own level-gated features, so this module reuses
//!   `is_absent_marker` unchanged (NOT Fighter's simplified "absent
//!   entirely below the gate" shape).
//!
//! The Rogue Talent choice-slot recognitions (`class_chassis.rogue.talent_choice`
//! through `.talent_10_choice`) only ever appear in
//! `explain_rogue_level1_chassis`'s output when the character's
//! `selected_choices` supplies a matching `choice:rogue_talent[_N]`
//! selection AND the level gate for that numbered slot is met; this
//! module's diff loop handles them with zero special-casing (same
//! `class_chassis.rogue.` prefix match as every other pillar): a slot
//! newly present in `to_explanations` but absent from `from_explanations`
//! is a newly-granted record, exactly like Master Strike's own "entirely
//! absent below gate, present at gate" shape.
//!
//! `pick_from_lists` stays empty: no PF1 Core Rulebook Rogue Talent
//! candidate catalog exists anywhere in `rules_tables::crb` to enumerate
//! from — the identical "no catalog to enumerate" boundary `barbarian.rs`
//! documented for the Rage Power list, here scoped to Rogue's own
//! choice-list surface. `resource_pool_change` stays empty: unlike
//! Barbarian's rage rounds/day, Bard's bardic performance rounds/day, or
//! Monk's ki pool, no PF1 Core Rulebook Rogue class feature is a
//! named per-day resource pool — `explain_rogue_level1_chassis` grounds no
//! such record, so there is nothing to compose here. These are documented,
//! bounded scope notes, not blockers on this cycle's `LevelUpPlan`: every
//! other field lands for real.

use crate::rules_core::character_input::{CharacterClassLevel, CharacterInput};
use crate::rules_core::level_up::{Grant, GrantEffect, LevelUpPlan};
use crate::rules_core::pilot_compute::{compute_pilot_base_chassis, ComputationExplanation};
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::class_tables::{class_tables, ClassId, ClassTableRow};
use crate::rules_core::rules_tables::RuleSetId;

const ROGUE_CLASS_ID: &str = "class:rogue";
const HUMAN_RACE_ID: &str = "race:human";
/// PF1 Core Rulebook Rogue capstone: Master Strike, granted at 20th level
/// (verified against `pilot_compute.rs`'s own `ROGUE_MASTER_STRIKE_LEVEL`
/// gate and its doc comment naming "Master Strike" as the class capstone —
/// that constant is private to `pilot_compute.rs`, so this mirrors its
/// already-verified value rather than importing it, the same boundary
/// `barbarian.rs`'s own `BARBARIAN_CAPSTONE_LEVEL` respects).
const ROGUE_CAPSTONE_LEVEL: u8 = 20;

/// Explanation ids that `append_class_table_grants` already covers from
/// `class_tables()` (the more authoritative, class-generic source per
/// this module's own doc comment). `pilot_compute.rs`'s chassis also
/// happens to carry standalone base-attack/base-save explanation records
/// for the same facts (grounded independently, pre-dating
/// `class_tables.rs`); skipping them here avoids reporting the identical
/// fact twice under two different names/provenances.
const CLASS_TABLE_COVERED_EXPLANATION_IDS: [&str; 4] = [
    "class_chassis.rogue.base_attack_bonus",
    "class_chassis.rogue.base_save.fortitude",
    "class_chassis.rogue.base_save.reflex",
    "class_chassis.rogue.base_save.will",
];

/// Composes a Rogue `LevelUpPlan` for the transition from `from_level` to
/// `to_level`. Bounded to single-class Human Rogue inputs, mirroring
/// `pilot_compute.rs`'s own `supported_rogue_level` gate (the source this
/// module reads from carries the identical bound, so widening this gate
/// independently would only ever surface an empty chassis snapshot, not
/// real data).
pub fn compute_rogue_level_up_grants(
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) -> LevelUpPlan {
    let mut plan = LevelUpPlan::default();

    let is_single_class_human_rogue = character.chosen.race_id == HUMAN_RACE_ID
        && matches!(
            character.chosen.class_levels.as_slice(),
            [class_level] if class_level.class_id == ROGUE_CLASS_ID
        );
    if !is_single_class_human_rogue {
        return plan;
    }

    append_class_table_grants(&mut plan, from_level, to_level);
    append_class_feature_grants(&mut plan, character, from_level, to_level);

    plan.capstone_threshold = to_level >= ROGUE_CAPSTONE_LEVEL;

    plan
}

fn class_table_row(level: u8) -> Option<ClassTableRow> {
    class_tables()
        .into_iter()
        .find(|row| row.class_id == ClassId::Rogue && row.level == level)
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
            name: format!("Rogue {column_key} at level {to_level}"),
            source_table: TableCellRef {
                rule_set: RuleSetId::Crb,
                table: "class_tables".to_owned(),
                row_key: format!("rogue:{to_level}"),
                column_key: column_key.to_owned(),
            },
            effects: vec![GrantEffect {
                description: format!(
                    "{column_key} rises to {to_value} at rogue level {to_level}"
                ),
                value: to_value,
            }],
        });
    }
}

/// Reads `pilot_compute::compute_pilot_base_chassis`'s rogue-specific
/// explanations for a synthetic single-class Rogue input at `level`,
/// keeping every other chosen field (race, ability scores, selected
/// choices) from `character` unchanged. Read-only: `pilot_compute.rs`
/// itself is never touched or re-derived, only its already-public output
/// is read twice (once per level) to diff.
fn rogue_chassis_explanations(character: &CharacterInput, level: u8) -> Vec<ComputationExplanation> {
    let mut probe = character.clone();
    probe.chosen.class_levels = vec![CharacterClassLevel {
        class_id: ROGUE_CLASS_ID.to_owned(),
        level,
    }];
    compute_pilot_base_chassis(&probe).explanations
}

/// `explain_rogue_level1_chassis`'s own level-gated explanations (Evasion,
/// Trap Sense, Uncanny Dodge, Improved Uncanny Dodge, Master Strike)
/// always word their "not yet granted" branch with this exact marker
/// phrase. Reading it is reading an already-computed, already-grounded
/// fact — not re-deriving the level gate that produced it. Unchanged from
/// `barbarian.rs`'s identical helper.
fn is_absent_marker(detail: &str) -> bool {
    detail.contains("correctly absent")
}

/// Grants sourced from `pilot_compute::compute_pilot_base_chassis`'s
/// rogue-specific explanations — the class-specific pillars
/// `class_tables.rs` does not carry. No resource pool is composed here:
/// unlike Barbarian/Bard/Monk, no PF1 Core Rulebook Rogue class feature is
/// a named per-day resource pool.
fn append_class_feature_grants(
    plan: &mut LevelUpPlan,
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) {
    let from_explanations = rogue_chassis_explanations(character, from_level);
    let to_explanations = rogue_chassis_explanations(character, to_level);

    for to_explanation in &to_explanations {
        let is_rogue_class_feature_id = to_explanation.id.starts_with("class_chassis.rogue.")
            || to_explanation.id.starts_with("class_feature.rogue.");
        let is_covered_elsewhere =
            CLASS_TABLE_COVERED_EXPLANATION_IDS.contains(&to_explanation.id.as_str());
        if !is_rogue_class_feature_id || is_covered_elsewhere {
            // The class-table-covered ids are already granted by
            // `append_class_table_grants`.
            continue;
        }

        let from_match = from_explanations
            .iter()
            .find(|explanation| explanation.id == to_explanation.id);
        let from_granted = from_match.map(|explanation| !is_absent_marker(&explanation.detail));
        let to_granted = !is_absent_marker(&to_explanation.detail);
        let value_changed =
            from_match.map(|explanation| explanation.value) != Some(to_explanation.value);
        let newly_granted = from_granted != Some(true) && to_granted;

        if !value_changed && !newly_granted {
            continue;
        }

        plan.automatic_features.push(Grant {
            name: friendly_name(&to_explanation.id),
            source_table: TableCellRef {
                rule_set: RuleSetId::Crb,
                table: "pilot_compute::explain_rogue_level1_chassis".to_owned(),
                row_key: format!("rogue:{to_level}"),
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
/// explanation id (e.g. `"class_feature.rogue.uncanny_dodge"` -> `"uncanny
/// dodge"`) — never a hand-typed label naming CRB rule text this module
/// has not itself verified.
fn friendly_name(id: &str) -> String {
    id.trim_start_matches("class_chassis.rogue.")
        .trim_start_matches("class_feature.rogue.")
        .replace(['_', '.'], " ")
}
