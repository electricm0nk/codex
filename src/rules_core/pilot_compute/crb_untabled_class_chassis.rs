//! Base-attack-bonus / base-save chassis for Core Rulebook's **NPC and
//! `Ex-*` classes** (SD-34, `AT-34-E3-001`, mechanism
//! `class_absent_from_ClassId_ALL_and_book_class_id_enums`): `ClassId::ALL`
//! (`rules_tables::crb::class_tables`) carries only CRB's eleven `Base.PC`
//! classes -- Adept, Aristocrat, Commoner, Expert, Warrior (`TYPE:Base.NPC`)
//! and Ex-Barbarian, Ex-Paladin (`TYPE:Base.PC`, `VISIBLE:NO`) are ingested,
//! real corpus records with their own genuine `BONUS:COMBAT|BASEAB` /
//! `BONUS:SAVE` formulas, and no `compute_class_chassis` dispatch arm reads
//! them, so `modelled_class_books()` never learned their names and every
//! `Kind::Class` unit for them reported `engine-does-not-hold` with the
//! `class_absent_from_ClassId_ALL_and_book_class_id_enums` evidence
//! (`docs/release/SD-34-book-completion/decisions.md §14`) regardless of
//! what the engine could otherwise do with them.
//!
//! # Scope: these 7, not CRB's ten prestige classes
//!
//! CRB's ten `PC.Prestige` classes are already registered a different, real
//! way -- `prestige_class_entry_gate.rs`'s own corpus-derived registry
//! evaluates their genuine `PRE*` entry requirements, and that module's own
//! doc comment states plainly that a full base-attack-bonus/save chassis for
//! six of the ten is deferred pending a caster-level-stacking mechanism this
//! codebase does not have yet. This module does not reopen that deferral --
//! `modelled_class_books()` registers the ten prestige classes straight from
//! `prestige_class_entry_gate::prestige_class_entry_requirements()`, no new
//! chassis code, respecting the existing SD-32 decision. This module's own
//! job is strictly the seven classes nothing else names at all: five NPC
//! classes and two `Ex-*` variant states.
//!
//! # Method: the same general formula evaluator `generic_class_chassis.rs`
//! already proved against 61 conventional classes across 14 other books
//!
//! Every one of these seven classes' `BONUS:COMBAT|BASEAB` / `BONUS:SAVE`
//! tokens uses the exact same `classlevel("APPLIEDAS=NONEPIC")`-based
//! formula shape CRB's own eleven base classes use (confirmed by reading
//! `data/corpus/core_rulebook/class/{adept,aristocrat,commoner,expert,
//! warrior,ex_barbarian,ex_paladin}.json`'s own `raw_tokens` directly, not
//! assumed) -- so rather than re-deriving a hand-typed `BabProgression`/
//! good-saves classification a second time, this module reuses
//! `formula_interpreter::PcgenFormulaEvaluator` to evaluate each class's own
//! corpus-sourced formula string directly, the same approach
//! `generic_class_chassis.rs` already uses for classes whose formula shape
//! is not a plain `Full`/`ThreeQuarter`/`Half` progression. A second,
//! parallel module rather than widening `generic_class_chassis.rs`'s own
//! `CLASS_FAMILY_BOOKS`: that module's population (61 conventional PC
//! classes) is mirrored byte-for-byte in `apps/desktop/src-tauri`'s
//! `class_catalog_generic.rs` reference-library browser, and CRB's NPC/
//! `Ex-*` classes are not "conventional PC classes" in that browser's own
//! sense -- widening the shared book list here without updating that
//! separate crate's own mirror would silently desynchronize the two, which
//! that module's own doc comment says is exactly what its parallel-copy
//! design is meant to avoid.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde_json::Value;

use super::formula_interpreter::{extract_formula_field, PcgenFormulaEvaluator};
use super::formula_reproduction_harness::FormulaEvaluator as _;

