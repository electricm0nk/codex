//! Real, book-agnostic `data/corpus/` loader.
//!
//! Built 2026-07-30 to close the desktop-runtime-reachability gap
//! (`docs/release/v0.6/book-agnostic-backend-gaps-scoping.md` finding 4):
//! every live character build previously used
//! `corpus_fixtures::corpus_fixture_bundle()` -- a hardcoded ~4-record
//! fixture, regardless of book. This module reads the real, on-disk Shape B
//! v1 JSON corpus (equipment content-kind first -- see module-level scope
//! note at the bottom) and pushes the canonical envelope for each record onto
//! one package, so every book-agnostic resolver (`equipment_id_resolve`,
//! `encumbrance.rs`, `equipment_effects.rs`, ...) reads real corpus data.
//!
//! # What this module reads, and what it does not
//!
//! SD-35 `AT-35-E6-003-RULED` cycle 13. It reads a corpus record's `data`
//! object and asks
//! [`crate::pcgen_import::corpus_equipment_json::corpus_equipment_source_record`]
//! what canonical record that object stands for. It does **not** read the
//! record's ingest token array or bonus-chain array, and it does not build an
//! ingest-format parser row: those field names, that traversal and the
//! `BONUS:` re-spelling are the converter's vocabulary and moved to the
//! converter's side of the boundary (`decisions.md` §11, §19). The envelope
//! this loader pushes carries the settled
//! [`crate::rules_core::equipment_record::CorpusEquipmentRecord`] alone.
//!
//! A record enriched with the two ingest arrays settles a full, accurate set
//! of values. A record without them (not yet enriched, or a
//! `web_second_source`/`same_book_fallback` record with no raw LST line to
//! enrich from) settles a *thin* record -- identity and name present
//! (synthesized from the JSON's own `key`/`name` fields) so name-based
//! resolution still works, weight and price present whenever the JSON's own
//! `weight_lbs`/`cost_gp` fields carry them, and every mechanical field an
//! honest `None`. This is an honest degradation, not a silent one.

use std::fs;
use std::path::Path;

// SD-35 `AT-35-E6-003-RULED` cycle 9: the spell parser import is GONE. It was
// here for `load_lst_fixture_corpus`, which parsed the desktop's bundled raw
// `.lst` fixture rows at run time; that package is produced at build time now
// by `src/bin/gen_desktop_fixture_corpus.rs` and read as data through
// [`load_book_corpus`] below, so no live path parses a PCGen row any more.
use crate::rules_core::source_content::{SourceContentRecord, SourcePackageContent, SourceRef};
use crate::rules_core::spell_record::CorpusSpellRecord;

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
            // SD-35 `AT-35-E6-003-RULED` cycle 13: ONE question, asked at the
            // ingest boundary -- "what canonical record does this corpus JSON
            // object stand for?". Before this cycle this function read the
            // record's own ingest token array and bonus-chain array, rebuilt an
            // ingest-format parser row out of them and ran the converter over
            // that row, all on the live side. The field names, the traversal
            // and the `BONUS:` re-spelling are converter vocabulary
            // (`decisions.md` §11), so they moved whole to
            // `pcgen_import::corpus_equipment_json` and this loader names no
            // token, no token array and no parser row.
            match crate::pcgen_import::corpus_equipment_json::corpus_equipment_source_record(data) {
                Some(record) => package.push(record),
                None => package.push_diagnostic(load_diagnostic(&path, "\"data\" is missing key/name")),
            }
        }
    }
    package
}

