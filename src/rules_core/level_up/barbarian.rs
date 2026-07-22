//! SD-20 Epic 7 (Level Up grant model): Barbarian — first core class per
//! Step 2's stated order (`scope-draft.md` §1.7, `technical-design.md`
//! §2.6).
//!
//! Composes with two already-grounded, already-landed sources rather
//! than re-deriving either (mirroring how Epic 6's cycles composed with
//! Epic 5's `equipment_effects.rs` output instead of re-deriving corpus
//! lookups):
//!
//! 1. `rules_tables::crb::class_tables::class_tables()` (SD-19's
//!    foundation slice) for the class-generic BAB/save progression —
//!    the class-generic pillars this table already carries.
//! 2. `pilot_compute::compute_pilot_base_chassis`'s own
//!    `explain_barbarian_level1_chassis` output (SD13/SD18, read
//!    read-only via the public `compute_pilot_base_chassis` seam and its
//!    `explanations` field — this module never touches `pilot_compute.rs`
//!    itself, staying inside Epic 7's file-touch partition) for the
//!    class-specific pillars `class_tables.rs` deliberately does not
//!    carry (its own doc comment: "Named per-level features ... are
//!    deliberately out of scope for this bootstrap"): rage rounds per
//!    day, the four flat while-raging rage constants (and their Greater
//!    Rage / Mighty Rage tier rises), Uncanny Dodge, Trap Sense, Improved
//!    Uncanny Dodge, Damage Reduction, Indomitable Will, and Tireless
//!    Rage.
//!
//! A level transition's automatic feature grants are computed as a
//! **diff** between the chassis snapshot at `from_level` and at
//! `to_level` (both computed against the same character, varying only
//! the class-level number), not by re-deriving PF1's own barbarian
//! class-progression rules a second time. Two signals drive the diff:
//!
//! - **Value change** (e.g. base attack bonus 1 -> 2, Trap Sense +1 ->
//!   +2): catches every magnitude-rising pillar.
//! - **Grant-state change** for the bounded identity/recognition
//!   features whose value is always 0 whether granted or not (Uncanny
//!   Dodge, Improved Uncanny Dodge, Tireless Rage): each such
//!   explanation's `detail` text deterministically distinguishes
//!   "correctly absent at level N by PF1 Core Rulebook level gate" from
//!   "granted at barbarian level N" (see `pilot_compute.rs`'s own
//!   `explain_barbarian_level1_chassis`) — reading that already-computed
//!   marker, not re-deriving the level gate.
//!
//! `pick_from_lists` stays empty: Barbarian's only genuinely open-ended
//! per-level choice is the Rage Power list (granted at barbarian levels
//! 2/4/6/8/10/12/14/16/18/20 per `BARBARIAN_RAGE_POWER_SLOTS` in
//! `pilot_compute.rs`), and no Rage Power catalog exists anywhere in
//! `rules_tables::crb` to enumerate real candidates from — fabricating a
//! candidate list would be exactly the counterfeit-completion risk
//! `AGENTS.md` rules out. This is a documented, bounded scope note (like
//! Epic 6's feat-effect modifier bounding to constant-valued feats only),
//! not a blocker on this cycle's `LevelUpPlan`: every other field lands
//! for real.
//!
//! **SD-25 Epic 7 residue audit (criterion 7.5): verified NO DEFECT found.**
//! SD-24 criterion 4.1 found and fixed a bug in `level_up/wizard.rs`: its
//! explanation-id filter admitted only `class_chassis.wizard.` (plus one
//! named recognition id), so once a later `pilot_compute.rs` grounding
//! landed a real `class_spell.wizard.*` explanation family — AFTER
//! `wizard.rs` was authored — every one of those real facts was silently
//! dropped from `LevelUpPlan` because the filter was never widened to
//! admit the new prefix. SD-24 never audited the other 9 CRB classes for
//! the same bug shape (`progress.md`'s `## DISCOVERED` register A6
//! carry-forward). This cycle audits Barbarian against that exact
//! pattern: grepped every `barbarian`-containing explanation id
//! `pilot_compute.rs` grounds (144 mentions, all identity/rule-text prose
//! or the two families below — confirmed exhaustively, not sampled) and
//! found only two live prefixes, `class_chassis.barbarian.*` and
//! `class_feature.barbarian.*` — the exact two
//! `append_class_feature_grants`'s `is_barbarian_class_feature_id` already
//! admits (see below). No `class_spell.barbarian.*` family exists:
//! Barbarian is a non-caster in the PF1 Core Rulebook, so unlike Wizard,
//! Sorcerer, Bard, Cleric, Druid, Paladin, and Ranger, there is no spell
//! grounding to have ever landed a third prefix behind an unwidened
//! filter. `tests/sd25_barbarian_level_up_explanation_filter_audit.rs`
//! proves this two ways: (1) every `barbarian`-containing explanation id
//! from `compute_pilot_base_chassis`, swept across all 20 supported
//! levels, is asserted to start with one of the two admitted prefixes;
//! (2) every real (non-"correctly absent", non-class-table-covered,
//! non-resource-pool) explanation that newly becomes granted across all
//! 19 level-up transitions is asserted to surface as a real grant (or, for
//! rage rounds per day, a real resource pool change) in this module's own
//! `compute_barbarian_level_up_grants` output. That test file's own doc
//! comment records that temporarily narrowing the filter to admit only
//! `class_chassis.barbarian.` (dropping `class_feature.barbarian.` — the
//! exact Wizard bug shape, applied to Barbarian's second prefix) was
//! confirmed live to make check (2) fail (Uncanny Dodge, Trap Sense,
//! Improved Uncanny Dodge, Damage Reduction, and Indomitable Will all
//! vanish from the plan) before being reverted — proof the audit test is
//! load-bearing, not a rubber-stamp. **No code change to this module's
//! filter was needed or made**; this is a real, verified negative
//! finding, not a skipped check. See
//! `docs/release/SD-25-ui-evaluation-defect-closure/artifacts/epic_7/barbarian-residue-audit-cycle_receipt.md`.

