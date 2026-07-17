//! SD-20 Epic 7 (Level Up grant model): Druid — fourth core class per
//! Step 2's stated order (barbarian, bard, cleric, druid, ...;
//! `scope-draft.md` §1.7, `technical-design.md` §2.6).
//!
//! Composes with `pilot_compute::compute_pilot_base_chassis`'s own
//! `explain_druid_level1_spell_baseline` output (SD13/SD18, read
//! read-only via the public `compute_pilot_base_chassis` seam and its
//! `explanations` field — this module never touches `pilot_compute.rs`
//! itself, staying inside Epic 7's file-touch partition) for every pillar
//! this cycle grounds: the class-generic base-attack/base-save
//! progression AND the class-specific pillars (Wild Empathy, Nature
//! Sense, Woodland Stride, Trackless Step, Resist Nature's Lure, Venom
//! Immunity, A Thousand Faces, Timeless Body).
//!
//! **Deliberate deviation from the Barbarian cycle's pattern (not a
//! blocker, a documented finding):** Barbarian's cycle (`barbarian.rs`)
//! sourced base-attack/base-save from
//! `rules_tables::crb::class_tables::class_tables()` and only its
//! class-specific pillars from `pilot_compute.rs`'s chassis explanations.
//! This module does NOT read `class_tables()` at all: `class_tables.rs`'s
//! `CLASS_META` entry for `ClassId::Druid` carries `good_saves:
//! GoodSaves { fortitude: false, reflex: false, will: true }`, which
//! computes a *poor* Fortitude save (`classlevel / 3`). That contradicts
//! `pilot_compute.rs`'s own `explain_druid_level1_spell_baseline`
//! formula — verified independently against two primary PF1 sources
//! (d20pfsrd and legacy.aonprd.com, cross-checked against the level 4/5
//! base-attack values to disambiguate the BAB fraction before any code
//! was written; see `tests/sd13_druid_base_attack_and_saves.rs` and
//! `tests/sd18_druid_level15_widening.rs`) — good Fortitude
//! (`classlevel/2+2`), poor Reflex (`classlevel/3`), good Will
//! (`classlevel/2+2`), the same save shape as PF1's Cleric. Composing
//! with the buggy `class_tables()` column would land a wrong number on a
//! future printed character sheet. `class_tables.rs` is outside this
//! cycle's file-touch partition (Epic 7 cycles may touch only
//! `level_up.rs` / `level_up/<class>.rs` / the cycle's own test file), so
//! the `CLASS_META` bug itself is not fixed here — every pillar this
//! module grants instead reads the single already-grounded, verified,
//! internally-consistent `pilot_compute.rs` source, and the discrepancy
//! is flagged in the progress doc for a future SD-19 cycle to correct
//! (Cleric's `CLASS_META` row shows the identical `fortitude: false`
//! shape, so the same fix likely applies there too).
//!
//! A level transition's automatic feature grants are computed as a
//! **diff** between the chassis snapshot at `from_level` and at
//! `to_level` (both computed against the same character, varying only
//! the class-level number), not by re-deriving PF1's own druid
//! class-progression rules a second time — the identical idiom
//! `barbarian.rs` established. Two signals drive the diff:
//!
//! - **Value change** (e.g. base attack bonus 0 -> 1, Wild Empathy 2 ->
//!   3): catches every magnitude-rising pillar.
//! - **Grant-state change** for the bounded identity/recognition
//!   features whose value is always 0 whether granted or not (Woodland
//!   Stride, Trackless Step, Venom Immunity, A Thousand Faces, Timeless
//!   Body): each such explanation's `detail` text deterministically
//!   distinguishes "correctly absent at level N by PF1 Core Rulebook
//!   level gate" from "granted at druid level N" (see
//!   `pilot_compute.rs`'s own `explain_druid_level1_spell_baseline`) —
//!   reading that already-computed marker, not re-deriving the level
//!   gate.
//!
//! `pick_from_lists` and `resource_pool_change` stay empty: Druid's only
//! per-level choice (the nature bond) is a one-time level-1 selection
//! already recognized as chosen input (not a recurring pick-list), and
//! Wild Shape uses/day was checked and confirmed NOT flat by
//! `pilot_compute.rs`'s own doc comment ("a full shapeshifting subsystem
//! ... with no execution engine anywhere in this codebase") — no
//! resource-pool value exists anywhere in this repo to compose from.
//! `MAX_SUPPORTED_DRUID_LEVEL` in `pilot_compute.rs` bounds the grounded
//! chassis data to Druid level 15; a transition whose `to_level` exceeds
//! that ceiling honestly produces no `automatic_features` (the chassis
//! probe returns no druid-namespaced explanations there), while
//! `capstone_threshold` still reports correctly — it is a pure PF1 Core
//! Rulebook level-number fact (level 20 is every core class's capstone
//! level), not derived from any per-class grounded data source, so it is
//! not fabrication to report it beyond Druid's own grounded ceiling.