// SD-35 `AT-35-E6-003-RULED` cycle 14: the ingest boundary asks the SAME
// question for a race and a racial trait that cycle 13 taught it to ask for a
// piece of equipment -- "what canonical record does this corpus JSON object
// stand for?". Both live here, in the loader, because THIS module is the live
// side's one ingest boundary; before this cycle `race_resolver` held its own
// three `use crate::pcgen_import::...` lines and re-read the ingest token and
// bonus-chain arrays on every accessor call (`decisions.md` §11, §19).
//
// BOOKED HONESTLY: this is a RELABEL for one hit and a CLOSURE for three. The
// eleven run-time token readings left the live side for good; the boundary
// call itself did not vanish -- it moved from `race_resolver` into the file
// that already owned the boundary, and this module's own hit count rises by
// one as a result. It clears the same way the equipment one does: when
// `data/corpus/` race JSON carries the settled fields itself.
use crate::pcgen_import::corpus_race_json;

/// The settled [`CorpusRaceRecord`](crate::rules_core::race_record::CorpusRaceRecord)
/// one `data/corpus/<book>/race/<slug>.json` record's `data` object stands for.
///
/// `None` when the object is not a race payload at all; the caller records the
/// skip as a diagnostic rather than dropping it silently.
pub(crate) fn corpus_race_record(
    data: &serde_json::Value,
) -> Option<crate::rules_core::race_record::CorpusRaceRecord> {
    corpus_race_json::corpus_race_source_record(data)
}

/// The settled
/// [`CorpusRaceTraitRecord`](crate::rules_core::race_record::CorpusRaceTraitRecord)
/// one `data/corpus/<book>/race_trait/<race>/<slug>.json` record's `data`
/// object stands for.
pub(crate) fn corpus_race_trait_record(
    data: &serde_json::Value,
) -> Option<crate::rules_core::race_record::CorpusRaceTraitRecord> {
    corpus_race_json::corpus_race_trait_source_record(data)
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
                    let record: &'static CorpusSpellRecord = Box::leak(Box::new(record));
                    package.push(SourceContentRecord::spell(
                        SourceRef {
                            lst_file: record.source_path.clone(),
                            line: record.line_number as u32,
                        },
                        record,
                    ));
                }
                None => package.push_diagnostic(load_diagnostic(&path, "\"data\" is missing \"key\"")),
            }
        }
    }
    package
}

