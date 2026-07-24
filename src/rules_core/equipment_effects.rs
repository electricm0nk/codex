//! Epic 5 — equipment-effect engine (SD-20 §1.5).
//!
//! Extends SD-19's bounded equipment baseline
//! (`pilot_compute_corpus::DerivedEquipmentStats`, deliberately left
//! `Default`-valued per that module's own doc comment: "wrapping resolved
//! equipment with its (currently unpopulated — bounded-baseline non-goal)
//! derived stats") with real per-item armor/shield stats, one CRB
//! equipment category per cycle
//! (`scope-draft.md` §1.5 work-unit order: `arms_armor`, then `general`,
//! `magic_items`, `equipmods`). `arms_armor` landed first — see
//! `equipment_effects/arms_armor.rs`. `general` landed second — see
//! `equipment_effects/general.rs`; unlike `arms_armor`'s AC/max-dex/
//! spell-failure stats, the `general` category's real load-bearing field
//! is a per-item skill-check circumstance bonus
//! (`ResolvedEquipmentEffect::skill_bonus`), so `general` does not
//! populate `EquipmentStatEffect` at all (that type stays scoped to the
//! armor/shield fields `arms_armor` defined it for). `magic_items` landed
//! third — see `equipment_effects/magic_items.rs`; like `general`, its
//! real load-bearing field (a per-item ability-score enhancement bonus,
//! `ResolvedEquipmentEffect::ability_bonus`) does not fit
//! `EquipmentStatEffect` either, so it follows the same
//! shared-struct-extension pattern `general` established (a new
//! `ResolvedEquipmentEffect` field, not a new `EquipmentStatEffect`
//! field). This cycle lands `equipmods` — see
//! `equipment_effects/equipmods.rs`, **closing Epic 5** (all four CRB
//! equipment categories done); its real load-bearing field is a per-item
//! weapon to-hit/damage enhancement bonus
//! (`ResolvedEquipmentEffect::weapon_enhancement_bonus`), following the
//! same shared-struct-extension pattern.
//!
//! Adapts `technical-design.md` §2.4's illustrative
//! `compute_equipment_effects(equipped: &[EquipmentSelection], rules_tables:
//! &RulesTables) -> EquipmentEffects` seam to this repo's real types —
//! `RulesTables` does not exist anywhere in this codebase (same situation
//! `pilot_compute_corpus::compute_pilot_with_corpus`'s own doc comment
//! notes for the doctrine doc's illustrative `PilotReceipt`: "does not
//! exist in this repo"). Corpus resolution here goes through the exact
//! same `SourcePackageContent` + `equipment_resolver::equipment_id_resolve`
//! path every other SD-19/20 corpus-derived field uses, and category
//! membership comes from a `TableCellRef`-style lookup by the resolved
//! record's `KEY:` token against the canonical
//! `rules_tables::crb::equipment_tables` store — not re-derived from raw
//! corpus text.

pub mod arms_armor;
pub mod equipmods;
pub mod general;
pub mod magic_items;

use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;
use crate::rules_core::character_input::EquipmentSelection;
use crate::rules_core::equipment_effects::equipmods::WeaponEnhancementBonus;
use crate::rules_core::equipment_effects::general::SkillCheckBonus;
use crate::rules_core::equipment_effects::magic_items::AbilityScoreBonus;
use crate::rules_core::equipment_resolver::{equipment_id_resolve, equipment_key_token};
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::equipment_tables::{equipment_tables, EquipmentCategory};
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::source_content::SourcePackageContent;

/// Per-category stat contribution shared across every
/// `equipment_effects/<category>.rs` file. `None` means the category's
/// resolver has not populated that field (either because the underlying
/// corpus record carries no such token, or — for a category whose cycle
/// has not landed yet — because no resolver exists yet at all).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct EquipmentStatEffect {
    pub armor_class_bonus: Option<i16>,
    pub max_dex: Option<i16>,
    pub spell_failure: Option<f32>,
    /// The record's `ACCHECK:` token (v0.6 alpha swarm item 1, shape (c)):
    /// a negative or zero value, PF1's usual convention for a penalty. Read
    /// the same way `max_dex`/`spell_failure` already read their own
    /// tokens; `arms_armor.rs`'s own module doc comment already cited this
    /// exact token as present on the records it resolves, it was simply
    /// never extracted into this struct until now.
    pub armor_check_penalty: Option<i16>,
}

/// One resolved equipment selection's computed effect.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedEquipmentEffect {
    pub item_id: String,
    pub equipment_record_key: String,
    pub category: EquipmentCategory,
    pub armor_class_bonus: Option<i16>,
    pub max_dex: Option<i16>,
    pub spell_failure: Option<f32>,
    /// Mirrors `EquipmentStatEffect::armor_check_penalty` exactly (v0.6
    /// alpha swarm item 1, shape (c)).
    pub armor_check_penalty: Option<i16>,
    /// The `general` category's per-item skill-check circumstance bonus
    /// (see `equipment_effects/general.rs`). `None` for every other
    /// category, and for a `general` record that carries no
    /// `BONUS:SKILL|...` token.
    pub skill_bonus: Option<SkillCheckBonus>,
    /// The `magic_items` category's per-item ability-score enhancement
    /// bonus (see `equipment_effects/magic_items.rs`). `None` for every
    /// other category, and for a `magic_items` record that carries no
    /// `BONUS:STAT|...` token.
    pub ability_bonus: Option<AbilityScoreBonus>,
    /// The `equipmods` category's per-item weapon to-hit/damage
    /// enhancement bonus (see `equipment_effects/equipmods.rs`). `None`
    /// for every other category, and for an `equipmods` record that
    /// carries no matching `BONUS:WEAPON|...|TYPE=Enhancement` token.
    pub weapon_enhancement_bonus: Option<WeaponEnhancementBonus>,
    pub table_cell: Option<TableCellRef>,
}

