//! SD-20 Epic 7 (Level Up grant model): Ranger — eighth core class per
//! Step 2's stated order (`scope-draft.md` §1.7, `technical-design.md`
//! §2.6), landed after barbarian, bard, cleric, druid, fighter, monk, and
//! paladin.
//!
//! Ranger composes with the identical two already-grounded, already-landed
//! sources every prior Epic 7 cycle used, mirroring `barbarian.rs`'s and
//! `fighter.rs`'s exact composition pattern rather than re-deriving either:
//!
//! 1. `rules_tables::crb::class_tables::class_tables()` (SD-19's foundation
//!    slice) for the class-generic BAB/save progression. Ranger's
//!    `CLASS_META` row was spot-checked against the PF1 Core Rulebook
//!    before composing with it (per this cycle's own brief, since the
//!    Cleric/Druid `good_saves.fortitude` row was found wrong and fixed in
//!    `28b0e88`): Ranger's row reads `good_saves { fortitude: true, reflex:
//!    true, will: false }` and `bab: Full` — independently verified
//!    against `pilot_compute.rs`'s own already-grounded,
//!    already-primary-source-verified
//!    `explain_ranger_level1_chassis_and_class_feature_separation` formulas
//!    (full BAB `classlevel`; good Fortitude/Reflex `classlevel/2+2`; poor
//!    Will `classlevel/3`; verified against the raw PF1 Core Rulebook
//!    Ranger class table level 1-5 rows, cross-checking the level 4/5
//!    base-attack-bonus values to disambiguate full BAB from 3/4 BAB) —
//!    CONFIRMED CORRECT, not a second latent defect. `class_tables()` is
//!    safe to compose with for Ranger.
//! 2. `pilot_compute::compute_pilot_base_chassis`'s own
//!    `explain_ranger_level1_chassis_and_class_feature_separation` output
//!    (SD13/SD18, read read-only via the public `compute_pilot_base_chassis`
//!    seam and its `explanations` field — this module never touches
//!    `pilot_compute.rs` itself, staying inside Epic 7's file-touch
//!    partition) for the class-specific pillars `class_tables.rs`
//!    deliberately does not carry: Track (a flat, rising Survival bonus),
//!    the Favored Enemy flat surface and its five level intervals (1st,
//!    5th, 10th, 15th, 20th ranger level), Combat Style Feat (the style
//!    choice and its five restricted/open-ended bonus-feat slots at 2nd,
//!    6th, 10th, 14th, and 18th level), Endurance, Favored Terrain and its
//!    four level intervals (3rd, 8th, 13th, 18th), Hunter's Bond, Woodland
//!    Stride, Swift Tracker, Evasion, Improved Evasion, Quarry, Improved
//!    Quarry, Camouflage, Hide in Plain Sight, and Master Hunter (the
//!    20th-level Ranger capstone). Wild Empathy is a genuine PF1 Core
//!    Rulebook Ranger 4th-level class feature but is NOT grounded here: no
//!    `class_chassis.ranger.wild_empathy` (or `class_feature.ranger.*`
//!    equivalent) explanation exists anywhere in
//!    `explain_ranger_level1_chassis_and_class_feature_separation` — only
//!    Druid's own Wild Empathy formula is grounded in this codebase
//!    (`explain_druid_...`). Composing a Ranger Wild Empathy grant would
//!    require fabricating a formula this cycle has not itself verified
//!    against `pilot_compute.rs`'s grounded surface, so it is deliberately
//!    left out — a documented, bounded scope note (like Barbarian's Rage
//!    Power list and Fighter's Bonus Feat candidate list), not a blocker on
//!    this cycle's `LevelUpPlan`: a future `pilot_compute.rs` slice that
//!    grounds Ranger's own Wild Empathy formula would let a later Epic 7
//!    cycle compose with it the same way.
//!
//! A level transition's automatic feature grants are computed as a
//! **diff** between the chassis snapshot at `from_level` and at `to_level`
//! (both computed against the same character, varying only the class-level
//! number), not by re-deriving PF1's own ranger class-progression rules a
//! second time — identical algorithm to `barbarian.rs`'s
//! `append_class_feature_grants`, parameterized to Ranger's own
//! explanation-id namespace (`class_chassis.ranger.*` /
//! `class_feature.ranger.*`). Two signals drive the diff, exactly as they
//! do for Barbarian:
//!
//! - **Value change** (e.g. base attack bonus 1 -> 2, Track's Survival
//!   bonus 9 -> 10): catches every magnitude-rising pillar.
//! - **Grant-state change** for level-gated features whose explanation
//!   record is either absent entirely below the gate (Evasion, Improved
//!   Evasion, which `explain_ranger_level1_chassis_and_class_feature_separation`
//!   pushes no record for at all below their own level gates) or present
//!   with an explicit "correctly absent" marker text below the gate
//!   (Endurance, Favored Terrain, Hunter's Bond, Woodland Stride, Swift
//!   Tracker, Quarry, Camouflage, Hide in Plain Sight, Master Hunter): both
//!   shapes are handled identically by treating a missing `from_explanations`
//!   match the same as an explicit absence marker (see `is_absent_marker`
//!   below, unchanged from `barbarian.rs`).
//!
//! Ranger has no per-level resource pool on this compute surface (unlike
//! Barbarian's rage rounds per day or Bard's bardic performance rounds per
//! day): Favored Enemy/Favored Terrain are rising flat bonuses, not a
//! daily-use pool, and no other Ranger pillar in
//! `explain_ranger_level1_chassis_and_class_feature_separation` carries a
//! per-day resource count. `resource_pool_change` therefore stays genuinely
//! empty, mirroring Fighter's own empty resource pool.
//!
//! `pick_from_lists` stays empty: Ranger's several genuinely open-ended
//! per-level choices (the Favored Enemy type, the Favored Terrain type, the
//! Combat Style's own restricted/open-ended bonus-feat slots) have no real
//! candidate catalog anywhere in `rules_tables::crb` to enumerate from —
//! the identical "no catalog to enumerate" boundary `barbarian.rs`
//! documented for the Rage Power list and `fighter.rs` documented for the
//! Bonus Feat candidate list, here scoped to Ranger's own multiple
//! choice-list surfaces. This is a documented, bounded scope note, not a
//! blocker on this cycle's `LevelUpPlan`: every other field lands for real.