/// Reconstruct one already-converted corpus spell record from its on-disk
/// Shape B v1 JSON.
///
/// SD-35 `AT-35-E6-003-RULED` cycle 8: this reads `data.key` and
/// `data.school` -- two settled corpus values, no ingest-format token
/// anywhere -- and yields the live side's own
/// [`CorpusSpellRecord`]. It used to build the ingest-format parser struct
/// and hand it to `pcgen_import::ir_converter` to be re-converted, which was
/// the live side running the converter over data the converter had already
/// produced.
fn spell_record_from_json(path: &Path, data: &serde_json::Value) -> Option<CorpusSpellRecord> {
    let name = data.get("key").and_then(serde_json::Value::as_str)?.to_string();
    let school = data.get("school").and_then(serde_json::Value::as_str).map(str::to_string);
    Some(CorpusSpellRecord::from_corpus_json_fields(path.display().to_string(), name, school))
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

/// The result of loading `data/sheet_rules/` (SD-35 AT-35-E2-002).
pub struct SheetRuleLoad {
    pub package: crate::rules_core::sheet_rule::SheetRulePackage,
    /// One per file that failed to read or parse; the rest of the package still loads.
    pub diagnostics: Vec<crate::rules_core::source_content::SourceContentDiagnostic>,
    pub rule_files: usize,
    pub var_files: usize,
}

/// Loads the whole `data/sheet_rules/` package -- every `<book>/<kind>/**/*.json` rule file
/// (each a JSON array of `SheetRule`) and every `_vars/<VarId>.json` table -- into one
/// [`SheetRulePackage`](crate::rules_core::sheet_rule::SheetRulePackage). The live side's only
/// reader of the package: it reads our schema and nothing else (`decisions.md` §11).
///
/// `_refused.json`, `_report.json`, `_defects/` and `GENERATED` are the converter's own
/// reports, not rules, and are skipped. Files are parsed on `available_parallelism` threads
/// (the package is ~52,000 files); the result is deterministic because rules are keyed by id.
pub fn load_sheet_rules(dir: &Path) -> SheetRuleLoad {
    load_sheet_rules_filtered(dir, &|_, _| true)
}

/// [`load_sheet_rules`] restricted to the `(book, kind)` pairs `keep` accepts; `_vars/` always
/// loads. The per-kind gates use it to read one kind's live directory.
pub fn load_sheet_rules_filtered(dir: &Path, keep: &dyn Fn(&str, &str) -> bool) -> SheetRuleLoad {
    use crate::rules_core::sheet_rule::{SheetRule, SheetRulePackage, VarTable};

    let mut rule_paths: Vec<std::path::PathBuf> = Vec::new();
    let mut var_paths: Vec<std::path::PathBuf> = Vec::new();
    let mut diagnostics = Vec::new();
    let mut books: Vec<std::path::PathBuf> = match fs::read_dir(dir) {
        Ok(entries) => entries.flatten().map(|e| e.path()).filter(|p| p.is_dir()).collect(),
        Err(_) => {
            diagnostics.push(load_diagnostic(dir, "cannot read the sheet-rules root"));
            Vec::new()
        }
    };
    books.sort();
    for book_dir in books {
        let book = book_dir.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        if book == "_vars" {
            var_paths.extend(find_json_files(&book_dir));
            continue;
        }
        if book.starts_with('_') {
            continue;
        }
        let mut kinds: Vec<std::path::PathBuf> = match fs::read_dir(&book_dir) {
            Ok(entries) => entries.flatten().map(|e| e.path()).filter(|p| p.is_dir()).collect(),
            Err(_) => continue,
        };
        kinds.sort();
        for kind_dir in kinds {
            let kind = kind_dir.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
            if keep(&book, &kind) {
                rule_paths.extend(find_json_files(&kind_dir));
            }
        }
    }

    fn parse_all<T: serde::de::DeserializeOwned + Send>(paths: &[std::path::PathBuf]) -> Vec<Result<T, (std::path::PathBuf, &'static str)>> {
        let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4).clamp(1, 8);
        let chunk = paths.len().div_ceil(threads).max(1);
        let mut out = Vec::with_capacity(paths.len());
        std::thread::scope(|scope| {
            let handles: Vec<_> = paths
                .chunks(chunk)
                .map(|slice| {
                    scope.spawn(move || {
                        slice
                            .iter()
                            .map(|path| {
                                let Ok(text) = fs::read_to_string(path) else { return Err((path.clone(), "failed to read file")) };
                                serde_json::from_str::<T>(&text).map_err(|_| (path.clone(), "failed to parse as a sheet-rules JSON file"))
                            })
                            .collect::<Vec<_>>()
                    })
                })
                .collect();
            for handle in handles {
                out.extend(handle.join().expect("a sheet-rules parse thread must not panic"));
            }
        });
        out
    }

    let mut package = SheetRulePackage::new();
    let rule_files = rule_paths.len();
    let var_files = var_paths.len();
    for parsed in parse_all::<Vec<SheetRule>>(&rule_paths) {
        match parsed {
            Ok(rules) => rules.into_iter().for_each(|r| package.insert_rule(r)),
            Err((path, message)) => diagnostics.push(load_diagnostic(&path, message)),
        }
    }
    for parsed in parse_all::<VarTable>(&var_paths) {
        match parsed {
            Ok(table) => package.insert_var(table),
            Err((path, message)) => diagnostics.push(load_diagnostic(&path, message)),
        }
    }
    package.finish();
    SheetRuleLoad { package, diagnostics, rule_files, var_files }
}

