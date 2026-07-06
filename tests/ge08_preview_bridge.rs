//! GE08-E4-F1 headless preview and explanation bridge gate.
//!
//! Proves the first bounded GE-08 preview/explanation bridge turns a validated
//! authored proof package plus the fixed GE08-E1 proof binding into a headless
//! result envelope without counterfeit success by:
//! 1. producing the bounded armor-class preview output for a valid package
//! 2. returning a blocked envelope with diagnostics and references preserved
//!    instead of counterfeit success
//! 3. keeping unsupported (widened) authored content explicit rather than
//!    silently widening it into a success
//!
//! This test consumes the bridge through the crate surface (`codex::homebrew_authoring`)
//! to prove the module is genuinely surfaced from `src/lib.rs`.

use std::path::PathBuf;

use codex::homebrew_authoring::SourcePackage;
use codex::homebrew_authoring::package_manifest::PackageValidationState;
use codex::homebrew_authoring::preview_bridge::{ArmorClassPreview, PreviewBridge, PreviewStatus};

const PROOF_CASE_ID: &str = "pf1-crb-human-fighter-level1-homebrew-feat-proof";
const PROOF_PACKAGE_ID: &str = "pf1.homebrew.proof.guard-stance";
const PROOF_FEAT_ID: &str = "feat.homebrew.guard_stance";
const PROOF_EFFECT_ID: &str = "effect.homebrew.guard_stance.ac_bonus";
const ARMOR_CLASS_DIMENSION: &str = "defense.baseline_armor_class";

fn fixture_root(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/ge08/{name}"))
}

#[test]
fn ge08_preview_bridge_valid_package_produces_bounded_armor_class() {
    // A validated on-disk proof package plus the fixed proof binding must reach
    // the bounded armor-class preview path through the bridge.
    let envelope = PreviewBridge::preview_from_root(&fixture_root("guard-stance-package"))
        .expect("valid fixture should load and preview");

    assert_eq!(envelope.preview_status, PreviewStatus::Success);
    assert_eq!(envelope.package_state, PackageValidationState::Valid);
    assert_eq!(envelope.case_id, PROOF_CASE_ID);
    assert_eq!(envelope.package_id, PROOF_PACKAGE_ID);

    // The slot resolution must echo the exact Human bonus feat substitution so
    // downstream consumers cannot mistake this for a generic feat preview.
    assert_eq!(envelope.selected_slot_resolution.slot, "human_bonus_feat");
    assert_eq!(envelope.selected_slot_resolution.removed, "dodge");
    assert_eq!(
        envelope.selected_slot_resolution.added,
        "homebrew_guard_stance"
    );
    assert_eq!(
        envelope.selected_slot_resolution.resolved_feat_id,
        PROOF_FEAT_ID
    );

    // The bounded armor class is the real derived value: GE-06 deterministic
    // baseline without the substituted-out Dodge (16) + authored effect (+1) = 17.
    assert_eq!(
        envelope.baseline_armor_class,
        ArmorClassPreview::Computed(17)
    );

    // A valid preview carries no claim-blocking diagnostics and no blocked claims.
    assert!(
        envelope.diagnostics.is_empty(),
        "valid preview must have no diagnostics, got: {:?}",
        envelope.diagnostics
    );
    assert!(envelope.blocked_claims.is_empty());

    // Provenance/source refs survive into the envelope.
    assert!(
        envelope
            .provenance_refs
            .iter()
            .any(|p| p.stable_id == PROOF_FEAT_ID)
    );
    assert!(
        envelope
            .provenance_refs
            .iter()
            .any(|p| p.stable_id == PROOF_EFFECT_ID)
    );

    // Explanation refs preserve the semantic path: slot -> feat -> effect -> AC.
    assert!(
        envelope
            .explanation_refs
            .iter()
            .any(|e| e.node_kind == "character_input")
    );
    assert!(
        envelope
            .explanation_refs
            .iter()
            .any(|e| e.node_kind == "canonical_object" && e.ref_id == PROOF_FEAT_ID)
    );
    assert!(
        envelope
            .explanation_refs
            .iter()
            .any(|e| e.node_kind == "effect" && e.ref_id == PROOF_EFFECT_ID)
    );
    assert!(
        envelope
            .explanation_refs
            .iter()
            .any(|e| e.node_kind == "derived_value")
    );

    // Oracle-dimension status reports the previewed armor-class dimension.
    assert!(
        envelope
            .oracle_dimension_status
            .iter()
            .any(|d| d.dimension == ARMOR_CLASS_DIMENSION && d.status == "previewed")
    );
}

