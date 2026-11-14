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

/// The census's record side, written by `settle_equipment_prices` (E1.15): per corpus equipment
/// key, its record paths, the converter cause its first record shows, its `COST` token and its
/// eqmods' cost kinds. The classifier reads the converted record's ingest tokens, so it lives
/// tool side (`decisions.md §17`); this test reads only its verdicts.
fn corpus_price_causes(root: &Path) -> HashMap<String, Value> {
    let path = root.join("docs/release/SD-41-gear-and-equipment-sets/artifacts/epic_1/E1.8/corpus-price-causes.json");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!("{}: {e}; run `cargo run --locked -p codex-ingest --bin settle_equipment_prices` from the repo root", path.display())
    });
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// Eqmod cost kinds E1.9 (a) names as evaluated by its converter fix.
const E19_LISTED_KINDS: [&str; 3] = ["numeric", "weight_or_basecost", "enhancement_plus"];
/// Kinds whose inputs all sit in the record (the formula plus the `EQMOD:` token's arguments).
const IN_RECORD_KINDS: [&str; 6] =
    ["numeric", "weight_or_basecost", "enhancement_plus", "spell_effect_args", "charges_arg", "choice_arg"];

/// Why a Market row has no positive price: the record's cause, with a record that carries no
/// tokens told apart by the name the Market serves for it.
fn cause<'a>(entry: &EquipmentCatalogEntryDto, catalog_name: &str, record: Option<&'a Value>) -> &'a str {
    // A PCGen removal directive (`Dust Knuckles.FORGET`) served as a row: not an item to price.
    if entry.key.ends_with(".FORGET") {
        return "retirement_directive_row";
    }
    let Some(record) = record else { return "no_corpus_record" };
    let why = record["cause"].as_str().expect("every census row names a cause");
    // The served name is another item's (`Breastplate (Base)`, `Donkey` for Mule): the converter
    // resolved the row to its base and dropped its own tokens.
    if why == "no_raw_tokens" && catalog_name != entry.key && catalog_name != format!("{} (Base)", entry.key) {
        return "composite_resolved_to_base";
    }
    why
}

fn bump(map: &mut BTreeMap<String, BTreeMap<String, usize>>, outer: &str, inner: &str) {
    *map.entry(outer.to_string()).or_default().entry(inner.to_string()).or_default() += 1;
}

