//! Epic 5, first equipment category (SD-20 §1.5 work-unit order): CRB
//! `arms_armor` per-item effect resolution.
//!
//! The canonical CRB equipment-table store
//! (`rules_tables::crb::equipment_tables`) only carries `key` / `category`
//! / `name` / `cost_gp` per record (see that module's own doc comment: a
//! "bootstrap coverage" projection). It does not yet carry the
//! armor/shield stat columns (`ACCHECK:` / `MAXDEX:` / `SPELLFAILURE:` /
//! `BONUS:COMBAT|AC|...`) this epic needs. Those columns *are* present on
//! the real PCGen corpus record (`EquipmentRecord.tokens` /
//! `EquipmentRecord.bonus_chains`), verified directly against
//! `core_rulebook/cr_equip_arms_armor.lst` (e.g. `KEY:Leather Armor
//! (Base)` carries `ACCHECK:0`, `MAXDEX:6`, `SPELLFAILURE:10`, and
//! `BONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0`) — so this
//! function reads them straight off the resolved record, exactly the
//! pattern `equipment_resolver.rs` already uses for `KEY:`/`name`
//! resolution and `spell_resolver.rs` uses for spell school. No field
//! here is hand-rolled or fabricated; every value traces back to a real,
//! verbatim corpus token.
//!
//! **SD-35 `AT-35-E6-003-RULED` cycle 11.** Every rule and every real-corpus
//! witness named above is unchanged and still real. What moved is WHERE the
//! reading happens: once, at ingest, in
//! [`crate::pcgen_import::ir_converter::equipment_record_to_corpus`], which is
//! where `decisions.md` §11 rules that rule conversion belongs. The functions
//! below report the settled value off
//! [`crate::rules_core::equipment_record::CorpusEquipmentRecord`] and name no
//! ingest-format token at all.

use crate::rules_core::equipment_effects::EquipmentStatEffect;
use crate::rules_core::equipment_record::CorpusEquipmentRecord;

/// One `arms_armor` corpus record's armor/shield stat contribution.
///
/// - `armor_class_bonus` is the item's first standing `AC` bonus magnitude,
///   falling back to the consumable-triggered form for the one real item
///   family that states its natural-armor bonus only that way
///   (`core_rulebook:equipment:cloak_of_the_manta_ray`, a real `+3`). A
///   situational (circumstance) AC bonus is never reported as a standing one:
///   the pinned oracle's standing reference character shows `0`, not the
///   literal, for the single corpus record of that shape
///   (`advanced_race_guide:equipment:sea_knife`).
/// - `max_dex`, `spell_failure` and `armor_check_penalty` are the base
///   armour/shield record's own stated values, falling back to the
///   contribution a material/masterwork/enhancement MODIFIER record states
///   instead (a modifier never carries the base-record form -- e.g.
///   `Material ~ Mithril ~ Armor / Light`'s real `+3` check-penalty and `+2`
///   max-Dex improvements).
///
/// SD-35 `AT-35-E6-003-RULED` cycle 11: every rule above is unchanged and
/// every witness still real, but the reading itself happens once, at ingest,
/// in [`crate::pcgen_import::ir_converter::equipment_record_to_corpus`]
/// (`decisions.md` §11 -- rule conversion happens at ingest into our own
/// schema; §1 -- the sheet prints one settled number). This function reports
/// the settled value.
///
/// Absence (`None`) is honest: the record does not state that value, not that
/// the value is zero.
pub fn compute_arms_armor_effect(record: &CorpusEquipmentRecord) -> EquipmentStatEffect {
    record.stat_effect
}

/// Sums every attached modifier item's own standing AC contribution into
/// `effect.armor_class_bonus`.
///
/// SD-33 remediation wave 4 (`AT-33-E5-003`): a base armour/shield item's own
/// magnitude (what [`compute_arms_armor_effect`] alone reports) is only the
/// item's base value. A magic armour's enhancement bonus is stated on a
/// SEPARATE modifier record the base item names -- `Armor of Grim Triumph`'s
/// own `6` is Breastplate's base, its attached `Special Ability ~ +1 ~ Armor`
/// carries the real `+1`, and the oracle's total is `7`. Summing (rather than
/// taking the highest, as the weapon dimension does) is PF1's rule here: an
/// armour's base value and its enhancement bonus are two different bonus
/// types.
///
/// Every attached non-enhancement modifier (materials, cosmetic qualities
/// like Spikes) states no AC contribution at all and adds exactly nothing, so
/// calling this over every resolved modifier is safe.
pub fn apply_eqmod_armor_class_bonus(
    effect: &mut EquipmentStatEffect,
    eqmod_records: &[&CorpusEquipmentRecord],
) {
    let extra: i16 = eqmod_records.iter().filter_map(|modifier| modifier.armor_class_chain_bonus).sum();
    if extra != 0 {
        effect.armor_class_bonus = Some(effect.armor_class_bonus.unwrap_or(0) + extra);
    }
}

