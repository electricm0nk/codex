//! Self-update for installs run from an AppImage (`InstallKind::AppImage`).
//!
//! The staged-replace transaction in `transaction.rs` (download to staging, size + sha256 check,
//! rolling backup, pending-update marker, atomic replace) is what actually changes the file; this
//! module is its caller. It turns the published manifest's `linux_appimage` block into the
//! transaction's inputs, refuses anything that is not this repository's release assets, and streams
//! the download from GitHub. The next launch's `verify_relaunch_artifact` confirms the new file.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde_json::Value;
use sha2::{Digest, Sha256};

use super::download::{download_to_writer, validate_asset};
use super::transaction::{
    config_update_dir, execute_transaction, sha256_of_file, EligibilityPolicy, InstallKind, InstalledState,
    ManifestIdentity, RelaunchPrompt, RunningBuildIdentity, TransactionConfig, TransactionOutcome,
    INSTALLED_STATE_FILENAME,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppImageInstallRequest {
    pub schema_version: String,
    pub channel: String,
    pub version: String,
    pub release_tag: String,
    pub tranche_id: String,
    pub source_commit: String,
    pub name: String,
    pub url: String,
    pub sha256: String,
    pub size_bytes: u64,
    /// sha256 of the manifest as received, recorded so the install can be traced to it.
    pub manifest_hash: String,
}

fn text(value: &Value, key: &str, context: &str) -> Result<String, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|v| !v.is_empty())
        .map(str::to_string)
        .ok_or(format!("manifest {context}{key} is missing"))
}

pub fn parse_appimage_install_request(manifest: &Value) -> Result<AppImageInstallRequest, String> {
    let block = manifest
        .get("linux_appimage")
        .ok_or("this release publishes no AppImage (manifest has no linux_appimage)")?;
    let size_bytes = block
        .get("size_bytes")
        .and_then(Value::as_u64)
        .ok_or("manifest linux_appimage.size_bytes is missing")?;
    let canonical = serde_json::to_vec(manifest).map_err(|e| format!("cannot serialize the manifest: {e}"))?;
    Ok(AppImageInstallRequest {
        schema_version: text(manifest, "schema_version", "")?,
        channel: text(manifest, "channel", "")?,
        version: text(manifest, "version", "")?,
        release_tag: text(manifest, "tag", "")?,
        tranche_id: text(manifest, "tranche_id", "")?,
        source_commit: text(manifest, "source_commit", "")?,
        name: text(block, "name", "linux_appimage.")?,
        url: text(block, "url", "linux_appimage.")?,
        sha256: text(block, "sha256", "linux_appimage.")?.to_ascii_lowercase(),
        size_bytes,
        manifest_hash: Sha256::digest(&canonical).iter().map(|b| format!("{b:02x}")).collect(),
    })
}

/// The AppImage file this process was launched from (`$APPIMAGE`). Inside an AppImage the running
/// executable is a file in a throwaway mount; the file to replace is the `.AppImage` itself.
pub fn running_appimage_path(appimage_env: Option<PathBuf>, installed: &InstalledState) -> Result<PathBuf, String> {
    appimage_env.ok_or_else(|| {
        format!(
            "this process is not running from an AppImage ($APPIMAGE is unset), so {} cannot be replaced in place",
            installed.managed_executable_path.display()
        )
    })
}

/// Download, verify and stage-replace the running AppImage. `download` writes the artifact's bytes;
/// the transaction checks size and sha256 against the manifest before anything is replaced.
pub fn install_appimage_update<F>(
    config_dir: &Path,
    installed: &InstalledState,
    running_appimage: &Path,
    req: &AppImageInstallRequest,
    download: F,
) -> Result<RelaunchPrompt, String>
where
    F: FnOnce(&mut dyn Write) -> std::io::Result<u64>,
{
    if installed.install_kind != InstallKind::AppImage {
        return Err("this install is not an AppImage install".to_string());
    }
    validate_asset(&req.url, &req.name, ".AppImage")?;
    let running_sha256 = sha256_of_file(running_appimage)
        .map_err(|e| format!("cannot hash the running AppImage {}: {e}", running_appimage.display()))?;

    let outcome = execute_transaction(TransactionConfig {
        config_dir: config_dir.to_path_buf(),
        manifest: ManifestIdentity {
            schema_version: req.schema_version.clone(),
            channel: req.channel.clone(),
            version: req.version.clone(),
            release_tag: req.release_tag.clone(),
            tranche_id: req.tranche_id.clone(),
            source_commit: req.source_commit.clone(),
            artifact_sha256: req.sha256.clone(),
            manifest_hash: req.manifest_hash.clone(),
            artifact_name: req.name.clone(),
            artifact_size: req.size_bytes,
            eligibility_policy: EligibilityPolicy { update_eligible: true, ineligible_reason: None },
        },
        running_build: RunningBuildIdentity {
            managed_executable_path: running_appimage.to_path_buf(),
            channel: installed.channel.clone(),
            version: installed.version.clone(),
            release_tag: installed.release_tag.clone(),
            source_commit: installed.source_commit.clone(),
            artifact_sha256: running_sha256,
        },
        installed_state: Some(installed.clone()),
        download,
    });
    match outcome {
        TransactionOutcome::RelaunchPrompt(prompt) => Ok(prompt),
        TransactionOutcome::Aborted(abort) => Err(abort.reason),
    }
}

