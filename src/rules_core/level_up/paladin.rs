//! SD-20 Epic 7 (Level Up grant model): Paladin — seventh core class per
//! Step 2's stated order (barbarian, bard, cleric, druid, fighter, monk,
//! paladin, ...; `scope-draft.md` §1.7, `technical-design.md` §2.6).
//!
//! Mirrors `barbarian.rs`'s exact composition pattern (NOT `druid.rs`'s
//! deviation): composes with two already-grounded, already-landed
//! sources rather than re-deriving either.
//!
//! 1. `rules_tables::crb::class_tables::class_tables()` (SD-19's
//!    foundation slice) for the class-generic BAB/save progression —
//!    the class-generic pillars this table already carries. Unlike
//!    Cleric/Druid (whose `CLASS_META` rows were found to wrongly encode
//!    `good_saves.fortitude: false`, fixed in `28b0e88`), Paladin's own
//!    `CLASS_META` row (`good_saves: { fortitude: true, reflex: false,
//!    will: true }`, full BAB) was spot-checked against
//!    `pilot_compute.rs`'s own `explain_paladin_level1_chassis_and_spell_burden_separation`
//!    formula (`good_save = paladin_level / 2 + 2` applied to Fortitude
//!    AND Will, `poor_save = paladin_level / 3` applied to Reflex,
//!    `base_attack_bonus = paladin_level` for full BAB) before writing
//!    any code here: the two sources agree at every level 1-20, so no
//!    second latent `CLASS_META` defect exists for Paladin — this module
//!    composes with `class_tables()` directly, the same as `barbarian.rs`
//!    and `bard.rs`.
//! 2. `pilot_compute::compute_pilot_base_chassis`'s own
//!    `explain_paladin_level1_chassis_and_spell_burden_separation` output
//!    (SD13/SD18, read read-only via the public
//!    `compute_pilot_base_chassis` seam and its `explanations` field —
//!    this module never touches `pilot_compute.rs` itself, staying
//!    inside Epic 7's file-touch partition) for the class-specific
//!    pillars `class_tables.rs` deliberately does not carry: Smite Evil
//!    (uses/day, attack bonus, damage bonus), Divine Grace, Lay on Hands
//!    (uses/day, heal amount), Mercy (the grant-only identity record, the
//!    optional chosen-mercy recognition, and the six numbered repeat-grant
//!    slots), Channel Positive Energy (dice), Aura of Justice, Aura of
//!    Faith, Aura of Righteousness, Holy Champion (the 20th-level
//!    capstone), the partial-caster effective-caster-level gate, and the
//!    partial-caster spell-level-access ladder plus its per-spell-level
//!    base/DC/bonus/total records once paladin level 4+ unlocks them.
//!
//! **Not grounded anywhere in this codebase, so not surfaced here
//! either** (the identical "no catalog / no engine to compose with"
//! boundary `barbarian.rs` documented for the Rage Power list): Aura of
//! Good, Detect Evil, and Aura of Courage have no explanation records in
//! `pilot_compute.rs` at all (grepped and confirmed absent — not even a
//! level-gate-absence record), and Divine Bond is explicitly named in
//! `pilot_compute.rs`'s own doc comment as "deliberately
//! named-but-unproven" (it needs an activation/resource-consumption
//! engine plus either a weapon-enhancement subsystem or a full mount
//! stat-block, neither of which exists in this repo). This module does
//! not fabricate grants for any of the four.
//!
//! A level transition's automatic feature grants are computed as a
//! **diff** between the chassis snapshot at `from_level` and at
//! `to_level` (both computed against the same character, varying only
//! the class-level number), not by re-deriving PF1's own paladin
//! class-progression rules a second time — identical algorithm to
//! `barbarian.rs`'s `append_class_feature_grants`, parameterized to the
//! Paladin's own explanation-id namespace (`class_chassis.paladin.*`)
//! and its own TWO resource pools (`smite_evil_uses_per_day` and
//! `lay_on_hands_uses_per_day`, both of which carry a genuine paladin-level
//! term in their PF1 Core Rulebook formula — unlike Cleric's flat
//! ability-modifier-only Channel Energy uses/day, which never changes
//! across a level transition). Two signals drive the diff, exactly as
//! they do for Barbarian/Bard:
//!
//! - **Value change** (e.g. base attack bonus 1 -> 2, Smite Evil damage
//!   bonus 1 -> 2): catches every magnitude-rising pillar.
//! - **Grant-state change** for level-gated features whose explanation
//!   id itself CHANGES across the gate (e.g.
//!   `class_chassis.paladin.level_gate.lay_on_hands` below the level-2
//!   gate becomes `class_chassis.paladin.lay_on_hands_uses_per_day` /
//!   `.lay_on_hands_heal_amount` at or above it): the generic
//!   id-match-against-`from_explanations` diff treats a to-explanation
//!   with no from-side match as newly granted, so a changed id across
//!   the gate is handled identically to Barbarian's same-id
//!   "correctly absent" -> "granted" marker-text transition, with no
//!   special-casing needed.
//!
//! `pick_from_lists` stays empty: Paladin's own per-level choices (which
//! mercy to select at levels 3/6/9/12/15/18) are recognized as chosen
//! input via `choice_selection`, not enumerated from a real mercy
//! catalog — no mercy-list catalog exists anywhere in `rules_tables::crb`
//! to enumerate real candidates from, the identical "no catalog to
//! enumerate" boundary `barbarian.rs` documented for the Rage Power list.
//! `prerequisites_added` stays empty: no per-class file yet composes with
//! Epic 3's `feat_prereqs` output (unchanged from every other landed Epic
//! 7 class so far).

