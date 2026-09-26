//! The sheet-rule converter (SD-35 AT-35-E2-001, `technical-design.md` §1).
//!
//! Run once at ingest time over every unit of `docs/work-inventory.json` (49,450 -- the
//! population every SD-35 figure states as its denominator), each joined to its shipped
//! `data/corpus` record by (book, source basename, source line) then (book, kind, key), and
//! write OUR rule record to `data/sheet_rules/<book>/<kind>/<key>.json`. This module is the
//! only place the sheet rule's source-format reading happens; it ships in no desktop binary and
//! runs at no run time. Its mapping rows are transcribed from
//! `artifacts/epic-2-sheet-rule/token-mapping/mapping-table.v1.json` (`table.rs`).
//!
//! Output layout (all `GENERATED FILE`s, regenerated whole, never hand-edited):
//!
//! - `<book>/<kind>/<key>.json` -- a JSON array of `SheetRule` for one record;
//! - `_vars/<VarId>.json` -- one `VarTable` per corpus variable some rule references;
//! - `_refused.json` -- the refusal report, per token type (never per unit);
//! - `_tokens.json` -- the token census (SD-35 AT-35-E2-004): per record, the mapping-table
//!   row key of every token its closure carried, and per refusal shape the token type it arose
//!   under -- what `scripts/token_coverage.py` counts from;
//! - `_defects/<kind>.json` -- converter defect lists (unresolved references, undefined
//!   variables, choice markers without a choice, grants by type);
//! - `_report.json` -- the run summary (`records=... converted=... refused=...`).
//!
//! Token types in `_refused.json`, `_tokens.json`, `_defects/` and `_report.json` are written with their `:`
//! JSON-escaped (`:`) so the source-format literal never appears in the data package
//! (`grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/` is 0); a JSON reader
//! sees the real string. The source-name -> `VarId` map goes to
//! `scripts/oracle_harness/var_names.json` (tool side, for the oracle harness only).

pub mod always_held;
pub mod attest;
pub mod closure;
pub mod convert;
pub mod ctx;
pub mod formula;
pub mod natural_attack;
pub mod oracle_terms;
pub mod pool_link;
pub mod pool_option;
pub mod pool_pick;
pub mod prereq;
pub mod prose;
pub mod reprint;
pub mod subclass;
pub mod table;
pub mod weapon_membership;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::pcgen_import::ingest_record;
use codex::rules_core::sheet_rule::*;
use closure::{Closure, PinnedTree, RowRef};
use ctx::{slug, CorpusIndex, OwnContribution, RecordRef};

/// `PCGEN_ORACLE_SHA` from `scripts/pcgen-oracle-pin.env`, or `"unpinned"`.
pub fn oracle_pin() -> String {
    static PIN: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    PIN.get_or_init(|| {
        let repo = repo_root();
        let text = std::fs::read_to_string(repo.join("scripts/pcgen-oracle-pin.env")).unwrap_or_default();
        text.lines()
            .find_map(|l| l.strip_prefix("PCGEN_ORACLE_SHA=").map(|s| s.split_whitespace().next().unwrap_or("").to_string()))
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "unpinned".into())
    })
    .clone()
}

pub fn repo_root() -> PathBuf {
    crate::repo_root()
}

// ---- population -------------------------------------------------------------------------------

#[derive(Deserialize)]
struct InventoryFile {
    units: Vec<InventoryUnit>,
}

#[derive(Deserialize, Clone)]
struct InventoryUnit {
    id: String,
    book: String,
    kind: String,
    name: String,
    corpus_key: Option<String>,
    source_file: Option<String>,
    source_line: Option<usize>,
    type_facet: Option<String>,
}

#[derive(Deserialize)]
struct CorpusRecord {
    #[serde(default)]
    data: serde_json::Value,
    #[serde(default)]
    license: Option<String>,
    #[serde(default)]
    pi_field: Option<String>,
    #[serde(default)]
    source: Option<serde_json::Value>,
}

struct CorpusEntry {
    path: PathBuf,
    kind: String,
    book: String,
    slug: String,
    basename: String,
    line: Option<usize>,
}

fn walk_corpus(repo: &Path) -> Vec<CorpusEntry> {
    let root = repo.join("data/corpus");
    let mut out = Vec::new();
    let Ok(books) = std::fs::read_dir(&root) else { return out };
    let mut book_dirs: Vec<PathBuf> = books.flatten().map(|e| e.path()).filter(|p| p.is_dir()).collect();
    book_dirs.sort();
    for bd in book_dirs {
        let book_dir_name = bd.file_name().unwrap().to_string_lossy().to_string();
        let book = if book_dir_name == "beastiary" { "bestiary".to_string() } else { book_dir_name.clone() };
        let mut stack = vec![bd.clone()];
        while let Some(d) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&d) else { continue };
            let mut items: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
            items.sort();
            for p in items {
                if p.is_dir() {
                    if p.file_name().is_some_and(|n| n == "_parity") {
                        continue;
                    }
                    stack.push(p);
                } else if p.extension().is_some_and(|e| e == "json") && p.file_name().is_some_and(|n| n != "LICENSE.json") {
                    let rel = p.strip_prefix(&bd).unwrap();
                    let kind = rel.components().next().map(|c| c.as_os_str().to_string_lossy().to_string()).unwrap_or_default();
                    let slug = p.file_stem().unwrap().to_string_lossy().to_string();
                    // Read only the `source` block here (cheap): basename + line for the join.
                    let (basename, line) = match std::fs::read_to_string(&p).ok().and_then(|t| serde_json::from_str::<CorpusRecord>(&t).ok()) {
                        Some(rec) => {
                            let src = rec.source.unwrap_or_default();
                            let path = src.get("path").and_then(|v| v.as_str()).unwrap_or("").to_string();
                            let basename = path.rsplit('/').next().unwrap_or("").to_string();
                            let line = src.get("line").and_then(|v| v.as_u64()).map(|v| v as usize);
                            (basename, line)
                        }
                        None => (String::new(), None),
                    };
                    out.push(CorpusEntry { path: p, kind, book: book.clone(), slug, basename, line });
                }
            }
        }
    }
    out
}

fn record_from_json(tree: &PinnedTree, unit: &InventoryUnit, path: &Path) -> Option<RecordRef> {
    let text = std::fs::read_to_string(path).ok()?;
    let rec: CorpusRecord = serde_json::from_str(&text).ok()?;
    let data = &rec.data;
    let src = rec.source.unwrap_or_default();
    let rel_path = src.get("path").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let line = src.get("line").and_then(|v| v.as_u64()).map(|v| v as usize).unwrap_or(0);
    let copy_base_key = src.get("inherited_from_record_key").and_then(|v| v.as_str()).map(|s| s.to_string());
    let key = data
        .get("key")
        .and_then(|v| v.as_str())
        .or_else(|| data.get("class_id").and_then(|v| v.as_str()))
        .or_else(|| src.get("record_key").and_then(|v| v.as_str()))
        .map(|s| s.trim_start_matches("CLASS:").to_string())
        .unwrap_or_else(|| unit.corpus_key.clone().unwrap_or_else(|| unit.name.clone()));
    let mut name = data.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_else(|| unit.name.clone());
    // SD-36 Epic E CONV-04: a `.COPY=` row's OWN head names the BASE record it inherits from
    // (`Potion.COPY=Oil of Darkness`, `Longsword (Base).COPY=Longsword`) -- the corpus ingest's
    // `data.name` field for such a row carries that base/generic word, never the specific
    // item's own name, while `key` (read from the row's own `KEY:` token above) already carries
    // the specific name correctly in every such case. Detected from the row's own source text
    // (never inferred from shape alone), so a legitimate non-`.COPY=` divergence between `key`
    // and `name` -- e.g. `Repeating Heavy Crossbow (Base)` (key) vs `Repeating Heavy Crossbow`
    // (name), where `name` is the better display string -- is left untouched.
    if let Some(idx) = tree.file_index(&rel_path)
        && line > 0
        && let Some(head) = tree.files[idx].lines.get(line - 1).and_then(|l| l.split('\t').next())
        && head.contains(".COPY=")
    {
        name = key.clone();
    }
    let mut shipped_tokens: Option<Vec<(String, String)>> = data.get("raw_tokens").and_then(|v| v.as_array()).map(|arr| {
        arr.iter()
            .filter_map(|t| Some((t.get("key")?.as_str()?.to_string(), t.get("value")?.as_str()?.to_string())))
            .collect()
    });
    // The ingest stores a row's `BONUS:` clauses in a SECOND array, `raw_bonus_chains`, never
    // in `raw_tokens` (`pcgen_import::bonus_chain_reader`'s module doc states the split). A
    // record that ships tokens has its shipped list REPLACE the base row's
    // (`closure::PinnedTree::closure`), so reading `raw_tokens` alone dropped every one of
    // those clauses on the floor -- silently, because a token nobody reads is not a refusal.
    // Re-joining each chain into the `BONUS:<sub>|<target>|<value>|<extras>` value the mapping
    // table already addresses hands them back. Traversal via the converter's own named reader,
    // never open-coded here (AT-35-E6-002 cycle 4's standing correction).
    let chains: Vec<(String, String)> = ingest_record::bonus_chain_qualifiers(data)
        .into_iter()
        .filter(|q| !q.is_empty())
        .map(|q| ("BONUS".to_string(), q.join("|")))
        .collect();
    if !chains.is_empty() {
        shipped_tokens.get_or_insert_with(Vec::new).extend(chains);
    }
    let shipped_tokens = shipped_tokens;
    let prerequisites: Vec<String> = data
        .get("prerequisites")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();
    let category = shipped_tokens
        .as_ref()
        .and_then(|t| t.iter().find(|(k, _)| k == "CATEGORY").map(|(_, v)| v.clone()))
        .unwrap_or_else(|| match unit.kind.as_str() {
            "feat" => "FEAT".to_string(),
            _ => String::new(),
        });
    let type_facet = shipped_tokens
        .as_ref()
        .and_then(|t| t.iter().find(|(k, _)| k == "TYPE").map(|(_, v)| v.clone()))
        .or_else(|| unit.type_facet.clone())
        .unwrap_or_default();
    let pi_fields: Vec<String> = rec.pi_field.as_deref().unwrap_or("").split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    let class_name = data.get("class").and_then(|v| v.as_str()).map(|s| s.to_string());
    Some(RecordRef {
        id: unit.id.clone(),
        book: unit.book.clone(),
        kind: unit.kind.clone(),
        name,
        key,
        category,
        type_facet,
        rel_path,
        line,
        shipped_tokens,
        prerequisites,
        copy_base_key,
        license_pi: rec.license.as_deref() == Some("PI-REDACTED"),
        pi_fields,
        description: data.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()),
        class_name,
        class_selection_of: None,
        joined: true,
    })
}

