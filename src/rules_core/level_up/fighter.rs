//! SD-20 Epic 7 (Level Up grant model): Fighter — fifth core class per
//! Step 2's stated order (`scope-draft.md` §1.7, `technical-design.md`
//! §2.6), landed after barbarian, bard, cleric, and druid.
//!
//! Fighter is a strong structural analog to Barbarian's own already-landed
//! cycle (full BAB, a simple class-feature progression with no spellcasting
//! or resource pool), so this module composes with the identical two
//! already-grounded, already-landed sources rather than re-deriving either:
//!
//! 1. `rules_tables::crb::class_tables::class_tables()` (SD-19's foundation
//!    slice) for the class-generic BAB/save progression. Fighter's
//!    `CLASS_META` row was spot-checked against the PF1 Core Rulebook
//!    before composing with it (per this cycle's own brief, since the
//!    identical Cleric/Druid `good_saves.fortitude` row was found wrong
//!    and fixed in `28b0e88`): Fighter's row reads `good_saves { fortitude:
//!    true, reflex: false, will: false }` and `bab: Full` —
//!    independently verified against `pilot_compute.rs`'s own
//!    already-grounded, already-primary-source-verified
//!    `compute_fighter_chassis` formulas (`cr_classes.lst:139`:
//!    `BONUS:COMBAT|BASEAB|classlevel` = full BAB;
//!    `BONUS:SAVE|BASE.Fortitude|classlevel/2+2` = good Fortitude;
//!    `BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel/3` = poor Reflex/Will) —
//!    CONFIRMED CORRECT, not a second latent defect. `class_tables()` is
//!    safe to compose with for Fighter.
//! 2. `pilot_compute::compute_pilot_base_chassis`'s own
//!    `explain_fighter_class_features` output (SD13/SD18, read read-only
//!    via the public `compute_pilot_base_chassis` seam and its
//!    `explanations` field — this module never touches `pilot_compute.rs`
//!    itself, staying inside Epic 7's file-touch partition) for the
//!    class-specific pillars `class_tables.rs` deliberately does not
//!    carry: Bravery (Will-vs-fear, levels 2/6/10/14/18), the ten Bonus
//!    Feat slot-recognition seams (levels 1/2/4/6/8/10/12/14/16/18/20 —
//!    value 0, naming the slot only per `pilot_compute.rs`'s own doc
//!    comment: "grounds the bonus-feat slot, not a general feat-effect or
//!    prerequisite engine"), Armor Training (ranks 1-4 at levels 3/7/11/15),
//!    Weapon Training (ranks 1-4 at levels 5/9/13/17, plus the second/
//!    third/fourth chosen weapon-group explanation-only records), Armor
//!    Mastery (level 19, DR 5/--), and Weapon Mastery (level 20, the
//!    Fighter capstone, critical-multiplier-increase +1).
//!
//! A level transition's automatic feature grants are computed as a diff
//! between the chassis snapshot at `from_level` and at `to_level` (both
//! computed against the same character, varying only the class-level
//! number) — the identical technique Barbarian's cycle established.
//! Fighter's own diff is structurally SIMPLER than Barbarian's: unlike
//! Barbarian's level-gated identity features (Uncanny Dodge, Improved
//! Uncanny Dodge, Tireless Rage), whose explanations always push a record
//! with an explicit "correctly absent" marker text even below the level
//! gate, every one of Fighter's `class_feature.fighter.*` explanations in
//! `explain_fighter_class_features` is pushed ONLY when its level gate is
//! met — the record is entirely absent below the gate, not present with an
//! absence marker. This is the identical shape Bard's level-20 capstone
//! (Deadly Performance) already proved: "the diff's `newly_granted` signal
//! handles a missing `from_explanations` match identically to an explicit
//! 'correctly absent' marker, so no new diff-algorithm branch was needed"
//! (per the progress doc). So this module's diff needs no
//! `is_absent_marker` helper at all: `newly_granted` is simply "no matching
//! id in `from_explanations`."
//!
//! Fighter's base-chassis explanation ids (`class_chassis.base_attack_bonus`,
//! `class_chassis.base_save.fortitude/reflex/will`) predate the later
//! per-class-namespaced convention Barbarian/Bard/Cleric/Druid use (Fighter
//! was this codebase's very first grounded class, SD13-E1/GE-04) and carry
//! no `.fighter.` infix at all — so, unlike Barbarian's cycle, this module
//! needs no explicit `CLASS_TABLE_COVERED_EXPLANATION_IDS` exclusion list:
//! filtering to the `class_feature.fighter.` prefix alone already excludes
//! them (they simply never match). The two remaining `class_chassis.
//! fighter.*` ids (`level_1_hit_points`, `favored_class_bonus_choice`) are
//! both gated to `supported_fighter_level(input) == Some(1)` inside
//! `pilot_compute.rs` — a Level Up transition's `to_level` is always >= 2,
//! so neither id can ever appear in a `to_explanations` snapshot this
//! module diffs; no exclusion is needed for them either.
//!
//! `pick_from_lists` stays empty for Fighter's ten Bonus Feat slots, even
//! though — unlike Barbarian's open-ended Rage Power list — a real feat
//! catalog now exists in `rules_tables::crb::feats` (Epic 3's General/
//! Combat/ItemCreation/Metamagic categories, 185 records). Composing a
//! genuine Fighter bonus-feat candidate list correctly would require
//! filtering that catalog to PF1's "Combat Feats" eligibility rule and
//! cross-checking each candidate's prerequisites against the character's
//! current build via Epic 3's `feat_prereqs` evaluator — a real design
//! surface of its own, not a one-line composition, and out of this bounded
//! cycle's scope. This is a documented, bounded scope note (like
//! Barbarian's Rage Power note), left as this cycle's own
//! `next_required_uplift` pointer for a future cycle, not a blocker on this
//! cycle's `LevelUpPlan`: every other field lands for real.

