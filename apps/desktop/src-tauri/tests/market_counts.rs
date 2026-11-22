//! Market count reconciliation (SD-43 E0.2, Q-11.1): every row of the built catalog is either
//! shown on one Market list or dropped for a named reason, and shown + dropped = source per list.
//! Writes `docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_0/market-counts.json`.
//!
//! The crate has no lib target, so the catalog modules are compiled in by path (as
//! `market_price_census.rs` does); the shown side is the app's own `list_market`. Their own
//! `#[cfg(test)]` suites compile here too; `cargo test --test market_counts market_counts` runs
//! only this one.
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

use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

use equipment_catalog::{
    built_catalog, list_market, market_category_of, EquipmentCatalogEntryDto, MarketCategory, MarketFilter,
};
use serde_json::json;

const LISTS: [(MarketCategory, &str); 5] = [
    (MarketCategory::Weapon, "Weapons"),
    (MarketCategory::Armor, "Armor"),
    (MarketCategory::Shield, "Shields"),
    (MarketCategory::Gear, "Gear"),
    (MarketCategory::MagicItem, "Magic"),
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn list_name(category: Option<MarketCategory>) -> &'static str {
    LISTS.iter().find(|(c, _)| Some(*c) == category).map_or("(no list)", |(_, name)| name)
}

/// Why a catalog row is not on a Market list, or `None` for no named reason. The order is
/// `market_from_catalog`'s: the first row of a key decides it, then the hides, then the list.
fn reason(entry: &EquipmentCatalogEntryDto, first_of_key: bool) -> Option<&'static str> {
    if !first_of_key {
        return Some("duplicate_key");
    }
    if entry.key.ends_with(".FORGET") {
        return Some("forget_directive");
    }
    if entry.cost_gp.is_none() && entry.name == format!("{} (Base)", entry.key) {
        return Some("template");
    }
    if entry.name.starts_with("Codex-Named Unit") {
        return Some("codex_named_unit");
    }
    if entry.category == "Equipmods" {
        return Some("equipmods");
    }
    None
}

#[test]
fn market_counts() {
    let catalog = built_catalog();
    assert!(catalog.types_error.is_none(), "no type data: {:?}", catalog.types_error);

    let mut shown: BTreeMap<&str, usize> = BTreeMap::new();
    let mut shown_keys: HashSet<String> = HashSet::new();
    let mut shown_rows = Vec::new();
    for (category, name) in LISTS {
        let rows = list_market(MarketFilter { category, name_contains: None, book: None }).entries;
        shown.insert(name, rows.len());
        for row in rows {
            shown_rows.push(json!({"list": name, "key": row.entry.key, "category": row.entry.category, "types": row.entry.types}));
            shown_keys.insert(row.entry.key);
        }
    }

    // Per list (the list a row's tags would put it in): source, and dropped by reason.
    let mut source: BTreeMap<&str, usize> = BTreeMap::new();
    let mut dropped: BTreeMap<&str, BTreeMap<&str, usize>> = BTreeMap::new();
    let mut dropped_rows = Vec::new();
    let mut shown_untagged: BTreeMap<&str, usize> = BTreeMap::new();
    let mut seen = HashSet::new();
    let mut unreasoned = Vec::new();
    let mut shown_with_reason = Vec::new();
    for entry in &catalog.entries {
        let first = seen.insert(entry.key.as_str());
        let list = list_name(market_category_of(entry));
        *source.entry(list).or_default() += 1;
        let why = reason(entry, first);
        if first && shown_keys.contains(&entry.key) {
            if let Some(why) = why {
                shown_with_reason.push(format!("{} [{}] {why}", entry.key, entry.book));
            }
            if entry.types.is_empty() {
                *shown_untagged.entry(list).or_default() += 1;
            }
            continue;
        }
        let Some(why) = why else {
            unreasoned.push(format!("{} [{}]", entry.key, entry.book));
            continue;
        };
        *dropped.entry(list).or_default().entry(why).or_default() += 1;
        println!("dropped\t{list}\t{why}\t{}\t{}", entry.book, entry.key);
        dropped_rows.push(json!({"list": list, "reason": why, "book": entry.book, "key": entry.key, "name": entry.name}));
    }
    assert!(unreasoned.is_empty(), "{} rows dropped without a reason, e.g. {:?}", unreasoned.len(), &unreasoned[..unreasoned.len().min(5)]);
    assert!(shown_with_reason.is_empty(), "shown rows the reasons would drop: {shown_with_reason:?}");

    let mut total = 0;
    let mut lists = Vec::new();
    for list in source.keys() {
        let (s, d) = (shown.get(list).copied().unwrap_or(0), dropped.get(list).map_or(0, |m| m.values().sum()));
        println!("{list}: shown {s} + dropped {d} = source {} {:?}; shown with no type tags {}", source[list], dropped.get(list), shown_untagged.get(list).copied().unwrap_or(0));
        assert_eq!(s + d, source[list], "{list}: shown + dropped != source");
        total += source[list];
        lists.push(json!({"list": list, "shown": s, "dropped": d, "source": source[list],
            "dropped_by_reason": dropped.get(list), "shown_with_no_type_tags": shown_untagged.get(list).copied().unwrap_or(0)}));
    }
    println!("sum of sources {total} = built_catalog().entries.len() {}", catalog.entries.len());
    assert_eq!(total, catalog.entries.len());

    // The operator's screen figures (Q-11.1), and the exact values the screen did not give.
    assert_eq!(shown["Magic"], 3680);
    assert_eq!(shown["Gear"], 1624);
    assert_eq!(shown["Weapons"], 480);
    assert_eq!(shown["Armor"], 63);
    assert_eq!(shown["Shields"], 31);
    println!("Magic {} Gear {} Weapons {} Armor {} Shields {}", shown["Magic"], shown["Gear"], shown["Weapons"], shown["Armor"], shown["Shields"]);

    let out = repo_root().join("docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_0/market-counts.json");
    std::fs::create_dir_all(out.parent().unwrap()).unwrap();
    let doc = json!({
        "source": "built_catalog() via tests/market_counts.rs",
        "catalog_rows": catalog.entries.len(),
        "lists": lists,
        "not_drop_reasons": {
            "no_type_tags": "a row with no type tags is filed under Gear and shown (market_category_of), not dropped",
            "book_not_ingested": "a book not ingested contributes no catalog row, so it has no row to drop here",
        },
        "dropped_rows": dropped_rows,
    });
    std::fs::write(&out, serde_json::to_string_pretty(&doc).unwrap() + "\n").unwrap();

    // Every shown row with its type tags, one line each: the Market filter model's test data (E1.10).
    let lines: Vec<String> = shown_rows.iter().map(|row| serde_json::to_string(row).unwrap()).collect();
    std::fs::write(out.with_file_name("market-rows.jsonl"), lines.join("\n") + "\n").unwrap();
}