/// This checkout's own `data/sheet_rules/` package, loaded once per process.
///
/// SD-35 `AT-35-E6-001`: the live-side prerequisite readers (`feat_prereqs`,
/// `pilot_compute::prestige_class_entry_gate`) evaluate a record's CONVERTED
/// [`Applies`](crate::rules_core::sheet_rule::Applies) gate rather than parsing the
/// ingest format's `PRE*` token text at run time, so they need the package the sheet is
/// rendered from. The desktop crate already keeps exactly this cache
/// (`character_hub::sheet_rule_package`) and passes its package down; a caller inside
/// `rules_core` that has no package to pass -- `compute_class_chassis`'s prestige entry
/// gate is called before any package is attached -- reads it here instead.
///
/// `None` when the directory is absent or carries no rules. A missing package is never a
/// verdict: every caller reports "not verified" for it rather than refusing a character.
pub fn live_sheet_rules() -> Option<&'static crate::rules_core::sheet_rule::SheetRulePackage> {
    static PACKAGE: std::sync::OnceLock<Option<crate::rules_core::sheet_rule::SheetRulePackage>> =
        std::sync::OnceLock::new();
    PACKAGE
        .get_or_init(|| {
            let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/sheet_rules");
            let load = load_sheet_rules(&dir);
            if load.package.rules.is_empty() {
                None
            } else {
                Some(load.package)
            }
        })
        .as_ref()
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

