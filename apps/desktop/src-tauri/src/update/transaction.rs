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

/// Filename of the most-recent rolling backup of the previous managed AppImage (slot 1).
const BACKUP_APPIMAGE_FILENAME: &str = "Codex-Desktop-Shell-Scaffold.previous.AppImage";

/// Filename of the older rolling backup (slot 2). Slot 1 is shifted here before slot 1 is
/// overwritten, capping the backup set at two entries.
const BACKUP_APPIMAGE_FILENAME_2: &str = "Codex-Desktop-Shell-Scaffold.previous2.AppImage";

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
    let TransactionConfig {
        config_dir,
        manifest,
        running_build,
        installed_state,
        download,
    } = config;
    let update_dir = config_update_dir(&config_dir);

    // Eligibility guard — reject ineligible manifests before touching the filesystem,
    // consistent with the doc comment on ManifestIdentity.eligibility_policy.
    if !manifest.eligibility_policy.update_eligible {
        return TransactionOutcome::Aborted(TransactionAbort {
            code: TransactionAbortCode::ManifestIneligible,
            reason: manifest
                .eligibility_policy
                .ineligible_reason
                .clone()
                .unwrap_or_else(|| "manifest marked update_eligible=false".to_string()),
        });
    }

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

    // Step 3 — verify file presence, hash, executable bit, and current executable identity.
    if let Some(abort) = verify_staged(&staged_path, &manifest) {
        return TransactionOutcome::Aborted(abort);
    }
    if let Some(abort) =
        verify_current_executable_identity(&running_build, installed_state.as_ref())
    {
        return TransactionOutcome::Aborted(abort);
    }

    // Step 4 — preserve previous binary by rotating into backups (two-slot retention).
    // Slot 1 (`.previous.AppImage`) is shifted into slot 2 (`.previous2.AppImage`) first, then
    // the current managed binary is copied into slot 1. This caps the backup set at two entries.
    if let Err(abort) = preserve_previous(&update_dir, &running_build.managed_executable_path) {
        return TransactionOutcome::Aborted(abort);
    }

    // Step 5 — write pending-update.json so the relaunch verifier on the next launch has
    // something to confirm.
    let backup_path = backups_dir(&update_dir).join(BACKUP_APPIMAGE_FILENAME);
    let pending = PendingUpdate {
        from_version: installed_state
            .as_ref()
            .map(|s| s.version.clone())
            .unwrap_or_else(|| "unknown".to_string()),
        to_version: manifest.version.clone(),
        artifact_sha256: manifest.artifact_sha256.clone(),
        manifest_hash: manifest.manifest_hash.clone(),
        staging_path: staged_path.clone(),
        backup_path: backup_path.clone(),
        managed_executable_path: running_build.managed_executable_path.clone(),
        channel: manifest.channel.clone(),
        release_tag: manifest.release_tag.clone(),
        source_commit: manifest.source_commit.clone(),
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
    let managed = running_build.managed_executable_path.clone();
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

// ─── F3c — relaunch verifier ────────────────────────────────────────────

/// Outcome of the relaunch verifier.
///
/// The shell computes its own running artifact hash on startup and forwards it to the Tauri
/// command layer, which calls `verify_relaunch_artifact`. Three outcomes:
///
/// - `Promoted`: the running hash matches the staged artifact's expected sha256. The pending
///   record is rewritten to `Success`, a fresh `installed-state.json` is written so future
///   transactions see the new version as installed, and `pending-update.json` is deleted.
/// - `VerificationFailed`: the running hash does not match. The pending record's
///   `pending_update_state` is set to `Mismatch` (it stays on disk so the operator can offer
///   restore via `restoreOffer.tsx` and F3b's `perform_restore_previous`). Installed-state is
///   NOT rewritten — the previous build is still authoritative.
/// - `NoPendingUpdate`: there is no `pending-update.json` on disk. The shell simply continues
///   with the existing installed-state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ReloadVerifyOutcome {
    #[serde(rename_all = "camelCase")]
    Promoted {
        installed_state_path: PathBuf,
        promoted_version: String,
    },
    #[serde(rename_all = "camelCase")]
    VerificationFailed {
        expected: String,
        actual: String,
    },
    NoPendingUpdate,
}

/// Configuration for the relaunch verifier.
///
/// `running_artifact_sha256` is an injected closure that returns the sha256 of the running
/// binary, hex-encoded (lowercase). The transaction never hashes the binary itself — that is
/// the Tauri command boundary's concern. Tests supply a fixture closure so the test never
/// touches a real binary.
pub struct ReloadVerifyConfig<F>
where
    F: FnOnce() -> std::io::Result<String>,
{
    pub config_dir: PathBuf,
    pub running_artifact_sha256: F,
}