/// The seven CRB classes this module covers, by their `"class:<slug>"` id
/// convention -- five `TYPE:Base.NPC` classes and two `TYPE:Base.PC,
/// VISIBLE:NO` `Ex-*` variant states. Fixed, not derived from a directory
/// walk over all 28 CRB class records: CRB's eleven real base classes
/// (`ClassId::ALL`) and ten prestige classes (`prestige_class_entry_gate`)
/// are each registered their own way, and this module must never
/// double-register or shadow either.
const COVERED_SLUGS: [&str; 7] =
    ["adept", "aristocrat", "commoner", "expert", "warrior", "ex_barbarian", "ex_paladin"];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
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

/// Identical in shape to `generic_class_chassis::select_baseab_formula`.
fn select_baseab_formula(tokens: &[(String, String)]) -> Option<String> {
    let candidates: Vec<&str> =
        tokens.iter().filter(|(k, v)| k == "BONUS" && v.contains("BASEAB")).map(|(_, v)| v.as_str()).collect();
    if candidates.len() == 1 {
        return extract_formula_field("BONUS", candidates[0]).map(str::to_string);
    }
    candidates
        .into_iter()
        .find(|v| v.trim_end().ends_with(",0"))
        .and_then(|v| extract_formula_field("BONUS", v))
        .map(str::to_string)
}

/// Identical in shape to `generic_class_chassis::select_save_formulas`.
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

fn max_level_for(tokens: &[(String, String)]) -> u8 {
    tokens.iter().find(|(k, _)| k == "MAXLEVEL").and_then(|(_, v)| v.parse::<u8>().ok()).unwrap_or(20)
}

struct ClassRecord {
    display_name: String,
    max_level: u8,
    baseab_formula: String,
    fort_formula: String,
    ref_formula: String,
    will_formula: String,
}

pub struct CrbUntabledClassChassisRow {
    pub display_name: String,
    pub base_attack_bonus: i16,
    pub fort_save: i16,
    pub ref_save: i16,
    pub will_save: i16,
}

/// One covered class's registration: its `"class:<slug>"` id and the book
/// it belongs to (always `"core_rulebook"` for this module). Exposed so
/// `modelled_class_books()` (`src/bin/v06_work_inventory.rs`) can register
/// these seven names as classes the engine models, the same shape it
/// already uses for `untabled_base_class_chassis::untabled_base_class_
/// registry()`.
pub struct CrbUntabledClassMeta {
    pub class_id: String,
    pub display_name: String,
}

fn load_records() -> Vec<(&'static str, ClassRecord)> {
    let repo_root = repo_root();
    let dir: PathBuf = repo_root.join("data/corpus/core_rulebook/class");
    let mut out = Vec::new();
    for slug in COVERED_SLUGS {
        let path: PathBuf = Path::new(&dir).join(format!("{slug}.json"));
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
        let data = &doc["data"];
        let Some(name) = data["name"].as_str() else { continue };
        let tokens = tokens_from(data);
        let max_level = max_level_for(&tokens);
        let Some(baseab_formula) = select_baseab_formula(&tokens) else { continue };
        let [fort_f, ref_f, will_f] = select_save_formulas(&tokens);
        let (Some(fort_formula), Some(ref_formula), Some(will_formula)) = (fort_f, ref_f, will_f) else {
            continue;
        };
        out.push((
            slug,
            ClassRecord {
                display_name: name.to_string(),
                max_level,
                baseab_formula,
                fort_formula,
                ref_formula,
                will_formula,
            },
        ));
    }
    out
}

fn records() -> &'static [(&'static str, ClassRecord)] {
    static TABLE: OnceLock<Vec<(&'static str, ClassRecord)>> = OnceLock::new();
    TABLE.get_or_init(load_records).as_slice()
}

/// The full registration list -- every covered class this module's own
/// corpus read actually resolved a record for (never assumed to be all
/// seven; a JSON file that failed to parse or carried no BASEAB/SAVE
/// formula is honestly absent, not silently substituted).
pub fn covered_classes() -> Vec<CrbUntabledClassMeta> {
    records()
        .iter()
        .map(|(slug, record)| CrbUntabledClassMeta {
            class_id: format!("class:{slug}"),
            display_name: record.display_name.clone(),
        })
        .collect()
}

fn find_by_class_id(class_id_str: &str) -> Option<&'static ClassRecord> {
    let bare = class_id_str.strip_prefix("class:").unwrap_or(class_id_str);
    records().iter().find(|(slug, _)| *slug == bare).map(|(_, record)| record)
}