/// Aggregate equipment-effect result for one character's full equipped
/// loadout. `armor_class_delta` sums every resolved item's
/// `armor_class_bonus` (PF1 lets armor and shield bonuses stack, unlike
/// most other bonus types). `max_dex_cap` is the tightest (lowest)
/// `max_dex` among items that carry one — an unarmored or shieldless
/// loadout leaves it `None` (uncapped), exactly like the real rule.
/// `spell_failure_chance` sums every resolved item's `spell_failure` —
/// armor and shield arcane spell-failure chances stack additively per
/// PF1's rule. `armor_check_penalty_total` sums every resolved item's
/// `armor_check_penalty` the same way (v0.6 alpha swarm item 1, shape (c))
/// — PF1's rule: armor and shield check penalties add together when both
/// are worn, the same additive-stacking shape `armor_class_delta` already
/// uses. Any item whose category has not resolved a value (`None`)
/// contributes nothing to any aggregate rather than a fabricated zero.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EquipmentEffects {
    pub per_item: Vec<ResolvedEquipmentEffect>,
    pub armor_class_delta: i16,
    pub max_dex_cap: Option<i16>,
    pub spell_failure_chance: Option<f32>,
    pub armor_check_penalty_total: i16,
}

/// The equipment-effect engine seam (`technical-design.md` §2.4, adapted
/// to real types — see module doc comment). Resolves every selection
/// against the corpus, looks up its category in the canonical CRB
/// equipment-table store, dispatches to that category's per-category
/// function, and aggregates the results.
pub fn compute_equipment_effects(
    equipped: &[EquipmentSelection],
    corpus: &SourcePackageContent,
) -> EquipmentEffects {
    let mut per_item = Vec::new();
    let mut armor_class_delta: i16 = 0;
    let mut max_dex_cap: Option<i16> = None;
    let mut spell_failure_chance: Option<f32> = None;
    let mut armor_check_penalty_total: i16 = 0;

    for selection in equipped {
        let Some((record, table_cell)) =
            equipment_id_resolve(&selection.item_id, RuleSetId::Crb, corpus)
        else {
            continue;
        };
        let key = equipment_key_token(record)
            .unwrap_or(&record.name)
            .to_string();
        // TableCellRef-style lookup against the canonical store: category
        // membership comes from `equipment_tables()`, not re-derived from
        // raw corpus TYPE: text.
        let Some(category) = equipment_tables()
            .iter()
            .find(|entry| entry.key == key)
            .map(|entry| entry.category)
        else {
            continue;
        };

        let effect = resolve_category_effect(category, record);
        let skill_bonus = match category {
            EquipmentCategory::General => general::compute_general_effect(record),
            EquipmentCategory::ArmsArmor | EquipmentCategory::MagicItems | EquipmentCategory::Equipmods => None,
        };
        let ability_bonus = match category {
            EquipmentCategory::MagicItems => magic_items::compute_magic_items_effect(record),
            EquipmentCategory::ArmsArmor | EquipmentCategory::General | EquipmentCategory::Equipmods => None,
        };
        let weapon_enhancement_bonus = match category {
            EquipmentCategory::Equipmods => equipmods::compute_equipmods_effect(record),
            EquipmentCategory::ArmsArmor | EquipmentCategory::General | EquipmentCategory::MagicItems => None,
        };

        if let Some(bonus) = effect.armor_class_bonus {
            armor_class_delta += bonus;
        }
        if let Some(dex) = effect.max_dex {
            max_dex_cap = Some(max_dex_cap.map_or(dex, |current| current.min(dex)));
        }
        if let Some(failure) = effect.spell_failure {
            spell_failure_chance =
                Some(spell_failure_chance.map_or(failure, |current| current + failure));
        }
        if let Some(penalty) = effect.armor_check_penalty {
            armor_check_penalty_total += penalty;
        }

        per_item.push(ResolvedEquipmentEffect {
            item_id: selection.item_id.clone(),
            equipment_record_key: key,
            category,
            armor_class_bonus: effect.armor_class_bonus,
            max_dex: effect.max_dex,
            spell_failure: effect.spell_failure,
            armor_check_penalty: effect.armor_check_penalty,
            skill_bonus,
            ability_bonus,
            weapon_enhancement_bonus,
            table_cell,
        });
    }

    EquipmentEffects {
        per_item,
        armor_class_delta,
        max_dex_cap,
        spell_failure_chance,
        armor_check_penalty_total,
    }
}

fn resolve_category_effect(category: EquipmentCategory, record: &EquipmentRecord) -> EquipmentStatEffect {
    match category {
        EquipmentCategory::ArmsArmor => arms_armor::compute_arms_armor_effect(record),
        // `General`'s real per-item field is `skill_bonus`,
        // `MagicItems`'s real per-item field is `ability_bonus`, and
        // `Equipmods`'s real per-item field is `weapon_enhancement_bonus`
        // (all three computed in the loop in `compute_equipment_effects`
        // above), not an `EquipmentStatEffect` field — none of these
        // three categories' records carry AC/max-dex/spell-failure
        // tokens.
        EquipmentCategory::General | EquipmentCategory::MagicItems | EquipmentCategory::Equipmods => {
            EquipmentStatEffect::default()
        }
    }
}
