//! SD-20 Epic 7 (Level Up grant model): Monk — sixth core class per
//! Step 2's stated order (barbarian, bard, cleric, druid, fighter, monk,
//! ...; `scope-draft.md` §1.7, `technical-design.md` §2.6).
//!
//! Composes with two already-grounded, already-landed sources — the
//! identical two-source idiom `barbarian.rs` established:
//!
//! 1. `rules_tables::crb::class_tables::class_tables()` (SD-19's
//!    foundation slice) for the class-generic BAB/save progression.
//!    Monk's `CLASS_META` row (`ClassId::Monk`) was spot-checked before
//!    composing with it here, per this cycle's own brief: 3/4-BAB
//!    (`BabProgression::ThreeQuarter`) and `good_saves: { fortitude: true,
//!    reflex: true, will: true }`. Both match `pilot_compute.rs`'s own
//!    independently-verified `explain_monk_level1_chassis` formulas
//!    exactly (`classlevel * 3 / 4` for base attack, `classlevel / 2 + 2`
//!    for all three saves — Monk is unusual among the martial classes
//!    already landed in having all three base saves good, unlike
//!    Barbarian's single-good-save split). Unlike the Cleric/Druid
//!    `good_saves.fortitude` defect flagged and fixed at `28b0e88`,
//!    Monk's row carries **no defect**: it is correct as landed, so this
//!    module composes with it directly (the same posture `barbarian.rs`
//!    and `bard.rs` already established for their own correct rows).
//! 2. `pilot_compute::compute_pilot_base_chassis`'s own
//!    `explain_monk_level1_chassis` output (SD13-E3/E5/SD18, read
//!    read-only via the public `compute_pilot_base_chassis` seam and its
//!    `explanations` field — this module never touches `pilot_compute.rs`
//!    itself, staying inside Epic 7's file-touch partition) for the
//!    class-specific pillars `class_tables.rs` deliberately does not
//!    carry: AC Bonus (Wisdom-to-AC), the unarmed strike damage die (and,
//!    from level 12, its die-count facet), the Flurry of Blows flat
//!    attack-bonus/attack-count surface, Evasion, Improved Evasion, Still
//!    Mind, the ki pool's flat size, Slow Fall, Purity of Body, and
//!    Diamond Body.
//!
//! A level transition's automatic feature grants are computed as a
//! **diff** between the chassis snapshot at `from_level` and at
//! `to_level` (both computed against the same character, varying only
//! the class-level number), not by re-deriving PF1's own Monk
//! class-progression rules a second time — the identical idiom
//! `barbarian.rs` and `druid.rs` established. Two signals drive the diff:
//!
//! - **Value change** (e.g. base attack bonus 0 -> 1, the unarmed strike
//!   damage die 8 -> 10 at the level-8 milestone): catches every
//!   magnitude-rising pillar.
//! - **Grant-state change** for the bounded identity/recognition
//!   features whose value is always 0 whether granted or not (Evasion,
//!   Improved Evasion, Slow Fall, Purity of Body, Diamond Body): each
//!   such explanation's `detail` text deterministically distinguishes
//!   "correctly absent at level N by PF1 Core Rulebook level gate" from
//!   "granted at monk level N" (see `pilot_compute.rs`'s own
//!   `explain_monk_level1_chassis`) — reading that already-computed
//!   marker, not re-deriving the level gate.
//!
//! **Documented bounded scope note (mirroring the discipline
//! `barbarian.rs`'s and `druid.rs`'s own module doc comments already
//! established for their own scope notes):** Slow Fall's reach magnitude
//! (20 ft. at 4th level, rising to 30/40/50/60 ft. at 6th/8th/10th/12th
//! level) is recorded only in the underlying explanation's `detail` text,
//! never in its `.value` field (`pilot_compute.rs`'s own grant-only
//! identity-record idiom — the same shape as Barbarian's Uncanny Dodge or
//! Druid's Woodland Stride, both always `.value == 0`). This diff, which
//! drives off `.value` plus the absence-marker text, therefore correctly
//! grounds Slow Fall's initial grant (at 4th level) but does not surface
//! its later reach-magnitude tier rises as their own grants — the same
//! bounded, honest scope limitation as every other grant-only identity
//! record already composed with this idiom (no fabricated per-tier
//! numeric grant is invented to paper over it).
//!
//! **Ki pool as a resource pool, not a discrete grant:** mirrors
//! Barbarian's rage-rounds-per-day idiom exactly. The ki pool's flat size
//! (1/2 monk level + Wisdom modifier, correctly 0 below its 4th-level
//! gate) is a numeric resource-count magnitude, not a named "you now have
//! feature X" grant, so it lands in `LevelUpPlan.resource_pool_change`
//! rather than `automatic_features`.
//!
//! `pick_from_lists` stays empty: Monk's only genuinely open-ended
//! per-level choices are the four numbered bonus-feat slots (1st, 2nd,
//! 6th, 10th level), each drawn from a PF1 Core Rulebook restricted list
//! `pilot_compute.rs` already recognizes as chosen input (not a
//! candidate-enumeration catalog) — the identical "no catalog to
//! enumerate real candidates from" boundary `barbarian.rs`'s own Rage
//! Power note already established, here scoped to Monk's bonus-feat
//! slots instead. `MAX_SUPPORTED_MONK_LEVEL` (`pilot_compute.rs`) bounds
//! the grounded chassis data to Monk level 12 (`class_tables.rs`'s own
//! `CLASS_META` row carries the identical `max_supported_level: 12`); a
//! transition whose `to_level` exceeds that ceiling honestly produces no
//! `automatic_features` (the chassis probe returns no monk-namespaced
//! explanations there), while `capstone_threshold` still reports
//! correctly — a pure PF1 Core Rulebook level-number fact (level 20 is
//! every core class's capstone level), not derived from any per-class
//! grounded data source, mirroring `druid.rs`'s identical precedent for
//! its own lower ceiling (`MAX_SUPPORTED_DRUID_LEVEL = 15`).