use crate::rules_core::character_input::{CharacterClassLevel, CharacterInput};
use crate::rules_core::level_up::{Grant, GrantEffect, LevelUpPlan};
use crate::rules_core::pilot_compute::{compute_pilot_base_chassis, ComputationExplanation};
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::class_tables::{class_tables, ClassId, ClassTableRow};
use crate::rules_core::rules_tables::RuleSetId;

const FIGHTER_CLASS_ID: &str = "class:fighter";
const HUMAN_RACE_ID: &str = "race:human";
/// PF1 Core Rulebook Fighter capstone: Weapon Mastery, granted at 20th
/// level (verified against `pilot_compute.rs`'s own
/// `FIGHTER_WEAPON_MASTERY_LEVEL` gate and its doc comment naming "Weapon
/// Mastery" as the level-20 capstone — that constant is private to
/// `pilot_compute.rs`, so this mirrors its already-verified value rather
/// than importing it, the same boundary Barbarian's
/// `BARBARIAN_CAPSTONE_LEVEL` respects).
const FIGHTER_CAPSTONE_LEVEL: u8 = 20;

/// Composes a Fighter `LevelUpPlan` for the transition from `from_level` to
/// `to_level`. Bounded to single-class Human Fighter inputs, mirroring
/// `pilot_compute.rs`'s own `supported_fighter_level` gate (the source this
/// module reads from carries the identical bound, so widening this gate
/// independently would only ever surface an empty chassis snapshot, not
/// real data).
pub fn compute_fighter_level_up_grants(
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) -> LevelUpPlan {
    let mut plan = LevelUpPlan::default();

    let is_single_class_human_fighter = character.chosen.race_id == HUMAN_RACE_ID
        && matches!(
            character.chosen.class_levels.as_slice(),
            [class_level] if class_level.class_id == FIGHTER_CLASS_ID
        );
    if !is_single_class_human_fighter {
        return plan;
    }

    append_class_table_grants(&mut plan, from_level, to_level);
    append_class_feature_grants(&mut plan, character, from_level, to_level);

    plan.capstone_threshold = to_level >= FIGHTER_CAPSTONE_LEVEL;

    plan
}

fn class_table_row(level: u8) -> Option<ClassTableRow> {
    class_tables()
        .into_iter()
        .find(|row| row.class_id == ClassId::Fighter && row.level == level)
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
            name: format!("Fighter {column_key} at level {to_level}"),
            source_table: TableCellRef {
                rule_set: RuleSetId::Crb,
                table: "class_tables".to_owned(),
                row_key: format!("fighter:{to_level}"),
                column_key: column_key.to_owned(),
            },
            effects: vec![GrantEffect {
                description: format!(
                    "{column_key} rises to {to_value} at fighter level {to_level}"
                ),
                value: to_value,
            }],
        });
    }
}

/// Reads `pilot_compute::compute_pilot_base_chassis`'s fighter-specific
/// explanations for a synthetic single-class Fighter input at `level`,
/// keeping every other chosen field (race, ability scores, selected
/// choices) from `character` unchanged. Read-only: `pilot_compute.rs`
/// itself is never touched or re-derived, only its already-public output
/// is read twice (once per level) to diff.
fn fighter_chassis_explanations(character: &CharacterInput, level: u8) -> Vec<ComputationExplanation> {
    let mut probe = character.clone();
    probe.chosen.class_levels = vec![CharacterClassLevel {
        class_id: FIGHTER_CLASS_ID.to_owned(),
        level,
    }];
    compute_pilot_base_chassis(&probe).explanations
}

/// Grants sourced from `pilot_compute::compute_pilot_base_chassis`'s
/// fighter-specific explanations — the class-specific pillars
/// `class_tables.rs` does not carry. Every `class_feature.fighter.*`
/// explanation is pushed only once its level gate is met (no
/// "correctly absent" marker below the gate, per this module's own doc
/// comment), so `newly_granted` is simply "absent from `from_explanations`"
/// — no absence-marker text-check helper is needed here, unlike
/// Barbarian's cycle.
fn append_class_feature_grants(
    plan: &mut LevelUpPlan,
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) {
    let from_explanations = fighter_chassis_explanations(character, from_level);
    let to_explanations = fighter_chassis_explanations(character, to_level);

    for to_explanation in &to_explanations {
        if !to_explanation.id.starts_with("class_feature.fighter.") {
            continue;
        }

        let from_match = from_explanations
            .iter()
            .find(|explanation| explanation.id == to_explanation.id);
        let value_changed = from_match.map(|explanation| explanation.value) != Some(to_explanation.value);
        let newly_granted = from_match.is_none();

        if !value_changed && !newly_granted {
            continue;
        }

        plan.automatic_features.push(Grant {
            name: friendly_name(&to_explanation.id),
            source_table: TableCellRef {
                rule_set: RuleSetId::Crb,
                table: "pilot_compute::explain_fighter_class_features".to_owned(),
                row_key: format!("fighter:{to_level}"),
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
/// explanation id (e.g. `"class_feature.fighter.weapon_mastery"` ->
/// `"weapon mastery"`) — never a hand-typed label naming CRB rule text
/// this module has not itself verified.
fn friendly_name(id: &str) -> String {
    id.trim_start_matches("class_feature.fighter.").replace(['_', '.'], " ")
}
