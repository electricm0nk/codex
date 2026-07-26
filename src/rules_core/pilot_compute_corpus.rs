//! SD-19 corpus-aware compute seam.
//!
//! Wraps [`compute_pilot_base_chassis`] with corpus-derived spell-school
//! and equipment contributions, without modifying `pilot_compute.rs`
//! itself — every landed SD-18 cycle keeps calling the chassis function
//! unchanged. See
//! `programs/codex/requirements/SD-19-corpus-aware-compute-seam/technical-design.md`
//! §1 for the design.
//!
//! This module's own `Pf1SchoolId`/`RuleSetId` types come from `rules_tables`
//! (the SD-19 foundation slice); `PilotReceipt` in the doctrine doc's
//! illustrative code does not exist in this repo — the real chassis
//! function returns `PilotBaseChassisComputation`, which is what `base`
//! wraps here.
//!
//! Resolution here is deliberately **generic**: it reads a resolved
//! corpus record's own school/category rather than dispatching through
//! per-school/per-category files (operator-confirmed 2026-07-16, see the
//! review note atop `technical-design.md`). This satisfies the
//! seam-shapes-correctness proof at slice-ship, before any loop cycle has
//! landed. Future SD-19 cycles ground *evidence tier* (matrix row
//! promotion, exhaustive per-school/category coverage) rather than
//! writing new dispatch code.

use std::collections::BTreeMap;

use crate::rules_core::character_input::{ActiveState, CharacterInput, EquipmentSelection};
use crate::rules_core::equipment_effects::{compute_equipment_effects, EquipmentEffects};
use crate::rules_core::equipment_resolver::equipment_id_resolve;
use crate::rules_core::pilot_compute::{
    choice_selection, compute_pilot_base_chassis, fighter_armor_training, fighter_level_in_mix,
    fighter_weapon_training_attack_bonus, has_supported_class_chassis, require_active_state,
    require_selected_skill_rank, selected_skill_climb_is_class_skill,
    selected_skill_intimidate_is_class_skill, selected_skill_swim_is_class_skill,
    supported_fighter_level, PilotBaseChassisComputation, ARMOR_CLASS_BASE, CLASS_SKILL_BONUS,
    CLIMB_SKILL_ID, DODGE_AC_BONUS, DODGE_FEAT_ID, FIGHTER_BONUS_FEAT_CHOICE_ID, FIGHTER_CLASS_ID,
    INTIMIDATE_SKILL_ID, LONGSWORD_ITEM_ID, MAX_SUPPORTED_FIGHTER_LEVEL, MAX_SUPPORTED_WIZARD_LEVEL,
    POWER_ATTACK_ITEM_ID, SELECTED_SKILL_RANK, SWIM_SKILL_ID, WEAPON_FOCUS_FEAT_ID,
    WEAPON_FOCUS_LONGSWORD_SELECTION, WEAPON_FOCUS_TO_HIT_BONUS, WIZARD_CLASS_ID,
};
use crate::rules_core::rules_tables::crb::spell_list::Pf1SchoolId;
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::source_content::SourcePackageContent;
use crate::rules_core::spell_resolver::spell_id_resolve;

/// Corpus-augmented compute result. Wraps the existing chassis
/// computation and adds a corpus-derived section carrying the
/// spell-coverage and equipped-items contributions produced by the seam.
#[derive(Debug, Clone, PartialEq)]
pub struct CorpusPilotReceipt {
    /// The unchanged chassis computation from `compute_pilot_base_chassis`.
    pub base: PilotBaseChassisComputation,
    /// The corpus-derived contributions grounded by this call. Empty when
    /// the input carried no `spells_selected` or `equipment_selections`
    /// entries that resolved against the corpus.
    pub corpus_derived: CorpusDerivedSection,
}

/// Per-domain corpus-derived contributions.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CorpusDerivedSection {
    pub school_coverage: BTreeMap<Pf1SchoolId, SchoolCoverage>,
    pub equipped_items: Vec<ResolvedEquipment>,
    /// v0.6 alpha swarm item 1, shape (c): the real, corpus-resolved
    /// aggregate equipment-effect totals (`armor_class_delta`,
    /// `armor_check_penalty_total`, `max_dex_cap`, `spell_failure_chance`,
    /// and the bounded single-weapon `attack_bonus_delta`) for the
    /// character's currently `EquippedActive` items, via the same
    /// already-existing `equipment_effects::compute_equipment_effects`
    /// `contract::to_pilot_receipt` already calls for `PilotReceipt`.
    /// Deliberately NOT wired into the claim-gated `PilotBaseChassisComputation`
    /// pillars (`baseline_armor_class`, `baseline_melee_attack_bonus`,
    /// `selected_skill_modifiers`) this receipt's own `base` field carries —
    /// this is an additive, explicitly-not-claim-gated section, the same
    /// posture `equipped_items` above already has.
    pub equipment_effects: EquipmentEffects,
    /// v0.6 alpha swarm (QA finding, 2026-07-24): every `spells_selected`
    /// entry whose `spell_id` did NOT resolve against `corpus` -- verbatim,
    /// not deduplicated against `school_coverage`. Before this field, an
    /// unresolved selection simply vanished from every corpus-derived
    /// output with no trace at all (the loop below `continue`s past it) --
    /// for the desktop app specifically, whose only bundled `corpus` is a
    /// deliberately tiny ~4-record demo fixture
    /// (`apps/desktop/src-tauri/src/corpus_fixtures.rs`), this meant a
    /// real, disk-persisted selection outside that tiny bundle looked
    /// identical to "nothing selected" -- a silent, honest-looking display
    /// bug, not a data-loss bug (the underlying `CharacterInput` field was
    /// never touched). This field makes that absence traceable rather than
    /// silent, matching this crate's "never fabricate, never silently
    /// drop" discipline. Whether/how a caller surfaces this to a user is
    /// its own decision -- this field only guarantees the information
    /// exists to make that decision with.
    pub unresolved_spell_ids: Vec<String>,
    /// Mirrors `unresolved_spell_ids` exactly, for
    /// `equipment_selections[].item_id` that did not resolve against
    /// `corpus`.
    pub unresolved_equipment_item_ids: Vec<String>,
}

