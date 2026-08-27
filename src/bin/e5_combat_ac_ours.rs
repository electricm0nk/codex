//! SD-33 Epic 5 combat/weapon lane (AT-33-E5-002 remainder) -- repo-local
//! "ours" batch probe for the equipment `other_bonus_shape` COMBAT|AC
//! sub-population.
//!
//! Same pattern as `src/bin/e5_equipment_remainder_skill_ours.rs`: one
//! process, one corpus load per book, real live calls into
//! `codex::rules_core::equipment_effects::compute_equipment_effects` for
//! every unit in the input manifest -- no stubs, no fixture-only values.
//! Reads `armor_class_bonus` (`equipment_effects::EquipmentStatEffect`,
//! produced by `arms_armor::compute_arms_armor_effect`, widened this
//! cycle to cover any `COMBAT|AC` chain, not just Armor/Shield-typed
//! ones -- see that module's own doc comment).
//!
//! Usage:
//!   e5_combat_ac_ours <repo_root> <manifest.json> <output.json>
//!
//! `manifest.json`: `{"items": [{"unit_id","book","key","expected_bonus"}, ...], "baselines": [...]}`
//! (`ac_generate.py`'s own manifest shape -- `baselines` is read but not
//! probed here, since a baseline character has no comparable item).
//!
//! Output: `{"unit_id": ["AC.TOTAL_DELTA", bonus], ...}` -- the exact
//! shape `scripts/oracle_harness/run.py --ours` reads. The oracle side
//! (built by `ac_build_results.py`) computes the SAME key
//! (`AC.TOTAL_DELTA`) as `oracle_item_AC.Total - oracle_baseline_AC.Total`
//! for the matching book, since PCGen's own `AC.Total` export token has
//! no per-bonus-type breakdown reliable across this population's real
//! grammar variety (see `ac_generate.py`'s own module doc comment).

use codex::rules_core::character_input::{ActiveState, EquipmentSelection};
use codex::rules_core::corpus_loader::{load_equipment_corpus, BookCorpusRoot};
use codex::rules_core::equipment_effects::compute_equipment_effects;
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
    expected_bonus: i16,
}

#[derive(serde::Deserialize)]
struct Manifest {
    items: Vec<ManifestItem>,
    #[allow(dead_code)]
    baselines: Vec<serde_json::Value>,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: e5_combat_ac_ours <repo_root> <manifest.json> <output.json>");
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
    let manifest: Manifest = match serde_json::from_str(&manifest_text) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to parse manifest {manifest_path}: {e}");
            return ExitCode::from(1);
        }
    };
    let items = manifest.items;

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
    let mut no_ac_bonus: Vec<String> = Vec::new();
    let mut value_mismatch: Vec<String> = Vec::new();

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
        let Some(bonus) = resolved.armor_class_bonus else {
            no_ac_bonus.push(item.unit_id.clone());
            continue;
        };
        if bonus != item.expected_bonus {
            // Real, execution-observed disagreement between the manifest's
            // own reading of the FIRST COMBAT|AC chain and
            // `compute_arms_armor_effect`'s (both read the same first-match
            // chain -- a mismatch here means the manifest is stale
            // relative to this cycle's corpus read, or a real multi-chain
            // record where "first" differs between the two readers).
            value_mismatch.push(format!(
                "{} (manifest={} engine={})",
                item.unit_id, item.expected_bonus, bonus
            ));
            continue;
        }
        let slug = item.unit_id.rsplit(':').next().unwrap_or(&item.unit_id);
        out.insert(item.unit_id.clone(), (format!("{slug}.AC.TOTAL_DELTA"), serde_json::json!(bonus)));
    }

    let json_out: BTreeMap<&String, &(String, serde_json::Value)> = out.iter().collect();
    let serialized = serde_json::to_string_pretty(&json_out).unwrap();
    if let Err(e) = fs::write(output_path, serialized) {
        eprintln!("failed to write {output_path}: {e}");
        return ExitCode::from(1);
    }

    println!(
        "e5_combat_ac_ours: {} units in manifest, {} resolved, {} unresolved, {} no-ac-bonus, {} value-mismatch -> {}",
        items.len(),
        out.len(),
        unresolved.len(),
        no_ac_bonus.len(),
        value_mismatch.len(),
        output_path
    );
    if !unresolved.is_empty() {
        eprintln!("UNRESOLVED: {unresolved:?}");
    }
    if !no_ac_bonus.is_empty() {
        eprintln!("NO AC BONUS: {no_ac_bonus:?}");
    }
    if !value_mismatch.is_empty() {
        eprintln!("VALUE MISMATCH: {value_mismatch:?}");
    }
    if out.len() != items.len() {
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}