/// Resolves `class_id_str` at `level` into a real base-attack-bonus/save
/// chassis row, evaluating this class's own corpus formula strings via
/// `PcgenFormulaEvaluator` -- the same evaluator, the same
/// `classlevel("APPLIEDAS=NONEPIC")` binding shape, `generic_class_chassis`
/// already proved against 61 other classes. `None` when `class_id_str`
/// names no class this module covers, or `level` exceeds the class's own
/// corpus `MAXLEVEL` ceiling.
pub fn resolve(class_id_str: &str, level: u8) -> Option<CrbUntabledClassChassisRow> {
    let record = find_by_class_id(class_id_str)?;
    if level < 1 || level > record.max_level {
        return None;
    }
    let evaluator = PcgenFormulaEvaluator;
    let mut vars = std::collections::BTreeMap::new();
    vars.insert("CLASSLEVEL::APPLIEDAS=NONEPIC".to_string(), i64::from(level));
    let bind = |f: &str| evaluator.evaluate(f, &vars).ok().map(|v| v as i16);
    Some(CrbUntabledClassChassisRow {
        display_name: record.display_name.clone(),
        base_attack_bonus: bind(&record.baseab_formula)?,
        fort_save: bind(&record.fort_formula)?,
        ref_save: bind(&record.ref_formula)?,
        will_save: bind(&record.will_formula)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_seven_covered_classes_resolve_a_real_chassis_at_level_one() {
        let covered = covered_classes();
        assert_eq!(
            covered.len(),
            7,
            "every one of the seven corpus class records must parse and carry BASEAB+SAVE formulas"
        );
        for meta in &covered {
            let row = resolve(&meta.class_id, 1);
            assert!(row.is_some(), "{} must resolve a real chassis at level 1", meta.class_id);
        }
    }

    #[test]
    fn warrior_full_bab_matches_the_corpus_classlevel_formula_at_level_ten() {
        // `CLASS:Warrior ... BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")` --
        // full BAB, so level 10 must resolve to base attack bonus 10.
        let row = resolve("class:warrior", 10).expect("warrior must resolve");
        assert_eq!(row.base_attack_bonus, 10);
        // `BONUS:SAVE|BASE.Fortitude|classlevel(...)/2+2` (good) -> 10/2+2 = 7.
        assert_eq!(row.fort_save, 7);
        // `BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel(...)/3` (poor) -> 10/3 = 3.
        assert_eq!(row.ref_save, 3);
        assert_eq!(row.will_save, 3);
    }

    #[test]
    fn commoner_half_bab_and_all_poor_saves_match_the_corpus_formula() {
        // `CLASS:Commoner ... BONUS:COMBAT|BASEAB|classlevel(...)/2` -> half BAB.
        let row = resolve("class:commoner", 9).expect("commoner must resolve");
        assert_eq!(row.base_attack_bonus, 4); // 9/2 = 4 (integer division)
        assert_eq!(row.fort_save, 3); // 9/3 = 3, all three saves poor
        assert_eq!(row.ref_save, 3);
        assert_eq!(row.will_save, 3);
    }

    #[test]
    fn a_level_beyond_max_level_resolves_nothing() {
        assert!(resolve("class:warrior", 21).is_none(), "MAXLEVEL:20 must cap resolution");
    }

    #[test]
    fn an_uncovered_class_id_resolves_nothing() {
        assert!(
            resolve("class:arcane_archer", 1).is_none(),
            "prestige classes are registered by prestige_class_entry_gate, never this module"
        );
        assert!(
            resolve("class:fighter", 1).is_none(),
            "CRB's real base classes are registered by ClassId::ALL, never this module"
        );
    }
}
