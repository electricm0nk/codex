//! SD-33 remediation wave 5 (`AT-33-E5-003`) -- absolute-method oracle
//! isolator for the AC-shape population `AT-33-E5-shape-combat`'s
//! whole-character `AC.TOTAL` diff already judged.
//!
//! `AT-33-E5-shape-combat`'s harness computed `oracle = item AC.TOTAL -
//! baseline AC.TOTAL`. That diff conflates the item's own
//! `armor_class_bonus` (what this engine computes and what
//! `AT-33-E5-003` grades against) with second-order effects the diff
//! cannot separate: a `MAXDEX` cap reducing the baseline's own Dex bonus
//! when the item is worn, or a co-located ability-score-enhancement
//! chain on the SAME record raising `AC.Total` via the normal Dex-to-AC
//! path. Both are real PF1 rules effects, but neither is part of the
//! item's own `COMBAT|AC` bonus-chain magnitude -- confirmed this cycle
//! by direct arithmetic on the already-committed raw exports for all 4
//! of `AT-33-E5-003`'s remaining `disagree` rows
//! (`docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003-disagreement-fixes_cycle_receipt.md`).
//!
//! This binary emits, per item, the exact set of PCGen bonus-`TYPE`
//! strings this engine's own `armor_class_bonus` is built from -- the
//! base record's first non-Circumstance `COMBAT|AC` chain's type, plus
//! each EQMOD-referenced modifier's own chain's type: the IDENTICAL
//! predicate `arms_armor::armor_class_bonus_from_bonus_chains` /
//! `arms_armor::apply_eqmod_armor_class_bonus` use (that function is
//! private to its module and returns only the numeric value, not the
//! type label, so the same match/skip-Circumstance/first-match logic is
//! duplicated here read-only, for isolation purposes only -- never a
//! second COMPUTE path: this binary never decides `armor_class_bonus`
//! itself, it calls `compute_equipment_effects` for that, exactly like
//! every other `AT-33-E5-00x` "ours" probe).
//!
//! A driver script (`combat-shape-work/ac_isolate_run.py`) then queries
//! PCGen's own `BONUS.COMBAT.AC.<Type>` export token directly, per type,
//! against the SAME single-item character build already committed under
//! `combat-shape-work/ac-pcg/` -- no baseline character needed at all,
//! since this asks PCGen for a bonus SUBTOTAL by type, not a whole-
//! character AC total, so nothing about Dex or a MAXDEX cap can leak in.
//!
//! Also emits this engine's own FRESH (current-code) `armor_class_bonus`
//! per item, so a row whose real fix landed in a commit later than the
//! row was last written (e.g. `full_plate_of_the_corpse`, fixed by the
//! general EQMOD resolver `abc72f75ec` landed but never re-run for this
//! specific unit) is never silently trusted as up to date.
//!
//! Usage:
//!   e5_ac_isolator <repo_root> <manifest.json> <output.json>
//!
//! `manifest.json` is `combat-shape-work/ac-manifest.json`'s own
//! `{"corpus_books": [...], "items": [{"unit_id","book","key","slug",...}]}`
//! shape (already committed, reused verbatim -- not regenerated).
//!
//! Output: `{"items": [{"unit_id","book","key","slug","ours","types":[...]}]}`.

use codex::pcgen_import::lst_parser::equipment::EquipmentRecord;
use codex::rules_core::character_input::{ActiveState, EquipmentSelection};
use codex::rules_core::corpus_loader::{load_equipment_corpus, BookCorpusRoot};
use codex::rules_core::equipment_effects::compute_equipment_effects;
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
    slug: String,
}

#[derive(serde::Deserialize)]
struct Manifest {
    /// Optional: `combat-shape-work/ac-manifest.json` (this binary's
    /// primary input) carries no `corpus_books` field at all -- its own
    /// `items`/`baselines` book sets are identical (confirmed:
    /// `python3 -c "import json; d=json.load(open('...ac-manifest.json'));
    /// print(sorted(set(i['book'] for i in d['items'])) ==
    /// sorted(set(b['book'] for b in d['baselines'])))"` -> `True`), so
    /// when absent this defaults to the unique set of `items[].book`
    /// values, which is already the exhaustive book list this AC lane's
    /// own oracle-generation cycle loaded.
    #[serde(default)]
    corpus_books: Vec<String>,
    items: Vec<ManifestItem>,
}