use crate::rules_core::character_input::{CharacterClassLevel, CharacterInput};
use crate::rules_core::level_up::{Grant, GrantEffect, LevelUpPlan, ResourcePoolDelta};
use crate::rules_core::pilot_compute::{compute_pilot_base_chassis, ComputationExplanation};
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::class_tables::{class_tables, ClassId, ClassTableRow};
use crate::rules_core::rules_tables::RuleSetId;

const MONK_CLASS_ID: &str = "class:monk";
const HUMAN_RACE_ID: &str = "race:human";
/// PF1 Core Rulebook capstone level, universal across every core class
/// (`LevelUpPlan.capstone_threshold`'s own doc comment). Monk's own
/// grounded chassis data stops at `MAX_SUPPORTED_MONK_LEVEL = 12`
/// (`pilot_compute.rs`) — this constant is not a Monk-specific named
/// ability, only the shared PF1 level-cap fact, mirroring
/// `barbarian.rs`'s `BARBARIAN_CAPSTONE_LEVEL` / `druid.rs`'s
/// `DRUID_CAPSTONE_LEVEL` for the same reason.
const MONK_CAPSTONE_LEVEL: u8 = 20;

const KI_POOL_SIZE_EXPLANATION_ID: &str = "class_chassis.monk.ki_pool_size";

/// Explanation ids that `append_class_table_grants` already covers from
/// `class_tables()` (the more authoritative, class-generic source per
/// this module's own doc comment). `pilot_compute.rs`'s chassis also
/// happens to carry standalone base-attack/base-save explanation records
/// for the same facts (grounded independently, pre-dating
/// `class_tables.rs`); skipping them here avoids reporting the identical
/// fact twice under two different names/provenances — the identical
/// idiom `barbarian.rs` established.
const CLASS_TABLE_COVERED_EXPLANATION_IDS: [&str; 4] = [
    "class_chassis.monk.base_attack_bonus",
    "class_chassis.monk.base_save.fortitude",
    "class_chassis.monk.base_save.reflex",
    "class_chassis.monk.base_save.will",
];

/// Composes a Monk `LevelUpPlan` for the transition from `from_level` to
/// `to_level`. Bounded to single-class Human Monk inputs, mirroring
/// `pilot_compute.rs`'s own `supported_monk_level` gate (the source this
/// module reads from carries the identical bound, so widening this gate
/// independently would only ever surface an empty chassis snapshot, not
/// real data).
pub fn compute_monk_level_up_grants(
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) -> LevelUpPlan {
    let mut plan = LevelUpPlan::default();

    let is_single_class_human_monk = character.chosen.race_id == HUMAN_RACE_ID
        && matches!(
            character.chosen.class_levels.as_slice(),
            [class_level] if class_level.class_id == MONK_CLASS_ID
        );
    if !is_single_class_human_monk {
        return plan;
    }

    append_class_table_grants(&mut plan, from_level, to_level);
    append_class_feature_grants(&mut plan, character, from_level, to_level);

    plan.capstone_threshold = to_level >= MONK_CAPSTONE_LEVEL;

    plan
}

fn class_table_row(level: u8) -> Option<ClassTableRow> {
    class_tables()
        .into_iter()
        .find(|row| row.class_id == ClassId::Monk && row.level == level)
}