/// A canonical Paizo-table-cell reference. Non-`None` proves the corpus
/// record the seam resolved lives at a specific cell of the rule-set's
/// source-book table, not just "a corpus record exists."
#[derive(Debug, Clone, PartialEq)]
pub struct TableCellRef {
    pub rule_set: RuleSetId,
    pub table: String,
    pub row_key: String,
    pub column_key: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SchoolCoverage {
    pub school: Pf1SchoolId,
    /// Corpus spell identities (see `spell_resolver`'s doc comment on why
    /// this is the spell's `name`, not a `KEY:` token), sorted.
    pub spells: Vec<String>,
    pub table_cell: Option<TableCellRef>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedEquipment {
    pub item_id: String,
    pub equipment_record_name: String,
    pub equipment_record_key: String,
    pub derived_stats: DerivedEquipmentStats,
    pub table_cell: Option<TableCellRef>,
    /// v0.6 alpha swarm items 1+27 sub-task 6: the resolved records for
    /// this selection's own `applied_modifiers` item_ids (see
    /// `character_input::EquipmentSelection`'s doc comment) -- e.g. a
    /// resolved "+1 Enhancement to Weapon" attached to this Longsword.
    /// Empty for a selection with no `applied_modifiers`, or whose
    /// modifiers all failed to resolve (those land in
    /// `CorpusDerivedSection::unresolved_equipment_item_ids` instead, same
    /// as any other unresolved equipment identity).
    pub applied_modifiers: Vec<ResolvedEquipment>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DerivedEquipmentStats {
    pub armor_bonus: Option<i16>,
    pub attack_bonus: Option<i16>,
    pub max_dex: Option<i16>,
    pub spell_failure: Option<f32>,
}

/// The corpus-aware compute seam. Computes the unchanged chassis, then
/// resolves every `spells_selected` / `equipment_selections` entry
/// against the corpus, grouping resolved spells by school and wrapping
/// resolved equipment with its (currently unpopulated — bounded-baseline
/// non-goal, see `scope-draft.md` §1.1) derived stats.
pub fn compute_pilot_with_corpus(
    input: &CharacterInput,
    corpus: &SourcePackageContent,
) -> CorpusPilotReceipt {
    let base = compute_pilot_base_chassis(input);

    let mut school_coverage: BTreeMap<Pf1SchoolId, SchoolCoverage> = BTreeMap::new();
    let mut unresolved_spell_ids = Vec::new();
    for selection in &input.chosen.spells_selected {
        let Some((record, table_cell)) =
            spell_id_resolve(&selection.spell_id, RuleSetId::Crb, corpus)
        else {
            unresolved_spell_ids.push(selection.spell_id.clone());
            continue;
        };
        let Some(school) = record
            .school
            .as_deref()
            .and_then(Pf1SchoolId::from_corpus_str)
        else {
            continue;
        };
        let entry = school_coverage.entry(school).or_insert_with(|| SchoolCoverage {
            school,
            spells: Vec::new(),
            table_cell: table_cell.clone(),
        });
        if !entry.spells.contains(&selection.spell_id) {
            entry.spells.push(selection.spell_id.clone());
        }
        if entry.table_cell.is_none() {
            entry.table_cell = table_cell;
        }
    }
    for coverage in school_coverage.values_mut() {
        coverage.spells.sort();
    }

    let mut equipped_items = Vec::new();
    let mut unresolved_equipment_item_ids = Vec::new();
    for selection in &input.chosen.equipment_selections {
        let Some((record, table_cell)) =
            equipment_id_resolve(&selection.item_id, RuleSetId::Crb, corpus)
        else {
            unresolved_equipment_item_ids.push(selection.item_id.clone());
            continue;
        };
        let key = crate::rules_core::equipment_resolver::equipment_key_token(record)
            .unwrap_or(&record.name)
            .to_string();

        // v0.6 alpha swarm items 1+27 sub-task 6: resolve this selection's
        // own `applied_modifiers` the same way the selection itself just
        // resolved -- an unresolvable modifier item_id lands in the same
        // flat `unresolved_equipment_item_ids` list a top-level
        // unresolvable selection would (frontend's existing
        // `UnresolvedNotice` already renders off that one list, so this
        // needs no new surfaced-list wiring).
        let mut applied_modifiers = Vec::new();
        for modifier_item_id in &selection.applied_modifiers {
            let Some((modifier_record, modifier_table_cell)) =
                equipment_id_resolve(modifier_item_id, RuleSetId::Crb, corpus)
            else {
                unresolved_equipment_item_ids.push(modifier_item_id.clone());
                continue;
            };
            let modifier_key = crate::rules_core::equipment_resolver::equipment_key_token(modifier_record)
                .unwrap_or(&modifier_record.name)
                .to_string();
            applied_modifiers.push(ResolvedEquipment {
                item_id: modifier_item_id.clone(),
                equipment_record_name: modifier_record.name.clone(),
                equipment_record_key: modifier_key,
                derived_stats: DerivedEquipmentStats::default(),
                table_cell: modifier_table_cell,
                applied_modifiers: Vec::new(),
            });
        }

        equipped_items.push(ResolvedEquipment {
            item_id: selection.item_id.clone(),
            equipment_record_name: record.name.clone(),
            equipment_record_key: key,
            derived_stats: DerivedEquipmentStats::default(),
            table_cell,
            applied_modifiers,
        });
    }

    // v0.6 alpha swarm item 1, shape (c): mirrors `contract::to_pilot_receipt`'s
    // own `EquippedActive`-only filtering exactly (a `SelectedInactive` or
    // `Absent` item contributes no armor/attack/skill effect, only
    // `equipped_items` above tracks identity for everything owned).
    let equipped: Vec<EquipmentSelection> = input
        .chosen
        .equipment_selections
        .iter()
        .filter(|selection| selection.active_state == ActiveState::EquippedActive)
        .cloned()
        .collect();
    let equipment_effects = compute_equipment_effects(&equipped, corpus);

    CorpusPilotReceipt {
        base,
        corpus_derived: CorpusDerivedSection {
            school_coverage,
            equipped_items,
            equipment_effects,
            unresolved_spell_ids,
            unresolved_equipment_item_ids,
        },
    }
}

/// v0.6 alpha swarm items 1+27 sub-task 4: real, corpus-resolved
/// `baseline_melee_attack_bonus`/`baseline_armor_class`, replacing
/// `pilot_compute::compute_combat_baseline`'s hardcoded arithmetic (which
/// has zero corpus access by construction, living in the headless layer).
///
/// **Widens armor/shield to any resolvable loadout** -- every
/// `EquippedActive` selection must resolve against `corpus` into a known
/// equipment-table category (mirroring the "corpus-resolved-with-known-math
/// or absent" requirement `items-1-and-27-scoping.md` Part B describes),
/// instead of requiring the exact Chain Shirt / shield-absent posture. Real
/// `armor_class_delta`/`max_dex_cap` (`equipment_effects::compute_equipment_effects`,
/// already summing every resolved armor+shield item) replace the old
/// Chain-Shirt-specific constants.
///
/// **Deliberately keeps the weapon requirement unchanged** (must still be
/// exactly the Longsword) and both feat requirements unchanged (`Dodge`,
/// `Weapon Focus`, contributing their existing fixed bonuses) -- weapon-
/// loadout widening is a distinct, later increment: Fighter Weapon
/// Training's attack bonus is itself hardcoded to the Longsword's "Heavy
/// Blades" weapon-training group, and widening the weapon would need a
/// real weapon-to-training-group mapping this crate does not have yet.
/// Whether feat *requirements* (as opposed to feat *effects*, already
/// covered by the "never block on an unrecognized feat effect" policy) get
/// widened is a separate, undecided question, not part of this slice.
///
/// The one real, meaningful attack-bonus widening this DOES capture: the
/// required Longsword may now carry `applied_modifiers` (sub-task 1/2/6),
/// and a real attached enhancement's `to_hit_bonus` folds into
/// `melee_attack_bonus` for the first time -- today's hardcoded formula has
/// no awareness of equipment modifiers at all.
///
/// Returns `Err` with the unmet-condition messages (mirroring
/// `unmet_combat_posture_conditions`'s own message shape) when any
/// requirement fails, including the case where 2+ real weapons are
/// equipped alongside the required Longsword (the attack bonus is
/// genuinely ambiguous then, same honest-absence reasoning
/// `EquipmentEffects.attack_bonus_delta` already uses).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CorpusAwareCombatBaseline {
    pub melee_attack_bonus: i16,
    pub armor_class: i16,
}

pub fn compute_combat_baseline_from_corpus(
    base: &PilotBaseChassisComputation,
    input: &CharacterInput,
    corpus: &SourcePackageContent,
) -> Result<CorpusAwareCombatBaseline, Vec<String>> {
    let chosen = &input.chosen;
    let mut unmet = Vec::new();

    if !has_supported_class_chassis(input) {
        unmet.push(format!(
            "missing supported {FIGHTER_CLASS_ID} levels 1-{MAX_SUPPORTED_FIGHTER_LEVEL} or \
             {WIZARD_CLASS_ID} levels 1-{MAX_SUPPORTED_WIZARD_LEVEL} chassis"
        ));
    }

    require_active_state(input, LONGSWORD_ITEM_ID, ActiveState::EquippedActive, &mut unmet);
    require_active_state(input, POWER_ATTACK_ITEM_ID, ActiveState::SelectedInactive, &mut unmet);

    if !chosen.selected_feats.iter().any(|f| f == DODGE_FEAT_ID) {
        unmet.push(format!("missing selected feat {DODGE_FEAT_ID}"));
    }
    if !chosen.selected_feats.iter().any(|f| f == WEAPON_FOCUS_FEAT_ID) {
        unmet.push(format!("missing selected feat {WEAPON_FOCUS_FEAT_ID}"));
    }
    if fighter_level_in_mix(input).is_some() {
        let fighter_bonus_selection = choice_selection(input, FIGHTER_BONUS_FEAT_CHOICE_ID);
        if fighter_bonus_selection != Some(WEAPON_FOCUS_LONGSWORD_SELECTION) {
            unmet.push(format!(
                "{FIGHTER_BONUS_FEAT_CHOICE_ID} selection must be \
                 {WEAPON_FOCUS_LONGSWORD_SELECTION}, got {fighter_bonus_selection:?}"
            ));
        }
    }

    // v0.6 alpha swarm sub-task 4 (real regression found and fixed during
    // sub-task 5's re-verification): deliberately does NOT require every
    // EquippedActive selection to resolve against corpus. The desktop app's
    // real bundled demo corpus (`corpus_fixtures.rs`) has only 2 equipment
    // records -- an earlier version of this function hard-blocked on any
    // OTHER equipped item (e.g. a Dagger added on top of the required
    // Longsword) failing to resolve, which regressed every existing test
    // that equips a second, non-fixed item the old exact-posture gate never
    // cared about. `compute_equipment_effects` already tolerates an
    // unresolvable selection gracefully (skips it, contributes nothing to
    // any total) -- this function trusts that same tolerance rather than
    // re-imposing a stricter, blocking requirement on top of it. The one
    // real requirement equipment resolution DOES need to enforce is below:
    // genuine 2-real-weapon ambiguity, which `attack_bonus_delta` already
    // detects on its own (an unresolvable item is never counted as a
    // weapon, so it can't manufacture a false ambiguity here either).
    let equipped: Vec<EquipmentSelection> = chosen
        .equipment_selections
        .iter()
        .filter(|selection| selection.active_state == ActiveState::EquippedActive)
        .cloned()
        .collect();
    let effects = compute_equipment_effects(&equipped, corpus);

    // Longsword is required above, so `None` here specifically means a
    // second real (successfully resolved) weapon is also equipped --
    // genuinely ambiguous, same honest-absence reasoning `attack_bonus_delta`
    // itself already uses.
    let Some(attack_bonus_delta) = effects.attack_bonus_delta else {
        unmet.push(
            "more than one real weapon is equipped alongside the required Longsword; the \
             resulting attack bonus is ambiguous"
                .to_owned(),
        );
        return Err(unmet);
    };

    if !unmet.is_empty() {
        return Err(unmet);
    }

    let level = supported_fighter_level(input).unwrap_or(1);
    let strength_modifier = base.ability_modifiers.strength;
    let weapon_training_bonus = fighter_weapon_training_attack_bonus(input, level);
    let melee_attack_bonus = base.base_attack_bonus
        + strength_modifier
        + WEAPON_FOCUS_TO_HIT_BONUS
        + weapon_training_bonus
        + attack_bonus_delta;

    let armor_training = fighter_armor_training(level);
    let effective_max_dex = effects.max_dex_cap.map(|cap| cap + armor_training.max_dex_increase);
    let dexterity_modifier = base.ability_modifiers.dexterity;
    let dexterity_contribution = match effective_max_dex {
        Some(cap) => dexterity_modifier.min(cap),
        None => dexterity_modifier,
    };
    let armor_class =
        ARMOR_CLASS_BASE + effects.armor_class_delta + dexterity_contribution + DODGE_AC_BONUS;

    Ok(CorpusAwareCombatBaseline { melee_attack_bonus, armor_class })
}

/// v0.6 alpha swarm items 1+27 sub-task 4: real, corpus-resolved
/// Climb/Intimidate/Swim selected-skill modifiers, replacing
/// `pilot_compute::compute_selected_skill_modifiers`'s hardcoded Chain
/// Shirt armor-check penalty. Widens the exact "Chain Shirt equipped"
/// requirement to any resolvable armor loadout (or none), using real
/// `armor_check_penalty_total` (`equipment_effects::compute_equipment_effects`)
/// in place of the old Chain-Shirt-specific constant -- the skill-allocation
/// posture itself (exactly Climb/Intimidate/Swim at rank 1, no other
/// allocations) is unchanged, same as `compute_combat_baseline_from_corpus`
/// leaves the weapon/feat requirements unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CorpusAwareSelectedSkillModifiers {
    pub climb: i16,
    pub intimidate: i16,
    pub swim: i16,
}

pub fn compute_selected_skill_modifiers_from_corpus(
    base: &PilotBaseChassisComputation,
    input: &CharacterInput,
    corpus: &SourcePackageContent,
) -> Result<CorpusAwareSelectedSkillModifiers, Vec<String>> {
    let allocations = &input.chosen.skill_allocations;
    let mut unmet = Vec::new();

    if !has_supported_class_chassis(input) {
        unmet.push(format!(
            "missing supported {FIGHTER_CLASS_ID} levels 1-{MAX_SUPPORTED_FIGHTER_LEVEL} or \
             {WIZARD_CLASS_ID} levels 1-{MAX_SUPPORTED_WIZARD_LEVEL} chassis"
        ));
    }

    let expected = [CLIMB_SKILL_ID, INTIMIDATE_SKILL_ID, SWIM_SKILL_ID];
    for skill_id in expected {
        require_selected_skill_rank(allocations, skill_id, &mut unmet);
    }
    for allocation in allocations {
        if !expected.contains(&allocation.skill_id.as_str()) {
            unmet.push(format!(
                "skill allocation {} is outside the selected Climb/Intimidate/Swim slice",
                allocation.skill_id
            ));
        }
    }

    // v0.6 alpha swarm sub-task 4: deliberately does not require every
    // EquippedActive selection to resolve against corpus -- see
    // `compute_combat_baseline_from_corpus`'s own doc comment for the real
    // regression this caused and why `compute_equipment_effects`'s existing
    // graceful-skip tolerance is trusted instead of re-blocking on top of it.
    if !unmet.is_empty() {
        return Err(unmet);
    }

    let equipped: Vec<EquipmentSelection> = input
        .chosen
        .equipment_selections
        .iter()
        .filter(|selection| selection.active_state == ActiveState::EquippedActive)
        .cloned()
        .collect();
    let effects = compute_equipment_effects(&equipped, corpus);

    let level = supported_fighter_level(input).unwrap_or(1);
    let armor_check_penalty =
        (effects.armor_check_penalty_total + fighter_armor_training(level).armor_check_reduction)
            .min(0);

    let rank = i16::from(SELECTED_SKILL_RANK);
    // v0.6 alpha swarm, Investigator full-build closure: three independent
    // per-skill checks, not one shared scalar -- see
    // `selected_skill_climb_is_class_skill`'s own doc comment in
    // `pilot_compute.rs` for why (Investigator's own real class-skill list
    // is a genuine partial match, Climb/Intimidate yes, Swim no).
    let climb_class_skill_bonus =
        if selected_skill_climb_is_class_skill(input) { CLASS_SKILL_BONUS } else { 0 };
    let intimidate_class_skill_bonus =
        if selected_skill_intimidate_is_class_skill(input) { CLASS_SKILL_BONUS } else { 0 };
    let swim_class_skill_bonus =
        if selected_skill_swim_is_class_skill(input) { CLASS_SKILL_BONUS } else { 0 };

    let climb =
        rank + base.ability_modifiers.strength + climb_class_skill_bonus + armor_check_penalty;
    let intimidate = rank + base.ability_modifiers.charisma + intimidate_class_skill_bonus;
    let swim =
        rank + base.ability_modifiers.strength + swim_class_skill_bonus + armor_check_penalty;

    Ok(CorpusAwareSelectedSkillModifiers { climb, intimidate, swim })
}

#[cfg(test)]
mod corpus_aware_posture_widening_tests {
    use super::*;
    use crate::pcgen_import::ir_converter::convert_equipment_record;
    use crate::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
    use crate::rules_core::character_input::load_character_input_fixture;
    use crate::rules_core::pilot_compute::{build_pilot_headless_receipt, HeadlessReceiptStatus};
    use crate::rules_core::source_content::SourceRef;

