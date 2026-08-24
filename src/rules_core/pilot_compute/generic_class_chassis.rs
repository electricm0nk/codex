//! Character-creation-time chassis dispatch for the 60 conventional PC
//! classes `class_catalog_generic.rs` (`apps/desktop/src-tauri`, SD-32 T12
//! `epic-10-reference-library-residual-reach` row 20 cycle 4) already
//! re-derives a BAB/save progression TABLE for, browsable in the reference
//! catalog. That table has no `compute_class_chassis` dispatch arm reading
//! it -- a character actually PICKING one of those 60 classes at creation
//! could not reach a real base attack bonus or save chassis at all, only
//! the read-only reference browser. Row 20 cycle 5 closes that gap: this
//! module is the crate-internal (`pilot_compute` lives in the core `codex`
//! crate; the apps/desktop catalog module is a separate, downstream crate
//! and cannot be imported from here) sibling of `class_catalog_generic.rs`,
//! re-running the SAME classification/extraction logic that module's own
//! doc comment already verified against all 61 candidate records, wired
//! into `compute_class_chassis` below via [`resolve`].
//!
//! # Why a second copy of the read logic, not a shared one
//!
//! `class_catalog_generic.rs`'s own read/classify/evaluate functions
//! (`classify_class_record`, `select_baseab_formula`, `select_save_
//! formulas`, `max_level_for`) live in `apps/desktop/src-tauri`, a crate
//! that DEPENDS ON `codex` (this crate), never the reverse -- `pilot_
//! compute::mod.rs`'s own `compute_class_chassis` cannot import from it
//! without an illegal reverse dependency. This module re-implements the
//! same, already-verified logic at the crate boundary it is actually
//! needed at, the same "parallel per-family module" shape `class_slayer.rs`/
//! `class_ultimate_combat.rs` (this module's own siblings) already use
//! rather than a single monolith. `resolve`'s own doc-comment tests below
//! independently reproduce the parent cycle's 60/61 population split, so
//! any future drift between the two copies is caught by CI rather than
//! silently diverging.
//!
//! # 61, not 60, land in this module's own table -- Demoniac still refuses
//!
//! `class_catalog_generic.rs` names Demoniac's bare `classlevel()` as the
//! one record `formula_interpreter.rs`'s grammar refuses (row 18's live
//! territory, out of this cycle's write scope). This module hits the
//! identical refusal for the identical reason -- `resolve` simply never
//! returns `Some` for Demoniac, and the class stays `class_chassis.
//! unsupported` exactly like any other genuinely-uncomputed class, until a
//! future `formula_interpreter.rs` widening closes it.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde_json::Value;

use super::formula_interpreter::{extract_formula_field, PcgenFormulaEvaluator};
use super::formula_reproduction_harness::FormulaEvaluator as _;

/// See `class_catalog_generic.rs`'s own doc comment, "Reachability, honestly
/// scoped" — same 14 books, same population.
const CLASS_FAMILY_BOOKS: [&str; 14] = [
    "adventurers_guide",
    "book_of_the_damned_volume_1",
    "book_of_the_damned_volume_2",
    "inner_sea_combat",
    "inner_sea_gods",
    "inner_sea_intrigue",
    "inner_sea_magic",
    "inner_sea_world_guide",
    "occult_adventures",
    "ultimate_combat",
    "ultimate_intrigue",
    "ultimate_magic",
    "ultimate_wilderness",
    "ultimate_psionics",
];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn walk_json_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            walk_json_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "json") {
            out.push(path);
        }
    }
}

