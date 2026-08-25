//! SD-33 remediation wave 3, `sd33-r3-statsave` lane -- repo-local "ours"
//! batch probe for this lane's own 42-unit equipment `SKILL` sub-population
//! (`docs/release/SD-33-computed-value-verification/artifacts/
//! epic-5-reverification/AT-33-E5-shape-stat-save-tail_cycle_receipt.md`).
//!
//! Same one-process/one-corpus-load-per-book pattern as
//! `e5_literal_stat_ours.rs` / `e5_equipment_remainder_skill_ours.rs`, real
//! live calls into `compute_equipment_effects`, reading `skill_bonus`
//! (`equipment_effects::general::SkillCheckBonus`). Unlike
//! `e5_equipment_remainder_skill_ours.rs` (which requires the manifest's
//! `skill` field to equal the engine's `sb.skill` verbatim -- correct for
//! that lane's single-skill-only population), this binary's manifest
//! carries a `target_skill` distinct from the item's real, possibly
//! comma-joined `raw_skill_field` -- this lane's own population includes
//! multi-skill items (`BONUS:SKILL|Bluff,Diplomacy|...`), verified against
//! the FIRST named skill only (this lane's own documented scope decision:
//! PF1 applies the same bonus to every named skill independently, so this
//! checks the mechanism once per unit, matching the sibling STAT lane's own
//! first-named-target convention for multi-ability items). This binary only
//! asserts that the engine's raw skill field STARTS WITH the target (a
//! real, observable relationship for a first-named-skill target), not
//! exact equality -- and always records the engine's own raw field
//! alongside the bonus for audit.
//!
//! Usage:
//!   e5_statsave_skill_ours <repo_root> <manifest.json> <output.json>
//!
//! `manifest.json`: `[{"unit_id","book","key","target_skill"}, ...]`.
//!
//! Output: `{"unit_id": {"oracle_key": "<slug>.SKILL.MISC", "ours": <bonus>,
//! "engine_raw_skill_field": "<sb.skill>"}, ...}`.

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
    target_skill: String,
}

#[derive(serde::Serialize)]
struct OutRow {
    oracle_key: String,
    ours: i16,
    engine_raw_skill_field: String,
}

fn slug_of(unit_id: &str) -> &str {
    unit_id.rsplit(':').next().unwrap_or(unit_id)
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: e5_statsave_skill_ours <repo_root> <manifest.json> <output.json>");
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

    let mut out: BTreeMap<String, OutRow> = BTreeMap::new();
    let mut unresolved: Vec<String> = Vec::new();
    let mut no_skill_bonus: Vec<String> = Vec::new();
    let mut target_not_prefix: Vec<String> = Vec::new();

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
        let Some(sb) = resolved.skill_bonus.as_ref() else {
            no_skill_bonus.push(item.unit_id.clone());
            continue;
        };
        let first_named: &str = sb.skill.split(',').next().unwrap_or(&sb.skill).trim();
        if first_named != item.target_skill {
            target_not_prefix.push(format!(
                "{} (target={} engine_first_named={} engine_raw={})",
                item.unit_id, item.target_skill, first_named, sb.skill
            ));
            continue;
        }
        let slug = slug_of(&item.unit_id);
        out.insert(
            item.unit_id.clone(),
            OutRow {
                oracle_key: format!("{slug}.SKILL.MISC"),
                ours: sb.bonus,
                engine_raw_skill_field: sb.skill.clone(),
            },
        );
    }

    let serialized = serde_json::to_string_pretty(&out).unwrap();
    if let Err(e) = fs::write(output_path, serialized) {
        eprintln!("failed to write {output_path}: {e}");
        return ExitCode::from(1);
    }

    println!(
        "e5_statsave_skill_ours: {} units in manifest, {} resolved, {} unresolved, {} no-skill-bonus, {} target-mismatch -> {}",
        items.len(),
        out.len(),
        unresolved.len(),
        no_skill_bonus.len(),
        target_not_prefix.len(),
        output_path
    );
    if !unresolved.is_empty() {
        eprintln!("UNRESOLVED: {unresolved:?}");
    }
    if !no_skill_bonus.is_empty() {
        eprintln!("NO SKILL BONUS: {no_skill_bonus:?}");
    }
    if !target_not_prefix.is_empty() {
        eprintln!("TARGET MISMATCH: {target_not_prefix:?}");
    }
    if out.len() != items.len() {
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}