use crate::rules_core::character_input::{CharacterClassLevel, CharacterInput};
use crate::rules_core::level_up::{Grant, GrantEffect, LevelUpPlan, ResourcePoolDelta};
use crate::rules_core::pilot_compute::{compute_pilot_base_chassis, ComputationExplanation};
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::class_tables::{class_tables, ClassId, ClassTableRow};
use crate::rules_core::rules_tables::RuleSetId;

const PALADIN_CLASS_ID: &str = "class:paladin";
const HUMAN_RACE_ID: &str = "race:human";
/// PF1 Core Rulebook Paladin capstone: Holy Champion, granted at 20th
/// level (verified against `pilot_compute.rs`'s own
/// `PALADIN_HOLY_CHAMPION_LEVEL` gate and its doc comment naming "Holy
/// Champion" as the 20th-level paladin capstone — that constant is
/// private to `pilot_compute.rs`, so this mirrors its already-verified
/// value rather than importing it, the same boundary `barbarian.rs`'s own
/// `BARBARIAN_CAPSTONE_LEVEL` respects).
const PALADIN_CAPSTONE_LEVEL: u8 = 20;

const SMITE_EVIL_USES_PER_DAY_EXPLANATION_ID: &str =
    "class_chassis.paladin.smite_evil_uses_per_day";
const LAY_ON_HANDS_USES_PER_DAY_EXPLANATION_ID: &str =
    "class_chassis.paladin.lay_on_hands_uses_per_day";

/// Explanation ids that `append_class_table_grants` already covers from
/// `class_tables()` (the more authoritative, class-generic source per
/// this module's own doc comment). `pilot_compute.rs`'s chassis also
/// happens to carry standalone base-attack/base-save explanation records
/// for the same facts (grounded independently, pre-dating
/// `class_tables.rs`); skipping them here avoids reporting the identical
/// fact twice under two different names/provenances.
const CLASS_TABLE_COVERED_EXPLANATION_IDS: [&str; 4] = [
    "class_chassis.paladin.base_attack_bonus",
    "class_chassis.paladin.base_save.fortitude",
    "class_chassis.paladin.base_save.reflex",
    "class_chassis.paladin.base_save.will",
];

/// Composes a Paladin `LevelUpPlan` for the transition from `from_level`
/// to `to_level`. Bounded to single-class Human Paladin inputs, mirroring
/// `pilot_compute.rs`'s own `supported_paladin_level` gate (the source
/// this module reads from carries the identical bound, so widening this
/// gate independently would only ever surface an empty chassis snapshot,
/// not real data).
pub fn compute_paladin_level_up_grants(
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) -> LevelUpPlan {
    let mut plan = LevelUpPlan::default();

    let is_single_class_human_paladin = character.chosen.race_id == HUMAN_RACE_ID
        && matches!(
            character.chosen.class_levels.as_slice(),
            [class_level] if class_level.class_id == PALADIN_CLASS_ID
        );
    if !is_single_class_human_paladin {
        return plan;
    }

    append_class_table_grants(&mut plan, from_level, to_level);
    append_class_feature_grants(&mut plan, character, from_level, to_level);

    plan.capstone_threshold = to_level >= PALADIN_CAPSTONE_LEVEL;

    plan
}

fn class_table_row(level: u8) -> Option<ClassTableRow> {
    class_tables()
        .into_iter()
        .find(|row| row.class_id == ClassId::Paladin && row.level == level)
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
            name: format!("Paladin {column_key} at level {to_level}"),
            source_table: TableCellRef {
                rule_set: RuleSetId::Crb,
                table: "class_tables".to_owned(),
                row_key: format!("paladin:{to_level}"),
                column_key: column_key.to_owned(),
            },
            effects: vec![GrantEffect {
                description: format!(
                    "{column_key} rises to {to_value} at paladin level {to_level}"
                ),
                value: to_value,
            }],
        });
    }
}

/// Reads `pilot_compute::compute_pilot_base_chassis`'s paladin-specific
/// explanations for a synthetic single-class Paladin input at `level`,
/// keeping every other chosen field (race, ability scores, selected
/// choices) from `character` unchanged. Read-only: `pilot_compute.rs`
/// itself is never touched or re-derived, only its already-public output
/// is read twice (once per level) to diff.
fn paladin_chassis_explanations(
    character: &CharacterInput,
    level: u8,
) -> Vec<ComputationExplanation> {
    let mut probe = character.clone();
    probe.chosen.class_levels = vec![CharacterClassLevel {
        class_id: PALADIN_CLASS_ID.to_owned(),
        level,
    }];
    compute_pilot_base_chassis(&probe).explanations
}

