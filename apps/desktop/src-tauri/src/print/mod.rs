//! Print (SD-39 E8.1; `decisions.md §4`, operator rulings R1, R2, R3, R5, R19).
//!
//! Rust makes the sheet's PDF: the character's `LoadSavedCharacterResponse` -- the very response
//! the Character Sheet renders -- goes through the system's layout file (`layout/`) into a
//! `SheetDocument` (`model.rs`), which the shared Typst frame (`resources/print/*.typ`) typesets
//! with the bundled fonts only (`typst.rs`). The print path computes no number (NR-2).
//!
//! The two sinks (operator rulings R5, R19): `save_sheet_pdf` writes the PDF where the frontend's
//! save-as dialog said and opens it; `print_sheet_pdf` writes it to a temp file and opens that in
//! the system's PDF viewer, where the player prints. Both open through `tauri-plugin-opener`.

pub mod blocks;
pub mod layout;
pub mod model;
mod typst;

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::character_custom::{custom_or_empty, CharacterCustomDto};
use crate::character_hub::{list_held_equipment_at_root, load_character_bio_at_root, resolve_character_root};
use crate::rule_system_adapter::load_saved_character_via_envelope;
use crate::sf_adapter::STARFINDER_RULE_SYSTEM_ID;
pub use model::Paper;

/// What to print: the paper, and the three option boxes (operator ruling R4, stacking: none of
/// them is "basic", all three is "full").
/// Every field defaults: a request naming only the paper prints the basic sheet.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PrintRequest {
    pub paper: Paper,
    /// Spellbook: the full text of every spell the character holds.
    pub spellbook: bool,
    /// Show the math: every engine row behind a printed number.
    pub math: bool,
    /// Bio and campaign notes: the character's bio and its campaigns' descriptions (R17).
    pub bio: bool,
    /// The campaigns whose party lists this character. They live in the frontend's store
    /// (`campaignModel.ts`), which the backend cannot read, so the caller sends them.
    pub campaigns: Vec<CampaignNote>,
}

/// One campaign the character is in: its name and its description, as the Campaign screen saved them.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignNote {
    pub name: String,
    #[serde(default)]
    pub description: String,
}

/// A rendered sheet: the PDF bytes and every warning Typst raised (none, on a sheet that fits).
#[derive(Debug)]
pub struct RenderedSheet {
    pub pdf: Vec<u8>,
    pub warnings: Vec<String>,
}

/// The font families the print world holds: the bundled faces, nothing from the system.
#[cfg(test)]
pub fn sheet_font_families() -> Vec<String> {
    typst::font_families()
}

/// Renders the character saved at `root` to a PDF through its system's layout file (Pathfinder,
/// Starfinder, Cyberpunk RED); another system is refused by name, never printed as another's sheet.
pub fn render_sheet_pdf_at_root(root: &Path, request: &PrintRequest) -> Result<RenderedSheet, String> {
    let document = sheet_document_at_root(root, request)?;
    let typeset = typst::typeset(&document)?;
    Ok(RenderedSheet { pdf: typeset.pdf, warnings: typeset.warnings })
}

/// The sheet the character saved at `root` prints as: its system's layout file over what the
/// screen reads -- the load response, the owned copies (`list_held_equipment_at_root`), the
/// Custom tab's records and the bio -- and the request's options.
///
/// The load is the one the `load_saved_character` command makes (`load_saved_character_via_envelope`,
/// which serves a Pathfinder save exactly as `load_saved_character_at_root` does). Starfinder and
/// Cyberpunk RED have no owned-copies list and no Custom tab, so their sources hold none.
pub fn sheet_document_at_root(root: &Path, request: &PrintRequest) -> Result<model::SheetDocument, String> {
    let loaded = load_saved_character_via_envelope(root)?;
    let system = loaded.summary.game_system.as_str();
    let layout: fn(&blocks::SheetSources) -> model::SheetDocument = match system {
        "pf1" => layout::pf1e::layout,
        STARFINDER_RULE_SYSTEM_ID => layout::sf1e::layout,
        CYBERPUNK_RED_ID => layout::cpr::layout,
        other => return Err(format!("print.unsupported_system: no print layout for game system `{other}`")),
    };
    let (held, custom) =
        if system == "pf1" { (list_held_equipment_at_root(root)?, custom_or_empty(root)) } else { (Vec::new(), CharacterCustomDto::default()) };
    let sources = blocks::SheetSources { loaded: &loaded, held, custom, bio: load_character_bio_at_root(root)?, request };
    Ok(layout(&sources))
}

