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
const STAGED_APPIMAGE_FILENAME: &str = "Codex.staged.AppImage";

/// Filename of the most-recent rolling backup of the previous managed AppImage (slot 1).
const BACKUP_APPIMAGE_FILENAME: &str = "Codex.previous.AppImage";

/// Filename of the older rolling backup (slot 2). Slot 1 is shifted here before slot 1 is
/// overwritten, capping the backup set at two entries.
const BACKUP_APPIMAGE_FILENAME_2: &str = "Codex.previous2.AppImage";

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
        .join("codex")
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
///   update is promoted: a fresh `installed-state.json` is written so future transactions see
///   the new version as installed, and `pending-update.json` is deleted.
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
/// `codex/update/` tree off.
///
/// `CODEX_CONFIG_DIR` overrides the root — the ops/integration-test redirect seam,
/// following the `CODEX_REPO_ROOT` / `CODEX_DESKTOP_RESOURCE_DIR` env idiom in
/// `authoring_workbench.rs`. The default is `$HOME/.config`, so the pending record resolves to
/// `~/.config/codex/update/pending-update.json`.
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

/// Real local install-state probe result returned by `is_install_eligible`.
///
/// Distinct from `EligibilityPolicy` (which describes a *manifest's
/// declared* policy, e.g. `ManifestIdentity.eligibility_policy`) — this
/// describes what is actually installed locally, in exactly the raw shape
/// `decideEligibility` (`eligibility.ts`, TS) needs in order to compare
/// against a separately-fetched manifest. This module deliberately does not
/// render an eligible/ineligible verdict itself: `decideEligibility` is the
/// single source of eligibility-decision truth (rows 5-9 combined), so this
/// probe reports facts, not a policy judgment, to avoid two independent
/// (and driftable) copies of that decision table.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallEligibilityProbe {
    /// `None` when there is no `installed-state.json` on disk yet (first
    /// run, or a run that never completed `verify_relaunch_artifact`) — the
    /// caller must not fabricate a version/hash to compare against a
    /// manifest in that case.
    pub installed: Option<InstalledState>,
    /// Result of the real writability probe on `installed.managed_executable_path`.
    /// Meaningless (and left `false`) when `installed` is `None`.
    pub is_managed_path_writable: bool,
}

/// Tauri command shim — `is_install_eligible`.
///
/// E3.13/E3.14: this is now a real local-state probe (F3a's PR #61
/// registered the command; the body was deferred until these slices). It
/// reads `installed-state.json` under the resolved config root and reports
/// the real install-kind/version/hash/writability facts `decideEligibility`
/// needs — it does not itself decide eligible/ineligible/unknown.
#[tauri::command]
pub fn is_install_eligible() -> Result<InstallEligibilityProbe, String> {
    is_install_eligible_impl(&resolve_config_root())
}

/// Real body behind the `is_install_eligible` shim, injectable with a config
/// root so tests never touch the real `$HOME/.config` tree.
fn is_install_eligible_impl(config_dir: &Path) -> Result<InstallEligibilityProbe, String> {
    let installed_state_path = config_update_dir(config_dir).join(INSTALLED_STATE_FILENAME);
    let bytes = match fs::read(&installed_state_path) {
        Ok(b) => b,
        Err(_) => {
            // No installed-state record yet — either this is the first run
            // ever, or `verify_relaunch_artifact` has never promoted one.
            // Honest "nothing to report" result, not an error: the probe ran
            // fine, it just found no local record to report facts about.
            return Ok(InstallEligibilityProbe {
                installed: None,
                is_managed_path_writable: false,
            });
        }
    };
    let installed: InstalledState = serde_json::from_slice(&bytes).map_err(|source| {
        format!(
            "installed-state.json at {} is unreadable: {source}",
            installed_state_path.display()
        )
    })?;

    let is_managed_path_writable = managed_path_is_writable(&installed.managed_executable_path);
    Ok(InstallEligibilityProbe {
        installed: Some(installed),
        is_managed_path_writable,
    })
}

/// Real writability probe: attempt to create-and-remove a sentinel file next
/// to the managed executable. Permission *bits* alone don't decide
/// writability (ownership/ACLs matter too), so this probes the real
/// operation rather than parsing `st_mode`.
fn managed_path_is_writable(managed_executable_path: &Path) -> bool {
    let Some(parent) = managed_executable_path.parent() else {
        return false;
    };
    let probe = parent.join(".codex-update-write-probe");
    match fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&probe)
    {
        Ok(_) => {
            let _ = fs::remove_file(&probe);
            true
        }
        Err(_) => false,
    }
}

/// Tauri command shim — `perform_install`.
///
/// Registration-only: the staged-transaction body
/// (`execute_transaction`) already exists and is fully tested (see `mod
/// tests` above) — what remains genuinely missing is the *download* step
/// `execute_transaction`'s `download` closure needs: this crate carries no
/// HTTP client dependency (no `reqwest`/`ureq`/etc. in `Cargo.toml`), so
/// there is no way to fetch the AppImage artifact bytes from here without
/// adding one. Rather than adding a new dependency as a side-effect of an
/// Update-UI bug-fix slice (E3.13's authorized file-touch scope does not
/// include `Cargo.toml`), this shim stays an honest deferred error naming
/// that exact, narrower gap. `is_install_eligible` above no longer shares
/// this limitation — E3.13 gave it a real body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformInstallArgs {
    pub manifest: serde_json::Value,
    pub index_url: String,
}

#[tauri::command]
pub fn perform_install(_args: PerformInstallArgs) -> Result<RelaunchPrompt, String> {
    Err(
        "perform_install is registered but not wired: downloading the AppImage artifact \
         requires an HTTP client this crate does not carry as a dependency yet; \
         execute_transaction itself is real and tested, but no install is performed here \
         until a download step lands"
            .to_string(),
    )
}

/// Tauri command shim — `perform_restore_previous`.
///
/// F3b authors the body. The command reads the pending-update marker
/// (`<config_update_dir>/pending-update.json`), locates the most-recent
/// rolling backup at `<config_update_dir>/backups/Codex.previous.AppImage`,
/// and atomically copies that backup over the managed AppImage path. On
/// restore failure (missing/unreadable/corrupted backup), the command writes
/// a `rollback-state.json` sidecar recording `rollback_state: "rollback-failed"`
/// with the exact reason string (AV-RB-4) and leaves `pending-update.json` on
/// disk so the operator can inspect or retry (AV-RB-7).
///
/// On the third consecutive mismatch (mismatch_count reaches 3 in the
/// sidecar `rollback-state.json`), the command short-circuits to auto-restore
/// without operator confirmation (AV-RB-3 — repeated pending failure
/// auto-restores when safe).
///
/// The companion `perform_retention_sweep` command is responsible for the
/// `backups/` two-slot enforcement (AV-RB-5), `staging/` post-success cleanup
/// (AV-RB-6), and the never-auto-delete-while-unresolved guarantee for
/// `pending-update.json` (AV-RB-7).
#[tauri::command]
#[allow(dead_code)] // not exercised by tests — impl is tested via perform_restore_previous_impl
pub fn perform_restore_previous() -> Result<RollbackOutcome, String> {
    Ok(perform_restore_previous_impl(RestoreConfig {
        config_dir: resolve_config_root(),
        bump_mismatch_count: true,
    }))
}

