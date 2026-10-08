//! A packaged app has only what `tauri.conf.json` bundles, laid out under its resource directory.
//! This test builds exactly that layout (symlinks to each bundled source, nothing else), points the
//! rules crate's data root at it, and asks the engine for the things the shipped app asks for.
//!
//! It exists because the class roster failed in every installed build with
//! `could not read /home/runner/work/codex/codex/tests/fixtures/...`: the engine read test fixtures
//! and data directories from a path baked in at compile time, and nothing checked that the bundle
//! carried what the engine reads. Test builds always found the checkout, so no test could see it.
//!
//! One process, one data root: `set_data_root` is process-wide and first-call-wins, so this file
//! must keep all of its checks in a single `#[test]`.

use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

use codex::rules_core::class_census::{
    class_creation_roster, load_mix_panel, load_sweep_fixture, MIX_PANEL_RELATIVE_PATH,
};
use codex::rules_core::class_seeds::FIXTURE_RELATIVE_PATH;
use codex::rules_core::equipment_types::{equipment_types, EQUIPMENT_TYPES_PATH};
use codex::rules_core::record_vars::RECORD_VARS_PATH;

fn src_tauri() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Lay out the bundle the way Tauri does: each `resources` entry's source appears at its
/// destination under the resource root.
fn packaged_root() -> PathBuf {
    let conf: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(src_tauri().join("tauri.conf.json")).unwrap()).unwrap();
    let resources = conf["bundle"]["resources"].as_object().expect("bundle.resources is a map");

    let root = std::env::temp_dir().join(format!("codex-packaged-root-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();

    for (source, dest) in resources {
        let source = fs::canonicalize(src_tauri().join(source))
            .unwrap_or_else(|e| panic!("bundled resource {source} does not exist: {e}"));
        let dest = root.join(dest.as_str().unwrap().trim_end_matches('/'));
        // Entries may nest (`corpus_fixtures/` and `corpus_fixtures/spell/`): the child is already
        // reachable through the parent's link. Accept that only when it is the same source.
        if dest.exists() {
            assert_eq!(
                fs::canonicalize(&dest).unwrap(),
                source,
                "two bundle entries disagree about what lives at {}",
                dest.display()
            );
            continue;
        }
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        symlink(&source, &dest).unwrap_or_else(|e| panic!("cannot lay out {}: {e}", dest.display()));
    }
    root
}

fn assert_bundled(root: &Path, relative: &str) {
    assert!(
        root.join(relative).exists(),
        "the engine reads `{relative}` at runtime but tauri.conf.json does not bundle it"
    );
}

#[test]
fn the_packaged_bundle_carries_everything_the_rules_engine_reads() {
    let root = packaged_root();
    assert!(codex::set_data_root(root.clone()), "data root must install on first call");

    // Every runtime path the engine joins onto its data root.
    for relative in [
        FIXTURE_RELATIVE_PATH,
        MIX_PANEL_RELATIVE_PATH,
        RECORD_VARS_PATH,
        EQUIPMENT_TYPES_PATH,
        "data/sheet_rules",
        "data/starfinder-1e/sheet_rules",
        "data/rules_tables",
        "data/class_feature_grants",
        "data/corpus",
    ] {
        assert_bundled(&root, relative);
    }

    // And the user-visible behaviour: the roster the Create dialog shows.
    load_sweep_fixture().expect("the shared sweep fixture loads from the packaged root");
    load_mix_panel().expect("the multiclass mix panel loads from the packaged root");
    let roster = class_creation_roster().expect("the class roster builds from the packaged root");
    assert!(
        roster.len() > 31,
        "roster has {} classes: the app would offer only the built-in 31-class fallback",
        roster.len()
    );

    assert!(
        equipment_types().is_ok(),
        "the equipment type sidecar does not load from the packaged root: {:?}",
        equipment_types().as_ref().err()
    );

    assert!(
        codex::rules_core::corpus_loader::live_sheet_rules().is_some(),
        "live_sheet_rules found no rules under the packaged root (it returns None silently)"
    );
    assert!(
        codex::rules_core::sheet_rule_package::package().is_ok(),
        "the sheet-rule package does not load from the packaged root"
    );

    // Every game system's package and the rules-table package resolve under the installed root,
    // not the compile-time checkout (the per-system roots read `runtime_repo_root`).
    let installed = codex::rules_core::game_system::runtime_repo_root();
    assert_eq!(installed, root, "the per-system package root ignores the installed data root");
    assert!(
        codex::rules_core::corpus_loader::live_sheet_rules_for(codex::rules_core::game_system::GameSystem::Starfinder1e)
            .is_some(),
        "the Starfinder sheet-rule package does not load from the packaged root"
    );
    assert_eq!(
        codex::rules_core::rules_data_package::runtime_package_root(),
        root.join("data/rules_tables"),
        "the rules-table package is not read from the packaged root"
    );
}
