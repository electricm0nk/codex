//! SD-20 Epic 2 — Universal per-school contribution function.
//!
//! Ninth and FINAL PF1 spell school landed per `scope-draft.md` §1.2 Step
//! 2's cycle order (abjuration landed in `3147b28`, conjuration landed in
//! `4f53724`, divination landed in `a7568a5`, enchantment landed in
//! `9a9b359`, evocation landed in `4bcfceb`, illusion landed in `d5f1926`,
//! necromancy landed in `396ebd4`, transmutation landed in `d1d0952`; this
//! cycle lands universal, CLOSING Epic 2 -- all nine PF1 spell schools
//! done).
//!
//! Reads spell level and effect text from the canonical CRB spell-list
//! table store (`rules_tables::crb::spell_list::SPELL_LIST`, SD-19's
//! foundation slice; 5 real Universal records) via a `TableCellRef`-style
//! lookup — never hand-rolled or re-derived. This mirrors
//! `spell_resolver::spell_id_resolve`'s own `TableCellRef` construction
//! (`table: "spell_list"`, `row_key: <spell name>`) so a resolved
//! Universal effect's provenance is identical in shape to the
//! reachability-only `TableCellRef` SD-19 already produces, and identical
//! in shape to `spellbook::abjuration`'s, `spellbook::conjuration`'s,
//! `spellbook::divination`'s, `spellbook::enchantment`'s,
//! `spellbook::evocation`'s, `spellbook::illusion`'s,
//! `spellbook::necromancy`'s, and `spellbook::transmutation`'s own
//! `TableCellRef`.
//!
//! `technical-design.md` §2.0's `RulesTables` parameter type was retired
//! (no `RulesTables` type exists in this repo) — this module reads the
//! table store directly (`rules_tables::crb::spell_list::SPELL_LIST`),
//! matching how every other landed school's contribution function, plus
//! `spell_resolver.rs` and `equipment_resolver.rs`, already read it.

use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::rules_tables::crb::spell_list::{Pf1SchoolId, SPELL_LIST};

/// One resolved Universal spell's effect: its level and effect text, both
/// read directly from `SPELL_LIST`, plus a `TableCellRef` proving the
/// provenance.
#[derive(Debug, Clone, PartialEq)]
pub struct UniversalSpellEffect {
    pub spell_id: String,
    pub level: u8,
    pub effect_text: String,
    pub table_cell: TableCellRef,
}

/// Resolves `spell_id` against the canonical CRB spell-list table store,
/// returning its Universal-school effect record. Returns `None` when
/// `spell_id` is not a real Universal record in the table store — SD-19
/// owns the table store; this function reads it, it never fabricates an
/// entry for a KEY the table store doesn't have.
pub fn resolve_universal_spell_effect(spell_id: &str) -> Option<UniversalSpellEffect> {
    let entry = SPELL_LIST
        .iter()
        .find(|entry| entry.key == spell_id && entry.school == Pf1SchoolId::Universal)?;
    Some(UniversalSpellEffect {
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
    fn resolves_a_real_universal_spell_with_level_and_effect_text_from_the_table_store() {
        let effect = resolve_universal_spell_effect("Permanency")
            .expect("Permanency is a real Universal record in SPELL_LIST");
        assert_eq!(effect.level, 5);
        assert!(
            effect.effect_text.contains("permanent"),
            "effect text must come from SPELL_LIST's own description, not be hand-rolled: {}",
            effect.effect_text
        );
        assert_eq!(effect.table_cell.table, "spell_list");
        assert_eq!(effect.table_cell.row_key, "Permanency");
        assert_eq!(effect.table_cell.rule_set, RuleSetId::Crb);
    }

    #[test]
    fn returns_none_for_a_non_universal_spell() {
        // "Charm Person" is a real SPELL_LIST record but Enchantment, not
        // Universal -- this school's contribution function must not claim
        // a spell that belongs to a different school.
        assert!(resolve_universal_spell_effect("Charm Person").is_none());
    }

    #[test]
    fn returns_none_for_an_unknown_spell_id() {
        assert!(resolve_universal_spell_effect("Not A Real Spell").is_none());
    }
}
