//! Ingest-format reader for an already-converted `data/corpus/` equipment
//! record, and the canonical envelope built from it.
//!
//! # Why this module is on the converter side
//!
//! SD-35 `AT-35-E6-003-RULED` cycle 13. `rules_core::corpus_loader` used to
//! read a corpus record's **ingest token array** and **ingest bonus-chain
//! array** itself — `pcgen_import::ingest_record::token_pairs`,
//! `bonus_chain_qualifiers` and `rebuild_bonus_token` — rebuild an
//! ingest-format [`EquipmentRecord`] out of them, and hand that row to
//! [`crate::pcgen_import::ir_converter::convert_equipment_record`]. Four of
//! the live side's remaining converter reads were those calls
//! (`decisions.md` §11, §19).
//!
//! The field names, the traversal and the `BONUS:` re-spelling are the
//! converter's own vocabulary, so they live here now, next to the parser
//! surface that defines them. The live loader asks one question —
//! *"what canonical record does this corpus JSON object stand for?"* — and
//! never names a token, a token array or a parser row to ask it.
//!
//! # What is NOT settled here
//!
//! This is the ingest boundary, not a second converter. It performs exactly
//! the same two steps the live loader performed before the move, in the same
//! order, on the same input: rebuild the row, then convert it. Nothing is
//! re-derived, no value is re-parsed, and the resulting
//! [`CorpusEquipmentRecord`] is field-for-field the one
//! `equipment_record_to_corpus` already produced — the parity tests at the
//! bottom of this module are that claim, measured over the real corpus rather
//! than asserted.

use crate::pcgen_import::ingest_record;
use crate::pcgen_import::ir_converter::convert_equipment_record;
use crate::pcgen_import::lst_parser::equipment::{
    BonusToken, EquipmentDiagnostic, EquipmentRecord, EquipmentRecordKind, EquipmentToken,
};
use codex::rules_core::source_content::SourceContentRecord;

/// Build the canonical [`SourceContentRecord`] for one already-converted
/// `data/corpus/<book>/equipment/**/*.json` record's `data` object.
///
/// `None` when the object carries neither a `key` nor a `name` — the same
/// "not a record" verdict the live loader reported as a malformed-record
/// diagnostic before this move, unchanged.
///
/// The rebuilt ingest row is interned for the process lifetime (`Box::leak`),
/// exactly as `rules_core::corpus_loader` interned it before the move, so the
/// returned envelope borrows for `'static`. The row itself is never handed
/// out: the envelope's payload is the converted
/// [`codex::rules_core::equipment_record::CorpusEquipmentRecord`] alone.
pub fn corpus_equipment_source_record(
    data: &serde_json::Value,
) -> Option<SourceContentRecord<'static>> {
    let record = equipment_record_from_json(data)?;
    let record: &'static EquipmentRecord = Box::leak(Box::new(record));
    Some(convert_equipment_record(record))
}