    // Real verbatim corpus tokens for the fixed posture's own two items,
    // matching `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`'s
    // real loadout exactly, plus a Buckler (a real, different arms_armor
    // shield record) and a Breastplate (a real, different arms_armor armor
    // record) to prove the widening actually accepts loadouts beyond the
    // fixed two, and the same "+1 Enhancement to Weapon" equipmods record
    // `equipment_effects.rs`'s own tests use, to prove a real attached
    // enhancement now folds into the melee attack bonus.
    const FIXTURE_TEXT: &str = "\
Chain Shirt\tKEY:Chain Shirt (Base)\tTYPE:Armor.Light\tCOST:100\tWT:25\tACCHECK:-2\tMAXDEX:4\tSPELLFAILURE:20\tBONUS:COMBAT|AC|4|TYPE=Armor|PREVAREQ:DisableArmorBonus,0
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8
Dagger\tKEY:Dagger (Base)\tTYPE:Weapon.Melee.Simple\tCOST:2\tWT:1\tCRITMULT:x2\tDAMAGE:1d4
Buckler\tKEY:Buckler (Base)\tTYPE:Shield.Light\tCOST:5\tWT:5\tBONUS:COMBAT|AC|1|TYPE=Shield|PREVAREQ:DisableShieldBonus,0
Breastplate\tKEY:Breastplate (Base)\tTYPE:Armor.Medium\tCOST:200\tWT:30\tACCHECK:-4\tMAXDEX:3\tSPELLFAILURE:25\tBONUS:COMBAT|AC|6|TYPE=Armor|PREVAREQ:DisableArmorBonus,0
+1 (Enhancement to Weapon)\tKEY:Special Ability ~ +1 ~ Weapon\tTYPE:Weapon\tPLUS:1\tCOST:0\tBONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement
";

