//! SD-19 spell-id resolver.
//!
//! Resolves a `CharacterInput.spells_selected[].spell_id` to its real
//! PCGen corpus record and (when available) the foundation slice's
//! canonical Paizo-table-cell reference.
//!
//! Note on identity: unlike equipment records, spell records in
//! `cr_spells.lst` carry no `KEY:` token — a spell's identity is its
//! `name` field (confirmed against the real corpus; see
//! `rules_tables::crb::spell_list`'s doc comment). So "spell_id" here
//! means the spell's corpus `name`, matched exactly; no normalization is
//! needed since PF1 spell names are unique across the strict-school
//! partition.

use crate::pcgen_import::lst_parser::spell::LstSpellRecord;
use crate::pcgen_import::source_content_payload::SourceContentPayload;
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::spell_list::SPELL_LIST;
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::source_content::{SourceContentKind, SourcePackageContent};

pub fn spell_id_resolve<'a>(
    spell_id: &str,
    rule_set: RuleSetId,
    corpus: &SourcePackageContent<'a>,
) -> Option<(&'a LstSpellRecord, Option<TableCellRef>)> {
    for record in corpus.records_by_kind(SourceContentKind::Spell) {
        if let SourceContentPayload::Spell(spell) = record.payload
            && spell.name == spell_id
        {
            let table_cell = SPELL_LIST
                .iter()
                .find(|entry| entry.key == spell_id)
                .map(|_| TableCellRef {
                    rule_set,
                    table: "spell_list".to_string(),
                    row_key: spell_id.to_string(),
                    column_key: String::new(),
                });
            return Some((spell, table_cell));
        }
    }
    None
}
