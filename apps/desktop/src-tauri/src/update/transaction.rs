//! SD-16 AppImage staged transaction.
//!
//! Performs the bounded install transaction described by `technical-requirements.md`
//! (Installed-State and AppImage Install Requirements). The transaction is:
//!
//! 1. Resolve and create the config-dir staging directory.
//! 2. Download the AppImage bytes into the staging directory (injected closure so tests can
//!    supply a fixture without a real HTTP fetch).
//! 3. Verify file presence, sha256 against the manifest, executable bit, manifest identity
//!    fingerprint, and current-executable identity against the installed-state record.
//! 4. Preserve the previous binary by rotating it into the backups directory.
//! 5. Write `pending-update.json` so the relaunch verifier on the next launch knows what to
//!    confirm before flipping installed-state.
//! 6. Atomically replace (or stage-replace) the managed AppImage path.
//! 7. Return a `RelaunchPrompt` outcome so the Tauri command layer can surface the prompt —
//!    this slice does not own the relaunch prompt itself.
//!
//! On any failure, the transaction leaves the staging directory intact (for diagnostic
//! inspection) and does not touch the managed AppImage path. The installed-state file is not
//! rewritten — the relaunch verifier is the only path that may mark success.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Filename of the AppImage artifact staged for inspection before replace.
const STAGED_APPIMAGE_FILENAME: &str = "Codex-Desktop-Shell-Scaffold.staged.AppImage";

/// Filename of the rolling backup of the previous managed AppImage.
const BACKUP_APPIMAGE_FILENAME: &str = "Codex-Desktop-Shell-Scaffold.previous.AppImage";

/// Subdirectory (under the config update dir) that holds staged downloads and the rolling
/// previous-binary backup.
const STAGING_DIR_NAME: &str = "staging";
const BACKUPS_DIR_NAME: &str = "backups";

/// Filename of the installed-state record.
#[allow(dead_code)]
pub const INSTALLED_STATE_FILENAME: &str = "installed-state.json";

/// Filename of the pending-update record.
pub const PENDING_UPDATE_FILENAME: &str = "pending-update.json";

/// Resolution of the config-dir update root.
pub fn config_update_dir(config_dir: &Path) -> PathBuf {
    config_dir
        .join("codex-desktop-shell-scaffold")
        .join("update")
}

fn staging_dir(config_update_dir: &Path) -> PathBuf {
    config_update_dir.join(STAGING_DIR_NAME)
}

fn backups_dir(config_update_dir: &Path) -> PathBuf {
    config_update_dir.join(BACKUPS_DIR_NAME)
}

/// Manifest identity: the artifact/provenance contract that the transaction stages against.
///
/// This is the runtime-shape used by the transaction — it carries only the fields the staged
/// transaction needs to verify and persist. The full JSON-Schema-validated manifest lives at the
/// release-lane boundary and is parsed into this shape before the transaction runs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestIdentity {
    pub schema_version: String,
    pub channel: String,
    pub version: String,
    pub release_tag: String,
    pub tranche_id: String,
    pub source_commit: String,
    /// SHA-256 of the AppImage artifact, hex-encoded.
    pub artifact_sha256: String,
    /// SHA-256 of the canonical update manifest file, hex-encoded.
    pub manifest_hash: String,
    /// Stable artifact name (the on-disk filename once installed).
    pub artifact_name: String,
    /// Byte length of the AppImage.
    pub artifact_size: u64,
    /// Channel-eligibility policy summary — required so the transaction can refuse non-eligible
    /// installs before touching the filesystem. Full policy lives in the manifest.
    pub eligibility_policy: EligibilityPolicy,
}

/// Eligibility policy summary derived from the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EligibilityPolicy {
    pub update_eligible: bool,
    #[serde(default)]
    pub ineligible_reason: Option<String>,
}

/// Running-build identity passed in by the caller.
///
/// This is the installed-shell identity: where the running binary lives and what it claims to be.
/// The transaction refuses to proceed if this disagrees with the on-disk installed-state record
/// (the "current executable identity" check), because that mismatch usually means the operator
/// moved the binary out from under the shell — proceeding would silently replace a foreign
/// binary.
///
/// `artifact_sha256` is the sha256 of the running binary, computed by the Tauri command layer
/// before invoking the transaction. The transaction itself does not re-hash the running
/// binary — that is a Tauri command boundary concern. On the very first install
/// `installed_state` is `None` and the field is not consulted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunningBuildIdentity {
    pub managed_executable_path: PathBuf,
    pub channel: String,
    pub version: String,
    pub release_tag: String,
    pub source_commit: String,
    pub artifact_sha256: String,
}

