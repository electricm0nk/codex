//! Generic reference-library catalog: twelve corpus content-kind directories,
//! served from the converted rule package.
//!
//! # The shape this closes
//!
//! SD-32 row 19 cycle 4 built this module for the twelve corpus content-kind
//! directories -- `ability`, `class_generic`, `deity`, `domain`,
//! `feat_generic`, `language`, `monster_generic`, `power`, `race_generic`,
//! `skill`, `template`, `trait_generic` -- that had a `CORPUS_KIND_NAMES`
//! entry so the census could classify them but no `reach_gate.rs` dispatch
//! arm at all. Per `decisions.md §17` it is **one generic mechanism** across
//! every book, not twelve catalogs and not per-book work.
//!
//! # What changed here (SD-35 `AT-35-E6-003`)
//!
//! It used to resolve each record's description by reading the **ingest
//! format** at run time: the description token buried in the record's stored
//! token array, a substitution pass over that token's argument tail, and --
//! for the many records the source ships as a bare mechanical row -- a
//! printed dump of the record's own token lines, key head and all (a key
//! ability abbreviation, a size letter, a domain list with a prerequisite
//! clause glued to it). That is a live-side read of the ingest format, which
//! `decisions.md §11` rules out, and ingest-format vocabulary on a player's
//! screen, which `decisions.md §1` rules out: the sheet shows a final number,
//! dice in final form, or the rule's words.
//!
//! Both already happen at ingest. The converter
//! (`src/pcgen_import/sheet_rule/`) performs the `%N` substitution and writes
//! the record's words as [`codex::rules_core::sheet_rule::SheetRule::prose`]
//! with typed slots, and writes the mechanical row's facts into the rule's
//! **typed fields** -- value, the second numbers beside it, the sheet total
//! it feeds and its stacking type, tags, the condition it applies under, the
//! choice it offers, what holding it does to the fact set, who hands it out.
//!
//! # What counts as a served entry
//!
//! Unchanged, and deliberately: **every** record under
//! `data/corpus/<book>/<kind_dir>/**/*.json` becomes exactly one entry, keyed
//! by its own `data.key` -- the exact raw field
//! `reach_gate.rs::corpus_record_keys` reads as its denominator, so the two
//! can never drift out of step. Identity (`key`, `name`) still comes from the
//! corpus record, because neither is a rule: they are the identity of the row
//! the screen is a catalog of. Only the **description** moved, and it now
//! comes from the package, joined to the record by the source row **both
//! sides record** (`source.path:line` against the rule's
//! `provenance.closure_rows[0]`). The join is checked, not assumed, by
//! [`tests::every_record_joins_a_converted_rule_on_its_own_source_row`].
//!
//! Description resolution is [`catalog_description_or_fields`]'s three tiers:
//!
//! 1. **Prose** -- the record's own authored words (`Desc`/`Benefit`/
//!    `Special`).
//! 2. **Stat block** -- the record's structured lines (a casting time, an
//!    advancement aspect): real content the source authored, but not a
//!    description.
//! 3. **Fields** -- [`codex::rules_core::sheet_rule_catalog::catalog_field_summary`],
//!    the rule's typed fields as words, for a record with no prose at all.
//!    This is the tier that replaces the token dump, and it is rendered
//!    through the **same vocabulary** a prerequisite line prints with.
//!
//! A record resolving to tier 1 is `Surfaced`-grade content, with
//! [`ReferenceLibraryEntryDto::is_mechanical_summary`] `false`. Tiers 2 and 3
//! set it `true`: both are real and neither is fabricated, but they are not
//! the record's authored words, and `reach_gate.rs::assess()` is entitled to
//! tell them apart. A record reaching none of the three is served with
//! `description: None` -- identity only, never dropped from the response.
//!
//! # PI screening
//!
//! Discharged upstream, twice over: by the ingest tools that wrote the corpus
//! records, and by the converter, which records its own product-identity
//! handling in each rule's `provenance`. This module reads only already-
//! screened converter output and re-runs no PI check of its own.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_json::Value;

