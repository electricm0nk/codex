//! `.pcg` character import (`decisions.md` §12, R13): a tool-side converter, shipped as the
//! `pcg_import` sidecar so the live app never parses a PCGen file. `parse` reads the line format,
//! `catalog` knows what Codex holds, `map` turns one into the other under the [`map::MAPPING`] table,
//! and `wire` is the JSON that comes out. The mapping table is printed in
//! `docs/architecture/pcg-import.md`.

pub mod catalog;
pub mod map;
pub mod parse;
pub mod wire;

#[cfg(test)]
mod fixtures_tests;

use std::path::Path;

/// Reads `input`, converts it, and writes the import file to `out`.
pub fn run(input: &Path, out: &Path, data_root: &Path) -> Result<wire::ImportFile, String> {
    let bytes = std::fs::read(input).map_err(|err| format!("{}: {err}", input.display()))?;
    // PCGen on Windows writes the platform's encoding, so a stray byte is not a reason to refuse.
    let text = String::from_utf8_lossy(&bytes);
    let catalog = catalog::Catalog::load(data_root)?;
    let source_file = input.file_name().map_or_else(|| input.display().to_string(), |name| name.to_string_lossy().into_owned());
    let file = map::convert(&text, &source_file, &catalog)?;
    let json = serde_json::to_string_pretty(&file).map_err(|err| err.to_string())?;
    std::fs::write(out, json).map_err(|err| format!("{}: {err}", out.display()))?;
    Ok(file)
}