/// Locate an inventory unit's own source row in the pinned tree by the coordinates the
/// inventory already carries: `(book, source_file basename, source_line)`.
///
/// SD-35 `AT-35-E3-002`. Used only when the `data/corpus` join found no record at all. Two
/// units in the shipped corpus are in that state (`book_of_the_damned_volume_2:spell:summon_
/// demons_nascent_demon_lord`, `ultimate_combat:spell:share_language_communal`): the corpus
/// ingest wrote no JSON for them, but the row they name is present in the pinned tree with a
/// real `description` head. Under the sheet rule (`decisions.md §1` form 3) a record whose
/// words exist prints those words, so resolving the row here is what turns a refusal with a
/// blank sheet line into a printed rule. Never invents a row: the file must sit in the unit's
/// own book directory, outside `_pfs/`, and the line must exist in it.
fn source_row_in_tree(tree: &PinnedTree, unit: &InventoryUnit) -> Option<(String, usize)> {
    let basename = unit.source_file.as_deref()?;
    let line = unit.source_line?;
    if line == 0 {
        return None;
    }
    let file = tree
        .files
        .iter()
        .find(|f| f.book == unit.book && !f.is_pfs && f.rel_path.rsplit('/').next() == Some(basename))?;
    if line > file.lines.len() {
        return None;
    }
    Some((file.rel_path.clone(), line))
}

/// The population: every inventory unit, joined to its corpus record where one exists, else to
/// its own source row in the pinned tree ([`source_row_in_tree`]). A unit that resolves to
/// neither comes back with an empty `rel_path` and is refused as `no_corpus_record`.
///
/// # The cross-book source-row fallback (SD-35 `AT-35-E7-CLOSURE-CLEANUP`)
///
/// The first two lookups are both keyed on the unit's OWN `book`. That is wrong for a reprint:
/// PCGen files a shared row once, in the directory of whichever book physically carries the
/// `.lst`, while every book that reprints it declares a unit of its own. `advanced_race_guide`
/// declares 33 races; only 12 race rows live in `data/corpus/advanced_race_guide/race/`, because
/// `elf` lives in `data/corpus/core_rulebook/race/elf.json` — the same record, filed under the
/// book that owns `elf_races.lst`. Keyed on `u.book`, both lookups miss, the unit is refused as
/// `no_corpus_record`, and **nothing renders for it at all** — the canonical printing is not
/// itself an inventory unit, so there is no other unit picking the rule up.
///
/// A corpus record is identified by the source ROW it was ingested from, not by the directory it
/// was filed under. So the third lookup drops the book and keys on `(source_file basename,
/// source_line)`, narrowed to the unit's `kind`, and is taken **only when it is unambiguous**.
/// The predicate is widened; no id is listed and no book is exempted (the `B18` precedent,
/// `decisions.md §21`).
///
/// Measured before it was trusted, over the whole population: 831 of 49,450 units miss the two
/// book-keyed lookups; this fallback resolves **exactly 142** of them — precisely the set that
/// was being refused — and **none** of the other 689, which keep resolving through
/// [`source_row_in_tree`] exactly as before. It therefore cannot silently re-join a unit that
/// was already converting.
pub fn load_population(repo: &Path, tree: &PinnedTree) -> Result<Vec<RecordRef>, String> {
    let inv_text = std::fs::read_to_string(repo.join("docs/work-inventory.json")).map_err(|e| format!("docs/work-inventory.json: {e}"))?;
    let inv: InventoryFile = serde_json::from_str(&inv_text).map_err(|e| format!("docs/work-inventory.json: {e}"))?;
    let entries = walk_corpus(repo);
    let mut by_line: BTreeMap<(String, String, usize), usize> = BTreeMap::new();
    let mut by_key: BTreeMap<(String, String, String), usize> = BTreeMap::new();
    // (basename, line, kind) -> every corpus record at that source row of that kind, in ANY
    // book's directory. A key with more than one entry is ambiguous and is never joined.
    let mut by_row_any_book: BTreeMap<(String, usize, String), Vec<usize>> = BTreeMap::new();
    for (i, e) in entries.iter().enumerate() {
        if let Some(l) = e.line
            && !e.basename.is_empty()
        {
            by_line.entry((e.book.clone(), e.basename.clone(), l)).or_insert(i);
            by_row_any_book.entry((e.basename.clone(), l, e.kind.clone())).or_default().push(i);
        }
        by_key.entry((e.book.clone(), e.kind.clone(), e.slug.clone())).or_insert(i);
    }
    let mut out = Vec::with_capacity(inv.units.len());
    let mut joined_entries: BTreeSet<usize> = BTreeSet::new();
    for u in &inv.units {
        let idx = u
            .source_file
            .as_ref()
            .zip(u.source_line)
            .and_then(|(f, l)| by_line.get(&(u.book.clone(), f.clone(), l)))
            .or_else(|| {
                let s = u.id.splitn(3, ':').nth(2).unwrap_or("").to_string();
                by_key.get(&(u.book.clone(), u.kind.clone(), s))
            })
            .or_else(|| {
                // Cross-book reprint: same source row, another book's directory. Unambiguous only.
                let (f, l) = (u.source_file.as_ref()?, u.source_line?);
                match by_row_any_book.get(&(f.clone(), l, u.kind.clone()))?.as_slice() {
                    [only] => Some(only),
                    _ => None,
                }
            })
            .copied();
        if let Some(i) = idx {
            joined_entries.insert(i);
        }
        let rec = idx.and_then(|i| record_from_json(tree, u, &entries[i].path));
        out.push(rec.unwrap_or_else(|| {
            let (rel_path, line) = source_row_in_tree(tree, u).unwrap_or_default();
            let joined = !rel_path.is_empty();
            RecordRef {
                id: u.id.clone(),
                book: u.book.clone(),
                kind: u.kind.clone(),
                name: u.name.clone(),
                key: u.corpus_key.clone().unwrap_or_default(),
                category: if u.kind == "feat" { "FEAT".to_string() } else { String::new() },
                type_facet: u.type_facet.clone().unwrap_or_default(),
                rel_path,
                line,
                shipped_tokens: None,
                prerequisites: Vec::new(),
                copy_base_key: None,
                license_pi: false,
                pi_fields: Vec::new(),
                description: None,
                class_name: None,
                class_selection_of: None,
                joined,
            }
        }));
    }
    mark_class_selections(&entries, &joined_entries, &mut out);
    Ok(out)
}

/// SD-36 F1c-3 (defect D3). A corpus CLASS record no inventory unit joined, which names a
/// `base_class_key` and whose `category` is `CLASS`, is a class-selection class: PCGen declares
/// no `CLASS:` object for it, only a `CATEGORY:CLASS` ability on the base class's line. The
/// inventory unit that joined the SAME source row (the ability) is marked with the base KEY, and
/// [`run`] writes the class principal it stands for ([`class_selection_principal`]). Only an
/// unambiguous row match is marked. Measured over the shipped corpus: exactly Pathfinder
/// Unchained's four (`data/corpus/pathfinder_unchained/class/*_unchained_class.json`).
fn mark_class_selections(entries: &[CorpusEntry], joined: &BTreeSet<usize>, records: &mut [RecordRef]) {
    for (i, e) in entries.iter().enumerate() {
        if e.kind != "class" || joined.contains(&i) {
            continue;
        }
        let Some(line) = e.line else { continue };
        let Some(rec) = std::fs::read_to_string(&e.path).ok().and_then(|t| serde_json::from_str::<CorpusRecord>(&t).ok()) else { continue };
        let is_class_category = rec.data.get("category").and_then(|v| v.as_str()).is_some_and(|c| c.eq_ignore_ascii_case("CLASS"));
        let Some(base) = rec.data.get("base_class_key").and_then(|v| v.as_str()).filter(|_| is_class_category) else { continue };
        let at_row: Vec<usize> = records
            .iter()
            .enumerate()
            .filter(|(_, r)| r.book == e.book && r.line == line && r.rel_path.rsplit('/').next() == Some(e.basename.as_str()))
            .map(|(j, _)| j)
            .collect();
        if let [only] = at_row.as_slice() {
            records[*only].class_selection_of = Some(base.to_string());
        }
    }
}

// ---- indexes ----------------------------------------------------------------------------------

/// The prefix of the codex-named placeholder a product-identity record's corpus key carries in
/// place of its real KEY (`Codex-Named Unit (<kind>_<book>_<file>_<line>)`).
const CODEX_PLACEHOLDER_KEY_PREFIX: &str = "Codex-Named Unit (";

fn is_codex_placeholder_key(key: &str) -> bool {
    key.starts_with(CODEX_PLACEHOLDER_KEY_PREFIX)
}

/// The KEY (upper-cased) the record's own base row declares in the pinned tree -- its `KEY:`
/// token, else its name field ([`closure::row_identity`]) -- or `None` when the record has no
/// row there or the row is not a record declaration (plain or `.COPY=`).
fn declared_row_key(tree: &PinnedTree, r: &RecordRef) -> Option<String> {
    let file = tree.file_index(&r.rel_path)?;
    if r.line == 0 || r.line > tree.files[file].lines.len() {
        return None;
    }
    let id = closure::row_identity(tree.row_text(RowRef { file, line: r.line }));
    (matches!(id.shape, closure::RowShape::Plain | closure::RowShape::Copy(_)) && !id.key.is_empty()).then_some(id.key)
}

/// Record both ids of a newly seen ambiguous pair (the first-indexed one once).
fn push_candidates(map: &mut BTreeMap<(String, String), Vec<RuleId>>, pair: &(String, String), existing: &RuleId, id: &RuleId) {
    let list = map.entry(pair.clone()).or_default();
    for candidate in [existing, id] {
        if !list.contains(candidate) {
            list.push(candidate.clone());
        }
    }
}

