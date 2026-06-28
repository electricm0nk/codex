#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ge08_workbench;

use serde::Serialize;
use std::path::PathBuf;

use ge08_workbench::{
    Ge08AuthoredRecords, Ge08AuthoredRecord, Ge08AuthoringWorkbenchRequest, Ge08AuthoringWorkbenchSnapshot,
    Ge08BaselineArmorClass, Ge08Diagnostic, Ge08ExplanationRef, Ge08LifecycleGateState, Ge08OracleDimensionStatus,
    Ge08PackageManifest, Ge08PreviewEnvelope, Ge08ProvenanceRef, Ge08SelectedSlotResolution,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PilotShellSnapshot {
    case_id: String,
    source_package_id: String,
    receipt_status: String,
    summary_values: Option<serde_json::Value>,
    diagnostics: Vec<String>,
    explanation_refs: Vec<String>,
    data_source: String,
    note: String,
}

fn codex_repo_root() -> Result<PathBuf, String> {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .map(PathBuf::from)
        .ok_or_else(|| "cannot determine codex repo root from CARGO_MANIFEST_DIR".to_string())
}

fn resolve_package_path(package_root: &str) -> Result<PathBuf, String> {
    let requested = PathBuf::from(package_root);

    if requested.is_absolute() {
        return Ok(requested);
    }

    Ok(codex_repo_root()?.join(requested))
}

#[tauri::command]
fn load_pilot_shell_snapshot() -> PilotShellSnapshot {
    PilotShellSnapshot {
        case_id: "ge07-e1-scaffold-placeholder".into(),
        source_package_id: "pending-real-ge06-source-package".into(),
        receipt_status: "Unknown/Unavailable".into(),
        summary_values: None,
        diagnostics: vec![
            "Desktop scaffold is active, but real GE-06 data wiring is deferred to a later slice.".into(),
            "Frontend fallback must never masquerade as product truth.".into(),
        ],
        explanation_refs: vec!["future/load_pilot_shell_snapshot".into()],
        data_source: "tauri-command".into(),
        note: "This command exists to hold the runtime seam while the root headless core remains sovereign.".into(),
    }
}

#[tauri::command]
fn load_ge08_authoring_workbench_snapshot(
    request: Ge08AuthoringWorkbenchRequest,
) -> Result<Ge08AuthoringWorkbenchSnapshot, String> {
    use codex::homebrew_authoring::preview_bridge::PreviewBridge;

    let package_path = resolve_package_path(&request.package_root)?;

    if !package_path.exists() {
        return Err(format!(
            "package root does not exist: {} (resolved to {})",
            request.package_root,
            package_path.display()
        ));
    }

    let envelope = PreviewBridge::preview_from_root(&package_path)
        .map_err(|e| format!("failed to load/preview package: {}", e))?;

    let package = codex::homebrew_authoring::package_store::PackageStore::load(&package_path)
        .map_err(|e| format!("failed to load package source: {}", e))?;

    let (actual_state, _diags) = package.recompute_validation();
    let baseline_ac = match envelope.baseline_armor_class {
        codex::homebrew_authoring::preview_bridge::ArmorClassPreview::Computed(value) => {
            Ge08BaselineArmorClass::Computed { value }
        }
        codex::homebrew_authoring::preview_bridge::ArmorClassPreview::Blocked(reason) => {
            Ge08BaselineArmorClass::Blocked { reason }
        }
    };

    let export_allowed = actual_state == codex::homebrew_authoring::package_manifest::PackageValidationState::Valid;
    let preview_allowed = export_allowed && envelope.preview_status != codex::homebrew_authoring::preview_bridge::PreviewStatus::Blocked;

    Ok(Ge08AuthoringWorkbenchSnapshot {
        package_root: request.package_root,
        package_state: actual_state.as_str().to_string(),
        package_manifest: Ge08PackageManifest {
            package_id: package.manifest.package_id,
            package_title: package.manifest.package_title,
            package_version: package.manifest.package_version,
            depends_on: package.manifest.depends_on,
            supported_object_kinds: package.manifest.supported_object_kinds,
        },
        active_record_ref: request.active_record_ref,
        authored_records: Ge08AuthoredRecords {
            feat: package.feat.map(|f| Ge08AuthoredRecord {
                stable_id: f.stable_id,
                owning_feat_id: None,
                display_name: f.display_name,
                object_kind: f.object_kind,
                target_family: None,
                modifier_type: None,
                modifier_value: None,
                predicate: None,
            }),
            effect: package.effect.map(|e| Ge08AuthoredRecord {
                stable_id: e.stable_id,
                owning_feat_id: Some(e.owning_feat_id),
                display_name: e.target_family.clone(),
                object_kind: "effect".to_string(),
                target_family: Some(e.target_family),
                modifier_type: Some(e.modifier_type),
                modifier_value: Some(e.modifier_value),
                predicate: None,
            }),
            prerequisite: package.prerequisite.map(|p| Ge08AuthoredRecord {
                stable_id: p.stable_id,
                owning_feat_id: Some(p.owning_feat_id),
                display_name: p.predicate.clone(),
                object_kind: "prerequisite".to_string(),
                target_family: None,
                modifier_type: None,
                modifier_value: None,
                predicate: Some(p.predicate),
            }),
        },
        preview: Ge08PreviewEnvelope {
            case_id: envelope.case_id,
            preview_status: match envelope.preview_status {
                codex::homebrew_authoring::preview_bridge::PreviewStatus::Success => "success".to_string(),
                codex::homebrew_authoring::preview_bridge::PreviewStatus::Blocked => "blocked".to_string(),
                codex::homebrew_authoring::preview_bridge::PreviewStatus::Unsupported => "unsupported".to_string(),
            },
            selected_slot_resolution: Ge08SelectedSlotResolution {
                slot: envelope.selected_slot_resolution.slot,
                removed: envelope.selected_slot_resolution.removed,
                added: envelope.selected_slot_resolution.added,
                resolved_feat_id: envelope.selected_slot_resolution.resolved_feat_id,
            },
            baseline_armor_class: baseline_ac,
            diagnostics: envelope
                .diagnostics
                .iter()
                .map(|d| Ge08Diagnostic {
                    class: d.class.clone(),
                    severity: d.severity.as_str().to_string(),
                    message: d.message.clone(),
                    subject_ref: d.subject_ref.clone(),
                    claim_blocking: d.claim_blocking,
                })
                .collect(),
            provenance_refs: envelope
                .provenance_refs
                .iter()
                .map(|p| Ge08ProvenanceRef {
                    stable_id: p.stable_id.clone(),
                    source_package_id: p.source_package_id.clone(),
                    authored_path: p.authored_path.clone(),
                })
                .collect(),
            explanation_refs: envelope
                .explanation_refs
                .iter()
                .map(|e| Ge08ExplanationRef {
                    node_kind: e.node_kind.clone(),
                    ref_id: e.ref_id.clone(),
                    detail: e.detail.clone(),
                })
                .collect(),
            oracle_dimension_status: envelope
                .oracle_dimension_status
                .iter()
                .map(|o| Ge08OracleDimensionStatus {
                    dimension: o.dimension.clone(),
                    status: o.status.clone(),
                })
                .collect(),
            blocked_claims: envelope.blocked_claims,
        },
        lifecycle_gate_state: Ge08LifecycleGateState {
            save_allowed: true,
            preview_allowed,
            export_allowed,
            diff_mode: "deferred".to_string(),
        },
        data_source: "ge08-headless-preview-bridge".to_string(),
        note: "Real GE-08 authoring workbench snapshot from headless substrate.".to_string(),
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_pilot_shell_snapshot, load_ge08_authoring_workbench_snapshot])
        .run(tauri::generate_context!())
        .expect("error while running codex desktop shell scaffold");
}

#[cfg(test)]
mod path_resolution_tests {
    use super::resolve_package_path;

    #[test]
    fn resolves_repo_relative_fixture_from_src_tauri_runtime() {
        let resolved = resolve_package_path("tests/fixtures/ge08/guard-stance-package")
            .expect("fixture path should resolve from repo root");

        assert!(
            resolved.ends_with("tests/fixtures/ge08/guard-stance-package"),
            "resolved path should end with the repo fixture path, got {}",
            resolved.display()
        );
        assert!(resolved.exists(), "resolved fixture path should exist: {}", resolved.display());
    }
}
