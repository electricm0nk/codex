//! v0.8 B-12 -- `export_dm_console`: writes the DM console's self-contained
//! HTML (built entirely by the frontend, D-5) to the file the DM chose in
//! the native save dialog. Mirrors `export_character_json`: the frontend
//! picks the path via plugin-dialog `save()`, the command writes bytes.
//!
//! Three decisions, each pinned by a test:
//! - **Verbatim.** The bytes written are exactly the string received -- no
//!   reformatting, escaping, trimming or wrapping. A self-contained page
//!   stops being self-contained the moment something here "helps".
//! - **Overwrite.** An existing file at the chosen path is replaced. The
//!   native save dialog has already asked "Replace?" before the path ever
//!   reaches this command; refusing here would contradict the answer the
//!   DM just gave. Same behaviour as `export_character_json`.
//! - **Honest failure, no partial file.** The document is written to a
//!   sibling temp file and renamed into place, so a failed write leaves the
//!   previous file (if any) intact and never a half-written console; any
//!   failure is `Err` naming the path. An empty document is refused
//!   before touching the disk -- there is no DM console with no content,
//!   so writing one would be reporting success for nothing.

use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportDmConsoleRequest {
    /// The destination the DM chose in the native save dialog. Accepted
    /// under both `filePath` (the key `export_character_json` already
    /// uses) and `fileName` (the key the DM Toolkit brief names), so
    /// either boundary spelling reaches this field.
    #[serde(alias = "fileName")]
    pub file_path: String,
    /// The complete, self-contained console document. Written verbatim.
    pub html: String,
}

/// Writes `html` to `path` verbatim, atomically (sibling temp file +
/// rename), refusing an empty path or a blank document before touching the
/// disk. Split from the `#[tauri::command]` wrapper so it is unit-testable.
pub(crate) fn export_dm_console_to_path(path: &Path, html: &str) -> Result<(), String> {
    if path.as_os_str().is_empty() {
        return Err("no destination path was chosen for the DM console export".to_owned());
    }
    if html.trim().is_empty() {
        return Err(format!(
            "{}: refusing to write an empty DM console document",
            path.display()
        ));
    }
    let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) else {
        return Err(format!("{}: destination has no parent directory", path.display()));
    };
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .ok_or_else(|| format!("{}: destination has no file name", path.display()))?;

    let temp_path: PathBuf = parent.join(format!(".{file_name}.{}.tmp", std::process::id()));
    std::fs::write(&temp_path, html.as_bytes())
        .map_err(|err| format!("{}: {err}", path.display()))?;
    if let Err(err) = std::fs::rename(&temp_path, path) {
        std::fs::remove_file(&temp_path).ok();
        return Err(format!("{}: {err}", path.display()));
    }
    Ok(())
}

/// Writes the DM console HTML the frontend built to the path the DM chose.
/// See the module doc for the verbatim / overwrite / honest-failure
/// decisions.
#[tauri::command]
pub fn export_dm_console(request: ExportDmConsoleRequest) -> Result<(), String> {
    export_dm_console_to_path(Path::new(&request.file_path), &request.html)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tempdir(label: &str) -> std::path::PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("codex-dm-console-{label}-{unique}"));
        std::fs::create_dir_all(&dir).expect("temp dir");
        dir
    }

    const SAMPLE: &str = "<!doctype html>\r\n<html><head><style>body{color:#000}</style></head>\
<body><h1>Séance at the Gilded Lantern — “Act II”</h1><script>const x = 1 < 2 && 'a' > \"b\";</script>\
<p>%%literal percent%% and a\ttab</p></body></html>\n\n";

    #[test]
    fn writes_the_html_bytes_verbatim() {
        let dir = tempdir("verbatim");
        let path = dir.join("console.html");

        export_dm_console_to_path(&path, SAMPLE).expect("write should succeed");

        let on_disk = std::fs::read(&path).expect("file should exist");
        assert_eq!(on_disk, SAMPLE.as_bytes(), "every byte, including CRLF, tabs, trailing newlines");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn overwrites_an_existing_file_at_the_chosen_path() {
        let dir = tempdir("overwrite");
        let path = dir.join("console.html");
        std::fs::write(&path, "<p>old</p>").expect("seed");

        export_dm_console_to_path(&path, SAMPLE).expect("overwrite should succeed");

        assert_eq!(std::fs::read_to_string(&path).expect("read"), SAMPLE);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn leaves_no_temp_file_behind_after_a_successful_write() {
        let dir = tempdir("no-temp");
        let path = dir.join("console.html");

        export_dm_console_to_path(&path, SAMPLE).expect("write should succeed");

        let names: Vec<String> = std::fs::read_dir(&dir)
            .expect("list")
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(names, vec!["console.html".to_owned()], "only the console itself remains");
        std::fs::remove_dir_all(&dir).ok();
    }

    /// A write that cannot happen is an `Err` naming the path -- never an
    /// `Ok` with nothing on disk.
    #[test]
    fn a_write_into_a_missing_directory_fails_honestly_and_names_the_path() {
        let dir = tempdir("missing-dir");
        let path = dir.join("no-such-subdir").join("console.html");

        let result = export_dm_console_to_path(&path, SAMPLE);

        let err = result.expect_err("writing into a missing directory must fail");
        assert!(err.contains("console.html"), "error must name the target: {err}");
        assert!(!path.exists());
        std::fs::remove_dir_all(&dir).ok();
    }

    /// A failed write never replaces what was there before.
    #[test]
    fn a_failed_write_leaves_the_previous_file_intact() {
        let dir = tempdir("keep-previous");
        let path = dir.join("console.html");
        std::fs::write(&path, "<p>previous</p>").expect("seed");

        let result = export_dm_console_to_path(&path, "");

        assert!(result.is_err());
        assert_eq!(std::fs::read_to_string(&path).expect("read"), "<p>previous</p>");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn an_empty_document_is_refused_before_touching_the_disk() {
        let dir = tempdir("empty");
        let path = dir.join("console.html");

        let result = export_dm_console_to_path(&path, "   \n");

        assert!(result.is_err(), "an empty console is not a successful export");
        assert!(!path.exists(), "nothing may be written");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn an_empty_path_is_refused() {
        let result = export_dm_console_to_path(std::path::Path::new(""), SAMPLE);
        assert!(result.is_err());
    }

    /// The request deserialises from the camelCase the brief names
    /// (`fileName`) and from the key `export_character_json` already uses
    /// (`filePath`), so either boundary spelling reaches the same field.
    #[test]
    fn the_request_accepts_both_file_name_and_file_path_keys() {
        let a: ExportDmConsoleRequest =
            serde_json::from_str(r#"{"fileName":"/tmp/a.html","html":"<p/>"}"#).expect("fileName");
        let b: ExportDmConsoleRequest =
            serde_json::from_str(r#"{"filePath":"/tmp/b.html","html":"<p/>"}"#).expect("filePath");
        assert_eq!(a.file_path, "/tmp/a.html");
        assert_eq!(b.file_path, "/tmp/b.html");
        assert_eq!(a.html, "<p/>");
    }
}
