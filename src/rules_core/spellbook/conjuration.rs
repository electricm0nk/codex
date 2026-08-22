//! SD-20 Epic 2 — Conjuration per-school contribution function.
//!
//! Second PF1 spell school landed per `scope-draft.md` §1.2 Step 2's cycle
//! order (abjuration landed first in `3147b28`; conjuration, then
//! divination, enchantment, evocation, illusion, necromancy,
//! transmutation, universal remain).
//!
//! Reads spell level and effect text from the canonical CRB spell-list
//! table store (`rules_tables::crb::spell_list::SPELL_LIST`, SD-19's
//! foundation slice; 116 real Conjuration records) via a `TableCellRef`
//! -style lookup — never hand-rolled or re-derived. This mirrors
//! `spell_resolver::spell_id_resolve`'s own `TableCellRef` construction
//! (`table: "spell_list"`, `row_key: <spell name>`) so a resolved
//! Conjuration effect's provenance is identical in shape to the
//! reachability-only `TableCellRef` SD-19 already produces, and identical
//! in shape to `spellbook::abjuration`'s own `TableCellRef`.

use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::rules_tables::acg::spell_list::{
    Pf1SchoolId as AcgPf1SchoolId, SPELL_LIST as ACG_SPELL_LIST,
};
use crate::rules_core::rules_tables::apg::spell_list::{
    Pf1SchoolId as ApgPf1SchoolId, SPELL_LIST as APG_SPELL_LIST,
};
use crate::rules_core::rules_tables::crb::spell_list::{Pf1SchoolId, SPELL_LIST};

/// One resolved Conjuration spell's effect: its level and effect text,
/// both read directly from `SPELL_LIST`, plus a `TableCellRef` proving
/// the provenance.
#[derive(Debug, Clone, PartialEq)]
pub struct ConjurationSpellEffect {
    pub spell_id: String,
    pub level: u8,
    pub effect_text: String,
    pub table_cell: TableCellRef,
}