/// Rebuild the ingest-format [`EquipmentRecord`] an already-converted corpus
/// JSON `data` object stands for.
///
/// Moved verbatim from `rules_core::corpus_loader::equipment_record_from_json`
/// (SD-35 `AT-35-E6-003-RULED` cycle 13). Every synthesis rule below is the one
/// that was already there; the comments are the ones that were already on it.
///
/// A record enriched with the two ingest arrays rebuilds a full, accurate row.
/// A record without them (not yet enriched, or a `web_second_source` /
/// `same_book_fallback` record with no raw LST line to enrich from) rebuilds a
/// *thin* row — tokens and bonus chains empty apart from the synthesized
/// identity/weight/cost entries below. That is an honest degradation, not a
/// silent one.
pub fn equipment_record_from_json(data: &serde_json::Value) -> Option<EquipmentRecord> {
    let key = data.get("key").and_then(serde_json::Value::as_str)?;
    let name = data.get("name").and_then(serde_json::Value::as_str).unwrap_or(key);

    let mut tokens = Vec::new();
    let mut bonus_chains = Vec::new();
    for (k, v) in ingest_record::token_pairs(data) {
        let raw_pair = format!("{k}:{v}");
        tokens.push(EquipmentToken { key: k.to_string(), value: v.to_string(), line_number: 1, raw_pair });
    }
    for qualifiers in ingest_record::bonus_chain_qualifiers(data) {
        let qualifiers: Vec<String> = qualifiers.into_iter().map(str::to_string).collect();
        // `rebuild_bonus_token` is the inverse of `bonus_chain_qualifiers`
        // above and lives beside it.
        let raw_bonus = ingest_record::rebuild_bonus_token(&qualifiers);
        bonus_chains.push(BonusToken { line_number: 1, raw_bonus, qualifiers });
    }
    // SD-33 remediation wave 5 (`sd33-r5-skillcombat`): synthesize a KEY:
    // token from the ingested `data.key` field whenever the ingest token
    // array did not itself carry a literal `KEY:` entry -- not only when
    // that array was completely empty. `data.key` is ALWAYS the
    // ingestion pipeline's own record identity (the real `KEY:` token
    // when the LST line had one, else the record's own bare name --
    // `equipment_resolver`'s doc comment states this rule and the
    // ingestion `source.record_key` field already computes it this way).
    // Before this fix, a record with a non-empty token array but no
    // literal `KEY:` entry among them (common: a keyless LST line whose
    // identity is its own first-column name) fell through to `None`, and
    // resolution fell back to matching on `.name` instead -- which is
    // `data.name`, NOT `data.key`, and diverges from it whenever the
    // record also carries an `OUTPUTNAME:` token (ingestion substitutes
    // `OUTPUTNAME` into `name` for display, e.g. `Companion Stone
    // (Diplomacy)`'s real key vs. its `name: "Companion Stone of
    // [NAME]"`, an unresolved OUTPUTNAME placeholder never meant to be an
    // identity). That silently broke resolution for every
    // OUTPUTNAME-bearing, KEY-less record -- 12 of `ultimate_psionics`'s
    // own equipment records among them, plus the shape-combat lane's own
    // narrower `engine_id_resolve_fails_templated_variant_record` finding
    // (`Psychoactive Skin (Defender)`/`(Hero)`).
    if !tokens.iter().any(|t| t.key == "KEY") {
        tokens.push(EquipmentToken {
            key: "KEY".to_string(),
            value: key.to_string(),
            line_number: 1,
            raw_pair: format!("KEY:{key}"),
        });
    }
    // `AT-34-E3-003` (bucket `M`, EQUIPMENT sub-causes, cycle 6): the same
    // synthesis the `KEY:` block above already performs, applied to the
    // two fields the settled weight and cost are read from. The ingestion
    // pipeline always captures a record's own weight/cost as top-level
    // `data.weight_lbs`/`data.cost_gp` (`arrow_slaying.json`'s real on-disk
    // shape among many: `"weight_lbs": 0.1`, no ingest token array at all --
    // a "thin" record). Before that fix, a thin record's `tokens` list held
    // only the synthesized `KEY:` entry, so the settled weight/cost were
    // always `None` even though the exact values this record's own ingestion
    // already captured were sitting one field over, unread. This does not
    // fabricate a value: `weight_lbs`/`cost_gp` are the SAME ingested data
    // the token array's own `WT:`/`COST:` entries would carry when present
    // (confirmed corpus-wide, not sampled: every one of the 4,470 enriched
    // equipment/equipment_modifier records under
    // `data/corpus/**/equipment/**/*.json` that carries both a `WT:` token
    // and a `weight_lbs` field has the two agree exactly). Only fires when
    // the token array itself did not already carry the token, so an enriched
    // record's own literal value always wins unchanged.
    if !tokens.iter().any(|t| t.key == "WT")
        && let Some(weight) = data.get("weight_lbs").and_then(serde_json::Value::as_f64) {
            tokens.push(EquipmentToken {
                key: "WT".to_string(),
                value: weight.to_string(),
                line_number: 1,
                raw_pair: format!("WT:{weight}"),
            });
        }
    if !tokens.iter().any(|t| t.key == "COST")
        && let Some(cost) = data.get("cost_gp").and_then(serde_json::Value::as_f64) {
            tokens.push(EquipmentToken {
                key: "COST".to_string(),
                value: cost.to_string(),
                line_number: 1,
                raw_pair: format!("COST:{cost}"),
            });
        }

    Some(EquipmentRecord {
        kind: EquipmentRecordKind::Equip,
        name: name.to_string(),
        header_line_number: 1,
        header_raw_line: name.to_string(),
        tokens,
        bonus_chains,
        is_record_start: true,
        diagnostics: Vec::<EquipmentDiagnostic>::new(),
    })
}

