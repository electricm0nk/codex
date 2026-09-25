//! SD-36 Epic E desktop-P1-02 (SD-34 R14-02): `PackageStore::save` writes every file in its
//! `file_map` loop with a temp-then-rename, so a crash/failure partway through the loop does
//! not leave a truncated or half-written file at any already-written path.

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use codex::homebrew_authoring::SourcePackage;
use codex::homebrew_authoring::package_store::PackageStore;

fn fresh_temp_dir(label: &str) -> PathBuf {
    let unique = SystemTime::now().duration_since(UNIX_EPOCH).expect("system time should be after unix epoch").as_nanos();
    let path = std::env::temp_dir().join(format!("codex-{label}-{}-{unique}", std::process::id()));
    fs::create_dir_all(&path).expect("temp dir should be creatable");
    path
}

fn collect_file_names(dir: &std::path::Path, out: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_file_names(&path, out);
        } else if let Some(name) = path.file_name() {
            out.push(name.to_string_lossy().into_owned());
        }
    }
}

#[test]
fn a_successful_save_leaves_no_tmp_files_behind() {
    let root = fresh_temp_dir("package-store-no-tmp-leftovers");
    PackageStore::save(&SourcePackage::guard_stance_proof(), &root).expect("save should succeed");

    let mut names = Vec::new();
    collect_file_names(&root, &mut names);
    assert!(!names.is_empty(), "the save must have written real files");
    assert!(!names.iter().any(|n| n.ends_with(".tmp")), "no .tmp file should remain after a successful save: {names:?}");
}

#[test]
fn a_write_that_fails_partway_through_does_not_touch_the_previous_manifest() {
    let root = fresh_temp_dir("package-store-partial-failure");
    PackageStore::save(&SourcePackage::guard_stance_proof(), &root).expect("the first save should succeed");
    let previous_manifest = fs::read_to_string(root.join("manifest.yaml")).expect("manifest.yaml exists after the first save");

    // Sabotage manifest.yaml's OWN temp path so its atomic_write fails outright before any
    // rename happens -- a directory can't be written over as a file.
    let sabotage_path = root.join("manifest.yaml.tmp");
    fs::create_dir_all(&sabotage_path).expect("sabotage directory should be creatable");

    let result = PackageStore::save(&SourcePackage::guard_stance_shell(), &root);
    assert!(result.is_err(), "the sabotaged save must report failure, not silently half-succeed");

    fs::remove_dir_all(&sabotage_path).ok();
    let after_failure = fs::read_to_string(root.join("manifest.yaml")).expect("manifest.yaml must still exist after the failed save");
    assert_eq!(after_failure, previous_manifest, "a failed save must not touch the previously-saved manifest");
}
