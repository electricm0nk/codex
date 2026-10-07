//! Installed-state seeding for installs the AppImage relaunch verifier never sees.
//!
//! `installed-state.json` is otherwise written only by `verify_relaunch_artifact` after an
//! AppImage self-update. A package-manager install (.deb) or a first AppImage run therefore had
//! no record, and the Update panel reported every installed field as unknown. This module records
//! the running build's real identity at startup, and refreshes the record for installs that a
//! package manager replaces underneath us.

use std::fs;
use std::path::{Path, PathBuf};

use super::transaction::{
    config_update_dir, now_iso8601, resolve_config_root, sha256_of_file, write_atomic_json, InstallKind, InstalledState, INSTALLED_STATE_FILENAME,
};

/// Facts about the running build, gathered by the caller (the Tauri setup hook).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunningInstall {
    /// The file that is replaced on update: the `$APPIMAGE` path for an AppImage, else the
    /// current executable.
    pub managed_executable_path: PathBuf,
    /// sha256 of `managed_executable_path`.
    pub artifact_sha256: String,
    pub version: String,
    pub source_commit: String,
    pub channel: String,
    /// `$APPIMAGE`, when the process was launched from an AppImage.
    pub appimage_path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeedOutcome {
    Written(Box<InstalledState>),
    Unchanged,
}

/// Directories a package manager installs into. An executable under one of these is a packaged
/// install; anything else (a cargo `target/` dir, a checkout) is a developer build.
const PACKAGED_PREFIXES: [&str; 3] = ["/usr/", "/opt/", "/snap/"];

pub fn detect_install_kind(running: &RunningInstall) -> InstallKind {
    detect_install_kind_for(cfg!(windows), running)
}

/// `is_windows` is a parameter so the Windows rule is testable on any host.
pub fn detect_install_kind_for(is_windows: bool, running: &RunningInstall) -> InstallKind {
    if is_windows {
        // A cargo build lives under `target\debug` or `target\release`; anything else came from the installer.
        let exe = running.managed_executable_path.to_string_lossy().replace('\\', "/");
        return if exe.contains("/target/") { InstallKind::DevLocal } else { InstallKind::WindowsNsis };
    }
    if running.appimage_path.is_some() {
        return InstallKind::AppImage;
    }
    let exe = running.managed_executable_path.to_string_lossy();
    if PACKAGED_PREFIXES.iter().any(|prefix| exe.starts_with(prefix)) {
        InstallKind::Deb
    } else {
        InstallKind::DevLocal
    }
}

fn build_record(running: &RunningInstall, kind: InstallKind) -> InstalledState {
    let commit8: String = running.source_commit.chars().take(8).collect();
    let (update_eligible, ineligible_reason) = match kind {
        InstallKind::DevLocal => (false, Some("dev build is not update-eligible".to_string())),
        InstallKind::AppImage | InstallKind::Deb | InstallKind::WindowsNsis => (true, None),
    };
    InstalledState {
        managed_executable_path: running.managed_executable_path.clone(),
        install_kind: kind,
        channel: running.channel.clone(),
        version: running.version.clone(),
        source_commit: running.source_commit.clone(),
        release_tag: format!("{}/v{}-{}", running.channel, running.version, commit8),
        // No manifest was involved: this record describes what is installed, not what was fetched.
        manifest_hash: String::new(),
        artifact_sha256: running.artifact_sha256.clone(),
        installed_at: now_iso8601(),
        update_eligible,
        ineligible_reason,
    }
}

/// Make `installed-state.json` describe the running build.
///
/// - No record: write one.
/// - Existing AppImage record: left alone; `verify_relaunch_artifact` owns it.
/// - Existing Deb/DevLocal record: rewritten when the binary or version changed, because a
///   package manager replaces the binary without going through the verifier.
/// - Unreadable record: an error, never overwritten.
pub fn reconcile_installed_state(
    config_dir: &Path,
    running: &RunningInstall,
) -> Result<SeedOutcome, String> {
    reconcile_installed_state_for(config_dir, running, cfg!(windows))
}

