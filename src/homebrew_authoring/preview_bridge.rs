//! Headless preview and explanation bridge for the first bounded GE-08 proof package.
//!
//! This bridge is the smallest layer that turns a validated authored proof
//! package plus the fixed GE08-E1 proof binding into a headless result envelope
//! distinguishing `success`, `blocked`, and `unsupported` preview outcomes. It
//! stays deliberately bounded:
//! - it consumes only the deterministic authored package bundle plus the fixed
//!   GE08-E1 proof binding (no GE-07 UI, no plugin runtime, no widened authoring)
//! - it preserves diagnostics, provenance/source refs, explanation refs, and
//!   oracle-dimension status in the envelope even when the preview is refused
//! - it computes the bounded armor-class output from the GE-06 deterministic
//!   baseline (sans the substituted-out Dodge) plus the authored effect, never a
//!   counterfeit success
//!
//! The bridge does not invoke the GE-06 rules-core combat path directly: that
//! path is locked to the exact Dodge posture (it requires `feat:dodge` and
//! hard-codes the Dodge AC bonus), so the bounded armor-class baseline used here
//! is grounded in that same deterministic derivation rather than re-running it
//! with the substituted slot.

use std::path::Path;

use super::package_manifest::{PackageValidationState, ProofBinding};
use super::package_store::{PackageStore, PackageStoreError};
use super::{PackageDiagnostic, SourcePackage};

/// Fixed GE08-E1 proof binding the bridge is allowed to honour. The bridge must
/// not infer a different pilot case or slot substitution than this closure fixed.
const GE08_E1_CASE_ID: &str = "pf1-crb-human-fighter-level1-homebrew-feat-proof";
const GE08_E1_BASE_CASE_ID: &str = "pf1-crb-human-fighter-level1";
const GE08_E1_SLOT: &str = "human_bonus_feat";
const GE08_E1_REMOVE: &str = "dodge";
const GE08_E1_ADD: &str = "homebrew_guard_stance";

/// The bounded first-proof posture supports only the armor-class derived family.
const BOUNDED_TARGET_FAMILY: &str = "armor_class";

/// Recognized GE-04/GE-06 derived-value families. Targeting a recognized family
/// other than `armor_class` is a widening (`unsupported`), not mere structural
/// breakage (`blocked`).
const RECOGNIZED_DERIVED_FAMILIES: &[&str] = &[
    "armor_class",
    "attack_bonus",
    "saving_throw",
    "skill_modifier",
    "initiative",
];

/// Oracle dimensions the first-proof bridge previews.
const ARMOR_CLASS_DIMENSION: &str = "defense.baseline_armor_class";
const SELECTED_FEATS_DIMENSION: &str = "character.selected_feats_and_choice_slots";

/// GE-06 deterministic baseline armor class without the Human bonus feat slot
/// contribution: base 10 + Chain Shirt armor bonus (+4) + Dexterity contribution
/// (+2, capped at MAXDEX:4). The full GE-06 deterministic baseline of 17 includes
/// Dodge (+1); this slice substitutes that slot, so the slot contribution comes
/// from the authored effect instead. Grounded in
/// `src/rules_core/pilot_compute.rs` baseline armor-class derivation.
const GE06_BASE_ARMOR_CLASS_WITHOUT_BONUS_FEAT_SLOT: i16 = 16;

/// Distinguishes the three honest preview outcomes for the first proof package.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewStatus {
    /// The real bounded preview path ran and explanation/provenance obligations
    /// were satisfied.
    Success,
    /// Claim-blocking diagnostics or missing structure prevent a truthful preview.
    Blocked,
    /// The package stays structurally known but widens beyond the first-proof
    /// semantic posture.
    Unsupported,
}

/// Bounded armor-class preview output. The blocked variant always carries an
/// explicit marker; null-without-explanation is forbidden by construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmorClassPreview {
    Computed(i16),
    Blocked(String),
}

/// Echoes the exact Human bonus feat substitution that produced this preview.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotResolution {
    pub slot: String,
    pub removed: String,
    pub added: String,
    pub resolved_feat_id: String,
}

/// A provenance/source reference carried forward from authored source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceRef {
    pub stable_id: String,
    pub source_package_id: String,
    pub authored_path: String,
}

/// A GE-04-aligned explanation graph node reference. Node kinds mirror the GE-04
/// obligations: `character_input`, `source_package`, `canonical_object`,
/// `effect`, `derived_value`, `provenance`, `prerequisite`, and `diagnostic`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplanationRef {
    pub node_kind: String,
    pub ref_id: String,
    pub detail: String,
}

/// Status of a previewed oracle dimension (`previewed`, `blocked`, `unsupported`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OracleDimensionStatus {
    pub dimension: String,
    pub status: String,
}