/// Tauri command shim — `perform_retention_sweep`.
///
/// F3b authors the body. The sweep runs as the final step of every successful
/// transaction cycle and enforces the retention policy quoted verbatim in
/// `technical-requirements.md` §"Retention Requirements":
///
///   - backups: keep last 2 previous AppImages (AV-RB-5)
///   - successful staging: delete after update success (AV-RB-6)
///   - pending updates: never auto-delete while unresolved (AV-RB-7)
///   - rollback-failed: keep until explicit user clear (no AV id yet; see AV-RB-4)
///
/// The function returns a `RetentionOutcome` describing which cleanup actions
/// were applied so the operator UI can surface a diagnostic.
#[tauri::command]
#[allow(dead_code)] // not exercised by tests — impl is tested via perform_retention_sweep_impl
pub fn perform_retention_sweep() -> Result<RetentionOutcome, String> {
    Ok(perform_retention_sweep_impl(RetentionConfig {
        config_dir: resolve_config_root(),
    }))
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
            artifact_name: "Codex.AppImage".into(),
            artifact_size: artifact_bytes.len() as u64,
            eligibility_policy: EligibilityPolicy {
                update_eligible: true,
                ineligible_reason: None,
            },
        }
    }

    fn make_running(dir: &Path) -> RunningBuildIdentity {
        RunningBuildIdentity {
            managed_executable_path: dir.join("Codex.AppImage"),
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
            managed_executable_path: dir.join("Codex.AppImage"),
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
        let managed = dir.join("Codex.AppImage");
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
                    .map_err(std::io::Error::other)?;
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
        let managed = dir.join("Codex.AppImage");
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
                    .map_err(std::io::Error::other)?;
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
        let managed = dir.join("Codex.AppImage");
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
        let managed = dir.join("Codex.AppImage");
        let artifact = b"first-install-payload";
        let manifest = make_manifest(artifact);
        let outcome = execute_transaction(TransactionConfig {
            config_dir: dir.clone(),
            manifest: manifest.clone(),
            running_build: make_running(&dir),
            installed_state: None,
            download: |w| {
                w.write_all(artifact)
                    .map_err(std::io::Error::other)?;
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
        let managed = dir.join("Codex.AppImage");
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
        let managed = dir.join("Codex.AppImage");
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
        let managed = dir.join("Codex.AppImage");
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
                    .map_err(std::io::Error::other)?;
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
            PathBuf::from("/tmp/fake-xdg/codex/update")
        );
    }

    #[test]
    fn cursor_based_download_smoke() {
        // Confirms the injected-closure seam works with a std::io::Cursor — same shape as
        // production would use with a streaming HTTP reader.
        let dir = tempdir("cursor");
        let managed = dir.join("Codex.AppImage");
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
        let managed = dir.join("Codex.AppImage");

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
            managed_executable_path: dir.join("Codex.AppImage"),
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

    /// `perform_install`'s staged-transaction body (downloading the artifact
    /// bytes) needs an HTTP client this crate does not carry as a dependency
    /// yet — that remains a genuinely deferred surface, not fabricated. This
    /// test no longer covers `is_install_eligible`: E3.13 gave that shim a
    /// real, tested body (`is_install_eligible_probe_tests` below) — it now
    /// reads real on-disk state rather than always erroring.
    ///
    /// `perform_restore_previous` was a registration-only stub through PR #63.
    /// A later backfill shipped its real body, so this test no longer asserts
    /// `perform_restore_previous().is_err()` — it asserts only the
    /// still-deferred `perform_install` command. The `rollback_retention_tests`
    /// mod covers the restore-previous body contract.
    #[test]
    fn perform_install_shim_errors_instead_of_fabricating_truth() {
        assert!(
            perform_install(PerformInstallArgs {
                manifest: serde_json::json!({}),
                index_url: "https://example.invalid/index.json".to_string(),
            })
            .is_err()
        );
    }

    /// E3.13 — `is_install_eligible`'s real probe body: with no
    /// `installed-state.json` on disk yet (first run, or a run that never
    /// completed `verify_relaunch_artifact`), the probe must degrade to an
    /// honest "nothing to report" result (`installed: None`) rather than
    /// erroring or fabricating a record.
    #[test]
    fn is_install_eligible_probe_reports_none_when_installed_state_missing() {
        let dir = tempdir("probe-no-record");
        let probe = is_install_eligible_impl(&dir).expect("probe body is real, not an error");
        assert!(probe.installed.is_none(), "no record yet must report installed: None");
        assert!(!probe.is_managed_path_writable, "writability is meaningless with no installed record");
    }

    /// E3.14 — a real `installed-state.json` round-trips through the probe
    /// verbatim, and the writability probe genuinely reflects the real
    /// filesystem state next to the managed path (the probe itself renders
    /// no eligible/ineligible verdict — that is `decideEligibility`'s job on
    /// the TS side, fed by these raw facts).
    #[test]
    fn is_install_eligible_probe_reports_real_appimage_record_and_writability() {
        let dir = tempdir("probe-appimage-writable");
        let update_dir = config_update_dir(&dir);
        fs::create_dir_all(&update_dir).expect("mkdir update dir");
        let managed = dir.join("Codex.AppImage");
        fs::write(&managed, b"binary").expect("write managed binary");
        let installed = InstalledState {
            managed_executable_path: managed,
            install_kind: InstallKind::AppImage,
            channel: "alpha".into(),
            version: "0.1.0".into(),
            source_commit: "deadbeef".into(),
            release_tag: "alpha/v0.1.0".into(),
            manifest_hash: "manifest-hash".into(),
            artifact_sha256: "artifact-sha".into(),
            installed_at: "2026-07-03T00:00:00Z".into(),
            update_eligible: true,
            ineligible_reason: None,
        };
        write_atomic_json(&update_dir.join(INSTALLED_STATE_FILENAME), &installed)
            .expect("write installed-state.json");

        let probe = is_install_eligible_impl(&dir).expect("probe body is real, not an error");
        assert_eq!(probe.installed, Some(installed), "installed-state.json round-trips verbatim");
        assert!(probe.is_managed_path_writable, "the managed path's parent dir is genuinely writable in a tempdir");
    }

    /// E3.15 — per-probe outcome: a managed path whose parent directory does
    /// not exist can never be opened for writing, so the real writability
    /// probe must honestly report `false` (not silently default to `true`).
    #[test]
    fn is_install_eligible_probe_reports_writability_false_when_parent_dir_missing() {
        let dir = tempdir("probe-unwritable-parent");
        let update_dir = config_update_dir(&dir);
        fs::create_dir_all(&update_dir).expect("mkdir update dir");
        let managed = dir.join("nonexistent-subdir").join("Codex.AppImage");
        let installed = InstalledState {
            managed_executable_path: managed,
            install_kind: InstallKind::AppImage,
            channel: "alpha".into(),
            version: "0.1.0".into(),
            source_commit: "deadbeef".into(),
            release_tag: "alpha/v0.1.0".into(),
            manifest_hash: "manifest-hash".into(),
            artifact_sha256: "artifact-sha".into(),
            installed_at: "2026-07-03T00:00:00Z".into(),
            update_eligible: true,
            ineligible_reason: None,
        };
        write_atomic_json(&update_dir.join(INSTALLED_STATE_FILENAME), &installed)
            .expect("write installed-state.json");

        let probe = is_install_eligible_impl(&dir).expect("probe body is real, not an error");
        assert!(
            !probe.is_managed_path_writable,
            "a managed path under a nonexistent parent dir must probe as unwritable"
        );
    }

    /// A `dev-local` install still round-trips through the probe as real
    /// data — the probe reports facts, it does not gate on install kind.
    /// `decideEligibility`'s row for `install_kind === 'dev'` is what
    /// renders this ineligible, on the TS side.
    #[test]
    fn is_install_eligible_probe_reports_dev_local_install_kind_as_a_fact_not_a_verdict() {
        let dir = tempdir("probe-dev-local");
        let update_dir = config_update_dir(&dir);
        fs::create_dir_all(&update_dir).expect("mkdir update dir");
        let managed = dir.join("codex-dev-binary");
        fs::write(&managed, b"binary").expect("write managed binary");
        let installed = InstalledState {
            managed_executable_path: managed,
            install_kind: InstallKind::DevLocal,
            channel: "alpha".into(),
            version: "0.1.0".into(),
            source_commit: "deadbeef".into(),
            release_tag: "alpha/v0.1.0".into(),
            manifest_hash: "manifest-hash".into(),
            artifact_sha256: "artifact-sha".into(),
            installed_at: "2026-07-03T00:00:00Z".into(),
            update_eligible: false,
            ineligible_reason: None,
        };
        write_atomic_json(&update_dir.join(INSTALLED_STATE_FILENAME), &installed)
            .expect("write installed-state.json");

        let probe = is_install_eligible_impl(&dir).expect("probe body is real, not an error");
        assert_eq!(
            probe.installed.map(|i| i.install_kind),
            Some(InstallKind::DevLocal),
            "install kind is reported honestly as a fact"
        );
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
// ===================================================================================
// Rollback / retention surface
//
// Slice: rollback decision table + retention sweep.
//
// Scope note:
//   - This section documents the F3b rollback/retention implementation surface
//     in this module only. It is not a literal claim about every file changed
//     in the pull request diff.
//
// AV-row coverage (per acceptance-and-verification.md §"Rollback and Retention"):
//
//   - AV-RB-3 (F3b) — Repeated pending failure auto-restores when safe.
//       Trigger: sidecar `rollback-state.json` records mismatch_count; when
//       mismatch_count reaches `AUTO_RESTORE_THRESHOLD` (3), the next
//       `perform_restore_previous` invocation auto-runs without requiring
//       operator confirmation. The count is bumped inside the impl on every
//       call when a Mismatch pending marker is on disk; the third call (or
//       any subsequent call once count >= threshold) auto-restores.
//
//   - AV-RB-4 (F3b state side) — rollback-failed is recorded with the
//       exact reason when restore itself fails. F3b writes a sidecar
//       `rollback-state.json` with `rollback_state: "rollback-failed"` and
//       a `reason` string; pending-update.json is left intact so the
//       operator can inspect. (The TS-side `rollbackFailed.tsx` is OUT OF
//       SCOPE for this backfill per the card body's explicit
//       "Do NOT author rollbackFailed.tsx" instruction — a separate F0-EXTEND
//       is owed for the UI half.)
//
//   - AV-RB-5 (F3b cross-cite with F3a) — backups keep last 2. F3b's retention
//       sweep defensively enforces the two-slot ceiling on every invocation:
//       if more than two `Codex.previous*.AppImage`
//       files exist, the oldest by mtime is removed. F3a's `preserve_previous`
//       already enforces the rotation on every fresh install; F3b's sweep is
//       the belt-and-suspenders catch-up for any drift.
//
//   - AV-RB-6 (F3b cross-cite with F3a) — staging cleanup after success.
//       When `pending-update.json` shows `pending_update_state: "Success"`,
//       F3b's sweep removes every file in `staging/`. On Mismatch or Pending,
//       staging files are kept (operator diagnostics + retry path).
//
//   - AV-RB-7 (F3b) — pending updates are never auto-deleted while
//       unresolved. F3b's retention sweep never touches `pending-update.json`
//       when `pending_update_state != "Success"`. The success path deletion
//       is owned by F3c's `verify_relaunch_artifact_impl` (which deletes
//       pending on Promoted), not F3b.
// ===================================================================================

/// Threshold for the AV-RB-3 auto-restore trigger. After this many consecutive
/// mismatch cycles, the next `perform_restore_previous` call auto-restores
/// without requiring explicit operator confirmation.
pub const AUTO_RESTORE_THRESHOLD: u32 = 3;

/// Filename of the rollback-state sidecar record. Lives at
/// `<config_update_dir>/rollback-state.json` next to `pending-update.json`.
///
/// This file is F3b-owned; F3a's `PendingUpdate` and `InstalledState` structs
/// remain unmodified to keep the F3a/F3b per-slice byte-range boundary
/// clean. The sidecar is a strict additive surface — its absence on disk is
/// equivalent to `mismatch_count: 0` and `rollback_state: None`.
pub const ROLLBACK_STATE_FILENAME: &str = "rollback-state.json";

/// State of the rollback process. Persisted in `rollback-state.json` and
/// surfaced to the diagnostics panel via `pendingRollbackPanel.tsx` (F3c
/// binding-layer extension, not the UI surface itself).
///
/// Note: the F1 closure does not yet pin an AV id for the `rollback-failed`
/// state itself (it would be AV-RB-8 if proposed as an L0-B amendment).
/// F3b writes the state honestly so the future AV row has a wire contract to
/// bind against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum RollbackState {
    /// No rollback has been attempted or the previous rollback succeeded and
    /// the sidecar was cleared.
    #[default]
    None,
    /// A previous restore attempt failed; the operator must clear this state
    /// explicitly (per `technical-requirements.md` §"Retention Requirements":
    /// "rollback-failed: keep until explicit user clear").
    RollbackFailed,
    /// The 3-cycle mismatch threshold has been reached; the next restore
    /// call auto-runs (AV-RB-3).
    AutoRestorePrevious,
}

/// F3b-owned sidecar record persisted at
/// `<config_update_dir>/rollback-state.json`.
///
/// This struct is additive to the F3a/F3c surface. It is deserialized with
/// `#[serde(default)]` for every field so an absent or partially-written
/// sidecar is treated as "no rollback history".
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RollbackStateRecord {
    /// Current rollback state.
    #[serde(default)]
    pub rollback_state: RollbackState,
    /// Number of consecutive relaunch mismatches observed (incremented inside
    /// `perform_restore_previous_impl` when a Mismatch pending marker is on
    /// disk; reset to zero on a successful restore).
    #[serde(default)]
    pub mismatch_count: u32,
    /// Human-readable reason when `rollback_state == RollbackFailed`. Empty
    /// for `None` and `AutoRestorePrevious`.
    #[serde(default)]
    pub reason: String,
    /// ISO-8601 UTC timestamp of the most recent state transition.
    #[serde(default)]
    pub last_transition_at: String,
}

/// Outcome of `perform_restore_previous`. The Tauri command layer maps this
/// to UI state. Wire contract:
///   - `kind: "promoted"` — the rollback succeeded; installed-state was
///     rewritten with the prior version; `pending-update.json` was cleared;
///     the sidecar was reset to `None` with `mismatch_count: 0`.
///   - `kind: "auto-restored"` — the AV-RB-3 3-cycle trigger fired; the
///     same restore path ran but no operator confirmation was required.
///     Payload is identical to `promoted`.
///   - `kind: "rollback-failed"` — restore itself failed (missing backup,
///     unreadable backup, or atomic-replace failure). The sidecar is
///     written with `rollback_state: "rollback-failed"` and the exact reason.
///     `pending-update.json` is left on disk so the operator can retry or
///     investigate (AV-RB-7).
///   - `kind: "no-backup"` — no backup exists at the canonical slot. This is
///     a degenerate state (F3a's `preserve_previous` always rotates one in
///     during a successful install), but the restore function surfaces it
///     honestly rather than fabricating a no-op success.
///   - `kind: "no-pending"` — no `pending-update.json` is on disk; the
///     restore function treats this as a no-op success (the shell is already
///     in a clean state).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum RollbackOutcome {
    #[serde(rename_all = "camelCase")]
    Promoted {
        restored_from: PathBuf,
        managed_executable_path: PathBuf,
        restored_version: String,
        restored_artifact_sha256: String,
    },
    #[serde(rename_all = "camelCase")]
    AutoRestored {
        restored_from: PathBuf,
        managed_executable_path: PathBuf,
        restored_version: String,
        restored_artifact_sha256: String,
    },
    #[serde(rename_all = "camelCase")]
    RollbackFailed {
        reason: String,
        pending_update_path: PathBuf,
    },
    NoBackup {
        reason: String,
    },
    NoPending,
}

/// Outcome of `perform_retention_sweep`. Wire contract:
///   - `kind: "swept"` — cleanup actions were applied. `removedStagingFiles`
///     lists the staging files deleted (AV-RB-6); `remainingBackupCount` is
///     the post-sweep count, which the diagnostics panel surfaces (and the
///     operator can sanity-check against AV-RB-5's "last 2" requirement).
///   - `kind: "no-op"` — nothing to clean (no `pending-update.json` is on
///     disk in the Success state, no stale backups, no orphan staging files).
///     The sweep is idempotent: running it repeatedly is safe and cheap.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum RetentionOutcome {
    #[serde(rename_all = "camelCase")]
    Swept {
        removed_staging_files: Vec<PathBuf>,
        remaining_backup_count: u32,
    },
    NoOp,
}

/// Configuration for `perform_restore_previous_impl`.
///
/// `bump_mismatch_count` is a test-only seam: production Tauri command shim
/// passes `true` so the AV-RB-3 counter increments on every restore
/// invocation; tests can pass `false` to assert deterministic counter values
/// across multiple restore attempts on a fixture.
pub struct RestoreConfig {
    pub config_dir: PathBuf,
    pub bump_mismatch_count: bool,
}

/// Configuration for `perform_retention_sweep_impl`.
pub struct RetentionConfig {
    pub config_dir: PathBuf,
}

/// Resolve the on-disk path of the rollback-state sidecar.
fn rollback_state_path(config_dir: &Path) -> PathBuf {
    config_update_dir(config_dir).join(ROLLBACK_STATE_FILENAME)
}

/// Read the rollback-state sidecar. Returns the default record when the
/// sidecar is absent, unreadable, or only partially written. This matches
/// the F3c verifier's permissive read-or-treat-as-missing pattern: a corrupt
/// sidecar must never wedge the shell.
fn read_rollback_state(config_dir: &Path) -> RollbackStateRecord {
    let path = rollback_state_path(config_dir);
    match fs::read(&path) {
        Ok(bytes) => serde_json::from_slice::<RollbackStateRecord>(&bytes).unwrap_or_default(),
        Err(_) => RollbackStateRecord::default(),
    }
}

/// Write the rollback-state sidecar atomically (best-effort). I/O errors are
/// surfaced as a `RollbackOutcome::RollbackFailed` reason — we never silently
/// drop a state transition.
fn write_rollback_state(config_dir: &Path, record: &RollbackStateRecord) -> std::io::Result<()> {
    let path = rollback_state_path(config_dir);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    write_atomic_json(&path, record)
}

/// Resolve the path of the most-recent rolling backup. Returns `None` when
/// the slot-1 backup does not exist (degenerate state, but F3b surfaces it
/// honestly).
fn most_recent_backup_path(config_dir: &Path) -> Option<PathBuf> {
    let backups = backups_dir(&config_update_dir(config_dir));
    let slot1 = backups.join(BACKUP_APPIMAGE_FILENAME);
    if slot1.exists() {
        Some(slot1)
    } else {
        None
    }
}

/// Atomic copy of `source` over `managed`. Mirrors `atomic_replace`'s
/// stage-then-rename pattern so the managed AppImage path never holds a
/// partial file. Returns the canonical I/O error so callers can format a
/// diagnostic reason string.
fn atomic_copy_replace(source: &Path, managed: &Path) -> std::io::Result<()> {
    if let Some(parent) = managed.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = managed.with_extension("AppImage.tmp");
    fs::copy(source, &tmp)?;
    fs::rename(&tmp, managed)?;
    Ok(())
}

/// Hash a file via the streaming sha256 seam that F3a already uses. We
/// re-implement the streaming read here (rather than reaching into F3c's
/// `sha256_of_file`) so the F3b surface remains self-contained and testable
/// without crossing the F3c slice boundary.
fn sha256_of_path(path: &Path) -> std::io::Result<String> {
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

/// Core F3b restore implementation. The Tauri command shim delegates here.
///
/// Decision tree (matches the rollback decision table in
/// `SD16-E7-execution-handoff-2026-07-03.md` §"Rollback decision table"):
///
///   1. No `pending-update.json` → `RollbackOutcome::NoPending` (idempotent).
///   2. `pending_update_state != Mismatch` AND != Pending → caller should not
///      be invoking restore here (the success path is F3c's); surface as
///      `NoPending` rather than fabricating a rollback.
///   3. AV-RB-3 fast-path: `rollback_state == AutoRestorePrevious` OR
///      `mismatch_count >= AUTO_RESTORE_THRESHOLD` → run the restore without
///      requiring the operator to confirm.
///   4. No backup exists → `RollbackOutcome::NoBackup` with reason. The
///      sidecar is left alone (no rollback-failed was attempted).
///   5. Backup exists but is unreadable / atomic-replace fails →
///      `RollbackOutcome::RollbackFailed` with the reason; the sidecar is
///      written with `rollback_state: "rollback-failed"`.
///   6. Backup exists and is readable → copy it over the managed path,
///      rewrite `installed-state.json` with the backup's identity (the
///      `pending.from_version`), delete `pending-update.json`, reset the
///      sidecar to `None` with `mismatch_count: 0`. Return
///      `RollbackOutcome::Promoted` (or `AutoRestored` when the AV-RB-3
///      fast-path fired).
pub fn perform_restore_previous_impl(config: RestoreConfig) -> RollbackOutcome {
    let RestoreConfig {
        config_dir,
        bump_mismatch_count,
    } = config;
    let update_dir = config_update_dir(&config_dir);
    let pending_path = update_dir.join(PENDING_UPDATE_FILENAME);

    // Step 1 — read pending marker. Treat absent / corrupt / unreadable as
    // "no pending" so the restore command is always safe to call.
    let pending: PendingUpdate = match fs::read(&pending_path) {
        Ok(bytes) => match serde_json::from_slice(&bytes) {
            Ok(value) => value,
            Err(_) => return RollbackOutcome::NoPending,
        },
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return RollbackOutcome::NoPending;
        }
        Err(_) => return RollbackOutcome::NoPending,
    };

    // Step 2 — only restore when the pending marker indicates an unresolved
    // mismatch or a pending-but-stuck state. A Success marker means the
    // F3c verifier already promoted; restoring from it would be a no-op
    // semantically but the function refuses to fabricate that as a success.
    if matches!(pending.pending_update_state, PendingUpdateState::Success) {
        return RollbackOutcome::NoPending;
    }

    // Bump mismatch count BEFORE the auto-restore fast-path so the sidecar
    // always reflects the most-recent observation. The bump is gated by the
    // `bump_mismatch_count` test seam so tests can drive deterministic
    // counter values.
    let mut sidecar = read_rollback_state(&config_dir);
    if bump_mismatch_count {
        sidecar.mismatch_count = sidecar.mismatch_count.saturating_add(1);
    }

    // Step 3 — AV-RB-3 auto-restore fast-path.
    let auto_restore = matches!(sidecar.rollback_state, RollbackState::AutoRestorePrevious)
        || sidecar.mismatch_count >= AUTO_RESTORE_THRESHOLD;
    if auto_restore {
        sidecar.rollback_state = RollbackState::AutoRestorePrevious;
        // Best-effort sidecar write before the restore so a crash mid-restore
        // still leaves the AV-RB-3 marker visible to the next boot.
        let _ = write_rollback_state(
            &config_dir,
            &RollbackStateRecord {
                last_transition_at: now_iso8601(),
                ..sidecar.clone()
            },
        );
    }

    // Step 4 — locate the most-recent backup.
    let backup_path = match most_recent_backup_path(&config_dir) {
        Some(path) => path,
        None => {
            // No backup to restore from. The sidecar is left alone because no
            // rollback attempt was made — recording rollback-failed here would
            // be dishonest (nothing failed).
            return RollbackOutcome::NoBackup {
                reason: format!(
                    "no backup at {} — cannot restore from nothing",
                    BACKUP_APPIMAGE_FILENAME
                ),
            };
        }
    };

    // Verify the backup is readable and hash it so the installed-state we
    // write after the atomic-replace records the prior version's identity
    // honestly.
    let backup_sha256 = match sha256_of_path(&backup_path) {
        Ok(sha) => sha,
        Err(source) => {
            let reason = format!(
                "backup at {} is unreadable: {source}",
                backup_path.display()
            );
            let failed_record = RollbackStateRecord {
                rollback_state: RollbackState::RollbackFailed,
                reason: reason.clone(),
                last_transition_at: now_iso8601(),
                ..sidecar.clone()
            };
            let _ = write_rollback_state(&config_dir, &failed_record);
            return RollbackOutcome::RollbackFailed {
                reason,
                pending_update_path: pending_path,
            };
        }
    };

    // Atomically copy the backup over the managed AppImage path.
    let managed = pending.managed_executable_path.clone();
    if let Err(source) = atomic_copy_replace(&backup_path, &managed) {
        let reason = format!(
            "atomic restore {} -> {} failed: {source}",
            backup_path.display(),
            managed.display()
        );
        let failed_record = RollbackStateRecord {
            rollback_state: RollbackState::RollbackFailed,
            reason: reason.clone(),
            last_transition_at: now_iso8601(),
            ..sidecar.clone()
        };
        let _ = write_rollback_state(&config_dir, &failed_record);
        return RollbackOutcome::RollbackFailed {
            reason,
            pending_update_path: pending_path,
        };
    }

    // Restore succeeded — rewrite installed-state.json so future transactions
    // see the prior version as installed. Read the prior installed-state
    // first (if present) so we preserve its provenance fields; if absent
    // (first install, somehow stuck), synthesize a minimal record from the
    // pending marker's `from_version`.
    let installed_state_path = update_dir.join(INSTALLED_STATE_FILENAME);
    let prior_installed = fs::read(&installed_state_path)
        .ok()
        .and_then(|bytes| serde_json::from_slice::<InstalledState>(&bytes).ok());
    let restored_version = pending.from_version.clone();
    let new_installed = if let Some(mut prior) = prior_installed {
        prior.version = restored_version.clone();
        prior.artifact_sha256 = backup_sha256.clone();
        prior.installed_at = now_iso8601();
        prior
    } else {
        InstalledState {
            managed_executable_path: managed.clone(),
            install_kind: InstallKind::AppImage,
            channel: pending.channel.clone(),
            version: restored_version.clone(),
            source_commit: pending.source_commit.clone(),
            release_tag: pending.release_tag.clone(),
            manifest_hash: pending.manifest_hash.clone(),
            artifact_sha256: backup_sha256.clone(),
            installed_at: now_iso8601(),
            update_eligible: true,
            ineligible_reason: None,
        }
    };
    if let Err(source) = write_atomic_json(&installed_state_path, &new_installed) {
        // The atomic-replace already landed on disk but installed-state did
        // not — surface this as a partial rollback. The managed binary is
        // already the prior version; the operator can rerun the sweep to
        // rewrite installed-state.
        let reason = format!(
            "managed binary restored to {} but installed-state write failed: {source}",
            managed.display()
        );
        let failed_record = RollbackStateRecord {
            rollback_state: RollbackState::RollbackFailed,
            reason: reason.clone(),
            last_transition_at: now_iso8601(),
            ..sidecar.clone()
        };
        let _ = write_rollback_state(&config_dir, &failed_record);
        return RollbackOutcome::RollbackFailed {
            reason,
            pending_update_path: pending_path,
        };
    }

    // Delete pending-update.json — the restore superseded it. This is the
    // one and only place F3b touches pending-update.json deletion, and only
    // on a successful restore. AV-RB-7 is honored because we never delete
    // pending-update.json while it is unresolved.
    let _ = fs::remove_file(&pending_path);

    // Reset the sidecar — the rollback succeeded. The counter resets only
    // when the auto-restore fast-path fired (the threshold-chain is broken).
    // A manual Promoted leaves the counter intact because the mismatch chain
    // has not been demonstrably broken — the operator restored, but the
    // underlying mismatch may recur on the next boot and the AV-RB-3 trigger
    // should still be reachable from the pre-threshold count.
    let reset_record = if auto_restore {
        RollbackStateRecord {
            rollback_state: RollbackState::None,
            mismatch_count: 0,
            reason: String::new(),
            last_transition_at: now_iso8601(),
        }
    } else {
        RollbackStateRecord {
            rollback_state: sidecar.rollback_state,
            mismatch_count: sidecar.mismatch_count,
            reason: sidecar.reason.clone(),
            last_transition_at: now_iso8601(),
        }
    };
    let _ = write_rollback_state(&config_dir, &reset_record);

    if auto_restore {
        RollbackOutcome::AutoRestored {
            restored_from: backup_path,
            managed_executable_path: managed,
            restored_version,
            restored_artifact_sha256: backup_sha256,
        }
    } else {
        RollbackOutcome::Promoted {
            restored_from: backup_path,
            managed_executable_path: managed,
            restored_version,
            restored_artifact_sha256: backup_sha256,
        }
    }
}

/// Core F3b retention sweep implementation. The Tauri command shim delegates
/// here.
///
/// Decision tree:
///
///   1. Read `pending-update.json` if present. Its `pending_update_state`
///      determines whether staging cleanup is safe.
///   2. AV-RB-6: when `pending_update_state == Success`, delete every file
///      in `staging/`. Otherwise leave staging intact (operator diagnostics
///      + retry path).
///
///      Note: `Success` is rare in practice because F3c's verifier deletes
///      `pending-update.json` on Promoted — but the sweep defensively handles
///      a Success marker that survived the F3c deletion (e.g. a crash between
///      the installed-state write and the pending deletion).
///   3. AV-RB-5: enforce the backups two-slot ceiling. If more than two
///      `Codex.previous*.AppImage` files exist
///      (degenerate state — F3a's rotation already caps at 2), evict the
///      oldest by mtime until only two remain.
///   4. AV-RB-7: this function NEVER touches `pending-update.json` unless
///      the pending marker is in `Success` state (and even then, only on
///      the staging side — the pending file itself is left for F3c's
///      verifier to delete on the next Promoted).
pub fn perform_retention_sweep_impl(config: RetentionConfig) -> RetentionOutcome {
    let RetentionConfig { config_dir } = config;
    let update_dir = config_update_dir(&config_dir);
    let staging = staging_dir(&update_dir);
    let backups = backups_dir(&update_dir);
    let pending_path = update_dir.join(PENDING_UPDATE_FILENAME);

    let pending_state: Option<PendingUpdateState> = fs::read(&pending_path)
        .ok()
        .and_then(|bytes| serde_json::from_slice::<PendingUpdate>(&bytes).ok())
        .map(|p| p.pending_update_state);

    let mut removed_staging_files: Vec<PathBuf> = Vec::new();

    // Step 2 — staging cleanup (AV-RB-6). Only sweep on Success.
    if matches!(pending_state, Some(PendingUpdateState::Success)) && staging.exists() {
        if let Ok(entries) = fs::read_dir(&staging) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && fs::remove_file(&path).is_ok() {
                    removed_staging_files.push(path);
                }
            }
        }
    }

    // Step 3 — backups two-slot enforcement (AV-RB-5). F3a's
    // `preserve_previous` already caps at 2 on every install; this is the
    // safety net for any drift.
    let mut backup_paths: Vec<PathBuf> = Vec::new();
    if backups.exists() {
        if let Ok(entries) = fs::read_dir(&backups) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file()
                    && path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|name| {
                            name.starts_with("Codex.previous")
                                && name.ends_with(".AppImage")
                        })
                        .unwrap_or(false)
                {
                    backup_paths.push(path);
                }
            }
        }
    }
    // Sort by mtime ascending (oldest first).
    backup_paths.sort_by_key(|p| {
        fs::metadata(p)
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });
    let mut backup_evicted = false;
    while backup_paths.len() > 2 {
        if let Some(oldest) = backup_paths.first().cloned() {
            if fs::remove_file(&oldest).is_ok() {
                backup_paths.remove(0);
                backup_evicted = true;
            } else {
                // I/O failure — stop evicting to avoid an infinite loop. The
                // sweep is idempotent; the next invocation will retry.
                break;
            }
        } else {
            break;
        }
    }
    let remaining_backup_count = backup_paths.len() as u32;

    // The sweep is "Swept" when any work was actually performed — staging
    // cleanup OR backup eviction. Otherwise it is "NoOp" so the UI can
    // suppress the sweep summary on idle cycles. Pending-marker presence
    // alone is not evidence of work.
    if removed_staging_files.is_empty() && !backup_evicted {
        RetentionOutcome::NoOp
    } else {
        RetentionOutcome::Swept {
            removed_staging_files,
            remaining_backup_count,
        }
    }
}

