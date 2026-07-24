//! SD-19 equipment-id resolver.
//!
//! Resolves a `CharacterInput.equipment_selections[].item_id` to its real
//! PCGen corpus record and (when available) the foundation slice's
//! canonical Paizo-table-cell reference.
//!
//! Lookup rule: (1) exact match against the record's verbatim `KEY:`
//! token, (2) exact match against the record's own unnormalized `name`
//! (needed for KEY-less records whose distinguishing content lives
//! inside parentheses, e.g. "Improvised Weapon (1d2)" vs "(1d3)" —
//! normalizing those away would collapse genuinely distinct items into
//! one), (3) a normalized match on the record's `name` (lowercase,
//! spaces -> underscores, parenthesized qualifiers like `"(Base)"`
//! stripped) as the last-resort fallback for the legacy
//! `"item:longsword"`-style fixture namespace, which predates
//! corpus-linkage and was never the corpus's own exact name.

use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;
use crate::pcgen_import::source_content_payload::SourceContentPayload;
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::equipment_tables::equipment_tables;
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
    equipment_tables()
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
        if equip.name == needle || equip.name == item_id {
            let key = equipment_key_token(equip).unwrap_or(&equip.name);
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

/// v0.6 alpha swarm (money-purchase coupling, risks-and-open-questions.md
/// item 9): resolves an `item_id` to its `cost_gp`, with NO corpus access
/// at all -- unlike `equipment_id_resolve` above (which needs a real
/// `SourcePackageContent`), this only needs the flat cost figure, and
/// `equipment_tables()` (`rules_tables::crb::equipment_tables`) already
/// carries `cost_gp` on a `pub const`/`OnceLock`-cached table compiled
/// directly into the binary -- generated from the corpus at build time,
/// verified to mirror the same `KEY:`/`name` identity `equipment_id_resolve`
/// discovers (e.g. `"item:longsword"` -> `key: "Longsword (Base)"`,
/// `name: "Longsword"`, `cost_gp: Some(15.0)` on both paths). This is NOT
/// the same headless-vs-corpus-aware architecture wall that blocked
/// AC-widening earlier this swarm (real per-item AC deltas only exist via
/// a corpus-resolved `EquipmentRecord`; cost does not have that problem) --
/// checked, not assumed, before writing this function.
///
/// Mirrors `equipment_id_resolve`'s exact three-tier match (key, then
/// unnormalized name, then normalized name) against the static table
/// instead of corpus records, so behavior is identical for any item_id
/// both resolvers can find. Returns `None` when no entry matches OR the
/// matched entry's `cost_gp` is itself `None` (a genuine corpus absence --
/// a `(Base)` template record or a formula-priced equipment modifier, per
/// `EquipmentTableEntry.cost_gp`'s own doc comment) -- callers must treat
/// both cases identically (an unaffordable-to-verify purchase, not a free
/// one).
pub fn equipment_cost_gp_headless_resolve(item_id: &str) -> Option<f64> {
    let needle = item_id.strip_prefix("item:").unwrap_or(item_id);
    let normalized_needle = normalize_equipment_name(needle);
    let table = equipment_tables();

    for entry in table {
        if entry.key == needle || entry.key == item_id {
            return entry.cost_gp;
        }
    }

    for entry in table {
        if entry.name == needle || entry.name == item_id {
            return entry.cost_gp;
        }
    }

    for entry in table {
        if normalize_equipment_name(entry.name) == normalized_needle {
            return entry.cost_gp;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcgen_import::ir_converter::convert_equipment_record;
    use crate::pcgen_import::lst_parser::equipment::parse_equipment_entries;
    use crate::rules_core::source_content::SourceRef;

    fn corpus_from(text: &str) -> SourcePackageContent<'static> {
        let result = parse_equipment_entries("test.lst", text);
        let source_ref = SourceRef {
            lst_file: "test.lst".to_string(),
            line: 1,
        };
        let mut corpus = SourcePackageContent::empty("test", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }

    /// Regression test: KEY-less records whose only distinguishing
    /// content is inside parentheses (e.g. the real corpus's
    /// "Improvised Weapon (1d2)" through "(2d10)" damage-die variants)
    /// must resolve to themselves exactly, not to whichever sibling the
    /// lossy normalized-name fallback happens to hit first.
    #[test]
    fn key_less_records_distinguished_only_by_parenthesized_content_resolve_exactly() {
        let text = "\
Improvised Weapon (1d2)\tTYPE:Weapon.Melee.Improvised\tCOST:0\tWT:1
Improvised Weapon (1d3)\tTYPE:Weapon.Melee.Improvised\tCOST:0\tWT:1
Improvised Weapon (1d4)\tTYPE:Weapon.Melee.Improvised\tCOST:0\tWT:2
";
        let corpus = corpus_from(text);

        let (record, _) = equipment_id_resolve("Improvised Weapon (1d3)", RuleSetId::Crb, &corpus)
            .expect("expected 'Improvised Weapon (1d3)' to resolve");
        assert_eq!(record.name, "Improvised Weapon (1d3)");

        let (record, _) = equipment_id_resolve("Improvised Weapon (1d2)", RuleSetId::Crb, &corpus)
            .expect("expected 'Improvised Weapon (1d2)' to resolve");
        assert_eq!(record.name, "Improvised Weapon (1d2)");
    }

    /// Control: the legacy `"item:longsword"`-style fixture namespace
    /// must still resolve via the normalized-name fallback, since it
    /// predates corpus-linkage and never matches the corpus's exact name.
    #[test]
    fn legacy_item_prefix_fixture_namespace_still_resolves_via_normalized_fallback() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\n";
        let corpus = corpus_from(text);

        let (record, _) = equipment_id_resolve("item:longsword", RuleSetId::Crb, &corpus)
            .expect("expected 'item:longsword' to resolve via the normalized fallback");
        assert_eq!(record.name, "Longsword");
    }

    #[test]
    fn equipment_cost_gp_headless_resolve_finds_a_real_item_by_the_legacy_item_prefix() {
        assert_eq!(equipment_cost_gp_headless_resolve("item:longsword"), Some(15.0));
    }

    #[test]
    fn equipment_cost_gp_headless_resolve_finds_a_real_item_by_its_exact_corpus_key() {
        assert_eq!(
            equipment_cost_gp_headless_resolve("Longsword (Base)"),
            Some(15.0)
        );
    }

    #[test]
    fn equipment_cost_gp_headless_resolve_returns_none_for_an_unknown_item() {
        assert_eq!(
            equipment_cost_gp_headless_resolve("item:not_a_real_item_at_all"),
            None
        );
    }
}
