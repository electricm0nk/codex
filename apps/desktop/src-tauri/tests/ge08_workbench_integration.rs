//! Integration test for the load_ge08_authoring_workbench_snapshot command.
//!
//! This test validates that the command correctly loads and previews the
//! valid guard-stance package, returning all required result families
//! without counterfeit success.

use std::path::PathBuf;

#[test]
fn test_ge08_workbench_command_with_valid_package() {
    // This test validates the command implementation by loading the
    // guard-stance package directly (not through the Tauri command interface,
    // which would require a running app).

    use codex::homebrew_authoring::preview_bridge::PreviewBridge;
    use codex::homebrew_authoring::package_store::PackageStore;
    use codex::homebrew_authoring::package_manifest::PackageValidationState;

    let fixture_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .map(|p| p.join("tests/fixtures/ge08/guard-stance-package"))
        .expect("cannot determine fixture path");

    // Load the package
    let package = PackageStore::load(&fixture_root)
        .expect("fixture package should load successfully");

    assert_eq!(package.manifest.package_id, "pf1.homebrew.proof.guard-stance");

    // Generate the preview envelope
    let envelope = PreviewBridge::preview(&package);

    // Verify success-path contract
    assert_eq!(envelope.preview_status, codex::homebrew_authoring::preview_bridge::PreviewStatus::Success);
    assert_eq!(envelope.package_state, PackageValidationState::Valid);

    // Verify selected-slot resolution
    assert_eq!(envelope.selected_slot_resolution.slot, "human_bonus_feat");
    assert_eq!(envelope.selected_slot_resolution.removed, "dodge");
    assert_eq!(envelope.selected_slot_resolution.added, "homebrew_guard_stance");

    // Verify baseline armor class was computed (not blocked)
    match &envelope.baseline_armor_class {
        codex::homebrew_authoring::preview_bridge::ArmorClassPreview::Computed(value) => {
            assert_eq!(*value, 17, "baseline AC should be 17 (16 + 1 from authored effect)");
        }
        other => panic!("expected computed armor class, got {:?}", other),
    }

    // Verify provenance and explanation refs are present
    assert!(!envelope.provenance_refs.is_empty(), "provenance refs should be present");
    assert!(!envelope.explanation_refs.is_empty(), "explanation refs should be present");

    // Verify no claim-blocking diagnostics
    assert!(
        envelope.diagnostics.is_empty(),
        "valid package should have no diagnostics, got: {:?}",
        envelope.diagnostics
    );

    // Verify blocked claims are empty on success path
    assert!(envelope.blocked_claims.is_empty(), "no claims should be blocked on success path");
}

#[test]
fn test_ge08_workbench_blocked_path_when_effect_missing() {
    // This test validates the blocked-path contract: when a required field
    // (the effect) is missing, the preview must be blocked with diagnostics
    // preserved and blocked claims recorded.

    use codex::homebrew_authoring::preview_bridge::PreviewBridge;
    use codex::homebrew_authoring::SourcePackage;

    // Create a blocked package variant by removing the effect
    let mut blocked_package = SourcePackage::guard_stance_proof();
    blocked_package.effect = None;

    let envelope = PreviewBridge::preview(&blocked_package);

    // Verify blocked status
    assert_eq!(
        envelope.preview_status,
        codex::homebrew_authoring::preview_bridge::PreviewStatus::Blocked,
        "missing effect should block preview"
    );

    // Verify armor class is explicitly blocked (not computed)
    match &envelope.baseline_armor_class {
        codex::homebrew_authoring::preview_bridge::ArmorClassPreview::Blocked(reason) => {
            assert!(!reason.is_empty(), "blocked reason must not be empty");
        }
        other => panic!("expected blocked armor class, got {:?}", other),
    }

    // Verify diagnostics are preserved
    assert!(!envelope.diagnostics.is_empty(), "blocked preview must preserve diagnostics");
    assert!(
        envelope.diagnostics.iter().any(|d| d.claim_blocking),
        "at least one diagnostic must be claim-blocking"
    );

    // Verify blocked claims include preview
    assert!(
        envelope.blocked_claims.iter().any(|c| c == "preview"),
        "blocked claims should include 'preview'"
    );

    // Verify provenance and explanation refs survive even on blocked path
    assert!(!envelope.provenance_refs.is_empty(), "provenance refs should survive blocked path");
    assert!(!envelope.explanation_refs.is_empty(), "explanation refs should survive blocked path");
}

#[test]
fn test_ge08_workbench_lifecycle_gates_for_invalid_package() {
    // This test validates that lifecycle gates correctly prevent export/preview
    // when the package is not valid.

    use codex::homebrew_authoring::package_manifest::PackageValidationState;
    use codex::homebrew_authoring::preview_bridge::PreviewBridge;
    use codex::homebrew_authoring::SourcePackage;

    let mut invalid_package = SourcePackage::guard_stance_proof();
    invalid_package.manifest.validation_state = PackageValidationState::Invalid;
    invalid_package.effect = None;

    let envelope = PreviewBridge::preview(&invalid_package);

    // Verify the package state is invalid
    assert_eq!(envelope.package_state, PackageValidationState::Invalid);

    // Verify that the command adapter would set export_allowed = false
    // (This is verified in the Tauri command handler)
    assert!(!envelope.blocked_claims.is_empty(), "invalid package should have blocked claims");
}