/// Run the AppImage self-update for the recorded install. `manifest` is the raw published manifest.
pub fn perform_appimage_install(config_dir: &Path, manifest: &Value) -> Result<RelaunchPrompt, String> {
    let record = config_update_dir(config_dir).join(INSTALLED_STATE_FILENAME);
    let bytes = fs::read(&record).map_err(|e| format!("no installed-state record at {}: {e}", record.display()))?;
    let installed: InstalledState =
        serde_json::from_slice(&bytes).map_err(|e| format!("installed-state.json is unreadable: {e}"))?;
    let running = running_appimage_path(std::env::var_os("APPIMAGE").map(PathBuf::from), &installed)?;
    let req = parse_appimage_install_request(manifest)?;
    install_appimage_update(config_dir, &installed, &running, &req, |writer| download_to_writer(&req.url, writer))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use std::path::PathBuf;

    const URL: &str = "https://github.com/electricm0nk/codex/releases/download/alpha-v0.16.141-abcdef12/Codex_0.16.141_amd64.AppImage";

    fn sha(bytes: &[u8]) -> String {
        use sha2::{Digest, Sha256};
        Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
    }

    fn manifest(payload: &[u8]) -> serde_json::Value {
        json!({
            "schema_version": "1.3.0",
            "channel": "alpha",
            "version": "0.16.141",
            "tag": "alpha/v0.16.141-abcdef12",
            "tranche_id": "STC-CODEX-SD-16",
            "source_commit": "a".repeat(40),
            "linux_appimage": {
                "name": "Codex_0.16.141_amd64.AppImage",
                "url": URL,
                "sha256": sha(payload),
                "size_bytes": payload.len(),
            },
        })
    }

    struct Fixture {
        config: PathBuf,
        managed: PathBuf,
        installed: InstalledState,
    }

    fn fixture(label: &str, old: &[u8]) -> Fixture {
        let dir = std::env::temp_dir().join(format!("codex-appimage-install-{label}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let managed = dir.join("Codex.AppImage");
        fs::write(&managed, old).unwrap();
        let installed = InstalledState {
            managed_executable_path: managed.clone(),
            install_kind: InstallKind::AppImage,
            channel: "alpha".into(),
            version: "0.16.140".into(),
            source_commit: "b".repeat(40),
            release_tag: "alpha/v0.16.140-bbbbbbbb".into(),
            manifest_hash: String::new(),
            artifact_sha256: sha(old),
            installed_at: "2026-10-01T00:00:00Z".into(),
            update_eligible: true,
            ineligible_reason: None,
        };
        Fixture { config: dir.join("config"), managed, installed }
    }

    fn serve(payload: &'static [u8]) -> impl FnOnce(&mut dyn std::io::Write) -> std::io::Result<u64> {
        move |writer| {
            writer.write_all(payload)?;
            Ok(payload.len() as u64)
        }
    }

    #[test]
    fn the_request_is_read_from_the_linux_appimage_block() {
        let req = parse_appimage_install_request(&manifest(b"new")).expect("parses");
        assert_eq!(req.version, "0.16.141");
        assert_eq!(req.name, "Codex_0.16.141_amd64.AppImage");
        assert_eq!(req.url, URL);
        assert_eq!(req.sha256, sha(b"new"));
        assert_eq!(req.size_bytes, 3);
        assert_eq!(req.release_tag, "alpha/v0.16.141-abcdef12");
        assert_eq!(req.manifest_hash.len(), 64, "the manifest's own hash is recorded");
        assert_eq!(parse_appimage_install_request(&manifest(b"new")).unwrap().manifest_hash, req.manifest_hash, "and is stable");
    }

    #[test]
    fn a_manifest_missing_what_the_install_needs_is_refused_by_name() {
        let mut no_block = manifest(b"new");
        no_block.as_object_mut().unwrap().remove("linux_appimage");
        assert!(parse_appimage_install_request(&no_block).unwrap_err().contains("linux_appimage"));
        for field in ["name", "url", "sha256", "size_bytes"] {
            let mut m = manifest(b"new");
            m["linux_appimage"].as_object_mut().unwrap().remove(field);
            assert!(parse_appimage_install_request(&m).unwrap_err().contains(field), "{field}");
        }
        for field in ["version", "tag", "channel", "source_commit"] {
            let mut m = manifest(b"new");
            m.as_object_mut().unwrap().remove(field);
            assert!(parse_appimage_install_request(&m).unwrap_err().contains(field), "{field}");
        }
    }

    #[test]
    fn the_download_replaces_the_running_appimage_and_keeps_the_previous_one() {
        let f = fixture("ok", b"old-appimage");
        let req = parse_appimage_install_request(&manifest(b"new-appimage")).unwrap();
        let prompt = install_appimage_update(&f.config, &f.installed, &f.managed, &req, serve(b"new-appimage")).expect("installs");
        assert_eq!(fs::read(&f.managed).unwrap(), b"new-appimage", "the managed file is the new build");
        assert_eq!(prompt.from_version, "0.16.140");
        assert_eq!(prompt.to_version, "0.16.141");
        assert_eq!(prompt.artifact_sha256, sha(b"new-appimage"));
        let update = super::super::transaction::config_update_dir(&f.config);
        assert_eq!(fs::read(update.join("backups/Codex.previous.AppImage")).unwrap(), b"old-appimage", "the previous build is the rollback copy");
        assert!(update.join("pending-update.json").is_file(), "the next launch has something to verify");
    }

    #[test]
    fn a_download_that_does_not_match_the_manifest_changes_nothing() {
        let f = fixture("bad-hash", b"old-appimage");
        let req = parse_appimage_install_request(&manifest(b"new-appimage")).unwrap();
        let err = install_appimage_update(&f.config, &f.installed, &f.managed, &req, serve(b"tampered-appx")).unwrap_err();
        assert!(err.contains("sha256") || err.contains("size"), "{err}");
        assert_eq!(fs::read(&f.managed).unwrap(), b"old-appimage", "the running build is untouched");
    }

    #[test]
    fn an_appimage_that_is_not_the_recorded_install_is_not_replaced() {
        let f = fixture("foreign", b"old-appimage");
        let elsewhere = f.managed.with_file_name("Other.AppImage");
        fs::write(&elsewhere, b"old-appimage").unwrap();
        let req = parse_appimage_install_request(&manifest(b"new-appimage")).unwrap();
        let err = install_appimage_update(&f.config, &f.installed, &elsewhere, &req, serve(b"new-appimage")).unwrap_err();
        assert!(err.contains("does not match"), "{err}");
        assert_eq!(fs::read(&f.managed).unwrap(), b"old-appimage");
        assert_eq!(fs::read(&elsewhere).unwrap(), b"old-appimage");
    }

    #[test]
    fn only_an_appimage_install_uses_this_path() {
        let mut f = fixture("kind", b"old-appimage");
        f.installed.install_kind = InstallKind::Deb;
        let req = parse_appimage_install_request(&manifest(b"new-appimage")).unwrap();
        let err = install_appimage_update(&f.config, &f.installed, &f.managed, &req, serve(b"new-appimage")).unwrap_err();
        assert!(err.contains("not an AppImage install"), "{err}");
    }

    #[test]
    fn a_url_outside_this_repositorys_releases_is_never_downloaded() {
        let f = fixture("url", b"old-appimage");
        let mut m = manifest(b"new-appimage");
        m["linux_appimage"]["url"] = json!("https://evil.example/Codex.AppImage");
        let req = parse_appimage_install_request(&m).unwrap();
        let err = install_appimage_update(&f.config, &f.installed, &f.managed, &req, serve(b"new-appimage")).unwrap_err();
        assert!(err.contains("not a codex release asset URL"), "{err}");
        assert_eq!(fs::read(&f.managed).unwrap(), b"old-appimage");
    }

    #[test]
    fn the_running_appimage_must_be_the_one_the_process_was_launched_from() {
        let f = fixture("env", b"old-appimage");
        assert!(running_appimage_path(None, &f.installed).unwrap_err().contains("not running from an AppImage"));
        assert_eq!(running_appimage_path(Some(f.managed.clone()), &f.installed).unwrap(), f.managed);
    }
}
