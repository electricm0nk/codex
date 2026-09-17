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
use crate::rules_core::encumbrance::{compute_encumbrance, EncumbranceComputation};
use crate::rules_core::contract::encumbrance_size_for_race;
use crate::rules_core::equipment_effects::{compute_equipment_effects, EquipmentEffects};
use crate::rules_core::equipment_resolver::equipment_id_resolve;
use crate::rules_core::feat_identity;
use crate::rules_core::race_resolver::race_size_for_race_token;
use crate::rules_core::pilot_compute::{
    apply_human_ability_bonus, character_is_proficient_with, choice_selection,
    combat_maneuver_bonus, combat_maneuver_defense, compute_pilot_base_chassis,
    equipped_weapon_stat_block, feat_derived_pillar_contributions, fighter_armor_training,
    fighter_level_in_mix, flat_footed_armor_class, touch_armor_class,
    fighter_weapon_training_attack_bonus, has_supported_class_chassis, require_active_state,
    require_selected_skill_rank, selected_skill_climb_is_class_skill,
    selected_skill_intimidate_is_class_skill, selected_skill_swim_is_class_skill,
    supported_fighter_level, PilotBaseChassisComputation, ARMOR_CLASS_BASE, CLASS_SKILL_BONUS,
    CLIMB_SKILL_ID, FIGHTER_BONUS_FEAT_CHOICE_ID, FIGHTER_CLASS_ID,
    INTIMIDATE_SKILL_ID, LONGSWORD_ITEM_ID, MAX_SUPPORTED_FIGHTER_LEVEL, MAX_SUPPORTED_WIZARD_LEVEL,
    POWER_ATTACK_ITEM_ID, SELECTED_SKILL_RANK, SWIM_SKILL_ID, WEAPON_FOCUS_FEAT_ID,
    WEAPON_FOCUS_LONGSWORD_SELECTION, WEAPON_FOCUS_TO_HIT_BONUS,
    WEAPON_NONPROFICIENCY_ATTACK_PENALTY, WIZARD_CLASS_ID,
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
///
/// Deliberately does NOT derive `Default`. It did until `encumbrance` was
/// added, but the derive had no call site anywhere in the workspace, and a
/// defaulted `EncumbranceComputation` would have to invent a carrying
/// capacity for a Strength score nobody supplied -- a fabricated rules
/// value of exactly the kind this crate refuses to produce. Every
/// construction of this type goes through `compute_pilot_with_corpus`,
/// which has a real `CharacterInput` to derive all of it from.
#[derive(Debug, Clone, PartialEq)]
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
    /// Real carried weight, PF1 carrying-capacity thresholds, load tier,
    /// and that tier's own max-Dex/armor-check penalties
    /// (`encumbrance::compute_encumbrance`).
    ///
    /// `contract::to_pilot_receipt` already computed this for
    /// `PilotReceipt`, but the desktop app reaches the engine through
    /// *this* function, not `contract.rs` -- so before this field, every
    /// weight and encumbrance number the engine computes was unreachable
    /// from the UI. Same additive, explicitly-not-claim-gated posture as
    /// `equipment_effects` above.
    ///
    /// Unlike `equipment_effects`/`equipped_items`, this is NOT scoped to
    /// `EquippedActive` -- a `SelectedInactive` item (owned, in the pack,
    /// simply not worn) still weighs something, matching
    /// `contract::to_pilot_receipt`'s own reasoning.
    pub encumbrance: EncumbranceComputation,
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
    let mut base = compute_pilot_base_chassis(input);

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
        // SD-35 `AT-35-E6-003-RULED` cycle 13: `record` is the settled
        // `CorpusEquipmentRecord` now, and `identity` IS the KEY-or-name rule
        // `equipment_key_token` applied to the parser row before the move.
        let key = record.identity.clone();

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
            let modifier_key = modifier_record.identity.clone();
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

    // Carrying capacity needs the effective Strength *score* (not the
    // modifier `base` carries), and reads the unfiltered selection list --
    // both matching `contract::to_pilot_receipt`'s own encumbrance wiring
    // exactly. The explanation `apply_human_ability_bonus` would push is
    // discarded: `compute_pilot_base_chassis` above already pushed it once,
    // for real, into `base`; pushing it again would duplicate an id.
    let mut discarded_explanations = Vec::new();
    let effective_ability_scores = apply_human_ability_bonus(input, &mut discarded_explanations);
    // Creature size scales carrying capacity, matching
    // `contract::to_pilot_receipt`'s own encumbrance wiring exactly -- the
    // same shared `encumbrance_size_for_race` seam, so the two receipts
    // cannot drift apart on size again. An unresolvable race pushes that
    // seam's claim-blocking diagnostic onto `base` rather than silently
    // computing at Medium.
    let (size, size_diagnostic) = encumbrance_size_for_race(&input.chosen.race_id);
    base.diagnostics.extend(size_diagnostic);
    let encumbrance = compute_encumbrance(
        &input.chosen.equipment_selections,
        corpus,
        effective_ability_scores.strength,
        size,
    );

    CorpusPilotReceipt {
        base,
        corpus_derived: CorpusDerivedSection {
            school_coverage,
            equipped_items,
            equipment_effects,
            unresolved_spell_ids,
            unresolved_equipment_item_ids,
            encumbrance,
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
/// exactly the Longsword) and the `Weapon Focus` feat requirement unchanged
/// (contributing its existing fixed bonus) -- weapon-
/// loadout widening is a distinct, later increment: Fighter Weapon
/// Training's attack bonus is itself hardcoded to the Longsword's "Heavy
/// Blades" weapon-training group, and widening the weapon would need a
/// real weapon-to-training-group mapping this crate does not have yet.
/// Whether the remaining feat *requirement* (as opposed to feat *effects*,
/// already covered by the "never block on an unrecognized feat effect"
/// policy) gets widened is a separate, undecided question, not part of this
/// slice. `Dodge`'s requirement HAS since been dropped -- it is now a
/// conditional contribution below rather than a precondition, because
/// requiring it forced character creation to claim the feat for characters
/// no slot had granted it to.
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
    /// SD-27, `decisions.md §28` defect 1: PF1 touch Armor Class -- `armor_class`
    /// above with the contributors a touch attack ignores removed. On this path
    /// that is the whole resolved armor+shield delta, since the corpus-aware
    /// formula grounds no natural-armor term.
    pub touch_armor_class: i16,
    /// SD-27, `decisions.md §28` flat-footed defect: PF1 flat-footed Armor
    /// Class -- `armor_class` above with the Dexterity bonus and every
    /// dodge-typed bonus removed. On this path the denied set is the
    /// (MAXDEX-capped) Dexterity contribution plus the shared feat seam's own
    /// dodge total; this formula grounds no class-derived dodge bonus and no
    /// Dexterity-substituting ability, so there is nothing else to deny.
    pub flat_footed_armor_class: i16,
    /// PF1 Combat Maneuver Bonus. See `pilot_compute::combat_maneuver_bonus`,
    /// which is the single owner of the formula both paths call.
    pub combat_maneuver_bonus: i16,
    /// PF1 Combat Maneuver Defense. See
    /// `pilot_compute::combat_maneuver_defense`, likewise shared.
    pub combat_maneuver_defense: i16,
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

    // v0.6 alpha swarm (creation-seed honesty fix): Dodge is deliberately NOT
    // required, mirroring `pilot_compute::unmet_combat_posture_conditions`'s
    // own change exactly -- see that function for the full reasoning. This
    // path is the one `create_character` actually gates on
    // (`resolve_unified_pilot_snapshot`), so leaving the requirement here
    // would keep every non-Human, non-Monk character uncreatable.
    //
    // Folded, not compared verbatim -- the same seam, and the same reason, as
    // `pilot_compute::unmet_combat_posture_conditions`'s own gate.
    if !feat_identity::holds(&chosen.selected_feats, WEAPON_FOCUS_FEAT_ID) {
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

    // v0.6 alpha swarm, risks item #89 / tasks #80+#86 (2026-07-29): the
    // nonproficiency penalty must be applied HERE TOO, not only in
    // `pilot_compute::compute_combat_baseline`.
    //
    // This is the exact divergence
    // `matches_the_hardcoded_baseline_exactly_for_every_currently_computed_build`
    // exists to catch, and it caught it: applying the penalty on only one
    // of the two paths made Wizard level 1 read 4 here and 0 there. Both
    // paths swing the same required Longsword against the same character,
    // so any proficiency rule that applies to one applies identically to
    // the other -- and this corpus-aware path is the intended eventual
    // replacement for the hardcoded one, so shipping it without the
    // penalty would just re-introduce the same silent overstatement later.
    let nonproficiency_penalty = match equipped_weapon_stat_block(LONGSWORD_ITEM_ID)
        .and_then(|weapon| character_is_proficient_with(input, weapon))
    {
        Some(false) => WEAPON_NONPROFICIENCY_ATTACK_PENALTY,
        Some(true) => 0,
        None => {
            // Unknown, never assumed. Same contract as the hardcoded path:
            // refuse to produce a number rather than guess a penalty.
            unmet.push(format!(
                "this character's proficiency with {LONGSWORD_ITEM_ID} is unknown (a class in \
                 the mix has no ingested weapon-proficiency record), so the nonproficiency \
                 penalty cannot be resolved"
            ));
            0
        }
    };

    if !unmet.is_empty() {
        return Err(unmet);
    }

    // SD-27, decisions.md §28 defect 1 (2026-07-31): resolved once, ahead of
    // both the attack bonus and the Armor Class, because PF1 gives them the
    // SAME size modifier (Table 8-1) and CMB/CMD the opposite-signed special
    // one. The unresolvable-race case refuses rather than assuming Medium --
    // this function's existing contract for an unknown fact, matching the
    // unknown-proficiency case above.
    let Some(size) = race_size_for_race_token(&chosen.race_id) else {
        return Err(vec![format!(
            "race {:?} resolves to no ingested race, so its creature size -- and its size \
             modifiers to Armor Class, touch AC, attack rolls, CMB and CMD -- are unknown",
            chosen.race_id
        )]);
    };
    let size_armor_class_modifier = size.armor_class_size_modifier();

    let level = supported_fighter_level(input).unwrap_or(1);
    let strength_modifier = base.ability_modifiers.strength;
    let weapon_training_bonus = fighter_weapon_training_attack_bonus(input, level);
    // The size modifier is applied HERE TOO, not only in
    // `pilot_compute::compute_combat_baseline`, for exactly the reason the
    // nonproficiency penalty above states: this is the path
    // `resolve_unified_pilot_snapshot` gates on, so it is the number the
    // player's sheet reads, while
    // `matches_the_hardcoded_baseline_exactly_for_every_currently_computed_build`
    // pins the two paths equal. One-sided application would both leave the
    // shipped sheet wrong and break that parity test.
    let melee_attack_bonus = base.base_attack_bonus
        + strength_modifier
        + WEAPON_FOCUS_TO_HIT_BONUS
        + size_armor_class_modifier
        + weapon_training_bonus
        + attack_bonus_delta
        + nonproficiency_penalty;

    let armor_training = fighter_armor_training(level);
    let effective_max_dex = effects.max_dex_cap.map(|cap| cap + armor_training.max_dex_increase);
    let dexterity_modifier = base.ability_modifiers.dexterity;
    let dexterity_contribution = match effective_max_dex {
        Some(cap) => dexterity_modifier.min(cap),
        None => dexterity_modifier,
    };
    // SD-27 (`decisions.md` §28, feat-seam defect, 2026-07-31): every
    // feat-derived contribution to this path's pillars, from the ONE seam
    // `pilot_compute::compute_combat_baseline` also reads
    // (`pilot_compute::feat_derived_pillar_contributions`).
    //
    // This file used to reference ZERO `feat_effects` producers while its twin
    // referenced 33, and hand-inlined Dodge as its only feat awareness. Since
    // this is the path `resolve_unified_pilot_snapshot` gates on, that made
    // every other wired feat effect invisible on the shipped sheet while its
    // own tests passed: CRB's Athletic, Persuasive and Intimidating Prowess
    // (skills, below) and ARG's Armor of the Pit (natural armor, here) and Sure
    // and Fleet (Climb, below) all moved a number in `pilot_compute.rs` and
    // moved nothing a player could see. Reading the shared struct -- rather
    // than re-deriving a parallel list of feat calls here -- is what makes
    // wiring one path without the other impossible rather than merely
    // discouraged.
    let feat_contributions =
        feat_derived_pillar_contributions(input, strength_modifier);
    // Conditional contribution, not a precondition -- see
    // `pilot_compute::compute_combat_baseline` for the full reasoning.
    let feat_armor_class_bonus = feat_contributions.armor_class_bonus();
    // SD-27, decisions.md §28 defect 1 (2026-07-31): the creature's PF1
    // Table 8-1 size modifier to Armor Class. Applied HERE TOO, not only in
    // `pilot_compute::compute_combat_baseline` -- and for exactly the reason
    // the nonproficiency penalty above states: this is the path
    // `resolve_unified_pilot_snapshot` / `create_character` actually gate on,
    // so it is the one a player's sheet reads, while
    // `matches_the_hardcoded_baseline_exactly_for_every_currently_computed_build`
    // pins the two paths equal. Applying it on one path only would both leave
    // the shipped sheet wrong and break that parity test.
    //
    // Same stacking position and same modifier-type reasoning as the hardcoded
    // path: after the Dexterity contribution, its own PF1 modifier type, one
    // size per creature so non-stacking holds structurally. `size` and
    // `size_armor_class_modifier` are resolved once, above the attack bonus,
    // because attack rolls take the identical Table 8-1 value.
    let armor_class = ARMOR_CLASS_BASE
        + effects.armor_class_delta
        + dexterity_contribution
        + size_armor_class_modifier
        + feat_armor_class_bonus;

    // SD-27, decisions.md §28 defect 1 (2026-07-31): touch AC / CMB / CMD on
    // the path the shipped sheet reads. All three used to be computed in React
    // (`CharacterSheet.tsx`), size-blind, with touch AC able to contradict this
    // very Armor Class on the same panel.
    //
    // Touch AC is derived by SUBTRACTION from `armor_class` above -- see
    // `pilot_compute::touch_armor_class` for why that shape rather than a
    // parallel formula. The excluded set is `effects.armor_class_delta`
    // (`compute_equipment_effects`' summed armor + shield bonus) plus the
    // feat-derived NATURAL armor the shared seam resolved -- taken from that
    // seam's own accessor, not by re-listing its fields, so a future natural-armor
    // feat leaves touch AC on both twins at once. The Dexterity contribution,
    // the size modifier and Dodge's dodge bonus are all retained, which is
    // correct for a touch attack.
    let touch = touch_armor_class(
        armor_class,
        effects.armor_class_delta + feat_contributions.excluded_from_touch_armor_class(),
    );
    // SD-27, decisions.md §28 (flat-footed defect, 2026-07-31): flat-footed AC
    // on the path the shipped sheet reads. It was the LAST of the four defense
    // cells still computed in React (`CharacterSheet.tsx`'s
    // `ac - Math.max(0, dexMod)`, live since `f5117103`), and it was the one
    // that was also arithmetically wrong: it subtracted the Dexterity bonus and
    // stopped, so a character holding Dodge read a flat-footed AC one point
    // higher than PF1 allows.
    //
    // Applied HERE TOO, not only in `pilot_compute::compute_combat_baseline`,
    // for the reason every other cell on this path states: this is the path
    // `resolve_unified_pilot_snapshot` / `create_character` gate on, so it is
    // the number a player's sheet reads, while
    // `every_catalog_feat_moves_both_compute_paths_identically` pins the two
    // paths equal.
    //
    // The denied set, per term of the `armor_class` sum above:
    //   DENIED -- `dexterity_contribution` (bonus only; see
    //     `pilot_compute::flat_footed_armor_class` on why a Dexterity penalty
    //     is kept) and the shared seam's dodge-typed feat total, taken from its
    //     own accessor rather than by re-listing its fields.
    //   KEPT -- `effects.armor_class_delta` (the resolved armor + shield
    //     bonus), `size_armor_class_modifier`, and the seam's natural-armor
    //     half. None of the three is a dodge bonus or a Dexterity bonus.
    let flat_footed = flat_footed_armor_class(
        armor_class,
        dexterity_contribution,
        feat_contributions.denied_to_flat_footed_armor_class(),
    );
    // Shared formulas, deliberately not re-derived: `pilot_compute` owns both,
    // so the two engine paths cannot drift into two different CMB/CMD rules the
    // way they nearly did on the nonproficiency penalty. `dexterity_modifier`
    // is the RAW ability modifier resolved above, not the MAXDEX-capped
    // `dexterity_contribution` -- see `pilot_compute::combat_maneuver_defense`
    // for why that distinction is deliberate and stated rather than guessed.
    let cmb = combat_maneuver_bonus(
        base.base_attack_bonus,
        strength_modifier,
        dexterity_modifier,
        Some(size),
        size.special_size_modifier(),
    );
    let cmd = combat_maneuver_defense(
        base.base_attack_bonus,
        strength_modifier,
        dexterity_modifier,
        size.special_size_modifier(),
    );

    Ok(CorpusAwareCombatBaseline {
        melee_attack_bonus,
        armor_class,
        touch_armor_class: touch,
        flat_footed_armor_class: flat_footed,
        combat_maneuver_bonus: cmb,
        combat_maneuver_defense: cmd,
    })
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

    // SD-27 (`decisions.md` §28, feat-seam defect, 2026-07-31): the same shared
    // seam `pilot_compute::compute_selected_skill_modifiers` reads. Before it,
    // this path -- the one `resolve_unified_pilot_snapshot` gates on, so the one
    // the sheet renders -- consumed no feat effects at all: CRB's Athletic
    // (+2 Climb/Swim), Persuasive (+2 Intimidate) and Intimidating Prowess
    // (Strength to Intimidate), and ARG's Sure and Fleet (+2 Climb) each moved
    // the hardcoded twin's number and moved the player's sheet by nothing.
    let feat_contributions =
        feat_derived_pillar_contributions(input, base.ability_modifiers.strength);

    let climb = rank
        + base.ability_modifiers.strength
        + climb_class_skill_bonus
        + armor_check_penalty
        + feat_contributions.climb_skill_bonus();
    let intimidate = rank
        + base.ability_modifiers.charisma
        + intimidate_class_skill_bonus
        + feat_contributions.intimidate_skill_bonus();
    let swim = rank
        + base.ability_modifiers.strength
        + swim_class_skill_bonus
        + armor_check_penalty
        + feat_contributions.swim_skill_bonus();

    Ok(CorpusAwareSelectedSkillModifiers { climb, intimidate, swim })
}

