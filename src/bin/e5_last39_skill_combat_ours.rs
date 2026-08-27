//! SD-33 remediation wave 6, `skill-combat-final` lane (`AT-33-E5-last39-
//! skill-combat`) -- repo-local "ours" batch probe for this lane's own
//! 11-unit population (the 39-of-8,330 unrowed-remainder's SKILL/COMBAT-
//! shape units, re-derived by reading every candidate's own
//! `raw_bonus_chains` -- see the cycle receipt for the full re-derivation).
//!
//! Same pattern as every prior `AT-33-E5-00x` batch probe: one process, one
//! corpus load per book, real live calls into
//! `codex::rules_core::equipment_effects::compute_equipment_effects` -- no
//! stubs, no hand-typed "ours" values. Dumps every field a COMBAT/SKILL/
//! WEAPON-shaped record could plausibly populate
//! (`armor_class_bonus`, `skill_bonus`, `weapon_enhancement_bonus`,
//! `to_hit_bonus`) so this lane's verdict (`agree`/`unverifiable`) is
//! backed by the engine's real, observed output, not by static reading of
//! the source alone.
//!
//! Usage:
//!   e5_last39_skill_combat_ours <repo_root> <manifest.json> <output.json>
//!
//! `manifest.json`: `[{"unit_id","book","key"}, ...]`.
//!
//! Output: `{"unit_id": {"resolved": bool, "armor_class_bonus": ..,
//! "skill_bonus": {"skill":..,"bonus":..} | null,
//! "weapon_enhancement_bonus": {"tohit_bonus":..,"damage_bonus":..} | null,
//! "to_hit_bonus": .. }, ...}`.

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

#[derive(serde::Serialize)]
struct OutRow {
    resolved: bool,
    armor_class_bonus: Option<i16>,
    skill_bonus: Option<(String, i16)>,
    weapon_enhancement_bonus: Option<(Option<i16>, Option<i16>)>,
    to_hit_bonus: Option<i16>,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: e5_last39_skill_combat_ours <repo_root> <manifest.json> <output.json>");
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
            out.insert(
                item.unit_id.clone(),
                OutRow {
                    resolved: false,
                    armor_class_bonus: None,
                    skill_bonus: None,
                    weapon_enhancement_bonus: None,
                    to_hit_bonus: None,
                },
            );
            continue;
        };
        out.insert(
            item.unit_id.clone(),
            OutRow {
                resolved: true,
                armor_class_bonus: resolved.armor_class_bonus,
                skill_bonus: resolved
                    .skill_bonus
                    .as_ref()
                    .map(|sb| (sb.skill.clone(), sb.bonus)),
                weapon_enhancement_bonus: resolved
                    .weapon_enhancement_bonus
                    .as_ref()
                    .map(|w| (w.tohit_bonus, w.damage_bonus)),
                to_hit_bonus: resolved.to_hit_bonus,
            },
        );
    }

    let serialized = serde_json::to_string_pretty(&out).unwrap();
    if let Err(e) = fs::write(output_path, serialized) {
        eprintln!("failed to write {output_path}: {e}");
        return ExitCode::from(1);
    }

    println!(
        "e5_last39_skill_combat_ours: {} units in manifest, {} unresolved -> {}",
        items.len(),
        unresolved.len(),
        output_path
    );
    if !unresolved.is_empty() {
        eprintln!("UNRESOLVED: {unresolved:?}");
    }
    ExitCode::SUCCESS
}