/// Build the corpus-wide index and every record's closure.
pub fn build_index(tree: &PinnedTree, records: Vec<RecordRef>) -> (CorpusIndex, Vec<Closure>) {
    let mut index = CorpusIndex::default();
    let mut closures: Vec<Closure> = Vec::with_capacity(records.len());
    let mut placeholder_declared: Vec<((String, String), RuleId)> = Vec::new();
    let mut row_declared_category: Vec<((String, String), RuleId, bool)> = Vec::new();
    for r in &records {
        let closure = if r.rel_path.is_empty() {
            Closure::default()
        } else {
            tree.closure(&r.rel_path, r.line, r.shipped_tokens.as_deref(), &r.category, &r.key, r.copy_base_key.as_deref())
        };
        let cat_u = r.category.to_ascii_uppercase();
        let key_u = r.key.to_ascii_uppercase();
        let name_u = r.name.to_ascii_uppercase();
        // First wins (unchanged): `by_cat_key`/`by_cat_name` keep resolving a direct-category
        // hit exactly as before. A pair claimed by more than one DIFFERENT record id is ALSO
        // recorded as ambiguous, so the parent-category retry (`resolve_rule_in_checked`) can
        // refuse to guess among them instead of silently inheriting whichever loaded first (F1
        // adversarial finding 4).
        let cat_key_pair = (cat_u.clone(), key_u.clone());
        match index.by_cat_key.get(&cat_key_pair) {
            Some(existing) if *existing != r.id => {
                push_candidates(&mut index.cat_key_candidates, &cat_key_pair, existing, &r.id);
                index.ambiguous_cat_key.insert(cat_key_pair);
            }
            Some(_) => {}
            None => {
                index.by_cat_key.insert(cat_key_pair, r.id.clone());
            }
        }
        let cat_name_pair = (cat_u.clone(), name_u.clone());
        match index.by_cat_name.get(&cat_name_pair) {
            Some(existing) if *existing != r.id => {
                push_candidates(&mut index.cat_name_candidates, &cat_name_pair, existing, &r.id);
                index.ambiguous_cat_name.insert(cat_name_pair);
            }
            Some(_) => {}
            None => {
                index.by_cat_name.insert(cat_name_pair, r.id.clone());
            }
        }
        // SD-36 F3b2: a product-identity record ships a codex-named placeholder corpus key
        // (`Codex-Named Unit (class_feature_..._lst_9)`), but every other record names it by the
        // KEY its own oracle row declares (`Aldori Swordlord ~ Adaptive Tactics`,
        // `ag_abilities_class.lst:9`). Indexed only under the placeholder, each such reference
        // missed and was written to `_defects/unresolved-references.json` although its target
        // converts. The declared key is collected here and indexed after every corpus key (below).
        if is_codex_placeholder_key(&r.key)
            && let Some(declared) = declared_row_key(tree, r)
            && declared != key_u
        {
            placeholder_declared.push(((cat_u.clone(), declared), r.id.clone()));
        }
        // SD-36 F3c4b: a record whose shipped tokens state no `CATEGORY:` (the corpus record
        // sits at a `.MOD` row, `CATEGORY=Internal|Bloodline Tracker.MOD`,
        // `cr_abilities_class.lst:1705`) is named by every other record under the category its
        // own source row declares (`ABILITY:Internal|AUTOMATIC|Bloodline Tracker`). Indexed
        // under it below, only where no record answers the pair (the F3b2 declared-key rule).
        if cat_u.is_empty()
            && let Some(i) = tree.file_index(&r.rel_path)
            && r.line > 0
        {
            let ident = closure::row_identity(tree.row_text(RowRef { file: i, line: r.line }));
            let is_mod = ident.shape == closure::RowShape::Mod;
            if !ident.category.is_empty() {
                row_declared_category.push(((ident.category.clone(), key_u.clone()), r.id.clone(), is_mod));
                row_declared_category.push(((ident.category, name_u.clone()), r.id.clone(), is_mod));
            }
        }
        index.by_kind_name.entry((r.kind.clone(), key_u.clone())).or_insert(r.id.clone());
        index.by_kind_name.entry((r.kind.clone(), name_u.clone())).or_insert(r.id.clone());
        if r.kind == "class" {
            let own = ctx::own_class_id(r);
            index.classes.entry(key_u.clone()).or_insert((r.id.clone(), own.clone()));
            index.classes.entry(name_u.clone()).or_insert((r.id.clone(), own.clone()));
            // The name the class's own base row declares (`CLASS:<name>`): other records name
            // the class by it, and a product-identity class's corpus key is a placeholder.
            if let Some(i) = tree.file_index(&r.rel_path)
                && r.line > 0
                && tree.files[i].lines.get(r.line - 1).is_some_and(|l| l.trim_start().to_ascii_uppercase().starts_with("CLASS:"))
            {
                let declared = closure::row_identity(tree.row_text(RowRef { file: i, line: r.line })).key;
                if !declared.is_empty() {
                    index.classes.entry(declared).or_insert((r.id.clone(), own));
                }
            }
        }
        if r.kind == "skill" {
            index.skills.entry(name_u.clone()).or_insert(slug(&r.name));
            index.skills.entry(key_u.clone()).or_insert(slug(&r.name));
        }
        let owning_class: Option<ClassId> = match r.kind.as_str() {
            "class" => Some(ctx::own_class_id(r)),
            _ => r.class_name.as_ref().map(|c| slug(c)),
        };
        for row in &closure.rows {
            if let Some(rr) = row.row
                && closure.own_rows.contains(&rr)
            {
                index.row_owner.entry(rr).or_insert(r.id.clone());
            }
            let own = row.row.is_some_and(|rr| closure.own_rows.contains(&rr));
            if !own {
                continue;
            }
            for (k, v) in &row.tokens {
                match k.as_str() {
                    "DEFINE" => {
                        if let Some((n, _)) = v.split_once('|') {
                            index.own_defines.entry(r.id.clone()).or_default().insert(n.trim().to_ascii_uppercase());
                        }
                    }
                    "BONUS" => {
                        if let Some(rest) = v.strip_prefix("VAR|") {
                            let (fields, gates) = ctx::split_gates(rest);
                            if fields.len() >= 2 {
                                let bonus_type = fields.iter().skip(2).find_map(|f| ctx::parse_bonus_type(f));
                                for n in fields[0].split(',') {
                                    index.own_var_contribs.entry(r.id.clone()).or_default().entry(n.trim().to_ascii_uppercase()).or_default().push(OwnContribution {
                                        formula: fields[1].clone(),
                                        bonus_type: bonus_type.clone(),
                                        gates: gates.clone(),
                                        level_gate: row.level_gate,
                                        owning_class: owning_class.clone(),
                                    });
                                }
                            }
                        }
                    }
                    "FACT" => {
                        if let Some((n, val)) = v.split_once('|') {
                            index.fact_declarers.entry((n.trim().to_ascii_uppercase(), val.trim().to_ascii_uppercase())).or_default().push(r.id.clone());
                        }
                    }
                    _ => {}
                }
            }
        }
        index.own_rows.insert(r.id.clone(), closure.own_rows.clone());
        index.facets.insert(r.id.clone(), convert::accumulated_facets(r, &closure));
        closures.push(closure);
    }
    // SD-36 F3b2: the declared keys of placeholder-keyed records, indexed only where no corpus
    // key already answers the pair -- a reference that resolved before keeps its target (a
    // reprint's real key wins over a PI twin's declared one). Two placeholder records declaring
    // one pair are ambiguous to each other, under the same rule as two corpus keys.
    let mut claimed: BTreeSet<(String, String)> = BTreeSet::new();
    for (pair, id) in placeholder_declared {
        match index.by_cat_key.get(&pair) {
            None => {
                claimed.insert(pair.clone());
                index.by_cat_key.insert(pair, id);
            }
            Some(existing) if *existing != id && claimed.contains(&pair) => {
                let existing = existing.clone();
                push_candidates(&mut index.cat_key_candidates, &pair, &existing, &id);
                index.ambiguous_cat_key.insert(pair);
            }
            Some(_) => {}
        }
    }
    // SD-36 F3c4b: the category a CATEGORY-less record's own source row declares. A pair
    // several such records claim, every one of them from a `.MOD` row, is ONE object whose `.MOD`
    // rows the inventory filed as one record per book (`Bloodline Tracker`: 8 books): PCGen
    // applies every `.MOD` row to the object its key names, so a grant of the object holds each
    // fragment (`CorpusIndex::mod_fragments`). Any other shared pair is left out (never guessed)
    // and named.
    let mut declared_claims: BTreeMap<(String, String), (BTreeSet<RuleId>, bool)> = BTreeMap::new();
    for (pair, id, is_mod) in row_declared_category {
        let e = declared_claims.entry(pair).or_insert_with(|| (BTreeSet::new(), true));
        e.0.insert(id);
        e.1 &= is_mod;
    }
    for (pair, (ids, all_mod)) in declared_claims {
        if index.by_cat_key.contains_key(&pair) || index.by_cat_name.contains_key(&pair) {
            continue;
        }
        if ids.len() == 1 {
            index.by_cat_key.insert(pair, ids.into_iter().next().unwrap_or_default());
        } else if all_mod {
            index.mod_fragments.insert(pair, ids.into_iter().collect());
        } else {
            index.index_defects.entry("row-declared-category-shared".into()).or_default().push(format!("{}|{}: {}", pair.0, pair.1, ids.into_iter().collect::<Vec<_>>().join(", ")));
        }
    }
    // SD-36 F3b2b: the standing supersession ruling, applied to every ambiguous pair.
    let by_id: BTreeMap<&str, &RecordRef> = records.iter().map(|r| (r.id.as_str(), r)).collect();
    index.reprint_newest_key = reprint::newest_printings(tree, &by_id, &index.cat_key_candidates);
    index.reprint_newest_name = reprint::newest_printings(tree, &by_id, &index.cat_name_candidates);
    // SD-36 F3c4b: ability-category pick rows no unit stands for become options of the choice
    // that picks them (`pool_option.rs`); their pairs are registered BEFORE any record converts,
    // so a record naming one resolves to it. Only pairs no unit answers are registered.
    let scan = {
        let answered = |cat: &str, key: &str| {
            let pair = (cat.to_string(), key.to_string());
            index.by_cat_key.contains_key(&pair) || index.by_cat_name.contains_key(&pair)
        };
        let owned = |row: RowRef| index.row_owner.contains_key(&row);
        pool_option::scan(tree, &records, &closures, &owned, &answered)
    };
    for d in &scan.options {
        for pair in [(d.category.clone(), d.key.clone()), (d.category.clone(), d.name.to_ascii_uppercase())] {
            let taken = index.by_cat_key.get(&pair).or_else(|| index.by_cat_name.get(&pair)).cloned();
            match taken {
                None => {
                    index.by_cat_key.insert(pair, d.id.clone());
                }
                Some(first) if first != d.id => {
                    index.index_defects.entry("pool-option-pair-shared".into()).or_default().push(format!("{}: {}|{} (resolves to {first})", d.id, pair.0, pair.1));
                }
                Some(_) => {}
            }
        }
    }
    for (k, v) in scan.defects {
        index.index_defects.entry(k).or_default().extend(v);
    }
    index.pool_options = scan.options;
    index.pool_option_choosers = scan.choosers;
    // SD-36 F3c5: `CATEGORY:Internal` natural-attack helper rows no unit or option stands for
    // (`natural_attack.rs`). Scanned after the options are registered, so a pair an option
    // answers is never claimed twice.
    let helpers = {
        let answered = |cat: &str, key: &str| {
            let pair = (cat.to_string(), key.to_string());
            index.by_cat_key.contains_key(&pair) || index.by_cat_name.contains_key(&pair)
        };
        let owned = |row: RowRef| index.row_owner.contains_key(&row);
        natural_attack::scan(tree, &owned, &answered)
    };
    for (k, v) in helpers.defects {
        index.index_defects.entry(k).or_default().extend(v);
    }
    index.natural_attack_helpers = helpers.helpers;
    index.records = records;
    index.filled_pools = pool_pick::filled_pools(tree, &index);
    (index, closures)
}

// ---- the run ----------------------------------------------------------------------------------

