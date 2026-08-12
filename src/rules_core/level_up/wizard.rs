//! SD-20 Epic 7 (Level Up grant model): Wizard — eleventh and FINAL core
//! class per Step 2's stated order (barbarian, bard, cleric, druid,
//! fighter, monk, paladin, ranger, rogue, sorcerer, wizard;
//! `scope-draft.md` §1.7, `technical-design.md` §2.6). **This cycle
//! closes Epic 7** — all 11 core classes now have a landed
//! `LevelUpPlan`.
//!
//! **Spot-check performed per this cycle's own brief**: `rules_tables::
//! crb::class_tables`'s `CLASS_META` row for `ClassId::Wizard` (`bab:
//! BabProgression::Half`, `good_saves: { fortitude: false, reflex:
//! false, will: true }`) was checked against `pilot_compute.rs`'s own
//! independently-grounded Wizard formulas
//! (`explain_wizard_level1_prepared_spell_baseline`:
//! `wizard_base_attack_bonus = wizard_level_value / 2`,
//! `wizard_good_save = wizard_level_value / 2 + 2` applied to Will only,
//! `wizard_poor_save = wizard_level_value / 3` applied to Fortitude and
//! Reflex — all three independently verified there against the PF1 Core
//! Rulebook Wizard class table's raw level 1-6 rows before ever landing)
//! **before** composing with it. The two sources agree at every level
//! 1-20 (checked by direct arithmetic, not sampled): `class_tables()`'s
//! `base_attack_bonus`/`fort_save`/`ref_save`/`will_save` cells use the
//! identical `BabProgression::Half` → `level / 2` and `save_bonus(level,
//! good) = level / 2 + 2` (good) / `level / 3` (poor) formulas, applied
//! to the identical good/poor save assignment (good Will only, matching
//! Sorcerer's shape exactly, per both sources' own doc comments). **No
//! defect exists for Wizard's `CLASS_META` row** — this module composes
//! `class_tables()` directly for the class-generic BAB/save pillars,
//! mirroring Barbarian's/Fighter's/Monk's/Paladin's/Ranger's/Rogue's/
//! Sorcerer's own composition precedent (NOT Cleric's/Druid's
//! deviation).
//!
//! The class-specific pillars — the Wizard level-1 prepared arcane
//! spell-bearing recognition, Scribe Scroll (the universal,
//! specialization-independent bonus feat every 1st-level Wizard is
//! granted), the arcane school specialization choice recognition (gated
//! on the canonical deterministic Evocation-specialist / Necromancy-and-
//! Transmutation-opposed selection, exactly as
//! `tests/fixtures/rules_core/pf1_human_wizard_level1_sd13_deterministic_input.txt`
//! already establishes), the specialist bonus spell slot flat count
//! ladder, and the two Evocation school powers' flat magnitudes (Intense
//! Spells' bonus damage, Force Missile's uses-per-day pool) — are
//! composed from `pilot_compute::compute_pilot_base_chassis`'s own
//! already-grounded `explain_wizard_level1_prepared_spell_baseline`
//! explanations, the class-specific pillars `class_tables.rs`
//! deliberately does not carry (its own doc comment: "Named per-level
//! features ... are deliberately out of scope for this bootstrap").
//!
//! **Not grounded anywhere in this codebase, so not surfaced here
//! either** (documented, not fabricated, mirroring Sorcerer's own
//! Bloodline Arcana / Arcane Bond boundary and Paladin's Aura of Good /
//! Divine Bond boundary): Wizard's own Arcane Bond (a bonded object or
//! familiar, PF1 Core Rulebook Wizard class feature) has no
//! corresponding explanation record, diagnostic, or any other mention
//! anywhere in `pilot_compute.rs`'s Wizard grounding (grepped and
//! confirmed absent — unlike Sorcerer's bloodline powers, which are at
//! least named-but-unproven via a live claim-blocking diagnostic,
//! Wizard's Arcane Bond is not named at all). This module does not
//! fabricate an Arcane Bond grant. Also ungrounded and not surfaced: the
//! two Evocation school powers' execution machinery (the spell-damage
//! application for Intense Spells, the casting execution for Force
//! Missile), still named by `pilot_compute.rs`'s own live
//! `class_feature.wizard.school_powers_and_opposed_school_cost.unsupported`
//! diagnostic whenever the prepared-spellbook posture below is unmet.
//!
//! **SD-24 Epic 4 correction (criterion 4.1 per-class coverage audit):**
//! this doc comment previously claimed the entire prepared spellbook /
//! spells-prepared / spell-slot posture was permanently ungrounded,
//! citing `pilot_compute.rs`'s `class_spell.wizard.prepared_spellbook.
//! unsupported` diagnostic as though it always fired. That was true when
//! this module first landed (SD-20 Epic 7) but went stale once SD-21
//! E6b.2's `ground_wizard_prepared_spellbook` landed a real, bounded
//! (wizard levels 1-3, canonical Evocation specialization) grounding of
//! the spellbook contents, daily preparation selection, and per-spell-
//! level base/Intelligence-bonus/total spells-per-day counts
//! (`class_spell.wizard.spellbook_contents`, `.daily_preparation`,
//! `.base_spells_per_day.spell_level_N`,
//! `.intelligence_bonus_spells_per_day.spell_level_N`,
//! `.total_spells_per_day.spell_level_N`) — this module's own explanation
//! filter was never updated to include that prefix, so every one of
//! those real facts was silently dropped from the `LevelUpPlan` even
//! once the posture was fully met. This audit cycle adds
//! `WIZARD_SPELL_EXPLANATION_PREFIX` to close that gap; see
//! `docs/release/SD-24-beta-readiness-and-multiclass/artifacts/epic_4/class_wizard_coverage.md`.
//! The posture remains unmet (and the diagnostic above still fires)
//! outside its own bounded range (levels 4+, non-canonical
//! specialization, an empty/inconsistent spellbook, or over-preparation
//! beyond the per-level slot budget) — that boundary is
//! `pilot_compute.rs`'s own, not fabricated wider by this module.
//! `capstone_threshold` still flags the transition crossing
//! PF1's universal character-level cap (every class reaches level 20),
//! but no separate named "capstone" grant is fabricated for it —
//! `class_tables.rs`'s `ClassTableRow` carries no "Special" column at
//! all, so there is no source to compose a named capstone grant from in
//! the first place, exactly mirroring Sorcerer's own "no named capstone"
//! finding.
//!
//! A level transition's automatic feature grants are computed as a pure
//! **value-change diff** between the chassis snapshot at `from_level`
//! and at `to_level` (the identical technique every prior Epic 7 cycle
//! uses), via a plain `Some(from_value) != Some(to_value)` comparison
//! (`from_value` is `None` whenever the id is absent at `from_level`).
//! The `class_chassis.wizard.*` pillars are single flat records present
//! at every supported level once their own gate is satisfied (the
//! recognition / Scribe Scroll / specialization-choice records are flat
//! +0 and never diff once granted; the specialist bonus slot and Intense
//! Spells bonus damage are numeric records whose value rises at specific
//! level thresholds, verified against `pilot_compute.rs`'s own already-
//! grounded formulas, not re-derived here). The `class_spell.wizard.*`
//! pillars (this audit cycle's addition, see above) DO exercise the
//! from-side-miss idiom for real, mirroring Sorcerer's own spontaneous
//! per-spell-level records: they are wholly ABSENT whenever
//! `unmet_wizard_spellbook_conditions` reports any unmet condition at
//! that level (including "not yet accessible at this level" for a
//! prepared spell targeting a spell level the character hasn't reached
//! yet), so a transition that crosses the posture from unmet to met (or
//! that unlocks a new accessible spell level within an already-met
//! posture) surfaces every affected record as newly granted, exactly as
//! Sorcerer's per-spell-level access-ladder crossing does.
//!
//! `resource_pool_change` stays genuinely empty for every Wizard
//! level-up transition: Wizard has no flat daily-use resource pool at
//! all (Force Missile's "3 + Int-mod" pool is level-independent and
//! never changes on a level-up, so it never produces a
//! `ResourcePoolDelta` even though it is a per-day-use pool in PF1's own
//! rules text) — mirroring Sorcerer's own "no resource pool" finding.
//!
//! `pick_from_lists` stays honestly empty: Wizard's only genuinely
//! open-ended per-level choices are WHICH spells are written into the
//! spellbook and the level-5+/level-10+ bonus-feat choice (metamagic,
//! item creation, or Spell Mastery), and no such candidate catalog
//! exists anywhere in `rules_tables::crb` to enumerate real candidates
//! from — the identical "no catalog to enumerate" boundary every prior
//! Epic 7 cycle's own choice-list feature hit.