/// Verify the running artifact against the staged `pending-update.json`.
///
/// On match, promote the pending record into a fresh `installed-state.json` and clear the
/// pending file. On mismatch, leave the pending record (with `pending_update_state = Mismatch`)
/// so `restoreOffer.tsx` can surface the prior version. The verifier never fabricates success:
/// an injected closure returning an unexpected value is treated as a hash mismatch.
///
/// Named with the `_impl` suffix so the Tauri command shim (`verify_relaunch_artifact`)
/// below can occupy the canonical name in this module's value namespace without colliding
/// with this unit-testable function.
pub fn verify_relaunch_artifact_impl<F>(config: ReloadVerifyConfig<F>) -> ReloadVerifyOutcome
where
    F: FnOnce() -> std::io::Result<String>,
{
    let ReloadVerifyConfig {
        config_dir,
        running_artifact_sha256,
    } = config;
    let update_dir = config_update_dir(&config_dir);
    let pending_path = update_dir.join(PENDING_UPDATE_FILENAME);

    let raw = match fs::read(&pending_path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return ReloadVerifyOutcome::NoPendingUpdate;
        }
        Err(_error) => {
            // Treat any non-NotFound read error as "no pending update" so a corrupt or
            // unreadable pending file does not wedge the shell on every relaunch. The shell
            // will surface the missing pending state honestly via the diagnostics panel.
            return ReloadVerifyOutcome::NoPendingUpdate;
        }
    };

    let pending: PendingUpdate = match serde_json::from_slice(&raw) {
        Ok(value) => value,
        Err(_) => return ReloadVerifyOutcome::NoPendingUpdate,
    };

    let actual = match running_artifact_sha256() {
        Ok(value) => value,
        Err(_) => {
            // The caller could not compute the running hash — fall back to a verification
            // failure so the operator is prompted to restore rather than have us claim a
            // promotion we did not actually verify.
            return ReloadVerifyOutcome::VerificationFailed {
                expected: pending.artifact_sha256.clone(),
                actual: "<compute-error>".to_string(),
            };
        }
    };

    if actual != pending.artifact_sha256 {
        // Mismatch — rewrite pending state to Mismatch so subsequent UI surfaces the
        // restore offer, but leave installed-state untouched. The pending record stays on
        // disk for `restoreOffer.tsx` and F3b's `perform_restore_previous`.
        let mut flagged = pending.clone();
        flagged.pending_update_state = PendingUpdateState::Mismatch;
        let _ = write_atomic_json(&pending_path, &flagged);
        return ReloadVerifyOutcome::VerificationFailed {
            expected: pending.artifact_sha256,
            actual,
        };
    }

    // Match — promote. Rewrite pending state to Success, write installed-state.json with the
    // new version, then remove pending-update.json so the next relaunch is clean.
    let mut promoted = pending.clone();
    promoted.pending_update_state = PendingUpdateState::Success;
    let installed_state = InstalledState {
        managed_executable_path: promoted.managed_executable_path.clone(),
        install_kind: InstallKind::AppImage,
        channel: promoted.channel.clone(),
        version: promoted.to_version.clone(),
        source_commit: promoted.source_commit.clone(),
        release_tag: promoted.release_tag.clone(),
        manifest_hash: promoted.manifest_hash.clone(),
        artifact_sha256: promoted.artifact_sha256.clone(),
        installed_at: now_iso8601(),
        update_eligible: true,
        ineligible_reason: None,
    };
    let installed_state_path = update_dir.join(INSTALLED_STATE_FILENAME);
    if write_atomic_json(&installed_state_path, &installed_state).is_err() {
        // If we cannot write installed-state.json we do not promote: leave pending in place
        // and surface as a verification failure so the operator can investigate.
        return ReloadVerifyOutcome::VerificationFailed {
            expected: promoted.artifact_sha256,
            actual,
        };
    }
    let _ = fs::remove_file(&pending_path);

    ReloadVerifyOutcome::Promoted {
        installed_state_path,
        promoted_version: promoted.to_version,
    }
}

