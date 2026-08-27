//! Runtime engine tables for the seven Epic 2 kinds that had no table at
//! all: `ability`, `template`, `trait`, `deity`, `domain`, `skill`,
//! `language` (`AT-34-E2-001`). `companion` (Epic 2's eighth kind) already
//! has a real table -- `rules_tables::companion_chassis`, built in SD-29 --
//! and is not rebuilt here; its own `companion_resolve` is exercised
//! directly by `--epic2-table-transcript` (`src/bin/v06_work_inventory.rs`).
//!
//! Each table here is loaded at call time from the live corpus tree under
//! `data/corpus/<book>/<dir>/*.json`, one book-directory read per kind,
//! never a hand-authored literal and never a fabricated field. A present
//! `(book, key)` returns the real record; an absent one returns `None` --
//! never a default (`AT-34-E2-002`).
//!
//! **The one directory name that disagrees with its kind:** `trait`'s 487
//! corpus units all live under `trait_generic/`, not `trait/` -- a bare
//! `data/corpus/*/trait/*.json` glob returns zero where 487 real records
//! exist one directory over. This is exactly the "shallow glob lies" hazard
//! named in `workflow-instruction.md §4`, caught here rather than shipped.
//! Verified: `find data/corpus -path '*/trait_generic/*.json' | wc -l` ->
//! `487`, matching `docs/work-inventory.json`'s `trait` population exactly.

use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// One corpus record, flattened to the fields every one of the seven kinds
/// carries in `data/corpus/**/*.json`. Nothing here is computed or
/// inferred -- every field is read straight off the record's own JSON.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleKindRecord {
    pub book: String,
    pub key: String,
    pub name: String,
    pub source_path: String,
    pub source_line: u64,
    pub description: Option<String>,
    pub raw_token_count: usize,
}

/// `kind -> corpus directory name`. Every entry matches its kind's own
/// name except `trait`, whose corpus records live under `trait_generic/`.
pub const SEVEN_KIND_DIRS: &[(&str, &str)] = &[
    ("ability", "ability"),
    ("template", "template"),
    ("trait", "trait_generic"),
    ("deity", "deity"),
    ("domain", "domain"),
    ("skill", "skill"),
    ("language", "language"),
];

/// The real corpus directory name for one of the seven kinds, or `None` for
/// any other kind (including `companion`, whose table lives in
/// `companion_chassis`, and `power`, which Epic 2 does not build).
pub fn kind_dir_for(kind: &str) -> Option<&'static str> {
    SEVEN_KIND_DIRS.iter().find(|(k, _)| *k == kind).map(|(_, d)| *d)
}

/// The engine table for one kind: every record the live corpus carries
/// under that kind's directory, across every book, keyed by
/// `(book, corpus key)` -- the same pair `docs/work-inventory.json` already
/// carries per unit as `(book, corpus_key)`.
#[derive(Debug, Default)]
pub struct SimpleKindTable {
    pub kind: String,
    pub dir: &'static str,
    records: BTreeMap<(String, String), SimpleKindRecord>,
    /// `"{book}:{source_file}:{source_line}"` -> `(book, key)`, populated
    /// ONLY for records whose own JSON carries a `rename.coordinate`
    /// (PI-masked records -- `decisions.md §14`). Never built from a
    /// derived or guessed coordinate, only the one the record's own
    /// ingestion already wrote down.
    by_coordinate: BTreeMap<String, (String, String)>,
}

impl SimpleKindTable {
    /// The real record for a present `(book, key)`, or a named refusal
    /// (`None`) for an absent one -- never a fabricated or defaulted entry
    /// (`AT-34-E2-002`).
    pub fn resolve(&self, book: &str, key: &str) -> Option<&SimpleKindRecord> {
        self.records.get(&(book.to_string(), key.to_string()))
    }