/// Mirrors `arms_armor::armor_class_bonus_from_bonus_chains`'s own
/// predicate exactly (first non-`TYPE=Circumstance` `COMBAT|AC|<n>`
/// chain), but additionally returns the chain's own type label --
/// `"UNTYPED"` for the real bare-qualifier corpus grammar quirk that
/// function's own doc comment names (a `COMBAT|AC|<n>` chain with no
/// `TYPE=`/`TYPE.` prefix at all), or the literal string after `TYPE=`
/// otherwise.
fn ac_chain_type(record: &EquipmentRecord) -> Option<(i16, String)> {
    record.bonus_chains.iter().find_map(|bonus| {
        let qualifiers = &bonus.qualifiers;
        let is_ac_bonus = qualifiers.len() >= 3
            && qualifiers[0] == "COMBAT"
            && qualifiers[1] == "AC"
            && !qualifiers.iter().any(|q| q == "TYPE=Circumstance");
        if !is_ac_bonus {
            return None;
        }
        let value: i16 = qualifiers[2].parse().ok()?;
        let type_label = qualifiers
            .iter()
            .find(|q| q.starts_with("TYPE="))
            .map(|q| q.trim_start_matches("TYPE=").to_string())
            .unwrap_or_else(|| "UNTYPED".to_string());
        Some((value, type_label))
    })
}

fn ac_bonus_for(key: &str, corpus: &SourcePackageContent) -> Option<i16> {
    let selection = EquipmentSelection {
        item_id: key.to_string(),
        equipped_or_active: true,
        active_state: ActiveState::EquippedActive,
        applied_modifiers: Vec::new(),
    };
    let effects = compute_equipment_effects(&[selection], corpus);
    effects.per_item.first().and_then(|resolved| resolved.armor_class_bonus)
}

fn types_for(key: &str, corpus: &SourcePackageContent) -> Vec<String> {
    let mut types = Vec::new();
    let Some((record, _table_cell)) = equipment_id_resolve(key, RuleSetId::Crb, corpus) else {
        return types;
    };
    if let Some((_v, t)) = ac_chain_type(record) {
        types.push(t);
    }
    for modifier in eqmod_referenced_records(record, RuleSetId::Crb, corpus) {
        if let Some((_v, t)) = ac_chain_type(modifier) {
            types.push(t);
        }
    }
    types.sort();
    types.dedup();
    types
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: e5_ac_isolator <repo_root> <manifest.json> <output.json>");
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
    let mut manifest: Manifest = match serde_json::from_str(&manifest_text) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to parse manifest {manifest_path}: {e}");
            return ExitCode::from(1);
        }
    };
    if manifest.corpus_books.is_empty() {
        let mut books: Vec<String> = manifest.items.iter().map(|i| i.book.clone()).collect();
        books.sort();
        books.dedup();
        manifest.corpus_books = books;
    }

    let book_dirs: Vec<std::path::PathBuf> =
        manifest.corpus_books.iter().map(|b| repo_root.join("data/corpus").join(b)).collect();
    let roots: Vec<BookCorpusRoot> = manifest
        .corpus_books
        .iter()
        .zip(book_dirs.iter())
        .map(|(book_id, dir)| BookCorpusRoot { book_id, dir })
        .collect();
    let corpus = load_equipment_corpus(&roots);

    let mut items = Vec::new();
    let mut unresolved = Vec::new();
    let mut no_types = Vec::new();
    for item in &manifest.items {
        let ours = ac_bonus_for(&item.key, &corpus);
        let types = types_for(&item.key, &corpus);
        if ours.is_none() {
            unresolved.push(item.unit_id.clone());
        }
        if types.is_empty() {
            no_types.push(item.unit_id.clone());
        }
        items.push(json!({
            "unit_id": item.unit_id,
            "book": item.book,
            "key": item.key,
            "slug": item.slug,
            "ours": ours,
            "types": types,
        }));
    }

    let output = json!({ "items": items });
    let serialized = serde_json::to_string_pretty(&output).unwrap();
    if let Err(e) = fs::write(output_path, serialized) {
        eprintln!("failed to write {output_path}: {e}");
        return ExitCode::from(1);
    }

    println!(
        "e5_ac_isolator: {} items, {} unresolved ours, {} with no AC type -> {}",
        manifest.items.len(),
        unresolved.len(),
        no_types.len(),
        output_path
    );
    if !unresolved.is_empty() {
        println!("unresolved: {unresolved:?}");
    }
    if !no_types.is_empty() {
        println!("no_types: {no_types:?}");
    }
    ExitCode::SUCCESS
}
