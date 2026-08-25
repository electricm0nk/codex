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

use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;
use crate::rules_core::equipment_effects::EquipmentStatEffect;

/// Resolve one `arms_armor` corpus record's armor/shield stat
/// contribution.
///
/// - `armor_class_bonus` comes from the record's first
///   `BONUS:COMBAT|AC|<n>|TYPE=Armor` (or `TYPE=Shield`) chain — the
///   record's own "Broken" penalty chain
///   (`PRETYPE:1,EQMOD=Special Quality ~ Broken ~ Armor`) is a
///   conditional variant that only applies to a broken item and is never
///   the first `COMBAT|AC` chain on an unbroken record, so taking the
///   first match is the correct default (non-broken) armor/shield bonus.
///   `SD31-W16-EQUIPMOD-001` widened the recognized `TYPE=` set to also
///   accept `TYPE=ArmorEnhancement`/`TYPE=ShieldEnhancement` — the shape
///   an `equipment_modifier` armor/shield enhancement-bonus special
///   ability (`KEY:Special Ability ~ +1 ~ Armor` through `~ +5 ~
///   Shield`) states its own AC contribution in, verified directly
///   against the real `core_rulebook/cr_equipmods.lst` corpus records.
/// - `max_dex` and `spell_failure` come straight off the `MAXDEX:` and
///   `SPELLFAILURE:` tokens when present (weapons and shieldless items
///   carry neither, so both are `None` for e.g. a longsword) — falling
///   back to the record's own `BONUS:EQMARMOR|MAXDEX|...` /
///   `BONUS:EQMARMOR|SPELLFAILURE|...` chain only when no bare token
///   exists (`SD31-W16-EQUIPMOD-001`: an `equipment_modifier` material
///   record like `KEY:Material ~ Mithril ~ Armor / Light` states its
///   real max-dex/spell-failure contribution only in that chain family,
///   never as a bare token — those live exclusively on BASE armor
///   records).
/// - `armor_check_penalty` comes straight off the `ACCHECK:` token (v0.6
///   alpha swarm item 1, shape (c)) — present on every armor/shield
///   record (`0` for no penalty, a negative number for a real one), the
///   same token this module's own doc comment already cited as present
///   on `KEY:Leather Armor (Base)` before this field existed to hold it.
///   `SD31-W16-EQUIPMOD-001`'s same `BONUS:EQMARMOR|ACCHECK|...`
///   fallback applies here too (a masterwork/material/magic-enhancement
///   modifier's own check-penalty improvement, e.g.
///   `KEY:Special Ability ~ +1 ~ Armor`'s `EQMARMOR|ACCHECK|1|
///   TYPE=Enhancement`) — never consulted when the bare `ACCHECK:` token
///   is present, so a base record's own real value (including its
///   conditional "Broken" `EQMARMOR|ACCHECK` chain, which only ever
///   accompanies a real `ACCHECK:` token) is never shadowed.
///
/// Absence (`None`) is honest: it means this record's raw tokens do not
/// carry that field, not that the field's value is zero.
pub fn compute_arms_armor_effect(record: &EquipmentRecord) -> EquipmentStatEffect {
    EquipmentStatEffect {
        armor_class_bonus: armor_class_bonus_from_bonus_chains(record),
        max_dex: token_i16(record, "MAXDEX").or_else(|| eqmarmor_chain_value(record, "MAXDEX")),
        spell_failure: token_value(record, "SPELLFAILURE")
            .and_then(|value| value.parse().ok())
            .or_else(|| eqmarmor_chain_value(record, "SPELLFAILURE").map(f32::from)),
        armor_check_penalty: token_i16(record, "ACCHECK")
            .or_else(|| eqmarmor_chain_value(record, "ACCHECK")),
    }
}

fn token_value<'a>(record: &'a EquipmentRecord, key: &str) -> Option<&'a str> {
    record
        .tokens
        .iter()
        .find(|token| token.key == key)
        .map(|token| token.value.as_str())
}

fn token_i16(record: &EquipmentRecord, key: &str) -> Option<i16> {
    token_value(record, key).and_then(|value| value.parse().ok())
}