#[test]
fn market_price_census() {
    let root = repo_root();
    let corpus = corpus_price_causes(&root);
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
            let record = corpus.get(&e.key);
            if record.and_then(|r| r["corpus"].as_array()).is_some_and(|paths| paths.len() > 1) {
                ambiguous_corpus_keys += 1;
            }
            let why = cause(e, catalog_name, record);
            let eqmods = record.map(|r| r["eqmods"].clone()).unwrap_or_else(|| json!([]));
            let eqmod_kinds: BTreeMap<String, String> = record
                .map(|r| serde_json::from_value(r["eqmod_cost_kinds"].clone()).expect("eqmod_cost_kinds is a map"))
                .unwrap_or_default();
            let out = json!({
                "key": e.key,
                "name": e.name,
                "catalog_name": catalog_name,
                "book": e.book,
                "catalog_category": e.category,
                "market_category": cat,
                "cost_gp": e.cost_gp,
                "cause": why,
                "corpus": record.map(|r| r["corpus"].clone()),
                "cost_token": record.map(|r| r["cost_token"].clone()),
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
                    && (why == "composite_resolved_to_base" || eqmod_kinds.values().all(|k| kinds.contains(&k.as_str())))
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

    // The two rows the operator named (`decisions.md §26`): E1.8 proved their causes on its base
    // (`composite_resolved_to_base`, `cost_offset_by_eqmod`); since E1.9's converter fix the
    // breastplate is priced and Astral Armor settles at 0 (an eqmod-offset literal, never negative).
    let cause_of = |key: &str| {
        by_cause.iter().find(|(_, keys)| keys.iter().any(|k| k == key)).map(|(c, _)| c.as_str())
    };
    assert_eq!(cause_of("Adamantine Breastplate"), None);
    assert_eq!(cause_of("Astral Armor"), Some("cost_offset_by_eqmod"));
    assert!(negative.is_empty(), "no Market row is negative after E1.9: {negative:?}");

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
            "predicate": "a row is left for the lookup unless its cause is a converter cause and (it is a composite resolved to its base, whose eqmods are unknown until E1.9 restores them, or every eqmod it names states a cost of an evaluated kind); in_record kinds = numeric, WT/BASECOST, +N, and the spell-effect, charges and choice-argument formulas whose inputs are the EQMOD token arguments; e19_listed = numeric, WT/BASECOST, +N",
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

/// The keys `data/price_overrides/price-unknown.json` lists, each with its reason. A missing file
/// lists nothing, so every unpriced Market row fails the gate below (fails closed).
fn price_unknown_keys(root: &Path) -> BTreeMap<String, String> {
    let path = root.join("data/price_overrides/price-unknown.json");
    let Ok(text) = std::fs::read_to_string(&path) else { return BTreeMap::new() };
    let rows: Vec<Value> = serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    rows.iter()
        .map(|row| {
            let key = row["key"].as_str().unwrap_or_else(|| panic!("price-unknown row without a key: {row}"));
            let reason = row["reason"].as_str().unwrap_or("");
            assert!(!reason.trim().is_empty(), "price-unknown row {key:?} names no reason");
            (key.to_string(), reason.to_string())
        })
        .collect()
}

/// The price gate (`decisions.md §26` (1), `§30.1`): every Market row has a positive price, or its
/// key is listed in `price-unknown.json` with a reason. E1.11 tightens what a listed row must carry.
#[test]
fn market_prices_are_positive_or_listed() {
    let root = repo_root();
    // The converter prices composites and eqmod-priced rows as the book does (CRB p.464, 465, 477, 496).
    let market: Vec<_> = MARKET_CATEGORIES
        .iter()
        .flat_map(|&category| list_market(MarketFilter { category, name_contains: None, book: None }).entries)
        .collect();
    let price = |key: &str| market.iter().find(|row| row.entry.key == key).and_then(|row| row.entry.cost_gp);
    assert_eq!(price("Adamantine Breastplate"), Some(10_200.0));
    assert_eq!(price("Banded Mail of Luck"), Some(18_900.0));
    assert_eq!(price("Potion of Cure Light Wounds"), Some(50.0));
    assert_eq!(price("Wand of Magic Missile"), Some(750.0));
    assert_eq!(price("Astral Armor"), Some(0.0), "an eqmod-offset literal settles at 0, never negative");
    let listed = price_unknown_keys(&root);
    let mut unlisted = Vec::new();
    for category in MARKET_CATEGORIES {
        let market = list_market(MarketFilter { category, name_contains: None, book: None });
        for row in &market.entries {
            let e = &row.entry;
            if !e.cost_gp.is_some_and(|c| c > 0.0) && !listed.contains_key(&e.key) {
                unlisted.push(format!("{} [{}] {:?}", e.key, e.book, e.cost_gp));
            }
        }
    }
    assert!(
        unlisted.is_empty(),
        "{} Market rows have a missing, zero or negative price and are not in price-unknown.json; first 20: {:#?}",
        unlisted.len(),
        unlisted.iter().take(20).collect::<Vec<_>>()
    );
}

/// E1.11's tightening of the gate (`decisions.md §30.1`): a Market key listed in
/// `price-unknown.json` must name every source tried — the converter and at least one d20pfsrd
/// page — and every price cited in `d20pfsrd-prices.json` must be the price the Market serves for
/// that key (the cited list reaches the Market through `settle_equipment_prices`).
#[test]
fn market_price_unknown_rows_name_every_source_tried() {
    let root = repo_root();
    let read = |name: &str| -> Vec<Value> {
        let path = root.join("data/price_overrides").join(name);
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
    };
    let unknown = read("price-unknown.json");
    let cited = read("d20pfsrd-prices.json");
    let market: HashMap<String, Option<f64>> = MARKET_CATEGORIES
        .iter()
        .flat_map(|&category| list_market(MarketFilter { category, name_contains: None, book: None }).entries)
        .map(|row| (row.entry.key.clone(), row.entry.cost_gp))
        .collect();

    let mut untried = Vec::new();
    for row in &unknown {
        let key = row["key"].as_str().unwrap_or_default();
        if !market.contains_key(key) {
            continue;
        }
        let tried: Vec<&str> =
            row["tried"].as_array().map(|t| t.iter().filter_map(Value::as_str).collect()).unwrap_or_default();
        let converter = tried.first() == Some(&"converter");
        let d20pfsrd = tried.iter().any(|t| t.starts_with("d20pfsrd:https://www.d20pfsrd.com/"));
        if !(converter && d20pfsrd) {
            untried.push(format!("{key}: tried {tried:?}"));
        }
    }
    assert!(
        untried.is_empty(),
        "{} price-unknown Market rows do not name every source tried (converter, then a d20pfsrd URL); first 20: {:#?}",
        untried.len(),
        untried.iter().take(20).collect::<Vec<_>>()
    );

    let unknown_keys: Vec<&str> = unknown.iter().filter_map(|r| r["key"].as_str()).collect();
    let mut unreached = Vec::new();
    for row in &cited {
        let key = row["key"].as_str().expect("a cited row has a key");
        let price = row["price_gp"].as_f64().expect("a cited row has a price_gp");
        assert!(price > 0.0, "{key}: a cited price is positive");
        assert!(row["url"].as_str().is_some_and(|u| u.starts_with("https://www.d20pfsrd.com/")), "{key}: cites a d20pfsrd page");
        assert!(row["fetched_at"].as_str().is_some_and(|t| !t.is_empty()), "{key}: says when the page was fetched");
        assert!(!unknown_keys.contains(&key), "{key}: both cited and listed price-unknown");
        if market.get(key) != Some(&Some(price)) {
            unreached.push(format!("{key}: cited {price}, Market {:?}", market.get(key)));
        }
    }
    assert!(unreached.is_empty(), "cited prices the Market does not serve: {unreached:#?}");
}
