//! Settles equipment prices into the converted corpus and the Market's tables (SD-41 E1.9,
//! `docs/release/SD-41-gear-and-equipment-sets/decisions.md §26`).
//!
//! The operator: "fix the converter so prices derive as PCGen's data defines them; prices still
//! missing come from d20pfsrd.com into ONE checked-in list with the page URL per item". So, for
//! every converted equipment record (not an eqmod) whose `cost_gp` is missing, zero or negative:
//!
//! 1. **The converter price.** Its cited LST row (`source.path` + `source.line` in the pinned
//!    oracle) and, for a `.COPY=` row, the base row it copies, priced by
//!    [`ir_converter::equipment_price_gp`] (base `COST` + every eqmod's cost). Eqmods resolve to
//!    the converted eqmod records (same book first); `%SPELLCOST` to a converted spell's `COST`.
//! 2. **The cited list.** A key still unpriced takes its row in
//!    `data/price_overrides/d20pfsrd-prices.json` (`[{ key, price_gp, url, fetched_at }]`).
//!
//! It writes the price to the record's `data.cost_gp` (the one field, edited in place: the file's
//! key order, license and PI stamps stay byte for byte) and to the Market's `data/rules_tables`
//! rows of that book and key whose price is missing, zero or negative (template rows, `name` =
//! `<key> (Base)` with no price, stay as they are: the Market hides them; eqmod rows stay too). A positive price is
//! never replaced. `gen_settled_corpus` then carries the price into `_settled/equipment.json`
//! (`corpus_equipment_json` reads the settled `cost_gp` over a non-positive `COST` literal).
//!
//! ```text
//! cargo run --locked -p codex-ingest --bin settle_equipment_prices              # write
//! cargo run --locked -p codex-ingest --bin settle_equipment_prices -- --check   # exit 1 if a write is due
//! ```
//! Run from the repo root. The oracle is `$PCGEN_CORPUS_ROOT` (default `$HOME/workspace/repos/pcgen/data`).

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use codex_ingest::pcgen_import::ir_converter::{equipment_price_gp, EqmodPrice, Unpriced};
use serde_json::Value;

type Tokens = Vec<(String, String)>;

const D20PFSRD_PRICES: &str = "data/price_overrides/d20pfsrd-prices.json";

fn pcgen_corpus_root() -> PathBuf {
    match std::env::var("PCGEN_CORPUS_ROOT") {
        Ok(root) => PathBuf::from(root),
        Err(_) => PathBuf::from(std::env::var("HOME").expect("HOME must be set")).join("workspace/repos/pcgen/data"),
    }
}

fn json_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    for path in paths {
        if path.is_dir() {
            json_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "json") {
            out.push(path);
        }
    }
}

struct Record {
    path: PathBuf,
    book: String,
    text: String,
    doc: Value,
}

impl Record {
    fn data(&self) -> &Value {
        &self.doc["data"]
    }
    fn key(&self) -> &str {
        self.data()["key"].as_str().unwrap_or("")
    }
    fn is_eqmod(&self) -> bool {
        self.data()["category"].as_str().is_some_and(|c| c.eq_ignore_ascii_case("equipmods"))
    }
}

fn load_kind(kind: &str) -> Vec<Record> {
    let mut books: Vec<PathBuf> = std::fs::read_dir("data/corpus")
        .expect("run from the repo root: data/corpus must exist")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    books.sort();
    let mut out = Vec::new();
    for book in books {
        let mut files = Vec::new();
        json_files(&book.join(kind), &mut files);
        for path in files {
            let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            let doc: Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            let name = book.file_name().unwrap().to_string_lossy().to_string();
            out.push(Record { path, book: name, text, doc });
        }
    }
    out
}

fn raw_tokens(data: &Value) -> Tokens {
    data["raw_tokens"]
        .as_array()
        .map(|ts| {
            ts.iter()
                .filter_map(|t| Some((t["key"].as_str()?.to_string(), t["value"].as_str()?.to_string())))
                .collect()
        })
        .unwrap_or_default()
}

fn bonus_chains(data: &Value) -> Vec<Vec<String>> {
    data["raw_bonus_chains"]
        .as_array()
        .map(|cs| {
            cs.iter()
                .filter_map(|c| c["qualifiers"].as_array())
                .map(|q| q.iter().filter_map(|s| s.as_str().map(str::to_string)).collect())
                .collect()
        })
        .unwrap_or_default()
}