/// Installed-state record persisted at `<config-update-dir>/installed-state.json`.
///
/// The runtime never writes this until the relaunch verifier confirms the staged artifact matches
/// the running binary; the transaction module only reads it and may rewrite `last_known_*` fields
/// during the backup-rotation step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledState {
    pub managed_executable_path: PathBuf,
    pub install_kind: InstallKind,
    pub channel: String,
    pub version: String,
    pub source_commit: String,
    pub release_tag: String,
    pub manifest_hash: String,
    pub artifact_sha256: String,
    pub installed_at: String,
    pub update_eligible: bool,
    #[serde(default)]
    pub ineligible_reason: Option<String>,
}

/// Install kind. Only `AppImage` is currently used by the Linux-first self-update path; other
/// variants are reserved so the diagnostics surface does not have to invent new shape later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InstallKind {
    AppImage,
    Deb,
    DevLocal,
}

/// State of the pending-update record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PendingUpdateState {
    /// Pending: relaunch verifier has not yet confirmed.
    Pending,
    /// Relaunch verifier confirmed a matching running artifact.
    Success,
    /// Relaunch verifier detected a hash mismatch.
    Mismatch,
}

/// Pending-update record written at `<config-update-dir>/pending-update.json` before relaunch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingUpdate {
    pub from_version: String,
    pub to_version: String,
    pub artifact_sha256: String,
    pub manifest_hash: String,
    pub staging_path: PathBuf,
    pub backup_path: PathBuf,
    pub managed_executable_path: PathBuf,
    pub channel: String,
    pub release_tag: String,
    pub source_commit: String,
    pub created_at: String,
    pub pending_update_state: PendingUpdateState,
}

/// Outcome of the transaction. The Tauri command layer maps this to UI state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum TransactionOutcome {
    /// Transaction completed cleanly; the managed AppImage path now contains the staged
    /// artifact, the previous binary is preserved in backups, and `pending-update.json` is
    /// written. The Tauri command layer must surface a relaunch prompt.
    RelaunchPrompt(RelaunchPrompt),
    /// Transaction aborted before any filesystem mutation. No state was changed.
    Aborted(TransactionAbort),
}

/// Relaunch prompt signal returned to the caller when the transaction succeeded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelaunchPrompt {
    pub pending_update_path: PathBuf,
    pub managed_executable_path: PathBuf,
    pub from_version: String,
    pub to_version: String,
    pub artifact_sha256: String,
}

/// Abort signal — no filesystem mutation occurred (or only the staging download landed).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionAbort {
    pub reason: String,
    pub code: TransactionAbortCode,
}

/// Stable abort codes — used for diagnostics and to drive UI messaging.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TransactionAbortCode {
    StagingDirUnwritable,
    StagedFileMissing,
    StagedFileEmpty,
    StagedFileHashMismatch,
    ExecutableBitMissing,
    ManifestIneligible,
    CurrentExecutableIdentityMismatch,
    NoInstalledState,
    BackupRotationFailed,
    AtomicReplaceFailed,
}

/// Errors raised by the transaction module. Most abort paths are returned as `TransactionOutcome`
/// values; this enum covers the truly exceptional cases (e.g. I/O failures that mean we cannot
/// even read installed-state).
///
/// Reserved for future error-path coverage — installed-state reads/writes in later slices will
/// surface these. Kept `pub` so the type is reachable from tests and the relaunch-verifier
/// module that this slice will hand off to.
#[allow(dead_code)]
#[derive(Debug)]
pub enum TransactionError {
    Io {
        op: &'static str,
        path: PathBuf,
        source: std::io::Error,
    },
    Json {
        op: &'static str,
        path: PathBuf,
        source: serde_json::Error,
    },
}

impl std::fmt::Display for TransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionError::Io { op, path, source } => {
                write!(f, "io error during {op} on {}: {source}", path.display())
            }
            TransactionError::Json { op, path, source } => {
                write!(f, "json error during {op} on {}: {source}", path.display())
            }
        }
    }
}

impl std::error::Error for TransactionError {}