use crate::rules_core::character_input::{CharacterClassLevel, CharacterInput};
use crate::rules_core::level_up::{Grant, GrantEffect, LevelUpPlan};
use crate::rules_core::pilot_compute::{compute_pilot_base_chassis, ComputationExplanation};
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::class_tables::{class_tables, ClassId, ClassTableRow};
use crate::rules_core::rules_tables::RuleSetId;

const WIZARD_CLASS_ID: &str = "class:wizard";
/// SD-24 Epic 5 (criterion 5.1): the only class `pilot_compute.rs`'s own
/// `is_supported_multiclass_mix` recognizes alongside Wizard. Used only to
/// widen this module's entry gate to a supported Fighter+Wizard mix -- this
/// module still never reads Fighter's own explanations or grants.
const FIGHTER_CLASS_ID: &str = "class:fighter";
const HUMAN_RACE_ID: &str = "race:human";
/// PF1's universal character-level cap. `class_tables.rs`'s
/// `ClassTableRow` carries no "Special" column at all, so this constant
/// only gates the boolean `capstone_threshold` flag, never a fabricated
/// named grant (see this module's own doc comment).
const WIZARD_CHARACTER_LEVEL_CAP: u8 = 20;

const WIZARD_RECOGNITION_ID: &str = "class_chassis.spell_baseline.wizard";
const WIZARD_EXPLANATION_PREFIX: &str = "class_chassis.wizard.";
/// SD-24 Epic 4 (criterion 4.1 per-class coverage audit) finding: the
/// SD-21 E6b.2 `ground_wizard_prepared_spellbook` grounding (spellbook
/// contents, daily preparation, and per-spell-level base/Intelligence-
/// bonus/total spells-per-day) predates this prefix and was never added
/// to it, so every real `class_spell.wizard.*` explanation was silently
/// dropped from the `LevelUpPlan` even once a wizard's prepared-
/// spellbook posture was fully met. Added by this audit cycle —
/// see `docs/release/SD-24-beta-readiness-and-multiclass/artifacts/epic_4/class_wizard_coverage.md`.
const WIZARD_SPELL_EXPLANATION_PREFIX: &str = "class_spell.wizard.";

