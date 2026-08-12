//! SD-20 Epic 7 (Level Up grant model): Sorcerer — tenth core class per
//! Step 2's stated order (barbarian, bard, cleric, druid, fighter, monk,
//! paladin, ranger, rogue, sorcerer, wizard; `scope-draft.md` §1.7,
//! `technical-design.md` §2.6).
//!
//! **Spot-check performed per this cycle's own brief**: `rules_tables::
//! crb::class_tables`'s `CLASS_META` row for `ClassId::Sorcerer`
//! (`bab: BabProgression::Half`, `good_saves: { fortitude: false, reflex:
//! false, will: true }`) was checked against
//! `pilot_compute.rs::explain_sorcerer_level1_spell_baseline`'s own
//! independently-grounded formulas (`base_attack_bonus = level / 2`,
//! `good_save = level / 2 + 2` applied to Will only, `poor_save = level /
//! 3` applied to Fortitude and Reflex — both verified there against
//! d20pfsrd and legacy.aonprd.com before ever landing) **before**
//! composing with it. The two sources agree at every level 1-20 (checked
//! by direct arithmetic, not sampled): `class_tables()`'s
//! `base_attack_bonus`/`fort_save`/`ref_save`/`will_save` cells use the
//! identical `BabProgression::Half` → `level / 2` and `save_bonus(level,
//! good) = level / 2 + 2` (good) / `level / 3` (poor) formulas, applied to
//! the identical good/poor save assignment (good Will only). **No second
//! latent `CLASS_META` defect exists for Sorcerer** (unlike the Cleric/
//! Druid `good_saves.fortitude` defect fixed at `28b0e88`) — this module
//! composes `class_tables()` directly for the class-generic BAB/save
//! pillars, mirroring Barbarian's/Fighter's/Monk's/Paladin's own
//! composition precedent (NOT Cleric's/Druid's deviation).
//!
//! The class-specific pillars — Eschew Materials, the bloodline and
//! bloodline-class-skill choice-slot recognitions, the spontaneous
//! spell-level access ladder, the base spells-per-day table, the base
//! spell-save-DC arithmetic, the base spells-known table, the Charisma
//! bonus-spell-slot table, and the integrated base+bonus totals — are
//! composed from `pilot_compute::compute_pilot_base_chassis`'s own
//! already-grounded `explain_sorcerer_level1_spell_baseline` explanations,
//! the class-specific pillars `class_tables.rs` deliberately does not
//! carry (its own doc comment: "Named per-level features ... are
//! deliberately out of scope for this bootstrap").
//!
//! **Bloodline progression** (paragraph corrected 2026-07-29; it previously
//! said the whole bloodline progression was "not grounded anywhere in this
//! codebase", citing the `class_feature.sorcerer.
//! arcane_bond_and_bloodline_progression.unsupported` diagnostic as "grepped
//! and confirmed still claim-blocking" — both statements have since become
//! false and were verified against live code, not against this comment).
//! For the canonical Arcane bloodline, `pilot_compute.rs`'s
//! `ground_sorcerer_arcane_bloodline_progression` now grounds the whole
//! 3rd-level-and-above progression from the corpus: the nine bonus spells,
//! the bonus-feat pool at 7th/13th/19th, and the 3rd/9th/15th/20th-level
//! bloodline powers (Metamagic Adept, New Arcana, School Power, Arcane
//! Apotheosis), so the PF1 Core Rulebook Sorcerer class table's
//! bloodline-specific level-19/20 "Special" column text ("Bloodline feat",
//! "Bloodline feat, bloodline spell") is proven for that bloodline and the
//! diagnostic no longer fires for it. Still **not** grounded, and so still
//! not surfaced here (documented, not fabricated, mirroring Paladin's own
//! Aura of Good / Divine Bond boundary): every other CRB bloodline's
//! progression, for which that diagnostic remains claim-blocking, and the
//! player sub-choices within the Arcane progression itself (which bloodline
//! feat, which New Arcana spells, which School Power school), which carry a
//! separate non-claim-blocking diagnostic. `capstone_threshold` still flags the
//! transition crossing PF1's universal character-level cap (every class
//! reaches level 20), but no separate named "capstone" grant is
//! fabricated for it — the ordinary base-attack / base-save /
//! spell-progression pillars simply keep rising through the same generic
//! diff used at every other level, exactly mirroring Cleric's own "no
//! named capstone" precedent.
//!
//! A level transition's automatic feature grants are computed as a pure
//! **value-change diff** between the chassis snapshot at `from_level` and
//! at `to_level` (the identical technique every prior Epic 7 cycle uses),
//! with no separate "newly granted" branch needed: unlike Barbarian's
//! text-marker on/off identity features, every one of Sorcerer's
//! class-specific pillars is either a flat +0 recognition record (never
//! diffs) or a per-spell-level numeric record that is simply ABSENT from
//! `explanations` below its own access-ladder threshold (Rust's
//! `Vec<ComputationExplanation>` has no entry for an inaccessible spell
//! level at all) — so a from-side miss (`from_value == None`) already
//! differs from any `Some(to_value)`, correctly surfacing a newly
//! ACCESSIBLE spell level's records (base per-day count, save DC, known
//! count, bonus count, total) as grants exactly as if they had "changed"
//! from nothing, with zero special-casing. This is the identical
//! from-side-miss idiom Paladin's own cycle proved handles an
//! explanation id that changes SHAPE across a level gate.
//!
//! `resource_pool_change` stays genuinely empty for every Sorcerer
//! level-up transition: Sorcerer has no flat daily-use resource pool at
//! all (unlike Barbarian's rage-rounds-per-day or Paladin's Smite
//! Evil/Lay on Hands uses-per-day) — its entire class-specific mechanic is
//! the spells-per-day ladder, which is already granted through
//! `automatic_features` above, mirroring Cleric's own "no resource pool"
//! finding (Cleric's pools are flat ability-modifier formulas with no
//! level term; Sorcerer has no comparable pool construct at all).
//!
//! `pick_from_lists` stays honestly empty: Sorcerer's only genuinely
//! open-ended per-level choice is WHICH spells are known (the spells-known
//! COUNT is grounded above; the spell-list content itself is not), and no
//! spell-list candidate catalog exists anywhere in `rules_tables::crb` to
//! enumerate real candidates from — the identical "no catalog to
//! enumerate" boundary every prior Epic 7 cycle's own choice-list feature
//! hit.