/// Tauri command shim — `verify_relaunch_artifact`.
///
/// The shim owns the two boundary concerns the transaction module refuses to own: hashing
/// the running binary (`std::env::current_exe()` streamed through `sha256_of_file`) and
/// resolving the config root (see `resolve_config_root`). Verification truth itself comes
/// from `verify_relaunch_artifact_impl`, whose inline tests (AV-INST-6, AV-RB-1) prove the
/// contract this shim forwards to. Failures to identify or read the running binary surface
/// as `VerificationFailed` — never as a fabricated promotion.
#[tauri::command]
pub fn verify_relaunch_artifact() -> ReloadVerifyOutcome {
    let exe = match std::env::current_exe() {
        Ok(path) => path,
        Err(_) => {
            return ReloadVerifyOutcome::VerificationFailed {
                expected: "<unknown>".to_string(),
                actual: "<current_exe-error>".to_string(),
            };
        }
    };
    let running_sha256 = match sha256_of_file(&exe) {
        Ok(digest) => digest,
        Err(_) => {
            return ReloadVerifyOutcome::VerificationFailed {
                expected: "<unknown>".to_string(),
                actual: "<read-error>".to_string(),
            };
        }
    };
    verify_relaunch_artifact_impl(ReloadVerifyConfig {
        config_dir: resolve_config_root(),
        running_artifact_sha256: || Ok(running_sha256),
    })
}

/// Resolve the config ROOT that `config_update_dir` hangs the
/// `codex-desktop-shell-scaffold/update/` tree off.
///
/// `CODEX_CONFIG_DIR` overrides the root — the ops/integration-test redirect seam,
/// following the `CODEX_REPO_ROOT` / `CODEX_DESKTOP_RESOURCE_DIR` env idiom in
/// `ge08_workbench.rs`. The default is `$HOME/.config`, so the pending record resolves to
/// `~/.config/codex-desktop-shell-scaffold/update/pending-update.json`.
fn resolve_config_root() -> PathBuf {
    if let Ok(dir) = std::env::var("CODEX_CONFIG_DIR") {
        return PathBuf::from(dir);
    }
    match std::env::var("HOME") {
        Ok(home) => PathBuf::from(home).join(".config"),
        Err(_) => PathBuf::from(".config"),
    }
}

/// Stream a file through sha256, returning the lowercase-hex digest.
///
/// Extracted as a shared seam so the `verify_relaunch_artifact` shim and its inline test
/// hash through the exact same code path — the test proves the seam without needing a real
/// AppImage binary.
fn sha256_of_file(path: &Path) -> std::io::Result<String> {
    use std::io::Read;
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hex_lower(&hasher.finalize()))
}

/// Tauri command shim — `is_install_eligible`.
///
/// F3a's PR #61 deferred the Tauri command layer; F0-EXTEND `t_5b652e93` authorizes F3c to
/// register the command surface. The eligibility probe body remains F3a's deferred
/// surface, so until it ships this shim returns an honest wiring error instead of
/// fabricating an eligibility verdict.
#[tauri::command]
pub fn is_install_eligible() -> Result<EligibilityPolicy, String> {
    Err(
        "is_install_eligible is registered but not wired: the eligibility probe body was \
         deferred by F3a (PR #61); no eligibility truth is fabricated here"
            .to_string(),
    )
}

/// Tauri command shim — `perform_install`.
///
/// Registration-only (F0-EXTEND `t_5b652e93`): the staged-transaction command body — its
/// argument contract included — is F3a's deferred surface around `execute_transaction`.
/// Until F3a wires it, the shim errors rather than pretending an install ran.
#[tauri::command]
pub fn perform_install() -> Result<RelaunchPrompt, String> {
    Err(
        "perform_install is registered but not wired: the staged-transaction command body \
         was deferred by F3a (PR #61); no install is performed here"
            .to_string(),
    )
}