/// The `KEY:VALUE` fields of one raw LST row, after its first (name) column.
fn line_tokens(line: &str) -> Tokens {
    line.split('\t')
        .skip(1)
        .filter_map(|field| field.trim().split_once(':'))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

/// The pinned oracle's equipment LST files, read once each.
struct Oracle {
    root: PathBuf,
    files: Vec<String>,
    lines: HashMap<String, Vec<String>>,
}

impl Oracle {
    fn new(root: PathBuf) -> Self {
        fn walk(dir: &Path, root: &Path, out: &mut Vec<String>) {
            let Ok(entries) = std::fs::read_dir(dir) else { return };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, root, out);
                } else if path.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.contains("equip") && n.ends_with(".lst")) {
                    out.push(path.strip_prefix(root).unwrap().to_string_lossy().to_string());
                }
            }
        }
        let mut files = Vec::new();
        walk(&root.join("pathfinder"), &root, &mut files);
        files.sort();
        Oracle { root, files, lines: HashMap::new() }
    }

    fn lines(&mut self, rel: &str) -> Option<&Vec<String>> {
        if !self.lines.contains_key(rel) {
            let bytes = std::fs::read(self.root.join(rel)).ok()?;
            let text = String::from_utf8_lossy(&bytes);
            self.lines.insert(rel.to_string(), text.lines().map(str::to_string).collect());
        }
        self.lines.get(rel)
    }

    /// The plain (not `.COPY=` / `.MOD` / `.FORGET`) row whose `KEY` (else first column) is
    /// `identity`: the citing file first, then its directory, then every other equipment file.
    fn base_row(&mut self, cited: &str, identity: &str) -> Option<Tokens> {
        let dir = Path::new(cited).parent().map(Path::to_path_buf);
        let mut order = vec![cited.to_string()];
        let (same, other): (Vec<String>, Vec<String>) = self
            .files
            .iter()
            .filter(|f| f.as_str() != cited)
            .cloned()
            .partition(|f| Path::new(f).parent().map(Path::to_path_buf) == dir);
        order.extend(same);
        order.extend(other);
        for file in order {
            let Some(lines) = self.lines(&file) else { continue };
            for line in lines {
                let first = line.split('\t').next().unwrap_or("").trim();
                if first.is_empty() || first.starts_with('#') || first.contains(".COPY=") || first.contains(".MOD") || first.contains(".FORGET") {
                    continue;
                }
                let tokens = line_tokens(line);
                let id = tokens.iter().find(|(k, _)| k == "KEY").map_or(first, |(_, v)| v.as_str());
                if id == identity {
                    return Some(tokens);
                }
            }
        }
        None
    }
}

#[derive(Default)]
struct Report {
    by_outcome: BTreeMap<String, usize>,
    unpriced: BTreeMap<String, usize>,
}

impl Report {
    fn count(&mut self, outcome: &str) {
        *self.by_outcome.entry(outcome.to_string()).or_default() += 1;
    }
}

/// The cited d20pfsrd list: every row must carry a positive price, a d20pfsrd URL and a fetch time.
fn d20pfsrd_prices() -> BTreeMap<String, f64> {
    let Ok(text) = std::fs::read_to_string(D20PFSRD_PRICES) else {
        panic!("{D20PFSRD_PRICES} must exist (an empty list is `[]`)")
    };
    parse_d20pfsrd_prices(&text).unwrap_or_else(|e| panic!("{D20PFSRD_PRICES}: {e}"))
}

fn parse_d20pfsrd_prices(text: &str) -> Result<BTreeMap<String, f64>, String> {
    let rows: Vec<Value> = serde_json::from_str(text).map_err(|e| e.to_string())?;
    let mut out = BTreeMap::new();
    for row in rows {
        let key = row["key"].as_str().ok_or_else(|| format!("a row has no key: {row}"))?;
        let price = row["price_gp"].as_f64().filter(|p| *p > 0.0);
        let url_ok = row["url"].as_str().is_some_and(|u| u.starts_with("https://www.d20pfsrd.com/"));
        let fetched = row["fetched_at"].as_str().is_some_and(|f| !f.trim().is_empty());
        match price {
            Some(price) if url_ok && fetched => {
                out.insert(key.to_string(), price);
            }
            _ => return Err(format!("row {key:?} needs a positive price_gp, a d20pfsrd url and fetched_at: {row}")),
        }
    }
    Ok(out)
}

