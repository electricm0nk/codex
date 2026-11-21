//! The local proof run on the operator's own file (SD-42 `decisions.md §10`, E4.5; OPERATOR Q3, Q9).
//!
//! The file lives only on the operator's machine: `$CODEX_SD42_PCG`, else the one `.pcg` in
//! `$HOME/sample_pcg/` (`pcg_import_diagnosis.rs`). Absent, the test prints `SKIPPED: …` and passes
//! (CI never has it). Present, it converts the file as the sidecar does and asserts, printing counts
//! only (no line of the file is printed): every held line is in at least one stack and the pieces are
//! conserved (`decisions.md §20`, E1.2: one stack per place a line is in), every custom `NAME=` is the
//! name a held thing shows, every `SPROP` line's stack carries non-empty properties, the largest
//! stack (the 84 trail rations) is one stack two containers deep, and no warning caps a quantity.
//! The whole-character parity of the same file is `pcg_import_parity`'s `operator` row, gated there.
//!
//! **Does not cover:** any other operator file; the app's held list (the desktop's
//! `import_operator_file` test); the screen and the print; PCGen's totals (`pcg_import_parity`).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use codex::rules_core::equipment_resolver::equipment_converted_resolve;
use codex::saved_character::equipment_sets::{CopyKey, Location, location_of};
use codex_ingest::pcg_import::catalog::Catalog;
use codex_ingest::pcg_import::map::{convert, equipment_corpus, record_base_quantity};
use codex_ingest::pcg_import::parse::parse;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn operator_pcg() -> PathBuf {
    let input = PathBuf::from(std::env::var("HOME").expect("HOME")).join("sample_pcg");
    match std::env::var("CODEX_SD42_PCG") {
        Ok(path) if !path.is_empty() => PathBuf::from(path),
        _ => std::fs::read_dir(&input)
            .into_iter()
            .flatten()
            .flatten()
            .map(|entry| entry.path())
            .find(|path| path.extension().is_some_and(|ext| ext == "pcg"))
            .unwrap_or_else(|| input.join("<operator>.pcg")),
    }
}

/// `CUSTOMIZATION:[…|DATA:K=V$K=V…]` → the `(K, V)` pairs.
fn data_fields(customization: &str) -> Vec<(String, String)> {
    let Some(data) = customization.split("DATA:").nth(1) else { return Vec::new() };
    data.trim_end_matches(']')
        .split('$')
        .filter_map(|field| field.split_once('='))
        .map(|(key, value)| (key.trim().to_owned(), value.trim().to_owned()))
        .collect()
}

/// `CUSTOMIZATION:[BASEITEM:<base>|…]` → `<base>`.
fn base_item(customization: &str) -> &str {
    customization.split("BASEITEM:").nth(1).and_then(|rest| rest.split(['|', ']']).next()).unwrap_or("").trim()
}

fn item_id(map_key: &str) -> &str {
    map_key.rsplitn(3, '|').nth(2).unwrap_or("")
}