/// `explain_paladin_level1_chassis_and_spell_burden_separation`'s own
/// level-gated explanations word their "not yet granted" branch with
/// this exact marker phrase. Reading it is reading an already-computed,
/// already-grounded fact — not re-deriving the level gate that produced
/// it. Unchanged from `barbarian.rs`'s identical helper.
fn is_absent_marker(detail: &str) -> bool {
    detail.contains("correctly absent")
}

/// Grants (and the two resource-pool changes) sourced from
/// `pilot_compute::compute_pilot_base_chassis`'s paladin-specific
/// explanations — the class-specific pillars `class_tables.rs` does not
/// carry.
fn append_class_feature_grants(
    plan: &mut LevelUpPlan,
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) {
    let from_explanations = paladin_chassis_explanations(character, from_level);
    let to_explanations = paladin_chassis_explanations(character, to_level);

    for to_explanation in &to_explanations {
        let is_paladin_class_feature_id = to_explanation.id.starts_with("class_chassis.paladin.")
            || to_explanation.id.starts_with("class_feature.paladin.");
        let is_covered_elsewhere = to_explanation.id == SMITE_EVIL_USES_PER_DAY_EXPLANATION_ID
            || to_explanation.id == LAY_ON_HANDS_USES_PER_DAY_EXPLANATION_ID
            || CLASS_TABLE_COVERED_EXPLANATION_IDS.contains(&to_explanation.id.as_str());
        if !is_paladin_class_feature_id || is_covered_elsewhere {
            // Smite Evil uses/day and Lay on Hands uses/day are resource
            // pools (handled separately below); the class-table-covered
            // ids are already granted by `append_class_table_grants`.
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
                table: "pilot_compute::explain_paladin_level1_chassis_and_spell_burden_separation"
                    .to_owned(),
                row_key: format!("paladin:{to_level}"),
                column_key: to_explanation.id.clone(),
            },
            effects: vec![GrantEffect {
                description: to_explanation.detail.clone(),
                value: to_explanation.value,
            }],
        });
    }

    append_resource_pool_change(
        plan,
        &from_explanations,
        &to_explanations,
        SMITE_EVIL_USES_PER_DAY_EXPLANATION_ID,
        "smite_evil_uses_per_day",
        to_level,
    );
    append_resource_pool_change(
        plan,
        &from_explanations,
        &to_explanations,
        LAY_ON_HANDS_USES_PER_DAY_EXPLANATION_ID,
        "lay_on_hands_uses_per_day",
        to_level,
    );
}

/// Pushes a `ResourcePoolDelta` when `explanation_id`'s value genuinely
/// changed between `from_explanations` and `to_explanations`. Shared by
/// both of Paladin's daily-use resource pools (Smite Evil uses/day and
/// Lay on Hands uses/day both carry a genuine paladin-level term in their
/// PF1 Core Rulebook formula, unlike Cleric's flat Channel Energy
/// uses/day) — factored out rather than duplicated inline, mirroring
/// `barbarian.rs`'s single inline block but generalized to Paladin's two
/// pools.
fn append_resource_pool_change(
    plan: &mut LevelUpPlan,
    from_explanations: &[ComputationExplanation],
    to_explanations: &[ComputationExplanation],
    explanation_id: &str,
    pool_id: &str,
    to_level: u8,
) {
    let from_value = from_explanations
        .iter()
        .find(|explanation| explanation.id == explanation_id)
        .map(|explanation| explanation.value);
    let to_value = to_explanations
        .iter()
        .find(|explanation| explanation.id == explanation_id)
        .map(|explanation| explanation.value);
    if let (Some(from_value), Some(to_value)) = (from_value, to_value)
        && from_value != to_value
    {
        plan.resource_pool_change.pools.push(ResourcePoolDelta {
            pool_id: pool_id.to_owned(),
            from_value,
            to_value,
            source_table: TableCellRef {
                rule_set: RuleSetId::Crb,
                table: "pilot_compute::explain_paladin_level1_chassis_and_spell_burden_separation"
                    .to_owned(),
                row_key: format!("paladin:{to_level}"),
                column_key: explanation_id.to_owned(),
            },
        });
    }
}

/// Derives a human-readable grant name mechanically from a grounded
/// explanation id (e.g. `"class_chassis.paladin.divine_grace_save_bonus"`
/// -> `"divine grace save bonus"`) — never a hand-typed label naming CRB
/// rule text this module has not itself verified. Identical idiom to
/// `barbarian.rs`'s `friendly_name`.
fn friendly_name(id: &str) -> String {
    id.trim_start_matches("class_chassis.paladin.")
        .trim_start_matches("class_feature.paladin.")
        .replace(['_', '.'], " ")
}
