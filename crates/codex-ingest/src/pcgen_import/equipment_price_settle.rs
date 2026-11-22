//! The converter's equipment-price derivation, shared by `settle_equipment_prices` (which writes
//! a settled `cost_gp`) and `corpus_literal_sweep` (which re-derives it to prove the written value
//! is the converter's and not a hand edit). SD-41 E1.9 / E5.1, `docs/release/SD-41-gear-and-equipment-sets/decisions.md §26`.
//!
//! One derivation, two readers: a price the sweep cannot find as a flat `COST:` token in the cited
//! row's closure is accepted only when [`PriceContext::converter_price`] reproduces it from the
//! same oracle row, the same eqmod records and the same spell costs the settle bin used.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use serde_json::Value;

use super::ir_converter::{equipment_price_gp, EqmodPrice, Unpriced};

pub type Tokens = Vec<(String, String)>;

/// Every `*.json` under `dir`, sorted.
pub fn json_files(dir: &Path, out: &mut Vec<PathBuf>) {
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

/// One converted corpus record: its file, its book directory name, its bytes and its parse.
pub struct Record {
    pub path: PathBuf,
    pub book: String,
    pub text: String,
    pub doc: Value,
}

impl Record {
    pub fn data(&self) -> &Value {
        &self.doc["data"]
    }
    pub fn key(&self) -> &str {
        self.data()["key"].as_str().unwrap_or("")
    }
    pub fn is_eqmod(&self) -> bool {
        self.data()["category"].as_str().is_some_and(|c| c.eq_ignore_ascii_case("equipmods"))
    }
}

/// Every `<corpus_dir>/<book>/<kind>/**/*.json` record, books sorted.
pub fn load_kind(corpus_dir: &Path, kind: &str) -> Result<Vec<Record>, String> {
    let mut books: Vec<PathBuf> = std::fs::read_dir(corpus_dir)
        .map_err(|e| format!("{}: {e}", corpus_dir.display()))?
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
            let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            let doc: Value = serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
            let name = book.file_name().unwrap().to_string_lossy().to_string();
            out.push(Record { path, book: name, text, doc });
        }
    }
    Ok(out)
}

pub fn raw_tokens(data: &Value) -> Tokens {
    data["raw_tokens"]
        .as_array()
        .map(|ts| {
            ts.iter()
                .filter_map(|t| Some((t["key"].as_str()?.to_string(), t["value"].as_str()?.to_string())))
                .collect()
        })
        .unwrap_or_default()
}