    fn corpus_with_fixture() -> SourcePackageContent<'static> {
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", FIXTURE_TEXT);
        assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
        let source_ref = SourceRef { lst_file: "cr_equip_arms_armor.lst".to_string(), line: 1 };
        let mut corpus = SourcePackageContent::empty("core_rulebook", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }

    /// Same fixed-posture fixture-text shape as
    /// `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
    /// (and its `sd18_..._widening` siblings for higher levels), built
    /// inline so each level/class variant is easy to eyeball. `class_id`
    /// lets this build the Wizard/Rogue variants too (same posture, per
    /// `compose_character_input`'s own identical fixed loadout across all
    /// three classes).
    fn fixed_posture_fixture(class_id: &str, level: u8) -> String {
        let weapon_training_choice = if class_id == "class:fighter" && level >= 5 {
            "choice=choice:fighter_weapon_training_group:group:heavy_blades\n"
        } else {
            ""
        };
        // Wizard's own, unrelated spellbook posture (canonical Evocation
        // specialization + starter spell) must also be satisfied for
        // `build_pilot_headless_receipt`'s overall status to reach
        // `Computed` -- irrelevant to the combat-baseline pillar this test
        // module exercises, but required for the byte-identical
        // re-verification to even have a headless value to compare against.
        let wizard_spellbook = if class_id == "class:wizard" {
            "choice=choice:wizard_school_specialization:school:evocation\n\
             choice=choice:wizard_opposed_schools:school:necromancy\n\
             choice=choice:wizard_opposed_schools:school:transmutation\n\
             spell=Light:class:wizard:known\n\
             spell=Light:class:wizard:prepared\n"
        } else {
            ""
        };
        format!(
            "case_id=case:corpus-aware-posture-test\n\
             source_package_id=core_rulebook\n\
             race_id=race:human\n\
             class_level={class_id}:{level}\n\
             ability=strength:16\n\
             ability=dexterity:14\n\
             ability=constitution:14\n\
             ability=intelligence:10\n\
             ability=wisdom:12\n\
             ability=charisma:8\n\
             feat=feat:power_attack\n\
             feat=feat:dodge\n\
             feat=feat:weapon_focus\n\
             skill=skill:climb:1\n\
             skill=skill:intimidate:1\n\
             skill=skill:swim:1\n\
             equipment=item:chain_shirt:equipped_worn_active\n\
             equipment=item:longsword:equipped_primary_active\n\
             equipment=item:shield:absent\n\
             equipment=power_attack:selected_inactive\n\
             choice=choice:fighter_bonus_feat:feat:weapon_focus:weapon:longsword\n\
             {weapon_training_choice}{wizard_spellbook}"
        )
    }