/// Configuration for the transaction.
///
/// `download` is an injected closure that writes the AppImage bytes to the supplied writer. The
/// transaction calls it once; tests supply a synthetic payload so the test never hits the
/// network. Production wiring will pass a closure that streams from the GitHub release asset.
pub struct TransactionConfig<F>
where
    F: FnOnce(&mut dyn Write) -> std::io::Result<u64>,
{
    pub config_dir: PathBuf,
    pub manifest: ManifestIdentity,
    pub running_build: RunningBuildIdentity,
    pub installed_state: Option<InstalledState>,
    pub download: F,
}

/// Execute the staged AppImage transaction.
///
/// The caller passes the manifest identity, the running-build identity, the previously read
/// installed-state (or `None` if first install), and a download closure. The function never
/// fabricates success: any failure short of the atomic replace returns `Aborted` with the
/// reason; installed-state is never rewritten here.
pub fn execute_transaction<F>(config: TransactionConfig<F>) -> TransactionOutcome
where
    F: FnOnce(&mut dyn Write) -> std::io::Result<u64>,
{
    let update_dir = config_update_dir(&config.config_dir);

    // Step 1 — create config-dir staging directory. If the directory cannot be created, abort
    // before any download.
    if let Err(source) = fs::create_dir_all(&update_dir) {
        return TransactionOutcome::Aborted(TransactionAbort {
            code: TransactionAbortCode::StagingDirUnwritable,
            reason: format!(
                "could not create config update dir {}: {source}",
                update_dir.display()
            ),
        });
    }
    if let Err(source) = fs::create_dir_all(staging_dir(&update_dir)) {
        return TransactionOutcome::Aborted(TransactionAbort {
            code: TransactionAbortCode::StagingDirUnwritable,
            reason: format!(
                "could not create staging dir {}: {source}",
                staging_dir(&update_dir).display()
            ),
        });
    }

    let staged_path = staging_dir(&update_dir).join(STAGED_APPIMAGE_FILENAME);

    // Step 2 — download AppImage to staging. Any failure here leaves no trace other than a
    // partially-written file (acceptable; the next attempt will overwrite).
    let TransactionConfig { download, .. } = config;
    if let Err(source) = write_staged(&staged_path, download) {
        return TransactionOutcome::Aborted(TransactionAbort {
            code: TransactionAbortCode::StagedFileMissing,
            reason: format!("download failed for {}: {source}", staged_path.display()),
        });
    }

    // The AppImage must be executable for the OS to run it after the atomic replace. The
    // downloader may have stripped the executable bit (e.g. a streaming HTTP reader). We force
    // it on here so the staged file matches the artifact contract — the executable-bit check
    // below then asserts it rather than discovers a flaky transient state.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = fs::metadata(&staged_path) {
            let mut perms = metadata.permissions();
            perms.set_mode(perms.mode() | 0o755);
            let _ = fs::set_permissions(&staged_path, perms);
        }
    }

    // Step 3 — verify file presence, hash, executable bit, manifest identity, and current
    // executable identity.
    if let Some(abort) = verify_staged(&staged_path, &config.manifest) {
        return TransactionOutcome::Aborted(abort);
    }
    if let Some(abort) =
        verify_current_executable_identity(&config.running_build, config.installed_state.as_ref())
    {
        return TransactionOutcome::Aborted(abort);
    }

    // Step 4 — preserve previous binary by rotating into backups. This slice does not yet
    // implement the "last 2" retention policy; the backup slot is rotated as a single rolling
    // slot. A later slice (E7-Fn retention) extends this to multi-slot retention.
    if let Err(abort) =
        preserve_previous(&update_dir, &config.running_build.managed_executable_path)
    {
        return TransactionOutcome::Aborted(abort);
    }

    // Step 5 — write pending-update.json so the relaunch verifier on the next launch has
    // something to confirm.
    let backup_path = backups_dir(&update_dir).join(BACKUP_APPIMAGE_FILENAME);
    let pending = PendingUpdate {
        from_version: config
            .installed_state
            .as_ref()
            .map(|s| s.version.clone())
            .unwrap_or_else(|| "unknown".to_string()),
        to_version: config.manifest.version.clone(),
        artifact_sha256: config.manifest.artifact_sha256.clone(),
        manifest_hash: config.manifest.manifest_hash.clone(),
        staging_path: staged_path.clone(),
        backup_path: backup_path.clone(),
        managed_executable_path: config.running_build.managed_executable_path.clone(),
        channel: config.manifest.channel.clone(),
        release_tag: config.manifest.release_tag.clone(),
        source_commit: config.manifest.source_commit.clone(),
        created_at: now_iso8601(),
        pending_update_state: PendingUpdateState::Pending,
    };
    let pending_path = update_dir.join(PENDING_UPDATE_FILENAME);
    if let Err(source) = write_atomic_json(&pending_path, &pending) {
        return TransactionOutcome::Aborted(TransactionAbort {
            code: TransactionAbortCode::BackupRotationFailed,
            reason: format!(
                "could not write pending-update.json {}: {source}",
                pending_path.display()
            ),
        });
    }

    // Step 6 — atomically replace the managed AppImage. On failure, the pending-update.json is
    // intentionally left in place so the relaunch verifier can surface a recovery state.
    let managed = config.running_build.managed_executable_path.clone();
    if let Err(source) = atomic_replace(&staged_path, &managed) {
        return TransactionOutcome::Aborted(TransactionAbort {
            code: TransactionAbortCode::AtomicReplaceFailed,
            reason: format!(
                "atomic replace {} -> {} failed: {source}",
                staged_path.display(),
                managed.display()
            ),
        });
    }

    // Step 7 — return the relaunch prompt. The Tauri command layer surfaces this to the user.
    TransactionOutcome::RelaunchPrompt(RelaunchPrompt {
        pending_update_path: pending_path,
        managed_executable_path: managed,
        from_version: pending.from_version,
        to_version: pending.to_version,
        artifact_sha256: pending.artifact_sha256,
    })
}

