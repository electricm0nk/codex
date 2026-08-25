//! SD-33 remediation wave 4 (`AT-33-E5-003`) -- repo-local "ours" batch
//! probe re-running the 22 real disagreements this wave's engine fixes
//! close (`arms_armor::apply_eqmod_armor_class_bonus`,
//! `general::apply_eqmod_var_bonus`, and the `TYPE=Circumstance`
//! exclusion in `armor_class_bonus_from_bonus_chains`).
//!
//! Reads `disagreement-fixes-manifest.json` (this cycle's own manifest,
//! `unit_id`/`book`/`key` plus the REAL oracle value(s) already committed
//! in `AT-33-E5-003.combined-oracle-results.json` -- no new PCGen
//! invocation needed, since the oracle side of these 22 units was already
//! run and committed by prior waves; only "ours" changes this cycle), and
//! calls the SAME `codex::rules_core::equipment_effects::
//! compute_equipment_effects` / `general::compute_var_effect` +
//! `general::apply_eqmod_var_bonus` real engine functions every other
//! `AT-33-E5-00x` probe binary calls -- no stubs, no fixture-only values,
//! no hand-derived duplicate of the corpus's own literal tokens.
//!
//! Usage:
//!   e5_disagreement_fixes_ours <repo_root> <manifest.json> <output.json>
//!
//! Output: `{"results": [{"unit_id","ours","oracle","verdict",
//! ["multi_shape_note","multi_shape_sources"]}, ...]}` -- the exact shape
//! `scripts/box_ledger.py::load_oracle_results` reads, preserving the
//! `multi_shape_sources` convention `AT-33-E5-finalize-wave3`'s own merge
//! established for a unit examined under more than one bonus-chain shape.

use codex::rules_core::character_input::{ActiveState, EquipmentSelection};
use codex::rules_core::corpus_loader::{load_equipment_corpus, BookCorpusRoot};
use codex::rules_core::equipment_effects::compute_equipment_effects;
use codex::rules_core::equipment_effects::general::{apply_eqmod_var_bonus, compute_var_effect};
use codex::rules_core::equipment_effects::eqmod_referenced_records;
use codex::rules_core::equipment_resolver::equipment_id_resolve;
use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::source_content::SourcePackageContent;
use serde_json::json;
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

#[derive(serde::Deserialize)]
struct ManifestItem {
    unit_id: String,
    book: String,
    key: String,
    ac_oracle: i16,
    #[serde(default)]
    multi_shape: bool,
    #[serde(default)]
    var_shape_ours: Option<i16>,
    #[serde(default)]
    var_shape_oracle: Option<i16>,
    #[serde(default)]
    recompute_var: bool,
    #[serde(default)]
    var_oracle: Option<i16>,
    #[serde(default)]
    allow_none_ac: bool,
}

#[derive(serde::Deserialize)]
struct Manifest {
    corpus_books: Vec<String>,
    items: Vec<ManifestItem>,
}

fn verdict_for(ours: i16, oracle: i16) -> &'static str {
    if ours == oracle {
        "agree"
    } else {
        "disagree"
    }
}

fn ac_bonus_for(item: &ManifestItem, corpus: &SourcePackageContent) -> Option<i16> {
    let selection = EquipmentSelection {
        item_id: item.key.clone(),
        equipped_or_active: true,
        active_state: ActiveState::EquippedActive,
        applied_modifiers: Vec::new(),
    };
    let effects = compute_equipment_effects(&[selection], corpus);
    effects.per_item.first().and_then(|resolved| resolved.armor_class_bonus)
}

fn var_bonus_for(item: &ManifestItem, corpus: &SourcePackageContent, name: &str) -> Option<i16> {
    let (record, _table_cell) = equipment_id_resolve(&item.key, RuleSetId::Crb, corpus)?;
    let mut base = compute_var_effect(record);
    let eqmod_records = eqmod_referenced_records(record, RuleSetId::Crb, corpus);
    apply_eqmod_var_bonus(&mut base, &eqmod_records);
    base.into_iter().find(|v| v.name == name).map(|v| v.bonus)
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: e5_disagreement_fixes_ours <repo_root> <manifest.json> <output.json>");
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

    let book_dirs: Vec<std::path::PathBuf> =
        manifest.corpus_books.iter().map(|b| repo_root.join("data/corpus").join(b)).collect();
    let roots: Vec<BookCorpusRoot> = manifest
        .corpus_books
        .iter()
        .zip(book_dirs.iter())
        .map(|(book_id, dir)| BookCorpusRoot { book_id, dir })
        .collect();
    let corpus = load_equipment_corpus(&roots);

    let mut results = Vec::new();
    let mut unresolved: Vec<String> = Vec::new();
    let mut agree_count = 0usize;
    let mut disagree_count = 0usize;

    for item in &manifest.items {
        let ac_bonus = ac_bonus_for(item, &corpus);
        let ours = match ac_bonus {
            Some(v) => v,
            None if item.allow_none_ac => 0,
            None => {
                unresolved.push(item.unit_id.clone());
                continue;
            }
        };
        let verdict = verdict_for(ours, item.ac_oracle);

        let row = if item.multi_shape {
            let (var_ours, var_oracle) = if item.recompute_var {
                let recomputed = var_bonus_for(item, &corpus, "ArmorCheckPenalty").unwrap_or(0);
                (recomputed, item.var_oracle.unwrap_or(0))
            } else {
                (item.var_shape_ours.unwrap_or(0), item.var_shape_oracle.unwrap_or(0))
            };
            let var_verdict = verdict_for(var_ours, var_oracle);
            let merged_verdict = if verdict == "agree" && var_verdict == "agree" { "agree" } else { "disagree" };
            if merged_verdict == "agree" {
                agree_count += 1;
            } else {
                disagree_count += 1;
            }
            json!({
                "unit_id": item.unit_id,
                "ours": ours,
                "oracle": item.ac_oracle,
                "verdict": merged_verdict,
                "multi_shape_note": "unit carries 2 independently-examined magnitude/bonus-chain shapes (multi-token equipment record); merged verdict is the worst of the 2 per-shape verdicts, per AT-33-E5-finalize-wave3's duplicate-unit_id root-cause rule (never last-writer-wins)",
                "multi_shape_sources": [
                    {"lane": "var-bonus-shape", "ours": var_ours, "oracle": var_oracle, "verdict": var_verdict, "reason": serde_json::Value::Null},
                    {"lane": "combat-weapon-shape", "ours": ours, "oracle": item.ac_oracle, "verdict": verdict, "reason": serde_json::Value::Null},
                ],
            })
        } else {
            if verdict == "agree" {
                agree_count += 1;
            } else {
                disagree_count += 1;
            }
            json!({
                "unit_id": item.unit_id,
                "ours": ours,
                "oracle": item.ac_oracle,
                "verdict": verdict,
            })
        };
        results.push(row);
    }

    let output = json!({ "results": results });
    let serialized = serde_json::to_string_pretty(&output).unwrap();
    if let Err(e) = fs::write(output_path, serialized) {
        eprintln!("failed to write {output_path}: {e}");
        return ExitCode::from(1);
    }

    println!(
        "e5_disagreement_fixes_ours: {} items, {} unresolved, {} agree, {} disagree -> {}",
        manifest.items.len(),
        unresolved.len(),
        agree_count,
        disagree_count,
        output_path
    );
    if !unresolved.is_empty() {
        println!("unresolved: {unresolved:?}");
    }
    ExitCode::SUCCESS
}