use crate::rules_core::character_input::{CharacterClassLevel, CharacterInput};
use crate::rules_core::level_up::{Grant, GrantEffect, LevelUpPlan};
use crate::rules_core::pilot_compute::{compute_pilot_base_chassis, ComputationExplanation};
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::class_tables::{class_tables, ClassId, ClassTableRow};
use crate::rules_core::rules_tables::RuleSetId;

const RANGER_CLASS_ID: &str = "class:ranger";
const HUMAN_RACE_ID: &str = "race:human";
/// PF1 Core Rulebook Ranger capstone: Master Hunter, granted at 20th level
/// (verified against `pilot_compute.rs`'s own `RANGER_MASTER_HUNTER_LEVEL`
/// gate and its doc comment naming "Master Hunter" as the 20th-level Ranger
/// capstone — that constant is private to `pilot_compute.rs`, so this
/// mirrors its already-verified value rather than importing it, the same
/// "read the grounded fact, don't reach into another module's private
/// state" boundary every other Epic 7 lookup in this file respects via the
/// public `compute_pilot_base_chassis` seam).
const RANGER_CAPSTONE_LEVEL: u8 = 20;

/// Explanation ids that `append_class_table_grants` already covers from
/// `class_tables()` (the more authoritative, class-generic source per this
/// module's own doc comment). `pilot_compute.rs`'s chassis also happens to
/// carry standalone base-attack/base-save explanation records for the same
/// facts (grounded independently, pre-dating `class_tables.rs`); skipping
/// them here avoids reporting the identical fact twice under two different
/// names/provenances.
const CLASS_TABLE_COVERED_EXPLANATION_IDS: [&str; 4] = [
    "class_chassis.ranger.base_attack_bonus",
    "class_chassis.ranger.base_save.fortitude",
    "class_chassis.ranger.base_save.reflex",
    "class_chassis.ranger.base_save.will",
];