    /// The real record for a present `"{book}:{source_file}:{source_line}"`
    /// coordinate, or `None` -- the PI-safe lookup path for a record whose
    /// `key`/`name` were masked at ingestion (`decisions.md §14`: match on
    /// the record's already-stored coordinates, never the redacted real
    /// name). Returns the SAME masked-key record `resolve` would, never a
    /// fabricated one.
    pub fn resolve_by_coordinate(&self, coordinate: &str) -> Option<&SimpleKindRecord> {
        let (book, key) = self.by_coordinate.get(coordinate)?;
        self.records.get(&(book.clone(), key.clone()))
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn books(&self) -> BTreeSet<&str> {
        self.records.keys().map(|(b, _)| b.as_str()).collect()
    }
}

/// Load one kind's table from the live corpus tree rooted at `repo_root`.
/// An unknown kind (anything `kind_dir_for` does not resolve) yields an
/// empty table rather than panicking -- callers who need to distinguish
/// "unknown kind" from "kind with zero corpus records" check
/// `kind_dir_for` themselves first.
pub fn load_simple_kind_table(repo_root: &Path, kind: &str) -> SimpleKindTable {
    let dir = match kind_dir_for(kind) {
        Some(d) => {
            d
        }
        None => {
            return SimpleKindTable {
                kind: kind.to_string(),
                dir: "",
                records: BTreeMap::new(),
                by_coordinate: BTreeMap::new(),
            }
        }
    };
    let mut records = BTreeMap::new();
    let mut by_coordinate = BTreeMap::new();
    let corpus_root = repo_root.join("data/corpus");
    if let Ok(book_dirs) = std::fs::read_dir(&corpus_root) {
        for book_entry in book_dirs.flatten() {
            let book_path = book_entry.path();
            if !book_path.is_dir() {
                continue;
            }
            let book = book_entry.file_name().to_string_lossy().to_string();
            let kind_path = book_path.join(dir);
            let Ok(files) = std::fs::read_dir(&kind_path) else { continue };
            for file_entry in files.flatten() {
                let path = file_entry.path();
                if path.extension().and_then(|e| e.to_str()) != Some("json") {
                    continue;
                }
                let Ok(raw) = std::fs::read_to_string(&path) else { continue };
                let Ok(v) = serde_json::from_str::<Value>(&raw) else { continue };
                let data = &v["data"];
                let Some(key) = data["key"].as_str() else { continue };
                let name = data["name"].as_str().unwrap_or(key).to_string();
                let description = data["description"].as_str().map(str::to_string);
                let raw_token_count = data["raw_tokens"].as_array().map(|a| a.len()).unwrap_or(0);
                let source_path = v["source"]["path"].as_str().unwrap_or_default().to_string();
                let source_line = v["source"]["line"].as_u64().unwrap_or(0);
                if let Some(coordinate) = v["rename"]["coordinate"].as_str() {
                    by_coordinate.insert(coordinate.to_string(), (book.clone(), key.to_string()));
                }
                records.insert(
                    (book.clone(), key.to_string()),
                    SimpleKindRecord {
                        book: book.clone(),
                        key: key.to_string(),
                        name,
                        source_path,
                        source_line,
                        description,
                        raw_token_count,
                    },
                );
            }
        }
    }
    SimpleKindTable { kind: kind.to_string(), dir, records, by_coordinate }
}

/// One transcript line per kind for `--epic2-table-transcript`: the table's
/// location, its record count, and either the sample key's real record or
/// an explicit `REFUSED` when it is absent -- the AT-34-E2-001 evidence
/// format (`epic-breakdown.md`: "the table's location and a transcript of
/// it holding a named record").
pub fn transcript_line(table: &SimpleKindTable, sample_book: &str, sample_key: &str) -> String {
    match table.resolve(sample_book, sample_key) {
        Some(r) => format!(
            "kind={} location=data/corpus/*/{}/*.json records={} sample=({sample_book}, {sample_key:?}) -> HELD name={:?} source={}:{} raw_tokens={}",
            table.kind, table.dir, table.len(), r.name, r.source_path, r.source_line, r.raw_token_count
        ),
        None => format!(
            "kind={} location=data/corpus/*/{}/*.json records={} sample=({sample_book}, {sample_key:?}) -> REFUSED (absent key)",
            table.kind, table.dir, table.len()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    /// `trait`'s directory is `trait_generic`, not `trait` -- confirming
    /// `AT-34-E2-001`'s own header comment holds against the live file
    /// tree, not just prose.
    #[test]
    fn trait_kind_resolves_to_the_trait_generic_directory() {
        assert_eq!(kind_dir_for("trait"), Some("trait_generic"));
        assert_ne!(kind_dir_for("trait"), Some("trait"));
    }

    #[test]
    fn an_unknown_kind_yields_an_empty_table_not_a_panic() {
        let table = load_simple_kind_table(&repo_root(), "power");
        assert!(table.is_empty(), "power is Epic 5's, not Epic 2's -- must not silently resolve here");
    }

    /// AT-34-E2-002's RED half: an absent key is refused, never fabricated.
    #[test]
    fn an_absent_key_is_refused_not_fabricated() {
        let table = load_simple_kind_table(&repo_root(), "ability");
        let refusal = table.resolve("advanced_class_guide", "This Key Does Not Exist In The Corpus");
        assert!(refusal.is_none(), "a key no corpus record carries must never resolve to a fabricated entry");
    }

    macro_rules! kind_holds_named_record {
        ($test_name:ident, $kind:literal, $book:literal, $key:literal) => {
            #[test]
            fn $test_name() {
                let table = load_simple_kind_table(&repo_root(), $kind);
                assert!(!table.is_empty(), "{} table loaded zero records from {:?}", $kind, table.dir);
                let record = table
                    .resolve($book, $key)
                    .unwrap_or_else(|| panic!("{}: expected {:?} in book {:?} to be HELD", $kind, $key, $book));
                assert_eq!(record.book, $book);
                assert_eq!(record.key, $key);

                // AT-34-E2-002's GREEN half's mirror: a genuinely absent key
                // in the SAME table is refused, not fabricated.
                let refusal = table.resolve($book, "___a_key_no_corpus_record_carries___");
                assert!(refusal.is_none(), "{}: a fabricated key must never resolve", $kind);
            }
        };
    }

    kind_holds_named_record!(
        ability_table_holds_aberrant_bloodline,
        "ability",
        "advanced_class_guide",
        "Aberrant Bloodline"
    );
    kind_holds_named_record!(
        template_table_holds_arcanist_spellbook,
        "template",
        "advanced_class_guide",
        "Arcanist SpellBook"
    );
    kind_holds_named_record!(trait_table_holds_trait_adopted, "trait", "advanced_players_guide", "Trait ~ Adopted");
    kind_holds_named_record!(
        deity_table_holds_a_pi_masked_codex_named_record,
        "deity",
        "bestiary_6",
        "Codex-Named Unit (deity_bestiary_6_b6_deities_lst_21)"
    );
    kind_holds_named_record!(domain_table_holds_battle_spirit, "domain", "advanced_class_guide", "Battle (Spirit)");
    kind_holds_named_record!(skill_table_holds_craft_rope, "skill", "bestiary_2", "Craft (Rope)");
    kind_holds_named_record!(language_table_holds_xenophobic, "language", "advanced_race_guide", "Xenophobic");

    /// `AT-34-E3-001` (`domain`, 1 unit): `Death (Pharasma)` at
    /// `cr_domains.lst:46` is PI-redacted at ingestion (the domain's own
    /// name embeds the deity `Pharasma`) -- its corpus record's `key`/`name`
    /// are rewritten to `Codex-Named Unit (...)`, so a plain `resolve` by
    /// the record's REAL corpus name (what `docs/work-inventory.json`'s
    /// unit carries as `corpus_key`) never finds it, even though the
    /// record physically exists. `resolve_by_coordinate` matches on the
    /// record's own stored `(book, source_file, source_line)` instead --
    /// never reading, logging, or reconstructing the redacted real name --
    /// and returns the SAME masked-key record.
    #[test]
    fn domain_table_resolves_a_pi_renamed_record_by_coordinate_not_by_the_real_name() {
        let table = load_simple_kind_table(&repo_root(), "domain");
        assert!(
            table.resolve("core_rulebook", "Death (Pharasma)").is_none(),
            "the real corpus_key must NOT resolve directly -- the record's own JSON key is masked"
        );
        let record = table
            .resolve_by_coordinate("core_rulebook:cr_domains.lst:46")
            .unwrap_or_else(|| panic!("domain: expected cr_domains.lst:46 to resolve by coordinate"));
        assert_eq!(record.book, "core_rulebook");
        assert_eq!(record.source_path, "pathfinder/paizo/roleplaying_game/core_rulebook/cr_domains.lst");
        assert_eq!(record.source_line, 46);
        assert!(record.key.starts_with("Codex-Named Unit ("), "must keep the masked key, never the real name");

        // A coordinate no record carries is refused, never fabricated.
        assert!(table.resolve_by_coordinate("core_rulebook:cr_domains.lst:9999").is_none());
    }

    /// Every one of the seven directories the population claims to be
    /// non-empty (`decisions.md §3`'s per-kind population) really is, at
    /// HEAD -- not carried forward from an older bundle.
    #[test]
    fn every_seven_kind_table_is_non_empty_at_head() {
        for (kind, _dir) in SEVEN_KIND_DIRS {
            let table = load_simple_kind_table(&repo_root(), kind);
            assert!(!table.is_empty(), "{kind}: table loaded zero records -- directory resolution regressed");
        }
    }
}