use codex::rules_core::corpus_loader::live_sheet_rules;
use codex::rules_core::sheet_rule::SheetRulePackage;
use codex::rules_core::sheet_rule_catalog::{catalog_description_or_fields, DescriptionTier};

/// The twelve corpus content-kind directories with no reach mechanism, per
/// `reach_gate.rs::CORPUS_KIND_NAMES`'s own comment. Kept as the singular
/// directory name (the form `data/corpus/<book>/<dir>/` uses) — the plural
/// kind name a `Family` carries is `reach_gate.rs`'s own concern.
pub const REFERENCE_LIBRARY_KIND_DIRS: &[&str] = &[
    "ability",
    "class_generic",
    "deity",
    "domain",
    "feat_generic",
    "language",
    "monster_generic",
    "power",
    "race_generic",
    "skill",
    "template",
    "trait_generic",
];

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceLibraryEntryDto {
    /// The corpus `data.key` field, verbatim — the same raw identity
    /// `reach_gate.rs::corpus_record_keys` reads as its denominator.
    pub key: String,
    pub name: String,
    /// `None` only for a record whose converted rule states nothing beyond
    /// its own identity — no prose, no stat block, and no typed field.
    pub description: Option<String>,
    /// `true` when `description` came from the rule's stat-block lines or its
    /// typed fields rather than the record's own authored prose.
    pub is_mechanical_summary: bool,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn json_files_under(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(json_files_under(&path));
        } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
            out.push(path);
        }
    }
    out
}

/// `source path:line` -> the converted rule ids whose closure starts at that
/// row, in package order. The same join `intelligent_item_catalog.rs` uses.
///
/// Deliberately NOT narrowed to the corpus directory's own name: the
/// converter's `kind` is its own vocabulary and does not always equal the
/// directory a record sits in (`trait_generic/` records convert to
/// `race_trait` rules). A source row is one record's line in one file, so the
/// row itself is the exact identity and the narrowing would only drop real
/// joins -- which is precisely what it did, on every `trait_generic` record,
/// before `every_record_joins_a_converted_rule_on_its_own_source_row` caught
/// it.
fn rules_by_source_row(package: &SheetRulePackage) -> std::collections::BTreeMap<String, Vec<String>> {
    let mut out: std::collections::BTreeMap<String, Vec<String>> = std::collections::BTreeMap::new();
    for (id, rule) in &package.rules {
        if let Some(row) = rule.provenance.closure_rows.first() {
            out.entry(row.clone()).or_default().push(id.clone());
        }
    }
    out
}

/// `source path:line` -> every rule whose closure read that row **anywhere**,
/// not only as its first row.
///
/// One corpus record can be absorbed into a second rule's closure -- a
/// dependency row that a class feature's closure also reads, say -- and when
/// that happens the record's whole content lands on the absorbing rule while
/// its own rule keeps only the identity. Used strictly as a fallback, so a
/// record that describes itself is never described by something else, and
/// keyed on the record's own source row, so the fallback can only ever reach
/// a rule that genuinely read that row.
fn absorbing_rules_by_source_row(
    package: &SheetRulePackage,
) -> std::collections::BTreeMap<String, Vec<String>> {
    let mut out: std::collections::BTreeMap<String, Vec<String>> = std::collections::BTreeMap::new();
    for (id, rule) in &package.rules {
        for row in rule.provenance.closure_rows.iter().skip(1) {
            out.entry(row.clone()).or_default().push(id.clone());
        }
    }
    out
}

/// The best description across every rule converted from one source row:
/// authored prose if any rule has it, then a stat block, then the typed
/// fields. Ordering by tier rather than by package order keeps a bookkeeping
/// sibling rule from masking the row's real words.
fn describe(
    package: &SheetRulePackage,
    ids: &[String],
) -> Option<(String, bool)> {
    let mut best: Option<(DescriptionTier, String)> = None;
    for id in ids {
        let Some(rule) = package.rules.get(id) else { continue };
        let Some(resolved) = catalog_description_or_fields(package, rule) else { continue };
        let better = match &best {
            None => true,
            Some((tier, _)) => tier_rank(resolved.tier) < tier_rank(*tier),
        };
        if better {
            best = Some((resolved.tier, resolved.text));
        }
    }
    best.map(|(tier, text)| (text, tier != DescriptionTier::Prose))
}

