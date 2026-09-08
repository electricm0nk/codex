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
//! - `_defects/<kind>.json` -- converter defect lists (unresolved references, undefined
//!   variables, choice markers without a choice, grants by type);
//! - `_report.json` -- the run summary (`records=... converted=... refused=...`).
//!
//! Token types in `_refused.json`, `_defects/` and `_report.json` are written with their `:`
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
    let shipped_tokens: Option<Vec<(String, String)>> = data.get("raw_tokens").and_then(|v| v.as_array()).map(|arr| {
        arr.iter()
            .filter_map(|t| Some((t.get("key")?.as_str()?.to_string(), t.get("value")?.as_str()?.to_string())))
            .collect()
    });
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

/// The population: every inventory unit, joined to its corpus record where one exists. A unit
/// with no record comes back with an empty `rel_path` and is refused as `no_corpus_record`.
pub fn load_population(repo: &Path) -> Result<Vec<RecordRef>, String> {
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
        out.push(rec.unwrap_or_else(|| RecordRef {
            id: u.id.clone(),
            book: u.book.clone(),
            kind: u.kind.clone(),
            name: u.name.clone(),
            key: u.corpus_key.clone().unwrap_or_default(),
            category: String::new(),
            type_facet: String::new(),
            rel_path: String::new(),
            line: 0,
            shipped_tokens: None,
            prerequisites: Vec::new(),
            copy_base_key: None,
            license_pi: false,
            pi_fields: Vec::new(),
            description: None,
            class_name: None,
            joined: false,
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

/// Everything one conversion pass produces, in memory.
pub struct Run {
    /// rule file path (relative to the output dir) -> the rules in it, sorted by id.
    pub files: BTreeMap<String, Vec<SheetRule>>,
    pub vars: BTreeMap<VarId, VarTable>,
    pub var_names: BTreeMap<VarId, String>,
    pub refused: RefusedReport,
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
pub fn run(tree: &PinnedTree, index: &CorpusIndex, closures: &[Closure]) -> Run {
    let mut files: BTreeMap<String, Vec<SheetRule>> = BTreeMap::new();
    let mut grants_out: BTreeMap<RuleId, Vec<Grant>> = BTreeMap::new();
    let mut contribs: BTreeMap<VarId, (String, Vec<VarContribution>)> = BTreeMap::new();
    let mut declares: BTreeMap<VarId, (String, BTreeSet<RuleId>)> = BTreeMap::new();
    let mut var_names: BTreeMap<VarId, String> = BTreeMap::new();
    let mut defects: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut refused = RefusedReport::default();
    let mut report = Report { oracle_pin: oracle_pin(), converter_version: convert::CONVERTER_VERSION.into(), ..Default::default() };
    let mut converted_ids: Vec<(String, String, String)> = Vec::new();
    for (r, closure) in index.records.iter().zip(closures.iter()) {
        report.records += 1;
        let kc = report.by_kind.entry(r.kind.clone()).or_default();
        kc.records += 1;
        if !r.joined || r.rel_path.is_empty() {
            let tt = if r.joined { "no_source_row".to_string() } else { "no_corpus_record".to_string() };
            *refused.by_token_type.entry(tt.clone()).or_default() += 1;
            refused.entries.push(RefusedRecord { id: r.id.clone(), book: r.book.clone(), kind: r.kind.clone(), token_types: vec![tt] });
            kc.refused += 1;
            continue;
        }
        if tree.file_index(&r.rel_path).is_none() && r.prerequisites.is_empty() && r.shipped_tokens.as_ref().is_none_or(|t| t.is_empty()) {
            let tt = "no_source_row".to_string();
            *refused.by_token_type.entry(tt.clone()).or_default() += 1;
            refused.entries.push(RefusedRecord { id: r.id.clone(), book: r.book.clone(), kind: r.kind.clone(), token_types: vec![tt] });
            kc.refused += 1;
            continue;
        }
        let c = convert::convert_record(tree, index, r, closure);
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
    Run { files, vars, var_names, refused, defects, report }
}

// ---- output -----------------------------------------------------------------------------------

/// JSON with `:` escaped inside strings, for the ledgers that name token types.
fn json_colon_escaped<T: Serialize>(value: &T) -> Vec<u8> {
    let text = serde_json::to_string_pretty(value).unwrap();
    // Escape ':' only inside string literals: walk the text tracking string state.
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
    out.push('\n');
    out.into_bytes()
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
    let records = load_population(repo)?;
    let (index, closures) = build_index(&tree, records);
    let run = run(&tree, &index, &closures);
    Ok((run, index))
}

/// Convert one unit by id (for tests and spot checks): the whole index is built, one record
/// is converted, and its `Converted` is returned with the closure rows it read.
pub fn convert_one(repo: &Path, unit_id: &str) -> Result<(convert::Converted, Vec<String>), String> {
    let tree = PinnedTree::load(&closure::corpus_root())?;
    let records = load_population(repo)?;
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