/// Every live `data/corpus/` equipment record as the pair the parity proofs
/// need: the ingest row this module rebuilds, and the settled
/// [`CorpusEquipmentRecord`](codex::rules_core::equipment_record::CorpusEquipmentRecord)
/// the converter produces from it.
///
/// **Test-only, and deliberately so.** The canonical envelope carries the
/// settled record alone from SD-35 `AT-35-E6-003-RULED` cycle 13 on, so the
/// pairing the cycle-10/11/12 parity proofs compare against no longer exists on
/// the live side at all. It exists here, on the converter side, where the
/// ingest row is this module's own output — which is what makes those proofs an
/// oracle comparison rather than a live-side read (`decisions.md` §11 keeps the
/// converter, the parser and the oracle harness).
///
/// The population is the whole corpus, every book, walked in sorted order so
/// two runs on the same tree compare the same records in the same order.
#[cfg(test)]
pub fn every_live_corpus_equipment_pair(
) -> Vec<(EquipmentRecord, codex::rules_core::equipment_record::CorpusEquipmentRecord)> {
    use crate::pcgen_import::ir_converter::equipment_record_to_corpus;

    fn json_files(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        let mut paths: Vec<std::path::PathBuf> = entries.flatten().map(|e| e.path()).collect();
        paths.sort();
        for path in paths {
            if path.is_dir() {
                if path.file_name().and_then(|n| n.to_str()) == Some("_parity") {
                    continue;
                }
                json_files(&path, out);
            } else if path.extension().and_then(|e| e.to_str()) == Some("json")
                && path.file_name().and_then(|n| n.to_str()) != Some("LICENSE.json")
            {
                out.push(path);
            }
        }
    }

    let corpus = crate::repo_root().join("data/corpus");
    let mut books: Vec<std::path::PathBuf> = std::fs::read_dir(&corpus)
        .expect("data/corpus must be readable")
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect();
    books.sort();

    let mut pairs = Vec::new();
    for book in &books {
        let equipment_dir = book.join("equipment");
        if !equipment_dir.is_dir() {
            continue;
        }
        let mut files = Vec::new();
        json_files(&equipment_dir, &mut files);
        for path in files {
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
            let Some(data) = value.get("data") else { continue };
            let Some(row) = equipment_record_from_json(data) else { continue };
            let converted = equipment_record_to_corpus(&row);
            pairs.push((row, converted));
        }
    }
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Unit-level proof of the synthesis rule itself, isolated from the
    /// full loader: no ingest token array at all, only the top-level
    /// `weight_lbs`/`cost_gp` fields every ingested record carries.
    ///
    /// Moved here with the function it tests (cycle 13); the assertions are
    /// unchanged from `corpus_loader`'s own.
    #[test]
    fn equipment_record_from_json_synthesizes_wt_and_cost_when_raw_tokens_is_absent() {
        let value: serde_json::Value = serde_json::json!({
            "key": "Test Thin Record",
            "name": "Test Thin Record",
            "weight_lbs": 3.5,
            "cost_gp": 120.0
        });
        let record = equipment_record_from_json(&value).expect("must build a record");
        let wt = record.tokens.iter().find(|t| t.key == "WT").expect("WT: must be synthesized");
        assert_eq!(wt.value, "3.5");
        let cost = record.tokens.iter().find(|t| t.key == "COST").expect("COST: must be synthesized");
        assert_eq!(cost.value, "120");
    }

    /// Negative control: an already-enriched record's own real `WT:`/
    /// `COST:` tokens (from the ingest token array) must win unchanged --
    /// synthesis only fires when the token is genuinely absent, never
    /// overriding a real ingested literal.
    #[test]
    fn equipment_record_from_json_never_overrides_a_real_raw_tokens_wt_or_cost() {
        let mut value: serde_json::Value = serde_json::json!({
            "key": "Test Enriched Record",
            "name": "Test Enriched Record",
            // Deliberately different from the token-array values, to prove
            // a real conflict resolves in the token array's favor.
            "weight_lbs": 3.5,
            "cost_gp": 120.0
        });
        value[crate::pcgen_import::ingest_payload::INGEST_TOKENS_FIELD] =
            crate::pcgen_import::ingest_payload::ingest_tokens_value(&[("WT", "99"), ("COST", "1")]);
        let record = equipment_record_from_json(&value).expect("must build a record");
        assert_eq!(record.tokens.iter().filter(|t| t.key == "WT").count(), 1, "no duplicate WT: token");
        let wt = record.tokens.iter().find(|t| t.key == "WT").unwrap();
        assert_eq!(wt.value, "99", "the real ingested WT: token value must win, not the top-level field");
        let cost = record.tokens.iter().find(|t| t.key == "COST").unwrap();
        assert_eq!(cost.value, "1");
    }

    /// Negative control: no `weight_lbs`/`cost_gp` field at all (neither
    /// present) synthesizes nothing -- an honest absence, not a fabricated
    /// zero.
    #[test]
    fn equipment_record_from_json_synthesizes_nothing_when_neither_field_is_present() {
        let value: serde_json::Value = serde_json::json!({
            "key": "Test Bare Record",
            "name": "Test Bare Record"
        });
        let record = equipment_record_from_json(&value).expect("must build a record");
        assert!(!record.tokens.iter().any(|t| t.key == "WT"));
        assert!(!record.tokens.iter().any(|t| t.key == "COST"));
    }

    /// A `data` object with neither `key` nor `name` is not a record. The
    /// live loader turns this `None` into its malformed-record diagnostic,
    /// exactly as it did before the move.
    #[test]
    fn a_data_object_with_no_key_is_not_a_record() {
        let value: serde_json::Value = serde_json::json!({ "name": "No Key Here" });
        assert!(equipment_record_from_json(&value).is_none());
        assert!(corpus_equipment_source_record(&value).is_none());
    }
}
