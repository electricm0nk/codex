//! Where a table generator writes: the `rules_tables` **data package** (SD-37 E4a.4a).
//!
//! SD-37 E4a.4 removed the compiled `rules_tables` module; the package
//! (`data/rules_tables/<table id>.json`, `codex::rules_core::rules_data_package`) is the tables'
//! only home. A generator builds the table's rows as the table's own row type (or a field-for-field
//! serde mirror of it) and hands them here. The file is rendered by the package's own
//! [`render_table`], so it is byte-for-byte the canonical form `rules_tables_package --check`
//! accepts, licence/PI stamp included.

use codex::rules_core::pi_table_sweep::{screen_generated_table, PiSweepHit};
use codex::rules_core::rules_data_package::{package_root, render_table, table_path, PACKAGE_RELATIVE};
use serde::Serialize;
use std::path::{Path, PathBuf};

/// Why a table was not written.
#[derive(Debug)]
pub enum WriteRefused {
    /// The rendered file carries a `pi_screening` blacklist term; nothing was written.
    ProductIdentity(Vec<PiSweepHit>),
    /// The file system refused the write.
    Io(std::io::Error),
}

impl std::fmt::Display for WriteRefused {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteRefused::ProductIdentity(hits) => {
                writeln!(f, "PI screening HARD STOP -- {} hit(s), nothing written:", hits.len())?;
                for hit in hits {
                    writeln!(f, "  {hit:?}")?;
                }
                Ok(())
            }
            WriteRefused::Io(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for WriteRefused {}

/// This checkout's package directory (`<repo>/data/rules_tables`).
pub fn package_dir() -> PathBuf {
    package_root(&crate::repo_root())
}

/// Render table `id` with `rows` and write it to `<root>/<id>.json`, creating its directory.
pub fn write_table<T: Serialize>(root: &Path, id: &str, rows: &[T]) -> Result<PathBuf, WriteRefused> {
    let text = render_table(id, rows);
    write_text(root, id, &text)
}

/// [`write_table`], refused before anything is written when the rendered file carries a
/// `pi_screening` blacklist term (`pi_table_sweep::screen_generated_table`, the screen the gap
/// generators ran over their generated text before E4a.4a).
pub fn write_screened_table<T: Serialize>(root: &Path, id: &str, rows: &[T]) -> Result<PathBuf, WriteRefused> {
    let hits = screen_table(id, rows);
    if !hits.is_empty() {
        return Err(WriteRefused::ProductIdentity(hits));
    }
    write_table(root, id, rows)
}

/// The blacklist hits in table `id`'s rendered file (empty = clean). A generator that writes
/// several tables screens all of them first, so a hit in one leaves every file untouched.
pub fn screen_table<T: Serialize>(id: &str, rows: &[T]) -> Vec<PiSweepHit> {
    screen_generated_table(&format!("{PACKAGE_RELATIVE}/{id}.json"), &render_table(id, rows))
}

fn write_text(root: &Path, id: &str, text: &str) -> Result<PathBuf, WriteRefused> {
    let path = table_path(root, id);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(WriteRefused::Io)?;
    }
    std::fs::write(&path, text).map_err(WriteRefused::Io)?;
    Ok(path)
}

/// `value` for a generator that builds `&'static str` row fields from owned corpus text. A
/// generator is a one-shot process: the strings live until it exits, as the compiled tables'
/// literals did.
pub fn leak(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}