use crate::rules_core::character_input::{CharacterClassLevel, CharacterInput};
use crate::rules_core::level_up::{Grant, GrantEffect, LevelUpPlan, ResourcePoolDelta};
use crate::rules_core::pilot_compute::{compute_pilot_base_chassis, ComputationExplanation};
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::class_tables::{class_tables, ClassId, ClassTableRow};
use crate::rules_core::rules_tables::RuleSetId;

const BARBARIAN_CLASS_ID: &str = "class:barbarian";
const HUMAN_RACE_ID: &str = "race:human";
/// PF1 Core Rulebook Barbarian capstone: Mighty Rage, granted at 20th
/// level (verified against `pilot_compute.rs`'s own
/// `BARBARIAN_MIGHTY_RAGE_LEVEL` gate and its doc comment naming "Mighty
/// Rage" as the level-20 tier — that constant is private to
/// `pilot_compute.rs`, so this mirrors its already-verified value rather
/// than importing it, the same "read the grounded fact, don't reach into
/// another module's private state" boundary every other Epic 7 lookup
/// in this file respects via the public `compute_pilot_base_chassis`
/// seam).
const BARBARIAN_CAPSTONE_LEVEL: u8 = 20;

const RAGE_ROUNDS_PER_DAY_EXPLANATION_ID: &str = "class_chassis.barbarian.rage_rounds_per_day";

/// Explanation ids that `append_class_table_grants` already covers from
/// `class_tables()` (the more authoritative, class-generic source per
/// this module's own doc comment). `pilot_compute.rs`'s chassis also
/// happens to carry standalone base-attack/base-save explanation
/// records for the same facts (grounded independently, pre-dating
/// `class_tables.rs`); skipping them here avoids reporting the identical
/// fact twice under two different names/provenances.
const CLASS_TABLE_COVERED_EXPLANATION_IDS: [&str; 4] = [
    "class_chassis.barbarian.base_attack_bonus",
    "class_chassis.barbarian.base_save.fortitude",
    "class_chassis.barbarian.base_save.reflex",
    "class_chassis.barbarian.base_save.will",
];

/// Composes a Barbarian `LevelUpPlan` for the transition from
/// `from_level` to `to_level`. Bounded to single-class Human Barbarian
/// inputs, mirroring `pilot_compute.rs`'s own
/// `supported_barbarian_level` gate (the source this module reads from
/// carries the identical bound, so widening this gate independently
/// would only ever surface an empty chassis snapshot, not real data).
pub fn compute_barbarian_level_up_grants(
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) -> LevelUpPlan {
    let mut plan = LevelUpPlan::default();

    let is_single_class_human_barbarian = character.chosen.race_id == HUMAN_RACE_ID
        && matches!(
            character.chosen.class_levels.as_slice(),
            [class_level] if class_level.class_id == BARBARIAN_CLASS_ID
        );
    if !is_single_class_human_barbarian {
        return plan;
    }

    append_class_table_grants(&mut plan, from_level, to_level);
    append_class_feature_grants(&mut plan, character, from_level, to_level);

    plan.capstone_threshold = to_level >= BARBARIAN_CAPSTONE_LEVEL;

    plan
}

