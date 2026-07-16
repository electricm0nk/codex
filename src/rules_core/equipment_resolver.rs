//! SD-19 equipment-id resolver.
//!
//! Resolves a `CharacterInput.equipment_selections[].item_id` to its real
//! PCGen corpus record and (when available) the foundation slice's
//! canonical Paizo-table-cell reference.
//!
//! Lookup rule: exact match against the record's verbatim `KEY:` token
//! first (handles the existing `"item:longsword"`-style fixture
//! namespace once the `"item:"` prefix is stripped), then a normalized
//! match on the record's `name` (lowercase, spaces -> underscores,
//! parenthesized qualifiers like `"(Base)"` stripped) — the corpus's own
//! `KEY:` tokens are not always snake_case, so the normalized-name path
//! is the fallback, not the primary.

use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;
use crate::pcgen_import::source_content_payload::SourceContentPayload;
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::equipment_tables::EQUIPMENT_TABLES;
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::source_content::{SourceContentKind, SourcePackageContent};

/// The record's `KEY:` token, if the corpus line carried one. PCGen
/// convention: absent means the record's `name` field is its own key.
pub fn equipment_key_token(record: &EquipmentRecord) -> Option<&str> {
    record
        .tokens
        .iter()
        .find(|token| token.key == "KEY")
        .map(|token| token.value.as_str())
}

fn normalize_equipment_name(name: &str) -> String {
    let stripped = match name.find('(') {
        Some(idx) => name[..idx].trim(),
        None => name.trim(),
    };
    stripped.to_lowercase().replace(' ', "_")
}

fn table_cell_for(rule_set: RuleSetId, key: &str) -> Option<TableCellRef> {
    EQUIPMENT_TABLES
        .iter()
        .find(|entry| entry.key == key)
        .map(|_| TableCellRef {
            rule_set,
            table: "equipment_tables".to_string(),
            row_key: key.to_string(),
            column_key: String::new(),
        })
}

pub fn equipment_id_resolve<'a>(
    item_id: &str,
    rule_set: RuleSetId,
    corpus: &SourcePackageContent<'a>,
) -> Option<(&'a EquipmentRecord, Option<TableCellRef>)> {
    let needle = item_id.strip_prefix("item:").unwrap_or(item_id);
    let normalized_needle = normalize_equipment_name(needle);

    let records: Vec<&'a EquipmentRecord> = corpus
        .records_by_kind(SourceContentKind::Equipment)
        .into_iter()
        .filter_map(|record| match record.payload {
            SourceContentPayload::Equipment(equip) => Some(equip),
            _ => None,
        })
        .collect();

    for equip in &records {
        if let Some(key) = equipment_key_token(equip)
            && (key == needle || key == item_id)
        {
            return Some((equip, table_cell_for(rule_set, key)));
        }
    }

    for equip in &records {
        if normalize_equipment_name(&equip.name) == normalized_needle {
            let key = equipment_key_token(equip).unwrap_or(&equip.name);
            return Some((equip, table_cell_for(rule_set, key)));
        }
    }

    None
}
