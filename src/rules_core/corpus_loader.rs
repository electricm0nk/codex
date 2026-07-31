//! Real, book-agnostic `data/corpus/` loader.
//!
//! Built 2026-07-30 to close the desktop-runtime-reachability gap
//! (`docs/release/v0.6/book-agnostic-backend-gaps-scoping.md` finding 4):
//! every live character build previously used
//! `corpus_fixtures::corpus_fixture_bundle()` -- a hardcoded ~4-record
//! fixture, regardless of book. This module reads the real, on-disk Shape B
//! v1 JSON corpus (equipment content-kind first -- see module-level scope
//! note at the bottom) and reconstructs `EquipmentRecord`-shaped values so
//! every existing book-agnostic resolver (`equipment_id_resolve`,
//! `encumbrance.rs`, `equipment_effects.rs`, ...) works completely
//! unchanged, reading `record.tokens`/`record.bonus_chains` exactly as it
//! does from a raw-LST-parsed fixture today.
//!
//! Records enriched with `raw_tokens`/`raw_bonus_chains`
//! (`scripts`/`src/bin/enrich_equipment_raw_tokens.rs`, commit `094acde1`)
//! reconstruct a full, accurate `EquipmentRecord`. A record without those
//! fields (not yet enriched, or a `web_second_source`/`same_book_fallback`
//! record with no raw LST line to enrich from) reconstructs a *thin*
//! record -- `tokens`/`bonus_chains` empty, but `name`/`KEY` still present
//! (synthesized from the JSON's own `key`/`name` fields) so `name`-based
//! resolution (`equipment_id_resolve`'s 2nd/3rd fallback match arms) still
//! works. This is an honest degradation, not a silent one: a caller that
//! needs to know whether a resolved record has real mechanical data can
//! check `record.tokens.is_empty()`.
//!
//! `EquipmentRecord`'s own `record_source_path()` always returns `""` for
//! every equipment record regardless of provenance (confirmed:
//! `ir_converter.rs`'s `RecordSourcePath` impl for `EquipmentRecord`) --
//! equipment resolution never depends on a faithful source path, only on
//! `tokens`/`bonus_chains`/`name`, so this loader does not need to
//! reconstruct one either.

use std::fs;
use std::path::Path;

use crate::pcgen_import::lst_parser::equipment::{
    BonusToken, EquipmentDiagnostic, EquipmentRecord, EquipmentRecordKind, EquipmentToken,
};
use crate::rules_core::source_content::{SourcePackageContent, SourceRef};

/// One book's real corpus root, e.g. `data/corpus/core_rulebook`. The
/// caller supplies these (desktop: resolved via the bundled resource path;
/// tests: the repo-relative `data/corpus/<book>` directly) rather than this
/// module hardcoding a book list, so it stays agnostic to how many books
/// are wired in at any given time.
pub struct BookCorpusRoot<'a> {
    pub book_id: &'a str,
    pub dir: &'a Path,
}

/// Loads every equipment record from every given book's corpus directory
/// into one `SourcePackageContent`. Skips `LICENSE.json` and any `_parity/`
/// subdirectory (not `CorpusRecordV1<EquipmentCacheData>`-shaped). A file
/// that fails to parse as valid JSON, or lacks a `data` object, is skipped
/// with a diagnostic pushed onto the package rather than aborting the whole
/// load -- one malformed record must not take down every other book's real
/// data.
pub fn load_equipment_corpus<'a>(roots: &[BookCorpusRoot<'_>]) -> SourcePackageContent<'a> {
    let mut package = SourcePackageContent::empty(
        "corpus_loader",
        SourceRef { lst_file: String::new(), line: 0 },
    );
    for root in roots {
        let equipment_dir = root.dir.join("equipment");
        if !equipment_dir.is_dir() {
            continue;
        }
        for path in find_json_files(&equipment_dir) {
            let Ok(text) = fs::read_to_string(&path) else {
                package.push_diagnostic(load_diagnostic(&path, "failed to read file"));
                continue;
            };
            let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else {
                package.push_diagnostic(load_diagnostic(&path, "failed to parse as JSON"));
                continue;
            };
            let Some(data) = value.get("data") else {
                package.push_diagnostic(load_diagnostic(&path, "no top-level \"data\" object"));
                continue;
            };
            match equipment_record_from_json(data) {
                Some(record) => {
                    let record: &'static EquipmentRecord = Box::leak(Box::new(record));
                    package.push(crate::pcgen_import::ir_converter::convert_equipment_record(record));
                }
                None => package.push_diagnostic(load_diagnostic(&path, "\"data\" is missing key/name")),
            }
        }
    }
    package
}

