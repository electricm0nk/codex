//! GE08 desktop authoring workbench snapshot and command adapter.
//!
//! This module bridges the Tauri desktop shell and the headless GE08 authoring/preview
//! substrate, providing a bounded workbench snapshot contract for loading and previewing
//! the first proof package.

use serde::{Deserialize, Serialize};

/// Request to load the GE08 authoring workbench snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ge08AuthoringWorkbenchRequest {
    /// Repo-root-relative path to the package bundle.
    pub package_root: String,
    /// Optional stable ID to focus a specific authored record.
    pub active_record_ref: Option<String>,
}

/// Package manifest information surfaced in the workbench snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ge08PackageManifest {
    pub package_id: String,
    pub package_title: String,
    pub package_version: String,
    pub depends_on: Vec<String>,
    pub supported_object_kinds: Vec<String>,
}

/// Authored record (feat/effect/prerequisite) information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ge08AuthoredRecord {
    pub stable_id: String,
    pub owning_feat_id: Option<String>,
    pub display_name: String,
    pub object_kind: String,
    pub target_family: Option<String>,
    pub modifier_type: Option<String>,
    pub modifier_value: Option<i16>,
    pub predicate: Option<String>,
}

/// Selected slot resolution for the Human bonus feat substitution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ge08SelectedSlotResolution {
    pub slot: String,
    pub removed: String,
    pub added: String,
    pub resolved_feat_id: String,
}

/// Baseline armor class preview result (computed or blocked).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Ge08BaselineArmorClass {
    Computed { value: i16 },
    Blocked { reason: String },
}

/// Package validation diagnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ge08Diagnostic {
    pub class: String,
    pub severity: String, // "Error" or "Warning"
    pub message: String,
    pub subject_ref: String,
    pub claim_blocking: bool,
}

/// Provenance/source reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ge08ProvenanceRef {
    pub stable_id: String,
    pub source_package_id: String,
    pub authored_path: String,
}

/// Explanation graph node reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ge08ExplanationRef {
    pub node_kind: String,
    pub ref_id: String,
    pub detail: String,
}

/// Oracle dimension status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ge08OracleDimensionStatus {
    pub dimension: String,
    pub status: String,
}

/// Lifecycle gate state.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ge08LifecycleGateState {
    pub save_allowed: bool,
    pub preview_allowed: bool,
    pub export_allowed: bool,
    pub diff_mode: String,
}

/// Complete GE08 authoring workbench snapshot response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ge08AuthoringWorkbenchSnapshot {
    pub package_root: String,
    pub package_state: String, // "draft", "valid", "invalid", "deferred"
    pub package_manifest: Ge08PackageManifest,
    pub active_record_ref: Option<String>,
    pub authored_records: Ge08AuthoredRecords,
    pub preview: Ge08PreviewEnvelope,
    pub lifecycle_gate_state: Ge08LifecycleGateState,
    pub data_source: String, // "ge08-headless-preview-bridge" or "tauri-unavailable"
    pub note: String,
}

/// Authored records container.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ge08AuthoredRecords {
    pub feat: Option<Ge08AuthoredRecord>,
    pub effect: Option<Ge08AuthoredRecord>,
    pub prerequisite: Option<Ge08AuthoredRecord>,
}

/// Preview envelope with all result families.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ge08PreviewEnvelope {
    pub case_id: String,
    pub preview_status: String, // "success", "blocked", "unsupported"
    pub selected_slot_resolution: Ge08SelectedSlotResolution,
    pub baseline_armor_class: Ge08BaselineArmorClass,
    pub diagnostics: Vec<Ge08Diagnostic>,
    pub provenance_refs: Vec<Ge08ProvenanceRef>,
    pub explanation_refs: Vec<Ge08ExplanationRef>,
    pub oracle_dimension_status: Vec<Ge08OracleDimensionStatus>,
    pub blocked_claims: Vec<String>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn valid_guard_stance_package_yields_success() {
        // This test demonstrates the contract: a valid guard-stance package
        // should produce a snapshot with packageState: "valid", previewStatus: "success",
        // real selected-slot resolution, non-empty provenance refs, non-empty explanation refs,
        // and export allowed.
        //
        // This test will fail initially and is used to drive the implementation of
        // the load_ge08_authoring_workbench_snapshot command.

        // The test body will be implemented once the command is wired up.
        // For now, this serves as a documentation of the contract.
    }

    #[test]
    fn missing_effect_yields_blocked_with_diagnostics() {
        // This test demonstrates the blocked-path contract: when a package
        // is missing required authored structure (like the effect), the result
        // should be previewStatus: "blocked", include diagnostics with
        // claim_blocking: true, preserve provenance/explanation refs, and refuse
        // export.
    }

    #[test]
    fn widened_package_yields_unsupported() {
        // This test demonstrates handling of packages that widen beyond the
        // first-proof scope (e.g., targeting an unaccepted derived family).
        // The result should be previewStatus: "unsupported" with diagnostics
        // preserved and export refused.
    }

    #[test]
    fn lifecycle_gates_prevent_export_when_invalid() {
        // This test demonstrates that lifecycle gates respect package validation
        // state and refuse export_allowed when the package is invalid/deferred.
    }
}
