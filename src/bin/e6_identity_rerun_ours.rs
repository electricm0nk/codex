//! SD-33 remediation wave 6 (`sd33-r6-method-rerun`) -- repo-local "ours"
//! recompute for the 5 examined rows still carrying a STALE
//! pre-`9df1c0b514` identity-resolve-failure reason
//! (`equipment_id_resolve_no_match_keyless_outputname_record` /
//! `engine_id_resolve_fails_templated_variant_record`) in
//! `AT-33-E5-003.combined-oracle-results.json`, after the `corpus_loader.rs`
//! KEY-synthesis fix (`sd33-r5-skillcombat`) landed. Confirms, live, whether
//! `equipment_id_resolve` now finds each record and what the current engine
//! computes for it -- no new resolver logic, only calls into the SAME
//! `equipment_id_resolve` / `compute_var_effect` / `compute_equipment_effects`
//! functions every other `AT-33-E5-00x` probe binary calls.
//!
//! Usage: e6_identity_rerun_ours <repo_root> <output.json>

use codex::rules_core::character_input::{ActiveState, EquipmentSelection};
use codex::rules_core::corpus_loader::{load_equipment_corpus, BookCorpusRoot};
use codex::rules_core::equipment_effects::compute_equipment_effects;
use codex::rules_core::equipment_effects::general::compute_var_effect;
use codex::rules_core::equipment_resolver::equipment_id_resolve;
use codex::rules_core::rules_tables::RuleSetId;
use serde_json::json;
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

struct Item {
    unit_id: &'static str,
    book: &'static str,
    key: &'static str,
    shape: &'static str, // "var" or "ac"
    var_name: &'static str,
}

const ITEMS: &[Item] = &[
    Item {
        unit_id: "advanced_players_guide:equipment:backpack_masterwork",
        book: "advanced_players_guide",
        key: "Backpack (Masterwork)",
        shape: "var",
        var_name: "LOADSCORE",
    },
    Item {
        unit_id: "ultimate_psionics:equipment:companion_stone_electrical_protection",
        book: "ultimate_psionics",
        key: "Companion Stone (Electrical Protection)",
        shape: "var",
        var_name: "ElectricityResistanceBonus",
    },
    Item {
        unit_id: "ultimate_psionics:equipment:psychoactive_skin_psion",
        book: "ultimate_psionics",
        key: "Psychoactive Skin (Psion)",
        shape: "var",
        var_name: "BonusPowerPoints",
    },
    Item {
        unit_id: "ultimate_psionics:equipment:psychoactive_skin_defender",
        book: "ultimate_psionics",
        key: "Psychoactive Skin (Defender)",
        shape: "ac",
        var_name: "",
    },
    Item {
        unit_id: "ultimate_psionics:equipment:psychoactive_skin_hero",
        book: "ultimate_psionics",
        key: "Psychoactive Skin (Hero)",
        shape: "ac",
        var_name: "",
    },
];

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: e6_identity_rerun_ours <repo_root> <output.json>");
        return ExitCode::from(2);
    }
    let repo_root = Path::new(&args[1]);
    let output_path = &args[2];

    let mut books: Vec<String> = ITEMS.iter().map(|i| i.book.to_string()).collect();
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

    let mut rows = Vec::new();
    for item in ITEMS {
        let resolved = equipment_id_resolve(item.key, RuleSetId::Crb, &corpus);
        let resolves = resolved.is_some();
        let mut row = json!({
            "unit_id": item.unit_id,
            "key": item.key,
            "shape": item.shape,
            "equipment_id_resolve_now_succeeds": resolves,
        });
        if let Some((record, _)) = resolved {
            if item.shape == "var" {
                let vars = compute_var_effect(record);
                let bonus = vars.iter().find(|v| v.name == item.var_name).map(|v| v.bonus);
                row["var_name"] = json!(item.var_name);
                row["ours"] = json!(bonus);
                row["all_vars_found"] = json!(vars
                    .iter()
                    .map(|v| format!("{}={}", v.name, v.bonus))
                    .collect::<Vec<_>>());
            } else {
                let selection = EquipmentSelection {
                    item_id: item.key.to_string(),
                    equipped_or_active: true,
                    active_state: ActiveState::EquippedActive,
                    applied_modifiers: Vec::new(),
                };
                let effects = compute_equipment_effects(&[selection], &corpus);
                if let Some(r) = effects.per_item.first() {
                    row["ours_ac"] = json!(r.armor_class_bonus);
                    row["ours_tohit"] = json!(r
                        .weapon_enhancement_bonus
                        .as_ref()
                        .and_then(|w| w.tohit_bonus));
                    row["ours_skill_bonus_present"] = json!(r.skill_bonus.is_some());
                } else {
                    row["ours_ac"] = json!(null);
                }
            }
        }
        rows.push(row);
    }

    let out = json!({ "results": rows });
    let serialized = serde_json::to_string_pretty(&out).unwrap();
    if let Err(e) = fs::write(output_path, serialized) {
        eprintln!("failed to write {output_path}: {e}");
        return ExitCode::from(1);
    }
    println!("e6_identity_rerun_ours: {} items -> {}", ITEMS.len(), output_path);
    for r in &rows {
        println!("{}", r);
    }
    ExitCode::SUCCESS
}