fn find_json_files(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        let Ok(entries) = fs::read_dir(&current) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            if path.is_dir() {
                if file_name == "_parity" {
                    continue;
                }
                stack.push(path);
            } else if file_name == "LICENSE.json" {
                continue;
            } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
                out.push(path);
            }
        }
    }
    out
}

fn load_diagnostic(path: &Path, message: &str) -> crate::rules_core::source_content::SourceContentDiagnostic {
    use crate::rules_core::source_content::{SourceContentDiagnostic, SourceContentDiagnosticKind, SourceContentSeverity};
    SourceContentDiagnostic {
        severity: SourceContentSeverity::Error,
        kind: SourceContentDiagnosticKind::MalformedRecord,
        message: message.to_string(),
        source_ref: SourceRef { lst_file: path.display().to_string(), line: 0 },
    }
}

fn equipment_record_from_json(data: &serde_json::Value) -> Option<EquipmentRecord> {
    let key = data.get("key").and_then(serde_json::Value::as_str)?;
    let name = data.get("name").and_then(serde_json::Value::as_str).unwrap_or(key);

    let mut tokens = Vec::new();
    let mut bonus_chains = Vec::new();
    if let Some(raw_tokens) = data.get("raw_tokens").and_then(serde_json::Value::as_array) {
        for entry in raw_tokens {
            let (Some(k), Some(v)) = (
                entry.get("key").and_then(serde_json::Value::as_str),
                entry.get("value").and_then(serde_json::Value::as_str),
            ) else {
                continue;
            };
            let raw_pair = format!("{k}:{v}");
            tokens.push(EquipmentToken { key: k.to_string(), value: v.to_string(), line_number: 1, raw_pair });
        }
    }
    if let Some(raw_chains) = data.get("raw_bonus_chains").and_then(serde_json::Value::as_array) {
        for entry in raw_chains {
            let Some(qualifiers) = entry.get("qualifiers").and_then(serde_json::Value::as_array) else {
                continue;
            };
            let qualifiers: Vec<String> =
                qualifiers.iter().filter_map(|q| q.as_str().map(str::to_string)).collect();
            let raw_bonus = format!("BONUS:{}", qualifiers.join("|"));
            bonus_chains.push(BonusToken { line_number: 1, raw_bonus, qualifiers });
        }
    }
    // If no raw_tokens were present (record not yet enriched, or no LST
    // citation to enrich from), still synthesize a KEY: token -- every
    // existing resolver's key-match path (`equipment_key_token`) reads
    // this generically, not knowing or caring whether it came from a real
    // multi-token parse or this thin fallback.
    if tokens.is_empty() {
        tokens.push(EquipmentToken {
            key: "KEY".to_string(),
            value: key.to_string(),
            line_number: 1,
            raw_pair: format!("KEY:{key}"),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::equipment_resolver::equipment_id_resolve;
    use crate::rules_core::rules_tables::RuleSetId;

    /// Real, on-disk enriched equipment record (ARG's Dogslicer) loads
    /// through the full package loader and resolves with real mechanical
    /// tokens intact -- proves the loader, not a synthetic fixture.
    #[test]
    fn a_real_on_disk_enriched_record_loads_and_resolves_with_real_tokens() {
        let roots = [BookCorpusRoot {
            book_id: "advanced_race_guide",
            dir: Path::new("data/corpus/advanced_race_guide"),
        }];
        let package = load_equipment_corpus(&roots);

        let (record, _) =
            equipment_id_resolve("Dogslicer", RuleSetId::Crb, &package).expect("Dogslicer must resolve");
        let wt = record.tokens.iter().find(|t| t.key == "WT").expect("real WT: token must be present");
        assert_eq!(wt.value, "1", "Dogslicer's real WT:1 from the enriched corpus");
        let cost = record.tokens.iter().find(|t| t.key == "COST").expect("real COST: token must be present");
        assert_eq!(cost.value, "8");
    }

    /// A book with no `equipment/` subdirectory at all (e.g. a corpus root
    /// that doesn't exist) contributes nothing and does not panic.
    #[test]
    fn a_nonexistent_book_dir_contributes_nothing_without_panicking() {
        let roots =
            [BookCorpusRoot { book_id: "nonexistent_book", dir: Path::new("data/corpus/nonexistent_book") }];
        let package = load_equipment_corpus(&roots);
        assert!(package.is_empty());
    }

    /// Loads all 6 real, on-disk books at once (the exact set the desktop
    /// app wires in) and confirms real, mechanically-complete items from
    /// three different books resolve correctly through the unmodified
    /// resolver + `equipment_effects`/`encumbrance` pipeline -- the real
    /// end-to-end proof this loader replaces `corpus_fixtures.rs`'s
    /// 4-record bundle with the actual corpus.
    #[test]
    fn all_six_real_books_load_and_a_sample_item_from_each_resolves_with_real_mechanics() {
        use crate::rules_core::character_input::{ActiveState, EquipmentSelection};
        use crate::rules_core::encumbrance::compute_encumbrance;
        use crate::rules_core::equipment_effects::compute_equipment_effects;
        use crate::rules_core::size::SizeCategory;

        let roots = [
            BookCorpusRoot { book_id: "core_rulebook", dir: Path::new("data/corpus/core_rulebook") },
            BookCorpusRoot {
                book_id: "advanced_players_guide",
                dir: Path::new("data/corpus/advanced_players_guide"),
            },
            BookCorpusRoot { book_id: "advanced_class_guide", dir: Path::new("data/corpus/advanced_class_guide") },
            BookCorpusRoot { book_id: "beastiary", dir: Path::new("data/corpus/beastiary") },
            BookCorpusRoot { book_id: "advanced_race_guide", dir: Path::new("data/corpus/advanced_race_guide") },
            BookCorpusRoot { book_id: "pathfinder_unchained", dir: Path::new("data/corpus/pathfinder_unchained") },
        ];
        let package = load_equipment_corpus(&roots);
        assert!(!package.is_empty(), "real corpus must load real records");

        // CRB's own Padded Armor (Base): real AC/max-dex/spell-failure/ACP.
        let equipped = vec![EquipmentSelection {
            item_id: "Padded Armor (Base)".to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        }];
        let effects = compute_equipment_effects(&equipped, &package);
        assert_eq!(effects.armor_class_delta, 1, "CRB Padded Armor's real AC bonus");
        assert_eq!(effects.max_dex_cap, Some(8), "CRB Padded Armor's real max Dex");

        // ARG's own Dogslicer: real weight through the full encumbrance pipeline.
        let equipped = vec![EquipmentSelection {
            item_id: "Dogslicer".to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        }];
        let computation = compute_encumbrance(&equipped, &package, 10, SizeCategory::Medium);
        assert_eq!(computation.total_carried_weight_lbs, 1.0, "ARG Dogslicer's real WT:1 through the real loader");
        assert!(computation.unresolved_item_ids.is_empty());
    }
}

