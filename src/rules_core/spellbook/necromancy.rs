//! SD-20 Epic 2 — Necromancy per-school contribution function.
//!
//! Seventh PF1 spell school landed per `scope-draft.md` §1.2 Step 2's cycle
//! order (abjuration landed in `3147b28`, conjuration landed in `4f53724`,
//! divination landed in `a7568a5`, enchantment landed in `9a9b359`,
//! evocation landed in `4bcfceb`, illusion landed in `d5f1926`; this cycle
//! lands necromancy; transmutation, universal remain).
//!
//! Reads spell level and effect text from the canonical CRB spell-list
//! table store (`rules_tables::crb::spell_list::SPELL_LIST`, SD-19's
//! foundation slice; 62 real Necromancy records) via a `TableCellRef`
//! -style lookup — never hand-rolled or re-derived. This mirrors
//! `spell_resolver::spell_id_resolve`'s own `TableCellRef` construction
//! (`table: "spell_list"`, `row_key: <spell name>`) so a resolved
//! Necromancy effect's provenance is identical in shape to the
//! reachability-only `TableCellRef` SD-19 already produces, and identical
//! in shape to `spellbook::abjuration`'s, `spellbook::conjuration`'s,
//! `spellbook::divination`'s, `spellbook::enchantment`'s,
//! `spellbook::evocation`'s, and `spellbook::illusion`'s own
//! `TableCellRef`.

use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::rules_tables::acg::spell_list::{
    Pf1SchoolId as AcgPf1SchoolId, SPELL_LIST as ACG_SPELL_LIST,
};
use crate::rules_core::rules_tables::apg::spell_list::{
    Pf1SchoolId as ApgPf1SchoolId, SPELL_LIST as APG_SPELL_LIST,
};
use crate::rules_core::rules_tables::crb::spell_list::{Pf1SchoolId, SPELL_LIST};

/// One resolved Necromancy spell's effect: its level and effect text, both
/// read directly from `SPELL_LIST`, plus a `TableCellRef` proving the
/// provenance.
#[derive(Debug, Clone, PartialEq)]
pub struct NecromancySpellEffect {
    pub spell_id: String,
    pub level: u8,
    pub effect_text: String,
    pub table_cell: TableCellRef,
}

/// Resolves `spell_id` against the canonical CRB spell-list table store,
/// returning its Necromancy-school effect record. Returns `None` when
/// `spell_id` is not a real Necromancy record in the table store — SD-19
/// owns the table store; this function reads it, it never fabricates an
/// entry for a KEY the table store doesn't have.
pub fn resolve_necromancy_spell_effect(spell_id: &str) -> Option<NecromancySpellEffect> {
    if let Some(entry) =
        SPELL_LIST.iter().find(|entry| entry.key == spell_id && entry.school == Pf1SchoolId::Necromancy)
    {
        return Some(NecromancySpellEffect {
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
    // a different seam.
    if let Some(entry) = APG_SPELL_LIST
        .iter()
        .find(|entry| entry.key == spell_id && entry.school == Some(ApgPf1SchoolId::Necromancy))
    {
        return Some(NecromancySpellEffect {
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
        .find(|entry| entry.key == spell_id && entry.school == AcgPf1SchoolId::Necromancy)
    {
        return Some(NecromancySpellEffect {
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
    fn resolves_a_real_necromancy_spell_with_level_and_effect_text_from_the_table_store() {
        let effect = resolve_necromancy_spell_effect("Chill Touch")
            .expect("Chill Touch is a real Necromancy record in SPELL_LIST");
        assert_eq!(effect.level, 1);
        assert!(
            effect.effect_text.contains("1d6"),
            "effect text must come from SPELL_LIST's own description, not be hand-rolled: {}",
            effect.effect_text
        );
        assert_eq!(effect.table_cell.table, "spell_list");
        assert_eq!(effect.table_cell.row_key, "Chill Touch");
        assert_eq!(effect.table_cell.rule_set, RuleSetId::Crb);
    }

    #[test]
    fn returns_none_for_a_non_necromancy_spell() {
        // "Charm Person" is a real SPELL_LIST record but Enchantment, not
        // Necromancy -- this school's contribution function must not claim
        // a spell that belongs to a different school.
        assert!(resolve_necromancy_spell_effect("Charm Person").is_none());
    }

    #[test]
    fn returns_none_for_an_unknown_spell_id() {
        assert!(resolve_necromancy_spell_effect("Not A Real Spell").is_none());
    }

    /// W21-SPELL-001: "Unliving Rage" is a real `acg::spell_list::SPELL_LIST`
    /// Necromancy record, absent from CRB's own table
    /// (`advanced_class_guide:spell:unliving_rage`, `wiring_class: computed`,
    /// stuck at `ingested-magnitude` before this widening — see
    /// `spellbook::transmutation`'s identical fix for the full
    /// explanation).
    #[test]
    fn resolves_a_real_acg_necromancy_spell_the_crb_table_does_not_carry() {
        let effect = resolve_necromancy_spell_effect("Unliving Rage")
            .expect("Unliving Rage is a real ACG Necromancy record, not in CRB's own table");
        assert_eq!(effect.level, 2);
        assert!(
            effect.effect_text.contains("undead creatures"),
            "effect text must come from ACG's own SPELL_LIST description: {}",
            effect.effect_text
        );
        assert_eq!(effect.table_cell.rule_set, RuleSetId::Acg);
    }

    /// The CRB table still wins on a name collision -- never shadowed by
    /// the wider search.
    #[test]
    fn crb_resolution_takes_priority_over_the_wider_search() {
        let effect = resolve_necromancy_spell_effect("Chill Touch")
            .expect("Chill Touch is a real CRB Necromancy record");
        assert_eq!(effect.table_cell.rule_set, RuleSetId::Crb);
    }
}
