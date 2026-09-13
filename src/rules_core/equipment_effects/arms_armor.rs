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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcgen_import::ir_converter::equipment_record_to_corpus;
    use crate::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
    use crate::rules_core::equipment_record::CorpusEquipmentRecord;

    /// The ingest-time conversion every live reader below is proved over: the
    /// same function `corpus_loader` runs for a real corpus record, applied to
    /// the real verbatim source line each test quotes.
    fn converted(record: &EquipmentRecord) -> CorpusEquipmentRecord {
        equipment_record_to_corpus(record)
    }

    /// Real verbatim tokens copied from `KEY:Leather Armor (Base)` in
    /// `core_rulebook/cr_equip_arms_armor.lst`.
    #[test]
    fn leather_armor_base_yields_real_armor_stats() {
        let text = "Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tACCHECK:0\tMAXDEX:6\tSPELLFAILURE:10\tBONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0\tBONUS:COMBAT|AC|-1|TYPE=Armor|PRETYPE:1,EQMOD=Special Quality ~ Broken ~ Armor\n";
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_arms_armor_effect(&converted(record));
        assert_eq!(effect.armor_class_bonus, Some(2));
        assert_eq!(effect.max_dex, Some(6));
        assert_eq!(effect.spell_failure, Some(10.0));
        assert_eq!(effect.armor_check_penalty, Some(0), "Leather Armor's real ACCHECK is 0");
    }

    /// Real verbatim tokens copied from `KEY:Buckler (Base)` (no
    /// `MAXDEX:` token on a buckler in the real corpus).
    #[test]
    fn buckler_base_has_shield_ac_bonus_but_no_max_dex_token() {
        let text = "Buckler\tKEY:Buckler (Base)\tTYPE:Shield.Buckler\tCOST:5\tWT:5\tACCHECK:-1\tSPELLFAILURE:5\tBONUS:COMBAT|AC|1|TYPE=Shield|PREVAREQ:DisableShieldBonus,0\n";
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", text);
        let record = &result.entries[0];

        let effect = compute_arms_armor_effect(&converted(record));
        assert_eq!(effect.armor_class_bonus, Some(1));
        assert_eq!(effect.max_dex, None);
        assert_eq!(effect.spell_failure, Some(5.0));
        assert_eq!(effect.armor_check_penalty, Some(-1), "Buckler's real ACCHECK is -1");
    }

    /// SD-33 Epic 5 combat/weapon lane: real verbatim tokens copied from
    /// `data/corpus/inner_sea_gods/equipment/knight_inheritor_s_ring.json`
    /// (`isg_equip.lst:160`) — a Ring of Protection-shaped AC bonus,
    /// `TYPE=Deflection`, not `TYPE=Armor`/`TYPE=Shield`. Before this
    /// cycle, `armor_class_bonus_from_bonus_chains`'s `TYPE=` allowlist
    /// (Armor/Shield/ArmorEnhancement/ShieldEnhancement) silently dropped
    /// this real, comparable, player-facing AC bonus to `None` even
    /// though `resolve_category_effect` calls this function
    /// unconditionally on every equipped item, not just base armor/
    /// shield records (see `equipment_effects.rs`'s own comment on why
    /// that call is unconditional). A Ring of Protection is exactly the
    /// canonical PF1 non-armor AC-bonus item this gap silently zeroed.
    #[test]
    fn ring_of_protection_shaped_deflection_ac_bonus_resolves() {
        let text = "Knight-Inheritor's Ring\tKEY:Knight-Inheritor's Ring\tTYPE:SLOT_Ring.Ring.Magic\tCOST:3000\tWT:0\tBONUS:COMBAT|AC|1|TYPE=Deflection\n";
        let result = parse_equipment_entries("isg_equip.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_arms_armor_effect(&converted(record));
        assert_eq!(
            effect.armor_class_bonus,
            Some(1),
            "a Deflection-type AC chain is a real, comparable magnitude, not an Armor/Shield-only one"
        );
    }

    /// SD-33 Epic 5 combat/weapon lane: real verbatim token copied from
    /// `data/corpus/ultimate_equipment/equipment/naga_scale_bindi_dark_naga.json`
    /// (`ue_equip_magic_items.lst:1209`) — the record's own real LST line
    /// is `BONUS:COMBAT|AC|4|NaturalArmor`, a bare bonus-type qualifier
    /// with no `TYPE=`/`TYPE.` prefix at all (confirmed against real
    /// PCGen source, `code/src/java/pcgen/core/bonus/Bonus.java`: a
    /// qualifier segment is only ever parsed as a bonus type when it
    /// literally starts with `TYPE=`/`TYPE.`; a bare `NaturalArmor`
    /// segment does not, and PCGen itself registers this bonus with an
    /// empty/default type). This engine's own literal magnitude (`4`) is
    /// still the real, comparable value on the record regardless of that
    /// grammar quirk — the widened match takes the chain's value
    /// unconditionally once `qualifiers[0]=="COMBAT"`/`qualifiers[1]==
    /// "AC"`, never gated on a specific `TYPE=` string being present.
    #[test]
    fn ac_bonus_with_a_bare_untyped_qualifier_still_resolves() {
        let text = "Naga-Scale Bindi (Dark Naga)\tKEY:Naga-Scale Bindi (Dark Naga)\tTYPE:Magic.Wondrous\tCOST:6600\tWT:0\tBONUS:COMBAT|AC|4|NaturalArmor\n";
        let result = parse_equipment_entries("ue_equip_magic_items.lst", text);
        let record = &result.entries[0];

        let effect = compute_arms_armor_effect(&converted(record));
        assert_eq!(effect.armor_class_bonus, Some(4));
    }

    /// Real verbatim tokens copied from `KEY:Longsword (Base)` — a
    /// weapon carries none of the armor-defining tokens at all.
    #[test]
    fn longsword_base_has_no_armor_stats() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n";
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", text);
        let record = &result.entries[0];

        let effect = compute_arms_armor_effect(&converted(record));
        assert_eq!(effect.armor_class_bonus, None);
        assert_eq!(effect.max_dex, None);
        assert_eq!(effect.spell_failure, None);
        assert_eq!(effect.armor_check_penalty, None, "a weapon carries no ACCHECK token at all");
    }

    /// Real verbatim tokens copied from `KEY:Special Ability ~ +1 ~
    /// Armor` in `core_rulebook/cr_equipmods.lst` (`equipment_modifier`
    /// kind, `SD31-W16-EQUIPMOD-001`). An armor-enhancement-bonus
    /// MODIFIER record carries no bare `ACCHECK:` token of its own (that
    /// token exists only on a BASE armor record) -- its AC contribution
    /// is stated as `BONUS:COMBAT|AC|1|TYPE=ArmorEnhancement`, one
    /// qualifier away from the base-item `TYPE=Armor` this module already
    /// recognized, and its check-penalty contribution is stated as
    /// `BONUS:EQMARMOR|ACCHECK|1|TYPE=Enhancement`, a token family this
    /// module did not read at all before this fix. Before the fix below,
    /// both fields read `None` even though the record carries real,
    /// verbatim corpus tokens for exactly these effects.
    #[test]
    fn special_ability_plus_one_armor_yields_enhancement_ac_and_acp_from_the_modifier_itself() {
        let text = "+1 (Enhancement to Armor)\tKEY:Special Ability ~ +1 ~ Armor\tTYPE:Armor\tPLUS:1\tBONUS:COMBAT|AC|1|TYPE=ArmorEnhancement|PREVAREQ:DisableArmorBonus,0\tBONUS:EQMARMOR|ACCHECK|1|TYPE=Enhancement|!PRETYPE:1,EQMODTYPE=MASTERWORKQUALITY\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_arms_armor_effect(&converted(record));
        assert_eq!(
            effect.armor_class_bonus,
            Some(1),
            "a +1 Armor Enhancement modifier's own TYPE=ArmorEnhancement chain is real AC magnitude"
        );
        assert_eq!(
            effect.armor_check_penalty,
            Some(1),
            "the same modifier's EQMARMOR|ACCHECK chain is its real check-penalty contribution"
        );
        assert_eq!(effect.max_dex, None, "this modifier carries no MAXDEX-shaped token at all");
    }

    /// Real verbatim tokens copied from `KEY:Material ~ Mithril ~ Armor /
    /// Light` in `core_rulebook/cr_equipmods.lst` — a material modifier
    /// with no `BONUS:COMBAT|AC|...` chain at all (mithral does not grant
    /// an enhancement AC bonus), but real `EQMARMOR|MAXDEX` and
    /// `EQMARMOR|SPELLFAILURE` chains this module did not read before
    /// this fix.
    #[test]
    fn mithril_light_armor_yields_max_dex_and_spell_failure_from_eqmarmor_chains() {
        let text = "Mithral\tKEY:Material ~ Mithril ~ Armor / Light\tTYPE:BaseMaterial.MasterworkQuality.Armor\tCOST:1000\tBONUS:EQMARMOR|ACCHECK|3|TYPE=Enhancement.REPLACE\tBONUS:EQMARMOR|MAXDEX|2\tBONUS:EQMARMOR|SPELLFAILURE|-10|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_arms_armor_effect(&converted(record));
        assert_eq!(effect.armor_class_bonus, None, "mithral grants no enhancement AC bonus");
        assert_eq!(effect.armor_check_penalty, Some(3), "mithral's real ACP improvement is +3");
        assert_eq!(effect.max_dex, Some(2), "mithral's real max-dex improvement is +2");
        assert_eq!(effect.spell_failure, Some(-10.0), "mithral's real spell-failure reduction is -10");
    }

    /// A base armor record's `ACCHECK:`/`MAXDEX:`/`SPELLFAILURE:` tokens
    /// still win over any `EQMARMOR` chain on the SAME record — proven
    /// against `Leather Armor (Base)`'s own real "Broken" chain shape
    /// (a `PRETYPE`-guarded `EQMARMOR|ACCHECK` variant some base records
    /// also carry), so the new fallback in
    /// `special_ability_plus_one_armor_yields_enhancement_ac_and_acp_from_the_modifier_itself`
    /// cannot shadow a real base-item token with a conditional chain.
    #[test]
    fn a_base_armor_records_own_acp_token_outranks_an_eqmarmor_broken_chain() {
        let text = "Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tACCHECK:0\tMAXDEX:6\tSPELLFAILURE:10\tBONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0\tBONUS:EQMARMOR|ACCHECK|-2|PRETYPE:1,EQMOD=Special Quality ~ Broken ~ Armor\n";
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", text);
        let record = &result.entries[0];

        let effect = compute_arms_armor_effect(&converted(record));
        assert_eq!(
            effect.armor_check_penalty,
            Some(0),
            "the record's own real ACCHECK:0 token must win over the conditional Broken EQMARMOR chain"
        );
    }

    /// SD-33 remediation wave 4 (`AT-33-E5-003`): real verbatim tokens
    /// copied from `advanced_race_guide/arg_equip_arms_armor.lst:46`
    /// (`KEY`-less, identity is `name`). This is the ONE real corpus
    /// record with a `TYPE=Circumstance` `COMBAT|AC` chain — the pinned
    /// oracle's standing reference character shows `0`, not the chain's
    /// literal `-2`, because the bonus only applies while "swimming,
    /// flying, or prone" (this record's own `SPROP`), a situational
    /// state this engine has no standing model of.
    #[test]
    fn a_circumstance_typed_ac_chain_is_conditional_not_a_standing_bonus() {
        let text = "Sea-Knife\tKEY:Sea-Knife\tTYPE:Weapon.Resizable.Light.Melee.Piercing.Slashing.Exotic.Finesseable\tCOST:8\tWT:1\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d4\tEQMOD:Material ~ Steel\tWIELD:Light\tSIZE:M\tBONUS:COMBAT|AC|-2|TYPE=Circumstance\tSPROP:The wearer cannot use a leg with a sea-knife strapped to it for walking or running.\n";
        let result = parse_equipment_entries("arg_equip_arms_armor.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_arms_armor_effect(&converted(record));
        assert_eq!(
            effect.armor_class_bonus, None,
            "a TYPE=Circumstance AC chain is situational, never a standing bonus this function reports"
        );
    }

    /// SD-33 remediation wave 4 (`AT-33-E5-003`): real verbatim tokens
    /// copied from `inner_sea_races/isr_equip_arms_armor.lst:12`
    /// (`Armor of Grim Triumph`) plus the base-armor-record's own
    /// `EQMOD:`-referenced modifier's real corpus record
    /// (`core_rulebook/cr_equipmods.lst`, `Special Ability ~ +1 ~
    /// Armor`). The base item's own chain alone (`Some(6)`) is
    /// Breastplate's base value; the pinned oracle's real total is `7`
    /// (`AT-33-E5-003.combined-oracle-results.json`) — the modifier's own
    /// separate `+1` enhancement chain, summed by
    /// `apply_eqmod_armor_class_bonus`.
    #[test]
    fn eqmod_referenced_enhancement_modifier_sums_into_the_base_items_ac_bonus() {
        let base_text = "Armor of Grim Triumph\tKEY:Armor of Grim Triumph\tTYPE:Armor.Magic.Medium.ArmorProfMedium.Suit.Specific\tCOST:250\tWT:40\tACCHECK:-4\tEQMOD:Special Ability ~ Enhancement Cost|12600.Special Ability ~ +1 ~ Armor.Special Quality ~ Spikes ~ Armor.Material ~ Steel\tMAXDEX:3\tSPELLFAILURE:25\tBONUS:COMBAT|AC|6|TYPE=Armor\n";
        let result = parse_equipment_entries("isr_equip_arms_armor.lst", base_text);
        let base_record = &result.entries[0];

        let modifier_text = "+1 (Enhancement to Armor)\tKEY:Special Ability ~ +1 ~ Armor\tTYPE:Armor\tPLUS:1\tBONUS:COMBAT|AC|1|TYPE=ArmorEnhancement|PREVAREQ:DisableArmorBonus,0\n";
        let modifier_result = parse_equipment_entries("cr_equipmods.lst", modifier_text);
        let modifier_record = &modifier_result.entries[0];

        // A real Spikes/Material-only reference resolves too (no chain
        // of its own) -- proving the sum is not just "the one modifier
        // that happens to matter", it genuinely adds only real per-record
        // magnitudes.
        let spikes_text = "Armor Spikes\tKEY:Special Quality ~ Spikes ~ Armor\tTYPE:Armor\tCOST:50\n";
        let spikes_result = parse_equipment_entries("cr_equipmods.lst", spikes_text);
        let spikes_record = &spikes_result.entries[0];

        let mut effect = compute_arms_armor_effect(&converted(base_record));
        assert_eq!(effect.armor_class_bonus, Some(6), "the base item's own chain alone is Breastplate's base value");

        apply_eqmod_armor_class_bonus(&mut effect, &[&converted(modifier_record), &converted(spikes_record)]);
        assert_eq!(
            effect.armor_class_bonus,
            Some(7),
            "the EQMOD-referenced +1 Armor modifier's own separate chain must sum in; Spikes contributes 0"
        );
    }

    /// `AT-34-E3-003` (bucket `M`, equipment sub-causes, cycle 5): real
    /// verbatim tokens copied from the committed corpus record
    /// (`data/corpus/core_rulebook/equipment/magic_items/
    /// cloak_of_the_manta_ray.json`, itself sourced from
    /// `cr_equip_magic_items.lst:109`). This record carries NO
    /// `BONUS:COMBAT|AC` chain at all (it declares no bonus chains at all —
    /// `pcgen_import::ingest_record::bonus_chain_qualifiers` returns empty) — its
    /// real +3 natural armor bonus is stated only as
    /// `TEMPBONUS:PC|COMBAT|AC|3|TYPE=NaturalArmor`, which
    /// `armor_class_bonus_from_bonus_chains` alone cannot see (it reads
    /// `bonus_chains`, never `TEMPBONUS`).
    #[test]
    fn tempbonus_combat_ac_token_resolves_when_no_bonus_chain_exists() {
        let text = "Cloak of the Manta Ray\tKEY:Cloak of the Manta Ray\tTYPE:Magic.Wondrous.Shoulders.Cloak\tCOST:7200\tWT:1\tEQMOD:Material ~ Cloth\tTEMPBONUS:PC|COMBAT|AC|3|TYPE=NaturalArmor\n";
        let result = parse_equipment_entries("cr_equip_magic_items.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_arms_armor_effect(&converted(record));
        assert_eq!(
            effect.armor_class_bonus,
            Some(3),
            "TEMPBONUS:PC|COMBAT|AC|3|TYPE=NaturalArmor is the item's real, literal +3 natural armor bonus"
        );
    }

    /// A base record's own real `BONUS:COMBAT|AC` chain still wins over a
    /// `TEMPBONUS:...|COMBAT|AC` fallback on the SAME record — the same
    /// `.or_else` discipline every other fallback in this file follows
    /// (`eqmarmor_chain_value`, `tempbonus_skill_fallback` in
    /// `general.rs`). No real corpus record carries both today; this is a
    /// negative control proving the ordering, not a shape this cycle
    /// found.
    #[test]
    fn a_records_own_bonus_combat_ac_chain_outranks_the_tempbonus_fallback() {
        let text = "Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tBONUS:COMBAT|AC|2|TYPE=Armor\tTEMPBONUS:PC|COMBAT|AC|99|TYPE=NaturalArmor\n";
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", text);
        let record = &result.entries[0];

        let effect = compute_arms_armor_effect(&converted(record));
        assert_eq!(
            effect.armor_class_bonus,
            Some(2),
            "the record's own real BONUS:COMBAT|AC chain must win over the TEMPBONUS fallback"
        );
    }

    /// `TEMPBONUS:EQ|...` is a structurally different, equipment-side
    /// effect (the same distinction `general.rs`'s `tempbonus_skill_
    /// fallback` already draws for `SKILL`) — never read as a character
    /// AC bonus. No real corpus record carries this shape for `COMBAT|AC`
    /// today; this negative control proves the guard, matching this
    /// file's existing style of proving absence as deliberately as
    /// presence.
    #[test]
    fn an_eq_targeted_tempbonus_is_never_read_as_a_character_ac_bonus() {
        let text = "Fake EQ-Targeted Item\tKEY:Fake EQ-Targeted Item\tTYPE:Magic.Wondrous\tCOST:1\tWT:0\tTEMPBONUS:EQ|COMBAT|AC|5|TYPE=NaturalArmor\n";
        let result = parse_equipment_entries("cr_equip_magic_items.lst", text);
        let record = &result.entries[0];

        let effect = compute_arms_armor_effect(&converted(record));
        assert_eq!(
            effect.armor_class_bonus, None,
            "an EQ-targeted TEMPBONUS is equipment-side, never a character AC bonus"
        );
    }
}
