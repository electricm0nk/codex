//! GE08-E3-F1 headless validation and diagnostics gate.
//!
//! Proves the first bounded GE-08 validation/diagnostics layer makes the first proof
//! package honest by:
//! 1. refusing to preview packages with missing required structure
//! 2. refusing to preview packages with widened/unsupported semantics
//! 3. preserving machine-readable diagnostics and claim-blocking posture
//! 4. keeping valid packages preview-eligible after validation

use std::path::PathBuf;

use codex::homebrew_authoring::package_manifest::PackageValidationState;
use codex::homebrew_authoring::package_store::PackageStore;
use codex::homebrew_authoring::SourcePackage;

fn fixture_root(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(format!("tests/fixtures/ge08/{}", name))
}

#[test]
fn ge08_validation_valid_package_remains_preview_eligible() {
    let valid = SourcePackage::guard_stance_proof();
    assert_eq!(valid.manifest.validation_state, PackageValidationState::Valid);

    let (state, diagnostics) = valid.recompute_validation();
    assert_eq!(state, PackageValidationState::Valid);
    assert!(
        diagnostics.is_empty(),
        "valid package must have no diagnostics, but got: {:?}",
        diagnostics
    );
}

#[test]
fn ge08_validation_valid_package_from_fixture() {
    let fixture = PackageStore::load(&fixture_root("guard-stance-package"))
        .expect("valid fixture should load");
    assert_eq!(fixture.manifest.validation_state, PackageValidationState::Valid);

    let (state, diagnostics) = fixture.recompute_validation();
    assert_eq!(state, PackageValidationState::Valid);
    assert!(
        diagnostics.is_empty(),
        "loaded valid package must have no diagnostics, but got: {:?}",
        diagnostics
    );
}

#[test]
fn ge08_validation_missing_effect_blocks_preview_with_diagnostics() {
    let invalid_fixture = PackageStore::load(&fixture_root("guard-stance-package-invalid-missing-effect"))
        .expect("invalid missing-effect fixture should load");

    let (state, diagnostics) = invalid_fixture.recompute_validation();
    assert_eq!(
        state,
        PackageValidationState::Invalid,
        "package with missing effect must be Invalid"
    );

    assert!(
        !diagnostics.is_empty(),
        "package with missing effect must emit diagnostics"
    );

    let has_missing_effect_diagnostic = diagnostics.iter().any(|d| {
        d.subject_ref == "effect"
            && d.message.contains("missing")
            && d.claim_blocking
    });
    assert!(
        has_missing_effect_diagnostic,
        "must have claim-blocking diagnostic for missing effect. Diagnostics: {:?}",
        diagnostics
    );

    assert!(
        diagnostics.iter().any(|d| d.claim_blocking),
        "at least one diagnostic must be claim-blocking"
    );
}

#[test]
fn ge08_validation_widened_effect_target_blocks_preview_with_diagnostics() {
    let invalid_fixture = PackageStore::load(&fixture_root("guard-stance-package-invalid-widened-preview"))
        .expect("invalid widened-preview fixture should load");

    let (state, diagnostics) = invalid_fixture.recompute_validation();
    assert_eq!(
        state,
        PackageValidationState::Invalid,
        "package with widened effect target must be Invalid"
    );

    assert!(
        !diagnostics.is_empty(),
        "package with widened effect target must emit diagnostics"
    );

    let has_target_mismatch_diagnostic = diagnostics.iter().any(|d| {
        d.subject_ref == "effect.target_family"
            && d.message.contains("armor_class")
            && d.claim_blocking
    });
    assert!(
        has_target_mismatch_diagnostic,
        "must have claim-blocking diagnostic for widened effect target. Diagnostics: {:?}",
        diagnostics
    );

    assert!(
        diagnostics.iter().any(|d| d.claim_blocking),
        "at least one diagnostic must be claim-blocking"
    );
}