/// Loads one book-corpus tree's **equipment and spell** records into a single
/// `SourcePackageContent`.
///
/// The two-kind sibling of [`load_equipment_corpus`] and [`load_spell_corpus`],
/// for a caller that wants one package covering both kinds of a
/// [`BookCorpusRoot`] rather than two it has to merge itself.
///
/// # Why this exists
///
/// SD-35 `AT-35-E6-003-RULED` cycle 9. This replaced `load_lst_fixture_corpus`,
/// which built the desktop's bundled fixture package by parsing raw `.lst`
/// record lines and running the converter over them **at run time** — the last
/// live path in the crate that did. Every Epic 6 census since cycle 1 named the
/// clearing condition for those hits in the same words: *"clears when that
/// package is produced at build time and read as data."*
///
/// It is produced at build time now, by
/// `src/bin/gen_desktop_fixture_corpus.rs`, into the same
/// `<root>/spell/*.json` + `<root>/equipment/*.json` layout the real corpus
/// uses; the desktop ships the converted records and this function reads them
/// with the loaders that already existed. The `.txt` fixtures stay put as that
/// producer's input — `decisions.md` §11 keeps the converter and its inputs;
/// what it forbids is the live side running them.
///
/// # Failure
///
/// Same contract as the two loaders it composes: a file that cannot be read or
/// parsed becomes a diagnostic on the package rather than aborting the load, so
/// one malformed record never takes down every other record in the tree. A
/// caller shipping a bounded, committed package should assert on the record
/// count it expects — an empty package means the resource did not ship.
pub fn load_book_corpus<'a>(roots: &[BookCorpusRoot<'_>]) -> SourcePackageContent<'a> {
    let mut package = load_equipment_corpus(roots);
    let spells = load_spell_corpus(roots);
    for record in spells.records {
        package.push(record);
    }
    for diagnostic in spells.diagnostics {
        package.push_diagnostic(diagnostic);
    }
    package
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::equipment_resolver::equipment_id_resolve;
    use crate::rules_core::rules_tables::RuleSetId;

    /// Real, on-disk enriched equipment record (ARG's Dogslicer) loads
    /// through the full package loader and resolves with real mechanical
    /// values intact -- proves the loader, not a synthetic fixture.
    ///
    /// SD-35 `AT-35-E6-003-RULED` cycle 13: the assertions read the settled
    /// `weight_lbs`/`cost_gp` rather than the `WT:`/`COST:` tokens they were
    /// settled from. Same record, same two numbers, same corpus file; the
    /// resolver hands back a `CorpusEquipmentRecord` now.
    #[test]
    fn a_real_on_disk_enriched_record_loads_and_resolves_with_real_values() {
        let roots = [BookCorpusRoot {
            book_id: "advanced_race_guide",
            dir: Path::new("data/corpus/advanced_race_guide"),
        }];
        let package = load_equipment_corpus(&roots);

        let (record, _) =
            equipment_id_resolve("Dogslicer", RuleSetId::Crb, &package).expect("Dogslicer must resolve");
        assert_eq!(record.weight_lbs, Some(1.0), "Dogslicer's real weight from the enriched corpus");
        assert_eq!(record.cost_gp, Some(8.0), "Dogslicer's real price from the enriched corpus");
    }

    /// `AT-34-E3-003` (bucket `M`, EQUIPMENT sub-causes, cycle 6): a real,
    /// on-disk "thin" record (no ingest token array at all --
    /// `data/corpus/core_rulebook/equipment/arms_armor/arrow_slaying.json`,
    /// verbatim: `"data": {"key": "Arrow (Slaying)", ..., "cost_gp": 0.0,
    /// "weight_lbs": 0.1}`, no token-array key). Before this cycle's fix
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
        // This record genuinely declares no bonus chains, so every field a
        // chain would have settled is an honest absence.
        assert!(record.var_bonuses.is_empty(), "this record genuinely declares no bonus chains");
        assert!(record.skill_check_bonus.is_none());
        assert!(record.ability_score_bonus.is_none());
        assert_eq!(record.weight_lbs, Some(0.1), "Arrow (Slaying)'s real ingested weight_lbs");
        assert_eq!(record.cost_gp, Some(0.0), "Arrow (Slaying)'s real ingested cost_gp");

        assert!(
            crate::rules_core::encumbrance::equipment_key_resolves_a_carried_weight(
                "Arrow (Slaying)",
                &package
            ),
            "the synthesized WT: token must now make this thin record resolve a carried weight"
        );
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

    /// SD-35 `AT-35-E6-003-RULED` cycle 8: the spell half of the corpus
    /// loader owns its own converted record shape. `load_spell_corpus`
    /// reads `data/corpus/<book>/spell/*.json` -- already-converted corpus
    /// data -- and must produce
    /// [`crate::rules_core::spell_record::CorpusSpellRecord`] values
    /// without routing through the ingest-format parser struct
    /// (`pcgen_import::lst_parser::spell::LstSpellRecord`) or the
    /// converter's `ir_converter::convert_spell_record` entry point at run
    /// time. The binding claim is the TYPE the live resolver hands back,
    /// proved here over a real on-disk record rather than a fixture:
    /// `spell_id_resolve` returns a `&CorpusSpellRecord`, and the live
    /// envelope constructor built it.
    #[test]
    fn the_live_spell_loader_yields_a_live_owned_converted_record() {
        use crate::rules_core::spell_record::CorpusSpellRecord;
        use crate::rules_core::spell_resolver::spell_id_resolve;

        let roots = [BookCorpusRoot {
            book_id: "core_rulebook",
            dir: Path::new("data/corpus/core_rulebook"),
        }];
        let package = load_spell_corpus(&roots);
        let (record, _) = spell_id_resolve("Animate Plants", RuleSetId::Crb, &package)
            .expect("Animate Plants must resolve");
        // The type annotation is the assertion: this does not compile if
        // the payload is still the ingest-format parser struct.
        let record: &CorpusSpellRecord = record;
        assert_eq!(record.name, "Animate Plants");
        assert_eq!(record.school.as_deref(), Some("Transmutation"));
        assert!(
            record.source_path.ends_with("animate_plants.json"),
            "provenance must be the corpus JSON the loader actually read, got {:?}",
            record.source_path
        );
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
    /// `None`: no raw `KEY:` token exists in the ingest token array (the field
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
        assert_eq!(record.identity, "Companion Stone (Diplomacy)");
        let bonus = record
            .skill_check_bonus
            .as_ref()
            .expect("this record's real +4 competence bonus to Diplomacy must be settled");
        assert_eq!(bonus.skill, "Diplomacy");
        assert_eq!(bonus.bonus, 4);
    }
}

