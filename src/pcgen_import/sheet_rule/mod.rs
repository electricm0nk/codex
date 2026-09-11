//! The sheet-rule converter (SD-35 AT-35-E2-001, `technical-design.md` §1).
//!
//! Run once at ingest time over every unit of `docs/work-inventory.json` (49,438 -- the
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

pub mod closure;
pub mod convert;
pub mod ctx;
pub mod formula;
pub mod prereq;
pub mod prose;
pub mod table;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::pcgen_import::ingest_record;
use crate::rules_core::sheet_rule::*;
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
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
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

fn record_from_json(unit: &InventoryUnit, path: &Path) -> Option<RecordRef> {
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
    let name = data.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_else(|| unit.name.clone());
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
pub fn load_population(repo: &Path, tree: &PinnedTree) -> Result<Vec<RecordRef>, String> {
    let inv_text = std::fs::read_to_string(repo.join("docs/work-inventory.json")).map_err(|e| format!("docs/work-inventory.json: {e}"))?;
    let inv: InventoryFile = serde_json::from_str(&inv_text).map_err(|e| format!("docs/work-inventory.json: {e}"))?;
    let entries = walk_corpus(repo);
    let mut by_line: BTreeMap<(String, String, usize), usize> = BTreeMap::new();
    let mut by_key: BTreeMap<(String, String, String), usize> = BTreeMap::new();
    for (i, e) in entries.iter().enumerate() {
        if let Some(l) = e.line
            && !e.basename.is_empty()
        {
            by_line.entry((e.book.clone(), e.basename.clone(), l)).or_insert(i);
        }
        by_key.entry((e.book.clone(), e.kind.clone(), e.slug.clone())).or_insert(i);
    }
    let mut out = Vec::with_capacity(inv.units.len());
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
            .copied();
        let rec = idx.and_then(|i| record_from_json(u, &entries[i].path));
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
                joined,
            }
        }));
    }
    Ok(out)
}

// ---- indexes ----------------------------------------------------------------------------------

/// Build the corpus-wide index and every record's closure.
pub fn build_index(tree: &PinnedTree, records: Vec<RecordRef>) -> (CorpusIndex, Vec<Closure>) {
    let mut index = CorpusIndex::default();
    let mut closures: Vec<Closure> = Vec::with_capacity(records.len());
    for r in &records {
        let closure = if r.rel_path.is_empty() {
            Closure::default()
        } else {
            tree.closure(&r.rel_path, r.line, r.shipped_tokens.as_deref(), &r.category, &r.key, r.copy_base_key.as_deref())
        };
        let cat_u = r.category.to_ascii_uppercase();
        let key_u = r.key.to_ascii_uppercase();
        let name_u = r.name.to_ascii_uppercase();
        index.by_cat_key.entry((cat_u.clone(), key_u.clone())).or_insert(r.id.clone());
        index.by_cat_name.entry((cat_u.clone(), name_u.clone())).or_insert(r.id.clone());
        index.by_kind_name.entry((r.kind.clone(), key_u.clone())).or_insert(r.id.clone());
        index.by_kind_name.entry((r.kind.clone(), name_u.clone())).or_insert(r.id.clone());
        if r.kind == "class" {
            index.classes.entry(key_u.clone()).or_insert((r.id.clone(), slug(&r.key)));
            index.classes.entry(name_u.clone()).or_insert((r.id.clone(), slug(&r.key)));
        }
        if r.kind == "skill" {
            index.skills.entry(name_u.clone()).or_insert(slug(&r.name));
            index.skills.entry(key_u.clone()).or_insert(slug(&r.name));
        }
        let owning_class: Option<ClassId> = match r.kind.as_str() {
            "class" => Some(slug(&r.key)),
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
        closures.push(closure);
    }
    index.records = records;
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
fn description_only_rules(r: &RecordRef) -> Option<Vec<SheetRule>> {
    let text = prose::decode_entities(r.description.as_deref()?.trim());
    if text.is_empty() || prose::pi_hit(&text).is_some() || r.license_pi || r.pi_fields.iter().any(|f| f == "description") {
        return None;
    }
    if FORBIDDEN_LITERALS.iter().any(|lit| text.contains(lit)) || has_pre_head(&text) || text.contains("%1") {
        return None;
    }
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

pub fn run(tree: &PinnedTree, index: &CorpusIndex, closures: &[Closure]) -> Run {
    let mut files: BTreeMap<String, Vec<SheetRule>> = BTreeMap::new();
    let mut grants_out: BTreeMap<RuleId, Vec<Grant>> = BTreeMap::new();
    let mut contribs: BTreeMap<VarId, (String, Vec<VarContribution>)> = BTreeMap::new();
    let mut declares: BTreeMap<VarId, (String, BTreeSet<RuleId>)> = BTreeMap::new();
    let mut var_names: BTreeMap<VarId, String> = BTreeMap::new();
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
        files.insert(rule_file_rel(&r.book, &r.kind, &r.id), c.rules);
    }
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
    let owned_rows: BTreeSet<RowRef> = index.row_owner.keys().copied().collect();
    let mut vars: BTreeMap<VarId, VarTable> = BTreeMap::new();
    for id in &referenced {
        let name = var_names.get(id).cloned().or_else(|| contribs.get(id).map(|(n, _)| n.clone())).or_else(|| declares.get(id).map(|(n, _)| n.clone()));
        let Some(name) = name else { continue };
        var_names.entry(id.clone()).or_insert(name.clone());
        let declared_by: Vec<RuleId> = declares.get(id).map(|(_, s)| s.iter().cloned().collect()).unwrap_or_default();
        let contributions: Vec<VarContribution> = contribs.get(id).map(|(_, v)| v.clone()).unwrap_or_default();
        let outside: Vec<String> = tree.variable_rows(&name).into_iter().filter(|r| !owned_rows.contains(r)).map(|r| tree.cite(r)).collect();
        vars.insert(id.clone(), VarTable { var: id.clone(), declared_by, contributions, provenance: VarProvenance { outside_corpus_rows: outside } });
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
    std::fs::write(out_dir.join("GENERATED"), b"GENERATED FILE TREE -- written by `cargo run --locked --bin sheet_rule_convert`; regenerated whole; never hand-edited.\n")?;
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
    let dir = repo.join("scripts/oracle_harness");
    std::fs::create_dir_all(&dir)?;
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
    problems.truncate(200);
    if problems.is_empty() { Ok(()) } else { Err(problems) }
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
    let c = convert::convert_record(&tree, &index, &index.records[pos], &closures[pos]);
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
            .expect("data/sheet_rules/_refused.json is generated (cargo run --locked --bin sheet_rule_convert)");
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

    /// Every degraded record reached the sheet, and prints its WORDS: the converter could not
    /// write one of its terms, so it writes none of them as a number
    /// (`decisions.md` §1 form 3). A partly-read magnitude never reaches a sheet total.
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
        still_refused.truncate(10);
        missing.truncate(10);
        numbered.truncate(10);
        assert!(still_refused.is_empty(), "degraded records must still convert: {still_refused:?}");
        assert!(missing.is_empty(), "degraded records must have a rule file: {missing:?}");
        assert!(numbered.is_empty(), "a degraded record must print words, never a partly-read number: {numbered:?}");
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
}