/// The token census (SD-35 AT-35-E2-004), written to `_tokens.json`: per record, the
/// mapping-table row key of every token its closure carried, and per refusal shape the token
/// type(s) it arose under. `scripts/token_coverage.py` counts "units carrying it" and "units
/// refused because of this token" from this -- the converter's own reading of the closure --
/// never from a second reading of the corpus. Tool side only: it names PCGen token types.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenCensus {
    pub schema: u32,
    pub entries: Vec<TokenCensusRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenCensusRecord {
    pub id: String,
    pub book: String,
    pub kind: String,
    /// Sorted, unique. Empty for a record whose closure has no row (`token-less`).
    pub tokens: Vec<String>,
    /// Refusal shape -> the token type(s) it arose under (`token-less` for `no_corpus_record`
    /// / `no_source_row`). Empty for a converted record.
    pub refusals: BTreeMap<String, Vec<String>>,
    /// Degradation shape -> the token type(s) it arose under (SD-35 AT-35-E3-001). The record
    /// DID convert; these are the terms whose number it could not write, so the rule prints
    /// its words. `scripts/token_coverage.py` reads `refusals` only, by design: a degraded
    /// record is converted, and this field is the ledger of what its line does not carry.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub degradations: BTreeMap<String, Vec<String>>,
}

/// The census key for a record refused before any token was read.
pub const TOKEN_LESS: &str = "token-less";

