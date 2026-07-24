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
///
/// `attack_bonus_delta` (v0.6 alpha swarm item 1, the bounded single-weapon
/// attack-bonus slice, greenlit as an engineering-scope call distinct from
/// item 27's claim-gating philosophy question) sums the to-hit-affecting
/// `weapon_enhancement_bonus` of every `equipmods` item, but ONLY when
/// `equipped`'s `EquippedActive` selections resolve to *exactly one* real
/// weapon (a record carrying a `DAMAGE:` token, the same signal
/// `damage_total.rs`'s own `resolve_base_damage_dice` uses to identify a
/// weapon). `CharacterInput`'s schema has no field recording which weapon
/// a modifier item attaches to (see `item-1-architecture-wall-design.md`),
/// so with zero or two-or-more weapons equipped, which weapon (if any) an
/// enhancement modifies is genuinely ambiguous -- `None` in both cases,
/// honest absence rather than a guess. With exactly one weapon equipped
/// there is no ambiguity: any equipped enhancement modifier unambiguously
/// applies to that one weapon.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EquipmentEffects {
    pub per_item: Vec<ResolvedEquipmentEffect>,
    pub armor_class_delta: i16,
    pub max_dex_cap: Option<i16>,
    pub spell_failure_chance: Option<f32>,
    pub armor_check_penalty_total: i16,
    pub attack_bonus_delta: Option<i16>,
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
    let mut weapon_count: u32 = 0;
    let mut to_hit_bonus_total: i16 = 0;

    for selection in equipped {
        let Some((record, table_cell)) =
            equipment_id_resolve(&selection.item_id, RuleSetId::Crb, corpus)
        else {
            continue;
        };
        if is_weapon_record(record) {
            weapon_count += 1;
        }
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
        if let Some(bonus) = &weapon_enhancement_bonus {
            if bonus.affects.contains("TOHIT") {
                to_hit_bonus_total += bonus.bonus;
            }
        }

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

    // v0.6 alpha swarm item 1, bounded single-weapon attack-bonus slice:
    // unambiguous only with exactly one real weapon equipped -- see
    // `EquipmentEffects.attack_bonus_delta`'s own doc comment.
    let attack_bonus_delta = (weapon_count == 1).then_some(to_hit_bonus_total);

    EquipmentEffects {
        per_item,
        armor_class_delta,
        max_dex_cap,
        spell_failure_chance,
        armor_check_penalty_total,
        attack_bonus_delta,
    }
}

/// Whether `record` is a real weapon -- carries a `DAMAGE:` corpus token.
/// The same signal `damage_total.rs`'s own `resolve_base_damage_dice` uses
/// to identify a weapon (re-declared here rather than imported, to avoid a
/// circular module dependency: `damage_total.rs` already imports
/// `EquipmentEffects` from this file). Armor, shields, and every other
/// non-weapon item carry no `DAMAGE:` token at all.
fn is_weapon_record(record: &EquipmentRecord) -> bool {
    record.tokens.iter().any(|token| token.key == "DAMAGE")
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

#[cfg(test)]
mod attack_bonus_delta_tests {
    use super::*;
    use crate::pcgen_import::ir_converter::convert_equipment_record;
    use crate::pcgen_import::lst_parser::equipment::parse_equipment_entries;
    use crate::rules_core::character_input::ActiveState;
    use crate::rules_core::source_content::SourceRef;

    // Real verbatim corpus tokens: Longsword/Chain Shirt/Masterwork quality
    // match `tests/sd20_contract_equipment_wiring.rs`'s own fixture exactly.
    // Dagger's DAMAGE:1d4 is the real CRB value. The `+1 (Enhancement to
    // Weapon)` line matches `equipmods.rs`'s own test fixture exactly. The
    // DAMAGE-only enhancement line uses the same `BONUS:WEAPON|<...>|<n>|
    // TYPE=Enhancement` grammar `equipmods.rs`'s own module doc comment
    // documents as a real, observed corpus shape (`TOHIT`, `DAMAGE`, or
    // `DAMAGE,TOHIT`) -- exercising the `DAMAGE`-only case specifically,
    // which no existing fixture in this codebase happens to cover.
    const FIXTURE_TEXT: &str = "\
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8
Dagger\tKEY:Dagger (Base)\tTYPE:Weapon.Melee.Simple\tCOST:2\tWT:1\tCRITMULT:x2\tDAMAGE:1d4
Chain Shirt\tKEY:Chain Shirt (Base)\tTYPE:Armor.Light\tCOST:100\tWT:25\tACCHECK:-2\tMAXDEX:4\tSPELLFAILURE:20\tBONUS:COMBAT|AC|4|TYPE=Armor|PREVAREQ:DisableArmorBonus,0
+1 (Enhancement to Weapon)\tKEY:Special Ability ~ +1 ~ Weapon\tTYPE:Weapon\tPLUS:1\tCOST:0\tBONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement
Masterwork (Weapon)\tKEY:Special Quality ~ Masterwork ~ Weapon\tTYPE:MasterworkQuality.Weapon\tCOST:0\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement
Flaming\tKEY:Special Ability ~ Flaming ~ Weapon\tTYPE:Weapon\tCOST:0\tBONUS:WEAPON|DAMAGE|2|TYPE=Enhancement
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

    fn equipped(item_id: &str) -> EquipmentSelection {
        EquipmentSelection {
            item_id: item_id.to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        }
    }

    #[test]
    fn exactly_one_weapon_plus_a_real_enhancement_yields_a_real_attack_bonus() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![
            equipped("Longsword (Base)"),
            equipped("Special Ability ~ +1 ~ Weapon"),
        ];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(effects.attack_bonus_delta, Some(1), "unambiguous single weapon: +1 TOHIT applies");
    }

    #[test]
    fn a_masterwork_tohit_only_bonus_also_counts() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![
            equipped("Longsword (Base)"),
            equipped("Special Quality ~ Masterwork ~ Weapon"),
        ];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(effects.attack_bonus_delta, Some(1));
    }

    #[test]
    fn a_damage_only_enhancement_does_not_affect_the_attack_bonus() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped("Longsword (Base)"), equipped("Special Ability ~ Flaming ~ Weapon")];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(
            effects.attack_bonus_delta,
            Some(0),
            "a DAMAGE-only enhancement must not contribute to the TOHIT-affecting attack bonus, \
             but the field itself is still a real, unambiguous Some(0), not None"
        );
    }

    #[test]
    fn exactly_one_weapon_with_no_enhancement_yields_a_real_zero() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped("Longsword (Base)"), equipped("Chain Shirt (Base)")];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(
            effects.attack_bonus_delta,
            Some(0),
            "no ambiguity with one weapon; the real value is honestly zero, not absent"
        );
    }

    #[test]
    fn zero_weapons_equipped_leaves_the_attack_bonus_honestly_absent() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped("Chain Shirt (Base)")];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(effects.attack_bonus_delta, None, "no weapon at all -- nothing to attach a bonus to");
    }

    #[test]
    fn two_weapons_equipped_leaves_the_attack_bonus_honestly_ambiguous() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![
            equipped("Longsword (Base)"),
            equipped("Dagger (Base)"),
            equipped("Special Ability ~ +1 ~ Weapon"),
        ];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(
            effects.attack_bonus_delta, None,
            "two weapons equipped: which one the enhancement attaches to is genuinely \
             ambiguous, must stay honestly absent rather than guess"
        );
    }
}