use crate::rules_core::character_input::{CharacterClassLevel, CharacterInput};
use crate::rules_core::level_up::{Grant, GrantEffect, LevelUpPlan};
use crate::rules_core::pilot_compute::{compute_pilot_base_chassis, ComputationExplanation};
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::class_tables::{class_tables, ClassId, ClassTableRow};
use crate::rules_core::rules_tables::RuleSetId;

const SORCERER_CLASS_ID: &str = "class:sorcerer";
const HUMAN_RACE_ID: &str = "race:human";
/// PF1's universal character-level cap. Sorcerer's level-19/20 "Special"
/// column names bloodline-specific text that `pilot_compute.rs` leaves
/// named-but-unproven (see this module's own doc comment) — this constant
/// only gates the boolean `capstone_threshold` flag, not a fabricated
/// named grant.
const SORCERER_CHARACTER_LEVEL_CAP: u8 = 20;

const SORCERER_RECOGNITION_ID: &str = "class_chassis.spell_baseline.sorcerer";
const SORCERER_EXPLANATION_PREFIX: &str = "class_chassis.sorcerer.";

/// Explanation ids that `append_class_table_grants` already covers from
/// `class_tables()` (the more authoritative, class-generic source per
/// this module's own doc comment, confirmed defect-free by this cycle's
/// own spot-check). `pilot_compute.rs`'s chassis also independently
/// carries standalone base-attack/base-save explanation records for the
/// same facts; skipping them here avoids reporting the identical fact
/// twice under two different provenances.
const CLASS_TABLE_COVERED_EXPLANATION_IDS: [&str; 4] = [
    "class_chassis.sorcerer.base_attack_bonus",
    "class_chassis.sorcerer.base_save.fortitude",
    "class_chassis.sorcerer.base_save.reflex",
    "class_chassis.sorcerer.base_save.will",
];

/// Composes a Sorcerer `LevelUpPlan` for the transition from `from_level`
/// to `to_level`. Bounded to single-class Human Sorcerer inputs, mirroring
/// `pilot_compute.rs`'s own `supported_sorcerer_level` gate (the source
/// this module reads from carries the identical bound, so widening this
/// gate independently would only ever surface an empty chassis snapshot,
/// not real data).
pub fn compute_sorcerer_level_up_grants(
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) -> LevelUpPlan {
    let mut plan = LevelUpPlan::default();

    let is_single_class_human_sorcerer = character.chosen.race_id == HUMAN_RACE_ID
        && matches!(
            character.chosen.class_levels.as_slice(),
            [class_level] if class_level.class_id == SORCERER_CLASS_ID
        );
    if !is_single_class_human_sorcerer {
        return plan;
    }

    append_class_table_grants(&mut plan, from_level, to_level);
    append_class_feature_grants(&mut plan, character, from_level, to_level);

    plan.capstone_threshold = to_level >= SORCERER_CHARACTER_LEVEL_CAP;

    plan
}