/// Composes a Ranger `LevelUpPlan` for the transition from `from_level` to
/// `to_level`. Bounded to single-class Human Ranger inputs, mirroring
/// `pilot_compute.rs`'s own `supported_ranger_level` gate (the source this
/// module reads from carries the identical bound, so widening this gate
/// independently would only ever surface an empty chassis snapshot, not
/// real data).
pub fn compute_ranger_level_up_grants(
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) -> LevelUpPlan {
    let mut plan = LevelUpPlan::default();

    let is_single_class_human_ranger = character.chosen.race_id == HUMAN_RACE_ID
        && matches!(
            character.chosen.class_levels.as_slice(),
            [class_level] if class_level.class_id == RANGER_CLASS_ID
        );
    if !is_single_class_human_ranger {
        return plan;
    }

    append_class_table_grants(&mut plan, from_level, to_level);
    append_class_feature_grants(&mut plan, character, from_level, to_level);

    plan.capstone_threshold = to_level >= RANGER_CAPSTONE_LEVEL;

    plan
}

fn class_table_row(level: u8) -> Option<ClassTableRow> {
    class_tables()
        .into_iter()
        .find(|row| row.class_id == ClassId::Ranger && row.level == level)
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
            name: format!("Ranger {column_key} at level {to_level}"),
            source_table: TableCellRef {
                rule_set: RuleSetId::Crb,
                table: "class_tables".to_owned(),
                row_key: format!("ranger:{to_level}"),
                column_key: column_key.to_owned(),
            },
            effects: vec![GrantEffect {
                description: format!(
                    "{column_key} rises to {to_value} at ranger level {to_level}"
                ),
                value: to_value,
            }],
        });
    }
}

/// Reads `pilot_compute::compute_pilot_base_chassis`'s ranger-specific
/// explanations for a synthetic single-class Ranger input at `level`,
/// keeping every other chosen field (race, ability scores, selected
/// choices) from `character` unchanged. Read-only: `pilot_compute.rs`
/// itself is never touched or re-derived, only its already-public output
/// is read twice (once per level) to diff.
fn ranger_chassis_explanations(character: &CharacterInput, level: u8) -> Vec<ComputationExplanation> {
    let mut probe = character.clone();
    probe.chosen.class_levels = vec![CharacterClassLevel {
        class_id: RANGER_CLASS_ID.to_owned(),
        level,
    }];
    compute_pilot_base_chassis(&probe).explanations
}

/// `explain_ranger_level1_chassis_and_class_feature_separation`'s
/// level-gated explanations (Endurance, Favored Terrain, Hunter's Bond,
/// Woodland Stride, Swift Tracker, Quarry, Camouflage, Hide in Plain
/// Sight, Master Hunter) word their "not yet granted" branch with this
/// exact marker phrase. Reading it is reading an already-computed,
/// already-grounded fact — not re-deriving the level gate that produced
/// it. Unchanged from `barbarian.rs`'s identical helper.
fn is_absent_marker(detail: &str) -> bool {
    detail.contains("correctly absent")
}

/// Grants sourced from `pilot_compute::compute_pilot_base_chassis`'s
/// ranger-specific explanations — the class-specific pillars
/// `class_tables.rs` does not carry. Ranger carries no per-level resource
/// pool on this compute surface (unlike Barbarian's rage rounds per day or
/// Bard's bardic performance rounds per day), so unlike those two modules
/// this function has no separate resource-pool block — mirroring
/// `fighter.rs`'s identical shape.
fn append_class_feature_grants(
    plan: &mut LevelUpPlan,
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) {
    let from_explanations = ranger_chassis_explanations(character, from_level);
    let to_explanations = ranger_chassis_explanations(character, to_level);

    for to_explanation in &to_explanations {
        let is_ranger_class_feature_id = to_explanation.id.starts_with("class_chassis.ranger.")
            || to_explanation.id.starts_with("class_feature.ranger.");
        let is_covered_elsewhere =
            CLASS_TABLE_COVERED_EXPLANATION_IDS.contains(&to_explanation.id.as_str());
        if !is_ranger_class_feature_id || is_covered_elsewhere {
            // The class-table-covered ids are already granted by
            // `append_class_table_grants`.
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
                table: "pilot_compute::explain_ranger_level1_chassis_and_class_feature_separation"
                    .to_owned(),
                row_key: format!("ranger:{to_level}"),
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
/// explanation id (e.g. `"class_feature.ranger.master_hunter"` -> `"master
/// hunter"`) — never a hand-typed label naming CRB rule text this module
/// has not itself verified.
fn friendly_name(id: &str) -> String {
    id.trim_start_matches("class_chassis.ranger.")
        .trim_start_matches("class_feature.ranger.")
        .replace(['_', '.'], " ")
}