/// The `game_system` a Cyberpunk RED save carries.
const CYBERPUNK_RED_ID: &str = codex::rules_core::game_system::GameSystem::CyberpunkRed.id();

/// What a save wrote: where, how many bytes, and any warning Typst raised making it (a sheet
/// that does not fit its paper says so here rather than printing clipped in silence; NR-4).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedSheetPdf {
    pub file_path: String,
    pub bytes: usize,
    pub warnings: Vec<String>,
}

/// Renders the sheet and writes it to `target`. A refused write names the path and writes nothing.
pub fn write_sheet_pdf_at_root(root: &Path, request: &PrintRequest, target: &Path) -> Result<SavedSheetPdf, String> {
    let rendered = render_sheet_pdf_at_root(root, request)?;
    std::fs::write(target, &rendered.pdf).map_err(|err| format!("{}: {err}", target.display()))?;
    Ok(SavedSheetPdf { file_path: target.display().to_string(), bytes: rendered.pdf.len(), warnings: rendered.warnings })
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveSheetPdfRequest {
    pub character_id: String,
    /// The paper and the option boxes.
    #[serde(flatten)]
    pub print: PrintRequest,
    /// The path the save-as dialog returned.
    pub file_path: String,
}

/// Save PDF (operator ruling R5): writes the sheet to the chosen file, then opens it in the
/// system's PDF viewer. A file that saved but could not be opened is reported as exactly that.
#[tauri::command]
pub fn save_sheet_pdf(app: tauri::AppHandle, request: SaveSheetPdfRequest) -> Result<SavedSheetPdf, String> {
    use tauri_plugin_opener::OpenerExt;
    let root = resolve_character_root(&app, &request.character_id)?;
    let target = PathBuf::from(&request.file_path);
    let saved = write_sheet_pdf_at_root(&root, &request.print, &target)?;
    app.opener()
        .open_path(saved.file_path.clone(), None::<&str>)
        .map_err(|err| format!("Saved {}, but it could not be opened: {err}", saved.file_path))?;
    Ok(saved)
}

/// The Print sink's file: `<temp_root>/codex-print/<character_id>.pdf`, replaced by each print of
/// that character, so printing never piles up files.
pub fn write_print_copy_at_root(
    root: &Path,
    character_id: &str,
    request: &PrintRequest,
    temp_root: &Path,
) -> Result<SavedSheetPdf, String> {
    let dir = temp_root.join("codex-print");
    std::fs::create_dir_all(&dir).map_err(|err| format!("{}: {err}", dir.display()))?;
    write_sheet_pdf_at_root(root, request, &dir.join(format!("{character_id}.pdf")))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrintSheetPdfRequest {
    pub character_id: String,
    /// The paper and the option boxes.
    #[serde(flatten)]
    pub print: PrintRequest,
}

/// Print (operator ruling R19): makes the PDF in the system temp directory and opens it in the
/// system's PDF viewer; the player prints from there. The same opener works on Linux, macOS and
/// Windows (`decisions.md §25`).
#[tauri::command]
pub fn print_sheet_pdf(app: tauri::AppHandle, request: PrintSheetPdfRequest) -> Result<SavedSheetPdf, String> {
    use tauri_plugin_opener::OpenerExt;
    // `resolve_character_root` refuses an id that is not a plain character id, so the file name
    // below cannot leave the print directory.
    let root = resolve_character_root(&app, &request.character_id)?;
    let saved = write_print_copy_at_root(
        &root,
        &request.character_id,
        &request.print,
        &std::env::temp_dir(),
    )?;
    app.opener()
        .open_path(saved.file_path.clone(), None::<&str>)
        .map_err(|err| format!("Made {}, but it could not be opened: {err}", saved.file_path))?;
    Ok(saved)
}

#[cfg(test)]
mod print_smoke;

#[cfg(test)]
mod print_golden;