    fn load(fixture: &str) -> CharacterInput {
        let result = load_character_input_fixture(fixture);
        assert!(result.diagnostics.is_empty(), "fixture must parse cleanly: {:?}", result.diagnostics);
        result.character_input.expect("fixture must produce a CharacterInput")
    }

    /// The correctness proof: for the exact existing fixed posture, the new
    /// corpus-aware combat baseline must produce byte-identical values to
    /// today's hardcoded `compute_combat_baseline`, at every Fighter
    /// armor-training/weapon-training breakpoint level plus Wizard/Rogue
    /// level 1. A mismatch here is a stop-and-flag discrepancy, not
    /// something to silently reconcile.
    #[test]
    fn matches_the_hardcoded_baseline_exactly_for_every_currently_computed_build() {
        let corpus = corpus_with_fixture();
        for (class_id, level) in [
            ("class:fighter", 1),
            ("class:fighter", 3),
            ("class:fighter", 5),
            ("class:fighter", 7),
            ("class:fighter", 9),
            ("class:fighter", 11),
            ("class:fighter", 15),
            ("class:fighter", 20),
            ("class:wizard", 1),
            ("class:rogue", 1),
        ] {
            let input = load(&fixed_posture_fixture(class_id, level));

            let headless = build_pilot_headless_receipt(&input);
            assert_eq!(
                headless.status,
                HeadlessReceiptStatus::Computed,
                "{class_id} level {level} must reach Computed today: {:?}",
                headless.computation.diagnostics
            );

            let corpus_aware_combat = compute_combat_baseline_from_corpus(&headless.computation, &input, &corpus)
                .unwrap_or_else(|unmet| {
                    panic!("{class_id} level {level}: corpus-aware combat baseline must also reach Computed, unmet: {unmet:?}")
                });
            assert_eq!(
                corpus_aware_combat.melee_attack_bonus, headless.computation.baseline_melee_attack_bonus,
                "{class_id} level {level}: melee attack bonus must match the hardcoded value exactly"
            );
            assert_eq!(
                corpus_aware_combat.armor_class, headless.computation.baseline_armor_class,
                "{class_id} level {level}: armor class must match the hardcoded value exactly"
            );

            let corpus_aware_skills =
                compute_selected_skill_modifiers_from_corpus(&headless.computation, &input, &corpus)
                    .unwrap_or_else(|unmet| {
                        panic!("{class_id} level {level}: corpus-aware selected-skill modifiers must also reach Computed, unmet: {unmet:?}")
                    });
            assert_eq!(
                corpus_aware_skills.climb, headless.computation.selected_skill_modifiers.climb,
                "{class_id} level {level}: climb must match the hardcoded value exactly"
            );
            assert_eq!(
                corpus_aware_skills.intimidate, headless.computation.selected_skill_modifiers.intimidate,
                "{class_id} level {level}: intimidate must match the hardcoded value exactly"
            );
            assert_eq!(
                corpus_aware_skills.swim, headless.computation.selected_skill_modifiers.swim,
                "{class_id} level {level}: swim must match the hardcoded value exactly"
            );
        }
    }