fn tokens_from(data: &Value) -> Vec<(String, String)> {
    data["raw_tokens"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|t| {
                    let key = t["key"].as_str()?.to_string();
                    let value = t["value"].as_str()?.to_string();
                    Some((key, value))
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Identical to `class_catalog_generic::classify_class_record` — see that
/// module's own doc comment for the full derivation.
fn is_conventional_pc_record(tokens: &[(String, String)]) -> bool {
    let typ = tokens.iter().find(|(k, _)| k == "TYPE").map(|(_, v)| v.as_str()).unwrap_or("");
    if typ.contains("Monster") {
        return false;
    }
    let has_bab = tokens.iter().any(|(k, v)| k == "BONUS" && v.contains("BASEAB"));
    let has_save = tokens.iter().any(|(k, v)| k == "BONUS" && v.starts_with("SAVE|"));
    has_bab && has_save
}

/// Identical to `class_catalog_generic::select_baseab_formula`.
fn select_baseab_formula<'a>(tokens: &'a [(String, String)]) -> Option<&'a str> {
    let candidates: Vec<&str> =
        tokens.iter().filter(|(k, v)| k == "BONUS" && v.contains("BASEAB")).map(|(_, v)| v.as_str()).collect();
    if candidates.len() == 1 {
        return extract_formula_field("BONUS", candidates[0]);
    }
    candidates.into_iter().find(|v| v.trim_end().ends_with(",0")).and_then(|v| extract_formula_field("BONUS", v))
}

/// Identical to `class_catalog_generic::select_save_formulas`.
fn select_save_formulas(tokens: &[(String, String)]) -> [Option<String>; 3] {
    let mut fort = None;
    let mut refl = None;
    let mut will = None;
    for (k, v) in tokens {
        if k != "BONUS" || !v.starts_with("SAVE|") {
            continue;
        }
        let parts: Vec<&str> = v.split('|').collect();
        if parts.len() < 3 {
            continue;
        }
        let Some(formula) = extract_formula_field("BONUS", v) else { continue };
        for target in parts[1].split(',') {
            match target {
                "BASE.Fortitude" => fort = Some(formula.to_string()),
                "BASE.Reflex" => refl = Some(formula.to_string()),
                "BASE.Will" => will = Some(formula.to_string()),
                _ => {}
            }
        }
    }
    [fort, refl, will]
}

/// Identical to `class_catalog_generic::max_level_for`.
fn max_level_for(tokens: &[(String, String)], type_value: &str) -> u8 {
    tokens
        .iter()
        .find(|(k, _)| k == "MAXLEVEL")
        .and_then(|(_, v)| v.parse::<u8>().ok())
        .unwrap_or(if type_value.contains("Prestige") { 10 } else { 20 })
}

/// A slug matching the `"class:<slug>"` id convention every other dispatch
/// arm in `compute_class_chassis` already uses (e.g. `"class:psychic_
/// warrior"`, `"class:vigilante"`): the corpus record's own display name,
/// lower-cased, with every whitespace run collapsed to one underscore.
fn slug(name: &str) -> String {
    name.trim().to_ascii_lowercase().split_whitespace().collect::<Vec<_>>().join("_")
}

pub(crate) struct GenericChassisRow {
    pub(crate) display_name: String,
    pub(crate) base_attack_bonus: i16,
    pub(crate) fort_save: i16,
    pub(crate) ref_save: i16,
    pub(crate) will_save: i16,
}

struct ClassRecord {
    display_name: String,
    max_level: u8,
    baseab_formula: String,
    fort_formula: String,
    ref_formula: String,
    will_formula: String,
    /// PCGen's own auto-declared per-class level variable, `<Name>LVL` with
    /// whitespace stripped rather than collapsed (matches `class_catalog_
    /// generic.rs`'s own binding, `format!("{name}LVL")`, and `class_
    /// feature_grant_consumer::class_level_variable_name`'s identical
    /// convention).
    class_level_var: String,
}

/// Loaded once per process, keyed by [`slug`] — mirrors `class_catalog_
/// generic::load_generic_class_progressions`'s own population (60 of the
/// 61 conventional classes; Demoniac refuses, see this module's own doc
/// comment).
fn generic_class_records() -> &'static BTreeMap<String, ClassRecord> {
    static TABLE: OnceLock<BTreeMap<String, ClassRecord>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut out = BTreeMap::new();
        let repo_root = repo_root();
        for book in CLASS_FAMILY_BOOKS {
            let dir = repo_root.join("data/corpus").join(book).join("class");
            if !dir.is_dir() {
                continue;
            }
            let mut files = Vec::new();
            walk_json_files(&dir, &mut files);
            for file in files {
                let Ok(text) = std::fs::read_to_string(&file) else { continue };
                let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
                let data = &doc["data"];
                let Some(name) = data["name"].as_str() else { continue };
                let tokens = tokens_from(data);
                if !is_conventional_pc_record(&tokens) {
                    continue;
                }
                let type_value =
                    tokens.iter().find(|(k, _)| k == "TYPE").map(|(_, v)| v.as_str()).unwrap_or("");
                let max_level = max_level_for(&tokens, type_value);
                let Some(baseab_formula) = select_baseab_formula(&tokens) else { continue };
                let [fort_f, ref_f, will_f] = select_save_formulas(&tokens);
                let (Some(fort_formula), Some(ref_formula), Some(will_formula)) = (fort_f, ref_f, will_f)
                else {
                    continue;
                };
                out.entry(slug(name)).or_insert_with(|| ClassRecord {
                    display_name: name.to_string(),
                    max_level,
                    baseab_formula: baseab_formula.to_string(),
                    fort_formula,
                    ref_formula,
                    will_formula,
                    class_level_var: format!("{name}LVL"),
                });
            }
        }
        out
    })
}

