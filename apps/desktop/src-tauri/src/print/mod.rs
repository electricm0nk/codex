//! Print (SD-39 E8.1; `decisions.md §4`, operator rulings R1, R2, R3, R5, R19).
//!
//! Rust makes the sheet's PDF: the character's `LoadSavedCharacterResponse` -- the very response
//! the Character Sheet renders -- goes through the system's layout file (`layout/`) into a
//! `SheetDocument` (`model.rs`), which the shared Typst frame (`resources/print/*.typ`) typesets
//! with the bundled fonts only (`typst.rs`). The print path computes no number (NR-2).
//!
//! `save_sheet_pdf` is the Save PDF sink: the frontend's save-as dialog names the file, this
//! writes the PDF there and opens it in the system's PDF viewer (`tauri-plugin-opener`).

pub mod layout;
pub mod model;
mod typst;

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::character_hub::{load_saved_character_at_root, resolve_character_root};
pub use model::Paper;

/// What to print. E8.2 adds the three option boxes (Spellbook, Show the math, Bio and campaign
/// notes); the paper is the one choice E8.1 makes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrintRequest {
    pub paper: Paper,
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

/// Renders the character saved at `root` to a PDF. Pathfinder only until E8.3 adds the other two
/// systems' layout files; another system is refused by name, never printed as a Pathfinder sheet.
pub fn render_sheet_pdf_at_root(root: &Path, request: &PrintRequest) -> Result<RenderedSheet, String> {
    let loaded = load_saved_character_at_root(root)?;
    let system = loaded.summary.game_system.as_str();
    let document = match system {
        "pf1" => layout::pf1e::layout(&loaded, request.paper.typst_name()),
        other => return Err(format!("print.unsupported_system: no print layout for game system `{other}` yet")),
    };
    let typeset = typst::typeset(&document)?;
    Ok(RenderedSheet { pdf: typeset.pdf, warnings: typeset.warnings })
}

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
    #[serde(default)]
    pub paper: Paper,
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
    let saved = write_sheet_pdf_at_root(&root, &PrintRequest { paper: request.paper }, &target)?;
    app.opener()
        .open_path(saved.file_path.clone(), None::<&str>)
        .map_err(|err| format!("Saved {}, but it could not be opened: {err}", saved.file_path))?;
    Ok(saved)
}

#[cfg(test)]
mod print_smoke;
