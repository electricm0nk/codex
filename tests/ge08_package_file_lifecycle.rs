//! GE08-E2-F1 headless package lifecycle gate.
//!
//! Proves the first bounded GE-08 package-file lifecycle can create a draft shell,
//! save deterministic authored source, reload the first proof package from disk,
//! produce a stable authored-source diff, and refuse export when the package is
//! not yet valid.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use codex::homebrew_authoring::package_manifest::PackageValidationState;
use codex::homebrew_authoring::package_store::PackageStore;
use codex::homebrew_authoring::SourcePackage;

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ge08/guard-stance-package")
}

fn fresh_temp_dir(label: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("codex-{label}-{}-{unique}", std::process::id()));
    fs::create_dir_all(&path).expect("temp dir should be creatable");
    path
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
}

#[test]
fn ge08_package_file_lifecycle_create_save_load_diff_and_export_gate() {
    let shell = SourcePackage::guard_stance_shell();
    assert_eq!(shell.manifest.validation_state, PackageValidationState::Draft);

    let shell_root = fresh_temp_dir("ge08-shell-save");
    PackageStore::save(&shell, &shell_root).expect("draft shell should save");

    let manifest = read(&shell_root.join("manifest.yaml"));
    assert!(manifest.contains("validation_state: draft"));
    assert!(
        PackageStore::export(&shell, &fresh_temp_dir("ge08-shell-export")).is_err(),
        "draft shell must not be exportable"
    );

    let proof = SourcePackage::guard_stance_proof();
    assert_eq!(proof.manifest.validation_state, PackageValidationState::Valid);

    let fixture = PackageStore::load(&fixture_root()).expect("fixture package should load");
    assert_eq!(PackageStore::diff(&proof, &fixture).changed_files, Vec::<String>::new());

    let mut edited = proof.clone();
    edited.manifest.package_version = "0.1.1".to_owned();
    let diff = PackageStore::diff(&proof, &edited);
    assert_eq!(diff.changed_files, vec!["manifest.yaml".to_owned()]);

    let export_root = fresh_temp_dir("ge08-proof-export");
    PackageStore::export(&proof, &export_root).expect("valid proof package should export");
    let exported = PackageStore::load(&export_root).expect("exported package should reload");
    assert_eq!(PackageStore::diff(&proof, &exported).changed_files, Vec::<String>::new());
}

#[test]
fn ge08_guard_stance_package_round_trip() {
    let fixture = PackageStore::load(&fixture_root()).expect("fixture package should load");

    let round_trip_root = fresh_temp_dir("ge08-round-trip");
    PackageStore::save(&fixture, &round_trip_root).expect("fixture package should save");

    let reloaded = PackageStore::load(&round_trip_root).expect("saved package should reload");
    assert_eq!(reloaded, fixture);

    let diff = PackageStore::diff_roots(&fixture_root(), &round_trip_root)
        .expect("saved package should produce the same authored-source surface");
    assert!(
        diff.changed_files.is_empty(),
        "round-trip should not rewrite authored source: {:?}",
        diff.changed_files
    );
}

#[test]
fn ge08_save_rejects_values_the_loader_cannot_read_back() {
    // save() deliberately persists Draft/Invalid packages, so every rendered
    // `key: value` line must stay loadable. An empty or multi-line value renders
    // a line the parser rejects, leaving an unloadable bundle on disk with no
    // warning at write time.
    let root = fresh_temp_dir("ge08-save-honesty");

    // Empty scalar value: renders "provenance_policy: " which the loader
    // refuses as "unsupported manifest line".
    let mut package = SourcePackage::guard_stance_proof();
    package.manifest.provenance_policy = String::new();
    let err = PackageStore::save(&package, &root)
        .expect_err("an empty provenance_policy must be rejected at save time");
    assert!(
        err.message.contains("provenance_policy"),
        "error must name the offending field: {}",
        err.message
    );

    // Multi-line value: the second line is garbage to the record parser.
    let mut package = SourcePackage::guard_stance_proof();
    if let Some(feat) = package.feat.as_mut() {
        feat.display_name = "Guard\nStance".to_owned();
    }
    let err = PackageStore::save(&package, &root)
        .expect_err("a multi-line display_name must be rejected at save time");
    assert!(
        err.message.contains("display_name"),
        "error must name the offending field: {}",
        err.message
    );

    fs::remove_dir_all(&root).ok();
}

#[test]
fn ge08_invalid_validation_state_is_reported_as_invalid_not_missing() {
    let root = fresh_temp_dir("ge08-invalid-validation-state");
    PackageStore::save(&SourcePackage::guard_stance_proof(), &root)
        .expect("proof package should save");

    let manifest_path = root.join("manifest.yaml");
    let corrupted = read(&manifest_path).replace("validation_state: valid", "validation_state: bogus");
    fs::write(&manifest_path, corrupted).expect("manifest should be writable");

    let err = PackageStore::load(&root)
        .expect_err("an unrecognized validation_state must fail to load");
    assert!(
        err.message.contains("bogus"),
        "the error must name the invalid value, not claim the field is missing: {}",
        err.message
    );

    fs::remove_dir_all(&root).ok();
}