fn tier_rank(tier: DescriptionTier) -> u8 {
    match tier {
        DescriptionTier::Prose => 0,
        DescriptionTier::StatBlock => 1,
        DescriptionTier::Fields => 2,
    }
}

/// Read every record's entry from one exact `<book>/<kind_dir>` directory.
/// Every ingested file becomes an entry — none are dropped — so this is
/// always safe to use as the `with_payload`/`identity_only`/`missing` source
/// for `reach_gate.rs::assess()` against `corpus_record_keys`' own
/// denominator.
pub fn load_reference_library_entries(
    repo_root: &Path,
    book_dir: &str,
    kind_dir: &str,
) -> Vec<ReferenceLibraryEntryDto> {
    let package: Option<&'static SheetRulePackage> = live_sheet_rules();
    let index = package.map(rules_by_source_row);
    let absorbed = package.map(absorbing_rules_by_source_row);
    let dir = repo_root.join("data/corpus").join(book_dir).join(kind_dir);
    let mut files = json_files_under(&dir);
    files.sort();
    let mut out = Vec::new();
    for file in files {
        let Ok(text) = fs::read_to_string(&file) else { continue };
        let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
        let data = &doc["data"];
        let Some(key) = data.get("key").and_then(Value::as_str) else { continue };
        let name = data.get("name").and_then(Value::as_str).unwrap_or(key).to_string();
        let source = &doc["source"];
        let resolved = match (package, &index, &absorbed) {
            (Some(package), Some(index), Some(absorbed)) => {
                match (source["path"].as_str(), source["line"].as_i64()) {
                    (Some(path), Some(line)) => {
                        let row = format!("{path}:{line}");
                        index
                            .get(&row)
                            .and_then(|ids| describe(package, ids))
                            // Only consulted when the record's own rule said nothing.
                            .or_else(|| {
                                absorbed.get(&row).and_then(|ids| describe(package, ids))
                            })
                    }
                    _ => None,
                }
            }
            _ => None,
        };
        let (description, is_mechanical_summary) = match resolved {
            Some((text, mechanical)) => (Some(text), mechanical),
            None => (None, false),
        };
        out.push(ReferenceLibraryEntryDto {
            key: key.to_string(),
            name,
            description,
            is_mechanical_summary,
        });
    }
    out
}