/// Everything one conversion pass produces, in memory.
pub struct Run {
    /// rule file path (relative to the output dir) -> the rules in it, sorted by id.
    pub files: BTreeMap<String, Vec<SheetRule>>,
    pub vars: BTreeMap<VarId, VarTable>,
    pub var_names: BTreeMap<VarId, String>,
    pub refused: RefusedReport,
    pub tokens: TokenCensus,
    pub defects: BTreeMap<String, Vec<String>>,
    pub report: Report,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Report {
    pub records: usize,
    pub converted: usize,
    pub refused: usize,
    pub rules_written: usize,
    pub var_tables: usize,
    pub by_kind: BTreeMap<String, KindCount>,
    pub refused_by_token_type: BTreeMap<String, usize>,
    /// SD-35 AT-35-E3-001: records that converted with at least one term degraded to words,
    /// counted per degradation shape. A record with k shapes counts under each.
    #[serde(default)]
    pub degraded_records: usize,
    #[serde(default)]
    pub degraded_by_token_type: BTreeMap<String, usize>,
    pub oracle_pin: String,
    pub converter_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KindCount {
    pub records: usize,
    pub converted: usize,
    pub refused: usize,
}

pub fn rule_file_rel(book: &str, kind: &str, id: &str) -> String {
    let s = id.splitn(3, ':').nth(2).unwrap_or(id);
    format!("{book}/{kind}/{s}.json")
}

/// SD-36 F1c-3 (defect D3): the class principal a class-selection record stands for.
///
/// PCGen declares no `CLASS:` object for a class-selection class: Pathfinder Unchained's four
/// are `CATEGORY:CLASS` abilities in the base class's `<Base> Class Selection` pool
/// (`pu_abilities_class.lst:114-117`), taken on the base class's own class line. The corpus
/// files them as classes (`data/corpus/<book>/class/`, `base_class_key` naming the base), the
/// inventory files each as a `class_feature` unit, and the converter used to write the ability
/// and no `class` record -- so `find("class", "unchained_monk")` found nothing.
///
/// One rule, read off the record itself ([`RecordRef::class_selection_of`]): the class
/// principal is `<book>:class:<slug(name)>`, carries [`Effect::TakenOnClass`] naming the base
/// class (every level in it is a level in the base class; the fixpoint then holds the base
/// class's own line exactly as PCGen does), the selection's tags, and a class-line grant
/// (`Granter::Class { <slug>, 1 }`) onto the selection ability, which carries everything the
/// oracle row states. It adds no inventory unit: the record count does not move.
///
/// `Err` names the base KEY when no converted class answers to it.
pub fn class_selection_principal(index: &CorpusIndex, record: &RecordRef, selection: &SheetRule) -> Result<(SheetRule, Grant), String> {
    let base_key = record.class_selection_of.as_deref().ok_or_else(|| "not a class-selection record".to_string())?;
    let (_, base) = index
        .classes
        .get(&base_key.to_ascii_uppercase())
        .ok_or_else(|| format!("{}: base class {base_key} has no converted class record", record.id))?;
    let class_slug = slug(&record.name);
    let id = format!("{}:class:{class_slug}", record.book);
    let principal = SheetRule {
        id,
        label: display_label(&record.name),
        value: SheetValue::Text,
        also: Vec::new(),
        prose: Vec::new(),
        applies: Applies::Always,
        target: None,
        bonus_type: None,
        print: true,
        pool: String::new(),
        tags: selection.tags.clone(),
        subject: Subject::Character,
        repeatable: false,
        granted_by: Vec::new(),
        offers: None,
        grants: vec![Effect::TakenOnClass(base.clone())],
        // The one row the class is declared on (the selection's own base row); the rows the
        // selection's closure reads stay on the selection.
        closure_complete: false,
        always_held: false,
        provenance: Provenance {
            kind: "class".into(),
            closure_rows: selection.provenance.closure_rows.iter().take(1).cloned().collect(),
            ..selection.provenance.clone()
        },
    };
    Ok((principal, Grant { by: Granter::Class { id: class_slug, at_level: 1 }, when: Applies::Always }))
}

/// Convert the whole population in memory.
/// The one rule a record with no source row still yields: its own words.
///
/// SD-35 `AT-35-E3-002`. Some shipped corpus records carry no PCGen row at all -- they were
/// ingested from a second source (`"source": {"kind": "web_second_source"}`) and hold a
/// `description` and nothing else. There is no closure to convert, so the token path refuses
/// them; under the sheet rule (`decisions.md §1` form 3) the words ARE the sheet line, so the
/// record converts to exactly one `Text` rule carrying them. Returns `None` -- and the caller
/// refuses as before -- when there is no description, when it is product identity
/// (`prose::pi_hit`, `§15` R2: the sheet must not print a redacted field), or when it would put
/// a source-format literal in the package (`FORBIDDEN_LITERALS`; the glyph scrub belongs to the
/// token path's `RecordCtx`, which this path has none of).
/// The record's own `description` field, decoded, when it is safe to print on a sheet.
///
/// The five refusal conditions are the ones [`description_only_rules`] has always applied, split
/// out so the token path can apply exactly the same bar (SD-35 `AT-35-E6-003-SWEEP` cycle 16).
/// They are: no description; it is product identity by term (`prose::pi_hit`), by record flag
/// (`license_pi`) or by declared field (`pi_fields`); it would put a source-format literal in
/// the package (`FORBIDDEN_LITERALS`); it carries a glued `PRE<KIND>:` head; or it carries a
/// `%1` slot with no argument row to fill it.
fn printable_description(r: &RecordRef) -> Option<String> {
    let text = prose::decode_entities(r.description.as_deref()?.trim());
    if text.is_empty() || prose::pi_hit(&text).is_some() || r.license_pi || r.pi_fields.iter().any(|f| f == "description") {
        return None;
    }
    if FORBIDDEN_LITERALS.iter().any(|lit| text.contains(lit)) || has_pre_head(&text) || text.contains("%1") {
        return None;
    }
    // The literal-percent escape — SD-35 `AT-35-E6-003-SWEEP` cycle 17.
    //
    // `%%` is how the source writes a literal `%`, and the `DESC:` path has always collapsed it
    // (`prose::template_pieces`), which is why a converted sentence states "roll d%". This
    // fallback took the corpus row verbatim and collapsed nothing, so
    // `core_rulebook:spell:plane_shift_to_shadow_or_material_plane` reached the Spell Catalog
    // screen reading "you appear 5 to 500 miles [5d%%] from your intended destination" — the
    // ingest format's own escape on a player's page. Four package files carried it.
    //
    // Collapsed here, the same way and to the same result as the other door, so the two doors
    // still apply one bar. Then refused if a `%` followed by a digit survives: that is a slot
    // marker, and this door has no argument row to fill one from. The old `text.contains("%1")`
    // check was the narrow form of that — it saw `%1` and not `%2`, and it ran before the
    // collapse, so `%%1` slipped past it and became `%1` on the page.
    let text = text.replace("%%", "%");
    let bytes = text.as_bytes();
    if bytes
        .iter()
        .enumerate()
        .any(|(i, b)| *b == b'%' && bytes.get(i + 1).is_some_and(u8::is_ascii_digit))
    {
        return None;
    }
    Some(text)
}

fn description_only_rules(r: &RecordRef) -> Option<Vec<SheetRule>> {
    let text = printable_description(r)?;
    Some(vec![SheetRule {
        id: r.id.clone(),
        label: r.name.clone(),
        value: SheetValue::Text,
        also: Vec::new(),
        prose: vec![ProseSegment {
            family: ProseFamily::Desc,
            pieces: vec![ProsePiece::Text(text)],
            applies: None,
            pick_last: false,
            suppress_when_all_zero: false,
        }],
        applies: Applies::Always,
        target: None,
        bonus_type: None,
        print: true,
        pool: String::new(),
        tags: r.type_facet.split('.').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
        subject: Subject::Character,
        repeatable: false,
        granted_by: Vec::new(),
        offers: None,
        grants: Vec::new(),
        closure_complete: false,
        always_held: false,
        provenance: Provenance {
            book: r.book.clone(),
            kind: r.kind.clone(),
            closure_rows: Vec::new(),
            oracle_pin: oracle_pin(),
            converter_version: convert::CONVERTER_VERSION.into(),
            ..Default::default()
        },
    }])
}

/// The words a sheet line names a corpus variable by, spaced out of its source name.
///
/// [`VarId`] is a content hash and the live side may not read a source name
/// (`decisions.md §11`), so before `VarTable::label` existed the sheet renderer printed every
/// variable as the generic phrase "a rules variable". This is the label it prints instead, and
/// it is produced HERE, at ingest, exactly once.
///
/// Purely mechanical, never an interpretation: `_`, `-` and `.` become a space, and a word
/// break is inserted where a lower-case or digit character is followed by an upper-case one
/// (`IntelligentItemEgo` -> `Intelligent Item Ego`). A run of capitals stays one word
/// (`IntItemStatINT` -> `Int Item Stat INT`), so a name the source wrote in all capitals comes
/// back unchanged apart from its separators. Nothing is title-cased, translated, expanded or
/// looked up in a table -- a label that reads oddly is the source name reading oddly, which is
/// the honest outcome.
pub fn display_label(source_name: &str) -> String {
    let mut out = String::with_capacity(source_name.len() + 8);
    let chars: Vec<char> = source_name.trim().chars().collect();
    for (i, ch) in chars.iter().copied().enumerate() {
        if matches!(ch, '_' | '-' | '.') {
            if !out.ends_with(' ') && !out.is_empty() {
                out.push(' ');
            }
            continue;
        }
        let prev_lower_or_digit =
            i > 0 && (chars[i - 1].is_lowercase() || chars[i - 1].is_ascii_digit());
        if ch.is_uppercase() && prev_lower_or_digit && !out.is_empty() && !out.ends_with(' ') {
            out.push(' ');
        }
        out.push(ch);
    }
    out.trim().to_string()
}

pub fn run(tree: &PinnedTree, index: &CorpusIndex, closures: &[Closure]) -> Run {
    let mut files: BTreeMap<String, Vec<SheetRule>> = BTreeMap::new();
    let mut grants_out: BTreeMap<RuleId, Vec<Grant>> = BTreeMap::new();
    let mut contribs: BTreeMap<VarId, (String, Vec<VarContribution>)> = BTreeMap::new();
    let mut declares: BTreeMap<VarId, (String, BTreeSet<RuleId>)> = BTreeMap::new();
    let mut var_names: BTreeMap<VarId, String> = BTreeMap::new();
    let mut var_labels: BTreeMap<VarId, String> = BTreeMap::new();
    let mut defects: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut refused = RefusedReport::default();
    let mut census = TokenCensus { schema: 1, entries: Vec::with_capacity(index.records.len()) };
    let mut report = Report { oracle_pin: oracle_pin(), converter_version: convert::CONVERTER_VERSION.into(), ..Default::default() };
    let mut converted_ids: Vec<(String, String, String)> = Vec::new();
    for (r, closure) in index.records.iter().zip(closures.iter()) {
        report.records += 1;
        let kc = report.by_kind.entry(r.kind.clone()).or_default();
        kc.records += 1;
        if !r.joined || r.rel_path.is_empty() {
            if let Some(rules) = description_only_rules(r) {
                census.entries.push(TokenCensusRecord {
                    id: r.id.clone(),
                    book: r.book.clone(),
                    kind: r.kind.clone(),
                    tokens: Vec::new(),
                    refusals: BTreeMap::new(),
                    degradations: BTreeMap::new(),
                });
                kc.converted += 1;
                converted_ids.push((r.book.clone(), r.kind.clone(), r.id.clone()));
                files.insert(rule_file_rel(&r.book, &r.kind, &r.id), rules);
                continue;
            }
            let tt = if r.joined { "no_source_row".to_string() } else { "no_corpus_record".to_string() };
            *refused.by_token_type.entry(tt.clone()).or_default() += 1;
            refused.entries.push(RefusedRecord { id: r.id.clone(), book: r.book.clone(), kind: r.kind.clone(), token_types: vec![tt.clone()] });
            census.entries.push(TokenCensusRecord { id: r.id.clone(), book: r.book.clone(), kind: r.kind.clone(), tokens: Vec::new(), refusals: BTreeMap::from([(tt, vec![TOKEN_LESS.to_string()])]), degradations: BTreeMap::new() });
            kc.refused += 1;
            continue;
        }
        if tree.file_index(&r.rel_path).is_none() && r.prerequisites.is_empty() && r.shipped_tokens.as_ref().is_none_or(|t| t.is_empty()) {
            if let Some(rules) = description_only_rules(r) {
                census.entries.push(TokenCensusRecord {
                    id: r.id.clone(),
                    book: r.book.clone(),
                    kind: r.kind.clone(),
                    tokens: Vec::new(),
                    refusals: BTreeMap::new(),
                    degradations: BTreeMap::new(),
                });
                kc.converted += 1;
                converted_ids.push((r.book.clone(), r.kind.clone(), r.id.clone()));
                files.insert(rule_file_rel(&r.book, &r.kind, &r.id), rules);
                continue;
            }
            let tt = "no_source_row".to_string();
            *refused.by_token_type.entry(tt.clone()).or_default() += 1;
            refused.entries.push(RefusedRecord { id: r.id.clone(), book: r.book.clone(), kind: r.kind.clone(), token_types: vec![tt.clone()] });
            census.entries.push(TokenCensusRecord { id: r.id.clone(), book: r.book.clone(), kind: r.kind.clone(), tokens: Vec::new(), refusals: BTreeMap::from([(tt, vec![TOKEN_LESS.to_string()])]), degradations: BTreeMap::new() });
            kc.refused += 1;
            continue;
        }
        let c = convert::convert_record(tree, index, r, closure);
        census.entries.push(TokenCensusRecord {
            id: r.id.clone(),
            book: r.book.clone(),
            kind: r.kind.clone(),
            tokens: c.tokens.iter().cloned().collect(),
            refusals: c.refusal_under.iter().map(|(shape, under)| (shape.clone(), under.iter().cloned().collect())).collect(),
            degradations: c.degraded_under.iter().map(|(shape, under)| (shape.clone(), under.iter().cloned().collect())).collect(),
        });
        for (k, v) in c.defects {
            defects.entry(k).or_default().extend(v);
        }
        for (id, name) in c.var_names {
            var_names.insert(id, name);
        }
        for (id, label) in c.var_labels {
            var_labels.entry(id).or_insert(label);
        }
        for (id, name) in c.var_declares {
            declares.entry(id).or_insert_with(|| (name, BTreeSet::new())).1.insert(r.id.clone());
        }
        for (id, name, contrib) in c.var_contribs {
            contribs.entry(id).or_insert_with(|| (name, Vec::new())).1.push(contrib);
        }
        if !c.degradations.is_empty() {
            report.degraded_records += 1;
            for t in c.degradations.iter() {
                *report.degraded_by_token_type.entry(t.clone()).or_default() += 1;
            }
        }
        if !c.refusals.is_empty() {
            let types: Vec<String> = c.refusals.into_iter().collect();
            for t in &types {
                *refused.by_token_type.entry(t.clone()).or_default() += 1;
            }
            refused.entries.push(RefusedRecord { id: r.id.clone(), book: r.book.clone(), kind: r.kind.clone(), token_types: types });
            kc.refused += 1;
            continue;
        }
        for (target, grant) in c.grants_out {
            grants_out.entry(target).or_default().push(grant);
        }
        kc.converted += 1;
        converted_ids.push((r.book.clone(), r.kind.clone(), r.id.clone()));
        let mut rules = c.rules;
        // The book's own sentence, when the token rows state none.
        //
        // SD-35 `AT-35-E6-003-SWEEP` cycle 16. The token path takes prose from `DESC:` /
        // `BENEFIT:` / `SPROP:` / `SAB:` / `TEMPDESC:` rows and from nowhere else, so a record
        // that carries structured tokens *and* a `description` field but no prose row converted
        // to a rule set with no prose at all and the sentence was dropped -- 241 records of the
        // 7,619 that state one, measured by
        // `sheet_rule_convert_gate::a_converted_record_never_drops_the_description_its_corpus_row_states`.
        // `decisions.md §1` form 3: the words ARE the sheet line.
        //
        // It is a **fallback**, never an addition: a record whose rows already state prose keeps
        // exactly the prose those rows state, because a `DESC:` row is the authored sheet line
        // and the `description` field is the same sentence in a second, unslotted form. Adding
        // both would print it twice. The bar is `printable_description`'s -- identical to the
        // one the no-source-row path has always applied, so a record cannot reach the sheet
        // through this door with words the other door would have refused.
        if !rules.iter().any(|rule| !rule.prose.is_empty())
            && let Some(text) = printable_description(r)
            && let Some(first) = rules.first_mut()
        {
            first.prose.push(ProseSegment {
                family: ProseFamily::Desc,
                pieces: vec![ProsePiece::Text(text)],
                applies: None,
                pick_last: false,
                suppress_when_all_zero: false,
            });
        }
        files.insert(rule_file_rel(&r.book, &r.kind, &r.id), rules);
    }
    // SD-36 F3c3: PCGen `SUBCLASS:` lines -> one choice on the class record, whose options are
    // `subclass` rules carrying each line's grants (`subclass.rs`). No inventory unit is added:
    // the record count does not move.
    let sub = subclass::convert_subclasses(tree, index, closures);
    let mut attached_choosers: BTreeSet<RuleId> = BTreeSet::new();
    for (class_id, chooser) in sub.choosers {
        let class_record = index.records.iter().find(|r| r.id == class_id);
        match class_record.and_then(|r| files.get_mut(&rule_file_rel(&r.book, &r.kind, &r.id))) {
            Some(rules) => {
                // Every line of a record carries the record's own gates (`record gates AND line
                // gate`, F1c-2); the choice line has no gate of its own.
                let mut chooser = chooser;
                if let Some(principal) = rules.first() {
                    chooser.applies = principal.applies.clone();
                }
                attached_choosers.insert(chooser.id.clone());
                rules.push(chooser);
            }
            None => defects.entry("subclass-class-unconverted".into()).or_default().push(format!("{class_id}: {}", chooser.id)),
        }
    }
    for opt in sub.options {
        let chooser = format!("{}#{}", opt.class_record, subclass::CHOICE_SUFFIX);
        if !attached_choosers.contains(&chooser) {
            continue;
        }
        let c = opt.converted;
        for (k, v) in c.defects {
            defects.entry(k).or_default().extend(v);
        }
        for (id, name) in c.var_names {
            var_names.insert(id, name);
        }
        for (id, label) in c.var_labels {
            var_labels.entry(id).or_insert(label);
        }
        for (id, name) in c.var_declares {
            declares.entry(id).or_insert_with(|| (name, BTreeSet::new())).1.insert(opt.rule.id.clone());
        }
        for (id, name, contrib) in c.var_contribs {
            contribs.entry(id).or_insert_with(|| (name, Vec::new())).1.push(contrib);
        }
        for (target, grant) in opt.grants_out {
            grants_out.entry(target).or_default().push(grant);
        }
        let rel = rule_file_rel(&opt.rule.provenance.book, subclass::SUBCLASS_KIND, &opt.rule.id);
        let mut rules = vec![opt.rule];
        rules.extend(opt.siblings);
        files.insert(rel, rules);
    }
    for (k, v) in sub.defects {
        defects.entry(k).or_default().extend(v);
    }
    if !sub.superseded.is_empty() {
        defects.entry("subclass-superseded-reprint".into()).or_default().extend(sub.superseded);
    }
    // SD-36 F3c4b: ability-category pick rows -> options of the choice that picks them
    // (`pool_option.rs`). No inventory unit is added: the record count does not move.
    let mut option_rows: BTreeSet<RowRef> = BTreeSet::new();
    {
        let offered = |chooser: &str, pool: &str| {
            index.records.iter().find(|r| r.id == chooser).and_then(|r| files.get(&rule_file_rel(&r.book, &r.kind, &r.id))).and_then(|rules| rules.first()).is_some_and(|p| {
                matches!(&p.offers, Some(Choice { id, from: OptionSet::Rules { pool: q, .. }, .. }) if id == chooser && q == pool)
            })
        };
        let (options, option_defects) = pool_option::convert_options(tree, index, &offered);
        for (k, v) in option_defects.into_iter().chain(index.index_defects.clone()).filter(|(_, v)| !v.is_empty()) {
            defects.entry(k).or_default().extend(v);
        }
        for opt in options {
            let c = opt.converted;
            for (k, v) in c.defects {
                defects.entry(k).or_default().extend(v);
            }
            for (id, name) in c.var_names {
                var_names.insert(id, name);
            }
            for (id, label) in c.var_labels {
                var_labels.entry(id).or_insert(label);
            }
            for (id, name) in c.var_declares {
                declares.entry(id).or_insert_with(|| (name, BTreeSet::new())).1.insert(opt.rule.id.clone());
            }
            for (id, name, contrib) in c.var_contribs {
                contribs.entry(id).or_insert_with(|| (name, Vec::new())).1.push(contrib);
            }
            for (target, grant) in opt.grants_out {
                grants_out.entry(target).or_default().push(grant);
            }
            option_rows.extend(opt.own_rows.iter().copied());
            let rel = rule_file_rel(&opt.rule.provenance.book, pool_option::POOL_OPTION_KIND, &opt.rule.id);
            let mut rules = vec![opt.rule];
            rules.extend(opt.siblings);
            files.insert(rel, rules);
        }
    }
    // D3: every class-selection record gets the class principal it stands for.
    for r in &index.records {
        if r.class_selection_of.is_none() {
            continue;
        }
        let Some(selection) = files.get(&rule_file_rel(&r.book, &r.kind, &r.id)).and_then(|rules| rules.first()).cloned() else { continue };
        match class_selection_principal(index, r, &selection) {
            Ok((principal, grant)) => {
                let rel = rule_file_rel(&r.book, "class", &principal.id);
                if files.contains_key(&rel) {
                    defects.entry("class-selection-principal-collision".into()).or_default().push(format!("{}: {rel}", r.id));
                    continue;
                }
                grants_out.entry(selection.id.clone()).or_default().push(grant);
                files.insert(rel, vec![principal]);
            }
            Err(e) => defects.entry("unresolved-references".into()).or_default().push(e),
        }
    }
    // D6: a pick into a one-member weapon-choice pool is linked to its options.
    let category_views = pool_link::category_views(tree);
    pool_link::link_weapon_choice_pools(&category_views, weapon_membership::index(tree), &mut files);
    // SD-36 F4pre (FS-21): every other pick into a child ability category, and every domain
    // count, as a converted choice over the members (`pool_link::link_pool_choices`).
    pool_link::link_pool_choices(&category_views, &mut files);
    // Attach grant edges to the principal rule of each target.
    for rules in files.values_mut() {
        if let Some(first) = rules.first_mut()
            && let Some(g) = grants_out.remove(&first.id)
        {
            first.granted_by.extend(g);
        }
    }
    for (target, g) in &grants_out {
        defects.entry("grants-to-unconverted-targets".into()).or_default().push(format!("{target}: {} grant(s)", g.len()));
    }
    // D4: the closure-complete attestation on every class principal.
    let defective = attest::defective_records(&defects, &grants_out);
    attest::attest_class_closures(&mut files, &defective);
    // D7: the global abilities every character holds unconditionally (`always_held.rs`).
    let (_, unresolved_globals) = always_held::mark_always_held(tree, &mut files, index, &always_held::global_grants(tree));
    if !unresolved_globals.is_empty() {
        defects.entry("unresolved-references".into()).or_default().extend(unresolved_globals);
    }
    // D8: an oracle member of a filled variable pool no converted record stands for.
    let unconverted_members = pool_pick::unconverted_member_defects(&index.filled_pools);
    if !unconverted_members.is_empty() {
        defects.entry("pool-member-unconverted".into()).or_default().extend(unconverted_members);
    }
    // Variable tables for every referenced id.
    let mut referenced: BTreeSet<VarId> = BTreeSet::new();
    for rules in files.values() {
        for rule in rules {
            referenced.extend(rule.var_ids());
        }
    }
    for (_, contribs_for) in contribs.values() {
        for c in contribs_for {
            let mut ids = Vec::new();
            c.expr.var_ids(&mut ids);
            c.when.var_ids(&mut ids);
            referenced.extend(ids);
        }
    }
    let mut owned_rows: BTreeSet<RowRef> = index.row_owner.keys().copied().collect();
    owned_rows.extend(option_rows);
    let mut vars: BTreeMap<VarId, VarTable> = BTreeMap::new();
    for id in &referenced {
        let name = var_names.get(id).cloned().or_else(|| contribs.get(id).map(|(n, _)| n.clone())).or_else(|| declares.get(id).map(|(n, _)| n.clone()));
        let Some(name) = name else { continue };
        var_names.entry(id.clone()).or_insert(name.clone());
        let declared_by: Vec<RuleId> = declares.get(id).map(|(_, s)| s.iter().cloned().collect()).unwrap_or_default();
        let contributions: Vec<VarContribution> = contribs.get(id).map(|(_, v)| v.clone()).unwrap_or_default();
        let outside: Vec<String> = tree.variable_rows(&name).into_iter().filter(|r| !owned_rows.contains(r)).map(|r| tree.cite(r)).collect();
        let label = display_label(var_labels.get(id).unwrap_or(&name));
        vars.insert(id.clone(), VarTable { var: id.clone(), label, declared_by, contributions, provenance: VarProvenance { outside_corpus_rows: outside } });
    }
    for v in defects.values_mut() {
        v.sort();
        v.dedup();
    }
    refused.records = report.records;
    refused.converted = converted_ids.len();
    refused.refused = refused.entries.len();
    report.converted = refused.converted;
    report.refused = refused.refused;
    report.rules_written = files.values().map(|v| v.len()).sum();
    report.var_tables = vars.len();
    report.refused_by_token_type = refused.by_token_type.clone();
    Run { files, vars, var_names, refused, tokens: census, defects, report }
}

// ---- output -----------------------------------------------------------------------------------

/// JSON with `:` escaped inside strings, for the ledgers that name token types.
fn json_colon_escaped<T: Serialize>(value: &T) -> Vec<u8> {
    let mut out = escape_token_strings(&serde_json::to_string_pretty(value).unwrap());
    out.push('\n');
    out.into_bytes()
}

/// The census, one record per line (49k rows: diffable, and no 10 MB single line), with the
/// same in-string escaping as the other ledgers.
fn json_census(census: &TokenCensus) -> Vec<u8> {
    let mut out = String::with_capacity(census.entries.len() * 160 + 64);
    out.push_str("{\n  \"schema\": ");
    out.push_str(&census.schema.to_string());
    out.push_str(",\n  \"entries\": [\n");
    for (i, e) in census.entries.iter().enumerate() {
        out.push_str("    ");
        out.push_str(&escape_token_strings(&serde_json::to_string(e).unwrap()));
        out.push_str(if i + 1 < census.entries.len() { ",\n" } else { "\n" });
    }
    out.push_str("  ]\n}\n");
    out.into_bytes()
}

/// Escape `:`, `%` and `=` inside JSON string literals only, walking the text tracking string
/// state, so no output file carries a source-format literal (`shape_violations`).
fn escape_token_strings(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 64);
    let mut in_str = false;
    let mut escaped = false;
    for c in text.chars() {
        match c {
            '\\' if in_str && !escaped => {
                escaped = true;
                out.push(c);
                continue;
            }
            '"' if !escaped => {
                in_str = !in_str;
                out.push(c);
            }
            ':' if in_str => out.push_str("\\u003a"),
            '%' if in_str => out.push_str("\\u0025"),
            '=' if in_str => out.push_str("\\u003d"),
            _ => out.push(c),
        }
        escaped = false;
    }
    out
}

/// Serialize the run to the on-disk layout: `(relative path, bytes)` for every file.
pub fn render(run: &Run) -> BTreeMap<String, Vec<u8>> {
    let mut out: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    for (rel, rules) in &run.files {
        let mut text = serde_json::to_string(rules).unwrap();
        text.push('\n');
        out.insert(rel.clone(), text.into_bytes());
    }
    for (id, table) in &run.vars {
        let mut text = serde_json::to_string(table).unwrap();
        text.push('\n');
        out.insert(format!("_vars/{id}.json"), text.into_bytes());
    }
    out.insert("_refused.json".into(), json_colon_escaped(&run.refused));
    out.insert("_tokens.json".into(), json_census(&run.tokens));
    for (kind, lines) in &run.defects {
        out.insert(format!("_defects/{kind}.json"), json_colon_escaped(lines));
    }
    out.insert("_report.json".into(), json_colon_escaped(&run.report));
    out
}

pub fn write_output(out_dir: &Path, rendered: &BTreeMap<String, Vec<u8>>) -> std::io::Result<()> {
    if out_dir.exists() {
        std::fs::remove_dir_all(out_dir)?;
    }
    std::fs::create_dir_all(out_dir)?;
    std::fs::write(out_dir.join("GENERATED"), b"GENERATED FILE TREE -- written by `cargo run --locked -p codex-ingest --bin sheet_rule_convert -- --write`; regenerated whole; never hand-edited.\n")?;
    for (rel, bytes) in rendered {
        let p = out_dir.join(rel);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(p, bytes)?;
    }
    Ok(())
}

pub fn write_var_names(repo: &Path, run: &Run) -> std::io::Result<()> {
    write_var_names_to(&repo.join("scripts/oracle_harness"), run)
}

/// Write `var_names.json` to an arbitrary directory (e.g. a `--dump` scratch dir), the same
/// content `write_var_names` writes to the tracked `scripts/oracle_harness/` -- so a `--dump` run
/// emits this file too, and a structural diff can cover it (SD-36 Epic F1 re-check round 1,
/// finding 3: a real run's blast radius on this tracked file was otherwise unmeasured by any
/// `--dump`-based diff).
pub fn write_var_names_to(dir: &Path, run: &Run) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    let mut text = serde_json::to_string_pretty(&run.var_names).unwrap();
    text.push('\n');
    std::fs::write(dir.join("var_names.json"), text)
}