use crate::rules_core::character_input::{CharacterClassLevel, CharacterInput};
use crate::rules_core::level_up::{Grant, GrantEffect, LevelUpPlan};
use crate::rules_core::pilot_compute::{compute_pilot_base_chassis, ComputationExplanation};
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::RuleSetId;

const DRUID_CLASS_ID: &str = "class:druid";
const HUMAN_RACE_ID: &str = "race:human";
/// PF1 Core Rulebook capstone level, universal across every core class
/// (`LevelUpPlan.capstone_threshold`'s own doc comment). Druid's own
/// grounded chassis data stops at `MAX_SUPPORTED_DRUID_LEVEL = 15`
/// (`pilot_compute.rs`) — this constant is not a Druid-specific named
/// ability, only the shared PF1 level-cap fact, mirroring
/// `barbarian.rs`'s `BARBARIAN_CAPSTONE_LEVEL` for the same reason.
const DRUID_CAPSTONE_LEVEL: u8 = 20;

/// Explanation id prefixes `pilot_compute::explain_druid_level1_spell_baseline`
/// uses for every pillar this module grounds. Deliberately excludes
/// `"class_chassis.spell_baseline.druid"` (a level-mentioning but
/// value-always-0, always-granted identity record — never produces a
/// grant since it never changes between `from_level` and `to_level`) by
/// simply not matching its different id shape.
fn is_druid_pillar_id(id: &str) -> bool {
    id.starts_with("class_chassis.druid.") || id.starts_with("class_feature.druid.")
}

/// Composes a Druid `LevelUpPlan` for the transition from `from_level` to
/// `to_level`. Bounded to single-class Human Druid inputs, mirroring
/// `pilot_compute.rs`'s own `supported_druid_level` gate (the source
/// this module reads from carries the identical bound, so widening this
/// gate independently would only ever surface an empty chassis snapshot,
/// not real data).
pub fn compute_druid_level_up_grants(
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) -> LevelUpPlan {
    let mut plan = LevelUpPlan::default();

    let is_single_class_human_druid = character.chosen.race_id == HUMAN_RACE_ID
        && matches!(
            character.chosen.class_levels.as_slice(),
            [class_level] if class_level.class_id == DRUID_CLASS_ID
        );
    if !is_single_class_human_druid {
        return plan;
    }

    append_druid_chassis_grants(&mut plan, character, from_level, to_level);

    plan.capstone_threshold = to_level >= DRUID_CAPSTONE_LEVEL;

    plan
}

/// Reads `pilot_compute::compute_pilot_base_chassis`'s druid-specific
/// explanations for a synthetic single-class Druid input at `level`,
/// keeping every other chosen field (race, ability scores) from
/// `character` unchanged. Read-only: `pilot_compute.rs` itself is never
/// touched or re-derived, only its already-public output is read twice
/// (once per level) to diff.
fn druid_chassis_explanations(character: &CharacterInput, level: u8) -> Vec<ComputationExplanation> {
    let mut probe = character.clone();
    probe.chosen.class_levels = vec![CharacterClassLevel {
        class_id: DRUID_CLASS_ID.to_owned(),
        level,
    }];
    compute_pilot_base_chassis(&probe).explanations
}

/// `explain_druid_level1_spell_baseline`'s own level-gated explanations
/// (Woodland Stride, Trackless Step, Resist Nature's Lure) always word
/// their "not yet granted" branch with this exact marker phrase. Reading
/// it is reading an already-computed, already-grounded fact — not
/// re-deriving the level gate that produced it. Identical idiom to
/// `barbarian.rs`'s `is_absent_marker`.
fn is_absent_marker(detail: &str) -> bool {
    detail.contains("correctly absent")
}

/// Grants sourced from `pilot_compute::compute_pilot_base_chassis`'s
/// druid-specific explanations — every pillar this cycle grounds (see
/// this module's own doc comment for why `class_tables()` is not used).
fn append_druid_chassis_grants(
    plan: &mut LevelUpPlan,
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) {
    let from_explanations = druid_chassis_explanations(character, from_level);
    let to_explanations = druid_chassis_explanations(character, to_level);

    for to_explanation in &to_explanations {
        if !is_druid_pillar_id(&to_explanation.id) {
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
                table: "pilot_compute::explain_druid_level1_spell_baseline".to_owned(),
                row_key: format!("druid:{to_level}"),
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
/// explanation id (e.g. `"class_feature.druid.woodland_stride"` ->
/// `"woodland stride"`) — never a hand-typed label naming CRB rule text
/// this module has not itself verified. Identical idiom to
/// `barbarian.rs`'s `friendly_name`.
fn friendly_name(id: &str) -> String {
    id.trim_start_matches("class_chassis.druid.")
        .trim_start_matches("class_feature.druid.")
        .replace(['_', '.'], " ")
}