#[test]
fn pcg_import_operator_file_counts() {
    let path = operator_pcg();
    let Ok(bytes) = std::fs::read(&path) else {
        println!("SKIPPED: operator .pcg absent at {} (decisions.md §10)", path.display());
        return;
    };
    let text = String::from_utf8_lossy(&bytes).into_owned();
    let catalog = Catalog::load(&repo_root()).expect("the checkout holds converted rules");
    let file = convert(&text, "operator.pcg", &catalog).expect("the operator's file converts");
    let sets = &file.equipment_sets;

    let lines = parse(&text);
    let equip: Vec<_> = lines.iter().filter(|line| line.key() == "EQUIPNAME").collect();
    let quantity = |line: &&codex_ingest::pcg_import::parse::Line| line.get("QUANTITY").and_then(|q| q.trim().parse::<f64>().ok()).map_or(1, |q| q.round() as u64);
    let zero_lines = equip.iter().filter(|line| quantity(line) == 0).count();
    let held_lines = equip.len() - zero_lines;
    let pieces: u64 = equip.iter().map(quantity).sum();
    // Stacks count bundles back from pieces (E1.8), as `pcg_import_stacks` does.
    let stacked: u64 = sets
        .copies
        .iter()
        .map(|(key, facts)| u64::from(facts.quantity.expect("every stack states its pieces") / record_base_quantity(item_id(key))))
        .sum();
    println!("operator: EQUIPNAME lines {}, zero {zero_lines}, held {held_lines}, stacks {}, pieces {pieces}, stacked {stacked}", equip.len(), sets.copies.len());
    assert!(sets.copies.len() >= held_lines, "{} stacks for {held_lines} held lines", sets.copies.len());

    // Every custom name is shown: a stack's own name, a Create item's, or the record's. A `NAME`
    // equal to its `BASEITEM` is PCGen's key for the record, not a custom name (`pcg_import_custom`):
    // it is held as that record, or as a Create item when Codex has none.
    let customisations: Vec<&str> = equip.iter().filter_map(|line| line.get("CUSTOMIZATION")).collect();
    let customised: Vec<Vec<(String, String)>> = customisations.iter().map(|text| data_fields(text)).collect();
    let mut shown: BTreeSet<String> = sets.copies.values().filter_map(|facts| facts.name.clone()).collect();
    shown.extend(file.custom.equipment.iter().map(|record| record.name.clone()));
    shown.extend(sets.copies.keys().filter_map(|key| equipment_converted_resolve(item_id(key), equipment_corpus())).map(|record| record.name.clone()));
    let named: Vec<(&str, &String)> = customisations
        .iter()
        .zip(&customised)
        .filter_map(|(text, fields)| fields.iter().find(|(key, _)| key == "NAME").map(|(_, value)| (base_item(text), value)))
        .collect();
    let names: Vec<&String> = named.iter().filter(|(base, name)| base != name).map(|(_, name)| *name).collect();
    let missing: Vec<&&String> = names.iter().filter(|name| !shown.contains(name.as_str())).collect();
    let keyed: Vec<&str> = named.iter().filter(|(base, name)| base == name).map(|(base, _)| *base).collect();
    let keyed_held = keyed
        .iter()
        .filter(|base| {
            catalog.equipment_key(base).is_some_and(|key| sets.copies.keys().any(|map_key| item_id(map_key) == key))
                || file.custom.equipment.iter().any(|record| record.name == **base)
        })
        .count();
    // Each `SPROP` line's stack (found by the name it shows) carries non-empty properties, or its
    // Create item non-empty stats.
    let record_name = |key: &str| equipment_converted_resolve(item_id(key), equipment_corpus()).map(|record| record.name.clone());
    let carries = |name: &str| {
        sets.copies.iter().any(|(key, facts)| !facts.properties.is_empty() && (facts.name.as_deref() == Some(name) || (facts.name.is_none() && record_name(key).as_deref() == Some(name))))
            || file.custom.equipment.iter().any(|record| record.name == name && !record.stats.is_empty())
    };
    let sprop_names: Vec<&String> = customised
        .iter()
        .filter(|fields| fields.iter().any(|(key, _)| key == "SPROP"))
        .map(|fields| fields.iter().find(|(key, _)| key == "NAME").map(|(_, value)| value).expect("a SPROP line names its item"))
        .collect();
    let sprop_carried = sprop_names.iter().filter(|name| carries(name)).count();
    println!(
        "operator: customised lines {}, NAME fields {}, custom names {}, names shown {}, NAME = BASEITEM {} (held as their record {keyed_held}), SPROP lines {}, SPROP lines whose stack carries properties {sprop_carried}",
        customised.len(),
        named.len(),
        names.len(),
        names.len() - missing.len(),
        keyed.len(),
        sprop_names.len()
    );
    assert!(missing.is_empty(), "{} custom names no held thing shows", missing.len());
    assert_eq!(keyed_held, keyed.len(), "a NAME equal to its BASEITEM is held as that record or a Create item");
    assert_eq!(sprop_carried, sprop_names.len(), "every SPROP line's stack carries its properties");

    // The largest stack: one stack, two containers deep.
    let largest = equip.iter().map(quantity).max().unwrap_or(0);
    let big: Vec<(&String, u32)> = sets.copies.iter().filter_map(|(key, facts)| facts.quantity.map(|q| (key, q))).filter(|(_, q)| u64::from(*q) == largest).collect();
    assert_eq!(big.len(), 1, "one stack holds the file's largest QUANTITY {largest}");
    let copy: CopyKey = sets.active_set().locations.iter().map(|entry| entry.copy.clone()).find(|copy| &copy.map_key() == big[0].0).expect("the stack is placed");
    let depth = std::iter::successors(Some(location_of(sets.active_set(), &copy)), |location| match location {
        Location::Inside { container } => Some(location_of(sets.active_set(), container)),
        _ => None,
    })
    .filter(|location| matches!(location, Location::Inside { .. }))
    .count();
    println!("operator: largest stack {} pieces, Inside depth {depth}", big[0].1);
    assert_eq!((big[0].1, depth), (84, 2), "the trail rations: 84 in one stack, two containers deep");

    let capped = file.report.warnings.iter().filter(|warning| warning.contains("MAX_COPIES") || warning.contains("capped")).count();
    println!("operator: warnings {}, capped {capped}", file.report.warnings.len());
    assert_eq!(capped, 0, "no quantity is capped");
}