fn positive(cost: Option<f64>) -> bool {
    cost.is_some_and(|c| c > 0.0)
}

/// Whether `new` replaces a missing, zero or negative `old`: a positive price always; zero only over
/// a negative (an eqmod-offset literal settles at 0, never below).
fn replaces(old: Option<f64>, new: f64) -> bool {
    !positive(old) && (new > 0.0 || old.is_some_and(|o| o < 0.0) && new >= 0.0) && old != Some(new)
}

fn number(value: f64) -> String {
    serde_json::to_string(&Value::from(value)).unwrap()
}

/// `text` with its single `"cost_gp": …` line set to `cost`; the parsed result must differ from
/// `doc` in `data.cost_gp` alone.
fn set_record_cost(text: &str, doc: &Value, cost: f64, path: &Path) -> String {
    let hits: Vec<usize> = text.lines().enumerate().filter(|(_, l)| l.trim_start().starts_with("\"cost_gp\":")).map(|(i, _)| i).collect();
    assert_eq!(hits.len(), 1, "{}: expected one cost_gp line", path.display());
    let mut out = String::with_capacity(text.len() + 16);
    for (i, line) in text.split_inclusive('\n').enumerate() {
        if i == hits[0] {
            let indent = &line[..line.len() - line.trim_start().len()];
            let comma = if line.trim_end().ends_with(',') { "," } else { "" };
            let newline = if line.ends_with('\n') { "\n" } else { "" };
            out.push_str(&format!("{indent}\"cost_gp\": {}{comma}{newline}", number(cost)));
        } else {
            out.push_str(line);
        }
    }
    let mut expected = doc.clone();
    expected["data"]["cost_gp"] = Value::from(cost);
    let parsed: Value = serde_json::from_str(&out).unwrap_or_else(|e| panic!("{}: edit broke the JSON: {e}", path.display()));
    assert_eq!(parsed, expected, "{}: the edit moved more than data.cost_gp", path.display());
    out
}

/// `text` (a `codex.rules_tables/1` table) with the `cost_gp` of each row whose key is in `costs` set;
/// the parsed result must differ from `doc` in those rows' `cost_gp` alone.
fn set_table_costs(text: &str, doc: &Value, costs: &BTreeMap<usize, f64>, path: &Path) -> String {
    let mut expected = doc.clone();
    for (index, cost) in costs {
        expected["rows"][*index]["cost_gp"] = Value::from(*cost);
    }
    let mut out = String::with_capacity(text.len() + 64);
    let mut row: Option<usize> = None;
    for line in text.split_inclusive('\n') {
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();
        if indent == 4 && trimmed.starts_with('{') {
            row = Some(row.map_or(0, |r| r + 1));
        }
        match row.and_then(|r| costs.get(&r)) {
            Some(cost) if indent == 6 && trimmed.starts_with("\"cost_gp\":") => {
                let comma = if line.trim_end().ends_with(',') { "," } else { "" };
                let newline = if line.ends_with('\n') { "\n" } else { "" };
                out.push_str(&format!("{}\"cost_gp\": {}{comma}{newline}", &line[..indent], number(*cost)));
            }
            _ => out.push_str(line),
        }
    }
    let parsed: Value = serde_json::from_str(&out).unwrap_or_else(|e| panic!("{}: edit broke the JSON: {e}", path.display()));
    assert_eq!(parsed, expected, "{}: the edit moved more than the named rows' cost_gp", path.display());
    out
}

/// The corpus book a Market table belongs to.
fn table_book(rel: &Path) -> Option<String> {
    let first = rel.components().next()?.as_os_str().to_string_lossy().to_string();
    Some(match first.as_str() {
        "crb" => "core_rulebook".to_string(),
        "apg" => "advanced_players_guide".to_string(),
        "acg" => "advanced_class_guide".to_string(),
        "beastiary1" => "beastiary".to_string(),
        "equipment_gap_tables" => rel.file_stem()?.to_string_lossy().strip_suffix("_GAP_ROWS")?.to_ascii_lowercase(),
        other => other.to_string(),
    })
}

