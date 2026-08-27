//! SD-33 equipment remainder lane (AT-33-E5-001/002 remainder) --
//! repo-local "ours" batch probe for the equipment `other_bonus_shape`
//! single-skill `BONUS:SKILL|<skill>|<n>|...` sub-population.
//!
//! Same pattern as `src/bin/e5_literal_stat_ours.rs` (one process, one
//! corpus load per book, real live calls into
//! `codex::rules_core::equipment_effects::compute_equipment_effects` for
//! every unit in the input manifest -- no stubs, no fixture-only values),
//! reading `skill_bonus` (`equipment_effects::general::SkillCheckBonus`,
//! produced by `compute_general_effect`) instead of `ability_bonus`.
//!
//! Usage:
//!   e5_equipment_remainder_skill_ours <repo_root> <manifest.json> <output.json>
//!
//! `manifest.json`: `[{"unit_id","book","key","skill","expected_bonus"}, ...]`
//! (`equipment-remainder-generate-skill-pcgs.py`'s own manifest shape).
//!
//! Output: `{"unit_id": ["<slug>.SKILL.MISC", bonus], ...}` -- the exact
//! shape `scripts/oracle_harness/run.py --ours` reads. `<slug>.SKILL.MISC`
//! matches `equipment-remainder-checks.txt.ftl`'s sibling per-unit
//! `<slug>.txt.ftl` templates (`SKILL.MISC=${pcstring('SKILL.<name>.MISC')}`,
//! PCGen's own `SkillToken.SKILL_MISC` -- `modifier(aSkill,pc) -
//! getStatMod(aSkill,pc)`, i.e. the skill's total bonus minus its ability
//! score component, isolating exactly the item's circumstance bonus on a
//! 0-rank test character).

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
    skill: String,
    expected_bonus: i16,
}

fn slug_of(unit_id: &str) -> &str {
    unit_id.rsplit(':').next().unwrap_or(unit_id)
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: e5_equipment_remainder_skill_ours <repo_root> <manifest.json> <output.json>");
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

    let mut out: BTreeMap<String, (String, serde_json::Value)> = BTreeMap::new();
    let mut unresolved: Vec<String> = Vec::new();
    let mut no_skill_bonus: Vec<String> = Vec::new();
    let mut skill_name_mismatch: Vec<String> = Vec::new();

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
        if sb.skill != item.skill {
            // Real, execution-observed disagreement between the census's
            // own reading of the FIRST SKILL chain and
            // `compute_general_effect`'s (they should always match --
            // both read the same first-match chain -- a mismatch here
            // means the manifest's own skill field is stale relative to
            // this cycle's corpus read, not a value comparison).
            skill_name_mismatch.push(format!("{} (manifest={} engine={})", item.unit_id, item.skill, sb.skill));
            continue;
        }
        let slug = slug_of(&item.unit_id);
        let oracle_key = format!("{slug}.SKILL.MISC");
        out.insert(item.unit_id.clone(), (oracle_key, serde_json::json!(sb.bonus)));
        // expected_bonus is carried through the manifest for the census's
        // own real, corpus-derived record; not used for computation here
        // (that would be circular) -- referenced only so a future reader
        // auditing this file's diff sees intent, matching
        // `e5_literal_stat_ours.rs`'s own precedent of no unused-field
        // warnings.
        let _ = item.expected_bonus;
    }

    let json_out: BTreeMap<&String, &(String, serde_json::Value)> = out.iter().collect();
    let serialized = serde_json::to_string_pretty(&json_out).unwrap();
    if let Err(e) = fs::write(output_path, serialized) {
        eprintln!("failed to write {output_path}: {e}");
        return ExitCode::from(1);
    }

    println!(
        "e5_equipment_remainder_skill_ours: {} units in manifest, {} resolved, {} unresolved, {} no-skill-bonus, {} skill-name-mismatch -> {}",
        items.len(),
        out.len(),
        unresolved.len(),
        no_skill_bonus.len(),
        skill_name_mismatch.len(),
        output_path
    );
    if !unresolved.is_empty() {
        eprintln!("UNRESOLVED: {unresolved:?}");
    }
    if !no_skill_bonus.is_empty() {
        eprintln!("NO SKILL BONUS: {no_skill_bonus:?}");
    }
    if !skill_name_mismatch.is_empty() {
        eprintln!("SKILL NAME MISMATCH: {skill_name_mismatch:?}");
    }
    if out.len() != items.len() {
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}
