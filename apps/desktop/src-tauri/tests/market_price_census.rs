//! Price census of the Market (SD-41 E1.8, `decisions.md §26`): every row `list_market` serves
//! whose price is missing, zero or negative, by book and category, each with the converter cause
//! that left it so. Writes `docs/release/SD-41-gear-and-equipment-sets/artifacts/epic_1/E1.8/
//! market-prices.json`; E1.9 fixes the converter classes, E1.11 looks up what is left.
//!
//! This crate is a binary with no lib target, so the catalog modules are compiled into this test
//! by path: the census reads the Market through the app's own `list_market`, not a copy of its
//! filter. Their own `#[cfg(test)]` suites compile here too; run this file's tests with the
//! `market_price` name filter (`cargo test --test market_price_census market_price`).
#![allow(dead_code, unused_imports)]

#[path = "../src/authoring_workbench.rs"]
mod authoring_workbench;
#[path = "../src/corpus_fixtures.rs"]
mod corpus_fixtures;
#[path = "../src/corpus_full.rs"]
mod corpus_full;
#[path = "../src/equipment_catalog.rs"]
mod equipment_catalog;
#[path = "../src/feat_catalog.rs"]
mod feat_catalog;
#[path = "../src/race_catalog.rs"]
mod race_catalog;
#[path = "../src/race_trait_picker.rs"]
mod race_trait_picker;
#[path = "../src/spell_catalog.rs"]
mod spell_catalog;
use codex::rules_core::converted_prose;

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use equipment_catalog::{
    build_equipment_catalog, list_market, EquipmentCatalogEntryDto, MarketCategory, MarketFilter,
};
use serde_json::{json, Value};

const MARKET_CATEGORIES: [MarketCategory; 5] = [
    MarketCategory::Weapon,
    MarketCategory::Armor,
    MarketCategory::Shield,
    MarketCategory::Gear,
    MarketCategory::MagicItem,
];

/// Causes E1.9's converter fix addresses (`epic-breakdown.md` E1.9 (a)): a composite row resolved
/// to its base, a literal `COST` the converter copied without its eqmods' costs, and a price the
/// record states only through its eqmods.
const CONVERTER_CAUSES: [&str; 3] =
    ["composite_resolved_to_base", "cost_offset_by_eqmod", "price_in_eqmods"];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// One converted corpus record: where it lives and its `data` object.
struct CorpusRecord {
    path: String,
    data: Value,
}

/// Every `data/corpus/<book>/equipment/**/*.json` record, by `data.key`.
fn corpus_equipment(root: &Path) -> HashMap<String, Vec<CorpusRecord>> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "json") {
                out.push(path);
            }
        }
    }
    let mut files = Vec::new();
    for book in std::fs::read_dir(root.join("data/corpus")).unwrap() {
        let equipment = book.unwrap().path().join("equipment");
        if equipment.is_dir() {
            walk(&equipment, &mut files);
        }
    }
    files.sort();
    let mut by_key: HashMap<String, Vec<CorpusRecord>> = HashMap::new();
    for file in files {
        let doc: Value = serde_json::from_str(&std::fs::read_to_string(&file).unwrap())
            .unwrap_or_else(|e| panic!("{}: {e}", file.display()));
        let Some(key) = doc["data"]["key"].as_str() else { continue };
        let path = file.strip_prefix(root).unwrap().display().to_string();
        by_key.entry(key.to_string()).or_default().push(CorpusRecord { path, data: doc["data"].clone() });
    }
    by_key
}

fn tokens<'a>(data: &'a Value, name: &str) -> Vec<&'a str> {
    data["raw_tokens"]
        .as_array()
        .map(|ts| {
            ts.iter()
                .filter(|t| t["key"] == name)
                .filter_map(|t| t["value"].as_str())
                .collect()
        })
        .unwrap_or_default()
}

/// The eqmod keys an `EQMOD:` token names: `.`-separated, each without its `|` arguments.
fn eqmod_keys(data: &Value) -> Vec<String> {
    tokens(data, "EQMOD")
        .iter()
        .flat_map(|v| v.split('.'))
        .map(|k| k.split('|').next().unwrap_or("").trim().to_string())
        .filter(|k| !k.is_empty())
        .collect()
}

