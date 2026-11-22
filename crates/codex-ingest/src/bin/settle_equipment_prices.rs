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
//! It also writes the Market price census's record side (`artifacts/epic_1/E1.8/corpus-price-causes.json`
//! in the SD-41 package): per key, the converter cause and eqmod cost kinds, so the desktop census
//! (`apps/desktop/src-tauri/tests/market_price_census.rs`) reads no ingest token (E1.15).
//!
//! ```text
//! cargo run --locked -p codex-ingest --bin settle_equipment_prices              # write
//! cargo run --locked -p codex-ingest --bin settle_equipment_prices -- --check   # exit 1 if a write is due
//! ```
//! Run from the repo root. The oracle is `$PCGEN_CORPUS_ROOT` (default `$HOME/workspace/repos/pcgen/data`).

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use codex_ingest::pcgen_import::equipment_price_settle::{
    json_files, load_kind, parse_d20pfsrd_prices, ConverterPrice, Oracle, PriceContext, Record,
};
use serde_json::Value;

const D20PFSRD_PRICES: &str = "data/price_overrides/d20pfsrd-prices.json";

fn pcgen_corpus_root() -> PathBuf {
    match std::env::var("PCGEN_CORPUS_ROOT") {
        Ok(root) => PathBuf::from(root),
        Err(_) => PathBuf::from(std::env::var("HOME").expect("HOME must be set")).join("workspace/repos/pcgen/data"),
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

/// The Market price census's record side (SD-41 E1.8's cause classifier, moved here by E1.15 so the
/// desktop census reads no ingest token, `decisions.md §17`): per corpus equipment key, its
/// records, the converter cause its first record shows and its eqmods' cost kinds.
const PRICE_CAUSES: &str = "docs/release/SD-41-gear-and-equipment-sets/artifacts/epic_1/E1.8/corpus-price-causes.json";

fn census_tokens<'a>(data: &'a Value, name: &str) -> Vec<&'a str> {
    data["raw_tokens"]
        .as_array()
        .map(|ts| ts.iter().filter(|t| t["key"] == name).filter_map(|t| t["value"].as_str()).collect())
        .unwrap_or_default()
}

/// The eqmod keys an `EQMOD:` token names: `.`-separated, each without its `|` arguments.
fn census_eqmod_keys(data: &Value) -> Vec<String> {
    census_tokens(data, "EQMOD")
        .iter()
        .flat_map(|v| v.split('.'))
        .map(|k| k.split('|').next().unwrap_or("").trim().to_string())
        .filter(|k| !k.is_empty())
        .collect()
}

/// The kind of cost an eqmod record states, from its `COST` token and its `ITEMCOST` bonuses, the
/// hardest first.
fn census_eqmod_cost_kind(record: Option<&Value>) -> &'static str {
    let Some(data) = record else { return "no_corpus_record" };
    let mut formulas: Vec<&str> = census_tokens(data, "COST");
    if let Some(chains) = data["raw_bonus_chains"].as_array() {
        for q in chains.iter().filter_map(|c| c["qualifiers"].as_array()) {
            if q.first().and_then(Value::as_str) == Some("ITEMCOST") {
                formulas.extend(q.get(2).and_then(Value::as_str));
            }
        }
    }
    let any = |needle: &str| formulas.iter().any(|f| f.contains(needle));
    if any("%SPELL") {
        "spell_effect_args"
    } else if any("%CHARGES") {
        "charges_arg"
    } else if any("%CHOICE") {
        "choice_arg"
    } else if any("PLUSTOTAL") {
        "enhancement_plus"
    } else if any("WT") || any("BASECOST") {
        "weight_or_basecost"
    } else if formulas.iter().all(|f| f.trim().parse::<f64>().is_ok()) {
        "numeric"
    } else {
        "other_formula"
    }
}

/// Why a record has no positive price. `no_raw_tokens` is refined by the census, which alone knows
/// the Market's served name (a composite resolved to its base serves another item's name).
fn census_cause(data: &Value) -> &'static str {
    if data["cost_gp"].as_f64().is_some_and(|c| c > 0.0) {
        return "table_disagrees_with_corpus";
    }
    if data["raw_tokens"].as_array().is_none_or(|t| t.is_empty()) {
        return "no_raw_tokens";
    }
    let cost = census_tokens(data, "COST");
    let has_eqmods = !census_eqmod_keys(data).is_empty();
    let literal = cost.first().and_then(|c| c.trim().parse::<f64>().ok());
    match (cost.first(), literal) {
        (Some(_), Some(c)) if c < 0.0 && has_eqmods => "cost_offset_by_eqmod",
        (Some(_), Some(c)) if c < 0.0 => "cost_literal_negative",
        _ if has_eqmods => "price_in_eqmods",
        (Some(_), None) => "cost_formula_unevaluated",
        (Some(_), Some(_)) => "cost_literal_zero",
        (None, _) => "no_cost_token",
    }
}

