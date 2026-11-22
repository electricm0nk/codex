//! E3.MC evidence tool: the sheet `compute_sheet` gives a saved Cyberpunk RED character.
//!
//! Reads a character directory exactly as `CyberpunkAdapter::load_saved_character`
//! (apps/desktop/src-tauri/src/cpr_adapter.rs) does -- `CprCharacterStore::load`, `load_choices`,
//! `load_lifepath`, then `compose_sheet` over the CPR corpus and `cpr_sheet_print::sheet_lines` --
//! and writes the result into a copy of a template `load_saved_character` response (the
//! Pathfinder-only blocks of a CPR answer are constant), replacing `summary`, `explanations`
//! and `sheetLines`. Nothing here computes a rule value.

use std::path::Path;

use codex::rules_core::cpr::sheet::{CprSheetCatalogs, compose_sheet};
use codex::rules_core::cpr::store::CprCharacterStore;
use serde::Serialize;

/// The wire shape `cpr_sheet_print.rs` builds (`character_hub::SheetLineDto`).
mod character_hub {
    #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SheetLineDto {
        pub id: String,
        pub kind: String,
        pub label: String,
        pub form: String,
        pub value: String,
        pub also: Vec<String>,
        pub prose: String,
        pub condition: Option<String>,
    }
}

#[allow(dead_code)]
#[path = "../../../../../../../../apps/desktop/src-tauri/src/cpr_sheet_print.rs"]
mod cpr_sheet_print;

#[derive(Serialize)]
struct Explanation {
    id: String,
    value: i16,
    detail: String,
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let [_, character_dir, corpus_dir, template, out] = args.as_slice() else {
        return Err("usage: e3mc_sheet_tool <character dir> <corpus dir> <template response json> <out json>".to_owned());
    };
    let root = Path::new(character_dir);
    let corpus = Path::new(corpus_dir);
    let character = CprCharacterStore::load(root)?;
    let choices = CprCharacterStore::load_choices(root)?;
    let lifepath = CprCharacterStore::load_lifepath(root)?;
    let sheet = compose_sheet(&CprSheetCatalogs::load(corpus)?, &character.input, &choices)?;
    let lines = cpr_sheet_print::sheet_lines(&cpr_sheet_print::CprPrintCatalog::load(corpus)?, &sheet, &choices, &lifepath)?;

    let text = std::fs::read_to_string(template).map_err(|e| format!("{template}: {e}"))?;
    let mut response: serde_json::Value = serde_json::from_str(&text).map_err(|e| format!("{template}: {e}"))?;
    let summary = &mut response["summary"];
    summary["characterId"] = character.character_id.clone().into();
    summary["displayLabel"] = character.display_label.clone().into();
    summary["schemaVersion"] = character.schema_version.into();
    summary["savedAt"] = character.saved_at.clone().into();
    let explanations: Vec<Explanation> =
        sheet.rows.iter().map(|e| Explanation { id: e.id.clone(), value: e.value, detail: e.detail.clone() }).collect();
    response["explanations"] = serde_json::to_value(&explanations).map_err(|e| e.to_string())?;
    response["sheetLines"] = serde_json::to_value(&lines).map_err(|e| e.to_string())?;
    response["sheetRulesUnavailableReason"] = if lines.is_empty() {
        "Cyberpunk RED has no sheet-rule package; its sheet renders the cpr. explanation rows".into()
    } else {
        serde_json::Value::Null
    };
    let json = serde_json::to_string_pretty(&response).map_err(|e| e.to_string())? + "\n";
    std::fs::write(out, json).map_err(|e| format!("{out}: {e}"))?;
    println!("{}: {} rows, {} printed lines", character.display_label, explanations.len(), lines.len());
    Ok(())
}