/// Explanation ids that `append_class_table_grants` already covers from
/// `class_tables()` (the more authoritative, class-generic source per
/// this module's own doc comment, confirmed defect-free by this cycle's
/// own spot-check). `pilot_compute.rs`'s chassis also independently
/// carries standalone base-attack/base-save explanation records for the
/// same facts; skipping them here avoids reporting the identical fact
/// twice under two different provenances.
const CLASS_TABLE_COVERED_EXPLANATION_IDS: [&str; 4] = [
    "class_chassis.wizard.base_attack_bonus",
    "class_chassis.wizard.base_save.fortitude",
    "class_chassis.wizard.base_save.reflex",
    "class_chassis.wizard.base_save.will",
];

/// Composes a Wizard `LevelUpPlan` for the transition from `from_level` to
/// `to_level` -- both of which are Wizard's OWN sub-level within
/// `character.chosen.class_levels` (identical to the character's total level
/// for a solo Wizard; strictly Wizard's own level count for a Fighter+Wizard
/// mix, per this function's own doc comment below). Bounded to Human Wizard
/// inputs, mirroring `pilot_compute.rs`'s own `supported_wizard_level` gate
/// (the source this module reads from carries the identical bound, so
/// widening this gate independently would only ever surface an empty chassis
/// snapshot, not real data).
///
/// SD-24 Epic 5 (criterion 5.1): widened from single-class-Wizard-only to
/// also accept a length-2 Fighter+Wizard mix (mirroring `pilot_compute.rs`'s
/// own `is_supported_multiclass_mix` / `wizard_level_in_mix` widening for the
/// identical combination), since the function body already reads
/// `from_level`/`to_level` from the caller rather than deriving them from
/// `character.chosen.class_levels`'s own level number -- the caller supplies
/// Wizard's own sub-level pair for the mix case exactly as it always has for
/// the solo case, so no other change was needed to compose real data:
/// `append_class_table_grants`/`append_class_feature_grants` were already
/// decoupled from the mix shape.
pub fn compute_wizard_level_up_grants(
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) -> LevelUpPlan {
    let mut plan = LevelUpPlan::default();

    let is_supported_wizard_participant = character.chosen.race_id == HUMAN_RACE_ID
        && is_wizard_or_supported_fighter_wizard_mix(&character.chosen.class_levels);
    if !is_supported_wizard_participant {
        return plan;
    }

    append_class_table_grants(&mut plan, from_level, to_level);
    append_class_feature_grants(&mut plan, character, from_level, to_level);

    plan.capstone_threshold = to_level >= WIZARD_CHARACTER_LEVEL_CAP;

    plan
}

/// True when `class_levels` is either a solo Wizard build, or a length-2
/// Fighter+Wizard multiclass mix (SD-24 Epic 5, criterion 5.1) -- the only
/// multiclass combination `pilot_compute.rs`'s own `is_supported_multiclass_mix`
/// recognizes. Any other shape (a different second class, or 3+ classes)
/// returns `false`, keeping this module's honest boundary identical to
/// `pilot_compute.rs`'s own.
fn is_wizard_or_supported_fighter_wizard_mix(class_levels: &[CharacterClassLevel]) -> bool {
    match class_levels {
        [class_level] => class_level.class_id == WIZARD_CLASS_ID,
        [a, b] => {
            let ids = [a.class_id.as_str(), b.class_id.as_str()];
            ids.contains(&FIGHTER_CLASS_ID) && ids.contains(&WIZARD_CLASS_ID)
        }
        _ => false,
    }
}