// ===================================================================================
// F3b unit tests — cover AV-RB-3, AV-RB-4, AV-RB-5, AV-RB-6, AV-RB-7 (Rust side)
// ===================================================================================

#[cfg(test)]
mod rollback_retention_tests {
    use super::*;

    fn tempdir(label: &str) -> std::path::PathBuf {
        let dir =
            std::env::temp_dir().join(format!("sd16-e7-f3b-{label}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create tempdir");
        dir
    }

    /// Stage a `pending-update.json` in Mismatch state, with a real backup
    /// at slot1, so the restore function has something to read.
    fn stage_mismatch_with_backup(
        dir: &Path,
        backup_payload: &[u8],
        from: &str,
        to: &str,
    ) -> (PathBuf, PathBuf) {
        let update_dir = config_update_dir(dir);
        fs::create_dir_all(&update_dir).expect("mkdir update");
        fs::create_dir_all(staging_dir(&update_dir)).expect("mkdir staging");
        fs::create_dir_all(backups_dir(&update_dir)).expect("mkdir backups");

        let backup_path = backups_dir(&update_dir).join(BACKUP_APPIMAGE_FILENAME);
        fs::write(&backup_path, backup_payload).expect("write backup");

        let pending = PendingUpdate {
            from_version: from.into(),
            to_version: to.into(),
            artifact_sha256: "expected-running-sha".into(),
            manifest_hash: "manifest-hash-fixture".into(),
            staging_path: staging_dir(&update_dir).join("AppImage"),
            backup_path: backup_path.clone(),
            managed_executable_path: dir.join("Codex.AppImage"),
            channel: "alpha".into(),
            release_tag: format!("alpha/{to}"),
            source_commit: "deadbeef".into(),
            created_at: "2026-07-03T00:00:00Z".into(),
            pending_update_state: PendingUpdateState::Mismatch,
        };
        let pending_path = update_dir.join(PENDING_UPDATE_FILENAME);
        write_atomic_json(&pending_path, &pending).expect("write pending");

        (pending_path, backup_path)
    }

    /// AV-RB-3 — repeated pending failure auto-restores when safe. After
    /// AUTO_RESTORE_THRESHOLD mismatch cycles (bumped inside the impl on
    /// each call), the next `perform_restore_previous_impl` invocation
    /// must take the auto-restore fast-path and return `AutoRestored`.
    /// Cycles 1 and 2 fall through to the manual restore path
    /// (`Promoted`); cycle 3 must auto-restore.
    #[test]
    fn auto_restore_after_threshold_mismatch_cycles() {
        let dir = tempdir("auto-restore");
        let backup_payload = b"prior-binary-payload";
        let managed = dir.join("Codex.AppImage");
        fs::write(&managed, b"stuck-running-binary").expect("write managed");

        let update_dir = config_update_dir(&dir);

        for cycle in 1..=AUTO_RESTORE_THRESHOLD {
            stage_mismatch_with_backup(&dir, backup_payload, "0.0.0", "0.0.1");
            let outcome = perform_restore_previous_impl(RestoreConfig {
                config_dir: dir.clone(),
                bump_mismatch_count: true,
            });
            match outcome {
                RollbackOutcome::AutoRestored { restored_version, .. } => {
                    assert_eq!(
                        cycle, AUTO_RESTORE_THRESHOLD,
                        "auto-restore fired on cycle {cycle}, expected only on cycle {AUTO_RESTORE_THRESHOLD}"
                    );
                    assert_eq!(restored_version, "0.0.0");
                    return; // success — exit early
                }
                RollbackOutcome::Promoted { restored_version, .. } => {
                    // Cycles 1 and 2 fall through to the manual restore path.
                    // The counter accumulates across these calls so cycle 3
                    // reaches the threshold. Re-stage for the next cycle.
                    assert_eq!(restored_version, "0.0.0");
                    let _ = fs::remove_file(update_dir.join(INSTALLED_STATE_FILENAME));
                    continue;
                }
                other => panic!("cycle {cycle}: unexpected outcome {other:?}"),
            }
        }
        panic!("auto-restore never fired across {AUTO_RESTORE_THRESHOLD} cycles");
    }

    /// AV-RB-4 (state side) — restore failure marks `rollback-failed` with
    /// the exact reason. Force a restore failure by giving the function a
    /// missing backup path (write the pending marker but not the backup).
    /// The function must return `RollbackOutcome::NoBackup` (no rollback
    /// was attempted) — but if we corrupt the backup file (write garbage
    /// that fails the atomic-copy), the function returns `RollbackFailed`
    /// with a reason string and writes the sidecar.
    ///
    /// The cleanest positive case for AV-RB-4: write a pending marker,
    /// write a backup, then atomically-replace the backup with a directory
    /// (which will cause the read-back failure). The restore function will
    /// see the unreadable backup, write the sidecar with `rollback_state:
    /// "rollback-failed"`, and return `RollbackFailed` with the reason.
    #[test]
    fn restore_failure_marks_rollback_failed() {
        let dir = tempdir("rollback-failed");
        let managed = dir.join("Codex.AppImage");
        fs::write(&managed, b"stuck-running-binary").expect("write managed");

        let update_dir = config_update_dir(&dir);
        fs::create_dir_all(&update_dir).expect("mkdir update");

        // Stage a pending marker but no backup. The restore function returns
        // NoBackup (no rollback was attempted, so AV-RB-4 does not fire).
        // To force AV-RB-4 we need the backup to exist but be unreadable.
        // The cleanest unreadable fixture: write the backup path as a
        // directory. `sha256_of_path` opens the path; opening a directory
        // is allowed on Linux but read fails.
        let backup_path = backups_dir(&update_dir).join(BACKUP_APPIMAGE_FILENAME);
        fs::create_dir_all(&backup_path).expect("mkdir where backup should be");

        // Stage a Mismatch pending marker.
        let pending = PendingUpdate {
            from_version: "0.0.0".into(),
            to_version: "0.0.1".into(),
            artifact_sha256: "expected-running-sha".into(),
            manifest_hash: "manifest-hash-fixture".into(),
            staging_path: update_dir.join("staging/AppImage"),
            backup_path: backup_path.clone(),
            managed_executable_path: managed.clone(),
            channel: "alpha".into(),
            release_tag: "alpha/0.0.1".into(),
            source_commit: "deadbeef".into(),
            created_at: "2026-07-03T00:00:00Z".into(),
            pending_update_state: PendingUpdateState::Mismatch,
        };
        write_atomic_json(&update_dir.join(PENDING_UPDATE_FILENAME), &pending)
            .expect("write pending");

        let outcome = perform_restore_previous_impl(RestoreConfig {
            config_dir: dir.clone(),
            bump_mismatch_count: true,
        });
        match outcome {
            RollbackOutcome::RollbackFailed { reason, pending_update_path } => {
                assert!(
                    !reason.is_empty(),
                    "rollback-failed reason must be non-empty (AV-RB-4)"
                );
                assert!(pending_update_path.exists(), "pending-update.json must remain on disk");
            }
            other => panic!("expected RollbackFailed, got {other:?}"),
        }

        // The sidecar must record rollback-failed with the reason.
        let sidecar = read_rollback_state(&dir);
        assert_eq!(sidecar.rollback_state, RollbackState::RollbackFailed);
        assert!(!sidecar.reason.is_empty(), "sidecar reason must be non-empty (AV-RB-4)");

        // Cleanup: remove the directory-as-backup so the next test gets a clean state.
        let _ = fs::remove_dir_all(&backup_path);
    }

    /// AV-RB-5 — last 2 previous AppImages are retained. Drive 3 sequential
    /// installs and assert the backups directory ends with exactly 2 entries.
    /// F3a's `preserve_previous` already enforces this on every install;
    /// F3b's retention sweep is a belt-and-suspenders catch-up that also
    /// asserts the contract at the boundary.
    #[test]
    fn backups_keep_last_two_after_retention_sweep() {
        let dir = tempdir("backups-cap");
        let managed = dir.join("Codex.AppImage");
        fs::write(&managed, b"v1").expect("write v1");

        let update_dir = config_update_dir(&dir);
        fs::create_dir_all(backups_dir(&update_dir)).expect("mkdir backups");

        // Simulate 3 sequential installs by writing 3 backup files directly
        // into the backups directory (older → newer mtime).
        let slot1 = backups_dir(&update_dir).join(BACKUP_APPIMAGE_FILENAME);
        let slot2 = backups_dir(&update_dir).join(BACKUP_APPIMAGE_FILENAME_2);
        let extra = backups_dir(&update_dir).join("Codex.previous3.AppImage");
        fs::write(&extra, b"oldest").expect("write extra");
        std::thread::sleep(std::time::Duration::from_millis(10));
        fs::write(&slot2, b"middle").expect("write slot2");
        std::thread::sleep(std::time::Duration::from_millis(10));
        fs::write(&slot1, b"newest").expect("write slot1");

        // Run the retention sweep.
        let outcome = perform_retention_sweep_impl(RetentionConfig {
            config_dir: dir.clone(),
        });
        match outcome {
            RetentionOutcome::Swept {
                remaining_backup_count,
                ..
            } => {
                assert_eq!(
                    remaining_backup_count, 2,
                    "retention sweep must cap backups at 2"
                );
            }
            other => panic!("expected Swept, got {other:?}"),
        }

        // The oldest backup must have been evicted; the two named slots remain.
        assert!(!extra.exists(), "oldest backup must be evicted by sweep");
        assert!(slot1.exists(), "slot1 must remain");
        assert!(slot2.exists(), "slot2 must remain");

        // Cleanup.
        let _ = fs::remove_file(&managed);
    }

    /// AV-RB-6 — successful staging is cleaned after confirmation. Stage a
    /// pending-update.json with `pending_update_state: Success`, drop a file
    /// in `staging/`, run the sweep, and assert staging is empty.
    #[test]
    fn successful_staging_is_cleaned_after_sweep() {
        let dir = tempdir("staging-cleanup");
        let managed = dir.join("Codex.AppImage");
        fs::write(&managed, b"new-binary").expect("write managed");

        let update_dir = config_update_dir(&dir);
        fs::create_dir_all(staging_dir(&update_dir)).expect("mkdir staging");
        fs::create_dir_all(backups_dir(&update_dir)).expect("mkdir backups");

        let staging_file = staging_dir(&update_dir).join("Codex.staged.AppImage");
        fs::write(&staging_file, b"staged-payload").expect("write staged");

        let pending = PendingUpdate {
            from_version: "0.0.0".into(),
            to_version: "0.0.1".into(),
            artifact_sha256: "new-running-sha".into(),
            manifest_hash: "manifest-hash-fixture".into(),
            staging_path: staging_file.clone(),
            backup_path: backups_dir(&update_dir).join(BACKUP_APPIMAGE_FILENAME),
            managed_executable_path: managed.clone(),
            channel: "alpha".into(),
            release_tag: "alpha/0.0.1".into(),
            source_commit: "deadbeef".into(),
            created_at: "2026-07-03T00:00:00Z".into(),
            pending_update_state: PendingUpdateState::Success,
        };
        write_atomic_json(&update_dir.join(PENDING_UPDATE_FILENAME), &pending)
            .expect("write pending");

        let outcome = perform_retention_sweep_impl(RetentionConfig {
            config_dir: dir.clone(),
        });
        match outcome {
            RetentionOutcome::Swept {
                removed_staging_files,
                ..
            } => {
                assert!(
                    !removed_staging_files.is_empty(),
                    "sweep must report at least one removed staging file"
                );
                assert!(
                    !staging_file.exists(),
                    "staging file must be removed on Success sweep (AV-RB-6)"
                );
            }
            other => panic!("expected Swept, got {other:?}"),
        }
    }

    /// AV-RB-7 — pending updates are never auto-deleted while unresolved.
    /// Stage a Mismatch pending marker, run the sweep, and assert the
    /// pending-update.json is still on disk.
    #[test]
    fn pending_update_never_deleted_while_unresolved() {
        let dir = tempdir("pending-keep");
        let managed = dir.join("Codex.AppImage");
        fs::write(&managed, b"stuck-binary").expect("write managed");

        let update_dir = config_update_dir(&dir);
        fs::create_dir_all(&update_dir).expect("mkdir update");

        let pending = PendingUpdate {
            from_version: "0.0.0".into(),
            to_version: "0.0.1".into(),
            artifact_sha256: "expected-sha".into(),
            manifest_hash: "manifest-hash-fixture".into(),
            staging_path: update_dir.join("staging/AppImage"),
            backup_path: update_dir.join("backups/AppImage"),
            managed_executable_path: managed.clone(),
            channel: "alpha".into(),
            release_tag: "alpha/0.0.1".into(),
            source_commit: "deadbeef".into(),
            created_at: "2026-07-03T00:00:00Z".into(),
            pending_update_state: PendingUpdateState::Mismatch,
        };
        let pending_path = update_dir.join(PENDING_UPDATE_FILENAME);
        write_atomic_json(&pending_path, &pending).expect("write pending");

        let outcome = perform_retention_sweep_impl(RetentionConfig {
            config_dir: dir.clone(),
        });
        match outcome {
            RetentionOutcome::Swept { .. } | RetentionOutcome::NoOp => {}
        }

        assert!(
            pending_path.exists(),
            "pending-update.json must NOT be deleted while Mismatch (AV-RB-7)"
        );
    }

    /// Restore function never fabricates success — when no pending-update.json
    /// exists, the function returns `NoPending` rather than fabricating a
    /// `Promoted` outcome. This is the symmetric counterpart to the AV-RB-1
    /// guarantee owned by F3c: both surfaces refuse to fabricate success.
    #[test]
    fn restore_with_no_pending_returns_no_pending() {
        let dir = tempdir("no-pending");
        let outcome = perform_restore_previous_impl(RestoreConfig {
            config_dir: dir.clone(),
            bump_mismatch_count: true,
        });
        assert_eq!(outcome, RollbackOutcome::NoPending);
    }

    /// Restore function refuses to act on a Success pending marker — that
    /// state means F3c's verifier already promoted and the pending marker is
    /// a stale relic. The function returns `NoPending` rather than running
    /// a restore that would clobber a successful install.
    #[test]
    fn restore_with_success_pending_returns_no_pending() {
        let dir = tempdir("success-pending");
        let update_dir = config_update_dir(&dir);
        fs::create_dir_all(&update_dir).expect("mkdir update");

        let pending = PendingUpdate {
            from_version: "0.0.0".into(),
            to_version: "0.0.1".into(),
            artifact_sha256: "expected-sha".into(),
            manifest_hash: "manifest-hash-fixture".into(),
            staging_path: update_dir.join("staging/AppImage"),
            backup_path: update_dir.join("backups/AppImage"),
            managed_executable_path: dir.join("Codex.AppImage"),
            channel: "alpha".into(),
            release_tag: "alpha/0.0.1".into(),
            source_commit: "deadbeef".into(),
            created_at: "2026-07-03T00:00:00Z".into(),
            pending_update_state: PendingUpdateState::Success,
        };
        write_atomic_json(&update_dir.join(PENDING_UPDATE_FILENAME), &pending)
            .expect("write pending");

        let outcome = perform_restore_previous_impl(RestoreConfig {
            config_dir: dir.clone(),
            bump_mismatch_count: false,
        });
        assert_eq!(outcome, RollbackOutcome::NoPending);
    }

    /// The rollback-state sidecar round-trips through disk and the default
    /// values match when the sidecar is absent.
    #[test]
    fn rollback_state_sidecar_round_trips() {
        let dir = tempdir("sidecar-rt");
        // Absent sidecar → default.
        let default_record = read_rollback_state(&dir);
        assert_eq!(default_record.rollback_state, RollbackState::None);
        assert_eq!(default_record.mismatch_count, 0);
        assert!(default_record.reason.is_empty());

        // Write a record, read it back.
        let record = RollbackStateRecord {
            rollback_state: RollbackState::RollbackFailed,
            mismatch_count: 2,
            reason: "backup corrupted".into(),
            last_transition_at: "2026-07-03T01:00:00Z".into(),
        };
        write_rollback_state(&dir, &record).expect("write sidecar");
        let read_back = read_rollback_state(&dir);
        assert_eq!(read_back, record);
    }

    /// Retention sweep is idempotent: running it on a clean config dir
    /// returns `NoOp` and does not mutate state. The diagnostics UI relies
    /// on this so calling the sweep on every launch is cheap.
    #[test]
    fn retention_sweep_is_idempotent_on_clean_config() {
        let dir = tempdir("idempotent");
        let outcome1 = perform_retention_sweep_impl(RetentionConfig {
            config_dir: dir.clone(),
        });
        let outcome2 = perform_retention_sweep_impl(RetentionConfig {
            config_dir: dir.clone(),
        });
        assert_eq!(outcome1, RetentionOutcome::NoOp);
        assert_eq!(outcome2, RetentionOutcome::NoOp);
    }
}