pub fn reconcile_installed_state_for(
    config_dir: &Path,
    running: &RunningInstall,
    is_windows: bool,
) -> Result<SeedOutcome, String> {
    let path = config_update_dir(config_dir).join(INSTALLED_STATE_FILENAME);
    let existing = match fs::read(&path) {
        Ok(bytes) => Some(serde_json::from_slice::<InstalledState>(&bytes).map_err(|source| {
            format!("installed-state.json at {} is unreadable: {source}", path.display())
        })?),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => None,
        Err(err) => return Err(format!("cannot read {}: {err}", path.display())),
    };

    if let Some(existing) = &existing {
        let owned_by_verifier = existing.install_kind == InstallKind::AppImage;
        let up_to_date = existing.artifact_sha256 == running.artifact_sha256
            && existing.version == running.version;
        if owned_by_verifier || up_to_date {
            return Ok(SeedOutcome::Unchanged);
        }
    }

    let record = build_record(running, detect_install_kind_for(is_windows, running));
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| format!("cannot create {}: {err}", parent.display()))?;
    }
    write_atomic_json(&path, &record).map_err(|err| format!("cannot write {}: {err}", path.display()))?;
    Ok(SeedOutcome::Written(Box::new(record)))
}

/// Gather the running build's identity. `appimage_env` is `$APPIMAGE`; when present it is the
/// managed file (the executable itself lives inside a throwaway mount).
pub fn gather_running_install(
    version: &str,
    current_exe: &Path,
    appimage_env: Option<PathBuf>,
    source_commit: &str,
    channel: &str,
) -> Result<RunningInstall, String> {
    let managed = appimage_env.clone().unwrap_or_else(|| current_exe.to_path_buf());
    let artifact_sha256 = sha256_of_file(&managed)
        .map_err(|err| format!("cannot hash running build at {}: {err}", managed.display()))?;
    Ok(RunningInstall {
        managed_executable_path: managed,
        artifact_sha256,
        version: version.to_string(),
        source_commit: source_commit.to_string(),
        channel: channel.to_string(),
        appimage_path: appimage_env,
    })
}

/// Channel this build publishes to. Overridable at compile time for a beta/stable lane.
const BUILD_CHANNEL: &str = match option_env!("CODEX_UPDATE_CHANNEL") {
    Some(channel) => channel,
    None => "alpha",
};