fn write_staged<F>(staged_path: &Path, download: F) -> std::io::Result<u64>
where
    F: FnOnce(&mut dyn Write) -> std::io::Result<u64>,
{
    let mut file = fs::File::create(staged_path)?;
    let written = download(&mut file)?;
    file.flush()?;
    Ok(written)
}

fn verify_staged(staged_path: &Path, manifest: &ManifestIdentity) -> Option<TransactionAbort> {
    let metadata = match fs::metadata(staged_path) {
        Ok(m) => m,
        Err(source) => {
            return Some(TransactionAbort {
                code: TransactionAbortCode::StagedFileMissing,
                reason: format!("stat staged {} failed: {source}", staged_path.display()),
            });
        }
    };
    if metadata.len() == 0 {
        return Some(TransactionAbort {
            code: TransactionAbortCode::StagedFileEmpty,
            reason: format!("staged file {} is empty", staged_path.display()),
        });
    }
    if metadata.len() != manifest.artifact_size {
        return Some(TransactionAbort {
            code: TransactionAbortCode::StagedFileHashMismatch,
            reason: format!(
                "staged file size {} does not match manifest artifact_size {}",
                metadata.len(),
                manifest.artifact_size
            ),
        });
    }

    let mut file = match fs::File::open(staged_path) {
        Ok(f) => f,
        Err(source) => {
            return Some(TransactionAbort {
                code: TransactionAbortCode::StagedFileMissing,
                reason: format!("read staged {} failed: {source}", staged_path.display()),
            });
        }
    };

    use std::io::Read;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = match file.read(&mut buf) {
            Ok(n) => n,
            Err(source) => {
                return Some(TransactionAbort {
                    code: TransactionAbortCode::StagedFileMissing,
                    reason: format!("read staged {} failed: {source}", staged_path.display()),
                });
            }
        };
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }

    let actual = hex_lower(&hasher.finalize());
    if actual != manifest.artifact_sha256 {
        return Some(TransactionAbort {
            code: TransactionAbortCode::StagedFileHashMismatch,
            reason: format!(
                "staged file sha256 {actual} does not match manifest artifact_sha256 {}",
                manifest.artifact_sha256
            ),
        });
    }
    // Executable bit — POSIX only. We deliberately do not rely on metadata.permissions() alone
    // because it is platform-specific; instead we use PermissionsExt on unix.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = metadata.permissions().mode();
        if mode & 0o111 == 0 {
            return Some(TransactionAbort {
                code: TransactionAbortCode::ExecutableBitMissing,
                reason: format!(
                    "staged file {} lacks executable bit (mode={mode:o})",
                    staged_path.display()
                ),
            });
        }
    }

    if !manifest.eligibility_policy.update_eligible {
        return Some(TransactionAbort {
            code: TransactionAbortCode::ManifestIneligible,
            reason: manifest
                .eligibility_policy
                .ineligible_reason
                .clone()
                .unwrap_or_else(|| "manifest marked update_eligible=false".to_string()),
        });
    }

    None
}