/// Grants sourced from `rules_tables::crb::class_tables::class_tables()`
/// — the class-generic BAB/save progression pillars. Identical shape to
/// `barbarian.rs`'s own `append_class_table_grants`, except Monk's three
/// saves are ALL good (so all three columns commonly rise together,
/// unlike Barbarian's single good save).
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
            name: format!("Monk {column_key} at level {to_level}"),
            source_table: TableCellRef {
                rule_set: RuleSetId::Crb,
                table: "class_tables".to_owned(),
                row_key: format!("monk:{to_level}"),
                column_key: column_key.to_owned(),
            },
            effects: vec![GrantEffect {
                description: format!(
                    "{column_key} rises to {to_value} at monk level {to_level}"
                ),
                value: to_value,
            }],
        });
    }
}

/// Reads `pilot_compute::compute_pilot_base_chassis`'s monk-specific
/// explanations for a synthetic single-class Monk input at `level`,
/// keeping every other chosen field (race, ability scores) from
/// `character` unchanged. Read-only: `pilot_compute.rs` itself is never
/// touched or re-derived, only its already-public output is read twice
/// (once per level) to diff. Identical idiom to `barbarian.rs`'s /
/// `druid.rs`'s own probe helpers.
fn monk_chassis_explanations(character: &CharacterInput, level: u8) -> Vec<ComputationExplanation> {
    let mut probe = character.clone();
    probe.chosen.class_levels = vec![CharacterClassLevel {
        class_id: MONK_CLASS_ID.to_owned(),
        level,
    }];
    compute_pilot_base_chassis(&probe).explanations
}

/// `explain_monk_level1_chassis`'s own level-gated explanations (Evasion,
/// Still Mind, the ki pool, Slow Fall, Purity of Body) always word their
/// "not yet granted" branch with this exact marker phrase. Reading it is
/// reading an already-computed, already-grounded fact — not re-deriving
/// the level gate that produced it. Identical idiom to `barbarian.rs`'s /
/// `druid.rs`'s own `is_absent_marker`.
fn is_absent_marker(detail: &str) -> bool {
    detail.contains("correctly absent")
}

/// Grants (and the one resource-pool change) sourced from
/// `pilot_compute::compute_pilot_base_chassis`'s monk-specific
/// explanations — the class-specific pillars `class_tables.rs` does not
/// carry.
fn append_class_feature_grants(
    plan: &mut LevelUpPlan,
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) {
    let from_explanations = monk_chassis_explanations(character, from_level);
    let to_explanations = monk_chassis_explanations(character, to_level);

    for to_explanation in &to_explanations {
        let is_monk_pillar_id = to_explanation.id.starts_with("class_chassis.monk.")
            || to_explanation.id.starts_with("class_feature.monk.");
        let is_covered_elsewhere = to_explanation.id == KI_POOL_SIZE_EXPLANATION_ID
            || CLASS_TABLE_COVERED_EXPLANATION_IDS.contains(&to_explanation.id.as_str());
        if !is_monk_pillar_id || is_covered_elsewhere {
            // The ki pool is a resource pool (handled separately below);
            // the class-table-covered ids are already granted by
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
                table: "pilot_compute::explain_monk_level1_chassis".to_owned(),
                row_key: format!("monk:{to_level}"),
                column_key: to_explanation.id.clone(),
            },
            effects: vec![GrantEffect {
                description: to_explanation.detail.clone(),
                value: to_explanation.value,
            }],
        });
    }

    let from_ki_pool = from_explanations
        .iter()
        .find(|explanation| explanation.id == KI_POOL_SIZE_EXPLANATION_ID)
        .map(|explanation| explanation.value);
    let to_ki_pool = to_explanations
        .iter()
        .find(|explanation| explanation.id == KI_POOL_SIZE_EXPLANATION_ID)
        .map(|explanation| explanation.value);
    if let (Some(from_value), Some(to_value)) = (from_ki_pool, to_ki_pool)
        && from_value != to_value
    {
        plan.resource_pool_change.pools.push(ResourcePoolDelta {
            pool_id: "ki_pool_size".to_owned(),
            from_value,
            to_value,
            source_table: TableCellRef {
                rule_set: RuleSetId::Crb,
                table: "pilot_compute::explain_monk_level1_chassis".to_owned(),
                row_key: format!("monk:{to_level}"),
                column_key: KI_POOL_SIZE_EXPLANATION_ID.to_owned(),
            },
        });
    }
}

/// Derives a human-readable grant name mechanically from a grounded
/// explanation id (e.g. `"class_feature.monk.evasion"` -> `"evasion"`)
/// — never a hand-typed label naming CRB rule text this module has not
/// itself verified. Identical idiom to `barbarian.rs`'s / `druid.rs`'s
/// own `friendly_name`.
fn friendly_name(id: &str) -> String {
    id.trim_start_matches("class_chassis.monk.")
        .trim_start_matches("class_feature.monk.")
        .replace(['_', '.'], " ")
}