pub fn bonus_chains(data: &Value) -> Vec<Vec<String>> {
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
pub fn line_tokens(line: &str) -> Tokens {
    line.split('\t')
        .skip(1)
        .filter_map(|field| field.trim().split_once(':'))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

/// The pinned oracle's equipment LST files, read once each.
pub struct Oracle {
    root: PathBuf,
    files: Vec<String>,
    lines: HashMap<String, Vec<String>>,
}

impl Oracle {
    pub fn new(root: PathBuf) -> Self {
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

    pub fn lines(&mut self, rel: &str) -> Option<&Vec<String>> {
        if !self.lines.contains_key(rel) {
            let bytes = std::fs::read(self.root.join(rel)).ok()?;
            let text = String::from_utf8_lossy(&bytes);
            self.lines.insert(rel.to_string(), text.lines().map(str::to_string).collect());
        }
        self.lines.get(rel)
    }

    /// The plain (not `.COPY=` / `.MOD` / `.FORGET`) row whose `KEY` (else first column) is
    /// `identity`: the citing file first, then its directory, then every other equipment file.
    pub fn base_row(&mut self, cited: &str, identity: &str) -> Option<Tokens> {
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

/// Parses the cited d20pfsrd list: every row needs a positive price, a d20pfsrd URL and a fetch time.
pub fn parse_d20pfsrd_prices(text: &str) -> Result<BTreeMap<String, f64>, String> {
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

/// What the converter says about one record's cited row.
#[derive(Debug, Clone, PartialEq)]
pub enum ConverterPrice {
    Priced(f64),
    /// The record cites no `.lst` file and line.
    NoLstCitation,
    /// The cited line is not in the oracle file.
    CitationMiss,
    /// The row `.COPY=`s a base the oracle's equipment files do not hold.
    CopyBaseMissing,
    /// The converter does not evaluate this row; the class names why.
    Unpriced(&'static str),
}

/// The eqmod and spell-cost lookups the price formulas need, built from the converted records.
pub struct PriceContext {
    eqmods_by_book: HashMap<(String, String), EqmodPrice>,
    eqmods: HashMap<String, EqmodPrice>,
    spell_costs: HashMap<String, f64>,
}

impl PriceContext {
    pub fn new(equipment: &[Record], spells: &[Record]) -> Self {
        let mut eqmods_by_book: HashMap<(String, String), EqmodPrice> = HashMap::new();
        let mut eqmods: HashMap<String, EqmodPrice> = HashMap::new();
        for r in equipment.iter().filter(|r| r.is_eqmod()) {
            let price = EqmodPrice::from_tokens(&raw_tokens(r.data()), &bonus_chains(r.data()));
            eqmods_by_book.entry((r.book.clone(), r.key().to_string())).or_insert_with(|| price.clone());
            eqmods.entry(r.key().to_string()).or_insert(price);
        }
        let mut spell_costs: HashMap<String, f64> = HashMap::new();
        for s in spells {
            let tokens = raw_tokens(&s.doc["data"]);
            if let Some(cost) = tokens.iter().find(|(k, _)| k == "COST").and_then(|(_, v)| v.trim().parse::<f64>().ok()) {
                for name in [&s.doc["data"]["key"], &s.doc["data"]["name"]].into_iter().filter_map(Value::as_str) {
                    spell_costs.entry(name.to_string()).or_insert(cost);
                }
            }
        }
        PriceContext { eqmods_by_book, eqmods, spell_costs }
    }

    /// The price of the row cited as `rel` line `line` (1-indexed) for a record of `book`.
    pub fn converter_price(&self, oracle: &mut Oracle, book: &str, rel: Option<&str>, line: Option<u64>) -> ConverterPrice {
        let (Some(rel), Some(line)) = (rel, line.filter(|l| *l > 0)) else { return ConverterPrice::NoLstCitation };
        let Some(row) = oracle.lines(rel).and_then(|ls| ls.get(line as usize - 1)).cloned() else {
            return ConverterPrice::CitationMiss;
        };
        let first = row.split('\t').next().unwrap_or("").trim().to_string();
        let base = first.split_once(".COPY=").map(|(b, _)| b.to_string());
        let base_tokens = match &base {
            Some(identity) => match oracle.base_row(rel, identity) {
                Some(t) => Some(t),
                None => return ConverterPrice::CopyBaseMissing,
            },
            None => None,
        };
        let lookup = |k: &str| self.eqmods_by_book.get(&(book.to_string(), k.to_string())).or_else(|| self.eqmods.get(k)).cloned();
        let spell = |name: &str| self.spell_costs.get(name).copied().unwrap_or(0.0);
        match equipment_price_gp(&line_tokens(&row), base_tokens.as_deref(), &lookup, &spell) {
            Ok(price) => ConverterPrice::Priced(price),
            Err(Unpriced::ItemCostFormula(_)) => ConverterPrice::Unpriced("item_cost_formula"),
            Err(Unpriced::EqmodMissing(_)) => ConverterPrice::Unpriced("eqmod_missing"),
            Err(Unpriced::Formula(_)) => ConverterPrice::Unpriced("formula_not_evaluated"),
            Err(Unpriced::Prerequisite(_)) => ConverterPrice::Unpriced("itemcost_prerequisite_not_evaluated"),
        }
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
    fn a_row_with_no_citation_or_a_missing_line_has_no_converter_price() {
        let ctx = PriceContext::new(&[], &[]);
        let mut oracle = Oracle::new(std::env::temp_dir().join("no-such-oracle-root"));
        assert_eq!(ctx.converter_price(&mut oracle, "b", None, Some(3)), ConverterPrice::NoLstCitation);
        assert_eq!(ctx.converter_price(&mut oracle, "b", Some("pathfinder/x_equip.lst"), None), ConverterPrice::NoLstCitation);
        assert_eq!(ctx.converter_price(&mut oracle, "b", Some("pathfinder/x_equip.lst"), Some(3)), ConverterPrice::CitationMiss);
    }
}