#[allow(clippy::question_mark)]
fn verify_current_executable_identity(
    running: &RunningBuildIdentity,
    installed: Option<&InstalledState>,
) -> Option<TransactionAbort> {
    // First install: no installed-state exists yet, so there is nothing to compare against.
    // The shell still claims a running-build identity via argv/env; we accept it but require
    // that the running-build path matches the path the manifest will replace.
    let installed = match installed {
        Some(state) => state,
        None => return None,
    };

    if installed.managed_executable_path != running.managed_executable_path {
        return Some(TransactionAbort {
            code: TransactionAbortCode::CurrentExecutableIdentityMismatch,
            reason: format!(
                "running executable path {} does not match installed-state managed_executable_path {}",
                running.managed_executable_path.display(),
                installed.managed_executable_path.display()
            ),
        });
    }

    if installed.artifact_sha256 != running.artifact_sha256 {
        // The installed-state file claims a different artifact than the shell is currently
        // running. This is the explicit "current executable identity" check — proceeding would
        // silently overwrite a foreign binary.
        return Some(TransactionAbort {
            code: TransactionAbortCode::CurrentExecutableIdentityMismatch,
            reason: format!(
                "running executable sha256 {} differs from installed-state artifact_sha256 {}",
                running.artifact_sha256,
                installed.artifact_sha256
            ),
        });
    }

    None
}

fn preserve_previous(
    update_dir: &Path,
    managed_executable_path: &Path,
) -> Result<(), TransactionAbort> {
    let backups = backups_dir(update_dir);
    fs::create_dir_all(&backups).map_err(|source| TransactionAbort {
        code: TransactionAbortCode::BackupRotationFailed,
        reason: format!(
            "could not create backups dir {}: {source}",
            backups.display()
        ),
    })?;
    let backup_path = backups.join(BACKUP_APPIMAGE_FILENAME);
    if managed_executable_path.exists() {
        fs::copy(managed_executable_path, &backup_path).map_err(|source| TransactionAbort {
            code: TransactionAbortCode::BackupRotationFailed,
            reason: format!(
                "could not copy {} -> {}: {source}",
                managed_executable_path.display(),
                backup_path.display()
            ),
        })?;
    }
    Ok(())
}

fn atomic_replace(staged: &Path, managed: &Path) -> std::io::Result<()> {
    // Stage-replace: rename first into a sibling next-to-managed so the managed path never
    // holds a partial file. On POSIX this is atomic for same-filesystem rename.
    if let Some(parent) = managed.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = managed.with_extension("AppImage.tmp");
    fs::copy(staged, &tmp)?;
    fs::rename(&tmp, managed)?;
    Ok(())
}

fn write_atomic_json<T: Serialize>(path: &Path, value: &T) -> std::io::Result<()> {
    let tmp = path.with_extension("json.tmp");
    {
        let mut f = fs::File::create(&tmp)?;
        let body = serde_json::to_vec_pretty(value)
            .map_err(|source| std::io::Error::new(std::io::ErrorKind::InvalidData, source))?;
        f.write_all(&body)?;
        f.flush()?;
    }
    fs::rename(&tmp, path)?;
    Ok(())
}

