//! Headless package-authoring surface for the first bounded GE-08 proof package.
//!
//! This slice deliberately stays small: one authored package, one feat, one
//! effect, one optional prerequisite, provenance, diagnostics, and deterministic
//! file persistence. No UI, plugin runtime, or broad rules-authoring breadth is
//! claimed here.

pub mod package_manifest;
pub mod package_store;

use std::collections::BTreeSet;

use package_manifest::{PackageManifest, PackageValidationState, ProofBinding};

const GUARD_STANCE_PACKAGE_ID: &str = "pf1.homebrew.proof.guard-stance";
const GUARD_STANCE_CASE_ID: &str = "pf1-crb-human-fighter-level1-homebrew-feat-proof";
const GUARD_STANCE_FEAT_ID: &str = "feat.homebrew.guard_stance";
const GUARD_STANCE_EFFECT_ID: &str = "effect.homebrew.guard_stance.ac_bonus";
const GUARD_STANCE_PREREQUISITE_ID: &str = "prerequisite.homebrew.guard_stance.dex13";
const PF1_CRB_PACKAGE_ID: &str = "pf1.crb";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourcePackage {
    pub manifest: PackageManifest,
    pub feat: Option<FeatRecord>,
    pub effect: Option<EffectRecord>,
    pub prerequisite: Option<PrerequisiteRecord>,
    pub provenance: Vec<ProvenanceEntry>,
    pub diagnostics: Vec<PackageDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatRecord {
    pub stable_id: String,
    pub package_id: String,
    pub display_name: String,
    pub object_kind: String,
    pub substitutes_for: String,
    pub effect_ids: Vec<String>,
    pub prerequisite_ids: Vec<String>,
    pub semantic_intent: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectRecord {
    pub stable_id: String,
    pub owning_feat_id: String,
    pub target_family: String,
    pub modifier_type: String,
    pub modifier_value: i16,
    pub stacking_posture: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrerequisiteRecord {
    pub stable_id: String,
    pub owning_feat_id: String,
    pub predicate: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceEntry {
    pub stable_id: String,
    pub source_package_id: String,
    pub canonical_target_id: String,
    pub canonical_target_field: String,
    pub authored_path: String,
    pub source_system: String,
    pub support: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PackageDiagnosticSeverity {
    Error,
    Warning,
}

impl PackageDiagnosticSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Error => "Error",
            Self::Warning => "Warning",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "Error" => Some(Self::Error),
            "Warning" => Some(Self::Warning),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PackageDiagnostic {
    pub class: String,
    pub severity: PackageDiagnosticSeverity,
    pub message: String,
    pub subject_ref: String,
    pub claim_blocking: bool,
}

impl SourcePackage {
    pub fn guard_stance_shell() -> Self {
        Self {
            manifest: guard_stance_manifest(PackageValidationState::Draft),
            feat: None,
            effect: None,
            prerequisite: None,
            provenance: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    pub fn guard_stance_proof() -> Self {
        Self {
            manifest: guard_stance_manifest(PackageValidationState::Valid),
            feat: Some(FeatRecord {
                stable_id: GUARD_STANCE_FEAT_ID.to_owned(),
                package_id: GUARD_STANCE_PACKAGE_ID.to_owned(),
                display_name: "Guard Stance".to_owned(),
                object_kind: "Feat".to_owned(),
                substitutes_for: "dodge".to_owned(),
                effect_ids: vec![GUARD_STANCE_EFFECT_ID.to_owned()],
                prerequisite_ids: vec![GUARD_STANCE_PREREQUISITE_ID.to_owned()],
                semantic_intent:
                    "substitute for Dodge in the GE-06 proof case with one bounded armor class bonus"
                        .to_owned(),
            }),
            effect: Some(EffectRecord {
                stable_id: GUARD_STANCE_EFFECT_ID.to_owned(),
                owning_feat_id: GUARD_STANCE_FEAT_ID.to_owned(),
                target_family: "armor_class".to_owned(),
                modifier_type: "dodge".to_owned(),
                modifier_value: 1,
                stacking_posture: "bounded-proof".to_owned(),
            }),
            prerequisite: Some(PrerequisiteRecord {
                stable_id: GUARD_STANCE_PREREQUISITE_ID.to_owned(),
                owning_feat_id: GUARD_STANCE_FEAT_ID.to_owned(),
                predicate: "DEX >= 13".to_owned(),
            }),
            provenance: vec![
                ProvenanceEntry {
                    stable_id: GUARD_STANCE_FEAT_ID.to_owned(),
                    source_package_id: GUARD_STANCE_PACKAGE_ID.to_owned(),
                    canonical_target_id: GUARD_STANCE_FEAT_ID.to_owned(),
                    canonical_target_field: "feat".to_owned(),
                    authored_path: "objects/feats/feat.homebrew.guard_stance.yaml".to_owned(),
                    source_system: "codex-authored".to_owned(),
                    support: "exact".to_owned(),
                },
                ProvenanceEntry {
                    stable_id: GUARD_STANCE_EFFECT_ID.to_owned(),
                    source_package_id: GUARD_STANCE_PACKAGE_ID.to_owned(),
                    canonical_target_id: GUARD_STANCE_EFFECT_ID.to_owned(),
                    canonical_target_field: "effect".to_owned(),
                    authored_path:
                        "rules/effects/effect.homebrew.guard_stance.ac_bonus.yaml".to_owned(),
                    source_system: "codex-authored".to_owned(),
                    support: "exact".to_owned(),
                },
                ProvenanceEntry {
                    stable_id: GUARD_STANCE_PREREQUISITE_ID.to_owned(),
                    source_package_id: GUARD_STANCE_PACKAGE_ID.to_owned(),
                    canonical_target_id: GUARD_STANCE_PREREQUISITE_ID.to_owned(),
                    canonical_target_field: "prerequisite".to_owned(),
                    authored_path:
                        "rules/prerequisites/prerequisite.homebrew.guard_stance.dex13.yaml"
                            .to_owned(),
                    source_system: "codex-authored".to_owned(),
                    support: "exact".to_owned(),
                },
            ],
            diagnostics: Vec::new(),
        }
    }

    pub fn normalized_for_persistence(&self) -> Self {
        let (validation_state, diagnostics) = self.recompute_validation();
        let mut normalized = self.clone();
        normalized.manifest.validation_state = validation_state;
        normalized.diagnostics = diagnostics;
        normalized
    }

    pub fn recompute_validation(&self) -> (PackageValidationState, Vec<PackageDiagnostic>) {
        let mut diagnostics = self.diagnostics.clone();

        if self.manifest.package_id.trim().is_empty() {
            diagnostics.push(diagnostic(
                "InvalidPackage",
                PackageDiagnosticSeverity::Error,
                "package_id",
                "manifest package_id must not be empty",
                true,
            ));
        }
        if self.manifest.package_title.trim().is_empty() {
            diagnostics.push(diagnostic(
                "InvalidPackage",
                PackageDiagnosticSeverity::Error,
                "package_title",
                "manifest package_title must not be empty",
                true,
            ));
        }
        if self.manifest.game_system_id.trim().is_empty() {
            diagnostics.push(diagnostic(
                "InvalidPackage",
                PackageDiagnosticSeverity::Error,
                "game_system_id",
                "manifest game_system_id must not be empty",
                true,
            ));
        }
        if self.manifest.package_version.trim().is_empty() {
            diagnostics.push(diagnostic(
                "InvalidPackage",
                PackageDiagnosticSeverity::Error,
                "package_version",
                "manifest package_version must not be empty",
                true,
            ));
        }
        if self.manifest.depends_on.iter().all(|value| value != PF1_CRB_PACKAGE_ID) {
            diagnostics.push(diagnostic(
                "InvalidPackage",
                PackageDiagnosticSeverity::Error,
                "depends_on",
                "first proof package must declare dependency on pf1.crb",
                true,
            ));
        }

        let has_any_authored_record = self.feat.is_some() || self.effect.is_some() || self.prerequisite.is_some();
        if !has_any_authored_record && self.provenance.is_empty() && diagnostics.is_empty() {
            return (PackageValidationState::Draft, Vec::new());
        }

        let Some(feat) = &self.feat else {
            diagnostics.push(diagnostic(
                "InvalidPackage",
                PackageDiagnosticSeverity::Error,
                "feat",
                "proof package is missing its authored feat record",
                true,
            ));
            return finalize_diagnostics(diagnostics);
        };

        let Some(effect) = &self.effect else {
            diagnostics.push(diagnostic(
                "InvalidPackage",
                PackageDiagnosticSeverity::Error,
                "effect",
                "proof package is missing its authored effect record",
                true,
            ));
            return finalize_diagnostics(diagnostics);
        };

        if feat.package_id != self.manifest.package_id {
            diagnostics.push(diagnostic(
                "InvalidPackage",
                PackageDiagnosticSeverity::Error,
                "feat.package_id",
                "feat record package_id must match manifest package_id",
                true,
            ));
        }
        if feat.object_kind != "Feat" {
            diagnostics.push(diagnostic(
                "InvalidPackage",
                PackageDiagnosticSeverity::Error,
                "feat.object_kind",
                "proof feat must declare object_kind Feat",
                true,
            ));
        }
        if effect.owning_feat_id != feat.stable_id {
            diagnostics.push(diagnostic(
                "InvalidPackage",
                PackageDiagnosticSeverity::Error,
                "effect.owning_feat_id",
                "effect owning_feat_id must match the authored feat stable_id",
                true,
            ));
        }
        if !feat.effect_ids.iter().any(|id| id == &effect.stable_id) {
            diagnostics.push(diagnostic(
                "InvalidPackage",
                PackageDiagnosticSeverity::Error,
                "feat.effect_ids",
                "feat record must reference the authored effect stable_id",
                true,
            ));
        }
        if effect.target_family != "armor_class" {
            diagnostics.push(diagnostic(
                "InvalidPackage",
                PackageDiagnosticSeverity::Error,
                "effect.target_family",
                "proof effect must target armor_class",
                true,
            ));
        }
        if effect.modifier_value != 1 {
            diagnostics.push(diagnostic(
                "InvalidPackage",
                PackageDiagnosticSeverity::Error,
                "effect.modifier_value",
                "proof effect must preserve the bounded +1 armor class modifier",
                true,
            ));
        }

        if let Some(prerequisite) = &self.prerequisite {
            if prerequisite.owning_feat_id != feat.stable_id {
                diagnostics.push(diagnostic(
                    "InvalidPackage",
                    PackageDiagnosticSeverity::Error,
                    "prerequisite.owning_feat_id",
                    "prerequisite owning_feat_id must match the authored feat stable_id",
                    true,
                ));
            }
            if prerequisite.predicate.trim().is_empty() {
                diagnostics.push(diagnostic(
                    "InvalidPackage",
                    PackageDiagnosticSeverity::Error,
                    "prerequisite.predicate",
                    "prerequisite parity must remain structured and non-empty",
                    true,
                ));
            }
            if !feat
                .prerequisite_ids
                .iter()
                .any(|id| id == &prerequisite.stable_id)
            {
                diagnostics.push(diagnostic(
                    "InvalidPackage",
                    PackageDiagnosticSeverity::Error,
                    "feat.prerequisite_ids",
                    "feat record must reference the authored prerequisite stable_id when prerequisite parity is present",
                    true,
                ));
            }
        }

        let mut stable_ids = BTreeSet::new();
        for stable_id in [
            feat.stable_id.as_str(),
            effect.stable_id.as_str(),
            self.prerequisite
                .as_ref()
                .map(|record| record.stable_id.as_str())
                .unwrap_or(""),
        ] {
            if stable_id.is_empty() {
                continue;
            }
            if !stable_ids.insert(stable_id.to_owned()) {
                diagnostics.push(diagnostic(
                    "InvalidPackage",
                    PackageDiagnosticSeverity::Error,
                    "stable_id",
                    format!("duplicate stable_id '{stable_id}'"),
                    true,
                ));
            }
        }

        let expected_provenance = {
            let mut expected = vec![
                (feat.stable_id.as_str(), "objects/feats/feat.homebrew.guard_stance.yaml"),
                (
                    effect.stable_id.as_str(),
                    "rules/effects/effect.homebrew.guard_stance.ac_bonus.yaml",
                ),
            ];
            if let Some(prerequisite) = &self.prerequisite {
                expected.push((
                    prerequisite.stable_id.as_str(),
                    "rules/prerequisites/prerequisite.homebrew.guard_stance.dex13.yaml",
                ));
            }
            expected
        };

        for (stable_id, authored_path) in expected_provenance {
            match self.provenance.iter().find(|entry| entry.stable_id == stable_id) {
                Some(entry) => {
                    if entry.source_package_id != self.manifest.package_id {
                        diagnostics.push(diagnostic(
                            "InvalidPackage",
                            PackageDiagnosticSeverity::Error,
                            "provenance.source_package_id",
                            format!(
                                "provenance for '{stable_id}' must use the manifest package_id"
                            ),
                            true,
                        ));
                    }
                    if entry.authored_path != authored_path {
                        diagnostics.push(diagnostic(
                            "InvalidPackage",
                            PackageDiagnosticSeverity::Error,
                            "provenance.authored_path",
                            format!(
                                "provenance for '{stable_id}' must point at '{authored_path}'"
                            ),
                            true,
                        ));
                    }
                }
                None => diagnostics.push(diagnostic(
                    "InvalidPackage",
                    PackageDiagnosticSeverity::Error,
                    "provenance",
                    format!("missing provenance for authored stable_id '{stable_id}'"),
                    true,
                )),
            }
        }

        finalize_diagnostics(diagnostics)
    }
}

fn guard_stance_manifest(validation_state: PackageValidationState) -> PackageManifest {
    PackageManifest {
        schema_version: 1,
        package_id: GUARD_STANCE_PACKAGE_ID.to_owned(),
        package_title: "Guard Stance Proof Package".to_owned(),
        game_system_id: "pf1".to_owned(),
        package_version: "0.1.0".to_owned(),
        depends_on: vec![PF1_CRB_PACKAGE_ID.to_owned()],
        supported_object_kinds: vec![
            "Feat".to_owned(),
            "Effect".to_owned(),
            "Prerequisite".to_owned(),
        ],
        validation_state,
        provenance_policy: "package-local".to_owned(),
        proof_binding: ProofBinding {
            case_id: GUARD_STANCE_CASE_ID.to_owned(),
            slot: "human_bonus_feat".to_owned(),
            remove: "dodge".to_owned(),
            add: "homebrew_guard_stance".to_owned(),
        },
    }
}

fn finalize_diagnostics(mut diagnostics: Vec<PackageDiagnostic>) -> (PackageValidationState, Vec<PackageDiagnostic>) {
    diagnostics.sort_by(|left, right| {
        left.subject_ref
            .cmp(&right.subject_ref)
            .then_with(|| left.class.cmp(&right.class))
            .then_with(|| left.message.cmp(&right.message))
    });
    diagnostics.dedup();

    let validation_state = if diagnostics.iter().any(|item| item.claim_blocking) {
        PackageValidationState::Invalid
    } else if diagnostics.is_empty() {
        PackageValidationState::Valid
    } else {
        PackageValidationState::Deferred
    };

    (validation_state, diagnostics)
}

fn diagnostic(
    class: impl Into<String>,
    severity: PackageDiagnosticSeverity,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
    claim_blocking: bool,
) -> PackageDiagnostic {
    PackageDiagnostic {
        class: class.into(),
        severity,
        message: message.into(),
        subject_ref: subject_ref.into(),
        claim_blocking,
    }
}
