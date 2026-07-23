#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod browser_handoff;
mod campaign_drive;
mod character_hub;
#[allow(non_snake_case)]
mod characterHub;
mod class_catalog;
mod corpus_fixtures;
mod corpus_ingest_diagnostic;
mod equipment_catalog;
mod ge08_workbench;
mod pf1_adapter;
mod race_catalog;
mod rule_system_adapter;
mod spell_catalog;
mod stub_adapter;
mod support_state_matrix_bridge;
mod update;

use serde::Serialize;

use campaign_drive::{
    drive_delete_campaign, drive_list_campaigns, drive_load_campaign, drive_save_campaign,
    write_campaign_drive_artifacts,
};
use character_hub::{
    add_equipment_selection, add_spell_selection, clone_character, create_character,
    delete_character, delete_character_portrait, export_character, export_character_json,
    import_character, level_up_character, list_saved_characters, load_character_bio,
    load_character_portrait, load_saved_character, save_character_portrait,
    set_skill_allocations, update_character_bio,
};
use characterHub::appendToCharacter::append_to_character;
use characterHub::recomputeCharacter::recompute_character;
use characterHub::reSaveCharacter::re_save_character;
use class_catalog::list_class_catalog;
use corpus_ingest_diagnostic::corpus_ingest_diagnostic;
use equipment_catalog::{list_equipment, list_equipment_catalog};
use race_catalog::list_race_catalog;
use spell_catalog::{list_spell_catalog, list_spells};
use support_state_matrix_bridge::{build_support_state_matrix_snapshot, SupportStateMatrixSnapshot};
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

/// Read-only SD-13 support-state/debt bridge for the SD-11 tester workbench.
/// Returns the seeded SD-13 matrix truth verbatim; no filtering or promotion.
#[tauri::command]
fn load_support_state_matrix() -> SupportStateMatrixSnapshot {
    build_support_state_matrix_snapshot()
}

/// Identifies which build of the Rust backend is actually running. `version`
/// is the crate's own Cargo.toml version (not the npm frontend version, which
/// is tracked separately); `gitCommit` is the short commit hash embedded at
/// compile time by build.rs, or "unknown" for a build outside a git checkout.
/// Reaching this command at all — regardless of what it returns — is itself
/// proof the Tauri IPC bridge to the Rust backend is alive.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BackendHealthSnapshot {
    version: String,
    git_commit: String,
}

#[tauri::command]
fn load_backend_health() -> BackendHealthSnapshot {
    BackendHealthSnapshot {
        version: env!("CARGO_PKG_VERSION").to_string(),
        git_commit: env!("CODEX_GIT_SHA").to_string(),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if let Err(err) = character_hub::seed_default_character_if_needed(app.handle()) {
                eprintln!("Failed to seed default character: {err}");
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_pilot_shell_snapshot,
            load_ge08_authoring_workbench_snapshot,
            load_support_state_matrix,
            load_backend_health,
            browser_handoff::handoff_defect_report_to_browser,
            is_install_eligible,
            perform_install,
            perform_restore_previous,
            verify_relaunch_artifact,
            create_character,
            clone_character,
            level_up_character,
            recompute_character,
            add_equipment_selection,
            add_spell_selection,
            set_skill_allocations,
            append_to_character,
            re_save_character,
            list_saved_characters,
            load_saved_character,
            save_character_portrait,
            load_character_portrait,
            delete_character_portrait,
            update_character_bio,
            load_character_bio,
            delete_character,
            export_character_json,
            export_character,
            import_character,
            write_campaign_drive_artifacts,
            drive_list_campaigns,
            drive_load_campaign,
            drive_save_campaign,
            drive_delete_campaign,
            list_equipment_catalog,
            list_spell_catalog,
            list_equipment,
            list_spells,
            list_class_catalog,
            list_race_catalog,
            corpus_ingest_diagnostic
        ])
        .run(tauri::generate_context!())
        .expect("error while running codex");
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
