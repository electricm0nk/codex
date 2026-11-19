//! The app path of the local proof run (SD-42 `decisions.md §10`, E4.5; OPERATOR Q3, Q9): the
//! operator's own file goes through the converter the sidecar runs and the app's `import_pcg_json`
//! into a fresh root, and the held list the gear screen reads is checked. Counts only are printed.
//!
//! The file is `$CODEX_SD42_PCG`, else the one `.pcg` in `$HOME/sd42-input/`; absent (CI), the test
//! prints `SKIPPED: …` and passes.
//!
//! **Does not cover:** any other operator file; the screen's pixels and the print (E4.MC, E5.G);
//! PCGen's totals (`pcg_import_parity`, E4.13).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use super::*;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn operator_pcg() -> PathBuf {
    let input = PathBuf::from(std::env::var("HOME").expect("HOME")).join("sd42-input");
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

#[test]
fn import_operator_file_held_list() {
    let pcg = operator_pcg();
    if !pcg.is_file() {
        println!("SKIPPED: operator .pcg absent at {} (decisions.md §10)", pcg.display());
        return;
    }
    let unique = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let scratch = std::env::temp_dir().join(format!("codex-import-operator-{}-{unique}", std::process::id()));
    std::fs::create_dir_all(&scratch).unwrap();
    let out = scratch.join("out.json");
    let wire = codex_ingest::pcg_import::run(&pcg, &out, &repo_root()).expect("the converter reads the operator's file");
    let root = scratch.join("characters").join("operator");
    let response = import_pcg_json(&std::fs::read_to_string(&out).unwrap(), &root, "operator", "2026-11-19T00:00:00Z", "0.0.0-test").expect("the import runs");
    assert!(matches!(response.outcome, CreateCharacterResponse::Saved { .. }), "the import saves a character");

    let rows = list_held_equipment_at_root(&root).unwrap();
    // Every stack the wire holds (a Carried one needs no location entry).
    let placed: BTreeSet<String> = wire.equipment_sets.copies.keys().cloned().collect();
    let listed: Vec<String> = rows.iter().flat_map(|row| row.copies.iter().map(|copy| copy.map_key())).collect();
    let listed_set: BTreeSet<String> = listed.iter().cloned().collect();
    let wire_pieces: u64 = wire.equipment_sets.copies.values().filter_map(|facts| facts.quantity).map(u64::from).sum();
    let row_pieces: u64 = rows.iter().map(|row| u64::from(row.quantity)).sum();
    let names: BTreeSet<&str> = rows.iter().map(|row| row.name.as_str()).collect();
    // A stack's own name, or a Create item's (Codex has no record of its base).
    let custom_names: Vec<&String> = wire.equipment_sets.copies.values().filter_map(|facts| facts.name.as_ref()).chain(wire.custom.equipment.iter().map(|record| &record.name)).collect();
    let custom_shown = custom_names.iter().filter(|name| names.contains(name.as_str())).count();
    let rations = rows.iter().filter(|row| row.quantity == 84).count();
    println!(
        "operator app: held rows {}, stacks on the wire {}, stacks listed {}, pieces wire {wire_pieces} rows {row_pieces}, custom names {} shown {custom_shown}, rows of 84 {rations}",
        rows.len(),
        placed.len(),
        listed_set.len(),
        custom_names.len()
    );
    assert_eq!(listed.len(), listed_set.len(), "no stack is listed twice");
    let unlisted = placed.difference(&listed_set).count() + listed_set.difference(&placed).count();
    assert_eq!(unlisted, 0, "every stack on the wire is on the held list, and nothing else");
    assert_eq!(row_pieces, wire_pieces, "the held list holds every piece the file places");
    assert_eq!(custom_shown, custom_names.len(), "every custom name reads on its row");
    assert_eq!(rations, 1, "the 84 trail rations are one row");
    let _ = std::fs::remove_dir_all(&scratch);
}
