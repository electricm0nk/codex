//! Self-update for installs from the Windows NSIS installer (`InstallKind::WindowsNsis`).
//!
//! A running `.exe` cannot be replaced, so the release's installer is downloaded, verified against
//! the manifest (size, sha256) and handed to a small detached script. The script waits for this
//! process to exit, runs the installer silently (`/S`, the NSIS default for the per-user install)
//! and starts Codex again. The installed-state record is refreshed on that next start by
//! `seed::reconcile_installed_state`, which sees a changed binary and version.
//!
//! The release's installer is not code-signed yet, so the manifest hash is the integrity check and
//! the URL allow-list is the provenance check; both run before anything is launched.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

use super::download::{download_to_file, validate_asset};
use super::transaction::{
    config_update_dir, sha256_of_file, InstallKind, InstalledState, RelaunchPrompt, INSTALLED_STATE_FILENAME,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowsInstallRequest {
    pub version: String,
    pub name: String,
    pub url: String,
    pub sha256: String,
    pub size_bytes: u64,
}

/// The side-effecting edges, injected so verification and script generation are testable off Windows.
pub trait WindowsSystem {
    fn download(&self, url: &str, dest: &Path) -> Result<(), String>;
    /// Starts the update script detached from this process.
    fn spawn_updater(&self, script: &Path) -> Result<(), String>;
}

pub fn parse_windows_install_request(manifest: &Value) -> Result<WindowsInstallRequest, String> {
    let version = manifest
        .get("version")
        .and_then(Value::as_str)
        .filter(|v| !v.is_empty())
        .ok_or("manifest has no version")?;
    let block = manifest
        .get("windows_nsis")
        .ok_or("this release publishes no Windows installer (manifest has no windows_nsis)")?;
    let field = |key: &str| {
        block
            .get(key)
            .and_then(Value::as_str)
            .filter(|v| !v.is_empty())
            .map(str::to_string)
            .ok_or(format!("manifest windows_nsis.{key} is missing"))
    };
    let size_bytes = block
        .get("size_bytes")
        .and_then(Value::as_u64)
        .ok_or("manifest windows_nsis.size_bytes is missing")?;
    Ok(WindowsInstallRequest {
        version: version.to_string(),
        name: field("name")?,
        url: field("url")?,
        sha256: field("sha256")?.to_ascii_lowercase(),
        size_bytes,
    })
}

/// Characters that would let a path break out of a quoted `cmd` argument.
const UNSAFE_PATH_CHARS: [char; 8] = ['"', '%', '^', '&', '|', '<', '>', '!'];

fn safe_for_batch(path: &Path) -> Result<String, String> {
    let text = path.to_string_lossy().into_owned();
    if text.chars().any(|c| UNSAFE_PATH_CHARS.contains(&c) || c.is_control()) {
        return Err(format!("refusing to build an update script around the unsafe path {text:?}"));
    }
    Ok(text)
}

/// The batch script that finishes the update: wait for process `pid` to exit, run `installer`
/// silently, then start `app` again. Paths are checked for characters that could inject commands.
pub fn update_script(pid: u32, installer: &Path, app: &Path) -> Result<String, String> {
    let installer = safe_for_batch(installer)?;
    let app = safe_for_batch(app)?;
    let lines = [
        "@echo off".to_string(),
        "rem Written by Codex to finish a self-update; safe to delete.".to_string(),
        ":wait".to_string(),
        format!("tasklist /FI \"PID eq {pid}\" /NH 2>NUL | findstr /C:\" {pid} \" >NUL"),
        "if not errorlevel 1 (".to_string(),
        "  ping -n 2 127.0.0.1 >NUL".to_string(),
        "  goto wait".to_string(),
        ")".to_string(),
        format!("\"{installer}\" /S"),
        "if errorlevel 1 exit /b 1".to_string(),
        format!("start \"\" \"{app}\""),
    ];
    Ok(lines.join("\r\n") + "\r\n")
}

/// Download and verify the release's installer, then start the updater script. Returns once the
/// updater is running; the installer itself runs after this process exits.
pub fn install_windows_update(
    config_dir: &Path,
    installed: &InstalledState,
    req: &WindowsInstallRequest,
    pid: u32,
    system: &dyn WindowsSystem,
) -> Result<RelaunchPrompt, String> {
    if installed.install_kind != InstallKind::WindowsNsis {
        return Err("this install is not a Windows installer install".to_string());
    }
    validate_asset(&req.url, &req.name, ".exe")?;

    let update_dir = config_update_dir(config_dir);
    let staging = update_dir.join("staging");
    fs::create_dir_all(&staging).map_err(|e| format!("cannot create {}: {e}", staging.display()))?;
    let installer: PathBuf = staging.join(&req.name);
    let script_path = staging.join("codex-update.cmd");
    let _ = fs::remove_file(&installer);
    let _ = fs::remove_file(&script_path);

    let verified = (|| {
        system.download(&req.url, &installer)?;
        let size = fs::metadata(&installer).map_err(|e| format!("downloaded file missing: {e}"))?.len();
        if size != req.size_bytes {
            return Err(format!("downloaded {size} bytes, manifest says {}", req.size_bytes));
        }
        let actual = sha256_of_file(&installer).map_err(|e| format!("cannot hash download: {e}"))?;
        if actual != req.sha256 {
            return Err(format!("sha256 mismatch: manifest {} downloaded {actual}", req.sha256));
        }
        Ok(())
    })();
    if let Err(error) = verified {
        let _ = fs::remove_file(&installer);
        return Err(error);
    }

    let started = update_script(pid, &installer, &installed.managed_executable_path)
        .and_then(|script| fs::write(&script_path, script).map_err(|e| format!("cannot write {}: {e}", script_path.display())))
        .and_then(|()| system.spawn_updater(&script_path));
    if let Err(error) = started {
        let _ = fs::remove_file(&installer);
        let _ = fs::remove_file(&script_path);
        return Err(error);
    }

    Ok(RelaunchPrompt {
        pending_update_path: update_dir.join(INSTALLED_STATE_FILENAME),
        managed_executable_path: installed.managed_executable_path.clone(),
        from_version: installed.version.clone(),
        to_version: req.version.clone(),
        artifact_sha256: req.sha256.clone(),
    })
}

/// The real edges: HTTPS download, and a detached `cmd` script on Windows.
pub struct SystemWindowsInstaller;

impl WindowsSystem for SystemWindowsInstaller {
    fn download(&self, url: &str, dest: &Path) -> Result<(), String> {
        download_to_file(url, dest)
    }

    #[cfg(windows)]
    fn spawn_updater(&self, script: &Path) -> Result<(), String> {
        use std::os::windows::process::CommandExt;
        const DETACHED_PROCESS: u32 = 0x0000_0008;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        std::process::Command::new(script)
            .creation_flags(DETACHED_PROCESS | CREATE_NO_WINDOW)
            .spawn()
            .map(|_| ())
            .map_err(|e| format!("cannot start the update script {}: {e}", script.display()))
    }

    #[cfg(not(windows))]
    fn spawn_updater(&self, _script: &Path) -> Result<(), String> {
        Err("the Windows updater can only be started on Windows".to_string())
    }
}

/// Run the Windows self-update for the recorded install. `manifest` is the raw published manifest.
pub fn perform_windows_install(config_dir: &Path, manifest: &Value) -> Result<RelaunchPrompt, String> {
    let record = config_update_dir(config_dir).join(INSTALLED_STATE_FILENAME);
    let bytes = fs::read(&record).map_err(|e| format!("no installed-state record at {}: {e}", record.display()))?;
    let installed: InstalledState =
        serde_json::from_slice(&bytes).map_err(|e| format!("installed-state.json is unreadable: {e}"))?;
    let req = parse_windows_install_request(manifest)?;
    install_windows_update(config_dir, &installed, &req, std::process::id(), &SystemWindowsInstaller)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::cell::RefCell;
    use std::fs;
    use std::path::PathBuf;

    const URL: &str = "https://github.com/electricm0nk/codex/releases/download/alpha-v0.16.141-abcdef12/Codex_0.16.141_x64-setup.exe";
    const PAYLOAD: &[u8] = b"nsis-installer-payload";

    fn sha(bytes: &[u8]) -> String {
        use sha2::{Digest, Sha256};
        Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
    }

    fn manifest() -> serde_json::Value {
        json!({
            "version": "0.16.141",
            "windows_nsis": {
                "name": "Codex_0.16.141_x64-setup.exe",
                "url": URL,
                "sha256": sha(PAYLOAD),
                "size_bytes": PAYLOAD.len(),
            },
        })
    }

    struct Fake {
        payload: Vec<u8>,
        spawned: RefCell<Vec<PathBuf>>,
        spawn_error: Option<String>,
    }

    impl Fake {
        fn serving(payload: &[u8]) -> Self {
            Fake { payload: payload.to_vec(), spawned: RefCell::new(Vec::new()), spawn_error: None }
        }
    }

    impl WindowsSystem for Fake {
        fn download(&self, _url: &str, dest: &Path) -> Result<(), String> {
            fs::write(dest, &self.payload).map_err(|e| e.to_string())
        }
        fn spawn_updater(&self, script: &Path) -> Result<(), String> {
            if let Some(error) = &self.spawn_error {
                return Err(error.clone());
            }
            self.spawned.borrow_mut().push(script.to_path_buf());
            Ok(())
        }
    }

    fn installed(dir: &Path) -> InstalledState {
        InstalledState {
            managed_executable_path: dir.join("Codex").join("codex-desktop.exe"),
            install_kind: InstallKind::WindowsNsis,
            channel: "alpha".into(),
            version: "0.16.140".into(),
            source_commit: "b".repeat(40),
            release_tag: "alpha/v0.16.140-bbbbbbbb".into(),
            manifest_hash: String::new(),
            artifact_sha256: "c".repeat(64),
            installed_at: "2026-10-01T00:00:00Z".into(),
            update_eligible: true,
            ineligible_reason: None,
        }
    }

    fn dir(label: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("codex-windows-install-{label}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn the_request_is_read_from_the_windows_nsis_block() {
        let req = parse_windows_install_request(&manifest()).expect("parses");
        assert_eq!((req.version.as_str(), req.name.as_str(), req.url.as_str()), ("0.16.141", "Codex_0.16.141_x64-setup.exe", URL));
        assert_eq!(req.sha256, sha(PAYLOAD));
        assert_eq!(req.size_bytes, PAYLOAD.len() as u64);
    }

    #[test]
    fn a_release_with_no_windows_installer_says_so() {
        let mut m = manifest();
        m.as_object_mut().unwrap().remove("windows_nsis");
        assert!(parse_windows_install_request(&m).unwrap_err().contains("windows_nsis"));
        for field in ["name", "url", "sha256", "size_bytes"] {
            let mut m = manifest();
            m["windows_nsis"].as_object_mut().unwrap().remove(field);
            assert!(parse_windows_install_request(&m).unwrap_err().contains(field), "{field}");
        }
    }

    #[test]
    fn a_verified_installer_is_handed_to_the_updater_and_the_app_is_told_to_close() {
        let d = dir("ok");
        let system = Fake::serving(PAYLOAD);
        let req = parse_windows_install_request(&manifest()).unwrap();
        let prompt = install_windows_update(&d, &installed(&d), &req, 4242, &system).expect("installs");
        assert_eq!((prompt.from_version.as_str(), prompt.to_version.as_str()), ("0.16.140", "0.16.141"));
        let spawned = system.spawned.borrow();
        assert_eq!(spawned.len(), 1, "the updater is started exactly once");
        let script = fs::read_to_string(&spawned[0]).unwrap();
        assert!(script.contains("Codex_0.16.141_x64-setup.exe"), "{script}");
        assert!(script.contains("4242"), "it waits for this process: {script}");
        assert!(script.contains("codex-desktop.exe"), "and starts Codex again: {script}");
        assert!(script.contains("/S"), "silently: {script}");
    }

    #[test]
    fn an_installer_that_does_not_match_the_manifest_is_never_run() {
        let d = dir("bad");
        let req = parse_windows_install_request(&manifest()).unwrap();
        for payload in [&b"tampered-installer!"[..], &b"nsis-installer-paylXad"[..]] {
            let system = Fake::serving(payload);
            let err = install_windows_update(&d, &installed(&d), &req, 1, &system).unwrap_err();
            assert!(err.contains("bytes") || err.contains("sha256"), "{err}");
            assert!(system.spawned.borrow().is_empty(), "nothing is launched");
        }
    }

    #[test]
    fn another_install_kind_or_a_foreign_url_is_refused_before_any_download() {
        let d = dir("refuse");
        let req = parse_windows_install_request(&manifest()).unwrap();
        let mut deb = installed(&d);
        deb.install_kind = InstallKind::Deb;
        assert!(install_windows_update(&d, &deb, &req, 1, &Fake::serving(PAYLOAD)).unwrap_err().contains("not a Windows installer install"));

        let mut m = manifest();
        m["windows_nsis"]["url"] = json!("https://evil.example/setup.exe");
        let foreign = parse_windows_install_request(&m).unwrap();
        assert!(install_windows_update(&d, &installed(&d), &foreign, 1, &Fake::serving(PAYLOAD)).unwrap_err().contains("not a codex release asset URL"));
    }

    #[test]
    fn a_spawn_failure_is_reported_not_swallowed() {
        let d = dir("spawn");
        let mut system = Fake::serving(PAYLOAD);
        system.spawn_error = Some("cannot start the updater".into());
        let req = parse_windows_install_request(&manifest()).unwrap();
        assert!(install_windows_update(&d, &installed(&d), &req, 1, &system).unwrap_err().contains("cannot start the updater"));
    }

    #[test]
    fn the_script_refuses_paths_that_could_inject_commands() {
        for bad in ["C:\\Users\\a\"b\\setup.exe", "C:\\a&calc\\setup.exe", "C:\\a%PATH%\\setup.exe", "C:\\a^b\\setup.exe", "C:\\a|b\\setup.exe", "C:\\a\nb\\setup.exe", "C:\\a!b\\setup.exe"] {
            let err = update_script(1, Path::new(bad), Path::new("C:\\ok\\codex.exe")).unwrap_err();
            assert!(err.contains("unsafe"), "{bad:?}: {err}");
        }
        assert!(update_script(1, Path::new("C:\\ok\\setup.exe"), Path::new("C:\\a<b\\codex.exe")).is_err());
        let ok = update_script(7, Path::new("C:\\Users\\Todd Hintzmann\\AppData\\setup.exe"), Path::new("C:\\Users\\Todd Hintzmann\\AppData\\Local\\Codex\\codex-desktop.exe")).unwrap();
        assert!(ok.contains("\r\n"), "batch files use CRLF");
        assert!(ok.contains("\"C:\\Users\\Todd Hintzmann\\AppData\\setup.exe\" /S"), "{ok}");
    }
}
