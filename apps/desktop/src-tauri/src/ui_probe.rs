//! Dev-only sink for the headless UI smoke harness
//! (`apps/desktop/scripts/ui-smoke/`).
//!
//! The frontend probe (`src/testSupport/uiProbe.ts`) posts a JSON snapshot of
//! the currently-rendered DOM (headings, body text, every interactive
//! element's name and rect) on every debounced mutation. This command's only
//! job is to get that JSON onto disk atomically, at a path the harness
//! chooses via `CODEX_UI_PROBE_FILE`, so a short-lived `node` process can
//! poll the file instead of needing any IPC of its own into the running app.
//!
//! Release builds keep the same command name and signature (so
//! `generate_handler!` compiles either way) but do nothing — this must never
//! ship a write-to-arbitrary-env-path primitive in a real build.

#[cfg(debug_assertions)]
#[tauri::command]
pub fn record_ui_probe(payload: String) -> Result<(), String> {
    use std::io::Write;

    let Ok(target_path) = std::env::var("CODEX_UI_PROBE_FILE") else {
        // No harness is watching — a normal `tauri dev` session. Silent no-op.
        return Ok(());
    };
    if target_path.trim().is_empty() {
        return Ok(());
    }

    let path = std::path::Path::new(&target_path);
    let dir = path.parent().unwrap_or_else(|| std::path::Path::new("."));

    // Write-temp-then-rename so the runner's poll loop (which reads this
    // file on its own schedule, concurrently with the app writing it) can
    // never observe a half-written JSON document.
    let mut tmp = tempfile_in(dir).map_err(|err| format!("ui_probe: failed to create temp file in {dir:?}: {err}"))?;
    tmp.write_all(payload.as_bytes())
        .map_err(|err| format!("ui_probe: failed to write temp file: {err}"))?;
    tmp.flush().map_err(|err| format!("ui_probe: failed to flush temp file: {err}"))?;
    let tmp_path = tmp.into_path();
    std::fs::rename(&tmp_path, path).map_err(|err| {
        format!("ui_probe: failed to rename {tmp_path:?} -> {path:?}: {err}")
    })?;

    Ok(())
}

#[cfg(not(debug_assertions))]
#[tauri::command]
pub fn record_ui_probe(_payload: String) -> Result<(), String> {
    Ok(())
}

/// Minimal named-temp-file-in-directory helper so this module does not need
/// a new crate dependency just for atomic-write. Returns an open, unlinked-
/// on-drop-if-never-persisted handle backed by a randomized filename in
/// `dir`; `into_path()` (used above, immediately followed by `rename`)
/// disarms that cleanup.
#[cfg(debug_assertions)]
fn tempfile_in(dir: &std::path::Path) -> std::io::Result<TempFile> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let pid = std::process::id();
    let path = dir.join(format!(".ui-probe-{pid}-{nanos}.tmp"));
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?;
    Ok(TempFile { file, path: Some(path) })
}

#[cfg(debug_assertions)]
struct TempFile {
    file: std::fs::File,
    path: Option<std::path::PathBuf>,
}

#[cfg(debug_assertions)]
impl TempFile {
    fn into_path(mut self) -> std::path::PathBuf {
        self.path.take().expect("into_path called twice")
    }
}

#[cfg(debug_assertions)]
impl std::io::Write for TempFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.file.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}

#[cfg(debug_assertions)]
impl Drop for TempFile {
    fn drop(&mut self) {
        // Only reached if `into_path` was never called (an error path above
        // returned before the rename) — clean up the abandoned temp file.
        if let Some(path) = self.path.take() {
            let _ = std::fs::remove_file(path);
        }
    }
}

#[cfg(all(test, debug_assertions))]
mod tests {
    use super::*;

    #[test]
    fn writes_payload_to_the_configured_path() {
        let dir = std::env::temp_dir().join(format!("ui_probe_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let target = dir.join("probe.json");
        std::env::set_var("CODEX_UI_PROBE_FILE", &target);

        record_ui_probe("{\"ts\":1}".to_string()).expect("record_ui_probe should succeed");

        let written = std::fs::read_to_string(&target).expect("probe file should exist");
        assert_eq!(written, "{\"ts\":1}");

        std::env::remove_var("CODEX_UI_PROBE_FILE");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn is_a_no_op_when_the_env_var_is_unset() {
        std::env::remove_var("CODEX_UI_PROBE_FILE");
        assert_eq!(record_ui_probe("{}".to_string()), Ok(()));
    }
}
