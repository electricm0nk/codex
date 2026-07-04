#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ge08_workbench;
mod sd11_update_action;
mod sd13_support_state_matrix;
mod update;

use serde::Serialize;

use sd11_update_action::{resolve_sd11_update_action, Sd11UpdateActionRequest, Sd11UpdateReleaseTruth};
use sd13_support_state_matrix::{load_sd13_support_state_matrix_snapshot, Sd13SupportStateMatrixSnapshot};
use update::transaction::{
    is_install_eligible, perform_install, perform_restore_previous, verify_relaunch_artifact,
};

use ge08_workbench::{
    build_ge08_workbench_snapshot, Ge08AuthoringWorkbenchRequest, Ge08AuthoringWorkbenchSnapshot,
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
    build_ge08_workbench_snapshot(request)
}

#[tauri::command]
fn sd11_update_action(request: Sd11UpdateActionRequest) -> Sd11UpdateReleaseTruth {
    resolve_sd11_update_action(request)
}

/// Read-only SD-13 support-state/debt bridge for the SD-11 tester workbench.
/// Returns the seeded SD-13 matrix truth verbatim; no filtering or promotion.
#[tauri::command]
fn load_sd13_support_state_matrix() -> Sd13SupportStateMatrixSnapshot {
    load_sd13_support_state_matrix_snapshot()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_pilot_shell_snapshot,
            load_ge08_authoring_workbench_snapshot,
            sd11_update_action,
            load_sd13_support_state_matrix,
            is_install_eligible,
            perform_install,
            perform_restore_previous,
            verify_relaunch_artifact
        ])
        .run(tauri::generate_context!())
        .expect("error while running codex desktop shell scaffold");
}

#[cfg(test)]
mod path_resolution_tests {
    use crate::ge08_workbench::resolve_package_path;

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