/// The census artifact over every equipment record (eqmods included), as settled by this run.
fn price_causes(equipment: &[Record]) -> String {
    let mut by_key: BTreeMap<&str, Vec<&Record>> = BTreeMap::new();
    for r in equipment {
        if let Some(key) = r.data()["key"].as_str() {
            by_key.entry(key).or_default().push(r);
        }
    }
    let first = |key: &str| by_key.get(key).map(|rs| rs[0].data());
    let out: BTreeMap<&str, Value> = by_key
        .iter()
        .map(|(key, rs)| {
            let data = rs[0].data();
            let eqmods = census_eqmod_keys(data);
            let kinds: BTreeMap<&String, &str> =
                eqmods.iter().map(|k| (k, census_eqmod_cost_kind(first(k)))).collect();
            let row = serde_json::json!({
                "corpus": rs.iter().map(|r| r.path.display().to_string()).collect::<Vec<_>>(),
                "cause": census_cause(data),
                "cost_token": census_tokens(data, "COST"),
                "eqmods": eqmods,
                "eqmod_cost_kinds": kinds,
            });
            (*key, row)
        })
        .collect();
    // One key per line: the file stays diffable at ~8k keys.
    let lines: Vec<String> = out.iter().map(|(k, v)| format!("  {}: {v}", Value::from(*k))).collect();
    format!("{{\n{}\n}}\n", lines.join(",\n"))
}

fn main() {
    let check = std::env::args().any(|a| a == "--check");
    let mut oracle = Oracle::new(pcgen_corpus_root());
    let cited = d20pfsrd_prices();

    let corpus = Path::new("data/corpus");
    let mut equipment = load_kind(corpus, "equipment").unwrap_or_else(|e| panic!("run from the repo root: {e}"));
    let prices = PriceContext::new(&equipment, &load_kind(corpus, "spell").unwrap_or_else(|e| panic!("{e}")));

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
        let new = match prices.converter_price(&mut oracle, &record.book, source["path"].as_str(), source["line"].as_u64()) {
            ConverterPrice::Priced(price) => Some(price),
            ConverterPrice::NoLstCitation => {
                report.count("no_lst_citation");
                None
            }
            ConverterPrice::CitationMiss => {
                report.count("citation_miss");
                None
            }
            // The `.COPY=` base is outside the oracle's files, so the converter cannot price the
            // row; the cited list below still can.
            ConverterPrice::CopyBaseMissing => {
                report.count("copy_base_missing");
                *report.unpriced.entry("copy_base_missing".to_string()).or_default() += 1;
                None
            }
            ConverterPrice::Unpriced(class) => {
                *report.unpriced.entry(class.to_string()).or_default() += 1;
                None
            }
        };
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

    let causes = price_causes(&equipment);
    if std::fs::read_to_string(PRICE_CAUSES).ok().as_deref() != Some(causes.as_str()) {
        writes += 1;
        if !check {
            std::fs::write(PRICE_CAUSES, causes).unwrap_or_else(|e| panic!("{PRICE_CAUSES}: {e}"));
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
    fn a_price_replaces_only_a_missing_zero_or_negative_one() {
        assert!(replaces(None, 5.0));
        assert!(replaces(Some(0.0), 5.0));
        assert!(replaces(Some(-150.0), 0.0));
        assert!(!replaces(None, 0.0), "no zero claimed where nothing was stated");
        assert!(!replaces(Some(10.0), 20.0), "a positive price is never replaced");
    }

    #[test]
    fn census_cause_and_eqmod_kind_read_the_record_tokens() {
        let rec = |cost: &str, eqmod: Option<&str>| {
            let mut ts = vec![serde_json::json!({"key": "COST", "value": cost})];
            ts.extend(eqmod.map(|e| serde_json::json!({"key": "EQMOD", "value": e})));
            serde_json::json!({"cost_gp": null, "raw_tokens": ts})
        };
        assert_eq!(census_cause(&serde_json::json!({"cost_gp": 5.0})), "table_disagrees_with_corpus");
        assert_eq!(census_cause(&serde_json::json!({"cost_gp": null, "raw_tokens": []})), "no_raw_tokens");
        assert_eq!(census_cause(&rec("-150", Some("MWORKW"))), "cost_offset_by_eqmod");
        assert_eq!(census_cause(&rec("-150", None)), "cost_literal_negative");
        assert_eq!(census_cause(&rec("0", Some("A.B|1"))), "price_in_eqmods");
        assert_eq!(census_cause(&rec("WT*2", None)), "cost_formula_unevaluated");
        assert_eq!(census_cause(&rec("0", None)), "cost_literal_zero");
        assert_eq!(census_eqmod_keys(&rec("0", Some("A.B|1|2. C"))), ["A", "B", "C"]);
        let chain = |formula: &str| {
            serde_json::json!({"raw_tokens": [], "raw_bonus_chains": [{"qualifiers": ["ITEMCOST", "TYPE.X", formula]}]})
        };
        assert_eq!(census_eqmod_cost_kind(Some(&chain("%CHOICE*2"))), "choice_arg");
        assert_eq!(census_eqmod_cost_kind(Some(&chain("PLUSTOTAL*1000"))), "enhancement_plus");
        assert_eq!(census_eqmod_cost_kind(Some(&rec("150", None))), "numeric");
        assert_eq!(census_eqmod_cost_kind(None), "no_corpus_record");
    }

    #[test]
    fn the_record_edit_moves_cost_gp_alone_and_keeps_key_order() {
        let text = "{\n  \"population\": \"in_scope\",\n  \"data\": {\n    \"key\": \"X\",\n    \"cost_gp\": null,\n    \"weight_lbs\": 1.0\n  },\n  \"license\": \"OGL\"\n}\n";
        let doc: Value = serde_json::from_str(text).unwrap();
        let out = set_record_cost(text, &doc, 10_200.0, Path::new("x.json"));
        assert_eq!(out, text.replace("\"cost_gp\": null", "\"cost_gp\": 10200.0"));
    }
}
