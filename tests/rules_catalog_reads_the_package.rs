//! SD-37 E4a.2: an importer's answer follows the data package, not a compiled copy.
//!
//! The package is copied under a scratch repo root with ONE row edited (Fighter's class-skill
//! list names Appraise where the compiled table names Handle Animal), `CODEX_REPO_ROOT` points
//! at that root, and the importer `skill_allocation::is_class_skill_for` is asked about both
//! skills. If any step between the importer and the table still read the compiled table, the
//! answer would be the compiled one.
//!
//! One test, one process: the catalog caches each table for the life of the process, so the
//! root is set before anything reads a table.

use std::path::{Path, PathBuf};

use codex::rules_core::skill_allocation::is_class_skill_for;

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

fn scratch_root() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("codex-e4a2-plant-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

#[test]
fn an_importer_answers_from_the_package_it_is_pointed_at() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root = scratch_root();
    copy_tree(&repo.join("data/rules_tables"), &root.join("data/rules_tables"));

    let file = root.join("data/rules_tables/crb/class_skill_tables/CLASS_SKILL_LISTS.json");
    let text = std::fs::read_to_string(&file).expect("read the class-skill table");
    let mut package: serde_json::Value = serde_json::from_str(&text).expect("parse the class-skill table");
    let fighter = package["rows"]
        .as_array_mut()
        .expect("rows")
        .iter_mut()
        .find(|row| row["owner_id"] == "class:fighter")
        .expect("the Fighter row");
    let skills = fighter["skills"].as_array_mut().expect("skills");
    let handle_animal = skills.iter_mut().find(|skill| skill["Named"] == "Handle Animal").expect("Handle Animal");
    *handle_animal = serde_json::json!({ "Named": "Appraise" });
    std::fs::write(&file, serde_json::to_string_pretty(&package).expect("serialise")).expect("write the planted table");

    // SAFETY: the only test in this binary, set before any other thread reads the environment.
    unsafe { std::env::set_var("CODEX_REPO_ROOT", &root) };

    assert!(
        is_class_skill_for("class:fighter", "skill:appraise"),
        "the planted package row names Appraise as a Fighter class skill"
    );
    assert!(
        !is_class_skill_for("class:fighter", "skill:handle_animal"),
        "the planted package row no longer names Handle Animal; the compiled table still does"
    );
    let _ = std::fs::remove_dir_all(&root);
}