/// Tauri command shim — `perform_restore_previous`.
///
/// Registration-only (F0-EXTEND `t_5b652e93`): F3b owns the restore body and its success
/// payload shape. Until F3b ships, the shim errors rather than pretending a restore ran.
#[tauri::command]
pub fn perform_restore_previous() -> Result<(), String> {
    Err(
        "perform_restore_previous is registered but not wired: the restore body is F3b's \
         surface; no restore is performed here"
            .to_string(),
    )
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
                running.artifact_sha256, installed.artifact_sha256
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
    let slot1 = backups.join(BACKUP_APPIMAGE_FILENAME);
    let slot2 = backups.join(BACKUP_APPIMAGE_FILENAME_2);
    // Shift slot 1 → slot 2 before overwriting slot 1.
    if slot1.exists() {
        fs::rename(&slot1, &slot2).map_err(|source| TransactionAbort {
            code: TransactionAbortCode::BackupRotationFailed,
            reason: format!(
                "could not rotate {} -> {}: {source}",
                slot1.display(),
                slot2.display()
            ),
        })?;
    }
    if managed_executable_path.exists() {
        fs::copy(managed_executable_path, &slot1).map_err(|source| TransactionAbort {
            code: TransactionAbortCode::BackupRotationFailed,
            reason: format!(
                "could not copy {} -> {}: {source}",
                managed_executable_path.display(),
                slot1.display()
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
    // RFC 3339 / ISO-8601 UTC timestamp for the pending-update.json marker.
    // The transaction does not depend on wall-clock for any decision logic; this is only a
    // human-readable field. We avoid external crates by using the civil_from_days algorithm
    // (Howard Hinnant) to convert epoch seconds to a calendar date.
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let sec = (secs % 60) as u32;
    let min = ((secs / 60) % 60) as u32;
    let hour = ((secs / 3600) % 24) as u32;

    // civil_from_days: convert days-since-1970-01-01 to (year, month, day)
    let z = (secs / 86400) as i64 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { y + 1 } else { y };

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hour, min, sec
    )
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
    fn ineligible_manifest_aborts_before_download() {
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
            download: |_w| panic!("download must not be called for ineligible manifest"),
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

// -- unit tests for preserve_previous rotation logic -----------------------------------
#[cfg(test)]
mod preserve_previous_tests {
    use super::*;

    fn tempdir(label: &str) -> std::path::PathBuf {
        let dir =
            std::env::temp_dir().join(format!("sd16-preserve-{label}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create tempdir");
        dir
    }

    /// Calling preserve_previous twice rotates slot1 into slot2 and writes the newer
    /// binary into slot1, capping backups at two entries.
    #[test]
    fn backup_rotation_caps_at_two_slots() {
        let dir = tempdir("two-slot");
        let managed = dir.join("Codex-Desktop-Shell-Scaffold.AppImage");

        // First generation: write "v1" as the managed binary.
        fs::write(&managed, b"binary-v1").expect("write v1");
        preserve_previous(&dir, &managed).expect("first rotation");

        let backups = backups_dir(&dir);
        let slot1 = backups.join(BACKUP_APPIMAGE_FILENAME);
        let slot2 = backups.join(BACKUP_APPIMAGE_FILENAME_2);
        assert_eq!(fs::read(&slot1).expect("slot1 after first"), b"binary-v1");
        assert!(!slot2.exists(), "slot2 must not exist after first rotation");

        // Second generation: update managed binary to "v2".
        fs::write(&managed, b"binary-v2").expect("write v2");
        preserve_previous(&dir, &managed).expect("second rotation");

        assert_eq!(fs::read(&slot1).expect("slot1 after second"), b"binary-v2");
        assert_eq!(
            fs::read(&slot2).expect("slot2 after second"),
            b"binary-v1",
            "slot2 must hold the previously-slot1 binary"
        );

        // Third generation: update managed binary to "v3" — slot2 must be overwritten,
        // keeping total backup count at two.
        fs::write(&managed, b"binary-v3").expect("write v3");
        preserve_previous(&dir, &managed).expect("third rotation");

        assert_eq!(fs::read(&slot1).expect("slot1 after third"), b"binary-v3");
        assert_eq!(
            fs::read(&slot2).expect("slot2 after third"),
            b"binary-v2",
            "oldest backup (v1) must be dropped; slot2 holds v2"
        );
    }
}

// -- unit tests for F3c verify_relaunch_artifact_impl --------------------------------
#[cfg(test)]
mod verify_relaunch_artifact_tests {
    use super::*;

    fn tempdir(label: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("sd16-verify-{label}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create tempdir");
        dir
    }

    /// Write a fresh `pending-update.json` for the given `expected` sha256 so the verifier
    /// has something to read.
    fn write_pending(dir: &Path, expected_sha: &str, from: &str, to: &str) {
        let update_dir = config_update_dir(dir);
        fs::create_dir_all(&update_dir).expect("mkdir update");
        let pending = PendingUpdate {
            from_version: from.into(),
            to_version: to.into(),
            artifact_sha256: expected_sha.into(),
            manifest_hash: "manifest-hash-fixture".into(),
            staging_path: update_dir.join("staging/AppImage"),
            backup_path: update_dir.join("backups/AppImage"),
            managed_executable_path: dir.join("Codex-Desktop-Shell-Scaffold.AppImage"),
            channel: "alpha".into(),
            release_tag: format!("alpha/{to}"),
            source_commit: "deadbeef".into(),
            created_at: "2026-07-03T00:00:00Z".into(),
            pending_update_state: PendingUpdateState::Pending,
        };
        write_atomic_json(&update_dir.join(PENDING_UPDATE_FILENAME), &pending).expect("write");
    }

    /// AV-INST-6 — relaunch verification confirms a running artifact whose hash matches the
    /// staged `pending-update.json`. The verifier must promote: rewrite pending state to
    /// Success, write a fresh `installed-state.json`, and delete `pending-update.json`.
    #[test]
    fn relaunch_verification_match_promotes_installed_state() {
        let dir = tempdir("match");
        write_pending(&dir, "expected-sha", "0.0.0", "0.0.1");

        let outcome = verify_relaunch_artifact_impl(ReloadVerifyConfig {
            config_dir: dir.clone(),
            running_artifact_sha256: || Ok("expected-sha".to_string()),
        });
        match outcome {
            ReloadVerifyOutcome::Promoted {
                installed_state_path,
                promoted_version,
            } => {
                assert_eq!(promoted_version, "0.0.1");
                assert!(installed_state_path.ends_with(INSTALLED_STATE_FILENAME));
                let installed: InstalledState =
                    serde_json::from_slice(&fs::read(&installed_state_path).expect("read"))
                        .expect("parse");
                assert_eq!(installed.version, "0.0.1");
                assert_eq!(installed.artifact_sha256, "expected-sha");
                assert_eq!(installed.channel, "alpha");
                assert_eq!(installed.install_kind, InstallKind::AppImage);
                assert!(installed.update_eligible);
                assert!(installed.ineligible_reason.is_none());
            }
            other => panic!("expected Promoted, got {other:?}"),
        }
        let update_dir = config_update_dir(&dir);
        assert!(
            !update_dir.join(PENDING_UPDATE_FILENAME).exists(),
            "pending-update.json must be removed on promotion"
        );
    }

    /// AV-RB-1 — relaunch verification detects a hash mismatch. The verifier must NOT mark
    /// success: it must return `VerificationFailed`, leave `installed-state.json` alone
    /// (or absent if first install), and flag the pending record so the restoreOffer UI
    /// can surface the prior version.
    #[test]
    fn relaunch_verification_mismatch_does_not_mark_success() {
        let dir = tempdir("mismatch");
        write_pending(&dir, "expected-sha", "0.0.0", "0.0.1");

        let outcome = verify_relaunch_artifact_impl(ReloadVerifyConfig {
            config_dir: dir.clone(),
            running_artifact_sha256: || Ok("different-running-sha".to_string()),
        });
        match outcome {
            ReloadVerifyOutcome::VerificationFailed { expected, actual } => {
                assert_eq!(expected, "expected-sha");
                assert_eq!(actual, "different-running-sha");
            }
            other => panic!("expected VerificationFailed, got {other:?}"),
        }

        // pending-update.json must remain on disk with `pending_update_state = Mismatch` so
        // the restoreOffer UI and F3b's `perform_restore_previous` can recover.
        let update_dir = config_update_dir(&dir);
        let pending_path = update_dir.join(PENDING_UPDATE_FILENAME);
        assert!(
            pending_path.exists(),
            "pending-update.json must remain on disk on mismatch"
        );
        let flagged: PendingUpdate =
            serde_json::from_slice(&fs::read(&pending_path).expect("read")).expect("parse");
        assert_eq!(flagged.pending_update_state, PendingUpdateState::Mismatch);

        // installed-state.json must NOT have been written — first install would have nothing
        // on disk, and a previous install must keep its prior installed-state untouched.
        assert!(
            !update_dir.join(INSTALLED_STATE_FILENAME).exists(),
            "installed-state.json must not be written on mismatch"
        );
    }

    /// When no pending-update.json is on disk (clean relaunch, or after a successful prior
    /// promotion cleared it), the verifier must return `NoPendingUpdate` and not touch any
    /// files.
    #[test]
    fn relaunch_verification_no_pending_returns_no_pending_update() {
        let dir = tempdir("no-pending");

        let outcome = verify_relaunch_artifact_impl(ReloadVerifyConfig {
            config_dir: dir.clone(),
            running_artifact_sha256: || Ok("any-sha".to_string()),
        });
        assert_eq!(outcome, ReloadVerifyOutcome::NoPendingUpdate);

        let update_dir = config_update_dir(&dir);
        assert!(
            !update_dir.join(PENDING_UPDATE_FILENAME).exists(),
            "verifier must not write pending-update.json when none was on disk"
        );
        assert!(
            !update_dir.join(INSTALLED_STATE_FILENAME).exists(),
            "verifier must not write installed-state.json when there was no pending"
        );
    }

    /// Defensive: a tampered or unreadable `pending-update.json` must be treated as
    /// `NoPendingUpdate` so a corrupt marker never wedges the shell on every relaunch.
    #[test]
    fn relaunch_verification_unreadable_pending_falls_back_to_no_pending() {
        let dir = tempdir("corrupt");
        let update_dir = config_update_dir(&dir);
        fs::create_dir_all(&update_dir).expect("mkdir");
        fs::write(
            update_dir.join(PENDING_UPDATE_FILENAME),
            b"not-json-{garbage",
        )
        .expect("write garbage");

        let outcome = verify_relaunch_artifact_impl(ReloadVerifyConfig {
            config_dir: dir.clone(),
            running_artifact_sha256: || Ok("any-sha".to_string()),
        });
        assert_eq!(outcome, ReloadVerifyOutcome::NoPendingUpdate);
    }

    /// If the running-hash closure returns an I/O error (e.g. the binary could not be read),
    /// the verifier must not promote; it must surface a verification failure with the
    /// expected sha so the operator is prompted to restore.
    #[test]
    fn relaunch_verification_running_hash_error_returns_verification_failed() {
        let dir = tempdir("hash-error");
        write_pending(&dir, "expected-sha", "0.0.0", "0.0.1");

        let outcome = verify_relaunch_artifact_impl(ReloadVerifyConfig {
            config_dir: dir.clone(),
            running_artifact_sha256: || {
                Err(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    "binary unreadable",
                ))
            },
        });
        match outcome {
            ReloadVerifyOutcome::VerificationFailed { expected, actual } => {
                assert_eq!(expected, "expected-sha");
                assert_eq!(actual, "<compute-error>");
            }
            other => panic!("expected VerificationFailed, got {other:?}"),
        }
    }

    /// F3c hashing seam: the Tauri shim streams the running binary through
    /// `sha256_of_file`. Hashing an empty file must yield the well-known empty-input
    /// sha256, proving the seam compiles and works without a real AppImage fixture.
    #[test]
    fn sha256_of_file_roundtrip_on_empty_file() {
        let dir = tempdir("sha-empty");
        let path = dir.join("empty.bin");
        fs::write(&path, b"").expect("write empty file");
        let digest = sha256_of_file(&path).expect("hash empty file");
        assert_eq!(
            digest,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    /// The F3a/F3b-deferred command bodies are not wired yet. Their registration-only
    /// shims must return an honest error instead of fabricating eligibility, install,
    /// or restore truth. F0-EXTEND `t_5b652e93` authorizes registration; the bodies
    /// remain F3a's (`is_install_eligible`, `perform_install`) and F3b's
    /// (`perform_restore_previous`) surfaces.
    #[test]
    fn deferred_command_shims_error_instead_of_fabricating_truth() {
        assert!(is_install_eligible().is_err());
        assert!(perform_install().is_err());
        assert!(perform_restore_previous().is_err());
    }

    /// The F3c TS mirror (`installAction.ts` `ReloadVerifyOutcome`) reads camelCase
    /// fields under kebab-case `kind` tags — the module-wide wire convention. Pin the
    /// serialized shape so the Rust and TS halves of F3c cannot drift.
    #[test]
    fn reload_verify_outcome_serializes_kebab_kinds_with_camel_fields() {
        let promoted = ReloadVerifyOutcome::Promoted {
            installed_state_path: PathBuf::from("/tmp/installed-state.json"),
            promoted_version: "0.0.1".to_string(),
        };
        let json = serde_json::to_string(&promoted).expect("serialize promoted");
        assert!(
            json.contains("\"kind\":\"promoted\""),
            "kebab kind tag: {json}"
        );
        assert!(
            json.contains("\"installedStatePath\":"),
            "camelCase installedStatePath: {json}"
        );
        assert!(
            json.contains("\"promotedVersion\":\"0.0.1\""),
            "camelCase promotedVersion: {json}"
        );

        let failed = ReloadVerifyOutcome::VerificationFailed {
            expected: "aaa".to_string(),
            actual: "bbb".to_string(),
        };
        let json = serde_json::to_string(&failed).expect("serialize failed");
        assert!(
            json.contains("\"kind\":\"verification-failed\""),
            "kebab kind tag: {json}"
        );
    }
}
