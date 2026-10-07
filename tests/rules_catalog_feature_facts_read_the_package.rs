//! SD-37 E4a.4: the Unchained Rogue / Summoner feature enums answer their per-variant facts
//! (`key`, `name`, `declaring_line`, `min_level`) from the data package, not from `match` arms
//! compiled into the catalog.
//!
//! The package is copied under a scratch repo root with one row of each feature table edited,
//! `CODEX_REPO_ROOT` points at that root, and the enum is asked for the edited facts. One test,
//! one process: the catalog caches each table for the life of the process, so the root is set
//! before anything reads a table.

use std::path::{Path, PathBuf};

use codex::rules_core::rules_catalog::pathfinder_unchained::rogue_features::UnchainedRogueFeature;
use codex::rules_core::rules_catalog::pathfinder_unchained::summoner_features::UnchainedSummonerFeature;

fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("create scratch dir");
    for entry in std::fs::read_dir(from).expect("read package dir").flatten() {
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).expect("copy package file");
        }
    }
}

fn plant(root: &Path, id: &str, feature: &str, edit: impl Fn(&mut serde_json::Value)) {
    let file = root.join(format!("data/rules_tables/{id}.json"));
    let text = std::fs::read_to_string(&file).expect("read the feature table");
    let mut package: serde_json::Value = serde_json::from_str(&text).expect("parse the feature table");
    let row = package["rows"]
        .as_array_mut()
        .expect("rows")
        .iter_mut()
        .find(|row| row["feature"] == feature)
        .unwrap_or_else(|| panic!("the {feature} row"));
    edit(row);
    std::fs::write(&file, serde_json::to_string_pretty(&package).expect("serialise")).expect("write the planted table");
}

#[test]
fn the_unchained_feature_facts_follow_the_package_they_are_pointed_at() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root: PathBuf = std::env::temp_dir().join(format!("codex-e4a4-plant-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    copy_tree(&repo.join("data/rules_tables"), &root.join("data/rules_tables"));

    plant(&root, "pathfinder_unchained/rogue_features/UnchainedRogueFeature", "Evasion", |row| {
        row["name"] = serde_json::json!("Planted Evasion");
        row["key"] = serde_json::json!("Unchained Rogue ~ Planted Evasion");
        row["declaring_line"] = serde_json::json!(9584);
        row["min_level"] = serde_json::json!(7);
    });
    plant(&root, "pathfinder_unchained/summoner_features/UnchainedSummonerFeature", "Eidolon", |row| {
        row["name"] = serde_json::json!("Planted Eidolon");
        row["min_level"] = serde_json::json!(9);
    });

    // SAFETY: the only test in this binary, set before any other thread reads the environment.
    unsafe { std::env::set_var("CODEX_REPO_ROOT", &root) };

    let evasion = UnchainedRogueFeature::Evasion;
    assert_eq!(evasion.name(), "Planted Evasion", "name answers from the package row");
    assert_eq!(evasion.key(), "Unchained Rogue ~ Planted Evasion", "key answers from the package row");
    assert_eq!(evasion.declaring_line(), 9584, "declaring_line answers from the package row");
    assert_eq!(evasion.min_level(), Some(7), "min_level answers from the package row");
    assert!(!evasion.is_granted_at(6) && evasion.is_granted_at(7), "is_granted_at follows min_level");
    // An unplanted variant still answers its own row.
    assert_eq!(UnchainedRogueFeature::SneakAttack.min_level(), Some(1));

    let eidolon = UnchainedSummonerFeature::Eidolon;
    assert_eq!(eidolon.name(), "Planted Eidolon", "name answers from the package row");
    assert_eq!(eidolon.min_level(), 9, "min_level answers from the package row");
    let _ = std::fs::remove_dir_all(&root);
}
