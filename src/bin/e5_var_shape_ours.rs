//! SD-33 remediation wave 3 (`var-bonus-shape` lane) -- repo-local "ours"
//! batch probe for the equipment `other_bonus_shape` `VAR` sub-population
//! (108 of the 391 units `AT-33-E5-002`'s own remainder receipt left
//! unattempted).
//!
//! Same pattern as `src/bin/e5_equipment_remainder_skill_ours.rs`: one
//! process, one corpus load per book set, real live calls into
//! `codex::rules_core::equipment_effects::general::compute_var_effect`
//! (new this cycle -- see that module's own doc comment) for every unit in
//! the input manifest. No stubs, no fixture-only values, no hand-derived
//! duplicate of the corpus's own literal tokens.
//!
//! Usage:
//!   e5_var_shape_ours <repo_root> <manifest.json> <output.json>
//!
//! `manifest.json`: `[{"unit_id","book","key"}, ...]`
//!
//! Output: `{"unit_id": [{"name": <var-name>, "bonus": <i16>}, ...], ...}`
//! -- one row per `VarBonus` the record's own `BONUS:VAR` chains produce,
//! in the SAME order `compute_var_effect` returns them.

use codex::rules_core::corpus_loader::{load_equipment_corpus, BookCorpusRoot};
use codex::rules_core::equipment_effects::general::compute_var_effect;
use codex::rules_core::equipment_resolver::equipment_id_resolve;
use codex::rules_core::rules_tables::RuleSetId;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

#[derive(serde::Deserialize)]
struct ManifestItem {
    unit_id: String,
    book: String,
    key: String,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: e5_var_shape_ours <repo_root> <manifest.json> <output.json>");
        return ExitCode::from(2);
    }
    let repo_root = Path::new(&args[1]);
    let manifest_path = &args[2];
    let output_path = &args[3];

    let manifest_text = match fs::read_to_string(manifest_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("failed to read manifest {manifest_path}: {e}");
            return ExitCode::from(1);
        }
    };
    let items: Vec<ManifestItem> = match serde_json::from_str(&manifest_text) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to parse manifest {manifest_path}: {e}");
            return ExitCode::from(1);
        }
    };

    let mut books: Vec<String> = items.iter().map(|i| i.book.clone()).collect();
    books.sort();
    books.dedup();
    let book_dirs: Vec<std::path::PathBuf> =
        books.iter().map(|b| repo_root.join("data/corpus").join(b)).collect();
    let roots: Vec<BookCorpusRoot> = books
        .iter()
        .zip(book_dirs.iter())
        .map(|(book_id, dir)| BookCorpusRoot { book_id, dir })
        .collect();
    let corpus = load_equipment_corpus(&roots);

    let mut out: BTreeMap<String, serde_json::Value> = BTreeMap::new();
    let mut unresolved: Vec<String> = Vec::new();
    let mut no_var_bonus: Vec<String> = Vec::new();

    for item in &items {
        let Some((record, _table_cell)) = equipment_id_resolve(&item.key, RuleSetId::Crb, &corpus)
        else {
            unresolved.push(item.unit_id.clone());
            continue;
        };
        let effect = compute_var_effect(record);
        if effect.is_empty() {
            no_var_bonus.push(item.unit_id.clone());
            continue;
        }
        let rows: Vec<serde_json::Value> = effect
            .iter()
            .map(|vb| serde_json::json!({"name": vb.name, "bonus": vb.bonus}))
            .collect();
        out.insert(item.unit_id.clone(), serde_json::json!(rows));
    }

    let serialized = serde_json::to_string_pretty(&out).unwrap();
    if let Err(e) = fs::write(output_path, serialized) {
        eprintln!("failed to write {output_path}: {e}");
        return ExitCode::from(1);
    }

    println!(
        "e5_var_shape_ours: {} units in manifest, {} resolved, {} unresolved, {} no-var-bonus -> {}",
        items.len(),
        out.len(),
        unresolved.len(),
        no_var_bonus.len(),
        output_path
    );
    if !unresolved.is_empty() {
        eprintln!("UNRESOLVED: {unresolved:?}");
    }
    if !no_var_bonus.is_empty() {
        eprintln!("NO VAR BONUS: {no_var_bonus:?}");
    }
    if out.len() != items.len() {
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}
