//! SD-33 AT-33-E5-002 remediation — repo-local "ours" batch probe for the
//! literal-verified `equipment` population's single-ability
//! STAT-enhancement / Belt-Headband shape.
//!
//! Replaces attempt 1's scratch crate (compiled OUTSIDE the repo — see
//! `artifacts/epic-5-reverification/ours-derivation/equipment-literal-ours-probe.rs`'s
//! own doc comment for why it lived there) with a real, repo-local binary,
//! per the remediation brief's explicit instruction: "Build a proper
//! repo-local binary under `src/bin/` that takes a unit list and emits our
//! values for all of them in one process." One process, one corpus load
//! per book, real live calls into
//! `codex::rules_core::equipment_effects::compute_equipment_effects` for
//! every unit in the input manifest — no stubs, no fixture-only values.
//!
//! Usage:
//!   e5_literal_stat_ours <repo_root> <manifest.json> <output.json>
//!
//! `manifest.json`: `[{"unit_id","book","key","ability_index","base_scores":
//! [str,dex,con,int,wis,cha]}, ...]` (the census/generator's own output
//! shape, `literal-scripts/census_stat_shape.py` +
//! `literal-scripts/generate_stat_pcgs.py`).
//!
//! Output: `{"unit_id": ["<slug>.STAT.<ability_index>.SCORE", <total>]}` —
//! the exact shape `scripts/oracle_harness/run.py --ours` reads, matching
//! AT-33-E5-001/002 attempt 1's oracle-export key convention
//! (`<slug>.<TOKEN>=<value>`, `literal-scripts/merge_oracle_export.py`).
//! `<total>` is the item's declared base ability score (16/14/14/10/10/8,
//! the same fixed `.pcg` baseline every AT-33-E5-00x cycle has used) PLUS
//! the REAL, live `ability_bonus.bonus` this cycle's engine call resolves
//! — i.e. this checks the applied total the same way attempt 1's did, not
//! merely that a bonus number parses.

use codex::rules_core::character_input::{ActiveState, EquipmentSelection};
use codex::rules_core::corpus_loader::{load_equipment_corpus, BookCorpusRoot};
use codex::rules_core::equipment_effects::compute_equipment_effects;
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const BASE_SCORES: [i16; 6] = [16, 14, 14, 10, 10, 8]; // STR DEX CON INT WIS CHA

#[derive(serde::Deserialize)]
struct ManifestItem {
    unit_id: String,
    book: String,
    key: String,
    ability_index: usize,
}

fn slug_of(unit_id: &str) -> &str {
    unit_id.rsplit(':').next().unwrap_or(unit_id)
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: e5_literal_stat_ours <repo_root> <manifest.json> <output.json>");
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

    // Load every distinct book referenced, once each -- one process, one
    // corpus load per book, real live engine calls for every unit.
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

    let mut out: BTreeMap<String, (String, serde_json::Value)> = BTreeMap::new();
    let mut unresolved: Vec<String> = Vec::new();
    let mut no_ability_bonus: Vec<String> = Vec::new();
    let mut resolved_ability: HashMap<String, String> = HashMap::new();

    for item in &items {
        let selection = EquipmentSelection {
            item_id: item.key.clone(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        };
        let effects = compute_equipment_effects(&[selection], &corpus);
        let Some(resolved) = effects.per_item.first() else {
            unresolved.push(item.unit_id.clone());
            continue;
        };
        let Some(ab) = resolved.ability_bonus.as_ref() else {
            no_ability_bonus.push(item.unit_id.clone());
            continue;
        };
        resolved_ability.insert(item.unit_id.clone(), ab.ability.clone());
        let base = BASE_SCORES[item.ability_index];
        let total = base + ab.bonus;
        let slug = slug_of(&item.unit_id);
        let oracle_key = format!("{slug}.STAT.{}.SCORE", item.ability_index);
        out.insert(item.unit_id.clone(), (oracle_key, serde_json::json!(total)));
    }

    let json_out: BTreeMap<&String, &(String, serde_json::Value)> = out.iter().collect();
    let serialized = serde_json::to_string_pretty(&json_out).unwrap();
    if let Err(e) = fs::write(output_path, serialized) {
        eprintln!("failed to write {output_path}: {e}");
        return ExitCode::from(1);
    }

    println!(
        "e5_literal_stat_ours: {} units in manifest, {} resolved, {} unresolved (no engine match), {} resolved-but-no-ability_bonus -> {}",
        items.len(),
        out.len(),
        unresolved.len(),
        no_ability_bonus.len(),
        output_path
    );
    if !unresolved.is_empty() {
        eprintln!("UNRESOLVED (equipment_id_resolve found no record): {unresolved:?}");
    }
    if !no_ability_bonus.is_empty() {
        eprintln!("NO ABILITY BONUS (compute_magic_items_effect returned None): {no_ability_bonus:?}");
    }
    if out.len() != items.len() {
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}
