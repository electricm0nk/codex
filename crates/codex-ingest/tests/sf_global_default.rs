//! SD-37 E7.1: Starfinder's global `Default` ability in the converted package.
//!
//! PCGen gives every Starfinder character the `Default` ability: the Strength row of the stat file
//! grants it unconditionally (`scr__stats.lst:4`, `ABILITY:Internal|AUTOMATIC|Default`), and every
//! book `.MOD`s its bookkeeping variables onto it (`scr_abilities.lst:5`, `:68` ...). One of them is
//! `EffectiveLVL` (`scr_abilities.lst:68`: `DEFINE:EffectiveLVL|0 BONUS:VAR|EffectiveLVL|TL|TYPE=Base`),
//! the character level every weapon specialization (`BONUS:VAR|Damage_<group>|EffectiveLVL`), the
//! Weapon Focus +2 test and the Resolve level term read. No population record stands for `Default`
//! (it is an internal helper), so before E7.1 its contributions were dropped and `EffectiveLVL` was
//! 0 on every character but a drone: the Soldier 3's longarm damage row printed +0 where the SRD and
//! the oracle say +3 (`progress.md` DISCOVERED, E5.MC -> E7.1).
//!
//! The converter now writes the global as one always-held record (`always_held.rs`): its variable
//! declarations and its unconditional `BONUS:VAR` contributions are the base state of every
//! evaluation, and nothing else of it is held or printed.
//!
//! These tests read the generated package (`data/starfinder-1e/sheet_rules`, kept fresh by
//! `sheet_rule_convert --system starfinder-1e --check`).

use std::path::{Path, PathBuf};

use serde_json::Value;

fn package_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/starfinder-1e/sheet_rules")
}

fn read(rel: &str) -> Value {
    let path = package_dir().join(rel);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    serde_json::from_str(&text).expect("parses")
}

/// The var table whose label is `label`.
fn var_labelled(label: &str) -> Value {
    let dir = package_dir().join("_vars");
    let mut found = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("_vars") {
        let v: Value = serde_json::from_str(&std::fs::read_to_string(entry.unwrap().path()).unwrap()).unwrap();
        if v["label"] == label {
            found.push(v);
        }
    }
    assert_eq!(found.len(), 1, "one var labelled {label:?}");
    found.remove(0)
}

#[test]
fn the_global_default_is_one_always_held_record_that_prints_nothing() {
    let rules = read("core/ability/default.json");
    let rules = rules.as_array().expect("an array of rules");
    assert_eq!(rules.len(), 1, "the principal only: {rules:?}");
    let r = &rules[0];
    assert_eq!(r["id"], "core:ability:default");
    assert_eq!(r["always_held"], true);
    assert_eq!(r["print"], false);
    assert!(r.get("target").is_none(), "folds no bonus of its own: {r}");
    let rows: Vec<&str> = r["provenance"]["closure_rows"].as_array().unwrap().iter().map(|c| c.as_str().unwrap()).collect();
    assert!(rows.contains(&"starfinder/paizo/core/scr_abilities.lst:5"), "{rows:?}");
    assert!(rows.contains(&"starfinder/paizo/core/scr_abilities.lst:68"), "{rows:?}");
}

#[test]
fn effective_level_is_the_character_level_for_every_character() {
    let v = var_labelled("Effective LVL");
    let from_default: Vec<&Value> =
        v["contributions"].as_array().unwrap().iter().filter(|c| c["rule_id"] == "core:ability:default").collect();
    assert_eq!(from_default.len(), 1, "{v}");
    assert_eq!(from_default[0]["when"], "Always", "{v}");
    assert!(v["declared_by"].as_array().unwrap().iter().any(|d| d == "core:ability:default"), "{v}");
    // The drone's own contribution (`scr_abilities.lst:1381`, `EffectiveLVL = DroneMasterLVL`) stays.
    assert!(v["contributions"].as_array().unwrap().iter().any(|c| c["rule_id"] == "core:ability:drone"), "{v}");
}