    /// The real widening: a Fighter wearing a different real armor record
    /// (Breastplate instead of Chain Shirt) now reaches a real, correctly
    /// computed, DIFFERENT armor class -- not blocked, and not the old
    /// Chain-Shirt-specific number.
    #[test]
    fn a_different_real_armor_record_computes_its_own_real_armor_class() {
        let corpus = corpus_with_fixture();
        let fixture = fixed_posture_fixture("class:fighter", 1).replace(
            "equipment=item:chain_shirt:equipped_worn_active",
            "equipment=Breastplate (Base):equipped_worn_active",
        );
        let input = load(&fixture);
        let base = build_pilot_headless_receipt(&input).computation;

        let result = compute_combat_baseline_from_corpus(&base, &input, &corpus)
            .expect("a different real, resolvable armor record must reach Computed");

        // 10 base + 6 Breastplate AC + min(2 DEX mod, MAXDEX:3) + 1 Dodge = 19
        assert_eq!(result.armor_class, 19);
    }

    /// A shield genuinely EQUIPPED (not required-absent any more) now adds
    /// its real AC bonus -- the old posture required shield absence
    /// unconditionally; this is a real, new capability.
    #[test]
    fn an_equipped_shield_now_adds_its_real_ac_bonus_instead_of_being_required_absent() {
        let corpus = corpus_with_fixture();
        let fixture = fixed_posture_fixture("class:fighter", 1)
            .replace("equipment=item:shield:absent", "equipment=Buckler (Base):equipped_worn_active");
        let input = load(&fixture);
        let base = build_pilot_headless_receipt(&input).computation;

        let result = compute_combat_baseline_from_corpus(&base, &input, &corpus)
            .expect("an equipped, resolvable shield must reach Computed");

        // Today's fixed-posture AC (17) + Buckler's real +1 shield bonus = 18.
        assert_eq!(result.armor_class, 18);
    }

    /// No armor at all is also a real, correctly-computed loadout now --
    /// honest unarmored AC, not a blocked build.
    #[test]
    fn no_armor_at_all_computes_a_real_unarmored_armor_class() {
        let corpus = corpus_with_fixture();
        let fixture = fixed_posture_fixture("class:fighter", 1)
            .lines()
            .filter(|line| !line.starts_with("equipment=item:chain_shirt"))
            .collect::<Vec<_>>()
            .join("\n");
        let input = load(&fixture);
        let base = build_pilot_headless_receipt(&input).computation;

        let result = compute_combat_baseline_from_corpus(&base, &input, &corpus)
            .expect("no armor equipped at all must still reach Computed");

        // 10 base + 0 armor + full uncapped 2 DEX mod (no MAXDEX cap) + 1 Dodge = 13.
        assert_eq!(result.armor_class, 13);
    }

    /// Connects sub-tasks 1/2/6: a real magical enhancement attached to
    /// the required Longsword now folds into the melee attack bonus --
    /// today's hardcoded formula has no awareness of equipment modifiers
    /// at all.
    #[test]
    fn a_real_enhancement_attached_to_the_required_longsword_raises_the_melee_attack_bonus() {
        let corpus = corpus_with_fixture();
        let mut input = load(&fixed_posture_fixture("class:fighter", 1));
        let longsword = input
            .chosen
            .equipment_selections
            .iter_mut()
            .find(|selection| selection.item_id == "item:longsword")
            .expect("Longsword must be present");
        longsword.applied_modifiers.push("Special Ability ~ +1 ~ Weapon".to_string());
        let base = build_pilot_headless_receipt(&input).computation;

        let result = compute_combat_baseline_from_corpus(&base, &input, &corpus)
            .expect("an attached real enhancement must still reach Computed");

        assert_eq!(
            result.melee_attack_bonus,
            base.baseline_melee_attack_bonus + 1,
            "the attached +1 enhancement must raise the melee attack bonus by exactly 1 over \
             today's hardcoded (enhancement-blind) value"
        );
    }

    /// A second real weapon equipped alongside the required Longsword is
    /// honestly ambiguous (same reasoning `attack_bonus_delta` itself
    /// already uses) -- must Block, not guess or silently drop the second
    /// weapon.
    #[test]
    fn a_second_real_weapon_equipped_is_honestly_blocked_not_guessed() {
        let corpus = corpus_with_fixture();
        let fixture =
            format!("{}equipment=Dagger (Base):equipped_worn_active\n", fixed_posture_fixture("class:fighter", 1));
        let input = load(&fixture);
        let base = build_pilot_headless_receipt(&input).computation;

        let unmet = compute_combat_baseline_from_corpus(&base, &input, &corpus)
            .expect_err("two real weapons equipped must be honestly blocked");

        assert!(
            unmet.iter().any(|message| message.contains("ambiguous")),
            "expected the real ambiguous-attack-bonus diagnostic: {unmet:?}"
        );
    }

    /// An equipped item that does not resolve against the corpus at all
    /// must NOT block the build -- it silently contributes nothing, same as
    /// `compute_equipment_effects`'s own existing graceful-skip tolerance
    /// (mirrors the OLD exact-posture gate's own behavior, which never
    /// noticed or cared about any equipped item beyond its 3 hardcoded
    /// ones). Real regression found during sub-task 5's re-verification: an
    /// earlier version of this function DID hard-block here, which broke
    /// every existing test that equips a second, non-fixed item against
    /// the desktop app's real, tiny (2-record) bundled corpus -- see this
    /// function's own doc comment for the full account.
    #[test]
    fn an_unresolvable_equipped_item_does_not_block_and_contributes_nothing() {
        let corpus = corpus_with_fixture();
        let fixture = format!(
            "{}equipment=Wand of Cure Light Wounds:equipped_worn_active\n",
            fixed_posture_fixture("class:fighter", 1)
        );
        let input = load(&fixture);
        let base = build_pilot_headless_receipt(&input).computation;

        let result = compute_combat_baseline_from_corpus(&base, &input, &corpus)
            .expect("an unresolvable equipped item must not block the build");

        assert_eq!(
            result.armor_class, base.baseline_armor_class,
            "an unresolvable item must contribute nothing, matching today's hardcoded value"
        );
    }