/// Convenience wrapper for production call sites, which always want the real
/// repo root.
pub fn load_reference_library_entries_prod(
    book_dir: &str,
    kind_dir: &str,
) -> Vec<ReferenceLibraryEntryDto> {
    load_reference_library_entries(&repo_root(), book_dir, kind_dir)
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceLibraryCatalogResponse {
    pub book: String,
    pub kind_dir: String,
    pub entries: Vec<ReferenceLibraryEntryDto>,
}

/// The Tauri command a browsable "reference library" panel calls — the
/// player-facing surface this module's entries reach through. Unlike
/// `companion_pool_catalog.rs` (folded into an existing command's response
/// field), these twelve kinds had no existing consumer at all, so this is a
/// new, standalone, genuinely-invokable command rather than a field added to
/// one.
#[tauri::command]
pub fn list_reference_library_catalog(
    book: String,
    kind_dir: String,
) -> Result<ReferenceLibraryCatalogResponse, String> {
    if !REFERENCE_LIBRARY_KIND_DIRS.contains(&kind_dir.as_str()) {
        return Err(format!(
            "'{kind_dir}' is not a registered reference-library kind directory; expected one of \
             {REFERENCE_LIBRARY_KIND_DIRS:?}"
        ));
    }
    let entries = load_reference_library_entries_prod(&book, &kind_dir);
    Ok(ReferenceLibraryCatalogResponse { book, kind_dir, entries })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pkg() -> &'static SheetRulePackage {
        live_sheet_rules().expect(
            "data/sheet_rules/ must be present -- regenerate with \
             `cargo run --locked --bin sheet_rule_convert`",
        )
    }

    /// Tier 1, proven against the exact real record the old module doc cited:
    /// `power`'s `description` field does not exist at all, and the words
    /// used to be read out of a `DESC` raw token at run time. The converter
    /// hoists them, so the package holds the same sentence.
    #[test]
    fn a_record_whose_words_lived_in_a_desc_token_is_served_from_the_package() {
        let repo = repo_root();
        let path = repo.join("data/corpus/ultimate_psionics/power/control_object.json");
        assert!(path.exists(), "fixture record moved or was renamed: {}", path.display());
        let entries = load_reference_library_entries(&repo, "ultimate_psionics", "power");
        let found = entries
            .iter()
            .find(|e| e.key == "Control Object")
            .expect("Control Object must be served by the reference-library catalog");
        assert_eq!(found.description.as_deref(), Some("Telekinetically animate a small object."));
        assert!(!found.is_mechanical_summary, "the record's own words are prose, not a summary");
    }

    /// Tier 3, proven against a real `skill` record — this kind never carries
    /// a `description` field or a `DESC` raw token at all. The screen used to
    /// print the record's token rows, key heads and all; it now prints the
    /// typed fields as words.
    #[test]
    fn a_record_with_no_prose_anywhere_is_served_from_its_typed_fields_as_words() {
        let repo = repo_root();
        let path = repo.join("data/corpus/inner_sea_bestiary/skill/perception_dim_light.json");
        assert!(path.exists(), "fixture record moved or was renamed: {}", path.display());
        let entries = load_reference_library_entries(&repo, "inner_sea_bestiary", "skill");
        let found = entries
            .iter()
            .find(|e| e.key == "Perception (Dim Light)")
            .expect("Perception (Dim Light) must be served by the reference-library catalog");
        let desc = found.description.as_deref().expect("must fall through to the field summary");
        assert!(found.is_mechanical_summary);
        assert!(
            ingest_vocabulary(desc).is_none(),
            "the ingest format's own token head must never reach the screen: {desc}"
        );
        assert!(
            desc.contains("Wisdom") || desc.contains("Applies if") || desc.contains("Tags:"),
            "the field summary must actually say something about the rule: {desc}"
        );
    }

    /// The ingest format's own vocabulary, detected by **shape**: an all-capitals
    /// run of four or more letters immediately followed by `:` or `=` is a token
    /// head, and a `%` followed by a digit or a capital is a substitution marker.
    /// Deliberately not a list of literal token heads -- such a list in this crate
    /// is itself a live-side occurrence of the ingest format, which
    /// `scripts/pcgen_residue_gate.py` counts and `decisions.md §11` forbids.
    fn ingest_vocabulary(text: &str) -> Option<String> {
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0usize;
        while i < chars.len() {
            if chars[i] == '%'
                && chars.get(i + 1).is_some_and(|c| c.is_ascii_digit() || c.is_ascii_uppercase())
            {
                return Some(chars[i..(i + 2).min(chars.len())].iter().collect());
            }
            if chars[i].is_ascii_uppercase() {
                let start = i;
                while i < chars.len() && chars[i].is_ascii_uppercase() {
                    i += 1;
                }
                if i - start >= 4 && matches!(chars.get(i), Some(':') | Some('=')) {
                    return Some(chars[start..=i].iter().collect());
                }
                continue;
            }
            i += 1;
        }
        None
    }

    /// The regression the whole swap exists to prevent: no served description
    /// carries ingest-format vocabulary, across every one of the twelve kinds
    /// in a book that has them. Over the live corpus directory, never a
    /// fixture (`decisions.md §4`).
    #[test]
    fn no_served_description_carries_ingest_format_vocabulary() {
        let repo = repo_root();
        let mut checked = 0usize;
        let mut described = 0usize;
        for kind in REFERENCE_LIBRARY_KIND_DIRS {
            for book in ["ultimate_psionics", "inner_sea_bestiary", "core_rulebook", "beastiary"] {
                for entry in load_reference_library_entries(&repo, book, kind) {
                    checked += 1;
                    let Some(desc) = &entry.description else { continue };
                    described += 1;
                    assert!(
                        ingest_vocabulary(desc).is_none(),
                        "{book}/{kind} {}: ingest vocabulary `{}` reached the screen: {desc}",
                        entry.key,
                        ingest_vocabulary(desc).unwrap_or_default()
                    );
                }
            }
        }
        println!("reference-library swap gate: entries={checked} described={described}");
        assert!(described > 0, "no entry carried a description -- this gate is measuring nothing");
    }

    /// A record whose converted rule states nothing beyond its identity is
    /// served bare, never dropped from the response.
    ///
    /// The fixture is a real size-bookkeeping template row: the source states
    /// its size and hides it from every screen, and the converter reads both
    /// as metadata, so the package holds the row's identity and nothing else.
    /// That is the honest answer, and it is where the remaining
    /// reference-library gap lives — see the cycle receipt's named remainder.
    #[test]
    fn a_record_with_nothing_at_all_is_served_bare_not_dropped() {
        let repo = repo_root();
        let path = repo.join("data/corpus/occult_adventures/template/medium.json");
        assert!(path.exists(), "fixture record moved or was renamed: {}", path.display());
        let entries = load_reference_library_entries(&repo, "occult_adventures", "template");
        let found = entries
            .iter()
            .find(|e| e.key == "MEDIUM")
            .expect("a truly bare record must still appear in the response, by identity only");
        assert!(found.description.is_none(), "got: {:?}", found.description);
    }

    /// Every entry loaded for a directory carries the SAME key set
    /// `reach_gate.rs::corpus_record_keys` would compute for it — the
    /// property `reach_gate.rs`'s `assess()` call site depends on to never
    /// see a spurious "missing" record.
    #[test]
    fn every_record_under_a_directory_becomes_exactly_one_entry() {
        let repo = repo_root();
        let dir = repo.join("data/corpus/ultimate_psionics/power");
        let on_disk = json_files_under(&dir).len();
        let entries = load_reference_library_entries(&repo, "ultimate_psionics", "power");
        assert_eq!(on_disk, entries.len(), "every JSON file under the directory must become one entry");
    }

    /// The join the swap stands on, checked rather than assumed, corpus-wide
    /// over all twelve kinds and every book: every reference-library record
    /// resolves to at least one converted rule on its own source row. A miss
    /// would be a record silently losing its description.
    #[test]
    fn every_record_joins_a_converted_rule_on_its_own_source_row() {
        let repo = repo_root();
        let package = pkg();
        let corpus = repo.join("data/corpus");
        let mut books: Vec<PathBuf> = fs::read_dir(&corpus)
            .expect("data/corpus must exist")
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect();
        books.sort();
        let mut total = 0usize;
        let mut misses: Vec<String> = Vec::new();
        let index = rules_by_source_row(package);
        for kind in REFERENCE_LIBRARY_KIND_DIRS {
            for book in &books {
                let dir = book.join(kind);
                if !dir.is_dir() {
                    continue;
                }
                for file in json_files_under(&dir) {
                    let Ok(text) = fs::read_to_string(&file) else { continue };
                    let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
                    let Some(key) = doc["data"]["key"].as_str() else { continue };
                    total += 1;
                    let (Some(path), Some(line)) =
                        (doc["source"]["path"].as_str(), doc["source"]["line"].as_i64())
                    else {
                        misses.push(key.to_string());
                        continue;
                    };
                    if !index.contains_key(&format!("{path}:{line}")) && misses.len() < 10 {
                        misses.push(key.to_string());
                    }
                }
            }
        }
        println!("reference-library join: records={total} misses={}", misses.len());
        assert!(total > 9_000, "the reference-library population collapsed to {total}");
        assert!(misses.is_empty(), "records with no converted rule on their own source row: {misses:?}");
    }
}