/// The kind of cost an eqmod states, from its `COST` token and its `ITEMCOST` bonuses, the
/// hardest first. E1.9 (a) lists the kinds its converter evaluates: numbers, `WT*N` / `BASECOST`
/// forms and the `+N` rate (`PLUSTOTAL`); the `%`-argument kinds read their inputs from the
/// `EQMOD:` token's own `[...]` arguments (`SPELLLEVEL[2]CASTERLEVEL[3]`, `|1721`, `CHARGES[50]`).
fn eqmod_cost_kind(key: &str, corpus: &HashMap<String, Vec<CorpusRecord>>) -> &'static str {
    let Some(record) = corpus.get(key).and_then(|rs| rs.first()) else { return "no_corpus_record" };
    let mut formulas: Vec<&str> = tokens(&record.data, "COST");
    if let Some(chains) = record.data["raw_bonus_chains"].as_array() {
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

/// Eqmod cost kinds E1.9 (a) names as evaluated by its converter fix.
const E19_LISTED_KINDS: [&str; 3] = ["numeric", "weight_or_basecost", "enhancement_plus"];
/// Kinds whose inputs all sit in the record (the formula plus the `EQMOD:` token's arguments).
const IN_RECORD_KINDS: [&str; 6] =
    ["numeric", "weight_or_basecost", "enhancement_plus", "spell_effect_args", "charges_arg", "choice_arg"];

/// Why a Market row has no positive price, read off its converted record.
fn cause(
    entry: &EquipmentCatalogEntryDto,
    catalog_name: &str,
    record: Option<&CorpusRecord>,
) -> &'static str {
    // A PCGen removal directive (`Dust Knuckles.FORGET`) served as a row: not an item to price.
    if entry.key.ends_with(".FORGET") {
        return "retirement_directive_row";
    }
    let Some(record) = record else { return "no_corpus_record" };
    let data = &record.data;
    let corpus_cost = data["cost_gp"].as_f64();
    if corpus_cost.is_some_and(|c| c > 0.0) {
        return "table_disagrees_with_corpus";
    }
    let has_tokens = data["raw_tokens"].as_array().is_some_and(|t| !t.is_empty());
    if !has_tokens {
        // The served name is another item's (`Breastplate (Base)`, `Donkey` for Mule): the
        // converter resolved the row to its base and dropped its own tokens.
        return if catalog_name != entry.key && catalog_name != format!("{} (Base)", entry.key) {
            "composite_resolved_to_base"
        } else {
            "no_raw_tokens"
        };
    }
    let cost = tokens(data, "COST");
    let has_eqmods = !eqmod_keys(data).is_empty();
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

fn bump(map: &mut BTreeMap<String, BTreeMap<String, usize>>, outer: &str, inner: &str) {
    *map.entry(outer.to_string()).or_default().entry(inner.to_string()).or_default() += 1;
}

#[test]
fn market_price_census() {
    let root = repo_root();
    let corpus = corpus_equipment(&root);
    // `list_market` keeps the first catalog row of a key; so does this lookup.
    let catalog = build_equipment_catalog();
    let mut catalog_names: HashMap<&str, &str> = HashMap::new();
    for e in &catalog.entries {
        catalog_names.entry(e.key.as_str()).or_insert(e.name.as_str());
    }

    let (mut none, mut zero, mut negative) = (Vec::new(), Vec::new(), Vec::new());
    let mut by_cause: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut by_book: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    let mut by_category: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    let mut market_rows = BTreeMap::new();
    // Rows a lookup must still price, by category, under three assumptions about E1.9.
    let mut left_if_in_record: BTreeMap<String, usize> = BTreeMap::new();
    let mut left_if_e19_listed: BTreeMap<String, usize> = BTreeMap::new();
    let mut every_unpriced: BTreeMap<String, usize> = BTreeMap::new();
    let mut by_eqmod_kind: BTreeMap<String, usize> = BTreeMap::new();
    let mut ambiguous_corpus_keys = 0usize;

    for category in MARKET_CATEGORIES {
        let market = list_market(MarketFilter { category, name_contains: None, book: None });
        assert!(market.types_error.is_none(), "Market has no type data: {:?}", market.types_error);
        let cat = serde_json::to_value(category).unwrap().as_str().unwrap().to_string();
        market_rows.insert(cat.clone(), market.entries.len());
        for row in &market.entries {
            let e = &row.entry;
            let kind = match e.cost_gp {
                None => "none",
                Some(0.0) => "zero",
                Some(c) if c < 0.0 => "negative",
                Some(_) => continue,
            };
            let catalog_name = catalog_names.get(e.key.as_str()).copied().unwrap_or(&e.name);
            let records = corpus.get(&e.key);
            if records.is_some_and(|r| r.len() > 1) {
                ambiguous_corpus_keys += 1;
            }
            let record = records.and_then(|r| r.first());
            let why = cause(e, catalog_name, record);
            let eqmods = record.map(|r| eqmod_keys(&r.data)).unwrap_or_default();
            let eqmod_kinds: BTreeMap<&String, &str> =
                eqmods.iter().map(|k| (k, eqmod_cost_kind(k, &corpus))).collect();
            let out = json!({
                "key": e.key,
                "name": e.name,
                "catalog_name": catalog_name,
                "book": e.book,
                "catalog_category": e.category,
                "market_category": cat,
                "cost_gp": e.cost_gp,
                "cause": why,
                "corpus": records.map(|rs| rs.iter().map(|r| r.path.clone()).collect::<Vec<_>>()),
                "cost_token": record.map(|r| tokens(&r.data, "COST")),
                "eqmods": eqmods,
                "eqmod_cost_kinds": eqmod_kinds,
            });
            match kind {
                "none" => none.push(out),
                "zero" => zero.push(out),
                _ => negative.push(out),
            }
            by_cause.entry(why.to_string()).or_default().push(e.key.clone());
            bump(&mut by_book, &e.book, kind);
            bump(&mut by_category, &cat, kind);
            for kind in eqmod_kinds.values().collect::<std::collections::BTreeSet<_>>() {
                *by_eqmod_kind.entry(kind.to_string()).or_default() += 1;
            }
            // A composite's dropped eqmods are unknown until E1.9 restores its tokens: counted fixed.
            let fixed_by = |kinds: &[&str]| {
                CONVERTER_CAUSES.contains(&why)
                    && (why == "composite_resolved_to_base" || eqmod_kinds.values().all(|k| kinds.contains(k)))
            };
            *every_unpriced.entry(cat.clone()).or_default() += 1;
            if !fixed_by(&IN_RECORD_KINDS) {
                *left_if_in_record.entry(cat.clone()).or_default() += 1;
            }
            if !fixed_by(&E19_LISTED_KINDS) {
                *left_if_e19_listed.entry(cat.clone()).or_default() += 1;
            }
        }
    }

    // The two rows the operator named (`decisions.md §26`) carry the causes it proved.
    let cause_of = |key: &str| {
        by_cause.iter().find(|(_, keys)| keys.iter().any(|k| k == key)).map(|(c, _)| c.as_str())
    };
    assert_eq!(cause_of("Adamantine Breastplate"), Some("composite_resolved_to_base"));
    assert_eq!(cause_of("Astral Armor"), Some("cost_offset_by_eqmod"));

    let counts = json!({
        "none": none.len(), "zero": zero.len(), "negative": negative.len(),
        "by_cause": by_cause.iter().map(|(k, v)| (k.clone(), v.len())).collect::<BTreeMap<_, _>>(),
    });
    let report = json!({
        "command": "cd apps/desktop/src-tauri && cargo test --locked -j 8 --test market_price_census market_price -- --test-threads=8 --nocapture",
        "population": "every row list_market serves across its 5 categories (no name or book filter)",
        "market_rows": market_rows,
        "counts": counts,
        "converter_causes": CONVERTER_CAUSES,
        "by_eqmod_cost_kind": by_eqmod_kind,
        "d20pfsrd_bound": {
            "if_converter_evaluates_every_in_record_kind": left_if_in_record,
            "if_converter_evaluates_every_in_record_kind_total": left_if_in_record.values().sum::<usize>(),
            "if_converter_evaluates_e19_listed_kinds_only": left_if_e19_listed,
            "if_converter_evaluates_e19_listed_kinds_only_total": left_if_e19_listed.values().sum::<usize>(),
            "every_unpriced": every_unpriced,
            "every_unpriced_total": every_unpriced.values().sum::<usize>(),
            "predicate": "a row is left for the lookup unless its cause is a converter cause and (it is a composite resolved to its base, whose eqmods are unknown until E1.9 restores them, or every eqmod it names states a cost of an evaluated kind); in_record kinds = numeric, WT/BASECOST, +N, and the %SPELL/%CHARGES/%CHOICE formulas whose inputs are the EQMOD token arguments; e19_listed = numeric, WT/BASECOST, +N",
        },
        "ambiguous_corpus_keys": ambiguous_corpus_keys,
        "by_book": by_book,
        "by_category": by_category,
        "by_cause": by_cause,
        "none": none,
        "zero": zero,
        "negative": negative,
    });
    let out = root.join("docs/release/SD-41-gear-and-equipment-sets/artifacts/epic_1/E1.8/market-prices.json");
    std::fs::create_dir_all(out.parent().unwrap()).unwrap();
    std::fs::write(&out, serde_json::to_string_pretty(&report).unwrap() + "\n").unwrap();
    println!("market rows {market_rows:?}");
    println!("{}", serde_json::to_string_pretty(&counts).unwrap());
    println!("d20pfsrd bound {}", serde_json::to_string(&report["d20pfsrd_bound"]).unwrap());
}
