#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_pilot_shell_snapshot])
        .run(tauri::generate_context!())
        .expect("error while running codex desktop shell scaffold");
}