/// The headless result envelope returned by the bridge. It stays useful even
/// when no GE-07 surface is present and even when the preview is refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreviewEnvelope {
    pub case_id: String,
    pub package_id: String,
    pub package_state: PackageValidationState,
    pub preview_status: PreviewStatus,
    pub selected_slot_resolution: SlotResolution,
    pub baseline_armor_class: ArmorClassPreview,
    pub diagnostics: Vec<PackageDiagnostic>,
    pub provenance_refs: Vec<ProvenanceRef>,
    pub explanation_refs: Vec<ExplanationRef>,
    pub oracle_dimension_status: Vec<OracleDimensionStatus>,
    pub blocked_claims: Vec<String>,
}

/// The bounded headless preview/explanation bridge entrypoint.
pub struct PreviewBridge;

impl PreviewBridge {
    /// Load a deterministic authored package bundle from disk and preview it.
    pub fn preview_from_root(root: &Path) -> Result<PreviewEnvelope, PackageStoreError> {
        let package = PackageStore::load(root)?;
        Ok(Self::preview(&package))
    }

    /// Produce the headless result envelope for an authored proof package.
    pub fn preview(package: &SourcePackage) -> PreviewEnvelope {
        let (package_state, diagnostics) = package.recompute_validation();
        let binding = &package.manifest.proof_binding;

        let selected_slot_resolution = SlotResolution {
            slot: binding.slot.clone(),
            removed: binding.remove.clone(),
            added: binding.add.clone(),
            resolved_feat_id: package
                .feat
                .as_ref()
                .map(|feat| feat.stable_id.clone())
                .unwrap_or_default(),
        };

        let provenance_refs = package
            .provenance
            .iter()
            .map(|entry| ProvenanceRef {
                stable_id: entry.stable_id.clone(),
                source_package_id: entry.source_package_id.clone(),
                authored_path: entry.authored_path.clone(),
            })
            .collect::<Vec<_>>();

        let posture = classify_posture(package, package_state, binding);

        let (
            preview_status,
            baseline_armor_class,
            explanation_refs,
            oracle_dimension_status,
            blocked_claims,
        ) = match posture {
            Posture::Supported => {
                let effect = package
                    .effect
                    .as_ref()
                    .expect("supported posture guarantees an authored effect");
                let armor_class =
                    GE06_BASE_ARMOR_CLASS_WITHOUT_BONUS_FEAT_SLOT + effect.modifier_value;
                (
                    PreviewStatus::Success,
                    ArmorClassPreview::Computed(armor_class),
                    supported_explanation_refs(package, binding, armor_class),
                    dimension_status("previewed"),
                    Vec::new(),
                )
            }
            Posture::Unsupported(reason) => (
                PreviewStatus::Unsupported,
                ArmorClassPreview::Blocked(reason.clone()),
                blocked_explanation_refs(package, binding, &reason),
                dimension_status("unsupported"),
                vec![
                    "preview".to_owned(),
                    "explanation".to_owned(),
                    "export".to_owned(),
                    "proof".to_owned(),
                ],
            ),
            Posture::Blocked(reason) => (
                PreviewStatus::Blocked,
                ArmorClassPreview::Blocked(reason.clone()),
                blocked_explanation_refs(package, binding, &reason),
                dimension_status("blocked"),
                vec![
                    "preview".to_owned(),
                    "explanation".to_owned(),
                    "export".to_owned(),
                    "proof".to_owned(),
                ],
            ),
        };

        PreviewEnvelope {
            case_id: binding.case_id.clone(),
            package_id: package.manifest.package_id.clone(),
            package_state,
            preview_status,
            selected_slot_resolution,
            baseline_armor_class,
            diagnostics,
            provenance_refs,
            explanation_refs,
            oracle_dimension_status,
            blocked_claims,
        }
    }
}

/// Internal proof-semantics posture classification, layered on top of structural
/// validation without changing validation semantics.
enum Posture {
    Supported,
    Unsupported(String),
    Blocked(String),
}