/// Read every file under `out_dir` as `(relative path, bytes)`.
pub fn read_output(out_dir: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    let mut stack = vec![out_dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else { continue };
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.file_name().is_some_and(|n| n != "GENERATED") {
                let rel = p.strip_prefix(out_dir).unwrap().to_string_lossy().replace('\\', "/");
                out.insert(rel, std::fs::read(&p).unwrap_or_default());
            }
        }
    }
    out
}

/// The source-format literals no output file may carry (B10 + `workflow-instruction.md` §6).
pub const FORBIDDEN_LITERALS: &[&str] = &["BONUS:", "DEFINE:", "%CHOICE", "%LIST", "CL=", "TYPE=", "SAB:", "DESC:"];

/// Files whose bytes carry a forbidden literal, a `PRE<X>:` head, or a `%<digit>` marker.
pub fn shape_violations(rendered: &BTreeMap<String, Vec<u8>>) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (rel, bytes) in rendered {
        let text = String::from_utf8_lossy(bytes);
        for lit in FORBIDDEN_LITERALS {
            if text.contains(lit) {
                out.push((rel.clone(), lit.to_string()));
            }
        }
        if has_pre_head(&text) {
            out.push((rel.clone(), "PRE[A-Z]+:".into()));
        }
        if has_percent_digit(&text) {
            out.push((rel.clone(), "%[0-9]".into()));
        }
    }
    out
}

fn has_pre_head(text: &str) -> bool {
    let b = text.as_bytes();
    let mut i = 0;
    while let Some(pos) = text[i..].find("PRE") {
        let start = i + pos;
        let mut j = start + 3;
        while j < b.len() && b[j].is_ascii_uppercase() {
            j += 1;
        }
        if j > start + 3 && j < b.len() && b[j] == b':' {
            return true;
        }
        i = start + 3;
    }
    false
}

fn has_percent_digit(text: &str) -> bool {
    let b = text.as_bytes();
    b.windows(2).any(|w| w[0] == b'%' && w[1].is_ascii_digit())
}