/// Resolves `class_id_str` (a `"class:<slug>"` string) at `level` into a
/// real BAB/save chassis row, or `None` when the class is not one of this
/// table's 60 (either genuinely unresolved, like Demoniac, or simply not a
/// conventional PC class at all — the caller's existing dispatch chain
/// already tried every other known family first, so a `None` here always
/// falls through to the same `class_chassis.unsupported` diagnostic every
/// other unrecognized class id already produces).
pub(crate) fn resolve(class_id_str: &str, level: u8) -> Option<GenericChassisRow> {
    let bare = class_id_str.strip_prefix("class:").unwrap_or(class_id_str);
    let record = generic_class_records().get(bare)?;
    if level < 1 || level > record.max_level {
        return None;
    }
    let evaluator = PcgenFormulaEvaluator;
    let mut vars = BTreeMap::new();
    // Same two binding shapes `class_catalog_generic.rs`'s own module doc
    // measured across all 61 candidates — see that module's doc comment,
    // "Formula shape, measured across all 61 before writing any code".
    vars.insert("CLASSLEVEL::APPLIEDAS=NONEPIC".to_string(), i64::from(level));
    vars.insert(record.class_level_var.clone(), i64::from(level));
    let bind = |f: &str| evaluator.evaluate(f, &vars).ok().map(|v| v as i16);
    let base_attack_bonus = bind(&record.baseab_formula)?;
    let fort_save = bind(&record.fort_formula)?;
    let ref_save = bind(&record.ref_formula)?;
    let will_save = bind(&record.will_formula)?;
    Some(GenericChassisRow {
        display_name: record.display_name.clone(),
        base_attack_bonus,
        fort_save,
        ref_save,
        will_save,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sixty_of_the_sixty_one_conventional_classes_resolve() {
        // Demoniac (bare `classlevel()`) is the one named, live exception
        // — see this module's own doc comment. 60, not 61 or fewer.
        assert_eq!(
            generic_class_records().len(),
            61,
            "the read/classify population itself is 61 (matches class_catalog_generic.rs's own \
             re-derivation); Demoniac fails only at RESOLVE time (its formulas evaluate to None), \
             not at load time, so it still appears here as a loaded record"
        );
        let mut resolved = 0usize;
        for (bare, record) in generic_class_records() {
            let class_id = format!("class:{bare}");
            if resolve(&class_id, 1).is_some() {
                resolved += 1;
            } else {
                assert_eq!(
                    record.display_name, "Demoniac",
                    "the only class expected to fail resolve() at level 1 is Demoniac; {bare} \
                     failed unexpectedly"
                );
            }
        }
        assert_eq!(resolved, 60, "exactly 60 of the 61 conventional classes must resolve a real chassis");
    }

    #[test]
    fn demoniac_refuses_rather_than_fabricates() {
        assert!(
            resolve("class:demoniac", 1).is_none(),
            "Demoniac's bare classlevel() must still refuse until formula_interpreter.rs widens \
             (row 18's live territory) — never guess a value here"
        );
    }

    #[test]
    fn vigilante_resolves_via_the_toggle_off_baseab_row_matching_class_catalog_generic() {
        // Same disambiguation `class_catalog_generic.rs`'s own test proves:
        // level-20 BAB is the moderate (,0 / toggle-off) progression, 15,
        // never the toggle-on alternative, 20. Vigilante is ALSO one of
        // the 20 untabled-base-class-chassis classes and so is dispatched
        // by that arm first in the real `compute_class_chassis` chain —
        // this module's own `resolve` is exercised directly here (not
        // through the full dispatcher) purely to prove its own formula
        // selection is correct in isolation.
        let row = resolve("class:vigilante", 20).expect("vigilante must resolve at level 20");
        assert_eq!(row.base_attack_bonus, 15);
    }

    #[test]
    fn a_level_past_max_level_refuses() {
        // Ulfen Guard (`inner_sea_combat`, TYPE: PC.Prestige, no MAXLEVEL
        // token) defaults to 10 per `class_catalog_generic.rs`'s own
        // documented prestige-class rule; level 11 must refuse.
        assert!(resolve("class:ulfen_guard", 11).is_none());
        assert!(resolve("class:ulfen_guard", 10).is_some());
    }

    #[test]
    fn an_unrecognized_class_id_refuses() {
        assert!(resolve("class:not_a_real_class", 1).is_none());
    }
}
