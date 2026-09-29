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
//! This module also carries the DOM command channel's Rust-side half:
//! `poll_ui_probe_command` hands the frontend probe one queued command at a
//! time (click/type/select/key/scroll), read from `CODEX_UI_PROBE_CMD_FILE`.
//! The frontend executes it against the real DOM (a genuine `el.click()` /
//! native-setter-driven `input` event, never a synthetic OS-level input
//! event), which is what makes this deterministic where `xdotool` under
//! Xvfb+WebKitGTK is not — see that command's own doc comment.
//!
//! Release builds keep the same command names and signatures (so
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

/// Hands the frontend probe (`src/testSupport/uiProbe.ts`) one queued
/// command, if any is waiting, so the ui-smoke runner (`run.mjs`) can drive
/// clicks/typing/keys through a real DOM event dispatched by the webview
/// itself instead of a synthetic OS-level `xdotool` event racing WebKitGTK's
/// own input pipeline (see `run-desktop`'s SKILL.md gotchas — xdotool clicks
/// under Xvfb have observed, unpredictable dead stretches).
///
/// The runner writes one JSON command object (`{id, op, target?, text?,
/// key?, index?}`) to `CODEX_UI_PROBE_CMD_FILE`, write-temp-then-rename, the
/// same atomicity discipline `record_ui_probe` uses in the other direction.
/// This command claims it atomically too, so a poll racing the runner's next
/// write can never observe (or execute) a half-written command, and two
/// concurrent pollers (there should only ever be one, but nothing enforces
/// that) can never both claim the same command: `rename` from the fixed
/// command path to a `.taken` sibling either succeeds exactly once or fails
/// with the source already gone, never partially.
#[cfg(debug_assertions)]
#[tauri::command]
pub fn poll_ui_probe_command() -> Option<String> {
    let Ok(cmd_path) = std::env::var("CODEX_UI_PROBE_CMD_FILE") else {
        return None;
    };
    if cmd_path.trim().is_empty() {
        return None;
    }
    let path = std::path::Path::new(&cmd_path);
    if !path.exists() {
        return None;
    }
    let taken_path = std::path::PathBuf::from(format!("{cmd_path}.taken"));
    // Claim first, read second: the rename is the atomic hand-off point.
    // If it fails (file already gone -- another poll won the race, or the
    // runner hasn't finished writing it yet under a transient name), this
    // poll simply reports "nothing waiting" rather than erroring.
    if std::fs::rename(path, &taken_path).is_err() {
        return None;
    }
    let contents = std::fs::read_to_string(&taken_path).ok();
    let _ = std::fs::remove_file(&taken_path);
    contents
}

#[cfg(not(debug_assertions))]
#[tauri::command]
pub fn poll_ui_probe_command() -> Option<String> {
    None
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
    use std::sync::Mutex;

    // `record_ui_probe`/`poll_ui_probe_command` both read process-global env
    // vars (`CODEX_UI_PROBE_FILE`/`CODEX_UI_PROBE_CMD_FILE`), and Rust's test
    // harness runs `#[test]` fns concurrently on multiple threads within the
    // same process by default. Without serializing the env-var-mutating tests
    // below, e.g. `poll_is_none_when_the_env_var_is_unset`'s `remove_var` can
    // interleave between another test's `set_var` and its call into
    // `poll_ui_probe_command`, wiping the var out from under it — observed
    // directly as a flaky `poll_claims_and_returns_a_queued_command_exactly_once`
    // failure (expected `Some(..)`, got `None`) with no source change between
    // runs. One lock, held for the duration of each test that touches either
    // var, makes the whole group mutually exclusive. `unwrap_or_else` recovers
    // from a poisoned lock (an earlier test in the group panicking while
    // holding it) instead of cascading that failure into every test after it.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn writes_payload_to_the_configured_path() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
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
        let _guard = ENV_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        std::env::remove_var("CODEX_UI_PROBE_FILE");
        assert_eq!(record_ui_probe("{}".to_string()), Ok(()));
    }

    #[test]
    fn poll_claims_and_returns_a_queued_command_exactly_once() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let dir = std::env::temp_dir().join(format!("ui_probe_cmd_test_{}_a", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let cmd_path = dir.join("cmd.json");
        std::env::set_var("CODEX_UI_PROBE_CMD_FILE", &cmd_path);
        std::fs::write(&cmd_path, "{\"id\":\"cmd-1\",\"op\":\"click\",\"target\":\"Back\"}").unwrap();

        let first = poll_ui_probe_command();
        assert_eq!(first, Some("{\"id\":\"cmd-1\",\"op\":\"click\",\"target\":\"Back\"}".to_string()));

        // Claimed: neither the original path nor a leftover `.taken` sibling
        // survive, and a second poll finds nothing.
        assert!(!cmd_path.exists());
        assert!(!std::path::PathBuf::from(format!("{}.taken", cmd_path.display())).exists());
        assert_eq!(poll_ui_probe_command(), None);

        std::env::remove_var("CODEX_UI_PROBE_CMD_FILE");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn poll_returns_none_when_no_command_is_queued() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let dir = std::env::temp_dir().join(format!("ui_probe_cmd_test_{}_b", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let cmd_path = dir.join("cmd.json");
        std::env::set_var("CODEX_UI_PROBE_CMD_FILE", &cmd_path);

        assert_eq!(poll_ui_probe_command(), None);

        std::env::remove_var("CODEX_UI_PROBE_CMD_FILE");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn poll_is_none_when_the_env_var_is_unset() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        std::env::remove_var("CODEX_UI_PROBE_CMD_FILE");
        assert_eq!(poll_ui_probe_command(), None);
    }
}