/// `--check`: the on-disk package equals a fresh conversion byte for byte, carries no
/// source-format literal, and every referenced variable id has a table.
pub fn check(out_dir: &Path, run: &Run) -> Result<(), Vec<String>> {
    let mut problems = Vec::new();
    let fresh = render(run);
    let disk = read_output(out_dir);
    for (rel, bytes) in &fresh {
        match disk.get(rel) {
            None => problems.push(format!("missing on disk: {rel}")),
            Some(d) if d != bytes => problems.push(format!("stale on disk: {rel}")),
            _ => {}
        }
    }
    for rel in disk.keys() {
        if !fresh.contains_key(rel) {
            problems.push(format!("unexpected file on disk: {rel}"));
        }
    }
    for (rel, lit) in shape_violations(&fresh) {
        problems.push(format!("source-format literal {lit:?} in {rel}"));
    }
    for rules in run.files.values() {
        for rule in rules {
            for id in rule.var_ids() {
                if !run.vars.contains_key(&id) {
                    problems.push(format!("{}: variable {id} has no table", rule.id));
                }
            }
        }
    }
    if run.report.converted + run.report.refused != run.report.records {
        problems.push("converted + refused != records".into());
    }
    if problems.is_empty() { Ok(()) } else { Err(problems) }
}

#[cfg(test)]
mod check_tests {
    use super::*;

    /// F1 adversarial finding 4: a `problems.truncate(200)` here used to silently hide everything
    /// past the 200th problem, so `--check` could never state the TRUE delta on a run whose
    /// structural diff is large by design (Option A's up-to-4,456 new edges, plus this branch's
    /// own +570 `_vars/` files) -- the 200 slots were consumed before a single named record even
    /// printed. A synthetic `Run` with 250 rendered rule files checked against an EMPTY on-disk
    /// directory produces one "missing on disk" problem per rendered file -- 250 from `files`,
    /// plus the three fixed files `render()` always emits (`_refused.json`/`_tokens.json`/
    /// `_report.json`) -- 253 in all; `check` must report every one, never cap at 200.
    #[test]
    fn check_reports_every_problem_not_just_the_first_two_hundred() {
        let run = Run {
            files: (0..250).map(|i| (format!("book/kind/r{i:04}.json"), Vec::new())).collect(),
            vars: BTreeMap::new(),
            var_names: BTreeMap::new(),
            refused: RefusedReport::default(),
            tokens: TokenCensus::default(),
            defects: BTreeMap::new(),
            report: Report::default(),
        };
        // Never created, never written to -- `read_output` treats a missing directory as empty,
        // exactly like a fresh checkout before the first `sheet_rule_convert` run.
        let empty_dir = std::env::temp_dir().join(format!("sheet_rule_check_test_{}_{}", std::process::id(), line!()));
        let err = check(&empty_dir, &run).expect_err("every rendered file missing on disk must fail the check");
        let expected = run.files.len() + 3;
        assert_eq!(err.len(), expected, "check() must report every problem, never truncate: got {} of {expected}", err.len());
    }
}

/// Load everything and run once: the tree, the population, the index, the conversion.
pub fn convert_repo(repo: &Path) -> Result<(Run, CorpusIndex), String> {
    let tree = PinnedTree::load(&closure::corpus_root())?;
    let records = load_population(repo, &tree)?;
    let (index, closures) = build_index(&tree, records);
    let run = run(&tree, &index, &closures);
    Ok((run, index))
}

/// Convert one unit by id (for tests and spot checks): the whole index is built, one record
/// is converted, and its `Converted` is returned with the closure rows it read.
pub fn convert_one(repo: &Path, unit_id: &str) -> Result<(convert::Converted, Vec<String>), String> {
    let tree = PinnedTree::load(&closure::corpus_root())?;
    let records = load_population(repo, &tree)?;
    let (index, closures) = build_index(&tree, records);
    let pos = index.records.iter().position(|r| r.id == unit_id).ok_or_else(|| format!("no unit {unit_id}"))?;
    let mut c = convert::convert_record(&tree, &index, &index.records[pos], &closures[pos]);
    // D3: a class-selection record also yields its class principal (printed with the rules).
    if let Some(selection) = c.rules.first().cloned()
        && let Ok((principal, grant)) = class_selection_principal(&index, &index.records[pos], &selection)
    {
        c.grants_out.push((selection.id.clone(), grant));
        c.rules.push(principal);
    }
    let mut rows: Vec<String> = closures[pos].rows.iter().map(|r| format!("{} {:?}", r.cite, r.tokens.iter().map(|(k, v)| format!("{k}:{v}")).collect::<Vec<_>>())).collect();
    for (id, name) in &c.var_names {
        let cites: Vec<String> = tree.variable_rows(name).into_iter().map(|r| tree.cite(r)).collect();
        rows.push(format!("var {id} {name}: {cites:?}"));
    }
    Ok((c, rows))
}

// -------------------------------------------------------------------------------------------
// SD-35 AT-35-E3-001: the term-level-refusal gate.
//
// `decisions.md` §4 asks for one gate per KIND that reads the live corpus directory, never a
// per-unit fixture with a hand-derived value. These two read the generated package and the
// converter's own census on disk and pin every record at once.
// -------------------------------------------------------------------------------------------
#[cfg(test)]
mod term_level_refusal_gate {
    use super::*;

    fn package_dir() -> PathBuf {
        repo_root().join("data/sheet_rules")
    }

    fn refused_report() -> RefusedReport {
        let text = std::fs::read_to_string(package_dir().join("_refused.json"))
            .expect("data/sheet_rules/_refused.json is generated (cargo run --locked -p codex-ingest --bin sheet_rule_convert -- --write)");
        serde_json::from_str(&text).expect("_refused.json parses")
    }

    fn census() -> TokenCensus {
        let text = std::fs::read_to_string(package_dir().join("_tokens.json"))
            .expect("data/sheet_rules/_tokens.json is generated");
        serde_json::from_str(&text).expect("_tokens.json parses")
    }

    /// A record leaves the sheet entirely for exactly three reasons: it has no corpus record,
    /// it has no source row, or its own value is the redacted field
    /// ([`ctx::RECORD_REFUSAL_SHAPES`], `decisions.md` §15 R2). Every other unlowerable token
    /// degrades that term instead of deleting the record -- the standing no-carve-outs ruling.
    #[test]
    fn the_only_record_level_refusals_are_the_named_shapes() {
        let allowed: BTreeSet<&str> = ["no_corpus_record", "no_source_row"]
            .into_iter()
            .chain(ctx::RECORD_REFUSAL_SHAPES.iter().copied())
            .collect();
        let report = refused_report();
        let offenders: Vec<String> = report
            .entries
            .iter()
            .flat_map(|e| e.token_types.iter().map(move |t| (e.id.clone(), t.clone())))
            .filter(|(_, t)| !allowed.contains(t.as_str()))
            .map(|(id, t)| format!("{id}: {t}"))
            .take(10)
            .collect();
        assert!(
            offenders.is_empty(),
            "a token shape outside {allowed:?} refused a whole record: {offenders:?}"
        );
    }

    /// Every degraded record reached the sheet. Before SD-36 Epic E CONV-05, degradation was
    /// record-wide -- ANY unlowerable term wiped EVERY line the record produced to Text, so a
    /// degraded record printed no number anywhere. CONV-05 tracks degradation per OCCURRENCE
    /// instead (`RecordCtx::current_seq`/`current_seq_degraded`, `convert.rs`): a sibling term
    /// the converter genuinely could not lower still wipes to Text, but a term on the SAME
    /// record that converted cleanly (`advanced_class_guide:class:bloodrager`'s BAB/save
    /// progressions, degraded only because an unrelated token elsewhere on the same giant class
    /// record could not lower) now correctly keeps its own number. A partly-read magnitude
    /// still never reaches a sheet total -- that per-occurrence guarantee is unit-tested
    /// directly against real corpus rows in `tests/sheet_rule_convert_gate.rs`'s
    /// `a_sibling_terms_degradation_does_not_erase_a_convertible_terms_number` -- this gate
    /// checks the coarser, record-level invariants the on-disk census can see: every degraded
    /// record still converts (never falls into `_refused.json`) and still has a rule file, and
    /// pins the corpus-wide count of "numbered lines inside a degraded record" as a re-derivable
    /// baseline so a large, unexplained jump (a fix regressing back toward record-wide wiping,
    /// or a new mass-degrading token) still fails loudly.
    #[test]
    fn every_degraded_record_converted_and_prints_its_words() {
        let refused: BTreeSet<String> = refused_report().entries.into_iter().map(|e| e.id).collect();
        let files = read_output(&package_dir());
        assert!(!files.is_empty(), "data/sheet_rules/ is generated");
        let census = census();
        let degraded: Vec<&TokenCensusRecord> = census.entries.iter().filter(|e| !e.degradations.is_empty()).collect();
        assert!(!degraded.is_empty(), "the corpus carries degraded records; the census names them");
        let mut still_refused = Vec::new();
        let mut missing = Vec::new();
        let mut numbered = Vec::new();
        for e in &degraded {
            if refused.contains(&e.id) {
                still_refused.push(e.id.clone());
                continue;
            }
            let rel = rule_file_rel(&e.book, &e.kind, &e.id);
            let Some(bytes) = files.get(&rel) else {
                missing.push(rel);
                continue;
            };
            let rules: Vec<SheetRule> = serde_json::from_slice(bytes).unwrap_or_else(|err| panic!("{rel}: {err}"));
            for r in &rules {
                if r.value != SheetValue::Text || r.target.is_some() || !r.also.is_empty() {
                    numbered.push(format!("{}: value={:?} target={:?}", r.id, r.value, r.target));
                }
            }
        }
        // Re-derive with: `python3 -c "import json; tokens=json.load(open('data/sheet_rules/_tokens.json')); refused={e['id'] for e in json.load(open('data/sheet_rules/_refused.json'))['entries']}; degraded=[e for e in tokens['entries'] if e.get('degradations') and e['id'] not in refused]; print(len(degraded))"` for record count, and the loop above (without `numbered.truncate`) for the line count.
        assert_eq!(degraded.len(), 424, "degraded-record count moved -- re-derive and update this pin (command in the doc comment)");
        let numbered_count = numbered.len();
        still_refused.truncate(10);
        missing.truncate(10);
        numbered.truncate(10);
        assert!(still_refused.is_empty(), "degraded records must still convert: {still_refused:?}");
        assert!(missing.is_empty(), "degraded records must have a rule file: {missing:?}");
        // 624 -> 636 (SD-36 F1c-3): six degraded product-identity class records now read their
        // own `CLASS:<name>` continuation rows and level lines -- Hellknight (both printings) +4
        // each, Hellknight Signifer, Red Mantis Assassin (both printings) and Cyphermage
        // (`inner_sea_magic`) +1 each. Their degradation (`BONUS:[redacted PI]`, a redacted
        // DEFINE) is unchanged.
        assert_eq!(numbered_count, 636, "numbered-lines-inside-a-degraded-record count moved (CONV-05 baseline) -- re-derive and update this pin, or investigate if it jumped unexpectedly: sample {numbered:?}");
    }

