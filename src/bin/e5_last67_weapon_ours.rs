//! SD-33 Epic 5 remediation wave 5, weapon/WEAPONPROF token-family lane
//! (`AT-33-E5-last67-weapon`) -- repo-local "ours" batch probe for the
//! 24-unit `WEAPONPROF=<x>` / bare `WEAPON` enhancement family named in
//! `AT-33-E5-last75_cycle_receipt.md`'s Finding 3.
//!
//! Same pattern as `src/bin/e5_equipment_remainder_skill_ours.rs`: one
//! process, one corpus load per book, real live calls into
//! `codex::rules_core::equipment_effects::compute_equipment_effects` for
//! every unit in the input manifest -- no stubs, no hand-typed "ours"
//! values. Reads `weapon_enhancement_bonus`
//! (`equipment_effects::equipmods::WeaponEnhancementBonus`, produced by
//! `equipmods::compute_equipmods_effect`, already covers this family per
//! wave 4's own finding -- zero `src/rules_core/` change needed for these
//! 24 units).
//!
//! Usage:
//!   e5_last67_weapon_ours <repo_root> <manifest.json> <output.json>
//!
//! `manifest.json`: `[{"unit_id","book","key"}, ...]`
//!
//! Output: `{"unit_id": {"tohit_bonus", "damage_bonus",
//! "natural_attack_only", "weapon_prof_scope"} | null, ...}` (updated by
//! the SD-33 remediation wave-5 finalize cycle: `WeaponEnhancementBonus`
//! split its single `bonus`/`affects` pair into independent
//! `tohit_bonus: Option<i16>`/`damage_bonus: Option<i16>` fields so a
//! record with two separately-scoped chains -- `ultimate_equipment:
//! equipment:heavy_hammer`'s real `WEAPONPROF=Warhammer|TOHIT|-2` +
//! `WEAPONPROF=Warhammer|DAMAGE|4` -- carries two different magnitudes
//! instead of the first chain silently shadowing the second; see
//! `equipmods.rs`'s own doc comment) -- `null` means this record carries
//! no `compute_equipmods_effect`-matched chain (the deliberately-excluded
//! shapes this lane's sibling shapes cover: bare `TYPE=Enhancement`-less
//! `WEAPON` chains, `WIELDCATEGORY` chains, `DAMAGEMULT` chains -- see
//! `equipmods.rs`'s own module doc comment for why those are excluded).

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
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: e5_last67_weapon_ours <repo_root> <manifest.json> <output.json>");
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
        let value = match &resolved.weapon_enhancement_bonus {
            Some(b) => serde_json::json!({
                "tohit_bonus": b.tohit_bonus,
                "damage_bonus": b.damage_bonus,
                "natural_attack_only": b.natural_attack_only,
                "weapon_prof_scope": b.weapon_prof_scope,
            }),
            None => serde_json::Value::Null,
        };
        out.insert(item.unit_id.clone(), value);
    }

    let serialized = serde_json::to_string_pretty(&out).unwrap();
    if let Err(e) = fs::write(output_path, serialized) {
        eprintln!("failed to write {output_path}: {e}");
        return ExitCode::from(1);
    }

    println!(
        "e5_last67_weapon_ours: {} units in manifest, {} resolved, {} unresolved -> {}",
        items.len(),
        out.len(),
        unresolved.len(),
        output_path
    );
    if !unresolved.is_empty() {
        eprintln!("UNRESOLVED: {unresolved:?}");
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}