fn armor_class_bonus_from_bonus_chains(record: &EquipmentRecord) -> Option<i16> {
    record.bonus_chains.iter().find_map(|bonus| {
        let qualifiers = &bonus.qualifiers;
        // SD-33 Epic 5 combat/weapon lane: widened from an Armor/Shield-
        // only `TYPE=` allowlist to any `COMBAT|AC|<n>` chain, regardless
        // of its bonus-type qualifier (or the qualifier's absence).
        // `resolve_category_effect` (`equipment_effects.rs`) already
        // calls this function unconditionally on EVERY equipped item,
        // not just base armor/shield records, so an item like a Ring of
        // Protection (`TYPE=Deflection`) or an Amulet of Natural Armor
        // (`TYPE=NaturalArmor`) carries an equally real, comparable
        // `COMBAT|AC` magnitude the old Armor/Shield-only gate silently
        // dropped to `None`. Also handles a real corpus grammar quirk
        // confirmed against PCGen's own parser
        // (`pcgen.core.bonus.Bonus.newBonus`, `code/src/java/pcgen/core/
        // bonus/Bonus.java`): a qualifier segment is only ever parsed as
        // a bonus type when it literally starts with `TYPE=`/`TYPE.`, so
        // a real corpus line like `BONUS:COMBAT|AC|4|NaturalArmor`
        // (`ultimate_equipment/ue_equip_magic_items.lst:1209`, no `TYPE=`
        // prefix at all) still carries a real literal magnitude on this
        // record even though PCGen itself never registers a `TYPE=`
        // string for it. The record's own "Broken" penalty chain is
        // still never the first `COMBAT|AC` chain on an unbroken record
        // (see this function's own doc comment above), so taking the
        // first match is still the correct default.
        //
        // SD-33 remediation wave 4 (`AT-33-E5-003`): `TYPE=Circumstance`
        // is excluded from this otherwise-unconditional match. A
        // circumstance AC bonus is, by PF1's own rules definition,
        // conditional on a specific in-game situation the item's holder
        // must be in (the one real corpus instance,
        // `advanced_race_guide:equipment:sea_knife`'s
        // `BONUS:COMBAT|AC|-2|TYPE=Circumstance`, only applies while
        // "swimming, flying, or prone" per the record's own `SPROP`) —
        // never a standing armor/shield/deflection/natural-armor/
        // enhancement-style AC contribution, which is what every other
        // `TYPE=` this widened match accepts represents. Reading it
        // unconditionally produced a real, confirmed disagreement
        // against the pinned oracle's standing (not prone/swimming)
        // reference character (`ours=-2`, oracle=`0`,
        // `AT-33-E5-003.combined-oracle-results.json`). Confirmed the
        // only record in the whole corpus with this exact shape (a
        // `python3` sweep of every `data/corpus/*/equipment*/**/*.json`
        // record's own `raw_bonus_chains` for `COMBAT|AC|*|TYPE=Circumstance`
        // finds exactly 1), so this exclusion cannot regress any other
        // already-verified unit.
        let is_ac_bonus = qualifiers.len() >= 3
            && qualifiers[0] == "COMBAT"
            && qualifiers[1] == "AC"
            && !qualifiers.iter().any(|q| q == "TYPE=Circumstance");
        if is_ac_bonus {
            qualifiers[2].parse::<i16>().ok()
        } else {
            None
        }
    })
}