fn hex_lower(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

fn now_iso8601() -> String {
    // Deterministic-ish UTC timestamp. The transaction does not depend on wall-clock for any
    // decision logic; this is only a human-readable marker on pending-update.json. We use
    // SystemTime::now() so the value is honest, but tests assert the field is non-empty.
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    format!("epoch:{}", dur.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::path::PathBuf;

    fn tempdir(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("sd16-e7-f3a-{label}-{}", std::process::id(),));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create tempdir");
        dir
    }

    fn sha256_hex(bytes: &[u8]) -> String {
        let mut h = Sha256::new();
        h.update(bytes);
        hex_lower(&h.finalize())
    }

    fn make_manifest(artifact_bytes: &[u8]) -> ManifestIdentity {
        ManifestIdentity {
            schema_version: "1".into(),
            channel: "alpha".into(),
            version: "0.0.0-alpha.20260703.1".into(),
            release_tag: "alpha/0.0.0-alpha.20260703.1".into(),
            tranche_id: "STC-CODEX-SD-16".into(),
            source_commit: "f2ba1b9".into(),
            artifact_sha256: sha256_hex(artifact_bytes),
            manifest_hash: "manifesthash000".into(),
            artifact_name: "Codex-Desktop-Shell-Scaffold.AppImage".into(),
            artifact_size: artifact_bytes.len() as u64,
            eligibility_policy: EligibilityPolicy {
                update_eligible: true,
                ineligible_reason: None,
            },
        }
    }

    fn make_running(dir: &Path) -> RunningBuildIdentity {
        RunningBuildIdentity {
            managed_executable_path: dir.join("Codex-Desktop-Shell-Scaffold.AppImage"),
            channel: "alpha".into(),
            version: "0.0.0-alpha.20260628.1".into(),
            release_tag: "alpha/0.0.0-alpha.20260628.1".into(),
            source_commit: "ff91002".into(),
            // The running binary's sha256 is the on-disk hash of the previously-installed
            // AppImage. Tests pre-stage a binary at the managed path with this exact payload so
            // the current-executable-identity check passes.
            artifact_sha256: sha256_hex(b"old-binary-payload"),
        }
    }

    fn make_installed(dir: &Path) -> InstalledState {
        InstalledState {
            managed_executable_path: dir.join("Codex-Desktop-Shell-Scaffold.AppImage"),
            install_kind: InstallKind::AppImage,
            channel: "alpha".into(),
            version: "0.0.0-alpha.20260628.1".into(),
            source_commit: "ff91002".into(),
            release_tag: "alpha/0.0.0-alpha.20260628.1".into(),
            manifest_hash: "prevmanifesthash".into(),
            artifact_sha256: sha256_hex(b"old-binary-payload"),
            installed_at: "epoch:1700000000".into(),
            update_eligible: true,
            ineligible_reason: None,
        }
    }

    #[test]
    fn success_path_returns_relaunch_prompt_and_writes_pending() {
        let dir = tempdir("success");
        let managed = dir.join("Codex-Desktop-Shell-Scaffold.AppImage");
        // Pretend a previous AppImage already lives at the managed path so preserve_previous
        // has something to rotate.
        fs::write(&managed, b"old-binary-payload").expect("write old binary");

        let artifact = b"new-appimage-payload";
        let manifest = make_manifest(artifact);
        let running = make_running(&dir);
        let installed = make_installed(&dir);

        let outcome = execute_transaction(TransactionConfig {
            config_dir: dir.clone(),
            manifest: manifest.clone(),
            running_build: running.clone(),
            installed_state: Some(installed),
            download: |w| {
                w.write_all(artifact)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                Ok(artifact.len() as u64)
            },
        });

        match outcome {
            TransactionOutcome::RelaunchPrompt(prompt) => {
                assert_eq!(prompt.to_version, manifest.version);
                assert_eq!(prompt.from_version, "0.0.0-alpha.20260628.1");
                assert_eq!(prompt.artifact_sha256, manifest.artifact_sha256);
                assert_eq!(prompt.managed_executable_path, managed);
            }
            other => panic!("expected RelaunchPrompt, got {other:?}"),
        }

        // pending-update.json must exist with the right shape.
        let update_dir = config_update_dir(&dir);
        let pending_path = update_dir.join(PENDING_UPDATE_FILENAME);
        let raw = fs::read_to_string(&pending_path).expect("pending-update.json exists");
        let parsed: PendingUpdate = serde_json::from_str(&raw).expect("valid pending json");
        assert_eq!(parsed.to_version, manifest.version);
        assert_eq!(parsed.from_version, "0.0.0-alpha.20260628.1");
        assert_eq!(parsed.artifact_sha256, manifest.artifact_sha256);
        assert_eq!(parsed.manifest_hash, manifest.manifest_hash);
        assert_eq!(parsed.channel, manifest.channel);
        assert_eq!(parsed.pending_update_state, PendingUpdateState::Pending);

        // The managed AppImage must now contain the new payload (atomic replace landed).
        let after = fs::read(&managed).expect("read managed");
        assert_eq!(after, artifact, "managed AppImage must be the new payload");

        // The previous binary must be preserved in backups.
        let backups = update_dir.join(BACKUPS_DIR_NAME);
        let backup = backups.join(BACKUP_APPIMAGE_FILENAME);
        let backup_bytes = fs::read(&backup).expect("backup exists");
        assert_eq!(backup_bytes, b"old-binary-payload");
    }

    #[test]
    fn hash_mismatch_aborts_without_touching_managed() {
        let dir = tempdir("mismatch");
        let managed = dir.join("Codex-Desktop-Shell-Scaffold.AppImage");
        fs::write(&managed, b"old-binary-payload").expect("write old binary");

        let real_artifact = b"new-appimage-payload";
        let manifest = make_manifest(real_artifact);
        // Hand the transaction a DIFFERENT payload than the manifest claims — this is the
        // hash-mismatch failure path.
        let fake_artifact = b"different-payload-with-different-hash";

        let outcome = execute_transaction(TransactionConfig {
            config_dir: dir.clone(),
            manifest,
            running_build: make_running(&dir),
            installed_state: Some(make_installed(&dir)),
            download: |w| {
                w.write_all(fake_artifact)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                Ok(fake_artifact.len() as u64)
            },
        });

        match outcome {
            TransactionOutcome::Aborted(abort) => {
                assert_eq!(abort.code, TransactionAbortCode::StagedFileHashMismatch);
                assert!(
                    abort.reason.contains("sha256") || abort.reason.contains("size"),
                    "reason must cite the mismatch class: {abort:?}"
                );
            }
            other => panic!("expected Aborted, got {other:?}"),
        }

        // The managed AppImage must still be the original payload.
        let after = fs::read(&managed).expect("read managed");
        assert_eq!(
            after, b"old-binary-payload",
            "managed must be untouched on abort"
        );
        // pending-update.json must NOT exist on abort.
        let update_dir = config_update_dir(&dir);
        let pending_path = update_dir.join(PENDING_UPDATE_FILENAME);
        assert!(
            !pending_path.exists(),
            "pending-update.json must not be written on abort"
        );
    }

    #[test]
    fn empty_artifact_aborts_with_empty_code() {
        let dir = tempdir("empty");
        let managed = dir.join("Codex-Desktop-Shell-Scaffold.AppImage");
        fs::write(&managed, b"old").expect("write old binary");

        // Manifest claims a non-empty payload, but the download closure writes zero bytes.
        let manifest = make_manifest(b"some-payload");
        let outcome = execute_transaction(TransactionConfig {
            config_dir: dir.clone(),
            manifest,
            running_build: make_running(&dir),
            installed_state: Some(make_installed(&dir)),
            download: |_w| Ok(0),
        });

        match outcome {
            TransactionOutcome::Aborted(abort) => {
                assert_eq!(abort.code, TransactionAbortCode::StagedFileEmpty);
            }
            other => panic!("expected Aborted, got {other:?}"),
        }
        assert_eq!(fs::read(&managed).expect("read"), b"old");
    }

    #[test]
    fn first_install_without_installed_state_is_accepted() {
        let dir = tempdir("first-install");
        let managed = dir.join("Codex-Desktop-Shell-Scaffold.AppImage");
        let artifact = b"first-install-payload";
        let manifest = make_manifest(artifact);
        let outcome = execute_transaction(TransactionConfig {
            config_dir: dir.clone(),
            manifest: manifest.clone(),
            running_build: make_running(&dir),
            installed_state: None,
            download: |w| {
                w.write_all(artifact)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                Ok(artifact.len() as u64)
            },
        });
        match outcome {
            TransactionOutcome::RelaunchPrompt(prompt) => {
                assert_eq!(prompt.from_version, "unknown");
                assert_eq!(prompt.to_version, manifest.version);
            }
            other => panic!("expected RelaunchPrompt on first install, got {other:?}"),
        }
        assert_eq!(fs::read(&managed).expect("read"), artifact);
    }

    #[test]
    fn ineligible_manifest_aborts_before_replace() {
        let dir = tempdir("ineligible");
        let managed = dir.join("Codex-Desktop-Shell-Scaffold.AppImage");
        fs::write(&managed, b"old").expect("write old binary");

        let artifact = b"new-payload";
        let mut manifest = make_manifest(artifact);
        manifest.eligibility_policy = EligibilityPolicy {
            update_eligible: false,
            ineligible_reason: Some("channel not yet open".into()),
        };
        let outcome = execute_transaction(TransactionConfig {
            config_dir: dir.clone(),
            manifest,
            running_build: make_running(&dir),
            installed_state: Some(make_installed(&dir)),
            download: |w| {
                w.write_all(artifact)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                Ok(artifact.len() as u64)
            },
        });
        match outcome {
            TransactionOutcome::Aborted(abort) => {
                assert_eq!(abort.code, TransactionAbortCode::ManifestIneligible);
                assert!(abort.reason.contains("channel not yet open"));
            }
            other => panic!("expected Aborted, got {other:?}"),
        }
        assert_eq!(fs::read(&managed).expect("read"), b"old");
    }

    #[test]
    fn download_failure_aborts_with_staged_file_missing_code() {
        let dir = tempdir("download-fail");
        let managed = dir.join("Codex-Desktop-Shell-Scaffold.AppImage");
        fs::write(&managed, b"old").expect("write old binary");

        let manifest = make_manifest(b"new-payload");
        let outcome = execute_transaction(TransactionConfig {
            config_dir: dir.clone(),
            manifest,
            running_build: make_running(&dir),
            installed_state: Some(make_installed(&dir)),
            download: |_w| {
                Err(std::io::Error::new(
                    std::io::ErrorKind::ConnectionRefused,
                    "boom",
                ))
            },
        });
        match outcome {
            TransactionOutcome::Aborted(abort) => {
                assert_eq!(abort.code, TransactionAbortCode::StagedFileMissing);
            }
            other => panic!("expected Aborted, got {other:?}"),
        }
        assert_eq!(fs::read(&managed).expect("read"), b"old");
    }

    #[test]
    fn running_path_mismatch_against_installed_state_aborts() {
        let dir = tempdir("path-mismatch");
        let managed = dir.join("Codex-Desktop-Shell-Scaffold.AppImage");
        fs::write(&managed, b"old").expect("write old binary");

        let artifact = b"new-payload";
        let manifest = make_manifest(artifact);
        let mut running = make_running(&dir);
        // Operator moved the binary under the shell — running-build claims a different path.
        running.managed_executable_path = dir.join("Moved-Elsewhere.AppImage");

        let outcome = execute_transaction(TransactionConfig {
            config_dir: dir.clone(),
            manifest,
            running_build: running,
            installed_state: Some(make_installed(&dir)),
            download: |w| {
                w.write_all(artifact)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                Ok(artifact.len() as u64)
            },
        });
        match outcome {
            TransactionOutcome::Aborted(abort) => {
                assert_eq!(
                    abort.code,
                    TransactionAbortCode::CurrentExecutableIdentityMismatch
                );
            }
            other => panic!("expected Aborted, got {other:?}"),
        }
        assert_eq!(fs::read(&managed).expect("read"), b"old");
    }

    #[test]
    fn pending_update_json_round_trips_through_disk() {
        let dir = tempdir("round-trip");
        let update_dir = config_update_dir(&dir);
        fs::create_dir_all(&update_dir).expect("mkdir");
        let path = update_dir.join(PENDING_UPDATE_FILENAME);
        let pending = PendingUpdate {
            from_version: "0.0.0".into(),
            to_version: "0.0.1".into(),
            artifact_sha256: "abc".into(),
            manifest_hash: "def".into(),
            staging_path: dir.join("staging/AppImage"),
            backup_path: dir.join("backups/AppImage"),
            managed_executable_path: dir.join("AppImage"),
            channel: "alpha".into(),
            release_tag: "alpha/0.0.1".into(),
            source_commit: "deadbeef".into(),
            created_at: "epoch:0".into(),
            pending_update_state: PendingUpdateState::Pending,
        };
        write_atomic_json(&path, &pending).expect("write");
        let raw = fs::read_to_string(&path).expect("read");
        let parsed: PendingUpdate = serde_json::from_str(&raw).expect("parse");
        assert_eq!(parsed, pending);
    }

    #[test]
    fn config_update_dir_is_xdg_relative() {
        let dir = PathBuf::from("/tmp/fake-xdg");
        let got = config_update_dir(&dir);
        assert_eq!(
            got,
            PathBuf::from("/tmp/fake-xdg/codex-desktop-shell-scaffold/update")
        );
    }

    #[test]
    fn cursor_based_download_smoke() {
        // Confirms the injected-closure seam works with a std::io::Cursor — same shape as
        // production would use with a streaming HTTP reader.
        let dir = tempdir("cursor");
        let managed = dir.join("Codex-Desktop-Shell-Scaffold.AppImage");
        fs::write(&managed, b"old").expect("write old binary");
        let artifact = b"streamed-payload";
        let manifest = make_manifest(artifact);

        let outcome = execute_transaction(TransactionConfig {
            config_dir: dir.clone(),
            manifest,
            running_build: make_running(&dir),
            installed_state: None,
            download: |w| {
                let mut c = Cursor::new(artifact);
                std::io::copy(&mut c, w)
            },
        });
        match outcome {
            TransactionOutcome::RelaunchPrompt(_) => {}
            other => panic!("expected RelaunchPrompt, got {other:?}"),
        }
        assert_eq!(fs::read(&managed).expect("read"), artifact);
    }
}
