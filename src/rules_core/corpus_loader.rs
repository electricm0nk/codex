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
use crate::pcgen_import::lst_parser::spell::{LstSpellRecord, LstSpellRecordPayload};
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

/// Loads every spell record from every given book's corpus directory into
/// one `SourcePackageContent`, the spell-side sibling of
/// [`load_equipment_corpus`] above (SD28-E14-F1: closes the observation gap
/// `docs/release/SD-28-ultimate-book-content-ingestion/epic-breakdown.md`
/// names for `Kind::Spell` -- there was no probe reading a spell's real
/// on-disk record at all before this).
///
/// The on-disk Shape B v1 JSON carries `key` (the spell's corpus identity;
/// PF1 spell records have no separate `KEY:` token, see
/// `spell_resolver`'s own doc comment) and `school`, enough to reconstruct a
/// minimal-but-real `LstSpellRecord` that `spell_id_resolve` and
/// `pilot_compute_corpus::compute_pilot_with_corpus` both already consume
/// unchanged. Every other `LstSpellRecord` field stays `None`: this loader
/// does not fabricate mechanical data the JSON does not carry.
pub fn load_spell_corpus<'a>(roots: &[BookCorpusRoot<'_>]) -> SourcePackageContent<'a> {
    let mut package = SourcePackageContent::empty(
        "corpus_loader",
        SourceRef { lst_file: String::new(), line: 0 },
    );
    for root in roots {
        let spell_dir = root.dir.join("spell");
        if !spell_dir.is_dir() {
            continue;
        }
        for path in find_json_files(&spell_dir) {
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
            match spell_record_from_json(&path, data) {
                Some(record) => {
                    let record: &'static LstSpellRecord = Box::leak(Box::new(record));
                    package.push(crate::pcgen_import::ir_converter::convert_spell_record(record));
                }
                None => package.push_diagnostic(load_diagnostic(&path, "\"data\" is missing \"key\"")),
            }
        }
    }
    package
}