#[test]
fn ge08_preview_bridge_blocked_package_returns_blocked_envelope_with_diagnostics() {
    // Drop the authored effect: the package is now structurally incomplete and
    // must be blocked rather than previewed.
    let mut blocked = SourcePackage::guard_stance_proof();
    blocked.effect = None;

    let envelope = PreviewBridge::preview(&blocked);

    assert_eq!(envelope.preview_status, PreviewStatus::Blocked);

    // Armor class must be an explicit blocked marker, never null-without-explanation.
    match &envelope.baseline_armor_class {
        ArmorClassPreview::Blocked(marker) => {
            assert!(
                !marker.trim().is_empty(),
                "blocked marker must explain the refusal"
            )
        }
        other => panic!("blocked preview must not compute an armor class, got {other:?}"),
    }

    // Diagnostics are preserved and remain claim-blocking.
    assert!(
        !envelope.diagnostics.is_empty(),
        "blocked preview must preserve diagnostics"
    );
    assert!(envelope.diagnostics.iter().any(|d| d.claim_blocking));

    // References survive even when the preview is blocked.
    assert!(!envelope.provenance_refs.is_empty());
    assert!(!envelope.explanation_refs.is_empty());

    // Blocked claims explicitly name preview, explanation, and export.
    assert!(envelope.blocked_claims.iter().any(|c| c == "preview"));
    assert!(envelope.blocked_claims.iter().any(|c| c == "explanation"));
    assert!(envelope.blocked_claims.iter().any(|c| c == "export"));

    // The slot resolution is still echoed even on the blocked path.
    assert_eq!(envelope.selected_slot_resolution.removed, "dodge");
    assert_eq!(
        envelope.selected_slot_resolution.added,
        "homebrew_guard_stance"
    );

    // Oracle-dimension status marks the armor-class dimension blocked.
    assert!(
        envelope
            .oracle_dimension_status
            .iter()
            .any(|d| d.dimension == ARMOR_CLASS_DIMENSION && d.status == "blocked")
    );
}

#[test]
fn ge08_preview_bridge_unsupported_widening_is_explicit() {
    // Widen the authored effect to a recognized-but-out-of-scope derived family.
    // The bridge must call this unsupported, not silently widen or counterfeit
    // success.
    let mut widened = SourcePackage::guard_stance_proof();
    if let Some(effect) = widened.effect.as_mut() {
        effect.target_family = "attack_bonus".to_owned();
    }

    let envelope = PreviewBridge::preview(&widened);

    assert_eq!(
        envelope.preview_status,
        PreviewStatus::Unsupported,
        "widening to a different derived family must be explicit, not silently accepted"
    );

    match &envelope.baseline_armor_class {
        ArmorClassPreview::Blocked(marker) => assert!(
            marker.contains("armor_class") || marker.to_lowercase().contains("unsupported"),
            "unsupported marker must name the bounded family or the unsupported posture: {marker}"
        ),
        other => panic!("unsupported preview must not compute an armor class, got {other:?}"),
    }

    // Diagnostics and references are still preserved.
    assert!(!envelope.diagnostics.is_empty());
    assert!(!envelope.provenance_refs.is_empty());

    // The proof and preview claims are explicitly blocked.
    assert!(envelope.blocked_claims.iter().any(|c| c == "proof"));
    assert!(envelope.blocked_claims.iter().any(|c| c == "preview"));

    // Oracle-dimension status marks the armor-class dimension unsupported.
    assert!(
        envelope
            .oracle_dimension_status
            .iter()
            .any(|d| d.dimension == ARMOR_CLASS_DIMENSION && d.status == "unsupported")
    );
}