/// Sums every EQMOD-referenced modifier record's own `COMBAT|AC` chain
/// (via [`armor_class_bonus_from_bonus_chains`], applied to each
/// modifier's own record) into `effect.armor_class_bonus`.
///
/// SD-33 remediation wave 4 (`AT-33-E5-003`): a base armor/shield item's
/// own literal `COMBAT|AC` chain (what [`compute_arms_armor_effect`]
/// alone reads) is only the item's OWN base value. A real magic
/// armor/shield item's enhancement bonus is stated on a *separate*
/// `equipment_modifier` corpus record the base item's own `EQMOD:` token
/// references by name (e.g. `KEY:Armor of Grim Triumph`'s own
/// `BONUS:COMBAT|AC|6|TYPE=Armor` chain is Breastplate's base 6; its
/// `EQMOD:...Special Ability ~ +1 ~ Armor...` token names a *different*,
/// separately-resolvable corpus record whose own
/// `BONUS:COMBAT|AC|1|TYPE=ArmorEnhancement` chain is the real +1
/// enhancement — oracle's real total is 7, not 6). Neither
/// `compute_arms_armor_effect` nor any prior cycle resolved and summed
/// that second record; this is the real, root-caused engine gap named
/// across 21 of `AT-33-E5-003`'s 26 disagreements
/// (`eqmod_embedded_modifier_chain_not_summed`) plus one more this cycle
/// root-caused the same way (`diviner_s_blight`, previously
/// "undiagnosed" — `9 - 4` under the same mechanism reproduces its
/// prior wave's own oracle value exactly).
///
/// Every EQMOD-referenced non-`+N`/enhancement modifier this cycle
/// examined (materials, cosmetic special qualities like Spikes/
/// Martyring) carries no `COMBAT|AC` chain of its own at all (confirmed
/// directly against each real corpus record this fix's own tests and
/// the disagreement-fix verification pass reference), so calling this
/// unconditionally on every resolved modifier is safe: it adds exactly
/// the real enhancement records' own magnitude and nothing else, never
/// fabricated, never double-counted.
pub fn apply_eqmod_armor_class_bonus(effect: &mut EquipmentStatEffect, eqmod_records: &[&EquipmentRecord]) {
    let extra: i16 = eqmod_records
        .iter()
        .filter_map(|modifier| armor_class_bonus_from_bonus_chains(modifier))
        .sum();
    if extra != 0 {
        effect.armor_class_bonus = Some(effect.armor_class_bonus.unwrap_or(0) + extra);
    }
}

/// An `equipment_modifier` record's own `BONUS:EQMARMOR|<field>|<n>[|...]`
/// chain — the token family a masterwork/material/magic-enhancement
/// modifier uses to state its armor/shield-stat contribution, distinct
/// from (and never present alongside a real value in) the bare
/// `MAXDEX:`/`SPELLFAILURE:`/`ACCHECK:` tokens a BASE armor/shield record
/// carries instead. Only ever consulted by [`compute_arms_armor_effect`]
/// as a fallback when the bare token is absent, so a base record's own
/// real token (and its conditional "Broken" `EQMARMOR` chain, which only
/// accompanies a real bare token) always wins first. `qualifiers[2]` is a
/// literal signed integer for every real corpus record this fallback
/// exists for; a non-numeric value (none observed in the pinned oracle)
/// yields `None` rather than a fabricated number.
fn eqmarmor_chain_value(record: &EquipmentRecord, field: &str) -> Option<i16> {
    record.bonus_chains.iter().find_map(|bonus| {
        let qualifiers = &bonus.qualifiers;
        if qualifiers.len() >= 3 && qualifiers[0] == "EQMARMOR" && qualifiers[1] == field {
            qualifiers[2].parse::<i16>().ok()
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcgen_import::lst_parser::equipment::parse_equipment_entries;

    /// Real verbatim tokens copied from `KEY:Leather Armor (Base)` in
    /// `core_rulebook/cr_equip_arms_armor.lst`.
    #[test]
    fn leather_armor_base_yields_real_armor_stats() {
        let text = "Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tACCHECK:0\tMAXDEX:6\tSPELLFAILURE:10\tBONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0\tBONUS:COMBAT|AC|-1|TYPE=Armor|PRETYPE:1,EQMOD=Special Quality ~ Broken ~ Armor\n";
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_arms_armor_effect(record);
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

        let effect = compute_arms_armor_effect(record);
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

        let effect = compute_arms_armor_effect(record);
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

        let effect = compute_arms_armor_effect(record);
        assert_eq!(effect.armor_class_bonus, Some(4));
    }

    /// Real verbatim tokens copied from `KEY:Longsword (Base)` — a
    /// weapon carries none of the armor-defining tokens at all.
    #[test]
    fn longsword_base_has_no_armor_stats() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n";
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", text);
        let record = &result.entries[0];

        let effect = compute_arms_armor_effect(record);
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

        let effect = compute_arms_armor_effect(record);
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

        let effect = compute_arms_armor_effect(record);
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

        let effect = compute_arms_armor_effect(record);
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

        let effect = compute_arms_armor_effect(record);
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

        let mut effect = compute_arms_armor_effect(base_record);
        assert_eq!(effect.armor_class_bonus, Some(6), "the base item's own chain alone is Breastplate's base value");

        apply_eqmod_armor_class_bonus(&mut effect, &[modifier_record, spikes_record]);
        assert_eq!(
            effect.armor_class_bonus,
            Some(7),
            "the EQMOD-referenced +1 Armor modifier's own separate chain must sum in; Spikes contributes 0"
        );
    }
}