fn classify_posture(
    package: &SourcePackage,
    package_state: PackageValidationState,
    binding: &ProofBinding,
) -> Posture {
    // The bridge may only honour the exact fixed GE08-E1 proof binding. A
    // different case or slot substitution is refused rather than guessed.
    if binding.case_id != GE08_E1_CASE_ID
        || binding.slot != GE08_E1_SLOT
        || binding.remove != GE08_E1_REMOVE
        || binding.add != GE08_E1_ADD
    {
        return Posture::Blocked(format!(
            "proof binding (case '{}', slot '{}', remove '{}', add '{}') does not match the fixed \
             GE08-E1 substitution; the bridge refuses to infer a different case or slot",
            binding.case_id, binding.slot, binding.remove, binding.add
        ));
    }

    // Structural coherence: the authored feat and effect must both be present
    // before any preview reasoning is honest.
    let (Some(_feat), Some(effect)) = (&package.feat, &package.effect) else {
        return Posture::Blocked(
            "authored proof records are incomplete (feat and effect required); preview blocked"
                .to_owned(),
        );
    };

    // Unsupported widening: a structurally coherent effect that contributes to a
    // recognized derived-value family other than the bounded armor-class family.
    if effect.target_family != BOUNDED_TARGET_FAMILY
        && RECOGNIZED_DERIVED_FAMILIES.contains(&effect.target_family.as_str())
    {
        return Posture::Unsupported(format!(
            "authored effect '{}' targets derived family '{}'; the first-proof posture supports only \
             '{}', so preview is unsupported (widened) rather than silently accepted",
            effect.stable_id, effect.target_family, BOUNDED_TARGET_FAMILY
        ));
    }

    // Any remaining claim-blocking validation failure blocks the preview.
    if package_state != PackageValidationState::Valid {
        return Posture::Blocked(format!(
            "package validation state is '{}' with claim-blocking diagnostics; preview blocked",
            package_state.as_str()
        ));
    }

    Posture::Supported
}

/// Base explanation graph nodes shared by every outcome: the slot input, the
/// authored package, and whichever authored records are present, plus provenance.
fn base_explanation_refs(package: &SourcePackage, binding: &ProofBinding) -> Vec<ExplanationRef> {
    let mut refs = vec![
        ExplanationRef {
            node_kind: "character_input".to_owned(),
            ref_id: binding.slot.clone(),
            detail: format!(
                "Human bonus feat slot in inherited pilot case {GE08_E1_BASE_CASE_ID}; removes '{}' and adds '{}'",
                binding.remove, binding.add
            ),
        },
        ExplanationRef {
            node_kind: "source_package".to_owned(),
            ref_id: package.manifest.package_id.clone(),
            detail: "authored homebrew proof package contributing the substituted feat".to_owned(),
        },
    ];

    if let Some(feat) = &package.feat {
        refs.push(ExplanationRef {
            node_kind: "canonical_object".to_owned(),
            ref_id: feat.stable_id.clone(),
            detail: format!(
                "authored feat '{}' selected into the Human bonus feat slot",
                feat.display_name
            ),
        });
    }
    if let Some(effect) = &package.effect {
        refs.push(ExplanationRef {
            node_kind: "effect".to_owned(),
            ref_id: effect.stable_id.clone(),
            detail: format!(
                "authored effect contributes {:+} to '{}'",
                effect.modifier_value, effect.target_family
            ),
        });
    }
    if let Some(prerequisite) = &package.prerequisite {
        refs.push(ExplanationRef {
            node_kind: "prerequisite".to_owned(),
            ref_id: prerequisite.stable_id.clone(),
            detail: format!("authored prerequisite '{}'", prerequisite.predicate),
        });
    }
    for entry in &package.provenance {
        refs.push(ExplanationRef {
            node_kind: "provenance".to_owned(),
            ref_id: entry.stable_id.clone(),
            detail: format!("authored-source lineage at '{}'", entry.authored_path),
        });
    }

    refs
}

/// Explanation refs for a successful preview: the base graph plus the bounded
/// armor-class derived value that the authored effect contributed to.
fn supported_explanation_refs(
    package: &SourcePackage,
    binding: &ProofBinding,
    armor_class: i16,
) -> Vec<ExplanationRef> {
    let mut refs = base_explanation_refs(package, binding);
    refs.push(ExplanationRef {
        node_kind: "derived_value".to_owned(),
        ref_id: ARMOR_CLASS_DIMENSION.to_owned(),
        detail: format!(
            "bounded armor class = GE-06 baseline without the bonus feat slot ({GE06_BASE_ARMOR_CLASS_WITHOUT_BONUS_FEAT_SLOT}) \
             + authored effect contribution = {armor_class}"
        ),
    });
    refs
}

/// Explanation refs for a blocked/unsupported preview: the base graph plus an
/// explicit diagnostic node so the explanation path is never blank.
fn blocked_explanation_refs(
    package: &SourcePackage,
    binding: &ProofBinding,
    reason: &str,
) -> Vec<ExplanationRef> {
    let mut refs = base_explanation_refs(package, binding);
    refs.push(ExplanationRef {
        node_kind: "diagnostic".to_owned(),
        ref_id: ARMOR_CLASS_DIMENSION.to_owned(),
        detail: reason.to_owned(),
    });
    refs
}

/// Build the oracle-dimension status set for the previewed dimensions.
fn dimension_status(status: &str) -> Vec<OracleDimensionStatus> {
    vec![
        OracleDimensionStatus {
            dimension: ARMOR_CLASS_DIMENSION.to_owned(),
            status: status.to_owned(),
        },
        OracleDimensionStatus {
            dimension: SELECTED_FEATS_DIMENSION.to_owned(),
            status: status.to_owned(),
        },
    ]
}
