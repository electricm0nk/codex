#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod browser_handoff;
mod campaign_drive;
mod character_hub;
#[allow(non_snake_case)]
mod characterHub;
mod class_catalog;
mod class_catalog_generic;
mod class_feature_descriptions;
mod class_feature_feat_bridge;
mod class_feature_pool_picker;
mod class_spell_levels;
mod corpus_fixtures;
mod corpus_full;
mod corpus_ingest_diagnostic;
mod equipment_catalog;
mod feat_catalog;
mod authoring_workbench;
mod companion_catalog;
mod companion_pool_catalog;
mod intelligent_item_catalog;
mod monster_catalog;
mod pf1_adapter;
mod race_catalog;
mod race_trait_picker;
mod reference_library_catalog;
/// Test-only: the reach gate, which fails when ingested content has no
/// consumer carrying it to a player. Compiled out of the shipping binary
/// because it is a verification surface, not a runtime one.
#[cfg(test)]
mod reach_gate;
mod rule_system_adapter;
mod spell_catalog;
mod stub_adapter;
mod support_state_matrix_bridge;
mod trait_picker;
mod update;

use serde::Serialize;

use campaign_drive::{
    drive_delete_campaign, drive_list_campaigns, drive_load_campaign, drive_save_campaign,
    write_campaign_drive_artifacts,
};
use character_hub::{
    add_equipment_selection, add_feat_selection, add_spell_selection, adjust_character_hp,
    adjust_character_money, attach_equipment_modifier, clone_character, create_character,
    delete_character, delete_character_portrait, export_character, export_character_json,
    import_character, level_up_character, list_feats_for_character, list_saved_characters,
    load_character_bio,
    load_character_durability, load_character_money, load_character_portrait,
    list_race_creation_roster, load_saved_character, preview_level_up, purchase_equipment,
    record_and_prepare_spell_selection, remove_equipment_selection, remove_feat_selection,
    remove_spell_selection,
    save_character_portrait, set_skill_allocations, update_character_bio,
};
use characterHub::appendToCharacter::append_to_character;
use characterHub::recomputeCharacter::recompute_character;
use characterHub::reSaveCharacter::re_save_character;
use class_catalog::list_class_catalog;
use class_feature_descriptions::list_class_feature_descriptions;
use class_feature_feat_bridge::list_class_feature_feat_bridge_descriptions;
use class_feature_pool_picker::list_class_feature_pool_options;
use class_spell_levels::list_class_spell_levels;
use corpus_ingest_diagnostic::corpus_ingest_diagnostic;
use equipment_catalog::{list_equipment, list_equipment_catalog};
use feat_catalog::{list_feat_catalog, list_feats, list_weapon_targets};
use companion_catalog::list_companion_catalog;
use reference_library_catalog::list_reference_library_catalog;
use intelligent_item_catalog::list_intelligent_item_catalog;
use monster_catalog::list_monster_catalog;
use race_catalog::list_race_catalog;
use race_trait_picker::{list_alternate_racial_traits, resolve_race_alternate_selection};
use spell_catalog::{list_spell_catalog, list_spells};
use support_state_matrix_bridge::{build_support_state_matrix_snapshot, SupportStateMatrixSnapshot};
use trait_picker::list_available_character_traits;
use update::transaction::{
    is_install_eligible, perform_install, perform_restore_previous, verify_relaunch_artifact,
};

use authoring_workbench::{
    build_authoring_workbench_snapshot, AuthoringWorkbenchRequest, AuthoringWorkbenchSnapshot,
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
fn load_authoring_workbench_snapshot(
    request: AuthoringWorkbenchRequest,
) -> Result<AuthoringWorkbenchSnapshot, String> {
    build_authoring_workbench_snapshot(request)
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
            load_authoring_workbench_snapshot,
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
            purchase_equipment,
            attach_equipment_modifier,
            add_spell_selection,
            record_and_prepare_spell_selection,
            add_feat_selection,
            remove_feat_selection,
            remove_spell_selection,
            remove_equipment_selection,
            set_skill_allocations,
            append_to_character,
            re_save_character,
            list_saved_characters,
            load_saved_character,
            preview_level_up,
            save_character_portrait,
            load_character_portrait,
            delete_character_portrait,
            update_character_bio,
            load_character_bio,
            load_character_money,
            adjust_character_money,
            load_character_durability,
            adjust_character_hp,
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
            list_feat_catalog,
            // SD-27: Bestiary 1's 41 ingested monster stat blocks, which
            // reached no surface at all until this catalog landed.
            list_monster_catalog,
            list_companion_catalog,
            // SD-32 row 19 cycle 4: the twelve corpus content-kind
            // directories (ability/class_generic/deity/domain/feat_generic/
            // language/monster_generic/power/race_generic/skill/template/
            // trait_generic) that reached no surface at all -- a browsable
            // reference library, generic across every book and kind.
            list_reference_library_catalog,
            // SD31-W18: the intelligent/legendary item build system's own
            // ability scores, Ego and alignment components -- reached no
            // screen at all before this catalog landed.
            list_intelligent_item_catalog,
            // SD31-D7-PROSE-003: real corpus `DESC:` text for class
            // features, joined to the character sheet's own explanation ids
            // -- `ClassFeatureRow.detail` renders the engine's COMPUTED
            // derivation, never the rulebook prose, so this is a second,
            // additive field, not a replacement.
            list_class_feature_descriptions,
            // SD31-W29-CLASSFEATURE-FEATBRIDGE-001 (THE-BOX §2.1 F2): a
            // class_feature record whose entire content is a grant of an
            // already-separately-modelled feat carries no local
            // description of its own -- this serves the matched feat's
            // own already-verified text through the SAME DTO shape, a
            // second, additive population `class_feature_descriptions.rs`
            // never covers (disjoint by construction; see that module's
            // own doc comment).
            list_class_feature_feat_bridge_descriptions,
            // SD31-W22-POOLMEMBER-001: the browsable option-pool reference
            // catalog (Rogue Talents today) -- a menu of every real record,
            // regardless of selection, modelled on
            // `list_alternate_racial_traits`'s own precedent.
            list_class_feature_pool_options,
            list_equipment,
            list_spells,
            list_feats,
            // SD-27: the same catalog with each record's real prerequisite
            // verdict for a specific saved character, so the picker can grey
            // out what that character cannot take and say why.
            list_feats_for_character,
            list_weapon_targets,
            list_class_catalog,
            list_class_spell_levels,
            list_race_catalog,
            list_race_creation_roster,
            list_alternate_racial_traits,
            resolve_race_alternate_selection,
            list_available_character_traits,
            corpus_ingest_diagnostic
        ])
        .run(tauri::generate_context!())
        .expect("error while running codex");
}

#[cfg(test)]
mod path_resolution_tests {
    use crate::authoring_workbench::resolve_package_path;

    #[test]
    fn resolves_repo_relative_fixture_from_src_tauri_runtime() {
        let resolved = resolve_package_path("tests/fixtures/authoring_workbench/guard-stance-package")
            .expect("fixture path should resolve from repo root");

        assert!(
            resolved.ends_with("tests/fixtures/authoring_workbench/guard-stance-package"),
            "resolved path should end with the repo fixture path, got {}",
            resolved.display()
        );
        assert!(resolved.exists(), "resolved fixture path should exist: {}", resolved.display());
    }
}