fn spell_record_from_json(path: &Path, data: &serde_json::Value) -> Option<LstSpellRecord> {
    let name = data.get("key").and_then(serde_json::Value::as_str)?.to_string();
    let school = data.get("school").and_then(serde_json::Value::as_str).map(str::to_string);
    let payload = LstSpellRecordPayload {
        name: name.clone(),
        output_name: None,
        spell_type: None,
        classes: None,
        school: school.clone(),
        descriptor: None,
        sub_school: None,
        components: None,
        casting_time: None,
        range: None,
        item: None,
        target_area: None,
        duration: None,
        save_info: None,
        spell_resistance: None,
        source_page: None,
        source_link: None,
        description: None,
        description_raw: None,
    };
    Some(LstSpellRecord {
        line_number: 1,
        source_path: path.display().to_string(),
        name,
        output_name: None,
        spell_type: None,
        classes: None,
        school,
        descriptor: None,
        sub_school: None,
        components: None,
        casting_time: None,
        range: None,
        item: None,
        target_area: None,
        duration: None,
        save_info: None,
        spell_resistance: None,
        source_page: None,
        source_link: None,
        description: None,
        description_raw: None,
        payload,
    })
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
    // Sorted, because the ORDER records are pushed into a
    // `SourcePackageContent` is significant -- a resolver reading that package
    // decides a key collision by position. Unsorted, that order is `read_dir`
    // order, which is the filesystem's, which is stable for one directory on
    // one machine and NOT stable across two checkouts of the same corpus. That
    // is the shape of nondeterminism that looks like a code change when two
    // agents compare measurements taken in different worktrees. Path order is a
    // property of the corpus itself, so every checkout of it agrees.
    // `race_resolver::find_json_files`, which copied this traversal, already
    // sorts for the same reason.
    out.sort();
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
    // SD-33 remediation wave 5 (`sd33-r5-skillcombat`): synthesize a KEY:
    // token from the ingested `data.key` field whenever `raw_tokens` did
    // not itself carry a literal `KEY:` entry -- not only when
    // `raw_tokens` was completely empty. `data.key` is ALWAYS the
    // ingestion pipeline's own record identity (the real `KEY:` token
    // when the LST line had one, else the record's own bare name --
    // `equipment_id_resolve`'s doc comment states this rule and the
    // ingestion `source.record_key` field already computes it this way).
    // Before this fix, a record with a non-empty `raw_tokens` list but no
    // literal `KEY:` entry among them (common: a keyless LST line whose
    // identity is its own first-column name) fell through
    // `equipment_key_token` to `None`, and `equipment_id_resolve` fell
    // back to matching on `.name` instead -- which is `data.name`, NOT
    // `data.key`, and diverges from it whenever the record also carries
    // an `OUTPUTNAME:` token (ingestion substitutes `OUTPUTNAME` into
    // `name` for display, e.g. `Companion Stone (Diplomacy)`'s real key
    // vs. its `name: "Companion Stone of [NAME]"`, an unresolved
    // OUTPUTNAME placeholder never meant to be an identity). That
    // silently broke `equipment_id_resolve` for every OUTPUTNAME-bearing,
    // KEY-less record -- 12 of `ultimate_psionics`'s own equipment
    // records among them, plus the shape-combat lane's own narrower
    // `engine_id_resolve_fails_templated_variant_record` finding
    // (`Psychoactive Skin (Defender)`/`(Hero)`), which this fix also
    // resolves as a side effect, not just those two instances.
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
    // two fields `encumbrance::weight_and_cost_from_record` reads
    // (`WT:`/`COST:`). The ingestion pipeline always captures a record's
    // own weight/cost as top-level `data.weight_lbs`/`data.cost_gp`
    // (`equipment_record_from_json`'s own caller, `arrow_slaying.json`'s
    // real on-disk shape among many: `"weight_lbs": 0.1`, no `raw_tokens`
    // array at all -- this module's own doc comment already names this a
    // "thin" record). Before this fix, a thin record's `tokens` list held
    // only the synthesized `KEY:` entry, so `weight_and_cost_from_record`
    // -- and therefore `encumbrance::equipment_key_resolves_a_carried_
    // weight`, the wiring probe's newly-widened check -- always returned
    // `None` for it even though the exact weight/cost this record's own
    // ingestion already captured was sitting one field over, unread. This
    // does not fabricate a value: `weight_lbs`/`cost_gp` are the SAME
    // ingested data `raw_tokens`' own `WT:`/`COST:` entries would carry
    // when present (confirmed corpus-wide, not sampled: every one of the
    // 4,470 enriched equipment/equipment_modifier records under
    // `data/corpus/**/equipment/**/*.json` that carries both a `WT:`
    // token and a `weight_lbs` field has the two agree exactly). Only fires
    // when `raw_tokens` itself did not already carry the token, so an
    // enriched record's own literal value always wins unchanged.
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

    /// `AT-34-E3-003` (bucket `M`, EQUIPMENT sub-causes, cycle 6): a real,
    /// on-disk "thin" record (no `raw_tokens` array at all --
    /// `data/corpus/core_rulebook/equipment/arms_armor/arrow_slaying.json`,
    /// verbatim: `"data": {"key": "Arrow (Slaying)", ..., "cost_gp": 0.0,
    /// "weight_lbs": 0.1}`, no `raw_tokens` key). Before this cycle's fix
    /// the loaded record's `tokens` held only a synthesized `KEY:` entry;
    /// now `WT:`/`COST:` are synthesized from the same already-ingested
    /// `weight_lbs`/`cost_gp` fields, so the real, already-wired
    /// `encumbrance::equipment_key_resolves_a_carried_weight` consumer can
    /// read them.
    #[test]
    fn a_thin_record_with_no_raw_tokens_still_synthesizes_its_real_weight_and_cost() {
        let roots = [BookCorpusRoot {
            book_id: "core_rulebook",
            dir: Path::new("data/corpus/core_rulebook"),
        }];
        let package = load_equipment_corpus(&roots);

        let (record, _) = equipment_id_resolve("Arrow (Slaying)", RuleSetId::Crb, &package)
            .expect("Arrow (Slaying) must resolve");
        assert!(record.bonus_chains.is_empty(), "this record genuinely has no raw_bonus_chains");
        let wt = record.tokens.iter().find(|t| t.key == "WT").expect("synthesized WT: token must be present");
        assert_eq!(wt.value, "0.1", "Arrow (Slaying)'s real ingested weight_lbs");
        let cost = record.tokens.iter().find(|t| t.key == "COST").expect("synthesized COST: token must be present");
        assert_eq!(cost.value, "0");

        assert!(
            crate::rules_core::encumbrance::equipment_key_resolves_a_carried_weight(
                "Arrow (Slaying)",
                &package
            ),
            "the synthesized WT: token must now make this thin record resolve a carried weight"
        );
    }

    /// Unit-level proof of the synthesis rule itself, isolated from the
    /// full loader: no `raw_tokens` at all, only the top-level
    /// `weight_lbs`/`cost_gp` fields every ingested record carries.
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
    /// `COST:` tokens (from `raw_tokens`) must win unchanged -- synthesis
    /// only fires when the token is genuinely absent, never overriding a
    /// real ingested literal.
    #[test]
    fn equipment_record_from_json_never_overrides_a_real_raw_tokens_wt_or_cost() {
        let value: serde_json::Value = serde_json::json!({
            "key": "Test Enriched Record",
            "name": "Test Enriched Record",
            "raw_tokens": [
                {"key": "WT", "value": "99"},
                {"key": "COST", "value": "1"}
            ],
            // Deliberately different from the raw_tokens values, to prove
            // a real conflict resolves in the raw_tokens' favor.
            "weight_lbs": 3.5,
            "cost_gp": 120.0
        });
        let record = equipment_record_from_json(&value).expect("must build a record");
        assert_eq!(record.tokens.iter().filter(|t| t.key == "WT").count(), 1, "no duplicate WT: token");
        let wt = record.tokens.iter().find(|t| t.key == "WT").unwrap();
        assert_eq!(wt.value, "99", "the real raw_tokens WT: value must win, not the top-level field");
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

    /// SD28-E14-F1: a real on-disk spell record (CRB's Animate Plants, seen
    /// while scoping the probe) loads and resolves with its real school
    /// intact through the unmodified `spell_id_resolve` path.
    #[test]
    fn a_real_on_disk_spell_record_loads_and_resolves_with_its_real_school() {
        use crate::rules_core::spell_resolver::spell_id_resolve;

        let roots = [BookCorpusRoot {
            book_id: "core_rulebook",
            dir: Path::new("data/corpus/core_rulebook"),
        }];
        let package = load_spell_corpus(&roots);
        assert!(!package.is_empty(), "real spell corpus must load real records");

        let (record, _) = spell_id_resolve("Animate Plants", RuleSetId::Crb, &package)
            .expect("Animate Plants must resolve");
        assert_eq!(record.school.as_deref(), Some("Transmutation"));
    }

    /// A book with no `spell/` subdirectory contributes nothing and does
    /// not panic -- mirrors the equipment loader's own equivalent test.
    #[test]
    fn a_book_with_no_spell_dir_contributes_nothing_without_panicking() {
        let roots = [BookCorpusRoot { book_id: "beastiary", dir: Path::new("data/corpus/beastiary") }];
        let package = load_spell_corpus(&roots);
        assert!(package.is_empty());
    }

    /// SD-33 remediation wave 5 (`sd33-r5-skillcombat`): a record whose
    /// ingested corpus JSON carries `data.key` != `data.name` (the LST
    /// record has no explicit `KEY:` token, but its `OUTPUTNAME:` token
    /// makes ingestion's own `name` field a *display* string, not the
    /// record's real identity) must still resolve by its real `key`, not
    /// silently fall through to `None`. Real, verbatim on-disk record:
    /// Ultimate Psionics' `Companion Stone (Diplomacy)`
    /// (`data/corpus/ultimate_psionics/equipment/companion_stone_diplomacy.json`)
    /// -- `key: "Companion Stone (Diplomacy)"`, `name: "Companion Stone of
    /// [NAME]"` (an unresolved `OUTPUTNAME:Companion Stone of [NAME]`
    /// placeholder, never meant to be an identity). Before this fix,
    /// `equipment_id_resolve("Companion Stone (Diplomacy)", ...)` returned
    /// `None`: no raw `KEY:` token exists among `raw_tokens` (the field
    /// simply isn't present on this LST line), so `equipment_key_token`
    /// returned `None` and identity fell back to `.name`, the OUTPUTNAME
    /// placeholder -- not `Companion Stone (Diplomacy)`. This is the same
    /// underlying defect the shape-combat lane named narrowly as
    /// `engine_id_resolve_fails_templated_variant_record` for two other
    /// units (`Psychoactive Skin (Defender)`/`(Hero)`); this fix closes
    /// the general case, not just those two instances.
    #[test]
    fn outputname_divergent_record_still_resolves_by_its_real_key() {
        let roots = [BookCorpusRoot {
            book_id: "ultimate_psionics",
            dir: Path::new("data/corpus/ultimate_psionics"),
        }];
        let package = load_equipment_corpus(&roots);

        let (record, _) = equipment_id_resolve("Companion Stone (Diplomacy)", RuleSetId::Crb, &package)
            .expect("Companion Stone (Diplomacy) must resolve by its real KEY, not its OUTPUTNAME display string");
        let bonus = record
            .bonus_chains
            .iter()
            .find(|b| b.qualifiers.first().map(String::as_str) == Some("SKILL"))
            .expect("real BONUS:SKILL|Diplomacy|4|TYPE=Competence chain must be present");
        assert_eq!(bonus.qualifiers.get(1).map(String::as_str), Some("Diplomacy"));
        assert_eq!(bonus.qualifiers.get(2).map(String::as_str), Some("4"));
    }
}