fn class_table_row(level: u8) -> Option<ClassTableRow> {
    class_tables()
        .into_iter()
        .find(|row| row.class_id == ClassId::Sorcerer && row.level == level)
}

/// Grants sourced from `rules_tables::crb::class_tables::class_tables()`
/// — the class-generic BAB/save progression pillars, confirmed defect-free
/// against Sorcerer's own grounded formula by this cycle's own spot-check
/// (see this module's own doc comment).
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
            name: format!("Sorcerer {column_key} at level {to_level}"),
            source_table: TableCellRef {
                rule_set: RuleSetId::Crb,
                table: "class_tables".to_owned(),
                row_key: format!("sorcerer:{to_level}"),
                column_key: column_key.to_owned(),
            },
            effects: vec![GrantEffect {
                description: format!(
                    "{column_key} rises to {to_value} at sorcerer level {to_level}"
                ),
                value: to_value,
            }],
        });
    }
}

/// Reads `pilot_compute::compute_pilot_base_chassis`'s sorcerer-specific
/// explanations for a synthetic single-class Sorcerer input at `level`,
/// keeping every other chosen field (race, ability scores, bloodline
/// choices) from `character` unchanged. Read-only: `pilot_compute.rs`
/// itself is never touched or re-derived, only its already-public output
/// is read twice (once per level) to diff — the identical technique every
/// prior Epic 7 cycle uses.
fn sorcerer_chassis_explanations(
    character: &CharacterInput,
    level: u8,
) -> Vec<ComputationExplanation> {
    let mut probe = character.clone();
    probe.chosen.class_levels = vec![CharacterClassLevel {
        class_id: SORCERER_CLASS_ID.to_owned(),
        level,
    }];
    compute_pilot_base_chassis(&probe).explanations
}

/// Grants sourced from `pilot_compute::compute_pilot_base_chassis`'s
/// Sorcerer-specific explanations — the class-specific pillars
/// `class_tables.rs` does not carry. A pure value-change diff: a
/// from-side miss (the id absent from `from_explanations`, e.g. a spell
/// level not yet accessible) already differs from any `Some(to_value)`,
/// so newly-accessible spell-level records are correctly surfaced as
/// grants with zero special-casing — see this module's own doc comment.
fn append_class_feature_grants(
    plan: &mut LevelUpPlan,
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) {
    let from_explanations = sorcerer_chassis_explanations(character, from_level);
    let to_explanations = sorcerer_chassis_explanations(character, to_level);

    for to_explanation in &to_explanations {
        let is_sorcerer_pillar = to_explanation.id == SORCERER_RECOGNITION_ID
            || to_explanation.id.starts_with(SORCERER_EXPLANATION_PREFIX);
        let is_covered_elsewhere =
            CLASS_TABLE_COVERED_EXPLANATION_IDS.contains(&to_explanation.id.as_str());
        if !is_sorcerer_pillar || is_covered_elsewhere {
            // The class-table-covered ids are already granted by
            // `append_class_table_grants`.
            continue;
        }

        let from_value = from_explanations
            .iter()
            .find(|explanation| explanation.id == to_explanation.id)
            .map(|explanation| explanation.value);
        if from_value == Some(to_explanation.value) {
            // Either genuinely unchanged (the common case: Charisma
            // doesn't change on a level-up, so the spell-save-DC and
            // bonus-spell-slot formulas stay flat wherever the access
            // ladder itself doesn't widen), or present-and-identical at
            // both ends (the +0 recognition / Eschew Materials / bloodline
            // choice records, which are never level-gated).
            continue;
        }

        plan.automatic_features.push(Grant {
            name: friendly_name(&to_explanation.id),
            source_table: TableCellRef {
                rule_set: RuleSetId::Crb,
                table: "pilot_compute::explain_sorcerer_level1_spell_baseline".to_owned(),
                row_key: format!("sorcerer:{to_level}"),
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
/// explanation id (e.g.
/// `"class_chassis.sorcerer.spontaneous.spell_level_access"` ->
/// `"spontaneous spell level access"`) — never a hand-typed label naming
/// CRB rule text this module has not itself verified. Mirrors every prior
/// Epic 7 cycle's own `friendly_name` helper.
fn friendly_name(id: &str) -> String {
    id.trim_start_matches(SORCERER_EXPLANATION_PREFIX)
        .trim_start_matches("class_chassis.spell_baseline.")
        .replace(['_', '.'], " ")
}
