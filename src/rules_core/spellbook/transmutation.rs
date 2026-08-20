//! SD-20 Epic 2 — Transmutation per-school contribution function.
//!
//! Eighth PF1 spell school landed per `scope-draft.md` §1.2 Step 2's cycle
//! order (abjuration landed in `3147b28`, conjuration landed in `4f53724`,
//! divination landed in `a7568a5`, enchantment landed in `9a9b359`,
//! evocation landed in `4bcfceb`, illusion landed in `d5f1926`, necromancy
//! landed in `396ebd4`; this cycle lands transmutation; universal remains
//! -- Epic 2's ninth and final school).
//!
//! Reads spell level and effect text from the canonical CRB spell-list
//! table store (`rules_tables::crb::spell_list::SPELL_LIST`, SD-19's
//! foundation slice; 152 real Transmutation records) via a `TableCellRef`
//! -style lookup — never hand-rolled or re-derived. This mirrors
//! `spell_resolver::spell_id_resolve`'s own `TableCellRef` construction
//! (`table: "spell_list"`, `row_key: <spell name>`) so a resolved
//! Transmutation effect's provenance is identical in shape to the
//! reachability-only `TableCellRef` SD-19 already produces, and identical
//! in shape to `spellbook::abjuration`'s, `spellbook::conjuration`'s,
//! `spellbook::divination`'s, `spellbook::enchantment`'s,
//! `spellbook::evocation`'s, `spellbook::illusion`'s, and
//! `spellbook::necromancy`'s own `TableCellRef`.

use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::rules_tables::acg::spell_list::{
    Pf1SchoolId as AcgPf1SchoolId, SPELL_LIST as ACG_SPELL_LIST,
};
use crate::rules_core::rules_tables::apg::spell_list::{
    Pf1SchoolId as ApgPf1SchoolId, SPELL_LIST as APG_SPELL_LIST,
};
use crate::rules_core::rules_tables::crb::spell_list::{Pf1SchoolId, SPELL_LIST};

/// One resolved Transmutation spell's effect: its level and effect text,
/// both read directly from `SPELL_LIST`, plus a `TableCellRef` proving the
/// provenance.
#[derive(Debug, Clone, PartialEq)]
pub struct TransmutationSpellEffect {
    pub spell_id: String,
    pub level: u8,
    pub effect_text: String,
    pub table_cell: TableCellRef,
}

/// Resolves `spell_id` against the canonical CRB spell-list table store,
/// returning its Transmutation-school effect record. Returns `None` when
/// `spell_id` is not a real Transmutation record in the table store —
/// SD-19 owns the table store; this function reads it, it never
/// fabricates an entry for a KEY the table store doesn't have.
pub fn resolve_transmutation_spell_effect(spell_id: &str) -> Option<TransmutationSpellEffect> {
    if let Some(entry) =
        SPELL_LIST.iter().find(|entry| entry.key == spell_id && entry.school == Pf1SchoolId::Transmutation)
    {
        return Some(TransmutationSpellEffect {
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
        .find(|entry| entry.key == spell_id && entry.school == Some(ApgPf1SchoolId::Transmutation))
    {
        return Some(TransmutationSpellEffect {
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
        .find(|entry| entry.key == spell_id && entry.school == AcgPf1SchoolId::Transmutation)
    {
        return Some(TransmutationSpellEffect {
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
    fn resolves_a_real_transmutation_spell_with_level_and_effect_text_from_the_table_store() {
        let effect = resolve_transmutation_spell_effect("Enlarge Person")
            .expect("Enlarge Person is a real Transmutation record in SPELL_LIST");
        assert_eq!(effect.level, 1);
        assert!(
            effect.effect_text.contains("doubling its height"),
            "effect text must come from SPELL_LIST's own description, not be hand-rolled: {}",
            effect.effect_text
        );
        assert_eq!(effect.table_cell.table, "spell_list");
        assert_eq!(effect.table_cell.row_key, "Enlarge Person");
        assert_eq!(effect.table_cell.rule_set, RuleSetId::Crb);
    }

    #[test]
    fn returns_none_for_a_non_transmutation_spell() {
        // "Charm Person" is a real SPELL_LIST record but Enchantment, not
        // Transmutation -- this school's contribution function must not
        // claim a spell that belongs to a different school.
        assert!(resolve_transmutation_spell_effect("Charm Person").is_none());
    }

    #[test]
    fn returns_none_for_an_unknown_spell_id() {
        assert!(resolve_transmutation_spell_effect("Not A Real Spell").is_none());
    }

    /// W21-SPELL-001: this function used to read ONLY the CRB table store,
    /// even though `rules_tables::crb::wizard_spell_list`'s own per-class
    /// level table already spans CRB + APG + ACG (580 records, per that
    /// module's doc comment) — so a Wizard casting an ACG spell like this
    /// one resolved a level (via `wizard_spell_level`) but then found no
    /// `SpellEffect` here, producing `SpellProbeOutcome::NoTableEffect`
    /// forever regardless of how complete the ACG data was. "Air Step" is
    /// a real `acg::spell_list::SPELL_LIST` Transmutation record
    /// (`work-inventory.json`'s `advanced_class_guide:spell:air_step`,
    /// `wiring_class: computed`, stuck at `ingested-magnitude`).
    #[test]
    fn resolves_a_real_acg_transmutation_spell_the_crb_table_does_not_carry() {
        let effect = resolve_transmutation_spell_effect("Air Step")
            .expect("Air Step is a real ACG Transmutation record, not in CRB's own table");
        assert_eq!(effect.level, 2);
        assert!(
            effect.effect_text.contains("air walk"),
            "effect text must come from ACG's own SPELL_LIST description: {}",
            effect.effect_text
        );
        assert_eq!(effect.table_cell.rule_set, RuleSetId::Acg);
    }

    /// Same shape, the APG side: "Ant Haul" is a real
    /// `apg::spell_list::SPELL_LIST` Transmutation record
    /// (`advanced_players_guide:spell:ant_haul`, `wiring_class: computed`,
    /// stuck at `ingested-magnitude` for the identical reason).
    #[test]
    fn resolves_a_real_apg_transmutation_spell_the_crb_table_does_not_carry() {
        let effect = resolve_transmutation_spell_effect("Ant Haul")
            .expect("Ant Haul is a real APG Transmutation record, not in CRB's own table");
        assert_eq!(effect.level, 1);
        assert!(
            effect.effect_text.contains("carrying capacity"),
            "effect text must come from APG's own SPELL_LIST description: {}",
            effect.effect_text
        );
        assert_eq!(effect.table_cell.rule_set, RuleSetId::Apg);
    }

    /// The CRB table still wins on a name collision -- never shadowed by
    /// the wider search. No real corpus collision is known to exist for
    /// Transmutation, so this proves the PRIORITY ORDER itself rather than
    /// a real corpus fact: if CRB's table resolves at all, the wider
    /// search must never be consulted for that name.
    #[test]
    fn crb_resolution_takes_priority_over_the_wider_search() {
        let effect = resolve_transmutation_spell_effect("Enlarge Person")
            .expect("Enlarge Person is a real CRB Transmutation record");
        assert_eq!(effect.table_cell.rule_set, RuleSetId::Crb);
    }
}