    /// The report's own sums: a degraded record is a CONVERTED record, and the per-shape
    /// counts are the census's, so `token_coverage.py`'s ledger and this report agree.
    #[test]
    fn the_report_counts_degradations_as_converted() {
        let text = std::fs::read_to_string(package_dir().join("_report.json")).expect("_report.json is generated");
        let report: Report = serde_json::from_str(&text).expect("_report.json parses");
        assert_eq!(report.converted + report.refused, report.records, "converted + refused == records");
        let census = census();
        let degraded = census.entries.iter().filter(|e| !e.degradations.is_empty()).count();
        assert_eq!(report.degraded_records, degraded, "_report.degraded_records equals the census's degraded records");
        assert!(report.degraded_records <= report.converted, "a degraded record converted");
    }

    /// **Every `BONUS:` clause the ingest stored in the record's SECOND array reaches the
    /// converter.**
    ///
    /// The ingest writes a `.lst` row's `BONUS:` clauses into `raw_bonus_chains`, not into
    /// `raw_tokens` (`pcgen_import::bonus_chain_reader`'s own module doc states the split).
    /// [`record_from_json`] built `shipped_tokens` from `raw_tokens` alone, and
    /// [`closure::PinnedTree::closure`] lets shipped tokens **replace** the base row's, so for
    /// every record that ships tokens at all the base row's `BONUS:` clauses were dropped on
    /// the floor — silently, because a dropped token is not a refusal.
    ///
    /// What that cost, measured over the live corpus directory rather than a fixture:
    /// `Dwarf ~ Ability Scores` states `BONUS:STAT|CON,WIS|2|TYPE=Racial` and
    /// `BONUS:STAT|CHA|-2|TYPE=Racial` on its source row, and its converted rule carried
    /// `value: Text`, no `target` and no `bonus_type` — the racial ability adjustment, absent
    /// from the package a sheet is printed from.
    ///
    /// The gate is per-kind and reads the live corpus (`decisions.md` §4): it walks
    /// `data/corpus/` for every record carrying a non-empty `raw_bonus_chains`, joins it to the
    /// converter's own census by `(book, file stem)`, and requires a `BONUS:` census key on the
    /// matched entry. A record the census does not hold is **counted and reported**, never
    /// excused: those are corpus records that are not inventory units, which the converter's
    /// population never sees.
    #[test]
    fn every_ingested_bonus_chain_reaches_the_converter() {
        let census = census();
        // Keyed on kind as well as book and slug: `core_rulebook` ships a `holy_symbol_silver`
        // under BOTH `equipment/general/` and `equipment/equipmods/`, and a (book, slug) key
        // resolves the chain-bearing one to the other's census entry.
        let mut by_id: BTreeMap<(String, String, String), &TokenCensusRecord> = BTreeMap::new();
        for entry in &census.entries {
            let slug = entry.id.rsplit(':').next().unwrap_or("").split('#').next().unwrap_or("").to_string();
            by_id.entry((entry.book.clone(), entry.kind.clone(), slug)).or_insert(entry);
        }

        let corpus = repo_root().join("data/corpus");
        let mut stack = vec![corpus.clone()];
        let mut with_chains = 0usize;
        let mut matched = 0usize;
        let mut not_a_unit = 0usize;
        let mut dropped = 0usize;
        let mut offenders: Vec<String> = Vec::new();
        while let Some(dir) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else { continue };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                    continue;
                }
                if path.extension().is_none_or(|e| e != "json")
                    || path.file_name().is_some_and(|n| n == "LICENSE.json")
                {
                    continue;
                }
                let Ok(text) = std::fs::read_to_string(&path) else { continue };
                let Ok(doc) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
                let chains = doc["data"]["raw_bonus_chains"].as_array().map(Vec::len).unwrap_or(0);
                if chains == 0 {
                    continue;
                }
                with_chains += 1;
                let rel = path.strip_prefix(&corpus).unwrap();
                let book = match rel.components().next().map(|c| c.as_os_str().to_string_lossy().to_string()) {
                    // The corpus directory's historical spelling of the book the converter
                    // writes as `bestiary`.
                    Some(b) if b == "beastiary" => "bestiary".to_string(),
                    Some(b) => b,
                    None => continue,
                };
                let parts: Vec<String> =
                    rel.components().map(|c| c.as_os_str().to_string_lossy().to_string()).collect();
                // The corpus directory's kind name, and the one place it differs from the
                // converter's: `<book>/equipment/equipmods/` is the `equipment_modifier` kind.
                let kind = if parts.iter().any(|p| p == "equipmods") {
                    "equipment_modifier".to_string()
                } else {
                    parts.get(1).cloned().unwrap_or_default()
                };
                let slug = path.file_stem().unwrap().to_string_lossy().to_string();
                let Some(entry) = by_id.get(&(book.clone(), kind, slug.clone())) else {
                    not_a_unit += 1;
                    continue;
                };
                matched += 1;
                if !entry.tokens.iter().any(|t| t.starts_with("BONUS:")) {
                    dropped += 1;
                    if offenders.len() < 6 {
                        offenders.push(format!("{} ({chains} chain(s))", entry.id));
                    }
                }
            }
        }
        assert!(with_chains > 0, "the corpus carries records with BONUS chains");
        assert_eq!(
            dropped, 0,
            "{dropped} of {matched} joined corpus records ship BONUS chains the converter never saw \
             (corpus records carrying chains: {with_chains}; not inventory units: {not_a_unit}). \
             First offenders: {offenders:?}"
        );
    }

    /// **A `.COPY=` row's own `VISIBLE:NO` wins over the value it inherits.**
    ///
    /// SD-35 `AT-35-E6-003` cycle 10, the companion to the gate above and the same lesson: a
    /// coverage instrument built over the reader's own input cannot see an input the reader
    /// mis-ordered, so this enumerates the **source** independently.
    ///
    /// PCGen applies a `.COPY=` record as *copied base -> the copy row's own tokens*. The
    /// corpus ingest flattens both into one `raw_tokens` array with the copy row's own tokens
    /// **first**, and `closure::PinnedTree::closure` uses that array in place of the base row's,
    /// so every last-wins metadata head the copy row overrides took the **inherited** value.
    /// 473 records whose copy row states `VISIBLE:NO` -- PCGen's own bookkeeping shadows, such
    /// as `Intelligent Item ~ Alignment / Lawful Good.COPY=Intelligent Item Alignment (LG)`,
    /// whose copied base states `VISIBLE:QUALIFY` -- therefore converted with `print: true`:
    /// the package said a row a player never sees in PCGen's own item builder belongs on a
    /// character sheet. [`closure::copy_own_tokens_last`] is the fix.
    ///
    /// The gate walks `data/corpus/` (the live directory, `decisions.md` §4 -- never a
    /// fixture), reads each record's own base row out of the pinned tree by the
    /// `source.path:line` the record itself carries, and requires `print: false` on every
    /// converted rule of a record whose base row is a `.COPY=` row stating `VISIBLE:NO`. A
    /// record the package does not hold under its own id is **counted and reported**, never
    /// excused.
    ///
    /// GitHub CI never fetches the pinned PCGen checkout, so this gate skips -- never
    /// false-passes -- when it is absent; `scripts/verify.sh`'s `preflight-oracle` stage
    /// guarantees the checkout is present for a local full run, where this gate always executes.
    #[test]
    fn a_copy_rows_own_visible_no_reaches_the_converted_rule() {
        let pinned = closure::corpus_root();
        if !pinned.join(closure::BOOKS_RELATIVE).is_dir() {
            eprintln!("skipping: no pinned PCGen corpus checkout at {pinned:?}");
            return;
        }
        let mut rows: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut row_text = |rel: &str, line: usize| -> Option<String> {
            let lines = rows.entry(rel.to_string()).or_insert_with(|| {
                std::fs::read_to_string(pinned.join(rel))
                    .map(|t| t.lines().map(str::to_string).collect())
                    .unwrap_or_default()
            });
            lines.get(line.checked_sub(1)?).cloned()
        };

        let files = read_output(&package_dir());
        assert!(!files.is_empty(), "data/sheet_rules/ is generated");
        let corpus = repo_root().join("data/corpus");
        let mut stack = vec![corpus.clone()];
        let mut hidden_copies = 0usize;
        let mut checked = 0usize;
        let mut not_in_package = 0usize;
        let mut offenders = 0usize;
        let mut printing: Vec<String> = Vec::new();
        while let Some(dir) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else { continue };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                    continue;
                }
                if path.extension().is_none_or(|e| e != "json")
                    || path.file_name().is_some_and(|n| n == "LICENSE.json")
                {
                    continue;
                }
                let Ok(text) = std::fs::read_to_string(&path) else { continue };
                let Ok(doc) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
                let (Some(rel), Some(line)) = (
                    doc["source"]["path"].as_str(),
                    doc["source"]["line"].as_u64().map(|v| v as usize),
                ) else {
                    continue;
                };
                let Some(row) = row_text(rel, line) else { continue };
                let (name, tokens) = closure::tokenize_row(&row);
                if !name.contains(".COPY=") {
                    continue;
                }
                if !tokens.iter().any(|(k, v)| k == "VISIBLE" && v.trim().eq_ignore_ascii_case("NO")) {
                    continue;
                }
                hidden_copies += 1;
                let parts: Vec<String> = path
                    .strip_prefix(&corpus)
                    .unwrap()
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy().to_string())
                    .collect();
                let book = match parts.first().map(String::as_str) {
                    Some("beastiary") => "bestiary".to_string(),
                    Some(b) => b.to_string(),
                    None => continue,
                };
                let kind = if parts.iter().any(|p| p == "equipmods") {
                    "equipment_modifier".to_string()
                } else {
                    parts.get(1).cloned().unwrap_or_default()
                };
                let slug = path.file_stem().unwrap().to_string_lossy().to_string();
                let file_rel = format!("{book}/{kind}/{slug}.json");
                let Some(bytes) = files.get(&file_rel) else {
                    not_in_package += 1;
                    continue;
                };
                let rules: Vec<SheetRule> =
                    serde_json::from_slice(bytes).unwrap_or_else(|err| panic!("{file_rel}: {err}"));
                checked += 1;
                // The offender COUNT and the offender EXAMPLES are separate numbers: cycle 9's
                // own first gate draft reported its display cap as a total. `offenders` is the
                // count; `printing` is at most eight names for the message.
                for rule in &rules {
                    if rule.print {
                        offenders += 1;
                        if printing.len() < 8 {
                            printing.push(rule.id.clone());
                        }
                    }
                }
            }
        }
        assert!(
            hidden_copies > 0,
            "the pinned tree carries `.COPY=` rows stating VISIBLE:NO; this gate proved nothing"
        );
        assert_eq!(
            offenders, 0,
            "{offenders} converted rules across {checked} records whose own `.COPY=` row states \
             VISIBLE:NO still print (hidden copy rows in the corpus: {hidden_copies}; not held by \
             the package under their own id: {not_in_package}). First offenders: {printing:?}"
        );
    }
}