/// Startup entry point: record the running build in `installed-state.json`. `version` must be the
/// packaged app version (Tauri's `package_info`), which the release stamp controls.
pub fn seed_installed_state_for_running_build(version: &str) -> Result<SeedOutcome, String> {
    let exe = std::env::current_exe().map_err(|err| format!("cannot locate running executable: {err}"))?;
    let appimage = std::env::var_os("APPIMAGE").map(PathBuf::from);
    let running = gather_running_install(version, &exe, appimage, env!("CODEX_GIT_SHA"), BUILD_CHANNEL)?;
    reconcile_installed_state(&resolve_config_root(), &running)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_config(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("sd16-seed-{label}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn deb_running(sha: &str, version: &str) -> RunningInstall {
        RunningInstall {
            managed_executable_path: PathBuf::from("/usr/bin/codex-desktop"),
            artifact_sha256: sha.into(),
            version: version.into(),
            source_commit: "157873a67e80".into(),
            channel: "alpha".into(),
            appimage_path: None,
        }
    }

    fn read_record(config: &Path) -> InstalledState {
        let path = config_update_dir(config).join(INSTALLED_STATE_FILENAME);
        serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
    }

    #[test]
    fn usr_bin_executable_is_a_deb_install() {
        assert_eq!(detect_install_kind(&deb_running("a", "0.16.140")), InstallKind::Deb);
    }

    #[test]
    fn appimage_env_wins_over_executable_location() {
        let mut r = deb_running("a", "0.16.140");
        r.managed_executable_path = PathBuf::from("/home/u/Apps/Codex.AppImage");
        r.appimage_path = Some(PathBuf::from("/home/u/Apps/Codex.AppImage"));
        assert_eq!(detect_install_kind(&r), InstallKind::AppImage);
    }

    #[test]
    fn executable_outside_system_dirs_is_dev_local() {
        let mut r = deb_running("a", "0.16.140");
        r.managed_executable_path = PathBuf::from("/home/u/src/codex/apps/desktop/src-tauri/target/debug/codex-desktop");
        assert_eq!(detect_install_kind(&r), InstallKind::DevLocal);
    }

    fn windows_running(exe: &str) -> RunningInstall {
        let mut r = deb_running("a", "0.16.140");
        r.managed_executable_path = PathBuf::from(exe);
        r
    }

    #[test]
    fn a_windows_executable_outside_a_cargo_target_dir_is_an_installer_install() {
        let installed = windows_running("C:\\Users\\u\\AppData\\Local\\Codex\\codex-desktop.exe");
        assert_eq!(detect_install_kind_for(true, &installed), InstallKind::WindowsNsis);
        let per_machine = windows_running("C:\\Program Files\\Codex\\codex-desktop.exe");
        assert_eq!(detect_install_kind_for(true, &per_machine), InstallKind::WindowsNsis);
    }

    #[test]
    fn a_windows_cargo_build_is_dev_local() {
        for exe in ["C:\\src\\codex\\apps\\desktop\\src-tauri\\target\\debug\\codex-desktop.exe", "C:/src/codex/target/release/codex-desktop.exe"] {
            assert_eq!(detect_install_kind_for(true, &windows_running(exe)), InstallKind::DevLocal, "{exe}");
        }
    }

    #[test]
    fn the_windows_rule_does_not_change_how_other_systems_are_classified() {
        assert_eq!(detect_install_kind_for(false, &deb_running("a", "1")), InstallKind::Deb);
        assert_eq!(detect_install_kind_for(false, &windows_running("C:\\Users\\u\\Codex\\codex-desktop.exe")), InstallKind::DevLocal);
    }

    #[test]
    fn a_windows_install_is_update_eligible_and_re_recorded_after_the_installer_replaces_it() {
        let config = temp_config("windows");
        let mut old = windows_running("C:\\Users\\u\\AppData\\Local\\Codex\\codex-desktop.exe");
        old.artifact_sha256 = "old".into();
        let record = build_record(&old, InstallKind::WindowsNsis);
        assert!(record.update_eligible);
        assert_eq!(record.install_kind, InstallKind::WindowsNsis);
        // The installer is not the verifier: a new binary or version on the next start must rewrite the record.
        let path = config_update_dir(&config).join(INSTALLED_STATE_FILENAME);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, serde_json::to_vec(&record).unwrap()).unwrap();
        let mut new = old.clone();
        new.artifact_sha256 = "new".into();
        new.version = "0.16.141".into();
        let outcome = reconcile_installed_state_for(&config, &new, true).unwrap();
        assert!(matches!(outcome, SeedOutcome::Written(_)), "{outcome:?}");
        assert_eq!(read_record(&config).version, "0.16.141");
    }

    #[test]
    fn missing_record_is_seeded_with_the_running_builds_real_identity() {
        let config = temp_config("missing");
        let outcome = reconcile_installed_state(&config, &deb_running("sha-new", "0.16.140")).unwrap();
        let record = read_record(&config);
        assert_eq!(outcome, SeedOutcome::Written(Box::new(record.clone())));
        assert_eq!(record.install_kind, InstallKind::Deb);
        assert_eq!(record.version, "0.16.140");
        assert_eq!(record.source_commit, "157873a67e80");
        assert_eq!(record.artifact_sha256, "sha-new");
        assert_eq!(record.channel, "alpha");
        assert_eq!(record.release_tag, "alpha/v0.16.140-157873a6");
        assert_eq!(record.managed_executable_path, PathBuf::from("/usr/bin/codex-desktop"));
        assert!(record.update_eligible);
        assert!(!record.installed_at.is_empty());
    }

    #[test]
    fn dev_local_seed_is_marked_not_update_eligible_with_a_reason() {
        let config = temp_config("dev");
        let mut r = deb_running("a", "0.16.140");
        r.managed_executable_path = PathBuf::from("/home/u/src/codex/target/debug/codex-desktop");
        reconcile_installed_state(&config, &r).unwrap();
        let record = read_record(&config);
        assert_eq!(record.install_kind, InstallKind::DevLocal);
        assert!(!record.update_eligible);
        assert!(record.ineligible_reason.is_some());
    }

    #[test]
    fn deb_record_is_refreshed_after_the_package_manager_replaced_the_binary() {
        let config = temp_config("refresh");
        reconcile_installed_state(&config, &deb_running("sha-old", "0.16.139")).unwrap();
        let outcome = reconcile_installed_state(&config, &deb_running("sha-new", "0.16.140")).unwrap();
        assert!(matches!(outcome, SeedOutcome::Written(_)));
        let record = read_record(&config);
        assert_eq!(record.version, "0.16.140");
        assert_eq!(record.artifact_sha256, "sha-new");
    }

    #[test]
    fn identical_deb_record_is_left_alone() {
        let config = temp_config("same");
        reconcile_installed_state(&config, &deb_running("sha", "0.16.140")).unwrap();
        let outcome = reconcile_installed_state(&config, &deb_running("sha", "0.16.140")).unwrap();
        assert_eq!(outcome, SeedOutcome::Unchanged);
    }

    #[test]
    fn existing_appimage_record_is_owned_by_the_verifier_and_never_rewritten() {
        let config = temp_config("appimage");
        let mut r = deb_running("sha-a", "0.16.139");
        r.managed_executable_path = PathBuf::from("/home/u/Apps/Codex.AppImage");
        r.appimage_path = Some(r.managed_executable_path.clone());
        reconcile_installed_state(&config, &r).unwrap();
        r.artifact_sha256 = "sha-b".into();
        r.version = "0.16.140".into();
        let outcome = reconcile_installed_state(&config, &r).unwrap();
        assert_eq!(outcome, SeedOutcome::Unchanged);
        assert_eq!(read_record(&config).artifact_sha256, "sha-a");
    }

    #[test]
    fn corrupt_record_is_reported_not_clobbered() {
        let config = temp_config("corrupt");
        let dir = config_update_dir(&config);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join(INSTALLED_STATE_FILENAME);
        fs::write(&path, b"{ not json").unwrap();
        assert!(reconcile_installed_state(&config, &deb_running("a", "0.16.140")).is_err());
        assert_eq!(fs::read(&path).unwrap(), b"{ not json");
    }

    #[test]
    fn gather_hashes_the_managed_file_not_the_appimage_mount_executable() {
        let dir = temp_config("gather");
        let appimage = dir.join("Codex.AppImage");
        fs::write(&appimage, b"appimage-bytes").unwrap();
        let inside_mount = dir.join("mount-codex-desktop");
        fs::write(&inside_mount, b"different-bytes").unwrap();
        let r = gather_running_install("0.16.140", &inside_mount, Some(appimage.clone()), "157873a67e80", "alpha").unwrap();
        assert_eq!(r.managed_executable_path, appimage);
        assert_eq!(r.appimage_path, Some(appimage));
        assert_eq!(r.artifact_sha256, super::super::transaction::sha256_of_file(&dir.join("Codex.AppImage")).unwrap());
    }

    #[test]
    fn gather_uses_the_current_executable_when_not_an_appimage() {
        let dir = temp_config("gather-deb");
        let exe = dir.join("codex-desktop");
        fs::write(&exe, b"deb-bytes").unwrap();
        let r = gather_running_install("0.16.140", &exe, None, "157873a67e80", "alpha").unwrap();
        assert_eq!(r.managed_executable_path, exe);
        assert_eq!(r.appimage_path, None);
        assert_eq!(r.version, "0.16.140");
        assert_eq!(r.artifact_sha256.len(), 64);
    }

    #[test]
    fn gather_fails_loudly_when_the_managed_file_cannot_be_read() {
        let dir = temp_config("gather-missing");
        assert!(gather_running_install("0.16.140", &dir.join("nope"), None, "c", "alpha").is_err());
    }
}