fn main() {
    let check = std::env::args().any(|a| a == "--check");
    let mut oracle = Oracle::new(pcgen_corpus_root());
    let cited = d20pfsrd_prices();

    let mut equipment = load_kind("equipment");
    let mut eqmods_by_book: HashMap<(String, String), EqmodPrice> = HashMap::new();
    let mut eqmods: HashMap<String, EqmodPrice> = HashMap::new();
    for r in equipment.iter().filter(|r| r.is_eqmod()) {
        let price = EqmodPrice::from_tokens(&raw_tokens(r.data()), &bonus_chains(r.data()));
        eqmods_by_book.entry((r.book.clone(), r.key().to_string())).or_insert_with(|| price.clone());
        eqmods.entry(r.key().to_string()).or_insert(price);
    }
    let mut spell_costs: HashMap<String, f64> = HashMap::new();
    for s in load_kind("spell") {
        let tokens = raw_tokens(&s.doc["data"]);
        if let Some(cost) = tokens.iter().find(|(k, _)| k == "COST").and_then(|(_, v)| v.trim().parse::<f64>().ok()) {
            for name in [&s.doc["data"]["key"], &s.doc["data"]["name"]].into_iter().filter_map(Value::as_str) {
                spell_costs.entry(name.to_string()).or_insert(cost);
            }
        }
    }

    let mut report = Report::default();
    let mut writes = 0usize;
    // The settled price per (book, key), read by the table pass below.
    let mut settled: HashMap<(String, String), f64> = HashMap::new();
    for record in equipment.iter_mut().filter(|r| !r.is_eqmod()) {
        let old = record.data()["cost_gp"].as_f64();
        if positive(old) {
            continue;
        }
        let key = record.key().to_string();
        let source = &record.doc["source"];
        let mut new = None;
        match (source["path"].as_str(), source["line"].as_u64()) {
            (Some(rel), Some(line)) => {
                let rel = rel.to_string();
                let row = oracle.lines(&rel).and_then(|ls| ls.get(line as usize - 1)).cloned();
                match row {
                    None => report.count("citation_miss"),
                    Some(row) => {
                        let first = row.split('\t').next().unwrap_or("").trim().to_string();
                        let base = first.split_once(".COPY=").map(|(b, _)| b.to_string());
                        let base_tokens = match &base {
                            Some(identity) => match oracle.base_row(&rel, identity) {
                                Some(t) => Some(t),
                                None => {
                                    report.count("copy_base_missing");
                                    *report.unpriced.entry("copy_base_missing".to_string()).or_default() += 1;
                                    continue;
                                }
                            },
                            None => None,
                        };
                        let book = record.book.clone();
                        let lookup = |k: &str| eqmods_by_book.get(&(book.clone(), k.to_string())).or_else(|| eqmods.get(k)).cloned();
                        let spell = |name: &str| spell_costs.get(name).copied().unwrap_or(0.0);
                        match equipment_price_gp(&line_tokens(&row), base_tokens.as_deref(), &lookup, &spell) {
                            Ok(price) => new = Some(price),
                            Err(why) => {
                                let class = match why {
                                    Unpriced::ItemCostFormula(_) => "item_cost_formula",
                                    Unpriced::EqmodMissing(_) => "eqmod_missing",
                                    Unpriced::Formula(_) => "formula_not_evaluated",
                                    Unpriced::Prerequisite(_) => "itemcost_prerequisite_not_evaluated",
                                };
                                *report.unpriced.entry(class.to_string()).or_default() += 1;
                            }
                        }
                    }
                }
            }
            _ => report.count("no_lst_citation"),
        }
        let mut chosen = new.filter(|p| replaces(old, *p));
        if !chosen.is_some_and(|p| p > 0.0)
            && let Some(price) = cited.get(&key)
        {
            chosen = Some(*price);
            report.count("priced_by_d20pfsrd_list");
        }
        match chosen {
            Some(price) => {
                if new == Some(price) {
                    report.count(if price > 0.0 { "priced_by_converter" } else { "negative_settled_to_zero" });
                }
                settled.insert((record.book.clone(), key.clone()), price);
                let text = set_record_cost(&record.text, &record.doc, price, &record.path);
                writes += 1;
                if !check {
                    std::fs::write(&record.path, &text).unwrap_or_else(|e| panic!("{}: {e}", record.path.display()));
                }
                record.doc["data"]["cost_gp"] = Value::from(price);
                record.text = text;
            }
            None => report.count("still_unpriced"),
        }
    }
    // Records already settled by an earlier run: the tables read their price too.
    for record in equipment.iter().filter(|r| !r.is_eqmod()) {
        if let Some(cost) = record.data()["cost_gp"].as_f64() {
            settled.entry((record.book.clone(), record.key().to_string())).or_insert(cost);
        }
    }

    let mut tables = Vec::new();
    json_files(Path::new("data/rules_tables"), &mut tables);
    let mut table_rows = 0usize;
    for path in tables {
        let text = std::fs::read_to_string(&path).unwrap();
        let doc: Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        let Some(rows) = doc["rows"].as_array() else { continue };
        let rel = path.strip_prefix("data/rules_tables").unwrap().to_path_buf();
        let Some(book) = table_book(&rel) else { continue };
        let mut costs = BTreeMap::new();
        for (index, row) in rows.iter().enumerate() {
            let (Some(key), Some(fields)) = (row["key"].as_str(), row.as_object()) else { continue };
            if !fields.contains_key("cost_gp") {
                continue;
            }
            let old = row["cost_gp"].as_f64();
            let template = old.is_none() && row["name"].as_str() == Some(&format!("{key} (Base)"));
            // An eqmod row is not an item (its price is Buy Custom's, `decisions.md §30` Q6), even
            // when an item shares its key (UE `Masterwork Tool`).
            let eqmod = row["category"].as_str().is_some_and(|c| c.eq_ignore_ascii_case("equipmods"));
            if template || eqmod {
                continue;
            }
            if let Some(price) = settled.get(&(book.clone(), key.to_string()))
                && replaces(old, *price)
            {
                costs.insert(index, *price);
            }
        }
        if costs.is_empty() {
            continue;
        }
        table_rows += costs.len();
        writes += 1;
        let out = set_table_costs(&text, &doc, &costs, &path);
        if !check {
            std::fs::write(&path, out).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        }
    }

    println!("settle_equipment_prices: records {:?}", report.by_outcome);
    println!("settle_equipment_prices: not evaluated by the converter {:?}", report.unpriced);
    println!("settle_equipment_prices: table rows priced {table_rows}; files {}", writes);
    if check && writes > 0 {
        eprintln!("settle_equipment_prices --check: {writes} file(s) would change");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d20pfsrd_list_reads_cited_rows_and_refuses_uncited_ones() {
        assert_eq!(parse_d20pfsrd_prices("[]"), Ok(BTreeMap::new()));
        let row = r#"[{"key":"Bomb","price_gp":150,"url":"https://www.d20pfsrd.com/x/bomb/","fetched_at":"2026-11-13"}]"#;
        assert_eq!(parse_d20pfsrd_prices(row), Ok(BTreeMap::from([("Bomb".to_string(), 150.0)])));
        for bad in [
            r#"[{"key":"Bomb","price_gp":150,"url":"https://example.com/bomb","fetched_at":"2026-11-13"}]"#,
            r#"[{"key":"Bomb","price_gp":0,"url":"https://www.d20pfsrd.com/x/","fetched_at":"2026-11-13"}]"#,
            r#"[{"key":"Bomb","price_gp":150,"url":"https://www.d20pfsrd.com/x/"}]"#,
        ] {
            assert!(parse_d20pfsrd_prices(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn a_price_replaces_only_a_missing_zero_or_negative_one() {
        assert!(replaces(None, 5.0));
        assert!(replaces(Some(0.0), 5.0));
        assert!(replaces(Some(-150.0), 0.0));
        assert!(!replaces(None, 0.0), "no zero claimed where nothing was stated");
        assert!(!replaces(Some(10.0), 20.0), "a positive price is never replaced");
    }

    #[test]
    fn the_record_edit_moves_cost_gp_alone_and_keeps_key_order() {
        let text = "{\n  \"population\": \"in_scope\",\n  \"data\": {\n    \"key\": \"X\",\n    \"cost_gp\": null,\n    \"weight_lbs\": 1.0\n  },\n  \"license\": \"OGL\"\n}\n";
        let doc: Value = serde_json::from_str(text).unwrap();
        let out = set_record_cost(text, &doc, 10_200.0, Path::new("x.json"));
        assert_eq!(out, text.replace("\"cost_gp\": null", "\"cost_gp\": 10200.0"));
    }
}