fn class_table_row(level: u8) -> Option<ClassTableRow> {
    class_tables()
        .into_iter()
        .find(|row| row.class_id == ClassId::Barbarian && row.level == level)
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
            name: format!("Barbarian {column_key} at level {to_level}"),
            source_table: TableCellRef {
                rule_set: RuleSetId::Crb,
                table: "class_tables".to_owned(),
                row_key: format!("barbarian:{to_level}"),
                column_key: column_key.to_owned(),
            },
            effects: vec![GrantEffect {
                description: format!(
                    "{column_key} rises to {to_value} at barbarian level {to_level}"
                ),
                value: to_value,
            }],
        });
    }
}

/// Reads `pilot_compute::compute_pilot_base_chassis`'s barbarian-specific
/// explanations for a synthetic single-class Barbarian input at `level`,
/// keeping every other chosen field (race, ability scores) from
/// `character` unchanged. Read-only: `pilot_compute.rs` itself is never
/// touched or re-derived, only its already-public output is read twice
/// (once per level) to diff.
fn barbarian_chassis_explanations(
    character: &CharacterInput,
    level: u8,
) -> Vec<ComputationExplanation> {
    let mut probe = character.clone();
    probe.chosen.class_levels = vec![CharacterClassLevel {
        class_id: BARBARIAN_CLASS_ID.to_owned(),
        level,
    }];
    compute_pilot_base_chassis(&probe).explanations
}

/// `explain_barbarian_level1_chassis`'s own level-gated explanations
/// (Uncanny Dodge, Trap Sense, Improved Uncanny Dodge, Damage Reduction,
/// Indomitable Will, Tireless Rage) always word their "not yet granted"
/// branch with this exact marker phrase. Reading it is reading an
/// already-computed, already-grounded fact — not re-deriving the level
/// gate that produced it.
fn is_absent_marker(detail: &str) -> bool {
    detail.contains("correctly absent")
}

/// Grants (and the one resource-pool change) sourced from
/// `pilot_compute::compute_pilot_base_chassis`'s barbarian-specific
/// explanations — the class-specific pillars `class_tables.rs` does not
/// carry.
fn append_class_feature_grants(
    plan: &mut LevelUpPlan,
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) {
    let from_explanations = barbarian_chassis_explanations(character, from_level);
    let to_explanations = barbarian_chassis_explanations(character, to_level);

    for to_explanation in &to_explanations {
        let is_barbarian_class_feature_id = to_explanation.id.starts_with("class_chassis.barbarian.")
            || to_explanation.id.starts_with("class_feature.barbarian.");
        let is_covered_elsewhere = to_explanation.id == RAGE_ROUNDS_PER_DAY_EXPLANATION_ID
            || CLASS_TABLE_COVERED_EXPLANATION_IDS.contains(&to_explanation.id.as_str());
        if !is_barbarian_class_feature_id || is_covered_elsewhere {
            // Rage rounds per day is a resource pool (handled
            // separately below); the class-table-covered ids are
            // already granted by `append_class_table_grants`.
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
                table: "pilot_compute::explain_barbarian_level1_chassis".to_owned(),
                row_key: format!("barbarian:{to_level}"),
                column_key: to_explanation.id.clone(),
            },
            effects: vec![GrantEffect {
                description: to_explanation.detail.clone(),
                value: to_explanation.value,
            }],
        });
    }

    let from_rage_rounds = from_explanations
        .iter()
        .find(|explanation| explanation.id == RAGE_ROUNDS_PER_DAY_EXPLANATION_ID)
        .map(|explanation| explanation.value);
    let to_rage_rounds = to_explanations
        .iter()
        .find(|explanation| explanation.id == RAGE_ROUNDS_PER_DAY_EXPLANATION_ID)
        .map(|explanation| explanation.value);
    if let (Some(from_value), Some(to_value)) = (from_rage_rounds, to_rage_rounds)
        && from_value != to_value
    {
        plan.resource_pool_change.pools.push(ResourcePoolDelta {
            pool_id: "rage_rounds_per_day".to_owned(),
            from_value,
            to_value,
            source_table: TableCellRef {
                rule_set: RuleSetId::Crb,
                table: "pilot_compute::explain_barbarian_level1_chassis".to_owned(),
                row_key: format!("barbarian:{to_level}"),
                column_key: RAGE_ROUNDS_PER_DAY_EXPLANATION_ID.to_owned(),
            },
        });
    }
}

/// Derives a human-readable grant name mechanically from a grounded
/// explanation id (e.g. `"class_feature.barbarian.uncanny_dodge"` ->
/// `"uncanny dodge"`) — never a hand-typed label naming CRB rule text
/// this module has not itself verified.
fn friendly_name(id: &str) -> String {
    id.trim_start_matches("class_chassis.barbarian.")
        .trim_start_matches("class_feature.barbarian.")
        .replace(['_', '.'], " ")
}
