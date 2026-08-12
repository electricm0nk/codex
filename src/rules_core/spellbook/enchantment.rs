//! SD-20 Epic 2 — Enchantment per-school contribution function.
//!
//! Fourth PF1 spell school landed per `scope-draft.md` §1.2 Step 2's cycle
//! order (abjuration landed in `3147b28`, conjuration landed in `4f53724`,
//! divination landed in `a7568a5`; this cycle lands enchantment;
//! evocation, illusion, necromancy, transmutation, universal remain).
//!
//! Reads spell level and effect text from the canonical CRB spell-list
//! table store (`rules_tables::crb::spell_list::SPELL_LIST`, SD-19's
//! foundation slice; 60 real Enchantment records) via a `TableCellRef`
//! -style lookup — never hand-rolled or re-derived. This mirrors
//! `spell_resolver::spell_id_resolve`'s own `TableCellRef` construction
//! (`table: "spell_list"`, `row_key: <spell name>`) so a resolved
//! Enchantment effect's provenance is identical in shape to the
//! reachability-only `TableCellRef` SD-19 already produces, and identical
//! in shape to `spellbook::abjuration`'s, `spellbook::conjuration`'s, and
//! `spellbook::divination`'s own `TableCellRef`.

use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::rules_tables::crb::spell_list::{Pf1SchoolId, SPELL_LIST};

/// One resolved Enchantment spell's effect: its level and effect text,
/// both read directly from `SPELL_LIST`, plus a `TableCellRef` proving
/// the provenance.
#[derive(Debug, Clone, PartialEq)]
pub struct EnchantmentSpellEffect {
    pub spell_id: String,
    pub level: u8,
    pub effect_text: String,
    pub table_cell: TableCellRef,
}

/// Resolves `spell_id` against the canonical CRB spell-list table store,
/// returning its Enchantment-school effect record. Returns `None` when
/// `spell_id` is not a real Enchantment record in the table store — SD-19
/// owns the table store; this function reads it, it never fabricates an
/// entry for a KEY the table store doesn't have.
pub fn resolve_enchantment_spell_effect(spell_id: &str) -> Option<EnchantmentSpellEffect> {
    let entry = SPELL_LIST
        .iter()
        .find(|entry| entry.key == spell_id && entry.school == Pf1SchoolId::Enchantment)?;
    Some(EnchantmentSpellEffect {
        spell_id: spell_id.to_string(),
        level: entry.level,
        effect_text: entry.description.to_string(),
        table_cell: TableCellRef {
            rule_set: RuleSetId::Crb,
            table: "spell_list".to_string(),
            row_key: spell_id.to_string(),
            column_key: String::new(),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_a_real_enchantment_spell_with_level_and_effect_text_from_the_table_store() {
        let effect = resolve_enchantment_spell_effect("Charm Person")
            .expect("Charm Person is a real Enchantment record in SPELL_LIST");
        assert_eq!(effect.level, 1);
        assert!(
            effect.effect_text.contains("trusted friend"),
            "effect text must come from SPELL_LIST's own description, not be hand-rolled: {}",
            effect.effect_text
        );
        assert_eq!(effect.table_cell.table, "spell_list");
        assert_eq!(effect.table_cell.row_key, "Charm Person");
        assert_eq!(effect.table_cell.rule_set, RuleSetId::Crb);
    }

    #[test]
    fn returns_none_for_a_non_enchantment_spell() {
        // "Mage Armor" is a real SPELL_LIST record but Conjuration, not
        // Enchantment -- this school's contribution function must not
        // claim a spell that belongs to a different school.
        assert!(resolve_enchantment_spell_effect("Mage Armor").is_none());
    }

    #[test]
    fn returns_none_for_an_unknown_spell_id() {
        assert!(resolve_enchantment_spell_effect("Not A Real Spell").is_none());
    }
}