fn class_table_row(level: u8) -> Option<ClassTableRow> {
    class_tables()
        .into_iter()
        .find(|row| row.class_id == ClassId::Wizard && row.level == level)
}

/// Grants sourced from `rules_tables::crb::class_tables::class_tables()`
/// — the class-generic BAB/save progression pillars, confirmed defect-free
/// against Wizard's own grounded formulas by this cycle's own spot-check
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
            name: format!("Wizard {column_key} at level {to_level}"),
            source_table: TableCellRef {
                rule_set: RuleSetId::Crb,
                table: "class_tables".to_owned(),
                row_key: format!("wizard:{to_level}"),
                column_key: column_key.to_owned(),
            },
            effects: vec![GrantEffect {
                description: format!(
                    "{column_key} rises to {to_value} at wizard level {to_level}"
                ),
                value: to_value,
            }],
        });
    }
}

/// Reads `pilot_compute::compute_pilot_base_chassis`'s wizard-specific
/// explanations for a synthetic single-class Wizard input at `level`,
/// keeping every other chosen field (race, ability scores, arcane school
/// specialization choices) from `character` unchanged. Read-only:
/// `pilot_compute.rs` itself is never touched or re-derived, only its
/// already-public output is read twice (once per level) to diff — the
/// identical technique every prior Epic 7 cycle uses.
fn wizard_chassis_explanations(
    character: &CharacterInput,
    level: u8,
) -> Vec<ComputationExplanation> {
    let mut probe = character.clone();
    probe.chosen.class_levels = vec![CharacterClassLevel {
        class_id: WIZARD_CLASS_ID.to_owned(),
        level,
    }];
    compute_pilot_base_chassis(&probe).explanations
}

/// Grants sourced from `pilot_compute::compute_pilot_base_chassis`'s
/// Wizard-specific explanations — the class-specific pillars
/// `class_tables.rs` does not carry. A pure value-change diff: unlike
/// Sorcerer's per-spell-level records, every Wizard pillar this module
/// reads is present at every level once its own choice-gate is
/// satisfied (the gate is level-independent), so a plain
/// `Some(from_value) != Some(to_value)` comparison is sufficient — see
/// this module's own doc comment.
fn append_class_feature_grants(
    plan: &mut LevelUpPlan,
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) {
    let from_explanations = wizard_chassis_explanations(character, from_level);
    let to_explanations = wizard_chassis_explanations(character, to_level);

    for to_explanation in &to_explanations {
        let is_wizard_pillar = to_explanation.id == WIZARD_RECOGNITION_ID
            || to_explanation.id.starts_with(WIZARD_EXPLANATION_PREFIX)
            || to_explanation.id.starts_with(WIZARD_SPELL_EXPLANATION_PREFIX);
        let is_covered_elsewhere =
            CLASS_TABLE_COVERED_EXPLANATION_IDS.contains(&to_explanation.id.as_str());
        if !is_wizard_pillar || is_covered_elsewhere {
            // The class-table-covered ids are already granted by
            // `append_class_table_grants`.
            continue;
        }

        let from_value = from_explanations
            .iter()
            .find(|explanation| explanation.id == to_explanation.id)
            .map(|explanation| explanation.value);
        if from_value == Some(to_explanation.value) {
            // Either genuinely unchanged (the common case: the
            // recognition / Scribe Scroll / specialization-choice
            // records are flat +0 and never level-gated, and
            // Intelligence doesn't change on a level-up so Force
            // Missile's pool stays flat), or a pillar whose own
            // threshold has not yet been crossed by this transition.
            continue;
        }

        plan.automatic_features.push(Grant {
            name: friendly_name(&to_explanation.id),
            source_table: TableCellRef {
                rule_set: RuleSetId::Crb,
                table: "pilot_compute::explain_wizard_level1_prepared_spell_baseline".to_owned(),
                row_key: format!("wizard:{to_level}"),
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
/// explanation id (e.g. `"class_chassis.wizard.specialist_bonus_slot"`
/// -> `"specialist bonus slot"`) — never a hand-typed label naming CRB
/// rule text this module has not itself verified. Mirrors every prior
/// Epic 7 cycle's own `friendly_name` helper.
fn friendly_name(id: &str) -> String {
    id.trim_start_matches(WIZARD_EXPLANATION_PREFIX)
        .trim_start_matches(WIZARD_SPELL_EXPLANATION_PREFIX)
        .trim_start_matches("class_chassis.spell_baseline.")
        .replace(['_', '.'], " ")
}