    /// The selected-skill pillar widens the same way: a different real
    /// armor record still produces a real, correctly-computed
    /// Climb/Swim armor-check penalty, not the old Chain-Shirt-specific
    /// value.
    #[test]
    fn selected_skill_modifiers_use_the_real_armor_check_penalty_of_whatever_is_equipped() {
        let corpus = corpus_with_fixture();
        let fixture = fixed_posture_fixture("class:fighter", 1).replace(
            "equipment=item:chain_shirt:equipped_worn_active",
            "equipment=Breastplate (Base):equipped_worn_active",
        );
        let input = load(&fixture);
        let base = build_pilot_headless_receipt(&input).computation;

        let result = compute_selected_skill_modifiers_from_corpus(&base, &input, &corpus)
            .expect("a different real, resolvable armor record must reach Computed");

        // rank 1 + STR mod 3 + class-skill 3 + Breastplate's real -4 ACP = 3.
        assert_eq!(result.climb, 3);
        assert_eq!(result.swim, 3);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcgen_import::ir_converter::convert_equipment_record;
    use crate::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
    use crate::rules_core::character_input::{
        AbilityScores, CharacterClassLevel, ChosenCharacterState,
    };
    use crate::rules_core::source_content::SourceRef;

    /// Real verbatim tokens for a Chain Shirt, matching
    /// `tests/sd20_contract_equipment_wiring.rs`'s own fixture exactly
    /// (`ACCHECK:-2`) -- same real corpus record, reused rather than
    /// re-derived.
    const CHAIN_SHIRT_FIXTURE_TEXT: &str = "Chain Shirt\tKEY:Chain Shirt (Base)\tTYPE:Armor.Light\tCOST:100\tWT:25\tACCHECK:-2\tMAXDEX:4\tSPELLFAILURE:20\tBONUS:COMBAT|AC|4|TYPE=Armor|PREVAREQ:DisableArmorBonus,0\n";

    fn corpus_with_chain_shirt() -> SourcePackageContent<'static> {
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", CHAIN_SHIRT_FIXTURE_TEXT);
        assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
        let source_ref = SourceRef { lst_file: "cr_equip_arms_armor.lst".to_string(), line: 1 };
        let mut corpus = SourcePackageContent::empty("core_rulebook", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }

    fn fighter_input_with(equipment_selections: Vec<EquipmentSelection>) -> CharacterInput {
        CharacterInput {
            case_id: Some("pilot-compute-corpus-equipment-effects-test".to_string()),
            source_package_id: "test".to_string(),
            chosen: ChosenCharacterState {
                race_id: "race:human".to_string(),
                class_levels: vec![CharacterClassLevel { class_id: "class:fighter".to_string(), level: 1 }],
                ability_scores: AbilityScores {
                    strength: 16,
                    dexterity: 14,
                    constitution: 14,
                    intelligence: 10,
                    wisdom: 12,
                    charisma: 8,
                },
                selected_feats: Vec::new(),
                skill_allocations: Vec::new(),
                equipment_selections,
                selected_choices: Vec::new(),
                spells_selected: Vec::new(),
                class_ability_activations: Vec::new(),
            },
            selection_provenance: Vec::new(),
        }
    }

    /// v0.6 alpha swarm item 1, shape (c): the new
    /// `CorpusDerivedSection.equipment_effects` field surfaces a real,
    /// corpus-resolved armor-check penalty for an equipped item -- the gap
    /// the item-1 design pass identified (the token was already present on
    /// the resolved record, just never read into a struct field).
    #[test]
    fn corpus_derived_section_carries_the_real_armor_check_penalty_for_equipped_active_armor() {
        let corpus = corpus_with_chain_shirt();
        let input = fighter_input_with(vec![EquipmentSelection {
            item_id: "Chain Shirt (Base)".to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        }]);

        let receipt = compute_pilot_with_corpus(&input, &corpus);

        assert_eq!(
            receipt.corpus_derived.equipment_effects.armor_check_penalty_total, -2,
            "Chain Shirt's real ACCHECK is -2"
        );
        assert_eq!(receipt.corpus_derived.equipment_effects.armor_class_delta, 4);
    }

    /// A resolvable item that is merely `SelectedInactive` (owned, not
    /// worn) must contribute no armor-check penalty -- proves the
    /// `EquippedActive` filter is real, not vacuous.
    #[test]
    fn corpus_derived_section_excludes_a_selected_inactive_items_armor_check_penalty() {
        let corpus = corpus_with_chain_shirt();
        let input = fighter_input_with(vec![EquipmentSelection {
            item_id: "Chain Shirt (Base)".to_string(),
            equipped_or_active: false,
            active_state: ActiveState::SelectedInactive,
            applied_modifiers: Vec::new(),
        }]);

        let receipt = compute_pilot_with_corpus(&input, &corpus);

        assert_eq!(receipt.corpus_derived.equipment_effects.armor_check_penalty_total, 0);
        assert_eq!(receipt.corpus_derived.equipment_effects.armor_class_delta, 0);
        assert!(
            receipt.corpus_derived.equipment_effects.per_item.is_empty(),
            "an inactive selection must produce no per-item equipment-effect entry"
        );
    }

    /// A build with no equipment at all must show a real, honest zero, not
    /// an error or a fabricated value.
    #[test]
    fn corpus_derived_section_defaults_to_zero_armor_check_penalty_with_no_equipment() {
        let corpus = corpus_with_chain_shirt();
        let input = fighter_input_with(Vec::new());

        let receipt = compute_pilot_with_corpus(&input, &corpus);

        assert_eq!(receipt.corpus_derived.equipment_effects.armor_check_penalty_total, 0);
    }

    /// v0.6 alpha swarm (QA finding, 2026-07-24): a real, disk-persisted
    /// equipment selection that does not resolve against `corpus` (e.g.
    /// the desktop app's tiny bundled demo corpus) must be traceable, not
    /// silently vanish from every corpus-derived output with no signal at
    /// all. A resolvable item and an unresolvable one are both present in
    /// the same input, proving the unresolved list doesn't just echo
    /// everything back.
    #[test]
    fn corpus_derived_section_tracks_an_equipment_selection_that_does_not_resolve() {
        let corpus = corpus_with_chain_shirt();
        let input = fighter_input_with(vec![
            EquipmentSelection {
                item_id: "Chain Shirt (Base)".to_string(),
                equipped_or_active: true,
                active_state: ActiveState::EquippedActive,
                applied_modifiers: Vec::new(),
            },
            EquipmentSelection {
                item_id: "Wand of Cure Light Wounds".to_string(),
                equipped_or_active: true,
                active_state: ActiveState::EquippedActive,
                applied_modifiers: Vec::new(),
            },
        ]);

        let receipt = compute_pilot_with_corpus(&input, &corpus);

        assert_eq!(
            receipt.corpus_derived.unresolved_equipment_item_ids,
            vec!["Wand of Cure Light Wounds".to_string()],
            "the unresolvable selection must be traceable, not silently dropped"
        );
        assert_eq!(
            receipt.corpus_derived.equipped_items.len(),
            1,
            "the resolvable Chain Shirt must still resolve normally"
        );
    }

    /// Mirrors the equipment case exactly, for `spells_selected`.
    #[test]
    fn corpus_derived_section_tracks_a_spell_selection_that_does_not_resolve() {
        let corpus = corpus_with_chain_shirt();
        let mut input = fighter_input_with(Vec::new());
        input.chosen.spells_selected.push(crate::rules_core::character_input::SpellSelection {
            spell_id: "Magic Missile".to_string(),
            source_class_id: "class:wizard".to_string(),
            acquisition_mode: crate::rules_core::character_input::AcquisitionMode::Known,
        });

        let receipt = compute_pilot_with_corpus(&input, &corpus);

        assert_eq!(
            receipt.corpus_derived.unresolved_spell_ids,
            vec!["Magic Missile".to_string()],
            "a real spell selection absent from this corpus must be traceable, not silently dropped"
        );
        assert!(receipt.corpus_derived.school_coverage.is_empty());
    }

    /// Every selection resolving cleanly must leave both unresolved lists
    /// genuinely empty, not just unpopulated by omission.
    #[test]
    fn corpus_derived_section_leaves_unresolved_lists_empty_when_everything_resolves() {
        let corpus = corpus_with_chain_shirt();
        let input = fighter_input_with(vec![EquipmentSelection {
            item_id: "Chain Shirt (Base)".to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        }]);

        let receipt = compute_pilot_with_corpus(&input, &corpus);

        assert!(receipt.corpus_derived.unresolved_equipment_item_ids.is_empty());
        assert!(receipt.corpus_derived.unresolved_spell_ids.is_empty());
    }

    // v0.6 alpha swarm items 1+27 sub-task 6: a Longsword plus the same
    // real "+1 Enhancement to Weapon" equipmods record `equipment_effects.rs`'s
    // own attack_bonus_delta_tests fixture uses, so `applied_modifiers`
    // resolution can be exercised against a genuine weapon+modifier pair.
    const LONGSWORD_AND_ENHANCEMENT_FIXTURE_TEXT: &str = "\
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8
+1 (Enhancement to Weapon)\tKEY:Special Ability ~ +1 ~ Weapon\tTYPE:Weapon\tPLUS:1\tCOST:0\tBONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement
";

    fn corpus_with_longsword_and_enhancement() -> SourcePackageContent<'static> {
        let result =
            parse_equipment_entries("cr_equip_arms_armor.lst", LONGSWORD_AND_ENHANCEMENT_FIXTURE_TEXT);
        assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
        let source_ref = SourceRef { lst_file: "cr_equip_arms_armor.lst".to_string(), line: 1 };
        let mut corpus = SourcePackageContent::empty("core_rulebook", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }

    /// The core case: a weapon selection's own `applied_modifiers` resolve
    /// into nested `ResolvedEquipment` entries on that selection's own
    /// `equipped_items` entry, not a separate flat top-level record.
    #[test]
    fn a_resolvable_applied_modifier_resolves_nested_under_its_weapon() {
        let corpus = corpus_with_longsword_and_enhancement();
        let input = fighter_input_with(vec![EquipmentSelection {
            item_id: "Longsword (Base)".to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: vec!["Special Ability ~ +1 ~ Weapon".to_string()],
        }]);

        let receipt = compute_pilot_with_corpus(&input, &corpus);

        assert_eq!(receipt.corpus_derived.equipped_items.len(), 1, "no separate top-level entry for the modifier");
        let longsword = &receipt.corpus_derived.equipped_items[0];
        assert_eq!(longsword.applied_modifiers.len(), 1);
        assert_eq!(longsword.applied_modifiers[0].item_id, "Special Ability ~ +1 ~ Weapon");
        assert_eq!(longsword.applied_modifiers[0].equipment_record_name, "+1 (Enhancement to Weapon)");
        assert!(
            receipt.corpus_derived.unresolved_equipment_item_ids.is_empty(),
            "a resolvable modifier must not appear in the unresolved list"
        );
    }

    /// v0.6 alpha swarm (frontend coordination, sub-task 6): an
    /// `applied_modifiers` item_id that does not resolve against `corpus`
    /// (e.g. attached from the full catalog picker but outside the
    /// desktop app's tiny bundled demo corpus) must be traceable through
    /// the same flat `unresolved_equipment_item_ids` list a top-level
    /// unresolvable selection already uses -- not a silent no-op, and not
    /// a second, new list frontend's existing `UnresolvedNotice` doesn't
    /// already render.
    #[test]
    fn an_unresolvable_applied_modifier_surfaces_in_the_shared_unresolved_list() {
        let corpus = corpus_with_longsword_and_enhancement();
        let input = fighter_input_with(vec![EquipmentSelection {
            item_id: "Longsword (Base)".to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: vec!["Special Ability ~ Flaming ~ Weapon".to_string()],
        }]);

        let receipt = compute_pilot_with_corpus(&input, &corpus);

        let longsword = &receipt.corpus_derived.equipped_items[0];
        assert!(longsword.applied_modifiers.is_empty(), "an unresolvable modifier contributes no nested entry");
        assert_eq!(
            receipt.corpus_derived.unresolved_equipment_item_ids,
            vec!["Special Ability ~ Flaming ~ Weapon".to_string()]
        );
    }
}