/// Resolves `spell_id` against the canonical CRB spell-list table store,
/// returning its Conjuration-school effect record. Returns `None` when
/// `spell_id` is not a real Conjuration record in the table store — SD-19
/// owns the table store; this function reads it, it never fabricates an
/// entry for a KEY the table store doesn't have.
pub fn resolve_conjuration_spell_effect(spell_id: &str) -> Option<ConjurationSpellEffect> {
    if let Some(entry) =
        SPELL_LIST.iter().find(|entry| entry.key == spell_id && entry.school == Pf1SchoolId::Conjuration)
    {
        return Some(ConjurationSpellEffect {
            spell_id: spell_id.to_string(),
            level: entry.level,
            effect_text: entry.description.to_string(),
            table_cell: TableCellRef {
                rule_set: RuleSetId::Crb,
                table: "spell_list".to_string(),
                row_key: spell_id.to_string(),
                column_key: String::new(),
            },
        });
    }
    // W21-SPELL-001: CRB's table has no record of this key -- but the
    // per-class level tables that route spells here (e.g.
    // `crb::wizard_spell_list::wizard_spell_level`) already span CRB + APG
    // + ACG. Widen the search to those same two books rather than leaving
    // an already-resolved level with no `SpellEffect` to attach to,
    // exactly the `duration`/`range` book-roster gap this program has
    // already found twice (`OPEN-ISSUES.md` rows 324/325) -- same shape,
    // a different seam. No known `advanced_class_guide`/
    // `advanced_players_guide` `wiring_class: computed` Conjuration unit
    // currently rides on this (unlike the other 7 schools this cycle
    // widened), but leaving Conjuration on the old CRB-only lookup would
    // recreate the identical, now-diagnosed landmine for the next book
    // widening or the next Conjuration-school ACG/APG record this
    // classifier marks `computed`.
    if let Some(entry) = APG_SPELL_LIST
        .iter()
        .find(|entry| entry.key == spell_id && entry.school == Some(ApgPf1SchoolId::Conjuration))
    {
        return Some(ConjurationSpellEffect {
            spell_id: spell_id.to_string(),
            level: entry.level?,
            effect_text: entry.description?.to_string(),
            table_cell: TableCellRef {
                rule_set: RuleSetId::Apg,
                table: "spell_list".to_string(),
                row_key: spell_id.to_string(),
                column_key: String::new(),
            },
        });
    }
    if let Some(entry) = ACG_SPELL_LIST
        .iter()
        .find(|entry| entry.key == spell_id && entry.school == AcgPf1SchoolId::Conjuration)
    {
        return Some(ConjurationSpellEffect {
            spell_id: spell_id.to_string(),
            level: entry.level,
            effect_text: entry.description.to_string(),
            table_cell: TableCellRef {
                rule_set: RuleSetId::Acg,
                table: "spell_list".to_string(),
                row_key: spell_id.to_string(),
                column_key: String::new(),
            },
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_a_real_conjuration_spell_with_level_and_effect_text_from_the_table_store() {
        let effect = resolve_conjuration_spell_effect("Mage Armor")
            .expect("Mage Armor is a real Conjuration record in SPELL_LIST");
        assert_eq!(effect.level, 1);
        assert!(
            effect.effect_text.contains("armor bonus"),
            "effect text must come from SPELL_LIST's own description, not be hand-rolled: {}",
            effect.effect_text
        );
        assert_eq!(effect.table_cell.table, "spell_list");
        assert_eq!(effect.table_cell.row_key, "Mage Armor");
        assert_eq!(effect.table_cell.rule_set, RuleSetId::Crb);
    }

    #[test]
    fn returns_none_for_a_non_conjuration_spell() {
        // "Shield" is a real SPELL_LIST record but Abjuration, not
        // Conjuration -- this school's contribution function must not
        // claim a spell that belongs to a different school.
        assert!(resolve_conjuration_spell_effect("Shield").is_none());
    }

    #[test]
    fn returns_none_for_an_unknown_spell_id() {
        assert!(resolve_conjuration_spell_effect("Not A Real Spell").is_none());
    }

    /// W21-SPELL-001: "Adhesive Spittle" is a real
    /// `acg::spell_list::SPELL_LIST` Conjuration record, absent from CRB's
    /// own table -- proves the widening itself even though no currently
    /// `wiring_class: computed` unit rides on this particular school (see
    /// this function's own doc comment, and `spellbook::transmutation`'s
    /// identical fix for the full explanation).
    #[test]
    fn resolves_a_real_acg_conjuration_spell_the_crb_table_does_not_carry() {
        let effect = resolve_conjuration_spell_effect("Adhesive Spittle")
            .expect("Adhesive Spittle is a real ACG Conjuration record, not in CRB's own table");
        assert_eq!(effect.level, 1);
        assert!(
            effect.effect_text.contains("tanglefoot bag"),
            "effect text must come from ACG's own SPELL_LIST description: {}",
            effect.effect_text
        );
        assert_eq!(effect.table_cell.rule_set, RuleSetId::Acg);
    }

    /// Same shape, the APG side: "Aqueous Orb" is a real
    /// `apg::spell_list::SPELL_LIST` Conjuration record.
    #[test]
    fn resolves_a_real_apg_conjuration_spell_the_crb_table_does_not_carry() {
        let effect = resolve_conjuration_spell_effect("Aqueous Orb")
            .expect("Aqueous Orb is a real APG Conjuration record, not in CRB's own table");
        assert_eq!(effect.level, 3);
        assert!(
            effect.effect_text.contains("rolling sphere of churning water"),
            "effect text must come from APG's own SPELL_LIST description: {}",
            effect.effect_text
        );
        assert_eq!(effect.table_cell.rule_set, RuleSetId::Apg);
    }

    /// The CRB table still wins on a name collision -- never shadowed by
    /// the wider search.
    #[test]
    fn crb_resolution_takes_priority_over_the_wider_search() {
        let effect = resolve_conjuration_spell_effect("Mage Armor")
            .expect("Mage Armor is a real CRB Conjuration record");
        assert_eq!(effect.table_cell.rule_set, RuleSetId::Crb);
    }
}
